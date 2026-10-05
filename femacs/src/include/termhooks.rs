// femacs: port of src/termhooks.h @ fd7afced403.

// WIP mirror (PORTING.md P-66): `struct input_event` and the two enumerations it carries,
// which is the slice `keyboard.h` needs (its `union buffered_input_event` embeds the event
// by value).  `struct terminal` and the `FRAME_*`/`TERMINAL_*` macros follow in the next
// slice: they name `struct frame`, `struct window`, `struct glyph`,
// `struct redisplay_interface` and `struct kboard`, none of which are mirrored yet, so they
// need P-70 stand-ins (measured below).

// termhooks.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_TERMHOOKS_H: bool = true;

// termhooks.h 72-355: `enum event_kind`.  C's members carry no initialiser, so their values
// are the declaration order (the constant gate compares them).
pub type event_kind = c_int;
pub const NO_EVENT: event_kind = 0;
pub const ASCII_KEYSTROKE_EVENT: event_kind = 1;
pub const MULTIBYTE_CHAR_KEYSTROKE_EVENT: event_kind = 2;
pub const NON_ASCII_KEYSTROKE_EVENT: event_kind = 3;
pub const TIMER_EVENT: event_kind = 4;
pub const MOUSE_CLICK_EVENT: event_kind = 5;
pub const WHEEL_EVENT: event_kind = 6;
pub const HORIZ_WHEEL_EVENT: event_kind = 7;
pub const SCROLL_BAR_CLICK_EVENT: event_kind = 8;
pub const HORIZONTAL_SCROLL_BAR_CLICK_EVENT: event_kind = 9;
pub const SELECTION_REQUEST_EVENT: event_kind = 10;
pub const SELECTION_CLEAR_EVENT: event_kind = 11;
pub const DELETE_WINDOW_EVENT: event_kind = 12;
pub const MENU_BAR_EVENT: event_kind = 13;
pub const ICONIFY_EVENT: event_kind = 14;
pub const DEICONIFY_EVENT: event_kind = 15;
pub const MENU_BAR_ACTIVATE_EVENT: event_kind = 16;
pub const DRAG_N_DROP_EVENT: event_kind = 17;
pub const USER_SIGNAL_EVENT: event_kind = 18;
pub const HELP_EVENT: event_kind = 19;
pub const TAB_BAR_EVENT: event_kind = 20;
pub const TOOL_BAR_EVENT: event_kind = 21;
pub const FOCUS_IN_EVENT: event_kind = 22;
pub const FOCUS_OUT_EVENT: event_kind = 23;
pub const MOVE_FRAME_EVENT: event_kind = 24;
pub const SELECT_WINDOW_EVENT: event_kind = 25;
pub const SAVE_SESSION_EVENT: event_kind = 26;
pub const CONFIG_CHANGED_EVENT: event_kind = 27;
pub const SLEEP_EVENT: event_kind = 28;
pub const PREEDIT_TEXT_EVENT: event_kind = 29;
pub const TOUCH_END_EVENT: event_kind = 30;
pub const TOUCHSCREEN_UPDATE_EVENT: event_kind = 31;
pub const TOUCHSCREEN_BEGIN_EVENT: event_kind = 32;
pub const TOUCHSCREEN_END_EVENT: event_kind = 33;
pub const PINCH_EVENT: event_kind = 34;
pub const MONITORS_CHANGED_EVENT: event_kind = 35;
pub const TOOLKIT_THEME_CHANGED_EVENT: event_kind = 36;

// termhooks.h 34-51: `enum scroll_bar_part`.
pub type scroll_bar_part = c_int;
pub const scroll_bar_nowhere: scroll_bar_part = 0;
pub const scroll_bar_above_handle: scroll_bar_part = 1;
pub const scroll_bar_handle: scroll_bar_part = 2;
pub const scroll_bar_below_handle: scroll_bar_part = 3;
pub const scroll_bar_up_arrow: scroll_bar_part = 4;
pub const scroll_bar_down_arrow: scroll_bar_part = 5;
pub const scroll_bar_to_top: scroll_bar_part = 6;
pub const scroll_bar_to_bottom: scroll_bar_part = 7;
pub const scroll_bar_end_scroll: scroll_bar_part = 8;
pub const scroll_bar_move_ratio: scroll_bar_part = 9;
pub const scroll_bar_before_handle: scroll_bar_part = 10;
pub const scroll_bar_horizontal_handle: scroll_bar_part = 11;
pub const scroll_bar_after_handle: scroll_bar_part = 12;
pub const scroll_bar_left_arrow: scroll_bar_part = 13;
pub const scroll_bar_right_arrow: scroll_bar_part = 14;
pub const scroll_bar_to_leftmost: scroll_bar_part = 15;
pub const scroll_bar_to_rightmost: scroll_bar_part = 16;

