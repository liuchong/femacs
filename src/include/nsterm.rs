// femacs: port of src/nsterm.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  `nsterm.h` is the NeXT/Open/GNUstep (Cocoa or GNUstep) window
// system's interface: `/* -*- objc -*- */`, C 22-27's six includes, and then everything from C 29
// on inside `#ifdef HAVE_NS` (C 29) **and** `#ifdef __OBJC__` (C 30-785) - the second half of the
// file, C 786-1395, is the "Non-OO stuff" and is only inside `HAVE_NS`.  `HAVE_NS`,
// `NS_IMPL_COCOA`, `NS_IMPL_GNUSTEP` and `__OBJC__` are all `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` (the frozen configuration compiles C, not
// Objective-C), so the frozen target compiles none of the declarations; C's readers are
// `callproc.c`, `lread.c`, `systhread.c` and `thread.c` plus the nine `.m` files, and
// `tools/check-cond-eval.py src/nsterm.h` -> `68 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, so every `#if` below is the frozen configuration's verdict.
//
// **The tool's live inventory for this file is empty** (`tools/extract-abi.py emacs/src/nsterm.h`
// -> 0 macros, 0 constants, 0 fields, 0 objects), because every declaration sits behind those
// guards; the counts the map prints for it (`docs/port-inventory.md` 569: 140 macros, 21
// constants, 102 fields, 8 objects) are the all-branches view, and they are what this mirror
// carries.  Nothing here is compared by `--check`, so the fidelity is C's text (the
// `include/w32gdiplus.rs` situation, one file over).
//
// **The split.**  Everything from C 29 on carries `#[cfg (feature = "have_ns")]` - P-51's arm for
// the non-frozen side, the `include/macfont.rs` (3.170, the `macos_sdk` arm) and
// `include/w32font.rs` (3.176) shape - and spells C's own platform names (`CGFloat`, `NSRect`,
// `NSString`, `NSColor`, `NSView`, `CGContextRef`, `id`, `NSUInteger`, ...), which that target
// supplies; the port has no mirror for AppKit, CoreGraphics or the GNUstep base library.  C's
// inner conditions become nested `#[cfg]`s or, where a condition is not a target feature, a
// recorded note:
//
//   * `NS_IMPL_COCOA` / `NS_IMPL_GNUSTEP` -> the features of the same name.
//   * `MAC_OS_X_VERSION_MIN_REQUIRED`/`MAX_ALLOWED`/`MAC_OS_X_VERSION_10_*` are SDK-version tests,
//     not target switches; the arm C selects for a Cocoa build is carried and the test is
//     recorded in the `/// C:` line (the GNUstep/old-SDK fallback macros C 1301-1391 are under
//     `ns_impl_gnustep`, which is the side that takes them).
//   * `#ifdef __OBJC__` halves are **recorded**, not mirrored (the `include/nsgui.rs` 16-27 reading
//     and this round's rule): C's `#else` half is carried where there is one, and the
//     Objective-C-only declaration is kept in a `// C:` line.  The 19 `@interface`/`@class`
//     blocks of C 350-785 (`NSColor`, `NSString`, `EmacsApp`, `EmacsDocument`, `EmacsWindow`,
//     `EmacsView`, `EmacsMenu`, `EmacsImage`, `EmacsScroller`, `EmacsToolbar`, `EmacsLayer`,
//     `EmacsDialogPanel`, `EmacsTooltip`, `EmacsFileDelegate`, ...) are Objective-C class syntax,
//     which a Rust item cannot express at all; they are recorded in the note at that C line, and
//     so are the three macro bodies that are Objective-C message sends
//     (`FRAME_NS_TITLEBAR_HEIGHT` C 1062, `FRAME_TOOLBAR_HEIGHT` C 1071,
//     `NS_SCROLL_BAR_WIDTH_DEFAULT`/`HEIGHT_DEFAULT` C 1274-1275, the two `@"..."` colour
//     constants C 1278-1279) and the dead-by-construction `NSTRACE` implementation (C 146-295 is
//     inside `#if NSTRACE_ENABLED`, which C 142-144 defines to 0 in this very file; the live
//     fallbacks C 301-343 are carried).
//   * No layout assertion is written for any record: every one of them holds AppKit, GNUstep or
//     CoreGraphics members (`NSRect`, `NSString *`, `CGImageRef`, `BOOL`, ...), so no compiler on
//     this target could measure it (`include/macfont.rs` 26-27, `include/emacsgtkfixed.rs`).
//   * Nothing else is degraded to `*mut c_void`, because the whole body is inert here: C's own
//     names are carried and the platform target resolves them.  The four Objective-C class
//     pointers of C 774-776 are the measured exception - they are the NS arm's only unresolved
//     names (status 3.526).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] dispextern.rs imported via super
// [U1] frame.rs imported via super
// [U1] character.rs imported via super
// [U1] font.rs imported via super
// [U1] sysselect.rs imported via super
// [U1] sysstdio.rs imported via super

// C 29-1395: `#ifdef HAVE_NS` ... `#endif /* HAVE_NS */` - the whole of the file below.

/* C 36-41: "CGFloat on GNUstep may be 4 or 8 byte ... Make compatible type."  C 37's
   `GNUSTEP_GUI_MAJOR_VERSION > 0 || GNUSTEP_GUI_MINOR_VERSION >= 22` version test is recorded in
   the `/// C:` line rather than modelled as a feature: the port folds it into the GNUstep arm.  */
/// C: #ifdef NS_IMPL_COCOA / typedef CGFloat EmacsCGFloat; / #elif GNUSTEP_GUI_MAJOR_VERSION > 0 ||
/// C: GNUSTEP_GUI_MINOR_VERSION >= 22 / typedef CGFloat EmacsCGFloat; / #else typedef float
/// C: EmacsCGFloat; / #endif
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
pub type EmacsCGFloat = CGFloat;
#[cfg (feature = "have_ns")]
#[cfg (all (not (feature = "ns_impl_cocoa"), feature = "ns_impl_gnustep"))]
pub type EmacsCGFloat = CGFloat;
#[cfg (feature = "have_ns")]
#[cfg (all (not (feature = "ns_impl_cocoa"), not (feature = "ns_impl_gnustep")))]
pub type EmacsCGFloat = core::ffi::c_float;

/* C 46-50: "NSFilenamesPboardType is deprecated in macOS 10.14, but NSPasteboardTypeFileURL is
   only available in 10.13".  The Cocoa arm's `MAC_OS_X_VERSION_MIN_REQUIRED >= 101300` test is
   recorded; the value C selects on a Cocoa build is 1.  */
/// C: #if defined NS_IMPL_COCOA && MAC_OS_X_VERSION_MIN_REQUIRED >= 101300 /
/// C: #define NS_USE_NSPasteboardTypeFileURL 1 / #else / #define NS_USE_NSPasteboardTypeFileURL 0 /
/// C: #endif
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
pub const NS_USE_NSPasteboardTypeFileURL: c_int = 1;
#[cfg (feature = "have_ns")]
#[cfg (not (feature = "ns_impl_cocoa"))]
pub const NS_USE_NSPasteboardTypeFileURL: c_int = 0;

// C 347-349: `#ifndef NATIVE_OBJC_INSTANCETYPE / typedef id instancetype; / #endif` - an
// Objective-C `id` typedef, recorded (no Rust item).
// C: typedef id instancetype;

/* C 176-182: the three trace counters and the three trace entry points.  `volatile` is a C
   qualifier with no Rust counterpart in a declaration (PORTING.md 1.1); the count is what the
   NSTRACE macros below read.  */
/// C: extern volatile int nstrace_num;
/// C: extern volatile int nstrace_depth;
/// C: extern volatile int nstrace_enabled_global;
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub static nstrace_num: VmGlobal<c_int>;
  pub static nstrace_depth: VmGlobal<c_int>;
  pub static nstrace_enabled_global: VmGlobal<c_int>;

  /// C 180-182: `void nstrace_leave(int *); void nstrace_restore_global_trace_state(int *);
  /// char const * nstrace_fullscreen_type_name (int);`
  pub fn nstrace_leave (depth: *mut c_int);
  pub fn nstrace_restore_global_trace_state (state: *mut c_int);
  pub fn nstrace_fullscreen_type_name (state: c_int) -> *const c_char;
}

