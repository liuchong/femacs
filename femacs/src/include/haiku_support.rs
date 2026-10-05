// femacs: port of src/haiku_support.h @ fd7afced403.

// Header mirror (PORTING.md 0.1): declarations only.  This is the Haiku backend's interface
// header - `haiku_support.cc`/`haikufns.cc` implement it and `haikuterm.c`/`haikufont.c` call
// it - and it is C++ text in C's eyes (C++ says so itself, in the header's first line).  The
// frozen target (darwin-arm64) does not build it, but the header's `#if`s are all decidable
// against the frozen macro table (`tools/check-cond-eval.py src/haiku_support.h` agrees with
// clang on every branch), so the live text below is complete and the dead arms are recorded in
// docs/ports/include-haiku_support.md (PORTING.md P-08): the `#ifdef __cplusplus` regions (the
// `haiku` typedef, `haiku_put_pixel`/`haiku_get_pixel`, `gui_abort`, `find_appropriate_view_for_draw`
// and the two `BE_RECT_*` macros), `HAVE_FREETYPE`'s two includes, `HAVE_NATIVE_IMAGE_API`'s
// three prototypes and `USE_BE_CAIRO`'s four.
//
// `__cplusplus` is undefined for the frozen configuration, which is why the C++-only region is
// recorded rather than transcribed: the one name it would add that the live text does not have
// (`haiku`) is already declared unconditionally by `haikugui.h`, and two `pub type haiku` in one
// crate is an `E0428` (`haikugui.rs` is the other mirror).
//
// The prototypes C spells `extern` sit in `pub mod haiku_support_decls` with a glob import, the
// P-17/P-21 shape: `haiku_support.cc`'s port is the unit that *defines* them, and a same-named
// declaration at this scope would be `E0428` (PORTING.md 0.1-10).  `extract-abi.py` does not
// inventory prototypes (its `functions` list is definitions only - the ledger says "Functions
// defined - 0"), so this block is translated in full from the reference text and is the one part
// of this mirror no gate compares; the transcription was checked name-for-name against the C
// (178 declarations, `docs/ports/include-haiku_support.md`).

include!("../../lib/include/stdint.in.rs");     // C 22: `#include <stdint.h>` (int32_t, uint32_t)
include!("../../lib/include/unistd.in.rs");     // `ssize_t` (C's <kernel/OS.h> chain reaches <sys/types.h>)
include!("../../lib/include/time.in.rs");       // `struct timespec` (the same chain reaches <time.h>)

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _HAIKU_SUPPORT_H: bool = true;

/* Haiku's own scalars, from C 39's `#include <kernel/OS.h>` (which pulls
   <support/SupportDefs.h>).  That header has no mirror - it is a system header of the Haiku
   toolchain, like gnulib's `<gmp.h>` was for `bignum.rs` (PORTING.md P-49/P-70) - so the four
   names this header uses are declared here, with the widths Haiku's `SupportDefs.h` gives them
   (`int32`/`status_t`/`team_id`/`port_id` are 32-bit, `bigtime_t` is 64-bit).  */
pub type int32 = int32_t;
pub type status_t = int32;
pub type team_id = int32;
pub type port_id = int32;
pub type bigtime_t = i64;

/* C 27-28's `#include <kits/interface/View.h>`-class platform types this header names: none -
   every handle in the live text is `void *`.  */

