// femacs: port of src/pgtkfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `pgtkfns.o`
// sits in `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac`
// 3875-3878 - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
// `if test "$window_system" = "pgtk"`), and `docs/baseline/frozen-build-sources.txt` lists no pgtk
// object for this target, so `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the
// ledger `docs/ports/pgtkfns.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.  This batch is P-86 ④'s first slice for this file:
// **C 1-1147**, which ends at the `}` of `update_watched_scale_factor` (C 1133-1147), and the
// **second slice C 1149-1196** - the "Lisp definitions" banner (C 1149-1153) and
// `DEFUN ("pgtk-set-monitor-scale-factor", …)` (C 1155-1196), whose two arms were measured on both
// sides of the slice and add no error and no warning to either (docs/ports/pgtkfns.md Evidence ⑥).
// **The next live definition is `Fx_create_frame` (C 1197-1757)** - the unit's biggest function -
// followed by the restack pair (C 1758-1796) and the GSETTINGS block (C 1798-1998).
//
//   * the **file gate** is `feature = "have_pgtk"` (`HAVE_PGTK`, `configure.ac` 3176's
//     `AC_DEFINE`, the macro the `window_system = pgtk` build defines), on every item *and on the
//     include graph* (P-86 ⑧ - an `include!` outside the gate breaks the frozen build at once,
//     which is why the reading in Evidence ① is a 336-byte empty object);
//   * the slice's own conditions are C 776/785's `GDK_PIXBUF_VERSION_2_44` `#if` pair (two `#pragma`
//     blocks - dead text with no Rust spelling) and the five `#ifdef CAIRO_HAS_*` arms of the
//     export-type chain (C 928/933/942/947).  `tools/check-cond-eval.py src/pgtkfns.c` reads
//     `11 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so all six are decided `false` for the
//     frozen configuration (the file **is** on `docs/baseline/config/cond-eval-verified.txt` 341);
//     the four cairo arms are carried, with the feature names `cairo_has_pdf_surface`,
//     `cairo_has_png_functions`, `cairo_has_ps_surface` and `cairo_has_svg_surface` - names this
//     tree has not registered (the ledger's Owed);
//   * two mirrors in the include graph still owe declarations: `include/gtkutil.rs` (every `xg_*`
//     entry point) and `include/pgtkterm.rs`'s `pgtk` chain (the `FRAME_*` accessors and the twelve
//     `pgtk_*` prototypes, which this unit carries itself because `gtkutil.h` 29's `pgtkterm.h` edge
//     is not in `include/gtkutil.rs` yet).  The GTK/GDK/cairo surface, by contrast, arrives from
//     `include/gtk.rs`, through this unit's own stand-in edge at C 28's chain, so this file no
//     longer declares it; the
//     records the accessors name (`pgtk_display_info`, `pgtk_output`) are `include/pgtkterm.rs`'s
//     and their layout is the named gap (`docs/ports/include-pgtkterm.md`).
//
// **The toolkit layer moved out (P-88 ⑤/P-90 ②③).**  `<gtk/gtk.h>` and the
// `<gdk/gdk.h>`/`<gdk-pixbuf/gdk-pixbuf.h>`/`<glib-object.h>`/`<cairo.h>` chain behind it used to be
// declared in this unit; they now come from the surface's single home, `femacs/src/include/gtk.rs`,
// through the stand-in edge this unit carries at C 28's chain (the edge belongs in
// `include/gtkutil.rs` - `## Owed` of `docs/ports/include-gtkutil.md` - and drops there).  The
// layering of the declaration is *read*, not invented:
// **GLib/GObject and cairo are on this machine** (`/opt/homebrew/include/glib-2.0` GLib 2.90.0,
// `/opt/homebrew/include/cairo` cairo 1.18.4), so every GLib and cairo declaration below quotes the
// header and line it was read from and `cairo_surface_type_t`'s layout is a **clang reading**
// (Evidence ②).  **GTK, GDK and gdk-pixbuf are not on this machine** (`ls -d
// /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0 /opt/homebrew/include/gdk-pixbuf-2.0`
// -> all absent; `pkg-config` is not installed; no `gtk/gtk.h` anywhere under `/opt/homebrew`,
// `/usr/local` or `/Library`), so `include/gtk.rs`'s citations of theirs name **the header alone,
// no line**, their records are the never-dereferenced `[u8; 0]` shape and every layout claim about
// them is UNVERIFIED and owed to a GTK toolchain (`docs/ports/include-gtk.md`).
//
// `docs/ports/pgtkfns.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no GTK
// tree to compile the arm against and no PGTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`

/* C 23-24: `<math.h>` and `<c-strcase.h>`, both gnulib mirrors.  The C lines sit outside every
   `#if` of the file body, so with the file gate on them (P-86 ⑧) the frozen configuration
   compiles not one line of this unit - the reading `docs/ports/pgtkfns.md`'s Evidence ① records.  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/math.rs");            // C 23: `#include <math.h>`
#[cfg (feature = "have_pgtk")]
include!("../lib/include/c-strcase.rs");       // C 24: `#include <c-strcase.h>`

/* C's include list (C 26-37).  Every `include!` carries the file gate (P-86 ⑧), and the C lines
   themselves stand as these comments where the mirror is what C would have included.  */
#[cfg (feature = "have_pgtk")]
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
/* lisp.h 30/33's `<string.h>` and `<limits.h>`, which C reaches through `"lisp.h"` (`strncmp` at
   C 122, `INT_MAX` at C 450/457/571/585): the two lib mirrors C's chain names (the `src/alloc.rs`
   56/59 and `src/atimer.rs` 39 readings of the same pair).  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (strncmp)
#[cfg (feature = "have_pgtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>` (INT_MAX)
#[cfg (feature = "have_pgtk")]
include!("include/blockinput.rs");             // C 27: `#include "blockinput.h"` (block_input)
#[cfg (feature = "have_pgtk")]
include!("include/gtkutil.rs");                // C 28: `#include "gtkutil.h"` (the GTK chain)
/* C 28's chain: `"gtkutil.h"` 26's `#include <gtk/gtk.h>` (C 24's `USE_GTK`) is where this unit's
   GTK/GDK/cairo names come from.  `include/gtkutil.rs` does not carry that edge yet (its head note
   records the measured blocker set - `src/xfns.rs`'s `GtkWidgetClass` needs `include/gtk.rs`'s
   record to grow a member, a change *inside* that mirror), so the edge stands here, at the C
   position that makes the names visible - the `src/xwidget.rs` C 30 stand-in shape.
   It carries **this file's own gate**, `have_pgtk` (every item of this unit is gated on it, and
   C's `HAVE_PGTK` implies `USE_GTK`, so the gate is the reach's, not a narrowing of C 24); it drops
   when `include/gtkutil.rs`'s C 26 edge lands (P-88 ⑤; `## Owed` of
   `docs/ports/include-gtkutil.md`).  */
#[cfg (feature = "have_pgtk")]
include!("include/gtk.rs");                    // gtkutil.h 26: `#include <gtk/gtk.h>`
#[cfg (feature = "have_pgtk")]
include!("include/window.rs");                 // C 29: `#include "window.h"` (update_mode_lines)
#[cfg (feature = "have_pgtk")]
include!("include/character.rs");              // C 30: `#include "character.h"`
#[cfg (feature = "have_pgtk")]
include!("include/buffer.rs");                 // C 31: `#include "buffer.h"`
#[cfg (feature = "have_pgtk")]
include!("include/keyboard.rs");               // C 32: `#include "keyboard.h"`
#[cfg (feature = "have_pgtk")]
include!("include/termhooks.rs");              // C 33: `#include "termhooks.h"` (FRAME_TERMINAL)
#[cfg (feature = "have_pgtk")]
include!("include/fontset.rs");                // C 34: `#include "fontset.h"`
#[cfg (feature = "have_pgtk")]
include!("include/font.rs");                   // C 35: `#include "font.h"` (font_open_by_name)
#[cfg (feature = "have_pgtk")]
include!("include/xsettings.rs");              // C 36: `#include "xsettings.h"`
#[cfg (feature = "have_pgtk")]
include!("include/atimer.rs");                 // C 37: `#include "atimer.h"` (struct atimer)
/* `frame.h` and `dispextern.h` are not in C 26-37's list: C reaches them through `"gtkutil.h"` ->
   `pgtkterm.h` 24 (`#include "frame.h"`) and 23 (`#include "dispextern.h"`), and this unit's text
   needs both - `struct frame` and its accessors (C 82-105 and every setter below), `Emacs_Color`
   (C 269/880) and `enum resource_types` (C 1078-1080).  Both `include!`s carry the reach's `#[cfg]`
   (P-72 rule 4, the `src/pgtkterm.rs` 130-145 shape).  */
#[cfg (feature = "have_pgtk")]
include!("include/systime.rs");                // termhooks.h 28: `#include "systime.h"` (Time)
#[cfg (feature = "have_pgtk")]
include!("include/frame.rs");                  // pgtkterm.h 24: `#include "frame.h"`
#[cfg (feature = "have_pgtk")]
include!("include/dispextern.rs");             // pgtkterm.h 23: `#include "dispextern.h"`

/* ----------------------------------------------------------------------------------------------
   C 28's chain: `<gtk/gtk.h>`, reached through `"gtkutil.h"` (`gtkutil.h` 26's
   `#include <gtk/gtk.h>` under `USE_GTK`).  The chain is live now - the stand-in edge above,
   at this unit's C 28, carries it (its owner is `include/gtkutil.rs`'s C 26, still owed) - so the
   third-party names this slice
   used to declare here (the `cairo_surface_type_t` family, `gpointer`/`gchar`/`gint`/`gboolean`/
   `gssize`/`GError`, the five `GdkPixbuf`/`GtkCssProvider`/`GtkWidget`/`GtkWindow`/`GdkWindow`
   handles with `GTK_WINDOW`, and their entry points) come from `include/gtk.rs` through it
   (P-88 ⑤'s move, P-90 ②; `## Evidence` of `docs/ports/pgtkfns.md`).  The pgtk `FRAME_*` family
   below is a different chain: it stays until `gtkutil.h` 29's `pgtkterm.h` edge lands in this
   mirror (P-90's next round).
   ---------------------------------------------------------------------------------------------- */

/* ----------------------------------------------------------------------------------------------
   `"pgtkterm.h"` (reached through `"gtkutil.h"` 29's `#include "pgtkterm.h"`): its mirror is a
   placeholder, so the accessors this slice uses and the entry points it calls are declared here
   with the reference's own line (P-72 rule 4; `docs/ports/pgtkterm.md` asks for the move).  The two
   records the accessors name (`pgtk_output`, `pgtk_display_info`) are that mirror's to define and
   stay the named gap - exactly the reading `src/pgtkterm.rs` 760-790 records for its own copies.
   ---------------------------------------------------------------------------------------------- */
/// C: pgtkterm.h 474: `#define FRAME_X_OUTPUT(f) ((f)->output_data.pgtk)` - the body is C's, with
/// the union member's `*mut c_void` cast spelled out (`include/frame.rs` 63's P-72 rule ③
/// degradation, the `src/pgtkterm.rs` 789 shape).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_OUTPUT {
  ($f:expr) => { ((*$f).output_data.pgtk as *mut pgtk_output) };
}
/// C: pgtkterm.h 477: `#define FRAME_DISPLAY_INFO(f) (FRAME_X_OUTPUT (f)->display_info)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_DISPLAY_INFO {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).display_info };
}
/// C: pgtkterm.h 478: `#define FRAME_FOREGROUND_COLOR(f) (FRAME_X_OUTPUT (f)->foreground_color)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FOREGROUND_COLOR {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).foreground_color };
}
/// C: pgtkterm.h 483/484: `#define FRAME_GTK_OUTER_WIDGET(f) (FRAME_X_OUTPUT (f)->widget)` /
/// `#define FRAME_GTK_WIDGET(f) (FRAME_X_OUTPUT (f)->edit_widget)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_GTK_OUTER_WIDGET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).widget };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_GTK_WIDGET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).edit_widget };
}
/// C: pgtkterm.h 490: `#define FRAME_X_WINDOW(f) FRAME_GTK_OUTER_WIDGET (f)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_WINDOW {
  ($f:expr) => { FRAME_GTK_OUTER_WIDGET! ($f) };
}
/// C: pgtkterm.h 517/518: `#define BLACK_PIX_DEFAULT(f) 0x000000` /
/// `#define WHITE_PIX_DEFAULT(f) 0xFFFFFF` - C's parameter is unused in both bodies and kept.
#[cfg (feature = "have_pgtk")]
macro_rules! BLACK_PIX_DEFAULT {
  ($f:expr) => {{ let _ = $f; 0x000000 as c_int }};
}
#[cfg (feature = "have_pgtk")]
macro_rules! WHITE_PIX_DEFAULT {
  ($f:expr) => {{ let _ = $f; 0xFFFFFF as c_int }};
}
/// C: pgtkterm.h 526-529: `#define FRAME_CR_SURFACE_DESIRED_WIDTH(f)
/// ((f)->output_data.pgtk->cr_surface_desired_width)` / `…_HEIGHT(f)
/// ((f)->output_data.pgtk->cr_surface_desired_height)` - C 1141-1143 reads both.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_SURFACE_DESIRED_WIDTH {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_width };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CR_SURFACE_DESIRED_HEIGHT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).cr_surface_desired_height };
}

/// C: pgtkterm.h 482/514: `#define FRAME_FONT(f)             (FRAME_X_OUTPUT (f)->font)` /
/// `#define FRAME_FONTSET(f) (FRAME_X_OUTPUT (f)->fontset)`.  C 1403 is the `if (!FRAME_FONT (f))`
/// test and C 1311 writes `FRAME_FONTSET (f) = -1;`.  Both read/write the record's own members
/// (`include/pgtkterm.rs` 298/302), which this unit's include graph does not carry (that mirror's
/// `pgtkterm.h` edge is owed), so the pair stands here with C's line (P-72 rule 4) - the same
/// device as `FRAME_X_OUTPUT!` above.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FONT {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).font };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FONTSET {
  ($f:expr) => { (*FRAME_X_OUTPUT! ($f)).fontset };
  ($f:expr, $v:expr) => { (*FRAME_X_OUTPUT! ($f)).fontset = $v };
}
/// C: pgtkterm.h 475: `#define FRAME_OUTPUT_DATA(f)      FRAME_X_OUTPUT (f)` - C 1629-1635 writes
/// seven members through it, so the calls keep C's macro name (this unit's own copy, the same
/// device as the two above).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { FRAME_X_OUTPUT! ($f) };
}
/// C: pgtkterm.h 495: `#define FRAME_X_DISPLAY(f)        (FRAME_DISPLAY_INFO (f)->gdpy)` - the
/// first argument of C 1577-1591's fifteen `gdk_cursor_new_for_display` calls, which C reaches
/// through `INSTALL_CURSOR`.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_X_DISPLAY {
  ($f:expr) => { (*FRAME_DISPLAY_INFO! ($f)).gdpy };
}

/* C's `f->output_method = …` (frame.h 446's `ENUM_BF`): `include/frame.rs` mirrors the reader
   (`FRAME_PGTK_P`) and the shift/mask, but not C's assignment shape, so this unit adds it under C's
   own name (the `src/androidfns.rs` 283-297 device; `frame_output_method` itself is private to that
   mirror).  C 1308's `f->output_method = output_pgtk;` is the only write this unit makes.  */
#[cfg (feature = "have_pgtk")]
macro_rules! set_output_method {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
                  | ((($v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT)
  };
}
#[cfg (feature = "have_pgtk")]
macro_rules! output_method {
  ($f:expr) => { output_method ($f) };
  ($f:expr, $v:expr) => { set_output_method! ($f, $v) };
}

/// C: frame.h 530: `bool_bf was_invisible : 1;` - read at C 1734/1748 and written at C 1731/1742/
/// 1745.  The mirror carries the reader function (`include/frame.rs` 1432) and the bit's
/// `_SHIFT`/`_UNIT`; the write goes through the mirror's `frame_set_flag` (the `FRAME_UNDECORATED!`
/// shape below, `src/androidfns.rs` 247-251).
#[cfg (feature = "have_pgtk")]
macro_rules! was_invisible {
  ($f:expr) => { (*$f).bits2 >> FRAME_WAS_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_WAS_INVISIBLE_UNIT, FRAME_WAS_INVISIBLE_SHIFT, ($v) as u32) };
}

/// C: frame.h 411: `bool_bf no_split : 1;` - written at C 1533 (`f->no_split = minibuffer_only ||
/// EQ (tem, Qt);`), read at C 2459/2471.  The mirror carries the bit's `_SHIFT`/`_UNIT`; the write
/// goes through the mirror's `frame_set_flag` (the shape above, `src/androidfns.rs` 205-208).
#[cfg (feature = "have_pgtk")]
macro_rules! no_split {
  ($f:expr) => { (*$f).bits1 >> FRAME_NO_SPLIT_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT, ($v) as u32) };
}
/// C: frame.h 491: `bool_bf can_set_window_size : 1;` - written at C 1706 (`f->can_set_window_size
/// = true;`).  Same device.
#[cfg (feature = "have_pgtk")]
macro_rules! can_set_window_size {
  ($f:expr) => { (*$f).bits1 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT,
		    ($v) as u32) };
}
/* C 2623's `f->wants_modeline = false;`, C 2639's `f->tooltip = true;` and C 2814's
   `f->no_split = true;` are the three frame bitfields this slice writes (`no_split!` stands above
   already); `include/frame.rs` carries each bit's `_SHIFT`/`_UNIT` and the readers, so the two
   writers stand here under C's names (the `src/haikufns.rs` 496-507 device).  */
/// C: frame.h 537: `bool_bf wants_modeline : 1;` - written at C 2623
#[cfg (feature = "have_pgtk")]
macro_rules! wants_modeline {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT, ($v) as u32) };
}
/// C: frame.h 553: `bool_bf tooltip : 1;` - written at C 2639
#[cfg (feature = "have_pgtk")]
macro_rules! tooltip {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT, ($v) as u32) };
}

/* C's four `FRAME_CONFIG_SCROLL_BAR_*` macros are l-values in C 2229/2230/2239/2240, and
   `include/frame.rs` mirrors each as a reader only, so this unit adds C's assignment shape under
   C's own names (the `src/androidfns.rs` 342-350 / `src/w32fns.rs` 3362-3372 device).  */
/// C: frame.h 1286: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f) ((f)->config_scroll_bar_width)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr) => { (*$f).config_scroll_bar_width };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_width = $v };
}
/// C: frame.h 1291: `#define FRAME_CONFIG_SCROLL_BAR_HEIGHT(f) ((f)->config_scroll_bar_height)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr) => { (*$f).config_scroll_bar_height };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_height = $v };
}
/// C: frame.h 1296: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) ((f)->config_scroll_bar_cols)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { (*$f).config_scroll_bar_cols };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}
/// C: frame.h 1301: `#define FRAME_CONFIG_SCROLL_BAR_LINES(f) ((f)->config_scroll_bar_lines)`
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr) => { (*$f).config_scroll_bar_lines };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_lines = $v };
}

/// C: pgtkterm.h 284: `extern struct pgtk_display_info *x_display_list;` - the object
/// `pgtkterm.c` defines and C 86/87/151/155 reads (the `src/pgtkterm.rs` 903 shape: the
/// declaration is a mirror's, and `include/pgtkterm.rs` has not got it yet).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  pub static mut x_display_list: *mut pgtk_display_info;
}

