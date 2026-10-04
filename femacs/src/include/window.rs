// femacs: port of src/window.h @ fd7afced403.
//
// WIP mirror (PORTING.md P-66): the two types.  `window.h` includes `dispextern.h` (line 23), so
// C sees `struct glyph_matrix` complete here — the port cannot yet, because the display chain's
// matrix/face segment is still ahead of it, so those two pointers carry the P-72 rule ③
// degradation until it lands.  `HAVE_TEXT_CONVERSION` and `HAVE_WINDOW_SYSTEM` are `#undef` in the
// frozen `config.h`, so the field scanner's live-line filter (P-73) kept the members they gate out
// of the first pass; the `HAVE_WINDOW_SYSTEM` four this mirror now carries are gated with it, their
// positions read from `docs/baseline/window-layout.tsv` (P-82: frozen 520, x11 536, ns_cocoa 536 -
// the two window-system arms add exactly these four, at 452/456/460/464 and none of the flag run
// moves, which is why no numbering is gated here).  `ephemeral_last_point` is the next one.  The
// 95 macros, 16 inlines and 9 file-scope objects land next.
//
// `tools/check-cond-eval.py` reports zero disagreements for this header, so the branch set above
// is the frozen configuration's.

// window.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const WINDOW_H_INCLUDED: bool = true;

// window.h 91-100.  Measured: 16 bytes, alignment 4.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cursor_pos
{
  /* Pixel position.  These are always window relative.  */
  pub x: c_int,                             // window.h 94
  pub y: c_int,
  /* Glyph matrix position.  */
  pub hpos: c_int,                          // 97
  pub vpos: c_int,
}

/* Most code should use this macro to set Lisp fields in struct window.  */
// window.h 134-453.  Measured: sizeof 520, alignof 8 — 27 Lisp_Objects from offset 8 to 223, the
// two matrix pointers at 224 and 232, `use_time` 240, `sequence_number` 248, `change_stamp` 256,
// the edge/width integers 260…308, `hscroll` 312, the bookkeeping that ends at 396, the three
// `cursor_pos` records 400/416/432, the fringe/margin/decoration integers 448…488,
// `window_end_pos` 488, `window_end_vpos` 496, one 4-byte bitfield unit at 500 (`ff ff 07 00`
// with all nineteen set), `vscroll` 504 and `window_end_bytepos` 512.  Every offset is asserted.
#[repr(C)]
pub struct window
{
  pub header: vectorlike_header,            // window.h 136

  pub frame: Lisp_Object,                   // 139
  pub next: Lisp_Object,
  pub prev: Lisp_Object,
  pub parent: Lisp_Object,
  pub normal_lines: Lisp_Object,
  pub normal_cols: Lisp_Object,
  pub new_total: Lisp_Object,
  pub new_normal: Lisp_Object,
  pub new_pixel: Lisp_Object,
  pub contents: Lisp_Object,
  pub prev_buffers: Lisp_Object,
  pub next_buffers: Lisp_Object,
  pub old_buffer: Lisp_Object,
  pub start: Lisp_Object,
  pub pointm: Lisp_Object,
  pub old_pointm: Lisp_Object,
  pub temslot: Lisp_Object,
  pub vertical_scroll_bar: Lisp_Object,
  pub vertical_scroll_bar_type: Lisp_Object,
  pub horizontal_scroll_bar: Lisp_Object,
  pub horizontal_scroll_bar_type: Lisp_Object,
  pub display_table: Lisp_Object,
  pub dedicated: Lisp_Object,
  pub combination_limit: Lisp_Object,
  pub window_parameters: Lisp_Object,
  pub cursor_type: Lisp_Object,
  pub mode_line_help_echo: Lisp_Object,

  /* The matrices this window's glyphs are stored in.  */
  pub current_matrix: *mut c_void,          /// C: `struct glyph_matrix *current_matrix;`
  pub desired_matrix: *mut c_void,          /// C: `struct glyph_matrix *desired_matrix;`

  /* Timestamps of the last redisplay and of the last change to the window's contents.  */
  pub use_time: EMACS_INT,                  // 336
  pub sequence_number: EMACS_INT,
  pub change_stamp: c_int,

  /* The window's edges, in pixels and in columns/lines.  */
  pub pixel_left: c_int,
  pub pixel_top: c_int,
  pub left_col: c_int,
  pub top_line: c_int,
  pub pixel_width: c_int,
  pub pixel_height: c_int,
  pub old_pixel_width: c_int,
  pub old_pixel_height: c_int,
  pub old_body_pixel_width: c_int,
  pub old_body_pixel_height: c_int,
  pub total_cols: c_int,
  pub total_lines: c_int,

  /* The horizontal scroll amounts, and when the window's text last changed.  */
  pub hscroll: ptrdiff_t,                   // 312
  pub min_hscroll: ptrdiff_t,
  pub hscroll_whole: ptrdiff_t,
  pub last_modified: modiff_count,
  pub last_overlay_modified: modiff_count,

  /* The last point, mark and line-number bookkeeping.  */
  pub last_point: ptrdiff_t,
  pub last_mark: ptrdiff_t,
  pub base_line_number: ptrdiff_t,
  pub base_line_pos: ptrdiff_t,
  pub column_number_displayed: ptrdiff_t,

  pub nrows_scale_factor: c_int,
  pub ncols_scale_factor: c_int,

  /* The cursor's position in the window, in three flavours.  */
  pub cursor: cursor_pos,                   // 400
  pub phys_cursor: cursor_pos,              // 416
  pub output_cursor: cursor_pos,            // 432
  pub last_cursor_vpos: c_int,              // 448

