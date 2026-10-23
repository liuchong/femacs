// femacs: port of src/haikufns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`.
//
// This file is one of P-86's class: the frozen configuration compiles no `haikufns.o`.  Its
// object variable is `HAIKU_OBJ`, which `configure.ac` 3097 clears and 3114 fills with the Haiku
// platform objects under `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s
// `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
// `docs/baseline/frozen-build-sources.txt` lists no `haikufns.o` and `extract-abi.py`'s checklist
// for the file is empty (P-86 ②).  `docs/ports/haikufns.md` therefore carries the substitute: the
// **C line-range table**, the `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_haiku"`, on every item *and on C 21-42's include
//     graph* (P-86 ⑧) - which is why the frozen configuration compiles nothing here;
//   * the three sub-gates are C 787/789's `USE_BE_CAIRO` (`be_cairo`, the name
//     `xtask/Cargo.toml` registers for it) with its nested `HAVE_HARFBUZZ` (`harfbuzz`),
//     C 1879/1883's `#ifndef USE_BE_CAIRO`, C 3311's `USE_BE_CAIRO`, and C 2404/2638's
//     `ENABLE_CHECKING` (`enable_checking`).  Every `#if` carries **both** arms (P-51/P-86 ①).
//     `tools/check-cond-eval.py src/haikufns.c` reads the file as `9 conditions, 0 unknown, 0
//     disagreements [VERIFIED]`, and all nine take their false arm on this target.
//
// **Acquisition note - this port is `text-only` (P-86 ③⑵).**  This machine has no Haiku SDK and
// no Haiku toolchain (the measurement is in `docs/ports/haikuterm.md` Evidence ②; it is the same
// machine).  So there is **no arm flip on a Haiku target**, and nothing here is compiled or linked
// against a Haiku system.  The `<InterfaceKit.h>`/`<ApplicationKit.h>` surface this file's headers
// pull in is already mirrored by `include/haikugui.rs`/`include/haiku_support.rs` (both landed
// before this unit); the few reference-tree names **no** mirror carries yet are declared at their
// use site below, each with one `/// C:` line and the standing `UNVERIFIED` mark (P-88 ①/②).
//
// `docs/ports/haikufns.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 19: `#include <config.h>` (ungated)

/* C 21-42: the file's own include list.  Every header sits inside the file gate, as C's own build
   does - no translation unit the frozen configuration builds reaches any one of them.  */
#[cfg (feature = "have_haiku")]
include!("../lib/include/math.rs");                   // C 21: `#include <math.h>`
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                          // C 23: `#include "lisp.h"`
/* lisp.h 28: `#include <stdcountof.h>` (P-72 rule 4) - C reaches `countof` (C 95/1710/1725/
   1757/2073/2117) through `lisp.h`'s own chain; the mirror `include/lisp.rs` does not carry that
   include, so the unit names it here (the `src/alloc.rs` 64 shape).  */
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdcountof.rs");             // lisp.h 28: `#include <stdcountof.h>`
#[cfg (feature = "have_haiku")]
include!("include/frame.rs");                         // C 24: `#include "frame.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");                    // C 25: `#include "blockinput.h"`
#[cfg (feature = "have_haiku")]
include!("include/termchar.rs");                      // C 26: `#include "termchar.h"`
#[cfg (feature = "have_haiku")]
include!("include/font.rs");                          // C 27: `#include "font.h"`
#[cfg (feature = "have_haiku")]
include!("include/keyboard.rs");                      // C 28: `#include "keyboard.h"`
#[cfg (feature = "have_haiku")]
include!("include/buffer.rs");                        // C 29: `#include "buffer.h"`
#[cfg (feature = "have_haiku")]
include!("include/dispextern.rs");                    // C 30: `#include "dispextern.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikugui.rs");                      // C 32: `#include "haikugui.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");                     // C 33: `#include "haikuterm.h"`
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");                 // C 34: `#include "haiku_support.h"`
#[cfg (feature = "have_haiku")]
include!("include/termhooks.rs");                     // C 35: `#include "termhooks.h"`
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdlib.rs");                 // C 40: `#include <stdlib.h>`

/* C 37-38: `#include "bitmaps/leftptr.xbm"` / `#include "bitmaps/leftpmsk.xbm"`.  Neither `.xbm`
   has a mirror in this tree (the `bitmaps/` directory of the reference has no ported file), and
   this unit is their only consumer, so P-88 ① declares the two objects here, byte for byte from
   the two reference files.  The `#define`s the `.xbm` files also carry (`left_ptr_width` and its
   three siblings, `left_ptrmsk_width`/`_height`) are read by no statement of this file and are
   recorded in the ledger's Deviations.  */
/* C 37: `#include "bitmaps/leftptr.xbm"` */
#[cfg (feature = "have_haiku")]
static left_ptr_bits: [c_char; 32] =                    // C: leftptr.xbm 4-6
  [
    0x00, 0x00, 0x08, 0x00, 0x18, 0x00, 0x38, 0x00, 0x78, 0x00, 0xf8, 0x00,
    0xf8, 0x01, 0xf8, 0x03, 0xf8, 0x07, 0xf8, 0x00, 0xd8, 0x00, 0x88, 0x01,
    0x80, 0x01, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00,
  ];
/* C 38: `#include "bitmaps/leftpmsk.xbm"` */
#[cfg (feature = "have_haiku")]
static left_ptrmsk_bits: [c_char; 32] =                 // C: leftpmsk.xbm 3-5
  [
    0x0c, 0x00, 0x1c, 0x00, 0x3c, 0x00, 0x7c, 0x00, 0xfc, 0x00, 0xfc, 0x01,
    0xfc, 0x03, 0xfc, 0x07, 0xfc, 0x0f, 0xfc, 0x0f, 0xfc, 0x01, 0xdc, 0x03,
    0xcc, 0x03, 0x80, 0x07, 0x80, 0x07, 0x00, 0x03,
  ];

/* C 42: `#include <kernel/OS.h>`.  `<kernel/OS.h>` is a system header of the Haiku toolchain, not
   part of the reference tree, so it has no mirror (P-88 ①).  The one name this file's live text
   uses from it is `write_port` (C 3086); it is declared here with the SDK's own prototype and the
   standing **UNVERIFIED** mark (no Haiku header on this machine - the `haiku_io.rs` 44-58 shape). */
/// C: `ssize_t write_port (port_id port, int32 code, const void *buffer, ssize_t bufferSize);`
/// [`<kernel/OS.h>`; **UNVERIFIED** here - `port_id`/`int32` are `haiku_support.rs`'s aliases and
/// the widths are the SDK's documented ones.]
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn write_port (port: port_id, code: int32, buffer: *const c_void, buffer_size: ssize_t)
                     -> ssize_t;
}

/* ------------------------------------------------------------------------------------------
   C's own entry points this file calls, from headers whose mirrors do not carry them yet
   (P-72 rule 4; each keeps C's prototype and the header line it was read from - the
   `src/pgtkfns.rs` 296-323 / `src/xfns.rs` shape).  The mirrors `include/frame.rs`,
   `include/dispextern.rs` and `include/window.rs` are WIP slices; these are the declarations
   their ledgers owe, named at the use site for now.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: lisp.h 4409: `void set_frame_cursor_types (struct frame *, Lisp_Object);`
  pub fn set_frame_cursor_types (f: *mut frame, arg: Lisp_Object);
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;


  /* dispextern.h.  */
  /// C: dispextern.h 3573: `int try_window (Lisp_Object, struct text_pos, int);`
  
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, invis: bool);
  /// C: dispextern.h 3794-3795: `void update_face_from_frame_parameter (struct frame *,
  /// Lisp_Object, Lisp_Object);`
  pub fn update_face_from_frame_parameter (f: *mut frame, param: Lisp_Object,
                                           new_value: Lisp_Object);
  /// C: dispextern.h 3821: `void init_frame_faces (struct frame *);`
  pub fn init_frame_faces (f: *mut frame);
  /// C: dispextern.h 3822: `void free_frame_faces (struct frame *);`
  pub fn free_frame_faces (f: *mut frame);
  /// C: dispextern.h 3896: `extern void redraw_frame (struct frame *);`
  pub fn redraw_frame (f: *mut frame);
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
  pub fn adjust_frame_glyphs (f: *mut frame);
  /// C: dispextern.h 3902: `void free_glyphs (struct frame *);`
  pub fn free_glyphs (f: *mut frame);
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);`
  
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);`
  pub fn clear_current_matrices (f: *mut frame);
  /// C: dispextern.h 3918: `void update_single_window (struct window *);`
  pub fn update_single_window (w: *mut window);
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  pub fn clear_frame (f: *mut frame);
  /// C: dispextern.h 3994-3996: `extern Lisp_Object gui_display_get_arg (Display_Info *,
  /// Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);`
  pub fn gui_display_get_arg (dpyinfo: *mut Display_Info, alist: Lisp_Object, item: Lisp_Object,
                              name: *const c_char, class: *const c_char,
                              type_: resource_types) -> Lisp_Object;
  /// C: dispextern.h 4001-4003: `extern Lisp_Object gui_default_parameter (struct frame *,
  /// Lisp_Object, Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);`
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
                                deflt: Lisp_Object, xprop: *const c_char,
                                xclass: *const c_char, type_: resource_types) -> Lisp_Object;

  /* frame.h.  */
  /// C: frame.h 1430: `extern EMACS_UINT frame_set_id_from_params (struct frame *f, Lisp_Object
  /// params);`
  pub fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object) -> EMACS_UINT;
  /// C: frame.h 1475-1476: `extern WINDOW_SYSTEM_RETURN struct frame * decode_window_system_frame
  /// (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;


  /// C: frame.h 1490-1492: `struct frame * make_frame_without_minibuffer (Lisp_Object
  /// mini_window, KBOARD *kb, Lisp_Object display);`
  pub fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut kboard,
                                        display: Lisp_Object) -> *mut frame;




  /* window.h.  */
  /// C: window.h 1139-1140: `void set_window_buffer (Lisp_Object window, Lisp_Object buffer,
  /// bool run_hooks_p, bool keep_margins_p);`
  pub fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object, run_hooks_p: bool,
                            keep_margins_p: bool);
}

/// C: dispextern.h 3983-3992: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT,
/// RES_TYPE_BOOLEAN, RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };`.  The mirror
/// `include/dispextern.rs` does not carry the enum yet, and `src/frame.rs` 6082-6094 carries a
/// use-site copy of its own, so this unit names it here (the `src/pgtkfns.rs` 286-291 shape).
#[cfg (feature = "have_haiku")]
pub type resource_types = c_int;
#[cfg (feature = "have_haiku")]
const RES_TYPE_NUMBER: resource_types = 0;              // C: dispextern.h 3985
#[cfg (feature = "have_haiku")]
const RES_TYPE_BOOLEAN: resource_types = 2;             // C: dispextern.h 3988
#[cfg (feature = "have_haiku")]
const RES_TYPE_STRING: resource_types = 3;              // C: dispextern.h 3988
#[cfg (feature = "have_haiku")]
const RES_TYPE_SYMBOL: resource_types = 4;              // C: dispextern.h 3989

/// C: dispextern.h 3063: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object,
/// Lisp_Object);` - the element type of C 3149-3203's `haiku_frame_parm_handlers[]`.  The mirror
/// does not carry the alias (its `redisplay_interface.frame_parm_handlers` is `*mut c_void`), so
/// the array spells the typedef's own body here - the `src/pgtkfns.rs` 291-296 precedent.
#[cfg (feature = "have_haiku")]
type frame_parm_handler =
  Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object)>;

/// C: frame.h 1844-1871's `gui_set_*` family, the entries C 3149-3203 puts in
/// `haiku_frame_parm_handlers[]`.  They are `frame.c`'s definitions and `frame.h`'s prototypes;
/// `include/frame.rs` does not carry the prototypes, so they are named here - the
/// `src/pgtkfns.rs` 340-380 block, same wording.
#[cfg (feature = "have_haiku")]
unsafe extern "C" {






















}

/* `window.h` 339-351's `bool_bf` bits this file reads and writes (`update_mode_line` at C 1430,
   `pseudo_window_p` at C 2575, `must_be_updated_p` at C 2695).  Each ported unit that touches the
   window bitfield keeps its own copy of the accessor (the `src/dispnew.rs` 396 / `src/window.rs`
   88 shape); the `WINDOW_*_SHIFT` constants are `include/window.rs`'s.  */
#[cfg (feature = "have_haiku")]
macro_rules! window_bits {
  (update_mode_line, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_UPDATE_MODE_LINE_SHIFT) }; };
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
  (must_be_updated_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }; };
}

/* ------------------------------------------------------------------------------------------
   The second half of the use-site declarations of the block above (same device, same reason):
   `frame.h`, `dispextern.h`, `font.h`, `lisp.h` and `<string.h>` names this file calls whose
   mirrors do not carry them.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {


  /// C: frame.h 774-775: `INLINE void fset_parent_frame (struct frame *f, Lisp_Object val)
  /// { f->parent_frame = val; }`
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
  /// C: dispextern.h 3781: `void request_sigio (void);`
  
  /// C: dispextern.h 3782: `void unrequest_sigio (void);`
  
  /// C: dispextern.h 3798-3800: `bool parse_color_spec (const char *,
  /// unsigned short *, unsigned short *, unsigned short *);`
  pub fn parse_color_spec (spec: *const c_char, r: *mut c_ushort, g: *mut c_ushort,
                           b: *mut c_ushort) -> bool;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *, struct frame
  /// *);` - `haikufont_driver` is `haikuterm_objs`' `*mut c_void` stand-in (its ledger owns the
  /// record), so the first parameter is untyped here (the `src/androidfns.rs` 157 shape).
  pub fn register_font_driver (driver: *const c_void, f: *mut frame);
  /// C: lisp.h 4566-4568: `extern Lisp_Object make_specified_string (const char *, ptrdiff_t,
  /// ptrdiff_t, bool);`
  pub fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                                multibyte: bool) -> Lisp_Object;
  /// C: `char *strtok (char *restrict, const char *restrict);` (`<string.h>`; the mirror
  /// `lib/include/string.rs` carries only the gnulib rename, the `src/sfntfont.rs` 461-464
  /// reading of the same gap).  **UNVERIFIED** in the sense that no name here comes from a
  /// reference header - the prototype is the C library's own.
  pub fn strtok (s: *mut c_char, delim: *const c_char) -> *mut c_char;
  /// C: haiku_support.h 620: `extern void *be_create_cursor_from_id (int);`
  pub fn be_create_cursor_from_id (id: c_int) -> *mut c_void;
  /// C: font.h 907: `extern void *ftcrfont_driver;` - the FT/CR font driver's object, which
  /// `ftcrfont.c` defines; declared here because only C 787-792's `USE_BE_CAIRO` arm names it.
  pub static mut ftcrfont_driver: *mut c_void;
  /// C: font.h 908: `extern void *ftcrhbfont_driver;` - same; only C 789-791's nested
  /// `HAVE_HARFBUZZ` arm names it.
  pub static mut ftcrhbfont_driver: *mut c_void;
}

/* `<cairo.h>`'s version macros, read only by C 3311-3321's `USE_BE_CAIRO` arm.  The header is a
   third-party system header with no mirror (P-88 ①) and this unit is its only consumer here.
   The values are the header's own and are **UNVERIFIED** on this machine (no cairo installed). */
/// C: `<cairo.h>`: `#define CAIRO_VERSION_MAJOR 1`
#[cfg (feature = "be_cairo")]
const CAIRO_VERSION_MAJOR: c_int = 1;
/// C: `<cairo.h>`: `#define CAIRO_VERSION_MINOR 18`
#[cfg (feature = "be_cairo")]
const CAIRO_VERSION_MINOR: c_int = 18;
/// C: `<cairo.h>`: `#define CAIRO_VERSION_MICRO 0`
#[cfg (feature = "be_cairo")]
const CAIRO_VERSION_MICRO: c_int = 0;

/* `frame.h`'s accessors and `bool_bf` members C *writes* in this file.  Each mirror accessor is
   a read-only function (or, for the `bool_bf` members, a flag pair), and a Rust function cannot
   be an assignment target, so C's two-argument write arm is carried under the same name - the
   `src/pgtkfns.rs` 436-560 / `src/haikuterm.rs` 626-691 precedent.  The `!` in an invocation is
   the port's spelling of C's assignment (a recorded deviation).  */
/// C: frame.h 1107/1108: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines` /
/// `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height` - written at C 129/130.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).tool_bar_lines = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).tool_bar_height = $v };
}
/// C: frame.h 1090/1091: `#define FRAME_TAB_BAR_LINES(f) (f)->tab_bar_lines` /
/// `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height` - written at C 184/185.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TAB_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).tab_bar_lines = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).tab_bar_height = $v };
}
/// C: frame.h 1086/1087: `#define FRAME_MENU_BAR_LINES(f) (f)->menu_bar_lines` /
/// `#define FRAME_MENU_BAR_HEIGHT(f) (f)->menu_bar_height` - written at C 1426/1427.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_lines = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_height = $v };
}
/// C: frame.h 1078-1083: `#ifdef HAVE_EXT_MENU_BAR` / `#define FRAME_EXTERNAL_MENU_BAR(f)
/// (f)->external_menu_bar` / `#else` / `#define FRAME_EXTERNAL_MENU_BAR(f) false` - written at
/// C 1420/1432.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_EXTERNAL_MENU_BAR {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}
/// C: haikuterm.h 374: `#define FRAME_HAIKU_MENU_BAR(f) FRAME_OUTPUT_DATA (f)->menubar` -
/// written at C 1433, read at C 1421/1672.  The mirror's own one-armed macro is shadowed here so
/// that C's assignment has a target (the `src/haikuterm.rs` 632-652 `FRAME_FONTSET` shape).
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_HAIKU_MENU_BAR {
  ($f:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).menubar };
  ($f:expr, $v:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).menubar = $v };
}
/// C: frame.h 1262-1267: `#define FRAME_Z_GROUP(f) ((f)->z_group)` - written at C 557/562/567.
/// `z_group` is `ENUM_BF (z_group) z_group : 2`, so the write is C's own read-modify-write of the
/// two-bit field (`include/frame.rs` has the reader `FRAME_Z_GROUP` and no two-bit setter - its
/// `frame_set_flag` writes one bit).
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_Z_GROUP {
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(FRAME_Z_GROUP_MASK << FRAME_Z_GROUP_FLAG_SHIFT))
                  | (((($v) as u32) & FRAME_Z_GROUP_MASK) << FRAME_Z_GROUP_FLAG_SHIFT) };
}
/// C: frame.h 1234/1239/1245/1247: `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` /
/// `#define FRAME_BACKGROUND_PIXEL(f) ((f)->background_pixel)` /
/// `#define FRAME_UNDECORATED(f) ((f)->undecorated)` /
/// `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)` - written at C 655 (the two
/// pixels through `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL`), 1381 and 1397.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_FOREGROUND_PIXEL {
  ($f:expr, $v:expr) => { (*($f)).foreground_pixel = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_BACKGROUND_PIXEL {
  ($f:expr, $v:expr) => { (*($f)).background_pixel = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_UNDECORATED {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_OVERRIDE_REDIRECT {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_OVERRIDE_REDIRECT_FLAG_UNIT, FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT,
                    ($v) as u32) };
}
/// C: frame.h 1122/1124: `#define FRAME_NO_FOCUS_ON_MAP(f) ((f)->no_focus_on_map)` /
/// `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)` - written at C 238 and 596.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_NO_FOCUS_ON_MAP {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_NO_ACCEPT_FOCUS {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT, ($v) as u32) };
}
/// C: frame.h 1035/1036: `#define FRAME_TOTAL_COLS(f) ((f)->total_cols)` /
/// `#define FRAME_TOTAL_LINES(f) ((f)->total_lines)` - `FRAME_TOTAL_COLS` is written at C 2617.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TOTAL_COLS {
  ($f:expr, $v:expr) => { (*($f)).total_cols = $v };
}
/// C: frame.h 1119/1120: `#define FRAME_PIXEL_WIDTH(f) ((f)->pixel_width)` /
/// `#define FRAME_PIXEL_HEIGHT(f) ((f)->pixel_height)` - written at C 2678/2679.
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_PIXEL_WIDTH {
  ($f:expr, $v:expr) => { (*($f)).pixel_width = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_PIXEL_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).pixel_height = $v };
}