/// C: pgtkterm.h 569-679's prototype block - the twelve this slice calls, in C's own order of
/// first use.  `pgtk_frame_scale_factor` (pgtkterm.h 677) is named by `FRAME_SCALE_FACTOR`'s pgtk
/// arm (`frame.h` 1059), which C 1137's call expands.
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);` - C 106's
  /// call (the mirror `include/termhooks.rs` does not carry it; `src/xfns.rs` 299-301 declares it
  /// the same way).
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: pgtkterm.h 575: `extern struct pgtk_display_info *pgtk_term_init (Lisp_Object, char *);`
  pub fn pgtk_term_init (name: Lisp_Object, resource_name: *mut c_char) -> *mut pgtk_display_info;
  /// C 213: `pgtk_clear_frame (f);`
  /// C: pgtkterm.h 579: `extern void pgtk_clear_frame (struct frame *);`
  pub fn pgtk_clear_frame (f: *mut frame);
  /// C 247: `pgtk_frame_rehighlight (FRAME_DISPLAY_INFO (f));`
  /// C: pgtkterm.h 648: `extern void pgtk_frame_rehighlight (struct pgtk_display_info *);`
  pub fn pgtk_frame_rehighlight (dpyinfo: *mut pgtk_display_info);
  /// C 271: `if (pgtk_defined_color (f, SSDATA (color_name), &cdef, true, 0))`
  /// C: pgtkterm.h 588: `extern bool pgtk_defined_color (struct frame *, const char *,
  /// Emacs_Color *, bool, int);`
  pub fn pgtk_defined_color (f: *mut frame, color_name: *const c_char, color: *mut Emacs_Color,
                             alloc: bool, index: c_int) -> bool;
  /// C 414/478/508/519/...: `pgtk_clear_under_internal_border (f);`
  /// C: pgtkterm.h 597: `extern void pgtk_clear_under_internal_border (struct frame *);`
  pub fn pgtk_clear_under_internal_border (f: *mut frame);
  /// C 638/661: `result = pgtk_text_icon (f, SSDATA (…));`
  /// C: pgtkterm.h 675: `extern bool pgtk_text_icon (struct frame *, const char *);`
  pub fn pgtk_text_icon (f: *mut frame, name: *const c_char) -> bool;
  /// C 719/724: `pgtk_make_frame_invisible (f);` / `pgtk_make_frame_visible (f);`
  /// C: pgtkterm.h 639/640: `extern void pgtk_make_frame_visible (struct frame *);` / `extern void
  /// pgtk_make_frame_invisible (struct frame *);`
  pub fn pgtk_make_frame_invisible (f: *mut frame);
  pub fn pgtk_make_frame_visible (f: *mut frame);
  /// C 1030: `pgtk_free_frame_resources (f);`
  /// C: pgtkterm.h 641: `extern void pgtk_free_frame_resources (struct frame *);`
  pub fn pgtk_free_frame_resources (f: *mut frame);
  /// C 874/887: `pgtk_parse_color (f, SSDATA (new_value), &rgb)`
  /// C: pgtkterm.h 592: `extern int pgtk_parse_color (struct frame *, const char *, Emacs_Color *);`
  pub fn pgtk_parse_color (f: *mut frame, color_name: *const c_char, color: *mut Emacs_Color)
                           -> c_int;
  /// C 1140-1144: `pgtk_cr_update_surface_desired_size (f, …);`
  /// C: pgtkterm.h 611: `extern void pgtk_cr_update_surface_desired_size (struct frame *, int, int,
  /// bool);`
  pub fn pgtk_cr_update_surface_desired_size (f: *mut frame, width: c_int, height: c_int,
                                              force: bool);
  /// C 958: `return pgtk_cr_export_frames (frames, surface_type);`
  /// C: pgtkterm.h 619: `extern Lisp_Object pgtk_cr_export_frames (Lisp_Object,
  /// cairo_surface_type_t);`
  pub fn pgtk_cr_export_frames (frames: Lisp_Object, surface_type: cairo_surface_type_t)
                                -> Lisp_Object;
  /// C 1006-1010's four entries of `pgtk_frame_parm_handlers` (pgtkterm.c's own definitions)
  /// C: pgtkterm.h 605/606/607/608: `extern void pgtk_set_parent_frame (struct frame *,
  /// Lisp_Object, Lisp_Object);` / `extern void pgtk_set_no_focus_on_map (…)` / `extern void
  /// pgtk_set_no_accept_focus (…)` / `extern void pgtk_set_z_group (…)`
  pub fn pgtk_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn pgtk_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn pgtk_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn pgtk_set_z_group (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 2169: `pgtk_delete_terminal (dpyinfo->terminal);` and C 2204's
  /// `pgtk_xlfd_to_fontname (SSDATA (name))` - the two `pgtkterm.h` entries this slice calls
  /// (pgtkterm.h 637/720; the mirror `include/pgtkterm.rs` carries both, but this unit's include
  /// graph does not).
  pub fn pgtk_delete_terminal (terminal: *mut terminal);
  pub fn pgtk_xlfd_to_fontname (name: *const c_char) -> *mut c_char;
  /// C 2908/2909: `pgtk_display_pixel_width (FRAME_DISPLAY_INFO (f))` /
  /// `pgtk_display_pixel_height (FRAME_DISPLAY_INFO (f))`
  /// C: pgtkterm.h 601/602: `extern int pgtk_display_pixel_height (struct pgtk_display_info *);` /
  /// `extern int pgtk_display_pixel_width (struct pgtk_display_info *);` (`src/pgtkterm.rs` 1355
  /// defines the latter; the former is still that unit's remainder)
  pub fn pgtk_display_pixel_width (dpyinfo: *mut pgtk_display_info) -> c_int;
  pub fn pgtk_display_pixel_height (dpyinfo: *mut pgtk_display_info) -> c_int;
  /// C 2986: `if (xg_hide_tooltip (f))`
  /// C: gtkutil.h 216: `extern bool xg_hide_tooltip (struct frame *f);` (`src/gtkutil.rs` 1399/1438
  /// defines it twice, one per `use_gtk` arm)
  pub fn xg_hide_tooltip (f: *mut frame) -> bool;
  /// C 1547/2747: `pgtk_set_event_handler (f);`
  /// C: pgtkterm.h 598: `extern void pgtk_set_event_handler (struct frame *);`
  pub fn pgtk_set_event_handler (f: *mut frame);
  /// C 1740: `pgtk_iconify_frame (f);`
  /// C: pgtkterm.h 642: `extern void pgtk_iconify_frame (struct frame *);`
  pub fn pgtk_iconify_frame (f: *mut frame);
}

/* ----------------------------------------------------------------------------------------------
   Emacs' own entry points and macros this slice calls, from headers whose mirrors do not carry them
   (P-72 rule 4; each keeps C's prototype and the header line it was read from, the `src/xfns.rs`
   296-323 shape).  The `pgtkterm.h` block above is the same device for the PGTK header.
   ---------------------------------------------------------------------------------------------- */

/// C: dispextern.h 3983-3992: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT,
/// RES_TYPE_BOOLEAN, RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };`, itself inside
/// C's `#ifdef HAVE_WINDOW_SYSTEM` (C 3983).  C's enum ABI is `c_int` (PORTING.md 1.3).  The mirror
/// `include/dispextern.rs` has not got it; `src/frame.rs` 6082-6094 carries a use-site copy of its
/// own (the ledger's Owed names both).
#[cfg (feature = "have_pgtk")]
pub type resource_types = c_int;
#[cfg (feature = "have_pgtk")]
const RES_TYPE_NUMBER: resource_types = 0;          // C: dispextern.h 3984
const RES_TYPE_FLOAT: resource_types = 1;           // C: dispextern.h 3985
const RES_TYPE_BOOLEAN: resource_types = 2;         // C: dispextern.h 3986
const RES_TYPE_STRING: resource_types = 3;          // C: dispextern.h 3988
const RES_TYPE_SYMBOL: resource_types = 4;          // C: dispextern.h 3989

/// C: dispextern.h 3063: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object,
/// Lisp_Object);` - the element type of C 963-1015's `pgtk_frame_parm_handlers[]`.  The mirror
/// `include/dispextern.rs` does not carry the alias (its `redisplay_interface.frame_parm_handlers`
/// is `*mut c_void`), so the array spells the typedef's own body here - exactly what
/// `src/frame.rs` 5011-5014 does for `handle_frame_param` and `include/w32term.rs` 46/320-355
/// records for its own handler array.
#[cfg (feature = "have_pgtk")]
type frame_parm_handler =
  Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object)>;

#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /* dispextern.h.  */
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
  pub fn adjust_frame_glyphs (f: *mut frame);
  /// C: dispextern.h 3902: `void free_glyphs (struct frame *);`
  pub fn free_glyphs (f: *mut frame);
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);`
  pub fn clear_glyph_matrix (matrix: *mut c_void);
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);`
  pub fn clear_current_matrices (f: *mut frame);
  /// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
  pub fn clear_frame (f: *mut frame);
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, invis: bool);
  /// C: dispextern.h 3748: `extern Lisp_Object image_find_image_file (Lisp_Object);`
  pub fn image_find_image_file (file: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3794-3795: `void update_face_from_frame_parameter (struct frame *,
  /// Lisp_Object, Lisp_Object);`
  pub fn update_face_from_frame_parameter (f: *mut frame, param: Lisp_Object, new_value: Lisp_Object);
  /// C: dispextern.h 3994-3996: `extern Lisp_Object gui_display_get_arg (Display_Info *,
  /// Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);` - C 1083's
  /// first argument is always `NULL`, so the port spells `Display_Info` as an untyped pointer
  /// (the mirror's `check_x_display_info`-shaped API reaches the record through its own mirror).
  pub fn gui_display_get_arg (dpyinfo: *mut c_void, alist: Lisp_Object, param: Lisp_Object,
                              name: *const c_char, x_name: *const c_char,
                              r#type: resource_types) -> Lisp_Object;
  /// C: dispextern.h 4001-4003: `extern Lisp_Object gui_default_parameter (struct frame *,
  /// Lisp_Object, Lisp_Object, Lisp_Object, const char *, const char *, enum resource_types);`
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
                                deflt: Lisp_Object, name: *const c_char, x_name: *const c_char,
                                r#type: resource_types) -> Lisp_Object;

  /* frame.h.  */
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool, …);`
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int, inhibit: c_int,
                            pretend: bool, parameter: Lisp_Object);
  /// C: frame.h 1474-1475: `extern WINDOW_SYSTEM_RETURN struct frame *decode_window_system_frame
  /// (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: frame.h 1873: `extern void validate_x_resource_name (void);`
  pub fn validate_x_resource_name ();
  /// C: frame.h 1883: `extern void free_frame_menubar (struct frame *);`
  pub fn free_frame_menubar (f: *mut frame);
  /* frame.h 1844-1871's `gui_set_*` family, in C's own declaration order; the names are the
     entries of C 965-1014's `pgtk_frame_parm_handlers[]`.  */
  /// C: frame.h 1844-1851: `extern void gui_set_fullscreen (struct frame *, Lisp_Object,
  /// Lisp_Object);` and its seven siblings
  pub fn gui_set_fullscreen (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_line_spacing (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_screen_gamma (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_font (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_font_backend (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_left_fringe (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_right_fringe (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1852-1855: `extern void gui_set_right_divider_width (struct frame *, Lisp_Object,
  /// Lisp_Object);` / `extern void gui_set_bottom_divider_width (…);`
  pub fn gui_set_right_divider_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_bottom_divider_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1856: `extern void gui_set_visibility (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn gui_set_visibility (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1857/1858: `extern void gui_set_autoraise (struct frame *, Lisp_Object,
  /// Lisp_Object);` / `extern void gui_set_autolower (…);`
  pub fn gui_set_autoraise (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_autolower (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1859-1863: `extern void gui_set_unsplittable (struct frame *, Lisp_Object,
  /// Lisp_Object);` and its four siblings
  pub fn gui_set_unsplittable (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_vertical_scroll_bars (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_horizontal_scroll_bars (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_scroll_bar_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  pub fn gui_set_scroll_bar_height (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1867: `extern void gui_set_alpha (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn gui_set_alpha (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1868: `extern void gui_set_alpha_background (struct frame *, Lisp_Object,
  /// Lisp_Object);` - C 226's call
  pub fn gui_set_alpha_background (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);
  /// C: frame.h 1869-1870: `extern void gui_set_borders_respect_alpha_background (struct frame *,
  /// Lisp_Object, Lisp_Object);`
  pub fn gui_set_borders_respect_alpha_background (f: *mut frame, arg: Lisp_Object,
                                                   oldval: Lisp_Object);
  /// C: frame.h 1871: `extern void gui_set_no_special_glyphs (struct frame *, Lisp_Object,
  /// Lisp_Object);`
  pub fn gui_set_no_special_glyphs (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object);

  /* lisp.h.  */
  /// C: lisp.h 607: `extern int check_int_nonnegative (Lisp_Object);` - C 604's call
  pub fn check_int_nonnegative (value: Lisp_Object) -> c_int;
  /// C: lisp.h 4409: `void set_frame_cursor_types (struct frame *, Lisp_Object);` - C 683's call
  pub fn set_frame_cursor_types (f: *mut frame, arg: Lisp_Object);
  /// C: lisp.h 5117/5120: `extern void store_frame_param (struct frame *, Lisp_Object,
  /// Lisp_Object);` / `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;

  /* font.h.  */
  /// C: font.h 883: `extern Lisp_Object font_open_by_name (struct frame *f, Lisp_Object name);`
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;

  /* gtkutil.h (its mirror is a placeholder; these five are the ones this slice calls).  */
  /// C: gtkutil.h 158/159: `extern void update_frame_tool_bar (struct frame *f);` / `extern void
  /// free_frame_tool_bar (struct frame *f);`
  pub fn update_frame_tool_bar (f: *mut frame);
  pub fn free_frame_tool_bar (f: *mut frame);
  /// C: gtkutil.h 160: `extern void xg_change_toolbar_position (struct frame *f, Lisp_Object pos);`
  pub fn xg_change_toolbar_position (f: *mut frame, pos: Lisp_Object);
  /// C: gtkutil.h 183: `extern void xg_set_background_color (struct frame *f, unsigned long bg);`
  pub fn xg_set_background_color (f: *mut frame, bg: c_ulong);
  /// C: gtkutil.h 200/202/205: `extern void xg_set_undecorated (struct frame *f, Lisp_Object
  /// undecorated);` / `extern void xg_set_skip_taskbar (…)` / `extern void xg_set_override_redirect
  /// (…)`
  pub fn xg_set_undecorated (f: *mut frame, undecorated: Lisp_Object);
  pub fn xg_set_skip_taskbar (f: *mut frame, skip_taskbar: Lisp_Object);
  pub fn xg_set_override_redirect (f: *mut frame, override_redirect: Lisp_Object);

  /* The third slice's own calls: C's prototype and the header line it was read from, the device
     this block already uses (P-72 rule 4).  */
  /// C: frame.h 1430: `extern EMACS_UINT frame_set_id_from_params (struct frame *f, Lisp_Object
  /// params);` - C 1305's call
  pub fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object) -> EMACS_UINT;
  /// C: frame.h 1479: `extern struct frame *make_frame (bool);` - C 1246's call
  pub fn make_frame (mini: bool) -> *mut frame;
  /// C: frame.h 1489: `struct frame *make_minibuffer_frame (void);` - C 1241's call
  pub fn make_minibuffer_frame () -> *mut frame;
  /// C: frame.h 1491-1493: `struct frame *make_frame_without_minibuffer (Lisp_Object mini_window,
  /// KBOARD *kb, Lisp_Object display);` - C 1239/1243's calls
  pub fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut kboard,
					display: Lisp_Object) -> *mut frame;
  /// C: frame.h 1509: `extern Lisp_Object delete_frame (Lisp_Object, Lisp_Object);` - C 1399's call
  pub fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1865: `extern long gui_figure_window_size (struct frame *, Lisp_Object, bool, bool);`
  /// - C 1520's call
  pub fn gui_figure_window_size (f: *mut frame, parms: Lisp_Object, toolbar_p: bool,
				 tabbar_p: bool) -> c_long;
  /// C: dispextern.h 3821: `void init_frame_faces (struct frame *);` - C 1479's call
  pub fn init_frame_faces (f: *mut frame);
  /// C: font.h 896: `extern void register_font_driver (struct font_driver const *, struct frame *);`
  /// - C 1387/1390's calls
  pub fn register_font_driver (driver: *const c_void, f: *mut frame);
  /// C: pgtkterm.h 625: `extern void initialize_frame_menubar (struct frame *);` - C 1703's call
  pub fn initialize_frame_menubar (f: *mut frame);
  /// C: gtkutil.h 156: `extern void xg_wm_set_size_hint (struct frame *, long int, bool);` - C 1710's
  /// call
  pub fn xg_wm_set_size_hint (f: *mut frame, prompting: c_long, need_p: bool);
  /// C: frame.h 774-776: `INLINE void fset_parent_frame (struct frame *f, Lisp_Object val)
  /// { f->parent_frame = val; }` (the C `INLINE` setter, inside `HAVE_WINDOW_SYSTEM`) - C 1248's
  /// call; the mirror has not got it (the `src/haikufns.rs` 304-306 shape).
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
  /// C: gtkutil.h 181: `extern bool xg_create_frame_widgets (struct frame *f);` - C 1545's call
  /// (`src/gtkutil.rs` 2085 defines it; the mirror `include/gtkutil.rs` owes every `xg_*` row).
  pub fn xg_create_frame_widgets (f: *mut frame) -> bool;
  /// C 2227/2237: `minw = xg_get_default_scrollbar_width (f);` /
  /// `min_height = xg_get_default_scrollbar_height (f);`
  /// C: gtkutil.h 153/154: `extern int xg_get_default_scrollbar_width (struct frame *f);` /
  /// `extern int xg_get_default_scrollbar_height (struct frame *f);`
  pub fn xg_get_default_scrollbar_width (f: *mut frame) -> c_int;
  pub fn xg_get_default_scrollbar_height (f: *mut frame) -> c_int;
  /// C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);` - C 1784-1785's calls
  /// (the mirror `include/frame.rs` has not got it; `src/haikufns.rs` 200-202 declares it the same
  /// way)
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
  /// C: gtkutil.h 201: `extern void xg_frame_restack (struct frame *f1, struct frame *f2, bool
  /// above);` - C 1762's call (`src/gtkutil.rs` 2687 defines it)
  pub fn xg_frame_restack (f1: *mut frame, f2: *mut frame, above: bool);
  /* `<string.h>`'s two that `lib/include/string.rs` has not got (`strlen`/`strcpy`/`strcmp` are
     that mirror's): C 1926's `strnlen` and C 1949's `strncpy` (the `src/doprnt.rs` 69 and
     `src/haikufont.rs` 90-92 shape).  */
  /// C: string.h: `size_t strnlen (const char *s, size_t maxlen);`
  pub fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
  /// C: string.h: `char *strncpy (char *dest, const char *src, size_t n);`
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
  /// C: frame.h 1504: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);` -
  /// C 2003's call passes `NULL` (the `src/androidmenu.rs` 92 shape); the mirror has not got it
  pub fn check_window_system (f: *mut frame);
  /// C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
  /// - C 2151's call (the `src/emacs.rs` 196 / `src/alloc.rs` 6231 shape, `-> !`)
  pub fn fatal (msgid: *const c_char, ...) -> !;
  /// C: dispextern.h 2068: `extern bool face_change;` - C 2606/2834 read and write it; the
  /// mirror carries the declaration inside its `dispextern_unit_names` module, which this unit's
  /// include graph does not glob-import, so it stands here (the `src/haikufns.rs` 857 shape)
  pub static mut face_change: bool;
  /// C: string.h: `char *strstr (const char *haystack, const char *needle);` - C 2201's two calls
  /// (`lib/include/string.rs` has not got it; the `src/font.rs` 437-438 shape)
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
  /// C: lisp.h 5555: `extern void dupstring (char **, char const *);` - C 2498's call (the mirror
  /// `include/lisp.rs` has not got it; `src/frame.rs`'s `MonitorInfo.name` is the `char **` side)
  pub fn dupstring (dest: *mut *mut c_char, src: *const c_char);

  /* font.h 991-993: the two FT/CR driver objects C 1387-1392 registers -
     `#if defined (USE_CAIRO) || defined (USE_BE_CAIRO)` / `extern struct font_driver const
     ftcrfont_driver;` / `#ifdef HAVE_HARFBUZZ` / `extern struct font_driver ftcrhbfont_driver;`.
     `include/font.rs` has not got either, so the pair stands here as the `void *` stand-ins
     `src/haikufns.rs` 330-335 uses for the same two names (P-72 rule 3: `struct font_driver`'s
     record is `src/ftcrfont.rs`'s).  */
  pub static mut ftcrfont_driver: *mut c_void;
  /// C: font.h 992-994: `#ifdef HAVE_HARFBUZZ` / `extern struct font_driver ftcrhbfont_driver;` /
  /// `#endif` - undef in the frozen configuration
  /// (`docs/baseline/config/darwin-arm64.macros.tsv` 1389), so C 1389-1391 is carried as C's arm.
  #[cfg (feature = "harfbuzz")]
  pub static mut ftcrhbfont_driver: *mut c_void;

  /* C 1213's `count` and C 1220's two null resource names come from mirrors this file already
     carries: `SPECPDL_INDEX` is `include/lisp.rs` 2624, the `const char *` arguments are C's
     `0`/`NULL` written as `std::ptr::null ()`.  */
}

/* C 39-41: C's two forward declarations -

     static int x_decode_color (struct frame *f, Lisp_Object color_name, int mono_color);
     static struct pgtk_display_info *pgtk_display_info_for_name (Lisp_Object);

   - are dropped (PORTING.md 0.1-12: Rust has no forward declaration and its items are
   order-independent).  Both definitions are inside this slice (C 1055-1072 and C 133-164), so
   nothing else is owed for them (`src/xfns.rs` 163-172's reading of the same rule).  */

/* frame.h's four `bool_bf` members this slice *writes* - `explicit_name` (C 331/335),
   `tab_bar_redisplayed` (C 509), `tab_bar_resized` (C 494/508) and `tool_bar_resized`
   (C 546/560) - are not accessors in `include/frame.rs` (they are bits of the record's
   `bits1`/`bits2`, whose `_SHIFT`/`_UNIT` constants the mirror carries).  Each keeps C's member
   name and gets the two-arm shape, reading through the mirror's own `frame_flag` (so the unit is
   the arm's data, not a hard-coded `bits1`) - the `src/pgtkterm.rs` 853-867 pattern and the
   private `macro_rules!` family `src/frame.rs` 238/274/282 keeps for its own crate root.  */
/// C: frame.h 417: `bool_bf explicit_name : 1;`
#[cfg (feature = "have_pgtk")]
macro_rules! explicit_name {
  ($f:expr) => { frame_flag ($f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT, ($v) as u32) };
}
/// C: frame.h 503: `bool_bf tab_bar_redisplayed : 1;`
#[cfg (feature = "have_pgtk")]
macro_rules! tab_bar_redisplayed {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_REDISPLAYED_UNIT, FRAME_TAB_BAR_REDISPLAYED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TAB_BAR_REDISPLAYED_UNIT, FRAME_TAB_BAR_REDISPLAYED_SHIFT,
                    ($v) as u32) };
}
/// C: frame.h 504: `bool_bf tab_bar_resized : 1;`
#[cfg (feature = "have_pgtk")]
macro_rules! tab_bar_resized {
  ($f:expr) => { frame_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT, ($v) as u32) };
}
/// C: frame.h 514: `bool_bf tool_bar_resized : 1;`
#[cfg (feature = "have_pgtk")]
macro_rules! tool_bar_resized {
  ($f:expr) => { frame_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_TOOL_BAR_RESIZED_UNIT, FRAME_TOOL_BAR_RESIZED_SHIFT, ($v) as u32) };
}

/* frame.h's accessors C *writes*.  `include/frame.rs` carries each as a read-only getter (and
   `FRAME_EXTERNAL_MENU_BAR`/`FRAME_EXTERNAL_TOOL_BAR` as a `false`-returning arm when their
   `HAVE_EXT_*_BAR` is off), so the port gives each the two-arm form the `src/pgtkterm.rs`
   794-812 treatment uses: the read arm is the mirror's getter, the write arm is C's own member.
   The `!` in an invocation is the port's spelling of C's assignment (a recorded deviation).  */
/// C: frame.h 1086/1087: `#define FRAME_MENU_BAR_LINES(f) (f)->menu_bar_lines` /
/// `#define FRAME_MENU_BAR_HEIGHT(f) (f)->menu_bar_height` - written at C 416/417.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr) => { FRAME_MENU_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).menu_bar_lines = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr) => { FRAME_MENU_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).menu_bar_height = $v };
}
/// C: frame.h 1090/1091: `#define FRAME_TAB_BAR_LINES(f) (f)->tab_bar_lines` /
/// `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height` - written at C 487/486.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TAB_BAR_LINES {
  ($f:expr) => { FRAME_TAB_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_lines = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
/// C: frame.h 1107/1108: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines` /
/// `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height` - written at C 528/529.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr) => { FRAME_TOOL_BAR_LINES ($f) };
  ($f:expr, $v:expr) => { (*$f).tool_bar_lines = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr) => { FRAME_TOOL_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tool_bar_height = $v };
}
/// C: frame.h 1234/1239: `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` /
/// `#define FRAME_BACKGROUND_PIXEL(f) ((f)->background_pixel)` - written at C 180/211.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_FOREGROUND_PIXEL {
  ($f:expr) => { FRAME_FOREGROUND_PIXEL ($f) };
  ($f:expr, $v:expr) => { (*$f).foreground_pixel = $v };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_BACKGROUND_PIXEL {
  ($f:expr) => { FRAME_BACKGROUND_PIXEL ($f) };
  ($f:expr, $v:expr) => { (*$f).background_pixel = $v };
}
/// C: frame.h 1078-1083: `#ifdef HAVE_EXT_MENU_BAR` / `#define FRAME_EXTERNAL_MENU_BAR(f)
/// (f)->external_menu_bar` / `#else` / `#define FRAME_EXTERNAL_MENU_BAR(f) false` - written at
/// C 419/429/432.  The write arm names the mirror's flag constant, which exists only under
/// `have_ext_menu_bar` (the arm C's own `lisp.h` 3931-3936 gives `USE_GTK`).
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_EXTERNAL_MENU_BAR {
  ($f:expr) => { FRAME_EXTERNAL_MENU_BAR ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}
/// C: frame.h 1093-1098: `#ifdef HAVE_EXT_TOOL_BAR` / `#define FRAME_EXTERNAL_TOOL_BAR(f)
/// (f)->external_tool_bar` / `#else` / `#define FRAME_EXTERNAL_TOOL_BAR(f) false` - written at
/// C 534/543.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_EXTERNAL_TOOL_BAR {
  ($f:expr) => { FRAME_EXTERNAL_TOOL_BAR ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_TOOL_BAR_FLAG_UNIT, FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT,
                    ($v) as u32) };
}
/// C: frame.h 1239/1245/1247: `#define FRAME_UNDECORATED(f) ((f)->undecorated)` /
/// `#define FRAME_OVERRIDE_REDIRECT(f) ((f)->override_redirect)` /
/// `#define FRAME_SKIP_TASKBAR(f) ((f)->skip_taskbar)` - `undecorated` is written at C 695,
/// the other two at C 707/718, each through the mirror's own flag constant.
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_UNDECORATED {
  ($f:expr) => { FRAME_UNDECORATED ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_SKIP_TASKBAR {
  ($f:expr) => { FRAME_SKIP_TASKBAR ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_SKIP_TASKBAR_UNIT, FRAME_SKIP_TASKBAR_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_pgtk")]
macro_rules! FRAME_OVERRIDE_REDIRECT {
  ($f:expr) => { FRAME_OVERRIDE_REDIRECT ($f) };
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_OVERRIDE_REDIRECT_FLAG_UNIT, FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT,
                    ($v) as u32) };
}

/* C 43 and C 45-46: this file's two file-scope objects.  */
/// C 43: `static const char *pgtk_app_name = "Emacs";`
#[cfg (feature = "have_pgtk")]
static mut pgtk_app_name: *const c_char = c"Emacs".as_ptr ();
/// C 45-46: `/* Scale factor manually set per monitor.  */`
/// `static Lisp_Object monitor_scale_factor_alist;` - C's tentative definition, i.e. zero
/// (PORTING.md 0.1-13).
#[cfg (feature = "have_pgtk")]
static mut monitor_scale_factor_alist: Lisp_Object = std::ptr::null_mut ();

/* ==========================================================================

    Internal utility functions

   ========================================================================== */

/// C: C 54-55: `static double pgtk_get_monitor_scale_factor (const char *model)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_monitor_scale_factor (model: *const c_char) -> c_double
{
  if model.is_null ()
    { return 0.0; }

  let mdl: Lisp_Object = build_string (model);
  let tem: Lisp_Object = Fassoc (mdl, monitor_scale_factor_alist, Qnil ());
  if NILP (tem)
    { return 0.0; }				// C 63: `return 0;`
  let cdr: Lisp_Object = Fcdr (tem);
  if NILP (cdr)
    { return 0.0; }				// C 66: `return 0;`
  if FIXNUMP (cdr)
    { return XFIXNUM (cdr) as c_double; }
  else if FLOATP (cdr)
    { return XFLOAT_DATA (cdr); }
  else
    { error (c"Unknown type of scale-factor".as_ptr ()); }
}

/// C: C 75-76: `struct pgtk_display_info *check_pgtk_display_info (Lisp_Object object)`
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_pgtk_display_info (object: Lisp_Object) -> *mut pgtk_display_info
{
  let mut dpyinfo: *mut pgtk_display_info = std::ptr::null_mut ();

  if NILP (object)
    {
      let sf: *mut frame = XFRAME (selected_frame);

      if FRAME_PGTK_P (sf) && FRAME_LIVE_P (sf)
	{ dpyinfo = FRAME_DISPLAY_INFO! (sf); }
      else if !x_display_list.is_null ()
	{ dpyinfo = x_display_list; }
      else
	{ error (c"Frames are not in use or not initialized".as_ptr ()); }
    }
  else if TERMINALP (object)
    {
      let t: *mut terminal = decode_live_terminal (object);

      if (*t).r#type != output_pgtk
	{ error (c"Terminal %d is not a display".as_ptr (), (*t).id); }

      dpyinfo = (*t).display_info.pgtk as *mut pgtk_display_info;
    }
  else if STRINGP (object)
    { dpyinfo = pgtk_display_info_for_name (object); }
  else
    {
      let f: *mut frame = decode_window_system_frame (object);
      dpyinfo = FRAME_DISPLAY_INFO! (f);
    }

  dpyinfo
}

/* On Wayland, even if without WAYLAND_DISPLAY, --display DISPLAY
   works, but gdk_display_get_name always return "wayland-0", which
   may be different from DISPLAY.  If with WAYLAND_DISPLAY, then it
   always returns WAYLAND_DISPLAY.  So pgtk Emacs is confused and
   enters multi display environment.  To workaround this situation,
   treat all the wayland-* as the same display.  */
/// C: C 117-118: `static Lisp_Object is_wayland_display (Lisp_Object dpyname)`
#[cfg (feature = "have_pgtk")]
unsafe fn is_wayland_display (dpyname: Lisp_Object) -> Lisp_Object
{
  let mut p: *const c_char = SSDATA (dpyname);
  if strncmp (p, c"wayland-".as_ptr (), 8) != 0
    { return Qnil (); }
  p = p.add (8);
  /* C 124-127's `do { … } while (*++p != '\0');`: the pre-increment is the `p.add (1)` before
     the test, and the test's `break` is the loop's end.  */
  loop {
    if (*p as c_int) < ('0' as c_int) || (*p as c_int) > ('9' as c_int)
      { return Qnil (); }
    p = p.add (1);
    if *p == 0 { break; }
  }
  Qt ()
}

/* Return the X display structure for the display named NAME.
   Open a new connection if necessary.  */
/// C: C 133-134: `static struct pgtk_display_info *pgtk_display_info_for_name (Lisp_Object name)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_display_info_for_name (name: Lisp_Object) -> *mut pgtk_display_info
{
  let mut dpyinfo: *mut pgtk_display_info;

  CHECK_STRING (name);

  if !NILP (is_wayland_display (name))
    {
      dpyinfo = x_display_list;
      while !dpyinfo.is_null ()
	{
	  if !NILP (is_wayland_display (XCAR ((*dpyinfo).name_list_element)))
	    { return dpyinfo; }
	  dpyinfo = (*dpyinfo).next;
	}
    }
  else
    {
      dpyinfo = x_display_list;
      while !dpyinfo.is_null ()
	{
	  if !NILP (Fstring_equal (XCAR ((*dpyinfo).name_list_element), name))
	    { return dpyinfo; }
	  dpyinfo = (*dpyinfo).next;
	}
    }

  /* Use this general default value to start with.  */
  Vx_resource_name = Vinvocation_name;

  validate_x_resource_name ();

  dpyinfo = pgtk_term_init (name, SSDATA (Vx_resource_name));

  if dpyinfo.is_null ()
    { error (c"Cannot connect to display server %s".as_ptr (), SDATA (name)); }

  dpyinfo
}

/* ==========================================================================

    Frame parameter setters

   ========================================================================== */

/// C: C 173-174: `static void pgtk_set_foreground_color (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_foreground_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut fg: c_ulong;
  let mut old_fg: c_ulong;

  block_input ();
  old_fg = FRAME_FOREGROUND_COLOR! (f);
  fg = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as c_ulong;
  FRAME_FOREGROUND_PIXEL! (f, fg);
  (*FRAME_X_OUTPUT! (f)).foreground_color = fg;

  if !FRAME_GTK_WIDGET! (f).is_null ()
    {
      if (*FRAME_X_OUTPUT! (f)).cursor_color == old_fg
	{
	  (*FRAME_X_OUTPUT! (f)).cursor_color = fg;
	  (*FRAME_X_OUTPUT! (f)).cursor_xgcv.background = fg;
	}

      update_face_from_frame_parameter (f, Qforeground_color (), arg);
      if FRAME_VISIBLE_P (f)
	{ SET_FRAME_GARBAGED! (f); }
    }
  unblock_input ();
}

/// C: C 200-201: `static void pgtk_set_background_color (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_background_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut bg: c_ulong;

  block_input ();
  bg = x_decode_color (f, arg, WHITE_PIX_DEFAULT! (f)) as c_ulong;
  FRAME_BACKGROUND_PIXEL! (f, bg);

  /* Clear the frame.  */
  if FRAME_VISIBLE_P (f)
    { pgtk_clear_frame (f); }

  (*FRAME_X_OUTPUT! (f)).background_color = bg;
  (*FRAME_X_OUTPUT! (f)).cursor_xgcv.foreground = bg;

  xg_set_background_color (f, bg);
  update_face_from_frame_parameter (f, Qbackground_color (), arg);

  if FRAME_VISIBLE_P (f)
    { SET_FRAME_GARBAGED! (f); }
  unblock_input ();
}

/// C: C 224-225: `static void pgtk_set_alpha_background (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_alpha_background (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  gui_set_alpha_background (f, arg, oldval);

  /* This prevents GTK from painting the window's background, which
     interferes with transparent background in some environments */

  gtk_widget_set_app_paintable (FRAME_GTK_OUTER_WIDGET! (f),
				((*f).alpha_background != 1.0) as gboolean);

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      && gtk_widget_get_realized (FRAME_GTK_OUTER_WIDGET! (f)) != 0
      && (*f).alpha_background != 1.0
    { gdk_window_set_opaque_region (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f)),
				    std::ptr::null_mut ()); }
}

/// C: C 242-243: `static void pgtk_set_border_color (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_border_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut pix: c_int;

  CHECK_STRING (arg);
  pix = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));
  (*FRAME_X_OUTPUT! (f)).border_pixel = pix;
  pgtk_frame_rehighlight (FRAME_DISPLAY_INFO! (f));
}

/// C: C 253-254: `static void pgtk_set_cursor_color (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_cursor_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut fore_pixel: c_ulong;
  let mut pixel: c_ulong;
  let x: *mut pgtk_output = (*f).output_data.pgtk as *mut pgtk_output;

  if !NILP (Vx_cursor_fore_pixel)
    {
      fore_pixel = x_decode_color (f, Vx_cursor_fore_pixel,
				   WHITE_PIX_DEFAULT! (f)) as c_ulong;
    }
  else
    { fore_pixel = FRAME_BACKGROUND_PIXEL (f); }

  pixel = x_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as c_ulong;

  /* Make sure that the cursor color differs from the background color.  */
  if pixel == FRAME_BACKGROUND_PIXEL (f)
    {
      pixel = (*x).mouse_color;
      if pixel == fore_pixel
	{
	  fore_pixel = FRAME_BACKGROUND_PIXEL (f);
	}
    }

  (*x).cursor_foreground_color = fore_pixel;
  (*x).cursor_color = pixel;

  if !FRAME_X_WINDOW! (f).is_null ()
    {
      (*x).cursor_xgcv.background = (*x).cursor_color;
      (*x).cursor_xgcv.foreground = fore_pixel;

      if FRAME_VISIBLE_P (f)
	{
	  gui_update_cursor (f, false);
	  gui_update_cursor (f, true);
	}
    }

  update_face_from_frame_parameter (f, Qcursor_color (), arg);
}

