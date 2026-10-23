// femacs: port of src/keyboard.h @ fd7afced403.
//
// WIP mirror (PORTING.md P-66): the two types, the `kset_*` setters and the event accessors
// the frozen configuration's live text carries.  `tools/check-cond-eval.py` reports zero
// disagreements for the header, so the branch set is this target's (for example
// `ENCODE_MENU_STRING` takes the `string_make_unibyte` arm: no GTK, no NS, no X i18n), and
// `struct selection_input_event sie` in `buffered_input_event` is dead (`HAVE_X11`/`HAVE_PGTK`
// are `#undef`).  The prototypes after line 520 land with keyboard.c (batch E).
//
// `KVAR` (keyboard.h 38) is a **token-pasting** macro (`((kboard)->field ## _)`) with no Rust
// counterpart; the port writes the field directly at each use site (every field it names ends
// in `_`), which is recorded in docs/ports/excluded-macros.txt.

// keyboard.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_KEYBOARD_H: bool = true;

// keyboard.h 23-24: the header's own `#include`s.  C reaches `coding.h` here, which is how a
// translation unit that includes `keyboard.h` sees `struct coding_system` complete - and
// `termhooks.h` 549/554/927-928 names that tag only as an incomplete one, so the port's
// `termhooks.rs` (`*mut coding_system`) resolves it through this mirror (P-72 rule ④).
include!("coding.rs");                        // C 23: `#include "coding.h"`
include!("termhooks.rs");                     // C 24: `#include "termhooks.h"`
/* keyboard.h 30-32: C reaches `struct selection_input_event` through the backend header, one per
   window system and each behind its own macro.  **The port does not carry that edge**: an
   `include!` is read by `tools/check-mirror-dupes.py` whatever its attribute, and the pgtkterm.h
   mirror brings pgtkterm.h's whole prototype surface with the record, so wiring it here made 24 of
   this mirror's 31 includers collide with names they declare themselves (the first reading of this
   slice: `40 duplicate declaration(s), 24 failure(s)`).  The member below is carried behind the same
   arm's macro instead, and the pgtk documentation unit resolves the type through its own include of
   the pgtkterm.h mirror, which precedes this one in C's own order (pgtkterm.c 28 vs 61).  */
// keyboard.h 267-269 declare `struct buffer *buffer_before_last_command_or_undo` and
// `struct buffer *prev_buffer`; C's `keyboard.h` does **not** include `buffer.h`, so the two
// tags are incomplete there and only legal behind pointers.  Rust has no incomplete types, and
// the declaration cannot degrade to `*mut c_void` (the declarations are at the crate root and
// `src/keyboard.rs` 1810 / `src/undo.rs` 97/302 assign and compare them against
// `current_buffer`), so the mirror carries the owner's include (P-72 rule ④ - the same shape
// as the two above, one header further than C's own list goes).  Measured no-op for every
// frozen unit: 31 units in `femacs/src/` include this mirror, and the other 30 all include
// `buffer.rs` already (only `src/androidmenu.c` does not).
include!("buffer.rs");                        // C keyboard.h 267/269's `struct buffer`

// keyboard.h 380-383.
pub const KBD_BUFFER_SIZE: c_int = 4096;

/* Each KBOARD represents one logical input stream from which Emacs
   gets input.  */
/// lisp.h 3570: `typedef struct kboard KBOARD;`.  C declares the typedef in lisp.h while the
/// tag's definition is here; the port keeps both in this file so that a mirror only ever names
/// types it declares (PORTING.md P-72), and `lisp.rs` says where it went.
pub type KBOARD = kboard;
// keyboard.h 77-236.  The layout is measured: 200 bytes, alignment 8 — `next_kboard` 0, ten
// Lisp_Objects 8…79, `kbd_macro_buffer`/`_ptr`/`_end` 80/88/96, `kbd_macro_bufsize` 104, six more
// Lisp_Objects 112…159, `reference_count` 168, `echo_string_` 176, `kbd_queue_has_data` 184,
// `immediate_echo` 185 and `echo_prompt_` 192.
#[repr(C)]
pub struct kboard
{
  pub next_kboard: *mut KBOARD,                     // keyboard.h 79

