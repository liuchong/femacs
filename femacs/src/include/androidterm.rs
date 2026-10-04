// femacs: port of src/androidterm.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): the five types, the anonymous focus-state enumeration, the
// eighteen function-like macros, the include guard and the `unsafe extern "C"` declarations C
// spells `extern`.  Nothing here emits a symbol.
//
// The frozen target (darwin-arm64, docs/baseline/config/darwin-arm64.macros.tsv) does not build
// this header - `WINDOW_SYSTEM_OBJ` is empty for it (docs/baseline/frozen-build-sources.txt) -
// but every `#if` in the file is decidable, so the mirror is the frozen configuration's text:
// `tools/check-cond-eval.py src/androidterm.h` reports `3 conditions, 0 unknown, 0
// disagreements [VERIFIED]`, and all three are taken: the guard's `#ifndef _ANDROID_TERM_H_`
// (C 20) and two `#ifndef ANDROID_STUBIFY` regions (C 448-452 and C 472-485), because
// `ANDROID_STUBIFY` is `undef` in the frozen table.  Nothing in this header is dead text, so
// P-08's accounting has no region to record (contrast include/gtkutil.rs and
// include/pgtkterm.rs, whose whole bodies sit under `USE_GTK`/`HAVE_PGTK`).
//
// Types this header names but does not declare are reached through its own `include!`s
// (PORTING.md P-69/P-72): `androidgui.rs` for `android_pixmap`/`Emacs_Window`/`android_cursor`/
// `android_gc` (androidgui.h 33-42, 148), `frame.rs` for `frame`, `dispextern.rs` for
// `Emacs_Rectangle`/`Mouse_HLInfo`/`Emacs_Cursor`/`Emacs_Color`, `termhooks.rs` for `terminal`
// and `input_event`, `font.rs` for font.h's declarations, `systime.rs` for `Time` (termhooks.h
// 28 pulls systime.h for it).  Three names have no owner mirror yet and keep the recorded
// stand-ins: `struct font` (font.rs is a WIP slice without the record), `struct font_driver`
// (same) and `struct font_entity` (same), plus `struct window *` (window.h is not in this
// header's include list) and `struct android_notification_event *` (no mirror anywhere in the
// tree) - each is spelled `*mut c_void` with the C text in a `/// C:` line, the P-72 rule ③
// shape dispextern.rs uses for `struct frame *`.
//
// `Lisp_Object`, `union vectorlike_header`, `XVECTOR` and the accessors come from `lisp.rs`,
// which no `src/` mirror includes (PORTING.md 0.1-9); every unit that carries this mirror has
// lisp.rs in its own include list, exactly as every C unit gets them from lisp.h.
//
// One known interaction with a future Android target: androidgui.h 42 and dispextern.h 175 both
// `typedef` `Emacs_Cursor` (the latter under `#ifndef HAVE_WINDOW_SYSTEM`, which is the branch
// `dispextern.rs` carries because the frozen configuration has no window system).  A TU that
// includes both mirrors would see two `Emacs_Cursor` items.  C's own guards keep the two apart
// on an Android build; the port will need the same `cfg` on dispextern.rs's alias when that
// target is frozen.  Recorded in docs/ports/include-androidterm.md.

include!("androidgui.rs");                    // androidterm.h 23: #include "androidgui.h"
include!("frame.rs");                         // 24: #include "frame.h"
include!("character.rs");                     // 25: #include "character.h"
include!("dispextern.rs");                    // 26: #include "dispextern.h"
include!("font.rs");                          // 27: #include "font.h"
include!("termhooks.rs");                     // 28: #include "termhooks.h"
include!("systime.rs");                       // `Time` (termhooks.h 28's #include "systime.h")

// androidterm.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).  The name is
// C's, underscores and all (PORTING.md 0.2), because the extractor's macro set keys it.
pub const _ANDROID_TERM_H_: bool = true;

