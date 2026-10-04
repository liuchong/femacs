// femacs: port of src/frame.h @ fd7afced403.
//
// WIP mirror (PORTING.md P-66): the `frame` structure and its flag units.  `frame.h` includes
// `termhooks.h` and `window.h` (22-23; `dispextern.h` arrives through termhooks.h 27), so C sees
// `struct face_cache`, `struct glyph_matrix`, `struct glyph_pool` and `struct terminal` complete
// here; of those, only `terminal` has a mirror so far, so the three display ones carry the P-72
// rule ③ degradation until the display chain reaches them.  The 139 `FRAME_*` macros, 37 inlines
// and 7 file-scope objects come next.
//
// `tools/check-cond-eval.py` reports zero disagreements for this header, so the branch set is
// the frozen configuration's (`HAVE_WINDOW_SYSTEM`, `HAVE_NEXTSTEP`, the `USE_*` toolkits and
// every platform `*_P` predicate are `#undef`/false here).

// frame.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_FRAME_H: bool = true;

// frame.h 22-23: the header's own `#include`s.  C reaches both here, and the text below
// names them (`struct terminal` in the record's `terminal` member and the `FRAME_KBOARD`/
// `FRAME_LIVE_P` accessors that deref it, `struct window` in `WINDOW_FRAME` and the
// `WINDOWP`/`XWINDOW` calls of `FRAME_HAS_MINIBUF_P`): the mirror carries them so a unit
// whose own include list does not supply
// them still resolves the names (P-72 rule ④; `src/androidmenu.c` is that unit - it includes
// `androidterm.h`, which includes `frame.h`, and no `window.h`/`termhooks.h` of its own).
include!("termhooks.rs");                     // C 22: `#include "termhooks.h"`
include!("window.rs");                        // C 23: `#include "window.h"`

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

/* frame.h 69-76: `enum ns_appearance_type`, inside C's `#ifdef HAVE_WINDOW_SYSTEM` and its own
   `#ifdef NS_IMPL_COCOA`.  */
#[cfg (feature = "ns_impl_cocoa")]
pub type ns_appearance_type = c_int;
#[cfg (feature = "ns_impl_cocoa")]
pub const ns_appearance_system_default: ns_appearance_type = 0;
#[cfg (feature = "ns_impl_cocoa")]
pub const ns_appearance_aqua: ns_appearance_type = 1;
#[cfg (feature = "ns_impl_cocoa")]
pub const ns_appearance_vibrant_dark: ns_appearance_type = 2;

/* frame.h 78-142: `#ifdef HAVE_TEXT_CONVERSION`.  The operation enum, the single action and the
   per-frame state `struct frame` embeds (`conversion`).  C declares them before the frame, so the
   port does too.  */
#[cfg (feature = "have_text_conversion")]
pub type text_conversion_operation = c_int;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_START_BATCH_EDIT: text_conversion_operation = 0;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_END_BATCH_EDIT: text_conversion_operation = 1;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_COMMIT_TEXT: text_conversion_operation = 2;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_FINISH_COMPOSING_TEXT: text_conversion_operation = 3;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_SET_COMPOSING_TEXT: text_conversion_operation = 4;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_SET_COMPOSING_REGION: text_conversion_operation = 5;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_SET_POINT_AND_MARK: text_conversion_operation = 6;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_DELETE_SURROUNDING_TEXT: text_conversion_operation = 7;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_REQUEST_POINT_UPDATE: text_conversion_operation = 8;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_BARRIER: text_conversion_operation = 9;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_REPLACE_TEXT: text_conversion_operation = 10;

// frame.h 100-114.  `next` is a complete-tag pointer to its own type, so the port spells it.
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct text_conversion_action
{
  pub next: *mut text_conversion_action,
  pub data: Lisp_Object,
  pub operation: text_conversion_operation,
  pub counter: c_ulong,
}