  pub Voverriding_terminal_local_map_: Lisp_Object, // 81
  pub Vlast_command_: Lisp_Object,
  pub Vreal_last_command_: Lisp_Object,
  pub Vkeyboard_translate_table_: Lisp_Object,
  pub Vlast_repeatable_command_: Lisp_Object,
  pub Vprefix_arg_: Lisp_Object,
  pub Vlast_prefix_arg_: Lisp_Object,
  pub kbd_queue_: Lisp_Object,
  pub defining_kbd_macro_: Lisp_Object,
  pub kbd_macro_buffer: *mut Lisp_Object,           // 80 (after the ten Lisp_Objects)
  pub kbd_macro_ptr: *mut Lisp_Object,
  pub kbd_macro_end: *mut Lisp_Object,
  pub kbd_macro_bufsize: ptrdiff_t,                 // 104
  pub Vlast_kbd_macro_: Lisp_Object,
  pub Vsystem_key_alist_: Lisp_Object,
  pub system_key_syms_: Lisp_Object,
  pub Vwindow_system_: Lisp_Object,
  pub Vlocal_function_key_map_: Lisp_Object,
  pub Vinput_decode_map_: Lisp_Object,
  pub Vdefault_minibuffer_frame_: Lisp_Object,      // 160
  pub reference_count: c_int,                       // 168
  pub echo_string_: Lisp_Object,                    // 176
  pub kbd_queue_has_data: bool,                     // 184 (`bool_bf` is conf_post.h's `bool`)
  // keyboard.h 181: `bool_bf immediate_echo : 1`.  `bool_bf` is one byte here, so this is its
  // own 1-byte unit: with both flags set the bytes are `01 01`, and with only this one `00 01`
  // (measured) — the port keeps the unit and masks bit 0.
  pub immediate_echo: u8,                           // 181
  pub echo_prompt_: Lisp_Object,                    // 192
}

// keyboard.h 228-235 (the `sie` member is inside `HAVE_X11 || HAVE_PGTK`).
//
// C's first member is the bitfield `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH`, which is the
// low 16 bits of the same bytes `ie` starts with; a Rust union cannot carry a bitfield, so the
// port declares the 16-bit view it is equal to on this target (`u16` at bit 0) next to the
// complete event.
#[repr(C)]
#[derive(Clone, Copy)]
pub union buffered_input_event
{
  pub kind: u16,                                    // keyboard.h 230 (`kind : EVENT_KIND_WIDTH`)
  pub ie: input_event,                              // 231
  /// keyboard.h 233: `struct selection_input_event sie;` - the record comes from a backend
  /// header C reaches through its own include, one per window system, and (keyboard.h 27-32)
  /// C's guard is `defined HAVE_X11 || defined HAVE_PGTK`.  The member is therefore carried
  /// behind **both** arms' macros: a PGTK unit resolves the record through the pgtkterm.h
  /// mirror, an X unit (`src/xterm.rs`'s selection cases are the readers) through xterm.h's,
  /// which that unit includes itself.  The member used to stand behind `have_pgtk` alone -
  /// that narrowing is what the xterm lane's arm `AX11` (`have_x_windows` + `have_x11`)
  /// measured, and neither macro is defined in the frozen configuration, so this widening is
  /// a no-op for every reading the tree carries today.
  #[cfg (any (feature = "have_x11", feature = "have_pgtk"))]
  pub sie: selection_input_event,                   // 233
}

/* Most code should use these functions to set Lisp fields in struct kboard.  */
// keyboard.h 188-228.
#[inline(always)]
pub fn kset_default_minibuffer_frame (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vdefault_minibuffer_frame_ = val; } }

#[inline(always)]
pub fn kset_defining_kbd_macro (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).defining_kbd_macro_ = val; } }

#[inline(always)]
pub fn kset_input_decode_map (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vinput_decode_map_ = val; } }

#[inline(always)]
pub fn kset_last_command (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vlast_command_ = val; } }

#[inline(always)]
pub fn kset_last_kbd_macro (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vlast_kbd_macro_ = val; } }

#[inline(always)]
pub fn kset_prefix_arg (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vprefix_arg_ = val; } }

#[inline(always)]
pub fn kset_system_key_alist (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vsystem_key_alist_ = val; } }

#[inline(always)]
pub fn kset_window_system (kb: *mut kboard, val: Lisp_Object)
{ unsafe { (*kb).Vwindow_system_ = val; } }

// keyboard.h 495-500: the prototype the inline calls.
unsafe extern "C" { pub fn kbd_buffer_store_buffered_event (e: *mut buffered_input_event,
                                                            hold_quit: *mut input_event); }

