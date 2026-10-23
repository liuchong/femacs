// femacs: port of src/haikuterm.h @ fd7afced403.

// Header mirror (PORTING.md 0.1): declarations only.  This is the Haiku terminal backend's
// header - `haikuterm.c`/`haikufns.c`/`haikufont.c`/`haikumenu.c`/`haikuselect.c` implement it.
// The frozen target does not build it, but every `#if` in the file is decidable
// (`tools/check-cond-eval.py src/haikuterm.h` -> `5 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`), and all five are false arms: `USE_BE_CAIRO` three times (C 24-26, C 296-301,
// C 362-366) and `HAVE_NATIVE_IMAGE_API` once (C 352-357).  Those regions - `cairo.h`,
// `FRAME_CR_CONTEXT`, `haiku_can_use_native_image_api`/`haiku_load_image`/`syms_of_haikuimage`
// and the two cairo prototypes - are recorded in docs/ports/include-haikuterm.md instead of
// transcribed (PORTING.md P-08); everything else is live text and is carried.
//
// This mirror's compile evidence is the Haiku platform probe
// (`spikes/haiku_inc_probe.rs`), not `xtask layout-check`: that gate compiles every
// `src/include` mirror into one crate, which cannot hold the platform-exclusive mirrors
// (`haikugui.h` and `androidgui.h` declare the same X-compat block) - see
// docs/ports/include-haikugui.md for the ruling and the 35 measured `E0428`s.
//
// Records this header names that have no owner mirror keep the recorded stand-ins (PORTING.md
// P-55/P-72 rule 3): `struct font_metrics` (`struct haikufont_info.metrics`) and
// `struct font_driver` (the `haikufont_driver` object, C 226) are `font.h`'s and neither has
// landed in `include/font.rs`, so those two stay `*mut *mut c_void`/`*mut c_void` with the C text
// in a `/// C:` line.  `struct font` is **not** in that class: it is `include/font.rs`'s record
// (font.h 275-418, landed with this header's dependency) and this mirror includes that file, so
// `struct haikufont_info`'s first member - a `struct font` **by value** - and
// `struct haiku_output`'s `struct font *font` are spelled with the record, as C does.

// haikuterm.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _HAIKU_TERM_H_: bool = true;

include!("haikugui.rs");       // C 29: `#include "haikugui.h"` (`haiku`, `Window`, `struct haiku_rect`)
include!("frame.rs");          // C 30: `#include "frame.h"`
include!("character.rs");      // C 31: `#include "character.h"` (declares no name the live text uses)
include!("dispextern.rs");     // C 32: `#include "dispextern.h"` (`Emacs_Color`, `Mouse_HLInfo`)
include!("font.rs");           // C 33: `#include "font.h"` (the WIP slice; see the note above)
include!("systime.rs");        // C 34: `#include "systime.h"` (`Time`)
include!("termhooks.rs");      // C reaches `struct terminal`/`struct input_event` through frame.h
include!("haiku_support.rs");  // C 32 of haikuterm.c: `struct haiku_menu_bar_click_event`, `struct haiku_font_change_event`

// haikuterm.h 35: C's implicit type is `int`.
pub const HAVE_CHAR_CACHE_MAX: c_int = 65535;

// haikuterm.h 38's `struct haiku_font_change_event;` is a forward declaration of the tag whose
// definition lives in haiku_support.h (reached above), so it has no declaration of its own here.

/* The Haiku font driver's font-object record.  Its first member is a `struct font` **by
   value**, which is why `include/font.rs` had to carry that record first (font.h 275-418);
   `be_font` is `haikugui.rs`'s `haiku` and `glyphs` is C's own `unsigned short **`.  */
// haikuterm.h 42-50.
#[repr(C)]
pub struct haikufont_info
{
  pub font: font,
  pub be_font: haiku,

  /// C: `struct font_metrics **metrics;` (haikuterm.h 46) - `struct font_metrics` is `font.h`'s
  /// record and no mirror declares it yet, so only the pointee is untyped and the pointer depth
  /// C has is kept (PORTING.md P-55/P-72 rule 3, the `w32inevt.rs` `struct frame **` spelling).
  pub metrics: *mut *mut c_void,
  pub metrics_nrows: c_short,

