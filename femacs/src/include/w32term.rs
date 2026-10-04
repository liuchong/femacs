// femacs: port of src/w32term.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  `w32term.h` is the Windows GUI's interface.  It has **no
// platform `#if` of its own** - C 21-25 include `w32gui.h`, `frame.h` and `atimer.h` and then the
// file is unconditional text - so unlike `include/nsterm.rs` its declarations are *live* for
// `tools/extract-abi.py`: this ledger's checklist is C 1's inventory, not an empty one (119
// macros, 19 constants, 140 struct fields, 25 file-scope objects; `docs/port-inventory.md` 653
// prints the same 119/19/129/25 for it).  The header is only ever *included* by a Windows GUI
// unit (`#ifdef HAVE_NTGUI` in `w32fns.c`, `w32term.c`, ...), and `HAVE_NTGUI`/`WINDOWSNT` are
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so **the whole body carries
// `#[cfg (feature = "windowsnt")]`** - P-51's arm for the non-frozen side, the
// `include/w32font.rs` (3.176) reading of a Windows header, spelled with C's own names
// (`MSG`, `HWND`, `HDC`, `COLORREF`, `RECT`, `WINDOWPLACEMENT`, `CRITICAL_SECTION`, `DWORD`,
// `LONG`, `LPBYTE`, `HANDLE`, `WINAPI`, ...) because that target supplies them from
// `<windows.h>`, which has no mirror here.  Nothing in the arm is compiled on this target, so no
// layout assertion is written for any of the ten records (the `include/macfont.rs` 26-27 rule);
// `include/w32gui.rs` is included the way C 21 includes `w32gui.h`, so `Emacs_Pixmap` and the
// X-compatibility block come from there and are not repeated here.
//
// **`tools/check-cond-eval.py src/w32term.h` -> `17 conditions, 0 unknown, 1 disagreements
// [NOT VERIFIED]`**: C 33-40's `#if USE_STACK_LISP_OBJECTS && !defined _WIN64 && !defined
// __x86_64__ && __GNUC__ + (__GNUC_MINOR__ > 1) >= 5` is **True to clang and False to the
// evaluator** (`USE_STACK_LISP_OBJECTS` is not in the macro table; `lisp.h` 5721-5736 is where it
// is defined for the Lisp build), so this header is *not* on `cond-eval-verified.txt` and the
// tool reads its conditions conservatively - both arms of every `#if` below stay live.  That is
// why the mirror carries both arms (`ALIGN_STACK`, `SCROLL_BAR_PACK`/`UNPACK`,
// `w32_kbdhook_active`, the `NTGUI_UNICODE` block); the one name that only the attribute arm
// defines is recorded in a `// C:` line, because an attribute spelling is not a Rust item.
//
// Inner conditions, each following the value the frozen table (or the C chain) gives it:
//   * `_WIN64` is not in the table (a compiler predefine); the port's own feature is `win64`
//     (`femacs/xtask/Cargo.toml` 561, `# _WIN64`, the `nt/inc/include/stdint.rs` 34 reading), so
//     C 536-552's 64-bit arm and its 32-bit `#else` carry `win64` and `not (win64)`.
//   * `WINDOWSNT` (C 803) is `undef` here; the file itself is the Windows side, so that arm is
//     carried unconditionally inside the umbrella and the `#else` arm (`#define
//     w32_kbdhook_active 0`) is recorded.
//   * `NTGUI_UNICODE` (C 892) is defined by `conf_post.h` 169-170 only for
//     `defined CYGWIN && defined HAVE_NTGUI`, so that arm carries `cygwin` and the `#else` arm -
//     the MinGW/Windows GUI one (`GUISTR(x) x`, `guichar_t` is `char`) - is live; `CYGWIN`
//     (C 927) is `undef` for the same reason, and `EMACSDEBUG` (C 887) too.
//   * The nine `#ifndef` blocks (C 664-686, 831-836, 850-861) are "define it if the SDK does not"
//     guards, carried as constants with C's values (the `include/widget.rs` `XtNfont` reading).
//
// Degradations (P-55/P-72 rule 3): `struct image`, `struct face`, `struct input_event` and
// `struct glyph_matrix` are only pointed at; `frame_parm_handler` is `dispextern.h`'s and
// `include/dispextern.rs` does not carry it yet, so `w32_frame_parm_handlers[]`'s elements are
// `*mut c_void` for now (the `include/androidterm.rs` 506-511 reading).

include!("w32gui.rs");           // C 21: `#include "w32gui.h"` (`Emacs_Pixmap`, `Window`, `RECT`)
include!("frame.rs");            // C 22: `#include "frame.h"` (`struct frame`)
include!("atimer.rs");           // C 23: `#include "atimer.h"`

/* C 26-40: "Stack alignment stuff.  Every CALLBACK and thread function should have the
   ALIGN_STACK attribute if it manipulates Lisp objects" - the attribute arm is inside `#ifdef
   __GNUC__` (`__GNUC__` is 4 in the frozen table) and the one disagreement above.  A Rust item
   cannot express `__attribute__((force_align_arg_pointer))`, so the empty arm is what the mirror
   carries and the attribute is recorded.  */
// C: #if USE_STACK_LISP_OBJECTS && !defined _WIN64 && !defined __x86_64__ && __GNUC__ + (__GNUC_MINOR__ > 1) >= 5
// C: # define ALIGN_STACK __attribute__((force_align_arg_pointer))
// C: #else
// C: # define ALIGN_STACK
// C: #endif
/// C 37-39: `#define ALIGN_STACK` (the `#else` arm).
#[cfg (feature = "windowsnt")]
macro_rules! ALIGN_STACK {
  () => {};
}

/// C 43-44: the two pixel defaults, C's `PALETTERGB` macro applied to `(0,0,0)`/`(255,255,255)`.
/// C: #define BLACK_PIX_DEFAULT(f) PALETTERGB(0,0,0)
/// C: #define WHITE_PIX_DEFAULT(f) PALETTERGB(255,255,255)
#[cfg (feature = "windowsnt")]
macro_rules! BLACK_PIX_DEFAULT {
  ($f:expr) => { PALETTERGB (0, 0, 0) };
}
#[cfg (feature = "windowsnt")]
macro_rules! WHITE_PIX_DEFAULT {
  ($f:expr) => { PALETTERGB (255, 255, 255) };
}

/// C 46: `#define CP_DEFAULT 1004` - "Indicates whether we are in the readsocket call and the
/// message we are processing in the current loop".
#[cfg (feature = "windowsnt")]
pub const CP_DEFAULT: c_int = 1004;

/* C 51-52: the current message and its default-handling flag.  */
/// C: extern MSG CurMsg;
/// C: extern BOOL bUseDflt;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut CurMsg: MSG;
  pub static mut bUseDflt: BOOL;
}

/* C 57-66: "Structure recording bitmaps and reference count.  If REFCOUNT is 0 then this record
   is free to be reused."  */
/// C: struct w32_bitmap_record { Emacs_Pixmap pixmap; char *file; Emacs_Pixmap stipple;
/// C: HINSTANCE hinst; int refcount; int height, width, depth; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32_bitmap_record
{
  pub pixmap: Emacs_Pixmap,
  pub file: *mut c_char,
  pub stipple: Emacs_Pixmap,
  pub hinst: HINSTANCE, /* Used to load the file */
  pub refcount: c_int,
  /* Record some info about this pixmap.  */
  pub height: c_int,
  pub width: c_int,
  pub depth: c_int,
}

/* C 68-73: the palette entry chain.  C 71-73 is an `#if 0` block (`unsigned refcount;`), which
   the extractor strips and the mirror records (PORTING.md P-08).  */
/// C: struct w32_palette_entry { struct w32_palette_entry * next; PALETTEENTRY entry; };
// C: #if 0 / unsigned refcount; / #endif
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32_palette_entry
{
  pub next: *mut w32_palette_entry,
  pub entry: PALETTEENTRY,
}

/// C 74-77: `extern void w32_regenerate_palette (struct frame *f); extern void
/// w32_fullscreen_rect (HWND hwnd, int fsmode, RECT normal, RECT *rect);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_regenerate_palette (f: *mut frame);
  pub fn w32_fullscreen_rect (hwnd: HWND, fsmode: c_int, normal: RECT, rect: *mut RECT);
}

/* C 83-221: "For each display (currently only one on w32), we have a structure that records
   information about it."  `Window` and `Emacs_Pixmap` come from `include/w32gui.rs` (C 21's
   `w32gui.h`), `Mouse_HLInfo` from `dispextern.rs`, `Time` from `systime.h` (through
   `termhooks.h`/`frame.h`), and `HCURSOR`/`HPALETTE`/`RECT` are `<windows.h>`'s.  */
