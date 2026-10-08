// femacs: port of src/androidfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except `do_unwind_create_frame`, whose address C takes
// for `record_unwind_protect` (it is `unsafe extern "C" fn`, the `callproc.rs`
// `call_process_kill` shape).
//
// **This file is P-86's third Android unit, and this is slice 1: C 1-1214.**  `configure.ac` 2597
// puts `androidfns.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (both Android branches,
// like `androidmenu.o` and `androidterm.o`), so `docs/baseline/frozen-build-sources.txt` lists no
// `androidfns.o` for this target, `extract-abi.py`'s checklist for the file is empty, and P-86 ②'s
// substitute is what `docs/ports/androidfns.md` carries: the C line-range table, the `## Evidence`
// readings and the `## Deviations`.  The gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID`) is `feature = "have_android"`, on every item *and on C 20-31's
//     include graph* - which is why the frozen configuration compiles nothing here (P-86 ⑧);
//   * C 33's `#ifndef ANDROID_STUBIFY` (with C 58's `#endif`) and C 124's `#ifndef ANDROID_STUBIFY`
//     (with C 729's `#endif /* ANDROID_STUBIFY */`) are `not_android_stubify`, the feature the tree
//     registered for C's `HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline`'s
//     `feature-coverage.tsv`, first used by `alloc.rs`).  Neither `#ifndef` has an `#else`: the
//     stub side of both is *nothing*, which is what `ANDROID_STUBIFY` means at C 2606;
//   * C 738's `#ifdef ANDROID_STUBIFY` / C 741's `#else` inside `Fx_create_frame` is that same
//     feature the other way round: the stub arm is `not (feature = "not_android_stubify")`.
//
// **Why 1214.**  P-86 ④ says to start at C 1 and stop at a whole-item boundary.  C 733-1214 is the
// file's first `DEFUN`, `Fx_create_frame` (its closing brace is C 1214, the whole-item boundary
// nearest the 1,150-line target; C 727's `android_create_frame_window` is the last boundary
// *before* 1,000).  **The next live definition is C 1216's `DEFUN ("xw-color-defined-p", …)`**,
// whose stub/real arm pair begins at C 1220.  Landing this `DEFUN` is also what stops
// `extract-abi.py --check` being vacuous for the file (P-86 ⑩).
//
// The mirror gaps this slice's text reaches are named in `docs/ports/androidfns.md` and declared at
// the use site below (P-72 rule 4 / P-88 ①): the mirrors carry most of frame.h, but the `bool_bf`
// accessors, the `l-value` frame macros and roughly twenty functions of frame.h/dispextern.h/
// lisp.h/font.h are defined in the *units* that own them, so a unit that reads or writes them
// declares its own copy (the `androidterm.rs` / `pgtkfns.rs` shape).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-22 and the two headers `lisp.h` itself pulls in (`<string.h>` 30 for `strcmp`,
   `<limits.h>` 33 for `INT_MAX`/`INT_MIN`).  The whole graph sits on the file gate, like C 24-31's
   (P-86 ⑧: an include outside the gate fails the frozen build at once).  */
#[cfg (feature = "have_android")]
include!("../lib/include/math.rs");                   // C 21: `#include <math.h>` (pow)
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");                 // C 22: `#include <stdlib.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");                 // lisp.h 30's `#include <string.h>` (strcmp)
#[cfg (feature = "have_android")]
include!("../lib/include/limits.in.rs");              // lisp.h 33's `#include <limits.h>` (INT_MAX)

/* C 24-31: the file's own include list, in C's order.  */
#[cfg (feature = "have_android")]
include!("include/lisp.rs");                          // C 24: `#include "lisp.h"`
#[cfg (feature = "have_android")]
include!("include/android.rs");                       // C 25: `#include "android.h"`
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");                   // C 26: `#include "androidterm.h"`
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");                    // C 27: `#include "blockinput.h"`
#[cfg (feature = "have_android")]
include!("include/keyboard.rs");                      // C 28: `#include "keyboard.h"`
#[cfg (feature = "have_android")]
include!("include/buffer.rs");                        // C 29: `#include "buffer.h"`
#[cfg (feature = "have_android")]
include!("include/androidgui.rs");                    // C 30: `#include "androidgui.h"`
#[cfg (feature = "have_android")]
include!("include/pdumper.rs");                       // C 31: `#include "pdumper.h"`

/* `android.h` 340-342's `ANDROID_DELETE_LOCAL_REF` (C 221's one use in this slice) expands to a
   `<jni.h>` vtable read.  `docs/ports/excluded-macros.txt` owns a row for the name whose premise
   ("`JNINativeInterface` has no mirror") `include/jni.rs` has since superseded, so - P-86 ⑪ - the
   unit that consumes it defines it under C's own name (the `androidmenu.rs` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* --------------------------------------------------------------------------------------------
   Use-site declarations (P-72 rule 4).  The frozen mirrors do not carry these names yet: each of
   them is a `frame.h` / `dispextern.h` / `lisp.h` / `font.h` declaration whose record or definition
   lives in another unit, so a unit that calls it declares it here.  Each line names the C header
   and line it comes from; `docs/ports/androidfns.md` names each as a mirror gap for the integrator.
   `struct font_driver` has no record anywhere in the tree yet (`include/font.rs` is a WIP slice),
   so `register_font_driver` and the two driver objects keep the untyped-pointer stand-in
   `include/androidterm.rs`'s `androidterm_objs` already uses (P-72 rule ③).  */
#[cfg (feature = "have_android")]
unsafe extern "C" {
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1475: `extern WINDOW_SYSTEM_RETURN struct frame *` / `decode_window_system_frame (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: frame.h 1479: `extern struct frame *make_frame (bool);`
  pub fn make_frame (mini_p: bool) -> *mut frame;
  /// C: frame.h 1489: `struct frame *make_minibuffer_frame (void);`
  pub fn make_minibuffer_frame () -> *mut frame;
  /// C: frame.h 1491: `make_frame_without_minibuffer (Lisp_Object mini_window,` (C 1492 continues
  /// `struct kboard *kb, Lisp_Object display);`)
  pub fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut KBOARD,
                                        display: Lisp_Object) -> *mut frame;
  /// C: frame.h 1509: `extern Lisp_Object delete_frame (Lisp_Object, Lisp_Object);`
  pub fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  pub fn decode_live_terminal (t: Lisp_Object) -> *mut terminal;
  /// C: frame.h 1430: `extern EMACS_UINT frame_set_id_from_params (struct frame *f, Lisp_Object params);`
  pub fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object) -> EMACS_UINT;
  /// C: frame.h 774: `fset_parent_frame (struct frame *f, Lisp_Object val)` (the C `INLINE` setter).
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
  /// C: frame.h 1865: `extern long gui_figure_window_size (struct frame *, Lisp_Object, bool, bool);`
  pub fn gui_figure_window_size (f: *mut frame, parms: Lisp_Object, x_frame: bool,
                                 toolbar_p: bool) -> c_long;
  /// C: dispextern.h 3994: `extern Lisp_Object gui_display_get_arg (Display_Info *, Lisp_Object,`
  pub fn gui_display_get_arg (dpyinfo: *mut Display_Info, alist: Lisp_Object, param: Lisp_Object,
                              name: *const c_char, x_name: *const c_char,
                              r#type: resource_types) -> Lisp_Object;
  /// C: dispextern.h 3997: `extern Lisp_Object gui_frame_get_and_record_arg (struct frame *, Lisp_Object,`
  pub fn gui_frame_get_and_record_arg (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
                                       name: *const c_char, x_name: *const c_char,
                                       r#type: resource_types) -> Lisp_Object;
  /// C: dispextern.h 4001: `extern Lisp_Object gui_default_parameter (struct frame *, Lisp_Object,`
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
                                deflt: Lisp_Object, name: *const c_char, x_name: *const c_char,
                                r#type: resource_types) -> Lisp_Object;
  /// C: dispextern.h 3821: `void init_frame_faces (struct frame *);`
  pub fn init_frame_faces (f: *mut frame);
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
  pub fn adjust_frame_glyphs (f: *mut frame);
  /// C: dispextern.h 3902: `void free_glyphs (struct frame *);`
  pub fn free_glyphs (f: *mut frame);
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);`
  /// - the mirror `include/dispextern.rs` 832 carries the record, so the parameter keeps its
  /// type (the `dispnew.rs` 1109 spelling).
  pub fn clear_glyph_matrix (matrix: *mut glyph_matrix);
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);`
  pub fn clear_current_matrices (f: *mut frame);
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  pub fn clear_frame (f: *mut frame);
  /// C: frame.h 1511: `extern void adjust_frame_size (struct frame *, int, int, int, bool, Lisp_Object);`
  /// (the port's `pgtkfns.rs` spells the sixth parameter `parameter`).
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int, inhibit: c_int,
                            pretend: bool, parameter: Lisp_Object);
  /// C: font.h 883: `extern Lisp_Object font_open_by_name (struct frame *f, Lisp_Object name);`
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *, struct frame *);`
  /// - `struct font_driver`'s record is font.h's and has not landed, so the parameter keeps the
  /// untyped pointer (P-72 rule ③).
  pub fn register_font_driver (driver: *const c_void, f: *mut frame);
}

/* `enum resource_types` (dispextern.h 3983-3992) is a WIP-slice gap of `include/dispextern.rs`;
   `frame.rs` 6082 and `pgtkfns.rs` 287 each carry a use-site copy today (P-72 rule 4).  C's enum
   ABI is `c_int` (PORTING.md 1.3), and the slice names four of the six members (C 771, 774, 784,
   795, 804, 822, 840, 852, 878, 1070, 1076, 1080, 1084, 1088, 1092, 1095, 1097).  */
/// C: dispextern.h 3983: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT, RES_TYPE_BOOLEAN, RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };`
#[cfg (feature = "have_android")]
pub type resource_types = c_int;
#[cfg (feature = "have_android")]
pub const RES_TYPE_NUMBER: resource_types = 0;          // C 3985
#[cfg (feature = "have_android")]
pub const RES_TYPE_FLOAT: resource_types = 1;           // C 3986
#[cfg (feature = "have_android")]
pub const RES_TYPE_BOOLEAN: resource_types = 2;         // C 3987
#[cfg (feature = "have_android")]
pub const RES_TYPE_STRING: resource_types = 3;          // C 3988
#[cfg (feature = "have_android")]
pub const RES_TYPE_SYMBOL: resource_types = 4;          // C 3989
/* The slice declares no record of its own, so it asserts no layout: every type it names is a
   mirror's, and those mirrors carry their own `offset_of!`/`sizeof` assertions.  The one
   measurement this slice's own text needs from the NDK is `jstring`'s size (C 219's
   `jstring java_name;`), measured with the NDK's clang for `--target=aarch64-linux-android24`
   and `x86_64-linux-android24`: `sizeof (jstring) == sizeof (jobject) == sizeof (jclass) ==
   sizeof (jmethodID) == 8`, `__ANDROID_API__ == 24`, `ANDROID_LOG_FATAL == 7` (all four
   `_Static_assert`s hold, exit 0; `docs/ports/androidfns.md` carries the command).  */

/* frame.h 300-341's `bool_bf` flag run: the readers/writers live in the unit that owns `struct
   frame` (`frame.rs`) and in each unit whose bodies touch the fields, so this unit defines its own
   copies for the eight flags its slice touches (PORTING.md 1.3; `docs/ports/frame.md`).  The
   shift constants are `include/frame.rs`'s and are gated to the frozen (non-window-system) arm -
   a window-system configuration has no numbering there yet, the `include/frame.rs` round the
   ledger owes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! set_frame_flag2 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
/// C: frame.h 411: `bool_bf no_split : 1;` (read and written at C 863)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! no_split {
  ($f:expr) => { (*$f).bits1 >> FRAME_NO_SPLIT_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPLIT_SHIFT, $v) };
}
/// C: frame.h 417: `bool_bf explicit_name : 1;` (read at C 249, written at C 931/936)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! explicit_name {
  ($f:expr) => { (*$f).bits1 >> FRAME_EXPLICIT_NAME_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_EXPLICIT_NAME_SHIFT, $v) };
}
/// C: frame.h 491: `bool_bf can_set_window_size : 1;` (written at C 1124)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! can_set_window_size {
  ($f:expr) => { (*$f).bits1 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_CAN_SET_WINDOW_SIZE_SHIFT, $v) };
}
/// C: frame.h 503: `bool_bf tab_bar_redisplayed : 1;` (read at C 471)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! tab_bar_redisplayed {
  ($f:expr) => { (*$f).bits1 >> FRAME_TAB_BAR_REDISPLAYED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_TAB_BAR_REDISPLAYED_SHIFT, $v) };
}
/// C: frame.h 504: `bool_bf tab_bar_resized : 1;` (read and written at C 471)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! tab_bar_resized {
  ($f:expr) => { (*$f).bits1 >> FRAME_TAB_BAR_RESIZED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_TAB_BAR_RESIZED_SHIFT, $v) };
}
/// C: frame.h 513: `bool_bf tool_bar_redisplayed : 1;` (read at C 373)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! tool_bar_redisplayed {
  ($f:expr) => { (*$f).bits2 >> FRAME_TOOL_BAR_REDISPLAYED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_TOOL_BAR_REDISPLAYED_SHIFT, $v) };
}
/// C: frame.h 514: `bool_bf tool_bar_resized : 1;` (read and written at C 373)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! tool_bar_resized {
  ($f:expr) => { (*$f).bits2 >> FRAME_TOOL_BAR_RESIZED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_TOOL_BAR_RESIZED_SHIFT, $v) };
}
/// C: frame.h 530: `bool_bf was_invisible : 1;` (read and written at C 1126/1131/1136)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! was_invisible {
  ($f:expr) => { (*$f).bits2 >> FRAME_WAS_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_WAS_INVISIBLE_SHIFT, $v) };
}

/* frame.h 1239/1245: `#define FRAME_UNDECORATED(f) ((f)->undecorated)` /
   `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)` - C writes both (C 844/856) and
   `include/frame.rs` mirrors them as readers only, so the assignment shape is here at the use site
   under C's own name (the `pgtkfns.rs` 543/555 shape).  `union output_data` / `struct frame`'s
   `bits1`/`bits2` are the mirror's.  */
/// C: frame.h 1245: `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_UNDECORATED {
  ($f:expr) => { FRAME_UNDECORATED ($f) };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_UNDECORATED_SHIFT, $v) };
}
/* frame.h 471's `bool_bf override_redirect : 1;` sits under `#ifdef HAVE_WINDOW_SYSTEM`, and
   `include/frame.rs` gates its two constants (`FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT`/`_UNIT`, lines
   625-627) the same way - so this unit's gate set (`have_android` + `not_android_stubify`, without
   `have_window_system`) has the mirror's *frozen* reader and no constants at all.  The one write C
   856 makes is spelled with the number all three window-system arms share (frame.h 471's measured
   `A 2:7; B 2:7; C 2:7`), under C's own name so the copy retires itself when the mirror's gate
   matches; `docs/ports/androidfns.md` names the mirror's gate as the gap.  */
/// C: frame.h 471: `bool_bf override_redirect : 1;`  - A 2:7; B 2:7; C 2:7 (measured)
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
           not (feature = "have_window_system")))]
const FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT: u32 = 7;

/// C: frame.h 1247: `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_OVERRIDE_REDIRECT {
  ($f:expr) => { FRAME_OVERRIDE_REDIRECT ($f) };
  ($f:expr, $v:expr) => {
    set_frame_flag2! ($f, FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT, $v) };
}

/* C's `f->output_method = …` (frame.h 446's `ENUM_BF`): the mirror carries the reader
   (`pub fn output_method`), so this unit adds C's assignment shape under C's own name (the
   `frame.rs` 373-378 shape; `frame_output_method` itself is private to that mirror).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! set_output_method {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
                  | ((($v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT)
  };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! output_method {
  ($f:expr) => { output_method ($f) };
  ($f:expr, $v:expr) => { set_output_method! ($f, $v) };
}

/* frame.h 1103-1108/1283-1301/1234-1239: the frame macros C's text uses as *l-values* (and twice
   as expressions).  `include/frame.rs` mirrors each as a reader function, so the writable form is
   here at the use site under C's own name, the `frame.rs` 6022-6056 / `pgtkfns.rs` 474-511 shape.  */
/// C: frame.h 1107: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr) => { (*$f).tool_bar_lines };
}
/// C: frame.h 1108: `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr) => { (*$f).tool_bar_height };
}
/// C: frame.h 1090: `#define FRAME_TAB_BAR_LINES(f) (f)->tab_bar_lines`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TAB_BAR_LINES {
  ($f:expr) => { (*$f).tab_bar_lines };
}
/// C: frame.h 1091: `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { (*$f).tab_bar_height };
}
/// C: frame.h 1234: `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_FOREGROUND_PIXEL {
  ($f:expr) => { (*$f).foreground_pixel };
}
/// C: frame.h 1239: `#define FRAME_BACKGROUND_PIXEL(f) ((f)->background_pixel)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_BACKGROUND_PIXEL {
  ($f:expr) => { (*$f).background_pixel };
}
/// C: frame.h 1286: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f) ((f)->config_scroll_bar_width)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr) => { (*$f).config_scroll_bar_width };
}
/// C: frame.h 1291: `#define FRAME_CONFIG_SCROLL_BAR_HEIGHT(f) ((f)->config_scroll_bar_height)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr) => { (*$f).config_scroll_bar_height };
}
/// C: frame.h 1296: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) ((f)->config_scroll_bar_cols)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { (*$f).config_scroll_bar_cols };
}
/// C: frame.h 1301: `#define FRAME_CONFIG_SCROLL_BAR_LINES(f) ((f)->config_scroll_bar_lines)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr) => { (*$f).config_scroll_bar_lines };
}

/* --------------------------------------------------------------------------------------------
   C 33-58: `#ifndef ANDROID_STUBIFY` - the tooltip state.  C writes no `#else`, so a
   configuration with `ANDROID_STUBIFY` compiles the include graph and no item of this block.
   All seven are C *tentative* definitions, i.e. zero (PORTING.md 0.1-13).  */