  pub glyphs: *mut *mut c_ushort,
}

/* This is a chain of structures for all the Haiku displays
   currently in use.  There is only ever one, but the rest of the C tree is
   written with systems on which there can be many in mind.  */
// haikuterm.h 51-64.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_bitmap_record
{
  pub img: haiku,
  pub file: *mut c_char,
  pub refcount: c_int,
  pub height: c_int,
  pub width: c_int,
  pub depth: c_int,

  pub stipple_foreground: uint32_t,
  pub stipple_background: uint32_t,
  pub stipple_bits: *mut c_void,
}

/* Structure describing a Haiku frame display.  */
// haikuterm.h 66-157.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_display_info
{
  /* Chain of all haiku_display_info structures.  */
  pub next: *mut haiku_display_info,
  pub terminal: *mut terminal,

  pub name_list_element: Lisp_Object,
  pub color_map: Lisp_Object,

  pub n_fonts: c_int,

  pub smallest_char_width: c_int,
  pub smallest_font_height: c_int,

  pub focused_frame: *mut frame,
  pub focus_event_frame: *mut frame,
  pub last_mouse_glyph_frame: *mut frame,

  pub bitmaps: *mut haiku_bitmap_record,
  pub bitmaps_size: ptrdiff_t,
  pub bitmaps_last: ptrdiff_t,

  pub grabbed: c_int,
  pub n_planes: c_int,
  pub color_p: c_int,

  pub rdb: Lisp_Object,
  pub default_name: Lisp_Object,

  pub vertical_scroll_bar_cursor: Emacs_Cursor,
  pub horizontal_scroll_bar_cursor: Emacs_Cursor,

  pub mouse_highlight: Mouse_HLInfo,

  pub highlight_frame: *mut frame,
  pub last_mouse_frame: *mut frame,
  pub last_mouse_motion_frame: *mut frame,

  pub last_mouse_motion_x: c_int,
  pub last_mouse_motion_y: c_int,

  pub last_mouse_glyph: haiku_rect,

  pub display: haiku,

  pub resx: f64,
  pub resy: f64,

  pub last_mouse_movement_time: Time,

  pub root_window: Window,

  pub text_cursor: Emacs_Cursor,
  pub nontext_cursor: Emacs_Cursor,
  pub modeline_cursor: Emacs_Cursor,
  pub hand_cursor: Emacs_Cursor,
  pub hourglass_cursor: Emacs_Cursor,
  pub horizontal_drag_cursor: Emacs_Cursor,
  pub vertical_drag_cursor: Emacs_Cursor,
  pub left_edge_cursor: Emacs_Cursor,
  pub top_left_corner_cursor: Emacs_Cursor,
  pub top_edge_cursor: Emacs_Cursor,
  pub top_right_corner_cursor: Emacs_Cursor,
  pub right_edge_cursor: Emacs_Cursor,
  pub bottom_right_corner_cursor: Emacs_Cursor,
  pub bottom_edge_cursor: Emacs_Cursor,
  pub bottom_left_corner_cursor: Emacs_Cursor,
  pub no_cursor: Emacs_Cursor,
}