  /* window.h 339-351: the cursor C remembers for undrawing, under its own gate.  */
  #[cfg (feature = "have_window_system")]
  pub phys_cursor_type: text_cursor_kinds,   // 344; measured 452 in both window-system arms
  #[cfg (feature = "have_window_system")]
  pub phys_cursor_width: c_int,              // 347 (456)
  #[cfg (feature = "have_window_system")]
  pub phys_cursor_ascent: c_int,             // 350 (460)
  #[cfg (feature = "have_window_system")]
  pub phys_cursor_height: c_int,             // 350 (464)

  /* Fringe and margin widths, and the heights of the window's decorations.  */
  pub left_fringe_width: c_int,             // 452
  pub right_fringe_width: c_int,
  pub left_margin_cols: c_int,
  pub right_margin_cols: c_int,
  pub scroll_bar_width: c_int,
  pub scroll_bar_height: c_int,
  pub mode_line_height: c_int,
  pub header_line_height: c_int,
  pub tab_line_height: c_int,

  /* Where the window's text ends.  */
  pub window_end_pos: ptrdiff_t,            // 488
  pub window_end_vpos: c_int,               // 496

  // window.h 392-410: nineteen `bool_bf : 1` members share one 4-byte unit at offset 500; with
  // all nineteen set the bytes are `ff ff 07 00` (= 0x0007FFFF, nineteen bits), which is what
  // fixes each member's bit (PORTING.md 1.3 names the unit and the accessors mask it).
  pub bits: u32,                            // 500

  pub vscroll: c_int,                       // 504
  pub window_end_bytepos: ptrdiff_t,        // 512
}

// The nineteen flags of that unit, in C's declaration order (window.h 392-410).
pub const WINDOW_MINI_SHIFT: u32 = 0;
pub const WINDOW_HORIZONTAL_SHIFT: u32 = 1;
pub const WINDOW_UPDATE_MODE_LINE_SHIFT: u32 = 2;
pub const WINDOW_LAST_HAD_STAR_SHIFT: u32 = 3;
pub const WINDOW_START_AT_LINE_BEG_SHIFT: u32 = 4;
pub const WINDOW_FORCE_START_SHIFT: u32 = 5;
pub const WINDOW_OPTIONAL_NEW_START_SHIFT: u32 = 6;
pub const WINDOW_PHYS_CURSOR_ON_P_SHIFT: u32 = 7;
pub const WINDOW_CURSOR_OFF_P_SHIFT: u32 = 8;
pub const WINDOW_LAST_CURSOR_OFF_P_SHIFT: u32 = 9;
pub const WINDOW_MUST_BE_UPDATED_P_SHIFT: u32 = 10;
pub const WINDOW_PSEUDO_WINDOW_P_SHIFT: u32 = 11;
pub const WINDOW_FRINGES_OUTSIDE_MARGINS_SHIFT: u32 = 12;
pub const WINDOW_FRINGES_PERSISTENT_SHIFT: u32 = 13;
pub const WINDOW_SCROLL_BARS_PERSISTENT_SHIFT: u32 = 14;
pub const WINDOW_WINDOW_END_VALID_SHIFT: u32 = 15;
pub const WINDOW_REDISPLAY_SHIFT: u32 = 16;
pub const WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT: u32 = 17;
pub const WINDOW_PRESERVE_VSCROLL_P_SHIFT: u32 = 18;

// window.h 476-494.
#[inline(always)]
pub fn WINDOWP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_WINDOW) }

#[inline(always)]
pub fn CHECK_WINDOW (x: Lisp_Object) { CHECK_TYPE (WINDOWP (x), Qwindowp (), x) }

#[inline(always)]
pub fn XWINDOW (a: Lisp_Object) -> *mut window
{
  eassert! (WINDOWP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, window)
}

/* Most code should use these functions to set Lisp fields in struct
   window.  */
// window.h 499-565: the thirteen setters.
#[inline(always)]
pub fn wset_frame (w: *mut window, val: Lisp_Object) { unsafe { (*w).frame = val; } }
#[inline(always)]
pub fn wset_next (w: *mut window, val: Lisp_Object) { unsafe { (*w).next = val; } }
#[inline(always)]
pub fn wset_prev (w: *mut window, val: Lisp_Object) { unsafe { (*w).prev = val; } }
#[inline(always)]
pub fn wset_mode_line_help_echo (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).mode_line_help_echo = val; } }
#[inline(always)]
pub fn wset_new_pixel (w: *mut window, val: Lisp_Object) { unsafe { (*w).new_pixel = val; } }
#[inline(always)]
pub fn wset_vertical_scroll_bar (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).vertical_scroll_bar = val; } }
#[inline(always)]
pub fn wset_horizontal_scroll_bar (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).horizontal_scroll_bar = val; } }
#[inline(always)]
pub fn wset_horizontal_scroll_bar_type (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).horizontal_scroll_bar_type = val; } }
#[inline(always)]
pub fn wset_cursor_type (w: *mut window, val: Lisp_Object) { unsafe { (*w).cursor_type = val; } }
#[inline(always)]
pub fn wset_prev_buffers (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).prev_buffers = val; } }
#[inline(always)]
pub fn wset_next_buffers (w: *mut window, val: Lisp_Object)
{ unsafe { (*w).next_buffers = val; } }

// window.h 1246-1255: record where the window's cursor is being drawn this update.
#[inline(always)]
pub fn output_cursor_to (w: *mut window, vpos: c_int, hpos: c_int, y: c_int, x: c_int)
{
  eassert! (!w.is_null ());
  unsafe {
    (*w).output_cursor.hpos = hpos;
    (*w).output_cursor.vpos = vpos;
    (*w).output_cursor.x = x;
    (*w).output_cursor.y = y;
  }
}

/* Return true, if overlay OV's properties should have an effect in
   window W. */
// window.h 1258-1266.
#[inline(always)]
pub fn overlay_matches_window (w: *const window, ov: Lisp_Object) -> bool
{
  eassert! (OVERLAYP (ov));
  let window_ = unsafe { Foverlay_get (ov, Qwindow ()) };
  !WINDOWP (window_) || std::ptr::eq (unsafe { XWINDOW (window_) }, w as *mut window)
}