/* The frame of the currently visible tooltip, or nil if none.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_frame: Lisp_Object = lisp_h_Qnil ();             // C 36 (tentative)

/* The window-system window corresponding to the frame of the currently visible tooltip.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_window: android_window = 0;                      // C 40 (tentative)

/* The X and Y deltas of the last call to `x-show-tip'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_dx: Lisp_Object = lisp_h_Qnil ();                // C 43 (tentative)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_dy: Lisp_Object = lisp_h_Qnil ();                // C 43 (tentative)

/* A timer that hides or deletes the currently visible tooltip when it fires.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_timer: Lisp_Object = lisp_h_Qnil ();             // C 47 (tentative)

/* STRING argument of last `x-show-tip' call.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_last_string: Lisp_Object = lisp_h_Qnil ();       // C 50 (tentative)

/* Normalized FRAME argument of last `x-show-tip' call.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_last_frame: Lisp_Object = lisp_h_Qnil ();        // C 53 (tentative)

/* PARMS argument of last `x-show-tip' call.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut tip_last_parms: Lisp_Object = lisp_h_Qnil ();        // C 56 (tentative)

/* --------------------------------------------------------------------------------------------
   C 60-120: the three display-info accessors.  These sit *outside* both `#ifndef ANDROID_STUBIFY`
   blocks of the file, so they carry the file gate alone.  */

#[cfg (feature = "have_android")]
unsafe fn android_display_info_for_name (name: Lisp_Object) -> *mut android_display_info
{                                                              // C 60-76
  let mut dpyinfo: *mut android_display_info;

  CHECK_STRING (name);

  dpyinfo = x_display_list;
  while !dpyinfo.is_null ()
    {
      if !NILP (Fstring_equal (XCAR ((*dpyinfo).name_list_element),
			       name))
	{ return dpyinfo; }
      dpyinfo = (*dpyinfo).next;
    }

  /* C 73-74: C's two adjacent literals concatenate into one.  */
  error (c"Cannot connect to Android if it was not initialized at startup".as_ptr ());
}

/* `Display_Info` (dispextern.h 165's `typedef struct android_display_info Display_Info;`) is the
   mirror's alias for the Android arm, and that arm is gated `have_window_system` - which this
   unit's gate set does not carry (the `androidterm.rs` reading's `HAVE_WINDOW_SYSTEM` note), so the
   port supplies C's own alias here until the mirror's gate matches.  */
/// C: dispextern.h 165: `typedef struct android_display_info Display_Info;`
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
pub type Display_Info = android_display_info;

#[cfg (feature = "have_android")]
unsafe fn check_android_display_info (object: Lisp_Object) -> *mut android_display_info
{                                                              // C 78-114
  let mut dpyinfo: *mut android_display_info;
  let sf: *mut frame;
  let f: *mut frame;
  let t: *mut terminal;

  if NILP (object)
    {
      sf = XFRAME (selected_frame);

      if FRAME_ANDROID_P (sf) && FRAME_LIVE_P (sf)
	{ dpyinfo = FRAME_DISPLAY_INFO! (sf); }
      else if ! x_display_list.is_null ()
	{ dpyinfo = x_display_list; }
      else
	{ error (c"Android windows are not in use or not initialized".as_ptr ()); }
    }
  else if TERMINALP (object)
    {
      t = decode_live_terminal (object);

      if (*t).r#type != output_android
        { error (c"Terminal %d is not an Android display".as_ptr (), (*t).id); }

      dpyinfo = (*t).display_info.android as *mut android_display_info;
    }
  else if STRINGP (object)
    { dpyinfo = android_display_info_for_name (object); }
  else
    {
      f = decode_window_system_frame (object);
      dpyinfo = FRAME_DISPLAY_INFO! (f);
    }

  return dpyinfo;
}

#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_x_display_info (object: Lisp_Object) -> *mut Display_Info
{                                                              // C 116-120
  return check_android_display_info (object);
}

/* --------------------------------------------------------------------------------------------
   C 124-729: the file's first `#ifndef ANDROID_STUBIFY` arm - the real implementation of the
   frame hooks, `android_free_gcs`'s owner and the frame-creation helpers.  C writes no `#else`
   here (C 729's `#endif` ends the block), so the stub side is *nothing* and every item carries
   `all (have_android, not_android_stubify)`.  */

/* Gamma-correct COLOR on frame F.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gamma_correct (f: *mut frame, color: *mut Emacs_Color)
{                                                              // C 126-135
  if (*f).gamma != 0.0
    {
      (*color).red = (pow ((*color).red as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
      (*color).green = (pow ((*color).green as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
      (*color).blue = (pow ((*color).blue as c_double / 65535.0, (*f).gamma) * 65535.0 + 0.5)
        as c_ushort;
    }
}

/* Decide if color named COLOR_NAME is valid for use on frame F.  If so, return the RGB values in
   COLOR.  If ALLOC_P, allocate the color.  Value is false if COLOR_NAME is invalid, or no color
   could be allocated.  MAKE_INDEX is some mysterious argument used on NS.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_defined_color (f: *mut frame, color_name: *const c_char,
                                                color: *mut Emacs_Color, alloc_p: bool,
                                                make_index: bool) -> bool
{                                                              // C 137-159
  let mut success_p: bool;

  success_p = false;

  block_input ();
  success_p = android_parse_color (f, color_name, color) != 0;
  if success_p && alloc_p
    { success_p = android_alloc_nearest_color (f, color); }
  unblock_input ();

  return success_p;
}

/* Return the pixel color value for color COLOR_NAME on frame F.  If F is a monochrome frame,
   return MONO_COLOR regardless of what ARG says.  Signal an error if color can't be allocated.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_decode_color (f: *mut frame, color_name: Lisp_Object, mono_color: c_int)
                                -> c_ulong
{                                                              // C 161-177
  let mut cdef: Emacs_Color = std::mem::zeroed ();

  CHECK_STRING (color_name);

  if android_defined_color (f, SSDATA (color_name), &raw mut cdef,
			    true, false)
    { return cdef.pixel; }

  signal_error (c"Undefined color".as_ptr (), color_name);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_parent_frame (f: *mut frame, new_value: Lisp_Object,
				    old_value: Lisp_Object)
{                                                              // C 179-210
  let mut p: *mut frame;

  p = std::ptr::null_mut ();

  if !NILP (new_value)
      && (!FRAMEP (new_value)
	  || { p = XFRAME (new_value); !FRAME_LIVE_P (p) }
	  || !FRAME_ANDROID_P (p))
    {
      store_frame_param (f, Qparent_frame (), old_value);
      error (c"Invalid specification of `parent-frame'".as_ptr ());
    }

  if p != FRAME_PARENT_FRAME (f)
    {
      block_input ();
      android_reparent_window (FRAME_ANDROID_WINDOW! (f),
			       if !p.is_null () { FRAME_ANDROID_WINDOW! (p) }
			       else { (*FRAME_DISPLAY_INFO! (f)).root_window },
			       (*f).left_pos, (*f).top_pos);
      unblock_input ();

      fset_parent_frame (f, new_value);
    }

  /* Update the fullscreen frame parameter as well.  */
  ((*FRAME_TERMINAL! (f)).fullscreen_hook.unwrap ()) (f as *mut c_void);
}

/* Set the WM name to NAME for frame F.  Also set the icon name.  If the frame already has an
   icon name, use that, otherwise set the icon name to NAME.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_name_internal (f: *mut frame, name: Lisp_Object)
{                                                              // C 212-227
  let java_name: jstring;

  /* C 214: `if (FRAME_ANDROID_WINDOW (f))` - the mirror's `FRAME_ANDROID_WINDOW!` (P-72 rule ③'s
     `Emacs_Window` is `android_handle`, an `intptr_t`).  */
  if FRAME_ANDROID_WINDOW! (f) != 0
    {
      java_name = android_build_string (name, std::ptr::null_mut::<c_void> ());
      android_set_wm_name (FRAME_ANDROID_WINDOW! (f), java_name);
      ANDROID_DELETE_LOCAL_REF! (java_name);
    }
}

/* Change the name of frame F to NAME.  If NAME is nil, set F's name to x_id_name. ...  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_name (f: *mut frame, name: Lisp_Object, explicit: bool)
{                                                              // C 229-282
  let mut name = name;                        /* C 266/273 reassign the parameter.  */

  /* Make sure that requests from lisp code override requests from Emacs redisplay code.  */
  if explicit
    {
      /* If we're switching from explicit to implicit, we had better update the mode lines and
	 thereby update the title.  */
      if explicit_name! (f) && NILP (name)
	{ update_mode_lines = 37; }

      explicit_name! (f, ! NILP (name));
    }
  else if explicit_name! (f)
    { return; }

  /* If NAME is nil, set the name to the x_id_name.  */
  if NILP (name)
    {
      /* Check for no change needed in this very common case before we do any consing.  */
      if strcmp ((*FRAME_DISPLAY_INFO! (f)).x_id_name,
		 SSDATA ((*f).name)) == 0
	{ return; }
      name = build_string ((*FRAME_DISPLAY_INFO! (f)).x_id_name);
    }
  else
    { CHECK_STRING (name); }

  /* Don't change the name if it's already NAME.  */
  if ! NILP (Fstring_equal (name, (*f).name))
    { return; }

  fset_name (f, name);

  /* For setting the frame title, the title parameter should override the name parameter.  */
  if ! NILP ((*f).title)
    { name = (*f).title; }

  android_set_name_internal (f, name);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_implicitly_set_name (f: *mut frame, arg: Lisp_Object,
						      oldval: Lisp_Object)
{                                                              // C 284-289
  android_set_name (f, arg, false);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_explicitly_set_name (f: *mut frame, arg: Lisp_Object,
						      oldval: Lisp_Object)
{                                                              // C 291-296
  android_set_name (f, arg, true);
}

/* Set the number of lines used for the tool bar of frame F to VALUE.  VALUE not an integer, or
   < 0 means set the lines to zero.  OLDVAL is the old number of tool bar lines.  This function
   changes the height of all windows on frame F to match the new tool bar height.  The frame's
   height doesn't change.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{                                                              // C 298-321
  let nlines: c_int;

  /* Treat tool bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  android_change_tool_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_tool_bar_position (f: *mut frame, new_value: Lisp_Object,
					 old_value: Lisp_Object)
{                                                              // C 323-348
  if !EQ (new_value, Qtop ()) && !EQ (new_value, Qbottom ())
    { error (c"Tool bar position must be either `top' or `bottom'".as_ptr ()); }

  if EQ (new_value, old_value)
    { return; }

  /* Set the tool bar position.  */
  fset_tool_bar_position (f, new_value);

  /* Now reconfigure frame glyphs to place the tool bar at the bottom.  While the inner height
     has not changed, call `resize_frame_windows' to place each of the windows at its new
     position.  */
  adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_position ());
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);

  if FRAME_ANDROID_WINDOW! (f) != 0
    { android_clear_under_internal_border (f); }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_change_tool_bar_height (f: *mut frame, height: c_int)
{                                                              // C 350-394
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TOOL_BAR_HEIGHT (f);
  let lines: c_int = (height + unit - 1) / unit;
  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  FRAME_TOOL_BAR_HEIGHT! (f) = height;
  FRAME_TOOL_BAR_LINES! (f) = lines;
  store_frame_param (f, Qtool_bar_lines (), make_fixnum (lines as EMACS_INT));

  if FRAME_ANDROID_WINDOW! (f) != 0 && FRAME_TOOL_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tool_bar_window)).current_matrix
			  as *mut glyph_matrix); }

  if !tool_bar_resized! (f)
    {
      /* As long as tool_bar_resized is false, effectively try to change F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
	{ adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
			     1, false, Qtool_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtool_bar_lines ()); }

      tool_bar_resized! (f, tool_bar_redisplayed! (f));
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
}

/* Set the number of lines used for the tab bar of frame F to VALUE.  VALUE not an integer, or
   < 0 means set the lines to zero.  OLDVAL is the old number of tab bar lines.  This function
   may change the height of all windows on frame F to match the new tab bar height.  The frame's
   height may change if frame_inhibit_implied_resize was set accordingly.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{                                                              // C 396-424
  let olines: c_int;
  let nlines: c_int;

  olines = FRAME_TAB_BAR_LINES (f);

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  if nlines != olines && (olines == 0 || nlines == 0)
    { android_change_tab_bar_height (f, nlines * FRAME_LINE_HEIGHT (f)); }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_change_tab_bar_height (f: *mut frame, height: c_int)
{                                                              // C 426-488
  let unit: c_int;
  let old_height: c_int;
  let mut lines: c_int;
  let fullscreen: Lisp_Object;

  unit = FRAME_LINE_HEIGHT (f);
  old_height = FRAME_TAB_BAR_HEIGHT (f);
  fullscreen = get_frame_param (f, Qfullscreen ());

  /* This differs from the tool bar code in that the tab bar height is not rounded up.
     Otherwise, if redisplay_tab_bar decides to grow the tab bar by even 1 pixel,
     FRAME_TAB_BAR_LINES will be changed, leading to the tab bar height being incorrectly set
     upon the next call to android_set_font.  (bug#59285)  */

  lines = height / unit;

  /* Even so, HEIGHT might be less than unit if the tab bar face is not so tall as the frame's
     font height; which if true lines will be set to 0 and the tab bar will thus vanish.  */

  if lines == 0 && height != 0
    { lines = 1; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  /* Recalculate tab bar and frame text sizes.  */
  FRAME_TAB_BAR_HEIGHT! (f) = height;
  FRAME_TAB_BAR_LINES! (f) = lines;
  store_frame_param (f, Qtab_bar_lines (), make_fixnum (lines as EMACS_INT));

  if FRAME_ANDROID_WINDOW! (f) != 0 && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).current_matrix
			  as *mut glyph_matrix); }

  if !tab_bar_resized! (f)
    {
      /* As long as tab_bar_resized is false, effectively try to change F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
	{ adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
			     1, false, Qtab_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtab_bar_lines ()); }

      tab_bar_resized! (f, tab_bar_redisplayed! (f));
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtab_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_scroll_bar_default_height (f: *mut frame)
{                                                              // C 490-503
  let height: c_int;

  height = FRAME_LINE_HEIGHT (f);

  /* The height of a non-toolkit scrollbar is 14 pixels.  */
  FRAME_CONFIG_SCROLL_BAR_LINES! (f) = (14 + height - 1) / height;

  /* Use all of that space (aside from required margins) for the scroll bar.  */
  FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f) = 14;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_scroll_bar_default_width (f: *mut frame)
{                                                              // C 505-515
  let unit: c_int;

  unit = FRAME_COLUMN_WIDTH (f);

  FRAME_CONFIG_SCROLL_BAR_COLS! (f) = (14 + unit - 1) / unit;
  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f)
    = FRAME_CONFIG_SCROLL_BAR_COLS (f) * unit;
}

/* Verify that the icon position args for this window are valid.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_icon_verify (f: *mut frame, parms: Lisp_Object)
{                                                              // C 517-539
  let icon_x: Lisp_Object;
  let icon_y: Lisp_Object;

  /* Set the position of the icon.  Note that twm groups all icons in an icon window.  */
  icon_x = gui_frame_get_and_record_arg (f, parms, Qicon_left (), std::ptr::null (),
					 std::ptr::null (), RES_TYPE_NUMBER);
  icon_y = gui_frame_get_and_record_arg (f, parms, Qicon_top (), std::ptr::null (),
					 std::ptr::null (), RES_TYPE_NUMBER);

  if !BASE_EQ (icon_x, Qunbound ()) && !BASE_EQ (icon_y, Qunbound ())
    {
      CHECK_FIXNUM (icon_x);
      CHECK_FIXNUM (icon_y);
    }
  else if !BASE_EQ (icon_x, Qunbound ()) || !BASE_EQ (icon_y, Qunbound ())
    { error (c"Both left and top icon corners of icon must be specified".as_ptr ()); }
}

/* Handle the icon stuff for this window.  Perhaps later we might want an x_set_icon_position
   which can be called interactively as well.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_icon (f: *mut frame, parms: Lisp_Object)
{                                                              // C 541-570
  /* Set the position of the icon.  Note that twm groups all icons in an icon window.  */
  let icon_x: Lisp_Object
    = gui_frame_get_and_record_arg (f, parms, Qicon_left (), std::ptr::null (),
				    std::ptr::null (), RES_TYPE_NUMBER);
  let icon_y: Lisp_Object
    = gui_frame_get_and_record_arg (f, parms, Qicon_top (), std::ptr::null (),
				    std::ptr::null (), RES_TYPE_NUMBER);

  let xgiven: bool = !BASE_EQ (icon_x, Qunbound ());
  let ygiven: bool = !BASE_EQ (icon_y, Qunbound ());

  if xgiven != ygiven
    { error (c"Both left and top icon corners of icon must be specified".as_ptr ()); }

  if xgiven
    {
      check_integer_range (icon_x, INT_MIN as intmax_t, INT_MAX as intmax_t);
      check_integer_range (icon_y, INT_MIN as intmax_t, INT_MAX as intmax_t);
    }

  /* Now return as this is not supported on Android.  */
}

/* Make the GCs needed for this window, setting the background color.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_make_gc (f: *mut frame)
{                                                              // C 572-605
  let mut gc_values: android_gc_values = std::mem::zeroed ();

  block_input ();

  /* Create the GCs of this frame.  Note that many default values are used.  */

  gc_values.foreground = FRAME_FOREGROUND_PIXEL (f);
  gc_values.background = FRAME_BACKGROUND_PIXEL (f);
  (*((*f).output_data.android as *mut android_output)).normal_gc
    = android_create_gc (ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND,
			 &raw mut gc_values);

  /* Reverse video style.  */
  gc_values.foreground = FRAME_BACKGROUND_PIXEL (f);
  gc_values.background = FRAME_FOREGROUND_PIXEL (f);
  (*((*f).output_data.android as *mut android_output)).reverse_gc
    = android_create_gc (ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND,
			 &raw mut gc_values);

  /* Cursor has cursor-color background, background-color foreground.  */
  gc_values.foreground = FRAME_BACKGROUND_PIXEL (f);
  gc_values.background = (*((*f).output_data.android as *mut android_output)).cursor_pixel;
  (*((*f).output_data.android as *mut android_output)).cursor_gc
    = android_create_gc (ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND,
			 &raw mut gc_values);
  unblock_input ();
}

