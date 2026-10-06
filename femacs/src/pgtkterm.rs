// femacs: port of src/pgtkterm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` - except the ones whose **address** C takes, which
// PORTING.md 1.4's "one ABI per function-pointer position" makes `unsafe extern "C" fn`: the
// fifteen `pgtk_redisplay_interface` slots (C 3756-3785, Deviation 47) and the thirty-nine
// `terminal` hook slots the thirty-eighth slice assigns in `pgtk_create_terminal` (C 4879-4919,
// Deviation 60).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `pgtkterm.o`
// sits in `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac`
// 3875-3878 - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
// `if test "$window_system" = "pgtk"`), and `docs/baseline/frozen-build-sources.txt` lists no
// `pgtk.o` for this target, so `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the
// ledger `docs/ports/pgtkterm.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.  This batch is P-86 ④'s first slice:
// **C 1-1078**, which ends at the `}` of `pgtk_set_no_accept_focus` (C 1072-1078);
// **the next live definition is `pgtk_set_z_group` (C 1080-)** - the first item of the next lane's
// slice (C 1079 is its `void` line).
//
//   * the **file gate** is `feature = "have_pgtk"` (`HAVE_PGTK`, `configure.ac` 3176's
//     `AC_DEFINE`, the macro the `window_system = pgtk` build defines), on every item *and on the
//     include graph* (P-86 ⑧ - an `include!` outside the gate breaks the frozen build at once,
//     which is why the reading in Evidence ① is a 336-byte empty object);
//   * the slice's own conditions are C 24's `#if 10 <= __GNUC__`, C 68's
//     `#ifdef GDK_WINDOWING_WAYLAND` and C 457's `#ifdef HAVE_XWIDGETS`.
//     `tools/check-cond-eval.py src/pgtkterm.c` reads
//     `31 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file **is** on
//     `docs/baseline/config/cond-eval-verified.txt` 346), so all three are decided: the pragma
//     block is dead text (clang's `__GNUC__` is 4 - carried as C's own text, the `window.rs` 14 /
//     `timefns.rs` 79 reading), and the two `#ifdef`s are false in the frozen configuration and
//     become the features `gdk_windowing_wayland` and `have_xwidgets`, **both arms carried**;
//   * the include graph reaches **five mirrors that are placeholders** - `include/pgtkterm.rs`
//     (`pgtk_display_info`, `pgtk_output`, `FRAME_X_OUTPUT`, `FRAME_FONTSET`, `FRAME_FONT`,
//     `FRAME_BASELINE_OFFSET`, `FRAME_DISPLAY_INFO`, `FRAME_GTK_WIDGET`, `FRAME_GTK_OUTER_WIDGET`,
//     `FRAME_WIDGET`), `include/gtkutil.rs` (the `xg_*` entry points), `include/xwidget.rs`
//     (`kill_frame_xwidget_views`), `include/emacsgtkfixed.rs` and `include/pgtkgui.rs`'s GTK
//     pointees.  Those mirrors keep their bodies for a PGTK target (their own ledgers say so -
//     `docs/ports/include-pgtkterm.md` "Why the pgtk records are still not here"), so this unit
//     spells the names it uses but does not define them: they are **named mirror gaps** in
//     `docs/ports/pgtkterm.md`, exactly as `src/image.rs` 531-534 keeps `pgtk_bitmap_record` as
//     `pgtkterm.h`'s.  The *functions* of those mirrors are declared at the use site (P-72 rule 4,
//     the `src/image.rs` 181-201 shape) and the ledger owes them the move.
//
// **The toolkit layer is declared here (P-88 ①), and this batch is `text-only` (P-86 ③⑵).**
// `<cairo.h>`, `<gtk/gtk.h>` (reached through `"gtkutil.h"`, C 48) and `<gdk/gdkwayland.h>` (C 69)
// are third-party headers with no file in the reference tree, so their declarations stand in this
// unit rather than in a new file under `femacs/src/include/` - and the layering is read, not
// invented: **GLib/GObject and cairo are on this machine** (`/opt/homebrew/include/glib-2.0` GLib
// 2.90.0, `/opt/homebrew/include/cairo` cairo 1.18.4) so every GLib and cairo declaration below
// quotes the header and line it was read from, and `GList`'s layout is a **clang reading**
// (Evidence ②).  **GTK and GDK are not on this machine** (`ls -d
// /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` -> both absent; `pkg-config` is not
// installed; no `gtk/gtk.h` or `gdk/gdk.h` anywhere under `/opt/homebrew`, `/usr/local` or
// `/Library`), so those records and entry points cite **the header alone, no line** (the
// `src/xfns.rs` 338-341 reading of the same absence) and their layout assertions are owed to a
// GTK toolchain.  `docs/ports/pgtkterm.md` names every one of them and the missing toolchain.
//
// `docs/ports/pgtkterm.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no GTK
// tree to compile the arm against and no PGTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`

/* C 23-26: the GCC-bug-102671 comment, then `#if 10 <= __GNUC__` / `# pragma GCC diagnostic
   ignored "-Wanalyzer-null-dereference"` / `#endif`.  A pragma has no Rust spelling and clang's
   `__GNUC__` is 4, so the guard is false on the frozen target; C's text is kept and no arm is
   written (the `window.rs` 14, `timefns.rs` 79 and `ccl.rs` 913 readings).  */

/* C 28's `<cairo.h>` and C 48's `"gtkutil.h"` chain (its 26 `<gtk/gtk.h>`, and under `HAVE_PGTK`
   its 29 `"pgtkterm.h"`): the third-party surface now lives in its single mirror
   `include/gtk.rs` (P-90 ①/②/③; the P-88 ⑤ move-back took the declarations this unit carried out
   of it) and the pgtk records/accessors in `include/pgtkterm.rs`.  This unit reaches the surface
   here; the *records'* mirror edge is not added yet - this unit's own `FRAME_*` accessors would
   then land twice in one TU (see `## Owed` in `docs/ports/pgtkterm.md`).  */
#[cfg (feature = "have_pgtk")]
include!("include/gtk.rs");                    // C 28: `#include <cairo.h>` / C 48's chain

/* C 31-42: `<pthread.h>`, `<dlfcn.h>` and the four gnulib headers.  `<pthread.h>` and `<dlfcn.h>`
   are third-party headers with no file in the reference tree either; this slice names nothing from
   them (`gtk_main_iteration` and the GLib main loop are the threading the file uses), so the two
   lines stand here and no declaration follows - `docs/ports/pgtkterm.md` records them.  */
// C 31: `#include <pthread.h>`
// C 42: `#include <dlfcn.h>`

/* C 29-42's gnulib headers and C 44-66's Emacs headers.  Every `include!` carries the file gate
   (P-86 ⑧); the header takes the reference's own directory (PORTING.md 0.1), so the five `lib/`
   headers come from `../lib/include/`.  C's own order is kept.  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/fcntl.in.rs");        // C 29: `#include <fcntl.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/math.rs");            // C 30: `#include <math.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/sys_types.in.rs");    // C 32: `#include <sys/types.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/time.in.rs");         // C 33: `#include <time.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/signal.rs");          // C 34: `#include <signal.h>` (raise)
#[cfg (feature = "have_pgtk")]
include!("../lib/include/unistd.in.rs");       // C 35: `#include <unistd.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/errno.rs");           // C 36: `#include <errno.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/c-ctype.rs");         // C 38: `#include <c-ctype.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/c-strcase.rs");       // C 39: `#include <c-strcase.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/ftoastr.rs");         // C 40: `#include <ftoastr.h>`
#[cfg (feature = "have_pgtk")]
include!("include/lisp.rs");                   // C 44: `#include "lisp.h"`
/* lisp.h 33's `#include <limits.h>`, which C reaches through `"lisp.h"` and which brings in
   `INT_MAX` (C 5151-5163's five masks): the lib mirror C's chain names (the `src/pgtkfns.rs` 89
   reading of the same line).  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>` (INT_MAX)
#[cfg (feature = "have_pgtk")]
include!("include/blockinput.rs");             // C 45: `#include "blockinput.h"`
#[cfg (feature = "have_pgtk")]
include!("include/frame.rs");                  // C 46: `#include "frame.h"`
#[cfg (feature = "have_pgtk")]
include!("include/sysselect.rs");              // C 47: `#include "sysselect.h"`
#[cfg (feature = "have_pgtk")]
include!("include/gtkutil.rs");                // C 48: `#include "gtkutil.h"` (the GTK chain)
#[cfg (feature = "have_pgtk")]
include!("include/systime.rs");                // C 49: `#include "systime.h"` (Time)
#[cfg (feature = "have_pgtk")]
include!("include/character.rs");              // C 50: `#include "character.h"`
#[cfg (feature = "have_pgtk")]
include!("include/xwidget.rs");                // C 51: `#include "xwidget.h"`
#[cfg (feature = "have_pgtk")]
include!("include/fontset.rs");                // C 52: `#include "fontset.h"`
#[cfg (feature = "have_pgtk")]
include!("include/composite.rs");              // C 53: `#include "composite.h"`
#[cfg (feature = "have_pgtk")]
include!("include/ccl.rs");                    // C 54: `#include "ccl.h"`
#[cfg (feature = "have_pgtk")]
include!("include/termhooks.rs");              // C 56: `#include "termhooks.h"`
#[cfg (feature = "have_pgtk")]
include!("include/termopts.rs");               // C 57: `#include "termopts.h"`
#[cfg (feature = "have_pgtk")]
include!("include/termchar.rs");               // C 58: `#include "termchar.h"`
#[cfg (feature = "have_pgtk")]
include!("include/menu.rs");                   // C 59: `#include "menu.h"`
#[cfg (feature = "have_pgtk")]
include!("include/window.rs");                 // C 60: `#include "window.h"`
#[cfg (feature = "have_pgtk")]
include!("include/keyboard.rs");               // C 61: `#include "keyboard.h"`
#[cfg (feature = "have_pgtk")]
include!("include/atimer.rs");                 // C 62: `#include "atimer.h"`
#[cfg (feature = "have_pgtk")]
include!("include/buffer.rs");                 // C 63: `#include "buffer.h"`
#[cfg (feature = "have_pgtk")]
include!("include/font.rs");                   // C 64: `#include "font.h"`
#[cfg (feature = "have_pgtk")]
include!("include/xsettings.rs");              // C 65: `#include "xsettings.h"`
#[cfg (feature = "have_pgtk")]
include!("include/emacsgtkfixed.rs");          // C 66: `#include "emacsgtkfixed.h"`
/* `dispextern.h` is not named by C 44-66; C reaches it through the chain (`frame.h` ->
   `termhooks.h` -> `dispextern.h`), and this slice reads `struct glyph_row`, `enum glyph_row_area`
   (C 116-117), `Mouse_HLInfo` (C 448) and `reset_mouse_highlight` (C 535) from it, so its mirror is
   in the graph (P-72 rule 4).  */
#[cfg (feature = "have_pgtk")]
include!("include/dispextern.rs");

/* C 68-70: `#ifdef GDK_WINDOWING_WAYLAND` / `#include <gdk/gdkwayland.h>` / `#endif`.  A GDK
   header macro, not a `configure` one - it is **not** in
   `docs/baseline/config/darwin-arm64.macros.tsv`, and the evaluator decides `#ifdef` on an absent
   name as false; the port's feature is the macro lower-cased (P-51).  The header is GDK's, has no
   file in the reference tree and no mirror, and this slice takes no name from it (the Wayland
   entry points it declares belong to the units beyond this slice), so the line stands as this
   comment (P-88 ①).  */
// C 68-70: the arm is `all (feature = "have_pgtk", feature = "gdk_windowing_wayland")`; the
// gated line is C 69: `#include <gdk/gdkwayland.h>` (no Rust item - nothing to include).
#[cfg (feature = "have_pgtk")]
include!("include/pgtkterm.rs");               // C 48's chain: `"gtkutil.h"` 29's `"pgtkterm.h"`
/* ----------------------------------------------------------------------------------------------
   The mirrors that own the Emacs-side names this slice uses and are placeholders today.  Each
   declaration keeps the reference's own prototype in a `/// C:` line and belongs in that mirror
   (P-72 rule 4); `docs/ports/pgtkterm.md` asks for the move and **this unit drops its copies in the
   commit that makes it** (the `src/image.rs` 181-201 shape).  The *records* those mirrors own
   (C 59-144, 146-455) are **not** spelled here - their pointees are GDK's and GTK's and no probe
   can read them on this machine (`docs/ports/include-pgtkterm.md`), so the names stand as C's
   tokens and the ledger records the gap (the `src/image.rs` 531-534 reading).
   ---------------------------------------------------------------------------------------------- */
/* termhooks.h 373-412: `enum event_kind kind : 16` shares the mirror's `input_event::bits` unit
   (PORTING.md 1.3 / P-80's member-named shape), so this unit spells the kind reads of C 308/309 and
   C 370/371 with its own copy (the `src/keyboard.rs` 4078-4085 / `src/androidterm.rs` 200 shape;
   this unit's items are gated because the frozen configuration compiles none of them).  */
/// C: termhooks.h 373-412: `enum event_kind kind : 16;` in `struct input_event`
#[cfg (feature = "have_pgtk")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}
/// C: dispextern.h 3026: `INLINE void reset_mouse_highlight (Mouse_HLInfo *hlinfo)` (C 535's call).
/// The mirror declares it inside `pub mod dispextern_externs`, which no glob import brings into this
/// unit, so the prototype stands here (P-72 rule 4, the `src/window.rs` 221-222 shape).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn reset_mouse_highlight (hlinfo: *mut Mouse_HLInfo);
}
/// C: lisp.h 4432-4438's family: `extern Lisp_Object make_formatted_string (const char *, …);`
/// (C 195's call site).  `docs/ports/lisp.md` owes the row.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn make_formatted_string (format: *const c_char, ...) -> Lisp_Object;
}
/// C: alloc.c's own entry point, declared in lisp.h; C 374-378's five call sites.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn mark_object (arg: Lisp_Object);
}
/// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object,
/// Lisp_Object);` (C 966's call).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
}
/// C: frame.h 774: `fset_parent_frame (struct frame *f, Lisp_Object val)` (C 1056's call).
/// `docs/ports/frame.md` owes the row.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
}
/// C: frame.h's `extern void check_window_system (struct frame *);` (C 450/607).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn check_window_system (f: *mut frame);
}
/// C: xfaces.c's `free_frame_faces` (C 460), declared in frame.h.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn free_frame_faces (f: *mut frame);
}
/// C: font.c's `get_font_ascent_descent` (C 896), declared in font.h.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int,
                                  descent: *mut c_int);
}
/// C: window.c's `adjust_frame_size` (C 930), declared in window.h.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                            inhibit: c_int, pretend: bool, parameter: Lisp_Object);
}
/// C: gtkutil.c's entry points (C 712/717/743/1017/1023/1067/1076).  `gtkutil.h`'s mirror is a
/// placeholder, so each is declared here with the reference's own prototype (P-72 rule 4).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: gtkutil.h 188: `extern void xg_create_frame_outer_widgets (struct frame *f);`
  pub fn xg_create_frame_outer_widgets (f: *mut frame);
  /// C: gtkutil.h 156: `extern void xg_wm_set_size_hint (struct frame *, long int, bool);`
  pub fn xg_wm_set_size_hint (f: *mut frame, flags: c_long, user_position: bool);
  /// C: gtkutil.h 165: `extern void xg_frame_set_char_size (struct frame *f, int width, int
  /// height);`
  pub fn xg_frame_set_char_size (f: *mut frame, width: c_int, height: c_int);
  /// C: gtkutil.h 203: `extern void xg_set_no_focus_on_map (struct frame *f, Lisp_Object
  /// no_focus_on_map);`
  pub fn xg_set_no_focus_on_map (f: *mut frame, no_focus_on_map: Lisp_Object);
  /// C: gtkutil.h 204: `extern void xg_set_no_accept_focus (struct frame *f, Lisp_Object
  /// no_accept_focus);`
  pub fn xg_set_no_accept_focus (f: *mut frame, no_accept_focus: Lisp_Object);
}
/// C: xwidget.h's `kill_frame_xwidget_views` (C 458, inside `#ifdef HAVE_XWIDGETS`).  That mirror
/// is a placeholder too, so the prototype stands here.
#[cfg (all (feature = "have_pgtk", feature = "have_xwidgets"))]
unsafe extern "C" {
  pub fn kill_frame_xwidget_views (f: *mut frame);
}

/* ----------------------------------------------------------------------------------------------
   C 72-74: this file's own three accessors.  `FRAME_X_OUTPUT`'s body is C's
   `((f)->output_data.pgtk)` with the union member's cast spelled out (`output_data.pgtk` is
   `*mut c_void` in `include/frame.rs` - the `src/xfns.rs` 996 reading).
   ---------------------------------------------------------------------------------------------- */
/// C: C 72: `#define FRAME_CR_CONTEXT(f) ((f)->output_data.pgtk->cr_context)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_CONTEXT {
  ($f:expr) => { (*(((*$f).output_data.pgtk) as *mut pgtk_output)).cr_context };
}
/// C: C 73: `#define FRAME_CR_ACTIVE_CONTEXT(f) ((f)->output_data.pgtk->cr_active)` (C 280-281
/// assigns it, hence the second arm)
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_ACTIVE_CONTEXT {
  ($f:expr) => { (*(((*$f).output_data.pgtk) as *mut pgtk_output)).cr_active };
  ($f:expr, $v:expr) => { (*(((*$f).output_data.pgtk) as *mut pgtk_output)).cr_active = $v };
}
/// C: C 74: `#define FRAME_CR_SURFACE(f) (cairo_get_target (FRAME_CR_CONTEXT (f)))`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_SURFACE {
  ($f:expr) => { cairo_get_target (FRAME_CR_CONTEXT! ($f)) };
}

/* C 474-516's pgtk accessors now live in their mirror, `include/pgtkterm.rs` (P-90 ①/②/③); the
   unit reaches them through the `include!` edge above.  The frame.h accessors C *writes* here stay
   below (their mirror, `include/frame.rs`, carries only the getter arm - Owed 5 of
   `docs/ports/pgtkterm.md`).  */
/// C: frame.h 1091's `FRAME_TAB_BAR_HEIGHT` and its three siblings are *this* unit's problem
/// because C assigns them here: the mirror's getter is the read arm, and the write arm is C's own
/// member (`include/frame.rs` 1222-1231, 977-1001).  Same two-arm shape; the `!` invocation is the
/// recorded deviation.
/// C: frame.h 1280/1282: `#define FRAME_LINE_HEIGHT(f) ...` / `#define FRAME_COLUMN_WIDTH(f) ...`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_COLUMN_WIDTH {
  ($f:expr) => { FRAME_COLUMN_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).column_width = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_LINE_HEIGHT {
  ($f:expr) => { FRAME_LINE_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).line_height = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_COLS ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_lines = $v };
}
/// C: frame.h 1248-1249: `#define FRAME_NO_FOCUS_ON_MAP(f) ((f)->no_focus_on_map)` /
/// `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)` - the two `bool_bf` members C writes
/// at C 1068/1077.  The write arm is the mirror's `frame_set_flag` (the `src/frame.rs` 259-260 /
/// `src/haikuterm.rs` 662 shape); the read arm is the mirror's getter.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_NO_FOCUS_ON_MAP {
  ($f:expr) => { FRAME_NO_FOCUS_ON_MAP ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT, $v as u32) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_NO_ACCEPT_FOCUS {
  ($f:expr) => { FRAME_NO_ACCEPT_FOCUS ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT, $v as u32) };
}

/* C 76-79: "Non-zero means that a HELP_EVENT has been generated since Emacs start." */
#[cfg (feature = "have_pgtk")]
static mut any_help_event_p: bool = false;

/* C 81-82: "Chain of existing displays" - this file's own non-`static` definition (pgtkterm.h 284
   declares it `extern`).  The X11 mirror carries the same name under its own gate
   (`include/xterm.rs`), which is why this one needs `have_pgtk`.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub static mut x_display_list: *mut pgtk_display_info = std::ptr::null_mut ();

/* C 84-88: this file's own `struct event_queue_t`.  */
/// C: C 84-88: `struct event_queue_t { union buffered_input_event *q; int nr, cap; };`
#[cfg (feature = "have_pgtk")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct event_queue_t
{
  pub q: *mut buffered_input_event,   // C: `union buffered_input_event *q;`
  pub nr: c_int,                      // C: `int nr, cap;`
  pub cap: c_int,
}
/// C 91: `static struct event_queue_t event_q;`
#[cfg (feature = "have_pgtk")]
static mut event_q: event_queue_t =
  event_queue_t { q: std::ptr::null_mut (), nr: 0, cap: 0 };

/// C 96: `static Time ignore_next_mouse_click_timeout;`
#[cfg (feature = "have_pgtk")]
static mut ignore_next_mouse_click_timeout: Time = 0;
/// C 99: `static Lisp_Object xg_default_icon_file;`
#[cfg (feature = "have_pgtk")]
static mut xg_default_icon_file: Lisp_Object = std::ptr::null_mut ();
/// C 102: `static GdkDragContext *current_drop_context;`
#[cfg (feature = "have_pgtk")]
static mut current_drop_context: *mut GdkDragContext = std::ptr::null_mut ();
/// C 106: `static bool current_drop_context_drop;`
#[cfg (feature = "have_pgtk")]
static mut current_drop_context_drop: bool = false;
/// C 109: `static guint32 current_drop_time;`
#[cfg (feature = "have_pgtk")]
static mut current_drop_time: guint32 = 0;

/* C 111-119: C's forward declarations.  Dropped (PORTING.md 0.1-12: Rust has no forward
   declaration and its items are order-independent); the definitions of C 111/112 are C 7314 and
   C 4835 (both outside this slice), which is why the four calls in this slice that C resolves
   through this block or through pgtkterm.h's prototype block - `pgtk_delete_display`,
   `pgtk_new_focus_frame`, `pgtk_clear_frame_area`, `pgtk_any_window_to_frame`, plus
   `pgtk_frame_rehighlight` (C 4770) and `pgtk_set_event_handler` (C 6831) - are kept as C's calls
   and recorded in `docs/ports/pgtkterm.md` (the `src/xfns.rs` 243 reading of the same rule).
   C 111: `static void pgtk_delete_display (struct pgtk_display_info *);`
   C 112: `static void pgtk_new_focus_frame (struct pgtk_display_info *, struct frame *);`
   C 113: `static void pgtk_clear_frame_area (struct frame *, int, int, int, int);`
   C 114-115: `static void pgtk_fill_rectangle (struct frame *, unsigned long, int, int, int, int,
   bool);`
   C 116-117: `static void pgtk_clip_to_row (struct window *, struct glyph_row *, enum
   glyph_row_area, cairo_t *);`
   C 118: `static struct frame *pgtk_any_window_to_frame (GdkWindow *);`
   C 119: `static void pgtk_regenerate_devices (struct pgtk_display_info *);` */

/* C 121-126: the device callbacks.  C's `gpointer user_data` is passed where a `struct
   pgtk_display_info *` is wanted; the port spells the conversion C performs implicitly
   (PORTING.md 1.4).  */
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_device_added_or_removal_cb (seat: *mut GdkSeat, device: *mut GdkDevice,
                                           user_data: gpointer)
{
  pgtk_regenerate_devices (user_data as *mut pgtk_display_info);
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_seat_added_cb (dpy: *mut GdkDisplay, seat: *mut GdkSeat,
                              user_data: gpointer)
{
  pgtk_regenerate_devices (user_data as *mut pgtk_display_info);

  g_signal_connect! (G_OBJECT! (seat), c"device-added".as_ptr (),
                     pgtk_device_added_or_removal_cb, user_data);
  g_signal_connect! (G_OBJECT! (seat), c"device-removed".as_ptr (),
                     pgtk_device_added_or_removal_cb, user_data);
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_seat_removed_cb (dpy: *mut GdkDisplay, seat: *mut GdkSeat,
                                user_data: gpointer)
{
  pgtk_regenerate_devices (user_data as *mut pgtk_display_info);

  g_signal_handlers_disconnect_by_func! (G_OBJECT! (seat),
                                         G_CALLBACK! (pgtk_device_added_or_removal_cb),
                                         user_data);
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_enumerate_devices (dpyinfo: *mut pgtk_display_info, initial_p: bool)
{
  let mut rec: *mut pgtk_device_t;
  let mut all_seats: *mut GList;
  let mut devices_on_seat: *mut GList;
  let mut tem: *mut GList;
  let mut t1: *mut GList;
  let mut seat: *mut GdkSeat;

  block_input ();
  all_seats = gdk_display_list_seats ((*dpyinfo).gdpy);

  tem = all_seats;
  while !tem.is_null ()
    {
      seat = GDK_SEAT! ((*tem).data);

      if initial_p
        {
          g_signal_connect! (G_OBJECT! (seat), c"device-added".as_ptr (),
                             pgtk_device_added_or_removal_cb, dpyinfo as gpointer);
          g_signal_connect! (G_OBJECT! (seat), c"device-removed".as_ptr (),
                             pgtk_device_added_or_removal_cb, dpyinfo as gpointer);
        }

      /* We only want slaves, not master devices.  */
      devices_on_seat = gdk_seat_get_slaves (seat, GDK_SEAT_CAPABILITY_ALL);

      t1 = devices_on_seat;
      while !t1.is_null ()
        {
          rec = xmalloc (core::mem::size_of::<pgtk_device_t> ()) as *mut pgtk_device_t;
          (*rec).seat = g_object_ref (seat as gpointer) as *mut GdkSeat;

          if !(*t1).data.is_null ()
            {
              let mut source: GdkInputSource;
              let mut name: *const gchar;

              (*rec).device = GDK_DEVICE! ((*t1).data);
              source = gdk_device_get_source ((*rec).device);
              name = gdk_device_get_name ((*rec).device);
              (*rec).name = make_formatted_string (c"%u:%s".as_ptr (), source, name);
            }
          else
            {
              /* GTK bug 7737 results in GDK seats being initialized
                 with NULL devices in some circumstances.  As events will
                 presumably also be delivered with their device fields
                 set to NULL, insert a ersatz device record associated
                 with NULL.  (bug#76239) */
              (*rec).device = std::ptr::null_mut ();
              (*rec).name = build_string (c"0:unknown device".as_ptr ());
            }

          (*rec).next = (*dpyinfo).devices;
          (*dpyinfo).devices = rec;

          t1 = (*t1).next;
        }

      g_list_free (devices_on_seat);

      tem = (*tem).next;
    }

  g_list_free (all_seats);
  unblock_input ();
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_free_devices (dpyinfo: *mut pgtk_display_info)
{
  let mut last: *mut pgtk_device_t;
  let mut tem: *mut pgtk_device_t;

  tem = (*dpyinfo).devices;
  while !tem.is_null ()
    {
      last = tem;
      tem = (*tem).next;

      g_object_unref ((*last).seat as gpointer);
      xfree (last as *mut c_void);
    }

  (*dpyinfo).devices = std::ptr::null_mut ();
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_regenerate_devices (dpyinfo: *mut pgtk_display_info)
{
  pgtk_free_devices (dpyinfo);
  pgtk_enumerate_devices (dpyinfo, false);
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_device_for_event (dpyinfo: *mut pgtk_display_info,
                                     event: *mut GdkEvent) -> Lisp_Object
{
  let mut tem: *mut pgtk_device_t;
  let mut device: *mut GdkDevice;

  device = gdk_event_get_source_device (event);

  if device.is_null ()
    {
      return Qt ();
    }

  tem = (*dpyinfo).devices;
  while !tem.is_null ()
    {
      if (*tem).device == device
        {
          return (*tem).name;
        }
      tem = (*tem).next;
    }

  return Qt ();
}

/* This is not a flip context in the same sense as gpu rendering
   scenes, it only occurs when a new context was required due to a
   resize or other fundamental change.  This is called when that
   context's surface has completed drawing.  */

#[cfg (feature = "have_pgtk")]
unsafe fn flip_cr_context (f: *mut frame)
{
  let cr: *mut cairo_t = FRAME_CR_ACTIVE_CONTEXT! (f);

  block_input ();
  if cr != FRAME_CR_CONTEXT! (f)
    {
      cairo_destroy (cr);

      FRAME_CR_ACTIVE_CONTEXT! (f, cairo_reference (FRAME_CR_CONTEXT! (f)));
    }
  unblock_input ();
}


#[cfg (feature = "have_pgtk")]
unsafe fn evq_enqueue (ev: *mut buffered_input_event)
{
  let evq: *mut event_queue_t = std::ptr::addr_of_mut! (event_q);
  let mut frame: *mut frame;
  let mut dpyinfo: *mut pgtk_display_info;

  if (*evq).cap == 0
    {
      (*evq).cap = 4;
      (*evq).q = xmalloc (core::mem::size_of::<buffered_input_event> ()
                          * (*evq).cap as usize) as *mut buffered_input_event;
    }

  if (*evq).nr >= (*evq).cap
    {
      (*evq).cap += (*evq).cap / 2;
      (*evq).q = xrealloc ((*evq).q as *mut c_void,
                           core::mem::size_of::<buffered_input_event> ()
                           * (*evq).cap as usize) as *mut buffered_input_event;
    }

  /* C's `evq->q[evq->nr++] = *ev`: the index is the old `nr` and the increment follows the store,
     which is the port's two statements (the post-increment is a recorded deviation).  */
  *(*evq).q.offset ((*evq).nr as isize) = *ev;
  (*evq).nr += 1;

  if input_event_bits! (kind, &raw mut (*ev).ie) != SELECTION_REQUEST_EVENT
      && input_event_bits! (kind, &raw mut (*ev).ie) != SELECTION_CLEAR_EVENT
    {
      frame = std::ptr::null_mut ();

      if WINDOWP ((*ev).ie.frame_or_window)
        {
          frame = WINDOW_XFRAME! (XWINDOW ((*ev).ie.frame_or_window));
        }

      if FRAMEP ((*ev).ie.frame_or_window)
        {
          frame = XFRAME ((*ev).ie.frame_or_window);
        }

      if !frame.is_null ()
        {
          dpyinfo = FRAME_DISPLAY_INFO! (frame);

          if ((*dpyinfo).last_user_time as c_ulong) < (*ev).ie.timestamp
            {
              (*dpyinfo).last_user_time = (*ev).ie.timestamp as guint32;
            }
        }
    }

  raise (SIGIO);
}

#[cfg (feature = "have_pgtk")]
unsafe fn evq_flush (hold_quit: *mut input_event) -> c_int
{
  let evq: *mut event_queue_t = std::ptr::addr_of_mut! (event_q);
  let mut n: c_int = 0;

  while (*evq).nr > 0
    {
      /* kbd_buffer_store_buffered_event may do longjmp, so
	 we need to shift event queue first and pass the event
	 to kbd_buffer_store_buffered_event so that events in
	 queue are not processed twice.  Bug#52941 */
      let mut ev: buffered_input_event = *(*evq).q;
      let mut i: c_int;
      i = 1;
      while i < (*evq).nr
        {
          *(*evq).q.offset ((i - 1) as isize) = *(*evq).q.offset (i as isize);
          i += 1;
        }
      (*evq).nr -= 1;

      kbd_buffer_store_buffered_event (&raw mut ev, hold_quit);
      n += 1;
    }

  return n;
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_pgtkterm ()
{
  let mut dpyinfo: *mut pgtk_display_info;
  let mut device: *mut pgtk_device_t;
  let evq: *mut event_queue_t = std::ptr::addr_of_mut! (event_q);
  let mut i: c_int;
  let n: c_int = (*evq).nr;

  i = 0;
  while i < n
    {
      let ev: *mut buffered_input_event = (*evq).q.offset (i as isize);

      /* Selection requests don't have Lisp object members.  */

      if input_event_bits! (kind, &raw mut (*ev).ie) == SELECTION_REQUEST_EVENT
	  || input_event_bits! (kind, &raw mut (*ev).ie) == SELECTION_CLEAR_EVENT
        {
          i += 1;
          continue;
        }

      mark_object ((*ev).ie.x);
      mark_object ((*ev).ie.y);
      mark_object ((*ev).ie.frame_or_window);
      mark_object ((*ev).ie.arg);
      mark_object ((*ev).ie.device);

      i += 1;
    }

  dpyinfo = x_display_list;
  while !dpyinfo.is_null ()
    {
      device = (*dpyinfo).devices;
      while !device.is_null ()
        {
          mark_object ((*device).name);
          device = (*device).next;
        }

      dpyinfo = (*dpyinfo).next;
    }
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_keysym_name (keysym: c_int) -> *mut c_char
{
  return gdk_keyval_name (keysym as guint);
}

/* --------------------------------------------------------------------------
     Programmatically reposition mouse pointer in pixel coordinates
   -------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_mouse_pixel_position (f: *mut frame, pix_x: c_int,
                                                         pix_y: c_int)
{
}

/* Raise frame F.  */

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_raise_frame (f: *mut frame)
{
  /* This works only for non-child frames on X.
     It does not work for child frames on X, and it does not work
     on Wayland too. */
  block_input ();
  if FRAME_VISIBLE_P (f)
    {
      gdk_window_raise (gtk_widget_get_window (FRAME_WIDGET! (f)));
    }
  unblock_input ();
}

/* Lower frame F.  */

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_lower_frame (f: *mut frame)
{
  if FRAME_VISIBLE_P (f)
    {
      block_input ();
      gdk_window_lower (gtk_widget_get_window (FRAME_WIDGET! (f)));
      unblock_input ();
    }
}

#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_frame_raise_lower (f: *mut frame, raise_flag: bool)
{
  if raise_flag
    {
      pgtk_raise_frame (f);
    }
  else
    {
      pgtk_lower_frame (f);
    }
}

/* Free X resources of frame F.  */

/* C 442-443: `static void pgtk_unlink_touch_points (struct frame *);` - a forward declaration,
   dropped (PORTING.md 0.1-12); its definition is C 6655, outside this slice, and C 456 calls it,
   so that call is kept and the name is one of the slice's unresolved ones (the `src/xfns.rs`
   243 reading).  */

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_free_frame_resources (f: *mut frame)
{
  let mut dpyinfo: *mut pgtk_display_info;
  let mut hlinfo: *mut Mouse_HLInfo;

  check_window_system (f);
  dpyinfo = FRAME_DISPLAY_INFO! (f);
  hlinfo = MOUSE_HL_INFO! (f);

  block_input ();

  pgtk_unlink_touch_points (f);
  #[cfg (feature = "have_xwidgets")]
  {
    kill_frame_xwidget_views (f);
  }
  free_frame_faces (f);

  if !(*FRAME_X_OUTPUT! (f)).scale_factor_atimer.is_null ()
    {
      cancel_atimer ((*FRAME_X_OUTPUT! (f)).scale_factor_atimer as *mut atimer);
      (*FRAME_X_OUTPUT! (f)).scale_factor_atimer = std::ptr::null_mut ();
    }

  /* C 468-469's `#define CLEAR_IF_EQ(FIELD) do { if (f == dpyinfo->FIELD) dpyinfo->FIELD = 0; }
     while (false)`.  C's `dpyinfo->FIELD` where FIELD is a *dotted* name (`im.focused_frame` at
     C 530) is why the port's parameter is a token sequence: `(*dpyinfo).$field` keeps C's
     invocation text.  C's `#undef CLEAR_IF_EQ` at C 532 has no Rust counterpart and the macro is
     function-scoped here, so it dies with the body.  */
  macro_rules! CLEAR_IF_EQ {
    ($($field:tt)+) => {
      if f == (*dpyinfo).$($field)+ { (*dpyinfo).$($field)+ = std::ptr::null_mut (); }
    };
  }

  /* If this frame currently holds keyboard focus, explicitly transfer
     focus to its parent frame before releasing resources.

     On X11 the X server delivers a FocusIn event to the parent when a
     child window is destroyed, so focus was restored automatically.
     On Wayland there is no such mechanism: the compositor does nothing,
     leaving Emacs with x_focus_frame == NULL permanently and ignoring
     all keyboard input (bug#64625).

     IMPORTANT: we must clear highlight_frame from f BEFORE calling
     pgtk_new_focus_frame.  pgtk_new_focus_frame calls
     pgtk_frame_rehighlight, which calls frame_unhighlight(old_highlight)
     where old_highlight = dpyinfo->highlight_frame.  If that is still f,
     frame_unhighlight -> gui_update_cursor(f) would try to use f's face
     cache, which was already freed by free_frame_faces(f) above ->
     segfault.  By clearing highlight_frame first, pgtk_frame_rehighlight
     sees old_highlight == NULL and skips frame_unhighlight entirely.  */
  if f == (*dpyinfo).x_focus_frame
    {
      let new_focus: *mut frame = FRAME_PARENT_FRAME (f);

      /* Clear highlight_frame from f so pgtk_frame_rehighlight will not
         call frame_unhighlight on the dying frame whose faces are freed. */
      if (*dpyinfo).highlight_frame == f
        {
          (*dpyinfo).highlight_frame = std::ptr::null_mut ();
        }

      if !new_focus.is_null ()
          && FRAME_LIVE_P (new_focus)
          && !FRAME_GTK_WIDGET! (new_focus).is_null ()
          && gtk_widget_get_realized (FRAME_GTK_WIDGET! (new_focus)) != 0
        {
          /* Transfer Emacs's internal focus/highlight to the parent.  */
          pgtk_new_focus_frame (dpyinfo, new_focus);
          /* Physically move the GTK/Wayland keyboard focus to the parent
             widget.  The Wayland compositor will not do this automatically
             when the child widget is destroyed.  */
          gtk_widget_grab_focus (FRAME_GTK_WIDGET! (new_focus));
        }
      else
        {
          /* Parent is gone or unrealized; just clear focus state.  */
          (*dpyinfo).x_focus_frame = std::ptr::null_mut ();
          pgtk_frame_rehighlight (dpyinfo);
        }
    }

  /* CLEAR_IF_EQ is now a no-op for highlight_frame if we already cleared
     it above, which is correct.  */
  CLEAR_IF_EQ! (highlight_frame);

  /* Clear x_focus_event_frame directly so that any focus-out event
     emitted by gtk_widget_destroy below is a no-op in
     pgtk_focus_changed, preventing a spurious
     pgtk_new_focus_frame(dpyinfo, NULL) that would undo the transfer.  */
  if f == (*dpyinfo).x_focus_event_frame
    {
      (*dpyinfo).x_focus_event_frame = std::ptr::null_mut ();
    }
  CLEAR_IF_EQ! (last_mouse_frame);
  CLEAR_IF_EQ! (last_mouse_motion_frame);
  CLEAR_IF_EQ! (last_mouse_glyph_frame);
  CLEAR_IF_EQ! (im.focused_frame);

  if f == (*hlinfo).mouse_face_mouse_frame as *mut frame
    {
      reset_mouse_highlight (hlinfo);
    }

  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).text_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).nontext_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).modeline_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).hand_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).hourglass_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).horizontal_drag_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).vertical_drag_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).left_edge_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).right_edge_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).top_edge_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).bottom_edge_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).top_left_corner_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).top_right_corner_cursor as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).bottom_right_corner_cursor
                  as *mut *mut GObject);
  g_clear_object (&raw mut (*FRAME_X_OUTPUT! (f)).bottom_left_corner_cursor
                  as *mut *mut GObject);


  if !(*FRAME_X_OUTPUT! (f)).border_color_css_provider.is_null ()
    {
      let ctxt: *mut GtkStyleContext = gtk_widget_get_style_context (FRAME_WIDGET! (f));
      let old: *mut GtkCssProvider = (*FRAME_X_OUTPUT! (f)).border_color_css_provider;
      gtk_style_context_remove_provider (ctxt, GTK_STYLE_PROVIDER! (old));
      g_object_unref (old as gpointer);
      (*FRAME_X_OUTPUT! (f)).border_color_css_provider = std::ptr::null_mut ();
    }

  if !(*FRAME_X_OUTPUT! (f)).scrollbar_foreground_css_provider.is_null ()
    {
      let old: *mut GtkCssProvider
        = (*FRAME_X_OUTPUT! (f)).scrollbar_foreground_css_provider;
      g_object_unref (old as gpointer);
      (*FRAME_X_OUTPUT! (f)).scrollbar_foreground_css_provider = std::ptr::null_mut ();
    }

  if !(*FRAME_X_OUTPUT! (f)).scrollbar_background_css_provider.is_null ()
    {
      let old: *mut GtkCssProvider
        = (*FRAME_X_OUTPUT! (f)).scrollbar_background_css_provider;
      g_object_unref (old as gpointer);
      (*FRAME_X_OUTPUT! (f)).scrollbar_background_css_provider = std::ptr::null_mut ();
    }

  gtk_widget_destroy (FRAME_WIDGET! (f));

  if !(*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell.is_null ()
    {
      cairo_surface_destroy ((*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell);
      (*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell = std::ptr::null_mut ();
    }

  if !(*FRAME_X_OUTPUT! (f)).atimer_visible_bell.is_null ()
    {
      cancel_atimer ((*FRAME_X_OUTPUT! (f)).atimer_visible_bell as *mut atimer);
      (*FRAME_X_OUTPUT! (f)).atimer_visible_bell = std::ptr::null_mut ();
    }

  xfree ((*f).output_data.pgtk);
  (*f).output_data.pgtk = std::ptr::null_mut ();

  unblock_input ();
}

/* --------------------------------------------------------------------------
     External: Delete the window
   -------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_destroy_window (f: *mut frame)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);

  check_window_system (f);
  if !(*dpyinfo).gdpy.is_null ()
    {
      pgtk_free_frame_resources (f);
    }

  (*dpyinfo).reference_count -= 1;
}

/* Calculate the absolute position in frame F
   from its current recorded position values and gravity.  */

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_calc_absolute_position (f: *mut frame)
{
  let flags: c_int = (*f).size_hint_flags;
  let p: *mut frame = FRAME_PARENT_FRAME (f);

  /* We have nothing to do if the current position
     is already for the top-left corner.  */
  if !((flags & XNegative) != 0 || (flags & YNegative) != 0)
    {
      return;
    }

  /* Treat negative positions as relative to the leftmost bottommost
     position that fits on the screen.  */
  if (flags & XNegative) != 0
    {
      let mut width: c_int = FRAME_PIXEL_WIDTH (f);

      /* A frame that has been visible at least once should have outer
	 edges.  */
      if (*((*f).output_data.pgtk as *mut pgtk_output)).has_been_visible != 0 && p.is_null ()
	{
	  let mut frame: Lisp_Object = std::ptr::null_mut ();
	  let mut edges: Lisp_Object = Qnil ();

	  XSETFRAME! (frame, f);
	  edges = Fpgtk_frame_edges (frame, Qouter_edges ());
	  if !NILP (edges)
	    {
	      width = ((XFIXNUM (Fnth (make_fixnum (2), edges))
			- XFIXNUM (Fnth (make_fixnum (0), edges)))) as c_int;
	    }
	}

      if !p.is_null ()
	{
	  (*f).left_pos = (FRAME_PIXEL_WIDTH (p) - width - 2 * (*f).border_width
			   + (*f).left_pos);
	}
      else
	{
	  (*f).left_pos = (pgtk_display_pixel_width (FRAME_DISPLAY_INFO! (f))
			   - width + (*f).left_pos);
	}

    }

  if (flags & YNegative) != 0
    {
      let mut height: c_int = FRAME_PIXEL_HEIGHT (f);

      if (*((*f).output_data.pgtk as *mut pgtk_output)).has_been_visible != 0 && p.is_null ()
	{
	  let mut frame: Lisp_Object = std::ptr::null_mut ();
	  let mut edges: Lisp_Object = Qnil ();

	  XSETFRAME! (frame, f);
	  if NILP (edges)
	    {
	      edges = Fpgtk_frame_edges (frame, Qouter_edges ());
	    }
	  if !NILP (edges)
	    {
	      height = ((XFIXNUM (Fnth (make_fixnum (3), edges))
			 - XFIXNUM (Fnth (make_fixnum (1), edges)))) as c_int;
	    }
	}

      if !p.is_null ()
	{
	  (*f).top_pos = (FRAME_PIXEL_HEIGHT (p) - height - 2 * (*f).border_width
			  + (*f).top_pos);
	}
      else
	{
	  (*f).top_pos = (pgtk_display_pixel_height (FRAME_DISPLAY_INFO! (f))
			  - height + (*f).top_pos);
	}
    }

  /* The left_pos and top_pos
     are now relative to the top and left screen edges,
     so the flags should correspond.  */
  (*f).size_hint_flags &= !(XNegative | YNegative);
}

/* CHANGE_GRAVITY is 1 when calling from Fset_frame_position,
   to really change the position, and 0 when calling from
   x_make_frame_visible (in that case, XOFF and YOFF are the current
   position values).  It is -1 when calling from x_set_frame_parameters,
   which means, do adjust for borders but don't change the gravity.  */

#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_gravity: c_int)
{
  if change_gravity > 0
    {
      (*f).top_pos = yoff;
      (*f).left_pos = xoff;
      (*f).size_hint_flags &= !(XNegative | YNegative);
      if xoff < 0
	{
	  (*f).size_hint_flags |= XNegative;
	}
      if yoff < 0
	{
	  (*f).size_hint_flags |= YNegative;
	}
      (*f).win_gravity = NorthWestGravity;
    }

  pgtk_calc_absolute_position (f);

  block_input ();
  xg_wm_set_size_hint (f, 0, false);

  if change_gravity != 0
    {
      if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
	{
	  gtk_window_move (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
			   (*f).left_pos, (*f).top_pos);
	}
      else
	{
	  let fixed: *mut GtkWidget = FRAME_GTK_WIDGET! (f);
	  let parent: *mut GtkWidget = gtk_widget_get_parent (fixed);
	  gtk_fixed_move (GTK_FIXED! (parent), fixed,
			  (*f).left_pos, (*f).top_pos);
	}
    }
  unblock_input ();
  return;
}

/* --------------------------------------------------------------------------
     Adjust window pixel size based on given width and height.
   -------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_window_size (f: *mut frame, change_gravity: bool,
				pixelwidth: c_int, pixelheight: c_int)
{
  block_input ();

  (*((*f).output_data.pgtk as *mut pgtk_output)).preferred_width = pixelwidth;
  (*((*f).output_data.pgtk as *mut pgtk_output)).preferred_height = pixelheight;

  xg_frame_set_char_size (f, pixelwidth, pixelheight);
  gtk_widget_queue_resize (FRAME_WIDGET! (f));

  unblock_input ();
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_iconify_frame (f: *mut frame)
{
  let mut window: *mut GtkWindow = std::ptr::null_mut ();

  /* Don't keep the highlight on an invisible frame.  */

  if (*FRAME_DISPLAY_INFO! (f)).highlight_frame == f
    {
      (*FRAME_DISPLAY_INFO! (f)).highlight_frame = std::ptr::null_mut ();
    }

  /* If the frame is already iconified, return.  */

  if FRAME_ICONIFIED_P (f)
    {
      return;
    }

  /* Child frames on PGTK have no outer widgets.  In that case, simply
     refuse to iconify the frame.  */

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    {
      if !FRAME_VISIBLE_P (f)
	{
	  gtk_widget_show_all (FRAME_GTK_OUTER_WIDGET! (f));
	}

      window = GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f));

      gtk_window_iconify (window);

      /* Don't make the frame iconified here.  Doing so will cause it
	 to be skipped by redisplay, until GDK says it is deiconified
	 (see window_state_event for more details).  However, if the
	 window server rejects the iconification request, GDK will
	 never tell Emacs about the iconification not happening,
	 leading to the frame not being redisplayed until the next
	 window state change.  */

      /* SET_FRAME_VISIBLE (f, 0);
	 SET_FRAME_ICONIFIED (f, true); */
    }
}

#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_make_frame_visible_wait_for_event_cb (widget: *mut GtkWidget,
							       event: *mut GdkEventAny,
							       user_data: gpointer)
  -> gboolean
{
  let foundptr: *mut c_int = user_data as *mut c_int;
  *foundptr = 1;
  return FALSE;
}

#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_make_frame_visible_wait_for_event_timeout (user_data: gpointer)
  -> gboolean
{
  let timedoutptr: *mut c_int = user_data as *mut c_int;
  *timedoutptr = 1;
  return FALSE;
}

#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_wait_for_event (f: *mut frame, signal_name: *const gchar)
{
  if FLOATP (Vpgtk_wait_for_event_timeout)
    {
      let msec: guint
	= (XFLOAT_DATA (Vpgtk_wait_for_event_timeout) * 1000.0) as guint;
      let mut found: c_int = 0;
      let mut timed_out: c_int = 0;
      let id: gulong
	= g_signal_connect! (FRAME_WIDGET! (f), signal_name,
			     pgtk_make_frame_visible_wait_for_event_cb,
			     &raw mut found as gpointer);
      let src: guint
	= g_timeout_add (msec,
			 Some (pgtk_make_frame_visible_wait_for_event_timeout),
			 &raw mut timed_out as gpointer);

      while found == 0 && timed_out == 0
	{
	  gtk_main_iteration ();
	}

      g_signal_handler_disconnect (FRAME_WIDGET! (f) as gpointer, id);

      if timed_out == 0
	{
	  g_source_remove (src);
	}
    }
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_make_frame_visible (f: *mut frame)
{
  let win: *mut GtkWidget = FRAME_GTK_OUTER_WIDGET! (f);

  if !FRAME_VISIBLE_P (f)
    {
      gtk_widget_show (FRAME_WIDGET! (f));
      if !win.is_null ()
	{
	  gtk_window_deiconify (GTK_WINDOW! (win));
	}

      pgtk_wait_for_event (f, c"map-event".as_ptr ());
    }
}


#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_make_frame_invisible (f: *mut frame)
{
  if FRAME_VISIBLE_P (f)
    {
      gtk_widget_hide (FRAME_WIDGET! (f));

      pgtk_wait_for_event (f, c"unmap-event".as_ptr ());

      SET_FRAME_VISIBLE! (f, 0);
      SET_FRAME_ICONIFIED (f, false as c_int);
    }
}

#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_make_frame_visible_invisible (f: *mut frame, visible: bool)
{
  if visible
    {
      pgtk_make_frame_visible (f);
    }
  else
    {
      pgtk_make_frame_invisible (f);
    }
}

/* C 875-876: `static Lisp_Object pgtk_new_font (...)`.  C's `static` stays internal (no
   `#[unsafe(no_mangle)]`, PORTING.md 0.2) but the address is taken - C 7480's
   `pgtk_frame_parm_handlers` table holds it - so the ABI is `extern "C"` (PORTING.md 1.4).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_new_font (f: *mut frame, font_object: Lisp_Object,
				   mut fontset: c_int) -> Lisp_Object
{
  let font: *mut font = XFONT_OBJECT (font_object);
  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;

  if fontset < 0
    {
      fontset = fontset_from_font (font_object);
    }
  FRAME_FONTSET! (f, fontset);

  if FRAME_FONT! (f) == font
    {
      /* This font is already set in frame F.  There's nothing more to
         do.  */
      return font_object;
    }

  FRAME_FONT! (f, font);

  FRAME_BASELINE_OFFSET! (f, (*font).baseline_offset);
  FRAME_COLUMN_WIDTH! (f, (*font).average_width);
  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);
  FRAME_LINE_HEIGHT! (f, font_ascent + font_descent);

  /* We could use a more elaborate calculation here.  */
  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));

  /* Compute the scroll bar width in character columns.  */
  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0
    {
      let wid: c_int = FRAME_COLUMN_WIDTH (f);
      FRAME_CONFIG_SCROLL_BAR_COLS! (f,
	(FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + wid - 1) / wid);
    }
  else
    {
      let wid: c_int = FRAME_COLUMN_WIDTH (f);
      FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + wid - 1) / wid);
    }

  /* Compute the scroll bar height in character lines.  */
  if FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) > 0
    {
      let height: c_int = FRAME_LINE_HEIGHT (f);
      FRAME_CONFIG_SCROLL_BAR_LINES! (f,
	(FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) + height - 1) / height);
    }
  else
    {
      let height: c_int = FRAME_LINE_HEIGHT (f);
      FRAME_CONFIG_SCROLL_BAR_LINES! (f, (14 + height - 1) / height);
    }

  /* Now make the frame display the given font.  */
  if !FRAME_GTK_WIDGET! (f).is_null ()
    {
      adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
			 FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 3,
			 false, Qfont ());
    }

  return font_object;
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_display_pixel_height (dpyinfo: *mut pgtk_display_info) -> c_int
{
  let gdpy: *mut GdkDisplay = (*dpyinfo).gdpy;
  let gscr: *mut GdkScreen = gdk_display_get_default_screen (gdpy);

  return gdk_screen_get_height (gscr);
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_display_pixel_width (dpyinfo: *mut pgtk_display_info) -> c_int
{
  let gdpy: *mut GdkDisplay = (*dpyinfo).gdpy;
  let gscr: *mut GdkScreen = gdk_display_get_default_screen (gdpy);

  return gdk_screen_get_width (gscr);
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_parent_frame (f: *mut frame, new_value: Lisp_Object,
						old_value: Lisp_Object)
{
  let mut p: *mut frame = std::ptr::null_mut ();

  if !NILP (new_value)
      && (!FRAMEP (new_value)
	  || { p = XFRAME (new_value); !FRAME_LIVE_P (p) }
	  || !FRAME_PGTK_P (p))
    {
      store_frame_param (f, Qparent_frame (), old_value);
      error (c"Invalid specification of `parent-frame'".as_ptr ());
    }

  if p != FRAME_PARENT_FRAME (f)
    {
      block_input ();

      if !p.is_null ()
	{
	  if FRAME_DISPLAY_INFO! (f) != FRAME_DISPLAY_INFO! (p)
	    {
	      error (c"Cross display reparent".as_ptr ());
	    }
	}

      let fixed: *mut GtkWidget = FRAME_GTK_WIDGET! (f);

      let mut alloc: GtkAllocation = GtkAllocation { x: 0, y: 0, width: 0, height: 0 };
      gtk_widget_get_allocation (fixed, &raw mut alloc);
      g_object_ref (fixed as gpointer);

      /* Remember the css provider, and restore it later. */
      let provider: *mut GtkCssProvider = (*FRAME_X_OUTPUT! (f)).border_color_css_provider;
      (*FRAME_X_OUTPUT! (f)).border_color_css_provider = std::ptr::null_mut ();
      {
	let ctxt: *mut GtkStyleContext = gtk_widget_get_style_context (FRAME_WIDGET! (f));
	if !provider.is_null ()
	  {
	    gtk_style_context_remove_provider (ctxt, GTK_STYLE_PROVIDER! (provider));
	  }
      }

      {
	let whbox_of_f: *mut GtkWidget = gtk_widget_get_parent (fixed);
	/* Here, unhighlight can be called and may change
	   border_color_css_provider.  */
	gtk_container_remove (GTK_CONTAINER! (whbox_of_f), fixed);

	if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
	  {
	    gtk_widget_destroy (FRAME_GTK_OUTER_WIDGET! (f));
	    FRAME_GTK_OUTER_WIDGET! (f, std::ptr::null_mut ());
	    (*FRAME_OUTPUT_DATA! (f)).vbox_widget = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).hbox_widget = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).menubar_widget = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).toolbar_widget = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).ttip_widget = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).ttip_lbl = std::ptr::null_mut ();
	    (*FRAME_OUTPUT_DATA! (f)).ttip_window = std::ptr::null_mut ();
	  }
      }

      if p.is_null ()
	{
	  xg_create_frame_outer_widgets (f);
	  pgtk_set_event_handler (f);
	  gtk_box_pack_start (GTK_BOX! ((*((*f).output_data.pgtk as *mut pgtk_output)).hbox_widget),
			      fixed, TRUE, TRUE, 0);
	  (*((*f).output_data.pgtk as *mut pgtk_output)).preferred_width = alloc.width;
	  (*((*f).output_data.pgtk as *mut pgtk_output)).preferred_height = alloc.height;
	  xg_wm_set_size_hint (f, 0, false);
	  xg_frame_set_char_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, alloc.width),
				  FRAME_PIXEL_TO_TEXT_HEIGHT (f, alloc.height));
	  gtk_widget_queue_resize (FRAME_WIDGET! (f));
	  gtk_widget_show_all (FRAME_GTK_OUTER_WIDGET! (f));
	}
      else
	{
	  let fixed_of_p: *mut GtkWidget = FRAME_GTK_WIDGET! (p);
	  gtk_fixed_put (GTK_FIXED! (fixed_of_p), fixed, (*f).left_pos, (*f).top_pos);
	  gtk_widget_set_size_request (fixed, alloc.width, alloc.height);
	  gtk_widget_show_all (fixed);
	}

      /* Restore css provider. */
      let ctxt: *mut GtkStyleContext = gtk_widget_get_style_context (FRAME_WIDGET! (f));
      let old: *mut GtkCssProvider = (*FRAME_X_OUTPUT! (f)).border_color_css_provider;
      (*FRAME_X_OUTPUT! (f)).border_color_css_provider = provider;
      if !provider.is_null ()
	{
	  gtk_style_context_add_provider (ctxt, GTK_STYLE_PROVIDER! (provider),
					  GTK_STYLE_PROVIDER_PRIORITY_USER);
	}
      if !old.is_null ()
	{
	  gtk_style_context_remove_provider (ctxt, GTK_STYLE_PROVIDER! (old));
	  g_object_unref (old as gpointer);
	}

      g_object_unref (fixed as gpointer);

      unblock_input ();

      fset_parent_frame (f, new_value);
    }
}

/* Doesn't work on wayland.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object,
						   old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      xg_set_no_focus_on_map (f, new_value);
      FRAME_NO_FOCUS_ON_MAP! (f, !NILP (new_value));
    }
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object,
						   old_value: Lisp_Object)
{
  xg_set_no_accept_focus (f, new_value);
  FRAME_NO_ACCEPT_FOCUS! (f, !NILP (new_value));
}

/// C: frame.h 1262-1267: `#define FRAME_Z_GROUP(f) ((f)->z_group)` - written at C 1092/1100/1106/
/// 1114.  `z_group` is `ENUM_BF (z_group) z_group : 2`, so the write is C's own read-modify-write
/// of the two-bit field (`include/frame.rs` has the reader and no two-bit setter - its
/// `frame_set_flag` writes one bit; the `src/haikufns.rs` 403-412 shape).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_Z_GROUP {
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(FRAME_Z_GROUP_MASK << FRAME_Z_GROUP_FLAG_SHIFT))
                  | (((($v) as u32) & FRAME_Z_GROUP_MASK) << FRAME_Z_GROUP_FLAG_SHIFT) };
}
/* C: dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);` - the
   C 1183/1224 calls (`src/xfaces.rs` defines it).  The `///` form warns on an `extern` block
   (the `docs/ports/pgtkfns.md` Deviation 2 shape), so these four blocks carry `/* */`.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
}
/* C: dispextern.h 3661-3662: `extern int get_glyph_string_clip_rects (struct glyph_string *,
   XRectangle *, int);` - the C 1265 call (`src/xdisp.rs` defines it).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn get_glyph_string_clip_rects (s: *mut glyph_string, r: *mut XRectangle,
				      n: c_int) -> c_int;
}
/* C: cairo.h 954-956: `void cairo_rectangle (cairo_t *cr, double x, double y, double width,
   double height);` - the C 1271 call (this machine's cairo header; `src/ftcrfont.rs` 283-286
   carries the same row).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_rectangle (cr: *mut cairo_t, x: c_double, y: c_double, width: c_double,
			  height: c_double);
}
/* C: cairo.h 1033: `void cairo_clip (cairo_t *cr);` - the C 1273 call.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_clip (cr: *mut cairo_t);
}

/* C 1080-1118: `void pgtk_set_z_group (struct frame *f, Lisp_Object new_value, Lisp_Object
   old_value)` - the four `FRAME_Z_GROUP` arms with their `gtk_window_set_keep_*` pairs, and C's
   `error` for anything else.  `old_value` is unused in C's body (`let _ =` keeps the name).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_z_group (f: *mut frame, new_value: Lisp_Object,
					    old_value: Lisp_Object)
{
  let _ = old_value;
  if FRAME_GTK_OUTER_WIDGET! (f).is_null ()                            // C 1083
    { return; }                                                        // C 1084

  if NILP (new_value)                                                  // C 1086
    {
      gtk_window_set_keep_above (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 FALSE);                                   // C 1088-1089
      gtk_window_set_keep_below (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 FALSE);                                   // C 1090-1091
      FRAME_Z_GROUP! (f, z_group_none);                                // C 1092
    }
  else if EQ (new_value, Qabove ())                                    // C 1094
    {
      gtk_window_set_keep_above (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 TRUE);                                    // C 1096-1097
      gtk_window_set_keep_below (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 FALSE);                                   // C 1098-1099
      FRAME_Z_GROUP! (f, z_group_above);                               // C 1100
    }
  else if EQ (new_value, Qabove_suspended ())                          // C 1102
    {
      gtk_window_set_keep_above (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 FALSE);                                   // C 1104-1105
      FRAME_Z_GROUP! (f, z_group_above_suspended);                     // C 1106
    }
  else if EQ (new_value, Qbelow ())                                    // C 1108
    {
      gtk_window_set_keep_above (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 FALSE);                                   // C 1110-1111
      gtk_window_set_keep_below (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				 TRUE);                                    // C 1112-1113
      FRAME_Z_GROUP! (f, z_group_below);                               // C 1114
    }
  else
    { error (c"Invalid z-group specification".as_ptr ()); }            // C 1117
}

/* --------------------------------------------------------------------------
      Initialize global info and storage for display.
   --------------------------------------------------------------------------  */
/// C 1120-1137: `static void pgtk_initialize_display_info (struct pgtk_display_info *dpyinfo)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_initialize_display_info (dpyinfo: *mut pgtk_display_info)
{
  (*dpyinfo).resx = 96.0;                                              // C 1126
  (*dpyinfo).resy = 96.0;                                              // C 1127
  (*dpyinfo).color_p = 1;                                              // C 1128
  (*dpyinfo).n_planes = 32;                                            // C 1129
  (*dpyinfo).root_window = 42;	/* a placeholder..  */                  // C 1130
  (*dpyinfo).highlight_frame = std::ptr::null_mut ();                  // C 1131
  (*dpyinfo).x_focus_frame = std::ptr::null_mut ();                    // C 1131
  (*dpyinfo).n_fonts = 0;                                              // C 1132
  (*dpyinfo).smallest_font_height = 1;                                 // C 1133
  (*dpyinfo).smallest_char_width = 1;                                  // C 1134

  reset_mouse_highlight (&raw mut (*dpyinfo).mouse_highlight);         // C 1136
}

/* Set S->gc to a suitable GC for drawing glyph string S in cursor
   face.  */
/// C 1142-1175: `static void pgtk_set_cursor_gc (struct glyph_string *s)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_cursor_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;   /* C's `s->face` is the port's `*mut c_void` */

  if (*s).font == FRAME_FONT! ((*s).f)                                 // C 1145
    && (*face).background == FRAME_BACKGROUND_PIXEL ((*s).f)           // C 1146
    && (*face).foreground == FRAME_FOREGROUND_PIXEL ((*s).f)           // C 1147
    && (*s).cmp.is_null ()                                     // C 1147's `!s->cmp`
    { (*s).xgcv = (*FRAME_X_OUTPUT! ((*s).f)).cursor_xgcv; }           // C 1148
  else
    {
      /* Cursor on non-default face: must merge.  */
      let mut xgcv: Emacs_GC = std::mem::zeroed ();                    // C 1152

      xgcv.background = (*FRAME_X_OUTPUT! ((*s).f)).cursor_color;      // C 1154
      xgcv.foreground = (*face).background;                            // C 1155

      /* If the glyph would be invisible, try a different foreground.  */
      if xgcv.foreground == xgcv.background                             // C 1158
	{ xgcv.foreground = (*face).foreground; }                           // C 1159
      if xgcv.foreground == xgcv.background                             // C 1160
	{ xgcv.foreground = (*FRAME_X_OUTPUT! ((*s).f)).cursor_foreground_color; }   // C 1161
      if xgcv.foreground == xgcv.background                             // C 1162
	{ xgcv.foreground = (*face).foreground; }                           // C 1163

      /* Make sure the cursor is distinct from text in this face.  */
      if xgcv.background == (*face).background                          // C 1166
	  && xgcv.foreground == (*face).foreground                        // C 1167
	{
	  xgcv.background = (*face).foreground;                           // C 1169
	  xgcv.foreground = (*face).background;                           // C 1170
	}

      (*s).xgcv = xgcv;                                                 // C 1173
    }
}

/* Set up S->gc of glyph string S for drawing text in mouse face.  */
/// C 1180-1202: `static void pgtk_set_mouse_face_gc (struct glyph_string *s)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_mouse_face_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;   /* C's `s->face` is the port's `*mut c_void` */

  prepare_face_for_display ((*s).f, face);                              // C 1183

  if (*s).font == (*face).font as *mut font                            // C 1185
    {
      (*s).xgcv.foreground = (*face).foreground;                        // C 1187
      (*s).xgcv.background = (*face).background;                        // C 1188
    }
  else
    {
      /* Otherwise construct scratch_cursor_gc with values from FACE
         except for FONT.  */
      let mut xgcv: Emacs_GC = std::mem::zeroed ();                     // C 1194

      xgcv.background = (*face).background;                             // C 1196
      xgcv.foreground = (*face).foreground;                             // C 1197

      (*s).xgcv = xgcv;                                                 // C 1199
    }
}

/* Set S->gc of glyph string S to a GC suitable for drawing a mode line.
   Faces to use in the mode line have already been computed when the
   matrix was built, so there isn't much to do, here.  */
/// C 1209-1214: `static void pgtk_set_mode_line_face_gc (struct glyph_string *s)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_mode_line_face_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;   /* C's `s->face` is the port's `*mut c_void` */

  (*s).xgcv.foreground = (*face).foreground;                            // C 1212
  (*s).xgcv.background = (*face).background;                            // C 1213
}

/* Set S->gc of glyph string S for drawing that glyph string.  Set
   S->stippled_p to a non-zero value if the face of S has a stipple
   pattern.  */
/// C 1221-1255: `static void pgtk_set_glyph_string_gc (struct glyph_string *s)` - C's five-arm
/// `s->hl` dispatch, whose last arm is `emacs_abort ()`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_glyph_string_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;   /* C's `s->face` is the port's `*mut c_void` */

  prepare_face_for_display ((*s).f, face);                              // C 1224

  if (*s).hl == DRAW_NORMAL_TEXT                                        // C 1226
    {
      (*s).xgcv.foreground = (*face).foreground;                        // C 1228
      (*s).xgcv.background = (*face).background;                        // C 1229
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);      // C 1230
    }
  else if (*s).hl == DRAW_INVERSE_VIDEO                                 // C 1232
    {
      pgtk_set_mode_line_face_gc (s);                                   // C 1234
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);      // C 1235
    }
  else if (*s).hl == DRAW_CURSOR                                        // C 1237
    {
      pgtk_set_cursor_gc (s);                                           // C 1239
      glyph_string_bits! (stippled_p, (*s), false);                     // C 1240
    }
  else if (*s).hl == DRAW_MOUSE_FACE                                    // C 1242
    {
      pgtk_set_mouse_face_gc (s);                                       // C 1244
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);      // C 1245
    }
  else if (*s).hl == DRAW_IMAGE_RAISED || (*s).hl == DRAW_IMAGE_SUNKEN  // C 1247
    {
      (*s).xgcv.foreground = (*face).foreground;                        // C 1249
      (*s).xgcv.background = (*face).background;                        // C 1250
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);      // C 1251
    }
  else
    { emacs_abort (); }                                                 // C 1254
}

/* Set clipping for output of glyph string S.  S may be part of a mode
   line or menu if we don't have X toolkit support.  */
/// C 1261-1275: `static void pgtk_set_glyph_string_clipping (struct glyph_string *s, cairo_t *cr)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_glyph_string_clipping (s: *mut glyph_string, cr: *mut cairo_t)
{
  let mut r: [XRectangle; 2] = std::mem::zeroed ();                    // C 1264
  let n: c_int = get_glyph_string_clip_rects (s, r.as_mut_ptr (), 2);   // C 1265

  if n > 0                                                              // C 1267
    {
      for i in 0..n                                                     // C 1269
	{
	  cairo_rectangle (cr, r[i as usize].x as c_double,             // C 1271
			   r[i as usize].y as c_double,
			   r[i as usize].width as c_double,
			   r[i as usize].height as c_double);
	}
      cairo_clip (cr);                                                  // C 1273
    }
}

/// C 1281-1293: `static void pgtk_set_glyph_string_clipping_exactly (struct glyph_string *src,
/// struct glyph_string *dst, cairo_t *cr)` - C's four `dst->clip[0]` writes go through the
/// degraded `NativeRectangle clip[2]` the mirror carries as `[c_int; 2]` (the cast device
/// `src/xterm.rs`'s `x_set_glyph_string_clipping_exactly` uses), with C's implicit narrowing to
/// `short`/`unsigned short` written out.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_glyph_string_clipping_exactly (src: *mut glyph_string, dst: *mut glyph_string,
						  cr: *mut cairo_t)
{
  let clip: *mut XRectangle = (&raw mut (*dst).clip[0]) as *mut XRectangle;

  /* C's `XRectangle` here is `pgtkgui.h`'s (the pgtk arm's - `{ int x, y; unsigned width,
     height; }`, `include/pgtkgui.rs` 139, status 3.533/3.542), not Xlib's, so C 1285-1288 are a
     plain int copy plus the two `int` -> `unsigned` conversions.  */
  (*clip).x = (*src).x;                                                 // C 1285
  (*clip).y = (*src).y;                                                 // C 1286
  (*clip).width = (*src).width as c_uint;                               // C 1287
  (*clip).height = (*src).height as c_uint;                             // C 1288
  (*dst).num_clips = 1;                                                 // C 1289

  cairo_rectangle (cr, (*src).x as c_double, (*src).y as c_double,      // C 1291
		   (*src).width as c_double, (*src).height as c_double);
  cairo_clip (cr);                                                      // C 1292
}

/* RIF:
   Compute left and right overhang of glyph string S.  */

/// C 1298-1333: `static void pgtk_compute_glyph_string_overhangs (struct glyph_string *s)` -
/// `s->first_glyph->type` is the glyph record's bitfield run read through `glyph_bits!` and the
/// `font->driver->text_extents` call is the driver's slot unwrapped, both the `src/xterm.rs`
/// `x_compute_glyph_string_overhangs` shapes; C 1309's `alloca` goes to the stack pool the
/// `AVAIL_ALLOCA!`/`SAFE_ALLOCA!` macros allocate from (PORTING.md P-05, the `src/alloc.rs`
/// 7948-7957 shape - a plain `alloca` site takes the pool directly and accounts nothing against
/// `sa_avail`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_compute_glyph_string_overhangs (s: *mut glyph_string)
{
  if (*s).cmp.is_null ()                                               // C 1301
    && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH          // C 1302
	|| glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH) // C 1303
    {
      let mut metrics: font_metrics = std::mem::zeroed ();             // C 1305

      if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH         // C 1307
	{
	  let code: *mut c_uint = sa_stack_alloc (std::mem::size_of::<c_uint> ()
						  * (*s).nchars as usize) as *mut c_uint;   // C 1309
	  let font: *mut font = (*s).font;                                 // C 1310
	  let mut i: c_int;                                                // C 1311

	  i = 0;                                                           // C 1313
	  while i < (*s).nchars
	    {
	      *code.offset (i as isize) = *(*s).char2b.offset (i as isize);   // C 1314
	      i += 1;                                                      // C 1313's step
	    }
	  ((*((*font).driver as *const font_driver)).text_extents.unwrap ())
	    (font, code, (*s).nchars, &raw mut metrics);                   // C 1315
	}
      else
	{
	  let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 1319

	  composition_gstring_width (gstring, (*s).cmp_from as ptrdiff_t,  // C 1321
				     (*s).cmp_to as ptrdiff_t,          // C 1322
				     &raw mut metrics as *mut c_void);
	}
      (*s).right_overhang = if metrics.rbearing > metrics.width        // C 1324
	{ (metrics.rbearing - metrics.width) as c_int } else { 0 };     // C 1325
      (*s).left_overhang = if metrics.lbearing < 0                     // C 1326
	{ (- metrics.lbearing) as c_int } else { 0 };
    }
  else if !(*s).cmp.is_null ()                                         // C 1328
    {
      (*s).right_overhang = ((*( (*s).cmp as *mut composition )).rbearing   // C 1330
			     - (*( (*s).cmp as *mut composition )).pixel_width) as c_int;
      (*s).left_overhang = (- (*( (*s).cmp as *mut composition )).lbearing) as c_int;   // C 1331
    }
}

/* Fill rectangle X, Y, W, H with background color of glyph string
   S.  */

/// C 1337-1344: `static void pgtk_clear_glyph_string_rect (struct glyph_string *s, int x, int y,
/// int w, int h)` - `pgtk_fill_rectangle` is C 5035's definition (forward-declared at C 114-115,
/// dropped by PORTING.md 0.1-12), i.e. later in this unit: the call is the class this ledger's
/// Evidence ③ table names for this file's not-yet-ported items.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_clear_glyph_string_rect (s: *mut glyph_string, x: c_int, y: c_int,
					w: c_int, h: c_int)
{
  pgtk_fill_rectangle ((*s).f, (*s).xgcv.background, x, y, w, h,       // C 1341
		       glyph_bits! (type, (*(*s).first_glyph)) != STRETCH_GLYPH   // C 1342
		       || (*s).hl != DRAW_CURSOR);                         // C 1343
}

/// C 1346-1377: `static void fill_background_by_face (struct frame *f, struct face *face, int x,
/// int y, int width, int height)` - the cairo source/pattern block, whose four channel values are
/// C's integer shifts divided by the `double` 255.0 (the conversion written out) and whose stipple
/// arm indexes `FRAME_DISPLAY_INFO (f)->bitmaps` through the pointer offset the `pgtk` lane uses
/// (`src/pgtkfns.rs`'s monitor walk shape).
#[cfg (feature = "have_pgtk")]
unsafe fn fill_background_by_face (f: *mut frame, face: *mut face, x: c_int, y: c_int,
				   width: c_int, height: c_int)
{
  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 1350
  let mut r: c_double;                                                  // C 1351
  let mut g: c_double;
  let mut b: c_double;
  let mut a: c_double;

  cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);                       // C 1353
  cairo_rectangle (cr, x as c_double, y as c_double,                    // C 1354
		   width as c_double, height as c_double);
  cairo_clip (cr);                                                      // C 1355

  r = (((*face).background >> 16) & 0xff) as c_double / 255.0;          // C 1357
  g = (((*face).background >> 8) & 0xff) as c_double / 255.0;           // C 1358
  b = (((*face).background >> 0) & 0xff) as c_double / 255.0;           // C 1359
  a = (*f).alpha_background;                                            // C 1360
  cairo_set_source_rgba (cr, r, g, b, a);                               // C 1361
  cairo_paint (cr);                                                     // C 1362

  if (*face).stipple != 0                                               // C 1364
    {
      let mask: *mut cairo_pattern_t
	= (*(*FRAME_DISPLAY_INFO! (f)).bitmaps.offset ((*face).stipple as isize - 1)).pattern;   // C 1367

      r = (((*face).foreground >> 16) & 0xff) as c_double / 255.0;      // C 1369
      g = (((*face).foreground >> 8) & 0xff) as c_double / 255.0;       // C 1370
      b = (((*face).foreground >> 0) & 0xff) as c_double / 255.0;       // C 1371
      cairo_set_source_rgba (cr, r, g, b, a);                           // C 1372
      cairo_mask (cr, mask);                                            // C 1373
    }

  pgtk_end_cr_clip (f);                                                 // C 1376
}

/// C 1379-1383: `static void fill_background (struct glyph_string *s, int x, int y, int width,
/// int height)` - C's `s->face` is the port's `*mut c_void` (the cast the six GC helpers use).
#[cfg (feature = "have_pgtk")]
unsafe fn fill_background (s: *mut glyph_string, x: c_int, y: c_int, width: c_int, height: c_int)
{
  fill_background_by_face ((*s).f, (*s).face as *mut face, x, y, width, height);   // C 1382
}

/* The `ENUM_BF (face_box_type) box : 2` / `ENUM_BF (face_underline_type) underline : 3`
   members and the seventeen `bool_bf` members C declares at dispextern.h 1893-1935, in C's
   declaration order (the `src/haikuterm.rs` 300-348 and `src/font.rs` 228-250 copies of the same
   run; `include/dispextern.rs` 2256 stores C's bytes as one `u32`, so the masks are what names
   every member).  The `face_bits!` macro below covers the members this file's slices read.  */
#[cfg (feature = "have_pgtk")]
pub const FACE_BOX_SHIFT: u32 = 0;
#[cfg (feature = "have_pgtk")]
pub const FACE_BOX_MASK: u32 = 0x3;
#[cfg (feature = "have_pgtk")]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (feature = "have_pgtk")]
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
#[cfg (feature = "have_pgtk")]
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
#[cfg (feature = "have_pgtk")]
pub const FACE_OVERLINE_P: u32 = 1 << 6;
#[cfg (feature = "have_pgtk")]
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
#[cfg (feature = "have_pgtk")]
pub const FACE_FOREGROUND_DEFAULTED_P: u32 = 1 << 8;
#[cfg (feature = "have_pgtk")]
pub const FACE_BACKGROUND_DEFAULTED_P: u32 = 1 << 9;
#[cfg (feature = "have_pgtk")]
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
#[cfg (feature = "have_pgtk")]
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
#[cfg (feature = "have_pgtk")]
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
#[cfg (feature = "have_pgtk")]
pub const FACE_BOX_COLOR_DEFAULTED_P: u32 = 1 << 13;
#[cfg (feature = "have_pgtk")]
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;
#[cfg (feature = "have_pgtk")]
pub const FACE_TTY_BOLD_P: u32 = 1 << 15;
#[cfg (feature = "have_pgtk")]
pub const FACE_TTY_ITALIC_P: u32 = 1 << 16;
#[cfg (feature = "have_pgtk")]
pub const FACE_TTY_REVERSE_P: u32 = 1 << 17;
#[cfg (feature = "have_pgtk")]
pub const FACE_TTY_STRIKE_THROUGH_P: u32 = 1 << 18;
#[cfg (feature = "have_pgtk")]
pub const FACE_TTY_OVERLINE_P: u32 = 1 << 19;
#[cfg (feature = "have_pgtk")]
pub const FACE_COLORS_COPIED_BITWISE_P: u32 = 1 << 20;
#[cfg (feature = "have_pgtk")]
pub const FACE_OVERSTRIKE: u32 = 1 << 21;

/* C's member names are the macro's first argument, so they survive the `--check` body comparison
   (the `src/xterm.rs` 9575-9589 `face_bits!` shape).  */
#[cfg (feature = "have_pgtk")]
macro_rules! face_bits {
  (box, $f:expr) => { (((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT) as c_int };
  (underline, $f:expr) =>
    { (((*$f).bits & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT) as c_int };
  (use_box_color_for_shadows_p, $f:expr) =>
    { ((*$f).bits & FACE_USE_BOX_COLOR_FOR_SHADOWS_P) != 0 };
  (overline_p, $f:expr) => { ((*$f).bits & FACE_OVERLINE_P) != 0 };
  (strike_through_p, $f:expr) => { ((*$f).bits & FACE_STRIKE_THROUGH_P) != 0 };
  (underline_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_DEFAULTED_P) != 0 };
  (overline_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_OVERLINE_COLOR_DEFAULTED_P) != 0 };
  (strike_through_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P) != 0 };
  (underline_at_descent_line_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_AT_DESCENT_LINE_P) != 0 };
  (overstrike, $f:expr) => { ((*$f).bits & FACE_OVERSTRIKE) != 0 };
}

/* Draw the background of glyph_string S.  If S->background_filled_p
   is non-zero don't draw it.  FORCE_P non-zero means draw the
   background even if it wouldn't be drawn normally.  This is used
   when a string preceding S draws into the background of S, or S
   contains the first component of a composition.  */

/// C 1390-1422: `static void pgtk_draw_glyph_string_background (struct glyph_string *s, bool
/// force_p)` - C's four `s->*_p` bitfields are the mirror's `glyph_string_bits!` reads and C's
/// `max` is the port's own fold (`include/lisp.rs` 3149), whose `isize` signature makes the two
/// operands' conversions explicit (the `src/xterm.rs` `x_draw_glyph_string_background` shape).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_glyph_string_background (s: *mut glyph_string, force_p: bool)
{
  /* Nothing to do if background has already been drawn or if it
     shouldn't be drawn in the first place.  */
  if !glyph_string_bits! (background_filled_p, (*s))                    // C 1395
    {
      let box_line_width: c_int
	= max ((*((*s).face as *mut face)).box_horizontal_line_width as isize, 0) as c_int;   // C 1397

      if glyph_string_bits! (stippled_p, (*s))                          // C 1399
	{
	  /* Fill background with a stipple pattern.  */
	  fill_background (s, (*s).x, (*s).y + box_line_width,          // C 1402
			   (*s).background_width,                       // C 1403
			   (*s).height - 2 * box_line_width);           // C 1404
	  glyph_string_bits! (background_filled_p, (*s), true);         // C 1405
	}
      else if FONT_HEIGHT! ((*s).font) < (*s).height - 2 * box_line_width   // C 1407
	       /* When xdisp.c ignores FONT_HEIGHT, we cannot trust
	          font dimensions, since the actual glyphs might be
	          much smaller.  So in that case we always clear the
	          rectangle with background color.  */
	       || FONT_TOO_HIGH! ((*s).font)                            // C 1412
	       || glyph_string_bits! (font_not_found_p, (*s))           // C 1413
	       || glyph_string_bits! (extends_to_end_of_line_p, (*s))   // C 1414
	       || force_p
	{
	  pgtk_clear_glyph_string_rect (s, (*s).x, (*s).y + box_line_width,   // C 1416
					(*s).background_width,          // C 1417
					(*s).height - 2 * box_line_width);   // C 1418
	  glyph_string_bits! (background_filled_p, (*s), true);         // C 1419
	}
    }
}

/// C 1425-1437: `static void pgtk_draw_rectangle (struct frame *f, unsigned long color, int x,
/// int y, int width, int height, bool respect_alpha_background)` - C's `x + 0.5` widening and the
/// `cairo_set_line_width (cr, 1)` are written out.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_rectangle (f: *mut frame, color: c_ulong, x: c_int, y: c_int,
			       width: c_int, height: c_int, respect_alpha_background: bool)
{
  let cr: *mut cairo_t;                                                 // C 1429

  cr = pgtk_begin_cr_clip (f);                                          // C 1431
  pgtk_set_cr_source_with_color (f, color, respect_alpha_background);   // C 1432
  cairo_rectangle (cr, x as c_double + 0.5, y as c_double + 0.5,        // C 1433
		   width as c_double, height as c_double);
  cairo_set_line_width (cr, 1.0);                                       // C 1434
  cairo_stroke (cr);                                                    // C 1435
  pgtk_end_cr_clip (f);                                                 // C 1436
}

/* Draw the foreground of glyph string S.  */

/// C 1440-1483: `static void pgtk_draw_glyph_string_foreground (struct glyph_string *s)` -
/// `face_bits!`/`glyph_bits!` for C's `s->face->box`/`s->first_glyph->left_box_line_p`/
/// `s->face->overstrike`, the `font->driver->draw` slot unwrapped, and C's `i`/`x` counters as
/// the port's `while` trio (the `src/xterm.rs` `x_draw_glyph_string_foreground` shapes).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;                                                     // C 1443
  let mut x: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX           // C 1447
    && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int; }   // C 1448
  else
    { x = (*s).x; }                                                     // C 1450

  /* Draw characters of S as rectangles if S's font could not be
     loaded.  */
  if glyph_string_bits! (font_not_found_p, (*s))                        // C 1454
    {
      i = 0;                                                            // C 1456
      while i < (*s).nchars
	{
	  let g: *mut glyph = (*s).first_glyph.offset (i as isize);      // C 1458
	  pgtk_draw_rectangle ((*s).f,                                   // C 1459
			       (*((*s).face as *mut face)).foreground, x, (*s).y,   // C 1460
			       (*g).pixel_width as c_int - 1,               // C 1461
			       (*s).height - 1,
			       false);                                      // C 1462
	  x += (*g).pixel_width as c_int;                               // C 1463
	  i += 1;                                                       // C 1456's step
	}
    }
  else
    {
      let font: *mut font = (*s).font;                                  // C 1468
      let mut boff: c_int = (*font).baseline_offset;                    // C 1469
      let mut y: c_int;                                                 // C 1470

      if (*font).vertical_centering                                     // C 1472
	{ boff = VCENTER_BASELINE_OFFSET! (font, (*s).f) - boff; }          // C 1473

      y = (*s).ybase - boff;                                            // C 1475
      if glyph_string_bits! (for_overlaps, (*s)) != 0                   // C 1476 (C's 3-bit field)
	  || (glyph_string_bits! (background_filled_p, (*s)) && (*s).hl != DRAW_CURSOR)
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, false); }                          // C 1477
      else
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, true); }                           // C 1479
      if face_bits! (overstrike, (*s).face as *mut face)               // C 1480
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x + 1, y, false); }                      // C 1481
    }
}

/* C 597-598's `bool_bf automatic : 1; unsigned id : 31;` are the `glyph.u.cmp` members C 1513
   and the composition readers below name; the constants are `include/dispextern.rs` 454-455's and
   the macro is this unit's own copy (the `src/xterm.rs` 9596 and `src/w32term.rs` 1732 shape), with
   the call sites passing the **pointer** (`$g:expr` is a value context - the reason written in
   Deviation 21).  */
#[cfg (feature = "have_pgtk")]
macro_rules! glyph_u_cmp_bits {
  (automatic, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_AUTOMATIC) != 0) };
  (id, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_ID_SHIFT) as ptrdiff_t) };
}

/* Draw the foreground of composite glyph string S.  */

/// C 1486-1572: `static void pgtk_draw_composite_glyph_string_foreground (struct glyph_string
/// *s)` - `face_bits!`/`glyph_bits!`/`glyph_u_cmp_bits!` for C's face-bit, glyph-bit and
/// `u.cmp.automatic` reads, `composition`'s two `offsets` reads through the pointer offset (the
/// `src/xterm.rs` shape), `COMPOSITION_GLYPH` for the TAB test, and `LGSTRING_GLYPH`/`LGLYPH_*`
/// from the composite mirror with C's conversions written out.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_composite_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;                                                     // C 1489
  let mut j: c_int;
  let mut x: c_int;
  let font: *mut font = (*s).font;                                      // C 1490

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if !(*s).face.is_null () && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX   // C 1494
    && glyph_bits! (left_box_line_p, (*(*s).first_glyph))               // C 1495
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int; }   // C 1496
  else
    { x = (*s).x; }                                                     // C 1498

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */
  if glyph_string_bits! (font_not_found_p, (*s))                        // C 1507
    {
      if (*s).cmp_from == 0                                             // C 1509
	{ pgtk_draw_rectangle ((*s).f, (*((*s).face as *mut face)).foreground, x, (*s).y,   // C 1510
			       (*s).width - 1, (*s).height - 1, false); }         // C 1511
    }
  else if !glyph_u_cmp_bits! (automatic, (*s).first_glyph)              // C 1513
    {
      let y: c_int = (*s).ybase;                                        // C 1515

      i = 0;                                                            // C 1517
      j = (*s).cmp_from;
      while i < (*s).nchars
	{
	  /* TAB in a composition means display glyphs with padding
	     space on the left or right.  */
	  if COMPOSITION_GLYPH (((*s).cmp as *mut composition), j as ptrdiff_t) != b'\t' as isize   // C 1520
	    {
	      let cmp: *mut composition = (*s).cmp as *mut composition;
	      let xx: c_int
		= x + *(*cmp).offsets.offset ((j * 2) as isize) as c_int;   // C 1522
	      let yy: c_int
		= y - *(*cmp).offsets.offset ((j * 2 + 1) as isize) as c_int;   // C 1523

	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, j, j + 1, xx, yy, false);                           // C 1525
	      if face_bits! (overstrike, (*s).face as *mut face)        // C 1526
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, j + 1, xx + 1, yy, false); }                 // C 1527
	    }
	  i += 1; j += 1;                                               // C 1517's step
	}
    }
  else
    {
      let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 1532
      let mut glyph: Lisp_Object;                                       // C 1533
      let y: c_int = (*s).ybase;                                        // C 1534
      let mut width: c_int = 0;                                         // C 1535

      i = (*s).cmp_from;                                                // C 1537
      j = (*s).cmp_from;
      while i < (*s).cmp_to
	{
	  glyph = LGSTRING_GLYPH (gstring, i as ptrdiff_t);            // C 1539
	  if NILP (LGLYPH_ADJUSTMENT (glyph))                           // C 1540
	    { width += LGLYPH_WIDTH (glyph) as c_int; }                 // C 1541
	  else
	    {
	      let mut xoff: c_int;                                      // C 1544
	      let mut yoff: c_int;
	      let mut wadjust: c_int;

	      if j < i                                                  // C 1546
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, i, x, y, false);                             // C 1548
		  if face_bits! (overstrike, (*s).face as *mut face)    // C 1549
		    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
			(s, j, i, x + 1, y, false); }                   // C 1550
		  x += width;                                           // C 1551
		}
	      xoff = LGLYPH_XOFF (glyph) as c_int;                      // C 1553
	      yoff = LGLYPH_YOFF (glyph) as c_int;                      // C 1554
	      wadjust = LGLYPH_WADJUST (glyph) as c_int;                // C 1555
	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, i, i + 1, x + xoff, y + yoff, false);               // C 1556
	      if face_bits! (overstrike, (*s).face as *mut face)        // C 1557
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, i, i + 1, x + xoff + 1, y + yoff, false); }     // C 1558-1559
	      x += wadjust;                                             // C 1560
	      j = i + 1;                                                // C 1561
	      width = 0;                                                // C 1562
	    }
	  i += 1;                                                       // C 1537's step
	}
      if j < i                                                          // C 1565
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, j, i, x, y, false);                                     // C 1567
	  if face_bits! (overstrike, (*s).face as *mut face)                // C 1568
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, j, i, x + 1, y, false); }                           // C 1569
	}
    }
}

/* C 2347-2365's `enum glyphless_display_method { GLYPHLESS_DISPLAY_THIN_SPACE,
   GLYPHLESS_DISPLAY_EMPTY_BOX, GLYPHLESS_DISPLAY_ACRONYM, GLYPHLESS_DISPLAY_HEX_CODE };` - C's enum
   ABI is `c_int` (PORTING.md 1.3), and the four names are `include/dispextern.rs` 3189-3197's
   inside that mirror's unit-names module, which a `use` cannot reach from here, so this unit
   carries its own copy (the `src/term.rs` 288-292 and `src/xdisp.rs` 1188-1191 shape).  */
#[cfg (feature = "have_pgtk")]
pub type glyphless_display_method = c_int;
#[cfg (feature = "have_pgtk")]
pub const GLYPHLESS_DISPLAY_THIN_SPACE: glyphless_display_method = 0;
#[cfg (feature = "have_pgtk")]
pub const GLYPHLESS_DISPLAY_EMPTY_BOX: glyphless_display_method = 1;
#[cfg (feature = "have_pgtk")]
pub const GLYPHLESS_DISPLAY_ACRONYM: glyphless_display_method = 2;
#[cfg (feature = "have_pgtk")]
pub const GLYPHLESS_DISPLAY_HEX_CODE: glyphless_display_method = 3;

/* C 590-638's `u.glyphless` members (`unsigned ch : 25; bool_bf for_no_font : 1; unsigned len : 4;
   enum glyphless_display_method method : 2;`) read through the mirror's `bits`
   (`include/dispextern.rs` 478-481's masks) - the `src/xterm.rs` 9596-class per-unit macro, with
   the pointer call form of Deviations 21/24.  */
#[cfg (feature = "have_pgtk")]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/* Draw the foreground of glyph string S for glyphless characters.  */

/// C 1576-1662: `static void pgtk_draw_glyphless_glyph_string_foreground (struct glyph_string
/// *s)` - `glyphless_bits!` for C's `glyph->u.glyphless` members, the `char2b` local array whose
/// address C hands to `s->char2b` (and clears again at the end), and C 1595-1601's `#ifdef
/// GCC_LINT` pair carried as two `const`s (P-51; the frozen table defines `GCC_LINT`, conf_post.h
/// 486, so the first arm is the compiled text and the `#else` arm is under `not gcc_lint` - the
/// `src/xterm.rs` 9910-9913 shape).  C's two `short` members take the port's written-out
/// promotions, as in Deviation 22.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_glyphless_glyph_string_foreground (s: *mut glyph_string)
{
  let mut glyph: *mut glyph = (*s).first_glyph;                         // C 1579
  let mut char2b: [c_uint; 8] = [0; 8];                                 // C 1580
  let mut x: c_int;                                                     // C 1581
  let mut i: c_int;
  let mut j: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if !(*s).face.is_null () && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX   // C 1585
    && glyph_bits! (left_box_line_p, (*(*s).first_glyph))               // C 1586
    { x = (*s).x + max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int; }   // C 1587
  else
    { x = (*s).x; }                                                     // C 1589

  (*s).char2b = char2b.as_mut_ptr ();                                   // C 1591

  i = 0;                                                                // C 1593
  while i < (*s).nchars
    {
      /* C's `#ifdef GCC_LINT` pair (C 1595-1601).  */
      #[cfg (feature = "gcc_lint")]
      const PACIFY_GCC_BUG_81401: c_int = 1;
      #[cfg (not (feature = "gcc_lint"))]
      const PACIFY_GCC_BUG_81401: c_int = 0;
      let mut buf: [c_char; 7 + PACIFY_GCC_BUG_81401 as usize]
	= [0; 7 + PACIFY_GCC_BUG_81401 as usize];                       // C 1602
      let mut str: *mut c_char = std::ptr::null_mut ();                 // C 1603
      let len: c_int = glyphless_bits! (len, glyph);                    // C 1604

      if glyphless_bits! (method, glyph) == GLYPHLESS_DISPLAY_ACRONYM   // C 1606
	{
	  if len > 0 && CHAR_TABLE_P (Vglyphless_char_display)          // C 1608-1609
	    && (CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display)) >= 1)   // C 1611-1612
	    {
	      let mut acronym: Lisp_Object
		= if !glyphless_bits! (for_no_font, glyph)              // C 1615
		  { CHAR_TABLE_REF (Vglyphless_char_display, glyphless_bits! (ch, glyph) as isize) }   // C 1616-1617
		  else { (*(XCHAR_TABLE (Vglyphless_char_display))).extras[0] };   // C 1618
	      if CONSP (acronym)                                        // C 1619
		{ acronym = XCAR (acronym); }                           // C 1620
	      if STRINGP (acronym)                                      // C 1621
		{ str = SSDATA (acronym); }                             // C 1622
	    }
	}
      else if glyphless_bits! (method, glyph) == GLYPHLESS_DISPLAY_HEX_CODE   // C 1625
	{
	  let ch: c_uint = glyphless_bits! (ch, glyph) as c_uint;       // C 1627
	  eassume! (ch as c_int <= MAX_CHAR);                           // C 1628
	  sprintf (buf.as_mut_ptr (), c"%0*X".as_ptr (),                // C 1629
		   if ch < 0x10000 { 4 } else { 6 }, ch);
	  str = buf.as_mut_ptr ();                                      // C 1630
	}

      if !str.is_null ()                                                // C 1633
	{
	  let upper_len: c_int = (len + 1) / 2;                         // C 1635

	  /* It is assured that all LEN characters in STR is ASCII.  */
	  j = 0;                                                        // C 1638
	  while j < len
	    {
	      char2b[j as usize]
		= ((*((*(*s).font).driver as *const font_driver)).encode_char.unwrap ())
		    ((*s).font, *str.offset (j as isize) as c_int) & 0xFFFF;   // C 1639-1640
	      j += 1;                                                   // C 1638's step
	    }
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, upper_len,                                           // C 1641
	     x + (*glyph).slice.glyphless.upper_xoff as c_int,          // C 1642
	     (*s).ybase + (*glyph).slice.glyphless.upper_yoff as c_int, // C 1643
	     false);
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, upper_len, len,                                         // C 1645
	     x + (*glyph).slice.glyphless.lower_xoff as c_int,          // C 1646
	     (*s).ybase + (*glyph).slice.glyphless.lower_yoff as c_int, // C 1647
	     false);
	}
      if glyphless_bits! (method, glyph) != GLYPHLESS_DISPLAY_THIN_SPACE   // C 1650
	{ pgtk_draw_rectangle ((*s).f, (*((*s).face as *mut face)).foreground,   // C 1651
			       x, (*s).ybase - (*glyph).ascent as c_int,    // C 1652
			       (*glyph).pixel_width as c_int - 1,           // C 1653
			       (*glyph).ascent as c_int + (*glyph).descent as c_int - 1,   // C 1654
			       false); }                                    // C 1655
      x += (*glyph).pixel_width as c_int;                               // C 1656
      i += 1; glyph = glyph.offset (1);                                 // C 1593's step
    }

  /* Pacify GCC 12 even though s->char2b is not used after this
     function returns.  */
  (*s).char2b = std::ptr::null_mut ();                                  // C 1661
}

/* Brightness beyond which a color won't have its highlight brightness
   boosted.

   Nominally, highlight colors for `3d' faces are calculated by
   brightening an object's color by a constant scale factor, but this
   doesn't yield good results for dark colors, so for colors who's
   brightness is less than this value (on a scale of 0-65535) have an
   use an additional additive factor.

   The value here is set so that the default menu-bar/mode-line color
   (grey75) will not have its highlights changed at all.  */
/// C 1675: `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` - a value, so the port writes the
/// constant the extractor reads it as (the `src/xterm.rs` 10773 shape; xterm.c 9568-9580 keeps the
/// same 48000 with the same comment).
#[cfg (feature = "have_pgtk")]
pub const HIGHLIGHT_COLOR_DARK_BOOST_LIMIT: c_int = 48000;

/* Compute a color which is lighter or darker than *PIXEL by FACTOR or
   DELTA.  Try a color with RGB values multiplied by FACTOR first.  If
   this produces the same color as PIXEL, try a color where all RGB
   values have DELTA added.  Return the computed color in *PIXEL.  F
   is the frame to act on.  */
/// C 1684-1746: `static void pgtk_compute_lighter_color (struct frame *f, unsigned long *pixel,
/// double factor, int delta)` - C's `Emacs_Color color, new;`, its `long bright`, the two
/// `new.pixel` folds and the closing store.  C's three `min (0xffff, factor * color.<channel>)`
/// calls are the f64 -> `isize` truncation the sibling lane writes (`src/xterm.rs` 10795-10797,
/// where C's `min`/`max` are lisp.h's `EMACS_INT` pair, so both operands and the result are
/// spelled out); `bright` is C's `int` arithmetic widened to `c_long`; each fold's `unsigned
/// short` channel takes the `c_int` promotion C's `>>`/`<<` implies - no sibling has the fold
/// (`w32term.c` 1626's `w32_alloc_lighter_color` builds `PALETTERGB (...)` and `xterm.rs`'s
/// allocator calls `x_alloc_nearest_color`), so the spelling is this unit's and `src/data.rs`
/// 2837-2908 is the tree's `<< 16` idiom.  C's comments are carried verbatim.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_compute_lighter_color (f: *mut frame, pixel: *mut c_ulong, factor: c_double,
					      delta: c_int)
{
  let mut color: Emacs_Color = core::mem::zeroed ();                     // C 1688
  let mut new: Emacs_Color = core::mem::zeroed ();                       // C 1688
  let mut bright: c_long;                                                // C 1689

  /* Get RGB color values.  */
  color.pixel = *pixel;                                                  // C 1692
  pgtk_query_color (f, &raw mut color);                                  // C 1693

  /* Change RGB values by specified FACTOR.  Avoid overflow!  */
  eassert! (factor >= 0.0);                                              // C 1696
  new.red = min (0xffff, (factor * color.red as c_double) as isize) as c_ushort;      // C 1697
  new.green = min (0xffff, (factor * color.green as c_double) as isize) as c_ushort;  // C 1698
  new.blue = min (0xffff, (factor * color.blue as c_double) as isize) as c_ushort;    // C 1699

  /* Calculate brightness of COLOR.  */
  bright = ((2 * color.red as c_int + 3 * color.green as c_int           // C 1702
		     + color.blue as c_int) / 6) as c_long;

  /* We only boost colors that are darker than
     HIGHLIGHT_COLOR_DARK_BOOST_LIMIT.  */
  if bright < HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_long                 // C 1706
    /* Make an additive adjustment to NEW, because it's dark enough so
       that scaling by FACTOR alone isn't enough.  */
    {
      /* How far below the limit this color is (0 - 1, 1 being darker).  */
      let dimness: c_double = 1.0 - bright as c_double                   // C 1711
			/ HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_double;
      /* The additive adjustment.  */
      let min_delta: c_int = (delta as c_double * dimness * factor / 2.0) as c_int;  // C 1713

      if factor < 1.0                                                    // C 1715
		{
		  new.red = max (0, new.red as isize - min_delta as isize) as c_ushort;    // C 1717
		  new.green = max (0, new.green as isize - min_delta as isize) as c_ushort;   // C 1718
		  new.blue = max (0, new.blue as isize - min_delta as isize) as c_ushort;     // C 1719
		}
      else
		{
		  new.red = min (0xffff, min_delta as isize + new.red as isize) as c_ushort;   // C 1723
		  new.green = min (0xffff, min_delta as isize + new.green as isize) as c_ushort;  // C 1724
		  new.blue = min (0xffff, min_delta as isize + new.blue as isize) as c_ushort;    // C 1725
		}
    }

  new.pixel = (new.red as c_int >> 8 << 16                               // C 1729
		       | new.green as c_int >> 8 << 8
		       | new.blue as c_int >> 8) as c_ulong;

  if new.pixel == *pixel                                                 // C 1733
    {
      /* If we end up with the same color as before, try adding
	 delta to the RGB values.  */
      new.red = min (0xffff, delta as isize + color.red as isize) as c_ushort;     // C 1737
      new.green = min (0xffff, delta as isize + color.green as isize) as c_ushort; // C 1738
      new.blue = min (0xffff, delta as isize + color.blue as isize) as c_ushort;   // C 1739
      new.pixel = (new.red as c_int >> 8 << 16                           // C 1740
			   | new.green as c_int >> 8 << 8
			   | new.blue as c_int >> 8) as c_ulong;
    }

  *pixel = new.pixel;                                                    // C 1745
}

/* C: cairo.h 906-907: `cairo_public void cairo_move_to (cairo_t *cr, double x, double y);` - the
   C 1756 and C 1797 calls (this machine's cairo header; `src/image.rs` 470-473 carries the same
   row).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_move_to (cr: *mut cairo_t, x: c_double, y: c_double);
}
/* C: cairo.h 912-913: `cairo_public void cairo_line_to (cairo_t *cr, double x, double y);` - the
   C 1757-1759 calls.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_line_to (cr: *mut cairo_t, x: c_double, y: c_double);
}
/* C: cairo.h 921-925: `cairo_public void cairo_arc (cairo_t *cr, double xc, double yc, double
   radius, double angle1, double angle2);` - the C 1798 call.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_arc (cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double,
		    angle1: c_double, angle2: c_double);
}
/* C: cairo.h 995-996: `cairo_public void cairo_fill (cairo_t *cr);` - the C 1760 and C 1802 calls
   (`src/ftcrfont.rs` 287 carries the same row).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_fill (cr: *mut cairo_t);
}

/// C 1748-1762: `static void pgtk_fill_trapezoid_for_relief (struct frame *f, unsigned long
/// color, int x, int y, int width, int height, int top_p)` - `pgtk_begin_cr_clip`/
/// `pgtk_set_cr_source_with_color`/`pgtk_end_cr_clip` come from the mirror
/// (`include/pgtkterm.rs` 681-690), and C's `top_p ? x : x + height` is the port's `if top_p != 0`
/// with the two `cairo` coordinates widened to `c_double` (`src/xterm.rs` 7248-7261, the same C
/// text under `USE_CAIRO`).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_fill_trapezoid_for_relief (f: *mut frame, color: c_ulong, x: c_int, y: c_int,
						  width: c_int, height: c_int, top_p: c_int)
{
  let cr: *mut cairo_t;                                                  // C 1752

  cr = pgtk_begin_cr_clip (f);                                           // C 1754
  pgtk_set_cr_source_with_color (f, color, false);                       // C 1755
  cairo_move_to (cr, (if top_p != 0 { x } else { x + height }) as c_double,
			 y as c_double);                                      // C 1756
  cairo_line_to (cr, x as c_double, (y + height) as c_double);           // C 1757
  cairo_line_to (cr, (if top_p != 0 { x + width - height } else { x + width }) as c_double,  // C 1758
			 (y + height) as c_double);
  cairo_line_to (cr, (x + width) as c_double, y as c_double);            // C 1759
  cairo_fill (cr);                                                       // C 1760
  pgtk_end_cr_clip (f);                                                  // C 1761
}

/// C 1764-1771: `enum corners { CORNER_BOTTOM_RIGHT, CORNER_BOTTOM_LEFT, CORNER_TOP_LEFT,
/// CORNER_TOP_RIGHT, CORNER_LAST };` - C's enum lives in this file (not in a header), so it is
/// this unit's record (`src/xterm.rs` 7266-7272 is the same enum in the same position under
/// `USE_CAIRO`); C's own comments are carried.
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub enum corners
{
  CORNER_BOTTOM_RIGHT,		/* 0 -> pi/2 */
  CORNER_BOTTOM_LEFT,		/* pi/2 -> pi */
  CORNER_TOP_LEFT,		/* pi -> 3pi/2 */
  CORNER_TOP_RIGHT,		/* 3pi/2 -> 2pi */
  CORNER_LAST,
}

/// C 1773-1804: `static void pgtk_erase_corners_for_relief (struct frame *f, unsigned long
/// color, int x, int y, int width, int height, double radius, double margin, int corners)` -
/// C's `for (i = 0; i < CORNER_LAST; i++)` is the port's `while` with C's own step at the end of
/// the body, and C's two comma expressions (`xm = x - margin, xc = xm + radius;`) are one
/// statement per assignment in C's own left-to-right order (`src/xterm.rs` 7278-7327, Devia-
/// tion 26).  `x + width + margin` stays C's int addition, widened where C widens it.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_erase_corners_for_relief (f: *mut frame, color: c_ulong, x: c_int, y: c_int,
						 width: c_int, height: c_int, radius: c_double,
						 margin: c_double, corners: c_int)
{
  let cr: *mut cairo_t;                                                  // C 1778
  let mut i: c_int;                                                      // C 1779

  cr = pgtk_begin_cr_clip (f);                                           // C 1781
  pgtk_set_cr_source_with_color (f, color, false);                       // C 1782
  i = 0;                                                                 // C 1783
  while i < corners::CORNER_LAST as c_int
    {
      if corners & (1 << i) != 0                                         // C 1784
		{
		  let mut xm: c_double;
		  let mut ym: c_double;
		  let mut xc: c_double;
		  let mut yc: c_double;

		  if i == corners::CORNER_TOP_LEFT as c_int                   // C 1788
			  || i == corners::CORNER_BOTTOM_LEFT as c_int
		    {
		      xm = x as c_double - margin;
		      xc = xm + radius;
		    }
		  else
		    {
		      xm = (x + width) as c_double + margin;
		      xc = xm - radius;
		    }
		  if i == corners::CORNER_TOP_LEFT as c_int                   // C 1792
			  || i == corners::CORNER_TOP_RIGHT as c_int
		    {
		      ym = y as c_double - margin;
		      yc = ym + radius;
		    }
		  else
		    {
		      ym = (y + height) as c_double + margin;
		      yc = ym - radius;
		    }

		  cairo_move_to (cr, xm, ym);                                 // C 1797
		  cairo_arc (cr, xc, yc, radius, i as c_double * M_PI_2,      // C 1798
				 (i + 1) as c_double * M_PI_2);
		}
      i += 1;                                                           // C 1783's step
    }
  cairo_clip (cr);                                                       // C 1800
  cairo_rectangle (cr, x as c_double, y as c_double, width as c_double,
			               height as c_double);                        // C 1801
  cairo_fill (cr);                                                       // C 1802
  pgtk_end_cr_clip (f);                                                  // C 1803
}

/* C 1806-1822: `static void pgtk_setup_relief_color (struct frame *f, struct relief *relief,
   double factor, int delta, unsigned long default_pixel)`.  C's chained
   `xgcv.foreground = relief->pixel = pixel;` is one statement per assignment in C's right-to-left
   order (the `src/w32term.rs` 2676-2678 reading of the same C text).  */
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_setup_relief_color (f: *mut frame, relief: *mut relief, factor: c_double,
					   delta: c_int, default_pixel: c_ulong)
{
  let mut xgcv: Emacs_GC = core::mem::zeroed ();                        // C 1810
  let di: *mut pgtk_output = FRAME_X_OUTPUT! (f);                       // C 1811
  let mut pixel: c_ulong;                                               // C 1812
  let background: c_ulong = (*di).relief_background;                    // C 1813

  /* Allocate new color.  */
  xgcv.foreground = default_pixel;                                      // C 1816
  pixel = background;                                                   // C 1817
  pgtk_compute_lighter_color (f, &raw mut pixel, factor, delta);        // C 1818
  (*relief).pixel = pixel;                                              // C 1819
  xgcv.foreground = (*relief).pixel;

  (*relief).xgcv = xgcv;                                                // C 1821
}

/// C: dispextern.h 3760-3762: `#if defined HAVE_X_WINDOWS || defined USE_CAIRO || defined
/// HAVE_NS || defined HAVE_HAIKU || defined HAVE_ANDROID` / `#define RGB_PIXEL_COLOR unsigned
/// long` - the arm this configuration takes (the pgtk build defines `USE_CAIRO`).  No mirror
/// carries it, so the unit declares it at the use site, the `src/image.rs` 2210-2215 and
/// `src/w32term.rs` 1775-1779 copies of the same pair.
#[cfg (feature = "have_pgtk")]
pub type RGB_PIXEL_COLOR = c_ulong;

/* `image_background` and `image_background_transparent` are dispextern.h 3769-3772's two
   prototypes (`src/image.rs` 2977/3026 define them; P-72 rule 4's use-site declaration).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn image_background (image: *mut image, f: *mut frame, img: Emacs_Pix_Context)
    -> RGB_PIXEL_COLOR;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn image_background_transparent (image: *mut image, f: *mut frame, mask: Emacs_Pix_Context)
    -> c_int;
}

/// C: dispextern.h 3860-3872's `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT` - the mirror
/// keeps the image record's masks but not the two readers, so each is defined here at the use
/// site (the `src/image.rs` 2241-2260, `src/w32term.rs` 1781-1806 and `src/haikuterm.rs` copies
/// of the same pair).  C 1835/1836's third argument is the `0` the call sites below pass.
/// C: #define IMAGE_BACKGROUND(img, f, ximg) ((img)->background_valid ? (img)->background : image_background (img, f, ximg))
/// C: #define IMAGE_BACKGROUND_TRANSPARENT(img, f, mask) ((img)->background_transparent_valid ? (img)->background_transparent : image_background_transparent (img, f, mask))
#[cfg (feature = "have_pgtk")]
macro_rules! IMAGE_BACKGROUND {
  ($img:expr, $f:expr, $ximg:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_VALID_MASK != 0
	 { (*$img).background }
       else
	 { image_background ($img, $f, $ximg) })
    };
}
#[cfg (feature = "have_pgtk")]
macro_rules! IMAGE_BACKGROUND_TRANSPARENT {
  ($img:expr, $f:expr, $mask:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK != 0
	 { ((*$img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int }
       else
	 { image_background_transparent ($img, $f, $mask) })
    };
}

/* Set up colors for the relief lines around glyph string S.  */

/// C 1824-1853: `static void pgtk_setup_relief_colors (struct glyph_string *s)` - the three-arm
/// colour choice (`face_bits!`'s `use_box_color_for_shadows_p` arm is added with this slice,
/// Deviation 29) and C's `relief_background_valid_p` test, which the mirror's `pgtk_output`
/// carries in its P-80 ① storage unit (`pgtk_output_bits!`, Deviation 28).  `s->img->pixmap`'s
/// truth test and the two image macros are C's own tests.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_setup_relief_colors (s: *mut glyph_string)
{
  let di: *mut pgtk_output = FRAME_X_OUTPUT! ((*s).f);                  // C 1828
  let mut color: c_ulong;                                               // C 1829

  if face_bits! (use_box_color_for_shadows_p, (*s).face as *mut face)   // C 1831
    { color = (*((*s).face as *mut face)).box_color; }
  else if glyph_bits! (type, (*(*s).first_glyph)) == IMAGE_GLYPH        // C 1833
		   && !(*(*s).img).pixmap.is_null ()                          // C 1834
		   && IMAGE_BACKGROUND_TRANSPARENT! ((*s).img, (*s).f, 0 as Emacs_Pix_Context) == 0   // C 1835
    { color = IMAGE_BACKGROUND! ((*s).img, (*s).f, 0 as Emacs_Pix_Context); }                   // C 1836
  else
    {
      /* Get the background color of the face.  */
      color = (*s).xgcv.background;                                     // C 1840
    }

  if !pgtk_output_bits! (relief_background_valid_p, di)                 // C 1843
      || (*di).relief_background != color                               // C 1844
    {
      pgtk_output_bits! (relief_background_valid_p, di, true);          // C 1846
      (*di).relief_background = color;                                  // C 1847
      pgtk_setup_relief_color ((*s).f, &raw mut (*di).white_relief, 1.2, 0x8000,       // C 1848
				       WHITE_PIX_DEFAULT! ((*s).f));            // C 1849
      pgtk_setup_relief_color ((*s).f, &raw mut (*di).black_relief, 0.6, 0x4000,       // C 1850
				       BLACK_PIX_DEFAULT! ((*s).f));            // C 1851
    }
}

/// C 1855-1866: `static void pgtk_set_clip_rectangles (struct frame *f, cairo_t *cr, XRectangle
/// *rectangles, int n)` - `f` is unused in C's body (`let _ =` keeps the name, the
/// `pgtk_set_z_group` shape), C's `for (int i = 0; i < n; i++)` declares its index in the header
/// and is the port's init/`while`/step (Deviation 26's spelling), and the four `short` members
/// widen to `cairo_rectangle`'s `double`s where C converts them.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_clip_rectangles (f: *mut frame, cr: *mut cairo_t, rectangles: *mut XRectangle,
					    n: c_int)
{
  let _ = f;

  if n > 0                                                              // C 1859
    {
      let mut i: c_int = 0;                                             // C 1861
      while i < n
	{
	  cairo_rectangle (cr, (*rectangles.offset (i as isize)).x as c_double,     // C 1862
				   (*rectangles.offset (i as isize)).y as c_double,
				   (*rectangles.offset (i as isize)).width as c_double, // C 1863
				   (*rectangles.offset (i as isize)).height as c_double);
	  i += 1;
	}
      cairo_clip (cr);                                                  // C 1864
    }
}

/* Draw a relief on frame F inside the rectangle given by LEFT_X,
   TOP_Y, RIGHT_X, and BOTTOM_Y.  WIDTH is the thickness of the relief
   to draw, it must be >= 0.  RAISED_P means draw a raised
   relief.  LEFT_P means draw a relief on the left side of
   the rectangle.  RIGHT_P means draw a relief on the right
   side of the rectangle.  CLIP_RECT is the clipping rectangle to use
   when drawing.  */

/// C 1876-1951: `static void pgtk_draw_relief_rect (...)` - C's six flags are Rust's `bool` (each
/// test is C's own), the eight `pgtk_fill_rectangle` calls keep C's callee (the definition is
/// C 5035's, in the remainder: the class the ledger's Evidence ② records), the two
/// `pgtk_fill_trapezoid_for_relief` and the `pgtk_erase_corners_for_relief` calls are this unit's
/// own definitions above, the `corners |=` writes are `corners::CORNER_* as c_int` (the
/// `src/xterm.rs` 11036-11047 shape), and C 1948's `6, 1` are the `6 as c_double,
/// 1 as c_double` the sibling unit writes (`src/xterm.rs` 11090-11092).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_relief_rect (f: *mut frame, left_x: c_int, top_y: c_int, right_x: c_int,
					 bottom_y: c_int, hwidth: c_int, vwidth: c_int, raised_p: bool,
					 top_p: bool, bot_p: bool, left_p: bool, right_p: bool,
					 clip_rect: *mut XRectangle)
{
  let mut top_left_color: c_ulong;                                      // C 1883
  let mut bottom_right_color: c_ulong;
  let mut corners: c_int = 0;                                           // C 1884

  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 1886

  if raised_p                                                           // C 1888
    {
      top_left_color = (*FRAME_X_OUTPUT! (f)).white_relief.xgcv.foreground;      // C 1890
      bottom_right_color = (*FRAME_X_OUTPUT! (f)).black_relief.xgcv.foreground;  // C 1891
    }
  else
    {
      top_left_color = (*FRAME_X_OUTPUT! (f)).black_relief.xgcv.foreground;      // C 1895
      bottom_right_color = (*FRAME_X_OUTPUT! (f)).white_relief.xgcv.foreground;  // C 1896
    }

  pgtk_set_clip_rectangles (f, cr, clip_rect, 1);                       // C 1899

  if left_p                                                             // C 1901
    {
      pgtk_fill_rectangle (f, top_left_color, left_x, top_y,            // C 1903
			   vwidth, bottom_y + 1 - top_y, false);                // C 1904
      if top_p
	{ corners |= 1 << (corners::CORNER_TOP_LEFT as c_int); }           // C 1906
      if bot_p
	{ corners |= 1 << (corners::CORNER_BOTTOM_LEFT as c_int); }        // C 1908
    }
  if right_p                                                            // C 1910
    {
      pgtk_fill_rectangle (f, bottom_right_color, right_x + 1 - vwidth, top_y,        // C 1912
			   vwidth, bottom_y + 1 - top_y, false);                // C 1913
      if top_p
	{ corners |= 1 << (corners::CORNER_TOP_RIGHT as c_int); }          // C 1915
      if bot_p
	{ corners |= 1 << (corners::CORNER_BOTTOM_RIGHT as c_int); }       // C 1917
    }
  if top_p                                                              // C 1919
    {
      if !right_p
	{ pgtk_fill_rectangle (f, top_left_color, left_x, top_y,             // C 1922
				     right_x + 1 - left_x, hwidth, false); }  // C 1923
      else
	{ pgtk_fill_trapezoid_for_relief (f, top_left_color, left_x, top_y,  // C 1925
						right_x + 1 - left_x, hwidth, 1); }  // C 1926
    }
  if bot_p                                                              // C 1928
    {
      if !left_p
	{ pgtk_fill_rectangle (f, bottom_right_color, left_x,                // C 1931
				     bottom_y + 1 - hwidth, right_x + 1 - left_x,   // C 1932
				     hwidth, false); }  // C 1933
      else
	{ pgtk_fill_trapezoid_for_relief (f, bottom_right_color,             // C 1935
						left_x, bottom_y + 1 - hwidth,      // C 1936
						right_x + 1 - left_x, hwidth, 0); }  // C 1937
    }
  if left_p && vwidth > 1                                               // C 1939
    { pgtk_fill_rectangle (f, bottom_right_color, left_x, top_y,        // C 1940
				   1, bottom_y + 1 - top_y, false); }           // C 1941
  if top_p && hwidth > 1                                                // C 1942
    { pgtk_fill_rectangle (f, bottom_right_color, left_x, top_y,        // C 1943
				   right_x + 1 - left_x, 1, false); }           // C 1944
  if corners != 0                                                       // C 1945
    { pgtk_erase_corners_for_relief (f, FRAME_BACKGROUND_PIXEL (f), left_x,         // C 1946
					     top_y, right_x - left_x + 1,               // C 1947
					     bottom_y - top_y + 1, 6 as c_double,       // C 1948
					     1 as c_double, corners); }

  pgtk_end_cr_clip (f);                                                 // C 1950
}

/* Draw a box on frame F inside the rectangle given by LEFT_X, TOP_Y,
   RIGHT_X, and BOTTOM_Y.  WIDTH is the thickness of the lines to
   draw, it must be >= 0.  LEFT_P means draw a line on the
   left side of the rectangle.  RIGHT_P means draw a line
   on the right side of the rectangle.  CLIP_RECT is the clipping
   rectangle to use when drawing.  */

/// C 1960-2000: `static void pgtk_draw_box_rect (struct glyph_string *s, int left_x, int top_y,
/// int right_x, int bottom_y, int hwidth, int vwidth, bool left_p, bool right_p, XRectangle *
/// clip_rect)` - the `foreground_backup` save/restore pair around C's four `pgtk_fill_rectangle`
/// calls (the definition is C 5035's, in the remainder: the class Evidence ② records),
/// `pgtk_set_clip_rectangles` from the slice above and C's own `/* Top.  */` comment lines.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_box_rect (s: *mut glyph_string, left_x: c_int, top_y: c_int, right_x: c_int,
				      bottom_y: c_int, hwidth: c_int, vwidth: c_int, left_p: bool,
				      right_p: bool, clip_rect: *mut XRectangle)
{
  let mut foreground_backup: c_ulong;                                   // C 1966

  let cr: *mut cairo_t = pgtk_begin_cr_clip ((*s).f);                   // C 1968

  foreground_backup = (*s).xgcv.foreground;                             // C 1970
  (*s).xgcv.foreground = (*((*s).face as *mut face)).box_color;         // C 1971

  pgtk_set_clip_rectangles ((*s).f, cr, clip_rect, 1);                  // C 1973

  /* Top.  */
  pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground,                    // C 1976
			       left_x, top_y, right_x - left_x + 1, hwidth,     // C 1977
			       false);                                          // C 1978

  /* Left.  */
  if left_p                                                             // C 1981
    { pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground,                // C 1982
				   left_x, top_y, vwidth, bottom_y - top_y + 1, // C 1983
				   false); }                                    // C 1984

  /* Bottom.  */
  pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground,                    // C 1987
			       left_x, bottom_y - hwidth + 1, right_x - left_x + 1,   // C 1988
			       hwidth, false);                                  // C 1989

  /* Right.  */
  if right_p                                                            // C 1992
    { pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground,                // C 1993
				   right_x - vwidth + 1, top_y, vwidth,         // C 1994
				   bottom_y - top_y + 1, false); }              // C 1995

  (*s).xgcv.foreground = foreground_backup;                             // C 1997

  pgtk_end_cr_clip ((*s).f);                                            // C 1999
}

/// C 2014: `!s->w->pseudo_window_p` - `window.h` 438's `bool_bf pseudo_window_p : 1`, bit 11 of
/// `window::bits` (`include/window.rs` 195 names the shift; that mirror's own reader
/// `WINDOW_PSEUDO_P` loses C's member token, so the unit keeps the member-name form - the
/// `src/w32term.rs` 937-945 copy of this same arm).
/// C: #define WINDOW_PSEUDO_P(w) ((w)->pseudo_window_p)
#[cfg (feature = "have_pgtk")]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr) => { ((*($w)).bits >> WINDOW_PSEUDO_WINDOW_P_SHIFT) & 1 != 0 };
  (phys_cursor_on_p, $w:expr, $v:expr) =>
    { (*($w)).bits = if $v { (*($w)).bits | (1 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }
                     else { (*($w)).bits & !(1 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) } };
}

/* C: dispextern.h 3581: `int window_box_right (struct window *, enum glyph_row_area);` - the
   C 2014 call (`src/xdisp.rs` defines it; no mirror carries it, so the unit declares it at the
   use site - the `src/w32term.rs` 1750-1760 declaration of the same prototype).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int;
}
/* C: dispextern.h 3663: `extern void get_glyph_string_clip_rect (struct glyph_string *s,
   NativeRectangle *r);` - the C 2036 call (`src/xdisp.rs` defines it; the mirror's copy spells
   the second parameter `*mut c_void`, so the unit declares C's `XRectangle` shape - the
   `src/w32term.rs` 966-968 reading of the same call).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn get_glyph_string_clip_rect (s: *mut glyph_string, r: *mut XRectangle);
}
/* C: cairo.h 944-945: `cairo_public void cairo_rel_line_to (cairo_t *cr, double dx, double dy);`
   - the C 2083 call.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_rel_line_to (cr: *mut cairo_t, dx: c_double, dy: c_double);
}

/* Draw a box around glyph string S.  */

/// C 2005-2048: `static void pgtk_draw_glyph_string_box (struct glyph_string *s)` -
/// `full_width_p` is `glyph_row_bits!`'s member, `pseudo_window_p` the unit's own
/// `window_bits!` arm above, `WINDOW_RIGHT_EDGE_X!` is `window.rs`'s macro and
/// `window_box_right` its use-site declaration, `eabs!` is `lisp.rs`'s macro, C's
/// `min (last_x, s->x + s->background_width)` is the port's `isize` fold (Deviation 22's class),
/// and `s->prev`/`s->next` are the mirror's pointers, so C's `NULL` tests are `.is_null ()`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_glyph_string_box (s: *mut glyph_string)
{
  let mut hwidth: c_int;                                                // C 2008
  let mut vwidth: c_int;
  let mut left_x: c_int;
  let mut right_x: c_int;
  let mut top_y: c_int;
  let mut bottom_y: c_int;
  let mut last_x: c_int;
  let mut raised_p: bool;                                               // C 2009
  let mut left_p: bool;
  let mut right_p: bool;
  let mut last_glyph: *mut glyph;                                       // C 2010
  let mut clip_rect: XRectangle = core::mem::zeroed ();                 // C 2011

  last_x = (if glyph_row_bits! (full_width_p, (*s).row)                 // C 2013
		       && !window_bits! (pseudo_window_p, (*s).w)             // C 2014
		    { WINDOW_RIGHT_EDGE_X! ((*s).w) }
		    else { window_box_right ((*s).w, (*s).area) });

  /* The glyph that may have a right box line.  */
  last_glyph = (if !(*s).cmp.is_null () || !(*s).img.is_null ()         // C 2017
			{ (*s).first_glyph }                                   // C 2018
		      else { (*s).first_glyph.offset (((*s).nchars - 1) as isize) });

  vwidth = eabs! ((*((*s).face as *mut face)).box_vertical_line_width);         // C 2020
  hwidth = eabs! ((*((*s).face as *mut face)).box_horizontal_line_width);       // C 2021
  raised_p = face_bits! (box, (*s).face as *mut face) == FACE_RAISED_BOX;       // C 2022
  left_x = (*s).x;                                                      // C 2023
  right_x = (if glyph_row_bits! (full_width_p, (*s).row)                // C 2024
		      && glyph_string_bits! (extends_to_end_of_line_p, (*s))
		   { last_x - 1 }
		   else { min (last_x as isize,                                // C 2025
				 ((*s).x + (*s).background_width) as isize) as c_int - 1 });
  top_y = (*s).y;                                                       // C 2026
  bottom_y = top_y + (*s).height - 1;                                   // C 2027

  left_p = (glyph_bits! (left_box_line_p, (*(*s).first_glyph))          // C 2029
		 || ((*s).hl == DRAW_MOUSE_FACE                               // C 2030
			 && ((*s).prev.is_null () || (*(*s).prev).hl != (*s).hl)));
  right_p = (glyph_bits! (right_box_line_p, (*last_glyph))              // C 2032
		  || ((*s).hl == DRAW_MOUSE_FACE                              // C 2033
			  && ((*s).next.is_null () || (*(*s).next).hl != (*s).hl)));

  get_glyph_string_clip_rect (s, &raw mut clip_rect);                   // C 2036

  if face_bits! (box, (*s).face as *mut face) == FACE_SIMPLE_BOX        // C 2038
    { pgtk_draw_box_rect (s, left_x, top_y, right_x, bottom_y, hwidth,  // C 2039
				  vwidth, left_p, right_p, &raw mut clip_rect); }   // C 2040
  else
    {
      pgtk_setup_relief_colors (s);                                     // C 2043
      pgtk_draw_relief_rect ((*s).f, left_x, top_y, right_x, bottom_y, hwidth,   // C 2044
			     vwidth, raised_p, true, true, left_p, right_p,      // C 2045
			     &raw mut clip_rect);                                // C 2046
    }
}

/// C 2050-2089: `static void pgtk_draw_horizontal_wave (struct frame *f, unsigned long color,
/// int x, int y, int width, int height, int wave_length)` - C's two `double` locals are the
/// `as c_double` widenings of `wave_length`/`height - 1`, `y` is reassigned in the body (so the
/// parameter is `mut`, the `src/image.rs` 2902 `mut width` shape), C's `--n` is the port's
/// `n -= 1` at the head of the loop (the `src/xterm.rs` 7364-7370 body of the same C text), and
/// the six cairo calls are this unit's declarations plus the mirror's `cairo_set_line_width`/
/// `cairo_stroke` pair added with slice 4.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_horizontal_wave (f: *mut frame, color: c_ulong, x: c_int, mut y: c_int,
					     width: c_int, height: c_int, wave_length: c_int)
{
  let cr: *mut cairo_t;                                                 // C 2054
  let mut dx: c_double = wave_length as c_double;                       // C 2055
  let mut dy: c_double = (height - 1) as c_double;
  let mut xoffset: c_int;                                               // C 2056
  let mut n: c_int;

  cr = pgtk_begin_cr_clip (f);                                          // C 2058
  pgtk_set_cr_source_with_color (f, color, false);                      // C 2059
  cairo_rectangle (cr, x as c_double, y as c_double, width as c_double, height as c_double);   // C 2060
  cairo_clip (cr);                                                      // C 2061

  if x >= 0                                                             // C 2063
    {
      xoffset = x % (wave_length * 2);                                  // C 2065
      if xoffset == 0                                                   // C 2066
	{ xoffset = wave_length * 2; }
    }
  else
    { xoffset = x % (wave_length * 2) + wave_length * 2; }              // C 2070
  n = (width + xoffset) / wave_length + 1;                              // C 2071
  if xoffset > wave_length                                              // C 2072
    {
      xoffset -= wave_length;                                           // C 2074
      n -= 1;                                                           // C 2075
      y += height - 1;                                                  // C 2076
      dy = -dy;                                                         // C 2077
    }

  cairo_move_to (cr, (x - xoffset) as c_double + 0.5, y as c_double + 0.5);    // C 2080
  loop
    {
      n -= 1;                                                           // C 2081's `--n >= 0`
      if n < 0
	{ break; }
      cairo_rel_line_to (cr, dx, dy);                                   // C 2083
      dy = -dy;                                                         // C 2084
    }
  cairo_set_line_width (cr, 1.0);                                       // C 2086
  cairo_stroke (cr);                                                    // C 2087
  pgtk_end_cr_clip (f);                                                 // C 2088
}

/// C 2091-2098: `static void pgtk_draw_underwave (struct glyph_string *s, unsigned long color)` -
/// C's two `int` locals are the last two arguments the horizontal wave above takes.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_underwave (s: *mut glyph_string, color: c_ulong)
{
  let wave_height: c_int = 3;                                           // C 2094
  let wave_length: c_int = 2;

  pgtk_draw_horizontal_wave ((*s).f, color, (*s).x, (*s).ybase - wave_height + 3,   // C 2096
				     (*s).width, wave_height, wave_length);      // C 2097
}

/* Draw a relief around the image glyph string S.  */

/* C: dispextern.h 3774: `int image_ascent (struct image *, struct face *, struct glyph_slice *);`
   - the C 2110 call (`src/image.rs` defines it, `#[unsafe(no_mangle)]`); no mirror carries it, so
   the unit declares it at the use site, C's `&s->slice` being the port's `&raw mut` - the
   `src/w32term.rs` 3121-3129 declaration of the same prototype.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn image_ascent (img: *mut image, face: *mut face, slice: *mut glyph_slice) -> c_int;
}

/// C 2102-2190: `static void pgtk_draw_image_relief (struct glyph_string *s)` - `s->slice`'s four
/// members are `glyph_slice_bits!` reads (the record stores them in its two measured units), the
/// `DRAW_IMAGE_SUNKEN`/`DRAW_IMAGE_RAISED` fork keeps C's `tab_bar_button_relief` /
/// `tool_bar_button_relief` globals with C's `min (…, 1000000)` as the port's `isize` fold
/// (`include/lisp.rs` 3152, `EMACS_INT` is `isize` there), the two `V*_button_margin` chains are
/// C's `CONSP`/`FIXNUMP`/`XCAR`/`XCDR`/`XFIXNUM` spellings, and C's four comma expressions
/// (`x -= thick + extra_x, left_p = true;`) are one statement per assignment - Deviation 26's
/// spelling.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_image_relief (s: *mut glyph_string)
{
  let mut x1: c_int;                                                    // C 2105
  let mut y1: c_int;
  let mut thick: c_int;
  let mut raised_p: bool;                                               // C 2106
  let mut top_p: bool;
  let mut bot_p: bool;
  let mut left_p: bool;
  let mut right_p: bool;
  let mut extra_x: c_int;                                               // C 2107
  let mut extra_y: c_int;
  let mut r: XRectangle = core::mem::zeroed ();                         // C 2108
  let mut x: c_int = (*s).x;                                            // C 2109
  let mut y: c_int = (*s).ybase                                         // C 2110
    - image_ascent ((*s).img, (*s).face as *mut face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX            // C 2114
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 2115
      && glyph_slice_bits! (x, (*s).slice) == 0                         // C 2116
    { x += max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int; }   // C 2117

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0                             // C 2121
    { x += (*(*s).img).hmargin; }                                       // C 2122
  if glyph_slice_bits! (y, (*s).slice) == 0                             // C 2123
    { y += (*(*s).img).vmargin; }                                       // C 2124

  if (*s).hl == DRAW_IMAGE_SUNKEN                                       // C 2126
      || (*s).hl == DRAW_IMAGE_RAISED                                   // C 2127
    {
      if (*((*s).face as *mut face)).id == TAB_BAR_FACE_ID              // C 2129
	{ thick = (if tab_bar_button_relief < 0                            // C 2130
		     { DEFAULT_TAB_BAR_BUTTON_RELIEF }                        // C 2131
		     else { min (tab_bar_button_relief, 1000000) as c_int }); }   // C 2132
      else
	{ thick = (if tool_bar_button_relief < 0                           // C 2134
		     { DEFAULT_TOOL_BAR_BUTTON_RELIEF }                       // C 2135
		     else { min (tool_bar_button_relief, 1000000) as c_int }); }  // C 2136
      raised_p = (*s).hl == DRAW_IMAGE_RAISED;                          // C 2137
    }
  else
    {
      thick = eabs! ((*(*s).img).relief);                               // C 2141
      raised_p = (*(*s).img).relief > 0;                                // C 2142
    }

  x1 = x + glyph_slice_bits! (width, (*s).slice) as c_int - 1;          // C 2145
  y1 = y + glyph_slice_bits! (height, (*s).slice) as c_int - 1;         // C 2146

  extra_x = 0;                                                          // C 2148
  extra_y = 0;
  if (*((*s).face as *mut face)).id == TAB_BAR_FACE_ID                  // C 2149
    {
      if CONSP (Vtab_bar_button_margin)                                 // C 2151
		  && FIXNUMP (XCAR (Vtab_bar_button_margin))
		  && FIXNUMP (XCDR (Vtab_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtab_bar_button_margin)) as c_int - thick;      // C 2155
	  extra_y = XFIXNUM (XCDR (Vtab_bar_button_margin)) as c_int - thick;      // C 2156
	}
      else if FIXNUMP (Vtab_bar_button_margin)                          // C 2158
	{
	  extra_x = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;             // C 2159
	  extra_y = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;
	}
    }

  if (*((*s).face as *mut face)).id == TOOL_BAR_FACE_ID                 // C 2162
    {
      if CONSP (Vtool_bar_button_margin)                                // C 2164
		  && FIXNUMP (XCAR (Vtool_bar_button_margin))
		  && FIXNUMP (XCDR (Vtool_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtool_bar_button_margin)) as c_int;             // C 2168
	  extra_y = XFIXNUM (XCDR (Vtool_bar_button_margin)) as c_int;             // C 2169
	}
      else if FIXNUMP (Vtool_bar_button_margin)                         // C 2171
	{
	  extra_x = XFIXNUM (Vtool_bar_button_margin) as c_int;                    // C 2172
	  extra_y = XFIXNUM (Vtool_bar_button_margin) as c_int;
	}
    }

  top_p = false;                                                        // C 2175
  bot_p = false;
  left_p = false;
  right_p = false;

  if glyph_slice_bits! (x, (*s).slice) == 0                             // C 2177
	{ x -= thick + extra_x; left_p = true; }                           // C 2178
  if glyph_slice_bits! (y, (*s).slice) == 0                             // C 2179
	{ y -= thick + extra_y; top_p = true; }                            // C 2180
  if (glyph_slice_bits! (x, (*s).slice) as c_int                         // C 2181
	+ glyph_slice_bits! (width, (*s).slice) as c_int) == (*(*s).img).width
	{ x1 += thick + extra_x; right_p = true; }                         // C 2182
  if (glyph_slice_bits! (y, (*s).slice) as c_int                         // C 2183
	+ glyph_slice_bits! (height, (*s).slice) as c_int) == (*(*s).img).height
	{ y1 += thick + extra_y; bot_p = true; }                           // C 2184

  pgtk_setup_relief_colors (s);                                         // C 2186
  get_glyph_string_clip_rect (s, &raw mut r);                           // C 2187
  pgtk_draw_relief_rect ((*s).f, x, y, x1, y1, thick, thick, raised_p,  // C 2188
				 top_p, bot_p, left_p, right_p, &raw mut r);    // C 2189
}

/// C 2195-2203: `static void pgtk_draw_glyph_string_bg_rect (struct glyph_string *s, int x, int y,
/// int w, int h)` - C's `s->stippled_p` read is `glyph_string_bits!`'s member, and the two
/// branches call this unit's own `fill_background`/`pgtk_clear_glyph_string_rect` (the third and
/// fourth slices).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_glyph_string_bg_rect (s: *mut glyph_string, x: c_int, y: c_int, w: c_int,
						  h: c_int)
{
  if glyph_string_bits! (stippled_p, (*s))                              // C 2199
    { fill_background (s, x, y, w, h); }                                // C 2200
  else
    { pgtk_clear_glyph_string_rect (s, x, y, w, h); }                   // C 2202
}

/* C: cairo.h 701-702: `void cairo_set_source (cairo_t *cr, cairo_pattern_t *source);` - the
   C 2228 call; cairo.h 870-871: `void cairo_translate (cairo_t *cr, double tx, double ty);` - the
   C 2221 call; cairo.h 998-999: `void cairo_fill_preserve (cairo_t *cr);` - the C 2218 call; and
   cairo.h 2728-2729: `cairo_format_t cairo_image_surface_get_format (cairo_surface_t *surface);` -
   the C 2225 call.  This machine's cairo header carries all four; `include/gtk.rs` has the other
   five names this slice calls (`cairo_pattern_get_surface`, `cairo_mask`, the operator rows and
   `CAIRO_FORMAT_ARGB32`), so these four are declared here at the use site.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_set_source (cr: *mut cairo_t, source: *mut cairo_pattern_t);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_translate (cr: *mut cairo_t, tx: c_double, ty: c_double);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_fill_preserve (cr: *mut cairo_t);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_image_surface_get_format (surface: *mut cairo_surface_t) -> cairo_format_t;
}
/* C: cairo.h 431-441: `typedef enum _cairo_format { CAIRO_FORMAT_INVALID = -1,
   CAIRO_FORMAT_ARGB32 = 0, CAIRO_FORMAT_RGB24 = 1, CAIRO_FORMAT_A8 = 2, CAIRO_FORMAT_A1 = 3, … }`
   - the C 2226 test's two values (the mirror carries `CAIRO_FORMAT_ARGB32`; `src/image.rs`
   353-358 is the tree's copy of this same measurement).  */
#[cfg (feature = "have_pgtk")]
pub const CAIRO_FORMAT_RGB24: cairo_format_t = 1;                        // cairo.h 434
#[cfg (feature = "have_pgtk")]
pub const CAIRO_FORMAT_A8: cairo_format_t = 2;                           // cairo.h 435
#[cfg (feature = "have_pgtk")]
pub const CAIRO_FORMAT_A1: cairo_format_t = 3;                           // cairo.h 436

/// C 2205-2239: `static void pgtk_cr_draw_image (struct frame *f, Emacs_GC *gc, cairo_pattern_t
/// *image, int src_x, int src_y, int width, int height, int dest_x, int dest_y, bool overlay_p)` -
/// C's `cairo_surface_t *surface;` is uninitialised and takes the port's zero initialisation
/// (PORTING.md 1.5), the two `pgtk_set_cr_source_with_gc_*` calls come from the mirror
/// (`include/pgtkterm.rs` 715-718), and C's `dest_x - src_x` widenings are written `as c_double`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_cr_draw_image (f: *mut frame, gc: *mut Emacs_GC, image: *mut cairo_pattern_t,
				      src_x: c_int, src_y: c_int, width: c_int, height: c_int,
				      dest_x: c_int, dest_y: c_int, overlay_p: bool)
{
  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 2210

  if overlay_p                                                          // C 2212
    { cairo_rectangle (cr, dest_x as c_double, dest_y as c_double,      // C 2213
			       width as c_double, height as c_double); }
  else
    {
      pgtk_set_cr_source_with_gc_background (f, gc, false);             // C 2216
      cairo_rectangle (cr, dest_x as c_double, dest_y as c_double,      // C 2217
			       width as c_double, height as c_double);
      cairo_fill_preserve (cr);                                         // C 2218
    }

  cairo_translate (cr, (dest_x - src_x) as c_double,                    // C 2221
			   (dest_y - src_y) as c_double);

  let mut surface: *mut cairo_surface_t = core::ptr::null_mut ();       // C 2223
  cairo_pattern_get_surface (image, &raw mut surface);                  // C 2224
  let format: cairo_format_t = cairo_image_surface_get_format (surface);        // C 2225
  if format != CAIRO_FORMAT_A8 && format != CAIRO_FORMAT_A1            // C 2226
    {
      cairo_set_source (cr, image);                                     // C 2228
      cairo_fill (cr);                                                  // C 2229
    }
  else
    {
      pgtk_set_cr_source_with_gc_foreground (f, gc, false);             // C 2233
      cairo_clip (cr);                                                  // C 2234
      cairo_mask (cr, image);                                           // C 2235
    }

  pgtk_end_cr_clip (f);                                                 // C 2238
}

/* Draw foreground of image glyph string S.  */

/// C 2243-2292: `static void pgtk_draw_image_foreground (struct glyph_string *s)` - `cr_data` is
/// the mirror's `*mut c_void` (C's `void *cr_data;`) handed to `pgtk_cr_draw_image` with the
/// use-site cast (P-72 rule 3), `!s->img->mask` is the pointer's null test, and the two
/// `pgtk_draw_rectangle` calls are this unit's own (slice 4).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_image_foreground (s: *mut glyph_string)
{
  let mut x: c_int = (*s).x;                                            // C 2246
  let mut y: c_int = (*s).ybase                                         // C 2247
    - image_ascent ((*s).img, (*s).face as *mut face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX            // C 2251
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 2252
      && glyph_slice_bits! (x, (*s).slice) == 0                         // C 2253
    { x += max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int; }   // C 2254

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0                             // C 2258
    { x += (*(*s).img).hmargin; }                                       // C 2259
  if glyph_slice_bits! (y, (*s).slice) == 0                             // C 2260
    { y += (*(*s).img).vmargin; }                                       // C 2261

  if !(*(*s).img).cr_data.is_null ()                                    // C 2263
    {
      let cr: *mut cairo_t = pgtk_begin_cr_clip ((*s).f);               // C 2265
      pgtk_set_glyph_string_clipping (s, cr);                           // C 2266
      pgtk_cr_draw_image ((*s).f, &raw mut (*s).xgcv,                   // C 2267
					  (*(*s).img).cr_data as *mut cairo_pattern_t,
					  glyph_slice_bits! (x, (*s).slice) as c_int,   // C 2268
					  glyph_slice_bits! (y, (*s).slice) as c_int,
					  glyph_slice_bits! (width, (*s).slice) as c_int,
					  glyph_slice_bits! (height, (*s).slice) as c_int,
					  x, y, true);                          // C 2269
      if (*(*s).img).mask.is_null ()                                    // C 2270
	{
	  /* When the image has a mask, we can expect that at
	     least part of a mouse highlight or a block cursor will
	     be visible.  If the image doesn't have a mask, make
	     a block cursor visible by drawing a rectangle around
	     the image.  I believe it's looking better if we do
	     nothing here for mouse-face.  */
	  if (*s).hl == DRAW_CURSOR                                        // C 2278
	    {
	      let relief: c_int = eabs! ((*(*s).img).relief);             // C 2280
	      pgtk_draw_rectangle ((*s).f, (*s).xgcv.foreground, x - relief,        // C 2281
				   y - relief,                                  // C 2282
				   glyph_slice_bits! (width, (*s).slice) as c_int + relief * 2 - 1,
				   glyph_slice_bits! (height, (*s).slice) as c_int + relief * 2 - 1,
				   false);                                      // C 2283
	    }
	}
      pgtk_end_cr_clip ((*s).f);                                        // C 2286
    }
  else
    /* Draw a rectangle if image could not be loaded.  */
    { pgtk_draw_rectangle ((*s).f, (*s).xgcv.foreground, x, y,          // C 2290
				   glyph_slice_bits! (width, (*s).slice) as c_int - 1,   // C 2291
				   glyph_slice_bits! (height, (*s).slice) as c_int - 1, false); }
}

/* Draw image glyph string S.

	    s->y
   s->x      +-------------------------
	     |   s->face->box
	     |
	     |     +-------------------------
	     |     |  s->img->margin
	     |     |
	     |     |       +-------------------
	     |     |       |  the image

 */

/// C 2308-2359: `static void pgtk_draw_image_glyph_string (struct glyph_string *s)` - the two
/// `max` folds over the face's box line widths, C's `s->stippled_p` / `s->background_filled_p`
/// bitfield **writes** through `glyph_string_bits!`'s setter arms (the record's measured
/// `bits: [u8; 4]` run), the `slice.y + slice.height >= img->height` fold with the two `as c_int`
/// promotions the tenth slice's Deviation 32 records, and the three calls this unit defines above.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_image_glyph_string (s: *mut glyph_string)
{
  let box_line_hwidth: c_int                                           // C 2311
    = max ((*((*s).face as *mut face)).box_vertical_line_width as isize, 0) as c_int;
  let box_line_vwidth: c_int                                           // C 2312
    = max ((*((*s).face as *mut face)).box_horizontal_line_width as isize, 0) as c_int;
  let mut height: c_int;                                               // C 2313

  height = (*s).height;                                                // C 2315
  if glyph_slice_bits! (y, (*s).slice) == 0                            // C 2316
    { height -= box_line_vwidth; }                                     // C 2317
  if (glyph_slice_bits! (y, (*s).slice) as c_int                       // C 2318
	  + glyph_slice_bits! (height, (*s).slice) as c_int) >= (*(*s).img).height
    { height -= box_line_vwidth; }                                     // C 2319

  /* Fill background with face under the image.  Do it only if row is
     taller than image or if image has a clip mask to reduce
     flickering.  */
  glyph_string_bits! (stippled_p, (*s),                                  // C 2324
		      (*((*s).face as *mut face)).stipple != 0);
  if height > glyph_slice_bits! (height, (*s).slice) as c_int          // C 2325
      || (*(*s).img).hmargin != 0                                      // C 2326
      || (*(*s).img).vmargin != 0                                      // C 2327
      || !(*(*s).img).mask.is_null ()                                  // C 2328
      || (*(*s).img).pixmap.is_null ()                                 // C 2329
      || (*s).width != (*s).background_width                           // C 2330
    {
      let mut x: c_int = (*s).x;                                       // C 2332
      let mut y: c_int = (*s).y;                                       // C 2333
      let mut width: c_int = (*s).background_width;                    // C 2334

      if glyph_bits! (left_box_line_p, (*(*s).first_glyph))            // C 2336
	  && glyph_slice_bits! (x, (*s).slice) == 0                       // C 2337
	{
	  x += box_line_hwidth;                                           // C 2339
	  width -= box_line_hwidth;                                       // C 2340
	}

      if glyph_slice_bits! (y, (*s).slice) == 0                        // C 2343
	{ y += box_line_vwidth; }                                         // C 2344

      pgtk_draw_glyph_string_bg_rect (s, x, y, width, height);         // C 2346

      glyph_string_bits! (background_filled_p, (*s), true);            // C 2348
    }

  /* Draw the foreground.  */
  pgtk_draw_image_foreground (s);                                      // C 2352

  /* If we must draw a relief around the image, do it.  */
  if (*(*s).img).relief != 0                                           // C 2355
      || (*s).hl == DRAW_IMAGE_RAISED                                  // C 2356
      || (*s).hl == DRAW_IMAGE_SUNKEN                                  // C 2357
    { pgtk_draw_image_relief (s); }                                    // C 2358
}

/* Draw stretch glyph string S.  */

/* C: dispextern.h 3579/3580/3692: `int window_box_left (struct window *, enum glyph_row_area);`,
   `int window_box_left_offset (struct window *, enum glyph_row_area);` and `extern bool
   cursor_in_mouse_face_p (struct window *w);` - the C 2377/2389/2414 calls (`src/xdisp.rs` defines
   them; no mirror carries the three, so the unit declares them at the use site - the
   `src/w32term.rs` 3134-3141 block of the same three; `window_box_right` is the ninth slice's).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cursor_in_mouse_face_p (w: *mut window) -> bool;
}

/// C 2363-2455: `static void pgtk_draw_stretch_glyph_string (struct glyph_string *s)` - C's
/// `eassert (s->first_glyph->type == STRETCH_GLYPH)` keeps the macro with the glyph record's `type`
/// bitfield read, `s->row->reversed_p`/`mouse_face_p`/`mode_line_p` are `glyph_row_bits!` reads,
/// `x_stretch_cursor_p` is `include/globals.rs` 7200's `bool` (so C's `!` is the port's), 
/// `FRAME_COLUMN_WIDTH` is `include/frame.rs` 1353's `pub fn`, C's `min` is the port's `isize` fold
/// (Deviation 22's class), and `s->area == TEXT_AREA` is C's enumerator test.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_stretch_glyph_string (s: *mut glyph_string)
{
  eassert! (glyph_bits! (type, (*(*s).first_glyph)) == STRETCH_GLYPH);  // C 2366

  if (*s).hl == DRAW_CURSOR && !x_stretch_cursor_p                      // C 2368
    {
      /* If `x-stretch-cursor' is nil, don't draw a block cursor as
	 wide as the stretch glyph.  */
      let mut width: c_int;                                             // C 2372
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;                                        // C 2373

      if !glyph_row_bits! (reversed_p, (*s).row)                        // C 2375
	{
	  let left_x: c_int = window_box_left_offset ((*s).w, TEXT_AREA);   // C 2377

	  if x < left_x                                                    // C 2379
	    {
	      background_width -= left_x - x;                              // C 2381
	      x = left_x;                                                  // C 2382
	    }
	}
      else
	{
	  /* In R2L rows, draw the cursor on the right edge of the
	     stretch glyph.  */
	  let right_x: c_int = window_box_right ((*s).w, TEXT_AREA);       // C 2389

	  if x + background_width > right_x                                // C 2391
	    { background_width -= x - right_x; }                           // C 2392
	  x += background_width;                                           // C 2393
	}
      width = min (FRAME_COLUMN_WIDTH ((*s).f) as isize,                // C 2395
		   background_width as isize) as c_int;
      if glyph_row_bits! (reversed_p, (*s).row)                         // C 2396
	{ x -= width; }                                                    // C 2397

      /* Draw cursor.  */
      pgtk_draw_glyph_string_bg_rect (s, x, (*s).y, width, (*s).height);   // C 2400

      /* Clear rest using the GC of the original non-cursor face.  */
      if width < background_width                                       // C 2403
	{
	  let mut y: c_int = (*s).y;                                       // C 2405
	  let w: c_int = background_width - width;                         // C 2406
	  let h: c_int = (*s).height;
	  let mut r: XRectangle = core::mem::zeroed ();                    // C 2407
	  let color: c_ulong;                                              // C 2408

	  if !glyph_row_bits! (reversed_p, (*s).row)                       // C 2410
	    { x += width; }                                                // C 2411
	  else
	    { x = (*s).x; }                                                // C 2413
	  if glyph_row_bits! (mouse_face_p, (*s).row)                      // C 2414
		  && cursor_in_mouse_face_p ((*s).w)
	    {
	      pgtk_set_mouse_face_gc (s);                                  // C 2416
	      color = (*s).xgcv.foreground;                                // C 2417
	    }
	  else
	    { color = (*((*s).face as *mut face)).background; }            // C 2420

	  let cr: *mut cairo_t = pgtk_begin_cr_clip ((*s).f);              // C 2422

	  get_glyph_string_clip_rect (s, &raw mut r);                      // C 2424
	  pgtk_set_clip_rectangles ((*s).f, cr, &raw mut r, 1);            // C 2425

	  if (*((*s).face as *mut face)).stipple != 0                      // C 2427
	    { fill_background (s, x, y, w, h); }                           // C 2428
	  else
	    { pgtk_fill_rectangle ((*s).f, color, x, y, w, h,              // C 2430
				       true); }                                 // C 2431

	  pgtk_end_cr_clip ((*s).f);                                       // C 2433
	}
    }
  else if !glyph_string_bits! (background_filled_p, (*s))               // C 2436
    {
      let mut background_width: c_int = (*s).background_width;          // C 2438
      let mut x: c_int = (*s).x;
      let text_left_x: c_int = window_box_left ((*s).w, TEXT_AREA);     // C 2439

      /* Don't draw into left fringe or scrollbar area except for
	 header line and mode line.  */
      if (*s).area == TEXT_AREA                                         // C 2443
	  && x < text_left_x && !glyph_row_bits! (mode_line_p, (*s).row)   // C 2444
	{
	  background_width -= text_left_x - x;                             // C 2446
	  x = text_left_x;                                                 // C 2447
	}

      if background_width > 0                                           // C 2450
	{ pgtk_draw_glyph_string_bg_rect (s, x, (*s).y, background_width, (*s).height); }   // C 2451
    }

  glyph_string_bits! (background_filled_p, (*s), true);                 // C 2454
}

/* Draw a dashed underline of thickness THICKNESS and width WIDTH onto F
   at a vertical offset of OFFSET from the position of the glyph string
   S, with each segment SEGMENT pixels in length.  */

/* C: cairo.h 861-865: `void cairo_set_dash (cairo_t *cr, const double *dashes, int num_dashes,
   double offset);` - the C 2475 call (this machine's cairo header; no mirror carries it, so the
   unit declares it at the use site).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_set_dash (cr: *mut cairo_t, dashes: *const c_double, num_dashes: c_int,
				 offset: c_double);
}

/// C 2462-2481: `static void pgtk_draw_dash (struct frame *f, struct glyph_string *s, unsigned
/// long foreground, int width, char segment, int offset, int thickness)` - C's `(double) segment`
/// and the three `double` arguments to the cairo calls are the port's widenings, and C's
/// `s->ybase + offset + (thickness / 2.0)` keeps the int sum before the double term (so the port
/// writes the sum's own conversion).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_dash (f: *mut frame, s: *mut glyph_string, foreground: c_ulong, width: c_int,
			  segment: c_char, offset: c_int, thickness: c_int)
{
  let cr: *mut cairo_t;                                                 // C 2467
  let mut cr_segment: c_double;                                         // C 2468
  let mut y_center: c_double;

  cr = pgtk_begin_cr_clip ((*s).f);                                     // C 2470
  pgtk_set_cr_source_with_color (f, foreground, false);                 // C 2471
  cr_segment = segment as c_double;                                     // C 2472
  y_center = ((*s).ybase + offset) as c_double                          // C 2473
		 + (thickness as c_double / 2.0);

  cairo_set_dash (cr, &raw const cr_segment, 1, (*s).x as c_double);    // C 2475
  cairo_set_line_width (cr, thickness as c_double);                     // C 2476
  cairo_move_to (cr, (*s).x as c_double, y_center);                     // C 2477
  cairo_line_to (cr, ((*s).x + width) as c_double, y_center);           // C 2478
  cairo_stroke (cr);                                                    // C 2479
  pgtk_end_cr_clip (f);                                                 // C 2480
}

/* Draw an underline of STYLE onto F at an offset of POSITION from the
   baseline of the glyph string S in the color provided by FOREGROUND,
   DECORATION_WIDTH in length, and THICKNESS in height.  */

/// C 2487-2522: `static void pgtk_fill_underline (struct frame *f, struct glyph_string *s,
/// unsigned long foreground, enum face_underline_type style, int position, int decoration_width,
/// int thickness)` - C's `switch` is the port's `match` over the same labels; the
/// `FACE_UNDERLINE_DOTS` arm's `FALLTHROUGH` is carried in the arm itself (PORTING.md 2.2, the
/// `src/w32term.rs` 4083-4108 reading of the same C text); and C's `case FACE_NO_UNDERLINE:` /
/// `case FACE_UNDERLINE_WAVE:` / `default:` all run `emacs_abort`, so they are one arm plus the
/// wildcard.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_fill_underline (f: *mut frame, s: *mut glyph_string, foreground: c_ulong,
			       style: face_underline_type, position: c_int,
			       decoration_width: c_int, thickness: c_int)
{
  let mut segment: c_int;                                               // C 2493

  segment = thickness * 3;                                              // C 2495

  match style
    {
      /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
	 the second line will be filled by another invocation of this
	 function.  */
    FACE_UNDERLINE_SINGLE | FACE_UNDERLINE_DOUBLE_LINE =>
      {
	pgtk_fill_rectangle (f, foreground, (*s).x, (*s).ybase + position,   // C 2504
				     decoration_width, thickness, false);       // C 2505
      }

    FACE_UNDERLINE_DOTS =>
      {
	segment = thickness;                                               // C 2509
	/* C's `FALLTHROUGH;` sends this arm on into the `FACE_UNDERLINE_DASHES` arm's
	   statement, which the port therefore writes here too (PORTING.md 2.2).  */
	pgtk_draw_dash (f, s, foreground, decoration_width, segment as c_char,   // C 2513
			    position, thickness);
      }

    FACE_UNDERLINE_DASHES =>
      {
	pgtk_draw_dash (f, s, foreground, decoration_width, segment as c_char,   // C 2513
			    position, thickness);
      }

    FACE_NO_UNDERLINE | FACE_UNDERLINE_WAVE =>
      { emacs_abort (); }                                               // C 2520
    _ =>
      { emacs_abort (); }
    }
}

/* C: dispextern.h 3638: `extern struct font *font_for_underline_metrics (struct glyph_string *);`
   - the C 2662 call (`src/xdisp.rs` defines it; no mirror carries it, so the unit declares it at
   the use site - the `src/haikuterm.rs` 172 declaration of the same prototype).  cairo.h 561-562/564-565:
   `void cairo_save (cairo_t *cr);` and `void cairo_restore (cairo_t *cr);` - the C 2777/2785 and
   C 2803/2809 calls.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn font_for_underline_metrics (s: *mut glyph_string) -> *mut font;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_save (cr: *mut cairo_t);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_restore (cr: *mut cairo_t);
}

/// C 2524-2825: `static void pgtk_draw_glyph_string (struct glyph_string *s)` - the file's whole
/// drawing entry point.  C's `for (width = 0, next = s->next; …; width += next->width, next =
/// next->next)` is the port's init/`while`/step trio (Deviation 26's spelling), C's
/// `switch (s->first_glyph->type)` is the port's `match` over the same enumerators (this
/// function's own `macro-body-notes.tsv` row), `x_draw_xwidget_glyph_string` takes the mirror's
/// `*mut c_void` parameter (the use-site cast, P-72 rule 3), and C's `s->row->stipple_p` write is
/// `glyph_row_bits!`'s setter.  The epilogue's decoration arithmetic is spelled the way the
/// sibling lane writes the same C text (`src/xterm.rs` 12558-12745): the `c_ulong` locals with
/// `wrapping_sub`/`wrapping_add` for C's unsigned arithmetic, `position = max (position as isize,
/// underline_minimum_offset as isize) as c_ulong` for C's `max`, and `font_for_underline_metrics`/
/// `cairo_save`/`cairo_restore` declared above.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_draw_glyph_string (s: *mut glyph_string)
{
  let mut relief_drawn_p: bool = false;                                 // C 2527

  /* If S draws into the background of its successors, draw the
     background of the successors first so that S can draw into it.
     This makes S->next use XDrawString instead of XDrawImageString.  */
  if !(*s).next.is_null () && (*s).right_overhang != 0                  // C 2532
      && glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut width: c_int;                                             // C 2534
      let mut next: *mut glyph_string;                                  // C 2535

      width = 0; next = (*s).next;                                      // C 2537
      while !next.is_null () && width < (*s).right_overhang             // C 2538
	{
	  if glyph_bits! (type, (*(*next).first_glyph)) != IMAGE_GLYPH     // C 2540
	    {
	      let cr: *mut cairo_t = pgtk_begin_cr_clip ((*next).f);      // C 2542
	      pgtk_set_glyph_string_gc (next);                            // C 2543
	      pgtk_set_glyph_string_clipping (next, cr);                  // C 2544
	      if glyph_bits! (type, (*(*next).first_glyph)) == STRETCH_GLYPH   // C 2545
		{ pgtk_draw_stretch_glyph_string (next); }                   // C 2546
	      else
		{ pgtk_draw_glyph_string_background (next, true); }          // C 2548
	      (*next).num_clips = 0;                                      // C 2549
	      pgtk_end_cr_clip ((*next).f);                               // C 2550
	    }
	  width += (*next).width;                                         // C 2539's step
	  next = (*next).next;
	}
    }

  /* Set up S->gc, set clipping and draw S.  */
  pgtk_set_glyph_string_gc (s);                                         // C 2555

  let cr: *mut cairo_t = pgtk_begin_cr_clip ((*s).f);                   // C 2557

  /* Draw relief (if any) in advance for char/composition so that the
     glyph string can be drawn over it.  */
  if glyph_string_bits! (for_overlaps, (*s)) == 0                       // C 2561
      && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX        // C 2562
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH         // C 2563
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 2564

    {
      pgtk_set_glyph_string_clipping (s, cr);                           // C 2567
      pgtk_draw_glyph_string_background (s, true);                      // C 2568
      pgtk_draw_glyph_string_box (s);                                   // C 2569
      pgtk_set_glyph_string_clipping (s, cr);                           // C 2570
      relief_drawn_p = true;                                            // C 2571
    }
  else if (*s).clip_head.is_null ()   /* draw_glyphs didn't specify a clip mask. */   // C 2573
	   && (*s).clip_tail.is_null ()                                    // C 2574
	   && ((!(*s).prev.is_null () && (*(*s).prev).hl != (*s).hl       // C 2575
		&& (*s).left_overhang != 0)
	       || (!(*s).next.is_null () && (*(*s).next).hl != (*s).hl
		  && (*s).right_overhang != 0))                               // C 2576
    /* We must clip just this glyph.  left_overhang part has already
       drawn when s->prev was drawn, and right_overhang part will be
       drawn later when s->next is drawn. */
    { pgtk_set_glyph_string_clipping_exactly (s, s, cr); }              // C 2580
  else
    { pgtk_set_glyph_string_clipping (s, cr); }                         // C 2582

  match glyph_bits! (type, (*(*s).first_glyph))                         // C 2584
    {
    IMAGE_GLYPH =>
      {
	pgtk_draw_image_glyph_string (s);                                  // C 2587
      }

    XWIDGET_GLYPH =>
      {
	x_draw_xwidget_glyph_string (s as *mut c_void);                    // C 2591
      }

    STRETCH_GLYPH =>
      {
	pgtk_draw_stretch_glyph_string (s);                                // C 2595
      }

    CHAR_GLYPH =>
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0                    // C 2599
	  { glyph_string_bits! (background_filled_p, (*s), true); }        // C 2600
	else
	  { pgtk_draw_glyph_string_background (s, false); }                // C 2602
	pgtk_draw_glyph_string_foreground (s);                             // C 2603
      }

    COMPOSITE_GLYPH =>
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0                    // C 2607
	    || ((*s).cmp_from > 0                                          // C 2607
		&& !glyph_u_cmp_bits! (automatic, (*s).first_glyph))          // C 2608
	  { glyph_string_bits! (background_filled_p, (*s), true); }        // C 2609
	else
	  { pgtk_draw_glyph_string_background (s, true); }                 // C 2611
	pgtk_draw_composite_glyph_string_foreground (s);                   // C 2612
      }

    GLYPHLESS_GLYPH =>
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0                    // C 2616
	  { glyph_string_bits! (background_filled_p, (*s), true); }        // C 2617
	else
	  { pgtk_draw_glyph_string_background (s, true); }                 // C 2619
	pgtk_draw_glyphless_glyph_string_foreground (s);                   // C 2620
      }

    _ =>
      { emacs_abort (); }                                               // C 2624
    }

  if glyph_string_bits! (for_overlaps, (*s)) == 0                       // C 2627
    {
      /* Draw relief if not yet drawn.  */
      if !relief_drawn_p && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX   // C 2630
	{ pgtk_draw_glyph_string_box (s); }                                // C 2631

      /* Draw underline.  */
      if face_bits! (underline, (*s).face as *mut face) != 0            // C 2634
	{
	  if face_bits! (underline, (*s).face as *mut face) == FACE_UNDERLINE_WAVE   // C 2636
	    {
	      if face_bits! (underline_defaulted_p, (*s).face as *mut face)          // C 2638
		{ pgtk_draw_underwave (s, (*s).xgcv.foreground); }            // C 2639
	      else
		{ pgtk_draw_underwave (s, (*((*s).face as *mut face)).underline_color); }   // C 2641
	    }
	  else if face_bits! (underline, (*s).face as *mut face) >= FACE_UNDERLINE_SINGLE   // C 2643
	    {
	      let mut thickness: c_ulong;                                  // C 2645
	      let mut position: c_ulong;
	      let mut foreground: c_ulong;                                 // C 2646

	      if !(*s).prev.is_null ()                                     // C 2648
		  && (face_bits! (underline, (*((*s).prev)).face as *mut face) != FACE_UNDERLINE_WAVE   // C 2649
			  && face_bits! (underline, (*((*s).prev)).face as *mut face)
			     >= FACE_UNDERLINE_SINGLE)                           // C 2650
		  && (face_bits! (underline_at_descent_line_p, (*((*s).prev)).face as *mut face)   // C 2651
			  == face_bits! (underline_at_descent_line_p, (*s).face as *mut face))        // C 2652
		  && ((*((*((*s).prev)).face as *mut face)).underline_pixels_above_descent_line    // C 2653
			  == (*((*s).face as *mut face)).underline_pixels_above_descent_line)         // C 2654
	{
	  /* We use the same underline style as the previous one.  */
	  thickness = (*(*s).prev).underline_thickness as c_ulong;         // C 2657
	  position = (*(*s).prev).underline_position as c_ulong;           // C 2658
	}
	      else
	{
	  let font: *mut font = font_for_underline_metrics (s);            // C 2662

	  /* Get the underline thickness.  Default is 1 pixel.  */
	  if !font.is_null () && (*font).underline_thickness > 0           // C 2665
		{ thickness = (*font).underline_thickness as c_ulong; }       // C 2666
	  else
		{ thickness = 1; }                                            // C 2668
	  if x_underline_at_descent_line                                   // C 2669
		  || face_bits! (underline_at_descent_line_p, (*s).face as *mut face)   // C 2670
		{
		  position = ((*s).height as c_ulong).wrapping_sub (thickness)   // C 2671
			 .wrapping_sub (((*s).ybase - (*s).y) as c_ulong)        // C 2672
			 .wrapping_sub ((*((*s).face as *mut face))
					     .underline_pixels_above_descent_line as c_ulong);   // C 2673
		}
	  else
		{
		  /* Get the underline position.  This is the recommended
		     vertical offset in pixels from the baseline to the top of
		     the underline.  This is a signed value according to the
		     specs, and its default is

		     ROUND ((maximum descent) / 2), with
		     ROUND(x) = floor (x + 0.5)  */

		  if x_use_underline_position_properties                  // C 2684
			  && !font.is_null () && (*font).underline_position >= 0   // C 2685
			{ position = (*font).underline_position as c_ulong; }      // C 2686
		  else if !font.is_null ()                                  // C 2687
			{ position = (((*font).descent + 1) / 2) as c_ulong; }     // C 2688
		  else
			{ position = underline_minimum_offset as c_ulong; }        // C 2690
		}

		  /* Ignore minimum_offset if the amount of pixels was
		     explicitly specified.  */
		  if (*((*s).face as *mut face)).underline_pixels_above_descent_line == 0   // C 2695
			{ position = max (position as isize, underline_minimum_offset as isize) as c_ulong; }   // C 2696
		}
	      /* Check the sanity of thickness and position.  We should
	         avoid drawing underline out of the current line area.  */
	      if (((*s).y + (*s).height) as c_ulong)                       // C 2700
		  <= ((*s).ybase as c_ulong).wrapping_add (position)
		{ position = (((*s).height - 1) - ((*s).ybase - (*s).y)) as c_ulong; }   // C 2701
	      if (((*s).y + (*s).height) as c_ulong)                       // C 2702
		  < ((*s).ybase as c_ulong).wrapping_add (position).wrapping_add (thickness)
		{
		  thickness = (((*s).y + (*s).height) as c_ulong)
			      .wrapping_sub ((*s).ybase as c_ulong + position);   // C 2703
		}
	      (*s).underline_thickness = thickness as c_int;              // C 2704
	      (*s).underline_position = position as c_int;                 // C 2705

	      if face_bits! (underline_defaulted_p, (*s).face as *mut face)   // C 2707
		{ foreground = (*s).xgcv.foreground; }                        // C 2708
	      else
		{ foreground = (*((*s).face as *mut face)).underline_color; }   // C 2710

	      pgtk_fill_underline ((*s).f, s, foreground,                  // C 2712
			     face_bits! (underline, (*s).face as *mut face),
			     position as c_int, (*s).width, thickness as c_int);   // C 2713

	      /* Place a second underline above the first if this was
		 requested in the face specification.  */

	      if face_bits! (underline, (*s).face as *mut face) == FACE_UNDERLINE_DOUBLE_LINE   // C 2718
		{
		  /* Compute the position of the second underline.  */
		  position = position - thickness - 1;                        // C 2721
		  pgtk_fill_underline ((*s).f, s, foreground,                 // C 2722
				     face_bits! (underline, (*s).face as *mut face),
				     position as c_int, (*s).width, thickness as c_int);   // C 2723
		}
	    }
	}
      /* Draw overline.  */
      if face_bits! (overline_p, (*s).face as *mut face)                // C 2728
	{
	  let mut dy: c_ulong = 0;                                         // C 2730
	  let h: c_ulong = 1;

	  if face_bits! (overline_color_defaulted_p, (*s).face as *mut face)   // C 2732
		{ pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground, (*s).x,   // C 2733
				       ((*s).y as c_ulong + dy) as c_int,
				       (*s).width, h as c_int, false); }              // C 2734
	  else
		{ pgtk_fill_rectangle ((*s).f, (*((*s).face as *mut face)).overline_color, (*s).x,   // C 2736
				       ((*s).y as c_ulong + dy) as c_int,
				       (*s).width, h as c_int, false); }              // C 2737
	}

      /* Draw strike-through.  */
      if face_bits! (strike_through_p, (*s).face as *mut face)          // C 2741
	{
	  /* Y-coordinate and height of the glyph string's first
	     glyph.  We cannot use s->y and s->height because those
	     could be larger if there are taller display elements
	     (e.g., characters displayed with a larger font) in the
	     same glyph row.  */
	  let glyph_y: c_int = (*s).ybase                                  // C 2748
		- (*(*s).first_glyph).ascent as c_int;
	  let glyph_height: c_int = (*(*s).first_glyph).ascent as c_int    // C 2749
			+ (*(*s).first_glyph).descent as c_int;
	  /* Strike-through width and offset from the glyph string's
	     top edge.  */
	  let h: c_ulong = 1;                                              // C 2752
	  let dy: c_ulong = ((glyph_height as c_ulong).wrapping_sub (h)) / 2;   // C 2753

	  if face_bits! (strike_through_color_defaulted_p, (*s).face as *mut face)   // C 2755
		{ pgtk_fill_rectangle ((*s).f, (*s).xgcv.foreground, (*s).x,   // C 2756
				       (glyph_y as c_ulong + dy) as c_int,
				       (*s).width, h as c_int, false); }              // C 2757
	  else
		{ pgtk_fill_rectangle ((*s).f, (*((*s).face as *mut face)).strike_through_color,   // C 2759
				       (*s).x, (glyph_y as c_ulong + dy) as c_int,
				       (*s).width, h as c_int, false); }              // C 2760
	}

      if !(*s).prev.is_null ()                                          // C 2763
	{
	  let mut prev: *mut glyph_string;                                 // C 2765

	  prev = (*s).prev;                                                // C 2767
	  while !prev.is_null ()
	    {
	      if (*prev).hl != (*s).hl                                 // C 2768
		  && (*prev).x + (*prev).width + (*prev).right_overhang > (*s).x   // C 2769
		{
		  /* As prev was drawn while clipped to its own area, we
		     must draw the right_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*prev).hl;                    // C 2773

		  (*prev).hl = (*s).hl;                                       // C 2775
		  pgtk_set_glyph_string_gc (prev);                            // C 2776
		  cairo_save (cr);                                            // C 2777
		  pgtk_set_glyph_string_clipping_exactly (s, prev, cr);       // C 2778
		  if glyph_bits! (type, (*(*prev).first_glyph)) == CHAR_GLYPH   // C 2779
			{ pgtk_draw_glyph_string_foreground (prev); }            // C 2780
		  else
			{ pgtk_draw_composite_glyph_string_foreground (prev); }  // C 2782
		  (*prev).hl = save;                                          // C 2783
		  (*prev).num_clips = 0;                                      // C 2784
		  cairo_restore (cr);                                         // C 2785
		}
	      prev = (*prev).prev;                                         // C 2767's step
	    }
	}

      if !(*s).next.is_null ()                                          // C 2789
	{
	  let mut next: *mut glyph_string;                                 // C 2791

	  next = (*s).next;                                                // C 2793
	  while !next.is_null ()
	    {
	      if (*next).hl != (*s).hl                                 // C 2794
		  && (*next).x - (*next).left_overhang < (*s).x + (*s).width   // C 2795
		{
		  /* As next will be drawn while clipped to its own area,
		     we must draw the left_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*next).hl;                    // C 2799

		  (*next).hl = (*s).hl;                                       // C 2801
		  pgtk_set_glyph_string_gc (next);                            // C 2802
		  cairo_save (cr);                                            // C 2803
		  pgtk_set_glyph_string_clipping_exactly (s, next, cr);       // C 2804
		  if glyph_bits! (type, (*(*next).first_glyph)) == CHAR_GLYPH   // C 2805
			{ pgtk_draw_glyph_string_foreground (next); }            // C 2806
		  else
			{ pgtk_draw_composite_glyph_string_foreground (next); }  // C 2808
		  cairo_restore (cr);                                         // C 2809
		  (*next).hl = save;                                          // C 2810
		  (*next).num_clips = 0;                                      // C 2811
		  (*next).clip_head = (*s).next;                              // C 2812
		}
	      next = (*next).next;                                         // C 2793's step
	    }
	}
    }

  /* TODO: figure out in which cases the stipple is actually drawn on
     PGTK.  */
  if !glyph_row_bits! (stipple_p, (*s).row)                             // C 2819
    { glyph_row_bits! (stipple_p, (*s).row, (*((*s).face as *mut face)).stipple != 0); }   // C 2820

  /* Reset clipping.  */
  pgtk_end_cr_clip ((*s).f);                                            // C 2823
  (*s).num_clips = 0;                                                   // C 2824
}

/* RIF: Define cursor CURSOR on frame F.  */

/// C 2829-2832: `!f->pointer_invisible` - `frame.h` 437's `bool_bf pointer_invisible : 1`, whose
/// shift `include/frame.rs` 540-545 names for this arm (A: x11/pgtk/android/w32); the mirror carries
/// the shift and no accessor, so the unit declares the member's own reader - the
/// `src/androidterm.rs` 240-243 copy of the same macro (`bits1` is the measured word 1).
#[cfg (feature = "have_pgtk")]
macro_rules! pointer_invisible {
  ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_POINTER_INVISIBLE_SHIFT))
                  | ((($v) as u32) << FRAME_POINTER_INVISIBLE_SHIFT) };
}
/// C 3405/3409/…: `f->borders_respect_alpha_background` - `frame.h` 540's `bool_bf
/// borders_respect_alpha_background : 1`, whose shift `include/frame.rs` 746-751 names
/// for this arm (A: byte 2 bit 24); the mirror carries the shift and no accessor, so the
/// unit declares the member's own reader - the `src/frame.rs` 298-301 copy of the same
/// macro over `bits2`.
#[cfg (feature = "have_pgtk")]
macro_rules! borders_respect_alpha_background {
  ($f:expr) => { (*$f).bits2 >> FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT & 1 != 0 };
}

/* C: window.h 1186: `struct glyph *get_phys_cursor_glyph (struct window *w);` - the C 2886/2933
   calls (`src/window.rs` 8061 defines it).  dispextern.h 3652-3653: `extern void
   get_phys_cursor_geometry (struct window *, struct glyph_row *, struct glyph *, int *, int *,
   int *);` - the C 2891 call (`src/abi_shim.rs` 2419 defines it).  No mirror carries either, so
   both are declared here at the use site.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn get_phys_cursor_glyph (w: *mut window) -> *mut glyph;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn get_phys_cursor_geometry (w: *mut window, row: *mut glyph_row, glyph: *mut glyph,
				   xp: *mut c_int, yp: *mut c_int, heightp: *mut c_int);
}

/// C 2829-2836: `static void pgtk_define_frame_cursor (struct frame *f, Emacs_Cursor cursor)` -
/// `f->pointer_invisible` is the unit's `pointer_invisible!` read (the mirror's shift), the
/// `current_cursor` member is the mirror's, and C's two GDK/GTK calls are `include/gtk.rs`'s rows
/// (`gdk_window_set_cursor` 564, `gtk_widget_get_window` 883) with the `Emacs_Cursor` cast the
/// mirror's `*mut GdkCursor` parameter needs.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_define_frame_cursor (f: *mut frame, cursor: Emacs_Cursor)
{
  if !pointer_invisible! (f)                                            // C 2832
      && (*FRAME_X_OUTPUT! (f)).current_cursor != cursor
    {
      gdk_window_set_cursor (gtk_widget_get_window (FRAME_GTK_WIDGET! (f)),   // C 2833
				     cursor as *mut GdkCursor);                 // C 2834
    }
  (*FRAME_X_OUTPUT! (f)).current_cursor = cursor;                       // C 2835
}

/// C 2838-2867: `static void pgtk_after_update_window_line (struct window *w, struct glyph_row
/// *desired_row)` - C's two comma expressions inside the condition (`(f = XFRAME (…), width = …,
/// width != 0)` and `(height = …, height > 0)`) are the port's blocks in the condition, the
/// `src/w32term.rs` 1431-1441 reading of the same C text; `mode_line_p`/`full_width_p`/
/// `redraw_fringe_bitmaps_p` are `glyph_row_bits!` members, `pseudo_window_p` this unit's
/// `window_bits!` arm, and `pgtk_clear_frame_area` is defined below.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_after_update_window_line (w: *mut window, desired_row: *mut glyph_row)
{
  let mut f: *mut frame;                                                // C 2842
  let mut width: c_int;
  let mut height: c_int;

  /* begin copy from other terms */
  eassert! (w);                                                         // C 2846

  if !glyph_row_bits! (mode_line_p, desired_row)                        // C 2848
      && !window_bits! (pseudo_window_p, w)
    { glyph_row_bits! (redraw_fringe_bitmaps_p, desired_row, true); }   // C 2849

  /* When a window has disappeared, make sure that no rest of
     full-width rows stays visible in the internal border.  */
  if windows_or_buffers_changed != 0                                    // C 2853
      && glyph_row_bits! (full_width_p, desired_row)                    // C 2854
      && {
	  f = XFRAME ((*w).frame);                                        // C 2855
	  width = FRAME_INTERNAL_BORDER_WIDTH (f);                         // C 2856
	  width != 0
	}
      && {
	  height = (*desired_row).visible_height;                          // C 2857
	  height > 0
	}
    {
      let mut y: c_int = WINDOW_TO_FRAME_PIXEL_Y! (w, max ((*desired_row).y as isize, 0) as c_int);   // C 2859

      block_input ();                                                   // C 2861
      pgtk_clear_frame_area (f, 0, y, width, height);                   // C 2862
      pgtk_clear_frame_area (f,                                         // C 2863
			     FRAME_PIXEL_WIDTH (f) - width, y, width, height);  // C 2864
      unblock_input ();                                                 // C 2865
    }
}

/// C 2869-2873: `static void pgtk_clear_frame_area (struct frame *f, int x, int y, int width, int
/// height)` - one call to `pgtk_clear_area` (C 7456's definition, in the remainder; the mirror
/// `include/pgtkterm.rs` declares the entry point, so no class is added here).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_clear_frame_area (f: *mut frame, x: c_int, y: c_int, width: c_int, height: c_int)
{
  pgtk_clear_area (f, x, y, width, height);                             // C 2872
}

/* Draw a hollow box cursor on window W in glyph row ROW.  */

/// C 2877-2914: `static void pgtk_draw_hollow_cursor (struct window *w, struct glyph_row *row)` -
/// `get_phys_cursor_glyph`/`get_phys_cursor_geometry` are the two use-site declarations above,
/// `cursor_glyph->resolved_level` is `glyph_bits!`'s member and `pixel_width` C's `short` (so the
/// comparison takes `as c_int`, Deviation 22's class), and `pgtk_clip_to_row` (C 3629's
/// definition, in the remainder) is a name this slice is the first to reach - a registered class
/// of its own in this ledger.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_hollow_cursor (w: *mut window, row: *mut glyph_row)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 2880
  let mut x: c_int = 0;                                                 // C 2881
  let mut y: c_int = 0;
  let mut wd: c_int = 0;
  let mut h: c_int = 0;
  let mut cursor_glyph: *mut glyph;                                     // C 2882

  /* Get the glyph the cursor is on.  If we can't tell because
     the current matrix is invalid or such, give up.  */
  cursor_glyph = get_phys_cursor_glyph (w);                             // C 2886
  if cursor_glyph.is_null ()                                            // C 2887
    { return; }                                                         // C 2888

  /* Compute frame-relative coordinates for phys cursor.  */
  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut x, &raw mut y, &raw mut h);   // C 2891
  wd = (*w).phys_cursor_width - 1;                                      // C 2892

  /* The foreground of cursor_gc is typically the same as the normal
     background color, which can cause the cursor box to be invisible.  */
  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 2896
  pgtk_set_cr_source_with_color (f, (*FRAME_X_OUTPUT! (f)).cursor_color, false);   // C 2897

  /* When on R2L character, show cursor at the right edge of the
     glyph, unless the cursor box is as wide as the glyph or wider
     (the latter happens when x-stretch-cursor is non-nil).  */
  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0           // C 2902
      && (*cursor_glyph).pixel_width as c_int > wd                      // C 2903
    {
      x += (*cursor_glyph).pixel_width as c_int - wd;                   // C 2905
      if wd > 0                                                         // C 2906
	{ wd -= 1; }                                                       // C 2907
    }
  /* Set clipping, draw the rectangle, and reset clipping again.  */
  pgtk_clip_to_row (w, row, TEXT_AREA, cr);                             // C 2910
  pgtk_draw_rectangle (f, (*FRAME_X_OUTPUT! (f)).cursor_color,          // C 2911
			       x, y, wd, h - 1, false);                         // C 2912
  pgtk_end_cr_clip (f);                                                 // C 2913
}

/* Draw a bar cursor on window W in glyph row ROW.

   Implementation note: One would like to draw a bar cursor with an
   angle equal to the one given by the font property XA_ITALIC_ANGLE.
   Unfortunately, I didn't find a font yet that has this property set.
   --gerd.  */

/* frame.h 1379 / 1643-1666's `FRAME_FACE_CACHE` and `FACE_FROM_ID` are C's *inline* definitions,
   which C pastes into every unit that includes the header, so the port keeps one copy per
   including unit (the `src/term.rs` 360-371 form; `include/frame.rs` 1534-1552 records why
   that mirror's own copy is a module instead).  */
#[cfg (feature = "have_pgtk")]
#[inline(always)]
pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
{ unsafe { (*f).face_cache as *mut face_cache } }
#[cfg (feature = "have_pgtk")]
#[inline(always)]
pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
  unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
}
#[cfg (feature = "have_pgtk")]
#[inline(always)]
pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = unsafe { (*FRAME_FACE_CACHE (f)).used };
  eassume! (0 <= used);
  if 0 <= id && id < used { unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) } }
  else { core::ptr::null_mut () }
}
/* C: dispextern.h 3649-3651: `extern void draw_phys_cursor_glyph (struct window *, struct
   glyph_row *, enum draw_glyphs_face);` - the C 2948 call (`src/abi_shim.rs` 2499 defines it);
   no mirror carries it, so the unit declares it at the use site.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn draw_phys_cursor_glyph (w: *mut window, row: *mut glyph_row, draw: draw_glyphs_face);
}

/// C 2923-3012: `static void pgtk_draw_bar_cursor (struct window *w, struct glyph_row *row, int
/// width, enum text_cursor_kinds kind)` - `MATRIX_ROW!` is the mirror's macro, `FACE_FROM_ID` is
/// `include/frame.rs`'s, `w->phys_cursor` is the window record's `cursor_pos`, `WINDOW_TEXT_TO_
/// FRAME_PIXEL_X!`/`WINDOW_TO_FRAME_PIXEL_Y!` are `window.rs`'s macros, C's two `min` calls are the
/// port's `isize` fold, and `pgtk_clip_to_row`/`pgtk_fill_rectangle` are this ledger's registered
/// classes (C 3629 / C 5035's definitions, in the remainder).  `width` is reassigned in C's body, so
/// the parameter is `mut` (the `src/image.rs` 2902 `mut width` shape).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_draw_bar_cursor (w: *mut window, row: *mut glyph_row, mut width: c_int,
				kind: text_cursor_kinds)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 2927
  let mut cursor_glyph: *mut glyph;                                     // C 2928

  /* If cursor is out of bounds, don't draw garbage.  This can happen
     in mini-buffer windows when switching between echo area glyphs
     and mini-buffer.  */
  cursor_glyph = get_phys_cursor_glyph (w);                             // C 2933
  if cursor_glyph.is_null ()                                            // C 2934
    { return; }                                                         // C 2935

  /* Experimental avoidance of cursor on xwidget.  */
  if glyph_bits! (type, (*cursor_glyph)) == XWIDGET_GLYPH                // C 2938
    { return; }                                                         // C 2939

  /* If on an image, draw like a normal cursor.  That's usually better
     visible than drawing a bar, esp. if the image is large so that
     the bar might not be in the window.  */
  if glyph_bits! (type, (*cursor_glyph)) == IMAGE_GLYPH                 // C 2944
    {
      let mut r: *mut glyph_row;                                        // C 2946
      r = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,   // C 2947
		     (*w).phys_cursor.vpos);
      draw_phys_cursor_glyph (w, r, DRAW_CURSOR);                       // C 2948
    }
  else
    {
      let face: *mut face = FACE_FROM_ID (f, glyph_bits! (face_id, (*cursor_glyph)));   // C 2952
      let mut color: c_ulong;                                           // C 2953

      let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                    // C 2955

      /* If the glyph's background equals the color we normally draw
	 the bars cursor in, the bar cursor in its normal color is
	 invisible.  Use the glyph's foreground color instead in this
	 case, on the assumption that the glyph's colors are chosen so
	 that the glyph is legible.  */
      if (*face).background == (*FRAME_X_OUTPUT! (f)).cursor_color     // C 2962
	{ color = (*face).foreground; }                                    // C 2963
      else
	{ color = (*FRAME_X_OUTPUT! (f)).cursor_color; }                   // C 2965

      pgtk_clip_to_row (w, row, TEXT_AREA, cr);                         // C 2967

      if kind == BAR_CURSOR                                             // C 2969
	{
	  let mut x: c_int = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);   // C 2971

	  if width < 0                                                     // C 2973
		{ width = FRAME_CURSOR_WIDTH (f); }                           // C 2974
	  width = min ((*cursor_glyph).pixel_width as isize, width as isize) as c_int;   // C 2975

	  (*w).phys_cursor_width = width;                                  // C 2977

	  /* If the character under cursor is R2L, draw the bar cursor
	     on the right of its glyph, rather than on the left.  */
	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0      // C 2981
		{ x += (*cursor_glyph).pixel_width as c_int - width; }        // C 2982

	  pgtk_fill_rectangle (f, color, x,                                // C 2984
				       WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y),   // C 2985
				       width, (*row).height, false);            // C 2986
	}
      else	/* HBAR_CURSOR */                                        // C 2988
	{
	  let mut dummy_x: c_int = 0;                                      // C 2990
	  let mut dummy_y: c_int = 0;
	  let mut dummy_h: c_int = 0;
	  let mut x: c_int = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);   // C 2991

	  if width < 0                                                     // C 2993
		{ width = (*row).height; }                                    // C 2994

	  width = min ((*row).height as isize, width as isize) as c_int;   // C 2996

	  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut dummy_x,   // C 2998
					&raw mut dummy_y, &raw mut dummy_h);    // C 2999

	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0      // C 3001
		  && (*cursor_glyph).pixel_width as c_int > (*w).phys_cursor_width - 1   // C 3002
		{ x += (*cursor_glyph).pixel_width as c_int - (*w).phys_cursor_width + 1; }   // C 3003
	  pgtk_fill_rectangle (f, color, x,                                // C 3004
				       WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y +   // C 3005
								    (*row).height - width),   // C 3006
				       (*w).phys_cursor_width - 1, width, false);   // C 3007
	}

      pgtk_end_cr_clip (f);                                             // C 3010
    }
}

/* C: dispextern.h 3703: `void draw_fringe_bitmap (struct window *, struct glyph_row *, int);` -
   the C 3034 call (`src/fringe.rs` 1280 defines it); no mirror carries it, so the unit declares it
   at the use site.  cairo.h 392-396: `typedef enum _cairo_content { CAIRO_CONTENT_COLOR =
   0x1000, CAIRO_CONTENT_ALPHA = 0x2000, CAIRO_CONTENT_COLOR_ALPHA = 0x3000 } cairo_content_t;` -
   the C 3088 argument (the mirror carries `cairo_format_t` and not this enum).
   cairo.h 2358-2361: `cairo_surface_t *cairo_surface_create_similar (cairo_surface_t *other,
   cairo_content_t content, int width, int height);` - the C 3087 call.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn draw_fringe_bitmap (w: *mut window, row: *mut glyph_row, left_p: c_int);
}
#[cfg (feature = "have_pgtk")]
pub type cairo_content_t = c_int;
#[cfg (feature = "have_pgtk")]
pub const CAIRO_CONTENT_COLOR: cairo_content_t = 0x1000;                 // cairo.h 393
#[cfg (feature = "have_pgtk")]
pub const CAIRO_CONTENT_ALPHA: cairo_content_t = 0x2000;                 // cairo.h 394
#[cfg (feature = "have_pgtk")]
pub const CAIRO_CONTENT_COLOR_ALPHA: cairo_content_t = 0x3000;           // cairo.h 395
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_surface_create_similar (other: *mut cairo_surface_t, content: cairo_content_t,
				       width: c_int, height: c_int) -> *mut cairo_surface_t;
}
/* C: cairo.h 1054-1056: `typedef struct _cairo_rectangle { double x, y, width, height; }
   cairo_rectangle_t;` - the C 3080-3081 parameters' record (the mirror carries the
   cairo types' pointers and not this record).  */
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct cairo_rectangle_t
{
  pub x: c_double,     // cairo.h 1055: `double x, y, width, height;`
  pub y: c_double,
  pub width: c_double,
  pub height: c_double,
}

/* RIF: Draw cursor on window W.  */

/// C 3016-3077: `static void pgtk_draw_window_cursor (struct window *w, struct glyph_row
/// *glyph_row, int x, int y, enum text_cursor_kinds cursor_type, int cursor_width, bool on_p,
/// bool active_p)` - `w->phys_cursor_on_p` is this unit's `window_bits!` setter (the window
/// record's measured `bits` word, whose shift `include/window.rs` 191 names),
/// `exact_window_width_line_p`/`reversed_p`/`cursor_in_fringe_p` are `glyph_row_bits!` members,
/// `WINDOW_LEFT_FRINGE_WIDTH!`/`WINDOW_LEFT_MARGIN_WIDTH!`/`WINDOW_TO_FRAME_PIXEL_*!` are
/// `window.rs`'s macros, and C's `switch (cursor_type)` is the port's `match` over the same
/// enumerators (this function's own `macro-body-notes.tsv` row).  `active_p` is unused in C's body
/// (`let _ =` keeps the name).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_draw_window_cursor (w: *mut window, glyph_row: *mut glyph_row, x: c_int, y: c_int,
				   cursor_type: text_cursor_kinds, cursor_width: c_int,
				   on_p: bool, active_p: bool)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 3021
  let _ = active_p;

  if on_p                                                               // C 3023
    {
      (*w).phys_cursor_type = cursor_type;                              // C 3025
      window_bits! (phys_cursor_on_p, w, true);                         // C 3026

      if glyph_row_bits! (exact_window_width_line_p, glyph_row)         // C 3028
	  && (if glyph_row_bits! (reversed_p, glyph_row)                   // C 3029
		{ (*w).phys_cursor.hpos < 0 }                                 // C 3030
		else { (*w).phys_cursor.hpos >= (*glyph_row).used [TEXT_AREA as usize] as c_int })   // C 3031
	{
	  glyph_row_bits! (cursor_in_fringe_p, glyph_row, true);           // C 3033
	  draw_fringe_bitmap (w, glyph_row,                               // C 3034
				   glyph_row_bits! (reversed_p, glyph_row) as c_int);
	}
      else
	{
	  match cursor_type                                              // C 3038
	    {
	    HOLLOW_BOX_CURSOR =>
		{ pgtk_draw_hollow_cursor (w, glyph_row); }                   // C 3041

	    FILLED_BOX_CURSOR =>
		{ draw_phys_cursor_glyph (w, glyph_row, DRAW_CURSOR); }       // C 3045

	    BAR_CURSOR =>
		{ pgtk_draw_bar_cursor (w, glyph_row, cursor_width, BAR_CURSOR); }    // C 3049

	    HBAR_CURSOR =>
		{ pgtk_draw_bar_cursor (w, glyph_row, cursor_width, HBAR_CURSOR); }   // C 3053

	    NO_CURSOR =>
		{ (*w).phys_cursor_width = 0; }                               // C 3057

	    _ =>
		{ emacs_abort (); }                                           // C 3061
	    }
	}

      if w == XWINDOW ((*f).selected_window)                            // C 3065
	{
	  let frame_x: c_int = (WINDOW_TO_FRAME_PIXEL_X! (w, x)            // C 3067
			+ WINDOW_LEFT_FRINGE_WIDTH! (w)                          // C 3068
			+ WINDOW_LEFT_MARGIN_WIDTH! (w));                        // C 3069
	  let frame_y: c_int = WINDOW_TO_FRAME_PIXEL_Y! (w, y);            // C 3070
	  pgtk_im_set_cursor_location (f, frame_x, frame_y,                // C 3071
					     (*w).phys_cursor_width,            // C 3072
					     (*w).phys_cursor_height);          // C 3073
	}
    }

}

/// C 3079-3110: `static void pgtk_copy_bits (struct frame *f, cairo_rectangle_t *src_rect,
/// cairo_rectangle_t *dst_rect)` - `FRAME_CR_SURFACE!` is the descriptor mirror's macro, the cairo
/// calls are `include/gtk.rs`'s rows plus the two declared above, and C's four `(int)` casts of the
/// record's `double` members are written out.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_copy_bits (f: *mut frame, src_rect: *mut cairo_rectangle_t,
			  dst_rect: *mut cairo_rectangle_t)
{
  let mut cr: *mut cairo_t;                                             // C 3083
  let surface: *mut cairo_surface_t;	/* temporary surface */          // C 3084

  surface = cairo_surface_create_similar (FRAME_CR_SURFACE! (f),        // C 3086
				  CAIRO_CONTENT_COLOR_ALPHA,                    // C 3088
				  (*src_rect).width as c_int,                   // C 3089
				  (*src_rect).height as c_int);                 // C 3090

  cr = cairo_create (surface);                                          // C 3092
  cairo_set_source_surface (cr, FRAME_CR_SURFACE! (f),                  // C 3093
			    -((*src_rect).x), -((*src_rect).y));                // C 3094
  cairo_rectangle (cr, 0.0, 0.0, (*src_rect).width, (*src_rect).height);   // C 3095
  cairo_clip (cr);                                                      // C 3096
  cairo_paint (cr);                                                     // C 3097
  cairo_destroy (cr);                                                   // C 3098

  cr = pgtk_begin_cr_clip (f);                                          // C 3100
  cairo_set_source_surface (cr, surface, (*dst_rect).x, (*dst_rect).y);   // C 3101
  cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);                       // C 3102
  cairo_rectangle (cr, (*dst_rect).x, (*dst_rect).y, (*dst_rect).width,   // C 3103
			   (*dst_rect).height);                                 // C 3104
  cairo_clip (cr);                                                      // C 3105
  cairo_paint (cr);                                                     // C 3106
  pgtk_end_cr_clip (f);                                                 // C 3107

  cairo_surface_destroy (surface);                                      // C 3109
}

/* Scroll part of the display as described by RUN.  */

/* C: dispextern.h 3574-3575: `void window_box (struct window *, enum glyph_row_area, int *, int *,
   int *, int *);` - the C 3123/3203 calls (`src/window.rs` defines it).  dispextern.h 3657:
   `extern void gui_clear_cursor (struct window *);` - the C 3246 call.  No mirror carries either,
   so both are declared at the use site.  `<gtk/gtk.h>`: `gtk_widget_queue_allocate` - the C 3236
   call, declared with the header alone and no line (the GTK segment's convention in this unit:
   this machine has no GTK tree).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn window_box (w: *mut window, area: glyph_row_area, x: *mut c_int, y: *mut c_int,
		     width: *mut c_int, height: *mut c_int);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gui_clear_cursor (w: *mut window);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_widget_queue_allocate (widget: *mut GtkWidget);
}

/// C 3114-3255: `static void pgtk_scroll_run (struct window *w, struct run *run)` - `window_box`
/// and `gui_clear_cursor` are the two use-site declarations above, `run` is
/// `include/dispextern.rs`'s record, C's two `cairo_rectangle_t` initialisers are this unit's
/// record (declared with the previous slice) written with the `int` -> `double` conversions C
/// performs, and the whole `#ifdef HAVE_XWIDGETS` block (C 3160-3243) is dead in the frozen
/// configuration and carried under `have_xwidgets` (P-51), with the `Emacs_Rectangle` record's
/// `c_uint` members taking C's implicit widenings and the C cast macros (`GTK_CONTAINER`,
/// `GTK_FIXED`, `G_OBJECT`) written as the mirror's pointer casts (P-72 rule 3).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_scroll_run (w: *mut window, run: *mut run)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 3117
  let mut x: c_int = 0;                                                 // C 3118
  let mut y: c_int = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  let mut from_y: c_int;
  let mut to_y: c_int;
  let mut bottom_y: c_int;

  /* Get frame-relative bounding box of the text display area of W,
     without mode lines.  Include in this box the left and right
     fringe of W.  */
  window_box (w, ANY_AREA, &raw mut x, &raw mut y, &raw mut width,      // C 3123
		  &raw mut height);

  from_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).current_y);              // C 3125
  to_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).desired_y);                // C 3126
  bottom_y = y + height;                                                // C 3127

  if to_y < from_y                                                      // C 3129
    {
      /* Scrolling up.  Make sure we don't copy part of the mode
	 line at the bottom.  */
      if from_y + (*run).height > bottom_y                              // C 3133
	{ height = bottom_y - from_y; }                                    // C 3134
      else
	{ height = (*run).height; }                                        // C 3136

      /* Don't draw over the display parts above if destination is off
	 the top of the current window.  */
      if to_y < y                                                       // C 3140
	{
	  let d: c_int = y - to_y;                                         // C 3142
	  height -= d;                                                     // C 3143
	  to_y += d;                                                       // C 3144
	  from_y += d;                                                     // C 3145
	}
    }
  else
    {
      /* Scrolling down.  Make sure we don't copy over the mode line.
	 at the bottom.  */
      if to_y + (*run).height > bottom_y                                // C 3152
	{ height = bottom_y - to_y; }                                      // C 3153
      else
	{ height = (*run).height; }                                        // C 3155
    }

  block_input ();                                                       // C 3158

  /* C's `#ifdef HAVE_XWIDGETS` block (C 3160-3243): dead in the frozen configuration, carried
     under its feature.  */
  #[cfg (feature = "have_xwidgets")]
  {
    let mut tem: *mut GtkWidget;                                        // C 3162
    let parent: *mut GtkWidget = FRAME_GTK_WIDGET! (f);
    let children: *mut GList = gtk_container_get_children (parent as *mut GtkContainer);   // C 3163
    let mut iter: *mut GList;                                           // C 3164
    let mut view: *mut xwidget_view;                                    // C 3165

    iter = children;                                                    // C 3167
    while !iter.is_null ()
	{
	  tem = (*iter).data as *mut GtkWidget;                            // C 3169
	  view = g_object_get_data (tem as *mut GObject, XG_XWIDGET_VIEW.as_ptr ()) as *mut xwidget_view;   // C 3170

	  if !view.is_null () && !(*view).hidden                           // C 3172
	    {
		let window_y: c_int = (*view).y + (*view).clip_top;           // C 3174
		let window_height: c_int = (*view).clip_bottom - (*view).clip_top;   // C 3175

		let mut r1: Emacs_Rectangle = core::mem::zeroed ();           // C 3177
		let mut r2: Emacs_Rectangle = core::mem::zeroed ();
		let mut result: Emacs_Rectangle = core::mem::zeroed ();
		r1.x = (*w).pixel_left;                                       // C 3178
		r1.y = from_y;                                                // C 3179
		r1.width = (*w).pixel_width as c_uint;                        // C 3180
		r1.height = height as c_uint;                                 // C 3181
		r2 = r1;                                                      // C 3182
		r2.y = window_y;                                              // C 3183
		r2.height = window_height as c_uint;                          // C 3184

		/* The window is offscreen, just unmap it.  */
		if window_height == 0                                         // C 3187
		  {
		    (*view).hidden = true;                                    // C 3189
		    gtk_widget_hide (tem);                                    // C 3190
		    iter = (*iter).next;                                      // C 3167's step
		    continue;
		  }

		let intersects_p: bool                                       // C 3194
		  = gui_intersect_rectangles (&raw const r1, &raw const r2, &raw mut result);   // C 3195

		if XWINDOW ((*view).w) == w && intersects_p                  // C 3197
		  {
		    let mut y: c_int = (*view).y + (to_y - from_y);          // C 3199
		    let mut text_area_x: c_int = 0;                              // C 3200
		    let mut text_area_y: c_int = 0;
		    let mut text_area_width: c_int = 0;
		    let mut text_area_height: c_int = 0;
		    let mut clip_top: c_int;                                 // C 3201
		    let mut clip_bottom: c_int;

		    window_box (w, (*view).area, &raw mut text_area_x,        // C 3203
				 &raw mut text_area_y, &raw mut text_area_width,
				 &raw mut text_area_height);                   // C 3204

		    (*view).y = y;                                            // C 3206

		    clip_top = 0;                                             // C 3208
		    clip_bottom = (*XXWIDGET ((*view).model)).height;      // C 3209

		    if y < text_area_y                                        // C 3211
		      { clip_top = text_area_y - y; }                         // C 3212

		    if (y + clip_bottom) > (text_area_y + text_area_height)   // C 3214
		      {
			clip_bottom -= (y + clip_bottom) - (text_area_y + text_area_height);   // C 3216
		      }

		    (*view).clip_top = clip_top;                              // C 3219
		    (*view).clip_bottom = clip_bottom;                        // C 3220

		    /* This means the view has moved offscreen.  Unmap
		       it and hide it here.  */
		    if ((*view).clip_bottom - (*view).clip_top) <= 0          // C 3224
		      {
			(*view).hidden = true;                                  // C 3226
			gtk_widget_hide (tem);                                  // C 3227
		      }
		    else
		      {
			gtk_fixed_move (FRAME_GTK_WIDGET! (f) as *mut GtkFixed,   // C 3231
				tem, (*view).x + (*view).clip_left,              // C 3232
				(*view).y + (*view).clip_top);                   // C 3233
			gtk_widget_set_size_request (tem,                       // C 3234
						     (*view).clip_right - (*view).clip_left,
						     (*view).clip_bottom - (*view).clip_top);   // C 3235
			gtk_widget_queue_allocate (tem);                        // C 3236
		      }
		  }
	    }
	  iter = (*iter).next;                                             // C 3167's step
	}

    g_list_free (children);                                             // C 3242
  }

  /* Cursor off.  Will be switched on again in x_update_window_end.  */
  gui_clear_cursor (w);                                                 // C 3246

  {
    let mut src_rect: cairo_rectangle_t = cairo_rectangle_t             // C 3249
      { x: x as c_double, y: from_y as c_double, width: width as c_double,
	height: height as c_double };
    let mut dst_rect: cairo_rectangle_t = cairo_rectangle_t             // C 3250
      { x: x as c_double, y: to_y as c_double, width: width as c_double,
	height: height as c_double };
    pgtk_copy_bits (f, &raw mut src_rect, &raw mut dst_rect);           // C 3251
  }

  unblock_input ();                                                     // C 3254
}

/* Icons.  */

/// C 3259-3326: `static bool pgtk_bitmap_icon (struct frame *f, Lisp_Object file)` - C's whole body
/// is `#if 0` (C 3267-3324), dead text in every configuration, so the port carries C's text inside
/// a comment (the `cm.rs` 86 reading; the `src/w32term.rs` 133-139 precedent) and only C 3325's
/// `return false;` is compiled.  `f` and `file` are therefore unused in the live text (`let _ =`
/// keeps the names).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_bitmap_icon (f: *mut frame, file: Lisp_Object) -> bool
{
  let _ = f;
  let _ = file;
  /* C 3267-3324 is `#if 0`: "This code has never worked anyway for the reason that Wayland
     uses icons set within desktop files, and has been disabled because leaving it intact would
     require image.c to retain a reference to a GdkPixbuf (which are no longer used) within new
     bitmaps."  C's own preprocessor never compiles it, so the port keeps C's text as a comment
     (the `cm.rs` 86 reading).

#if 0
  ptrdiff_t bitmap_id;

  if (FRAME_GTK_WIDGET (f) == 0)
    return true;

  /* Free up our existing icon bitmap and mask if any.  */
  if (f->output_data.pgtk->icon_bitmap > 0)
    image_destroy_bitmap (f, f->output_data.pgtk->icon_bitmap);
  f->output_data.pgtk->icon_bitmap = 0;

  if (STRINGP (file))
    {
      /* Use gtk_window_set_icon_from_file () if available,
	 It's not restricted to bitmaps */
      if (xg_set_icon (f, file))
	return false;
      bitmap_id = image_create_bitmap_from_file (f, file);
    }
  else
    {
      /* Create the GNU bitmap and mask if necessary.  */
      if (FRAME_DISPLAY_INFO (f)->icon_bitmap_id < 0)
	{
	  ptrdiff_t rc = -1;

          if (xg_set_icon (f, xg_default_icon_file)
              || xg_set_icon_from_xpm_data (f, gnu_xpm_bits))
            {
              FRAME_DISPLAY_INFO (f)->icon_bitmap_id = -2;
              return false;
            }

	  /* If all else fails, use the (black and white) xbm image. */
	  if (rc == -1)
	    {
              rc = image_create_bitmap_from_data (f,
                                                  (char *) gnu_xbm_bits,
                                                  gnu_xbm_width,
                                                  gnu_xbm_height);
	      if (rc == -1)
		return true;

	      FRAME_DISPLAY_INFO (f)->icon_bitmap_id = rc;
	    }
	}

      /* The first time we create the GNU bitmap and mask,
	 this increments the ref-count one extra time.
	 As a result, the GNU bitmap and mask are never freed.
	 That way, we don't have to worry about allocating it again.  */
      image_reference_bitmap (f, FRAME_DISPLAY_INFO (f)->icon_bitmap_id);

      bitmap_id = FRAME_DISPLAY_INFO (f)->icon_bitmap_id;
    }

  f->output_data.pgtk->icon_bitmap = bitmap_id;
#endif /* 0 */
*/
  return false;                                                         // C 3325
}

/* <gtk/gtkwindow.h>: `void gtk_window_set_icon (GtkWindow *window, GdkPixbuf *icon);` - the
   C 3337 call.  `include/gtk.rs` carries the prototype's doc comment (1423-1424) and no
   `pub fn` row for it, so the unit declares it here (the GTK segment's convention: the header
   alone, no line, because this machine has no GTK tree).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_window_set_icon (window: *mut GtkWindow, icon: *mut GdkPixbuf);
}

/* Make the x-window of frame F use a rectangle with text.
   Use ICON_NAME as the text.  */

/// C 3332-3342: `bool pgtk_text_icon (struct frame *f, const char *icon_name)` - a **non-`static`**
/// C definition, so the port carries `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2); the
/// two GTK calls are `include/gtk.rs`'s rows and C's two `GTK_WINDOW` cast macros are the port's
/// pointer casts (P-72 rule 3).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_text_icon (f: *mut frame, icon_name: *const c_char) -> bool
{
  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()                            // C 3335
    {
      gtk_window_set_icon (FRAME_GTK_OUTER_WIDGET! (f) as *mut GtkWindow,   // C 3337
			   core::ptr::null_mut ());
      gtk_window_set_title (FRAME_GTK_OUTER_WIDGET! (f) as *mut GtkWindow,  // C 3338
			    icon_name);
    }

  return false;                                                         // C 3341
}

/***********************************************************************
			    Starting and ending an update
 ***********************************************************************/

/* Start an update of frame F.  This function is installed as a hook
   for update_begin, i.e. it is called when update_begin is called.
   This function is called prior to calls to x_update_window_begin for
   each window being updated.  Currently, there is nothing to do here
   because all interesting stuff is done on a window basis.  */

/// C 3354-3358: `static void pgtk_update_begin (struct frame *f)` - one call to
/// `pgtk_clear_under_internal_border` (C 5047's definition, in the remainder - a name this slice
/// is the first to reach, so it is a registered class of its own in this ledger).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_update_begin (f: *mut frame)
{
  pgtk_clear_under_internal_border (f);                                 // C 3357
}

/* Draw a vertical window border from (x,y0) to (x,y1)  */

/// C 3362-3379: `static void pgtk_draw_vertical_window_border (struct window *w, int x, int y0,
/// int y1)` - `FACE_FROM_ID_OR_NULL` is this unit's own copy (frame.h 1656's inline definition,
/// which C pastes into every including unit: the `src/term.rs` 372-377 form, added beside
/// `FRAME_FACE_CACHE`/`FACE_FROM_ID` with this slice), and C's `if (face)` truth test is the
/// pointer's null test.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_draw_vertical_window_border (w: *mut window, x: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 3365
  let face: *mut face;                                                  // C 3366
  let cr: *mut cairo_t;                                                 // C 3367

  cr = pgtk_begin_cr_clip (f);                                          // C 3369

  face = FACE_FROM_ID_OR_NULL (f, VERTICAL_BORDER_FACE_ID);             // C 3371
  if !face.is_null ()                                                   // C 3372
    { pgtk_set_cr_source_with_color (f, (*face).foreground, false); }   // C 3373

  cairo_rectangle (cr, x as c_double, y0 as c_double, 1.0, (y1 - y0) as c_double);   // C 3375
  cairo_fill (cr);                                                      // C 3376

  pgtk_end_cr_clip (f);                                                 // C 3378
}

/* C: dispextern.h 3719: `extern bool buffer_flipping_blocked_p (void);` - the C 3459 call
   (`src/xdisp.rs` defines it; the `src/androidterm.rs` 167 and `src/haikuterm.rs` 180 copies of
   the same use-site declaration).  <gtk/gtkwidget.h>: `void gtk_widget_queue_draw (GtkWidget
   *widget);` - the C 3462 call (the GTK segment's convention: the header alone, no line).  */
/* C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);` - the
   call `FRAME_MOUSE_UPDATE!` expands to (C 3458; `include/frame.rs` 1725 records that the
   name's declaration is owed to `include/dispextern.rs`, whose own note at 4050 keeps it in
   that mirror's "owed, not declared" list, so the unit declares it here).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn note_mouse_highlight (f: *mut c_void, x: c_int, y: c_int);
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn buffer_flipping_blocked_p () -> bool;
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_widget_queue_draw (widget: *mut GtkWidget);
}

/* Draw a window divider from (x0,y0) to (x1,y1)  */

/// C 3383-3442: `static void pgtk_draw_window_divider (struct window *w, int x0, int x1, int y0,
/// int y1)` - `FACE_FROM_ID_OR_NULL` is this unit's own copy (the twentieth slice), C's three
/// `face ? … : FRAME_FOREGROUND_PIXEL (f)` folds are the port's null tests,
/// `f->borders_respect_alpha_background` is the frame record's member (passed to every
/// `pgtk_set_cr_source_with_color` call), and C's `if`/`else if`/`else` chain selects the three
/// bands, the horizontal pair or the single rectangle.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_draw_window_divider (w: *mut window, x0: c_int, x1: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 3386
  let face: *mut face = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FACE_ID);   // C 3387
  let face_first: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);     // C 3388-3389
  let face_last: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);      // C 3390-3391
  let color: c_ulong = if !face.is_null () { (*face).foreground }       // C 3392
		       else { FRAME_FOREGROUND_PIXEL (f) };
  let color_first: c_ulong = if !face_first.is_null () { (*face_first).foreground }   // C 3393-3395
			     else { FRAME_FOREGROUND_PIXEL (f) };
  let color_last: c_ulong = if !face_last.is_null () { (*face_last).foreground }      // C 3396-3398
			    else { FRAME_FOREGROUND_PIXEL (f) };
  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 3399

  if y1 - y0 > x1 - x0 && x1 - x0 > 2                                  // C 3401
    /* Vertical.  */
    {
      pgtk_set_cr_source_with_color (f, color_first,                    // C 3404
		     borders_respect_alpha_background! (f));                 // C 3405
      cairo_rectangle (cr, x0 as c_double, y0 as c_double, 1.0,         // C 3406
		       (y1 - y0) as c_double);
      cairo_fill (cr);                                                  // C 3407
      pgtk_set_cr_source_with_color (f, color,                          // C 3408
		     borders_respect_alpha_background! (f));                 // C 3409
      cairo_rectangle (cr, (x0 + 1) as c_double, y0 as c_double,        // C 3410
		       (x1 - x0 - 2) as c_double, (y1 - y0) as c_double);
      cairo_fill (cr);                                                  // C 3411
      pgtk_set_cr_source_with_color (f, color_last,                     // C 3412
		     borders_respect_alpha_background! (f));                 // C 3413
      cairo_rectangle (cr, (x1 - 1) as c_double, y0 as c_double, 1.0,   // C 3414
		       (y1 - y0) as c_double);
      cairo_fill (cr);                                                  // C 3415
    }
  else if x1 - x0 > y1 - y0 && y1 - y0 > 3                             // C 3417
    /* Horizontal.  */
    {
      pgtk_set_cr_source_with_color (f, color_first,                    // C 3420
		     borders_respect_alpha_background! (f));                 // C 3421
      cairo_rectangle (cr, x0 as c_double, y0 as c_double,              // C 3422
		       (x1 - x0) as c_double, 1.0);
      cairo_fill (cr);                                                  // C 3423
      pgtk_set_cr_source_with_color (f, color,                          // C 3424
		     borders_respect_alpha_background! (f));                 // C 3425
      cairo_rectangle (cr, x0 as c_double, (y0 + 1) as c_double,        // C 3426
		       (x1 - x0) as c_double, (y1 - y0 - 2) as c_double);
      cairo_fill (cr);                                                  // C 3427
      pgtk_set_cr_source_with_color (f, color_last,                     // C 3428
		     borders_respect_alpha_background! (f));                 // C 3429
      cairo_rectangle (cr, x0 as c_double, (y1 - 1) as c_double,        // C 3430
		       (x1 - x0) as c_double, 1.0);
      cairo_fill (cr);                                                  // C 3431
    }
  else
    {
      pgtk_set_cr_source_with_color (f, color,                          // C 3435
		     borders_respect_alpha_background! (f));                 // C 3436
      cairo_rectangle (cr, x0 as c_double, y0 as c_double,              // C 3437
		       (x1 - x0) as c_double, (y1 - y0) as c_double);
      cairo_fill (cr);                                                  // C 3438
    }

  pgtk_end_cr_clip (f);                                                 // C 3441
}

/* End update of frame F.  This function is installed as a hook in
   update_end.  */

/// C 3447-3452: `static void pgtk_update_end (struct frame *f)` - C's
/// `MOUSE_HL_INFO (f)->mouse_face_defer = false;` is the mirror's `MOUSE_HL_INFO!` accessor plus
/// its `set_mouse_face_defer` writer (the record's bitfield has function pairs rather than a
/// `*_bits!` macro).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_update_end (f: *mut frame)
{
  /* Mouse highlight may be displayed again.  */
  set_mouse_face_defer (MOUSE_HL_INFO! (f), false);                     // C 3451
}

/// C 3454-3465: `static void pgtk_frame_up_to_date (struct frame *f)` - `FRAME_MOUSE_UPDATE!` is
/// the frame mirror's macro, `buffer_flipping_blocked_p` (dispextern.h 3719, defined in
/// `src/xdisp.rs`) is a use-site declaration (the `src/androidterm.rs` 167 and
/// `src/haikuterm.rs` 180 copies), `flip_cr_context` is this file's own C 271 definition, and
/// `gtk_widget_queue_draw` (C 3462) follows the GTK segment's convention.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_frame_up_to_date (f: *mut frame)
{
  block_input ();                                                       // C 3457
  FRAME_MOUSE_UPDATE! (f);                                              // C 3458
  if !buffer_flipping_blocked_p ()                                      // C 3459
    {
      flip_cr_context (f);                                              // C 3461
      gtk_widget_queue_draw (FRAME_GTK_WIDGET! (f));                    // C 3462
    }
  unblock_input ();                                                     // C 3464
}

/// C 3509: `XFRAME (frame)->mouse_moved = false;` - `frame.h` 434's `bool_bf mouse_moved : 1`,
/// whose shift `include/frame.rs` 529-537 names for this arm (A: x11/pgtk/android/w32); the
/// mirror carries the shift and no accessor, so the unit declares the member's own writer - the
/// `src/androidterm.rs` 234-238 copy of the same macro (`bits1` is the measured word 1; that copy
/// carries both arms, this one the write C's body performs, and the reader lands with the first
/// C site that reads the flag).
#[cfg (feature = "have_pgtk")]
macro_rules! mouse_moved {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_MOUSE_MOVED_SHIFT))
                  | ((($v) as u32) << FRAME_MOUSE_MOVED_SHIFT) };
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_SHIFT_MASK = 1 << 0, …, GDK_META_MASK = 1 << 28 }
/// GdkModifierType;` - the type of C 3497's local `mask`, which the C 3523/3549 calls write
/// through a pointer (`src/xwidget.rs` 260-263 carries the same row; its values are placeholders,
/// there is no GDK header on this machine).  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
pub type GdkModifierType = c_int;

/* C: dispextern.h 4006: `extern bool gui_mouse_grabbed (Display_Info *);` - the C 3513 call
   (`src/frame.rs` 6420 defines it; dispextern.h 152's `typedef struct pgtk_display_info
   Display_Info;` is the record this unit uses, which the mirror's own alias table still defers).
   dispextern.h 3586: `void remember_mouse_glyph (struct frame *, int, int, NativeRectangle *);` -
   the C 3554 call, with pgtkgui.h 101's `#define NativeRectangle XRectangle`.  The mirror carries
   neither, so both are declared here at the use site (P-90 ①, the `src/androidterm.rs` 163-171
   shape).  */
/* <gdk/gdkwindow.h>: `GdkWindow *gdk_window_get_device_position (GdkWindow *window, GdkDevice
   *device, gint *x, gint *y, GdkModifierType *mask);` - the C 3523/3549 calls (the GTK segment's
   convention: the header alone, no line).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gui_mouse_grabbed (dpyinfo: *mut pgtk_display_info) -> bool;
  pub fn remember_mouse_glyph (f: *mut frame, x: c_int, y: c_int,
			       rectangle: *mut XRectangle);
  pub fn gdk_window_get_device_position (window: *mut GdkWindow, device: *mut GdkDevice,
					 x: *mut c_int, y: *mut c_int,
					 mask: *mut GdkModifierType) -> *mut GdkWindow;
}

/* Return the current position of the mouse.
   *FP should be a frame which indicates which display to ask about.

   If the mouse movement started in a scroll bar, set *FP, *BAR_WINDOW,
   and *PART to the frame, window, and scroll bar part that the mouse
   is over.  Set *X and *Y to the portion and whole of the mouse's
   position on the scroll bar.

   If the mouse movement started elsewhere, set *FP to the frame the
   mouse is on, *BAR_WINDOW to nil, and *X and *Y to the character cell
   the mouse is over.

   Set *TIMESTAMP to the server time-stamp for the time at which the mouse
   was at this position.

   Don't store anything if we don't have a valid set of values to report.

   This clears the mouse_moved flag, so we can wait for the next mouse
   movement.  */

/// C 3487-3567: `static void pgtk_mouse_position (struct frame **fp, int insist, Lisp_Object
/// *bar_window, enum scroll_bar_part *part, Lisp_Object *x, Lisp_Object *y, Time *timestamp)` -
/// `insist` is unused in C's body (`let _ =` keeps the name), `dpyinfo->gdpy` is the record's own
/// field (the `FRAME_X_DISPLAY!` payload), the C 3494/3497 locals `win_x`/`win_y`/`mask` are
/// filled by the callee through pointers and are zero-initialized here (PORTING.md 1.5, Deviation
/// 10's class), the C 3506 `FOR_EACH_FRAME` becomes the mirror's macro with C's body as its last
/// argument, and C 3526's `pgtk_any_window_to_frame` is this file's own C 118 forward-declared
/// definition (C 7314, in the remainder - the ledger's registered class).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_mouse_position (fp: *mut *mut frame, insist: c_int,
			       bar_window: *mut Lisp_Object, part: *mut scroll_bar_part,
			       x: *mut Lisp_Object, y: *mut Lisp_Object,
			       timestamp: *mut Time)
{
  let _ = insist;                                                       // C 3488
  let mut f1: *mut frame;                                               // C 3492
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (*fp);      // C 3493
  let mut win_x: c_int = 0;                                             // C 3494
  let mut win_y: c_int = 0;                                             // C 3494
  let mut seat: *mut GdkSeat;                                           // C 3495
  let mut device: *mut GdkDevice;                                       // C 3496
  let mut mask: GdkModifierType = 0;                                    // C 3497
  let mut win: *mut GdkWindow;                                          // C 3498
  let mut return_frame_flag: bool = false;                              // C 3499

  block_input ();                                                       // C 3501

  let mut frame: Lisp_Object;                                           // C 3503
  let mut tail: Lisp_Object;                                            // C 3503

  /* Clear the mouse-moved flag for every frame on this display.  */
  FOR_EACH_FRAME! (tail, frame, {                                       // C 3506
    if FRAME_PGTK_P (XFRAME (frame))                                    // C 3507
      && FRAME_X_DISPLAY! (XFRAME (frame)) == FRAME_X_DISPLAY! (*fp)    // C 3508
      { mouse_moved! (XFRAME (frame), false); }                         // C 3509
  });

  (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();            // C 3511

  if gui_mouse_grabbed (dpyinfo)                                        // C 3513
    && (!EQ (track_mouse, Qdropping ())                                 // C 3514
	&& !EQ (track_mouse, Qdrag_source ()))                          // C 3515
    { f1 = (*dpyinfo).last_mouse_frame; }                               // C 3516
  else
    {
      f1 = *fp;                                                         // C 3519
      win = gtk_widget_get_window (FRAME_GTK_WIDGET! (*fp));            // C 3520
      seat = gdk_display_get_default_seat ((*dpyinfo).gdpy);            // C 3521
      device = gdk_seat_get_pointer (seat);                             // C 3522
      win = gdk_window_get_device_position (win, device, &mut win_x,    // C 3523
					    &mut win_y, &mut mask);     // C 3524
      if !win.is_null ()                                                // C 3525
	{ f1 = pgtk_any_window_to_frame (win); }                        // C 3526
      else
	{
	  f1 = SELECTED_FRAME ();                                       // C 3529

	  if !FRAME_PGTK_P (f1)                                         // C 3531
	    { f1 = (*dpyinfo).last_mouse_frame; }                       // C 3532

	  return_frame_flag = EQ (track_mouse, Qdrag_source ());        // C 3534
	}
    }

  /* F1 can be a terminal frame.  (Bug#50322) */
  if f1.is_null () || !FRAME_PGTK_P (f1)                                // C 3539
    {
      unblock_input ();                                                 // C 3541
      return;                                                           // C 3542
    }

  win = gtk_widget_get_window (FRAME_GTK_WIDGET! (f1));                 // C 3545
  seat = gdk_display_get_default_seat ((*dpyinfo).gdpy);                // C 3546
  device = gdk_seat_get_pointer (seat);                                 // C 3547

  win = gdk_window_get_device_position (win, device, &mut win_x,        // C 3549
					&mut win_y, &mut mask);         // C 3550

  if !f1.is_null ()                                                     // C 3552
    {
      remember_mouse_glyph (f1, win_x, win_y,                           // C 3554
			    &mut (*dpyinfo).last_mouse_glyph);          // C 3555
      (*dpyinfo).last_mouse_glyph_frame = f1;                           // C 3556

      *bar_window = Qnil ();                                            // C 3558
      *part = 0;                                                        // C 3559
      *fp = if !return_frame_flag { f1 } else { core::ptr::null_mut () };   // C 3560
      XSETINT! (*x, win_x as EMACS_INT);                                // C 3561
      XSETINT! (*y, win_y as EMACS_INT);                                // C 3562
      *timestamp = (*dpyinfo).last_mouse_movement_time;                 // C 3563
    }

  unblock_input ();                                                     // C 3566
}

/* Fringe bitmaps.  */

/// C 3571: `static int max_fringe_bmp = 0;`
#[cfg (feature = "have_pgtk")]
static mut max_fringe_bmp: c_int = 0;
/// C 3572: `static cairo_pattern_t **fringe_bmp = 0;` - C's `0` is the null pattern-table pointer
/// the first `xrealloc` replaces.
#[cfg (feature = "have_pgtk")]
static mut fringe_bmp: *mut *mut cairo_pattern_t = core::ptr::null_mut ();

/// C: cairo.h 532-535: `typedef struct _cairo_rectangle_int { int x, y; int width, height; }
/// cairo_rectangle_int_t;` - C 3633's local `rect`.  The machine *has* this header
/// (`/opt/homebrew/include/cairo/cairo.h`), so the record's fields are a read of it; C assigns the
/// four members one at a time, which Rust's "no partial initialisation" rule has no counterpart
/// for (PORTING.md 1.13), so the port zero-initialises it first (Deviation 45).
#[cfg (feature = "have_pgtk")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct cairo_rectangle_int_t
{
  pub x: c_int,        // cairo.h 533: `int x, y;`
  pub y: c_int,
  pub width: c_int,    // cairo.h 534: `int width, height;`
  pub height: c_int,
}

/* C: cairo.h 2710-2712: `cairo_surface_t *cairo_image_surface_create (cairo_format_t format, int
   width, int height);` - the C 3594 call.  cairo.h 2738: `int cairo_image_surface_get_stride
   (cairo_surface_t *surface);` - the C 3595 call.  cairo.h 2726: `unsigned char
   *cairo_image_surface_get_data (cairo_surface_t *surface);` - the C 3596 call.  cairo.h 2659:
   `void cairo_surface_mark_dirty (cairo_surface_t *surface);` - the C 3604 call.  cairo.h 2923:
   `cairo_pattern_t *cairo_pattern_create_for_surface (cairo_surface_t *surface);` - the C 3605
   call.  cairo.h 2940: `void cairo_pattern_destroy (cairo_pattern_t *pattern);` - the C 3622 call.
   The seventh, `cairo_surface_destroy` (C 3606), is `include/gtk.rs` 338; the six above are this
   unit's own rows, the P-88 ① convention this file's cairo segment uses.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_image_surface_create (format: cairo_format_t, width: c_int, height: c_int)
				     -> *mut cairo_surface_t;
  pub fn cairo_image_surface_get_stride (surface: *mut cairo_surface_t) -> c_int;
  pub fn cairo_image_surface_get_data (surface: *mut cairo_surface_t) -> *mut c_uchar;
  pub fn cairo_surface_mark_dirty (surface: *mut cairo_surface_t);
  pub fn cairo_pattern_create_for_surface (surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
  pub fn cairo_pattern_destroy (pattern: *mut cairo_pattern_t);
}

/// C 3574-3611: `static void pgtk_define_fringe_bitmap (int which, unsigned short *bits, int h,
/// int wd)` - the growth arm (`which + 20`, the `xrealloc` whose result C assigns back, the
/// `while` that nulls the new tail), then the A1 surface: `cairo_image_surface_create`, the
/// stride/data pair, the per-row `*((unsigned short *) data) = bits[i]` store with `data +=
/// stride`, and the mark-dirty / create-for-surface / destroy triple.  Rust's statics need the
/// `unsafe` the callers already carry, and C's `fringe_bmp[i]` reads and writes are the port's
/// `*fringe_bmp.offset (i as isize)`.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_define_fringe_bitmap (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)
{
  let mut i: c_int;                                                     // C 3577
  let mut stride: c_int;                                                // C 3577
  let mut surface: *mut cairo_surface_t;                                // C 3578
  let mut data: *mut c_uchar;                                           // C 3579
  let mut pattern: *mut cairo_pattern_t;                                // C 3580

  if which >= max_fringe_bmp                                           // C 3582
    {
      i = max_fringe_bmp;                                               // C 3584
      max_fringe_bmp = which + 20;                                      // C 3585
      fringe_bmp = xrealloc (fringe_bmp as *mut c_void,                 // C 3586-3587
			     core::mem::size_of::<*mut cairo_pattern_t> ()
			     * max_fringe_bmp as usize) as *mut *mut cairo_pattern_t;
      while i < max_fringe_bmp                                          // C 3588
	{
	  *fringe_bmp.offset (i as isize) = core::ptr::null_mut ();     // C 3589
	  i += 1;                                                       // C 3589's `i++`
	}
    }

  block_input ();                                                       // C 3592

  surface = cairo_image_surface_create (CAIRO_FORMAT_A1, wd, h);        // C 3594
  stride = cairo_image_surface_get_stride (surface);                    // C 3595
  data = cairo_image_surface_get_data (surface);                        // C 3596

  i = 0;                                                                // C 3598
  while i < h
    {
      *(data as *mut c_ushort) = *bits.offset (i as isize);             // C 3600
      data = data.offset (stride as isize);                             // C 3601
      i += 1;                                                           // C 3598's `i++`
    }

  cairo_surface_mark_dirty (surface);                                   // C 3604
  pattern = cairo_pattern_create_for_surface (surface);                 // C 3605
  cairo_surface_destroy (surface);                                      // C 3606

  unblock_input ();                                                     // C 3608

  *fringe_bmp.offset (which as isize) = pattern;                        // C 3610
}

/// C 3613-3626: `static void pgtk_destroy_fringe_bitmap (int which)` - the early return past the
/// table's end, the guarded `cairo_pattern_destroy` pair and the closing null store.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_destroy_fringe_bitmap (which: c_int)
{
  if which >= max_fringe_bmp                                           // C 3616
    { return; }                                                        // C 3617

  if !(*fringe_bmp.offset (which as isize)).is_null ()                 // C 3619
    {
      block_input ();                                                  // C 3621
      cairo_pattern_destroy (*fringe_bmp.offset (which as isize));     // C 3622
      unblock_input ();                                                // C 3623
    }
  *fringe_bmp.offset (which as isize) = core::ptr::null_mut ();        // C 3625
}

/// C 3628-3645: `static void pgtk_clip_to_row (struct window *w, struct glyph_row *row, enum
/// glyph_row_area area, cairo_t *cr)` - `window_box`'s five outputs (`0` for the height is C's
/// null), the record's four assignments with C's two `max` calls, and the rectangle/clip pair.
/// `window_box` is this unit's own declaration (C 3123's call site), `WINDOW_TO_FRAME_PIXEL_Y!` is
/// `include/window.rs`'s macro and `max` is the mirror's C-`max` helper (the C 3125 spelling).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_clip_to_row (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
			    cr: *mut cairo_t)
{
  let mut window_x: c_int = 0;                                          // C 3632
  let mut window_y: c_int = 0;                                          // C 3632
  let mut window_width: c_int = 0;                                      // C 3632
  let mut rect: cairo_rectangle_int_t = cairo_rectangle_int_t { x: 0, y: 0, width: 0, height: 0 };   // C 3633

  window_box (w, area, &raw mut window_x, &raw mut window_y,            // C 3635
	      &raw mut window_width, core::ptr::null_mut ());

  rect.x = window_x;                                                    // C 3637
  rect.y = WINDOW_TO_FRAME_PIXEL_Y! (w, max (0, (*row).y as isize) as c_int);   // C 3638
  rect.y = max (rect.y as isize, window_y as isize) as c_int;           // C 3639
  rect.width = window_width;                                            // C 3640
  rect.height = (*row).visible_height;                                  // C 3641

  cairo_rectangle (cr, rect.x as c_double, rect.y as c_double,          // C 3643
		   rect.width as c_double, rect.height as c_double);
  cairo_clip (cr);                                                      // C 3644
}

/* C: dispextern.h 3711: `void gui_define_fringe_bitmap (struct frame *, int);` - the C 3676 call
   (`src/fringe.rs` defines it).  The mirror's extern block carries dispextern.h 3710's
   `max_used_fringe_bitmap` and not this line, so it is declared here at the use site.  */
/* <gtk/gtkeventbox.h>: `GtkWidget *gtk_event_box_new (void);` - the C 3708 call (the GTK
   segment's convention: the header alone, no line).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gui_define_fringe_bitmap (f: *mut frame, which: c_int);
  pub fn gtk_event_box_new () -> *mut GtkWidget;
}

/// C 3647-3689: `static void pgtk_draw_fringe_bitmap (struct window *w, struct glyph_row *row,
/// struct draw_fringe_bitmap_params *p)` - **the record's name is the module path, not C's bare
/// one**: `include/dispextern.rs` carries the five records of C 2159-3200 (`bidi_it`,
/// `draw_fringe_bitmap_params`, `composition_it`, `it`, `redisplay_interface`) inside
/// `pub mod dispextern_unit_names`, "because every unit that includes the mirror declares its own
/// copy of each", and this unit is no C record's owner - so the type, its two `bool_bf` accessors
/// (`get_overlay_p`/`get_cursor_p`) and the `max_used_fringe_bitmap` extern are named the way
/// `src/w32term.rs` 1489 does (that unit records the reading: the unqualified name is `E0425` in
/// the flip arm, `src/abi_shim.rs` 2087 being the other precedent).  `fringe_bmp` is this unit's
/// C 3572 object (the C 3684 store's own table), `pgtk_cr_draw_image` is this file's C 2205 definition,
/// and C's `Emacs_GC gcv;` is zero-initialised because C fills two of its members one at a time
/// (PORTING.md 1.13, Deviation 46).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_draw_fringe_bitmap (w: *mut window, row: *mut glyph_row,
				   p: *mut dispextern_unit_names::draw_fringe_bitmap_params)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 3651
  let face: *mut face = (*p).face;                                      // C 3652

  let cr: *mut cairo_t = pgtk_begin_cr_clip (f);                        // C 3654

  /* Must clip because of partially visible lines.  */
  pgtk_clip_to_row (w, row, ANY_AREA, cr);                              // C 3657

  if (*p).bx >= 0 && !dispextern_unit_names::get_overlay_p (p)                                 // C 3659
    { fill_background_by_face (f, face, (*p).bx, (*p).by, (*p).nx,      // C 3660
			       (*p).ny); }
  if (*p).which != 0                                                    // C 3662
      && (*p).which < max_fringe_bmp                                    // C 3663
      && (*p).which < dispextern_unit_names::max_used_fringe_bitmap                            // C 3664
    {
      let mut gcv: Emacs_GC = Emacs_GC { foreground: 0, background: 0 };   // C 3666

      if (*fringe_bmp.offset ((*p).which as isize)).is_null ()          // C 3668
	{
	  /* This fringe bitmap is known to fringe.c, but lacks the
	     cairo_pattern_t pattern which shadows that bitmap.  This
	     is typical to define-fringe-bitmap being called when the
	     selected frame was not a GUI frame, for example, when
	     packages that define fringe bitmaps are loaded by a
	     daemon Emacs.  Create the missing pattern now.  */
	  gui_define_fringe_bitmap (f, (*p).which);                     // C 3676
	}

      gcv.foreground = if dispextern_unit_names::get_cursor_p (p)                              // C 3679
	{
	  if dispextern_unit_names::get_overlay_p (p)                                          // C 3680
	    { (*face).background }                                      // C 3680
	  else
	    { (*FRAME_X_OUTPUT! (f)).cursor_color }                     // C 3681
	}
      else
	{ (*face).foreground };                                         // C 3682
      gcv.background = (*face).background;                              // C 3683
      pgtk_cr_draw_image (f, &mut gcv,                                  // C 3684
			  *fringe_bmp.offset ((*p).which as isize), 0, (*p).dh,
			  (*p).wd, (*p).h, (*p).x, (*p).y, dispextern_unit_names::get_overlay_p (p));   // C 3685
    }

  pgtk_end_cr_clip (f);                                                 // C 3688
}

/// C 3691: `static struct atimer *hourglass_atimer = NULL;`
#[cfg (feature = "have_pgtk")]
static mut hourglass_atimer: *mut atimer = core::ptr::null_mut ();
/// C 3692: `static int hourglass_enter_count = 0;`
#[cfg (feature = "have_pgtk")]
static mut hourglass_enter_count: c_int = 0;

/// C 3694-3698: `static void hourglass_cb (struct atimer *timer)` - **C's body is empty**, so the
/// port keeps it empty (the `frame_set_mouse_pixel_position` (C 401) shape); the function's
/// *address* is what C hands to `start_atimer`, which is why it is `extern "C"` (atimer.h 44's
/// `atimer_callback`, `include/atimer.rs` 29).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn hourglass_cb (timer: *mut atimer)
{
}

/// C 3700-3727: `static void pgtk_show_hourglass (struct frame *f)` - the four GTK calls that
/// build the `GDK_INPUT_ONLY` window (`GTK_FIXED!` is `include/gtk.rs` 1677's cast macro,
/// `gtk_event_box_new` the use-site declaration above), the cursor call with C's `Emacs_Cursor`
/// pointer cast (P-72 rule 3), and the timer arm.  C's `hourglass_enter_count++ == 0` keeps the
/// post-increment inside the condition (`src/keyboard.rs` 3889's shape): the increment must happen
/// on the test too, so the port spells it as a block expression evaluated once per test.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_show_hourglass (f: *mut frame)
{
  let x: *mut pgtk_output = FRAME_X_OUTPUT! (f);                        // C 3703
  if !(*x).hourglass_widget.is_null ()                                  // C 3704
    { gtk_widget_destroy ((*x).hourglass_widget); }                     // C 3705

  /* This creates a GDK_INPUT_ONLY window.  */
  (*x).hourglass_widget = gtk_event_box_new ();                         // C 3708
  gtk_widget_set_has_window ((*x).hourglass_widget, true as gboolean);  // C 3709
  gtk_fixed_put (GTK_FIXED! (FRAME_GTK_WIDGET! (f)), (*x).hourglass_widget, 0, 0);   // C 3710
  gtk_widget_show ((*x).hourglass_widget);                              // C 3711
  gtk_widget_set_size_request ((*x).hourglass_widget, 30000, 30000);    // C 3712
  gdk_window_raise (gtk_widget_get_window ((*x).hourglass_widget));     // C 3713
  gdk_window_set_cursor (gtk_widget_get_window ((*x).hourglass_widget), // C 3714
			 (*x).hourglass_cursor as *mut GdkCursor);      // C 3715

  /* For cursor animation, we receive signals, set pending_signals,
     and wait for the signal handler to run.  */
  if { hourglass_enter_count += 1; hourglass_enter_count - 1 == 0 }     // C 3719
    {
      let ts: timespec = make_timespec (0, 50 * 1000 * 1000);           // C 3721
      if !hourglass_atimer.is_null ()                                   // C 3722
	{ cancel_atimer (hourglass_atimer); }                           // C 3723
      hourglass_atimer = start_atimer (ATIMER_CONTINUOUS, ts,           // C 3724-3725
				       Some (hourglass_cb), core::ptr::null_mut ());
    }
}

/// C 3729-3746: `static void pgtk_hide_hourglass (struct frame *f)` - C's `--hourglass_enter_count
/// == 0` is the pre-decrement (the new value is the one tested), so the port spells the pair as one
/// block expression; the timer's cancel pair and the widget's destroy/null pair follow.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_hide_hourglass (f: *mut frame)
{
  let x: *mut pgtk_output = FRAME_X_OUTPUT! (f);                        // C 3732
  if { hourglass_enter_count -= 1; hourglass_enter_count == 0 }         // C 3733
    {
      if !hourglass_atimer.is_null ()                                   // C 3735
	{
	  cancel_atimer (hourglass_atimer);                             // C 3737
	  hourglass_atimer = core::ptr::null_mut ();                    // C 3738
	}
    }
  if !(*x).hourglass_widget.is_null ()                                  // C 3741
    {
      gtk_widget_destroy ((*x).hourglass_widget);                       // C 3743
      (*x).hourglass_widget = core::ptr::null_mut ();                   // C 3744
    }
}

/* Flushes changes to display.  */

/// C 3748-3752: `static void pgtk_flush_display (struct frame *f)` - **C's body is empty**, kept
/// empty (the same shape as `hourglass_cb` above and C 401's `frame_set_mouse_pixel_position`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_flush_display (f: *mut frame)
{
}

/* C: dispextern.h 3636-3638: `extern void gui_get_glyph_overhangs (struct glyph *, struct frame *,
   int *, int *);` - the C 3768 entry.  dispextern.h 3639: `extern void gui_produce_glyphs (struct
   it *);` - C 3758.  dispextern.h 3641-3642 / 3643-3644: `extern void gui_write_glyphs (struct
   window *, struct glyph_row *, struct glyph *, enum glyph_row_area, int);` and the
   `gui_insert_glyphs` pair - C 3759/3760.  dispextern.h 3645-3646: `extern void
   gui_clear_end_of_line (struct window *, struct glyph_row *, enum glyph_row_area, int);` -
   C 3761.  dispextern.h 3647-3648: `extern void gui_fix_overlapping_area (struct window *, struct
   glyph_row *, enum glyph_row_area, int);` - C 3769.  `gui_clear_window_mouse_face` (C 3767) is
   `src/xdisp.rs` 31968's definition.  `include/gtkutil.rs` carries none of the seven, so all six
   are declared here at the use site with C's own prototypes (the `src/androidterm.rs` 163-171
   shape), and `it` takes the module path the record lives in.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gui_get_glyph_overhangs (glyph: *mut glyph, f: *mut frame, left: *mut c_int,
				  right: *mut c_int);
  pub fn gui_produce_glyphs (it: *mut dispextern_unit_names::it);
  pub fn gui_write_glyphs (w: *mut window, row: *mut glyph_row, string: *mut glyph,
			   area: glyph_row_area, len: c_int);
  pub fn gui_insert_glyphs (w: *mut window, row: *mut glyph_row, start: *mut glyph,
			    area: glyph_row_area, len: c_int);
  pub fn gui_clear_end_of_line (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				x: c_int);
  pub fn gui_fix_overlapping_area (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				   overlaps: c_int);
  pub fn gui_clear_window_mouse_face (w: *mut window);                  /* C 3767's entry;
									   dispextern.h 3689 */
}

/* C 3754: `extern frame_parm_handler pgtk_frame_parm_handlers[];` - the table C *defines* in
   pgtkfns.c 963-1015, which the port defines in `src/pgtkfns.rs` 1919 as
   `[frame_parm_handler; 51]` with that unit's own copy of dispextern.h 3063's typedef (the
   `src/frame.rs` 5011-5014 precedent; `include/dispextern.rs` carries no alias - its
   `redisplay_interface.frame_parm_handlers` is `*mut c_void`, which `include/w32term.rs` 46/319-355
   records for its own handler array).  The length and the element type are the *definition's*
   measurement, taken here so the declaration and the definition agree; C's declaration is an
   array, so this one is too.  */
#[cfg (feature = "have_pgtk")]
type frame_parm_handler =
  Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object)>;
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub static mut pgtk_frame_parm_handlers: [frame_parm_handler; 51];
}

/// C 3756-3785: `static struct redisplay_interface pgtk_redisplay_interface = { … }` - **the
/// first redisplay-interface object in the port**, so its two decisions are written down here
/// (Deviation 47).  ⑴ C's initializer is *positional* and Rust has none: the port writes a
/// named-field literal, and the names come from the mirror's record
/// (`dispextern_unit_names::redisplay_interface`, C 3069-3200's 13 base fields plus the 15
/// `have_window_system` hooks).  Measured: C's initializer has exactly **28 entries** and the
/// record exactly **28 fields**, in the same order, so the values land in C's slots and nothing
/// is left to C's implicit zeroing.  ⑵ An entry's type is the *field's* type, and six of them
/// are `*mut c_void` where the port's function takes `*mut frame`/`*mut window` (the mirror
/// spells the incomplete-type parameters C's prototypes have): where the declaration already has
/// the field's type the entry is `Some (name)`, and where it does not the item's address is
/// converted with the `G_CALLBACK`-class transmute this ledger already documents (Deviation 11) -
/// a direct `as` is `E0605`, measured.  C's three `NULL` entries are `None` (C 3764-3765's two
/// commented-out hooks and C 3781's `shift_glyphs_for_insert`).
#[cfg (feature = "have_pgtk")]
static mut pgtk_redisplay_interface: dispextern_unit_names::redisplay_interface =
  dispextern_unit_names::redisplay_interface {
    frame_parm_handlers: &raw mut pgtk_frame_parm_handlers as *mut c_void,   // C 3757
    produce_glyphs: Some (gui_produce_glyphs),                               // C 3758
    write_glyphs: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, string: *mut glyph, area: c_int, len: c_int)> (gui_write_glyphs as *const ()) }),       // C 3759
    insert_glyphs: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, start: *mut glyph, area: c_int, len: c_int)> (gui_insert_glyphs as *const ()) }),      // C 3760
    clear_end_of_line: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, x: c_int)> (gui_clear_end_of_line as *const ()) }),                   // C 3761
    scroll_run_hook: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, run: *mut run)> (pgtk_scroll_run as *const ()) }),                                                       // C 3762
    after_update_window_line_hook: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, desired_row: *mut glyph_row)> (pgtk_after_update_window_line as *const ()) }),               // C 3763
    update_window_begin_hook: None,                                          // C 3764 (`NULL`)
    update_window_end_hook: None,                                            // C 3765 (`NULL`)
    flush_display: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void)> (pgtk_flush_display as *const ()) }),                                                                     // C 3766
    clear_window_mouse_face: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void)> (gui_clear_window_mouse_face as *const ()) }),                                                   // C 3767
    get_glyph_overhangs: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (glyph: *mut glyph, f: *mut c_void, left: *mut c_int, right: *mut c_int)> (gui_get_glyph_overhangs as *const ()) }),     // C 3768
    fix_overlapping_area: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, overlaps: c_int)> (gui_fix_overlapping_area as *const ()) }),        // C 3769
    draw_fringe_bitmap: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (*mut c_void, *mut glyph_row, *mut dispextern_unit_names::draw_fringe_bitmap_params)> (pgtk_draw_fringe_bitmap as *const ()) }),   // C 3770
    define_fringe_bitmap: Some (pgtk_define_fringe_bitmap),                  // C 3771
    destroy_fringe_bitmap: Some (pgtk_destroy_fringe_bitmap),                // C 3772
    compute_glyph_string_overhangs: Some (pgtk_compute_glyph_string_overhangs),   // C 3773
    draw_glyph_string: Some (pgtk_draw_glyph_string),                        // C 3774
    define_frame_cursor: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void, cursor: Emacs_Cursor)> (pgtk_define_frame_cursor as *const ()) }),                                     // C 3775
    clear_frame_area: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int)> (pgtk_clear_frame_area as *const ()) }),                // C 3776
    clear_under_internal_border: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void)> (pgtk_clear_under_internal_border as *const ()) }),                                          // C 3777
    draw_window_cursor: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, glyph_row: *mut glyph_row, x: c_int, y: c_int, cursor_type: text_cursor_kinds, cursor_width: c_int, on_p: bool, active_p: bool)> (pgtk_draw_window_cursor as *const ()) }),   // C 3778
    draw_vertical_window_border: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, x: c_int, y_0: c_int, y_1: c_int)> (pgtk_draw_vertical_window_border as *const ()) }),        // C 3779
    draw_window_divider: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (w: *mut c_void, x_0: c_int, x_1: c_int, y_0: c_int, y_1: c_int)> (pgtk_draw_window_divider as *const ()) }),           // C 3780
    shift_glyphs_for_insert: None,                                           // C 3781 (`NULL`)
    show_hourglass: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void)> (pgtk_show_hourglass as *const ()) }),                                                                   // C 3782
    hide_hourglass: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void)> (pgtk_hide_hourglass as *const ()) }),                                                                   // C 3783
    default_font_parameter: Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn (f: *mut c_void, parms: Lisp_Object)> (pgtk_default_font_parameter as *const ()) }),                                // C 3784
  };

/// C 3787-3798: `void pgtk_clear_frame (struct frame *f)` - a **non-`static`** C definition, so the
/// port carries `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2); `FRAME_DEFAULT_FACE` is
/// the frame mirror's macro, `XWINDOW`/`FRAME_ROOT_WINDOW` are `frame.rs`'s, `mark_window_cursors_off`
/// is `include/window.rs` 340's extern and `pgtk_clear_area` is `include/pgtkterm.rs` 687's row
/// (this unit defines it at C 5035's neighbour, in the remainder).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_clear_frame (f: *mut frame)
{
  if FRAME_DEFAULT_FACE! (f).is_null ()                                 // C 3790
    { return; }                                                         // C 3791

  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));           // C 3793

  block_input ();                                                       // C 3795
  pgtk_clear_area (f, 0, 0, FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f));   // C 3796
  unblock_input ();                                                     // C 3797
}

/// C 3800-3813: `static void recover_from_visible_bell (struct atimer *timer)` - the function whose
/// **address** C 3900 hands to `start_atimer`, so the port defines it `unsafe extern "C" fn`
/// (PORTING.md 1.4; atimer.h 44's `atimer_callback` is `include/atimer.rs` 29's type, so this
/// signature matches it exactly and the entry needs no conversion).  C's `timer->client_data` is
/// the atimer record's own field (`include/atimer.rs` 53, `void *`), and C's store into
/// `struct frame *f` is the pointer cast the port spells (P-72 rule 3, PORTING.md 1.4).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn recover_from_visible_bell (timer: *mut atimer)
{
  let f: *mut frame = (*timer).client_data as *mut frame;               // C 3803

  if !(*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell.is_null ()         // C 3805
    {
      cairo_surface_destroy ((*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell);   // C 3807
      (*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell = core::ptr::null_mut ();  // C 3808
    }

  if !(*FRAME_X_OUTPUT! (f)).atimer_visible_bell.is_null ()             // C 3811
    { (*FRAME_X_OUTPUT! (f)).atimer_visible_bell = core::ptr::null_mut (); }    // C 3812
}

/* Invert the middle quarter of the frame for .15 sec.  */

/* C: cairo.h 705-706: `void cairo_set_source_rgb (cairo_t *cr, double red, double green, double
   blue);` - the C 3844 call (this unit's own row, the cairo segment's practice in this file).
   cairo.h 690: `CAIRO_OPERATOR_DIFFERENCE,` of `typedef enum _cairo_operator { … }` - the C 3845
   argument, whose **value is measured against this machine's header** (clang:
   `CAIRO_OPERATOR_DIFFERENCE=23 CLEAR=0 SOURCE=1 sizeof (cairo_operator_t)=4`, the same reading
   `include/gtk.rs` 296-297's `CAIRO_OPERATOR_SOURCE = 1` carries).  The `///` form is a warning on
   an `extern` block (the Deviation 2 shape), so this row is `/* */`.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
}
#[cfg (feature = "have_pgtk")]
pub const CAIRO_OPERATOR_DIFFERENCE: cairo_operator_t = 23;             // cairo.h 690
/* <gtk/gtkwidget.h>: `void gtk_widget_error_bell (GtkWidget *widget);` - the C 3919 call (the GTK
   segment's convention: the header alone, no line).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_widget_error_bell (widget: *mut GtkWidget);
}

/// C 3817-3905: `static void pgtk_flash (struct frame *f)` - the context guard, the similar
/// surface, the five `cairo_*` calls with C's int literals to `double` parameters (`0 as c_double`,
/// the `src/gtkutil.rs` 807 spelling), the flash rectangles built from the frame's own
/// `FRAME_PIXEL_HEIGHT`/`FRAME_LINE_HEIGHT`/`FRAME_INTERNAL_BORDER_WIDTH`/`FRAME_TOP_MARGIN_HEIGHT`/
/// `FRAME_BOTTOM_MARGIN_HEIGHT` (all `include/frame.rs` functions; the line height is this unit's
/// `FRAME_LINE_HEIGHT!` macro pair), and the timer pair.  C 3893-3900's `atimer_visible_bell` is the
/// `pgtk_output` field the mirror declares `*mut c_void` (P-55: `struct atimer` was a bare tag when
/// that record was written), so the `cancel_atimer` argument and the `start_atimer` result carry the
/// pointer cast - the same two sites Deviation 48 repairs in `pgtk_free_frame_resources`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_flash (f: *mut frame)
{
  let mut surface_orig: *mut cairo_surface_t;                           // C 3820
  let mut surface: *mut cairo_surface_t;                                // C 3820
  let mut cr: *mut cairo_t;                                             // C 3821
  let mut width: c_int;                                                 // C 3822
  let mut height: c_int;                                                // C 3822
  let mut flash_height: c_int;                                          // C 3822
  let mut flash_left: c_int;                                            // C 3822
  let mut flash_right: c_int;                                           // C 3822
  let mut delay: timespec;                                              // C 3823

  if FRAME_CR_CONTEXT! (f).is_null ()                                   // C 3825
    { return; }                                                         // C 3826

  block_input ();                                                       // C 3828

  surface_orig = FRAME_CR_SURFACE! (f);                                 // C 3830

  width = FRAME_CR_SURFACE_DESIRED_WIDTH! (f);                          // C 3832
  height = FRAME_CR_SURFACE_DESIRED_HEIGHT! (f);                        // C 3833
  surface = cairo_surface_create_similar (surface_orig,                 // C 3834
					  CAIRO_CONTENT_COLOR_ALPHA, width, height);

  cr = cairo_create (surface);                                          // C 3838
  cairo_set_source_surface (cr, surface_orig, 0 as c_double, 0 as c_double);   // C 3839
  cairo_rectangle (cr, 0 as c_double, 0 as c_double, width as c_double, height as c_double);   // C 3840
  cairo_clip (cr);                                                      // C 3841
  cairo_paint (cr);                                                     // C 3842

  cairo_set_source_rgb (cr, 1 as c_double, 1 as c_double, 1 as c_double);   // C 3844
  cairo_set_operator (cr, CAIRO_OPERATOR_DIFFERENCE);                   // C 3845

  /* Get the height not including a menu bar widget.  */
  height = FRAME_PIXEL_HEIGHT (f);                                      // C 3848
  /* Height of each line to flash.  */
  flash_height = FRAME_LINE_HEIGHT! (f);                                // C 3850
  /* These will be the left and right margins of the rectangles.  */
  flash_left = FRAME_INTERNAL_BORDER_WIDTH (f);                         // C 3852
  flash_right = (FRAME_PIXEL_WIDTH (f)                                  // C 3853
		 - FRAME_INTERNAL_BORDER_WIDTH (f));                    // C 3854
  width = flash_right - flash_left;                                     // C 3855

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT! (f)                                // C 3858
    {
      cairo_rectangle (cr, flash_left as c_double,                      // C 3860-3863
		       (FRAME_INTERNAL_BORDER_WIDTH (f)
			+ FRAME_TOP_MARGIN_HEIGHT (f)) as c_double,
		       width as c_double, flash_height as c_double);
      cairo_fill (cr);                                                  // C 3865

      cairo_rectangle (cr, flash_left as c_double,                      // C 3867-3871
		       (height - flash_height
			- FRAME_INTERNAL_BORDER_WIDTH (f)
			- FRAME_BOTTOM_MARGIN_HEIGHT (f)) as c_double,
		       width as c_double, flash_height as c_double);
      cairo_fill (cr);                                                  // C 3873
    }
  else
    {
      /* If it is short, flash it all.  */
      cairo_rectangle (cr, flash_left as c_double,                      // C 3878-3880
		       FRAME_INTERNAL_BORDER_WIDTH (f) as c_double,
		       width as c_double,
		       (height - 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) as c_double);
      cairo_fill (cr);                                                  // C 3881
    }

  /* This surface may be leaked if XTflash is invoked again after a
     visible bell but before the atimer has had an opportunity to undo
     the first invocation.  (bug#77128) */
  if !(*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell.is_null ()         // C 3887
    { cairo_surface_destroy ((*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell); }   // C 3888
  (*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell = surface;             // C 3889

  delay = make_timespec (0, 50 * 1000 * 1000);                          // C 3891

  if !(*FRAME_X_OUTPUT! (f)).atimer_visible_bell.is_null ()             // C 3893
    {
      cancel_atimer ((*FRAME_X_OUTPUT! (f)).atimer_visible_bell as *mut atimer);   // C 3895
      (*FRAME_X_OUTPUT! (f)).atimer_visible_bell = core::ptr::null_mut ();         // C 3896
    }

  (*FRAME_X_OUTPUT! (f)).atimer_visible_bell                            // C 3899-3900
    = start_atimer (ATIMER_RELATIVE, delay, Some (recover_from_visible_bell),
		    f as *mut c_void) as *mut c_void;


  cairo_destroy (cr);                                                   // C 3903
  unblock_input ();                                                     // C 3904
}

/* Make audible bell.  */

/// C 3909-3922: `static void pgtk_ring_bell (struct frame *f)` - `visible_bell` is
/// `include/globals.rs` 7145's extern (dispextern.h 3072), and `gtk_widget_error_bell` is the
/// use-site declaration above.  C's address is taken later (the terminal's `ring_bell_hook` in
/// `pgtk_term_init`, C 7442), so this definition is converted to `unsafe extern "C" fn` **when that
/// slice lands** (PORTING.md 1.4's rule, the same repair slice 25 made for fifteen of them).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_ring_bell (f: *mut frame)
{
  if visible_bell                                                       // C 3912
    {
      pgtk_flash (f);                                                   // C 3914
    }
  else
    {
      block_input ();                                                   // C 3918
      gtk_widget_error_bell (FRAME_GTK_WIDGET! (f));                    // C 3919
      unblock_input ();                                                 // C 3920
    }
}

/* Read events coming from the X server.
   Return as soon as there are no more events to be read.

   Return the number of characters stored into the buffer,
   thus pretending to be `read' (except the characters we store
   in the keyboard buffer can be multibyte, so are not necessarily
   C chars).  */

/// C 3924-3970: `static int pgtk_read_socket (struct terminal *terminal, struct input_event
/// *hold_quit)` - C's body never reads `terminal` (`let _ =` keeps the name), `evq_flush` is this
/// file's own C 604 definition, the four GLib calls are `include/gtk.rs` 1919-1925's rows (GLib is
/// installed on this machine, so those rows are header readings), and C's two `int`-to-`bool`
/// conversions are the port's `!= 0`.  C 7447 takes this function's address (the terminal's
/// `read_socket_hook`), so it becomes `unsafe extern "C" fn` in the slice that lands that
/// assignment (PORTING.md 1.4, the `pgtk_ring_bell` note above).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_read_socket (terminal: *mut terminal, hold_quit: *mut input_event) -> c_int
{
  let _ = terminal;                                                     // C 3933
  let mut context: *mut GMainContext;                                   // C 3935
  let mut context_acquired: bool = false;                               // C 3936
  let mut count: c_int;                                                 // C 3937

  count = evq_flush (hold_quit);                                        // C 3939
  if count > 0                                                          // C 3940
    {
      return count;                                                     // C 3942
    }

  context = g_main_context_default ();                                  // C 3945
  context_acquired = g_main_context_acquire (context) != 0;             // C 3946

  block_input ();                                                       // C 3948

  if context_acquired                                                   // C 3950
    {
      while g_main_context_pending (context) != 0                       // C 3952
	{
	  g_main_context_dispatch (context);                            // C 3954
	}
    }

  unblock_input ();                                                     // C 3958

  if context_acquired                                                   // C 3960
    { g_main_context_release (context); }                               // C 3961

  count = evq_flush (hold_quit);                                        // C 3963
  if count > 0                                                          // C 3964
    {
      return count;                                                     // C 3966
    }

  return 0;                                                             // C 3969
}

/* Lisp window being scrolled.  Set when starting to interact with
   a toolkit scroll bar, reset to nil when ending the interaction.  */

/// C 3975: `static Lisp_Object window_being_scrolled;` - C leaves it uninitialised (a file-scope
/// object, so C's own storage is zero-filled); the port writes `lisp_h_Qnil ()` (`NIL_IS_ZERO`).
#[cfg (feature = "have_pgtk")]
static mut window_being_scrolled: Lisp_Object = lisp_h_Qnil ();

/// C 3977-3998: `static void pgtk_send_scroll_bar_event (Lisp_Object window, enum scroll_bar_part
/// part, int portion, int whole, bool horizontal)` - C's local `union buffered_input_event inev` is
/// zero-initialised (`EVENT_INIT (inev.ie)` fills the member C uses), `include/termhooks.rs` 114-131
/// folds `kind`/`part` into the record's `bits` word (PORTING.md 1.3), so C's two members are this
/// unit's own `input_event_bits!` writer pair (C 214-222), and C's implicit `int` -> `EMACS_INT`
/// conversions at the two `make_fixnum` calls are spelled out (the `src/androidterm.rs` 814-815
/// shape, Deviation 44's class).  `evq_enqueue` is this file's own C 604 definition.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_send_scroll_bar_event (window: Lisp_Object, part: scroll_bar_part,
				      portion: c_int, whole: c_int, horizontal: bool)
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 3981

  EVENT_INIT (&raw mut inev.ie);                                        // C 3983

  input_event_bits! (kind, &raw mut inev.ie,                            // C 3985-3987
		     if horizontal { HORIZONTAL_SCROLL_BAR_CLICK_EVENT }
		     else { SCROLL_BAR_CLICK_EVENT });
  inev.ie.frame_or_window = window;                                     // C 3988
  inev.ie.arg = Qnil ();                                                // C 3989
  inev.ie.timestamp = 0;                                                // C 3990
  inev.ie.code = 0;                                                     // C 3991
  input_event_bits! (part, &raw mut inev.ie, part);                     // C 3992
  inev.ie.x = make_fixnum (portion as EMACS_INT);                       // C 3993
  inev.ie.y = make_fixnum (whole as EMACS_INT);                         // C 3994
  inev.ie.modifiers = 0;                                                // C 3995

  evq_enqueue (&raw mut inev);                                          // C 3997
}

/* Scroll bar callback for GTK scroll bars.  WIDGET is the scroll
   bar widget.  DATA is a pointer to the scroll_bar structure. */

/* <gtk/gtkrange.h>: `GtkAdjustment *gtk_range_get_adjustment (GtkRange *range);` - the C 4011 call,
   whose C-side checked cast `GTK_ADJUSTMENT (…)` has no counterpart because the declared return
   type is already the record the cast produces (P-72 rule 3).  <gtk/gtkadjustment.h>: `gdouble
   gtk_adjustment_get_upper (GtkAdjustment *adjustment);` - the C 4022/4030 calls; `gdouble
   gtk_adjustment_get_page_size (GtkAdjustment *adjustment);` - the C 4023/4031 calls.  The three
   are the GTK segment's convention (header alone, no line: this machine has no GTK tree), and
   `include/gtk.rs` carries neither them nor the two records below - that mirror is the
   `<gtk/gtk.h>` chain's home, so these five rows are owed to it (Owed 11).  */
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct GtkRange
{
  _private: [u8; 0],
}
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct GtkAdjustment
{
  _private: [u8; 0],
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_range_get_adjustment (range: *mut GtkRange) -> *mut GtkAdjustment;
  pub fn gtk_adjustment_get_upper (adjustment: *mut GtkAdjustment) -> gdouble;
  pub fn gtk_adjustment_get_page_size (adjustment: *mut GtkAdjustment) -> gdouble;
}
/* C: gtypes.h 61-64: `typedef double gdouble;` - C 4006's parameter type and the two
   `gtk_adjustment_get_*` returns.  `include/gtk.rs` 40-41 has `gint`/`gboolean` and not this row,
   so it stands here (the mirror gap is Owed 11's fourth item).  */
#[cfg (feature = "have_pgtk")]
pub type gdouble = c_double;
/* C 4013's `xg_ignore_gtk_scrollbar` is `gtkutil.c` 4351's `bool xg_ignore_gtk_scrollbar;`;
   `include/gtkutil.rs` 36 names it as owed and no mirror carries it, so it is declared here.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub static mut xg_ignore_gtk_scrollbar: bool;
}
/* C: gtk/gtkenums.h: `typedef enum { GTK_SCROLL_NONE, GTK_SCROLL_JUMP, GTK_SCROLL_STEP_BACKWARD,
   GTK_SCROLL_STEP_FORWARD, GTK_SCROLL_PAGE_BACKWARD, GTK_SCROLL_PAGE_FORWARD, GTK_SCROLL_STEP_UP,
   … } GtkScrollType;` - the type of C 4006's `scroll` parameter and of the five labels C 4018-4054
   names.  GTK's values cannot be measured on this machine (no GTK tree), so each is a placeholder
   in the same sense as `include/gtk.rs`'s `GDK_*` rows, and the enum's ABI is `c_int`
   (PORTING.md 1.3/P-51).  */
#[cfg (feature = "have_pgtk")]
pub type GtkScrollType = c_int;
#[cfg (feature = "have_pgtk")]
pub const GTK_SCROLL_JUMP: GtkScrollType = 1;                 // placeholder, not GTK's value
#[cfg (feature = "have_pgtk")]
pub const GTK_SCROLL_STEP_BACKWARD: GtkScrollType = 2;        // placeholder, not GTK's value
#[cfg (feature = "have_pgtk")]
pub const GTK_SCROLL_STEP_FORWARD: GtkScrollType = 3;         // placeholder, not GTK's value
#[cfg (feature = "have_pgtk")]
pub const GTK_SCROLL_PAGE_BACKWARD: GtkScrollType = 4;        // placeholder, not GTK's value
#[cfg (feature = "have_pgtk")]
pub const GTK_SCROLL_PAGE_FORWARD: GtkScrollType = 5;         // placeholder, not GTK's value

/// C 4004-4067: `static gboolean xg_scroll_callback (GtkRange *range, GtkScrollType scroll, gdouble
/// value, gpointer user_data)` - C's `struct scroll_bar *bar = user_data` is the pointer cast the
/// port spells (P-72 rule 3; `scroll_bar` is `include/pgtkterm.rs` 141's record), C's `switch
/// (scroll)` is the port's `match` over the five labels (PORTING.md 2.2, this function's own
/// `macro-body-notes.tsv` row), the two `(int)` casts of `gdouble` expressions and the two
/// `min ((int) value, whole)` calls are spelled with C's conversions (`min` is the mirror's
/// C-`max`/`min` helper, `include/lisp.rs` 3152), `user_data`'s `gpointer` arrives as the record
/// pointer C casts it to, and C's two `false` returns from a `gboolean` function are the port's
/// `false as gboolean` (the `gtk_widget_set_has_window` conversion, slice 25).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn xg_scroll_callback (range: *mut GtkRange, scroll: GtkScrollType,
					 value: gdouble, user_data: gpointer) -> gboolean
{
  let mut whole: c_int = 0;                                             // C 4008
  let mut portion: c_int = 0;                                           // C 4008
  let bar: *mut scroll_bar = user_data as *mut scroll_bar;              // C 4009
  let mut part: scroll_bar_part = scroll_bar_nowhere;                   // C 4010
  let adj: *mut GtkAdjustment = gtk_range_get_adjustment (range);       // C 4011

  if xg_ignore_gtk_scrollbar                                            // C 4013
    { return false as gboolean; }                                       // C 4014

  match scroll                                                          // C 4016
    {
      GTK_SCROLL_JUMP =>                                                // C 4018
	{
	  if (*bar).horizontal                                          // C 4019
	    {
	      part = scroll_bar_horizontal_handle;                      // C 4021
	      whole = (gtk_adjustment_get_upper (adj)                   // C 4022-4023
		       - gtk_adjustment_get_page_size (adj)) as c_int;
	      portion = min (value as c_int as isize, whole as isize) as c_int;   // C 4024
	      (*bar).dragging = portion;                                // C 4025
	    }
	  else
	    {
	      part = scroll_bar_handle;                                 // C 4029
	      whole = (gtk_adjustment_get_upper (adj)                   // C 4030-4031
		       - gtk_adjustment_get_page_size (adj)) as c_int;
	      portion = min (value as c_int as isize, whole as isize) as c_int;   // C 4032
	      (*bar).dragging = portion;                                // C 4033
	    }
	}
      GTK_SCROLL_STEP_BACKWARD =>                                       // C 4036
	{
	  part = if (*bar).horizontal { scroll_bar_left_arrow }         // C 4037
		 else { scroll_bar_up_arrow };
	  (*bar).dragging = -1;                                         // C 4038
	}
      GTK_SCROLL_STEP_FORWARD =>                                        // C 4040
	{
	  part = if (*bar).horizontal { scroll_bar_right_arrow }        // C 4041-4042
		 else { scroll_bar_down_arrow };
	  (*bar).dragging = -1;                                         // C 4043
	}
      GTK_SCROLL_PAGE_BACKWARD =>                                       // C 4045
	{
	  part = if (*bar).horizontal { scroll_bar_before_handle }      // C 4046-4047
		 else { scroll_bar_above_handle };
	  (*bar).dragging = -1;                                         // C 4048
	}
      GTK_SCROLL_PAGE_FORWARD =>                                        // C 4050
	{
	  part = if (*bar).horizontal { scroll_bar_after_handle }       // C 4051-4052
		 else { scroll_bar_below_handle };
	  (*bar).dragging = -1;                                         // C 4053
	}
      _ =>                                                              // C 4055
	{
	}
    }

  if part != scroll_bar_nowhere                                         // C 4059
    {
      window_being_scrolled = (*bar).window;                            // C 4061
      pgtk_send_scroll_bar_event ((*bar).window, part, portion, whole,  // C 4062-4063
				  (*bar).horizontal);
    }

  return false as gboolean;                                             // C 4066
}

/* Callback for button release. Sets dragging to -1 when dragging is done.  */

/* C: gtkutil.h 118-122: `extern void xg_create_scroll_bar (struct frame *f, struct scroll_bar *bar,
   GCallback scroll_callback, GCallback end_callback, const char *scroll_bar_name);` - the C 4100
   call; gtkutil.h 123-127: the `xg_create_horizontal_scroll_bar` twin - the C 4112 call;
   gtkutil.h 143-146: `extern void xg_set_toolkit_scroll_bar_thumb (struct scroll_bar *bar, int
   portion, int position, int whole);` - the C 4125 call; gtkutil.h 147-150: the
   `xg_set_toolkit_horizontal_scroll_bar_thumb` twin - the C 4133 call.  `include/gtkutil.rs`
   carries none of the four (that mirror is `gtkutil.h`'s home: Owed 12), so all four stand here at
   the use site with C's own prototypes (P-88 ①), `GCallback` being `include/gtk.rs` 93's type.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn xg_create_scroll_bar (f: *mut frame, bar: *mut scroll_bar, scroll_callback: GCallback,
			       end_callback: GCallback, scroll_bar_name: *const c_char);
  pub fn xg_create_horizontal_scroll_bar (f: *mut frame, bar: *mut scroll_bar,
					  scroll_callback: GCallback, end_callback: GCallback,
					  scroll_bar_name: *const c_char);
  pub fn xg_set_toolkit_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
					  position: c_int, whole: c_int);
  pub fn xg_set_toolkit_horizontal_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
						     position: c_int, whole: c_int);
}

/// C 4071-4086: `static gboolean xg_end_scroll_callback (GtkWidget *widget, GdkEventButton *event,
/// gpointer user_data)` - **its address goes through `G_CALLBACK` at C 4101/4113**, so the port
/// defines it `unsafe extern "C" fn` (PORTING.md 1.4; `include/gtk.rs` 141-148's `G_CALLBACK!` is
/// the macro C's cast is spelled with).  C's body never reads `widget`/`event` (`let _ =` keeps the
/// names, the Deviation 44 shape), and its `return false;` from a `gboolean` function is the port's
/// `false as gboolean` (Deviation 50).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn xg_end_scroll_callback (widget: *mut GtkWidget, event: *mut GdkEventButton,
					     user_data: gpointer) -> gboolean
{
  let _ = (widget, event);                                              // C 4072-4073
  let bar: *mut scroll_bar = user_data as *mut scroll_bar;              // C 4075
  (*bar).dragging = -1;                                                 // C 4076
  if WINDOWP (window_being_scrolled)                                    // C 4077
    {
      pgtk_send_scroll_bar_event (window_being_scrolled,                // C 4079-4081
				  scroll_bar_end_scroll, 0, 0, (*bar).horizontal);
      window_being_scrolled = Qnil ();                                  // C 4082
    }

  return false as gboolean;                                             // C 4085
}

/// C 4088: `#define SCROLL_BAR_NAME "verticalScrollBar"`
#[cfg (feature = "have_pgtk")]
pub const SCROLL_BAR_NAME: &core::ffi::CStr = c"verticalScrollBar";
/// C 4089: `#define SCROLL_BAR_HORIZONTAL_NAME "horizontalScrollBar"`
#[cfg (feature = "have_pgtk")]
pub const SCROLL_BAR_HORIZONTAL_NAME: &core::ffi::CStr = c"horizontalScrollBar";

/* Create the widget for scroll bar BAR on frame F.  Record the widget
   and X window of the scroll bar in BAR.  */

/// C 4094-4103: `static void pgtk_create_toolkit_scroll_bar (struct frame *f, struct scroll_bar
/// *bar)` - the `SCROLL_BAR_NAME` constant's pointer (the `nt/addpm.rs` `REG_ROOT` shape for a C
/// string macro) and the two `G_CALLBACK` arguments.  C takes this function's address later (the
/// terminal's `create_scroll_bar_hook`), so it stays a plain `unsafe fn` until that slice converts
/// it (the `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_create_toolkit_scroll_bar (f: *mut frame, bar: *mut scroll_bar)
{
  let scroll_bar_name: *const c_char = SCROLL_BAR_NAME.as_ptr ();       // C 4097

  block_input ();                                                       // C 4099
  xg_create_scroll_bar (f, bar, G_CALLBACK! (xg_scroll_callback),       // C 4100-4101
			G_CALLBACK! (xg_end_scroll_callback), scroll_bar_name);
  unblock_input ();                                                     // C 4102
}

/// C 4105-4116: `static void pgtk_create_horizontal_toolkit_scroll_bar (struct frame *f, struct
/// scroll_bar *bar)` - the horizontal twin, with `SCROLL_BAR_HORIZONTAL_NAME`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_create_horizontal_toolkit_scroll_bar (f: *mut frame, bar: *mut scroll_bar)
{
  let scroll_bar_name: *const c_char = SCROLL_BAR_HORIZONTAL_NAME.as_ptr ();   // C 4109

  block_input ();                                                       // C 4111
  xg_create_horizontal_scroll_bar (f, bar, G_CALLBACK! (xg_scroll_callback),   // C 4112-4114
				   G_CALLBACK! (xg_end_scroll_callback), scroll_bar_name);
  unblock_input ();                                                     // C 4115
}

/* Set the thumb size and position of scroll bar BAR.  We are currently
   displaying PORTION out of a whole WHOLE, and our position POSITION.  */

/// C 4121-4126: `static void pgtk_set_toolkit_scroll_bar_thumb (struct scroll_bar *bar, int
/// portion, int position, int whole)` - one call to `xg_set_toolkit_scroll_bar_thumb`
/// (gtkutil.h 143-146).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_toolkit_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
					     position: c_int, whole: c_int)
{
  xg_set_toolkit_scroll_bar_thumb (bar, portion, position, whole);      // C 4125
}

/// C 4128-4134: `static void pgtk_set_toolkit_horizontal_scroll_bar_thumb (struct scroll_bar *bar,
/// int portion, int position, int whole)` - the horizontal twin (gtkutil.h 147-150).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_toolkit_horizontal_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
							position: c_int, whole: c_int)
{
  xg_set_toolkit_horizontal_scroll_bar_thumb (bar, portion, position, whole);   // C 4133
}

/* Create a scroll bar and return the scroll bar vector for it.  W is
   the Emacs window on which to create the scroll bar. TOP, LEFT,
   WIDTH and HEIGHT are the pixel coordinates and dimensions of the
   scroll bar. */

/* C: gtkutil.h 128: `extern void xg_remove_scroll_bar (struct frame *f, ptrdiff_t scrollbar_id);` -
   the C 4198 call; gtkutil.h 129-134: `extern void xg_update_scrollbar_pos (struct frame *f,
   ptrdiff_t scrollbar_id, int top, int left, int width, int height);` - the C 4181 call;
   gtkutil.h 135-140: the `xg_update_horizontal_scrollbar_pos` twin - the C 4178 call.
   `include/gtkutil.rs` carries none of the three (Owed 12's list grows to seven), so all three
   stand here with C's own prototypes.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn xg_remove_scroll_bar (f: *mut frame, scrollbar_id: ptrdiff_t);
  pub fn xg_update_scrollbar_pos (f: *mut frame, scrollbar_id: ptrdiff_t, top: c_int, left: c_int,
				  width: c_int, height: c_int);
  pub fn xg_update_horizontal_scrollbar_pos (f: *mut frame, scrollbar_id: ptrdiff_t, top: c_int,
					     left: c_int, width: c_int, height: c_int);
}

/// C 4141-4187: `static struct scroll_bar *pgtk_scroll_bar_create (struct window *w, int top, int
/// left, int width, int height, bool horizontal)` - `ALLOCATE_PSEUDOVECTOR` is
/// `include/lisp.rs` 2970's macro (the `src/alloc.rs` 4139 call shape, with the mirror's
/// `pvec_type::PVEC_OTHER`), `XSETWINDOW`/`XSETVECTOR` are that mirror's 3135/3108,
/// `XSCROLL_BAR` is `include/pgtkterm.rs` 520-522's, `FRAME_SCROLL_BARS`/`fset_scroll_bars` are
/// `include/frame.rs` 1169/1497's, and C's two `max (height, 1)` calls are the mirror's C-`max`
/// helper with C's `int` conversion spelled back (`as c_int`; the `src/androidterm.rs` 814-815
/// family).  C 7442 takes this function's address (the terminal's `create_scroll_bar_hook`), so it
/// stays a plain `unsafe fn` until that slice converts it (the `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_scroll_bar_create (w: *mut window, top: c_int, left: c_int, width: c_int,
				  height: c_int, horizontal: bool) -> *mut scroll_bar
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 4145
  let bar: *mut scroll_bar                                            // C 4146-4147
    = ALLOCATE_PSEUDOVECTOR! (scroll_bar, prev, pvec_type::PVEC_OTHER);
  let mut barobj: Lisp_Object;                                          // C 4148

  block_input ();                                                       // C 4150

  if horizontal                                                         // C 4152
    { pgtk_create_horizontal_toolkit_scroll_bar (f, bar); }             // C 4153
  else
    { pgtk_create_toolkit_scroll_bar (f, bar); }                        // C 4155

  XSETWINDOW! ((*bar).window, w);                                       // C 4157
  (*bar).top = top;                                                     // C 4158
  (*bar).left = left;                                                   // C 4159
  (*bar).width = width;                                                 // C 4160
  (*bar).height = height;                                               // C 4161
  (*bar).start = 0;                                                     // C 4162
  (*bar).end = 0;                                                       // C 4163
  (*bar).dragging = -1;                                                 // C 4164
  (*bar).horizontal = horizontal;                                       // C 4165

  /* Add bar to its frame's list of scroll bars.  */
  (*bar).next = FRAME_SCROLL_BARS (f);                                  // C 4168
  (*bar).prev = Qnil ();                                                // C 4169
  XSETVECTOR! (barobj, bar);                                            // C 4170
  fset_scroll_bars (f, barobj);                                         // C 4171
  if !NILP ((*bar).next)                                                // C 4172
    { XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }          // C 4173

  /* Map the window/widget.  */
  {
    if horizontal                                                       // C 4177
      { xg_update_horizontal_scrollbar_pos (f, (*bar).x_window as ptrdiff_t, top,   // C 4178-4179
					    left, width, max (height as isize, 1) as c_int); }
    else
      { xg_update_scrollbar_pos (f, (*bar).x_window as ptrdiff_t, top,  // C 4181-4182
				 left, width, max (height as isize, 1) as c_int); }
  }

  unblock_input ();                                                     // C 4185
  return bar;                                                           // C 4186
}

/* Destroy scroll bar BAR, and set its Emacs window's scroll bar to
   nil.  */

/// C 4192-4207: `static void pgtk_scroll_bar_remove (struct scroll_bar *bar)` - `WINDOW_FRAME` is
/// `include/frame.rs` 1607's function and `wset_horizontal_scroll_bar`/`wset_vertical_scroll_bar`
/// are `include/window.rs` 236/233's setters (the mirror's shape for C's `w->…_scroll_bar = v`).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_scroll_bar_remove (bar: *mut scroll_bar)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME (XWINDOW ((*bar).window)));  // C 4195
  block_input ();                                                       // C 4196

  xg_remove_scroll_bar (f, (*bar).x_window as ptrdiff_t);               // C 4198

  /* Dissociate this scroll bar from its window.  */
  if (*bar).horizontal                                                  // C 4201
    { wset_horizontal_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }  // C 4202
  else
    { wset_vertical_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }    // C 4204

  unblock_input ();                                                     // C 4206
}

/* Set the handle of the vertical scroll bar for WINDOW to indicate
   that we are displaying PORTION characters out of a total of WHOLE
   characters, starting at POSITION.  If WINDOW has no scroll bar,
   create one.  */

/// C 4214-4285: `static void pgtk_set_vertical_scroll_bar (struct window *w, int portion, int whole,
/// int position)` - `window_box` is this unit's own declaration (C 3123's call site) and C's two
/// `0`s are its null outputs, `WINDOW_SCROLL_BAR_AREA_X`/`_WIDTH` are `include/window.rs`
/// 750/744's macros, the record's two Lisp_Object slots are that mirror's plain fields
/// (`vertical_scroll_bar` 92), `XSCROLL_BAR` is `include/pgtkterm.rs` 520-522's, and C's
/// `max (height, 1)` carries its `int` conversion back (`as c_int`).  C 7442 takes this function's
/// address (the terminal's `set_vertical_scroll_bar_hook`), so it stays a plain `unsafe fn` until
/// that slice converts it (the `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_vertical_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
					position: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 4218
  let mut barobj: Lisp_Object;                                          // C 4219
  let bar: *mut scroll_bar;                                             // C 4220
  let mut top: c_int;                                                   // C 4221
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  let mut window_y: c_int = 0;                                          // C 4222
  let mut window_height: c_int = 0;

  /* Get window dimensions.  */
  window_box (w, ANY_AREA, core::ptr::null_mut (), &raw mut window_y,   // C 4225
	      core::ptr::null_mut (), &raw mut window_height);
  top = window_y;                                                       // C 4226
  height = window_height;                                               // C 4227
  left = WINDOW_SCROLL_BAR_AREA_X! (w);                                 // C 4228
  width = WINDOW_SCROLL_BAR_AREA_WIDTH! (w);                            // C 4229

  /* Does the scroll bar exist yet?  */
  if NILP ((*w).vertical_scroll_bar)                                    // C 4232
    {
      if width > 0 && height > 0                                        // C 4234
	{
	  block_input ();                                               // C 4236
	  pgtk_clear_area (f, left, top, width, height);                // C 4237
	  unblock_input ();                                             // C 4238
	}

      bar = pgtk_scroll_bar_create (w, top, left, width,                // C 4241
				    max (height as isize, 1) as c_int, false);
    }
  else
    {
      /* It may just need to be moved and resized.  */
      let mut mask: c_uint = 0;                                         // C 4246

      bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                    // C 4248

      block_input ();                                                   // C 4250

      if left != (*bar).left                                            // C 4252
	{ mask |= 1; }                                                  // C 4253
      if top != (*bar).top                                              // C 4254
	{ mask |= 1; }                                                  // C 4255
      if width != (*bar).width                                          // C 4256
	{ mask |= 1; }                                                  // C 4257
      if height != (*bar).height                                        // C 4258
	{ mask |= 1; }                                                  // C 4259

      /* Move/size the scroll bar widget.  */
      if mask != 0                                                      // C 4262
	{
	  /* Since toolkit scroll bars are smaller than the space reserved
	     for them on the frame, we have to clear "under" them.  */
	  if width > 0 && height > 0                                    // C 4266
	    { pgtk_clear_area (f, left, top, width, height); }          // C 4267
	  xg_update_scrollbar_pos (f, (*bar).x_window as ptrdiff_t, top, // C 4268-4269
				   left, width, max (height as isize, 1) as c_int);
	}

      /* Remember new settings.  */
      (*bar).left = left;                                               // C 4273
      (*bar).top = top;                                                 // C 4274
      (*bar).width = width;                                             // C 4275
      (*bar).height = height;                                           // C 4276

      unblock_input ();                                                 // C 4278
    }

  pgtk_set_toolkit_scroll_bar_thumb (bar, portion, position, whole);    // C 4281

  XSETVECTOR! (barobj, bar);                                            // C 4283
  wset_vertical_scroll_bar (w, barobj);                                 // C 4284
}

/// C 4287-4365: `static void pgtk_set_horizontal_scroll_bar (struct window *w, int portion, int
/// whole, int position)` - the horizontal twin: `WINDOW_PIXEL_WIDTH` is `include/window.rs` 415's
/// function, `WINDOW_SCROLL_BAR_AREA_Y`/`_HEIGHT` are its 827/778 macros, and the clear area in the
/// move arm spans `WINDOW_LEFT_EDGE_X` (601) to `WINDOW_RIGHT_DIVIDER_WIDTH` (582) short of the
/// window's pixel width.  Its address is taken by the same slice as the vertical one's.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_horizontal_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
					  position: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 4291
  let mut barobj: Lisp_Object;                                          // C 4292
  let bar: *mut scroll_bar;                                             // C 4293
  let mut top: c_int;                                                   // C 4294
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  let mut window_x: c_int = 0;                                          // C 4295
  let mut window_width: c_int = 0;
  let pixel_width: c_int = WINDOW_PIXEL_WIDTH (w);                      // C 4296

  /* Get window dimensions.  */
  window_box (w, ANY_AREA, &raw mut window_x, core::ptr::null_mut (),   // C 4299
	      &raw mut window_width, core::ptr::null_mut ());
  left = window_x;                                                      // C 4300
  width = window_width;                                                 // C 4301
  top = WINDOW_SCROLL_BAR_AREA_Y! (w);                                  // C 4302
  height = WINDOW_SCROLL_BAR_AREA_HEIGHT! (w);                          // C 4303

  /* Does the scroll bar exist yet?  */
  if NILP ((*w).horizontal_scroll_bar)                                  // C 4306
    {
      if width > 0 && height > 0                                        // C 4308
	{
	  block_input ();                                               // C 4310

	  /* Clear also part between window_width and
	     WINDOW_PIXEL_WIDTH.  */
	  pgtk_clear_area (f, left, top, pixel_width, height);          // C 4314
	  unblock_input ();                                             // C 4315
	}

      bar = pgtk_scroll_bar_create (w, top, left, width, height, true);  // C 4318
    }
  else
    {
      /* It may just need to be moved and resized.  */
      let mut mask: c_uint = 0;                                         // C 4323

      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                  // C 4325

      block_input ();                                                   // C 4327

      if left != (*bar).left                                            // C 4329
	{ mask |= 1; }                                                  // C 4330
      if top != (*bar).top                                              // C 4331
	{ mask |= 1; }                                                  // C 4332
      if width != (*bar).width                                          // C 4333
	{ mask |= 1; }                                                  // C 4334
      if height != (*bar).height                                        // C 4335
	{ mask |= 1; }                                                  // C 4336

      /* Move/size the scroll bar widget.  */
      if mask != 0                                                      // C 4339
	{
	  /* Since toolkit scroll bars are smaller than the space reserved
	     for them on the frame, we have to clear "under" them.  */
	  if width > 0 && height > 0                                    // C 4343
	    {
	      pgtk_clear_area (f,                                        // C 4344-4347
			       WINDOW_LEFT_EDGE_X! (w), top,
			       pixel_width - WINDOW_RIGHT_DIVIDER_WIDTH! (w),
			       height);
	    }
	  xg_update_horizontal_scrollbar_pos (f, (*bar).x_window as ptrdiff_t, top, left,   // C 4348-4349
					      width, height);
	}

      /* Remember new settings.  */
      (*bar).left = left;                                               // C 4353
      (*bar).top = top;                                                 // C 4354
      (*bar).width = width;                                             // C 4355
      (*bar).height = height;                                           // C 4356

      unblock_input ();                                                 // C 4358
    }

  pgtk_set_toolkit_horizontal_scroll_bar_thumb (bar, portion, position, whole);   // C 4361

  XSETVECTOR! (barobj, bar);                                            // C 4363
  wset_horizontal_scroll_bar (w, barobj);                               // C 4364
}

/* The following three hooks are used when we're doing a thorough
   redisplay of the frame.  We don't explicitly know which scroll bars
   are going to be deleted, because keeping track of when windows go
   away is a real pain - "Can you say set-window-configuration, boys
   and girls?"  Instead, we just assert at the beginning of redisplay
   that *all* scroll bars are to be removed, and then save a scroll bar
   from the fiery pit when we actually redisplay its window.  */

/* Arrange for all scroll bars on FRAME to be removed at the next call
   to `*judge_scroll_bars_hook'.  A scroll bar may be spared if
   `*redeem_scroll_bar_hook' is applied to its window before the judgment.  */

/// C 4379-4399: `static void pgtk_condemn_scroll_bars (struct frame *frame)` - `FRAME_SCROLL_BARS`/
/// `FRAME_CONDEMNED_SCROLL_BARS`/`fset_condemned_scroll_bars`/`fset_scroll_bars` are
/// `include/frame.rs` 1169/1171/1479/1497's accessors over the frame's three Lisp_Object slots, and
/// C's `while (!NILP (XSCROLL_BAR (last)->next)) last = …` is the port's `while` with the step at
/// the body's end (the `FOR_EACH_FRAME!` shape).  C 7442 takes this function's address (the
/// terminal's `condemn_scroll_bars_hook`), so it stays a plain `unsafe fn` until that slice converts
/// it (the `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_condemn_scroll_bars (frame: *mut frame)
{
  if !NILP (FRAME_SCROLL_BARS (frame))                                  // C 4382
    {
      if !NILP (FRAME_CONDEMNED_SCROLL_BARS (frame))                    // C 4384
	{
	  /* Prepend scrollbars to already condemned ones.  */
	  let mut last: Lisp_Object = FRAME_SCROLL_BARS (frame);        // C 4387

	  while !NILP ((*XSCROLL_BAR! (last)).next)                     // C 4389
	    { last = (*XSCROLL_BAR! (last)).next; }                     // C 4390

	  (*XSCROLL_BAR! (last)).next = FRAME_CONDEMNED_SCROLL_BARS (frame);   // C 4392
	  (*XSCROLL_BAR! (FRAME_CONDEMNED_SCROLL_BARS (frame))).prev = last;   // C 4393
	}

      fset_condemned_scroll_bars (frame, FRAME_SCROLL_BARS (frame));    // C 4396
      fset_scroll_bars (frame, Qnil ());                                // C 4397
    }
}

/* Un-mark WINDOW's scroll bar for deletion in this judgment cycle.
   Note that WINDOW isn't necessarily condemned at all.  */

/// C 4404-4484: `static void pgtk_redeem_scroll_bar (struct window *w)` - **C's forward
/// `goto horizontal` (C 4426) is the port's labelled block**: PORTING.md 2.2's rule for a forward
/// goto whose target is the code that *follows* the block (the horizontal arm here, which an early
/// `return` would skip) is a `'label:`-wrapped block and a `break 'label`, so the block carries C's
/// own label name and this function's `macro-body-notes.tsv` row records the `goto` token.  C's
/// `emacs_abort ()` returns `!` (`include/lisp.rs` 3818), `WINDOW_HAS_VERTICAL_SCROLL_BAR` is
/// `include/window.rs` 691's macro and `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` its 472 (that mirror's
/// standing `false` accessor, window.h 1046-1051's no-window-system arm - the read this unit
/// re-states rather than changes).  Its address goes to the terminal's `redeem_scroll_bar_hook`
/// (C 7442) in the same later slice as its two siblings.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_redeem_scroll_bar (w: *mut window)
{
  let mut bar: *mut scroll_bar;                                         // C 4407
  let mut barobj: Lisp_Object;                                          // C 4408
  let mut f: *mut frame;                                                // C 4409

  /* We can't redeem this window's scroll bar if it doesn't have one.  */
  if NILP ((*w).vertical_scroll_bar) && NILP ((*w).horizontal_scroll_bar)   // C 4412
    { emacs_abort (); }                                                 // C 4413

  'horizontal: {                        /* C 4449's label: C 4426's `goto horizontal` lands here */
    if !NILP ((*w).vertical_scroll_bar) && WINDOW_HAS_VERTICAL_SCROLL_BAR! (w)   // C 4415
      {
	bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                  // C 4417
	/* Unlink it from the condemned list.  */
	f = XFRAME (WINDOW_FRAME (w));                                  // C 4419
	if NILP ((*bar).prev)                                           // C 4420
	  {
	    /* If the prev pointer is nil, it must be the first in one of
	       the lists.  */
	    if EQ (FRAME_SCROLL_BARS (f), (*w).vertical_scroll_bar)     // C 4424
	      /* It's not condemned.  Everything's fine.  */
	      { break 'horizontal; }                                    // C 4426's `goto horizontal`
	    else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),                // C 4427-4428
			(*w).vertical_scroll_bar)
	      { fset_condemned_scroll_bars (f, (*bar).next); }          // C 4429
	    else
	      /* If its prev pointer is nil, it must be at the front of
		 one or the other!  */
	      { emacs_abort (); }                                       // C 4433
	  }
	else
	  { (*XSCROLL_BAR! ((*bar).prev)).next = (*bar).next; }         // C 4436

	if !NILP ((*bar).next)                                          // C 4438
	  { (*XSCROLL_BAR! ((*bar).next)).prev = (*bar).prev; }         // C 4439

	(*bar).next = FRAME_SCROLL_BARS (f);                            // C 4441
	(*bar).prev = Qnil ();                                          // C 4442
	XSETVECTOR! (barobj, bar);                                      // C 4443
	fset_scroll_bars (f, barobj);                                   // C 4444
	if !NILP ((*bar).next)                                          // C 4445
	  { XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }    // C 4446
      }
  }

  if !NILP ((*w).horizontal_scroll_bar)                                 // C 4450
      && WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)                           // C 4451
    {
      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                  // C 4453
      /* Unlink it from the condemned list.  */
      f = XFRAME (WINDOW_FRAME (w));                                    // C 4455
      if NILP ((*bar).prev)                                             // C 4456
	{
	  /* If the prev pointer is nil, it must be the first in one of
	     the lists.  */
	  if EQ (FRAME_SCROLL_BARS (f), (*w).horizontal_scroll_bar)     // C 4460
	    /* It's not condemned.  Everything's fine.  */
	    { return; }                                                 // C 4462
	  else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),                  // C 4463-4464
		      (*w).horizontal_scroll_bar)
	    { fset_condemned_scroll_bars (f, (*bar).next); }            // C 4465
	  else
	    /* If its prev pointer is nil, it must be at the front of
	       one or the other!  */
	    { emacs_abort (); }                                         // C 4469
	}
      else
	{ (*XSCROLL_BAR! ((*bar).prev)).next = (*bar).next; }           // C 4472

      if !NILP ((*bar).next)                                            // C 4474
	{ (*XSCROLL_BAR! ((*bar).next)).prev = (*bar).prev; }           // C 4475

      (*bar).next = FRAME_SCROLL_BARS (f);                              // C 4477
      (*bar).prev = Qnil ();                                            // C 4478
      XSETVECTOR! (barobj, bar);                                        // C 4479
      fset_scroll_bars (f, barobj);                                     // C 4480
      if !NILP ((*bar).next)                                            // C 4481
	{ XSETVECTOR! ((*XSCROLL_BAR! ((*bar).next)).prev, bar); }      // C 4482
    }
}

/* Remove all scroll bars on FRAME that haven't been saved since the
   last call to `*condemn_scroll_bars_hook'.  */

/// C 4489-4512: `static void pgtk_judge_scroll_bars (struct frame *f)` - C's `for (; !NILP (bar);
/// bar = next)` is the port's `while` with C's step at the body's end, `pgtk_scroll_bar_remove` is
/// this file's C 4192 definition, and C's two-member assignment `b->next = b->prev = Qnil` is one
/// line per store (`E0381`'s sibling rule, PORTING.md 1.13)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_judge_scroll_bars (f: *mut frame)
{
  let mut bar: Lisp_Object;                                             // C 4492
  let mut next: Lisp_Object;                                            // C 4492

  bar = FRAME_CONDEMNED_SCROLL_BARS (f);                                // C 4494

  /* Clear out the condemned list now so we won't try to process any
     more events on the hapless scroll bars.  */
  fset_condemned_scroll_bars (f, Qnil ());                              // C 4498

  while !NILP (bar)                                                     // C 4500
    {
      let b: *mut scroll_bar = XSCROLL_BAR! (bar);                      // C 4502

      pgtk_scroll_bar_remove (b);                                       // C 4504

      next = (*b).next;                                                 // C 4506
      (*b).next = Qnil ();                                              // C 4507
      (*b).prev = Qnil ();                                              // C 4507's second store

      bar = next;                                                       // C 4500's step
    }

  /* Now there should be no references to the condemned scroll bars,
     and they should get garbage-collected.  */
}

/* <gtk/gtkwindow.h>: `void gtk_window_unfullscreen (GtkWindow *window);` / `void
   gtk_window_fullscreen (GtkWindow *window);` / `void gtk_window_maximize (GtkWindow *window);` /
   `void gtk_window_unmaximize (GtkWindow *window);` - the C 4524-4537 calls (the GTK segment's
   convention: the header alone, no line, since this machine has no GTK tree).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_window_unfullscreen (window: *mut GtkWindow);
  pub fn gtk_window_fullscreen (window: *mut GtkWindow);
  pub fn gtk_window_maximize (window: *mut GtkWindow);
  pub fn gtk_window_unmaximize (window: *mut GtkWindow);
}

/// C: frame.h 460: `ENUM_BF (fullscreen_type) want_fullscreen : 4;` - **read at C 4521 and written
/// at C 4548**, i.e. this unit needs both arms.  `include/frame.rs` 757/607/609 has the mask and
/// the shift/unit of the packed word and no accessor, so the reader is that mirror's `frame_field`
/// (the `src/haikufns.rs` 1312-1318 copy) and the writer is C's own masked write on `bits2` (unit
/// 2; the `FRAME_Z_GROUP!` shape this unit already carries at 1508-1511).
#[cfg (feature = "have_pgtk")]
macro_rules! want_fullscreen {
  ($f:expr) => {
    frame_field ($f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT,
		 FRAME_WANT_FULLSCREEN_MASK) as fullscreen_type };
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(FRAME_WANT_FULLSCREEN_MASK << FRAME_WANT_FULLSCREEN_SHIFT))
                  | (((($v) as u32) & FRAME_WANT_FULLSCREEN_MASK) << FRAME_WANT_FULLSCREEN_SHIFT) };
}

/// C 4514-4549: `static void set_fullscreen_state (struct frame *f)` - `FRAME_GTK_OUTER_WIDGET`
/// is `include/pgtkterm.rs`'s macro and C's `GTK_WINDOW (…)` cast is the port's pointer cast
/// (P-72 rule 3, the `pgtk_text_icon` spelling), `store_frame_param` is this unit's own use-site
/// declaration (C 966's call site, line 221-223), the three `Qfull*` symbols are
/// `include/globals.rs` 2734/2728/3829's, and C's `switch (f->want_fullscreen)` is the port's
/// `match` over the five `FULLSCREEN_*` constants with the two shared-body labels written as one
/// alternative (PORTING.md 2.2; this function's own `macro-body-notes.tsv` row).
#[cfg (feature = "have_pgtk")]
unsafe fn set_fullscreen_state (f: *mut frame)
{
  if FRAME_GTK_OUTER_WIDGET! (f).is_null ()                             // C 4517
    { return; }                                                         // C 4518

  let widget: *mut GtkWindow = FRAME_GTK_OUTER_WIDGET! (f) as *mut GtkWindow;   // C 4520
  match want_fullscreen! (f)                                            // C 4521
    {
      FULLSCREEN_NONE =>                                                // C 4523
	{
	  gtk_window_unfullscreen (widget);                             // C 4524
	  gtk_window_unmaximize (widget);                               // C 4525
	  store_frame_param (f, Qfullscreen (), Qnil ());               // C 4526
	}
      FULLSCREEN_BOTH =>                                                // C 4529
	{
	  gtk_window_unmaximize (widget);                               // C 4530
	  gtk_window_fullscreen (widget);                               // C 4531
	  store_frame_param (f, Qfullscreen (), Qfullboth ());          // C 4532
	}
      FULLSCREEN_MAXIMIZED =>                                           // C 4535
	{
	  gtk_window_unfullscreen (widget);                             // C 4536
	  gtk_window_maximize (widget);                                 // C 4537
	  store_frame_param (f, Qfullscreen (), Qmaximized ());         // C 4538
	}
      FULLSCREEN_WIDTH | FULLSCREEN_HEIGHT =>                           // C 4541-4542
	{
	  /* Restoring from fullscreen to tiled (Bug#81165, Bug#81320).  */
	  gtk_window_unfullscreen (widget);                             // C 4544
	}
      _ =>                                                              // C's `switch` has no `default`
	{
	}
    }

  want_fullscreen! (f, FULLSCREEN_NONE);                                // C 4548
}

/// C 4551-4560: `static void pgtk_fullscreen_hook (struct frame *f)` - `FRAME_VISIBLE_P` is
/// `include/frame.rs` 1120's accessor; C 7442 takes this function's address (the terminal's
/// `fullscreen_hook`), so it stays a plain `unsafe fn` until that slice converts it (the
/// `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_fullscreen_hook (f: *mut frame)
{
  if FRAME_VISIBLE_P (f)                                                // C 4554
    {
      block_input ();                                                   // C 4556
      set_fullscreen_state (f);                                         // C 4557
      unblock_input ();                                                 // C 4558
    }
}

/* C: dispextern.h 3741: `extern void image_destroy_all_bitmaps (Display_Info *);` - the C 4580
   call, with dispextern.h 152's `typedef struct pgtk_display_info Display_Info;` (this unit's own
   record, the `gui_mouse_grabbed` shape).  gtkutil.h 173/177: `extern void xg_display_close
   (Display *dpy);` / `extern void xg_display_close (GdkDisplay *gdpy);` - C 4600's call is the
   GdkDisplay arm for this file (173 is the X11 one; Owed 12's list grows to eight), and
   `include/gtkutil.rs` carries neither.  lisp.h 5195: `extern void delete_keyboard_wait_descriptor
   (int);` - the C 4610 call (`src/term.rs` 188 carries the same use-site declaration).  The three
   stand here with C's own prototypes (P-88 ①).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn image_destroy_all_bitmaps (dpyinfo: *mut pgtk_display_info);
  pub fn xg_display_close (gdpy: *mut GdkDisplay);
  pub fn delete_keyboard_wait_descriptor (fd: c_int);
}
/* <gdk/gdkevents.h>: `void gdk_event_free (GdkEvent *event);` - the C 4588 call (the GTK
   segment's convention: the header alone, no line, since this machine has no GTK tree).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gdk_event_free (event: *mut GdkEvent);
}
/* <gdk/gdkevents.h>: `GdkEvent *gdk_event_copy (const GdkEvent *event);` - the C 6212 call, the
   same header and the same use-site convention as the row above.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gdk_event_copy (event: *const GdkEvent) -> *mut GdkEvent;
}

/* This function is called when the last frame on a display is deleted. */

/// C 4562-4614: `void pgtk_delete_terminal (struct terminal *terminal)` - a **non-`static`** C
/// definition, so the port carries `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2).
/// `terminal->display_info.pgtk` is `include/termhooks.rs` 170-180's union member, whose pointee
/// that header cannot complete, so C's store into `struct pgtk_display_info *` is the port's
/// pointer cast (P-72 rule 3); the four `g_clear_object` calls are that mirror's declared function
/// (`include/gtk.rs` 989) with the parameter's address cast, the shape this unit already uses at
/// C 816-820; `G_OBJECT`/`G_CALLBACK` and `g_signal_handlers_disconnect_by_func` are
/// `include/gtk.rs` 146/1624's, whose third argument is a `gpointer` (C's implicit conversion is
/// spelled `as gpointer`); `emacs_close` is `include/lisp.rs` 4218's; and
/// **`pgtk_delete_display` (C 4612) is this file's own forward-declared definition (C 7314, in the
/// remainder)** - the ledger's Evidence ② table gains that class with count 1.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_delete_terminal (terminal: *mut terminal)
{
  let dpyinfo: *mut pgtk_display_info                                 // C 4566
    = (*terminal).display_info.pgtk as *mut pgtk_display_info;

  /* Protect against recursive calls.  delete_frame in
     delete_terminal calls us back when it deletes our last frame.  */
  if (*terminal).name.is_null ()                                        // C 4570
    { return; }                                                         // C 4571

  block_input ();                                                       // C 4573

  pgtk_im_finish (dpyinfo);                                             // C 4575

  /* Normally, the display is available...  */
  if !(*dpyinfo).gdpy.is_null ()                                        // C 4578
    {
      image_destroy_all_bitmaps (dpyinfo);                              // C 4580

      g_clear_object (&raw mut (*dpyinfo).xg_cursor as *mut *mut GObject);   // C 4582
      g_clear_object (&raw mut (*dpyinfo).vertical_scroll_bar_cursor as *mut *mut GObject);   // C 4583
      g_clear_object (&raw mut (*dpyinfo).horizontal_scroll_bar_cursor as *mut *mut GObject); // C 4584
      g_clear_object (&raw mut (*dpyinfo).invisible_cursor as *mut *mut GObject);   // C 4585
      if !(*dpyinfo).last_click_event.is_null ()                        // C 4586
	{
	  gdk_event_free ((*dpyinfo).last_click_event);                 // C 4588
	  (*dpyinfo).last_click_event = core::ptr::null_mut ();         // C 4589
	}

      /* Disconnect these handlers before the display closes so
	 useless removal signals don't fire.  */
      g_signal_handlers_disconnect_by_func! (G_OBJECT! ((*dpyinfo).gdpy),    // C 4594-4595
					     G_CALLBACK! (pgtk_seat_added_cb),
					     dpyinfo as gpointer);
      g_signal_handlers_disconnect_by_func! (G_OBJECT! ((*dpyinfo).gdpy),    // C 4597-4598
					     G_CALLBACK! (pgtk_seat_removed_cb),
					     dpyinfo as gpointer);
      xg_display_close ((*dpyinfo).gdpy);                               // C 4600

      (*dpyinfo).gdpy = core::ptr::null_mut ();                         // C 4602
    }

  if (*dpyinfo).connection >= 0                                         // C 4605
    { emacs_close ((*dpyinfo).connection); }                            // C 4606

  (*dpyinfo).connection = -1;                                           // C 4608

  delete_keyboard_wait_descriptor (0);                                  // C 4610

  pgtk_delete_display (dpyinfo);                                        // C 4612
  unblock_input ();                                                     // C 4613
}

/* Store F's background color into *BGCOLOR.  */

/// C 4617-4622: `static void pgtk_query_frame_background_color (struct frame *f, Emacs_Color
/// *bgcolor)` - `FRAME_BACKGROUND_PIXEL` is `include/frame.rs` 1347's accessor and `pgtk_query_color`
/// this file's own C 2396 definition (declared at 680's use site).  C 4904 takes this function's
/// address (the terminal's `query_frame_background_color`), so it stays a plain `unsafe fn` until
/// that slice converts it (the `pgtk_ring_bell` note).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_query_frame_background_color (f: *mut frame, bgcolor: *mut Emacs_Color)
{
  (*bgcolor).pixel = FRAME_BACKGROUND_PIXEL (f);                        // C 4620
  pgtk_query_color (f, bgcolor);                                        // C 4621
}

/// C 4624-4632: `static void pgtk_free_pixmap (struct frame *f, Emacs_Pixmap pixmap)` -
/// `Emacs_Pixmap` is `include/dispextern.rs` 773's alias for `*mut emacs_pix_container` (whose
/// `data` is that record's own field, dispextern.h 127), `xfree` is `include/lisp.rs` 3897's, and
/// the `f` parameter C never reads keeps its name with `let _ =` (Deviation 44's shape; C 4919 takes
/// this function's address, the same later conversion).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_free_pixmap (f: *mut frame, pixmap: Emacs_Pixmap)
{
  let _ = f;                                                            // C 4625
  if !pixmap.is_null ()                                                 // C 4627
    {
      xfree ((*pixmap).data as *mut c_void);                            // C 4629
      xfree (pixmap as *mut c_void);                                    // C 4630
    }
}

/// C 4634-4660: `void pgtk_focus_frame (struct frame *f, bool noactivate)` - a **non-`static`** C
/// definition (pgtkterm.h 643 declares it), so the port carries `#[unsafe(no_mangle)]` and
/// `extern "C"` (PORTING.md 0.2); `FRAME_DISPLAY_INFO`/`FRAME_GTK_OUTER_WIDGET`/`FRAME_WIDGET` are
/// the pgtkterm mirror's macros, C's `GTK_WINDOW (…)` is the port's pointer cast (Deviation 55's
/// reading), `gtk_window_present_with_time` is the GTK-segment declaration below, and
/// `gtk_widget_grab_focus` is `include/gtk.rs` 1489's (C 855's other call site).  C 4917 takes this
/// function's address (the terminal's `focus_frame_hook`), which is the same later conversion.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_focus_frame (f: *mut frame, noactivate: bool)
{
  let dpyinfo: *mut pgtk_display_info;                                  // C 4637
  let widget: *mut GtkWidget;                                           // C 4638
  let window: *mut GtkWindow;                                           // C 4639

  dpyinfo = FRAME_DISPLAY_INFO! (f);                                    // C 4641

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null () && !noactivate             // C 4643
    {
      /* The user says it is okay to activate the frame.  Call
	 gtk_window_present_with_time.  If the timestamp specified
	 (actually a display serial on Wayland) is new enough, then
	 any Wayland compositor supporting gtk_surface1_present will
	 cause the frame to be activated.  */

      window = FRAME_GTK_OUTER_WIDGET! (f) as *mut GtkWindow;           // C 4651
      gtk_window_present_with_time (window, (*dpyinfo).last_user_time); // C 4652
      return;                                                           // C 4653
    }

  widget = FRAME_WIDGET! (f);                                           // C 4656

  if !widget.is_null ()                                                 // C 4658
    { gtk_widget_grab_focus (widget); }                                 // C 4659
}

#[cfg (feature = "have_pgtk")]
macro_rules! GTK_IS_CONTAINER {
  ($o:expr) => { (! ($o).is_null ()) };
}

/// C 4662-4670: `static void set_opacity_recursively (GtkWidget *w, gpointer data)` - **its address
/// is C 4669's `GtkCallback` argument**, so the port defines it `unsafe extern "C" fn` (PORTING.md
/// 1.4; the signature is the `GtkCallback` type's exactly).  C's `*(double *) data` is the port's
/// pointer cast (P-72 rule 3), `GTK_IS_CONTAINER` is the unit's own macro below (the mirror's
/// placeholder shape, `include/gtk.rs` 805-817), and `GTK_CONTAINER`/`gtk_container_foreach` are
/// that mirror's 790 and the declaration below.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn set_opacity_recursively (w: *mut GtkWidget, data: gpointer)
{
  gtk_widget_set_opacity (w, *(data as *mut c_double));                 // C 4665

  if GTK_IS_CONTAINER! (w)                                              // C 4667
    {
      gtk_container_foreach (GTK_CONTAINER! (w),                        // C 4668-4669
			     Some (set_opacity_recursively), data);
    }
}

/// C 4672-4700: `static void pgtk_set_frame_alpha (struct frame *f)` - `f->alpha[0]`/`[1]` is
/// `include/frame.rs` 262's two-element array field, `dpyinfo->highlight_frame` is the pgtk
/// display-info record's own field, `FLOATP`/`FIXNUMP`/`XFLOAT_DATA`/`XFIXNUM` are
/// `include/lisp.rs` 921/726/1710/757's, `Vframe_alpha_lower_limit` is `include/globals.rs`
/// 6527's extern, C's `&alpha` handed to the `gpointer` parameter is the port's
/// `&raw mut alpha as gpointer`, and `gtk_widget_queue_resize_no_redraw` is the declaration below.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_frame_alpha (f: *mut frame)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);        // C 4675
  let mut alpha: c_double = 1.0;                                        // C 4676
  let mut alpha_min: c_double = 1.0;                                    // C 4677

  if (*dpyinfo).highlight_frame == f                                    // C 4679
    { alpha = (*f).alpha[0]; }                                          // C 4680
  else
    { alpha = (*f).alpha[1]; }                                          // C 4682

  if alpha < 0.0                                                        // C 4684
    { return; }                                                         // C 4685

  if FLOATP (Vframe_alpha_lower_limit)                                  // C 4687
    { alpha_min = XFLOAT_DATA (Vframe_alpha_lower_limit); }             // C 4688
  else if FIXNUMP (Vframe_alpha_lower_limit)                            // C 4689
    { alpha_min = (XFIXNUM (Vframe_alpha_lower_limit) as c_double) / 100.0; }    // C 4690

  if alpha > 1.0                                                        // C 4692
    { alpha = 1.0; }                                                    // C 4693
  else if alpha < alpha_min && alpha_min <= 1.0                         // C 4694
    { alpha = alpha_min; }                                              // C 4695

  set_opacity_recursively (FRAME_WIDGET! (f), &raw mut alpha as gpointer);   // C 4697
  /* without this, blending mode is strange on wayland. */
  gtk_widget_queue_resize_no_redraw (FRAME_WIDGET! (f));                // C 4699
}

/* <gtk/gtkwindow.h>: `void gtk_window_present_with_time (GtkWindow *window, guint32 timestamp);` -
   the C 4652 call.  <gtk/gtkwidget.h>: `void gtk_widget_set_opacity (GtkWidget *widget, double
   opacity);` - the C 4665 call; `void gtk_widget_queue_resize_no_redraw (GtkWidget *widget);` - the
   C 4699 call.  <gtk/gtkcontainer.h>: `void gtk_container_foreach (GtkContainer *container,
   GtkCallback callback, gpointer callback_data);` - the C 4668 call, with that header's
   `typedef void (*GtkCallback) (GtkWidget *widget, gpointer data);`.  The four are the GTK
   segment's convention (header alone, no line: this machine has no GTK tree), and the two records
   below are gaps of `include/gtk.rs` (Owed 13).  */
#[cfg (feature = "have_pgtk")]
pub type GtkCallback = Option<unsafe extern "C" fn (widget: *mut GtkWidget, data: gpointer)>;
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_window_present_with_time (window: *mut GtkWindow, timestamp: guint32);
/* <gtk/gtkwindow.h>: `void gtk_window_present (GtkWindow *window);` - the C 6155 call (the
   same header's plain entry point; the row above it is the timed one).  */
  pub fn gtk_window_present (window: *mut GtkWindow);
  pub fn gtk_widget_set_opacity (widget: *mut GtkWidget, opacity: c_double);
  pub fn gtk_widget_queue_resize_no_redraw (widget: *mut GtkWidget);
  pub fn gtk_container_foreach (container: *mut GtkContainer, callback: GtkCallback,
				callback_data: gpointer);
}
/* C: gtk/gtk.h: `#define GTK_IS_CONTAINER(w) G_TYPE_CHECK_INSTANCE_TYPE ((w),
   gtk_container_get_type ())` - the C 4667 test.  This unit's own copy of the placeholder shape
   `include/gtk.rs` 805-817 carries for `GTK_IS_MENU`/`GTK_IS_WINDOW`/`GTK_IS_LABEL` ("each keeping
   the macro's name and its result type"): no GTK tree on this machine can measure the real type
   check, so the stand-in is the null test that mirror already uses for its three.  */

/* C 4702-4731: `static void frame_highlight (struct frame *f)` - C defines it `static` and the
   only use is C 4792's call inside this file (no site takes its address), so it stays a plain
   `unsafe fn`; PORTING.md 1.4's `unsafe extern "C" fn` form is for the positions whose address
   C takes.  */
#[cfg (feature = "have_pgtk")]
unsafe fn frame_highlight (f: *mut frame)
{
  block_input ();                                                       // C 4705
  let w: *mut GtkWidget = FRAME_WIDGET! (f);                            // C 4706

  let css: *mut c_char = g_strdup_printf (c"decoration { border: solid %dpx #%06x; }".as_ptr (),   // C 4708
					  (*f).border_width,            // C 4709
					  ((*FRAME_X_OUTPUT! (f)).border_pixel as c_uint) & 0x00ffffff);   // C 4710-4711

  let ctxt: *mut GtkStyleContext = gtk_widget_get_style_context (w);    // C 4713
  let css_provider: *mut GtkCssProvider = gtk_css_provider_new ();      // C 4714
  gtk_css_provider_load_from_data (css_provider, css, -1, core::ptr::null_mut ());   // C 4715
  gtk_style_context_add_provider (ctxt, GTK_STYLE_PROVIDER! (css_provider),   // C 4716
				  GTK_STYLE_PROVIDER_PRIORITY_USER);    // C 4717
  g_free (css as gpointer);                                                         // C 4718

  let old: *mut GtkCssProvider = (*FRAME_X_OUTPUT! (f)).border_color_css_provider;   // C 4720
  (*FRAME_X_OUTPUT! (f)).border_color_css_provider = css_provider;      // C 4721
  if !old.is_null ()                                                    // C 4722
    {
      gtk_style_context_remove_provider (ctxt, GTK_STYLE_PROVIDER! (old));   // C 4724
      g_object_unref (old as gpointer);                                 // C 4725
    }

  unblock_input ();                                                     // C 4728
  gui_update_cursor (f, true);                                          // C 4729
  pgtk_set_frame_alpha (f);                                             // C 4730
}

/* C 4733-4766: `static void frame_unhighlight (struct frame *f)` - `static` again and one call
   site (C 4790), so a plain `unsafe fn`.  C declares its five locals at the top and assigns
   them below; the port keeps C's order with uninitialised `let` bindings (this unit's
   `pgtk_free_devices` shape, C 1869-1870).  */
#[cfg (feature = "have_pgtk")]
unsafe fn frame_unhighlight (f: *mut frame)
{
  let w: *mut GtkWidget;
  let css: *mut c_char;
  let ctxt: *mut GtkStyleContext;
  let css_provider: *mut GtkCssProvider;
  let old: *mut GtkCssProvider;

  block_input ();                                                       // C 4741

  w = FRAME_WIDGET! (f);                                                // C 4743

  css = g_strdup_printf (c"decoration { border: dotted %dpx #ffffff; }".as_ptr (),   // C 4745
			 (*f).border_width);                            // C 4746

  ctxt = gtk_widget_get_style_context (w);                              // C 4748
  css_provider = gtk_css_provider_new ();                               // C 4749
  gtk_css_provider_load_from_data (css_provider, css, -1, core::ptr::null_mut ());   // C 4750
  gtk_style_context_add_provider (ctxt, GTK_STYLE_PROVIDER! (css_provider),   // C 4751
				  GTK_STYLE_PROVIDER_PRIORITY_USER);    // C 4752
  g_free (css as gpointer);                                                         // C 4753

  old = (*FRAME_X_OUTPUT! (f)).border_color_css_provider;               // C 4755
  (*FRAME_X_OUTPUT! (f)).border_color_css_provider = css_provider;      // C 4756
  if !old.is_null ()                                                    // C 4757
    {
      gtk_style_context_remove_provider (ctxt, GTK_STYLE_PROVIDER! (old));   // C 4759
      g_object_unref (old as gpointer);                                 // C 4760
    }

  unblock_input ();                                                     // C 4763
  gui_update_cursor (f, true);                                          // C 4764
  pgtk_set_frame_alpha (f);                                             // C 4765
}

/* C 4768-4793: `void pgtk_frame_rehighlight (struct pgtk_display_info *dpyinfo)` - non-`static`
   in C (pgtkterm.h 648 declares it and C 513 calls it), so it carries `#[unsafe(no_mangle)]` and
   `extern "C"` (PORTING.md 0.2).  C 4776-4779's assignment is one ternary over two `FRAME_*`
   reads, and C 4787's `= 0` is the null pointer of this record's `struct frame *x_focus_frame`
   type.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_frame_rehighlight (dpyinfo: *mut pgtk_display_info)
{
  let old_highlight: *mut frame = (*dpyinfo).highlight_frame;           // C 4772

  if !(*dpyinfo).x_focus_frame.is_null ()                               // C 4774
    {
      (*dpyinfo).highlight_frame                                        // C 4776-4779
	= if FRAMEP (FRAME_FOCUS_FRAME ((*dpyinfo).x_focus_frame))
	    { XFRAME (FRAME_FOCUS_FRAME ((*dpyinfo).x_focus_frame)) }
	  else
	    { (*dpyinfo).x_focus_frame };
      if !FRAME_LIVE_P ((*dpyinfo).highlight_frame)                     // C 4780
	{
	  fset_focus_frame ((*dpyinfo).x_focus_frame, Qnil ());            // C 4782
	  (*dpyinfo).highlight_frame = (*dpyinfo).x_focus_frame;        // C 4783
	}
    }
  else
    { (*dpyinfo).highlight_frame = core::ptr::null_mut (); }            // C 4787

  if !old_highlight.is_null ()                                          // C 4789
    { frame_unhighlight (old_highlight); }                              // C 4790
  if !(*dpyinfo).highlight_frame.is_null ()                             // C 4791
    { frame_highlight ((*dpyinfo).highlight_frame); }                   // C 4792
}

/* C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);` - the C 4729 and
   C 4764 calls.  This unit's own use-site declaration (the `src/haikufns.rs` 138 and
   `src/pgtkfns.rs` 547 shape); `include/dispextern.rs` carries no row for it yet, which Owed 14
   names.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gui_update_cursor (f: *mut frame, on_p: bool);
}

/* The focus has changed, or we have redirected a frame's focus to
   another frame (this happens when a frame uses a surrogate
   mini-buffer frame).  Shift the highlight as appropriate.

   The FRAME argument doesn't necessarily have anything to do with which
   frame is being highlighted or un-highlighted; we only use it to find
   the appropriate X display info.  */                                  // C 4795-4801

/// C 4803-4807: `static void pgtk_frame_rehighlight_hook (struct frame *frame)` - `static` in C,
/// but C 4887 takes its address (`terminal->frame_rehighlight_hook`), so PORTING.md 1.4's
/// `unsafe extern "C" fn` form applies from the slice that lands C 4887; until then it stays a
/// plain `unsafe fn` (the `pgtk_fullscreen_hook` shape, Deviation 55).  `pgtk_frame_rehighlight`
/// is this unit's own C 4769 definition.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_frame_rehighlight_hook (frame: *mut frame)
{
  pgtk_frame_rehighlight (FRAME_DISPLAY_INFO! (frame));                 // C 4806
}

/* Set whether or not the mouse pointer should be visible on frame
   F.  */                                                               // C 4809-4810

/// C 4811-4826: `static void pgtk_toggle_invisible_pointer (struct frame *f, bool invisible)` -
/// `static` in C, address taken at C 4881 (`terminal->toggle_invisible_pointer_hook`), so a plain
/// `unsafe fn` until that slice.  `Emacs_Cursor` is the mirror's alias (`include/pgtkterm.rs`
/// 214/271 carry the two fields C 4816/4818 read), C's `f->pointer_invisible = invisible` is the
/// `pointer_invisible!` writer arm this slice adds to the reader the twenty-second slice landed
/// (the `src/androidterm.rs` 234-243 pair shape, `bits1` the measured word 1), and
/// `gdk_display_flush` is this unit's own use-site declaration below.  The two GDK/GTK entry
/// points are `include/gtk.rs`'s rows (`gdk_window_set_cursor` 564, `gtk_widget_get_window` 883)
/// with the `Emacs_Cursor`-to-`*mut GdkCursor` cast the mirror's parameter needs (the
/// `pgtk_define_frame_cursor` shape, C 2833-2834).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_toggle_invisible_pointer (f: *mut frame, invisible: bool)
{
  let cursor: Emacs_Cursor;
  if invisible                                                          // C 4815
    { cursor = (*FRAME_DISPLAY_INFO! (f)).invisible_cursor; }           // C 4816
  else
    { cursor = (*FRAME_X_OUTPUT! (f)).current_cursor; }                 // C 4818
  gdk_window_set_cursor (gtk_widget_get_window (FRAME_GTK_WIDGET! (f)),   // C 4819
			 cursor as *mut GdkCursor);                     // C 4820
  pointer_invisible! (f, invisible);                                    // C 4821

  /* This is needed to make the pointer visible upon receiving a
     motion notify event.  */
  gdk_display_flush (FRAME_X_DISPLAY! (f));                             // C 4825
}

/* The focus has changed.  Update the frames as necessary to reflect
   the new situation.  Note that we can't change the selected frame
   here, because the Lisp code we are interrupting might become confused.
   Each event gets marked with the frame in which it occurred, so the
   Lisp code can tell when the switch took place by examining the events.  */   // C 4828-4832

/// C 4834-4858: `static void pgtk_new_focus_frame (struct pgtk_display_info *dpyinfo, struct frame
/// *frame)` - `static` in C with one call site (C 513, inside `pgtk_delete_terminal`, which this
/// unit already carries at line 859), so a plain `unsafe fn`; landing it retires the standing
/// `cannot find function pgtk_new_focus_frame` class.  `auto_lower`/`auto_raise` are frame.h
/// 408/405's `bool_bf` flags: the mirror carries the shifts and no accessor (`include/frame.rs`
/// 465-479), so the unit declares the two readers below - the `src/androidterm.rs` 225-233 copy of
/// the same pair (`bits1` is the measured word 1).  C's nested `if`s have no braces; the port's
/// one-line `{ … }` bodies are the `src/androidterm.rs` 668-671 shape.
#[cfg (feature = "have_pgtk")]
macro_rules! auto_raise {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_AUTO_RAISE_SHIFT))
                  | ((($v) as u32) << FRAME_AUTO_RAISE_SHIFT) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! auto_lower {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_AUTO_LOWER_SHIFT))
                  | ((($v) as u32) << FRAME_AUTO_LOWER_SHIFT) };
}
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_new_focus_frame (dpyinfo: *mut pgtk_display_info, frame: *mut frame)
{
  let old_focus: *mut frame = (*dpyinfo).x_focus_frame;                 // C 4837
  /* doesn't work on wayland */

  if frame != (*dpyinfo).x_focus_frame                                  // C 4840
    {
      /* Set this before calling other routines, so that they see
	 the correct value of x_focus_frame.  */
      (*dpyinfo).x_focus_frame = frame;                                 // C 4844

      if !old_focus.is_null () && auto_lower! (old_focus)               // C 4846
	{
	  if !FRAME_GTK_OUTER_WIDGET! (old_focus).is_null ()            // C 4847
	    {
	      gdk_window_lower (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (old_focus)));   // C 4848-4849
	    }
	}

      if !(*dpyinfo).x_focus_frame.is_null () && auto_raise! ((*dpyinfo).x_focus_frame)   // C 4851
	{
	  if !FRAME_GTK_OUTER_WIDGET! ((*dpyinfo).x_focus_frame).is_null ()   // C 4852
	    {
	      gdk_window_raise (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! ((*dpyinfo).x_focus_frame)));   // C 4853-4854
	    }
	}
    }

  pgtk_frame_rehighlight (dpyinfo);                                     // C 4857
}

/// C 4860-4867: `static void pgtk_buffer_flipping_unblocked_hook (struct frame *f)` - `static` in
/// C, address taken at C 4888 (`terminal->buffer_flipping_unblocked_hook`), so a plain `unsafe fn`
/// until that slice.  `flip_cr_context` is this file's own C 271 definition (line 575) and
/// `gtk_widget_queue_draw` this unit's GTK-segment row (line 4795's block, C 3462's call).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_buffer_flipping_unblocked_hook (f: *mut frame)
{
  block_input ();                                                       // C 4863
  flip_cr_context (f);                                                  // C 4864
  gtk_widget_queue_draw (FRAME_GTK_WIDGET! (f));                        // C 4865
  unblock_input ();                                                     // C 4866
}

/* C: gdk/gdkdisplay.h: `void gdk_display_flush (GdkDisplay *display);` - the C 4825 call.  The GTK
   segment's convention (header alone, no line: this machine has no GTK/GDK tree), and
   `GdkDisplay` is `include/gtk.rs` 372-374's record.  Owed 15 names the mirror row.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gdk_display_flush (display: *mut GdkDisplay);
}

/* C hook assignments: termhooks.h degrades `struct frame *`/`struct window *` parameters to
   `*mut c_void` (P-72 rule 3: the mirror cannot name those types) while the definitions keep C's
   pointer; the conversion is C's own implicit one and the ABI does not depend on the pointee.
   `src/term.rs` 140-144 carries the same helper.  */
#[cfg (feature = "have_pgtk")]
macro_rules! hook {
  ($target:ty, $f:expr) => { unsafe {
    let p: *const c_void = $f as *const c_void;
    std::mem::transmute::<*const c_void, Option<$target>> (p) } };
}

/* C: termhooks.h 964: `extern struct terminal *create_terminal (enum output_method, struct
   redisplay_interface *);` - the C 4874 call (`src/terminal.rs` 366 defines it).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn create_terminal (r#type: output_method, rif: *mut c_void) -> *mut terminal;
}
/* C: menu.h 63-65: `extern Lisp_Object pgtk_menu_show (struct frame *, int, int, int, Lisp_Object,
   char **);` and `extern void pgtk_activate_menubar (struct frame *);` - the C 4892/4893
   assignments.  `include/menu.rs` records both as the `HAVE_PGTK` arm's declarations (it holds the
   frozen configuration's arm only, and names them in `docs/ports/menu.md`), so this unit carries
   the two use-site declarations - Owed 16 names the mirror round that makes them live.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn pgtk_menu_show (f: *mut frame, x: c_int, y: c_int, menuflags: c_int,
			 title: Lisp_Object, error_name: *mut *const c_char) -> Lisp_Object;
  pub fn pgtk_activate_menubar (f: *mut frame);
}

/// C 4869-4924: `static struct terminal *pgtk_create_terminal (struct pgtk_display_info *dpyinfo)` -
/// `static` in C with one call site (C 7171, inside C 7169-7172's `pgtk_term_init`, in the
/// remainder), so a plain `unsafe fn`; C 4874's `&pgtk_redisplay_interface` is the unit's C 3757
/// cast shape and `terminal->display_info.pgtk` is `include/termhooks.rs` 170-180's union member
/// (the `pgtk_delete_terminal` reading).  **The thirty-nine assignments are the addresses
/// PORTING.md 1.4 is about**: each names a `static` in C whose address C stores in a hook field,
/// so each is `unsafe extern "C" fn` as of this slice - the twenty-two that were still plain
/// `unsafe fn` are converted by it (the head comment names the two groups) - and each assignment
/// is this unit's `hook!` cast, i.e. C's own implicit conversion of a `struct frame *`-taking
/// function to a `*mut c_void`-taking field.  Two lines carry the **field's** condition rather
/// than C's text, both recorded in Deviation 60: `activate_menubar_hook` (termhooks.h 719-722's
/// `#ifdef HAVE_EXT_MENU_BAR`, off in this machine's arms) and `free_pixmap` (termhooks.h
/// 849-853's `#ifdef HAVE_WINDOW_SYSTEM`).  Eleven of the thirty-nine are defined later in the C
/// file (C 5160-7420's remainder), so the port reaches them through `include/pgtkterm.rs`'s
/// declarations or the two above.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_create_terminal (dpyinfo: *mut pgtk_display_info) -> *mut terminal
{
  let terminal: *mut terminal;

  terminal = create_terminal (output_pgtk, &raw mut pgtk_redisplay_interface as *mut c_void);   // C 4874

  (*terminal).display_info.pgtk = dpyinfo as *mut c_void;              // C 4876
  (*dpyinfo).terminal = terminal;                                       // C 4877

  (*terminal).clear_frame_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_clear_frame);     // C 4879
  (*terminal).ring_bell_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_ring_bell);       // C 4880
  (*terminal).toggle_invisible_pointer_hook
    = hook! (unsafe extern "C" fn (*mut c_void, bool), pgtk_toggle_invisible_pointer);   // C 4881
  (*terminal).update_begin_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_update_begin);    // C 4882
  (*terminal).update_end_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_update_end);      // C 4883
  (*terminal).read_socket_hook
    = hook! (unsafe extern "C" fn (*mut terminal, *mut input_event) -> c_int,
	     pgtk_read_socket);                                         // C 4884
  (*terminal).frame_up_to_date_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_frame_up_to_date);   // C 4885
  (*terminal).mouse_position_hook
    = hook! (unsafe extern "C" fn (*mut *mut c_void, c_int, *mut Lisp_Object,
				   *mut scroll_bar_part, *mut Lisp_Object, *mut Lisp_Object,
				   *mut Time), pgtk_mouse_position);    // C 4886
  (*terminal).frame_rehighlight_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_frame_rehighlight_hook);   // C 4887
  (*terminal).buffer_flipping_unblocked_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_buffer_flipping_unblocked_hook);   // C 4888
  (*terminal).frame_raise_lower_hook
    = hook! (unsafe extern "C" fn (*mut c_void, bool), pgtk_frame_raise_lower);   // C 4889
  (*terminal).frame_visible_invisible_hook
    = hook! (unsafe extern "C" fn (*mut c_void, bool), pgtk_make_frame_visible_invisible);   // C 4890
  (*terminal).fullscreen_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_fullscreen_hook);   // C 4891
  (*terminal).menu_show_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int, Lisp_Object,
				   *mut *const c_char) -> Lisp_Object, pgtk_menu_show);   // C 4892
  #[cfg (feature = "have_ext_menu_bar")]
  { (*terminal).activate_menubar_hook
      = hook! (unsafe extern "C" fn (*mut c_void), pgtk_activate_menubar); }   // C 4893
  (*terminal).popup_dialog_hook
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object) -> Lisp_Object,
	     pgtk_popup_dialog);                                        // C 4894
  (*terminal).change_tab_bar_height_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int), pgtk_change_tab_bar_height);   // C 4895
  (*terminal).set_vertical_scroll_bar_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int),
	     pgtk_set_vertical_scroll_bar);                             // C 4896
  (*terminal).set_horizontal_scroll_bar_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int),
	     pgtk_set_horizontal_scroll_bar);                           // C 4897
  (*terminal).condemn_scroll_bars_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_condemn_scroll_bars);   // C 4898
  (*terminal).redeem_scroll_bar_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_redeem_scroll_bar);   // C 4899
  (*terminal).judge_scroll_bars_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_judge_scroll_bars);   // C 4900
  (*terminal).get_string_resource_hook
    = hook! (unsafe extern "C" fn (*mut c_void, *const c_char, *const c_char) -> *const c_char,
	     pgtk_get_string_resource);                                 // C 4901
  (*terminal).delete_frame_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_destroy_window);  // C 4902
  (*terminal).delete_terminal_hook
    = hook! (unsafe extern "C" fn (*mut terminal), pgtk_delete_terminal);   // C 4903
  (*terminal).query_frame_background_color
    = hook! (unsafe extern "C" fn (*mut c_void, *mut Emacs_Color),
	     pgtk_query_frame_background_color);                        // C 4904
  (*terminal).defined_color_hook
    = hook! (unsafe extern "C" fn (*mut c_void, *const c_char, *mut Emacs_Color, bool, bool)
	     -> bool, pgtk_defined_color);                              // C 4905
  (*terminal).set_new_font_hook
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, c_int) -> Lisp_Object,
	     pgtk_new_font);                                            // C 4906
  (*terminal).set_bitmap_icon_hook
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object) -> bool, pgtk_bitmap_icon);   // C 4907
  (*terminal).implicit_set_name_hook
    = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object),
	     pgtk_implicitly_set_name);                                 // C 4908
  (*terminal).iconify_frame_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_iconify_frame);   // C 4909
  (*terminal).set_scroll_bar_default_width_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_set_scroll_bar_default_width);   // C 4910-4911
  (*terminal).set_scroll_bar_default_height_hook
    = hook! (unsafe extern "C" fn (*mut c_void), pgtk_set_scroll_bar_default_height);   // C 4912-4913
  (*terminal).set_window_size_hook
    = hook! (unsafe extern "C" fn (*mut c_void, bool, c_int, c_int),
	     pgtk_set_window_size);                                     // C 4914
  (*terminal).query_colors
    = hook! (unsafe extern "C" fn (*mut c_void, *mut Emacs_Color, c_int),
	     pgtk_query_colors);                                        // C 4915
  (*terminal).get_focus_frame
    = hook! (unsafe extern "C" fn (*mut c_void) -> Lisp_Object, pgtk_get_focus_frame);   // C 4916
  (*terminal).focus_frame_hook
    = hook! (unsafe extern "C" fn (*mut c_void, bool), pgtk_focus_frame);   // C 4917
  (*terminal).set_frame_offset_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int), pgtk_set_offset);   // C 4918
  #[cfg (feature = "have_window_system")]
  { (*terminal).free_pixmap
      = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void), pgtk_free_pixmap); }   // C 4919

  /* Other hooks are NULL by default.  */

  return terminal;                                                      // C 4923
}

/* C 4926-4931: `struct pgtk_window_is_of_frame_recursive_t` - the record travels to
   `gtk_container_foreach` through its `gpointer` parameter and comes back as the third argument,
   so it is a unit-level `#[repr (C)]` record (the `cairo_rectangle_t` shape, line 4312).  */
#[repr(C)]
pub struct pgtk_window_is_of_frame_recursive_t
{
  pub window: *mut GdkWindow,                  // C 4928: `GdkWindow *window;`
  pub result: bool,                            // C 4929: `bool result;`
  pub emacs_gtk_fixed: *mut GtkWidget,         // C 4930: `GtkWidget *emacs_gtk_fixed;`
}

/* C: emacsgtkfixed.h 30-33: `#define EMACS_TYPE_FIXED emacs_fixed_get_type ()` and
   `#define EMACS_IS_FIXED(obj) G_TYPE_CHECK_INSTANCE_TYPE (obj, EMACS_TYPE_FIXED)` - the C 4941
   test.  Both live inside that header's `#ifdef HAVE_PGTK`, which the frozen configuration does not
   define, so `include/emacsgtkfixed.rs` records them as dead text and defers the two records; this
   unit therefore carries the same placeholder shape the file's `GTK_IS_CONTAINER!` uses (line
   6685): the null test, because no GTK tree on this machine can measure the real type check.  */
#[cfg (feature = "have_pgtk")]
macro_rules! EMACS_IS_FIXED {
  ($o:expr) => { (! ($o).is_null ()) };
}

/// C 4933-4953: `static void pgtk_window_is_of_frame_recursive (GtkWidget *widget, gpointer data)` -
/// **its address is C 4952's `gtk_container_foreach` argument**, so it is `unsafe extern "C" fn`
/// (PORTING.md 1.4) and its signature is exactly that header's `typedef void (*GtkCallback)
/// (GtkWidget *widget, gpointer data);` (`src/pgtkterm.rs`'s own `GtkCallback`, line 6741), which is
/// why the call passes the item without a cast.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_window_is_of_frame_recursive (widget: *mut GtkWidget, data: gpointer)
{
  let datap: *mut pgtk_window_is_of_frame_recursive_t
    = data as *mut pgtk_window_is_of_frame_recursive_t;                 // C 4936

  if (*datap).result                                                    // C 4938
    { return; }                                                         // C 4939

  if EMACS_IS_FIXED! (widget) && widget != (*datap).emacs_gtk_fixed     // C 4941
    { return; }                                                         // C 4942

  if gtk_widget_get_window (widget) == (*datap).window                  // C 4944
    {
      (*datap).result = true;                                           // C 4946
      return;                                                           // C 4947
    }

  if GTK_IS_CONTAINER! (widget)                                         // C 4950
    { gtk_container_foreach (GTK_CONTAINER! (widget),                   // C 4951
			     Some (pgtk_window_is_of_frame_recursive), datap as gpointer); }   // C 4952
}

/// C 4955-4964: `static bool pgtk_window_is_of_frame (struct frame *f, GdkWindow *window)` -
/// `static` with one call site (C 4984) and no address taken, so a plain `unsafe fn`.  C declares
/// `struct pgtk_window_is_of_frame_recursive_t data;` and fills the three fields in C's order
/// (C 4959-4961); Rust has no counterpart for a field-by-field fill of an uninitialised binding
/// (PORTING.md 1.3), so the port uses the same-name field literal with C's three values in C's order
/// (Deviation 47's shape), then hands `&data` over as the `gpointer`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_window_is_of_frame (f: *mut frame, window: *mut GdkWindow) -> bool
{
  let mut data: pgtk_window_is_of_frame_recursive_t = pgtk_window_is_of_frame_recursive_t {
    window,                                                             // C 4959
    result: false,                                                      // C 4960
    emacs_gtk_fixed: FRAME_GTK_WIDGET! (f),                             // C 4961
  };
  pgtk_window_is_of_frame_recursive (FRAME_WIDGET! (f), &raw mut data as gpointer);   // C 4962
  return data.result;                                                   // C 4963
}

/// C 4966-4990: `static struct frame *pgtk_any_window_to_frame (GdkWindow *window)` - `static` with
/// two call sites earlier in this file (C 3519's `pgtk_mouse_position`, this unit's line 5004, and
/// C 7332's `pgtk_any_window_to_frame` caller), no address taken, so a plain `unsafe fn`; landing it
/// retires the standing `cannot find function pgtk_any_window_to_frame` class.  C's
/// `Lisp_Object tail, frame;` pair and `struct frame *f, *found = NULL;` keep C's two lines, and the
/// `FOR_EACH_FRAME` walk is the mirror's macro with the body as its third argument (the C 3506
/// reading at line 5000), `break` inside it included.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_any_window_to_frame (window: *mut GdkWindow) -> *mut frame
{
  let mut tail: Lisp_Object;                                            // C 4971
  let mut frame: Lisp_Object;                                           // C 4971
  let mut f: *mut frame;                                                // C 4972
  let mut found: *mut frame = core::ptr::null_mut ();                    // C 4972

  if window.is_null ()                                                  // C 4974
    { return core::ptr::null_mut (); }                                  // C 4975

  FOR_EACH_FRAME! (tail, frame, {                                       // C 4977
    if !found.is_null ()                                                // C 4979
      { break; }                                                        // C 4980
    f = XFRAME (frame);                                                 // C 4981
    if FRAME_PGTK_P (f)                                                 // C 4982
      {
	if pgtk_window_is_of_frame (f, window)                          // C 4984
	  { found = f; }                                                // C 4985
      }
  });

  return found;                                                         // C 4989
}

/* C: gdk/gdktypes.h: `typedef enum { … GDK_TOUCHPAD_PINCH = 33, … } GdkEventType;` - the C 5000
   comparison.  **Placeholder, not GDK's value**, and the same one `src/xwidget.rs` 2256 carries for
   its own GDK_TOUCHPAD_PINCH read (two units, two copies - P-90 7); UNVERIFIED, since no GDK header
   exists on this machine.  */
#[cfg (feature = "have_pgtk")]
pub const GDK_TOUCHPAD_PINCH: GdkEventType = 12;            // placeholder, not GDK's value
/* C: gdk/gdktypes.h: `typedef enum { GDK_TOUCHPAD_GESTURE_PHASE_BEGIN, …,
   GDK_TOUCHPAD_GESTURE_PHASE_END, … } GdkTouchpadGesturePhase;` - the C 5001-5002 comparison, the
   same placeholder `src/xwidget.rs` 2249 carries (`c_int` here because the union member's `phase` is
   that type); UNVERIFIED.  */
#[cfg (feature = "have_pgtk")]
pub const GDK_TOUCHPAD_GESTURE_PHASE_END: c_int = 2;       // placeholder, not GDK's value

/* C: gtk/gtkwidget.h: `gboolean gtk_widget_translate_coordinates (GtkWidget *src_widget, GtkWidget
   *dest_widget, gint src_x, gint src_y, gint *dest_x, gint *dest_y);` - the C 5007 call.  The
   mirror has no row for it, so this unit declares it (the `src/xwidget.rs` 488-493 shape); Owed 18
   names the mirror round that moves it there.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_widget_translate_coordinates (src_widget: *mut GtkWidget, dest_widget: *mut GtkWidget,
					   src_x: c_int, src_y: c_int, dest_x: *mut c_int,
					   dest_y: *mut c_int) -> gboolean;
}
/* C: math.h: `long int lrint (double x);` - the C 5008/5009 calls.  It is the C library's, and
   `emacs/lib/math.h` (this repository's gnulib mirror) does not name it, so the declaration stands
   here (P-88 1; the `src/xwidget.rs` 160-164 and `src/xfaces.rs` 509 shape).  C's `long int` result
   feeds `gint` parameters, so the two calls carry C's own narrowing (`as c_int`).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn lrint (x: c_double) -> c_long;
}
/* C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);` - the
   C 5060/5064 calls (`src/xfaces.rs` 3730 defines it).  `include/dispextern.rs` 4051 records the row
   as **owed, not declared**, so the prototype stands here (the `src/w32term.rs` 980-982 and
   `src/font.rs` 370-371 shape).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
}

/// C 4992-5033: `static gboolean pgtk_handle_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *data)` - C 6852/6902 take its address through `G_CALLBACK`, so PORTING.md 1.4's
/// `unsafe extern "C" fn` form is what this slice lands (C 6831 takes its
/// address through `G_CALLBACK`); it is defined `unsafe extern "C" fn` (the `pgtk_fullscreen_hook` shape,
/// Deviation 55).  `touchpad_pinch` and `GDK_TOUCHPAD_*` are `include/gtk.rs`'s rows (this slice's
/// mirror round), `PINCH_EVENT` is `include/termhooks.rs`'s, the two `XSET…` macros are
/// `include/frame.rs` 778 and `include/lisp.rs`'s, and C's `gint x, y;` pair is zero-initialised
/// because C only fills it through the translate call's out-pointers (the `win_x`/`win_y` shape,
/// line 4986).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_handle_event (widget: *mut GtkWidget, event: *mut GdkEvent, data: *mut gpointer)
    -> gboolean
{
  let mut f: *mut frame;                                                // C 4995
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 4996
  let mut frame_widget: *mut GtkWidget;                                 // C 4997
  let mut x: c_int = 0;                                                 // C 4998
  let mut y: c_int = 0;                                                 // C 4998

  if (*event).r#type == GDK_TOUCHPAD_PINCH                              // C 5000
      && ((*event).touchpad_pinch.phase != GDK_TOUCHPAD_GESTURE_PHASE_END)   // C 5001-5002
    {
      f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));    // C 5004
      frame_widget = FRAME_GTK_WIDGET! (f);                             // C 5005

      gtk_widget_translate_coordinates (widget, frame_widget,           // C 5007
					lrint ((*event).touchpad_pinch.x) as c_int,   // C 5008
					lrint ((*event).touchpad_pinch.y) as c_int,   // C 5009
					&raw mut x, &raw mut y);            // C 5010
      if !f.is_null ()                                                  // C 5011
	{

	  input_event_bits! (kind, &raw mut inev.ie, PINCH_EVENT);      // C 5014
	  XSETFRAME! (inev.ie.frame_or_window, f);                      // C 5015
	  XSETINT! (inev.ie.x, x as EMACS_INT);                         // C 5016
	  XSETINT! (inev.ie.y, y as EMACS_INT);                         // C 5017
	  inev.ie.arg = list4 (make_float ((*event).touchpad_pinch.dx),   // C 5018
			       make_float ((*event).touchpad_pinch.dy),   // C 5019
			       make_float ((*event).touchpad_pinch.scale),   // C 5020
			       make_float ((*event).touchpad_pinch.angle_delta));   // C 5021
	  inev.ie.modifiers = pgtk_gtk_to_emacs_modifiers (FRAME_DISPLAY_INFO! (f),   // C 5022
							   (*event).touchpad_pinch.state as c_int) as c_uint;   // C 5023
	  inev.ie.device
	    = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5024-5025
	  evq_enqueue (&raw mut inev);                                    // C 5026
	}

      return true as gboolean;                                            // C 5029
    }
  return false as gboolean;                                               // C 5031
}

/// C 5034-5044: `static void pgtk_fill_rectangle (struct frame *f, unsigned long color, int x, int
/// y, int width, int height, bool respect_alpha_background)` - C's prototype at C 114 is the one
/// nineteen ported call sites already use; `static` with no address taken, so a plain `unsafe fn`.
/// `pgtk_begin_cr_clip`/`pgtk_set_cr_source_with_color`/`pgtk_end_cr_clip` come from
/// `include/pgtkterm.rs` 712-720 and `cairo_rectangle`/`cairo_fill` are this unit's own rows
/// (1541-1545/2481-2484); C's four `int` coordinates are widened to `c_double` at the cairo call
/// the way C's own implicit conversion does (line 1820's reading).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_fill_rectangle (f: *mut frame, color: c_ulong, x: c_int, y: c_int,
			       width: c_int, height: c_int, respect_alpha_background: bool)
{
  let cr: *mut cairo_t;                                                 // C 5038
  cr = pgtk_begin_cr_clip (f);                                          // C 5039
  pgtk_set_cr_source_with_color (f, color, respect_alpha_background);   // C 5040
  cairo_rectangle (cr, x as c_double, y as c_double, width as c_double,
		   height as c_double);                                 // C 5041
  cairo_fill (cr);                                                      // C 5042
  pgtk_end_cr_clip (f);                                                 // C 5043
}

/// C 5046-5093: `void pgtk_clear_under_internal_border (struct frame *f)` - non-`static` in C
/// (pgtkterm.h 597) **and its address is C 3777's `pgtk_redisplay_interface` member**, which the
/// port already writes at line 5444, so it carries `#[unsafe(no_mangle)]` and `extern "C"`
/// (PORTING.md 0.2/1.4).  C 5058-5066's nested ternaries are the port's nested `if` expressions
/// (the C 3507 shape), C's `NULL` window argument is `core::ptr::null_mut ()` (the
/// `src/w32term.rs` 1446 shape), and the five `FRAME_*` readers plus `FACE_FROM_ID_OR_NULL` are
/// `include/frame.rs`'s (1375/1036/1038/1105/1113/1576).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_clear_under_internal_border (f: *mut frame)
{
  if FRAME_INTERNAL_BORDER_WIDTH (f) > 0                                // C 5049
      && (FRAME_GTK_OUTER_WIDGET! (f).is_null ()                        // C 5050
	  || gtk_widget_get_realized (FRAME_GTK_OUTER_WIDGET! (f)) != 0)   // C 5051
    {
      let border: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);              // C 5053
      let width: c_int = FRAME_PIXEL_WIDTH (f);                         // C 5054
      let height: c_int = FRAME_PIXEL_HEIGHT (f);                       // C 5055
      let margin: c_int = FRAME_TOP_MARGIN_HEIGHT (f);                  // C 5056
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT (f);        // C 5057
      let face_id: c_int = if !FRAME_PARENT_FRAME (f).is_null ()        // C 5058
	  {
	    if !NILP (Vface_remapping_alist)                            // C 5059
	      { lookup_basic_face (core::ptr::null_mut (), f, CHILD_FRAME_BORDER_FACE_ID) }   // C 5060-5061
	    else
	      { CHILD_FRAME_BORDER_FACE_ID }                            // C 5062
	  }
	else
	  {
	    if !NILP (Vface_remapping_alist)                            // C 5063
	      { lookup_basic_face (core::ptr::null_mut (), f, INTERNAL_BORDER_FACE_ID) }   // C 5064-5065
	    else
	      { INTERNAL_BORDER_FACE_ID }                               // C 5066
	  };
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);          // C 5067

      block_input ();                                                   // C 5069

      if !face.is_null ()                                               // C 5071
	{
	  fill_background_by_face (f, face, 0, margin, width, border);   // C 5073
	  fill_background_by_face (f, face, 0, 0, border, height);       // C 5074
	  fill_background_by_face (f, face, width - border, 0, border,
				   height);                             // C 5075-5076
	  fill_background_by_face (f, face, 0, (height
						- bottom_margin
						- border),
				   width, border);                      // C 5077-5080
	}
      else
	{
	  pgtk_clear_area (f, 0, 0, border, height);                     // C 5084
	  pgtk_clear_area (f, 0, margin, width, border);                 // C 5085
	  pgtk_clear_area (f, width - border, 0, border, height);        // C 5086
	  pgtk_clear_area (f, 0, height - bottom_margin - border,
			   width, border);                              // C 5087-5088
	}

      unblock_input ();                                                 // C 5091
    }
}

/* C: gdk/gdktypes.h: `typedef enum { GDK_SHIFT_MASK = 1 << 0, …, GDK_META_MASK = 1 << 28 }
   GdkModifierType;` - the six C 5180-5189 tests.  `GdkModifierType` itself is this unit's row at
   line 4927 and the mirror has no mask rows, so the six constants are this unit's own copies with
   the **same placeholders** `src/xwidget.rs` 306-316 carries for its own tests (two units, two
   copies - P-90 7); no GDK header exists on this machine, so every value is **UNVERIFIED**.  */
#[cfg (feature = "have_pgtk")]
pub const GDK_MOD1_MASK: GdkModifierType = 1 << 0;    // placeholder, not GDK's value
#[cfg (feature = "have_pgtk")]
pub const GDK_META_MASK: GdkModifierType = 1 << 1;    // placeholder, not GDK's value
#[cfg (feature = "have_pgtk")]
pub const GDK_SUPER_MASK: GdkModifierType = 1 << 2;   // placeholder, not GDK's value
#[cfg (feature = "have_pgtk")]
pub const GDK_HYPER_MASK: GdkModifierType = 1 << 3;   // placeholder, not GDK's value
#[cfg (feature = "have_pgtk")]
pub const GDK_CONTROL_MASK: GdkModifierType = 1 << 4; // placeholder, not GDK's value
#[cfg (feature = "have_pgtk")]
pub const GDK_SHIFT_MASK: GdkModifierType = 1 << 5;   // placeholder, not GDK's value

/* C: gtkutil.h 162: `extern void xg_frame_resized (struct frame *f, int width, int height);` - the
   C 5132 call (`src/gtkutil.rs` 1589 defines it, the C 1144 body).  The mirror's `include/gtkutil.rs`
   has no row for it, so the prototype stands here (P-90 1; Owed 19 names the round).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn xg_frame_resized (f: *mut frame, width: c_int, height: c_int);
}

/// C 5095-5119: `static gboolean pgtk_handle_draw (GtkWidget *widget, cairo_t *cr, gpointer *data)` -
/// C 6836/6890 take its address through `G_CALLBACK`, so PORTING.md 1.4's `unsafe extern "C" fn`
/// form is what this slice lands (it is one of the nineteen this slice's `pgtk_set_event_handler`
/// converts; the `pgtk_fullscreen_hook` shape, Deviation 55).  `cr_surface_visible_bell` is
/// `include/pgtkterm.rs`'s member, `FRAME_CR_ACTIVE_CONTEXT!` this unit's own C 73 macro (line 293,
/// the setter included), and the three cairo entry points are the mirror's rows (`cairo_get_target`,
/// `cairo_set_source_surface`, `cairo_paint`) with C's two integer `0`s widened to `c_double` (the
/// `cairo_set_source_surface` reading at line 5551).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_handle_draw (widget: *mut GtkWidget, cr: *mut cairo_t, data: *mut gpointer)
    -> gboolean
{
  let mut f: *mut frame;                                                // C 5098

  let win: *mut GdkWindow = gtk_widget_get_window (widget);             // C 5100

  if !win.is_null ()                                                    // C 5102
    {
      let mut src: *mut cairo_surface_t = core::ptr::null_mut ();       // C 5104
      f = pgtk_any_window_to_frame (win);                               // C 5105
      if !f.is_null ()                                                  // C 5106
	{
	  src = (*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell;         // C 5108
	  if src.is_null () && !FRAME_CR_ACTIVE_CONTEXT! (f).is_null ()   // C 5109
	    { src = cairo_get_target (FRAME_CR_ACTIVE_CONTEXT! (f)); }   // C 5110
	}
      if !src.is_null ()                                                // C 5112
	{
	  cairo_set_source_surface (cr, src, 0 as c_double, 0 as c_double);   // C 5114
	  cairo_paint (cr);                                             // C 5115
	}
    }
  return false as gboolean;                                             // C 5118
}

/// C 5121-5135: `static void size_allocate (GtkWidget *widget, GtkAllocation *alloc, gpointer
/// user_data)` - C 6860 takes its address through `G_CALLBACK`, so the port defines it `unsafe extern "C" fn` here (PORTING.md 1.4; C 6831
/// takes its address through `G_CALLBACK`).  C 5128's `f = user_data` is the `gpointer`-to-`struct frame *`
/// conversion, which the port writes out; `pgtk_cr_update_surface_desired_size` is
/// `include/pgtkterm.rs`'s row and `xg_frame_resized` the declaration above.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn size_allocate (widget: *mut GtkWidget, alloc: *mut GtkAllocation, user_data: gpointer)
{
  let mut f: *mut frame = pgtk_any_window_to_frame (gtk_widget_get_window (widget));   // C 5125

  if f.is_null ()                                                       // C 5127
    { f = user_data as *mut frame; }                                    // C 5128

  if !f.is_null ()                                                      // C 5130
    {
      xg_frame_resized (f, (*alloc).width, (*alloc).height);            // C 5132
      pgtk_cr_update_surface_desired_size (f, (*alloc).width, (*alloc).height, false);   // C 5133
    }
}

/// C 5137-5164: `static void get_modifier_values (int *mod_ctrl, int *mod_meta, int *mod_alt, int
/// *mod_hyper, int *mod_super)` - `static`, one call site (C 5176, this unit's own
/// `pgtk_gtk_to_emacs_modifiers`) and one in C 5204's `pgtk_emacs_to_gtk_modifiers` (the
/// forty-first slice's neighbour), no address taken, so a plain `unsafe fn`.  The five modifier
/// constants and `Fget`/`Qmodifier_value` are `include/termhooks.rs`'s and `include/globals.rs`'s,
/// and `XFIXNUM (tem) & INT_MAX` keeps C's own mask before the narrowing to `c_int` (`INT_MAX` from
/// the `<limits.h>` edge above, which is `lisp.h`'s).
#[cfg (feature = "have_pgtk")]
unsafe fn get_modifier_values (mod_ctrl: *mut c_int, mod_meta: *mut c_int, mod_alt: *mut c_int,
			       mod_hyper: *mut c_int, mod_super: *mut c_int)
{
  let mut tem: Lisp_Object;                                             // C 5141

  *mod_ctrl = ctrl_modifier;                                            // C 5143
  *mod_meta = meta_modifier;                                            // C 5144
  *mod_alt = alt_modifier;                                              // C 5145
  *mod_hyper = hyper_modifier;                                          // C 5146
  *mod_super = super_modifier;                                          // C 5147

  tem = Fget (Vx_ctrl_keysym, Qmodifier_value ());                      // C 5149
  if INTEGERP (tem)                                                     // C 5150
    { *mod_ctrl = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }    // C 5151
  tem = Fget (Vx_alt_keysym, Qmodifier_value ());                       // C 5152
  if INTEGERP (tem)                                                     // C 5153
    { *mod_alt = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }     // C 5154
  tem = Fget (Vx_meta_keysym, Qmodifier_value ());                      // C 5155
  if INTEGERP (tem)                                                     // C 5156
    { *mod_meta = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }    // C 5157
  tem = Fget (Vx_hyper_keysym, Qmodifier_value ());                     // C 5158
  if INTEGERP (tem)                                                     // C 5159
    { *mod_hyper = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }   // C 5160
  tem = Fget (Vx_super_keysym, Qmodifier_value ());                     // C 5161
  if INTEGERP (tem)                                                     // C 5162
    { *mod_super = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }   // C 5163
}

/// C 5166-5192: `int pgtk_gtk_to_emacs_modifiers (struct pgtk_display_info *dpyinfo, int state)` -
/// non-`static` in C (pgtkterm.h 668, which `include/pgtkterm.rs` 689 declares) and no address is
/// taken, so `#[unsafe(no_mangle)]` + `extern "C"` (PORTING.md 0.2).  C declares its five locals
/// uninitialised and `get_modifier_values` fills them through the pointers, so the port
/// zero-initialises them (the `win_x`/`dummy_x` shape - an uninitialised binding handed to
/// `&raw mut` is an `E0381`, measured in the fortieth slice); C's `state & GDK_*_MASK` tests keep
/// C's spelling (both sides are `GdkModifierType`/`c_int`).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_gtk_to_emacs_modifiers (dpyinfo: *mut pgtk_display_info, state: c_int)
    -> c_int
{
  let mut mod_ctrl: c_int = 0;                                          // C 5169
  let mut mod_meta: c_int = 0;                                          // C 5170
  let mut mod_alt: c_int = 0;                                           // C 5171
  let mut mod_hyper: c_int = 0;                                         // C 5172
  let mut mod_super: c_int = 0;                                         // C 5173
  let mut mod_: c_int;                                                  // C 5174 (`int mod`)

  get_modifier_values (&raw mut mod_ctrl, &raw mut mod_meta, &raw mut mod_alt,
		       &raw mut mod_hyper, &raw mut mod_super);         // C 5176-5177

  mod_ = 0;                                                             // C 5179
  if state & GDK_SHIFT_MASK != 0                                        // C 5180
    { mod_ |= shift_modifier; }                                         // C 5181
  if state & GDK_CONTROL_MASK != 0                                      // C 5182
    { mod_ |= mod_ctrl; }                                               // C 5183
  if state & GDK_META_MASK != 0 || state & GDK_MOD1_MASK != 0           // C 5184
    { mod_ |= mod_meta; }                                               // C 5185
  if state & GDK_SUPER_MASK != 0                                        // C 5186
    { mod_ |= mod_super; }                                              // C 5187
  if state & GDK_HYPER_MASK != 0                                        // C 5188
    { mod_ |= mod_hyper; }                                              // C 5189

  return mod_;                                                          // C 5191
}

/* C: gdk/gdkkeysyms.h: `#define GDK_KEY_Shift_L 0xffe1`, `GDK_KEY_Hyper_R 0xffee`,
   `GDK_KEY_ISO_Lock 0xfe01`, `GDK_KEY_ISO_Level5_Lock 0xfe13`, `GDK_KEY_Mode_switch 0xff7e` and
   `GDK_KEY_Num_Lock 0xff7f` - the six C 5226-5229 comparisons.  That header is not on this machine,
   so each value is **measured** in `/opt/homebrew/include/X11/keysymdef.h` (391/405/415/422/274/276),
   the X11 table GDK's own file is generated from: the names are GDK's and the numbers are the
   measured X11 keysyms (C 5407's `0xff50 <= x < 0xff60` comment corroborates the same table).  */
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Shift_L: c_uint = 0xffe1;            // keysymdef.h 391: `#define XK_Shift_L`
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Hyper_R: c_uint = 0xffee;            // keysymdef.h 405: `#define XK_Hyper_R`
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_ISO_Lock: c_uint = 0xfe01;           // keysymdef.h 415: `#define XK_ISO_Lock`
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_ISO_Level5_Lock: c_uint = 0xfe13;    // keysymdef.h 422
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Mode_switch: c_uint = 0xff7e;        // keysymdef.h 274: `#define XK_Mode_switch`
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Num_Lock: c_uint = 0xff7f;           // keysymdef.h 276: `#define XK_Num_Lock`

/* C: <glib.h>: `typedef guint32 gunichar;` and `gunichar *g_utf8_to_ucs4 (const gchar *str, glong
   len, glong *items_read, glong *items_written, GError **error);` - the C 5235/5237 pair.  The GTK
   segment's convention (header alone, no line: this machine has no GLib/GTK tree); Owed 20 names
   both rows.  */
#[cfg (feature = "have_pgtk")]
pub type gunichar = u32;
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn g_utf8_to_ucs4 (str_: *const gchar, len: c_long, items_read: *mut c_long,
			 items_written: *mut c_long, error: *mut *mut GError) -> *mut gunichar;
}

/// C 5221-5229: the five `Is…Key` keysym macros, each keeping C's name and its comparison - the C
/// 5407/5408 and 5461 uses are inside `key_press_event` (in the remainder), and `macro_rules!` are
/// visible in *text* order (PORTING.md P-92), so they stand here, above the C 5232 neighbours.
/// C 5297's `guint keysym` is the `c_uint` argument, which is why the four range literals carry
/// C's own widening (`as c_uint`); the six keysym names are the measured rows above.
#[cfg (feature = "have_pgtk")]
macro_rules! IsCursorKey {
  ($keysym:expr) => { (0xff50 as c_uint) <= ($keysym) && ($keysym) < (0xff60 as c_uint) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! IsMiscFunctionKey {
  ($keysym:expr) => { (0xff60 as c_uint) <= ($keysym) && ($keysym) < (0xff6c as c_uint) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! IsKeypadKey {
  ($keysym:expr) => { (0xff80 as c_uint) <= ($keysym) && ($keysym) < (0xffbe as c_uint) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! IsFunctionKey {
  ($keysym:expr) => { (0xffbe as c_uint) <= ($keysym) && ($keysym) < (0xffe1 as c_uint) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! IsModifierKey {
  ($keysym:expr) => {
    ((($keysym) >= GDK_KEY_Shift_L) && (($keysym) <= GDK_KEY_Hyper_R))
    || ((($keysym) >= GDK_KEY_ISO_Lock) && (($keysym) <= GDK_KEY_ISO_Level5_Lock))
    || (($keysym) == GDK_KEY_Mode_switch)
    || (($keysym) == GDK_KEY_Num_Lock) };
}

/// C 5194-5219: `int pgtk_emacs_to_gtk_modifiers (struct pgtk_display_info *dpyinfo, int state)` -
/// non-`static` in C (pgtkterm.h 678, declared by `include/pgtkterm.rs` 691) and no address taken,
/// so `#[unsafe(no_mangle)]` + `extern "C"` (PORTING.md 0.2).  C 5197-5201's five locals are
/// zero-initialised for the `&raw mut` hand-off (the fortieth slice's `E0381` reading) and `mask`
/// is a plain writer, so only the five need it.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_emacs_to_gtk_modifiers (dpyinfo: *mut pgtk_display_info, state: c_int)
    -> c_int
{
  let mut mod_ctrl: c_int = 0;                                          // C 5197
  let mut mod_meta: c_int = 0;                                          // C 5198
  let mut mod_alt: c_int = 0;                                           // C 5199
  let mut mod_hyper: c_int = 0;                                         // C 5200
  let mut mod_super: c_int = 0;                                         // C 5201
  let mut mask: c_int;                                                  // C 5202

  get_modifier_values (&raw mut mod_ctrl, &raw mut mod_meta, &raw mut mod_alt,
		       &raw mut mod_hyper, &raw mut mod_super);         // C 5204-5205

  mask = 0;                                                             // C 5207
  if state & mod_super != 0                                             // C 5208
    { mask |= GDK_SUPER_MASK; }                                         // C 5209
  if state & mod_hyper != 0                                             // C 5210
    { mask |= GDK_HYPER_MASK; }                                         // C 5211
  if state & shift_modifier != 0                                        // C 5212
    { mask |= GDK_SHIFT_MASK; }                                         // C 5213
  if state & mod_ctrl != 0                                              // C 5214
    { mask |= GDK_CONTROL_MASK; }                                       // C 5215
  if state & mod_meta != 0                                              // C 5216
    { mask |= GDK_MOD1_MASK; }                                          // C 5217
  return mask;                                                          // C 5218
}

/// C 5232-5257: `void pgtk_enqueue_string (struct frame *f, gchar *str)` - non-`static` in C
/// (pgtkterm.h 662) and no address taken, so `#[unsafe(no_mangle)]` + `extern "C"`.
/// `g_utf8_to_ucs4`'s four `NULL`s are C's own arguments, the loop's step is C's `ustr++` (the
/// `glyph.offset (1)` shape, line 2357), `make_fixnum (*ustr)` keeps C's widening to `EMACS_INT`,
/// `inev.ie.code` is `c_uint` (`include/termhooks.rs` 117) and `g_free`'s `gpointer` is written out
/// (the thirty-sixth slice's reading).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_enqueue_string (f: *mut frame, str_: *mut gchar)
{
  let mut ustr: *mut gunichar;                                          // C 5235
  let mut uptr: *mut gunichar;                                          // C 5235

  ustr = g_utf8_to_ucs4 (str_, -1 as c_long, core::ptr::null_mut (),
			 core::ptr::null_mut (), core::ptr::null_mut ());   // C 5237
  uptr = ustr;                                                          // C 5237
  if ustr.is_null ()                                                    // C 5238
    { return; }                                                         // C 5239
  while *ustr != 0                                                      // C 5240
    {
      let mut inev: buffered_input_event = core::mem::zeroed ();        // C 5242
      let c: Lisp_Object = make_fixnum (*ustr as EMACS_INT);            // C 5243
      EVENT_INIT (&raw mut inev.ie);                                    // C 5244
      input_event_bits! (kind, &raw mut inev.ie,                        // C 5245
			 if SINGLE_BYTE_CHAR_P (XFIXNAT (c) as intmax_t)
			   { ASCII_KEYSTROKE_EVENT }
			 else
			   { MULTIBYTE_CHAR_KEYSTROKE_EVENT });         // C 5247
      inev.ie.arg = Qnil ();                                            // C 5248
      inev.ie.code = XFIXNAT (c) as c_uint;                             // C 5249
      XSETFRAME! (inev.ie.frame_or_window, f);                          // C 5250
      inev.ie.modifiers = 0;                                            // C 5251
      inev.ie.timestamp = 0;                                            // C 5252
      evq_enqueue (&raw mut inev);                                      // C 5253
      ustr = ustr.offset (1);                                           // C 5240's step
    }

  g_free (uptr as gpointer);                                            // C 5256
}

/// C 5259-5271: `void pgtk_enqueue_preedit (struct frame *f, Lisp_Object preedit)` - non-`static`
/// in C (pgtkterm.h 663) and no address taken, so `#[unsafe(no_mangle)]` + `extern "C"`;
/// `PREEDIT_TEXT_EVENT` is `include/termhooks.rs`'s and `inev` takes the zero-init reading like its
/// neighbour above.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_enqueue_preedit (f: *mut frame, preedit: Lisp_Object)
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5262
  EVENT_INIT (&raw mut inev.ie);                                        // C 5263
  input_event_bits! (kind, &raw mut inev.ie, PREEDIT_TEXT_EVENT);       // C 5264
  inev.ie.arg = preedit;                                                // C 5265
  inev.ie.code = 0;                                                     // C 5266
  XSETFRAME! (inev.ie.frame_or_window, f);                              // C 5267
  inev.ie.modifiers = 0;                                                // C 5268
  inev.ie.timestamp = 0;                                                // C 5269
  evq_enqueue (&raw mut inev);                                          // C 5270
}

/* `Mouse_HLInfo`'s third flag bit - `dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, whose
   mask is `include/dispextern.rs` 2139's `MOUSE_FACE_HIDDEN`.  The mirror carries the mask and no
   accessor, and C 5289/5292 read and write the member, so this unit names the pair the way its
   `mouse_invisible!`/`mouse_moved!` do - the `src/msdos.rs` 3440-3445 copy of the same macro.  */
#[cfg (feature = "have_pgtk")]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* C: dispextern.h 3691: `extern bool clear_mouse_face (Mouse_HLInfo *);` - the C 5291 call
   (`src/xdisp.rs` 30425 defines it).  The mirror carries no row, so the prototype stands here (the
   `src/w32inevt.rs` 496 shape: declared in-unit, in an `unsafe extern "C"` block).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
}

/* C: gdk/gdkkeysyms.h: the twelve `dead_*` keysyms and the six others C 5402-5489 names, besides
   the eight the forty-second slice declared.  That header is not on this machine, so each value is
   **measured** in `/opt/homebrew/include/X11/keysymdef.h` (200/208/209/435/455/263/280/191/457/458/
   459/460/462/463/464/465/467/468/469/470/433), the table GDK's own file is generated from: names
   are GDK's, numbers are the measured X11 keysyms.  `dead_degree` is the exception and is **not**
   here - X11's table has no such keysym, so C's `#ifdef GDK_KEY_dead_degree` is the one arm this
   slice keeps gated (the presence it asks about is exactly what cannot be measured here).  */
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_BackSpace: c_uint = 0xff08;          // keysymdef.h 200
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Escape: c_uint = 0xff1b;             // keysymdef.h 208
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Delete: c_uint = 0xffff;             // keysymdef.h 209
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_ISO_Last_Group_Lock: c_uint = 0xfe0f;   // keysymdef.h 433
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_ISO_Left_Tab: c_uint = 0xfe20;       // keysymdef.h 435
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_ISO_Enter: c_uint = 0xfe34;          // keysymdef.h 455
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_Select: c_uint = 0xff60;             // keysymdef.h 263
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_KP_Space: c_uint = 0xff80;           // keysymdef.h 280
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_VoidSymbol: c_uint = 0xffffff;       // keysymdef.h 191
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_grave: c_uint = 0xfe50;         // keysymdef.h 457
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_acute: c_uint = 0xfe51;         // keysymdef.h 458
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_circumflex: c_uint = 0xfe52;    // keysymdef.h 459
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_tilde: c_uint = 0xfe53;         // keysymdef.h 460
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_macron: c_uint = 0xfe54;        // keysymdef.h 462
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_breve: c_uint = 0xfe55;         // keysymdef.h 463
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_abovedot: c_uint = 0xfe56;      // keysymdef.h 464
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_diaeresis: c_uint = 0xfe57;     // keysymdef.h 465
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_doubleacute: c_uint = 0xfe59;   // keysymdef.h 467
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_caron: c_uint = 0xfe5a;         // keysymdef.h 468
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_cedilla: c_uint = 0xfe5b;       // keysymdef.h 469
#[cfg (feature = "have_pgtk")]
pub const GDK_KEY_dead_ogonek: c_uint = 0xfe5c;        // keysymdef.h 470

/* GDK's extended table lists `dead_degree` after `dead_ogonek`; X11's table (the measured source
   above) has no such keysym, so this one name and its value cannot be measured here.  It is
   therefore declared **only for the arm C's own `#ifdef GDK_KEY_dead_degree` guards**, with a
   placeholder value, and that arm is off in every configuration on this machine - Owed 21 records
   the GDK-target round that replaces both.  */
#[cfg (all (feature = "have_pgtk", feature = "gdk_key_dead_degree"))]
pub const GDK_KEY_dead_degree: c_uint = 0xfe5d;        // placeholder, not a measured value

/// C 5273-5502: `static gboolean key_press_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - C 6864/6897 (in the remainder) take its address through `G_CALLBACK`, so
/// PORTING.md 1.4's `unsafe extern "C" fn` form is what this slice lands (C 6831
/// takes its address through `G_CALLBACK`); it is defined `unsafe extern "C" fn` (the `pgtk_fullscreen_hook`
/// shape, Deviation 55).  **The five `goto done` sites** (C 5369/5383/5398/5480/5490) are a `'done:`
/// block plus `break 'done` (Deviation 54's shape, the `pgtk_redeem_scroll_bar` reading): the block
/// wraps the whole `if (f != 0)` arm, so a `break` lands where C's `goto` lands, and C 5495-5499's
/// label statements follow the block.  C's local `unsigned char copy_buffer[513]` is the port's
/// `[c_uchar; 513]` array with `as_mut_ptr()` for C's decay and `size_of_val` for C's `sizeof`;
/// `event->key` is `include/gtk.rs`'s record, which this slice gives its three missing members
/// (`length`, `string`, `is_modifier` - the last one read through its one-bit storage unit, P-80);
/// the twelve `#ifdef GDK_KEY_dead_*` arms are live (their names and values are measured in X11's
/// table, which GDK's own file is generated from) except `dead_degree`, which X11 does not carry and
/// which therefore keeps C's own `#ifdef` as a feature gate; the `#ifdef HPUX` arm is that
/// configuration feature; `clear_mouse_face`, `mouse_face_hidden!` and the keysym constants above
/// are this slice's own rows.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn key_press_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5276
  let mut nbytes: ptrdiff_t;                                            // C 5277
  let mut hlinfo: *mut Mouse_HLInfo;                                    // C 5278
  let mut f: *mut frame;                                                // C 5279
  let mut dpyinfo: *mut pgtk_display_info;                              // C 5280

  f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 5282
  EVENT_INIT (&raw mut inev.ie);                                        // C 5283
  hlinfo = MOUSE_HL_INFO! (f);                                          // C 5284
  nbytes = 0;                                                           // C 5285

  /* If mouse-highlight is an integer, input clears out
     mouse highlighting.  */
  if !mouse_face_hidden! (hlinfo) && INTEGERP (Vmouse_highlight)        // C 5289
    {
      clear_mouse_face (hlinfo);                                        // C 5291
      mouse_face_hidden! (hlinfo, 1);                                   // C 5292
    }

  'done: {                     /* C 5494's label: the five `goto done` sites land here */
    if !f.is_null ()                                                    // C 5295
      {
	let mut keysym: c_uint;                                         // C 5297
	let mut orig_keysym: c_uint;                                    // C 5297
	/* al%imercury@uunet.uu.net says that making this 81
	   instead of 80 fixed a bug whereby meta chars made
	   his Emacs hang.

	   It seems that some version of XmbLookupString has
	   a bug of not returning XBufferOverflow in
	   status_return even if the input is too long to
	   fit in 81 bytes.  So, we must prepare sufficient
	   bytes for copy_buffer.  513 bytes (256 chars for
	   two-byte character set) seems to be a fairly good
	   approximation.  -- 2000.8.10 handa@etl.go.jp  */
	let mut copy_buffer: [c_uchar; 513] = [0; 513];                 // C 5309
	let copy_bufptr: *mut c_uchar = copy_buffer.as_mut_ptr ();      // C 5310
	let copy_bufsiz: c_int = std::mem::size_of_val (&copy_buffer) as c_int;   // C 5311
	let mut modifiers: c_int;                                       // C 5312
	let mut c: Lisp_Object;                                         // C 5313
	let mut state: c_uint;                                          // C 5314

	dpyinfo = FRAME_DISPLAY_INFO! (f);                              // C 5316

	/* Set the last user time for pgtk_focus_frame to work
	   correctly.  */
	(*dpyinfo).last_user_time = (*event).key.time;                  // C 5320

	state = (*event).key.state;                                     // C 5322

	/* While super is pressed, the input method will always always
	   resend the key events ignoring super.  As a workaround, don't
	   filter key events with super or hyper pressed.  */
	if (*event).key.state & (GDK_SUPER_MASK as c_uint | GDK_HYPER_MASK as c_uint) == 0   // C 5327
	  {
	    if pgtk_im_filter_keypress (f, &raw mut (*event).key)       // C 5329
	      { return true as gboolean; }                              // C 5330
	  }

	state |= pgtk_emacs_to_gtk_modifiers (FRAME_DISPLAY_INFO! (f),  // C 5333
					      extra_keyboard_modifiers as c_int) as c_uint;   // C 5334
	modifiers = state as c_int;                                     // C 5335

	/* This will have to go some day...  */

	/* make_lispy_event turns chars into control chars.
	   Don't do it here because XLookupString is too eager.  */
	state &= ! (GDK_CONTROL_MASK as c_uint);                        // C 5341
	state &= ! ((GDK_META_MASK | GDK_SUPER_MASK | GDK_HYPER_MASK | GDK_MOD1_MASK) as c_uint);   // C 5342-5343

	nbytes = (*event).key.length as ptrdiff_t;                      // C 5345
	if nbytes > copy_bufsiz as ptrdiff_t                            // C 5346
	  { nbytes = copy_bufsiz as ptrdiff_t; }                        // C 5347
	memcpy (copy_bufptr as *mut c_void, (*event).key.string as *const c_void,
		nbytes as usize);                                       // C 5348

	keysym = (*event).key.keyval;                                   // C 5350
	orig_keysym = keysym;                                           // C 5351

	/* Common for all keysym input events.  */
	XSETFRAME! (inev.ie.frame_or_window, f);                        // C 5354
	inev.ie.modifiers
	  = pgtk_gtk_to_emacs_modifiers (FRAME_DISPLAY_INFO! (f), modifiers) as c_uint;   // C 5355-5356
	inev.ie.timestamp = (*event).key.time as Time;                  // C 5357

	/* First deal with keysyms which have defined
	   translations to characters.  */
	if keysym >= 32 && keysym < 128                                 // C 5361
	  /* Avoid explicitly decoding each ASCII character.  */
	  {
	    input_event_bits! (kind, &raw mut inev.ie, ASCII_KEYSTROKE_EVENT);   // C 5364
	    inev.ie.code = keysym;                                      // C 5365

	    inev.ie.device
	      = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5367-5368
	    break 'done;                    /* C 5369's `goto done` */
	  }

	/* Keysyms directly mapped to Unicode characters.  */
	if keysym >= 0x01000000 && keysym <= 0x0110FFFF                  // C 5373
	  {
	    input_event_bits! (kind, &raw mut inev.ie,                   // C 5375-5376
			       if keysym < 0x01000080
				 { ASCII_KEYSTROKE_EVENT }
			       else
				 { MULTIBYTE_CHAR_KEYSTROKE_EVENT });    // C 5377-5378
	    inev.ie.code = keysym & 0xFFFFFF;                            // C 5379

	    inev.ie.device
	      = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5381-5382
	    break 'done;                    /* C 5383's `goto done` */
	  }

	/* Now non-ASCII.  */
	if HASH_TABLE_P (Vpgtk_keysym_table)                            // C 5387
	    && { c = Fgethash (make_fixnum (keysym as EMACS_INT),        // C 5388
			       Vpgtk_keysym_table, Qnil ()); FIXNATP (c) }   // C 5389
	  {
	    input_event_bits! (kind, &raw mut inev.ie,                   // C 5391-5392
			       if SINGLE_BYTE_CHAR_P (XFIXNAT (c) as intmax_t)
				 { ASCII_KEYSTROKE_EVENT }
			       else
				 { MULTIBYTE_CHAR_KEYSTROKE_EVENT });    // C 5393
	    inev.ie.code = XFIXNAT (c) as c_uint;                        // C 5394

	    inev.ie.device
	      = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5396-5397
	    break 'done;                    /* C 5398's `goto done` */
	  }

	/* Random non-modifier sorts of keysyms.  */
	if (((keysym >= GDK_KEY_BackSpace && keysym <= GDK_KEY_Escape)   // C 5402
	     || keysym == GDK_KEY_Delete                                 // C 5403
	     || (keysym >= GDK_KEY_ISO_Left_Tab && keysym <= GDK_KEY_ISO_Enter)   // C 5405
	     || IsCursorKey! (keysym)                                    // C 5407
	     || IsMiscFunctionKey! (keysym)                              // C 5408
	     || {                                                        // C 5409-5415 (`#ifdef HPUX`)
		  /* This recognizes the "extended function
		     keys".  It seems there's no cleaner way.
		     Test IsModifierKey to avoid handling
		     mode_switch incorrectly.  */
		  { #[cfg (feature = "hpux")]
		    { GDK_KEY_Select <= keysym && keysym < GDK_KEY_KP_Space }
		    #[cfg (not (feature = "hpux"))]
		    { false } }
		}
	     || orig_keysym == GDK_KEY_dead_circumflex                 // C 5417
	     || orig_keysym == GDK_KEY_dead_grave                      // C 5420
	     || orig_keysym == GDK_KEY_dead_tilde                      // C 5423
	     || orig_keysym == GDK_KEY_dead_diaeresis                  // C 5426
	     || orig_keysym == GDK_KEY_dead_macron                     // C 5429
	     || {                                                      // C 5431-5433
		  { #[cfg (feature = "gdk_key_dead_degree")]
		    { orig_keysym == GDK_KEY_dead_degree }
		    #[cfg (not (feature = "gdk_key_dead_degree"))]
		    { false } }
		}
	     || orig_keysym == GDK_KEY_dead_acute                      // C 5435
	     || orig_keysym == GDK_KEY_dead_cedilla                    // C 5438
	     || orig_keysym == GDK_KEY_dead_breve                      // C 5441
	     || orig_keysym == GDK_KEY_dead_ogonek                     // C 5444
	     || orig_keysym == GDK_KEY_dead_caron                      // C 5447
	     || orig_keysym == GDK_KEY_dead_doubleacute                // C 5450
	     || orig_keysym == GDK_KEY_dead_abovedot                   // C 5453
	     || IsKeypadKey! (keysym)                                   // C 5455
	     || IsFunctionKey! (keysym)                                 // C 5456
	     /* Any "vendor-specific" key is ok.  */
	     || (orig_keysym & (1 << 28)) != 0                          // C 5458
	     || (keysym != GDK_KEY_VoidSymbol && nbytes == 0))          // C 5459
	    && !(((*event).key.is_modifier & 1) != 0                    // C 5460
		 || IsModifierKey! (orig_keysym)                        // C 5461
		 /* The symbols from GDK_KEY_ISO_Lock
		    to GDK_KEY_ISO_Last_Group_Lock
		    don't have real modifiers but
		    should be treated similarly to
		    Mode_switch by Emacs. */
		 || (GDK_KEY_ISO_Lock <= orig_keysym                   // C 5468
		     && orig_keysym <= GDK_KEY_ISO_Last_Group_Lock)))   // C 5469
	  {
	    /* make_lispy_event will convert this to a symbolic
	       key.  */
	    input_event_bits! (kind, &raw mut inev.ie, NON_ASCII_KEYSTROKE_EVENT);   // C 5475
	    inev.ie.code = keysym;                                      // C 5476

	    inev.ie.device
	      = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5478-5479
	    break 'done;                    /* C 5480's `goto done` */
	  }

	{
	  input_event_bits! (kind, &raw mut inev.ie, MULTIBYTE_CHAR_KEYSTROKE_EVENT);   // C 5484
	  inev.ie.arg = make_unibyte_string (copy_bufptr as *const c_char, nbytes);   // C 5485
	  inev.ie.device
	    = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 5486-5487

	  if keysym == GDK_KEY_VoidSymbol                               // C 5489
	    { break 'done; }                /* C 5490's `goto done` */
	}
      }
  }

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5495
    {
      XSETFRAME! (inev.ie.frame_or_window, f);                          // C 5497
      evq_enqueue (&raw mut inev);                                      // C 5498
    }

  return true as gboolean;                                              // C 5501
}

/* C: keyboard.h 513-514: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object,
   Lisp_Object, ptrdiff_t);` - the C 5555 call (`src/keyboard.rs` 4295 defines it; `src/dispnew.rs`
   281 carries the same prototype).  The mirror has no row, so it stands here (the
   `clear_mouse_face` shape, P-90 1).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gen_help_event (string: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
			 object: Lisp_Object, pos: ptrdiff_t);
}
/* C: dispextern.h 3614-3616: `extern Lisp_Object help_echo_string, help_echo_window; extern
   Lisp_Object help_echo_object, previous_help_echo_string;` - the C 5554 write.  The mirror
   declares the four inside `pub mod dispextern_externs`, which no glob import brings into this
   unit, so the one this slice touches stands here (the `src/dispnew.rs` 283-287 shape).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  static mut help_echo_string: Lisp_Object;
}
/* C 5558's `NorthWestGravity` needs no row here: `include/dispextern.rs` 694's
   `include!("pgtkgui.rs")` brings `include/pgtkgui.rs` 158's `pub const NorthWestGravity: c_int =
   1;` into this graph (X11's `X.h` 422 defines the same value for the Xlib spelling).  */

/// C 5504-5516: `static struct pgtk_display_info *pgtk_display_info_for_display (GdkDisplay *dpy)` -
/// `static` with no address taken, so a plain `unsafe fn`; `x_display_list` is this unit's own
/// definition (line 368) and C's `for (…; dpyinfo; dpyinfo = dpyinfo->next)` is the port's `while`
/// with C's step at the body's end (the `pgtk_redeem_scroll_bar` family's shape), the `gdpy` member (C 154-161's anonymous
/// union spells both `gdpy` and `display`; the mirror keeps the first name and records the alias)
/// and `next` being `include/pgtkterm.rs`'s.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_display_info_for_display (dpy: *mut GdkDisplay) -> *mut pgtk_display_info
{
  let mut dpyinfo: *mut pgtk_display_info;                              // C 5507

  dpyinfo = x_display_list;                                             // C 5509
  while !dpyinfo.is_null ()                                             // C 5509
    {
      if (*dpyinfo).gdpy == dpy                                         // C 5511 (C's `display` alias)

	{ return dpyinfo; }                                             // C 5512

      dpyinfo = (*dpyinfo).next;                                        // C 5509's step
    }

  return core::ptr::null_mut ();                                        // C 5515
}

/// C 5518-5536: `static gboolean key_release_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - C 6879 (in the remainder) takes its address through `G_CALLBACK`, so a plain
/// `unsafe fn` until that slice (the `pgtk_fullscreen_hook` shape, Deviation 55); C 5529's
/// `if (dpyinfo)` carries its comment before the statement, which the port keeps, and the
/// definition above is the call C 5527 makes.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn key_release_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut display: *mut GdkDisplay;                                     // C 5523
  let mut dpyinfo: *mut pgtk_display_info;                              // C 5524

  display = gtk_widget_get_display (widget);                            // C 5526
  dpyinfo = pgtk_display_info_for_display (display);                    // C 5527

  if !dpyinfo.is_null ()                                                // C 5529
    /* This is needed on Wayland because of some brain dead
       compositors.  Without them, we would not have to keep track of
       the serial of key release events.  */
    { (*dpyinfo).last_user_time = (*event).key.time; }                  // C 5533

  return true as gboolean;                                              // C 5535
}

/// C 5538-5568: `static gboolean configure_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6870 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`)
/// until that slice.  `event->configure` is the union member this slice's mirror round adds (C's
/// text reads `window`, `x` and `y`), `any_help_event_p` is this unit's own static (line 361),
/// `help_echo_string` and `gen_help_event` are the two declarations above, `GTK_WINDOW!` is
/// `include/gtk.rs` 778's cast row with `gtk_window_get_position` its 903, and the
/// `win_gravity`/`left_pos`/`top_pos` members are `include/frame.rs`'s.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn configure_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let f: *mut frame = pgtk_any_window_to_frame ((*event).configure.window);   // C 5543

  if !f.is_null () && widget == FRAME_GTK_OUTER_WIDGET! (f)             // C 5545
    {
      if any_help_event_p                                               // C 5547
	{
	  let mut frame: Lisp_Object;                                   // C 5549
	  if !f.is_null ()                                              // C 5550
	    { XSETFRAME! (frame, f); }                                  // C 5551
	  else
	    { frame = Qnil (); }                                        // C 5553
	  help_echo_string = Qnil ();                                   // C 5554
	  gen_help_event (Qnil (), frame, Qnil (), Qnil (), 0);         // C 5555
	}

      if (*f).win_gravity == NorthWestGravity                           // C 5558
	{ gtk_window_get_position (GTK_WINDOW! (widget),                // C 5559
				   &raw mut (*f).left_pos, &raw mut (*f).top_pos); }   // C 5560
      else
	{
	  (*f).top_pos = (*event).configure.y;                          // C 5563
	  (*f).left_pos = (*event).configure.x;                         // C 5564
	}
    }
  return false as gboolean;                                             // C 5567
}

/// C 5570-5615: `static gboolean map_event (GtkWidget *widget, GdkEvent *event, gpointer *user_data)` -
/// C 6856 (in the remainder) takes its address through `G_CALLBACK`, so the port defines it `unsafe extern "C" fn` here (PORTING.md 1.4; C 6831
/// takes its address through `G_CALLBACK` - the `pgtk_fullscreen_hook` shape, Deviation
/// 55).  `event->any` is the union member `include/gtk.rs` already carried, `set_fullscreen_state` is
/// this unit's own C 4514 definition, `FRAME_Z_GROUP` is `include/frame.rs` 1642-1645's *reader* function while `FRAME_Z_GROUP!` (this unit's line 1515) is C 4518-4525's *writer* macro (the reference spells the two with the same name),
/// `z_group_above`/`z_group_below` are `include/frame.rs` 1590-1591's, and `SET_FRAME_VISIBLE!`
/// (frame.rs 1751) is the macro form while `SET_FRAME_ICONIFIED` (1417) is a function, which is why
/// its second argument carries C's conversion (`false as c_int`), and `has_been_visible` is C's `int` field (pgtkterm.h 424), so C's truth tests are `== 0` and its stores are `1` (lines 1008/1039's reading).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn map_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let f: *mut frame = pgtk_any_window_to_frame ((*event).any.window);   // C 5575
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5576

  EVENT_INIT (&raw mut inev.ie);                                        // C 5578
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5579
  inev.ie.arg = Qnil ();                                                // C 5580

  if !f.is_null ()                                                      // C 5582
    {
      let iconified: bool = FRAME_ICONIFIED_P (f);                      // C 5584

      /* Check if fullscreen was specified before we where mapped the
	 first time, i.e. from the command line.  */
      if (*FRAME_X_OUTPUT! (f)).has_been_visible == 0                  // C 5588
	{ set_fullscreen_state (f); }                                   // C 5589

      if !iconified                                                     // C 5591
	{
	  /* The `z-group' is reset every time a frame becomes
	     invisible.  Handle this here.  */
	  if FRAME_Z_GROUP (f) == z_group_above                         // C 5595
	    { pgtk_set_z_group (f, Qabove (), Qnil ()); }               // C 5596
	  else if FRAME_Z_GROUP (f) == z_group_below                    // C 5597
	    { pgtk_set_z_group (f, Qbelow (), Qnil ()); }               // C 5598
	}

      SET_FRAME_VISIBLE! (f, 1);                                        // C 5601
      SET_FRAME_ICONIFIED (f, false as c_int);                          // C 5602
      (*FRAME_X_OUTPUT! (f)).has_been_visible = 1;                      // C 5603

      if iconified                                                      // C 5605
	{
	  input_event_bits! (kind, &raw mut inev.ie, DEICONIFY_EVENT);   // C 5607
	  XSETFRAME! (inev.ie.frame_or_window, f);                      // C 5608
	}
    }

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5612
    { evq_enqueue (&raw mut inev); }                                    // C 5613
  return false as gboolean;                                             // C 5614
}

/// C 5617-5690: `static gboolean window_state_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6858 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`) until
/// that slice.  `event->window_state` is the union member this slice's mirror round adds (C reads
/// `window` and `new_window_state`), `new_state` is `GdkWindowState` with the twelve `GDK_WINDOW_STATE_*`
/// flags that round declares as placeholders, and `store_frame_param` is this unit's own C 120
/// declaration (line 230); C 5649-5666's eighteen-line Wayland comment is carried verbatim.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn window_state_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let f: *mut frame = pgtk_any_window_to_frame ((*event).window_state.window);   // C 5622
  let mut new_state: GdkWindowState;                                    // C 5623
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5624

  new_state = (*event).window_state.new_window_state;                   // C 5626

  EVENT_INIT (&raw mut inev.ie);                                        // C 5628
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5629
  inev.ie.arg = Qnil ();                                                // C 5630

  if new_state & GDK_WINDOW_STATE_FULLSCREEN != 0                       // C 5632
    { store_frame_param (f, Qfullscreen (), Qfullboth ()); }            // C 5633
  else if new_state & GDK_WINDOW_STATE_MAXIMIZED != 0                   // C 5634
    { store_frame_param (f, Qfullscreen (), Qmaximized ()); }           // C 5635
  else if (new_state & GDK_WINDOW_STATE_TOP_TILED) != 0                 // C 5636
	   && (new_state & GDK_WINDOW_STATE_BOTTOM_TILED) != 0          // C 5637
	   && (new_state & GDK_WINDOW_STATE_TOP_RESIZABLE) == 0         // C 5638
	   && (new_state & GDK_WINDOW_STATE_BOTTOM_RESIZABLE) == 0      // C 5639
    { store_frame_param (f, Qfullscreen (), Qfullheight ()); }          // C 5640
  else if (new_state & GDK_WINDOW_STATE_LEFT_TILED) != 0                // C 5641
	   && (new_state & GDK_WINDOW_STATE_RIGHT_TILED) != 0           // C 5642
	   && (new_state & GDK_WINDOW_STATE_LEFT_RESIZABLE) == 0        // C 5643
	   && (new_state & GDK_WINDOW_STATE_RIGHT_RESIZABLE) == 0       // C 5644
    { store_frame_param (f, Qfullscreen (), Qfullwidth ()); }           // C 5645
  else
    { store_frame_param (f, Qfullscreen (), Qnil ()); }                 // C 5647

  /* The Wayland protocol provides no way for the client to know
     whether or not one of its toplevels has actually been
     deiconified.  It only provides a request for clients to iconify a
     toplevel, without even the ability to determine whether or not
     the iconification request was rejected by the display server.

     GDK computes the iconified state by sending a window state event
     containing only GDK_WINDOW_STATE_ICONIFIED immediately after
     gtk_window_iconify is called.  That is error-prone if the request
     to iconify the frame was rejected by the display server, but is
     not the main problem here, as Wayland compositors only rarely
     reject such requests.  GDK also assumes that it can clear the
     iconified state upon receiving the next toplevel configure event
     from the display server.  Unfortunately, such events can be sent
     by Wayland compositors while the frame is iconified, and may also
     not be sent upon deiconification.  So, no matter what Emacs does,
     the iconification state of a frame is likely to be wrong under
     one situation or another.  */

  if new_state & GDK_WINDOW_STATE_ICONIFIED != 0                        // C 5668
    {
      SET_FRAME_ICONIFIED (f, true as c_int);                           // C 5670
      SET_FRAME_VISIBLE! (f, false);                                    // C 5671
    }
  else
    {
      (*FRAME_X_OUTPUT! (f)).has_been_visible = 1;                      // C 5675
      input_event_bits! (kind, &raw mut inev.ie, DEICONIFY_EVENT);      // C 5676
      XSETFRAME! (inev.ie.frame_or_window, f);                          // C 5677
      SET_FRAME_ICONIFIED (f, false as c_int);                          // C 5678
      SET_FRAME_VISIBLE! (f, true);                                     // C 5679
    }

  if new_state & GDK_WINDOW_STATE_STICKY != 0                           // C 5682
    { store_frame_param (f, Qsticky (), Qt ()); }                       // C 5683
  else
    { store_frame_param (f, Qsticky (), Qnil ()); }                     // C 5685

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5687
    { evq_enqueue (&raw mut inev); }                                    // C 5688
  return false as gboolean;                                             // C 5689
}

/// C 5692-5712: `static gboolean delete_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6862 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`) until
/// that slice; `event->any` is the union member already carried and `DELETE_WINDOW_EVENT` is
/// `include/termhooks.rs`'s.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn delete_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let f: *mut frame = pgtk_any_window_to_frame ((*event).any.window);   // C 5696
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5697

  EVENT_INIT (&raw mut inev.ie);                                        // C 5699
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5700
  inev.ie.arg = Qnil ();                                                // C 5701

  if !f.is_null ()                                                      // C 5703
    {
      input_event_bits! (kind, &raw mut inev.ie, DELETE_WINDOW_EVENT);  // C 5705
      XSETFRAME! (inev.ie.frame_or_window, f);                          // C 5706
    }

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5709
    { evq_enqueue (&raw mut inev); }                                    // C 5710
  return true as gboolean;                                              // C 5711
}

/* C: gdk/gdktypes.h: `typedef enum { GDK_NOTIFY_ANCESTOR, …, GDK_NOTIFY_INFERIOR, …,
   GDK_NOTIFY_UNKNOWN } GdkNotifyType;` - the type of `GdkEventCrossing.detail` and the one value the
   reference names (C 5789/5826).  `src/xwidget.rs` 2192-2196 carries the same rows for its own reads
   and a mirror may not define what an includer already has (P-90 7), so the unit keeps its own copy -
   the `GDK_*_MASK` and `GDK_TOUCHPAD_*` shape.  No GDK header exists on this machine, so the value is
   a **placeholder** (Owed 21's GDK-target round).  */
#[cfg (feature = "have_pgtk")]
pub type GdkNotifyType = c_int;
#[cfg (feature = "have_pgtk")]
pub const GDK_NOTIFY_INFERIOR: GdkNotifyType = 2;      // placeholder, not GDK's value

/// C 5723-5767: `static void pgtk_focus_changed (gboolean is_enter, int state, struct
/// pgtk_display_info *dpyinfo, struct frame *frame, union buffered_input_event *bufp)` - `static`
/// with four call sites in this same slice (C 5791/5828/5860/5884) and no address taken, so a plain
/// `unsafe fn`.  C's `gboolean is_enter` is the mirror's `c_int` alias, which is why the four calls
/// pass `TRUE`/`FALSE` as `true as gboolean`/`false as gboolean`; `bufp`'s members are reached
/// through the pointer and C 5744/5760's `kind` writes are `input_event_bits!`;
/// `frame->output_data.pgtk->focus_state` is the mirror's `c_int` field with
/// `include/pgtkterm.rs` 413-415's two constants; `pointer_invisible` is this unit's reader macro
/// (line 3991) and `pgtk_toggle_invisible_pointer` its C 4811 definition.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_focus_changed (is_enter: gboolean, state: c_int,
			      dpyinfo: *mut pgtk_display_info, frame: *mut frame,
			      bufp: *mut buffered_input_event)
{
  if is_enter != 0                                                      // C 5728
    {
      if (*dpyinfo).x_focus_event_frame != frame                        // C 5730
	{
	  pgtk_new_focus_frame (dpyinfo, frame);                        // C 5732
	  (*dpyinfo).x_focus_event_frame = frame;                       // C 5733

	  /* Don't stop displaying the initial startup message
	     for a switch-frame event we don't need.  */
	  /* When run as a daemon, Vterminal_frame is always nil.
	     FIXME: Isn't it actually the other way around?  */
	  (*bufp).ie.arg                                                // C 5739-5743
	    = if (NILP (Vterminal_frame)
		  || !FRAME_PGTK_P (XFRAME (Vterminal_frame))
		  || IS_DAEMON ())
		 && CONSP (Vframe_list)
		 && !NILP (XCDR (Vframe_list))
		{ Qt () }
	      else
		{ Qnil () };
	  input_event_bits! (kind, &raw mut (*bufp).ie, FOCUS_IN_EVENT);   // C 5744
	  XSETFRAME! ((*bufp).ie.frame_or_window, frame);               // C 5745
	}

      (*FRAME_X_OUTPUT! (frame)).focus_state |= state;                  // C 5748

    }
  else
    {
      (*FRAME_X_OUTPUT! (frame)).focus_state &= ! state;                // C 5753

      if (*dpyinfo).x_focus_event_frame == frame                        // C 5755
	{
	  (*dpyinfo).x_focus_event_frame = core::ptr::null_mut ();      // C 5757
	  pgtk_new_focus_frame (dpyinfo, core::ptr::null_mut ());       // C 5758

	  input_event_bits! (kind, &raw mut (*bufp).ie, FOCUS_OUT_EVENT);   // C 5760
	  XSETFRAME! ((*bufp).ie.frame_or_window, frame);               // C 5761
	}

      if pointer_invisible! (frame)                                     // C 5764
	{ pgtk_toggle_invisible_pointer (frame, false); }               // C 5765
    }
}

/// C 5769-5795: `static gboolean enter_notify_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6850 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`) until
/// that slice.  `event->crossing.detail`/`focus` are the union member this slice's mirror round
/// completes; `GDK_NOTIFY_INFERIOR` is that round's placeholder constant and C's `TRUE` is
/// `true as gboolean` (the `pgtk_focus_changed` reading above).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn enter_notify_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5773
  let frame: *mut frame
    = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 5774-5775

  if frame.is_null ()                                                   // C 5777
    { return false as gboolean; }                                       // C 5778

  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (frame);    // C 5780
  let focus_frame: *mut frame = (*dpyinfo).x_focus_frame;               // C 5781
  let focus_state: c_int
    = if !focus_frame.is_null ()                                        // C 5782
	{ (*FRAME_X_OUTPUT! (focus_frame)).focus_state }
      else
	{ 0 };                                                          // C 5783

  EVENT_INIT (&raw mut inev.ie);                                        // C 5785
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5786
  inev.ie.arg = Qnil ();                                                // C 5787

  if (*event).crossing.detail != GDK_NOTIFY_INFERIOR                    // C 5789
      && (*event).crossing.focus != 0                                   // C 5790
      && focus_state & FOCUS_EXPLICIT == 0
    { pgtk_focus_changed (true as gboolean, FOCUS_IMPLICIT, dpyinfo, frame, &raw mut inev); }   // C 5791
  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5792
    { evq_enqueue (&raw mut inev); }                                    // C 5793
  return true as gboolean;                                              // C 5794
}

/// C 5797-5844: `static gboolean leave_notify_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6848 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`);
/// `hlinfo->mouse_face_mouse_frame` is `include/dispextern.rs` 2129's bare-tag pointer, so C's
/// comparison and its `= 0` carry the casts (`frame as *mut c_void`, `core::ptr::null_mut ()`), and
/// the help-event arm is the C 5830-5839 pair the forty-fourth slice declared.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn leave_notify_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5801
  let frame: *mut frame
    = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 5802-5803

  if frame.is_null ()                                                   // C 5805
    { return false as gboolean; }                                       // C 5806

  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (frame);    // C 5808
  let focus_frame: *mut frame = (*dpyinfo).x_focus_frame;               // C 5809
  let focus_state: c_int
    = if !focus_frame.is_null ()                                        // C 5810
	{ (*FRAME_X_OUTPUT! (focus_frame)).focus_state }
      else
	{ 0 };                                                          // C 5811
  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (frame);               // C 5812

  if (*hlinfo).mouse_face_mouse_frame == frame as *mut c_void           // C 5814
    {
      /* If we move outside the frame, then we're
	 certainly no longer on any text in the frame.  */
      clear_mouse_face (hlinfo);                                        // C 5818
      (*hlinfo).mouse_face_mouse_frame = core::ptr::null_mut ();        // C 5819
    }

  EVENT_INIT (&raw mut inev.ie);                                        // C 5822
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5823
  inev.ie.arg = Qnil ();                                                // C 5824

  if (*event).crossing.detail != GDK_NOTIFY_INFERIOR                    // C 5826
      && (*event).crossing.focus != 0                                   // C 5827
      && focus_state & FOCUS_EXPLICIT == 0
    { pgtk_focus_changed (false as gboolean, FOCUS_IMPLICIT, dpyinfo, frame, &raw mut inev); }   // C 5828

  if !frame.is_null ()                                                  // C 5830
    {
      if any_help_event_p                                               // C 5832
	{
	  let mut frame_obj: Lisp_Object;                               // C 5834
	  XSETFRAME! (frame_obj, frame);                                // C 5835
	  help_echo_string = Qnil ();                                   // C 5836
	  gen_help_event (Qnil (), frame_obj, Qnil (), Qnil (), 0);     // C 5837
	}
    }

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5841
    { evq_enqueue (&raw mut inev); }                                    // C 5842
  return true as gboolean;                                              // C 5843
}

/// C 5846-5868: `static gboolean focus_in_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6844 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`);
/// `pgtk_im_focus_in` is `include/pgtkterm.rs` 786's row and the call passes C's `TRUE` as
/// `true as gboolean` with `FOCUS_EXPLICIT` from the mirror.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn focus_in_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5849
  let frame: *mut frame
    = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 5850-5851

  if frame.is_null ()                                                   // C 5853
    { return true as gboolean; }                                        // C 5854

  EVENT_INIT (&raw mut inev.ie);                                        // C 5856
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5857
  inev.ie.arg = Qnil ();                                                // C 5858

  pgtk_focus_changed (true as gboolean, FOCUS_EXPLICIT,
		      FRAME_DISPLAY_INFO! (frame), frame, &raw mut inev);   // C 5860-5861
  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5862
    { evq_enqueue (&raw mut inev); }                                    // C 5863

  pgtk_im_focus_in (frame);                                             // C 5865

  return true as gboolean;                                              // C 5867
}

/// C 5870-5892: `static gboolean focus_out_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static`, address taken at C 6846 (in the remainder), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`);
/// `pgtk_im_focus_out` is `include/pgtkterm.rs` 788's row and C's `FALSE` is
/// `false as gboolean`.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn focus_out_event (widget: *mut GtkWidget, event: *mut GdkEvent, user_data: *mut gpointer)
    -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5873
  let frame: *mut frame
    = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 5874-5875

  if frame.is_null ()                                                   // C 5877
    { return true as gboolean; }                                        // C 5878

  EVENT_INIT (&raw mut inev.ie);                                        // C 5880
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5881
  inev.ie.arg = Qnil ();                                                // C 5882

  pgtk_focus_changed (false as gboolean, FOCUS_EXPLICIT,
		      FRAME_DISPLAY_INFO! (frame), frame, &raw mut inev);   // C 5884-5885
  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 5886
    { evq_enqueue (&raw mut inev); }                                    // C 5887

  pgtk_im_focus_out (frame);                                            // C 5889

  return true as gboolean;                                              // C 5891
}

/* Function to report a mouse movement to the mainstream Emacs code.
   The input handler calls this.

   We have received a mouse movement event, which is given in *event.
   If the mouse is over a different glyph than it was last time, tell
   the mainstream emacs code by setting mouse_moved.  If not, ask for
   another motion event, so we can check again the next time it moves.  */

/// C 5902-5950: `static bool note_mouse_movement (struct frame *frame, const GdkEventMotion
/// *event)` - `static` and never used as a value (C 6033 is its only reference), so a plain
/// `unsafe fn`.  C's `const GdkEventMotion *` is the port's `*mut GdkEventMotion` (the
/// `pgtk_get_device_for_event` pointer shape at line 550; the sibling reading is
/// `src/androidterm.rs` 736-766's `android_note_mouse_movement`, the same body C wrote for
/// Android).  `event->time`'s narrowing into the `Time` field is written out (`as Time`), the two
/// `double` members C 5934-5935 compares against the rectangle's `int` members are promoted
/// explicitly, and both C 5922/5939's `struct frame *` arguments reach the mirror-worded
/// `note_mouse_highlight` through the `*mut c_void` cast its declaration here fixes.
#[cfg (feature = "have_pgtk")]
unsafe fn note_mouse_movement (frame: *mut frame, event: *mut GdkEventMotion) -> bool
{
  let mut r: *mut XRectangle;                                           // C 5906
  let mut dpyinfo: *mut pgtk_display_info;                              // C 5907

  if FRAME_X_OUTPUT! (frame).is_null ()                                 // C 5909
    { return false; }                                                   // C 5910

  dpyinfo = FRAME_DISPLAY_INFO! (frame);                                // C 5912
  (*dpyinfo).last_mouse_movement_time = (*event).time as Time;           // C 5913
  (*dpyinfo).last_mouse_motion_frame = frame;                           // C 5914
  (*dpyinfo).last_mouse_motion_x = (*event).x as c_int;                 // C 5915
  (*dpyinfo).last_mouse_motion_y = (*event).y as c_int;                 // C 5916

  if (*event).window != gtk_widget_get_window (FRAME_GTK_WIDGET! (frame))  // C 5918
    {
      mouse_moved! (frame, true);                                       // C 5920
      (*dpyinfo).last_mouse_scroll_bar = std::ptr::null_mut ();         // C 5921
      note_mouse_highlight (frame as *mut c_void, -1, -1);              // C 5922
      (*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut ();        // C 5923
      (*frame).last_mouse_device
	= pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (frame),
				     event as *mut GdkEvent);           // C 5924-5926
      return true;                                                      // C 5927
    }

  /* Has the mouse moved off the glyph it was on at the last sighting?  */
  r = &raw mut (*dpyinfo).last_mouse_glyph;                             // C 5932
  if frame != (*dpyinfo).last_mouse_glyph_frame                         // C 5933
      || (*event).x < (*r).x as c_double
      || (*event).x >= ((*r).x + (*r).width as c_int) as c_double       // C 5934
      || (*event).y < (*r).y as c_double
      || (*event).y >= ((*r).y + (*r).height as c_int) as c_double      // C 5935
    {
      mouse_moved! (frame, true);                                       // C 5937
      (*dpyinfo).last_mouse_scroll_bar = std::ptr::null_mut ();         // C 5938
      note_mouse_highlight (frame as *mut c_void, (*event).x as c_int,
			    (*event).y as c_int);                       // C 5939
      /* Remember which glyph we're now on.  */
      remember_mouse_glyph (frame, (*event).x as c_int,
			    (*event).y as c_int, r);                    // C 5941
      (*dpyinfo).last_mouse_glyph_frame = frame;                        // C 5942
      (*frame).last_mouse_device
	= pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (frame),
				     event as *mut GdkEvent);           // C 5943-5945
      return true;                                                      // C 5946
    }

  return false;                                                         // C 5949
}

/* C: gtkutil.h 151-152: `extern bool xg_event_is_for_scrollbar (struct frame *, const EVENT *,
   bool for_valuator);` - the C 5991 call, with `typedef GdkEvent EVENT;` being that header's own
   C 24 spelling for this arm; `src/gtkutil.rs` has not reached the definition yet, so it stands
   here (the `note_mouse_highlight`/`gen_help_event` shape, P-88 ①).  dispextern.h 3614-3616 names
   the four help-echo objects; the mirror keeps them inside `pub mod dispextern_externs`, which no
   glob import brings into this unit, so the three the C 6060 call reads stand here beside the
   `help_echo_string` row at line 8077 (C's own prototype line is the citation).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn xg_event_is_for_scrollbar (f: *mut frame, event: *const GdkEvent,
				    for_valuator: bool) -> bool;
  static mut previous_help_echo_string: Lisp_Object;
  static mut help_echo_window: Lisp_Object;
  static mut help_echo_object: Lisp_Object;
  static mut help_echo_pos: ptrdiff_t;
}

/// C 5952-6065: `static gboolean motion_notify_event (GtkWidget *widget, GdkEvent *event,
/// gpointer *user_data)` - `static` with its address taken at C 6874 (`G_CALLBACK
/// (motion_notify_event)`), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`) (the `map_event`
/// family's note).  C 5968's assignment inside the condition (`(device = gdk_event_get_source_device
/// (event)) && …`) has no Rust expression form, so the port keeps C's order in a block: assign,
/// then test the pointer and the device's source.  C's `NULL`/`0` are `std::ptr::null_mut ()` (C
/// 5992, and the `window_part *` argument C 6011 passes as `0`), the two `event->motion.x`/`y`
/// readings are C's implicit `double` -> `int` conversions written out, and `selected_frame` is the
/// mirror's extern static (the `src/xfns.rs` 360 `unsafe { … }` reading).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn motion_notify_event (widget: *mut GtkWidget, event: *mut GdkEvent,
			       user_data: *mut gpointer) -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 5956
  let mut f: *mut frame;                                                // C 5957
  let mut frame: *mut frame;                                            // C 5957
  let mut dpyinfo: *mut pgtk_display_info;                              // C 5958
  let mut hlinfo: *mut Mouse_HLInfo;                                    // C 5959
  let mut device: *mut GdkDevice;                                       // C 5960

  /* Ignore emulated pointer events generated from a touch screen
     event.  */
  if gdk_event_get_pointer_emulated (event) != 0                        // C 5964
      /* The event must not have emerged from a touch device either, as
         GDK does not set pointer_emulated in events generated on
         Wayland as on X, and as the X Input Extension specifies.  */
      || { device = gdk_event_get_source_device (event);                // C 5968
	   !device.is_null ()
	     && gdk_device_get_source (device) == GDK_SOURCE_TOUCHSCREEN }  // C 5969
    { return false as gboolean; }                                       // C 5970

  EVENT_INIT (&raw mut inev.ie);                                        // C 5972
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 5973
  inev.ie.arg = Qnil ();                                                // C 5974

  previous_help_echo_string = help_echo_string;                         // C 5976
  help_echo_string = Qnil ();                                           // C 5977

  frame = pgtk_any_window_to_frame (gtk_widget_get_window (widget));    // C 5979
  dpyinfo = FRAME_DISPLAY_INFO! (frame);                                // C 5980
  f = if gui_mouse_grabbed (dpyinfo) { (*dpyinfo).last_mouse_frame }    // C 5981-5982
      else { pgtk_any_window_to_frame (gtk_widget_get_window (widget)) };
  hlinfo = MOUSE_HL_INFO! (f);                                          // C 5983

  if mouse_face_hidden! (hlinfo)                                        // C 5985
    {
      mouse_face_hidden! (hlinfo, false);                               // C 5987
      clear_mouse_face (hlinfo);                                        // C 5988
    }

  if !f.is_null () && xg_event_is_for_scrollbar (f, event, false)       // C 5991
    { f = std::ptr::null_mut (); }                                      // C 5992

  if !f.is_null ()                                                      // C 5994
    {
      /* Maybe generate a SELECT_WINDOW_EVENT for
         `mouse-autoselect-window' but don't let popup menus
         interfere with this (Bug#1261).  */
      if !NILP (Vmouse_autoselect_window)                               // C 5999
	  /* Don't switch if we're currently in the minibuffer.
	     This tries to work around problems where the
	     minibuffer gets unselected unexpectedly, and where
	     you then have to move your mouse all the way down to
	     the minibuffer to select it.  */
	  && !MINI_WINDOW_P (XWINDOW (selected_window))                 // C 6005
	  /* With `focus-follows-mouse' non-nil create an event
	     also when the target window is on another frame.  */
	  && (f == XFRAME (unsafe { selected_frame })
	      || !NILP (focus_follows_mouse))                           // C 6008
	{
	  let window: Lisp_Object = window_from_coordinates
	    (f as *mut c_void, (*event).motion.x as c_int, (*event).motion.y as c_int,
	     std::ptr::null_mut (), false, false, false);               // C 6010-6011

	  /* A window will be autoselected only when it is not
	     selected now and the last mouse movement event was
	     not in it.  The remainder of the code is a bit vague
	     wrt what a "window" is.  For immediate autoselection,
	     the window is usually the entire window but for GTK
	     where the scroll bars don't count.  For delayed
	     autoselection the window is usually the window's text
	     area including the margins.  */
	  if WINDOWP (window)                                           // C 6021
	      && !EQ (window, last_mouse_window)                        // C 6022
	      && !EQ (window, selected_window)                          // C 6023
	    {
	      input_event_bits! (kind, &raw mut inev.ie, SELECT_WINDOW_EVENT);  // C 6025
	      inev.ie.frame_or_window = window;                         // C 6026
	    }

	  /* Remember the last window where we saw the mouse.  */
	  last_mouse_window = window;                                   // C 6030
	}

      if !note_mouse_movement (f, &raw mut (*event).motion)             // C 6033
	{ help_echo_string = previous_help_echo_string; }               // C 6034
    }
  else
    /* If we move outside the frame, then we're
       certainly no longer on any text in the frame.  */
    { clear_mouse_face (hlinfo); }                                      // C 6039

  /* If the contents of the global variable help_echo_string
     has changed, generate a HELP_EVENT.  */
  let mut do_help: c_int = 0;                                           // C 6043
  if !NILP (help_echo_string) || !NILP (previous_help_echo_string)      // C 6044
    { do_help = 1; }                                                    // C 6045

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 6047
    { evq_enqueue (&raw mut inev); }                                    // C 6048

  if do_help > 0                                                        // C 6050
    {
      let mut frame: Lisp_Object;                                       // C 6052

      if !f.is_null ()                                                  // C 6054
	{ XSETFRAME! (frame, f); }                                      // C 6055
      else
	{ frame = Qnil (); }                                            // C 6057

      any_help_event_p = true;                                          // C 6059
      gen_help_event (help_echo_string, frame, help_echo_window,
		      help_echo_object, help_echo_pos);                 // C 6060-6061
    }

  return true as gboolean;                                              // C 6064
}

/* Prepare a mouse-event in *RESULT for placement in the input queue.

   If the event is a button press, then note that we have grabbed
   the mouse.  */

/// C: gdk/gdkevents.h: `typedef enum { GDK_NOTHING = -1, … GDK_BUTTON_PRESS = 4, …, GDK_BUTTON_RELEASE
/// = 7, … } GdkEventType;` - the C 6084 comparison's constant.  **No GDK tree exists on this
/// machine**, and `src/xwidget.rs` 288 declares its own copy of this name for its own reads, so a
/// mirror may not own it (P-90 7) and this unit keeps its own row - the `GDK_NOTIFY_INFERIOR` shape
/// of the forty-sixth slice.  The value is a **placeholder** and joins Owed 21's round.
/// C: gdk/gdkevents.h: `typedef enum { GDK_SCROLL_UP, GDK_SCROLL_DOWN, GDK_SCROLL_LEFT,
/// GDK_SCROLL_RIGHT, GDK_SCROLL_SMOOTH } GdkScrollDirection;` - the type of C 6234's local `dir`,
/// which the C 6267 accessor writes through a pointer and C 6271-6287 switches on.  **No GDK tree
/// exists on this machine**, and `src/xwidget.rs` declares this type and all five constants for its
/// own reads, so a mirror may not own them (P-90 7) and the unit keeps its own rows - the
/// `GDK_BUTTON_PRESS`/`GDK_BUTTON_RELEASE` shape (the fifty-first slice's named reading caught the
/// six `E0428`s before the mirror could keep them).  The values are **placeholders** (Owed 21).
/// **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
pub type GdkScrollDirection = c_int;

#[cfg (feature = "have_pgtk")]
pub const GDK_SCROLL_UP: GdkScrollDirection = 0;
#[cfg (feature = "have_pgtk")]
pub const GDK_SCROLL_DOWN: GdkScrollDirection = 1;
#[cfg (feature = "have_pgtk")]
pub const GDK_SCROLL_LEFT: GdkScrollDirection = 2;
#[cfg (feature = "have_pgtk")]
pub const GDK_SCROLL_RIGHT: GdkScrollDirection = 3;
#[cfg (feature = "have_pgtk")]
pub const GDK_SCROLL_SMOOTH: GdkScrollDirection = 4;

#[cfg (feature = "have_pgtk")]
pub const GDK_BUTTON_PRESS: GdkEventType = 4;

#[cfg (feature = "have_pgtk")]
pub const GDK_BUTTON_RELEASE: GdkEventType = 7;

/// C 6072-6094: `static Lisp_Object construct_mouse_click (struct input_event *result, const
/// GdkEventButton *event, struct frame *f)` - `static` with no address taken (C 6192/6198 call it),
/// so a plain `unsafe fn`.  C's `const GdkEventButton *` is the port's `*mut GdkEventButton` (the
/// `note_mouse_movement` shape) and its `(GdkEvent *) event` is `event as *mut GdkEvent`.  C's
/// `?:` inside the `|` is the port's `if … { up_modifier } else { down_modifier }`, C 6082's
/// implicit `guint` -> `int` argument and the `unsigned | int` result are written out
/// (`as c_int`, `as c_uint`), the `timestamp` store is `event->time`'s widening (`as Time`), and the
/// two `XSETINT` arguments are C's `double` -> `EMACS_INT` conversions written out.
#[cfg (feature = "have_pgtk")]
unsafe fn construct_mouse_click (result: *mut input_event, event: *mut GdkEventButton,
				 f: *mut frame) -> Lisp_Object
{
  /* Make the event type NO_EVENT; we'll change that when we decide
     otherwise.  */
  input_event_bits! (kind, result, MOUSE_CLICK_EVENT);                  // C 6079
  (*result).code = (*event).button - 1;                                 // C 6080
  (*result).timestamp = (*event).time as Time;                          // C 6081
  (*result).modifiers = (pgtk_gtk_to_emacs_modifiers (FRAME_DISPLAY_INFO! (f),
						      (*event).state as c_int)
			 | (if (*event).r#type == GDK_BUTTON_RELEASE
			    { up_modifier } else { down_modifier })) as c_uint;  // C 6082-6085

  XSETINT! ((*result).x, (*event).x as EMACS_INT);                      // C 6087
  XSETINT! ((*result).y, (*event).y as EMACS_INT);                      // C 6088
  XSETFRAME! ((*result).frame_or_window, f);                            // C 6089
  (*result).arg = Qnil ();                                              // C 6090
  (*result).device = pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f),
						event as *mut GdkEvent);  // C 6091-6092
  return Qnil ();                                                       // C 6093
}

/* C: dispextern.h 3667-3668: `extern Lisp_Object handle_tab_bar_click (struct frame *, int, int,
   bool, int);` - the C 6179 call; frame.h 1911: `extern bool frame_ancestor_p (struct frame *af,
   struct frame *df);` - the C 6149 call.  Neither mirror carries a row (`include/window.rs` and
   `include/frame.rs` were read for them), so both stand here at the use site with C's own
   prototypes (P-88 ①), the frame argument in the `*mut c_void` wording this unit's
   `note_mouse_highlight` row already uses.  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn handle_tab_bar_click (f: *mut c_void, x: c_int, y: c_int, down_p: bool,
			       modifiers: c_int) -> Lisp_Object;
  pub fn frame_ancestor_p (af: *mut frame, df: *mut frame) -> bool;
}

/// C 6096-6226: `static gboolean button_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static` with its address taken at C 6884 (`G_CALLBACK (button_event)`), so the port defines it `unsafe extern "C" fn` here
/// (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`) (the `motion_notify_event` shape).  The body is
/// `motion_notify_event`'s sibling in this file: the same touch-screen guard, the same two GNOME
/// comments, and C's `(device = …) && …` in a block again.  C's `bool` locals are the port's `bool`,
/// C's `NULL`/`0` are `std::ptr::null_mut ()`, the `double` -> `int` coordinate conversions and the
/// `event->button.time` comparison are written out (`as c_int`, `as Time`), and `grabbed` is C's
/// `int` field (`include/pgtkterm.rs` 194) so C 6207/6215's `1 << button` is converted (`as c_int`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn button_event (widget: *mut GtkWidget, event: *mut GdkEvent,
			user_data: *mut gpointer) -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6100
  let mut f: *mut frame;                                                // C 6101
  let mut frame: *mut frame;                                            // C 6101
  let mut dpyinfo: *mut pgtk_display_info;                              // C 6102

  /* If we decide we want to generate an event to be seen
     by the rest of Emacs, we put it here.  */
  let mut tab_bar_p: bool = false;                                      // C 6106
  let mut tool_bar_p: bool = false;                                     // C 6107
  let mut tab_bar_arg: Lisp_Object = Qnil ();                           // C 6108
  let mut device: *mut GdkDevice;                                       // C 6109

  /* Ignore emulated pointer events generated from a touch screen
     event.  */
  if gdk_event_get_pointer_emulated (event) != 0                        // C 6113
      /* The event must not have emerged from a touch device either, as
         GDK does not set pointer_emulated in events generated on
         Wayland as on X, and as the X Input Extension specifies.  */
      || { device = gdk_event_get_source_device (event);                // C 6117
	   !device.is_null ()
	     && gdk_device_get_source (device) == GDK_SOURCE_TOUCHSCREEN }  // C 6118
    { return false as gboolean; }                                       // C 6119

  EVENT_INIT (&raw mut inev.ie);                                        // C 6121
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 6122
  inev.ie.arg = Qnil ();                                                // C 6123

  /* ignore double click and triple click. */
  if (*event).r#type != GDK_BUTTON_PRESS
      && (*event).r#type != GDK_BUTTON_RELEASE                         // C 6126
    { return true as gboolean; }                                        // C 6127

  frame = pgtk_any_window_to_frame (gtk_widget_get_window (widget));    // C 6129
  dpyinfo = FRAME_DISPLAY_INFO! (frame);                                // C 6130

  (*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut ();            // C 6132

  if gui_mouse_grabbed (dpyinfo)                                        // C 6134
    { f = (*dpyinfo).last_mouse_frame; }                                // C 6135
  else
    {
      f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));    // C 6138

      if !f.is_null () && (*event).button.r#type == GDK_BUTTON_PRESS    // C 6140
	  && FRAME_NO_ACCEPT_FOCUS (f) == 0                              // C 6141
	{
	  /* When clicking into a child frame or when clicking
	     into a parent frame with the child frame selected and
	     `no-accept-focus' is not set, select the clicked
	     frame.  */
	  let mut hf: *mut frame = (*dpyinfo).highlight_frame;          // C 6147

	  if !FRAME_PARENT_FRAME (f).is_null ()                         // C 6149
	      || (!hf.is_null () && frame_ancestor_p (f, hf))
	    {
	      block_input ();                                           // C 6151
	      gtk_widget_grab_focus (FRAME_GTK_WIDGET! (f));            // C 6152

	      if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()                // C 6154
		{ gtk_window_present (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f))); }  // C 6155
	      unblock_input ();                                         // C 6156
	    }
	}
    }

  /* Set the last user time, used to activate the frame in
     pgtk_focus_frame.  */
  (*dpyinfo).last_user_time = (*event).button.time;                     // C 6163

  if !f.is_null ()                                                      // C 6165
    {
      /* Is this in the tab-bar?  */
      if WINDOWP ((*f).tab_bar_window)                                  // C 6168
	  && WINDOW_TOTAL_LINES (XWINDOW ((*f).tab_bar_window)) != 0    // C 6169
	{
	  let mut window: Lisp_Object;                                  // C 6171
	  let mut x: c_int = (*event).button.x as c_int;                // C 6172
	  let mut y: c_int = (*event).button.y as c_int;                // C 6173

	  window = window_from_coordinates
	    (f as *mut c_void, x, y, std::ptr::null_mut (), true, true, true);  // C 6175
	  tab_bar_p = EQ (window, (*f).tab_bar_window);                 // C 6176

	  if tab_bar_p                                                  // C 6178
	    { tab_bar_arg = handle_tab_bar_click
		(f as *mut c_void, x, y, (*event).r#type == GDK_BUTTON_PRESS,
		 pgtk_gtk_to_emacs_modifiers (dpyinfo,
					      (*event).button.state as c_int)); }  // C 6179-6181
	}

      if !(tab_bar_p && NILP (tab_bar_arg)) && !tool_bar_p              // C 6184
	{
	  if ignore_next_mouse_click_timeout != 0                       // C 6186
	    {
	      if (*event).r#type == GDK_BUTTON_PRESS                    // C 6188
		  && (*event).button.time as Time
		     > ignore_next_mouse_click_timeout                      // C 6189
		{
		  ignore_next_mouse_click_timeout = 0;                  // C 6191
		  construct_mouse_click (&raw mut inev.ie,
					 &raw mut (*event).button, f);  // C 6192
		}
	      if (*event).r#type == GDK_BUTTON_RELEASE                   // C 6194
		{ ignore_next_mouse_click_timeout = 0; }                // C 6195
	    }
	  else
	    { construct_mouse_click (&raw mut inev.ie,
				     &raw mut (*event).button, f); }    // C 6198

	  if !NILP (tab_bar_arg)                                        // C 6200
	    { inev.ie.arg = tab_bar_arg; }                              // C 6201
	}
    }

  if (*event).r#type == GDK_BUTTON_PRESS                                 // C 6205
    {
      (*dpyinfo).grabbed |= (1 << (*event).button.button) as c_int;      // C 6207
      (*dpyinfo).last_mouse_frame = f;                                   // C 6208

      if !(*dpyinfo).last_click_event.is_null ()                         // C 6210
	{ gdk_event_free ((*dpyinfo).last_click_event); }                // C 6211
      (*dpyinfo).last_click_event = gdk_event_copy (event);              // C 6212
    }
  else
    { (*dpyinfo).grabbed &= ! ((1 << (*event).button.button) as c_int); }  // C 6215

  /* Ignore any mouse motion that happened before this event;
     any subsequent mouse-movement Emacs events should reflect
     only motion after the ButtonPress/Release.  */
  if !f.is_null ()                                                       // C 6220
    { mouse_moved! (f, false); }                                         // C 6221

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT              // C 6223
    { evq_enqueue (&raw mut inev); }                                     // C 6224
  return true as gboolean;                                               // C 6225
}

/* `<math.h>`: `double fabs (double);` - the two C 6295 calls, declared the way `src/editfns.rs`
   182-185 declares it for its own C 4013 call.  `list3` (lisp.h) needs no row here: the first
   reading's `check-mirror-dupes` leg named it as a duplicate - `include/lisp.rs` 3850 already
   declares it in this unit's include graph, beside the `make_float` the four C 6301/6313/6323/6335
   calls also use (which the unit reads bare).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn fabs (x: c_double) -> c_double;
}

/// C 6228-6362: `static gboolean scroll_event (GtkWidget *widget, GdkEvent *event, gpointer
/// *user_data)` - `static` with its address taken at C 6890 (`G_CALLBACK (scroll_event)`), so the port defines it `unsafe extern "C" fn` here
/// (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`).  C's `switch (dir)` is the port's `match` with
/// C's five cases and a trailing `_ => {}` (C's `GDK_SCROLL_SMOOTH` arm is the empty one, kept
/// because C keeps it); the two `?:`-inside-`|=` and `?:`-inside-assignment forms become `if … { … }
/// else { … }`; `inev.ie.kind` is read and written through the unit's bitfield macro; the
/// `double` -> `int` truncations C 6310/6320/6332/6343 performs and the `double` -> `EMACS_INT`
/// conversions of the two `XSETINT` arguments are written out; and C's integer literals inside
/// `double` arithmetic are the port's `100.0` / `* nlines as c_double`.
/// C 6234/6235's three uninitialised locals are zero-initialised (Deviation 12's rule), because the
/// C 6267/6291 accessors hand them out through `&raw mut` and Rust cannot see that they write them.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn scroll_event (widget: *mut GtkWidget, event: *mut GdkEvent,
			user_data: *mut gpointer) -> gboolean
{
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6231
  let mut f: *mut frame;                                                // C 6232
  let mut frame: *mut frame;                                            // C 6232
  let mut dpyinfo: *mut pgtk_display_info;                              // C 6233
  let mut dir: GdkScrollDirection = core::mem::zeroed ();               // C 6234
  let mut delta_x: c_double = core::mem::zeroed ();                     // C 6235
  let mut delta_y: c_double = core::mem::zeroed ();                     // C 6235

  EVENT_INIT (&raw mut inev.ie);                                        // C 6237
  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 6238
  inev.ie.arg = Qnil ();                                                // C 6239

  frame = pgtk_any_window_to_frame (gtk_widget_get_window (widget));    // C 6241
  dpyinfo = FRAME_DISPLAY_INFO! (frame);                                // C 6242

  if gui_mouse_grabbed (dpyinfo)                                        // C 6244
    { f = (*dpyinfo).last_mouse_frame; }                                // C 6245
  else
    { f = pgtk_any_window_to_frame (gtk_widget_get_window (widget)); }  // C 6247

  input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);                 // C 6249
  inev.ie.timestamp = (*event).scroll.time as Time;                     // C 6250
  inev.ie.modifiers
    = pgtk_gtk_to_emacs_modifiers (FRAME_DISPLAY_INFO! (f),
				   (*event).scroll.state as c_int) as c_uint;  // C 6251-6252
  XSETINT! (inev.ie.x, (*event).scroll.x as EMACS_INT);                 // C 6253
  XSETINT! (inev.ie.y, (*event).scroll.y as EMACS_INT);                 // C 6254
  XSETFRAME! (inev.ie.frame_or_window, f);                              // C 6255
  inev.ie.arg = Qnil ();                                                // C 6256

  if gdk_event_is_scroll_stop_event (event) != 0                        // C 6258
    {
      input_event_bits! (kind, &raw mut inev.ie, TOUCH_END_EVENT);      // C 6260
      inev.ie.device
	= pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 6261-6262
      evq_enqueue (&raw mut inev);                                      // C 6263
      return true as gboolean;                                          // C 6264
    }

  if gdk_event_get_scroll_direction (event, &raw mut dir) != 0          // C 6267
    {
      match dir
	{
	  GDK_SCROLL_UP =>
	    {
	      input_event_bits! (kind, &raw mut inev.ie, WHEEL_EVENT);  // C 6272
	      inev.ie.modifiers |= up_modifier as c_uint;               // C 6273
	    }
	  GDK_SCROLL_DOWN =>
	    {
	      input_event_bits! (kind, &raw mut inev.ie, WHEEL_EVENT);  // C 6276
	      inev.ie.modifiers |= down_modifier as c_uint;             // C 6277
	    }
	  GDK_SCROLL_LEFT =>
	    {
	      input_event_bits! (kind, &raw mut inev.ie, HORIZ_WHEEL_EVENT);  // C 6280
	      inev.ie.modifiers |= up_modifier as c_uint;               // C 6281
	    }
	  GDK_SCROLL_RIGHT =>
	    {
	      input_event_bits! (kind, &raw mut inev.ie, HORIZ_WHEEL_EVENT);  // C 6284
	      inev.ie.modifiers |= down_modifier as c_uint;             // C 6285
	    }
	  GDK_SCROLL_SMOOTH => { }	    /* shut up warning */      // C 6287-6288
	  _ => { }
	}
    }
  else if gdk_event_get_scroll_deltas (event, &raw mut delta_x,
				       &raw mut delta_y) != 0           // C 6291
    {
      if !mwheel_coalesce_scroll_events                                 // C 6293
	{
	  input_event_bits! (kind, &raw mut inev.ie,
			     if fabs (delta_x) > fabs (delta_y)         // C 6295
			       { HORIZ_WHEEL_EVENT }
			       else { WHEEL_EVENT });                   // C 6296-6297
	  inev.ie.modifiers |=
	    (if input_event_bits! (kind, &raw mut inev.ie) == HORIZ_WHEEL_EVENT
	       { if delta_x >= 0.0 { up_modifier } else { down_modifier } }
	       else { if delta_y >= 0.0 { down_modifier } else { up_modifier } })
	    as c_uint;                                                  // C 6298-6300
	  inev.ie.arg = list3 (Qnil (), make_float (-delta_x * 100.0),
			       make_float (-delta_y * 100.0));          // C 6301-6302
	}
      else
	{
	  (*dpyinfo).scroll.acc_x += delta_x;                           // C 6306
	  (*dpyinfo).scroll.acc_y += delta_y;                           // C 6307
	  if (*dpyinfo).scroll.acc_y >= (*dpyinfo).scroll.y_per_line    // C 6308
	    {
	      let mut nlines: c_int = ((*dpyinfo).scroll.acc_y
				       / (*dpyinfo).scroll.y_per_line) as c_int;  // C 6310
	      input_event_bits! (kind, &raw mut inev.ie, WHEEL_EVENT);  // C 6311
	      inev.ie.modifiers |= down_modifier as c_uint;             // C 6312
	      inev.ie.arg = list3 (make_fixnum (nlines as EMACS_INT),
				   make_float (-(*dpyinfo).scroll.acc_x * 100.0),
				   make_float (-(*dpyinfo).scroll.acc_y * 100.0));  // C 6313-6315
	      (*dpyinfo).scroll.acc_y
		-= (*dpyinfo).scroll.y_per_line * nlines as c_double;   // C 6316
	    }
	  else if (*dpyinfo).scroll.acc_y <= -(*dpyinfo).scroll.y_per_line  // C 6318
	    {
	      let mut nlines: c_int = ((-(*dpyinfo).scroll.acc_y)
				       / (*dpyinfo).scroll.y_per_line) as c_int;  // C 6320
	      input_event_bits! (kind, &raw mut inev.ie, WHEEL_EVENT);  // C 6321
	      inev.ie.modifiers |= up_modifier as c_uint;               // C 6322
	      inev.ie.arg = list3 (make_fixnum (nlines as EMACS_INT),
				   make_float (-(*dpyinfo).scroll.acc_x * 100.0),
				   make_float (-(*dpyinfo).scroll.acc_y * 100.0));  // C 6323-6325

	      (*dpyinfo).scroll.acc_y
		-= -(*dpyinfo).scroll.y_per_line * nlines as c_double;  // C 6327
	    }
	  else if (*dpyinfo).scroll.acc_x >= (*dpyinfo).scroll.x_per_char
		   || !mwheel_coalesce_scroll_events                       // C 6329-6330
	    {
	      let mut nchars: c_int = ((*dpyinfo).scroll.acc_x
				       / (*dpyinfo).scroll.x_per_char) as c_int;  // C 6332
	      input_event_bits! (kind, &raw mut inev.ie, HORIZ_WHEEL_EVENT);  // C 6333
	      inev.ie.modifiers |= up_modifier as c_uint;               // C 6334
	      inev.ie.arg = list3 (make_fixnum (nchars as EMACS_INT),
				   make_float (-(*dpyinfo).scroll.acc_x * 100.0),
				   make_float (-(*dpyinfo).scroll.acc_y * 100.0));  // C 6335-6337

	      (*dpyinfo).scroll.acc_x
		-= (*dpyinfo).scroll.x_per_char * nchars as c_double;   // C 6339
	    }
	  else if (*dpyinfo).scroll.acc_x <= -(*dpyinfo).scroll.x_per_char  // C 6341
	    {
	      let mut nchars: c_int = ((-(*dpyinfo).scroll.acc_x)
				       / (*dpyinfo).scroll.x_per_char) as c_int;  // C 6343
	      input_event_bits! (kind, &raw mut inev.ie, HORIZ_WHEEL_EVENT);  // C 6344
	      inev.ie.modifiers |= down_modifier as c_uint;             // C 6345
	      inev.ie.arg = list3 (make_fixnum (nchars as EMACS_INT),
				   make_float (-(*dpyinfo).scroll.acc_x * 100.0),
				   make_float (-(*dpyinfo).scroll.acc_y * 100.0));  // C 6346-6348

	      (*dpyinfo).scroll.acc_x
		-= -(*dpyinfo).scroll.x_per_char * nchars as c_double;  // C 6350
	    }
	}
    }

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT              // C 6355
    {
      inev.ie.device
	= pgtk_get_device_for_event (FRAME_DISPLAY_INFO! (f), event);   // C 6357-6358
      evq_enqueue (&raw mut inev);                                      // C 6359
    }
  return true as gboolean;                                              // C 6361
}

/* C: glib/gmem.h 135-145's `g_clear_pointer` macro (the `__GNUC__` arm this machine takes) - the
   C 6443 call, which is this file's only one; `src/xwidget.rs` 2332-2341 carries the same macro for
   its own two calls, and a `macro_rules!` is textually scoped, so the unit keeps its own copy (the
   `mouse_face_hidden!`/`input_event_bits!` shape).  */
macro_rules! g_clear_pointer {
  ($pp:expr, $destroy:expr) => {
    {
      let pp = ($pp);
      let ptr: *mut c_void = *pp as *mut c_void;
      *pp = std::ptr::null_mut ();
      if ! ptr.is_null () { $destroy (ptr as gpointer); }
    }
  };
}

/// C 6369-6388: `static GdkDragAction symbol_to_drag_action (Lisp_Object act)` - `static` and never
/// used as a value (C 6424 is its only call), so a plain `unsafe fn`.  `Qcopy`/`Qmove`/`Qlink`/
/// `Qprivate` are `include/globals.rs`'s symbol functions (the unit reads them bare, 1789's shape),
/// and C 6387's `signal_error` is `include/lisp.rs` 4161's row (which the include graph brings into
/// this unit, so the port declares no second one - the fifty-first slice's `list3` lesson).
#[cfg (feature = "have_pgtk")]
unsafe fn symbol_to_drag_action (act: Lisp_Object) -> GdkDragAction
{
  if EQ (act, Qcopy ())                                                 // C 6372
    { return GDK_ACTION_COPY; }                                         // C 6373

  if EQ (act, Qmove ())                                                 // C 6375
    { return GDK_ACTION_MOVE; }                                         // C 6376

  if EQ (act, Qlink ())                                                 // C 6378
    { return GDK_ACTION_LINK; }                                         // C 6379

  if EQ (act, Qprivate ())                                              // C 6381
    { return GDK_ACTION_PRIVATE; }                                      // C 6382

  if NILP (act)                                                         // C 6384
    { return GDK_ACTION_DEFAULT; }                                      // C 6385

  signal_error (c"Invalid drag action".as_ptr (), act);                 // C 6387
}

/// C 6390-6411: `static Lisp_Object drag_action_to_symbol (GdkDragAction action)` - `static` with no
/// address taken, so a plain `unsafe fn`.  C's `switch` is the port's `match`; its last two labels
/// (`case GDK_ACTION_DEFAULT: default:`) are two arms, as C writes them.
#[cfg (feature = "have_pgtk")]
unsafe fn drag_action_to_symbol (action: GdkDragAction) -> Lisp_Object
{
  match action
    {
      GDK_ACTION_COPY =>
	{ return Qcopy (); }                                            // C 6396-6397

      GDK_ACTION_MOVE =>
	{ return Qmove (); }                                            // C 6399-6400

      GDK_ACTION_LINK =>
	{ return Qlink (); }                                            // C 6402-6403

      GDK_ACTION_PRIVATE =>
	{ return Qprivate (); }                                         // C 6405-6406

      GDK_ACTION_DEFAULT =>
	{ return Qnil (); }                                             // C 6408-6409
      _ =>
	{ return Qnil (); }                                             // C 6408-6409
    }
}

/// C 6413-6426: `void pgtk_update_drop_status (Lisp_Object action, Lisp_Object event_time)` - a
/// **non-`static`** C definition, so the port carries `#[unsafe(no_mangle)]` and `extern "C"`
/// (PORTING.md 0.2).  C's `CONS_TO_INTEGER (event_time, guint32, time)` is the mirror's two-argument
/// `CONS_TO_INTEGER!` (`include/lisp.rs` 3692 - the type is the local's), and C's
/// `!current_drop_context` is the port's `is_null ()` test.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_update_drop_status (action: Lisp_Object, event_time: Lisp_Object)
{
  /* C 6418's `CONS_TO_INTEGER (event_time, guint32, time);` - the port's trait picks the helper
     from the variable's type and has the two 64-bit impls, so C's `guint32` conversion is the
     explicit cast (the `src/inotify.rs` 448-455 shape).  */
  let mut time_wide: i64;
  CONS_TO_INTEGER! (event_time, time_wide);
  let time: guint32 = time_wide as guint32;

  if current_drop_context.is_null () || time < current_drop_time        // C 6420
    { return; }                                                         // C 6421

  gdk_drag_status (current_drop_context,                               // C 6423
		   symbol_to_drag_action (action),                      // C 6424
		   time);                                               // C 6425
}

/// C 6428-6445: `void pgtk_finish_drop (Lisp_Object success, Lisp_Object event_time, Lisp_Object
/// del)` - a **non-`static`** C definition, so the port carries `#[unsafe(no_mangle)]` and
/// `extern "C"` (PORTING.md 0.2); the `guint32` local is zero-initialised (C's uninitialised-local
/// rule, Deviation 12) because `CONS_TO_INTEGER!` writes it through the macro, and the `g_clear_pointer`
/// macro above is the C 6443 call.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_finish_drop (success: Lisp_Object, event_time: Lisp_Object,
					   del: Lisp_Object)
{
  /* C 6434's `CONS_TO_INTEGER (event_time, guint32, time);` - the same cast as above.  */
  let mut time_wide: i64;
  CONS_TO_INTEGER! (event_time, time_wide);
  let time: guint32 = time_wide as guint32;

  if current_drop_context.is_null () || time < current_drop_time        // C 6436
    { return; }                                                         // C 6437

  gtk_drag_finish (current_drop_context, !NILP (success) as gboolean,   // C 6439
		   !NILP (del) as gboolean, time);                      // C 6440

  if current_drop_context_drop                                          // C 6442
    { g_clear_pointer! (&raw mut current_drop_context,                  // C 6443
			g_object_unref); }                              // C 6444
}

/* C: gdk/gdkatom.h: `gchar *gdk_atom_name (GdkAtom atom);` - the C 6516/6565 calls.  `gdk_atom_name`
   is not in the mirror: `src/pgtkselect.rs` 210-212 declares its own row for its one call (the
   C 141 shape), so this unit declares its own too (P-88 1) rather than making the mirror carry a
   name an includer already has (the `GDK_BUTTON_RELEASE` rule).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gdk_atom_name (atom: GdkAtom) -> *mut gchar;
}

/// C 6447-6478: `static void drag_leave (GtkWidget *widget, GdkDragContext *context, guint time,
/// gpointer user_data)` - `static` with its address taken at C 6902 (`G_CALLBACK (drag_leave)`), so the port defines it `unsafe extern "C" fn` here
/// (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`).  `DRAG_N_DROP_EVENT` is `include/termhooks.rs`'s
/// row, C's `FALSE` is `false as gboolean`, the `guint` -> `Time` widening of the timestamp store is
/// written out, and C 6462's `g_clear_pointer` is this unit's own macro.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn drag_leave (widget: *mut GtkWidget, context: *mut GdkDragContext, time: guint,
		      user_data: gpointer)
{
  let mut f: *mut frame;                                                // C 6451
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6452

  f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 6454

  if !current_drop_context.is_null ()                                   // C 6456
    {
      if current_drop_context_drop                                      // C 6458
	{ gtk_drag_finish (current_drop_context,                         // C 6459
			   false as gboolean, false as gboolean,         // C 6460
			   current_drop_time); }                         // C 6460

      g_clear_pointer! (&raw mut current_drop_context,                   // C 6462
			g_object_unref);                                 // C 6463
    }

  EVENT_INIT (&raw mut inev.ie);                                        // C 6466

  input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);        // C 6468
  inev.ie.modifiers = 0;                                                // C 6469
  inev.ie.arg = Qnil ();                                                // C 6470
  inev.ie.timestamp = time as Time;                                     // C 6471

  XSETINT! (inev.ie.x, 0);                                              // C 6473
  XSETINT! (inev.ie.y, 0);                                              // C 6474
  XSETFRAME! (inev.ie.frame_or_window, f);                              // C 6475

  evq_enqueue (&raw mut inev);                                          // C 6477
}

/// C 6480-6528: `static gboolean drag_motion (GtkWidget *widget, GdkDragContext *context, gint x,
/// gint y, guint time)` - `static` with its address taken at C 6904 (`G_CALLBACK (drag_motion)`), so it is
/// defined `unsafe extern "C" fn` here (PORTING.md 1.4; C 6831 takes its address through
/// `G_CALLBACK`).  C's `gpointer` -> `GdkDragContext *` round trip through `g_object_ref` is
/// written out on both sides (`as gpointer`, `as *mut GdkDragContext`), `make_uint` takes this
/// mirror's `uintmax_t` so the `guint` argument is converted, C's `FALSE` is `false as gboolean`, and
/// `intern`/`list4`/`Qlambda` are the include graph's own rows (`intern` is `src/lisp.rs`'s function
/// at `include/lisp.rs` 2728).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn drag_motion (widget: *mut GtkWidget, context: *mut GdkDragContext, x: c_int, y: c_int,
		       time: guint) -> gboolean
{
  let mut f: *mut frame;                                                // C 6485
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6486
  let mut name: GdkAtom;                                                // C 6487
  let mut suggestion: GdkDragAction;                                    // C 6488

  f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 6490

  if f.is_null ()                                                       // C 6492
    { return false as gboolean; }                                       // C 6493

  if !current_drop_context.is_null ()                                   // C 6495
    {
      if current_drop_context_drop                                      // C 6497
	{ gtk_drag_finish (current_drop_context,                         // C 6498
			   false as gboolean, false as gboolean,         // C 6499
			   current_drop_time); }                         // C 6499

      g_clear_pointer! (&raw mut current_drop_context,                   // C 6501
			g_object_unref);                                 // C 6502
    }

  current_drop_context = g_object_ref (context as gpointer) as *mut GdkDragContext;  // C 6505
  current_drop_time = time;                                             // C 6506
  current_drop_context_drop = false;                                    // C 6507

  name = gdk_drag_get_selection (context);                              // C 6509
  suggestion = gdk_drag_context_get_suggested_action (context);         // C 6510

  EVENT_INIT (&raw mut inev.ie);                                        // C 6512

  input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);        // C 6514
  inev.ie.modifiers = 0;                                                // C 6515
  inev.ie.arg = list4 (Qlambda (), intern (gdk_atom_name (name)),       // C 6516
		       make_uint (time as uintmax_t),                   // C 6517
		       drag_action_to_symbol (suggestion));             // C 6518
  inev.ie.timestamp = time as Time;                                     // C 6519

  XSETINT! (inev.ie.x, x as EMACS_INT);                                 // C 6521
  XSETINT! (inev.ie.y, y as EMACS_INT);                                 // C 6522
  XSETFRAME! (inev.ie.frame_or_window, f);                              // C 6523

  evq_enqueue (&raw mut inev);                                          // C 6525

  return true as gboolean;                                              // C 6527
}

/// C 6530-6577: `static gboolean drag_drop (GtkWidget *widget, GdkDragContext *context, int x, int
/// y, guint time, gpointer user_data)` - `static` with its address taken at C 6906 (`G_CALLBACK
/// (drag_drop)`), so the port defines it `unsafe extern "C" fn` (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`).  The body is `drag_motion`'s with `Qquote`, the selected
/// action and `current_drop_context_drop = true`.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn drag_drop (widget: *mut GtkWidget, context: *mut GdkDragContext, x: c_int, y: c_int,
		    time: guint, user_data: gpointer) -> gboolean
{
  let mut f: *mut frame;                                                // C 6534
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6535
  let mut name: GdkAtom;                                                // C 6536
  let mut selected_action: GdkDragAction;                               // C 6537

  f = pgtk_any_window_to_frame (gtk_widget_get_window (widget));        // C 6539

  if f.is_null ()                                                       // C 6541
    { return false as gboolean; }                                       // C 6542

  if !current_drop_context.is_null ()                                   // C 6544
    {
      if current_drop_context_drop                                      // C 6546
	{ gtk_drag_finish (current_drop_context,                         // C 6547
			   false as gboolean, false as gboolean,         // C 6548
			   current_drop_time); }                         // C 6548

      g_clear_pointer! (&raw mut current_drop_context,                   // C 6550
			g_object_unref);                                 // C 6551
    }

  current_drop_context = g_object_ref (context as gpointer) as *mut GdkDragContext;  // C 6554
  current_drop_time = time;                                             // C 6555
  current_drop_context_drop = true;                                     // C 6556

  name = gdk_drag_get_selection (context);                              // C 6558
  selected_action = gdk_drag_context_get_selected_action (context);     // C 6559

  EVENT_INIT (&raw mut inev.ie);                                        // C 6561

  input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);        // C 6563
  inev.ie.modifiers = 0;                                                // C 6564
  inev.ie.arg = list4 (Qquote (), intern (gdk_atom_name (name)),        // C 6565
		       make_uint (time as uintmax_t),                   // C 6566
		       drag_action_to_symbol (selected_action));        // C 6567
  inev.ie.timestamp = time as Time;                                     // C 6568

  XSETINT! (inev.ie.x, x as EMACS_INT);                                 // C 6570
  XSETINT! (inev.ie.y, y as EMACS_INT);                                 // C 6571
  XSETFRAME! (inev.ie.frame_or_window, f);                              // C 6572

  evq_enqueue (&raw mut inev);                                          // C 6574

  return true as gboolean;                                              // C 6576
}

/* Touch screen events.  */

/* Record a touch sequence with the identifier DETAIL from the given
   FRAME on the specified DPYINFO.  Round X and Y and record them as its
   current position, assign an identifier to the touch sequence suitable
   for reporting to Lisp, and return the same.  */

/// C 6588-6613: `static EMACS_INT pgtk_link_touch_point (struct pgtk_display_info *dpyinfo,
/// GdkEventSequence *detail, gdouble x, gdouble y, struct frame *frame)` - `static` with no address
/// taken, so a plain `unsafe fn`, and its sibling in another unit is `src/xterm.rs` 6047-6080's
/// `xi_link_touch_point`.  C 6594's `static EMACS_INT local_detail;` is a **function static**, which
/// the port writes where C declares it (the same file's shape); C 6609's post-increment is written
/// out as the store plus the increment; `xmalloc`'s C `sizeof *touchpoint` is
/// `std::mem::size_of::<pgtk_touch_point> ()` and its `void *` is converted; and the two `lrint`
/// results are `long` -> `int` conversions written out.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_link_touch_point (dpyinfo: *mut pgtk_display_info,
				 detail: *mut GdkEventSequence, x: gdouble, y: gdouble,
				 frame: *mut frame) -> EMACS_INT
{
  let mut touchpoint: *mut pgtk_touch_point;                            // C 6593
  static mut local_detail: EMACS_INT = 0;                               // C 6594

  /* Assign an identifier suitable for reporting to Lisp.  On builds
     with 64-bit Lisp_Object, this is largely a theoretical problem, but
     CARD32s easily overflow 32-bit systems, as they are not specific to
     X clients (e.g. Emacs) but grow uniformly across all of them.  */

  if FIXNUM_OVERFLOW_P (local_detail)                                   // C 6601
    { local_detail = 0; }                                               // C 6602

  touchpoint = xmalloc (std::mem::size_of::<pgtk_touch_point> ())
	       as *mut pgtk_touch_point;                                // C 6604
  (*touchpoint).next = (*dpyinfo).touchpoints;                          // C 6605
  (*touchpoint).x = lrint (x) as c_int;                                 // C 6606
  (*touchpoint).y = lrint (y) as c_int;                                 // C 6607
  (*touchpoint).number = detail;                                        // C 6608
  (*touchpoint).local_detail = local_detail;                            // C 6609
  local_detail += 1;                                                    // C 6609
  (*touchpoint).frame = frame;                                          // C 6610
  (*dpyinfo).touchpoints = touchpoint;                                  // C 6611
  return (*touchpoint).local_detail;                                    // C 6612
}

/* Free and remove the touch sequence with the identifier DETAIL.
   DPYINFO is the display in which the touch sequence should be
   recorded.  If such a touch sequence exists, return its local
   identifier in *LOCAL_DETAIL.

   Value is 0 if no touch sequence by that identifier exists inside
   DPYINFO, or 1 if a touch sequence has been found.  */

/// C 6623-6648: `static int pgtk_unlink_touch_point (GdkEventSequence *detail, struct
/// pgtk_display_info *dpyinfo, EMACS_INT *local_detail)` - `static` with no address taken, so a plain
/// `unsafe fn`.  C's `for (last = NULL, tem = …; tem; last = tem, tem = tem->next)` is the port's
/// init + `while` with C's two step statements at the body's end (the `pgtk_display_info_for_display`
/// shape), C's `!last` is `last.is_null ()` and `xfree`'s `void *` is converted.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_unlink_touch_point (detail: *mut GdkEventSequence,
				   dpyinfo: *mut pgtk_display_info,
				   local_detail: *mut EMACS_INT) -> c_int
{
  let mut last: *mut pgtk_touch_point;                                  // C 6628
  let mut tem: *mut pgtk_touch_point;                                   // C 6628

  last = std::ptr::null_mut ();                                         // C 6630
  tem = (*dpyinfo).touchpoints;                                         // C 6630
  while !tem.is_null ()                                                 // C 6630
    {
      if (*tem).number == detail                                        // C 6633
	{
	  if last.is_null ()                                            // C 6635
	    { (*dpyinfo).touchpoints = (*tem).next; }                     // C 6636
	  else
	    { (*last).next = (*tem).next; }                               // C 6638

	  *local_detail = (*tem).local_detail;                            // C 6640
	  xfree (tem as *mut c_void);                                     // C 6641

	  return 1;                                                       // C 6643
	}
      last = tem;                                                         // C 6630
      tem = (*tem).next;                                                  // C 6630
    }

  return 0;                                                               // C 6647
}

/* Unlink all touch points associated with the frame F.  This is done
   upon destroying F's window (or its being destroyed), because touch
   point delivery after that point is undefined.  */

/// C 6654-6673: `static void pgtk_unlink_touch_points (struct frame *f)` - `static` with no address
/// taken, so a plain `unsafe fn`.  C's pointer-to-pointer walk is the port's `next` of type
/// `*mut *mut pgtk_touch_point`; C 6663's assignment **inside the condition**
/// (`for (next = &dpyinfo->touchpoints; (last = *next);)`) has no Rust expression form, so the port
/// keeps C's order in the loop head (assign, then test - the `motion_notify_event` guard's shape).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_unlink_touch_points (f: *mut frame)
{
  let mut next: *mut *mut pgtk_touch_point;                             // C 6657
  let mut last: *mut pgtk_touch_point;                                  // C 6657
  let mut dpyinfo: *mut pgtk_display_info;                              // C 6658

  /* Now unlink all touch points on F's display matching F.  */

  dpyinfo = FRAME_DISPLAY_INFO! (f);                                    // C 6662
  next = &raw mut (*dpyinfo).touchpoints;                               // C 6663
  loop
    {
      last = *next;                                                     // C 6663
      if last.is_null ()                                                // C 6663
	{ break; }

      if (*last).frame == f                                             // C 6665
	{
	  *next = (*last).next;                                         // C 6667
	  xfree (last as *mut c_void);                                  // C 6668
	}
      else
	{ next = &raw mut (*last).next; }                               // C 6671
    }
}

/* Return the data associated with a touch sequence DETAIL recorded by
   `pgtk_link_touch_point' from DPYINFO, or NULL if it can't be
   found.  */

/// C 6679-6692: `static struct pgtk_touch_point *pgtk_find_touch_point (struct pgtk_display_info
/// *dpyinfo, GdkEventSequence *detail)` - `static` with no address taken, so a plain `unsafe fn`;
/// C's `for` is the port's init + `while` with C's step at the body's end and its `NULL` is
/// `std::ptr::null_mut ()`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_find_touch_point (dpyinfo: *mut pgtk_display_info, detail: *mut GdkEventSequence)
    -> *mut pgtk_touch_point
{
  let mut point: *mut pgtk_touch_point;                                 // C 6683

  point = (*dpyinfo).touchpoints;                                       // C 6685
  while !point.is_null ()                                               // C 6685
    {
      if (*point).number == detail                                      // C 6687
	{ return point; }                                               // C 6688
      point = (*point).next;                                            // C 6685
    }

  return std::ptr::null_mut ();                                         // C 6691
}

/// C 6694-6806: `static gboolean touch_event_cb (GtkWidget *self, GdkEvent *event, gpointer
/// user_data)` - `static` with its address taken at C 6898 (`G_CALLBACK (touch_event_cb)`), so the port defines it `unsafe extern "C" fn` here
/// (PORTING.md 1.4; C 6831 takes its address through `G_CALLBACK`).  C's `switch (event->type)` is the port's
/// `match` with C's four `GDK_TOUCH_*` labels (the last two share one arm, as C writes them) and a
/// trailing `_ => { }`; the two **early `break`s** inside case bodies are the port's `'arm:` block
/// plus `break 'arm` (Deviation 54's shape, the `goto` reading).  `eassert` is the mirror's
/// `eassert!`, C's `bool` -> `unsigned`/`gboolean` conversions (`modifiers`, the return value) are
/// written out, the `lrint` `long` results are `int`/`EMACS_INT` conversions written out, and
/// `Fcons`/`list3i`/`TOUCHSCREEN_*_EVENT` are the include graph's own rows.
/// C 6699's uninitialised `local_detail` is zero-initialised (Deviation 12's rule), because the
/// C 6766 call hands it out through `&raw mut` and the first reading named the `E0381` that skip
/// produces.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn touch_event_cb (self_: *mut GtkWidget, event: *mut GdkEvent, user_data: gpointer)
    -> gboolean
{
  let mut dpyinfo: *mut pgtk_display_info;                              // C 6697
  let mut f: *mut frame;                                                // C 6698
  let mut local_detail: EMACS_INT = 0;                                 // C 6699
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6700
  let mut touchpoint: *mut pgtk_touch_point;                            // C 6701
  let mut arg: Lisp_Object = Qnil ();                                   // C 6702
  let mut state: c_int;                                                 // C 6703

  EVENT_INIT (&raw mut inev.ie);                                        // C 6705

  f = pgtk_any_window_to_frame (gtk_widget_get_window (self_));         // C 6707
  eassert! (f);                                                         // C 6708
  dpyinfo = FRAME_DISPLAY_INFO! (f);                                    // C 6709
  match (*event).r#type
    {
      GDK_TOUCH_BEGIN =>
	'arm: {
	  /* Verify that no touch point with this identifier is already at
	     large.  */
	  if !pgtk_find_touch_point (dpyinfo, (*event).touch.sequence)
	      .is_null ()                                                   // C 6716
	    { break 'arm; }                                                 // C 6717

	  /* Record this in the display structure.  */
	  local_detail = pgtk_link_touch_point (dpyinfo, (*event).touch.sequence,
						(*event).touch.x, (*event).touch.y,
						f);                            // C 6720-6722
	  /* Generate the input event.  */
	  input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_BEGIN_EVENT);  // C 6724
	  inev.ie.timestamp = (*event).touch.time as Time;                   // C 6725
	  XSETFRAME! (inev.ie.frame_or_window, f);                          // C 6726
	  XSETINT! (inev.ie.x, lrint ((*event).touch.x) as EMACS_INT);       // C 6727
	  XSETINT! (inev.ie.y, lrint ((*event).touch.y) as EMACS_INT);       // C 6728
	  XSETINT! (inev.ie.arg, local_detail);                             // C 6729
	}                                                                   // C 6730

      GDK_TOUCH_UPDATE =>
	'arm: {
	  touchpoint = pgtk_find_touch_point (dpyinfo, (*event).touch.sequence);  // C 6733-6734

	  if touchpoint.is_null ()                                          // C 6736
	      /* Don't send this event if nothing has changed
		 either.  */
	      || ((*touchpoint).x == lrint ((*event).touch.x) as c_int     // C 6739
		  && (*touchpoint).y == lrint ((*event).touch.y) as c_int)  // C 6740
	    { break 'arm; }                                                 // C 6741

	  /* Construct the input event.  */
	  (*touchpoint).x = lrint ((*event).touch.x) as c_int;              // C 6744
	  (*touchpoint).y = lrint ((*event).touch.y) as c_int;              // C 6745
	  input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_UPDATE_EVENT);  // C 6746
	  inev.ie.timestamp = (*event).touch.time as Time;                  // C 6747
	  XSETFRAME! (inev.ie.frame_or_window, f);                          // C 6748

	  touchpoint = (*dpyinfo).touchpoints;                              // C 6750
	  while !touchpoint.is_null ()                                      // C 6751
	    {
	      if (*touchpoint).frame == f                                   // C 6753
		{ arg = Fcons (list3i ((*touchpoint).x as EMACS_INT,
				       (*touchpoint).y as EMACS_INT,
				       (*touchpoint).local_detail),           // C 6754-6755
			       arg); }                                      // C 6756
	      touchpoint = (*touchpoint).next;                              // C 6751
	    }

	  inev.ie.arg = arg;                                                // C 6759
	}                                                                   // C 6760

      GDK_TOUCH_END | GDK_TOUCH_CANCEL =>                                   // C 6762-6763
	{
	  /* Remove this touch point's record, also establishing its
	     existence.  */
	  state = pgtk_unlink_touch_point ((*event).touch.sequence,         // C 6766
					   dpyinfo, &raw mut local_detail);  // C 6767
	  /* If it did exist... */
	  if state != 0                                                     // C 6769
	    {
	      /* ... generate a suitable event.  */
	      input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_END_EVENT);  // C 6772
	      inev.ie.timestamp = (*event).touch.time as Time;              // C 6773
	      inev.ie.modifiers = ((*event).r#type != GDK_TOUCH_END) as c_uint;  // C 6774

	      XSETFRAME! (inev.ie.frame_or_window, f);                      // C 6776
	      XSETINT! (inev.ie.x, lrint ((*event).touch.x) as EMACS_INT);   // C 6777
	      XSETINT! (inev.ie.y, lrint ((*event).touch.y) as EMACS_INT);   // C 6778
	      XSETINT! (inev.ie.arg, local_detail);                         // C 6779
	    }
	}
      _ => { }                                                              // C 6783-6784
    }

  /* If the above produced a workable event, report the name of the
     device that gave rise to it.  */

  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT             // C 6790
    {
      inev.ie.device = pgtk_get_device_for_event (dpyinfo, event);      // C 6792
      evq_enqueue (&raw mut inev);                                      // C 6793

      /* Next, save this event for future menu activations, unless it is
	 only an update.  */
      if (*event).r#type != GDK_TOUCH_UPDATE                            // C 6797
	{
	  if !(*dpyinfo).last_click_event.is_null ()                    // C 6799
	    { gdk_event_free ((*dpyinfo).last_click_event); }           // C 6800
	  (*dpyinfo).last_click_event = gdk_event_copy (event);         // C 6801
	}
    }

  return (input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT) as gboolean;  // C 6805
}

/* Callbacks for sundries.  */

/// C 6812-6824: `static void pgtk_monitors_changed_cb (GdkScreen *screen, gpointer user_data)` -
/// `static` with its address taken at C 1218 (`g_signal_connect (…, "monitors-changed", …)`, in the
/// slice that landed long before this one), so a plain `unsafe fn` until that call site's own
/// conversion round.  C's `gpointer` -> `struct terminal *` conversion is written out,
/// `MONITORS_CHANGED_EVENT` is `include/termhooks.rs` 62's row and `XSETTERMINAL` is
/// `include/lisp.rs` 3137's macro (which `src/term.rs` 3037 reads the same way).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_monitors_changed_cb (screen: *mut GdkScreen, user_data: gpointer)
{
  let mut terminal: *mut terminal;                                      // C 6815
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6816

  EVENT_INIT (&raw mut inev.ie);                                        // C 6818
  terminal = user_data as *mut terminal;                                // C 6819
  input_event_bits! (kind, &raw mut inev.ie, MONITORS_CHANGED_EVENT);   // C 6820
  XSETTERMINAL! (inev.ie.arg, terminal);                                // C 6821

  evq_enqueue (&raw mut inev);                                          // C 6823
}

/* C 6826: `static gboolean pgtk_selection_event (GtkWidget *, GdkEvent *, gpointer);` - a forward
   declaration of this file's own `static` (its definition is in the remainder, C 7477), which Rust
   does not need; the name is declared where C's later call sites use it, so C's line stands here as
   a comment (the C 442-443 `pgtk_unlink_touch_points` reading at this unit's line 790).  */

/// C 6830-6903: `void pgtk_set_event_handler (struct frame *f)` - a **non-`static`** C definition, so
/// the port carries `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2).  **This is the slice
/// that lands C's `G_CALLBACK` round**: every function whose address C takes below is defined
/// `unsafe extern "C" fn` in this unit (the nineteen of this file's own hooks plus the two remainder
/// names), which is why the readings of this slice cover those twenty signatures as well.  The
/// signal names are the port's `c"…".as_ptr ()` (the `pgtk_seat_added_cb` shape at line 436), C's
/// `NULL` is `std::ptr::null_mut ()`, C 6860's data argument is the `f as gpointer` conversion, and
/// C 6840's `0`/`NULL`/`0` are the same.  C 6833's `f->tooltip` is the mirror's bitfield accessor
/// `FRAME_TOOLTIP_P` (`include/frame.rs` 1285-1287, `frame.h` 457's `bool_bf tooltip : 1`), and the
/// `G_CALLBACK`-taking macros apply their own cast, so the handler arguments below are the bare
/// names (`g_signal_connect!`'s body), while `g_signal_handlers_disconnect_by_func!`'s body does
/// not, so those three call sites still pass `G_CALLBACK! (…)` as C does.  `pgtk_selection_event` (C 7477, the remainder) has no
/// definition here yet, so its four `G_CALLBACK`s stand as the registered not-yet-ported rows this
/// slice's readings carry (the `pgtk_unlink_touch_points` class).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_event_handler (f: *mut frame)
{
  if FRAME_TOOLTIP_P (f)                                               // C 6833
    {
      g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"draw".as_ptr (),
			 pgtk_handle_draw, std::ptr::null_mut ());  // C 6835-6836
      return;                                                           // C 6837
    }

  gtk_drag_dest_set (FRAME_GTK_WIDGET! (f), 0, std::ptr::null_mut (), 0,
		     GDK_ACTION_MOVE | GDK_ACTION_COPY
		     | GDK_ACTION_LINK | GDK_ACTION_PRIVATE);           // C 6840-6842

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()                            // C 6844
    {
      g_signal_connect! (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)),
			 c"window-state-event".as_ptr (), window_state_event,
			 std::ptr::null_mut ());                        // C 6846-6848
      g_signal_connect! (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)), c"delete-event".as_ptr (),
			 delete_event, std::ptr::null_mut ());  // C 6849-6850
      g_signal_connect! (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)), c"event".as_ptr (),
			 pgtk_handle_event, std::ptr::null_mut ());  // C 6851-6852
      g_signal_connect! (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)), c"configure-event".as_ptr (),
			 configure_event, std::ptr::null_mut ());  // C 6853-6854
    }

  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"map-event".as_ptr (),
		     map_event, std::ptr::null_mut ());   // C 6857-6858
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"size-allocate".as_ptr (),
		     size_allocate, f as gpointer);       // C 6859-6860
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"key-press-event".as_ptr (),
		     key_press_event, std::ptr::null_mut ());  // C 6861-6862
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"key-release-event".as_ptr (),
		     key_release_event, std::ptr::null_mut ());  // C 6863-6864
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"focus-in-event".as_ptr (),
		     focus_in_event, std::ptr::null_mut ());  // C 6865-6866
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"focus-out-event".as_ptr (),
		     focus_out_event, std::ptr::null_mut ());  // C 6867-6868
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"enter-notify-event".as_ptr (),
		     enter_notify_event, std::ptr::null_mut ());  // C 6869-6870
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"leave-notify-event".as_ptr (),
		     leave_notify_event, std::ptr::null_mut ());  // C 6871-6872
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"motion-notify-event".as_ptr (),
		     motion_notify_event, std::ptr::null_mut ());  // C 6873-6874
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"button-press-event".as_ptr (),
		     button_event, std::ptr::null_mut ());  // C 6875-6876
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"button-release-event".as_ptr (),
		     button_event, std::ptr::null_mut ());  // C 6877-6878
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"scroll-event".as_ptr (),
		     scroll_event, std::ptr::null_mut ());  // C 6879-6880
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"configure-event".as_ptr (),
		     configure_event, std::ptr::null_mut ());  // C 6881-6882
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"drag-leave".as_ptr (),
		     drag_leave, std::ptr::null_mut ());    // C 6883-6884
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"drag-motion".as_ptr (),
		     drag_motion, std::ptr::null_mut ());   // C 6885-6886
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"drag-drop".as_ptr (),
		     drag_drop, std::ptr::null_mut ());     // C 6887-6888
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"draw".as_ptr (),
		     pgtk_handle_draw, std::ptr::null_mut ());  // C 6889-6890
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"property-notify-event".as_ptr (),
		     pgtk_selection_event, std::ptr::null_mut ());  // C 6891-6892
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"selection-clear-event".as_ptr (),
		     pgtk_selection_event, std::ptr::null_mut ());  // C 6893-6894
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"selection-request-event".as_ptr (),
		     pgtk_selection_event, std::ptr::null_mut ());  // C 6895-6896
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"selection-notify-event".as_ptr (),
		     pgtk_selection_event, std::ptr::null_mut ());  // C 6897-6898
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"touch-event".as_ptr (),
		     touch_event_cb, std::ptr::null_mut ());  // C 6899-6900
  g_signal_connect! (G_OBJECT! (FRAME_GTK_WIDGET! (f)), c"event".as_ptr (),
		     pgtk_handle_event, std::ptr::null_mut ());  // C 6901-6902
}

/* <string.h>: `char *strstr (const char *haystack, const char *needle);` - the C 6909 call.  The
   include graph carries `strncmp` (`lib/include/string.rs` 32) but no `strstr`, so the prototype
   stands here with its own header citation (P-88 1).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/// C 6905-6911: `static void my_log_handler (const gchar *log_domain, GLogLevelFlags log_level,
/// const gchar *msg, gpointer user_data)` - `static` with its address taken at C 7139
/// (`g_log_set_handler (…, my_log_handler, NULL)`, in the remainder), so the port defines it
/// `unsafe extern "C" fn` (PORTING.md 1.4).  `strstr`/`fprintf`/`stderr` are the include graph's
/// `<string.h>`/`<stdio.h>` rows (the `src/alloc.rs` 5426 reading of the same three names), C's
/// string literals are the port's `c"…".as_ptr ()`, and C 6909's negated pointer test is the port's
/// `is_null ()` (so C's one-statement body keeps its shape).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn my_log_handler (log_domain: *const gchar, log_level: GLogLevelFlags,
				     msg: *const gchar, user_data: gpointer)
{
  if strstr (msg, c"g_set_prgname".as_ptr ()).is_null ()                // C 6909
    { fprintf (stderr, c"%s-WARNING **: %s".as_ptr (), log_domain, msg); }  // C 6910
}

/* Test whether two display-name strings agree up to the dot that separates
   the screen number from the server number.  */

/// C 6913-6958: `static bool same_x_server (const char *name1, const char *name2)` - `static` with
/// no address taken, so a plain `unsafe fn`.  C's two parameters are re-assigned, so the port's are
/// `mut`; `Fsystem_name ()` is `include/globals.rs` 8542's row and `SSDATA`/`SBYTES` the include
/// graph's string readers; C's `name1 += 4`/`name1++` have no Rust spelling for raw pointers, so
/// they are `name1.offset (…)`; C 6948's `for` is the port's `while` with C's two steps at the body's
/// end; the character constants are the port's `b':' as c_char`/`b'.' as c_char` and C's `'\0'` is
/// `0`.
#[cfg (feature = "have_pgtk")]
unsafe fn same_x_server (mut name1: *const c_char, mut name2: *const c_char) -> bool
{
  let mut seen_colon: bool = false;                                     // C 6918
  let sysname: Lisp_Object = Fsystem_name ();                           // C 6919
  let system_name: *const c_char = SSDATA (sysname);                    // C 6920
  let system_name_length: ptrdiff_t = SBYTES (sysname);                 // C 6921
  let mut length_until_period: ptrdiff_t = 0;                           // C 6922

  while *system_name.offset (length_until_period) != 0                  // C 6924
	 && *system_name.offset (length_until_period) != b'.' as c_char  // C 6925
    { length_until_period += 1; }                                       // C 6926

  /* Treat `unix' like an empty host name.  */
  if strncmp (name1, c"unix:".as_ptr (), 5) == 0                        // C 6929
    { name1 = name1.offset (4); }                                       // C 6930
  if strncmp (name2, c"unix:".as_ptr (), 5) == 0                        // C 6931
    { name2 = name2.offset (4); }                                       // C 6932
  /* Treat this host's name like an empty host name.  */
  if strncmp (name1, system_name, system_name_length as usize) == 0     // C 6934
      && *name1.offset (system_name_length) == b':' as c_char           // C 6935
    { name1 = name1.offset (system_name_length); }                      // C 6936
  if strncmp (name2, system_name, system_name_length as usize) == 0     // C 6937
      && *name2.offset (system_name_length) == b':' as c_char           // C 6938
    { name2 = name2.offset (system_name_length); }                      // C 6939
  /* Treat this host's domainless name like an empty host name.  */
  if strncmp (name1, system_name, length_until_period as usize) == 0    // C 6941
      && *name1.offset (length_until_period) == b':' as c_char          // C 6942
    { name1 = name1.offset (length_until_period); }                     // C 6943
  if strncmp (name2, system_name, length_until_period as usize) == 0    // C 6944
      && *name2.offset (length_until_period) == b':' as c_char          // C 6945
    { name2 = name2.offset (length_until_period); }                     // C 6946

  while *name1 != 0 && *name1 == *name2                                 // C 6948
    {
      if *name1 == b':' as c_char                                       // C 6950
	{ seen_colon = true; }                                          // C 6951
      if seen_colon && *name1 == b'.' as c_char                         // C 6952
	{ return true; }                                                // C 6953
      name1 = name1.offset (1);                                         // C 6948
      name2 = name2.offset (1);                                         // C 6948
    }
  return seen_colon                                                     // C 6955
	 && (*name1 == b'.' as c_char || *name1 == 0)                   // C 6956
	 && (*name2 == b'.' as c_char || *name2 == 0);                  // C 6957
}

/// C 6960-6976: `static struct frame *pgtk_find_selection_owner (GdkWindow *window)` - `static` with
/// no address taken, so a plain `unsafe fn`; `FOR_EACH_FRAME!` is this unit's own macro, `XFRAME`
/// and `FRAME_PGTK_P` are the frame mirror's rows, `FRAME_GDK_WINDOW!` is `include/pgtkterm.rs`
/// 508-510's macro, and C's `NULL` is `std::ptr::null_mut ()`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_find_selection_owner (window: *mut GdkWindow) -> *mut frame
{
  let mut tail: Lisp_Object;                                            // C 6963
  let mut tem: Lisp_Object;                                             // C 6963
  let mut f: *mut frame;                                                // C 6964

  FOR_EACH_FRAME! (tail, tem, {                                         // C 6966
      f = XFRAME (tem);                                                 // C 6968

      if FRAME_PGTK_P (f)                                               // C 6970
	  && FRAME_GDK_WINDOW! (f) == window                            // C 6971
	{ return f; }                                                   // C 6972
  });

  return std::ptr::null_mut ();                                         // C 6975
}

/// C 6978-7025: `static gboolean pgtk_selection_event (GtkWidget *widget, GdkEvent *event, gpointer
/// user_data)` - `static` with its address taken at C 6892-6898 (the four `g_signal_connect!`s in
/// this file's own `pgtk_set_event_handler`, the fifty-seventh slice), so the port defines it
/// **`unsafe extern "C" fn`** (PORTING.md 1.4) - and that is why this definition closes the four
/// `E0425`s that slice registered.  C's three-way chain is the port's `if`/`else if`/`else if` with
/// C's two `||` arms merged as written; `inev.sie.kind` goes through this unit's `input_event_bits!`
/// (the record's own `bits` storage, `include/pgtkterm.rs` 594-607), the six `SELECTION_EVENT_*!`
/// macros are that mirror's rows, `GDK_PROPERTY_NOTIFY`/`GDK_SELECTION_*` are this round's mirror
/// placeholders and C's `TRUE`/`FALSE` are `true`/`false as gboolean`.  C 7012's assignment puns a `GdkAtom` into
/// the record's `GdkWindow *requestor` (C's own warning-level conversion), so the port writes the
/// cast out (`as *mut GdkWindow`) - the reading named the `E0308` that skips.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_selection_event (widget: *mut GtkWidget, event: *mut GdkEvent,
					   user_data: gpointer) -> gboolean
{
  let mut f: *mut frame;                                                // C 6982
  let mut inev: buffered_input_event = core::mem::zeroed ();            // C 6983

  if (*event).r#type == GDK_PROPERTY_NOTIFY                             // C 6985
    { pgtk_handle_property_notify (&raw mut (*event).property); }       // C 6986
  else if (*event).r#type == GDK_SELECTION_CLEAR
	   || (*event).r#type == GDK_SELECTION_REQUEST                  // C 6987-6988
    {
      f = pgtk_find_selection_owner ((*event).selection.window);        // C 6990

      if !f.is_null ()                                                  // C 6992
	{
	  EVENT_INIT (&raw mut inev.ie);                                // C 6994

	  input_event_bits! (kind, &raw mut inev.sie,
			     if (*event).r#type == GDK_SELECTION_CLEAR
			       { SELECTION_CLEAR_EVENT }
			       else { SELECTION_REQUEST_EVENT });       // C 6996-6998

	  SELECTION_EVENT_DPYINFO! (&raw mut inev.sie)
	    = FRAME_DISPLAY_INFO! (f);                                  // C 7000
	  SELECTION_EVENT_SELECTION! (&raw mut inev.sie)
	    = (*event).selection.selection;                             // C 7001
	  SELECTION_EVENT_TIME! (&raw mut inev.sie)
	    = (*event).selection.time;                                  // C 7002

	  if (*event).r#type == GDK_SELECTION_REQUEST                   // C 7004
	    {
	      /* FIXME: when does GDK destroy the requestor GdkWindow
		 object?

		 It would make sense to wait for the transfer to
		 complete.  But I don't know if GDK actually does
		 that.  */
	      SELECTION_EVENT_REQUESTOR! (&raw mut inev.sie)
		= (*event).selection.requestor as *mut GdkWindow;       // C 7012
	      SELECTION_EVENT_TARGET! (&raw mut inev.sie)
		= (*event).selection.target;                            // C 7013
	      SELECTION_EVENT_PROPERTY! (&raw mut inev.sie)
		= (*event).selection.property;                          // C 7014
	    }

	  evq_enqueue (&raw mut inev);                                  // C 7017
	  return true as gboolean;                                      // C 7018
	}
    }
  else if (*event).r#type == GDK_SELECTION_NOTIFY                       // C 7021
    { pgtk_handle_selection_notify (&raw mut (*event).selection); }     // C 7022

  return false as gboolean;                                             // C 7024
}

/* C: gtk/gtkdialog.h: `GtkWidget *gtk_dialog_get_content_area (GtkDialog *dialog);` - the C 7048
   call.  The mirror cannot own this row: `src/xwidget.rs` declares it for its own call (P-90 7), and
   `tools/check-mirror-dupes.py` named exactly that collision (`FAIL src/xwidget.rs: …`), so it
   stands here instead (the `gdk_atom_name`/`strstr` shape).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn gtk_dialog_get_content_area (dialog: *mut GtkDialog) -> *mut GtkWidget;
}

/// C: gtk/gtkdialog.h: `typedef struct _GtkDialog GtkDialog;` (a `GtkWindow`) - the C 7034 local's
/// type.  **The mirror cannot own it**: `src/xwidget.rs` 2373-2377 defines the same record for its
/// own casts, and the named reading of this slice caught the collision
/// (`E0428: the name GtkDialog is defined multiple times`), so the unit keeps its own row (P-90 7,
/// the `GDK_BUTTON_PRESS` shape).  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr(C)]
pub struct GtkDialog
{
  _private: [u8; 0],
}

/// C: gtk/gtkdialog.h: `#define GTK_DIALOG(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_DIALOG,
/// GtkDialog))` - C 7045's cast.  `src/xwidget.rs` 2384-2385 owns the same macro for its own casts,
/// so this unit carries its own copy (a `macro_rules!` is textually scoped, and the mirror may not
/// define what an includer has - P-90 7).
#[cfg (feature = "have_pgtk")]
macro_rules! GTK_DIALOG {
  ($o:expr) => { ($o) as *mut GtkDialog };
}

/// C 7027-7063: `static void pgtk_display_x_warning (GdkDisplay *display)` - `static` with no
/// address taken (C 7126 calls it), so a plain `unsafe fn`.  C 7038-7039's comment is carried
/// verbatim; `G_OBJECT_TYPE_NAME` is the mirror's macro, `strcmp` comes from the include graph
/// (`lib/include/string.rs`; the first reading named the `E0428` a use-site row produced there), C's
/// string literal is the port's
/// `c"…".as_ptr ()`; `GTK_DIALOG` is this slice's unit-local cast macro (xwidget owns the mirror's would-be row) and
/// `GTK_LABEL`/`GTK_CONTAINER` are the mirror's;
/// `gtk_dialog_new` is this slice's new mirror row and `gtk_dialog_get_content_area` its new
/// unit-local row (the collision above); and C's
/// `TRUE` is `true as gboolean`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_display_x_warning (display: *mut GdkDisplay)
{
  let mut dialog_widget: *mut GtkWidget;                                // C 7033
  let mut label: *mut GtkWidget;                                        // C 7033
  let mut content_area: *mut GtkWidget;                                 // C 7033
  let mut dialog: *mut GtkDialog;                                       // C 7034
  let mut window: *mut GtkWindow;                                       // C 7035
  let mut screen: *mut GdkScreen;                                       // C 7036

  /* Do this instead of GDK_IS_X11_DISPLAY because the GDK X header
     pulls in Xlib, which conflicts with definitions in pktgui.h.  */
  if strcmp (G_OBJECT_TYPE_NAME! (display),
	     c"GdkX11Display".as_ptr ()) != 0                           // C 7040-7041
    { return; }                                                         // C 7042

  dialog_widget = gtk_dialog_new ();                                    // C 7044
  dialog = GTK_DIALOG! (dialog_widget);                                 // C 7045
  window = GTK_WINDOW! (dialog_widget);                                 // C 7046
  screen = gdk_display_get_default_screen (display);                    // C 7047
  content_area = gtk_dialog_get_content_area (dialog);                  // C 7048

  gtk_window_set_title (window, c"Warning".as_ptr ());                  // C 7050
  gtk_window_set_screen (window, screen);                               // C 7051

  // C 7053-7058's six adjacent literals are **one** literal in the port (`c"…"` with continuation
  // backslashes, C's boundary spaces kept before them): Rust joins adjoined literals only through
  // `concat!`, which yields a `&str`, not the `*const c_char` this call takes (the first reading
  // named the six parse errors the adjoined spelling produces).
  label = gtk_label_new (c"You are trying to run Emacs configured with\n \
			   the \"pure-GTK\" interface under the X Window\n \
			   System.  That configuration is unsupported and\n \
			   will lead to sporadic crashes during transfer of\n \
			   large selection data.  It will also lead to\n \
			   various problems with keyboard input.\n".as_ptr ());  // C 7053-7058
  gtk_label_set_line_wrap (GTK_LABEL! (label), true as gboolean);       // C 7059
  gtk_container_add (GTK_CONTAINER! (content_area), label);             // C 7060
  gtk_widget_show (label);                                              // C 7061
  gtk_widget_show (dialog_widget);                                      // C 7062
}

/* Get rid of display DPYINFO, deleting all frames on it,
   and without sending any more commands to the X server.  */

/* <stdio.h>: `int sscanf (const char *s, const char *format, ...);` - the C 7364/7369 calls.  The
   include graph carries `strncmp`/`strlen`/`strcpy`, so only this one stands here (P-88 1).
   termhooks.h 906: `extern void delete_terminal (struct terminal *);` - the C 7324 call, whose
   definition is `src/terminal.rs` 416 and whose sibling use-site declaration is `src/term.rs`
   159-160; this unit declares its own (the `gen_help_event` shape).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
  pub fn delete_terminal (terminal: *mut terminal);
}

/// C 7313-7351: `static void pgtk_delete_display (struct pgtk_display_info *dpyinfo)` - `static` with
/// no address taken (C 4606's call), so a plain `unsafe fn`.  C's `for (t = terminal_list; t;
/// t = t->next_terminal)` is the port's `while` with C's step at the body's end and its `break` in
/// the body; `terminal_list`/`next_terminal`/`r#type`/`display_info` are `include/termhooks.rs`'s
/// rows, `output_pgtk` is that header's enumeration member, `delete_terminal` is this slice's new use-site
/// declaration (its definition is `src/terminal.rs` 416) and `pgtk_free_devices` this unit's own,
/// and C 7322's union member is the `*mut c_void` the mirror records, so the comparison casts
/// (`dpyinfo as *mut c_void`, the P-72 rule-3 shape), and C's two `xfree`s take the `void *` conversions.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_delete_display (dpyinfo: *mut pgtk_display_info)
{
  let mut t: *mut terminal;                                             // C 7316
  let mut last: *mut pgtk_touch_point;                                  // C 7317
  let mut tem: *mut pgtk_touch_point;                                   // C 7317

  /* Close all frames and delete the generic struct terminal for this
     X display.  */
  t = terminal_list;                                                    // C 7321
  while !t.is_null ()                                                   // C 7321
    {
      if (*t).r#type == output_pgtk
	  && (*t).display_info.pgtk == dpyinfo as *mut c_void           // C 7322
	{
	  delete_terminal (t);                                          // C 7324
	  break;                                                        // C 7325
	}
      t = (*t).next_terminal;                                           // C 7321
    }

  if x_display_list == dpyinfo                                          // C 7328
    { x_display_list = (*dpyinfo).next; }                               // C 7329
  else
    {
      let mut tail: *mut pgtk_display_info;                             // C 7332

      tail = x_display_list;                                            // C 7334
      while !tail.is_null ()                                            // C 7334
	{
	  if (*tail).next == dpyinfo                                    // C 7335
	    { (*tail).next = (*(*tail).next).next; }                    // C 7336
	  tail = (*tail).next;                                          // C 7334
	}
    }

  /* Free remaining touchpoints.  */
  tem = (*dpyinfo).touchpoints;                                         // C 7340
  while !tem.is_null ()                                                 // C 7341
    {
      last = tem;                                                       // C 7343
      tem = (*tem).next;                                                // C 7344
      xfree (last as *mut c_void);                                      // C 7345
    }

  pgtk_free_devices (dpyinfo);                                          // C 7348
  xfree (dpyinfo as *mut c_void);                                       // C 7349
}

/// C 7352-7378: `char *pgtk_xlfd_to_fontname (const char *xlfd)` - a **non-`static`** C definition,
/// so the port carries `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md 0.2).  C 7354-7358's
/// comment sits between the signature and the body, which the port keeps as a comment above the
/// body; `xmalloc`'s C `180` is `std::mem::size_of`-free here (a byte count) so it is passed as
/// written, `strncmp`/`strlen`/`strcpy` are the include graph's rows, `sscanf` is the row above, and
/// C's `name[0] = '\\0'` is the port's `*name = 0` while `"--%179[^-]-"`/`"-%*[^-]-%179[^-]-"`/
/// `"Monospace"` are `c"…".as_ptr ()`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_xlfd_to_fontname (xlfd: *const c_char) -> *mut c_char
/* --------------------------------------------------------------------------
    Convert an X font name (XLFD) to an Gtk font name.
    Only family is used.
    The string returned is temporarily allocated.
   -------------------------------------------------------------------------- */
{
  let mut name: *mut c_char = xmalloc (180) as *mut c_char;             // C 7360

  if strncmp (xlfd, c"--".as_ptr (), 2) == 0                            // C 7362
    {
      if sscanf (xlfd, c"--%179[^-]-".as_ptr (), name) != 1             // C 7364
	{ *name = 0; }                                                  // C 7365
    }
  else
    {
      if sscanf (xlfd, c"-%*[^-]-%179[^-]-".as_ptr (), name) != 1       // C 7369
	{ *name = 0; }                                                  // C 7370
    }

  /* stopgap for malformed XLFD input */
  if strlen (name) == 0                                                 // C 7374
    { strcpy (name, c"Monospace".as_ptr ()); }                          // C 7375

  return name;                                                          // C 7377
}