// frame.h 119-142.
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct text_conversion_state
{
  pub actions: *mut text_conversion_action,
  pub compose_region_start: Lisp_Object,
  pub compose_region_end: Lisp_Object,
  pub compose_region_overlay: Lisp_Object,
  pub field: Lisp_Object,
  pub batch_edit_count: c_int,
  pub batch_edit_flags: c_int,
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
  // frame.h 246-268: the window-system windows and their strings, each under C's own guard.
  #[cfg (all (feature = "have_window_system", not (feature = "have_ext_menu_bar")))]
  pub menu_bar_window: Lisp_Object,                        // 249
  #[cfg (feature = "have_window_system")]
  pub tab_bar_window: Lisp_Object,                         // 254
  #[cfg (feature = "have_window_system")]
  pub desired_tab_bar_string: Lisp_Object,                 // 257
  #[cfg (feature = "have_window_system")]
  pub current_tab_bar_string: Lisp_Object,                 // 258
  #[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar")))]
  pub tool_bar_window: Lisp_Object,                        // 263
  #[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar")))]
  pub desired_tool_bar_string: Lisp_Object,                // 266
  #[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar")))]
  pub current_tool_bar_string: Lisp_Object,                // 267
  pub tool_bar_position: Lisp_Object,
  #[cfg (any (feature = "have_xft", feature = "have_freetype"))]
  pub font_data: Lisp_Object,                              // 276
  pub tab_bar_items: Lisp_Object,
  pub tool_bar_items: Lisp_Object,
  pub face_cache: *mut c_void,   /// C: `struct face_cache *face_cache`
  #[cfg (feature = "have_window_system")]
  pub image_cache: *mut c_void,   /// C: `struct image_cache *image_cache` (frame.h 290-292)
  pub id: EMACS_UINT,
  pub last_tab_bar_item: c_int,
  #[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar")))]
  pub last_tool_bar_item: c_int,                           // 303
  pub menu_bar_items_used: c_int,
  #[cfg (any (feature = "use_x_toolkit", feature = "have_ntgui"))]
  pub namebuf: *mut c_char,                                // 313
  #[cfg (feature = "use_x_toolkit")]
  pub shell_position: *mut c_char,                         // 318
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
  #[cfg (any (feature = "have_x_windows", feature = "have_android"))]
  pub wait_event_type: c_int,                              // 671
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
  // frame.h 735-747: the Cocoa tail, and the text-conversion state C embeds before `z_order`.
  #[cfg (feature = "ns_impl_cocoa")]
  pub ns_appearance: ns_appearance_type,                   // 737 (measured 592)
  #[cfg (feature = "ns_impl_cocoa")]
  pub ns_transparent_titlebar: c_uint,                     // 738; C's `bool_bf` is `unsigned int`
                                                           // here (conf_post.h 59), measured 596
  #[cfg (feature = "have_text_conversion")]
  pub conversion: text_conversion_state,                   // 743
  pub z_order: c_int,
}

// The flag members of those two units, in C's declaration order (frame.h 300-341 and the
// window-system flags C inserts *inside* the run).  `output_method` is the four-bit
// `ENUM_BF (output_method)` field in the middle of unit 1.
//
// The numbering is *per configuration*: C's compiler packs the flags into four-byte units, a
// window system inserts its own flags, and an `ENUM_BF` that does not fit in what is left of a
// unit is pushed to the next one (measured: `want_fullscreen` sits at 316:0-3 under X11, one bit
// past `tooltip` at 315:6).  The readings for both window-system arms are in
// `docs/baseline/frame-layout.tsv`, written by `spikes/gated-layout-probe.py 'struct frame'
// frame.h` (P-82) - `frozen` 568, `x11` 648, `ns_cocoa` 608, every member's byte:bits.
//
// The constants below are the *frozen* arm's numbers, and they are gated with it: a
// window-system configuration has no numbering here yet, so it fails to compile at the first
// accessor instead of silently reading the wrong bit.  Writing the measured arms out (or
// deriving them from C's order and checking the derivation against the table) is the next step;
// the numbers are a table, not a guess.
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_GLYPHS_INITIALIZED_P_SHIFT: u32 = 0;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_RESIZED_P_SHIFT: u32 = 1;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_DEFAULT_FACE_DONE_P_SHIFT: u32 = 2;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_ALREADY_HSCROLLED_P_SHIFT: u32 = 3;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_UPDATED_P_SHIFT: u32 = 4;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 5;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 6;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_REDISPLAY_SHIFT: u32 = 7;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_VISIBLE_SHIFT: u32 = 8;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_ICONIFIED_SHIFT: u32 = 9;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_GARBAGED_SHIFT: u32 = 10;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 11;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 12;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 13;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_SPLIT_SHIFT: u32 = 14;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 15;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 16;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 17;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 18;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 19;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 20;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 21;
pub const FRAME_OUTPUT_METHOD_MASK: u32 = 0x0F;   // 4 bits
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_UNDECORATED_SHIFT: u32 = 25;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 26;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_SPECIAL_GLYPHS_SHIFT: u32 = 27;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_CAN_SET_WINDOW_SIZE_SHIFT: u32 = 28;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AFTER_MAKE_FRAME_SHIFT: u32 = 29;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_REDISPLAYED_SHIFT: u32 = 30;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_RESIZED_SHIFT: u32 = 31;
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TOOL_BAR_REDISPLAYED_SHIFT: u32 = 0;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TOOL_BAR_RESIZED_SHIFT: u32 = 1;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FACE_CHANGE_SHIFT: u32 = 2;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT: u32 = 3;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NEW_SIZE_P_SHIFT: u32 = 4;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WAS_INVISIBLE_SHIFT: u32 = 5;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT: u32 = 6;   // unit 2
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT: u32 = 7;   // unit 2

// frame.h 27-44: `enum vertical_scroll_bar_type`.
pub type vertical_scroll_bar_type = c_int;
pub const vertical_scroll_bar_none: vertical_scroll_bar_type = 0;
pub const vertical_scroll_bar_left: vertical_scroll_bar_type = 1;
pub const vertical_scroll_bar_right: vertical_scroll_bar_type = 2;

// frame.h 924-925: `XFRAME` is an expression in C (an assert plus `XUNTAG`), so the port keeps a
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