// keyboard.h 501-509.
#[inline(always)]
pub fn kbd_buffer_store_event_hold (event: *mut input_event, hold_quit: *mut input_event)
{
  // C's `static_assert` sits inside this inline and is what ties the union's modelling to
  // `input_event`; the port keeps it in the same place (PORTING.md 1.3).
  static_assert! (align_of::<input_event> () == align_of::<buffered_input_event> ()
                  && size_of::<input_event> () == size_of::<buffered_input_event> ());
  unsafe {
    kbd_buffer_store_buffered_event (event as *mut buffered_input_event, hold_quit);
  }
}

/* This is how to deal with multibyte text if HAVE_MULTILINGUAL_MENU isn't defined.  */
// keyboard.h 389-398: the live arm is `string_make_unibyte` (no GTK, no NS, no X i18n, no NTGUI).
#[inline(always)]
pub fn ENCODE_MENU_STRING (str: Lisp_Object) -> Lisp_Object
{
  #[cfg(feature = "have_ns")]
  { return unsafe { ENCODE_UTF_8(str) }; }
  #[cfg(not(feature = "have_ns"))]
  { unsafe { string_make_unibyte (str) } }
}

// keyboard.h 367-378 and 361-366: the slots of a menu-item vector and of a menu-bar pane.
// C's members carry no initialiser except the aliases at the top, so the values are the
// declaration order (the constant gate compares them).
pub type menu_item_idx = c_int;
pub const MENU_ITEMS_ITEM_NAME: menu_item_idx = 0;
pub const MENU_ITEMS_ITEM_ENABLE: menu_item_idx = 1;
pub const MENU_ITEMS_ITEM_VALUE: menu_item_idx = 2;
pub const MENU_ITEMS_ITEM_EQUIV_KEY: menu_item_idx = 3;
pub const MENU_ITEMS_ITEM_DEFINITION: menu_item_idx = 4;
pub const MENU_ITEMS_ITEM_TYPE: menu_item_idx = 5;
pub const MENU_ITEMS_ITEM_SELECTED: menu_item_idx = 6;
pub const MENU_ITEMS_ITEM_HELP: menu_item_idx = 7;
pub const MENU_ITEMS_ITEM_LENGTH: menu_item_idx = 8;
pub const MENU_ITEMS_PANE_NAME: menu_item_idx = 1;
pub const MENU_ITEMS_PANE_PREFIX: menu_item_idx = 2;
pub const MENU_ITEMS_PANE_LENGTH: menu_item_idx = 3;

// keyboard.h 297-321: `enum item_property_idx` — the slots of the menu-item vector
// `item_properties` points at.  C's members carry no initialiser, so their values are the
// declaration order; `ITEM_PROPERTY_MAX` is spelled `= ITEM_PROPERTY_ENABLE` as C does.
pub type item_property_idx = c_int;
pub const ITEM_PROPERTY_ITEM: item_property_idx = 0;
pub const ITEM_PROPERTY_NAME: item_property_idx = 1;
pub const ITEM_PROPERTY_DEF: item_property_idx = 2;
pub const ITEM_PROPERTY_MAP: item_property_idx = 3;
pub const ITEM_PROPERTY_TYPE: item_property_idx = 4;
pub const ITEM_PROPERTY_KEYEQ: item_property_idx = 5;
pub const ITEM_PROPERTY_SELECTED: item_property_idx = 6;
pub const ITEM_PROPERTY_HELP: item_property_idx = 7;
pub const ITEM_PROPERTY_ENABLE: item_property_idx = 8;
pub const ITEM_PROPERTY_MAX: item_property_idx = ITEM_PROPERTY_ENABLE;

/* Macros for dealing with lispy events.  */
// keyboard.h 404-450.  C writes these as macros over Lisp primitives; the port keeps the names
// and returns the same values.
#[inline(always)]
pub fn EVENT_HAS_PARAMETERS (event: Lisp_Object) -> bool { CONSP (event) }

#[inline(always)]
pub fn EVENT_HEAD (event: Lisp_Object) -> Lisp_Object
{ if EVENT_HAS_PARAMETERS (event) { XCAR (event) } else { event } }

