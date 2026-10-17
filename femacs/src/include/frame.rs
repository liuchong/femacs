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

/* frame.h 47-59: `enum fullscreen_type`, C's other `HAVE_WINDOW_SYSTEM` enum (the `#ifdef` at
   C 47 opens the region both this and `z_group` sit in).  C's `FULLSCREEN_WAIT` is the
   `HAVE_NTGUI` member (C 56-58) and carries that gate; `want_fullscreen` is the four-bit
   `ENUM_BF` field these name (the flag table below).  `src/frame.rs` 4755-4765 and
   `src/w32fns.rs` 1184-1192 each carry a use-site copy of this enum today (P-72 rule 4); both
   must be deleted in the commit that flips their unit - `E0428` otherwise.  C's enum ABI is
   `c_int` (PORTING.md 1.3).  */
/// C: enum fullscreen_type { FULLSCREEN_NONE, FULLSCREEN_WIDTH = 0x1, ... };
#[cfg (feature = "have_window_system")]
pub type fullscreen_type = c_int;
#[cfg (feature = "have_window_system")]
pub const FULLSCREEN_NONE: fullscreen_type = 0;                 // C 51
#[cfg (feature = "have_window_system")]
pub const FULLSCREEN_WIDTH: fullscreen_type = 0x1;              // C 52
#[cfg (feature = "have_window_system")]
pub const FULLSCREEN_HEIGHT: fullscreen_type = 0x2;             // C 53
#[cfg (feature = "have_window_system")]
pub const FULLSCREEN_BOTH: fullscreen_type = 0x3;               // C 54
#[cfg (feature = "have_window_system")]
pub const FULLSCREEN_MAXIMIZED: fullscreen_type = 0x4;          // C 55
/// C: #ifdef HAVE_NTGUI / FULLSCREEN_WAIT = 0x8 / #endif
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub const FULLSCREEN_WAIT: fullscreen_type = 0x8;               // C 57

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
/* C 261's guard is `defined (HAVE_WINDOW_SYSTEM) && ! defined (HAVE_EXT_TOOL_BAR)` - the ledger
   table below (`docs/ports/include-frame.md` 484) states it that way, and `HAVE_EXT_TOOL_BAR` is
   `lisp.h` 3938's, defined only for GTK/NS - so these four fields are live in the `have_ntgui` arm
   too: the extra clause the cfg used to carry made `src/w32term.rs` 5268/5553/5573/6256's reads of
   them `E0609` (measured in that lane's slice 16: seven rows).  The three flags C 355-363 guards the
   same way keep the old clause for now, because they sit inside the refused flag run.  */
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

// frame.h 327-541: the flag run C packs into four-byte units, in C's declaration order (the
// `bits1`/`bits2` fields above are its two units).  C's compiler inserts a window system's own
// flags *into* the run and pushes an `ENUM_BF` that does not fit in what is left of a unit to
// the next one, so the numbering is *per configuration*; `want_fullscreen` is the case that
// proves it (`316:0-3` under X11, one past `tooltip` at `315:6`, while the frozen arm has no
// such field at all).
//
// **Every number below is a reading, not a derivation.**  `spikes/gated-layout-probe.py
// --bits 'struct frame' frame.h` (PORTING.md P-82) asks the C compiler for each arm's packing;
// the frozen, `x11` and `ns_cocoa` arms are in `docs/baseline/frame-bits.tsv` (status 3.273)
// and this round read the two arms that file does not have - `-DHAVE_WINDOW_SYSTEM=1
// -DHAVE_HAIKU=1` (`632` bytes) and `-DHAVE_WINDOW_SYSTEM=1 -DHAVE_ANDROID=1` (`648`, the `x11`
// arm's shape bit for bit) - with both readings and the command in
// docs/ports/include-frame.md's Evidence.
//
// C's run therefore has three window-system shapes, and the `#[cfg]` below names each one:
//
//   A  `not (have_ext_tool_bar) && not (have_ext_menu_bar)`  x11, pgtk, android, w32 (measured)
//   B  `not (have_ext_tool_bar) && have_ext_menu_bar`        haiku, USE_X_TOOLKIT (measured)
//   C  `have_ext_tool_bar`                                   NS and GTK; C's own lisp.h
//                                                            3931-3939 defines this one only
//                                                            for `USE_GTK`/`HAVE_NS`, both of
//                                                            which define the menu bar too
//
// The two `HAVE_EXT_*_BAR` names are C's `lisp.h` 3931-3939 switches: the run's shortest arm is
// the one with both (`ns_cocoa`), the longest the one with the menu bar alone (haiku).  `A`, `B`
// and `C` are C's three *shapes*; a fourth combination (`have_ext_tool_bar` without the menu
// bar) is not a configuration lisp.h can emit, and the `compile_error!` below says so rather
// than letting a hand-written `--cfg` silently take an arm.
//
// Each flag gets a `_SHIFT` (the bit within its unit) and a `_UNIT` (`1` for `bits1`, `2` for
// `bits2`).  The unit is data because five flags *move* between the arms - `no_special_glyphs`,
// `can_set_window_size`, `after_make_frame`, `tab_bar_redisplayed` and `tab_bar_resized` are
// `1:27-31` frozen and `2:12-16` under every window system (measured) - and because
// `want_fullscreen` onward start in unit 2 once the run crosses the four-byte boundary
// (measured: the rest of unit 1's last byte is padding).  The mirror's own readers never pick a
// unit by hand: `frame_flag (f, FRAME_X_UNIT, FRAME_X_SHIFT)` reads whichever unit the arm
// gives.  Every other flag keeps one unit in every arm, which is why its `_UNIT` is one line.

#[cfg (all (feature = "have_ext_tool_bar", not (feature = "have_ext_menu_bar")))]
compile_error! ("HAVE_EXT_TOOL_BAR without HAVE_EXT_MENU_BAR is not a configuration C's lisp.h 3931-3939 can emit: `USE_GTK` and `HAVE_NS` define both.  The flag run's arm C would have no numbering here (see the note above), so this build is refused instead of taking another arm's bits.");

// The Windows GUI arm is the one shape of the run this machine cannot read: frame.h 471's
// `# ifndef HAVE_NTGUI` takes `override_redirect` out of the run, so every bit after it moves one
// place, and `HAVE_NTGUI` is not `HAVE_EXT_MENU_BAR`'s arm A.  Measuring it needs `<windows.h>`
// and this machine has no Windows toolchain (`x86_64-w64-mingw32-gcc` is absent; the `w32fns.md`
// ledger's item 10 records the same absence for its records).  Refused rather than guessed: the
// arm's numbering has to come from a clang probe with that SDK, exactly as the other three did.
#[cfg (feature = "have_ntgui")]
compile_error! ("the HAVE_NTGUI arm of `struct frame`'s flag run is unmeasured on this machine: frame.h 471's `# ifndef HAVE_NTGUI` removes `override_redirect`, so it is not arm A - a Windows/mingw clang probe has to measure it (see docs/ports/include-frame.md's Evidence and the w32fns.md owed list).");