/// C: struct w32_display_info { struct w32_display_info *next; struct terminal *terminal;
/// C: Lisp_Object name_list_element; int reference_count; double resx, resy; int n_planes,
/// C: n_cbits, grabbed; ptrdiff_t icon_bitmap_id; Window root_window; HCURSOR
/// C: vertical_scroll_bar_cursor, horizontal_scroll_bar_cursor; const char *rdb; int
/// C: has_palette; struct w32_palette_entry *color_list; unsigned num_colors; HPALETTE
/// C: palette; int regen_palette; int faked_key; int smallest_char_width,
/// C: smallest_font_height; Emacs_GC *scratch_cursor_gc; Mouse_HLInfo mouse_highlight;
/// C: char *w32_id_name; int n_fonts; struct w32_bitmap_record *bitmaps; ptrdiff_t
/// C: bitmaps_size, bitmaps_last; struct frame *w32_focus_frame, *w32_focus_event_frame,
/// C: *highlight_frame, *w32_pending_autoraise_frame, *last_mouse_frame,
/// C: *last_mouse_motion_frame, *last_mouse_glyph_frame; int last_mouse_motion_x,
/// C: last_mouse_motion_y; RECT last_mouse_glyph; struct scroll_bar *last_mouse_scroll_bar;
/// C: int last_mouse_scroll_bar_pos; Time last_mouse_movement_time;
/// C: int cursor_display_counter; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32_display_info
{
  /* Chain of all w32_display_info structures.  */
  pub next: *mut w32_display_info,

  /* The generic display parameters corresponding to this w32 display.  */
  pub terminal: *mut terminal,

  /* This is a cons cell of the form (NAME . FONT-LIST-CACHE).  */
  pub name_list_element: Lisp_Object,

  /* Number of frames that are on this display.  */
  pub reference_count: c_int,

  /* Dots per inch of the screen.  */
  pub resx: f64,
  pub resy: f64,

  /* Number of planes on this screen.  */
  pub n_planes: c_int,

  /* Number of bits per pixel on this screen.  */
  pub n_cbits: c_int,

  /* Mask of things that cause the mouse to be grabbed.  */
  pub grabbed: c_int,

  /* Emacs bitmap-id of the default icon bitmap for this frame.
     Or -1 if none has been allocated yet.  */
  pub icon_bitmap_id: ptrdiff_t,

  /* The root window of this screen.  */
  pub root_window: Window,

  /* The cursor to use for vertical scroll bars.  */
  pub vertical_scroll_bar_cursor: HCURSOR,

  /* The cursor to use for horizontal scroll bars.  */
  pub horizontal_scroll_bar_cursor: HCURSOR,

  /* Resource data base */
  pub rdb: *const c_char,

  /* color palette information.  */
  pub has_palette: c_int,
  pub color_list: *mut w32_palette_entry,
  pub num_colors: c_uint,
  pub palette: HPALETTE,

  /* deferred action flags checked when starting frame update.  */
  pub regen_palette: c_int,

  /* Keystroke that has been faked by Emacs and will be ignored when
     received; value is reset after key is received.  */
  pub faked_key: c_int,

  /* Minimum width over all characters in all fonts in font_table.  */
  pub smallest_char_width: c_int,

  /* Minimum font height over all fonts in font_table.  */
  pub smallest_font_height: c_int,

  /* Reusable Graphics Context for drawing a cursor in a non-default face. */
  pub scratch_cursor_gc: *mut Emacs_GC,

  /* Information about the range of text currently shown in
     mouse-face.  */
  pub mouse_highlight: Mouse_HLInfo,

  pub w32_id_name: *mut c_char,

  /* The number of fonts actually stored in w32_font_table. … */
  pub n_fonts: c_int,

  /* Pointer to bitmap records.  */
  pub bitmaps: *mut w32_bitmap_record,

  /* Allocated size of bitmaps field.  */
  pub bitmaps_size: ptrdiff_t,

  /* Last used bitmap index.  */
  pub bitmaps_last: ptrdiff_t,

  /* The frame (if any) which has the window that has keyboard focus. … */
  pub w32_focus_frame: *mut frame,

  /* The last frame mentioned in a FocusIn or FocusOut event. … */
  pub w32_focus_event_frame: *mut frame,

  /* The frame which currently has the visual highlight, and should get
     keyboard input … */
  pub highlight_frame: *mut frame,

  /* The frame waiting to be auto-raised in w32_read_socket.  */
  pub w32_pending_autoraise_frame: *mut frame,

  /* The frame where the mouse was last time we reported a mouse event.  */
  pub last_mouse_frame: *mut frame,

  /* The frame where the mouse was last time we reported a mouse motion.  */
  pub last_mouse_motion_frame: *mut frame,

  /* The frame where the mouse was last time we reported a mouse position.  */
  pub last_mouse_glyph_frame: *mut frame,

  /* Position where the mouse was last time we reported a motion. … */
  pub last_mouse_motion_x: c_int,
  pub last_mouse_motion_y: c_int,

  /* Where the mouse was last time we reported a mouse position. … */
  pub last_mouse_glyph: RECT,

  /* The scroll bar in which the last motion event occurred.  */
  pub last_mouse_scroll_bar: *mut scroll_bar,

  /* Mouse position on the scroll bar above. … */
  pub last_mouse_scroll_bar_pos: c_int,

  /* Time of last mouse movement.  */
  pub last_mouse_movement_time: Time,

  /* Value returned by last call of ShowCursor.  */
  pub cursor_display_counter: c_int,
}

/* C 223-228: the display chain, the single Windows display and the two menu objects "set by
   w32fns.c and examined in w32term.c".  */
/// C: extern struct w32_display_info *x_display_list;
/// C: extern struct w32_display_info one_w32_display_info;
/// C: extern HMENU current_popup_menu;
/// C: extern int menubar_in_use;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut x_display_list: *mut w32_display_info;
  pub static mut one_w32_display_info: w32_display_info;
  pub static mut current_popup_menu: HMENU;
  pub static mut menubar_in_use: c_int;
}

/* C 230-274: the entry points, in C's order, ending with the frame-parameter handler table.  */
/// C 230: `extern struct frame *w32_window_to_frame (struct w32_display_info *, HWND);`
/// C 232: `extern void w32_real_positions (struct frame *f, int *xptr, int *yptr);`
/// C 234: `extern void w32_clear_under_internal_border (struct frame *);`
/// C 236-237: `extern void w32_change_tab_bar_height (struct frame *, int); extern void
/// w32_change_tool_bar_height (struct frame *, int);`
/// C 238: `extern void w32_implicitly_set_name (struct frame *, Lisp_Object, Lisp_Object);`
/// C 239-240: `extern void w32_set_scroll_bar_default_width (struct frame *); extern void
/// w32_set_scroll_bar_default_height (struct frame *);`
/// C 243-244: `extern struct w32_display_info *w32_term_init (Lisp_Object, char *, char *);`
/// C 245-246: `extern bool w32_defined_color (struct frame *, const char *, Emacs_Color *, bool,
/// bool);`
/// C 247-248: `extern int w32_display_pixel_height (struct w32_display_info *); extern int
/// w32_display_pixel_width (struct w32_display_info *);`
/// C 249: `extern void initialize_frame_menubar (struct frame *);`
/// C 250: `extern void w32_dialog_in_progress (Lisp_Object in_progress);`
/// C 251-252: `extern void w32_query_frame_background_color (struct frame *f, Emacs_Color
/// *bgcolor);`
/// C 254-255: `extern void w32_make_frame_visible (struct frame *f); extern void
/// w32_make_frame_invisible (struct frame *f);`
/// C 256-258: `extern void w32_iconify_frame (struct frame *f); extern void
/// w32_free_frame_resources (struct frame *); extern void w32_wm_set_size_hint (struct frame *,
/// long, bool);`
/// C 261: `extern int w32_kbd_patch_key (KEY_EVENT_RECORD *event, int cpId);`
/// C 262: `extern int w32_kbd_mods_to_emacs (DWORD mods, WORD key);`
/// C 265-266: `extern void w32con_hide_cursor (void); extern void w32con_show_cursor (void);`
/// C 269-271: `extern const char *w32_get_string_resource (void *v_rdb, const char *name, const
/// char *class);`
/// C 274: `extern frame_parm_handler w32_frame_parm_handlers[];`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_window_to_frame (dpyinfo: *mut w32_display_info, hwnd: HWND) -> *mut frame;
  pub fn w32_real_positions (f: *mut frame, xptr: *mut c_int, yptr: *mut c_int);
  pub fn w32_clear_under_internal_border (f: *mut frame);
  pub fn w32_change_tab_bar_height (f: *mut frame, height: c_int);
  pub fn w32_change_tool_bar_height (f: *mut frame, height: c_int);
  pub fn w32_implicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn w32_set_scroll_bar_default_width (f: *mut frame);
  pub fn w32_set_scroll_bar_default_height (f: *mut frame);
  pub fn w32_term_init (display_name: Lisp_Object, resource_name: *mut c_char,
                        resource_class: *mut c_char) -> *mut w32_display_info;
  pub fn w32_defined_color (f: *mut frame, color_name: *const c_char,
                            color_def: *mut Emacs_Color, alloc: bool,
                            make_index: bool) -> bool;
  pub fn w32_display_pixel_height (dpyinfo: *mut w32_display_info) -> c_int;
  pub fn w32_display_pixel_width (dpyinfo: *mut w32_display_info) -> c_int;
  pub fn initialize_frame_menubar (f: *mut frame);
  pub fn w32_dialog_in_progress (in_progress: Lisp_Object);
  pub fn w32_query_frame_background_color (f: *mut frame, bgcolor: *mut Emacs_Color);
  pub fn w32_make_frame_visible (f: *mut frame);
  pub fn w32_make_frame_invisible (f: *mut frame);
  pub fn w32_iconify_frame (f: *mut frame);
  pub fn w32_free_frame_resources (f: *mut frame);
  pub fn w32_wm_set_size_hint (f: *mut frame, long_arg: c_long, bool_arg: bool);
  pub fn w32_kbd_patch_key (event: *mut KEY_EVENT_RECORD, cpId: c_int) -> c_int;
  pub fn w32_kbd_mods_to_emacs (mods: DWORD, key: WORD) -> c_int;
  pub fn w32con_hide_cursor ();
  pub fn w32con_show_cursor ();
  pub fn w32_get_string_resource (v_rdb: *mut c_void, name: *const c_char,
                                  class: *const c_char) -> *const c_char;
  /// `frame_parm_handler` is `dispextern.h` 3063's typedef and `include/dispextern.rs` does not
  /// carry it yet, so the array's elements are `*mut c_void` for now - the
  /// `include/androidterm.rs` 506-511 reading of its own `android_frame_parm_handlers[]`.
  pub static mut w32_frame_parm_handlers: [*mut c_void; 0];
}