/* Free what was allocated in android_make_gc.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_free_gcs (f: *mut frame)
{                                                              // C 607-634
  block_input ();

  if (*((*f).output_data.android as *mut android_output)).normal_gc != std::ptr::null_mut ()
    {
      android_free_gc ((*((*f).output_data.android as *mut android_output)).normal_gc);
      (*((*f).output_data.android as *mut android_output)).normal_gc = std::ptr::null_mut ();
    }

  if (*((*f).output_data.android as *mut android_output)).reverse_gc != std::ptr::null_mut ()
    {
      android_free_gc ((*((*f).output_data.android as *mut android_output)).reverse_gc);
      (*((*f).output_data.android as *mut android_output)).reverse_gc = std::ptr::null_mut ();
    }

  if (*((*f).output_data.android as *mut android_output)).cursor_gc != std::ptr::null_mut ()
    {
      android_free_gc ((*((*f).output_data.android as *mut android_output)).cursor_gc);
      (*((*f).output_data.android as *mut android_output)).cursor_gc = std::ptr::null_mut ();
    }

  unblock_input ();
}

/* Handler for signals raised during x_create_frame and Fx_create_tip_frame.  FRAME is the frame
   which is partially constructed.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn unwind_create_frame (frame: Lisp_Object) -> Lisp_Object
{                                                              // C 636-660
  let f: *mut frame = XFRAME (frame);

  /* If frame is already dead, nothing to do.  This can happen if the display is disconnected
     after the frame has become official, but before Fx_create_frame removes the unwind
     protect.  */
  if !FRAME_LIVE_P (f)
    { return Qnil (); }

  /* If frame is ``official'', nothing to do.  */
  if NILP (Fmemq (frame, Vframe_list))
    {
      android_free_frame_resources (f);
      free_glyphs (f);
      return Qt ();
    }

  return Qnil ();
}

/* C 662-666: `do_unwind_create_frame` - the one helper of this slice whose address C takes
   (C 885's `record_unwind_protect`), so it is an `unsafe extern "C" fn` item and not a plain
   `unsafe fn` (PORTING.md 0.2; the `callproc.rs` `call_process_kill` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn do_unwind_create_frame (frame: Lisp_Object)
{
  unwind_create_frame (frame);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_default_font_parameter (f: *mut frame, parms: Lisp_Object)
{                                                              // C 668-707
  let dpyinfo: *mut android_display_info = FRAME_DISPLAY_INFO! (f);
  let mut font_param: Lisp_Object = gui_display_get_arg (dpyinfo, parms, Qfont (),
							 std::ptr::null (), std::ptr::null (),
							 RES_TYPE_STRING);
  let mut font: Lisp_Object = Qnil ();
  if BASE_EQ (font_param, Qunbound ())
    { font_param = Qnil (); }

  if NILP (font)
    { font = if !NILP (font_param)
	     { font_param }
	     else
	     { gui_display_get_arg (dpyinfo, parms,
				    Qfont (), c"font".as_ptr (), c"Font".as_ptr (),
				    RES_TYPE_STRING) }; }

  if ! FONTP (font) && ! STRINGP (font)
    {
      let names: [*const c_char; 4] = [
	c"Droid Sans Mono-12".as_ptr (),
	c"Monospace-12".as_ptr (),
	c"DroidSansMono-12".as_ptr (),
	std::ptr::null ()
      ];
      let mut i: c_int;

      i = 0;
      while ! names[i as usize].is_null ()
	{
	  font = font_open_by_name (f, build_unibyte_string (names[i as usize]));
	  if ! NILP (font)
	    { break; }
	  i += 1;
	}
    }

  if NILP (font)
    { error (c"No suitable font was found".as_ptr ()); }

  gui_default_parameter (f, parms, Qfont (), font, c"font".as_ptr (), c"Font".as_ptr (),
			 RES_TYPE_STRING);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_create_frame_window (f: *mut frame)
{                                                              // C 709-727
  let mut attributes: android_set_window_attributes = std::mem::zeroed ();
  let attribute_mask: android_window_value_mask;

  attributes.background_pixel = FRAME_BACKGROUND_PIXEL (f);
  attribute_mask = ANDROID_CW_BACK_PIXEL;

  block_input ();
  /* C 714-720: `FRAME_ANDROID_WINDOW (f) = android_create_window (…)` - C's macro is an
     l-value and the mirror carries the read arm only, so the write is C's own member path (the
     `androidterm.rs` 504 `output_data` shape).  */
  (*((*f).output_data.android as *mut android_output)).window
    = android_create_window ((*FRAME_DISPLAY_INFO! (f)).root_window,
			     (*f).left_pos,
			     (*f).top_pos,
			     FRAME_PIXEL_WIDTH (f),
			     FRAME_PIXEL_HEIGHT (f),
			     attribute_mask, &raw mut attributes);
  unblock_input ();
}

/* One more use-site declaration (P-72 rule 4): `android_create_gc`'s C declaration is in
   `androidgui.h`, but its mirror `include/androidgui.rs` does not carry it and the definition is
   `androidterm.c`'s (`androidterm.rs` 180 declares it inside that unit).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: androidgui.h 926: `extern struct android_gc *android_create_gc (enum android_gc_value_mask,`
  /// (C 927 continues `struct android_gc_values *);`)
  pub fn android_create_gc (valuemask: android_gc_value_mask,
                            values: *mut android_gc_values) -> *mut android_gc;
}

/* --------------------------------------------------------------------------------------------
   C 733-1214: the file's first `DEFUN`, `Fx_create_frame`.  The item carries the file gate; its
   body carries C 738's `#ifdef ANDROID_STUBIFY` / C 741's `#else` as two cfg'd blocks in C's
   order (the stub arm is `not (not_android_stubify)`), so exactly one of them compiles inside an
   Android configuration and `extract-abi.py --check` compares the whole subr (P-86 ⑩).  */

#[cfg (feature = "have_android")]
DEFUN! ("x-create-frame", Fx_create_frame, Sx_create_frame,
	1, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(parms: Lisp_Object),
	{
  /* C 738-740: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());
    return Qnil ();
  }

  /* C 741-1213: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut f: *mut frame;
  let mut frame: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut name: Lisp_Object;
  let mut minibuffer_only: bool;
  let mut undecorated: bool;
  let mut override_redirect: bool;
  let mut window_prompting: c_long;
  let count: specpdl_ref;
  let mut display: Lisp_Object;
  let mut dpyinfo: *mut android_display_info;
  let mut parent: Lisp_Object;
  let mut parent_frame: Lisp_Object;
  let mut kb: *mut KBOARD;
  let mut parms = parms;                       /* C 764 reassigns the parameter.  */

  minibuffer_only = false;
  undecorated = false;
  override_redirect = false;
  window_prompting = 0;
  count = SPECPDL_INDEX ();
  dpyinfo = std::ptr::null_mut ();

  /* Not actually used, but be consistent with X.  */
  let _ = window_prompting;

  parms = Fcopy_alist (parms);

  /* Use this general default value to start with until we know if this frame has a specified
     name.  */
  Vx_resource_name = Vinvocation_name;

  display = gui_display_get_arg (dpyinfo, parms, Qterminal (), std::ptr::null (),
				 std::ptr::null (), RES_TYPE_NUMBER);
  if BASE_EQ (display, Qunbound ())
    { display = gui_display_get_arg (dpyinfo, parms, Qdisplay (), std::ptr::null (),
				     std::ptr::null (), RES_TYPE_STRING); }
  if BASE_EQ (display, Qunbound ())
    { display = Qnil (); }
  dpyinfo = check_android_display_info (display);
  /* C 778: `dpyinfo->terminal->kboard` - the mirror's `terminal.kboard` is `*mut c_void`
     (`struct kboard`'s record is keyboard.h's, P-72 rule ③), so the port casts it.  */
  kb = (*((*dpyinfo).terminal)).kboard as *mut KBOARD;

  if (*((*dpyinfo).terminal)).name.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  name = gui_display_get_arg (dpyinfo, parms, Qname (), c"name".as_ptr (), c"Name".as_ptr (),
			      RES_TYPE_STRING);
  if !STRINGP (name)
      && ! BASE_EQ (name, Qunbound ())
      && ! NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  if STRINGP (name)
    { Vx_resource_name = name; }

  /* See if parent window is specified.  */
  parent = gui_display_get_arg (dpyinfo, parms, Qparent_id (), std::ptr::null (),
				std::ptr::null (), RES_TYPE_NUMBER);
  if BASE_EQ (parent, Qunbound ())
    { parent = Qnil (); }
  if ! NILP (parent)
    { CHECK_FIXNUM (parent); }

  frame = Qnil ();
  tem = gui_display_get_arg (dpyinfo, parms, Qminibuffer (), c"minibuffer".as_ptr (),
			     c"Minibuffer".as_ptr (), RES_TYPE_SYMBOL);
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

  parent_frame = gui_display_get_arg (dpyinfo, parms, Qparent_frame (), std::ptr::null (),
				      std::ptr::null (), RES_TYPE_SYMBOL);
  /* Accept parent-frame iff parent-id was not specified.  */
  if !NILP (parent)
      || BASE_EQ (parent_frame, Qunbound ())
      || NILP (parent_frame)
      || !FRAMEP (parent_frame)
      || !FRAME_LIVE_P (XFRAME (parent_frame))
      || !FRAME_ANDROID_P (XFRAME (parent_frame))
    { parent_frame = Qnil (); }

  fset_parent_frame (f, parent_frame);
  store_frame_param (f, Qparent_frame (), parent_frame);

  /* C 835-841: the assignment is C's `tem = (...)` inside `!NILP (…)`; Rust's assignment is a
     statement, so it stands on its own line (the `frame.rs` chained-assignment note).  */
  tem = gui_display_get_arg (dpyinfo, parms, Qundecorated (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_BOOLEAN);
  if !NILP (tem)
      && !BASE_EQ (tem, Qunbound ())
    { undecorated = true; }

  FRAME_UNDECORATED! (f, undecorated);
  store_frame_param (f, Qundecorated (), if undecorated { Qt () } else { Qnil () });

  /* C 847-853: the same assignment-inside-`NILP` spelling as C 835.  */
  tem = gui_display_get_arg (dpyinfo, parms, Qoverride_redirect (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_BOOLEAN);
  if !NILP (tem)
      && !BASE_EQ (tem, Qunbound ())
    { override_redirect = true; }

  FRAME_OVERRIDE_REDIRECT! (f, override_redirect);
  store_frame_param (f, Qoverride_redirect (), if override_redirect { Qt () } else { Qnil () });

  XSETFRAME! (frame, f);

  frame_set_id_from_params (f, parms);

  (*f).terminal = (*dpyinfo).terminal;

  output_method! (f, output_android);
  (*f).output_data.android = xzalloc (std::mem::size_of::<android_output> ());
  /* C 871: `FRAME_FONTSET (f) = -1;` - C's macro is an l-value and the mirror carries the read
     arm only (`E0428` forbids a second `FRAME_FONTSET!`), so the write is C's own member path
     (the `androidterm.rs` 504 shape).  */
  (*((*f).output_data.android as *mut android_output)).fontset = -1;        /* C 871: `c_int` */
  (*((*f).output_data.android as *mut android_output)).scroll_bar_foreground_pixel = (-1 as c_long) as c_ulong;
  (*((*f).output_data.android as *mut android_output)).scroll_bar_background_pixel = (-1 as c_long) as c_ulong;
  (*((*f).output_data.android as *mut android_output)).white_relief.pixel = (-1 as c_long) as c_ulong;
  (*((*f).output_data.android as *mut android_output)).black_relief.pixel = (-1 as c_long) as c_ulong;

  fset_icon_name (f, gui_display_get_arg (dpyinfo, parms, Qicon_name (),
					  c"iconName".as_ptr (), c"Title".as_ptr (),
					  RES_TYPE_STRING));
  if ! STRINGP ((*f).icon_name)
    { fset_icon_name (f, Qnil ()); }

  /* C 882: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as C 871.  */
  (*((*f).output_data.android as *mut android_output)).display_info = dpyinfo;

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (Some (do_unwind_create_frame), frame);

  /* These colors will be set anyway later, but it's important to get the color reference counts
     right, so initialize them!

     (Not really on Android, but it's best to be consistent with X.)  */
  {
    let black: Lisp_Object;

    /* Function x_decode_color can signal an error.  Make sure to initialize color slots so that
       we won't try to free colors we haven't allocated.  */
    FRAME_FOREGROUND_PIXEL! (f) = (-1 as c_long) as c_ulong;
    FRAME_BACKGROUND_PIXEL! (f) = (-1 as c_long) as c_ulong;
    (*((*f).output_data.android as *mut android_output)).cursor_pixel = (-1 as c_long) as c_ulong;
    (*((*f).output_data.android as *mut android_output)).cursor_foreground_pixel = (-1 as c_long) as c_ulong;
    (*((*f).output_data.android as *mut android_output)).mouse_pixel = (-1 as c_long) as c_ulong;

    black = build_string (c"black".as_ptr ());
    FRAME_FOREGROUND_PIXEL! (f)
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));
    FRAME_BACKGROUND_PIXEL! (f)
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));
    (*((*f).output_data.android as *mut android_output)).cursor_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));
    (*((*f).output_data.android as *mut android_output)).cursor_foreground_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));
    (*((*f).output_data.android as *mut android_output)).mouse_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));
  }

  /* Set the name; the functions to which we pass f expect the name to be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)
    {
      /* C 921's string is an identity string ("what *this* program calls itself"), so
         PORTING.md 0.1-19's name half (status 3.433) makes it the port's own name, the
         `src/emacs.rs` 359 `PACKAGE_NAME` precedent; the C text stays in the ledger.  */
      fset_name (f, build_string (c"femacs".as_ptr ()));
      explicit_name! (f, false);
    }
  else
    {
      fset_name (f, name);
      explicit_name! (f, true);
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);
    }

  /* C 920-921: `&androidfont_driver` / `&android_sfntfont_driver` - the two objects are declared
     `*mut c_void` stand-ins by `include/androidterm.rs` 530-531 (`struct font_driver` has no
     record yet), so the address keeps the untyped form (P-72 rule ③).  */
  register_font_driver (&raw mut androidfont_driver as *const c_void, f);
  register_font_driver (&raw mut android_sfntfont_driver as *const c_void, f);

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values that are needed to determine window
     geometry.  */
  android_default_font_parameter (f, parms);
  if FRAME_FONT! (f).is_null ()
    {
      delete_frame (frame, Qnoelisp ());
      error (c"Invalid frame font".as_ptr ());
    }

  if NILP (Fassq (Qinternal_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())
	{ parms = Fcons (Fcons (Qinternal_border_width (), value),
			 parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (),
			 make_fixnum (0),
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* Same for child frames.  */
  if NILP (Fassq (Qchild_frame_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qchild_frame_border_width (),
				   c"childFrameBorder".as_ptr (), c"childFrameBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())
	{ parms = Fcons (Fcons (Qchild_frame_border_width (), value),
			 parms); }
    }

  gui_default_parameter (f, parms, Qchild_frame_border_width (), Qnil (),
			 c"childFrameBorderWidth".as_ptr (), c"childFrameBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);

  /* `vertical-scroll-bars' defaults to nil on Android as a consequence of scroll bars not being
     supported at all.  */

  gui_default_parameter (f, parms, Qvertical_scroll_bars (), Qnil (),
                         c"verticalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
                         RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qhorizontal_scroll_bars (), Qnil (),
                         c"horizontalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
                         RES_TYPE_SYMBOL);

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),
                         c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),
                         c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),
                         c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),
                         c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qscreen_gamma (), Qnil (),
                         c"screenGamma".as_ptr (), c"ScreenGamma".as_ptr (),
                         RES_TYPE_FLOAT);
  gui_default_parameter (f, parms, Qline_spacing (), Qnil (),
                         c"lineSpacing".as_ptr (), c"LineSpacing".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qleft_fringe (), Qnil (),
                         c"leftFringe".as_ptr (), c"LeftFringe".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_fringe (), Qnil (),
                         c"rightFringe".as_ptr (), c"RightFringe".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* C 1015-1022: `#if 0` - C's own dead text (the two
     `android_default_scroll_bar_color_parameter` calls).  No configuration compiles it, so it is
     recorded in `docs/ports/androidfns.md` and not transcribed (the `xfns.rs` 958 `#if false`
     precedent).  */

  /* Init faces before gui_default_parameter is called for the scroll-bar-width parameter because
     otherwise we end up in init_iterator with a null face cache, which should not happen.  */

  init_frame_faces (f);

  tem = gui_display_get_arg (dpyinfo, parms, Qmin_width (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_width (), tem); }
  tem = gui_display_get_arg (dpyinfo, parms, Qmin_height (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_height (), tem); }

  adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
		     FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 5, true,
		     Qx_create_frame_1 ());

  /* Set the menu-bar-lines and tool-bar-lines parameters.  We don't look up the X resources
     controlling the menu-bar and tool-bar here; they are processed specially at startup, and
     reflected in the values of the mode variables.  */

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
  gui_default_parameter (f, parms, Qtitle (), Qnil (),
                         c"title".as_ptr (), c"Title".as_ptr (),
                         RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qwait_for_wm (), Qt (),
                         c"waitForWM".as_ptr (), c"WaitForWM".as_ptr (),
                         RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qtool_bar_position (),
                         FRAME_TOOL_BAR_POSITION (f), std::ptr::null (), std::ptr::null (),
                         RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
                         c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
                         RES_TYPE_BOOLEAN);

  /* Compute the size of the X window.  */
  window_prompting = gui_figure_window_size (f, parms, true, true);

  tem = gui_display_get_arg (dpyinfo, parms, Qunsplittable (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_BOOLEAN);
  no_split! (f, minibuffer_only || EQ (tem, Qt ()));

  android_icon_verify (f, parms);
  android_create_frame_window (f);
  android_icon (f, parms);
  android_make_gc (f);

  /* Now consider the frame official.  */
  (*((*f).terminal)).reference_count += 1;
  Vframe_list = Fcons (frame, Vframe_list);

  /* We need to do this after creating the window, so that the icon-creation functions can say
     whose icon they're describing.  */
  gui_default_parameter (f, parms, Qicon_type (), Qt (),
                         c"bitmapIcon".as_ptr (), c"BitmapIcon".as_ptr (),
                         RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
                         c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (),
                         RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
                         c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (),
                         RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
                         c"cursorType".as_ptr (), c"CursorType".as_ptr (),
                         RES_TYPE_SYMBOL);
  /* Scroll bars are not supported on Android, as they are near useless.  */
  gui_default_parameter (f, parms, Qscroll_bar_width (), Qnil (),
                         c"scrollBarWidth".as_ptr (), c"ScrollBarWidth".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qscroll_bar_height (), Qnil (),
                         c"scrollBarHeight".as_ptr (), c"ScrollBarHeight".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
                         c"alpha".as_ptr (), c"Alpha".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
                         c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
                         c"bordersRespectAlphaBackground".as_ptr (),
                         c"BordersRespectAlphaBackground".as_ptr (),
                         RES_TYPE_NUMBER);

  if !NILP (parent_frame)
    {
      let p: *mut frame = XFRAME (parent_frame);

      block_input ();
      android_reparent_window (FRAME_ANDROID_WINDOW! (f),
			       FRAME_ANDROID_WINDOW! (p),
			       (*f).left_pos, (*f).top_pos);
      unblock_input ();
    }

  gui_default_parameter (f, parms, Qno_focus_on_map (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qno_accept_focus (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* Consider frame official, now.  */
  can_set_window_size! (f, true);

  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qx_create_frame_2 ());

  /* Process fullscreen parameter here in the hope that normalizing a fullheight/fullwidth frame
     will produce the size set by the last adjust_frame_size call.  Note that Android only
     supports the `maximized' state.  */
  gui_default_parameter (f, parms, Qfullscreen (), Qmaximized (),
                         c"fullscreen".as_ptr (), c"Fullscreen".as_ptr (),
                         RES_TYPE_SYMBOL);

  /* When called from `x-create-frame-with-faces' visibility is always explicitly nil.  */
  let mut visibility: Lisp_Object
    = gui_display_get_arg (dpyinfo, parms, Qvisibility (), std::ptr::null (),
			   std::ptr::null (), RES_TYPE_SYMBOL);
  let height: Lisp_Object
    = gui_display_get_arg (dpyinfo, parms, Qheight (), std::ptr::null (),
			   std::ptr::null (), RES_TYPE_NUMBER);
  let width: Lisp_Object
    = gui_display_get_arg (dpyinfo, parms, Qwidth (), std::ptr::null (),
			   std::ptr::null (), RES_TYPE_NUMBER);

  if EQ (visibility, Qicon ())
    {
      was_invisible! (f, true);
      android_iconify_frame (f);
    }
  else
    {
      if BASE_EQ (visibility, Qunbound ())
	{ visibility = Qt (); }

      if !NILP (visibility)
	{ android_make_frame_visible (f); }
      else
	{ was_invisible! (f, true); }
    }

  /* Leave f->was_invisible true only if height or width were specified too.  This takes effect
     only when we are not called from `x-create-frame-with-faces' (see above comment).  */
  was_invisible! (f, was_invisible! (f)
		     && (!BASE_EQ (height, Qunbound ()) || !BASE_EQ (width, Qunbound ())));

  store_frame_param (f, Qvisibility (), visibility);

  /* Set whether or not frame synchronization is enabled.  */
  gui_default_parameter (f, parms, Quse_frame_synchronization (), Qt (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* Works iff frame has been already mapped.  */
  gui_default_parameter (f, parms, Qskip_taskbar (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  /* The `z-group' parameter works only for visible frames.  */
  gui_default_parameter (f, parms, Qz_group (), Qnil (),
                         std::ptr::null (), std::ptr::null (), RES_TYPE_SYMBOL);

  /* Initialize `default-minibuffer-frame' in case this is the first frame on this terminal.  */
  /* C 1181-1182: `KVAR (kb, Vdefault_minibuffer_frame)` is keyboard.h 38's token-pasting macro
     (`(kboard)->field ## _`), which Rust cannot spell; keyboard.h 158's member is
     `Vdefault_minibuffer_frame_`, so the port writes the member (the `frame.rs` 1690 shape).  */
  if FRAME_HAS_MINIBUF_P (f)
      && (!FRAMEP ((*kb).Vdefault_minibuffer_frame_)
          || !FRAME_LIVE_P (XFRAME ((*kb).Vdefault_minibuffer_frame_)))
    { kset_default_minibuffer_frame (kb, frame); }

  /* All remaining specified parameters, which have not been "used" by gui_display_get_arg and
     friends, now go in the misc. alist of the frame.  */
  tem = parms;
  while CONSP (tem)
    {
      if CONSP (XCAR (tem)) && !NILP (XCAR (XCAR (tem)))
	{ fset_param_alist (f, Fcons (XCAR (tem), (*f).param_alist)); }
      tem = XCDR (tem);
    }

  /* Make sure windows on this frame appear in calls to next-window and similar functions.  */
  Vwindow_list = Qnil ();

  return unbind_to (count, frame);
  }
});

/* --------------------------------------------------------------------------------------------
   C 1216-1489: the terminal-information `DEFUN`s (C 1216-1424), the multimonitor list's static
   helper (C 1426-1457) and its `DEFUN` (C 1459-1489).  Nine of the sixteen items carry C's
   `#ifdef ANDROID_STUBIFY` / `#else` pair as two cfg'd blocks in C's order (the stub arm is
   `not (not_android_stubify)`, the `Fx_create_frame` shape above), and the two `#ifndef
   ANDROID_STUBIFY` blocks (C 1426, and C 1491's first half outside this slice) take the pair as
   their gate.  `frame_geometry` (C 1491-1622) opens the next slice.  */

/* C 1216-1236.  */
#[cfg (feature = "have_android")]
DEFUN! ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p,
	1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  /* C 1220-1222: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1221
    return Qnil ();                                                // C 1222
  }

  /* C 1223-1235: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  /* C 1224 spells `Emacs_Color foo;` with no initialiser and fills it through C 1231's out
     parameter, so the port zero-initialises it (PORTING.md 1.5; `docs/ports/androidfns.md` names
     it).  */
  let mut foo: Emacs_Color = std::mem::zeroed ();                  // C 1224
  let mut f: *mut frame;                                           // C 1225

  f = decode_window_system_frame (frame);                          // C 1227

  CHECK_STRING (color);                                            // C 1229

  if android_defined_color (f, SSDATA (color), &raw mut foo,       // C 1231
			    false, false)
    { return Qt (); }                                              // C 1232
  else
    { return Qnil (); }                                            // C 1234
  }
});

/* C 1238-1258.  */
#[cfg (feature = "have_android")]
DEFUN! ("xw-color-values", Fxw_color_values, Sxw_color_values,
	1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  /* C 1242-1244: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1243
    return Qnil ();                                                // C 1244
  }

  /* C 1245-1257: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut foo: Emacs_Color = std::mem::zeroed ();                  // C 1246 (PORTING.md 1.5)
  let mut f: *mut frame;                                           // C 1247

  f = decode_window_system_frame (frame);                          // C 1249

  CHECK_STRING (color);                                            // C 1251

  if android_defined_color (f, SSDATA (color), &raw mut foo,       // C 1253
			    false, false)
    /* C's three `unsigned short` members widen to `list3i`'s `EMACS_INT` parameters.  */
    { return list3i (foo.red as EMACS_INT,                         // C 1254
		     foo.green as EMACS_INT,
		     foo.blue as EMACS_INT); }
  else
    { return Qnil (); }                                            // C 1256
  }
});

