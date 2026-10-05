// femacs: port of src/androidgui.h @ fd7afced403.

// Header mirror (PORTING.md 0.1): declarations only.  `androidgui.h` is the Android backend's
// X-shaped GUI header; `androidterm.c`/`androidfns.c`/`androidfont.c` include it, and its own
// `#include <stdint.h>` (C 22) is the only include that reaches a type the prototypes use.
// `ANDROID_STUBIFY` is undefined in the frozen macro table (so the C 256-867 block is live, not
// a stub arm - `androidgui.h` is in `docs/baseline/config/cond-eval-verified.txt`) and the file
// has no other conditional, so **nothing here is dead text**: every declaration is carried.
//
// Names that repeat other headers' text: the X-compat block (C 218-254: `NativeRectangle`,
// `STORE_NATIVE_RECT`, the eleven `*Gravity`, the eight `*Value`, the ten `*Position` macros) is
// the same block `haikugui.h`/`nsgui.h`/`pgtkgui.h`/`w32gui.h` declare, `XCharStruct`,
// `Emacs_Pixmap`, `Emacs_Window`, `Emacs_Cursor`, `Drawable` and `GContext` are declared by
// `haikugui.h` too, and `Emacs_Cursor`/`Emacs_Rectangle` are `dispextern.rs`'s
// `#ifndef HAVE_WINDOW_SYSTEM` arm.  C separates the twins with platform `#if`s; the mirrors meet
// in one crate when `xtask layout-check` compiles them (both are platform-exclusive there) and
// again in an Android unit's `include!` graph, which reaches `androidgui.rs` and `dispextern.rs`
// through `androidterm.rs` 41/44.  **`Emacs_Cursor`'s `E0428` is resolved on the other mirror**:
// C turns `dispextern.h` 175 off whenever a window system is on (`HAVE_ANDROID` implies
// `HAVE_WINDOW_SYSTEM`), so `dispextern.rs`'s arm carries `all (not have_window_system,
// not have_android)` and this file's alias stays exactly as C declares it (androidgui.h 42).
// The 2026-10-04 ruling recorded below was "no `cfg` *here*", and this file still has none;
// what changed is that the twin now carries C's own condition.  docs/ports/include-androidgui.md
// records the reading.

include!("../../lib/include/stdint.in.rs");     // C 22: `#include <stdint.h>`
include!("../../lib/include/wchar.rs");         // `wchar_t` (C 812's `android_wc_lookup_string`)

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _ANDROID_GUI_H_: bool = true;

// C 23-33: `typedef struct android_char_struct { … } XCharStruct;` - here the typedef name comes
// after the closing brace, so the extractor does see the struct (unlike `haikugui.h`'s spelling)
// and the ledger lists its five fields.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_char_struct
{
  pub rbearing: c_int,
  pub lbearing: c_int,
  pub width: c_int,
  pub ascent: c_int,
  pub descent: c_int,
}
pub type XCharStruct = android_char_struct;

// C 35-42: handles are JNI handles cast to `intptr_t` (the port's `intptr_t` is lisp.rs's
// `isize`, like `size_t`/`ptrdiff_t` - PORTING.md 0.1-9: no `src/` mirror includes lisp.rs).
pub type android_handle = intptr_t;

pub type android_pixmap = android_handle;
pub type Emacs_Pixmap = android_handle;
pub type android_window = android_handle;
pub type Emacs_Window = android_handle;
pub type android_gcontext = android_handle;
pub type GContext = android_handle;
pub type android_drawable = android_handle;
pub type Drawable = android_handle;
pub type android_cursor = android_handle;
pub type Emacs_Cursor = android_handle;

// C 44: `typedef unsigned int android_time;`
pub type android_time = c_uint;


