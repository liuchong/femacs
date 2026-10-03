// femacs: port of src/frame.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): the `frame` structure and its flag units.  `frame.h` includes
// `dispextern.h` and `termhooks.h` (so C sees `struct face_cache`, `struct glyph_matrix`,
// `struct glyph_pool` and `struct terminal` complete here); of those, only `terminal` has a
// mirror so far, so the three display ones carry the P-72 rule ③ degradation until the display
// chain reaches them.  The 139 `FRAME_*` macros, 37 inlines and 7 file-scope objects come next.
//
// `tools/check-cond-eval.py` reports zero disagreements for this header, so the branch set is
// the frozen configuration's (`HAVE_WINDOW_SYSTEM`, `HAVE_NEXTSTEP`, the `USE_*` toolkits and
// every platform `*_P` predicate are `#undef`/false here).

// frame.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_FRAME_H: bool = true;

/* Device-dependent, frame-local auxiliary data used for displaying
   the contents.  */
// frame.h 653-663.  The tag is `output_data` and so is the member's name; the port keeps both.
// All seven pointees live in headers `frame.h` does not include (termchar.h, xterm.h, …), so C
// sees them as incomplete tags and the port keeps the untyped pointer (P-72 rule ③).  The union
// is 8 bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub union output_data
{
  pub tty: *mut c_void,        // C: `struct tty_output *tty;`
  pub x: *mut c_void,          // C: `struct x_output *x;`
  pub w32: *mut c_void,        // C: `struct w32_output *w32;`
  pub ns: *mut c_void,         // C: `struct ns_output *ns;`
  pub pgtk: *mut c_void,       // C: `struct pgtk_output *pgtk;`
  pub haiku: *mut c_void,      // C: `struct haiku_output *haiku;`
  pub android: *mut c_void,    // C: `struct android_output *android;`
}