/// C 142-144: `#ifndef NSTRACE_ENABLED / #define NSTRACE_ENABLED 0 / #endif` - the trace build is
/// off unless the source is edited, so the implementation at C 146-295 (the `NSTRACE_MSG_NO_DASHES`
/// machinery, `NSTRACE_FMT_*`/`NSTRACE_ARG_*`, the `__attribute__((cleanup))` hook and the seven
/// `NSTRACE_GROUP_*` switches) is dead by construction and recorded in this mirror's note rather
/// than transcribed (the `include/pgtkterm.rs` class; its names: `NSTRACE_ALL_GROUPS`,
/// `NSTRACE_GROUP_EVENTS`, `NSTRACE_GROUP_UPDATES`, `NSTRACE_GROUP_FRINGE`,
/// `NSTRACE_GROUP_COLOR`, `NSTRACE_GROUP_GLYPHS`, `NSTRACE_GROUP_FOCUS`,
/// `NSTRACE_MSG_NO_DASHES`, `NSTRACE_FMT_SIZE`, `NSTRACE_FMT_POINT`, `NSTRACE_FMT_RECT`,
/// `NSTRACE_FMT_FSTYPE`, `NSTRACE_FMT_RETURN`, `NSTRACE_ARG_SIZE`, `NSTRACE_ARG_POINT`,
/// `NSTRACE_ARG_RECT`, `NSTRACE_ARG_FSTYPE`, `NSTRACE_WHEN` (the real one), `NSTRACE_SIZE`,
/// `NSTRACE_POINT`, `NSTRACE_RECT`, `NSTRACE_FSTYPE`, `NSTRACE_RETURN`,
/// `NSTRACE_RETURN_SIZE`, `NSTRACE_RETURN_POINT`, `NSTRACE_RETURN_RECT`,
/// `NSTRACE_RETURN_FSTYPE`, `NSTRACE_UNSILENCE`).
#[cfg (feature = "have_ns")]
pub const NSTRACE_ENABLED: c_int = 0;

/* C 297-343: the live half of the trace interface - with `NSTRACE_ENABLED` 0 the first `#ifndef`
   pair wins and every one of these is an empty macro, which is what the port writes (a
   `macro_rules!` with an empty body).  C's variadic `__VA_ARGS__` is Rust's `$($args:tt)*`.  */
/// C: #define NSTRACE(...) NSTRACE_WHEN(1, __VA_ARGS__)
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE {
  ($($args:tt)*) => { NSTRACE_WHEN! (1, $($args)*) };
}
/// C: #define NSTRACE_UNLESS(cond, ...) NSTRACE_WHEN(!(cond), __VA_ARGS__)
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_UNLESS {
  ($cond:expr $(, $args:tt)*) => { NSTRACE_WHEN! (!($cond), $($args)*) };
}
/// C 301-303: `#ifndef NSTRACE_WHEN / #define NSTRACE_WHEN(...) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_WHEN { ($($args:tt)*) => {}; }
/// C 305-307: `#ifndef NSTRACE_MSG / #define NSTRACE_MSG(...) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_MSG { ($($args:tt)*) => {}; }
/// C 309-311: `#ifndef NSTRACE_SIZE / #define NSTRACE_SIZE(str,size) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_SIZE { ($str:expr, $size:expr) => {}; }
/// C 313-315: `#ifndef NSTRACE_POINT / #define NSTRACE_POINT(str,point) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_POINT { ($str:expr, $point:expr) => {}; }
/// C 317-319: `#ifndef NSTRACE_RECT / #define NSTRACE_RECT(str,rect) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_RECT { ($str:expr, $rect:expr) => {}; }
/// C 321-323: `#ifndef NSTRACE_FSTYPE / #define NSTRACE_FSTYPE(str,fs_type) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_FSTYPE { ($str:expr, $fs_type:expr) => {}; }
/// C 325-327: `#ifndef NSTRACE_RETURN_SIZE / #define NSTRACE_RETURN_SIZE(size) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_RETURN_SIZE { ($size:expr) => {}; }
/// C 329-331: `#ifndef NSTRACE_RETURN_POINT / #define NSTRACE_RETURN_POINT(point) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_RETURN_POINT { ($point:expr) => {}; }
/// C 333-335: `#ifndef NSTRACE_RETURN_RECT / #define NSTRACE_RETURN_RECT(rect) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_RETURN_RECT { ($rect:expr) => {}; }
/// C 337-339: `#ifndef NSTRACE_RETURN_FSTYPE / #define NSTRACE_RETURN_FSTYPE(fs_type) / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_RETURN_FSTYPE { ($fs_type:expr) => {}; }
/// C 341-343: `#ifndef NSTRACE_UNSILENCE / #define NSTRACE_UNSILENCE() / #endif`
#[cfg (feature = "have_ns")]
macro_rules! NSTRACE_UNSILENCE { () => {}; }

/* C 350-785: the Objective-C class half of the file - `@interface NSColor (EmacsColor)`,
   `NSString (EmacsString)`, `EmacsApp`, `EmacsDocument`, `EmacsWindow`, `EmacsView`, `EmacsMenu`,
   `EmacsImage`, `EmacsScroller`, `EmacsToolbar`, `EmacsLayer`, `EmacsDialogPanel`, `EmacsTooltip`,
   `EmacsFileDelegate`, `EmacsWindow`, the `@class EmacsToolbar`/`EmacsImage`/`EmacsLayer`
   forward declarations, and the `NSApplication (EmacsApp)` category - each with its `{ ... }`
   ivars (AppKit/GNUstep records, `BOOL`, `NSPoint`, `NSRect`, `NSString *`, ...) and its method
   declarations (C 498-784's `- (instancetype)…`/`+ (NSColor *)…` lists).  Objective-C class
   syntax is not expressible as a Rust item, so the region is recorded here rather than
   transcribed; its three plain-C declarations are carried below, and the three globals the
   `#ifdef __OBJC__` arm ends with (C 774-776, `ns_send_types`, `ns_return_types`, `ns_app_name`,
   `svcsMenu`) are carried with the rest of the objects.  */

/* C 411-417: `enum ns_return_frame_mode { RETURN_FRAME_NEVER, RETURN_FRAME_EVENTUALLY,
   RETURN_FRAME_NOW };` - plain C, so it is carried with C's own type name and one constant per
   member (PORTING.md 1.3, the `include/dispextern.rs` `text_cursor_kinds` shape).  */
/// C: enum ns_return_frame_mode { RETURN_FRAME_NEVER, RETURN_FRAME_EVENTUALLY, RETURN_FRAME_NOW };
#[cfg (feature = "have_ns")]
pub type ns_return_frame_mode = c_int;
#[cfg (feature = "have_ns")]
pub const RETURN_FRAME_NEVER: ns_return_frame_mode = 0;
#[cfg (feature = "have_ns")]
pub const RETURN_FRAME_EVENTUALLY: ns_return_frame_mode = 1;
#[cfg (feature = "have_ns")]
pub const RETURN_FRAME_NOW: ns_return_frame_mode = 2;

// C 419-432's `@interface EmacsWindow : NSWindow { NSPoint grabOffset; NSEvent *last_drag_event;
// NSDragOperation drag_op, selected_op; struct frame *dnd_return_frame; enum
// ns_return_frame_mode dnd_mode; BOOL dnd_allow_same_frame, dnd_move_tooltip_with_frame; }` and
// its method list are part of the recorded Objective-C region above.

/* C 774-776: the four objects the `#ifdef __OBJC__` arm ends with ("Apple removed the
   declaration, but kept the implementation" is about the `NSApplication (EmacsApp)` category
   below them, which is recorded with the rest of the Objective-C region).

   The four classes are AppKit's (Cocoa) or GNUstep's, and this tree declares none of them:
   P-72 rule 3 lets a mirror name only a type it declares itself or pulls in, so the statics are
   the untyped `*mut c_void` and the `/// C:` line keeps each class name written down (status
   3.526; the `coding.rs` `ccl_spec` reading of the same rule).  With C's names these were the
   `fontset` arm's last four errors (`E0425 cannot find type NSArray/NSString/EmacsMenu`).  */
/// C: nsterm.h 774: `extern NSArray *ns_send_types, *ns_return_types;`
/// C: nsterm.h 775: `extern NSString *ns_app_name;`
/// C: nsterm.h 776: `extern EmacsMenu *svcsMenu;`
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub static ns_send_types: VmGlobal<*mut c_void>;
  pub static ns_return_types: VmGlobal<*mut c_void>;
  pub static ns_app_name: VmGlobal<*mut c_void>;
  pub static svcsMenu: VmGlobal<*mut c_void>;
}

/* C 795-806: "Special keycodes that we pass down the event chain."  C's implicit type is `int`
   and its value is the same three-term expression for all eleven.  */