// window.h 1100-1206: the file-scope objects (the .c files define them).
unsafe extern "C" { pub static mut selected_window: Lisp_Object; }             // 1100
unsafe extern "C" { pub static mut window_select_count: EMACS_INT; }           // 1106
unsafe extern "C" { pub static mut minibuf_window: Lisp_Object; }              // 1112
unsafe extern "C" { pub static mut minibuf_selected_window: Lisp_Object; }     // 1117
unsafe extern "C" { pub static mut last_mouse_window: Lisp_Object; }           // 1121
unsafe extern "C" { pub static mut echo_area_window: Lisp_Object; }            // 1146
unsafe extern "C" { pub static mut update_mode_lines: c_int; }                 // 1154
unsafe extern "C" { pub static mut windows_or_buffers_changed: c_int; }        // 1161
unsafe extern "C" { pub static mut Vwindow_list: Lisp_Object; }                // 1206

// window.h 1123-1238: the header's 29 function prototypes.  Declared in a module with a glob
// import (the `window_redisplay_decls` shape below) because nine of them are *defined* in this
// tree - `src/window.rs` defines `make_window`, `window_from_coordinates`, `decode_live_window`,
// `window_point`, `window_list`, `mru_rooted_frame`, `init_window_once`, `init_window` and
// `syms_of_window`, and `src/xdisp.rs` defines several more - and a definition at the unit's
// crate root would collide with a crate-root declaration (E0428, PORTING.md 0.1-10); the unit's
// item shadows the glob import instead.  A unit that only *calls* one of these now finds the
// declaration here instead of a use-site copy: this batch removed 26 such copies (P-26).
// Parameters whose type this mirror cannot name degrade per P-72 rule 3 and are spelled
// `*mut c_void`, exactly as `fset_redisplay`/`bset_redisplay` below already spell `struct
// frame *`/`struct buffer *`; the callers cast.
// `window_from_coordinates`/`window_relative_x_coord` take C's `enum window_part` as `c_int`:
// three units include this mirror without including dispextern.rs (`editfns`, `insdel`,
// `textprop`), and C's window.h only needs the tag, while the port's `window_part` is an
// alias to `c_int` (dispextern.rs 96) - so the mirror names the underlying type and every
// caller passing a `window_part` is unaffected.  Measured: 3.345 spelled `window_part` here
// and build-tu src/editfns.c failed E0425 twice until 3.375 (status 3.375).
pub mod window_fn_decls {
  use super::*;

  unsafe extern "C" {
    /// C: window.h 1123: `make_window` - `Lisp_Object make_window (void)`
    pub fn make_window () -> Lisp_Object;
    /// C: window.h 1124: `window_from_coordinates` - `Lisp_Object window_from_coordinates (struct frame *, int, int, enum window_part *, bool, bool, bool)`
    pub fn window_from_coordinates (f: *mut c_void, x: c_int, y: c_int, part: *mut c_int,
                              menu_bar_p: bool, tab_bar_p: bool, tool_bar_p: bool) -> Lisp_Object;
    /// C: window.h 1126: `resize_frame_windows` - `void resize_frame_windows (struct frame *, int, bool)`
    pub fn resize_frame_windows (f: *mut c_void, size: c_int, horflag: bool);
    /// C: window.h 1127: `restore_window_configuration` - `void restore_window_configuration (Lisp_Object)`
    pub fn restore_window_configuration (configuration: Lisp_Object);
    /// C: window.h 1128: `restore_focus_frame` - `void restore_focus_frame (Lisp_Object)`
    pub fn restore_focus_frame (frame_and_focus_frame: Lisp_Object);
    /// C: window.h 1129: `delete_all_child_windows` - `void delete_all_child_windows (Lisp_Object)`
    pub fn delete_all_child_windows (window: Lisp_Object);
    /// C: window.h 1130: `grow_mini_window` - `void grow_mini_window (struct window *, int, int)`
    pub fn grow_mini_window (w: *mut window, delta: c_int, unit: c_int);
    /// C: window.h 1131: `shrink_mini_window` - `void shrink_mini_window (struct window *, int)`
    pub fn shrink_mini_window (w: *mut window, unit: c_int);
    /// C: window.h 1132: `window_relative_x_coord` - `int window_relative_x_coord (struct window *, enum window_part, int)`
    pub fn window_relative_x_coord (w: *mut window, part: c_int, x: c_int) -> c_int;
    /// C: window.h 1175: `bset_update_mode_line` - `void bset_update_mode_line (struct buffer *)`
    pub fn bset_update_mode_line (b: *mut c_void);
    /// C: window.h 1176: `wset_update_mode_line` - `void wset_update_mode_line (struct window *)`
    pub fn wset_update_mode_line (w: *mut window);
    /// C: window.h 1180: `redisplay_other_windows` - `void redisplay_other_windows (void)`
    pub fn redisplay_other_windows ();
    /// C: window.h 1208: `window_list` - `Lisp_Object window_list (void)`
    pub fn window_list () -> Lisp_Object;
    /// C: window.h 1209: `window_parameter` - `Lisp_Object window_parameter (struct window *, Lisp_Object parameter)`
    pub fn window_parameter (w: *mut window, parameter: Lisp_Object) -> Lisp_Object;
    /// C: window.h 1210: `*decode_live_window` - `struct window *decode_live_window (Lisp_Object)`
    pub fn decode_live_window (window: Lisp_Object) -> *mut window;
    /// C: window.h 1211: `*decode_any_window` - `struct window *decode_any_window (Lisp_Object)`
    pub fn decode_any_window (window: Lisp_Object) -> *mut window;
    /// C: window.h 1212: `mark_window_cursors_off` - `void mark_window_cursors_off (struct window *)`
    pub fn mark_window_cursors_off (w: *mut window);
    /// C: window.h 1216: `window_internal_height` - `int window_internal_height (struct window *)`
    pub fn window_internal_height (w: *mut window) -> c_int;
    /// C: window.h 1227: `temp_output_buffer_show` - `void temp_output_buffer_show (Lisp_Object)`
    pub fn temp_output_buffer_show (buf: Lisp_Object);
    /// C: window.h 1228: `replace_buffer_in_windows` - `void replace_buffer_in_windows (Lisp_Object)`
    pub fn replace_buffer_in_windows (buffer: Lisp_Object);
    /// C: window.h 1229: `replace_buffer_in_windows_safely` - `void replace_buffer_in_windows_safely (Lisp_Object)`
    pub fn replace_buffer_in_windows_safely (buffer: Lisp_Object);
    /// C: window.h 1231: `wset_buffer` - `void wset_buffer (struct window *, Lisp_Object)`
    pub fn wset_buffer (w: *mut window, buf: Lisp_Object);
    /// C: window.h 1232: `window_outdated` - `bool window_outdated (struct window *)`
    pub fn window_outdated (w: *mut window) -> bool;
    /// C: window.h 1233: `window_point` - `ptrdiff_t window_point (struct window *w)`
    pub fn window_point (w: *mut window) -> ptrdiff_t;
    /// C: window.h 1234: `window_discard_buffer_from_dead_windows` - `void window_discard_buffer_from_dead_windows (Lisp_Object)`
    pub fn window_discard_buffer_from_dead_windows (buffer: Lisp_Object);
    /// C: window.h 1235: `mru_rooted_frame` - `Lisp_Object mru_rooted_frame (struct frame *)`
    pub fn mru_rooted_frame (f: *mut c_void) -> Lisp_Object;
    /// C: window.h 1236: `init_window_once` - `void init_window_once (void)`
    pub fn init_window_once ();
    /// C: window.h 1237: `init_window` - `void init_window (void)`
    pub fn init_window ();
    /// C: window.h 1238: `syms_of_window` - `void syms_of_window (void)`
    pub fn syms_of_window ();
  }
}
use window_fn_decls::*;