/* The structure that describes a frame.  */
// frame.h 145-680.  Measured: sizeof 568, alignof 8.  It holds scalars, Lisp_Objects and
// pointers; the only aggregates by value are the vectorlike header, two
// `enum text_cursor_kinds` values and the eight-byte flag run at 240-247.  Every field's
// offset is asserted against the C compiler.
#[repr(C)]
pub struct frame
{
  pub header: vectorlike_header,
  pub name: Lisp_Object,
  pub icon_name: Lisp_Object,
  pub title: Lisp_Object,
  pub parent_frame: Lisp_Object,
  pub last_mouse_device: Lisp_Object,
  pub focus_frame: Lisp_Object,
  pub root_window: Lisp_Object,
  pub selected_window: Lisp_Object,
  pub old_selected_window: Lisp_Object,
  pub minibuffer_window: Lisp_Object,
  pub param_alist: Lisp_Object,
  pub scroll_bars: Lisp_Object,
  pub condemned_scroll_bars: Lisp_Object,
  pub menu_bar_items: Lisp_Object,
  pub face_hash_table: Lisp_Object,
  pub menu_bar_vector: Lisp_Object,
  pub buffer_predicate: Lisp_Object,
  pub buffer_list: Lisp_Object,
  pub buried_buffer_list: Lisp_Object,
  pub tool_bar_position: Lisp_Object,
  pub tab_bar_items: Lisp_Object,
  pub tool_bar_items: Lisp_Object,
  pub face_cache: *mut c_void,   /// C: `struct face_cache *face_cache`
  pub id: EMACS_UINT,
  pub last_tab_bar_item: c_int,
  pub menu_bar_items_used: c_int,
  pub current_pool: *mut c_void,   /// C: `struct glyph_pool *current_pool`
  pub desired_pool: *mut c_void,   /// C: `struct glyph_pool *desired_pool`
  pub desired_matrix: *mut c_void,   /// C: `struct glyph_matrix *desired_matrix`
  pub current_matrix: *mut c_void,   /// C: `struct glyph_matrix *current_matrix`
  // frame.h 300-341: the 37 `bool_bf`/`unsigned`/`ENUM_BF` members declared between here and
  // `change_stamp` fill eight bytes, measured as two 4-byte units — with every flag set the
  // bytes are `ff ff ff ff ff 00 00 00`, so unit 1 holds the first 21 flags, then
  // `output_method : 4` and seven more flags, and unit 2 the last eight (the shift constants
  // below mask them; PORTING.md 1.3 names the unit and lets the accessors mask).
  pub bits1: u32,               // 240
  pub bits2: u32,               // 244
  pub change_stamp: c_int,
  pub number_of_windows: ptrdiff_t,
  pub tab_bar_lines: c_int,
  pub tab_bar_height: c_int,
  pub n_tab_bar_rows: c_int,
  pub n_tab_bar_items: c_int,
  pub tool_bar_lines: c_int,
  pub tool_bar_height: c_int,
  pub n_tool_bar_rows: c_int,
  pub n_tool_bar_items: c_int,
  pub decode_mode_spec_buffer: *mut c_char,   /// C: `char *decode_mode_spec_buffer`
  pub insert_line_cost: *mut c_int,   /// C: `int *insert_line_cost`
  pub delete_line_cost: *mut c_int,   /// C: `int *delete_line_cost`
  pub insert_n_lines_cost: *mut c_int,   /// C: `int *insert_n_lines_cost`
  pub delete_n_lines_cost: *mut c_int,   /// C: `int *delete_n_lines_cost`
  pub text_cols: c_int,
  pub text_lines: c_int,
  pub text_width: c_int,
  pub text_height: c_int,
  pub total_cols: c_int,
  pub total_lines: c_int,
  pub pixel_width: c_int,
  pub pixel_height: c_int,
  pub new_width: c_int,
  pub new_height: c_int,
  pub left_pos: c_int,
  pub top_pos: c_int,
  pub win_gravity: c_int,
  pub size_hint_flags: c_int,
  pub border_width: c_int,
  pub child_frame_border_width: c_int,
  pub internal_border_width: c_int,
  pub right_divider_width: c_int,
  pub bottom_divider_width: c_int,
  pub left_fringe_width: c_int,
  pub right_fringe_width: c_int,
  pub fringe_cols: c_int,
  pub menu_bar_lines: c_int,
  pub menu_bar_height: c_int,
  pub column_width: c_int,
  pub line_height: c_int,
  pub terminal: *mut terminal,
  pub output_data: output_data,        // frame.h 653-663 (C's `union output_data`)
  pub font_driver_list: *mut c_void,   /// C: `struct font_driver_list *font_driver_list`
  pub desired_cursor: text_cursor_kinds,
  pub cursor_width: c_int,
  pub blink_off_cursor: text_cursor_kinds,
  pub blink_off_cursor_width: c_int,
  pub config_scroll_bar_width: c_int,
  pub config_scroll_bar_cols: c_int,
  pub config_scroll_bar_height: c_int,
  pub config_scroll_bar_lines: c_int,
  pub cost_calculation_baud_rate: intmax_t,
  pub alpha: [c_double; 2],
  pub alpha_background: c_double,
  pub gamma: c_double,
  pub extra_line_spacing: c_int,
  pub extra_line_spacing_above: c_int,
  pub background_pixel: c_ulong,
  pub foreground_pixel: c_ulong,
  pub z_order: c_int,
}

// The 37 flag members of those two units, in C's declaration order (frame.h 300-341).
// `output_method` is the four-bit `ENUM_BF (output_method)` field in the middle of unit 1.
pub const FRAME_GLYPHS_INITIALIZED_P_SHIFT: u32 = 0;
pub const FRAME_RESIZED_P_SHIFT: u32 = 1;
pub const FRAME_DEFAULT_FACE_DONE_P_SHIFT: u32 = 2;
pub const FRAME_ALREADY_HSCROLLED_P_SHIFT: u32 = 3;
pub const FRAME_UPDATED_P_SHIFT: u32 = 4;
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 5;
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 6;
pub const FRAME_REDISPLAY_SHIFT: u32 = 7;
pub const FRAME_VISIBLE_SHIFT: u32 = 8;
pub const FRAME_ICONIFIED_SHIFT: u32 = 9;
pub const FRAME_GARBAGED_SHIFT: u32 = 10;
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 11;
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 12;
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 13;
pub const FRAME_NO_SPLIT_SHIFT: u32 = 14;
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 15;
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 16;
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 17;
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 18;
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 19;
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 20;
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 21;
pub const FRAME_OUTPUT_METHOD_MASK: u32 = 0x0F;   // 4 bits
pub const FRAME_UNDECORATED_SHIFT: u32 = 25;
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 26;
pub const FRAME_NO_SPECIAL_GLYPHS_SHIFT: u32 = 27;
pub const FRAME_CAN_SET_WINDOW_SIZE_SHIFT: u32 = 28;
pub const FRAME_AFTER_MAKE_FRAME_SHIFT: u32 = 29;
pub const FRAME_TAB_BAR_REDISPLAYED_SHIFT: u32 = 30;
pub const FRAME_TAB_BAR_RESIZED_SHIFT: u32 = 31;
pub const FRAME_TOOL_BAR_REDISPLAYED_SHIFT: u32 = 0;   // unit 2
pub const FRAME_TOOL_BAR_RESIZED_SHIFT: u32 = 1;   // unit 2
pub const FRAME_FACE_CHANGE_SHIFT: u32 = 2;   // unit 2
pub const FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT: u32 = 3;   // unit 2
pub const FRAME_NEW_SIZE_P_SHIFT: u32 = 4;   // unit 2
pub const FRAME_WAS_INVISIBLE_SHIFT: u32 = 5;   // unit 2
pub const FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT: u32 = 6;   // unit 2
pub const FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT: u32 = 7;   // unit 2

