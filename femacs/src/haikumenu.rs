// femacs: port of src/haikumenu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct field, file-scope object, local), the same control flow and the same error model.
// The non-`static` C definitions carry `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2);
// the C `static` helpers stay plain `unsafe fn` except the four whose address C hands to another
// function (`haiku_menu_show_help`, `haiku_process_pending_signals_for_menu` and its two
// catch-all handlers `_1`/`_2`), which are `unsafe extern "C" fn` (PORTING.md 1.4/P-12: one
// definition per linkage unit, which is what C's `static` gives too).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `haikumenu.o`
// sits in `HAIKU_OBJ` (`emacs/src/Makefile.in`), which `configure.ac` 3097 clears and 3114 fills
// with the Haiku platform objects under `test "${HAVE_BE_APP}" = "yes"` - the same test that
// `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = `
// (empty), so `docs/baseline/frozen-build-sources.txt` lists no haikumenu.o and `extract-abi.py`'s
// checklist for the file is empty (P-86 ②).  The **file gate** is therefore `feature = "have_haiku"`,
// on every item *and on C 21-29's own include graph* (P-86 ⑧: an `include!` outside the gate breaks
// the frozen build at once, which is exactly why `build-tu src/haikumenu.c` reads a 336-byte empty
// object - see the ledger's Evidence ①).
//
// The file's **own** conditions: `grep -n '#if' src/haikumenu.c` is empty - the whole body is one
// live region, so there is no sub-gate to carry, and `python3 tools/check-cond-eval.py
// src/haikumenu.c` reads `0 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
//
// **Acquisition note - this port is `text-only` (P-86 ③⑵).**  There is no Haiku SDK and no Haiku
// toolchain on this machine (the measurement is `docs/ports/haikuterm.md` Evidence ②, same
// machine), so there is **no arm flip on a Haiku target** and nothing here is compiled or linked
// against a Haiku system.  The `<InterfaceKit.h>`/`<SupportDefs.h>` surface this file's headers
// pull in is already mirrored by `include/haiku_support.rs` (landed before this unit); **every
// Haiku width, constant and enum value this port spells** (`HAIKU_INFO_ALERT`, `HAIKU_IDEA_ALERT`,
// `MENU_BAR_RESIZE`, the `haiku_alert_type` the mirror's `BAlert_new` takes, `int32`) **is
// `UNVERIFIED`** - declared by the mirror from the reference header, never measured against a
// Haiku system (P-88 ①/②).
//
// `docs/ports/haikumenu.md` carries the C line-range table, the acceptance readings, every
// deviation and the items the integrator owes.  No behaviour of this file was executed anywhere:
// there is no Haiku SDK to compile the arm against and no Haiku target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");                // C 19: `#include <config.h>` (ungated)

/* C 21-29: the file's own include list.  No translation unit the frozen configuration builds
   reaches any one of them, so the whole graph sits under the file gate (see the header above).
   C's lines stand as these comments where the mirror *is* what C would have included.  */
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                  // C 21: `#include "lisp.h"` (reaches <string.h> at lisp.h 30: memcpy, memset, strcmp)
#[cfg (feature = "have_haiku")]
include!("include/frame.rs");                 // C 22: `#include "frame.h"`
#[cfg (feature = "have_haiku")]
include!("include/keyboard.rs");              // C 23: `#include "keyboard.h"` (reaches coding.h 23: ENCODE_UTF_8)
#[cfg (feature = "have_haiku")]
include!("include/menu.rs");                  // C 24: `#include "menu.h"`
#[cfg (feature = "have_haiku")]
include!("include/buffer.rs");                // C 25: `#include "buffer.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");            // C 26: `#include "blockinput.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");             // C 28: `#include "haikuterm.h"`
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");         // C 29: `#include "haiku_support.h"`

/* ------------------------------------------------------------------------------------------
   C 21-29's includes reach C's own entry points, but several of their mirrors do not carry
   the names this file's live text uses (P-72 rule 4: a mirror reaches what it needs through its
   own `include!`s).  Each keeps C's prototype and the header line it was read from (the
   `src/haikufns.rs` 293-323 / `src/haikuselect.rs` 167-188 shape); their move into the mirrors
   is owed (`docs/ports/haikumenu.md` ## Owed).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: src/frame.h 1475-1476: `extern WINDOW_SYSTEM_RETURN struct frame *
  /// decode_window_system_frame (Lisp_Object);` - C 773.
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;


  /// C: src/lisp.h 3880: `extern void process_pending_signals (void);` - C 282, C 369.
  pub fn process_pending_signals ();
  /// C: src/lisp.h 4786: `extern Lisp_Object quit (void);` - C 287, C 294, C 495.
  pub fn quit () -> Lisp_Object;
  /// C: src/lisp.h 4811: `extern Lisp_Object internal_catch_all (Lisp_Object (*) (void *), void *,
  /// Lisp_Object (*) (enum nonlocal_exit, Lisp_Object));` - C 374-375.
  pub fn internal_catch_all (func: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,
                             arg: *mut c_void,
                             handler: Option<unsafe extern "C" fn (nonlocal_exit, Lisp_Object)
                                              -> Lisp_Object>) -> Lisp_Object;
  /// C: src/lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));` - C 313.
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: src/lisp.h 4998: `extern void record_unwind_save_match_data (void);` - C 620.
  pub fn record_unwind_save_match_data ();
  /// C: src/lisp.h 5087: `extern Lisp_Object menu_bar_items (Lisp_Object);` - C 635.
  pub fn menu_bar_items (menu_bar_items: Lisp_Object) -> Lisp_Object;
  /// C: src/lisp.h 5097: `extern void safe_run_hooks (Lisp_Object);` - C 630, C 634.
  pub fn safe_run_hooks (hook: Lisp_Object);

  /// C: src/keyboard.h 473: `extern bool menu_separator_name_p (const char *);` - C 161.
  pub fn menu_separator_name_p (name: *const c_char) -> bool;


  /// C: src/keyboard.h 521: `extern struct timespec timer_check (void);` - C 352.
  pub fn timer_check () -> timespec;
  
  
}

/* C 251 and C 425: `BAlert_go (…, block_input, unblock_input, …)` and `BMenu_run (…,
   block_input, unblock_input, …)` hand C's INLINE `block_input` (blockinput.h 54:
   `INLINE void block_input (void) { interrupt_input_blocked++; }`) to a `void (*) (void)`
   parameter, which makes C emit an out-of-line copy of the INLINE for this translation unit.
   `include/blockinput.rs` 24 spells `block_input` as a Rust-ABI `unsafe fn` (it is an INLINE,
   PORTING.md 0.1-6), which a C function pointer cannot take, so this unit writes the C-ABI copy
   C's own address-taking would emit (the `src/haikuselect.rs` 189-202 reading).  The module
   keeps the name out of the crate root, where the mirror's `block_input` already is - a second
   item of the same name there would be an `E0428` (P-26/P-39).  `docs/ports/haikumenu.md` Owed
   names the mirror round that owes the C-ABI form (`include/blockinput.rs`'s own ledger owns it,
   the same item `docs/ports/haikuselect.md` Owed 1 records).  */