/* A single bitmap record, as `android_bitmap_record` in the C.  */
// androidterm.h 30-46.  Measured with the frozen target's clang (evidence in the ledger):
// sizeof 48, alignof 8; pixmap 0, mask 8, file 16, refcount 24, height 28, width 32, depth 36,
// have_mask 40.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_bitmap_record
{
  /* The image backing the bitmap and its mask.  */
  pub pixmap: android_pixmap,
  pub mask: android_pixmap,

  /* The file from which it comes.  */
  pub file: *mut c_char,

  /* The number of references to it.  */
  pub refcount: c_int,

  /* The height and width and the depth.  */
  pub height: c_int,
  pub width: c_int,
  pub depth: c_int,

  /* Whether or not there is a mask.  */
  pub have_mask: bool,
}
const _: () = assert! (std::mem::size_of::<android_bitmap_record> () == 48);

/* Per-display information.  */
// androidterm.h 48-155.  Measured: sizeof 312, alignof 8.  `double` is `f64` (PORTING.md 1.1);
// `ptrdiff_t` is `isize`; `Mouse_HLInfo` is dispextern.h's 72-byte record.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_display_info
{
  /* Chain of all struct android_display_info structures.  */
  pub next: *mut android_display_info,

  /* The terminal.  */
  pub terminal: *mut terminal,

  /* The root window.  This field is unused.  */
  pub root_window: Emacs_Window,

  /* List possibly used only for the font cache but probably used for
     something else too.  */
  pub name_list_element: Lisp_Object,

  /* List of predefined X colors.  */
  pub color_map: Lisp_Object,

  /* DPI of the display.  */
  pub resx: f64,
  pub resy: f64,

  /* DPI used to convert font point sizes into pixel dimensions.
     This is resy adjusted by a fixed scaling factor specified by
     the user.  */
  pub font_resolution: f64,

  /* Scratch GC for drawing a cursor in a non-default face. */
  pub scratch_cursor_gc: *mut android_gc,

  /* Mouse highlight information.  */
  pub mouse_highlight: Mouse_HLInfo,

  /* Number of planes on this screen, and the same for the purposes of
     image processing.  */
  pub n_planes: c_int,
  pub n_image_planes: c_int,

  /* Mask of things causing the mouse to be grabbed.  */
  pub grabbed: c_int,

  /* Minimum width over all characters in all fonts in font_table.  */
  pub smallest_char_width: c_int,

  /* Minimum font height over all fonts in font_table.  */
  pub smallest_font_height: c_int,

  /* Default name for all frames on this display.  */
  pub x_id_name: *mut c_char,

  /* The number of fonts opened for this display.  */
  pub n_fonts: c_int,

  /* Pointer to bitmap records.  */
  pub bitmaps: *mut android_bitmap_record,

  /* Allocated size of bitmaps field.  */
  pub bitmaps_size: ptrdiff_t,

  /* Last used bitmap index.  */
  pub bitmaps_last: ptrdiff_t,

  /* The frame currently with the input focus.  */
  pub focus_frame: *mut frame,

  /* The last frame mentioned in a focus event.  */
  pub x_focus_event_frame: *mut frame,

  /* The frame which currently has the visual highlight, and should
     get keyboard input.  It points to the focus frame's selected
     window's frame, but can differ.  */
  pub highlight_frame: *mut frame,

  /* The frame waiting to be auto-raised in android_read_socket.  */
  pub pending_autoraise_frame: *mut frame,

  /* The frame where the mouse was the last time a button event
     happened.  */
  pub last_mouse_frame: *mut frame,

  /* The frame where the mouse was the last time the mouse glyph
     changed.  */
  pub last_mouse_glyph_frame: *mut frame,

  /* The frame where the mouse was the last time mouse motion
     happened.  */
  pub last_mouse_motion_frame: *mut frame,

  /* Position where the mouse was last time we reported a motion.
     This is a position on last_mouse_motion_frame.  It is used in to
     report the mouse position as well: see
     android_mouse_position.  */
  pub last_mouse_motion_x: c_int,
  pub last_mouse_motion_y: c_int,

  /* Where the mouse was the last time the mouse moved.  */
  pub last_mouse_glyph: Emacs_Rectangle,

  /* The time of the last mouse movement.  */
  pub last_mouse_movement_time: Time,

  /* ID of the last menu event received.  -1 means Emacs is waiting
     for a context menu event.  */
  pub menu_event_id: c_int,

  /* The invisible cursor used for pointer blanking.  */
  pub invisible_cursor: android_cursor,
}
const _: () = assert! (std::mem::size_of::<android_display_info> () == 312);