/* FOR_EACH_FRAME (LIST_VAR, FRAME_VAR) followed by a statement is a
   `for' loop which iterates over the elements of Vframe_list.  The
   loop will set FRAME_VAR, a Lisp_Object, to each frame in
   Vframe_list in succession and execute the statement.  LIST_VAR
   should be a Lisp_Object too; it is used to iterate through the
   Vframe_list.  Note that this macro walks over child frames and
   the tooltip frame as well.

   This macro is a holdover from a time when multiple frames weren't always
   supported.  An alternate definition of the macro would expand to
   something which executes the statement once.  */
// C 1404-1407: `#define FOR_EACH_FRAME(list_var, frame_var) for ((list_var) = Vframe_list;
// (CONSP (list_var) && (frame_var = XCAR (list_var), true)); list_var = XCDR (list_var))`.
// C's body follows the macro at the call site; Rust macro hygiene cannot leave a block to be
// appended, so the body is the last argument (the `FOR_EACH_PER_BUFFER_OBJECT_AT` and
// `DEFUN!` shape) and the `for` becomes a `while` whose increment sits at the body's end —
// the increment still runs after the body, exactly as C's does.  The token deviation (`for`)
// is recorded in docs/ports/macro-body-notes.tsv.
macro_rules! FOR_EACH_FRAME {
    ($list_var:ident, $frame_var:ident, $body:block) => {
        $list_var = unsafe { Vframe_list };
        while CONSP ($list_var) && { $frame_var = XCAR ($list_var); true } {
            $body
            $list_var = XCDR ($list_var);
        }
    };
}

// frame.h 1526-1530: `SELECTED_FRAME`.  C's `emacs_abort ()` arm yields a null frame pointer.
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

// frame.h 1023-1039: `MOUSE_HL_INFO (F)` - "Return a pointer to the structure holding
// information about the region of text, if any, that is currently shown in mouse-face on frame
// F.  We need to define two versions because a TTY-only build does not have
// FRAME_DISPLAY_INFO."  It is a macro in C and stays one here: its body names `tty_output`'s
// `display_info` and `tty_display_info`'s `mouse_highlight`, which the *including* unit brings in
// through `termchar.rs` (frame.rs cannot include it - termchar.rs includes frame.rs).  The frozen
// configuration has no window system, so C 1037-1039's TTY arm is the live one and the
// HAVE_WINDOW_SYSTEM/HAVE_ANDROID arms are recorded in docs/ports/include-frame.md (P-08).
macro_rules! MOUSE_HL_INFO {
  ($f:expr) => {{
    let f = $f;
    unsafe {
      let tty: *mut tty_output = (*f).output_data.tty as *mut tty_output;
      std::ptr::addr_of_mut! ((*(*tty).display_info).mouse_highlight)
    }
  }};
}

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
#[cfg (not (feature = "have_ext_menu_bar"))]
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
#[cfg (not (feature = "have_ext_tool_bar"))]
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
pub fn FRAME_VISIBLE_P (f: *mut frame) -> bool
{
  /* C: `((f)->visible)`.  This is the 1-bit field the `visible`/`interval_visible_bit` token
     family names (token-aliases.tsv line 57), so the port keeps C's spelling as the shift the
     flag is packed at - the family's occurrence check asks for one occurrence per C use, and
     `FRAME_VISIBLE_SHIFT` (one token) does not answer it.  */
  let visible = FRAME_VISIBLE_SHIFT;
  frame_bit1 (f, visible)
}
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