// C 46-56.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_rectangle
{
  pub x: c_int,
  pub y: c_int,
  pub width: c_uint,
  pub height: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_point
{
  pub x: c_int,
  pub y: c_int,
}

/* Keep this in sync with EmacsGC.java!  */
// C 58-113: four enums plus the window-value mask.  C's members carry explicit values.
pub type android_gc_function = c_int;
pub const ANDROID_GC_COPY: android_gc_function = 0;
pub const ANDROID_GC_INVERT: android_gc_function = 1;

pub type android_gc_value_mask = c_int;
pub const ANDROID_GC_FOREGROUND: android_gc_value_mask = 1 << 0;
pub const ANDROID_GC_BACKGROUND: android_gc_value_mask = 1 << 1;
pub const ANDROID_GC_FUNCTION: android_gc_value_mask = 1 << 2;
pub const ANDROID_GC_CLIP_X_ORIGIN: android_gc_value_mask = 1 << 3;
pub const ANDROID_GC_CLIP_Y_ORIGIN: android_gc_value_mask = 1 << 4;
pub const ANDROID_GC_CLIP_MASK: android_gc_value_mask = 1 << 5;
pub const ANDROID_GC_STIPPLE: android_gc_value_mask = 1 << 6;
pub const ANDROID_GC_FILL_STYLE: android_gc_value_mask = 1 << 7;
pub const ANDROID_GC_TILE_STIP_X_ORIGIN: android_gc_value_mask = 1 << 8;
pub const ANDROID_GC_TILE_STIP_Y_ORIGIN: android_gc_value_mask = 1 << 9;
pub const ANDROID_GC_LINE_STYLE: android_gc_value_mask = 1 << 10;
pub const ANDROID_GC_LINE_WIDTH: android_gc_value_mask = 1 << 11;
pub const ANDROID_GC_DASH_LIST: android_gc_value_mask = 1 << 12;
pub const ANDROID_GC_DASH_OFFSET: android_gc_value_mask = 1 << 13;

pub type android_fill_style = c_int;
pub const ANDROID_FILL_SOLID: android_fill_style = 0;
pub const ANDROID_FILL_OPAQUE_STIPPLED: android_fill_style = 1;

pub type android_line_style = c_int;
pub const ANDROID_LINE_SOLID: android_line_style = 0;
pub const ANDROID_LINE_ON_OFF_DASH: android_line_style = 1;

pub type android_window_value_mask = c_int;
pub const ANDROID_CW_BACK_PIXEL: android_window_value_mask = 1 << 1;
pub const ANDROID_CW_OVERRIDE_REDIRECT: android_window_value_mask = 1 << 2;

// C 115-194.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_set_window_attributes
{
  /* The background pixel.  */
  pub background_pixel: c_ulong,

  /* Whether or not the window is override redirect.  This cannot be
     set after creation on Android.  */
  pub override_redirect: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_gc_values
{
  /* The foreground and background.  */
  pub foreground: c_ulong,
  pub background: c_ulong,

  /* The function.  */
  pub function: android_gc_function,

  /* The fill style.  */
  pub fill_style: android_fill_style,

  /* The clip X and Y origin.  */
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,

  /* The clip mask image and stipple.  */
  pub clip_mask: android_pixmap,
  pub stipple: android_pixmap,

  /* The tile-stipple X and Y origins.  */
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,

  /* The line style.  */
  pub line_style: android_line_style,

  /* The line width.  */
  pub line_width: c_int,

  /* Offset in pixels into the dash pattern specified below.  */
  pub dash_offset: c_int,

  /* One integer providing both segments of a even-odd dash pattern.  */
  pub dash: c_int,
}

/* X-like graphics context structure.  This is implemented in
   EmacsGC.java, but a copy is kept here to avoid sending changes all
   the time.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_gc
{
  /* Array of clip rectangles.  */
  pub clip_rects: *mut android_rectangle,

  /* Number of clip rectangles.  When -1, it means clipping should not
     be applied.  */
  pub num_clip_rects: c_int,

  /* The Java-side handle.  */
  pub gcontext: android_gcontext,

  /* Current foreground color.  */
  pub foreground: c_ulong,

  /* Current background color.  */
  pub background: c_ulong,

  /* The function.  */
  pub function: android_gc_function,

  /* The fill style.  */
  pub fill_style: android_fill_style,

  /* The clip X and Y origin.  */
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,

  /* The clip mask image and stipple.  */
  pub clip_mask: android_pixmap,
  pub stipple: android_pixmap,

  /* The tile-stipple X and Y origins.  */
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,

  /* The line style.  */
  pub line_style: android_line_style,

  /* The line width.  */
  pub line_width: c_int,

  /* Offset in pixels into the dash pattern specified below.  */
  pub dash_offset: c_int,

  /* The segments of an even/odd dash pattern.  */
  pub dashes: *mut c_int,
  pub n_segments: c_int,
}

pub type android_swap_action = c_int;
pub const ANDROID_COPIED: android_swap_action = 0;

pub type android_shape = c_int;
pub const ANDROID_CONVEX: android_shape = 0;

pub type android_coord_mode = c_int;
pub const ANDROID_COORD_MODE_ORIGIN: android_coord_mode = 0;

// C 204-215.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_swap_info
{
  /* The window to swap.  */
  pub swap_window: android_window,

  /* Unused field present only for consistency with X.  */
  pub swap_action: android_swap_action,
}

// C 218: `#define NativeRectangle Emacs_Rectangle` - a type macro, so it is a `macro_rules!`
// with no metavariables (a `pub type` alias is not what the gate reads for a macro, and C uses
// the name in type position).  `Emacs_Rectangle` itself is `dispextern.rs`'s
// `#ifndef HAVE_WINDOW_SYSTEM` typedef, which is the arm the frozen target takes.
macro_rules! NativeRectangle {
  () => { Emacs_Rectangle };
}

// C 220-223.
macro_rules! STORE_NATIVE_RECT {
  ($nr:expr, $rx:expr, $ry:expr, $rwidth:expr, $rheight:expr) =>
    { {
        ($nr).x = ($rx);
        ($nr).y = ($ry);
        ($nr).width = ($rwidth);
        ($nr).height = ($rheight);
      } };
}

/* Values for the `gravity' field of XSetWindowAttributes.  */
// C 224-254, the X-compat block (see the note at the top).
pub const ForgetGravity: c_int = 0;
pub const NorthWestGravity: c_int = 1;
pub const NorthGravity: c_int = 2;
pub const NorthEastGravity: c_int = 3;
pub const WestGravity: c_int = 4;
pub const CenterGravity: c_int = 5;
pub const EastGravity: c_int = 6;
pub const SouthWestGravity: c_int = 7;
pub const SouthGravity: c_int = 8;
pub const SouthEastGravity: c_int = 9;
pub const StaticGravity: c_int = 10;

pub const NoValue: c_int = 0x0000;
pub const XValue: c_int = 0x0001;
pub const YValue: c_int = 0x0002;
pub const WidthValue: c_int = 0x0004;
pub const HeightValue: c_int = 0x0008;
pub const AllValues: c_int = 0x000F;
pub const XNegative: c_int = 0x0010;
pub const YNegative: c_int = 0x0020;

pub const USPosition: c_long = 1 << 0; /* user specified x, y */
pub const USSize: c_long = 1 << 1;     /* user specified width, height */
pub const PPosition: c_long = 1 << 2;  /* program specified position */
pub const PSize: c_long = 1 << 3;      /* program specified size */
pub const PMinSize: c_long = 1 << 4;   /* program specified minimum size */
pub const PMaxSize: c_long = 1 << 5;   /* program specified maximum size */
pub const PResizeInc: c_long = 1 << 6; /* program specified resize increments */
pub const PAspect: c_long = 1 << 7;    /* program specified min, max aspect ratios */
pub const PBaseSize: c_long = 1 << 8;  /* program specified base for incrementing */
pub const PWinGravity: c_long = 1 << 9;/* program specified window gravity */

/* Universal NULL handle.  */
// C 259: `static const int ANDROID_NONE, ANDROID_NO_SYMBOL;` - a tentative definition with no
// initialiser, i.e. two internals of value 0.  `pub const` keeps the value and emits no symbol.
pub const ANDROID_NONE: c_int = 0;
pub const ANDROID_NO_SYMBOL: c_int = 0;

/* Keep these as conceptually close to X as possible: that makes
   synchronizing code between the ports much easier.  */

// C 267-293.
pub type android_event_type = c_int;
pub const ANDROID_KEY_PRESS: android_event_type = 0;
pub const ANDROID_KEY_RELEASE: android_event_type = 1;
pub const ANDROID_CONFIGURE_NOTIFY: android_event_type = 2;
pub const ANDROID_FOCUS_IN: android_event_type = 3;
pub const ANDROID_FOCUS_OUT: android_event_type = 4;
pub const ANDROID_WINDOW_ACTION: android_event_type = 5;
pub const ANDROID_ENTER_NOTIFY: android_event_type = 6;
pub const ANDROID_LEAVE_NOTIFY: android_event_type = 7;
pub const ANDROID_MOTION_NOTIFY: android_event_type = 8;
pub const ANDROID_BUTTON_PRESS: android_event_type = 9;
pub const ANDROID_BUTTON_RELEASE: android_event_type = 10;
pub const ANDROID_TOUCH_DOWN: android_event_type = 11;
pub const ANDROID_TOUCH_UP: android_event_type = 12;
pub const ANDROID_TOUCH_MOVE: android_event_type = 13;
pub const ANDROID_WHEEL: android_event_type = 14;
pub const ANDROID_ICONIFIED: android_event_type = 15;
pub const ANDROID_DEICONIFIED: android_event_type = 16;
pub const ANDROID_CONTEXT_MENU: android_event_type = 17;
pub const ANDROID_EXPOSE: android_event_type = 18;
pub const ANDROID_INPUT_METHOD: android_event_type = 19;
pub const ANDROID_DND_DRAG_EVENT: android_event_type = 20;
pub const ANDROID_DND_URI_EVENT: android_event_type = 21;
pub const ANDROID_DND_TEXT_EVENT: android_event_type = 22;
pub const ANDROID_NOTIFICATION_DELETED: android_event_type = 23;
pub const ANDROID_NOTIFICATION_ACTION: android_event_type = 24;
pub const ANDROID_CONFIGURATION_CHANGED: android_event_type = 25;

// C 295-300.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_any_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
}