// haikuterm.h 159-217.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_output
{
  pub display_info: *mut haiku_display_info,

  pub text_cursor: Emacs_Cursor,
  pub nontext_cursor: Emacs_Cursor,
  pub modeline_cursor: Emacs_Cursor,
  pub hand_cursor: Emacs_Cursor,
  pub hourglass_cursor: Emacs_Cursor,
  pub horizontal_drag_cursor: Emacs_Cursor,
  pub vertical_drag_cursor: Emacs_Cursor,
  pub left_edge_cursor: Emacs_Cursor,
  pub top_left_corner_cursor: Emacs_Cursor,
  pub top_edge_cursor: Emacs_Cursor,
  pub top_right_corner_cursor: Emacs_Cursor,
  pub right_edge_cursor: Emacs_Cursor,
  pub bottom_right_corner_cursor: Emacs_Cursor,
  pub bottom_edge_cursor: Emacs_Cursor,
  pub bottom_left_corner_cursor: Emacs_Cursor,
  pub no_cursor: Emacs_Cursor,
  pub current_cursor: Emacs_Cursor,

  pub cursor_color: Emacs_Color,

  pub parent_desc: Window,

  pub window: haiku,
  pub view: haiku,
  pub menubar: haiku,

  pub fontset: c_int,
  pub baseline_offset: c_int,

  /// C 190-205: the four `bool_bf` members `hourglass_p : 1`, `menu_bar_open_p : 1`,
  /// `dirty_p : 1` and `complete_p : 1`.  `bool_bf` is one byte on this target, so C packs the
  /// four bits into one storage byte, which the port folds into this field (PORTING.md 1.3,
  /// P-80; the same shape `itree.rs` uses for its three `bool_bf : 1` members).
  pub bits: u8,

  /// C: `struct font *font;` (haikuterm.h 181) - `struct font` is `include/font.rs`'s record.
  pub font: *mut font,

  /* The pending position we're waiting for.  */
  pub pending_top: c_int,
  pub pending_left: c_int,

  /* Whether or not adjust_frame_size and haiku_set_offset have yet
     been called by haiku_create_frame.  */
  pub configury_done: bool,

  /* The default cursor foreground color.  */
  pub cursor_fg: uint32_t,

  /* If non-NULL, the last menu bar click event received.  */
  pub saved_menu_event: *mut haiku_menu_bar_click_event,

  /* The type of any event that's being waited for.  */
  pub wait_for_event_type: c_int,

  /* The "dark" color of the current relief.  */
  pub black_relief_pixel: uint32_t,

  /* The "light" color of the current relief.  */
  pub white_relief_pixel: uint32_t,

  /* The background for which the relief colors above were computed.
     They are changed only when a different background is involved.
     -1 means no color has been computed.  */
  pub relief_background: c_long,

  /* The absolute position of this frame.  This differs from left_pos
     and top_pos in that the decorator and parent frames are not taken
     into account.  */
  pub frame_x: c_int,
  pub frame_y: c_int,

  /* The current fullscreen mode of this frame.  This should be `enum
     haiku_fullscreen_mode', but that isn't available here.  */
  pub fullscreen_mode: c_int,
}

/* The four packed bits above, named after the members C declares (PORTING.md 1.3/P-80).  */
pub const HAIKU_OUTPUT_HOURGLASS_P_BIT: u8 = 1 << 0;
pub const HAIKU_OUTPUT_MENU_BAR_OPEN_P_BIT: u8 = 1 << 1;
pub const HAIKU_OUTPUT_DIRTY_P_BIT: u8 = 1 << 2;
pub const HAIKU_OUTPUT_COMPLETE_P_BIT: u8 = 1 << 3;

/* The two accessors take the pointer `FRAME_OUTPUT_DATA` yields - the untyped P-72 rule-3
   member - and cast once inside, which is what lets `FRAME_DIRTY_P`/`FRAME_COMPLETE_P`/
   `FRAME_HAIKU_DRAWABLE` below pass it straight through (the first consumer, `haikuterm.rs`,
   measured `E0308 expected *mut haiku_output, found *mut c_void` at every
   `MAKE_FRAME_DIRTY` expansion while the parameter was `*const haiku_output`).  */
#[inline(always)]
pub fn haiku_output_bit_p (o: *mut c_void, bit: u8) -> bool
{
  return unsafe { (*(o as *mut haiku_output)).bits } & bit != 0;
}

#[inline(always)]
pub fn haiku_output_bit_set (o: *mut c_void, bit: u8, value: bool)
{
  let o = o as *mut haiku_output;
  unsafe {
    if value
      {
        (*o).bits |= bit;
      }
    else
      {
        (*o).bits &= ! bit;
      }
  }
}