/* `frame.h` 503/514/499/500/509/510/546/551/597/598/606's `bool_bf` members this file reads and
   writes.  Each is a flag pair in `include/frame.rs`; the per-unit member macro is the
   `src/pgtkfns.rs` 436-462 `explicit_name`/`tab_bar_resized`/`tool_bar_resized` shape.  */
/// C: frame.h 505-512: `bool_bf explicit_name : 1;` - read at C 1847/1852, written at
/// C 778/783/1069/1073/1850.
#[cfg (feature = "have_haiku")]
macro_rules! explicit_name {
  ($f:expr) => { frame_flag ($f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT, ($v) as u32) };
}
/// C: frame.h 496: `bool_bf no_split : 1;` - written at C 885/1223.
#[cfg (feature = "have_haiku")]
macro_rules! no_split {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT, ($v) as u32) };
}
/// C: frame.h 517: `bool_bf can_set_window_size : 1;` - written at C 953/1233.
#[cfg (feature = "have_haiku")]
macro_rules! can_set_window_size {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT,
                    ($v) as u32) };
}
/// C: frame.h 566: `bool_bf was_invisible : 1;` - written at C 970.
#[cfg (feature = "have_haiku")]
macro_rules! was_invisible {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_WAS_INVISIBLE_UNIT, FRAME_WAS_INVISIBLE_SHIFT, ($v) as u32) };
}
/// C: frame.h 553: `bool_bf tooltip : 1;` - written at C 1060.
#[cfg (feature = "have_haiku")]
macro_rules! tooltip {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT, ($v) as u32) };
}
/// C: frame.h 537: `bool_bf wants_modeline : 1;` - written at C 1045.
#[cfg (feature = "have_haiku")]
macro_rules! wants_modeline {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT, ($v) as u32) };
}
/// C: frame.h 503: `bool_bf tab_bar_redisplayed : 1;` - read at C 217.
#[cfg (feature = "have_haiku")]
macro_rules! tab_bar_redisplayed {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_REDISPLAYED_UNIT, FRAME_TAB_BAR_REDISPLAYED_SHIFT) };
}
/// C: frame.h 500: `bool_bf tab_bar_resized : 1;` - read at C 205, written at C 217.
#[cfg (feature = "have_haiku")]
macro_rules! tab_bar_resized {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT, ($v) as u32) };
}
/// C: frame.h 510: `bool_bf tool_bar_redisplayed : 1;` - read at C 153.
#[cfg (feature = "have_haiku")]
macro_rules! tool_bar_redisplayed {
  ($f:expr) => {
    frame_flag ($f, FRAME_TOOL_BAR_REDISPLAYED_UNIT, FRAME_TOOL_BAR_REDISPLAYED_SHIFT) };
}
/// C: frame.h 514: `bool_bf tool_bar_resized : 1;` - read at C 143, written at C 153.
#[cfg (feature = "have_haiku")]
macro_rules! tool_bar_resized {
  ($f:expr) => { frame_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT, ($v) as u32) };
}

/* C 44-48: the four colour macros.  C's `RGB_TO_ULONG` body ends in a `;`, which this port keeps
   (the `src/androidterm.rs` 663-676 spelling of the same four).  */
#[cfg (feature = "have_haiku")]
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
#[cfg (feature = "have_haiku")]
macro_rules! RED_FROM_ULONG {
  ($color:expr) => { ((($color) >> 16) & 0xff) };
}
#[cfg (feature = "have_haiku")]
macro_rules! GREEN_FROM_ULONG {
  ($color:expr) => { ((($color) >> 8) & 0xff) };
}
#[cfg (feature = "have_haiku")]
macro_rules! BLUE_FROM_ULONG {
  ($color:expr) => { (($color) & 0xff) };
}

/* The frame of the currently visible tooltip.  */
// C 51: `Lisp_Object tip_frame;` - the definition `include/haikuterm.rs`'s `haikuterm_objs`
// declares (PORTING.md 0.1-10/P-21).
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut tip_frame: Lisp_Object = std::ptr::null_mut ();

/* The X and Y deltas of the last call to `x-show-tip'.  */
// C 54: `Lisp_Object tip_dx, tip_dy;`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut tip_dx: Lisp_Object = std::ptr::null_mut ();
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut tip_dy: Lisp_Object = std::ptr::null_mut ();

/* A timer that hides or deletes the currently visible tooltip when it
   fires.  */
// C 58: `static Lisp_Object tip_timer;` (C's tentative definition, i.e. zero)
#[cfg (feature = "have_haiku")]
static mut tip_timer: Lisp_Object = std::ptr::null_mut ();

/* STRING argument of last `x-show-tip' call.  */
// C 61: `static Lisp_Object tip_last_string;` (tentative)
#[cfg (feature = "have_haiku")]
static mut tip_last_string: Lisp_Object = std::ptr::null_mut ();

/* Normalized FRAME argument of last `x-show-tip' call.  */
// C 64: `static Lisp_Object tip_last_frame;` (tentative)
#[cfg (feature = "have_haiku")]
static mut tip_last_frame: Lisp_Object = std::ptr::null_mut ();

/* PARMS argument of last `x-show-tip' call.  */
// C 67: `static Lisp_Object tip_last_parms;` (tentative)
#[cfg (feature = "have_haiku")]
static mut tip_last_parms: Lisp_Object = std::ptr::null_mut ();

/* C 69-70's two `static` forward declarations (`haiku_explicitly_set_name`, `haiku_set_title`)
   have no Rust counterpart (PORTING.md 0.1-12): each definition stands at C's own line below.  */

// C 73-102.
#[cfg (feature = "have_haiku")]
unsafe fn get_geometry_from_preferences (dpyinfo: *mut haiku_display_info,
                                        mut parms: Lisp_Object) -> Lisp_Object
{
  /* C's local record is anonymous; the port names it after C's own variable (`r`), and Rust's
     separate type/value namespaces let the two names coexist (a recorded deviation).  */
  #[repr(C)]
  struct r
  {
    val: *const c_char,
    cls: *const c_char,
    tem: Lisp_Object,
  }

  let r: [r; 4] = [
    r { val: c"width".as_ptr (),  cls: c"Width".as_ptr (),  tem: Qwidth () },
    r { val: c"height".as_ptr (), cls: c"Height".as_ptr (), tem: Qheight () },
    r { val: c"left".as_ptr (),   cls: c"Left".as_ptr (),   tem: Qleft () },
    r { val: c"top".as_ptr (),    cls: c"Top".as_ptr (),    tem: Qtop () },
  ];

  let mut i: c_int = 0;
  while i < countof! (r) as c_int
    {
      if NILP (Fassq (r[i as usize].tem, parms))
        {
          let value: Lisp_Object
            = gui_display_get_arg (dpyinfo, parms, r[i as usize].tem, r[i as usize].val,
                                   r[i as usize].cls, RES_TYPE_NUMBER);
          if !BASE_EQ (value, Qunbound ())
            { parms = Fcons (Fcons (r[i as usize].tem, value), parms); }
        }
      i += 1;
    }

  return parms;
}

/* Update the left and top offsets of F after its decorators
   change.  */
// C 104-118.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_update_after_decoration_change (f: *mut frame)
{
  /* Don't reset offsets during initial frame creation, since the
     contents of f->left_pos and f->top_pos won't be applied to the
     window until `x-create-frame' finishes, so setting them here will
     overwrite the offsets that the window should be moved to.  */

  if !(*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).configury_done
    { return; }

  be_send_move_frame_event (FRAME_HAIKU_WINDOW! (f));
}

// C 120-167.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_change_tool_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TOOL_BAR_HEIGHT (f);
  let lines: c_int = (height + unit - 1) / unit;
  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  FRAME_TOOL_BAR_HEIGHT! (f, height);
  FRAME_TOOL_BAR_LINES! (f, lines);
  store_frame_param (f, Qtool_bar_lines (), make_fixnum (lines as EMACS_INT));

  if !FRAME_HAIKU_WINDOW! (f).is_null () && FRAME_TOOL_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tool_bar_window)).current_matrix as *mut glyph_matrix); }

  if !tool_bar_resized! (f)
    {
      /* As long as tool_bar_resized is false, effectively try to change
	 F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
	{ adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f), 1, false,
			     Qtool_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtool_bar_lines ()); }

      tool_bar_resized! (f, tool_bar_redisplayed! (f));
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);

  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    { haiku_clear_under_internal_border (f); }
}

// C 169-230.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_change_tab_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TAB_BAR_HEIGHT (f);

  /* This differs from the tool bar code in that the tab bar height is
     not rounded up.  Otherwise, if redisplay_tab_bar decides to grow
     the tab bar by even 1 pixel, FRAME_TAB_BAR_LINES will be changed,
     leading to the tab bar height being incorrectly set upon the next
     call to x_set_font.  (bug#59285) */
  let mut lines: c_int = height / unit;

  /* Even so, HEIGHT might be less than unit if the tab bar face is
     not so tall as the frame's font height; which if true lines will
     be set to 0 and the tab bar will thus vanish.  */

  if lines == 0 && height != 0
    { lines = 1; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  /* Recalculate tab bar and frame text sizes.  */
  FRAME_TAB_BAR_HEIGHT! (f, height);
  FRAME_TAB_BAR_LINES! (f, lines);
  store_frame_param (f, Qtab_bar_lines (), make_fixnum (lines as EMACS_INT));

  if !FRAME_HAIKU_WINDOW! (f).is_null () && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).current_matrix as *mut glyph_matrix); }

  if !tab_bar_resized! (f)
    {
      let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

      /* As long as tab_bar_resized is false, effectively try to change
	 F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
	{ adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f), 1, false,
			     Qtab_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtab_bar_lines ()); }

      tab_bar_resized! (f, tab_bar_redisplayed! (f));
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtab_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    { haiku_clear_under_internal_border (f); }
}

// C 232-239.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_no_focus_on_map (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  if !EQ (value, oldval)
    { FRAME_NO_FOCUS_ON_MAP! (f, !NILP (value)); }
}

// C 240-259.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  if FRAME_TOOLTIP_P (f)
    { return; }
  let nlines: c_int;

  /* Treat tool bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  haiku_change_tool_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

// C 260-285.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_tool_bar_position (f: *mut frame, new_value: Lisp_Object,
                                       old_value: Lisp_Object)
{
  if !EQ (new_value, Qtop ()) && !EQ (new_value, Qbottom ())
    { error (c"Tool bar position must be either `top' or `bottom'".as_ptr ()); }

  if EQ (new_value, old_value)
    { return; }

  /* Set the tool bar position.  */
  fset_tool_bar_position (f, new_value);

  /* Now reconfigure frame glyphs to place the tool bar at the bottom.
     While the inner height has not changed, call
     `resize_frame_windows' to place each of the windows at its new
     position.  */

  adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_position ());
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);

  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    { haiku_clear_under_internal_border (f); }
}

// C 287-307.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  if FRAME_TOOLTIP_P (f)
    { return; }
  let olines: c_int = FRAME_TAB_BAR_LINES (f);
  let nlines: c_int;

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  if nlines != olines && (olines == 0 || nlines == 0)
    { haiku_change_tab_bar_height (f, nlines * FRAME_LINE_HEIGHT (f)); }
}

/* `termhooks.h` 961's `decode_live_terminal` (C 301) - the mirror's ledger names the ten
   prototypes it defers, so this unit declares the one it calls (P-72 rule 4).  */
/// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn decode_live_terminal (object: Lisp_Object) -> *mut terminal;
  /* `haiku_support.h`'s and `haikuterm.h`'s prototypes whose *mirrored* copies carry no return
     type (`include/haiku_support.rs` 686-704 declares eight of them as `pub fn name ();`) are
     declared in the unit-local module below and imported by name: a crate-root re-declaration
     would be what `check-mirror-dupes.py` reports as a copy of the mirror's own declaration
     (P-26/P-39), and the mirror's signature cannot be used as written.  The ledger's `## Owed`
     names the mirror round that owes the eight corrected prototypes.  */
  /// C: dispextern.h 2068: `extern bool face_change;` - declared inside the mirror's
  /// `dispextern_unit_names` module and reached through the `use` below.
  pub static mut face_change: bool;
  /// C: lisp.h 3148: `extern AVOID quit (void);` - the mirror does not carry it, so the unit
  /// names it here (C 2942's `quit ()`; the `src/alloc.rs` 6230-6231 shape).
  pub fn quit () -> !;

  /// C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
  /// - the mirror `include/lisp.rs` carries only `fatal_error_in_progress`, so the unit names it
  /// here (the `src/alloc.rs` 6230-6231 shape).
  pub fn fatal (msg: *const c_char, ...) -> !;
  /// C: dispextern.h 104-108: `INLINE int xstrcasecmp (char const *a, char const *b)
  /// { return c_strcasecmp (a, b); }`.  The copy in `include/dispextern.rs` sits in the
  /// `dispextern_externs` module (not glob-imported, because `src/font.rs` defines its own), so
  /// this unit names it here - the `src/fontset.rs` 108 shape.
  pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int;
}

/* `include/dispextern.rs` 2759's `dispextern_unit_names` is never glob-imported by the mirror
   (its own note says a unit takes it), so the names this file takes from it - `reset_mouse_highlight`
   (C 1691) and the text-position accessors C 2590 expands - are reachable only through this edge
   (the `src/msdos.rs` 524-531 shape; the ledger's Owed asks the mirror round to decide between the
   mirror's own glob and this per-consumer edge).  */
#[cfg (feature = "have_haiku")]
use dispextern_unit_names::*;