// frame.h 27-44: `enum vertical_scroll_bar_type`.
pub type vertical_scroll_bar_type = c_int;
pub const vertical_scroll_bar_none: vertical_scroll_bar_type = 0;
pub const vertical_scroll_bar_left: vertical_scroll_bar_type = 1;
pub const vertical_scroll_bar_right: vertical_scroll_bar_type = 2;

// frame.h 840-...: `XFRAME` is an expression in C (an assert plus `XUNTAG`), so the port keeps a
// function of the same name.
#[inline(always)]
pub fn XFRAME (p: Lisp_Object) -> *mut frame
{
  eassert! (FRAMEP (p));
  XUNTAG! (p, Lisp_Type::Lisp_Vectorlike, frame)
}

// C's `XSETFRAME (a, b)` writes into its first argument, so the port keeps the macro form the
// other `XSET*` setters use (a function would take the value, not the caller's slot).
macro_rules! XSETFRAME {
    ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_FRAME) };
}

// frame.h 1459-1462: `SELECTED_FRAME`.  C's `emacs_abort ()` arm yields a null frame pointer.
#[inline(always)]
pub fn SELECTED_FRAME () -> *mut frame
{
  if FRAMEP (unsafe { selected_frame }) && FRAME_LIVE_P (XFRAME (unsafe { selected_frame }))
    { XFRAME (unsafe { selected_frame }) }
  else { unsafe { emacs_abort () }; std::ptr::null_mut () }
}

// frame.h 833-880: the frame's own fields (the accessors C writes as macros).  Every one reads a
// field the layout gate asserts the offset of.
#[inline(always)]
pub fn FRAME_KBOARD (f: *mut frame) -> *mut c_void
{ unsafe { (*unsafe { (*f).terminal }).kboard } }   /// C: `struct kboard *((f)->terminal->kboard)`

#[inline(always)]
pub fn FRAME_PARENT_FRAME (f: *mut frame) -> *mut frame
{ if NILP (unsafe { (*f).parent_frame }) { std::ptr::null_mut () } else { XFRAME (unsafe { (*f).parent_frame }) } }

#[inline(always)]
pub fn FRAME_LIVE_P (f: *mut frame) -> bool { !unsafe { (*f).terminal }.is_null () }

#[inline(always)]
pub fn FRAME_INITIAL_P (f: *mut frame) -> bool
{ frame_output_method (f) == output_initial }

#[inline(always)]
pub fn FRAME_TERMCAP_P (f: *mut frame) -> bool
{ frame_output_method (f) == output_termcap }

#[inline(always)]
pub fn FRAME_X_P (f: *mut frame) -> bool
{ frame_output_method (f) == output_x_window }

// frame.h 846-880: the platforms this target is not.
#[inline(always)]
pub fn FRAME_W32_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_MSDOS_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_NS_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_PGTK_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_HAIKU_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_ANDROID_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { let _ = f; false }

// frame.h 884-895.
#[inline(always)]
pub fn FRAME_RES_X (f: *mut frame) -> c_double { let _ = f; unsafe { default_pixels_per_inch_x () } }
#[inline(always)]
pub fn FRAME_RES_Y (f: *mut frame) -> c_double { let _ = f; unsafe { default_pixels_per_inch_y () } }
#[inline(always)]
pub fn FRAME_RES (f: *mut frame) -> c_double { let _ = f; unsafe { default_pixels_per_inch_y () } }

#[inline(always)]
pub fn FRAME_MINIBUF_ONLY_P (f: *mut frame) -> bool
{ EQ (FRAME_ROOT_WINDOW (f), FRAME_MINIBUF_WINDOW (f)) }