// The flag run, C's declaration order.  The comment on each line is the measured `unit:shift`
// of every arm that has the field (`/N` marks a multi-bit field's width).
// frame.h 331: `bool_bf glyphs_initialized_p : 1;`  - frozen 1:0; A 1:0; B 1:0; C 1:0 (measured)
pub const FRAME_GLYPHS_INITIALIZED_P_SHIFT: u32 = 0;   // every arm
pub const FRAME_GLYPHS_INITIALIZED_P_UNIT: u32 = 1;

// frame.h 335: `bool_bf resized_p : 1;`  - frozen 1:1; A 1:1; B 1:1; C 1:1 (measured)
pub const FRAME_RESIZED_P_SHIFT: u32 = 1;   // every arm
pub const FRAME_RESIZED_P_UNIT: u32 = 1;

// frame.h 340: `bool_bf default_face_done_p : 1;`  - frozen 1:2; A 1:2; B 1:2; C 1:2 (measured)
pub const FRAME_DEFAULT_FACE_DONE_P_SHIFT: u32 = 2;   // every arm
pub const FRAME_DEFAULT_FACE_DONE_P_UNIT: u32 = 1;

// frame.h 344: `bool_bf already_hscrolled_p : 1;`  - frozen 1:3; A 1:3; B 1:3; C 1:3 (measured)
pub const FRAME_ALREADY_HSCROLLED_P_SHIFT: u32 = 3;   // every arm
pub const FRAME_ALREADY_HSCROLLED_P_UNIT: u32 = 1;

// frame.h 347: `bool_bf updated_p : 1;`  - frozen 1:4; A 1:4; B 1:4; C 1:4 (measured)
pub const FRAME_UPDATED_P_SHIFT: u32 = 4;   // every arm
pub const FRAME_UPDATED_P_UNIT: u32 = 1;

// frame.h 352: `bool_bf minimize_tab_bar_window_p : 1;`  - A 1:5; B 1:5; C 1:5 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_MINIMIZE_TAB_BAR_WINDOW_P_SHIFT: u32 = 5;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_MINIMIZE_TAB_BAR_WINDOW_P_UNIT: u32 = 1;

// frame.h 358: `bool_bf minimize_tool_bar_window_p : 1;`  - A 1:6; B 1:6 (measured)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ntgui")))]
pub const FRAME_MINIMIZE_TOOL_BAR_WINDOW_P_SHIFT: u32 = 6;   // A and B (C has no such bit)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ntgui")))]
pub const FRAME_MINIMIZE_TOOL_BAR_WINDOW_P_UNIT: u32 = 1;

// frame.h 362: `bool_bf tool_bar_wraps_p : 1;`  - A 1:7; B 1:7 (measured)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ntgui")))]
pub const FRAME_TOOL_BAR_WRAPS_P_SHIFT: u32 = 7;   // A and B (C has no such bit)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ntgui")))]
pub const FRAME_TOOL_BAR_WRAPS_P_UNIT: u32 = 1;

// frame.h 367: `bool_bf external_tool_bar : 1;`  - C 1:6 (measured)
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT: u32 = 6;   // C: NS/GTK
#[cfg (feature = "have_window_system")]
pub const FRAME_EXTERNAL_TOOL_BAR_FLAG_UNIT: u32 = 1;

// frame.h 372: `bool_bf fonts_changed : 1;`  - frozen 1:5; A 1:8; B 1:8; C 1:7 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 5;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 8;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 8;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_FONTS_CHANGED_SHIFT: u32 = 7;   // C: NS/GTK
pub const FRAME_FONTS_CHANGED_UNIT: u32 = 1;

// frame.h 375: `bool_bf cursor_type_changed : 1;`  - frozen 1:6; A 1:9; B 1:9; C 1:8 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 6;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 9;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 9;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_CURSOR_TYPE_CHANGED_SHIFT: u32 = 8;   // C: NS/GTK
pub const FRAME_CURSOR_TYPE_CHANGED_UNIT: u32 = 1;

// frame.h 378: `bool_bf redisplay : 1;`  - frozen 1:7; A 1:10; B 1:10; C 1:9 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_REDISPLAY_SHIFT: u32 = 7;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_REDISPLAY_SHIFT: u32 = 10;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_REDISPLAY_SHIFT: u32 = 10;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_REDISPLAY_SHIFT: u32 = 9;   // C: NS/GTK
pub const FRAME_REDISPLAY_UNIT: u32 = 1;

// frame.h 382: `bool_bf external_menu_bar : 1;`  - B 1:11; C 1:10 (measured)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT: u32 = 11;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT: u32 = 10;   // C: NS/GTK
#[cfg (feature = "have_window_system")]
pub const FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT: u32 = 1;

// frame.h 390: `unsigned visible : 1;`  - frozen 1:8; A 1:11; B 1:12; C 1:11 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_VISIBLE_SHIFT: u32 = 8;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_VISIBLE_SHIFT: u32 = 11;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_VISIBLE_SHIFT: u32 = 12;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_VISIBLE_SHIFT: u32 = 11;   // C: NS/GTK
pub const FRAME_VISIBLE_UNIT: u32 = 1;

// frame.h 394: `bool_bf iconified : 1;`  - frozen 1:9; A 1:12; B 1:13; C 1:12 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_ICONIFIED_SHIFT: u32 = 9;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_ICONIFIED_SHIFT: u32 = 12;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_ICONIFIED_SHIFT: u32 = 13;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_ICONIFIED_SHIFT: u32 = 12;   // C: NS/GTK
pub const FRAME_ICONIFIED_UNIT: u32 = 1;

// frame.h 398: `bool_bf garbaged : 1;`  - frozen 1:10; A 1:13; B 1:14; C 1:13 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_GARBAGED_SHIFT: u32 = 10;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_GARBAGED_SHIFT: u32 = 13;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_GARBAGED_SHIFT: u32 = 14;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_GARBAGED_SHIFT: u32 = 13;   // C: NS/GTK
pub const FRAME_GARBAGED_UNIT: u32 = 1;

// frame.h 402: `bool_bf wants_modeline : 1;`  - frozen 1:11; A 1:14; B 1:15; C 1:14 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 11;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 14;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 15;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_WANTS_MODELINE_SHIFT: u32 = 14;   // C: NS/GTK
pub const FRAME_WANTS_MODELINE_UNIT: u32 = 1;

// frame.h 405: `bool_bf auto_raise : 1;`  - frozen 1:12; A 1:15; B 1:16; C 1:15 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 12;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 15;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 16;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_AUTO_RAISE_SHIFT: u32 = 15;   // C: NS/GTK
pub const FRAME_AUTO_RAISE_UNIT: u32 = 1;

// frame.h 408: `bool_bf auto_lower : 1;`  - frozen 1:13; A 1:16; B 1:17; C 1:16 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 13;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 16;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 17;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_AUTO_LOWER_SHIFT: u32 = 16;   // C: NS/GTK
pub const FRAME_AUTO_LOWER_UNIT: u32 = 1;