/* True if W is a leaf window.  */
// window.h 567-1244's accessors.  C writes them as macros over the structure's fields; the port
// keeps the names and the values, and reads the bitfield members through the unit's shift
// constants (PORTING.md 1.3 — the note file records the field names that cannot occur).
#[inline(always)]
pub fn WINDOW_LEAF_P (w: *mut window) -> bool
{ BUFFERP (unsafe { (*w).contents }) }

#[inline(always)]
pub fn WINDOW_INTERNAL_P (w: *mut window) -> bool
{ WINDOWP (unsafe { (*w).contents }) }

#[inline(always)]
pub fn WINDOW_HORIZONTAL_COMBINATION_P (w: *mut window) -> bool
{ WINDOW_INTERNAL_P (w) && window_bit (w, WINDOW_HORIZONTAL_SHIFT) }

#[inline(always)]
pub fn WINDOW_VERTICAL_COMBINATION_P (w: *mut window) -> bool
{ WINDOW_INTERNAL_P (w) && !window_bit (w, WINDOW_HORIZONTAL_SHIFT) }

#[inline(always)]
pub fn WINDOW_PSEUDO_P (w: *mut window) -> bool
{ window_bit (w, WINDOW_PSEUDO_WINDOW_P_SHIFT) }

#[inline(always)]
pub fn WINDOW_BUFFER (w: *mut window) -> Lisp_Object
{ if WINDOW_LEAF_P (w) { unsafe { (*w).contents } } else { lisp_h_Qnil () } }

#[inline(always)]
pub fn WINDOW_BUFFER_LOCAL_VALUE (w: *mut window, v: Lisp_Object) -> Lisp_Object
{
  if BUFFERP (unsafe { (*w).contents })
    { unsafe { buffer_local_value (v, (*w).contents) } }
  else { lisp_h_Qnil () }
}

#[inline(always)]
pub fn WINDOW_PIXEL_WIDTH (w: *mut window) -> c_int { unsafe { (*w).pixel_width } }
#[inline(always)]
pub fn WINDOW_PIXEL_HEIGHT (w: *mut window) -> c_int { unsafe { (*w).pixel_height } }
#[inline(always)]
pub fn WINDOW_TOTAL_COLS (w: *mut window) -> c_int { unsafe { (*w).total_cols } }
#[inline(always)]
pub fn WINDOW_TOTAL_LINES (w: *mut window) -> c_int { unsafe { (*w).total_lines } }

#[inline(always)]
pub fn WINDOW_LEFT_EDGE_COL (w: *mut window) -> c_int { unsafe { (*w).left_col } }
#[inline(always)]
pub fn WINDOW_RIGHT_EDGE_COL (w: *mut window) -> c_int
{ WINDOW_LEFT_EDGE_COL (w) + WINDOW_TOTAL_COLS (w) }
#[inline(always)]
pub fn WINDOW_TOP_EDGE_LINE (w: *mut window) -> c_int { unsafe { (*w).top_line } }
#[inline(always)]
pub fn WINDOW_BOTTOM_EDGE_LINE (w: *mut window) -> c_int
{ WINDOW_TOP_EDGE_LINE (w) + WINDOW_TOTAL_LINES (w) }

#[inline(always)]
pub fn WINDOW_LEFT_PIXEL_EDGE (w: *mut window) -> c_int { unsafe { (*w).pixel_left } }
#[inline(always)]
pub fn WINDOW_RIGHT_PIXEL_EDGE (w: *mut window) -> c_int
{ WINDOW_LEFT_PIXEL_EDGE (w) + WINDOW_PIXEL_WIDTH (w) }
#[inline(always)]
pub fn WINDOW_TOP_PIXEL_EDGE (w: *mut window) -> c_int { unsafe { (*w).pixel_top } }
#[inline(always)]
pub fn WINDOW_BOTTOM_PIXEL_EDGE (w: *mut window) -> c_int
{ WINDOW_TOP_PIXEL_EDGE (w) + WINDOW_PIXEL_HEIGHT (w) }