/// C 280: `#define PIX_TYPE COLORREF` - a *type* macro, so it is a `macro_rules!` with no
/// metavariables (the `include/w32gui.rs` `NativeRectangle` spelling): a `pub type` alias is not
/// what the gate reads for a macro, and C uses the name in type position.
#[cfg (feature = "windowsnt")]
macro_rules! PIX_TYPE {
  () => { COLORREF };
}

/* C 284-291: "Put some things in x_output for compatibility.  NTEMACS_TODO: Move all common
   things here to eliminate unnecessary diffs between X and w32 code."  */
/// C: struct x_output { int focus_state; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct x_output
{
  /// Keep track of focus.  May be EXPLICIT if we received a FocusIn for this
  /// frame, or IMPLICIT if we received an EnterNotify.
  /// FocusOut and LeaveNotify clears EXPLICIT/IMPLICIT.
  pub focus_state: c_int,
}

/* C 293-306: `enum { FOCUS_NONE = 0, FOCUS_IMPLICIT = 1, FOCUS_EXPLICIT = 2 };` - "Values for
   focus_state, used as bit mask", declared as C's own type plus one constant per member
   (PORTING.md 1.3).  */
/// C: enum { FOCUS_NONE = 0, FOCUS_IMPLICIT = 1, FOCUS_EXPLICIT = 2 };
#[cfg (feature = "windowsnt")]
pub type focus_state_t = c_int;
#[cfg (feature = "windowsnt")]
pub const FOCUS_NONE: focus_state_t = 0;
#[cfg (feature = "windowsnt")]
pub const FOCUS_IMPLICIT: focus_state_t = 1;
#[cfg (feature = "windowsnt")]
pub const FOCUS_EXPLICIT: focus_state_t = 2;

/* C 309-456: "Each W32 frame object points to its own struct w32_display object in the
   output_data.w32 field."  The bitfields (`hourglass_p` C 378, `explicit_parent` C 394,
   `asked_for_visible` C 397, `menubar_active` C 400, `paint_buffer_dirty` C 436,
   `want_paint_buffer` C 439) are recorded rather than packed: PORTING.md 1.3 / P-80 require the
   real compiler's byte dump before a storage unit may stand for a bitfield, and this record
   cannot be compiled on this target.  C 402-408's `struct relief { Emacs_GC *gc; unsigned long
   pixel; } black_relief, white_relief;` is a nested record, so the port declares it as C's own
   tag (`include/androidterm.rs` 222 declares the same tag for its own output record; the two
   never meet, because that mirror is Android's and this one is Windows').  */
/// C: struct w32_output { struct x_output x_compatible; HMENU menubar_widget; HPALETTE
/// C: old_palette; Emacs_GC *cursor_gc; Window window_desc, parent_desc; struct font *font;
/// C: int baseline_offset, fontset; COLORREF cursor_pixel, border_pixel, mouse_pixel,
/// C: cursor_foreground_pixel, scroll_bar_foreground_pixel, scroll_bar_background_pixel; HCURSOR
/// C: text_cursor … bottom_left_corner_cursor (15); [bitfields hourglass_p, explicit_parent,
/// C: asked_for_visible, menubar_active]; HCURSOR current_cursor; DWORD dwStyle; struct relief
/// C: { Emacs_GC *gc; unsigned long pixel; } black_relief, white_relief; unsigned long
/// C: relief_background; WINDOWPLACEMENT normal_placement; int prev_fsmode; HBITMAP paint_buffer;
/// C: HDC paint_dc, paint_buffer_handle; HGDIOBJ paint_dc_object; int paint_buffer_width,
/// C: paint_buffer_height; [bitfields paint_buffer_dirty, want_paint_buffer]; int
/// C: touch_ids[MAX_TOUCH_POINTS]; LONG touch_x[MAX_TOUCH_POINTS], touch_y[MAX_TOUCH_POINTS];
/// C: EMACS_INT touch_base; DWORD tool_bar_dwID; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct relief
{
  pub gc: *mut Emacs_GC,
  pub pixel: c_ulong,
}

/// C 441: `#define MAX_TOUCH_POINTS 10` - C defines it inside the record, above the two arrays
/// that use it; the port keeps it there (`[T; MAX_TOUCH_POINTS]` needs the constant in scope).
#[cfg (feature = "windowsnt")]
pub const MAX_TOUCH_POINTS: usize = 10;

#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32_output
{
  /* Placeholder for things accessed through output_data.x.  */
  pub x_compatible: x_output,

  /* Menubar "widget" handle.  */
  pub menubar_widget: HMENU,

  /* Original palette (used to deselect real palette after drawing) */
  pub old_palette: HPALETTE,

  /* Here are the Graphics Contexts for the default font.  */
  pub cursor_gc: *mut Emacs_GC,          /* cursor drawing */

  /* The window used for this frame. … */
  pub window_desc: Window,

  /* The window that is the parent of this window. … */
  pub parent_desc: Window,

  /* Default ASCII font of this frame. */
  pub font: *mut font,

  /* The baseline offset of the default ASCII font.  */
  pub baseline_offset: c_int,

  /* If a fontset is specified for this frame instead of font, this
     value contains an ID of the fontset, else -1.  */
  pub fontset: c_int,

  /* Pixel values used for various purposes. … */
  pub cursor_pixel: COLORREF,
  pub border_pixel: COLORREF,
  pub mouse_pixel: COLORREF,
  pub cursor_foreground_pixel: COLORREF,

  /* Foreground color for scroll bars. … */
  pub scroll_bar_foreground_pixel: COLORREF,

  /* Background color for scroll bars. … */
  pub scroll_bar_background_pixel: COLORREF,

  /* Descriptor for the cursor in use for this window.  */
  pub text_cursor: HCURSOR,
  pub nontext_cursor: HCURSOR,
  pub modeline_cursor: HCURSOR,
  pub hand_cursor: HCURSOR,
  pub hourglass_cursor: HCURSOR,
  pub horizontal_drag_cursor: HCURSOR,
  pub vertical_drag_cursor: HCURSOR,
  pub left_edge_cursor: HCURSOR,
  pub top_left_corner_cursor: HCURSOR,
  pub top_edge_cursor: HCURSOR,
  pub top_right_corner_cursor: HCURSOR,
  pub right_edge_cursor: HCURSOR,
  pub bottom_right_corner_cursor: HCURSOR,
  pub bottom_edge_cursor: HCURSOR,
  pub bottom_left_corner_cursor: HCURSOR,

  // C 378: `unsigned hourglass_p : 1;` - "Non-zero means hourglass cursor is currently
  // displayed."  Recorded, not packed (see the file note).
  /* Non-hourglass cursor that is currently active.  */
  pub current_cursor: HCURSOR,

  /* The window style for this frame. … */
  pub dwStyle: DWORD,

  // C 394/397/400: `unsigned explicit_parent : 1; unsigned asked_for_visible : 1;
  // unsigned menubar_active : 1;` - recorded, not packed.

  /* Relief GCs, colors etc.  */
  pub black_relief: relief,
  pub white_relief: relief,

  /* The background for which the above relief GCs were set up. … */
  pub relief_background: c_ulong,

  /* Frame geometry and full-screen mode before it was resized … */
  pub normal_placement: WINDOWPLACEMENT,
  pub prev_fsmode: c_int,

  /* The back buffer if there is an ongoing double-buffered drawing
     operation.  */
  pub paint_buffer: HBITMAP,

  /* The handle of the back buffer and a DC that ought to be released
     alongside the back buffer.  */
  pub paint_dc: HDC,
  pub paint_buffer_handle: HDC,

  /* The object previously selected into `paint_dc'.  */
  pub paint_dc_object: HGDIOBJ,

  /* The width and height of `paint_buffer'.  */
  pub paint_buffer_width: c_int,
  pub paint_buffer_height: c_int,

  // C 436/439: `unsigned paint_buffer_dirty : 1; unsigned want_paint_buffer : 1;` - recorded,
  // not packed.

  /* Array of dwIDs of presently active touch points, or -1 when
     unpopulated.  */
  pub touch_ids: [c_int; MAX_TOUCH_POINTS],

  /* X and Y coordinates of active touchpoints.  */
  pub touch_x: [LONG; MAX_TOUCH_POINTS],
  pub touch_y: [LONG; MAX_TOUCH_POINTS],

  /* Base value for touch point identifiers registered by this
     frame.  */
  pub touch_base: EMACS_INT,

  /* Windows identifier of any touch point reserved for the tool bar, or
     -1.  */
  pub tool_bar_dwID: DWORD,
}