/* Structure representing a single tool (finger or stylus) pressed
   onto a frame.  */
// androidterm.h 157-171.  Measured: sizeof 24, alignof 8; next 0, tool_id 8, x 12, y 16,
// tool_bar_p 20.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_touch_point
{
  /* The next tool on this list.  */
  pub next: *mut android_touch_point,

  /* The tool ID and the last known X and Y positions.  */
  pub tool_id: c_int,
  pub x: c_int,
  pub y: c_int,

  /* Whether or not the tool is pressed on the tool bar.  */
  pub tool_bar_p: bool,
}
const _: () = assert! (std::mem::size_of::<android_touch_point> () == 24);

// androidterm.h 197-199: C's nested `struct relief { struct android_gc *gc; unsigned long
// pixel; }`, the type of the two `black_relief`/`white_relief` members.  A nested struct
// needs a name in Rust (PORTING.md 1.3), and C's own tag is `relief`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct relief
{
  pub gc: *mut android_gc,
  pub pixel: c_ulong,
}

/* Per-frame output data.  */
// androidterm.h 173-265.  Measured: sizeof 320, alignof 8; hourglass 224, display_info 232,
// the bitfield byte 240, black_relief 248, white_relief 264, relief_background 280,
// focus_state 288, touch_points 296, extracted_text_flags 304, extracted_text_token 308,
// extracted_text_hint 312.  The five `bool_bf : 1` members (C 231-249) share one 1-byte
// allocation unit (`bool_bf` is `bool`, conf_post.h 61), folded into `bits` after
// `display_info` (PORTING.md 1.3) - the position is what the offsets above assert.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_output
{
  /* Graphics contexts for the default font.  */
  pub normal_gc: *mut android_gc,
  pub reverse_gc: *mut android_gc,
  pub cursor_gc: *mut android_gc,

  /* The window used for this frame.  */
  pub window: Emacs_Window,

  /* Unused field.  */
  pub parent_desc: Emacs_Window,

  /* Default ASCII font of this frame.  */
  /// C: `struct font *font;` (androidterm.h 185) - font.rs does not carry `struct font` yet.
  pub font: *mut c_void,

  /* The baseline offset of the default ASCII font.  */
  pub baseline_offset: c_int,

  /* If a fontset is specified for this frame instead of font, this
     value contains an ID of the fontset, else -1.  */
  pub fontset: c_int,

  /* Various colors.  */
  pub cursor_pixel: c_ulong,
  pub mouse_pixel: c_ulong,
  pub cursor_foreground_pixel: c_ulong,

  /* Foreground color for scroll bars.  A value of -1 means use the
     default (black for non-toolkit scroll bars).  */
  pub scroll_bar_foreground_pixel: c_ulong,

  /* Background color for scroll bars.  A value of -1 means use the
     default (background color of the frame for non-toolkit scroll
     bars).  */
  pub scroll_bar_background_pixel: c_ulong,

  /* Cursors associated with this frame.  */
  pub text_cursor: Emacs_Cursor,
  pub nontext_cursor: Emacs_Cursor,
  pub modeline_cursor: Emacs_Cursor,
  pub hand_cursor: Emacs_Cursor,
  pub hourglass_cursor: Emacs_Cursor,
  pub horizontal_drag_cursor: Emacs_Cursor,
  pub vertical_drag_cursor: Emacs_Cursor,
  pub current_cursor: Emacs_Cursor,
  pub left_edge_cursor: Emacs_Cursor,
  pub top_left_corner_cursor: Emacs_Cursor,
  pub top_edge_cursor: Emacs_Cursor,
  pub top_right_corner_cursor: Emacs_Cursor,
  pub right_edge_cursor: Emacs_Cursor,
  pub bottom_right_corner_cursor: Emacs_Cursor,
  pub bottom_edge_cursor: Emacs_Cursor,
  pub bottom_left_corner_cursor: Emacs_Cursor,

  /* Whether or not the hourglass cursor is being displayed.  */
  pub hourglass: bool,

  /* This is the Emacs structure for the display this frame is on.  */
  pub display_info: *mut android_display_info,

  // androidterm.h 231-249: `bool_bf has_been_visible : 1;`, `alpha_identical_p : 1`,
  // `complete : 1`, `need_buffer_flip : 1` and `need_cursor_updates : 1`, five bits of one
  // unsigned-char-sized unit (the `bits: u8` shape cm.rs uses, PORTING.md 1.3).
  pub bits: u8,

  /* Relief GCs, colors etc.  */
  pub black_relief: relief,
  pub white_relief: relief,

  /* The background for which the above relief GCs were set up.
     They are changed only when a different background is involved.  */
  pub relief_background: c_ulong,

  /* Focus state.  Only present for consistency with X; it is actually
     a boolean.  */
  pub focus_state: c_int,

  /* List of all tools (either styluses or fingers) pressed onto the
     frame.  */
  pub touch_points: *mut android_touch_point,

  /* Flags associated with the last request to obtain ``extracted
     text''.  */
  pub extracted_text_flags: c_int,

  /* Token associated with that request.  */
  pub extracted_text_token: c_int,

  /* The number of characters of extracted text wanted by the IM.  */
  pub extracted_text_hint: c_int,
}
const _: () = assert! (std::mem::size_of::<android_output> () == 320);
const _: () = assert! (std::mem::offset_of! (android_output, hourglass) == 224);
const _: () = assert! (std::mem::offset_of! (android_output, display_info) == 232);
const _: () = assert! (std::mem::offset_of! (android_output, bits) == 240);
const _: () = assert! (std::mem::offset_of! (android_output, black_relief) == 248);
const _: () = assert! (std::mem::offset_of! (android_output, relief_background) == 280);
const _: () = assert! (std::mem::offset_of! (android_output, focus_state) == 288);
const _: () = assert! (std::mem::offset_of! (android_output, touch_points) == 296);
const _: () = assert! (std::mem::offset_of! (android_output, extracted_text_hint) == 312);