#[inline(always)]
pub fn EVENT_START (event: Lisp_Object) -> Lisp_Object
{
  if EQ (EVENT_HEAD (event), Qtouchscreen_begin ())
     || EQ (EVENT_HEAD (event), Qtouchscreen_end ())
    { CDR_SAFE (CAR_SAFE (CDR_SAFE (event))) }
  else { CAR_SAFE (CDR_SAFE (event)) }
}

#[inline(always)]
pub fn EVENT_END (event: Lisp_Object) -> Lisp_Object
{ CAR_SAFE (CDR_SAFE (CDR_SAFE (event))) }

#[inline(always)]
pub fn EVENT_CLICK_COUNT (event: Lisp_Object) -> Lisp_Object
{ unsafe { Fnth (make_fixnum (2), event) } }

#[inline(always)]
pub fn EVENT_HEAD_KIND (event_head: Lisp_Object) -> Lisp_Object
{ unsafe { Fget (event_head, Qevent_kind ()) } }

#[inline(always)]
pub fn POSN_WINDOW (posn: Lisp_Object) -> Lisp_Object { CAR_SAFE (posn) }
#[inline(always)]
pub fn POSN_POSN (posn: Lisp_Object) -> Lisp_Object { CAR_SAFE (CDR_SAFE (posn)) }
#[inline(always)]
pub fn POSN_SET_POSN (posn: Lisp_Object, x: Lisp_Object) { XSETCAR (XCDR (posn), x) }
#[inline(always)]
pub fn POSN_WINDOW_POSN (posn: Lisp_Object) -> Lisp_Object
{ CAR_SAFE (CDR_SAFE (CDR_SAFE (posn))) }
#[inline(always)]
pub fn POSN_TIMESTAMP (posn: Lisp_Object) -> Lisp_Object
{ CAR_SAFE (CDR_SAFE (CDR_SAFE (CDR_SAFE (posn)))) }
#[inline(always)]
pub fn POSN_SCROLLBAR_PART (posn: Lisp_Object) -> Lisp_Object
{ unsafe { Fnth (make_fixnum (4), posn) } }
#[inline(always)]
pub fn POSN_STRING (posn: Lisp_Object) -> Lisp_Object
{ unsafe { Fnth (make_fixnum (4), posn) } }
#[inline(always)]
pub fn POSN_INBUFFER_P (posn: Lisp_Object) -> bool { NILP (POSN_STRING (posn)) }
#[inline(always)]
pub fn POSN_BUFFER_POSN (posn: Lisp_Object) -> Lisp_Object
{ unsafe { Fnth (make_fixnum (5), posn) } }

// keyboard.h 238-470: the file-scope objects (the C files define them).
unsafe extern "C" { pub static mut initial_kboard: *mut KBOARD; }                        // 238
// lisp.h 5081's `extern struct kboard *echo_kboard;` is declared in lisp.rs as `*mut c_void`
// (the tag is incomplete in a unit that includes only lisp.h — PORTING.md P-72); this comment
// records where the definition it points at lives.
unsafe extern "C" { pub static mut current_kboard: *mut KBOARD; }                        // 246
unsafe extern "C" { pub static mut num_input_events: uintmax_t; }                        // 257
unsafe extern "C" { pub static mut point_before_last_command_or_undo: ptrdiff_t; }       // 262
unsafe extern "C" { pub static mut buffer_before_last_command_or_undo: *mut buffer; }    // 267
unsafe extern "C" { pub static mut prev_buffer: *mut buffer; }                           // 269
unsafe extern "C" { pub static mut poll_suppress_count: c_int; }                         // 272
unsafe extern "C" { pub static mut this_command_keys: Lisp_Object; }                     // 278
unsafe extern "C" { pub static mut this_command_key_count: ptrdiff_t; }                  // 279
unsafe extern "C" { pub static mut internal_last_event_frame: Lisp_Object; }             // 286
unsafe extern "C" { pub static mut item_properties: Lisp_Object; }                       // 292
// keyboard.h 344-358's five menu objects are `src/menu.c`'s own definitions (its ledger's
// file-scope objects), so they sit in `pub mod keyboard_objs` + glob import: the owning
// unit's `#[unsafe(no_mangle)] pub static mut` shadows the glob (PORTING.md 0.1-10/P-21,
// the globals.rs `variables` shape) instead of colliding with it (E0428).
pub mod keyboard_objs {
  use super::*;
  unsafe extern "C" {
    pub static mut menu_items: Lisp_Object;                            // 344
    pub static mut menu_items_inuse: bool;                             // 348
    pub static mut menu_items_allocated: c_int;                        // 351
    pub static mut menu_items_used: c_int;                             // 354
    pub static mut menu_items_n_panes: c_int;                          // 358
    /* C 471 and C 531, keyboard.c's own definitions (`keyboard.rs`'s crate root, status 3.155).  */
    pub static mut timers_run: c_uint;                                 // 471
    pub static DEV_TTY: [c_char; 9];                                   // 531
  }
}
use keyboard_objs::*;
unsafe extern "C" { pub static mut input_available_clear_time: *mut timespec; }           // 453
unsafe extern "C" { pub static mut kbd_buffer: [buffered_input_event; KBD_BUFFER_SIZE as usize]; }  // 455
unsafe extern "C" { pub static mut kbd_fetch_ptr: *mut buffered_input_event; }            // 456
unsafe extern "C" { pub static mut kbd_store_ptr: *mut buffered_input_event; }            // 457
unsafe extern "C" { pub static mut ignore_mouse_drag_p: bool; }                           // 459
unsafe extern "C" { pub static mut real_this_command: Lisp_Object; }                      // 467
unsafe extern "C" { pub static mut quit_char: c_int; }                                    // 469
unsafe extern "C" { pub static mut input_was_pending: bool; }                             // 470
unsafe extern "C" { pub static mut dev_tty: *mut c_char; }                                // 529