// termhooks.h 359-360: the width of the `kind` bitfield `input_event` and
// `buffered_input_event` share.
pub const EVENT_KIND_WIDTH: c_int = 16;

/* Bits in the modifiers member of the input_event structure.  */
// termhooks.h 434-459: the modifier bits.  C's enum has type `int`, and the last six
// members are aliases of character.h's `CHAR_*` constants, so the port keeps the same
// expression C writes and the constant gate compares its tokens.
pub const up_modifier: c_int = 1;
pub const down_modifier: c_int = 2;
pub const drag_modifier: c_int = 4;
pub const click_modifier: c_int = 8;
pub const double_modifier: c_int = 16;
pub const triple_modifier: c_int = 32;
pub const alt_modifier: c_int = CHAR_ALT;
pub const super_modifier: c_int = CHAR_SUPER;
pub const hyper_modifier: c_int = CHAR_HYPER;
pub const shift_modifier: c_int = CHAR_SHIFT;
pub const ctrl_modifier: c_int = CHAR_CTL;
pub const meta_modifier: c_int = CHAR_META;


/* Input events.  */
// termhooks.h 373-412.  Measured (sizeof 64, align 8): `kind : EVENT_KIND_WIDTH` and
// `part : 16` share the 4-byte unit at offset 0 (`34 12 78 56` for kind 0x1234 / part
// 0x5678), `code` at 4, `modifiers` at 8, `x` at 16, `y` at 24, `timestamp` at 32,
// `frame_or_window` at 40, `arg` at 48 and `device` at 56.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct input_event
{
  pub bits: u32,                       // kind:16 | part:16 (PORTING.md 1.3)
  pub code: c_uint,                    // termhooks.h 379
  pub modifiers: c_uint,               // 382
  pub x: Lisp_Object,                  // 384
  pub y: Lisp_Object,
  pub timestamp: Time,                 // 387 (`Time` is systime.h's, `unsigned long` here)
  pub frame_or_window: Lisp_Object,    // 389
  pub arg: Lisp_Object,                // 390
  pub device: Lisp_Object,             // 395
}

// `kind` is the low half of the unit and `part` the high half (measured above).
pub const INPUT_EVENT_KIND_MASK: u32 = 0xFFFF;
pub const INPUT_EVENT_PART_SHIFT: u32 = 16;

// C: `#define EVENT_INIT(event) ((event) = (struct input_event) {.device = Qt})`
// termhooks.h 414.  C's compound literal zero-fills every member it does not name, so only
// `device` is `Qt` and the two Lisp_Objects C leaves are nil (NIL_IS_ZERO).
#[inline(always)]
pub fn EVENT_INIT (event: *mut input_event)
{
  unsafe {
    *event = input_event {
      bits: 0,
      code: 0,
      modifiers: 0,
      x: lisp_h_Qnil (),
      y: lisp_h_Qnil (),
      timestamp: 0,
      frame_or_window: lisp_h_Qnil (),
      arg: lisp_h_Qnil (),
      device: Qt (),
    };
  }
}

// termhooks.h 57-68: `enum output_method`.
pub type output_method = c_int;
pub const output_initial: output_method = 0;
pub const output_termcap: output_method = 1;
pub const output_x_window: output_method = 2;
pub const output_msdos_raw: output_method = 3;
pub const output_w32: output_method = 4;
pub const output_ns: output_method = 5;
pub const output_pgtk: output_method = 6;
pub const output_haiku: output_method = 7;
pub const output_android: output_method = 8;