// C 302-310.
pub type android_modifier_mask = c_int;
pub const ANDROID_SHIFT_MASK: android_modifier_mask = 193;
pub const ANDROID_CONTROL_MASK: android_modifier_mask = 4096;
pub const ANDROID_ALT_MASK: android_modifier_mask = 2;
pub const ANDROID_SUPER_MASK: android_modifier_mask = 4;
pub const ANDROID_META_MASK: android_modifier_mask = 65536;

// C 312-332.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_key_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub time: android_time,
  pub state: c_uint,
  pub keycode: c_uint,

  /* If this field is -1, then android_lookup_string should be called
     to retrieve the associated individual characters.  */
  pub unicode_char: c_uint,

  /* If this field is non-zero, a text conversion barrier should be
     generated with its value as the counter.  */
  pub counter: c_ulong,
}

// C 334.
pub type android_key_pressed_event = android_key_event;

/* These hard coded values are Android modifier keycodes derived
   through experimentation.  */

// C 338-345.
macro_rules! ANDROID_IS_MODIFIER_KEY {
  ($key:expr) =>
    { (($key) == 57 || ($key) == 58 || ($key) == 113 || ($key) == 114
       || ($key) == 119 || ($key) == 117 || ($key) == 118 || ($key) == 78
       || ($key) == 94 || ($key) == 59 || ($key) == 60 || ($key) == 95
       || ($key) == 63 || ($key) == 115) };
}