/// C: C 297-298: `static void pgtk_set_name_internal (struct frame *f, Lisp_Object name)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_name_internal (f: *mut frame, name: Lisp_Object)
{
  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    {
      block_input ();
      {
	let encoded_name: Lisp_Object;

	/* As ENCODE_UTF_8 may cause GC and relocation of string data,
	   we use it before x_encode_text that may return string data.  */
	encoded_name = ENCODE_UTF_8 (name);

	gtk_window_set_title (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
			      SSDATA (encoded_name));
      }
      unblock_input ();
    }
}

/// C: C 317-318: `static void pgtk_set_name (struct frame *f, Lisp_Object name, int explicit)`
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_set_name (f: *mut frame, name: Lisp_Object, explicit: c_int)
{
  /* C's `name` is reassigned below; the port rebinds it (`src/term.rs`'s shape).  */
  let mut name = name;

  /* Make sure that requests from lisp code override requests from
     Emacs redisplay code.  */
  if explicit != 0
    {
      /* If we're switching from explicit to implicit, we had better
         update the mode lines and thereby update the title.  */
      if explicit_name! (f) && NILP (name)
	{ update_mode_lines = 12; }

      explicit_name! (f, !NILP (name));
    }
  else if explicit_name! (f)
    { return; }

  if NILP (name)
    { name = build_string (pgtk_app_name); }
  else
    { CHECK_STRING (name); }

  /* Don't change the name if it's already NAME.  */
  if !NILP (Fstring_equal (name, (*f).name))
    { return; }

  fset_name (f, name);

  /* Title overrides explicit name.  */
  if !NILP ((*f).title)
    { name = (*f).title; }

  pgtk_set_name_internal (f, name);
}

/* This function should be called when the user's lisp code has
   specified a name for the frame; the name will override any set by the
   redisplay code.  */
/// C: C 356-357: `static void pgtk_explicitly_set_name (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_explicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  pgtk_set_name (f, arg, 1);
}

/* This function should be called by Emacs redisplay code to set the
   name; names set this way will never override names set by the user's
   lisp code.  */
/// C: C 367: `void pgtk_implicitly_set_name (struct frame *f, Lisp_Object arg, Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_implicitly_set_name (f: *mut frame, arg: Lisp_Object,
                                                   oldval: Lisp_Object)
{
  pgtk_set_name (f, arg, 0);
}

/* Change the title of frame F to NAME.
   If NAME is nil, use the frame name as the title.  */

/// C: C 378-379: `static void pgtk_set_title (struct frame *f, Lisp_Object name, Lisp_Object
/// old_name)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_title (f: *mut frame, name: Lisp_Object, old_name: Lisp_Object)
{
  /* C's `name` is reassigned below (the rebinding `pgtk_set_name` uses).  */
  let mut name = name;

  /* Don't change the title if it's already NAME.  */
  if EQ (name, (*f).title)
    { return; }

  update_mode_lines = 22;

  fset_title (f, name);

  if NILP (name)
    { name = (*f).name; }
  else
    { CHECK_STRING (name); }

  pgtk_set_name_internal (f, name);
}

/// C: C 397-398: `static void pgtk_set_menu_bar_lines (struct frame *f, Lisp_Object value,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let nlines: c_int;
  /* Right now, menu bars don't work properly in minibuf-only frames;
     most of the commands try to apply themselves to the minibuffer
     frame itself, and get an error because you can't switch buffers
     in or split the minibuffer window.  */
  if FRAME_MINIBUF_ONLY_P (f) || !FRAME_PARENT_FRAME (f).is_null ()
    { return; }

  if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNUM (value) as c_int; }
  else
    { nlines = 0; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  FRAME_MENU_BAR_LINES! (f, 0);
  FRAME_MENU_BAR_HEIGHT! (f, 0);
  if nlines != 0
    {
      FRAME_EXTERNAL_MENU_BAR! (f, 1);
      if FRAME_PGTK_P (f) && (*((*f).output_data.pgtk as *mut pgtk_output)).menubar_widget == 0
	/* Make sure next redisplay shows the menu bar.  */
	{ wset_update_mode_line (XWINDOW (FRAME_SELECTED_WINDOW (f))); }
    }
  else
    {
      if FRAME_EXTERNAL_MENU_BAR (f) == true
	{ free_frame_menubar (f); }
      FRAME_EXTERNAL_MENU_BAR! (f, 0);
      if FRAME_X_P (f)
	{ (*((*f).output_data.pgtk as *mut pgtk_output)).menubar_widget = 0; }
    }

  adjust_frame_glyphs (f);
}

/* Set the number of lines used for the tab bar of frame F to VALUE.
   VALUE not an integer, or < 0 means set the lines to zero.  OLDVAL
   is the old number of tab bar lines.  This function changes the
   height of all windows on frame F to match the new tab bar height.
   The frame's height doesn't change.  */

/// C: C 443-444: `static void pgtk_set_tab_bar_lines (struct frame *f, Lisp_Object value,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let nlines: c_int;

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  pgtk_change_tab_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

/* Set the pixel height of the tab bar of frame F to HEIGHT.  */
/// C: C 462: `void pgtk_change_tab_bar_height (struct frame *f, int height)`
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_change_tab_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TAB_BAR_HEIGHT (f);

  /* This differs from the tool bar code in that the tab bar height is
     not rounded up.  Otherwise, if redisplay_tab_bar decides to grow
     the tab bar by even 1 pixel, FRAME_TAB_BAR_LINES will be changed,
     leading to the tab bar height being incorrectly set upon the next
     call to x_set_font.  (bug#59285) */
  let lines: c_int = height / unit;

  /* Even so, HEIGHT might be less than unit if the tab bar face is
     not so tall as the frame's font height; which if true lines will
     be set to 0 and the tab bar will thus vanish.  */

  let lines: c_int = if lines == 0 && height != 0 { 1 } else { lines };

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f as *mut c_void);

  /* Recalculate tab bar and frame text sizes.  */
  FRAME_TAB_BAR_HEIGHT! (f, height);
  FRAME_TAB_BAR_LINES! (f, lines);
  store_frame_param (f, Qtab_bar_lines (), make_fixnum (lines as EMACS_INT));

  if !FRAME_X_WINDOW! (f).is_null () && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*XWINDOW ((*f).tab_bar_window)).current_matrix); }

  if !tab_bar_resized! (f)
    {
      let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

      /* As long as tab_bar_resized is false, effectively try to change
        F's native height.  */
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

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
  if !FRAME_X_WINDOW! (f).is_null ()
    { pgtk_clear_under_internal_border (f); }
}

/* Set the pixel height of the tool bar of frame F to HEIGHT.  */
/// C: C 526-527: `static void x_change_tool_bar_height (struct frame *f, int height)`
#[cfg (feature = "have_pgtk")]
unsafe fn x_change_tool_bar_height (f: *mut frame, height: c_int)
{
  FRAME_TOOL_BAR_LINES! (f, 0);
  FRAME_TOOL_BAR_HEIGHT! (f, 0);
  if height != 0
    {
      FRAME_EXTERNAL_TOOL_BAR! (f, true);
      if FRAME_X_P (f) && (*((*f).output_data.pgtk as *mut pgtk_output)).toolbar_widget == 0
	/* Make sure next redisplay shows the tool bar.  */
	{ wset_update_mode_line (XWINDOW (FRAME_SELECTED_WINDOW (f))); }
      update_frame_tool_bar (f);
    }
  else
    {
      if FRAME_EXTERNAL_TOOL_BAR (f)
	{ free_frame_tool_bar (f); }
      FRAME_EXTERNAL_TOOL_BAR! (f, false);
      /* Make sure implied resizing of frames without initial tool bar
	 can be inhibited too.  */
      tool_bar_resized! (f, true);
    }
}

/* Toolbar support.  */
/// C: C 551-552: `static void pgtk_set_tool_bar_lines (struct frame *f, Lisp_Object value,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let nlines: c_int;

  /* Treat tool bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    {
      /* Make sure implied resizing can be inhibited for minibuffer-only
	 frames too.  */
      tool_bar_resized! (f, true);

      return;
    }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  x_change_tool_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

/// C: C 576-577: `static void pgtk_set_child_frame_border_width (struct frame *f, Lisp_Object
/// arg, Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
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

      if !FRAME_GTK_WIDGET! (f).is_null ()
	{
	  adjust_frame_size (f, -1, -1, 3,
			     false, Qchild_frame_border_width ());
	  pgtk_clear_under_internal_border (f);
	}
    }
}

/// C: C 601-602: `static void pgtk_set_internal_border_width (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_internal_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let border: c_int = check_int_nonnegative (arg);

  if border != FRAME_INTERNAL_BORDER_WIDTH (f)
    {
      (*f).internal_border_width = border;

      if !FRAME_X_WINDOW! (f).is_null ()
	{
	  adjust_frame_size (f, -1, -1, 3, false, Qinternal_border_width ());
	  pgtk_clear_under_internal_border (f);
	}
    }
}

/// C: C 619-620: `static void pgtk_set_icon_type (struct frame *f, Lisp_Object arg, Lisp_Object
/// oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_icon_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let result: bool;

  if STRINGP (arg)
    {
      if STRINGP (oldval) && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
	{ return; }
    }
  else if !STRINGP (oldval) && NILP (oldval) == NILP (arg)
    { return; }

  block_input ();
  if NILP (arg)
    { result = pgtk_text_icon (f,
			       SSDATA (if !NILP ((*f).icon_name)
				       { (*f).icon_name } else { (*f).name })); }
  else
    { result = (*FRAME_TERMINAL! (f)).set_bitmap_icon_hook.unwrap () (f as *mut c_void, arg); }

  if result
    {
      unblock_input ();
      error (c"No icon window available".as_ptr ());
    }

  unblock_input ();
}

/// C: C 649-650: `static void pgtk_set_icon_name (struct frame *f, Lisp_Object arg, Lisp_Object
/// oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_icon_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let result: bool;

  if STRINGP (arg)
    {
      if STRINGP (oldval) && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
	{ return; }
    }
  else if !NILP (arg) || NILP (oldval)
    { return; }

  fset_icon_name (f, arg);

  block_input ();

  result = pgtk_text_icon (f,
			   SSDATA (if !NILP ((*f).icon_name)
				   { (*f).icon_name }
				   else if !NILP ((*f).title)
				   { (*f).title } else { (*f).name }));

  if result
    {
      unblock_input ();
      error (c"No icon window available".as_ptr ());
    }

  unblock_input ();
}

/// C: C 681-682: `static void pgtk_set_cursor_type (struct frame *f, Lisp_Object arg, Lisp_Object
/// oldval)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_cursor_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);
}

/// C: C 687-688: `static void pgtk_set_mouse_color (struct frame *f, Lisp_Object arg,
/// Lisp_Object oldval)` - C's own body is empty (C 689-690 are the braces) and the port keeps it
/// empty: this is the reference's text, not a stub the port introduced (a recorded deviation).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_mouse_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
}

/// C: C 692-693: `static void pgtk_set_undecorated (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_undecorated (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      FRAME_UNDECORATED! (f, if NILP (new_value) { false } else { true });
      xg_set_undecorated (f, new_value);
    }
}

/// C: C 703-704: `static void pgtk_set_skip_taskbar (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_skip_taskbar (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      xg_set_skip_taskbar (f, new_value);
      FRAME_SKIP_TASKBAR! (f, !NILP (new_value));
    }
}

/// C: C 714-715: `static void pgtk_set_override_redirect (struct frame *f, Lisp_Object
/// new_value, Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_override_redirect (f: *mut frame, new_value: Lisp_Object,
                                      old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    {
      /* Here (xfwm) override_redirect can be changed for invisible
         frames only.  */
      pgtk_make_frame_invisible (f);

      xg_set_override_redirect (f, new_value);

      pgtk_make_frame_visible (f);
      FRAME_OVERRIDE_REDIRECT! (f, !NILP (new_value));
    }
}