// androidterm.h 267-277: the anonymous `enum` whose members are the `focus_state` values
// (C 295-296's comment says the field is "actually a boolean").  C gives the enumeration no
// tag, so the port keeps the three members as `c_int` constants, the cm.rs `CM_*` shape.
pub const FOCUS_NONE: c_int = 0;
pub const FOCUS_IMPLICIT: c_int = 1;
pub const FOCUS_EXPLICIT: c_int = 2;

/* Android scroll bars.  */
// androidterm.h 328-379.  Measured: sizeof 72, alignof 8; header 0, window 8, next 16, prev 24,
// x_window 32, top 40, left 44, width 48, height 52, start 56, end 60, dragging 64,
// horizontal 68.  C's own `struct scroll_bar` (the header defines it rather than including
// xterm.h), so the port declares it here too.
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

  /* The X window representing this scroll bar.  */
  pub x_window: Emacs_Window,

  /* The position and size of the scroll bar in pixels, relative to the
     frame.  */
  pub top: c_int,
  pub left: c_int,
  pub width: c_int,
  pub height: c_int,

  /* The starting and ending positions of the handle, relative to the
     handle area (i.e. zero is the top position, not
     SCROLL_BAR_TOP_BORDER).  If they're equal, that means the handle
     hasn't been drawn yet.

     These are not actually the locations where the beginning and end
     are drawn; in order to keep handles from becoming invisible when
     editing large files, we establish a minimum height by always
     drawing handle bottoms VERTICAL_SCROLL_BAR_MIN_HANDLE pixels below
     where they would be normally; the bottom and top are in a
     different coordinate system.  */
  pub start: c_int,
  pub end: c_int,

  /* If the scroll bar handle is currently being dragged by the user,
     this is the number of pixels from the top of the handle to the
     place where the user grabbed it.  If the handle isn't currently
     being dragged, this is -1.  */
  pub dragging: c_int,

  /* True if the scroll bar is horizontal.  */
  pub horizontal: bool,
}
const _: () = assert! (std::mem::size_of::<scroll_bar> () == 72);