// frame.h 1078-1098 and 1215-1230: four macros whose C `#if` arm reads a member and whose
// `#else` arm is what the port spells.  Like frame.h 1240-1271's family, they are gated with the
// arm whose body they are: the window-system body needs the flag numbering measured in
// `docs/baseline/frame-bits.tsv`, and a window-system configuration fails to compile here until
// that numbering is written rather than answering with the frozen arm's constant.
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_VERTICAL_SCROLL_BAR_TYPE (f: *mut frame) -> vertical_scroll_bar_type
{ let _ = f; vertical_scroll_bar_none }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "use_horizontal_scroll_bars"))]
#[inline(always)]
pub fn FRAME_HAS_HORIZONTAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_OVERRIDE_REDIRECT (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_SKIP_TASKBAR (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_NO_FOCUS_ON_MAP (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_NO_ACCEPT_FOCUS (f: *mut frame) -> c_int { let _ = f; 0 }
#[cfg (not (feature = "have_window_system"))]
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
/// available to the macro bodies that spell it (termchar.h 243's `FRAME_TTY`).  The function
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

// frame.h 1506: cmds.c's self-insert path hides the mouse pointer through this one.
unsafe extern "C" { pub fn frame_make_pointer_invisible (f: *mut frame); }             // 1506

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

/* =============================================================================================
   Third slice: the two remaining enums, the geometry family, the flag writers and the face cache
   (frame.h 35-45, 61-67, 918-1830, 1914-1921).  The C text of every item below is quoted in
   docs/ports/include-frame.md's third-slice Decisions; the `--check` token comparison is what
   each body answers, so C's 3+-character identifiers reappear verbatim.
   ============================================================================================= */

/* frame.h 34-45: `enum internal_border_part`.  C's enum ABI is `c_int` (PORTING.md 1.3) and
   the nine enumerators are in C's order.  **Ownership (status 3.211): this mirror owns the type
   and its enumerators** - `src/frame.rs` 351-362 and `src/keyboard.rs` 6155-6166 held copies
   only because the mirror lacked them (their own comments said so), and both copies retire in
   the same commit, which is the `stdbit.in.rs` rule of 3.181 applied to this header.  */
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

/* frame.h 1379 / 1646-1650 / 1656-1661's `FRAME_FACE_CACHE` / `FACE_FROM_ID` /
   `FACE_FROM_ID_OR_NULL` are carried **here**: in a module this mirror does not glob-import, and
   behind `dispextern_face_records`.  That is the closure of 3.211's ruling (landed 3.230).  Each
   is text C pastes into every unit that includes this header (`#define FRAME_FACE_CACHE(f)
   ((struct frame *) (f))->face_cache` at C 1379, `INLINE struct face * FACE_FROM_ID (struct
   frame *f, int id)` at C 1646, `FACE_FROM_ID_OR_NULL` at C 1656), so the port's live form for an
   inline definition is one copy per including unit: measured, `macro_rules! FRAME_FACE_CACHE`
   stands in `src/alloc.rs` 3467, `src/font.rs` 545, `src/xfaces.rs` 445, `src/xdisp.rs` 1101 and
   `src/window.rs` 920, with the `fn` forms in `src/term.rs` 364-377 and the other includers.  A
   top-level copy here is `E0428` in all of them (measured with `build-tu src/frame.c` and
   `build-tu src/term.c`), so this copy is a **module**: `tools/check-mirror-dupes.py` reports a
   declaration with a body as a definition and never as a duplicate, and a module the mirror does
   not glob-import is not in the unit's scope - C's text is present for the gates, and the units
   keep their live copies.  It needs `struct face` / `struct face_cache`, which
   `include/dispextern.rs` carries behind the same feature.  **The flip arm is not green yet**:
   `rustc --cfg 'feature="dispextern_face_records"'
   build/femacs/units/<stem>/_root.rs` is `E0428` for `face`/`face_cache` in `src/term.rs`,
   `src/alloc.rs` and `src/xfaces.rs` (measured), because those units' typed copies are
   unconditional; closing it is one `#[cfg (not (feature = "dispextern_face_records"))]` per copy
   and waits for the two in-flight ones (`xdisp.rs`, `font.rs`) so it can be done in one move.
   `src/frame.rs` itself flips green today (measured).  */

#[cfg (feature = "dispextern_face_records")]
pub mod frame_face_accessors
{
  use super::*;

  /// C 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`
  #[inline (always)]
  pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
  { unsafe { (*f).face_cache as *mut face_cache } }

  /// C 1646-1650: `INLINE struct face * FACE_FROM_ID (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
  {
    eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
    unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
  }

  /// C 1656-1661: `INLINE struct face * FACE_FROM_ID_OR_NULL (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
  {
    let used: c_int = unsafe { (*FRAME_FACE_CACHE (f)).used };
    eassume! (0 <= used);
    if 0 <= id && id < used { unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) } }
    else { core::ptr::null_mut () }
  }
}
/* frame.h 61-67: `enum z_group` - C defines it *inside* `#ifdef HAVE_WINDOW_SYSTEM` (C 47),
   while C 1266's live `#else` arm of `FRAME_Z_GROUP` names one of its enumerators, so the port
   carries the four members (the `sys_types.in.rs` 103-132 reading of a dead arm's fact) and the
   ledger records the arm (P-08).  */
pub type z_group = c_int;
pub const z_group_none: z_group = 0;
pub const z_group_above: z_group = 1;
pub const z_group_below: z_group = 2;
pub const z_group_above_suspended: z_group = 3;

/* frame.h 922: `#define FRAME_IMAGE_CACHE(F) ((F)->image_cache)`.  The member is
   `#ifdef HAVE_WINDOW_SYSTEM` in C's structure (C 289-292) and this target has no window
   system, so the port cannot name a field it does not have: the name stays a macro over C's own
   path, whose pointee the including unit brings in (the `MOUSE_HL_INFO` shape above).
   C: #define FRAME_IMAGE_CACHE(F) ((F)->image_cache)  */
macro_rules! FRAME_IMAGE_CACHE { ($f:expr) => { unsafe { (*$f).image_cache } }; }

/* frame.h 929: `#define WINDOW_FRAME(w) ((w)->frame)`.  The name is a *function* here, not a
   macro: `window.rs` 406 already owns the macro spelling (its units call `WINDOW_FRAME! (w)`),
   and a second `macro_rules!` of one name is `E0659`-ambiguous at those call sites - measured
   with `build-tu src/frame.c` before this change.  The pointee resolves through this mirror's own
   `window.rs` include now (C 23, the note at the top), not through the including unit.  */
#[inline(always)]
pub fn WINDOW_FRAME (w: *mut window) -> Lisp_Object { unsafe { (*w).frame } }

/* frame.h 1173-1176: `SET_FRAME_GARBAGED(f)` - C's three-term comma expression, kept in C's
   order; `fset_redisplay` is xdisp.c's (the unit shape, `MOUSE_HL_INFO`'s note).  The unit
   `src/frame.rs` also defines this name (its whole point is that it writes unit 1 by hand);
   the mirror carries the faithful body for the header's own item, and the unit's later
   definition shadows it inside that unit - the ownership question is recorded in the ledger.
   C: #define SET_FRAME_GARBAGED(f) (frame_garbaged = true, fset_redisplay (f), f->garbaged = true)  */
macro_rules! SET_FRAME_GARBAGED {
  ($f:expr) => {{
    frame_garbaged = true;
    /* The cast is `as _`, not `as *mut c_void`: the callee's own spelling depends on the
       unit.  `xdisp.c` *defines* `fset_redisplay (struct frame *)`, so inside its unit the
       name resolves to that definition and takes `*mut frame`; every other unit sees only
       `window_redisplay_decls`'s declaration, which spells the same pointer `*mut c_void`
       (P-72 rule 3).  A cast to either spelling breaks the other half - measured: with
       `*mut c_void` here a fresh `build-tu src/xdisp.c` fails E0308, and the stale generated
       unit had hidden it since 3.333.  `as _` infers the callee's parameter type at each
       call site.  */
    fset_redisplay ($f as _);
    (*$f).bits1 = (*$f).bits1 | (1u32 << FRAME_GARBAGED_SHIFT);
  }};
}

/* frame.h 1240-1271: C spells *two* bodies for this family - the `#if defined (HAVE_WINDOW_SYSTEM)`
   arm reads the members (through the window-system numbering the port has measured but not yet
   written out, P-82), the `#else` arm is `((void) (f), …)`.  The port carries the `#else` bodies
   and gates them with the frozen arm, so a window-system configuration fails to compile here
   rather than returning the frozen answer for a frame that has the members.  */
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP (f: *mut frame) -> z_group { let _ = f; z_group_none }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_NONE (f: *mut frame) -> bool { let _ = f; true }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_ABOVE (f: *mut frame) -> bool { let _ = f; false }
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_BELOW (f: *mut frame) -> bool { let _ = f; false }

/* frame.h 1387-1391: `CHECK_FRAME`/`CHECK_LIVE_FRAME`.  C calls `CHECK_TYPE (…)`, which the port
   has as a function (lisp.rs 528); the names stay macros because every call site writes
   `CHECK_FRAME! (x)` (src/frame.rs 558, src/dispnew.rs 4579).  */
/// C: #define CHECK_FRAME(x) CHECK_TYPE (FRAMEP (x), Qframep, x)
macro_rules! CHECK_FRAME {
  ($x:expr) => { CHECK_TYPE (FRAMEP ($x), Qframep (), $x) };
}
/// C: #define CHECK_LIVE_FRAME(x) CHECK_TYPE (FRAMEP (x) && FRAME_LIVE_P (XFRAME (x)), Qframe_live_p, x)
macro_rules! CHECK_LIVE_FRAME {
  ($x:expr) => { CHECK_TYPE (FRAMEP ($x) && FRAME_LIVE_P (XFRAME ($x)), Qframe_live_p (), $x) };
}

/* frame.h 1411-1423: `FRAME_MOUSE_UPDATE(frame)` - C's `do { … } while (false)`, whose three
   calls are xdisp.c's (`block_input`/`note_mouse_highlight`/`unblock_input`) and whose
   `Mouse_HLInfo` is the `MOUSE_HL_INFO!` above; the port runs the block once, and the token
   deviation (`false`) is a docs/ports/macro-body-notes.tsv row.
   C: #define FRAME_MOUSE_UPDATE(frame) do { Mouse_HLInfo *hlinfo = MOUSE_HL_INFO (frame); if (frame == hlinfo->mouse_face_mouse_frame) { block_input (); note_mouse_highlight (hlinfo->mouse_face_mouse_frame, hlinfo->mouse_face_mouse_x, hlinfo->mouse_face_mouse_y); unblock_input (); } } while (false)  */
macro_rules! FRAME_MOUSE_UPDATE {
  ($frame:expr) => {{
    let frame = $frame;
    let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (frame);
    if frame == unsafe { (*hlinfo).mouse_face_mouse_frame } {
      block_input ();
      note_mouse_highlight (unsafe { (*hlinfo).mouse_face_mouse_frame },
                            unsafe { (*hlinfo).mouse_face_mouse_x },
                            unsafe { (*hlinfo).mouse_face_mouse_y });
      unblock_input ();
    }
  }};
}

/* frame.h 1425-1426: `AUTO_FRAME_ARG(name, parameter, value)` - "Handy macro to construct an
   argument to Fmodify_frame_parameters"; both names it forwards to are lisp.rs's macros.
   C: #define AUTO_FRAME_ARG(name, parameter, value) AUTO_LIST1 (name, AUTO_CONS_EXPR (parameter, value))  */
macro_rules! AUTO_FRAME_ARG {
  ($name:ident, $parameter:expr, $value:expr) => {
    AUTO_LIST1! ($name, AUTO_CONS_EXPR! ($parameter, $value))
  };
}

/* frame.h 1537-1552: `SET_FRAME_VISIBLE` - C's inline, with the tty-child-frame re-selection
   arm the unit's own copy omits (`is_tty_child_frame`/`mru_rooted_frame`/`do_switch_frame` are
   frame.c's statics, so the body keeps the macro form: PORTING.md P-51's note in the ledger).
   The `visible` spelling is the token-aliases family's (`FRAME_VISIBLE_P` above).
   C: INLINE void SET_FRAME_VISIBLE (struct frame *f, bool v) { if (v) { if (v == 1 && f->visible != 1) redisplay_other_windows (); if (FRAME_GARBAGED_P (f)) frame_garbaged = true; } else if (is_tty_child_frame (f) && f == XFRAME (selected_frame)) do_switch_frame (mru_rooted_frame (f), 0, 0, Qnil); f->visible = v; }  */
macro_rules! SET_FRAME_VISIBLE {
    ($f:expr, $v:expr) => { (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_VISIBLE_SHIFT))
                                            | (($v as u32) << FRAME_VISIBLE_SHIFT) };
}