// C 347-379.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_configure_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub time: android_time,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_focus_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub time: android_time,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_window_action_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,

  /* The window handle.  This can be ANDROID_NONE.  */
  pub window: android_window,

  /* Numerical identifier for this action.  If 0 and WINDOW is set,
     then it means the frame associated with that window has been
     destroyed.  Otherwise, it means Emacs should create a new
     frame.  */
  pub action: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_crossing_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub x: c_int,
  pub y: c_int,
  pub time: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_motion_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub x: c_int,
  pub y: c_int,
  pub time: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_button_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub x: c_int,
  pub y: c_int,
  pub time: c_ulong,
  pub state: c_uint,
  pub button: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_expose_event
{
  pub r#type: android_event_type,
  pub serial: c_ulong,
  pub window: android_window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
}

// C 386-392.
pub type android_touch_event_flags = c_int;
/* This touch sequence has been intercepted by the WM (probably
   for back gesture navigation or some such.)  */
pub const ANDROID_TOUCH_SEQUENCE_CANCELED: android_touch_event_flags = 1;

// C 394-420.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_touch_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* Serial identifying the event.  */
  pub serial: c_ulong,

  /* Window associated with the event.  */
  pub window: android_window,

  /* X and Y coordinates of the event.  */
  pub x: c_int,
  pub y: c_int,

  /* Time of the event, and the pointer identifier.  */
  pub time: c_ulong,

  /* Index of the pointer being tracked.  */
  pub pointer_id: c_uint,

  /* Flags associated with this event.  */
  pub flags: c_int,
}

// C 422-447.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_wheel_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* Serial identifying the event.  */
  pub serial: c_ulong,

  /* Window associated with the event.  */
  pub window: android_window,

  /* X and Y coordinates of the event.  */
  pub x: c_int,
  pub y: c_int,

  /* Time of the event, and the pointer identifier.  */
  pub time: c_ulong,

  /* Modifier state at the time of the event.  */
  pub state: c_int,

  /* Motion alongside the X and Y axes.  */
  pub x_delta: f64,
  pub y_delta: f64,
}

// C 449-476.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_iconify_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* Serial identifying the event.  */
  pub serial: c_ulong,

  /* Window associated with the event.  */
  pub window: android_window,
}

// C 478-497.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_menu_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* Serial identifying the event.  */
  pub serial: c_ulong,

  /* Window associated with the event.  Always None.  */
  pub window: android_window,

  /* Menu event ID.  */
  pub menu_event_id: c_int,

  /* Menu event serial; this counter identifies the context menu.  */
  pub menu_event_serial: c_int,
}