/* C 1260-1269.  */
#[cfg (feature = "have_android")]
DEFUN! ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let mut dpyinfo: *mut android_display_info;                      // C 1265

  dpyinfo = check_android_display_info (terminal);                 // C 1267
  return if (*dpyinfo).n_planes > 8 { Qt () } else { Qnil () };     // C 1268
});

/* C 1271-1280.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let mut dpyinfo: *mut android_display_info;                      // C 1276

  dpyinfo = check_android_display_info (terminal);                 // C 1278
  return if (*dpyinfo).n_planes > 1 { Qt () } else { Qnil () };     // C 1279
});

/* C 1282-1293.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1287-1289: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1288
    return Qnil ();                                                // C 1289
  }

  /* C 1290-1292: the `#else` arm.  C's `int` result widens to `make_fixnum`'s `EMACS_INT`.  */
  #[cfg (feature = "not_android_stubify")]
  {
    return make_fixnum (android_get_screen_width () as EMACS_INT); // C 1291
  }
});

/* C 1295-1306.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1300-1302: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1301
    return Qnil ();                                                // C 1302
  }

  /* C 1303-1305: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
    return make_fixnum (android_get_screen_height () as EMACS_INT); // C 1304
  }
});

/* C 1308-1318.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-planes", Fx_display_planes, Sx_display_planes,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let mut dpyinfo: *mut android_display_info;                      // C 1313

  dpyinfo = check_android_display_info (terminal);                 // C 1315

  return make_fixnum ((*dpyinfo).n_planes as EMACS_INT);           // C 1317
});

/* C 1320-1337.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let mut dpyinfo: *mut android_display_info;                      // C 1325
  let mut nr_planes: c_int;                                        // C 1326

  dpyinfo = check_android_display_info (terminal);                 // C 1328
  nr_planes = (*dpyinfo).n_planes;                                 // C 1329

  /* Truncate nr_planes to 24 to avoid integer overflow.  */         // C 1331

  if nr_planes > 24                                                // C 1333
    { nr_planes = 24; }                                            // C 1334

  return make_fixnum ((1 << nr_planes) as EMACS_INT);              // C 1336
});

/* C 1339-1350.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-server-vendor", Fx_server_vendor, Sx_server_vendor,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1343-1345: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1344
    return Qnil ();                                                // C 1345
  }

  /* C 1346-1349: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  check_android_display_info (terminal);                           // C 1347
  return Vandroid_build_manufacturer;                              // C 1348
  }
});

/* C 1352-1363.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-server-version", Fx_server_version, Sx_server_version,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1356-1358: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1357
    return Qnil ();                                                // C 1358
  }

  /* C 1359-1362: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  check_android_display_info (terminal);                           // C 1360
  return list3i (android_get_current_api_level () as EMACS_INT,     // C 1361
		 0, 0);
  }
});

/* C 1365-1371.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-screens", Fx_display_screens, Sx_display_screens,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_android_display_info (terminal);                           // C 1369
  return make_fixnum (1);                                          // C 1370
});

/* C 1373-1384.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1377-1379: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1378
    return Qnil ();                                                // C 1379
  }

  /* C 1380-1383: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  check_android_display_info (terminal);                           // C 1381
  return make_fixnum (android_get_mm_width () as EMACS_INT);       // C 1382
  }
});

/* C 1386-1397.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  /* C 1390-1392: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1391
    return Qnil ();                                                // C 1392
  }

  /* C 1393-1396: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  check_android_display_info (terminal);                           // C 1394
  return make_fixnum (android_get_mm_height () as EMACS_INT);      // C 1395
  }
});

/* C 1399-1409.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_android_display_info (terminal);                           // C 1404

  /* Window contents are preserved insofar as they remain mapped, in a
     fashion tantamount to WhenMapped.  */                        // C 1406-1407
  return Qwhen_mapped ();                                          // C 1408
});

/* C 1411-1424.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let mut dpyinfo: *mut android_display_info;                      // C 1416

  dpyinfo = check_android_display_info (terminal);                 // C 1418

  if (*dpyinfo).n_planes < 24                                      // C 1420
    { return Qstatic_gray (); }                                    // C 1421

  return Qtrue_color ();                                           // C 1423
});

/* Two use-site copies (P-72 rule 4; the `Display_Info` shape at line 412): frame.h 1926's
   `#ifdef HAVE_WINDOW_SYSTEM` gates `struct MonitorInfo` and `make_monitor_attribute_list`, and
   `include/frame.rs` 1936/1953 carry the pair under `have_window_system` - a feature this unit's
   gate set does not include - so the port supplies C's own record and declaration under C's own
   names until the mirror's gate matches.  The complement gate is what keeps the two copies from
   colliding, and the layout assertions below are the ones the mirror's own comment records
   (sizeof 48 / align 8, members at 0 / 16 / 32 / 36 / 40; the non-X `Emacs_Rectangle` this
   configuration's dispextern.h carries).  `docs/ports/androidfns.md` names both as mirror gaps.  */
/// C: frame.h 1934: `struct MonitorInfo { Emacs_Rectangle geom, work; int mm_width, mm_height; char *name; };`
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
#[repr(C)]
pub struct MonitorInfo
{
  pub geom: Emacs_Rectangle,                                       // C 1934
  pub work: Emacs_Rectangle,
  pub mm_width: c_int,
  pub mm_height: c_int,
  pub name: *mut c_char,
}
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::size_of::<MonitorInfo> () == 48);
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::offset_of! (MonitorInfo, geom) == 0);
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::offset_of! (MonitorInfo, work) == 16);
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::offset_of! (MonitorInfo, mm_width) == 32);
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::offset_of! (MonitorInfo, mm_height) == 36);
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
const _: () = core::assert! (core::mem::offset_of! (MonitorInfo, name) == 40);
/// C: frame.h 1938: `extern Lisp_Object make_monitor_attribute_list (struct MonitorInfo *monitors,`
/// (C 1939 continues `int n_monitors, int primary_monitor, Lisp_Object monitor_frames, const char *source);`)
#[cfg (all (feature = "have_android", not (feature = "have_window_system")))]
unsafe extern "C" {
  pub fn make_monitor_attribute_list (monitors: *mut MonitorInfo, n_monitors: c_int,
                                      primary_monitor: c_int, monitor_frames: Lisp_Object,
                                      source: *const c_char) -> Lisp_Object;
}

/* C 1426-1457: `#ifndef ANDROID_STUBIFY` (C 1426) around the static helper.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_make_monitor_attribute_list (monitors: *mut MonitorInfo,
					       n_monitors: c_int,
					       primary_monitor: c_int) -> Lisp_Object
{                                                              // C 1428-1455
  let mut monitor_frames: Lisp_Object;                             // C 1433
  let mut frame: Lisp_Object;                                      // C 1434
  let mut rest: Lisp_Object;
  let mut f: *mut frame;                                           // C 1435

  monitor_frames = make_nil_vector (n_monitors as isize);          // C 1437

  /* C's `FOR_EACH_FRAME (rest, frame)` takes the body as its last argument (the mirror's
     `FOR_EACH_FRAME!`; docs/ports/macro-body-notes.tsv).  */
  FOR_EACH_FRAME! (rest, frame, {                                  // C 1439
      f = XFRAME (frame);                                          // C 1441

      /* Associate all frames with the primary monitor.  */         // C 1443

      if FRAME_WINDOW_P (f)                                        // C 1445
	  && !FRAME_TOOLTIP_P (f)                                  // C 1446
	{ ASET (monitor_frames, primary_monitor as isize,          // C 1447
		Fcons (frame, AREF (monitor_frames,                // C 1448
				    primary_monitor as isize)));   // C 1449
	}
  });

  return make_monitor_attribute_list (monitors, n_monitors,        // C 1452
				      primary_monitor,
                                      monitor_frames, std::ptr::null ());  // C 1454
}

/* C 1459-1489.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-display-monitor-attributes-list", Fandroid_display_monitor_attributes_list,
	 Sandroid_display_monitor_attributes_list,
	 0, 1, 0,
	 doc: "Return a list of physical monitor attributes on the X display TERMINAL.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nInternal use only, use `display-monitor-attributes-list' instead.  ",
	 (terminal: Lisp_Object),
	 {
  /* C 1472-1474: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1473
    return Qnil ();                                                // C 1474
  }

  /* C 1475-1488: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  /* C 1476 spells `struct MonitorInfo monitor;` with no initialiser and fills it field by field
     from C 1479, so the port zero-initialises it (PORTING.md 1.5).  */
  let mut monitor: MonitorInfo = std::mem::zeroed ();              // C 1476

  check_android_display_info (terminal);                           // C 1478
  memset (&raw mut monitor as *mut c_void, 0,                      // C 1479
	  std::mem::size_of::<MonitorInfo> ());
  monitor.geom.width = android_get_screen_width () as c_uint;      // C 1480
  monitor.geom.height = android_get_screen_height () as c_uint;    // C 1481
  monitor.mm_width = android_get_mm_width ();                      // C 1482
  monitor.mm_height = android_get_mm_height ();                    // C 1483
  monitor.work = monitor.geom;                                     // C 1484
  monitor.name = c"Android device monitor".as_ptr () as *mut c_char;  // C 1485

  return android_make_monitor_attribute_list (&raw mut monitor, 1, 0);  // C 1487
  }
});

/* C 1490-1704: `frame_geometry` under C 1491's `#ifndef ANDROID_STUBIFY` (C 1491-1624) and the two
   `DEFUN`s that call it - `Fandroid_frame_geometry` (C 1626-1675) and `Fandroid_frame_edges`
   (C 1677-1704).  The last two carry C's pair in C's own order, which for `Fandroid_frame_edges`
   is the **real arm first** (C 1696's `#ifndef ANDROID_STUBIFY`, C 1701's `#else`).  */

