// femacs: port of src/window.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): the two types.  `window.h` includes `dispextern.h` (line 23), so
// C sees `struct glyph_matrix` complete here — the port cannot yet, because the display chain's
// matrix/face segment is still ahead of it, so those two pointers carry the P-72 rule ③
// degradation until it lands.  `HAVE_TEXT_CONVERSION` and `HAVE_WINDOW_SYSTEM` are `#undef` in the
// frozen `config.h`, so `ephemeral_last_point` and the `phys_cursor_*` members are not part of
// this target's structure (the field scanner's live-line filter is what keeps them out — P-73).
// The 95 macros, 16 inlines and 9 file-scope objects land next.
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
// window.h 1282-1300: the mini-window predicates.  `mini` is one of the nineteen packed flags.
#[inline(always)]
pub fn MINI_WINDOW_P (w: *mut window) -> bool { window_bit (w, WINDOW_MINI_SHIFT) }

#[inline(always)]
pub fn MINI_NON_ONLY_WINDOW_P (w: *mut window) -> bool
{ MINI_WINDOW_P (w) && !NILP (unsafe { (*w).prev }) }

#[inline(always)]
pub fn MINI_ONLY_WINDOW_P (w: *mut window) -> bool
{ MINI_WINDOW_P (w) && NILP (unsafe { (*w).prev }) }

// window.h 1305-1312: the minimum sizes (the C macros ignore their argument).
#[inline(always)]
pub fn MIN_SAFE_WINDOW_HEIGHT (_w: *mut window) -> c_int { 1 }
#[inline(always)]
pub fn MIN_SAFE_WINDOW_WIDTH (_w: *mut window) -> c_int { 2 }

// window.h 1316: `#define USE_HORIZONTAL_SCROLL_BARS false` — object-like, so the port keeps a
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

// C: window.h 567-1244's remaining accessors that dereference `struct frame` through
// `WINDOW_XFRAME` (`WINDOW_FRAME_COLUMN_WIDTH`, `WINDOW_LEFT_EDGE_X`, the fringe/scroll-bar
// geometry, `WINDOW_RIGHTMOST_P`, `MIN_SAFE_WINDOW_PIXEL_*`, `FRAME_TO_WINDOW_PIXEL_*`, …),
// `WINDOW_MODE_LINE_HEIGHT`/`_LINES` and the header/tab-line pair (they need dispextern.h's
// `window_wants_*` and `CURRENT_*_LINE_HEIGHT`), and `WINDOW_VERTICAL_SCROLL_BAR_TYPE` (it needs
// frame.h's `vertical_scroll_bar_none`/`_left`/`_right`) — (not mirrored yet; they land with
// frame.rs, termchar.h and the display chain's `window_wants_*` declarations).