// C 309-324.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gamma_correct (f: *mut frame, color: *mut Emacs_Color)
{
  if (*f).gamma != 0.0
    {
      (*color).red = (pow ((*color).red as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
                     as c_ushort;
      (*color).green = (pow ((*color).green as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
                       as c_ushort;
      (*color).blue = (pow ((*color).blue as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
                      as c_ushort;
      (*color).pixel = RGB_TO_ULONG! ((((*color).red / 256) as c_ulong),
                                      (((*color).green / 256) as c_ulong),
                                      (((*color).blue / 256) as c_ulong));
    }
}

// C 326-392.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_get_color (name: *const c_char, color: *mut Emacs_Color) -> c_int
{
  let mut r16: c_ushort = 0;
  let mut g16: c_ushort = 0;
  let mut b16: c_ushort = 0;
  let mut tem: Lisp_Object;
  let mut col: Lisp_Object;
  let mut clr: int32 = 0;
  let mut rc: int32;
  let mut ui_color: uint32_t = 0;
  let mut size: ptrdiff_t;
  let mut i: ptrdiff_t;
  let mut string: Lisp_Object;

  if parse_color_spec (name, &raw mut r16, &raw mut g16, &raw mut b16)
    {
      (*color).pixel = RGB_TO_ULONG! (((r16 / 256) as c_ulong), ((g16 / 256) as c_ulong),
                                      ((b16 / 256) as c_ulong));
      (*color).red = r16;
      (*color).green = g16;
      (*color).blue = b16;
      return 0;
    }
  else
    {
      block_input ();
      eassert! (!x_display_list.is_null () && !NILP ((*x_display_list).color_map));
      tem = (*x_display_list).color_map;
      while CONSP (tem)
	{
	  col = XCAR (tem);

	  if CONSP (col) && xstrcasecmp (SSDATA (XCAR (col)), name) == 0
	    {
	      clr = XFIXNUM (XCDR (col)) as int32;
	      (*color).pixel = clr as c_ulong;
	      (*color).red = (RED_FROM_ULONG! (clr) * 257) as c_ushort;
	      (*color).green = (GREEN_FROM_ULONG! (clr) * 257) as c_ushort;
	      (*color).blue = (BLUE_FROM_ULONG! (clr) * 257) as c_ushort;
	      unblock_input ();
	      return 0;
	  }

	  tem = XCDR (tem);
	}
      unblock_input ();
    }

  rc = 1;
  if VECTORP (Vhaiku_allowed_ui_colors)
    {
      size = ASIZE (Vhaiku_allowed_ui_colors);

      i = 0;
      while i < size
	{
	  string = AREF (Vhaiku_allowed_ui_colors, i);

	  block_input ();
	  if STRINGP (string) && strcmp (SSDATA (string), name) == 0
	    { rc = be_get_ui_color (name, &raw mut ui_color); }
	  unblock_input ();
	  i += 1;
	}
    }

  if rc == 0
    {
      (*color).pixel = ui_color as c_ulong;
      (*color).red = (RED_FROM_ULONG! (ui_color) * 257) as c_ushort;
      (*color).green = (GREEN_FROM_ULONG! (ui_color) * 257) as c_ushort;
      (*color).blue = (BLUE_FROM_ULONG! (ui_color) * 257) as c_ushort;
    }

  return rc;
}

// C 394-408.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_display_info_for_name (name: Lisp_Object) -> *mut haiku_display_info
{
  CHECK_STRING (name);

  if strcmp (SSDATA (name), c"be".as_ptr ()) == 0
    {
      if !x_display_list.is_null ()
	{ return x_display_list; }

      return haiku_term_init ();
    }

  error (c"Haiku displays can only be named \"be\"".as_ptr ());
}

// C 410-444.
#[cfg (feature = "have_haiku")]
unsafe fn check_haiku_display_info (object: Lisp_Object) -> *mut haiku_display_info
{
  let mut dpyinfo: *mut haiku_display_info = std::ptr::null_mut ();

  if NILP (object)
    {
      let sf: *mut frame = XFRAME (selected_frame);

      if FRAME_HAIKU_P (sf) && FRAME_LIVE_P (sf)
	{ dpyinfo = FRAME_DISPLAY_INFO! (sf); }
      else if !x_display_list.is_null ()
	{ dpyinfo = x_display_list; }
      else
	{ error (c"Haiku windowing not present".as_ptr ()); }
    }
  else if TERMINALP (object)
    {
      let t: *mut terminal = decode_live_terminal (object);

      if (*t).r#type != output_haiku
	{ error (c"Terminal %d is not a Haiku display".as_ptr (), (*t).id); }

      dpyinfo = (*t).display_info.haiku as *mut haiku_display_info;
    }
  else if STRINGP (object)
    { dpyinfo = haiku_display_info_for_name (object); }
  else
    {
      let f: *mut frame = decode_window_system_frame (object);
      dpyinfo = FRAME_DISPLAY_INFO! (f);
    }

  return dpyinfo;
}

// C 446-455.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_set_title_bar_text (f: *mut frame, text: Lisp_Object)
{
  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    {
      block_input ();
      BWindow_retitle (FRAME_HAIKU_WINDOW! (f), SSDATA (ENCODE_UTF_8 (text)));
      unblock_input ();
    }
}

// C 457-472.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_title (f: *mut frame, name: Lisp_Object, old_name: Lisp_Object)
{
  /* Don't change the title if it's already NAME.  */
  if EQ (name, (*f).title)
    { return; }

  update_mode_lines = 26;

  fset_title (f, name);

  let mut name = name;
  if NILP (name)
    { name = (*f).name; }

  haiku_set_title_bar_text (f, name);
}

// C 474-496.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object,
                                              oldval: Lisp_Object)
{
  let border: c_int;

  if NILP (arg)
    { border = -1; }
  else if RANGED_FIXNUMP (0, arg, INT_MAX as EMACS_INT)
    { border = XFIXNAT (arg) as c_int; }
  else
    { signal_error (c"Invalid child frame border width".as_ptr (), arg); }

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH (f)
    {
      (*f).child_frame_border_width = border;

      if !FRAME_HAIKU_WINDOW! (f).is_null ()
	{ adjust_frame_size (f, -1, -1, 3, false, Qchild_frame_border_width ()); }

      SET_FRAME_GARBAGED! (f);
    }
}

// C 498-539.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_parent_frame (f: *mut frame, new_value: Lisp_Object,
                                 old_value: Lisp_Object)
{
  let mut p: *mut frame = std::ptr::null_mut ();
  block_input ();
  if !NILP (new_value)
      && (!FRAMEP (new_value)
	  || { p = XFRAME (new_value); !FRAME_LIVE_P (p) }
	  || !FRAME_HAIKU_P (p))
    {
      store_frame_param (f, Qparent_frame (), old_value);
      unblock_input ();
      error (c"Invalid specification of `parent-frame'".as_ptr ());
    }

  if EQ (new_value, old_value)
    {
      unblock_input ();
      return;
    }

  if !NILP (old_value)
    {
      EmacsWindow_unparent (FRAME_HAIKU_WINDOW! (f));
      /* C's `Window` is `haiku` (haikugui.h 97), but this crate's first `Window` definition is
	 `include/nsgui.rs`'s `c_int` (the measured X-compat collision `docs/ports/include-haikugui.md`
	 records), so the assignment keeps an explicit `as Window` cast - C's own value, and the
	 member is never read by this unit.  */
      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).parent_desc
	= std::ptr::null_mut::<c_void> () as Window;
    }

  if !NILP (new_value)
    {
      EmacsWindow_parent_to (FRAME_HAIKU_WINDOW! (f), FRAME_HAIKU_WINDOW! (p));
      BWindow_set_offset (FRAME_HAIKU_WINDOW! (f), (*f).left_pos, (*f).top_pos);

      /* This isn't actually used for anything, but makes the
	 `parent-id' parameter correct.  */
      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).parent_desc = FRAME_HAIKU_WINDOW! (p)
	as Window;
    }
  fset_parent_frame (f, new_value);
  unblock_input ();
}

// C 541-580.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_z_group (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  let mut rc: c_int;

  /* Tooltip frames can't have Z groups, since the window feel is
     overridden during frame creation.  */
  if FRAME_TOOLTIP_P (f)
    { return; }

  rc = 1;
  block_input ();

  if NILP (new_value)
    {
      BWindow_set_z_group (FRAME_HAIKU_WINDOW! (f), Z_GROUP_NONE);
      FRAME_Z_GROUP! (f, z_group_none as u32);
    }
  else if EQ (new_value, Qabove ())
    {
      BWindow_set_z_group (FRAME_HAIKU_WINDOW! (f), Z_GROUP_ABOVE);
      FRAME_Z_GROUP! (f, z_group_above as u32);
    }
  else if EQ (new_value, Qbelow ())
    {
      BWindow_set_z_group (FRAME_HAIKU_WINDOW! (f), Z_GROUP_BELOW);
      FRAME_Z_GROUP! (f, z_group_below as u32);
    }
  else
    { rc = 0; }

  unblock_input ();

  if rc == 0
    { error (c"Invalid z-group specification".as_ptr ()); }

  /* Setting the Z-group can change the frame's decorator.  */
  haiku_update_after_decoration_change (f);
}

// C 582-586.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_explicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  haiku_set_name (f, arg, true);
}

// C 588-599.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object,
                                     old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    { FRAME_NO_ACCEPT_FOCUS! (f, !NILP (new_value)); }

  block_input ();
  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    { BWindow_set_avoid_focus (FRAME_HAIKU_WINDOW! (f), FRAME_NO_ACCEPT_FOCUS (f) as c_int); }
  unblock_input ();
}

// C 601-608.
#[cfg (feature = "have_haiku")]
unsafe fn initial_setup_back_buffer (f: *mut frame)
{
  block_input ();
  if NILP (CDR (Fassq (Qinhibit_double_buffering (), (*f).param_alist)))
    { EmacsView_set_up_double_buffering (FRAME_HAIKU_VIEW! (f)); }
  unblock_input ();
}

// C 610-630.
#[cfg (feature = "have_haiku")]
unsafe fn unwind_create_frame (frame: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame = XFRAME (frame);

  /* If frame is already dead, nothing to do.  This can happen if the
     display is disconnected after the frame has become official, but
     before x_create_frame removes the unwind protect.  */
  if !FRAME_LIVE_P (f)
    { return Qnil (); }

  /* If frame is ``official'', nothing to do.  */
  if NILP (Fmemq (frame, Vframe_list))
    {
      haiku_free_frame_resources (f);
      free_glyphs (f);
      return Qt ();
    }

  return Qnil ();
}

// C 632-640.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn unwind_create_tip_frame (frame: Lisp_Object)
{
  let deleted: Lisp_Object;

  deleted = unwind_create_frame (frame);
  if !NILP (deleted)
    { tip_frame = Qnil (); }
}

// C 642-653.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_decode_color (f: *mut frame, color_name: Lisp_Object) -> c_ulong
{
  let mut cdef: Emacs_Color = std::mem::zeroed ();

  CHECK_STRING (color_name);

  if haiku_get_color (SSDATA (color_name), &raw mut cdef) == 0
    { return cdef.pixel; }

  signal_error (c"Undefined color".as_ptr (), color_name);
}

// C 655-676.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_foreground_color (f: *mut frame, arg: Lisp_Object,
                                                 oldval: Lisp_Object)
{
  let output: *mut haiku_output;
  let fg: c_ulong;
  let old_fg: c_ulong;

  fg = haiku_decode_color (f, arg);
  old_fg = FRAME_FOREGROUND_PIXEL (f);
  FRAME_FOREGROUND_PIXEL! (f, fg);
  output = FRAME_OUTPUT_DATA! (f) as *mut haiku_output;

  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    {
      if (*output).cursor_color.pixel == old_fg
	{ haiku_query_color (fg as uint32_t, &raw mut (*output).cursor_color); }

      update_face_from_frame_parameter (f, Qforeground_color (), arg);

      if FRAME_VISIBLE_P (f)
        { redraw_frame (f); }
    }
}

// C 678-682.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn do_unwind_create_frame (frame: Lisp_Object)
{
  unwind_create_frame (frame);
}


/* keyboard.h 38's `KVAR` (C 995/996/996's `KVAR (kb, Vdefault_minibuffer_frame)`) is a
   token-pasting macro; the port keeps the name and one arm per member (the `src/keyboard.rs`
   722-735 shape, `docs/ports/excluded-macros.txt`).  */
#[cfg (feature = "have_haiku")]
macro_rules! KVAR {
  ($kb:expr, Vdefault_minibuffer_frame) => { (*$kb).Vdefault_minibuffer_frame_ };
}

/// C: frame.h 446: `ENUM_BF (output_method) output_method : 4;` - written at C 743.  The mirror
/// has the four-bit *reader* (`include/frame.rs` 1362-1365) and no four-bit setter, so the unit
/// spells C's own masked write (`output_method` is `FRAME_OUTPUT_METHOD_UNIT`/`_SHIFT`/`_MASK`).
#[cfg (feature = "have_haiku")]
macro_rules! output_method {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & !(FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
                  | (((($v) as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT) };
}
/// C: frame.h 460: `ENUM_BF (fullscreen_type) want_fullscreen : 4;` - read at C 1001.  The mirror
/// has the shift/unit/mask constants and no accessor, so the unit reads the field itself.
#[cfg (feature = "have_haiku")]
macro_rules! want_fullscreen {
  ($f:expr) => {
    frame_field ($f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT,
                 FRAME_WANT_FULLSCREEN_MASK) as fullscreen_type };
}

/// C: haikuterm.h 289: `#define FRAME_DISPLAY_INFO(f) (FRAME_OUTPUT_DATA (f)->display_info)` -
/// read at C 762/855/928/1364/1673/1823 and written at C 738.  The mirror's macro is read-only, so
/// the two-arm form shadows it (the `src/haikuterm.rs` 632-652 `FRAME_FONTSET` shape).
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { ((*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).display_info) };
  ($f:expr, $v:expr) => {
    (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).display_info = $v };
}

/* C: dispextern.h 3069-3196's `struct redisplay_interface`: `include/dispextern.rs` carries the
   record behind `feature = "dispextern_face_records"` (off in every configuration today), so the
   unit carries C's record itself, as `src/dispnew.rs` 3559 and `src/androidterm.rs` 111-126 do
   for their own uses.  Only `default_font_parameter` (C 3194) is read here, but the members before
   it are what put it at its offset, so the whole tail is spelt.  */
#[cfg (all (feature = "have_haiku", not (feature = "dispextern_face_records")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct redisplay_interface
{
  pub frame_parm_handlers: *mut c_void,                                        // offset 0
  /// C: dispextern.h 3076: `void (*produce_glyphs) (struct it *it);` - `struct it` is a record in
  /// the mirror's `dispextern_unit_names` module (not glob-imported), so only the pointee is
  /// untyped here (PORTING.md P-55/P-72 rule 3); the field is one pointer wide either way.
  pub produce_glyphs: Option<unsafe extern "C" fn (*mut c_void)>,             // offset 8
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, string: *mut glyph, area: c_int, len: c_int)>,
  pub insert_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, start: *mut glyph, area: c_int, len: c_int)>,
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, x: c_int)>,
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut c_void, run: *mut run)>,
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut c_void, desired_row: *mut glyph_row)>,
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut c_void)>,
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut c_void, cursor_on_p: bool, mouse_face_overwritten_p: bool)>,
  pub flush_display: Option<unsafe extern "C" fn (w: *mut c_void)>,            // 72
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut c_void)>,  // 80
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut c_void, left: *mut c_int, right: *mut c_int)>,
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, overlaps: c_int)>,  // 96
  /// C: dispextern.h 3134: `void (*draw_fringe_bitmap) (struct window *, struct glyph_row *,
  /// struct draw_fringe_bitmap_params *);` - `draw_fringe_bitmap_params` is in the same
  /// non-glob-imported module, so only the pointee is untyped (as above).
  pub draw_fringe_bitmap: Option<unsafe extern "C" fn (*mut c_void, *mut glyph_row, *mut c_void)>,
  pub define_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)>,
  pub destroy_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int)>,
  pub compute_glyph_string_overhangs: Option<unsafe extern "C" fn (s: *mut glyph_string)>,
  pub draw_glyph_string: Option<unsafe extern "C" fn (s: *mut glyph_string)>,
  pub define_frame_cursor: Option<unsafe extern "C" fn (f: *mut c_void, cursor: Emacs_Cursor)>,
  pub clear_frame_area: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int)>,
  pub clear_under_internal_border: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub draw_window_cursor: Option<unsafe extern "C" fn (w: *mut c_void, glyph_row: *mut glyph_row, x: c_int, y: c_int, cursor_type: text_cursor_kinds, cursor_width: c_int, on_p: bool, active_p: bool)>,
  pub draw_vertical_window_border: Option<unsafe extern "C" fn (w: *mut c_void, x: c_int, y_0: c_int, y_1: c_int)>,
  pub draw_window_divider: Option<unsafe extern "C" fn (w: *mut c_void, x_0: c_int, x_1: c_int, y_0: c_int, y_1: c_int)>,
  pub shift_glyphs_for_insert: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int, shift_by: c_int)>,
  pub show_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub hide_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,
  pub default_font_parameter: Option<unsafe extern "C" fn (f: *mut c_void, parms: Lisp_Object)>,  // 216
}
#[cfg (all (feature = "have_haiku", not (feature = "dispextern_face_records")))]
const _: () = core::assert! (core::mem::offset_of! (redisplay_interface, default_font_parameter)
                             == 216);