// C 499-511.
pub type android_ime_operation = c_int;
pub const ANDROID_IME_COMMIT_TEXT: android_ime_operation = 0;
pub const ANDROID_IME_DELETE_SURROUNDING_TEXT: android_ime_operation = 1;
pub const ANDROID_IME_FINISH_COMPOSING_TEXT: android_ime_operation = 2;
pub const ANDROID_IME_SET_COMPOSING_TEXT: android_ime_operation = 3;
pub const ANDROID_IME_SET_COMPOSING_REGION: android_ime_operation = 4;
pub const ANDROID_IME_SET_POINT: android_ime_operation = 5;
pub const ANDROID_IME_START_BATCH_EDIT: android_ime_operation = 6;
pub const ANDROID_IME_END_BATCH_EDIT: android_ime_operation = 7;
pub const ANDROID_IME_REQUEST_SELECTION_UPDATE: android_ime_operation = 8;
pub const ANDROID_IME_REQUEST_CURSOR_UPDATES: android_ime_operation = 9;
pub const ANDROID_IME_REPLACE_TEXT: android_ime_operation = 10;

// C 513-517: C's anonymous enum, so the port declares the same two constants.
pub const ANDROID_CURSOR_UPDATE_IMMEDIATE: c_int = 1;
pub const ANDROID_CURSOR_UPDATE_MONITOR: c_int = 1 << 1;

// C 519-555.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_ime_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* The event serial.  */
  pub serial: c_ulong,

  /* The associated window.  */
  pub window: android_window,

  /* What operation is being performed.  */
  pub operation: android_ime_operation,

  /* The details of the operation.  START and END provide buffer
     indices, and may actually mean ``left'' and ``right''.  */
  pub start: ptrdiff_t,
  pub end: ptrdiff_t,
  pub position: ptrdiff_t,

  /* The number of characters in TEXT.

     If OPERATION is ANDROID_IME_REQUEST_CURSOR_UPDATES, then this is
     actually the cursor update mode associated with that
     operation.  */
  pub length: size_t,

  /* TEXT is either NULL, or a pointer to LENGTH bytes of malloced
     UTF-16 encoded text that must be decoded by Emacs.

     POSITION is where point should end up after the text is
     committed, relative to TEXT.  If POSITION is less than 0, it is
     relative to TEXT's start; otherwise, it is relative to its
     end.  */
  pub text: *mut c_ushort,

  /* Value to set the counter to after the operation completes.  */
  pub counter: c_ulong,
}

// C 557-583.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_dnd_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* The event serial.  */
  pub serial: c_ulong,

  /* The window that gave rise to the event.  */
  pub window: android_window,

  /* X and Y coordinates of the event.  */
  pub x: c_int,
  pub y: c_int,

  /* Data tied to this event, such as a URI or clipboard string.
     Must be deallocated with `free'.  */
  pub uri_or_string: *mut c_ushort,

  /* Length of that data.  */
  pub length: size_t,
}

// C 585-612.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_notification_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* The event serial.  */
  pub serial: c_ulong,

  /* The window that gave rise to the event (None).  */
  pub window: android_window,

  /* The identifier of the notification whose status changed.
     Must be deallocated with `free'.  */
  pub tag: *mut c_char,

  /* The action that was activated, if any.  Must be deallocated with
     `free'.  */
  pub action: *mut c_ushort,

  /* Length of that data.  */
  pub length: size_t,
}

// C 614-618.
pub type android_configuration_change_type = c_int;
pub const ANDROID_PIXEL_DENSITY_CHANGED: android_configuration_change_type = 0;
pub const ANDROID_UI_MODE_CHANGED: android_configuration_change_type = 1;

// C 620-623.
pub const UI_MODE_NIGHT_MASK: c_int = 0x00000030;
pub const UI_MODE_NIGHT_NO: c_int = 0x00000010;
pub const UI_MODE_NIGHT_YES: c_int = 0x00000020;
pub const UI_MODE_NIGHT_UNDEFINED: c_int = 0x00000000;