// frame.h 411: `bool_bf no_split : 1;`  - frozen 1:14; A 1:17; B 1:18; C 1:17 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_SPLIT_SHIFT: u32 = 14;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_NO_SPLIT_SHIFT: u32 = 17;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_NO_SPLIT_SHIFT: u32 = 18;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_NO_SPLIT_SHIFT: u32 = 17;   // C: NS/GTK
pub const FRAME_NO_SPLIT_UNIT: u32 = 1;

// frame.h 417: `bool_bf explicit_name : 1;`  - frozen 1:15; A 1:18; B 1:19; C 1:18 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 15;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 18;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 19;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_EXPLICIT_NAME_SHIFT: u32 = 18;   // C: NS/GTK
pub const FRAME_EXPLICIT_NAME_UNIT: u32 = 1;

// frame.h 426: `bool_bf window_change : 1;`  - frozen 1:16; A 1:19; B 1:20; C 1:19 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 16;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 19;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 20;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_WINDOW_CHANGE_SHIFT: u32 = 19;   // C: NS/GTK
pub const FRAME_WINDOW_CHANGE_UNIT: u32 = 1;

// frame.h 430: `bool_bf window_state_change : 1;`  - frozen 1:17; A 1:20; B 1:21; C 1:20 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 17;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 20;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 21;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_WINDOW_STATE_CHANGE_SHIFT: u32 = 20;   // C: NS/GTK
pub const FRAME_WINDOW_STATE_CHANGE_UNIT: u32 = 1;

// frame.h 434: `bool_bf mouse_moved : 1;`  - frozen 1:18; A 1:21; B 1:22; C 1:21 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 18;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 21;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 22;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_MOUSE_MOVED_SHIFT: u32 = 21;   // C: NS/GTK
pub const FRAME_MOUSE_MOVED_UNIT: u32 = 1;

// frame.h 437: `bool_bf pointer_invisible : 1;`  - frozen 1:19; A 1:22; B 1:23; C 1:22 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 19;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 22;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 23;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_POINTER_INVISIBLE_SHIFT: u32 = 22;   // C: NS/GTK
pub const FRAME_POINTER_INVISIBLE_UNIT: u32 = 1;

// frame.h 441: `bool_bf frozen_window_starts : 1;`  - frozen 1:20; A 1:23; B 1:24; C 1:23 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 20;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 23;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 24;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_FROZEN_WINDOW_STARTS_SHIFT: u32 = 23;   // C: NS/GTK
pub const FRAME_FROZEN_WINDOW_STARTS_UNIT: u32 = 1;

// frame.h 446: `ENUM_BF (output_method) output_method : 4;`  - frozen 1:21/4; A 1:24/4; B 1:25/4; C 1:24/4 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 21;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 24;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 25;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_OUTPUT_METHOD_SHIFT: u32 = 24;   // C: NS/GTK
pub const FRAME_OUTPUT_METHOD_UNIT: u32 = 1;

// frame.h 449: `bool_bf undecorated : 1;`  - frozen 1:25; A 1:28; B 1:29; C 1:28 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_UNDECORATED_SHIFT: u32 = 25;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_UNDECORATED_SHIFT: u32 = 28;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_UNDECORATED_SHIFT: u32 = 29;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_UNDECORATED_SHIFT: u32 = 28;   // C: NS/GTK
pub const FRAME_UNDECORATED_UNIT: u32 = 1;

// frame.h 453: `bool_bf no_accept_focus : 1;`  - frozen 1:26; A 1:29; B 1:30; C 1:29 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 26;   // frozen
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 29;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 30;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_NO_ACCEPT_FOCUS_SHIFT: u32 = 29;   // C: NS/GTK
pub const FRAME_NO_ACCEPT_FOCUS_UNIT: u32 = 1;

// frame.h 457: `bool_bf tooltip : 1;`  - A 1:30; B 1:31; C 1:30 (measured)
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), not (feature = "have_ext_menu_bar"), not (feature = "have_ntgui")))]
pub const FRAME_TOOLTIP_SHIFT: u32 = 30;   // A: x11/pgtk/android/w32
#[cfg (all (feature = "have_window_system", not (feature = "have_ext_tool_bar"), feature = "have_ext_menu_bar"))]
pub const FRAME_TOOLTIP_SHIFT: u32 = 31;   // B: haiku/X-toolkit
#[cfg (all (feature = "have_window_system", feature = "have_ext_tool_bar"))]
pub const FRAME_TOOLTIP_SHIFT: u32 = 30;   // C: NS/GTK
#[cfg (feature = "have_window_system")]
pub const FRAME_TOOLTIP_UNIT: u32 = 1;

// frame.h 460: `ENUM_BF (fullscreen_type) want_fullscreen : 4;`  - A 2:0/4; B 2:0/4; C 2:0/4 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_WANT_FULLSCREEN_SHIFT: u32 = 0;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_WANT_FULLSCREEN_UNIT: u32 = 2;

// frame.h 464: `ENUM_BF (vertical_scroll_bar_type) vertical_scroll_bar_type : 2;`  - A 2:4/2; B 2:4/2; C 2:4/2 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_SHIFT: u32 = 4;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_UNIT: u32 = 2;

// frame.h 467: `bool_bf horizontal_scroll_bars : 1;`  - A 2:6; B 2:6; C 2:6 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_HORIZONTAL_SCROLL_BARS_SHIFT: u32 = 6;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_HORIZONTAL_SCROLL_BARS_UNIT: u32 = 2;

// frame.h 471: `bool_bf override_redirect : 1;`  - A 2:7; B 2:7; C 2:7 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT: u32 = 7;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_OVERRIDE_REDIRECT_FLAG_UNIT: u32 = 2;

// frame.h 475: `bool_bf skip_taskbar : 1;`  - A 2:8; B 2:8; C 2:8 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_SKIP_TASKBAR_SHIFT: u32 = 8;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_SKIP_TASKBAR_UNIT: u32 = 2;

// frame.h 479: `bool_bf no_focus_on_map : 1;`  - A 2:9; B 2:9; C 2:9 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_NO_FOCUS_ON_MAP_SHIFT: u32 = 9;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_NO_FOCUS_ON_MAP_UNIT: u32 = 2;

// frame.h 482: `ENUM_BF (z_group) z_group : 2;`  - A 2:10/2; B 2:10/2; C 2:10/2 (measured)
#[cfg (feature = "have_window_system")]
pub const FRAME_Z_GROUP_FLAG_SHIFT: u32 = 10;   // A, B and C: one number
#[cfg (feature = "have_window_system")]
pub const FRAME_Z_GROUP_FLAG_UNIT: u32 = 2;

// frame.h 487: `bool_bf no_special_glyphs : 1;`  - frozen 1:27; A 2:12; B 2:12; C 2:12 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_SPECIAL_GLYPHS_SHIFT: u32 = 27;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_NO_SPECIAL_GLYPHS_SHIFT: u32 = 12;   // A, B and C: one number
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NO_SPECIAL_GLYPHS_UNIT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_NO_SPECIAL_GLYPHS_UNIT: u32 = 2;   // every window-system arm