// C 684-1019.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_create_frame (mut parms: Lisp_Object) -> Lisp_Object
{
  let mut f: *mut frame;
  let mut cascade_target: *mut frame;
  let mut frame: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut name: Lisp_Object;
  let mut minibuffer_only: bool = false;
  let mut window_prompting: c_long = 0;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut display: Lisp_Object;
  let mut dpyinfo: *mut haiku_display_info = std::ptr::null_mut ();
  let mut kb: *mut kboard;

  if !(*x_display_list).focused_frame.is_null ()
    { cascade_target = (*x_display_list).focused_frame; }
  else if !(*x_display_list).focus_event_frame.is_null ()
    { cascade_target = (*x_display_list).focus_event_frame; }
  else
    { cascade_target = std::ptr::null_mut (); }

  /* Always cascade from the most toplevel frame.  */

  while !cascade_target.is_null () && !FRAME_PARENT_FRAME (cascade_target).is_null ()
    { cascade_target = FRAME_PARENT_FRAME (cascade_target); }

  parms = Fcopy_alist (parms);

  Vx_resource_name = Vinvocation_name;

  display = gui_display_get_arg (dpyinfo, parms, Qterminal (), std::ptr::null (),
                                 std::ptr::null (), RES_TYPE_STRING);
  if BASE_EQ (display, Qunbound ())
    { display = Qnil (); }
  dpyinfo = check_haiku_display_info (display);
  kb = (*(*dpyinfo).terminal).kboard as *mut kboard;

  if (*(*dpyinfo).terminal).name.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  name = gui_display_get_arg (dpyinfo, parms, Qname (), std::ptr::null (), std::ptr::null (),
                              RES_TYPE_STRING);
  if !STRINGP (name)
      && !BASE_EQ (name, Qunbound ())
      && !NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  if STRINGP (name)
    { Vx_resource_name = name; }

  /* make_frame_without_minibuffer can run Lisp code and garbage collect.  */
  /* No need to protect DISPLAY because that's not used after passing
     it to make_frame_without_minibuffer.  */
  frame = Qnil ();
  tem = gui_display_get_arg (dpyinfo, parms, Qminibuffer (),
                             c"minibuffer".as_ptr (), c"Minibuffer".as_ptr (),
                             RES_TYPE_SYMBOL);
  if EQ (tem, Qnone ()) || NILP (tem)
    { f = make_frame_without_minibuffer (Qnil (), kb, display); }
  else if EQ (tem, Qonly ())
    {
      f = make_minibuffer_frame ();
      minibuffer_only = true;
    }
  else if WINDOWP (tem)
    { f = make_frame_without_minibuffer (tem, kb, display); }
  else
    { f = make_frame (true); }

  XSETFRAME! (frame, f);

  frame_set_id_from_params (f, parms);

  (*f).terminal = (*dpyinfo).terminal;

  output_method! (f, output_haiku);
  (*f).output_data.haiku = xzalloc (std::mem::size_of::<haiku_output> ());
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type = -1;
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).relief_background = -1;

  fset_icon_name (f, gui_display_get_arg (dpyinfo, parms, Qicon_name (),
                                          c"iconName".as_ptr (), c"Title".as_ptr (),
                                          RES_TYPE_STRING));
  if !STRINGP ((*f).icon_name)
    { fset_icon_name (f, Qnil ()); }

  FRAME_DISPLAY_INFO! (f, dpyinfo);

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (Some (do_unwind_create_frame), frame);

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name) || !STRINGP (name)
    {
      fset_name (f, Vinvocation_name);
      explicit_name! (f, 0);
    }
  else
    {
      fset_name (f, name);
      explicit_name! (f, 1);
      specbind (Qx_resource_name (), name);
    }

  /* C 787-792: `#ifdef USE_BE_CAIRO` / the two FT/CR driver registrations / `#endif`, with the
     nested `#ifdef HAVE_HARFBUZZ`.  Both arms are carried (P-51/P-86 ①); the cairo arm names
     `ftcrfont_driver`/`ftcrhbfont_driver`, which only that configuration defines.  */
  #[cfg (feature = "be_cairo")]
  {
    register_font_driver (&raw const ftcrfont_driver as *const c_void, f);
    #[cfg (feature = "harfbuzz")]
    { register_font_driver (&raw const ftcrhbfont_driver as *const c_void, f); }
  }
  register_font_driver (&raw const haikufont_driver as *const c_void, f);

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
                         c"fontBackend".as_ptr (), c"FontBackend".as_ptr (), RES_TYPE_STRING);

  ((*(FRAME_RIF! (f) as *mut redisplay_interface)).default_font_parameter.unwrap ())
    (f as *mut c_void, parms);

  if FRAME_FONT! (f).is_null ()
    {
      delete_frame (frame, Qnoelisp ());
      error (c"Invalid frame font".as_ptr ());
    }

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),
                         c"borderwidth".as_ptr (), c"BorderWidth".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (0),
                         c"internalBorderWidth".as_ptr (), c"InternalBorderWidth".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qchild_frame_border_width (), Qnil (),
			 c"childFrameBorderWidth".as_ptr (), c"childFrameBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qvertical_scroll_bars (), Qt (),
			 c"verticalScrollBars".as_ptr (), c"VerticalScrollBars".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qhorizontal_scroll_bars (), Qnil (),
                         c"horizontalScrollBars".as_ptr (), c"HorizontalScrollBars".as_ptr (),
                         RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qforeground_color (),
                         build_string (c"black".as_ptr ()),
                         c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color (),
                         build_string (c"white".as_ptr ()),
                         c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"font-color".as_ptr ()),
                         c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qline_spacing (), Qnil (),
                         c"lineSpacing".as_ptr (), c"LineSpacing".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qleft_fringe (), Qnil (),
                         c"leftFringe".as_ptr (), c"LeftFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_fringe (), Qnil (),
                         c"rightFringe".as_ptr (), c"RightFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  init_frame_faces (f);

  /* Read comment about this code in corresponding place in xfns.c.  */
  tem = gui_display_get_arg (dpyinfo, parms, Qmin_width (), std::ptr::null (), std::ptr::null (),
                             RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_width (), tem); }
  tem = gui_display_get_arg (dpyinfo, parms, Qmin_height (), std::ptr::null (),
                             std::ptr::null (), RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_height (), tem); }

  adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
		     FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 5, true,
		     Qx_create_frame_1 ());

  gui_default_parameter (f, parms, Qno_focus_on_map (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qno_accept_focus (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* The resources controlling the menu-bar, tool-bar, and tab-bar are
     processed specially at startup, and reflected in the mode
     variables; ignore them here.  */
  gui_default_parameter (f, parms, Qmenu_bar_lines (),
			 if NILP (Vmenu_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qtab_bar_lines (),
			 if NILP (Vtab_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qtool_bar_lines (),
			 if NILP (Vtool_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbuffer_predicate (), Qnil (),
			 c"bufferPredicate".as_ptr (), c"BufferPredicate".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qtitle (), Qnil (), c"title".as_ptr (),
			 c"Title".as_ptr (), RES_TYPE_STRING);

  parms = get_geometry_from_preferences (dpyinfo, parms);
  window_prompting = gui_figure_window_size (f, parms, false, true);

  tem = gui_display_get_arg (dpyinfo, parms, Qunsplittable (), std::ptr::null (),
                             std::ptr::null (), RES_TYPE_BOOLEAN);
  no_split! (f, (minibuffer_only || (!BASE_EQ (tem, Qunbound ()) && !NILP (tem))) as u32);

  (*(*f).terminal).reference_count += 1;

  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).window
    = BWindow_new (&raw mut (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).view);

  if (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).window.is_null ()
    { xsignal1 (Qerror (), build_unibyte_string (c"Could not create window".as_ptr ())); }

  block_input ();
  if !minibuffer_only && FRAME_EXTERNAL_MENU_BAR (f)
    { initialize_frame_menubar (f); }
  unblock_input ();

  Vframe_list = Fcons (frame, Vframe_list);

  let mut parent_frame: Lisp_Object = gui_display_get_arg (dpyinfo, parms,
						  Qparent_frame (), std::ptr::null (),
						  std::ptr::null (), RES_TYPE_SYMBOL);

  if BASE_EQ (parent_frame, Qunbound ())
      || NILP (parent_frame)
      || !FRAMEP (parent_frame)
      || !FRAME_LIVE_P (XFRAME (parent_frame))
    { parent_frame = Qnil (); }

  /* It doesn't make sense to center child frames, the resulting
     position makes no sense.  */
  if !NILP (parent_frame)
    { window_prompting |= PPosition; }

  fset_parent_frame (f, parent_frame);
  store_frame_param (f, Qparent_frame (), parent_frame);

  if !NILP (parent_frame)
    { haiku_set_parent_frame (f, parent_frame, Qnil ()); }

  gui_default_parameter (f, parms, Qundecorated (), Qnil (), std::ptr::null (),
                         std::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qoverride_redirect (), Qnil (), std::ptr::null (),
                         std::ptr::null (), RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parms, Qicon_type (), Qnil (),
                         c"bitmapIcon".as_ptr (), c"BitmapIcon".as_ptr (), RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (), RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qscroll_bar_width (), Qnil (),
			 c"scrollBarWidth".as_ptr (), c"ScrollBarWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qscroll_bar_height (), Qnil (),
			 c"scrollBarHeight".as_ptr (), c"ScrollBarHeight".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
			 c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
                         c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
                         c"bordersRespectAlphaBackground".as_ptr (),
                         c"BordersRespectAlphaBackground".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qfullscreen (), Qnil (),
			 c"fullscreen".as_ptr (), c"Fullscreen".as_ptr (), RES_TYPE_SYMBOL);

  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
			 RES_TYPE_BOOLEAN);

  can_set_window_size! (f, 1);

  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qx_create_frame_2 ());

  let mut visibility: Lisp_Object;

  visibility = gui_display_get_arg (dpyinfo, parms, Qvisibility (), std::ptr::null (),
				    std::ptr::null (), RES_TYPE_SYMBOL);
  if BASE_EQ (visibility, Qunbound ())
    { visibility = Qt (); }
  if EQ (visibility, Qicon ())
    { haiku_iconify_frame (f); }
  else if !NILP (visibility)
    { haiku_visualize_frame (f); }
  else /* Qnil */
    {
      was_invisible! (f, 1);
    }

  if FRAME_HAS_MINIBUF_P (f)
      && (!FRAMEP (KVAR! (kb, Vdefault_minibuffer_frame))
	  || !FRAME_LIVE_P (XFRAME (KVAR! (kb, Vdefault_minibuffer_frame))))
    { kset_default_minibuffer_frame (kb, frame); }

  /* Set whether or not frame synchronization is enabled.  */
  gui_default_parameter (f, parms, Quse_frame_synchronization (), Qt (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parms, Qz_group (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_SYMBOL);

  tem = parms;
  while CONSP (tem)
    {
      if CONSP (XCAR (tem)) && !NILP (XCAR (XCAR (tem)))
	{ fset_param_alist (f, Fcons (XCAR (tem), (*f).param_alist)); }
      tem = XCDR (tem);
    }

  block_input ();
  if window_prompting & (USPosition | PPosition) as c_long != 0
    { haiku_set_offset (f, (*f).left_pos, (*f).top_pos, 1); }
  else if !cascade_target.is_null ()
    { haiku_set_offset (f, (*cascade_target).left_pos + 15,
		      (*cascade_target).top_pos + 15, 1); }
  else
    { BWindow_center_on_screen (FRAME_HAIKU_WINDOW! (f)); }
  unblock_input ();

  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).configury_done = true;

  if want_fullscreen! (f) != FULLSCREEN_NONE
    { ((*(FRAME_TERMINAL! (f))).fullscreen_hook.unwrap ()) (f as *mut c_void); }

  /* Make sure windows on this frame appear in calls to next-window
     and similar functions.  */
  Vwindow_list = Qnil ();

  return unbind_to (count, frame);
}

/* Create a frame for a tooltip.  PARMS is a list of frame parameters.
   TEXT is the string to display in the tip frame.  Value is the
   frame.

   Note that functions called here, esp. gui_default_parameter can
   signal errors, for instance when a specified color name is
   undefined.  We have to make sure that we're in a consistent state
   when this happens.  */

// C 1021-1248.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_create_tip_frame (parms: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame;
  let mut frame: Lisp_Object;
  let mut name: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let face_change_before: bool = face_change;
  let dpyinfo: *mut haiku_display_info = x_display_list;

  if (*dpyinfo).terminal.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  let mut parms = parms;
  parms = Fcopy_alist (parms);

  /* Get the name of the frame to use for resource lookup.  */
  name = gui_display_get_arg (dpyinfo, parms, Qname (), c"name".as_ptr (), c"Name".as_ptr (),
                              RES_TYPE_STRING);
  if !STRINGP (name)
      && !BASE_EQ (name, Qunbound ())
      && !NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  frame = Qnil ();
  f = make_frame (false);
  wants_modeline! (f, 0);
  XSETFRAME! (frame, f);
  record_unwind_protect (Some (unwind_create_tip_frame), frame);

  (*f).terminal = (*dpyinfo).terminal;

  /* By setting the output method, we're essentially saying that
     the frame is live, as per FRAME_LIVE_P.  If we get a signal
     from this point on, x_destroy_window might screw up reference
     counts etc.  */
  output_method! (f, output_haiku);
  (*f).output_data.haiku = xzalloc (std::mem::size_of::<haiku_output> ());
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).wait_for_event_type = -1;
  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).relief_background = -1;

  tooltip! (f, 1);
  fset_icon_name (f, Qnil ());
  FRAME_DISPLAY_INFO! (f, dpyinfo);

  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).parent_desc
    = std::ptr::null_mut::<c_void> () as Window;

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)
    { explicit_name! (f, 0); }
  else
    {
      fset_name (f, name);
      explicit_name! (f, 1);
      /* use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);
    }

  /* C 1078-1083: the same `#ifdef USE_BE_CAIRO` / `#ifdef HAVE_HARFBUZZ` pair as C 787-792.  */
  #[cfg (feature = "be_cairo")]
  {
    register_font_driver (&raw const ftcrfont_driver as *const c_void, f);
    #[cfg (feature = "harfbuzz")]
    { register_font_driver (&raw const ftcrhbfont_driver as *const c_void, f); }
  }
  register_font_driver (&raw const haikufont_driver as *const c_void, f);

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
                         c"fontBackend".as_ptr (), c"FontBackend".as_ptr (), RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values that are
     needed to determine window geometry.  */
  ((*(FRAME_RIF! (f) as *mut redisplay_interface)).default_font_parameter.unwrap ())
    (f as *mut c_void, parms);

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP (Fassq (Qinternal_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),
                                   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
                                   RES_TYPE_NUMBER);
      if !BASE_EQ (value, Qunbound ())
	{ parms = Fcons (Fcons (Qinternal_border_width (), value), parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (1),
                         c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
                         RES_TYPE_NUMBER);

  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),
                         c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);

  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),
                         c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);

  /* FIXME: is there a better method to tell Emacs to not recolor the
     cursors other than setting the color to a special value?  */
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"font-color".as_ptr ()),
                         c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qcursor_color (), build_string (c"black".as_ptr ()),
                         c"cursorColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),
                         c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);

  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
                         c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
                         RES_TYPE_BOOLEAN);

  gui_figure_window_size (f, parms, false, false);

  {
    let mut window: *mut c_void;

    block_input ();
    window = BWindow_new (&raw mut (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).view);

    (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).window = window;
    if window.is_null ()
      { emacs_abort (); }

    BWindow_set_tooltip_decoration (window);
    unblock_input ();
  }

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
                         c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
                         c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
                         c"cursorType".as_ptr (), c"CursorType".as_ptr (), RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
                         c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
                         c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
                         c"bordersRespectAlphaBackground".as_ptr (),
                         c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  initial_setup_back_buffer (f);

  /* Add `tooltip' frame parameter's default value. */
  if NILP (Fframe_parameter (frame, Qtooltip ()))
    {
      AUTO_FRAME_ARG! (arg, Qtooltip (), Qt ());
      Fmodify_frame_parameters (frame, arg);
    }

  /* FIXME - can this be done in a similar way to normal frames?
     https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html */

  {
    let disptype: Lisp_Object;

    if be_get_display_planes () == 1
      { disptype = Qmono (); }
    else if be_is_display_grayscale ()
      { disptype = Qgrayscale (); }
    else
      { disptype = Qcolor (); }

    if NILP (Fframe_parameter (frame, Qdisplay_type ()))
      {
	AUTO_FRAME_ARG! (arg, Qdisplay_type (), disptype);
	Fmodify_frame_parameters (frame, arg);
      }
  }

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which let's the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  {
    let bg: Lisp_Object = Fframe_parameter (frame, Qbackground_color ());

    calln! (Qface_set_after_frame_default (), frame, Qnil ());

    if !EQ (bg, Fframe_parameter (frame, Qbackground_color ()))
      {
	AUTO_FRAME_ARG! (arg, Qbackground_color (), bg);
	Fmodify_frame_parameters (frame, arg);
      }
  }

  no_split! (f, 1);

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */
  (*(*f).terminal).reference_count += 1;

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons (frame, Vframe_list);
  can_set_window_size! (f, 1);
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qtip_frame ());

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the clearing of
     all current matrices.  Since this isn't necessary here, avoid it
     by resetting face_change to the value it had before we created
     the tip frame.  */
  face_change = face_change_before;

  /* Discard the unwind_protect.  */
  return unbind_to (count, frame);
}

// C 1250-1317.
#[cfg (feature = "have_haiku")]
unsafe fn compute_tip_xy (f: *mut frame, parms: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object,
                         width: c_int, height: c_int, root_x: *mut c_int, root_y: *mut c_int)
{
  let left: Lisp_Object;
  let top: Lisp_Object;
  let right: Lisp_Object;
  let bottom: Lisp_Object;
  let mut min_x: c_int = 0;
  let mut min_y: c_int = 0;
  let mut max_x: c_int = 0;
  let mut max_y: c_int = 0;

  /* User-specified position?  */
  left = Fcdr (Fassq (Qleft (), parms));
  top  = Fcdr (Fassq (Qtop (), parms));
  right = Fcdr (Fassq (Qright (), parms));
  bottom = Fcdr (Fassq (Qbottom (), parms));

  /* Move the tooltip window where the mouse pointer is.  Resize and
     show it.  */
  if (!FIXNUMP (left) && !FIXNUMP (right))
      || (!FIXNUMP (top) && !FIXNUMP (bottom))
    {
      let mut x: c_int = 0;
      let mut y: c_int = 0;

      /* Default min and max values.  */
      min_x = 0;
      min_y = 0;

      be_get_screen_dimensions (&raw mut max_x, &raw mut max_y);

      max_x = max_x - 1;
      max_y = max_y - 1;

      block_input ();
      BView_get_mouse (FRAME_HAIKU_VIEW! (f), &raw mut x, &raw mut y);
      BView_convert_to_screen (FRAME_HAIKU_VIEW! (f), &raw mut x, &raw mut y);
      *root_x = x;
      *root_y = y;
      unblock_input ();
    }

  if FIXNUMP (top)
    { *root_y = XFIXNUM (top) as c_int; }
  else if FIXNUMP (bottom)
    { *root_y = XFIXNUM (bottom) as c_int - height; }
  else if *root_y + XFIXNUM (dy) as c_int <= min_y
    { *root_y = min_y; /* Can happen for negative dy */ }
  else if *root_y + XFIXNUM (dy) as c_int + height <= max_y
    /* It fits below the pointer */
    { *root_y += XFIXNUM (dy) as c_int; }
  else if height + XFIXNUM (dy) as c_int + min_y <= *root_y
    /* It fits above the pointer.  */
    { *root_y -= height + XFIXNUM (dy) as c_int; }
  else
    /* Put it on the top.  */
    { *root_y = min_y; }

  if FIXNUMP (left)
    { *root_x = XFIXNUM (left) as c_int; }
  else if FIXNUMP (right)
    { *root_x = XFIXNUM (right) as c_int - width; }
  else if *root_x + XFIXNUM (dx) as c_int <= min_x
    { *root_x = 0; /* Can happen for negative dx */ }
  else if *root_x + XFIXNUM (dx) as c_int + width <= max_x
    /* It fits to the right of the pointer.  */
    { *root_x += XFIXNUM (dx) as c_int; }
  else if width + XFIXNUM (dx) as c_int + min_x <= *root_x
    /* It fits to the left of the pointer.  */
    { *root_x -= width + XFIXNUM (dx) as c_int; }
  else
    /* Put it left justified on the screen -- it ought to fit that way.  */
    { *root_x = min_x; }
}

/// C: frame.h 1835-1840: `INLINE void flush_frame (struct frame *f)` - the mirror
/// `include/frame.rs`'s macro spells `FRAME_RIF` without the `*mut c_void` cast this unit's
/// `FRAME_RIF!` needs (`terminal.rif` is `*mut c_void` there), so the unit shadows it with C's
/// own body (the ledger's Owed names the macro).
#[cfg (feature = "have_haiku")]
macro_rules! flush_frame {
  ($f:expr) => {
    {
      let rif: *mut redisplay_interface = FRAME_RIF! ($f) as *mut redisplay_interface;
      if !rif.is_null () && (*rif).flush_display.is_some ()
	{ ((*rif).flush_display.unwrap ()) ($f as *mut c_void); }
    }
  };
}


/* (status 3.545) the eight prototypes this unit used to re-declare in a module-local block
   are the mirror's now, with C's return types (`include/haiku_support.rs` 689-727 and
   `include/haikuterm.rs` 474); the workaround and its named `use` are gone.  */

/* C: dispextern.h 291-294: the text-position setters C 2590 expands (`BYTEPOS`/`CHARPOS`/
   `SET_TEXT_POS`).  `include/dispextern.rs` keeps them inside its `dispextern_unit_names`
   module, and a `use` of that module does not bring its `macro_rules!`s, so the unit carries
   C's own bodies (the `src/xdisp.rs` 1918-1926 shape).  */
/// C: #define SET_TEXT_POS(POS, CHARPOS, BYTEPOS) ((POS).charpos = (CHARPOS), (POS).bytepos = BYTEPOS)
#[cfg (feature = "have_haiku")]
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}

/// C: haikuterm.h 283-294: `#define FRAME_OUTPUT_DATA(f) ((f)->output_data.haiku)` - written at
/// C 1719.  The mirror's macro is read-only, so the two-arm form shadows it (the
/// `src/haikuterm.rs` 632-652 shape).
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { ((*$f).output_data.haiku) };
  ($f:expr, $v:expr) => { (*$f).output_data.haiku = $v };
}

// C 1321-1371.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_hide_tip (delete: bool) -> Lisp_Object
{
  let mut it: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();

  if !NILP (tip_timer)
    {
      calln! (Qcancel_timer (), tip_timer);
      tip_timer = Qnil ();
    }

  FOR_EACH_FRAME! (it, frame, {
    if FRAME_WINDOW_P (XFRAME (frame))
	&& !FRAME_HAIKU_VIEW! (XFRAME (frame)).is_null ()
      { BView_set_tooltip (FRAME_HAIKU_VIEW! (XFRAME (frame)), std::ptr::null ()); }
  });

  if NILP (tip_frame)
      || (!delete && !NILP (tip_frame)
	  && !FRAME_VISIBLE_P (XFRAME (tip_frame)))
    { return Qnil (); }
  else
    {
      let mut was_open: Lisp_Object = Qnil ();

      let count: specpdl_ref = SPECPDL_INDEX ();
      specbind (Qinhibit_redisplay (), Qt ());
      specbind (Qinhibit_quit (), Qt ());

      if !NILP (tip_frame)
	{
	  if FRAME_LIVE_P (XFRAME (tip_frame))
	    {
	      if delete
		{
		  delete_frame (tip_frame, Qnil ());
		  tip_frame = Qnil ();
		}
	      else
		{ haiku_unvisualize_frame (XFRAME (tip_frame)); }

	      was_open = Qt ();
	    }
	  else
	    { tip_frame = Qnil (); }
	}
      else
	{ tip_frame = Qnil (); }

      return unbind_to (count, was_open);
    }
}

// C 1373-1386.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_undecorated (f: *mut frame, new_value: Lisp_Object,
                                old_value: Lisp_Object)
{
  if EQ (new_value, old_value)
    { return; }

  block_input ();
  FRAME_UNDECORATED! (f, !NILP (new_value));
  BWindow_change_decoration (FRAME_HAIKU_WINDOW! (f), NILP (new_value) as c_int);
  unblock_input ();

  haiku_update_after_decoration_change (f);
}

// C 1388-1402.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_override_redirect (f: *mut frame, new_value: Lisp_Object,
                                      old_value: Lisp_Object)
{
  if EQ (new_value, old_value)
    { return; }

  block_input ();
  BWindow_set_override_redirect (FRAME_HAIKU_WINDOW! (f), !NILP (new_value));
  FRAME_OVERRIDE_REDIRECT! (f, !NILP (new_value));
  unblock_input ();

  haiku_update_after_decoration_change (f);
}