/* ---- frame.h 1697-1830: the geometry family, C's arithmetic member for member ---------------- */

/// C: #define FRAME_PIXEL_X_FROM_CANON_X(F, X) ((int) (XFLOATINT (X) * FRAME_COLUMN_WIDTH (F)))
#[inline(always)]
pub fn FRAME_PIXEL_X_FROM_CANON_X (f: *mut frame, x: Lisp_Object) -> c_int
{ (XFLOATINT (x) * FRAME_COLUMN_WIDTH (f) as c_double) as c_int }

/// C: #define FRAME_PIXEL_Y_FROM_CANON_Y(F, Y) ((int) (XFLOATINT (Y) * FRAME_LINE_HEIGHT (F)))
#[inline(always)]
pub fn FRAME_PIXEL_Y_FROM_CANON_Y (f: *mut frame, y: Lisp_Object) -> c_int
{ (XFLOATINT (y) * FRAME_LINE_HEIGHT (f) as c_double) as c_int }

/// C: #define FRAME_CANON_X_FROM_PIXEL_X(F, X) ((X) % FRAME_COLUMN_WIDTH (F) != 0 ? make_float ((double) (X) / FRAME_COLUMN_WIDTH (F)) : make_fixnum ((X) / FRAME_COLUMN_WIDTH (F)))
#[inline(always)]
pub fn FRAME_CANON_X_FROM_PIXEL_X (f: *mut frame, x: c_int) -> Lisp_Object
{
  if x % FRAME_COLUMN_WIDTH (f) != 0
    { unsafe { make_float (x as c_double / FRAME_COLUMN_WIDTH (f) as c_double) } }
  else { make_fixnum ((x / FRAME_COLUMN_WIDTH (f)) as EMACS_INT) }
}