// C 625-648: the anonymous `union { … } u` whose first member is the anonymous
// `struct { … } pixel_density`.  PORTING.md P-67 names an anonymous union after its member:
// `android_configuration_changed_event_u`, and the nested struct after the union and its own
// member.  Neither type may be flattened (the layout is what C's compiler measured).
#[repr(C)]
#[derive(Clone, Copy)]
pub union android_configuration_changed_event_u
{
  pub pixel_density: android_configuration_changed_event_u_pixel_density,
  /* A change in the reported user interface UI mode.  */
  pub ui_mode: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_configuration_changed_event_u_pixel_density
{
  /* The density of the display along the horizontal and vertical
     axes.  */
  pub dpi_x: f64,
  pub dpi_y: f64,

  /* The density to take into account when converting between point
     and pixel dimensions.  */
  pub dpi_scaled: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_configuration_changed_event
{
  /* Type of the event.  */
  pub r#type: android_event_type,

  /* The event serial.  */
  pub serial: c_ulong,

  /* The window that gave rise to the event (None).  */
  pub window: android_window,

  /* What type of change this event represents.  */
  pub detail: android_configuration_change_type,

  pub u: android_configuration_changed_event_u,
}

// C 650-688: the union of every event record, by value.
#[repr(C)]
#[derive(Clone, Copy)]
pub union android_event
{
  pub r#type: android_event_type,
  pub xany: android_any_event,
  pub xkey: android_key_event,
  pub xconfigure: android_configure_event,
  pub xfocus: android_focus_event,
  pub xaction: android_window_action_event,
  pub xcrossing: android_crossing_event,
  pub xmotion: android_motion_event,
  pub xbutton: android_button_event,
  pub xexpose: android_expose_event,

  /* This has no parallel in X, since the X model of having
     monotonically increasing touch IDs can't work on Android.  */
  pub touch: android_touch_event,

  /* This has no parallel in X outside the X Input Extension, and
     emulating the input extension interface would be awfully
     complicated.  */
  pub wheel: android_wheel_event,

  /* This has no parallel in X because Android doesn't have window
     properties.  */
  pub iconified: android_iconify_event,

  /* This is only used to transmit selected menu items.  */
  pub menu: android_menu_event,

  /* This is used to dispatch input method editing requests.  */
  pub ime: android_ime_event,

  /* There is no analog under X because Android defines a strict DND
     protocol, whereas there exist several competing X protocols
     implemented in terms of X client messages.  */
  pub dnd: android_dnd_event,

  /* X provides no equivalent interface for displaying
     notifications.  */
  pub notification: android_notification_event,

  /* The equivalent under X is provided through XSettings, which is a
     byzantine protocol that extends client messages and is therefore
     not worthwhile to emulate.  */
  pub config: android_configuration_changed_event,
}

// C 690-693: C's anonymous enum; `0L` is a `long`, so the constant is `c_long`.
pub const ANDROID_CURRENT_TIME: c_long = 0;

// C 695-701.
pub type android_lookup_status = c_int;
pub const ANDROID_BUFFER_OVERFLOW: android_lookup_status = 0;
pub const ANDROID_LOOKUP_NONE: android_lookup_status = 1;
pub const ANDROID_LOOKUP_CHARS: android_lookup_status = 2;
pub const ANDROID_LOOKUP_KEYSYM: android_lookup_status = 3;
pub const ANDROID_LOOKUP_BOTH: android_lookup_status = 4;

// C 703-709.
pub type android_ic_mode = c_int;
pub const ANDROID_IC_MODE_NULL: android_ic_mode = 0;
pub const ANDROID_IC_MODE_ACTION: android_ic_mode = 1;
pub const ANDROID_IC_MODE_TEXT: android_ic_mode = 2;
pub const ANDROID_IC_MODE_PASSWORD: android_ic_mode = 3;

// C 711-715.
pub type android_stack_mode = c_int;
pub const ANDROID_ABOVE: android_stack_mode = 0;
pub const ANDROID_BELOW: android_stack_mode = 1;

// C 717-721.
pub type android_wc_value_mask = c_int;
pub const ANDROID_CW_SIBLING: android_wc_value_mask = 0;
pub const ANDROID_CW_STACK_MODE: android_wc_value_mask = 1;

// C 723-727.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_window_changes
{
  pub sibling: android_window,
  pub stack_mode: android_stack_mode,
}

// C 729-737.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_compose_status
{
  /* Accent character to be combined with another.  */
  pub accent: c_uint,

  /* Number of characters matched.  */
  pub chars_matched: c_int,
}

// C 846-861.
pub type android_cursor_shape = c_int;
pub const ANDROID_XC_XTERM: android_cursor_shape = 1008;
pub const ANDROID_XC_LEFT_PTR: android_cursor_shape = 1000;
pub const ANDROID_XC_WATCH: android_cursor_shape = 1004;
pub const ANDROID_XC_HAND2: android_cursor_shape = 1002;
pub const ANDROID_XC_SB_H_DOUBLE_ARROW: android_cursor_shape = 1014;
pub const ANDROID_XC_SB_V_DOUBLE_ARROW: android_cursor_shape = 1015;
pub const ANDROID_XC_LEFT_SIDE: android_cursor_shape = 1020;
pub const ANDROID_XC_TOP_LEFT_CORNER: android_cursor_shape = 1020;
pub const ANDROID_XC_TOP_SIDE: android_cursor_shape = 1020;
pub const ANDROID_XC_TOP_RIGHT_CORNER: android_cursor_shape = 1020;
pub const ANDROID_XC_RIGHT_SIDE: android_cursor_shape = 1020;
pub const ANDROID_XC_BOTTOM_RIGHT_CORNER: android_cursor_shape = 1020;
pub const ANDROID_XC_BOTTOM_SIDE: android_cursor_shape = 1020;
pub const ANDROID_XC_BOTTOM_LEFT_CORNER: android_cursor_shape = 1020;
pub const ANDROID_XC_NULL: android_cursor_shape = 0;

/* Image support.  Keep the API as similar to XImage as possible.  To
   avoid leaving a huge mess of "#ifndef ANDROID_STUBIFY" in image.c,
   stubs should be defined for all functions.  */

// C 876-879.
pub type android_image_format = c_int;
pub const ANDROID_Z_PIXMAP: android_image_format = 0;

// C 881-890.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_image
{
  pub width: c_int,
  pub height: c_int,
  pub format: android_image_format,
  pub data: *mut c_char,
  pub depth: c_int,
  pub bytes_per_line: c_int,
  pub bits_per_pixel: c_int,
}

/* 3x2 matrix describing a projective transform.  See
   android_transform_coordinates for details.  */

// C 911-917.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_transform
{
  pub m1: f32,
  pub m2: f32,
  pub m3: f32,
  pub m4: f32,
  pub m5: f32,
  pub m6: f32,
}

/* The 63 prototypes C spells `extern`.  They sit in a glob-imported module: `androidterm.rs` and
   the unit that ports `android.c`-adjacent C code define some of them, and a same-named
   declaration at this scope would be `E0428` (PORTING.md 0.1-10/P-17/P-21).  `extract-abi.py`
   does not inventory prototypes, so this block is the part of the mirror no gate compares; it was
   checked name-for-name against the reference (63 of 63).  */
pub mod androidgui_decls {
  use super::*;

  unsafe extern "C" {
    pub fn android_pending () -> c_int;   // C: `extern int android_pending (void);`
    pub fn android_next_event (arg0: *mut android_event);
    pub fn android_check_if_event (arg0: *mut android_event, arg1: Option<unsafe extern "C" fn (*mut android_event, *mut c_void) -> bool>, arg2: *mut c_void) -> bool;
    pub fn android_create_window (arg0: android_window, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: android_window_value_mask, arg6: *mut android_set_window_attributes) -> android_window;
    pub fn android_change_window_attributes (arg0: android_window, arg1: android_window_value_mask, arg2: *mut android_set_window_attributes);
    pub fn android_set_window_background (arg0: android_window, arg1: c_ulong);
    pub fn android_destroy_window (arg0: android_window);
    pub fn android_reparent_window (arg0: android_window, arg1: android_window, arg2: c_int, arg3: c_int);
    pub fn android_set_clip_rectangles (arg0: *mut android_gc, arg1: c_int, arg2: c_int, arg3: *mut android_rectangle, arg4: c_int);
    pub fn android_set_dashes (arg0: *mut android_gc, arg1: c_int, arg2: *mut c_int, arg3: c_int);
    pub fn android_change_gc (arg0: *mut android_gc, arg1: android_gc_value_mask, arg2: *mut android_gc_values);
    pub fn android_clear_window (arg0: android_window);
    pub fn android_map_window (arg0: android_window);
    pub fn android_unmap_window (arg0: android_window);
    pub fn android_resize_window (arg0: android_window, arg1: c_uint, arg2: c_uint);
    pub fn android_move_window (arg0: android_window, arg1: c_int, arg2: c_int);
    pub fn android_swap_buffers (arg0: *mut android_swap_info, arg1: c_int);
    pub fn android_get_gc_values (arg0: *mut android_gc, arg1: android_gc_value_mask, arg2: *mut android_gc_values);
    pub fn android_set_foreground (arg0: *mut android_gc, arg1: c_ulong);
    pub fn android_fill_rectangle (arg0: android_drawable, arg1: *mut android_gc, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint);
    pub fn android_create_pixmap_from_bitmap_data (arg0: *mut c_char, arg1: c_uint, arg2: c_uint, arg3: c_ulong, arg4: c_ulong, arg5: c_uint) -> android_pixmap;
    pub fn android_set_clip_mask (arg0: *mut android_gc, arg1: android_pixmap);
    pub fn android_set_fill_style (arg0: *mut android_gc, arg1: android_fill_style);
    pub fn android_copy_area (arg0: android_drawable, arg1: android_drawable, arg2: *mut android_gc, arg3: c_int, arg4: c_int, arg5: c_uint, arg6: c_uint, arg7: c_int, arg8: c_int);
    pub fn android_free_pixmap (arg0: android_drawable);
    pub fn android_set_background (arg0: *mut android_gc, arg1: c_ulong);
    pub fn android_fill_polygon (arg0: android_drawable, arg1: *mut android_gc, arg2: *mut android_point, arg3: c_int, arg4: android_shape, arg5: android_coord_mode);
    pub fn android_draw_rectangle (arg0: android_drawable, arg1: *mut android_gc, arg2: c_int, arg3: c_int, arg4: c_uint, arg5: c_uint);
    pub fn android_draw_point (arg0: android_window, arg1: *mut android_gc, arg2: c_int, arg3: c_int);
    pub fn android_draw_line (arg0: android_window, arg1: *mut android_gc, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
    pub fn android_create_pixmap (arg0: c_uint, arg1: c_uint, arg2: c_int) -> android_pixmap;
    pub fn android_set_ts_origin (arg0: *mut android_gc, arg1: c_int, arg2: c_int);
    pub fn android_clear_area (arg0: android_window, arg1: c_int, arg2: c_int, arg3: c_uint, arg4: c_uint);
    pub fn android_create_bitmap_from_data (arg0: *mut c_char, arg1: c_uint, arg2: c_uint) -> android_pixmap;
    pub fn android_bell ();
    pub fn android_set_input_focus (arg0: android_window, arg1: c_ulong);
    pub fn android_raise_window (arg0: android_window);
    pub fn android_lower_window (arg0: android_window);
    pub fn android_reconfigure_wm_window (arg0: android_window, arg1: android_wc_value_mask, arg2: *mut android_window_changes);
    pub fn android_query_tree (arg0: android_window, arg1: *mut android_window, arg2: *mut android_window, arg3: *mut *mut android_window, arg4: *mut c_uint) -> c_int;
    pub fn android_get_geometry (arg0: android_window, arg1: *mut android_window, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_uint, arg5: *mut c_uint, arg6: *mut c_uint);
    pub fn android_move_resize_window (arg0: android_window, arg1: c_int, arg2: c_int, arg3: c_uint, arg4: c_uint);
    pub fn android_map_raised (arg0: android_window);
    pub fn android_translate_coordinates (arg0: android_window, arg1: c_int, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int);
    pub fn android_wc_lookup_string (arg0: *mut android_key_pressed_event, arg1: *mut wchar_t, arg2: c_int, arg3: *mut c_int, arg4: *mut android_lookup_status, arg5: *mut android_compose_status) -> c_int;
    pub fn android_recreate_activity (arg0: android_window);
    pub fn android_update_ic (arg0: android_window, arg1: ptrdiff_t, arg2: ptrdiff_t, arg3: ptrdiff_t, arg4: ptrdiff_t);
    pub fn android_reset_ic (arg0: android_window, arg1: android_ic_mode);
    pub fn android_update_extracted_text (arg0: android_window, arg1: *mut c_void, arg2: c_int);
    pub fn android_update_cursor_anchor_info (arg0: android_window, arg1: f32, arg2: f32, arg3: f32, arg4: f32);
    pub fn android_set_fullscreen (arg0: android_window, arg1: bool) -> c_int;
    pub fn android_create_font_cursor (arg0: android_cursor_shape) -> android_cursor;
    pub fn android_define_cursor (arg0: android_window, arg1: android_cursor);
    pub fn android_free_cursor (arg0: android_cursor);
    pub fn android_create_image (arg0: c_uint, arg1: android_image_format, arg2: *mut c_char, arg3: c_uint, arg4: c_uint) -> *mut android_image;
    pub fn android_destroy_image (arg0: *mut android_image);
    pub fn android_put_pixel (arg0: *mut android_image, arg1: c_int, arg2: c_int, arg3: c_ulong);
    pub fn android_get_pixel (arg0: *mut android_image, arg1: c_int, arg2: c_int) -> c_ulong;
    pub fn android_get_image (arg0: android_drawable, arg1: android_image_format) -> *mut android_image;
    pub fn android_put_image (arg0: android_pixmap, arg1: *mut android_image);
    pub fn android_project_image_bilinear (arg0: *mut android_image, arg1: *mut android_image, arg2: *mut android_transform);
    pub fn android_project_image_nearest (arg0: *mut android_image, arg1: *mut android_image, arg2: *mut android_transform);
    pub fn android_free_gc (arg0: *mut android_gc);
  }
}
use androidgui_decls::*;