// termhooks.h 533-542.  Every member's pointee is incomplete in *this* header too (the platform
// headers that define them are included by the .c files, not here — `termhooks.h` includes only
// lisp.h, dispextern.h and systime.h), so the port keeps the untyped pointer it uses for
// incomplete targets (PORTING.md P-72 rule ③).  The union is 8 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub union display_info
{
  // Each member's pointee is incomplete in this header (termchar.h/xterm.h/… are included by the
  // platform .c files, not by termhooks.h), so the port keeps the untyped pointer it uses for
  // incomplete targets — PORTING.md P-72 rule ③.
  pub tty: *mut c_void,        // C: `struct tty_display_info *tty;`
  pub x: *mut c_void,          // C: `struct x_display_info *x;`
  pub w32: *mut c_void,        // C: `struct w32_display_info *w32;`
  pub ns: *mut c_void,         // C: `struct ns_display_info *ns;`
  pub pgtk: *mut c_void,       // C: `struct pgtk_display_info *pgtk;`
  pub haiku: *mut c_void,      // C: `struct haiku_display_info *haiku;`
  pub android: *mut c_void,    // C: `struct android_display_info *android;`
}

/* Each terminal device is represented by one of these structures.  */
// termhooks.h 466-888.  Measured: sizeof 520, alignof 8 — the offsets of all 66 non-union
// members are asserted (header 0, the four Lisp_Objects 8…32, next_terminal 40, id 48,
// reference_count 52, type 56, name 64, kboard 72, display_info 80, terminal_coding 88,
// keyboard_coding 96, rif 104, then the hook table from 112 to 512).
//
// Every `struct frame *`, `struct window *` and `struct glyph *` parameter is an *incomplete*
// pointer in this header (frame.h/window.h are not included by termhooks.h), which is why the
// port spells those as `*mut c_void`; `struct glyph *` and `struct redisplay_interface *` are
// complete in C's view here but their mirrors are not written yet (PORTING.md P-66's display
// chain), so they carry the same degradation and the `/// C:` note until they land.
#[repr(C)]
pub struct terminal
{
  pub header: vectorlike_header,                   // termhooks.h 478

  /* The terminal's parameter alist (frame parameters), and the list of charsets it can
     display.  */
  pub param_alist: Lisp_Object,                    // 481
  pub charset_list: Lisp_Object,                   // 486
  /* The Lisp list of selection alists.  */
  pub Vselection_alist: Lisp_Object,               // 502
  /* The glyph code table, a char-table.  */
  pub glyph_code_table: Lisp_Object,               // 507

  /* For use by the display engine: the next terminal in the chain.  */
  pub next_terminal: *mut terminal,                // 513
  /* The terminal's id, and how many frames reference it.  */
  pub id: c_int,                                   // 516
  pub reference_count: c_int,                      // 519
  /* What kind of device this is.  */
  pub r#type: output_method,                       // 522 (C spells it `type`)
  /* The name of the terminal.  */
  pub name: *mut c_char,                           // 527

  /* The terminal's keyboard, when it has one of its own.  */
  pub kboard: *mut c_void,                         /// C: `struct kboard *kboard;` (keyboard.h)

  /* Device-dependent, display-specific data.  */
  pub display_info: display_info,                  // 542

  /* The coding systems used for terminal output and keyboard input.  */
  pub terminal_coding: *mut c_void,               // 549 (`coding.rs` 133's record; P-72 rule 3 stand-in)
  pub keyboard_coding: *mut c_void,               // 554 (same stand-in)

  /* The terminal's redisplay interface (dispextern.h).  */
  pub rif: *mut c_void,                            /// C: `struct redisplay_interface *rif;`