// C 1404-1444.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let nlines: c_int;

  if FRAME_TOOLTIP_P (f)
    { return; }

  if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNUM (value) as c_int; }
  else
    { nlines = 0; }

  fset_redisplay (f as *mut c_void);

  if nlines != 0
    {
      FRAME_EXTERNAL_MENU_BAR! (f, 1);
      if FRAME_HAIKU_P (f) && FRAME_HAIKU_MENU_BAR! (f).is_null ()
	{ window_bits! (update_mode_line, XWINDOW (FRAME_SELECTED_WINDOW (f)), true); }
    }
  else
    {
      FRAME_MENU_BAR_LINES! (f, 0);
      FRAME_MENU_BAR_HEIGHT! (f, 0);

      if FRAME_EXTERNAL_MENU_BAR (f)
	{ free_frame_menubar (f); }

      FRAME_EXTERNAL_MENU_BAR! (f, 0);
      FRAME_HAIKU_MENU_BAR! (f, std::ptr::null_mut ());
    }

  adjust_frame_glyphs (f);
}

/* Return geometric attributes of FRAME.  According to the value of
   ATTRIBUTES return the outer edges of FRAME (Qouter_edges), the
   inner edges of FRAME, the root window edges of frame (Qroot_edges).
   Any other value means to return the geometry as returned by
   Fx_frame_geometry.  */

// C 1446-1548.
#[cfg (feature = "have_haiku")]
unsafe fn frame_geometry (frame: Lisp_Object, attribute: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame;
  let parent: *mut frame;
  let window: *mut c_void;
  let mut outer_x: c_int = 0;
  let mut outer_y: c_int = 0;
  let mut outer_width: c_int = 0;
  let mut outer_height: c_int = 0;
  let mut right_off: c_int = 0;
  let mut bottom_off: c_int = 0;
  let mut top_off: c_int = 0;
  let mut native_x: c_int;
  let mut native_y: c_int;
  let mut inner_left: c_int;
  let mut inner_top: c_int;
  let inner_right: c_int;
  let mut inner_bottom: c_int;
  let internal_border_width: c_int;
  let tab_bar_height: c_int;
  let tool_bar_height: c_int;
  let tab_bar_width: c_int;

  f = decode_window_system_frame (frame);
  parent = FRAME_PARENT_FRAME (f);
  window = FRAME_HAIKU_WINDOW! (f);

  be_lock_window (window);
  be_get_window_decorator_frame (window, &raw mut outer_x, &raw mut outer_y,
				 &raw mut outer_width, &raw mut outer_height);
  be_get_window_decorator_dimensions (window, std::ptr::null_mut (), &raw mut top_off,
				      &raw mut right_off, &raw mut bottom_off);
  be_unlock_window (window);

  native_x = (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).frame_x;
  native_y = (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).frame_y;

  if !parent.is_null ()
    {
      /* Adjust all the coordinates by the coordinates of the parent
	 frame.  */
      outer_x -= (*(FRAME_OUTPUT_DATA! (parent) as *mut haiku_output)).frame_x;
      outer_y -= (*(FRAME_OUTPUT_DATA! (parent) as *mut haiku_output)).frame_y;
      native_x -= (*(FRAME_OUTPUT_DATA! (parent) as *mut haiku_output)).frame_x;
      native_y -= (*(FRAME_OUTPUT_DATA! (parent) as *mut haiku_output)).frame_y;
    }

  internal_border_width = FRAME_INTERNAL_BORDER_WIDTH (f);
  inner_left = native_x + internal_border_width;
  inner_top = native_y + internal_border_width;
  inner_right = (native_x + FRAME_PIXEL_WIDTH (f)
		 - internal_border_width);
  inner_bottom = (native_y + FRAME_PIXEL_HEIGHT (f)
		  - internal_border_width);

  tab_bar_height = FRAME_TAB_BAR_HEIGHT (f);
  tab_bar_width = if tab_bar_height != 0
		   { FRAME_PIXEL_WIDTH (f) - 2 * internal_border_width }
		   else { 0 };
  inner_top += tab_bar_height;

  tool_bar_height = FRAME_TOOL_BAR_HEIGHT (f);

  /* Subtract or add to the inner dimensions based on the tool bar
     position.  */

  if EQ (FRAME_TOOL_BAR_POSITION (f), Qtop ())
    { inner_top += tool_bar_height; }
  else
    { inner_bottom -= tool_bar_height; }

  if EQ (attribute, Qouter_edges ())
    { return list4i (outer_x as EMACS_INT, outer_y as EMACS_INT,
		   (outer_x + outer_width) as EMACS_INT,
		   (outer_y + outer_height) as EMACS_INT); }
  else if EQ (attribute, Qnative_edges ())
    { return list4i (native_x as EMACS_INT, native_y as EMACS_INT,
		   (native_x + FRAME_PIXEL_WIDTH (f)) as EMACS_INT,
		   (native_y + FRAME_PIXEL_HEIGHT (f)) as EMACS_INT); }
  else if EQ (attribute, Qinner_edges ())
    { return list4i (inner_left as EMACS_INT, inner_top as EMACS_INT, inner_right as EMACS_INT,
		     inner_bottom as EMACS_INT); }
  else
    { return list! (Fcons (Qouter_position (),
			Fcons (make_fixnum (outer_x as EMACS_INT),
			       make_fixnum (outer_y as EMACS_INT))),
		 Fcons (Qouter_size (),
			Fcons (make_fixnum (outer_width as EMACS_INT),
			       make_fixnum (outer_height as EMACS_INT))),
		 Fcons (Qexternal_border_size (),
			Fcons (make_fixnum (right_off as EMACS_INT),
			       make_fixnum (bottom_off as EMACS_INT))),
		 Fcons (Qtitle_bar_size (),
			Fcons (make_fixnum (outer_width as EMACS_INT),
			       make_fixnum (top_off as EMACS_INT))),
		 Fcons (Qmenu_bar_external (), Qnil ()),
		 Fcons (Qmenu_bar_size (),
			Fcons (make_fixnum ((FRAME_PIXEL_WIDTH (f)
					    - (FRAME_INTERNAL_BORDER_WIDTH (f)
					       * 2)) as EMACS_INT),
			       make_fixnum (FRAME_MENU_BAR_HEIGHT (f) as EMACS_INT))),
		 Fcons (Qtab_bar_size (),
			Fcons (make_fixnum (tab_bar_width as EMACS_INT),
			       make_fixnum (tab_bar_height as EMACS_INT))),
		 Fcons (Qtool_bar_external (), Qnil ()),
		 Fcons (Qtool_bar_position (), FRAME_TOOL_BAR_POSITION (f)),
		 Fcons (Qtool_bar_size (),
			Fcons (make_fixnum ((FRAME_PIXEL_WIDTH (f)
					    - (FRAME_INTERNAL_BORDER_WIDTH (f)
					       * 2)) as EMACS_INT),
			       make_fixnum (FRAME_TOOL_BAR_HEIGHT (f) as EMACS_INT))),
		 Fcons (Qinternal_border_width (),
			make_fixnum (FRAME_INTERNAL_BORDER_WIDTH (f) as EMACS_INT))); }
}

// C 1550-1572.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_background_color (f: *mut frame, arg: Lisp_Object,
                                                   oldval: Lisp_Object)
{
  let background: c_ulong;

  background = haiku_decode_color (f, arg);

  (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).cursor_fg = background as uint32_t;
  FRAME_BACKGROUND_PIXEL! (f, background);

  if !FRAME_HAIKU_VIEW! (f).is_null ()
    {
     BView_draw_lock (FRAME_HAIKU_DRAWABLE! (f), false, 0, 0, 0, 0);
      BView_SetViewColor (FRAME_HAIKU_DRAWABLE! (f), background as uint32_t);
      BView_draw_unlock (FRAME_HAIKU_DRAWABLE! (f));

      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).cursor_fg = background as uint32_t;
      update_face_from_frame_parameter (f, Qbackground_color (), arg);

      if FRAME_VISIBLE_P (f)
	{ redraw_frame (f); }
    }
}

// C 1574-1598.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_cursor_color (f: *mut frame, arg: Lisp_Object,
                                               oldval: Lisp_Object)
{
  let fore_pixel: c_ulong;
  let pixel: c_ulong;

  pixel = haiku_decode_color (f, arg);

  if !NILP (Vx_cursor_fore_pixel)
    {
      fore_pixel = haiku_decode_color (f, Vx_cursor_fore_pixel);
      (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).cursor_fg = fore_pixel as uint32_t;
    }
  else
    { (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).cursor_fg
	= FRAME_BACKGROUND_PIXEL (f) as uint32_t; }

  haiku_query_color (pixel as uint32_t, &raw mut FRAME_CURSOR_COLOR! (f));

  if FRAME_VISIBLE_P (f)
    {
      gui_update_cursor (f, false);
      gui_update_cursor (f, true);
    }

  update_face_from_frame_parameter (f, Qcursor_color (), arg);
}

// C 1600-1604.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_cursor_type (f: *mut frame, arg: Lisp_Object,
                                              oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);
}

// C 1606-1625.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_get_pixel (bitmap: haiku, x: c_int, y: c_int) -> c_ulong
{
  let data: *mut c_uchar;
  let mut bytes_per_row: int32_t = 0;
  let mut mono_p: c_int = 0;
  let mut left: c_int = 0;
  let mut right: c_int = 0;
  let mut top: c_int = 0;
  let mut bottom: c_int = 0;
  let byte: c_int;

  data = BBitmap_data (bitmap) as *mut c_uchar;
  BBitmap_dimensions (bitmap, &raw mut left, &raw mut top, &raw mut right, &raw mut bottom,
		      &raw mut bytes_per_row, &raw mut mono_p);

  if x < 0 || x > right - left || y < 0 || y > bottom - top
    { emacs_abort (); }

  if mono_p == 0
    { return *((data.offset ((bytes_per_row * y) as isize)) as *mut uint32_t).offset (x as isize)
	     as c_ulong; }

  byte = y * bytes_per_row + x / 8;
  return (*data.offset (byte as isize) as c_ulong) & ((1u32 << (x % 8)) as c_ulong);
}

// C 1627-1656.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_put_pixel (bitmap: haiku, x: c_int, y: c_int, pixel: c_ulong)
{
  let data: *mut c_uchar;
  let mut byte_: *mut c_uchar = std::ptr::null_mut ();
  let mut bytes_per_row: int32_t = 0;
  let mut mono_p: c_int = 0;
  let mut left: c_int = 0;
  let mut right: c_int = 0;
  let mut top: c_int = 0;
  let mut bottom: c_int = 0;
  let off: ptrdiff_t;
  let bit: ptrdiff_t;
  let xoff: ptrdiff_t;

  data = BBitmap_data (bitmap) as *mut c_uchar;
  BBitmap_dimensions (bitmap, &raw mut left, &raw mut top, &raw mut right, &raw mut bottom,
		      &raw mut bytes_per_row, &raw mut mono_p);

  if x < 0 || x > right - left || y < 0 || y > bottom - top
    { emacs_abort (); }

  if mono_p != 0
    {
      off = y as ptrdiff_t * bytes_per_row as ptrdiff_t;
      bit = (x % 8) as ptrdiff_t;
      xoff = (x / 8) as ptrdiff_t;

      byte_ = data.offset ((off + xoff) as isize);
      if pixel == 0
	{ *byte_ &= !((1u32 << bit) as c_uchar); }
      else
	{ *byte_ |= (1u32 << bit) as c_uchar; }
    }
  else
    { *((data.offset ((bytes_per_row * y) as isize)) as *mut uint32_t).offset (x as isize)
	= pixel as uint32_t; }
}

// C 1658-1723.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_free_frame_resources (f: *mut frame)
{
  let window: haiku;
  let drawable: haiku;
  let mbar: haiku;
  let hlinfo: *mut Mouse_HLInfo;
  let dpyinfo: *mut haiku_display_info;
  let mut bar: Lisp_Object;
  let mut b: *mut scroll_bar;

  check_window_system (f);
  block_input ();

  hlinfo = MOUSE_HL_INFO! (f);
  window = FRAME_HAIKU_WINDOW! (f);
  drawable = FRAME_HAIKU_VIEW! (f);
  mbar = FRAME_HAIKU_MENU_BAR! (f);
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  free_frame_faces (f);
  haiku_free_custom_cursors (f);

  /* Free scroll bars */
  bar = FRAME_SCROLL_BARS (f);
  while !NILP (bar)
    {
      b = XSCROLL_BAR! (bar);
      haiku_scroll_bar_remove (b);
      bar = (*b).next;
    }

  if f == (*dpyinfo).highlight_frame
    { (*dpyinfo).highlight_frame = std::ptr::null_mut (); }
  if f == (*dpyinfo).focused_frame
    { (*dpyinfo).focused_frame = std::ptr::null_mut (); }
  if f == (*dpyinfo).last_mouse_motion_frame
    { (*dpyinfo).last_mouse_motion_frame = std::ptr::null_mut (); }
  if f == (*dpyinfo).last_mouse_frame
    { (*dpyinfo).last_mouse_frame = std::ptr::null_mut (); }
  if f == (*dpyinfo).focus_event_frame
    { (*dpyinfo).focus_event_frame = std::ptr::null_mut (); }

  if f as *mut c_void == (*hlinfo).mouse_face_mouse_frame
    { reset_mouse_highlight (hlinfo); }

  if !mbar.is_null ()
    {
      BMenuBar_delete (mbar);
      if haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f))
	{
	  popup_activated_p -= 1;
	  haiku_output_bits! (menu_bar_open_p, FRAME_OUTPUT_DATA! (f), 0);
	}
    }

  if !drawable.is_null ()
    { BView_emacs_delete (drawable); }

  if !window.is_null ()
    { BWindow_quit (window); }

  if !(*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event.is_null ()
    { xfree ((*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).saved_menu_event as *mut c_void); }

  xfree (FRAME_OUTPUT_DATA! (f));
  FRAME_OUTPUT_DATA! (f, std::ptr::null_mut ());

  unblock_input ();
}

// C 1725-1737.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_iconify_frame (frame: *mut frame)
{
  if FRAME_ICONIFIED_P (frame)
    { return; }

  SET_FRAME_VISIBLE! (frame, false);
  SET_FRAME_ICONIFIED (frame, true as c_int);

  block_input ();
  BWindow_iconify (FRAME_HAIKU_WINDOW! (frame));
  unblock_input ();
}

// C 1739-1761.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_visualize_frame (f: *mut frame)
{
  block_input ();

  if !FRAME_VISIBLE_P (f)
    {
      if FRAME_NO_FOCUS_ON_MAP (f) != 0
	{ BWindow_set_avoid_focus (FRAME_HAIKU_WINDOW! (f), 1); }
      BWindow_set_visible (FRAME_HAIKU_WINDOW! (f), 1);
      if FRAME_NO_FOCUS_ON_MAP (f) != 0
	  && FRAME_NO_ACCEPT_FOCUS (f) == 0
	{ BWindow_set_avoid_focus (FRAME_HAIKU_WINDOW! (f), 0); }
      BWindow_sync (FRAME_HAIKU_WINDOW! (f));

      haiku_set_offset (f, (*f).left_pos, (*f).top_pos, 0);

      SET_FRAME_VISIBLE! (f, 1);
      SET_FRAME_ICONIFIED (f, 0);
    }

  unblock_input ();
}

// C 1763-1774.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_unvisualize_frame (f: *mut frame)
{
  block_input ();

  BWindow_set_visible (FRAME_HAIKU_WINDOW! (f), 0);
  BWindow_sync (FRAME_HAIKU_WINDOW! (f));
  SET_FRAME_VISIBLE! (f, 0);
  SET_FRAME_ICONIFIED (f, 0);

  unblock_input ();
}

// C 1776-1795.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_internal_border_width (f: *mut frame, arg: Lisp_Object,
                                                         oldval: Lisp_Object)
{
  let old_width: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
  let new_width: c_int = check_int_nonnegative (arg);

  if new_width == old_width
    { return; }

  (*f).internal_border_width = new_width;

  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    {
      adjust_frame_size (f, -1, -1, 3, false, Qinternal_border_width ());
      haiku_clear_under_internal_border (f);
    }

  SET_FRAME_GARBAGED! (f);
}

// C 1797-1804.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_frame_visible_invisible (f: *mut frame, visible_p: bool)
{
  if visible_p
    { haiku_visualize_frame (f); }
  else
    { haiku_unvisualize_frame (f); }
}

// C 1806-1815.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_mouse_pixel_position (f: *mut frame, mut pix_x: c_int,
                                                        mut pix_y: c_int)
{
  block_input ();

  BView_convert_to_screen (FRAME_HAIKU_VIEW! (f), &raw mut pix_x, &raw mut pix_y);
  be_warp_pointer (pix_x, pix_y);

  unblock_input ();
}

// C 1817-1825.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_query_color (col: uint32_t, color_def: *mut Emacs_Color)
{
  (*color_def).red = (RED_FROM_ULONG! (col) * 257) as c_ushort;
  (*color_def).green = (GREEN_FROM_ULONG! (col) * 257) as c_ushort;
  (*color_def).blue = (BLUE_FROM_ULONG! (col) * 257) as c_ushort;

  (*color_def).pixel = col as c_ulong;
}

// C 1827-1830.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_x_display_info (object: Lisp_Object) -> *mut Display_Info
{
  return check_haiku_display_info (object);
}

/* Rename frame F to NAME.  If NAME is nil, set F's name to the
   default name.  If EXPLICIT_P is non-zero, that indicates Lisp code
   is setting the name, not redisplay; in that case, set F's name to
   NAME and set F->explicit_name; if NAME is nil, clear
   F->explicit_name.

   If EXPLICIT_P is zero, it means redisplay is setting the name; the
   name provided will be ignored if explicit_name is set.  */
// C 1841-1870.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_set_name (f: *mut frame, name: Lisp_Object, explicit_p: bool)
{
  let dpyinfo: *mut haiku_display_info;
  let mut name = name;

  if explicit_p
    {
      if explicit_name! (f) && NILP (name)
	{ update_mode_lines = 37; }

      explicit_name! (f, !NILP (name));
    }
  else if explicit_name! (f)
    { return; }

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  if NILP (name)
    { name = (*dpyinfo).default_name; }

  if !NILP (Fstring_equal (name, (*f).name))
    { return; }

  fset_name (f, name);

  if !NILP ((*f).title)
    { name = (*f).title; }

  haiku_set_title_bar_text (f, name);
}

// C 1872-1893.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_inhibit_double_buffering (f: *mut frame, new_value: Lisp_Object,
                                              new_value_old: Lisp_Object)
{
  block_input ();
  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    {
      /* C 1879-1886: `#ifndef USE_BE_CAIRO` around the `if` and its `else`.  Both arms are
	 carried; with cairo there is no condition at all.  */
      #[cfg (feature = "be_cairo")]
      { EmacsView_set_up_double_buffering (FRAME_HAIKU_VIEW! (f)); }
      #[cfg (not (feature = "be_cairo"))]
      if NILP (new_value)
	{ EmacsView_set_up_double_buffering (FRAME_HAIKU_VIEW! (f)); }
      else
	{ EmacsView_disable_double_buffering (FRAME_HAIKU_VIEW! (f)); }

      SET_FRAME_GARBAGED! (f);
    }
  unblock_input ();
}