#[inline(always)]
pub fn WINDOW_LEFTMOST_P (w: *mut window) -> bool { WINDOW_LEFT_PIXEL_EDGE (w) == 0 }
#[inline(always)]
pub fn WINDOW_TOPMOST_P (w: *mut window) -> bool { WINDOW_TOP_PIXEL_EDGE (w) == 0 }

// window.h 683-697: three bars this target never has (no window system).
#[inline(always)]
pub fn WINDOW_MENU_BAR_P (_w: *mut window) -> bool { false }
#[inline(always)]
pub fn WINDOW_TAB_BAR_P (_w: *mut window) -> bool { false }
#[inline(always)]
pub fn WINDOW_TOOL_BAR_P (_w: *mut window) -> bool { false }

#[inline(always)]
pub fn WINDOW_LEFT_MARGIN_COLS (w: *mut window) -> c_int { unsafe { (*w).left_margin_cols } }
#[inline(always)]
pub fn WINDOW_RIGHT_MARGIN_COLS (w: *mut window) -> c_int { unsafe { (*w).right_margin_cols } }
#[inline(always)]
pub fn WINDOW_MARGINS_COLS (w: *mut window) -> c_int
{ WINDOW_LEFT_MARGIN_COLS (w) + WINDOW_RIGHT_MARGIN_COLS (w) }

#[inline(always)]
pub fn WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w: *mut window) -> bool
{ window_bit (w, WINDOW_FRINGES_OUTSIDE_MARGINS_SHIFT) }

// window.h 1046-1051: this target has no horizontal scroll bars.
#[inline(always)]
pub fn WINDOW_HAS_HORIZONTAL_SCROLL_BAR (_w: *mut window) -> bool { false }

#[inline(always)]
pub fn WINDOW_VALID_P (window_: Lisp_Object) -> bool
{ WINDOWP (window_) && !NILP (unsafe { (*XWINDOW (window_)).contents }) }
#[inline(always)]
pub fn WINDOW_LIVE_P (window_: Lisp_Object) -> bool
{ WINDOWP (window_) && BUFFERP (unsafe { (*XWINDOW (window_)).contents }) }

/* Most code should use this macro to set Lisp fields in struct window.  */
// window.h 565-574: the mini-window predicates (`MINI_WINDOW_P`, `MINI_NON_ONLY_WINDOW_P`,
// `MINI_ONLY_WINDOW_P`; `mini` is one of the nineteen packed flags).
#[inline(always)]
pub fn MINI_WINDOW_P (w: *mut window) -> bool { window_bit (w, WINDOW_MINI_SHIFT) }

#[inline(always)]
pub fn MINI_NON_ONLY_WINDOW_P (w: *mut window) -> bool
{ MINI_WINDOW_P (w) && !NILP (unsafe { (*w).prev }) }

#[inline(always)]
pub fn MINI_ONLY_WINDOW_P (w: *mut window) -> bool
{ MINI_WINDOW_P (w) && NILP (unsafe { (*w).prev }) }

// window.h 668, 673: the minimum sizes (the C macros ignore their argument).
#[inline(always)]
pub fn MIN_SAFE_WINDOW_HEIGHT (_w: *mut window) -> c_int { 1 }
#[inline(always)]
pub fn MIN_SAFE_WINDOW_WIDTH (_w: *mut window) -> c_int { 2 }

// window.h 898-904: `#define USE_HORIZONTAL_SCROLL_BARS false` — object-like, so the port keeps a
// constant.
pub const USE_HORIZONTAL_SCROLL_BARS: bool = false;

#[inline(always)]
pub fn CHECK_VALID_WINDOW (window_: Lisp_Object)
{ CHECK_TYPE (WINDOW_VALID_P (window_), Qwindow_valid_p (), window_) }

#[inline(always)]
pub fn CHECK_LIVE_WINDOW (window_: Lisp_Object)
{ CHECK_TYPE (WINDOW_LIVE_P (window_), Qwindow_live_p (), window_) }

// The packed-flag reader the accessors above share.
#[inline(always)]
fn window_bit (w: *mut window, shift: u32) -> bool
{ (unsafe { (*w).bits }) >> shift & 1 != 0 }

// window.h 620-1092: the geometry family.  C writes all of it as macros, and the port keeps
// that shape (`macro_rules!`, PORTING.md 2.1) rather than the `#[inline(always)]` functions
// the accessors above use, because a macro's body is resolved where it is *expanded*: it may
// name frame.h's `struct frame`, `FRAME_*` and dispextern.h's `CURRENT_*_LINE_HEIGHT`/
// `window_box_left` exactly as C's does, and only in the units that have those headers — while
// a function body would have to compile in every unit that includes this mirror, and most of
// those units (C's own include chain says so) never opened frame.h or dispextern.h at all
// (PORTING.md P-72 rule ①/④ — the hazard include/dispextern.rs's `MATRIX_BOTTOM_TEXT_ROW`
// records).  C's bodies also mention their window argument once per use, and a macro keeps
// that evaluation count (P-16); `NAME! (…)` is the call in Rust, whose `!` the token check
// ignores.
//
// Two groups still name a construct of a header whose mirror is behind:
//
//   * `WINDOW_FRAME` is C's frame.h 929 and frame.h's mirror has not reached it, so it is
//     named here at its use point (PORTING.md P-72 rule ③; the ledger's Decisions record it).
//     C's `((w)->frame)` is a *place*, so it stays a macro.
//   * `CURRENT_MODE_LINE_HEIGHT`/`CURRENT_HEADER_LINE_HEIGHT`/`CURRENT_TAB_LINE_HEIGHT` and the
//     `MATRIX_*_LINE_HEIGHT`/`*_ACTIVE_FACE_ID` family they call are dispextern.h's (1553-1660)
//     and now live in include/dispextern.rs, next to the matrix rows and the face ids already
//     there; `WINDOW_TEXT_TO_FRAME_PIXEL_X` calls dispextern.h 3579's `window_box_left`.