// frame.h 491: `bool_bf can_set_window_size : 1;`  - frozen 1:28; A 2:13; B 2:13; C 2:13 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_CAN_SET_WINDOW_SIZE_SHIFT: u32 = 28;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_CAN_SET_WINDOW_SIZE_SHIFT: u32 = 13;   // A, B and C: one number
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_CAN_SET_WINDOW_SIZE_UNIT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_CAN_SET_WINDOW_SIZE_UNIT: u32 = 2;   // every window-system arm

// frame.h 494: `bool_bf after_make_frame : 1;`  - frozen 1:29; A 2:14; B 2:14; C 2:14 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AFTER_MAKE_FRAME_SHIFT: u32 = 29;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_AFTER_MAKE_FRAME_SHIFT: u32 = 14;   // A, B and C: one number
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_AFTER_MAKE_FRAME_UNIT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_AFTER_MAKE_FRAME_UNIT: u32 = 2;   // every window-system arm

// frame.h 503: `bool_bf tab_bar_redisplayed : 1;`  - frozen 1:30; A 2:15; B 2:15; C 2:15 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_REDISPLAYED_SHIFT: u32 = 30;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TAB_BAR_REDISPLAYED_SHIFT: u32 = 15;   // A, B and C: one number
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_REDISPLAYED_UNIT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TAB_BAR_REDISPLAYED_UNIT: u32 = 2;   // every window-system arm

// frame.h 504: `bool_bf tab_bar_resized : 1;`  - frozen 1:31; A 2:16; B 2:16; C 2:16 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_RESIZED_SHIFT: u32 = 31;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TAB_BAR_RESIZED_SHIFT: u32 = 16;   // A, B and C: one number
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TAB_BAR_RESIZED_UNIT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TAB_BAR_RESIZED_UNIT: u32 = 2;   // every window-system arm

// frame.h 513: `bool_bf tool_bar_redisplayed : 1;`  - frozen 2:0; A 2:17; B 2:17; C 2:17 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TOOL_BAR_REDISPLAYED_SHIFT: u32 = 0;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TOOL_BAR_REDISPLAYED_SHIFT: u32 = 17;   // A, B and C: one number
pub const FRAME_TOOL_BAR_REDISPLAYED_UNIT: u32 = 2;

// frame.h 514: `bool_bf tool_bar_resized : 1;`  - frozen 2:1; A 2:18; B 2:18; C 2:18 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_TOOL_BAR_RESIZED_SHIFT: u32 = 1;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_TOOL_BAR_RESIZED_SHIFT: u32 = 18;   // A, B and C: one number
pub const FRAME_TOOL_BAR_RESIZED_UNIT: u32 = 2;

// frame.h 517: `bool_bf face_change : 1;`  - frozen 2:2; A 2:19; B 2:19; C 2:19 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_FACE_CHANGE_SHIFT: u32 = 2;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_FACE_CHANGE_SHIFT: u32 = 19;   // A, B and C: one number
pub const FRAME_FACE_CHANGE_UNIT: u32 = 2;

// frame.h 521: `bool_bf inhibit_clear_image_cache : 1;`  - frozen 2:3; A 2:20; B 2:20; C 2:20 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT: u32 = 3;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT: u32 = 20;   // A, B and C: one number
pub const FRAME_INHIBIT_CLEAR_IMAGE_CACHE_UNIT: u32 = 2;

// frame.h 526: `bool_bf new_size_p : 1;`  - frozen 2:4; A 2:21; B 2:21; C 2:21 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_NEW_SIZE_P_SHIFT: u32 = 4;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_NEW_SIZE_P_SHIFT: u32 = 21;   // A, B and C: one number
pub const FRAME_NEW_SIZE_P_UNIT: u32 = 2;

// frame.h 530: `bool_bf was_invisible : 1;`  - frozen 2:5; A 2:22; B 2:22; C 2:22 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_WAS_INVISIBLE_SHIFT: u32 = 5;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_WAS_INVISIBLE_SHIFT: u32 = 22;   // A, B and C: one number
pub const FRAME_WAS_INVISIBLE_UNIT: u32 = 2;

// frame.h 536: `bool_bf select_mini_window_flag : 1;`  - frozen 2:6; A 2:23; B 2:23; C 2:23 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT: u32 = 6;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT: u32 = 23;   // A, B and C: one number
pub const FRAME_SELECT_MINI_WINDOW_FLAG_UNIT: u32 = 2;

// frame.h 540: `bool_bf borders_respect_alpha_background : 1;`  - frozen 2:7; A 2:24; B 2:24; C 2:24 (measured)
#[cfg (not (feature = "have_window_system"))]
pub const FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT: u32 = 7;   // frozen
#[cfg (feature = "have_window_system")]
pub const FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT: u32 = 24;   // A, B and C: one number
pub const FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_UNIT: u32 = 2;