/* Return the Android output data for frame F.  */
// androidterm.h 290 and 291 - two names, one body (`((f)->output_data.android)`), so C lets the
// two spellings diverge; the port keeps both macro_rules! (PORTING.md P-37).
macro_rules! FRAME_ANDROID_OUTPUT {
  ($f:expr) => { ((*$f).output_data.android) };
}
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { ((*$f).output_data.android) };
}

/* Return the Android window used for displaying data in frame F.  */
// androidterm.h 294 and 295: `((f)->output_data.android->window)` twice under two names.
macro_rules! FRAME_ANDROID_WINDOW {
  ($f:expr) => { ((*(*$f).output_data.android).window) };
}
macro_rules! FRAME_NATIVE_WINDOW {
  ($f:expr) => { ((*(*$f).output_data.android).window) };
}

/* Return the need-buffer-flip flag for frame F.  */
// androidterm.h 298-299.
macro_rules! FRAME_ANDROID_NEED_BUFFER_FLIP {
  ($f:expr) => { ((*(*$f).output_data.android).need_buffer_flip) };
}

/* Return the drawable used for rendering to frame F and mark the
   frame as needing a buffer flip later.  There's no easy way to run
   code after any drawing command, but code can be run whenever
   someone asks for the handle necessary to draw.  */
// androidterm.h 305-307.  C's comma expression is two statements in the port.
macro_rules! FRAME_ANDROID_DRAWABLE {
  ($f:expr) => {
    {
      (*(*$f).output_data.android).need_buffer_flip = true;
      FRAME_ANDROID_WINDOW! ($f)
    }
  };
}

/* Return whether or not the frame F has been completely drawn.  Used
   while handling async input.  */
// androidterm.h 311-312.
macro_rules! FRAME_ANDROID_COMPLETE_P {
  ($f:expr) => { ((*(*$f).output_data.android).complete) };
}

// androidterm.h 314-315.
macro_rules! FRAME_FONT {
  ($f:expr) => { ((*(*$f).output_data.android).font) };
}
macro_rules! FRAME_FONTSET {
  ($f:expr) => { ((*(*$f).output_data.android).fontset) };
}

// androidterm.h 317-318.
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { ((*(*$f).output_data.android).baseline_offset) };
}

/* This gives the android_display_info structure for the display F is
   on.  */
// androidterm.h 322.
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { ((*(*$f).output_data.android).display_info) };
}

/* Some things for X compatibility.  */
// androidterm.h 325-326.  Both take `f` and ignore it, as C's do.
macro_rules! BLACK_PIX_DEFAULT {
  ($f:expr) => { 0 };
}
macro_rules! WHITE_PIX_DEFAULT {
  ($f:expr) => { 0xffffffff };
}

/* Turning a lisp vector value into a pointer to a struct scroll_bar.  */
// androidterm.h 382.
macro_rules! XSCROLL_BAR {
  ($vec:expr) => { (XVECTOR ($vec) as *mut scroll_bar) };
}