/// C 190-205's four `bool_bf` members: the member name is the macro's first argument, so the C
/// token survives the `--check` body comparison, and the write form is the second arm (C
/// assigns to the field, which Rust cannot do through a macro: `MAKE_FRAME_DIRTY` below is the
/// one site that writes, and it uses the two-argument arm - recorded in the ledger's Decisions).
macro_rules! haiku_output_bits {
  (hourglass_p, $o:expr) => { haiku_output_bit_p ($o, HAIKU_OUTPUT_HOURGLASS_P_BIT) };
  (hourglass_p, $o:expr, $v:expr)
    => { haiku_output_bit_set ($o, HAIKU_OUTPUT_HOURGLASS_P_BIT, ($v) != 0) };
  (menu_bar_open_p, $o:expr) => { haiku_output_bit_p ($o, HAIKU_OUTPUT_MENU_BAR_OPEN_P_BIT) };
  (menu_bar_open_p, $o:expr, $v:expr)
    => { haiku_output_bit_set ($o, HAIKU_OUTPUT_MENU_BAR_OPEN_P_BIT, ($v) != 0) };
  (dirty_p, $o:expr) => { haiku_output_bit_p ($o, HAIKU_OUTPUT_DIRTY_P_BIT) };
  (dirty_p, $o:expr, $v:expr)
    => { haiku_output_bit_set ($o, HAIKU_OUTPUT_DIRTY_P_BIT, ($v) != 0) };
  (complete_p, $o:expr) => { haiku_output_bit_p ($o, HAIKU_OUTPUT_COMPLETE_P_BIT) };
  (complete_p, $o:expr, $v:expr)
    => { haiku_output_bit_set ($o, HAIKU_OUTPUT_COMPLETE_P_BIT, ($v) != 0) };
}

// haikuterm.h 220-223.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct x_output
{
  /* Unused, makes term.c happy.  */
}

/* The nine file-scope objects.  Their definitions belong to the units (`haikuterm.c`'s
   `x_display_list`, `haikufns.c`'s `haiku_frame_parm_handlers`/`tip_frame`/`tip_dx`/`tip_dy`,
   `haikufont.c`'s `haikufont_driver`, `haiku_support.cc`'s `popup_activated_p`), so the
   declarations sit in a glob-imported module whose import those definitions shadow
   (PORTING.md 0.1-10/P-21, the `androidterm_objs` shape).  */
pub mod haikuterm_objs {
  use super::*;

  unsafe extern "C" {
    pub static mut popup_activated_p: c_int;                       // haikuterm.h 40

    pub static mut x_display_list: *mut haiku_display_info;        // 225

    // haikuterm.h 226: `extern struct font_driver const haikufont_driver;` - `struct font_driver`
    // is font.h's record and has not landed in `include/font.rs` (`struct font` has; the note at
    // the top of this file says which records are which), so the object keeps the pointer-sized
    // stand-in (the `androidterm_objs` shape).
    pub static mut haikufont_driver: *mut c_void;

    pub static mut tip_frame: Lisp_Object;                         // 228
    pub static mut tip_dx: Lisp_Object;                            // 229
    pub static mut tip_dy: Lisp_Object;                            // 230

    pub static mut haiku_dnd_frame: *mut frame;                    // 232
    pub static mut haiku_dnd_follow_tooltip: bool;                 // 233

    // haikuterm.h 235: `extern frame_parm_handler haiku_frame_parm_handlers[];` -
    // `frame_parm_handler` is dispextern.h 3063's typedef, which `include/dispextern.rs` does not
    // carry yet, so the unbounded array keeps the pointer-sized element stand-in and becomes
    // `[frame_parm_handler; 0]` in the commit that lands that alias (the `androidterm_objs`
    // precedent).
    pub static mut haiku_frame_parm_handlers: [*mut c_void; 0];
  }
}
use haikuterm_objs::*;