/* One use-site declaration (P-72 rule 4): `decode_live_frame`'s C home is frame.h 1476 and no
   mirror in this unit's include graph carries it (`include/frame.rs` has no such row; `buffer.rs`
   92 keeps its own copy inside an `extern` block), so the unit declares it under C's name.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);`
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
}

/* C 1491's `#ifndef ANDROID_STUBIFY` around the file's first static helper of the frame-geometry
   family; the `#endif` is C 1624.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn frame_geometry (frame: Lisp_Object, attribute: Lisp_Object) -> Lisp_Object
{                                                              // C 1493-1622
  let f: *mut frame = decode_live_frame (frame);                  // C 1496
  /* C 1497-1498 declare these three with no initialiser and C 1515 fills them through out
     parameters, so the port zero-initialises them (PORTING.md 1.5).  */
  let mut rootw: android_window = 0;                              // C 1497
  let mut native_width: c_uint = 0;                               // C 1498
  let mut native_height: c_uint = 0;
  let mut x_border_width: c_uint = 0;
  let mut x_native: c_int = 0;                                    // C 1499
  let mut y_native: c_int = 0;
  let mut xptr: c_int = 0;
  let mut yptr: c_int = 0;
  let mut left_off: c_int = 0;                                    // C 1500
  let mut right_off: c_int = 0;
  let mut top_off: c_int = 0;
  let mut bottom_off: c_int = 0;
  let mut outer_left: c_int;                                      // C 1501
  let mut outer_top: c_int;
  let mut outer_right: c_int;
  let mut outer_bottom: c_int;
  let mut native_left: c_int;                                     // C 1502
  let mut native_top: c_int;
  let mut native_right: c_int;
  let mut native_bottom: c_int;
  let mut inner_left: c_int;                                      // C 1503
  let mut inner_top: c_int;
  let mut inner_right: c_int;
  let mut inner_bottom: c_int;
  let mut internal_border_width: c_int;                           // C 1504
  let mut menu_bar_external: bool = false;                        // C 1505
  let mut tool_bar_external: bool = false;
  let mut menu_bar_height: c_int = 0;                             // C 1506
  let mut menu_bar_width: c_int = 0;
  let mut tab_bar_height: c_int = 0;                              // C 1507
  let mut tab_bar_width: c_int = 0;
  let mut tool_bar_height: c_int = 0;                             // C 1508
  let mut tool_bar_width: c_int = 0;

  /* C's `!FRAME_ANDROID_WINDOW (f)` tests an integer handle (`android_window` is `android_handle`
     = `intptr_t`), so the port's zero test is `== 0` (the slice-1 shape at line 569).  */
  if FRAME_INITIAL_P (f) || !FRAME_ANDROID_P (f)                  // C 1510
      || FRAME_ANDROID_WINDOW! (f) == 0                           // C 1511
    { return Qnil (); }                                            // C 1512

  block_input ();                                                 // C 1514
  android_get_geometry (FRAME_ANDROID_WINDOW! (f),                // C 1515
			&raw mut rootw, &raw mut x_native, &raw mut y_native,
			&raw mut native_width, &raw mut native_height, &raw mut x_border_width);
  unblock_input ();                                               // C 1518

  if ! FRAME_PARENT_FRAME (f).is_null ()                          // C 1520
    {
      let mut parent: Lisp_Object;                                // C 1522
      let mut edges: Lisp_Object;

      XSETFRAME! (parent, FRAME_PARENT_FRAME (f));                 // C 1524
      edges = Fandroid_frame_edges (parent, Qnative_edges ());     // C 1525
      if !NILP (edges)                                            // C 1526
	{
	  x_native += XFIXNUM (Fnth (make_fixnum (0), edges)) as c_int;   // C 1528
	  y_native += XFIXNUM (Fnth (make_fixnum (1), edges)) as c_int;   // C 1529
	}

      outer_left = x_native;                                      // C 1532
      outer_top = y_native;                                       // C 1533
      /* C's `outer_left + native_width + 2 * x_border_width` is int + unsigned int + unsigned int
	 and lands in an `int` local; the two `as c_int` are C's own narrowing at that assignment
	 (`docs/ports/androidfns.md` Deviations).  */
      outer_right = outer_left + native_width as c_int + 2 * x_border_width as c_int;   // C 1534
      outer_bottom = outer_top + native_height as c_int + 2 * x_border_width as c_int;  // C 1535

      native_left = x_native + x_border_width as c_int;            // C 1537
      native_top = y_native + x_border_width as c_int;             // C 1538
      native_right = native_left + native_width as c_int;          // C 1539
      native_bottom = native_top + native_height as c_int;         // C 1540
    }
  else
    {
      outer_left = xptr;                                          // C 1544
      outer_top = yptr;                                           // C 1545
      outer_right = outer_left + left_off + native_width as c_int + right_off;     // C 1546
      outer_bottom = outer_top + top_off + native_height as c_int + bottom_off;    // C 1547

      native_left = outer_left + left_off;                         // C 1549
      native_top = outer_top + top_off;                            // C 1550
      native_right = native_left + native_width as c_int;          // C 1551
      native_bottom = native_top + native_height as c_int;         // C 1552
    }

  internal_border_width = FRAME_INTERNAL_BORDER_WIDTH (f);         // C 1555
  inner_left = native_left + internal_border_width;                // C 1556
  inner_top = native_top + internal_border_width;                  // C 1557
  inner_right = native_right - internal_border_width;              // C 1558
  inner_bottom = native_bottom - internal_border_width;            // C 1559

  menu_bar_height = FRAME_MENU_BAR_HEIGHT (f);                     // C 1561
  inner_top += menu_bar_height;                                    // C 1562
  menu_bar_width = if menu_bar_height != 0                        // C 1563
		   { native_width as c_int }
		   else
		   { 0 };

  tab_bar_height = FRAME_TAB_BAR_HEIGHT (f);                       // C 1565
  tab_bar_width = if tab_bar_height != 0                           // C 1566-1568
		  { native_width as c_int - 2 * internal_border_width }
		  else
		  { 0 };
  inner_top += tab_bar_height;                                     // C 1569

  tool_bar_height = FRAME_TOOL_BAR_HEIGHT (f);                     // C 1571
  tool_bar_width = if tool_bar_height != 0                         // C 1572-1574
		   { native_width as c_int - 2 * internal_border_width }
		   else
		   { 0 };

  /* Subtract or add to the inner dimensions based on the tool bar
     position.  */                                                // C 1576-1577

  if EQ (FRAME_TOOL_BAR_POSITION (f), Qtop ())                     // C 1579
    { inner_top += tool_bar_height; }                              // C 1580
  else
    { inner_bottom -= tool_bar_height; }                           // C 1582

  /* Construct list.  */                                           // C 1584
  if EQ (attribute, Qouter_edges ())                               // C 1585
    { return list4i (outer_left as EMACS_INT, outer_top as EMACS_INT,
		     outer_right as EMACS_INT, outer_bottom as EMACS_INT); }      // C 1586
  else if EQ (attribute, Qnative_edges ())                         // C 1587
    { return list4i (native_left as EMACS_INT, native_top as EMACS_INT,
		     native_right as EMACS_INT, native_bottom as EMACS_INT); }    // C 1588
  else if EQ (attribute, Qinner_edges ())                          // C 1589
    { return list4i (inner_left as EMACS_INT, inner_top as EMACS_INT,
		     inner_right as EMACS_INT, inner_bottom as EMACS_INT); }      // C 1590
  else
    { return                                                       // C 1592
       list! (Fcons (Qouter_position (),                            // C 1593
		     Fcons (make_fixnum (outer_left as EMACS_INT),
			    make_fixnum (outer_top as EMACS_INT))),
	      Fcons (Qouter_size (),                                // C 1596
		     Fcons (make_fixnum ((outer_right - outer_left) as EMACS_INT),
			    make_fixnum ((outer_bottom - outer_top) as EMACS_INT))),
	      /* Approximate.  */                                    // C 1599
	      Fcons (Qexternal_border_size (),                       // C 1600
		     Fcons (make_fixnum (right_off as EMACS_INT),
			    make_fixnum (bottom_off as EMACS_INT))),
	      Fcons (Qouter_border_width (),                         // C 1603
		     make_fixnum (x_border_width as EMACS_INT)),
	      /* Approximate.  */                                    // C 1604
	      Fcons (Qtitle_bar_size (),                             // C 1605
		     Fcons (make_fixnum (0),
			    make_fixnum ((top_off - bottom_off) as EMACS_INT))),
	      Fcons (Qmenu_bar_external (),                          // C 1608
		     if menu_bar_external { Qt () } else { Qnil () }),
	      Fcons (Qmenu_bar_size (),                              // C 1609
		     Fcons (make_fixnum (menu_bar_width as EMACS_INT),
			    make_fixnum (menu_bar_height as EMACS_INT))),
	      Fcons (Qtab_bar_size (),                               // C 1612
		     Fcons (make_fixnum (tab_bar_width as EMACS_INT),
			    make_fixnum (tab_bar_height as EMACS_INT))),
	      Fcons (Qtool_bar_external (),                          // C 1615
		     if tool_bar_external { Qt () } else { Qnil () }),
	      Fcons (Qtool_bar_position (), FRAME_TOOL_BAR_POSITION (f)),   // C 1616
	      Fcons (Qtool_bar_size (),                              // C 1617
		     Fcons (make_fixnum (tool_bar_width as EMACS_INT),
			    make_fixnum (tool_bar_height as EMACS_INT))),
	      Fcons (Qinternal_border_width (),                      // C 1620
		     make_fixnum (internal_border_width as EMACS_INT)));
    }
}

/* C 1626-1675.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-frame-geometry", Fandroid_frame_geometry, Sandroid_frame_geometry,
	0, 1, 0,
	doc: "Return geometric attributes of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is an association list of the attributes listed below.  All height\nand width values are in pixels.\n\n`outer-position' is a cons of the outer left and top edges of FRAME\n  relative to the origin - the position (0, 0) - of FRAME's display.\n\n`outer-size' is a cons of the outer width and height of FRAME.  The\n  outer size includes the title bar and the external borders as well as\n  any menu and/or tool bar of frame.\n\n`external-border-size' is a cons of the horizontal and vertical width of\n  FRAME's external borders as supplied by the window manager.\n\n`title-bar-size' is a cons of the width and height of the title bar of\n  FRAME as supplied by the window manager.  If both of them are zero,\n  FRAME has no title bar.  If only the width is zero, Emacs was not\n  able to retrieve the width information.\n\n`menu-bar-external', if non-nil, means the menu bar is external (never\n  included in the inner edges of FRAME).\n\n`menu-bar-size' is a cons of the width and height of the menu bar of\n  FRAME.\n\n`tool-bar-external', if non-nil, means the tool bar is external (never\n  included in the inner edges of FRAME).\n\n`tool-bar-position' tells on which side the tool bar on FRAME is and can\n  be one of `left', `top', `right' or `bottom'.  If this is nil, FRAME\n  has no tool bar.\n\n`tool-bar-size' is a cons of the width and height of the tool bar of\n  FRAME.\n\n`internal-border-width' is the width of the internal border of\n  FRAME.  ",
	(frame: Lisp_Object),
	{
  /* C 1669-1671: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 1670
    return Qnil ();                                                // C 1671
  }

  /* C 1672-1674: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  return frame_geometry (frame, Qnil ());                          // C 1673
  }
});

/* C 1677-1704.  The real arm comes first, as C's `#ifndef` does.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-frame-edges", Fandroid_frame_edges, Sandroid_frame_edges,
	0, 2, 0,
	doc: "Return edge coordinates of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a list of the form (LEFT, TOP, RIGHT, BOTTOM).  All values are\nin pixels relative to the origin - the position (0, 0) - of FRAME's\ndisplay.\n\nIf optional argument TYPE is the symbol `outer-edges', return the outer\nedges of FRAME.  The outer edges comprise the decorations of the window\nmanager (like the title bar or external borders) as well as any external\nmenu or tool bar of FRAME.  If optional argument TYPE is the symbol\n`native-edges' or nil, return the native edges of FRAME.  The native\nedges exclude the decorations of the window manager and any external\nmenu or tool bar of FRAME.  If TYPE is the symbol `inner-edges', return\nthe inner edges of FRAME.  These edges exclude title bar, any borders,\nmenu bar or tool bar of FRAME.  ",
	(frame: Lisp_Object, r#type: Lisp_Object),
	{
  /* C 1696-1700: the real arm (`#ifndef ANDROID_STUBIFY`).  */
  #[cfg (feature = "not_android_stubify")]
  {
  return frame_geometry (frame, if EQ (r#type, Qouter_edges ())      // C 1697-1700
			       || EQ (r#type, Qinner_edges ())
			 { r#type } else { Qnative_edges () });
  }

  /* C 1701-1703: the `#else` arm.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
  return Qnil ();                                                  // C 1702
  }
});

/* C 1705-1800: `android_frame_list_z_order` under C 1706's `#ifndef ANDROID_STUBIFY` (C 1706-1743),
   the `DEFUN` that calls it (C 1745-1778) and `android_frame_restack` under C 1780's
   `#ifndef ANDROID_STUBIFY` (C 1780-1800, C's `#endif /* !ANDROID_STUBIFY */`).  */

/* C 1706's `#ifndef ANDROID_STUBIFY` around the static helper; the `#endif` is C 1743.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_list_z_order (dpyinfo: *mut android_display_info,
				      window: android_window) -> Lisp_Object
{                                                              // C 1708-1741
  /* C 1712-1714 declare these five with no initialiser; C 1719 fills the four out parameters
     and C 1722 assigns `i`, so the port zero-initialises the four (PORTING.md 1.5, the same
     reading slice 3's probe gave).  */
  let mut root: android_window = 0;                               // C 1712
  let mut parent: android_window = 0;
  let mut children: *mut android_window = std::ptr::null_mut ();
  let mut nchildren: c_uint = 0;                                  // C 1713
  let mut i: c_ulong;                                             // C 1714
  let mut frames: Lisp_Object;                                    // C 1715

  frames = Qnil ();                                               // C 1717

  /* C tests the `int` result as a truth value; Rust needs the comparison (PROBE reading: the
     bare form is `E0308`, expected `bool`).  */
  if android_query_tree (window, &raw mut root, &raw mut parent,   // C 1719
			 &raw mut children, &raw mut nchildren) != 0
    {
      /* C's `for (i = 0; i < nchildren; i++)` is the while-with-its-increment-at-the-end the
	 mirror's `FOR_EACH_FRAME!` documents (`docs/ports/macro-body-notes.tsv`); the body has no
	 `continue`, so the increment still runs exactly where C's does.  */
      i = 0;                                                      // C 1722
      while i < nchildren as c_ulong
	{
	  let mut frame: Lisp_Object;                             // C 1724
	  let mut tail: Lisp_Object;

	  FOR_EACH_FRAME! (tail, frame, {                         // C 1726
	      let cf: *mut frame = XFRAME (frame);                // C 1728

	      if FRAME_ANDROID_P (cf)                             // C 1730
		  && FRAME_ANDROID_WINDOW! (cf) == *children.offset (i as isize)   // C 1731
		{ frames = Fcons (frame, frames); }               // C 1732
	  });
	  i += 1;
	}

      if ! children.is_null ()                                    // C 1736
	{ xfree (children as *mut c_void); }                      // C 1737
    }

  return frames;                                                  // C 1740
}

/* C 1745-1778.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-frame-list-z-order", Fandroid_frame_list_z_order,
	 Sandroid_frame_list_z_order,
	 0, 1, 0,
	 doc: "Return list of Emacs's frames, in Z (stacking) order.\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be either a frame or a display name (a string).  If\nomitted or nil, that stands for the selected frame's display.  Return\nnil if TERMINAL contains no Emacs frame.\n\nAs a special case, if TERMINAL is non-nil and specifies a live frame,\nreturn the child frames of that frame in Z (stacking) order.\n\nFrames are listed from topmost (first) to bottommost (last).\n\nOn Android, the order of the frames returned is undefined unless\nTERMINAL is a frame.  ",
	 (terminal: Lisp_Object),
	 {
  /* C 1762-1764: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());  // C 1763
    return Qnil ();                                               // C 1764
  }

  /* C 1765-1777: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut dpyinfo: *mut android_display_info;                     // C 1766
  let mut window: android_window;                                 // C 1767

  dpyinfo = check_android_display_info (terminal);                // C 1769

  if FRAMEP (terminal) && FRAME_LIVE_P (XFRAME (terminal))         // C 1771
    { window = FRAME_ANDROID_WINDOW! (XFRAME (terminal)); }        // C 1772
  else
    { window = (*dpyinfo).root_window; }                           // C 1774

  return android_frame_list_z_order (dpyinfo, window);             // C 1776
  }
});

/* C 1780's `#ifndef ANDROID_STUBIFY` around `android_frame_restack`; C's own `#endif` marker for
   that block sits at C 1800, and its trailing comment names the negated macro.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_restack (f1: *mut frame, f2: *mut frame, above_flag: bool)
{                                                              // C 1782-1798
  let mut window1: android_window;                                // C 1786
  let mut wc: android_window_changes = std::mem::zeroed ();       // C 1787
  let mut mask: c_ulong;                                          // C 1788

  window1 = FRAME_ANDROID_WINDOW! (f1);                           // C 1790
  wc.sibling = FRAME_ANDROID_WINDOW! (f2);                        // C 1791
  wc.stack_mode = if above_flag { ANDROID_ABOVE } else { ANDROID_BELOW };   // C 1792
  /* C's `mask` is `unsigned long` and the two constants are `enum android_wc_value_mask` (an
     `int`), so C widens at this assignment and narrows again at C 1796's call; the two casts are
     those two conversions spelled out.  */
  mask = (ANDROID_CW_SIBLING | ANDROID_CW_STACK_MODE) as c_ulong;  // C 1793

  block_input ();                                                 // C 1795
  android_reconfigure_wm_window (window1, mask as android_wc_value_mask,   // C 1796
				 &raw mut wc);
  unblock_input ();                                               // C 1797
}

/* C 1801-1910: `Fandroid_frame_restack` (C 1802-1837), the two mouse-position `DEFUN`s
   (C 1839-1862), `Fandroid_get_connection` (C 1864-1895) and `Fx_display_list` (C 1897-1910).
   C 1912 opens the `#ifndef ANDROID_STUBIFY` block that the tooltip frame helpers live in.  */

/* C 1802-1837.  C 1815's `\(stacking)` is make-docfile's escape, whose *value* is `(stacking)`,
   so the literal spells the value: the tool reads `subr Fandroid_frame_restack: docstring
   differs` while the literal carries `\\(` - the measurement `pgtkfns.rs` 2801-2804 records for
   the same sentence in the pgtk unit.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-frame-restack", Fandroid_frame_restack, Sandroid_frame_restack,
	2, 3, 0,
	doc: "Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nThis may be thought of as an atomic action performed in two steps: The\nfirst step removes FRAME1's window-step window from the display.  The\nsecond step reinserts FRAME1's window below (above if ABOVE is true)\nthat of FRAME2.  Hence the position of FRAME2 in its display's Z\n(stacking) order relative to all other frames excluding FRAME1 remains\nunaltered.\n\nAndroid does not facilitate restacking top-level windows managed by\nits own window manager; nor is it possible to restack frames that are\nchildren of different parents.  Consequently, this function only\nfunctions when FRAME1 and FRAME2 are both child frames subordinate to\nthe same parent frame.  ",
	(frame1: Lisp_Object, frame2: Lisp_Object, above: Lisp_Object),
	{
  /* C 1825-1827: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());  // C 1826
    return Qnil ();                                               // C 1827
  }

  /* C 1828-1836: the `#else /* !ANDROID_STUBIFY */` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let f1: *mut frame = decode_live_frame (frame1);                // C 1829
  let f2: *mut frame = decode_live_frame (frame2);                // C 1830

  /* C's `!(a && b)` over two integer handles; both are `android_handle` here, so the truth tests
     are zero comparisons.  */
  if ! (FRAME_ANDROID_WINDOW! (f1) != 0 && FRAME_ANDROID_WINDOW! (f2) != 0)   // C 1832
    { error (c"Cannot restack frames".as_ptr ()); }               // C 1833
  android_frame_restack (f1, f2, !NILP (above));                  // C 1834
  return Qt ();                                                   // C 1835
  }
});

/* C 1839-1850.  C's parameter list is `(void)`.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-mouse-absolute-pixel-position", Fandroid_mouse_absolute_pixel_position,
	 Sandroid_mouse_absolute_pixel_position,
	 0, 0, 0,
	 doc: "Return absolute position of mouse cursor in pixels.\nThe position is returned as a cons cell (X . Y) of the coordinates of\nthe mouse cursor position in pixels relative to a position (0, 0) of the\nselected frame's display.  This does not work on Android.  ",
	 (),
	 {
  /* This cannot be implemented on Android.  */                   // C 1848
  return Qnil ();                                                 // C 1849
});

/* C 1852-1862.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-set-mouse-absolute-pixel-position", Fandroid_set_mouse_absolute_pixel_position,
	 Sandroid_set_mouse_absolute_pixel_position,
	 2, 2, 0,
	 doc: "Move mouse pointer to a pixel position at (X, Y).  The\ncoordinates X and Y are interpreted to start from the top-left corner\nof the screen.  This does not work on Android.  ",
	 (x: Lisp_Object, y: Lisp_Object),
	 {
  /* This cannot be implemented on Android.  */                   // C 1860
  return Qnil ();                                                 // C 1861
});

/* C 1864-1895.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-get-connection", Fandroid_get_connection, Sandroid_get_connection,
	0, 0, 0,
	doc: "Get the connection to the display server.\nReturn the terminal if it exists, else nil.\n\nEmacs cannot open a connection to the display server itself under\nAndroid, so there is no equivalent of `x-open-connection'.  ",
	(),
	{
  /* C 1873-1875: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());  // C 1874
    return Qnil ();                                               // C 1875
  }

  /* C 1876-1894: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut terminal: Lisp_Object;                                  // C 1877

  terminal = Qnil ();                                             // C 1879

  if x_display_list != std::ptr::null_mut ()                      // C 1881 (C's `if (x_display_list)`)
    {
      XSETTERMINAL! (terminal, (*x_display_list).terminal);       // C 1883

      /* Update the display's bit depth from
	 `android_display_planes'.  */                            // C 1885-1886
      (*x_display_list).n_planes
	= (if android_display_planes > 8                          // C 1888
	   { 24 } else { if android_display_planes > 1            // C 1889
			  { android_display_planes as c_int }     // C 1890
			  else { 1 } });
    }

  return terminal;                                                // C 1893
  }
});