  /* A table of hooks for terminal-specific display actions.  Every one may be NULL.  */
  pub cursor_to_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int)>,          // 564
  pub raw_cursor_to_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int)>,      // 565
  pub clear_to_end_hook: Option<unsafe extern "C" fn (*mut c_void)>,                     // 567
  pub clear_frame_hook: Option<unsafe extern "C" fn (*mut c_void)>,                      // 568
  pub clear_end_of_line_hook: Option<unsafe extern "C" fn (*mut c_void, c_int)>,         // 569
  pub ins_del_lines_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int)>,      // 571
  pub insert_glyphs_hook: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, c_int)>, // 573
  pub write_glyphs_hook: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, c_int)>,  // 574
  pub delete_glyphs_hook: Option<unsafe extern "C" fn (*mut c_void, c_int)>,             // 575
  pub ring_bell_hook: Option<unsafe extern "C" fn (*mut c_void)>,                        // 577
  pub toggle_invisible_pointer_hook:
      Option<unsafe extern "C" fn (*mut c_void, bool)>,                                  // 578
  pub reset_terminal_modes_hook: Option<unsafe extern "C" fn (*mut terminal)>,           // 580
  pub set_terminal_modes_hook: Option<unsafe extern "C" fn (*mut terminal)>,             // 581
  pub update_begin_hook: Option<unsafe extern "C" fn (*mut c_void)>,                     // 583
  pub update_end_hook: Option<unsafe extern "C" fn (*mut c_void)>,                       // 584
  pub set_terminal_window_hook: Option<unsafe extern "C" fn (*mut c_void, c_int)>,       // 585

  /* Hook to define a color, and to ask for a frame's background.  */
  pub defined_color_hook: Option<unsafe extern "C" fn (*mut c_void, *const c_char,
                                                        *mut Emacs_Color, bool, bool)
                                                       -> bool>,                         // 596
  pub query_frame_background_color: Option<unsafe extern "C" fn (*mut c_void,
                                                                 *mut Emacs_Color)>,     // 613

  // termhooks.h 615-621: C's `#if defined (HAVE_X_WINDOWS) || defined (HAVE_NTGUI) ||
  // defined (HAVE_PGTK) || defined (HAVE_ANDROID)` arm.
  #[cfg (any (feature = "have_x_windows", feature = "have_ntgui", feature = "have_pgtk",
              feature = "have_android"))]
  pub query_colors: Option<unsafe extern "C" fn (*mut c_void, *mut Emacs_Color,
                                                  c_int)>,                               // 620

  /* Hook to get the position of the mouse.  */
  pub mouse_position_hook: Option<unsafe extern "C" fn (*mut *mut c_void, c_int,
                                                        *mut Lisp_Object,
                                                        *mut scroll_bar_part,
                                                        *mut Lisp_Object, *mut Lisp_Object,
                                                        *mut Time)>,                     // 642

  /* Hooks for focusing and raising frames.  */
  pub get_focus_frame: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,        // 645
  pub focus_frame_hook: Option<unsafe extern "C" fn (*mut c_void, bool)>,                // 648
  pub frame_rehighlight_hook: Option<unsafe extern "C" fn (*mut c_void)>,                // 653
  pub frame_raise_lower_hook: Option<unsafe extern "C" fn (*mut c_void, bool)>,          // 665
  pub frame_visible_invisible_hook: Option<unsafe extern "C" fn (*mut c_void, bool)>,    // 669
  pub fullscreen_hook: Option<unsafe extern "C" fn (*mut c_void)>,                       // 674
  pub iconify_frame_hook: Option<unsafe extern "C" fn (*mut c_void)>,                    // 677

  /* Hooks to resize and move a frame, and to set its alpha.  */
  pub set_window_size_hook: Option<unsafe extern "C" fn (*mut c_void, bool, c_int, c_int)>, // 684
  pub set_window_size_and_position_hook:
      Option<unsafe extern "C" fn (*mut c_void, c_int, c_int)>,                          // 689
  pub set_frame_offset_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int)>, // 698
  pub set_frame_alpha_hook: Option<unsafe extern "C" fn (*mut c_void)>,                  // 701

  /* Hooks to set a new font, the icon, and the frame's name.  */
  pub set_new_font_hook: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object,
                                                      c_int) -> Lisp_Object>,            // 705
  pub set_bitmap_icon_hook: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object) -> bool>, // 708
  pub implicit_set_name_hook: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object,
                                                           Lisp_Object)>,                // 713

  /* Hooks to show a menu or a dialog.  */
  pub menu_show_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int,
                                                   Lisp_Object,
                                                   *mut *const c_char) -> Lisp_Object>,  // 717
  #[cfg (feature = "have_ext_menu_bar")]
  pub activate_menubar_hook: Option<unsafe extern "C" fn (*mut c_void)>,                 // 721
  pub popup_dialog_hook: Option<unsafe extern "C" fn (*mut c_void, Lisp_Object,
                                                      Lisp_Object) -> Lisp_Object>,      // 726

  /* Hooks to change the tab bar's and tool bar's height.  */
  pub change_tab_bar_height_hook: Option<unsafe extern "C" fn (*mut c_void, c_int)>,     // 729
  pub change_tool_bar_height_hook: Option<unsafe extern "C" fn (*mut c_void, c_int)>,    // 732

  /* Hooks for scroll bars.  */
  pub set_vertical_scroll_bar_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int,
                                                                 c_int)>,                // 763
  pub set_horizontal_scroll_bar_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int,
                                                                   c_int)>,              // 773
  pub set_scroll_bar_default_width_hook: Option<unsafe extern "C" fn (*mut c_void)>,     // 776
  pub set_scroll_bar_default_height_hook: Option<unsafe extern "C" fn (*mut c_void)>,    // 779
  pub condemn_scroll_bars_hook: Option<unsafe extern "C" fn (*mut c_void)>,              // 801
  pub redeem_scroll_bar_hook: Option<unsafe extern "C" fn (*mut c_void)>,                // 805
  pub judge_scroll_bars_hook: Option<unsafe extern "C" fn (*mut c_void)>,                // 818

  /* Hook to read input from the terminal's device.  */
  pub read_socket_hook: Option<unsafe extern "C" fn (*mut terminal,
                                                     *mut input_event) -> c_int>,        // 832

  pub frame_up_to_date_hook: Option<unsafe extern "C" fn (*mut c_void)>,                 // 835
  pub buffer_flipping_unblocked_hook: Option<unsafe extern "C" fn (*mut c_void)>,        // 840
  pub get_string_resource_hook: Option<unsafe extern "C" fn (*mut c_void, *const c_char,
                                                             *const c_char)
                                                -> *const c_char>,                       // 846
  // termhooks.h 849-853: the "Image hooks" pair.  C's parameter types have no mirror here
  // (`Emacs_Pixmap` is xterm.h's `typedef XImage *`, `Display_Info` xterm.h's display record),
  // so the parameters stay untyped pointers (P-72 rule 3).
  #[cfg (feature = "have_window_system")]
  pub free_pixmap: Option<unsafe extern "C" fn (*mut c_void, *mut c_void)>,              // 851
  pub delete_frame_hook: Option<unsafe extern "C" fn (*mut c_void)>,                     // 859
  pub delete_terminal_hook: Option<unsafe extern "C" fn (*mut terminal)>,                // 871
  pub toolkit_position_hook: Option<unsafe extern "C" fn (*mut c_void, c_int, c_int,
                                                          *mut bool, *mut bool)>,        // 878
  #[cfg (feature = "have_window_system")]
  pub any_grab_hook: Option<unsafe extern "C" fn (*mut c_void) -> bool>,                 // 884
}