/// C 458: `extern struct w32_output w32term_display;`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut w32term_display: w32_output;
}

/* C 461-475: the frame accessors.  C writes them as macros over `->` chains; the port keeps the
   names and casts the union member once (the `include/haikuterm.rs` 393-440 shape for the same
   family), because `frame.rs`'s `output_data` member is still `*mut c_void` (P-72 rule 3).  */
/// C: #define FRAME_OUTPUT_DATA(f) ((f)->output_data.w32)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { ((*$f).output_data.w32) };
}
/// C: #define FRAME_W32_WINDOW(f) ((f)->output_data.w32->window_desc)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_W32_WINDOW {
  ($f:expr) => { ((*((*$f).output_data.w32 as *mut w32_output)).window_desc) };
}
/// C: #define FRAME_NATIVE_WINDOW(f) FRAME_W32_WINDOW (f)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_NATIVE_WINDOW {
  ($f:expr) => { FRAME_W32_WINDOW! ($f) };
}
/// C: #define FRAME_FONT(f) ((f)->output_data.w32->font)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_FONT {
  ($f:expr) => { ((*((*$f).output_data.w32 as *mut w32_output)).font) };
}
/// C: #define FRAME_FONTSET(f) ((f)->output_data.w32->fontset)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_FONTSET {
  ($f:expr) => { ((*((*$f).output_data.w32 as *mut w32_output)).fontset) };
}
/// C: #define FRAME_BASELINE_OFFSET(f) ((f)->output_data.w32->baseline_offset)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { ((*((*$f).output_data.w32 as *mut w32_output)).baseline_offset) };
}
/// C: #define FRAME_DISPLAY_INFO(f) ((void) (f), (&one_w32_display_info))
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { { (void) ($f); (&raw mut one_w32_display_info) } };
}
/// C: #define FRAME_NORMAL_PLACEMENT(F) ((F)->output_data.w32->normal_placement)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_NORMAL_PLACEMENT {
  ($F:expr) => { ((*((*$F).output_data.w32 as *mut w32_output)).normal_placement) };
}
/// C: #define FRAME_PREV_FSMODE(F) ((F)->output_data.w32->prev_fsmode)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_PREV_FSMODE {
  ($F:expr) => { ((*((*$F).output_data.w32 as *mut w32_output)).prev_fsmode) };
}

/* C 483-530: "We represent scroll bars as lisp vectors. … We use struct scroll_bar as a template
   for accessing fields of the vector."  */
/// C: struct scroll_bar { union vectorlike_header header; Lisp_Object window, next, prev;
/// C: Lisp_Object w32_window_low, w32_window_high, w32_widget_low, w32_widget_high;
/// C: int top, left, width, height; int start, end; int dragging; bool horizontal; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct scroll_bar
{
  /* This field is shared by all vectors.  */
  pub header: vectorlike_header,

  /* The window we're a scroll bar for.  */
  pub window: Lisp_Object,

  /* The next and previous in the chain of scroll bars in this frame.  */
  pub next: Lisp_Object,
  pub prev: Lisp_Object,

  /* The window representing this scroll bar.  Since this is a full
     32-bit quantity, we store it split into two 32-bit values.  */
  pub w32_window_low: Lisp_Object,
  pub w32_window_high: Lisp_Object,

  /* Same as above for the widget.  */
  pub w32_widget_low: Lisp_Object,
  pub w32_widget_high: Lisp_Object,

  /* The position and size of the scroll bar in pixels, relative to the
     frame.  */
  pub top: c_int,
  pub left: c_int,
  pub width: c_int,
  pub height: c_int,

  /* The starting and ending positions of the handle … */
  pub start: c_int,
  pub end: c_int,

  /* If the scroll bar handle is currently being dragged by the user … */
  pub dragging: c_int,

  /* true if the scroll bar is horizontal.  */
  pub horizontal: bool,
}

/// C 533: `#define XSCROLL_BAR(vec) ((struct scroll_bar *) XVECTOR (vec))`
#[cfg (feature = "windowsnt")]
macro_rules! XSCROLL_BAR {
  ($vec:expr) => { (XVECTOR ($vec) as *mut scroll_bar) };
}

/* C 536-552: the two packing pairs, one per `_WIN64` arm ("Building a 64-bit C integer from two
   32-bit lisp integers" / "a 32-bit C unsigned integer from two 16-bit lisp integers").  The
   feature is the tree's own `win64` (`femacs/xtask/Cargo.toml` 561, `# _WIN64`).  */
/// C: #define SCROLL_BAR_PACK(low, high) (XFIXNUM (high) << 32 | XFIXNUM (low))
#[cfg (feature = "windowsnt")]
#[cfg (feature = "win64")]
macro_rules! SCROLL_BAR_PACK {
  ($low:expr, $high:expr) => { XFIXNUM ($high) << 32 | XFIXNUM ($low) };
}
/// C: #define SCROLL_BAR_UNPACK(low, high, int64) (XSETINT ((low), ((DWORDLONG)(int64)) &
/// C: 0xffffffff), XSETINT ((high), ((DWORDLONG)(int64) >> 32) & 0xffffffff))
#[cfg (feature = "windowsnt")]
#[cfg (feature = "win64")]
macro_rules! SCROLL_BAR_UNPACK {
  ($low:expr, $high:expr, $int64:expr) => {
    {
      XSETINT (($low), ($int64) as DWORDLONG & 0xffffffff);
      XSETINT (($high), ($int64) as DWORDLONG >> 32 & 0xffffffff);
    }
  };
}
/// C: #else /* not _WIN64 */ / #define SCROLL_BAR_PACK(low, high) ((UINT_PTR)(XFIXNUM (high)
/// C: << 16 | XFIXNUM (low)))
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "win64"))]
macro_rules! SCROLL_BAR_PACK {
  ($low:expr, $high:expr) => { (XFIXNUM ($high) << 16 | XFIXNUM ($low)) as UINT_PTR };
}
/// C: #define SCROLL_BAR_UNPACK(low, high, int32) (XSETINT ((low), (int32) & 0xffff),
/// C: XSETINT ((high), ((int32) >> 16) & 0xffff))
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "win64"))]
macro_rules! SCROLL_BAR_UNPACK {
  ($low:expr, $high:expr, $int32:expr) => {
    {
      XSETINT (($low), ($int32) & 0xffff);
      XSETINT (($high), ($int32) >> 16 & 0xffff);
    }
  };
}