/* Set icon from FILE for frame F.  */
/// C: C 732: `bool xg_set_icon (struct frame *f, Lisp_Object file)` - C's non-`static` definition
/// (`pgtkterm.h` 672 declares it `extern`).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_icon (f: *mut frame, file: Lisp_Object) -> bool
{
  let mut result = false;
  let found: Lisp_Object;

  if FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { return false; }

  found = image_find_image_file (file);

  if !NILP (found)
    {
      let pixbuf: *mut GdkPixbuf;
      let mut err: *mut GError = std::ptr::null_mut ();
      let filename: *mut c_char = SSDATA (ENCODE_FILE (found));
      block_input ();

      pixbuf = gdk_pixbuf_new_from_file (filename, &raw mut err);

      if !pixbuf.is_null ()
	{
	  gtk_window_set_icon (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
			       pixbuf);
	  g_object_unref (pixbuf as gpointer);

	  result = true;
	}
      else
	{ g_error_free (err); }

      unblock_input ();
    }

  result
}

/// C: C 769: `bool xg_set_icon_from_xpm_data (struct frame *f, const char **data)` - C's
/// non-`static` definition (`pgtkterm.h` 673 declares it `extern`).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_icon_from_xpm_data (f: *mut frame, data: *mut *const c_char)
                                                   -> bool
{
  /* gdk-pixbuf 2.44 deprecated gdk_pixbuf_new_from_xpm_data.
     Emacs should convert assets to PNG and use gdk_pixbuf_new_from_stream
     with a GMemoryInputStream, or transition to PNG via GResource.
     Pacify GCC for now.  */
  /* C 776-781: `#if (defined GDK_PIXBUF_VERSION_2_44 && GDK_PIXBUF_VERSION_2_44 <=
     GDK_PIXBUF_VERSION_MIN_REQUIRED && (GNUC_PREREQ (4, 6, 0) || defined __clang__))` around two
     `# pragma GCC diagnostic` lines.  A pragma has no Rust spelling and the condition is decided
     false by the frozen table (docs/baseline/config/cond-eval-verified.txt), so C's text is kept
     and no arm is written (the `window.rs` 14 / `timefns.rs` 79 reading).  */

  let pixbuf: *mut GdkPixbuf = gdk_pixbuf_new_from_xpm_data (data);

  /* C 785-789: the same `#if` around the `# pragma GCC diagnostic pop`.  */

  if pixbuf.is_null ()
    { return false; }

  if FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { return false; }

  gtk_window_set_icon (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), pixbuf);
  g_object_unref (pixbuf as gpointer);
  true
}

/// C: C 802-803: `static void pgtk_set_sticky (struct frame *f, Lisp_Object new_value,
/// Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_sticky (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { return; }

  if !NILP (new_value)
    { gtk_window_stick (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f))); }
  else
    { gtk_window_unstick (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f))); }
}

/// C: C 815-816: `static void pgtk_set_tool_bar_position (struct frame *f, Lisp_Object
/// new_value, Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_tool_bar_position (f: *mut frame, new_value: Lisp_Object,
                                      old_value: Lisp_Object)
{
  let choice: Lisp_Object = list4 (Qleft (), Qright (), Qtop (), Qbottom ());

  if !NILP (Fmemq (new_value, choice))
    {
      if !EQ (new_value, old_value)
	{
	  xg_change_toolbar_position (f, new_value);
	  fset_tool_bar_position (f, new_value);
	}
    }
  else
    { wrong_choice (choice, new_value); }
}

/// C: C 833-834: `static void pgtk_set_scroll_bar_foreground (struct frame *f, Lisp_Object
/// new_value, Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_scroll_bar_foreground (f: *mut frame, new_value: Lisp_Object,
                                          old_value: Lisp_Object)
{
  let css_provider: *mut GtkCssProvider =
    (*FRAME_X_OUTPUT! (f)).scrollbar_foreground_css_provider;

  if FRAME_TOOLTIP_P (f)
    { return; }

  if NILP (new_value)
    {
      gtk_css_provider_load_from_data (css_provider, c"".as_ptr (), -1,
				       std::ptr::null_mut ());
      update_face_from_frame_parameter (f, Qscroll_bar_foreground (), new_value);
    }
  else if STRINGP (new_value)
    {
      let mut rgb: Emacs_Color = std::mem::zeroed ();

      if pgtk_parse_color (f, SSDATA (new_value), &raw mut rgb) == 0
	{ error (c"Unknown color".as_ptr ()); }

      let mut css: [c_char; 64] = [0; 64];
      sprintf (css.as_mut_ptr (), c"scrollbar slider { background-color: #%06x; }".as_ptr (),
	       (rgb.pixel as c_uint) & 0xffffff);
      gtk_css_provider_load_from_data (css_provider, css.as_mut_ptr (), -1,
				       std::ptr::null_mut ());
      update_face_from_frame_parameter (f, Qscroll_bar_foreground (), new_value);

    }
  else
    { error (c"Invalid scroll-bar-foreground".as_ptr ()); }
}

/// C: C 866-867: `static void pgtk_set_scroll_bar_background (struct frame *f, Lisp_Object
/// new_value, Lisp_Object old_value)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_set_scroll_bar_background (f: *mut frame, new_value: Lisp_Object,
                                          old_value: Lisp_Object)
{
  let css_provider: *mut GtkCssProvider =
    (*FRAME_X_OUTPUT! (f)).scrollbar_background_css_provider;

  if NILP (new_value)
    {
      gtk_css_provider_load_from_data (css_provider, c"".as_ptr (), -1,
				       std::ptr::null_mut ());
      update_face_from_frame_parameter (f, Qscroll_bar_background (), new_value);
    }
  else if STRINGP (new_value)
    {
      let mut rgb: Emacs_Color = std::mem::zeroed ();

      if pgtk_parse_color (f, SSDATA (new_value), &raw mut rgb) == 0
	{ error (c"Unknown color".as_ptr ()); }

      /* On pgtk, this frame parameter should be ignored, and honor
	 gtk theme.  (It honors the GTK theme if not explicitly set, so
	 I see no harm in letting users tinker a bit more.)  */
      let mut css: [c_char; 64] = [0; 64];
      sprintf (css.as_mut_ptr (), c"scrollbar trough { background-color: #%06x; }".as_ptr (),
	       (rgb.pixel as c_uint) & 0xffffff);
      gtk_css_provider_load_from_data (css_provider, css.as_mut_ptr (), -1,
				       std::ptr::null_mut ());
      update_face_from_frame_parameter (f, Qscroll_bar_background (), new_value);

    }
  else
    { error (c"Invalid scroll-bar-background".as_ptr ()); }
}

/***********************************************************************
			       Printing
 ***********************************************************************/