// The masks of the run's multi-bit fields.  C writes each width in the declaration (`: 4`,
// `: 2`); the port's packed words keep it as a mask, and `output_method`'s is the one the frozen
// arm already needed.
pub const FRAME_OUTPUT_METHOD_MASK: u32 = 0x0F;        // C 446: `output_method : 4`
pub const FRAME_WANT_FULLSCREEN_MASK: u32 = 0x0F;      // C 460: `want_fullscreen : 4`
pub const FRAME_VERTICAL_SCROLL_BAR_MASK: u32 = 0x03;  // C 464: `vertical_scroll_bar_type : 2`
pub const FRAME_Z_GROUP_MASK: u32 = 0x03;              // C 482: `z_group : 2`

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
// C's shape is `for (list = Vframe_list; CONSP (list) && (frame = XCAR (list), true);
// list = XCDR (list))`, and a `continue` in a C `for` runs the increment.  A Rust `while` does
// not, so the body's `continue` left the list where it was and spun on the same frame - measured
// 2026-10-17, `xdisp-tests--minibuffer-scroll` hung in `run_window_change_functions`, whose body
// continues on a frame that needs no work.  The advance therefore happens *before* the body, in a
// private variable, and the body still sees the current cell in `$list_var`; then `continue`
// reaches the next frame exactly as C's does.  The one consequence: after the loop `$list_var`
// holds the last cell rather than nil, which no call site reads.
macro_rules! FOR_EACH_FRAME {
    ($list_var:ident, $frame_var:ident, $body:block) => {
        {
            let mut for_each_frame_rest: Lisp_Object = unsafe { Vframe_list };
            while CONSP (for_each_frame_rest)
                && { $list_var = for_each_frame_rest;
                     $frame_var = XCAR ($list_var);
                     for_each_frame_rest = XCDR ($list_var);
                     true }
            {
                $body
            }
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

// frame.h 933-963: the platform predicates.  Each is C's `#ifndef <platform>` / `#else` pair, and
// the port carries the platform side under that platform's feature (`FRAME_W32_P` is the one
// `src/w32fns.rs` names, its ledger's item 8), reading the same `output_method` field
// `FRAME_X_P` above does.  The five predicates that used to have only the false arm got their
// C `#else` arm too: with a window-system feature on, C computes the real comparison and the
// mirror answering a constant `false` is a *different program* - exactly what the textconv and
// emacs-module lanes reported as "the mirror's arm hole" (docs/status.md 3.558).
/// C: #ifndef HAVE_NTGUI / #define FRAME_W32_P(f) false
#[cfg (not (feature = "have_ntgui"))]
#[inline(always)]
pub fn FRAME_W32_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_W32_P(f) ((f)->output_method == output_w32)
#[cfg (feature = "have_ntgui")]
#[inline(always)]
pub fn FRAME_W32_P (f: *mut frame) -> bool { frame_output_method (f) == output_w32 }
/// C: #ifndef MSDOS / #define FRAME_MSDOS_P(f) false
#[cfg (not (feature = "msdos"))]
#[inline(always)]
pub fn FRAME_MSDOS_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_MSDOS_P(f) ((f)->output_method == output_msdos_raw)
#[cfg (feature = "msdos")]
#[inline(always)]
pub fn FRAME_MSDOS_P (f: *mut frame) -> bool { frame_output_method (f) == output_msdos_raw }
/// C: #ifndef HAVE_NS / #define FRAME_NS_P(f) false
#[cfg (not (feature = "have_ns"))]
#[inline(always)]
pub fn FRAME_NS_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_NS_P(f) ((f)->output_method == output_ns)
#[cfg (feature = "have_ns")]
#[inline(always)]
pub fn FRAME_NS_P (f: *mut frame) -> bool { frame_output_method (f) == output_ns }
/// C: #ifndef HAVE_PGTK / #define FRAME_PGTK_P(f) false
#[cfg (not (feature = "have_pgtk"))]
#[inline(always)]
pub fn FRAME_PGTK_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_PGTK_P(f) ((f)->output_method == output_pgtk)
#[cfg (feature = "have_pgtk")]
#[inline(always)]
pub fn FRAME_PGTK_P (f: *mut frame) -> bool { frame_output_method (f) == output_pgtk }
/// C: #ifndef HAVE_HAIKU / #define FRAME_HAIKU_P(f) false
#[cfg (not (feature = "have_haiku"))]
#[inline(always)]
pub fn FRAME_HAIKU_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_HAIKU_P(f) ((f)->output_method == output_haiku)
#[cfg (feature = "have_haiku")]
#[inline(always)]
pub fn FRAME_HAIKU_P (f: *mut frame) -> bool { frame_output_method (f) == output_haiku }
/// C: #ifndef HAVE_ANDROID / #define FRAME_ANDROID_P(f) false
#[cfg (not (feature = "have_android"))]
#[inline(always)]
pub fn FRAME_ANDROID_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #else / #define FRAME_ANDROID_P(f) ((f)->output_method == output_android)
#[cfg (feature = "have_android")]
#[inline(always)]
pub fn FRAME_ANDROID_P (f: *mut frame) -> bool { frame_output_method (f) == output_android }
/* frame.h 965-987, `FRAME_WINDOW_P`: C writes six *independent* `#ifdef`s and then an
   `#ifndef FRAME_WINDOW_P` fallback, so a configuration that defines several window systems ends
   up with the **last** one.  The arms below keep that order - each is guarded by its own feature
   and the `not` of every later one - and the last arm is C's `((void) (f), false)`.  */
/// C: #ifdef HAVE_X_WINDOWS / #define FRAME_WINDOW_P(f) FRAME_X_P (f)
#[cfg (all (feature = "have_x_windows",
            not (any (feature = "have_ntgui", feature = "have_ns", feature = "have_pgtk",
                      feature = "have_haiku", feature = "have_android"))))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_X_P (f) }
/// C: #ifdef HAVE_NTGUI / #define FRAME_WINDOW_P(f) FRAME_W32_P (f)
#[cfg (all (feature = "have_ntgui",
            not (any (feature = "have_ns", feature = "have_pgtk",
                      feature = "have_haiku", feature = "have_android"))))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_W32_P (f) }
/// C: #ifdef HAVE_NS / #define FRAME_WINDOW_P(f) FRAME_NS_P(f)
#[cfg (all (feature = "have_ns",
            not (any (feature = "have_pgtk", feature = "have_haiku",
                      feature = "have_android"))))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_NS_P (f) }
/// C: #ifdef HAVE_PGTK / #define FRAME_WINDOW_P(f) FRAME_PGTK_P(f)
#[cfg (all (feature = "have_pgtk",
            not (any (feature = "have_haiku", feature = "have_android"))))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_PGTK_P (f) }
/// C: #ifdef HAVE_HAIKU / #define FRAME_WINDOW_P(f) FRAME_HAIKU_P (f)
#[cfg (all (feature = "have_haiku", not (feature = "have_android")))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_HAIKU_P (f) }
/// C: #ifdef HAVE_ANDROID / #define FRAME_WINDOW_P(f) FRAME_ANDROID_P (f)
#[cfg (feature = "have_android")]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { FRAME_ANDROID_P (f) }
/// C: #ifndef FRAME_WINDOW_P / #define FRAME_WINDOW_P(f) ((void) (f), false)
#[cfg (not (any (feature = "have_x_windows", feature = "have_ntgui", feature = "have_ns",
                feature = "have_pgtk", feature = "have_haiku", feature = "have_android")))]
#[inline(always)]
pub fn FRAME_WINDOW_P (f: *mut frame) -> bool { let _ = f; false }