/// C: #define KEY_NS_POWER_OFF ((1<<28)|(0<<16)|1)
/// C: #define KEY_NS_OPEN_FILE ((1<<28)|(0<<16)|2)
/// C: #define KEY_NS_OPEN_TEMP_FILE ((1<<28)|(0<<16)|3)
/// C: #define KEY_NS_CHANGE_FONT ((1<<28)|(0<<16)|7)
/// C: #define KEY_NS_OPEN_FILE_LINE ((1<<28)|(0<<16)|8)
/// C: #define KEY_NS_PUT_WORKING_TEXT ((1<<28)|(0<<16)|9)
/// C: #define KEY_NS_UNPUT_WORKING_TEXT ((1<<28)|(0<<16)|10)
/// C: #define KEY_NS_SPI_SERVICE_CALL ((1<<28)|(0<<16)|11)
/// C: #define KEY_NS_NEW_FRAME ((1<<28)|(0<<16)|12)
/// C: #define KEY_NS_TOGGLE_TOOLBAR ((1<<28)|(0<<16)|13)
/// C: #define KEY_NS_SHOW_PREFS ((1<<28)|(0<<16)|14)
#[cfg (feature = "have_ns")]
pub const KEY_NS_POWER_OFF: c_int = (1 << 28) | (0 << 16) | 1;
#[cfg (feature = "have_ns")]
pub const KEY_NS_OPEN_FILE: c_int = (1 << 28) | (0 << 16) | 2;
#[cfg (feature = "have_ns")]
pub const KEY_NS_OPEN_TEMP_FILE: c_int = (1 << 28) | (0 << 16) | 3;
#[cfg (feature = "have_ns")]
pub const KEY_NS_CHANGE_FONT: c_int = (1 << 28) | (0 << 16) | 7;
#[cfg (feature = "have_ns")]
pub const KEY_NS_OPEN_FILE_LINE: c_int = (1 << 28) | (0 << 16) | 8;
#[cfg (feature = "have_ns")]
pub const KEY_NS_PUT_WORKING_TEXT: c_int = (1 << 28) | (0 << 16) | 9;
#[cfg (feature = "have_ns")]
pub const KEY_NS_UNPUT_WORKING_TEXT: c_int = (1 << 28) | (0 << 16) | 10;
#[cfg (feature = "have_ns")]
pub const KEY_NS_SPI_SERVICE_CALL: c_int = (1 << 28) | (0 << 16) | 11;
#[cfg (feature = "have_ns")]
pub const KEY_NS_NEW_FRAME: c_int = (1 << 28) | (0 << 16) | 12;
#[cfg (feature = "have_ns")]
pub const KEY_NS_TOGGLE_TOOLBAR: c_int = (1 << 28) | (0 << 16) | 13;
#[cfg (feature = "have_ns")]
pub const KEY_NS_SHOW_PREFS: c_int = (1 << 28) | (0 << 16) | 14;

/* C 810-820: "Could use list to store these, but rest of emacs has a big infrastructure for
   managing a table of bitmap "records"."  The `img` member's `#ifdef __OBJC__` half is the
   recorded one; C's `#else` half is carried.  */
/// C: struct ns_bitmap_record { [__OBJC__] EmacsImage *img; [else] void *img; char *file;
/// C: int refcount; int height, width, depth; };
#[cfg (feature = "have_ns")]
#[repr(C)]
pub struct ns_bitmap_record
{
  pub img: *mut c_void,             // C: `void *img;` (the `__OBJC__` arm is `EmacsImage *img;`)
  pub file: *mut c_char,
  pub refcount: c_int,
  pub height: c_int,
  pub width: c_int,
  pub depth: c_int,
}

/* C 822-850: `struct nsfont_info` - "this extends font backend font", and it is inside
   `#ifdef NS_IMPL_GNUSTEP`, so it carries that feature.  The `struct font font` member is
   `font.h`'s record by value (included at C 25), `XCharStruct` is `nsgui.h`'s (reached through
   `dispextern.h` 142-148), and the `nsfont` member's `#ifdef __OBJC__` half is recorded.  */
/// C: struct nsfont_info { struct font font; char *name; float width, height, underpos,
/// C: underwidth, size; [__OBJC__] NSFont *nsfont; [else] void *nsfont; char bold, ital;
/// C: char synthItal; XCharStruct max_bounds; unsigned int **glyphs; struct font_metrics **metrics; };
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
#[repr(C)]
pub struct nsfont_info
{
  pub font: font,
  pub name: *mut c_char,
  pub width: core::ffi::c_float,
  pub height: core::ffi::c_float,
  pub underpos: core::ffi::c_float,
  pub underwidth: core::ffi::c_float,
  pub size: core::ffi::c_float,
  pub nsfont: *mut c_void,          // C: `void *nsfont;` (the `__OBJC__` arm is `NSFont *nsfont;`)
  pub bold: c_char,
  pub ital: c_char,
  pub synthItal: c_char,
  pub max_bounds: XCharStruct,
  pub glyphs: *mut *mut c_uint,
  pub metrics: *mut *mut font_metrics,
}

/* C 851-929: `struct ns_display_info` - "Initialized in ns_initialize_display_info ()".  The
   `Window` members are `nsgui.h` 67's `typedef int Window` (reached through `dispextern.h`
   142-148), `Time` is `systime.h`'s (`termhooks.h` 25 includes it, and `frame.h` includes
   `termhooks.h`), `Mouse_HLInfo` is `dispextern.h`'s, `Emacs_Cursor` is `dispextern.rs` 62's, and
   `NSRect`/`EmacsScroller *` are the platform's.  */
/// C: struct ns_display_info { struct ns_display_info *next; struct terminal *terminal;
/// C: Lisp_Object name_list_element; int n_fonts, smallest_char_width, smallest_font_height;
/// C: struct ns_bitmap_record *bitmaps; ptrdiff_t bitmaps_size, bitmaps_last; double resx, resy;
/// C: int grabbed, n_planes, color_p; Window root_window; Lisp_Object rdb; Emacs_Cursor
/// C: vertical_scroll_bar_cursor, horizontal_scroll_bar_cursor; Mouse_HLInfo mouse_highlight;
/// C: struct frame *highlight_frame, *ns_focus_frame, *last_mouse_frame,
/// C: *last_mouse_motion_frame; int last_mouse_motion_x, last_mouse_motion_y;
/// C: NSRect last_mouse_glyph; Time last_mouse_movement_time;
/// C: [__OBJC__] EmacsScroller *last_mouse_scroll_bar; [else] void *last_mouse_scroll_bar; };
#[cfg (feature = "have_ns")]
#[repr(C)]
pub struct ns_display_info
{
  /* Chain of all ns_display_info structures.  */
  pub next: *mut ns_display_info,

  /* The generic display parameters corresponding to this NS display.  */
  pub terminal: *mut terminal,

  /* This is a cons cell of the form (NAME . FONT-LIST-CACHE).  */
  pub name_list_element: Lisp_Object,

  /* The number of fonts loaded.  */
  pub n_fonts: c_int,

  /* Minimum width over all characters in all fonts in font_table.  */
  pub smallest_char_width: c_int,

  /* Minimum font height over all fonts in font_table.  */
  pub smallest_font_height: c_int,

  pub bitmaps: *mut ns_bitmap_record,
  pub bitmaps_size: ptrdiff_t,
  pub bitmaps_last: ptrdiff_t,

  /* DPI resolution of this screen.  */
  pub resx: f64,
  pub resy: f64,

  /* Mask of things that cause the mouse to be grabbed.  */
  pub grabbed: c_int,

  pub n_planes: c_int,

  pub color_p: c_int,

  pub root_window: Window,

  /* Xism */
  pub rdb: Lisp_Object,

  /* The cursor to use for vertical scroll bars.  */
  pub vertical_scroll_bar_cursor: Emacs_Cursor,

  /* The cursor to use for horizontal scroll bars.  */
  pub horizontal_scroll_bar_cursor: Emacs_Cursor,

  /* Information about the range of text currently shown in
     mouse-face.  */
  pub mouse_highlight: Mouse_HLInfo,

  pub highlight_frame: *mut frame,
  pub ns_focus_frame: *mut frame,

  /* The frame where the mouse was last time we reported a mouse event.  */
  pub last_mouse_frame: *mut frame,

  /* The frame where the mouse was last time we reported a mouse motion.  */
  pub last_mouse_motion_frame: *mut frame,

  /* Position where the mouse was last time we reported a motion.
     This is a position on last_mouse_motion_frame.  */
  pub last_mouse_motion_x: c_int,
  pub last_mouse_motion_y: c_int,

  /* Where the mouse was last time we reported a mouse position.  */
  pub last_mouse_glyph: NSRect,

  /* Time of last mouse movement.  */
  pub last_mouse_movement_time: Time,

  /* The scroll bar in which the last motion event occurred.  */
  pub last_mouse_scroll_bar: *mut c_void,   // C: `void *last_mouse_scroll_bar;` (the `__OBJC__` arm is `EmacsScroller *`)
}

/* C 931-933: "This is a chain of structures for all the NS displays currently in use."  */
/// C: extern struct ns_display_info *x_display_list;
/// C: extern long context_menu_value;
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub static x_display_list: VmGlobal<*mut ns_display_info>;
  pub static context_menu_value: VmGlobal<c_long>;
}

/* C 935-1023: `struct ns_output`.  The nine leading members have an `#ifdef __OBJC__` pair; C's
   `#else` half (`void *`) is carried and the Objective-C half is recorded in the note.  `Window`
   is `nsgui.h` 67's (`typedef int Window`), `XCharStruct`-free, and the two conditionally present
   members carry the features their C conditions name (`double_buffered` is
   `defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`, `tool_bar_adjusted` is
   `NS_IMPL_GNUSTEP`).  */