#[inline(always)]
pub fn FRAME_HAS_MINIBUF_P (f: *mut frame) -> bool
{
  WINDOWP (unsafe { (*f).minibuffer_window })
    && std::ptr::eq (XFRAME (unsafe { (*XWINDOW (unsafe { (*f).minibuffer_window })).frame }), f)
}

#[inline(always)]
pub fn FRAME_SCALE_FACTOR (f: *mut frame) -> c_int { let _ = f; 1 }

// frame.h 906-940: the sizes.
#[inline(always)]
pub fn FRAME_PIXEL_WIDTH (f: *mut frame) -> c_int { unsafe { (*f).pixel_width } }
#[inline(always)]
pub fn FRAME_PIXEL_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).pixel_height } }
#[inline(always)]
pub fn FRAME_TOTAL_COLS (f: *mut frame) -> c_int { unsafe { (*f).total_cols } }
#[inline(always)]
pub fn FRAME_TOTAL_LINES (f: *mut frame) -> c_int { unsafe { (*f).total_lines } }
#[inline(always)]
pub fn FRAME_TEXT_WIDTH (f: *mut frame) -> c_int { unsafe { (*f).text_width } }
#[inline(always)]
pub fn FRAME_TEXT_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).text_height } }
#[inline(always)]
pub fn FRAME_COLS (f: *mut frame) -> c_int { unsafe { (*f).text_cols } }
#[inline(always)]
pub fn FRAME_LINES (f: *mut frame) -> c_int { unsafe { (*f).text_lines } }