// C 1894-1899.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_sticky (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  block_input ();
  BWindow_set_sticky (FRAME_HAIKU_WINDOW! (f), !NILP (new_value));
  unblock_input ();
}

// C 1901-1914.
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct user_cursor_info
{
  /* A pointer to the Lisp_Object describing the cursor.  */
  pub lisp_cursor: *mut Lisp_Object,

  /* The offset of the cursor in the `struct haiku_output' of each
     frame.  */
  pub output_offset: ptrdiff_t,

  /* The offset of the default value of the cursor in the display
     info structure.  */
  pub default_offset: ptrdiff_t,
}

// C 1916-1931.
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct user_cursor_bitmap_info
{
  /* A bitmap to use instead of the font cursor to create cursors in a
     certain color.  */
  pub bits: *const c_void,

  /* The mask for that bitmap.  */
  pub mask: *const c_void,

  /* The dimensions of the cursor bitmap.  */
  pub width: c_int,
  pub height: c_int,

  /* The position inside the cursor bitmap corresponding to the
     position of the mouse pointer.  */
  pub x: c_int,
  pub y: c_int,
}

// C 1933-1935.
#[cfg (feature = "have_haiku")]
macro_rules! INIT_USER_CURSOR {
  ($lisp:expr, $cursor:ident) => {
    user_cursor_info { lisp_cursor: $lisp,
      output_offset: core::mem::offset_of! (haiku_output, $cursor) as ptrdiff_t,
      default_offset: core::mem::offset_of! (haiku_display_info, $cursor) as ptrdiff_t }
  };
}

// C 1937-1955.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut custom_cursors: [user_cursor_info; 16] =
  [
    INIT_USER_CURSOR! (&raw mut Vx_pointer_shape,		text_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			nontext_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			modeline_cursor),
    INIT_USER_CURSOR! (&raw mut Vx_sensitive_text_pointer_shape, hand_cursor),
    INIT_USER_CURSOR! (&raw mut Vx_hourglass_pointer_shape,	hourglass_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			horizontal_drag_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			vertical_drag_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			left_edge_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			top_left_corner_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			top_edge_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			top_right_corner_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			right_edge_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			bottom_right_corner_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			bottom_edge_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			bottom_left_corner_cursor),
    INIT_USER_CURSOR! (std::ptr::null_mut (),			no_cursor),
  ];

// C 1957-1978.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut cursor_bitmaps: [user_cursor_bitmap_info; 16] =
  [
    user_cursor_bitmap_info { bits: &raw const ibeam_ptr_bits as *const c_void,
      mask: &raw const ibeam_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: &raw const left_ptr_bits as *const c_void,
      mask: &raw const left_ptrmsk_bits as *const c_void, width: 16, height: 16, x: 3, y: 1 },
    user_cursor_bitmap_info { bits: &raw const left_ptr_bits as *const c_void,
      mask: &raw const left_ptrmsk_bits as *const c_void, width: 16, height: 16, x: 3, y: 1 },
    user_cursor_bitmap_info { bits: &raw const hand_ptr_bits as *const c_void,
      mask: &raw const hand_ptrmask_bits as *const c_void, width: 15, height: 15, x: 4, y: 3 },
    user_cursor_bitmap_info { bits: &raw const hourglass_bits as *const c_void,
      mask: &raw const hourglass_mask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: &raw const horizd_ptr_bits as *const c_void,
      mask: &raw const horizd_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: &raw const vertd_ptr_bits as *const c_void,
      mask: &raw const vertd_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (),
      width: 0, height: 0, x: 0, y: 0 },
  ];

/* Array of cursor bitmaps for each system cursor ID.  This is used to
   color in user-specified cursors.  */
// C 1979-2010.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut cursor_bitmaps_for_id: [user_cursor_bitmap_info; 28] =
  [
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: &raw const left_ptr_bits as *const c_void,
      mask: &raw const left_ptrmsk_bits as *const c_void, width: 16, height: 16, x: 3, y: 1 },
    user_cursor_bitmap_info { bits: &raw const ibeam_ptr_bits as *const c_void,
      mask: &raw const ibeam_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: &raw const cross_ptr_bits as *const c_void,
      mask: &raw const cross_ptrmask_bits as *const c_void, width: 30, height: 30, x: 15, y: 15 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: &raw const hand_ptr_bits as *const c_void,
      mask: &raw const hand_ptrmask_bits as *const c_void, width: 15, height: 15, x: 4, y: 3 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: &raw const hourglass_bits as *const c_void,
      mask: &raw const hourglass_mask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: &raw const horizd_ptr_bits as *const c_void,
      mask: &raw const horizd_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: &raw const vertd_ptr_bits as *const c_void,
      mask: &raw const vertd_ptrmask_bits as *const c_void, width: 15, height: 15, x: 7, y: 7 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
    user_cursor_bitmap_info { bits: std::ptr::null (), mask: std::ptr::null (), width: 0, height: 0, x: 0, y: 0 },
  ];

// C 2012-2054.
#[cfg (feature = "have_haiku")]
unsafe fn haiku_create_colored_cursor (info: *mut user_cursor_bitmap_info,
                                      foreground: uint32_t, background: uint32_t)
                                      -> *mut c_void
{
  let mut bits: *const c_char;
  let mut mask: *const c_char;
  let bitmap: *mut c_void;
  let cursor: *mut c_void;
  let width: c_int;
  let height: c_int;
  let bytes_per_line: c_int;
  let mut x: c_int;
  let mut y: c_int;

  bits = (*info).bits as *const c_char;
  mask = (*info).mask as *const c_char;
  width = (*info).width;
  height = (*info).height;
  bytes_per_line = (width + 7) / 8;

  bitmap = BBitmap_new (width, height, false as c_int);

  if bitmap.is_null ()
    { memory_full_up (); }

  y = 0;
  while y < height
    {
      x = 0;
      while x < width
	{
	  if (*mask.offset ((x / 8) as isize) as c_int) >> (x % 8) & 1 != 0
	    { haiku_put_pixel (bitmap, x, y,
			     (if (*bits.offset ((x / 8) as isize) as c_int) >> (x % 8) & 1 != 0
			      { (foreground | 255u32 << 24) as c_ulong }
			      else { (background | 255u32 << 24) as c_ulong })); }
	  else
	    { haiku_put_pixel (bitmap, x, y, 0); }
	  x += 1;
	}

      mask = mask.offset (bytes_per_line as isize);
      bits = bits.offset (bytes_per_line as isize);
      y += 1;
    }

  cursor = be_create_pixmap_cursor (bitmap, (*info).x, (*info).y);
  BBitmap_free (bitmap);

  return cursor;
}

/* Free all cursors on F that were allocated specifically for the
   frame.  */
// C 2056-2087.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_free_custom_cursors (f: *mut frame)
{
  let mut cursor: *mut user_cursor_info;
  let output: *mut haiku_output;
  let dpyinfo: *mut haiku_display_info;
  let mut frame_cursor: *mut Emacs_Cursor;
  let mut display_cursor: *mut Emacs_Cursor;
  let mut i: c_int;

  output = FRAME_OUTPUT_DATA! (f) as *mut haiku_output;
  dpyinfo = FRAME_DISPLAY_INFO! (f);

  i = 0;
  while i < countof! (custom_cursors) as c_int
    {
      cursor = &raw mut custom_cursors [i as usize];
      frame_cursor = (output as *mut c_char).offset ((*cursor).output_offset as isize)
		     as *mut Emacs_Cursor;
      display_cursor = (dpyinfo as *mut c_char).offset ((*cursor).default_offset as isize)
		       as *mut Emacs_Cursor;

      if *frame_cursor != *display_cursor && !(*frame_cursor).is_null ()
	{
	  if (*output).current_cursor == *frame_cursor
	    { (*output).current_cursor = *display_cursor; }

	  be_delete_cursor (*frame_cursor);
	}

      *frame_cursor = *display_cursor;
      i += 1;
    }
}

// C 2089-2174.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_mouse_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let output: *mut haiku_output;
  /* C leaves `frame_cursor` uninitialised and only reads it after its loop; the port zeroes it
     (PORTING.md 1.5 - the loop always runs, `countof (custom_cursors)` is 16).  */
  let mut frame_cursor: *mut Emacs_Cursor = std::ptr::null_mut ();
  let mut old: Emacs_Cursor = std::ptr::null_mut ();
  /* C declares this `Emacs_Cursor *recolored` and assigns `haiku_create_colored_cursor`'s
     `void *` to it (an implicit `void *` conversion C permits and this port cannot); the value
     C actually stores is the cursor itself, so the port gives the binding `Emacs_Cursor`
     (a recorded deviation).  */
  let mut recolored: Emacs_Cursor;
  let mut i: c_int;
  let mut n: c_int = 0;
  let mut rc: c_int = 0;
  let mut color_specified_p: bool;
  let mut color: Emacs_Color = std::mem::zeroed ();

  CHECK_STRING (arg);
  color_specified_p = true;

  if strcmp (SSDATA (arg), c"font-color".as_ptr ()) == 0
    { color_specified_p = false; }
  else
    { rc = haiku_get_color (SSDATA (arg), &raw mut color); }

  if color_specified_p && rc != 0
    { signal_error (c"Undefined color".as_ptr (), arg); }

  output = FRAME_OUTPUT_DATA! (f) as *mut haiku_output;

  /* This will also reset all the cursors back to their default
     values.  */
  haiku_free_custom_cursors (f);

  i = 0;
  while i < countof! (custom_cursors) as c_int
    {
      frame_cursor = (output as *mut c_char).offset (custom_cursors [i as usize].output_offset
						     as isize) as *mut Emacs_Cursor;
      old = *frame_cursor;

      if !custom_cursors [i as usize].lisp_cursor.is_null ()
	  && FIXNUMP (*custom_cursors [i as usize].lisp_cursor)
	{
	  if !RANGED_FIXNUMP (0, *custom_cursors [i as usize].lisp_cursor,
			      28) /* 28 is the largest Haiku cursor ID.  */
	    { signal_error (c"Invalid cursor".as_ptr (),
			  *custom_cursors [i as usize].lisp_cursor); }

	  n = XFIXNUM (*custom_cursors [i as usize].lisp_cursor) as c_int;

	  if color_specified_p && !cursor_bitmaps_for_id [n as usize].bits.is_null ()
	    {
	      recolored
		= haiku_create_colored_cursor (&raw mut cursor_bitmaps_for_id [n as usize],
					       color.pixel as uint32_t,
					       FRAME_BACKGROUND_PIXEL (f) as uint32_t)
		  as Emacs_Cursor;

	      if !recolored.is_null ()
		{
		  *frame_cursor = recolored;
		  i += 1;
		  continue;
		}
	    }

	  /* Create and set the custom cursor.  */
	  *frame_cursor = be_create_cursor_from_id (n);
	}
      else if color_specified_p && !cursor_bitmaps [i as usize].bits.is_null ()
	{
	  recolored
	    = haiku_create_colored_cursor (&raw mut cursor_bitmaps [i as usize],
					   color.pixel as uint32_t,
					   FRAME_BACKGROUND_PIXEL (f) as uint32_t)
	      as Emacs_Cursor;

	  if !recolored.is_null ()
	    { *frame_cursor = recolored; }
	}
      i += 1;
    }

  /* This function can be called before the frame's window is
     created.  */
  if !FRAME_HAIKU_WINDOW! (f).is_null ()
    {
      if (*output).current_cursor == old
	  && old != *frame_cursor
	{
	  (*output).current_cursor = *frame_cursor;

	  BView_set_view_cursor (FRAME_HAIKU_VIEW! (f), *frame_cursor);
	}
    }

  update_face_from_frame_parameter (f, Qmouse_color (), arg);
}

// C 2176-2181.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_set_use_frame_synchronization (f: *mut frame, arg: Lisp_Object,
                                              oldval: Lisp_Object)
{
  be_set_use_frame_synchronization (FRAME_HAIKU_VIEW! (f), !NILP (arg));
}

// C 2183-2188.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_should_pass_control_tab_to_system () -> bool
{
  return haiku_pass_control_tab_to_system;
}

// C 2190-2208.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-set-mouse-absolute-pixel-position",
       Fhaiku_set_mouse_absolute_pixel_position,
       Shaiku_set_mouse_absolute_pixel_position, 2, 2, 0,
       doc: "Move mouse pointer to a pixel position at (X, Y).  The\ncoordinates X and Y are interpreted to start from the top-left\ncorner of the screen.  ",
       (x: Lisp_Object, y: Lisp_Object),
       {
  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

  if x_display_list.is_null ()
    { error (c"Window system not initialized".as_ptr ()); }

  block_input ();
  be_warp_pointer (xval, yval);
  unblock_input ();
  return Qnil ();
});

// C 2210-2232.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-mouse-absolute-pixel-position", Fhaiku_mouse_absolute_pixel_position,
       Shaiku_mouse_absolute_pixel_position, 0, 0, 0,
       doc: "Return absolute position of mouse cursor in pixels.\nThe position is returned as a cons cell (X . Y) of the coordinates of\nthe mouse cursor position in pixels relative to a position (0, 0) of the\nselected frame's display.  ",
       (),
       {
  let f: *mut frame = SELECTED_FRAME ();
  let view: *mut c_void;
  let mut x: c_int = 0;
  let mut y: c_int = 0;

  if FRAME_INITIAL_P (f) || !FRAME_HAIKU_P (f)
    { return Qnil (); }

  block_input ();
  view = FRAME_HAIKU_VIEW! (f);
  BView_get_mouse (view, &raw mut x, &raw mut y);
  BView_convert_to_screen (view, &raw mut x, &raw mut y);
  unblock_input ();

  return Fcons (make_fixnum (x as EMACS_INT), make_fixnum (y as EMACS_INT));
});

// C 2234-2241.
#[cfg (feature = "have_haiku")]
DEFUN! ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);

  return if be_is_display_grayscale () { Qnil () } else { Qt () };
});

// C 2243-2253.
#[cfg (feature = "have_haiku")]
DEFUN! ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p, 1, 2, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (color: Lisp_Object, frame: Lisp_Object),
       {
  let mut col: Emacs_Color = std::mem::zeroed ();

  CHECK_STRING (color);
  decode_window_system_frame (frame);

  return if haiku_get_color (SSDATA (color), &raw mut col) != 0 { Qnil () } else { Qt () };
});

// C 2255-2273.
#[cfg (feature = "have_haiku")]
DEFUN! ("xw-color-values", Fxw_color_values, Sxw_color_values, 1, 2, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (color: Lisp_Object, frame: Lisp_Object),
       {
  let mut col: Emacs_Color = std::mem::zeroed ();
  let rc: c_int;

  CHECK_STRING (color);
  decode_window_system_frame (frame);

  block_input ();
  rc = haiku_get_color (SSDATA (color), &raw mut col);
  unblock_input ();

  if rc != 0
    { return Qnil (); }

  return list3i (col.red as EMACS_INT, col.green as EMACS_INT, col.blue as EMACS_INT);
});

// C 2275-2283.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);

  return if be_is_display_grayscale () { Qt () } else { Qnil () };
});

// C 2285-2309.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-open-connection", Fx_open_connection, Sx_open_connection,
       1, 3, 0, doc: "SKIP: real doc in xfns.c.  ",
       (display: Lisp_Object, resource_string: Lisp_Object, must_succeed: Lisp_Object),
       {
  CHECK_STRING (display);

  if NILP (Fstring_equal (display, build_string (c"be".as_ptr ())))
    {
      if !NILP (must_succeed)
	{ fatal (c"Invalid display %s".as_ptr (), SDATA (display)); }
      else
	{ signal_error (c"Invalid display".as_ptr (), display); }
    }

  if !x_display_list.is_null ()
    {
      if !NILP (must_succeed)
	{ fatal (c"A display is already open".as_ptr ()); }
      else
	{ error (c"A display is already open".as_ptr ()); }
    }

  haiku_term_init ();
  return Qnil ();
});

// C 2311-2322.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  check_haiku_display_info (terminal);

  be_get_screen_dimensions (&raw mut width, &raw mut height);
  return make_fixnum (width as EMACS_INT);
});

// C 2324-2335.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  check_haiku_display_info (terminal);

  be_get_screen_dimensions (&raw mut width, &raw mut height);
  return make_fixnum (height as EMACS_INT);
});

// C 2337-2347.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let dpyinfo: *mut haiku_display_info = check_haiku_display_info (terminal);
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  be_get_screen_dimensions (&raw mut width, &raw mut height);
  return make_fixnum ((height as c_double / ((*dpyinfo).resy / 25.4)) as EMACS_INT);
});

// C 2349-2358.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let dpyinfo: *mut haiku_display_info = check_haiku_display_info (terminal);
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  be_get_screen_dimensions (&raw mut width, &raw mut height);
  return make_fixnum ((width as c_double / ((*dpyinfo).resx / 25.4)) as EMACS_INT);
});

// C 2360-2366.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-create-frame", Fx_create_frame, Sx_create_frame,
       1, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (parms: Lisp_Object),
       {
  return haiku_create_frame (parms);
});

// C 2368-2387.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-visual-class", Fx_display_visual_class,
       Sx_display_visual_class, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let planes: c_int;
  let grayscale_p: bool;

  check_haiku_display_info (terminal);

  grayscale_p = be_is_display_grayscale ();
  if grayscale_p
    { return Qstatic_gray (); }

  planes = be_get_display_planes ();
  if planes == 8
    { return Qstatic_color (); }

  return Qtrue_color ();
});