/// C: struct ns_output { [__OBJC__] EmacsView *view; id miniimage; NSColor *cursor_color,
/// C: *foreground_color, *background_color, *relief_background_color, *light_relief_color,
/// C: *dark_relief_color; EmacsToolbar *toolbar; [else void * for all nine]; Emacs_Cursor
/// C: text_cursor, nontext_cursor, modeline_cursor, hand_cursor, hourglass_cursor,
/// C: horizontal_drag_cursor, vertical_drag_cursor, left_edge_cursor, top_left_corner_cursor,
/// C: top_edge_cursor, top_right_corner_cursor, right_edge_cursor, bottom_right_corner_cursor,
/// C: bottom_edge_cursor, bottom_left_corner_cursor, current_pointer; Window window_desc,
/// C: parent_desc; char explicit_parent; struct font *font; int baseline_offset, fontset,
/// C: icon_top, icon_left, vertical_scroll_bar_extra, titlebar_height, toolbar_height;
/// C: struct ns_display_info *display_info; int zooming, in_animation;
/// C: [Cocoa >= 101400] bool double_buffered; [GNUstep] int tool_bar_adjusted; };
#[cfg (feature = "have_ns")]
#[repr(C)]
pub struct ns_output
{
  pub view: *mut c_void,                    // C: `void *view;`      (`__OBJC__`: `EmacsView *view;`)
  pub miniimage: *mut c_void,               // C: `void *miniimage;` (`__OBJC__`: `id miniimage;`)
  pub cursor_color: *mut c_void,            // C: `void *cursor_color;`            (`NSColor *`)
  pub foreground_color: *mut c_void,        // C: `void *foreground_color;`        (`NSColor *`)
  pub background_color: *mut c_void,        // C: `void *background_color;`        (`NSColor *`)
  pub relief_background_color: *mut c_void, // C: `void *relief_background_color;` (`NSColor *`)
  pub light_relief_color: *mut c_void,      // C: `void *light_relief_color;`      (`NSColor *`)
  pub dark_relief_color: *mut c_void,       // C: `void *dark_relief_color;`       (`NSColor *`)
  pub toolbar: *mut c_void,                 // C: `void *toolbar;`   (`__OBJC__`: `EmacsToolbar *toolbar;`)

  /* NSCursors are initialized in initFrameFromEmacs.  */
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

  /* NS-specific.  */
  pub current_pointer: Emacs_Cursor,

  /* Lord knows why Emacs needs to know about our Window ids..  */
  pub window_desc: Window,
  pub parent_desc: Window,
  pub explicit_parent: c_char,

  pub font: *mut font,
  pub baseline_offset: c_int,

  /* If a fontset is specified for this frame instead of font, this
     value contains an ID of the fontset, else -1.  */
  pub fontset: c_int, /* only used with font_backend */

  pub icon_top: c_int,
  pub icon_left: c_int,

  /* The size of the extra width currently allotted for vertical
     scroll bars, in pixels.  */
  pub vertical_scroll_bar_extra: c_int,

  /* The height of the titlebar decoration (included in NSWindow's frame).  */
  pub titlebar_height: c_int,

  /* The height of the toolbar if displayed, else 0.  */
  pub toolbar_height: c_int,

  /* This is the Emacs structure for the NS display this frame is on.  */
  pub display_info: *mut ns_display_info,

  /* Non-zero if we are zooming (maximizing) the frame.  */
  pub zooming: c_int,

  /* Non-zero if we are doing an animation, e.g. toggling the tool bar.  */
  pub in_animation: c_int,

  /* Is the frame double buffered?  */
  #[cfg (feature = "ns_impl_cocoa")]
  pub double_buffered: bool,

  /* Zero if this is the first time a toolbar has been updated on this
     frame.  */
  #[cfg (feature = "ns_impl_gnustep")]
  pub tool_bar_adjusted: c_int,
}

/* C 1025-1030: "This dummy declaration needed to support TTYs."  */
/// C: struct x_output { int unused; };
#[cfg (feature = "have_ns")]
#[repr(C)]
pub struct x_output
{
  pub unused: c_int,
}

/* C 1032-1105: the frame accessors.  C writes them as macros over `->` chains; the port keeps
   the names and casts the union member once (the `include/haikuterm.rs` 393-440 shape for the
   same family), because `frame.rs`'s `output_data` member is still `*mut c_void` (P-72 rule 3).
   The helpers they call are the port's own spellings: `FRAME_SCROLL_BAR_COLS`,
   `FRAME_COLUMN_WIDTH`, `FRAME_SCROLL_BAR_LINES`, `FRAME_LINE_HEIGHT`,
   `FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT` and `FACE_FROM_ID_OR_NULL` are functions in this
   tree (`frame.rs`/`dispextern.rs`), `WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT` is a
   `macro_rules!` (`window.rs` 549).  */
/// C: #define FRAME_DISPLAY_INFO(f) ((f)->output_data.ns->display_info)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).display_info) };
}
/// C: #define FRAME_OUTPUT_DATA(f) ((f)->output_data.ns)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { ((*$f).output_data.ns) };
}
/// C: #define FRAME_NS_WINDOW(f) ((f)->output_data.ns->window_desc)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_NS_WINDOW {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).window_desc) };
}
/// C: #define FRAME_NATIVE_WINDOW(f) FRAME_NS_WINDOW (f)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_NATIVE_WINDOW {
  ($f:expr) => { FRAME_NS_WINDOW! ($f) };
}
/// C: #define FRAME_FOREGROUND_COLOR(f) ((f)->output_data.ns->foreground_color)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_FOREGROUND_COLOR {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).foreground_color) };
}
/// C: #define FRAME_BACKGROUND_COLOR(f) ((f)->output_data.ns->background_color)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_BACKGROUND_COLOR {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).background_color) };
}
/// C: #define NS_FACE_FOREGROUND(f) ((f)->foreground)
#[cfg (feature = "have_ns")]
macro_rules! NS_FACE_FOREGROUND {
  ($f:expr) => { ((*$f).foreground) };
}
/// C: #define NS_FACE_BACKGROUND(f) ((f)->background)
#[cfg (feature = "have_ns")]
macro_rules! NS_FACE_BACKGROUND {
  ($f:expr) => { ((*$f).background) };
}
/// C: #define FRAME_DEFAULT_FACE(f) FACE_FROM_ID_OR_NULL (f, DEFAULT_FACE_ID)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_DEFAULT_FACE {
  ($f:expr) => { FACE_FROM_ID_OR_NULL ($f, DEFAULT_FACE_ID) };
}
/// C: #define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_NS_VIEW {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).view) };
}
/// C: #define FRAME_CURSOR_COLOR(f) ((f)->output_data.ns->cursor_color)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_CURSOR_COLOR {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).cursor_color) };
}
/// C: #define FRAME_POINTER_TYPE(f) ((f)->output_data.ns->current_pointer)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_POINTER_TYPE {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).current_pointer) };
}
/// C: #define FRAME_FONT(f) ((f)->output_data.ns->font)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_FONT {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).font) };
}
/// C: #if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400 /
/// C: #define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered) / #endif
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
macro_rules! FRAME_DOUBLE_BUFFERED {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).double_buffered) };
}
/// C: #ifdef __OBJC__ / #define XNS_SCROLL_BAR(vec) ((id) xmint_pointer (vec)) / #else /
/// C: #define XNS_SCROLL_BAR(vec) xmint_pointer (vec) / #endif - C carries the `id` cast in an
/// Objective-C build only.
// C: [__OBJC__] #define XNS_SCROLL_BAR(vec) ((id) xmint_pointer (vec))
#[cfg (feature = "have_ns")]
macro_rules! XNS_SCROLL_BAR {
  ($vec:expr) => { xmint_pointer ($vec) };
}
// C 1062-1070: `FRAME_NS_TITLEBAR_HEIGHT(f)` - "Compute pixel height of the frame's titlebar."
// Its body is an Objective-C message send (`NSHeight([FRAME_NS_VIEW (f) frame])`, with four more
// `[ ... ]` sends and a `styleMask:` keyword argument), which a Rust item cannot express; recorded.
// C: #define FRAME_NS_TITLEBAR_HEIGHT(f) (NSHeight([FRAME_NS_VIEW (f) frame]) == 0 ? 0 : (int)(NSHeight([FRAME_NS_VIEW (f) window].frame) - NSHeight([NSWindow contentRectForFrameRect: [[FRAME_NS_VIEW (f) window] frame] styleMask:[[FRAME_NS_VIEW (f) window] styleMask]])))
// C 1071-1081: `FRAME_TOOLBAR_HEIGHT(f)` - "Compute pixel height of the toolbar."  The same
// Objective-C message-send body; recorded.
// C: #define FRAME_TOOLBAR_HEIGHT(f) (([[FRAME_NS_VIEW (f) window] toolbar] == nil || ! [[FRAME_NS_VIEW (f) window] toolbar].isVisible) ? 0 : (int)(NSHeight([NSWindow contentRectForFrameRect: [[FRAME_NS_VIEW (f) window] frame] styleMask:[[FRAME_NS_VIEW (f) window] styleMask]]) - NSHeight([[[FRAME_NS_VIEW (f) window] contentView] frame])))
/// C: #define NS_SCROLL_BAR_ADJUST(w, f) (WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT (w) ?
/// C: (FRAME_SCROLL_BAR_COLS (f) * FRAME_COLUMN_WIDTH (f) - NS_SCROLL_BAR_WIDTH (f)) : 0)
#[cfg (feature = "have_ns")]
macro_rules! NS_SCROLL_BAR_ADJUST {
  ($w:expr, $f:expr) => {
    {
      if WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT! ($w) != 0
        { FRAME_SCROLL_BAR_COLS ($f) * FRAME_COLUMN_WIDTH ($f) - NS_SCROLL_BAR_WIDTH ($f) }
      else { 0 }
    }
  };
}
/// C: #define NS_SCROLL_BAR_ADJUST_HORIZONTALLY(w, f) (WINDOW_HAS_HORIZONTAL_SCROLL_BARS (w) ?
/// C: (FRAME_SCROLL_BAR_LINES (f) * FRAME_LINE_HEIGHT (f) - NS_SCROLL_BAR_HEIGHT (f)) : 0)
#[cfg (feature = "have_ns")]
macro_rules! NS_SCROLL_BAR_ADJUST_HORIZONTALLY {
  ($w:expr, $f:expr) => {
    {
      if WINDOW_HAS_HORIZONTAL_SCROLL_BARS ($w) != 0
        { FRAME_SCROLL_BAR_LINES ($f) * FRAME_LINE_HEIGHT ($f) - NS_SCROLL_BAR_HEIGHT ($f) }
      else { 0 }
    }
  };
}
/// C: #define FRAME_NS_FONT_TABLE(f) (FRAME_DISPLAY_INFO (f)->font_table)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_NS_FONT_TABLE {
  ($f:expr) => { ((*(FRAME_DISPLAY_INFO! ($f))).font_table) };
}
/// C: #define FRAME_FONTSET(f) ((f)->output_data.ns->fontset)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_FONTSET {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).fontset) };
}
/// C: #define FRAME_BASELINE_OFFSET(f) ((f)->output_data.ns->baseline_offset)
#[cfg (feature = "have_ns")]
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut ns_output)).baseline_offset) };
}
/// C: #define BLACK_PIX_DEFAULT(f) 0x000000
#[cfg (feature = "have_ns")]
macro_rules! BLACK_PIX_DEFAULT { ($f:expr) => { 0x000000 }; }
/// C: #define WHITE_PIX_DEFAULT(f) 0xFFFFFF
#[cfg (feature = "have_ns")]
macro_rules! WHITE_PIX_DEFAULT { ($f:expr) => { 0xFFFFFF }; }
/// C: #define FIRST_CHAR_POSITION(f) (! (FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f)) ? 0 :
/// C: FRAME_SCROLL_BAR_COLS (f))
#[cfg (feature = "have_ns")]
macro_rules! FIRST_CHAR_POSITION {
  ($f:expr) => {
    {
      if FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT ($f) == 0 { 0 }
      else { FRAME_SCROLL_BAR_COLS ($f) }
    }
  };
}