// frame.h 950-1010: the menu-bar, tab-bar and tool-bar margins.
#[inline(always)]
pub fn FRAME_EXTERNAL_MENU_BAR (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_MENU_BAR_LINES (f: *mut frame) -> c_int { unsafe { (*f).menu_bar_lines } }
#[inline(always)]
pub fn FRAME_MENU_BAR_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).menu_bar_height } }
#[inline(always)]
pub fn FRAME_TAB_BAR_LINES (f: *mut frame) -> c_int { unsafe { (*f).tab_bar_lines } }
#[inline(always)]
pub fn FRAME_TAB_BAR_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).tab_bar_height } }
#[inline(always)]
pub fn FRAME_EXTERNAL_TOOL_BAR (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_TOOL_BAR_POSITION (f: *mut frame) -> Lisp_Object { unsafe { (*f).tool_bar_position } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_LINES (f: *mut frame) -> c_int { unsafe { (*f).tool_bar_lines } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).tool_bar_height } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_TOP_HEIGHT (f: *mut frame) -> c_int
{ if BASE_EQ (unsafe { (*f).tool_bar_position }, Qtop ()) { unsafe { (*f).tool_bar_height } } else { 0 } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_TOP_LINES (f: *mut frame) -> c_int
{ if BASE_EQ (unsafe { (*f).tool_bar_position }, Qtop ()) { unsafe { (*f).tool_bar_lines } } else { 0 } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_BOTTOM_HEIGHT (f: *mut frame) -> c_int
{ if BASE_EQ (unsafe { (*f).tool_bar_position }, Qbottom ()) { unsafe { (*f).tool_bar_height } } else { 0 } }
#[inline(always)]
pub fn FRAME_TOOL_BAR_BOTTOM_LINES (f: *mut frame) -> c_int
{ if BASE_EQ (unsafe { (*f).tool_bar_position }, Qbottom ()) { unsafe { (*f).tool_bar_lines } } else { 0 } }

#[inline(always)]
pub fn FRAME_TOP_MARGIN (f: *mut frame) -> c_int
{ FRAME_MENU_BAR_LINES (f) + FRAME_TAB_BAR_LINES (f) + FRAME_TOOL_BAR_TOP_LINES (f) }
#[inline(always)]
pub fn FRAME_TOP_MARGIN_HEIGHT (f: *mut frame) -> c_int
{ FRAME_MENU_BAR_HEIGHT (f) + FRAME_TAB_BAR_HEIGHT (f) + FRAME_TOOL_BAR_TOP_HEIGHT (f) }
#[inline(always)]
pub fn FRAME_BOTTOM_MARGIN (f: *mut frame) -> c_int { FRAME_TOOL_BAR_BOTTOM_LINES (f) }
#[inline(always)]
pub fn FRAME_BOTTOM_MARGIN_HEIGHT (f: *mut frame) -> c_int { FRAME_TOOL_BAR_BOTTOM_HEIGHT (f) }
#[inline(always)]
pub fn FRAME_MARGINS (f: *mut frame) -> c_int
{ FRAME_MENU_BAR_LINES (f) + FRAME_TAB_BAR_LINES (f) + FRAME_TOOL_BAR_LINES (f) }
#[inline(always)]
pub fn FRAME_MARGIN_HEIGHT (f: *mut frame) -> c_int
{ FRAME_MENU_BAR_HEIGHT (f) + FRAME_TAB_BAR_HEIGHT (f) + FRAME_TOOL_BAR_HEIGHT (f) }

// frame.h 1020-1080: the flag predicates (the flags are the two units the gate measured).
#[inline(always)]
pub fn FRAME_VISIBLE_P (f: *mut frame) -> bool { frame_bit1 (f, FRAME_VISIBLE_SHIFT) }
#[inline(always)]
pub fn FRAME_ICONIFIED_P (f: *mut frame) -> bool { frame_bit1 (f, FRAME_ICONIFIED_SHIFT) }
#[inline(always)]
pub fn FRAME_GARBAGED_P (f: *mut frame) -> bool { frame_bit1 (f, FRAME_GARBAGED_SHIFT) }
#[inline(always)]
pub fn FRAME_NO_SPLIT_P (f: *mut frame) -> bool { frame_bit1 (f, FRAME_NO_SPLIT_SHIFT) }
#[inline(always)]
pub fn FRAME_WANTS_MODELINE_P (f: *mut frame) -> bool { frame_bit1 (f, FRAME_WANTS_MODELINE_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOWS_FROZEN (f: *mut frame) -> bool { frame_bit1 (f, FRAME_FROZEN_WINDOW_STARTS_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOW_CHANGE (f: *mut frame) -> bool { frame_bit1 (f, FRAME_WINDOW_CHANGE_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOW_STATE_CHANGE (f: *mut frame) -> bool
{ frame_bit1 (f, FRAME_WINDOW_STATE_CHANGE_SHIFT) }
#[inline(always)]
pub fn FRAME_UNDECORATED (f: *mut frame) -> c_int
{ frame_bit1 (f, FRAME_UNDECORATED_SHIFT) as c_int }

// frame.h 1090-1130: the windows and the scroll-bar bookkeeping.
#[inline(always)]
pub fn FRAME_MINIBUF_WINDOW (f: *mut frame) -> Lisp_Object { unsafe { (*f).minibuffer_window } }
#[inline(always)]
pub fn FRAME_ROOT_WINDOW (f: *mut frame) -> Lisp_Object { unsafe { (*f).root_window } }
#[inline(always)]
pub fn FRAME_SELECTED_WINDOW (f: *mut frame) -> Lisp_Object { unsafe { (*f).selected_window } }
#[inline(always)]
pub fn FRAME_OLD_SELECTED_WINDOW (f: *mut frame) -> Lisp_Object
{ unsafe { (*f).old_selected_window } }
#[inline(always)]
pub fn FRAME_INSERT_COST (f: *mut frame) -> *mut c_int { unsafe { (*f).insert_line_cost } }
#[inline(always)]
pub fn FRAME_DELETE_COST (f: *mut frame) -> *mut c_int { unsafe { (*f).delete_line_cost } }
#[inline(always)]
pub fn FRAME_INSERTN_COST (f: *mut frame) -> *mut c_int { unsafe { (*f).insert_n_lines_cost } }
#[inline(always)]
pub fn FRAME_DELETEN_COST (f: *mut frame) -> *mut c_int { unsafe { (*f).delete_n_lines_cost } }
#[inline(always)]
pub fn FRAME_FOCUS_FRAME (f: *mut frame) -> Lisp_Object { unsafe { (*f).focus_frame } }
#[inline(always)]
pub fn FRAME_SCROLL_BARS (f: *mut frame) -> Lisp_Object { unsafe { (*f).scroll_bars } }
#[inline(always)]
pub fn FRAME_CONDEMNED_SCROLL_BARS (f: *mut frame) -> Lisp_Object
{ unsafe { (*f).condemned_scroll_bars } }
#[inline(always)]
pub fn FRAME_MENU_BAR_ITEMS (f: *mut frame) -> Lisp_Object { unsafe { (*f).menu_bar_items } }

// frame.h 1150-1220: the scroll bars this target does not have.
#[inline(always)]
pub fn FRAME_VERTICAL_SCROLL_BAR_TYPE (f: *mut frame) -> vertical_scroll_bar_type
{ let _ = f; vertical_scroll_bar_none }
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_HAS_HORIZONTAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_OVERRIDE_REDIRECT (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_SKIP_TASKBAR (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_NO_FOCUS_ON_MAP (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_NO_ACCEPT_FOCUS (f: *mut frame) -> c_int { let _ = f; 0 }
#[inline(always)]
pub fn FRAME_TOOLTIP_P (f: *mut frame) -> bool { let _ = f; false }
#[inline(always)]
pub fn FRAME_NO_SPECIAL_GLYPHS (f: *mut frame) -> bool
{ frame_bit1 (f, FRAME_NO_SPECIAL_GLYPHS_SHIFT) }

#[inline(always)]
pub fn FRAME_CONFIG_SCROLL_BAR_WIDTH (f: *mut frame) -> c_int
{ unsafe { (*f).config_scroll_bar_width } }
#[inline(always)]
pub fn FRAME_CONFIG_SCROLL_BAR_HEIGHT (f: *mut frame) -> c_int
{ unsafe { (*f).config_scroll_bar_height } }
#[inline(always)]
pub fn FRAME_CONFIG_SCROLL_BAR_COLS (f: *mut frame) -> c_int
{ unsafe { (*f).config_scroll_bar_cols } }
#[inline(always)]
pub fn FRAME_CONFIG_SCROLL_BAR_LINES (f: *mut frame) -> c_int
{ unsafe { (*f).config_scroll_bar_lines } }
#[inline(always)]
pub fn FRAME_LEFT_SCROLL_BAR_AREA_WIDTH (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f) != 0 { FRAME_CONFIG_SCROLL_BAR_WIDTH (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_RIGHT_SCROLL_BAR_AREA_WIDTH (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f) != 0 { FRAME_CONFIG_SCROLL_BAR_WIDTH (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_LEFT_SCROLL_BAR_COLS (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f) != 0 { FRAME_CONFIG_SCROLL_BAR_COLS (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_RIGHT_SCROLL_BAR_COLS (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f) != 0 { FRAME_CONFIG_SCROLL_BAR_COLS (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_SCROLL_BAR_AREA_WIDTH (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS (f) != 0 { FRAME_CONFIG_SCROLL_BAR_WIDTH (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_SCROLL_BAR_AREA_HEIGHT (f: *mut frame) -> c_int
{ if FRAME_HAS_HORIZONTAL_SCROLL_BARS (f) != 0 { FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_SCROLL_BAR_COLS (f: *mut frame) -> c_int
{ if FRAME_HAS_VERTICAL_SCROLL_BARS (f) != 0 { FRAME_CONFIG_SCROLL_BAR_COLS (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_SCROLL_BAR_LINES (f: *mut frame) -> c_int
{ if FRAME_HAS_HORIZONTAL_SCROLL_BARS (f) != 0 { FRAME_CONFIG_SCROLL_BAR_LINES (f) } else { 0 } }
#[inline(always)]
pub fn FRAME_CURSOR_X_LIMIT (f: *mut frame) -> c_int
{ FRAME_COLS (f) + FRAME_LEFT_SCROLL_BAR_COLS (f) }
#[inline(always)]
pub fn FRAME_COST_BAUD_RATE (f: *mut frame) -> intmax_t
{ unsafe { (*f).cost_calculation_baud_rate } }
#[inline(always)]
pub fn FRAME_DESIRED_CURSOR (f: *mut frame) -> text_cursor_kinds { unsafe { (*f).desired_cursor } }
#[inline(always)]
pub fn FRAME_BLINK_OFF_CURSOR (f: *mut frame) -> text_cursor_kinds
{ unsafe { (*f).blink_off_cursor } }
#[inline(always)]
pub fn FRAME_CURSOR_WIDTH (f: *mut frame) -> c_int { unsafe { (*f).cursor_width } }
#[inline(always)]
pub fn FRAME_BLINK_OFF_CURSOR_WIDTH (f: *mut frame) -> c_int
{ unsafe { (*f).blink_off_cursor_width } }
#[inline(always)]
pub fn FRAME_FOREGROUND_PIXEL (f: *mut frame) -> c_ulong { unsafe { (*f).foreground_pixel } }
#[inline(always)]
pub fn FRAME_BACKGROUND_PIXEL (f: *mut frame) -> c_ulong { unsafe { (*f).background_pixel } }
#[inline(always)]
pub fn FRAME_MESSAGE_BUF_SIZE (f: *mut frame) -> c_int { 4 * FRAME_COLS (f) }
#[inline(always)]
pub fn FRAME_LINE_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).line_height } }
#[inline(always)]
pub fn FRAME_COLUMN_WIDTH (f: *mut frame) -> c_int { unsafe { (*f).column_width } }

// frame.h 1560-1645: the fringe/dimension helpers (they all go through `frame_dimension`,
// which is zero on a target without a window system).
#[inline(always)]
pub fn frame_dimension (_x: c_int) -> c_int { 0 }

#[inline(always)]
pub fn FRAME_FRINGE_COLS (f: *mut frame) -> c_int { frame_dimension (unsafe { (*f).fringe_cols }) }
#[inline(always)]
pub fn FRAME_LEFT_FRINGE_WIDTH (f: *mut frame) -> c_int
{ frame_dimension (unsafe { (*f).left_fringe_width }) }
#[inline(always)]
pub fn FRAME_RIGHT_FRINGE_WIDTH (f: *mut frame) -> c_int
{ frame_dimension (unsafe { (*f).right_fringe_width }) }
#[inline(always)]
pub fn FRAME_TOTAL_FRINGE_WIDTH (f: *mut frame) -> c_int
{ FRAME_LEFT_FRINGE_WIDTH (f) + FRAME_RIGHT_FRINGE_WIDTH (f) }
#[inline(always)]
pub fn FRAME_CHILD_FRAME_BORDER_WIDTH (f: *mut frame) -> c_int
{ frame_dimension (unsafe { (*f).child_frame_border_width }) }
#[inline(always)]
pub fn FRAME_INTERNAL_BORDER_WIDTH (f: *mut frame) -> c_int
{
  if !FRAME_PARENT_FRAME (f).is_null ()
    {
      if FRAME_CHILD_FRAME_BORDER_WIDTH (f) >= 0 { FRAME_CHILD_FRAME_BORDER_WIDTH (f) }
      else { frame_dimension (unsafe { (*f).internal_border_width }) }
    }
  else { frame_dimension (unsafe { (*f).internal_border_width }) }
}
#[inline(always)]
pub fn FRAME_RIGHT_DIVIDER_WIDTH (f: *mut frame) -> c_int
{ frame_dimension (unsafe { (*f).right_divider_width }) }
#[inline(always)]
pub fn FRAME_BOTTOM_DIVIDER_WIDTH (f: *mut frame) -> c_int
{ frame_dimension (unsafe { (*f).bottom_divider_width }) }

// frame.h 896-920: the two `NUMVAL` helpers (the inline plus the two defaults it feeds).
#[inline(always)]
pub fn NUMVAL (x: Lisp_Object) -> c_double
{ if NUMBERP (x) { unsafe { XFLOATINT (x) } } else { -1.0 } }

#[inline(always)]
pub unsafe fn default_pixels_per_inch_x () -> c_double
{
  let v = if CONSP (Vdisplay_pixels_per_inch) { XCAR (Vdisplay_pixels_per_inch) }
          else { Vdisplay_pixels_per_inch };
  if NUMVAL (v) > 0.0 { NUMVAL (v) } else { 72.0 }
}

#[inline(always)]
pub unsafe fn default_pixels_per_inch_y () -> c_double
{
  let v = if CONSP (Vdisplay_pixels_per_inch) { XCDR (Vdisplay_pixels_per_inch) }
          else { Vdisplay_pixels_per_inch };
  if NUMVAL (v) > 0.0 { NUMVAL (v) } else { 72.0 }
}

#[inline(always)]
pub fn window_system_available (f: *mut frame) -> bool { let _ = f; false }

// frame.h 1437-1450: sets the `iconified` flag (bit 9 of unit 1).
#[inline(always)]
pub fn SET_FRAME_ICONIFIED (f: *mut frame, i: c_int)
{
  eassert! (0 <= i && i <= 1);
  let mask = 1u32 << FRAME_ICONIFIED_SHIFT;
  unsafe { (*f).bits1 = ((*f).bits1 & !mask) | ((i as u32) << FRAME_ICONIFIED_SHIFT); }
}

// The readers the accessors above share: `output_method` is the four-bit field in the middle of
// unit 1, and the flags are single bits of either unit (PORTING.md 1.3; the note file records
// that C's field names cannot occur in the Rust bodies).
#[inline(always)]
/// C: `(f)->output_method`.  frame.h 1069's family reads the field through a macro; the port
/// reads it through `frame_output_method`, and this alias keeps the C token `output_method`
/// available to the macro bodies that spell it (termchar.h 231's `FRAME_TTY`).  The function
/// and the field's type share a name because they live in different namespaces.
#[inline(always)]
pub fn output_method (f: *mut frame) -> output_method { frame_output_method (f) }

fn frame_output_method (f: *mut frame) -> output_method
{ ((unsafe { (*f).bits1 }) >> FRAME_OUTPUT_METHOD_SHIFT & FRAME_OUTPUT_METHOD_MASK) as output_method }

#[inline(always)]
fn frame_bit1 (f: *mut frame, shift: u32) -> bool { (unsafe { (*f).bits1 }) >> shift & 1 != 0 }

// frame.h 1428-1521: the file-scope objects (the .c files define them).
unsafe extern "C" { pub static mut frame_next_id: EMACS_UINT; }              // 1428
unsafe extern "C" { pub static mut frame_garbaged: bool; }                   // 1433
unsafe extern "C" { pub static mut selected_frame: Lisp_Object; }            // 1459
unsafe extern "C" { pub static mut old_selected_frame: Lisp_Object; }        // 1460
unsafe extern "C" { pub static mut frame_default_tab_bar_height: c_int; }    // 1462
unsafe extern "C" { pub static mut frame_default_tool_bar_height: c_int; }   // 1465
unsafe extern "C" { pub static mut Vframe_list: Lisp_Object; }               // 1521

/* Most code should use these functions to set Lisp fields in struct frame.  */
// frame.h 753-895: the frame's Lisp-Object setters, one assignment each (C's `fset_*`).
#[inline(always)]
pub fn fset_buffer_list (f: *mut frame, val: Lisp_Object) { unsafe { (*f).buffer_list = val; } }
#[inline(always)]
pub fn fset_buried_buffer_list (f: *mut frame, val: Lisp_Object) { unsafe { (*f).buried_buffer_list = val; } }
#[inline(always)]
pub fn fset_condemned_scroll_bars (f: *mut frame, val: Lisp_Object) { unsafe { (*f).condemned_scroll_bars = val; } }
#[inline(always)]
pub fn fset_face_hash_table (f: *mut frame, val: Lisp_Object) { unsafe { (*f).face_hash_table = val; } }
#[inline(always)]
pub fn fset_focus_frame (f: *mut frame, val: Lisp_Object) { unsafe { (*f).focus_frame = val; } }
#[inline(always)]
pub fn fset_icon_name (f: *mut frame, val: Lisp_Object) { unsafe { (*f).icon_name = val; } }
#[inline(always)]
pub fn fset_menu_bar_items (f: *mut frame, val: Lisp_Object) { unsafe { (*f).menu_bar_items = val; } }
#[inline(always)]
pub fn fset_menu_bar_vector (f: *mut frame, val: Lisp_Object) { unsafe { (*f).menu_bar_vector = val; } }
#[inline(always)]
pub fn fset_name (f: *mut frame, val: Lisp_Object) { unsafe { (*f).name = val; } }
#[inline(always)]
pub fn fset_param_alist (f: *mut frame, val: Lisp_Object) { unsafe { (*f).param_alist = val; } }
#[inline(always)]
pub fn fset_root_window (f: *mut frame, val: Lisp_Object) { unsafe { (*f).root_window = val; } }
#[inline(always)]
pub fn fset_scroll_bars (f: *mut frame, val: Lisp_Object) { unsafe { (*f).scroll_bars = val; } }
#[inline(always)]
pub fn fset_selected_window (f: *mut frame, val: Lisp_Object) { unsafe { (*f).selected_window = val; } }
#[inline(always)]
pub fn fset_old_selected_window (f: *mut frame, val: Lisp_Object) { unsafe { (*f).old_selected_window = val; } }
#[inline(always)]
pub fn fset_title (f: *mut frame, val: Lisp_Object) { unsafe { (*f).title = val; } }
#[inline(always)]
pub fn fset_tab_bar_items (f: *mut frame, val: Lisp_Object) { unsafe { (*f).tab_bar_items = val; } }
#[inline(always)]
pub fn fset_tool_bar_items (f: *mut frame, val: Lisp_Object) { unsafe { (*f).tool_bar_items = val; } }
#[inline(always)]
pub fn fset_tool_bar_position (f: *mut frame, val: Lisp_Object) { unsafe { (*f).tool_bar_position = val; } }