/* C 1897-1910.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-display-list", Fx_display_list, Sx_display_list,
	0, 0, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(),
	{
  let mut result: Lisp_Object;                                    // C 1901

  result = Qnil ();                                               // C 1903

  if x_display_list != std::ptr::null_mut ()                      // C 1905
    { result = Fcons (XCAR ((*x_display_list).name_list_element),  // C 1906
		      result); }
  return result;                                                  // C 1909
});

/* C 1911-2170: `unwind_create_tip_frame` (C 1914-1925) and `android_create_tip_frame`
   (C 1927-2169).  Both are inside C 1912's `#ifndef ANDROID_STUBIFY`, whose `#endif` is C 2296
   (the tooltip helpers at C 2171-2295 are inside it too, and are the next slice).  The
   body is C's tooltip-frame twin of `Fx_create_frame` (slice 1), so its spellings are this
   file's own: the l-value frame macros as member paths (C 871's note above), the flag writers
   through the file's `set_frame_flag1!`/`set_frame_flag2!`, and the zero initialisers
   PORTING.md 1.5 asks for.  */

/* frame.h 457's `bool_bf tooltip : 1;` sits under `#ifdef HAVE_WINDOW_SYSTEM`, and
   `include/frame.rs` gates its shift constant the same way (596-603), so this unit's gate set
   (`have_android` + `not_android_stubify`, without `have_window_system`) has no constant at all.
   The write C 1968 makes is spelled with the number the Android arm reads (frame.rs 597's
   `A: x11/pgtk/android/w32`, 30) under C's own name, so the copy retires itself when the mirror's
   gate matches (the `FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT` shape at line 271).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify",
           not (feature = "have_window_system")))]
const FRAME_TOOLTIP_SHIFT: u32 = 30;
/// C: frame.h 457: `bool_bf tooltip : 1;`  - A 1:30; B 1:31; C 1:30 (measured)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! tooltip {
  ($f:expr) => { (*$f).bits1 >> FRAME_TOOLTIP_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_TOOLTIP_SHIFT, $v) };
}
/// C: frame.h 402: `bool_bf wants_modeline : 1;`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! wants_modeline {
  ($f:expr) => { (*$f).bits1 >> FRAME_WANTS_MODELINE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WANTS_MODELINE_SHIFT, $v) };
}

/* One more use-site declaration (P-72 rule 4): dispextern.h 2068's `extern bool face_change;` is
   inside the mirror's `dispextern_unit_names` module (the `haikufns.rs` 856-858 note), so this
   unit names it here under C's name.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 2068: `extern bool face_change;`
  pub static mut face_change: bool;
}

/* C takes this helper's address for C 1954's `record_unwind_protect`, so it is
   `unsafe extern "C" fn` (the `do_unwind_create_frame` shape the file's header names).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn unwind_create_tip_frame (frame: Lisp_Object)
{                                                              // C 1914-1925
  let mut deleted: Lisp_Object;                                   // C 1917

  deleted = unwind_create_frame (frame);                          // C 1919
  if EQ (deleted, Qt ())                                          // C 1920
    {
      /* C's `ANDROID_NONE` is `static const int` (androidgui.h 259) and `tip_window` is
	 `android_window` (`intptr_t`), so C widens here.  */
      tip_window = ANDROID_NONE as android_window;                // C 1922
      tip_frame = Qnil ();                                        // C 1923
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_create_tip_frame (dpyinfo: *mut android_display_info,
				    mut parms: Lisp_Object) -> Lisp_Object
{                                                              // C 1927-2169
  let mut f: *mut frame;                                          // C 1931
  let mut frame: Lisp_Object;                                     // C 1932
  let mut name: Lisp_Object;                                      // C 1933
  let count: specpdl_ref = SPECPDL_INDEX ();                      // C 1934
  let face_change_before: bool = face_change;                     // C 1935

  if (*((*dpyinfo).terminal)).name.is_null ()                     // C 1937
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }   // C 1938

  parms = Fcopy_alist (parms);                                    // C 1940

  /* Get the name of the frame to use for resource lookup.  */     // C 1942
  name = gui_display_get_arg (dpyinfo, parms, Qname (),           // C 1943
			      c"name".as_ptr (), c"Name".as_ptr (),
			      RES_TYPE_STRING);
  if !STRINGP (name)                                              // C 1945
      && !BASE_EQ (name, Qunbound ())
      && !NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }   // C 1948

  frame = Qnil ();                                                // C 1950
  f = make_frame (false);                                         // C 1951
  wants_modeline! (f, false);                                     // C 1952
  XSETFRAME! (frame, f);                                          // C 1953
  record_unwind_protect (Some (unwind_create_tip_frame), frame);   // C 1954

  (*f).terminal = (*dpyinfo).terminal;                            // C 1956

  /* By setting the output method, we're essentially saying that
     the frame is live, as per FRAME_LIVE_P.  If we get a signal
     from this point on, x_destroy_window might screw up reference
     counts etc.  */                                              // C 1958-1961
  output_method! (f, output_android);                             // C 1962
  (*f).output_data.android = xzalloc (std::mem::size_of::<android_output> ());  // C 1963
  /* C 1964: `FRAME_FONTSET (f) = -1;` - C's macro is an l-value and the mirror carries the read
     arm only, so the write is C's own member path (the slice-1 note at line 1216).  */
  (*((*f).output_data.android as *mut android_output)).fontset = -1;                  // C 1964
  (*((*f).output_data.android as *mut android_output)).white_relief.pixel = (-1 as c_long) as c_ulong;   // C 1965
  (*((*f).output_data.android as *mut android_output)).black_relief.pixel = (-1 as c_long) as c_ulong;   // C 1966

  tooltip! (f, true);                                             // C 1968
  fset_icon_name (f, Qnil ());                                    // C 1969
  /* C 1970: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as C 1964.  */
  (*((*f).output_data.android as *mut android_output)).display_info = dpyinfo;
  (*((*f).output_data.android as *mut android_output)).parent_desc
    = (*FRAME_DISPLAY_INFO! (f)).root_window;                     // C 1971

  /* These colors will be set anyway later, but it's important
     to get the color reference counts right, so initialize them!  */   // C 1973-1974
  {
    let black: Lisp_Object;                                       // C 1976

    /* Function android_decode_color can signal an error.  Make sure
       to initialize color slots so that we won't try to free colors
       we haven't allocated.  */                                  // C 1978-1980
    FRAME_FOREGROUND_PIXEL! (f) = (-1 as c_long) as c_ulong;      // C 1981
    FRAME_BACKGROUND_PIXEL! (f) = (-1 as c_long) as c_ulong;      // C 1982
    (*((*f).output_data.android as *mut android_output)).cursor_pixel = (-1 as c_long) as c_ulong;   // C 1983
    (*((*f).output_data.android as *mut android_output)).cursor_foreground_pixel = (-1 as c_long) as c_ulong;   // C 1984
    (*((*f).output_data.android as *mut android_output)).mouse_pixel = (-1 as c_long) as c_ulong;    // C 1985

    black = build_string (c"black".as_ptr ());                    // C 1987
    FRAME_FOREGROUND_PIXEL! (f)
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));  // C 1988-1989
    FRAME_BACKGROUND_PIXEL! (f)
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));  // C 1990-1991
    (*((*f).output_data.android as *mut android_output)).cursor_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));  // C 1992-1993
    (*((*f).output_data.android as *mut android_output)).cursor_foreground_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));  // C 1994-1995
    (*((*f).output_data.android as *mut android_output)).mouse_pixel
      = android_decode_color (f, black, BLACK_PIX_DEFAULT! (f));  // C 1996-1997
  }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */                                                  // C 2000-2001
  if BASE_EQ (name, Qunbound ()) || NILP (name)                   // C 2002
    { explicit_name! (f, false); }                                // C 2003
  else
    {
      fset_name (f, name);                                        // C 2006
      explicit_name! (f, true);                                   // C 2007
      /* use the frame's title when getting resources for this frame.  */   // C 2008
      specbind (Qx_resource_name (), name);                       // C 2009
    }

  register_font_driver (&raw mut androidfont_driver as *const c_void, f);        // C 2012
  register_font_driver (&raw mut android_sfntfont_driver as *const c_void, f);   // C 2013

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),     // C 2015
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values that are
     needed to determine window geometry.  */                     // C 2018-2019
  android_default_font_parameter (f, parms);                      // C 2020

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),            // C 2022
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */                                                     // C 2025-2027
  if NILP (Fassq (Qinternal_border_width (), parms))              // C 2028
    {
      let value: Lisp_Object;                                     // C 2030

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),     // C 2032
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())                           // C 2035
	{ parms = Fcons (Fcons (Qinternal_border_width (), value),   // C 2036
			 parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (1),    // C 2040
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),      // C 2043
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),     // C 2045
			 std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);

  /* Also do the stuff which must be set before the window exists.  */             // C 2048
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),   // C 2049
			 c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),   // C 2051
			 c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),        // C 2053
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qcursor_color (), build_string (c"black".as_ptr ()),       // C 2055
			 c"cursorColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),       // C 2057
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),                            // C 2059
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  {
    let mut attrs: android_set_window_attributes = std::mem::zeroed ();   // C 2063
    let mut mask: c_ulong;                                        // C 2064

    block_input ();                                               // C 2066
    mask = (ANDROID_CW_OVERRIDE_REDIRECT | ANDROID_CW_BACK_PIXEL) as c_ulong;   // C 2067

    attrs.override_redirect = true;                               // C 2069
    attrs.background_pixel = FRAME_BACKGROUND_PIXEL! (f);          // C 2070
    /* C 2071-2076's `tip_window = FRAME_ANDROID_WINDOW (f) = android_create_window (…)` is a
       chained assignment, and Rust's assignment is a statement, so the port writes C's two
       stores, innermost first (the `dosfns.rs` 656-660 shape).  */
    FRAME_ANDROID_WINDOW! (f)
      = android_create_window ((*FRAME_DISPLAY_INFO! (f)).root_window,   // C 2073
			       /* x, y, width, height, value-mask,
				  attrs.  */                          // C 2074-2075
			       0, 0, 1, 1, mask as android_window_value_mask, &raw mut attrs);
    tip_window = FRAME_ANDROID_WINDOW! (f);                        // C 2071
    unblock_input ();                                             // C 2077
  }

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */   // C 2080-2082
  init_frame_faces (f);                                           // C 2083

  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),         // C 2085
			 c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
			 RES_TYPE_BOOLEAN);

  gui_figure_window_size (f, parms, false, false);                // C 2089

  (*((*f).output_data.android as *mut android_output)).parent_desc
    = (*FRAME_DISPLAY_INFO! (f)).root_window;                     // C 2091

  android_make_gc (f);                                            // C 2093

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),       // C 2095
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),       // C 2097
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),      // C 2099
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (), RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qalpha (), Qnil (),            // C 2101
			 c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),  // C 2103
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),  // C 2105
			 c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  /* Add `tooltip' frame parameter's default value. */             // C 2109
  if NILP (Fframe_parameter (frame, Qtooltip ()))                 // C 2110
    {
      AUTO_FRAME_ARG! (arg, Qtooltip (), Qt ());                   // C 2112
      Fmodify_frame_parameters (frame, arg);                       // C 2113
    }

  /* FIXME - can this be done in a similar way to normal frames?
     https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html  */   // C 2116-2117

  /* Set the `display-type' frame parameter before setting up faces. */          // C 2119
  {
    let mut disptype: Lisp_Object;                                // C 2121

    disptype = Qcolor ();                                         // C 2123

    if NILP (Fframe_parameter (frame, Qdisplay_type ()))          // C 2125
      {
	AUTO_FRAME_ARG! (arg, Qdisplay_type (), disptype);         // C 2127
	Fmodify_frame_parameters (frame, arg);                     // C 2128
      }
  }

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.  */   // C 2132-2133
  {
    let bg: Lisp_Object = Fframe_parameter (frame, Qbackground_color ());   // C 2135

    calln! (Qface_set_after_frame_default (), frame, Qnil ());     // C 2137

    if !EQ (bg, Fframe_parameter (frame, Qbackground_color ()))    // C 2139
      {
	AUTO_FRAME_ARG! (arg, Qbackground_color (), bg);           // C 2141
	Fmodify_frame_parameters (frame, arg);                     // C 2142
      }
  }

  no_split! (f, true);                                            // C 2146

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */                                // C 2148-2149
  (*(*f).terminal).reference_count += 1;                           // C 2150

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */                                      // C 2152-2154
  Vframe_list = Fcons (frame, Vframe_list);                        // C 2155
  can_set_window_size! (f, true);                                  // C 2156
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),   // C 2157
		     0, true, Qtip_frame ());

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the clearing of
     all current matrices.  Since this isn't necessary here, avoid it
     by resetting face_change to the value it had before we created
     the tip frame.  */                                           // C 2160-2164
  face_change = face_change_before;                               // C 2165

  /* Discard the unwind_protect.  */                              // C 2167
  return unbind_to (count, frame);                                // C 2168
}