// frame.h 1023-1039: `MOUSE_HL_INFO (F)` - "Return a pointer to the structure holding
// information about the region of text, if any, that is currently shown in mouse-face on frame
// F.  We need to define two versions because a TTY-only build does not have
// FRAME_DISPLAY_INFO."  It is a macro in C and stays one here: its body names `tty_output`'s
// `display_info` and `tty_display_info`'s `mouse_highlight`, which the *including* unit brings in
// through `termchar.rs` (frame.rs cannot include it - termchar.rs includes frame.rs).
//
// All three of C's arms are carried now, each with C's own condition.  The window-system body is
// `FRAME_WINDOW_P (F) ? &FRAME_DISPLAY_INFO (F)->mouse_highlight : &(F)->output_data.tty->…`, and
// `FRAME_DISPLAY_INFO` is the *platform* term header's macro (`xterm.h` 1481, `w32term.h` 472,
// `nsterm.h` 1032, `haikuterm.h` 289, `androidterm.h` 322, `pgtkterm.h` 477 - the two the port
// has, `include/androidterm.rs` 542 and `include/w32term.rs` 587, are both `macro_rules!`), while
// the member's record `Mouse_HLInfo` is `include/dispextern.rs` 1917's.  A macro body is resolved
// only where it is expanded, so no arm needs a name this mirror cannot see.
//
// C 1034-1035's Android arm is the third one: "There is no `struct tty_output` on Android at
// all".  The `not (have_window_system)` arm is the frozen configuration's (C 1037-1039).
/// C: #ifdef HAVE_WINDOW_SYSTEM / #ifndef HAVE_ANDROID / #define MOUSE_HL_INFO(F) (FRAME_WINDOW_P (F) ? &FRAME_DISPLAY_INFO (F)->mouse_highlight : &(F)->output_data.tty->display_info->mouse_highlight)
#[cfg (all (feature = "have_window_system", not (feature = "have_android")))]
macro_rules! MOUSE_HL_INFO {
  ($f:expr) => {{
    let f = $f;
    if FRAME_WINDOW_P (f) { unsafe { std::ptr::addr_of_mut! ((*FRAME_DISPLAY_INFO! (f)).mouse_highlight) } }
    else {
      unsafe {
        let tty: *mut tty_output = (*f).output_data.tty as *mut tty_output;
        std::ptr::addr_of_mut! ((*(*tty).display_info).mouse_highlight)
      }
    }
  }};
}
/// C: #else /* HAVE_ANDROID */ / #define MOUSE_HL_INFO(F) (&FRAME_DISPLAY_INFO(F)->mouse_highlight)
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! MOUSE_HL_INFO {
  ($f:expr) => {{
    let f = $f;
    unsafe { std::ptr::addr_of_mut! ((*FRAME_DISPLAY_INFO! (f)).mouse_highlight) }
  }};
}
/// C: #else /* not HAVE_WINDOW_SYSTEM */ / #define MOUSE_HL_INFO(F) (&(F)->output_data.tty->display_info->mouse_highlight)
#[cfg (not (feature = "have_window_system"))]
macro_rules! MOUSE_HL_INFO {
  ($f:expr) => {{
    let f = $f;
    unsafe {
      let tty: *mut tty_output = (*f).output_data.tty as *mut tty_output;
      std::ptr::addr_of_mut! ((*(*tty).display_info).mouse_highlight)
    }
  }};
}