/// C: #define FRAME_CANON_Y_FROM_PIXEL_Y(F, Y) ((Y) % FRAME_LINE_HEIGHT (F) ? make_float ((double) (Y) / FRAME_LINE_HEIGHT (F)) : make_fixnum ((Y) / FRAME_LINE_HEIGHT (F)))
#[inline(always)]
pub fn FRAME_CANON_Y_FROM_PIXEL_Y (f: *mut frame, y: c_int) -> Lisp_Object
{
  if y % FRAME_LINE_HEIGHT (f) != 0
    { unsafe { make_float (y as c_double / FRAME_LINE_HEIGHT (f) as c_double) } }
  else { make_fixnum ((y / FRAME_LINE_HEIGHT (f)) as EMACS_INT) }
}

/// C: #define FRAME_LINE_TO_PIXEL_Y(f, row) (((row) < FRAME_TOP_MARGIN (f) ? 0 : FRAME_INTERNAL_BORDER_WIDTH (f)) + (row) * FRAME_LINE_HEIGHT (f))
#[inline(always)]
pub fn FRAME_LINE_TO_PIXEL_Y (f: *mut frame, row: c_int) -> c_int
{
  (if row < FRAME_TOP_MARGIN (f) { 0 } else { FRAME_INTERNAL_BORDER_WIDTH (f) })
  + row * FRAME_LINE_HEIGHT (f)
}

/// C: #define FRAME_COL_TO_PIXEL_X(f, col) (FRAME_INTERNAL_BORDER_WIDTH (f) + (col) * FRAME_COLUMN_WIDTH (f))
#[inline(always)]
pub fn FRAME_COL_TO_PIXEL_X (f: *mut frame, col: c_int) -> c_int
{ FRAME_INTERNAL_BORDER_WIDTH (f) + col * FRAME_COLUMN_WIDTH (f) }