// C 41-72: `enum haiku_cursor` (explicit values, not declaration order), declared as C's own
// type plus one constant per member - the shape PORTING.md 1.3 fixes for an enum whose members
// C compares as integers (`termhooks.rs`, `dispextern.rs`, `atimer.rs`).
pub type haiku_cursor = c_int;
pub const CURSOR_ID_SYSTEM_DEFAULT: haiku_cursor = 1;
pub const CURSOR_ID_CONTEXT_MENU: haiku_cursor = 3;
pub const CURSOR_ID_COPY: haiku_cursor = 4;
pub const CURSOR_ID_CREATE_LINK: haiku_cursor = 29;
pub const CURSOR_ID_CROSS_HAIR: haiku_cursor = 5;
pub const CURSOR_ID_FOLLOW_LINK: haiku_cursor = 6;
pub const CURSOR_ID_GRAB: haiku_cursor = 7;
pub const CURSOR_ID_GRABBING: haiku_cursor = 8;
pub const CURSOR_ID_HELP: haiku_cursor = 9;
pub const CURSOR_ID_I_BEAM: haiku_cursor = 2;
pub const CURSOR_ID_I_BEAM_HORIZONTAL: haiku_cursor = 10;
pub const CURSOR_ID_MOVE: haiku_cursor = 11;
pub const CURSOR_ID_NO_CURSOR: haiku_cursor = 12;
pub const CURSOR_ID_NOT_ALLOWED: haiku_cursor = 13;
pub const CURSOR_ID_PROGRESS: haiku_cursor = 14;
pub const CURSOR_ID_RESIZE_NORTH: haiku_cursor = 15;
pub const CURSOR_ID_RESIZE_EAST: haiku_cursor = 16;
pub const CURSOR_ID_RESIZE_SOUTH: haiku_cursor = 17;
pub const CURSOR_ID_RESIZE_WEST: haiku_cursor = 18;
pub const CURSOR_ID_RESIZE_NORTH_EAST: haiku_cursor = 19;
pub const CURSOR_ID_RESIZE_NORTH_WEST: haiku_cursor = 20;
pub const CURSOR_ID_RESIZE_SOUTH_EAST: haiku_cursor = 21;
pub const CURSOR_ID_RESIZE_SOUTH_WEST: haiku_cursor = 22;
pub const CURSOR_ID_RESIZE_NORTH_SOUTH: haiku_cursor = 23;
pub const CURSOR_ID_RESIZE_EAST_WEST: haiku_cursor = 24;
pub const CURSOR_ID_RESIZE_NORTH_EAST_SOUTH_WEST: haiku_cursor = 25;
pub const CURSOR_ID_RESIZE_NORTH_WEST_SOUTH_EAST: haiku_cursor = 26;
pub const CURSOR_ID_ZOOM_IN: haiku_cursor = 27;
pub const CURSOR_ID_ZOOM_OUT: haiku_cursor = 28;

// C 74-79.
pub type haiku_z_group = c_int;
pub const Z_GROUP_ABOVE: haiku_z_group = 0;
pub const Z_GROUP_NONE: haiku_z_group = 1;
pub const Z_GROUP_BELOW: haiku_z_group = 2;

// C 81-88.
pub type haiku_alert_type = c_int;
pub const HAIKU_EMPTY_ALERT: haiku_alert_type = 0;
pub const HAIKU_INFO_ALERT: haiku_alert_type = 1;
pub const HAIKU_IDEA_ALERT: haiku_alert_type = 2;
pub const HAIKU_WARNING_ALERT: haiku_alert_type = 3;
pub const HAIKU_STOP_ALERT: haiku_alert_type = 4;

// C 90-121: C's members carry no initialiser, so their values are the declaration order.
pub type haiku_event_type = c_int;
pub const QUIT_REQUESTED: haiku_event_type = 0;
pub const FRAME_RESIZED: haiku_event_type = 1;
pub const FRAME_EXPOSED: haiku_event_type = 2;
pub const KEY_DOWN: haiku_event_type = 3;
pub const KEY_UP: haiku_event_type = 4;
pub const ACTIVATION: haiku_event_type = 5;
pub const MOUSE_MOTION: haiku_event_type = 6;
pub const BUTTON_DOWN: haiku_event_type = 7;
pub const BUTTON_UP: haiku_event_type = 8;
pub const ICONIFICATION: haiku_event_type = 9;
pub const MOVE_EVENT: haiku_event_type = 10;
pub const SCROLL_BAR_VALUE_EVENT: haiku_event_type = 11;
pub const SCROLL_BAR_PART_EVENT: haiku_event_type = 12;
pub const SCROLL_BAR_DRAG_EVENT: haiku_event_type = 13;
pub const WHEEL_MOVE_EVENT: haiku_event_type = 14;
pub const MENU_BAR_RESIZE: haiku_event_type = 15;
pub const MENU_BAR_CLICK: haiku_event_type = 16;
pub const MENU_BAR_OPEN: haiku_event_type = 17;
pub const MENU_BAR_SELECT_EVENT: haiku_event_type = 18;
pub const MENU_BAR_CLOSE: haiku_event_type = 19;
pub const MENU_BAR_HELP_EVENT: haiku_event_type = 20;
pub const ZOOM_EVENT: haiku_event_type = 21;
pub const DRAG_AND_DROP_EVENT: haiku_event_type = 22;
pub const APP_QUIT_REQUESTED_EVENT: haiku_event_type = 23;
pub const DUMMY_EVENT: haiku_event_type = 24;
pub const SCREEN_CHANGED_EVENT: haiku_event_type = 25;
pub const MENU_BAR_LEFT: haiku_event_type = 26;
pub const CLIPBOARD_CHANGED_EVENT: haiku_event_type = 27;
pub const FONT_CHANGE_EVENT: haiku_event_type = 28;
pub const NOTIFICATION_CLICK_EVENT: haiku_event_type = 29;

