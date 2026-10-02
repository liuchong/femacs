// femacs: port of src/termhooks.h @ fd7afced403 (GNU Emacs 32.0.50).

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

/* Input events.  */
// termhooks.h 373-412.  Measured (sizeof 64, align 8): `kind : EVENT_KIND_WIDTH` and
// `part : 16` share the 4-byte unit at offset 0 (`34 12 78 56` for kind 0x1234 / part
// 0x5678), `code` at 4, `modifiers` at 8, `x` at 16, `y` at 24, `timestamp` at 32,
// `frame_or_window` at 40, `arg` at 48 and `device` at 56.
#[repr(C)]
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

// C: termhooks.h 54-355's `enum output_method` and the remaining `event_kind` documentation,
// 414-886's modifier bits and terminal-adapter tables, and 888-1010's `struct terminal`, the
// `FRAME_*`/`TERMINAL_*` macros, `TERMINALP`/`XTERMINAL`, `tset_charset_list`/
// `tset_selection_alist`, `cursorX`/`cursorY` and the two file-scope objects — (not mirrored
// yet; `struct terminal` needs P-70 stand-ins for `frame`, `window`, `glyph`,
// `redisplay_interface`, `kboard` and the seven `*_display_info` types, and `cursorX`/`cursorY`
// call `curX`/`curY` from termchar.h).