#[cfg (feature = "have_haiku")]
mod block_input_abi {
  /// C: blockinput.h 54-56: `INLINE void block_input (void) { interrupt_input_blocked++; }`
  pub unsafe extern "C" fn block_input () { unsafe { super::interrupt_input_blocked += 1; } }
}

/* `frame.h`'s accessors C *writes* in this file (C 510-512) and `haikuterm.h` 374's menubar
   accessor, which C both reads and writes (C 509, C 516, C 519...).  Each mirror accessor is a
   read-only function (or, for `FRAME_EXTERNAL_MENU_BAR`, a flag reader), and a Rust function
   cannot be an assignment target, so C's two-argument write arm is carried under the same name
   (the `src/haikufns.rs` 350-410 / `src/pgtkfns.rs` precedent).  The `!` in an invocation is the
   port's spelling of C's assignment (a recorded deviation).  */
/// C: frame.h 1086/1087: `#define FRAME_MENU_BAR_LINES(f) (f)->menu_bar_lines` /
/// `#define FRAME_MENU_BAR_HEIGHT(f) (f)->menu_bar_height` - written at C 510/511.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_lines = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_height = $v };
}
/// C: frame.h 1078-1083: `#ifdef HAVE_EXT_MENU_BAR` / `#define FRAME_EXTERNAL_MENU_BAR(f)
/// (f)->external_menu_bar` / `#else` / `#define FRAME_EXTERNAL_MENU_BAR(f) false` - written at
/// C 508 and C 512.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_EXTERNAL_MENU_BAR {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}
/// C: haikuterm.h 374: `#define FRAME_HAIKU_MENU_BAR(f) FRAME_OUTPUT_DATA (f)->menubar` -
/// read at C 509/519/545/627/673/703/799/803/806/820/823, written at C 516.  The mirror's own
/// one-armed macro is shadowed here so that C's assignment has a target (the
/// `src/haikufns.rs` 395-402 shape).
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_HAIKU_MENU_BAR {
  ($f:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).menubar };
  ($f:expr, $v:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).menubar = $v };
}

/* ==============================================================================================
   C 31-34.  */

/* The item selected in the popup menu.  */
// C 31: `static Lisp_Object *volatile menu_item_selection;` (tentative, i.e. NULL; the `volatile`
// qualifier on the object itself has no Rust counterpart - recorded in the ledger's Deviations).
#[cfg (feature = "have_haiku")]
static mut menu_item_selection: *mut Lisp_Object = std::ptr::null_mut ();

// C 32: `static struct timespec menu_timer_timespec;` (tentative, i.e. every member zero).
#[cfg (feature = "have_haiku")]
static mut menu_timer_timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };

/* Flag indicating whether or not a popup menu or dialog has been posted and not yet
   popped down.  */
// C 34: `int popup_activated_p = 0;` - the definition `include/haikuterm.rs`'s `haikuterm_objs`
// declares (PORTING.md 0.1-10/P-21).
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut popup_activated_p: c_int = 0;

/* ==============================================================================================
   C 36-186: `static void digest_menu_items (void *first_menu, int start, int menu_items_used,
   bool is_menu_bar)`.  */

/* C 37's third parameter is named `menu_items_used`, which shadows the file-scope static of
   the same name (`include/keyboard.rs`'s `keyboard_objs`); Rust forbids a function parameter
   named like a static (E0530), so the port spells it with the trailing underscore the token
   comparison strips (the `src/alloc.rs` 4333-4334 `finalizers_` / PORTING.md 0.2 shape).  The
   ledger records the spelling.  */