/// C: #define SCROLL_BAR_W32_WINDOW(ptr) ((Window) SCROLL_BAR_PACK ((ptr)->w32_window_low,
/// C: (ptr)->w32_window_high))
#[cfg (feature = "windowsnt")]
macro_rules! SCROLL_BAR_W32_WINDOW {
  ($ptr:expr) => {
    (SCROLL_BAR_PACK! ((*$ptr).w32_window_low, (*$ptr).w32_window_high) as Window)
  };
}
/// C: #define SET_SCROLL_BAR_W32_WINDOW(ptr, id) (SCROLL_BAR_UNPACK
/// C: ((ptr)->w32_window_low, (ptr)->w32_window_high, (intptr_t) id))
#[cfg (feature = "windowsnt")]
macro_rules! SET_SCROLL_BAR_W32_WINDOW {
  ($ptr:expr, $id:expr) => {
    {
      SCROLL_BAR_UNPACK! ((*$ptr).w32_window_low, (*$ptr).w32_window_high, ($id) as intptr_t);
    }
  };
}

/* C 561-631: the scroll-bar geometry accessors.  C's bodies are the four arithmetic expressions
   its own borders and minima define; the port keeps every name and the order.  */
/// C: #define VERTICAL_SCROLL_BAR_INSIDE_WIDTH(f,width) ((width) -
/// C: VERTICAL_SCROLL_BAR_LEFT_BORDER - VERTICAL_SCROLL_BAR_RIGHT_BORDER)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_INSIDE_WIDTH {
  ($f:expr, $width:expr) => {
    { ($width) - VERTICAL_SCROLL_BAR_LEFT_BORDER! () - VERTICAL_SCROLL_BAR_RIGHT_BORDER! () }
  };
}
/// C: #define VERTICAL_SCROLL_BAR_TOP_RANGE(f,height) (VERTICAL_SCROLL_BAR_INSIDE_HEIGHT (f,
/// C: height) - VERTICAL_SCROLL_BAR_MIN_HANDLE)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_TOP_RANGE {
  ($f:expr, $height:expr) => {
    { VERTICAL_SCROLL_BAR_INSIDE_HEIGHT! ($f, $height) - VERTICAL_SCROLL_BAR_MIN_HANDLE! () }
  };
}
/// C: #define VERTICAL_SCROLL_BAR_INSIDE_HEIGHT(f,height) ((height) -
/// C: VERTICAL_SCROLL_BAR_TOP_BORDER - VERTICAL_SCROLL_BAR_BOTTOM_BORDER)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_INSIDE_HEIGHT {
  ($f:expr, $height:expr) => {
    { ($height) - VERTICAL_SCROLL_BAR_TOP_BORDER! () - VERTICAL_SCROLL_BAR_BOTTOM_BORDER! () }
  };
}
/// C: #define HORIZONTAL_SCROLL_BAR_INSIDE_HEIGHT(f,height) ((height) -
/// C: HORIZONTAL_SCROLL_BAR_TOP_BORDER - HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_INSIDE_HEIGHT {
  ($f:expr, $height:expr) => {
    {
      ($height) - HORIZONTAL_SCROLL_BAR_TOP_BORDER! () - HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER! ()
    }
  };
}
/// C: #define HORIZONTAL_SCROLL_BAR_LEFT_RANGE(f,width) (HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH (f,
/// C: width) - HORIZONTAL_SCROLL_BAR_MIN_HANDLE)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_LEFT_RANGE {
  ($f:expr, $width:expr) => {
    { HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH! ($f, $width) - HORIZONTAL_SCROLL_BAR_MIN_HANDLE! () }
  };
}
/// C: #define HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH(f,width) ((width) -
/// C: HORIZONTAL_SCROLL_BAR_LEFT_BORDER - HORIZONTAL_SCROLL_BAR_RIGHT_BORDER)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH {
  ($f:expr, $width:expr) => {
    {
      ($width) - HORIZONTAL_SCROLL_BAR_LEFT_BORDER! ()
        - HORIZONTAL_SCROLL_BAR_RIGHT_BORDER! ()
    }
  };
}

/* C 618-630: the border widths and the two handle minima.  C 620/621's top/bottom borders and
   C 628-629's minima are the global variables `w32term.c` defines (`vertical_scroll_bar_top_border`,
   …, `horizontal_scroll_bar_min_handle`); the port names them as C does (a `macro_rules!` body is
   not resolved until it is expanded).  */
/// C: #define VERTICAL_SCROLL_BAR_LEFT_BORDER (0)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_LEFT_BORDER { () => { (0) }; }
/// C: #define VERTICAL_SCROLL_BAR_RIGHT_BORDER (0)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_RIGHT_BORDER { () => { (0) }; }
/// C: #define VERTICAL_SCROLL_BAR_TOP_BORDER (vertical_scroll_bar_top_border)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_TOP_BORDER { () => { (vertical_scroll_bar_top_border) }; }
/// C: #define VERTICAL_SCROLL_BAR_BOTTOM_BORDER (vertical_scroll_bar_bottom_border)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_BOTTOM_BORDER { () => { (vertical_scroll_bar_bottom_border) }; }
/// C: #define HORIZONTAL_SCROLL_BAR_LEFT_BORDER (horizontal_scroll_bar_left_border)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_LEFT_BORDER { () => { (horizontal_scroll_bar_left_border) }; }
/// C: #define HORIZONTAL_SCROLL_BAR_RIGHT_BORDER (horizontal_scroll_bar_right_border)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_RIGHT_BORDER { () => { (horizontal_scroll_bar_right_border) }; }
/// C: #define HORIZONTAL_SCROLL_BAR_TOP_BORDER (0)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_TOP_BORDER { () => { (0) }; }
/// C: #define HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER (0)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER { () => { (0) }; }
/// C: #define VERTICAL_SCROLL_BAR_MIN_HANDLE (vertical_scroll_bar_min_handle)
#[cfg (feature = "windowsnt")]
macro_rules! VERTICAL_SCROLL_BAR_MIN_HANDLE { () => { (vertical_scroll_bar_min_handle) }; }
/// C: #define HORIZONTAL_SCROLL_BAR_MIN_HANDLE (horizontal_scroll_bar_min_handle)
#[cfg (feature = "windowsnt")]
macro_rules! HORIZONTAL_SCROLL_BAR_MIN_HANDLE { () => { (horizontal_scroll_bar_min_handle) }; }

// C 634: `struct frame;` - a forward declaration of `frame.h`'s record (included at C 22).  The
// port names `frame` directly; the repeat in C is a reminder, not a second declaration.

/// C 636-637: `extern void w32_fill_rect (struct frame *, HDC, COLORREF, RECT *); extern void
/// w32_clear_window (struct frame *);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_fill_rect (f: *mut frame, hdc: HDC, pix: COLORREF, rect: *mut RECT);
  pub fn w32_clear_window (f: *mut frame);
}

/* C 639-661: the four fill/clear macros.  C's bodies are `do { … } while (0)` statements with a
   local `RECT rect`; the port keeps the same statements as a block (Rust has no `do … while`,
   and the value C's caller reads is none).  */
/// C: #define w32_fill_area(f,hdc,pix,x,y,nx,ny) do { RECT rect; rect.left = x; rect.top = y;
/// C: rect.right = x + nx; rect.bottom = y + ny; w32_fill_rect (f,hdc,pix,&rect); } while (0)
#[cfg (feature = "windowsnt")]
macro_rules! w32_fill_area {
  ($f:expr, $hdc:expr, $pix:expr, $x:expr, $y:expr, $nx:expr, $ny:expr) => {
    {
      let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
      rect.left = $x;
      rect.top = $y;
      rect.right = $x + $nx;
      rect.bottom = $y + $ny;
      w32_fill_rect ($f, $hdc, $pix, &raw mut rect);
    }
  };
}
/// C: #define w32_fill_area_abs(f,hdc,pix,x0,y0,x1,y1) do { RECT rect; rect.left = x0;
/// C: rect.top = y0; rect.right = x1; rect.bottom = y1; w32_fill_rect (f,hdc,pix,&rect); }
/// C: while (0)
#[cfg (feature = "windowsnt")]
macro_rules! w32_fill_area_abs {
  ($f:expr, $hdc:expr, $pix:expr, $x0:expr, $y0:expr, $x1:expr, $y1:expr) => {
    {
      let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
      rect.left = $x0;
      rect.top = $y0;
      rect.right = $x1;
      rect.bottom = $y1;
      w32_fill_rect ($f, $hdc, $pix, &raw mut rect);
    }
  };
}
/// C: #define w32_clear_rect(f,hdc,lprect) w32_fill_rect (f, hdc, FRAME_BACKGROUND_PIXEL (f),
/// C: lprect)
#[cfg (feature = "windowsnt")]
macro_rules! w32_clear_rect {
  ($f:expr, $hdc:expr, $lprect:expr) => {
    { w32_fill_rect ($f, $hdc, FRAME_BACKGROUND_PIXEL ($f), $lprect) }
  };
}
/// C: #define w32_clear_area(f,hdc,px,py,nx,ny) w32_fill_area (f, hdc,
/// C: FRAME_BACKGROUND_PIXEL (f), px, py, nx, ny)
#[cfg (feature = "windowsnt")]
macro_rules! w32_clear_area {
  ($f:expr, $hdc:expr, $px:expr, $py:expr, $nx:expr, $ny:expr) => {
    { w32_fill_area! ($f, $hdc, FRAME_BACKGROUND_PIXEL ($f), $px, $py, $nx, $ny) }
  };
}