// C: keyboard.h 511-529's prototypes — (not mirrored yet; they land with keyboard.c, batch E).

/* keyboard.h 483: `extern void temporarily_switch_to_single_kboard (struct frame *);` -
   callint.c's funcall-interactively calls it for a single-kboard configuration.  */
unsafe extern "C" { pub fn temporarily_switch_to_single_kboard (f: *mut c_void); }

/* keyboard.h 498: the event-queue entry kqueue.c uses.  The event pointer stays `*mut c_void`
   (the mirror cannot name `struct input_event` here, which termhooks.h owns).

   The read-descriptor entries that stood here in `pub mod keyboard_fd_decls` were process.h
   298-305's, not this header's: they moved to `include/process.rs` with the rest of that
   header's prototype block (status 3.78, the mirror's own heading was wrong too).  */

unsafe extern "C" {
  pub fn kbd_buffer_store_event (event: *mut c_void);
}

/* C 471 and C 531: this header's two file-scope objects.  The unit owns the definitions
   (`keyboard.rs`'s `keyboard_globals`: `timers_run` is keyboard.c 340's,
   `DEV_TTY` is keyboard.c 92-95's), so these are declarations only - the defining unit's own item shadows a
   glob import (PORTING.md 0.1-10 / P-21), which is why `keyboard.rs` itself compiles with
   both in scope.  They are here because status 3.143's tool fix made the extractor see them:
   its `GLOBAL_LINE` split `extern unsigned int timers_run;` after `unsigned`, so the object
   never reached the gate's C side and both mirrors looked complete without them.  */
/* C 471's `timers_run` and C 531's `DEV_TTY` (and C 529's `dev_tty` above) are declared in
   `pub mod keyboard_objs` below, next to the five menu objects: their definitions are crate-root
   items in the *owning* unit (`keyboard.rs`), so the declaration has to arrive through a glob
   import for the definition to shadow it (PORTING.md 0.1-10/P-21) - a crate-root declaration
   collides with them instead, which is what `build-tu src/keyboard.c` reported as
   `E0428: the name timers_run is defined multiple times` (status 3.155).  */

/* C 387-524: the prototypes this header declares for functions
   other units define.  They stand in a module + glob import so a unit that writes the same
   prototype at its use site shadows the import instead of colliding with it (P-297, the P-39
   shape); a prototype carries the gate of the records it names.  */
pub mod keyboard_fn_decls {
  use super::*;