/* C 1108-1262: the entry points, in C's order and grouping ("Implemented in nsterm.m, published
   in or needed from nsfns.m", "Implemented in nsfns.m, published in nsterm.m", "From nsimage.m,
   needed in image.c", "Defined in nsterm.m").  Each `/// C:` line is C's own prototype.  Records
   this mirror cannot name (`struct image`, `struct input_event`, `struct glyph_string`) keep
   P-55's untyped pointer with the prototype in the line above; `fd_set`, `timespec` and
   `sigset_t` are the system mirrors' (`lib/include/`), reached through C 26's `sysselect.h`.  */
/// C 1108: `extern struct ns_display_info *ns_term_init (Lisp_Object display_name);`
/// C 1109: `extern void ns_term_shutdown (int sig);`
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub fn ns_term_init (display_name: Lisp_Object) -> *mut ns_display_info;
  pub fn ns_term_shutdown (sig: c_int);

  /// C 1113-1114: `extern void nsfont_make_fontset_for_font (Lisp_Object name,
  /// Lisp_Object font_object);` - inside `#ifdef NS_IMPL_GNUSTEP` ("In nsfont.m, called from
  /// fontset.c").
  #[cfg (feature = "ns_impl_gnustep")]
  pub fn nsfont_make_fontset_for_font (name: Lisp_Object, font_object: Lisp_Object);
  /// C 1117-1118: `struct glyph_string; void ns_dump_glyphstring (struct glyph_string *s)
  /// EXTERNALLY_VISIBLE;` - the same GNUstep arm; `EXTERNALLY_VISIBLE` is an attribute with no
  /// Rust counterpart (docs/ports/excluded-macros.txt).
  #[cfg (feature = "ns_impl_gnustep")]
  pub fn ns_dump_glyphstring (s: *mut c_void);

  /// C 1122-1123: `extern Lisp_Object ns_list_fonts (struct frame *f, Lisp_Object pattern, int
  /// size, int maxnames);`
  pub fn ns_list_fonts (f: *mut frame, pattern: Lisp_Object, size: c_int,
                        maxnames: c_int) -> Lisp_Object;
  /// C 1124: `extern void ns_clear_frame (struct frame *f);`
  pub fn ns_clear_frame (f: *mut frame);
  /// C 1126-1127: `extern void ns_set_offset (struct frame *f, int xoff, int yoff, int
  /// change_grav);`
  pub fn ns_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_grav: c_int);
  /// C 1129: `extern const char *ns_xlfd_to_fontname (const char *xlfd);`
  pub fn ns_xlfd_to_fontname (xlfd: *const c_char) -> *const c_char;
  /// C 1131: `extern Lisp_Object ns_map_event_to_object (void);`
  pub fn ns_map_event_to_object () -> Lisp_Object;
  /// C 1133-1134: `extern Lisp_Object ns_string_from_pasteboard (id pb); extern void
  /// ns_string_to_pasteboard (id pb, Lisp_Object str);` - inside `#ifdef __OBJC__`, recorded
  /// (their parameter is Objective-C's `id`).
  // C: [__OBJC__] extern Lisp_Object ns_string_from_pasteboard (id pb);
  // C: [__OBJC__] extern void ns_string_to_pasteboard (id pb, Lisp_Object str);
  /// C 1136-1137: `extern Lisp_Object ns_get_local_selection (Lisp_Object selection_name,
  /// Lisp_Object target_type);`
  pub fn ns_get_local_selection (selection_name: Lisp_Object,
                                 target_type: Lisp_Object) -> Lisp_Object;
  /// C 1138: `extern void nxatoms_of_nsselect (void);`
  pub fn nxatoms_of_nsselect ();
  /// C 1139: `extern void ns_set_doc_edited (void);`
  pub fn ns_set_doc_edited ();
  /// C 1141-1145: `extern bool ns_defined_color (struct frame *f, const char *name,
  /// Emacs_Color *color_def, bool alloc, bool makeIndex);` - `Emacs_Color` is
  /// `dispextern.rs`'s record.
  pub fn ns_defined_color (f: *mut frame, name: *const c_char, color_def: *mut Emacs_Color,
                           alloc: bool, makeIndex: bool) -> bool;
  /// C 1148-1149: `extern int ns_lisp_to_color (Lisp_Object color, NSColor **col); extern
  /// const char *ns_get_pending_menu_title (void);` - the first is inside `#ifdef __OBJC__`
  /// (its parameter is AppKit's `NSColor **`), recorded; the second is plain C.
  // C: [__OBJC__] extern int ns_lisp_to_color (Lisp_Object color, NSColor **col);
  pub fn ns_get_pending_menu_title () -> *const c_char;

  /// C 1153: `extern void ns_init_colors (void);`
  pub fn ns_init_colors ();
  /// C 1155: `extern void ns_move_tooltip_to_mouse_location (NSPoint);` - inside
  /// `#ifdef __OBJC__` (AppKit's `NSPoint`), recorded.
  // C: [__OBJC__] extern void ns_move_tooltip_to_mouse_location (NSPoint);
  /// C 1157-1158: `extern void ns_implicitly_set_name (struct frame *f, Lisp_Object arg,
  /// Lisp_Object oldval);`
  pub fn ns_implicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C 1159: `extern void ns_set_scroll_bar_default_width (struct frame *f);`
  pub fn ns_set_scroll_bar_default_width (f: *mut frame);
  /// C 1160: `extern void ns_set_scroll_bar_default_height (struct frame *f);`
  pub fn ns_set_scroll_bar_default_height (f: *mut frame);
  /// C 1161: `extern void ns_change_tab_bar_height (struct frame *f, int height);`
  pub fn ns_change_tab_bar_height (f: *mut frame, height: c_int);
  /// C 1162-1164: `extern const char *ns_get_string_resource (void *_rdb, const char *name,
  /// const char *class);`
  pub fn ns_get_string_resource (_rdb: *mut c_void, name: *const c_char,
                                 class: *const c_char) -> *const c_char;
  /// C 1166-1169: `extern void ns_release_object (void *obj); extern void ns_retain_object
  /// (void *obj); extern void *ns_alloc_autorelease_pool (void); extern void
  /// ns_release_autorelease_pool (void *);`
  pub fn ns_release_object (obj: *mut c_void);
  pub fn ns_retain_object (obj: *mut c_void);
  pub fn ns_alloc_autorelease_pool () -> *mut c_void;
  pub fn ns_release_autorelease_pool (arg: *mut c_void);
  /// C 1170: `extern const char *ns_get_defaults_value (const char *key);`
  pub fn ns_get_defaults_value (key: *const c_char) -> *const c_char;
  /// C 1171: `extern void ns_init_pool (void);`
  pub fn ns_init_pool ();
  /// C 1172: `extern void ns_init_locale (void);`
  pub fn ns_init_locale ();

  /// C 1175: `extern void update_frame_tool_bar (struct frame *f);`  ("C access to ObjC
  /// functionality.")
  pub fn update_frame_tool_bar (f: *mut frame);
  /// C 1177: `extern void update_frame_tool_bar_1 (struct frame *f, EmacsToolbar *toolbar);` -
  /// inside `#ifdef __OBJC__` (the `EmacsToolbar` class of the recorded region), recorded.
  // C: [__OBJC__] extern void update_frame_tool_bar_1 (struct frame *f, EmacsToolbar *toolbar);
  /// C 1180: `extern void free_frame_tool_bar (struct frame *f);`
  pub fn free_frame_tool_bar (f: *mut frame);
  /// C 1181-1183: `extern Lisp_Object find_and_return_menu_selection (struct frame *f, bool
  /// keymaps, void *client_data);`
  /// C 1184-1185: `extern Lisp_Object ns_popup_dialog (struct frame *, Lisp_Object header,
  /// Lisp_Object contents);`
  pub fn ns_popup_dialog (f: *mut frame, header: Lisp_Object, contents: Lisp_Object)
                          -> Lisp_Object;
  /// C 1187: `extern void ns_free_frame_resources (struct frame *);`
  pub fn ns_free_frame_resources (f: *mut frame);
}