/* C 2171-2296: the two tooltip helpers still inside C 1912's `#ifndef ANDROID_STUBIFY` -
   `android_hide_tip` (C 2171-2218) and `compute_tip_xy` (C 2220-2294) - and that block's own
   `#endif` at C 2296.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_hide_tip (delete: bool) -> Lisp_Object
{                                                              // C 2171-2218
  if !NILP (tip_timer)                                            // C 2174
    {
      calln! (Qcancel_timer (), tip_timer);                       // C 2176
      tip_timer = Qnil ();                                        // C 2177
    }

  if NILP (tip_frame)                                             // C 2180
      || (!delete
	  && !NILP (tip_frame)
	  && FRAME_LIVE_P (XFRAME (tip_frame))
	  && !FRAME_VISIBLE_P (XFRAME (tip_frame)))
    { return Qnil (); }                                           // C 2185
  else
    {
      let mut was_open: Lisp_Object = Qnil ();                    // C 2188

      let count: specpdl_ref = SPECPDL_INDEX ();                  // C 2190
      specbind (Qinhibit_redisplay (), Qt ());                    // C 2191
      specbind (Qinhibit_quit (), Qt ());                         // C 2192

      if !NILP (tip_frame)                                        // C 2194
	{
	  let f: *mut frame = XFRAME (tip_frame);                 // C 2196

	  if FRAME_LIVE_P (f)                                     // C 2198
	    {
	      if delete                                           // C 2200
		{
		  delete_frame (tip_frame, Qnil ());              // C 2202
		  tip_frame = Qnil ();                            // C 2203
		}
	      else
		{ android_make_frame_invisible (XFRAME (tip_frame)); }   // C 2206

	      was_open = Qt ();                                   // C 2208
	    }
	  else
	    { tip_frame = Qnil (); }                              // C 2211
	}
      else
	{ tip_frame = Qnil (); }                                  // C 2214

      return unbind_to (count, was_open);                         // C 2216
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn compute_tip_xy (f: *mut frame, parms: Lisp_Object, dx: Lisp_Object,
			  dy: Lisp_Object, width: c_int, height: c_int,
			  root_x: *mut c_int, root_y: *mut c_int)
{                                                              // C 2220-2294
  let mut left: Lisp_Object;                                      // C 2225
  let mut top: Lisp_Object;
  let mut right: Lisp_Object;
  let mut bottom: Lisp_Object;
  let mut min_x: c_int;                                           // C 2226
  let mut min_y: c_int;
  let mut max_x: c_int;
  let mut max_y: c_int = -1;
  let mut window: android_window;                                 // C 2227
  let mut mouse_frame: *mut frame;                                // C 2228

  /* Initialize these values in case there is no mouse frame.  */   // C 2230
  *root_x = 0;                                                    // C 2231
  *root_y = 0;                                                    // C 2232

  /* User-specified position?  */                                 // C 2234
  left = CDR (Fassq (Qleft (), parms));                           // C 2235
  top  = CDR (Fassq (Qtop (), parms));                            // C 2236
  right = CDR (Fassq (Qright (), parms));                         // C 2237
  bottom = CDR (Fassq (Qbottom (), parms));                       // C 2238

  /* Move the tooltip window where the mouse pointer was last seen.
     Resize and show it.  */                                      // C 2240-2241
  if (!FIXNUMP (left) && !FIXNUMP (right))                         // C 2242
      || (!FIXNUMP (top) && !FIXNUMP (bottom))                     // C 2243
    {
      /* C's `x_display_list->last_mouse_motion_frame` truth test is a pointer test here.  */
      if !(*x_display_list).last_mouse_motion_frame.is_null ()     // C 2245
	{
	  *root_x = (*x_display_list).last_mouse_motion_x;          // C 2247
	  *root_y = (*x_display_list).last_mouse_motion_y;          // C 2248
	  mouse_frame = (*x_display_list).last_mouse_motion_frame;   // C 2249
	  window = FRAME_ANDROID_WINDOW! (mouse_frame);             // C 2250

	  /* Translate the coordinates to the screen.  */           // C 2252
	  android_translate_coordinates (window, *root_x, *root_y,  // C 2253
					 root_x, root_y);
	}
    }

  min_x = 0;                                                      // C 2258
  min_y = 0;                                                      // C 2259
  max_x = android_get_screen_width ();                            // C 2260
  max_y = android_get_screen_height ();                           // C 2261

  /* Every assignment below narrows C's `EMACS_INT` (the `XFIXNUM` call) into the `int` lvalue the
     C statement stores it in, which the port spells `as c_int`.  */
  if FIXNUMP (top)                                                // C 2263
    { *root_y = XFIXNUM (top) as c_int; }                         // C 2264
  else if FIXNUMP (bottom)                                        // C 2265
    { *root_y = XFIXNUM (bottom) as c_int - height; }             // C 2266
  else if (*root_y + XFIXNUM (dy) as c_int) <= min_y              // C 2267
    { *root_y = min_y; } /* Can happen for negative dy */         // C 2268
  else if (*root_y + XFIXNUM (dy) as c_int + height) <= max_y     // C 2269
    /* It fits below the pointer */
    { *root_y += XFIXNUM (dy) as c_int; }                         // C 2271
  else if (height + XFIXNUM (dy) as c_int + min_y) <= *root_y     // C 2272
    /* It fits above the pointer.  */
    { *root_y -= height + XFIXNUM (dy) as c_int; }                // C 2274
  else
    /* Put it on the top.  */
    { *root_y = min_y; }                                          // C 2277

  if FIXNUMP (left)                                               // C 2279
    { *root_x = XFIXNUM (left) as c_int; }                        // C 2280
  else if FIXNUMP (right)                                         // C 2281
    { *root_x = XFIXNUM (right) as c_int - width; }               // C 2282
  else if (*root_x + XFIXNUM (dx) as c_int) <= min_x              // C 2283
    { *root_x = 0; } /* Can happen for negative dx */             // C 2284
  else if (*root_x + XFIXNUM (dx) as c_int + width) <= max_x      // C 2285
    /* It fits to the right of the pointer.  */
    { *root_x += XFIXNUM (dx) as c_int; }                         // C 2287
  else if (width + XFIXNUM (dx) as c_int + min_x) <= *root_x      // C 2288
    /* It fits to the left of the pointer.  */
    { *root_x -= width + XFIXNUM (dx) as c_int; }                 // C 2290
  else
    /* Put it left justified on the screen -- it ought to fit that way.  */
    { *root_x = min_x; }                                          // C 2293
}

/* C 2296: the `#endif` of C 1912's `#ifndef ANDROID_STUBIFY` block.  */

/* C 2295-2583: `Fx_show_tip` (C 2298-2583), the tooltip family's `DEFUN`.  Its body carries C
   2303's `#ifdef ANDROID_STUBIFY` pair as two cfg'd blocks in C's order, and C 2371's
   `goto start_timer` plus C 2577's label as the labelled block the `xfns.rs` 12598-12607 note
   describes (`break 'start_timer` is the port's spelling of the jump, PORTING.md 2.2).  The
   spellings are the sibling unit's, adapted to this file's two helpers (`android_hide_tip`,
   `compute_tip_xy`) and its window calls (`android_move_window`, `android_move_resize_window`,
   `android_map_raised`).  */

/* `struct redisplay_interface` - dispextern.h 3069-3196 - has no mirror at this unit's gate set
   (`include/dispextern.rs` 3794 gates its record on `dispextern_face_records`, and
   `include/termhooks.rs`'s `rif` is `*mut c_void`), while `include/frame.rs` 1912-1920's
   `flush_frame!` names the type.  The unit therefore declares the record at the use site and
   writes C's own `flush_frame` body, the `androidterm.rs` 111-143 shape (P-72 rule 4; the ten
   members C declares before `flush_display` are all pointers, so the prefix is an opaque pointer
   array and the one offset that matters is pinned).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct redisplay_interface
{
  _prefix: [*mut c_void; 9],
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  assert! (std::mem::offset_of! (redisplay_interface, flush_display) == 72);
};

/* C 2565's `flush_frame (f)`.  C's is frame.h 1914-1921's inline; the mirror's `macro_rules!`
   body hands `FRAME_RIF!`'s `*mut c_void` back to a `*mut redisplay_interface` binding, so the
   unit writes C's own body with C's own text (the `androidterm.rs` 129-143 note).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface =
    (*(*f).terminal).rif as *mut redisplay_interface;   /* C: `FRAME_RIF (f)`.  */

  if !rif.is_null () && (*rif).flush_display.is_some ()
    { ((*rif).flush_display).unwrap () (f); }
}

/* Use-site copies for the three names C 2470-2515 reaches and no mirror in this unit's graph
   carries at the top level (P-72 rule 4):
     * `window.h` 339-351's `bool_bf` bits (`pseudo_window_p` at C 2471, `must_be_updated_p` at
       C 2563) - the per-unit accessor the `haikufns.rs` 274-289 note describes, with
       `include/window.rs`'s `WINDOW_*_SHIFT` constants;
     * `frame.h` 1035's `FRAME_TOTAL_COLS`, written at C 2499 (the `haikufns.rs` 449-454 shape);
     * dispextern.h 291-292's `SET_TEXT_POS`, expanded at C 2515 (the mirror's copy sits inside a
       module; the `bidi.rs` 213-221 shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
  (must_be_updated_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_MUST_BE_UPDATED_P_SHIFT) }; };
}
/// C: frame.h 1035: `#define FRAME_TOTAL_COLS(f) ((f)->total_cols)` - written at C 2499.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TOTAL_COLS {
  ($f:expr, $v:expr) => { (*($f)).total_cols = $v };
}
/// C: dispextern.h 291-292: `#define SET_TEXT_POS(pos, charpos, bytepos) ((pos).charpos = (charpos), (pos).bytepos = (bytepos))`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: window.h 1139-1140: `void set_window_buffer (Lisp_Object window, Lisp_Object buffer,`
  /// (window.h 1140 continues `bool run_hooks_p, bool keep_margins_p);`)
  pub fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object, run_hooks_p: bool,
                            keep_margins_p: bool);
  /// C: dispextern.h 3573: `int try_window (Lisp_Object, struct text_pos, int);`
  pub fn try_window (window: Lisp_Object, pos: text_pos, flags: c_int) -> c_int;
  /// C: dispextern.h 3918: `void update_single_window (struct window *);`
  pub fn update_single_window (w: *mut window);
}

/* C 2298-2583.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-show-tip", Fx_show_tip, Sx_show_tip, 1, 6, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(string: Lisp_Object, frame: Lisp_Object, parms: Lisp_Object,
	 timeout: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object),
	{
  /* C 2303-2305: `#ifdef ANDROID_STUBIFY`.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    error (c"Android cross-compilation stub called!".as_ptr ());   // C 2304
    return Qnil ();                                                // C 2305
  }

  /* C 2306-2582: the `#else` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut f: *mut frame;                                          // C 2307
  let mut tip_f: *mut frame;
  let mut w: *mut window;                                         // C 2308
  /* C 2309 declares `root_x`/`root_y` with no initialiser and C 2365/2545 pass their addresses,
     so the port zero-initialises them (PORTING.md 1.5).  */
  let mut root_x: c_int = 0;
  let mut root_y: c_int = 0;
  let mut old_buffer: *mut buffer;                                // C 2310
  let mut pos: text_pos = std::mem::zeroed ();                    // C 2311
  let mut width: c_int = 0;                                       // C 2312
  let mut height: c_int = 0;
  let old_windows_or_buffers_changed: c_int = windows_or_buffers_changed;  // C 2313
  let count: specpdl_ref = SPECPDL_INDEX ();                      // C 2314
  let mut window: Lisp_Object;                                    // C 2315
  let mut size: Lisp_Object;
  let mut tip_buf: Lisp_Object;
  let mut displayed: bool;                                        // C 2316
  /* C 2317-2319: `#ifdef ENABLE_CHECKING` around the two row locals.  */
  #[cfg (feature = "enable_checking")]
  let mut row: *mut glyph_row = std::ptr::null_mut ();
  #[cfg (feature = "enable_checking")]
  let mut end: *mut glyph_row;
  AUTO_STRING! (tip, c" *tip*".as_ptr ());                        // C 2320

  /* C's `goto start_timer` (C 2371) and its label (C 2577) become this labelled block.  The six
     parameters are re-bound as `mut` because C reassigns them and the `DEFUN!` macro's
     `$arg:ident` cannot say `mut` (the `x-show-tip` spelling in `xfns.rs`; the macro's own note
     in `docs/ports/macro-body-notes.tsv`).  */
  let mut string = string;
  let mut frame = frame;
  let mut parms = parms;
  let mut timeout = timeout;
  let mut dx = dx;
  let mut dy = dy;

  'start_timer: {
  specbind (Qinhibit_redisplay (), Qt ());                        // C 2322

  CHECK_STRING (string);                                          // C 2324
  if SCHARS (string) == 0                                         // C 2325
    { string = make_unibyte_string (c" ".as_ptr (), 1); }          // C 2326

  if NILP (frame)                                                 // C 2328
    { frame = selected_frame; }                                   // C 2329
  f = decode_window_system_frame (frame);                         // C 2330

  if NILP (timeout)                                               // C 2332
    { timeout = Vx_show_tooltip_timeout; }                        // C 2333
  CHECK_FIXNAT (timeout);                                         // C 2334

  if NILP (dx)                                                    // C 2336
    { dx = make_fixnum (5); }                                     // C 2337
  else
    { CHECK_FIXNUM (dx); }                                        // C 2339

  if NILP (dy)                                                    // C 2341
    { dy = make_fixnum (-10); }                                   // C 2342
  else
    { CHECK_FIXNUM (dy); }                                        // C 2344

  tip_dx = dx;                                                    // C 2346
  tip_dy = dy;                                                    // C 2347

  if !NILP (tip_frame) && FRAME_LIVE_P (XFRAME (tip_frame))        // C 2349
    {
      if FRAME_VISIBLE_P (XFRAME (tip_frame))                      // C 2351
	  && !NILP (Fequal_including_properties (tip_last_string,   // C 2352
						 string))
	  && !NILP (Fequal (tip_last_parms, parms))                 // C 2354
	{
	  /* Only DX and DY have changed.  */                      // C 2356
	  tip_f = XFRAME (tip_frame);                              // C 2357
	  if !NILP (tip_timer)                                     // C 2358
	    {
	      calln! (Qcancel_timer (), tip_timer);                // C 2360
	      tip_timer = Qnil ();                                 // C 2361
	    }

	  block_input ();                                          // C 2364
	  compute_tip_xy (tip_f, parms, dx, dy, FRAME_PIXEL_WIDTH (tip_f),   // C 2365
			  FRAME_PIXEL_HEIGHT (tip_f), &raw mut root_x, &raw mut root_y);
	  android_move_window (FRAME_ANDROID_WINDOW! (tip_f),       // C 2367
			       root_x, root_y);
	  unblock_input ();                                        // C 2369

	  break 'start_timer;                                      // C 2371 (C's `goto start_timer`)
	}
      else if tooltip_reuse_hidden_frame && BASE_EQ (frame, tip_last_frame)   // C 2373
	{
	  let mut delete: bool = false;                            // C 2375
	  let mut tail: Lisp_Object;                               // C 2376
	  let mut elt: Lisp_Object;
	  let mut parm: Lisp_Object;
	  let mut last: Lisp_Object;

	  /* Check if every parameter in PARMS has the same value in
	     tip_last_parms.  This may destruct tip_last_parms which,
	     however, will be recreated below.  */                 // C 2378-2380
	  tail = parms;                                            // C 2381
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);                                   // C 2383
	      parm = CAR (elt);                                    // C 2384
	      /* The left, top, right and bottom parameters are handled
		 by compute_tip_xy so they can be ignored here.  */  // C 2385-2386
	      if !EQ (parm, Qleft ()) && !EQ (parm, Qtop ())        // C 2387
		  && !EQ (parm, Qright ()) && !EQ (parm, Qbottom ())  // C 2388
		{
		  last = Fassq (parm, tip_last_parms);              // C 2390
		  if NILP (Fequal (CDR (elt), CDR (last)))          // C 2391
		    {
		      /* We lost, delete the old tooltip.  */       // C 2393
		      delete = true;                                // C 2394
		      break;
		    }
		  else
		    { tip_last_parms                                 // C 2398-2399
			= calln! (Qassq_delete_all (), parm, tip_last_parms); }
		}
	      else
		{ tip_last_parms                                     // C 2402-2403
		    = calln! (Qassq_delete_all (), parm, tip_last_parms); }
	      tail = XCDR (tail);                                   // (C 2381's increment)
	    }

	  /* Now check if every parameter in what is left of
	     tip_last_parms with a non-nil value has an association in
	     PARMS.  */                                             // C 2406-2408
	  tail = tip_last_parms;                                    // C 2409
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);                                    // C 2411
	      parm = CAR (elt);                                     // C 2412
	      if (!EQ (parm, Qleft ()) && !EQ (parm, Qtop ()) && !EQ (parm, Qright ())   // C 2413
		  && !EQ (parm, Qbottom ()) && !NILP (CDR (elt)))    // C 2414
		{
		  /* We lost, delete the old tooltip.  */            // C 2416
		  delete = true;                                     // C 2417
		  break;
		}
	      tail = XCDR (tail);                                    // (C 2409's increment)
	    }

	  android_hide_tip (delete);                                // C 2422
	}
      else
	{ android_hide_tip (true); }                                // C 2425
    }
  else
    { android_hide_tip (true); }                                // C 2428

  tip_last_frame = frame;                                       // C 2430
  tip_last_string = string;                                     // C 2431
  tip_last_parms = parms;                                       // C 2432

  if NILP (tip_frame) || !FRAME_LIVE_P (XFRAME (tip_frame))       // C 2434
    {
      /* Add default values to frame parameters.  */              // C 2436
      if NILP (Fassq (Qname (), parms))                           // C 2437
	{ parms = Fcons (Fcons (Qname (), build_string (c"tooltip".as_ptr ())), parms); }   // C 2438
      if NILP (Fassq (Qinternal_border_width (), parms))           // C 2439
	{ parms = Fcons (Fcons (Qinternal_border_width (), make_fixnum (3)),   // C 2440-2441
			 parms); }
      if NILP (Fassq (Qborder_width (), parms))                    // C 2442
	{ parms = Fcons (Fcons (Qborder_width (), make_fixnum (1)), parms); }   // C 2443
      if NILP (Fassq (Qborder_color (), parms))                    // C 2444
	{ parms = Fcons (Fcons (Qborder_color (), build_string (c"lightyellow".as_ptr ())),   // C 2445-2446
			 parms); }
      if NILP (Fassq (Qbackground_color (), parms))                // C 2447
	{ parms = Fcons (Fcons (Qbackground_color (),               // C 2448-2450
			      build_string (c"lightyellow".as_ptr ())),
			 parms); }

      /* Create a frame for the tooltip, and record it in the global
	 variable tip_frame.  */                                  // C 2452-2453
      /* C 2454's `NILP (tip_frame = android_create_tip_frame (…))` assigns inside the condition;
	 Rust's assignment is a statement, so the port keeps C's call as its own statement and the
	 test on the next line (the `dosfns.rs` 656-660 class).  */
      tip_frame = android_create_tip_frame (FRAME_DISPLAY_INFO! (f),   // C 2454-2455
					    parms);
      if NILP (tip_frame)                                         // C 2454
	/* Creating the tip frame failed.  */                      // C 2456
	{ return unbind_to (count, Qnil ()); }                     // C 2457
    }
  else
    { tip_window = FRAME_ANDROID_WINDOW! (XFRAME (tip_frame)); }   // C 2460

  tip_f = XFRAME (tip_frame);                                     // C 2462
  window = FRAME_ROOT_WINDOW (tip_f);                             // C 2463
  tip_buf = Fget_buffer_create (tip, Qnil ());                    // C 2464
  /* We will mark the tip window a "pseudo-window" below, and such
     windows cannot have display margins.  */                     // C 2465-2466
  bset_left_margin_cols (XBUFFER (tip_buf), make_fixnum (0));      // C 2467
  bset_right_margin_cols (XBUFFER (tip_buf), make_fixnum (0));     // C 2468
  set_window_buffer (window, tip_buf, false, false);               // C 2469
  w = XWINDOW (window);                                            // C 2470
  window_bits! (pseudo_window_p, w, true);                         // C 2471
  /* Try to avoid that `other-window' select us (Bug#47207).  */    // C 2472
  Fset_window_parameter (window, Qno_other_window (), Qt ());       // C 2473

  /* Set up the frame's root window.  Note: The following code does not
     try to size the window or its frame correctly.  Its only purpose is
     to make the subsequent text size calculations work.  The right
     sizes should get installed when the toolkit gets back to us.  */   // C 2475-2478
  (*w).left_col = 0;                                               // C 2479
  (*w).top_line = 0;                                               // C 2480
  (*w).pixel_left = 0;                                             // C 2481
  (*w).pixel_top = 0;                                              // C 2482

  if CONSP (Vx_max_tooltip_size)                                   // C 2484
      && RANGED_FIXNUMP (1, XCAR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)   // C 2485
      && RANGED_FIXNUMP (1, XCDR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)   // C 2486
    {
      (*w).total_cols = XFIXNAT (XCAR (Vx_max_tooltip_size)) as c_int;    // C 2488
      (*w).total_lines = XFIXNAT (XCDR (Vx_max_tooltip_size)) as c_int;   // C 2489
    }
  else
    {
      (*w).total_cols = 80;                                        // C 2493
      (*w).total_lines = 40;                                       // C 2494
    }

  (*w).pixel_width = (*w).total_cols * FRAME_COLUMN_WIDTH (tip_f);          // C 2497
  (*w).pixel_height = (*w).total_lines * FRAME_LINE_HEIGHT (tip_f);         // C 2498
  FRAME_TOTAL_COLS! (tip_f, (*w).total_cols);                              // C 2499
  adjust_frame_glyphs (tip_f);                                             // C 2500

  /* Insert STRING into root window's buffer and fit the frame to the
     buffer.  */                                                    // C 2502-2503
  let count_1: specpdl_ref = SPECPDL_INDEX ();                     // C 2504
  old_buffer = current_buffer! ();                                 // C 2505
  set_buffer_internal_1 (XBUFFER ((*w).contents));                 // C 2506
  bset_truncate_lines (current_buffer! (), Qnil ());                // C 2507
  specbind (Qinhibit_read_only (), Qt ());                          // C 2508
  specbind (Qinhibit_modification_hooks (), Qt ());                 // C 2509
  specbind (Qinhibit_point_motion_hooks (), Qt ());                 // C 2510
  Ferase_buffer ();                                                // C 2511
  Finsert (1, &raw mut string);                                    // C 2512
  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);    // C 2513
  clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);    // C 2514
  SET_TEXT_POS! (pos, BEGV! (), BEGV_BYTE! ());                     // C 2515
  displayed = try_window (window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE) != 0;   // C 2516

  if !displayed && NILP (Vx_max_tooltip_size)                       // C 2518
    {
      /* C 2520-2533: `#ifdef ENABLE_CHECKING`.  */
      #[cfg (feature = "enable_checking")]
      {
	row = (*((*w).desired_matrix as *mut glyph_matrix)).rows;   // C 2521
	end = (*((*w).desired_matrix as *mut glyph_matrix)).rows    // C 2522
	  .offset ((*((*w).desired_matrix as *mut glyph_matrix)).nrows as isize);

	while row < end                                            // C 2524
	  {
	    if !glyph_row_bits! (displays_text_p, row)              // C 2526
		|| glyph_row_bits! (ends_at_zv_p, row)              // C 2527
	      { break; }                                            // C 2528
	    row = row.offset (1);                                   // C 2529
	  }

	eassert! (row < end && glyph_row_bits! (ends_at_zv_p, row));   // C 2532
      }
    }

  /* Calculate size of tooltip window.  */                          // C 2536
  size = Fwindow_text_pixel_size (window, Qnil (), Qnil (), Qnil (),   // C 2537-2539
				  make_fixnum ((*w).pixel_height as EMACS_INT), Qnil (),
				  Qnil ());
  /* Add the frame's internal border to calculated size.  */        // C 2540
  width = (XFIXNUM (CAR (size)) as c_int) + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);    // C 2541
  height = (XFIXNUM (CDR (size)) as c_int) + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);   // C 2542

  /* Calculate position of tooltip frame.  */                       // C 2544
  compute_tip_xy (tip_f, parms, dx, dy, width, height, &raw mut root_x,   // C 2545
		  &raw mut root_y);

  /* Show tooltip frame.  */                                        // C 2547
  block_input ();                                                   // C 2548
  /* C's third and fourth arguments are the `int` locals `width`/`height` and the call's
     parameters are `unsigned int`, so the port spells C's implicit conversion.  */
  android_move_resize_window (FRAME_ANDROID_WINDOW! (tip_f),        // C 2549
			      root_x, root_y, width as c_uint,
			      height as c_uint);
  android_map_raised (FRAME_ANDROID_WINDOW! (tip_f));               // C 2552
  unblock_input ();                                                 // C 2553

  /* Garbage the tip frame too.  */                                 // C 2555
  SET_FRAME_GARBAGED! (tip_f);                                      // C 2556

  /* Block input around `update_single_window' and `flush_frame', lest a
     ConfigureNotify and Expose event arrive during the update, and set
     flags, e.g. garbaged_p, that are cleared once the update completes,
     leaving the requested exposure or configuration outstanding.  */   // C 2558-2561
  block_input ();                                                   // C 2562
  window_bits! (must_be_updated_p, w, true);                        // C 2563
  update_single_window (w);                                         // C 2564
  flush_frame (tip_f);                                              // C 2565
  unblock_input ();                                                 // C 2566

  set_buffer_internal_1 (old_buffer);                               // C 2568
  unbind_to (count_1, Qnil ());                                     // C 2569
  windows_or_buffers_changed = old_windows_or_buffers_changed;      // C 2570

  /* MapNotify events are not sent on Android, so make the frame
     visible.  */                                                   // C 2572-2573

  SET_FRAME_VISIBLE! (tip_f, true);                                 // C 2575

  }   /* 'start_timer: */

  /* Let the tip disappear after timeout seconds.  */               // C 2578
  tip_timer = calln! (Qrun_at_time (), timeout, Qnil (), Qx_hide_tip ());   // C 2579

  return unbind_to (count, Qnil ());                                // C 2581
  }
});