// C 2389-2701.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-show-tip", Fx_show_tip, Sx_show_tip, 1, 6, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (string: Lisp_Object, frame: Lisp_Object, parms: Lisp_Object,
        timeout: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object),
       {
  let mut f: *mut frame;
  let tip_f: *mut frame;
  let mut w: *mut window;
  let mut root_x: c_int = 0;
  let mut root_y: c_int = 0;
  let old_buffer: *mut buffer;
  let mut pos: text_pos = std::mem::zeroed ();
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  let old_windows_or_buffers_changed: c_int = windows_or_buffers_changed;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut window: Lisp_Object;
  let size: Lisp_Object;
  let tip_buf: Lisp_Object;
  let displayed: bool;
  /* C 2404-2406: `#ifdef ENABLE_CHECKING` around the two row locals.  */
  #[cfg (feature = "enable_checking")]
  let mut row: *mut glyph_row = std::ptr::null_mut ();
  #[cfg (feature = "enable_checking")]
  let end: *mut glyph_row;
  /* C's two `goto start_timer` jumps (C 2472/2497) and its label (C 2693) become this labelled
     block, whose `break 'start_timer` is the port's spelling of the jump (PORTING.md 2.2, the
     `src/xselect.rs` `'done` precedent).  */
  AUTO_STRING! (tip, c" *tip*".as_ptr ());

  let mut string = string;
  let mut frame = frame;
  let mut parms = parms;
  let mut timeout = timeout;
  let mut dx = dx;
  let mut dy = dy;

  'start_timer: {
  specbind (Qinhibit_redisplay (), Qt ());

  CHECK_STRING (string);
  if SCHARS (string) == 0
    { string = make_unibyte_string (c" ".as_ptr (), 1); }

  if NILP (frame)
    { frame = selected_frame; }
  f = decode_window_system_frame (frame);

  if NILP (timeout)
    { timeout = Vx_show_tooltip_timeout; }
  CHECK_FIXNAT (timeout);

  if NILP (dx)
    { dx = make_fixnum (5); }
  else
    { CHECK_FIXNUM (dx); }

  if NILP (dy)
    { dy = make_fixnum (-10); }
  else
    { CHECK_FIXNUM (dy); }

  tip_dx = dx;
  tip_dy = dy;

  if use_system_tooltips
    {
      CHECK_STRING (string);
      if STRING_MULTIBYTE (string)
	{ string = ENCODE_UTF_8 (string); }

      if NILP (frame)
	{ frame = selected_frame; }

      let f: *mut frame = decode_window_system_frame (frame);
      block_input ();

      let str_: *mut c_char = xstrdup (SSDATA (string));
      let mut height: c_int = be_plain_font_height ();
      let mut width: c_int;
      let mut tok: *mut c_char = strtok (str_, c"\n".as_ptr ());
      width = be_string_width_with_plain_font (tok);

      loop
	{
	  tok = strtok (std::ptr::null_mut (), c"\n".as_ptr ());
	  if tok.is_null ()
	    { break; }
	  height = be_plain_font_height ();
	  let mut w: c_int = be_string_width_with_plain_font (tok);
	  if w > width
	    { w = width; }
	}
      free (str_ as *mut c_void);

      height += 16; /* Default margin.  */
      width += 16; /* Ditto.  Unfortunately there isn't a more
		      reliable way to get it.  */
      compute_tip_xy (f, parms, dx, dy, width, height, &raw mut root_x, &raw mut root_y);
      BView_convert_from_screen (FRAME_HAIKU_VIEW! (f), &raw mut root_x, &raw mut root_y);
      be_show_sticky_tooltip (FRAME_HAIKU_VIEW! (f), SSDATA (string),
			      root_x, root_y);
      unblock_input ();
      break 'start_timer;
    }

  if !NILP (tip_frame) && FRAME_LIVE_P (XFRAME (tip_frame))
    {
      if FRAME_VISIBLE_P (XFRAME (tip_frame))
	  && !NILP (Fequal_including_properties (tip_last_string, string))
	  && !NILP (Fequal (tip_last_parms, parms))
	{
	  /* Only DX and DY have changed.  */
	  tip_f = XFRAME (tip_frame);
	  if !NILP (tip_timer)
	    {
	      calln! (Qcancel_timer (), tip_timer);
	      tip_timer = Qnil ();
	    }

	  block_input ();
	  compute_tip_xy (tip_f, parms, dx, dy, FRAME_PIXEL_WIDTH (tip_f),
			  FRAME_PIXEL_HEIGHT (tip_f), &raw mut root_x, &raw mut root_y);
	  BWindow_set_offset (FRAME_HAIKU_WINDOW! (tip_f), root_x, root_y);
	  unblock_input ();

	  break 'start_timer;
	}
      else if tooltip_reuse_hidden_frame && EQ (frame, tip_last_frame)
	{
	  let mut delete: bool = false;
	  let mut tail: Lisp_Object;
	  let mut elt: Lisp_Object;
	  let mut parm: Lisp_Object;
	  let mut last: Lisp_Object;

	  /* Check if every parameter in PARMS has the same value in
	     tip_last_parms.  This may destruct tip_last_parms which,
	     however, will be recreated below.  */
	  tail = parms;
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);
	      parm = Fcar (elt);
	      /* The left, top, right and bottom parameters are handled
		 by compute_tip_xy so they can be ignored here.  */
	      if !EQ (parm, Qleft ()) && !EQ (parm, Qtop ())
		  && !EQ (parm, Qright ()) && !EQ (parm, Qbottom ())
		{
		  last = Fassq (parm, tip_last_parms);
		  if NILP (Fequal (Fcdr (elt), Fcdr (last)))
		    {
		      /* We lost, delete the old tooltip.  */
		      delete = true;
		      break;
		    }
		  else
		    { tip_last_parms = calln! (Qassq_delete_all (), parm, tip_last_parms); }
		}
	      else
		{ tip_last_parms = calln! (Qassq_delete_all (), parm, tip_last_parms); }
	      tail = XCDR (tail);
	    }

	  /* Now check if every parameter in what is left of
	     tip_last_parms with a non-nil value has an association in
	     PARMS.  */
	  tail = tip_last_parms;
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);
	      parm = Fcar (elt);
	      if !EQ (parm, Qleft ()) && !EQ (parm, Qtop ()) && !EQ (parm, Qright ())
		  && !EQ (parm, Qbottom ()) && !NILP (Fcdr (elt))
		{
		  /* We lost, delete the old tooltip.  */
		  delete = true;
		  break;
		}
	      tail = XCDR (tail);
	    }

	  haiku_hide_tip (delete);
	}
      else
	{ haiku_hide_tip (true); }
    }
  else
    { haiku_hide_tip (true); }

  tip_last_frame = frame;
  tip_last_string = string;
  tip_last_parms = parms;

  if NILP (tip_frame) || !FRAME_LIVE_P (XFRAME (tip_frame))
    {
      /* Add default values to frame parameters.  */
      if NILP (Fassq (Qname (), parms))
	{ parms = Fcons (Fcons (Qname (), build_string (c"tooltip".as_ptr ())), parms); }
      if NILP (Fassq (Qinternal_border_width (), parms))
	{ parms = Fcons (Fcons (Qinternal_border_width (), make_fixnum (3)), parms); }
      if NILP (Fassq (Qborder_width (), parms))
	{ parms = Fcons (Fcons (Qborder_width (), make_fixnum (1)), parms); }
      if NILP (Fassq (Qborder_color (), parms))
	{ parms = Fcons (Fcons (Qborder_color (), build_string (c"lightyellow".as_ptr ())),
			 parms); }
      if NILP (Fassq (Qbackground_color (), parms))
	{ parms = Fcons (Fcons (Qbackground_color (),
				build_string (c"lightyellow".as_ptr ())), parms); }

      /* Create a frame for the tooltip, and record it in the global
	 variable tip_frame.  */
      tip_frame = haiku_create_tip_frame (parms);
      if NILP (tip_frame)
	/* Creating the tip frame failed.  */
	{ return unbind_to (count, Qnil ()); }
    }

  tip_f = XFRAME (tip_frame);
  window = FRAME_ROOT_WINDOW (tip_f);
  tip_buf = Fget_buffer_create (tip, Qnil ());
  /* We will mark the tip window a "pseudo-window" below, and such
     windows cannot have display margins.  */
  bset_left_margin_cols (XBUFFER (tip_buf), make_fixnum (0));
  bset_right_margin_cols (XBUFFER (tip_buf), make_fixnum (0));
  set_window_buffer (window, tip_buf, false, false);
  w = XWINDOW (window);
  window_bits! (pseudo_window_p, w, true);
  /* Try to avoid that `other-window' select us (Bug#47207).  */
  Fset_window_parameter (window, Qno_other_window (), Qt ());

  /* Set up the frame's root window.  Note: The following code does not
     try to size the window or its frame correctly.  Its only purpose is
     to make the subsequent text size calculations work.  The right
     sizes should get installed when the toolkit gets back to us.  */
  (*w).left_col = 0;
  (*w).top_line = 0;
  (*w).pixel_left = 0;
  (*w).pixel_top = 0;

  if CONSP (Vx_max_tooltip_size)
      && RANGED_FIXNUMP (1, XCAR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)
      && RANGED_FIXNUMP (1, XCDR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)
    {
      (*w).total_cols = XFIXNAT (XCAR (Vx_max_tooltip_size)) as c_int;
      (*w).total_lines = XFIXNAT (XCDR (Vx_max_tooltip_size)) as c_int;
    }
  else
    {
      (*w).total_cols = 80;
      (*w).total_lines = 40;
    }

  (*w).pixel_width = (*w).total_cols * FRAME_COLUMN_WIDTH (tip_f);
  (*w).pixel_height = (*w).total_lines * FRAME_LINE_HEIGHT (tip_f);
  FRAME_TOTAL_COLS! (tip_f, (*w).total_cols);
  adjust_frame_glyphs (tip_f);

  /* Insert STRING into root window's buffer and fit the frame to the
     buffer.  */
  let count_1: specpdl_ref = SPECPDL_INDEX ();
  old_buffer = (current_buffer! ());
  set_buffer_internal_1 (XBUFFER ((*w).contents));
  bset_truncate_lines ((current_buffer! ()), Qnil ());
  specbind (Qinhibit_read_only (), Qt ());
  specbind (Qinhibit_modification_hooks (), Qt ());
  specbind (Qinhibit_point_motion_hooks (), Qt ());
  Ferase_buffer ();
  Finsert (1, &raw mut string);
  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
  clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
  SET_TEXT_POS! (pos, BEGV! (), BEGV_BYTE! ());
  displayed = try_window (window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE) != 0;

  if !displayed && NILP (Vx_max_tooltip_size)
    {
      /* C 2638-2651: `#ifdef ENABLE_CHECKING`.  */
      #[cfg (feature = "enable_checking")]
      {
	row = (*((*w).desired_matrix as *mut glyph_matrix)).rows;
	end = (*((*w).desired_matrix as *mut glyph_matrix)).rows
	  .offset ((*((*w).desired_matrix as *mut glyph_matrix)).nrows as isize);

	while row < end
	  {
	    if !glyph_row_bits! (displays_text_p, row)
		|| glyph_row_bits! (ends_at_zv_p, row)
	      { break; }
	    row = row.offset (1);
	  }

	eassert! (row < end && glyph_row_bits! (ends_at_zv_p, row));
      }
    }

  /* Calculate size of tooltip window.  */
  size = Fwindow_text_pixel_size (window, Qnil (), Qnil (), Qnil (),
				  make_fixnum ((*w).pixel_height as EMACS_INT), Qnil (),
				  Qnil ());
  /* Add the frame's internal border to calculated size.  */
  width = XFIXNUM (Fcar (size)) as c_int + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);
  height = XFIXNUM (Fcdr (size)) as c_int + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);

  /* Calculate position of tooltip frame.  */
  compute_tip_xy (tip_f, parms, dx, dy, width, height, &raw mut root_x, &raw mut root_y);

  /* Show tooltip frame.  */
  block_input ();
  let wnd: *mut c_void = FRAME_HAIKU_WINDOW! (tip_f);
  BWindow_resize (wnd, width, height);
  /* The window decorator might cause the actual width and height to
     be larger than WIDTH and HEIGHT, so use the actual sizes.  */
  BWindow_dimensions (wnd, &raw mut width, &raw mut height);
  BView_resize_to (FRAME_HAIKU_VIEW! (tip_f), width, height);
  BView_set_view_cursor (FRAME_HAIKU_VIEW! (tip_f),
			 (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).current_cursor);
  BWindow_set_offset (wnd, root_x, root_y);
  BWindow_set_visible (wnd, true as c_int);
  SET_FRAME_VISIBLE! (tip_f, true);
  FRAME_PIXEL_WIDTH! (tip_f, width);
  FRAME_PIXEL_HEIGHT! (tip_f, height);
  BWindow_sync (wnd);

  /* This is needed because the app server resets the cursor whenever
     a new window is mapped, so we won't see the cursor set on the
     tooltip if the mouse pointer isn't actually over it.  */
  BView_set_view_cursor (FRAME_HAIKU_VIEW! (f),
			 (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).current_cursor);
  unblock_input ();

  window_bits! (must_be_updated_p, w, true);
  update_single_window (w);
  flush_frame! (tip_f);
  set_buffer_internal_1 (old_buffer);
  unbind_to (count_1, Qnil ());
  windows_or_buffers_changed = old_windows_or_buffers_changed;

  }

  /* Let the tip disappear after timeout seconds.  */
  tip_timer = calln! (Qrun_at_time (), timeout, Qnil (), Qx_hide_tip ());

  return unbind_to (count, Qnil ());
});

// C 2703-2708.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (),
       {
  return haiku_hide_tip (!tooltip_reuse_hidden_frame);
});

// C 2710-2718.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-close-connection", Fx_close_connection, Sx_close_connection, 1, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       attributes: noreturn,
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);

  error (c"Cannot close Haiku displays".as_ptr ());
});

// C 2720-2728.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-list", Fx_display_list, Sx_display_list, 0, 0, 0,
       doc: "SKIP: real doc in xfns.c.",
       (),
       {
  if x_display_list.is_null ()
    { return Qnil (); }

  return list1 (XCAR ((*x_display_list).name_list_element));
});

// C 2730-2736.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-server-vendor", Fx_server_vendor, Sx_server_vendor, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);
  return build_string (c"Haiku, Inc.".as_ptr ());
});

// C 2738-2744.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-server-version", Fx_server_version, Sx_server_version, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);
  return list3i (5, 1, 1);
});

// C 2746-2752.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-screens", Fx_display_screens, Sx_display_screens, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);
  return make_fixnum (be_get_display_screens () as EMACS_INT);
});

// C 2754-2763.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-get-version-string", Fhaiku_get_version_string,
       Shaiku_get_version_string, 0, 0, 0,
       doc: "Return a string describing the current Haiku version.  ",
       (),
       {
  let mut buf: [c_char; 1024] = [0; 1024];

  be_get_version_string (&raw mut buf as *mut c_char, std::mem::size_of_val (&buf) as c_int);
  return build_string (buf.as_ptr ());
});

// C 2765-2773.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);

  return make_fixnum (be_get_display_color_cells () as EMACS_INT);
});

// C 2775-2783.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-planes", Fx_display_planes, Sx_display_planes,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  check_haiku_display_info (terminal);

  return make_fixnum (be_get_display_planes () as EMACS_INT);
});

// C 2785-2793.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-double-buffered-p", Fx_double_buffered_p, Sx_double_buffered_p,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_window_system_frame (frame);

  return if EmacsView_double_buffered_p (FRAME_HAIKU_VIEW! (f)) != 0 { Qt () }
    else { Qnil () };
});

// C 2795-2817.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store,
       0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let f: *mut frame;

  if FRAMEP (terminal)
    {
      f = decode_window_system_frame (terminal);

      if !FRAME_HAIKU_VIEW! (f).is_null ()
	  && EmacsView_double_buffered_p (FRAME_HAIKU_VIEW! (f)) != 0
	{ return if !FRAME_PARENT_FRAME (f).is_null () { Qwhen_mapped () } else { Qalways () }; }
      else
	{ return Qnot_useful (); }
    }
  else
    {
      check_haiku_display_info (terminal);
      return Qnot_useful ();
    }
});

// C 2819-2861.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-frame-geometry", Fhaiku_frame_geometry, Shaiku_frame_geometry, 0, 1, 0,
       doc: "Return geometric attributes of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is an association list of the attributes listed below.  All height\nand width values are in pixels.\n\n`outer-position' is a cons of the outer left and top edges of FRAME\n  relative to the origin - the position (0, 0) - of FRAME's display.\n\n`outer-size' is a cons of the outer width and height of FRAME.  The\n  outer size includes the title bar and the external borders as well as\n  any menu and/or tool bar of frame.\n\n`external-border-size' is a cons of the horizontal and vertical width of\n  FRAME's external borders as supplied by the window manager.\n\n`title-bar-size' is a cons of the width and height of the title bar of\n  FRAME as supplied by the window manager.  If both of them are zero,\n  FRAME has no title bar.  If only the width is zero, Emacs was not\n  able to retrieve the width information.\n\n`menu-bar-external', if non-nil, means the menu bar is external (never\n  included in the inner edges of FRAME).\n\n`menu-bar-size' is a cons of the width and height of the menu bar of\n  FRAME.\n\n`tool-bar-external', if non-nil, means the tool bar is external (never\n  included in the inner edges of FRAME).\n\n`tool-bar-position' tells on which side the tool bar on FRAME is and can\n  be one of `left', `top', `right' or `bottom'.  If this is nil, FRAME\n  has no tool bar.\n\n`tool-bar-size' is a cons of the width and height of the tool bar of\n  FRAME.\n\n`internal-border-width' is the width of the internal border of\n  FRAME.  ",
       (frame: Lisp_Object),
       {
  return frame_geometry (frame, Qnil ());
});

// C 2863-2885.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-frame-edges", Fhaiku_frame_edges, Shaiku_frame_edges, 0, 2, 0,
       doc: "Return edge coordinates of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a list of the form (LEFT, TOP, RIGHT, BOTTOM).  All values are\nin pixels relative to the origin - the position (0, 0) - of FRAME's\ndisplay.\n\nIf optional argument TYPE is the symbol `outer-edges', return the outer\nedges of FRAME.  The outer edges comprise the decorations of the window\nmanager (like the title bar or external borders) as well as any external\nmenu or tool bar of FRAME.  If optional argument TYPE is the symbol\n`native-edges' or nil, return the native edges of FRAME.  The native\nedges exclude the decorations of the window manager and any external\nmenu or tool bar of FRAME.  If TYPE is the symbol `inner-edges', return\nthe inner edges of FRAME.  These edges exclude title bar, any borders,\nmenu bar or tool bar of FRAME.  ",
       (frame: Lisp_Object, r#type: Lisp_Object),
       {
  return frame_geometry (frame, if EQ (r#type, Qouter_edges ())
				  || EQ (r#type, Qinner_edges ())
				 { r#type }
				 else { Qnative_edges () });
});

// C 2887-2944.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-read-file-name", Fhaiku_read_file_name, Shaiku_read_file_name, 1, 6, 0,
       doc: "Use a graphical panel to read a file name, using prompt PROMPT.\nOptional arg FRAME specifies a frame on which to display the file panel.\nIf it is nil, the current frame is used instead.\nThe frame being used will be brought to the front of\nthe display after the file panel is closed.\nOptional arg DIR, if non-nil, supplies a default directory.\nOptional arg MUSTMATCH, if non-nil, means the returned file or\ndirectory must exist.\nOptional arg DIR_ONLY_P, if non-nil, means choose only directories.\nOptional arg SAVE_TEXT, if non-nil, specifies some text to show in the entry field.  ",
       (prompt: Lisp_Object, frame: Lisp_Object, dir: Lisp_Object,
        mustmatch: Lisp_Object, dir_only_p: Lisp_Object, save_text: Lisp_Object),
       {
  let f: *mut frame;
  let file_name: *mut c_char;
  let value: Lisp_Object;

  let mut dir = dir;
  let mut frame = frame;

  if popup_activated_p != 0
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }

  if !NILP (dir)
    {
      CHECK_STRING (dir);
      dir = ENCODE_FILE (dir);
    }

  if !NILP (save_text)
    { CHECK_STRING (save_text); }

  if NILP (frame)
    { frame = selected_frame; }

  CHECK_STRING (prompt);

  f = decode_window_system_frame (frame);

  popup_activated_p += 1;
  unrequest_sigio ();
  file_name = be_popup_file_dialog ((!NILP (mustmatch) || !NILP (dir_only_p)) as c_int,
				    if !NILP (dir) { SSDATA (dir) } else { std::ptr::null () },
				    !NILP (mustmatch) as c_int, !NILP (dir_only_p) as c_int,
				    FRAME_HAIKU_WINDOW! (f),
				    if !NILP (save_text)
				      { SSDATA (ENCODE_UTF_8 (save_text)) }
				      else { std::ptr::null () },
				    SSDATA (ENCODE_UTF_8 (prompt)),
				    Some (process_pending_signals));
  request_sigio ();
  popup_activated_p -= 1;

  if file_name.is_null ()
    { quit (); }

  value = build_string (file_name);
  free (file_name as *mut c_void);

  return DECODE_FILE (value);
});

// C 2946-2957.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-put-resource", Fhaiku_put_resource, Shaiku_put_resource,
       2, 2, 0, doc: "Place STRING by the key RESOURCE in the resource database.\nIt can later be retrieved with `x-get-resource'.  ",
       (resource: Lisp_Object, string: Lisp_Object),
       {
  CHECK_STRING (resource);
  if !NILP (string)
    { CHECK_STRING (string); }

  put_xrm_resource (resource, string);
  return Qnil ();
});