/// C 1188/1192: `#define NSAPP_DATA2_RUNASSCRIPT 10` and `#define NSAPP_DATA2_RUNFILEDIALOG 11` -
/// the two values the entries below are called with.  C interleaves them among the prototypes and
/// Rust cannot (an `extern` block holds only `fn`/`static` - measured: `error: extern items
/// cannot be 'const'`), so the block is split at C's own lines rather than moving the constants.
/// C: #define NSAPP_DATA2_RUNASSCRIPT 10
/// C: #define NSAPP_DATA2_RUNFILEDIALOG 11
#[cfg (feature = "have_ns")]
pub const NSAPP_DATA2_RUNASSCRIPT: c_int = 10;
#[cfg (feature = "have_ns")]
pub const NSAPP_DATA2_RUNFILEDIALOG: c_int = 11;

/// C 1190: `extern void ns_run_ascript (void);`
/// C 1193: `extern void ns_run_file_dialog (void);`
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub fn ns_run_ascript ();
  pub fn ns_run_file_dialog ();
  /// C 1195: `extern const char *ns_relocate (const char *epath);`
  pub fn ns_relocate (epath: *const c_char) -> *const c_char;
  /// C 1196-1199: `extern void syms_of_nsterm (void); extern void syms_of_nsfns (void); extern
  /// void syms_of_nsmenu (void); extern void syms_of_nsselect (void);`
  pub fn syms_of_nsterm ();
  pub fn syms_of_nsfns ();
  pub fn syms_of_nsmenu ();
  pub fn syms_of_nsselect ();

  /// C 1203: `extern bool ns_can_use_native_image_api (Lisp_Object type);`  ("From nsimage.m,
  /// needed in image.c.")
  pub fn ns_can_use_native_image_api (type_: Lisp_Object) -> bool;
  /// C 1204-1205: `extern void *ns_image_from_XBM (char *bits, int width, int height, unsigned
  /// long fg, unsigned long bg);`
  pub fn ns_image_from_XBM (bits: *mut c_char, width: c_int, height: c_int, fg: c_ulong,
                            bg: c_ulong) -> *mut c_void;
  /// C 1206: `extern void *ns_image_for_XPM (int width, int height, int depth);`
  pub fn ns_image_for_XPM (width: c_int, height: c_int, depth: c_int) -> *mut c_void;
  /// C 1207: `extern void *ns_image_from_file (Lisp_Object file);`
  pub fn ns_image_from_file (file: Lisp_Object) -> *mut c_void;
  /// C 1208-1209: `extern bool ns_load_image (struct frame *f, struct image *img, Lisp_Object
  /// spec_file, Lisp_Object spec_data);` - `struct image` is image.h's and this mirror includes
  /// nothing of it, so the parameter keeps P-55's untyped pointer.
  pub fn ns_load_image (f: *mut frame, img: *mut c_void, spec_file: Lisp_Object,
                        spec_data: Lisp_Object) -> bool;
  /// C 1210-1213: `extern int ns_image_width (void *img); extern int ns_image_height (void
  /// *img); extern void ns_image_set_size (void *img, int width, int height); extern void
  /// ns_image_set_transform (void *img, double m[3][3]);`
  pub fn ns_image_width (img: *mut c_void) -> c_int;
  pub fn ns_image_height (img: *mut c_void) -> c_int;
  pub fn ns_image_set_size (img: *mut c_void, width: c_int, height: c_int);
  pub fn ns_image_set_transform (img: *mut c_void, m: *mut [f64; 3]);
  /// C 1214: `extern void ns_image_set_smoothing (void *img, bool smooth);`
  pub fn ns_image_set_smoothing (img: *mut c_void, smooth: bool);
  /// C 1215-1218: `extern unsigned long ns_get_pixel (void *img, int x, int y); extern void
  /// ns_put_pixel (void *img, int x, int y, unsigned long argb); extern void ns_set_alpha (void
  /// *img, int x, int y, unsigned char a); extern void *ns_image_reset (void *img, int width,
  /// int height);`
  pub fn ns_get_pixel (img: *mut c_void, x: c_int, y: c_int) -> c_ulong;
  pub fn ns_put_pixel (img: *mut c_void, x: c_int, y: c_int, argb: c_ulong);
  pub fn ns_set_alpha (img: *mut c_void, x: c_int, y: c_int, a: c_uchar);
  pub fn ns_image_reset (img: *mut c_void, width: c_int, height: c_int) -> *mut c_void;

  /// C 1220-1221: `extern int ns_display_pixel_height (struct ns_display_info *); extern int
  /// ns_display_pixel_width (struct ns_display_info *);`
  pub fn ns_display_pixel_height (d: *mut ns_display_info) -> c_int;
  pub fn ns_display_pixel_width (d: *mut ns_display_info) -> c_int;
  /// C 1222: `extern size_t ns_image_size_in_bytes (void *img);`
  pub fn ns_image_size_in_bytes (img: *mut c_void) -> usize;

  /// C 1226-1228: `extern void ns_make_frame_visible (struct frame *f); extern void
  /// ns_make_frame_invisible (struct frame *f); extern void ns_iconify_frame (struct frame *f);`
  /// ("Defined in nsterm.m.")
  pub fn ns_make_frame_visible (f: *mut frame);
  pub fn ns_make_frame_invisible (f: *mut frame);
  pub fn ns_iconify_frame (f: *mut frame);
  /// C 1229-1246: the six `ns_set_*` frame-parameter setters, each `(struct frame *f,
  /// Lisp_Object new_value, Lisp_Object old_value)`; `ns_set_appearance_1` (C 1240) is inside
  /// `#ifdef NS_IMPL_COCOA`.
  /// C: extern void ns_set_undecorated (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  /// C: extern void ns_set_parent_frame (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  /// C: extern void ns_set_no_focus_on_map (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  /// C: extern void ns_set_no_accept_focus (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  /// C: extern void ns_set_z_group (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  pub fn ns_set_undecorated (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn ns_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn ns_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object,
                                 old_value: Lisp_Object);
  pub fn ns_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object,
                                 old_value: Lisp_Object);
  pub fn ns_set_z_group (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 1239-1246: C's `#ifdef NS_IMPL_COCOA` covers all three of these - `ns_set_appearance_1`,
  /// `ns_set_appearance` and `ns_set_transparent_titlebar`.
  /// C: extern void ns_set_appearance_1 (struct frame *f, Lisp_Object value);
  /// C: extern void ns_set_transparent_titlebar (struct frame *f, Lisp_Object new_value, Lisp_Object old_value);
  #[cfg (feature = "ns_impl_cocoa")]
  pub fn ns_set_appearance_1 (f: *mut frame, value: Lisp_Object);
  #[cfg (feature = "ns_impl_cocoa")]
  pub fn ns_set_appearance (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  #[cfg (feature = "ns_impl_cocoa")]
  pub fn ns_set_transparent_titlebar (f: *mut frame, new_value: Lisp_Object,
                                      old_value: Lisp_Object);

  /// C 1247-1249: `extern int ns_select (int nfds, fd_set *readfds, fd_set *writefds, fd_set
  /// *exceptfds, struct timespec *timeout, sigset_t *sigmask);` - `fd_set` is `sysselect.rs`'s
  /// system mirror, `struct timespec` is `lib/include/timespec.rs`'s, `sigset_t` is the system
  /// header's and no mirror in this tree declares it yet.
  pub fn ns_select (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
                    exceptfds: *mut fd_set, timeout: *mut timespec,
                    sigmask: *mut sigset_t) -> c_int;
  /// C 1250-1252: `#ifdef HAVE_PTHREAD / extern void ns_run_loop_break (void); / #endif` -
  /// `HAVE_PTHREAD` is 1 in the frozen configuration
  /// (`docs/baseline/config/darwin-arm64.macros.tsv` 1502), so this region is live text and the
  /// declaration is carried unconditionally (the `include/syssignal.rs` 56 reading of the same
  /// macro).
  pub fn ns_run_loop_break ();
  /// C 1253-1254: `extern unsigned long ns_get_rgb_color (struct frame *f, float r, float g,
  /// float b, float a);`
  pub fn ns_get_rgb_color (f: *mut frame, r: core::ffi::c_float, g: core::ffi::c_float, b: core::ffi::c_float,
                           a: core::ffi::c_float) -> c_ulong;
  /// C 1257: `extern void ns_init_events (struct input_event *);` - `struct input_event` is
  /// termhooks.h's and this mirror does not include it (C 1256 only forward-declares it), so the
  /// parameter keeps P-55's untyped pointer.
  pub fn ns_init_events (event: *mut c_void);
  /// C 1258: `extern void ns_finish_events (void);`
  pub fn ns_finish_events ();
  /// C 1260: `extern double ns_frame_scale_factor (struct frame *);`
  pub fn ns_frame_scale_factor (f: *mut frame) -> f64;

  /// C 1264-1266: `#ifdef NS_IMPL_GNUSTEP / extern char gnustep_base_version[]; / #endif`
  /// ("version tracking").
  #[cfg (feature = "ns_impl_gnustep")]
  pub static gnustep_base_version: VmGlobal<*mut c_char>;

  /// C 1393: `extern void mark_nsterm (void);` - the file's last declaration, after the
  /// GNUstep/old-SDK fallback macros.
  pub fn mark_nsterm ();
}

/* C 1268-1285: the constants and the two bound macros.  `NS_SCROLL_BAR_WIDTH_DEFAULT`,
   `NS_SCROLL_BAR_HEIGHT_DEFAULT` (C 1274-1275, `[EmacsScroller scrollerWidth]`) and the two
   `@"..."` colour strings (C 1278-1279) are Objective-C text and are recorded here; the rest is
   plain C.  */
/// C: #define MINWIDTH 10
/// C: #define MINHEIGHT 10
/// C: #define SCREENMAX 16000
/// C: #define RESIZE_HANDLE_SIZE 12
#[cfg (feature = "have_ns")]
pub const MINWIDTH: c_int = 10;
#[cfg (feature = "have_ns")]
pub const MINHEIGHT: c_int = 10;
#[cfg (feature = "have_ns")]
pub const SCREENMAX: c_int = 16000;
#[cfg (feature = "have_ns")]
pub const RESIZE_HANDLE_SIZE: c_int = 12;
// C: #define NS_SCROLL_BAR_WIDTH_DEFAULT     [EmacsScroller scrollerWidth]
// C: #define NS_SCROLL_BAR_HEIGHT_DEFAULT    [EmacsScroller scrollerHeight]
// C: #define NS_SELECTION_BG_COLOR_DEFAULT	@"LightGoldenrod2";
// C: #define NS_SELECTION_FG_COLOR_DEFAULT	@"Black";
/// C: #define IN_BOUND(min, x, max) (((x) < (min)) ? (min) : ((x) > (max) ? (max) : (x)))
#[cfg (feature = "have_ns")]
macro_rules! IN_BOUND {
  ($min:expr, $x:expr, $max:expr) => {
    {
      if ($x) < ($min) { ($min) }
      else if ($x) > ($max) { ($max) }
      else { ($x) }
    }
  };
}
/// C: #define SCREENMAXBOUND(x) IN_BOUND (-SCREENMAX, x, SCREENMAX)
#[cfg (feature = "have_ns")]
macro_rules! SCREENMAXBOUND {
  ($x:expr) => { IN_BOUND! (-SCREENMAX, $x, SCREENMAX) };
}

/* C 1225: `extern float ns_antialias_threshold;` ("Defined in nsterm.m.") and C 1262:
   `extern frame_parm_handler ns_frame_parm_handlers[];`.  `frame_parm_handler` is
   `dispextern.h` 3063's typedef and this tree's `include/dispextern.rs` does not carry it yet, so
   the array's elements are `*mut c_void` for now - the `include/androidterm.rs` 506-511 reading
   of its own `android_frame_parm_handlers[]`; the commit that lands the alias says
   `[frame_parm_handler; 0]`.  */
/// C: extern float ns_antialias_threshold;
/// C: extern frame_parm_handler ns_frame_parm_handlers[];
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" {
  pub static ns_antialias_threshold: VmGlobal<core::ffi::c_float>;
  pub static ns_frame_parm_handlers: VmGlobal<[*mut c_void; 0]>;
}

