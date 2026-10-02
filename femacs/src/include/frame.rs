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
  pub decode_mode_spec_buffer: *mut c_void,   /// C: `char *decode_mode_spec_buffer`
  pub insert_line_cost: *mut c_void,   /// C: `int *insert_line_cost`
  pub delete_line_cost: *mut c_void,   /// C: `int *delete_line_cost`
  pub insert_n_lines_cost: *mut c_void,   /// C: `int *insert_n_lines_cost`
  pub delete_n_lines_cost: *mut c_void,   /// C: `int *delete_n_lines_cost`
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

// C: frame.h 700-1900's 139 `FRAME_*` macros, the 37 inlines and the 7 file-scope objects
// — (not mirrored yet; they land next, in this same file).