#[cfg (feature = "have_pgtk")]
DEFUN! ("x-export-frames", Fx_export_frames, Sx_export_frames, 0, 2, 0,
       doc: "SKIP: real doc in xfns.c.  ",                     // C 905-907
       (frames: Lisp_Object, r#type: Lisp_Object),
       {
  /* C's `frames` is reassigned on the next line; the port rebinds it.  */
  let mut frames = frames;
  let mut rest: Lisp_Object;
  let mut tmp: Lisp_Object;
  let surface_type: cairo_surface_type_t;

  if !CONSP (frames)
    { frames = list1 (frames); }

  tmp = Qnil ();
  /* C 916-925's `for (rest = frames; CONSP (rest); rest = XCDR (rest))`: init, test, body, then the
     increment C writes in the loop head.  */
  rest = frames;
  while CONSP (rest)
    {
      let f: *mut frame = decode_window_system_frame (XCAR (rest));
      let mut frame: Lisp_Object;

      XSETFRAME! (frame, f);
      if !FRAME_VISIBLE_P (f)
	{ error (c"Frames to be exported must be visible".as_ptr ()); }
      tmp = Fcons (frame, tmp);

      rest = XCDR (rest);
    }
  frames = Fnreverse (tmp);

  /* C 928-956: four `#ifdef`s, each wrapping one `if (…) …; else`, so the statement C compiles is
     a chain of exactly the arms whose cairo feature is defined, ending in C 957's `error`.  Rust
     cannot leave the `else` of one `#[cfg]` arm to be supplied by the next arm, so each arm's
     *presence* is the compile-time `cfg!` of its feature and C's chain follows once, in C's
     order - the arm-chooser idiom `docs/ports/gtkutil.md`'s Deviation 4 records for
     `xg_set_geometry` (`take_body`).  A disabled arm takes C's `false`, so the run-time chain and
     the compiled text are C's for every enablement of `CAIRO_HAS_PDF_SURFACE` (C 928),
     `CAIRO_HAS_PNG_FUNCTIONS` (C 933), `CAIRO_HAS_PS_SURFACE` (C 942) and `CAIRO_HAS_SVG_SURFACE`
     (C 947).  */
  if cfg! (feature = "cairo_has_pdf_surface") && (NILP (r#type) || EQ (r#type, Qpdf ()))
    { surface_type = CAIRO_SURFACE_TYPE_PDF; }
  else if cfg! (feature = "cairo_has_png_functions") && EQ (r#type, Qpng ())
    {
      if !NILP (XCDR (frames))
	{ error (c"PNG export cannot handle multiple frames".as_ptr ()); }
      surface_type = CAIRO_SURFACE_TYPE_IMAGE;
    }
  else if cfg! (feature = "cairo_has_ps_surface") && EQ (r#type, Qpostscript ())
    { surface_type = CAIRO_SURFACE_TYPE_PS; }
  else if cfg! (feature = "cairo_has_svg_surface") && EQ (r#type, Qsvg ())
    {
      /* For now, we stick to SVG 1.1.  */
      if !NILP (XCDR (frames))
	{ error (c"SVG export cannot handle multiple frames".as_ptr ()); }
      surface_type = CAIRO_SURFACE_TYPE_SVG;
    }
  else
    { error (c"Unsupported export type".as_ptr ()); }

  pgtk_cr_export_frames (frames, surface_type)
});

/* C 963's `extern frame_parm_handler pgtk_frame_parm_handlers[];` is C's redundant re-declaration
   of the object C defines on the next line (a C idiom for an array of function pointers); Rust has
   one item per name (E0428 otherwise), so the definition is written and the re-declaration is
   recorded here (PORTING.md 0.1-12's forward-declaration class).  */
/// C: C 963-1015: `frame_parm_handler pgtk_frame_parm_handlers[] = …`
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub static mut pgtk_frame_parm_handlers: [frame_parm_handler; 51] =
  [
    Some (gui_set_autoraise),			/* generic OK */
    Some (gui_set_autolower),			/* generic OK */
    Some (pgtk_set_background_color),
    Some (pgtk_set_border_color),
    Some (gui_set_border_width),
    Some (pgtk_set_cursor_color),
    Some (pgtk_set_cursor_type),
    Some (gui_set_font),			/* generic OK */
    Some (pgtk_set_foreground_color),
    Some (pgtk_set_icon_name),
    Some (pgtk_set_icon_type),
    Some (pgtk_set_child_frame_border_width),
    Some (pgtk_set_internal_border_width),	/* generic OK */
    Some (gui_set_right_divider_width),
    Some (gui_set_bottom_divider_width),
    Some (pgtk_set_menu_bar_lines),
    Some (pgtk_set_mouse_color),
    Some (pgtk_explicitly_set_name),
    Some (gui_set_scroll_bar_width),		/* generic OK */
    Some (gui_set_scroll_bar_height),		/* generic OK */
    Some (pgtk_set_title),
    Some (gui_set_unsplittable),		/* generic OK */
    Some (gui_set_vertical_scroll_bars),	/* generic OK */
    Some (gui_set_horizontal_scroll_bars),	/* generic OK */
    Some (gui_set_visibility),			/* generic OK */
    Some (pgtk_set_tab_bar_lines),
    Some (pgtk_set_tool_bar_lines),
    Some (pgtk_set_scroll_bar_foreground),
    Some (pgtk_set_scroll_bar_background),
    Some (gui_set_screen_gamma),		/* generic OK */
    Some (gui_set_line_spacing),		/* generic OK, sets f->extra_line_spacing to int */
    Some (gui_set_left_fringe),			/* generic OK */
    Some (gui_set_right_fringe),		/* generic OK */
    None,					// C 998: `0,`
    Some (gui_set_fullscreen),			/* generic OK */
    Some (gui_set_font_backend),		/* generic OK */
    Some (gui_set_alpha),
    Some (pgtk_set_sticky),
    Some (pgtk_set_tool_bar_position),
    None,					// C 1004: `0,`
    Some (pgtk_set_undecorated),
    Some (pgtk_set_parent_frame),
    Some (pgtk_set_skip_taskbar),
    Some (pgtk_set_no_focus_on_map),
    Some (pgtk_set_no_accept_focus),
    Some (pgtk_set_z_group),
    Some (pgtk_set_override_redirect),
    Some (gui_set_no_special_glyphs),
    Some (pgtk_set_alpha_background),
    Some (gui_set_borders_respect_alpha_background),
    None,					// C 1015: `NULL,`
  ];

/* Handler for signals raised during x_create_frame and
   x_create_tip_frame.  FRAME is the frame which is partially
   constructed.  */

/// C: C 1023-1024: `static Lisp_Object unwind_create_frame (Lisp_Object frame)`
#[cfg (feature = "have_pgtk")]
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
      pgtk_free_frame_resources (f);
      free_glyphs (f);
      return Qt ();
    }

  Qnil ()
}

/// C: C 1045-1046: `static void do_unwind_create_frame (Lisp_Object frame)` - C's `static void`,
/// spelled `unsafe extern "C" fn` because C 1327 passes it to `record_unwind_protect`, whose
/// parameter is `Option<unsafe extern "C" fn (Lisp_Object)>` (`include/lisp.rs` 4063); a Rust fn
/// item is the Rust ABI and does not unify (`src/androidmenu.rs`'s `android_unwind_local_frame`
/// is the precedent, PORTING.md 1.4).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn do_unwind_create_frame (frame: Lisp_Object)
{
  unwind_create_frame (frame);
}

/* Return the pixel color value for color COLOR_NAME on frame F.  If F
   is a monochrome frame, return MONO_COLOR regardless of what ARG says.
   Signal an error if color can't be allocated.  */

/// C: C 1055-1056: `static int x_decode_color (struct frame *f, Lisp_Object color_name, int
/// mono_color)`
#[cfg (feature = "have_pgtk")]
unsafe fn x_decode_color (f: *mut frame, color_name: Lisp_Object, mono_color: c_int) -> c_int
{
  let mut cdef: Emacs_Color = std::mem::zeroed ();

  CHECK_STRING (color_name);

  /* Return MONO_COLOR for monochrome frames.  */
  if (*FRAME_DISPLAY_INFO! (f)).n_planes == 1
    { return mono_color; }

  /* x_defined_color is responsible for coping with failures
     by looking for a near-miss.  */
  if pgtk_defined_color (f, SSDATA (color_name), &raw mut cdef, true, 0)
    { return cdef.pixel as c_int; }

  signal_error (c"Undefined color".as_ptr (), color_name);
}

/// C: C 1074-1075: `void pgtk_default_font_parameter (struct frame *f, Lisp_Object parms)`
/// - C's non-`static` definition (`pgtkterm.h`/`xterm.h` declare it).
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_default_font_parameter (f: *mut frame, parms: Lisp_Object)
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  let mut font_param: Lisp_Object =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qfont (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_STRING);
  let mut font: Lisp_Object = Qnil ();
  if BASE_EQ (font_param, Qunbound ())
    { font_param = Qnil (); }

  if NILP (font_param)
    {
      /* System font should take precedence over X resources.  We suggest this
         regardless of font-use-system-font because .emacs may not have been
         read yet.  */
      let system_font: *const c_char = xsettings_get_system_font ();
      if !system_font.is_null ()
	{ font = font_open_by_name (f, build_unibyte_string (system_font)); }
    }

  if NILP (font)
    { font = if !NILP (font_param) { font_param }
      else { gui_display_get_arg (dpyinfo as *mut c_void, parms, Qfont (), c"font".as_ptr (),
				   c"Font".as_ptr (), RES_TYPE_STRING) }; }

  if !FONTP (font) && !STRINGP (font)
    {
      let names: [*const c_char; 8] = [
	c"monospace-10".as_ptr (),
	c"-adobe-courier-medium-r-*-*-*-120-*-*-*-*-iso8859-1".as_ptr (),
	c"-misc-fixed-medium-r-normal-*-*-140-*-*-c-*-iso8859-1".as_ptr (),
	c"-*-*-medium-r-normal-*-*-140-*-*-c-*-iso8859-1".as_ptr (),
	/* This was formerly the first thing tried, but it finds
	   too many fonts and takes too long.  */
	c"-*-*-medium-r-*-*-*-*-*-*-c-*-iso8859-1".as_ptr (),
	/* If those didn't work, look for something which will
	   at least work.  */
	c"-*-fixed-*-*-*-*-*-140-*-*-c-*-iso8859-1".as_ptr (),
	c"fixed".as_ptr (),
	std::ptr::null (),
      ];
      let mut i: c_int;

      /* C 1120-1127's `for (i = 0; names[i]; i++) { … }`: the test is the pointer itself.  */
      i = 0;
      while !names[i as usize].is_null ()
	{
	  font = font_open_by_name (f, build_unibyte_string (names[i as usize]));
	  if !NILP (font)
	    { break; }
	  i += 1;
	}
      if NILP (font)
	{ error (c"No suitable font was found".as_ptr ()); }
    }

  /* This call will make X resources override any system font setting.  */
  gui_default_parameter (f, parms, Qfont (), font, c"font".as_ptr (), c"Font".as_ptr (),
			 RES_TYPE_STRING);
}

/// C: C 1133-1134: `static void update_watched_scale_factor (struct atimer *timer)`
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn update_watched_scale_factor (timer: *mut atimer)
{
  let f: *mut frame = (*timer).client_data as *mut frame;
  let scale_factor: c_double = FRAME_SCALE_FACTOR (f);

  if scale_factor != (*FRAME_X_OUTPUT! (f)).watched_scale_factor
    {
      (*FRAME_X_OUTPUT! (f)).watched_scale_factor = scale_factor;
      pgtk_cr_update_surface_desired_size (f,
					   FRAME_CR_SURFACE_DESIRED_WIDTH! (f),
					   FRAME_CR_SURFACE_DESIRED_HEIGHT! (f),
					   true);
    }
}


/* ==========================================================================

    Lisp definitions

   ========================================================================== */

#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-set-monitor-scale-factor", Fpgtk_set_monitor_scale_factor,   // C 1155-1196
       Spgtk_set_monitor_scale_factor, 2, 2, 0,
       doc: "Set monitor MONITOR-MODEL's scale factor to SCALE-FACTOR.\nSince Gdk's scale factor is integer, physical pixel width/height is\nincorrect when you specify fractional scale factor in compositor.\nIf you set scale factor by this function, it is used instead of Gdk's one.\n\nPass nil as SCALE-FACTOR if you want to reset the specified monitor's\nscale factor.",
       (monitor_model: Lisp_Object, scale_factor: Lisp_Object),
       {
  CHECK_STRING (monitor_model);
  if !NILP (scale_factor)
    {
      CHECK_NUMBER (scale_factor);
      if FIXNUMP (scale_factor)
	{
	  if XFIXNUM (scale_factor) <= 0
	    { error (c"Scale factor must be > 0".as_ptr ()); }	// C 1168
	}
      else if FLOATP (scale_factor)
	{
	  if XFLOAT_DATA (scale_factor) <= 0.0
	    { error (c"Scale factor must be > 0".as_ptr ()); }	// C 1173
	}
      else
	{ error (c"Unknown type of scale-factor".as_ptr ()); }	// C 1177
    }

  /* C's `Lisp_Object tem = Fassoc (monitor_model, monitor_scale_factor_alist, Qnil);` - the
     initialiser keeps C's form.  */
  let tem: Lisp_Object = Fassoc (monitor_model, monitor_scale_factor_alist, Qnil ());
  if NILP (tem)
    {
      if !NILP (scale_factor)
	{ monitor_scale_factor_alist = Fcons (Fcons (monitor_model, scale_factor),
					      monitor_scale_factor_alist); }	// C 1186-1187
    }
  else
    { Fsetcdr (tem, scale_factor); }	// C 1190

  return scale_factor;
});

/* ==========================================================================

   C 1197-1757: `x-create-frame` - the file's first `DEFUN` and its largest item

   ========================================================================== */

/* C 1197-1200: `DEFUN ("x-create-frame", Fx_create_frame, Sx_create_frame, 1, 1, 0,` with C's own
   short docstring (`doc: /* SKIP: real doc in xfns.c.  */`; the real one is `src/xfns.c` 1173's,
   and the docstring gate compares this text with the reference byte for byte).  C 1201-1216's
   declarations keep C's types and order: C's `f`/`frame`/`tem`/`name`/`parms` are assigned along
   the way, so the port declares them and assigns where C does (the `src/androidfns.rs` 1085-1103
   shape for this same function).  A `DEFUN!` invocation takes no doc comment (the `unused doc
   comment` the second slice's first run read), so the citation is a block comment.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-create-frame", Fx_create_frame, Sx_create_frame,   // C 1197-1757
	1, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(parms: Lisp_Object),
	{
  let mut f: *mut frame;                        // C 1205: `struct frame *f;`
  let mut frame: Lisp_Object;                   // C 1206: `Lisp_Object frame, tem;`
  let mut tem: Lisp_Object;
  let mut name: Lisp_Object;                    // C 1207: `Lisp_Object name;`
  let mut minibuffer_only: bool;                // C 1208: `bool minibuffer_only = false;`
  let mut undecorated: bool;                    // C 1209: `bool undecorated = false,
  let mut override_redirect: bool;              //         override_redirect = false;`
  let mut window_prompting: c_long;             // C 1210: `long window_prompting = 0;`
  let count: specpdl_ref;                       // C 1211: `specpdl_ref count = SPECPDL_INDEX ();`
  let mut display: Lisp_Object;                 // C 1212: `Lisp_Object display;`
  let mut dpyinfo: *mut pgtk_display_info;      // C 1213: `struct pgtk_display_info *dpyinfo = NULL;`
  let mut parent: Lisp_Object;                  // C 1214: `Lisp_Object parent, parent_frame;`
  let mut parent_frame: Lisp_Object;
  let mut kb: *mut kboard;                      // C 1215: `struct kboard *kb;`
  let mut parms = parms;                        // C 1216: `parms = Fcopy_alist (parms);`

  /* C 1208-1213's initialisers, in C's own order.  */
  minibuffer_only = false;
  undecorated = false;
  override_redirect = false;
  window_prompting = 0;
  count = SPECPDL_INDEX ();
  dpyinfo = std::ptr::null_mut ();

  parms = Fcopy_alist (parms);

  /* Use this general default value to start with
     until we know if this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name;

  display =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qterminal (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_NUMBER);
  if BASE_EQ (display, Qunbound ())
    { display =
	gui_display_get_arg (dpyinfo as *mut c_void, parms, Qdisplay (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_STRING); }
  if BASE_EQ (display, Qunbound ())
    { display = Qnil (); }
  dpyinfo = check_pgtk_display_info (display);
  /* C 1228: `kb = dpyinfo->terminal->kboard;` - `include/termhooks.rs` 220 records the member as
     `*mut c_void` (`struct kboard`'s record is `keyboard.rs`'s, P-72 rule 3), so the port names the
     record (the `src/androidfns.rs` 1079 shape).  */
  kb = (*((*dpyinfo).terminal)).kboard as *mut kboard;

  if (*((*dpyinfo).terminal)).name.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  name =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qname (), c"name".as_ptr (),
			 c"Name".as_ptr (), RES_TYPE_STRING);
  if !STRINGP (name)
      && ! BASE_EQ (name, Qunbound ())
      && ! NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  if STRINGP (name)
    { Vx_resource_name = name; }

  /* See if parent window is specified.  */
  parent =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qparent_id (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_NUMBER);
  if BASE_EQ (parent, Qunbound ())
    { parent = Qnil (); }
  if ! NILP (parent)
    { CHECK_NUMBER (parent); }

  frame = Qnil ();
  tem =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qminibuffer (), c"minibuffer".as_ptr (),
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

  parent_frame =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qparent_frame (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_SYMBOL);
  /* Accept parent-frame iff parent-id was not specified.  */
  if !NILP (parent)
      || BASE_EQ (parent_frame, Qunbound ())
      || NILP (parent_frame)
      || !FRAMEP (parent_frame)
      || !FRAME_LIVE_P (XFRAME (parent_frame))
      || !FRAME_PGTK_P (XFRAME (parent_frame))
    { parent_frame = Qnil (); }

  fset_parent_frame (f, parent_frame);
  store_frame_param (f, Qparent_frame (), parent_frame);

  /* C 838-843's assignment is C's `tem = …` inside `!NILP (…)`; Rust's assignment is a statement,
     so it stands on its own line (the chained-assignment note in `src/frame.rs`; the
     `src/androidfns.rs` 1203 shape).  */
  tem =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qundecorated (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_BOOLEAN);
  if !NILP (tem)
      && ! (BASE_EQ (tem, Qunbound ()))
    { undecorated = true; }

  FRAME_UNDECORATED! (f, undecorated);
  store_frame_param (f, Qundecorated (), if undecorated { Qt () } else { Qnil () });

  tem =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qoverride_redirect (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  if !NILP (tem)
      && ! (BASE_EQ (tem, Qunbound ()))
    { override_redirect = true; }

  FRAME_OVERRIDE_REDIRECT! (f, override_redirect);
  store_frame_param (f, Qoverride_redirect (), if override_redirect { Qt () } else { Qnil () });

  XSETFRAME! (frame, f);

  frame_set_id_from_params (f, parms);

  (*f).terminal = (*dpyinfo).terminal;

  output_method! (f, output_pgtk);
  /* C 1308: `FRAME_X_OUTPUT (f) = xzalloc (sizeof *FRAME_X_OUTPUT (f));` - C's macro is an
     l-value and this unit's `FRAME_X_OUTPUT!` carries the read arm only, so the write is C's own
     member path with the record's size (the `src/haikufns.rs` 1458 shape for `output_data.haiku`;
     C's `sizeof` of the expression is that record's size).  */
  (*f).output_data.pgtk = xzalloc (std::mem::size_of::<pgtk_output> ());
  FRAME_FONTSET! (f, -1);
  (*FRAME_X_OUTPUT! (f)).white_relief.pixel = (-1 as c_long) as c_ulong;
  (*FRAME_X_OUTPUT! (f)).black_relief.pixel = (-1 as c_long) as c_ulong;

  (*FRAME_X_OUTPUT! (f)).scrollbar_foreground_css_provider = gtk_css_provider_new ();
  (*FRAME_X_OUTPUT! (f)).scrollbar_background_css_provider = gtk_css_provider_new ();

  fset_icon_name (f,
		  gui_display_get_arg (dpyinfo as *mut c_void, parms, Qicon_name (),
				       c"iconName".as_ptr (), c"Title".as_ptr (),
				       RES_TYPE_STRING));
  if !STRINGP ((*f).icon_name)
    { fset_icon_name (f, Qnil ()); }

  /* C 1335: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as C 1308.  */
  (*FRAME_X_OUTPUT! (f)).display_info = dpyinfo;

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (Some (do_unwind_create_frame), frame);

  /* These colors will be set anyway later, but it's important
     to get the color reference counts right, so initialize them!  */
  {
    let black: Lisp_Object;

    /* Function x_decode_color can signal an error.  Make
       sure to initialize color slots so that we won't try
       to free colors we haven't allocated.  */
    FRAME_FOREGROUND_PIXEL! (f, (-1 as c_long) as c_ulong);
    FRAME_BACKGROUND_PIXEL! (f, (-1 as c_long) as c_ulong);
    (*FRAME_X_OUTPUT! (f)).cursor_color = (-1 as c_long) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).cursor_foreground_color = (-1 as c_long) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).border_pixel = (-1 as c_long) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).mouse_color = (-1 as c_long) as c_ulong;

    black = build_string (c"black".as_ptr ());
    FRAME_FOREGROUND_PIXEL! (f, x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong);
    FRAME_BACKGROUND_PIXEL! (f, x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong);
    (*FRAME_X_OUTPUT! (f)).cursor_color
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).cursor_foreground_color
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).border_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong;
    (*FRAME_X_OUTPUT! (f)).mouse_color
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong;
  }

  /* Specify the parent under which to make this X window.  */
  if ! NILP (parent)
    {
      (*FRAME_X_OUTPUT! (f)).parent_desc = XFIXNAT (parent) as Window;
      (*FRAME_X_OUTPUT! (f)).explicit_parent = true as c_char;
    }
  else
    {
      (*FRAME_X_OUTPUT! (f)).parent_desc = (*FRAME_DISPLAY_INFO! (f)).root_window;
      (*FRAME_X_OUTPUT! (f)).explicit_parent = false as c_char;
    }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)
    {
      fset_name (f, build_string ((*dpyinfo).x_id_name));
      explicit_name! (f, false);
    }
  else
    {
      fset_name (f, name);
      explicit_name! (f, true);
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);
    }

  register_font_driver (&raw const ftcrfont_driver as *const c_void, f);
  #[cfg (feature = "harfbuzz")]
  { register_font_driver (&raw const ftcrhbfont_driver as *const c_void, f); }

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values
     that are needed to determine window geometry.  */
  pgtk_default_font_parameter (f, parms);
  if FRAME_FONT! (f).is_null ()
    {
      delete_frame (frame, Qnoelisp ());
      error (c"Invalid frame font".as_ptr ());
    }

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (), RES_TYPE_NUMBER);

  if NILP (Fassq (Qinternal_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if !BASE_EQ (value, Qunbound ())
	{ parms = Fcons (Fcons (Qinternal_border_width (), value), parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (),
			 make_fixnum (0),
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* Same for child frames.  */
  if NILP (Fassq (Qchild_frame_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qchild_frame_border_width (),
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
  gui_default_parameter (f, parms, Qvertical_scroll_bars (),
			 Qright (),
			 c"verticalScrollBars".as_ptr (), c"ScrollBars".as_ptr (), RES_TYPE_SYMBOL);
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
			 c"screenGamma".as_ptr (), c"ScreenGamma".as_ptr (), RES_TYPE_FLOAT);
  gui_default_parameter (f, parms, Qline_spacing (), Qnil (),
			 c"lineSpacing".as_ptr (), c"LineSpacing".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qleft_fringe (), Qnil (),
			 c"leftFringe".as_ptr (), c"LeftFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_fringe (), Qnil (),
			 c"rightFringe".as_ptr (), c"RightFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parms, Qscroll_bar_foreground (), Qnil (),
			 c"scrollBarForeground".as_ptr (), c"ScrollBarForeground".as_ptr (),
			 RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qscroll_bar_background (), Qnil (),
			 c"scrollBarBackground".as_ptr (), c"ScrollBarBackground".as_ptr (),
			 RES_TYPE_STRING);

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);

  /* We have to call adjust_frame_size here since otherwise
     pgtk_set_tool_bar_lines will already work with the character
     sizes installed by init_frame_faces while the frame's pixel size
     is still calculated from a character size of 1 and we
     subsequently hit the (height >= 0) assertion in
     window_box_height.

     The non-pixelwise code apparently worked around this because it
     had one frame line vs one toolbar line which left us with a zero
     root window height which was obviously wrong as well ...

     Also process `min-width' and `min-height' parameters right here
     because `frame-windows-min-size' needs them.  */
  tem = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qmin_width (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);
  if NUMBERP (tem)
    { store_frame_param (f, Qmin_width (), tem); }
  tem = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qmin_height (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);
  if NUMBERP (tem)
    { store_frame_param (f, Qmin_height (), tem); }
  adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
		     FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 5, true,
		     Qx_create_frame_1 ());

  /* Set the menu-bar-lines and tool-bar-lines parameters.  We don't
     look up the X resources controlling the menu-bar and tool-bar
     here; they are processed specially at startup, and reflected in
     the values of the mode variables.  */

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
			 c"title".as_ptr (), c"Title".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qwait_for_wm (), Qt (),
			 c"waitForWM".as_ptr (), c"WaitForWM".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qtool_bar_position (),
			 FRAME_TOOL_BAR_POSITION (f), std::ptr::null (), std::ptr::null (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
			 RES_TYPE_BOOLEAN);

  /* Compute the size of the X window.  */
  window_prompting =
    gui_figure_window_size (f, parms, true, true);

  tem =
    gui_display_get_arg (dpyinfo as *mut c_void, parms, Qunsplittable (), std::ptr::null (),
			 std::ptr::null (), RES_TYPE_BOOLEAN);
  no_split! (f, minibuffer_only || EQ (tem, Qt ()));

  xg_create_frame_widgets (f);
  pgtk_set_event_handler (f);

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { gtk_widget_realize (FRAME_GTK_OUTER_WIDGET! (f)); }

  /* Many callers (including the Lisp functions that call
     FRAME_SCALE_FACTOR) expect the widget to be realized.  */
  if !FRAME_GTK_WIDGET! (f).is_null ()
    { gtk_widget_realize (FRAME_GTK_WIDGET! (f)); }

  /* C 1577-1578: `#define INSTALL_CURSOR(FIELD, NAME) FRAME_X_OUTPUT (f)->FIELD =
     gdk_cursor_new_for_display (FRAME_X_DISPLAY (f), GDK_ ## NAME)` - C's two-line macro pastes
     `GDK_` onto the name, which Rust cannot spell, so each call site passes the pasted name and the
     macro takes the member and the constant (the device this file's first slice uses for C's other
     token-pasting macro); `f` is the enclosing function's own, exactly as C's `f` is.  C's
     `Emacs_Cursor` is `void *` here, so the pointer conversion is written out.  */
  macro_rules! INSTALL_CURSOR {
    ($field:ident, $name:ident) => {
      (*FRAME_X_OUTPUT! (f)).$field
	= gdk_cursor_new_for_display (FRAME_X_DISPLAY! (f), $name) as Emacs_Cursor;
    };
  }

  INSTALL_CURSOR! (text_cursor, GDK_XTERM);
  INSTALL_CURSOR! (nontext_cursor, GDK_LEFT_PTR);
  INSTALL_CURSOR! (modeline_cursor, GDK_XTERM);
  INSTALL_CURSOR! (hand_cursor, GDK_HAND2);
  INSTALL_CURSOR! (hourglass_cursor, GDK_WATCH);
  INSTALL_CURSOR! (horizontal_drag_cursor, GDK_SB_H_DOUBLE_ARROW);
  INSTALL_CURSOR! (vertical_drag_cursor, GDK_SB_V_DOUBLE_ARROW);
  INSTALL_CURSOR! (left_edge_cursor, GDK_LEFT_SIDE);
  INSTALL_CURSOR! (right_edge_cursor, GDK_RIGHT_SIDE);
  INSTALL_CURSOR! (top_edge_cursor, GDK_TOP_SIDE);
  INSTALL_CURSOR! (bottom_edge_cursor, GDK_BOTTOM_SIDE);
  INSTALL_CURSOR! (top_left_corner_cursor, GDK_TOP_LEFT_CORNER);
  INSTALL_CURSOR! (top_right_corner_cursor, GDK_TOP_RIGHT_CORNER);
  INSTALL_CURSOR! (bottom_right_corner_cursor, GDK_BOTTOM_RIGHT_CORNER);
  INSTALL_CURSOR! (bottom_left_corner_cursor, GDK_BOTTOM_LEFT_CORNER);

  /* Now consider the frame official.  */
  (*((*f).terminal)).reference_count += 1;
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;
  Vframe_list = Fcons (frame, Vframe_list);

  /* We need to do this after creating the X window, so that the
     icon-creation functions can say whose icon they're describing.  */
  gui_default_parameter (f, parms, Qicon_type (), Qt (),
			 c"bitmapIcon".as_ptr (), c"BitmapIcon".as_ptr (), RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (), RES_TYPE_BOOLEAN);
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
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  if !NILP (parent_frame)
    {
      let p: *mut frame = XFRAME (parent_frame);

      block_input ();

      let fixed: *mut GtkWidget = FRAME_GTK_WIDGET! (f);
      let fixed_of_p: *mut GtkWidget = FRAME_GTK_WIDGET! (p);
      let whbox_of_f: *mut GtkWidget = gtk_widget_get_parent (fixed);
      g_object_ref (fixed as gpointer);
      gtk_container_remove (GTK_CONTAINER! (whbox_of_f), fixed);
      gtk_fixed_put (GTK_FIXED! (fixed_of_p), fixed, (*f).left_pos, (*f).top_pos);
      gtk_widget_show_all (fixed);
      g_object_unref (fixed as gpointer);

      gtk_widget_destroy (FRAME_GTK_OUTER_WIDGET! (f));
      /* C 1628's `FRAME_GTK_OUTER_WIDGET (f) = NULL;` - the same l-value case as C 1308.  */
      (*FRAME_X_OUTPUT! (f)).widget = std::ptr::null_mut ();
      /* C 1629-1635's seven `FRAME_OUTPUT_DATA (f)->… = NULL;`, through C's own macro name.  */
      (*FRAME_OUTPUT_DATA! (f)).vbox_widget = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).hbox_widget = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).menubar_widget = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).toolbar_widget = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).ttip_widget = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).ttip_lbl = std::ptr::null_mut ();
      (*FRAME_OUTPUT_DATA! (f)).ttip_window = std::ptr::null_mut ();

      unblock_input ();
    }

  if !FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    {
      let mut w: *mut GList =
	gtk_container_get_children (GTK_CONTAINER! (FRAME_GTK_OUTER_WIDGET! (f)));
      while !w.is_null ()
	{
	  gtk_widget_show_all (GTK_WIDGET! ((*w).data));
	  w = (*w).next;
	}
    }

  gui_default_parameter (f, parms, Qno_focus_on_map (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qno_accept_focus (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);

  /* Create the menu bar.  */
  if !minibuffer_only && FRAME_EXTERNAL_MENU_BAR! (f)
    {
      /* If this signals an error, we haven't set size hints for the
	 frame and we didn't make it visible.  */
      initialize_frame_menubar (f);

    }

  /* Consider frame official, now.  */
  can_set_window_size! (f, true);

  /* Tell the server what size and position, etc, we want, and how
     badly we want them.  This should be done after we have the menu
     bar so that its size can be taken into account.  */
  block_input ();
  xg_wm_set_size_hint (f, window_prompting, false);
  unblock_input ();

  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qx_create_frame_2 ());

  /* Process fullscreen parameter here in the hope that normalizing a
     fullheight/fullwidth frame will produce the size set by the last
     adjust_frame_size call.  */
  gui_default_parameter (f, parms, Qfullscreen (), Qnil (),
			 c"fullscreen".as_ptr (), c"Fullscreen".as_ptr (), RES_TYPE_SYMBOL);

  /* Make the window appear on the frame and enable display, unless
     the caller says not to.  However, with explicit parent, Emacs
     cannot control visibility, so don't try.  */
  if !(*FRAME_X_OUTPUT! (f)).explicit_parent
    {
      /* When called from `x-create-frame-with-faces' visibility is
	 always explicitly nil.  */
      let mut visibility: Lisp_Object
	= gui_display_get_arg (dpyinfo as *mut c_void, parms, Qvisibility (),
			       std::ptr::null (), std::ptr::null (), RES_TYPE_SYMBOL);
      let height: Lisp_Object
	= gui_display_get_arg (dpyinfo as *mut c_void, parms, Qheight (),
			       std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);
      let width: Lisp_Object
	= gui_display_get_arg (dpyinfo as *mut c_void, parms, Qwidth (),
			       std::ptr::null (), std::ptr::null (), RES_TYPE_NUMBER);

      if EQ (visibility, Qicon ())
	{
	  was_invisible! (f, true);
	  pgtk_iconify_frame (f);
	}
      else
	{
	  if BASE_EQ (visibility, Qunbound ())
	    { visibility = Qt (); }

	  if !NILP (visibility)
	    { pgtk_make_frame_visible (f); }
	  else
	    { was_invisible! (f, true); }
	}

      /* Leave f->was_invisible true only if height or width were
	 specified too.  This takes effect only when we are not called
	 from `x-create-frame-with-faces' (see above comment).  */
      was_invisible! (f, was_invisible! (f)
			 && (!BASE_EQ (height, Qunbound ()) || !BASE_EQ (width, Qunbound ())));

      store_frame_param (f, Qvisibility (), visibility);
    }

  /* Works iff frame has been already mapped.  */
  gui_default_parameter (f, parms, Qskip_taskbar (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_BOOLEAN);
  /* The `z-group' parameter works only for visible frames.  */
  gui_default_parameter (f, parms, Qz_group (), Qnil (),
			 std::ptr::null (), std::ptr::null (), RES_TYPE_SYMBOL);

  /* Initialize `default-minibuffer-frame' in case this is the first
     frame on this terminal.  */
  /* C 1745-1746: `KVAR (kb, Vdefault_minibuffer_frame)` is keyboard.h 38's token-pasting macro
     (`(kboard)->field ## _`), which Rust cannot spell; the member is
     `include/keyboard.rs` 71's `Vdefault_minibuffer_frame_`, so the port writes the member (the
     `src/androidfns.rs` 1555 shape).  */
  if FRAME_HAS_MINIBUF_P (f)
      && (!FRAMEP ((*kb).Vdefault_minibuffer_frame_)
	  || !FRAME_LIVE_P (XFRAME ((*kb).Vdefault_minibuffer_frame_)))
    { kset_default_minibuffer_frame (kb, frame); }

  /* All remaining specified parameters, which have not been "used"
     by gui_display_get_arg and friends, now go in the misc. alist of the frame.  */
  tem = parms;
  while CONSP (tem)
    {
      if CONSP (XCAR (tem)) && !NILP (XCAR (XCAR (tem)))
	{ fset_param_alist (f, Fcons (XCAR (tem), (*f).param_alist)); }
      tem = XCDR (tem);
    }

  (*FRAME_X_OUTPUT! (f)).border_color_css_provider = std::ptr::null_mut ();

  (*FRAME_X_OUTPUT! (f)).cr_surface_visible_bell = std::ptr::null_mut ();
  (*FRAME_X_OUTPUT! (f)).atimer_visible_bell = std::ptr::null_mut ();
  (*FRAME_X_OUTPUT! (f)).watched_scale_factor = 1.0;
  let ts: timespec = make_timespec (1, 0);
  (*FRAME_X_OUTPUT! (f)).scale_factor_atimer = start_atimer (ATIMER_CONTINUOUS,
							     ts,
							     Some (update_watched_scale_factor),
							     f as *mut c_void);

  /* Make sure windows on this frame appear in calls to next-window
     and similar functions.  */
  Vwindow_list = Qnil ();

  return unbind_to (count, frame);
});

/* Restack frame F1 below frame F2, above if ABOVE_FLAG is non-nil.
   In practice this is a two-step action: The first step removes F1's
   window-system window from the display.  The second step reinserts
   F1's window below (above if ABOVE_FLAG is true) that of F2.  */

/// C: C 1758-1759: `static void pgtk_frame_restack (struct frame *f1, struct frame *f2, bool
/// above_flag)` - C's `static void` with the file's own `unsafe fn` shape for C's statics; C
/// 1789's `DEFUN` calls it.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_frame_restack (f1: *mut frame, f2: *mut frame, above_flag: bool)
{
  block_input ();
  xg_frame_restack (f1, f2, above_flag);
  unblock_input ();
}

/* C 1766: `DEFUN ("pgtk-frame-restack", Fpgtk_frame_restack, Spgtk_frame_restack, 2, 3, 0,` with
   C 1767-1781's own docstring; its `\(stacking)` is make-docfile's escape, whose *value* is
   `(stacking)` - measured: `tools/extract-abi.py --check` reads `subr Fpgtk_frame_restack:
   docstring differs` while the literal carries `\\(`, i.e. a backslash the reference's text does
   not have - so the literal spells the value.  C 1782's parameter list and C 1783-1791's body
   follow; C 1787's pointer truth test `!(A && B)` is written with the port's `is_null ()` spelling
   (the `check_pgtk_display_info` row of `docs/ports/macro-body-notes.tsv`).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-frame-restack", Fpgtk_frame_restack, Spgtk_frame_restack,   // C 1766-1791
	2, 3, 0,
	doc: "Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nThis may be thought of as an atomic action performed in two steps: The\nfirst step removes FRAME1's window-step window from the display.  The\nsecond step reinserts FRAME1's window below (above if ABOVE is true)\nthat of FRAME2.  Hence the position of FRAME2 in its display's Z\n(stacking) order relative to all other frames excluding FRAME1 remains\nunaltered.\n\nSome window managers may refuse to restack windows.  ",
	(frame1: Lisp_Object, frame2: Lisp_Object, above: Lisp_Object),
	{
  let f1: *mut frame = decode_live_frame (frame1);              // C 1784
  let f2: *mut frame = decode_live_frame (frame2);              // C 1785

  if FRAME_GTK_OUTER_WIDGET! (f1).is_null ()
      || FRAME_GTK_OUTER_WIDGET! (f2).is_null ()
    { error (c"Cannot restack frames".as_ptr ()); }             // C 1787-1788
  pgtk_frame_restack (f1, f2, !NILP (above));                   // C 1789
  return Qt ();                                                 // C 1790
});

/* C 1793-1996: `#ifdef HAVE_GSETTINGS` ... C 1982's `#else /* not HAVE_GSETTINGS */` ... C 1996's
   `#endif`.  `HAVE_GSETTINGS` is `undef` in the frozen configuration
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1384), so C's live text is the three-line `#else`
   arm - **both arms are carried** (P-86 ①), which is why the two names both arms define
   (`pgtk_get_defaults_value` and `pgtk_set_defaults_value`) are one `#[cfg]`-selected definition
   each, while C 1795-1800's five `#define`s, C 1802-1818's three predicates and C 1820-1921's
   `parse_resource_key` belong to the `#if` arm alone.  */