/* C 1288-1297: the two AppKit version constants, each behind `#ifndef` because a recent SDK
   defines the name itself (the `include/widget.rs` `XtNfont` reading of an `#ifndef`-guarded
   macro).  AppKit's own constants are `double`s, so the port types them `c_double`; C's literals
   are written as they are.  */
/// C: #ifdef NS_IMPL_COCOA / #ifndef NSAppKitVersionNumber10_7 / #define
/// C: NSAppKitVersionNumber10_7 1138 / #endif / #ifndef NSAppKitVersionNumber10_10 / #define
/// C: NSAppKitVersionNumber10_10 1343 / #endif / #endif /* NS_IMPL_COCOA */
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
pub const NSAppKitVersionNumber10_7: c_double = 1138.;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
pub const NSAppKitVersionNumber10_10: c_double = 1343.;

/* C 1301-1391: the GNUstep / pre-10.x-SDK fallback names.  Each block's C condition is
   `!defined (NS_IMPL_COCOA) || !defined (MAC_OS_X_VERSION_<n>_<m>)`: a Cocoa build against a
   current SDK takes *none* of them, and the GNUstep side takes all of them - which is the
   feature each block carries (the second disjunct, an older Cocoa SDK, is recorded here rather
   than modelled).  The values are AppKit's older spellings, so each is an alias of a platform
   constant (`NSUInteger`/`NSInteger`-typed), carried with C's own name.  */
/// C 1301-1308: `#if !defined (NS_IMPL_COCOA) || !defined (MAC_OS_X_VERSION_10_7)` -
/// `NSFullScreenWindowMask (1 << 14)`, `NSWindowCollectionBehaviorFullScreenPrimary (1 << 7)`,
/// `NSWindowCollectionBehaviorFullScreenAuxiliary (1 << 8)`,
/// `NSApplicationPresentationFullScreen (1 << 10)`,
/// `NSApplicationPresentationAutoHideToolbar (1 << 11)`, `NSAppKitVersionNumber10_7 1138`.
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSFullScreenWindowMask: NSUInteger = 1 << 14;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowCollectionBehaviorFullScreenPrimary: NSUInteger = 1 << 7;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowCollectionBehaviorFullScreenAuxiliary: NSUInteger = 1 << 8;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSApplicationPresentationFullScreen: NSUInteger = 1 << 10;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSApplicationPresentationAutoHideToolbar: NSUInteger = 1 << 11;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSAppKitVersionNumber10_7: c_double = 1138.;

