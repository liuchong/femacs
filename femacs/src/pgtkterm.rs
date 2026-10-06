// femacs: port of src/pgtkterm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
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
                     G_CALLBACK! (pgtk_device_added_or_removal_cb), user_data);
  g_signal_connect! (G_OBJECT! (seat), c"device-removed".as_ptr (),
                     G_CALLBACK! (pgtk_device_added_or_removal_cb), user_data);
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
                             G_CALLBACK! (pgtk_device_added_or_removal_cb), dpyinfo as gpointer);
          g_signal_connect! (G_OBJECT! (seat), c"device-removed".as_ptr (),
                             G_CALLBACK! (pgtk_device_added_or_removal_cb), dpyinfo as gpointer);
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
unsafe fn pgtk_frame_raise_lower (f: *mut frame, raise_flag: bool)
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
      cancel_atimer ((*FRAME_X_OUTPUT! (f)).scale_factor_atimer);
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
      cancel_atimer ((*FRAME_X_OUTPUT! (f)).atimer_visible_bell);
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
unsafe fn pgtk_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_gravity: c_int)
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
unsafe fn pgtk_set_window_size (f: *mut frame, change_gravity: bool,
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
			     G_CALLBACK! (pgtk_make_frame_visible_wait_for_event_cb),
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
unsafe fn pgtk_make_frame_visible_invisible (f: *mut frame, visible: bool)
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