/* This is a chain of structures for all the Android displays
   currently in use.  There is only ever one, but the rest of Emacs is
   written with systems on which there can be many in mind.  */
// androidterm.h 386-389, plus the three file-scope objects the prototypes below name.  The
// definitions belong to the units (androidterm.c's `x_display_list`, androidfns.c's
// `android_frame_parm_handlers`, androidfont.c's `androidfont_driver`, sfntfont-android.c's
// `android_sfntfont_driver`, androidmenu.c's `current_menu_serial`), so the declarations sit in
// a glob-imported module whose import those definitions shadow (PORTING.md 0.1-10/P-21, the
// keyboard.rs `keyboard_objs` shape).
pub mod androidterm_objs {
  use super::*;

  unsafe extern "C" {
    pub static mut x_display_list: *mut android_display_info;    // androidterm.h 389

    // androidterm.h 398: `extern frame_parm_handler android_frame_parm_handlers[];` -
    // `frame_parm_handler` is dispextern.h 3063's typedef, which `include/dispextern.rs` does
    // not carry yet (P-66), so the unbounded array keeps the pointer-sized element stand-in
    // (the lisp.rs `my_edata: [c_char; 0]` shape for `extern char my_edata[];`) and becomes
    // `[frame_parm_handler; 0]` in the commit that lands that alias.
    pub static mut android_frame_parm_handlers: [*mut c_void; 0];

    // androidterm.h 439 and 464: `extern struct font_driver androidfont_driver;` and
    // `extern const struct font_driver android_sfntfont_driver;` - `struct font_driver`'s
    // record is font.h's, and `include/font.rs` is a WIP slice that does not carry it yet, so
    // both objects keep the pointer-width stand-in until the driver interface lands there.
    pub static mut androidfont_driver: *mut c_void;
    pub static mut android_sfntfont_driver: *mut c_void;

    // androidterm.h 448-450, live because `ANDROID_STUBIFY` is `undef` in the frozen table.
    pub static mut current_menu_serial: c_uint;
  }
}
use androidterm_objs::*;

/* Start of function definitions.  These should be a neat subset of
   the same ones in xterm.h, and come in the same order.  */
// androidterm.h 396-485's prototypes.  Every one of them is a definition in one of the
// `src/android*.c` units, so they sit in a module the unit glob-imports and its own
// `#[unsafe(no_mangle)]` definition shadows (PORTING.md 0.1-10/P-21/P-39, the atimer.rs shape).
pub mod androidterm_decls {
  use super::*;