#[cfg (feature = "have_haiku")]
unsafe fn digest_menu_items (first_menu: *mut c_void, start: c_int, menu_items_used_: c_int,
                             is_menu_bar: bool)                       // C 36-38
{
  let mut menus: *mut *mut c_void = std::ptr::null_mut ();            // C 40
  let mut panes: *mut *mut c_void = std::ptr::null_mut ();
  let mut i: c_int;                                                   // C 41
  let mut menu_depth: c_int;
  let mut menu: *mut c_void;                                          // C 42
  let mut window: *mut c_void;
  let mut view: *mut c_void;
  let mut pane_name: Lisp_Object;                                     // C 43
  let mut prefix: Lisp_Object;
  let mut pane_string: *const c_char;                                 // C 44
  let mut item_name: Lisp_Object;                                     // C 45
  let mut enable: Lisp_Object;
  let mut descrip: Lisp_Object;
  let mut def: Lisp_Object;
  let mut selected: Lisp_Object;
  let mut help: Lisp_Object;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                              // C 47

  let menu_len: c_int = menu_items_used_ - start + 1;                 // C 49
  let pane_len: c_int = menu_items_used_ - start + 1;                 // C 50
  menu = first_menu;                                                  // C 51

  i = start;                                                          // C 53
  menu_depth = 0;                                                     // C 54

  SAFE_NALLOCA! (sa_avail, menus, 1, menu_len);                       // C 56
  SAFE_NALLOCA! (sa_avail, panes, 1, pane_len);                       // C 57
  memset (menus as *mut c_void, 0,
          menu_len as usize * core::mem::size_of::<*mut c_void> ());  // C 58
  *menus = first_menu;                                                // C 59
  memset (panes as *mut c_void, 0,
          pane_len as usize * core::mem::size_of::<*mut c_void> ());  // C 60

  window = std::ptr::null_mut ();                                     // C 62
  view = std::ptr::null_mut ();                                       // C 63

  if FRAMEP (Vmenu_updating_frame)
     && FRAME_LIVE_P (XFRAME (Vmenu_updating_frame))
     && FRAME_HAIKU_P (XFRAME (Vmenu_updating_frame))                 // C 65-67
    {
      window = FRAME_HAIKU_WINDOW! (XFRAME (Vmenu_updating_frame));   // C 69
      view = FRAME_HAIKU_VIEW! (XFRAME (Vmenu_updating_frame));       // C 70
    }

  if !view.is_null ()                                                 // C 73
    { BView_draw_lock (view, false, 0, 0, 0, 0); }                    // C 74

  while i < menu_items_used_                                          // C 76-180
    {
      if NILP (AREF (menu_items, i as isize))                         // C 78
	{
	  menu_depth += 1;                                             // C 80: `menus[++menu_depth] = menu;`
	  *menus.offset (menu_depth as isize) = menu;
	  i += 1;                                                      // C 81
	}
      else if EQ (AREF (menu_items, i as isize), Qlambda ())           // C 83
	{
	  *panes.offset (menu_depth as isize) = std::ptr::null_mut ();   // C 85
	  menu_depth -= 1;
	  menu = if !(*panes.offset (menu_depth as isize)).is_null ()    // C 86
		   { *panes.offset (menu_depth as isize) }
		 else
		   { *menus.offset (menu_depth as isize) };
	  i += 1;                                                      // C 87
	}
      else if EQ (AREF (menu_items, i as isize), Qquote ())            // C 89
	{ i += 1; }                                                    // C 90
      else if EQ (AREF (menu_items, i as isize), Qt ())                // C 91
	{
	  if menu_items_n_panes == 1                                   // C 93
	    {
	      i += MENU_ITEMS_PANE_LENGTH;                             // C 95
	      continue;                                                // C 96
	    }

	  pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);   // C 99
	  prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);    // C 100

	  if STRINGP (pane_name) && STRING_MULTIBYTE (pane_name)       // C 102
	    {
	      pane_name = ENCODE_UTF_8 (pane_name);                    // C 104
	      ASET (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize, pane_name);   // C 105
	    }

	  pane_string = if NILP (pane_name)                            // C 108-109
			  { c"".as_ptr () }
			else
			  { SSDATA (pane_name) };
	  if !NILP (prefix)                                            // C 110
	    { pane_string = pane_string.add (1); }                     // C 111

	  if strcmp (pane_string, c"".as_ptr ()) != 0                  // C 113
	    {
	      menu = BMenu_new_submenu (*menus.offset (menu_depth as isize),
					pane_string, true);            // C 115-116
	      *panes.offset (menu_depth as isize) = menu;
	    }

	  i += MENU_ITEMS_PANE_LENGTH;                                 // C 119
	}
      else
	{
	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 122
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 123
	  descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 124
	  def = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_DEFINITION as isize);   // C 125
	  selected = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_SELECTED as isize);   // C 126
	  help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);        // C 127

	  if STRINGP (item_name) && STRING_MULTIBYTE (item_name)       // C 129
	    {
	      item_name = ENCODE_UTF_8 (item_name);                    // C 131
	      ASET (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize, item_name);   // C 132
	    }

	  if STRINGP (descrip) && STRING_MULTIBYTE (descrip)           // C 135
	    {
	      descrip = ENCODE_UTF_8 (descrip);                        // C 137
	      ASET (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize, descrip);   // C 138
	    }

	  if STRINGP (help) && STRING_MULTIBYTE (help)                 // C 141
	    { help = ENCODE_UTF_8 (help); }                            // C 142

	  if i + MENU_ITEMS_ITEM_LENGTH < menu_items_used_
	     && NILP (AREF (menu_items, i as isize + MENU_ITEMS_ITEM_LENGTH as isize))   // C 144-145
	    { menu = BMenu_new_submenu (menu, SSDATA (item_name), !NILP (enable)); }   // C 146
	  else if NILP (def) && menu_separator_name_p (SSDATA (item_name))   // C 147
	    { BMenu_add_separator (menu); }                            // C 148
	  else if !is_menu_bar                                        // C 149
	    {
	      if !use_system_tooltips || NILP (Fsymbol_value (Qtooltip_mode ()))   // C 151
		{
		  BMenu_add_item (menu, SSDATA (item_name),            // C 152-158
				  if !NILP (def)
				    { aref_addr (menu_items, i as isize) as *mut c_void }
				  else
				    { std::ptr::null_mut () },
				  !NILP (enable), !NILP (selected), false, window,
				  if !NILP (descrip) { SSDATA (descrip) }
				  else { std::ptr::null () },
				  std::ptr::null ());
		}
	      else
		{
		  BMenu_add_item (menu, SSDATA (item_name),            // C 160-166
				  if !NILP (def)
				    { aref_addr (menu_items, i as isize) as *mut c_void }
				  else
				    { std::ptr::null_mut () },
				  !NILP (enable), !NILP (selected), false, window,
				  if !NILP (descrip) { SSDATA (descrip) }
				  else { std::ptr::null () },
				  if STRINGP (help) { SSDATA (help) }
				  else { std::ptr::null () });
		}
	    }
	  else if !use_system_tooltips || NILP (Fsymbol_value (Qtooltip_mode ()))   // C 168
	    {
	      BMenu_add_item (menu, SSDATA (item_name),                // C 169-174
			      if !NILP (def) { i as intptr_t as *mut c_void }
			      else { std::ptr::null_mut () },
			      !NILP (enable), !NILP (selected), true, window,
			      if !NILP (descrip) { SSDATA (descrip) }
			      else { std::ptr::null () },
			      std::ptr::null ());
	    }
	  else
	    {
	      BMenu_add_item (menu, SSDATA (item_name),                // C 176-181
			      if !NILP (def) { i as intptr_t as *mut c_void }
			      else { std::ptr::null_mut () },
			      !NILP (enable), !NILP (selected), true, window,
			      if !NILP (descrip) { SSDATA (descrip) }
			      else { std::ptr::null () },
			      if STRINGP (help) { SSDATA (help) }
			      else { std::ptr::null () });
	    }

	  i += MENU_ITEMS_ITEM_LENGTH;                                 // C 183
	}
    }

  if !view.is_null ()                                                 // C 182
    { BView_draw_unlock (view); }                                     // C 183

  SAFE_FREE! (sa_count, sa_mark);                                              // C 185
}

/* ==============================================================================================
   C 188-301: `static Lisp_Object haiku_dialog_show (struct frame *f, Lisp_Object title,
   Lisp_Object header, const char **error_name)`.  */