/* C 1795-1800's five `#define`s.  C 1976-1980's five `#undef`s have no Rust counterpart
   (PORTING.md 0.1: a `#define` is an item here, and an item is not un-defined);
   `PATH_MAX_LEN`'s `max (sizeof A, sizeof B)` cannot be a call in an array's length, so the port
   writes the constant C's expression denotes.  */
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
const RESOURCE_KEY_MAX_LEN: usize = 128;                                 // C 1795
/// C 1796: `#define SCHEMA_ID` - C's string macro is the port's same-named constant below, whose
/// bytes are the GSettings schema id the reference's own data file installs: an *external* name
/// the GIO schema lookup resolves, so the value is carried and **allowlisted**, not renamed
/// (`docs/baseline/declarations-allowlist.tsv`'s row holds the reason; the `src/w32.rs` 4676
/// `REG_ROOT` shape for the same class of macro).  The C line's text stays in this ledger.
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
const SCHEMA_ID: *const c_char = c"org.gnu.emacs.defaults".as_ptr ();
/// C 1797: `#define PATH_FOR_CLASS_TYPE "/org/gnu/emacs/defaults-by-class/"`
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
const PATH_FOR_CLASS_TYPE: *const c_char = c"/org/gnu/emacs/defaults-by-class/".as_ptr ();
/// C 1798: `#define PATH_PREFIX_FOR_NAME_TYPE "/org/gnu/emacs/defaults-by-name/"`
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
const PATH_PREFIX_FOR_NAME_TYPE: *const c_char = c"/org/gnu/emacs/defaults-by-name/".as_ptr ();
/// C 1799-1800: `#define PATH_MAX_LEN \` / `(max (sizeof PATH_FOR_CLASS_TYPE, sizeof
/// PATH_PREFIX_FOR_NAME_TYPE))` - C's `sizeof` of a string literal is its length plus the NUL
/// (measured: 34 and 33, so C's `max` is 34), and `lisp.h`'s `max` cannot stand in an array's
/// length (PORTING.md 1.3's const half).
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
const PATH_MAX_LEN: usize = 34;

/// C: C 1802-1806: `static inline int pgtk_is_lower_char (int c)` - C's `static inline` (P-12: no
/// Rust spelling to carry), with this file's `unsafe fn` shape for C's `static` helpers.
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn pgtk_is_lower_char (c: c_int) -> c_int
{
  return (c >= 'a' as c_int && c <= 'z' as c_int) as c_int;
}
/// C: C 1808-1812: `static inline int pgtk_is_upper_char (int c)`
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn pgtk_is_upper_char (c: c_int) -> c_int
{
  return (c >= 'A' as c_int && c <= 'Z' as c_int) as c_int;
}
/// C: C 1814-1818: `static inline int pgtk_is_numeric_char (int c)`
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn pgtk_is_numeric_char (c: c_int) -> c_int
{
  return (c >= '0' as c_int && c <= '9' as c_int) as c_int;
}

/* The GSettings surface, from `<gio/gio.h>`: C 1793's `#ifdef` reaches it through `"gtkutil.h"`
   26's `<gtk/gtk.h>` chain (C 31).  **GLib and GIO are on this machine**
   (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0 + the generated `glibconfig.h`), so every
   declaration below quotes the header and the line it was read from - the `src/xsettings.rs`
   286-370 segment's shape, which is where this class of names lives today (P-88 ①'s third-party
   segment; `include/gtk.rs` carries the GLib/GObject half this file already reaches -
   `g_free`/`g_object_unref`/`TRUE` - and not GIO's, so `docs/ports/pgtkfns.md`'s Owed names this
   segment as the one to move if that mirror grows a GIO part).  */
/// C: gio/giotypes.h 61: `typedef struct _GSettings GSettings;` (opaque; C never dereferences it)
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub struct _GSettings { _private: [u8; 0] }
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub type GSettings = _GSettings;
/// C: gio/gsettingsschema.h 29-30: `typedef struct _GSettingsSchemaSource
/// GSettingsSchemaSource;` / `typedef struct _GSettingsSchema GSettingsSchema;` (both opaque)
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub struct _GSettingsSchemaSource { _private: [u8; 0] }
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub type GSettingsSchemaSource = _GSettingsSchemaSource;
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub struct _GSettingsSchema { _private: [u8; 0] }
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
pub type GSettingsSchema = _GSettingsSchema;

#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe extern "C" {
  /// C: gio/gsettingsschema.h 38: `GSettingsSchemaSource *g_settings_schema_source_get_default
  /// (void);` (C 1904)
  pub fn g_settings_schema_source_get_default () -> *mut GSettingsSchemaSource;
  /// C: gio/gsettingsschema.h 51-54: `GSettingsSchema *g_settings_schema_source_lookup
  /// (GSettingsSchemaSource *source, const gchar *schema_id, gboolean recursive);` (C 1907)
  pub fn g_settings_schema_source_lookup (source: *mut GSettingsSchemaSource,
					  schema_id: *const gchar,
					  recursive: gboolean) -> *mut GSettingsSchema;
  /// C: gio/gsettingsschema.h 78-80: `gboolean g_settings_schema_has_key (GSettingsSchema *schema,
  /// const gchar *name);` (C 1910)
  pub fn g_settings_schema_has_key (schema: *mut GSettingsSchema, name: *const gchar) -> gboolean;
  /// C: gio/gsettingsschema.h 68: `void g_settings_schema_unref (GSettingsSchema *schema);`
  /// (C 1912/1918)
  pub fn g_settings_schema_unref (schema: *mut GSettingsSchema);
  /// C: gio/gsettings.h 92-95: `GSettings *g_settings_new_full (GSettingsSchema *schema,
  /// GSettingsBackend *backend, const gchar *path);` (C 1917; C's second argument is `NULL`, and
  /// `GSettingsBackend` is not named here because C never spells its type at this call site)
  pub fn g_settings_new_full (schema: *mut GSettingsSchema, backend: *mut c_void,
			      path: *const gchar) -> *mut GSettings;
  /// C: gio/gsettings.h 165-166: `gchar *g_settings_get_string (GSettings *settings, const gchar
  /// *key);` (C 1941)
  pub fn g_settings_get_string (settings: *mut GSettings, key: *const gchar) -> *mut gchar;
  /// C: gio/gsettings.h 168-170: `gboolean g_settings_set_string (GSettings *settings, const gchar
  /// *key, const gchar *value);` (C 1969)
  pub fn g_settings_set_string (settings: *mut GSettings, key: *const gchar, value: *const gchar)
				-> gboolean;
  /// C: gio/gsettings.h 133-134: `void g_settings_reset (GSettings *settings, const gchar *key);`
  /// (C 1971)
  pub fn g_settings_reset (settings: *mut GSettings, key: *const gchar);
}

/// C: C 1820-1821: `static GSettings *parse_resource_key (const char *res_key, char
/// *setting_key)` - C's `static`, so the file's `unsafe fn` shape.  C 1823's array length is
/// `PATH_MAX_LEN + RESOURCE_KEY_MAX_LEN` (162); C's `*sp++` in the condition at C 1877 is the
/// load-then-step pair (the `pbm_next_char` shape, PORTING.md 2.2); C's three `(void) 0;` NOPs
/// (C 1863/1867/1886/1894) are `let _ = 0;` (C's cast-to-void expression as a statement).
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn parse_resource_key (res_key: *const c_char, setting_key: *mut c_char)
			      -> *mut GSettings
{
  let mut path: [c_char; PATH_MAX_LEN + RESOURCE_KEY_MAX_LEN] =
    [0; PATH_MAX_LEN + RESOURCE_KEY_MAX_LEN];                   // C 1823
  let mut sp: *const c_char = res_key;                          // C 1824
  let mut dp: *mut c_char;                                      // C 1825

  /*
   * res_key="emacs.cursorBlink"
   *   -> path="/org/gnu/emacs/defaults-by-name/emacs/"
   *      setting_key="cursor-blink"
   *
   * res_key="Emacs.CursorBlink"
   *   -> path="/org/gnu/emacs/defaults-by-class/"
   *      setting_key="cursor-blink"
   *
   * Returns GSettings* if setting_key exists in schema, otherwise NULL.
   */

  /* generate path */
  if pgtk_is_upper_char (*sp as c_int) != 0                      // C 1840
    {
      /* First letter is upper case.  It should be "Emacs",
       * but don't care.
       */
      strcpy (path.as_mut_ptr (), PATH_FOR_CLASS_TYPE);          // C 1845
      while *sp != '\0' as c_char                                // C 1846
	{
	  if *sp == b'.' as c_char
	    { break; }
	  sp = sp.offset (1);
	}
    }
  else
    {
      strcpy (path.as_mut_ptr (), PATH_PREFIX_FOR_NAME_TYPE);    // C 1855
      dp = path.as_mut_ptr ().offset (strlen (path.as_ptr ()) as isize);   // C 1856
      while *sp != '\0' as c_char                                // C 1857
	{
	  let mut c: c_int = *sp as c_int;                       // C 1859
	  if c == b'.' as c_int
	    { break; }
	  if pgtk_is_lower_char (c) != 0
	    { let _ = 0; }		/* lower -> NOP */
	  else if pgtk_is_upper_char (c) != 0
	    { c = c - 'A' as c_int + 'a' as c_int; }	/* upper -> lower */
	  else if pgtk_is_numeric_char (c) != 0
	    { let _ = 0; }		/* numeric -> NOP */
	  else
	    { return std::ptr::null_mut (); }	/* invalid */
	  *dp = c as c_char;                                     // C 1870
	  dp = dp.offset (1);
	  sp = sp.offset (1);
	}
      *dp = b'/' as c_char;                                      // C 1873
      dp = dp.offset (1);		/* must end with '/' */
      *dp = '\0' as c_char;                                      // C 1874
    }

  /* C 1877-1878: `if (*sp++ != '.') return NULL;` - the load, the step, then the test (the
     `pbm_next_char` shape of PORTING.md 2.2).  */
  let ch: c_char = *sp;
  sp = sp.offset (1);
  if ch != b'.' as c_char
    { return std::ptr::null_mut (); }

  /* generate setting_key */
  dp = setting_key;                                              // C 1881
  while *sp != '\0' as c_char                                    // C 1882
    {
      let mut c: c_int = *sp as c_int;                           // C 1884
      if pgtk_is_lower_char (c) != 0
	{ let _ = 0; }		/* lower -> NOP */
      else if pgtk_is_upper_char (c) != 0
	{
	  c = c - 'A' as c_int + 'a' as c_int;	/* upper -> lower */
	  if dp != setting_key
	    {							// C 1891
	      *dp = b'-' as c_char;	/* store '-' unless first char */
	      dp = dp.offset (1);
	    }
	}
      else if pgtk_is_numeric_char (c) != 0
	{ let _ = 0; }		/* numeric -> NOP */
      else
	{ return std::ptr::null_mut (); }	/* invalid */

      *dp = c as c_char;                                         // C 1898
      dp = dp.offset (1);
      sp = sp.offset (1);
    }
  *dp = '\0' as c_char;                                          // C 1901

  /* check existence of setting_key */
  let ssrc: *mut GSettingsSchemaSource = g_settings_schema_source_get_default ();   // C 1904
  if ssrc.is_null ()
    { return std::ptr::null_mut (); }	/* No GSettings schemas installed.  */
  let scm: *mut GSettingsSchema =
    g_settings_schema_source_lookup (ssrc, SCHEMA_ID, TRUE);     // C 1907
  if scm.is_null ()
    { return std::ptr::null_mut (); }	/* *.schema.xml is not installed. */
  if g_settings_schema_has_key (scm, setting_key) == 0           // C 1910
    {
      g_settings_schema_unref (scm);
      return std::ptr::null_mut ();
    }

  /* create GSettings, and return it */
  let gs: *mut GSettings = g_settings_new_full (scm, std::ptr::null_mut (), path.as_ptr ());

  g_settings_schema_unref (scm);                                 // C 1919
  return gs;                                                     // C 1920
}

/// C: C 1923-1924: `static void pgtk_check_resource_key_length (const char *key)`
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn pgtk_check_resource_key_length (key: *const c_char)
{
  if strnlen (key, RESOURCE_KEY_MAX_LEN) >= RESOURCE_KEY_MAX_LEN    // C 1926
    { error (c"Resource key too long".as_ptr ()); }
}

/// C: C 1930-1931: `const char *pgtk_get_defaults_value (const char *key)` - C's non-`static`
/// definition (pgtkterm.h 583 declares it), so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_get_defaults_value (key: *const c_char) -> *const c_char
{
  let mut skey: [c_char; (RESOURCE_KEY_MAX_LEN + 1) * 2] =
    [0; (RESOURCE_KEY_MAX_LEN + 1) * 2];                         // C 1933

  pgtk_check_resource_key_length (key);                          // C 1935

  let gs: *mut GSettings = parse_resource_key (key, skey.as_mut_ptr ());   // C 1937
  if gs.is_null ()
    { return std::ptr::null (); }

  let str_: *mut gchar = g_settings_get_string (gs, skey.as_ptr ());       // C 1941

  /* There is no timing to free str.
   * So, copy it here and free it.
   *
   * MEMO: Resource values for emacs shouldn't need such a long string value.
   */

  /* C 1948: `static char holder[128];` - C's function-scope static, whose storage duration is the
     program's.  The port writes Rust's own function-scope `static mut` (zero-initialised once -
     PORTING.md 0.1-13's tentative-definition half) and takes its address with `addr_of_mut!`
     instead of a reference (Rust 2024's `static_mut_refs`; the `src/alloc.rs` 588 shape), so the
     parameter `holder` C passes on is that pointer, exactly as C's array name decays.  */
  static mut HOLDER: [c_char; 128] = [0; 128];
  let holder: *mut c_char = std::ptr::addr_of_mut! (HOLDER) as *mut c_char;
  strncpy (holder, str_, 128);                                   // C 1949
  *holder.offset (127) = '\0' as c_char;                         // C 1950

  /* C's `g_object_unref (gs)` and `g_free (str)` hand a `GSettings *` and a `gchar *` to
     `gpointer` parameters; the conversion C makes implicitly is written out (Deviation 5's rule).  */
  g_object_unref (gs as gpointer);                               // C 1952
  g_free (str_ as gpointer);                                     // C 1953
  return if *holder != '\0' as c_char { holder } else { std::ptr::null () };   // C 1954
}

/// C: C 1957-1958: `static void pgtk_set_defaults_value (const char *key, const char *value)` -
/// C's `static` (both arms declare it so).
#[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
unsafe fn pgtk_set_defaults_value (key: *const c_char, value: *const c_char)
{
  let mut skey: [c_char; (RESOURCE_KEY_MAX_LEN + 1) * 2] =
    [0; (RESOURCE_KEY_MAX_LEN + 1) * 2];                         // C 1960

  pgtk_check_resource_key_length (key);                          // C 1962

  let gs: *mut GSettings = parse_resource_key (key, skey.as_mut_ptr ());   // C 1964
  if gs.is_null ()
    { error (c"Unknown resource key".as_ptr ()); }               // C 1966

  if !value.is_null ()                                           // C 1968
    { g_settings_set_string (gs, skey.as_ptr (), value); }
  else
    { g_settings_reset (gs, skey.as_ptr ()); }

  g_object_unref (gs as gpointer);                               // C 1973
}

/* C 1982-1996: the `#else /* not HAVE_GSETTINGS */` arm - the live one in the frozen
   configuration - which defines the *same two names* the `#if` arm does.  Each keeps its own
   definition, `#[cfg]`-selected (the `src/image.rs` arm-pair shape).  */

/// C: C 1984-1985: `const char *pgtk_get_defaults_value (const char *key)`
#[cfg (all (feature = "have_pgtk", not (feature = "have_gsettings")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_get_defaults_value (key: *const c_char) -> *const c_char
{
  let _ = key;                 /* C's parameter is unused in this arm (the `src/print.rs` shape).  */
  return std::ptr::null ();                                    // C 1987
}

/// C: C 1990-1991: `static void pgtk_set_defaults_value (const char *key, const char *value)`
#[cfg (all (feature = "have_pgtk", not (feature = "have_gsettings")))]
unsafe fn pgtk_set_defaults_value (key: *const c_char, value: *const c_char)
{
  let _ = (key, value);        /* C's parameters are unused in this arm.  */
  error (c"gsettings not supported".as_ptr ());                // C 1993
}

/* ==========================================================================

   C 1999-2208: the resource subr, the eleven display subrs and `pgtk-font-name`

   ========================================================================== */

/* C 1999-2000: `DEFUN ("pgtk-set-resource", Fpgtk_set_resource, Spgtk_set_resource, 2, 2, 0,` with
   C's own one-line docstring; C 2001's parameter list and C 2002-2019's body follow.  C 2009-2010's
   two `SSDATA` results are C's `char *` locals, so the port keeps them as `*mut c_char` (the
   `x_decode_color` shape above).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-set-resource", Fpgtk_set_resource, Spgtk_set_resource,   // C 1999-2019
	2, 2, 0,
	doc: "Set the value of ATTRIBUTE, of class CLASS, as VALUE, into defaults database. ",
	(attribute: Lisp_Object, value: Lisp_Object),
	{
  check_window_system (std::ptr::null_mut ());                  // C 2003

  CHECK_STRING (attribute);                                     // C 2005
  if !NILP (value)
    { CHECK_STRING (value); }                                   // C 2007

  let mut res: *mut c_char = SSDATA (Vx_resource_name);         // C 2009
  let attr: *mut c_char = SSDATA (attribute);                   // C 2010
  /* C 2011's `attr[0]` is the port's `*attr.offset (0)` - C's own index read (the `src/alloc.rs`
     3967 shape for a raw pointer's element).  */
  if *attr.offset (0) >= 'A' as c_char && *attr.offset (0) <= 'Z' as c_char   // C 2011
    { res = SSDATA (Vx_resource_class); }                                   // C 2012

  let key: *mut c_char = g_strdup_printf (c"%s.%s".as_ptr (), res, attr);   // C 2014

  pgtk_set_defaults_value (key,                                       // C 2016
			   if NILP (value) { std::ptr::null () } else { SSDATA (value) });

  return Qnil ();                                               // C 2018
});

/* C 2022-2030: `x-server-max-request-size` and C 2033-2039: `x-display-screens` - two of the eleven
   `x-*` display subrs this file defines for the `x-*` Lisp API (`xfns.c`'s docstrings are the real
   ones, hence the file's `SKIP` form).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-server-max-request-size", Fx_server_max_request_size, Sx_server_max_request_size,   // C 2022-2030
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2026
  /* This function has no real equivalent under PGTK.  Return nil to
     indicate this. */
  return Qnil ();                                               // C 2029
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-screens", Fx_display_screens, Sx_display_screens,   // C 2033-2039
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2037
  return make_fixnum (1);                                       // C 2038
});

// C: C 2042-2043: `DEFUN ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height, 0, 1,
// 0,` - C 2046-2072: the monitor walk that sums the millimetre heights of the monitors whose
// rectangle starts at y = 0 and adds the rest.  C 2064's `max` is `lisp.h`'s macro over two
// `int`s, written as the mirror's `max (a, b) -> isize` with the two conversions spelled (the
// `src/abi_shim.rs` 2457 shape); `GdkRectangle rec;` (C 2057) is C's uninitialised local that C
// 2060 fills, so the port zero-initialises it (the `x_decode_color` C 1058 shape).
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height,   // C 2042-2073
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2046
  let mut gdpy: *mut GdkDisplay;                                             // C 2047
  let mut n_monitors: gint;                                                  // C 2048
  let mut i: gint;
  let mut height_mm_at_0: c_int = 0;                                         // C 2049
  let mut height_mm_at_other: c_int = 0;

  block_input ();                                                            // C 2051
  gdpy = (*dpyinfo).gdpy;                                                    // C 2052
  n_monitors = gdk_display_get_n_monitors (gdpy);                            // C 2053

  i = 0;
  while i < n_monitors                                                       // C 2055
    {
      let mut rec: GdkRectangle = std::mem::zeroed ();                       // C 2057

      let monitor: *mut GdkMonitor = gdk_display_get_monitor (gdpy, i);      // C 2059
      gdk_monitor_get_geometry (monitor, &raw mut rec);                      // C 2060

      let mm: c_int = gdk_monitor_get_height_mm (monitor);                   // C 2062

      if rec.y == 0                                                          // C 2064
	{ height_mm_at_0 = max (height_mm_at_0 as isize, mm as isize) as c_int; }
      else
	{ height_mm_at_other += mm; }                                        // C 2067
      i += 1;
    }

  unblock_input ();                                                          // C 2070

  return make_fixnum ((height_mm_at_0 + height_mm_at_other) as EMACS_INT);   // C 2072
});

// C: C 2076-2077: `DEFUN ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width, 0, 1, 0,`
// - the same walk over `rec.x` and `gdk_monitor_get_width_mm` (C 2080-2106).
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width,   // C 2076-2107
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2080
  let mut gdpy: *mut GdkDisplay;                                             // C 2081
  let mut n_monitors: gint;                                                  // C 2082
  let mut i: gint;
  let mut width_mm_at_0: c_int = 0;                                          // C 2083
  let mut width_mm_at_other: c_int = 0;

  block_input ();                                                            // C 2085
  gdpy = (*dpyinfo).gdpy;                                                    // C 2086
  n_monitors = gdk_display_get_n_monitors (gdpy);                            // C 2087

  i = 0;
  while i < n_monitors                                                       // C 2089
    {
      let mut rec: GdkRectangle = std::mem::zeroed ();                       // C 2091

      let monitor: *mut GdkMonitor = gdk_display_get_monitor (gdpy, i);      // C 2093
      gdk_monitor_get_geometry (monitor, &raw mut rec);                      // C 2094

      let mm: c_int = gdk_monitor_get_width_mm (monitor);                    // C 2096

      if rec.x == 0                                                          // C 2098
	{ width_mm_at_0 = max (width_mm_at_0 as isize, mm as isize) as c_int; }
      else
	{ width_mm_at_other += mm; }                                         // C 2101
      i += 1;
    }

  unblock_input ();                                                          // C 2104

  return make_fixnum ((width_mm_at_0 + width_mm_at_other) as EMACS_INT);     // C 2106
});

// C: C 2110-2111: `DEFUN ("x-display-backing-store", …, 0, 1, 0,`
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store,   // C 2110-2116
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2114
  return Qnil ();                                               // C 2115
});

// C: C 2119-2120: `DEFUN ("x-display-visual-class", …, 0, 1, 0,` - C 2123's `Qtrue_color` with no
// `check_pgtk_display_info` call (C's own text has none either)
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class,   // C 2119-2124
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let _ = terminal;
  return Qtrue_color ();                                        // C 2123
});