/// C 1311-1348: `#if !defined (NS_IMPL_COCOA) || !defined (MAC_OS_X_VERSION_10_12)` - the
/// `NSEventModifierFlag*`/`NSEventType*`/`NSEventMask*`/`NSWindowStyleMask*`/`NSAlertStyle*`/
/// `NSControlSize*`/`NSCompositingOperation*`/`NSTextAlignment*` names, each an alias of the
/// pre-10.12 AppKit spelling C names in the block.
/// C: #define NSEventModifierFlagCommand NSCommandKeyMask
/// C: #define NSEventModifierFlagControl NSControlKeyMask
/// C: #define NSEventModifierFlagHelp NSHelpKeyMask
/// C: #define NSEventModifierFlagNumericPad NSNumericPadKeyMask
/// C: #define NSEventModifierFlagOption NSAlternateKeyMask
/// C: #define NSEventModifierFlagShift NSShiftKeyMask
/// C: #define NSCompositingOperationSourceOver NSCompositeSourceOver
/// C: #define NSEventMaskApplicationDefined NSApplicationDefinedMask
/// C: #define NSEventTypeApplicationDefined NSApplicationDefined
/// C: #define NSEventTypeCursorUpdate NSCursorUpdate
/// C: #define NSEventTypeMouseMoved NSMouseMoved
/// C: #define NSEventTypeLeftMouseDown NSLeftMouseDown
/// C: #define NSEventTypeRightMouseDown NSRightMouseDown
/// C: #define NSEventTypeOtherMouseDown NSOtherMouseDown
/// C: #define NSEventTypeLeftMouseUp NSLeftMouseUp
/// C: #define NSEventTypeRightMouseUp NSRightMouseUp
/// C: #define NSEventTypeOtherMouseUp NSOtherMouseUp
/// C: #define NSEventTypeLeftMouseDragged NSLeftMouseDragged
/// C: #define NSEventTypeRightMouseDragged NSRightMouseDragged
/// C: #define NSEventTypeOtherMouseDragged NSOtherMouseDragged
/// C: #define NSEventTypeScrollWheel NSScrollWheel
/// C: #define NSEventTypeKeyDown NSKeyDown
/// C: #define NSEventTypeKeyUp NSKeyUp
/// C: #define NSEventTypeFlagsChanged NSFlagsChanged
/// C: #define NSEventMaskAny NSAnyEventMask
/// C: #define NSEventTypeSystemDefined NSSystemDefined
/// C: #define NSWindowStyleMaskBorderless NSBorderlessWindowMask
/// C: #define NSWindowStyleMaskClosable NSClosableWindowMask
/// C: #define NSWindowStyleMaskFullScreen NSFullScreenWindowMask
/// C: #define NSWindowStyleMaskMiniaturizable NSMiniaturizableWindowMask
/// C: #define NSWindowStyleMaskResizable NSResizableWindowMask
/// C: #define NSWindowStyleMaskTitled NSTitledWindowMask
/// C: #define NSWindowStyleMaskUtilityWindow NSUtilityWindowMask
/// C: #define NSAlertStyleCritical NSCriticalAlertStyle
/// C: #define NSControlSizeRegular NSRegularControlSize
/// C: #define NSCompositingOperationCopy NSCompositeCopy
/// C: #define NSTextAlignmentRight NSRightTextAlignment
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagCommand: NSUInteger = NSCommandKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagControl: NSUInteger = NSControlKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagHelp: NSUInteger = NSHelpKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagNumericPad: NSUInteger = NSNumericPadKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagOption: NSUInteger = NSAlternateKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventModifierFlagShift: NSUInteger = NSShiftKeyMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSCompositingOperationSourceOver: NSUInteger = NSCompositeSourceOver;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventMaskApplicationDefined: NSUInteger = NSApplicationDefinedMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeApplicationDefined: NSUInteger = NSApplicationDefined;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeCursorUpdate: NSUInteger = NSCursorUpdate;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeMouseMoved: NSUInteger = NSMouseMoved;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeLeftMouseDown: NSUInteger = NSLeftMouseDown;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeRightMouseDown: NSUInteger = NSRightMouseDown;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeOtherMouseDown: NSUInteger = NSOtherMouseDown;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeLeftMouseUp: NSUInteger = NSLeftMouseUp;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeRightMouseUp: NSUInteger = NSRightMouseUp;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeOtherMouseUp: NSUInteger = NSOtherMouseUp;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeLeftMouseDragged: NSUInteger = NSLeftMouseDragged;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeRightMouseDragged: NSUInteger = NSRightMouseDragged;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeOtherMouseDragged: NSUInteger = NSOtherMouseDragged;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeScrollWheel: NSUInteger = NSScrollWheel;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeKeyDown: NSUInteger = NSKeyDown;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeKeyUp: NSUInteger = NSKeyUp;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeFlagsChanged: NSUInteger = NSFlagsChanged;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventMaskAny: NSUInteger = NSAnyEventMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSEventTypeSystemDefined: NSUInteger = NSSystemDefined;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskBorderless: NSUInteger = NSBorderlessWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskClosable: NSUInteger = NSClosableWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskFullScreen: NSUInteger = NSFullScreenWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskMiniaturizable: NSUInteger = NSMiniaturizableWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskResizable: NSUInteger = NSResizableWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskTitled: NSUInteger = NSTitledWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowStyleMaskUtilityWindow: NSUInteger = NSUtilityWindowMask;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSAlertStyleCritical: NSUInteger = NSCriticalAlertStyle;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSControlSizeRegular: NSUInteger = NSRegularControlSize;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSCompositingOperationCopy: NSUInteger = NSCompositeCopy;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSTextAlignmentRight: NSUInteger = NSRightTextAlignment;

// C 1351-1353: `#ifdef __OBJC__ / typedef NSUInteger NSWindowStyleMask; / #endif` - the
// Objective-C half, recorded.
// C: [__OBJC__] typedef NSUInteger NSWindowStyleMask;

/* C 1356-1361: `enum NSWindowTabbingMode` - "Window tabbing mode enums are new too", inside the
   same `!defined (NS_IMPL_COCOA) || !defined (MAC_OS_X_VERSION_10_12)` block.  */
/// C: enum NSWindowTabbingMode { NSWindowTabbingModeAutomatic, NSWindowTabbingModePreferred,
/// C: NSWindowTabbingModeDisallowed };
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub type NSWindowTabbingMode = c_int;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowTabbingModeAutomatic: NSWindowTabbingMode = 0;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowTabbingModePreferred: NSWindowTabbingMode = 1;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSWindowTabbingModeDisallowed: NSWindowTabbingMode = 2;

/* C 1364-1391: the last three fallback blocks, each `!defined (NS_IMPL_COCOA) ||
   !defined (MAC_OS_X_VERSION_<n>)` and each an alias list C gives for the newer AppKit
   spellings (`NSPasteboardName*`, `NSPasteboardType*`, `NSControlStateValue*`,
   `NSBezelStyleRounded`, `NSButtonTypeMomentaryPushIn`,
   `NSLevelIndicatorStyleContinuousCapacity`).  */
/// C: #define NSPasteboardNameGeneral NSGeneralPboard
/// C: #define NSPasteboardNameDrag NSDragPboard
/// C: #define NSPasteboardTypeString NSStringPboardType
/// C: #define NSPasteboardTypeTabularText NSTabularTextPboardType
/// C: #define NSPasteboardTypeURL NSURLPboardType
/// C: #define NSPasteboardTypeHTML NSHTMLPboardType
/// C: #define NSPasteboardTypePDF NSPDFPboardType
/// C: #define NSPasteboardTypeRTF NSRTFPboardType
/// C: #define NSPasteboardTypeRTFD NSRTFDPboardType
/// C: #define NSPasteboardTypeTIFF NSTIFFPboardType
/// C: #define NSControlStateValueOn NSOnState
/// C: #define NSControlStateValueOff NSOffState
/// C: #define NSBezelStyleRounded NSRoundedBezelStyle
/// C: #define NSButtonTypeMomentaryPushIn NSMomentaryPushInButton
/// C: #define NSLevelIndicatorStyleContinuousCapacity NSContinuousCapacityLevelIndicatorStyle
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardNameGeneral: c_uint = NSGeneralPboard;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardNameDrag: c_uint = NSDragPboard;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeString: c_uint = NSStringPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeTabularText: c_uint = NSTabularTextPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeURL: c_uint = NSURLPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeHTML: c_uint = NSHTMLPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypePDF: c_uint = NSPDFPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeRTF: c_uint = NSRTFPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeRTFD: c_uint = NSRTFDPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSPasteboardTypeTIFF: c_uint = NSTIFFPboardType;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSControlStateValueOn: NSUInteger = NSOnState;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSControlStateValueOff: NSUInteger = NSOffState;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSBezelStyleRounded: NSUInteger = NSRoundedBezelStyle;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSButtonTypeMomentaryPushIn: NSUInteger = NSMomentaryPushInButton;
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_gnustep")]
pub const NSLevelIndicatorStyleContinuousCapacity: NSUInteger =
  NSContinuousCapacityLevelIndicatorStyle;

// C 1392-1395: the file's last declaration (`extern void mark_nsterm (void);`, carried with the
// prototypes above) and `#endif	/* HAVE_NS */` - the end of the guarded body.  This header has
// no include guard of its own: C 29's `#ifdef HAVE_NS` is the outermost guard, and `#ifndef`
// around the whole file does not exist (measured: `rg -n 'ifndef .*NSTERM' emacs/src/nsterm.h`
// finds only the NSTRACE/`NATIVE_OBJC_INSTANCETYPE`/`NSAppKitVersionNumber*` ones inside it).

#[cfg(feature = "have_ns")]
pub mod ns_menu_decls {
  use super::*;
  unsafe extern "C-unwind" {
    pub fn find_and_return_menu_selection(f: *mut frame, keymaps: bool, client_data: *mut c_void) -> Lisp_Object;
  }
}
#[cfg(feature = "have_ns")]
pub use self::ns_menu_decls::*;