#[cfg (feature = "have_haiku")]
unsafe fn haiku_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
			     error_name: *mut *const c_char) -> Lisp_Object   // C 188-190
{
  let mut i: c_int;                                                   // C 192
  let mut nb_buttons: c_int = 0;
  let mut boundary_seen: bool = false;                                // C 193
  let mut pane_name: Lisp_Object;                                     // C 194
  let mut vals: [Lisp_Object; 10] = [Qnil (); 10];
  let mut alert: *mut c_void;                                         // C 195
  let mut button: *mut c_void;
  let mut enabled_item_seen_p: bool = false;                           // C 196
  let mut val: int32;                                                  // C 197

  *error_name = std::ptr::null ();                                     // C 199

  if menu_items_n_panes > 1                                            // C 202
    {
      *error_name = c"Multiple panes in dialog box".as_ptr ();          // C 204
      return Qnil ();                                                  // C 205
    }

  pane_name = AREF (menu_items, MENU_ITEMS_PANE_NAME as isize);        // C 208
  i = MENU_ITEMS_PANE_LENGTH;                                          // C 209

  if STRING_MULTIBYTE (pane_name)                                      // C 211
    { pane_name = ENCODE_UTF_8 (pane_name); }                          // C 212

  block_input ();                                                      // C 214
  alert = BAlert_new (SSDATA (pane_name), if NILP (header)             // C 215-216
					  { HAIKU_INFO_ALERT }
					else
					  { HAIKU_IDEA_ALERT });

  while i < menu_items_used                                           // C 218-266
    {
      let mut item_name: Lisp_Object;                                  // C 220
      let mut enable: Lisp_Object;
      let mut descrip: Lisp_Object;
      let mut value: Lisp_Object;
      item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 221
      enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 222
      descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 223
      value = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);      // C 224

      if NILP (item_name)                                             // C 226
	{
	  BAlert_delete (alert);                                       // C 228
	  *error_name = c"Submenu in dialog items".as_ptr ();           // C 229
	  unblock_input ();                                            // C 230
	  return Qnil ();                                              // C 231
	}

      if EQ (item_name, Qquote ())                                    // C 234
	{
	  if nb_buttons != 0                                           // C 236
	    { boundary_seen = true; }                                  // C 237

	  i += 1;                                                      // C 239
	  continue;                                                    // C 240
	}

      if nb_buttons >= 9                                              // C 243
	{
	  BAlert_delete (alert);                                       // C 245
	  *error_name = c"Too many dialog items".as_ptr ();             // C 246
	  unblock_input ();                                            // C 247
	  return Qnil ();                                              // C 248
	}

      if STRING_MULTIBYTE (item_name)                                 // C 250
	{ item_name = ENCODE_UTF_8 (item_name); }                      // C 251
      if !NILP (descrip) && STRING_MULTIBYTE (descrip)                 // C 252
	{ descrip = ENCODE_UTF_8 (descrip); }                          // C 253

      button = BAlert_add_button (alert, SSDATA (item_name));          // C 255

      BButton_set_enabled (button, (!NILP (enable)) as c_int);          // C 257
      enabled_item_seen_p |= !NILP (enable);                           // C 258

      if !NILP (descrip)                                              // C 260
	{ BView_set_tooltip (button, SSDATA (descrip)); }               // C 261

      vals[nb_buttons as usize] = value;                               // C 263
      nb_buttons += 1;                                                // C 264
      i += MENU_ITEMS_ITEM_LENGTH;                                    // C 265
    }

  /* Haiku only lets us specify a single button to place on the
     left.  */
  if boundary_seen                                                   // C 270
    { BAlert_set_offset_spacing (alert); }                            // C 271

  /* If there isn't a single enabled item, add an "Ok" button so the
     popup can be dismissed.  */
  if !enabled_item_seen_p                                            // C 275
    { BAlert_add_button (alert, c"Ok".as_ptr ()); }                   // C 276
  unblock_input ();                                                  // C 277

  unrequest_sigio ();                                                // C 279
  popup_activated_p += 1;                                            // C 280
  val = BAlert_go (alert, Some (block_input_abi::block_input), Some (unblock_input),
		   Some (process_pending_signals));                  // C 281-282
  popup_activated_p -= 1;                                            // C 283
  request_sigio ();                                                  // C 284

  if val < 0                                                         // C 286
    { quit (); }                                                     // C 287
  else if val < nb_buttons                                           // C 288
    { return vals[val as usize]; }                                   // C 289

  /* The dialog was dismissed via the button appended to dismiss popup
     dialogs without a single enabled item.  */
  if nb_buttons != 0                                                 // C 293
    { quit (); }                                                     // C 294
  /* Otherwise, the Ok button was added because no buttons were seen
     at all.  */
  else
    { return Qt (); }                                                // C 298

  emacs_abort ();                                                    // C 300
}

/* ==============================================================================================
   C 303-335: `Lisp_Object haiku_popup_dialog (struct frame *f, Lisp_Object header,
   Lisp_Object contents)`.  */

/// C: Lisp_Object haiku_popup_dialog (struct frame *f, Lisp_Object header, Lisp_Object contents)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_popup_dialog (f: *mut frame, header: Lisp_Object,
					    mut contents: Lisp_Object) -> Lisp_Object   // C 303-304
{
  let mut title: Lisp_Object;                                        // C 306
  let mut error_name: *const c_char = std::ptr::null ();              // C 307
  let mut selection: Lisp_Object;                                    // C 308
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                  // C 309

  check_window_system (f);                                            // C 311

  /* Decode the dialog items from what was specified.  */
  title = Fcar (contents);                                            // C 314
  CHECK_STRING (title);                                               // C 315
  record_unwind_protect_void (Some (unuse_menu_items));               // C 316

  if NILP (Fcar (Fcdr (contents)))                                     // C 318
    /* No buttons specified, add an "Ok" button so users can pop down
       the dialog.  Also, the lesstif/motif version crashes if there are
       no buttons.  */
    { contents = list2 (title, Fcons (build_string (c"Ok".as_ptr ()), Qt ())); }   // C 322

  list_of_panes (list1 (contents));                                    // C 324

  /* Display them in a dialog box.  */
  selection = haiku_dialog_show (f, title, header, &raw mut error_name);   // C 327

  unbind_to (specpdl_count, Qnil ());                                  // C 329
  discard_menu_items ();                                               // C 330

  if !error_name.is_null ()                                           // C 332
    { error (c"%s".as_ptr (), error_name); }                           // C 333
  return selection;                                                    // C 334
}