  unsafe extern "C" {
    /// C: keyboard.h 387: `extern void unuse_menu_items (void);`
    pub fn unuse_menu_items ();
    /// C: keyboard.h 463: `extern Lisp_Object parse_modifiers (Lisp_Object);`
    pub fn parse_modifiers (item: Lisp_Object) -> Lisp_Object;
    /// C: keyboard.h 464: `extern Lisp_Object reorder_modifiers (Lisp_Object);`
    pub fn reorder_modifiers (event: Lisp_Object) -> Lisp_Object;
    /// C: keyboard.h 465: `extern int parse_solitary_modifier (Lisp_Object symbol);`
    pub fn parse_solitary_modifier (symbol: Lisp_Object) -> c_int;
    /// C: keyboard.h 476: `extern bool parse_menu_item (Lisp_Object, int);`
    pub fn parse_menu_item (item: Lisp_Object, in_menubar: c_int) -> bool;
    /// C: keyboard.h 478: `extern void init_raw_keybuf_count (void);`
    pub fn init_raw_keybuf_count ();
    /// C: keyboard.h 479: `extern KBOARD *allocate_kboard (Lisp_Object);`
    pub fn allocate_kboard (type_: Lisp_Object) -> *mut kboard;
    /// C: keyboard.h 480: `extern void delete_kboard (KBOARD *);`
    pub fn delete_kboard (kb: *mut KBOARD);
    /// C: keyboard.h 481: `extern void not_single_kboard_state (KBOARD *);`
    pub fn not_single_kboard_state (kb: *mut KBOARD);
    /// C: keyboard.h 482: `extern void push_kboard (struct kboard *);`
    pub fn push_kboard (k: *mut kboard);
    /// C: keyboard.h 484: `extern void pop_kboard (void);`
    pub fn pop_kboard ();
    /// C: keyboard.h 487: `extern void start_polling (void);`
    pub fn start_polling ();
    /// C: keyboard.h 488: `extern void stop_polling (void);`
    pub fn stop_polling ();
    /// C: keyboard.h 489: `extern void set_poll_suppress_count (int);`
    pub fn set_poll_suppress_count (count: c_int);
    /// C: keyboard.h 490: `extern int gobble_input (void);`
    pub fn gobble_input () -> c_int;
    /// C: keyboard.h 491: `extern bool input_polling_used (void);`
    pub fn input_polling_used () -> bool;
    /// C: keyboard.h 492: `extern void clear_input_pending (void);`
    pub fn clear_input_pending ();
    /// C: keyboard.h 493: `extern bool requeued_command_events_pending_p (void);`
    pub fn requeued_command_events_pending_p () -> bool;
    /// C: keyboard.h 494: `extern void bind_polling_period (int);`
    pub fn bind_polling_period (n: c_int);
    /// C: keyboard.h 495: `extern int make_ctrl_char (int) ATTRIBUTE_CONST;`
    pub fn make_ctrl_char (c: c_int) -> c_int;
    /// C: keyboard.h 496: `extern void stuff_buffered_input (Lisp_Object);`
    pub fn stuff_buffered_input (stuff: Lisp_Object);
    /// C: keyboard.h 497: `extern void clear_waiting_for_input (void);`
    pub fn clear_waiting_for_input ();
    /// C: keyboard.h 498: `extern void swallow_events (bool);`
    pub fn swallow_events (do_display: bool);
    /// C: keyboard.h 499: `extern bool lucid_event_type_list_p (Lisp_Object);`
    pub fn lucid_event_type_list_p (object: Lisp_Object) -> bool;
    /// C: keyboard.h 512: `extern void poll_for_input_1 (void);`
    pub fn poll_for_input_1 ();
    /// C: keyboard.h 513: `extern void show_help_echo (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object);`
    pub fn show_help_echo (string: Lisp_Object, window: Lisp_Object, object: Lisp_Object, object_pos: Lisp_Object);
    /// C: keyboard.h 515: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, ptrdiff_t);`
    pub fn gen_help_event (help: Lisp_Object, frame: Lisp_Object, window: Lisp_Object, object: Lisp_Object, pos: ptrdiff_t);
    /// C: keyboard.h 517: `extern void kbd_buffer_store_help_event (Lisp_Object, Lisp_Object);`
    pub fn kbd_buffer_store_help_event (frame: Lisp_Object, help: Lisp_Object);
    /// C: keyboard.h 518: `extern Lisp_Object menu_item_eval_property (Lisp_Object);`
    pub fn menu_item_eval_property (sexpr: Lisp_Object) -> Lisp_Object;
    /// C: keyboard.h 519: `extern bool kbd_buffer_events_waiting (void);`
    pub fn kbd_buffer_events_waiting () -> bool;
    /// C: keyboard.h 524: `extern void mark_kboards (void);`
    pub fn mark_kboards ();
  }
}
use keyboard_fn_decls::*;