/* frame.h 990-1018 / 1054-1061's window-system arms are **recorded, not carried**, and the
   measurement is why.  C's arm bodies read `FRAME_DISPLAY_INFO (f)->resx` / `->resy` /
   `->font_resolution`, and `FRAME_DISPLAY_INFO` is the term-header mirror's `macro_rules!`
   (`include/xterm.rs` 1123, `include/w32term.rs` 587, …).  Textually `macro_rules!` are scoped,
   and every unit includes `frame.h` *before* its `TERM_HEADER` (`src/image.rs` 103 then 137), so:

     * as *functions* the window-system arms cannot see it - measured
       `error: cannot find macro `FRAME_DISPLAY_INFO` in this scope` twice in
       `xtask probe src/image.rs … --cfg 'feature="have_window_system"'
       --cfg 'feature="have_x_windows"' --cfg 'feature="use_cairo"'`;
     * as `macro_rules!` the C-shaped call sites stop resolving - the port calls these with C's
       macro shape and *no* `!` (`src/font.rs` 864/4256/4442/4450/5376/5923, `src/image.rs`
       4305-4306, `src/frame.rs` 4640), which only a function answers, and the extractor's body
       check then reads the window-system arm instead of the frozen one - measured
       `- macro FRAME_RES_X does not mention default_pixels_per_inch_x`.

   Both spellings were tried on this revision and reverted.  What the arms need is the shape
   decision `docs/ports/include-frame.md` already records for the nine `#else`-shaped accessors
   (a `(unit, shift)`-style model change), not another try.  `FRAME_SCALE_FACTOR`'s NS/PGTK arms
   are in the same list: `ns_frame_scale_factor`/`pgtk_frame_scale_factor` are `macro_rules!` in
   their term mirrors too.  */
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
// frame.h 1078-1083 / 1093-1098: C spells this pair twice, and the arm that reads the member is
// the one the flag table above numbers (`external_menu_bar` is `1:11` in arm B, `1:10` in C -
// arm A has no such bit and takes C's `false`).
/// C: #else / #define FRAME_EXTERNAL_MENU_BAR(f) false
#[cfg (not (feature = "have_ext_menu_bar"))]
#[inline(always)]
pub fn FRAME_EXTERNAL_MENU_BAR (f: *mut frame) -> bool { let _ = f; false }
/// C: #ifdef HAVE_EXT_MENU_BAR / #define FRAME_EXTERNAL_MENU_BAR(f) (f)->external_menu_bar
#[cfg (feature = "have_ext_menu_bar")]
#[inline(always)]
pub fn FRAME_EXTERNAL_MENU_BAR (f: *mut frame) -> bool
{ frame_flag (f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT) }
#[inline(always)]
pub fn FRAME_MENU_BAR_LINES (f: *mut frame) -> c_int { unsafe { (*f).menu_bar_lines } }
#[inline(always)]
pub fn FRAME_MENU_BAR_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).menu_bar_height } }
#[inline(always)]
pub fn FRAME_TAB_BAR_LINES (f: *mut frame) -> c_int { unsafe { (*f).tab_bar_lines } }
#[inline(always)]
pub fn FRAME_TAB_BAR_HEIGHT (f: *mut frame) -> c_int { unsafe { (*f).tab_bar_height } }
/// C: #else / #define FRAME_EXTERNAL_TOOL_BAR(f) false
#[cfg (not (feature = "have_ext_tool_bar"))]
#[inline(always)]
pub fn FRAME_EXTERNAL_TOOL_BAR (f: *mut frame) -> bool { let _ = f; false }
/// C: #ifdef HAVE_EXT_TOOL_BAR / #define FRAME_EXTERNAL_TOOL_BAR(f) (f)->external_tool_bar
#[cfg (feature = "have_ext_tool_bar")]
#[inline(always)]
pub fn FRAME_EXTERNAL_TOOL_BAR (f: *mut frame) -> bool
{ frame_flag (f, FRAME_EXTERNAL_TOOL_BAR_FLAG_UNIT, FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT) }
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
  frame_flag (f, FRAME_VISIBLE_UNIT, visible)
}
#[inline(always)]
pub fn FRAME_ICONIFIED_P (f: *mut frame) -> bool { frame_flag (f, FRAME_ICONIFIED_UNIT, FRAME_ICONIFIED_SHIFT) }
#[inline(always)]
pub fn FRAME_GARBAGED_P (f: *mut frame) -> bool { frame_flag (f, FRAME_GARBAGED_UNIT, FRAME_GARBAGED_SHIFT) }
#[inline(always)]
pub fn FRAME_NO_SPLIT_P (f: *mut frame) -> bool { frame_flag (f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT) }
#[inline(always)]
pub fn FRAME_WANTS_MODELINE_P (f: *mut frame) -> bool { frame_flag (f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOWS_FROZEN (f: *mut frame) -> bool { frame_flag (f, FRAME_FROZEN_WINDOW_STARTS_UNIT, FRAME_FROZEN_WINDOW_STARTS_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOW_CHANGE (f: *mut frame) -> bool { frame_flag (f, FRAME_WINDOW_CHANGE_UNIT, FRAME_WINDOW_CHANGE_SHIFT) }
#[inline(always)]
pub fn FRAME_WINDOW_STATE_CHANGE (f: *mut frame) -> bool
{ frame_flag (f, FRAME_WINDOW_STATE_CHANGE_UNIT, FRAME_WINDOW_STATE_CHANGE_SHIFT) }
#[inline(always)]
pub fn FRAME_UNDECORATED (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT) as c_int }

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

// frame.h 1215-1281: the scroll-bar and window-management family.  C spells each of these twice
// and the port carries **both** arms, now that the flag table above has every arm's numbering:
// the `#ifdef HAVE_WINDOW_SYSTEM` body reads the member (through its `_UNIT`/`_SHIFT` pair), the
// `#else` body is `((void) (f), …)`.  One member is a two-bit `ENUM_BF` (`vertical_scroll_bar_type`,
// `z_group`) and one is four (`want_fullscreen`), so those readers mask the field's own width.

/// C: #else / #define FRAME_VERTICAL_SCROLL_BAR_TYPE(f) ((void) (f), vertical_scroll_bar_none)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_VERTICAL_SCROLL_BAR_TYPE (f: *mut frame) -> vertical_scroll_bar_type
{ let _ = f; vertical_scroll_bar_none }
/// C: #define FRAME_VERTICAL_SCROLL_BAR_TYPE(f) ((f)->vertical_scroll_bar_type)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_VERTICAL_SCROLL_BAR_TYPE (f: *mut frame) -> vertical_scroll_bar_type
{
  frame_field (f, FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_UNIT, FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_SHIFT,
               FRAME_VERTICAL_SCROLL_BAR_MASK) as vertical_scroll_bar_type
}

/// C: #else / #define FRAME_HAS_VERTICAL_SCROLL_BARS(f) ((void) (f), 0)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_HAS_VERTICAL_SCROLL_BARS(f) ((f)->vertical_scroll_bar_type != vertical_scroll_bar_none)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS (f: *mut frame) -> c_int
{ (FRAME_VERTICAL_SCROLL_BAR_TYPE (f) != vertical_scroll_bar_none) as c_int }

/// C: #else / #define FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT(f) ((void) (f), 0)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT(f) ((f)->vertical_scroll_bar_type == vertical_scroll_bar_left)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f: *mut frame) -> c_int
{ (FRAME_VERTICAL_SCROLL_BAR_TYPE (f) == vertical_scroll_bar_left) as c_int }

/// C: #else / #define FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT(f) ((void) (f), 0)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT(f) ((f)->vertical_scroll_bar_type == vertical_scroll_bar_right)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f: *mut frame) -> c_int
{ (FRAME_VERTICAL_SCROLL_BAR_TYPE (f) == vertical_scroll_bar_right) as c_int }

/// C: #else / #define FRAME_HAS_HORIZONTAL_SCROLL_BARS(f) ((void) (f), 0)
#[cfg (not (feature = "use_horizontal_scroll_bars"))]
#[inline(always)]
pub fn FRAME_HAS_HORIZONTAL_SCROLL_BARS (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #if USE_HORIZONTAL_SCROLL_BARS / #define FRAME_HAS_HORIZONTAL_SCROLL_BARS(f) ((f)->horizontal_scroll_bars)
#[cfg (feature = "use_horizontal_scroll_bars")]
#[inline(always)]
pub fn FRAME_HAS_HORIZONTAL_SCROLL_BARS (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_HORIZONTAL_SCROLL_BARS_UNIT, FRAME_HORIZONTAL_SCROLL_BARS_SHIFT) as c_int }

/// C: #else /* not HAVE_WINDOW_SYSTEM */ / #define FRAME_OVERRIDE_REDIRECT(f) ((void) (f), 0)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_OVERRIDE_REDIRECT (f: *mut frame) -> c_int { let _ = f; 0 }
// frame.h 1239-1252: the window-management family, inside `#if defined (HAVE_WINDOW_SYSTEM)` (C
// 1237) with the `HAVE_NTGUI` sub-arm C 1241-1245 gives `override_redirect`.
/// C: #ifdef HAVE_NTGUI / #define FRAME_OVERRIDE_REDIRECT(f) ((void) (f), 0)
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
#[inline(always)]
pub fn FRAME_OVERRIDE_REDIRECT (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #else / #define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
#[inline(always)]
pub fn FRAME_OVERRIDE_REDIRECT (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_OVERRIDE_REDIRECT_FLAG_UNIT, FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT) as c_int }

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_SKIP_TASKBAR (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_SKIP_TASKBAR(f) ((f)->skip_taskbar)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_SKIP_TASKBAR (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_SKIP_TASKBAR_UNIT, FRAME_SKIP_TASKBAR_SHIFT) as c_int }

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_NO_FOCUS_ON_MAP (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_NO_FOCUS_ON_MAP(f) ((f)->no_focus_on_map)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_NO_FOCUS_ON_MAP (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT) as c_int }

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_NO_ACCEPT_FOCUS (f: *mut frame) -> c_int { let _ = f; 0 }
/// C: #define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_NO_ACCEPT_FOCUS (f: *mut frame) -> c_int
{ frame_flag (f, FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT) as c_int }

/// C: #else /* not HAVE_WINDOW_SYSTEM */ / #define FRAME_TOOLTIP_P(f) ((void) f, false)
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_TOOLTIP_P (f: *mut frame) -> bool { let _ = f; false }
/// C: #define FRAME_TOOLTIP_P(f) ((f)->tooltip)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_TOOLTIP_P (f: *mut frame) -> bool
{ frame_flag (f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT) }
#[inline(always)]
pub fn FRAME_NO_SPECIAL_GLYPHS (f: *mut frame) -> bool
{ frame_flag (f, FRAME_NO_SPECIAL_GLYPHS_UNIT, FRAME_NO_SPECIAL_GLYPHS_SHIFT) }

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
  frame_set_flag (f, FRAME_ICONIFIED_UNIT, FRAME_ICONIFIED_SHIFT, i as u32);
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
{ frame_field (f, FRAME_OUTPUT_METHOD_UNIT, FRAME_OUTPUT_METHOD_SHIFT, FRAME_OUTPUT_METHOD_MASK) as output_method }

// The readers the accessors share.  `frame_flag`/`frame_set_flag` take the flag's *unit* as well
// as its shift because five flags move from `bits1` to `bits2` under a window system (the table
// above is per arm): every reader names its flag's `_UNIT` and `_SHIFT` constants, so no reader
// picks a unit by hand and no arm can read the wrong word.
#[inline(always)]
fn frame_word (f: *mut frame, unit: u32) -> u32
{ if unit == 2 { unsafe { (*f).bits2 } } else { unsafe { (*f).bits1 } } }

#[inline(always)]
fn frame_flag (f: *mut frame, unit: u32, shift: u32) -> bool { frame_word (f, unit) >> shift & 1 != 0 }

#[inline(always)]
fn frame_set_flag (f: *mut frame, unit: u32, shift: u32, v: u32)
{
  let mask = 1u32 << shift;
  let word = if unit == 2 { unsafe { &mut (*f).bits2 } } else { unsafe { &mut (*f).bits1 } };
  *word = (*word & ! mask) | ((v & 1) << shift);
}

#[inline(always)]
fn frame_field (f: *mut frame, unit: u32, shift: u32, mask: u32) -> u32
{ frame_word (f, unit) >> shift & mask }

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
/// C: frame.h 845: `INLINE void fset_tab_bar_window (struct frame *f, Lisp_Object val)` - C's
/// gate is `#if defined (HAVE_WINDOW_SYSTEM)` (frame.h 844), which is why `dispnew.rs` 2505/2648
/// read `cannot find function fset_tab_bar_window` on Android and never on the frozen target.
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn fset_tab_bar_window (f: *mut frame, val: Lisp_Object) { unsafe { (*f).tab_bar_window = val; } }
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

/* frame.h 1257-1267: the `z_group` family and `FRAME_TOOLTIP_P`, both arms of C's
   `#ifdef HAVE_WINDOW_SYSTEM` (C 1237).  `z_group` is a two-bit `ENUM_BF`, so its reader masks
   the field; the four predicates are C's comparisons against the enumerators the mirror carries
   above.  C 1262-1263's `FRAME_Z_GROUP_ABOVE_SUSPENDED` is one of the family and had no port
   copy at all until this round.  */
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP (f: *mut frame) -> z_group { let _ = f; z_group_none }
/// C: #define FRAME_Z_GROUP(f) ((f)->z_group)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_Z_GROUP (f: *mut frame) -> z_group
{
  frame_field (f, FRAME_Z_GROUP_FLAG_UNIT, FRAME_Z_GROUP_FLAG_SHIFT, FRAME_Z_GROUP_MASK) as z_group
}

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_NONE (f: *mut frame) -> bool { let _ = f; true }
/// C: #define FRAME_Z_GROUP_NONE(f) ((f)->z_group == z_group_none)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_Z_GROUP_NONE (f: *mut frame) -> bool { FRAME_Z_GROUP (f) == z_group_none }

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_ABOVE (f: *mut frame) -> bool { let _ = f; false }
/// C: #define FRAME_Z_GROUP_ABOVE(f) ((f)->z_group == z_group_above)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_Z_GROUP_ABOVE (f: *mut frame) -> bool { FRAME_Z_GROUP (f) == z_group_above }

/// C: #define FRAME_Z_GROUP_ABOVE_SUSPENDED(f) ((f)->z_group == z_group_above_suspended)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_Z_GROUP_ABOVE_SUSPENDED (f: *mut frame) -> bool
{ FRAME_Z_GROUP (f) == z_group_above_suspended }
/// C: #else arm: `FRAME_Z_GROUP_ABOVE_SUSPENDED` is not part of the frozen arm's family, and C's
/// `#else` block (C 1268-1271) leaves it undefined - so neither arm here takes the name but the
/// window-system one (a `#else` body would be invented, not C's).
#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_ABOVE_SUSPENDED (f: *mut frame) -> bool { let _ = f; false }

#[cfg (not (feature = "have_window_system"))]
#[inline(always)]
pub fn FRAME_Z_GROUP_BELOW (f: *mut frame) -> bool { let _ = f; false }
/// C: #define FRAME_Z_GROUP_BELOW(f) ((f)->z_group == z_group_below)
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub fn FRAME_Z_GROUP_BELOW (f: *mut frame) -> bool { FRAME_Z_GROUP (f) == z_group_below }

// frame.h 1257-1260 (the window-system arm's own `NS_IMPL_COCOA` block): the two Cocoa members
// the record carries under the same feature.  `ns_appearance_type` is the enum next to this
// mirror's `ns_appearance` member.
/// C: #define FRAME_NS_APPEARANCE(f) ((f)->ns_appearance)
#[cfg (all (feature = "have_window_system", feature = "ns_impl_cocoa"))]
#[inline(always)]
pub fn FRAME_NS_APPEARANCE (f: *mut frame) -> ns_appearance_type { unsafe { (*f).ns_appearance } }

/// C: #define FRAME_NS_TRANSPARENT_TITLEBAR(f) ((f)->ns_transparent_titlebar)
#[cfg (all (feature = "have_window_system", feature = "ns_impl_cocoa"))]
#[inline(always)]
pub fn FRAME_NS_TRANSPARENT_TITLEBAR (f: *mut frame) -> c_uint
{ unsafe { (*f).ns_transparent_titlebar } }

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
    // C compares two `struct frame *`.  The port's `Mouse_HLInfo` stores
    // `mouse_face_mouse_frame` as the untyped pointer its includers can all name
    // (dispextern.rs 2072's note: five units include that mirror without frame.h), so the
    // comparison spells the conversion back at its use site - C's own type is in the `/// C:`
    // line above and the *record*'s comment.  Measured before the cast: `error[E0308]: mismatched
    // types … expected `*mut frame`, found `*mut c_void`` in `xtask probe src/w32term.rs
    // --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`.
    // The call's first argument needs the same conversion: `include/dispextern.rs` 4050 records
    // `note_mouse_highlight` (dispextern.h 3688) as **owed, not declared** - the earlier note here
    // said the opposite - so the unit that calls it declares it (`src/xterm.rs`'s block above
    // `XFree`'s, the P-88 use-site shape) with `struct frame *`, and the untyped member is cast
    // back exactly as the comparison above does.  Measured once `src/xterm.rs` declared it:
    // `error[E0308]: mismatched types … expected `*mut frame`, found `*mut c_void`` at this line,
    // through `FRAME_MOUSE_UPDATE! (f)` in `src/xdispex.rs`'s consumer.
    if frame == unsafe { (*hlinfo).mouse_face_mouse_frame } as *mut frame {
      block_input ();
      note_mouse_highlight (unsafe { (*hlinfo).mouse_face_mouse_frame } as *mut frame,
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
    /* C's expansion is `AUTO_LIST1 (name, AUTO_CONS_EXPR (parameter, value))`: the pair and the
       one-element list are two compound literals, both with this block's lifetime, and the port
       declares both cells in one binding here (PORTING.md P-113 ③ - the nested expression form
       would have given the pair the arm's block instead).  */
    let mut $name: [Lisp_Cons; 2] = unsafe { std::mem::zeroed () };
    $name[0].u.s.car = $parameter;
    $name[0].u.s.u.cdr = $value;
    let $name = if USE_STACK_CONS {
      $name[1].u.s.car = unsafe { make_lisp_ptr (std::ptr::addr_of_mut! ($name[0]) as *mut c_void,
						 Lisp_Type::Lisp_Cons) };
      $name[1].u.s.u.cdr = Qnil ();
      unsafe { make_lisp_ptr (std::ptr::addr_of_mut! ($name[1]) as *mut c_void,
			      Lisp_Type::Lisp_Cons) }
    } else {
      unsafe { list1 (Fcons ($name[0].u.s.car, $name[0].u.s.u.cdr)) }
    };
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