// haikuterm.h 237-272.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct scroll_bar
{
  /* These fields are shared by all vectors.  */
  pub header: vectorlike_header,

  /* The window we're a scroll bar for.  */
  pub window: Lisp_Object,

  /* The next and previous in the chain of scroll bars in this frame.  */
  pub next: Lisp_Object,
  pub prev: Lisp_Object,

  /* Fields after 'prev' are not traced by the GC.  */

  /* The position and size of the scroll bar in pixels, relative to the
     frame.  */
  pub top: c_int,
  pub left: c_int,
  pub width: c_int,
  pub height: c_int,

  /* The actual scrollbar.  */
  pub scroll_bar: *mut c_void,

  /* Non-nil if the scroll bar handle is currently being dragged by
     the user.  */
  pub dragging: c_int,

  /* The update position if we are waiting for a scrollbar update, or
     -1.  */
  pub update: c_int,

  /* The last known position of this scrollbar.  */
  pub position: c_int,

  /* The total number of units inside this scrollbar.  */
  pub total: c_int,

  /* True if the scroll bar is horizontal.  */
  pub horizontal: bool,

  /* The amount of units taken up by the thumb, which represents the
     portion of the buffer currently on screen.  */
  pub page_size: c_int,
}

/* Turning a lisp vector value into a pointer to a struct scroll_bar.  */
// haikuterm.h 279.
macro_rules! XSCROLL_BAR {
  ($vec:expr) => { (XVECTOR ($vec) as *mut scroll_bar) };
}

/* Return the frame's Haiku output data.  */
// haikuterm.h 283-294.  `frame.rs`'s `output_data.haiku` member is still the P-72 rule-3
// stand-in (`*mut c_void`, because `haiku_output` was not mirrored when frame.h landed), so the
// field-reading macros below cast it once; the cast is what P-72 rule 3 says the use site writes,
// and it stays valid when `frame.rs`'s member becomes `*mut haiku_output`.
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { ((*$f).output_data.haiku) };
}
macro_rules! FRAME_DIRTY_P {
  ($f:expr) => { haiku_output_bits! (dirty_p, FRAME_OUTPUT_DATA! ($f)) };
  ($f:expr, $v:expr) => { haiku_output_bits! (dirty_p, FRAME_OUTPUT_DATA! ($f), $v) };
}
macro_rules! FRAME_COMPLETE_P {
  ($f:expr) => { haiku_output_bits! (complete_p, FRAME_OUTPUT_DATA! ($f)) };
  ($f:expr, $v:expr) => { haiku_output_bits! (complete_p, FRAME_OUTPUT_DATA! ($f), $v) };
}
macro_rules! MAKE_FRAME_DIRTY {
  ($f:expr) => { FRAME_DIRTY_P! ($f, 1) };
}
macro_rules! FRAME_HAIKU_WINDOW {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).window) };
}
macro_rules! FRAME_HAIKU_VIEW {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).view) };
}
macro_rules! FRAME_HAIKU_DRAWABLE {
  ($f:expr) => {
    {
      MAKE_FRAME_DIRTY! ($f);
      FRAME_HAIKU_VIEW! ($f)
    }
  };
}
macro_rules! FRAME_HAIKU_MENU_BAR {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).menubar) };
}
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).display_info) };
}
macro_rules! FRAME_FONT {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).font) };
}
macro_rules! FRAME_FONTSET {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).fontset) };
}
macro_rules! FRAME_NATIVE_WINDOW {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).window) };
}
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).baseline_offset) };
}
macro_rules! FRAME_CURSOR_COLOR {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).cursor_color) };
}

/* The 43 prototypes C spells `extern`.  They sit in a glob-imported module because the units
   above define every one of them and a crate-root declaration would be `E0428`
   (PORTING.md 0.1-10/P-17/P-21).  `extract-abi.py` does not inventory prototypes, so this block
   is the part of the mirror no gate compares; it was checked name-for-name against the reference
   (43 of 43).  */
pub mod haikuterm_decls {
  use super::*;

