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

// C: window.h 102-474 and 567-1244's 95 `WINDOW_*` macros — (not mirrored yet; they land next,
// in this same file).