// termhooks.h 888-909.
#[inline(always)]
pub fn TERMINALP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_TERMINAL) }

#[inline(always)]
pub fn XTERMINAL (a: Lisp_Object) -> *mut terminal
{
  eassert! (TERMINALP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, terminal)
}

/* Most code should use these functions to set Lisp fields in struct
   terminal.  */
#[inline(always)]
pub fn tset_charset_list (t: *mut terminal, val: Lisp_Object)
{ unsafe { (*t).charset_list = val; } }

#[inline(always)]
pub fn tset_selection_alist (t: *mut terminal, val: Lisp_Object)
{ unsafe { (*t).Vselection_alist = val; } }

// termhooks.h 927-928: the two `TERMINAL_*` accessors that need only `struct terminal`
// (the rest of the `FRAME_*`/`TERMINAL_*` family dereferences `struct frame`).
#[inline(always)]
pub fn TERMINAL_TERMINAL_CODING (d: *mut terminal) -> *mut c_void
{ unsafe { (*d).terminal_coding } }
#[inline(always)]
pub fn TERMINAL_KEYBOARD_CODING (d: *mut terminal) -> *mut c_void
{ unsafe { (*d).keyboard_coding } }

/* Chain of all terminal devices currently in use.  */
unsafe extern "C" { pub static mut terminal_list: *mut terminal; }        // termhooks.h 915

/* The initial terminal device, created by initial_term_init.  */
unsafe extern "C" { pub static mut initial_terminal: *mut terminal; }     // termhooks.h 971
// termhooks.h 920-921: both read the `tty_display_info` that `FRAME_TTY` (termchar.h 243) hands
// back, so they stay macros — `FRAME_TTY` is one, and a `fn` cannot call it (PORTING.md P-33).
// `scroll_region_ok` is the `bool_bf` in `tty_display_info.bits_caps` (the TTY_* masks), which
// is why that one token is exempted in docs/ports/macro-body-notes.tsv (the FRAME_GARBAGED_P
// precedent); `scroll_region_cost` is a plain field.
macro_rules! FRAME_SCROLL_REGION_OK {
  ($f:expr) => { (unsafe { (*FRAME_TTY! ($f)).bits_caps } & TTY_SCROLL_REGION_OK) != 0 };
}
macro_rules! FRAME_SCROLL_REGION_COST {
  ($f:expr) => { (*FRAME_TTY! ($f)).scroll_region_cost };
}