/* ==============================================================================================
   C 337-347: `static void haiku_menu_show_help (void *help, void *data)`.  Its address is handed
   to `BMenu_run` (C 424), so it is `unsafe extern "C" fn` (PORTING.md 1.4/P-12).  */

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_menu_show_help (help: *mut c_void, data: *mut c_void)   // C 337-338
{
  let id: *mut Lisp_Object = help as *mut Lisp_Object;                 // C 340

  if !help.is_null ()                                                  // C 342
    { show_help_echo (*id.offset (MENU_ITEMS_ITEM_HELP as isize),      // C 343-344
		      Qnil (), Qnil (), Qnil ()); }
  else
    { show_help_echo (Qnil (), Qnil (), Qnil (), Qnil ()); }            // C 346
}

/* ==============================================================================================
   C 349-355: `static Lisp_Object haiku_process_pending_signals_for_menu_1 (void *ptr)`.  Its
   address is handed to `internal_catch_all` (C 374), so it is `unsafe extern "C" fn`.  */

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_process_pending_signals_for_menu_1 (ptr: *mut c_void) -> Lisp_Object   // C 349-350
{
  menu_timer_timespec = timer_check ();                                // C 352

  return Qnil ();                                                      // C 354
}

/* ==============================================================================================
   C 357-364: `static Lisp_Object haiku_process_pending_signals_for_menu_2 (enum nonlocal_exit
   exit, Lisp_Object error)`.  Its address is handed to `internal_catch_all` (C 374-375), so it is
   `unsafe extern "C" fn`.  */

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_process_pending_signals_for_menu_2 (exit: nonlocal_exit,
							      error: Lisp_Object)
  -> Lisp_Object                                                       // C 357-358
{
  menu_timer_timespec.tv_sec = 0;                                      // C 360
  menu_timer_timespec.tv_nsec = -1;                                    // C 361

  return Qnil ();                                                      // C 363
}

/* ==============================================================================================
   C 366-378: `static struct timespec haiku_process_pending_signals_for_menu (void)`.  Its address
   is handed to `BMenu_run` (C 426), so it is `unsafe extern "C" fn () -> timespec`.  */

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_process_pending_signals_for_menu () -> timespec   // C 366-367
{
  process_pending_signals ();                                          // C 369

  /* The original idea was to let timers throw so that timeouts can
     work correctly, but there's no way to pop down a BPopupMenu
     that's currently popped up.  */
  internal_catch_all (Some (haiku_process_pending_signals_for_menu_1),
		      std::ptr::null_mut (),
		      Some (haiku_process_pending_signals_for_menu_2));   // C 374-375

  return menu_timer_timespec;                                          // C 377
}

/* ==============================================================================================
   C 380-503: `Lisp_Object haiku_menu_show (struct frame *f, int x, int y, int menuflags,
   Lisp_Object title, const char **error_name)`.  */

/// C: Lisp_Object haiku_menu_show (struct frame *f, int x, int y, int menuflags, Lisp_Object
/// title, const char **error_name)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_menu_show (f: *mut frame, mut x: c_int, mut y: c_int,
					  menuflags: c_int, mut title: Lisp_Object,
					  error_name: *mut *const c_char) -> Lisp_Object   // C 380-382
{
  let mut i: c_int;                                                   // C 383
  let mut submenu_depth: c_int;
  let mut j: c_int;
  let mut view: *mut c_void;                                          // C 384
  let mut menu: *mut c_void;
  let mut subprefix_stack: *mut Lisp_Object = std::ptr::null_mut ();   // C 385
  let mut prefix: Lisp_Object;                                        // C 386
  let mut entry: Lisp_Object;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                              // C 388

  view = FRAME_HAIKU_VIEW! (f);                                       // C 390
  i = 0;                                                              // C 391
  submenu_depth = 0;                                                  // C 392
  SAFE_NALLOCA! (sa_avail, subprefix_stack, 1, menu_items_used);       // C 393

  eassert! (FRAME_HAIKU_P (f));                                       // C 395

  *error_name = std::ptr::null ();                                     // C 397

  if menu_items_used <= MENU_ITEMS_PANE_LENGTH                        // C 399
    {
      *error_name = c"Empty menu".as_ptr ();                            // C 401

      SAFE_FREE! (sa_count, sa_mark);                                          // C 403
      return Qnil ();                                                 // C 404
    }

  block_input ();                                                     // C 406
  if STRINGP (title) && STRING_MULTIBYTE (title)                       // C 407
    { title = ENCODE_UTF_8 (title); }                                  // C 408

  menu = BPopUpMenu_new (if STRINGP (title) { SSDATA (title) }         // C 410
			 else { std::ptr::null () });
  if STRINGP (title)                                                  // C 411
    {
      BMenu_add_title (menu, SSDATA (title));                          // C 413
      BMenu_add_separator (menu);                                      // C 414
    }
  digest_menu_items (menu, 0, menu_items_used, false);                 // C 415
  BView_convert_to_screen (view, &raw mut x, &raw mut y);              // C 416
  unblock_input ();                                                   // C 417

  unrequest_sigio ();                                                 // C 419
  popup_activated_p += 1;                                             // C 420
  menu_item_selection = BMenu_run (menu, x, y, Some (haiku_menu_show_help),   // C 421-423
				   Some (block_input_abi::block_input),
				   Some (unblock_input),
				   Some (haiku_process_pending_signals_for_menu),
				   std::ptr::null_mut ()) as *mut Lisp_Object;
  popup_activated_p -= 1;                                             // C 424
  request_sigio ();                                                   // C 425

  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                             // C 427

  /* Clear the grab view manually.  There is a race condition here if
     the window thread receives a button press between here and the
     end of BMenu_run.  */
  be_clear_grab_view ();                                              // C 432

  if !menu_item_selection.is_null ()                                  // C 434
    {
      prefix = Qnil (); entry = Qnil ();                              // C 436
      i = 0;                                                          // C 437
      while i < menu_items_used                                       // C 438-488
	{
	  if NILP (AREF (menu_items, i as isize))                     // C 440
	    {
	      *subprefix_stack.offset (submenu_depth as isize) = prefix;   // C 442
	      submenu_depth += 1;
	      prefix = entry;                                         // C 443
	      i += 1;                                                 // C 444
	    }
	  else if EQ (AREF (menu_items, i as isize), Qlambda ())       // C 446
	    {
	      submenu_depth -= 1;
	      prefix = *subprefix_stack.offset (submenu_depth as isize);   // C 448
	      i += 1;                                                 // C 449
	    }
	  else if EQ (AREF (menu_items, i as isize), Qt ())            // C 450
	    {
	      prefix = AREF (menu_items,
			     i as isize + MENU_ITEMS_PANE_PREFIX as isize);   // C 452
	      i += MENU_ITEMS_PANE_LENGTH;                             // C 453
	    }
	  /* Ignore a nil in the item list.
	     It's meaningful only for dialog boxes.  */
	  else if EQ (AREF (menu_items, i as isize), Qquote ())        // C 457
	    { i += 1; }                                               // C 458
	  else
	    {
	      entry = AREF (menu_items,
			    i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 462
	      if menu_item_selection == aref_addr (menu_items, i as isize)   // C 463
		{
		  if (menuflags & MENU_KEYMAPS) != 0                  // C 465
		    {
		      entry = list1 (entry);                          // C 467
		      if !NILP (prefix)                               // C 468
			{ entry = Fcons (prefix, entry); }             // C 469
		      j = submenu_depth - 1;                          // C 470
		      while j >= 0
			{
			  if !NILP (*subprefix_stack.offset (j as isize))   // C 472
			    { entry = Fcons (*subprefix_stack.offset (j as isize), entry); }   // C 473
			  j -= 1;
			}
		    }
		  block_input ();                                    // C 475
		  BPopUpMenu_delete (menu);                          // C 476
		  unblock_input ();                                  // C 477

		  SAFE_FREE! (sa_count, sa_mark);                             // C 479
		  return entry;                                      // C 480
		}
	      i += MENU_ITEMS_ITEM_LENGTH;                           // C 486
	    }
	}
    }
  else if (menuflags & MENU_FOR_CLICK) == 0                           // C 490
    {
      block_input ();                                                // C 492
      BPopUpMenu_delete (menu);                                      // C 493
      unblock_input ();                                              // C 494
      quit ();                                                       // C 495
    }
  block_input ();                                                    // C 497
  BPopUpMenu_delete (menu);                                          // C 498
  unblock_input ();                                                  // C 499

  SAFE_FREE! (sa_count, sa_mark);                                             // C 501
  return Qnil ();                                                    // C 502
}

/* ==============================================================================================
   C 505-527: `void free_frame_menubar (struct frame *f)`.  */

/// C: void free_frame_menubar (struct frame *f)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_frame_menubar (f: *mut frame)           // C 505-506
{
  let mut mbar: *mut c_void;                                          // C 508

  FRAME_MENU_BAR_LINES! (f, 0);                                       // C 510
  FRAME_MENU_BAR_HEIGHT! (f, 0);                                      // C 511
  FRAME_EXTERNAL_MENU_BAR! (f, 0);                                    // C 512

  block_input ();                                                     // C 514
  mbar = FRAME_HAIKU_MENU_BAR! (f);                                   // C 515
  FRAME_HAIKU_MENU_BAR! (f, std::ptr::null_mut ());                    // C 516

  if !mbar.is_null ()                                                 // C 518
    { BMenuBar_delete (mbar); }                                       // C 519

  if haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f))      // C 521
    { popup_activated_p -= 1; }                                       // C 522
  haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 0);     // C 523
  unblock_input ();                                                   // C 524

  adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ());         // C 526
}