/* C 664-686: the nine "Define for earlier versions of Visual C" / "Older compiler environments
   don't have these defined" guards.  Each is an `#ifndef` on a `<windows.h>` name, so the port
   carries C's value unconditionally with the guard recorded (the `include/widget.rs` `XtNfont`
   reading).  `MSH_MOUSEWHEEL` is a *string* (`"MSWHEEL_ROLLMSG"`), the widget.rs `&CStr` shape.  */
/// C: #ifndef WM_MOUSEWHEEL / #define WM_MOUSEWHEEL (WM_MOUSELAST + 1) / #endif
/// C: #ifndef WM_XBUTTONDOWN / #define WM_XBUTTONDOWN (WM_MOUSEWHEEL + 1) / #define
/// C: WM_XBUTTONUP (WM_MOUSEWHEEL + 2) / #endif
/// C: #ifndef WM_MOUSEHWHEEL / #define WM_MOUSEHWHEEL (WM_MOUSEWHEEL + 4) / #endif
#[cfg (feature = "windowsnt")]
pub const WM_MOUSEWHEEL: c_int = WM_MOUSELAST + 1;
#[cfg (feature = "windowsnt")]
pub const WM_XBUTTONDOWN: c_int = WM_MOUSEWHEEL + 1;
#[cfg (feature = "windowsnt")]
pub const WM_XBUTTONUP: c_int = WM_MOUSEWHEEL + 2;
#[cfg (feature = "windowsnt")]
pub const WM_MOUSEHWHEEL: c_int = WM_MOUSEWHEEL + 4;
/// C: #ifndef MSH_MOUSEWHEEL / #define MSH_MOUSEWHEEL "MSWHEEL_ROLLMSG" / #endif
#[cfg (feature = "windowsnt")]
pub const MSH_MOUSEWHEEL: &core::ffi::CStr = c"MSWHEEL_ROLLMSG";
/// C: #ifndef WM_APPCOMMAND / #define WM_APPCOMMAND 0x319 / #define
/// C: GET_APPCOMMAND_LPARAM(lParam) (HIWORD(lParam) & 0x7fff) / #endif
#[cfg (feature = "windowsnt")]
pub const WM_APPCOMMAND: c_int = 0x319;
#[cfg (feature = "windowsnt")]
macro_rules! GET_APPCOMMAND_LPARAM {
  ($lParam:expr) => { (HIWORD ($lParam) & 0x7fff) };
}
/// C: #ifndef WM_UNICHAR / #define WM_UNICHAR 0x109 / #endif
#[cfg (feature = "windowsnt")]
pub const WM_UNICHAR: c_int = 0x109;
/// C: #ifndef UNICODE_NOCHAR / #define UNICODE_NOCHAR 0xFFFF / #endif
#[cfg (feature = "windowsnt")]
pub const UNICODE_NOCHAR: c_int = 0xFFFF;

/* C 688-719: the Emacs-private window messages, all offsets from `WM_EMACS_START`.  */
/// C: #define WM_EMACS_START (WM_USER + 1) … #define WM_EMACS_SET_TOOLKIT_THEME (WM_EMACS_START + 30)
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_START: c_int = WM_USER + 1;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_KILL: c_int = WM_EMACS_START + 0;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_CREATEWINDOW: c_int = WM_EMACS_START + 1;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_DONE: c_int = WM_EMACS_START + 2;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_CREATEVSCROLLBAR: c_int = WM_EMACS_START + 3;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_CREATEHSCROLLBAR: c_int = WM_EMACS_START + 4;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SHOWWINDOW: c_int = WM_EMACS_START + 5;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETWINDOWPOS: c_int = WM_EMACS_START + 6;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_DESTROYWINDOW: c_int = WM_EMACS_START + 7;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_TRACKPOPUPMENU: c_int = WM_EMACS_START + 8;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETFOCUS: c_int = WM_EMACS_START + 9;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETFOREGROUND: c_int = WM_EMACS_START + 10;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETLOCALE: c_int = WM_EMACS_START + 11;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETKEYBOARDLAYOUT: c_int = WM_EMACS_START + 12;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_REGISTER_HOT_KEY: c_int = WM_EMACS_START + 13;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_UNREGISTER_HOT_KEY: c_int = WM_EMACS_START + 14;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_TOGGLE_LOCK_KEY: c_int = WM_EMACS_START + 15;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_TRACK_CARET: c_int = WM_EMACS_START + 16;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_DESTROY_CARET: c_int = WM_EMACS_START + 17;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SHOW_CARET: c_int = WM_EMACS_START + 18;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_HIDE_CARET: c_int = WM_EMACS_START + 19;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SETCURSOR: c_int = WM_EMACS_START + 20;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SHOWCURSOR: c_int = WM_EMACS_START + 21;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_PAINT: c_int = WM_EMACS_START + 22;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_BRINGTOTOP: c_int = WM_EMACS_START + 23;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_INPUT_READY: c_int = WM_EMACS_START + 24;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_FILENOTIFY: c_int = WM_EMACS_START + 25;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_IME_STATUS: c_int = WM_EMACS_START + 26;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_DRAGOVER: c_int = WM_EMACS_START + 27;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_DROP: c_int = WM_EMACS_START + 28;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_END: c_int = WM_EMACS_START + 29;
#[cfg (feature = "windowsnt")]
pub const WM_EMACS_SET_TOOLKIT_THEME: c_int = WM_EMACS_START + 30;

/* C 721-729: `WND_*` - the offsets inside the window-extra area `w32_createwindow` reserves.  */
/// C: #define WND_FONTWIDTH_INDEX (0) … #define WND_LAST_INDEX (24) / #define WND_EXTRA_BYTES
/// C: (WND_LAST_INDEX)
#[cfg (feature = "windowsnt")]
pub const WND_FONTWIDTH_INDEX: c_int = 0;
#[cfg (feature = "windowsnt")]
pub const WND_LINEHEIGHT_INDEX: c_int = 4;
#[cfg (feature = "windowsnt")]
pub const WND_BORDER_INDEX: c_int = 8;
#[cfg (feature = "windowsnt")]
pub const WND_VSCROLLBAR_INDEX: c_int = 12;
#[cfg (feature = "windowsnt")]
pub const WND_HSCROLLBAR_INDEX: c_int = 16;
#[cfg (feature = "windowsnt")]
pub const WND_BACKGROUND_INDEX: c_int = 20;
#[cfg (feature = "windowsnt")]
pub const WND_LAST_INDEX: c_int = 24;
#[cfg (feature = "windowsnt")]
pub const WND_EXTRA_BYTES: c_int = WND_LAST_INDEX;

/// C 731-734: the four thread objects, `DWORD`/`HANDLE` typed as C writes them.
/// C: extern DWORD dwWindowsThreadId; extern HANDLE hWindowsThread; extern DWORD
/// C: dwMainThreadId; extern HANDLE hMainThread;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut dwWindowsThreadId: DWORD;
  pub static mut hWindowsThread: HANDLE;
  pub static mut dwMainThreadId: DWORD;
  pub static mut hMainThread: HANDLE;
}

/* C 736-741: the message record `W32Msg` and its queue entry point.  */
/// C: typedef struct W32Msg { MSG msg; DWORD dwModifiers; RECT rect; } W32Msg;
/// C: extern BOOL prepend_msg (W32Msg *lpmsg);
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct W32Msg
{
  pub msg: MSG,
  pub dwModifiers: DWORD,
  pub rect: RECT,
}
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn prepend_msg (lpmsg: *mut W32Msg) -> BOOL;
}

/* C 747-753: "Structure for recording message when input thread must return a result that
   depends on lisp thread to compute."  */
/// C: typedef struct deferred_msg { struct deferred_msg * next; W32Msg w32msg; LRESULT result;
/// C: int completed; } deferred_msg;
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct deferred_msg
{
  pub next: *mut deferred_msg,
  pub w32msg: W32Msg,
  pub result: LRESULT,
  pub completed: c_int,
}

/// C 755-765: the critical section and its two wrappers, plus `signal_quit` and the
/// `enter_crit`/`leave_crit` pair.
/// C: extern CRITICAL_SECTION critsect;
/// C: extern void init_crit (void); extern void delete_crit (void); extern void signal_quit (void);
/// C: #define enter_crit() EnterCriticalSection (&critsect)
/// C: #define leave_crit() LeaveCriticalSection (&critsect)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut critsect: CRITICAL_SECTION;
  pub fn init_crit ();
  pub fn delete_crit ();
  pub fn signal_quit ();
}
#[cfg (feature = "windowsnt")]
macro_rules! enter_crit {
  () => { EnterCriticalSection (&raw mut critsect) };
}
#[cfg (feature = "windowsnt")]
macro_rules! leave_crit {
  () => { LeaveCriticalSection (&raw mut critsect) };
}