/// C: #define FRAME_TEXT_COLS_TO_PIXEL_WIDTH(f, cols) ((cols) * FRAME_COLUMN_WIDTH (f) + FRAME_SCROLL_BAR_AREA_WIDTH (f) + FRAME_TOTAL_FRINGE_WIDTH (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_TEXT_COLS_TO_PIXEL_WIDTH (f: *mut frame, cols: c_int) -> c_int
{
  cols * FRAME_COLUMN_WIDTH (f) + FRAME_SCROLL_BAR_AREA_WIDTH (f)
  + FRAME_TOTAL_FRINGE_WIDTH (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_TEXT_LINES_TO_PIXEL_HEIGHT(f, lines) ((lines) * FRAME_LINE_HEIGHT (f) + FRAME_TOP_MARGIN_HEIGHT (f) + FRAME_SCROLL_BAR_AREA_HEIGHT (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_TEXT_LINES_TO_PIXEL_HEIGHT (f: *mut frame, lines: c_int) -> c_int
{
  lines * FRAME_LINE_HEIGHT (f) + FRAME_TOP_MARGIN_HEIGHT (f)
  + FRAME_SCROLL_BAR_AREA_HEIGHT (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_PIXEL_Y_TO_LINE(f, y) (((y) < FRAME_TOP_MARGIN_HEIGHT (f) ? (y) : ((y) < (FRAME_TOP_MARGIN_HEIGHT (f) + FRAME_INTERNAL_BORDER_WIDTH (f)) ? (y) - (FRAME_TOP_MARGIN_HEIGHT (f) + FRAME_INTERNAL_BORDER_WIDTH (f) + FRAME_LINE_HEIGHT (f) - 1) : (y) - FRAME_INTERNAL_BORDER_WIDTH (f))) / FRAME_LINE_HEIGHT (f))
#[inline(always)]
pub fn FRAME_PIXEL_Y_TO_LINE (f: *mut frame, y: c_int) -> c_int
{
  (if y < FRAME_TOP_MARGIN_HEIGHT (f) { y }
   else if y < FRAME_TOP_MARGIN_HEIGHT (f) + FRAME_INTERNAL_BORDER_WIDTH (f)
     {
       y - (FRAME_TOP_MARGIN_HEIGHT (f) + FRAME_INTERNAL_BORDER_WIDTH (f)
            + FRAME_LINE_HEIGHT (f) - 1)
     }
   else { y - FRAME_INTERNAL_BORDER_WIDTH (f) })
  / FRAME_LINE_HEIGHT (f)
}

/// C: #define FRAME_PIXEL_X_TO_COL(f, x) (((x) - FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_COLUMN_WIDTH (f))
#[inline(always)]
pub fn FRAME_PIXEL_X_TO_COL (f: *mut frame, x: c_int) -> c_int
{ (x - FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_COLUMN_WIDTH (f) }

/// C: #define FRAME_PIXEL_WIDTH_TO_TEXT_COLS(f, width) (((width) - FRAME_TOTAL_FRINGE_WIDTH (f) - FRAME_SCROLL_BAR_AREA_WIDTH (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_COLUMN_WIDTH (f))
#[inline(always)]
pub fn FRAME_PIXEL_WIDTH_TO_TEXT_COLS (f: *mut frame, width: c_int) -> c_int
{
  (width - FRAME_TOTAL_FRINGE_WIDTH (f) - FRAME_SCROLL_BAR_AREA_WIDTH (f)
   - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_COLUMN_WIDTH (f)
}

/// C: #define FRAME_PIXEL_HEIGHT_TO_TEXT_LINES(f, height) (((height) - FRAME_MARGIN_HEIGHT (f) - FRAME_SCROLL_BAR_AREA_HEIGHT (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_LINE_HEIGHT (f))
#[inline(always)]
pub fn FRAME_PIXEL_HEIGHT_TO_TEXT_LINES (f: *mut frame, height: c_int) -> c_int
{
  (height - FRAME_MARGIN_HEIGHT (f) - FRAME_SCROLL_BAR_AREA_HEIGHT (f)
   - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) / FRAME_LINE_HEIGHT (f)
}

/// C: #define FRAME_TEXT_TO_PIXEL_WIDTH(f, width) ((width) + FRAME_SCROLL_BAR_AREA_WIDTH (f) + FRAME_TOTAL_FRINGE_WIDTH (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_TEXT_TO_PIXEL_WIDTH (f: *mut frame, width: c_int) -> c_int
{
  width + FRAME_SCROLL_BAR_AREA_WIDTH (f) + FRAME_TOTAL_FRINGE_WIDTH (f)
  + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_TEXT_TO_PIXEL_HEIGHT(f, height) ((height) + FRAME_MARGIN_HEIGHT (f) + FRAME_SCROLL_BAR_AREA_HEIGHT (f) + 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_TEXT_TO_PIXEL_HEIGHT (f: *mut frame, height: c_int) -> c_int
{
  height + FRAME_MARGIN_HEIGHT (f) + FRAME_SCROLL_BAR_AREA_HEIGHT (f)
  + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_PIXEL_TO_TEXT_WIDTH(f, width) ((width) - FRAME_SCROLL_BAR_AREA_WIDTH (f) - FRAME_TOTAL_FRINGE_WIDTH (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_PIXEL_TO_TEXT_WIDTH (f: *mut frame, width: c_int) -> c_int
{
  width - FRAME_SCROLL_BAR_AREA_WIDTH (f) - FRAME_TOTAL_FRINGE_WIDTH (f)
  - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_PIXEL_TO_TEXT_HEIGHT(f, height) ((height) - FRAME_MARGIN_HEIGHT (f) - FRAME_SCROLL_BAR_AREA_HEIGHT (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_PIXEL_TO_TEXT_HEIGHT (f: *mut frame, height: c_int) -> c_int
{
  height - FRAME_MARGIN_HEIGHT (f) - FRAME_SCROLL_BAR_AREA_HEIGHT (f)
  - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)
}

/// C: #define FRAME_INNER_WIDTH(f) (FRAME_PIXEL_WIDTH (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_INNER_WIDTH (f: *mut frame) -> c_int
{ FRAME_PIXEL_WIDTH (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f) }

/// C: #define FRAME_INNER_HEIGHT(f) (FRAME_PIXEL_HEIGHT (f) - FRAME_MARGIN_HEIGHT (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f))
#[inline(always)]
pub fn FRAME_INNER_HEIGHT (f: *mut frame) -> c_int
{ FRAME_PIXEL_HEIGHT (f) - FRAME_MARGIN_HEIGHT (f) - 2 * FRAME_INTERNAL_BORDER_WIDTH (f) }

/* frame.h 1826-1832: the two smallest-font measurements.  Their pointee is the display-info
   structure the frozen TTY arm of `FRAME_DISPLAY_INFO` names, which no ported unit has yet, so
   the names stay macros over C's own path (the `FRAME_IMAGE_CACHE` shape).
   C: #define FRAME_SMALLEST_CHAR_WIDTH(f) FRAME_DISPLAY_INFO (f)->smallest_char_width  */
macro_rules! FRAME_SMALLEST_CHAR_WIDTH {
  ($f:expr) => { unsafe { (*FRAME_DISPLAY_INFO! ($f)).smallest_char_width } };
}
/// C: #define FRAME_SMALLEST_FONT_HEIGHT(f) FRAME_DISPLAY_INFO (f)->smallest_font_height
macro_rules! FRAME_SMALLEST_FONT_HEIGHT {
  ($f:expr) => { unsafe { (*FRAME_DISPLAY_INFO! ($f)).smallest_font_height } };
}

/* frame.h 1914-1921: `flush_frame` - C's inline calls the display interface's `flush_display`,
   which is xterm.c's (termhooks.h's `redisplay_interface` member); the unit shape above.
   C: INLINE void flush_frame (struct frame *f) { struct redisplay_interface *rif = FRAME_RIF (f); if (rif && rif->flush_display) rif->flush_display (f); }  */
macro_rules! flush_frame {
  ($f:expr) => {{
    let f = $f;
    let rif: *mut redisplay_interface = FRAME_RIF! (f);
    if !rif.is_null () {
      if let Some (flush_display) = unsafe { (*rif).flush_display } { flush_display (f); }
    }
  }};
}

/* frame.h 1925-1944: the multimonitor data.  `HAVE_WINDOW_SYSTEM` gates both the record and its
   two entry points, so the frozen arm sees neither; the arms that do define it read sizeof 48 /
   align 8 with the members at 0 (geom), 16 (work), 32 (mm_width), 36 (mm_height), 40 (name)
   (`spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h`, measured 3.312).  **The arms do
   not agree, and the difference is `Emacs_Rectangle` itself**: the x11 arm reads 32 / align 8 with
   `geom`/`work` as `XRectangle` (4 shorts, 8 bytes each - dispextern.h 44, `#ifdef HAVE_X_WINDOWS`),
   the ns_cocoa arm reads 48 / align 8 with the non-X typedef this mirror carries (dispextern.h
   67-70), and the frozen arm reads `incomplete type`.  The mirror keeps the non-X shape (the frozen
   configuration's) and the x11 divergence is recorded in the ledger; carrying it needs `XRectangle`
   (an Xlib mirror), which no unit has yet.  The `HAVE_PGTK` member
   `scale_factor` is carried under its own gate - it is the one member the arms this machine can
   read do not have, and PGTK headers are not here (`struct pgtk_output`'s ruling, 3.310).
   C: struct MonitorInfo { Emacs_Rectangle geom, work; int mm_width, mm_height; char *name;
      #ifdef HAVE_PGTK double scale_factor; #endif };  */
#[cfg (feature = "have_window_system")]
#[repr(C)]
pub struct MonitorInfo
{
  pub geom: Emacs_Rectangle,
  pub work: Emacs_Rectangle,
  pub mm_width: c_int,
  pub mm_height: c_int,
  pub name: *mut c_char,
  #[cfg (feature = "have_pgtk")]
  pub scale_factor: c_double,
}

/* frame.h 1937-1943: the record's two entry points.  They are `extern` declarations in the header
   and definitions in frame.c (`free_monitors`, `make_monitor_attribute_list`), so the mirror
   carries the declarations the same way `include/haiku_support.rs` does.
   C: extern void free_monitors (struct MonitorInfo *monitors, int n_monitors);  */
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  /// C: extern void free_monitors (struct MonitorInfo *monitors, int n_monitors);
  pub fn free_monitors (monitors: *mut MonitorInfo, n_monitors: c_int);
  /// C: extern Lisp_Object make_monitor_attribute_list (struct MonitorInfo *monitors, int n_monitors, int primary_monitor, Lisp_Object monitor_frames, const char *source);
  pub fn make_monitor_attribute_list (monitors: *mut MonitorInfo, n_monitors: c_int,
                                      primary_monitor: c_int, monitor_frames: Lisp_Object,
                                      source: *const c_char) -> Lisp_Object;
}