/* ==============================================================================================
   C 529-536: `void initialize_frame_menubar (struct frame *f)`.  */

/// C: void initialize_frame_menubar (struct frame *f)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_frame_menubar (f: *mut frame)     // C 529-530
{
  /* This function is called before the first chance to redisplay
     the frame.  It has to be, so the frame will have the right size.  */
  fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 534
  set_frame_menubar (f, true);                                        // C 535
}

/* ==============================================================================================
   C 538-729: `void set_frame_menubar (struct frame *f, bool deep_p)`.  */

/// C: void set_frame_menubar (struct frame *f, bool deep_p)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_frame_menubar (f: *mut frame, deep_p: bool)   // C 538-539
{
  let mut mbar: *mut c_void = FRAME_HAIKU_MENU_BAR! (f);               // C 540
  let view: *mut c_void = FRAME_HAIKU_VIEW! (f);                       // C 541
  let mut first_time_p: bool = false;                                  // C 542

  if mbar.is_null ()                                                   // C 544
    {
      block_input ();                                                  // C 546
      mbar = BMenuBar_new (view);                                      // C 547
      FRAME_HAIKU_MENU_BAR! (f, mbar);
      first_time_p = true;                                             // C 548

      /* Now wait for the MENU_BAR_RESIZE event informing us of the
	 initial dimensions of that menu bar.  */
      if FRAME_VISIBLE_P (f)                                           // C 552
	{ haiku_wait_for_event (f, MENU_BAR_RESIZE); }                  // C 553

      unblock_input ();                                                // C 555
    }

  let mut items: Lisp_Object;                                          // C 559
  let prev: *mut buffer = (current_buffer! ());                              // C 560
  let mut buffer: Lisp_Object;                                         // C 561
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                    // C 562
  let mut previous_menu_items_used: c_int = (*f).menu_bar_items_used;   // C 563
  let mut previous_items: *mut Lisp_Object
    = sa_stack_alloc (previous_menu_items_used as usize
		      * core::mem::size_of::<Lisp_Object> ()) as *mut Lisp_Object;   // C 564-565
  let mut count: c_int = 0;                                            // C 566
  let mut subitems: ptrdiff_t;                                         // C 567
  let mut i: ptrdiff_t;                                                // C 567
  let mut submenu_start: *mut c_int;                                   // C 568
  let mut submenu_end: *mut c_int;
  let mut submenu_n_panes: *mut c_int;
  let mut submenu_names: *mut Lisp_Object;                             // C 569

  XSETFRAME! (Vmenu_updating_frame, f);                                // C 571

  if !deep_p                                                           // C 573
    {
      items = FRAME_MENU_BAR_ITEMS (f);                                // C 575
      let mut string: Lisp_Object;                                     // C 576

      block_input ();                                                  // C 578
      let count: c_int = BMenu_count_items (mbar);                     // C 579

      let mut i: c_int = 0;                                            // C 581
      while i < ASIZE (items) as c_int                                 // C 582
	{
	  string = AREF (items, i as isize + 1);                        // C 584

	  if !STRINGP (string)                                          // C 586
	    { break; }                                                  // C 587

	  if STRING_MULTIBYTE (string)                                  // C 589
	    { string = ENCODE_UTF_8 (string); }                         // C 590

	  if i / 4 < count                                              // C 592
	    {
	      let it: *mut c_void = BMenu_item_at (mbar, i / 4);        // C 594
	      BMenu_item_set_label (it, SSDATA (string));                // C 595
	    }
	  else
	    { BMenu_new_menu_bar_submenu (mbar, SSDATA (string)); }      // C 598

	  i += 4;                                                       // C 582 (the for's `i += 4`)
	}

      if i / 4 < count                                                  // C 601
	{ BMenu_delete_from (mbar, i / 4, count - i / 4 + 1); }          // C 602
      unblock_input ();                                                 // C 603

      (*f).menu_bar_items_used = 0;                                     // C 605
    }
  else                                                                 // C 607
    {
      /* If we are making a new widget, its contents are empty,
	 do always reinitialize them.  */
      if first_time_p                                                  // C 611
	{ previous_menu_items_used = 0; }                                // C 612

      buffer = (*XWINDOW (FRAME_SELECTED_WINDOW (f))).contents;         // C 614
      specbind (Qinhibit_quit (), Qt ());                                // C 615
      /* Don't let the debugger step into this code
	 because it is not reentrant.  */
      specbind (Qdebug_on_next_call (), Qnil ());                        // C 618

      record_unwind_save_match_data ();                                 // C 620
      if NILP (Voverriding_local_map_menu_flag)                         // C 621
	{
	  specbind (Qoverriding_terminal_local_map (), Qnil ());            // C 623
	  specbind (Qoverriding_local_map (), Qnil ());                     // C 624
	}

      set_buffer_internal_1 (XBUFFER (buffer));                         // C 627

      /* Run the Lucid hook.  */
      safe_run_hooks (Qactivate_menubar_hook ());                        // C 630

      /* If it has changed current-menubar from previous value,
	 really recompute the menubar from the value.  */
      safe_run_hooks (Qmenu_bar_update_hook ());                         // C 634
      fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 635

      items = FRAME_MENU_BAR_ITEMS (f);                                 // C 637

      /* Save the frame's previous menu bar contents data.  */
      if previous_menu_items_used != 0                                  // C 640
	{ memcpy (previous_items as *mut c_void,
		  xvector_contents ((*f).menu_bar_vector),
		  previous_menu_items_used as usize * word_size as usize); }   // C 641-642

      /* Fill in menu_items with the current menu bar contents.
	 This can evaluate Lisp code.  */
      save_menu_items ();                                               // C 646

      menu_items = (*f).menu_bar_vector;                                 // C 648
      menu_items_allocated = if VECTORP (menu_items)                     // C 649
			       { ASIZE (menu_items) as c_int }
			     else
			       { 0 };
      subitems = ASIZE (items) / 4;                                     // C 650
      submenu_start = sa_stack_alloc (((subitems + 1) as usize)
				      * core::mem::size_of::<c_int> ()) as *mut c_int;   // C 651
      submenu_end = sa_stack_alloc ((subitems as usize)
				    * core::mem::size_of::<c_int> ()) as *mut c_int;   // C 652
      submenu_n_panes = sa_stack_alloc ((subitems as usize)
					* core::mem::size_of::<c_int> ()) as *mut c_int;   // C 653
      submenu_names = sa_stack_alloc ((subitems as usize)
				      * core::mem::size_of::<Lisp_Object> ()) as *mut Lisp_Object;   // C 654

      init_menu_items ();                                               // C 656
      i = 0;                                                            // C 657
      while i < subitems                                                // C 657-678
	{
	  let mut key: Lisp_Object;                                       // C 659
	  let mut string: Lisp_Object;
	  let mut maps: Lisp_Object;

	  key = AREF (items, 4 * i);                                      // C 661
	  string = AREF (items, 4 * i + 1);                               // C 662
	  maps = AREF (items, 4 * i + 2);                                 // C 663
	  if NILP (string)                                                // C 664
	    { break; }                                                    // C 665

	  if STRINGP (string) && STRING_MULTIBYTE (string)                // C 667
	    { string = ENCODE_UTF_8 (string); }                           // C 668

	  *submenu_start.offset (i) = menu_items_used;                    // C 670

	  menu_items_n_panes = 0;                                         // C 672
	  parse_single_submenu (key, string, maps);                       // C 673
	  *submenu_n_panes.offset (i) = menu_items_n_panes;               // C 674

	  *submenu_end.offset (i) = menu_items_used;                      // C 676
	  *submenu_names.offset (i) = string;                             // C 677

	  i += 1;                                                         // C 657 (the for's `i++`)
	}

      *submenu_start.offset (i) = -1;                                   // C 680
      finish_menu_items ();                                             // C 681

      set_buffer_internal_1 (prev);                                     // C 683

      /* If there has been no change in the Lisp-level contents
	 of the menu bar, skip redisplaying it.  Just exit.  */

      /* Compare the new menu items with the ones computed last time.  */
      i = 0;                                                            // C 689
      while i < previous_menu_items_used as ptrdiff_t                    // C 689-692
	{
	  if menu_items_used as ptrdiff_t == i
	     || !EQ (*previous_items.offset (i), AREF (menu_items, i))     // C 690-691
	    { break; }
	  i += 1;
	}
      if i == menu_items_used as ptrdiff_t
	 && i == previous_menu_items_used as ptrdiff_t
	 && i != 0                                                       // C 693
	{
	  /* The menu items have not changed.  Don't bother updating
	     the menus in any form, since it would be a no-op.  */
	  discard_menu_items ();                                          // C 697
	  unbind_to (specpdl_count, Qnil ());                              // C 698
	  return;                                                        // C 699
	}

      /* Convert menu_items into widget_value trees
	 to display the menu.  This cannot evaluate Lisp code.  */

      block_input ();                                                   // C 705
      count = BMenu_count_items (mbar);                                  // C 706
      i = 0;                                                             // C 707
      while *submenu_start.offset (i) >= 0                               // C 707-719
	{
	  let mut mn: *mut c_void = std::ptr::null_mut ();                  // C 709
	  if i < count as ptrdiff_t                                        // C 710
	    { mn = BMenu_item_get_menu (BMenu_item_at (mbar, i as c_int)); }   // C 711
	  if !mn.is_null ()                                                // C 712
	    { BMenu_delete_all (mn); }                                     // C 713
	  else
	    { mn = BMenu_new_menu_bar_submenu (mbar,
					       SSDATA (*submenu_names.offset (i))); }   // C 715

	  menu_items_n_panes = *submenu_n_panes.offset (i);                 // C 717
	  digest_menu_items (mn, *submenu_start.offset (i), *submenu_end.offset (i), true);   // C 718

	  i += 1;                                                          // C 707 (the for's `++i`)
	}
      unblock_input ();                                                // C 720

      /* The menu items are different, so store them in the frame.  */
      fset_menu_bar_vector (f, menu_items);                             // C 723
      (*f).menu_bar_items_used = menu_items_used;                        // C 724
    }

  /* This undoes save_menu_items.  */
  unbind_to (specpdl_count, Qnil ());                                  // C 728
}