  unsafe extern "C" {
    /* From androidfns.c.  */

    // androidterm.h 399-401.
    pub fn android_free_gcs (f: *mut frame);
    pub fn android_default_font_parameter (f: *mut frame, parm: Lisp_Object);
    /// C: `void android_set_preeditarea (struct window *, int, int);` (androidterm.h 401) -
    /// `struct window`'s record is window.h's, which this header does not include (P-72
    /// rule ③); the pointer keeps the untyped form its use site casts.
    pub fn android_set_preeditarea (w: *mut c_void, x: c_int, y: c_int);

    /* Defined in androidterm.c.  */

    // androidterm.h 405-411.
    pub fn android_term_init ();
    pub fn android_set_window_size (f: *mut frame, change: bool, cols: c_int, rows: c_int);
    pub fn android_set_window_size_and_position (f: *mut frame, cols: c_int, rows: c_int);
    pub fn android_iconify_frame (f: *mut frame);
    pub fn android_make_frame_visible (f: *mut frame);
    pub fn android_make_frame_invisible (f: *mut frame);
    pub fn android_free_frame_resources (f: *mut frame);

    // androidterm.h 413-417.
    pub fn android_parse_color (f: *mut frame, color_name: *const c_char,
                                color: *mut Emacs_Color) -> c_int;
    pub fn android_alloc_nearest_color (f: *mut frame, color: *mut Emacs_Color) -> bool;
    pub fn android_query_colors (f: *mut frame, colors: *mut Emacs_Color, ncolors: c_int);
    pub fn android_clear_under_internal_border (f: *mut frame);

    // androidterm.h 419-420.
    pub fn syms_of_androidterm ();
    pub fn mark_androidterm ();

    /* Defined in androidfns.c.  */

    // androidterm.h 424-427.
    pub fn android_change_tab_bar_height (f: *mut frame, height: c_int);
    pub fn android_change_tool_bar_height (f: *mut frame, height: c_int);
    pub fn android_set_scroll_bar_default_width (f: *mut frame);
    pub fn android_set_scroll_bar_default_height (f: *mut frame);

    // androidterm.h 428-433.
    pub fn android_defined_color (f: *mut frame, color_name: *const c_char,
                                  color: *mut Emacs_Color, alloc: bool,
                                  query: bool) -> bool;
    pub fn android_implicitly_set_name (f: *mut frame, name: Lisp_Object,
                                        oldval: Lisp_Object);
    pub fn android_explicitly_set_name (f: *mut frame, name: Lisp_Object,
                                        oldval: Lisp_Object);

    // androidterm.h 435.
    pub fn syms_of_androidfns ();

    /* Defined in androidfont.c.  */

    // androidterm.h 441-444.
    pub fn init_androidfont ();
    pub fn syms_of_androidfont ();
    /// C: `void android_finalize_font_entity (struct font_entity *);` (androidterm.h 444) -
    /// `struct font_entity`'s record is font.h's, which `include/font.rs` does not carry yet.
    pub fn android_finalize_font_entity (entity: *mut c_void);

    /* Defined in androidmenu.c.  */

    // androidterm.h 454-460.
    pub fn android_menu_show (f: *mut frame, x: c_int, y: c_int, menuflags: c_int,
                              title: Lisp_Object, error_name: *mut *const c_char)
                              -> Lisp_Object;
    pub fn android_popup_dialog (f: *mut frame, header: Lisp_Object,
                                 contents: Lisp_Object) -> Lisp_Object;
    pub fn init_androidmenu ();
    pub fn syms_of_androidmenu ();

    /* Defined in sfntfont-android.c.  */

    // androidterm.h 466-468.
    pub fn sfntfont_android_shrink_scanline_buffer ();
    pub fn init_sfntfont_android ();
    pub fn syms_of_sfntfont_android ();

    /* Defined in androidselect.c.  */

    // androidterm.h 474-483.  The two notification hooks are in the `#ifndef ANDROID_STUBIFY`
    // region (C 472-485), which is live for the frozen table.
    /// C: `struct android_notification_event` has a tag and no record in the ported tree; C
    /// only ever passes a pointer to it (P-72 rule ③).
    pub fn android_notification_deleted (event: *mut c_void, ie: *mut input_event);
    pub fn android_notification_action (event: *mut c_void, ie: *mut input_event,
                                        action: Lisp_Object);

    pub fn init_androidselect ();
    pub fn syms_of_androidselect ();

    /* Defined in androidvfs.c.  */
    pub fn syms_of_androidvfs ();
  }
}
use androidterm_decls::*;

// androidterm.h 489-492's four colour macros are the same four pgtkterm.h 71-76 declares;
// each header defines its own, so the port keeps a set in each mirror (PORTING.md P-26's
// one-spelling rule is about a *unit*'s scope, and no unit includes both headers).
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
macro_rules! RED_FROM_ULONG {
  ($color:expr) => { ((($color) >> 16) & 0xff) };
}
macro_rules! GREEN_FROM_ULONG {
  ($color:expr) => { ((($color) >> 8) & 0xff) };
}
macro_rules! BLUE_FROM_ULONG {
  ($color:expr) => { (($color) & 0xff) };
}