// termhooks.h 932/935: `FRAME_TERMINAL` and `TERMINAL_ACTIVE_P`.  Both dereference a type
// this header cannot name (`struct frame`, `struct tty_display_info`), so they keep the macro
// shape — the name resolves at the use site, where those mirrors are in scope (a `fn` here
// would be E0412 for every unit that includes termhooks.rs without frame.rs, e.g.
// src/callint.rs).  `TERMINAL_ACTIVE_P`'s `(d)->display_info.tty->input` reads the union
// member the mirror spells `*mut c_void` (P-72 rule ③) and casts it once.
macro_rules! FRAME_TERMINAL {
  ($f:expr) => { unsafe { (*$f).terminal } };
}
macro_rules! TERMINAL_ACTIVE_P {
  ($d:expr) => {
    unsafe { (*$d).r#type != output_termcap && (*$d).r#type != output_msdos_raw
	     || ! (*((*$d).display_info.tty as *mut tty_display_info)).input.is_null () }
  };
}


// termhooks.h 917-922: `FRAME_MUST_WRITE_SPACES`, `FRAME_LINE_INS_DEL_OK`,
// `FRAME_CHAR_INS_DEL_OK` and `FRAME_MEMORY_BELOW_FRAME`.  Each dereferences the
// `tty_display_info` `FRAME_TTY` (termchar.h 243) hands back, whose four `bool_bf : 1`
// members live in the one-byte `bits_caps` (termchar.rs's TTY_* masks), so they stay macros
// expanded at their use site — a `fn` here would be E0412 for every unit that includes
// termhooks.rs without termchar.rs (the FRAME_SCROLL_REGION_OK shape above).  That member
// token is what docs/ports/macro-body-notes.tsv exempts, as it does for `scroll_region_ok`.
macro_rules! FRAME_MUST_WRITE_SPACES {
  ($f:expr) => { (unsafe { (*FRAME_TTY! ($f)).bits_caps } & TTY_MUST_WRITE_SPACES) != 0 };
}
macro_rules! FRAME_LINE_INS_DEL_OK {
  ($f:expr) => { (unsafe { (*FRAME_TTY! ($f)).bits_caps } & TTY_LINE_INS_DEL_OK) != 0 };
}
macro_rules! FRAME_CHAR_INS_DEL_OK {
  ($f:expr) => { (unsafe { (*FRAME_TTY! ($f)).bits_caps } & TTY_CHAR_INS_DEL_OK) != 0 };
}
macro_rules! FRAME_MEMORY_BELOW_FRAME {
  ($f:expr) => { (unsafe { (*FRAME_TTY! ($f)).bits_caps } & TTY_MEMORY_BELOW_FRAME) != 0 };
}

// termhooks.h 924-925/930: `FRAME_TERMINAL_CODING`, `FRAME_KEYBOARD_CODING` and `FRAME_RIF`.
// Their parameter is `struct frame *`, which this header never names (frame.h is not in its
// include chain), so they stay macros that resolve at the use site — a `fn` here would be
// E0412 for every unit that includes termhooks.rs without frame.rs (the §3.54 rule that
// caught this very row; `FRAME_SCROLL_REGION_OK` above is the same shape).
macro_rules! FRAME_TERMINAL_CODING {
  ($f:expr) => { (*(*$f).terminal).terminal_coding };
}
macro_rules! FRAME_KEYBOARD_CODING {
  ($f:expr) => { (*(*$f).terminal).keyboard_coding };
}
macro_rules! FRAME_RIF { ($f:expr) => { (*(*$f).terminal).rif }; }

// termhooks.h 983-985: `cursorX`/`cursorY` are `curX`/`curY` on this target (the WINDOWSNT
// arm is dead), and both of those are cm.h macros, so these two stay macros and resolve at
// their use site exactly as C's do.
macro_rules! cursorX { ($t:expr) => { curX! ($t) }; }
macro_rules! cursorY { ($t:expr) => { curY! ($t) }; }

// C: termhooks.h 961-989's prototypes (`decode_live_terminal`, `decode_tty_terminal`,
// `get_named_terminal`, `create_terminal`, `delete_terminal`, `delete_terminal_internal`,
// `terminal_glyph_code`, `encode_terminal_code`, `tty_hide_cursor`, `tty_show_cursor`) and
// the `#ifdef HAVE_WINDOW_SYSTEM`/`#ifdef HAVE_GPM`/`#ifdef WINDOWSNT` arms are not mirrored
// yet; each is declared at its use site (term.rs).
