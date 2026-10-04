// femacs: port of src/keyboard.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, union, enum, macro, local), same control flow, same error model.  C's 38 `DEFUN`s use
// `DEFUN!` (PORTING.md 5.1); C's non-`static` definitions carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2) and the C `static` helpers stay plain `unsafe fn` — except the ones whose
// address C stores in an unwind-protect or in a signal action, which are `unsafe extern "C" fn`
// (PORTING.md 1.4).
//
// **File-scope objects live in `pub mod keyboard_globals`.**  `src/keyboard.h`'s mirror
// declares this file's objects at the crate root (`include/keyboard.rs` 238-470), and
// `blockinput.h`'s mirror declares `interrupt_input_blocked` the same way; a crate-root
// definition of a name the mirror also declares at the crate root is `E0428` (measured), so the
// definitions sit in a module that is glob-imported — the `frame.rs` `frame_globals` /
// `term.rs` `term_globals` shape, which PORTING.md P-21 describes from the other side.  Every
// reference in this file still spells the C name; the `DEFVAR_*` holders (below) are plain
// crate-root `static mut`s, which shadow the `globals.rs` `variables` glob import as P-21
// intends.
//
// **Carried arms.**  `src/keyboard.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/keyboard.c` reports `216 conditions, 0 unknown,
// 0 disagreements [VERIFIED]` — the frozen macro table decides every one of them the way clang
// does, but the file has not been added to the list), so `tools/extract-abi.py` keeps *both*
// arms of every `#if` live and the port carries the text the frozen configuration does not
// compile, each under a `#[cfg (feature = "…")]` naming the C condition's non-frozen side
// (docs/status.md 3.49, PORTING.md P-51).  None of those features is ever set.  The arms, in C
// order:
//
//   * C 44-46, 51-53, 87-89 and the `#include` arms (the sections below).
//   * C 104-120 `#ifdef HAVE_TEXT_CONVERSION` -> `have_text_conversion`
//     (`reading_key_sequence`, the event-type and the `handle_interrupt` arms).
//   * C 127-141 `#if INTPTR_MAX <= INT_MAX` -> `intptr_leq_int` (`MAX_NUM_RECENT_KEYS`).
//   * C 605-631, 1000-1050, 1100-1130, 1270-1290, 1290-1330, 1690-1780, 2000-2060,
//     2300-2340, 2400-2500, 2700-3000, 3700-3800, 5600-5680, 6000-6100, 6500-6700,
//     7100-7200, 7500-7600, 7900-8000, 8100-8200, 8300-8400, 9200-9300, 9500-9700,
//     10000-10100, 10300-10400, 11900-12000, 12400-12500, 12700-12760, 13000-13100,
//     13200-13300 and 14500-14600: the `HAVE_NS`, `HAVE_X_WINDOWS`, `HAVE_NTGUI`,
//     `WINDOWSNT`, `MSDOS`, `DOS_NT`, `HAVE_ANDROID`, `HAVE_PGTK`, `HAVE_HAIKU`,
//     `HAVE_DBUS`, `HAVE_GPM`, `HAVE_XWIDGETS`, `USE_TOOLKIT_SCROLL_BARS`, `POLL_FOR_INPUT`,
//     `USE_FILE_NOTIFY`, `THREADS_ENABLED`, `USG5_4`, `CYGWIN` and `HAVE_TEXT_CONVERSION`
//     arms; the exact line ranges are recorded per site in the port's own comments and in
//     `docs/ports/keyboard.md`.
//   * C 4598-4673: `process_special_events`'s `HAVE_X11`/`HAVE_PGTK`/`HAVE_HAIKU` chain
//     (`have_x11_or_have_pgtk`, `have_haiku`, and the live `emacs_abort` arm).
//   * C 4977-5080: `lispy_accent_codes`'s twenty `#ifdef XK_dead_<name>` element pairs
//     (`xk_dead_<name>`).
//   * C 5109-5461: the `HAVE_ANDROID` and `HAVE_NTGUI` arms of the key tables
//     (`have_android`, `have_ntgui`), and C 5509-5512's `HAVE_NS || !HAVE_WINDOW_SYSTEM` guard
//     around the live table's `static` keyword.
//   * C 6106-6110 (`have_ext_menu_bar`), C 6124-6145 (`window_system_no_ext_menu_bar`) and
//     C 6147-6168 (`have_window_system`): the toolkit predicate's `#else` and the two
//     coordinates-in-bar helpers.
//
// **The live configuration** (docs/baseline/config/darwin-arm64.macros.tsv): `HAVE_PTHREAD` 1,
// `HAVE_STACK_OVERFLOW_HANDLING` 1, `USABLE_FIONREAD` 1, `subprocesses` 1, `HAVE_PDUMPER` 1;
// `HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/`HAVE_NS`/`HAVE_PGTK`/`HAVE_HAIKU`/`HAVE_ANDROID`/
// `HAVE_NTGUI`/`WINDOWSNT`/`MSDOS`/`DOS_NT`/`CYGWIN`/`HAVE_X11`/`HAVE_DBUS`/`HAVE_GPM`/
// `HAVE_XWIDGETS`/`HAVE_GTK3`/`USE_X_TOOLKIT`/`USE_TOOLKIT_SCROLL_BARS`/`POLL_FOR_INPUT`/
// `USE_FILE_NOTIFY`/`THREADS_ENABLED`/`HAVE_TEXT_CONVERSION`/`USG5_4` all `#undef` (the
// character-terminal build; `HAVE_EXT_MENU_BAR`/`HAVE_EXT_TOOL_BAR` are lisp.h 3929-3932's
// and 3935-3937's *conditional* defines — `USE_X_TOOLKIT`/`HAVE_NTGUI`/`HAVE_NS`/`USE_GTK`/
// `HAVE_HAIKU` decide both, and all of those are `#undef` above — so both macros are undefined
// here and the *terminal* menu-bar and tool-bar arms are the live text.  (`lisp.rs` mirrors the
// `#define`'s body as the constant `true`; the condition it sits under is what decides, which is
// why the `#[cfg (feature = "have_ext_menu_bar")]` sites below are the dead arms — the reading
// this lane's `toolkit_menubar_in_use` follows.)
//
// **Forward declarations are dropped** (PORTING.md 0.1-12): C 321-323, 325-326 and 379-410's
// `static` prototypes have no Rust counterpart (Rust is order-independent and a declaration
// plus a definition is `E0428`); the definitions stay at C's positions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons, overflowing_literals,
         unnecessary_cast, dropping_copy_types, unused_must_use, clashing_extern_declarations)]

/* C 20-95: the `#include`s.  The mirrors are named with C's own header in the comment.  */
include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("../lib/include/sys_stat.in.rs");     // C 22: `#include <sys/stat.h>`
include!("../lib/include/stdio.rs");           // C 41: `#include "sysstdio.h"`
include!("../lib/include/errno.rs");           // C 55: `#include <errno.h>`
include!("../lib/include/sys_types.in.rs");    // C 78: `#include <sys/types.h>`
include!("../lib/include/unistd.in.rs");       // C 79: `#include <unistd.h>`
include!("../lib/include/fcntl.in.rs");        // C 80: `#include <fcntl.h>`
include!("../lib/include/math.rs");            // C 81: `#include <math.h>`
include!("../lib/include/signal.rs");          // C 72: `#include "syssignal.h"` -> <signal.h> (pthread_sigmask)
include!("include/lisp.rs");                   // C 24: `#include "lisp.h"`
/* C's `lisp.h` 28 includes <stdcountof.h>, which `include/lisp.rs` does not carry (P-26/P-72③
   mirror gap); the port puts the mirror where `countof` is read, the alloc.rs/bignum.rs shape.  */
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>`
include!("include/character.rs");              // C 34: `#include "character.h"`
include!("include/intervals.rs");              // C 38: `#include "intervals.h"` (buffer.h 27)
include!("include/itree.rs");                  // intervals.h: `itree_node`/`itree_tree`
include!("include/systime.rs");                // C 42: `#include "systime.h"`
include!("include/atimer.rs");                 // C 43: `#include "atimer.h"`
include!("include/coding.rs");                 // C 25: `#include "coding.h"`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/buffer.rs");                 // C 35: `#include "buffer.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termchar.rs");               // C 26: `#include "termchar.h"`
include!("include/dispextern.rs");             // C 36: `#include "dispextern.h"`
include!("include/termhooks.rs");              // C 29: `#include "termhooks.h"`
include!("include/frame.rs");                  // C 28: `#include "frame.h"`
include!("include/window.rs");                 // C 32: `#include "window.h"`
include!("include/keyboard.rs");               // C 31: `#include "keyboard.h"`
include!("include/macros.rs");                 // C 30: `#include "macros.h"`
include!("include/commands.rs");               // C 33: `#include "commands.h"`
include!("include/syntax.rs");                 // C 37: `#include "syntax.h"`
include!("include/keymap.rs");                 // C 39: `#include "keymap.h"`
include!("include/blockinput.rs");             // C 40: `#include "blockinput.h"`
include!("include/process.rs");                // C 44: `#include "process.h"`
include!("include/menu.rs");                   // C 45: `#include "menu.h"`
include!("include/syssignal.rs");              // C 72: `#include "syssignal.h"`
include!("../lib/include/ignore-value.rs"); // C 83: `#include <ignore-value.h>`
include!("include/pdumper.rs");                // C 85: `#include "pdumper.h"`

/* C 92-95: `char const DEV_TTY[]` is defined twice (the `WINDOWSNT` `"CONOUT$"` arm and the
   live `#else`); `emacs.rs` declares the object as `[c_char; 9]`, which is the nine bytes of
   `"/dev/tty"` counting the terminating NUL.  */
#[unsafe(no_mangle)]
pub static DEV_TTY: [c_char; 9] =
  [b'/' as c_char, b'd' as c_char, b'e' as c_char, b'v' as c_char, b'/' as c_char,
   b't' as c_char, b't' as c_char, b'y' as c_char, 0];

/* C 96: `char *dev_tty;	/* set by init_keyboard */`  */
/* C 99-105: `volatile int interrupt_input_blocked;` and `volatile bool pending_signals;` --
   Rust has no `volatile` statics; the objects stay ordinary statics (the port's `volatile`
   reading is the C compiler's, and every access is already in an `unsafe` block).  */
/* C 107-109, 112, 148-149, 214, 231-233, 251, 262, 310, 314-321, 344, 453, 467-470, 529:
   the objects whose declarations `include/keyboard.rs` (238-470) and `include/blockinput.rs`
   (50) put at the crate root.  A crate-root definition of one of them is `E0428`, so the
   definitions live in this module, which is glob-imported below (the frame.rs `frame_globals`
   shape; PORTING.md P-21).  */
pub mod keyboard_globals {
  use super::*;

  /* C 96: `char *dev_tty;`  */
  #[unsafe(no_mangle)]
  pub static mut dev_tty: *mut c_char = std::ptr::null_mut ();                      // 529 declares it

  /* C 100: `volatile int interrupt_input_blocked;`  */
  #[unsafe(no_mangle)]
  pub static mut interrupt_input_blocked: c_int = 0;

  /* C 107-109.  */
  #[unsafe(no_mangle)]
  pub static mut initial_kboard: *mut KBOARD = std::ptr::null_mut ();
  #[unsafe(no_mangle)]
  pub static mut current_kboard: *mut KBOARD = std::ptr::null_mut ();

  /* C 148-149.  */
  #[unsafe(no_mangle)]
  pub static mut this_command_keys: Lisp_Object = lisp_h_Qnil ();
  #[unsafe(no_mangle)]
  pub static mut this_command_key_count: ptrdiff_t = 0;

  /* C 214: `int quit_char;`  */
  #[unsafe(no_mangle)]
  pub static mut quit_char: c_int = 0;

  /* C 231-233.  */
  #[unsafe(no_mangle)]
  pub static mut num_input_events: uintmax_t = 0;
  #[unsafe(no_mangle)]
  pub static mut point_before_last_command_or_undo: ptrdiff_t = 0;
  #[unsafe(no_mangle)]
  pub static mut buffer_before_last_command_or_undo: *mut buffer = std::ptr::null_mut ();

  /* C 251: `Lisp_Object internal_last_event_frame;`  */
  #[unsafe(no_mangle)]
  pub static mut internal_last_event_frame: Lisp_Object = lisp_h_Qnil ();

  /* C 310: `bool input_was_pending;`  */
  #[unsafe(no_mangle)]
  pub static mut input_was_pending: bool = false;

  /* C 314-321: the circular buffer and its two pointers.  C's static storage zeroes the
     array; `kind: 0` is the zeroed union.  */
  #[unsafe(no_mangle)]
  pub static mut kbd_buffer: [buffered_input_event; KBD_BUFFER_SIZE as usize] =
    [const { buffered_input_event { kind: 0 } }; KBD_BUFFER_SIZE as usize];
  #[unsafe(no_mangle)]
  pub static mut kbd_fetch_ptr: *mut buffered_input_event = std::ptr::null_mut ();
  #[unsafe(no_mangle)]
  pub static mut kbd_store_ptr: *mut buffered_input_event = std::ptr::null_mut ();

  /* C 344: `struct timespec *input_available_clear_time;`  */
  #[unsafe(no_mangle)]
  pub static mut input_available_clear_time: *mut timespec = std::ptr::null_mut ();

  /* C 269: `static struct buffer *prev_buffer;` — a C `static`, so no `#[unsafe(no_mangle)]`
     (the reference object exports no `_prev_buffer`).  The mirror's crate-root `extern`
     declaration is what the C-comment references resolve to, and nothing in the ported prefix
     reads the object (command_loop_1's local of the same name is `prev_buffer_`).  */
  static mut prev_buffer: *mut buffer = std::ptr::null_mut ();

  /* C 1289: `bool ignore_mouse_drag_p;`  */
  #[unsafe(no_mangle)]
  pub static mut ignore_mouse_drag_p: bool = false;

  /* C 2016: `int poll_suppress_count;`  */
  #[unsafe(no_mangle)]
  pub static mut poll_suppress_count: c_int = 0;

  /* C 8784: `Lisp_Object item_properties;` — `keyboard.h` 292 declares it at the crate root
     (`include/keyboard.rs` 236), so its definition lives here (the `kbd_buffer` shape).  The
     ledger row was ticked before this round because the mirror's declaration alone satisfies
     `tools/extract-abi.py`; this is the definition.  */
  #[unsafe(no_mangle)]
  pub static mut item_properties: Lisp_Object = lisp_h_Qnil ();
}
use keyboard_globals::*;

/* C 101-102.  */
#[unsafe(no_mangle)]
pub static mut pending_signals: bool = false;

/* C 109: `static KBOARD *all_kboards;`  */
static mut all_kboards: *mut KBOARD = std::ptr::null_mut ();

/* C 112: `static bool single_kboard;`  */
static mut single_kboard: bool = false;

/* C 117-119: `#ifdef HAVE_TEXT_CONVERSION`'s `bool reading_key_sequence;` (the macro is
   `#undef` on this target; carried per P-51).  */
#[cfg (feature = "have_text_conversion")]
pub static mut reading_key_sequence: bool = false;

/* C 122-126: `MIN_NUM_RECENT_KEYS` and the two arms of `MAX_NUM_RECENT_KEYS` (the frozen
   target has `INTPTR_MAX > INT_MAX`, so the `#else` arm is live).  The bodies of this macro
   have several configuration definitions and are not compared (docs/ports/keyboard.md).  */
pub const MIN_NUM_RECENT_KEYS: c_int = 100;
pub const MAX_NUM_RECENT_KEYS: c_int = c_int::MAX / EMACS_INT_WIDTH as c_int;
#[cfg (feature = "intptr_leq_int")]
pub const MAX_NUM_RECENT_KEYS: c_int = c_int::MAX / EMACS_INT_WIDTH as c_int / 10;

/* C 128-141.  */
static mut recent_keys_index: c_int = 0;
static mut total_keys: c_int = 0;
static mut lossage_limit: c_int = 3 * MIN_NUM_RECENT_KEYS;
static mut recent_keys: Lisp_Object = lisp_h_Qnil ();

/* C 152-154.  */
static mut raw_keybuf: Lisp_Object = lisp_h_Qnil ();
static mut raw_keybuf_count: c_int = 0;

/* C 156-158: `GROW_RAW_KEYBUF`.  */
macro_rules! GROW_RAW_KEYBUF { () => {
  /* C's `raw_keybuf_count` is `int` and `ASIZE` yields `ptrdiff_t`; the port's `c_int` and
     `isize` need the cast C's promotion performs implicitly.  */
  if raw_keybuf_count as isize == ASIZE (raw_keybuf) {
    raw_keybuf = larger_vector (raw_keybuf, 1, -1);
  }
} }

/* C 162.  */
static mut this_single_command_key_start: ptrdiff_t = 0;

/* C 164-172: `#ifdef HAVE_STACK_OVERFLOW_HANDLING` is live (`1`), so both members are live.  */
#[unsafe(no_mangle)]
pub static mut return_to_command_loop: sigjmp_buf = [0; 196];
static mut recover_top_level_message: Lisp_Object = lisp_h_Qnil ();

/* C 174-175.  */
static mut regular_top_level_message: Lisp_Object = lisp_h_Qnil ();

/* C 177-179.  */
static mut echoing: bool = false;

/* C 184: `static struct kboard *ok_to_echo_at_next_pause;`  */
static mut ok_to_echo_at_next_pause: *mut kboard = std::ptr::null_mut ();

/* C 191: `struct kboard *echo_kboard;` (lisp.rs 3892 declares it in `other_units`, which is a
   glob import: the crate-root item here shadows it, PORTING.md P-21).  */
#[unsafe(no_mangle)]
pub static mut echo_kboard: *mut kboard = std::ptr::null_mut ();

/* C 196.  */
#[unsafe(no_mangle)]
pub static mut echo_message_buffer: Lisp_Object = lisp_h_Qnil ();

/* C 217: `EMACS_INT command_loop_level;`  */
#[unsafe(no_mangle)]
pub static mut command_loop_level: EMACS_INT = 0;

/* C 226.  */
static mut unread_switch_frame: Lisp_Object = lisp_h_Qnil ();

/* C 229: `static ptrdiff_t last_non_minibuf_size;`  */
static mut last_non_minibuf_size: ptrdiff_t = 0;

/* C 237: `static intmax_t last_auto_save;`  */
static mut last_auto_save: intmax_t = 0;

/* C 240: `static ptrdiff_t last_point_position;`  */
static mut last_point_position: ptrdiff_t = 0;

/* C 255-256.  */
static mut read_key_sequence_cmd: Lisp_Object = lisp_h_Qnil ();
static mut read_key_sequence_remapped: Lisp_Object = lisp_h_Qnil ();

/* C 259: `static FILE *dribble;`  */
static mut dribble: *mut FILE = std::ptr::null_mut ();

/* C 262: `bool input_pending;` (lisp.rs 3893's `other_units` glob import is shadowed here).  */
#[unsafe(no_mangle)]
pub static mut input_pending: bool = false;

/* C 340: `unsigned timers_run;`  */
#[unsafe(no_mangle)]
pub static mut timers_run: c_uint = 0;

/* C 348-351: `termopts.h`'s two objects (that header has no mirror: it declares only these,
   which this file defines).  */
#[unsafe(no_mangle)]
pub static mut interrupt_input: bool = false;
#[unsafe(no_mangle)]
pub static mut interrupts_deferred: bool = false;

/* C 355-361.  */
static mut timer_idleness_start_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut timer_last_idleness_start_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };

/* C 364-369.  */
static mut virtual_core_pointer_name: Lisp_Object = lisp_h_Qnil ();
static mut virtual_core_keyboard_name: Lisp_Object = lisp_h_Qnil ();
static mut menu_bar_touch_id: Lisp_Object = lisp_h_Qnil ();

/* C 372-377: the `readable_events` flags.  */
pub const READABLE_EVENTS_DO_TIMERS_NOW: c_int = 1 << 0;
pub const READABLE_EVENTS_FILTER_EVENTS: c_int = 1 << 1;
pub const READABLE_EVENTS_IGNORE_SQUEEZABLES: c_int = 1 << 2;

/* C 380: `static void (*keyboard_init_hook) (void);`  */
static mut keyboard_init_hook: Option<unsafe extern "C" fn ()> = None;

/* C 321-323, 325-326, 379-410: the `static` prototypes are dropped (PORTING.md 0.1-12; they
   name `command_loop`, `echo_now`, `echo_length`, `read_char_x_menu_prompt`,
   `read_char_minibuf_menu_prompt`, `make_lispy_event`, `make_lispy_movement`,
   `modify_event_symbol`, `make_lispy_switch_frame`, `make_lispy_focus_in`,
   `make_lispy_focus_out`, `help_char_p`, `save_getcjmp`, `restore_getcjmp`,
   `apply_modifiers`, `restore_kboard_configuration`, `handle_interrupt`,
   `quit_throw_to_read_char`, `timer_start_idle`, `timer_stop_idle`, `timer_resume_idle`,
   `deliver_user_signal`, `find_user_signal_name`, `store_user_signal_events` and
   `is_ignored_event`, all defined below).  */

/* C 414-419: `next_kbd_event`.  */
unsafe fn next_kbd_event (ptr: *mut buffered_input_event) -> *mut buffered_input_event
{
  return if ptr == (&raw mut kbd_buffer as *mut buffered_input_event)
              .add (KBD_BUFFER_SIZE as usize - 1) { &raw mut kbd_buffer as *mut buffered_input_event }
         else { ptr.add (1) };
}

/* C 421-428: `xevent_start`.  */
unsafe fn xevent_start (event: Lisp_Object) -> Lisp_Object
{
  return XCAR (XCDR (event));
}

/* C 430-479: the `kset_*` setters.  */
unsafe fn kset_echo_string (kb: *mut kboard, val: Lisp_Object) { (*kb).echo_string_ = val; }
unsafe fn kset_echo_prompt (kb: *mut kboard, val: Lisp_Object) { (*kb).echo_prompt_ = val; }
unsafe fn kset_kbd_queue (kb: *mut kboard, val: Lisp_Object) { (*kb).kbd_queue_ = val; }
unsafe fn kset_keyboard_translate_table (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Vkeyboard_translate_table_ = val; }
unsafe fn kset_last_prefix_arg (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Vlast_prefix_arg_ = val; }
unsafe fn kset_last_repeatable_command (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Vlast_repeatable_command_ = val; }
unsafe fn kset_local_function_key_map (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Vlocal_function_key_map_ = val; }
unsafe fn kset_overriding_terminal_local_map (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Voverriding_terminal_local_map_ = val; }
unsafe fn kset_real_last_command (kb: *mut kboard, val: Lisp_Object)
{ (*kb).Vreal_last_command_ = val; }
unsafe fn kset_system_key_syms (kb: *mut kboard, val: Lisp_Object)
{ (*kb).system_key_syms_ = val; }

/* C 481-487: `echo_keystrokes_p`.  */
unsafe fn echo_keystrokes_p () -> bool
{
  return if FLOATP (Vecho_keystrokes) { XFLOAT_DATA (Vecho_keystrokes) > 0.0 }
         else if FIXNUMP (Vecho_keystrokes) { XFIXNUM (Vecho_keystrokes) > 0 }
         else { false };
}

/* gnulib's `INT_STRLEN_BOUND (t)` (intprops.h 88-108) is
   `((TYPE_WIDTH (t) - signed) * 146 + 484) / 485 + signed`; `<intprops.h>` has no mirror
   (P-26), so this is the instantiation this file needs, `INT_STRLEN_BOUND (EMACS_INT)` = 20
   (the emacs.rs/data.rs shape for the same macro).  */
const INT_STRLEN_BOUND: usize = ((EMACS_INT_WIDTH - 1) * 146 + 484) / 485 + 1;

/* `lisp.h`'s `FUNCTIONP` (lisp.h 3772) reaches this file as a macro; the mirror does not carry
   it (P-26), so the port keeps the name (the alloc.rs precedent).  */
macro_rules! FUNCTIONP {
  ($x:expr) => { SYMBOLP ($x) && !NILP (Ffboundp ($x)) };
}

/* frame.h 331: `bool_bf glyphs_initialized_p : 1`.  The mirror carries the storage unit
   (`bits1`) and the shift (`FRAME_GLYPHS_INITIALIZED_P_SHIFT`), not an accessor; PORTING.md
   P-80's member-named shape keeps the C member token.  */
macro_rules! frame_bits {
  (glyphs_initialized_p, $f:expr) => { ((*$f).bits1 >> FRAME_GLYPHS_INITIALIZED_P_SHIFT) & 1 != 0 };
  (mouse_moved, $f:expr) => { ((*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT) & 1 != 0 };
  (mouse_moved, $f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & !(1 << FRAME_MOUSE_MOVED_SHIFT))
                  | (((($v) as u32) & 1) << FRAME_MOUSE_MOVED_SHIFT); };
}

/* C's `lisp.h` prototypes this file uses that the `lisp.rs` mirror does not carry (PORTING.md
   P-26: the mirror gap, reported in docs/ports/keyboard.md, is why the declaration stands at
   its use site; P-72③ wants C's prototype on the `/// C:` line).  */
unsafe extern "C" {
  /// C: lisp.h 4245: `extern Lisp_Object larger_vector (Lisp_Object, ptrdiff_t, ptrdiff_t);`
  fn larger_vector (a: Lisp_Object, len: ptrdiff_t, init: ptrdiff_t) -> Lisp_Object;
  /// C: lisp.h 4323: `extern ptrdiff_t copy_text (const unsigned char *, unsigned char *,
  ///					      ptrdiff_t, bool, bool);`
  fn copy_text (from: *const u8, to: *mut u8, n: ptrdiff_t, from_multibyte: bool,
                to_multibyte: bool) -> ptrdiff_t;
  /// C: lisp.h 4393: `extern void message3_nolog (Lisp_Object);`
  fn message3_nolog (m: Lisp_Object);
  /// C: lisp.h 4401: `extern void truncate_echo_area (ptrdiff_t);`
  fn truncate_echo_area (n: ptrdiff_t);
  /// C: lisp.h 5131: `extern _Noreturn void quit (void);`
  fn quit () -> !;
  /// C: eval.c (lisp.h 5387's `extern bool signal_quit_p (Lisp_Object);`)
  fn signal_quit_p (x: Lisp_Object) -> bool;
  /// C: lisp.h 4399: `extern void clear_message (bool, bool);`
  fn clear_message (current_echo_area: bool, last_displayed_p: bool);
  /// C: xdisp.c (lisp.h 4400's `extern void message_log_maybe_newline (void);`)
  fn message_log_maybe_newline ();
  /// C: print.c (lisp.h 5396's `extern void print_error_message (Lisp_Object, Lisp_Object, const char *, Lisp_Object);`)
  fn print_error_message (data: Lisp_Object, errname: Lisp_Object, errmsg: *const c_char,
                          severity: Lisp_Object);
  /// C: lisp.h 4803: `extern Lisp_Object internal_catch (Lisp_Object, Lisp_Object (*) (Lisp_Object), Lisp_Object);`
  fn internal_catch (tag: Lisp_Object, func: unsafe extern "C" fn (Lisp_Object) -> Lisp_Object,
                     val: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4805: `extern Lisp_Object internal_condition_case (Lisp_Object (*) (void), Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  fn internal_condition_case (bfun: unsafe extern "C" fn () -> Lisp_Object, handlers: Lisp_Object,
                              hfun: unsafe extern "C" fn (Lisp_Object) -> Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4814: `extern void pop_handler (void);`
  fn pop_handler ();
  /// C: lisp.h 4815: `extern void push_handler_bind (Lisp_Object, Lisp_Object, int);`
  fn push_handler_bind (conditions: Lisp_Object, handler: Lisp_Object, var: c_int);
  /// C: eval.c (lisp.h 5160's `extern void init_eval (void);`)
  fn init_eval ();
  /// C: <setjmp.h>: `int sigsetjmp (sigjmp_buf, int);`
  fn sigsetjmp (env: *mut c_void, savemask: c_int) -> c_int;
  /// C: xdisp.c (dispextern.h 3481's `extern void redisplay_preserve_echo_area (int);`)
  fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: lisp.h 4373: `extern Lisp_Object sit_for (Lisp_Object, bool, int);`
  fn sit_for (timeout: Lisp_Object, reading: bool, display: c_int) -> Lisp_Object;
  /// C: lisp.h 4653: `extern void display_malloc_warning (void);`
  fn display_malloc_warning ();
  /// C: dispextern.h 3569: `void resize_echo_area_exactly (void);`
  fn resize_echo_area_exactly ();
  /// C: dispextern.h 3570: `bool resize_mini_window (struct window *, bool);`
  fn resize_mini_window (w: *mut window, use_height: bool) -> bool;
  /// C: dispextern.h 3721: `extern void update_redisplay_ticks (int, struct window *);`
  fn update_redisplay_ticks (ticks: c_int, w: *mut window);
  /// C: dispextern.h 3568: `bool display_prop_intangible_p (Lisp_Object, Lisp_Object, ptrdiff_t, ptrdiff_t);`
  fn display_prop_intangible_p (prop: Lisp_Object, overlay: Lisp_Object,
                                start: ptrdiff_t, end: ptrdiff_t) -> bool;
  /// C: dispextern.h 3593: `ptrdiff_t get_large_narrowing_begv (ptrdiff_t);`
  fn get_large_narrowing_begv (pos: ptrdiff_t) -> ptrdiff_t;
  /// C: dispextern.h 3593: `ptrdiff_t get_large_narrowing_zv (ptrdiff_t);`
  fn get_large_narrowing_zv (pos: ptrdiff_t) -> ptrdiff_t;
  /// C: process.h 261: `extern void wait_reading_process_output (intmax_t, int, int, bool,
  ///   Lisp_Object, struct Lisp_Process *, int);`
  fn wait_reading_process_output (time_limit: intmax_t, nsecs: c_int, read_kbd: c_int,
                                  do_display: bool, wait_for_cell: Lisp_Object,
                                  wait_proc: *mut Lisp_Process, just_wait_proc: c_int);
  /// C: lisp.h 4809: `extern Lisp_Object internal_condition_case_1
  ///   (Lisp_Object (*) (Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  fn internal_condition_case_1 (bfun: unsafe extern "C" fn (Lisp_Object) -> Lisp_Object,
                                arg: Lisp_Object, handlers: Lisp_Object,
                                hfun: unsafe extern "C" fn (Lisp_Object) -> Lisp_Object)
                                -> Lisp_Object;
  /// C: coding.h 800: `extern Lisp_Object code_convert_string (Lisp_Object, Lisp_Object,
  ///   Lisp_Object, bool, bool, ptrdiff_t);`
  fn code_convert_string (string: Lisp_Object, coding_system: Lisp_Object,
                          nocopy: Lisp_Object, buffer: bool, gc: bool,
                          carryover: ptrdiff_t) -> Lisp_Object;
  /// C: <string.h>: `void *memcpy (void *restrict, const void *restrict, size_t);` (the
  /// include list does not pull `lib/include/string.rs` in, the coding.rs shape).
  fn memcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
  /// C: <string.h>: `int memcmp (const void *, const void *, size_t);` (the same include-list
  /// gap as `memcpy` above; the modifier parser is its first reader).
  fn memcmp (s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
  /// C: <string.h>: `int strcmp (const char *, const char *);` (the same include-list gap;
  ///   `handle_user_signal` is its first reader).
  fn strcmp (s1: *const c_char, s2: *const c_char) -> c_int;
  /// C: <string.h>: `void *memset (void *, int, size_t);` (the same include-list gap;
  ///   `store_user_signal_events` is its first reader).
  fn memset (s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
  /// C: <string.h>: `size_t strnlen (const char *, size_t);` (the same include-list gap;
  ///   `menu_separator_name_p` is its first reader; `process.rs`/`sysdep.rs` declare their own).
  fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
  /// C: <string.h>: `char *strcpy (char *restrict, const char *restrict);` (the same
  ///   include-list gap; `parse_tool_bar_item` is its first reader).
  fn strcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /// C: <string.h>: `char *stpcpy (char *restrict, const char *restrict);` (the same
  /// include-list gap; `apply_modifiers_uncached` is its first reader).
  fn stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /// C: lisp.h 4717: `extern ptrdiff_t esprintf (char *, char const *, ...)
  ///   ATTRIBUTE_FORMAT_PRINTF (2, 3);` (doprnt.c; the lisp.rs mirror does not carry it,
  ///   `keymap.rs` declares its own copy).
  fn esprintf (buf: *mut c_char, format: *const c_char, ...) -> ptrdiff_t;
  /// C: <string.h>: `void *memmove (void *, const void *, size_t);` (the same include-list
  /// gap as `memcpy` above; `process_special_events`'s carried `HAVE_X11`/`HAVE_PGTK` arm is
  /// the only user).
  fn memmove (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
  /// C: frame.h 1508: `extern struct frame *root_frame (struct frame *f);` (frame.c defines
  /// it; `term.rs`/`dispnew.rs` declare it for their own callers, this unit for its own —
  /// P-72③, and the frame.rs mirror does not carry it).
  fn root_frame (f: *mut frame) -> *mut frame;
  /// C: frame.h 1909: `extern enum internal_border_part frame_internal_border_part
  ///   (struct frame *f, int x, int y);` (frame.c; `frame.rs` defines it and declares it for
  /// its own callers only).
  fn frame_internal_border_part (f: *mut frame, x: c_int, y: c_int) -> internal_border_part;
  /// C: lisp.h 5111: `extern void line_number_display_width (struct window *, int *, int *);`
  /// (xdisp.c; the `lisp.rs` mirror does not carry it).
  fn line_number_display_width (w: *mut window, col_width: *mut c_int, pixel_width: *mut c_int);
  /// C: <sys/ioctl.h>: `int ioctl (int, unsigned long, ...);` (the mirror does not carry it,
  ///   `callproc.rs` declares its own copy).
  fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /// C: lisp.h 5062: `extern void barf_if_interaction_inhibited (void);` (eval.c; the mirror
  ///   does not carry it).
  fn barf_if_interaction_inhibited ();
  /// C: lisp.h 4802: `extern Lisp_Object apply1 (Lisp_Object, Lisp_Object);` (eval.c; the
  ///   mirror does not carry it, `process.rs` declares its own copy).
  fn apply1 (fn_: Lisp_Object, arg: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (data.c; the mirror does not carry
  ///   it, `bytecode.rs` declares its own copy).
  fn FUNCTIONP (o: Lisp_Object) -> bool;
  /// C: lisp.h 5137: `extern AVOID terminate_due_to_signal (int, int);` (emacs.c; the mirror
  /// does not carry it).
  fn terminate_due_to_signal (sig: c_int, backtrace_limit: c_int) -> !;
  /// C: frame.h 1507: `extern void frame_make_pointer_visible (struct frame *);` (frame.c;
  /// the mirror does not carry it, `frame.rs` declares its own copy for its own callers).
  fn frame_make_pointer_visible (f: *mut frame);
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  /// (frame.c defines it; the mirror does not declare it for this unit, `font.rs` declares its
  /// own copy for a carried arm only).
  fn get_frame_param (frame: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 4013-4015: `bool is_tty_child_frame (struct frame *f);` and
  ///   `bool is_tty_root_frame_with_visible_child (struct frame *f);` (dispnew.c defines both;
  ///   the `is_tty_frame` declaration above is this unit's third member of that family).
  fn is_tty_child_frame (f: *mut frame) -> bool;
  fn is_tty_root_frame_with_visible_child (f: *mut frame) -> bool;
  /// C: sysdep.c: `extern void sys_suspend (void);` (systty.h; no mirror carries it)
  fn sys_suspend ();
  /// C: process.c (process.h 254: `extern bool kbd_on_hold_p (void);`)
  fn kbd_on_hold_p () -> bool;
  /// C: process.c (process.h 252: `extern void hold_keyboard_input (void);`)
  fn hold_keyboard_input ();
  /// C: process.c (process.h 253: `extern void unhold_keyboard_input (void);`)
  fn unhold_keyboard_input ();
  /// C: dispnew.c (keyboard.h 500-501: `extern void request_sigio (void);`)
  fn request_sigio ();
  /// C: dispnew.c (keyboard.h 501: `extern void unrequest_sigio (void);`)
  fn unrequest_sigio ();
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4397: `extern void message_with_string (const char *, Lisp_Object, bool);`
  fn message_with_string (m: *const c_char, string: Lisp_Object, log: bool);
  /// C: lisp.h 5619: `extern void record_unwind_protect_void (void (*) (void));`
  fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: lisp.h 4929: `extern void labeled_narrow_to_region (Lisp_Object, Lisp_Object, Lisp_Object);`
  fn labeled_narrow_to_region (start: Lisp_Object, end: Lisp_Object, label: Lisp_Object);
  /// C: lisp.h 4783: `extern Lisp_Object run_hook_with_args (ptrdiff_t nargs, Lisp_Object *args,
  ///   Lisp_Object (*funcall) (ptrdiff_t nargs, Lisp_Object *args));`
  fn run_hook_with_args (nargs: ptrdiff_t, args: *mut Lisp_Object,
                         funcall: unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object)
                         -> Lisp_Object;
  /// C: lisp.h 4808: `extern Lisp_Object internal_condition_case_n
  ///   (Lisp_Object (*) (ptrdiff_t, Lisp_Object *), ptrdiff_t, Lisp_Object *,
  ///    Lisp_Object, Lisp_Object (*) (Lisp_Object, ptrdiff_t, Lisp_Object *));`
  fn internal_condition_case_n (bfun: unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object,
                                nargs: ptrdiff_t, args: *mut Lisp_Object, handlers: Lisp_Object,
                                hfun: unsafe extern "C" fn (Lisp_Object, ptrdiff_t, *mut Lisp_Object)
                                                              -> Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4412: `extern Lisp_Object safe_eval (Lisp_Object);`
  fn safe_eval (form: Lisp_Object) -> Lisp_Object;
  /// C: intervals.h 279: `extern bool get_property_and_range (ptrdiff_t, Lisp_Object, Lisp_Object *,
  ///   ptrdiff_t *, ptrdiff_t *, Lisp_Object);`
  fn get_property_and_range (pos: ptrdiff_t, prop: Lisp_Object, val: *mut Lisp_Object,
                             start: *mut ptrdiff_t, end: *mut ptrdiff_t,
                             base_interval: Lisp_Object) -> bool;
  /// C: intervals.h 305: `Lisp_Object get_char_property_and_overlay (Lisp_Object, Lisp_Object,
  ///   Lisp_Object, Lisp_Object *);`
  fn get_char_property_and_overlay (position: Lisp_Object, prop: Lisp_Object,
                                    object: Lisp_Object, overlay: *mut Lisp_Object) -> Lisp_Object;
  /// C: term.c (termopts.h/termchar.h 425's `extern bool is_tty_frame (struct frame *);`)
  fn is_tty_frame (f: *mut frame) -> bool;
  /// C: lisp.h 4712: `extern void message3_frame (Lisp_Object, Lisp_Object);`
  fn message3_frame (m: Lisp_Object, frame: *mut frame);
  /// C: coding.c (coding.h 786's `extern bool raw_text_coding_system_p (struct coding_system *);`)
  fn raw_text_coding_system_p (coding: *mut coding_system) -> bool;
  /// C: window.h 1124-1125: `extern Lisp_Object window_from_coordinates (struct frame *, int,
  ///   int, enum window_part *, bool, bool, bool);` (window.c defines it; the `window.rs`
  ///   mirror does not declare it).
  fn window_from_coordinates (f: *mut frame, x: c_int, y: c_int, part: *mut window_part,
                              menu_bar_p: bool, tab_bar_p: bool, tool_bar_p: bool) -> Lisp_Object;
  /// C: dispextern.h 3582: `int estimate_mode_line_height (struct frame *, enum face_id);`
  ///   (xdisp.c defines it; the mirror does not declare it, and the
  ///   `WINDOW_HEADER_LINE_HEIGHT`/`WINDOW_TAB_LINE_HEIGHT` expansions above need it).
  fn estimate_mode_line_height (f: *mut frame, face_id: face_id) -> c_int;
  /// C: dispextern.h 3578-3579: `int window_box_width (struct window *, enum glyph_row_area);`
  ///   and `int window_box_left (struct window *, enum glyph_row_area);` (xdisp.c defines both;
  ///   neither mirror declares them).
  fn window_box_width (w: *mut window, area: glyph_row_area) -> c_int;
  fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3883-3887: `extern Lisp_Object buffer_posn_from_coords (struct window *,
  ///   int *, int *, struct display_pos *, Lisp_Object *, int *, int *, int *, int *);`
  ///   (xdisp.c defines it; the `dispextern.rs` mirror does not declare it).
  fn buffer_posn_from_coords (w: *mut window, x: *mut c_int, y: *mut c_int,
                              pos: *mut display_pos, object: *mut Lisp_Object,
                              dx: *mut c_int, dy: *mut c_int, width: *mut c_int,
                              height: *mut c_int) -> Lisp_Object;
  /// C: dispextern.h 3888-3891: `extern Lisp_Object mode_line_string (struct window *,
  ///   enum window_part, int *, int *, ptrdiff_t *, Lisp_Object *, int *, int *, int *, int *);`
  ///   (xdisp.c defines it; the mirror does not declare it).
  fn mode_line_string (w: *mut window, part: window_part, x: *mut c_int, y: *mut c_int,
                       charpos: *mut ptrdiff_t, object: *mut Lisp_Object, dx: *mut c_int,
                       dy: *mut c_int, width: *mut c_int, height: *mut c_int) -> Lisp_Object;
  /// C: dispextern.h 3892-3895: `extern Lisp_Object marginal_area_string (struct window *,
  ///   enum window_part, int *, int *, ptrdiff_t *, Lisp_Object *, int *, int *, int *, int *);`
  ///   (xdisp.c defines it; the mirror does not declare it).
  fn marginal_area_string (w: *mut window, part: window_part, x: *mut c_int, y: *mut c_int,
                           charpos: *mut ptrdiff_t, object: *mut Lisp_Object, dx: *mut c_int,
                           dy: *mut c_int, width: *mut c_int, height: *mut c_int) -> Lisp_Object;
  /// C: dispextern.h 3584-3585: `void pixel_to_glyph_coords (struct frame *, int, int,
  ///   int *, int *, NativeRectangle *, bool);` — `NativeRectangle` is `int` on this target
  ///   (dispextern.h 178-179's `#ifndef NativeRectangle #define NativeRectangle int`).  xdisp.c
  ///   defines it; the mirror does not declare it, and `make_lispy_event`'s `MOUSE_CLICK_EVENT`
  ///   arm reads it in the *live* text (the `#if` above it, C 6483-6494, is what is dead).
  fn pixel_to_glyph_coords (f: *mut frame, x: c_int, y: c_int, px: *mut c_int,
                            py: *mut c_int, bounds: *mut c_int, noclip: bool);
  /// C: lisp.h 4562: `extern Lisp_Object make_event_array (ptrdiff_t, Lisp_Object *);`
  ///   (alloc.c defines it; the `lisp.rs` mirror does not carry the declaration).
  fn make_event_array (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool,
  ///   bool);` (dispnew.c defines it; the `include/frame.rs` mirror does not carry it, and
  ///   `frame.rs` declares its own copy for its own callers).
  fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int, pretend: bool,
                        delay: bool, safe: bool);
  /// C: dispextern.h 3780: `void get_tty_size (int, int *, int *);` (dispnew.c defines it;
  ///   neither mirror carries it).
  fn get_tty_size (fd: c_int, widthp: *mut c_int, heightp: *mut c_int);
  /// C: lisp.h 5289: `extern void discard_tty_input (void);` (sysdep.c; the mirror does not
  ///   carry it).
  fn discard_tty_input ();
  /// C: lisp.h 5292: `extern void init_all_sys_modes (void);` (sysdep.c) — its function
  ///   pointer is what `suspend-emacs` hands to `record_unwind_protect_void`.
  fn init_all_sys_modes ();
  /// C: lisp.h 5293: `extern void reset_all_sys_modes (void);` (sysdep.c).
  fn reset_all_sys_modes ();
  /// C: lisp.h 5287: `extern void sys_subshell (void);` (sysdep.c) — the `sys_suspend`
  ///   declaration above is the same family.
  fn sys_subshell ();
  /// C: lisp.h 5285: `extern int stuff_char (char c);` (sysdep.c defines it with C's
  ///   `int` return, which `lisp.h` 5285 declares).
  fn stuff_char (c: c_char) -> c_int;
  /// C: lisp.h 5320: `extern FILE *emacs_fdopen (int, const char *);` (`ATTRIBUTE_MALLOC
  ///   ATTRIBUTE_DEALLOC (emacs_fclose, 1)`; sysdep.c defines it, and the `lread.rs` mirror
  ///   declares its own copy for its own callers).
  fn emacs_fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
  /// C: lisp.h 5324: `extern int emacs_unlink (const char *);` (sysdep.c; `emacs.rs`
  ///   declares its own copy).
  fn emacs_unlink (name: *const c_char) -> c_int;
  /// C: lisp.h 4436: `extern void mark_object (Lisp_Object);` (alloc.c defines it; the
  ///   `lisp.rs` mirror carries `mark_objects` only, `print.rs` declares its own copy).
  fn mark_object (obj: Lisp_Object);
  /// C: lisp.h 5290-5291: `extern void init_sys_modes (struct tty_display_info *);`
  ///   and `extern void reset_sys_modes (struct tty_display_info *);` (sysdep.c defines both;
  ///   neither mirror declares them for this unit).
  fn init_sys_modes (tty: *mut tty_display_info);
  fn reset_sys_modes (tty: *mut tty_display_info);
  /// C: termhooks.h 962-963: `extern struct terminal *decode_tty_terminal (Lisp_Object);`
  ///   and `extern struct terminal *get_named_terminal (const char *);` (term.c defines both;
  ///   the `include/termhooks.rs` mirror does not carry them, `term.rs` declares its own).
  fn decode_tty_terminal (terminal: Lisp_Object) -> *mut terminal;
  fn get_named_terminal (name: *const c_char) -> *mut terminal;
  /// C: window.h 1210: `extern struct window *decode_live_window (Lisp_Object);` (window.c
  ///   defines it; the mirror does not declare it, `indent.rs`/`font.rs` declare their own).
  fn decode_live_window (window: Lisp_Object) -> *mut window;
  /// C: dispextern.h 3580: `int window_box_left_offset (struct window *, enum
  ///   glyph_row_area);` (xdisp.c defines it; the mirror does not declare it).
  fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: lisp.h 5119-5122: `extern Lisp_Object do_switch_frame (Lisp_Object, int, int,
  ///   Lisp_Object);` (frame.c defines it; `minibuf.rs` declares its own copy).
  fn do_switch_frame (frame: Lisp_Object, for_deletion: c_int, track: c_int,
                      norecord: Lisp_Object) -> Lisp_Object;
}

/// C: dispextern.h 3610: `extern bool display_working_on_window_p;`
unsafe extern "C" { pub static mut display_working_on_window_p: bool; }
/// C: dispextern.h 3613: `extern bool help_echo_showing_p;`
unsafe extern "C" { pub static mut help_echo_showing_p: bool; }
/// C: xdisp.c (dispextern.h 3565's `extern bool composition_break_at_point;`)
unsafe extern "C" { pub static mut composition_break_at_point: bool; }

/// C: dispextern.h 3604: `extern void redisplay (void);`
unsafe extern "C" { fn redisplay (); }
/// C: dispnew.c's `bool redisplaying_p;` (dispextern.h 3508's `extern bool redisplaying_p;`)
unsafe extern "C" { pub static mut redisplaying_p: bool; }
/// C: lisp.h 5139's `extern bool noninteractive;` is lisp.rs's; this is keyboard.c's own
/// `multiple_terminals_merge_keyboards` (lisp.h 5124) and the two kbd-macro counters.
unsafe extern "C" {
  pub static mut executing_kbd_macro: Lisp_Object;
  pub static mut executing_kbd_macro_iterations: EMACS_INT;
}

/* coding.h 408-440: `common_flags:14` is the low 14 bits of the mirror's `coding_system::bits`
   storage word (PORTING.md P-80's member-named shape); this file reads the mask bit that
   matters here.  */
macro_rules! coding_bits {
  (common_flags, $c:expr) => { (*$c).bits };
}

/* buffer.h 716: `bool_bf long_line_optimizations_p : 1`.  The mirror carries the storage unit
   (`buffer::bits`) and the mask (`BUFFER_LONG_LINE_OPTIMIZATIONS_P`); PORTING.md P-80's
   member-named shape.  */
macro_rules! buffer_bits {
  (long_line_optimizations_p, $b:expr) => { ((*$b).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P) != 0 };
}

/* window.h 392-410's flags the mirror stores in `window::bits` (PORTING.md P-80's
   member-named shape; the dispnew.rs `window_bits` precedent, one arm per member this file
   names).  */
macro_rules! window_bits {
  (force_start, $w:expr) => { ((*$w).bits >> WINDOW_FORCE_START_SHIFT) & 1 != 0 };
  (force_start, $w:expr, $v:expr) => {
    (*$w).bits = ((*$w).bits & !(1u32 << WINDOW_FORCE_START_SHIFT))
                 | (((($v) as u32) & 1) << WINDOW_FORCE_START_SHIFT); };
  (optional_new_start, $w:expr) => { ((*$w).bits >> WINDOW_OPTIONAL_NEW_START_SHIFT) & 1 != 0 };
  (optional_new_start, $w:expr, $v:expr) => {
    (*$w).bits = ((*$w).bits & !(1u32 << WINDOW_OPTIONAL_NEW_START_SHIFT))
                 | (((($v) as u32) & 1) << WINDOW_OPTIONAL_NEW_START_SHIFT); };
}

/* keyboard.h 38: `KVAR` is a token-pasting macro (`((kboard)->field ## _)`) with no Rust
   counterpart; the port keeps the macro's name and the member token C pastes onto, one arm per
   member this file names (the term.rs shape; `KVAR` is listed in docs/ports/excluded-macros.txt
   because its *body* cannot be mirrored, but each call site keeps C's tokens).  */
macro_rules! KVAR {
  ($kb:expr, Voverriding_terminal_local_map) => { (*$kb).Voverriding_terminal_local_map_ };
  ($kb:expr, Vinput_decode_map) => { (*$kb).Vinput_decode_map_ };
  ($kb:expr, Vkeyboard_translate_table) => { (*$kb).Vkeyboard_translate_table_ };
  ($kb:expr, Vlast_command) => { (*$kb).Vlast_command_ };
  ($kb:expr, Vlast_kbd_macro) => { (*$kb).Vlast_kbd_macro_ };
  ($kb:expr, Vlast_prefix_arg) => { (*$kb).Vlast_prefix_arg_ };
  ($kb:expr, Vlast_repeatable_command) => { (*$kb).Vlast_repeatable_command_ };
  ($kb:expr, Vlocal_function_key_map) => { (*$kb).Vlocal_function_key_map_ };
  ($kb:expr, Vprefix_arg) => { (*$kb).Vprefix_arg_ };
  ($kb:expr, Vreal_last_command) => { (*$kb).Vreal_last_command_ };
  ($kb:expr, Vsystem_key_alist) => { (*$kb).Vsystem_key_alist_ };
  ($kb:expr, Vwindow_system) => { (*$kb).Vwindow_system_ };
  ($kb:expr, Vdefault_minibuffer_frame) => { (*$kb).Vdefault_minibuffer_frame_ };
  ($kb:expr, defining_kbd_macro) => { (*$kb).defining_kbd_macro_ };
  ($kb:expr, echo_prompt) => { (*$kb).echo_prompt_ };
  ($kb:expr, echo_string) => { (*$kb).echo_string_ };
  ($kb:expr, kbd_queue) => { (*$kb).kbd_queue_ };
  ($kb:expr, system_key_syms) => { (*$kb).system_key_syms_ };
}

/* C's `max`/`min` (lisp.h 71-72) reach this file as macros over any integer type; the port
   spells them as macros of the same name so every C site keeps its token (the alloc.rs
   precedent: a macro and the mirror's `fn` share a name).  */
macro_rules! max {
  ($a:expr, $b:expr) => { if ($a) > ($b) { ($a) } else { ($b) } };
}
macro_rules! min {
  ($a:expr, $b:expr) => { if ($a) < ($b) { ($a) } else { ($b) } };
}

/* keyboard.h 233: `bool_bf immediate_echo : 1`.  The mirror carries the storage unit (`u8`);
   the port reads and writes the member through a member-named accessor (PORTING.md P-80's
   shape for a bitfield a mirror spells as an integer).  */
macro_rules! kboard_bits {
  (immediate_echo, $kb:expr) => { ((*$kb).immediate_echo & 1) != 0 };
  (immediate_echo, $kb:expr, $v:expr) => {
    (*$kb).immediate_echo = ((*$kb).immediate_echo & !1) | ((($v) as u8) & 1); };
}

/* C 491-500.  */
unsafe fn echo_add_key (mut c: Lisp_Object)
{
  let mut initbuf: [c_char; (KEY_DESCRIPTION_SIZE + 100) as usize] =
    [0; (KEY_DESCRIPTION_SIZE + 100) as usize];
  let mut size: ptrdiff_t = std::mem::size_of_val (&initbuf) as ptrdiff_t;
  let mut buffer: *mut c_char = initbuf.as_mut_ptr ();
  let mut ptr: *mut c_char = buffer;
  let echo_string: Lisp_Object = KVAR! (current_kboard, echo_string);
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  if STRINGP (echo_string) && SCHARS (echo_string) > 0
    {
      /* Add a space at the end as a separator between keys.  */
      *ptr = b' ' as c_char;
      ptr = ptr.add (1);
    }

  /* If someone has passed us a composite event, use its head symbol.  */
  c = EVENT_HEAD (c);

  if FIXNUMP (c)
    { ptr = push_key_description (XFIXNUM (c), ptr); }
  else if SYMBOLP (c)
    {
      let name: Lisp_Object = SYMBOL_NAME (c);
      let nbytes: ptrdiff_t = SBYTES (name);

      if size - buffer.offset_from (ptr) < nbytes
	{
	  let offset: ptrdiff_t = buffer.offset_from (ptr);
	  size = max! (2 * size, size + nbytes);
	  buffer = SAFE_ALLOCA! (sa_avail, size) as *mut c_char;
	  ptr = buffer.offset (offset);
	}

      ptr = ptr.add (copy_text (SDATA (name), ptr as *mut u8, nbytes,
				STRING_MULTIBYTE (name), true) as usize);
    }

  let mut new_string: Lisp_Object = make_string (buffer, buffer.offset_from (ptr));
  if (NILP (echo_string) || SCHARS (echo_string) == 0)
    && help_char_p (c)
    {
      AUTO_STRING! (str, c" (Type ? for further options, C-q for quick help)".as_ptr ());
      AUTO_LIST2! (props, Qface (), Qhelp_key_binding ());
      Fadd_text_properties (make_fixnum (7), make_fixnum (8), props, str);
      Fadd_text_properties (make_fixnum (30), make_fixnum (33), props, str);
      new_string = concat2 (new_string, str);
    }

  kset_echo_string (current_kboard, concat2 (echo_string, new_string));
  SAFE_FREE! (sa_count);
}

/* C 538-596.  */
unsafe fn echo_dash ()
{
  /* Do nothing if not echoing at all.  */
  if NILP (KVAR! (current_kboard, echo_string))
    { return; }

  if !kboard_bits! (immediate_echo, current_kboard)
      && SCHARS (KVAR! (current_kboard, echo_string)) == 0
    { return; }

  /* Do nothing if we just printed a prompt.  */
  if STRINGP (KVAR! (current_kboard, echo_prompt))
      && (SCHARS (KVAR! (current_kboard, echo_prompt))
	  == SCHARS (KVAR! (current_kboard, echo_string)))
    { return; }

  /* Do nothing if we have already put a dash at the end.  */
  if SCHARS (KVAR! (current_kboard, echo_string)) > 1
    {
      let mut last_char: Lisp_Object;
      let mut prev_char: Lisp_Object;
      let mut idx: Lisp_Object;

      idx = make_fixnum (SCHARS (KVAR! (current_kboard, echo_string)) - 2);
      prev_char = Faref (KVAR! (current_kboard, echo_string), idx);

      idx = make_fixnum (SCHARS (KVAR! (current_kboard, echo_string)) - 1);
      last_char = Faref (KVAR! (current_kboard, echo_string), idx);

      if (XFIXNUM (last_char) == b'-' as EMACS_INT && XFIXNUM (prev_char) != b' ' as EMACS_INT)
	  /* Or a keystroke help message.  */
	  || (echo_keystrokes_help
	      && XFIXNUM (last_char) == b')' as EMACS_INT && XFIXNUM (prev_char) == b'p' as EMACS_INT)
	{ return; }
    }

  /* Put a dash at the end of the buffer temporarily,
     but make it go away when the next character is added.  */
  AUTO_STRING! (dash, c"-".as_ptr ());
  kset_echo_string (current_kboard, concat2 (KVAR! (current_kboard, echo_string), dash));

  if echo_keystrokes_help
    { kset_echo_string (current_kboard,
			calln! (Qhelp__append_keystrokes_help (), KVAR! (current_kboard, echo_string))); }

  echo_now ();
}

/* C 598-621.  */
unsafe fn echo_update ()
{
  if kboard_bits! (immediate_echo, current_kboard)
    {
      let mut i: ptrdiff_t;
      let prompt: Lisp_Object = KVAR! (current_kboard, echo_prompt);
      let prefix: Lisp_Object = call0 (Qinternal_echo_keystrokes_prefix ());
      kset_echo_string (current_kboard,
			if NILP (prompt) { prefix }
			else if NILP (prefix) { prompt }
			else { concat2 (prompt, prefix) });

      i = 0;
      while i < this_command_key_count
	{
	  let c: Lisp_Object = AREF (this_command_keys, i);
	  if ! (EVENT_HAS_PARAMETERS (c)
		&& EQ (EVENT_HEAD_KIND (EVENT_HEAD (c)), Qmouse_movement ()))
	    { echo_add_key (c); }
	  i += 1;
	}

      echo_now ();
    }
}

/* C 623-655.  */
unsafe fn echo_now ()
{
  if !kboard_bits! (immediate_echo, current_kboard)
      /* This test breaks calls that use `echo_now' to display the echo_prompt.
         && echo_keystrokes_p () */
    {
      kboard_bits! (immediate_echo, current_kboard, true);
      echo_update ();
      /* Put a dash at the end to invite the user to type more.  */
      echo_dash ();
    }

  echoing = true;
  /* FIXME: Use call (Qmessage) so it can be advised (e.g. emacspeak).  */
  message3_nolog (KVAR! (current_kboard, echo_string));
  echoing = false;

  /* Record in what buffer we echoed, and from which kboard.  */
  echo_message_buffer = echo_area_buffer[0];
  echo_kboard = current_kboard;

  if waiting_for_input! () && !NILP (Vquit_flag)
    { quit_throw_to_read_char (false); }
}

/* C 657-665.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cancel_echoing ()
{
  kboard_bits! (immediate_echo, current_kboard, false);
  kset_echo_prompt (current_kboard, Qnil ());
  kset_echo_string (current_kboard, Qnil ());
  ok_to_echo_at_next_pause = std::ptr::null_mut ();
  echo_kboard = std::ptr::null_mut ();
  echo_message_buffer = Qnil ();
}

/* C 667-674.  */
unsafe fn echo_length () -> ptrdiff_t
{
  return if STRINGP (KVAR! (current_kboard, echo_string))
	 { SCHARS (KVAR! (current_kboard, echo_string)) }
	 else { 0 };
}

/* C 676-688.  */
unsafe fn echo_truncate (nchars: ptrdiff_t)
{
  let es: Lisp_Object = KVAR! (current_kboard, echo_string);
  if STRINGP (es) && SCHARS (es) > nchars
    { kset_echo_string (current_kboard,
			Fsubstring (KVAR! (current_kboard, echo_string),
				    make_fixnum (0), make_fixnum (nchars))); }
  truncate_echo_area (nchars);
}

/* Functions for manipulating this_command_keys.  */
/* C 693-706.  */
unsafe fn add_command_key (key: Lisp_Object)
{
  if this_command_key_count >= ASIZE (this_command_keys)
    { this_command_keys = larger_vector (this_command_keys, 1, -1); }

  ASET (this_command_keys, this_command_key_count, key);
  this_command_key_count += 1;
}

/* C 708-763.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recursive_edit_1 () -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut val: Lisp_Object;

  if command_loop_level > 0
    {
      specbind (Qstandard_output (), Qt ());
      specbind (Qstandard_input (), Qt ());
      specbind (Qsymbols_with_pos_enabled (), Qnil ());
      specbind (Qprint_symbols_bare (), Qnil ());
    }

  /* C 718-725: `#ifdef HAVE_WINDOW_SYSTEM`'s `cancel_hourglass ();` — `HAVE_WINDOW_SYSTEM` is
     `#undef` on this target (carried per PORTING.md P-51).  */
  #[cfg (feature = "have_window_system")]
  { cancel_hourglass (); }

  /* This function may have been called from a debugger called from
     within redisplay, for instance by Edebugging a function called
     from fontification-functions.  We want to allow redisplay in
     the debugging session.

     The recursive edit is left with a `(throw exit ...)'.  The `exit'
     tag is not caught anywhere in redisplay, i.e. when we leave the
     recursive edit, the original redisplay leading to the recursive
     edit will be unwound.  The outcome should therefore be safe.  */
  specbind (Qinhibit_redisplay (), Qnil ());
  redisplaying_p = false;

  /* This variable stores buffers that have changed so that an undo
     boundary can be added. specbind this so that changes in the
     recursive edit will not result in undo boundaries in buffers
     changed before we entered there recursive edit.
     See Bug #23632.
  */
  specbind (Qundo_auto__undoably_changed_buffers (), Qnil ());

  val = command_loop ();
  if EQ (val, Qt ())
    { quit (); }
  /* Handle throw from read_minibuf when using minibuffer
     while it's active but we're in another window.  */
  if STRINGP (val)
    { xsignal1 (Qerror (), val); }

  if FUNCTIONP! (val)
    { call0 (val); }

  return unbind_to (count, Qnil ());
}

/* When an auto-save happens, record the "time", and don't do again soon.  */
/* C 765-769.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn record_auto_save ()
{
  last_auto_save = num_nonmacro_input_events as intmax_t;
}

/* Make an auto save happen as soon as possible at command level.  */
/* C 771-779: `#ifdef SIGDANGER`.  `SIGDANGER` is a `#undef` feature test on this target
   (carried per PORTING.md P-51).  */
#[cfg (feature = "sigdanger")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_auto_save_soon ()
{
  last_auto_save = - auto_save_interval - 1;
}

/* C 777-808: `DEFUN ("recursive-edit", …)`.  */
DEFUN! ("recursive-edit", Frecursive_edit, Srecursive_edit, 0, 0, "",
       doc: "Invoke the editor command loop recursively.\nTo get out of the recursive edit, a command can throw to `exit' -- for\ninstance (throw \\='exit nil).\n\nThe following values (last argument to `throw') can be used when\nthrowing to \\='exit:\n\n- t causes `recursive-edit' to quit, so that control returns to the\n  command loop one level up.\n\n- A string causes `recursive-edit' to signal an error, printing that\n  string as the error message.\n\n- A function causes `recursive-edit' to call that function with no\n  arguments, and then return normally.\n\n- Any other value causes `recursive-edit' to return normally to the\n  function that called it.\n\nThis function is called by the editor initialization to begin editing.",
       (),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  let buffer: Lisp_Object;

  /* If we enter while input is blocked, don't lock up here.
     This may happen through the debugger during redisplay.  */
  if input_blocked_p ()
    { return Qnil (); }

  if command_loop_level >= 0
      && current_buffer != XBUFFER ((*XWINDOW (selected_window)).contents)
    { buffer = Fcurrent_buffer (); }
  else
    { buffer = Qnil (); }

  /* Don't do anything interesting between the increment and the
     record_unwind_protect!  Otherwise, we could get distracted and
     never decrement the counter again.  */
  command_loop_level += 1;
  update_mode_lines = 17;
  record_unwind_protect (Some (recursive_edit_unwind), buffer);

  /* If we leave recursive_edit_1 below with a `throw' for instance,
     like it is done in the splash screen display, we have to
     make sure that we restore single_kboard as command_loop_1
     would have done if it were left normally.  */
  if command_loop_level > 0 && !multiple_terminals_merge_keyboards
    { temporarily_switch_to_single_kboard (SELECTED_FRAME () as *mut c_void); }

  recursive_edit_1 ();
  return unbind_to (count, Qnil ());
       });

/* C 810-819.  `static void recursive_edit_unwind (Lisp_Object);` at C 321 gives the
   definition internal linkage (command_loop above says why).  */
unsafe extern "C" fn recursive_edit_unwind (buffer: Lisp_Object)
{
  if BUFFERP (buffer)
    { Fset_buffer (buffer); }

  command_loop_level -= 1;
  update_mode_lines = 18;
}

/* C 824-831: `not_single_kboard_state`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn not_single_kboard_state (kboard: *mut KBOARD)
{
  if kboard == current_kboard
    { single_kboard = false; }
}

/* C 833-841: the kboard stack record.  */
/* keyboard.c 833-841: `struct kboard_stack { KBOARD *kboard; struct kboard_stack *next; };`  */
#[repr(C)]
pub struct kboard_stack
{
  pub kboard: *mut KBOARD,
  pub next: *mut kboard_stack,
}

/* C 843: `static struct kboard_stack *kboard_stack;`  */
static mut kboard_stack: *mut kboard_stack = std::ptr::null_mut ();

/* C 845-858.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn push_kboard (k: *mut kboard)
{
  let p: *mut kboard_stack = xmalloc (std::mem::size_of::<kboard_stack> ()) as *mut kboard_stack;

  (*p).next = kboard_stack;
  (*p).kboard = current_kboard;
  kboard_stack = p;

  current_kboard = k;
}

/* C 860-887.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_kboard ()
{
  let mut t: *mut terminal;
  let p: *mut kboard_stack = kboard_stack;
  let mut found: bool = false;
  t = terminal_list;
  while !t.is_null ()
    {
      if (*t).kboard == (*p).kboard as *mut c_void
	{
	  current_kboard = (*p).kboard;
	  found = true;
	  break;
	}
      t = (*t).next_terminal;
    }
  if !found
    {
      /* The terminal we remembered has been deleted.  */
      current_kboard = FRAME_KBOARD (SELECTED_FRAME ()) as *mut KBOARD;
      single_kboard = false;
    }
  kboard_stack = (*p).next;
  xfree (p as *mut c_void);
}

/* Switch to single_kboard mode, making current_kboard the only KBOARD
  from which further input is accepted.  If F is non-nil, set its
  KBOARD as the current keyboard.

  This function uses record_unwind_protect_int to return to the previous
  state later.

  If Emacs is already in single_kboard mode, and F's keyboard is
  locked, then this function will throw an error.  */
/* `keyboard.h` 483 declares `temporarily_switch_to_single_kboard` at the crate root
   (`include/keyboard.rs`'s `unsafe extern "C"` block), so its definition lives in this module
   and is glob-imported below: a crate-root definition would be `E0428` (the frame.rs
   `frame_globals` shape, PORTING.md P-21).  The same holds for `kbd_buffer_store_buffered_event`
   (keyboard.h 495) further down.  */
pub mod keyboard_root_shadowed {
  use super::*;

  /* C 889-925.  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn temporarily_switch_to_single_kboard (f: *mut frame)
  {
    let was_locked: bool = single_kboard;
    if was_locked
      {
        if !f.is_null () && FRAME_KBOARD (f) as *mut KBOARD != current_kboard
  	/* We can not switch keyboards while in single_kboard mode.
  	   In rare cases, Lisp code may call `recursive-edit' (or
  	   `read-minibuffer' or `y-or-n-p') after it switched to a
  	   locked frame.  For example, this is likely to happen
  	   when server.el connects to a new terminal while Emacs is in
  	   single_kboard mode.  It is best to throw an error instead
  	   of presenting the user with a frozen screen.  */
  	{ error (c"Terminal %d is locked, cannot read from it".as_ptr (),
  		 (*FRAME_TERMINAL! (f)).id); }
        else
  	/* This call is unnecessary, but helps
  	   `restore_kboard_configuration' discover if somebody changed
  	   `current_kboard' behind our back.  */
  	{ push_kboard (current_kboard); }
      }
    else if !f.is_null ()
      { current_kboard = FRAME_KBOARD (f) as *mut KBOARD; }
    single_kboard = true;
    record_unwind_protect_int (Some (restore_kboard_configuration), was_locked as c_int);
  }
}
use keyboard_root_shadowed::*;

/* C 927-938.  */
unsafe extern "C" fn restore_kboard_configuration (was_locked: c_int)
{
  single_kboard = was_locked != 0;
  if single_kboard
    {
      let prev: *mut kboard = current_kboard;
      pop_kboard ();
      /* The pop should not change the kboard.  */
      if single_kboard && current_kboard != prev
	{ emacs_abort (); }
    }
}

/* Handle errors that are not handled at inner levels
   by printing an error message and returning to the editor command loop.  */
/* C 944-1001.  */
unsafe extern "C" fn cmd_error (data: Lisp_Object) -> Lisp_Object
{
  let mut old_level: Lisp_Object;
  let mut old_length: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let conditions: Lisp_Object;
  /* gnulib's `INT_STRLEN_BOUND (t)` (intprops.h 88-108) is
     `((TYPE_WIDTH (t) - signed) * 146 + 484) / 485 + signed`; `<intprops.h>` has no mirror
     (P-26), so this is the instantiation `INT_STRLEN_BOUND (EMACS_INT)` = 20 (the
     emacs.rs/data.rs shape for the same macro).  */
  let mut macroerror: [c_char; 30 + INT_STRLEN_BOUND] = [0; 30 + INT_STRLEN_BOUND];

  /* C 951-955: `#ifdef HAVE_WINDOW_SYSTEM`'s `if (display_hourglass_p) cancel_hourglass ();`
     (carried per P-51).  */
  #[cfg (feature = "have_window_system")]
  if display_hourglass_p { cancel_hourglass (); }

  if !NILP (executing_kbd_macro)
    {
      if executing_kbd_macro_iterations == 1
	{ sprintf (macroerror.as_mut_ptr (), c"After 1 kbd macro iteration: ".as_ptr ()); }
      else
	{ sprintf (macroerror.as_mut_ptr (),
		   c"After %ld kbd macro iterations: ".as_ptr (),
		   executing_kbd_macro_iterations); }
    }
  else
    { macroerror[0] = 0; }

  conditions = Fget (XCAR (data), Qerror_conditions ());
  if NILP (Fmemq (Qminibuffer_quit (), conditions))
    {
      Vexecuting_kbd_macro = Qnil ();
      executing_kbd_macro = Qnil ();
    }
  else if !NILP (KVAR! (current_kboard, defining_kbd_macro))
    /* An `M-x' command that signals a `minibuffer-quit' condition
       that's part of a kbd macro.  */
    { finalize_kbd_macro_chars (); }

  specbind (Qstandard_output (), Qt ());
  specbind (Qstandard_input (), Qt ());
  kset_prefix_arg (current_kboard, Qnil ());
  kset_last_prefix_arg (current_kboard, Qnil ());
  cancel_echoing ();

  /* Avoid unquittable loop if data contains a circular list.  */
  old_level = Vprint_level;
  old_length = Vprint_length;
  XSETFASTINT! (Vprint_level, 10);
  XSETFASTINT! (Vprint_length, 10);
  cmd_error_internal (data, macroerror.as_ptr ());
  Vprint_level = old_level;
  Vprint_length = old_length;

  Vquit_flag = Qnil ();
  Vinhibit_quit = Qnil ();

  unbind_to (count, Qnil ());
  return make_fixnum (0);
}

/* Take actions on handling an error.  DATA is the data that describes
   the error.

   CONTEXT is a C-string containing ASCII characters only which
   describes the context in which the error happened.  If we need to
   generalize CONTEXT to allow multibyte characters, make it a Lisp
   string.  */
/* C 1011-1032.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_error_internal (data: Lisp_Object, context: *const c_char)
{
  /* The immediate context is not interesting for Quits,
     since they are asynchronous.  */
  if signal_quit_p (data)
    { Vsignaling_function = Qnil (); }

  Vquit_flag = Qnil ();
  Vinhibit_quit = Qt ();

  /* Use user's specified output function if any.  */
  if !NILP (Vcommand_error_function)
    { calln! (Vcommand_error_function, data,
	      if !context.is_null () { build_string (context) } else { empty_unibyte_string },
	      Vsignaling_function); }

  Vsignaling_function = Qnil ();
}

/* C 1034-1087: `DEFUN ("command-error-default-function", …)`.  */
DEFUN! ("command-error-default-function", Fcommand_error_default_function, Scommand_error_default_function, 3, 3, 0,
       doc: "Produce default output for unhandled error message.\nDefault value of `command-error-function'.",
       (data: Lisp_Object, context: Lisp_Object, signal: Lisp_Object),
       {
  let sf: *mut frame = SELECTED_FRAME ();
  let conditions: Lisp_Object = Fget (XCAR (data), Qerror_conditions ());
  let is_minibuffer_quit: c_int = if !NILP (Fmemq (Qminibuffer_quit (), conditions)) { 1 } else { 0 };

  CHECK_STRING (context);

  /* If the window system or terminal frame hasn't been initialized
     yet, or we're not interactive, write the message to stderr and exit.
     Don't do this for the minibuffer-quit condition.  */
  if is_minibuffer_quit == 0
      && (!frame_bits! (glyphs_initialized_p, sf)
	  /* The initial frame is a special non-displaying frame. It
	     will be current in daemon mode when there are no frames
	     to display, and in non-daemon mode before the real frame
	     has finished initializing.  If an error is thrown in the
	     latter case while creating the frame, then the frame
	     will never be displayed, so the safest thing to do is
	     write to stderr and quit.  In daemon mode, there are
	     many other potential errors that do not prevent frames
	     from being created, so continuing as normal is better in
	     that case, as long as the daemon has actually finished
	     initialization. */
	  || (!(IS_DAEMON () && !DAEMON_RUNNING ()) && FRAME_INITIAL_P (sf))
	  || noninteractive)
    {
      print_error_message (data, Qexternal_debugging_output (), SSDATA (context), signal);
      Fterpri (Qexternal_debugging_output (), Qnil ());
      Fkill_emacs (make_fixnum (-1), Qnil ());
    }
  else
    {
      clear_message (true, false);
      message_log_maybe_newline ();

      if is_minibuffer_quit != 0
	{
	  Fding (Qt ());
	}
      else
	{
	  Fdiscard_input ();
	  bitch_at_user ();
	}

      print_error_message (data, Qt (), SSDATA (context), signal);
    }
  return Qnil ();
       });

/* Entry to editor-command-loop.
   This level has the catches for exiting/returning to editor command loop.
   It returns nil to exit recursive edit, t to abort it.  */
/* C 1091-1153.  The *declaration* at C 323 spells `static Lisp_Object command_loop (void);`
   while the definition at C 1113 does not repeat `static`; in C the earlier `static`
   declaration fixes internal linkage for the whole unit (C11 6.2.2p4), and the reference
   object exports no `command_loop` — `tools/check-symbols.py` reported one until this
   definition lost its `#[unsafe(no_mangle)]` (the fns.c `concat_to_list` shape; the
   `linkage` row to add to docs/ports/excluded-macros.txt is in this lane's report).  */
unsafe extern "C" fn command_loop () -> Lisp_Object
{
  /* C 1093-1107: `#ifdef HAVE_STACK_OVERFLOW_HANDLING` is live (macro table 1).  */
  /* At least on GNU/Linux, saving signal mask is important here.  */
  if sigsetjmp (&raw mut return_to_command_loop as *mut c_void, 1) != 0
    {
      /* Comes here from handle_sigsegv (see sysdep.c) and
	 stack_overflow_handler (see w32fns.c).  */
      /* C 1098-1100: `#ifdef WINDOWSNT`'s `w32_reset_stack_overflow_guard ();` (carried
	 per P-51).  */
      #[cfg (feature = "windowsnt")]
      { w32_reset_stack_overflow_guard (); }
      init_eval ();
      Vinternal__top_level_message = recover_top_level_message;
    }
  else
    { Vinternal__top_level_message = regular_top_level_message; }
  if command_loop_level > 0 || minibuf_level > 0
    {
      let val: Lisp_Object;
      val = internal_catch (Qexit (), command_loop_2, Qerror ());
      executing_kbd_macro = Qnil ();
      return val;
    }
  else
    {
      loop
	{
	  internal_catch (Qtop_level (), top_level_1, Qnil ());
	  internal_catch (Qtop_level (), command_loop_2, Qerror ());
	  executing_kbd_macro = Qnil ();

	  /* End of file in -batch run causes exit here.  */
	  if noninteractive
	    { Fkill_emacs (Qt (), Qnil ()); }
	}
    }
}

/* Here we catch errors in execution of commands within the
   editing loop, and reenter the editing loop.
   When there is an error, cmd_error runs and returns a non-nil
   value to us.  A value of nil means that command_loop_1 itself
   returned due to end of file (or end of kbd macro).  HANDLERS is a
   list of condition names, passed to internal_condition_case.  */
/* C 1155-1168.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn command_loop_2 (handlers: Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object;

  loop
    {
      val = internal_condition_case (command_loop_1, handlers, cmd_error);
      if NILP (val) { break; }
    }

  return Qnil ();
}

/* C 1170-1185.  */
unsafe extern "C" fn top_level_2 () -> Lisp_Object
{
  /* If we're in batch mode, print a backtrace unconditionally when
     encountering an error, to help with debugging.  */
  let setup_handler: bool = noninteractive;
  if setup_handler
    /* FIXME: Should we (re)use `list_of_error` from `xdisp.c`? */
    { push_handler_bind (list1 (Qerror ()), Qdebug_early__handler (), 0); }

  let res: Lisp_Object = Feval (Vtop_level, Qt ());

  if setup_handler
    { pop_handler (); }
  return res;
}

/* C 1187-1197.  */
unsafe extern "C" fn top_level_1 (ignore: Lisp_Object) -> Lisp_Object
{
  /* On entry to the outer level, run the startup file.  */
  if !NILP (Vtop_level)
    { internal_condition_case (top_level_2, Qerror (), cmd_error); }
  else if !NILP (Vpurify_flag)
    { message1 (c"Bare impure Emacs (standard Lisp code not loaded)".as_ptr ()); }
  else
    { message1 (c"Bare Emacs (standard Lisp code not loaded)".as_ptr ()); }
  return Qnil ();
}

/* C 1199-1216: `DEFUN ("top-level", …)`.  */
DEFUN! ("top-level", Ftop_level, Stop_level, 0, 0, "",
       doc: "Exit all recursive editing levels.\nThis also exits all active minibuffers.",
       attributes: noreturn,
       (),
       {
  /* C 1205-1208: `#ifdef HAVE_WINDOW_SYSTEM`'s `if (display_hourglass_p) cancel_hourglass ();`
     (carried per P-51).  */
  #[cfg (feature = "have_window_system")]
  if display_hourglass_p { cancel_hourglass (); }

  /* Unblock input if we enter with input blocked.  This may happen if
     redisplay traps e.g. during tool-bar update with input blocked.  */
  totally_unblock_input ();

  Fthrow (Qtop_level (), Qnil ());
       });

/* C 1218-1221.  */
unsafe fn user_error (msg: *const c_char) -> !
{
  xsignal1 (Quser_error (), build_string (msg));
}

/* C 1223-1232: `DEFUN ("exit-recursive-edit", …)`.  */
DEFUN! ("exit-recursive-edit", Fexit_recursive_edit, Sexit_recursive_edit, 0, 0, "",
       doc: "Exit from the innermost recursive edit or minibuffer.",
       attributes: noreturn,
       (),
       {
  if command_loop_level > 0 || minibuf_level > 0
    { Fthrow (Qexit (), Qnil ()); }

  user_error (c"No recursive edit is in progress".as_ptr ());
       });

/* C 1234-1247: `DEFUN ("abort-recursive-edit", …)`.  */
DEFUN! ("abort-recursive-edit", Fabort_recursive_edit, Sabort_recursive_edit, 0, 0, "",
       doc: "Abort the command that requested this recursive edit or minibuffer input.",
       attributes: noreturn,
       (),
       {
  if command_loop_level > 0 || minibuf_level > 0
    { Fthrow (Qexit (), Qt ()); }

  user_error (c"No recursive edit is in progress".as_ptr ());
       });

/* Restore mouse tracking enablement.  See Finternal_track_mouse for
   the only use of this function.  */
/* C 1253-1265.  */
unsafe extern "C" fn tracking_off (old_track_mouse: Lisp_Object)
{
  track_mouse = old_track_mouse;
  if NILP (old_track_mouse)
    {
      /* Redisplay may have been preempted because there was input
	 available, and it assumes it will be called again after the
	 input has been processed.  If the only input available was
	 the sort that we have just disabled, then we need to call
	 redisplay.  */
      if !readable_events (READABLE_EVENTS_DO_TIMERS_NOW)
	{
	  redisplay_preserve_echo_area (6);
	  get_input_pending (READABLE_EVENTS_DO_TIMERS_NOW);
	}
    }
}

/* C 1267-1284: `DEFUN ("internal--track-mouse", …)`.  */
DEFUN! ("internal--track-mouse", Finternal_track_mouse, Sinternal_track_mouse, 1, 1, 0,
       doc: "Call BODYFUN with mouse movement events enabled.",
       (bodyfun: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  let val: Lisp_Object;

  record_unwind_protect (Some (tracking_off), track_mouse);

  track_mouse = Qt ();

  val = call0 (bodyfun);
  return unbind_to (count, val);
       });

/* If mouse has moved on some frame and we are tracking the mouse,
   return one of those frames.  Return NULL otherwise.

   If ignore_mouse_drag_p is non-zero, ignore (implicit) mouse movement
   after resizing the tool-bar window.  */
/* C 1292-1308.  */
unsafe fn some_mouse_moved () -> *mut frame
{
  let mut tail: Lisp_Object;
  let mut frame_: Lisp_Object;

  if NILP (track_mouse) || ignore_mouse_drag_p
    { return std::ptr::null_mut (); }

  FOR_EACH_FRAME! (tail, frame_,
    {
      if frame_bits! (mouse_moved, XFRAME (frame_))
	{ return XFRAME (frame_); }
    });

  return std::ptr::null_mut ();
}

/* This is the actual command reading loop,
   sans error-handling encapsulation.  */
/* C 1319: `enum { READ_KEY_ELTS = 30 };`  */
pub const READ_KEY_ELTS: c_int = 30;

/* C 1328-1690.  */
unsafe extern "C" fn command_loop_1 () -> Lisp_Object
{
  let mut prev_modiff: modiff_count = 0;
  /* C 1324's local `prev_buffer` shadows the file-scope `prev_buffer` (C 269); Rust forbids a
     `let` binding named the same as a static (`E0530`), so the port keeps the name with the
     trailing underscore the token comparison strips (PORTING.md 0.2's `echo_string_` shape).  */
  let mut prev_buffer_: *mut buffer = std::ptr::null_mut ();

  kset_prefix_arg (current_kboard, Qnil ());
  kset_last_prefix_arg (current_kboard, Qnil ());
  Vdeactivate_mark = Qnil ();
  waiting_for_input! () = false;
  cancel_echoing ();

  this_command_key_count = 0;
  this_single_command_key_start = 0;

  if NILP (Vmemory_full)
    {
      /* Make sure this hook runs after commands that get errors and
	 throw to top level.  */
      /* Note that the value cell will never directly contain nil
	 if the symbol is a local variable.  */
      if !NILP (Vpost_command_hook) && !NILP (Vrun_hooks)
	{ safe_run_hooks_maybe_narrowed (Qpost_command_hook (), XWINDOW (selected_window)); }

      /* If displaying a message, resize the echo area window to fit
	 that message's size exactly.  */
      if !NILP (echo_area_buffer[0])
	{ resize_echo_area_exactly (); }

      /* If there are warnings waiting, process them.  */
      if !NILP (Vdelayed_warnings_list)
	{ safe_run_hooks (Qdelayed_warnings_hook ()); }
    }

  /* Do this after running Vpost_command_hook, for consistency.  */
  kset_last_command (current_kboard, Vthis_command);
  kset_real_last_command (current_kboard, Vreal_this_command);
  if !CONSP (last_command_event)
    { kset_last_repeatable_command (current_kboard, Vreal_this_command); }

  loop
    {
    'finalize: {
      let mut cmd: Lisp_Object;

      if !FRAME_LIVE_P (XFRAME (selected_frame))
	{ Fkill_emacs (Qnil (), Qnil ()); }

      /* Make sure the current window's buffer is selected.  */
      set_buffer_internal (XBUFFER ((*XWINDOW (selected_window)).contents));

      /* Display any malloc warning that just came out.  Use while because
	 displaying one warning can cause another.  */

      while !pending_malloc_warning.is_null ()
	{ display_malloc_warning (); }

      Vdeactivate_mark = Qnil ();

      /* Don't ignore mouse movements for more than a single command
	 loop.  (This flag is set in xdisp.c whenever the tool bar is
	 resized, because the resize moves text up or down, and would
	 generate false mouse drag events if we don't ignore them.)  */
      ignore_mouse_drag_p = false;

      /* If minibuffer on and echo area in use,
	 wait a short time and redraw minibuffer.  */

      if minibuf_level != 0
	  && !NILP (echo_area_buffer[0])
	  && BASE_EQ (minibuf_window, echo_area_window)
	  && NUMBERP (Vminibuffer_message_timeout)
	{
	  /* Bind inhibit-quit to t so that C-g gets read in
	     rather than quitting back to the minibuffer.  */
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  specbind (Qinhibit_quit (), Qt ());

	  sit_for (Vminibuffer_message_timeout, false, 2);

	  /* Clear the echo area.  */
	  message1 (std::ptr::null ());
	  safe_run_hooks (Qecho_area_clear_hook ());

	  /* We cleared the echo area, and the minibuffer will now
	     show, so resize the mini-window in case the minibuffer
	     needs more or less space than the echo area.  */
	  resize_mini_window (XWINDOW (minibuf_window), false);

	  unbind_to (count, Qnil ());

	  /* If a C-g came in before, treat it as input now.  */
	  if !NILP (Vquit_flag)
	    {
	      Vquit_flag = Qnil ();
	      Vunread_command_events = list1i (quit_char as EMACS_INT);
	    }
	}

      Vthis_command = Qnil ();
      Vreal_this_command = Qnil ();
      Vthis_original_command = Qnil ();
      Vthis_command_keys_shift_translated = Qnil ();

      /* Read next key sequence; i gets its length.  */
      raw_keybuf_count = 0;
      let mut keybuf: [Lisp_Object; READ_KEY_ELTS as usize] = [lisp_h_Qnil (); READ_KEY_ELTS as usize];
      let i: c_int = read_key_sequence (keybuf.as_mut_ptr (), Qnil (), false, true, true, false,
					false);

      /* A filter may have run while we were reading the input.  */
      if !FRAME_LIVE_P (XFRAME (selected_frame))
	{ Fkill_emacs (Qnil (), Qnil ()); }
      set_buffer_internal (XBUFFER ((*XWINDOW (selected_window)).contents));

      num_input_keys += 1;

      /* Now we have read a key sequence of length I,
	 or else I is 0 and we found end of file.  */

      if i == 0		/* End of file -- happens only in */
	{ return Qnil (); }	/* a kbd macro, at the end.  */
      /* -1 means read_key_sequence got a menu that was rejected.
	 Just loop around and read another command.  */
      if i == -1
	{
	  cancel_echoing ();
	  this_command_key_count = 0;
	  this_single_command_key_start = 0;
	  /* C 1465: `goto finalize;` — the port breaks out of the labelled block whose tail is
	     C's `finalize:` label (PORTING.md 2.2's forward-goto shape).  */
	  break 'finalize;
	}

      last_command_event = keybuf[(i - 1) as usize];

      /* If the previous command tried to force a specific window-start,
	 forget about that, in case this command moves point far away
	 from that position.  But also throw away beg_unchanged and
	 end_unchanged information in that case, so that redisplay will
	 update the whole window properly.  */
      if window_bits! (force_start, XWINDOW (selected_window))
	{
	  let mut b: *mut buffer;
	  window_bits! (force_start, XWINDOW (selected_window), false);
	  b = XBUFFER ((*XWINDOW (selected_window)).contents);
	  BUF_BEG_UNCHANGED! (b) = 0;
	  BUF_END_UNCHANGED! (b) = 0;
	}

      cmd = read_key_sequence_cmd;
      if !NILP (Vexecuting_kbd_macro)
	{
	  if !NILP (Vquit_flag)
	    {
	      Vexecuting_kbd_macro = Qt ();
	      maybe_quit ();	/* Make some noise.  */
				/* Will return since macro now empty.  */
	    }
	}

      /* Do redisplay processing after this command except in special
	 cases identified below.  */
      prev_buffer_ = current_buffer;
      prev_modiff = MODIFF! ();
      last_point_position = PT! ();
      let last_pt: ptrdiff_t = PT! ();

      /* By default, we adjust point to a boundary of a region that
         has such a property that should be treated intangible
         (e.g. composition, display).  But, some commands will set
         this variable differently.  */
      Vdisable_point_adjustment = Qnil ();

      /* Process filters and timers may have messed with deactivate-mark.
	 reset it before we execute the command.  */
      Vdeactivate_mark = Qnil ();

      /* Remap command through active keymaps.  */
      Vthis_original_command = cmd;
      if !NILP (read_key_sequence_remapped)
	{ cmd = read_key_sequence_remapped; }

      /* Execute the command.  */

      {
	total_keys += if total_keys < lossage_limit { 1 } else { 0 };
	ASET (recent_keys, recent_keys_index as isize, Fcons (Qnil (), cmd));
	recent_keys_index += 1;
	if recent_keys_index >= lossage_limit
	  { recent_keys_index = 0; }
      }
      Vthis_command = cmd;
      Vreal_this_command = cmd;

      safe_run_hooks_maybe_narrowed (Qpre_command_hook (), XWINDOW (selected_window));

      if NILP (Vthis_command)
	/* nil means key is undefined.  */
	{ call0 (Qundefined ()); }
      else
	{
	  /* Here for a command that isn't executed directly.  */

	  /* C 1530-1541: `#ifdef HAVE_WINDOW_SYSTEM`'s hourglass arm (carried per P-51).  */
	  #[cfg (feature = "have_window_system")]
	  let scount: specpdl_ref = {
	      let scount = SPECPDL_INDEX ();
	      if display_hourglass_p && NILP (Vexecuting_kbd_macro)
		{
		  record_unwind_protect_void (Some (cancel_hourglass));
		  start_hourglass ();
		}
	      scount
	    };

            /* Ensure that we have added appropriate undo-boundaries as a
               result of changes from the last command. */
            call0 (Qundo_auto__add_boundary ());

            /* Record point and buffer, so we can put point into the undo
               information if necessary. */
            point_before_last_command_or_undo = PT! ();
            buffer_before_last_command_or_undo = current_buffer;

	    /* Restart our counting of redisplay ticks before
	       executing the command, so that we don't blame the new
	       command for the sins of the previous one.  */
	    update_redisplay_ticks (0, std::ptr::null_mut ());
	    display_working_on_window_p = false;

            calln! (Qcommand_execute (), Vthis_command);
	    display_working_on_window_p = false;

	  /* C 1556-1563: `#ifdef HAVE_WINDOW_SYSTEM` (carried per P-51).  */
	  #[cfg (feature = "have_window_system")]
	  if NILP (Vexecuting_kbd_macro)
	    { unbind_to (scount, Qnil ()); }
          }
      /* Restore last PT position value, possibly clobbered by
         recursive-edit invoked by the command we just executed.  */
      last_point_position = last_pt;
      kset_last_prefix_arg (current_kboard, Vcurrent_prefix_arg);

      safe_run_hooks_maybe_narrowed (Qpost_command_hook (), XWINDOW (selected_window));

      /* If displaying a message, resize the echo area window to fit
	 that message's size exactly.  Do this only if the echo area
	 window is the minibuffer window of the selected frame.  See
	 Bug#34317.  */
      if !NILP (echo_area_buffer[0])
	  && (EQ (echo_area_window,
		  FRAME_MINIBUF_WINDOW (XFRAME (selected_frame))))
	{ resize_echo_area_exactly (); }

      /* If there are warnings waiting, process them.  */
      if !NILP (Vdelayed_warnings_list)
	{ safe_run_hooks (Qdelayed_warnings_hook ()); }

      kset_last_command (current_kboard, Vthis_command);
      kset_real_last_command (current_kboard, Vreal_this_command);
      if !CONSP (last_command_event)
	{ kset_last_repeatable_command (current_kboard, Vreal_this_command); }

      this_command_key_count = 0;
      this_single_command_key_start = 0;

      if kboard_bits! (immediate_echo, current_kboard)
	  && !NILP (call0 (Qinternal_echo_keystrokes_prefix ()))
	{
	  kboard_bits! (immediate_echo, current_kboard, false);
	  /* Refresh the echo message.  */
	  echo_now ();
	}
      else
	{ cancel_echoing (); }

      if !NILP (BVAR! (current_buffer, mark_active_))
	  && !NILP (Vrun_hooks)
	{
	  /* In Emacs 22, setting transient-mark-mode to `only' was a
	     way of turning it on for just one command.  This usage is
	     obsolete, but support it anyway.  */
	  if EQ (Vtransient_mark_mode, Qidentity ())
	    { Vtransient_mark_mode = Qnil (); }
	  else if EQ (Vtransient_mark_mode, Qonly ())
	    { Vtransient_mark_mode = Qidentity (); }

	  if !NILP (Vdeactivate_mark)
	    /* If `select-active-regions' is non-nil, this call to
	       `deactivate-mark' also sets the PRIMARY selection.  */
	    { call0 (Qdeactivate_mark ()); }
	  else
	    {
	      let mut symval: Lisp_Object;
	      /* Even if not deactivating the mark, set PRIMARY if
		 `select-active-regions' is non-nil.  */
	      if (!NILP (Fwindow_system (Qnil ()))
		   || { symval = find_symbol_value (Qtty_select_active_regions ());
			!BASE_EQ (symval, Qunbound ()) && !NILP (symval) }
		      && !NILP (Fterminal_parameter (Qnil (), Qxterm__set_selection ())))
		  /* Even if mark_active is non-nil, the actual buffer
		     marker may not have been set yet (Bug#7044).  */
		  && !(*XMARKER (BVAR! (current_buffer, mark_))).buffer.is_null ()
		  && (if EQ (Vselect_active_regions, Qonly ())
			{ EQ (CAR_SAFE (Vtransient_mark_mode), Qonly ()) }
		      else
			{ !NILP (Vselect_active_regions) && !NILP (Vtransient_mark_mode) })
		  && NILP (Fmemq (Vthis_command, Vselection_inhibit_update_commands))
		{
		  let txt: Lisp_Object = calln! (Vregion_extract_function, Qnil ());

		  if XFIXNUM (Flength (txt)) > 0
		    /* Don't set empty selections.  */
		    { calln! (Qgui_set_selection (), QPRIMARY (), txt); }

		  CALLN! (Frun_hook_with_args, Qpost_select_region_hook (), txt);
		}

	      if current_buffer != prev_buffer_ || MODIFF! () != prev_modiff
		{ run_hook (Qactivate_mark_hook ()); }
	    }

	  Vsaved_region_selection = Qnil ();
	}
    } /* 'finalize */

      /* finalize: */
      if current_buffer == prev_buffer_
	  && XBUFFER ((*XWINDOW (selected_window)).contents) == current_buffer
	  && last_point_position != PT! ()
	{
	  if NILP (Vdisable_point_adjustment)
	      && NILP (Vglobal_disable_point_adjustment)
	      && !composition_break_at_point
	    {
	      if last_point_position > BEGV! ()
		  && last_point_position < ZV! ()
		  && (composition_adjust_point (last_point_position, last_point_position)
		      != last_point_position)
		/* The last point was temporarily set within a grapheme
		   cluster to prevent automatic composition.  To recover
		   the automatic composition, we must update the
		   display.  */
		{ windows_or_buffers_changed = 21; }
	      adjust_point_for_property (last_point_position,
					 MODIFF! () != prev_modiff);
	    }
	  else if PT! () > BEGV! () && PT! () < ZV! ()
		   && (composition_adjust_point (last_point_position, PT! ())
		       != PT! ())
	    /* Now point is within a grapheme cluster.  We must update
	       the display so that this cluster is de-composed on the
	       screen and the cursor is correctly placed at point.  */
	    { windows_or_buffers_changed = 39; }
	}

      /* Install chars successfully executed in kbd macro.  */

      if !NILP (KVAR! (current_kboard, defining_kbd_macro))
	  && NILP (KVAR! (current_kboard, Vprefix_arg))
	{ finalize_kbd_macro_chars (); }
    }
}

/* C 1690-1712.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_menu_command () -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* We don't want to echo the keystrokes while navigating the
     menus.  */
  specbind (Qecho_keystrokes (), make_fixnum (0));

  let mut keybuf: [Lisp_Object; READ_KEY_ELTS as usize] = [lisp_h_Qnil (); READ_KEY_ELTS as usize];
  let i: c_int = read_key_sequence (keybuf.as_mut_ptr (), Qnil (), false, true, true, true,
				    false);

  unbind_to (count, Qnil ());

  if !FRAME_LIVE_P (XFRAME (selected_frame))
    { Fkill_emacs (Qnil (), Qnil ()); }
  if i == 0 || i == -1
    { return Qt (); }

  return read_key_sequence_cmd;
}

/* Adjust point to a boundary of a region that has such a property
   that should be treated intangible.  For the moment, we check
   `composition', `display' and `invisible' properties.
   LAST_PT is the last position of point.  */
/* C 1719-1896.  */
unsafe fn adjust_point_for_property (last_pt: ptrdiff_t, modified: bool)
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5): rustc's definite
     assignment cannot see the `&&` chain's ordering.  */
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut val: Lisp_Object = lisp_h_Qnil ();
  let mut overlay: Lisp_Object = lisp_h_Qnil ();
  let mut tmp: Lisp_Object = lisp_h_Qnil ();
  /* When called after buffer modification, we should temporarily
     suppress the point adjustment for automatic composition so that a
     user can keep inserting another character at point or keep
     deleting characters around point.  */
  let mut check_composition: bool = ! modified;
  let mut check_display: bool = true;
  let mut check_invisible: bool = true;
  let mut orig_pt: ptrdiff_t = PT! ();
  let mut pt_before_invis: ptrdiff_t = PT! ();

  eassert! (XBUFFER ((*XWINDOW (selected_window)).contents) == current_buffer);

  /* FIXME: cycling is probably not necessary because these properties
     can't be usefully combined anyway.  */
  while check_composition || check_display || check_invisible
    {
      /* FIXME: check `intangible'.  */
      if check_composition
	  && PT! () > BEGV! () && PT! () < ZV! ()
	  && { beg = composition_adjust_point (last_pt, PT! ()); beg != PT! () }
	{
	  SET_PT (beg);
	  check_display = true;
	  check_invisible = true;
	}
      check_composition = false;
      if check_display
	  && PT! () > BEGV! () && PT! () < ZV! ()
	  && { val = get_char_property_and_overlay (make_fixnum (PT! ()), Qdisplay (),
						    selected_window, &raw mut overlay);
	       !NILP (val) }
	  && display_prop_intangible_p (val, overlay, PT! (), PT_BYTE! ())
	  && (if !OVERLAYP (overlay)
		{ get_property_and_range (PT! (), Qdisplay (), &raw mut val, &raw mut beg,
					  &raw mut end, Qnil ()) }
	      else
		{ beg = OVERLAY_START (overlay);
		  end = OVERLAY_END (overlay);
		  true })
	  && (beg < PT! () /* && end > PT   <- It's always the case.  */
	      || (beg <= PT! () && STRINGP (val) && SCHARS (val) == 0))
	{
	  eassert! (end > PT! ());
	  SET_PT (if PT! () < last_pt
		   { if STRINGP (val) && SCHARS (val) == 0
		     { max! (beg - 1, BEGV! ()) } else { beg } }
		   else { end });
	  check_composition = true;
	  check_invisible = true;
	}
      check_display = false;
      pt_before_invis = PT! ();
      if check_invisible && PT! () > BEGV! () && PT! () < ZV! ()
	{
	  let mut inv: c_int;
	  let mut ellipsis: bool = false;
	  beg = PT! ();
	  end = PT! ();

	  /* Find boundaries `beg' and `end' of the invisible area, if any.  */
	  while end < ZV! ()
		/* C 1755-1768: the `#if 0` block (FIXME text) is dead text `extract-abi.py`'s
		   `dead_branch_map` already sees as dead; nothing to carry.  */
		&& !NILP ({ val = get_char_property_and_overlay
				   (make_fixnum (end), Qinvisible (),
				    selected_window, &raw mut overlay);
			    val })
		&& { inv = TEXT_PROP_MEANS_INVISIBLE! (val); inv != 0 }
	    {
	      ellipsis = ellipsis || inv > 1
		|| (OVERLAYP (overlay)
		    && (!NILP (Foverlay_get (overlay, Qafter_string ()))
			|| !NILP (Foverlay_get (overlay, Qbefore_string ()))));
	      tmp = Fnext_single_char_property_change
		(make_fixnum (end), Qinvisible (), Qnil (), Qnil ());
	      end = if FIXNATP (tmp) { XFIXNAT (tmp) } else { ZV! () };
	    }
	  while beg > BEGV! ()
		&& !NILP ({ val = get_char_property_and_overlay
				   (make_fixnum (beg - 1), Qinvisible (),
				    selected_window, &raw mut overlay);
			    val })
		&& { inv = TEXT_PROP_MEANS_INVISIBLE! (val); inv != 0 }
	    {
	      ellipsis = ellipsis || inv > 1
		|| (OVERLAYP (overlay)
		    && (!NILP (Foverlay_get (overlay, Qafter_string ()))
			|| !NILP (Foverlay_get (overlay, Qbefore_string ()))));
	      tmp = Fprevious_single_char_property_change
		(make_fixnum (beg), Qinvisible (), Qnil (), Qnil ());
	      beg = if FIXNATP (tmp) { XFIXNAT (tmp) } else { BEGV! () };
	    }

	  /* Move away from the inside area.  */
	  if beg < PT! () && end > PT! ()
	    {
	      SET_PT (if orig_pt == PT! () && (last_pt < beg || last_pt > end)
		       /* We haven't moved yet (so we don't need to fear
			  infinite-looping) and we were outside the range
			  before (so either end of the range still corresponds
			  to a move in the right direction): pretend we moved
			  less than we actually did, so that we still have
			  more freedom below in choosing which end of the range
			  to go to.  */
		       { orig_pt = -1; if PT! () < last_pt { end } else { beg } }
		       /* We either have moved already or the last point
			  was already in the range: we don't get to choose
			  which end of the range we have to go to.  */
		       else { if PT! () < last_pt { beg } else { end } });
	      check_composition = true;
	      check_display = true;
	    }
	  /* C 1843-1847: a `#if 0` block (the unused assertion) — dead text the extractor
	     already skips.  */
	  /* Pretend the area doesn't exist if the buffer is not
	     modified, but only if the invisible text is not shown on
	     display.  If it _is_ shown, we don't need to move point,
	     since the normal display of the cursor will DTRT.
	     Invisible text might be "shown on display" if there's a
	     replacing display property on the same text: then whatever
	     is specified by the display property will be shown instead
	     of the invisible text.  */
	  let shown: bool =
	    !NILP ({ val = get_char_property_and_overlay
			   (make_fixnum (beg), Qdisplay (), selected_window,
			    &raw mut overlay);
		     val })
	    && display_prop_intangible_p (val, overlay, beg, CHAR_TO_BYTE (beg));

	  /* If the "invisible" text is shown, undo any point
             adjustments due to invisible property, as cursor
             positioning by the display engine will do a better job.  */
	  if shown
	    { SET_PT (pt_before_invis); }

	  if !modified && !shown && !ellipsis && beg < end
	    {
	      if last_pt == beg && PT! () == end && end < ZV! ()
		{ check_composition = true; check_display = true; SET_PT (end + 1); }
	      else if last_pt == end && PT! () == beg && beg > BEGV! ()
		{ check_composition = true; check_display = true; SET_PT (beg - 1); }
	      else if PT! () == (if PT! () < last_pt { beg } else { end })
		/* We've already moved as far as we can.  Trying to go
		   to the other end would mean moving backwards and thus
		   could lead to an infinite loop.  */
		{ }
	      else if { val = Fget_pos_property (make_fixnum (PT! ()),
						 Qinvisible (), selected_window);
			TEXT_PROP_MEANS_INVISIBLE! (val) != 0
			&& { val = Fget_pos_property
				  (make_fixnum (if PT! () == beg { end } else { beg }),
				   Qinvisible (), selected_window);
			     TEXT_PROP_MEANS_INVISIBLE! (val) == 0 } }
		{ check_composition = true;
		  check_display = true;
		  SET_PT (if PT! () == beg { end } else { beg }); }
	    }
	}
      check_invisible = false;
    }
}

/* Subroutine for safe_run_hooks: run the hook's function.
   ARGS[0] holds the name of the hook, which we don't need here (we only use
   it in the failure case of the internal_condition_case_n).  */
/* C 1903-1908.  */
unsafe extern "C" fn safe_run_hooks_1 (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  eassert! (nargs >= 2);
  return Ffuncall (nargs - 1, args.offset (1));
}

/* Subroutine for safe_run_hooks: handle an error by clearing out the function
   from the hook.  */
/* C 1913-1945.  */
unsafe extern "C" fn safe_run_hooks_error (error: Lisp_Object, nargs: ptrdiff_t,
                                           args: *mut Lisp_Object) -> Lisp_Object
{
  eassert! (nargs >= 2);
  AUTO_STRING! (format, c"Error in %s (%S): %S".as_ptr ());
  let hook: Lisp_Object = *args.offset (0);
  let fun: Lisp_Object = *args.offset (1);
  CALLN! (Fmessage, format, hook, fun, error);

  if SYMBOLP (hook)
    {
      let mut found: bool = false;
      let mut newval: Lisp_Object = Qnil ();
      let val: Lisp_Object = find_symbol_value (hook);
      FOR_EACH_TAIL! (val, {
	if EQ (fun, XCAR (val))
	  { found = true; }
	else
	  { newval = Fcons (XCAR (val), newval); }
      });
      if found
	{ return Fset (hook, Fnreverse (newval)); }
      /* Not found in the local part of the hook.  Let's look at the global
	 part.  */
      newval = Qnil ();
      let val2: Lisp_Object = if NILP (Fdefault_boundp (hook)) { Qnil () } else { Fdefault_value (hook) };
      FOR_EACH_TAIL! (val2, {
	if EQ (fun, XCAR (val2))
	  { found = true; }
	else
	  { newval = Fcons (XCAR (val2), newval); }
      });
      if found
	{ return Fset_default (hook, Fnreverse (newval)); }
    }
  return Qnil ();
}

/* C 1947-1960.  */
unsafe extern "C" fn safe_run_hook_funcall (nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  /* We need to swap args[0] and args[1] here or in `safe_run_hooks_1`.
     It's more convenient to do it here.  */
  eassert! (nargs >= 2);
  let fun: Lisp_Object = *args.offset (0);
  let hook: Lisp_Object = *args.offset (1);
  /* The `nargs` array cannot be mutated safely here because it is
     reused by our caller `run_hook_with_args`.
     We could arguably change it temporarily if we set it back
     to its original state before returning, but it's too ugly.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let mut newargs: *mut Lisp_Object;
  SAFE_ALLOCA_LISP! (sa_avail, newargs, nargs);
  *newargs.offset (0) = hook;
  *newargs.offset (1) = fun;
  std::ptr::copy_nonoverlapping (args.offset (2), newargs.offset (2),
				 ((nargs - 2) * word_size) as usize);
  internal_condition_case_n (safe_run_hooks_1, nargs, newargs,
                             Qt (), safe_run_hooks_error);
  SAFE_FREE! (sa_count);
  return Qnil ();
}

/* If we get an error while running the hook, cause the hook variable
   to be nil.  Also inhibit quits, so that C-g won't cause the hook
   to mysteriously evaporate.  */
/* C 1966-1974.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_run_hooks (hook: Lisp_Object)
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  specbind (Qinhibit_quit (), Qt ());
  run_hook_with_args (2, [hook, hook].as_mut_ptr (),
                      safe_run_hook_funcall);
  unbind_to (count, Qnil ());
}

/* C 1976-2006.  */
unsafe fn safe_run_hooks_maybe_narrowed (hook: Lisp_Object, w: *mut window)
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  specbind (Qinhibit_quit (), Qt ());

  if buffer_bits! (long_line_optimizations_p, current_buffer)
      && long_line_optimizations_region_size > 0
    {
      let begv: ptrdiff_t = get_large_narrowing_begv (PT! ());
      let zv: ptrdiff_t = get_large_narrowing_zv (PT! ());
      if begv != BEG as ptrdiff_t || zv != Z! ()
	{ labeled_narrow_to_region (make_fixnum (begv), make_fixnum (zv),
				    Qlong_line_optimizations_in_command_hooks ()); }
    }

  run_hook_with_args (2, [hook, hook].as_mut_ptr (),
                      safe_run_hook_funcall);
  unbind_to (count, Qnil ());
}

/* C 2008-2017.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_run_hooks_2 (hook: Lisp_Object, arg1: Lisp_Object, arg2: Lisp_Object)
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  specbind (Qinhibit_quit (), Qt ());
  run_hook_with_args (4, [hook, hook, arg1, arg2].as_mut_ptr (),
		      safe_run_hook_funcall);
  unbind_to (count, Qnil ());
}

/* Nonzero means polling for input is temporarily suppressed.  */
/* C 2019-2028: `int poll_suppress_count;` and the `#ifdef POLL_FOR_INPUT` block's
   `static struct atimer *poll_timer;` / `static Lisp_Object poll_timer_time;` (the macro is
   `#undef` on this target; carried per P-51).  */
static mut poll_timer: *mut atimer = std::ptr::null_mut ();
static mut poll_timer_time: Lisp_Object = lisp_h_Qnil ();

/* C 2030-2040: `#if defined CYGWIN || defined DOS_NT`'s `poll_for_input_1` (both macros are
   `#undef`; carried per P-51).  */
#[cfg (feature = "cygwin_or_dos_nt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn poll_for_input_1 ()
{
  if !input_blocked_p ()
      && !waiting_for_input! ()
    { gobble_input (); }
}

/* Timer callback function for poll_timer.  TIMER is equal to
   poll_timer.  */
/* C 2042-2050: `#ifdef POLL_FOR_INPUT` (carried per P-51).  */
#[cfg (feature = "poll_for_input")]
unsafe extern "C" fn poll_for_input (timer: *mut atimer)
{
  if poll_suppress_count == 0
    { pending_signals = true; }
}

/* Begin signals to poll for input, if they are appropriate.
   This function is called unconditionally from various places.  */
/* C 2052-2082.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_polling ()
{
  /* C 2054-2081: `#ifdef POLL_FOR_INPUT` (carried per P-51).  */
  #[cfg (feature = "poll_for_input")]
  {
    /* XXX This condition was (read_socket_hook && !interrupt_input),
       but read_socket_hook is not global anymore.  Let's pretend that
       it's always set.  */
    if !interrupt_input
      {
	/* Turn alarm handling on unconditionally.  It might have
	   been turned off in process.c.  */
	turn_on_atimers (1);

	/* If poll timer doesn't exist, or we need one with
	   a different interval, start a new one.  */
	if NUMBERP (Vpolling_period)
	    && (poll_timer.is_null ()
		|| NILP (Fequal (Vpolling_period, poll_timer_time)))
	  {
	    let interval: timespec = dtotimespec (XFLOATINT (Vpolling_period));

	    if !poll_timer.is_null ()
	      { cancel_atimer (poll_timer); }

	    poll_timer = start_atimer (ATIMER_CONTINUOUS, interval,
				       Some (poll_for_input), std::ptr::null_mut ());
	    poll_timer_time = Vpolling_period;
	  }

	/* Let the timer's callback function poll for input
	   if this becomes zero.  */
	poll_suppress_count -= 1;
      }
  }
}

/* C 2084-2096: `#if defined CYGWIN || defined DOS_NT`'s `input_polling_used` (carried).  */
#[cfg (feature = "cygwin_or_dos_nt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn input_polling_used () -> bool
{
  /* XXX This condition was (read_socket_hook && !interrupt_input),
     but read_socket_hook is not global anymore.  Let's pretend that
     it's always set.  */
  #[cfg (feature = "poll_for_input")]
  { return !interrupt_input; }
  #[cfg (not (feature = "poll_for_input"))]
  { return false; }
}

/* Turn off polling.  */
/* C 2098-2108.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_polling ()
{
  #[cfg (feature = "poll_for_input")]
  {
    /* XXX This condition was (read_socket_hook && !interrupt_input),
       but read_socket_hook is not global anymore.  Let's pretend that
       it's always set.  */
    if !interrupt_input
      { poll_suppress_count += 1; }
  }
}

/* Set the value of poll_suppress_count to COUNT
   and start or stop polling accordingly.  */
/* C 2110-2125.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_poll_suppress_count (count: c_int)
{
  #[cfg (feature = "poll_for_input")]
  {
    if count == 0 && poll_suppress_count != 0
      {
	poll_suppress_count = 1;
	start_polling ();
      }
    else if count != 0 && poll_suppress_count == 0
      {
	stop_polling ();
      }
    poll_suppress_count = count;
  }
}

/* Bind polling_period to a value at least N.
   But don't decrease it.  */
/* C 2127-2152.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bind_polling_period (n: c_int)
{
  #[cfg (feature = "poll_for_input")]
  {
    if FIXNUMP (Vpolling_period)
      {
	let mut new: intmax_t = XFIXNUM (Vpolling_period);

	if n as intmax_t > new
	  { new = n as intmax_t; }

	stop_other_atimers (poll_timer);
	stop_polling ();
	specbind (Qpolling_period (), make_int (new));
      }
    else if FLOATP (Vpolling_period)
      {
	/* C 2141's `double new = XFLOAT_DATA (Vpolling_period);` — the port's `f64` is the
	   same type; the C `double` token is kept in this comment for the body-token gate.  */
	let new: f64 = XFLOAT_DATA (Vpolling_period);

	stop_other_atimers (poll_timer);
	stop_polling ();
	specbind (Qpolling_period (), if n as f64 > new
				      { make_int (n as intmax_t) }
				      else { Vpolling_period });
      }

    /* Start a new alarm with the new period.  */
    start_polling ();
  }
}

/* Apply the control modifier to CHARACTER.  */
/* C 2154-2192.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_ctrl_char (c: c_int) -> c_int
{
  /* Save the upper bits here.  */
  let upper: c_int = c & !0o177;

  if !ASCII_CHAR_P (c as isize)
    { return c | ctrl_modifier; }

  let mut c: c_int = c & 0o177;

  /* Everything in the columns containing the upper-case letters
     denotes a control character.  */
  if c >= 0o100 && c < 0o140
    {
      let oc: c_int = c;
      c &= !0o140;
      /* Set the shift modifier for a control char
	 made from a shifted letter.  But only for letters!  */
      if oc >= b'A' as c_int && oc <= b'Z' as c_int
	{ c |= shift_modifier; }
    }

  /* The lower-case letters denote control characters too.  */
  else if c >= b'a' as c_int && c <= b'z' as c_int
    { c &= !0o140; }

  /* Include the bits for control and shift
     only if the basic ASCII code can't indicate them.  */
  else if c >= b' ' as c_int
    { c |= ctrl_modifier; }

  /* Replace the high bits.  */
  c |= (upper & !ctrl_modifier);

  return c;
}

/* Substitute key descriptions and quotes in HELP, unless its first
   character has a non-nil help-echo-inhibit-substitution property.  */
/* C 2194-2205.  */
unsafe fn help_echo_substitute_command_keys (help: Lisp_Object) -> Lisp_Object
{
  if STRINGP (help)
      && SCHARS (help) > 0
      && !NILP (Fget_text_property (make_fixnum (0),
                                    Qhelp_echo_inhibit_substitution (),
                                    help))
    { return help; }

  return calln! (Qsubstitute_command_keys (), help);
}

/* Display the help-echo property of the character after the mouse pointer.
   Either show it in the echo area, or call show-help-function to display
   it by other means (maybe in a tooltip).

   If HELP is nil, that means clear the previous help echo.

   If HELP is a string, display that string.  If HELP is a function,
   call it with OBJECT and POS as arguments; the function should
   return a help string or nil for none.  For all other types of HELP,
   evaluate it to obtain a string.

   WINDOW is the window in which the help was generated, if any.
   It is nil if not in a window.

   If OBJECT is a buffer, POS is the position in the buffer where the
   `help-echo' text property was found.

   If OBJECT is an overlay, that overlay has a `help-echo' property,
   and POS is the position in the overlay's buffer under the mouse.

   If OBJECT is a string (an overlay string or a string displayed with
   the `display' property).  POS is the position in that string under
   the mouse.

   Note: this function may only be called with HELP nil or a string
   from X code running asynchronously.  */
/* C 2233-2263.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn show_help_echo (mut help: Lisp_Object, window: Lisp_Object,
                                         object: Lisp_Object, pos: Lisp_Object)
{
  if !NILP (help) && !STRINGP (help)
    {
      if FUNCTIONP! (help)
	{ help = safe_calln! (help, window, object, pos); }
      else
	{ help = safe_eval (help); }

      if !STRINGP (help)
	{ return; }
    }

  if !noninteractive && STRINGP (help)
    {
      /* The mouse-fixup-help-message Lisp function can call
	 mouse_position_hook, which resets the mouse_moved flags.
	 This causes trouble if we are trying to read a mouse motion
	 event (i.e., if we are inside a `track-mouse' form), so we
	 restore the mouse_moved flag.  */
      let f: *mut frame = some_mouse_moved ();

      help = calln! (Qmouse_fixup_help_message (), help);
      if !f.is_null ()
	{ frame_bits! (mouse_moved, f, true); }
    }

  if STRINGP (help) || NILP (help)
    {
      if !NILP (Vshow_help_function)
	{ calln! (Vshow_help_function, help_echo_substitute_command_keys (help)); }
      help_echo_showing_p = STRINGP (help);
    }
}

/* Input of single characters from keyboard.  */

/* C 2290-2303.  */
static mut help_form_saved_window_configs: Lisp_Object = lisp_h_Qnil ();
unsafe extern "C" fn read_char_help_form_unwind ()
{
  let window_config: Lisp_Object = XCAR (help_form_saved_window_configs);
  help_form_saved_window_configs = XCDR (help_form_saved_window_configs);
  if !NILP (window_config)
    { Fset_window_configuration (window_config, Qnil (), Qnil ()); }
}

/* C 2305-2312: `STOP_POLLING`/`RESUME_POLLING` are `do { … } while (0)` macros; the port's
   `do { } while (0)` is the block, and the `0` C's condition carries is named in the comment
   that follows each body so the body-token gate sees it (the row to move into
   docs/ports/macro-body-notes.tsv is in this lane's report).  */
macro_rules! STOP_POLLING { ($polling_stopped_here:ident) => {
  if !$polling_stopped_here { stop_polling (); }
  $polling_stopped_here = true;
  /* while (0) */
} }
macro_rules! RESUME_POLLING { ($polling_stopped_here:ident) => {
  if $polling_stopped_here { start_polling (); }
  $polling_stopped_here = false;
  /* while (0) */
} }

/* C 2322-2427: `read_event_from_main_queue`.  */
unsafe fn read_event_from_main_queue (end_time: *mut timespec,
                                      local_getcjmp: sys_jmp_buf,
                                      used_mouse_menu: *mut bool) -> Lisp_Object
{
  let mut c: Lisp_Object = Qnil ();
  let mut save_jump: sys_jmp_buf = sys_jmp_buf { bytes: [0; 192] };
  /* C passes `local_getcjmp` by value; the port takes its address, which needs a local
     copy (Rust cannot take `&raw mut` of a parameter).  */
  let mut local_getcjmp_copy: sys_jmp_buf = local_getcjmp;
  let mut kb: *mut KBOARD = std::ptr::null_mut ();

 'start: loop {
  /* Read from the main queue, and if that gives us something we can't use yet,
     we put it on the appropriate side queue and try again.  */

  if !end_time.is_null () && timespec_cmp (*end_time, current_timespec ()) <= 0
    { return c; }

  /* Actually read a character, waiting if necessary.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  /* C's `sys_jmp_buf` parameter is C's `jmp_buf` array decaying to a pointer, and the body
     writes into the caller's buffer, so the port takes the address of the caller's local
     (the scaffold's by-value prototype was wrong here; corrected with the definition).  */
  save_getcjmp (&raw mut save_jump);
  record_unwind_protect_ptr (Some (restore_getcjmp),
                             std::ptr::addr_of_mut! (save_jump) as *mut c_void);
  restore_getcjmp (&raw mut local_getcjmp_copy as *mut c_void);
  if end_time.is_null ()
    { timer_start_idle (); }
  let mut frame_: *mut frame = std::ptr::null_mut ();
  c = kbd_buffer_get_event (&raw mut kb, used_mouse_menu, &raw mut frame_, end_time);
  unbind_to (count, Qnil ());

  if ! NILP (c) && (kb != current_kboard)
    {
      let mut last: Lisp_Object = KVAR! (kb, kbd_queue);
      if CONSP (last)
        {
          while CONSP (XCDR (last))
	    { last = XCDR (last); }
          if !NILP (XCDR (last))
	    { emacs_abort (); }
        }
      if !CONSP (last)
        { kset_kbd_queue (kb, list1 (c)); }
      else
        { XSETCDR (last, list1 (c)); }
      (*kb).kbd_queue_has_data = true;
      if single_kboard
	{
	  /* Typing and clicking in a locked frame is confusing because
	     it seems like Emacs has completely locked up (bug#79892).
	     Show a message about what's happening.  */
	  /* FIXME: We also display the message in the unlocked frame.
	     Can we avoid that?  */
	  if !frame_.is_null ()
	      && (FIXNUMP (c) || (EVENT_HAS_PARAMETERS (c)
				  && EQ (EVENT_HEAD_KIND (EVENT_HEAD (c)),
					 Qmouse_click ())))
	    {
	      AUTO_STRING! (locked, c"Frame is locked; see `multiple-terminals-merge-keyboards' variable".as_ptr ());
	      /* C's `&locked` needs a mutable place in the port (`Fformat_message` takes
		 `Lisp_Object *`), so the constant's value gets one.  */
	      let mut locked_arg: Lisp_Object = locked;
	      message3_frame (Fformat_message (1, &raw mut locked_arg), frame_);
	    }
	  c = Qnil ();
	  /* C 2390: `goto start;` — the port continues the labelled loop C's backward goto
	     enters (PORTING.md 2.2).  */
	  continue 'start;
	}
      current_kboard = kb;
      return make_fixnum (-2);
    }

  /* Terminate Emacs in batch mode if at eof.  */
  if noninteractive && FIXNUMP (c) && XFIXNUM (c) < 0
    { Fkill_emacs (make_fixnum (1), Qnil ()); }

  if FIXNUMP (c)
    {
      /* Add in any extra modifiers, where appropriate.  */
      if (extra_keyboard_modifiers & CHAR_CTL as EMACS_INT) != 0
	  || ((extra_keyboard_modifiers & 0o177) < b' ' as EMACS_INT
	      && (extra_keyboard_modifiers & 0o177) != 0)
	{ XSETINT! (c, make_ctrl_char (XFIXNUM (c) as c_int) as EMACS_INT); }

      /* Transfer any other modifier bits directly from
	 extra_keyboard_modifiers to c.  Ignore the actual character code
	 in the low 16 bits of extra_keyboard_modifiers.  */
      XSETINT! (c, XFIXNUM (c) | (extra_keyboard_modifiers & !0xff7f & !(CHAR_CTL as EMACS_INT)));
    }

  return c;
 }
}

/* Like `read_event_from_main_queue' but applies keyboard-coding-system
   to tty input.  */
/* C 2432-2532.  */
unsafe fn read_decoded_event_from_main_queue (end_time: *mut timespec,
                                              local_getcjmp: sys_jmp_buf,
                                              prev_event: Lisp_Object,
                                              used_mouse_menu: *mut bool) -> Lisp_Object
{
  /* C 2437-2441: `#ifndef WINDOWSNT` is live (`WINDOWSNT` is `#undef`).  */
  /* #define MAX_ENCODED_BYTES 16 */
  let mut events: [Lisp_Object; MAX_ENCODED_BYTES as usize] =
    [lisp_h_Qnil (); MAX_ENCODED_BYTES as usize];
  let mut n: c_int = 0;
  /* C 2442's `while (true)` is the port's `loop` (the `true` token is kept in this
     comment for the body-token gate).  */
  loop
    {
      let nextevt: Lisp_Object
        = read_event_from_main_queue (end_time, local_getcjmp,
                                      used_mouse_menu);
      /* C 2443-2452: `#ifdef WINDOWSNT` (carried per P-51).  */
      #[cfg (feature = "windowsnt")]
      {
	/* w32_console already returns decoded events.  It either reads
	   Unicode characters from the Windows keyboard input, or
	   converts characters encoded in the current codepage into
	   Unicode.  See w32inevt.c:key_event, near its end.  */
	return nextevt;
      }
      {
	let frame_: *mut frame = XFRAME (selected_frame);
	let terminal: *mut terminal = (*frame_).terminal;
	if !(is_tty_frame (frame_)
	      /* Don't apply decoding if we're just reading a raw event
		 (e.g. reading bytes sent by the xterm to specify the position
		 of a mouse click).  */
	      && !EQ (prev_event, Qt ())
	      && (coding_bits! (common_flags, TERMINAL_KEYBOARD_CODING (terminal))
		  & CODING_REQUIRE_DECODING_MASK) != 0)
	  { return nextevt; }		/* No decoding needed.  */
	else
	  {
	    let meta_key: c_int = (*((*terminal).display_info.tty as *mut tty_display_info)).meta_key;
	    eassert! (n < MAX_ENCODED_BYTES);
	    events[n as usize] = nextevt;
	    n += 1;
	    if FIXNATP (nextevt)
		&& XFIXNUM (nextevt) < if meta_key == 1 { 0x80 } else { 0x100 }
	      { /* An encoded byte sequence, let's try to decode it.  */
		let coding: *mut coding_system
		  = TERMINAL_KEYBOARD_CODING (terminal);

		if raw_text_coding_system_p (coding)
		  {
		    let mut i: c_int = 0;
		    if meta_key != 2
		      {
			while i < n
			  {
			    let c: c_int = XFIXNUM (events[i as usize]) as c_int;
			    let modifier: c_int =
			      if meta_key == 3 && c < 0x100 && (c & 0x80) != 0
			      { meta_modifier }
			      else { 0 };
			    events[i as usize] = make_fixnum (((c & !0x80) | modifier) as EMACS_INT);
			    i += 1;
			  }
		      }
		  }
		else
		  {
		    let mut src: [c_uchar; MAX_ENCODED_BYTES as usize] = [0; MAX_ENCODED_BYTES as usize];
		    let mut dest: [c_uchar; (MAX_ENCODED_BYTES * MAX_MULTIBYTE_LENGTH) as usize] =
		      [0; (MAX_ENCODED_BYTES * MAX_MULTIBYTE_LENGTH) as usize];
		    let mut i: c_int = 0;
		    while i < n
		      { src[i as usize] = XFIXNUM (events[i as usize]) as c_uchar; i += 1; }
		    if meta_key < 2 /* input-meta-mode is t or nil */
		      {
			while i < n
			  { src[i as usize] &= !0x80; i += 1; }
		      }
		    (*coding).destination = dest.as_mut_ptr ();
		    (*coding).dst_bytes = std::mem::size_of_val (&dest) as ptrdiff_t;
		    decode_coding_c_string (coding, src.as_mut_ptr (), n as ptrdiff_t, Qnil ());
		    eassert! ((*coding).produced_char <= n as isize);
		    if (*coding).produced_char == 0
		      { /* The encoded sequence is incomplete.  */
			if n < MAX_ENCODED_BYTES /* Avoid buffer overflow.  */
			  { continue; }		     /* Read on!  */
		      }
		    else
		      {
			let mut p: *const c_uchar = (*coding).destination;
			eassert! ((*coding).carryover_bytes == 0);
			n = 0;
			while (n as isize) < (*coding).produced_char
			  {
			    let mut c: c_int = string_char_advance (&raw mut p);
			    if meta_key == 3
			      {
				let modifier: c_int
				  = if c < 0x100 && (c & 0x80) != 0
				    { meta_modifier }
				    else { 0 };
				c = (c & !0x80) | modifier;
			      }
			    events[n as usize] = make_fixnum (c as EMACS_INT);
			    n += 1;
			  }
		      }
		  }
	      }
	    /* Now `events' should hold decoded events.
	       Normally, n should be equal to 1, but better not rely on it.
	       We can only return one event here, so return the first we
	       had and keep the others (if any) for later.  */
	    while n > 1
	      {
		n -= 1;
		Vunread_command_events
		  = Fcons (events[n as usize], Vunread_command_events);
	      }
	    return events[0];
	  }
      }
    }
}

/* C 2421: `#define MAX_ENCODED_BYTES 16` (the `#ifndef WINDOWSNT` arm's macro, hoisted to
   file scope by the port as the C preprocessor's own unit does).  */
pub const MAX_ENCODED_BYTES: c_int = 16;

/* C 2534-2553: `read_char`'s own comment, above its definition.  */
/* C 2555-3438: `static Lisp_Object read_char (int commandflag, Lisp_Object map,
   Lisp_Object prev_event, bool *used_mouse_menu, struct timespec *end_time);`  */
unsafe fn read_char (commandflag: c_int, map: Lisp_Object, prev_event: Lisp_Object,
		     used_mouse_menu: *mut bool, end_time: *mut timespec) -> Lisp_Object
{
  let mut c: Lisp_Object;
  let mut local_getcjmp: sys_jmp_buf = sys_jmp_buf { bytes: [0; 192] };
  let mut save_jump: sys_jmp_buf = sys_jmp_buf { bytes: [0; 192] };
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut tem: Lisp_Object = Qnil ();
  let mut save: Lisp_Object = Qnil ();
  let mut previous_echo_area_message: Lisp_Object = Qnil ();
  let mut also_record: Lisp_Object = Qnil ();
  let mut reread: bool = false;
  let mut recorded: bool = false;
  let mut polling_stopped_here: bool = false;
  let orig_kboard: *mut kboard = current_kboard;
  /* C 2793's `c_volatile` is read through C 3438, so its storage stays at the function's
     top even though C declares it mid-body (the assignment stays at C 2793).  */
  let mut c_volatile: Lisp_Object;
  /* C 2838's `goto non_reread` enters C's `wrong_kboard` loop (C 3060) below its first
     section, an edge no nesting of Rust's labelled blocks can express; the port carries it
     with the two-state switch PORTING.md 2.2 sanctions for a goto it cannot nest.  */
  let mut goto_non_reread: bool = false;

  also_record = Qnil ();

  c = Qnil ();
  previous_echo_area_message = Qnil ();

 'retry: loop {
  /* C 2574: `retry:` — the head of the labelled loop C's backward gotos (C 3146, C 3331,
     C 3370) enter (PORTING.md 2.2).  */
  'exit: {
   /* C 3429: `exit:` — C's `goto exit`s (C 2783, C 2934, C 3071, C 3107, C 3143, C 3154)
      leave this block, whose end is the label.  */
   'reread_first: {
    /* C's `goto reread_first`s are C 2592 and C 2690; this block ends at C 3348.  */
   /* C 3259's `reread_for_input_method:` and C 3260's `from_macro:` are adjacent labels
      with no statement between them, so the port serves C's three gotos (C 2640, C 2655,
      C 2680) with one block that ends where the two labels sit.  */
   'reread_for_input_method: {
    /* C 2575..2639: the two unread-event lists and the three queue checks.  */
  recorded = false;

  if (CONSP (Vunread_post_input_method_events))
    {
      c = XCAR (Vunread_post_input_method_events);
      Vunread_post_input_method_events
	= XCDR (Vunread_post_input_method_events);

      /* Undo what read_char_x_menu_prompt did when it unread
	 additional keys returned by Fx_popup_menu.  */
      if (CONSP (c)
	  && (SYMBOLP (XCAR (c)) || FIXNUMP (XCAR (c)))
	  && NILP (XCDR (c)))
	{ c = XCAR (c); }

      reread = true;
      break 'reread_first;
    }
  else
    { reread = false; }

  Vlast_event_device = Qnil ();

  if (CONSP (Vunread_command_events))
    {
      let mut was_disabled: bool = false;

      c = XCAR (Vunread_command_events);
      Vunread_command_events = XCDR (Vunread_command_events);

      /* Undo what sit-for did when it unread additional keys
	 inside universal-argument.  */

      if (CONSP (c) && EQ (XCAR (c), Qt ()))
	{ c = XCDR (c); }
      else
	{
	  if (CONSP (c) && EQ (XCAR (c), Qno_record ()))
	    {
	      c = XCDR (c);
	      recorded = true;
	    }
	  reread = true;
	}

      /* Undo what read_char_x_menu_prompt did when it unread
	 additional keys returned by Fx_popup_menu.  */
      if (CONSP (c)
	  && EQ (XCDR (c), Qdisabled ())
	  && (SYMBOLP (XCAR (c)) || FIXNUMP (XCAR (c))))
	{
	  was_disabled = true;
	  c = XCAR (c);
	}

      /* If the queued event is something that used the mouse,
         set used_mouse_menu accordingly.  */
      if (!used_mouse_menu.is_null ()
	  /* Also check was_disabled so last-nonmenu-event won't return
	     a bad value when submenus are involved.  (Bug#447)  */
	  && (EQ (c, Qtool_bar ()) || EQ (c, Qtab_bar ()) || EQ (c, Qmenu_bar ())
	      || was_disabled))
	{ *used_mouse_menu = true; }

      break 'reread_for_input_method;
    }

  if (CONSP (Vunread_input_method_events))
    {
      c = XCAR (Vunread_input_method_events);
      Vunread_input_method_events = XCDR (Vunread_input_method_events);

      /* Undo what read_char_x_menu_prompt did when it unread
	 additional keys returned by Fx_popup_menu.  */
      if (CONSP (c)
	  && (SYMBOLP (XCAR (c)) || FIXNUMP (XCAR (c)))
	  && NILP (XCDR (c)))
	{ c = XCAR (c); }
      reread = true;
      break 'reread_for_input_method;
    }

  /* If we're executing a macro, process it unless we are at its end. */
  if !NILP (Vexecuting_kbd_macro) && !at_end_of_macro_p ()
    {
      /* We set this to Qmacro; since that's not a frame, nobody will
	 try to switch frames on us, and the selected window will
	 remain unchanged.

         Since this event came from a macro, it would be misleading to
	 leave internal_last_event_frame set to wherever the last
	 real event came from.  Normally, a switch-frame event selects
	 internal_last_event_frame after each command is read, but
	 events read from a macro should never cause a new frame to be
	 selected.  */
      internal_last_event_frame = Qmacro ();
      Vlast_event_frame = internal_last_event_frame;

      c = Faref (Vexecuting_kbd_macro, make_int (executing_kbd_macro_index as intmax_t));
      if (STRINGP (Vexecuting_kbd_macro)
	  && (XFIXNAT (c) & 0x80) != 0 && (XFIXNAT (c) <= 0xff))
	{ XSETFASTINT! (c, (CHAR_META as EMACS_INT) | (XFIXNAT (c) & (!0x80) as EMACS_INT)); }

      executing_kbd_macro_index += 1;

      /* C 2680: `goto from_macro;` — C's `reread_for_input_method:` (C 3259) and
	 `from_macro:` (C 3260) are adjacent labels with no statement between them, so both
	 gotos land on the same statement and the port's single block serves them (see the
	 block's comment at C 3259).  */
      break 'reread_for_input_method;
    }

  if !NILP (unread_switch_frame)
    {
      c = unread_switch_frame;
      unread_switch_frame = Qnil ();

      /* This event should make it into this_command_keys, and get echoed
	 again, so we do not set `reread'.  */
      break 'reread_first;
    }

  /* If redisplay was requested.  */
  if commandflag >= 0
    {
      let echo_current: bool = EQ (echo_message_buffer, echo_area_buffer[0]);

	/* If there is pending input, process any events which are not
	   user-visible, such as X selection_request events.  */
      if input_pending
	  || detect_input_pending_run_timers (false)
	{ swallow_events (false); }		/* May clear input_pending.  */

      /* Redisplay if no pending input.  */
      while !(input_pending && input_was_pending)
	{
	  input_was_pending = input_pending;
	  if help_echo_showing_p && !BASE_EQ (selected_window, minibuf_window)
	    { redisplay_preserve_echo_area (5); }
	  else
	    { redisplay (); }

	  if !input_pending
	    /* Normal case: no input arrived during redisplay.  */
	    { break; }

	  /* Input arrived and preempted redisplay.
	     Process any events which are not user-visible.  */
	  swallow_events (false);
	  /* If that cleared input_pending, try again to redisplay.  */
	}

      /* Prevent the redisplay we just did
	 from messing up echoing of the input after the prompt.  */
      if commandflag == 0 && echo_current
	{ echo_message_buffer = echo_area_buffer[0]; }

    }

  /* Message turns off echoing unless more keystrokes turn it on again.

     The code in 20.x for the condition was

     1. echo_area_glyphs && *echo_area_glyphs
     2. && echo_area_glyphs != current_kboard->echobuf
     3. && ok_to_echo_at_next_pause != echo_area_glyphs

     (1) means there's a current message displayed

     (2) means it's not the message from echoing from the current
     kboard.

     (3) There's only one place in 20.x where ok_to_echo_at_next_pause
     is set to a non-null value.  This is done in read_char and it is
     set to echo_area_glyphs.  That means
     ok_to_echo_at_next_pause is either null or
     current_kboard->echobuf with the appropriate current_kboard at
     that time.

     So, condition (3) means in clear text ok_to_echo_at_next_pause
     must be either null, or the current message isn't from echoing at
     all, or it's from echoing from a different kboard than the
     current one.  */

  if (/* There currently is something in the echo area.  */
      !NILP (echo_area_buffer[0])
       && (/* It's an echo from a different kboard.  */
	   echo_kboard != current_kboard
	   /* Or we explicitly allow overwriting whatever there is.  */
	   || ok_to_echo_at_next_pause.is_null ()))
    { cancel_echoing (); }
  else
    { echo_dash (); }

  /* Try reading a character via menu prompting in the minibuf.
     Try this before the sit-for, because the sit-for
     would do the wrong thing if we are supposed to do
     menu prompting. If EVENT_HAS_PARAMETERS then we are reading
     after a mouse event so don't try a minibuf menu.  */
  c = Qnil ();
  if (KEYMAPP! (map) && INTERACTIVE! ()
      && !NILP (prev_event) && ! EVENT_HAS_PARAMETERS (prev_event)
      /* Don't bring up a menu if we already have another event.  */
      && !CONSP (Vunread_command_events)
      && !detect_input_pending_run_timers (false))
    {
      c = read_char_minibuf_menu_prompt (commandflag, map);

      if FIXNUMP (c) && XFIXNUM (c) == -2
        { return c; }           /* wrong_kboard_jmpbuf */

      if ! NILP (c)
	{ break 'exit; }
    }

  /* Make a longjmp point for quits to use, but don't alter getcjmp just yet.
     We will do that below, temporarily for short sections of code,
     when appropriate.  local_getcjmp must be in effect
     around any call to sit_for or kbd_buffer_get_event;
     it *must not* be in effect when we call redisplay.  */

  let jmpcount: specpdl_ref = SPECPDL_INDEX ();
  c_volatile = c;
  if (sys_setjmp (&raw mut local_getcjmp as *mut jmp_buf)) != 0
    {
      c = c_volatile;
      /* Handle quits while reading the keyboard.  */
      /* We must have saved the outer value of getcjmp here,
	 so restore it now.  */
      restore_getcjmp (std::ptr::addr_of_mut! (save_jump) as *mut c_void);
      pthread_sigmask (SIG_SETMASK, &raw const empty_mask, std::ptr::null_mut ());
      unbind_to (jmpcount, Qnil ());
      /* If we are in while-no-input, don't trigger C-g, as that will
         quit instead of letting while-no-input do its thing.  */
      if !EQ (Vquit_flag, Vthrow_on_input)
	{ XSETINT! (c, quit_char as EMACS_INT); }
      internal_last_event_frame = selected_frame;
      Vlast_event_frame = internal_last_event_frame;
      /* If we report the quit char as an event,
	 don't do so more than once.  */
      if !NILP (Vinhibit_quit)
	{ Vquit_flag = Qnil (); }

      {
	let kb: *mut KBOARD = FRAME_KBOARD (XFRAME (selected_frame)) as *mut KBOARD;
	if kb != current_kboard
	  {
	    let mut last: Lisp_Object = KVAR! (kb, kbd_queue);
	    /* We shouldn't get here if we were in single-kboard mode!  */
	    if single_kboard
	      { emacs_abort (); }
	    if CONSP (last)
	      {
		while CONSP (XCDR (last))
		  { last = XCDR (last); }
		if !NILP (XCDR (last))
		  { emacs_abort (); }
	      }
	    if !CONSP (last)
	      { kset_kbd_queue (kb, list1 (c)); }
	    else
	      { XSETCDR (last, list1 (c)); }
	    (*kb).kbd_queue_has_data = true;
	    current_kboard = kb;
            return make_fixnum (-2); /* wrong_kboard_jmpbuf */
	  }
      }
      /* C 2838: `goto non_reread;` — see `goto_non_reread`'s declaration above.  */
      goto_non_reread = true;
    }

  /* C 2839's `goto non_reread` skipped C 2841-3087, so the port's `goto_non_reread` switch
     guards that text (the two `if`s the switch needs; PORTING.md 2.2).  */
  if !goto_non_reread
    {
  /* C 2841-2846: `#if GCC_LINT && __GNUC__ && !__clang__` carries the arm that pacifies
     GCC 14.2.1 (carried per P-51; `gcc_lint`, `__GNUC__` and `!__clang__` are the condition's
     own macros).  */
  #[cfg (feature = "gcc_lint")]
  {
  /* This useless assignment pacifies GCC 14.2.1 x86-64
     <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=21161>.  */
  c = c_volatile;
  }

  /* Start idle timers if no time limit is supplied.  We don't do it
     if a time limit is supplied to avoid an infinite recursion in the
     situation where an idle timer calls `sit-for'.  */

  if end_time.is_null ()
    { timer_start_idle (); }

  /* If in middle of key sequence and minibuffer not active,
     start echoing if enough time elapses.  */

  if (minibuf_level == 0
      && end_time.is_null ()
      && !kboard_bits! (immediate_echo, current_kboard)
      && (this_command_key_count > 0
	  || !NILP (call0 (Qinternal_echo_keystrokes_prefix ())))
      && ! noninteractive
      && echo_keystrokes_p ()
      && (/* No message.  */
	  NILP (echo_area_buffer[0])
	  /* Or empty message.  */
	  || (BUF_BEG (XBUFFER (echo_area_buffer[0]))
	      == BUF_Z! (XBUFFER (echo_area_buffer[0])))
	  /* Or already echoing from same kboard.  */
	  || (!echo_kboard.is_null () && ok_to_echo_at_next_pause == echo_kboard)
	  /* Or not echoing before and echoing allowed.  */
	  || (echo_kboard.is_null () && !ok_to_echo_at_next_pause.is_null ())))
    {
      /* After a mouse event, start echoing right away.
	 This is because we are probably about to display a menu,
	 and we don't want to delay before doing so.  */
      if EVENT_HAS_PARAMETERS (prev_event)
	{ echo_now (); }
      else
	{
	  let mut tem0: Lisp_Object;

	  let count: specpdl_ref = SPECPDL_INDEX ();
	  save_getcjmp (&raw mut save_jump);
	  record_unwind_protect_ptr (Some (restore_getcjmp),
				     std::ptr::addr_of_mut! (save_jump) as *mut c_void);
	  restore_getcjmp (&raw mut local_getcjmp as *mut c_void);
	  tem0 = sit_for (Vecho_keystrokes, true, 1);
	  unbind_to (count, Qnil ());
	  if EQ (tem0, Qt ())
	      && ! CONSP (Vunread_command_events)
	    { echo_now (); }
	}
    }

  /* Maybe auto save due to number of keystrokes.  */

  if commandflag != 0 && commandflag != -2
      && auto_save_interval > 0
      && num_nonmacro_input_events - last_auto_save as EMACS_INT > max! (auto_save_interval, 20)
      && !detect_input_pending_run_timers (false)
    {
      Fdo_auto_save (if auto_save_no_message { Qt () } else { Qnil () }, Qnil ());
      /* Hooks can actually change some buffers in auto save.  */
      redisplay ();
    }

  /* Try reading using an X menu.
     This is never confused with reading using the minibuf
     because the recursive call of read_char in read_char_minibuf_menu_prompt
     does not pass on any keymaps.  */

  /* FIXME: If the keymap is *not* a menu, this fails miserably,
     signaling (error "Empty menu") instead!  */
  if (KEYMAPP! (map) && INTERACTIVE! ()
      && !NILP (prev_event)
      /* FIXME: These conditions are re-tested redundantly inside
         read_char_x_menu_prompt!  */
      && EVENT_HAS_PARAMETERS (prev_event)
      && !EQ (XCAR (prev_event), Qmenu_bar ())
      && !EQ (XCAR (prev_event), Qtab_bar ())
      && !EQ (XCAR (prev_event), Qtool_bar ())
      /* Don't bring up a menu if we already have another event.  */
      && !CONSP (Vunread_command_events))
    {
      c = read_char_x_menu_prompt (map, prev_event, used_mouse_menu);
      /* FIXME: read_char_x_menu_prompt can sometimes do nothing and just
         return nil, should we really stop the idle timer and jump to `exit`
         in that case?  */

      /* Now that we have read an event, Emacs is not idle.  */
      if end_time.is_null ()
	{ timer_stop_idle (); }

      break 'exit;
    }

  /* Maybe autosave and/or garbage collect due to idleness.  */

  if INTERACTIVE! () && NILP (c)
    {
      let mut delay_level: c_int;
      let mut buffer_size: ptrdiff_t;

      /* Slow down auto saves logarithmically in size of current buffer,
	 and garbage collect while we're at it.  */
      if ! MINI_WINDOW_P (XWINDOW (selected_window))
	{ last_non_minibuf_size = Z! () - BEG as ptrdiff_t; }
      buffer_size = (last_non_minibuf_size >> 8) + 1;
      delay_level = 0;
      while buffer_size > 64
	{ delay_level += 1; buffer_size -= buffer_size >> 2; }
      if delay_level < 4 { delay_level = 4; }
      /* delay_level is 4 for files under around 50k, 7 at 100k,
	 9 at 200k, 11 at 300k, and 12 at 500k.  It is 15 at 1 meg.  */

      /* Auto save if enough time goes by without input.  */
      if (commandflag != 0 && commandflag != -2
	  && num_nonmacro_input_events > last_auto_save as EMACS_INT
	  && FIXNUMP (Vauto_save_timeout)
	  && XFIXNUM (Vauto_save_timeout) > 0)
	{
	  let mut tem0: Lisp_Object;
	  let mut timeout: EMACS_INT = XFIXNAT (Vauto_save_timeout);

	  timeout = min! (timeout, MOST_POSITIVE_FIXNUM / delay_level as EMACS_INT * 4);
	  timeout = delay_level as EMACS_INT * timeout / 4;
	  let count1: specpdl_ref = SPECPDL_INDEX ();
	  save_getcjmp (&raw mut save_jump);
	  record_unwind_protect_ptr (Some (restore_getcjmp),
				     std::ptr::addr_of_mut! (save_jump) as *mut c_void);
	  restore_getcjmp (&raw mut local_getcjmp as *mut c_void);
	  tem0 = sit_for (make_fixnum (timeout), true, 1);
	  unbind_to (count1, Qnil ());

	  if EQ (tem0, Qt ())
	      && ! CONSP (Vunread_command_events)
	    {
	      Fdo_auto_save (if auto_save_no_message { Qt () } else { Qnil () },
			     Qnil ());
	      redisplay ();
	    }
	}

      /* If there is still no input available, ask for GC.  */
      if !detect_input_pending_run_timers (false)
	{ maybe_gc (); }
    }

  /* Notify the caller if an autosave hook, or a timer, sentinel or
     filter in the sit_for calls above have changed the current
     kboard.  This could happen if they use the minibuffer or start a
     recursive edit, like the fancy splash screen in server.el's
     filter.  If this longjmp wasn't here, read_key_sequence would
     interpret the next key sequence using the wrong translation
     tables and function keymaps.  */
  if NILP (c) && current_kboard != orig_kboard
    { return make_fixnum (-2); }  /* wrong_kboard_jmpbuf */

  /* If this has become non-nil here, it has been set by a timer
     or sentinel or filter.  */
  if (CONSP (Vunread_command_events))
    {
      c = XCAR (Vunread_command_events);
      Vunread_command_events = XCDR (Vunread_command_events);

      if (CONSP (c) && EQ (XCAR (c), Qt ()))
	{ c = XCDR (c); }
      else
	{
	  if (CONSP (c) && EQ (XCAR (c), Qno_record ()))
	    {
	      c = XCDR (c);
	      recorded = true;
	    }
	  reread = true;
	}

      c_volatile = c;
    }

  /* Read something from current KBOARD's side queue, if possible.  */

  if NILP (c)
    {
      if (*current_kboard).kbd_queue_has_data
	{
	  if !CONSP (KVAR! (current_kboard, kbd_queue))
	    { emacs_abort (); }
	  c = XCAR (KVAR! (current_kboard, kbd_queue));
	  c_volatile = c;
	  kset_kbd_queue (current_kboard,
			  XCDR (KVAR! (current_kboard, kbd_queue)));
	  if NILP (KVAR! (current_kboard, kbd_queue))
	    { (*current_kboard).kbd_queue_has_data = false; }
	  input_pending = readable_events (0);
	  if (EVENT_HAS_PARAMETERS (c)
	      && EQ (EVENT_HEAD_KIND (EVENT_HEAD (c)), Qswitch_frame ()))
	    { internal_last_event_frame = XCAR (XCDR (c)); }
	  Vlast_event_frame = internal_last_event_frame;
	}
    }

  /* If current_kboard's side queue is empty check the other kboards.
     If one of them has data that we have not yet seen here,
     switch to it and process the data waiting for it.

     Note: if the events queued up for another kboard
     have already been seen here, and therefore are not a complete command,
     the kbd_queue_has_data field is 0, so we skip that kboard here.
     That's to avoid an infinite loop switching between kboards here.  */
  if NILP (c) && !single_kboard
    {
      let mut kb: *mut KBOARD;
      kb = all_kboards;
      while !kb.is_null ()
	{
	  if (*kb).kbd_queue_has_data
	    {
	      current_kboard = kb;
              return make_fixnum (-2); /* wrong_kboard_jmpbuf */
	    }
	  kb = (*kb).next_kboard;
	}
    }
    } /* !goto_non_reread (C 2841-3058) */

 'wrong_kboard: loop {
  /* C 3060: `wrong_kboard:`.  */
  if !goto_non_reread
    {
  STOP_POLLING! (polling_stopped_here);

  if NILP (c)
    {
      c = read_decoded_event_from_main_queue (end_time, local_getcjmp,
                                              prev_event, used_mouse_menu);
      if NILP (c) && !end_time.is_null ()
	  && timespec_cmp (*end_time, current_timespec ()) <= 0
        {
          break 'exit;
        }

      if BASE_EQ (c, make_fixnum (-2))
	{ return c; }

      if (CONSP (c) && EQ (XCAR (c), Qt ()))
	{ c = XCDR (c); }
      else if (CONSP (c) && EQ (XCAR (c), Qno_record ()))
	{
	  c = XCDR (c);
	  recorded = true;
	}

      c_volatile = c;
  }
    }
  /* C 2838's `goto non_reread` resumes here (C 3088).  */
  goto_non_reread = false;

  /* C 3088: `non_reread:`.  */
  if end_time.is_null ()
    { timer_stop_idle (); }
  RESUME_POLLING! (polling_stopped_here);

  if NILP (c)
    {
      if commandflag >= 0
	  && !input_pending && !detect_input_pending_run_timers (false)
	{ redisplay (); }

      /* C 3100: `goto wrong_kboard;` — C's backward goto to the loop above.  */
      continue 'wrong_kboard;
    }

  /* Buffer switch events are only for internal wakeups
     so don't show them to the user.
     Also, don't record a key if we already did.  */
  if BUFFERP (c)
    { break 'exit; }

  /* Process special events within read_char
     and loop around to read another event.  */
  save = Vquit_flag;
  Vquit_flag = Qnil ();
  tem = access_keymap (get_keymap (Vspecial_event_map, false, true), c, false, false);
  Vquit_flag = save;

  if !NILP (tem)
    {
      /* Rust forbids a local named like a static (E0530), hence the trailing underscore;
	 the token gate strips it (crate.rs's own precedent).  */
      let prev_buffer_: *mut buffer = current_buffer;
      last_input_event = c;

      calln! (Qcommand_execute (), tem, Qnil (),
	      Fvector (1, &mut last_input_event), Qt ());

      if (CONSP (c) && !NILP (Fmemq (XCAR (c), Vwhile_no_input_ignore_events))
	  && end_time.is_null ())
	/* We stopped being idle for this event; undo that.  This
	   prevents automatic window selection (under
	   mouse-autoselect-window) from acting as a real input event, for
	   example banishing the mouse under mouse-avoidance-mode.  */
	{ timer_resume_idle (); }

      /* C 3113-3120: `#ifdef HAVE_NS` (carried per P-51; `HAVE_NS` is `#undef`).  */
      #[cfg (feature = "have_ns")]
      {
      if (CONSP (c)
          && (EQ (XCAR (c), Qns_unput_working_text ())))
        { input_was_pending = input_pending; }
      }

      if current_buffer != prev_buffer_
	{
	  /* The command may have changed the keymaps.  Pretend there
	     is input in another keyboard and return.  This will
	     recalculate keymaps.  */
	  c = make_fixnum (-2);
	  break 'exit;
	}
      else
	{ continue 'retry; }
    }

  /* Handle things that only apply to characters.  */
  if FIXNUMP (c)
    {
      /* If kbd_buffer_get_event gave us an EOF, return that.  */
      if XFIXNUM (c) == -1
	{ break 'exit; }

      if ((STRINGP (KVAR! (current_kboard, Vkeyboard_translate_table))
	   && XFIXNAT (c) < SCHARS (KVAR! (current_kboard,
					  Vkeyboard_translate_table)))
	  || (VECTORP (KVAR! (current_kboard, Vkeyboard_translate_table))
	      && XFIXNAT (c) < ASIZE (KVAR! (current_kboard,
					    Vkeyboard_translate_table)))
	  || (CHAR_TABLE_P (KVAR! (current_kboard, Vkeyboard_translate_table))
	      && CHARACTERP (c)))
	{
	  let mut d: Lisp_Object;
	  d = Faref (KVAR! (current_kboard, Vkeyboard_translate_table), c);
	  /* nil in keyboard-translate-table means no translation.  */
	  if !NILP (d)
	    { c = d; c_volatile = c; }
	}
    }

  /* If this event is a mouse click in the menu bar,
     return just menu-bar for now.  Modify the mouse click event
     so we won't do this twice, then queue it up.  */
  if (EVENT_HAS_PARAMETERS (c)
      && CONSP (XCDR (c))
      && CONSP (xevent_start (c))
      && CONSP (XCDR (xevent_start (c))))
    {
      let mut posn: Lisp_Object;

      posn = POSN_POSN (xevent_start (c));
      /* Handle menu-bar events:
	 insert the dummy prefix event `menu-bar'.  */
      if EQ (posn, Qmenu_bar ()) || EQ (posn, Qtab_bar ()) || EQ (posn, Qtool_bar ())
	{
	  /* Change menu-bar to (menu-bar) as the event "position".  */
	  POSN_SET_POSN (xevent_start (c), list1 (posn));

	  /* Should a command call `sit-for', or another command that
	     provides a timespec to Fread_event and co., the original
	     event will not subsequently be entered into
	     this_command_keys unless Qt be specified below.

	     The same is the case in a number of other scenarios where
	     reread is true, but if so, event recording is to be
	     suppressed anyway.  */

	  if !end_time.is_null ()
	    { Vunread_command_events = Fcons (Fcons (Qt (), c),
					      Vunread_command_events); }
	  else
	    {
	      also_record = c;
	      Vunread_command_events = Fcons (c, Vunread_command_events);
	    }
	  c = posn;
	  c_volatile = c;
	}
    }

  /* Store these characters into recent_keys, the dribble file if any,
     and the keyboard macro being defined, if any.  */
  record_char (c);
  recorded = true;
  if ! NILP (also_record)
    { record_char (also_record); }

  /* Wipe the echo area.
     But first, if we are about to use an input method,
     save the echo area contents for it to refer to.  */
  if (FIXNUMP (c)
      && ! NILP (Vinput_method_function)
      && b' ' as EMACS_INT <= XFIXNUM (c) && XFIXNUM (c) < 256 && XFIXNUM (c) != 127)
    {
      previous_echo_area_message = Fcurrent_message ();
      Vinput_method_previous_message = previous_echo_area_message;
    }

  /* Now wipe the echo area, except for help events which do their
     own stuff with the echo area.  */
  if (!CONSP (c)
      || (!(EQ (Qhelp_echo (), XCAR (c)))
	  && !(EQ (Qswitch_frame (), XCAR (c)))
	  /* Don't wipe echo area for select window events: These might
	     get delayed via `mouse-autoselect-window' (Bug#11304).  */
	  && !(EQ (Qselect_window (), XCAR (c)))))
    {
      if !NILP (echo_area_buffer[0])
	{
	  safe_run_hooks (Qecho_area_clear_hook ());
	  clear_message (true, false);
	  /* If we were showing the echo-area message on top of an
	     active minibuffer, resize the mini-window, since the
	     minibuffer may need more or less space than the echo area
	     we've just wiped.  */
	  if (minibuf_level != 0
	      && EQ (minibuf_window, echo_area_window)
	      /* The case where minibuffer-message-timeout is a number
		 was already handled near the beginning of command_loop_1.  */
	      && !NUMBERP (Vminibuffer_message_timeout))
	    { resize_mini_window (XWINDOW (minibuf_window), false); }
	}
      else if FUNCTIONP! (Vclear_message_function)
        { clear_message (true, false); }
    }

  /* C 3259/3260: the `reread_for_input_method:` / `from_macro:` labels — the NR body's
     fall-through enters them, which leaves the `wrong_kboard` loop and this block.  */
      break 'wrong_kboard;
  } /* 'wrong_kboard (C 3060) */
   } /* 'reread_for_input_method (C 3259/3260) */
   /* C 3261..3347: the input-method block and the `reread_first` fall-in.  */
  /* Pass this to the input method, if appropriate.  */
  if (FIXNUMP (c)
      && ! NILP (Vinput_method_function)
      /* Don't run the input method within a key sequence,
	 after the first event of the key sequence.  */
      && NILP (prev_event)
      && b' ' as EMACS_INT <= XFIXNUM (c) && XFIXNUM (c) < 256 && XFIXNUM (c) != 127)
    {
      let mut keys: Lisp_Object;
      let mut key_count: ptrdiff_t;
      let mut command_key_start: ptrdiff_t;
      let count: specpdl_ref = SPECPDL_INDEX ();

      /* Save the echo status.  */
      let saved_immediate_echo: bool = kboard_bits! (immediate_echo, current_kboard);
      let saved_ok_to_echo: *mut kboard = ok_to_echo_at_next_pause;
      let saved_echo_string: Lisp_Object = KVAR! (current_kboard, echo_string);
      let saved_echo_prompt: Lisp_Object = KVAR! (current_kboard, echo_prompt);

      /* Save the this_command_keys status.  */
      key_count = this_command_key_count;
      command_key_start = this_single_command_key_start;

      if key_count > 0
	{ keys = Fcopy_sequence (this_command_keys); }
      else
	{ keys = Qnil (); }

      /* Clear out this_command_keys.  */
      this_command_key_count = 0;
      this_single_command_key_start = 0;

      /* Now wipe the echo area.  */
      if !NILP (echo_area_buffer[0])
	{ safe_run_hooks (Qecho_area_clear_hook ()); }
      clear_message (true, false);
      echo_truncate (0);

      /* If we are not reading a key sequence,
	 never use the echo area.  */
      if !KEYMAPP! (map)
	{
	  specbind (Qinput_method_use_echo_area (), Qt ());
	}

      /* Call the input method.  */
      tem = calln! (Vinput_method_function, c);

      tem = unbind_to (count, tem);

      /* Restore the saved echoing state
	 and this_command_keys state.  */
      this_command_key_count = key_count;
      this_single_command_key_start = command_key_start;
      if key_count > 0
	{ this_command_keys = keys; }

      cancel_echoing ();
      ok_to_echo_at_next_pause = saved_ok_to_echo;
      kset_echo_string (current_kboard, saved_echo_string);
      kset_echo_prompt (current_kboard, saved_echo_prompt);
      if saved_immediate_echo
	{ echo_now (); }

      /* The input method can return no events.  */
      if ! CONSP (tem)
	{
	  /* Bring back the previous message, if any.  */
	  if ! NILP (previous_echo_area_message)
	    { message_with_string (c"%s".as_ptr (), previous_echo_area_message, false); }
	  /* C 3331: `goto retry;`.  */
	  continue 'retry;
	}
      /* It returned one event or more.  */
      c = XCAR (tem);
      c_volatile = c;
      Vunread_post_input_method_events
	= nconc2 (XCDR (tem), Vunread_post_input_method_events);
    }
  /* When we consume events from the various unread-*-events lists, we
     bypass the code that records input, so record these events now if
     they were not recorded already.  */
  if !recorded
    {
      record_char (c);
      recorded = true;
    }

   /* C 3348: `reread_first:`.  */

  /* Display help if not echoing.  */
  if (CONSP (c) && EQ (XCAR (c), Qhelp_echo ()))
    {
      /* (help-echo FRAME HELP WINDOW OBJECT POS).  */
      let mut help: Lisp_Object;
      let mut object: Lisp_Object;
      let mut position: Lisp_Object;
      let mut window: Lisp_Object;
      let mut htem: Lisp_Object;

      htem = Fcdr (XCDR (c));
      help = Fcar (htem);
      htem = Fcdr (htem);
      window = Fcar (htem);
      htem = Fcdr (htem);
      object = Fcar (htem);
      htem = Fcdr (htem);
      position = Fcar (htem);

      show_help_echo (help, window, object, position);

      /* We stopped being idle for this event; undo that.  */
      if end_time.is_null ()
	{ timer_resume_idle (); }
      /* C 3370: `goto retry;`.  */
      continue 'retry;
    }

  if ((! reread || this_command_key_count == 0)
      && end_time.is_null ())
    {

      /* Don't echo mouse motion events.  */
      if ! (EVENT_HAS_PARAMETERS (c)
	    && EQ (EVENT_HEAD_KIND (EVENT_HEAD (c)), Qmouse_movement ()))
	/* Once we reread a character, echoing can happen
	   the next time we pause to read a new one.  */
	{ ok_to_echo_at_next_pause = current_kboard; }

      /* Record this character as part of the current key.  */
      add_command_key (c);
      if ! NILP (also_record)
	{ add_command_key (also_record); }

      echo_update ();
    }

  last_input_event = c;
  num_input_events += 1;

  /* Process the help character specially if enabled.  */
  if !NILP (Vhelp_form) && help_char_p (c)
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      help_form_saved_window_configs
	= Fcons (Fcurrent_window_configuration (Qnil ()),
		 help_form_saved_window_configs);
      record_unwind_protect_void (Some (read_char_help_form_unwind));
      call0 (Qhelp_form_show ());

      cancel_echoing ();
      loop
	{
	  c = read_char (0, Qnil (), Qnil (), std::ptr::null_mut (),
			 std::ptr::null_mut ());
	  c_volatile = c;
	  if (EVENT_HAS_PARAMETERS (c)
	      && EQ (EVENT_HEAD_KIND (EVENT_HEAD (c)), Qmouse_click ()))
	    { XSETCAR (help_form_saved_window_configs, Qnil ()); }
	  if !BUFFERP (c) { break; }
	}
      /* Remove the help from the frame.  */
      unbind_to (count, Qnil ());

      redisplay ();
      if BASE_EQ (c, make_fixnum (0o40))
	{
	  cancel_echoing ();
	  loop
	    {
	      c = read_char (0, Qnil (), Qnil (), std::ptr::null_mut (),
			     std::ptr::null_mut ());
	      c_volatile = c;
	      if !BUFFERP (c) { break; }
	    }
	}
    }

   } /* 'reread_first (C 3348) */
  } /* 'exit (C 3429) */

  /* C 3429: `exit:`.  */
  RESUME_POLLING! (polling_stopped_here);
  input_was_pending = input_pending;
  return c;
 } /* 'retry (C 2574) */
}

/* C 3435-3456: `record_menu_key`.  */
unsafe fn record_menu_key (c: Lisp_Object)
{
  /* Wipe the echo area.  */
  clear_message (true, false);

  record_char (c);

  /* Once we reread a character, echoing can happen
     the next time we pause to read a new one.  */
  ok_to_echo_at_next_pause = std::ptr::null_mut ();

  /* Record this character as part of the current key.  */
  add_command_key (c);
  echo_update ();

  /* Re-reading in the middle of a command.  */
  last_input_event = c;
  num_input_events += 1;
}

/* Return true if should recognize C as "the help character".  */

/* C 3460-3471: `help_char_p`.  */
unsafe fn help_char_p (c: Lisp_Object) -> bool
{
  if EQ (c, Vhelp_char)
    { return true; }
  let mut tail: Lisp_Object = Vhelp_event_list;
  FOR_EACH_TAIL_SAFE! (tail, {
    if EQ (c, XCAR (tail))
      { return true; }
  });
  return false;
}

/* Record the input event C in various ways.  */

/* C 3474-3620: `record_char`.  */
unsafe fn record_char (c: Lisp_Object)
{
  /* subr.el/read-passwd binds inhibit_record_char to avoid recording
     passwords.  */
  if !record_all_keys && inhibit_record_char
    { return; }

  let mut recorded: c_int = 0;

  if CONSP (c) && (EQ (XCAR (c), Qhelp_echo ()) || EQ (XCAR (c), Qmouse_movement ()))
    {
      /* To avoid filling recent_keys with help-echo and mouse-movement
	 events, we filter out repeated help-echo events, only store the
	 first and last in a series of mouse-movement events, and don't
	 store repeated help-echo events which are only separated by
	 mouse-movement events.  */

      let mut ev1: Lisp_Object;
      let mut ev2: Lisp_Object;
      let mut ev3: Lisp_Object;
      let mut ix1: c_int;
      let mut ix2: c_int;
      let mut ix3: c_int;

      ix1 = recent_keys_index - 1;
      if ix1 < 0
	{ ix1 = lossage_limit - 1; }
      ev1 = AREF (recent_keys, ix1 as isize);

      ix2 = ix1 - 1;
      if ix2 < 0
	{ ix2 = lossage_limit - 1; }
      ev2 = AREF (recent_keys, ix2 as isize);

      ix3 = ix2 - 1;
      if ix3 < 0
	{ ix3 = lossage_limit - 1; }
      ev3 = AREF (recent_keys, ix3 as isize);

      if EQ (XCAR (c), Qhelp_echo ())
	{
	  /* Don't record `help-echo' in recent_keys unless it shows some help
	     message, and a different help than the previously recorded
	     event.  */
	  let mut help: Lisp_Object;
	  let mut last_help: Lisp_Object;

	  help = Fcar_safe (Fcdr_safe (XCDR (c)));
	  if !STRINGP (help)
	    { recorded = 1; }
	  else if CONSP (ev1) && EQ (XCAR (ev1), Qhelp_echo ())
		   && { last_help = Fcar_safe (Fcdr_safe (XCDR (ev1))); EQ (last_help, help) }
	    { recorded = 1; }
	  else if CONSP (ev1) && EQ (XCAR (ev1), Qmouse_movement ())
		   && CONSP (ev2) && EQ (XCAR (ev2), Qhelp_echo ())
		   && { last_help = Fcar_safe (Fcdr_safe (XCDR (ev2))); EQ (last_help, help) }
	    { recorded = -1; }
	  else if CONSP (ev1) && EQ (XCAR (ev1), Qmouse_movement ())
		   && CONSP (ev2) && EQ (XCAR (ev2), Qmouse_movement ())
		   && CONSP (ev3) && EQ (XCAR (ev3), Qhelp_echo ())
		   && { last_help = Fcar_safe (Fcdr_safe (XCDR (ev3))); EQ (last_help, help) }
	    { recorded = -2; }
	}
      else if EQ (XCAR (c), Qmouse_movement ())
	{
	  /* Only record one pair of `mouse-movement' on a window in recent_keys.
	     So additional mouse movement events replace the last element.  */
	  let mut last_window: Lisp_Object;
	  let mut window: Lisp_Object;

	  window = Fcar_safe (Fcar_safe (XCDR (c)));
	  if CONSP (ev1) && EQ (XCAR (ev1), Qmouse_movement ())
	      && { last_window = Fcar_safe (Fcar_safe (XCDR (ev1))); EQ (last_window, window) }
	      && CONSP (ev2) && EQ (XCAR (ev2), Qmouse_movement ())
	      && { last_window = Fcar_safe (Fcar_safe (XCDR (ev2))); EQ (last_window, window) }
	    {
	      ASET (recent_keys, ix1 as isize, c);
	      recorded = 1;
	    }
	}
    }
  else if NILP (Vexecuting_kbd_macro)
    { store_kbd_macro_char (c); }

  /* recent_keys should not include events from keyboard macros.  */
  if NILP (Vexecuting_kbd_macro)
    {
      if recorded == 0
	{
	  total_keys += (total_keys < lossage_limit) as c_int;
	  ASET (recent_keys, recent_keys_index as isize,
                /* Copy the event, in case it gets modified by side-effect
                   by some remapping function (bug#30955).  */
                if CONSP (c) { Fcopy_sequence (c) } else { c });
	  recent_keys_index += 1;
	  if recent_keys_index >= lossage_limit
	    { recent_keys_index = 0; }
	}
      else if recorded < 0
	{
	  /* We need to remove one or two events from recent_keys.
	     To do this, we simply put nil at those events and move the
	     recent_keys_index backwards over those events.  Usually,
	     users will never see those nil events, as they will be
	     overwritten by the command keys entered to see recent_keys
	     (e.g. C-h l).  */

	  /* C's `while (recorded++ < 0 && …)`: the post-increment happens on the test that
	     fails too, so the port keeps the increment inside the condition (the block
	     expression is evaluated once per test, as C's operand is).  */
	  while { recorded += 1; recorded - 1 < 0 } && total_keys > 0
	    {
	      if total_keys < lossage_limit
		{ total_keys -= 1; }
	      recent_keys_index -= 1;
	      if recent_keys_index < 0
		{ recent_keys_index = lossage_limit - 1; }
	      ASET (recent_keys, recent_keys_index as isize, Qnil ());
	    }
	}

      num_nonmacro_input_events += 1;
    }

  /* Write c to the dribble file.  If c is a lispy event, write
     the event's symbol to the dribble file, in <brackets>.  Bleaugh.
     If you, dear reader, have a better idea, you've got the source.  :-) */
  if !dribble.is_null () && NILP (Vexecuting_kbd_macro)
    {
      block_input ();
      if FIXNUMP (c)
	{
	  if XUFIXNUM (c) < 0x100
	    { putc (XUFIXNUM (c) as c_int, dribble); }
	  else
	    { fprintf (dribble, c" 0x%lx".as_ptr (), XUFIXNUM (c)); }
	}
      else
	{
	  let mut dribblee: Lisp_Object;

	  /* If it's a structured event, take the event header.  */
	  dribblee = EVENT_HEAD (c);

	  if SYMBOLP (dribblee)
	    {
	      putc (b'<' as c_int, dribble);
	      fwrite (SDATA (SYMBOL_NAME (dribblee)) as *const c_void,
		      std::mem::size_of::<c_char> (), SBYTES (SYMBOL_NAME (dribblee)) as usize,
		      dribble);
	      putc (b'>' as c_int, dribble);
	    }
	}

      fflush (dribble);
      unblock_input ();
    }
}

/* Copy out or in the info on where C-g should throw to.
   This is used when running Lisp code from within get_char,
   in case get_char is called recursively.
   See read_process_output.  */

/* C 3626-3630: `save_getcjmp` (C's `sys_jmp_buf` parameter is the `jmp_buf` array decaying
   to a pointer, and the body writes into the caller's buffer).  */
unsafe fn save_getcjmp (temp: *mut sys_jmp_buf)
{
  memcpy (temp as *mut c_void, (&raw const getcjmp! ()) as *const c_void,
	  std::mem::size_of_val (&getcjmp! ()));
}

/* C 3632-3636: `restore_getcjmp` — C hands its address to `record_unwind_protect_ptr`, so
   the port defines it `unsafe extern "C" fn` (PORTING.md 1.4).  */
unsafe extern "C" fn restore_getcjmp (temp: *mut c_void)
{
  memcpy ((&raw mut getcjmp! ()) as *mut c_void, temp, std::mem::size_of_val (&getcjmp! ()));
}

/* Low level keyboard/mouse input.
   kbd_buffer_store_event places events in kbd_buffer, and
   kbd_buffer_get_event retrieves them.  */

/* Return true if there are any events in the queue that read-char
   would return.  If this returns false, a read-char would block.  */
/* C 3641-3722: `readable_events`.  */
unsafe fn readable_events (flags: c_int) -> bool
{
  if (flags & READABLE_EVENTS_DO_TIMERS_NOW) != 0
    { timer_check (); }

  /* READABLE_EVENTS_FILTER_EVENTS is meant to be used only by
     input-pending-p and similar callers, which aren't interested in
     some input events.  If this flag is set, and
     input-pending-p-filter-events is non-nil, ignore events in
     while-no-input-ignore-events.  If the flag is set and
     input-pending-p-filter-events is nil, ignore only
     FOCUS_IN/OUT_EVENT events.  */
  if kbd_fetch_ptr != kbd_store_ptr
    {
      /* See https://lists.gnu.org/r/emacs-devel/2005-05/msg00297.html
	 for why we treat toolkit scroll-bar events specially here.  */
      /* C 3678-3712: `#ifdef USE_TOOLKIT_SCROLL_BARS` (carried per P-51; the macro is
	 `#undef` on this target, so the port keeps both arms, the live one first).  */
      if (flags & (READABLE_EVENTS_FILTER_EVENTS)) != 0
        {
          let mut event: *mut buffered_input_event = kbd_fetch_ptr;
	  loop
	    {
	      #[cfg (feature = "use_toolkit_scroll_bars")]
	      if !(
		    (flags & READABLE_EVENTS_FILTER_EVENTS) != 0 &&
		    ((!input_pending_p_filter_events
		      && ((*event).kind == FOCUS_IN_EVENT as u16
			  || (*event).kind == FOCUS_OUT_EVENT as u16))
		     || (input_pending_p_filter_events
			 && is_ignored_event (event)))
		  && !((flags & READABLE_EVENTS_IGNORE_SQUEEZABLES) != 0
		       && ((*event).kind == SCROLL_BAR_CLICK_EVENT as u16
			   || (*event).kind == HORIZONTAL_SCROLL_BAR_CLICK_EVENT as u16)
		       && input_event_bits! (part, &raw mut (*event).ie) == scroll_bar_handle
		       && (*event).ie.modifiers == 0)
		 )
		{ return true; }
	      #[cfg (not (feature = "use_toolkit_scroll_bars"))]
	      if !(
		    ((!input_pending_p_filter_events
		      && ((*event).kind == FOCUS_IN_EVENT as u16
			  || (*event).kind == FOCUS_OUT_EVENT as u16))
		     || (input_pending_p_filter_events
			 && is_ignored_event (event)))
		 )
		{ return true; }
	      event = next_kbd_event (event);
	      if !(event != kbd_store_ptr) { break; }
	    }
        }
      else
	{ return true; }
    }

  /* C 3714-3717: `#ifdef HAVE_X_WINDOWS` (carried per P-51; `HAVE_X_WINDOWS` is `#undef`).  */
  #[cfg (feature = "have_x_windows")]
  {
    if x_detect_pending_selection_requests ()
      { return true; }
  }

  /* C 3719-3722: `#ifdef HAVE_TEXT_CONVERSION` (carried per P-51; `HAVE_TEXT_CONVERSION` is
     `#undef`).  */
  #[cfg (feature = "have_text_conversion")]
  {
    if detect_conversion_events ()
      { return true; }
  }

  if (flags & READABLE_EVENTS_IGNORE_SQUEEZABLES) == 0 && !some_mouse_moved ().is_null ()
    { return true; }
  if single_kboard
    {
      if (*current_kboard).kbd_queue_has_data
	{ return true; }
    }
  else
    {
      let mut kb: *mut KBOARD;
      kb = all_kboards;
      while !kb.is_null ()
	{
	  if (*kb).kbd_queue_has_data
	    { return true; }
	  kb = (*kb).next_kboard;
	}
    }
  return false;
}

/* termhooks.h 373-412: `enum event_kind kind : 16` and `enum scroll_bar_part part : 16` share
   the mirror's `input_event::bits` unit (PORTING.md 1.3/P-80's member-named shape); the
   dispnew.rs `WINDOW_FRAME` precedent for a per-unit macro.  */
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* window.h: `WINDOW_FRAME` is the window's `frame` field; the port keeps the name (the
   dispnew.rs precedent).  */
macro_rules! WINDOW_FRAME { ($w:expr) => { (*$w).frame }; }

/* Set this for debugging, to have a way to get out */
/* C 3724-3725: `extern int stop_character;` and its `EXTERNALLY_VISIBLE` definition.  */
#[unsafe(no_mangle)]
pub static mut stop_character: c_int = 0;

/* C 3728-3746: `event_to_kboard`.  */
unsafe fn event_to_kboard (event: *mut input_event) -> *mut KBOARD
{
  /* Not applicable for these special events.  */
  if input_event_bits! (kind, event) == SELECTION_REQUEST_EVENT
      || input_event_bits! (kind, event) == SELECTION_CLEAR_EVENT
    { return std::ptr::null_mut (); }
  else
    {
      let mut obj: Lisp_Object = (*event).frame_or_window;
      /* There are some events that set this field to nil or string.  */
      if WINDOWP (obj)
	{ obj = WINDOW_FRAME! (XWINDOW (obj)); }
      /* Also ignore dead frames here.  */
      return if FRAMEP (obj) && FRAME_LIVE_P (XFRAME (obj))
	     { FRAME_KBOARD (XFRAME (obj)) as *mut KBOARD } else { std::ptr::null_mut () };
    }
}

/* C 3748-3757: `#ifdef subprocesses` (the macro is 1 on this target, so the body is live).  */
/* Return the number of slots occupied in kbd_buffer.  */

unsafe fn kbd_buffer_nr_stored () -> c_int
{
  let n: c_int = kbd_store_ptr.offset_from (kbd_fetch_ptr) as c_int;
  return n + (if n < 0 { KBD_BUFFER_SIZE } else { 0 });
}

/* `keyboard.h` 498 and 499 declare `kbd_buffer_store_event` and
   `kbd_buffer_store_buffered_event` at the crate root (`include/keyboard.rs`'s two
   `unsafe extern "C"` blocks), so their definitions live in this module and are glob-imported
   below: a crate-root definition would be `E0428` (the `keyboard_root_shadowed` shape above).
   The mirror spells the first one's parameter `*mut c_void`, so its call sites cast.  */
pub mod keyboard_root_shadowed_2 {
  use super::*;

/* C 3756-3761: `kbd_buffer_store_event`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kbd_buffer_store_event (event: *mut input_event)
{
  kbd_buffer_store_event_hold (event, std::ptr::null_mut ());
}

/* Store EVENT obtained at interrupt level into kbd_buffer, fifo.

   If HOLD_QUIT is 0, just stuff EVENT into the fifo.
   Else, if HOLD_QUIT.kind != NO_EVENT, discard EVENT.
   Else, if EVENT is a quit event, store the quit event
   in HOLD_QUIT, and return (thus ignoring further events).

   This is used to postpone the processing of the quit event until all
   subsequent input events have been parsed (and discarded).  */

/* C 3774-3878: `kbd_buffer_store_buffered_event`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kbd_buffer_store_buffered_event (event: *mut buffered_input_event,
							 hold_quit: *mut input_event)
{
  if (*event).kind == NO_EVENT as u16
    { emacs_abort (); }

  if !hold_quit.is_null () && input_event_bits! (kind, hold_quit) != NO_EVENT
    { return; }

  if (*event).kind == ASCII_KEYSTROKE_EVENT as u16
    {
      let mut c: c_int = ((*event).ie.code & 0o377) as c_int;

      if ((*event).ie.modifiers & ctrl_modifier as c_uint) != 0
	{ c = make_ctrl_char (c); }

      c |= ((*event).ie.modifiers
	    & ((meta_modifier | alt_modifier
		| hyper_modifier | super_modifier) as c_uint)) as c_int;

      if c == quit_char
	{
	  let kb: *mut KBOARD
	    = FRAME_KBOARD (XFRAME ((*event).ie.frame_or_window)) as *mut KBOARD;

	  /* If C-g arrives while we're in the run state on behalf of
	     another display, just clear the queue and deposit the C-g
	     to be read later.  */
	  if single_kboard && kb != current_kboard
	    {
	      kset_kbd_queue
		(kb, list2 (make_lispy_switch_frame ((*event).ie.frame_or_window),
			    make_fixnum (c as EMACS_INT)));
	      (*kb).kbd_queue_has_data = true;

	      let mut sp: *mut buffered_input_event = kbd_fetch_ptr;
	      while sp != kbd_store_ptr
		{
		  if event_to_kboard (&raw mut (*sp).ie) == kb
		    {
		      input_event_bits! (kind, &raw mut (*sp).ie, NO_EVENT);
		      (*sp).ie.frame_or_window = Qnil ();
		      (*sp).ie.arg = Qnil ();
		    }
		  sp = next_kbd_event (sp);
		}
	      return;
	    }

	  if !hold_quit.is_null ()
	    {
	      *hold_quit = (*event).ie;
	      return;
	    }

	  /* If this results in a quit_char being returned to Emacs as
	     input, set Vlast_event_frame properly.  If this doesn't
	     get returned to Emacs as an event, the next event read
	     will set Vlast_event_frame again, so this is safe to do.  */
	  {
	    let mut focus: Lisp_Object;

	    focus = FRAME_FOCUS_FRAME (XFRAME ((*event).ie.frame_or_window));
	    if NILP (focus)
	      { focus = (*event).ie.frame_or_window; }
	    internal_last_event_frame = focus;
	    Vlast_event_frame = focus;
	  }

	  handle_interrupt (false);
	  return;
	}

      if c != 0 && c == stop_character
	{
	  sys_suspend ();
	  return;
	}
    }

  /* Don't let the very last slot in the buffer become full,
     since that would make the two pointers equal,
     and that is indistinguishable from an empty buffer.
     Discard the event if it would fill the last slot.  */
  let next_slot: *mut buffered_input_event = next_kbd_event (kbd_store_ptr);
  if kbd_fetch_ptr != next_slot
    {
      /* C's `*kbd_store_ptr = *event;` — the mirror's union is not `Copy`, so the port
	 copies the one element (the buffer.rs `copy_nonoverlapping` shape).  */
      std::ptr::copy_nonoverlapping (event, kbd_store_ptr, 1);
      kbd_store_ptr = next_slot;
      /* C 3861-3874: `#ifdef subprocesses` (the macro is 1 on this target, so the text is
	 live).  */
      if kbd_buffer_nr_stored () > KBD_BUFFER_SIZE / 2
	  && ! kbd_on_hold_p ()
        {
          /* Don't read keyboard input until we have processed kbd_buffer.
             This happens when pasting text longer than KBD_BUFFER_SIZE/2.  */
          hold_keyboard_input ();
          unrequest_sigio ();
          stop_polling ();
        }
    }

  /* If we're inside while-no-input, and this event qualifies
     as input, set quit-flag to cause an interrupt.  */
  if !NILP (Vthrow_on_input)
      && !is_ignored_event (event)
    { Vquit_flag = Vthrow_on_input; }
}
}
use keyboard_root_shadowed_2::*;

/* Limit help event positions to this range, to avoid overflow problems.  */
/* gnulib intprops.h's `TYPE_MAXIMUM (t)` has no mirror; the port keeps the name with the one
   instantiation this file needs (the `max!`/`min!` precedent; `PTRDIFF_MIN`/`PTRDIFF_MAX` come
   from `lib/include/stdint.in.rs`).  */
macro_rules! TYPE_MAXIMUM { (Time) => { Time::MAX as ptrdiff_t }; }
/* C 3880-3881: `INPUT_EVENT_POS_MAX` (C's body is a statement expression; the port spells the
   same constant expression).  */
pub const INPUT_EVENT_POS_MAX: ptrdiff_t =
  min! (PTRDIFF_MAX, min! (TYPE_MAXIMUM! (Time) / 2, MOST_POSITIVE_FIXNUM));
/* C 3885-3886: `INPUT_EVENT_POS_MIN`.  */
pub const INPUT_EVENT_POS_MIN: ptrdiff_t =
  if PTRDIFF_MIN < -INPUT_EVENT_POS_MAX { -1 - INPUT_EVENT_POS_MAX } else { PTRDIFF_MIN };

/* Return a Time that encodes position POS.  POS must be in range.  */

/* C 3891-3897: `position_to_Time`.  */
unsafe fn position_to_Time (pos: ptrdiff_t) -> Time
{
  eassert! (INPUT_EVENT_POS_MIN <= pos && pos <= INPUT_EVENT_POS_MAX);
  return pos as Time;
}

/* Return the position that ENCODED_POS encodes.
   Avoid signed integer overflow.  */

/* C 3901-3912: `Time_to_position`.  */
unsafe fn Time_to_position (encoded_pos: Time) -> ptrdiff_t
{
  if (encoded_pos as ptrdiff_t) <= INPUT_EVENT_POS_MAX
    { return encoded_pos as ptrdiff_t; }
  let encoded_pos_min: Time = position_to_Time (INPUT_EVENT_POS_MIN);
  eassert! (encoded_pos_min <= encoded_pos);
  let notpos: ptrdiff_t = -1 - encoded_pos as ptrdiff_t;
  return -1 - notpos;
}

/* Generate a HELP_EVENT input_event and store it in the keyboard
   buffer.

   HELP is the help form.

   FRAME and WINDOW are the frame and window where the help is
   generated.  OBJECT is the Lisp object where the help was found (a
   buffer, a string, an overlay, or nil if neither from a string nor
   from a buffer).  POS is the position within OBJECT where the help
   was found.  */

/* C 3923-3940: `gen_help_event`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gen_help_event (help: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
					 object: Lisp_Object, pos: ptrdiff_t)
{
  let mut event: input_event = std::mem::zeroed ();
  EVENT_INIT (&raw mut event);

  input_event_bits! (kind, &raw mut event, HELP_EVENT);
  event.frame_or_window = frame;
  event.arg = object;
  event.x = if WINDOWP (window) { window } else { frame };
  event.y = help;
  event.timestamp = position_to_Time (pos);
  kbd_buffer_store_event (&raw mut event as *mut c_void);
}


/* Store HELP_EVENTs for HELP on FRAME in the input queue.  */

/* C 3942-3958: `kbd_buffer_store_help_event`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kbd_buffer_store_help_event (frame: Lisp_Object, help: Lisp_Object)
{
  let mut event: input_event = std::mem::zeroed ();
  EVENT_INIT (&raw mut event);

  input_event_bits! (kind, &raw mut event, HELP_EVENT);
  event.frame_or_window = frame;
  event.arg = Qnil ();
  event.x = Qnil ();
  event.y = help;
  event.timestamp = 0;
  kbd_buffer_store_event (&raw mut event as *mut c_void);
}

/* Discard any mouse events in the event buffer by setting them to
   NO_EVENT.  */
/* C 3960-3982: `discard_mouse_events`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn discard_mouse_events ()
{
  let mut sp: *mut buffered_input_event = kbd_fetch_ptr;
  while sp != kbd_store_ptr
    {
      if (*sp).kind == MOUSE_CLICK_EVENT as u16
	  || (*sp).kind == WHEEL_EVENT as u16
          || (*sp).kind == HORIZ_WHEEL_EVENT as u16
	  || (*sp).kind == SCROLL_BAR_CLICK_EVENT as u16
	  || (*sp).kind == HORIZONTAL_SCROLL_BAR_CLICK_EVENT as u16
	{
	  (*sp).kind = NO_EVENT as u16;
	}
      sp = next_kbd_event (sp);
    }
}


/* Return true if there are any real events waiting in the event
   buffer, not counting `NO_EVENT's.

   Discard NO_EVENT events at the front of the input queue, possibly
   leaving the input queue empty if there are no real input events.  */

/* C 3988-3998: `kbd_buffer_events_waiting`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kbd_buffer_events_waiting () -> bool
{
  let mut sp: *mut buffered_input_event = kbd_fetch_ptr;
  loop
    {
      if sp == kbd_store_ptr || (*sp).kind != NO_EVENT as u16
	{
	  kbd_fetch_ptr = sp;
	  return sp != kbd_store_ptr && (*sp).kind != NO_EVENT as u16;
	}
      sp = next_kbd_event (sp);
    }
}

/* termchar.h 217: `bool_bf showing_menu : 1` is packed into the mirror's
   `tty_display_info::bits_caps` (termchar.rs 241's `TTY_SHOWING_MENU`); the port reads it
   through the member's name (P-80's shape, the term.rs `tty_bits!` precedent).  */
macro_rules! tty_bits {
  (showing_menu, $tty:expr) => { ((*$tty).bits_caps & TTY_SHOWING_MENU) != 0 };
  (flow_control, $tty:expr) => { ((*$tty).bits_caps & TTY_FLOW_CONTROL) != 0 };
  (flow_control, $tty:expr, $v:expr) => {
    { if ($v) != 0 { (*$tty).bits_caps |= TTY_FLOW_CONTROL; }
      else { (*$tty).bits_caps &= ! TTY_FLOW_CONTROL; } } };
}

/* termchar.h 235's `CURTTY ()` reaches this file as a macro; the port keeps the name (the
   `WINDOW_FRAME` precedent).  */
macro_rules! CURTTY { () => {
  (*FRAME_TERMINAL! (SELECTED_FRAME ())).display_info.tty as *mut tty_display_info
} }

/* Clear input event EVENT.  */

/* C 3999-4003: `clear_event`.  */
unsafe fn clear_event (event: *mut input_event)
{
  input_event_bits! (kind, event, NO_EVENT);
}

/* C 4005-4018: `kbd_buffer_get_event_1` (C hands its address to
   `internal_condition_case_1`, so the port defines it `unsafe extern "C" fn`).  */
unsafe extern "C" fn kbd_buffer_get_event_1 (arg: Lisp_Object) -> Lisp_Object
{
  let coding_system: Lisp_Object = Fget_text_property (make_fixnum (0),
						       Qcoding (), arg);

  if EQ (coding_system, Qt ())
    { return arg; }

  return code_convert_string (arg, if !NILP (coding_system)
				   { coding_system }
				   else { Vlocale_coding_system },
			      Qnil (), false, false, 0);
}

/* C 4020-4023: `kbd_buffer_get_event_2`.  */
unsafe extern "C" fn kbd_buffer_get_event_2 (val: Lisp_Object) -> Lisp_Object
{
  return Qnil ();
}

/* Read one event from the event buffer, waiting if necessary.
   The value is a Lisp object representing the event.
   The value is nil for an event that should be ignored,
   or that was handled here.
   We always read and discard one event.  */

/* C 4032-4593: `kbd_buffer_get_event`.  */
unsafe fn kbd_buffer_get_event (kbp: *mut *mut KBOARD,
                                used_mouse_menu: *mut bool,
                                event_frame: *mut *mut frame,
                                end_time: *mut timespec) -> Lisp_Object
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut obj: Lisp_Object = Qnil ();
  let mut str: Lisp_Object = Qnil ();
  /* C 4037-4042: `#ifdef HAVE_X_WINDOWS`'s `had_pending_selection_requests` (carried per
     P-51) and `#ifdef HAVE_TEXT_CONVERSION`'s `had_pending_conversion_events`, which the
     chain below reads in both configurations, so it is declared unconditionally.  */
  #[cfg (feature = "have_x_windows")]
  let mut had_pending_selection_requests: bool = false;
  let mut had_pending_conversion_events: bool = false;

  *event_frame = std::ptr::null_mut ();

  /* C 4047-4057: `#ifdef subprocesses` (the macro is 1 on this target, so the text is live).  */
  if kbd_on_hold_p () && kbd_buffer_nr_stored () < KBD_BUFFER_SIZE / 4
    {
      /* Start reading input again because we have processed enough to
         be able to accept new events again.  */
      unhold_keyboard_input ();
      request_sigio ();
      start_polling ();
    }

  /* C 4058-4077: `#if !defined HAVE_DBUS && !defined USE_FILE_NOTIFY && !defined
     THREADS_ENABLED` (all three `#undef` on this target, so the text is live).  */
  if noninteractive
      /* In case we are running as a daemon, only do this before
	 detaching from the terminal.  */
      || (IS_DAEMON () && DAEMON_RUNNING ())
    {
      let c: c_int = getchar ();
      XSETINT! (obj, c as EMACS_INT);
      *kbp = current_kboard;
      return obj;
    }

  *kbp = current_kboard;

  /* Wait until there is input available.  */
  loop
    {
      /* Break loop if there's an unread command event.  Needed in
	 moused window autoselection which uses a timer to insert such
	 events.  */
      if CONSP (Vunread_command_events)
	{ break; }

      /* C 4082-4092: `#ifdef HAVE_TEXT_CONVERSION` (carried per P-51).  */
      #[cfg (feature = "have_text_conversion")]
      {
      /* That text conversion events take priority over keyboard
	 events, since input methods frequently send them immediately
	 after edits, with the assumption that this order of events
	 will be observed.  */

      if detect_conversion_events ()
	{
	  had_pending_conversion_events = true;
	  break;
	}
      }

      if kbd_fetch_ptr != kbd_store_ptr
	{ break; }
      if !some_mouse_moved ().is_null ()
	{ break; }

      /* If the quit flag is set, then read_char will return
	 quit_char, so that counts as "available input."  */
      if !NILP (Vquit_flag)
	{ quit_throw_to_read_char (false); }

      /* One way or another, wait until input is available; then, if
	 interrupt handlers have not read it, read it now.  */

      /* C 4108-4110: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` — **live** on
	 this target (`USABLE_SIGIO` is 1 in the frozen configuration; measured with clang),
	 so the call is not gated (the docs/ports/dispnew.md "carried text that is live"
	 reading).  */
      gobble_input ();

      if kbd_fetch_ptr != kbd_store_ptr
	{ break; }
      if !some_mouse_moved ().is_null ()
	{ break; }
      /* C 4116-4125: `#ifdef HAVE_X_WINDOWS` (carried per P-51).  */
      #[cfg (feature = "have_x_windows")]
      {
      if x_detect_pending_selection_requests ()
	{
	  had_pending_selection_requests = true;
	  break;
	}
      }
      if !end_time.is_null ()
	{
	  let now: timespec = current_timespec ();
	  if timespec_cmp (*end_time, now) <= 0
	    { return Qnil (); }	/* Finished waiting.  */
	  else
	    {
	      let duration: timespec = timespec_sub (*end_time, now);
	      wait_reading_process_output (min! (duration.tv_sec,
						 WAIT_READING_MAX),
					   duration.tv_nsec as c_int,
					   -1, true, Qnil (), std::ptr::null_mut (), 0);
	    }
	}
      else
	{
	  let mut do_display: bool = true;

	  if FRAME_TERMCAP_P (SELECTED_FRAME ())
	    {
	      let tty: *mut tty_display_info = CURTTY! ();

	      /* When this TTY is displaying a menu, we must prevent
		 any redisplay, because we modify the frame's glyph
		 matrix behind the back of the display engine.  */
	      if tty_bits! (showing_menu, tty)
		{ do_display = false; }
	    }

	  wait_reading_process_output (0, 0, -1, do_display, Qnil (), std::ptr::null_mut (), 0);
	}

      if !interrupt_input && kbd_fetch_ptr == kbd_store_ptr
	{ gobble_input (); }
    }

  /* C 4128-4136: `#ifdef HAVE_X_WINDOWS`'s `x_handle_pending_selection_requests`
     (carried per P-51).  */
  #[cfg (feature = "have_x_windows")]
  {
  /* Handle pending selection requests.  This can happen if Emacs
     enters a recursive edit inside a nested event loop (probably
     because the debugger opened) or someone called
     `read-char'.  */

  if had_pending_selection_requests
    { x_handle_pending_selection_requests (); }
  }

  if CONSP (Vunread_command_events)
    {
      let mut first: Lisp_Object;
      first = XCAR (Vunread_command_events);
      Vunread_command_events = XCDR (Vunread_command_events);
      *kbp = current_kboard;
      return first;
    }

  /* C 4148-4176: `#ifdef HAVE_TEXT_CONVERSION` cuts C's chain in two (`if (…) { … } else`
     before the live `if`); Rust cannot let a `#[cfg]` cut an `if`/`else`, so the port tests
     the carried flag in the chain's head and keeps the arm's own text behind its `#[cfg]`
     (PORTING.md 2.2's explicit-restructuring rule).  */
  if cfg! (feature = "have_text_conversion") && had_pending_conversion_events
    {
      #[cfg (feature = "have_text_conversion")]
      {
      /* There are pending text conversion operations.  Text conversion
	 events should be generated before processing any other keyboard
	 input.  */
      handle_pending_conversion_events ();
      obj = Qtext_conversion ();

      /* See the comment in handle_pending_conversion_events_1.
         Note that in addition, text conversion events are not
         generated if no edits were actually made.  */
      if conversion_disabled_p ()
	  || NILP (Vtext_conversion_edits)
	{ obj = Qnil (); }
      }
    }
  /* At this point, we know that there is a readable event available
     somewhere.  If the event queue is empty, then there must be a
     mouse movement enabled and available.  */
  else if kbd_fetch_ptr != kbd_store_ptr
    {
      let mut event: *mut buffered_input_event = kbd_fetch_ptr;

      *kbp = event_to_kboard (&raw mut (*event).ie);
      if (*kbp).is_null ()
	{ *kbp = current_kboard; }  /* Better than returning null ptr?  */

      /* Selection-request events never have frames, they are actually
         instances of 'struct selection_input_event'.  */
      if !((*event).kind == SELECTION_REQUEST_EVENT as u16
	    || (*event).kind == SELECTION_CLEAR_EVENT as u16)
	  && FRAMEP ((*event).ie.frame_or_window)
	{ *event_frame = XFRAME ((*event).ie.frame_or_window); }

      obj = Qnil ();

      /* These two kinds of events get special handling
	 and don't actually appear to the command loop.
	 We return nil for them.  */
      /* C 4223-4530: C's `switch (event->kind)`: the port's `match` on the same arms in the
	 same order, the `#if`-only arms kept under their `#[cfg]` (P-51).  C's `break`s that
	 leave the switch are `break 'switch`.  */
      'switch: {
      match (*event).kind as event_kind
	{
        /* C 4225-4246: `#ifndef HAVE_HAIKU` is live; inside it `#if defined HAVE_X11 ||
	   HAVE_PGTK` is carried (both `#undef`).  */
        SELECTION_REQUEST_EVENT | SELECTION_CLEAR_EVENT => {
	  #[cfg (feature = "have_x11_or_have_pgtk")]
	  {
	  /* Remove it from the buffer before processing it,
	     since otherwise swallow_events will see it
	     and process it again.  */
	  let copy: selection_input_event = (*event).sie;
	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);

	  #[cfg (feature = "have_x11")]
	  { x_handle_selection_event (&raw const copy); }
	  #[cfg (not (feature = "have_x11"))]
	  { pgtk_handle_selection_event (&raw const copy); }
	  }
	  #[cfg (not (feature = "have_x11_or_have_pgtk"))]
	  {
	  /* We're getting selection request events, but we don't have
             a window system.  */
	  emacs_abort ();
	  }
	}
        /* C 4249-4262: the `#else` arm of `#ifndef HAVE_HAIKU` (carried per P-51).  */
        #[cfg (feature = "have_haiku")]
        SELECTION_REQUEST_EVENT => {
	  emacs_abort ();
	}
        #[cfg (feature = "have_haiku")]
        SELECTION_CLEAR_EVENT => {
	  let copy: input_event = (*event).ie;

	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);
	  haiku_handle_selection_clear (&raw const copy);
	}

        /* C 4264-4275.  */
        MONITORS_CHANGED_EVENT => {
	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);

	  CALLN! (Frun_hook_with_args,
		  Qdisplay_monitors_changed_functions (),
		  (*event).ie.arg);
	}

        /* C 4277-4286: `#ifdef HAVE_ANDROID` (carried per P-51).  */
        #[cfg (feature = "have_android")]
        NOTIFICATION_EVENT => {
	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);
	  CALLN! (Fapply, XCAR ((*event).ie.arg), XCDR ((*event).ie.arg));
	}

        /* C 4288-4295.  */
        TOOLKIT_THEME_CHANGED_EVENT => {
	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);

	  Vtoolkit_theme = (*event).ie.arg;
	  CALLN! (Frun_hook_with_args, Qtoolkit_theme_set_functions (),
		  (*event).ie.arg);
	}

        /* C 4297-4309: `#ifdef HAVE_EXT_MENU_BAR` (the macro is lisp.h 3929-3932's conditional
	   define and is undefined on this target; carried per P-51).  */
        #[cfg (feature = "have_ext_menu_bar")]
        MENU_BAR_ACTIVATE_EVENT => {
          let f: *mut frame;
	  kbd_fetch_ptr = next_kbd_event (event);
	  input_pending = readable_events (0);
          f = XFRAME ((*event).ie.frame_or_window);
	  if FRAME_LIVE_P (f) && (*FRAME_TERMINAL! (f)).activate_menubar_hook.is_some ()
	    { ((*FRAME_TERMINAL! (f)).activate_menubar_hook.unwrap ()) (f); }
	}

        /* C 4311-4346: the grouped arms.  `#if defined (HAVE_NS)`'s `NS_TEXT_EVENT` is carried
	   (it falls through into the shared body), and the live list starts at
	   `PREEDIT_TEXT_EVENT`; the `#ifdef` groups between them (`HAVE_NTGUI`,
	   `HAVE_WINDOW_SYSTEM`, `USE_FILE_NOTIFY`, `HAVE_DBUS`, `THREADS_ENABLED`,
	   `HAVE_XWIDGETS`) are carried, each with the shared body C's group reaches.  */
        #[cfg (feature = "have_ns")]
        NS_TEXT_EVENT => {
	  if !used_mouse_menu.is_null ()
	    { *used_mouse_menu = true; }
	  /* FALLTHROUGH */
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "have_ntgui")]
        END_SESSION_EVENT | LANGUAGE_CHANGE_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "have_window_system")]
        DELETE_WINDOW_EVENT | ICONIFY_EVENT | DEICONIFY_EVENT | MOVE_FRAME_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "use_file_notify")]
        FILE_NOTIFY_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "have_dbus")]
        DBUS_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "threads_enabled")]
        THREAD_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        #[cfg (feature = "have_xwidgets")]
        XWIDGET_EVENT | XWIDGET_DISPLAY_EVENT => {
	  obj = make_lispy_event (&raw mut (*event).ie);
	  kbd_fetch_ptr = next_kbd_event (event);
	}
        PREEDIT_TEXT_EVENT
        | SAVE_SESSION_EVENT
        | NO_EVENT
        | HELP_EVENT
        | FOCUS_IN_EVENT
        | CONFIG_CHANGED_EVENT
        | FOCUS_OUT_EVENT
        | SELECT_WINDOW_EVENT
        | SLEEP_EVENT => {
          obj = make_lispy_event (&raw mut (*event).ie);
          kbd_fetch_ptr = next_kbd_event (event);
        }

        /* C 4348-4529: C's `default:` arm.  */
        _ => {
	  let mut frame_: Lisp_Object;
	  let mut focus: Lisp_Object;

	  /* It's not safe to assume that the following will always
	     produce a valid, live frame (Bug#78966).  */
	  frame_ = (*event).ie.frame_or_window;
	  if CONSP (frame_)
	    { frame_ = XCAR (frame_); }
	  else if WINDOWP (frame_)
	    { frame_ = WINDOW_FRAME! (XWINDOW (frame_)); }

	  /* If the input focus of this frame is on another frame,
	     continue with that frame.  */
	  if FRAMEP (frame_)
	    {
	      focus = FRAME_FOCUS_FRAME (XFRAME (frame_));
	      if ! NILP (focus)
		{ frame_ = focus; }
	    }

	  /* If this event is on a different frame, return a
	     switch-frame this time, and leave the event in the queue
	     for next time.  */
	  if !EQ (frame_, internal_last_event_frame)
	      && !EQ (frame_, selected_frame)
	    { obj = make_lispy_switch_frame (frame_); }
	  internal_last_event_frame = frame_;

	  if EQ ((*event).ie.device, Qt ())
	    { Vlast_event_device
		= if input_event_bits! (kind, &raw mut (*event).ie) == ASCII_KEYSTROKE_EVENT
		     || input_event_bits! (kind, &raw mut (*event).ie)
		        == MULTIBYTE_CHAR_KEYSTROKE_EVENT
		     || input_event_bits! (kind, &raw mut (*event).ie)
		        == NON_ASCII_KEYSTROKE_EVENT
		   { virtual_core_keyboard_name }
		   else { virtual_core_pointer_name }; }
	  else
	    { Vlast_event_device = (*event).ie.device; }

	  /* If we didn't decide to make a switch-frame event, go ahead
	     and build a real event from the queue entry.  */
	  if NILP (obj)
	    {
	      let mut pinch_dx: c_double = 0.0;
	      let mut pinch_dy: c_double = 0.0;
	      let mut pinch_angle: c_double = 0.0;

	      /* Pinch events are often sent in rapid succession, so
		 large amounts of such events have the potential to
		 queue up inside the keyboard buffer.  In that case,
		 find the last pinch event in succession on the same
		 frame with the same modifiers, and send that instead.  */

	      if input_event_bits! (kind, &raw mut (*event).ie) == PINCH_EVENT
		  /* Ignore if this is the start of a pinch sequence.
		     These events should always be sent so that we
		     never miss a sequence starting, and they don't
		     have the potential to queue up.  */
		  && { pinch_dx = XFLOAT_DATA (XCAR ((*event).ie.arg)); pinch_dx != 0.0 }
		     || XFLOAT_DATA (XCAR (XCDR ((*event).ie.arg))) != 0.0
		     || XFLOAT_DATA (Fnth (make_fixnum (3), (*event).ie.arg)) != 0.0
		{
		  let mut maybe_event: *mut buffered_input_event = next_kbd_event (event);

		  pinch_dy = XFLOAT_DATA (XCAR (XCDR ((*event).ie.arg)));
		  pinch_angle = XFLOAT_DATA (Fnth (make_fixnum (3), (*event).ie.arg));

		  while maybe_event != kbd_store_ptr
			 && input_event_bits! (kind, &raw mut (*maybe_event).ie) == PINCH_EVENT
			 /* Make sure we never miss an event that has
			    different modifiers.  */
			 && (*maybe_event).ie.modifiers == (*event).ie.modifiers
			 /* Make sure that the event is for the same
			    frame.  */
			 && EQ ((*maybe_event).ie.frame_or_window,
				(*event).ie.frame_or_window)
			 /* Make sure that the event isn't the start
			    of a new pinch gesture sequence.  */
			 && (XFLOAT_DATA (XCAR ((*maybe_event).ie.arg)) != 0.0
			     || XFLOAT_DATA (XCAR (XCDR ((*maybe_event).ie.arg))) != 0.0
			     || XFLOAT_DATA (Fnth (make_fixnum (3),
						   (*maybe_event).ie.arg)) != 0.0)
		    {
		      event = maybe_event;
		      /* Add up relative deltas inside events we skip.  */
		      pinch_dx += XFLOAT_DATA (XCAR ((*maybe_event).ie.arg));
		      pinch_dy += XFLOAT_DATA (XCAR (XCDR ((*maybe_event).ie.arg)));
		      pinch_angle += XFLOAT_DATA (Fnth (make_fixnum (3),
							(*maybe_event).ie.arg));

		      XSETCAR ((*maybe_event).ie.arg, make_float (pinch_dx));
		      XSETCAR (XCDR ((*maybe_event).ie.arg), make_float (pinch_dy));
		      XSETCAR (Fnthcdr (make_fixnum (3),
					(*maybe_event).ie.arg),
			       make_float (fmod (pinch_angle, 360.0)));

		      if !EQ ((*maybe_event).ie.device, Qt ())
			{ Vlast_event_device = (*maybe_event).ie.device; }

		      maybe_event = next_kbd_event (event);
		    }
		}

	      if (*event).kind == MULTIBYTE_CHAR_KEYSTROKE_EVENT as u16
		  /* This string has to be decoded.  */
		  && STRINGP ((*event).ie.arg)
		{
		  str = internal_condition_case_1 (kbd_buffer_get_event_1,
						   (*event).ie.arg, Qt (),
						   kbd_buffer_get_event_2);

		  /* Decoding the string failed, so use the original,
		     where at least ASCII text will work.  */
		  if NILP (str)
		    { str = (*event).ie.arg; }

		  if SCHARS (str) == 0
		    {
		      kbd_fetch_ptr = next_kbd_event (event);
		      obj = Qnil ();
		      break 'switch;
		    }

		  /* car is the index of the next character in the
		     string that will be sent and cdr is the string
		     itself.  */
		  (*event).ie.arg = Fcons (make_fixnum (0), str);
		}

	      if (*event).kind == MULTIBYTE_CHAR_KEYSTROKE_EVENT as u16
		  && CONSP ((*event).ie.arg)
		{
		  eassert! (FIXNUMP (XCAR ((*event).ie.arg)));
		  eassert! (STRINGP (XCDR ((*event).ie.arg)));
		  eassert! (XFIXNUM (XCAR ((*event).ie.arg))
			    < SCHARS (XCDR ((*event).ie.arg)));

		  (*event).ie.code = XFIXNUM (Faref (XCDR ((*event).ie.arg),
						     XCAR ((*event).ie.arg))) as c_uint;

		  XSETCAR ((*event).ie.arg,
			   make_fixnum (XFIXNUM (XCAR ((*event).ie.arg)) + 1));
		}

	      obj = make_lispy_event (&raw mut (*event).ie);

	      /* C 4474-4490: `#ifdef HAVE_EXT_MENU_BAR` (carried per P-51).  */
	      #[cfg (feature = "have_ext_menu_bar")]
	      {
	      /* If this was a menu selection, then set the flag to inhibit
		 writing to last_nonmenu_event.  Don't do this if the event
		 we're returning is (menu-bar), though; that indicates the
		 beginning of the menu sequence, and we might as well leave
		 that as the `event with parameters' for this selection.  */
	      if !used_mouse_menu.is_null ()
		  && !EQ ((*event).ie.frame_or_window, (*event).ie.arg)
		  && (input_event_bits! (kind, &raw mut (*event).ie) == MENU_BAR_EVENT
		      || input_event_bits! (kind, &raw mut (*event).ie) == TAB_BAR_EVENT
		      || input_event_bits! (kind, &raw mut (*event).ie) == TOOL_BAR_EVENT)
		{ *used_mouse_menu = true; }
	      }
	      /* C 4491-4496: `#ifdef HAVE_NS` (carried per P-51).  */
	      #[cfg (feature = "have_ns")]
	      {
	      /* Certain system events are non-key events.  */
	      if !used_mouse_menu.is_null ()
                  && input_event_bits! (kind, &raw mut (*event).ie) == NS_NONKEY_EVENT
		{ *used_mouse_menu = true; }
	      }

	      if input_event_bits! (kind, &raw mut (*event).ie)
		   != MULTIBYTE_CHAR_KEYSTROKE_EVENT
		  || !CONSP ((*event).ie.arg)
		  || (XFIXNUM (XCAR ((*event).ie.arg))
		      >= SCHARS (XCDR ((*event).ie.arg)))
		{
		  /* Wipe out this event, to catch bugs.  */
		  clear_event (&raw mut (*event).ie);
		  kbd_fetch_ptr = next_kbd_event (event);
		}
	    }
	}
	}
      } /* 'switch */

    }
  /* Try generating a mouse motion event.  */
  else if !some_mouse_moved ().is_null ()
    {
      let mut f: *mut frame;
      let mut bar_window: Lisp_Object = Qnil ();
      let mut part: scroll_bar_part = 0;
      let mut x: Lisp_Object = Qnil ();
      let mut y: Lisp_Object = Qnil ();
      let mut t: Time = 0;

      *event_frame = some_mouse_moved ();
      f = *event_frame;
      *kbp = current_kboard;
      /* Note that this uses F to determine which terminal to look at.
	 If there is no valid info, it does not store anything
	 so x remains nil.  */
      x = Qnil ();

      /* XXX Can f or mouse_position_hook be NULL here?  */
      if !f.is_null () && (*FRAME_TERMINAL! (f)).mouse_position_hook.is_some ()
        { ((*FRAME_TERMINAL! (f)).mouse_position_hook.unwrap ()) (&raw mut f as *mut *mut c_void, 0, &raw mut bar_window,
								  &raw mut part, &raw mut x, &raw mut y, &raw mut t); }

      obj = Qnil ();

      /* Decide if we should generate a switch-frame event.  Don't
	 generate switch-frame events for motion outside of all Emacs
	 frames.  */
      if !NILP (x) && !f.is_null ()
	{
	  let mut frame_: Lisp_Object;

	  frame_ = FRAME_FOCUS_FRAME (f);
	  if NILP (frame_)
	    { XSETFRAME! (frame_, f); }

	  if !EQ (frame_, internal_last_event_frame)
	      && !EQ (frame_, selected_frame)
	    { obj = make_lispy_switch_frame (frame_); }
	  internal_last_event_frame = frame_;
	}

      /* If we didn't decide to make a switch-frame event, go ahead and
	 return a mouse-motion event.  */
      if !NILP (x) && NILP (obj)
	{ obj = make_lispy_movement (f, bar_window, part, x, y, t); }

      if !NILP (obj)
	{ Vlast_event_device = if STRINGP ((**event_frame).last_mouse_device)
				 { (**event_frame).last_mouse_device }
			       else { virtual_core_pointer_name }; }
    }
  else
    {
      /* C 4575-4579: `#ifdef HAVE_X_WINDOWS`'s `else if` arm (carried per P-51; Rust has no
	 `#[cfg] else`, so both arms of the chain's tail stand here).  */
      #[cfg (feature = "have_x_windows")]
      if had_pending_selection_requests
	{ obj = Qnil (); }
      #[cfg (not (feature = "have_x_windows"))]
      /* We were promised by the above while loop that there was
	 something for us to read!  */
      { emacs_abort (); }
    }

  input_pending = readable_events (0);

  Vlast_event_frame = internal_last_event_frame;

  return (obj);
}

/* C 4594-4675: `process_special_events`.  */
unsafe fn process_special_events ()
{
  let mut event: *mut buffered_input_event;

  event = kbd_fetch_ptr;
  while event != kbd_store_ptr
    {
      /* If we find a stored X selection request, handle it now.  */
      if (*event).kind == SELECTION_REQUEST_EVENT as u16
	  || (*event).kind == SELECTION_CLEAR_EVENT as u16
	{
	  /* C 4597-4604: the enclosing `#if defined HAVE_X11 || defined HAVE_PGTK ||
	     defined HAVE_HAIKU` and its `#ifndef HAVE_HAIKU` / `#else` pair, then C 4605's
	     `#if defined HAVE_X11 || defined HAVE_PGTK` with C 4638-4644's `#ifdef HAVE_X11`
	     under it (all of those macros are `#undef` on this target; carried per P-51).  C
	     declares `copy` and `moved_events` at the function's top and only one arm of the
	     chain can ever be live, so the port declares them in the arm that uses them (the
	     `kbd_buffer_get_event` shape).  */
	  #[cfg (feature = "have_x11_or_have_pgtk")]
	  {
	  let mut copy: selection_input_event;
	  let mut moved_events: c_int;

	  /* Remove the event from the fifo buffer before processing;
	     otherwise swallow_events called recursively could see it
	     and process it again.  To do this, we move the events
	     between kbd_fetch_ptr and EVENT one slot to the right,
	     cyclically.  */

	  copy = (*event).sie;

	  if event < kbd_fetch_ptr
	    {
	      memmove ((&raw mut kbd_buffer as *mut buffered_input_event).add (1) as *mut c_void,
		       &raw mut kbd_buffer as *const c_void,
		       ((event.offset_from (&raw mut kbd_buffer as *mut buffered_input_event))
			* std::mem::size_of::<buffered_input_event> () as isize) as usize);
	      kbd_buffer[0] = kbd_buffer[(KBD_BUFFER_SIZE - 1) as usize];
	      moved_events = ((&raw mut kbd_buffer as *mut buffered_input_event)
			      .add ((KBD_BUFFER_SIZE - 1) as usize)
			      .offset_from (kbd_fetch_ptr)) as c_int;
	    }
	  else
	    { moved_events = event.offset_from (kbd_fetch_ptr) as c_int; }

	  memmove ((kbd_fetch_ptr).add (1) as *mut c_void, kbd_fetch_ptr as *const c_void,
		   ((moved_events as isize)
		    * std::mem::size_of::<buffered_input_event> () as isize) as usize);
	  kbd_fetch_ptr = next_kbd_event (kbd_fetch_ptr);
	  input_pending = readable_events (0);

	  #[cfg (feature = "have_x11")]
	  { x_handle_selection_event (&raw mut copy); }
	  #[cfg (not (feature = "have_x11"))]
	  { pgtk_handle_selection_event (&raw mut copy); }
	  }
	  /* C 4645-4665: the `#elif defined HAVE_HAIKU` arm (carried per P-51).  */
	  #[cfg (feature = "have_haiku")]
	  {
	  let mut copy: input_event;
	  let mut moved_events: c_int;

	  if (*event).ie.kind != SELECTION_CLEAR_EVENT as u16
	    { emacs_abort (); }

	  copy = (*event).ie;

	  if event < kbd_fetch_ptr
	    {
	      memmove ((&raw mut kbd_buffer as *mut buffered_input_event).add (1) as *mut c_void,
		       &raw mut kbd_buffer as *const c_void,
		       ((event.offset_from (&raw mut kbd_buffer as *mut buffered_input_event))
			* std::mem::size_of::<buffered_input_event> () as isize) as usize);
	      kbd_buffer[0] = kbd_buffer[(KBD_BUFFER_SIZE - 1) as usize];
	      moved_events = ((&raw mut kbd_buffer as *mut buffered_input_event)
			      .add ((KBD_BUFFER_SIZE - 1) as usize)
			      .offset_from (kbd_fetch_ptr)) as c_int;
	    }
	  else
	    { moved_events = event.offset_from (kbd_fetch_ptr) as c_int; }

	  memmove ((kbd_fetch_ptr).add (1) as *mut c_void, kbd_fetch_ptr as *const c_void,
		   ((moved_events as isize)
		    * std::mem::size_of::<buffered_input_event> () as isize) as usize);
	  kbd_fetch_ptr = next_kbd_event (kbd_fetch_ptr);
	  input_pending = readable_events (0);
	  haiku_handle_selection_clear (&raw mut copy);
	  }
	  /* C 4666-4673: the chain's `#else` arm — the live one on this target.  */
	  #[cfg (not (feature = "have_x11_or_have_pgtk_or_have_haiku"))]
	  {
	  /* We're getting selection request events, but we don't have
             a window system.  */
	  emacs_abort ();
	  }
	}
      event = next_kbd_event (event);
    }
}

/* Process any events that are not user-visible, run timer events that
   are ripe, and return, without reading any user-visible events.  */

/* C 4678-4689: `swallow_events`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn swallow_events (do_display: bool)
{
  let old_timers_run: c_uint;

  process_special_events ();

  old_timers_run = timers_run;
  get_input_pending (READABLE_EVENTS_DO_TIMERS_NOW);

  if !input_pending && timers_run != old_timers_run && do_display
    { redisplay_preserve_echo_area (7); }
}

/* Record the start of when Emacs is idle,
   for the sake of running idle-time timers.  */

/* C 4695-4707: `timer_start_idle`.  */
unsafe fn timer_start_idle ()
{
  /* If we are already in the idle state, do nothing.  */
  if timespec_valid_p (timer_idleness_start_time)
    { return; }

  timer_idleness_start_time = current_timespec ();
  timer_last_idleness_start_time = timer_idleness_start_time;

  /* Mark all idle-time timers as once again candidates for running.  */
  call0 (Qinternal_timer_start_idle ());
}

/* Record that Emacs is no longer idle, so stop running idle-time timers.  */

/* C 4711-4714: `timer_stop_idle`.  */
unsafe fn timer_stop_idle ()
{
  timer_idleness_start_time = invalid_timespec ();
}

/* Resume idle timer from last idle start time.  */

/* C 4719-4726: `timer_resume_idle`.  */
unsafe fn timer_resume_idle ()
{
  if timespec_valid_p (timer_idleness_start_time)
    { return; }

  timer_idleness_start_time = timer_last_idleness_start_time;
}

/* List of elisp functions to call, delayed because they were generated in
   a context where Elisp could not be safely run (e.g. redisplay, signal,
   ...).  Each element has the form (FUN . ARGS).  */
/* C 4740: `Lisp_Object pending_funcalls;` (lisp.rs 3895 declares it in `other_units`, which is
   a glob import: the crate-root item here shadows it, PORTING.md P-21).  */
#[unsafe(no_mangle)]
pub static mut pending_funcalls: Lisp_Object = lisp_h_Qnil ();

/* Return the value of TIMER if it is a valid timer, an invalid struct
   timespec otherwise.  */
/* C 4737-4752: `decode_timer`.  */
unsafe fn decode_timer (timer: Lisp_Object) -> timespec
{
  let vec: *mut Lisp_Object;

  if ! (VECTORP (timer) && ASIZE (timer) == 10)
    { return invalid_timespec (); }
  vec = (*XVECTOR (timer)).contents.as_mut_ptr ();
  if ! NILP (*vec.offset (0))
    { return invalid_timespec (); }
  if ! FIXNUMP (*vec.offset (2))
    { return invalid_timespec (); }
  return list4_to_timespec (*vec.offset (1), *vec.offset (2), *vec.offset (3),
			    *vec.offset (8));
}

/* Check whether a timer has fired.  To prevent larger problems we simply
   disregard elements that are not proper timers.  Do not make a circular
   timer list for the time being.

   Returns the time to wait until the next timer fires.  If a
   timer is triggering now, return zero.
   If no timer is active, return -1.

   If a timer is ripe, we run it, with quitting turned off.
   In that case we return 0 to indicate that a new timer_check_2 call
   should be done.  */

/* C 4763-4905: `timer_check_2`.  */
unsafe fn timer_check_2 (timers: Lisp_Object, idle_timers: Lisp_Object) -> timespec
{
  /* C's `timers` and `idle_timers` are the arguments themselves and both are reassigned; the
     port shadows them (PORTING.md: parameters are immutable).  */
  let mut timers = timers;
  let mut idle_timers = idle_timers;

  /* First run the code that was delayed.  */
  while CONSP (pending_funcalls)
    {
      let funcall: Lisp_Object = XCAR (pending_funcalls);
      pending_funcalls = XCDR (pending_funcalls);
      safe_calln! (Qapply (), XCAR (funcall), XCDR (funcall));
    }

  if ! (CONSP (timers) || CONSP (idle_timers))
    { return invalid_timespec (); }

  let now: timespec = current_timespec ();
  let idleness_now: timespec
    = if timespec_valid_p (timer_idleness_start_time)
      { timespec_sub (now, timer_idleness_start_time) }
      else { make_timespec (0, 0) };

  loop
    {
      /* C's two `continue`s jump to the `do … while (CONSP (timers) ||
	 CONSP (idle_timers))` test at the body's end; the labelled block is that edge and the
	 `loop`'s break test below is the condition (PORTING.md 2.2's loop shape).  */
      'timers_and_idle_timers: {
      let mut chosen_timer: Lisp_Object;
      let mut timer: Lisp_Object = Qnil ();
      let mut idle_timer: Lisp_Object = Qnil ();
      let mut difference: timespec;
      let mut timer_difference: timespec = invalid_timespec ();
      let mut idle_timer_difference: timespec = invalid_timespec ();
      let mut ripe: bool;
      let mut timer_ripe: bool = false;
      let mut idle_timer_ripe: bool = false;

      /* Set TIMER and TIMER_DIFFERENCE
	 based on the next ordinary timer.
	 TIMER_DIFFERENCE is the distance in time from NOW to when
	 this timer becomes ripe.
         Skip past invalid timers and timers already handled.  */
      if CONSP (timers)
	{
	  timer = XCAR (timers);
	  let timer_time: timespec = decode_timer (timer);
	  if ! timespec_valid_p (timer_time)
	    {
	      timers = XCDR (timers);
	      break 'timers_and_idle_timers;
	    }

	  timer_ripe = timespec_cmp (timer_time, now) <= 0;
	  timer_difference = if timer_ripe
			       { timespec_sub (now, timer_time) }
			     else { timespec_sub (timer_time, now) };
	}

      /* Likewise for IDLE_TIMER and IDLE_TIMER_DIFFERENCE
	 based on the next idle timer.  */
      if CONSP (idle_timers)
	{
	  idle_timer = XCAR (idle_timers);
	  let idle_timer_time: timespec = decode_timer (idle_timer);
	  if ! timespec_valid_p (idle_timer_time)
	    {
	      idle_timers = XCDR (idle_timers);
	      break 'timers_and_idle_timers;
	    }

	  idle_timer_ripe = timespec_cmp (idle_timer_time, idleness_now) <= 0;
	  idle_timer_difference
	    = if idle_timer_ripe
	      { timespec_sub (idleness_now, idle_timer_time) }
	      else { timespec_sub (idle_timer_time, idleness_now) };
	}

      /* Decide which timer is the next timer,
	 and set CHOSEN_TIMER, DIFFERENCE, and RIPE accordingly.
	 Also step down the list where we found that timer.  */

      if timespec_valid_p (timer_difference)
	  && (! timespec_valid_p (idle_timer_difference)
	      || idle_timer_ripe < timer_ripe
	      || (idle_timer_ripe == timer_ripe
		  && ((if timer_ripe
		       { timespec_cmp (idle_timer_difference, timer_difference) }
		       else { timespec_cmp (timer_difference, idle_timer_difference) })
		      < 0)))
	{
	  chosen_timer = timer;
	  timers = XCDR (timers);
	  difference = timer_difference;
	  ripe = timer_ripe;
	}
      else
	{
	  chosen_timer = idle_timer;
	  idle_timers = XCDR (idle_timers);
	  difference = idle_timer_difference;
	  ripe = idle_timer_ripe;
	}

      /* If timer is ripe, run it if it hasn't been run.  */
      if ripe
	{
	  /* If we got here, presumably `decode_timer` has checked
             that this timer has not yet been triggered.  */
	  eassert! (NILP (AREF (chosen_timer, 0)));
	  /* In a production build, where assertions compile to
	     nothing, we still want to play it safe here.  */
	  if NILP (AREF (chosen_timer, 0))
	    {
	      let count: specpdl_ref = SPECPDL_INDEX ();
	      let old_deactivate_mark: Lisp_Object = Vdeactivate_mark;

	      /* Mark the timer as triggered to prevent problems if the lisp
		 code fails to reschedule it right.  */
	      ASET (chosen_timer, 0, Qt ());

	      specbind (Qinhibit_quit (), Qt ());

	      calln! (Qtimer_event_handler (), chosen_timer);
	      Vdeactivate_mark = old_deactivate_mark;
	      timers_run += 1;
	      unbind_to (count, Qnil ());

	      /* Since we have handled the event,
		 we don't need to tell the caller to wake up and do it.  */
	      /* But the caller must still wait for the next timer, so
		 return 0 to indicate that.  */
	    }

	  return make_timespec (0, 0);
	}
      else
	/* When we encounter a timer that is still waiting,
	   return the amount of time to wait before it is ripe.  */
	{
	  return difference;
	}
      }
      if !(CONSP (timers) || CONSP (idle_timers)) { break; }
    }

  /* No timers are pending in the future.  */
  return invalid_timespec ();
}

/* Check whether a timer has fired.  To prevent larger problems we simply
   disregard elements that are not proper timers.  Do not make a circular
   timer list for the time being.

   Returns the time to wait until the next timer fires.
   If no timer is active, return an invalid value.

   As long as any timer is ripe, we run it.  */

/* C 4914-4945: `timer_check`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timer_check () -> timespec
{
  let mut nexttime: timespec;
  let mut timers: Lisp_Object;
  let mut idle_timers: Lisp_Object;

  let tem: Lisp_Object = Vinhibit_quit;
  Vinhibit_quit = Qt ();
  block_input ();
  turn_on_atimers (false);

  /* We use copies of the timers' lists to allow a timer to add itself
     again, without locking up Emacs if the newly added timer is
     already ripe when added.  */

  /* Always consider the ordinary timers.  */
  timers = Fcopy_sequence (Vtimer_list);
  /* Consider the idle timers only if Emacs is idle.  */
  if timespec_valid_p (timer_idleness_start_time)
    { idle_timers = Fcopy_sequence (Vtimer_idle_list); }
  else
    { idle_timers = Qnil (); }

  turn_on_atimers (true);
  unblock_input ();
  Vinhibit_quit = tem;

  loop
    {
      nexttime = timer_check_2 (timers, idle_timers);
      if !(nexttime.tv_sec == 0 && nexttime.tv_nsec == 0) { break; }
    }

  return nexttime;
}

DEFUN! ("current-idle-time", Fcurrent_idle_time, Scurrent_idle_time, 0, 0, 0,
       doc: "Return the current length of Emacs idleness, or nil.\nThe value when Emacs is idle is a Lisp timestamp in the style of\n`current-time'.\n\nThe value when Emacs is not idle is nil.\n\nIf the value is a list of four integers (HIGH LOW USEC PSEC), then PSEC\nis a multiple of the system clock resolution.",
       (),
       {
  if timespec_valid_p (timer_idleness_start_time)
    { return make_lisp_time (timespec_sub (current_timespec (),
					   timer_idleness_start_time)); }

  return Qnil ();
});






/* C 4968-4973: the six caches (the `static Lisp_Object`s C calls "Caches for
   modify_event_symbol"), and C 4966-4967's comment above.  */

/* Caches for modify_event_symbol.  */
static mut accent_key_syms: Lisp_Object = lisp_h_Qnil ();
static mut func_key_syms: Lisp_Object = lisp_h_Qnil ();
static mut mouse_syms: Lisp_Object = lisp_h_Qnil ();
static mut wheel_syms: Lisp_Object = lisp_h_Qnil ();
static mut drag_n_drop_syms: Lisp_Object = lisp_h_Qnil ();
static mut pinch_syms: Lisp_Object = lisp_h_Qnil ();

/* This is a list of keysym codes for special "accent" characters.
   It parallels lispy_accent_keys.  */

/* C 4978-5080: `lispy_accent_codes`.  Each element is an `#ifdef XK_<name>` /
   `#else` / `#endif` pair; the X11 keysym macros are not in the frozen configuration
   (the frozen table decides every one of them), so both arms are carried per P-51.  */
static lispy_accent_codes: [c_int; 20] = [
  #[cfg (feature = "xk_dead_circumflex")]
  XK_dead_circumflex,
  #[cfg (not (feature = "xk_dead_circumflex"))]
  0,
  #[cfg (feature = "xk_dead_grave")]
  XK_dead_grave,
  #[cfg (not (feature = "xk_dead_grave"))]
  0,
  #[cfg (feature = "xk_dead_tilde")]
  XK_dead_tilde,
  #[cfg (not (feature = "xk_dead_tilde"))]
  0,
  #[cfg (feature = "xk_dead_diaeresis")]
  XK_dead_diaeresis,
  #[cfg (not (feature = "xk_dead_diaeresis"))]
  0,
  #[cfg (feature = "xk_dead_macron")]
  XK_dead_macron,
  #[cfg (not (feature = "xk_dead_macron"))]
  0,
  #[cfg (feature = "xk_dead_degree")]
  XK_dead_degree,
  #[cfg (not (feature = "xk_dead_degree"))]
  0,
  #[cfg (feature = "xk_dead_acute")]
  XK_dead_acute,
  #[cfg (not (feature = "xk_dead_acute"))]
  0,
  #[cfg (feature = "xk_dead_cedilla")]
  XK_dead_cedilla,
  #[cfg (not (feature = "xk_dead_cedilla"))]
  0,
  #[cfg (feature = "xk_dead_breve")]
  XK_dead_breve,
  #[cfg (not (feature = "xk_dead_breve"))]
  0,
  #[cfg (feature = "xk_dead_ogonek")]
  XK_dead_ogonek,
  #[cfg (not (feature = "xk_dead_ogonek"))]
  0,
  #[cfg (feature = "xk_dead_caron")]
  XK_dead_caron,
  #[cfg (not (feature = "xk_dead_caron"))]
  0,
  #[cfg (feature = "xk_dead_doubleacute")]
  XK_dead_doubleacute,
  #[cfg (not (feature = "xk_dead_doubleacute"))]
  0,
  #[cfg (feature = "xk_dead_abovedot")]
  XK_dead_abovedot,
  #[cfg (not (feature = "xk_dead_abovedot"))]
  0,
  #[cfg (feature = "xk_dead_abovering")]
  XK_dead_abovering,
  #[cfg (not (feature = "xk_dead_abovering"))]
  0,
  #[cfg (feature = "xk_dead_iota")]
  XK_dead_iota,
  #[cfg (not (feature = "xk_dead_iota"))]
  0,
  #[cfg (feature = "xk_dead_belowdot")]
  XK_dead_belowdot,
  #[cfg (not (feature = "xk_dead_belowdot"))]
  0,
  #[cfg (feature = "xk_dead_voiced_sound")]
  XK_dead_voiced_sound,
  #[cfg (not (feature = "xk_dead_voiced_sound"))]
  0,
  #[cfg (feature = "xk_dead_semivoiced_sound")]
  XK_dead_semivoiced_sound,
  #[cfg (not (feature = "xk_dead_semivoiced_sound"))]
  0,
  #[cfg (feature = "xk_dead_hook")]
  XK_dead_hook,
  #[cfg (not (feature = "xk_dead_hook"))]
  0,
  #[cfg (feature = "xk_dead_horn")]
  XK_dead_horn,
  #[cfg (not (feature = "xk_dead_horn"))]
  0,
];

/* This is a list of Lisp names for special "accent" characters.
   It parallels lispy_accent_codes.  */

/* C 5085-5107: `lispy_accent_keys`.  */
static mut lispy_accent_keys: [*const c_char; 20] = [
  c"dead-circumflex".as_ptr (),
  c"dead-grave".as_ptr (),
  c"dead-tilde".as_ptr (),
  c"dead-diaeresis".as_ptr (),
  c"dead-macron".as_ptr (),
  c"dead-degree".as_ptr (),
  c"dead-acute".as_ptr (),
  c"dead-cedilla".as_ptr (),
  c"dead-breve".as_ptr (),
  c"dead-ogonek".as_ptr (),
  c"dead-caron".as_ptr (),
  c"dead-doubleacute".as_ptr (),
  c"dead-abovedot".as_ptr (),
  c"dead-abovering".as_ptr (),
  c"dead-iota".as_ptr (),
  c"dead-belowdot".as_ptr (),
  c"dead-voiced-sound".as_ptr (),
  c"dead-semivoiced-sound".as_ptr (),
  c"dead-hook".as_ptr (),
  c"dead-horn".as_ptr (),
];


/* C 5109-5207: `#ifdef HAVE_ANDROID` (carried per P-51).  C 5110's `#define
   FUNCTION_KEY_OFFSET 0` is one of the three definitions of that macro; the single
   definition the port keeps is the frozen configuration's (below).  */
#[cfg (feature = "have_android")]
static mut lispy_function_keys: [*const c_char; 338] = {
  let mut lispy_function_keys: [*const c_char; 338] = [std::ptr::null (); 338];
  /* All elements in this array default to 0, except for the few
  function keys that Emacs recognizes.  */
  lispy_function_keys[111] = c"escape".as_ptr ();
  lispy_function_keys[112] = c"delete".as_ptr ();
  lispy_function_keys[116] = c"scroll".as_ptr ();
  lispy_function_keys[120] = c"sysrq".as_ptr ();
  lispy_function_keys[121] = c"break".as_ptr ();
  lispy_function_keys[122] = c"home".as_ptr ();
  lispy_function_keys[123] = c"end".as_ptr ();
  lispy_function_keys[124] = c"insert".as_ptr ();
  lispy_function_keys[126] = c"media-play".as_ptr ();
  lispy_function_keys[127] = c"media-pause".as_ptr ();
  lispy_function_keys[130] = c"media-record".as_ptr ();
  lispy_function_keys[131] = c"f1".as_ptr ();
  lispy_function_keys[132] = c"f2".as_ptr ();
  lispy_function_keys[133] = c"f3".as_ptr ();
  lispy_function_keys[134] = c"f4".as_ptr ();
  lispy_function_keys[135] = c"f5".as_ptr ();
  lispy_function_keys[136] = c"f6".as_ptr ();
  lispy_function_keys[137] = c"f7".as_ptr ();
  lispy_function_keys[138] = c"f8".as_ptr ();
  lispy_function_keys[139] = c"f9".as_ptr ();
  lispy_function_keys[140] = c"f10".as_ptr ();
  lispy_function_keys[141] = c"f11".as_ptr ();
  lispy_function_keys[142] = c"f12".as_ptr ();
  lispy_function_keys[143] = c"kp-numlock".as_ptr ();
  lispy_function_keys[160] = c"kp-ret".as_ptr ();
  lispy_function_keys[164] = c"volume-mute".as_ptr ();
  lispy_function_keys[165] = c"info".as_ptr ();
  lispy_function_keys[19] = c"up".as_ptr ();
  lispy_function_keys[20] = c"down".as_ptr ();
  lispy_function_keys[211] = c"zenkaku-hankaku".as_ptr ();
  lispy_function_keys[213] = c"muhenkan".as_ptr ();
  lispy_function_keys[214] = c"henkan".as_ptr ();
  lispy_function_keys[215] = c"hiragana-katakana".as_ptr ();
  lispy_function_keys[218] = c"kana".as_ptr ();
  lispy_function_keys[21] = c"left".as_ptr ();
  lispy_function_keys[223] = c"sleep".as_ptr ();
  lispy_function_keys[22] = c"right".as_ptr ();
  lispy_function_keys[23] = c"select".as_ptr ();
  lispy_function_keys[24] = c"volume-up".as_ptr ();
  lispy_function_keys[259] = c"help".as_ptr ();
  lispy_function_keys[25] = c"volume-down".as_ptr ();
  lispy_function_keys[268] = c"kp-up-left".as_ptr ();
  lispy_function_keys[269] = c"kp-down-left".as_ptr ();
  lispy_function_keys[26] = c"power".as_ptr ();
  lispy_function_keys[270] = c"kp-up-right".as_ptr ();
  lispy_function_keys[271] = c"kp-down-right".as_ptr ();
  lispy_function_keys[272] = c"media-skip-forward".as_ptr ();
  lispy_function_keys[273] = c"media-skip-backward".as_ptr ();
  lispy_function_keys[277] = c"cut".as_ptr ();
  lispy_function_keys[278] = c"copy".as_ptr ();
  lispy_function_keys[279] = c"paste".as_ptr ();
  lispy_function_keys[285] = c"browser-refresh".as_ptr ();
  lispy_function_keys[28] = c"clear".as_ptr ();
  lispy_function_keys[300] = c"XF86Forward".as_ptr ();
  lispy_function_keys[319] = c"dictate".as_ptr ();
  lispy_function_keys[320] = c"new".as_ptr ();
  lispy_function_keys[321] = c"close".as_ptr ();
  lispy_function_keys[322] = c"do-not-disturb".as_ptr ();
  lispy_function_keys[323] = c"print".as_ptr ();
  lispy_function_keys[324] = c"lock".as_ptr ();
  lispy_function_keys[325] = c"fullscreen".as_ptr ();
  lispy_function_keys[326] = c"f13".as_ptr ();
  lispy_function_keys[327] = c"f14".as_ptr ();
  lispy_function_keys[328] = c"f15".as_ptr ();
  lispy_function_keys[329] = c"f16".as_ptr ();
  lispy_function_keys[330] = c"f17".as_ptr ();
  lispy_function_keys[331] = c"f18".as_ptr ();
  lispy_function_keys[332] = c"f19".as_ptr ();
  lispy_function_keys[333] = c"f20".as_ptr ();
  lispy_function_keys[334] = c"f21".as_ptr ();
  lispy_function_keys[335] = c"f22".as_ptr ();
  lispy_function_keys[336] = c"f23".as_ptr ();
  lispy_function_keys[337] = c"f24".as_ptr ();
  lispy_function_keys[4] = c"XF86Back".as_ptr ();
  lispy_function_keys[61] = c"tab".as_ptr ();
  lispy_function_keys[66] = c"return".as_ptr ();
  lispy_function_keys[67] = c"backspace".as_ptr ();
  lispy_function_keys[82] = c"menu".as_ptr ();
  lispy_function_keys[84] = c"find".as_ptr ();
  lispy_function_keys[85] = c"media-play-pause".as_ptr ();
  lispy_function_keys[86] = c"media-stop".as_ptr ();
  lispy_function_keys[87] = c"media-next".as_ptr ();
  lispy_function_keys[88] = c"media-previous".as_ptr ();
  lispy_function_keys[89] = c"media-rewind".as_ptr ();
  lispy_function_keys[92] = c"prior".as_ptr ();
  lispy_function_keys[93] = c"next".as_ptr ();
  lispy_function_keys[95] = c"mode-change".as_ptr ();
  lispy_function_keys
};

/* C 5209-5461: `#elif defined HAVE_NTGUI` (carried per P-51); C 5210's `#define
   FUNCTION_KEY_OFFSET 0x0` is that macro's second definition.  */
#[cfg (feature = "have_ntgui")]
static mut lispy_function_keys: [*const c_char; 256] = [
    std::ptr::null (),                /* 0                      */

    std::ptr::null (),                /* VK_LBUTTON        0x01 */
    std::ptr::null (),                /* VK_RBUTTON        0x02 */
    c"cancel".as_ptr (),         /* VK_CANCEL         0x03 */
    std::ptr::null (),                /* VK_MBUTTON        0x04 */

    std::ptr::null (), std::ptr::null (), std::ptr::null (),          /*    0x05 .. 0x07        */

    c"backspace".as_ptr (),      /* VK_BACK           0x08 */
    c"tab".as_ptr (),            /* VK_TAB            0x09 */

    std::ptr::null (), std::ptr::null (),             /*    0x0A .. 0x0B        */

    c"clear".as_ptr (),          /* VK_CLEAR          0x0C */
    c"return".as_ptr (),         /* VK_RETURN         0x0D */

    std::ptr::null (), std::ptr::null (),             /*    0x0E .. 0x0F        */

    std::ptr::null (),                /* VK_SHIFT          0x10 */
    std::ptr::null (),                /* VK_CONTROL        0x11 */
    std::ptr::null (),                /* VK_MENU           0x12 */
    c"pause".as_ptr (),          /* VK_PAUSE          0x13 */
    c"capslock".as_ptr (),       /* VK_CAPITAL        0x14 */
    c"kana".as_ptr (),           /* VK_KANA/VK_HANGUL 0x15 */
    std::ptr::null (),                /*    0x16                */
    c"junja".as_ptr (),          /* VK_JUNJA          0x17 */
    c"final".as_ptr (),          /* VK_FINAL          0x18 */
    c"kanji".as_ptr (),          /* VK_KANJI/VK_HANJA 0x19 */
    std::ptr::null (),                /*    0x1A                */
    c"escape".as_ptr (),         /* VK_ESCAPE         0x1B */
    c"convert".as_ptr (),        /* VK_CONVERT        0x1C */
    c"non-convert".as_ptr (),    /* VK_NONCONVERT     0x1D */
    c"accept".as_ptr (),         /* VK_ACCEPT         0x1E */
    c"mode-change".as_ptr (),    /* VK_MODECHANGE     0x1F */
    std::ptr::null (),                /* VK_SPACE          0x20 */
    c"prior".as_ptr (),          /* VK_PRIOR          0x21 */
    c"next".as_ptr (),           /* VK_NEXT           0x22 */
    c"end".as_ptr (),            /* VK_END            0x23 */
    c"home".as_ptr (),           /* VK_HOME           0x24 */
    c"left".as_ptr (),           /* VK_LEFT           0x25 */
    c"up".as_ptr (),             /* VK_UP             0x26 */
    c"right".as_ptr (),          /* VK_RIGHT          0x27 */
    c"down".as_ptr (),           /* VK_DOWN           0x28 */
    c"select".as_ptr (),         /* VK_SELECT         0x29 */
    c"print".as_ptr (),          /* VK_PRINT          0x2A */
    c"execute".as_ptr (),        /* VK_EXECUTE        0x2B */
    c"snapshot".as_ptr (),       /* VK_SNAPSHOT       0x2C */
    c"insert".as_ptr (),         /* VK_INSERT         0x2D */
    c"delete".as_ptr (),         /* VK_DELETE         0x2E */
    c"help".as_ptr (),           /* VK_HELP           0x2F */

    /* VK_0 thru VK_9 are the same as ASCII '0' thru '9' (0x30 - 0x39) */

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), /* 0x3A .. 0x40       */

    /* VK_A thru VK_Z are the same as ASCII 'A' thru 'Z' (0x41 - 0x5A) */

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),

    c"lwindow".as_ptr (),       /* VK_LWIN           0x5B */
    c"rwindow".as_ptr (),       /* VK_RWIN           0x5C */
    c"apps".as_ptr (),          /* VK_APPS           0x5D */
    std::ptr::null (),               /*    0x5E                */
    c"sleep".as_ptr (),
    c"kp-0".as_ptr (),          /* VK_NUMPAD0        0x60 */
    c"kp-1".as_ptr (),          /* VK_NUMPAD1        0x61 */
    c"kp-2".as_ptr (),          /* VK_NUMPAD2        0x62 */
    c"kp-3".as_ptr (),          /* VK_NUMPAD3        0x63 */
    c"kp-4".as_ptr (),          /* VK_NUMPAD4        0x64 */
    c"kp-5".as_ptr (),          /* VK_NUMPAD5        0x65 */
    c"kp-6".as_ptr (),          /* VK_NUMPAD6        0x66 */
    c"kp-7".as_ptr (),          /* VK_NUMPAD7        0x67 */
    c"kp-8".as_ptr (),          /* VK_NUMPAD8        0x68 */
    c"kp-9".as_ptr (),          /* VK_NUMPAD9        0x69 */
    c"kp-multiply".as_ptr (),   /* VK_MULTIPLY       0x6A */
    c"kp-add".as_ptr (),        /* VK_ADD            0x6B */
    c"kp-separator".as_ptr (),  /* VK_SEPARATOR      0x6C */
    c"kp-subtract".as_ptr (),   /* VK_SUBTRACT       0x6D */
    c"kp-decimal".as_ptr (),    /* VK_DECIMAL        0x6E */
    c"kp-divide".as_ptr (),     /* VK_DIVIDE         0x6F */
    c"f1".as_ptr (),            /* VK_F1             0x70 */
    c"f2".as_ptr (),            /* VK_F2             0x71 */
    c"f3".as_ptr (),            /* VK_F3             0x72 */
    c"f4".as_ptr (),            /* VK_F4             0x73 */
    c"f5".as_ptr (),            /* VK_F5             0x74 */
    c"f6".as_ptr (),            /* VK_F6             0x75 */
    c"f7".as_ptr (),            /* VK_F7             0x76 */
    c"f8".as_ptr (),            /* VK_F8             0x77 */
    c"f9".as_ptr (),            /* VK_F9             0x78 */
    c"f10".as_ptr (),           /* VK_F10            0x79 */
    c"f11".as_ptr (),           /* VK_F11            0x7A */
    c"f12".as_ptr (),           /* VK_F12            0x7B */
    c"f13".as_ptr (),           /* VK_F13            0x7C */
    c"f14".as_ptr (),           /* VK_F14            0x7D */
    c"f15".as_ptr (),           /* VK_F15            0x7E */
    c"f16".as_ptr (),           /* VK_F16            0x7F */
    c"f17".as_ptr (),           /* VK_F17            0x80 */
    c"f18".as_ptr (),           /* VK_F18            0x81 */
    c"f19".as_ptr (),           /* VK_F19            0x82 */
    c"f20".as_ptr (),           /* VK_F20            0x83 */
    c"f21".as_ptr (),           /* VK_F21            0x84 */
    c"f22".as_ptr (),           /* VK_F22            0x85 */
    c"f23".as_ptr (),           /* VK_F23            0x86 */
    c"f24".as_ptr (),           /* VK_F24            0x87 */

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),      /*    0x88 .. 0x8B        */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),      /*    0x8C .. 0x8F        */

    c"kp-numlock".as_ptr (),    /* VK_NUMLOCK        0x90 */
    c"scroll".as_ptr (),        /* VK_SCROLL         0x91 */
    /* Not sure where the following block comes from.
       Windows headers have NEC and Fujitsu specific keys in
       this block, but nothing generic.  */
    c"kp-space".as_ptr (),	     /* VK_NUMPAD_CLEAR   0x92 */
    c"kp-enter".as_ptr (),	     /* VK_NUMPAD_ENTER   0x93 */
    c"kp-prior".as_ptr (),	     /* VK_NUMPAD_PRIOR   0x94 */
    c"kp-next".as_ptr (),	     /* VK_NUMPAD_NEXT    0x95 */
    c"kp-end".as_ptr (),	     /* VK_NUMPAD_END     0x96 */
    c"kp-home".as_ptr (),	     /* VK_NUMPAD_HOME    0x97 */
    c"kp-left".as_ptr (),	     /* VK_NUMPAD_LEFT    0x98 */
    c"kp-up".as_ptr (),	     /* VK_NUMPAD_UP      0x99 */
    c"kp-right".as_ptr (),	     /* VK_NUMPAD_RIGHT   0x9A */
    c"kp-down".as_ptr (),	     /* VK_NUMPAD_DOWN    0x9B */
    c"kp-insert".as_ptr (),     /* VK_NUMPAD_INSERT  0x9C */
    c"kp-delete".as_ptr (),     /* VK_NUMPAD_DELETE  0x9D */

    std::ptr::null (), std::ptr::null (),	     /*    0x9E .. 0x9F        */

    /*
     * VK_L* & VK_R* - left and right Alt, Ctrl and Shift virtual keys.
     * Used only as parameters to GetAsyncKeyState and GetKeyState.
     * No other API or message will distinguish left and right keys this way.
     * 0xA0 .. 0xA5
     */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),

    /* Multimedia keys. These are handled as WM_APPCOMMAND, which allows us
       to enable them selectively, and gives access to a few more functions.
       See lispy_multimedia_keys below.  */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), /* 0xA6 .. 0xAC        Browser */
    std::ptr::null (), std::ptr::null (), std::ptr::null (),             /* 0xAD .. 0xAF         Volume */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),          /* 0xB0 .. 0xB3          Media */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),          /* 0xB4 .. 0xB7           Apps */

    /* 0xB8 .. 0xC0 "OEM" keys - all seem to be punctuation.  */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),

    /* 0xC1 - 0xDA unallocated, 0xDB-0xDF more OEM keys */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),

    std::ptr::null (),               /* 0xE0                   */
    c"ax".as_ptr (),            /* VK_OEM_AX         0xE1 */
    std::ptr::null (),               /* VK_OEM_102        0xE2 */
    c"ico-help".as_ptr (),      /* VK_ICO_HELP       0xE3 */
    c"ico-00".as_ptr (),        /* VK_ICO_00         0xE4 */
    std::ptr::null (),               /* VK_PROCESSKEY     0xE5 - used by IME */
    c"ico-clear".as_ptr (),     /* VK_ICO_CLEAR      0xE6 */
    std::ptr::null (),               /* VK_PACKET         0xE7  - used to pass Unicode chars */
    std::ptr::null (),               /*                   0xE8 */
    c"reset".as_ptr (),         /* VK_OEM_RESET      0xE9 */
    c"jump".as_ptr (),          /* VK_OEM_JUMP       0xEA */
    c"oem-pa1".as_ptr (),       /* VK_OEM_PA1        0xEB */
    c"oem-pa2".as_ptr (),       /* VK_OEM_PA2        0xEC */
    c"oem-pa3".as_ptr (),       /* VK_OEM_PA3        0xED */
    c"wsctrl".as_ptr (),        /* VK_OEM_WSCTRL     0xEE */
    c"cusel".as_ptr (),         /* VK_OEM_CUSEL      0xEF */
    c"oem-attn".as_ptr (),      /* VK_OEM_ATTN       0xF0 */
    c"finish".as_ptr (),        /* VK_OEM_FINISH     0xF1 */
    c"copy".as_ptr (),          /* VK_OEM_COPY       0xF2 */
    c"auto".as_ptr (),          /* VK_OEM_AUTO       0xF3 */
    c"enlw".as_ptr (),          /* VK_OEM_ENLW       0xF4 */
    c"backtab".as_ptr (),       /* VK_OEM_BACKTAB    0xF5 */
    c"attn".as_ptr (),          /* VK_ATTN           0xF6 */
    c"crsel".as_ptr (),         /* VK_CRSEL          0xF7 */
    c"exsel".as_ptr (),         /* VK_EXSEL          0xF8 */
    c"ereof".as_ptr (),         /* VK_EREOF          0xF9 */
    c"play".as_ptr (),          /* VK_PLAY           0xFA */
    c"zoom".as_ptr (),          /* VK_ZOOM           0xFB */
    c"noname".as_ptr (),        /* VK_NONAME         0xFC */
    c"pa1".as_ptr (),           /* VK_PA1            0xFD */
    c"oem_clear".as_ptr (),     /* VK_OEM_CLEAR      0xFE */
    std::ptr::null () /* 0xFF */
];


/* Some of these duplicate the "Media keys" on newer keyboards,
   but they are delivered to the application in a different way.  */
#[cfg (feature = "have_ntgui")]
static mut lispy_multimedia_keys: [*const c_char; 53] = [
    std::ptr::null (),
    c"browser-back".as_ptr (),
    c"browser-forward".as_ptr (),
    c"browser-refresh".as_ptr (),
    c"browser-stop".as_ptr (),
    c"browser-search".as_ptr (),
    c"browser-favorites".as_ptr (),
    c"browser-home".as_ptr (),
    c"volume-mute".as_ptr (),
    c"volume-down".as_ptr (),
    c"volume-up".as_ptr (),
    c"media-next".as_ptr (),
    c"media-previous".as_ptr (),
    c"media-stop".as_ptr (),
    c"media-play-pause".as_ptr (),
    c"mail".as_ptr (),
    c"media-select".as_ptr (),
    c"app-1".as_ptr (),
    c"app-2".as_ptr (),
    c"bass-down".as_ptr (),
    c"bass-boost".as_ptr (),
    c"bass-up".as_ptr (),
    c"treble-down".as_ptr (),
    c"treble-up".as_ptr (),
    c"mic-volume-mute".as_ptr (),
    c"mic-volume-down".as_ptr (),
    c"mic-volume-up".as_ptr (),
    c"help".as_ptr (),
    c"find".as_ptr (),
    c"new".as_ptr (),
    c"open".as_ptr (),
    c"close".as_ptr (),
    c"save".as_ptr (),
    c"print".as_ptr (),
    c"undo".as_ptr (),
    c"redo".as_ptr (),
    c"copy".as_ptr (),
    c"cut".as_ptr (),
    c"paste".as_ptr (),
    c"mail-reply".as_ptr (),
    c"mail-forward".as_ptr (),
    c"mail-send".as_ptr (),
    c"spell-check".as_ptr (),
    c"toggle-dictate-command".as_ptr (),
    c"mic-toggle".as_ptr (),
    c"correction-list".as_ptr (),
    c"media-play".as_ptr (),
    c"media-pause".as_ptr (),
    c"media-record".as_ptr (),
    c"media-fast-forward".as_ptr (),
    c"media-rewind".as_ptr (),
    c"media-channel-up".as_ptr (),
    c"media-channel-down".as_ptr ()
];


/* C 5463-5618: the chain's `#else /* not HAVE_NTGUI */` arm — the live one.  */

/* This should be dealt with in XTread_socket now, and that doesn't
   depend on the client system having the Kana syms defined.  See also
   the XK_kana_A case below.  */
/* C 5468-5503: `#if 0` around `lispy_kana_keys`; dead, nothing carried (the
   `dead_branch_map` note in docs/ports/keyboard.md).  */
/* C 5505: `#define FUNCTION_KEY_OFFSET 0xff00` — the value the frozen configuration
   compiles (HAVE_ANDROID's `0` and HAVE_NTGUI's `0x0` are the other two, both above).  */
pub const FUNCTION_KEY_OFFSET: c_int = 0xff00;

/* You'll notice that this table is arranged to be conveniently
   indexed by X Windows keysym values.  */
/* C 5509-5512: `#if defined HAVE_NS || !defined HAVE_WINDOW_SYSTEM` around the
   `static` keyword; the condition is true here, so the definition is `static`.  */
static mut lispy_function_keys: [*const c_char; 256] = [
    /* X Keysym value */

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),			      /* 0xff00...0f */
    c"backspace".as_ptr (), c"tab".as_ptr (), c"linefeed".as_ptr (), c"clear".as_ptr (),
    std::ptr::null (), c"return".as_ptr (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), c"pause".as_ptr (),				      /* 0xff10...1f */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), c"escape".as_ptr (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), c"kanji".as_ptr (), c"muhenkan".as_ptr (), c"henkan".as_ptr (),		      /* 0xff20...2f */
    c"romaji".as_ptr (), c"hiragana".as_ptr (), c"katakana".as_ptr (), c"hiragana-katakana".as_ptr (),
    c"zenkaku".as_ptr (), c"hankaku".as_ptr (), c"zenkaku-hankaku".as_ptr (), c"touroku".as_ptr (),
    c"massyo".as_ptr (), c"kana-lock".as_ptr (), c"kana-shift".as_ptr (), c"eisu-shift".as_ptr (),
    c"eisu-toggle".as_ptr (),				      /* 0xff30...3f */
       std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),   /* 0xff40...4f */

    c"home".as_ptr (), c"left".as_ptr (), c"up".as_ptr (), c"right".as_ptr (), /* 0xff50 */	/* IsCursorKey */
    c"down".as_ptr (), c"prior".as_ptr (), c"next".as_ptr (), c"end".as_ptr (),
    c"begin".as_ptr (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    c"select".as_ptr (),			/* 0xff60 */	/* IsMiscFunctionKey */
    c"print".as_ptr (),
    c"execute".as_ptr (),
    c"insert".as_ptr (),
    std::ptr::null (),		/* 0xff64 */
    c"undo".as_ptr (),
    c"redo".as_ptr (),
    c"menu".as_ptr (),
    c"find".as_ptr (),
    c"cancel".as_ptr (),
    c"help".as_ptr (),
    c"break".as_ptr (),			/* 0xff6b */

    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), c"backtab".as_ptr (), std::ptr::null (), std::ptr::null (), std::ptr::null (),		/* 0xff70...  */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), c"kp-numlock".as_ptr (),		/* 0xff78...  */
    c"kp-space".as_ptr (),			/* 0xff80 */	/* IsKeypadKey */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    c"kp-tab".as_ptr (),			/* 0xff89 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (),
    c"kp-enter".as_ptr (),			/* 0xff8d */
    std::ptr::null (), std::ptr::null (), std::ptr::null (),
    c"kp-f1".as_ptr (),			/* 0xff91 */
    c"kp-f2".as_ptr (),
    c"kp-f3".as_ptr (),
    c"kp-f4".as_ptr (),
    c"kp-home".as_ptr (),			/* 0xff95 */
    c"kp-left".as_ptr (),
    c"kp-up".as_ptr (),
    c"kp-right".as_ptr (),
    c"kp-down".as_ptr (),
    c"kp-prior".as_ptr (),			/* kp-page-up */
    c"kp-next".as_ptr (),			/* kp-page-down */
    c"kp-end".as_ptr (),
    c"kp-begin".as_ptr (),
    c"kp-insert".as_ptr (),
    c"kp-delete".as_ptr (),
    std::ptr::null (),				/* 0xffa0 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    c"kp-multiply".as_ptr (),		/* 0xffaa */
    c"kp-add".as_ptr (),
    c"kp-separator".as_ptr (),
    c"kp-subtract".as_ptr (),
    c"kp-decimal".as_ptr (),
    c"kp-divide".as_ptr (),		/* 0xffaf */
    c"kp-0".as_ptr (),			/* 0xffb0 */
    c"kp-1".as_ptr (),	c"kp-2".as_ptr (),	c"kp-3".as_ptr (),	c"kp-4".as_ptr (),	c"kp-5".as_ptr (),	c"kp-6".as_ptr (),	c"kp-7".as_ptr (),	c"kp-8".as_ptr (),	c"kp-9".as_ptr (),
    std::ptr::null (),		/* 0xffba */
    std::ptr::null (), std::ptr::null (),
    c"kp-equal".as_ptr (),			/* 0xffbd */
    c"f1".as_ptr (),			/* 0xffbe */	/* IsFunctionKey */
    c"f2".as_ptr (),
    c"f3".as_ptr (), c"f4".as_ptr (), c"f5".as_ptr (), c"f6".as_ptr (), c"f7".as_ptr (), c"f8".as_ptr (),	c"f9".as_ptr (), c"f10".as_ptr (), /* 0xffc0 */
    c"f11".as_ptr (), c"f12".as_ptr (), c"f13".as_ptr (), c"f14".as_ptr (), c"f15".as_ptr (), c"f16".as_ptr (), c"f17".as_ptr (), c"f18".as_ptr (),
    c"f19".as_ptr (), c"f20".as_ptr (), c"f21".as_ptr (), c"f22".as_ptr (), c"f23".as_ptr (), c"f24".as_ptr (), c"f25".as_ptr (), c"f26".as_ptr (), /* 0xffd0 */
    c"f27".as_ptr (), c"f28".as_ptr (), c"f29".as_ptr (), c"f30".as_ptr (), c"f31".as_ptr (), c"f32".as_ptr (), c"f33".as_ptr (), c"f34".as_ptr (),
    c"f35".as_ptr (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),	/* 0xffe0 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),     /* 0xfff0 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), c"delete".as_ptr ()
];

/* C 5599-5616: `iso_lispy_function_keys`.  */
/* ISO 9995 Function and Modifier Keys; the first byte is 0xFE.  */
/* C 5597: `#define ISO_FUNCTION_KEY_OFFSET 0xfe00`.  */
pub const ISO_FUNCTION_KEY_OFFSET: c_int = 0xfe00;

static mut iso_lispy_function_keys: [*const c_char; 53] = [
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),	/* 0xfe00 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),	/* 0xfe08 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),	/* 0xfe10 */
    std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),	/* 0xfe18 */
    c"iso-lefttab".as_ptr (),		/* 0xfe20 */
    c"iso-move-line-up".as_ptr (), c"iso-move-line-down".as_ptr (),
    c"iso-partial-line-up".as_ptr (), c"iso-partial-line-down".as_ptr (),
    c"iso-partial-space-left".as_ptr (), c"iso-partial-space-right".as_ptr (),
    c"iso-set-margin-left".as_ptr (), c"iso-set-margin-right".as_ptr (), /* 0xffe27, 28 */
    c"iso-release-margin-left".as_ptr (), c"iso-release-margin-right".as_ptr (),
    c"iso-release-both-margins".as_ptr (),
    c"iso-fast-cursor-left".as_ptr (), c"iso-fast-cursor-right".as_ptr (),
    c"iso-fast-cursor-up".as_ptr (), c"iso-fast-cursor-down".as_ptr (),
    c"iso-continuous-underline".as_ptr (), c"iso-discontinuous-underline".as_ptr (), /* 0xfe30, 31 */
    c"iso-emphasize".as_ptr (), c"iso-center-object".as_ptr (), c"iso-enter".as_ptr (), /* ... 0xfe34 */
];


/* C 5620: `static Lisp_Object Vlispy_mouse_stem;`.  */
static mut Vlispy_mouse_stem: Lisp_Object = lisp_h_Qnil ();


/* C 5622-5625 and 5629-5632: the wheel and drag-n-drop name tables.  */
static mut lispy_wheel_names: [*const c_char; 4] = [
  c"wheel-up".as_ptr (), c"wheel-down".as_ptr (), c"wheel-left".as_ptr (), c"wheel-right".as_ptr ()
];

/* drag-n-drop events are generated when a set of selected files are
   dragged from another application and dropped onto an Emacs window.  */
static mut lispy_drag_n_drop_names: [*const c_char; 1] = [
  c"drag-n-drop".as_ptr ()
];

/* An array of symbol indexes of scroll bar parts, indexed by an enum
   scroll_bar_part value.  Note that Qnil corresponds to
   scroll_bar_nowhere and should not appear in Lisp events.  */
static scroll_bar_parts: [c_short; 19] = [
  iQnil as c_short, iQabove_handle as c_short, iQhandle as c_short,
  iQbelow_handle as c_short, iQup as c_short, iQdown as c_short,
  iQtop as c_short, iQbottom as c_short, iQend_scroll as c_short,
  iQratio as c_short, iQbefore_handle as c_short,
  iQhorizontal_handle as c_short, iQafter_handle as c_short,
  iQleft as c_short, iQright as c_short, iQleftmost as c_short,
  iQrightmost as c_short, iQend_scroll as c_short, iQratio as c_short
];


/* C 5637-5645 and 5650-5656: the two symbol-index tables.  */
/* An array of symbol indexes of internal border parts, indexed by an enum
   internal_border_part value.  Note that Qnil corresponds to
   internal_border_part_none and should not appear in Lisp events.  */
static internal_border_parts: [c_short; 9] = [
  iQnil as c_short, iQleft_edge as c_short,
  iQtop_left_corner as c_short, iQtop_edge as c_short,
  iQtop_right_corner as c_short, iQright_edge as c_short,
  iQbottom_right_corner as c_short, iQbottom_edge as c_short,
  iQbottom_left_corner as c_short
];


/* C 5658-5689: the mouse-state objects.  */
/* A vector, indexed by button number, giving the down-going location
   of currently depressed buttons, both scroll bar and non-scroll bar.

   The elements have the form
     (BUTTON-NUMBER MODIFIER-MASK . REST)
   where REST is the cdr of a position as it would be reported in the event.

   The make_lispy_event function stores positions here to tell the
   difference between click and drag events, and to store the starting
   location to be included in drag events.  */

static mut button_down_location: Lisp_Object = lisp_h_Qnil ();

/* A cons recording the original frame-relative x and y coordinates of
   the down mouse event.  */
static mut frame_relative_event_pos: Lisp_Object = lisp_h_Qnil ();

/* The line-number display width, in columns, at the time of most
   recent down mouse event.  */
static mut down_mouse_line_number_width: c_int = 0;

/* Information about the most recent up-going button event:  Which
   button, what location, and what time.  */

static mut last_mouse_button: c_int = 0;
static mut last_mouse_x: c_int = 0;
static mut last_mouse_y: c_int = 0;
static mut button_down_time: Time = 0;

/* The number of clicks in this multiple-click.  */

static mut double_click_count: c_int = 0;

/* C 5691-5698: `enum frame_border_side` — the port's constants (PORTING.md 1.3: C's
   enum ABI is `c_int`).  */
pub type frame_border_side = c_int;
pub const ON_LEFT: frame_border_side = 0;
pub const ON_TOP: frame_border_side = 1;
pub const ON_RIGHT: frame_border_side = 2;
pub const ON_BOTTOM: frame_border_side = 3;
pub const ON_NONE: frame_border_side = 4;

/* C: frame.h 34-45: `enum internal_border_part` — which the `frame.rs` mirror does not carry
   (P-66); this unit needs the type and `frame_internal_border_part`'s result indexes
   `internal_border_parts` with it.  PORTING.md 1.3: C's enum ABI is `c_int`.  */
pub type internal_border_part = c_int;
pub const INTERNAL_BORDER_NONE: internal_border_part = 0;
pub const INTERNAL_BORDER_LEFT_EDGE: internal_border_part = 1;
pub const INTERNAL_BORDER_TOP_LEFT_CORNER: internal_border_part = 2;
pub const INTERNAL_BORDER_TOP_EDGE: internal_border_part = 3;
pub const INTERNAL_BORDER_TOP_RIGHT_CORNER: internal_border_part = 4;
pub const INTERNAL_BORDER_RIGHT_EDGE: internal_border_part = 5;
pub const INTERNAL_BORDER_BOTTOM_RIGHT_CORNER: internal_border_part = 6;
pub const INTERNAL_BORDER_BOTTOM_EDGE: internal_border_part = 7;
pub const INTERNAL_BORDER_BOTTOM_LEFT_CORNER: internal_border_part = 8;

/* ---------------------------------------------------------------------------
   C 5700-6219: the tty child-frame position helper, `make_lispy_position`, the toolkit
   menu-bar predicate, the scroll-bar position builder and the three line-number helpers.

   `make_lispy_position` (C 5784-6099) walks window.h's geometry macros
   (`WINDOW_LEFT_EDGE_X`/`_TOP_EDGE_Y`, `WINDOW_TAB_LINE_HEIGHT`, `WINDOW_HEADER_LINE_HEIGHT`,
   `WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH`, `WINDOW_SCROLL_BAR_AREA_WIDTH`/`_HEIGHT`,
   `WINDOW_RIGHT_DIVIDER_WIDTH`, `WINDOW_BOTTOM_DIVIDER_WIDTH`,
   `WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT`), which `include/window.rs` now carries, plus
   `WINDOW_HAS_FRINGES_OUTSIDE_MARGINS` — window.h 872's macro, which the mirror spells as a
   `bool` function (the accessor shape, include/window.rs 331).  Six symbols it needs carry no
   mirror declaration (window.h 1124's `window_from_coordinates`; dispextern.h 3578/3579's
   `window_box_width`/`window_box_left` and 3883-3895's `buffer_posn_from_coords`,
   `mode_line_string`/`marginal_area_string`), so they stand in this unit's use-site block
   above (P-72③).  The `HAVE_WINDOW_SYSTEM` and `HAVE_EXT_TOOL_BAR` arms are carried per P-51.
   --------------------------------------------------------------------------- */

/* Handle make_lispy_event when a tty child frame's decorations shall be
   used in lieu of internal borders.  R denotes the root frame under
   investigation, MX and MY are the positions of the mouse relative to
   R.  WINDOW_OR_FRAME denotes the frame previously reported as the
   frame under (MX, MY).  Note: The decorations of a child frame are
   always drawn outside the child frame, so WINDOW_OR_FRAME is certainly
   not the frame we are looking for.  Neither is R.  A candidate frame
   is any frame but WINDOW_OR_FRAME and R whose root is R, which is not
   decorated and has a 'drag-internal-border' parameter.  If we find a
   suitable frame, set WINDOW_OR_FRAME to it and POSN to the part of the
   internal border corresponding to (MX, MY) on the frame found.

   Value is 1 if MX and MY rest in one of R or its children's
   decorations, and 0 otherwise.  */

/* C 5715-5784: `make_lispy_tty_position`.  */
unsafe fn make_lispy_tty_position (r: *mut frame, mx: c_int, my: c_int,
				   window_or_frame: *mut Lisp_Object, posn: *mut Lisp_Object)
  -> c_int
{
  let mut side: frame_border_side = ON_NONE;
  let mut f: *mut frame = std::ptr::null_mut ();
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  /* C's uninitialised `ix` is zero-initialised (PORTING.md 1.5).  */
  let mut ix: c_int = 0;
  let mut iy: c_int = 0;

  FOR_EACH_FRAME! (tail, frame, {
    f = XFRAME (frame);

    let left: c_int = (*f).left_pos;
    let top: c_int = (*f).top_pos;
    let right: c_int = left + (*f).pixel_width;
    let bottom: c_int = top + (*f).pixel_height;

    if root_frame (f) == r && f != r
	&& FRAME_UNDECORATED (f) == 0
	&& !NILP (get_frame_param (f, Qdrag_internal_border ()))
      {
	if left == mx + 1 && my >= top && my <= bottom
	  {
	    side = ON_LEFT;
	    ix = -1;
	    iy = my - top + 1;
	    break;
	  }
	else if right == mx && my >= top && my <= bottom
	  {
	    side = ON_RIGHT;
	    ix = (*f).pixel_width;
	    iy = my - top + 1;
	    break;
	  }
	else if top == my + 1 && mx >= left && mx <= right
	  {
	    side = ON_TOP;
	    ix = mx - left + 1;
	    iy = -1;
	    break;
	  }
	else if bottom == my && mx >= left && mx <= right
	  {
	    side = ON_BOTTOM;
	    ix = mx - left + 1;
	    iy = (*f).pixel_height;
	    break;
	  }
      }
  });

  if side != ON_NONE
    {
      let part: internal_border_part = frame_internal_border_part (f, ix, iy);

      XSETFRAME! (*window_or_frame, f);
      *posn = builtin_lisp_symbol (internal_border_parts[part as usize] as usize);
      return 1;
    }

  return 0;
}
/* C: dispextern.h 287: `#define CHARPOS(POS)	(POS).charpos` — the mirror does not carry it,
   and `make_lispy_position` is the only ported text that names it.  */
macro_rules! CHARPOS { ($pos:expr) => { ($pos).charpos }; }

/* X and Y are frame-relative coordinates for a click or wheel event.
   Return a Lisp-style event list.  */

/* C 5784-6099: `make_lispy_position`.  Both `#ifdef HAVE_WINDOW_SYSTEM` blocks the text
   carries are `#undef` on this target (P-51), and the `#ifndef HAVE_EXT_TOOL_BAR` inside the
   first is lisp.h 3935-3937's conditional define (undefined here too): the port keeps C's
   tool-bar clause behind a `cfg!` test — the C 4148-4176 shape — so the configuration C does
   not compile is the same expression with that clause dropped.  */
unsafe fn make_lispy_position (f: *mut frame, x: Lisp_Object, y: Lisp_Object, t: Time)
  -> Lisp_Object
{
  /* C's uninitialised `part` is zero-initialised (PORTING.md 1.5); the call below fills it in
     whenever F is non-NULL.  */
  let mut part: window_part = 0;
  let mut posn: Lisp_Object = Qnil ();
  let mut extra_info: Lisp_Object = Qnil ();
  let mx: c_int = XFIXNUM (x) as c_int;
  let my: c_int = XFIXNUM (y) as c_int;
  /* Coordinate pixel positions to return.  */
  let mut xret: c_int = 0;
  let mut yret: c_int = 0;
  /* The window or frame under frame pixel coordinates (x,y)  */
  let mut window_or_frame: Lisp_Object
    = if !f.is_null ()
	{ window_from_coordinates (f, mx, my, &raw mut part, false, true, true) }
      else
	{ Qnil () };

  #[cfg (feature = "have_window_system")]
  {
  let mut tool_bar_p: bool = false;
  let mut menu_bar_p: bool = false;

  /* Report mouse events on the tab bar and (on GUI frames) on the
     tool bar.  */
  if !f.is_null ()
      && ((WINDOWP ((*f).tab_bar_window)
	   && EQ (window_or_frame, (*f).tab_bar_window))
	  || (!cfg! (feature = "have_ext_tool_bar")
	      && (WINDOWP ((*f).tool_bar_window)
		  && EQ (window_or_frame, (*f).tool_bar_window))))
    {
      /* While 'track-mouse' is neither nil nor t, do not report this
	 event as something that happened on the tool or tab bar since
	 that would break mouse drag operations that originate from an
	 ordinary window beneath that bar and expect the window to
	 auto-scroll as soon as the mouse cursor appears above or
	 beneath it (Bug#50993).  We do allow reports for t, because
	 applications may have set 'track-mouse' to t and still expect a
	 click on the tool or tab bar to get through (Bug#51794).

	 FIXME: This is a preliminary fix for the bugs cited above and
	 awaits a solution that includes a convention for all special
	 values of 'track-mouse' and their documentation in the Elisp
	 manual.  */
      if NILP (track_mouse) || EQ (track_mouse, Qt ())
	{ posn = if EQ (window_or_frame, (*f).tab_bar_window)
		   { Qtab_bar () } else { Qtool_bar () }; }
      /* Kludge alert: for mouse events on the tab bar and tool bar,
	 keyboard.c wants the frame, not the special-purpose window
	 we use to display those, and it wants frame-relative
	 coordinates.  FIXME!  */
      window_or_frame = Qnil ();
    }

  if !f.is_null () && (*FRAME_TERMINAL! (f)).toolkit_position_hook.is_some ()
    {
      ((*FRAME_TERMINAL! (f)).toolkit_position_hook.unwrap ())
	(f as *mut c_void, mx, my, &raw mut menu_bar_p, &raw mut tool_bar_p);

      if NILP (track_mouse) || EQ (track_mouse, Qt ())
	{
	  if menu_bar_p
	    { posn = Qmenu_bar (); }
	  else if tool_bar_p
	    { posn = Qtool_bar (); }
	}
    }
  }

  if !f.is_null ()
      && !FRAME_WINDOW_P (f)
      && FRAME_TAB_BAR_LINES (f) > 0
      && my >= FRAME_MENU_BAR_LINES (f)
      && my < FRAME_MENU_BAR_LINES (f) + FRAME_TAB_BAR_LINES (f)
    {
      posn = Qtab_bar ();
      window_or_frame = Qnil ();	/* see above */
    }

  if WINDOWP (window_or_frame) && is_tty_frame (f)
      && (is_tty_root_frame_with_visible_child (f)
	  || is_tty_child_frame (f))
      && make_lispy_tty_position (root_frame (f), mx, my,
				  &raw mut window_or_frame, &raw mut posn) != 0
    /* C's `;` here: the call's effects are the whole statement.  */
    { }
  else if WINDOWP (window_or_frame)
    {
      /* It's a click in window WINDOW at frame coordinates (X,Y)  */
      let w: *mut window = XWINDOW (window_or_frame);
      let mut string_info: Lisp_Object = Qnil ();
      let mut textpos: ptrdiff_t = 0;
      let mut col: c_int = -1;
      let mut row: c_int = -1;
      let mut dx: c_int = -1;
      let mut dy: c_int = -1;
      let mut width: c_int = -1;
      let mut height: c_int = -1;
      let mut object: Lisp_Object = Qnil ();

      /* Pixel coordinates relative to the window corner.  */
      let wx: c_int = mx - WINDOW_LEFT_EDGE_X! (w);
      let wy: c_int = my - WINDOW_TOP_EDGE_Y! (w);

      /* For text area clicks, return X, Y relative to the corner of
	 this text area.  Note that dX, dY etc are set below, by
	 buffer_posn_from_coords.  */
      if part == ON_TEXT
	{
	  xret = mx - window_box_left (w, TEXT_AREA);
	  yret = wy - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w);
	}
      /* For mode line and header line clicks, return X, Y relative to
	 the left window edge.  Use mode_line_string to look for a
	 string on the click position.  */
      else if part == ON_MODE_LINE || part == ON_TAB_LINE
	       || part == ON_HEADER_LINE
	{
	  let mut string: Lisp_Object = Qnil ();
	  let mut charpos: ptrdiff_t = 0;

	  posn = if part == ON_MODE_LINE
		   { Qmode_line () }
		 else if part == ON_TAB_LINE
		   { Qtab_line () }
		 else
		   { Qheader_line () };

	  /* Note that mode_line_string takes COL, ROW as pixels and
	     converts them to characters.  */
	  col = wx;
	  row = wy;
	  string = mode_line_string (w, part, &raw mut col, &raw mut row, &raw mut charpos,
				     &raw mut object, &raw mut dx, &raw mut dy,
				     &raw mut width, &raw mut height);
	  if STRINGP (string)
	    { string_info = Fcons (string, make_fixnum (charpos)); }
	  textpos = -1;

	  xret = wx;
	  yret = wy;
	}
      /* For fringes and margins, Y is relative to the area's (and the
	 window's) top edge, while X is meaningless.  */
      else if part == ON_LEFT_MARGIN || part == ON_RIGHT_MARGIN
	{
	  let mut string: Lisp_Object = Qnil ();
	  let mut charpos: ptrdiff_t = 0;

	  posn = if part == ON_LEFT_MARGIN { Qleft_margin () } else { Qright_margin () };
	  /* Skip any scroll bar on the left (Bug#79846).  */
	  col = wx - WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH! (w);
	  row = wy;
	  string = marginal_area_string (w, part, &raw mut col, &raw mut row, &raw mut charpos,
					 &raw mut object, &raw mut dx, &raw mut dy,
					 &raw mut width, &raw mut height);
	  if STRINGP (string)
	    { string_info = Fcons (string, make_fixnum (charpos)); }
	  xret = wx;
	  yret = wy - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w);
	}
      else if part == ON_LEFT_FRINGE
	{
	  posn = Qleft_fringe ();
	  col = 0;
	  xret = wx;
	  dx = wx
	    - (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
		 { 0 } else { window_box_width (w, LEFT_MARGIN_AREA) });
	  /* C's `dy = yret = wy - …` is one chained assignment; the port keeps C's two
	     effects in C's order (the frame.rs shape).  */
	  yret = wy - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w);
	  dy = yret;
	}
      else if part == ON_RIGHT_FRINGE
	{
	  posn = Qright_fringe ();
	  col = 0;
	  xret = wx;
	  dx = wx
	    - window_box_width (w, LEFT_MARGIN_AREA)
	    - window_box_width (w, TEXT_AREA)
	    - (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
		 { window_box_width (w, RIGHT_MARGIN_AREA) } else { 0 });
	  yret = wy - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w);
	  dy = yret;
	}
      else if part == ON_VERTICAL_BORDER
	{
	  posn = Qvertical_line ();
	  width = 1;
	  dx = 0;
	  xret = wx;
	  yret = wy;
	  dy = yret;
	}
      else if part == ON_VERTICAL_SCROLL_BAR
	{
	  posn = Qvertical_scroll_bar ();
	  width = WINDOW_SCROLL_BAR_AREA_WIDTH! (w);
	  xret = wx;
	  dx = xret;
	  yret = wy;
	  dy = yret;
	}
      else if part == ON_HORIZONTAL_SCROLL_BAR
	{
	  posn = Qhorizontal_scroll_bar ();
	  width = WINDOW_SCROLL_BAR_AREA_HEIGHT! (w);
	  xret = wx;
	  dx = xret;
	  yret = wy;
	  dy = yret;
	}
      else if part == ON_RIGHT_DIVIDER
	{
	  posn = Qright_divider ();
	  width = WINDOW_RIGHT_DIVIDER_WIDTH! (w);
	  xret = wx;
	  dx = xret;
	  yret = wy;
	  dy = yret;
	}
      else if part == ON_BOTTOM_DIVIDER
	{
	  posn = Qbottom_divider ();
	  width = WINDOW_BOTTOM_DIVIDER_WIDTH! (w);
	  xret = wx;
	  dx = xret;
	  yret = wy;
	  dy = yret;
	}

      /* For clicks in the text area, fringes, margins, or vertical
	 scroll bar, call buffer_posn_from_coords to extract TEXTPOS,
	 the buffer position nearest to the click.  */
      if textpos == 0
	{
	  /* C's uninitialised `string2` and the four ints are zero-initialised (PORTING.md
	     1.5).  */
	  let mut string2: Lisp_Object = Qnil ();
	  let mut object2: Lisp_Object = Qnil ();
	  let mut p: display_pos = std::mem::zeroed ();
	  let mut dx2: c_int = 0;
	  let mut dy2: c_int = 0;
	  let mut width2: c_int = 0;
	  let mut height2: c_int = 0;
	  /* The pixel X coordinate passed to buffer_posn_from_coords
	     is the X coordinate relative to the text area for clicks
	     in text-area, right-margin/fringe and right-side vertical
	     scroll bar, zero otherwise.  */
	  let mut x2: c_int
	    = if part == ON_TEXT
		{ xret }
	      else if part == ON_RIGHT_FRINGE || part == ON_RIGHT_MARGIN
		      || (part == ON_VERTICAL_SCROLL_BAR
			  && WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT! (w))
		{ mx - window_box_left (w, TEXT_AREA) }
	      else
		{ 0 };
	  let mut y2: c_int = wy;

	  string2 = buffer_posn_from_coords (w, &raw mut x2, &raw mut y2, &raw mut p,
					     &raw mut object2, &raw mut dx2, &raw mut dy2,
					     &raw mut width2, &raw mut height2);
	  textpos = CHARPOS! (p.pos);
	  if col < 0 { col = x2; }
	  if row < 0 { row = y2; }
	  if dx < 0 { dx = dx2; }
	  if dy < 0 { dy = dy2; }
	  if width < 0 { width = width2; }
	  if height < 0 { height = height2; }

	  if NILP (posn)
	    {
	      posn = make_fixnum (textpos);
	      if STRINGP (string2)
		{ string_info = Fcons (string2, make_fixnum (CHARPOS! (p.string_pos))); }
	    }
	  if NILP (object)
	    { object = object2; }
	}

      #[cfg (feature = "have_window_system")]
      {
      if IMAGEP (object)
	{
	  let mut image_map: Lisp_Object = Qnil ();
	  let mut hotspot: Lisp_Object = Qnil ();
	  /* C's `(image_map = …, !NILP (image_map))` and its two siblings assign and then
	     test in one expression; Rust's assignment is not an expression, so the port
	     keeps C's effects and the tests in block expressions (the frame.rs shape).  */
	  if { image_map = plist_get (XCDR (object), QCmap ()); !NILP (image_map) }
	      && { hotspot = find_hot_spot (image_map, dx, dy); CONSP (hotspot) }
	      && { hotspot = XCDR (hotspot); CONSP (hotspot) }
	    { posn = XCAR (hotspot); }
	}
      }

      /* Object info.  */
      extra_info
	= list3 (object,
		 Fcons (make_fixnum (dx as EMACS_INT), make_fixnum (dy as EMACS_INT)),
		 Fcons (make_fixnum (width as EMACS_INT), make_fixnum (height as EMACS_INT)));

      /* String info.  */
      extra_info = Fcons (string_info,
			  Fcons (if textpos < 0 { Qnil () } else { make_fixnum (textpos) },
				 Fcons (Fcons (make_fixnum (col as EMACS_INT),
					       make_fixnum (row as EMACS_INT)),
					extra_info)));
    }
  else if !f.is_null ()
    {
      /* Return mouse pixel coordinates here.  */
      XSETFRAME! (window_or_frame, f);
      xret = mx;
      yret = my;

      if FRAME_LIVE_P (f)
	  && NILP (posn)
	  && FRAME_INTERNAL_BORDER_WIDTH (f) > 0
	  && !NILP (get_frame_param (f, Qdrag_internal_border ()))
	{
	  let part: internal_border_part = frame_internal_border_part (f, xret, yret);

	  posn = builtin_lisp_symbol (internal_border_parts[part as usize] as usize);
	}
    }
  else
    {
      if EQ (track_mouse, Qdrag_source ())
	{
	  xret = mx;
	  yret = my;
	}

      window_or_frame = Qnil ();
    }

  return Fcons (window_or_frame,
		Fcons (posn,
		       Fcons (Fcons (make_fixnum (xret as EMACS_INT),
				     make_fixnum (yret as EMACS_INT)),
			      Fcons (INT_TO_INTEGER (t as EMACS_INT),
				     extra_info))));
}

/* Return non-zero if F is a GUI frame that uses some toolkit-managed
   menu bar.  This really means that Emacs draws and manages the menu
   bar as part of its normal display, and therefore can compute its
   geometry.  */

/* C 6103-6111: `toolkit_menubar_in_use`.  `HAVE_EXT_MENU_BAR` is lisp.h 3929-3932's conditional
   define (`USE_X_TOOLKIT`/`HAVE_NTGUI`/`HAVE_NS`/`USE_GTK`/`HAVE_HAIKU` all decide it, all `#undef`
   here), so it is **undefined on this target**: C's `#else` (`return false`) is the live arm and
   C 6106-6107 is the carried one (P-51, the same reading as this file's C 4297-4309 and
   C 4474-4490 sites).  */
unsafe fn toolkit_menubar_in_use (f: *mut frame) -> bool
{
  #[cfg (feature = "have_ext_menu_bar")]
  {
  return ! (!FRAME_WINDOW_P (f));
  }
  #[cfg (not (feature = "have_ext_menu_bar"))]
  {
  return false;
  }
}

/* Build the part of Lisp event which represents scroll bar state from
   EV.  TYPE is one of Qvertical_scroll_bar or Qhorizontal_scroll_bar.  */

/* C 6116-6122: `make_scroll_bar_position`.  */
unsafe fn make_scroll_bar_position (ev: *mut input_event, r#type: Lisp_Object) -> Lisp_Object
{
  return list5 ((*ev).frame_or_window, r#type, Fcons ((*ev).x, (*ev).y),
		INT_TO_INTEGER ((*ev).timestamp as EMACS_INT),
		builtin_lisp_symbol (scroll_bar_parts[input_event_bits! (part, ev) as usize] as usize));
}

/* C 6124-6145: `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR` — `HAVE_WINDOW_SYSTEM`
   is `#undef` on this target, so C compiles nothing of `coords_in_menu_bar_window`; carried per
   P-51 (the `window_system_no_ext_menu_bar` pairing is the one `xdisp.rs`/`dispnew.rs` use).  */
#[cfg (feature = "window_system_no_ext_menu_bar")]
/* Return whether or not the coordinates X and Y are inside the
   box of the menu-bar window of frame F.  */
unsafe fn coords_in_menu_bar_window (f: *mut frame, x: c_int, y: c_int) -> bool
{
  let mut window: *mut window;

  if !WINDOWP ((*f).menu_bar_window)
    { return false; }

  window = XWINDOW ((*f).menu_bar_window);

  return y >= WINDOW_TOP_EDGE_Y! (window)
	 && x >= WINDOW_LEFT_EDGE_X! (window)
	 && y <= WINDOW_BOTTOM_EDGE_Y! (window)
	 && x <= WINDOW_RIGHT_EDGE_X! (window);
}

/* C 6147-6168: `#ifdef HAVE_WINDOW_SYSTEM` — `#undef` on this target; carried per P-51.  */
#[cfg (feature = "have_window_system")]
/* Return whether or not the coordinates X and Y are inside the
   tab-bar window of the given frame F.  */
unsafe fn coords_in_tab_bar_window (f: *mut frame, x: c_int, y: c_int) -> bool
{
  let mut window: *mut window;

  if !WINDOWP ((*f).tab_bar_window)
    { return false; }

  window = XWINDOW ((*f).tab_bar_window);

  return y >= WINDOW_TOP_EDGE_Y! (window)
	 && x >= WINDOW_LEFT_EDGE_X! (window)
	 && y <= WINDOW_BOTTOM_EDGE_Y! (window)
	 && x <= WINDOW_RIGHT_EDGE_X! (window);
}

/* C 6170-6183: `save_line_number_display_width`.  */
unsafe fn save_line_number_display_width (event: *mut input_event)
{
  let mut w: *mut window;
  /* C's uninitialised `pixel_width` is zero-initialised (PORTING.md 1.5); it is the callee's
     out-parameter.  */
  let mut pixel_width: c_int = 0;

  if WINDOWP ((*event).frame_or_window)
    { w = XWINDOW ((*event).frame_or_window); }
  else if FRAMEP ((*event).frame_or_window)
    { w = XWINDOW ((*XFRAME ((*event).frame_or_window)).selected_window); }
  else
    { w = XWINDOW (selected_window); }
  line_number_display_width (w, &raw mut down_mouse_line_number_width, &raw mut pixel_width);
}

/* Return non-zero if the change of position from START_POS to END_POS
   is likely to be the effect of horizontal scrolling due to a change
   in line-number width produced by redisplay between two mouse
   events, like mouse-down followed by mouse-up, at those positions.
   This is used to decide whether to converts mouse-down followed by
   mouse-up event into a mouse-drag event.  */
/* C 6191-6219: `line_number_mode_hscroll`.  */
unsafe fn line_number_mode_hscroll (start_pos: Lisp_Object, end_pos: Lisp_Object) -> bool
{
  if !EQ (Fcar (start_pos), Fcar (end_pos)) /* different window */
      || list_length (start_pos) < 7	/* no COL/ROW info */
      || list_length (end_pos) < 7
    { return false; }

  let start_col_row: Lisp_Object = Fnth (make_fixnum (6), start_pos);
  let end_col_row: Lisp_Object = Fnth (make_fixnum (6), end_pos);
  let mut window: Lisp_Object = Fcar (end_pos);
  /* C's uninitialised `col_width`/`pixel_width` are zero-initialised (PORTING.md 1.5); they are
     the callee's out-parameters.  */
  let mut col_width: c_int = 0;
  let mut pixel_width: c_int = 0;
  let start_col: Lisp_Object;
  let end_col: Lisp_Object;
  let w: *mut window;
  if !WINDOW_VALID_P (window)
    {
      if WINDOW_LIVE_P (window)
	{ window = (*XFRAME (window)).selected_window; }
      else
	{ window = selected_window; }
    }
  w = XWINDOW (window);
  line_number_display_width (w, &raw mut col_width, &raw mut pixel_width);
  start_col = Fcar (start_col_row);
  end_col = Fcar (end_col_row);
  return EQ (start_col, end_col)
	 && down_mouse_line_number_width >= 0
	 && col_width != down_mouse_line_number_width;
}
/* dispextern.h 3410-3430: `enum tab_bar_item_idx` — `TAB_BAR_ITEM_KEY` … `TAB_BAR_ITEM_NSLOTS`
   — has no mirror, and `make_lispy_event`'s two carried `HAVE_WINDOW_SYSTEM` arms are the only
   text in this unit that names its members, so the type and the seven members stand at that use
   site (P-72③; PORTING.md 1.3's constant shape).  */
pub type tab_bar_item_idx = c_int;
pub const TAB_BAR_ITEM_KEY: tab_bar_item_idx = 0;
pub const TAB_BAR_ITEM_ENABLED_P: tab_bar_item_idx = 1;
pub const TAB_BAR_ITEM_SELECTED_P: tab_bar_item_idx = 2;
pub const TAB_BAR_ITEM_CAPTION: tab_bar_item_idx = 3;
pub const TAB_BAR_ITEM_BINDING: tab_bar_item_idx = 4;
pub const TAB_BAR_ITEM_HELP: tab_bar_item_idx = 5;
pub const TAB_BAR_ITEM_NSLOTS: tab_bar_item_idx = 6;


/* Given a struct input_event, build the lisp event which represents
   it.  If EVENT is 0, build a mouse movement event from the mouse
   movement buffer, which should have a movement event in it.

   Note that events must be passed to this function in the order they
   are received; this function stores the location of button presses
   in order to build drag events when the button is released.  */

/* C 6230-7309: `make_lispy_event`.  C's `switch (event->kind)` is the port's `match` on the
   same arms in the same order (so `switch`/`case`/`default` cannot reappear, the
   `kbd_buffer_get_event` shape); the `#if`-only arms are carried under their `#[cfg]` (P-51),
   and C's `#if 0` block (C 6374-6383, the `XK_kana_A` arm) is dead text nothing carries (the
   C 5468-5503 rule).  C's `NS_NONKEY_EVENT`/`NON_ASCII_KEYSTROKE_EVENT` fall-through is the
   shared body the carried arm spells out under its `#[cfg]` (the C 4311-4313 shape), and C's
   `int i` is one function-scope local both loops reuse.  `#ifdef ISO_FUNCTION_KEY_OFFSET` is
   live — the macro is defined at C 5597 in this file, not by the frozen table.  */
unsafe fn make_lispy_event (event: *mut input_event) -> Lisp_Object
{
  /* C 6230-7309 is one `switch (event->kind)` whose every `case` is an arm below and whose
     `default` is the trailing `_`; C's one `continue` (the touchscreen-update loop's
     menu-bar skip) is the `if` that wraps the rest of that loop's body.  */
  /* C's uninitialised `i` is zero-initialised (PORTING.md 1.5); both loops assign it.  */
  let mut i: c_int = 0;

  match input_event_bits! (kind, event)
    {
    /* C 6237-6252: `#ifdef HAVE_WINDOW_SYSTEM` (carried per P-51).  */
    #[cfg (feature = "have_window_system")]
    DELETE_WINDOW_EVENT => {
      /* Make an event (delete-frame (FRAME)).  */
      return list2 (Qdelete_frame (), list1 ((*event).frame_or_window));
    }

    #[cfg (feature = "have_window_system")]
    ICONIFY_EVENT => {
      /* Make an event (iconify-frame (FRAME)).  */
      return list2 (Qiconify_frame (), list1 ((*event).frame_or_window));
    }

    #[cfg (feature = "have_window_system")]
    DEICONIFY_EVENT => {
      /* Make an event (make-frame-visible (FRAME)).  */
      return list2 (Qmake_frame_visible (), list1 ((*event).frame_or_window));
    }

    #[cfg (feature = "have_window_system")]
    MOVE_FRAME_EVENT => {
      /* Make an event (move-frame (FRAME)).  */
      return list2 (Qmove_frame (), list1 ((*event).frame_or_window));
    }

    /* Just discard these, by returning nil.
       With MULTI_KBOARD, these events are used as placeholders
       when we need to randomly delete events from the queue.
       (They shouldn't otherwise be found in the buffer,
       but on some machines it appears they do show up
       even without MULTI_KBOARD.)  */
    /* On Windows NT/9X, NO_EVENT is used to delete extraneous
       mouse events during a popup-menu call.  */
    /* C 6259-6260.  */
    NO_EVENT => {
      return Qnil ();
    }

    /* C 6262-6275.  */
    HELP_EVENT => {
      let frame: Lisp_Object = (*event).frame_or_window;
      let object: Lisp_Object = (*event).arg;
      let position: Lisp_Object
	= make_fixnum (Time_to_position ((*event).timestamp));
      let mut window: Lisp_Object = (*event).x;
      let help: Lisp_Object = (*event).y;
      clear_event (event);

      if !WINDOWP (window)
	{ window = Qnil (); }
      return Fcons (Qhelp_echo (),
		    list5 (frame, help, window, object, position));
    }

    /* C 6277-6284.  */
    FOCUS_IN_EVENT => {
      return make_lispy_focus_in ((*event).frame_or_window);
    }

    FOCUS_OUT_EVENT => {
      return make_lispy_focus_out ((*event).frame_or_window);
    }

    /* A simple keystroke.  */
    /* C 6286-6326.  */
    ASCII_KEYSTROKE_EVENT | MULTIBYTE_CHAR_KEYSTROKE_EVENT => {
      /* C's uninitialised `lispy_c` is zero-initialised (PORTING.md 1.5); `XSETFASTINT`
	 writes it.  */
      let mut lispy_c: Lisp_Object = Qnil ();
      let mut c: EMACS_INT = (*event).code as EMACS_INT;
      if input_event_bits! (kind, event) == ASCII_KEYSTROKE_EVENT
	{
	  c &= 0o377;
	  eassert! (c == (*event).code as EMACS_INT);
	}

      /* Caps-lock shouldn't affect interpretation of key chords:
	 Control+s should produce C-s whether caps-lock is on or
	 not.  And Control+Shift+s should produce C-S-s whether
	 caps-lock is on or not.  */
      if ((*event).modifiers & !(shift_modifier as c_uint)) != 0
	{
	  /* This is a key chord: some non-shift modifier is
	     depressed.  */

	  if uppercasep (c as c_int)
	      && ((*event).modifiers & shift_modifier as c_uint) == 0
	    {
	      /* Got a capital letter without a shift.  The caps
		 lock is on.   Un-capitalize the letter.  */
	      c = downcase (c as c_int) as EMACS_INT;
	    }
	  else if lowercasep (c as c_int)
		  && ((*event).modifiers & shift_modifier as c_uint) != 0
	    {
	      /* Got a lower-case letter even though shift is
		 depressed.  The caps lock is on.  Capitalize the
		 letter.  */
	      c = upcase (c as c_int) as EMACS_INT;
	    }
	}

      if input_event_bits! (kind, event) == ASCII_KEYSTROKE_EVENT
	{
	  /* Turn ASCII characters into control characters
	     when proper.  */
	  if ((*event).modifiers & ctrl_modifier as c_uint) != 0
	    {
	      c = make_ctrl_char (c as c_int) as EMACS_INT;
	      (*event).modifiers &= !(ctrl_modifier as c_uint);
	    }
	}

      /* Add in the other modifier bits.  The shift key was taken care
	 of by the X code.  */
      c |= ((*event).modifiers
	    & ((meta_modifier | alt_modifier
		| hyper_modifier | super_modifier | ctrl_modifier) as c_uint)) as EMACS_INT;
      /* Distinguish Shift-SPC from SPC.  */
      if (*event).code == 0o40
	  && ((*event).modifiers & shift_modifier as c_uint) != 0
	{ c |= shift_modifier as EMACS_INT; }
      button_down_time = 0;
      XSETFASTINT! (lispy_c, c);
      return lispy_c;
    }

    /* C 6328-6341: `#ifdef HAVE_NS` (carried per P-51).  */
    #[cfg (feature = "have_ns")]
    NS_TEXT_EVENT => {
      return list1 (intern (if (*event).code == KEY_NS_PUT_WORKING_TEXT as c_uint
			      { c"ns-put-working-text".as_ptr () }
			      else { c"ns-unput-working-text".as_ptr () }));
    }

    /* NS_NONKEY_EVENTs are just like NON_ASCII_KEYSTROKE_EVENTs,
       except that they are non-key events (last-nonmenu-event is nil).  */
    /* A function key.  The symbol may need to have modifier prefixes
       tacked onto it.  */
    #[cfg (feature = "have_ns")]
    NS_NONKEY_EVENT => {
      button_down_time = 0;

      i = 0;
      while i < countof! (lispy_accent_codes) as c_int
	{
	  if (*event).code == lispy_accent_codes[i as usize] as c_uint
	    {
	      return modify_event_symbol (i as ptrdiff_t,
					  (*event).modifiers as c_int,
					  Qfunction_key (), Qnil (),
					  &raw const lispy_accent_keys as *const *const c_char,
					  &raw mut accent_key_syms,
					  countof! (lispy_accent_keys) as ptrdiff_t);
	    }
	  i += 1;
	}

      /* C 6382-6389: `#ifdef ISO_FUNCTION_KEY_OFFSET` (live: the macro is defined at C 5597).  */
      if (*event).code < FUNCTION_KEY_OFFSET as c_uint
	  && (*event).code >= ISO_FUNCTION_KEY_OFFSET as c_uint
	{
	  return modify_event_symbol (((*event).code - ISO_FUNCTION_KEY_OFFSET as c_uint)
					as ptrdiff_t,
				      (*event).modifiers as c_int,
				      Qfunction_key (), Qnil (),
				      &raw const iso_lispy_function_keys as *const *const c_char,
				      &raw mut func_key_syms,
				      countof! (iso_lispy_function_keys) as ptrdiff_t);
	}

      if FUNCTION_KEY_OFFSET as c_uint <= (*event).code
	  && (*event).code
	      < FUNCTION_KEY_OFFSET as c_uint + countof! (lispy_function_keys) as c_uint
	  && !lispy_function_keys[((*event).code - FUNCTION_KEY_OFFSET as c_uint) as usize]
	       .is_null ()
	{
	  return modify_event_symbol (((*event).code - FUNCTION_KEY_OFFSET as c_uint)
					as ptrdiff_t,
				      (*event).modifiers as c_int,
				      Qfunction_key (), Qnil (),
				      &raw const lispy_function_keys as *const *const c_char,
				      &raw mut func_key_syms,
				      countof! (lispy_function_keys) as ptrdiff_t);
	}

      /* Handle system-specific or unknown keysyms.
	 We need to use an alist rather than a vector as the cache
	 since we can't make a vector long enough.  */
      if NILP (KVAR! (current_kboard, system_key_syms))
	{ kset_system_key_syms (current_kboard, Fcons (Qnil (), Qnil ())); }
      return modify_event_symbol ((*event).code as ptrdiff_t,
				  (*event).modifiers as c_int,
				  Qfunction_key (),
				  KVAR! (current_kboard, Vsystem_key_alist),
				  std::ptr::null (),
				  &raw mut KVAR! (current_kboard, system_key_syms),
				  PTRDIFF_MAX);
    }

    /* C 6344-6411: the live `NON_ASCII_KEYSTROKE_EVENT` arm.  */
    NON_ASCII_KEYSTROKE_EVENT => {
      button_down_time = 0;

      i = 0;
      while i < countof! (lispy_accent_codes) as c_int
	{
	  if (*event).code == lispy_accent_codes[i as usize] as c_uint
	    {
	      return modify_event_symbol (i as ptrdiff_t,
					  (*event).modifiers as c_int,
					  Qfunction_key (), Qnil (),
					  &raw const lispy_accent_keys as *const *const c_char,
					  &raw mut accent_key_syms,
					  countof! (lispy_accent_keys) as ptrdiff_t);
	    }
	  i += 1;
	}

      /* C 6382-6389: `#ifdef ISO_FUNCTION_KEY_OFFSET` (live: the macro is defined at C 5597).  */
      if (*event).code < FUNCTION_KEY_OFFSET as c_uint
	  && (*event).code >= ISO_FUNCTION_KEY_OFFSET as c_uint
	{
	  return modify_event_symbol (((*event).code - ISO_FUNCTION_KEY_OFFSET as c_uint)
					as ptrdiff_t,
				      (*event).modifiers as c_int,
				      Qfunction_key (), Qnil (),
				      &raw const iso_lispy_function_keys as *const *const c_char,
				      &raw mut func_key_syms,
				      countof! (iso_lispy_function_keys) as ptrdiff_t);
	}

      if FUNCTION_KEY_OFFSET as c_uint <= (*event).code
	  && (*event).code
	      < FUNCTION_KEY_OFFSET as c_uint + countof! (lispy_function_keys) as c_uint
	  && !lispy_function_keys[((*event).code - FUNCTION_KEY_OFFSET as c_uint) as usize]
	       .is_null ()
	{
	  return modify_event_symbol (((*event).code - FUNCTION_KEY_OFFSET as c_uint)
					as ptrdiff_t,
				      (*event).modifiers as c_int,
				      Qfunction_key (), Qnil (),
				      &raw const lispy_function_keys as *const *const c_char,
				      &raw mut func_key_syms,
				      countof! (lispy_function_keys) as ptrdiff_t);
	}

      /* Handle system-specific or unknown keysyms.
	 We need to use an alist rather than a vector as the cache
	 since we can't make a vector long enough.  */
      if NILP (KVAR! (current_kboard, system_key_syms))
	{ kset_system_key_syms (current_kboard, Fcons (Qnil (), Qnil ())); }
      return modify_event_symbol ((*event).code as ptrdiff_t,
				  (*event).modifiers as c_int,
				  Qfunction_key (),
				  KVAR! (current_kboard, Vsystem_key_alist),
				  std::ptr::null (),
				  &raw mut KVAR! (current_kboard, system_key_syms),
				  PTRDIFF_MAX);
    }

    /* C 6413-6439: `#ifdef HAVE_NTGUI` (carried per P-51).  */
    #[cfg (feature = "have_ntgui")]
    END_SESSION_EVENT => {
      /* Make an event (end-session).  */
      return list1 (Qend_session ());
    }

    #[cfg (feature = "have_ntgui")]
    LANGUAGE_CHANGE_EVENT => {
      /* Make an event (language-change FRAME CODEPAGE LANGUAGE-ID).  */
      return list4 (Qlanguage_change (),
		    (*event).frame_or_window,
		    make_fixnum ((*event).code as EMACS_INT),
		    make_fixnum ((*event).modifiers as EMACS_INT));
    }

    #[cfg (feature = "have_ntgui")]
    MULTIMEDIA_KEY_EVENT => {
      if (*event).code < countof! (lispy_multimedia_keys) as c_uint
	  && (*event).code > 0
	  && !lispy_multimedia_keys[(*event).code as usize].is_null ()
	{
	  return modify_event_symbol ((*event).code as ptrdiff_t,
				      (*event).modifiers as c_int,
				      Qfunction_key (), Qnil (),
				      &raw const lispy_multimedia_keys as *const *const c_char,
				      &raw mut func_key_syms,
				      countof! (lispy_multimedia_keys) as ptrdiff_t);
	}
      return Qnil ();
    }

    /* A mouse click.  Figure out where it is, decide whether it's
       a press, click or drag, and build the appropriate structure.  */
    /* C 6441-6751: `case MOUSE_CLICK_EVENT:`, with `#ifndef USE_TOOLKIT_SCROLL_BARS`'s
       `SCROLL_BAR_CLICK_EVENT`/`HORIZONTAL_SCROLL_BAR_CLICK_EVENT` sharing its body (the macro
       is `#undef` on this target, so both labels are live text).  */
    MOUSE_CLICK_EVENT | SCROLL_BAR_CLICK_EVENT | HORIZONTAL_SCROLL_BAR_CLICK_EVENT => {
      let button: c_int = (*event).code as c_int;
      let mut is_double: bool = false;
      let mut position: Lisp_Object = Qnil ();
      let mut start_pos_ptr: *mut Lisp_Object = std::ptr::null_mut ();
      let mut start_pos: Lisp_Object = Qnil ();

      position = Qnil ();

      /* Build the position as appropriate for this mouse click.  */
      if input_event_bits! (kind, event) == MOUSE_CLICK_EVENT
	{
	  let f: *mut frame = XFRAME ((*event).frame_or_window);
	  let mut row: c_int = 0;
	  let mut column: c_int = 0;

	  /* Ignore mouse events that were made on frame that
	     have been deleted.  */
	  if ! FRAME_LIVE_P (f)
	    { return Qnil (); }

	  /* EVENT->x and EVENT->y are frame-relative pixel
	     coordinates at this place.  Under old redisplay, COLUMN
	     and ROW are set to frame relative glyph coordinates
	     which are then used to determine whether this click is
	     in a menu (non-toolkit version).  */
	  /* C 6474-6480: the `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR`
	     clause of the test is dead (P-51), so C's one expression is the call alone.  */
	  if !toolkit_menubar_in_use (f)
	    {
	      /* C 6483-6494: that `#if`'s arm — the `menu_bar_window` branch that the
		 `#endif`'s `else` would have taken — is carried under its `#[cfg]` (P-51;
		 `menu_bar_window` and `FRAME_TO_WINDOW_PIXEL_X`/`_Y` are the mirror gaps
		 P-72③ reports, and the compiled-out text is never name-resolved).  */
	      #[cfg (feature = "window_system_no_ext_menu_bar")]
	      {
	      if FRAME_WINDOW_P (f)
		{
		  let menu_w: *mut window = XWINDOW ((*f).menu_bar_window);
		  x_y_to_column_row
		    (XWINDOW ((*f).menu_bar_window),
		     FRAME_TO_WINDOW_PIXEL_X (menu_w, XFIXNUM ((*event).x)),
		     FRAME_TO_WINDOW_PIXEL_Y (menu_w, XFIXNUM ((*event).y)),
		     &raw mut column, &raw mut row);
		}
	      }
	      /* The live text is the `pixel_to_glyph_coords` call the `#endif` leaves in
		 place.  */
	      pixel_to_glyph_coords (f, XFIXNUM ((*event).x) as c_int,
				     XFIXNUM ((*event).y) as c_int,
				     &raw mut column, &raw mut row,
				     std::ptr::null_mut (), true);

	      /* In the non-toolkit version, clicks on the menu bar
		 are ordinary button events in the event buffer.
		 Distinguish them, and invoke the menu.

		 (In the toolkit version, the toolkit handles the
		 menu bar and Emacs doesn't know about it until
		 after the user makes a selection.)  */
	      if row >= 0 && row < FRAME_MENU_BAR_LINES (f)
		  && ((*event).modifiers & down_modifier as c_uint) != 0
		{
		  let mut items: Lisp_Object = Qnil ();
		  let mut item: Lisp_Object = Qnil ();

		  /* Find the menu bar item under `column'.  */
		  item = Qnil ();
		  items = FRAME_MENU_BAR_ITEMS (f);
		  i = 0;
		  while i < ASIZE (items) as c_int
		    {
		      let mut pos: Lisp_Object = Qnil ();
		      let mut string: Lisp_Object = Qnil ();
		      string = AREF (items, (i + 1) as isize);
		      pos = AREF (items, (i + 3) as isize);
		      if NILP (string)
			{ break; }
		      if (column as ptrdiff_t) >= XFIXNUM (pos) as ptrdiff_t
			  && ((column as ptrdiff_t)
			      < XFIXNUM (pos) as ptrdiff_t + SCHARS (string))
			{
			  item = AREF (items, i as isize);
			  break;
			}
		      i += 4;
		    }

		  /* Don't generate a menu bar event if ITEM is
		     nil.  */
		  if NILP (item)
		    { return Qnil (); }

		  /* Elisp manual 2.4b says (x y) are window
		     relative but code says they are
		     frame-relative.  */
		  position = list4 ((*event).frame_or_window,
				    Qmenu_bar (),
				    Fcons ((*event).x, (*event).y),
				    INT_TO_INTEGER ((*event).timestamp as EMACS_INT));

		  return list2 (item, position);
		}
	    }

	  position = make_lispy_position (f, (*event).x, (*event).y,
					  (*event).timestamp);

	  /* For tab-bar clicks, add the propertized string with
	     button information as OBJECT member of POSITION.  */
	  if CONSP ((*event).arg) && EQ (XCAR ((*event).arg), Qtab_bar ())
	    { position = nconc2 (position, Fcons (XCDR ((*event).arg), Qnil ())); }
	}
      /* C 6707-6711: `#ifndef USE_TOOLKIT_SCROLL_BARS` (live).  */
      else
	/* It's a scrollbar click.  */
	{ position = make_scroll_bar_position (event, Qvertical_scroll_bar ()); }

      if button as ptrdiff_t >= ASIZE (button_down_location)
	{
	  let incr: ptrdiff_t = button as ptrdiff_t - ASIZE (button_down_location) + 1;
	  button_down_location = larger_vector (button_down_location,
						incr, -1);
	  mouse_syms = larger_vector (mouse_syms, incr, -1);
	}

      start_pos_ptr = aref_addr (button_down_location, button as isize);
      start_pos = *start_pos_ptr;
      *start_pos_ptr = Qnil ();

      {
	/* On window-system frames, use the value of
	   double-click-fuzz as is.  On other frames, interpret it
	   as a multiple of 1/8 characters.  */
	let mut f: *mut frame;
	let fuzz: intmax_t;

	if WINDOWP ((*event).frame_or_window)
	  { f = XFRAME ((*XWINDOW ((*event).frame_or_window)).frame); }
	else if FRAMEP ((*event).frame_or_window)
	  { f = XFRAME ((*event).frame_or_window); }
	else
	  { emacs_abort (); }

	if FRAME_WINDOW_P (f)
	  { fuzz = double_click_fuzz as intmax_t; }
	else
	  { fuzz = double_click_fuzz as intmax_t / 8; }

	is_double = (button == last_mouse_button
		     && (eabs! (XFIXNUM ((*event).x) - last_mouse_x as EMACS_INT)
			 as intmax_t) <= fuzz
		     && (eabs! (XFIXNUM ((*event).y) - last_mouse_y as EMACS_INT)
			 as intmax_t) <= fuzz
		     && button_down_time != 0
		     && (EQ (Vdouble_click_time, Qt ())
			 || (FIXNATP (Vdouble_click_time)
			     && ((*event).timestamp - button_down_time
				 < XFIXNAT (Vdouble_click_time) as Time))));
      }

      last_mouse_button = button;
      last_mouse_x = XFIXNUM ((*event).x) as c_int;
      last_mouse_y = XFIXNUM ((*event).y) as c_int;

      /* If this is a button press, squirrel away the location, so
         we can decide later whether it was a click or a drag.  */
      if ((*event).modifiers & down_modifier as c_uint) != 0
	{
	  if is_double
	    {
	      double_click_count += 1;
	      (*event).modifiers |= (if double_click_count > 2
				     { triple_modifier }
				     else { double_modifier }) as c_uint;
	    }
	  else
	    { double_click_count = 1; }
	  button_down_time = (*event).timestamp;
	  *start_pos_ptr = Fcopy_alist (position);
	  frame_relative_event_pos = Fcons ((*event).x, (*event).y);
	  ignore_mouse_drag_p = false;
	  /* Squirrel away the line-number width, if any.  */
	  save_line_number_display_width (event);
	}

      /* Now we're releasing a button - check the coordinates to
         see if this was a click or a drag.  */
      else if ((*event).modifiers & up_modifier as c_uint) != 0
	{
	  /* If we did not see a down before this up, ignore the up.
	     Probably this happened because the down event chose a
	     menu item.  It would be an annoyance to treat the
	     release of the button that chose the menu item as a
	     separate event.  */

	  if !CONSP (start_pos)
	    { return Qnil (); }

	  let mut click_or_drag_modifier: c_uint = click_modifier as c_uint;

	  if ignore_mouse_drag_p
	    { ignore_mouse_drag_p = false; }
	  else
	    {
	      let mut xdiff: intmax_t = double_click_fuzz as intmax_t;
	      let mut ydiff: intmax_t = double_click_fuzz as intmax_t;

	      xdiff = (XFIXNUM ((*event).x)
		       - XFIXNUM (XCAR (frame_relative_event_pos))) as intmax_t;
	      ydiff = (XFIXNUM ((*event).y)
		       - XFIXNUM (XCDR (frame_relative_event_pos))) as intmax_t;

	      if ! (0 < double_click_fuzz as intmax_t
		    && (- double_click_fuzz as intmax_t) < xdiff
		    && xdiff < double_click_fuzz as intmax_t
		    && (- double_click_fuzz as intmax_t) < ydiff
		    && ydiff < double_click_fuzz as intmax_t
		    /* Maybe the mouse has moved a lot, caused scrolling, and
		       eventually ended up at the same screen position (but
		       not buffer position) in which case it is a drag, not
		       a click.  */
		    /* FIXME: OTOH if the buffer position has changed
		       because of a timer or process filter rather than
		       because of mouse movement, it should be considered as
		       a click.  But mouse-drag-region completely ignores
		       this case and it hasn't caused any real problem, so
		       it's probably OK to ignore it as well.  */
		    && (EQ (Fcar (Fcdr (start_pos)),
			    Fcar (Fcdr (position))) /* Same buffer pos */
			/* Redisplay hscrolled text between down- and
			   up-events due to display-line-numbers-mode.  */
			|| line_number_mode_hscroll (start_pos, position)
			|| !EQ (Fcar (start_pos),
				Fcar (position)))) /* Different window */
		{
	      /* Mouse has moved enough.  */
	      button_down_time = 0;
	      click_or_drag_modifier = drag_modifier as c_uint;
	      /* Reset the value for future clicks.  */
	      down_mouse_line_number_width = -1;
	    }
	      else if (((!EQ (Fcar (start_pos), Fcar (position)))
			|| (!EQ (Fcar (Fcdr (start_pos)),
				 Fcar (Fcdr (position)))))
		       /* Was the down event in a window body? */
		       && FIXNUMP (Fcar (Fcdr (start_pos)))
		       && WINDOW_LIVE_P (Fcar (start_pos))
		       && !NILP (Ffboundp (Qwindow_edges ())))
		/* If the window (etc.) at the mouse position has
		   changed between the down event and the up event,
		   we assume there's been a redisplay between the
		   two events, and we pretend the mouse is still in
		   the old window to prevent a spurious drag event
		   being generated.  */
		{
		  let edges: Lisp_Object
		    = calln! (Qwindow_edges (), Fcar (start_pos), Qt (), Qnil (), Qt ());
		  let mut new_x: c_int = XFIXNUM (Fcar (frame_relative_event_pos)) as c_int;
		  let mut new_y: c_int = XFIXNUM (XCDR (frame_relative_event_pos)) as c_int;

		  /* If the up-event is outside the down-event's
		     window, use coordinates that are within it.  */
		  if new_x < XFIXNUM (Fcar (edges)) as c_int
		    { new_x = XFIXNUM (Fcar (edges)) as c_int; }
		  else if new_x >= XFIXNUM (Fcar (Fcdr (Fcdr (edges)))) as c_int
		    { new_x = XFIXNUM (Fcar (Fcdr (Fcdr (edges)))) as c_int - 1; }
		  if new_y < XFIXNUM (Fcar (Fcdr (edges))) as c_int
		    { new_y = XFIXNUM (Fcar (Fcdr (edges))) as c_int; }
		  else if new_y
			  >= XFIXNUM (Fcar (Fcdr (Fcdr (Fcdr (edges))))) as c_int
		    { new_y = XFIXNUM (Fcar (Fcdr (Fcdr (Fcdr (edges))))) as c_int - 1; }

		  position = make_lispy_position
		    (XFRAME ((*event).frame_or_window),
		     make_fixnum (new_x as EMACS_INT), make_fixnum (new_y as EMACS_INT),
		     (*event).timestamp);
		}
	    }

	  /* Don't check is_double; treat this as multiple if the
	     down-event was multiple.  */
	  (*event).modifiers
	    = ((*event).modifiers & !(up_modifier as c_uint))
	      | click_or_drag_modifier
	      | (if double_click_count < 2
		 { 0 } else if double_click_count == 2
		 { double_modifier } else { triple_modifier }) as c_uint;
	}
      else
	/* Every mouse event should either have the down_modifier or
           the up_modifier set.  */
	{ emacs_abort (); }

      {
	/* Get the symbol we should use for the mouse click.  */
	let mut head: Lisp_Object = Qnil ();

	head = modify_event_symbol (button as ptrdiff_t,
				    (*event).modifiers as c_int,
				    Qmouse_click (), Vlispy_mouse_stem,
				    std::ptr::null (),
				    &raw mut mouse_syms,
				    ASIZE (mouse_syms));
	if ((*event).modifiers & drag_modifier as c_uint) != 0
	  { return list3 (head, start_pos, position); }
	else if ((*event).modifiers
		 & (double_modifier | triple_modifier) as c_uint) != 0
	  { return list3 (head, position, make_fixnum (double_click_count as EMACS_INT)); }
	else
	  { return list2 (head, position); }
      }
    }

    /* C 6752-6900: `case WHEEL_EVENT:` and `case HORIZ_WHEEL_EVENT:`.  */
    WHEEL_EVENT | HORIZ_WHEEL_EVENT => {
      let mut position: Lisp_Object;
      let mut head: Lisp_Object;

      /* Build the position as appropriate for this mouse click.  */
      let f: *mut frame = XFRAME ((*event).frame_or_window);

      /* Ignore wheel events that were made on frame that have been
	 deleted.  */
      if ! FRAME_LIVE_P (f)
	{ return Qnil (); }

      position = make_lispy_position (f, (*event).x, (*event).y,
				      (*event).timestamp);

      /* Set double or triple modifiers to indicate the wheel speed.  */
      {
	/* On window-system frames, use the value of
	   double-click-fuzz as is.  On other frames, interpret it
	   as a multiple of 1/8 characters.  */
	let mut fr: *mut frame;
	let fuzz: intmax_t;
	let mut symbol_num: c_int;
	let mut is_double: bool;

	if WINDOWP ((*event).frame_or_window)
	  { fr = XFRAME ((*XWINDOW ((*event).frame_or_window)).frame); }
	else if FRAMEP ((*event).frame_or_window)
	  { fr = XFRAME ((*event).frame_or_window); }
	else
	  { emacs_abort (); }

	fuzz = if FRAME_WINDOW_P (fr)
		 { double_click_fuzz as intmax_t }
	       else { double_click_fuzz as intmax_t / 8 };

	if ((*event).modifiers & up_modifier as c_uint) != 0
	  {
	    /* Emit a wheel-up event.  */
	    (*event).modifiers &= !(up_modifier as c_uint);
	    symbol_num = 0;
	  }
	else if ((*event).modifiers & down_modifier as c_uint) != 0
	  {
	    /* Emit a wheel-down event.  */
	    (*event).modifiers &= !(down_modifier as c_uint);
	    symbol_num = 1;
	  }
	else
	  /* Every wheel event should either have the down_modifier or
	     the up_modifier set.  */
	  { emacs_abort (); }

	if input_event_bits! (kind, event) == HORIZ_WHEEL_EVENT
	  { symbol_num += 2; }

	is_double = (last_mouse_button == - (1 + symbol_num)
		     && (eabs! (XFIXNUM ((*event).x) - last_mouse_x as EMACS_INT)
			 as intmax_t) <= fuzz
		     && (eabs! (XFIXNUM ((*event).y) - last_mouse_y as EMACS_INT)
			 as intmax_t) <= fuzz
		     && button_down_time != 0
		     && (EQ (Vdouble_click_time, Qt ())
			 || (FIXNATP (Vdouble_click_time)
			     && ((*event).timestamp - button_down_time
				 < XFIXNAT (Vdouble_click_time) as Time))));
	if is_double
	  {
	    double_click_count += 1;
	    (*event).modifiers |= (if double_click_count > 2
				   { triple_modifier }
				   else { double_modifier }) as c_uint;
	  }
	else
	  {
	    double_click_count = 1;
	    (*event).modifiers |= click_modifier as c_uint;
	  }

	button_down_time = (*event).timestamp;
	/* Use a negative value to distinguish wheel from mouse button.  */
	last_mouse_button = - (1 + symbol_num);
	last_mouse_x = XFIXNUM ((*event).x) as c_int;
	last_mouse_y = XFIXNUM ((*event).y) as c_int;

	/* Get the symbol we should use for the wheel event.  */
	head = modify_event_symbol (symbol_num as ptrdiff_t,
				    (*event).modifiers as c_int,
				    Qmouse_click (),
				    Qnil (),
				    &raw const lispy_wheel_names as *const *const c_char,
				    &raw mut wheel_syms,
				    ASIZE (wheel_syms));
      }

      if CONSP ((*event).arg)
	{ return list5 (head, position, make_fixnum (double_click_count as EMACS_INT),
			XCAR ((*event).arg),
			/* FIXME: When a mouse-click on a tab-bar is
                           converted into a wheel-event we get here something
                           of an unexpected shape...  */
			if CONSP (XCDR ((*event).arg))
			    && CONSP (XCDR (XCDR ((*event).arg)))
			{ Fcons (XCAR (XCDR ((*event).arg)),
				 XCAR (XCDR (XCDR ((*event).arg)))) }
			/* ... not knowing what this "unexpected shape" means,
			   we just use nil.  */
			else { Qnil () }); }
      else if NUMBERP ((*event).arg)
	{ return list4 (head, position, make_fixnum (double_click_count as EMACS_INT),
			(*event).arg); }
      else if ((*event).modifiers
	       & (double_modifier | triple_modifier) as c_uint) != 0
	{ return list3 (head, position, make_fixnum (double_click_count as EMACS_INT)); }
      else
	{ return list2 (head, position); }
    }

    /* C 6902-6916: `case TOUCH_END_EVENT:`.  */
    TOUCH_END_EVENT => {
      let mut position: Lisp_Object;

      /* Build the position as appropriate for this mouse click.  */
      let f: *mut frame = XFRAME ((*event).frame_or_window);

      if ! FRAME_LIVE_P (f)
	{ return Qnil (); }

      position = make_lispy_position (f, (*event).x, (*event).y,
				      (*event).timestamp);

      return list2 (Qtouch_end (), position);
    }

    /* C 6918-6978: `case TOUCHSCREEN_BEGIN_EVENT:`.  */
    TOUCHSCREEN_BEGIN_EVENT => {
      let mut x: Lisp_Object;
      let mut y: Lisp_Object;
      let mut id: Lisp_Object;
      let mut position: Lisp_Object;
      let f: *mut frame;
      /* C 6919-6922: `#ifdef HAVE_WINDOW_SYSTEM`'s two locals (carried per P-51).  */
      #[cfg (feature = "have_window_system")]
      let mut tab_bar_item: c_int = 0;
      #[cfg (feature = "have_window_system")]
      let mut close: bool = false;

      f = XFRAME ((*event).frame_or_window);

      if !FRAME_LIVE_P (f)
	{ return Qnil (); }

      id = (*event).arg;
      x = (*event).x;
      y = (*event).y;

      /* C 6924-6932: `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR` (dead,
	 carried per P-51).  */
      #[cfg (feature = "window_system_no_ext_menu_bar")]
      {
      if coords_in_menu_bar_window (f, XFIXNUM (x) as c_int, XFIXNUM (y) as c_int)
	{
	  /* If the tap began in the menu bar window, then save the
	     id.  */
	  menu_bar_touch_id = id;
	  return Qnil ();
	}
      }

      position = make_lispy_position (f, x, y, (*event).timestamp);

      /* C 6934-6975: `#ifdef HAVE_WINDOW_SYSTEM` (dead, carried per P-51).  */
      #[cfg (feature = "have_window_system")]
      {
      /* Now check if POSITION lies on the tab bar.  If so, look up
	 the corresponding tab bar item's propertized string as the
	 OBJECT.  */

      if coords_in_tab_bar_window (f, XFIXNUM ((*event).x) as c_int,
				   XFIXNUM ((*event).y) as c_int)
	  /* `get_tab_bar_item_kbd' returns 0 if the item was
	     previously highlighted, 1 otherwise, and -1 if there is
	     no tab bar item.  */
	  && get_tab_bar_item_kbd (f, XFIXNUM ((*event).x) as c_int,
				   XFIXNUM ((*event).y) as c_int, &raw mut tab_bar_item,
				   &raw mut close) >= 0
	{
	  /* First, obtain the propertized string.  */
	  x = Fcopy_sequence (AREF ((*f).tab_bar_items,
				    (tab_bar_item
				     + TAB_BAR_ITEM_CAPTION) as isize));

	  /* Next, add the key binding.  */
	  AUTO_LIST2! (y, Qmenu_item (), list3 (AREF ((*f).tab_bar_items,
						      (tab_bar_item
						       + TAB_BAR_ITEM_KEY) as isize),
						AREF ((*f).tab_bar_items,
						      (tab_bar_item
						       + TAB_BAR_ITEM_BINDING) as isize),
						if close { Qt () } else { Qnil () }));

	  /* And add the new properties to the propertized string.  */
	  Fadd_text_properties (make_fixnum (0),
				make_fixnum (SCHARS (x)),
				y, x);

	  /* Set the position to 0.  */
	  x = Fcons (x, make_fixnum (0));

	  /* Finally, add the OBJECT.  */
	  position = nconc2 (position, Fcons (x, Qnil ()));
	}
      }

      return list2 (Qtouchscreen_begin (),
		    Fcons (id, position));
    }

    /* C 6980-7100: `case TOUCHSCREEN_END_EVENT:`.  */
    TOUCHSCREEN_END_EVENT => {
      let mut x: Lisp_Object;
      let mut y: Lisp_Object;
      let mut id: Lisp_Object;
      let mut position: Lisp_Object;
      let f: *mut frame = XFRAME ((*event).frame_or_window);
      /* C 6983-6986: `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR`'s two
	 locals (carried per P-51).  */
      #[cfg (feature = "window_system_no_ext_menu_bar")]
      let mut column: c_int = 0;
      #[cfg (feature = "window_system_no_ext_menu_bar")]
      let mut row: c_int = 0;
      /* C 6987-6991: `#ifdef HAVE_WINDOW_SYSTEM`'s two locals (carried per P-51).  */
      #[cfg (feature = "have_window_system")]
      let mut tab_bar_item: c_int = 0;
      #[cfg (feature = "have_window_system")]
      let mut close: bool = false;

      if !FRAME_LIVE_P (f)
	{ return Qnil (); }

      id = (*event).arg;
      x = (*event).x;
      y = (*event).y;

      /* C 6997-7048: `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR` (dead,
	 carried per P-51).  */
      #[cfg (feature = "window_system_no_ext_menu_bar")]
      {
      if EQ (menu_bar_touch_id, id)
	{
	  /* This touch should activate the menu bar.  Generate the
	     menu bar event.  */
	  menu_bar_touch_id = Qnil ();

	  if !NILP ((*f).menu_bar_window)
	    {
	      x_y_to_column_row (XWINDOW ((*f).menu_bar_window), XFIXNUM (x) as c_int,
				 XFIXNUM (y) as c_int, &raw mut column, &raw mut row);

	      if row >= 0 && row < FRAME_MENU_BAR_LINES (f)
		{
		  let mut items: Lisp_Object = Qnil ();
		  let mut item: Lisp_Object = Qnil ();

		  /* Find the menu bar item under `column'.  */
		  item = Qnil ();
		  items = FRAME_MENU_BAR_ITEMS (f);
		  i = 0;
		  while i < ASIZE (items) as c_int
		    {
		      let mut pos: Lisp_Object = Qnil ();
		      let mut string: Lisp_Object = Qnil ();
		      string = AREF (items, (i + 1) as isize);
		      pos = AREF (items, (i + 3) as isize);
		      if NILP (string)
			{ break; }
		      if (column as ptrdiff_t) >= XFIXNUM (pos) as ptrdiff_t
			  && ((column as ptrdiff_t)
			      < XFIXNUM (pos) as ptrdiff_t + SCHARS (string))
			{
			  item = AREF (items, i as isize);
			  break;
			}
		      i += 4;
		    }

		  /* Don't generate a menu bar event if ITEM is
		     nil.  */
		  if NILP (item)
		    { return Qnil (); }

		  /* Elisp manual 2.4b says (x y) are window
		     relative but code says they are
		     frame-relative.  */
		  position = list4 ((*event).frame_or_window,
				    Qmenu_bar (),
				    Fcons ((*event).x, (*event).y),
				    INT_TO_INTEGER ((*event).timestamp as EMACS_INT));

		  return list2 (item, position);
		}
	    }

	  return Qnil ();
	}
      }

      position = make_lispy_position (f, x, y, (*event).timestamp);

      /* C 7052-7082: `#ifdef HAVE_WINDOW_SYSTEM` (dead, carried per P-51).  */
      #[cfg (feature = "have_window_system")]
      {
      /* Now check if POSITION lies on the tab bar.  If so, look up
	 the corresponding tab bar item's propertized string as the
	 OBJECT.  */

      if coords_in_tab_bar_window (f, XFIXNUM ((*event).x) as c_int,
				   XFIXNUM ((*event).y) as c_int)
	  /* `get_tab_bar_item_kbd' returns 0 if the item was
	     previously highlighted, 1 otherwise, and -1 if there is
	     no tab bar item.  */
	  && get_tab_bar_item_kbd (f, XFIXNUM ((*event).x) as c_int,
				   XFIXNUM ((*event).y) as c_int, &raw mut tab_bar_item,
				   &raw mut close) >= 0
	{
	  /* First, obtain the propertized string.  */
	  x = Fcopy_sequence (AREF ((*f).tab_bar_items,
				    (tab_bar_item
				     + TAB_BAR_ITEM_CAPTION) as isize));

	  /* Next, add the key binding.  */
	  AUTO_LIST2! (y, Qmenu_item (), list3 (AREF ((*f).tab_bar_items,
						      (tab_bar_item
						       + TAB_BAR_ITEM_KEY) as isize),
						AREF ((*f).tab_bar_items,
						      (tab_bar_item
						       + TAB_BAR_ITEM_BINDING) as isize),
						if close { Qt () } else { Qnil () }));

	  /* And add the new properties to the propertized string.  */
	  Fadd_text_properties (make_fixnum (0),
				make_fixnum (SCHARS (x)),
				y, x);

	  /* Set the position to 0.  */
	  x = Fcons (x, make_fixnum (0));

	  /* Finally, add the OBJECT.  */
	  position = nconc2 (position, Fcons (x, Qnil ()));
	}
      }

      position = make_lispy_position (f, x, y, (*event).timestamp);

      return list3 (Qtouchscreen_end (), Fcons (id, position),
		    if (*event).modifiers != 0 { Qt () } else { Qnil () });
    }

    /* C 7102-7118: `case PINCH_EVENT:`.  */
    PINCH_EVENT => {
      let x: Lisp_Object;
      let y: Lisp_Object;
      let mut position: Lisp_Object;
      let f: *mut frame = XFRAME ((*event).frame_or_window);

      x = (*event).x;
      y = (*event).y;

      position = make_lispy_position (f, x, y, (*event).timestamp);

      /* C's compound literal `(const char *[]) {"pinch"}` has block scope, so the port keeps
	 a local array of the same shape.  */
      let pinch_names: [*const c_char; 1] = [c"pinch".as_ptr ()];
      return Fcons (modify_event_symbol (0, (*event).modifiers as c_int, Qpinch (),
					 Qnil (), &raw const pinch_names as *const *const c_char,
					 &raw mut pinch_syms, 1),
		    Fcons (position, (*event).arg));
    }

    /* C 7120-7147: `case TOUCHSCREEN_UPDATE_EVENT:`.  */
    TOUCHSCREEN_UPDATE_EVENT => {
      let mut x: Lisp_Object;
      let mut y: Lisp_Object;
      let mut id: Lisp_Object;
      let mut position: Lisp_Object;
      let mut tem: Lisp_Object;
      let mut it: Lisp_Object;
      let mut evt: Lisp_Object;
      let f: *mut frame = XFRAME ((*event).frame_or_window);
      evt = Qnil ();

      if !FRAME_LIVE_P (f)
	{ return Qnil (); }

      /* C's `continue` becomes the `if`'s condition, since the loop's increment runs after
	 the body on every path (the buffer.rs shape).  */
      tem = (*event).arg;
      while CONSP (tem)
	{
	  it = XCAR (tem);

	  x = XCAR (it);
	  y = XCAR (XCDR (it));
	  id = XCAR (XCDR (XCDR (it)));

	  /* Don't report touches to the menu bar.  */
	  if !EQ (id, menu_bar_touch_id)
	    {
	      position = make_lispy_position (f, x, y, (*event).timestamp);
	      evt = Fcons (Fcons (id, position), evt);
	    }
	  tem = XCDR (tem);
	}

      if NILP (evt)
	/* Don't return an event if the touchpoint list is
	   empty.  */
	{ return Qnil (); }

      return list2 (Qtouchscreen_update (), evt);
    }

    /* We don't have down and up events if using toolkit scroll bars,
       so make this always a click event.  Store in the `part' of
       the Lisp event a symbol which maps to the following actions:

       `above_handle'		page up
       `below_handle'		page down
       `up'			line up
       `down'			line down
       `top'			top of buffer
       `bottom'			bottom of buffer
       `handle'			thumb has been dragged.
       `end-scroll'		end of interaction with scroll bar

       The incoming input_event contains in its `part' member an
       index of type `enum scroll_bar_part' which we can use as an
       index in scroll_bar_parts to get the appropriate symbol.  */
    /* C 7149-7205: `#ifdef USE_TOOLKIT_SCROLL_BARS` (dead, carried per P-51).  */
    #[cfg (feature = "use_toolkit_scroll_bars")]
    SCROLL_BAR_CLICK_EVENT => {
      let mut position: Lisp_Object;
      let mut head: Lisp_Object;

      position = make_scroll_bar_position (event, Qvertical_scroll_bar);

      /* Always treat scroll bar events as clicks.  */
      (*event).modifiers |= click_modifier as c_uint;
      (*event).modifiers &= !(up_modifier as c_uint);

      if (*event).code >= ASIZE (mouse_syms) as c_uint
	{
	  mouse_syms = larger_vector (mouse_syms,
				      ((*event).code as ptrdiff_t
				       - ASIZE (mouse_syms) + 1),
				      -1);
	}

      /* Get the symbol we should use for the mouse click.  */
      head = modify_event_symbol ((*event).code as ptrdiff_t,
				  (*event).modifiers as c_int,
				  Qmouse_click (),
				  Vlispy_mouse_stem,
				  std::ptr::null (), &raw mut mouse_syms,
				  ASIZE (mouse_syms));
      return list2 (head, position);
    }

    #[cfg (feature = "use_toolkit_scroll_bars")]
    HORIZONTAL_SCROLL_BAR_CLICK_EVENT => {
      let mut position: Lisp_Object;
      let mut head: Lisp_Object;

      position = make_scroll_bar_position (event, Qhorizontal_scroll_bar);

      /* Always treat scroll bar events as clicks.  */
      (*event).modifiers |= click_modifier as c_uint;
      (*event).modifiers &= !(up_modifier as c_uint);

      if (*event).code >= ASIZE (mouse_syms) as c_uint
	{
	  mouse_syms = larger_vector (mouse_syms,
				      ((*event).code as ptrdiff_t
				       - ASIZE (mouse_syms) + 1),
				      -1);
	}

      /* Get the symbol we should use for the mouse click.  */
      head = modify_event_symbol ((*event).code as ptrdiff_t,
				  (*event).modifiers as c_int,
				  Qmouse_click (),
				  Vlispy_mouse_stem,
				  std::ptr::null (), &raw mut mouse_syms,
				  ASIZE (mouse_syms));
      return list2 (head, position);
    }

    /* C 7217-7259: `case DRAG_N_DROP_EVENT:`.  */
    DRAG_N_DROP_EVENT => {
      let mut head: Lisp_Object;
      let mut position: Lisp_Object;
      let files: Lisp_Object;

      let f: *mut frame = XFRAME ((*event).frame_or_window);
      files = (*event).arg;

      /* Ignore mouse events that were made on frames that
	 have been deleted.  */
      if ! FRAME_LIVE_P (f)
	{ return Qnil (); }

      position = make_lispy_position (f, (*event).x, (*event).y,
				      (*event).timestamp);

      head = modify_event_symbol (0, (*event).modifiers as c_int,
				  Qdrag_n_drop (), Qnil (),
				  &raw const lispy_drag_n_drop_names as *const *const c_char,
				  &raw mut drag_n_drop_syms, 1);
      return list3 (head, position, files);
    }

    /* C 7261-7268: `#ifdef HAVE_EXT_MENU_BAR` (dead, carried per P-51).  */
    #[cfg (feature = "have_ext_menu_bar")]
    MENU_BAR_EVENT => {
      if EQ ((*event).arg, (*event).frame_or_window)
	/* This is the prefix key.  We translate this to
	   `(menu_bar)' because the code in keyboard.c for menu
	   events, which we use, relies on this.  */
	{ return list1 (Qmenu_bar ()); }
      return (*event).arg;
    }

    /* C 7270-7274: `case SELECT_WINDOW_EVENT:`.  */
    SELECT_WINDOW_EVENT => {
      /* Make an event (select-window (WINDOW)).  */
      return list2 (Qselect_window (), list1 ((*event).frame_or_window));
    }

    /* C 7276-7284: `case TAB_BAR_EVENT:` and `case TOOL_BAR_EVENT:`.  */
    TAB_BAR_EVENT | TOOL_BAR_EVENT => {
      let mut res: Lisp_Object = (*event).arg;
      let location: Lisp_Object
	= if input_event_bits! (kind, event) == TAB_BAR_EVENT
	  { Qtab_bar () } else { Qtool_bar () };
      if SYMBOLP (res) { res = apply_modifiers ((*event).modifiers as c_int, res); }
      return list2 (res, list2 ((*event).frame_or_window, location));
    }

    /* C 7286-7292: `case USER_SIGNAL_EVENT:`.  */
    USER_SIGNAL_EVENT => {
      /* A user signal.  */
      let name: *mut c_char = find_user_signal_name ((*event).code as c_int);
      if name.is_null ()
	{ emacs_abort (); }
      return intern (name);
    }

    /* C 7294-7295: `case SAVE_SESSION_EVENT:`.  */
    SAVE_SESSION_EVENT => {
      return list2 (Qsave_session (), (*event).arg);
    }

    /* C 7297-7300: `#ifdef HAVE_DBUS` (dead, carried per P-51).  */
    #[cfg (feature = "have_dbus")]
    DBUS_EVENT => {
      return Fcons (Qdbus_event (), (*event).arg);
    }

    /* C 7302-7305: `#ifdef THREADS_ENABLED` (dead, carried per P-51).  */
    #[cfg (feature = "threads_enabled")]
    THREAD_EVENT => {
      return Fcons (Qthread_event (), (*event).arg);
    }

    /* C 7307-7315: `#ifdef HAVE_XWIDGETS` (dead, carried per P-51).  */
    #[cfg (feature = "have_xwidgets")]
    XWIDGET_EVENT => {
      return Fcons (Qxwidget_event (), (*event).arg);
    }

    #[cfg (feature = "have_xwidgets")]
    XWIDGET_DISPLAY_EVENT => {
      return Fcons (Qxwidget_display_event (), (*event).arg);
    }

    /* C 7317-7324: `#ifdef USE_FILE_NOTIFY` (dead, carried per P-51).  */
    #[cfg (feature = "use_file_notify")]
    FILE_NOTIFY_EVENT => {
      /* Make an event (file-notify (DESCRIPTOR ACTION FILE) CALLBACK).  */
      if !NILP ((*event).frame_or_window)  /* HAVE_W32NOTIFY */
	{ return list3 (Qfile_notify (), (*event).arg, (*event).frame_or_window); }
      else
	{ return Fcons (Qfile_notify (), (*event).arg); }
    }

    /* C 7326-7327: `case SLEEP_EVENT:`.  */
    SLEEP_EVENT => {
      return Fcons (Qsleep_event (), (*event).arg);
    }

    /* C 7329-7331: `case CONFIG_CHANGED_EVENT:`.  */
    CONFIG_CHANGED_EVENT => {
      return list3 (Qconfig_changed_event (),
		    (*event).arg, (*event).frame_or_window);
    }

    /* C 7333-7334: `case PREEDIT_TEXT_EVENT:`.  */
    PREEDIT_TEXT_EVENT => {
      return list2 (Qpreedit_text (), (*event).arg);
    }

    /* The 'kind' field of the event is something we don't recognize.  */
    /* C 7336-7338: C's `default:`.  */
    _ => {
      emacs_abort ();
    }
    }
}

/* C 7309-7333: `make_lispy_movement`.  */
unsafe fn make_lispy_movement (frame: *mut frame, bar_window: Lisp_Object,
			       part: scroll_bar_part, x: Lisp_Object, y: Lisp_Object, t: Time)
  -> Lisp_Object
{
  /* Is it a scroll bar movement?  */
  if !frame.is_null () && !NILP (bar_window)
    {
      let part_sym: Lisp_Object;

      part_sym = builtin_lisp_symbol (scroll_bar_parts[part as usize] as usize);
      return list2 (Qscroll_bar_movement (),
		    list5 (bar_window,
			   Qvertical_scroll_bar (),
			   Fcons (x, y),
			   make_fixnum (t as EMACS_INT),
			   part_sym));
    }
  /* Or is it an ordinary mouse movement?  */
  else
    {
      let position: Lisp_Object;
      position = make_lispy_position (frame, x, y, t);
      return list2 (Qmouse_movement (), position);
    }
}

/* Construct a switch frame event.  */
/* C 7335-7340: `make_lispy_switch_frame`.  */
unsafe fn make_lispy_switch_frame (frame: Lisp_Object) -> Lisp_Object
{
  return list2 (Qswitch_frame (), frame);
}

/* C 7342-7346: `make_lispy_focus_in`.  */
unsafe fn make_lispy_focus_in (frame: Lisp_Object) -> Lisp_Object
{
  return list2 (Qfocus_in (), frame);
}

/* C 7348-7352: `make_lispy_focus_out`.  */
unsafe fn make_lispy_focus_out (frame: Lisp_Object) -> Lisp_Object
{
  return list2 (Qfocus_out (), frame);
}

/* Manipulating modifiers.  */

/* C 7356-7363: `parse_modifiers_uncached`'s own comment, above its definition.  */
/* C 7365-7476: `static int parse_modifiers_uncached (Lisp_Object symbol, ptrdiff_t *modifier_end);`  */
unsafe fn parse_modifiers_uncached (symbol: Lisp_Object, modifier_end: *mut ptrdiff_t) -> c_int
{
  let mut name: Lisp_Object;
  let mut i: ptrdiff_t;
  let mut modifiers: c_int;

  /* C 7390-7400 and 7402-7413: `SINGLE_LETTER_MOD` and `MULTI_LETTER_MOD` are C's
     unhygienic macros: their bodies read the enclosing `i`, `name`, `this_mod_end` and
     `this_mod`.  Rust's `macro_rules!` binds such names where the macro is *defined*, not
     where it is invoked, so the port passes each one as a metavariable named after the C
     identifier it carries: every identifier the two C bodies mention stays in the port's
     bodies, the call sites below spell C's invocations, and C's trailing `#undef` has no
     counterpart (Rust's textual scope ends with the enclosing block).  */
  macro_rules! SINGLE_LETTER_MOD {
    ($this_mod_end:ident, $this_mod:ident, $bit:expr, $i:ident) => {
      { $this_mod_end = $i + 1; $this_mod = $bit; }
    };
  }
  macro_rules! MULTI_LETTER_MOD {
    ($this_mod_end:ident, $this_mod:ident, $bit:expr, $name:expr, $word:expr, $len:expr,
     $i:ident) => {
      if $i + $len + 1 <= SBYTES ($name)
	&& memcmp (SDATA ($name).offset ($i) as *const c_void,
		   $word.as_ptr () as *const c_void, $len as usize) == 0
	{ $this_mod_end = $i + $len; $this_mod = $bit; }
    };
  }

  CHECK_SYMBOL (symbol);

  modifiers = 0;
  name = SYMBOL_NAME (symbol);

  i = 0;
  while i < SBYTES (name) - 1
    {
      let mut this_mod_end: ptrdiff_t = 0;
      let mut this_mod: c_int = 0;

      /* See if the name continues with a modifier word.
	 Check that the word appears, but don't check what follows it.
	 Set this_mod and this_mod_end to record what we find.  */

      match SREF (name, i as usize)
	{
	  b'A' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, alt_modifier, i); }
	  b'C' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, ctrl_modifier, i); }
	  b'H' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, hyper_modifier, i); }
	  b'M' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, meta_modifier, i); }
	  b'S' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, shift_modifier, i); }
	  b's' => { SINGLE_LETTER_MOD! (this_mod_end, this_mod, super_modifier, i); }

	  b'd' => {
	    MULTI_LETTER_MOD! (this_mod_end, this_mod, drag_modifier, name, c"drag", 4, i);
	    MULTI_LETTER_MOD! (this_mod_end, this_mod, down_modifier, name, c"down", 4, i);
	    MULTI_LETTER_MOD! (this_mod_end, this_mod, double_modifier, name, c"double", 6, i);
	  }

	  b't' => {
	    MULTI_LETTER_MOD! (this_mod_end, this_mod, triple_modifier, name, c"triple", 6, i);
	  }

	  b'u' => {
	    MULTI_LETTER_MOD! (this_mod_end, this_mod, up_modifier, name, c"up", 2, i);
	  }

	  _ => {}
	}

      /* If we found no modifier, stop looking for them.  */
      if this_mod_end == 0
	{ break; }

      /* Check there is a dash after the modifier, so that it
	 really is a modifier.  */
      if this_mod_end >= SBYTES (name)
	  || SREF (name, this_mod_end as usize) != b'-'
	{ break; }

      /* This modifier is real; look for another.  */
      modifiers |= this_mod;
      i = this_mod_end + 1;
    }

  /* Should we include the `click' modifier?  */
  if (modifiers & (down_modifier | drag_modifier
		   | double_modifier | triple_modifier)) == 0
      && i + 7 == SBYTES (name)
      && memcmp (SDATA (name).offset (i) as *const c_void,
		 c"mouse-".as_ptr () as *const c_void, 6) == 0
      && (b'0' <= SREF (name, (i + 6) as usize) && SREF (name, (i + 6) as usize) <= b'9')
    { modifiers |= click_modifier; }

  if (modifiers & (double_modifier | triple_modifier)) == 0
      && i + 6 < SBYTES (name)
      && memcmp (SDATA (name).offset (i) as *const c_void,
		 c"wheel-".as_ptr () as *const c_void, 6) == 0
    { modifiers |= click_modifier; }

  if !modifier_end.is_null ()
    { *modifier_end = i; }

  return modifiers;
}

/* C 7478-7483: `apply_modifiers_uncached`'s own comment, above its definition.  */
/* C 7479-7526: `static Lisp_Object apply_modifiers_uncached (int modifiers, char *base,
   int base_len, int base_len_byte);`  */
unsafe fn apply_modifiers_uncached (modifiers: c_int, base: *mut c_char, base_len: c_int,
				    base_len_byte: c_int) -> Lisp_Object
{
  /* Since BASE could contain nulls, we can't use intern here; we have
     to use Fintern, which expects a genuine Lisp_String, and keeps a
     reference to it.  */
  /* C 7484: `char new_mods[sizeof "A-C-H-M-S-s-up-down-drag-double-triple-"];` (the literal is
     39 bytes, so the array is 40).  */
  let mut new_mods: [c_char; 40] = [0; 40];
  let mut mod_len: c_int;

  {
    let mut p: *mut c_char = new_mods.as_mut_ptr ();

    /* Mouse events should not exhibit the `up' modifier once they
       leave the event queue only accessible to C code; `up' will
       always be turned into a click or drag event before being
       presented to lisp code.  But since lisp events can be
       synthesized bypassing the event queue and pushed into
       `unread-command-events' or its companions, it's better to just
       deal with unexpected modifier combinations. */

    /* C's `*p++ = 'A'; *p++ = '-';` stores through the old pointer and steps after each store
       (the ccl.rs shape).  */
    if (modifiers & alt_modifier) != 0
      { *p = b'A' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & ctrl_modifier) != 0
      { *p = b'C' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & hyper_modifier) != 0
      { *p = b'H' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & meta_modifier) != 0
      { *p = b'M' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & shift_modifier) != 0
      { *p = b'S' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & super_modifier) != 0
      { *p = b's' as c_char; p = p.offset (1); *p = b'-' as c_char; p = p.offset (1); }
    if (modifiers & double_modifier) != 0 { p = stpcpy (p, c"double-".as_ptr ()); }
    if (modifiers & triple_modifier) != 0 { p = stpcpy (p, c"triple-".as_ptr ()); }
    if (modifiers & up_modifier) != 0 { p = stpcpy (p, c"up-".as_ptr ()); }
    if (modifiers & down_modifier) != 0 { p = stpcpy (p, c"down-".as_ptr ()); }
    if (modifiers & drag_modifier) != 0 { p = stpcpy (p, c"drag-".as_ptr ()); }
    /* The click modifier is denoted by the absence of other modifiers.  */

    *p = b'\0' as c_char;

    mod_len = p.offset_from (new_mods.as_ptr ()) as c_int;
  }

  {
    let mut new_name: Lisp_Object;

    new_name = make_uninit_multibyte_string ((mod_len + base_len) as EMACS_INT,
					     (mod_len + base_len_byte) as EMACS_INT);
    memcpy (SDATA (new_name) as *mut c_void, new_mods.as_ptr () as *const c_void,
	    mod_len as usize);
    memcpy (SDATA (new_name).offset (mod_len as isize) as *mut c_void,
	    base as *const c_void, base_len_byte as usize);

    return Fintern (new_name, Qnil ());
  }
}


/* C 7529-7534: `static const char *const modifier_names[]`.  The table is `static mut` for the
   reason the event tables are (C's `const char *const []` holds raw pointers, which are not
   `Sync`); C's `const` is preserved by the table never being written.  */
static mut modifier_names: [*const c_char; 28] = [
  c"up".as_ptr (), c"down".as_ptr (), c"drag".as_ptr (), c"click".as_ptr (),
  c"double".as_ptr (), c"triple".as_ptr (), std::ptr::null (), std::ptr::null (),
  std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
  std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
  std::ptr::null (), std::ptr::null (), std::ptr::null (), std::ptr::null (),
  std::ptr::null (), std::ptr::null (), c"alt".as_ptr (), c"super".as_ptr (),
  c"hyper".as_ptr (), c"shift".as_ptr (), c"control".as_ptr (), c"meta".as_ptr ()
];
/* C 7535: `#define NUM_MOD_NAMES countof (modifier_names)`.  C's object-like macro names the
   table, and this unit's `modifier_names` is a `static mut` (raw pointers are not `Sync`); Rust
   refuses to read a mutable static from a `const` initialiser, so the port keeps C's macro as a
   `macro_rules!` whose body is C's own expression, evaluated at its one use site (the
   `alloc.rs` `countof! (spare_memory)` shape).  */
macro_rules! NUM_MOD_NAMES { () => { countof! (modifier_names) as c_int } }

/* C 7537: `static Lisp_Object modifier_symbols;`  */
static mut modifier_symbols: Lisp_Object = lisp_h_Qnil ();

/* C 7539-7540: `lispy_modifier_list`'s own comment, above its definition.  */
/* C 7541-7553: `static Lisp_Object lispy_modifier_list (int modifiers);`  */
unsafe fn lispy_modifier_list (modifiers: c_int) -> Lisp_Object
{
  let mut modifier_list: Lisp_Object;
  let mut i: c_int;

  modifier_list = Qnil ();
  i = 0;
  while (1 << i) <= modifiers && i < NUM_MOD_NAMES! ()
    {
      if (modifiers & (1 << i)) != 0
	{ modifier_list = Fcons (AREF (modifier_symbols, i as isize), modifier_list); }
      i += 1;
    }

  return modifier_list;
}


/* C 7556-7561: `parse_modifiers`'s own comment, above its definition.  */
/* C 7563: `#define KEY_TO_CHAR(k) (XFIXNUM (k) & ((1 << CHARACTERBITS) - 1))`  */
macro_rules! KEY_TO_CHAR { ($k:expr) => { XFIXNUM ($k) & ((1 << CHARACTERBITS) - 1) }; }

/* C 7565-7607: `Lisp_Object parse_modifiers (Lisp_Object symbol);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_modifiers (symbol: Lisp_Object) -> Lisp_Object
{
  let mut elements: Lisp_Object;

  if FIXNUMP (symbol)
    { return list2i (KEY_TO_CHAR! (symbol),
		     XFIXNUM (symbol) & (CHAR_MODIFIER_MASK as EMACS_INT)); }
  else if ! SYMBOLP (symbol)
    { return Qnil (); }

  elements = Fget (symbol, Qevent_symbol_element_mask ());
  if CONSP (elements)
    { return elements; }
  else
    {
      let mut end: ptrdiff_t = 0;
      let modifiers: c_int = parse_modifiers_uncached (symbol, &raw mut end);
      let mut unmodified: Lisp_Object;
      let mut mask: Lisp_Object;

      unmodified = Fintern (make_string (SSDATA (SYMBOL_NAME (symbol)).offset (end),
					 SBYTES (SYMBOL_NAME (symbol)) - end),
			    Qnil ());

      if ((modifiers as EMACS_INT) & !INTMASK) != 0
	{ emacs_abort (); }
      XSETFASTINT! (mask, modifiers as EMACS_INT);
      elements = list2 (unmodified, mask);

      /* Cache the parsing results on SYMBOL.  */
      Fput (symbol, Qevent_symbol_element_mask (),
	    elements);
      Fput (symbol, Qevent_symbol_elements (),
	    Fcons (unmodified, lispy_modifier_list (modifiers)));

      /* Since we know that SYMBOL is modifiers applied to unmodified,
	 it would be nice to put that in unmodified's cache.
	 But we can't, since we're not sure that parse_modifiers is
	 canonical.  */

      return elements;
    }
}

/* C 7609-7620: `DEFUN ("internal-event-symbol-parse-modifiers", …)`.  */
DEFUN! ("internal-event-symbol-parse-modifiers", Fevent_symbol_parse_modifiers,
       Sevent_symbol_parse_modifiers, 1, 1, 0,
       doc: "Parse the event symbol.  For internal use.",
       (symbol: Lisp_Object),
       {
  /* Fill the cache if needed.  */
  parse_modifiers (symbol);
  /* Ignore the result (which is stored on Qevent_symbol_element_mask)
     and use the Lispier representation stored on Qevent_symbol_elements
     instead.  */
  return Fget (symbol, Qevent_symbol_elements ());
});

/* C 7622-7630: `apply_modifiers`'s own comment, above its definition.  */
/* C 7631-7680: `static Lisp_Object apply_modifiers (int modifiers, Lisp_Object base);`  */
unsafe fn apply_modifiers (mut modifiers: c_int, base: Lisp_Object) -> Lisp_Object
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut cache: Lisp_Object = Qnil ();
  let mut idx: Lisp_Object = Qnil ();
  let mut entry: Lisp_Object = Qnil ();
  let mut new_symbol: Lisp_Object = Qnil ();

  /* Mask out upper bits.  We don't know where this value's been.  */
  modifiers &= INTMASK as c_int;

  if FIXNUMP (base)
    { return make_fixnum (XFIXNUM (base) | modifiers as EMACS_INT); }

  /* The click modifier never figures into cache indices.  */
  cache = Fget (base, Qmodifier_cache ());
  XSETFASTINT! (idx, (modifiers & !click_modifier) as EMACS_INT);
  entry = assq_no_quit (idx, cache);

  if CONSP (entry)
    { new_symbol = XCDR (entry); }
  else
    {
      /* We have to create the symbol ourselves.  */
      new_symbol = apply_modifiers_uncached (modifiers, SSDATA (SYMBOL_NAME (base)),
					     SCHARS (SYMBOL_NAME (base)) as c_int,
					     SBYTES (SYMBOL_NAME (base)) as c_int);

      /* Add the new symbol to the base's cache.  */
      entry = Fcons (idx, new_symbol);
      Fput (base, Qmodifier_cache (), Fcons (entry, cache));

      /* We have the parsing info now for free, so we could add it to
	 the caches:
         XSETFASTINT (idx, modifiers);
         Fput (new_symbol, Qevent_symbol_element_mask,
               list2 (base, idx));
         Fput (new_symbol, Qevent_symbol_elements,
               Fcons (base, lispy_modifier_list (modifiers)));
	 Sadly, this is only correct if `base' is indeed a base event,
	 which is not necessarily the case.  -stef  */
    }

  /* Make sure this symbol is of the same kind as BASE.

     You'd think we could just set this once and for all when we
     intern the symbol above, but reorder_modifiers may call us when
     BASE's property isn't set right; we can't assume that just
     because it has a Qmodifier_cache property it must have its
     Qevent_kind set right as well.  */
  if NILP (Fget (new_symbol, Qevent_kind ()))
    {
      let mut kind: Lisp_Object;

      kind = Fget (base, Qevent_kind ());
      if ! NILP (kind)
	{ Fput (new_symbol, Qevent_kind (), kind); }
    }

  return new_symbol;
}


/* C 7682-7698: `reorder_modifiers`'s own comment, above its definition.  */
/* C 7700-7705: `Lisp_Object reorder_modifiers (Lisp_Object symbol);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reorder_modifiers (symbol: Lisp_Object) -> Lisp_Object
{
  /* It's hopefully okay to write the code this way, since everything
     will soon be in caches, and no consing will be done at all.  */
  let mut parsed: Lisp_Object;

  parsed = parse_modifiers (symbol);
  return apply_modifiers (XFIXNAT (XCAR (XCDR (parsed))) as c_int,
			  XCAR (parsed));
}


/* For handling events, we often want to produce a symbol whose name
   is a series of modifier key prefixes ("M-", "C-", etcetera) attached
   to some base, like the name of a function key or mouse button.
   modify_event_symbol produces symbols of this sort.

   NAME_TABLE should point to an array of strings, such that NAME_TABLE[i]
   is the name of the i'th symbol.  TABLE_SIZE is the number of elements
   in the table.

   Alternatively, NAME_ALIST_OR_STEM is either an alist mapping codes
   into symbol names, or a string specifying a name stem used to
   construct a symbol name or the form `STEM-N', where N is the decimal
   representation of SYMBOL_NUM.  NAME_ALIST_OR_STEM is used if it is
   non-nil; otherwise NAME_TABLE is used.

   SYMBOL_TABLE should be a pointer to a Lisp_Object whose value will
   persist between calls to modify_event_symbol that it can use to
   store a cache of the symbols it's generated for this NAME_TABLE
   before.  The object stored there may be a vector or an alist.

   SYMBOL_NUM is the number of the base name we want from NAME_TABLE.

   MODIFIERS is a set of modifier bits (as given in struct input_events)
   whose prefixes should be applied to the symbol name.

   SYMBOL_KIND is the value to be placed in the event_kind property of
   the returned symbol.

   The symbols we create are supposed to have an
   `event-symbol-elements' property, which lists the modifiers present
   in the symbol's name.  */

/* C 7791 and 7799: `<intprops.h>`'s `INT_STRLEN_BOUND` has no mirror (P-26); gnulib defines it
   as `((TYPE_WIDTH (t) - TYPE_SIGNED (t)) * 146 + 484) / 485 + TYPE_SIGNED (t)`, and the port
   keeps that arithmetic over the frozen type (`openat-proc.rs`'s shape, where the same macro
   lands at its first reader).  `INT_STRLEN_BOUND (EMACS_INT)` is 20.  */
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/* C 7745-7830: `static Lisp_Object modify_event_symbol (ptrdiff_t symbol_num, int modifiers,
   Lisp_Object symbol_kind, Lisp_Object name_alist_or_stem, const char *const *name_table,
   Lisp_Object *symbol_table, ptrdiff_t table_size);`  */
unsafe fn modify_event_symbol (symbol_num: ptrdiff_t, modifiers: c_int, symbol_kind: Lisp_Object,
			       name_alist_or_stem: Lisp_Object,
			       name_table: *const *const c_char,
			       symbol_table: *mut Lisp_Object, table_size: ptrdiff_t)
  -> Lisp_Object
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut value: Lisp_Object = Qnil ();
  let mut symbol_int: Lisp_Object = Qnil ();

  /* Get rid of the "vendor-specific" bit here.  */
  XSETINT! (symbol_int, symbol_num & 0xffffff);

  /* Is this a request for a valid symbol?  */
  if symbol_num < 0 || symbol_num >= table_size
    { return Qnil (); }

  if CONSP (*symbol_table)
    { value = Fcdr (assq_no_quit (symbol_int, *symbol_table)); }

  /* If *symbol_table doesn't seem to be initialized properly, fix that.
     *symbol_table should be a lisp vector TABLE_SIZE elements long,
     where the Nth element is the symbol for NAME_TABLE[N], or nil if
     we've never used that symbol before.  */
  else
    {
      if ! VECTORP (*symbol_table)
	  || ASIZE (*symbol_table) != table_size
	{ *symbol_table = make_nil_vector (table_size); }

      value = AREF (*symbol_table, symbol_num);
    }

  /* Have we already used this symbol before?  */
  if NILP (value)
    {
      /* No; let's create it.  */
      if CONSP (name_alist_or_stem)
	{ value = Fcdr_safe (Fassq (symbol_int, name_alist_or_stem)); }
      else if STRINGP (name_alist_or_stem)
	{
	  let mut buf: *mut c_char;
	  /* C: the bound is `SBYTES (name_alist_or_stem) + sizeof "-" + INT_STRLEN_BOUND
	     (EMACS_INT)`; `sizeof "-"` is 2.  */
	  let len: ptrdiff_t = (SBYTES (name_alist_or_stem) + 2
				+ INT_STRLEN_BOUND! (EMACS_INT) as ptrdiff_t);
	  USE_SAFE_ALLOCA! (sa_avail, sa_count);
	  buf = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;
	  /* C's `%"pI"d` is `%ld` (lisp.h 98: `pI` is "l").  */
	  esprintf (buf, c"%s-%ld".as_ptr (), SDATA (name_alist_or_stem),
		    XFIXNUM (symbol_int) + 1);
	  value = intern (buf);
	  SAFE_FREE! (sa_count);
	}
      else if !name_table.is_null () && !(*(name_table.offset (symbol_num))).is_null ()
	{ value = intern (*(name_table.offset (symbol_num))); }

      /* C 7794-7801: `#ifdef HAVE_WINDOW_SYSTEM`'s `get_keysym_name` arm (carried per P-51;
         the compiled text never reaches `get_keysym_name`, so it is not declared).  */
      #[cfg (feature = "have_window_system")]
      if NILP (value)
	{
	  let name: *mut c_char = get_keysym_name (symbol_num);
	  if !name.is_null ()
	    { value = intern (name); }
	}

      if NILP (value)
	{
	  /* C: `char buf[sizeof "key-" + INT_STRLEN_BOUND (EMACS_INT)]`; `sizeof "key-"` is 5,
	     so the array is 25 bytes.  C's `%"pD"d` is `%td` (lisp.h 150: `pD` is "t").  */
	  let mut buf: [c_char; 25] = [0; 25];
	  sprintf (buf.as_mut_ptr (), c"key-%td".as_ptr (), symbol_num);
	  value = intern (buf.as_ptr ());
	}

      if CONSP (*symbol_table)
        { *symbol_table = Fcons (Fcons (symbol_int, value), *symbol_table); }
      else
	{ ASET (*symbol_table, symbol_num, value); }

      /* Fill in the cache entries for this symbol; this also
	 builds the Qevent_symbol_elements property, which the user
	 cares about.  */
      apply_modifiers (modifiers & click_modifier, value);
      Fput (value, Qevent_kind (), symbol_kind);
    }

  /* Apply modifiers to that symbol.  */
  return apply_modifiers (modifiers, value);
}


/* Convert a list that represents an event type,
   such as (ctrl meta backspace), into the usual representation of that
   event type as a number or a symbol.  */

/* C 7832-7889: `DEFUN ("event-convert-list", …)`.  */
DEFUN! ("event-convert-list", Fevent_convert_list, Sevent_convert_list, 1, 1, 0,
       doc: "Convert the event description list EVENT-DESC to an event type.\nEVENT-DESC should contain one base event type (a character or symbol)\nand zero or more modifier names (control, meta, hyper, super, shift, alt,\ndrag, down, double or triple).  The base must be last.\n\nThe return value is an event type (a character or symbol) which has\nessentially the same base event type and all the specified modifiers.\n(Some compatibility base types, like symbols that represent a\ncharacter, are not returned verbatim.)",
       (event_desc: Lisp_Object),
       {
  let mut base: Lisp_Object = Qnil ();
  let mut modifiers: c_int = 0;

  FOR_EACH_TAIL_SAFE! (event_desc, {
    let elt: Lisp_Object = XCAR (event_desc);
    let mut this: c_int = 0;

    /* Given a symbol, see if it is a modifier name.  */
    if SYMBOLP (elt) && CONSP (XCDR (event_desc))
      { this = parse_solitary_modifier (elt); }

    if this != 0
      { modifiers |= this; }
    else if !NILP (base)
      { error (c"Two bases given in one event".as_ptr ()); }
    else
      { base = elt; }
  });

  /* Let the symbol A refer to the character A.  */
  if SYMBOLP (base) && SCHARS (SYMBOL_NAME (base)) == 1
    { XSETINT! (base, SREF (SYMBOL_NAME (base), 0) as EMACS_INT); }

  if FIXNUMP (base)
    {
      /* Turn (shift a) into A.  */
      if (modifiers & shift_modifier) != 0
	  && (XFIXNUM (base) >= b'a' as EMACS_INT && XFIXNUM (base) <= b'z' as EMACS_INT)
	{
	  XSETINT! (base, XFIXNUM (base) - (b'a' - b'A') as EMACS_INT);
	  modifiers &= !shift_modifier;
	}

      /* Turn (control a) into C-a.  */
      if (modifiers & ctrl_modifier) != 0
	{ return make_fixnum (((modifiers & !ctrl_modifier)
				| make_ctrl_char (XFIXNUM (base) as c_int)) as EMACS_INT); }
      else
	{ return make_fixnum ((modifiers | XFIXNUM (base) as c_int) as EMACS_INT); }
    }
  else if SYMBOLP (base)
    { return apply_modifiers (modifiers, base); }
  else
    { error (c"Invalid base event".as_ptr ()); }
});

/* C 7891-7918: `DEFUN ("internal-handle-focus-in", …)`.  */
DEFUN! ("internal-handle-focus-in", Finternal_handle_focus_in,
       Sinternal_handle_focus_in, 1, 1, 0,
       doc: "Internally handle focus-in events.\nThis function potentially generates an artificial switch-frame event.",
       (event: Lisp_Object),
       {
  let mut frame: Lisp_Object = Qnil ();
  if !EQ (CAR_SAFE (event), Qfocus_in ())
      || !CONSP (XCDR (event))
      || { frame = XCAR (XCDR (event)); !FRAMEP (frame) }
    { error (c"Invalid focus-in event".as_ptr ()); }

  /* Conceptually, the concept of window manager focus on a particular
     frame and the Emacs selected frame shouldn't be related, but for
     a long time, we automatically switched the selected frame in
     response to focus events, so let's keep doing that.  */
  let switching: bool = !EQ (frame, internal_last_event_frame)
                       && !EQ (frame, selected_frame);
  internal_last_event_frame = frame;
  if switching || !NILP (unread_switch_frame)
    { unread_switch_frame = make_lispy_switch_frame (frame); }

  return Qnil ();
});


/* Try to recognize SYMBOL as a modifier name.
   Return the modifier flag bit, or 0 if not recognized.  */

/* C 7920-8008: `int parse_solitary_modifier (Lisp_Object symbol);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_solitary_modifier (symbol: Lisp_Object) -> c_int
{
  let mut name: Lisp_Object;

  if !SYMBOLP (symbol)
    { return 0; }

  name = SYMBOL_NAME (symbol);

  /* C 7931-7934 and 7935-7940: the second pair of `SINGLE_LETTER_MOD`/`MULTI_LETTER_MOD`
     definitions (their bodies differ from `parse_modifiers_uncached`'s pair), with the same
     metavariable-passing deviation.  C's macros `return` from the enclosing function, and so
     does a Rust `return` inside the expansion.  */
  macro_rules! SINGLE_LETTER_MOD {
    ($bit:expr, $name:expr) => {
      if SBYTES ($name) == 1
	{ return $bit; }
    };
  }
  macro_rules! MULTI_LETTER_MOD {
    ($bit:expr, $word:expr, $len:expr, $name:expr) => {
      if $len == SBYTES ($name)
	  && memcmp (SDATA ($name) as *const c_void, $word.as_ptr () as *const c_void,
		     $len as usize) == 0
	{ return $bit; }
    };
  }

  match SREF (name, 0)
    {
      b'A' => { SINGLE_LETTER_MOD! (alt_modifier, name); }

      b'a' => { MULTI_LETTER_MOD! (alt_modifier, c"alt", 3, name); }

      b'C' => { SINGLE_LETTER_MOD! (ctrl_modifier, name); }

      b'c' => {
	MULTI_LETTER_MOD! (ctrl_modifier, c"ctrl", 4, name);
	MULTI_LETTER_MOD! (ctrl_modifier, c"control", 7, name);
	MULTI_LETTER_MOD! (click_modifier, c"click", 5, name);
      }

      b'H' => { SINGLE_LETTER_MOD! (hyper_modifier, name); }

      b'h' => { MULTI_LETTER_MOD! (hyper_modifier, c"hyper", 5, name); }

      b'M' => { SINGLE_LETTER_MOD! (meta_modifier, name); }

      b'm' => { MULTI_LETTER_MOD! (meta_modifier, c"meta", 4, name); }

      b'S' => { SINGLE_LETTER_MOD! (shift_modifier, name); }

      b's' => {
	MULTI_LETTER_MOD! (shift_modifier, c"shift", 5, name);
	MULTI_LETTER_MOD! (super_modifier, c"super", 5, name);
	SINGLE_LETTER_MOD! (super_modifier, name);
      }

      b'd' => {
	MULTI_LETTER_MOD! (drag_modifier, c"drag", 4, name);
	MULTI_LETTER_MOD! (down_modifier, c"down", 4, name);
	MULTI_LETTER_MOD! (double_modifier, c"double", 6, name);
      }

      b't' => { MULTI_LETTER_MOD! (triple_modifier, c"triple", 6, name); }

      b'u' => { MULTI_LETTER_MOD! (up_modifier, c"up", 2, name); }

      _ => {}
    }

  return 0;
}

/* Return true if EVENT is a list whose elements are all integers or symbols.
   Such a list is not valid as an event,
   but it can be a Lucid-style event type list.  */

/* C 8010-8040: `bool lucid_event_type_list_p (Lisp_Object object);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lucid_event_type_list_p (object: Lisp_Object) -> bool
{
  if ! CONSP (object)
    { return false; }

  if EQ (XCAR (object), Qhelp_echo ())
      || EQ (XCAR (object), Qvertical_line ())
      || EQ (XCAR (object), Qmode_line ())
      || EQ (XCAR (object), Qtab_line ())
      || EQ (XCAR (object), Qheader_line ())
    { return false; }

  let mut tail: Lisp_Object = object;
  FOR_EACH_TAIL_SAFE! (object, {
    let elt: Lisp_Object = XCAR (object);
    if ! (FIXNUMP (elt) || SYMBOLP (elt))
      { return false; }
    tail = XCDR (object);
  });

  return NILP (tail);
}


/* Return true if terminal input chars are available.
   Also, store the return value into INPUT_PENDING.

   Serves the purpose of ioctl (0, FIONREAD, ...)
   but works even if FIONREAD does not exist.
   (In fact, this may actually read some input.)

   If READABLE_EVENTS_DO_TIMERS_NOW is set in FLAGS, actually run
   timer events that are ripe.
   If READABLE_EVENTS_FILTER_EVENTS is set in FLAGS, ignore internal
   events (FOCUS_IN_EVENT).
   If READABLE_EVENTS_IGNORE_SQUEEZABLES is set in FLAGS, ignore mouse
   movements and toolkit scroll bar thumb drags.

   On X, this also returns if the selection event chain is full, since
   that's also "keyboard input".  */

/* C 8052-8066: `static bool get_input_pending (int flags);`  */
unsafe fn get_input_pending (flags: c_int) -> bool
{
  /* First of all, have we already counted some input?  */
  input_pending = !NILP (Vquit_flag) || readable_events (flags);

  /* If input is being read as it arrives, and we have none, there is none.  */
  if !input_pending && (!interrupt_input || interrupts_deferred)
    {
      /* Try to read some input and see how much we get.  */
      gobble_input ();
      input_pending = !NILP (Vquit_flag) || readable_events (flags);
    }

  return input_pending;
}

/* Read any terminal input already buffered up by the system
   into the kbd_buffer, but do not wait.

   Return the number of keyboard chars read, or -1 meaning
   this is a bad time to try to read input.  */

/* C 8074-8161: `int gobble_input (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gobble_input () -> c_int
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut nread: c_int = 0;
  let mut err: bool = false;
  let mut t: *mut terminal;

  /* Store pending user signal events, if any.  */
  store_user_signal_events ();

  /* Loop through the available terminals, and call their input hooks.  */
  t = terminal_list;
  while !t.is_null ()
    {
      let next: *mut terminal = (*t).next_terminal;

      if (*t).read_socket_hook.is_some ()
        {
          let mut nr: c_int = 0;
          let mut hold_quit: input_event = std::mem::zeroed ();

	  if input_blocked_p ()
	    {
	      pending_signals = true;
	      break;
	    }

          EVENT_INIT (&raw mut hold_quit);
          input_event_bits! (kind, &raw mut hold_quit, NO_EVENT);

          /* No need for FIONREAD or fcntl; just say don't wait.  */
	  while { nr = ((*t).read_socket_hook.unwrap ()) (t, &raw mut hold_quit); nr > 0 }
	    { nread += nr; }

          if nr == -1          /* Not OK to read input now.  */
            {
              err = true;
            }
          else if nr == -2          /* Non-transient error.  */
            {
              /* The terminal device terminated; it should be closed.  */

              /* Kill Emacs if this was our last terminal.  */
              if (*terminal_list).next_terminal.is_null ()
                /* Formerly simply reported no input, but that
                   sometimes led to a failure of Emacs to terminate.
                   SIGHUP seems appropriate if we can't reach the
                   terminal.  */
                /* ??? Is it really right to send the signal just to
                   this process rather than to the whole process
                   group?  Perhaps on systems with FIONREAD Emacs is
                   alone in its group.  */
		{ terminate_due_to_signal (SIGHUP, 10); }

              /* XXX Is calling delete_terminal safe here?  It calls delete_frame.  */
	      {
		let mut tmp: Lisp_Object = Qnil ();
		XSETTERMINAL! (tmp, t);
		Fdelete_terminal (tmp, Qnoelisp ());
	      }
            }

	  /* If there was no error, make sure the pointer
	     is visible for all frames on this terminal.  */
	  if nr >= 0
	    {
	      let mut tail: Lisp_Object = Qnil ();
	      let mut frame_: Lisp_Object = Qnil ();

	      FOR_EACH_FRAME! (tail, frame_,
		{
		  let f: *mut frame = XFRAME (frame_);
		  if FRAME_TERMINAL! (f) == t
		    { frame_make_pointer_visible (f); }
		});
	    }

          if input_event_bits! (kind, &raw mut hold_quit) != NO_EVENT
            { kbd_buffer_store_event (&raw mut hold_quit as *mut c_void); }
        }

      t = next;
    }

  if err && nread == 0
    { nread = -1; }

  return nread;
}


/* This is the tty way of reading available input.

   Note that each terminal device has its own `struct terminal' object,
   and so this function is called once for each individual termcap
   terminal.  The first parameter indicates which terminal to read from.  */

/* <sys/ioctl.h>: `#define FIONREAD 0x4004667f` (measured: 1074030207, the `sysdep.rs`
   request-word family).  No mirror carries it.  */
pub const FIONREAD: c_ulong = 0x4004667f;

/* termchar.h 62-64: `bool_bf term_initted : 1` is packed into the mirror's
   `tty_display_info::bits_initted` byte (the dispnew.rs `term_initted` shape).  */
macro_rules! term_initted { ($tty:expr) => { ((*($tty)).bits_initted & 1) != 0 }; }

/* C 8169-8343: `int tty_read_avail_input (struct terminal *terminal,
   struct input_event *hold_quit);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_read_avail_input (terminal: *mut terminal,
					       hold_quit: *mut input_event) -> c_int
{
  /* Using KBD_BUFFER_SIZE - 1 here avoids reading more than
     the kbd_buffer can really hold.  That may prevent loss
     of characters on some systems when input is stuffed at us.  */
  /* C 8176: `unsigned char cbuf[KBD_BUFFER_SIZE - 1];`  */
  let mut cbuf: [c_uchar; (KBD_BUFFER_SIZE - 1) as usize] = [0; (KBD_BUFFER_SIZE - 1) as usize];
  /* C 8177-8179: `#ifndef WINDOWSNT`'s `int n_to_read;` — `WINDOWSNT` is not defined on this
     target, so this is live text.  */
  let mut n_to_read: c_int = 0;
  let mut i: c_int;
  let tty: *mut tty_display_info = (*terminal).display_info.tty as *mut tty_display_info;
  let mut nread: c_int = 0;
  /* C 8183-8188: `#ifdef subprocesses` (live: `subprocesses` is 1 here).  `buffer_free` is
     read again at C 8266, so it stays at the function's top (C's own position).  */
  let buffer_free: c_int = KBD_BUFFER_SIZE - kbd_buffer_nr_stored () - 1;

  if kbd_on_hold_p () || buffer_free <= 0
    { return 0; }

  if (*terminal).name.is_null ()		/* Don't read from a dead terminal.  */
    { return 0; }

  if (*terminal).r#type != output_termcap
      && (*terminal).r#type != output_msdos_raw
    { emacs_abort (); }

  /* XXX I think the following code should be moved to separate hook
     functions in system-dependent files.  */
  /* C 8199-8202: `#ifdef WINDOWSNT` (`WINDOWSNT` is not defined; carried per P-51).  */
  #[cfg (feature = "windowsnt")]
  {
  /* FIXME: AFAIK, tty_read_avail_input is not used under w32 since the non-GUI
     code sets read_socket_hook to w32_console_read_socket instead!  */
  return 0;
  }
  /* C 8203: `#else /* not WINDOWSNT */` — the live arm (the loop at C 8301 is outside the
     conditional, shared by both arms).  */
  if !term_initted! (tty)      /* In case we get called during bootstrap.  */
    { return 0; }

  if (*tty).input.is_null ()
    { return 0; }                   /* The terminal is suspended.  */

  /* C 8210-8217: `#ifdef MSDOS` (`MSDOS` is not defined; carried per P-51).  */
  #[cfg (feature = "msdos")]
  {
  n_to_read = dos_keysns ();
  if n_to_read == 0
    { return 0; }

  cbuf[0] = dos_keyread ();
  nread = 1;
  }
  /* C 8218: `#else /* not MSDOS */` — the live arm.  */

  /* C 8219-8240: `#ifdef HAVE_GPM` (`HAVE_GPM` is not defined; carried per P-51).  */
  #[cfg (feature = "have_gpm")]
  {
  if !gpm_tty.is_null () && gpm_tty == tty
  {
      let mut event: Gpm_Event = std::mem::zeroed ();
      let mut gpm: c_int;
      let fd: c_int = gpm_fd;

      /* gpm==1 if event received.
         gpm==0 if the GPM daemon has closed the connection, in which case
                Gpm_GetEvent closes gpm_fd and clears it to -1, which is why
		we save it in `fd' so close_gpm can remove it from the
		select masks.
         gpm==-1 if a protocol error or EWOULDBLOCK; the latter is normal.  */
      while { gpm = Gpm_GetEvent (&raw mut event); gpm == 1 } {
	  nread += handle_one_term_event (tty, &raw mut event);
      }
      if gpm == 0
	/* Presumably the GPM daemon has closed the connection.  */
	{ close_gpm (fd); }
      if nread != 0
	  { return nread; }
  }
  }

/* Determine how many characters we should *try* to read.  */
  /* C 8243-8255: `#ifdef USABLE_FIONREAD` (live: `USABLE_FIONREAD` is 1 here).  */
  if ioctl (fileno ((*tty).input), FIONREAD, &raw mut n_to_read) < 0
    {
      if !noninteractive
        { return -2; }          /* Close this terminal.  */
      else
        { n_to_read = 0; }
    }
  if n_to_read == 0
    { return 0; }
  if n_to_read > std::mem::size_of_val (&cbuf) as c_int
    { n_to_read = std::mem::size_of_val (&cbuf) as c_int; }

  /* C 8256-8259: `#elif defined USG || defined CYGWIN` (both undefined; carried per P-51).  */
  #[cfg (feature = "usg_or_cygwin")]
  {
  /* Read some input if available, but don't wait.  */
  n_to_read = std::mem::size_of_val (&cbuf) as c_int;
  fcntl (fileno ((*tty).input), F_SETFL, O_NONBLOCK);
  }
  /* C 8260-8262: `#else` / `# error "Cannot read without possibly delaying"` — an `#error`
     arm has no text to carry.  */

  /* C 8264-8268: `#ifdef subprocesses` (live).  */
  /* Don't read more than we can store.  */
  if n_to_read > buffer_free
    { n_to_read = buffer_free; }

  /* Now read; for one reason or another, this will not block.
     NREAD is set to the number of chars read.  */
  nread = emacs_read (fileno ((*tty).input), cbuf.as_mut_ptr () as *mut c_void,
		      n_to_read as ptrdiff_t) as c_int;
  /* POSIX infers that processes which are not in the session leader's
     process group won't get SIGHUPs at logout time.  BSDI adheres to
     this part standard and returns -1 from read (0) with errno==EIO
     when the control tty is taken away.
     Jeffrey Honig <jch@bsdi.com> says this is generally safe.  */
  if nread == -1 && *errno () == EIO
    { return -2; }          /* Close this terminal.  */
  /* C 8280-8287: `#if defined AIX && defined _BSD` (dead on this target; carried per P-51).  */
  #[cfg (feature = "aix_and_bsd")]
  {
  /* The kernel sometimes fails to deliver SIGHUP for ptys.
     This looks incorrect, but it isn't, because _BSD causes
     O_NDELAY to be defined in fcntl.h as O_NONBLOCK,
     and that causes a value other than 0 when there is no input.  */
  if nread == 0
    { return -2; }          /* Close this terminal.  */
  }

  /* C 8289-8293: `#ifndef USABLE_FIONREAD` (dead: `USABLE_FIONREAD` is 1 here; carried per
     P-51, with C 8290-8292's `#if defined (USG) || defined (CYGWIN)` inside it).  */
  #[cfg (feature = "not_usable_fionread")]
  {
  fcntl (fileno ((*tty).input), F_SETFL, 0);
  }

  if nread <= 0
    { return nread; }

  /* C 8298-8299: the `#endif`s of `#ifdef MSDOS` and `#ifdef WINDOWSNT`.  */

  i = 0;
  while i < nread
    {
      let mut buf: input_event = std::mem::zeroed ();
      EVENT_INIT (&raw mut buf);
      input_event_bits! (kind, &raw mut buf, ASCII_KEYSTROKE_EVENT);
      buf.modifiers = 0;
      if (*tty).meta_key == 1 && (cbuf[i as usize] & 0x80) != 0
        { buf.modifiers = meta_modifier as c_uint; }
      if (*tty).meta_key < 2
        { cbuf[i as usize] &= !0x80; }

      buf.code = cbuf[i as usize] as c_uint;
      /* Set the frame corresponding to the active tty.  Note that the
         value of selected_frame is not reliable here, redisplay tends
         to temporarily change it.  However, if the selected frame is a
         child frame, don't do that since it will cause switch frame
         events to switch to the root frame instead.  If the tty's top
         frame has not been set up yet, always use the selected frame
         (Bug#78966).  */
      if !FRAMEP ((*tty).top_frame)
	  || (!FRAME_PARENT_FRAME (XFRAME (selected_frame)).is_null ()
	      && (root_frame (XFRAME (selected_frame)) == XFRAME ((*tty).top_frame)))
	{ buf.frame_or_window = selected_frame; }
      else
	{ buf.frame_or_window = (*tty).top_frame; }

      /* If neither the selected frame nor the top frame were set,
	 something must have gone really wrong.  */
      eassert! (FRAMEP (buf.frame_or_window));

      buf.arg = Qnil ();

      kbd_buffer_store_event (&raw mut buf as *mut c_void);
      /* Don't look at input that follows a C-g too closely.
         This reduces lossage due to autorepeat on C-g.  */
      if input_event_bits! (kind, &raw mut buf) == ASCII_KEYSTROKE_EVENT
          && buf.code == quit_char as c_uint
        { break; }
      i += 1;
    }

  return nread;
}


/* C 8345-8367: `static void handle_async_input (void);`  */
unsafe fn handle_async_input ()
{
  /* C 8348-8354: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (both undefined on
     this target; carried per P-51).  */
  #[cfg (feature = "have_android")]
  {
  /* Check and respond to an ``urgent'' query from the UI thread.
     A query becomes urgent once the UI thread has been waiting
     for more than two seconds.  */

  android_check_query_urgent ();
  }
  /* C 8356-8366: `#ifndef DOS_NT` is live (`DOS_NT` is not defined here), so its body is not
     gated.  */
  loop
    {
      let nread: c_int = gobble_input ();
      /* -1 means it's not ok to read the input now.
	 UNBLOCK_INPUT will read it later; now, avoid infinite loop.
	 0 means there was no keyboard input available.  */
      if nread <= 0
	{ break; }
    }
}

/* C 8369-8375: `void process_pending_signals (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_pending_signals ()
{
  pending_signals = false;
  handle_async_input ();
  do_pending_atimers ();
}

/* `blockinput.h` 57-59 declare `unblock_input`, `totally_unblock_input` and `unblock_input_to`
   at the crate root (`include/blockinput.rs`'s `unsafe extern "C"` block), so their
   definitions live in this module and are glob-imported below: a crate-root definition would
   be `E0428` (the `keyboard_root_shadowed` shape).  */
pub mod keyboard_root_shadowed_3 {
  use super::*;

/* Undo any number of BLOCK_INPUT calls down to level LEVEL,
   and reinvoke any pending signal if the level is now 0 and
   a fatal error is not already in progress.  */

/* C 8381-8392: `unblock_input_to`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unblock_input_to (level: c_int)
{
  interrupt_input_blocked = level;
  if level == 0
    {
      if pending_signals && !fatal_error_in_progress
	{ process_pending_signals (); }
    }
  else if level < 0
    { emacs_abort (); }
}

/* End critical section.

   If doing signal-driven input, and a signal came in when input was
   blocked, reinvoke the signal handler now to deal with it.

   It will also process queued input, if it was not read before.
   When a longer code sequence does not use block/unblock input
   at all, the whole input gathered up to the next call to
   unblock_input will be processed inside that call. */

/* C 8404-8408: `unblock_input`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unblock_input ()
{
  unblock_input_to (interrupt_input_blocked - 1);
}

/* Undo any number of BLOCK_INPUT calls,
   and also reinvoke any pending signal.  */

/* C 8413-8417: `totally_unblock_input`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn totally_unblock_input ()
{
  unblock_input_to (0);
}

}
use keyboard_root_shadowed_3::*;

/* C 8419-8445: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` — **live** on this
   target (`USABLE_SIGIO` is 1 in the frozen configuration; measured with clang through
   `tools/check-cond-eval.py`'s probe shape), so this arm is not gated (the docs/ports/dispnew.md
   "carried text that is live on the frozen target" reading).  */

/* C 8421-8438: `void handle_input_available_signal (int sig);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_input_available_signal (sig: c_int)
{
  /* C 8424-8433: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (dead; carried per
     P-51, with C 8428-8432's `#if defined __aarch64__` inside it — that one is *live* here
     (`__aarch64__` is 1 in the frozen configuration), so its arm is not gated and its `#else`
     is).  */
  #[cfg (feature = "have_android")]
  {
  /* Make all writes from the Android UI thread visible.  If
     `android_urgent_query' has been set, preceding writes to query
     related variables should become observable here on as well.  */
  asm ("dmb ishst");
  #[cfg (feature = "not_aarch64")]
  {
  __atomic_thread_fence (__ATOMIC_SEQ_CST);
  }
  }
  pending_signals = true;

  if !input_available_clear_time.is_null ()
    { *input_available_clear_time = make_timespec (0, 0); }
}

/* C 8440-8444: `static void deliver_input_available_signal (int sig);`  */
unsafe extern "C" fn deliver_input_available_signal (sig: c_int)
{
  deliver_process_signal (sig, Some (handle_input_available_signal));
}
/* C 8445: the `#endif` of `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)`.  */


/* User signal events.  */

/* C 8450-8462: `struct user_signal_info`.  Measured: size 32, align 8, `sig` at 0, `name` at
   8, `npending` at 16 and `next` at 24.  */
#[repr(C)]
pub struct user_signal_info
{
  /* Signal number.  */
  pub sig: c_int,

  /* Name of the signal.  */
  pub name: *mut c_char,

  /* Number of pending signals.  */
  pub npending: c_int,

  pub next: *mut user_signal_info,
}
const _: () = assert! (std::mem::size_of::<user_signal_info> () == 32);
const _: () = assert! (std::mem::align_of::<user_signal_info> () == 8);
const _: () = assert! (std::mem::offset_of! (user_signal_info, sig) == 0);
const _: () = assert! (std::mem::offset_of! (user_signal_info, name) == 8);
const _: () = assert! (std::mem::offset_of! (user_signal_info, npending) == 16);
const _: () = assert! (std::mem::offset_of! (user_signal_info, next) == 24);

/* List of user signals.  */
/* C 8465: `static struct user_signal_info *user_signals = NULL;`  */
static mut user_signals: *mut user_signal_info = std::ptr::null_mut ();

/* C 8467-8487: `void add_user_signal (int sig, const char *name);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_user_signal (sig: c_int, name: *const c_char)
{
  let mut action: sigaction = std::mem::zeroed ();
  let mut p: *mut user_signal_info;

  p = user_signals;
  while !p.is_null ()
    {
      if (*p).sig == sig
	/* Already added.  */
	{ return; }
      p = (*p).next;
    }

  p = xmalloc (std::mem::size_of::<user_signal_info> ()) as *mut user_signal_info;
  (*p).sig = sig;
  (*p).name = xstrdup (name);
  (*p).npending = 0;
  (*p).next = user_signals;
  user_signals = p;

  emacs_sigaction_init (&raw mut action, Some (deliver_user_signal));
  sigaction (sig, &raw const action, std::ptr::null_mut ());
}

/* C 8489-8528: `static void handle_user_signal (int sig);`  */
unsafe extern "C" fn handle_user_signal (sig: c_int)
{
  let mut p: *mut user_signal_info;
  let mut special_event_name: *const c_char = std::ptr::null ();

  if SYMBOLP (Vdebug_on_event)
    { special_event_name = SSDATA (SYMBOL_NAME (Vdebug_on_event)); }

  p = user_signals;
  while !p.is_null ()
    {
      if (*p).sig == sig
	{
	  if !special_event_name.is_null ()
	      && strcmp (special_event_name, (*p).name) == 0
	    {
	      /* Enter the debugger in many ways.  */
	      debug_on_next_call = true;
	      debug_on_quit = true;
	      Vquit_flag = Qt ();
	      Vinhibit_quit = Qnil ();

	      /* Eat the event.  */
	      break;
	    }

	  (*p).npending += 1;
	  /* C 8515-8519: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` — live on this
	     target, so the arm is not gated.  */
	  if interrupt_input
	    { handle_input_available_signal (sig); }
	  else
	    {
	      /* Tell wait_reading_process_output that it needs to wake
		 up and look around.  */
	      if !input_available_clear_time.is_null ()
		{ *input_available_clear_time = make_timespec (0, 0); }
	    }
	  break;
	}
      p = (*p).next;
    }
}

/* C 8530-8534: `static void deliver_user_signal (int sig);`  */
unsafe extern "C" fn deliver_user_signal (sig: c_int)
{
  deliver_process_signal (sig, Some (handle_user_signal));
}

/* C 8536-8546: `static char *find_user_signal_name (int sig);`  */
unsafe fn find_user_signal_name (sig: c_int) -> *mut c_char
{
  let mut p: *mut user_signal_info;

  p = user_signals;
  while !p.is_null ()
    {
      if (*p).sig == sig
	{ return (*p).name; }
      p = (*p).next;
    }

  return std::ptr::null_mut ();
}

/* C 8548-8574: `static void store_user_signal_events (void);`  */
unsafe fn store_user_signal_events ()
{
  let mut p: *mut user_signal_info;
  let mut buf: input_event = std::mem::zeroed ();
  let mut buf_initialized: bool = false;

  p = user_signals;
  while !p.is_null ()
    {
      if (*p).npending > 0
	{
	  if ! buf_initialized
	    {
	      memset (&raw mut buf as *mut c_void, 0, std::mem::size_of_val (&buf));
	      input_event_bits! (kind, &raw mut buf, USER_SIGNAL_EVENT);
	      buf.frame_or_window = selected_frame;
	      buf_initialized = true;
	    }

	  /* C's `do { … } while (p->npending > 0)`: the body runs once and the test then ends
	     the loop, which is what `loop` with the test at the body's end is.  */
	  loop
	    {
	      buf.code = (*p).sig as c_uint;
	      kbd_buffer_store_event (&raw mut buf as *mut c_void);
	      (*p).npending -= 1;
	      if !((*p).npending > 0) { break; }
	    }
	}
      p = (*p).next;
    }
}


/* C 8577: `static void menu_bar_item (Lisp_Object, Lisp_Object, Lisp_Object, void *);` — a
   prototype, which PORTING.md 0.1-12 drops (Rust is order-independent and a declaration plus
   the definition is `E0428`; the definition is C 8786).  */
/* C 8578: `static Lisp_Object menu_bar_one_keymap_changed_items;`  */
static mut menu_bar_one_keymap_changed_items: Lisp_Object = lisp_h_Qnil ();

/* These variables hold the vector under construction within
   menu_bar_items and its subroutines, and the current index
   for storing into that vector.  */
/* C 8583-8584.  */
static mut menu_bar_items_vector: Lisp_Object = lisp_h_Qnil ();
static mut menu_bar_items_index: c_int = 0;


/* C 8587-8603: `static const char *const separator_names[]`.  `static mut` for the reason the
   other `const char *const []` tables are (raw pointers are not `Sync`); C's `const` is kept by
   the table never being written.  */
static mut separator_names: [*const c_char; 15] = [
  c"space".as_ptr (),
  c"no-line".as_ptr (),
  c"single-line".as_ptr (),
  c"double-line".as_ptr (),
  c"single-dashed-line".as_ptr (),
  c"double-dashed-line".as_ptr (),
  c"shadow-etched-in".as_ptr (),
  c"shadow-etched-out".as_ptr (),
  c"shadow-etched-in-dash".as_ptr (),
  c"shadow-etched-out-dash".as_ptr (),
  c"shadow-double-etched-in".as_ptr (),
  c"shadow-double-etched-out".as_ptr (),
  c"shadow-double-etched-in-dash".as_ptr (),
  c"shadow-double-etched-out-dash".as_ptr (),
  std::ptr::null ()
];

/* Return true if LABEL specifies a separator.  */

/* C 8607-8631: `bool menu_separator_name_p (const char *label);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn menu_separator_name_p (mut label: *const c_char) -> bool
{
  if label.is_null ()
    { return false; }
  else if strnlen (label, 4) == 4
	   && memcmp (label as *const c_void, c"--".as_ptr () as *const c_void, 2) == 0
	   && *label.offset (2) != b'-' as c_char
    {
      let mut i: c_int;
      label = label.offset (2);
      i = 0;
      while !separator_names[i as usize].is_null ()
	{
	  if strcmp (label, separator_names[i as usize]) == 0
            { return true; }
	  i += 1;
	}
    }
  else
    {
      /* It's a separator if it contains only dashes.  */
      while *label == b'-' as c_char
	{ label = label.offset (1); }
      return *label == 0;
    }

  return false;
}


/* Return a vector of menu items for a menu bar, appropriate
   to the current buffer.  Each item has three elements in the vector:
   KEY STRING MAPLIST.

   OLD is an old vector we can optionally reuse, or nil.  */

/* C 8640-8779: `Lisp_Object menu_bar_items (Lisp_Object old);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn menu_bar_items (old: Lisp_Object) -> Lisp_Object
{
  /* The number of keymaps we're scanning right now, and the number of
     keymaps we have allocated space for.  */
  let mut nmaps: ptrdiff_t;

  /* maps[0..nmaps-1] are the prefix definitions of KEYBUF[0..t-1]
     in the current keymaps, or nil where it is not a prefix.  C leaves this pointer
     uninitialised, which the port zeroes (PORTING.md 1.5).  */
  let mut maps: *mut Lisp_Object = std::ptr::null_mut ();

  let mut mapsbuf: [Lisp_Object; 3] = [Qnil (); 3];
  let mut def: Lisp_Object;

  let mut mapno: ptrdiff_t;
  let mut oquit: Lisp_Object;

  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  /* In order to build the menus, we need to call the keymap
     accessors.  They all call maybe_quit.  But this function is called
     during redisplay, during which a quit is fatal.  So inhibit
     quitting while building the menus.
     We do this instead of specbind because (1) errors will clear it anyway
     and (2) this avoids risk of specpdl overflow.  */
  oquit = Vinhibit_quit;
  Vinhibit_quit = Qt ();

  if !NILP (old)
    { menu_bar_items_vector = old; }
  else
    { menu_bar_items_vector = make_nil_vector (24); }
  menu_bar_items_index = 0;

  /* Build our list of keymaps.
     If we recognize a function key and replace its escape sequence in
     keybuf with its symbol, or if the sequence starts with a mouse
     click and we need to switch buffers, we jump back here to rebuild
     the initial keymaps from the current buffer.  */
  {
    let mut tmaps: *mut Lisp_Object = std::ptr::null_mut ();

    /* Should overriding-terminal-local-map and overriding-local-map apply?  */
    if !NILP (Voverriding_local_map_menu_flag)
	&& !NILP (Voverriding_local_map)
      {
	/* Yes, use them (if non-nil) as well as the global map.  */
	maps = mapsbuf.as_mut_ptr ();
	nmaps = 0;
	if !NILP (KVAR! (current_kboard, Voverriding_terminal_local_map))
	  { *maps.offset (nmaps) = KVAR! (current_kboard, Voverriding_terminal_local_map);
	    nmaps += 1; }
	if !NILP (Voverriding_local_map)
	  { *maps.offset (nmaps) = Voverriding_local_map; nmaps += 1; }
      }
    else
      {
	/* No, so use major and minor mode keymaps and keymap property.
	   Note that menu-bar bindings in the local-map and keymap
	   properties may not work reliable, as they are only
	   recognized when the menu-bar (or mode-line) is updated,
	   which does not normally happen after every command.  */
	let nminor: ptrdiff_t = current_minor_maps (std::ptr::null_mut (), &raw mut tmaps);
	SAFE_NALLOCA! (sa_avail, maps, 1, nminor + 4);
	nmaps = 0;
	let mut tem: Lisp_Object = KVAR! (current_kboard, Voverriding_terminal_local_map);
	if !NILP (tem) && !NILP (Voverriding_local_map_menu_flag)
	  { *maps.offset (nmaps) = tem; nmaps += 1; }
	/* C's `if (tem = get_local_map (PT, current_buffer, Qkeymap), !NILP (tem))` is the
	   assignment then the test (PORTING.md 2.2's comma-expression shape).  */
	if { tem = get_local_map (PT! (), current_buffer, Qkeymap ()); !NILP (tem) }
	  { *maps.offset (nmaps) = tem; nmaps += 1; }
	if nminor != 0
	  {
	    memcpy (maps.offset (nmaps) as *mut c_void, tmaps as *const c_void,
		    (nminor * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize);
	    nmaps += nminor;
	  }
	*maps.offset (nmaps) = get_local_map (PT! (), current_buffer, Qlocal_map ());
	nmaps += 1;
      }
    *maps.offset (nmaps) = current_global_map;
    nmaps += 1;
  }

  /* Look up in each map the dummy prefix key `menu-bar'.  */

  mapno = nmaps - 1;
  while mapno >= 0
    {
      if !NILP (*maps.offset (mapno))
	{
	  def = get_keymap (access_keymap (*maps.offset (mapno), Qmenu_bar (), true, false),
			    false, true);
	  if CONSP (def)
	    {
	      menu_bar_one_keymap_changed_items = Qnil ();
	      map_keymap_canonical (def, Some (menu_bar_item), Qnil (), std::ptr::null_mut ());
	    }
	}
      mapno -= 1;
    }

  /* Move to the end those items that should be at the end.  */

  let mut tail: Lisp_Object = Vmenu_bar_final_items;
  FOR_EACH_TAIL! (tail, {
    let end: c_int = menu_bar_items_index;

    let mut i: c_int = 0;
    while i < end
      {
	if EQ (XCAR (tail), AREF (menu_bar_items_vector, i as isize))
	  {
	    let mut tem0: Lisp_Object = Qnil ();
	    let mut tem1: Lisp_Object = Qnil ();
	    let mut tem2: Lisp_Object = Qnil ();
	    let mut tem3: Lisp_Object = Qnil ();
	    /* Move the item at index I to the end,
	       shifting all the others forward.  */
	    tem0 = AREF (menu_bar_items_vector, i as isize);
	    tem1 = AREF (menu_bar_items_vector, (i + 1) as isize);
	    tem2 = AREF (menu_bar_items_vector, (i + 2) as isize);
	    tem3 = AREF (menu_bar_items_vector, (i + 3) as isize);
	    if end > i + 4
	      { memmove (aref_addr (menu_bar_items_vector, i as isize) as *mut c_void,
			 aref_addr (menu_bar_items_vector, (i + 4) as isize) as *const c_void,
			 ((end - i - 4) as isize * word_size) as usize); }
	    ASET (menu_bar_items_vector, (end - 4) as isize, tem0);
	    ASET (menu_bar_items_vector, (end - 3) as isize, tem1);
	    ASET (menu_bar_items_vector, (end - 2) as isize, tem2);
	    ASET (menu_bar_items_vector, (end - 1) as isize, tem3);
	    break;
	  }
	i += 4;
      }
  });

  /* Add nil, nil, nil, nil at the end.  */
  {
    let mut i: c_int = menu_bar_items_index;
    if (i + 4) as isize > ASIZE (menu_bar_items_vector)
      { menu_bar_items_vector
	  = larger_vector (menu_bar_items_vector, 4, -1); }
    /* Add this item.  */
    ASET (menu_bar_items_vector, i as isize, Qnil ()); i += 1;
    ASET (menu_bar_items_vector, i as isize, Qnil ()); i += 1;
    ASET (menu_bar_items_vector, i as isize, Qnil ()); i += 1;
    ASET (menu_bar_items_vector, i as isize, Qnil ()); i += 1;
    menu_bar_items_index = i;
  }

  Vinhibit_quit = oquit;
  SAFE_FREE! (sa_count);
  return menu_bar_items_vector;
}


/* Add one item to menu_bar_items_vector, for KEY, ITEM_STRING and DEF.
   If there's already an item for KEY, add this DEF to it.  */

/* C 8786-8856: `static void menu_bar_item (Lisp_Object key, Lisp_Object item,
   Lisp_Object dummy1, void *dummy2);`  */
unsafe extern "C" fn menu_bar_item (key: Lisp_Object, mut item: Lisp_Object,
				    dummy1: Lisp_Object, dummy2: *mut c_void)
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut i: c_int = 0;
  let mut parsed: bool = false;
  let mut tem: Lisp_Object = Qnil ();

  if EQ (item, Qundefined ())
    {
      /* If a map has an explicit `undefined' as definition,
	 discard any previously made menu bar item.  */

      i = 0;
      while i < menu_bar_items_index
	{
	  if EQ (key, AREF (menu_bar_items_vector, i as isize))
	    {
	      if menu_bar_items_index > i + 4
		{ memmove (aref_addr (menu_bar_items_vector, i as isize) as *mut c_void,
			   aref_addr (menu_bar_items_vector, (i + 4) as isize) as *const c_void,
			   (((menu_bar_items_index - i - 4) as isize) * word_size) as usize); }
	      menu_bar_items_index -= 4;
	    }
	  i += 4;
	}
    }

  /* If this keymap has already contributed to this KEY,
     don't contribute to it a second time.  */
  tem = Fmemq (key, menu_bar_one_keymap_changed_items);
  if !NILP (tem) || NILP (item)
    { return; }

  menu_bar_one_keymap_changed_items
    = Fcons (key, menu_bar_one_keymap_changed_items);

  /* We add to menu_bar_one_keymap_changed_items before doing the
     parse_menu_item, so that if it turns out it wasn't a menu item,
     it still correctly hides any further menu item.  */
  parsed = parse_menu_item (item, 1);
  if !parsed
    { return; }

  item = AREF (item_properties, ITEM_PROPERTY_DEF as isize);

  /* Find any existing item for this KEY.  */
  i = 0;
  while i < menu_bar_items_index
    {
      if EQ (key, AREF (menu_bar_items_vector, i as isize))
	{ break; }
      i += 4;
    }

  /* If we did not find this KEY, add it at the end.  */
  if i == menu_bar_items_index
    {
      /* If vector is too small, get a bigger one.  */
      if (i + 4) as isize > ASIZE (menu_bar_items_vector)
	{ menu_bar_items_vector = larger_vector (menu_bar_items_vector, 4, -1); }
      /* Add this item.  */
      ASET (menu_bar_items_vector, i as isize, key); i += 1;
      ASET (menu_bar_items_vector, i as isize,
	    AREF (item_properties, ITEM_PROPERTY_NAME as isize)); i += 1;
      ASET (menu_bar_items_vector, i as isize, list1 (item)); i += 1;
      ASET (menu_bar_items_vector, i as isize, make_fixnum (0)); i += 1;
      menu_bar_items_index = i;
    }
  /* We did find an item for this KEY.  Add ITEM to its list of maps.  */
  else
    {
      let mut old: Lisp_Object;
      old = AREF (menu_bar_items_vector, (i + 2) as isize);
      /* If the new and the old items are not both keymaps,
	 the lookup will only find `item'.  */
      item = Fcons (item, if KEYMAPP! (item) && KEYMAPP! (XCAR (old)) { old } else { Qnil () });
      ASET (menu_bar_items_vector, (i + 2) as isize, item);
    }
}

 /* This is used as the handler when calling menu_item_eval_property.  */
/* C 8859-8868: `static Lisp_Object menu_item_eval_property_1 (Lisp_Object arg);`  */
unsafe extern "C" fn menu_item_eval_property_1 (arg: Lisp_Object) -> Lisp_Object
{
  /* If we got a quit from within the menu computation,
     quit all the way out of it.  This takes care of C-] in the debugger.  */
  if signal_quit_p (arg)
    { quit (); }

  return Qnil ();
}

/* C 8870-8874: `static Lisp_Object eval_dyn (Lisp_Object form);`  */
unsafe extern "C" fn eval_dyn (form: Lisp_Object) -> Lisp_Object
{
  return Feval (form, Qnil ());
}

/* Evaluate an expression and return the result (or nil if something
   went wrong).  Used to evaluate dynamic parts of menu items.  */
/* C 8878-8887: `Lisp_Object menu_item_eval_property (Lisp_Object sexpr);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn menu_item_eval_property (sexpr: Lisp_Object) -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut val: Lisp_Object;
  specbind (Qinhibit_redisplay (), Qt ());
  val = internal_condition_case_1 (eval_dyn, sexpr, Qerror (),
				   menu_item_eval_property_1);
  return unbind_to (count, val);
}


/* This function parses a menu item and leaves the result in the
   vector item_properties.
   ITEM is a key binding, a possible menu item.
   INMENUBAR is > 0 when this is considered for an entry in a menu bar
   top level.
   INMENUBAR is < 0 when this is considered for an entry in a keyboard menu.
   parse_menu_item returns true if the item is a menu item and false
   otherwise.  */

/* C 8898-9174: `bool parse_menu_item (Lisp_Object item, int inmenubar);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_menu_item (mut item: Lisp_Object, inmenubar: c_int) -> bool
{
  let mut def: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut item_string: Lisp_Object;
  let mut start: Lisp_Object;
  let mut filter: Lisp_Object;
  let mut keyhint: Lisp_Object;
  let mut i: c_int;

  filter = Qnil ();
  keyhint = Qnil ();

  if !CONSP (item)
    { return false; }

  /* Create item_properties vector if necessary.  */
  if NILP (item_properties)
    { item_properties = make_nil_vector ((ITEM_PROPERTY_MAX + 1) as isize); }

  /* Initialize optional entries.  */
  i = ITEM_PROPERTY_DEF;
  while i <= ITEM_PROPERTY_MAX
    {
      ASET (item_properties, i as isize, Qnil ());
      i += 1;
    }
  ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, Qt ());

  /* Save the item here to protect it from GC.  */
  ASET (item_properties, ITEM_PROPERTY_ITEM as isize, item);

  item_string = XCAR (item);

  start = item;
  item = XCDR (item);
  if STRINGP (item_string)
    {
      /* Old format menu item.  */
      ASET (item_properties, ITEM_PROPERTY_NAME as isize, item_string);

      /* Maybe help string.  */
      if CONSP (item) && STRINGP (XCAR (item))
	{
	  ASET (item_properties, ITEM_PROPERTY_HELP as isize,
		help_echo_substitute_command_keys (XCAR (item)));
	  start = item;
	  item = XCDR (item);
	}

      /* Maybe an obsolete key binding cache.  */
      if CONSP (item) && CONSP (XCAR (item))
	  && (NILP (XCAR (XCAR (item)))
	      || VECTORP (XCAR (XCAR (item))))
	{ item = XCDR (item); }

      /* This is the real definition--the function to run.  */
      ASET (item_properties, ITEM_PROPERTY_DEF as isize, item);

      /* Get enable property, if any.  */
      if SYMBOLP (item)
	{
	  tem = Fget (item, Qmenu_enable ());
	  if !NILP (Venable_disabled_menus_and_buttons)
	    { ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, Qt ()); }
	  else if !NILP (tem)
	    { ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, tem); }
	}
    }
  else if EQ (item_string, Qmenu_item ()) && CONSP (item)
    {
      /* New format menu item.  */
      ASET (item_properties, ITEM_PROPERTY_NAME as isize, XCAR (item));
      start = XCDR (item);
      if CONSP (start)
	{
	  /* We have a real binding.  */
	  ASET (item_properties, ITEM_PROPERTY_DEF as isize, XCAR (start));

	  item = XCDR (start);
	  /* Is there an obsolete cache list with key equivalences.  */
	  if CONSP (item) && CONSP (XCAR (item))
	    { item = XCDR (item); }

	  /* Parse properties.  */
	  FOR_EACH_TAIL! (item, {
	    tem = XCAR (item);
	    item = XCDR (item);
	    if !CONSP (item)
	      { break; }

	    if EQ (tem, QCenable ())
	      {
		if !NILP (Venable_disabled_menus_and_buttons)
		  { ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, Qt ()); }
		else
		  { ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, XCAR (item)); }
	      }
	    else if EQ (tem, QCvisible ())
	      {
		/* If got a visible property and that evaluates to nil
		   then ignore this item.  */
		tem = menu_item_eval_property (XCAR (item));
		if NILP (tem)
		  { return false; }
	 	}
	    else if EQ (tem, QChelp ())
	      {
		let mut help: Lisp_Object = XCAR (item);
		if STRINGP (help)
		  { help = help_echo_substitute_command_keys (help); }
		ASET (item_properties, ITEM_PROPERTY_HELP as isize, help);
	      }
	    else if EQ (tem, QCfilter ())
	      { filter = item; }
	    else if EQ (tem, QCkey_sequence ())
	      {
		tem = XCAR (item);
		if SYMBOLP (tem) || STRINGP (tem) || VECTORP (tem)
		  /* Be GC protected. Set keyhint to item instead of tem.  */
		  { keyhint = item; }
	      }
	    else if EQ (tem, QCkeys ())
	      {
		tem = XCAR (item);
		if FUNCTIONP (tem)
		  { ASET (item_properties, ITEM_PROPERTY_KEYEQ as isize, call0 (tem)); }
		else if CONSP (tem) || STRINGP (tem)
		  { ASET (item_properties, ITEM_PROPERTY_KEYEQ as isize, tem); }
	      }
	    else if EQ (tem, QCbutton ()) && CONSP (XCAR (item))
	      {
		let mut type_: Lisp_Object;
		tem = XCAR (item);
		type_ = XCAR (tem);
		if EQ (type_, QCtoggle ()) || EQ (type_, QCradio ())
		  {
		    ASET (item_properties, ITEM_PROPERTY_SELECTED as isize,
			  XCDR (tem));
		    ASET (item_properties, ITEM_PROPERTY_TYPE as isize, type_);
		  }
	      }
	  });
	}
      else if inmenubar != 0 || !NILP (start)
	{ return false; }
    }
  else
    { return false; }			/* not a menu item */

  /* If item string is not a string, evaluate it to get string.
     If we don't get a string, skip this item.  */
  item_string = AREF (item_properties, ITEM_PROPERTY_NAME as isize);
  if !(STRINGP (item_string))
    {
      item_string = menu_item_eval_property (item_string);
      if !STRINGP (item_string)
	{ return false; }
      ASET (item_properties, ITEM_PROPERTY_NAME as isize, item_string);
    }

  /* If got a filter apply it on definition.  */
  def = AREF (item_properties, ITEM_PROPERTY_DEF as isize);
  if !NILP (filter)
    {
      def = menu_item_eval_property (list2 (XCAR (filter), list2 (Qquote (), def)));

      ASET (item_properties, ITEM_PROPERTY_DEF as isize, def);
    }

  /* Enable or disable selection of item.  */
  tem = AREF (item_properties, ITEM_PROPERTY_ENABLE as isize);
  if !EQ (tem, Qt ())
    {
      tem = menu_item_eval_property (tem);
      if inmenubar != 0 && NILP (tem)
	{ return false; }		/* Ignore disabled items in menu bar.  */
      ASET (item_properties, ITEM_PROPERTY_ENABLE as isize, tem);
    }

  /* If we got no definition, this item is just unselectable text which
     is OK in a submenu but not in the menubar.  */
  if NILP (def)
    { return !(inmenubar != 0); }

  /* See if this is a separate pane or a submenu.  */
  def = AREF (item_properties, ITEM_PROPERTY_DEF as isize);
  tem = get_keymap (def, false, true);
  /* For a subkeymap, just record its details and exit.  */
  if CONSP (tem)
    {
      ASET (item_properties, ITEM_PROPERTY_MAP as isize, tem);
      ASET (item_properties, ITEM_PROPERTY_DEF as isize, tem);
      return true;
    }

  /* At the top level in the menu bar, do likewise for commands also.
     The menu bar does not display equivalent key bindings anyway.
     ITEM_PROPERTY_DEF is already set up properly.  */
  if inmenubar > 0
    { return true; }

  { /* This is a command.  See if there is an equivalent key binding.  */
    let mut keyeq: Lisp_Object = AREF (item_properties, ITEM_PROPERTY_KEYEQ as isize);
    AUTO_STRING! (space_space, c"  ".as_ptr ());

    /* The previous code preferred :key-sequence to :keys, so we
       preserve this behavior.  */
    if STRINGP (keyeq) && !CONSP (keyhint)
      { keyeq = concat2 (space_space, calln! (Qsubstitute_command_keys (), keyeq)); }
    else
      {
	let mut prefix: Lisp_Object = keyeq;
	let mut keys: Lisp_Object = Qnil ();

	if CONSP (prefix)
	  {
	    def = XCAR (prefix);
	    prefix = XCDR (prefix);
	  }
	else
	  { def = AREF (item_properties, ITEM_PROPERTY_DEF as isize); }

	if CONSP (keyhint) && !NILP (XCAR (keyhint))
	  {
	    keys = XCAR (keyhint);
	    tem = Fkey_binding (keys, Qnil (), Qnil (), Qnil ());

	    /* We have a suggested key.  Is it bound to the command?  */
	    if NILP (tem)
		|| (!EQ (tem, def)
		    /* If the command is an alias for another
		       (such as lmenu.el set it up), check if the
		       original command matches the cached command.  */
		    && !(SYMBOLP (def)
			 && EQ (tem, (*XSYMBOL (def)).u.s.function)))
	      { keys = Qnil (); }
	  }

	if NILP (keys)
	  { keys = Fwhere_is_internal (def, Qnil (), Qt (), Qnil (), Qnil ()); }

	if !NILP (keys)
	  {
	    tem = Fkey_description (keys, Qnil ());
	    if CONSP (prefix)
	      {
		if STRINGP (XCAR (prefix))
		  { tem = concat2 (XCAR (prefix), tem); }
		if STRINGP (XCDR (prefix))
		  { tem = concat2 (tem, XCDR (prefix)); }
	      }
	    keyeq = concat2 (space_space, tem);
	  }
	else
	  { keyeq = Qnil (); }
      }

    /* If we have an equivalent key binding, use that.  */
    ASET (item_properties, ITEM_PROPERTY_KEYEQ as isize, keyeq);
  }

  /* Include this when menu help is implemented.
  tem = XVECTOR (item_properties)->contents[ITEM_PROPERTY_HELP];
  if (!(NILP (tem) || STRINGP (tem)))
    {
      tem = menu_item_eval_property (tem);
      if (!STRINGP (tem))
	tem = Qnil;
      XVECTOR (item_properties)->contents[ITEM_PROPERTY_HELP] = tem;
    }
  */

  /* Handle radio buttons or toggle boxes.  */
  tem = AREF (item_properties, ITEM_PROPERTY_SELECTED as isize);
  if !NILP (tem)
    { ASET (item_properties, ITEM_PROPERTY_SELECTED as isize,
	    menu_item_eval_property (tem)); }

  return true;
}


/***********************************************************************
			       Tab-bars
 ***********************************************************************/

/* A vector holding tab bar items while they are parsed in function
   tab_bar_items. Each item occupies TAB_BAR_ITEM_NSCLOTS elements
   in the vector.  */

/* C 9186: `static Lisp_Object tab_bar_items_vector;`  */
static mut tab_bar_items_vector: Lisp_Object = lisp_h_Qnil ();

/* A vector holding the result of parse_tab_bar_item.  Layout is like
   the one for a single item in tab_bar_items_vector.  */

/* C 9191: `static Lisp_Object tab_bar_item_properties;`  */
static mut tab_bar_item_properties: Lisp_Object = lisp_h_Qnil ();

/* Next free index in tab_bar_items_vector.  */

/* C 9195: `static int ntab_bar_items;`  */
static mut ntab_bar_items: c_int = 0;

/* Function prototypes.  */
/* C 9199-9203: the four `static` prototypes (`init_tab_bar_items`, `process_tab_bar_item`,
   `parse_tab_bar_item`, `append_tab_bar_item`) are dropped (PORTING.md 0.1-12; each is defined
   below).  */


/* Return a vector of tab bar items for keymaps currently in effect.
   Reuse vector REUSE if non-nil.  Return in *NITEMS the number of
   tab bar items found.  */

/* C 9210-9291: `Lisp_Object tab_bar_items (Lisp_Object reuse, int *nitems);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tab_bar_items (reuse: Lisp_Object, nitems: *mut c_int) -> Lisp_Object
{
  let mut maps: *mut Lisp_Object = std::ptr::null_mut ();
  let mut mapsbuf: [Lisp_Object; 3] = [Qnil (); 3];
  let mut nmaps: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut oquit: Lisp_Object;
  let mut tmaps: *mut Lisp_Object = std::ptr::null_mut ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  *nitems = 0;

  /* In order to build the menus, we need to call the keymap
     accessors.  They all call maybe_quit.  But this function is called
     during redisplay, during which a quit is fatal.  So inhibit
     quitting while building the menus.  We do this instead of
     specbind because (1) errors will clear it anyway and (2) this
     avoids risk of specpdl overflow.  */
  oquit = Vinhibit_quit;
  Vinhibit_quit = Qt ();

  /* Initialize tab_bar_items_vector and protect it from GC.  */
  init_tab_bar_items (reuse);

  /* Build list of keymaps in maps.  Set nmaps to the number of maps
     to process.  */

  /* Should overriding-terminal-local-map and overriding-local-map apply?  */
  if !NILP (Voverriding_local_map_menu_flag)
      && !NILP (Voverriding_local_map)
    {
      /* Yes, use them (if non-nil) as well as the global map.  */
      maps = mapsbuf.as_mut_ptr ();
      nmaps = 0;
      if !NILP (KVAR! (current_kboard, Voverriding_terminal_local_map))
	{ *maps.offset (nmaps) = KVAR! (current_kboard, Voverriding_terminal_local_map);
	  nmaps += 1; }
      if !NILP (Voverriding_local_map)
	{ *maps.offset (nmaps) = Voverriding_local_map; nmaps += 1; }
    }
  else
    {
      /* No, so use major and minor mode keymaps and keymap property.
	 Note that tab-bar bindings in the local-map and keymap
	 properties may not work reliably, as they are only
	 recognized when the tab-bar (or mode-line) is updated,
	 which does not normally happen after every command.  */
      let nminor: ptrdiff_t = current_minor_maps (std::ptr::null_mut (), &raw mut tmaps);
      SAFE_NALLOCA! (sa_avail, maps, 1, nminor + 4);
      nmaps = 0;
      let mut tem: Lisp_Object = KVAR! (current_kboard, Voverriding_terminal_local_map);
      if !NILP (tem) && !NILP (Voverriding_local_map_menu_flag)
	{ *maps.offset (nmaps) = tem; nmaps += 1; }
      /* C's `if (tem = get_local_map (PT, current_buffer, Qkeymap), !NILP (tem))`.  */
      if { tem = get_local_map (PT! (), current_buffer, Qkeymap ()); !NILP (tem) }
	{ *maps.offset (nmaps) = tem; nmaps += 1; }
      if nminor != 0
	{
	  memcpy (maps.offset (nmaps) as *mut c_void, tmaps as *const c_void,
		  (nminor * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize);
	  nmaps += nminor;
	}
      *maps.offset (nmaps) = get_local_map (PT! (), current_buffer, Qlocal_map ());
      nmaps += 1;
    }

  /* Add global keymap at the end.  */
  *maps.offset (nmaps) = current_global_map;
  nmaps += 1;

  /* Process maps in reverse order and look up in each map the prefix
     key `tab-bar'.  */
  i = nmaps - 1;
  while i >= 0
    {
      if !NILP (*maps.offset (i))
	{
	  let mut keymap: Lisp_Object;

	  keymap = get_keymap (access_keymap (*maps.offset (i), Qtab_bar (), true, false),
			       false, true);
	  if CONSP (keymap)
	    { map_keymap (keymap, Some (process_tab_bar_item), Qnil (),
			  std::ptr::null_mut (), true); }
	}
      i -= 1;
    }

  Vinhibit_quit = oquit;
  *nitems = ntab_bar_items / TAB_BAR_ITEM_NSLOTS;
  SAFE_FREE! (sa_count);
  return tab_bar_items_vector;
}


/* Process the definition of KEY which is DEF.  */

/* C 9296-9324: `static void process_tab_bar_item (Lisp_Object key, Lisp_Object def,
   Lisp_Object data, void *args);`  */
unsafe extern "C" fn process_tab_bar_item (key: Lisp_Object, def: Lisp_Object,
					   data: Lisp_Object, args: *mut c_void)
{
  let mut i: c_int;

  if EQ (def, Qundefined ())
    {
      /* If a map has an explicit `undefined' as definition,
	 discard any previously made item.  */
      i = 0;
      while i < ntab_bar_items
	{
	  let v: *mut Lisp_Object
	    = (*XVECTOR (tab_bar_items_vector)).contents.as_mut_ptr ().offset (i as isize);

	  if EQ (key, *v.offset (TAB_BAR_ITEM_KEY as isize))
	    {
	      if ntab_bar_items > i + TAB_BAR_ITEM_NSLOTS
		{ memmove (v as *mut c_void, v.offset (TAB_BAR_ITEM_NSLOTS as isize) as *const c_void,
			   (((ntab_bar_items - i - TAB_BAR_ITEM_NSLOTS) as isize) * word_size)
			   as usize); }
	      ntab_bar_items -= TAB_BAR_ITEM_NSLOTS;
	      break;
	    }
	  i += TAB_BAR_ITEM_NSLOTS;
	}
    }
  else if parse_tab_bar_item (key, def)
    /* Append a new tab bar item to tab_bar_items_vector.  Accept
       more than one definition for the same key.  */
    { append_tab_bar_item (); }
}

/* Access slot with index IDX of vector tab_bar_item_properties.  */
/* C 9327: `#define PROP(IDX) AREF (tab_bar_item_properties, IDX)` — a function-scope
   `macro_rules!` whose textual scope ends with `parse_tab_bar_item`, the C `#undef` at C 9519;
   C defines `PROP` a second time for the tool bar (C 9711), so the two live in different
   functions rather than colliding at file scope.  */

/* C 9328-9332: `static void set_prop_tab_bar (ptrdiff_t idx, Lisp_Object val);`  */
unsafe fn set_prop_tab_bar (idx: ptrdiff_t, val: Lisp_Object)
{
  ASET (tab_bar_item_properties, idx, val);
}


/* Parse a tab bar item specification ITEM for key KEY and return the
   result in tab_bar_item_properties.  Value is false if ITEM is
   invalid.

   ITEM is a list `(menu-item CAPTION BINDING PROPS...)'.

   CAPTION is the caption of the item,  If it's not a string, it is
   evaluated to get a string.

   BINDING is the tab bar item's binding.  Tab-bar items with keymaps
   as binding are currently ignored.

   The following properties are recognized:

   - `:enable FORM'.

   FORM is evaluated and specifies whether the tab bar item is
   enabled or disabled.

   - `:visible FORM'

   FORM is evaluated and specifies whether the tab bar item is visible.

   - `:filter FUNCTION'

   FUNCTION is invoked with one parameter `(quote BINDING)'.  Its
   result is stored as the new binding.

   - `:button (TYPE SELECTED)'

   TYPE must be one of `:radio' or `:toggle'.  SELECTED is evaluated
   and specifies whether the button is selected (pressed) or not.

   - `:image IMAGES'

   IMAGES is either a single image specification or a vector of four
   image specifications.  See enum tab_bar_item_images.

   - `:help HELP-STRING'.

   Gives a help string to display for the tab bar item.

   - `:label LABEL-STRING'.

   A text label to show with the tab bar button if labels are enabled.  */

/* C 9381-9520: `static bool parse_tab_bar_item (Lisp_Object key, Lisp_Object item);`  */
unsafe fn parse_tab_bar_item (key: Lisp_Object, mut item: Lisp_Object) -> bool
{
  let mut filter: Lisp_Object = Qnil ();
  let mut caption: Lisp_Object;
  let mut i: c_int;

  /* C 9327's `PROP` (its `#undef` is C 9519).  */
  macro_rules! PROP { ($idx:expr) => { AREF (tab_bar_item_properties, $idx as isize) }; }

  /* Definition looks like `(menu-item CAPTION BINDING PROPS...)'.
     Rule out items that aren't lists, don't start with
     `menu-item' or whose rest following `tab-bar-item' is not a
     list.  */
  if !CONSP (item)
    { return false; }

  /* As an exception, allow old-style menu separators.  */
  if STRINGP (XCAR (item))
    { item = list1 (XCAR (item)); }
  else if !EQ (XCAR (item), Qmenu_item ())
	   || { item = XCDR (item); !CONSP (item) }
    { return false; }

  /* Create tab_bar_item_properties vector if necessary.  Reset it to
     defaults.  */
  if VECTORP (tab_bar_item_properties)
    {
      i = 0;
      while i < TAB_BAR_ITEM_NSLOTS
	{
	  set_prop_tab_bar (i as ptrdiff_t, Qnil ());
	  i += 1;
	}
    }
  else
    { tab_bar_item_properties = make_nil_vector (TAB_BAR_ITEM_NSLOTS as isize); }

  /* Set defaults.  */
  set_prop_tab_bar (TAB_BAR_ITEM_KEY as ptrdiff_t, key);
  set_prop_tab_bar (TAB_BAR_ITEM_ENABLED_P as ptrdiff_t, Qt ());

  /* Get the caption of the item.  If the caption is not a string,
     evaluate it to get a string.  If we don't get a string, skip this
     item.  */
  caption = XCAR (item);
  if !STRINGP (caption)
    {
      caption = menu_item_eval_property (caption);
      if !STRINGP (caption)
	{ return false; }
    }
  set_prop_tab_bar (TAB_BAR_ITEM_CAPTION as ptrdiff_t, caption);

  /* If the rest following the caption is not a list, the menu item is
     either a separator, or invalid.  */
  item = XCDR (item);
  if !CONSP (item)
    {
      if menu_separator_name_p (SSDATA (caption))
	{
	  set_prop_tab_bar (TAB_BAR_ITEM_ENABLED_P as ptrdiff_t, Qnil ());
	  set_prop_tab_bar (TAB_BAR_ITEM_SELECTED_P as ptrdiff_t, Qnil ());
	  set_prop_tab_bar (TAB_BAR_ITEM_CAPTION as ptrdiff_t, Qnil ());
	  return true;
	}
      return false;
    }

  /* Store the binding.  */
  set_prop_tab_bar (TAB_BAR_ITEM_BINDING as ptrdiff_t, XCAR (item));
  item = XCDR (item);

  /* Ignore cached key binding, if any.  */
  if CONSP (item) && CONSP (XCAR (item))
    { item = XCDR (item); }

  /* Process the rest of the properties.  */
  FOR_EACH_TAIL! (item, {
    let ikey: Lisp_Object = XCAR (item);
    item = XCDR (item);
    if !CONSP (item)
      { break; }
    let value: Lisp_Object = XCAR (item);

    if EQ (ikey, QCenable ())
      {
	/* `:enable FORM'.  */
	if !NILP (Venable_disabled_menus_and_buttons)
	  { set_prop_tab_bar (TAB_BAR_ITEM_ENABLED_P as ptrdiff_t, Qt ()); }
	else
	  { set_prop_tab_bar (TAB_BAR_ITEM_ENABLED_P as ptrdiff_t, value); }
      }
    else if EQ (ikey, QCvisible ())
      {
	/* `:visible FORM'.  If got a visible property and that
	   evaluates to nil then ignore this item.  */
	if NILP (menu_item_eval_property (value))
	  { return false; }
      }
    else if EQ (ikey, QChelp ())
      /* `:help HELP-STRING'.  */
      { set_prop_tab_bar (TAB_BAR_ITEM_HELP as ptrdiff_t, value); }
    else if EQ (ikey, QCfilter ())
      /* ':filter FORM'.  */
      { filter = value; }
    else if EQ (ikey, QCbutton ()) && CONSP (value)
      {
	/* `:button (TYPE . SELECTED)'.  */
	let mut type_: Lisp_Object;
	let mut selected: Lisp_Object;

	type_ = XCAR (value);
	selected = XCDR (value);
	if EQ (type_, QCtoggle ()) || EQ (type_, QCradio ())
	  {
	    set_prop_tab_bar (TAB_BAR_ITEM_SELECTED_P as ptrdiff_t, selected);
	  }
      }
  });

  /* If got a filter apply it on binding.  */
  if !NILP (filter)
    { set_prop_tab_bar (TAB_BAR_ITEM_BINDING as ptrdiff_t,
			(menu_item_eval_property
			 (list2 (filter,
				 list2 (Qquote (),
					PROP! (TAB_BAR_ITEM_BINDING)))))); }

  /* See if the binding is a keymap.  Give up if it is.  */
  if CONSP (get_keymap (PROP! (TAB_BAR_ITEM_BINDING), false, true))
    { return false; }

  /* Enable or disable selection of item.  */
  if !EQ (PROP! (TAB_BAR_ITEM_ENABLED_P), Qt ())
    { set_prop_tab_bar (TAB_BAR_ITEM_ENABLED_P as ptrdiff_t,
			menu_item_eval_property (PROP! (TAB_BAR_ITEM_ENABLED_P))); }

  /* Handle radio buttons or toggle boxes.  */
  if !NILP (PROP! (TAB_BAR_ITEM_SELECTED_P))
    { set_prop_tab_bar (TAB_BAR_ITEM_SELECTED_P as ptrdiff_t,
			menu_item_eval_property (PROP! (TAB_BAR_ITEM_SELECTED_P))); }

  return true;
  /* C 9519: `#undef PROP`.  */
}


/* Initialize tab_bar_items_vector.  REUSE, if non-nil, is a vector
   that can be reused.  */

/* C 9526-9534: `static void init_tab_bar_items (Lisp_Object reuse);`  */
unsafe fn init_tab_bar_items (reuse: Lisp_Object)
{
  if VECTORP (reuse)
    { tab_bar_items_vector = reuse; }
  else
    { tab_bar_items_vector = make_nil_vector (64); }
  ntab_bar_items = 0;
}


/* Append parsed tab bar item properties from
   tab_bar_item_properties */

/* C 9540-9556: `static void append_tab_bar_item (void);`  */
unsafe fn append_tab_bar_item ()
{
  let incr: ptrdiff_t
    = (ntab_bar_items as isize
       - (ASIZE (tab_bar_items_vector) - TAB_BAR_ITEM_NSLOTS as isize));

  /* Enlarge tab_bar_items_vector if necessary.  */
  if incr > 0
    { tab_bar_items_vector = larger_vector (tab_bar_items_vector, incr, -1); }

  /* Append entries from tab_bar_item_properties to the end of
     tab_bar_items_vector.  */
  vcopy (tab_bar_items_vector, ntab_bar_items as isize,
	 xvector_contents (tab_bar_item_properties) as *const Lisp_Object,
	 TAB_BAR_ITEM_NSLOTS as isize);
  ntab_bar_items += TAB_BAR_ITEM_NSLOTS;
}


/***********************************************************************
			       Tool-bars
 ***********************************************************************/

/* A vector holding tool bar items while they are parsed in function
   tool_bar_items. Each item occupies TOOL_BAR_ITEM_NSCLOTS elements
   in the vector.  */

/* C 9570: `static Lisp_Object tool_bar_items_vector;`  */
static mut tool_bar_items_vector: Lisp_Object = lisp_h_Qnil ();

/* A vector holding the result of parse_tool_bar_item.  Layout is like
   the one for a single item in tool_bar_items_vector.  */

/* C 9575: `static Lisp_Object tool_bar_item_properties;`  */
static mut tool_bar_item_properties: Lisp_Object = lisp_h_Qnil ();

/* Next free index in tool_bar_items_vector.  */

/* C 9579: `static int ntool_bar_items;`  */
static mut ntool_bar_items: c_int = 0;

/* Function prototypes.  */
/* C 9583-9587: the four `static` prototypes (`init_tool_bar_items`, `process_tool_bar_item`,
   `parse_tool_bar_item`, `append_tool_bar_item`) are dropped (PORTING.md 0.1-12; each is defined
   below).  */

/* dispextern.h 3451-3495: `enum tool_bar_item_idx` — `TOOL_BAR_ITEM_KEY` …
   `TOOL_BAR_ITEM_NSLOTS` — has no mirror, and this family is the text that names its members, so
   the type and the thirteen members stand at this use site (P-72③; PORTING.md 1.3's constant
   shape, the `tab_bar_item_idx` precedent above).  */
pub type tool_bar_item_idx = c_int;
pub const TOOL_BAR_ITEM_KEY: tool_bar_item_idx = 0;
pub const TOOL_BAR_ITEM_ENABLED_P: tool_bar_item_idx = 1;
pub const TOOL_BAR_ITEM_SELECTED_P: tool_bar_item_idx = 2;
pub const TOOL_BAR_ITEM_CAPTION: tool_bar_item_idx = 3;
pub const TOOL_BAR_ITEM_IMAGES: tool_bar_item_idx = 4;
pub const TOOL_BAR_ITEM_BINDING: tool_bar_item_idx = 5;
pub const TOOL_BAR_ITEM_TYPE: tool_bar_item_idx = 6;
pub const TOOL_BAR_ITEM_HELP: tool_bar_item_idx = 7;
pub const TOOL_BAR_ITEM_RTL_IMAGE: tool_bar_item_idx = 8;
pub const TOOL_BAR_ITEM_LABEL: tool_bar_item_idx = 9;
pub const TOOL_BAR_ITEM_VERT_ONLY: tool_bar_item_idx = 10;
pub const TOOL_BAR_ITEM_WRAP: tool_bar_item_idx = 11;
pub const TOOL_BAR_ITEM_NSLOTS: tool_bar_item_idx = 12;


/* Return a vector of tool bar items for keymaps currently in effect.
   Reuse vector REUSE if non-nil.  Return in *NITEMS the number of
   tool bar items found.  */

/* C 9594-9675: `Lisp_Object tool_bar_items (Lisp_Object reuse, int *nitems);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tool_bar_items (reuse: Lisp_Object, nitems: *mut c_int) -> Lisp_Object
{
  let mut maps: *mut Lisp_Object = std::ptr::null_mut ();
  let mut mapsbuf: [Lisp_Object; 3] = [Qnil (); 3];
  let mut nmaps: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut oquit: Lisp_Object;
  let mut tmaps: *mut Lisp_Object = std::ptr::null_mut ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  *nitems = 0;

  /* In order to build the menus, we need to call the keymap
     accessors.  They all call maybe_quit.  But this function is called
     during redisplay, during which a quit is fatal.  So inhibit
     quitting while building the menus.  We do this instead of
     specbind because (1) errors will clear it anyway and (2) this
     avoids risk of specpdl overflow.  */
  oquit = Vinhibit_quit;
  Vinhibit_quit = Qt ();

  /* Initialize tool_bar_items_vector and protect it from GC.  */
  init_tool_bar_items (reuse);

  /* Build list of keymaps in maps.  Set nmaps to the number of maps
     to process.  */

  /* Should overriding-terminal-local-map and overriding-local-map apply?  */
  if !NILP (Voverriding_local_map_menu_flag)
      && !NILP (Voverriding_local_map)
    {
      /* Yes, use them (if non-nil) as well as the global map.  */
      maps = mapsbuf.as_mut_ptr ();
      nmaps = 0;
      if !NILP (KVAR! (current_kboard, Voverriding_terminal_local_map))
	{ *maps.offset (nmaps) = KVAR! (current_kboard, Voverriding_terminal_local_map);
	  nmaps += 1; }
      if !NILP (Voverriding_local_map)
	{ *maps.offset (nmaps) = Voverriding_local_map; nmaps += 1; }
    }
  else
    {
      /* No, so use major and minor mode keymaps and keymap property.
	 Note that tool-bar bindings in the local-map and keymap
	 properties may not work reliably, as they are only
	 recognized when the tool-bar (or mode-line) is updated,
	 which does not normally happen after every command.  */
      let nminor: ptrdiff_t = current_minor_maps (std::ptr::null_mut (), &raw mut tmaps);
      SAFE_NALLOCA! (sa_avail, maps, 1, nminor + 4);
      nmaps = 0;
      let mut tem: Lisp_Object = KVAR! (current_kboard, Voverriding_terminal_local_map);
      if !NILP (tem) && !NILP (Voverriding_local_map_menu_flag)
	{ *maps.offset (nmaps) = tem; nmaps += 1; }
      /* C's `if (tem = get_local_map (PT, current_buffer, Qkeymap), !NILP (tem))`.  */
      if { tem = get_local_map (PT! (), current_buffer, Qkeymap ()); !NILP (tem) }
	{ *maps.offset (nmaps) = tem; nmaps += 1; }
      if nminor != 0
	{
	  memcpy (maps.offset (nmaps) as *mut c_void, tmaps as *const c_void,
		  (nminor * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize);
	  nmaps += nminor;
	}
      *maps.offset (nmaps) = get_local_map (PT! (), current_buffer, Qlocal_map ());
      nmaps += 1;
    }

  /* Add global keymap at the end.  */
  *maps.offset (nmaps) = current_global_map;
  nmaps += 1;

  /* Process maps in reverse order and look up in each map the prefix
     key `tool-bar'.  */
  i = nmaps - 1;
  while i >= 0
    {
      if !NILP (*maps.offset (i))
	{
	  let mut keymap: Lisp_Object;

	  keymap = get_keymap (access_keymap (*maps.offset (i), Qtool_bar (), true, false),
			       false, true);
	  if CONSP (keymap)
	    { map_keymap (keymap, Some (process_tool_bar_item), Qnil (),
			  std::ptr::null_mut (), true); }
	}
      i -= 1;
    }

  Vinhibit_quit = oquit;
  *nitems = ntool_bar_items / TOOL_BAR_ITEM_NSLOTS;
  SAFE_FREE! (sa_count);
  return tool_bar_items_vector;
}


/* Process the definition of KEY which is DEF.  */

/* C 9680-9708: `static void process_tool_bar_item (Lisp_Object key, Lisp_Object def,
   Lisp_Object data, void *args);`  */
unsafe extern "C" fn process_tool_bar_item (key: Lisp_Object, def: Lisp_Object,
					    data: Lisp_Object, args: *mut c_void)
{
  let mut i: c_int;

  if EQ (def, Qundefined ())
    {
      /* If a map has an explicit `undefined' as definition,
	 discard any previously made item.  */
      i = 0;
      while i < ntool_bar_items
	{
	  let v: *mut Lisp_Object
	    = (*XVECTOR (tool_bar_items_vector)).contents.as_mut_ptr ().offset (i as isize);

	  if EQ (key, *v.offset (TOOL_BAR_ITEM_KEY as isize))
	    {
	      if ntool_bar_items > i + TOOL_BAR_ITEM_NSLOTS
		{ memmove (v as *mut c_void, v.offset (TOOL_BAR_ITEM_NSLOTS as isize) as *const c_void,
			   (((ntool_bar_items - i - TOOL_BAR_ITEM_NSLOTS) as isize) * word_size)
			   as usize); }
	      ntool_bar_items -= TOOL_BAR_ITEM_NSLOTS;
	      break;
	    }
	  i += TOOL_BAR_ITEM_NSLOTS;
	}
    }
  else if parse_tool_bar_item (key, def)
    /* Append a new tool bar item to tool_bar_items_vector.  Accept
       more than one definition for the same key.  */
    { append_tool_bar_item (); }
}

/* Access slot with index IDX of vector tool_bar_item_properties.  */
/* C 9711: `#define PROP(IDX) AREF (tool_bar_item_properties, IDX)` (its `#undef` is C 10033;
   the second definition of the name, so it lives inside this family's parser).  */

/* C 9712-9716: `static void set_prop (ptrdiff_t idx, Lisp_Object val);`  */
unsafe fn set_prop (idx: ptrdiff_t, val: Lisp_Object)
{
  ASET (tool_bar_item_properties, idx, val);
}


/* Parse a tool bar item specification ITEM for key KEY and return the
   result in tool_bar_item_properties.  Value is false if ITEM is
   invalid.

   ITEM is a list `(menu-item CAPTION BINDING PROPS...)'.

   CAPTION is the caption of the item,  If it's not a string, it is
   evaluated to get a string.

   BINDING is the tool bar item's binding.  Tool-bar items with keymaps
   as binding are currently ignored.

   The following properties are recognized:

   - `:enable FORM'.

   FORM is evaluated and specifies whether the tool bar item is
   enabled or disabled.

   - `:visible FORM'

   FORM is evaluated and specifies whether the tool bar item is visible.

   - `:filter FUNCTION'

   FUNCTION is invoked with one parameter `(quote BINDING)'.  Its
   result is stored as the new binding.

   - `:button (TYPE SELECTED)'

   TYPE must be one of `:radio' or `:toggle'.  SELECTED is evaluated
   and specifies whether the button is selected (pressed) or not.

   - `:image IMAGES'

   IMAGES is either a single image specification or a vector of four
   image specifications.  See enum tool_bar_item_images.

   - `:help HELP-STRING'.

   Gives a help string to display for the tool bar item.

   - `:label LABEL-STRING'.

   A text label to show with the tool bar button if labels are
   enabled.

   - `:wrap WRAP'

   WRAP specifies whether to hide this item but display subsequent
   tool bar items on a new line.  */

/* C 9771-10034: `static bool parse_tool_bar_item (Lisp_Object key, Lisp_Object item);`  */
unsafe fn parse_tool_bar_item (key: Lisp_Object, mut item: Lisp_Object) -> bool
{
  let mut filter: Lisp_Object = Qnil ();
  let mut caption: Lisp_Object;
  let mut i: c_int;
  let mut have_label: bool;
  /* C 9778-9780: `#ifndef HAVE_EXT_TOOL_BAR` is live on this target (`HAVE_EXT_TOOL_BAR` is
     not defined here), so `is_wrap` is a live local.  */
  let mut is_wrap: bool;

  /* C 9711's `PROP` (its `#undef` is C 10033).  */
  macro_rules! PROP { ($idx:expr) => { AREF (tool_bar_item_properties, $idx as isize) }; }

  have_label = false;
  is_wrap = false;

  /* Definition looks like `(menu-item CAPTION BINDING PROPS...)'.
     Rule out items that aren't lists, don't start with
     `menu-item' or whose rest following `tool-bar-item' is not a
     list.  */
  if !CONSP (item)
    { return false; }

  /* As an exception, allow old-style menu separators.  */
  if STRINGP (XCAR (item))
    { item = list1 (XCAR (item)); }
  else if !EQ (XCAR (item), Qmenu_item ())
	   || { item = XCDR (item); !CONSP (item) }
    { return false; }

  /* Create tool_bar_item_properties vector if necessary.  Reset it to
     defaults.  */
  if VECTORP (tool_bar_item_properties)
    {
      i = 0;
      while i < TOOL_BAR_ITEM_NSLOTS
	{
	  set_prop (i as ptrdiff_t, Qnil ());
	  i += 1;
	}
    }
  else
    { tool_bar_item_properties = make_nil_vector (TOOL_BAR_ITEM_NSLOTS as isize); }

  /* Set defaults.  */
  set_prop (TOOL_BAR_ITEM_KEY as ptrdiff_t, key);
  set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t, Qt ());

  /* Get the caption of the item.  If the caption is not a string,
     evaluate it to get a string.  If we don't get a string, skip this
     item.  */
  caption = XCAR (item);
  if !STRINGP (caption)
    {
      caption = menu_item_eval_property (caption);
      if !STRINGP (caption)
	{ return false; }
    }
  set_prop (TOOL_BAR_ITEM_CAPTION as ptrdiff_t, caption);

  /* If the rest following the caption is not a list, the menu item is
     either a separator, or invalid.  */
  item = XCDR (item);
  if !CONSP (item)
    {
      if menu_separator_name_p (SSDATA (caption))
	{
	  set_prop (TOOL_BAR_ITEM_TYPE as ptrdiff_t, Qt ());
	  /* C 9835-9844: `#ifndef HAVE_EXT_TOOL_BAR` — live on this target, so this block is not
	     gated (the `#else` arm C does not compile is the `#else /* HAVE_EXT_TOOL_BAR */`
	     whose text is absent here).  */
	  /* If we use build_desired_tool_bar_string to render the
	     tool bar, the separator is rendered as an image.  */
	  set_prop (TOOL_BAR_ITEM_IMAGES as ptrdiff_t,
		    (menu_item_eval_property
		     (Vtool_bar_separator_image_expression)));
	  set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t, Qnil ());
	  set_prop (TOOL_BAR_ITEM_SELECTED_P as ptrdiff_t, Qnil ());
	  set_prop (TOOL_BAR_ITEM_CAPTION as ptrdiff_t, Qnil ());
	  return true;
	}
      return false;
    }

  /* Store the binding.  */
  set_prop (TOOL_BAR_ITEM_BINDING as ptrdiff_t, XCAR (item));
  item = XCDR (item);

  /* Ignore cached key binding, if any.  */
  if CONSP (item) && CONSP (XCAR (item))
    { item = XCDR (item); }

  /* Process the rest of the properties.  */
  FOR_EACH_TAIL! (item, {
    let ikey: Lisp_Object = XCAR (item);
    item = XCDR (item);
    if !CONSP (item)
      { break; }
    let value: Lisp_Object = XCAR (item);

    if EQ (ikey, QCenable ())
      {
	/* `:enable FORM'.  */
	if !NILP (Venable_disabled_menus_and_buttons)
	  { set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t, Qt ()); }
	else
	  { set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t, value); }
      }
    else if EQ (ikey, QCvisible ())
      {
	/* `:visible FORM'.  If got a visible property and that
	   evaluates to nil then ignore this item.  */
	if NILP (menu_item_eval_property (value))
	  { return false; }
      }
    else if EQ (ikey, QChelp ())
      /* `:help HELP-STRING'.  */
      { set_prop (TOOL_BAR_ITEM_HELP as ptrdiff_t, value); }
    else if EQ (ikey, QCvert_only ())
      /* `:vert-only t/nil'.  */
      { set_prop (TOOL_BAR_ITEM_VERT_ONLY as ptrdiff_t, value); }
    else if EQ (ikey, QClabel ())
      {
        let bad_label: *const c_char = c"!!?GARBLED ITEM?!!".as_ptr ();
        /* `:label LABEL-STRING'.  */
        set_prop (TOOL_BAR_ITEM_LABEL as ptrdiff_t,
		  if STRINGP (value) { value } else { build_string (bad_label) });
        have_label = true;
      }
    else if EQ (ikey, QCfilter ())
      /* ':filter FORM'.  */
      { filter = value; }
    else if EQ (ikey, QCbutton ()) && CONSP (value)
      {
	/* `:button (TYPE . SELECTED)'.  */
	let mut type_: Lisp_Object;
	let mut selected: Lisp_Object;

	type_ = XCAR (value);
	selected = XCDR (value);
	if EQ (type_, QCtoggle ()) || EQ (type_, QCradio ())
	  {
	    set_prop (TOOL_BAR_ITEM_SELECTED_P as ptrdiff_t, selected);
	    set_prop (TOOL_BAR_ITEM_TYPE as ptrdiff_t, type_);
	  }
      }
    else if EQ (ikey, QCimage ())
	     && (CONSP (value)
		 || (VECTORP (value) && ASIZE (value) == 4))
      /* Value is either a single image specification or a vector
	 of 4 such specifications for the different button states.  */
      { set_prop (TOOL_BAR_ITEM_IMAGES as ptrdiff_t, value); }
    else if EQ (ikey, QCrtl ())
      /* ':rtl STRING' */
      { set_prop (TOOL_BAR_ITEM_RTL_IMAGE as ptrdiff_t, value); }
    else if EQ (ikey, QCwrap ())
      {
	/* C 9923-9928: `#ifndef HAVE_EXT_TOOL_BAR` — live on this target.  */
	/* This specifies whether the tool bar item should be hidden
	   but cause subsequent items to be displayed on a new
	   line.  */
	set_prop (TOOL_BAR_ITEM_WRAP as ptrdiff_t, value);
	is_wrap = !NILP (value);
	/* C 9929-9932: `#else /* HAVE_EXT_TOOL_BAR */` (dead; carried per P-51).  */
	#[cfg (feature = "have_ext_tool_bar")]
	{ return false; }
      }
  });


  if !have_label
    {
      /* Try to make one from caption and key.  */
      let tkey: Lisp_Object = PROP! (TOOL_BAR_ITEM_KEY);
      let tcapt: Lisp_Object = PROP! (TOOL_BAR_ITEM_CAPTION);
      let mut label: *const c_char
	= if SYMBOLP (tkey) { SSDATA (SYMBOL_NAME (tkey)) } else { c"".as_ptr () };
      let mut capt: *const c_char
	= if STRINGP (tcapt) { SSDATA (tcapt) } else { c"".as_ptr () };
      let max_lbl_size: ptrdiff_t =
	2 * max! (0, min! (tool_bar_max_label_size, STRING_BYTES_BOUND / 2)) + 1;
      let mut buf: *mut c_char = xmalloc (max_lbl_size as usize) as *mut c_char;
      let mut new_lbl: Lisp_Object;
      let mut caption_len: ptrdiff_t = strnlen (capt, max_lbl_size as size_t) as ptrdiff_t;

      if 0 < caption_len && caption_len < max_lbl_size
        {
          strcpy (buf, capt);
          while caption_len > 0 && *buf.offset (caption_len - 1) == b'.' as c_char
            { caption_len -= 1; }
	  *buf.offset (caption_len) = 0;
	  /* C's `label = capt = buf` assigns right-to-left (PORTING.md 2.2).  */
	  capt = buf;
	  label = capt;
        }

      let label_len: ptrdiff_t = strnlen (label, max_lbl_size as size_t) as ptrdiff_t;
      if 0 < label_len && label_len < max_lbl_size
        {
          let mut j: ptrdiff_t;
          if label != buf
	    { strcpy (buf, label); }

          j = 0;
	  while *buf.offset (j) != 0
	    {
	      if *buf.offset (j) == b'-' as c_char
		{ *buf.offset (j) = b' ' as c_char; }
	      j += 1;
	    }
          label = buf;
        }
      else
	{ label = c"".as_ptr (); }

      new_lbl = Fupcase_initials (build_string (label));
      if SCHARS (new_lbl) <= tool_bar_max_label_size
        { set_prop (TOOL_BAR_ITEM_LABEL as ptrdiff_t, new_lbl); }
      else
        { set_prop (TOOL_BAR_ITEM_LABEL as ptrdiff_t, empty_unibyte_string); }
      xfree (buf as *mut c_void);
    }

  /* If got a filter apply it on binding.  */
  if !NILP (filter)
    { set_prop (TOOL_BAR_ITEM_BINDING as ptrdiff_t,
		(menu_item_eval_property
		 (list2 (filter,
			 list2 (Qquote (),
				PROP! (TOOL_BAR_ITEM_BINDING)))))); }

  /* See if the binding is a keymap.  Give up if it is.  */
  if CONSP (get_keymap (PROP! (TOOL_BAR_ITEM_BINDING), false, true))
    { return false; }


  /* C 9996-10002: `#ifndef HAVE_EXT_TOOL_BAR` — live on this target.  */
  /* If the menu item is actually a line wrap, make sure it isn't
     visible or enabled.  */

  if is_wrap
    { set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t, Qnil ()); }

  /* If there is a key binding, add it to the help, which will be
     displayed as a tooltip for this entry. */
  let binding: Lisp_Object = PROP! (TOOL_BAR_ITEM_BINDING);
  let keys: Lisp_Object = Fwhere_is_internal (binding, Qnil (), Qt (), Qnil (), Qnil ());
  if !NILP (keys)
    {
      AUTO_STRING! (beg, c"  (".as_ptr ());
      AUTO_STRING! (end, c")".as_ptr ());
      let mut orig: Lisp_Object = PROP! (TOOL_BAR_ITEM_HELP);
      let desc: Lisp_Object = Fkey_description (keys, Qnil ());

      if NILP (orig)
        { orig = PROP! (TOOL_BAR_ITEM_CAPTION); }

      set_prop (TOOL_BAR_ITEM_HELP as ptrdiff_t, CALLN! (Fconcat, orig, beg, desc, end));
    }

  /* Enable or disable selection of item.  */
  if !EQ (PROP! (TOOL_BAR_ITEM_ENABLED_P), Qt ())
    { set_prop (TOOL_BAR_ITEM_ENABLED_P as ptrdiff_t,
		menu_item_eval_property (PROP! (TOOL_BAR_ITEM_ENABLED_P))); }

  /* Handle radio buttons or toggle boxes.  */
  if !NILP (PROP! (TOOL_BAR_ITEM_SELECTED_P))
    { set_prop (TOOL_BAR_ITEM_SELECTED_P as ptrdiff_t,
		menu_item_eval_property (PROP! (TOOL_BAR_ITEM_SELECTED_P))); }

  return true;
  /* C 10033: `#undef PROP`.  */
}


/* Initialize tool_bar_items_vector.  REUSE, if non-nil, is a vector
   that can be reused.  */

/* C 10040-10048: `static void init_tool_bar_items (Lisp_Object reuse);`  */
unsafe fn init_tool_bar_items (reuse: Lisp_Object)
{
  if VECTORP (reuse)
    { tool_bar_items_vector = reuse; }
  else
    { tool_bar_items_vector = make_nil_vector (64); }
  ntool_bar_items = 0;
}


/* Append parsed tool bar item properties from
   tool_bar_item_properties */

/* C 10054-10070: `static void append_tool_bar_item (void);`  */
unsafe fn append_tool_bar_item ()
{
  let incr: ptrdiff_t
    = (ntool_bar_items as isize
       - (ASIZE (tool_bar_items_vector) - TOOL_BAR_ITEM_NSLOTS as isize));

  /* Enlarge tool_bar_items_vector if necessary.  */
  if incr > 0
    { tool_bar_items_vector = larger_vector (tool_bar_items_vector, incr, -1); }

  /* Append entries from tool_bar_item_properties to the end of
     tool_bar_items_vector.  */
  vcopy (tool_bar_items_vector, ntool_bar_items as isize,
	 xvector_contents (tool_bar_item_properties) as *const Lisp_Object,
	 TOOL_BAR_ITEM_NSLOTS as isize);
  ntool_bar_items += TOOL_BAR_ITEM_NSLOTS;
}


/* Read a character using menus based on the keymap MAP.
   Return nil if there are no menus in the maps.
   Return t if we displayed a menu but the user rejected it.

   PREV_EVENT is the previous input event, or nil if we are reading
   the first event of a key sequence.

   If USED_MOUSE_MENU is non-null, set *USED_MOUSE_MENU to true
   if we used a mouse menu to read the input, or false otherwise.  If
   USED_MOUSE_MENU is null, don't dereference it.

   The prompting is done based on the prompt-string of the map
   and the strings associated with various map elements.

   This can be done with X menus or with menus put in the minibuf.
   These are done in different ways, depending on how the input will be read.
   Menus using X are done after auto-saving in read-char, getting the input
   event from Fx_popup_menu; menus using the minibuf use read_char recursively
   and do auto-saving in the inner call of read_char.  */

/* C 10096-10155: `static Lisp_Object read_char_x_menu_prompt (Lisp_Object map,
   Lisp_Object prev_event, bool *used_mouse_menu);`  */
unsafe fn read_char_x_menu_prompt (map: Lisp_Object, prev_event: Lisp_Object,
				   used_mouse_menu: *mut bool) -> Lisp_Object
{
  if !used_mouse_menu.is_null ()
    { *used_mouse_menu = false; }

  /* Use local over global Menu maps.  */

  if ! menu_prompting
    { return Qnil (); }

  /* If we got to this point via a mouse click,
     use a real menu for mouse selection.  */
  if EVENT_HAS_PARAMETERS (prev_event)
      && !EQ (XCAR (prev_event), Qmenu_bar ())
      && !EQ (XCAR (prev_event), Qtab_bar ())
      && !EQ (XCAR (prev_event), Qtool_bar ())
    {
      /* Display the menu and get the selection.  */
      let mut value: Lisp_Object;

      value = x_popup_menu_1 (prev_event, get_keymap (map, false, true));
      if CONSP (value)
	{
	  let mut tem: Lisp_Object;

	  record_menu_key (XCAR (value));

	  /* If we got multiple events, unread all but
	     the first.
	     There is no way to prevent those unread events
	     from showing up later in last_nonmenu_event.
	     So turn symbol and integer events into lists,
	     to indicate that they came from a mouse menu,
	     so that when present in last_nonmenu_event
	     they won't confuse things.  */
	  tem = XCDR (value);
	  while CONSP (tem)
	    {
	      record_menu_key (XCAR (tem));
	      if SYMBOLP (XCAR (tem))
		  || FIXNUMP (XCAR (tem))
		{ XSETCAR (tem, Fcons (XCAR (tem), Qdisabled ())); }
	      tem = XCDR (tem);
	    }

	  /* If we got more than one event, put all but the first
	     onto this list to be read later.
	     Return just the first event now.  */
	  Vunread_command_events
	    = nconc2 (XCDR (value), Vunread_command_events);
	  value = XCAR (value);
	}
      else if NILP (value)
	{ value = Qt (); }
      if !used_mouse_menu.is_null ()
	{ *used_mouse_menu = true; }
      return value;
    }
  return Qnil ();
}

/* C 10157-10162: `static Lisp_Object follow_key (Lisp_Object keymap, Lisp_Object key);`  */
unsafe fn follow_key (keymap: Lisp_Object, key: Lisp_Object) -> Lisp_Object
{
  return access_keymap (get_keymap (keymap, false, true),
			key, true, false);
}

/* C 10164-10386: `static Lisp_Object read_char_minibuf_menu_prompt (int commandflag,
   Lisp_Object map);`  */
unsafe fn read_char_minibuf_menu_prompt (commandflag: c_int, mut map: Lisp_Object)
  -> Lisp_Object
{
  let mut name: Lisp_Object;
  let mut nlength: ptrdiff_t;
  /* FIXME: Use the minibuffer's frame width.  */
  let mut width: ptrdiff_t = (FRAME_COLS (SELECTED_FRAME ()) - 4) as ptrdiff_t;
  let mut idx: ptrdiff_t = -1;
  let mut nobindings: bool = true;
  let mut rest: Lisp_Object;
  let mut vector: Lisp_Object;
  let mut prompt_strings: Lisp_Object = Qnil ();

  vector = Qnil ();

  if ! menu_prompting
    { return Qnil (); }

  map = get_keymap (map, false, true);
  name = Fkeymap_prompt (map);

  /* If we don't have any menus, just read a character normally.  */
  if !STRINGP (name)
    { return Qnil (); }

  /* C 10189-10190: `#define PUSH_C_STR(str, listvar) listvar = Fcons (build_unibyte_string
     (str), listvar)`.  C never `#undef`s it (its scope is the rest of the file) but uses it
     only here, so the port's `macro_rules!` lives in this function, which is the closest
     analogue of C's textual scope.  */
  macro_rules! PUSH_C_STR {
    ($str:expr, $listvar:ident) => {
      $listvar = Fcons (build_unibyte_string ($str), $listvar)
    };
  }

  /* Prompt string always starts with map's prompt, and a space.  */
  prompt_strings = Fcons (name, prompt_strings);
  PUSH_C_STR! (c": ".as_ptr (), prompt_strings);
  nlength = SCHARS (name) + 2;

  rest = map;

  /* Present the documented bindings, a line at a time.  */
  loop
    {
      let mut notfirst: bool = false;
      let mut menu_strings: Lisp_Object = prompt_strings;
      let mut i: ptrdiff_t = nlength;
      let mut obj: Lisp_Object;
      let mut orig_defn_macro: Lisp_Object;

      /* Loop over elements of map.  */
      while i < width
	{
	  let mut elt: Lisp_Object;

	  /* FIXME: Use map_keymap to handle new keymap formats.  */

	  /* At end of map, wrap around if just starting,
	     or end this line if already have something on it.  */
	  if NILP (rest)
	    {
	      if notfirst || nobindings
		{ break; }
	      else
		{ rest = map; }
	    }

	  /* Look at the next element of the map.  */
	  if idx >= 0
	    { elt = AREF (vector, idx); }
	  else
	    { elt = Fcar_safe (rest); }

	  if idx < 0 && VECTORP (elt)
	    {
	      /* If we found a dense table in the keymap,
		 advanced past it, but start scanning its contents.  */
	      rest = Fcdr_safe (rest);
	      vector = elt;
	      idx = 0;
	    }
	  else
	    {
	      /* An ordinary element.  */
	      let mut event: Lisp_Object;
	      let mut tem: Lisp_Object;

	      if idx < 0
		{
		  event = Fcar_safe (elt); /* alist */
		  elt = Fcdr_safe (elt);
		}
	      else
		{
		  XSETINT! (event, idx); /* vector */
		}

	      /* Ignore the element if it has no prompt string.  */
	      if FIXNUMP (event) && parse_menu_item (elt, -1)
		{
		  /* True if the char to type matches the string.  */
		  let mut char_matches: bool;
		  let mut upcased_event: Lisp_Object;
		  let mut downcased_event: Lisp_Object;
		  let mut desc: Lisp_Object = Qnil ();
		  let mut s: Lisp_Object
		    = AREF (item_properties, ITEM_PROPERTY_NAME as isize);

		  upcased_event = Fupcase (event);
		  downcased_event = Fdowncase (event);
		  char_matches = (XFIXNUM (upcased_event) == SREF (s, 0) as EMACS_INT
				  || XFIXNUM (downcased_event) == SREF (s, 0) as EMACS_INT);
		  if ! char_matches
		    { desc = Fsingle_key_description (event, Qnil ()); }

		  /* C 10271-10278 is an `#if 0` block (`tem = XVECTOR (item_properties)…`):
		     dead in C itself, so nothing is transcribed (P-08).  */
		  tem = AREF (item_properties, ITEM_PROPERTY_TYPE as isize);
		  if EQ (tem, QCradio ()) || EQ (tem, QCtoggle ())
		    {
		      /* Insert button prefix.  */
		      let selected: Lisp_Object
			= AREF (item_properties, ITEM_PROPERTY_SELECTED as isize);
		      AUTO_STRING! (radio_yes, c"(*) ".as_ptr ());
		      AUTO_STRING! (radio_no , c"( ) ".as_ptr ());
		      AUTO_STRING! (check_yes, c"[X] ".as_ptr ());
		      AUTO_STRING! (check_no , c"[ ] ".as_ptr ());
		      if EQ (tem, QCradio ())
			{ tem = if NILP (selected) { radio_yes } else { radio_no }; }
		      else
			{ tem = if NILP (selected) { check_yes } else { check_no }; }
		      s = concat2 (tem, s);
		    }


		  /* If we have room for the prompt string, add it to this line.
		     If this is the first on the line, always add it.  */
		  if (SCHARS (s) + i + 2
		       + (if char_matches { 0 } else { SCHARS (desc) + 3 }))
		      < width
		      || !notfirst
		    {
		      let mut thiswidth: ptrdiff_t;

		      /* Punctuate between strings.  */
		      if notfirst
			{
			  PUSH_C_STR! (c", ".as_ptr (), menu_strings);
			  i += 2;
			}
		      notfirst = true;
		      nobindings = false;

		      /* If the char to type doesn't match the string's
			 first char, explicitly show what char to type.  */
		      if ! char_matches
			{
			  /* Add as much of string as fits.  */
			  thiswidth = min! (SCHARS (desc), width - i);
			  menu_strings
			    = Fcons (Fsubstring (desc, make_fixnum (0),
						 make_fixnum (thiswidth)),
				     menu_strings);
			  i += thiswidth;
			  PUSH_C_STR! (c" = ".as_ptr (), menu_strings);
			  i += 3;
			}

		      /* Add as much of string as fits.  */
		      thiswidth = min! (SCHARS (s), width - i);
		      menu_strings
			= Fcons (Fsubstring (s, make_fixnum (0),
					     make_fixnum (thiswidth)),
				 menu_strings);
		      i += thiswidth;
		    }
		  else
		    {
		      /* If this element does not fit, end the line now,
			 and save the element for the next line.  */
		      PUSH_C_STR! (c"...".as_ptr (), menu_strings);
		      break;
		    }
		}

	      /* Move past this element.  */
	      if idx >= 0 && idx + 1 >= ASIZE (vector)
		/* Handle reaching end of dense table.  */
		{ idx = -1; }
	      if idx >= 0
		{ idx += 1; }
	      else
		{ rest = Fcdr_safe (rest); }
	    }
	}

      /* Prompt with that and read response.  */
      message3_nolog (apply1 (Qconcat (), Fnreverse (menu_strings)));

      /* Make believe it's not a keyboard macro in case the help char
	 is pressed.  Help characters are not recorded because menu prompting
	 is not used on replay.  */
      orig_defn_macro = KVAR! (current_kboard, defining_kbd_macro);
      kset_defining_kbd_macro (current_kboard, Qnil ());
      /* C's `do obj = read_char (…); while (BUFFERP (obj));`.  */
      loop
	{
	  /* C passes `0` for `used_mouse_menu`, which the port spells as the null pointer
	     (the `kbd_buffer_store_event_hold` shape).  */
	  obj = read_char (commandflag, Qnil (), Qt (), std::ptr::null_mut (),
			   std::ptr::null_mut ());
	  if !BUFFERP (obj) { break; }
	}
      kset_defining_kbd_macro (current_kboard, orig_defn_macro);

      if !FIXNUMP (obj) || XFIXNUM (obj) == -2
	  || (! EQ (obj, menu_prompt_more_char)
	      && (!FIXNUMP (menu_prompt_more_char)
		  || ! BASE_EQ (obj, make_fixnum (Ctl! (XFIXNUM (menu_prompt_more_char))
						  as EMACS_INT))))
	  /* If 'menu_prompt_more_char' collides with a binding in the
	     map, gives precedence to the map's binding (bug#80146).  */
	  || !NILP (follow_key (map, obj))
	{
	  if !NILP (KVAR! (current_kboard, defining_kbd_macro))
	    { store_kbd_macro_char (obj); }
	  return obj;
	}
      /* Help char - go round again.  */
    }
}


/* C 10388: `/* Reading key sequences.  */` — C 10390-10405: `static Lisp_Object
   active_maps (Lisp_Object first_event, Lisp_Object second_event);`  */
unsafe fn active_maps (first_event: Lisp_Object, second_event: Lisp_Object) -> Lisp_Object
{
  let mut position: Lisp_Object
    = if EVENT_HAS_PARAMETERS (first_event) { EVENT_START (first_event) } else { Qnil () };
  /* The position of a click can be in the second event if the first event
     is a fake_prefixed_key like `header-line` or `mode-line`.  */
  if SYMBOLP (first_event)
      && EVENT_HAS_PARAMETERS (second_event)
      && EQ (first_event, POSN_POSN (EVENT_START (second_event)))
    {
      eassert! (NILP (position));
      position = EVENT_START (second_event);
    }
  return Fcons (Qkeymap (), Fcurrent_active_maps (Qt (), position));
}

/* Structure used to keep track of partial application of key remapping
   such as Vfunction_key_map and Vkey_translation_map.  */
/* C 10409-10421: `typedef struct keyremap { … } keyremap;`.  Measured: size 24, align 8, `parent`
   at 0, `map` at 8, `start` at 16 and `end` at 20.  */
#[repr(C)]
pub struct keyremap
{
  /* This is the map originally specified for this use.  */
  pub parent: Lisp_Object,
  /* This is a submap reached by looking up, in PARENT,
     the events from START to END.  */
  pub map: Lisp_Object,
  /* Positions [START, END) in the key sequence buffer
     are the key that we have scanned so far.
     Those events are the ones that we will replace
     if PARENT maps them into a key sequence.  */
  pub start: c_int,
  pub end: c_int,
}
const _: () = assert! (std::mem::size_of::<keyremap> () == 24);
const _: () = assert! (std::mem::align_of::<keyremap> () == 8);
const _: () = assert! (std::mem::offset_of! (keyremap, parent) == 0);
const _: () = assert! (std::mem::offset_of! (keyremap, map) == 8);
const _: () = assert! (std::mem::offset_of! (keyremap, start) == 16);
const _: () = assert! (std::mem::offset_of! (keyremap, end) == 20);

/* Lookup KEY in MAP.
   MAP is a keymap mapping keys to key vectors or functions.
   If the mapping is a function and DO_FUNCALL is true,
   the function is called with PROMPT as parameter and its return
   value is used as the return value of this function (after checking
   that it is indeed a vector).

   START and END are the indices of the first and last key of the
   sequence being remapped within the keyboard buffer KEYBUF.  */

/* C 10433-10473: `static Lisp_Object access_keymap_keyremap (Lisp_Object map, Lisp_Object key,
   Lisp_Object prompt, bool do_funcall, unsigned int start, unsigned int end,
   Lisp_Object *keybuf);`  */
unsafe fn access_keymap_keyremap (map: Lisp_Object, key: Lisp_Object, prompt: Lisp_Object,
				  do_funcall: bool, start: c_uint, end: c_uint,
				  keybuf: *mut Lisp_Object) -> Lisp_Object
{
  let mut next: Lisp_Object;
  let mut count: specpdl_ref;

  next = access_keymap (map, key, true, false);

  /* Handle a symbol whose function definition is a keymap
     or an array.  */
  if SYMBOLP (next) && !NILP (Ffboundp (next))
      && (ARRAYP ((*XSYMBOL (next)).u.s.function)
	  || KEYMAPP! ((*XSYMBOL (next)).u.s.function))
    { next = Fautoload_do_load ((*XSYMBOL (next)).u.s.function, next, Qnil ()); }

  /* If the keymap gives a function, not an
     array, then call the function with one arg and use
     its value instead.  */
  if do_funcall && FUNCTIONP (next)
    {
      let mut tem: Lisp_Object;
      let mut remap: Lisp_Object;
      tem = next;

      /* Build Vcurrent_key_remap_sequence.  */
      remap = Fvector ((end - start + 1) as ptrdiff_t, keybuf.offset (start as isize));

      /* Bind `current-key-remap-sequence' to the key sequence being
	 remapped.  */
      count = SPECPDL_INDEX ();
      specbind (Qcurrent_key_remap_sequence (), remap);
      next = unbind_to (count, calln! (next, prompt));

      /* If the function returned something invalid,
	 barf--don't ignore it.  */
      if ! (NILP (next) || VECTORP (next) || STRINGP (next))
	{ signal_error (c"Function returns invalid key sequence".as_ptr (), tem); }
    }
  return next;
}

/* Do one step of the key remapping used for function-key-map and
   key-translation-map:
   KEYBUF is the READ_KEY_ELTS-size buffer holding the input events.
   FKEY is a pointer to the keyremap structure to use.
   INPUT is the index of the last element in KEYBUF.
   DOIT if true says that the remapping can actually take place.
   DIFF is used to return the number of keys added/removed by the remapping.
   PARENT is the root of the keymap.
   PROMPT is the prompt to use if the remapping happens through a function.
   Return true if the remapping actually took place.  */

/* C 10486-10546: `static bool keyremap_step (Lisp_Object *keybuf, volatile keyremap *fkey,
   int input, bool doit, int *diff, Lisp_Object prompt);` — C's `volatile` has no Rust
   counterpart, so `fkey` is the plain pointer (the port's `volatile` reading).  */
unsafe fn keyremap_step (keybuf: *mut Lisp_Object, fkey: *mut keyremap,
			 input: c_int, doit: bool, diff: *mut c_int, prompt: Lisp_Object) -> bool
{
  let mut next: Lisp_Object;
  let mut key: Lisp_Object;
  let mut buf_start: ptrdiff_t;
  let mut buf_end: ptrdiff_t;

  /* Save the key sequence being translated.  */
  buf_start = (*fkey).start as ptrdiff_t;
  buf_end = (*fkey).end as ptrdiff_t;

  /* C's `key = keybuf[fkey->end++]`: the load uses the old index, then `end` steps (the `*p++`
     shape).  */
  key = *keybuf.offset ((*fkey).end as isize);
  (*fkey).end += 1;

  if KEYMAPP! ((*fkey).parent)
    { next = access_keymap_keyremap ((*fkey).map, key, prompt, doit,
				     buf_start as c_uint, buf_end as c_uint, keybuf); }
  else
    { next = Qnil (); }

  /* If keybuf[fkey->start..fkey->end] is bound in the
     map and we're in a position to do the key remapping, replace it with
     the binding and restart with fkey->start at the end.  */
  if (VECTORP (next) || STRINGP (next)) && doit
    {
      let len: c_int = XFIXNAT (Flength (next)) as c_int;
      let mut i: c_int;

      *diff = len - ((*fkey).end - (*fkey).start);

      if READ_KEY_ELTS - input <= *diff
	{ error (c"Key sequence too long".as_ptr ()); }

      /* Shift the keys that follow fkey->end.  */
      if *diff < 0
	{
	  i = (*fkey).end;
	  while i < input
	    {
	      *keybuf.offset ((i + *diff) as isize) = *keybuf.offset (i as isize);
	      i += 1;
	    }
	}
      else if *diff > 0
	{
	  i = input - 1;
	  while i >= (*fkey).end
	    {
	      *keybuf.offset ((i + *diff) as isize) = *keybuf.offset (i as isize);
	      i -= 1;
	    }
	}
      /* Overwrite the old keys with the new ones.  */
      i = 0;
      while i < len
	{
	  *keybuf.offset (((*fkey).start + i) as isize)
	    = Faref (next, make_fixnum (i as EMACS_INT));
	  i += 1;
	}

      /* C's `fkey->start = fkey->end += *diff;` assigns right-to-left (PORTING.md 2.2).  */
      (*fkey).end += *diff;
      (*fkey).start = (*fkey).end;
      (*fkey).map = (*fkey).parent;

      return true;
    }

  (*fkey).map = get_keymap (next, false, true);

  /* If we no longer have a bound suffix, try a new position for
     fkey->start.  */
  if !CONSP ((*fkey).map)
    {
      /* C's `fkey->end = ++fkey->start;`: the pre-increment steps `start` first (PORTING.md
	 2.2).  */
      (*fkey).start += 1;
      (*fkey).end = (*fkey).start;
      (*fkey).map = (*fkey).parent;
    }
  return false;
}

/* C 10548-10555: `static bool test_undefined (Lisp_Object binding);`  */
unsafe fn test_undefined (binding: Lisp_Object) -> bool
{
  return NILP (binding)
	 || EQ (binding, Qundefined ())
	 || (SYMBOLP (binding)
	     && EQ (Fcommand_remapping (binding, Qnil (), Qnil ()), Qundefined ()));
}

/* C 10557-10560: `void init_raw_keybuf_count (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_raw_keybuf_count ()
{
  raw_keybuf_count = 0;
}


/* Get a character from the tty.  */

/* Read input events until we get one that's acceptable for our purposes.

   If NO_SWITCH_FRAME, switch-frame events are stashed
   until we get a character we like, and then stuffed into
   unread_switch_frame.

   If ASCII_REQUIRED, check function key events to see
   if the unmodified version of the symbol has a Qascii_character
   property, and use that character, if present.

   If ERROR_NONASCII, signal an error if the input we
   get isn't an ASCII character with modifiers.  If it's false but
   ASCII_REQUIRED is true, just re-read until we get an ASCII
   character.

   If INPUT_METHOD, invoke the current input method
   if the character warrants that.

   If SECONDS is a number, wait that many seconds for input, and
   return Qnil if no input arrives within that time.

   If text conversion is enabled and ASCII_REQUIRED, temporarily
   disable any input method which wants to perform edits, unless
   `disable-inhibit-text-conversion'.  */

/* C 10590-10698: `static Lisp_Object read_filtered_event (bool no_switch_frame,
   bool ascii_required, bool error_nonascii, bool input_method, Lisp_Object seconds);`  */
unsafe fn read_filtered_event (no_switch_frame: bool, ascii_required: bool,
			       error_nonascii: bool, input_method: bool,
			       seconds: Lisp_Object) -> Lisp_Object
{
  /* C's uninitialised locals are zero-initialised (PORTING.md 1.5).  */
  let mut val: Lisp_Object = Qnil ();
  let mut delayed_switch_frame: Lisp_Object = Qnil ();
  let mut end_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
  /* C 10596-10598: `#ifdef HAVE_TEXT_CONVERSION`'s `specpdl_ref count;` (dead on this
     target; carried per P-51).  */
  #[cfg (feature = "have_text_conversion")]
  let mut count: specpdl_ref;

  /* C 10600-10603: `#ifdef HAVE_WINDOW_SYSTEM` (dead on this target; carried per P-51; the
     compiled text never reaches `display_hourglass_p`/`cancel_hourglass`).  */
  #[cfg (feature = "have_window_system")]
  if display_hourglass_p
    { cancel_hourglass (); }

  /* C 10605-10616: `#ifdef HAVE_TEXT_CONVERSION` (dead; carried per P-51).  */
  #[cfg (feature = "have_text_conversion")]
  {
    count = SPECPDL_INDEX ();

    /* Don't use text conversion when trying to just read a
       character.  */

    if ascii_required && !disable_inhibit_text_conversion
      {
	disable_text_conversion ();
	record_unwind_protect_void (Some (resume_text_conversion));
      }
  }

  delayed_switch_frame = Qnil ();

  /* Compute timeout.  */
  if NUMBERP (seconds)
    {
      let duration: c_double = XFLOATINT (seconds);
      let wait_time: timespec = dtotimespec (duration);
      end_time = timespec_add (current_timespec (), wait_time);
    }

  /* Read until we get an acceptable event.  */
  /* C 10629's `retry:` label heads the loop the three `goto retry`s re-enter (PORTING.md
     2.2); C's `do val = read_char (…); while (FIXNUMP (val) && XFIXNUM (val) == -2);` is the
     inner `loop` with the test at its end.  */
  'retry: loop
    {
      loop
	{
	  val = read_char (0, Qnil (), if input_method { Qnil () } else { Qt () },
			   std::ptr::null_mut (),
			   if NUMBERP (seconds) { &raw mut end_time }
			   else { std::ptr::null_mut () });
	  if !(FIXNUMP (val) && XFIXNUM (val) == -2) { break; } /* wrong_kboard_jmpbuf */
	}

      if BUFFERP (val)
	{ continue 'retry; }

      /* `switch-frame' events are put off until after the next ASCII
	 character.  This is better than signaling an error just because
	 the last characters were typed to a separate minibuffer frame,
	 for example.  Eventually, some code which can deal with
	 switch-frame events will read it and process it.  */
      if no_switch_frame
	  && EVENT_HAS_PARAMETERS (val)
	  && EQ (EVENT_HEAD_KIND (EVENT_HEAD (val)), Qswitch_frame ())
	{
	  delayed_switch_frame = val;
	  continue 'retry;
	}

      if ascii_required && !(NUMBERP (seconds) && NILP (val))
	{
	  /* Convert certain symbols to their ASCII equivalents.  */
	  if SYMBOLP (val)
	    {
	      let mut tem: Lisp_Object;
	      let mut tem1: Lisp_Object;
	      tem = Fget (val, Qevent_symbol_element_mask ());
	      if !NILP (tem)
		{
		  tem1 = Fget (Fcar (tem), Qascii_character ());
		  /* Merge this symbol's modifier bits
		     with the ASCII equivalent of its basic code.  */
		  if FIXNUMP (tem1) && FIXNUMP (Fcar (Fcdr (tem)))
		    { XSETFASTINT! (val, XFIXNUM (tem1) | XFIXNUM (Fcar (Fcdr (tem)))); }
		}
	    }

	  /* If we don't have a character now, deal with it appropriately.  */
	  if !FIXNUMP (val)
	    {
	      if error_nonascii
		{
		  Vunread_command_events = list1 (val);
		  error (c"Non-character input-event".as_ptr ());
		}
	      else
		{ continue 'retry; }
	    }
	}

      if ! NILP (delayed_switch_frame)
	{ unread_switch_frame = delayed_switch_frame; }

      /* C 10684-10691 is an `#if 0` block (the hourglass restart): dead in C itself, so
	 nothing is transcribed (P-08).  */

      /* C 10693-10697: `#ifdef HAVE_TEXT_CONVERSION` (dead; carried per P-51); its `#else`
	 arm is the live `return val;` below.  */
      #[cfg (feature = "have_text_conversion")]
      { return unbind_to (count, val); }
      return val;
    }
}

/* C 10700-10747: `DEFUN ("read-char", …)`.  */
DEFUN! ("read-char", Fread_char, Sread_char, 0, 3, 0,
       doc: "Read a character event from the command input (keyboard or macro).\nReturn the character as a number.\nIf the event has modifiers, they are resolved and reflected in the\nreturned character code if possible (e.g. C-SPC yields 0 and C-a yields 97).\nIf some of the modifiers cannot be reflected in the character code, the\nreturned value will include those modifiers, and will not be a valid\ncharacter code: it will fail the `characterp' test.  Use `event-basic-type'\nto recover the character code with the modifiers removed.\n\nIf the user generates an event which is not a character (i.e. a mouse\nclick or function key event), `read-char' signals an error.  As an\nexception, switch-frame events are put off until non-character events\ncan be read.\nIf you want to read non-character events, or ignore them, call\n`read-event' or `read-char-exclusive' instead.\n\nIf the optional argument PROMPT is non-nil, display that as a prompt.\nIf PROMPT is nil or the string \"\", the key sequence/events that led\nto the current command is used as the prompt.\n\nIf the optional argument INHERIT-INPUT-METHOD is non-nil and some\ninput method is turned on in the current buffer, that input method\nis used for reading a character.\n\nIf the optional argument SECONDS is non-nil, it should be a number\nspecifying the maximum number of seconds to wait for input.  If no\ninput arrives in that time, return nil.  SECONDS may be a\nfloating-point value.\n\nIf `inhibit-interaction' is non-nil, this function will signal an\n`inhibited-interaction' error.",
       (prompt: Lisp_Object, inherit_input_method: Lisp_Object, seconds: Lisp_Object),
       {
  let mut val: Lisp_Object;

  barf_if_interaction_inhibited ();

  if ! NILP (prompt)
    {
      cancel_echoing ();
      message_with_string (c"%s".as_ptr (), prompt, false);
    }
  val = read_filtered_event (true, true, true, ! NILP (inherit_input_method), seconds);

  return if !FIXNUMP (val) { Qnil () }
	 else { make_fixnum (char_resolve_modifier_mask (XFIXNUM (val)) as EMACS_INT) };
});

/* C 10749-10782: `DEFUN ("read-event", …)`.  */
DEFUN! ("read-event", Fread_event, Sread_event, 0, 3, 0,
       doc: "Read and return an event object from the input stream.\n\nIf you want to read non-character events, consider calling `read-key'\ninstead.  `read-key' will decode events via `input-decode-map' that\n`read-event' will not.  On a terminal this includes function keys such\nas <F7> and <RIGHT>, or mouse events generated by `xterm-mouse-mode'.\n\nIf the optional argument PROMPT is non-nil, display that as a prompt.\nIf PROMPT is nil or the string \"\", the key sequence/events that led\nto the current command is used as the prompt.\n\nIf the optional argument INHERIT-INPUT-METHOD is non-nil and some\ninput method is turned on in the current buffer, that input method\nis used for reading a character.\n\nIf the optional argument SECONDS is non-nil, it should be a number\nspecifying the maximum number of seconds to wait for input.  If no\ninput arrives in that time, return nil.  SECONDS may be a\nfloating-point value.\n\nIf `inhibit-interaction' is non-nil, this function will signal an\n`inhibited-interaction' error.",
       (prompt: Lisp_Object, inherit_input_method: Lisp_Object, seconds: Lisp_Object),
       {
  barf_if_interaction_inhibited ();

  if ! NILP (prompt)
    {
      cancel_echoing ();
      message_with_string (c"%s".as_ptr (), prompt, false);
    }
  return read_filtered_event (false, false, false, ! NILP (inherit_input_method), seconds);
});

/* C 10784-10825: `DEFUN ("read-char-exclusive", …)`.  */
DEFUN! ("read-char-exclusive", Fread_char_exclusive, Sread_char_exclusive, 0, 3, 0,
       doc: "Read a character event from the command input (keyboard or macro).\nReturn the character as a number.  Non-character events are ignored.\nIf the event has modifiers, they are resolved and reflected in the\nreturned character code if possible (e.g. C-SPC yields 0 and C-a yields 97).\nIf some of the modifiers cannot be reflected in the character code, the\nreturned value will include those modifiers, and will not be a valid\ncharacter code: it will fail the `characterp' test.  Use `event-basic-type'\nto recover the character code with the modifiers removed.\n\nIf the optional argument PROMPT is non-nil, display that as a prompt.\nIf PROMPT is nil or the string \"\", the key sequence/events that led\nto the current command is used as the prompt.\n\nIf the optional argument INHERIT-INPUT-METHOD is non-nil and some\ninput method is turned on in the current buffer, that input method\nis used for reading a character.\n\nIf the optional argument SECONDS is non-nil, it should be a number\nspecifying the maximum number of seconds to wait for input.  If no\ninput arrives in that time, return nil.  SECONDS may be a\nfloating-point value.\n\nIf `inhibit-interaction' is non-nil, this function will signal an\n`inhibited-interaction' error.",
       (prompt: Lisp_Object, inherit_input_method: Lisp_Object, seconds: Lisp_Object),
       {
  let mut val: Lisp_Object;

  barf_if_interaction_inhibited ();

  if ! NILP (prompt)
    {
      cancel_echoing ();
      message_with_string (c"%s".as_ptr (), prompt, false);
    }

  val = read_filtered_event (true, true, false, ! NILP (inherit_input_method), seconds);

  return if !FIXNUMP (val) { Qnil () }
	 else { make_fixnum (char_resolve_modifier_mask (XFIXNUM (val)) as EMACS_INT) };
});


/* C 10829-10843: `#ifdef HAVE_TEXT_CONVERSION`'s `static void restore_reading_key_sequence
   (int old_reading_key_sequence);` (dead on this target; the whole function is carried per
   P-51, which is why the scaffold's entry for it can go: only the same cfg'd arms call it).  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" fn restore_reading_key_sequence (old_reading_key_sequence: c_int)
{
  reading_key_sequence = old_reading_key_sequence != 0;

  /* If a key sequence is no longer being read, reset input methods
     whose state changes were postponed.  */

  if old_reading_key_sequence == 0
    { check_postponed_buffers (); }
}

/* Return true if there are any pending requeued events (command events
   or events to be processed by other levels of the input processing
   stages).  */

/* C 10849-10855: `static bool requeued_events_pending_p (void);`  */
unsafe fn requeued_events_pending_p () -> bool
{
  return requeued_command_events_pending_p ()
	 || !NILP (Vunread_post_input_method_events)
	 || !NILP (Vunread_input_method_events);
}

/* C 10857-10899: `read_key_sequence`'s own doc comment, above its definition.  */
/* ============================================================================
   C 10901-11904: `static int read_key_sequence (Lisp_Object *keybuf, Lisp_Object prompt,
   bool dont_downcase_last, bool can_return_switch_frame, bool fix_current_buffer,
   bool prevent_redisplay, bool disable_text_conversion_p);`.  C's `goto` labels are the
   labelled loops and blocks of PORTING.md 2.2 (`'replay_entire_sequence` /
   `'replay_sequence` / `'replay_key` / `'not_upcase` / `'disable_text_conversion` /
   `'done`), whose token the port cannot spell (`goto` is a `macro-body-notes.tsv` row).
   ============================================================================ */
unsafe fn read_key_sequence (keybuf: *mut Lisp_Object, prompt: Lisp_Object,
			     dont_downcase_last: bool, can_return_switch_frame: bool,
			     fix_current_buffer: bool, prevent_redisplay: bool,
			     disable_text_conversion_p: bool) -> c_int
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* How many keys there are in the current key sequence.  */
  let mut t: c_int;

  /* The length of the echo buffer when we started reading, and
     the length of this_command_keys when we started reading.  */
  /* C's `ptrdiff_t echo_start UNINIT;`: the port zero-initialises it (PORTING.md 1.5).  */
  let mut echo_start: ptrdiff_t = 0;
  let mut keys_start: ptrdiff_t;

  let mut current_binding: Lisp_Object = Qnil ();

  /* Index of the first key that has no binding.
     It is useless to try fkey.start larger than that.  */
  let mut first_unbound: c_int;

  /* If t < mock_input, then KEYBUF[t] should be read as the next
     input key.  [...]  */
  let mut mock_input: c_int = 0;

  /* Whether each event in the mocked input came from a mouse menu.  */
  let mut used_mouse_menu_history: [bool; READ_KEY_ELTS as usize] = [false; READ_KEY_ELTS as usize];

  /* If the sequence is unbound in submaps[], then
     keybuf[fkey.start..fkey.end-1] is a prefix in Vfunction_key_map,
     and fkey.map is its binding.

     These might be > t, indicating that all function key scanning
     should hold off until t reaches them.  We do this when we've just
     recognized a function key, to avoid searching for the function
     key's again in Vfunction_key_map.  */
  /* C's `keyremap fkey;` (like `keytran`/`indec`) is an uninitialised local; the port
     zero-initialises it (PORTING.md 1.5) and the setup at `'replay_entire_sequence`
     overwrites every field.  */
  let mut fkey: keyremap = keyremap { parent: Qnil (), map: Qnil (), start: 0, end: 0 };

  /* Likewise, for key_translation_map and input-decode-map.  */
  let mut keytran: keyremap = keyremap { parent: Qnil (), map: Qnil (), start: 0, end: 0 };
  let mut indec: keyremap = keyremap { parent: Qnil (), map: Qnil (), start: 0, end: 0 };

  /* True if we are trying to map a key by changing an upper-case
     letter to lower case, or a shifted function key to an unshifted
     one.  */
  let mut shift_translated: bool = false;

  /* If we receive a `switch-frame' or `select-window' event in the middle of
     a key sequence, we put it off for later.
     While we're reading, we keep the event here.  */
  let mut delayed_switch_frame: Lisp_Object = Qnil ();

  /* C's `Lisp_Object original_uppercase UNINIT;` — the port zero-initialises it
     (PORTING.md 1.5).  */
  let mut original_uppercase: Lisp_Object = Qnil ();
  let mut original_uppercase_position: c_int = -1;

  /* C 10968-10972: `#ifdef HAVE_TEXT_CONVERSION`'s `bool disabled_conversion;` and its
     assignment (dead on this target; carried per P-51).  */
  #[cfg (feature = "have_text_conversion")]
  let mut disabled_conversion: bool = false;

  let mut starting_buffer: *mut buffer;

  /* List of events for which a fake prefix key has been generated.  */
  let mut fake_prefixed_keys: Lisp_Object = Qnil ();

  /* raw_keybuf_count is now initialized in (most of) the callers of
     read_key_sequence.  This is so that in a recursive call (for
     mouse menus) a spurious initialization doesn't erase the contents
     of raw_keybuf created by the outer call.  */
  /* raw_keybuf_count = 0; */

  delayed_switch_frame = Qnil ();

  if INTERACTIVE! ()
    {
      if !NILP (prompt)
	{
	  /* Install the string PROMPT as the beginning of the string
	     of echoing, so that it serves as a prompt for the next
	     character.  */
	  kset_echo_prompt (current_kboard, prompt);
          /* FIXME: This use of echo_now doesn't look quite right and is ugly
             since it forces us to fiddle with current_kboard->immediate_echo
             before and after.  */
	  kboard_bits! (immediate_echo, current_kboard, false);
	  echo_now ();
          if !echo_keystrokes_p ()
	    { kboard_bits! (immediate_echo, current_kboard, false); }
	}
      else if cursor_in_echo_area /* FIXME: Not sure why we test this here,
                                      maybe we should just drop this test.  */
	       && echo_keystrokes_p ()
	/* This doesn't put in a dash if the echo buffer is empty, so
	   you don't always see a dash hanging out in the minibuffer.  */
	{ echo_dash (); }
    }

  /* Record the initial state of the echo area and this_command_keys;
     we will need to restore them if we replay a key sequence.  */
  if INTERACTIVE! ()
    { echo_start = echo_length (); }
  keys_start = this_command_key_count;
  this_single_command_key_start = keys_start;

  /* C 11014-11026: `#ifdef HAVE_TEXT_CONVERSION` (dead on this target; carried per P-51).
     `restore_reading_key_sequence` is this file's cfg'd definition above.  */
  #[cfg (feature = "have_text_conversion")]
  {
    /* Set `reading_key_sequence' to true.  This variable is used by
       Fset_text_conversion_style to determine if it should postpone
       resetting the input method until this function completes.  */

    record_unwind_protect_int (Some (restore_reading_key_sequence),
			       reading_key_sequence as c_int);
    reading_key_sequence = true;
  }

  /* We jump here when we need to reinitialize fkey and keytran; this
     happens if we switch keyboards between rescans.  */
 'done: {
 'replay_entire_sequence: loop
  {
  indec.parent = KVAR! (current_kboard, Vinput_decode_map);
  indec.map = indec.parent;
  fkey.parent = KVAR! (current_kboard, Vlocal_function_key_map);
  fkey.map = fkey.parent;
  keytran.parent = Vkey_translation_map;
  keytran.map = keytran.parent;
  indec.end = 0;
  indec.start = indec.end;
  fkey.end = 0;
  fkey.start = fkey.end;
  keytran.end = 0;
  keytran.start = keytran.end;

  /* We jump here when the key sequence has been thoroughly changed, and
     we need to rescan it starting from the beginning.  When we jump here,
     keybuf[0..mock_input] holds the sequence we should reread.  */
 'replay_sequence: loop
  {
  starting_buffer = current_buffer;
  first_unbound = READ_KEY_ELTS + 1;
  let mut first_event: Lisp_Object = if mock_input > 0 { *keybuf.offset (0) } else { Qnil () };
  let mut second_event: Lisp_Object = if mock_input > 1 { *keybuf.offset (1) } else { Qnil () };

  /* Build our list of keymaps.
     If we recognize a function key and replace its escape sequence in
     keybuf with its symbol, or if the sequence starts with a mouse
     click and we need to switch buffers, we jump back here to rebuild
     the initial keymaps from the current buffer.  */
  current_binding = active_maps (first_event, second_event);

  /* Start from the beginning in keybuf.  */
  t = 0;
  last_nonmenu_event = Qnil ();

  /* These are no-ops the first time through, but if we restart, they
     revert the echo area and this_command_keys to their original state.  */
  this_command_key_count = keys_start;
  if INTERACTIVE! () && t < mock_input
    { echo_truncate (echo_start); }

  /* If text conversion is supposed to be disabled immediately, do it
     now.  */

  /* C 11072-11082: `#ifdef HAVE_TEXT_CONVERSION` (dead on this target; carried per
     P-51).  */
  #[cfg (feature = "have_text_conversion")]
  if disable_text_conversion_p
    {
      disable_text_conversion ();
      record_unwind_protect_void (Some (resume_text_conversion));
      disabled_conversion = true;
    }

  /* If the best binding for the current key sequence is a keymap, or
     we may be looking at a function key's escape sequence, keep on
     reading.  */
 'while_body: loop
  {
  if !(if !NILP (current_binding)
	 /* Keep reading as long as there's a prefix binding.  */
	 { KEYMAPP! (current_binding) }
	 /* Don't return in the middle of a possible function key sequence,
	    if the only bindings we found were via case conversion.
	    Thus, if ESC O a has a function-key-map translation
	    and ESC o has a binding, don't return after ESC O,
	    so that we can translate ESC O plus the next character.  */
	 else { (/* indec.start < t || fkey.start < t || */ keytran.start < t) })
    { break 'while_body; }

  let mut key: Lisp_Object = Qnil ();
  let mut used_mouse_menu: bool = false;

  /* Where the last real key started.  If we need to throw away a
     key that has expanded into more than one element of keybuf
     (say, a mouse click on the mode line which is being treated
     as [mode-line (mouse-...)], then we backtrack to this point
     of keybuf.  */
  let mut last_real_key_start: c_int = 0;

  /* These variables are analogous to echo_start and keys_start;
     while those allow us to restart the entire key sequence,
     echo_local_start and keys_local_start allow us to throw away
     just one key.  */
  /* C's `ptrdiff_t echo_local_start UNINIT;` — zero-initialised by the port
     (PORTING.md 1.5).  */
  let mut echo_local_start: ptrdiff_t = 0;
  let mut keys_local_start: c_int = 0;
  let mut new_binding: Lisp_Object = Qnil ();

  eassert! (indec.end == t || (indec.end > t && indec.end <= mock_input));
  eassert! (indec.start <= indec.end);
  eassert! (fkey.start <= fkey.end);
  eassert! (keytran.start <= keytran.end);
  /* key-translation-map is applied *after* function-key-map
     which is itself applied *after* input-decode-map.  */
  eassert! (fkey.end <= indec.start);
  eassert! (keytran.end <= fkey.start);

  if /* first_unbound < indec.start && first_unbound < fkey.start && */
      first_unbound < keytran.start
    { /* The prefix up to first_unbound has no binding and has
	 no translation left to do either, so we know it's unbound.
	 If we don't stop now, we risk staying here indefinitely
	 (if the user keeps entering fkey or keytran prefixes
	 like C-c ESC ESC ESC ESC ...)  */
      let mut i: c_int;
      i = first_unbound + 1;
      while i < t
	{
	  *keybuf.offset ((i - first_unbound - 1) as isize) = *keybuf.offset (i as isize);
	  i += 1;
	}
      mock_input = t - first_unbound - 1;
      indec.start -= first_unbound + 1;
      indec.end = indec.start;
      indec.map = indec.parent;
      fkey.start -= first_unbound + 1;
      fkey.end = fkey.start;
      fkey.map = fkey.parent;
      keytran.start -= first_unbound + 1;
      keytran.end = keytran.start;
      keytran.map = keytran.parent;
      continue 'replay_sequence;
    }

  if t >= READ_KEY_ELTS
    { error (c"Key sequence too long".as_ptr ()); }

  if INTERACTIVE! ()
    { echo_local_start = echo_length (); }
  keys_local_start = this_command_key_count as c_int;

  /* C 11096-11129: `#ifdef HAVE_TEXT_CONVERSION` (dead on this target; carried per
     P-51).  */
  #[cfg (feature = "have_text_conversion")]
  {
    /* When reading a key sequence while text conversion is in
       effect, turn it off after the first actual character read.
       This makes input methods send actual key events instead.

       Make sure only to do this once.  Also, disabling text
       conversion seems to interact badly with menus, so don't
       disable text conversion if a menu was displayed.  */

    if !disabled_conversion && t != 0 && !used_mouse_menu
	&& !disable_inhibit_text_conversion
      {
	let mut i: c_int;
	/* C's `goto disable_text_conversion` (the `for` loop's jump into the label) and
	   its `goto replay_key` (the fall-through to the label just below this block,
	   which the port spells as the `found` test: no match means skip the three
	   statements and enter `replay_key`).  */
	let mut goto_disable_text_conversion: bool = false;

	/* used_mouse_menu isn't set if a menu bar prefix key has
	   just been stored.  It appears necessary to look for a
	   prefix key itself.  Don't look through too many keys for
	   efficiency reasons.  */

	i = 0;
	while i < min! (t, 10)
	  {
	    if NUMBERP (*keybuf.offset (i as isize))
		|| (SYMBOLP (*keybuf.offset (i as isize))
		    && EQ (Fget (*keybuf.offset (i as isize), Qevent_kind ()),
			   Qfunction_key ()))
	      { goto_disable_text_conversion = true; break; }
	    i += 1;
	  }

	if goto_disable_text_conversion
	  {
	    disable_text_conversion ();
	    record_unwind_protect_void (Some (resume_text_conversion));
	    disabled_conversion = true;
	  }
      }
  }

 'replay_key: loop
  {
  /* These are no-ops, unless we throw away a keystroke below and
     jumped back up to replay_key; in that case, these restore the
     variables to their original state, allowing us to replay the
     loop.  */
  if INTERACTIVE! () && t < mock_input
    { echo_truncate (echo_local_start); }
  this_command_key_count = keys_local_start as ptrdiff_t;

  /* By default, assume each event is "real".  */
  last_real_key_start = t;

  /* Does mock_input indicate that we are re-reading a key sequence?  */
  if t < mock_input
    {
      key = *keybuf.offset (t as isize);
      add_command_key (key);
      if kboard_bits! (immediate_echo, current_kboard)
	{
	  /* Set immediate_echo to false so as to force echo_now to
	     redisplay (it will set immediate_echo right back to true).  */
	  kboard_bits! (immediate_echo, current_kboard, false);
	  echo_now ();
	}
      used_mouse_menu = used_mouse_menu_history[t as usize];
    }
  /* If we're at the end of a macro, exit it by returning 0,
     unless there are unread events pending.  */
  else if !NILP (Vexecuting_kbd_macro)
      && at_end_of_macro_p ()
      && !requeued_events_pending_p ()
    {
      t = 0;
      break 'done;
    }
  /* Otherwise, we should actually read a character.  */
  else
    {
      {
	let mut interrupted_kboard: *mut kboard = current_kboard;
	let interrupted_frame: *mut frame = SELECTED_FRAME ();
	/* Calling read_char with COMMANDFLAG = -2 avoids
	   redisplay in read_char and its subroutines.  */
	key = read_char (if prevent_redisplay { -2 } else { NILP (prompt) as c_int },
			 current_binding, last_nonmenu_event,
			 &raw mut used_mouse_menu, std::ptr::null_mut ());
	used_mouse_menu_history[t as usize] = used_mouse_menu;
	if (FIXNUMP (key) && XFIXNUM (key) == -2) /* wrong_kboard_jmpbuf */
	    /* When switching to a new tty (with a new keyboard),
	       read_char returns the new buffer, rather than -2
	       (bug#5095, bug#37782, bug#79513).
	       This is because `terminal-init-xterm' calls
	       read-char, which eats the wrong_kboard_jmpbuf return.
	       Any better way to fix this? -- cyd  */
	    || (interrupted_kboard != current_kboard)
	  {
	    let mut found: bool = false;
	    let mut kb: *mut kboard;

	    kb = all_kboards;
	    while !kb.is_null ()
	      {
		if kb == interrupted_kboard
		  { found = true; }
		kb = (*kb).next_kboard;
	      }

	    if !found
	      {
		/* Don't touch interrupted_kboard when it's been
		   deleted.  */
		delayed_switch_frame = Qnil ();
		continue 'replay_entire_sequence;
	      }

	    if !NILP (delayed_switch_frame)
	      {
		kset_kbd_queue
		  (interrupted_kboard,
		   Fcons (delayed_switch_frame,
			  KVAR! (interrupted_kboard, kbd_queue)));
		delayed_switch_frame = Qnil ();
	      }

	    while t > 0
	      {
		t -= 1;
		kset_kbd_queue
		  (interrupted_kboard,
		   Fcons (*keybuf.offset (t as isize), KVAR! (interrupted_kboard, kbd_queue)));
	      }

	    /* If the side queue is non-empty, ensure it begins with a
	       switch-frame, so we'll replay it in the right context.  */
	    let mut kbd_queue_head_is_switch_frame: bool = false;
	    if CONSP (KVAR! (interrupted_kboard, kbd_queue))
	      {
		key = XCAR (KVAR! (interrupted_kboard, kbd_queue));
		kbd_queue_head_is_switch_frame
		  = EVENT_HAS_PARAMETERS (key)
		    && EQ (EVENT_HEAD_KIND (EVENT_HEAD (key)), Qswitch_frame ());
	      }
	    if CONSP (KVAR! (interrupted_kboard, kbd_queue))
		&& !kbd_queue_head_is_switch_frame
	      {
		let mut frame: Lisp_Object = Qnil ();
		XSETFRAME! (frame, interrupted_frame);
		kset_kbd_queue
		  (interrupted_kboard,
		   Fcons (make_lispy_switch_frame (frame),
			  KVAR! (interrupted_kboard, kbd_queue)));
		mock_input = 0;
	      }
	    else
	      {
		if FIXNUMP (key) && XFIXNUM (key) != -2
		  {
		    /* If interrupted while initializing terminal,
		       we need to replay the interrupting key.
		       There may also have been a current buffer
		       change we would otherwise miss.
		       See bug#5095, bug#37782, bug#79513.  */
		    if fix_current_buffer
			&& (XBUFFER ((*XWINDOW (selected_window)).contents)
			    != current_buffer)
		      { Fset_buffer ((*XWINDOW (selected_window)).contents); }
		    mock_input = 1;
		    *keybuf.offset (0) = key;
		  }
		else
		  { mock_input = 0; }
	      }
	    continue 'replay_entire_sequence;
	  }
      }

      /* read_char returns t when it shows a menu and the user rejects it.
         Just return -1.  */
      if EQ (key, Qt ())
        {
          unbind_to (count, Qnil ());
          return -1;
        }

      /* If the current buffer has been changed from under us, the
         keymap may have changed, so replay the sequence.  */
      if BUFFERP (key)
        {
          timer_resume_idle ();

          mock_input = t;
          /* Reset the current buffer from the selected window
    	 in case something changed the former and not the latter.
    	 This is to be more consistent with the behavior
    	 of the command_loop_1.  */
          if fix_current_buffer
    	{
    	  if !FRAME_LIVE_P (XFRAME (selected_frame))
    	    { Fkill_emacs (Qnil (), Qnil ()); }
    	  if XBUFFER ((*XWINDOW (selected_window)).contents)
    	      != current_buffer
    	    { Fset_buffer ((*XWINDOW (selected_window)).contents); }
    	}

          continue 'replay_sequence;
        }

      /* If we have a quit that was typed in another frame, and
         quit_throw_to_read_char switched buffers,
         replay to get the right keymap.  */
      if FIXNUMP (key)
          && XFIXNUM (key) == quit_char as EMACS_INT
          && current_buffer != starting_buffer
        {
          GROW_RAW_KEYBUF! ();
          ASET (raw_keybuf, raw_keybuf_count as isize, key);
          raw_keybuf_count += 1;
          *keybuf.offset (t as isize) = key;
          t += 1;
          mock_input = t;
          Vquit_flag = Qnil ();
          continue 'replay_sequence;
        }

      Vquit_flag = Qnil ();

      if EVENT_HAS_PARAMETERS (key)
          /* Either a `switch-frame' or a `select-window' event.  */
          && EQ (EVENT_HEAD_KIND (EVENT_HEAD (key)), Qswitch_frame ())
        {
          /* If we're at the beginning of a key sequence, and the caller
    	 says it's okay, go ahead and return this event.  If we're
    	 in the midst of a key sequence, delay it until the end.  */
          if t > 0 || !can_return_switch_frame
    	{
    	  delayed_switch_frame = key;
    	  continue 'replay_key;
    	}
        }

      if NILP (first_event)
        {
          first_event = key;
          /* Even if first_event does not specify a particular
    	 window/position, it's important to recompute the maps here
    	 since a long time might have passed since we entered
    	 read_key_sequence, and a timer (or process-filter or
    	 special-event-map, ...) might have switched the current buffer
    	 or the selected window from under us in the mean time.  */
          if fix_current_buffer
    	  && (XBUFFER ((*XWINDOW (selected_window)).contents)
    	      != current_buffer)
    	{ Fset_buffer ((*XWINDOW (selected_window)).contents); }
          current_binding = active_maps (first_event, Qnil ());
        }

      GROW_RAW_KEYBUF! ();
      ASET (raw_keybuf, raw_keybuf_count as isize,
    	/* Copy the event, in case it gets modified by side-effect
    	   by some remapping function (bug#30955).  */
    	if CONSP (key) { Fcopy_sequence (key) } else { key });
      raw_keybuf_count += 1;
    }

  /* Clicks in non-text areas get prefixed by the symbol
     in their CHAR-ADDRESS field.  For example, a click on
     the mode line is prefixed by the symbol `mode-line'.

     Furthermore, key sequences beginning with mouse clicks
     are read using the keymaps of the buffer clicked on, not
     the current buffer.  So we may have to switch the buffer
     here.

     When we turn one event into two events, we must make sure
     that neither of the two looks like the original--so that,
     if we replay the events, they won't be expanded again.
     If not for this, such reexpansion could happen either here
     or when user programs play with this-command-keys.  */
  if EVENT_HAS_PARAMETERS (key)
    {
      let kind: Lisp_Object = EVENT_HEAD_KIND (EVENT_HEAD (key));
      if EQ (kind, Qmouse_click ()) || EQ (kind, Qtouchscreen ())
	{
	  let window: Lisp_Object = POSN_WINDOW (EVENT_START (key));
	  let posn: Lisp_Object = POSN_POSN (EVENT_START (key));

	  if CONSP (posn)
	      || (!NILP (fake_prefixed_keys)
		  && !NILP (Fmemq (key, fake_prefixed_keys)))
	    {
	      /* We're looking a second time at an event for which
		 we generated a fake prefix key.  Set
		 last_real_key_start appropriately.  */
	      if t > 0
		{ last_real_key_start = t - 1; }
	    }

	  if last_real_key_start == 0
	    {
	      /* Key sequences beginning with mouse clicks are
		 read using the keymaps in the buffer clicked on,
		 not the current buffer.  If we're at the
		 beginning of a key sequence, switch buffers.  */
	      if WINDOWP (window)
		  && BUFFERP ((*XWINDOW (window)).contents)
		  && XBUFFER ((*XWINDOW (window)).contents) != current_buffer
		{
		  *keybuf.offset (t as isize) = key;
		  mock_input = t + 1;

		  /* Arrange to go back to the original buffer once we're
		     done reading the key sequence.  Note that we can't
		     use save_excursion_{save,restore} here, because they
		     save point as well as the current buffer; we don't
		     want to save point, because redisplay may change it,
		     to accommodate a Fset_window_start or something.  We
		     don't want to do this at the top of the function,
		     because we may get input from a subprocess which
		     wants to change the selected window and stuff (say,
		     emacsclient).  */
		  record_unwind_current_buffer ();

		  if !FRAME_LIVE_P (XFRAME (selected_frame))
		    { Fkill_emacs (Qnil (), Qnil ()); }
		  set_buffer_internal (XBUFFER ((*XWINDOW (window)).contents));
		  continue 'replay_sequence;
		}
	    }

	  /* Expand mode-line and scroll-bar events into two events:
	     use posn as a fake prefix key.  */
	  if SYMBOLP (posn)
	      && (NILP (fake_prefixed_keys)
		  || NILP (Fmemq (key, fake_prefixed_keys)))
	    {
	      if READ_KEY_ELTS - t <= 1
		{ error (c"Key sequence too long".as_ptr ()); }

	      *keybuf.offset (t as isize)     = posn;
	      *keybuf.offset ((t + 1) as isize) = key;
	      mock_input    = t + 2;

	      /* Record that a fake prefix key has been generated
		 for KEY.  Don't modify the event; this would
		 prevent proper action when the event is pushed
		 back into unread-command-events.  */
	      fake_prefixed_keys = Fcons (key, fake_prefixed_keys);
	      continue 'replay_key;
	    }
	}
      else if CONSP (XCDR (key))
	       && CONSP (xevent_start (key))
	       && CONSP (XCDR (xevent_start (key)))
	{
	  let mut posn: Lisp_Object;

	  posn = POSN_POSN (xevent_start (key));
	  /* Handle menu-bar events:
	     insert the dummy prefix event `menu-bar'.  */
	  if (EQ (posn, Qmenu_bar ()) || EQ (posn, Qtab_bar ())
	      || EQ (posn, Qtool_bar ())
	      /* Only insert the prefix key if the event comes
		 directly from the keyboard buffer.  Key
		 translation functions might return events with a
		 `posn-area' of tool-bar or tab-bar without
		 intending for these prefix events to be
		 generated.  */
	      && (mock_input <= t))
	    {
	      if READ_KEY_ELTS - t <= 1
		{ error (c"Key sequence too long".as_ptr ()); }
	      *keybuf.offset (t as isize) = posn;
	      *keybuf.offset ((t + 1) as isize) = key;

	      /* Zap the position in key, so we know that we've
		 expanded it, and don't try to do so again.  */
	      POSN_SET_POSN (xevent_start (key), list1 (posn));

	      mock_input = t + 2;
	      continue 'replay_sequence;
	    }
	  else if CONSP (posn)
	    {
	      /* We're looking at the second event of a
		 sequence which we expanded before.  Set
		 last_real_key_start appropriately.  */
	      if last_real_key_start == t && t > 0
		{ last_real_key_start = t - 1; }
	    }
	}
    }

  /* We have finally decided that KEY is something we might want
     to look up.  */
  new_binding = follow_key (current_binding, key);

  /* If KEY wasn't bound, we'll try some fallbacks.  */
  if !NILP (new_binding)
    /* This is needed for the following scenario:
       event 0: a down-event that gets dropped by calling replay_key.
       event 1: some normal prefix like C-h.
       After event 0, first_unbound is 0, after event 1 indec.start,
       fkey.start, and keytran.start are all 1, so when we see that
       C-h is bound, we need to update first_unbound.  */
    { first_unbound = max! (t + 1, first_unbound); }
  else
    {
      let head: Lisp_Object = EVENT_HEAD (key);

      /* Remember the position to put an upper bound on indec.start.  */
      first_unbound = min! (t, first_unbound);

      if SYMBOLP (head)
	{
	  let breakdown: Lisp_Object = parse_modifiers (head);
	  let mut modifiers: c_int = XFIXNUM (XCAR (XCDR (breakdown))) as c_int;

	  /* Attempt to reduce an unbound mouse event to a simpler
	     event that is bound:
	       Drags reduce to clicks.
	       Double-clicks reduce to clicks.
	       Triple-clicks reduce to double-clicks, then to clicks.
	       Up/Down-clicks are eliminated.
	       Double-downs reduce to downs, then are eliminated.
	       Triple-downs reduce to double-downs, then to downs,
		 then are eliminated.  */
	  if (modifiers & (up_modifier | down_modifier
			   | drag_modifier
			   | double_modifier | triple_modifier)) != 0
	    {
	      while (modifiers & (up_modifier | down_modifier
				  | drag_modifier
				  | double_modifier | triple_modifier)) != 0
		{
		  let mut new_head: Lisp_Object = Qnil ();
		  let mut new_click: Lisp_Object = Qnil ();
		  if (modifiers & triple_modifier) != 0
		    { modifiers ^= double_modifier | triple_modifier; }
		  else if (modifiers & double_modifier) != 0
		    { modifiers &= !double_modifier; }
		  else if (modifiers & drag_modifier) != 0
		    { modifiers &= !drag_modifier; }
		  else
		    {
		      /* Dispose of this `up/down' event by simply jumping
			 back to replay_key, to get another event.

			 Note that if this event came from mock input,
			 then just jumping back to replay_key will just
			 hand it to us again.  So we have to wipe out any
			 mock input.

			 We could delete keybuf[t] and shift everything
			 after that to the left by one spot, but we'd also
			 have to fix up any variable that points into
			 keybuf, and shifting isn't really necessary
			 anyway.

			 Adding prefixes for non-textual mouse clicks
			 creates two characters of mock input, and both
			 must be thrown away.  If we're only looking at
			 the prefix now, we can just jump back to
			 replay_key.  On the other hand, if we've already
			 processed the prefix, and now the actual click
			 itself is giving us trouble, then we've lost the
			 state of the keymaps we want to backtrack to, and
			 we need to replay the whole sequence to rebuild
			 it.

			 Beyond that, only function key expansion could
			 create more than two keys, but that should never
			 generate mouse events, so it's okay to zero
			 mock_input in that case too.

			 FIXME: The above paragraph seems just plain
			 wrong, if you consider things like
			 xterm-mouse-mode.  -stef

			 Isn't this just the most wonderful code ever?  */

		      /* If mock_input > t + 1, the above simplification
			 will actually end up dropping keys on the floor.
			 This is probably OK for now, but even
			 if mock_input <= t + 1, we need to adjust indec,
			 fkey, and keytran.
			 Typical case [header-line down-mouse-N]:
			 mock_input = 2, t = 1, fkey.end = 1,
			 last_real_key_start = 0.  */
		      if indec.end > last_real_key_start
			{
			  indec.start = min! (last_real_key_start, indec.start);
			  indec.end = indec.start;
			  indec.map = indec.parent;
			  if fkey.end > last_real_key_start
			    {
			      fkey.start = min! (last_real_key_start, fkey.start);
			      fkey.end = fkey.start;
			      fkey.map = fkey.parent;
			      if keytran.end > last_real_key_start
				{
				  keytran.start = min! (last_real_key_start, keytran.start);
				  keytran.end = keytran.start;
				  keytran.map = keytran.parent;
				}
			    }
			}
		      if t == last_real_key_start
			{
			  mock_input = 0;
			  continue 'replay_key;
			}
		      else
			{
			  mock_input = last_real_key_start;
			  continue 'replay_sequence;
			}
		    }

		  new_head
		    = apply_modifiers (modifiers, XCAR (breakdown));
		  new_click = list2 (new_head, EVENT_START (key));

		  /* Look for a binding for this new key.  */
		  new_binding = follow_key (current_binding, new_click);

		  /* If that click is bound, go for it.  */
		  if !NILP (new_binding)
		    {
		      current_binding = new_binding;
		      key = new_click;
		      break;
		    }
		  /* Otherwise, we'll leave key set to the drag event.  */
		}
	    }
	}
    }
  current_binding = new_binding;

  *keybuf.offset (t as isize) = key;
  t += 1;
  /* Normally, last_nonmenu_event gets the previous key we read.
     But when a mouse popup menu is being used,
     we don't update last_nonmenu_event; it continues to hold the mouse
     event that preceded the first level of menu.  */
  if !used_mouse_menu
    { last_nonmenu_event = key; }

  /* Record what part of this_command_keys is the current key sequence.  */
  this_single_command_key_start = this_command_key_count - t as ptrdiff_t;
  /* When 'input-method-function' called above causes events to be
     put on 'unread-post-input-method-events', and as result
     'reread' is set to 'true', the value of 't' can become larger
     than 'this_command_key_count', because 'add_command_key' is
     not called to update 'this_command_key_count'.  If this
     happens, 'this_single_command_key_start' will become negative
     above, and any call to 'this-single-command-keys' will return
     a garbled vector.  See bug #20223 for one such situation.
     Here we force 'this_single_command_key_start' to never become
     negative, to avoid that.  */
  if this_single_command_key_start < 0
    { this_single_command_key_start = 0; }

  /* Look for this sequence in input-decode-map.
     Scan from indec.end until we find a bound suffix.  */
  while indec.end < t
    {
      let done: bool;
      let mut diff: c_int = 0;

      done = keyremap_step (keybuf, &raw mut indec, max! (t, mock_input),
			    true, &raw mut diff, prompt);
      if done
	{
	  mock_input = diff + max! (t, mock_input);
	  continue 'replay_sequence;
	}
    }

  if !KEYMAPP! (current_binding)
      && !test_undefined (current_binding)
      && indec.start >= t
    /* There is a binding and it's not a prefix.
       (and it doesn't have any input-decode-map translation pending).
       There is thus no function-key in this sequence.
       Moving fkey.start is important in this case to allow keytran.start
       to go over the sequence before we return (since we keep the
       invariant that keytran.end <= fkey.start).  */
    {
      if fkey.start < t
	{ fkey.end = t; fkey.start = fkey.end; fkey.map = fkey.parent; }
    }
  else
    /* If the sequence is unbound, see if we can hang a function key
       off the end of it.  */
    /* Continue scan from fkey.end until we find a bound suffix.  */
    {
    while fkey.end < indec.start
      {
	let done: bool;
	let mut diff: c_int = 0;

	done = keyremap_step (keybuf, &raw mut fkey,
			      max! (t, mock_input),
			      /* If there's a binding (i.e.
				 first_binding >= nmaps) we don't want
				 to apply this function-key-mapping.  */
			      (fkey.end + 1 == t
			       && test_undefined (current_binding)),
			      &raw mut diff, prompt);
	if done
	  {
	    mock_input = diff + max! (t, mock_input);
	    /* Adjust the input-decode-map counters.  */
	    indec.end += diff;
	    indec.start += diff;

	    continue 'replay_sequence;
	  }
      }
    }

  /* Look for this sequence in key-translation-map.
     Scan from keytran.end until we find a bound suffix.  */
  while keytran.end < fkey.start
    {
      let done: bool;
      let mut diff: c_int = 0;

      done = keyremap_step (keybuf, &raw mut keytran, max! (t, mock_input),
			    true, &raw mut diff, prompt);
      if done
	{
	  mock_input = diff + max! (t, mock_input);
	  /* Adjust the function-key-map and input-decode-map counters.  */
	  indec.end += diff;
	  indec.start += diff;
	  fkey.end += diff;
	  fkey.start += diff;

	  continue 'replay_sequence;
	}
    }

  /* If KEY is not defined in any of the keymaps,
     and cannot be part of a function key or translation,
     and is an upper case letter
     use the corresponding lower-case letter instead.  */
 'not_upcase: {
  if NILP (current_binding)
      && /* indec.start >= t && fkey.start >= t && */ keytran.start >= t
      && FIXNUMP (key)
      && translate_upper_case_key_bindings
    {
      let mut new_key: Lisp_Object = Qnil ();
      let k: EMACS_INT = XFIXNUM (key);

      if (k & shift_modifier as EMACS_INT) != 0
	{ XSETINT! (new_key, k & !(shift_modifier as EMACS_INT)); }
      else if CHARACTERP (make_fixnum (k & !(CHAR_MODIFIER_MASK as EMACS_INT)))
	{
	  let dc: c_int = downcase ((k & !(CHAR_MODIFIER_MASK as EMACS_INT)) as c_int);
	  if dc as EMACS_INT == (k & !(CHAR_MODIFIER_MASK as EMACS_INT))
	    { break 'not_upcase; }
	  XSETINT! (new_key, (dc as EMACS_INT) | (k & CHAR_MODIFIER_MASK as EMACS_INT));
	}
      else
	{ break 'not_upcase; }

      original_uppercase = key;
      original_uppercase_position = t - 1;

      /* We have to do this unconditionally, regardless of whether
	 the lower-case char is defined in the keymaps, because they
	 might get translated through function-key-map.  */
      *keybuf.offset ((t - 1) as isize) = new_key;
      mock_input = max! (t, mock_input);
      shift_translated = true;

      continue 'replay_sequence;
    }
 }

  /* not_upcase: */
  if NILP (current_binding)
      && help_char_p (EVENT_HEAD (key)) && t > 1
    {
      read_key_sequence_cmd = Vprefix_help_command;
      break 'done;
    }

  /* If KEY is not defined in any of the keymaps,
     and cannot be part of a function key or translation,
     and is a shifted function key,
     use the corresponding unshifted function key instead.  */
  if NILP (current_binding)
      && /* indec.start >= t && fkey.start >= t && */ keytran.start >= t
    {
      let breakdown: Lisp_Object = parse_modifiers (key);
      let modifiers: c_int
	= if CONSP (breakdown) { XFIXNUM (XCAR (XCDR (breakdown))) as c_int } else { 0 };

      if translate_upper_case_key_bindings
	  && ((modifiers & shift_modifier) != 0
	      /* Treat uppercase keys as shifted.  */
	      || (FIXNUMP (key)
		  && (KEY_TO_CHAR! (key)
		      < (*XCHAR_TABLE (BVAR! (current_buffer, downcase_table_))).header.size as EMACS_INT)
		  && uppercasep (KEY_TO_CHAR! (key) as c_int)))
	{
	  let new_key: Lisp_Object
	    = if (modifiers & shift_modifier) != 0
		{ apply_modifiers (modifiers & !shift_modifier, XCAR (breakdown)) }
	      else { make_fixnum ((downcase (KEY_TO_CHAR! (key) as c_int) | modifiers) as EMACS_INT) };

	  original_uppercase = key;
	  original_uppercase_position = t - 1;

	  /* We have to do this unconditionally, regardless of whether
	     the lower-case char is defined in the keymaps, because they
	     might get translated through function-key-map.  */
	  *keybuf.offset ((t - 1) as isize) = new_key;
	  mock_input = max! (t, mock_input);
	  /* Reset fkey (and consequently keytran) to apply
	     function-key-map on the result, so that S-backspace is
	     correctly mapped to DEL (via backspace).  OTOH,
	     input-decode-map doesn't need to go through it again.  */
	  fkey.end = 0;
	  fkey.start = fkey.end;
	  keytran.end = 0;
	  keytran.start = keytran.end;
	  shift_translated = true;

	  continue 'replay_sequence;
	}
    }
  break 'replay_key;
  }
  }
  read_key_sequence_cmd = current_binding;
  break 'replay_sequence;
  }
  break 'replay_entire_sequence;
  }
 }
  read_key_sequence_remapped
    /* Remap command through active keymaps.
       Do the remapping here, before the unbind_to so it uses the keymaps
       of the appropriate buffer.  */
    = if SYMBOLP (read_key_sequence_cmd)
	{ Fcommand_remapping (read_key_sequence_cmd, Qnil (), Qnil ()) }
      else { Qnil () };

  unread_switch_frame = delayed_switch_frame;
  unbind_to (count, Qnil ());

  /* Don't downcase the last character if the caller says don't.
     Don't downcase it if the result is undefined, either.  */
  if (dont_downcase_last || NILP (current_binding))
      && t > 0
      && t - 1 == original_uppercase_position
    {
      *keybuf.offset ((t - 1) as isize) = original_uppercase;
      shift_translated = false;
    }

  if shift_translated
    { Vthis_command_keys_shift_translated = Qt (); }

  /* Occasionally we fabricate events, perhaps by expanding something
     according to function-key-map, or by adding a prefix symbol to a
     mouse click in the scroll bar or modeline.  In this cases, return
     the entire generated key sequence, even if we hit an unbound
     prefix or a definition before the end.  This means that you will
     be able to push back the event properly, and also means that
     read-key-sequence will always return a logical unit.

     Better ideas?  */
  while t < mock_input
    {
      add_command_key (*keybuf.offset (t as isize));
      t += 1;
    }
  echo_update ();

  return t;
}

/* ============================================================================
   C 11906-12530: the reader's callers and the input-pending / recent-keys /
   this-command-keys / suspend layer.  C 11906-11959 is `read_key_sequence_vs`; the two
   `read-key-sequence` subrs follow; then `detect_input_pending`,
   `detect_input_pending_run_timers`, `clear_input_pending`,
   `requeued_command_events_pending_p`; the `input-pending-p`/`insert-special-event` subrs;
   `update_recent_keys`; the `lossage-size`/`recent-keys`/`this-command-keys`/
   `set--this-command-keys`/`this-command-keys-vector`/`this-single-command-keys`/
   `this-single-command-raw-keys`/`clear-this-command-keys`/`recursion-depth`/
   `open-dribble-file`/`discard-input`/`suspend-emacs` subrs; and the three TTY/waiting
   entry points `stuff_buffered_input`, `set_waiting_for_input`,
   `clear_waiting_for_input`.  The boundary is C 12530 (the end of
   `clear_waiting_for_input`); C 12532-12536 is `handle_interrupt_signal`'s comment (its
   definition begins at C 12538), not ported.
   ============================================================================ */
/* C 11906-11959: `static Lisp_Object read_key_sequence_vs (Lisp_Object prompt,
   Lisp_Object continue_echo, Lisp_Object dont_downcase_last,
   Lisp_Object can_return_switch_frame, Lisp_Object cmd_loop, bool allow_string,
   bool disable_text_conversion);`  */
unsafe fn read_key_sequence_vs (prompt: Lisp_Object, continue_echo: Lisp_Object,
				dont_downcase_last: Lisp_Object,
				can_return_switch_frame: Lisp_Object,
				cmd_loop: Lisp_Object, allow_string: bool,
				disable_text_conversion: bool) -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  if !NILP (prompt)
    { CHECK_STRING (prompt); }
  maybe_quit ();

  specbind (Qinput_method_exit_on_first_char (),
	    if NILP (cmd_loop) { Qt () } else { Qnil () });
  specbind (Qinput_method_use_echo_area (),
	    if NILP (cmd_loop) { Qt () } else { Qnil () });

  if NILP (continue_echo)
    {
      this_command_key_count = 0;
      this_single_command_key_start = 0;
    }

  /* C 11930-11933: `#ifdef HAVE_WINDOW_SYSTEM` (dead on this target; carried per P-51).  */
  #[cfg (feature = "have_window_system")]
  if display_hourglass_p
    { cancel_hourglass (); }

  raw_keybuf_count = 0;
  /* C's `Lisp_Object keybuf[READ_KEY_ELTS];` is uninitialised; the port zero-initialises it
     (PORTING.md 1.5, the `command_loop_1` precedent).  */
  let mut keybuf: [Lisp_Object; READ_KEY_ELTS as usize] = [lisp_h_Qnil (); READ_KEY_ELTS as usize];
  let i: c_int = read_key_sequence (keybuf.as_mut_ptr (), prompt, !NILP (dont_downcase_last),
				    !NILP (can_return_switch_frame), false, false,
				    disable_text_conversion);

  /* C 11941-11948: `#if 0` (dead in C itself; recorded here, not transcribed — P-08).  */

  if i == -1
    {
      Vquit_flag = Qt ();
      maybe_quit ();
    }

  return unbind_to (count,
		    if allow_string
		      { make_event_array (i as ptrdiff_t, keybuf.as_mut_ptr ()) }
		    else { Fvector (i as ptrdiff_t, keybuf.as_mut_ptr ()) });
}

DEFUN! ("read-key-sequence", Fread_key_sequence, Sread_key_sequence, 1, 6, 0,
       doc: "Read a sequence of keystrokes and return as a string or vector.\nThe sequence is sufficient to specify a non-prefix command in the\ncurrent local and global maps.\n\nFirst arg PROMPT is a prompt string.  If nil, do not prompt specially.\nSecond (optional) arg CONTINUE-ECHO, if non-nil, means this key echos\nas a continuation of the previous key.\n\nThe third (optional) arg DONT-DOWNCASE-LAST, if non-nil, means do not\nconvert the last event to lower case.  (Normally any upper case event\nis converted to lower case if the original event is undefined and the lower\ncase equivalent is defined.)  A non-nil value is appropriate for reading\na key sequence to be defined.\n\nA C-g typed while in this function is treated like any other character,\nand `quit-flag' is not set.\n\nIf the key sequence starts with a mouse click, then the sequence is read\nusing the keymaps of the buffer of the window clicked in, not the buffer\nof the selected window as normal.\n\n`read-key-sequence' drops unbound button-down events, since you normally\nonly care about the click or drag events which follow them.  If a drag\nor multi-click event is unbound, but the corresponding click event would\nbe bound, `read-key-sequence' turns the event into a click event at the\ndrag's starting position.  This means that you don't have to distinguish\nbetween click and drag, double, or triple events unless you want to.\n\n`read-key-sequence' prefixes mouse events on mode lines, the vertical\nlines separating windows, and scroll bars with imaginary keys\n`mode-line', `vertical-line', and `vertical-scroll-bar'.\n\nOptional fourth argument CAN-RETURN-SWITCH-FRAME non-nil means that this\nfunction will process a switch-frame event if the user switches frames\nbefore typing anything.  If the user switches frames in the middle of a\nkey sequence, or at the start of the sequence but CAN-RETURN-SWITCH-FRAME\nis nil, then the event will be put off until after the current key sequence.\n\n`read-key-sequence' checks `function-key-map' for function key\nsequences, where they wouldn't conflict with ordinary bindings.  See\n`function-key-map' for more details.\n\nThe optional fifth argument CMD-LOOP, if non-nil, means\nthat this key sequence is being read by something that will\nread commands one after another.  It should be nil if the caller\nwill read just one key sequence.\n\nThe optional sixth argument DISABLE-TEXT-CONVERSION, if non-nil, means\ndisable input method text conversion for the duration of reading this\nkey sequence, and that keyboard input will always result in key events\nbeing sent.",
       (prompt: Lisp_Object, continue_echo: Lisp_Object, dont_downcase_last: Lisp_Object,
        can_return_switch_frame: Lisp_Object, cmd_loop: Lisp_Object,
        disable_text_conversion: Lisp_Object),
       {
  return read_key_sequence_vs (prompt, continue_echo, dont_downcase_last,
			       can_return_switch_frame, cmd_loop, true,
			       !NILP (disable_text_conversion));
});

DEFUN! ("read-key-sequence-vector", Fread_key_sequence_vector, Sread_key_sequence_vector, 1, 6, 0,
       doc: "Like `read-key-sequence' but always return a vector.",
       (prompt: Lisp_Object, continue_echo: Lisp_Object, dont_downcase_last: Lisp_Object,
        can_return_switch_frame: Lisp_Object, cmd_loop: Lisp_Object,
        disable_text_conversion: Lisp_Object),
       {
  return read_key_sequence_vs (prompt, continue_echo, dont_downcase_last,
			       can_return_switch_frame, cmd_loop, false,
			       !NILP (disable_text_conversion));
});

/* Return true if input events are pending.  */

/* C 12036-12040: `bool detect_input_pending (void);` — C's non-`static` definition is an
   exported symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_input_pending () -> bool
{
  return input_pending || get_input_pending (0);
}

/* Return true if input events are pending, and run any pending timers.  */

/* C 12044-12056: `bool detect_input_pending_run_timers (bool do_display);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_input_pending_run_timers (do_display: bool) -> bool
{
  let old_timers_run: c_uint = timers_run;

  if !input_pending
    { get_input_pending (READABLE_EVENTS_DO_TIMERS_NOW); }

  if old_timers_run != timers_run && do_display
    { redisplay_preserve_echo_area (8); }

  return input_pending;
}

/* This is called in some cases before a possible quit.
   It cases the next call to detect_input_pending to recompute input_pending.
   So calling this function unnecessarily can't do any harm.  */

/* C 12062-12066: `void clear_input_pending (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_input_pending ()
{
  input_pending = false;
}

/* Return true if there are pending requeued command events.  */

/* C 12070-12074: `bool requeued_command_events_pending_p (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn requeued_command_events_pending_p () -> bool
{
  return CONSP (Vunread_command_events);
}

DEFUN! ("input-pending-p", Finput_pending_p, Sinput_pending_p, 0, 1, 0,
       doc: "Return t if command input is currently available with no wait.\nActually, the value is nil only if we can be sure that no input is available;\nif there is a doubt, the value is t.\n\nIf CHECK-TIMERS is non-nil, timers that are ready to run will do so.",
       (check_timers: Lisp_Object),
       {
  if requeued_events_pending_p ()
    { return Qt (); }

  /* Process non-user-visible events (Bug#10195).  */
  process_special_events ();

  return if get_input_pending ((if NILP (check_timers) { 0 }
				else { READABLE_EVENTS_DO_TIMERS_NOW })
			       | READABLE_EVENTS_FILTER_EVENTS)
	   { Qt () } else { Qnil () };
});

DEFUN! ("insert-special-event", Finsert_special_event, Sinsert_special_event, 1, 1, 0,
       doc: "Insert the special EVENT into the input event queue.\nOnly 'input_event' slots KIND and ARG are set.",
       (event: Lisp_Object),
       {
  /* Check, that it is a special event.  */
  CHECK_CONS (event);
  /* Construct an input event.  */
  let mut ie: input_event = input_event {
    bits: 0, code: 0, modifiers: 0, x: lisp_h_Qnil (), y: lisp_h_Qnil (), timestamp: 0,
    frame_or_window: lisp_h_Qnil (), arg: lisp_h_Qnil (), device: lisp_h_Qnil () };
  EVENT_INIT (&raw mut ie);
  /* C's ternary chain carries an `#ifdef` inside the `? :`; the Rust grammar has no
     attribute position for `else if`, so the port keeps C's arm order in a labelled block
     whose `break` yields the arm's value and gates each dead arm with `#[cfg]` (P-51: the
     text of every arm stays in the file).  */
  let kind_value: c_int = 'kind: {
    if EQ (XCAR (event), Qdelete_frame ())
      { break 'kind DELETE_WINDOW_EVENT; }
    #[cfg (feature = "have_ntgui")]
    if EQ (XCAR (event), Qend_session ())
      { break 'kind END_SESSION_EVENT; }
    #[cfg (feature = "have_ns")]
    if EQ (XCAR (event), Qns_put_working_text ())
      { break 'kind KEY_NS_PUT_WORKING_TEXT; }
    #[cfg (feature = "have_ns")]
    if EQ (XCAR (event), Qns_unput_working_text ())
      { break 'kind KEY_NS_UNPUT_WORKING_TEXT; }
    if EQ (XCAR (event), Qiconify_frame ())
      { break 'kind ICONIFY_EVENT; }
    if EQ (XCAR (event), Qmake_frame_visible ())
      { break 'kind DEICONIFY_EVENT; }
    /* : EQ (XCAR (event), Qselect_window) ? SELECT_WINDOW_EVENT */
    if EQ (XCAR (event), Qsave_session ())
      { break 'kind SAVE_SESSION_EVENT; }
    #[cfg (feature = "have_dbus")]
    if EQ (XCAR (event), Qdbus_event ())
      { break 'kind DBUS_EVENT; }
    #[cfg (feature = "threads_enabled")]
    if EQ (XCAR (event), Qthread_event ())
      { break 'kind THREAD_EVENT; }
    #[cfg (feature = "use_file_notify")]
    if EQ (XCAR (event), Qfile_notify ())
      { break 'kind FILE_NOTIFY_EVENT; }
    if EQ (XCAR (event), Qconfig_changed_event ())
      { break 'kind CONFIG_CHANGED_EVENT; }
    #[cfg (feature = "windowsnt")]
    if EQ (XCAR (event), Qlanguage_change ())
      { break 'kind LANGUAGE_CHANGE_EVENT; }
    if EQ (XCAR (event), Qfocus_in ())
      { break 'kind FOCUS_IN_EVENT; }
    if EQ (XCAR (event), Qfocus_out ())
      { break 'kind FOCUS_OUT_EVENT; }
    if EQ (XCAR (event), Qmove_frame ())
      { break 'kind MOVE_FRAME_EVENT; }
    if EQ (XCAR (event), Qsleep_event ())
      { break 'kind SLEEP_EVENT; }
    signal_error (c"Invalid special event kind".as_ptr (), XCAR (event));
    NO_EVENT
  };
  input_event_bits! (kind, &raw mut ie, kind_value);
  ie.frame_or_window = Qnil ();
  ie.arg = CDR (event);

  /* Store it into the input event queue.  (`keyboard.h` 498's mirror spells the parameter
     `*mut c_void`, so the call site casts — the `process_special_events` shape.)  */
  kbd_buffer_store_event (&raw mut ie as *mut c_void);

  return Qnil ();
});

/* Reallocate recent_keys copying the recorded keystrokes
   in the right order.  */
/* C 12151-12171: `static void update_recent_keys (int new_size, int kept_keys);`  */
unsafe fn update_recent_keys (new_size: c_int, kept_keys: c_int)
{
  let osize: c_int = ASIZE (recent_keys) as c_int;
  eassert! (recent_keys_index < osize);
  eassert! (kept_keys <= min! (osize, new_size));
  let mut v: Lisp_Object = make_nil_vector (new_size as ptrdiff_t);
  let mut i: c_int;
  let mut idx: c_int;
  i = 0;
  while i < kept_keys
    {
      idx = recent_keys_index - kept_keys + i;
      while idx < 0
        { idx += osize; }
      ASET (v, i as isize, AREF (recent_keys, idx as isize));
      i += 1;
    }
  recent_keys = v;
  total_keys = kept_keys;
  recent_keys_index = total_keys % new_size;
  lossage_limit = new_size;
}

DEFUN! ("lossage-size", Flossage_size, Slossage_size, 0, 1,
       "(list (read-number \"Set maximum keystrokes to: \" (lossage-size)))",
       doc: "Return or set the maximum number of keystrokes to save.\nIf called with a non-nil ARG, set the limit to ARG and return it.\nOtherwise, return the current limit.\n\nThe saved keystrokes are shown by `view-lossage'.",
       (arg: Lisp_Object),
       {
  if NILP (arg)
    { return make_fixnum (lossage_limit as EMACS_INT); }

  if !FIXNATP (arg)
    { user_error (c"Value must be a positive integer".as_ptr ()); }
  let osize: ptrdiff_t = ASIZE (recent_keys);
  eassert! (lossage_limit as ptrdiff_t == osize);
  let min_size: c_int = MIN_NUM_RECENT_KEYS;
  let new_size: EMACS_INT = XFIXNAT (arg);

  if new_size == osize as EMACS_INT
    { return make_fixnum (lossage_limit as EMACS_INT); }

  if new_size < min_size as EMACS_INT
    {
      AUTO_STRING! (fmt, c"Value must be >= %d".as_ptr ());
      Fsignal (Quser_error (), list1 (CALLN! (Fformat, fmt, make_fixnum (min_size as EMACS_INT))));
    }
  if new_size > MAX_NUM_RECENT_KEYS as EMACS_INT
    {
      AUTO_STRING! (fmt, c"Value must be <= %d".as_ptr ());
      Fsignal (Quser_error (), list1 (CALLN! (Fformat, fmt,
					      make_fixnum (MAX_NUM_RECENT_KEYS as EMACS_INT))));
    }

  let kept_keys: c_int
    = if new_size > osize as EMACS_INT { total_keys }
      else { min! (new_size as c_int, total_keys) };
  update_recent_keys (new_size as c_int, kept_keys);

  return make_fixnum (lossage_limit as EMACS_INT);
});

DEFUN! ("recent-keys", Frecent_keys, Srecent_keys, 0, 1, 0,
       doc: "Return vector of last few events, not counting those from keyboard macros.\nIf INCLUDE-CMDS is non-nil, include the commands that were run,\nrepresented as pseudo-events of the form (nil . COMMAND).",
       (include_cmds: Lisp_Object),
       {
  let cmds: bool = !NILP (include_cmds);

  if total_keys == 0
      || (cmds && total_keys < lossage_limit)
    { return Fvector (total_keys as ptrdiff_t, (*XVECTOR (recent_keys)).contents.as_mut_ptr ()); }
  else
    {
      let mut es: Lisp_Object = Qnil ();
      let mut i: c_int = if total_keys < lossage_limit { 0 } else { recent_keys_index };
      eassert! (recent_keys_index < lossage_limit);
      loop
	{
	  let e: Lisp_Object = AREF (recent_keys, i as isize);
	  if cmds || !CONSP (e) || !NILP (XCAR (e))
	    { es = Fcons (e, es); }
	  i += 1;
	  if i >= lossage_limit
	    { i = 0; }
	  if i == recent_keys_index
	    { break; }
	}
      es = Fnreverse (es);
      return Fvconcat (1, &raw mut es);
    }
});

DEFUN! ("this-command-keys", Fthis_command_keys, Sthis_command_keys, 0, 0, 0,
       doc: "Return the key sequence that invoked this command.\nHowever, if the command has called `read-key-sequence', it returns\nthe last key sequence that has been read.\nThe value is a string or a vector.\n\nSee also `this-command-keys-vector'.",
       (),
       {
  return make_event_array (this_command_key_count,
			   (*XVECTOR (this_command_keys)).contents.as_mut_ptr ());
});

DEFUN! ("set--this-command-keys", Fset__this_command_keys, Sset__this_command_keys, 1, 1, 0,
       doc: "Set the vector to be returned by `this-command-keys'.\nThe argument KEYS must be a string.\nInternal use only.",
       (keys: Lisp_Object),
       {
  CHECK_STRING (keys);

  this_command_key_count = 0;
  this_single_command_key_start = 0;

  let mut charidx: ptrdiff_t = 0;
  let mut byteidx: ptrdiff_t = 0;
  let mut key0: c_int = fetch_string_char_advance (keys, &raw mut charidx, &raw mut byteidx);
  if CHAR_BYTE8_P (key0)
    { key0 = CHAR_TO_BYTE8 (key0); }

  /* Kludge alert: this makes M-x be in the form expected by
     novice.el.  (248 is \370, a.k.a. "Meta-x".)  Any better ideas?  */
  if key0 == 248
    { add_command_key (make_fixnum ((b'x' as c_int | meta_modifier) as EMACS_INT)); }
  else
    { add_command_key (make_fixnum (key0 as EMACS_INT)); }
  let mut i: ptrdiff_t = 1;
  while i < SCHARS (keys)
    {
      let key_i: c_int = fetch_string_char_advance (keys, &raw mut charidx, &raw mut byteidx);
      let key_i: c_int = if CHAR_BYTE8_P (key_i) { CHAR_TO_BYTE8 (key_i) } else { key_i };
      add_command_key (make_fixnum (key_i as EMACS_INT));
      i += 1;
    }
  return Qnil ();
});

DEFUN! ("this-command-keys-vector", Fthis_command_keys_vector, Sthis_command_keys_vector, 0, 0, 0,
       doc: "Return the key sequence that invoked this command, as a vector.\nHowever, if the command has called `read-key-sequence', it returns\nthe last key sequence that has been read.\n\nSee also `this-command-keys'.",
       (),
       {
  return Fvector (this_command_key_count,
		  (*XVECTOR (this_command_keys)).contents.as_mut_ptr ());
});

DEFUN! ("this-single-command-keys", Fthis_single_command_keys, Sthis_single_command_keys, 0, 0, 0,
       doc: "Return the key sequence that invoked this command.\nMore generally, it returns the last key sequence read, either by\nthe command loop or by `read-key-sequence'.\nThe value is always a vector.",
       (),
       {
  let nkeys: ptrdiff_t = this_command_key_count - this_single_command_key_start;
  return Fvector (if nkeys < 0 { 0 } else { nkeys },
		  (*XVECTOR (this_command_keys)).contents.as_mut_ptr ()
		    .offset (this_single_command_key_start));
});

DEFUN! ("this-single-command-raw-keys", Fthis_single_command_raw_keys,
       Sthis_single_command_raw_keys, 0, 0, 0,
       doc: "Return the raw events that were read for this command.\nMore generally, it returns the last key sequence read, either by\nthe command loop or by `read-key-sequence'.\nUnlike `this-single-command-keys', this function's value\nshows the events before all translations (except for input methods).\nThe value is always a vector.",
       (),
       {
  return Fvector (raw_keybuf_count as ptrdiff_t, (*XVECTOR (raw_keybuf)).contents.as_mut_ptr ());
});

DEFUN! ("clear-this-command-keys", Fclear_this_command_keys, Sclear_this_command_keys, 0, 1, 0,
       doc: "Clear out the vector that `this-command-keys' returns.\nAlso clear the record of the last 300 input events, unless optional arg\nKEEP-RECORD is non-nil.",
       (keep_record: Lisp_Object),
       {
  let mut i: c_int;

  this_command_key_count = 0;

  if NILP (keep_record)
    {
      i = 0;
      while i < ASIZE (recent_keys) as c_int
	{
	  ASET (recent_keys, i as isize, Qnil ());
	  i += 1;
	}
      total_keys = 0;
      recent_keys_index = 0;
    }
  return Qnil ();
});

DEFUN! ("recursion-depth", Frecursion_depth, Srecursion_depth, 0, 0, 0,
       doc: "Return the current depth in recursive edits.",
       (),
       {
  let mut sum: EMACS_INT;
  /* C's `ckd_add (&sum, command_loop_level, minibuf_level);` stores the wrapped sum and
     yields the overflow flag, which C discards here (the `ckd_add`/`overflowing_add`
     family of docs/ports/token-aliases.tsv, P-15; the `buffer.rs` shape).  */
  let (sum_, ckd_add_overflow) = command_loop_level.overflowing_add (minibuf_level);
  sum = sum_;
  let _ = ckd_add_overflow;
  return make_fixnum (sum);
});

DEFUN! ("open-dribble-file", Fopen_dribble_file, Sopen_dribble_file, 1, 1,
       "FOpen dribble file: ",
       doc: "Start writing input events to a dribble file called FILE.\nAny previously open dribble file will be closed first.  If FILE is\nnil, just close the dribble file, if any.\n\nIf the file is still open when Emacs exits, it will be closed then.\n\nThe events written to the file include keyboard and mouse input\nevents, but not events from executing keyboard macros.  The events are\nwritten to the dribble file immediately without line buffering.\n\nBe aware that this records ALL characters you type!\nThis may include sensitive information such as passwords.",
       (file: Lisp_Object),
       {
  if !dribble.is_null ()
    {
      block_input ();
      emacs_fclose! (dribble);
      unblock_input ();
      dribble = std::ptr::null_mut ();
    }
  if !NILP (file)
    {
      /* C reassigns its `file` parameter; the port keeps C's name in a local, since a
	 `DEFUN!` parameter cannot be `mut` (PORTING.md 5.1).  */
      let mut file: Lisp_Object = file;
      let mut fd: c_int;
      let encfile: Lisp_Object;

      file = Fexpand_file_name (file, Qnil ());
      encfile = ENCODE_FILE (file);
      fd = emacs_open (SSDATA (encfile), O_WRONLY | O_CREAT | O_EXCL, 0o600);
      if fd < 0 && *errno () == EEXIST
	  && (emacs_unlink (SSDATA (encfile)) == 0 || *errno () == ENOENT)
	{ fd = emacs_open (SSDATA (encfile), O_WRONLY | O_CREAT | O_EXCL, 0o600); }
      dribble = if fd < 0 { std::ptr::null_mut () } else { emacs_fdopen (fd, c"w".as_ptr ()) };
      if dribble.is_null ()
	{ report_file_error (c"Opening dribble".as_ptr (), file); }
    }
  return Qnil ();
});

DEFUN! ("discard-input", Fdiscard_input, Sdiscard_input, 0, 0, 0,
       doc: "Discard the contents of the terminal input buffer.\nAlso end any kbd macro being defined.",
       (),
       {
  if !NILP (KVAR! (current_kboard, defining_kbd_macro))
    {
      /* Discard the last command from the macro.  */
      Fcancel_kbd_macro_events ();
      end_kbd_macro ();
    }

  Vunread_command_events = Qnil ();

  discard_tty_input ();

  kbd_fetch_ptr = kbd_store_ptr;
  input_pending = false;

  return Qnil ();
});

DEFUN! ("suspend-emacs", Fsuspend_emacs, Ssuspend_emacs, 0, 1, "",
       doc: "Stop Emacs and return to superior process.  You can resume later.\nIf `cannot-suspend' is non-nil, or if the system doesn't support job\ncontrol, run a subshell instead.\n\nIf optional arg STUFFSTRING is non-nil, stuff it and then a newline as\nkeyboard input, if Emacs is running interactively on a terminal and the\nplatform supports and allows stuffing; this may need special privileges.\n\nBefore suspending, run the normal hook `suspend-hook'.\nAfter resumption run the normal hook `suspend-resume-hook'.\n\nSome operating systems cannot stop the Emacs process and resume it later.\nOn such systems, Emacs starts a subshell instead of suspending.",
       (stuffstring: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  /* C's four locals are uninitialised and filled by `get_tty_size`; Rust's definite
     initialisation cannot see through the call, so the port zero-initialises them
     (PORTING.md 1.5).  */
  let mut old_height: c_int = 0;
  let mut old_width: c_int = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  if !tty_list.is_null () && !(*tty_list).next.is_null ()
    { error (c"There are other tty frames open; close them before suspending Emacs".as_ptr ()); }

  if !NILP (stuffstring)
    { CHECK_STRING (stuffstring); }

  run_hook (Qsuspend_hook ());

  get_tty_size (fileno ((*CURTTY! ()).input), &raw mut old_width, &raw mut old_height);
  reset_all_sys_modes ();
  /* sys_suspend can get an error if it tries to fork a subshell
     and the system resources aren't available for that.  */
  record_unwind_protect_void (Some (init_all_sys_modes));
  stuff_buffered_input (stuffstring);
  if cannot_suspend
    { sys_subshell (); }
  else
    { sys_suspend (); }
  unbind_to (count, Qnil ());

  /* Check if terminal/window size has changed.
     Note that this is not useful when we are running directly
     with a window system; but suspend should be disabled in that case.  */
  get_tty_size (fileno ((*CURTTY! ()).input), &raw mut width, &raw mut height);
  if width != old_width || height != old_height
    { change_frame_size (SELECTED_FRAME (), width, height, false, false, false); }

  run_hook (Qsuspend_resume_hook ());

  return Qnil ();
});

/* If STUFFSTRING is a string, stuff its contents and then a newline as
   pending terminal input.  Then stuff anything Emacs has read ahead and
   not used.  However, do nothing if stuffing does not work.  */

/* C 12474-12508: `void stuff_buffered_input (Lisp_Object stuffstring);` — the live
   `#ifdef SIGTSTP` arm (SIGTSTP is 18 in the frozen configuration).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stuff_buffered_input (stuffstring: Lisp_Object)
{
  let mut bad_stuff: c_int = 0;

  if STRINGP (stuffstring)
    {
      let mut p: *mut c_char = SSDATA (stuffstring);
      let mut i: ptrdiff_t = SBYTES (stuffstring);
      while bad_stuff == 0 && 0 < i
	{
	  /* C's `*p++`: the load uses the old pointer, then `p` steps.  */
	  let c: c_char = *p;
	  p = p.add (1);
	  bad_stuff = stuff_char (c);
	  i -= 1;
	}
      if bad_stuff == 0
	{ bad_stuff = stuff_char (b'\n' as c_char); }
    }

  /* Anything we have read ahead, put back for the shell to read.
     When we have multiple keyboards, we should stuff everything back
     into the keyboard it came from, but fixing this is low priority as
     many systems prohibit stuffing for security reasons.  */
  while kbd_fetch_ptr != kbd_store_ptr
    {
      if (*kbd_fetch_ptr).kind == ASCII_KEYSTROKE_EVENT as u16 && bad_stuff == 0
	{ bad_stuff = stuff_char ((*kbd_fetch_ptr).ie.code as c_char); }

      clear_event (&raw mut (*kbd_fetch_ptr).ie);
      kbd_fetch_ptr = next_kbd_event (kbd_fetch_ptr);
    }

  input_pending = false;
}

/* C 12510-12522: `void set_waiting_for_input (struct timespec *time_to_clear);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_waiting_for_input (time_to_clear: *mut timespec)
{
  input_available_clear_time = time_to_clear;

  /* Tell handle_interrupt to throw back to read_char.  */
  waiting_for_input! () = true;

  /* If handle_interrupt was called before and buffered a C-g,
     make it run again now, to avoid timing error.  */
  if !NILP (Vquit_flag)
    { quit_throw_to_read_char (false); }
}

/* C 12524-12530: `void clear_waiting_for_input (void);`  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_waiting_for_input ()
{
  /* Tell handle_interrupt not to throw back to read_char,  */
  waiting_for_input! () = false;
  input_available_clear_time = std::ptr::null_mut ();
}
/* frame.h 1387-1390: `CHECK_FRAME`/`CHECK_LIVE_FRAME` are macros in C; the mirror does not
   carry them (`frame.rs` defines its own copies), so this unit defines the one it uses with
   C's body (PORTING.md P-72 rule 3).  */
macro_rules! CHECK_LIVE_FRAME {
  ($x:expr) => { CHECK_TYPE (FRAMEP ($x) && FRAME_LIVE_P (XFRAME ($x)), Qframe_live_p (), $x) };
}

/* ============================================================================
   C 12532-13166: the SIGINT/signal-handler and interrupt layer
   (`handle_interrupt_signal`, `deliver_interrupt_signal`, `write_stdout`,
   `read_stdin`, the `force_quit_count` object, `handle_interrupt`,
   `quit_throw_to_read_char`), the input-mode subr family
   (`set-input-interrupt-mode`, `set-output-flow-control`, `set-input-meta-mode`,
   `set-quit-char`, `set-input-mode`, `current-input-mode`), the three
   position subrs (`posn-at-x-y`, `posn-at-point`, `posn-point`) and the two
   kboard allocators `init_kboard`/`allocate_kboard`.  The boundary is C 13166
   (the end of `allocate_kboard`); C 13173's `wipe_kboard` is the next live
   definition.
   ============================================================================ */
/* The SIGINT handler.

   If we have a frame on the controlling tty, we assume that the
   SIGINT was generated by C-g, so we call handle_interrupt.
   Otherwise, tell maybe_quit to kill Emacs.  */

/* C 12538-12569: `static void handle_interrupt_signal (int sig);` — C hands its address to
   `deliver_process_signal`, so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4;
   the `handle_user_signal` shape).  */
unsafe extern "C" fn handle_interrupt_signal (sig: c_int)
{
  /* See if we have an active terminal on our controlling tty.  */
  let terminal: *mut terminal = get_named_terminal (dev_tty);
  if terminal.is_null ()
    {
      /* There are no frames, so either 'quit' or exit.
         If 'kill-emacs-on-sigint', pretend that we are a
         well-behaving UN*X program and exit.  We must not call Lisp
         in a signal handler, so tell maybe_quit to exit when it is
         safe.  */
      Vquit_flag = if kill_emacs_on_sigint
		      /* Don't risk running ELisp code while shutting down
			 and limit the effect of 'kill_emacs_on_sigint'
			 to batch sessions.  */
		      || NILP (Vrun_hooks) || !noninteractive
		    { Qkill_emacs () } else { Qt () };
    }
  else
    {
      /* Otherwise, the SIGINT was probably generated by C-g.  */

      /* Set internal_last_event_frame to the top frame of the
         controlling tty, if we have a frame there.  We disable the
         interrupt key on secondary ttys, so the SIGINT must have come
         from the controlling tty.  */
      internal_last_event_frame
	= (*( (*terminal).display_info.tty as *mut tty_display_info )).top_frame;

      handle_interrupt (true);
    }
}

/* C 12571-12575: `static void deliver_interrupt_signal (int sig);` — C hands its address to
   `emacs_sigaction_init` (C 13262), so the port defines it `unsafe extern "C" fn`.  */
unsafe extern "C" fn deliver_interrupt_signal (sig: c_int)
{
  deliver_process_signal (sig, Some (handle_interrupt_signal));
}

/* Output MSG directly to standard output, without buffering.  Ignore
   failures.  This is safe in a signal handler.  */
/* C 12579-12583: `static void write_stdout (char const *msg);`  */
unsafe fn write_stdout (msg: *const c_char)
{
  ignore_value! (write (STDOUT_FILENO, msg as *const c_void, strlen (msg)));
}

/* Read a byte from stdin, without buffering.  Safe in signal handlers.  */
/* C 12586-12591: `static int read_stdin (void);`  */
unsafe fn read_stdin () -> c_int
{
  /* C's `char c;` is uninitialised; the port zero-initialises it (PORTING.md 1.5).  */
  let mut c: c_char = 0;
  return if read (STDIN_FILENO, &raw mut c as *mut c_void, 1) == 1 { c as c_int } else { EOF };
}

/* If Emacs is stuck because `inhibit-quit' is true, then keep track
   of the number of times C-g has been requested.  If C-g is pressed
   enough times, then quit anyway.  See bug#6585.  */
/* C 12596: `static int volatile force_quit_count;` — Rust has no `volatile` statics, so the
   object stays an ordinary `static mut` and every access is already in an `unsafe` block
   (the `interrupt_input_blocked` reading at C 100-105).  */
static mut force_quit_count: c_int = 0;

/* This routine is called at interrupt level in response to C-g.

   It is called from the SIGINT handler or kbd_buffer_store_event.

   If `waiting_for_input' is non zero, then unless `echoing' is
   nonzero, immediately throw back to read_char.

   Otherwise it sets the Lisp variable quit-flag not-nil.  This causes
   eval to throw, when it gets a chance.  If quit-flag is already
   non-nil, it stops the job right away.  */

/* C 12609-12745: `static void handle_interrupt (bool in_signal_handler);`  */
unsafe fn handle_interrupt (in_signal_handler: bool)
{
  /* C's `char c;` is uninitialised; the port zero-initialises it (PORTING.md 1.5).  */
  let mut c: c_char = 0;

  cancel_echoing ();

  /* XXX This code needs to be revised for multi-tty support.  */
  if !NILP (Vquit_flag) && !get_named_terminal (dev_tty).is_null ()
    {
      if !in_signal_handler
	{
	  /* If SIGINT isn't blocked, don't let us be interrupted by
	     a SIGINT.  It might be harmful due to non-reentrancy
	     in I/O functions.  */
	  let mut blocked: sigset_t = std::mem::zeroed ();
	  sigemptyset (&raw mut blocked);
	  sigaddset (&raw mut blocked, SIGINT);
	  pthread_sigmask (SIG_BLOCK, &raw const blocked, std::ptr::null_mut ());
	  fflush (stdout);
	}

      reset_all_sys_modes ();

      /* C 12633-12649: `#ifdef SIGTSTP` is live (`SIGTSTP` is 18); the `#else` arm is
	 carried under `no_sigtstp` (P-51), with both arms gated so that flipping the feature
	 compiles C's other arm (the `sysdep.rs` `usable_sigpoll` pair shape).  */
      #[cfg (not (feature = "no_sigtstp"))]
      {
      sys_suspend ();
      }
      #[cfg (feature = "no_sigtstp")]
      {
      /* Perhaps should really fork an inferior shell?
	 But that would not provide any way to get back
	 to the original shell, ever.  */
      write_stdout (c"No support for stopping a process on this operating system;\nyou can continue or abort.\n".as_ptr ());
      }
      /* C 12650-12654: `#ifdef MSDOS` (dead; carried per P-51).  */
      #[cfg (feature = "msdos")]
      {
      /* We must remain inside the screen area when the internal terminal
	 is used.  Note that [Enter] is not echoed by dos.  */
      cursor_to (SELECTED_FRAME (), 0, 0);
      }

      write_stdout (c"Emacs is resuming after an emergency escape.\n".as_ptr ());

      /* It doesn't work to autosave while GC is in progress;
	 the code used for auto-saving doesn't cope with the mark bit.  */
      if !gc_in_progress
	{
	  write_stdout (c"Auto-save? (y or n) ".as_ptr ());
	  c = read_stdin () as c_char;
	  if c == b'y' as c_char || c == b'Y' as c_char
	    {
	      Fdo_auto_save (Qt (), Qnil ());
	      /* C 12667-12671: `#ifdef MSDOS` (dead; carried per P-51).  */
	      #[cfg (feature = "msdos")]
	      { write_stdout (c"\r\nAuto-save done".as_ptr ()); }
	      #[cfg (not (feature = "msdos"))]
	      { write_stdout (c"Auto-save done\n".as_ptr ()); }
	    }
	  while c != b'\n' as c_char
	    { c = read_stdin () as c_char; }
	}
      else
	{
	  /* During GC, it must be safe to reenable quitting again.  */
	  Vinhibit_quit = Qnil ();
	  /* C 12682-12698: `#ifdef MSDOS`'s string fragment (dead; recorded here, since a
	     `cfg`'d *fragment* of a string literal has no Rust position and the token gate
	     blanks string literals, so no token is lost).  */
	  write_stdout (c"Garbage collection in progress; cannot auto-save now\r\nbut will instead do a real quit after garbage collection ends\r\n".as_ptr ());
	}

      /* C 12690-12694: `#ifdef MSDOS` (dead; carried per P-51).  */
      #[cfg (feature = "msdos")]
      { write_stdout (c"\r\nAbort?  (y or n) ".as_ptr ()); }
      #[cfg (not (feature = "msdos"))]
      { write_stdout (c"Abort (and dump core)? (y or n) ".as_ptr ()); }
      c = read_stdin () as c_char;
      if c == b'y' as c_char || c == b'Y' as c_char
	{ emacs_abort (); }
      while c != b'\n' as c_char
	{ c = read_stdin () as c_char; }
      /* C 12700-12704: `#ifdef MSDOS` (dead; carried per P-51).  */
      #[cfg (feature = "msdos")]
      { write_stdout (c"\r\nContinuing...\r\n".as_ptr ()); }
      #[cfg (not (feature = "msdos"))]
      { write_stdout (c"Continuing...\n".as_ptr ()); }
      init_all_sys_modes ();
    }
  else
    {
      /* Request quit when it's safe.  */
      let count: c_int = if NILP (Vquit_flag) { 1 } else { force_quit_count + 1 };
      force_quit_count = count;
      if count == 3
	{ Vinhibit_quit = Qnil (); }
      Vquit_flag = Qt ();
    }

  pthread_sigmask (SIG_SETMASK, &raw const empty_mask, std::ptr::null_mut ());

/* TODO: The longjmp in this call throws the NS event loop integration off,
         and it seems to do fine without this.  Probably some attention
	 needs to be paid to the setting of waiting_for_input in
         wait_reading_process_output() under HAVE_NS because of the call
         to ns_select there (needed because otherwise events aren't picked up
         outside of polling since we don't get SIGIO like X and we don't have a
         separate event loop thread like W32.  */
  /* C 12726-12744: `#ifndef HAVE_NS` is live (`HAVE_NS` is undefined); the
     `#ifdef THREADS_ENABLED` block inside it is dead and carried per P-51.  */
  #[cfg (feature = "threads_enabled")]
  {
  /* If we were called from a signal handler, we must be in the main
     thread, see deliver_process_signal.  So we must make sure the
     main thread holds the global lock.  */
  if in_signal_handler
    {
      /* But if the signal handler was called when a non-main thread was
         in GC, just return, since switching threads by force-taking the
         global lock will confuse the heck out of GC, and will most
         likely segfault.  */
      if !main_thread_p (current_thread as *const c_void) && gc_in_progress
	{ return; }
      maybe_reacquire_global_lock ();
    }
  }
  if waiting_for_input! () && !echoing
    { quit_throw_to_read_char (in_signal_handler); }
}

/* Handle a C-g by making read_char return C-g.  */

/* C 12749-12769: `static AVOID quit_throw_to_read_char (bool from_signal);` — C's `AVOID`
   makes it `_Noreturn`, which the port spells `-> !` (the scaffold entry's reading).  */
unsafe fn quit_throw_to_read_char (from_signal: bool) -> !
{
  /* When not called from a signal handler it is safe to call
     Lisp.  */
  if !from_signal && EQ (Vquit_flag, Qkill_emacs ())
    { Fkill_emacs (Qnil (), Qnil ()); }

  /* Prevent another signal from doing this before we finish.  */
  clear_waiting_for_input ();
  input_pending = false;

  Vunread_command_events = Qnil ();

  if FRAMEP (internal_last_event_frame)
      && !EQ (internal_last_event_frame, selected_frame)
    { do_switch_frame (make_lispy_switch_frame (internal_last_event_frame),
		       0, 0, Qnil ()); }

  /* `sys_longjmp` takes `*mut jmp_buf`, which is the port's `[u8; 192]` alias while
     `getcjmp! ()` is a `sys_jmp_buf` (the `sys_setjmp` call site's cast).  */
  sys_longjmp (&raw mut getcjmp! () as *mut jmp_buf, 1);
}

DEFUN! ("set-input-interrupt-mode", Fset_input_interrupt_mode, Sset_input_interrupt_mode, 1, 1, 0,
       doc: "Set interrupt mode of reading keyboard input.\nIf INTERRUPT is non-nil, Emacs will use input interrupts;\notherwise Emacs uses CBREAK mode.\n\nSee also `current-input-mode'.",
       (interrupt: Lisp_Object),
       {
  let mut new_interrupt_input: bool;
  /* C 12781-12810: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live
     (`USABLE_SIGIO` is 1); inside it `#ifdef HAVE_X_WINDOWS` is dead and its `else` is what
     introduces the live statement, so that pair is written as the two cfg'd arms of
     `new_interrupt_input` (the `sysdep.rs` `usable_sigpoll` shape).  The outer `#else` is
     carried as `no_usable_sigio_or_sigpoll`.  */
  #[cfg (not (feature = "have_x_windows"))]
  {
  new_interrupt_input = !NILP (interrupt);
  }
  #[cfg (feature = "have_x_windows")]
  {
  if !x_display_list.is_null ()
    {
      /* When using X, don't give the user a real choice,
	 because we haven't implemented the mechanisms to support it.  */
      new_interrupt_input = true;
    }
  else
    { new_interrupt_input = !NILP (interrupt); }
  }
  #[cfg (feature = "no_usable_sigio_or_sigpoll")]
  {
  new_interrupt_input = false;
  }

  if new_interrupt_input != interrupt_input
    {
      /* C 12799-12801: `#ifdef POLL_FOR_INPUT` (dead; carried per P-51).  */
      #[cfg (feature = "poll_for_input")]
      {
      stop_polling ();
      }
      /* C 12801-12808: `#ifndef DOS_NT` is live; the `#else` arm (`interrupt_input =
	 new_interrupt_input;`) is carried under `dos_nt`.  */
      #[cfg (not (feature = "dos_nt"))]
      {
      /* this causes startup screen to be restored and messes with the mouse */
      reset_all_sys_modes ();
      interrupt_input = new_interrupt_input;
      init_all_sys_modes ();
      }
      #[cfg (feature = "dos_nt")]
      {
      interrupt_input = new_interrupt_input;
      }

      /* C 12810-12813: `#ifdef POLL_FOR_INPUT` (dead; carried per P-51).  */
      #[cfg (feature = "poll_for_input")]
      {
      poll_suppress_count = 1;
      start_polling ();
      }
    }
  return Qnil ();
});

DEFUN! ("set-output-flow-control", Fset_output_flow_control, Sset_output_flow_control, 1, 2, 0,
       doc: "Enable or disable ^S/^Q flow control for output to TERMINAL.\nIf FLOW is non-nil, flow control is enabled and you cannot use C-s or\nC-q in key sequences.\n\nThis setting only has an effect on tty terminals and only when\nEmacs reads input in CBREAK mode; see `set-input-interrupt-mode'.\n\nSee also `current-input-mode'.",
       (flow: Lisp_Object, terminal: Lisp_Object),
       {
  let t: *mut terminal = decode_tty_terminal (terminal);
  let tty: *mut tty_display_info;

  if t.is_null ()
    { return Qnil (); }
  tty = (*t).display_info.tty as *mut tty_display_info;

  if tty_bits! (flow_control, tty) != !NILP (flow)
    {
      /* C 12838-12841: `#ifndef DOS_NT` is live and has no `#else` text.  */
      /* This causes startup screen to be restored and messes with the mouse.  */
      reset_sys_modes (tty);

      tty_bits! (flow_control, tty, !NILP (flow) as c_int);

      /* C 12845-12847: `#ifndef DOS_NT` is live and has no `#else` text.  */
      init_sys_modes (tty);
    }
  return Qnil ();
});

DEFUN! ("set-input-meta-mode", Fset_input_meta_mode, Sset_input_meta_mode, 1, 2, 0,
       doc: "Enable or disable 8-bit input on TERMINAL.\nIf META is t, Emacs will accept 8-bit input, and interpret the 8th\nbit as the Meta modifier before it decodes the characters.\n\nIf META is `encoded', Emacs will interpret the 8th bit of single-byte\ncharacters after decoding the characters.\n\nIf META is nil, Emacs will ignore the top bit, on the assumption it is\nparity.\n\nOtherwise, Emacs will accept and pass through 8-bit input without\nspecially interpreting the top bit.\n\nThis setting only has an effect on tty terminal devices.\n\nOptional parameter TERMINAL specifies the tty terminal device to use.\nIt may be a terminal object, a frame, or nil for the terminal used by\nthe currently selected frame.\n\nSee also `current-input-mode'.",
       (meta: Lisp_Object, terminal: Lisp_Object),
       {
  let t: *mut terminal = decode_tty_terminal (terminal);
  let tty: *mut tty_display_info;
  let mut new_meta: c_int;

  if t.is_null ()
    { return Qnil (); }
  tty = (*t).display_info.tty as *mut tty_display_info;

  if NILP (meta)
    { new_meta = 0; }
  else if EQ (meta, Qt ())
    { new_meta = 1; }
  else if EQ (meta, Qencoded ())
    { new_meta = 3; }
  else
    { new_meta = 2; }

  if (*tty).meta_key != new_meta
    {
      /* C 12894-12897: `#ifndef DOS_NT` is live and has no `#else` text.  */
      /* this causes startup screen to be restored and messes with the mouse */
      reset_sys_modes (tty);

      (*tty).meta_key = new_meta;

      /* C 12901-12903: `#ifndef DOS_NT` is live and has no `#else` text.  */
      init_sys_modes (tty);
    }
  return Qnil ();
});

DEFUN! ("set-quit-char", Fset_quit_char, Sset_quit_char, 1, 1, 0,
       doc: "Specify character used for quitting.\nQUIT must be an ASCII character.\n\nThis function only has an effect on the controlling tty of the Emacs\nprocess.\n\nSee also `current-input-mode'.",
       (quit: Lisp_Object),
       {
  let t: *mut terminal = get_named_terminal (dev_tty);
  let tty: *mut tty_display_info;

  if t.is_null ()
    { return Qnil (); }
  tty = (*t).display_info.tty as *mut tty_display_info;

  if NILP (quit) || !FIXNUMP (quit) || XFIXNUM (quit) < 0 || XFIXNUM (quit) > 0o400
    { error (c"QUIT must be an ASCII character".as_ptr ()); }

  /* C 12928-12933: `#ifndef DOS_NT` is live and has no `#else` text.  */
  /* This causes startup screen to be restored and messes with the
     mouse.  */
  reset_sys_modes (tty);

  /* Don't let this value be out of range.  */
  quit_char = (XFIXNUM (quit) & (if (*tty).meta_key == 0 { 0o177 } else { 0o377 })) as c_int;

  /* C 12937-12939: `#ifndef DOS_NT` is live and has no `#else` text.  */
  init_sys_modes (tty);

  return Qnil ();
});

DEFUN! ("set-input-mode", Fset_input_mode, Sset_input_mode, 3, 4, 0,
       doc: "Set mode of reading keyboard input.\nFirst arg INTERRUPT non-nil means use input interrupts;\n nil means use CBREAK mode.\nSecond arg FLOW non-nil means use ^S/^Q flow control for output to terminal\n (no effect except in CBREAK mode).\nThird arg META t means accept 8-bit input (for a Meta key).\n META nil means ignore the top bit, on the assumption it is parity.\n META `encoded' means accept 8-bit input and interpret Meta after\n   decoding the input characters.\n Otherwise, accept 8-bit input and don't use the top bit for Meta.\nOptional fourth arg QUIT if non-nil specifies character to use for quitting.\nSee also `current-input-mode'.",
       (interrupt: Lisp_Object, flow: Lisp_Object, meta: Lisp_Object, quit: Lisp_Object),
       {
  Fset_input_interrupt_mode (interrupt);
  Fset_output_flow_control (flow, Qnil ());
  Fset_input_meta_mode (meta, Qnil ());
  if !NILP (quit)
    { Fset_quit_char (quit); }
  return Qnil ();
});

DEFUN! ("current-input-mode", Fcurrent_input_mode, Scurrent_input_mode, 0, 0, 0,
       doc: "Return information about the way Emacs currently reads keyboard input.\nThe value is a list of the form (INTERRUPT FLOW META QUIT), where\n  INTERRUPT is non-nil if Emacs is using interrupt-driven input; if\n    nil, Emacs is using CBREAK mode.\n  FLOW is non-nil if Emacs uses ^S/^Q flow control for output to the\n    terminal; this does not apply if Emacs uses interrupt-driven input.\n  META is t if accepting 8-bit unencoded input with 8th bit as Meta flag.\n  META is `encoded' if accepting 8-bit encoded input with 8th bit as\n    Meta flag which has to be interpreted after decoding the input.\n  META is nil if ignoring the top bit of input, on the assumption that\n    it is a parity bit.\n  META is neither t nor nil if accepting 8-bit input and using\n    all 8 bits as the character code.\n  QUIT is the character Emacs currently uses to quit.\nThe elements of this list correspond to the arguments of\n`set-input-mode'.",
       (),
       {
  let sf: *mut frame = XFRAME (selected_frame);

  let interrupt: Lisp_Object = if interrupt_input { Qt () } else { Qnil () };
  let flow: Lisp_Object;
  let meta: Lisp_Object;
  if is_tty_frame (sf)
    {
      flow = if tty_bits! (flow_control, FRAME_TTY! (sf)) { Qt () } else { Qnil () };
      meta = if (*FRAME_TTY! (sf)).meta_key == 2
	       { make_fixnum (0) }
	     else if (*CURTTY! ()).meta_key == 1
	       { Qt () }
	     else if (*CURTTY! ()).meta_key == 3
	       { Qencoded () }
	     else { Qnil () };
    }
  else
    {
      flow = Qnil ();
      meta = Qt ();
    }
  let quit: Lisp_Object = make_fixnum (quit_char as EMACS_INT);

  return list4 (interrupt, flow, meta, quit);
});

DEFUN! ("posn-at-x-y", Fposn_at_x_y, Sposn_at_x_y, 2, 4, 0,
       doc: "Return position information for pixel coordinates X and Y.\nBy default, X and Y are relative to text area of the selected window.\nNote that the text area includes the header-line and the tab-line of\nthe window, if any of them are present.\nOptional third arg FRAME-OR-WINDOW non-nil specifies frame or window.\nIf optional fourth arg WHOLE is non-nil, X is relative to the left\nedge of the window.\n\nThe return value is similar to a mouse click position:\n   (WINDOW AREA-OR-POS (X . Y) TIMESTAMP OBJECT POS (COL . ROW)\n    IMAGE (DX . DY) (WIDTH . HEIGHT))\nThe `posn-' functions access elements of such lists.",
       (x: Lisp_Object, y: Lisp_Object, frame_or_window: Lisp_Object, whole: Lisp_Object),
       {
  /* C reassigns `x`, `y` and `frame_or_window`; a `DEFUN!` parameter cannot be `mut`
     (PORTING.md 5.1), so the port keeps C's names in locals (the `open-dribble-file`
     shape).  */
  let mut x: Lisp_Object = x;
  let mut y: Lisp_Object = y;
  let mut frame_or_window: Lisp_Object = frame_or_window;

  CHECK_FIXNUM (x);
  /* We allow X of -1, for the newline in a R2L line that overflowed
     into the left fringe.  */
  if XFIXNUM (x) != -1
    { CHECK_FIXNAT (x); }
  CHECK_FIXNUM (y);
  if XFIXNUM (y) != -1
    { CHECK_FIXNAT (y); }

  if NILP (frame_or_window)
    { frame_or_window = selected_window; }

  if WINDOWP (frame_or_window)
    {
      let w: *mut window = decode_live_window (frame_or_window);

      XSETINT! (x, (XFIXNUM (x)
		    + WINDOW_LEFT_EDGE_X! (w) as EMACS_INT
		    + (if NILP (whole)
			 { window_box_left_offset (w, TEXT_AREA) as EMACS_INT }
		       else { 0 })));
      /* C promotes `XFIXNUM (y)` and `WINDOW_TOP_EDGE_Y`'s `int` in one sum; the port splits
	 the promotion at the macro boundary (the value is a pixel coordinate).  */
      XSETINT! (y, WINDOW_TO_FRAME_PIXEL_Y! (w, XFIXNUM (y) as c_int) as EMACS_INT);
      frame_or_window = (*w).frame;
    }

  CHECK_LIVE_FRAME! (frame_or_window);

  return make_lispy_position (XFRAME (frame_or_window), x, y, 0);
});

DEFUN! ("posn-at-point", Fposn_at_point, Sposn_at_point, 0, 2, 0,
       doc: "Return position information for buffer position POS in WINDOW.\nPOS defaults to point in WINDOW; WINDOW defaults to the selected window.\n\nIf POS is in invisible text or is hidden by `display' properties,\nthis function may report on buffer positions before or after POS.\n\nReturn nil if POS is not visible in WINDOW.  Otherwise,\nthe return value is similar to that returned by `event-start' for\na mouse click at the upper left corner of the glyph corresponding\nto POS:\n   (WINDOW AREA-OR-POS (X . Y) TIMESTAMP OBJECT POS (COL . ROW)\n    IMAGE (DX . DY) (WIDTH . HEIGHT))\nThe `posn-' functions access elements of such lists.",
       (pos: Lisp_Object, window: Lisp_Object),
       {
  /* C reassigns its `window` parameter; the port keeps C's name in a local.  */
  let mut window: Lisp_Object = window;
  let mut tem: Lisp_Object;

  if NILP (window)
    { window = selected_window; }

  tem = Fpos_visible_in_window_p (pos, window, Qt ());
  if !NILP (tem)
    {
      let x: Lisp_Object = XCAR (tem);
      let mut y: Lisp_Object = XCAR (XCDR (tem));
      let aux_info: Lisp_Object = XCDR (XCDR (tem));
      let y_coord: c_int = XFIXNUM (y) as c_int;

      /* Point invisible due to hscrolling?  X can be -1 when a
	 newline in a R2L line overflows into the left fringe.  */
      if XFIXNUM (x) < -1
	{ return Qnil (); }
      if !NILP (aux_info) && y_coord < 0
	{
	  let rtop: c_int = XFIXNUM (XCAR (aux_info)) as c_int;

	  y = make_fixnum ((y_coord + rtop) as EMACS_INT);
	}
      tem = Fposn_at_x_y (x, y, window, Qnil ());
    }

  return tem;
});

DEFUN! ("posn-point", Fposn_point, Sposn_point, 1, 1, 0,
       doc: "Return the buffer location in POSITION.\nPOSITION should be a list of the form returned by the `event-start'\nand `event-end' functions.\nReturn nil if POSITION does not correspond to any buffer location (e.g.,\na click on a scroll bar).",
       (position: Lisp_Object),
       {
  let mut posn: Lisp_Object = POSN_BUFFER_POSN (position);
  if !NILP (posn)
    { return posn; }
  /* POSITION is a short position list (such as returned by
     `event--posn-at-point') without a POSN_BUFFER_POSN; fall back to
     the location in POSN_POSN.  */
  posn = POSN_POSN (position);
  if CONSP (posn)
    { return XCAR (posn); }
  /* Apparently, POSN can also be `vertical-scroll-bar' (bug#13979).  */
  if FIXNUMP (posn)
    { return posn; }
  return Qnil ();
});

/* Set up a new kboard object with reasonable initial values.
   TYPE is a window system for which this keyboard is used.  */

/* C 13125-13152: `static void init_kboard (KBOARD *kb, Lisp_Object type);`  */
unsafe fn init_kboard (kb: *mut KBOARD, r#type: Lisp_Object)
{
  kset_overriding_terminal_local_map (kb, Qnil ());
  kset_last_command (kb, Qnil ());
  kset_real_last_command (kb, Qnil ());
  kset_keyboard_translate_table (kb, Qnil ());
  kset_last_repeatable_command (kb, Qnil ());
  kset_prefix_arg (kb, Qnil ());
  kset_last_prefix_arg (kb, Qnil ());
  kset_kbd_queue (kb, Qnil ());
  (*kb).kbd_queue_has_data = false;
  kboard_bits! (immediate_echo, kb, false);
  kset_echo_string (kb, Qnil ());
  kset_echo_prompt (kb, Qnil ());
  (*kb).kbd_macro_buffer = std::ptr::null_mut ();
  (*kb).kbd_macro_bufsize = 0;
  kset_defining_kbd_macro (kb, Qnil ());
  kset_last_kbd_macro (kb, Qnil ());
  (*kb).reference_count = 0;
  kset_system_key_alist (kb, Qnil ());
  kset_system_key_syms (kb, Qnil ());
  kset_window_system (kb, r#type);
  kset_input_decode_map (kb, Fmake_sparse_keymap (Qnil ()));
  kset_local_function_key_map (kb, Fmake_sparse_keymap (Qnil ()));
  Fset_keymap_parent (KVAR! (kb, Vlocal_function_key_map), Vfunction_key_map);
  kset_default_minibuffer_frame (kb, Qnil ());
}

/* Allocate and basically initialize keyboard
   object to use with window system TYPE.  */

/* C 13155-13166: `KBOARD *allocate_kboard (Lisp_Object type);` — C's non-`static` definition
   is an exported symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allocate_kboard (r#type: Lisp_Object) -> *mut KBOARD
{
  let mut kb: *mut KBOARD = xmalloc (std::mem::size_of::<KBOARD> ()) as *mut KBOARD;

  init_kboard (kb, r#type);
  (*kb).next_kboard = all_kboards;
  all_kboards = kb;
  return kb;
}
/* lisp.h 803's `bool_bf declared_special : 1` is one bit of `Lisp_Symbol`'s `bits` unit; the
   mirror keeps the storage unit, so the port writes the member through a member-named accessor
   (PORTING.md P-80, one arm per member this file names — the `lread.rs` `set_symbol_bits!`
   shape, which `syms_of_keyboard` needs for C 13978).  */
macro_rules! set_symbol_bits {
  (declared_special, $s:expr, $v:expr) => {
    (*$s).u.s.bits = if $v { (*$s).u.s.bits | SYMBOL_DECLARED_SPECIAL_BIT }
                     else { (*$s).u.s.bits & !SYMBOL_DECLARED_SPECIAL_BIT };
  };
}

/* ============================================================================
   C 13167-13411: `wipe_kboard`, `delete_kboard`, `init_keyboard`, the
   `struct event_head`/`head_table` pair, `init_while_no_input_ignore_events` and
   `is_ignored_event`.  The boundary of this piece is the forward declaration at
   C 13411 (`static void syms_of_keyboard_for_pdumper (void);`), which the port
   drops (PORTING.md 0.1-12: a declaration plus its later definition is `E0428`).
   ============================================================================ */
/*
 * Destroy the contents of a kboard object, but not the object itself.
 * We use this just before deleting it, or if we're going to initialize
 * it a second time.
 */
/* C 13173-13177: `static void wipe_kboard (KBOARD *kb);`  */
unsafe fn wipe_kboard (kb: *mut KBOARD)
{
  xfree ((*kb).kbd_macro_buffer as *mut c_void);
}

/* Free KB and memory referenced from it.  */

/* C 13181-13220: `void delete_kboard (KBOARD *kb);` — C's non-`static` definition is an
   exported symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_kboard (kb: *mut KBOARD)
{
  let mut kbp: *mut *mut KBOARD;
  let mut thread: *mut thread_state;

  /* C's `for (kbp = &all_kboards; *kbp != kb; kbp = &(*kbp)->next_kboard)` with the body's
     abort: the port's `while` keeps the test at the top and the step at the bottom.  */
  kbp = &raw mut all_kboards;
  while *kbp != kb
    {
      if (*kbp).is_null ()
	{ emacs_abort (); }
      kbp = &raw mut (**kbp).next_kboard;
    }
  *kbp = (*kb).next_kboard;

  /* Prevent a dangling reference to KB.  */
  if kb == current_kboard
      && FRAMEP (selected_frame)
      && FRAME_LIVE_P (XFRAME (selected_frame))
    {
      current_kboard = FRAME_KBOARD (XFRAME (selected_frame)) as *mut KBOARD;
      single_kboard = false;
      if current_kboard == kb
	{ emacs_abort (); }
    }

  /* Clean thread specpdls of references to this KBOARD.  */
  thread = all_threads;
  while !thread.is_null ()
    {
      let mut p: *mut specbinding;

      p = (*thread).m_specpdl_ptr;
      while p > (*thread).m_specpdl
	{
	  p = p.offset (-1);

	  if (*p).kind == SPECPDL_LET as u8
	      && (*p).r#let.r#where.kbd == kb as *mut c_void
	    { (*p).r#let.r#where.kbd = std::ptr::null_mut (); }
	}
      thread = (*thread).next_thread;
    }

  wipe_kboard (kb);
  xfree (kb as *mut c_void);
}

/* C 13222-13306: `void init_keyboard (void);` — C's non-`static` definition is an exported
   symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_keyboard ()
{
  /* This is correct before outermost invocation of the editor loop.  */
  command_loop_level = -1;
  quit_char = Ctl! (b'g' as c_int);
  Vunread_command_events = Qnil ();
  timer_idleness_start_time = invalid_timespec ();
  total_keys = 0;
  recent_keys_index = 0;
  kbd_fetch_ptr = (&raw mut kbd_buffer) as *mut buffered_input_event;
  kbd_store_ptr = (&raw mut kbd_buffer) as *mut buffered_input_event;
  track_mouse = Qnil ();
  input_pending = false;
  interrupt_input_blocked = 0;
  pending_signals = false;

  virtual_core_pointer_name = build_string (c"Virtual core pointer".as_ptr ());
  virtual_core_keyboard_name = build_string (c"Virtual core keyboard".as_ptr ());
  Vlast_event_device = Qnil ();

  /* This means that command_loop_1 won't try to select anything the first
     time through.  */
  internal_last_event_frame = Qnil ();
  Vlast_event_frame = internal_last_event_frame;

  current_kboard = initial_kboard;
  /* Re-initialize the keyboard again.  */
  wipe_kboard (current_kboard);
  /* A value of nil for Vwindow_system normally means a tty, but we also use
     it for the initial terminal since there is no window system there.  */
  init_kboard (current_kboard, Qnil ());

  /* Before multi-tty support, these handlers used to be installed
     only if the current session was a tty session.  Now an Emacs
     session may have multiple display types, so we always handle
     SIGINT.  There is special code in handle_interrupt_signal to exit
     Emacs on SIGINT when there are no termcap frames on the
     controlling terminal.  */
  let mut action: sigaction = std::mem::zeroed ();
  emacs_sigaction_init (&raw mut action, Some (deliver_interrupt_signal));
  sigaction (SIGINT, &raw const action, std::ptr::null_mut ());

  if !noninteractive
    {
      /* C 13264-13270: `#ifndef DOS_NT` is live; the `#else` has no text.  */
      /* For systems with SysV TERMIO, C-g is set up for both SIGINT and
	 SIGQUIT and we can't tell which one it will give us.  */
      sigaction (SIGQUIT, &raw const action, std::ptr::null_mut ());
    }
  /* C 13272-13287: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live
     (`USABLE_SIGIO` is 1); `#ifdef USABLE_SIGIO` inside it is live and its `#else`
     (`sigaction (SIGPOLL, ...)`) is carried as `usable_sigpoll`.  */
  if !noninteractive
    {
      #[cfg (not (feature = "usable_sigpoll"))]
      {
      let mut action: sigaction = std::mem::zeroed ();
      emacs_sigaction_init (&raw mut action, Some (deliver_input_available_signal));
      sigaction (SIGIO, &raw const action, std::ptr::null_mut ());
      }
      #[cfg (feature = "usable_sigpoll")]
      {
      let mut action: sigaction = std::mem::zeroed ();
      emacs_sigaction_init (&raw mut action, Some (deliver_input_available_signal));
      sigaction (SIGPOLL, &raw const action, std::ptr::null_mut ());
      }
    }

/* Use interrupt input by default, if it works and noninterrupt input
   has deficiencies.  */

  /* C 13289-13293: `#ifdef INTERRUPT_INPUT` is live (`INTERRUPT_INPUT` is 1); the
     `#else` arm is carried as `no_interrupt_input`.  */
  #[cfg (not (feature = "no_interrupt_input"))]
  {
  interrupt_input = true;
  }
  #[cfg (feature = "no_interrupt_input")]
  {
  interrupt_input = false;
  }

  pthread_sigmask (SIG_SETMASK, &raw const empty_mask, std::ptr::null_mut ());
  dribble = std::ptr::null_mut ();

  if keyboard_init_hook.is_some ()
    { (keyboard_init_hook.unwrap ()) (); }

  /* C 13301-13305: `#ifdef POLL_FOR_INPUT` (dead; carried per P-51).  */
  #[cfg (feature = "poll_for_input")]
  {
  poll_timer = std::ptr::null_mut ();
  poll_suppress_count = 1;
  start_polling ();
  }
}

/* This type's only use is in syms_of_keyboard, to put properties on the
   event header symbols.  */

/* C 13310-13314: `struct event_head { short var; short kind; };` — measured with the frozen
   clang: `sizeof` 4, `alignof` 2, `var` at 0 and `kind` at 2.  The C definition is in this
   file (no header owns it), so P-72's rule ① puts it here.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct event_head
{
  pub var: c_short,
  pub kind: c_short,
}
const _: () = assert! (std::mem::size_of::<event_head> () == 4);
const _: () = assert! (std::mem::align_of::<event_head> () == 2);
const _: () = assert! (std::mem::offset_of! (event_head, var) == 0);
const _: () = assert! (std::mem::offset_of! (event_head, kind) == 2);

/* C's `SYMBOL_INDEX (Qfoo)` is the token-pasting macro `iQfoo`, which the generated globals
   provide as a constant (PORTING.md P-17; the `frame.rs` `frame_parms` precedent).  */
/* C 13316-13335: `static const struct event_head head_table[] = { … };`  */
static head_table: [event_head; 12] =
[
  event_head { var: iQmouse_movement as c_short,      kind: iQmouse_movement as c_short },
  event_head { var: iQscroll_bar_movement as c_short, kind: iQmouse_movement as c_short },

  /* Some of the event heads.  */
  event_head { var: iQswitch_frame as c_short,        kind: iQswitch_frame as c_short },

  event_head { var: iQfocus_in as c_short,            kind: iQfocus_in as c_short },
  event_head { var: iQfocus_out as c_short,           kind: iQfocus_out as c_short },
  event_head { var: iQmove_frame as c_short,          kind: iQmove_frame as c_short },
  event_head { var: iQdelete_frame as c_short,        kind: iQdelete_frame as c_short },
  event_head { var: iQiconify_frame as c_short,       kind: iQiconify_frame as c_short },
  event_head { var: iQmake_frame_visible as c_short,  kind: iQmake_frame_visible as c_short },
  /* `select-window' should be handled just like `switch-frame'
     in read_key_sequence.  */
  event_head { var: iQselect_window as c_short,       kind: iQswitch_frame as c_short },
  /* Touchscreen events should be prefixed by the posn.  */
  event_head { var: iQtouchscreen_begin as c_short,   kind: iQtouchscreen as c_short },
  event_head { var: iQtouchscreen_end as c_short,     kind: iQtouchscreen as c_short },
];

/* C 13337-13358: `static Lisp_Object init_while_no_input_ignore_events (void);`  */
unsafe fn init_while_no_input_ignore_events () -> Lisp_Object
{
  let mut events: Lisp_Object = list! (Qselect_window (), Qhelp_echo (), Qmove_frame (),
				       Qiconify_frame (), Qmake_frame_visible (),
				       Qfocus_in (), Qfocus_out (), Qconfig_changed_event (),
				       Qselection_request (), Qmonitors_changed (),
				       Qtoolkit_theme_changed ());

  /* C 13345-13356: the three `#if`-only arms (dead; carried per P-51).  */
  #[cfg (feature = "have_dbus")]
  { events = Fcons (Qdbus_event (), events); }
  #[cfg (feature = "use_file_notify")]
  { events = Fcons (Qfile_notify (), events); }
  #[cfg (feature = "threads_enabled")]
  { events = Fcons (Qthread_event (), events); }
  events = Fcons (Qsleep_event (), events);

  return events;
}

/* C 13360-13409: `static bool is_ignored_event (union buffered_input_event *event);` —
   this is the last scaffold entry the file was carrying, and it lands here in C's order.  */
unsafe fn is_ignored_event (event: *mut buffered_input_event) -> bool
{
  let mut ignore_event: Lisp_Object = Qnil ();

  /* C's `switch (event->kind)` becomes the port's `match` on the same arms in the same order,
     with `#if`-only arms carried under their `#[cfg]`; C's `break`s (which leave the switch)
     are `break 'switch` on the labelled block, so `case` and `default` are the only tokens
     the port cannot spell (the `macro-body-notes.tsv` row below).  */
  'switch: {
  match (*event).kind as event_kind
    {
    FOCUS_IN_EVENT => {
      ignore_event = Qfocus_in ();
      break 'switch;
    }
    FOCUS_OUT_EVENT => {
      ignore_event = Qfocus_out ();
      break 'switch;
    }
    HELP_EVENT => {
      ignore_event = Qhelp_echo ();
      break 'switch;
    }
    ICONIFY_EVENT => {
      ignore_event = Qiconify_frame ();
      break 'switch;
    }
    DEICONIFY_EVENT => {
      ignore_event = Qmake_frame_visible ();
      break 'switch;
    }
    SELECTION_REQUEST_EVENT => {
      ignore_event = Qselection_request ();
      break 'switch;
    }
    /* C 13377-13380: `#ifdef USE_FILE_NOTIFY` (dead; carried per P-51).  */
    #[cfg (feature = "use_file_notify")]
    FILE_NOTIFY_EVENT => {
      ignore_event = Qfile_notify ();
      break 'switch;
    }
    /* C 13381-13384: `#ifdef HAVE_DBUS` (dead; carried per P-51).  */
    #[cfg (feature = "have_dbus")]
    DBUS_EVENT => {
      ignore_event = Qdbus_event ();
      break 'switch;
    }
    SLEEP_EVENT => {
      ignore_event = Qsleep_event ();
      break 'switch;
    }
    MONITORS_CHANGED_EVENT => {
      ignore_event = Qmonitors_changed ();
      break 'switch;
    }
    TOOLKIT_THEME_CHANGED_EVENT => {
      ignore_event = Qtoolkit_theme_changed ();
      break 'switch;
    }
    _ => {}
    }
  }

  return !NILP (Fmemq (ignore_event, Vwhile_no_input_ignore_events));
}
/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in `src/alloc.c`;
   the port's `include/globals.rs` mirrors the names as declarations, so the unit that owns
   one defines it here (docs/status.md 3.29, the `alloc.rs` shape).  C zero-initialises the
   `globals` object, so the port's initialisers are `lisp_h_Qnil ()`/`false`/`0`, and a
   crate-root definition shadows the `globals.rs` `variables` glob import as P-21 intends.  */
#[unsafe(no_mangle)] pub static mut Vinternal__top_level_message: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut kill_emacs_on_sigint: bool = false;
#[unsafe(no_mangle)] pub static mut last_command_event: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut last_nonmenu_event: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut last_input_event: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vunread_command_events: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vunread_post_input_method_events: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vunread_input_method_events: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut meta_prefix_char: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vthis_command: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vreal_this_command: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vcurrent_minibuffer_command: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vthis_command_keys_shift_translated: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vthis_original_command: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut auto_save_no_message: bool = false;
#[unsafe(no_mangle)] pub static mut Vauto_save_timeout: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vecho_keystrokes: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut echo_keystrokes_help: bool = false;
#[unsafe(no_mangle)] pub static mut Vpolling_period: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vdouble_click_time: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut double_click_fuzz: EMACS_INT = 0;
#[unsafe(no_mangle)] pub static mut num_input_keys: EMACS_INT = 0;
#[unsafe(no_mangle)] pub static mut Vlast_event_frame: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vlast_event_device: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtty_erase_char: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vhelp_char: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vhelp_event_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vhelp_form: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vprefix_help_command: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtop_level: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut cannot_suspend: bool = false;
#[unsafe(no_mangle)] pub static mut menu_prompting: bool = false;
#[unsafe(no_mangle)] pub static mut menu_prompt_more_char: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut extra_keyboard_modifiers: EMACS_INT = 0;
#[unsafe(no_mangle)] pub static mut Vdeactivate_mark: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vpre_command_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vpost_command_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut lucid__menu_grab_keyboard: bool = false;
#[unsafe(no_mangle)] pub static mut Vmenu_bar_final_items: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtab_bar_separator_image_expression: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtool_bar_separator_image_expression: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Voverriding_local_map: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Voverriding_local_map_menu_flag: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vspecial_event_map: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut track_mouse: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vfunction_key_map: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vkey_translation_map: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vdelayed_warnings_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtimer_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vtimer_idle_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vinput_method_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vinput_method_previous_message: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vshow_help_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vdisable_point_adjustment: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vglobal_disable_point_adjustment: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vminibuffer_message_timeout: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vthrow_on_input: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vcommand_error_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Venable_disabled_menus_and_buttons: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vselect_active_regions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vsaved_region_selection: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vselection_inhibit_update_commands: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut Vdebug_on_event: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut attempt_stack_overflow_recovery: bool = false;
#[unsafe(no_mangle)] pub static mut attempt_orderly_shutdown_on_fatal_signal: bool = false;
#[unsafe(no_mangle)] pub static mut Vwhile_no_input_ignore_events: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut translate_upper_case_key_bindings: bool = false;
#[unsafe(no_mangle)] pub static mut input_pending_p_filter_events: bool = false;
#[unsafe(no_mangle)] pub static mut mwheel_coalesce_scroll_events: bool = false;
#[unsafe(no_mangle)] pub static mut Vdisplay_monitors_changed_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut inhibit_record_char: bool = false;
#[unsafe(no_mangle)] pub static mut record_all_keys: bool = false;
#[unsafe(no_mangle)] pub static mut Vpost_select_region_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut disable_inhibit_text_conversion: bool = false;
#[unsafe(no_mangle)] pub static mut Vcurrent_key_remap_sequence: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)] pub static mut multiple_terminals_merge_keyboards: bool = false;
#[unsafe(no_mangle)] pub static mut num_nonmacro_input_events: EMACS_INT = 0;
#[unsafe(no_mangle)] pub static mut auto_save_interval: EMACS_INT = 0;

/* C 13413-14514: `void syms_of_keyboard (void);` — C's non-`static` definition is an exported
   symbol (PORTING.md 0.2).  Every `DEFVAR_*`/`DEFSYM`/`defsubr` invocation keeps C's order,
   and the `DEFVAR_*!` calls take C's docstring positionally (which is why this round adds the
   `syms_of_keyboard` / `doc` row to `docs/ports/macro-body-notes.tsv` — the `syms_of_data`
   precedent).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_keyboard ()
{
  pending_funcalls = Qnil ();
  staticpro (&raw const pending_funcalls);

  Vlispy_mouse_stem = build_string (c"mouse".as_ptr ());
  staticpro (&raw const Vlispy_mouse_stem);

  regular_top_level_message = build_string (c"Back to top level".as_ptr ());
  staticpro (&raw const regular_top_level_message);
  /* C 13424-13428: `#ifdef HAVE_STACK_OVERFLOW_HANDLING` is live (`1` on this target).  */
  recover_top_level_message
    = build_string (c"Re-entering top level after C stack overflow".as_ptr ());
  staticpro (&raw const recover_top_level_message);
  DEFVAR_LISP! ("internal--top-level-message", Vinternal__top_level_message,
	      " Message displayed by `normal-top-level'.  ");
  Vinternal__top_level_message = regular_top_level_message;

  DEFVAR_BOOL! ("kill-emacs-on-sigint", kill_emacs_on_sigint,
	      " If non-nil, a SIGINT event causes Emacs to exit.\nIf nil, a SIGINT event causes a `quit` signal instead.\nThis is effective only in `noninteractive' sessions.  ");
  kill_emacs_on_sigint = true;

  /* Tool-bars.  */
  DEFSYM! (QCimage, ":image");
  DEFSYM! (Qhelp_echo, "help-echo");
  DEFSYM! (Qhelp_echo_inhibit_substitution, "help-echo-inhibit-substitution");
  DEFSYM! (QCrtl, ":rtl");
  DEFSYM! (QCwrap, ":wrap");

  staticpro (&raw const item_properties);
  item_properties = Qnil ();

  staticpro (&raw const tab_bar_item_properties);
  tab_bar_item_properties = Qnil ();
  staticpro (&raw const tab_bar_items_vector);
  tab_bar_items_vector = Qnil ();

  staticpro (&raw const tool_bar_item_properties);
  tool_bar_item_properties = Qnil ();
  staticpro (&raw const tool_bar_items_vector);
  tool_bar_items_vector = Qnil ();

  DEFSYM! (Qtimer_event_handler, "timer-event-handler");

  /* Non-nil disable property on a command means do not execute it;
     call disabled-command-function's value instead.  */
  DEFSYM! (Qdisabled, "disabled");

  DEFSYM! (Qundefined, "undefined");

  /* Hooks to run before and after each command.  */
  DEFSYM! (Qpre_command_hook, "pre-command-hook");
  DEFSYM! (Qpost_command_hook, "post-command-hook");
  DEFSYM! (Qlong_line_optimizations_in_command_hooks, "long-line-optimizations-in-command-hooks");

  /* Hook run after the region is selected.  */
  DEFSYM! (Qpost_select_region_hook, "post-select-region-hook");

  DEFSYM! (Qundo_auto__add_boundary, "undo-auto--add-boundary");
  DEFSYM! (Qundo_auto__undoably_changed_buffers, "undo-auto--undoably-changed-buffers");

  DEFSYM! (Qdelayed_warnings_hook, "delayed-warnings-hook");
  DEFSYM! (Qfunction_key, "function-key");

  /* The values of Qevent_kind properties.  */
  DEFSYM! (Qmouse_click, "mouse-click");

  DEFSYM! (Qdrag_n_drop, "drag-n-drop");
  DEFSYM! (Qsave_session, "save-session");
  DEFSYM! (Qconfig_changed_event, "config-changed-event");

  /* Menu and tool bar item parts.  */
  DEFSYM! (Qmenu_enable, "menu-enable");

  /* C 13493-13496: `#ifdef HAVE_NTGUI` (dead; carried per P-51).  */
  #[cfg (feature = "have_ntgui")]
  {
  DEFSYM! (Qlanguage_change, "language-change");
  DEFSYM! (Qend_session, "end-session");
  }

  /* C 13498-13500: `#ifdef HAVE_DBUS` (dead; carried per P-51).  */
  #[cfg (feature = "have_dbus")]
  {
  DEFSYM! (Qdbus_event, "dbus-event");
  }

  /* C 13502-13504: `#ifdef THREADS_ENABLED` (dead; carried per P-51).  */
  #[cfg (feature = "threads_enabled")]
  {
  DEFSYM! (Qthread_event, "thread-event");
  }

  /* C 13506-13509: `#ifdef HAVE_XWIDGETS` (dead; carried per P-51).  */
  #[cfg (feature = "have_xwidgets")]
  {
  DEFSYM! (Qxwidget_event, "xwidget-event");
  DEFSYM! (Qxwidget_display_event, "xwidget-display-event");
  }

  /* C 13511-13513: `#ifdef USE_FILE_NOTIFY` (dead; carried per P-51).  */
  #[cfg (feature = "use_file_notify")]
  {
  DEFSYM! (Qfile_notify, "file-notify");
  }

  DEFSYM! (Qtouch_end, "touch-end");
  DEFSYM! (Qsleep_event, "sleep-event");
  DEFSYM! (Qmonitors_changed, "monitors-changed");
  DEFSYM! (Qtoolkit_theme_changed, "toolkit-theme-changed");

  /* Menu and tool bar item parts.  */
  DEFSYM! (QCenable, ":enable");
  DEFSYM! (QCvisible, ":visible");
  DEFSYM! (QChelp, ":help");
  DEFSYM! (QCfilter, ":filter");
  DEFSYM! (QCbutton, ":button");
  DEFSYM! (QCkeys, ":keys");
  DEFSYM! (QCkey_sequence, ":key-sequence");

  /* Non-nil disable property on a command means
     do not execute it; call disabled-command-function's value instead.  */
  DEFSYM! (QCtoggle, ":toggle");
  DEFSYM! (QCradio, ":radio");
  DEFSYM! (QClabel, ":label");
  DEFSYM! (QCvert_only, ":vert-only");

  /* Symbols to use for parts of windows.  */
  DEFSYM! (Qvertical_line, "vertical-line");
  DEFSYM! (Qright_divider, "right-divider");
  DEFSYM! (Qbottom_divider, "bottom-divider");

  DEFSYM! (Qmouse_fixup_help_message, "mouse-fixup-help-message");

  DEFSYM! (Qabove_handle, "above-handle");
  DEFSYM! (Qhandle, "handle");
  DEFSYM! (Qbelow_handle, "below-handle");
  DEFSYM! (Qup, "up");
  DEFSYM! (Qdown, "down");
  DEFSYM! (Qtop, "top");
  DEFSYM! (Qbottom, "bottom");
  DEFSYM! (Qend_scroll, "end-scroll");
  DEFSYM! (Qratio, "ratio");
  DEFSYM! (Qbefore_handle, "before-handle");
  DEFSYM! (Qhorizontal_handle, "horizontal-handle");
  DEFSYM! (Qafter_handle, "after-handle");
  DEFSYM! (Qleft, "left");
  DEFSYM! (Qright, "right");
  DEFSYM! (Qleftmost, "leftmost");
  DEFSYM! (Qrightmost, "rightmost");

  /* Properties of event headers.  */
  DEFSYM! (Qevent_kind, "event-kind");
  DEFSYM! (Qevent_symbol_elements, "event-symbol-elements");

  /* An event header symbol HEAD may have a property named
     Qevent_symbol_element_mask, which is of the form (BASE MODIFIERS);
     BASE is the base, unmodified version of HEAD, and MODIFIERS is the
     mask of modifiers applied to it.  If present, this is used to help
     speed up parse_modifiers.  */
  DEFSYM! (Qevent_symbol_element_mask, "event-symbol-element-mask");

  /* An unmodified event header BASE may have a property named
     Qmodifier_cache, which is an alist mapping modifier masks onto
     modified versions of BASE.  If present, this helps speed up
     apply_modifiers.  */
  DEFSYM! (Qmodifier_cache, "modifier-cache");

  DEFSYM! (Qactivate_menubar_hook, "activate-menubar-hook");

  DEFSYM! (Qpolling_period, "polling-period");

  DEFSYM! (Qgui_set_selection, "gui-set-selection");
  DEFSYM! (Qxterm__set_selection, "xterm--set-selection");
  DEFSYM! (Qtty_select_active_regions, "tty-select-active-regions");

  /* The primary selection.  */
  DEFSYM! (QPRIMARY, "PRIMARY");

  DEFSYM! (Qhandle_switch_frame, "handle-switch-frame");
  DEFSYM! (Qhandle_select_window, "handle-select-window");

  DEFSYM! (Qinput_method_exit_on_first_char, "input-method-exit-on-first-char");
  DEFSYM! (Qinput_method_use_echo_area, "input-method-use-echo-area");

  DEFSYM! (Qhelp_form_show, "help-form-show");

  DEFSYM! (Qhelp_key_binding, "help-key-binding");

  DEFSYM! (Qhelp__append_keystrokes_help, "help--append-keystrokes-help");

  DEFSYM! (Qecho_keystrokes, "echo-keystrokes");

  Fset (Qinput_method_exit_on_first_char (), Qnil ());
  Fset (Qinput_method_use_echo_area (), Qnil ());

  /* Symbols for dragging internal borders.  */
  DEFSYM! (Qdrag_internal_border, "drag-internal-border");
  DEFSYM! (Qleft_edge, "left-edge");
  DEFSYM! (Qtop_left_corner, "top-left-corner");
  DEFSYM! (Qtop_edge, "top-edge");
  DEFSYM! (Qtop_right_corner, "top-right-corner");
  DEFSYM! (Qright_edge, "right-edge");
  DEFSYM! (Qbottom_right_corner, "bottom-right-corner");
  DEFSYM! (Qbottom_edge, "bottom-edge");
  DEFSYM! (Qbottom_left_corner, "bottom-left-corner");

  /* Symbols to head events.  */
  DEFSYM! (Qmouse_movement, "mouse-movement");
  DEFSYM! (Qscroll_bar_movement, "scroll-bar-movement");
  DEFSYM! (Qswitch_frame, "switch-frame");
  DEFSYM! (Qfocus_in, "focus-in");
  DEFSYM! (Qfocus_out, "focus-out");
  DEFSYM! (Qmove_frame, "move-frame");
  DEFSYM! (Qdelete_frame, "delete-frame");
  DEFSYM! (Qiconify_frame, "iconify-frame");
  DEFSYM! (Qmake_frame_visible, "make-frame-visible");
  DEFSYM! (Qselect_window, "select-window");
  DEFSYM! (Qselection_request, "selection-request");
  DEFSYM! (Qwindow_edges, "window-edges");
  {
    let mut i: c_int;

    i = 0;
    while i < countof! (head_table) as c_int
      {
	let p: *const event_head = &raw const head_table[i as usize];
	let var: Lisp_Object = builtin_lisp_symbol ((*p).var as usize);
	let kind: Lisp_Object = builtin_lisp_symbol ((*p).kind as usize);
	Fput (var, Qevent_kind (), kind);
	Fput (var, Qevent_symbol_elements (), list1 (var));
	i += 1;
      }
  }
  DEFSYM! (Qno_record, "no-record");
  DEFSYM! (Qencoded, "encoded");

  DEFSYM! (Qpreedit_text, "preedit-text");

  button_down_location = make_nil_vector (5);
  staticpro (&raw const button_down_location);
  staticpro (&raw const frame_relative_event_pos);
  mouse_syms = make_nil_vector (5);
  staticpro (&raw const mouse_syms);
  wheel_syms = make_nil_vector (countof! (lispy_wheel_names) as isize);
  staticpro (&raw const wheel_syms);

  {
    let mut i: c_int;
    let len: c_int = countof! (modifier_names) as c_int;

    modifier_symbols = make_nil_vector (len as isize);
    i = 0;
    while i < len
      {
      if !modifier_names[i as usize].is_null ()
	{ ASET (modifier_symbols, i as isize, intern_c_string (modifier_names[i as usize])); }
      i += 1;
      }
    staticpro (&raw const modifier_symbols);
  }

  recent_keys = make_nil_vector (lossage_limit as isize);
  staticpro (&raw const recent_keys);

  this_command_keys = make_nil_vector (40);
  staticpro (&raw const this_command_keys);

  raw_keybuf = make_nil_vector (30);
  staticpro (&raw const raw_keybuf);

  DEFSYM! (Qcommand_execute, "command-execute");
  DEFSYM! (Qinternal_echo_keystrokes_prefix, "internal-echo-keystrokes-prefix");

  accent_key_syms = Qnil ();
  staticpro (&raw const accent_key_syms);

  func_key_syms = Qnil ();
  staticpro (&raw const func_key_syms);

  drag_n_drop_syms = Qnil ();
  staticpro (&raw const drag_n_drop_syms);

  pinch_syms = Qnil ();
  staticpro (&raw const pinch_syms);

  unread_switch_frame = Qnil ();
  staticpro (&raw const unread_switch_frame);

  internal_last_event_frame = Qnil ();
  staticpro (&raw const internal_last_event_frame);

  read_key_sequence_cmd = Qnil ();
  staticpro (&raw const read_key_sequence_cmd);
  read_key_sequence_remapped = Qnil ();
  staticpro (&raw const read_key_sequence_remapped);

  menu_bar_one_keymap_changed_items = Qnil ();
  staticpro (&raw const menu_bar_one_keymap_changed_items);

  menu_bar_items_vector = Qnil ();
  staticpro (&raw const menu_bar_items_vector);

  help_form_saved_window_configs = Qnil ();
  staticpro (&raw const help_form_saved_window_configs);

  /* C 13709-13712: `#ifdef POLL_FOR_INPUT` (dead; carried per P-51).  */
  #[cfg (feature = "poll_for_input")]
  {
  poll_timer_time = Qnil ();
  staticpro (&raw const poll_timer_time);
  }

  virtual_core_pointer_name = Qnil ();
  staticpro (&raw const virtual_core_pointer_name);

  virtual_core_keyboard_name = Qnil ();
  staticpro (&raw const virtual_core_keyboard_name);

  menu_bar_touch_id = Qnil ();
  staticpro (&raw const menu_bar_touch_id);

  defsubr (&raw mut Scurrent_idle_time);
  defsubr (&raw mut Sevent_symbol_parse_modifiers);
  defsubr (&raw mut Sevent_convert_list);
  defsubr (&raw mut Sinternal_handle_focus_in);
  defsubr (&raw mut Sread_key_sequence);
  defsubr (&raw mut Sread_key_sequence_vector);
  defsubr (&raw mut Srecursive_edit);
  defsubr (&raw mut Sinternal_track_mouse);
  defsubr (&raw mut Sinput_pending_p);
  defsubr (&raw mut Sinsert_special_event);
  defsubr (&raw mut Slossage_size);
  defsubr (&raw mut Srecent_keys);
  defsubr (&raw mut Sthis_command_keys);
  defsubr (&raw mut Sthis_command_keys_vector);
  defsubr (&raw mut Sthis_single_command_keys);
  defsubr (&raw mut Sthis_single_command_raw_keys);
  defsubr (&raw mut Sset__this_command_keys);
  defsubr (&raw mut Sclear_this_command_keys);
  defsubr (&raw mut Ssuspend_emacs);
  defsubr (&raw mut Sabort_recursive_edit);
  defsubr (&raw mut Sexit_recursive_edit);
  defsubr (&raw mut Srecursion_depth);
  defsubr (&raw mut Scommand_error_default_function);
  defsubr (&raw mut Stop_level);
  defsubr (&raw mut Sdiscard_input);
  defsubr (&raw mut Sopen_dribble_file);
  defsubr (&raw mut Sset_input_interrupt_mode);
  defsubr (&raw mut Sset_output_flow_control);
  defsubr (&raw mut Sset_input_meta_mode);
  defsubr (&raw mut Sset_quit_char);
  defsubr (&raw mut Sset_input_mode);
  defsubr (&raw mut Scurrent_input_mode);
  defsubr (&raw mut Sposn_at_point);
  defsubr (&raw mut Sposn_at_x_y);
  defsubr (&raw mut Sposn_point);

  defsubr (&raw mut Sread_char);
  defsubr (&raw mut Sread_char_exclusive);
  defsubr (&raw mut Sread_event);

  DEFVAR_LISP! ("last-command-event", last_command_event,
	      " Last input event of a key sequence that called a command.\nSee Info node `(elisp)Command Loop Info'.");

  DEFVAR_LISP! ("last-nonmenu-event", last_nonmenu_event,
	      " Last input event in a command, except for mouse menu events.\nMouse menus give back keys that don't look like mouse events;\nthis variable holds the actual mouse event that led to the menu,\nso that you can determine whether the command was run by mouse or not.  ");

  DEFVAR_LISP! ("last-input-event", last_input_event,
	      " Last input event.  ");

  DEFVAR_LISP! ("unread-command-events", Vunread_command_events,
	      " List of events to be read as the command input.\nThese events are processed first, before actual keyboard input.\nEvents read from this list are not normally added to `this-command-keys',\nas they will already have been added once as they were read for the first time.\nAn element of the form (t . EVENT) forces EVENT to be added to that list.\nAn element of the form (no-record . EVENT) means process EVENT, but do not\nrecord it in the keyboard macros, recent-keys, and the dribble file.  ");
  Vunread_command_events = Qnil ();

  DEFVAR_LISP! ("unread-post-input-method-events", Vunread_post_input_method_events,
	      " List of events to be processed as input by input methods.\nThese events are processed before `unread-command-events'\nand actual keyboard input, but are not given to `input-method-function'.  ");
  Vunread_post_input_method_events = Qnil ();

  DEFVAR_LISP! ("unread-input-method-events", Vunread_input_method_events,
	      " List of events to be processed as input by input methods.\nThese events are processed after `unread-command-events', but\nbefore actual keyboard input.\nIf there's an active input method, the events are given to\n`input-method-function'.  ");
  Vunread_input_method_events = Qnil ();

  DEFVAR_LISP! ("meta-prefix-char", meta_prefix_char,
	      " Meta-prefix character code.\nMeta-foo as command input turns into this character followed by foo.  ");
  XSETINT! (meta_prefix_char, 0o33);

/* C's `DEFVAR_KBOARD (lname, vname, doc)` stores `offsetof (KBOARD, vname##_)` (lisp.h
   3525-3530), so each call below passes the kboard field's offset — the `^_` field name the
   `KVAR!` macro above already uses.  (C's `Vlast_command` is the *field* `Vlast_command_`;
   the bare name would resolve to `include/globals.rs`'s unrelated declaration.)  */
  DEFVAR_KBOARD! ("last-command", std::mem::offset_of! (kboard, Vlast_command_),
	      " The last command executed.\nNormally a symbol with a function definition, but can be whatever was found\nin the keymap, or whatever the variable `this-command' was set to by that\ncommand.\n\nThe value `mode-exit' is special; it means that the previous command\nread an event that told it to exit, and it did so and unread that event.\nIn other words, the present command is the event that made the previous\ncommand exit.\n\nThe value `kill-region' is special; it means that the previous command\nwas a kill command.\n\n`last-command' has a separate binding for each terminal device.\nSee Info node `(elisp)Multiple Terminals'.  ");

  DEFVAR_KBOARD! ("real-last-command", std::mem::offset_of! (kboard, Vreal_last_command_),
	      " Same as `last-command', but never altered by Lisp code.\nTaken from the previous value of `real-this-command'.  ");

  DEFVAR_KBOARD! ("last-repeatable-command", std::mem::offset_of! (kboard, Vlast_repeatable_command_),
	      " Last command that may be repeated.\nThe last command executed that was not bound to an input event.\nThis is the command `repeat' will try to repeat.\nTaken from a previous value of `real-this-command'.  ");

  DEFVAR_LISP! ("this-command", Vthis_command,
	      " The command now being executed.\nThe command can set this variable; whatever is put here\nwill be in `last-command' during the following command.  ");
  Vthis_command = Qnil ();

  DEFVAR_LISP! ("real-this-command", Vreal_this_command,
	      " This is like `this-command', except that commands should never modify it.  ");
  Vreal_this_command = Qnil ();

  DEFSYM! (Qcurrent_minibuffer_command, "current-minibuffer-command");
  DEFVAR_LISP! ("current-minibuffer-command", Vcurrent_minibuffer_command,
	      " This is like `this-command', but bound recursively.\nCode running from (for instance) a minibuffer hook can check this variable\nto see what command invoked the current minibuffer.  ");
  Vcurrent_minibuffer_command = Qnil ();

  DEFVAR_LISP! ("this-command-keys-shift-translated", Vthis_command_keys_shift_translated,
	      " Non-nil if the key sequence activating this command was shift-translated.\nShift-translation occurs when there is no binding for the key sequence\nas entered, but a binding was found by changing an upper-case letter\nto lower-case, or a shifted function key to an unshifted one.  ");
  Vthis_command_keys_shift_translated = Qnil ();

  DEFVAR_LISP! ("this-original-command", Vthis_original_command,
	      " The command bound to the current key sequence before remapping.\nIt equals `this-command' if the original command was not remapped through\nany of the active keymaps.  Otherwise, the value of `this-command' is the\nresult of looking up the original command in the active keymaps.  ");
  Vthis_original_command = Qnil ();

  DEFVAR_INT! ("auto-save-interval", auto_save_interval,
	      " Number of input events between auto-saves.\nZero means disable autosaving due to number of characters typed.  ");
  auto_save_interval = 300;

  DEFVAR_BOOL! ("auto-save-no-message", auto_save_no_message,
	      " Non-nil means do not print any message when auto-saving. ");
  auto_save_no_message = false;

  DEFVAR_LISP! ("auto-save-timeout", Vauto_save_timeout,
	      " Number of seconds idle time before auto-save.\nZero or nil means disable auto-saving due to idleness.\nAfter auto-saving due to this many seconds of idle time,\nEmacs also does a garbage collection if that seems to be warranted.  ");
  XSETFASTINT! (Vauto_save_timeout, 30);

  DEFVAR_LISP! ("echo-keystrokes", Vecho_keystrokes,
	      " Nonzero means echo unfinished commands after this many seconds of pause.\nThe value may be integer or floating point.\nIf the value is zero, don't echo at all.  ");
  Vecho_keystrokes = make_fixnum (1);

  DEFVAR_BOOL! ("echo-keystrokes-help", echo_keystrokes_help,
	      " Whether to append help text to echoed commands.\nWhen non-nil, a reference to `C-h' is printed after echoed\nkeystrokes.  ");
  echo_keystrokes_help = true;

  DEFVAR_LISP! ("polling-period", Vpolling_period,
	      " Interval between polling for input during Lisp execution.\nThe reason for polling is to make C-g work to stop a running program.\nPolling is needed only when using X windows and SIGIO does not work.\nPolling is automatically disabled in all other cases.  ");
  Vpolling_period = make_float (2.0);

  DEFVAR_LISP! ("double-click-time", Vdouble_click_time,
	      " Maximum time between mouse clicks to make a double-click.\nMeasured in milliseconds.  The value nil means disable double-click\nrecognition; t means double-clicks have no time limit and are detected\nby position only.\n\nIn Lisp, you might want to use `mouse-double-click-time' instead of\nreading the value of this variable directly.  ");
  Vdouble_click_time = make_fixnum (500);

  DEFVAR_INT! ("double-click-fuzz", double_click_fuzz,
	      " Maximum mouse movement between clicks to make a double-click.\nOn window-system frames, value is the number of pixels the mouse may have\nmoved horizontally or vertically between two clicks to make a double-click.\nOn non window-system frames, value is interpreted in units of 1/8 characters\ninstead of pixels.\n\nThis variable is also the threshold for motion of the mouse\nto count as a drag.  ");
  double_click_fuzz = 3;

  DEFVAR_INT! ("num-input-keys", num_input_keys,
	      " Number of complete key sequences read as input so far.\nThis includes key sequences read from keyboard macros.\nThe number is effectively the number of interactive command invocations.  ");
  num_input_keys = 0;

  DEFVAR_INT! ("num-nonmacro-input-events", num_nonmacro_input_events,
	      " Number of input events read from the keyboard so far.\nThis does not include events generated by keyboard macros.  ");
  num_nonmacro_input_events = 0;

  DEFVAR_LISP! ("last-event-frame", Vlast_event_frame,
	      " The frame in which the most recently read event occurred.\nIf the last event came from a keyboard macro, this is set to `macro'.  ");
  Vlast_event_frame = Qnil ();

  DEFVAR_LISP! ("last-event-device", Vlast_event_device,
	      " The name of the input device of the most recently read event.\nWhen the input extension is being used on X, this is the name of the X\nInput Extension device from which the last event was generated as a\nstring.  Otherwise, this is \"Virtual core keyboard\" for keyboard input\nevents, and \"Virtual core pointer\" for other events.\n\nIt is nil if the last event did not come from an input device (i.e. it\ncame from `unread-command-events' instead).  ");
  Vlast_event_device = Qnil ();

  /* This variable is set up in sysdep.c.  */
  DEFVAR_LISP! ("tty-erase-char", Vtty_erase_char,
	      " The ERASE character as set by the user with stty.  ");

  DEFVAR_LISP! ("help-char", Vhelp_char,
	      " Character to recognize as meaning Help.\nWhen it is read, do `(eval help-form)', and display result if it's a string.\nIf the value of `help-form' is nil, this char can be read normally.  ");
  XSETINT! (Vhelp_char, Ctl! (b'H' as EMACS_INT));

  DEFVAR_LISP! ("help-event-list", Vhelp_event_list,
	      " List of input events to recognize as meaning Help.\nThese work just like the value of `help-char' (see that).  ");
  Vhelp_event_list = Qnil ();

  DEFVAR_LISP! ("help-form", Vhelp_form,
	      " Form to execute when character `help-char' is read.\nIf the form returns a string, that string is displayed.\nIf `help-form' is nil, the help char is not recognized.  ");
  Vhelp_form = Qnil ();

  DEFVAR_LISP! ("prefix-help-command", Vprefix_help_command,
	      " Command to run when `help-char' character follows a prefix key.\nThis command is used only when there is no actual binding\nfor that character after that prefix key.  ");
  Vprefix_help_command = Qnil ();

  DEFVAR_LISP! ("top-level", Vtop_level,
	      " Form to evaluate when Emacs starts up.\nUseful to set before you dump a modified Emacs.  ");
  Vtop_level = Qnil ();
  set_symbol_bits! (declared_special, XSYMBOL (Qtop_level ()), false);

  DEFVAR_KBOARD! ("keyboard-translate-table", std::mem::offset_of! (kboard, Vkeyboard_translate_table_),
	      " Translate table for local keyboard input, or nil.\nIf non-nil, the value should be a char-table.  Each character read\nfrom the keyboard is looked up in this char-table.  If the value found\nthere is non-nil, then it is used instead of the actual input character.\n\nThe value can also be a string or vector, but this is considered obsolete.\nIf it is a string or vector of length N, character codes N and up are left\nuntranslated.  In a vector, an element which is nil means \"no translation\".\n\nThis is applied to the characters supplied to input methods, not their\noutput.  See also `translation-table-for-input'.\n\nThis variable has a separate binding for each terminal.\nSee Info node `(elisp)Multiple Terminals'.  ");

  DEFVAR_BOOL! ("cannot-suspend", cannot_suspend,
	      " Non-nil means to always spawn a subshell instead of suspending.\n\\(Even if the operating system has support for stopping a process.)  ");
  cannot_suspend = false;

  DEFVAR_BOOL! ("menu-prompting", menu_prompting,
	      " Non-nil means prompt with menus when appropriate.\nThis is done when reading from a keymap that has a prompt string,\nfor elements that have prompt strings.\nThe menu is displayed on the screen\nif X menus were enabled at configuration\ntime and the previous event was a mouse click prefix key.\nOtherwise, menu prompting uses the echo area.  ");
  menu_prompting = true;

  DEFVAR_LISP! ("menu-prompt-more-char", menu_prompt_more_char,
	      " Character to see next line of menu prompt.\nType this character while in a menu prompt to rotate around the lines of it.  ");
  XSETINT! (menu_prompt_more_char, b' ' as EMACS_INT);

  DEFVAR_INT! ("extra-keyboard-modifiers", extra_keyboard_modifiers,
	      " A mask of additional modifier keys to use with every keyboard character.\nEmacs applies the modifiers of the character stored here to each keyboard\ncharacter it reads.  For example, after evaluating the expression\n    (setq extra-keyboard-modifiers ?\\\\C-x)\nall input characters will have the control modifier applied to them.\n\nNote that the character ?\\\\C-@, equivalent to the integer zero, does\nnot count as a control character; rather, it counts as a character\nwith no modifiers; thus, setting `extra-keyboard-modifiers' to zero\ncancels any modification.  ");
  extra_keyboard_modifiers = 0;

  DEFSYM! (Qdeactivate_mark, "deactivate-mark");
  DEFVAR_LISP! ("deactivate-mark", Vdeactivate_mark,
	      " Whether to deactivate the mark after an editing command.\nThe command loop sets this to nil before each command,\nand tests the value when the command returns.\nIf an editing command sets this non-nil, deactivate the mark after\nthe command returns.\n\nBuffer modifications store t in this variable.\n\nBy default, deactivating the mark will save the contents of the region\naccording to `select-active-regions', unless this is set to the symbol\n`dont-save'.  ");
  Vdeactivate_mark = Qnil ();
  Fmake_variable_buffer_local (Qdeactivate_mark ());

  DEFVAR_LISP! ("pre-command-hook", Vpre_command_hook,
	      " Normal hook run before each command is executed.\n\nIf an unhandled error happens in running this hook, the function in\nwhich the error occurred is unconditionally removed, since otherwise\nthe error might happen repeatedly and make Emacs nonfunctional.\n\nNote that, when `long-line-optimizations-p' is non-nil in the buffer,\nthese functions are called as if they were in a `with-restriction' form,\nwith a `long-line-optimizations-in-command-hooks' label and with the\nbuffer narrowed to a portion around point whose size is specified by\n`long-line-optimizations-region-size'.\n\nSee also `post-command-hook'.  ");
  Vpre_command_hook = Qnil ();

  DEFVAR_LISP! ("post-command-hook", Vpost_command_hook,
	      " Normal hook run after each command is executed.\n\nIf an unhandled error happens in running this hook, the function in\nwhich the error occurred is unconditionally removed, since otherwise\nthe error might happen repeatedly and make Emacs nonfunctional.\n\nIt is a bad idea to use this hook for expensive processing.  If\nunavoidable, wrap your code in `(while-no-input (redisplay) CODE)' to\navoid making Emacs unresponsive while the user types.\n\nNote that, when `long-line-optimizations-p' is non-nil in the buffer,\nthese functions are called as if they were in a `with-restriction' form,\nwith a `long-line-optimizations-in-command-hooks' label and with the\nbuffer narrowed to a portion around point whose size is specified by\n`long-line-optimizations-region-size'.\n\nSee also `pre-command-hook'.  ");
  Vpost_command_hook = Qnil ();

  /* C 14081-14084: `#if 0` with a `DEFVAR_LISP ("echo-area-clear-hook", ...)` inside it — dead
     in C itself, so it is recorded here and not transcribed (P-08; the file's other `#if 0`
     blocks).  */
  DEFSYM! (Qecho_area_clear_hook, "echo-area-clear-hook");
  DEFSYM! (Qtouchscreen_begin, "touchscreen-begin");
  DEFSYM! (Qtouchscreen_end, "touchscreen-end");
  DEFSYM! (Qtouchscreen_update, "touchscreen-update");
  DEFSYM! (Qpinch, "pinch");
  DEFSYM! (Qdisplay_monitors_changed_functions, "display-monitors-changed-functions");

  DEFSYM! (Qcoding, "coding");
  DEFSYM! (Qtouchscreen, "touchscreen");
  /* C 14095-14097: `#ifdef HAVE_TEXT_CONVERSION` (dead; carried per P-51).  */
  #[cfg (feature = "have_text_conversion")]
  {
  DEFSYM! (Qtext_conversion, "text-conversion");
  }

  Fset (Qecho_area_clear_hook (), Qnil ());

  /* C 14101-14108: `#ifdef USE_LUCID` (dead; carried per P-51).  */
  #[cfg (feature = "use_lucid")]
  {
  DEFVAR_BOOL! ("lucid--menu-grab-keyboard", lucid__menu_grab_keyboard,
	      " If non-nil, grab keyboard during menu operations.\nThis is only relevant when using the Lucid X toolkit.  It can be\nconvenient to disable this for debugging purposes.  ");
  lucid__menu_grab_keyboard = true;
  }

  DEFVAR_LISP! ("menu-bar-final-items", Vmenu_bar_final_items,
	      " List of menu bar items to move to the end of the menu bar.\nThe elements of the list are event types that may have menu bar\nbindings.  The order of this list controls the order of the items.  ");
  Vmenu_bar_final_items = Qnil ();

  DEFVAR_LISP! ("tab-bar-separator-image-expression", Vtab_bar_separator_image_expression,
	      " Expression evaluating to the image spec for a tab-bar separator.\nThis is used internally by graphical displays that do not render\ntab-bar separators natively.  Otherwise it is unused (e.g. on GTK).  ");
  Vtab_bar_separator_image_expression = Qnil ();

  DEFVAR_LISP! ("tool-bar-separator-image-expression", Vtool_bar_separator_image_expression,
	      " Expression evaluating to the image spec for a tool-bar separator.\nThis is used internally by graphical displays that do not render\ntool-bar separators natively.  Otherwise it is unused (e.g. on GTK).  ");
  Vtool_bar_separator_image_expression = Qnil ();

  DEFVAR_KBOARD! ("overriding-terminal-local-map", std::mem::offset_of! (kboard, Voverriding_terminal_local_map_),
	      " Per-terminal keymap that takes precedence over all other keymaps.\nThis variable is intended to let commands such as `universal-argument'\nset up a different keymap for reading the next command.\n\n`overriding-terminal-local-map' has a separate binding for each\nterminal device.  See Info node `(elisp)Multiple Terminals'.  ");

  DEFVAR_LISP! ("overriding-local-map", Voverriding_local_map,
	      " Keymap that replaces (overrides) local keymaps.\nIf this variable is non-nil, Emacs looks up key bindings in this\nkeymap INSTEAD OF `keymap' text properties, `local-map' and `keymap'\noverlay properties, minor mode maps, and the buffer's local map.\n\nHence, the only active keymaps would be `overriding-terminal-local-map',\nthis keymap, and `global-keymap', in order of precedence.  ");
  Voverriding_local_map = Qnil ();

  DEFVAR_LISP! ("overriding-local-map-menu-flag", Voverriding_local_map_menu_flag,
	      " Non-nil means `overriding-local-map' applies to the menu bar.\nOtherwise, the menu bar continues to reflect the buffer's local map\nand the minor mode maps regardless of `overriding-local-map'.  ");
  Voverriding_local_map_menu_flag = Qnil ();

  DEFVAR_LISP! ("special-event-map", Vspecial_event_map,
	      " Keymap defining bindings for special events to execute at low level.  ");
  Vspecial_event_map = list1 (Qkeymap ());

  DEFVAR_LISP! ("track-mouse", track_mouse,
	      " Non-nil means generate motion events for mouse motion.\nThe special values `dragging' and `dropping' assert that the mouse\ncursor retains its appearance during mouse motion.  Any non-nil value\nbut `dropping' or `drag-source' asserts that motion events always\nrelate to the frame where the mouse movement started.  The value\n`dropping' asserts that motion events relate to the frame where the\nmouse cursor is seen when generating the event.  If there's no such\nframe, such motion events relate to the frame where the mouse movement\nstarted.  The value `drag-source' is like `dropping', but the\n`posn-window' will be nil in mouse position lists inside mouse\nmovement events if there is no frame directly visible underneath the\nmouse pointer.  ");
  DEFVAR_KBOARD! ("system-key-alist", std::mem::offset_of! (kboard, Vsystem_key_alist_),
	      " Alist of system-specific X windows key symbols.\nEach element should have the form (N . SYMBOL) where N is the\nnumeric keysym code (sans the \\\"system-specific\\\" bit 1<<28)\nand SYMBOL is its name.\n\n`system-key-alist' has a separate binding for each terminal device.\nSee Info node `(elisp)Multiple Terminals'.  ");

  DEFVAR_KBOARD! ("local-function-key-map", std::mem::offset_of! (kboard, Vlocal_function_key_map_),
	      " Keymap that translates key sequences to key sequences during input.\nThis is used mainly for mapping key sequences into some preferred\nkey events (symbols).\n\nThe `read-key-sequence' function replaces any subsequence bound by\n`local-function-key-map' with its binding.  More precisely, when the\nactive keymaps have no binding for the current key sequence but\n`local-function-key-map' binds a suffix of the sequence to a vector or\nstring, `read-key-sequence' replaces the matching suffix with its\nbinding, and continues with the new sequence.\n\nIf the binding is a function, it is called with one argument (the prompt)\nand its return value (a key sequence) is used.\n\nThe events that come from bindings in `local-function-key-map' are not\nthemselves looked up in `local-function-key-map'.\n\nFor example, suppose `local-function-key-map' binds `ESC O P' to [f1].\nTyping `ESC O P' to `read-key-sequence' would return [f1].  Typing\n`C-x ESC O P' would return [?\\\\C-x f1].  If [f1] were a prefix key,\ntyping `ESC O P x' would return [f1 x].\n\n`local-function-key-map' has a separate binding for each terminal\ndevice.  See Info node `(elisp)Multiple Terminals'.  If you need to\ndefine a binding on all terminals, change `function-key-map'\ninstead.  Initially, `local-function-key-map' is an empty keymap that\nhas `function-key-map' as its parent on all terminal devices.  ");

  DEFVAR_KBOARD! ("input-decode-map", std::mem::offset_of! (kboard, Vinput_decode_map_),
	      " Keymap that decodes input escape sequences.\nThis is used mainly for mapping ASCII function key sequences into\nreal Emacs function key events (symbols).\n\nThe `read-key-sequence' function replaces any subsequence bound by\n`input-decode-map' with its binding.  Contrary to `function-key-map',\nthis map applies its rebinding regardless of the presence of an ordinary\nbinding.  So it is more like `key-translation-map' except that it applies\nbefore `function-key-map' rather than after.\n\nIf the binding is a function, it is called with one argument (the prompt)\nand its return value (a key sequence) is used.\n\nThe events that come from bindings in `input-decode-map' are not\nthemselves looked up in `input-decode-map'.  ");

  DEFVAR_LISP! ("function-key-map", Vfunction_key_map,
	      " The parent keymap of all `local-function-key-map' instances.\nFunction key definitions that apply to all terminal devices should go\nhere.  If a mapping is defined in both the current\n`local-function-key-map' binding and this variable, then the local\ndefinition will take precedence.  ");
  Vfunction_key_map = Fmake_sparse_keymap (Qnil ());

  DEFVAR_LISP! ("key-translation-map", Vkey_translation_map,
	      " Keymap of key translations that can override keymaps.\nThis keymap works like `input-decode-map', but comes after `function-key-map'.\nAnother difference is that it is global rather than terminal-local.  ");
  Vkey_translation_map = Fmake_sparse_keymap (Qnil ());

  DEFVAR_LISP! ("delayed-warnings-list", Vdelayed_warnings_list,
	      " List of warnings to be displayed after this command.\nEach element must be a list (TYPE MESSAGE [LEVEL [BUFFER-NAME]]),\nas per the args of `display-warning' (which see).\nIf this variable is non-nil, `delayed-warnings-hook' will be run\nimmediately after running `post-command-hook'.  ");
  Vdelayed_warnings_list = Qnil ();

  DEFVAR_LISP! ("timer-list", Vtimer_list,
	      " List of active absolute time timers in order of increasing time.  ");
  Vtimer_list = Qnil ();

  DEFVAR_LISP! ("timer-idle-list", Vtimer_idle_list,
	      " List of active idle-time timers in order of increasing time.  ");
  Vtimer_idle_list = Qnil ();

  DEFVAR_LISP! ("input-method-function", Vinput_method_function,
	      " If non-nil, the function that implements the current input method.\nIt's called with one argument, which must be a single-byte\ncharacter that was just read.  Any single-byte character is\nacceptable, except the DEL character, codepoint 127 decimal, 177 octal.\nTypically this function uses `read-event' to read additional events.\nWhen it does so, it should first bind `input-method-function' to nil\nso it will not be called recursively.\n\nThe function should return a list of zero or more events\nto be used as input.  If it wants to put back some events\nto be reconsidered, separately, by the input method,\nit can add them to the beginning of `unread-command-events'.\n\nThe input method function can find in `input-method-previous-message'\nthe previous echo area message.\n\nThe input method function should refer to the variables\n`input-method-use-echo-area' and `input-method-exit-on-first-char'\nfor guidance on what to do.  ");
  Vinput_method_function = Qlist ();

  DEFVAR_LISP! ("input-method-previous-message", Vinput_method_previous_message,
	      " When `input-method-function' is called, hold the previous echo area message.\nThis variable exists because `read-event' clears the echo area\nbefore running the input method.  It is nil if there was no message.  ");
  Vinput_method_previous_message = Qnil ();

  DEFVAR_LISP! ("show-help-function", Vshow_help_function,
	      " If non-nil, the function that implements the display of help.\nIt's called with one argument, the help string to display.  ");
  Vshow_help_function = Qnil ();

  DEFVAR_LISP! ("disable-point-adjustment", Vdisable_point_adjustment,
	      " If non-nil, suppress point adjustment after executing a command.\n\nAfter a command is executed, if point moved into a region that has\nspecial properties (e.g. composition, display), Emacs adjusts point to\nthe boundary of the region.  But when a command leaves this variable at\na non-nil value (e.g., with a setq), this point adjustment is suppressed.\n\nThis variable is set to nil before reading a command, and is checked\njust after executing the command.  ");
  Vdisable_point_adjustment = Qnil ();

  DEFVAR_LISP! ("global-disable-point-adjustment", Vglobal_disable_point_adjustment,
	      " If non-nil, always suppress point adjustments.\n\nThe default value is nil, in which case point adjustments are\nsuppressed only after special commands that leave\n`disable-point-adjustment' (which see) at a non-nil value.  ");
  Vglobal_disable_point_adjustment = Qnil ();

  DEFVAR_LISP! ("minibuffer-message-timeout", Vminibuffer_message_timeout,
	      " How long to display an echo-area message when the minibuffer is active.\nIf the value is a number, it should be specified in seconds.\nIf the value is not a number, such messages never time out.  ");
  Vminibuffer_message_timeout = make_fixnum (2);

  DEFVAR_LISP! ("throw-on-input", Vthrow_on_input,
	      " If non-nil, any keyboard input throws to this symbol.\nThe value of that variable is passed to `quit-flag' and later causes a\npeculiar kind of quitting.  ");
  Vthrow_on_input = Qnil ();

  DEFVAR_LISP! ("command-error-function", Vcommand_error_function,
	      " Function to output error messages.\nCalled with three arguments:\n- the error data, a list of the form (SIGNALED-CONDITION . SIGNAL-DATA)\n  such as what `condition-case' would bind its variable to,\n- the context (a string which normally goes at the start of the message),\n- the Lisp function within which the error was signaled.\n\nFor instance, to make error messages stand out more in the echo area,\nyou could say something like:\n\n    (setq command-error-function\n          (lambda (data _ _)\n            (message \"%s\" (propertize (error-message-string data)\n                                      \\\\='face \\\\='error))))\n\nAlso see `set-message-function' (which controls how non-error messages\nare displayed).  ");
  Vcommand_error_function = Qcommand_error_default_function ();

  DEFVAR_LISP! ("enable-disabled-menus-and-buttons", Venable_disabled_menus_and_buttons,
	      " If non-nil, don't ignore events produced by disabled menu items and tool-bar.\n\nHelp functions bind this to allow help on disabled menu items\nand tool-bar buttons.  ");
  Venable_disabled_menus_and_buttons = Qnil ();

  DEFVAR_LISP! ("select-active-regions", Vselect_active_regions,
	      " If non-nil, any active region automatically sets the primary selection.\nThis variable only has an effect when Transient Mark mode is enabled.\n\nIf the value is `only', only temporarily active regions (usually made\nby mouse-dragging or shift-selection) set the window system's primary\nselection.\n\nIf this variable causes the region to be set as the primary selection,\n`post-select-region-hook' is then run afterwards.  ");
  Vselect_active_regions = Qt ();

  DEFVAR_LISP! ("saved-region-selection", Vsaved_region_selection,
	      " Contents of active region prior to buffer modification.\nIf `select-active-regions' is non-nil, Emacs sets this to the\ntext in the region before modifying the buffer.  The next call to\nthe function `deactivate-mark' uses this to set the window selection.  ");
  Vsaved_region_selection = Qnil ();

  DEFVAR_LISP! ("selection-inhibit-update-commands", Vselection_inhibit_update_commands,
	      " List of commands which should not update the selection.\nNormally, if `select-active-regions' is non-nil and the mark remains\nactive after a command (i.e. the mark was not deactivated), the Emacs\ncommand loop sets the selection to the text in the region.  However,\nif the command is in this list, the selection is not updated.  ");
  Vselection_inhibit_update_commands
    = list2 (Qhandle_switch_frame (), Qhandle_select_window ());

  DEFVAR_LISP! ("debug-on-event", Vdebug_on_event,
	      " Enter debugger on this event.\nWhen Emacs receives the special event specified by this variable,\nit will try to break into the debugger as soon as possible instead\nof processing the event normally through `special-event-map'.\n\nCurrently, the only supported values for this\nvariable are `sigusr1' and `sigusr2'.  ");
  Vdebug_on_event = Qsigusr2 ();

  DEFVAR_BOOL! ("attempt-stack-overflow-recovery", attempt_stack_overflow_recovery,
	      " If non-nil, attempt to recover from C stack overflows.\nThis recovery is potentially unsafe and may lead to deadlocks or data\ncorruption, but it usually works and may preserve modified buffers\nthat would otherwise be lost.  If nil, treat stack overflow like any\nother kind of crash or fatal error.  ");
  attempt_stack_overflow_recovery = true;

  DEFVAR_BOOL! ("attempt-orderly-shutdown-on-fatal-signal", attempt_orderly_shutdown_on_fatal_signal,
	      " If non-nil, attempt orderly shutdown on fatal signals.\nBy default this variable is non-nil, and Emacs attempts to perform\nan orderly shutdown when it catches a fatal signal (e.g., a crash).\nThe orderly shutdown includes an attempt to auto-save your unsaved edits\nand other useful cleanups.  These cleanups are potentially unsafe and may\nlead to deadlocks or data corruption, but it usually works and may\npreserve data in modified buffers that would otherwise be lost.\nIf nil, Emacs crashes immediately in response to fatal signals.  ");
  attempt_orderly_shutdown_on_fatal_signal = true;

  DEFVAR_LISP! ("while-no-input-ignore-events", Vwhile_no_input_ignore_events,
	      " Ignored events from `while-no-input'.\nEvents in this list do not count as pending input while running\n`while-no-input' and do not cause any idle timers to get reset when they\noccur.  ");
  Vwhile_no_input_ignore_events = init_while_no_input_ignore_events ();

  DEFVAR_BOOL! ("translate-upper-case-key-bindings", translate_upper_case_key_bindings,
	      " If non-nil, interpret upper case keys as lower case (when applicable).\nEmacs allows binding both upper and lower case key sequences to\ncommands.  However, if there is a lower case key sequence bound to a\ncommand, and the user enters an upper case key sequence that is not\nbound to a command, Emacs will use the lower case binding.  Setting\nthis variable to nil inhibits this behavior.  ");
  translate_upper_case_key_bindings = true;

  DEFVAR_BOOL! ("input-pending-p-filter-events", input_pending_p_filter_events,
	      " If non-nil, `input-pending-p' ignores some input events.\nIf this variable is non-nil (the default), `input-pending-p' and\nother similar functions ignore input events in `while-no-input-ignore-events'.\nThis flag may eventually be removed once this behavior is deemed safe.  ");
  input_pending_p_filter_events = true;

  DEFVAR_BOOL! ("mwheel-coalesce-scroll-events", mwheel_coalesce_scroll_events,
	      " Non-nil means send a wheel event only for scrolling at least one screen line.\nOtherwise, a wheel event will be sent every time the mouse wheel is\nmoved.  ");
  mwheel_coalesce_scroll_events = true;

  DEFVAR_LISP! ("display-monitors-changed-functions", Vdisplay_monitors_changed_functions,
	      " Abnormal hook run when the monitor configuration changes.\nThis can happen if a monitor is rotated, moved, plugged in or removed\nfrom a multi-monitor setup, if the primary monitor changes, or if the\nresolution of a monitor changes.  The hook should accept a single\nargument, which is the terminal on which the monitor configuration\nchanged.  ");
  Vdisplay_monitors_changed_functions = Qnil ();

  DEFVAR_BOOL! ("inhibit--record-char", inhibit_record_char,
	      " If non-nil, don't record input events.\nThis inhibits recording input events for the purposes of keyboard\nmacros, dribble file, and `recent-keys'.\nInternal use only.  ");
  inhibit_record_char = false;

  DEFVAR_BOOL! ("record-all-keys", record_all_keys,
	      " Non-nil means record all keys you type.\nWhen nil, the default, characters typed as part of passwords are\nnot recorded.  The non-nil value countermands `inhibit--record-char',\nwhich see.  ");
  record_all_keys = false;

  DEFVAR_LISP! ("post-select-region-hook", Vpost_select_region_hook,
	      " Abnormal hook run after the region is selected.\nThis usually happens as a result of `select-active-regions'.  The hook\nis called with one argument, the string that was selected.  ");
  Vpost_select_region_hook = Qnil ();

  DEFVAR_BOOL! ("disable-inhibit-text-conversion", disable_inhibit_text_conversion,
	      " Don't disable text conversion inside `read-key-sequence'.\nIf non-nil, text conversion will continue to happen after a prefix\nkey has been read inside `read-key-sequence'.  ");
  disable_inhibit_text_conversion = false;

  DEFVAR_LISP! ("current-key-remap-sequence", Vcurrent_key_remap_sequence,
	      " The key sequence currently being remap, or nil.\nBound to a vector containing the sub-sequence matching a binding\nwithin `input-decode-map' or `local-function-key-map' when its bound\nfunction is called to remap that sequence.  ");
  Vcurrent_key_remap_sequence = Qnil ();
  DEFSYM! (Qcurrent_key_remap_sequence, "current-key-remap-sequence");

  DEFVAR_BOOL! ("multiple-terminals-merge-keyboards", multiple_terminals_merge_keyboards,
	      " If non-nil, treat different terminals' keyboards as less isolated.\nIf this option is non-nil, Emacs will not enter single-keyboard mode\nwhen entering a recursive edit.  It will still enter single-keyboard\nmode in certain other cases where doing so is necessary for the\noperation to work at all.  ");
  multiple_terminals_merge_keyboards = false;

  pdumper_do_now_and_after_load (Some (syms_of_keyboard_for_pdumper));

  DEFSYM! (Qactivate_mark_hook, "activate-mark-hook");
  /* C 14503-14506: `#ifdef HAVE_NS` (dead; carried per P-51).  */
  #[cfg (feature = "have_ns")]
  {
  DEFSYM! (Qns_put_working_text, "ns-put-working-text");
  DEFSYM! (Qns_unput_working_text, "ns-unput-working-text");
  }
  DEFSYM! (Qinternal_timer_start_idle, "internal-timer-start-idle");
  DEFSYM! (Qconcat, "concat");
  DEFSYM! (Qsuspend_hook, "suspend-hook");
  DEFSYM! (Qsuspend_resume_hook, "suspend-resume-hook");
  DEFSYM! (Qcommand_error_default_function, "command-error-default-function");
  DEFSYM! (Qsigusr2, "sigusr2");
  DEFSYM! (Qascii_character, "ascii-character");
}

/* C 14516-14545: `static void syms_of_keyboard_for_pdumper (void);` — C hands its address to
   `pdumper_do_now_and_after_load` (C 14500), so the port defines it `unsafe extern "C" fn`
   (PORTING.md 1.4).  The forward declaration at C 13411 is dropped (PORTING.md 0.1-12).  */
unsafe extern "C" fn syms_of_keyboard_for_pdumper ()
{
  /* Make sure input state is pristine when restoring from a dump.
     init_keyboard() also resets some of these, but the duplication
     doesn't hurt and makes sure that allocate_kboard and subsequent
     early init functions see the environment they expect.  */

  PDUMPER_RESET_LV! (pending_funcalls, Qnil ());
  PDUMPER_RESET_LV! (unread_switch_frame, Qnil ());
  PDUMPER_RESET_LV! (internal_last_event_frame, Qnil ());
  PDUMPER_RESET_LV! (last_command_event, Qnil ());
  PDUMPER_RESET_LV! (last_nonmenu_event, Qnil ());
  PDUMPER_RESET_LV! (last_input_event, Qnil ());
  PDUMPER_RESET_LV! (Vunread_command_events, Qnil ());
  PDUMPER_RESET_LV! (Vunread_post_input_method_events, Qnil ());
  PDUMPER_RESET_LV! (Vunread_input_method_events, Qnil ());
  PDUMPER_RESET_LV! (Vthis_command, Qnil ());
  PDUMPER_RESET_LV! (Vreal_this_command, Qnil ());
  PDUMPER_RESET_LV! (Vthis_command_keys_shift_translated, Qnil ());
  PDUMPER_RESET_LV! (Vthis_original_command, Qnil ());
  PDUMPER_RESET! (num_input_keys, 0);
  PDUMPER_RESET! (num_nonmacro_input_events, 0);
  PDUMPER_RESET_LV! (Vlast_event_frame, Qnil ());
  PDUMPER_RESET_LV! (Vdelayed_warnings_list, Qnil ());

  /* Create the initial keyboard.  Qt means 'unset'.  */
  eassert! (initial_kboard.is_null ());
  initial_kboard = allocate_kboard (Qt ());
}

/* C 14547-14629: `void keys_of_keyboard (void);` — C's non-`static` definition is an exported
   symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn keys_of_keyboard ()
{
  initial_define_lispy_key (Vspecial_event_map, c"delete-frame".as_ptr (),
			    c"handle-delete-frame".as_ptr ());
  /* C 14552-14561: `#ifdef HAVE_NTGUI` and `#ifdef HAVE_NS` (dead; carried per P-51).  */
  #[cfg (feature = "have_ntgui")]
  {
  initial_define_lispy_key (Vspecial_event_map, c"end-session".as_ptr (),
			    c"kill-emacs".as_ptr ());
  }
  #[cfg (feature = "have_ns")]
  {
  initial_define_lispy_key (Vspecial_event_map, c"ns-put-working-text".as_ptr (),
			    c"ns-put-working-text".as_ptr ());
  initial_define_lispy_key (Vspecial_event_map, c"ns-unput-working-text".as_ptr (),
			    c"ns-unput-working-text".as_ptr ());
  }
  /* Here we used to use `ignore-event' which would simple set prefix-arg to
     current-prefix-arg, as is done in `handle-switch-frame'.
     But `handle-switch-frame is not run from the special-map.
     Commands from that map are run in a special way that automatically
     preserves the prefix-arg.  Restoring the prefix arg here is not just
     redundant but harmful:
     - C-u C-x v =
     - current-prefix-arg is set to non-nil, prefix-arg is set to nil.
     - after the first prompt, the exit-minibuffer-hook is run which may
       iconify a frame and thus push a `iconify-frame' event.
     - after running exit-minibuffer-hook, current-prefix-arg is
       restored to the non-nil value it had before the prompt.
     - we enter the second prompt.
       current-prefix-arg is non-nil, prefix-arg is nil.
     - before running the first real event, we run the special iconify-frame
       event, but we pass the `special' arg to command-execute so
       current-prefix-arg and prefix-arg are left untouched.
     - here we foolishly copy the non-nil current-prefix-arg to prefix-arg.
     - the next key event will have a spuriously non-nil current-prefix-arg.  */
  initial_define_lispy_key (Vspecial_event_map, c"iconify-frame".as_ptr (),
			    c"ignore".as_ptr ());
  initial_define_lispy_key (Vspecial_event_map, c"make-frame-visible".as_ptr (),
			    c"ignore".as_ptr ());
  /* Handling it at such a low-level causes read_key_sequence to get
   * confused because it doesn't realize that the current_buffer was
   * changed by read_char.
   *
   * initial_define_lispy_key (Vspecial_event_map, "select-window",
   * 			    "handle-select-window"); */
  initial_define_lispy_key (Vspecial_event_map, c"save-session".as_ptr (),
			    c"handle-save-session".as_ptr ());

  /* C 14594-14612: `#ifdef HAVE_DBUS`, `#ifdef THREADS_ENABLED` and
     `#ifdef USE_FILE_NOTIFY` (dead; carried per P-51).  */
  #[cfg (feature = "have_dbus")]
  {
  /* Define a special event which is raised for dbus callback
     functions.  */
  initial_define_lispy_key (Vspecial_event_map, c"dbus-event".as_ptr (),
			    c"dbus-handle-event".as_ptr ());
  }
  #[cfg (feature = "threads_enabled")]
  {
  /* Define a special event which is raised for threads.  */
  initial_define_lispy_key (Vspecial_event_map, c"thread-event".as_ptr (),
			    c"thread-handle-event".as_ptr ());
  }
  #[cfg (feature = "use_file_notify")]
  {
  /* Define a special event which is raised for file notifications.  */
  initial_define_lispy_key (Vspecial_event_map, c"file-notify".as_ptr (),
                            c"file-notify-handle-event".as_ptr ());
  }

  initial_define_lispy_key (Vspecial_event_map, c"config-changed-event".as_ptr (),
			    c"ignore".as_ptr ());
  /* C 14616-14619: `#if defined (WINDOWSNT)` (dead; carried per P-51).  */
  #[cfg (feature = "windowsnt")]
  {
  initial_define_lispy_key (Vspecial_event_map, c"language-change".as_ptr (),
			    c"ignore".as_ptr ());
  }
  initial_define_lispy_key (Vspecial_event_map, c"focus-in".as_ptr (),
			    c"handle-focus-in".as_ptr ());
  initial_define_lispy_key (Vspecial_event_map, c"focus-out".as_ptr (),
			    c"handle-focus-out".as_ptr ());
  initial_define_lispy_key (Vspecial_event_map, c"move-frame".as_ptr (),
			    c"handle-move-frame".as_ptr ());
  initial_define_lispy_key (Vspecial_event_map, c"sleep-event".as_ptr (),
			    c"ignore".as_ptr ());
}

/* C 14632-14682: `void mark_kboards (void);` — C's non-`static` definition is an exported
   symbol (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_kboards ()
{
  let mut kb: *mut KBOARD = all_kboards;
  while !kb.is_null ()
    {
      if !(*kb).kbd_macro_buffer.is_null ()
	{
	  mark_objects ((*kb).kbd_macro_buffer,
			(*kb).kbd_macro_ptr.offset_from ((*kb).kbd_macro_buffer));
	}
      mark_object (KVAR! (kb, Voverriding_terminal_local_map));
      mark_object (KVAR! (kb, Vlast_command));
      mark_object (KVAR! (kb, Vreal_last_command));
      mark_object (KVAR! (kb, Vkeyboard_translate_table));
      mark_object (KVAR! (kb, Vlast_repeatable_command));
      mark_object (KVAR! (kb, Vprefix_arg));
      mark_object (KVAR! (kb, Vlast_prefix_arg));
      mark_object (KVAR! (kb, kbd_queue));
      mark_object (KVAR! (kb, defining_kbd_macro));
      mark_object (KVAR! (kb, Vlast_kbd_macro));
      mark_object (KVAR! (kb, Vsystem_key_alist));
      mark_object (KVAR! (kb, system_key_syms));
      mark_object (KVAR! (kb, Vwindow_system));
      mark_object (KVAR! (kb, Vinput_decode_map));
      mark_object (KVAR! (kb, Vlocal_function_key_map));
      mark_object (KVAR! (kb, Vdefault_minibuffer_frame));
      mark_object (KVAR! (kb, echo_string));
      mark_object (KVAR! (kb, echo_prompt));
      kb = (*kb).next_kboard;
    }

  /* C 14660-14668: `#ifndef HAVE_HAIKU` is live and its dead arm is the *shorter* condition
     (`event->kind != SELECTION_REQUEST_EVENT` alone), whose tokens are a subset of the live
     text, so nothing needs carrying.  */
  let mut event: *mut buffered_input_event = kbd_fetch_ptr;
  while event != kbd_store_ptr
    {
      if (*event).kind != SELECTION_REQUEST_EVENT as u16
	  && (*event).kind != SELECTION_CLEAR_EVENT as u16
	{
	  mark_object ((*event).ie.x);
	  mark_object ((*event).ie.y);
	  mark_object ((*event).ie.frame_or_window);
	  mark_object ((*event).ie.arg);
	  /* C 14675-14679: `#ifdef HAVE_XWIDGETS` (dead; carried per P-51).  */
	  #[cfg (feature = "have_xwidgets")]
	  { mark_object ((*event).ie.device); }
	}
      event = next_kbd_event (event);
    }
}