// window.h 620.
/// C: frame.h 929: `#define WINDOW_FRAME(w) ((w)->frame)`
macro_rules! WINDOW_FRAME { ($w:expr) => { (*$w).frame }; }
macro_rules! WINDOW_XFRAME { ($w:expr) => { XFRAME (WINDOW_FRAME! ($w)) }; }

// window.h 639-643: the canonical metrics of W's frame.
macro_rules! WINDOW_FRAME_COLUMN_WIDTH {
  ($w:expr) => { FRAME_COLUMN_WIDTH (WINDOW_XFRAME! ($w)) };
}
macro_rules! WINDOW_FRAME_LINE_HEIGHT {
  ($w:expr) => { FRAME_LINE_HEIGHT (WINDOW_XFRAME! ($w)) };
}

// window.h 668-676: the smallest acceptable dimensions (C's `2` and the line height).
macro_rules! MIN_SAFE_WINDOW_PIXEL_WIDTH {
  ($w:expr) => { 2 * WINDOW_FRAME_COLUMN_WIDTH! ($w) };
}
macro_rules! MIN_SAFE_WINDOW_PIXEL_HEIGHT {
  ($w:expr) => { WINDOW_FRAME_LINE_HEIGHT! ($w) };
}

// window.h 679-703: the "outermost" and "full width" predicates.
macro_rules! WINDOW_RIGHTMOST_P {
  ($w:expr) => {
    WINDOW_RIGHT_PIXEL_EDGE ($w)
    == WINDOW_RIGHT_PIXEL_EDGE (XWINDOW (FRAME_ROOT_WINDOW (WINDOW_XFRAME! ($w))))
  };
}
macro_rules! WINDOW_BOTTOMMOST_P {
  ($w:expr) => {
    WINDOW_BOTTOM_PIXEL_EDGE ($w)
    == WINDOW_BOTTOM_PIXEL_EDGE (XWINDOW (FRAME_ROOT_WINDOW (WINDOW_XFRAME! ($w))))
  };
}
macro_rules! WINDOW_FULL_WIDTH_P {
  ($w:expr) => {
    WINDOW_PIXEL_WIDTH ($w)
    == WINDOW_PIXEL_WIDTH (XWINDOW (FRAME_ROOT_WINDOW (WINDOW_XFRAME! ($w))))
  };
}

// window.h 707-718: the divider widths.
macro_rules! WINDOW_RIGHT_DIVIDER_WIDTH {
  ($w:expr) => {
    if WINDOW_RIGHTMOST_P! ($w) { 0 }
    else { FRAME_RIGHT_DIVIDER_WIDTH (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_BOTTOM_DIVIDER_WIDTH {
  ($w:expr) => {
    if (WINDOW_BOTTOMMOST_P! ($w)
        && NILP ((*XWINDOW (FRAME_ROOT_WINDOW (WINDOW_XFRAME! ($w)))).next))
       || EQ ((*$w).prev, FRAME_ROOT_WINDOW (WINDOW_XFRAME! ($w)))
       || WINDOW_PSEUDO_P ($w)
      { 0 }
    else { FRAME_BOTTOM_DIVIDER_WIDTH (WINDOW_XFRAME! ($w)) }
  };
}

// window.h 758-803: the frame-relative edges.  The menu/tab/tool-bar predicates are the
// mirrors' `false` accessors (no window system on this target).
macro_rules! WINDOW_LEFT_EDGE_X {
  ($w:expr) => {
    FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) + WINDOW_LEFT_PIXEL_EDGE ($w)
  };
}
macro_rules! WINDOW_RIGHT_EDGE_X {
  ($w:expr) => {
    FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) + WINDOW_RIGHT_PIXEL_EDGE ($w)
  };
}
macro_rules! WINDOW_TOP_EDGE_Y {
  ($w:expr) => {
    (if WINDOW_MENU_BAR_P ($w) || WINDOW_TAB_BAR_P ($w) || WINDOW_TOOL_BAR_P ($w)
       { 0 } else { FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) })
    + WINDOW_TOP_PIXEL_EDGE ($w)
  };
}
macro_rules! WINDOW_BOTTOM_EDGE_Y {
  ($w:expr) => {
    (if WINDOW_MENU_BAR_P ($w) || WINDOW_TAB_BAR_P ($w) || WINDOW_TOOL_BAR_P ($w)
       { 0 } else { FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) })
    + WINDOW_BOTTOM_PIXEL_EDGE ($w)
  };
}

// window.h 810-835: the text-box edges (the scroll bar and divider on the right are out).
macro_rules! WINDOW_BOX_LEFT_PIXEL_EDGE {
  ($w:expr) => { WINDOW_LEFT_PIXEL_EDGE ($w) + WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH! ($w) };
}
macro_rules! WINDOW_BOX_RIGHT_PIXEL_EDGE {
  ($w:expr) => {
    WINDOW_RIGHT_PIXEL_EDGE ($w)
    - WINDOW_RIGHT_DIVIDER_WIDTH! ($w)
    - WINDOW_RIGHT_SCROLL_BAR_AREA_WIDTH! ($w)
  };
}
macro_rules! WINDOW_BOX_LEFT_EDGE_X {
  ($w:expr) => {
    FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) + WINDOW_BOX_LEFT_PIXEL_EDGE! ($w)
  };
}
macro_rules! WINDOW_BOX_RIGHT_EDGE_X {
  ($w:expr) => {
    FRAME_INTERNAL_BORDER_WIDTH (WINDOW_XFRAME! ($w)) + WINDOW_BOX_RIGHT_PIXEL_EDGE! ($w)
  };
}