/* C 767-781: the palette, DC and message-loop entry points.  */
/// C 767-768: `extern void select_palette (struct frame * f, HDC hdc); extern void
/// deselect_palette (struct frame * f, HDC hdc);`
/// C 769-770: `extern HDC get_frame_dc (struct frame * f); extern int release_frame_dc (struct
/// frame * f, HDC hDC);`
/// C 772: `extern int drain_message_queue (void);`
/// C 774-776: `extern BOOL get_next_msg (W32Msg *, BOOL); extern BOOL post_msg (W32Msg *); extern
/// void complete_deferred_msg (HWND hwnd, UINT msg, LRESULT result);`
/// C 778: `extern BOOL parse_button (int, int, int *, int *);`
/// C 780-781: `extern void w32_sys_ring_bell (struct frame *f); extern void w32_query_color
/// (struct frame *, Emacs_Color *);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn select_palette (f: *mut frame, hdc: HDC);
  pub fn deselect_palette (f: *mut frame, hdc: HDC);
  pub fn get_frame_dc (f: *mut frame) -> HDC;
  pub fn release_frame_dc (f: *mut frame, hDC: HDC) -> c_int;
  pub fn drain_message_queue () -> c_int;
  pub fn get_next_msg (msg: *mut W32Msg, blocking: BOOL) -> BOOL;
  pub fn post_msg (msg: *mut W32Msg) -> BOOL;
  pub fn complete_deferred_msg (hwnd: HWND, msg: UINT, result: LRESULT);
  pub fn parse_button (a: c_int, b: c_int, c: *mut c_int, d: *mut c_int) -> BOOL;
  pub fn w32_sys_ring_bell (f: *mut frame);
  pub fn w32_query_color (f: *mut frame, color: *mut Emacs_Color);
}

/// C 783: `extern void w32_delete_display (struct w32_display_info *dpyinfo);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_delete_display (dpyinfo: *mut w32_display_info);
}

/// C 782: `#define FILE_NOTIFICATIONS_SIZE 16384`
#[cfg (feature = "windowsnt")]
pub const FILE_NOTIFICATIONS_SIZE: c_int = 16384;

/* C 785-791: "Notifications come in sets.  We use a doubly linked list with a sentinel to
   communicate those sets from the watching threads to the main thread."  */
/// C: struct notifications_set { LPBYTE notifications; DWORD size; void *desc; struct
/// C: notifications_set *next; struct notifications_set *prev; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct notifications_set
{
  pub notifications: LPBYTE,
  pub size: DWORD,
  pub desc: *mut c_void,
  pub next: *mut notifications_set,
  pub prev: *mut notifications_set,
}

/// C 793-800: the list head and the three notification helpers.
/// C: extern struct notifications_set *notifications_set_head;
/// C: extern Lisp_Object w32_get_watch_object (void *);
/// C: extern Lisp_Object w32_lispy_file_action (DWORD);
/// C: extern int handle_file_notifications (struct input_event *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut notifications_set_head: *mut notifications_set;
  pub fn w32_get_watch_object (obj: *mut c_void) -> Lisp_Object;
  pub fn w32_lispy_file_action (action: DWORD) -> Lisp_Object;
  /// `struct input_event` is termhooks.h's; this mirror includes nothing of it, so the parameter
  /// keeps P-55's untyped pointer.
  pub fn handle_file_notifications (event: *mut c_void) -> c_int;
}

/// C 802-805: `extern void w32_initialize_display_info (Lisp_Object); extern void
/// initialize_w32_display (struct terminal *, int *, int *); extern bool w32_image_rotations_p
/// (void);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_initialize_display_info (display_name: Lisp_Object);
  pub fn initialize_w32_display (terminal: *mut terminal, width: *mut c_int, height: *mut c_int);
  pub fn w32_image_rotations_p () -> bool;
}

/* C 803-812: `#ifdef WINDOWSNT`'s keyboard-hook arm ("Keyboard hooks").  The file itself is the
   Windows side, so this arm is carried unconditionally inside the umbrella and the `#else` arm
   (`#define w32_kbdhook_active 0`) is recorded - C's own two values are kept where C keeps them
   (`w32_kbdhook_active` is the one name whose definition the two arms disagree on).  */
// C: #ifdef WINDOWSNT
/// C: extern void setup_w32_kbdhook (HWND); extern void remove_w32_kbdhook (void); extern void
/// C: reset_w32_kbdhook_state (void); extern int check_w32_winkey_state (int);
/// C: #define w32_kbdhook_active (os_subtype != OS_SUBTYPE_9X)
#[cfg (feature = "windowsnt")]
pub const w32_kbdhook_active: c_int = (os_subtype != OS_SUBTYPE_9X) as c_int;
/// C 811-812: `#else / #define w32_kbdhook_active 0 / #endif` - recorded (this file is the
/// Windows side; the non-WindowsNT value is what C gives a compile that omits the arm above).
// C: #define w32_kbdhook_active 0
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn setup_w32_kbdhook (hwnd: HWND);
  pub fn remove_w32_kbdhook ();
  pub fn reset_w32_kbdhook_state ();
  pub fn check_w32_winkey_state (key: c_int) -> c_int;
}

/* C 814-829: "Keypad command key support.  W32 doesn't have virtual keys defined for the
   function keys on the keypad (they are mapped to the standard function keys), so we define our
   own."  */
/// C: #define VK_NUMPAD_BEGIN 0x92 … #define VK_NUMPAD_DELETE (VK_NUMPAD_BEGIN + 11)
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_BEGIN: c_int = 0x92;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_CLEAR: c_int = VK_NUMPAD_BEGIN + 0;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_ENTER: c_int = VK_NUMPAD_BEGIN + 1;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_PRIOR: c_int = VK_NUMPAD_BEGIN + 2;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_NEXT: c_int = VK_NUMPAD_BEGIN + 3;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_END: c_int = VK_NUMPAD_BEGIN + 4;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_HOME: c_int = VK_NUMPAD_BEGIN + 5;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_LEFT: c_int = VK_NUMPAD_BEGIN + 6;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_UP: c_int = VK_NUMPAD_BEGIN + 7;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_RIGHT: c_int = VK_NUMPAD_BEGIN + 8;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_DOWN: c_int = VK_NUMPAD_BEGIN + 9;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_INSERT: c_int = VK_NUMPAD_BEGIN + 10;
#[cfg (feature = "windowsnt")]
pub const VK_NUMPAD_DELETE: c_int = VK_NUMPAD_BEGIN + 11;

/// C 831-836: `#ifndef VK_LWIN / #define VK_LWIN 0x5B / #define VK_RWIN 0x5C / #define VK_APPS
/// C: 0x5D / #endif` - "Older compiler environments don't have these defined."
#[cfg (feature = "windowsnt")]
pub const VK_LWIN: c_int = 0x5B;
#[cfg (feature = "windowsnt")]
pub const VK_RWIN: c_int = 0x5C;
#[cfg (feature = "windowsnt")]
pub const VK_APPS: c_int = 0x5D;

/// C 838-843: `#define LEFT_WIN_PRESSED 0x8000 / #define RIGHT_WIN_PRESSED 0x4000 / #define
/// C: APPS_PRESSED 0x2000` - "Support for treating Windows and Apps keys as modifiers."
#[cfg (feature = "windowsnt")]
pub const LEFT_WIN_PRESSED: c_int = 0x8000;
#[cfg (feature = "windowsnt")]
pub const RIGHT_WIN_PRESSED: c_int = 0x4000;
#[cfg (feature = "windowsnt")]
pub const APPS_PRESSED: c_int = 0x2000;

/// C 845-846: "The current ANSI input codepage for GUI sessions."
/// C: extern int w32_keyboard_codepage;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut w32_keyboard_codepage: c_int;
}

/* C 848-861: the Windows 9x/ME and NT 3.x fallbacks: `WM_MOUSELEAVE` (0x02A3), `TME_LEAVE`
   (0x00000002) and the `TRACKMOUSEEVENT` record C defines when the SDK lacks it.  C's `TME_LEAVE`
   body ends in a semicolon; a Rust constant takes it as C's value (`0x00000002`).  */