// C: C 2127-2128: `DEFUN ("x-display-save-under", …, 0, 1, 0,`
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-save-under", Fx_display_save_under, Sx_display_save_under,   // C 2127-2133
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2131
  return Qnil ();                                               // C 2132
});

// C: C 2136-2137: `DEFUN ("x-open-connection", Fx_open_connection, Sx_open_connection, 1, 3, 0,` -
// C 2142-2143's `display = build_string ("")` when the display is nil, the `CHECK_STRING`, the
// `pgtk_term_init` call and the two-message failure path.
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-open-connection", Fx_open_connection, Sx_open_connection,   // C 2136-2157
	1, 3, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object, resource_string: Lisp_Object, must_succeed: Lisp_Object),
	{
  let mut display = display;                                    // C 2138 reassigns the parameter.
  let _ = resource_string;            /* C's third parameter is unused in C's own body.  */
  let mut dpyinfo: *mut pgtk_display_info;                      // C 2140

  if NILP (display)                                             // C 2142
    { display = build_string (c"".as_ptr ()); }

  CHECK_STRING (display);                                       // C 2145

  dpyinfo = pgtk_term_init (display, SSDATA (Vx_resource_name));    // C 2147
  if dpyinfo == 0 as *mut pgtk_display_info                     // C 2148 (C's `== 0`, the
                                                                // `src/w32term.rs` 2680 shape)
    {
      if !NILP (must_succeed)                                   // C 2150
	{ fatal (c"Display on %s not responding.\n".as_ptr (), SSDATA (display)); }
      else
	{ error (c"Display on %s not responding.\n".as_ptr (), SSDATA (display)); }
    }

  return Qnil ();                                               // C 2156
});

// C: C 2160-2161: `DEFUN ("x-close-connection", …, 1, 1, 0,`
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-close-connection", Fx_close_connection, Sx_close_connection,   // C 2160-2172
	1, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2164

  if (*dpyinfo).reference_count > 0                             // C 2166
    { error (c"Display still has frames on it".as_ptr ()); }    // C 2167

  pgtk_delete_terminal ((*dpyinfo).terminal);                   // C 2169

  return Qnil ();                                               // C 2171
});

/* C 2175-2176: `DEFUN ("x-display-list", …, 0, 0, 0,` - C 2177's `(void)` parameter list is the
   port's `()` (no parameter at all), and C 2182-2183's list walk is the file's `while` shape for
   C's `for`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-list", Fx_display_list, Sx_display_list,   // C 2175-2186
	0, 0, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(),
	{
  let mut result: Lisp_Object = Qnil ();                        // C 2179
  let mut ndi: *mut pgtk_display_info;                          // C 2180

  ndi = x_display_list;                                         // C 2182
  while !ndi.is_null ()
    {
      result = Fcons (XCAR ((*ndi).name_list_element), result);  // C 2183
      ndi = (*ndi).next;
    }

  return result;                                                // C 2185
});

/* C 2188-2189: `DEFUN ("pgtk-font-name", Fpgtk_font_name, Spgtk_font_name, 1, 1, 0,` with C's own
   docstring (C 2189-2192); C 2195's `char *nm;` is a `*mut c_char` here, C 2205 reassigns the
   parameter (the port shadows it, the `Fpgtk_set_monitor_scale_factor` shape), and C 2206's
   `xfree (str)` takes the mirror's `*mut c_void`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-font-name", Fpgtk_font_name, Spgtk_font_name,   // C 2188-2208
	1, 1, 0,
	doc: "Determine font PostScript or family name for font NAME.\nNAME should be a string containing either the font name or an XLFD\nfont descriptor.  If string contains `fontset' and not\n`fontset-startup', it is left alone. ",
	(name: Lisp_Object),
	{
  let mut name = name;                                          // C 2205 reassigns the parameter.
  let mut nm: *mut c_char;                                      // C 2195
  CHECK_STRING (name);                                          // C 2196
  nm = SSDATA (name);                                           // C 2197

  if *nm.offset (0) != b'-' as c_char                           // C 2199 (C's `nm[0]`)
    { return name; }
  if !strstr (nm, c"fontset".as_ptr ()).is_null ()              // C 2201
      && strstr (nm, c"fontset-startup".as_ptr ()).is_null ()
    { return name; }

  let str_: *mut c_char = pgtk_xlfd_to_fontname (SSDATA (name));   // C 2204
  name = build_string (str_);                                   // C 2205
  xfree (str_ as *mut c_void);                                  // C 2206
  return name;                                                  // C 2207
});

/* ==========================================================================

    Miscellaneous functions not called through hooks

   ========================================================================== */

/* ==========================================================================

   C 2216-2403: `check_x_display_info`, the scroll-bar defaults, the resource
   accessor and the first seven colour/geometry subrs

   ========================================================================== */

/* C 2216-2221: `check_x_display_info` - C's own "Called from frame.c." comment, a non-`static`
   definition (so `#[unsafe(no_mangle)]`, PORTING.md 0.2) that answers for the X API by delegating
   to this file's `check_pgtk_display_info`.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_x_display_info (frame: Lisp_Object) -> *mut pgtk_display_info
{
  return check_pgtk_display_info (frame);                       // C 2220
}

/* C 2223-2231: `pgtk_set_scroll_bar_default_width` and C 2233-2241:
   `pgtk_set_scroll_bar_default_height` - the two non-`static` scroll-bar defaults the frame/scroll
   bar code calls (pgtkterm.h's own two, C 2223/2233).  C writes the four `FRAME_CONFIG_SCROLL_BAR_*`
   macros as l-values, so this unit carries C's own two-arm copies of them (Deviation 43).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_scroll_bar_default_width (f: *mut frame)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);                     // C 2226
  let minw: c_int = xg_get_default_scrollbar_width (f);         // C 2227
  /* A minimum width of 14 doesn't look good for toolkit scroll bars.  */
  FRAME_CONFIG_SCROLL_BAR_COLS! (f, (minw + unit - 1) / unit);  // C 2229
  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, minw);                     // C 2230
}

#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_set_scroll_bar_default_height (f: *mut frame)
{
  let height: c_int = FRAME_LINE_HEIGHT (f);                    // C 2236
  let min_height: c_int = xg_get_default_scrollbar_height (f);  // C 2237
  /* A minimum height of 14 doesn't look good for toolkit scroll bars.  */
  FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f, min_height);              // C 2239
  FRAME_CONFIG_SCROLL_BAR_LINES! (f, (min_height + height - 1) / height);   // C 2240
}

/* C 2243-2268: `pgtk_get_string_resource` - "Terminals implement this instead of x-get-resource
   directly."  C's `rdb` is never read (the GSettings path takes only the name and the class), so
   the port keeps the parameter and discards it (the `src/print.rs` 1693 shape); `inhibit_x_resources`
   is `emacs.rs` 682's `bool`, and the two resource results are `pgtk_get_defaults_value`'s.  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_get_string_resource (rdb: XrmDatabase, name: *const c_char,
						   class: *const c_char) -> *const c_char
{
  let _ = rdb;                     /* C's first parameter is unused in C's own body.  */

  check_window_system (std::ptr::null_mut ());                  // C 2248

  if inhibit_x_resources                                        // C 2250
    /* --quick was passed, so this is a no-op.  */
    { return std::ptr::null (); }                               // C 2252

  let mut res: *const c_char = pgtk_get_defaults_value (name);  // C 2254
  if res.is_null ()                                             // C 2255
    { res = pgtk_get_defaults_value (class); }                  // C 2256

  if res.is_null ()                                             // C 2258
    { return std::ptr::null (); }

  if c_strncasecmp (res, c"YES".as_ptr (), 3) == 0              // C 2261
    { return c"true".as_ptr (); }                               // C 2262

  if c_strncasecmp (res, c"NO".as_ptr (), 2) == 0               // C 2264
    { return c"false".as_ptr (); }                              // C 2265

  return res;                                                   // C 2267
}

/* C 2270-2281: `pgtk_get_focus_frame` - the terminal's keyboard-focus frame as a Lisp object
   (`dpyinfo->x_focus_frame` is `include/pgtkterm.rs` 204's member).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_get_focus_frame (frame: *mut frame) -> Lisp_Object
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (frame);   // C 2273
  let mut focus: Lisp_Object;                                        // C 2274

  if (*dpyinfo).x_focus_frame.is_null ()                             // C 2276
    { return Qnil (); }                                              // C 2277

  XSETFRAME! (focus, (*dpyinfo).x_focus_frame);                      // C 2279
  return focus;                                                      // C 2280
}

/* C 2283-2312: `xw-color-defined-p` and `xw-color-values` - the two colour predicates, both
   through `decode_window_system_frame` + `pgtk_defined_color`; C 2309's `list3i` takes the three
   `unsigned short` members widened (Deviation 44).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p,   // C 2283-2296
	1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut col: Emacs_Color;                                     // C 2287
  let f: *mut frame = decode_window_system_frame (frame);       // C 2288

  CHECK_STRING (color);                                         // C 2290

  col = std::mem::zeroed ();
  /* C 2292's second `false` goes to `pgtk_defined_color`'s `int index` parameter, so C's own
     token is kept and the promotion written (Deviation 5's rule).  */
  if pgtk_defined_color (f, SSDATA (color), &raw mut col, false, false as c_int)   // C 2292
    { return Qt (); }                                           // C 2293
  else
    { return Qnil (); }                                         // C 2295
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("xw-color-values", Fxw_color_values, Sxw_color_values,   // C 2299-2312
	1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut col: Emacs_Color;                                     // C 2303
  let f: *mut frame = decode_window_system_frame (frame);       // C 2304

  CHECK_STRING (color);                                         // C 2306

  col = std::mem::zeroed ();
  if pgtk_defined_color (f, SSDATA (color), &raw mut col, false, false as c_int)   // C 2308
    { return list3i (col.red as EMACS_INT, col.green as EMACS_INT,   // C 2309
		     col.blue as EMACS_INT); }
  else
    { return Qnil (); }                                         // C 2311
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p,   // C 2314-2320
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2318
  return Qt ();                                                 // C 2319
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p,   // C 2322-2327
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let _ = terminal;         /* C's parameter is unused in C's own body (no check call).  */
  return Qt ();                                                 // C 2326
});

/* C 2329-2365: `x-display-pixel-width` and C 2367-2403: `x-display-pixel-height` - the two
   monitor walks that scale each monitor's rectangle by the monitor's scale factor and keep the
   largest right/bottom edge.  C 2354-2357's four assignments are C's double-to-int truncation
   written out, C 2353's `scale = gdk_monitor_get_scale_factor (monitor)` widens an `int` to
   `double`, and C 2359's `max` over two `int`s is the mirror's `isize` fold (the `x-display-mm-*`
   shape, Deviation 38).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width,   // C 2329-2365
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2333
  let mut gdpy: *mut GdkDisplay;                                             // C 2334
  let mut n_monitors: gint;                                                  // C 2335
  let mut i: gint;
  let mut width: c_int = 0;                                                  // C 2336

  block_input ();                                                            // C 2338
  gdpy = (*dpyinfo).gdpy;                                                    // C 2339
  n_monitors = gdk_display_get_n_monitors (gdpy);                            // C 2340

  i = 0;
  while i < n_monitors                                                       // C 2342
    {
      let mut rec: GdkRectangle = std::mem::zeroed ();                       // C 2344
      let mut scale: c_double = 1.0;                                         // C 2345

      let monitor: *mut GdkMonitor = gdk_display_get_monitor (gdpy, i);      // C 2347
      gdk_monitor_get_geometry (monitor, &raw mut rec);                      // C 2348

      /* GTK returns scaled sizes for the workareas.  */
      scale = pgtk_get_monitor_scale_factor (gdk_monitor_get_model (monitor));   // C 2351
      if scale == 0.0                                                        // C 2352
	{ scale = gdk_monitor_get_scale_factor (monitor) as c_double; }      // C 2353
      rec.x = (rec.x as c_double * scale + 0.5) as c_int;                    // C 2354
      rec.y = (rec.y as c_double * scale + 0.5) as c_int;                    // C 2355
      rec.width = (rec.width as c_double * scale + 0.5) as c_int;            // C 2356
      rec.height = (rec.height as c_double * scale + 0.5) as c_int;          // C 2357

      width = max (width as isize, (rec.x + rec.width) as isize) as c_int;   // C 2359
      i += 1;
    }

  unblock_input ();                                                          // C 2362

  return make_fixnum (width as EMACS_INT);                                   // C 2364
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height,   // C 2367-2403
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2371
  let mut gdpy: *mut GdkDisplay;                                             // C 2372
  let mut n_monitors: gint;                                                  // C 2373
  let mut i: gint;
  let mut height: c_int = 0;                                                 // C 2374

  block_input ();                                                            // C 2376
  gdpy = (*dpyinfo).gdpy;                                                    // C 2377
  n_monitors = gdk_display_get_n_monitors (gdpy);                            // C 2378

  i = 0;
  while i < n_monitors                                                       // C 2380
    {
      let mut rec: GdkRectangle = std::mem::zeroed ();                       // C 2382
      let mut scale: c_double = 1.0;                                         // C 2383

      let monitor: *mut GdkMonitor = gdk_display_get_monitor (gdpy, i);      // C 2385
      gdk_monitor_get_geometry (monitor, &raw mut rec);                      // C 2386

      /* GTK returns scaled sizes for the workareas.  */
      scale = pgtk_get_monitor_scale_factor (gdk_monitor_get_model (monitor));   // C 2389
      if scale == 0.0                                                        // C 2390
	{ scale = gdk_monitor_get_scale_factor (monitor) as c_double; }      // C 2391
      rec.x = (rec.x as c_double * scale + 0.5) as c_int;                    // C 2392
      rec.y = (rec.y as c_double * scale + 0.5) as c_int;                    // C 2393
      rec.width = (rec.width as c_double * scale + 0.5) as c_int;            // C 2394
      rec.height = (rec.height as c_double * scale + 0.5) as c_int;          // C 2395

      height = max (height as isize, (rec.y + rec.height) as isize) as c_int;   // C 2397
      i += 1;
    }

  unblock_input ();                                                          // C 2400

  return make_fixnum (height as EMACS_INT);                                  // C 2402
});

/* ==========================================================================

   C 2405-2549: the monitor-attribute builder, `pgtk_frame_scale_factor` and
   the last two display subrs of this group

   ========================================================================== */

/* C 2405-2422: `DEFUN ("pgtk-display-monitor-attributes-list", …)` with C's own docstring (C
   2408-2421; its `\"Gdk\"` is make-docfile's escaped quote, whose value is `"Gdk"` - the literal
   carries the value, Deviation 32's rule) and C 2422's one-parameter list.  C 2428's
   `gint primary_monitor = 0` and C 2430's function-scope `static const char *source = "Gdk"`
   keep their C shapes (the latter as Rust's own function-scope `static mut`, Deviation 30's
   device); C 2437's `xcalloc (n_monitors, sizeof *monitors)` is the record's size.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-display-monitor-attributes-list", Fpgtk_display_monitor_attributes_list,   // C 2405-2510
	Spgtk_display_monitor_attributes_list,
	0, 1, 0,
	doc: "Return a list of physical monitor attributes on the X display TERMINAL.\n\nThe optional argument TERMINAL specifies which display to ask about.\nTERMINAL should be a terminal object, a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nIn addition to the standard attribute keys listed in\n`display-monitor-attributes-list', the following keys are contained in\nthe attributes:\n\n source -- String describing the source from which multi-monitor\n	   information is obtained, \"Gdk\"\n\nInternal use only, use `display-monitor-attributes-list' instead.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2424
  let mut attributes_list: Lisp_Object = Qnil ();                             // C 2425

  let mut gdpy: *mut GdkDisplay;                                           // C 2427
  let mut primary_monitor: gint = 0;                                       // C 2428
  let mut n_monitors: gint;
  let mut i: gint;
  let mut monitor_frames: Lisp_Object;                                     // C 2429
  let mut rest: Lisp_Object;
  let mut frame: Lisp_Object;
  /* C 2430: `static const char *source = "Gdk";` - C's function-scope static, whose storage
     duration is the program's; the port writes Rust's function-scope `static mut` with C's own
     value (Deviation 30's device for the same C shape).  */
  static mut source: *const c_char = c"Gdk".as_ptr ();
  let mut monitors: *mut MonitorInfo;                                      // C 2431

  block_input ();                                                          // C 2433
  gdpy = (*dpyinfo).gdpy;                                                  // C 2434
  n_monitors = gdk_display_get_n_monitors (gdpy);                          // C 2435
  monitor_frames = make_nil_vector (n_monitors as EMACS_INT);              // C 2436
  monitors = xcalloc (n_monitors as size_t,                                // C 2437
		      std::mem::size_of::<MonitorInfo> ()) as *mut MonitorInfo;

  FOR_EACH_FRAME! (rest, frame, {                                          // C 2439
      let f: *mut frame = XFRAME (frame);                                  // C 2441

      if FRAME_PGTK_P (f)                                                  // C 2443
	  && FRAME_DISPLAY_INFO! (f) == dpyinfo
	  && !FRAME_TOOLTIP_P (f)
	{
	  let gwin: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_WIDGET! (f));   // C 2447

          i = 0;
          while i < n_monitors                                            // C 2449
	    {
	      if gdk_display_get_monitor_at_window (gdpy, gwin)               // C 2450-2451
		  == gdk_display_get_monitor (gdpy, i)
		{ break; }
	      i += 1;
	    }
	  ASET (monitor_frames, i as EMACS_INT,                               // C 2453
		Fcons (frame, AREF (monitor_frames, i as EMACS_INT)));
	}
    });

  i = 0;
  while i < n_monitors                                                     // C 2457
    {
      let mut width_mm: gint;                                              // C 2459
      let mut height_mm: gint;
      let mut rec: GdkRectangle = std::mem::zeroed ();                     // C 2460
      let mut work: GdkRectangle = std::mem::zeroed ();
      /* C 2461's `struct MonitorInfo *mi = &monitors[i];` - the port's element address.  */
      let mi: *mut MonitorInfo = monitors.offset (i as isize);
      let mut scale: c_double = 1.0;                                       // C 2462

      let monitor: *mut GdkMonitor = gdk_display_get_monitor (gdpy, i);     // C 2464
      if gdk_monitor_is_primary (monitor) != 0                             // C 2465
	{ primary_monitor = i; }                                           // C 2466
      gdk_monitor_get_geometry (monitor, &raw mut rec);                    // C 2467

      width_mm = gdk_monitor_get_width_mm (monitor);                       // C 2469
      height_mm = gdk_monitor_get_height_mm (monitor);                     // C 2470
      gdk_monitor_get_workarea (monitor, &raw mut work);                   // C 2471

      /* GTK returns scaled sizes for the workareas.  */
      scale = pgtk_get_monitor_scale_factor (gdk_monitor_get_model (monitor));   // C 2474
      if scale == 0.0                                                      // C 2475
	{ scale = gdk_monitor_get_scale_factor (monitor) as c_double; }     // C 2476
      rec.x = (rec.x as c_double * scale + 0.5) as c_int;                  // C 2477
      rec.y = (rec.y as c_double * scale + 0.5) as c_int;                  // C 2478
      rec.width = (rec.width as c_double * scale + 0.5) as c_int;          // C 2479
      rec.height = (rec.height as c_double * scale + 0.5) as c_int;        // C 2480
      work.x = (work.x as c_double * scale + 0.5) as c_int;                // C 2481
      work.y = (work.y as c_double * scale + 0.5) as c_int;                // C 2482
      work.width = (work.width as c_double * scale + 0.5) as c_int;        // C 2483
      work.height = (work.height as c_double * scale + 0.5) as c_int;      // C 2484

      (*mi).geom.x = rec.x;                                               // C 2486
      (*mi).geom.y = rec.y;                                               // C 2487
      (*mi).geom.width = rec.width as c_uint;                             // C 2488
      (*mi).geom.height = rec.height as c_uint;                           // C 2489
      (*mi).work.x = work.x;                                              // C 2490
      (*mi).work.y = work.y;                                              // C 2491
      (*mi).work.width = work.width as c_uint;                            // C 2492
      (*mi).work.height = work.height as c_uint;                          // C 2493
      (*mi).mm_width = width_mm;                                          // C 2494
      (*mi).mm_height = height_mm;                                        // C 2495
      (*mi).scale_factor = scale;                                         // C 2496

      dupstring (&raw mut (*mi).name, gdk_monitor_get_model (monitor));   // C 2498
      i += 1;
    }

  attributes_list = make_monitor_attribute_list (monitors,                 // C 2501-2505
						 n_monitors,
						 primary_monitor,
						 monitor_frames,
						 source);
  free_monitors (monitors, n_monitors);                                   // C 2506
  unblock_input ();                                                       // C 2507

  return attributes_list;                                                 // C 2509
});

/* C 2512-2531: `pgtk_frame_scale_factor` - the frame's monitor scale, the same lookup the two
   scaling walks make (C's own `double` return).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_frame_scale_factor (f: *mut frame) -> c_double
{
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);          // C 2515
  let gdpy: *mut GdkDisplay = (*dpyinfo).gdpy;                            // C 2516

  block_input ();                                                         // C 2518

  let gwin: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_WIDGET! (f));   // C 2520
  let gmon: *mut GdkMonitor = gdk_display_get_monitor_at_window (gdpy, gwin);  // C 2521

  /* GTK returns scaled sizes for the workareas.  */
  let mut scale: c_double = pgtk_get_monitor_scale_factor (gdk_monitor_get_model (gmon));   // C 2524
  if scale == 0.0                                                         // C 2525
    { scale = gdk_monitor_get_scale_factor (gmon) as c_double; }          // C 2526

  unblock_input ();                                                       // C 2528

  return scale;                                                           // C 2530
}

/* C 2533-2539: `x-display-planes` and C 2542-2549: `x-display-color-cells` - the group's last two
   subrs; C 2548's `1 << min (dpyinfo->n_planes, 24)` keeps C's shift and folds the mirror's `min`
   over two `isize`s (the `x-display-mm-*` rule, Deviation 38).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-planes", Fx_display_planes, Sx_display_planes,   // C 2533-2539
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  check_pgtk_display_info (terminal);                           // C 2537
  return make_fixnum (32);                                      // C 2538
});

#[cfg (feature = "have_pgtk")]
DEFUN! ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells,   // C 2542-2549
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  let dpyinfo: *mut pgtk_display_info = check_pgtk_display_info (terminal);   // C 2546
  /* We force 24+ bit depths to 24-bit to prevent an overflow.  */
  return make_fixnum (((1 as isize) << min ((*dpyinfo).n_planes as isize, 24))   // C 2548
		      as EMACS_INT);
});

/***********************************************************************
				Tool tips
 ***********************************************************************/

/* C 2555-2573: the group's six file-scope statics.  C's are *tentative* definitions, i.e. zero
   (PORTING.md 0.1-13), which is what the port writes; each keeps C's own comment above it in this
   file's `monitor_scale_factor_alist` shape.  */