// window.h 847-869: the margin and fringe pixel widths.
macro_rules! WINDOW_LEFT_MARGIN_WIDTH {
  ($w:expr) => { (*$w).left_margin_cols * WINDOW_FRAME_COLUMN_WIDTH! ($w) };
}
macro_rules! WINDOW_RIGHT_MARGIN_WIDTH {
  ($w:expr) => { (*$w).right_margin_cols * WINDOW_FRAME_COLUMN_WIDTH! ($w) };
}
macro_rules! WINDOW_MARGINS_WIDTH {
  ($w:expr) => { WINDOW_LEFT_MARGIN_WIDTH! ($w) + WINDOW_RIGHT_MARGIN_WIDTH! ($w) };
}
macro_rules! WINDOW_LEFT_FRINGE_WIDTH {
  ($w:expr) => {
    if (*$w).left_fringe_width >= 0 { (*$w).left_fringe_width }
    else { FRAME_LEFT_FRINGE_WIDTH (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_RIGHT_FRINGE_WIDTH {
  ($w:expr) => {
    if (*$w).right_fringe_width >= 0 { (*$w).right_fringe_width }
    else { FRAME_RIGHT_FRINGE_WIDTH (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_FRINGES_WIDTH {
  ($w:expr) => { WINDOW_LEFT_FRINGE_WIDTH! ($w) + WINDOW_RIGHT_FRINGE_WIDTH! ($w) };
}

// window.h 877-901: which side the vertical scroll bar is on, and whether there is one.
macro_rules! WINDOW_VERTICAL_SCROLL_BAR_TYPE {
  ($w:expr) => {
    if WINDOW_PSEUDO_P ($w) { vertical_scroll_bar_none }
    else if EQ ((*$w).vertical_scroll_bar_type, Qt ())
      { FRAME_VERTICAL_SCROLL_BAR_TYPE (WINDOW_XFRAME! ($w)) }
    else if EQ ((*$w).vertical_scroll_bar_type, Qleft ()) { vertical_scroll_bar_left }
    else if EQ ((*$w).vertical_scroll_bar_type, Qright ()) { vertical_scroll_bar_right }
    else { vertical_scroll_bar_none }
  };
}
macro_rules! WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT {
  ($w:expr) => { WINDOW_VERTICAL_SCROLL_BAR_TYPE! ($w) == vertical_scroll_bar_left };
}
macro_rules! WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT {
  ($w:expr) => { WINDOW_VERTICAL_SCROLL_BAR_TYPE! ($w) == vertical_scroll_bar_right };
}
macro_rules! WINDOW_HAS_VERTICAL_SCROLL_BAR {
  ($w:expr) => {
    WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! ($w)
    || WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT! ($w)
  };
}

// window.h 926-983: the scroll bar's width in pixels and columns, and where its area starts.
// `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` is the mirror's `false` accessor (this target has none).
macro_rules! WINDOW_CONFIG_SCROLL_BAR_WIDTH {
  ($w:expr) => {
    if (*$w).scroll_bar_width >= 0 { (*$w).scroll_bar_width }
    else { FRAME_CONFIG_SCROLL_BAR_WIDTH (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_CONFIG_SCROLL_BAR_COLS {
  ($w:expr) => {
    if (*$w).scroll_bar_width >= 0
      { ((*$w).scroll_bar_width + WINDOW_FRAME_COLUMN_WIDTH! ($w) - 1)
          / WINDOW_FRAME_COLUMN_WIDTH! ($w) }
    else { FRAME_CONFIG_SCROLL_BAR_COLS (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_LEFT_SCROLL_BAR_COLS {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! ($w) { WINDOW_CONFIG_SCROLL_BAR_COLS! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_RIGHT_SCROLL_BAR_COLS {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT! ($w) { WINDOW_CONFIG_SCROLL_BAR_COLS! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_SCROLL_BAR_COLS {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR! ($w) { WINDOW_CONFIG_SCROLL_BAR_COLS! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! ($w) { WINDOW_CONFIG_SCROLL_BAR_WIDTH! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_RIGHT_SCROLL_BAR_AREA_WIDTH {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT! ($w) { WINDOW_CONFIG_SCROLL_BAR_WIDTH! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_SCROLL_BAR_AREA_WIDTH {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR! ($w) { WINDOW_CONFIG_SCROLL_BAR_WIDTH! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_SCROLL_BAR_AREA_X {
  ($w:expr) => {
    if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT! ($w) { WINDOW_BOX_RIGHT_EDGE_X! ($w) }
    else { WINDOW_LEFT_EDGE_X! ($w) }
  };
}

// window.h 991-1016: the scroll bar's height in pixels and lines.
macro_rules! WINDOW_CONFIG_SCROLL_BAR_HEIGHT {
  ($w:expr) => {
    if (*$w).scroll_bar_height >= 0 { (*$w).scroll_bar_height }
    else { FRAME_CONFIG_SCROLL_BAR_HEIGHT (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_CONFIG_SCROLL_BAR_LINES {
  ($w:expr) => {
    if (*$w).scroll_bar_height >= 0
      { ((*$w).scroll_bar_height + WINDOW_FRAME_LINE_HEIGHT! ($w) - 1)
          / WINDOW_FRAME_LINE_HEIGHT! ($w) }
    else { FRAME_CONFIG_SCROLL_BAR_LINES (WINDOW_XFRAME! ($w)) }
  };
}
macro_rules! WINDOW_SCROLL_BAR_LINES {
  ($w:expr) => {
    if WINDOW_HAS_HORIZONTAL_SCROLL_BAR ($w) { WINDOW_CONFIG_SCROLL_BAR_LINES! ($w) }
    else { 0 }
  };
}
macro_rules! WINDOW_SCROLL_BAR_AREA_HEIGHT {
  ($w:expr) => {
    if WINDOW_HAS_HORIZONTAL_SCROLL_BAR ($w) { WINDOW_CONFIG_SCROLL_BAR_HEIGHT! ($w) }
    else { 0 }
  };
}

// window.h 1020-1046: the mode/header/tab line heights and their `LINES` predicates.
// `window_wants_*` is declared by window.h 1213-1215 (the module below).
macro_rules! WINDOW_MODE_LINE_HEIGHT {
  ($w:expr) => { if window_wants_mode_line ($w) { CURRENT_MODE_LINE_HEIGHT! ($w) } else { 0 } };
}
macro_rules! WINDOW_MODE_LINE_LINES {
  ($w:expr) => { window_wants_mode_line ($w) };
}
macro_rules! WINDOW_HEADER_LINE_HEIGHT {
  ($w:expr) => {
    if window_wants_header_line ($w) { CURRENT_HEADER_LINE_HEIGHT! ($w) } else { 0 }
  };
}
macro_rules! WINDOW_HEADER_LINE_LINES {
  ($w:expr) => { window_wants_header_line ($w) };
}
macro_rules! WINDOW_TAB_LINE_HEIGHT {
  ($w:expr) => { if window_wants_tab_line ($w) { CURRENT_TAB_LINE_HEIGHT! ($w) } else { 0 } };
}
macro_rules! WINDOW_TAB_LINE_LINES {
  ($w:expr) => { window_wants_tab_line ($w) };
}

// window.h 1050-1072: the box heights, and where the horizontal scroll bar's area starts.
macro_rules! WINDOW_BOX_HEIGHT_NO_MODE_LINE {
  ($w:expr) => {
    WINDOW_PIXEL_HEIGHT ($w)
    - WINDOW_BOTTOM_DIVIDER_WIDTH! ($w)
    - WINDOW_SCROLL_BAR_AREA_HEIGHT! ($w)
    - WINDOW_MODE_LINE_HEIGHT! ($w)
  };
}
macro_rules! WINDOW_BOX_TEXT_HEIGHT {
  ($w:expr) => {
    WINDOW_PIXEL_HEIGHT ($w)
    - WINDOW_BOTTOM_DIVIDER_WIDTH! ($w)
    - WINDOW_SCROLL_BAR_AREA_HEIGHT! ($w)
    - WINDOW_MODE_LINE_HEIGHT! ($w)
    - WINDOW_HEADER_LINE_HEIGHT! ($w)
    - WINDOW_TAB_LINE_HEIGHT! ($w)
  };
}
macro_rules! WINDOW_SCROLL_BAR_AREA_Y {
  ($w:expr) => {
    WINDOW_TOP_EDGE_Y! ($w)
    + (if WINDOW_HAS_HORIZONTAL_SCROLL_BAR ($w) { WINDOW_BOX_HEIGHT_NO_MODE_LINE! ($w) }
       else { 0 })
  };
}

// window.h 1074-1092: the window/frame pixel conversions.
macro_rules! WINDOW_TO_FRAME_PIXEL_X {
  ($w:expr, $x:expr) => { ($x) + WINDOW_BOX_LEFT_EDGE_X! ($w) };
}
macro_rules! WINDOW_TO_FRAME_PIXEL_Y {
  ($w:expr, $y:expr) => { ($y) + WINDOW_TOP_EDGE_Y! ($w) };
}
macro_rules! FRAME_TO_WINDOW_PIXEL_X {
  ($w:expr, $x:expr) => { ($x) - WINDOW_BOX_LEFT_EDGE_X! ($w) };
}
macro_rules! FRAME_TO_WINDOW_PIXEL_Y {
  ($w:expr, $y:expr) => { ($y) - WINDOW_TOP_EDGE_Y! ($w) };
}
macro_rules! WINDOW_TEXT_TO_FRAME_PIXEL_X {
  ($w:expr, $x:expr) => { window_box_left ($w, TEXT_AREA) + ($x) };
}

// window.h 1213-1215: the three line predicates.  C declares them here and window.c defines
// them; declared in a module + glob import because window.rs *defines* them, and a crate-root
// declaration would collide (PORTING.md 0.1-10, the window_redisplay_decls shape below).
pub mod window_wants_decls {
  use super::*;

  unsafe extern "C" {
    /// C: bool window_wants_mode_line (struct window *)
    pub fn window_wants_mode_line (w: *mut window) -> bool;
    /// C: bool window_wants_header_line (struct window *)
    pub fn window_wants_header_line (w: *mut window) -> bool;
    /// C: bool window_wants_tab_line (struct window *)
    pub fn window_wants_tab_line (w: *mut window) -> bool;
  }
}
use window_wants_decls::*;

/// window.h 1225: `enum margin_unit { MARGIN_IN_LINES, MARGIN_IN_PIXELS };` — C's implicit
/// values, the unit `window_scroll_margin` measures in.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum margin_unit
  {
    MARGIN_IN_LINES = 0,
    MARGIN_IN_PIXELS,
  }
// window.h 1167-1169's redisplay entry points (`wset_redisplay`/`fset_redisplay`/
// `bset_redisplay`); xdisp.c defines them.  Declared in a module + glob import because a unit
// that defines one would otherwise collide with the crate-root declaration (PORTING.md 0.1-10).
pub mod window_redisplay_decls {
  use super::*;

  unsafe extern "C" {
    /// C: void wset_redisplay (struct window *w)
    pub fn wset_redisplay (w: *mut window);
    /// C: void fset_redisplay (struct frame *f)
    pub fn fset_redisplay (f: *mut c_void);
    /// C: void bset_redisplay (struct buffer *b)
    pub fn bset_redisplay (b: *mut c_void);
  }
}
use window_redisplay_decls::*;

/// window.h 1217-1222: `enum window_body_unit`, the unit `window_body_width` and
/// `window_body_height` measure in.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum window_body_unit
  {
    WINDOW_BODY_IN_CANONICAL_CHARS = 0,
    WINDOW_BODY_IN_PIXELS,
    WINDOW_BODY_IN_REMAPPED_CHARS,
  }