// C 2959-2995.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-frame-list-z-order", Fhaiku_frame_list_z_order,
       Shaiku_frame_list_z_order, 0, 1, 0,
       doc: "Return list of Emacs's frames, in Z (stacking) order.\nIf TERMINAL is non-nil and specifies a live frame, return the child\nframes of that frame in Z (stacking) order.\n\nAs it is impossible to reliably determine the frame stacking order on\nHaiku, the selected frame is always the first element of the returned\nlist, while the rest are not guaranteed to be in any particular order.\n\nFrames are listed from topmost (first) to bottommost (last).  ",
       (terminal: Lisp_Object),
       {
  let mut frames: Lisp_Object = Qnil ();
  let mut head: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();
  let mut sel: Lisp_Object = Qnil ();

  FOR_EACH_FRAME! (head, tail, {
    {
      let f: *mut frame = XFRAME (tail);
      /* C's `continue` jumps to the `for` increment; the mirror's FOR_EACH_FRAME puts the
	 increment at the body's end, so the body is a labelled block named after C's own
	 keyword and `break 'continue_;` leaves it (the `src/androidterm.rs` 279-293 shape).  */
      'continue_: {
	if !FRAME_HAIKU_P (f)
	    || (FRAMEP (terminal)
		&& FRAME_LIVE_P (XFRAME (terminal))
		&& !EQ (terminal, get_frame_param (f, Qparent_frame ())))
	  { break 'continue_; }

	if EQ (tail, selected_frame)
	  { sel = tail; }
	else
	  { frames = Fcons (tail, frames); }
      }
    }
  });

  if NILP (sel)
    { return frames; }

  return Fcons (sel, frames);
});

// C 2997-3014.
#[cfg (feature = "have_haiku")]
DEFUN! ("x-display-save-under", Fx_display_save_under,
       Sx_display_save_under, 0, 1, 0,
       doc: "SKIP: real doc in xfns.c.  ",
       (terminal: Lisp_Object),
       {
  let f: *mut frame;
  check_haiku_display_info (terminal);

  if FRAMEP (terminal)
    {
      f = decode_window_system_frame (terminal);
      return if !FRAME_HAIKU_VIEW! (f).is_null ()
	       && EmacsView_double_buffered_p (FRAME_HAIKU_VIEW! (f)) != 0
	       { Qt () } else { Qnil () };
    }

  return Qnil ();
});

// C 3016-3072.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-frame-restack", Fhaiku_frame_restack, Shaiku_frame_restack, 2, 3, 0,
       doc: "Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nSome window managers may refuse to restack windows.  ",
       (frame1: Lisp_Object, frame2: Lisp_Object, above: Lisp_Object),
       {
  let f1: *mut frame = decode_window_system_frame (frame1);
  let f2: *mut frame = decode_window_system_frame (frame2);

  block_input ();

  if NILP (above)
    {
      /* If the window that is currently active will be sent behind
	 another window, make the window that it is being sent behind
	 active first, to avoid both windows being moved to the back of
	 the display.  */

      if BWindow_is_active (FRAME_HAIKU_WINDOW! (f1))
	  /* But don't do this if any of the frames involved have
	     child frames, since they are guaranteed to be in front of
	     their toplevel parents.  */
	  && FRAME_PARENT_FRAME (f1).is_null ()
	  && FRAME_PARENT_FRAME (f2).is_null ()
	{
	  BWindow_activate (FRAME_HAIKU_WINDOW! (f2));
	  BWindow_sync (FRAME_HAIKU_WINDOW! (f2));
	}

      BWindow_send_behind (FRAME_HAIKU_WINDOW! (f1),
			   FRAME_HAIKU_WINDOW! (f2));
    }
  else
    {
      if BWindow_is_active (FRAME_HAIKU_WINDOW! (f2))
	  && FRAME_PARENT_FRAME (f1).is_null ()
	  && FRAME_PARENT_FRAME (f2).is_null ()
	{
	  BWindow_activate (FRAME_HAIKU_WINDOW! (f1));
	  BWindow_sync (FRAME_HAIKU_WINDOW! (f1));
	}

      BWindow_send_behind (FRAME_HAIKU_WINDOW! (f2),
			   FRAME_HAIKU_WINDOW! (f1));
    }
  BWindow_sync (FRAME_HAIKU_WINDOW! (f1));
  BWindow_sync (FRAME_HAIKU_WINDOW! (f2));

  unblock_input ();

  return Qnil ();
});

// C 3074-3096.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-save-session-reply", Fhaiku_save_session_reply,
       Shaiku_save_session_reply, 1, 1, 0,
       doc: "Reply to a `save-session' event.\nQUIT-REPLY means whether or not all files were saved and program\ntermination should proceed.\n\nCalls to this function must be balanced by the amount of\n`save-session' events received.  This is done automatically, so do not\ncall this function yourself.  ",
       (quit_reply: Lisp_Object),
       {
  let mut reply: haiku_session_manager_reply = std::mem::zeroed ();
  reply.quit_reply = !NILP (quit_reply);

  block_input ();
  unrequest_sigio ();
  write_port (port_emacs_to_session_manager, 0, &raw const reply as *const c_void,
	      std::mem::size_of_val (&reply) as ssize_t);
  request_sigio ();
  unblock_input ();

  return Qnil ();
});

// C 3098-3147.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-display-monitor-attributes-list",
       Fhaiku_display_monitor_attributes_list,
       Shaiku_display_monitor_attributes_list,
       0, 1, 0,
       doc: "Return a list of physical monitor attributes on the display TERMINAL.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nInternal use only, use `display-monitor-attributes-list' instead.  ",
       (terminal: Lisp_Object),
       {
  let mut monitor: MonitorInfo = std::mem::zeroed ();
  let dpyinfo: *mut haiku_display_info;
  let mut frames: Lisp_Object;
  let mut tail: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();

  dpyinfo = check_haiku_display_info (terminal);
  frames = Qnil ();

  FOR_EACH_FRAME! (tail, tem, {
    maybe_quit ();

    if FRAME_HAIKU_P (XFRAME (tem))
	&& !FRAME_TOOLTIP_P (XFRAME (tem))
      { frames = Fcons (tem, frames); }
  });

  monitor.geom.x = 0;
  monitor.geom.y = 0;
  be_get_screen_dimensions (&raw mut monitor.geom.width as *mut c_int,
			    &raw mut monitor.geom.height as *mut c_int);

  monitor.mm_width = (monitor.geom.width as c_double
		      / ((*dpyinfo).resx / 25.4)) as c_int;
  monitor.mm_height = (monitor.geom.height as c_double
		       / ((*dpyinfo).resy / 25.4)) as c_int;
  monitor.name = c"BeOS monitor".as_ptr () as *mut c_char;

  if !be_get_explicit_workarea (&raw mut monitor.work.x, &raw mut monitor.work.y,
				&raw mut monitor.work.width as *mut c_int,
				&raw mut monitor.work.height as *mut c_int)
    { monitor.work = monitor.geom; }

  return make_monitor_attribute_list (&raw mut monitor, 1, 0,
				      make_vector (1, frames),
				      c"fallback".as_ptr ());
});

/* C 3149-3203: the frame parameter handler table.  Its 51 entries are the `frame_parm_handler`
   values this file defines or the generic `frame.c` setters; C's `NULL` entries are `None`.
   `include/haikuterm.rs`'s `haikuterm_objs` declares this object (haikuterm.h 235) and this
   definition shadows the declaration (PORTING.md 0.1-10/P-21).  */
// C 3149-3203.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_frame_parm_handlers: [frame_parm_handler; 51] =
  [
    Some (gui_set_autoraise),
    Some (gui_set_autolower),
    Some (haiku_set_background_color),
    None,					/* x_set_border_color */
    Some (gui_set_border_width),
    Some (haiku_set_cursor_color),
    Some (haiku_set_cursor_type),
    Some (gui_set_font),
    Some (haiku_set_foreground_color),
    None,					/* set icon name */
    None,					/* set icon type */
    Some (haiku_set_child_frame_border_width),
    Some (haiku_set_internal_border_width),
    Some (gui_set_right_divider_width),
    Some (gui_set_bottom_divider_width),
    Some (haiku_set_menu_bar_lines),
    Some (haiku_set_mouse_color),
    Some (haiku_explicitly_set_name),
    Some (gui_set_scroll_bar_width),
    Some (gui_set_scroll_bar_height),
    Some (haiku_set_title),
    Some (gui_set_unsplittable),
    Some (gui_set_vertical_scroll_bars),
    Some (gui_set_horizontal_scroll_bars),
    Some (gui_set_visibility),
    Some (haiku_set_tab_bar_lines),
    Some (haiku_set_tool_bar_lines),
    None,					/* set scroll bar fg */
    None,					/* set scroll bar bkg */
    Some (gui_set_screen_gamma),
    Some (gui_set_line_spacing),
    Some (gui_set_left_fringe),
    Some (gui_set_right_fringe),
    None,					/* x wait for wm */
    Some (gui_set_fullscreen),
    Some (gui_set_font_backend),
    Some (gui_set_alpha),
    Some (haiku_set_sticky),
    Some (haiku_set_tool_bar_position),
    Some (haiku_set_inhibit_double_buffering),
    Some (haiku_set_undecorated),
    Some (haiku_set_parent_frame),
    None,					/* set skip taskbar */
    Some (haiku_set_no_focus_on_map),
    Some (haiku_set_no_accept_focus),
    Some (haiku_set_z_group),
    Some (haiku_set_override_redirect),
    Some (gui_set_no_special_glyphs),
    Some (gui_set_alpha_background),
    Some (gui_set_borders_respect_alpha_background),
    Some (haiku_set_use_frame_synchronization),
  ];

/* C 3291-3321's five `DEFVAR_LISP`/`DEFVAR_BOOL` variables: C's `DEFVAR_*` defines the object
   (`globals.f_##vname`), so the port defines it here and `globals.rs`'s declaration is shadowed
   (the `src/alloc.rs` 8026-8032 shape).  */
// C 3291: `DEFVAR_LISP ("x-max-tooltip-size", Vx_max_tooltip_size, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vx_max_tooltip_size: Lisp_Object = lisp_h_Qnil ();
// C 3295: `DEFVAR_LISP ("x-cursor-fore-pixel", Vx_cursor_fore_pixel, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vx_cursor_fore_pixel: Lisp_Object = lisp_h_Qnil ();
// C 3299: `DEFVAR_LISP ("x-pointer-shape", Vx_pointer_shape, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vx_pointer_shape: Lisp_Object = lisp_h_Qnil ();
// C 3303: `DEFVAR_LISP ("x-hourglass-pointer-shape", Vx_hourglass_pointer_shape, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vx_hourglass_pointer_shape: Lisp_Object = lisp_h_Qnil ();
// C 3307: `DEFVAR_LISP ("x-sensitive-text-pointer-shape", Vx_sensitive_text_pointer_shape, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vx_sensitive_text_pointer_shape: Lisp_Object = lisp_h_Qnil ();
// C 3314: `DEFVAR_BOOL ("haiku-pass-control-tab-to-system", haiku_pass_control_tab_to_system, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_pass_control_tab_to_system: bool = false;
// C 3319: `DEFVAR_LISP ("haiku-allowed-ui-colors", Vhaiku_allowed_ui_colors, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vhaiku_allowed_ui_colors: Lisp_Object = lisp_h_Qnil ();

// C 3312: `DEFVAR_LISP ("cairo-version-string", Vcairo_version_string, ...)` - C 3311's
// `#ifdef USE_BE_CAIRO`, so its storage carries the `be_cairo` gate of the `DEFVAR` block itself in
// addition to the file gate.  C's `DEFVAR_*` defines the object (`globals.f_##vname`), so the port
// defines it here and `globals.rs`'s declaration is shadowed (docs/status.md 3.29).
#[cfg (feature = "have_haiku")]
#[cfg (feature = "be_cairo")]
#[unsafe(no_mangle)]
pub static mut Vcairo_version_string: Lisp_Object = lisp_h_Qnil ();

// C 3205-3324.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_haikufns ()
{
  DEFSYM! (Qfont_parameter, "font-parameter");
  DEFSYM! (Qcancel_timer, "cancel-timer");
  DEFSYM! (Qassq_delete_all, "assq-delete-all");

  DEFSYM! (Qrun_at_time, "run-at-time");
  DEFSYM! (Qx_hide_tip, "x-hide-tip");

  DEFSYM! (Qalways, "always");
  DEFSYM! (Qnot_useful, "not-useful");
  DEFSYM! (Qwhen_mapped, "when-mapped");
  DEFSYM! (Qtooltip_reuse_hidden_frame, "tooltip-reuse-hidden-frame");

  DEFSYM! (Qstatic_color, "static-color");
  DEFSYM! (Qstatic_gray, "static-gray");
  DEFSYM! (Qtrue_color, "true-color");
  DEFSYM! (Qmono, "mono");
  DEFSYM! (Qgrayscale, "grayscale");
  DEFSYM! (Qcolor, "color");

  defsubr (&raw mut Sx_hide_tip);
  defsubr (&raw mut Sxw_display_color_p);
  defsubr (&raw mut Sx_display_grayscale_p);
  defsubr (&raw mut Sx_open_connection);
  defsubr (&raw mut Sx_create_frame);
  defsubr (&raw mut Sx_display_pixel_width);
  defsubr (&raw mut Sx_display_pixel_height);
  defsubr (&raw mut Sxw_color_values);
  defsubr (&raw mut Sxw_color_defined_p);
  defsubr (&raw mut Sx_display_visual_class);
  defsubr (&raw mut Sx_show_tip);
  defsubr (&raw mut Sx_display_mm_height);
  defsubr (&raw mut Sx_display_mm_width);
  defsubr (&raw mut Sx_close_connection);
  defsubr (&raw mut Sx_display_list);
  defsubr (&raw mut Sx_server_vendor);
  defsubr (&raw mut Sx_server_version);
  defsubr (&raw mut Sx_display_screens);
  defsubr (&raw mut Shaiku_get_version_string);
  defsubr (&raw mut Sx_display_color_cells);
  defsubr (&raw mut Sx_display_planes);
  defsubr (&raw mut Shaiku_set_mouse_absolute_pixel_position);
  defsubr (&raw mut Shaiku_mouse_absolute_pixel_position);
  defsubr (&raw mut Shaiku_frame_geometry);
  defsubr (&raw mut Shaiku_frame_edges);
  defsubr (&raw mut Sx_double_buffered_p);
  defsubr (&raw mut Sx_display_backing_store);
  defsubr (&raw mut Shaiku_read_file_name);
  defsubr (&raw mut Shaiku_put_resource);
  defsubr (&raw mut Shaiku_frame_list_z_order);
  defsubr (&raw mut Sx_display_save_under);
  defsubr (&raw mut Shaiku_frame_restack);
  defsubr (&raw mut Shaiku_save_session_reply);
  defsubr (&raw mut Shaiku_display_monitor_attributes_list);

  tip_timer = Qnil ();
  staticpro (&raw mut tip_timer);
  tip_frame = Qnil ();
  staticpro (&raw mut tip_frame);
  tip_last_frame = Qnil ();
  staticpro (&raw mut tip_last_frame);
  tip_last_string = Qnil ();
  staticpro (&raw mut tip_last_string);
  tip_last_parms = Qnil ();
  staticpro (&raw mut tip_last_parms);
  tip_dx = Qnil ();
  staticpro (&raw mut tip_dx);
  tip_dy = Qnil ();
  staticpro (&raw mut tip_dy);

  DEFVAR_LISP! (c"x-max-tooltip-size", Vx_max_tooltip_size,
	       "SKIP: real doc in xfns.c.  ");
  Vx_max_tooltip_size = Qnil ();

  DEFVAR_LISP! (c"x-cursor-fore-pixel", Vx_cursor_fore_pixel,
	       "SKIP: real doc in xfns.c.  ");
  Vx_cursor_fore_pixel = Qnil ();

  DEFVAR_LISP! (c"x-pointer-shape", Vx_pointer_shape,
	       "SKIP: real doc in xfns.c.  ");
  Vx_pointer_shape = Qnil ();

  DEFVAR_LISP! (c"x-hourglass-pointer-shape", Vx_hourglass_pointer_shape,
	       "SKIP: real doc in xfns.c.  ");
  Vx_hourglass_pointer_shape = Qnil ();

  DEFVAR_LISP! (c"x-sensitive-text-pointer-shape",
	       Vx_sensitive_text_pointer_shape,
	       "SKIP: real doc in xfns.c.  ");
  Vx_sensitive_text_pointer_shape = Qnil ();

  DEFVAR_BOOL! (c"haiku-pass-control-tab-to-system",
	       haiku_pass_control_tab_to_system,
	       "Whether or not to pass C-TAB to the system.\nSetting this variable will cause Emacs to pass C-TAB to the system\n(allowing window switching on the Haiku operating system), rather than\nintercepting it.  ");
  haiku_pass_control_tab_to_system = true;

  DEFVAR_LISP! (c"haiku-allowed-ui-colors", Vhaiku_allowed_ui_colors,
	       "Vector of UI colors that Emacs can look up from the system.\nIf this is set up incorrectly, Emacs can crash when encountering an\ninvalid color.  ");
  Vhaiku_allowed_ui_colors = Qnil ();

  /* C 3311-3321: `#ifdef USE_BE_CAIRO` around the `cairo-version-string` DEFVAR.  */
  #[cfg (feature = "be_cairo")]
  {
    DEFVAR_LISP! (c"cairo-version-string", Vcairo_version_string,
                 "Version info for cairo.  ");
    {
      /* C's `sizeof ".." + 3 * INT_STRLEN_BOUND (int)` is 36 for this target's `int`.  */
      let mut cairo_version: [c_char; 36] = [0; 36];
      let len: c_int = sprintf (cairo_version.as_mut_ptr (), c"%d.%d.%d".as_ptr (),
			       CAIRO_VERSION_MAJOR, CAIRO_VERSION_MINOR,
                       CAIRO_VERSION_MICRO);
      Vcairo_version_string = make_specified_string (cairo_version.as_ptr (), len as ptrdiff_t,
						     len as ptrdiff_t, false);
    }
  }

  return;
}