/* C 2584-2662: four `DEFUN`s - `Fx_hide_tip` (C 2585-2600), `Fandroid_detect_mouse`
   (C 2602-2619), `Fandroid_detect_keyboard` (C 2621-2638) and
   `Fandroid_toggle_on_screen_keyboard` (C 2640-2662).  The first carries C's pair in C's order
   (the stub arm first), the next two carry C's `#ifndef` first, and the last one's real arm is
   followed by C's `return Qnil;` outside the `#endif` (C 2661).  */

/* C 2585-2600.  */
#[cfg (feature = "have_android")]
DEFUN! ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(),
	{
  /* C 2589-2596: `#ifdef ANDROID_STUBIFY`.  C 2590-2592's comment explains why the stub stays
     silent (its caller is `pre-command-hook`, so signalling would stop every test) and C 2593-2595
     wraps the `error` call in C's own `#if 0`, which no configuration compiles, so it is recorded
     in `docs/ports/androidfns.md` and not transcribed (the `xfns.rs` 958 `#if false` precedent).  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
    return Qnil ();                                                // C 2596
  }

  /* C 2597-2598: the `#else /* !ANDROID_STUBIFY */` arm.  */
  #[cfg (feature = "not_android_stubify")]
  {
  return android_hide_tip (!tooltip_reuse_hidden_frame);            // C 2598
  }
});

/* C 2602-2619.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-detect-mouse", Fandroid_detect_mouse, Sandroid_detect_mouse,
	0, 0, 0,
	doc: "Figure out whether or not there is a mouse.\nReturn non-nil if a mouse is connected to this computer, and nil if\nthere is no mouse.  ",
	(),
	{
  /* C 2609-2615: the real arm (`#ifndef ANDROID_STUBIFY`).  */
  #[cfg (feature = "not_android_stubify")]
  {
  /* If no display connection is present, just return nil.  */      // C 2610

  if !android_init_gui                                             // C 2612
    { return Qnil (); }                                            // C 2613

  return if android_detect_mouse () { Qt () } else { Qnil () };     // C 2615
  }

  /* C 2616-2617: the `#else` arm.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
  return Qnil ();                                                  // C 2617
  }
});

/* C 2621-2638.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-detect-keyboard", Fandroid_detect_keyboard, Sandroid_detect_keyboard,
	0, 0, 0,
	doc: "Return whether a keyboard is connected.\nReturn non-nil if a key is connected to this computer, or nil\nif there is no keyboard.  ",
	(),
	{
  /* C 2628-2634: the real arm (`#ifndef ANDROID_STUBIFY`).  */
  #[cfg (feature = "not_android_stubify")]
  {
  /* If no display connection is present, just return nil.  */      // C 2629

  if !android_init_gui                                             // C 2631
    { return Qnil (); }                                            // C 2632

  return if android_detect_keyboard () { Qt () } else { Qnil () };  // C 2634
  }

  /* C 2635-2636: the `#else /* ANDROID_STUBIFY */` arm.  */
  #[cfg (not (feature = "not_android_stubify"))]
  {
  return Qt ();                                                    // C 2636
  }
});

/* C 2640-2662.  */
#[cfg (feature = "have_android")]
DEFUN! ("android-toggle-on-screen-keyboard", Fandroid_toggle_on_screen_keyboard,
	 Sandroid_toggle_on_screen_keyboard,
	 2, 2, 0,
	 doc: "Display or hide the on-screen keyboard.\nIf HIDE is non-nil, hide the on screen keyboard if it is currently\nbeing displayed.  Else, request that the system display it on behalf\nof FRAME.  This request may be rejected if FRAME does not have the\ninput focus.  ",
	 (frame: Lisp_Object, hide: Lisp_Object),
	 {
  /* C 2650-2659: the real arm (`#ifndef ANDROID_STUBIFY`); C's `#endif` has no `#else`, so the
     stub side of this pair is C 2661's `return Qnil;` outside it.  */
  #[cfg (feature = "not_android_stubify")]
  {
  let mut f: *mut frame;                                           // C 2651

  f = decode_window_system_frame (frame);                          // C 2653

  block_input ();                                                  // C 2655
  android_toggle_on_screen_keyboard (FRAME_ANDROID_WINDOW! (f),    // C 2656
				     NILP (hide));
  unblock_input ();                                                // C 2658
  }

  return Qnil ();                                                  // C 2661
});

/* C 2663-2832: C 2666's `#ifndef ANDROID_STUBIFY` and the first seven frame-parameter setters -
   `android_set_background_color` (C 2668-2693), `android_set_border_color` (C 2695-2703),
   `android_set_cursor_color` (C 2705-2748), `android_set_cursor_type` (C 2750-2755),
   `android_set_foreground_color` (C 2757-2789), `android_set_child_frame_border_width`
   (C 2791-2814) and `android_set_internal_border_width` (C 2816-2832).  The `#ifndef` block runs
   to C 3357; the next setter is C 2834's `android_set_menu_bar_lines`.  */

/* Use-site declarations (P-72 rule 4) for the five names these setters call whose mirrors do not
   carry them: each is another unit's definition, so the unit declares it under C's own name.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3794: `void update_face_from_frame_parameter (struct frame *, Lisp_Object,`
  /// (dispextern.h 3795 continues `Lisp_Object);`)
  pub fn update_face_from_frame_parameter (f: *mut frame, prop: Lisp_Object, new_value: Lisp_Object);
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, invisible: bool);
  /// C: lisp.h 4409: `void set_frame_cursor_types (struct frame *, Lisp_Object);`
  pub fn set_frame_cursor_types (f: *mut frame, cursor_type: Lisp_Object);
  /// C: lisp.h 607: `extern int check_int_nonnegative (Lisp_Object);`
  pub fn check_int_nonnegative (x: Lisp_Object) -> c_int;
  /// C: dispextern.h 3896: `extern void redraw_frame (struct frame *);`
  pub fn redraw_frame (f: *mut frame);
}

/* C 2668-2693.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_background_color (f: *mut frame, arg: Lisp_Object,
					oldval: Lisp_Object)
{
  let mut x: *mut android_output;                                 // C 2672
  let mut bg: c_ulong;                                            // C 2673

  x = (*f).output_data.android as *mut android_output;            // C 2675
  bg = android_decode_color (f, arg, WHITE_PIX_DEFAULT! (f));      // C 2676
  FRAME_BACKGROUND_PIXEL! (f) = bg;                               // C 2677

  if FRAME_ANDROID_WINDOW! (f) != 0                               // C 2679
    {
      block_input ();                                             // C 2681
      android_set_background ((*x).normal_gc, bg);                // C 2682
      android_set_foreground ((*x).reverse_gc, bg);               // C 2683
      android_set_window_background (FRAME_ANDROID_WINDOW! (f), bg);   // C 2684
      android_set_foreground ((*x).cursor_gc, bg);                // C 2685
      unblock_input ();                                           // C 2686

      update_face_from_frame_parameter (f, Qbackground_color (), arg);   // C 2688

      if FRAME_VISIBLE_P (f)                                      // C 2690
        { redraw_frame (f); }                                     // C 2691
    }
}

/* C 2695-2703.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_border_color (f: *mut frame, arg: Lisp_Object,
				    oldval: Lisp_Object)
{
  /* Left unimplemented because Android has no window borders.  */   // C 2699
  CHECK_STRING (arg);                                             // C 2700
  android_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));           // C 2701
  update_face_from_frame_parameter (f, Qborder_color (), arg);     // C 2702
}

/* C 2705-2748.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_cursor_color (f: *mut frame, arg: Lisp_Object,
				    oldval: Lisp_Object)
{
  let mut fore_pixel: c_ulong;                                    // C 2709
  let mut pixel: c_ulong;
  let mut x: *mut android_output;                                 // C 2710

  x = (*f).output_data.android as *mut android_output;            // C 2712

  if !NILP (Vx_cursor_fore_pixel)                                 // C 2714
    { fore_pixel = android_decode_color (f, Vx_cursor_fore_pixel,   // C 2715-2716
					 WHITE_PIX_DEFAULT! (f)); }
  else
    { fore_pixel = FRAME_BACKGROUND_PIXEL! (f); }                  // C 2718

  pixel = android_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));    // C 2720

  /* Make sure that the cursor color differs from the background color.  */   // C 2722
  if pixel == FRAME_BACKGROUND_PIXEL! (f)                          // C 2723
    {
      pixel = FRAME_FOREGROUND_PIXEL! (f);                         // C 2725
      if pixel == fore_pixel                                      // C 2726
	{ fore_pixel = FRAME_BACKGROUND_PIXEL! (f); }              // C 2727
    }

  (*x).cursor_foreground_pixel = fore_pixel;                      // C 2730
  (*x).cursor_pixel = pixel;                                      // C 2731

  if FRAME_ANDROID_WINDOW! (f) != 0                               // C 2733
    {
      block_input ();                                             // C 2735
      android_set_background ((*x).cursor_gc, (*x).cursor_pixel);   // C 2736
      android_set_foreground ((*x).cursor_gc, fore_pixel);         // C 2737
      unblock_input ();                                           // C 2738

      if FRAME_VISIBLE_P (f)                                      // C 2740
	{
	  gui_update_cursor (f, false);                            // C 2742
	  gui_update_cursor (f, true);                             // C 2743
	}
    }

  update_face_from_frame_parameter (f, Qcursor_color (), arg);     // C 2747
}

/* C 2750-2755.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_cursor_type (f: *mut frame, arg: Lisp_Object,
				   oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);                                // C 2754
}

/* C 2757-2789.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_foreground_color (f: *mut frame, arg: Lisp_Object,
					oldval: Lisp_Object)
{
  let mut x: *mut android_output;                                 // C 2761
  let mut fg: c_ulong;                                            // C 2762
  let mut old_fg: c_ulong;

  x = (*f).output_data.android as *mut android_output;            // C 2764

  fg = android_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));       // C 2766
  old_fg = FRAME_FOREGROUND_PIXEL! (f);                            // C 2767
  FRAME_FOREGROUND_PIXEL! (f) = fg;                                // C 2768

  if FRAME_ANDROID_WINDOW! (f) != 0                                // C 2770
    {
      block_input ();                                              // C 2772
      android_set_foreground ((*x).normal_gc, fg);                 // C 2773
      android_set_background ((*x).reverse_gc, fg);                // C 2774

      if (*x).cursor_pixel == old_fg                               // C 2776
	{
	  (*x).cursor_pixel = fg;                                   // C 2778
	  android_set_background ((*x).cursor_gc, (*x).cursor_pixel);   // C 2779
	}

      unblock_input ();                                            // C 2782

      update_face_from_frame_parameter (f, Qforeground_color (), arg);   // C 2784

      if FRAME_VISIBLE_P (f)                                       // C 2786
        { redraw_frame (f); }                                      // C 2787
    }
}

/* C 2791-2814.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object,
						oldval: Lisp_Object)
{
  let mut border: c_int;                                          // C 2795

  if NILP (arg)                                                   // C 2797
    { border = -1; }                                              // C 2798
  else if RANGED_FIXNUMP (0, arg, INT_MAX as EMACS_INT)           // C 2799
    { border = XFIXNAT (arg) as c_int; }                          // C 2800
  else
    { signal_error (c"Invalid child frame border width".as_ptr (), arg); }   // C 2802

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH (f)                 // C 2804
    {
      (*f).child_frame_border_width = border;                     // C 2806

      if FRAME_ANDROID_WINDOW! (f) != 0                           // C 2808
	{
	  adjust_frame_size (f, -1, -1, 3, false, Qchild_frame_border_width ());   // C 2810
	  android_clear_under_internal_border (f);                // C 2811
	}
    }
}

/* C 2816-2832.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_internal_border_width (f: *mut frame, arg: Lisp_Object,
					     oldval: Lisp_Object)
{
  let border: c_int = check_int_nonnegative (arg);                 // C 2820

  if border != FRAME_INTERNAL_BORDER_WIDTH (f)                     // C 2822
    {
      (*f).internal_border_width = border;                         // C 2824

      if FRAME_ANDROID_WINDOW! (f) != 0                            // C 2826
	{
	  adjust_frame_size (f, -1, -1, 3, false, Qinternal_border_width ());   // C 2828
	  android_clear_under_internal_border (f);                 // C 2829
	}
    }
}