/// C: #ifndef WM_MOUSELEAVE / #define WM_MOUSELEAVE 0x02A3 / #define TME_LEAVE 0x00000002;
/// C: typedef struct tagTRACKMOUSEEVENT { DWORD cbSize; DWORD dwFlags; HWND hwndTrack; DWORD
/// C: dwHoverTime; } TRACKMOUSEEVENT; / #endif
#[cfg (feature = "windowsnt")]
pub const WM_MOUSELEAVE: c_int = 0x02A3;
#[cfg (feature = "windowsnt")]
pub const TME_LEAVE: c_int = 0x00000002;
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct tagTRACKMOUSEEVENT
{
  pub cbSize: DWORD,
  pub dwFlags: DWORD,
  pub hwndTrack: HWND,
  pub dwHoverTime: DWORD,
}
#[cfg (feature = "windowsnt")]
pub type TRACKMOUSEEVENT = tagTRACKMOUSEEVENT;

// C 863-864: `struct image; struct face;` - the two forward declarations (image.h's and
// dispextern.h's records).  The port names neither pointer's pointee; every parameter that takes
// one is `*mut c_void` below.

/// C 866: `Emacs_GC *XCreateGC (void *, HWND, unsigned long, Emacs_GC *);` - the X-compatibility
/// entry point `w32fns.c` defines.
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn XCreateGC (dpy: *mut c_void, hwnd: HWND, valuemask: c_ulong,
                    values: *mut Emacs_GC) -> *mut Emacs_GC;
}

/* C 868-873: the two lazily loaded GDI entry points.  C's `WINAPI` is the `extern "system"` ABI
   in Rust and a C function pointer is nullable, so each typedef is an `Option<…>` (the
   `include/w32.rs` 317-329 reading).  */
/// C: typedef DWORD (WINAPI * ClipboardSequence_Proc) (void);
/// C: typedef BOOL (WINAPI * AppendMenuW_Proc) (IN HMENU, IN UINT, IN UINT_PTR, IN LPCWSTR);
#[cfg (feature = "windowsnt")]
pub type ClipboardSequence_Proc = Option<unsafe extern "system" fn () -> DWORD>;
#[cfg (feature = "windowsnt")]
pub type AppendMenuW_Proc
  = Option<unsafe extern "system" fn (HMENU, UINT, UINT_PTR, LPCWSTR) -> BOOL>;

/* C 875-885: the system caret and the tooltip window.  `struct window` and `struct font` are
   window.h's and font.h's; `struct font` is named because `include/font.rs` owns the record and
   `struct window` is not (`*mut c_void`, P-55).  */
/// C: extern struct font *w32_system_remap_font;
/// C: extern HWND w32_system_caret_hwnd; extern int w32_system_caret_height;
/// C: extern int w32_system_caret_x; extern int w32_system_caret_y;
/// C: extern struct window *w32_system_caret_window;
/// C: extern int w32_system_caret_hdr_height; extern int w32_system_caret_mode_height;
/// C: extern Window tip_window;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut w32_system_remap_font: *mut font;
  pub static mut w32_system_caret_hwnd: HWND;
  pub static mut w32_system_caret_height: c_int;
  pub static mut w32_system_caret_x: c_int;
  pub static mut w32_system_caret_y: c_int;
  pub static mut w32_system_caret_window: *mut c_void;
  pub static mut w32_system_caret_hdr_height: c_int;
  pub static mut w32_system_caret_mode_height: c_int;
  pub static mut tip_window: Window;
}

// C 887-890: `#if EMACSDEBUG / extern const char* w32_name_of_message (UINT msg); / #endif` -
// `EMACSDEBUG` is `undef` in the frozen table, so the prototype is recorded.
// C: #if EMACSDEBUG / extern const char* w32_name_of_message (UINT msg); / #endif

/* C 892-905: the `NTGUI_UNICODE` pair.  `conf_post.h` 169-170 defines `NTGUI_UNICODE` only for
   `defined CYGWIN && defined HAVE_NTGUI` ("Cygwin runs only on UNICODE-supporting systems"), so
   that arm carries `cygwin` and the `#else` arm - the MinGW/Windows GUI one - is live.  */
/// C: #ifdef NTGUI_UNICODE / extern Lisp_Object ntgui_encode_system (Lisp_Object str); #define
/// C: GUISTR(x) (L ## x) / #define GUI_ENCODE_FILE GUI_ENCODE_SYSTEM / #define
/// C: GUI_ENCODE_SYSTEM(x) ntgui_encode_system (x) / #define GUI_FN(fn) fn ## W / typedef
/// C: wchar_t guichar_t;
#[cfg (feature = "windowsnt")]
#[cfg (feature = "cygwin")]
unsafe extern "C" {
  pub fn ntgui_encode_system (str_: Lisp_Object) -> Lisp_Object;
}
// C 895-898: the Cygwin arm's four macros are recorded rather than written: C's bodies use the
// token-pasting operator (`GUISTR(x)` is `(L ## x)`, `GUI_FN(fn)` is `fn ## W`), and Rust's `##`
// is a reserved token that cannot appear in a `macro_rules!` body at all (measured:
// `error: reserved multi-hash token is forbidden`, caught by `layout-check`).  The names are
// already declared by the `#else` arm below, and `tools/extract-abi.py` treats a name C defines
// twice as a configuration-branch duplicate, so only its presence is required.
// C: #define GUISTR(x) (L ## x)
// C: #define GUI_ENCODE_FILE GUI_ENCODE_SYSTEM
// C: #define GUI_ENCODE_SYSTEM(x) ntgui_encode_system (x)
// C: #define GUI_FN(fn) fn ## W
#[cfg (feature = "windowsnt")]
#[cfg (feature = "cygwin")]
pub type guichar_t = wchar_t;

/// C 899-905: the `#else /* !NTGUI_UNICODE */` arm - `#define GUISTR(x) x / #define
/// GUI_ENCODE_FILE ENCODE_FILE / #define GUI_ENCODE_SYSTEM ENCODE_SYSTEM / #define GUI_FN(fn) fn /
/// typedef char guichar_t;`.
/// C: #define GUISTR(x) x
/// C: #define GUI_ENCODE_FILE ENCODE_FILE
/// C: #define GUI_ENCODE_SYSTEM ENCODE_SYSTEM
/// C: #define GUI_FN(fn) fn
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "cygwin"))]
macro_rules! GUISTR { ($x:expr) => { $x }; }
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "cygwin"))]
macro_rules! GUI_ENCODE_FILE { () => { ENCODE_FILE }; }
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "cygwin"))]
macro_rules! GUI_ENCODE_SYSTEM { () => { ENCODE_SYSTEM }; }
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "cygwin"))]
macro_rules! GUI_FN { ($fn:expr) => { $fn }; }
#[cfg (feature = "windowsnt")]
#[cfg (not (feature = "cygwin"))]
pub type guichar_t = c_char;

/// C 907: `#define GUI_SDATA(x) ((guichar_t*) SDATA (x))`
#[cfg (feature = "windowsnt")]
macro_rules! GUI_SDATA {
  ($x:expr) => { (SDATA ($x) as *mut guichar_t) };
}

/* C 909-925: the last entry points.  */
/// C 909: `extern Lisp_Object w32_popup_dialog (struct frame *, Lisp_Object, Lisp_Object);`
/// C 910: `extern void w32_arrow_cursor (void);`
/// C 911: `extern void w32_release_paint_buffer (struct frame *);`
/// C 912: `extern void w32_flip_buffers_if_dirty (struct frame *);`
/// C 914-916: `extern void syms_of_w32term (void); extern void syms_of_w32menu (void); extern
/// void syms_of_w32fns (void);`
/// C 918-920: `extern void globals_of_w32menu (void); extern void globals_of_w32fns (void);
/// extern void globals_of_w32notify (void);`
/// C 922-923: `extern void syms_of_w32image (void); extern void globals_of_w32image (void);`
/// C 925: `extern void w32_init_main_thread (void);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_popup_dialog (f: *mut frame, header: Lisp_Object, contents: Lisp_Object)
                           -> Lisp_Object;
  pub fn w32_arrow_cursor ();
  pub fn w32_release_paint_buffer (f: *mut frame);
  pub fn w32_flip_buffers_if_dirty (f: *mut frame);
  pub fn syms_of_w32term ();
  pub fn syms_of_w32menu ();
  pub fn syms_of_w32fns ();
  pub fn globals_of_w32menu ();
  pub fn globals_of_w32fns ();
  pub fn globals_of_w32notify ();
  pub fn syms_of_w32image ();
  pub fn globals_of_w32image ();
  pub fn w32_init_main_thread ();
}

/// C 927-929: `#ifdef CYGWIN / extern int w32_message_fd; / #endif /* CYGWIN */` - `CYGWIN` is
/// `undef` in the frozen table, so the object carries that feature (the `cygwin` name this tree
/// already declares).
#[cfg (feature = "windowsnt")]
#[cfg (feature = "cygwin")]
unsafe extern "C" {
  pub static mut w32_message_fd: c_int;
}