/* The frame of the currently visible tooltip.  */
#[cfg (feature = "have_pgtk")]
static mut tip_frame: Lisp_Object = std::ptr::null_mut ();                // C 2556

/* The window-system window corresponding to the frame of the
   currently visible tooltip.  */
#[cfg (feature = "have_pgtk")]
static mut tip_window: *mut GtkWidget = std::ptr::null_mut ();            // C 2560

/* A timer that hides or deletes the currently visible tooltip when it
   fires.  */
#[cfg (feature = "have_pgtk")]
static mut tip_timer: Lisp_Object = std::ptr::null_mut ();                // C 2564

/* STRING argument of last `x-show-tip' call.  */
#[cfg (feature = "have_pgtk")]
static mut tip_last_string: Lisp_Object = std::ptr::null_mut ();          // C 2567

/* Normalized FRAME argument of last `x-show-tip' call.  */
#[cfg (feature = "have_pgtk")]
static mut tip_last_frame: Lisp_Object = std::ptr::null_mut ();           // C 2570

/* PARMS argument of last `x-show-tip' call.  */
#[cfg (feature = "have_pgtk")]
static mut tip_last_parms: Lisp_Object = std::ptr::null_mut ();           // C 2573

/// C: C 2576-2577: `static void unwind_create_tip_frame (Lisp_Object frame)` - C's `static void`,
/// spelled `unsafe extern "C" fn` because C 2625 passes it to `record_unwind_protect` whose
/// parameter is `Option<unsafe extern "C" fn (Lisp_Object)>` (the `do_unwind_create_frame` device
/// above).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn unwind_create_tip_frame (frame: Lisp_Object)
{
  let deleted: Lisp_Object;                                     // C 2579

  deleted = unwind_create_frame (frame);                        // C 2581
  if EQ (deleted, Qt ())                                        // C 2582
    {
      tip_window = std::ptr::null_mut ();                       // C 2584
      tip_frame = Qnil ();                                      // C 2585
    }
}

/* C 2590-2597's comment:

   Create a frame for a tooltip on the display described by DPYINFO.
   PARMS is a list of frame parameters.  TEXT is the string to
   display in the tip frame.  Value is the frame.

   Note that functions called here, esp. gui_default_parameter can
   signal errors, for instance when a specified color name is
   undefined.  We have to make sure that we're in a consistent state
   when this happens.  */

/// C: C 2599-2600: `static Lisp_Object pgtk_create_tip_frame (struct pgtk_display_info *dpyinfo,
/// Lisp_Object parms, struct frame *p)` - C's `static Lisp_Object`, and the same devices
/// `Fx_create_frame` above uses (`output_method!`, the `FRAME_X_OUTPUT!` member writes,
/// `explicit_name!`, the written-out conversions).  C 2606 reads the `face_change` global
/// (`include/dispextern.rs` 2915) and C 2639/2623/2814 write three frame bitfields through this
/// unit's own `tooltip!`/`wants_modeline!`/`no_split!` (Deviation 53).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_create_tip_frame (dpyinfo: *mut pgtk_display_info, parms: Lisp_Object,
				 p: *mut frame) -> Lisp_Object
{
  let mut f: *mut frame;                                        // C 2602
  let mut frame: Lisp_Object;                                   // C 2603
  let mut name: Lisp_Object;                                    // C 2604
  let count: specpdl_ref = SPECPDL_INDEX ();                    // C 2605
  let face_change_before: bool = face_change;                   // C 2606
  let mut parms = parms;                                        // C 2611 reassigns the parameter.

  if (*((*dpyinfo).terminal)).name.is_null ()                   // C 2608
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  parms = Fcopy_alist (parms);                                  // C 2611

  /* Get the name of the frame to use for resource lookup.  */
  name = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qname (), c"name".as_ptr (),
			      c"Name".as_ptr (), RES_TYPE_STRING);          // C 2614-2615
  if !STRINGP (name)                                            // C 2616
      && !BASE_EQ (name, Qunbound ())
      && !NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  frame = Qnil ();                                              // C 2621
  f = make_frame (false);                                       // C 2622
  wants_modeline! (f, false);                                   // C 2623
  XSETFRAME! (frame, f);                                        // C 2624
  record_unwind_protect (Some (unwind_create_tip_frame), frame);   // C 2625

  (*f).terminal = (*dpyinfo).terminal;                          // C 2627

  /* By setting the output method, we're essentially saying that
     the frame is live, as per FRAME_LIVE_P.  If we get a signal
     from this point on, x_destroy_window might screw up reference
     counts etc.  */
  output_method! (f, output_pgtk);                              // C 2633
  (*f).output_data.pgtk = xzalloc (std::mem::size_of::<pgtk_output> ());   // C 2634
  FRAME_FONTSET! (f, -1);                                       // C 2635
  (*FRAME_X_OUTPUT! (f)).white_relief.pixel = (-1 as c_long) as c_ulong;   // C 2636
  (*FRAME_X_OUTPUT! (f)).black_relief.pixel = (-1 as c_long) as c_ulong;   // C 2637

  tooltip! (f, true);                                           // C 2639
  fset_icon_name (f, Qnil ());                                  // C 2640
  /* C 2641: `FRAME_DISPLAY_INFO (f) = dpyinfo;` - the same l-value case as `Fx_create_frame`'s.  */
  (*FRAME_X_OUTPUT! (f)).display_info = dpyinfo;
  (*FRAME_X_OUTPUT! (f)).parent_desc = (*FRAME_DISPLAY_INFO! (f)).root_window;   // C 2642
  (*FRAME_X_OUTPUT! (f)).explicit_parent = false as c_char;    // C 2643

  /* These colors will be set anyway later, but it's important
     to get the color reference counts right, so initialize them!  */
  {
    let black: Lisp_Object;                                     // C 2648

    /* Function x_decode_color can signal an error.  Make
       sure to initialize color slots so that we won't try
       to free colors we haven't allocated.  */
    FRAME_FOREGROUND_PIXEL! (f, (-1 as c_long) as c_ulong);     // C 2653
    FRAME_BACKGROUND_PIXEL! (f, (-1 as c_long) as c_ulong);     // C 2654
    (*FRAME_X_OUTPUT! (f)).border_pixel = (-1 as c_long) as c_ulong;   // C 2655

    black = build_string (c"black".as_ptr ());                  // C 2657
    FRAME_FOREGROUND_PIXEL! (f, x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong);   // C 2658-2659
    FRAME_BACKGROUND_PIXEL! (f, x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong);   // C 2660-2661
    (*FRAME_X_OUTPUT! (f)).border_pixel
      = x_decode_color (f, black, BLACK_PIX_DEFAULT! (f)) as c_ulong;   // C 2662-2663
  }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)                 // C 2668
    {
      fset_name (f, build_string ((*dpyinfo).x_id_name));       // C 2670
      explicit_name! (f, false);                                // C 2671
    }
  else
    {
      fset_name (f, name);                                      // C 2675
      explicit_name! (f, true);                                 // C 2676
      /* use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);                     // C 2678
    }

  register_font_driver (&raw const ftcrfont_driver as *const c_void, f);   // C 2681
  #[cfg (feature = "harfbuzz")]
  { register_font_driver (&raw const ftcrhbfont_driver as *const c_void, f); }   // C 2683

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),   // C 2686-2687
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values that are
     needed to determine window geometry.  */
  pgtk_default_font_parameter (f, parms);                       // C 2691

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (0),
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);                      // C 2693-2694

  /* This defaults to 2 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP (Fassq (Qinternal_border_width (), parms))            // C 2699
    {
      let value: Lisp_Object;                                   // C 2701

      value = gui_display_get_arg (dpyinfo as *mut c_void, parms, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);            // C 2703-2705
      if ! BASE_EQ (value, Qunbound ())                         // C 2706
	{ parms = Fcons (Fcons (Qinternal_border_width (), value), parms); }   // C 2707-2708
    }

  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (1),
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);                      // C 2711-2713
  gui_default_parameter (f, parms, Qright_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (),
			 RES_TYPE_NUMBER);                      // C 2714-2715
  gui_default_parameter (f, parms, Qbottom_divider_width (), make_fixnum (0),
			 std::ptr::null (), std::ptr::null (),
			 RES_TYPE_NUMBER);                      // C 2716-2717

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),
			 c"foreground".as_ptr (), c"Foreground".as_ptr (),
			 RES_TYPE_STRING);                      // C 2720-2721
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),
			 c"background".as_ptr (), c"Background".as_ptr (),
			 RES_TYPE_STRING);                      // C 2722-2723
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (),
			 RES_TYPE_STRING);                      // C 2724-2725
  gui_default_parameter (f, parms, Qcursor_color (), build_string (c"black".as_ptr ()),
			 c"cursorColor".as_ptr (), c"Foreground".as_ptr (),
			 RES_TYPE_STRING);                      // C 2726-2727
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (),
			 RES_TYPE_STRING);                      // C 2728-2729
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qnil (),
			 std::ptr::null (), std::ptr::null (),
			 RES_TYPE_BOOLEAN);                     // C 2730-2731

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);                                         // C 2736

  (*FRAME_X_OUTPUT! (f)).parent_desc = (*FRAME_DISPLAY_INFO! (f)).root_window;   // C 2738

  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (), c"InhibitDoubleBuffering".as_ptr (),
			 RES_TYPE_BOOLEAN);                     // C 2740-2742

  gui_figure_window_size (f, parms, false, false);              // C 2744

  xg_create_frame_widgets (f);                                  // C 2746
  pgtk_set_event_handler (f);                                   // C 2747
  tip_window = FRAME_GTK_OUTER_WIDGET! (f);                     // C 2748
  gtk_window_set_transient_for (GTK_WINDOW! (tip_window),       // C 2749-2750
				GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (p)));
  gtk_window_set_attached_to (GTK_WINDOW! (tip_window), FRAME_GTK_WIDGET! (p));   // C 2751
  gtk_window_set_destroy_with_parent (GTK_WINDOW! (tip_window), TRUE);            // C 2752
  gtk_window_set_decorated (GTK_WINDOW! (tip_window), FALSE);                     // C 2753
  gtk_window_set_type_hint (GTK_WINDOW! (tip_window),
			    GDK_WINDOW_TYPE_HINT_TOOLTIP);      // C 2754
  (*FRAME_X_OUTPUT! (f)).current_cursor = (*FRAME_X_OUTPUT! (f)).text_cursor;     // C 2755

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);                     // C 2757-2758
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);                     // C 2759-2760
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (),
			 RES_TYPE_SYMBOL);                      // C 2761-2762
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
			 c"alpha".as_ptr (), c"Alpha".as_ptr (),
			 RES_TYPE_NUMBER);                      // C 2763-2764
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);                      // C 2765-2766
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
			 c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);                      // C 2767-2769

  /* Add `tooltip' frame parameter's default value. */
  if NILP (Fframe_parameter (frame, Qtooltip ()))               // C 2772
    {
      AUTO_FRAME_ARG! (arg, Qtooltip (), Qt ());                // C 2774
      Fmodify_frame_parameters (frame, arg);                    // C 2775
    }

  /* FIXME - can this be done in a similar way to normal frames?
     https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html */

  /* Set the `display-type' frame parameter before setting up faces. */
  {
    let disptype: Lisp_Object;                                  // C 2783

    disptype = Qcolor ();                                       // C 2785

    if NILP (Fframe_parameter (frame, Qdisplay_type ()))        // C 2787
      {
	AUTO_FRAME_ARG! (arg, Qdisplay_type (), disptype);      // C 2789
	Fmodify_frame_parameters (frame, arg);                  // C 2790
      }
  }

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which lets the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  {
    let bg: Lisp_Object = Fframe_parameter (frame, Qbackground_color ());   // C 2803

    calln! (Qface_set_after_frame_default (), frame, Qnil ());   // C 2805

    if !EQ (bg, Fframe_parameter (frame, Qbackground_color ()))  // C 2807
      {
	AUTO_FRAME_ARG! (arg, Qbackground_color (), bg);         // C 2809
	Fmodify_frame_parameters (frame, arg);                   // C 2810
      }
  }

  no_split! (f, true);                                          // C 2814

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;              // C 2818
  (*((*f).terminal)).reference_count += 1;                      // C 2819

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons (frame, Vframe_list);                     // C 2824
  can_set_window_size! (f, true);                               // C 2825
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qtip_frame ());                   // C 2826-2827

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the clearing of
     all current matrices.  Since this isn't necessary here, avoid it
     by resetting face_change to the value it had before we created
     the tip frame.  */
  face_change = face_change_before;                             // C 2834

  /* Discard the unwind_protect.  */
  return unbind_to (count, frame);                              // C 2837
}

/* Compute where to display tip frame F.  PARMS is the list of frame
   parameters for F.  DX and DY are specified offsets from the current
   location of the mouse.  WIDTH and HEIGHT are the width and height
   of the tooltip.  Return coordinates relative to the root window of
   the display in *ROOT_X, and *ROOT_Y.  */

/// C: C 2846-2849: `static void compute_tip_xy (struct frame *f, Lisp_Object parms, Lisp_Object dx,
/// Lisp_Object dy, int width, int height, int *root_x, int *root_y)` - C's `static void`; the two
/// `int *` out-parameters keep C's pointer shape (`&raw mut` at the one call), and C 2886-2889's
/// four `XFIXNUM` reads go through C's `make_fixnum` arguments.  C's `int`-and-`EMACS_INT`
/// arithmetic is written out where C's usual conversions widen and the assignment narrows
/// (Deviation 58).
#[cfg (feature = "have_pgtk")]
unsafe fn compute_tip_xy (f: *mut frame, parms: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object,
			  width: c_int, height: c_int, root_x: *mut c_int, root_y: *mut c_int)
{
  let mut left: Lisp_Object;                                    // C 2851
  let mut top: Lisp_Object;
  let mut right: Lisp_Object;
  let mut bottom: Lisp_Object;
  let mut min_x: c_int;                                         // C 2852
  let mut min_y: c_int;
  let mut max_x: c_int;
  let mut max_y: c_int = -1;

  /* User-specified position?  */
  left = Fcdr (Fassq (Qleft (), parms));                        // C 2855
  top = Fcdr (Fassq (Qtop (), parms));                          // C 2856
  right = Fcdr (Fassq (Qright (), parms));                      // C 2857
  bottom = Fcdr (Fassq (Qbottom (), parms));                    // C 2858

  /* Move the tooltip window where the mouse pointer is.  Resize and
     show it.  */
  if (!FIXNUMP (left) && !FIXNUMP (right))                      // C 2862
      || (!FIXNUMP (top) && !FIXNUMP (bottom))                  // C 2863
    {
      let mut frame: Lisp_Object;                               // C 2865
      let mut attributes: Lisp_Object;
      let mut monitor: Lisp_Object;
      let mut geometry: Lisp_Object;
      let seat: *mut GdkSeat =
	gdk_display_get_default_seat ((*FRAME_DISPLAY_INFO! (f)).gdpy);   // C 2866-2867
      let dev: *mut GdkDevice = gdk_seat_get_pointer (seat);    // C 2868
      let mut scr: *mut GdkScreen;                              // C 2869

      block_input ();                                           // C 2871
      gdk_device_get_position (dev, &raw mut scr, root_x, root_y);   // C 2872
      unblock_input ();                                         // C 2873

      XSETFRAME! (frame, f);                                    // C 2875
      attributes = Fpgtk_display_monitor_attributes_list (frame);   // C 2876

      /* Try to determine the monitor where the mouse pointer is and
         its geometry.  See bug#22549.  */
      while CONSP (attributes)                                  // C 2880
	{
	  monitor = XCAR (attributes);                          // C 2882
	  geometry = Fassq (Qgeometry (), monitor);             // C 2883
	  if CONSP (geometry)                                   // C 2884
	    {
	      min_x = XFIXNUM (Fnth (make_fixnum (1), geometry)) as c_int;   // C 2886
	      min_y = XFIXNUM (Fnth (make_fixnum (2), geometry)) as c_int;   // C 2887
	      max_x = (min_x as EMACS_INT
		       + XFIXNUM (Fnth (make_fixnum (3), geometry))) as c_int;   // C 2888
	      max_y = (min_y as EMACS_INT
		       + XFIXNUM (Fnth (make_fixnum (4), geometry))) as c_int;   // C 2889
	      if min_x <= *root_x && *root_x < max_x                // C 2890
		  && min_y <= *root_y && *root_y < max_y            // C 2891
		{
		  break;                                           // C 2893
		}
	      max_y = -1;                                          // C 2895
	    }

	  attributes = XCDR (attributes);                          // C 2898
	}
    }

  /* It was not possible to determine the monitor's geometry, so we
     assign some sane defaults here: */
  if max_y < 0                                                  // C 2904
    {
      min_x = 0;                                                // C 2906
      min_y = 0;                                                // C 2907
      max_x = pgtk_display_pixel_width (FRAME_DISPLAY_INFO! (f));   // C 2908
      max_y = pgtk_display_pixel_height (FRAME_DISPLAY_INFO! (f));   // C 2909
    }

  if FIXNUMP (top)                                              // C 2912
    { *root_y = XFIXNUM (top) as c_int; }                       // C 2913
  else if FIXNUMP (bottom)                                     // C 2914
    { *root_y = (XFIXNUM (bottom) - height as EMACS_INT) as c_int; }   // C 2915
  else if *root_y as EMACS_INT + XFIXNUM (dy) <= min_y as EMACS_INT   // C 2916
    { *root_y = min_y; }		/* Can happen for negative dy */
  else if *root_y as EMACS_INT + XFIXNUM (dy) + height as EMACS_INT
	   <= max_y as EMACS_INT                               // C 2918
    /* It fits below the pointer */
    { *root_y = (*root_y as EMACS_INT + XFIXNUM (dy)) as c_int; }   // C 2920
  else if (height as EMACS_INT + XFIXNUM (dy) + min_y as EMACS_INT)
	   <= *root_y as EMACS_INT                             // C 2921
    /* It fits above the pointer.  */
    { *root_y = (*root_y as EMACS_INT - height as EMACS_INT
		 - XFIXNUM (dy)) as c_int; }                   // C 2923
  else
    /* Put it on the top.  */
    { *root_y = min_y; }                                       // C 2926

  if FIXNUMP (left)                                            // C 2928
    { *root_x = XFIXNUM (left) as c_int; }                     // C 2929
  else if FIXNUMP (right)                                      // C 2930
    { *root_x = (XFIXNUM (right) - width as EMACS_INT) as c_int; }   // C 2931
  else if *root_x as EMACS_INT + XFIXNUM (dx) <= min_x as EMACS_INT   // C 2932
    { *root_x = 0; }		/* Can happen for negative dx */
  else if *root_x as EMACS_INT + XFIXNUM (dx) + width as EMACS_INT
	   <= max_x as EMACS_INT                               // C 2934
    /* It fits to the right of the pointer.  */
    { *root_x = (*root_x as EMACS_INT + XFIXNUM (dx)) as c_int; }   // C 2936
  else if (width as EMACS_INT + XFIXNUM (dx) + min_x as EMACS_INT)
	   <= *root_x as EMACS_INT                             // C 2937
    /* It fits to the left of the pointer.  */
    { *root_x = (*root_x as EMACS_INT - width as EMACS_INT
		 - XFIXNUM (dx)) as c_int; }                   // C 2939
  else
    /* Put it left justified on the screen -- it ought to fit that way.  */
    { *root_x = min_x; }                                       // C 2942
}

/* Hide tooltip.  Delete its frame if DELETE is true.  */

/// C: C 2947-2948: `static Lisp_Object pgtk_hide_tip (bool delete)` - C's `static Lisp_Object`;
/// C 2952's `calln (Qcancel_timer, tip_timer)` is the port's `calln!` and C 2961-2966's guard
/// keeps C's five conditions in C's order.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_hide_tip (delete: bool) -> Lisp_Object
{
  if !NILP (tip_timer)                                          // C 2950
    {
      calln! (Qcancel_timer (), tip_timer);                     // C 2952
      tip_timer = Qnil ();                                      // C 2953
    }

  /* Any GTK+ system tooltip can be found via the x_output structure of
     tip_last_frame, provided that frame is still live.  Any Emacs
     tooltip is found via the tip_frame variable.  Note that the current
     value of x_gtk_use_system_tooltips might not be the same as used
     for the tooltip we have to hide, see Bug#30399.  */
  if (NILP (tip_last_frame) && NILP (tip_frame))                // C 2961
      || (!use_system_tooltips                                  // C 2962
	  && !delete                                             // C 2963
	  && FRAMEP (tip_frame)                                  // C 2964
	  && FRAME_LIVE_P (XFRAME (tip_frame))                   // C 2965
	  && !FRAME_VISIBLE_P (XFRAME (tip_frame)))              // C 2966
    /* Either there's no tooltip to hide or it's an already invisible
       Emacs tooltip and we don't want to change its type.  Return
       quickly.  */
    { return Qnil (); }                                         // C 2970
  else
    {
      let mut was_open: Lisp_Object = Qnil ();                  // C 2973

      let count: specpdl_ref = SPECPDL_INDEX ();                // C 2975
      specbind (Qinhibit_redisplay (), Qt ());                  // C 2976
      specbind (Qinhibit_quit (), Qt ());                       // C 2977

      /* Try to hide the GTK+ system tip first.  */
      if FRAMEP (tip_last_frame)                                // C 2980
	{
	  let f: *mut frame = XFRAME (tip_last_frame);          // C 2982

	  if FRAME_LIVE_P (f)                                   // C 2984
	    {
	      if xg_hide_tooltip (f)                            // C 2986
		{ was_open = Qt (); }                           // C 2987
	    }
	}

      /* When using GTK+ system tooltips (compare Bug#41200) reset
	 tip_last_frame.  It will be reassigned when showing the next
	 GTK+ system tooltip.  */
      if use_system_tooltips                                    // C 2994
	{ tip_last_frame = Qnil (); }                           // C 2995

      /* Now look whether there's an Emacs tip around.  */
      if FRAMEP (tip_frame)                                     // C 2998
	{
	  let f: *mut frame = XFRAME (tip_frame);               // C 3000

	  if FRAME_LIVE_P (f)                                   // C 3002
	    {
	      if delete || use_system_tooltips                   // C 3004
		{
		  /* Delete the Emacs tooltip frame when DELETE is true
		     or we change the tooltip type from an Emacs one to
		     a GTK+ system one.  */
		  delete_frame (tip_frame, Qnil ());            // C 3009
		  tip_frame = Qnil ();                          // C 3010
		}
	      else
		{ pgtk_make_frame_invisible (f); }              // C 3013

	      was_open = Qt ();                                 // C 3015
	    }
	  else
	    { tip_frame = Qnil (); }                            // C 3018
	}
      else
	{ tip_frame = Qnil (); }                                // C 3021

      return unbind_to (count, was_open);                       // C 3023
    }
}