/* The events themselves.  */
// C 123-201: one record per event kind, plus the string/screen/quit/app-quit/dummy records.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_clipboard_changed_event
{
  pub dummy: c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_screen_changed_event
{
  pub when: bigtime_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_quit_requested_event
{
  pub window: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_resize_event
{
  pub window: *mut c_void,
  pub width: f32,
  pub height: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_expose_event
{
  pub window: *mut c_void,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_drag_and_drop_event
{
  pub window: *mut c_void,
  pub x: c_int,
  pub y: c_int,
  pub message: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_app_quit_requested_event
{
  pub dummy: c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_dummy_event
{
  pub dummy: c_char,
}

// C 156-179: `enum haiku_modifier_specification`.
pub type haiku_modifier_specification = c_int;
pub const HAIKU_MODIFIER_ALT: haiku_modifier_specification = 1;
pub const HAIKU_MODIFIER_CTRL: haiku_modifier_specification = 1 << 1;
pub const HAIKU_MODIFIER_SHIFT: haiku_modifier_specification = 1 << 2;
pub const HAIKU_MODIFIER_SUPER: haiku_modifier_specification = 1 << 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_key_event
{
  pub window: *mut c_void,
  pub modifiers: c_int,
  pub keysym: c_uint,
  pub multibyte_char: uint32_t,

  /* Time the keypress occurred, in microseconds.  */
  pub time: bigtime_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_activation_event
{
  pub window: *mut c_void,
  pub activated_p: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_mouse_motion_event
{
  pub window: *mut c_void,
  pub just_exited_p: bool,
  pub x: c_int,
  pub y: c_int,
  pub time: bigtime_t,
  pub dnd_message: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_left_event
{
  pub window: *mut c_void,
  pub x: c_int,
  pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_click_event
{
  pub window: *mut c_void,
  pub x: c_int,
  pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_button_event
{
  pub window: *mut c_void,
  pub scroll_bar: *mut c_void,
  pub btn_no: c_int,
  pub modifiers: c_int,
  pub x: c_int,
  pub y: c_int,
  pub time: bigtime_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_iconification_event
{
  pub window: *mut c_void,
  pub iconified_p: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_move_event
{
  pub window: *mut c_void,
  pub x: c_int,
  pub y: c_int,
  pub decorator_width: c_int,
  pub decorator_height: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_wheel_move_event
{
  pub window: *mut c_void,
  pub modifiers: c_int,
  pub delta_x: f32,
  pub delta_y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_select_event
{
  pub window: *mut c_void,
  pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_help_event
{
  pub window: *mut c_void,
  pub mb_idx: c_int,
  pub data: *mut c_void,
  pub highlight_p: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_zoom_event
{
  pub window: *mut c_void,
  pub fullscreen_mode: c_int,
}

// C 204-218: `enum haiku_font_specification`.
pub type haiku_font_specification = c_int;
pub const FSPEC_FAMILY: haiku_font_specification = 1;
pub const FSPEC_STYLE: haiku_font_specification = 1 << 1;
pub const FSPEC_SLANT: haiku_font_specification = 1 << 2;
pub const FSPEC_WEIGHT: haiku_font_specification = 1 << 3;
pub const FSPEC_SPACING: haiku_font_specification = 1 << 4;
pub const FSPEC_WANTED: haiku_font_specification = 1 << 5;
pub const FSPEC_NEED_ONE_OF: haiku_font_specification = 1 << 6;
pub const FSPEC_WIDTH: haiku_font_specification = 1 << 7;
pub const FSPEC_LANGUAGE: haiku_font_specification = 1 << 8;
pub const FSPEC_INDICES: haiku_font_specification = 1 << 9;
pub const FSPEC_ANTIALIAS: haiku_font_specification = 1 << 10;

// C 220: `typedef char haiku_font_family_or_style[64];` - an array typedef, so the type is the
// array (`sizeof` is 64) and the pointer uses in the prototypes are `*mut haiku_font_family_or_style`.
pub type haiku_font_family_or_style = [c_char; 64];

// C 222-228.
pub type haiku_font_slant = c_int;
pub const NO_SLANT: haiku_font_slant = -1;
pub const SLANT_OBLIQUE: haiku_font_slant = 0;
pub const SLANT_REGULAR: haiku_font_slant = 1;
pub const SLANT_ITALIC: haiku_font_slant = 2;

// C 230-242.
pub type haiku_font_width = c_int;
pub const NO_WIDTH: haiku_font_width = -1;
pub const ULTRA_CONDENSED: haiku_font_width = 0;
pub const EXTRA_CONDENSED: haiku_font_width = 1;
pub const CONDENSED: haiku_font_width = 2;
pub const SEMI_CONDENSED: haiku_font_width = 3;
pub const NORMAL_WIDTH: haiku_font_width = 4;
pub const SEMI_EXPANDED: haiku_font_width = 5;
pub const EXPANDED: haiku_font_width = 6;
pub const EXTRA_EXPANDED: haiku_font_width = 7;
pub const ULTRA_EXPANDED: haiku_font_width = 8;

// C 244-250.
pub type haiku_font_language = c_int;
pub const LANGUAGE_CN: haiku_font_language = 0;
pub const LANGUAGE_KO: haiku_font_language = 1;
pub const LANGUAGE_JP: haiku_font_language = 2;
pub const MAX_LANGUAGE: haiku_font_language = 3; /* This isn't a language.  */

// C 252-268.
pub type haiku_font_weight = c_int;
pub const NO_WEIGHT: haiku_font_weight = -1;
pub const HAIKU_THIN: haiku_font_weight = 0;
pub const HAIKU_EXTRALIGHT: haiku_font_weight = 40;
pub const HAIKU_LIGHT: haiku_font_weight = 50;
pub const HAIKU_SEMI_LIGHT: haiku_font_weight = 75;
pub const HAIKU_REGULAR: haiku_font_weight = 100;
pub const HAIKU_SEMI_BOLD: haiku_font_weight = 180;
pub const HAIKU_BOLD: haiku_font_weight = 200;
pub const HAIKU_EXTRA_BOLD: haiku_font_weight = 205;
pub const HAIKU_BOOK: haiku_font_weight = 400;
pub const HAIKU_HEAVY: haiku_font_weight = 800;
pub const HAIKU_ULTRA_HEAVY: haiku_font_weight = 900;
pub const HAIKU_BLACK: haiku_font_weight = 1000;
pub const HAIKU_MEDIUM: haiku_font_weight = 2000;

// C 270-277.
pub type haiku_fullscreen_mode = c_int;
pub const FULLSCREEN_MODE_NONE: haiku_fullscreen_mode = 0;
pub const FULLSCREEN_MODE_WIDTH: haiku_fullscreen_mode = 1;
pub const FULLSCREEN_MODE_HEIGHT: haiku_fullscreen_mode = 2;
pub const FULLSCREEN_MODE_BOTH: haiku_fullscreen_mode = 3;
pub const FULLSCREEN_MODE_MAXIMIZED: haiku_fullscreen_mode = 4;

/* A structure describing a font pattern.  */
// C 279-347.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_font_pattern
{
  /* Bitmask indicating which fields are set.  */
  pub specified: c_int,

  /* The next font in this list.  */
  pub next: *mut haiku_font_pattern,

  /* The last font in the list during font lookup.  */
  pub last: *mut haiku_font_pattern,

  /* The next font in the list whose family differs from this one.
     Only valid during font lookup.  */
  pub next_family: *mut haiku_font_pattern,

  /* The family of the font.  */
  pub family: haiku_font_family_or_style,

  /* The style of the font.  */
  pub style: haiku_font_family_or_style,

  /* Whether or the font is monospace.  */
  pub mono_spacing_p: c_int,

  /* The slant of the font.  */
  pub slant: haiku_font_slant,

  /* The width of the font.  */
  pub width: haiku_font_width,

  /* The language of the font.  Used during font lookup.  */
  pub language: haiku_font_language,

  /* The weight of the font.  */
  pub weight: haiku_font_weight,

  /* List of characters that must be present in the font for the match
     to succeed.  */
  pub wanted_chars: *mut c_int,

  /* The number of characters in `wanted_chars'.  */
  pub want_chars_len: c_int,

  /* List of characters.  The font must fulfill at least one of
     them for the match to succeed.  */
  pub need_one_of: *mut c_int,

  /* The number of characters in `need_one_of'.  */
  pub need_one_of_len: c_int,

  /* The index of the family of the font this pattern represents.  */
  pub family_index: c_int,

  /* The index of the style of the font this pattern represents.  */
  pub style_index: c_int,

  /* Temporary field used during font enumeration.  */
  pub oblique_seen_p: c_int,

  /* Whether or not to enable antialiasing in the font.  This field is
     special in that it's not handled by `BFont_open_pattern'.  */
  pub use_antialiasing: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_scroll_bar_value_event
{
  pub scroll_bar: *mut c_void,
  pub window: *mut c_void,
  pub position: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_scroll_bar_drag_event
{
  pub scroll_bar: *mut c_void,
  pub window: *mut c_void,
  pub dragging_p: c_int,
}

// C 437-442.
pub type haiku_scroll_bar_part = c_int;
pub const HAIKU_SCROLL_BAR_UP_BUTTON: haiku_scroll_bar_part = 0;
pub const HAIKU_SCROLL_BAR_DOWN_BUTTON: haiku_scroll_bar_part = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_scroll_bar_part_event
{
  pub scroll_bar: *mut c_void,
  pub window: *mut c_void,
  pub part: haiku_scroll_bar_part,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_resize_event
{
  pub window: *mut c_void,
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_menu_bar_state_event
{
  pub window: *mut c_void,
}

// C 465-471.
pub type haiku_what_font = c_int;
pub const FIXED_FAMILY: haiku_what_font = 0;
pub const FIXED_STYLE: haiku_what_font = 1;
pub const DEFAULT_FAMILY: haiku_what_font = 2;
pub const DEFAULT_STYLE: haiku_what_font = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_font_change_event
{
  /* New family, style and size of the font.  */
  pub new_family: haiku_font_family_or_style,
  pub new_style: haiku_font_family_or_style,
  pub new_size: c_int,

  /* What changed.  FIXED_FAMILY means this is the new fixed font.
     DEFAULT_FAMILY means this is the new plain font.  The other enums
     have no meaning.  */
  pub what: haiku_what_font,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_notification_click_event
{
  /* ID uniquely designating a single notification.  */
  // C's field is `intmax_t id;`.  PORTING.md 1.1 maps `intmax_t` to `i64`, and the port's
  // `intmax_t` alias lives in `lisp.rs`, which this header does not include (it includes
  // `<stdint.h>`, and that mirror deliberately leaves `intmax_t` to lisp.rs).
  pub id: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct haiku_session_manager_reply
{
  pub quit_reply: bool,
}

// C 468-500's `BE_RECT_HEIGHT`/`BE_RECT_WIDTH` sit inside `#ifdef __cplusplus` (dead on the
// frozen target) and are recorded in the ledger, together with the C++-only `haiku` typedef and
// the two pixel accessors (C 507-512) and the two tail declarations (C 773-778).

/* Function prototypes, and the three port objects.  */
pub mod haiku_support_decls {
  use super::*;

  unsafe extern "C" {
    pub static mut port_application_to_emacs: port_id;
    pub static mut port_popup_menu_to_emacs: port_id;
    pub static mut port_emacs_to_session_manager: port_id;
    pub fn haiku_io_init ();
    pub fn haiku_io_init_in_app_thread ();
    pub fn haiku_read_size (arg0: *mut ssize_t, arg1: bool);
    pub fn haiku_read (arg0: *mut haiku_event_type, arg1: *mut c_void, arg2: ssize_t) -> c_int;
    pub fn haiku_read_with_timeout (arg0: *mut haiku_event_type, arg1: *mut c_void, arg2: ssize_t, arg3: bigtime_t, arg4: bool) -> c_int;
    pub fn haiku_write (arg0: haiku_event_type, arg1: *mut c_void) -> c_int;
    pub fn haiku_write_without_signal (arg0: haiku_event_type, arg1: *mut c_void, arg2: bool) -> c_int;
    pub fn rgb_color_hsl (arg0: uint32_t, arg1: *mut f64, arg2: *mut f64, arg3: *mut f64);
    pub fn hsl_color_rgb (arg0: f64, arg1: f64, arg2: f64, arg3: *mut uint32_t);
    pub fn BBitmap_new (arg0: c_int, arg1: c_int, arg2: c_int) -> *mut c_void;
    pub fn BBitmap_data (arg0: *mut c_void) -> *mut c_void;
    pub fn BBitmap_convert (arg0: *mut c_void, arg1: *mut *mut c_void) -> c_int;
    pub fn be_draw_cross_on_pixmap (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: uint32_t);
    pub fn BBitmap_free (arg0: *mut c_void);
    pub fn BBitmap_dimensions (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut int32_t, arg6: *mut c_int);
    pub fn BApplication_setup ();
    pub fn BWindow_new (arg0: *mut *mut c_void) -> *mut c_void;
    pub fn BWindow_quit (arg0: *mut c_void);
    pub fn BWindow_set_offset (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn BWindow_iconify (arg0: *mut c_void);
    pub fn BWindow_set_visible (arg0: *mut c_void, arg1: c_int);
    pub fn BWindow_retitle (arg0: *mut c_void, arg1: *const c_char);
    pub fn BWindow_resize (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn BWindow_activate (arg0: *mut c_void);
    pub fn BWindow_center_on_screen (arg0: *mut c_void);
    pub fn BWindow_change_decoration (arg0: *mut c_void, arg1: c_int);
    pub fn BWindow_set_tooltip_decoration (arg0: *mut c_void);
    pub fn BWindow_set_avoid_focus (arg0: *mut c_void, arg1: c_int);
    pub fn BWindow_set_size_alignment (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn BWindow_sync (arg0: *mut c_void);
    pub fn BWindow_send_behind (arg0: *mut c_void, arg1: *mut c_void);
    pub fn BWindow_is_active (arg0: *mut c_void) -> bool;
    pub fn BWindow_set_override_redirect (arg0: *mut c_void, arg1: bool);
    pub fn BWindow_dimensions (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int);
    pub fn BWindow_set_z_group (arg0: *mut c_void, arg1: haiku_z_group);
    pub fn BWindow_set_sticky (arg0: *mut c_void, arg1: bool);
    pub fn BWindow_Flush (arg0: *mut c_void);
    pub fn BFont_close (arg0: *mut c_void);
    pub fn BFont_metrics (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_int, arg6: *mut c_int, arg7: *mut c_int, arg8: *mut c_int, arg9: *mut c_int, arg10: *mut c_int);
    pub fn BFont_have_char_p (arg0: *mut c_void, arg1: int32_t) -> c_int;
    pub fn BFont_have_char_block (arg0: *mut c_void, arg1: int32_t, arg2: int32_t) -> c_int;
    pub fn BFont_char_bounds (arg0: *mut c_void, arg1: *const c_char, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int);
    pub fn BFont_nchar_bounds (arg0: *mut c_void, arg1: *const c_char, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int, arg5: int32_t);
    pub fn BFont_find (arg0: *mut haiku_font_pattern) -> *mut haiku_font_pattern;
    pub fn BView_StartClip (arg0: *mut c_void);
    pub fn BView_EndClip (arg0: *mut c_void);
    pub fn BView_SetHighColor (arg0: *mut c_void, arg1: uint32_t);
    pub fn BView_SetLowColor (arg0: *mut c_void, arg1: uint32_t);
    pub fn BView_SetPenSize (arg0: *mut c_void, arg1: c_int);
    pub fn BView_SetFont (arg0: *mut c_void, arg1: *mut c_void);
    pub fn BView_MovePenTo (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn BView_DrawString (arg0: *mut c_void, arg1: *const c_char, arg2: ptrdiff_t);
    pub fn BView_DrawChar (arg0: *mut c_void, arg1: c_char);
    pub fn BView_FillRectangle (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_FillRectangleAbs (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_FillTriangle (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int);
    pub fn BView_StrokeRectangle (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_SetViewColor (arg0: *mut c_void, arg1: uint32_t);
    pub fn BView_ClipToRect (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_ClipToInverseRect (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_StrokeLine (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_CopyBits (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int);
    pub fn BView_InvertRect (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_DrawBitmap (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int, arg10: bool);
    pub fn BView_DrawBitmapWithEraseOp (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
    pub fn BView_DrawBitmapTiled (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int);
    pub fn BView_resize_to (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn BView_set_view_cursor (arg0: *mut c_void, arg1: *mut c_void);
    pub fn BView_move_frame (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_scroll_bar_update (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: bool);
    pub fn be_transform_bitmap (arg0: *mut c_void, arg1: *mut c_void, arg2: uint32_t, arg3: f64, arg4: c_int, arg5: c_int, arg6: bool) -> *mut c_void;
    pub fn be_apply_affine_transform (arg0: *mut c_void, arg1: f64, arg2: f64, arg3: f64, arg4: f64, arg5: f64, arg6: f64);
    pub fn be_apply_inverse_transform (arg0: *mut [f64; 3], arg1: c_int, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int);
    pub fn be_draw_image_mask (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int, arg10: uint32_t);
    pub fn be_draw_bitmap_with_mask (arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int, arg10: c_int, arg11: bool);
    pub fn be_get_display_resolution (arg0: *mut f64, arg1: *mut f64);
    pub fn be_get_screen_dimensions (arg0: *mut c_int, arg1: *mut c_int);
    pub fn be_create_pixmap_cursor (arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
    pub fn be_delete_cursor (arg0: *mut c_void);
    pub fn be_make_scroll_bar_for_view (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> *mut c_void;
    pub fn BScrollBar_delete (arg0: *mut c_void);
    pub fn BScrollBar_default_size (arg0: c_int) -> c_int;
    pub fn BView_invalidate (arg0: *mut c_void);
    pub fn BView_draw_lock (arg0: *mut c_void, arg1: bool, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
    pub fn BView_invalidate_region (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_draw_unlock (arg0: *mut c_void);
    pub fn BBitmap_import_fringe_bitmap (arg0: *mut c_void, arg1: *mut c_ushort, arg2: c_int, arg3: c_int);
    pub fn haiku_font_pattern_free (arg0: *mut haiku_font_pattern);
    pub fn BFont_open_pattern (arg0: *mut haiku_font_pattern, arg1: *mut *mut c_void, arg2: f32) -> c_int;
    pub fn BFont_populate_fixed_family (arg0: *mut haiku_font_pattern);
    pub fn BFont_populate_plain_family (arg0: *mut haiku_font_pattern);
    pub fn BView_publish_scroll_bar (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_forget_scroll_bar (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int);
    pub fn BView_inside_scroll_bar (arg0: *mut c_void, arg1: c_int, arg2: c_int) -> bool;
    pub fn BView_get_mouse (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int);
    pub fn BView_convert_to_screen (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int);
    pub fn BView_convert_from_screen (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int);
    pub fn BView_emacs_delete (arg0: *mut c_void);
    pub fn BPopUpMenu_new (arg0: *const c_char) -> *mut c_void;
    pub fn BMenu_add_item (arg0: *mut c_void, arg1: *const c_char, arg2: *mut c_void, arg3: bool, arg4: bool, arg5: bool, arg6: *mut c_void, arg7: *const c_char, arg8: *const c_char);
    pub fn BMenu_add_separator (arg0: *mut c_void);
    pub fn BMenu_new_submenu (arg0: *mut c_void, arg1: *const c_char, arg2: bool) -> *mut c_void;
    pub fn BMenu_new_menu_bar_submenu (arg0: *mut c_void, arg1: *const c_char) -> *mut c_void;
    pub fn BMenu_count_items (arg0: *mut c_void) -> c_int;
    pub fn BMenu_item_at (arg0: *mut c_void, arg1: c_int) -> *mut c_void;
    pub fn BMenu_run (arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: Option<unsafe extern "C" fn (*mut c_void, *mut c_void)>, arg4: Option<unsafe extern "C" fn ()>, arg5: Option<unsafe extern "C" fn ()>, arg6: Option<unsafe extern "C" fn () -> timespec>, arg7: *mut c_void) -> *mut c_void;
    pub fn BPopUpMenu_delete (arg0: *mut c_void);
    pub fn BMenuBar_new (arg0: *mut c_void) -> *mut c_void;
    pub fn BMenu_delete_all (arg0: *mut c_void);
    pub fn BMenuBar_delete (arg0: *mut c_void);
    pub fn BMenu_item_set_label (arg0: *mut c_void, arg1: *const c_char);
    pub fn BMenu_item_get_menu (arg0: *mut c_void) -> *mut c_void;
    pub fn BMenu_delete_from (arg0: *mut c_void, arg1: c_int, arg2: c_int);
    pub fn haiku_ring_bell ();
    pub fn BAlert_new (arg0: *const c_char, arg1: haiku_alert_type) -> *mut c_void;
    pub fn BAlert_add_button (arg0: *mut c_void, arg1: *const c_char) -> *mut c_void;
    pub fn BAlert_set_offset_spacing (arg0: *mut c_void);
    pub fn BAlert_go (arg0: *mut c_void, arg1: Option<unsafe extern "C" fn ()>, arg2: Option<unsafe extern "C" fn ()>, arg3: Option<unsafe extern "C" fn ()>) -> int32;
    pub fn BButton_set_enabled (arg0: *mut c_void, arg1: c_int);
    pub fn BView_set_tooltip (arg0: *mut c_void, arg1: *const c_char);
    pub fn BView_show_tooltip (arg0: *mut c_void);
    pub fn be_show_sticky_tooltip (arg0: *mut c_void, arg1: *const c_char, arg2: c_int, arg3: c_int);
    pub fn BAlert_delete (arg0: *mut c_void);
    pub fn EmacsWindow_parent_to (arg0: *mut c_void, arg1: *mut c_void);
    pub fn EmacsWindow_unparent (arg0: *mut c_void);
    pub fn EmacsWindow_move_weak_child (arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int);
    pub fn be_get_version_string (arg0: *mut c_char, arg1: c_int);
    pub fn be_get_display_planes () -> c_int;   // C 689: `extern int be_get_display_planes (void);`
    pub fn be_get_display_color_cells () -> c_int;   // C 690: `extern int …`
    pub fn be_is_display_grayscale () -> bool;   // C 691: `extern bool …`
    pub fn be_warp_pointer (arg0: c_int, arg1: c_int);
    pub fn haiku_should_pass_control_tab_to_system ();
    pub fn EmacsView_set_up_double_buffering (arg0: *mut c_void);
    pub fn EmacsView_disable_double_buffering (arg0: *mut c_void);
    pub fn EmacsView_flip_and_blit (arg0: *mut c_void);
    pub fn EmacsView_double_buffered_p (arg0: *mut c_void) -> c_int;
    pub fn be_popup_file_dialog (arg0: c_int, arg1: *const c_char, arg2: c_int, arg3: c_int, arg4: *mut c_void, arg5: *const c_char, arg6: *const c_char, arg7: Option<unsafe extern "C" fn ()>) -> *mut c_char;
    pub fn BMenuBar_start_tracking (arg0: *mut c_void) -> bool;
    pub fn BBitmap_bytes_length (arg0: *mut c_void) -> size_t;
    pub fn BMenu_add_title (arg0: *mut c_void, arg1: *const c_char);
    pub fn be_plain_font_height () -> c_int;   // C 723: `extern int …`
    pub fn be_string_width_with_plain_font (arg0: *const c_char) -> c_int;
    pub fn be_init_font_data ();
    pub fn be_evict_font_cache ();
    pub fn be_get_display_screens () -> c_int;   // C 727: `extern int …`
    pub fn be_use_subpixel_antialiasing ();
    pub fn be_find_setting (arg0: *const c_char) -> *const c_char;
    pub fn be_list_font_families (arg0: *mut size_t) -> *mut haiku_font_family_or_style;
    pub fn be_font_style_to_flags (arg0: *const c_char, arg1: *mut haiku_font_pattern);
    pub fn be_open_font_at_index (arg0: c_int, arg1: c_int, arg2: f32) -> *mut c_void;
    pub fn be_set_font_antialiasing (arg0: *mut c_void, arg1: bool);
    pub fn be_get_ui_color (arg0: *const c_char, arg1: *mut uint32_t) -> c_int;
    pub fn BMessage_delete (arg0: *mut c_void);
    pub fn be_drag_message (arg0: *mut c_void, arg1: *mut c_void, arg2: bool, arg3: Option<unsafe extern "C" fn ()>, arg4: Option<unsafe extern "C" fn ()>, arg5: Option<unsafe extern "C" fn ()>, arg6: Option<unsafe extern "C" fn () -> bool>) -> bool;
    pub fn be_drag_and_drop_in_progress ();
    pub fn be_replay_menu_bar_event (arg0: *mut c_void, arg1: *mut haiku_menu_bar_click_event) -> bool;
    pub fn be_select_font (arg0: Option<unsafe extern "C" fn ()>, arg1: Option<unsafe extern "C" fn () -> bool>, arg2: *mut haiku_font_family_or_style, arg3: *mut haiku_font_family_or_style, arg4: *mut c_int, arg5: bool, arg6: c_int, arg7: c_int, arg8: c_int, arg9: bool, arg10: *mut bool) -> bool;
    pub fn be_find_font_indices (arg0: *mut haiku_font_pattern, arg1: *mut c_int, arg2: *mut c_int) -> c_int;
    pub fn be_roster_launch (arg0: *const c_char, arg1: *const c_char, arg2: *mut *mut c_char, arg3: ptrdiff_t, arg4: *mut c_void, arg5: *mut team_id) -> status_t;
    pub fn be_get_window_decorator_dimensions (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int);
    pub fn be_get_window_decorator_frame (arg0: *mut c_void, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int);
    pub fn be_send_move_frame_event (arg0: *mut c_void);
    pub fn be_set_window_fullscreen_mode (arg0: *mut c_void, arg1: haiku_fullscreen_mode);
    pub fn be_write_node_message (arg0: *const c_char, arg1: *const c_char, arg2: *mut c_void) -> status_t;
    pub fn be_send_message (arg0: *const c_char, arg1: *mut c_void);
    pub fn be_lock_window (arg0: *mut c_void);
    pub fn be_unlock_window (arg0: *mut c_void);
    pub fn be_get_explicit_workarea (arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> bool;
    pub fn be_clear_grab_view ();
    pub fn be_set_use_frame_synchronization (arg0: *mut c_void, arg1: bool);
    pub fn be_listen_font_settings ();
    pub fn be_lock_font_defaults ();
    pub fn be_get_font_default (arg0: haiku_what_font) -> *const c_char;
    pub fn be_get_font_size (arg0: haiku_what_font) -> c_int;
    pub fn be_unlock_font_defaults ();
  }
}
use haiku_support_decls::*;

// C 780-782, the file's own local-variables block, carried verbatim (PORTING.md 0.3).
// Local Variables:
// eval: (setf (alist-get 'inextern-lang c-offsets-alist) 0)
// End:
