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
// **C 1-1147**, which ends at the `}` of `update_watched_scale_factor` (C 1133-1147);
// **the next live definition is the `DEFUN ("pgtk-set-monitor-scale-factor", …)` at C 1155** - the
// first item of the next lane's slice (C 1149-1154 is the "Lisp definitions" banner, C 1148 and
// C 1147's `}` close this slice).
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
const RES_TYPE_STRING: resource_types = 3;          // C: dispextern.h 3988

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
unsafe fn update_watched_scale_factor (timer: *mut atimer)
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