  unsafe extern "C" {
    pub fn syms_of_haikuterm ();
    pub fn syms_of_haikufns ();
    pub fn syms_of_haikumenu ();
    pub fn syms_of_haikufont ();
    pub fn syms_of_haikuselect ();
    pub fn init_haiku_select ();
    pub fn haiku_iconify_frame (arg0: *mut frame);
    pub fn haiku_visualize_frame (arg0: *mut frame);
    pub fn haiku_unvisualize_frame (arg0: *mut frame);
    pub fn haiku_set_offset (arg0: *mut frame, arg1: c_int, arg2: c_int, arg3: c_int);
    pub fn haiku_set_frame_visible_invisible (arg0: *mut frame, arg1: bool);
    pub fn haiku_free_frame_resources (arg0: *mut frame);
    pub fn haiku_scroll_bar_remove (arg0: *mut scroll_bar);
    pub fn haiku_clear_under_internal_border (arg0: *mut frame);
    pub fn haiku_set_name (arg0: *mut frame, arg1: Lisp_Object, arg2: bool);
    pub fn haiku_message_to_lisp (arg0: *mut c_void) -> Lisp_Object;
    pub fn haiku_term_init () -> *mut haiku_display_info;   // C haikuterm.h 321
    pub fn mark_haiku_display ();
    pub fn haiku_get_color (arg0: *const c_char, arg1: *mut Emacs_Color) -> c_int;
    pub fn haiku_set_background_color (arg0: *mut frame, arg1: Lisp_Object, arg2: Lisp_Object);
    pub fn haiku_set_cursor_color (arg0: *mut frame, arg1: Lisp_Object, arg2: Lisp_Object);
    pub fn haiku_set_cursor_type (arg0: *mut frame, arg1: Lisp_Object, arg2: Lisp_Object);
    pub fn haiku_set_internal_border_width (arg0: *mut frame, arg1: Lisp_Object, arg2: Lisp_Object);
    pub fn haiku_change_tab_bar_height (arg0: *mut frame, arg1: c_int);
    pub fn haiku_change_tool_bar_height (arg0: *mut frame, arg1: c_int);
    pub fn haiku_free_custom_cursors (arg0: *mut frame);
    pub fn haiku_query_color (arg0: uint32_t, arg1: *mut Emacs_Color);
    pub fn haiku_get_pixel (arg0: haiku, arg1: c_int, arg2: c_int) -> c_ulong;
    pub fn haiku_put_pixel (arg0: haiku, arg1: c_int, arg2: c_int, arg3: c_ulong);
    pub fn haiku_menu_show (arg0: *mut frame, arg1: c_int, arg2: c_int, arg3: c_int, arg4: Lisp_Object, arg5: *const *const c_char) -> Lisp_Object;
    pub fn haiku_popup_dialog (arg0: *mut frame, arg1: Lisp_Object, arg2: Lisp_Object) -> Lisp_Object;
    pub fn haiku_activate_menubar (arg0: *mut frame);
    pub fn haiku_wait_for_event (arg0: *mut frame, arg1: c_int);
    pub fn haiku_note_drag_motion ();
    pub fn haiku_note_drag_wheel (arg0: *mut input_event);
    pub fn initialize_frame_menubar (arg0: *mut frame);
    pub fn run_menu_bar_help_event (arg0: *mut frame, arg1: c_int);
    pub fn put_xrm_resource (arg0: Lisp_Object, arg1: Lisp_Object);
    /// C: `void haiku_draw_background_rect (struct glyph_string *, struct face *, int, int,
    /// int, int);` - neither `struct glyph_string` nor `struct face` has a mirror yet
    /// (`include/dispextern.rs` carries the glyph datatype but not those two records), so the
    /// pointers keep the untyped form (PORTING.md P-55/P-72 rule 3).
    pub fn haiku_draw_background_rect (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
    /// C: `void haiku_merge_cursor_foreground (struct glyph_string *, unsigned long *,
    /// unsigned long *);` - same degradation as `haiku_draw_background_rect`.
    pub fn haiku_merge_cursor_foreground (arg0: *mut c_void, arg1: *mut c_ulong, arg2: *mut c_ulong);
    pub fn haiku_handle_selection_clear (arg0: *mut input_event);
    pub fn haiku_start_watching_selections ();
    pub fn haiku_handle_font_change_event (arg0: *mut haiku_font_change_event, arg1: *mut input_event);
  }
}
use haikuterm_decls::*;