/* ==============================================================================================
   C 731-754: `void run_menu_bar_help_event (struct frame *f, int mb_idx)`.  */

/// C: void run_menu_bar_help_event (struct frame *f, int mb_idx)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_menu_bar_help_event (f: *mut frame, mb_idx: c_int)   // C 731-732
{
  let mut frame: Lisp_Object;                                          // C 734
  let mut vec: Lisp_Object;
  let mut help: Lisp_Object;

  XSETFRAME! (frame, f);                                               // C 736

  block_input ();                                                      // C 738
  if mb_idx < 0                                                        // C 739
    {
      kbd_buffer_store_help_event (frame, Qnil ());                     // C 741
      unblock_input ();                                                 // C 742
      return;                                                          // C 743
    }

  vec = (*f).menu_bar_vector;                                          // C 746
  if (mb_idx + MENU_ITEMS_ITEM_HELP) as isize >= ASIZE (vec)            // C 747
    { return; }                                                        // C 748

  help = AREF (vec, (mb_idx + MENU_ITEMS_ITEM_HELP) as isize);          // C 750
  if STRINGP (help) || NILP (help)                                     // C 751
    { kbd_buffer_store_help_event (frame, help); }                     // C 752
  unblock_input ();                                                    // C 753
}

/* ==============================================================================================
   C 756-761.  C 757's `doc: /* SKIP: real doc in xmenu.c. */` is the docstring.  */

/* The following is used by delayed window autoselection.  */
#[cfg (feature = "have_haiku")]
DEFUN! ("menu-or-popup-active-p", Fmenu_or_popup_active_p, Smenu_or_popup_active_p, 0, 0, 0,
	doc: "SKIP: real doc in xmenu.c.",                            // C 757
	(),                                                           // C 758
	{
  return if popup_activated_p != 0 { Qt () } else { Qnil () };        // C 760
});

/* ==============================================================================================
   C 763-793.  C 763's `"i"` is the interactive spec, C 771's `(Lisp_Object frame)` the argument
   list.  */

#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-menu-bar-open", Fhaiku_menu_bar_open, Shaiku_menu_bar_open, 0, 1, "i",
	doc: "Show and start key navigation of the menu bar in FRAME.\n\
This initially opens the first menu bar item and you can then navigate\n\
with the arrow keys, select a menu entry with the return key, or\n\
cancel with the escape key.  If FRAME is nil or not given, use the\n\
selected frame.  If FRAME has no menu bar, a pop-up is displayed at\n\
the position of the last non-menu event instead.",                   // C 764-769
	(frame: Lisp_Object),                                         // C 771
	{
  let f: *mut frame = decode_window_system_frame (frame);             // C 773
  let mut rc: c_int;                                                  // C 774

  if FRAME_EXTERNAL_MENU_BAR (f)                                      // C 776
    {
      block_input ();                                                 // C 778
      set_frame_menubar (f, true);                                    // C 779
      rc = BMenuBar_start_tracking (FRAME_HAIKU_MENU_BAR! (f)) as c_int;   // C 780
      unblock_input ();                                               // C 781

      if rc == 0                                                      // C 783
	{ return Qnil (); }                                           // C 784

      haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 1);   // C 786
      popup_activated_p += 1;                                         // C 787
    }
  else
    { return calln! (Qpopup_menu (), calln! (Qmouse_menu_bar_map ()),
		     last_nonmenu_event); }                           // C 788-790

  return Qnil ();                                                     // C 792
});

/* ==============================================================================================
   C 795-832: `void haiku_activate_menubar (struct frame *f)`.  */

/// C: void haiku_activate_menubar (struct frame *f)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_activate_menubar (f: *mut frame)       // C 795-796
{
  let mut rc: c_int;                                                   // C 798

  if FRAME_HAIKU_MENU_BAR! (f).is_null ()                               // C 800
    { return; }

  set_frame_menubar (f, true);                                         // C 803

  if !(*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event.is_null ()   // C 805
    {
      block_input ();                                                  // C 807
      rc = be_replay_menu_bar_event (FRAME_HAIKU_MENU_BAR! (f),
				     (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event) as c_int;   // C 808-809
      xfree ((*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event as *mut c_void);   // C 810
      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event = std::ptr::null_mut ();   // C 811
      unblock_input ();                                                // C 812

      if rc == 0                                                       // C 814
	{ return; }

      haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 1);   // C 817
      popup_activated_p += 1;                                          // C 818
    }
  else
    {
      block_input ();                                                  // C 821
      rc = BMenuBar_start_tracking (FRAME_HAIKU_MENU_BAR! (f)) as c_int;   // C 822
      unblock_input ();                                                // C 823

      if rc == 0                                                       // C 825
	{ return; }

      haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 1);   // C 828
      popup_activated_p += 1;                                          // C 829
    }
}

/* ==============================================================================================
   C 834-845: `void syms_of_haikumenu (void)`.  */

/// C: void syms_of_haikumenu (void)
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_haikumenu ()                         // C 834-835
{
  DEFSYM! (Qdebug_on_next_call, c"debug-on-next-call");               // C 837
  DEFSYM! (Qpopup_menu, c"popup-menu");                               // C 838
  DEFSYM! (Qmouse_menu_bar_map, c"mouse-menu-bar-map");               // C 839
  DEFSYM! (Qtooltip_mode, c"tooltip-mode");                           // C 840

  defsubr (&raw mut Smenu_or_popup_active_p);                         // C 842
  defsubr (&raw mut Shaiku_menu_bar_open);                            // C 843
  return;                                                             // C 844
}
