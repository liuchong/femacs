// femacs: port of src/gtkutil.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `gtkutil.o`
// joins `GTK_OBJ` only in the `USE_GTK` branch of the build (`configure.ac` 3827's
// `GTK_OBJ="gtkutil.o $GTK_OBJ"`, under the `emacs_cv_gtk_compiles = yes` arm whose 3825
// `AC_DEFINE ([USE_GTK], [1])` is what opens this file's own `#ifdef USE_GTK`, and
// `emacs/src/Makefile.in` 182/452 puts `$(GTK_OBJ)` in the object list), `USE_GTK` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 3861, and
// `docs/baseline/frozen-build-sources.txt` lists no `gtkutil.o` for this target - so the text can
// never be *compiled* here.  Note the difference from the class P-86 was written for: this file's
// own conditions are all *decided* (`python3 tools/check-cond-eval.py src/gtkutil.c` ->
// `212 conditions, 1 unknown, 1 disagreements [NOT VERIFIED]`, the unknown and the disagreement
// both at C 6652, far past this slice), so `extract-abi.py`'s checklist is **not** empty and the
// `--check` reading in the ledger's Evidence is not vacuous.  What the ledger adds is P-86 ②'s
// substitute shape - the **C line-range table**, accounting for every line of the reference -
// beside the `## Evidence` readings and the `## Deviations`.
//
// **This batch is P-86 ④'s first slice: C 1-1296.**  It ends at the `}` of `xg_frame_set_char_size`
// (C 1176-1296); **the next live definition is `xg_frame_set_size_and_position` (C 1316-1397)**,
// the first item of the next lane's slice (C 1313-1315 is its comment and `void` line, and
// C 1301-1311 is the `#if 0` block holding `xg_height_or_width_changed` - dead text C itself
// excludes).
//
//   * the **file gate** is `feature = "use_gtk"` (`USE_GTK`, `configure.ac` 3825's `AC_DEFINE`),
//     on every item *and on the include graph* (P-86 ⑧);
//   * the slice's own conditions are C 33's `#ifndef HAVE_PGTK` (feature `have_pgtk`), C 51's
//     `#ifdef HAVE_XINPUT2`, C 55's `#ifdef HAVE_XFT`, C 59-64's `#ifdef HAVE_GTK3` pair, C 66's
//     `#ifdef HAVE_XDBE`, C 104/1023/1065/1104's `GTK_CHECK_VERSION (...)` (below), C 108's
//     `#ifndef HAVE_GTK3`, C 110's `#ifdef HAVE_FREETYPE`, C 142's `#ifdef HAVE_XWIDGETS`, C 204's
//     `#ifndef HAVE_PGTK`, C 237/246/259's three `#ifndef HAVE_PGTK` declarators, C 270/287/290's
//     version/GTK3 pair, C 306/331/366/372's `#ifndef HAVE_PGTK` blocks in the display functions,
//     C 428's `#if defined USE_CAIRO && !defined HAVE_GTK3` and C 512/544/552's cairo/GTK3 pair -
//     all decided by the frozen table, and **both arms of every one of them are carried**;
//   * `GTK_CHECK_VERSION (major, minor, micro)` is decided by the reference's *own* text, not by a
//     `configure` macro: `xterm.h` 73-75 is `#ifndef USE_GTK / #define GTK_CHECK_VERSION(i, j, k)
//     false / #endif`, the definition the frozen configuration evaluates (a `use_gtk` build takes
//     GTK's own `gtk/gtkversion.h`).  The port's arms are the derived-condition features
//     `gtk_check_version_ge_3_10` and `gtk_check_version_ge_3_22` (the `android_api_ge_9` /
//     `android_api_le_11` naming, `docs/baseline/feature-coverage.tsv`), **neither registered in
//     this tree** (see `## Owed`);
//   * the include graph reaches **placeholders**: `include/gtkutil.rs` (the whole `xg_*` prototype
//     block and the three `xg_*_cb_data` records, which is why this unit declares the pieces it
//     uses at their C position - P-72 rule 4) and `include/pgtkterm.rs` (the pgtk records, which the
//     `have_pgtk` arms need).  `docs/ports/gtkutil.md` owes those moves.
//
// **The toolkit surface is declared in this unit (P-88 ①, and P-90 keeps that allowance for this
// round), and this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
// `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` -> both absent; `pkg-config`
// is not installed; no `gtk/gtk.h` or `gdk/gdk.h` under `/opt/homebrew`, `/usr/local` or `/Library`.
// So **GDK and GTK** records and entry points cite the header alone (no line, no probe) and every
// size, offset and enum width of theirs is UNVERIFIED - no arm flip is possible and no layout
// assertion is written for them.  **GLib/GObject, cairo and X11 *are* on this machine**
// (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0, with the generated `glibconfig.h` under
// `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`; `/opt/homebrew/include/cairo`, cairo
// 1.18.4; `/opt/homebrew/include/X11`), so their declarations quote the header and line they were
// read from and `GList`'s layout is a real clang reading (Evidence ②).  P-88 ⑤/P-90 ③: the move
// into a dedicated SDK mirror is owed (`## Owed`), and this unit's copy is the one to drop then.
//
// `docs/ports/gtkutil.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no GTK
// tree to compile the arm against and no GTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>`

/* C 22: `#ifdef USE_GTK` - the file gate.  C 23 through the end of the file sit inside it, and
   every `include!` below carries it (P-86 ⑧).  */

#[cfg (feature = "use_gtk")]
include!("../lib/include/float.in.rs");        // C 23: `#include <float.h>`
#[cfg (feature = "use_gtk")]
include!("../lib/include/stdio.rs");           // C 24: `#include <stdio.h>`
#[cfg (feature = "use_gtk")]
include!("../lib/include/stdlib.rs");          // C 25: `#include <stdlib.h>`
#[cfg (feature = "use_gtk")]
include!("../lib/include/c-ctype.rs");         // C 27: `#include <c-ctype.h>`

/* C 29-32: the four C headers this slice's records and calls come from.  Each `include!`
   carries the file gate (P-86 ⑧); `lisp.h` is first, C's own order (PORTING.md 0.1-14).  */
#[cfg (feature = "use_gtk")]
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
/* C 29's `"lisp.h"` chain (lisp.h 30/33): `<string.h>` for `strlen`/`strcpy`/`memcpy`/`strstr`
   and `<limits.h>` for `INT_MAX` (C 277/696/700/719).  `include/lisp.rs` carries neither mirror
   (its own note records the intprops/sys-limits omission), so the two edges are here, at the C
   position that makes the names visible.  */
#[cfg (feature = "use_gtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
#[cfg (feature = "use_gtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>`

#[cfg (feature = "use_gtk")]
include!("include/dispextern.rs");             // C 30: `#include "dispextern.h"`
#[cfg (feature = "use_gtk")]
include!("include/frame.rs");                  // C 31: `#include "frame.h"`
#[cfg (feature = "use_gtk")]
include!("include/systime.rs");                // C 32: `#include "systime.h"`

/* C 33-41.  The two arms of C's `#ifndef HAVE_PGTK` are the two `include!`s and the two `xp_output`
   typedefs below; `xterm.h`'s mirror is `include/xterm.rs`, `xsettings.h`'s is
   `include/xsettings.rs`.  C 35/38's `#define xp x` / `#define xp pgtk` is a *token* macro C uses
   inside an expression (`f->output_data.xp`), which Rust cannot spell: every use site writes the
   union member C's own `#define` selects (one `## Deviations` entry).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
include!("include/xterm.rs");                  // C 34: `#include "xterm.h"`
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
type xp_output = x_output;                     // C 36: `typedef struct x_output xp_output;`
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
include!("include/xsettings.rs");              // C 39: `#include "xsettings.h"`
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
type xp_output = pgtk_output;                  // C 40: `typedef struct pgtk_output xp_output;`

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/emacsgtkfixed.rs");          // C 63: `#include "emacsgtkfixed.h"`

/* C 42-47: the rest of the C headers.  `gtkutil.h`'s mirror is a placeholder (it carries the
   include guard alone), so the `xg_*` names this unit uses are declared where C's text uses them -
   `xg_list_node` at C 619-630 and `xg_wm_set_size_hint` at C 1209 (P-72 rule 4), both owed to that
   mirror; `window.h`, `termhooks.h`, `keyboard.h` and `coding.h`'s mirrors carry this slice's
   `SSDATA`/`ENCODE_FILE`/`ENCODE_UTF_8`.  */
#[cfg (feature = "use_gtk")]
include!("include/blockinput.rs");             // C 42: `#include "blockinput.h"`
#[cfg (feature = "use_gtk")]
include!("include/window.rs");                 // C 43: `#include "window.h"`
#[cfg (feature = "use_gtk")]
include!("include/gtkutil.rs");                // C 44: `#include "gtkutil.h"`
/* C 44's chain: `"gtkutil.h"` 27's `#include "../lwlib/lwlib-widget.h"`, which declares the
   `widget_value` record C 2289 onward takes.  The mirror is `femacs/lwlib/include/lwlib-widget.rs`
   (the `src/pgtkmenu.rs` 95 device).  */
#[cfg (feature = "use_gtk")]
include!("../lwlib/include/lwlib-widget.rs");
#[cfg (feature = "use_gtk")]
include!("include/termhooks.rs");              // C 45: `#include "termhooks.h"`
#[cfg (feature = "use_gtk")]
include!("include/keyboard.rs");               // C 46: `#include "keyboard.h"`
#[cfg (feature = "use_gtk")]
include!("include/coding.rs");                 // C 47: `#include "coding.h"`

/* C 49-68: the third-party headers.  `<gdk/gdkkeysyms.h>` (C 49) is GDK's, and GDK's and GTK's
   names are the section marked `C 44/49's chain` below; `<gtk/gtk.h>` itself arrives through
   `"gtkutil.h"` (C 44, whose C 26 includes it) - which is why the section stands here, at the first
   direct third-party include, and holds the whole surface.  */
// C 49: `#include <gdk/gdkkeysyms.h>` (GDK; absent from the tree and from this machine)
/* C 51-53: `#ifdef HAVE_XINPUT2` / `#include <X11/extensions/XInput2.h>` / `#endif`.  XInput2 is
   `undef` in the frozen table; the header is X11's own (an SDK surface whose mirror is
   `include/xlib.rs`, which this slice reaches through `include/xterm.rs`), and this slice takes no
   name from it - C 1033 uses the *feature* alone.  */
/* C 55-57: `#ifdef HAVE_XFT` / `#include <X11/Xft/Xft.h>` / `#endif`.  `HAVE_XFT` is `undef`, and
   this slice takes no name from Xft.  */
/* C 59-64: `#ifdef HAVE_GTK3` / `#ifndef HAVE_PGTK` / `#include <gtk/gtkx.h>` / `#endif` /
   `#include "emacsgtkfixed.h"` / `#endif`.  `gtkx.h` is GTK's (nothing in this slice comes from
   it); `emacsgtkfixed.h`'s mirror is `include/emacsgtkfixed.rs` and this slice takes
   `emacs_fixed_new`'s family from it in a later slice, not here.  */
/* C 66-68: `#ifdef HAVE_XDBE` / `#include <X11/extensions/Xdbe.h>` / `#endif`.  `HAVE_XDBE` is
   `undef`, and this slice takes no name from Xdbe.  */

/* ----------------------------------------------------------------------------------------------
   C 44/49's chain: `<gtk/gtk.h>` (through `"gtkutil.h"` 26; C 44) and `<gdk/gdkkeysyms.h>`
   (C 49), plus the GLib/GObject and cairo surfaces those pull in - and, under `HAVE_PGTK`,
   `"gtkutil.h"` 29's `"pgtkterm.h"`.  The third-party declarations now live in their single
   mirror, `femacs/src/include/gtk.rs` (P-90 ①/②/③; the P-88 ⑤ move-back took them out of this
   unit), and the pgtk records/accessors in `include/pgtkterm.rs`; the two edges below reach them
   at C's own include position.  The header mirrors that own those `#include`s carry the same
   edges (`include/pgtkterm.rs` C 31); `include/gtkutil.rs`'s C 26 edge is owed - see `## Owed`
   in `docs/ports/gtkutil.md`.  GLib, cairo and X11 *are* on this machine, so their declarations
   quote the header and line and `GList`/`GError`/`GType`'s layouts are real clang readings;
   **GDK and GTK are not**, so their citations name the header alone and every value of theirs is
   a placeholder (P-88 ②, `UNVERIFIED`).  */
#[cfg (feature = "use_gtk")]
include!("include/gtk.rs");                    // C 44's chain: `"gtkutil.h"` 26's `<gtk/gtk.h>`
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // C 44's chain: `"gtkutil.h"` 29's `"pgtkterm.h"`
/* X11 names this slice calls that `include/xlib.rs` does not carry yet (it declares the ones its
   own arms needed), plus the five `XSizeHints` flag constants C 1062-1112 tests.  X11 *is* on this
   machine (`/opt/homebrew/include/X11`), so each quotes its line, and each is owed to that mirror
   (the `src/xfns.rs` 210-218 shape).  */
/// C: Xutil.h 102/103/105: `#define USPosition (1L << 0)` / `#define USSize (1L << 1)` /
/// `#define PPosition (1L << 2)`, and Xutil.h 74/75: `#define XNegative 0x0010` /
/// `#define YNegative 0x0020`.  `size_hint_flags` is C's `int` (frame.h 610), so every use site
/// writes C's usual-arithmetic conversion.
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const USPosition: c_long = 1 << 0;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const PPosition: c_long = 1 << 2;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const XNegative: c_int = 0x0010;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const YNegative: c_int = 0x0020;
/// C: X.h 607: `#define XYPixmap 1 /* depth == drawable depth */` - C 398/403/416's `XGetImage`
/// format argument - and X.h 201: `#define ConfigureNotify 22`, C 1274's `x_wait_for_event`
/// argument.  `include/xlib.rs`'s `x11_constants` carries neither (`include/xterm.rs` 103's
/// `use` list is the mechanism the mirror uses for the eleven it needs), so both stand here and
/// `## Owed` records the move.
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const XYPixmap: c_int = 1;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const ConfigureNotify: c_int = 22;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" {
  /// C 393-394/397-398/402-403: `XGetGeometry (FRAME_X_DISPLAY (f), pix, &wunused, &iunused,
  /// &iunused, &width, &height, &uunused, &depth);` / `XImage *xim = XGetImage (…);`
  /// C: Xlib.h 2630-2640: `extern Status XGetGeometry (Display *, Drawable, Window *, int *,
  /// int *, unsigned int *, unsigned int *, unsigned int *, unsigned int *);`
  pub fn XGetGeometry (display: *mut Display, d: Drawable, root_return: *mut Window,
                       x_return: *mut c_int, y_return: *mut c_int,
                       width_return: *mut c_uint, height_return: *mut c_uint,
                       border_width_return: *mut c_uint, depth_return: *mut c_uint) -> c_int;
}

/* X11 names the second slice (C 1297-2281) calls that `include/xlib.rs` does not carry yet, plus
   the `XSizeHints` flag and the ten `*Gravity` ids its common bodies test.  X11 *is* on this
   machine (`/opt/homebrew/include/X11`), so each quotes its line, and each is owed to that mirror
   (the `src/xfns.rs` 210-218 shape).  */
/// C: Xutil.h 103: `#define USSize (1L << 1)` - C 1342's `flags |= USSize;` and C 2082's
/// `if (flags & USSize)`.  `flags`/`size_hint_flags` are C's `int` (frame.h 610), so every use
/// site writes C's usual-arithmetic conversion (with `USPosition` at Xutil.h 102, below).
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const USSize: c_long = 1 << 1;
/// C: X.h 422-431: `#define NorthWestGravity 1` / `NorthGravity 2` / `NorthEastGravity 3` /
/// `WestGravity 4` / `CenterGravity 5` / `EastGravity 6` / `SouthWestGravity 7` /
/// `SouthGravity 8` / `SouthEastGravity 9` / `StaticGravity 10` - the ten `win_gravity`
/// comparisons C 2054-2072 makes.  (A pgtk build reads the same ten from `pgtkgui.h`; the mirror
/// is the placeholder, so the arm is owed - `## Owed`.)
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const NorthWestGravity: c_int = 1;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const NorthGravity: c_int = 2;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const NorthEastGravity: c_int = 3;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const WestGravity: c_int = 4;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const CenterGravity: c_int = 5;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const EastGravity: c_int = 6;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const SouthWestGravity: c_int = 7;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const SouthGravity: c_int = 8;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const SouthEastGravity: c_int = 9;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
pub const StaticGravity: c_int = 10;
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" {
  /// C 1432: `if (XQueryColor (FRAME_X_DISPLAY (f), FRAME_X_COLORMAP (f), &xbg))`
  /// C: Xlib.h 2967-2969: `extern int XQueryColor (Display *, Colormap, XColor *);` (C's
  /// `Emacs_Color` is `XColor` in this arm, dispextern.h 46-48, so the pointer is C's).
  pub fn XQueryColor (display: *mut Display, colormap: Colormap, def_in_out: *mut Emacs_Color)
		      -> c_int;
  /// C 1736: `XVisualIDFromVisual (FRAME_X_VISUAL (f))`
  /// C: Xlib.h 1724-1726: `extern VisualID XVisualIDFromVisual (Visual *);`
  pub fn XVisualIDFromVisual (visual: *mut Visual) -> VisualID;
  /* C 2176's `XSync (FRAME_X_DISPLAY (f1), False);` declaration moved to `include/xlib.rs` in the
     sixty-sixth `xterm` slice (P-90 5's move-back): gtkutil.rs reaches that mirror and C's `False`
     is its `0` (Xlib.h 81), which is what the mirror's `Bool` parameter takes.  */
}

/* ----------------------------------------------------------------------------------------------
   C 70-146: this file's own declarations, in C's order.
   ---------------------------------------------------------------------------------------------- */

/* C 70-78: `#ifdef HAVE_GTK3` / the three dialog text ids / `#else` / GTK2's three stock ids /
   `#endif`.  The GTK3 arm's values are C's own string literals; the GTK2 arm's are
   gtk/gtkstock.h's macros, and no header on this machine defines them, so the arm is carried as
   C writes it - the three names are named by the GTK2 arm's bodies and are owed (`## Owed`).  */
/// C 71-73: `#define XG_TEXT_CANCEL "Cancel"` / `#define XG_TEXT_OK "OK"` /
/// `#define XG_TEXT_OPEN "Open"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! XG_TEXT_CANCEL { () => { c"Cancel" }; }
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! XG_TEXT_OK { () => { c"OK" }; }
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! XG_TEXT_OPEN { () => { c"Open" }; }
/// C 75-77: `#define XG_TEXT_CANCEL GTK_STOCK_CANCEL` / `#define XG_TEXT_OK GTK_STOCK_OK` /
/// `#define XG_TEXT_OPEN GTK_STOCK_OPEN`
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! XG_TEXT_CANCEL { () => { GTK_STOCK_CANCEL }; }
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! XG_TEXT_OK { () => { GTK_STOCK_OK }; }
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! XG_TEXT_OPEN { () => { GTK_STOCK_OPEN }; }

/* C 81-82: C's two `static` forward declarations inside the `#ifdef HAVE_GTK3` arm:

     static void emacs_menu_bar_get_preferred_width (GtkWidget *, gint *, gint *);
     static GType emacs_menu_bar_get_type (void);

   PORTING.md 0.1-12 drops forward declarations (Rust has none and its items are order-independent);
   `emacs_menu_bar_get_preferred_width`'s definition stays at C 166-177 below, and
   `emacs_menu_bar_get_type`'s belongs to C 94's `G_DEFINE_TYPE` (owed, below).  */

/// C 84-87: `typedef struct _EmacsMenuBar { GtkMenuBar parent; } EmacsMenuBar;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct EmacsMenuBar
{
  pub parent: GtkMenuBar,        // C: `GtkMenuBar parent;`
}
/// C 89-92: `typedef struct _EmacsMenuBarClass { GtkMenuBarClass parent; } EmacsMenuBarClass;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct EmacsMenuBarClass
{
  pub parent: GtkMenuBarClass,   // C: `GtkMenuBarClass parent;`
}

/* C 94: `G_DEFINE_TYPE (EmacsMenuBar, emacs_menu_bar, GTK_TYPE_MENU_BAR)`.  GLib's own definition
   is `gtype.h` 1840 (`#define G_DEFINE_TYPE(TN, t_n, T_P) G_DEFINE_TYPE_EXTENDED (TN, t_n, T_P, 0,
   {})`), whose `gtype.h` 1826-1839 documentation says the macro "declares a class initialization
   function, an instance initialization function ... and a static variable named `t_n_parent_class`
   pointing to the parent class.  Furthermore, it defines a `*_get_type()` function."  The
   expansion (`_G_DEFINE_TYPE_EXTENDED_BEGIN`/`_END`, gtype.h 2293/2317/2339/2348) is the type
   registration boilerplate and needs `GTK_TYPE_MENU_BAR`, `sizeof (EmacsMenuBar)`,
   `sizeof (EmacsMenuBarClass)`, `GTypeInfo` and `g_type_register_static_simple` - three of them
   GTK's, and GTK is not on this machine.  **PORTING.md has no entry for an item-defining macro of
   a third-party header** (its §11 owes one), so this unit carries C's invocation here and leaves
   the two names the expansion introduces that this slice reads and calls -
   `emacs_menu_bar_parent_class` (C 172) and `emacs_menu_bar_get_type` (C 182) - to it,
   `docs/ports/gtkutil.md` recording the expansion as owed.  */

/* C 97-102: the `#ifndef HAVE_PGTK` arm's four forward declarations (they are definitions at
   C 6360-6448, outside this slice), dropped per PORTING.md 0.1-12; their parameter types
   `GtkIMContext`, `GdkEvent` and `gtk_widget_key_press_event_cb`'s `bool` result are the ones the
   next slice's definitions carry.  */
/* C 104-106: `#if GTK_CHECK_VERSION (3, 10, 0)` / `static void xg_widget_style_updated (GtkWidget
   *, gpointer);` / `#endif`.  The version's frozen verdict is C's own `false` (xterm.h 73-75), and
   the declaration is dropped either way (0.1-12); the definition is C 6732-6752, outside this
   slice.  */

/* C 108-125: `#ifndef HAVE_GTK3` - the GTK2 arm's five compatibility macros.  C 110-117 is
   `#ifdef HAVE_FREETYPE`'s three font-chooser renames (plus the `#undef GTK_FONT_CHOOSER` above
   them); C 119-124 are the two widget constructors.  Each keeps C's name and C's body; the GTK2
   functions they call are GTK2's and owed (the GTK section above).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3"), feature = "have_freetype"))]
macro_rules! gtk_font_chooser_dialog_new {
  ($x:expr, $y:expr) => { gtk_font_selection_dialog_new ($x) };
}
/// C 113-114: `#undef GTK_FONT_CHOOSER` / `#define GTK_FONT_CHOOSER(x) GTK_FONT_SELECTION_DIALOG
/// (x)` - C's `#undef` has no Rust counterpart, so the port carries the second definition alone.
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3"), feature = "have_freetype"))]
macro_rules! GTK_FONT_CHOOSER {
  ($x:expr) => { ($x) as *mut GtkFontSelectionDialog };
}
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3"), feature = "have_freetype"))]
macro_rules! gtk_font_chooser_set_font {
  ($x:expr, $y:expr) => { gtk_font_selection_dialog_set_font_name ($x, $y) };
}
/// C 119-121: `#define gtk_box_new(ori, spacing) ((ori) == GTK_ORIENTATION_HORIZONTAL ?
/// gtk_hbox_new (FALSE, spacing) : gtk_vbox_new (FALSE, spacing))`
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! gtk_box_new {
  ($ori:expr, $spacing:expr) => {
    { if ($ori) == GTK_ORIENTATION_HORIZONTAL
        { gtk_hbox_new (FALSE, ($spacing)) }
      else { gtk_vbox_new (FALSE, ($spacing)) } }
  };
}
/// C 122-124: `#define gtk_scrollbar_new(ori, spacing) ((ori) == GTK_ORIENTATION_HORIZONTAL ?
/// gtk_hscrollbar_new (spacing) : gtk_vscrollbar_new (spacing))`
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! gtk_scrollbar_new {
  ($ori:expr, $spacing:expr) => {
    { if ($ori) == GTK_ORIENTATION_HORIZONTAL
        { gtk_hscrollbar_new (($spacing)) }
      else { gtk_vscrollbar_new (($spacing)) } }
  };
}

/// C 127: `#define XG_BIN_CHILD(x) gtk_bin_get_child (GTK_BIN (x))`
#[cfg (feature = "use_gtk")]
macro_rules! XG_BIN_CHILD {
  ($x:expr) => { gtk_bin_get_child (GTK_BIN! ($x)) };
}

/* C 129-130: `static void update_theme_scrollbar_width (void);` and the `_height` twin - forward
   declarations (definitions at C 4475 and C 4499, outside this slice), dropped per PORTING.md
   0.1-12.  */

/// C 132: `#define TB_INFO_KEY "xg_frame_tb_info"`
#[cfg (feature = "use_gtk")]
pub const TB_INFO_KEY: &core::ffi::CStr = c"xg_frame_tb_info";
/// C 133-140: `struct xg_frame_tb_info`.  C's record, whose `dir` member is GTK's
/// `GtkTextDirection` (above).
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct xg_frame_tb_info
{
  pub last_tool_bar: Lisp_Object,   // C: `Lisp_Object last_tool_bar;`
  pub style: Lisp_Object,           // C: `Lisp_Object style;`
  pub n_last_items: c_int,          // C: `int n_last_items;`
  pub hmargin: c_int,               // C: `int hmargin, vmargin;`
  pub vmargin: c_int,
  pub dir: GtkTextDirection,        // C: `GtkTextDirection dir;`
}

/// C 143: `bool xg_gtk_initialized; /* Used to make sure xwidget calls are possible */` - a
/// non-`static` definition, so the port exports C's symbol (PORTING.md 0.2).
#[cfg (all (feature = "use_gtk", feature = "have_xwidgets"))]
#[unsafe(no_mangle)]
pub static mut xg_gtk_initialized: bool = false;

/* C 146: `static GtkWidget *xg_get_widget_from_map (ptrdiff_t idx, Display *dpy);` - a forward
   declaration (the definitions are C 4430/4449, outside this slice), dropped per PORTING.md
   0.1-12.  */

/* C 150: `#ifdef HAVE_GTK3` - the arm that runs to C 185's `#endif`.  It holds the four functions
   `G_DEFINE_TYPE` (C 94) registers, and `emacs_menu_bar_get_preferred_width`'s signature is the
   `GtkWidgetClass` member's (C 163 assigns it, C 173 calls it), so it is `unsafe extern "C" fn`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn emacs_menu_bar_init (menu_bar: *mut EmacsMenuBar)         // C 151-155
{
  return;
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn emacs_menu_bar_class_init (klass: *mut EmacsMenuBarClass)   // C 157-164
{
  let mut widget_class: *mut GtkWidgetClass;

  widget_class = GTK_WIDGET_CLASS! (klass);
  (*widget_class).get_preferred_width = Some (emacs_menu_bar_get_preferred_width);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn emacs_menu_bar_get_preferred_width (widget: *mut GtkWidget,
							 minimum: *mut gint,
							 natural: *mut gint)   // C 166-177
{
  let mut widget_class: *mut GtkWidgetClass;

  widget_class = GTK_WIDGET_CLASS! (emacs_menu_bar_parent_class);
  ((*widget_class).get_preferred_width.unwrap_unchecked ()) (widget, minimum, natural);

  if ! minimum.is_null ()
    { *minimum = 0; }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn emacs_menu_bar_new () -> *mut GtkWidget       // C 179-183
{
  return GTK_WIDGET! (g_object_new (emacs_menu_bar_get_type (), std::ptr::null ()));
}

/* ----------------------------------------------------------------------------------------------
   The calls this slice makes whose declarations the mirrors do not carry yet (P-72 rule 4: the
   unit declares them at the use site and the ledger owes the move).  `min`/`max` follow the
   `src/w32term.rs` 1681-1687 treatment of the same pair; `INT_STRLEN_BOUND` follows
   `lib/strftime.rs` 542's (its mirror, `lib/include/intprops.rs`, keeps C's own text and is not
   compiled); `request_sigio`/`unrequest_sigio` are dispextern.h 3781-3782's; the rest are
   frame.h/lisp.h/xterm.h's, each with its line.  */
/// C: lisp.h 71-73: `#undef min` / `#define max(a, b) ((a) > (b) ? (a) : (b))` /
/// `#define min(a, b) ((a) < (b) ? (a) : (b))` - C 277's `min (scale, INT_MAX)` is the one use.
/// Each expands its argument twice, which the call site survives (a local and a literal).
#[cfg (feature = "use_gtk")]
macro_rules! max { ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } }; }
#[cfg (feature = "use_gtk")]
macro_rules! min { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }
/// C: intprops.h 88-90: `#define INT_STRLEN_BOUND(t)
/// (INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - _GL_SIGNED_TYPE_OR_EXPR (t)) + _GL_SIGNED_TYPE_OR_EXPR
/// (t))`, i.e. `((sizeof (t) * CHAR_BIT - 1) * 146 + 484) / 485 + 1` for a signed type - the body
/// `lib/strftime.rs` 542 carries for `time_t`, here for `int` (C 1113's array bound).  C's own
/// `INT_STRLEN_BOUND (int)` spelling stays where the array is declared.
#[cfg (feature = "use_gtk")]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((core::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 245/251: `unrequest_sigio ();` / `request_sigio ();` (twice in each arm of
  /// `xg_display_open`)
  /// C: dispextern.h 3781-3782: `void request_sigio (void);` / `void unrequest_sigio (void);`
  pub fn request_sigio ();
  pub fn unrequest_sigio ();
  /// C 1161: `change_frame_size (f, width, height, false, true, false);`
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
  pub fn change_frame_size (f: *mut frame, width: c_int, height: c_int, pretend: bool,
                            delay: bool, safe: bool);
  /// C 1160: `FRAME_RIF (f)->clear_under_internal_border (f);` - `FRAME_RIF` itself is
  /// `include/frame.rs`'s macro; the call site is C's member call.
  /// C 1253/1163: `cancel_mouse_face (f);`
  /// C: dispextern.h 3690: `extern void cancel_mouse_face (struct frame *);`
  pub fn cancel_mouse_face (f: *mut frame);
  /// C 521/529: `Lisp_Object specified_file = file_for_image (img->spec);` /
  /// `STRINGP (file = image_find_image_file (specified_file))`
  /// C: dispextern.h 3748: `extern Lisp_Object image_find_image_file (Lisp_Object);`
  pub fn image_find_image_file (file: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);`
  pub fn adjust_frame_size (f: *mut frame, width: c_int, height: c_int, inhibit: c_int,
                            pretend: bool, parameter: Lisp_Object);
  /// C: frame.h 1844: `extern void gui_set_fullscreen (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn gui_set_fullscreen (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  /// C 1280: `store_frame_param (f, Qfullscreen, fullscreen);`
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  /// C 1179: `Lisp_Object fullscreen = get_frame_param (f, Qfullscreen);`
  /// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  /// C 1899: `x_wait_for_event (f, ConfigureNotify);` (C 1274 in this slice)
  /// C: xterm.h 1899: `extern void x_wait_for_event (struct frame *, int);`
  pub fn x_wait_for_event (f: *mut frame, event_type: c_int);
  /// C 792: `success_p = x_parse_color (f, buf, color) != 0;`
  /// C: xterm.h 391: `extern Status x_parse_color (struct frame *, const char *, XColor *);`
  pub fn x_parse_color (f: *mut frame, color_name: *const c_char,
                        color: *mut Emacs_Color) -> c_int;
  /// C 1238: `x_cr_update_surface_desired_size (f, width, height);`
  /// C: xterm.h 1812: `extern void x_cr_update_surface_desired_size (struct frame *, int, int);`
  pub fn x_cr_update_surface_desired_size (f: *mut frame, width: c_int, height: c_int);
}
/* C 1088/1094 (the `HAVE_PGTK` arm of `xg_set_geometry`), C 795 (the pgtk arm of
   `xg_check_special_colors`) and C 1132's `pgtk_parse_color`: `pgtkterm.h`'s two pixel
   measurements and its colour parser, reached in C through `"gtkutil.h"` 29's
   `#include "pgtkterm.h"`.  They now live in that header's mirror, `include/pgtkterm.rs` (P-90
   ③'s move-back; the edge above reaches it in this arm), so the unit's copies are gone.  */
/// C: frame.h 1515-1516: `extern void frame_size_history_extra (struct frame *, Lisp_Object, int,
/// int, int, int, int, int);` - C 1154/1166/1265/1287's four calls.
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn frame_size_history_extra (f: *mut frame, name: Lisp_Object,
                                   old_w: c_int, old_h: c_int, new_w: c_int, new_h: c_int,
                                   f_new_w: c_int, f_new_h: c_int);
}

/* C 187-190: the "Display handling functions" banner C 188-190 writes.  */

/* C 192-195: "Keep track of the default display, or NULL if there is none.  Emacs may close all
   its displays." and the object itself.  */
#[cfg (feature = "use_gtk")]
static mut gdpy_def: *mut GdkDisplay = std::ptr::null_mut ();   // C 195

/* C 197-199's comment, C 201-226's `xg_set_screen`.  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_set_screen (w: *mut GtkWidget, f: *mut frame)   // C 201-226
{
  #[cfg (not (feature = "have_pgtk"))]
  {
    if FRAME_X_DISPLAY! (f) != DEFAULT_GDK_DISPLAY! ()
      {
        let gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay (FRAME_X_DISPLAY! (f));
        let gscreen: *mut GdkScreen = gdk_display_get_default_screen (gdpy);

        if GTK_IS_MENU! (w)
          { gtk_menu_set_screen (GTK_MENU! (w), gscreen); }
        else
          { gtk_window_set_screen (GTK_WINDOW! (w), gscreen); }
      }
  }
  #[cfg (feature = "have_pgtk")]
  {
    if FRAME_X_DISPLAY! (f) != DEFAULT_GDK_DISPLAY! ()
      {
        let gscreen: *mut GdkScreen = gdk_display_get_default_screen (FRAME_X_DISPLAY! (f));

        if GTK_IS_MENU! (w)
          { gtk_menu_set_screen (GTK_MENU! (w), gscreen); }
        else
          { gtk_window_set_screen (GTK_WINDOW! (w), gscreen); }
      }
  }
}

/* C 229-234's comment, C 236-264.  The `#ifndef HAVE_PGTK` of C 237-241 splits the *declarator*,
   and the two bodies below C 246-250 and C 259-263 differ too, so each arm is its own `fn` - the
   `src/sysdep.rs` 4680/4959/5183 `system_process_attributes` shape for a C function whose whole
   signature and body are version-specific.  C's parameter names are kept in both arms (`dpy` in
   the X arm, `gdpy` in the pgtk one, which is what C's own two texts write).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_display_open (display_name: *mut c_char, dpy: *mut *mut Display)
{                                                        // C 236-264, the `!HAVE_PGTK` arm
  let mut gdpy: *mut GdkDisplay;

  unrequest_sigio ();  /* See comment in x_display_ok, xterm.c.  */
  gdpy = gdk_display_open (display_name);
  request_sigio ();
  if gdpy_def.is_null () && ! gdpy.is_null ()
    {
      gdpy_def = gdpy;
      gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy);
    }

  *dpy = if ! gdpy.is_null () { GDK_DISPLAY_XDISPLAY! (gdpy) } else { std::ptr::null_mut () };
}
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_display_open (display_name: *mut c_char, dpy: *mut *mut GdkDisplay)
{                                                        // C 236-264, the `HAVE_PGTK` arm
  let mut gdpy: *mut GdkDisplay;

  unrequest_sigio ();  /* See comment in x_display_ok, xterm.c.  */
  gdpy = gdk_display_open (if strlen (display_name) == 0 { std::ptr::null () }
                           else { display_name });
  request_sigio ();
  if gdpy_def.is_null () && ! gdpy.is_null ()
    {
      gdpy_def = gdpy;
      gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy);
    }

  *dpy = gdpy;
}

/* C 266: "Scaling/HiDPI functions.", C 267-282's `xg_get_gdk_scale`.  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_get_gdk_scale () -> c_int   // C 267-282
{
  #[cfg (feature = "have_gtk3")]
  {
    let sscale: *const c_char = getenv (c"GDK_SCALE".as_ptr ());

    if ! sscale.is_null ()
      {
        let scale: c_long = atol (sscale);
        if 0 < scale
          { return min! (scale, INT_MAX as c_long) as c_int; }
      }
  }

  return 1;
}

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_get_scale (f: *mut frame) -> c_int   // C 284-295
{
  #[cfg (feature = "have_pgtk")]
  { return 1; }
  #[cfg (feature = "have_gtk3")]
  {
    if ! FRAME_GTK_WIDGET! (f).is_null ()
      { return gtk_widget_get_scale_factor (FRAME_GTK_WIDGET! (f)); }
  }
  return xg_get_gdk_scale ();
}

/* C 297's comment, C 299-355's `xg_display_close` - the other function whose whole body C writes
   twice; C 306-330 is the X arm (`dpy`, the local `gdpy` and `dpyinfo->display`) and C 331-354 the
   pgtk one (the parameter *is* `gdpy`, and the loop tests `dpyinfo->gdpy`).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_display_close (dpy: *mut Display)   // C 299-330
{
  let mut gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay (dpy);

  /* If this is the default display, try to change it before closing.
     If there is no other display to use, gdpy_def is set to NULL, and
     the next call to xg_display_open resets the default display.  */
  if gdk_display_get_default () == gdpy
    {
      let mut dpyinfo: *mut x_display_info;
      let mut gdpy_new: *mut GdkDisplay = std::ptr::null_mut ();

      /* Find another display.  */
      dpyinfo = x_display_list;
      while ! dpyinfo.is_null ()
        {
          if (*dpyinfo).display != dpy
            {
              gdpy_new = gdk_x11_lookup_xdisplay ((*dpyinfo).display);
              gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy_new);
              break;
            }
          dpyinfo = (*dpyinfo).next;
        }
      gdpy_def = gdpy_new;
    }

  gdk_display_close (gdpy);
}
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_display_close (gdpy: *mut GdkDisplay)   // C 331-354
{
  /* If this is the default display, try to change it before closing.
     If there is no other display to use, gdpy_def is set to NULL, and
     the next call to xg_display_open resets the default display.  */
  if gdk_display_get_default () == gdpy
    {
      let mut dpyinfo: *mut pgtk_display_info;
      let mut gdpy_new: *mut GdkDisplay = std::ptr::null_mut ();

      /* Find another display.  */
      dpyinfo = x_display_list as *mut pgtk_display_info;
      while ! dpyinfo.is_null ()
        {
          if (*dpyinfo).gdpy != gdpy
            {
              gdpy_new = (*dpyinfo).gdpy;
              gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy_new);
              break;
            }
          dpyinfo = (*dpyinfo).next;
        }
      gdpy_def = gdpy_new;
    }

  gdk_display_close (gdpy);
}

/* C 357-360's banner and C 362-363's comment, then C 365-376's `xg_create_default_cursor` - the
   third arm-specific signature (C's pgtk arm names the parameter `gdpy` and uses it directly).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_create_default_cursor (dpy: *mut Display) -> *mut GdkCursor
{                                                        // C 365-376, the `!HAVE_PGTK` arm
  let gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay (dpy);
  return gdk_cursor_new_for_display (gdpy, GDK_LEFT_PTR);
}
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_create_default_cursor (gdpy: *mut GdkDisplay) -> *mut GdkCursor
{                                                        // C 365-376, the `HAVE_PGTK` arm
  return gdk_cursor_new_for_display (gdpy, GDK_LEFT_PTR);
}

/* C 378: `#ifndef HAVE_PGTK` - the arm that runs to C 478's `#endif /* !HAVE_PGTK */`.  Both of
   its items are below, C 379-380's comment and banner with the first.  */

/* Apply GMASK to GPIX and return a GdkPixbuf with an alpha channel.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe fn xg_get_pixbuf_from_pix_and_mask (f: *mut frame, pix: Pixmap, mask: Pixmap)
  -> *mut GdkPixbuf                                        // C 381-426
{
  let mut icon_buf: *mut GdkPixbuf = std::ptr::null_mut ();
  let mut iunused: c_int = 0;
  let mut wunused: Window = 0;
  let mut width: c_uint = 0;
  let mut height: c_uint = 0;
  let mut depth: c_uint = 0;
  let mut uunused: c_uint = 0;

  if (*FRAME_DISPLAY_INFO! (f)).red_bits != 8
    { return std::ptr::null_mut (); }
  XGetGeometry (FRAME_X_DISPLAY! (f), pix, &raw mut wunused, &raw mut iunused, &raw mut iunused,
                &raw mut width, &raw mut height, &raw mut uunused, &raw mut depth);
  if depth != 24
    { return std::ptr::null_mut (); }
  let xim: *mut XImage = XGetImage (FRAME_X_DISPLAY! (f), pix, 0, 0, width, height,
                                    (! 0) as c_ulong, XYPixmap);
  if ! xim.is_null ()
    {
      let xmm: *mut XImage = if mask == 0
        { std::ptr::null_mut () }
        else
        { XGetImage (FRAME_X_DISPLAY! (f), mask, 0, 0, width, height, (! 0) as c_ulong, XYPixmap) };
      icon_buf = gdk_pixbuf_new (GDK_COLORSPACE_RGB, TRUE, 8, width as c_int, height as c_int);
      if ! icon_buf.is_null ()
	{
	  let mut pixels: *mut guchar = gdk_pixbuf_get_pixels (icon_buf);
	  let rowjunkwidth: c_int = gdk_pixbuf_get_rowstride (icon_buf) - (width as c_int) * 4;
	  let mut y: c_int = 0;
	  while y < height as c_int
	    {
	      let mut x: c_int = 0;
	      while x < width as c_int
		{
		  let rgb: c_ulong = XGetPixel (xim, x, y);
		  *pixels = ((rgb >> 16) & 255) as guchar;
		  pixels = pixels.add (1);
		  *pixels = ((rgb >> 8) & 255) as guchar;
		  pixels = pixels.add (1);
		  *pixels = (rgb & 255) as guchar;
		  pixels = pixels.add (1);
		  *pixels = (if ! xmm.is_null () && XGetPixel (xmm, x, y) == 0
			     { 0 } else { 255 }) as guchar;
		  pixels = pixels.add (1);
		  x += 1;
		}
	      y += 1;
	      pixels = pixels.offset (rowjunkwidth as isize);
	    }
	}

      if ! xmm.is_null ()
	{ XDestroyImage (xmm); }
      XDestroyImage (xim);
    }

  return icon_buf;
}

/* C 428-476: `#if defined USE_CAIRO && !defined HAVE_GTK3` - inside C 378's arm, so the gate is
   the pair's.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk"),
            feature = "use_cairo", not (feature = "have_gtk3")))]
unsafe fn xg_get_pixbuf_from_surface (surface: *mut cairo_surface_t)
  -> *mut GdkPixbuf                                        // C 429-475
{
  let width: c_int = cairo_image_surface_get_width (surface);
  let height: c_int = cairo_image_surface_get_height (surface);
  let icon_buf: *mut GdkPixbuf = gdk_pixbuf_new (GDK_COLORSPACE_RGB, TRUE, 8, width, height);
  if ! icon_buf.is_null ()
    {
      let mut pixels: *mut guchar = gdk_pixbuf_get_pixels (icon_buf);
      let rowstride: c_int = gdk_pixbuf_get_rowstride (icon_buf);
      let icon_surface: *mut cairo_surface_t
	= cairo_image_surface_create_for_data (pixels, CAIRO_FORMAT_ARGB32, width, height,
					       rowstride);
      let cr: *mut cairo_t = cairo_create (icon_surface);
      cairo_surface_destroy (icon_surface);
      cairo_set_source_surface (cr, surface, 0 as c_double, 0 as c_double);
      cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);
      cairo_paint (cr);
      cairo_destroy (cr);

      let mut y: c_int = 0;
      while y < height
	{
	  let mut x: c_int = 0;
	  while x < width
	    {
	      let argb: guint32 = *((pixels as *mut guint32).offset (x as isize));
	      let alpha: c_int = (argb >> 24) as c_int;

	      if alpha == 0
		{ *((pixels as *mut guint32).offset (x as isize)) = 0; }
	      else
		{
		  let red: c_int = ((argb >> 16) & 0xff) as c_int;
		  let green: c_int = ((argb >> 8) & 0xff) as c_int;
		  let blue: c_int = (argb & 0xff) as c_int;

		  *pixels.offset ((x * 4) as isize) = (red * 0xff / alpha) as guchar;
		  *pixels.offset ((x * 4 + 1) as isize) = (green * 0xff / alpha) as guchar;
		  *pixels.offset ((x * 4 + 2) as isize) = (blue * 0xff / alpha) as guchar;
		  *pixels.offset ((x * 4 + 3) as isize) = alpha as guchar;
		}
	      x += 1;
	    }
	  pixels = pixels.offset (rowstride as isize);
	  y += 1;
	}
    }

  return icon_buf;
}

/* C 480-493.  */
#[cfg (feature = "use_gtk")]
unsafe fn file_for_image (image: Lisp_Object) -> Lisp_Object   // C 480-493
{
  let mut specified_file: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object;

  tail = XCDR (image);
  while NILP (specified_file) && CONSP (tail) && CONSP (XCDR (tail))
    {
      if EQ (XCAR (tail), QCfile ())
	{ specified_file = XCAR (XCDR (tail)); }
      tail = XCDR (XCDR (tail));
    }

  return specified_file;
}

/* C 495-504's comment, C 506-595's `xg_get_image_for_pixmap`.  C 512-516's `#ifdef USE_CAIRO`
   declaration pair is carried as two pre-initialized bindings (PORTING.md 0.1-13).  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_get_image_for_pixmap (f: *mut frame, img: *mut image, widget: *mut GtkWidget,
                                   mut old_widget: *mut GtkImage) -> *mut GtkWidget
                                                                   // C 506-595
{
  #[cfg (feature = "use_cairo")]
  let mut surface: *mut cairo_surface_t = std::ptr::null_mut ();
  #[cfg (not (feature = "use_cairo"))]
  let mut icon_buf: *mut GdkPixbuf = std::ptr::null_mut ();

  /* If we have a file, let GTK do all the image handling.
     This seems to be the only way to make insensitive and activated icons
     look good in all cases.  */
  let specified_file: Lisp_Object = file_for_image ((*img).spec);
  /* C's `Lisp_Object file;` is uninitialized and assigned inside C 528-529's condition; the port
     pre-initializes it (PORTING.md 0.1-13) and keeps C's assignment where C writes it.  */
  let mut file: Lisp_Object = std::ptr::null_mut ();

  /* We already loaded the image once before calling this
     function, so this only fails if the image file has been removed.
     In that case, use the pixmap already loaded.  */

  if STRINGP (specified_file)
      && { file = image_find_image_file (specified_file); STRINGP (file) }
    {
      let encoded_file: *mut c_char = SSDATA (ENCODE_FILE (file));
      if old_widget.is_null ()
        { old_widget = GTK_IMAGE! (gtk_image_new_from_file (encoded_file)); }
      else
        { gtk_image_set_from_file (old_widget, encoded_file); }

      return GTK_WIDGET! (old_widget);
    }

  /* No file, do the image handling ourselves.  This will look very bad
     on a monochrome display, and sometimes bad on all displays with
     certain themes.  */

  #[cfg (feature = "use_cairo")]
  {
    if cairo_pattern_get_type ((*img).cr_data as *mut cairo_pattern_t) == CAIRO_PATTERN_TYPE_SURFACE
      { cairo_pattern_get_surface ((*img).cr_data as *mut cairo_pattern_t, &raw mut surface); }
    else
      { surface = std::ptr::null_mut (); }

    if ! surface.is_null ()
      {
        #[cfg (feature = "have_gtk3")]
        {
          if old_widget.is_null ()
            { old_widget = GTK_IMAGE! (gtk_image_new_from_surface (surface)); }
          else
            { gtk_image_set_from_surface (old_widget, surface); }
        }
        #[cfg (not (feature = "have_gtk3"))]
        {
          let icon_buf: *mut GdkPixbuf = xg_get_pixbuf_from_surface (surface);

          if ! icon_buf.is_null ()
            {
              if old_widget.is_null ()
                { old_widget = GTK_IMAGE! (gtk_image_new_from_pixbuf (icon_buf)); }
              else
                { gtk_image_set_from_pixbuf (old_widget, icon_buf); }

              g_object_unref (G_OBJECT! (icon_buf) as gpointer);
            }
        }
      }
  }
  #[cfg (not (feature = "use_cairo"))]
  {
    /* This is a workaround to make icons look good on pseudo color
       displays.  Apparently GTK expects the images to have an alpha
       channel.  If they don't, insensitive and activated icons will
       look bad.  This workaround does not work on monochrome displays,
       and is strictly not needed on true color/static color displays (i.e.
       16 bits and higher).  But we do it anyway so we get a pixbuf that is
       not associated with the img->pixmap.  The img->pixmap may be removed
       by clearing the image cache and then the tool bar redraw fails, since
       Gtk+ assumes the pixmap is always there.  */
    icon_buf = xg_get_pixbuf_from_pix_and_mask (f, (*img).pixmap as Pixmap, (*img).mask as Pixmap);

    if ! icon_buf.is_null ()
      {
        if old_widget.is_null ()
          { old_widget = GTK_IMAGE! (gtk_image_new_from_pixbuf (icon_buf)); }
        else
          { gtk_image_set_from_pixbuf (old_widget, icon_buf); }

        g_object_unref (G_OBJECT! (icon_buf) as gpointer);
      }
  }

  return GTK_WIDGET! (old_widget);
}

/* C 597-600's comment, C 602-617's `xg_set_cursor`.  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_set_cursor (w: *mut GtkWidget, cursor: *mut GdkCursor)   // C 602-617
{
  let window: *mut GdkWindow = gtk_widget_get_window (w);
  let mut children: *mut GList = gdk_window_peek_children (window);

  gdk_window_set_cursor (window, cursor);

  /* The scroll bar widget has more than one GDK window (had to look at
     the source to figure this out), and there is no way to set cursor
     on widgets in GTK.  So we must set the cursor for all GDK windows.
     Ditto for menus.  */

  while ! children.is_null ()
    {
      gdk_window_set_cursor (GDK_WINDOW! ((*children).data), cursor);
      children = g_list_next! (children);
    }
}

/* C 619's comment.  `xg_list_node` is `gtkutil.h` 52-56's record:

     typedef struct xg_list_node_
     {
       struct xg_list_node_ *prev;
       struct xg_list_node_ *next;
     } xg_list_node;

   whose mirror (`include/gtkutil.rs`) is a placeholder, so it stands here (P-72 rule 4; the move
   back is owed).  C 624/626/637/641/645-646 read both members.  */
/// C: gtkutil.h 52-56: `typedef struct xg_list_node_ { struct xg_list_node_ *prev; struct
/// xg_list_node_ *next; } xg_list_node;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct xg_list_node
{
  pub prev: *mut xg_list_node,   // C: `struct xg_list_node_ *prev;`
  pub next: *mut xg_list_node,   // C: `struct xg_list_node_ *next;`
}

/* C 619's other comment, C 621-630's `xg_list_insert`.  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_list_insert (list: *mut xg_list_node, node: *mut xg_list_node)   // C 621-630
{
  let list_start: *mut xg_list_node = (*list).next;

  if ! list_start.is_null ()
    { (*list_start).prev = node; }
  (*node).next = list_start;
  (*node).prev = std::ptr::null_mut ();
  (*list).next = node;
}

/* C 632's comment, C 634-648's `xg_list_remove`.  */
#[cfg (feature = "use_gtk")]
unsafe fn xg_list_remove (list: *mut xg_list_node, node: *mut xg_list_node)   // C 634-648
{
  let list_start: *mut xg_list_node = (*list).next;
  if node == list_start
    {
      (*list).next = (*node).next;
      if ! (*list).next.is_null ()
        { (*(*list).next).prev = std::ptr::null_mut (); }
    }
  else
    {
      (*(*node).prev).next = (*node).next;
      if ! (*node).next.is_null ()
        { (*(*node).next).prev = (*node).prev; }
    }
}

/* `atol` (C 275) and `strstr` (C 1028) are the two libc functions this slice names that no mirror
   carries: `lib/include/stdlib.rs` has `strtol` but not `atol`, and `lib/include/string.rs`
   (whose head says it keeps `strlen`/`strcmp`/`strchr`) has neither `strstr` nor the
   `lib/include/string.in.rs` 252 rename that is dead on the frozen target.  Both are declared here
   at the use site - the `src/xfns.rs` 210-218 shape - and `## Owed` records them.  */
/// C: stdlib.h: `long int atol (const char *nptr);` - C 275's `long scale = atol (sscale);`
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn atol (nptr: *const c_char) -> c_long;
}
/// C: string.h: `char *strstr (const char *haystack, const char *needle);` - C 1028's
/// `if (!strstr (msg, "visible children"))`
#[cfg (all (feature = "use_gtk", not (feature = "gtk_check_version_ge_3_22")))]
unsafe extern "C" {
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/* gnulib's `<stdckdint.h>` `ckd_mul`/`ckd_add` as C 698-699 uses them: on the frozen target the
   header defines the names as clang's `__builtin_*_overflow` (P-15), whose port counterpart is the
   `overflowing_*` pair with a write-back.  The bodies are `src/coding.rs` 165-172's pair, which is
   the sibling copy of the same two gnulib names.  */
macro_rules! ckd_mul {
  ($dst:ident, $a:expr, $b:expr) => {{
    let (ckd_mul_value, ckd_mul_overflow) = ($a).overflowing_mul ($b);
    $dst = ckd_mul_value;
    ckd_mul_overflow
  }};
}
macro_rules! ckd_add {
  ($dst:ident, $a:expr, $b:expr) => {{
    let (ckd_add_value, ckd_add_overflow) = ($a).overflowing_add ($b);
    $dst = ckd_add_value;
    ckd_add_overflow
  }};
}

/* C 650-653's comment, C 655-729's `get_utf8_string`.  */
#[cfg (feature = "use_gtk")]
unsafe fn get_utf8_string (str: *const c_char) -> *mut c_char   // C 655-729
{
  let mut utf8_str: *mut c_char;

  if str.is_null ()
    { return std::ptr::null_mut (); }

  /* If not UTF-8, try current locale.  */
  if g_utf8_validate (str, -1, std::ptr::null_mut ()) == 0
    { utf8_str = g_locale_to_utf8 (str, -1, std::ptr::null_mut (), std::ptr::null_mut (),
				   std::ptr::null_mut ()); }
  else
    { return g_strdup (str); }

  if utf8_str.is_null ()
    {
      /* Probably some control characters in str.  Escape them. */
      let mut len: ptrdiff_t;
      let mut nr_bad: ptrdiff_t = 0;
      let mut bytes_read: gsize = 0;
      let mut bytes_written: gsize = 0;
      let mut p: *mut c_uchar = str as *mut c_uchar;
      let mut cp: *mut c_char;
      let mut up: *mut c_char;
      let mut err: *mut GError = std::ptr::null_mut ();

      loop
	{
	  cp = g_locale_to_utf8 (p as *mut c_char, -1, &raw mut bytes_read,
				 &raw mut bytes_written, &raw mut err);
	  if ! (cp.is_null () && (*err).code == G_CONVERT_ERROR_ILLEGAL_SEQUENCE)
	    { break; }
	  nr_bad += 1;
	  p = p.offset ((bytes_written + 1) as isize);
	  g_error_free (err);
	  err = std::ptr::null_mut ();
	}

      if ! err.is_null ()
        { g_error_free (err); err = std::ptr::null_mut (); }
      if ! cp.is_null ()
        { g_free (cp as gpointer); }

      len = strlen (str) as ptrdiff_t;
      let mut alloc: ptrdiff_t = 0;
      if ckd_mul! (alloc, nr_bad, 4)
	  || ckd_add! (alloc, alloc, len + 1)
	  || SIZE_MAX < alloc as usize
	{ memory_full_up (); }
      utf8_str = xmalloc (alloc as usize) as *mut c_char;
      up = utf8_str;
      p = str as *mut c_uchar;

      loop
	{
	  cp = g_locale_to_utf8 (p as *mut c_char, -1, &raw mut bytes_read,
				 &raw mut bytes_written, &raw mut err);
	  if ! (cp.is_null () && (*err).code == G_CONVERT_ERROR_ILLEGAL_SEQUENCE)
	    { break; }
	  memcpy (up as *mut c_void, p as *const c_void, bytes_written as usize);
	  up = up.offset (bytes_written as isize);
	  up = up.offset (sprintf (up, c"\\%03o".as_ptr (),
				   *p.offset (bytes_written as isize) as c_int) as isize);
	  p = p.offset ((bytes_written + 1) as isize);
	  g_error_free (err);
	  err = std::ptr::null_mut ();
	}

      if ! cp.is_null ()
        { strcpy (up, cp); g_free (cp as gpointer); }
      if ! err.is_null ()
        { g_error_free (err); err = std::ptr::null_mut (); }
    }
  return utf8_str;
}

/* C 731-733's comment, C 735-810's `xg_check_special_colors`.  C 743-753's `#ifdef HAVE_GTK3`
   declaration pairs are below inside each arm (the GTK2 arm's two names are the ones GTK2's header
   owns; see the GTK section).  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_check_special_colors (mut f: *mut frame,
						  color_name: *const c_char,
						  color: *mut Emacs_Color) -> bool
{                                                        // C 735-810
  let mut success_p: bool;
  let mut get_bg: bool;
  let mut get_fg: bool;
  #[cfg (feature = "have_gtk3")]
  let mut gsty: *mut GtkStyleContext;
  #[cfg (not (feature = "have_gtk3"))]
  let mut gsty: *mut GtkStyle;
  #[cfg (feature = "have_gtk3")]
  let mut col: GdkRGBA = GdkRGBA { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 };
  #[cfg (feature = "have_gtk3")]
  let mut buf: [c_char; b"rgb://rrrr/gggg/bbbb".len () + 1]
    = [0; b"rgb://rrrr/gggg/bbbb".len () + 1];
  #[cfg (feature = "have_gtk3")]
  let mut state: c_int;
  #[cfg (feature = "have_gtk3")]
  let mut c: *mut GdkRGBA = std::ptr::null_mut ();
  #[cfg (feature = "have_gtk3")]
  let mut r: c_ushort;
  #[cfg (feature = "have_gtk3")]
  let mut g: c_ushort;
  #[cfg (feature = "have_gtk3")]
  let mut b: c_ushort;
  #[cfg (not (feature = "have_gtk3"))]
  let mut grgb: *mut GdkColor;

  get_bg = strcmp (c"gtk_selection_bg_color".as_ptr (), color_name) == 0;
  get_fg = ! get_bg && strcmp (c"gtk_selection_fg_color".as_ptr (), color_name) == 0;
  success_p = false;

  #[cfg (feature = "have_pgtk")]
  {
    while ! FRAME_PARENT_FRAME (f).is_null ()
      { f = FRAME_PARENT_FRAME (f); }
  }

  if FRAME_GTK_WIDGET! (f).is_null () || ! (get_bg || get_fg)
    { return success_p; }

  block_input ();
  #[cfg (feature = "have_gtk3")]
  {
    gsty = gtk_widget_get_style_context (FRAME_GTK_OUTER_WIDGET! (f));
    state = GTK_STATE_FLAG_SELECTED | GTK_STATE_FLAG_FOCUSED;

    if get_fg
      { gtk_style_context_get_color (gsty, state, &raw mut col); }
    else
      {
	/* FIXME: Retrieving the background color is deprecated in
	   GTK+ 3.16.  New versions of GTK+ don't use the concept of a
	   single background color any more, so we shouldn't query for
	   it.  */
	gtk_style_context_get (gsty, state, GTK_STYLE_PROPERTY_BACKGROUND_COLOR, &raw mut c,
			       std::ptr::null_mut () as *mut c_void);
	col = *c;
	gdk_rgba_free (c);
      }

    r = (col.red * 65535.0) as c_ushort;
    g = (col.green * 65535.0) as c_ushort;
    b = (col.blue * 65535.0) as c_ushort;
    #[cfg (not (feature = "have_pgtk"))]
    {
      sprintf (buf.as_mut_ptr (), c"rgb:%04x/%04x/%04x".as_ptr (), r as c_int, g as c_int,
	       b as c_int);
      success_p = x_parse_color (f, buf.as_ptr (), color) != 0;
    }
    #[cfg (feature = "have_pgtk")]
    {
      sprintf (buf.as_mut_ptr (), c"#%04x%04x%04x".as_ptr (), r as c_int, g as c_int,
	       b as c_int);
      success_p = pgtk_parse_color (f, buf.as_ptr (), color) != 0;
    }
  }
  #[cfg (not (feature = "have_gtk3"))]
  {
    gsty = gtk_widget_get_style (FRAME_GTK_WIDGET! (f));
    grgb = if get_bg
      { std::ptr::addr_of_mut! ((*gsty).bg[GTK_STATE_SELECTED as usize]) }
      else
      { std::ptr::addr_of_mut! ((*gsty).fg[GTK_STATE_SELECTED as usize]) };

    (*color).red = (*grgb).red;
    (*color).green = (*grgb).green;
    (*color).blue = (*grgb).blue;
    (*color).pixel = (*grgb).pixel as c_ulong;
    success_p = true;
  }
  unblock_input ();
  return success_p;
}

/* C 812-816: the "Tooltips" banner.  */
/* C 818: `#ifndef HAVE_PGTK` - the arm that runs to C 994's `#else  /* HAVE_PGTK */`; C 820-822's
   comment comes with the first item below.  */

/* Gtk+ calls this callback when the parent of our tooltip dummy changes.
   We use that to pop down the tooltip.  This happens if Gtk+ for some
   reason wants to change or hide the tooltip.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" fn hierarchy_ch_cb (widget: *mut GtkWidget, previous_toplevel: *mut GtkWidget,
				      user_data: gpointer)             // C 824-835
{
  let f: *mut frame = user_data as *mut frame;
  let x: *mut xp_output = (*f).output_data.x as *mut xp_output;
  let top: *mut GtkWidget = gtk_widget_get_toplevel ((*x).ttip_lbl);

  if top.is_null () || ! GTK_IS_WINDOW! (top)
    { gtk_widget_hide (previous_toplevel); }
}

/* Callback called when Gtk+ thinks a tooltip should be displayed.
   We use it to get the tooltip window and the tooltip widget so
   we can manipulate the ourselves.

   Return FALSE ensures that the tooltip is not shown.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" fn qttip_cb (widget: *mut GtkWidget, xpos: gint, ypos: gint,
			       keyboard_mode: gboolean, tooltip: *mut GtkTooltip,
			       user_data: gpointer) -> gboolean          // C 843-888
{
  let f: *mut frame = user_data as *mut frame;
  let x: *mut xp_output = (*f).output_data.x as *mut xp_output;
  if (*x).ttip_widget.is_null ()
    {
      let mut p: *mut GtkWidget;
      let mut list: *mut GList;
      let mut iter: *mut GList;

      g_object_set (G_OBJECT! (widget) as gpointer, c"has-tooltip".as_ptr (), FALSE,
		    std::ptr::null_mut () as *mut c_void);
      (*x).ttip_widget = tooltip;
      g_object_ref (G_OBJECT! (tooltip) as gpointer);
      (*x).ttip_lbl = gtk_label_new (c"".as_ptr ());
      g_object_ref (G_OBJECT! ((*x).ttip_lbl) as gpointer);
      gtk_tooltip_set_custom (tooltip, (*x).ttip_lbl);
      (*x).ttip_window = GTK_WINDOW! (gtk_widget_get_toplevel ((*x).ttip_lbl));

      /* Change stupid Gtk+ default line wrapping.  */
      p = gtk_widget_get_parent ((*x).ttip_lbl);
      list = gtk_container_get_children (GTK_CONTAINER! (p));
      iter = list;
      while ! iter.is_null ()
	{
	  let w: *mut GtkWidget = GTK_WIDGET! ((*iter).data);
	  if GTK_IS_LABEL! (w)
	    { gtk_label_set_line_wrap (GTK_LABEL! (w), FALSE); }
	  iter = g_list_next! (iter);
	}
      g_list_free (list);

      /* ATK needs an empty title for some reason.  */
      gtk_window_set_title ((*x).ttip_window, c"".as_ptr ());
      /* Realize so we can safely get screen later on.  */
      gtk_widget_realize (GTK_WIDGET! ((*x).ttip_window));
      gtk_widget_realize ((*x).ttip_lbl);

      g_signal_connect! ((*x).ttip_lbl, c"hierarchy-changed".as_ptr (), hierarchy_ch_cb,
			 f as gpointer);
    }

  return FALSE;
}

/* Prepare a tooltip to be shown, i.e. calculate WIDTH and HEIGHT.
   Return true if a system tooltip is available.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_prepare_tooltip (f: *mut frame, string: Lisp_Object,
					     width: *mut c_int, height: *mut c_int) -> bool
{                                                        // C 893-942
  let x: *mut xp_output = (*f).output_data.x as *mut xp_output;
  let mut widget: *mut GtkWidget;
  let mut gwin: *mut GdkWindow;
  let mut screen: *mut GdkScreen;
  let mut settings: *mut GtkSettings;
  let mut tt_enabled: gboolean = TRUE;
  let mut req: GtkRequisition = GtkRequisition { width: 0, height: 0 };
  let mut encoded_string: Lisp_Object;

  if (*x).ttip_lbl.is_null ()
    { return false; }

  block_input ();
  encoded_string = ENCODE_UTF_8 (string);
  widget = GTK_WIDGET! ((*x).ttip_lbl);
  gwin = gtk_widget_get_window (GTK_WIDGET! ((*x).ttip_window));
  screen = gdk_window_get_screen (gwin);
  settings = gtk_settings_get_for_screen (screen);
  g_object_get (settings as gpointer, c"gtk-enable-tooltips".as_ptr (),
		&raw mut tt_enabled, std::ptr::null_mut () as *mut c_void);
  if tt_enabled != 0
    {
      g_object_set (settings as gpointer, c"gtk-enable-tooltips".as_ptr (), FALSE,
		    std::ptr::null_mut () as *mut c_void);
      /* Record that we disabled it so it can be enabled again.  */
      g_object_set_data (G_OBJECT! ((*x).ttip_window), c"restore-tt".as_ptr (),
			 f as gpointer);
    }

  /* Prevent Gtk+ from hiding tooltip on mouse move and such.  */
  g_object_set_data (G_OBJECT! (gtk_widget_get_display (GTK_WIDGET! ((*x).ttip_window))),
		     c"gdk-display-current-tooltip".as_ptr (), std::ptr::null_mut ());

  /* Put our dummy widget in so we can get callbacks for unrealize and
     hierarchy-changed.  */
  gtk_tooltip_set_custom ((*x).ttip_widget, widget);
  gtk_tooltip_set_text ((*x).ttip_widget, SSDATA (encoded_string));
  gtk_widget_get_preferred_size (GTK_WIDGET! ((*x).ttip_window), std::ptr::null_mut (),
				 &raw mut req);
  if ! width.is_null ()
    { *width = req.width; }
  if ! height.is_null ()
    { *height = req.height; }

  unblock_input ();

  return true;
}

/* Show the tooltip at ROOT_X and ROOT_Y.
   xg_prepare_tooltip must have been called before this function.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_show_tooltip (f: *mut frame, root_x: c_int, root_y: c_int)
{                                                        // C 947-965
  let x: *mut xp_output = (*f).output_data.x as *mut xp_output;
  if ! (*x).ttip_window.is_null ()
    {
      block_input ();
      gtk_window_move ((*x).ttip_window, root_x / xg_get_scale (f),
		       root_y / xg_get_scale (f));
      gtk_widget_show (GTK_WIDGET! ((*x).ttip_window));
      unblock_input ();
    }
}

/* Hide tooltip if shown.  Do nothing if not shown.
   Return true if tip was hidden, false if not (i.e. not using
   system tooltips).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_hide_tooltip (f: *mut frame) -> bool   // C 971-992
{
  if ! (*((*f).output_data.x as *mut xp_output)).ttip_window.is_null ()
    {
      let win: *mut GtkWindow = (*((*f).output_data.x as *mut xp_output)).ttip_window;
      block_input ();
      gtk_widget_hide (GTK_WIDGET! (win));

      if ! g_object_get_data (G_OBJECT! (win), c"restore-tt".as_ptr ()).is_null ()
	{
	  let gwin: *mut GdkWindow = gtk_widget_get_window (GTK_WIDGET! (win));
	  let screen: *mut GdkScreen = gdk_window_get_screen (gwin);
	  let settings: *mut GtkSettings = gtk_settings_get_for_screen (screen);
	  g_object_set (settings as gpointer, c"gtk-enable-tooltips".as_ptr (), TRUE,
			std::ptr::null_mut () as *mut c_void);
	}
      unblock_input ();

      return true;
    }
  return false;
}

/* C 994's `#else  /* HAVE_PGTK */` arm.  Its two entry points have *different signatures* from
   the arm above (C 996-1006's `xg_show_tooltip` takes the string and no coordinates), so each is
   its own `fn` - the `src/sysdep.rs` shape again.  */
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_show_tooltip (f: *mut frame, string: Lisp_Object)   // C 996-1006
{
  let encoded_string: Lisp_Object = ENCODE_UTF_8 (string);
  gtk_widget_set_tooltip_text (if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
			       { FRAME_GTK_OUTER_WIDGET! (f) }
			       else { FRAME_GTK_WIDGET! (f) },
			       SSDATA (encoded_string));
}

#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_hide_tooltip (f: *mut frame) -> bool   // C 1008-1016
{
  if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { gtk_widget_set_tooltip_text (FRAME_GTK_OUTER_WIDGET! (f), std::ptr::null ()); }
  gtk_widget_set_tooltip_text (FRAME_GTK_WIDGET! (f), std::ptr::null ());
  return true;
}

/* C 1017-1021's banner and C 1023's `#if ! GTK_CHECK_VERSION (3, 22, 0)`.  */
#[cfg (all (feature = "use_gtk", not (feature = "gtk_check_version_ge_3_22")))]
unsafe extern "C" fn my_log_handler (log_domain: *const gchar, log_level: GLogLevelFlags,
				      msg: *const gchar,
				      user_data: gpointer)   // C 1024-1030
{
  if strstr (msg, c"visible children".as_ptr ()).is_null ()
    { fprintf (stderr, c"XX %s-WARNING **: %s\n".as_ptr (), log_domain, msg); }
}

/* C 1033: `#if defined HAVE_GTK3 && defined HAVE_XINPUT2`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_xinput2"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_is_menu_window (dpy: *mut Display, wdesc: Window) -> bool
{                                                        // C 1034-1051
  let gwdesc: *mut GtkWidget = xg_win_to_widget (dpy, wdesc);

  if GTK_IS_WINDOW! (gwdesc)
    {
      let fw: *mut GtkWidget = gtk_bin_get_child (GTK_BIN! (gwdesc));
      if GTK_IS_MENU! (fw)
	{
	  let parent: *mut GtkWidget
	    = gtk_menu_shell_get_parent_shell (GTK_MENU_SHELL! (fw));
	  return GTK_IS_MENU_BAR! (parent);
	}
    }

  return false;
}

/* C 1054-1057's comment, C 1059-1139's `xg_set_geometry`.

   C 1065's `#if ! GTK_CHECK_VERSION (3, 22, 0)` opens *inside* the block C 1063 opens and its
   `#endif` (C 1104) closes after C 1066's `if (x_gtk_use_window_move)` body: an older GTK takes
   C 1066's `if` with C 1106-1136's `else` arm, a GTK >= 3.22 takes C 1069-1103's statements alone.
   C's statements appear once in C's text, so the port keeps one copy of them and puts the two
   features on the *arm chooser* (`take_body`), C's `if`/`else` and both bodies following in C's
   order - the one `## Deviations` entry this shape needs.  */
/* C 1076/1082's two callees, declared at this unit's own use site (P-72 rule 4): their definition
   is `src/xterm.rs`'s C 18481-18497 pair, which landed after this unit, and `include/gtk.rs`'s
   copy of the pair had to go with it - this unit and that one both include the mirror, so a copy
   there is `E0428 the name ... is defined multiple times` in every `use_gtk` arm (the
   `include/frame.rs` 45-50 rule).  The parameter keeps the P-55 stand-in `*mut c_void`, because
   C 1076/1082's call sites cast (the `include/xsettings.rs` 12-14 reading of the same pair).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" {
  /// C: xterm.h 1863: `extern int x_display_pixel_height (struct x_display_info *);`
  pub fn x_display_pixel_height (dpyinfo: *mut c_void) -> c_int;
  /// C: xterm.h 1864: `extern int x_display_pixel_width (struct x_display_info *);`
  pub fn x_display_pixel_width (dpyinfo: *mut c_void) -> c_int;
}
#[cfg (feature = "use_gtk")]
unsafe fn xg_set_geometry (f: *mut frame)                // C 1059-1139
{
  if ((*f).size_hint_flags as c_long & (USPosition | PPosition)) != 0
    {
      let scale: c_int = xg_get_scale (f);
      #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
      let take_body: bool = x_gtk_use_window_move;
      #[cfg (feature = "gtk_check_version_ge_3_22")]
      let take_body: bool = true;

      if take_body
	{
	  /* Handle negative positions without consulting
	     gtk_window_parse_geometry (Bug#25851).  The position will
	     be off by scrollbar width + window manager decorations.  */
	  #[cfg (not (feature = "have_pgtk"))]
	  {
	    if ((*f).size_hint_flags & XNegative) != 0
	      { (*f).left_pos = ((if ! FRAME_PARENT_FRAME (f).is_null ()
				     { FRAME_PIXEL_WIDTH (FRAME_PARENT_FRAME (f)) }
				   else
				     { x_display_pixel_width (FRAME_DISPLAY_INFO! (f) as *mut c_void) })
				  - FRAME_PIXEL_WIDTH (f) + (*f).left_pos); }

	    if ((*f).size_hint_flags & YNegative) != 0
	      { (*f).top_pos = ((if ! FRAME_PARENT_FRAME (f).is_null ()
				    { FRAME_PIXEL_HEIGHT (FRAME_PARENT_FRAME (f)) }
				  else
				    { x_display_pixel_height (FRAME_DISPLAY_INFO! (f) as *mut c_void) })
				 - FRAME_PIXEL_HEIGHT (f) + (*f).top_pos); }
	  }
	  #[cfg (feature = "have_pgtk")]
	  {
	    if ((*f).size_hint_flags & XNegative) != 0
	      { (*f).left_pos = ((if ! FRAME_PARENT_FRAME (f).is_null ()
				     { FRAME_PIXEL_WIDTH (FRAME_PARENT_FRAME (f)) }
				   else
				     { pgtk_display_pixel_width (FRAME_DISPLAY_INFO! (f)
								 as *mut pgtk_display_info) })
				  - FRAME_PIXEL_WIDTH (f) + (*f).left_pos); }

	    if ((*f).size_hint_flags & YNegative) != 0
	      { (*f).top_pos = ((if ! FRAME_PARENT_FRAME (f).is_null ()
				    { FRAME_PIXEL_HEIGHT (FRAME_PARENT_FRAME (f)) }
				  else
				    { pgtk_display_pixel_height (FRAME_DISPLAY_INFO! (f)
								 as *mut pgtk_display_info) })
				 - FRAME_PIXEL_HEIGHT (f) + (*f).top_pos); }
	  }

	  /* GTK works in scaled pixels, so convert from X pixels.  */
	  gtk_window_move (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
			   (*f).left_pos / scale, (*f).top_pos / scale);

	  /* Reset size hint flags.  */
	  (*f).size_hint_flags &= ! (XNegative | YNegative);
	}
      /* C 1106-1136: the `else` arm an older GTK reaches (C 1105's `}` and C 1136's `}` are C's
	 own braces around it).  */
      else
	{
	  #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
	  {
	    /* GTK works in scaled pixels, so convert from X pixels.  */
	    let mut left: c_int = (*f).left_pos / scale;
	    let xneg: c_int = (*f).size_hint_flags & XNegative;
	    let mut top: c_int = (*f).top_pos / scale;
	    let yneg: c_int = (*f).size_hint_flags & YNegative;
	    /* C 1113: `char geom_str[sizeof "=x--" + 4 * INT_STRLEN_BOUND (int)];` */
	    let mut geom_str: [c_char; b"=x--".len () + 1 + 4 * INT_STRLEN_BOUND! (c_int)]
	      = [0; b"=x--".len () + 1 + 4 * INT_STRLEN_BOUND! (c_int)];
	    let mut id: guint;

	    if xneg != 0
	      { left = - left; }
	    if yneg != 0
	      { top = - top; }

	    sprintf (geom_str.as_mut_ptr (), c"=%dx%d%c%d%c%d".as_ptr (),
		     FRAME_PIXEL_WIDTH (f),
		     FRAME_PIXEL_HEIGHT (f),
		     (if xneg != 0 { b'-' } else { b'+' }) as c_int, left,
		     (if yneg != 0 { b'-' } else { b'+' }) as c_int, top);

	    /* Silence warning about visible children.  */
	    id = g_log_set_handler (c"Gtk".as_ptr (),
				    G_LOG_LEVEL_WARNING | G_LOG_FLAG_FATAL | G_LOG_FLAG_RECURSION,
				    Some (my_log_handler), std::ptr::null_mut ());

	    if gtk_window_parse_geometry (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
					  geom_str.as_ptr ()) == 0
	      { fprintf (stderr, c"Failed to parse: '%s'\n".as_ptr (), geom_str.as_ptr ()); }

	    g_log_remove_handler (c"Gtk".as_ptr (), id);
	  }
	}
    }
}

/** Function to handle resize of native frame F to WIDTH and HEIGHT
    pixels after we got a ConfigureNotify event.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_frame_resized (f: *mut frame, width: c_int, height: c_int)
{                                                        // C 1143-1172
  /* Ignore case where size of native rectangle didn't change.  */
  if width != FRAME_PIXEL_WIDTH (f)
      || height != FRAME_PIXEL_HEIGHT (f)
      || ((*f).new_size_p
	  && (((*f).new_width >= 0 && width != (*f).new_width)
	      || ((*f).new_height >= 0 && height != (*f).new_height)))
    {
      if CONSP (frame_size_history)
	{ frame_size_history_extra
	    (f, build_string (c"xg_frame_resized, changed".as_ptr ()),
	     FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f), width, height,
	     if (*f).new_size_p { (*f).new_width } else { -1 },
	     if (*f).new_size_p { (*f).new_height } else { -1 }); }

      (*(FRAME_RIF! (f) as *mut redisplay_interface)).clear_under_internal_border
	.unwrap_unchecked () (f as *mut c_void);
      change_frame_size (f, width, height, false, true, false);
      SET_FRAME_GARBAGED! (f);
      cancel_mouse_face (f);
    }
  else if CONSP (frame_size_history)
    { frame_size_history_extra
	(f, build_string (c"xg_frame_resized, unchanged".as_ptr ()),
	 FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f), width, height,
	 if (*f).new_size_p { (*f).new_width } else { -1 },
	 if (*f).new_size_p { (*f).new_height } else { -1 }); }

}

/** Resize the outer window of frame F.  WIDTH and HEIGHT are the new
    native pixel sizes of F.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_frame_set_char_size (f: *mut frame, mut width: c_int,
						  mut height: c_int)
{                                                        // C 1176-1296
  let mut fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());
  let mut gwidth: gint = 0;
  let mut gheight: gint = 0;
  let mut outer_height: c_int
    = height + FRAME_TOOLBAR_HEIGHT! (f) + FRAME_MENUBAR_HEIGHT! (f);
  let mut outer_width: c_int = width + FRAME_TOOLBAR_WIDTH! (f);
  let scale: c_int = xg_get_scale (f);

  #[cfg (not (feature = "have_pgtk"))]
  {
    gtk_window_get_size (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), &raw mut gwidth,
			 &raw mut gheight);
  }
  #[cfg (feature = "have_pgtk")]
  {
    if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      {
	gtk_window_get_size (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), &raw mut gwidth,
			     &raw mut gheight);
      }
    else
      {
	let mut alloc: GtkAllocation = GtkAllocation { x: 0, y: 0, width: 0, height: 0 };
	gtk_widget_get_allocation (FRAME_GTK_WIDGET! (f), &raw mut alloc);
	gwidth = alloc.width;
	gheight = alloc.height;
      }
  }

  outer_height /= scale;
  outer_width /= scale;
  height = height / scale * scale;
  width = width / scale * scale;

  xg_wm_set_size_hint (f, 0, false);

  /* Resize the top level widget so rows and columns remain constant.

     When the frame is fullheight and we only want to change the width
     or it is fullwidth and we only want to change the height we should
     be able to preserve the fullscreen property.  However, due to the
     fact that we have to send a resize request anyway, the window
     manager will abolish it.  At least the respective size should
     remain unchanged but giving the frame back its normal size will
     be broken ... */
  if EQ (fullscreen, Qfullwidth ()) && width == FRAME_PIXEL_WIDTH (f)
    { outer_width = gwidth; }
  else if EQ (fullscreen, Qfullheight ()) && height == FRAME_PIXEL_HEIGHT (f)
    { outer_height = gheight; }
  else
    { fullscreen = Qnil (); }

  #[cfg (not (feature = "have_pgtk"))]
  {
    if ! FRAME_PARENT_FRAME (f).is_null ()
      {
	/* Send the resize request immediately.  */
	gdk_window_resize (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f)),
			   outer_width, outer_height);
	/* Resize all inner widgets and Cairo surface right away so the
	   next redisplay drawing isn't clipped to the old size.  */
	let mut alloc: GtkAllocation = GtkAllocation { x: 0, y: 0, width: outer_width,
						       height: outer_height };
	gtk_widget_size_allocate (FRAME_GTK_OUTER_WIDGET! (f), &raw mut alloc);
	#[cfg (feature = "use_cairo")]
	{ x_cr_update_surface_desired_size (f, width, height); }
      }
    gtk_window_resize (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), outer_width, outer_height);
  }
  #[cfg (feature = "have_pgtk")]
  {
    if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      { gtk_window_resize (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), outer_width,
			   outer_height); }
    else				/* PGTK child frame.  */
      { gtk_widget_set_size_request (FRAME_GTK_WIDGET! (f), outer_width, outer_height); }
  }

  SET_FRAME_GARBAGED! (f);
  cancel_mouse_face (f);

  /* We can not call change_frame_size for a mapped frame,
     we can not set pixel width/height either.  The window manager may
     override our resize request, XMonad does this all the time.
     The best we can do is try to sync, so lisp code sees the updated
     size as fast as possible.
     For unmapped windows, we can set rows/cols.  When
     the frame is mapped again we will (hopefully) get the correct size.  */
  if FRAME_VISIBLE_P (f) && FRAME_PARENT_FRAME (f).is_null ()
    {
      if CONSP (frame_size_history)
	{ frame_size_history_extra
	    (f, build_string (c"xg_frame_set_char_size, visible".as_ptr ()),
	     FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f), width, height,
	     (*f).new_width, (*f).new_height); }

      /* Must call this to flush out events */
      gtk_events_pending ();
      gdk_flush ();
      #[cfg (not (feature = "have_pgtk"))]
      { x_wait_for_event (f, ConfigureNotify); }

      if ! NILP (fullscreen)
	/* Try to restore fullscreen state.  */
	{
	  store_frame_param (f, Qfullscreen (), fullscreen);
	  gui_set_fullscreen (f, fullscreen, fullscreen);
	}
    }
  else
    {
      if CONSP (frame_size_history)
	{ frame_size_history_extra
	    (f, build_string (c"xg_frame_set_char_size, invisible".as_ptr ()),
	     FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f), width, height,
	     (*f).new_width, (*f).new_height); }

      adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),
			 5, false, Qxg_frame_set_char_size ());
    }
}
/* ============================================================================================== */

/* The C-side names the second slice's text uses that the mirrors do not carry yet (P-72
   rule 4: the unit declares them at the use site and the ledger owes the move).  */
/// C: frame.h 1840: `#define EMACS_CLASS "Emacs"` - C 1650/1855 pass it to `gtk_widget_set_name`.
/// `include/frame.rs` does not carry it, so it stands here; `## Owed` records the move.
#[cfg (feature = "use_gtk")]
pub const EMACS_CLASS: &core::ffi::CStr = c"Emacs";
/// termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
/// `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80).  C 1488/1528 write `inev.kind = x;`
/// and `event.kind = x;`, so the port keeps C's field name in a per-unit macro - the
/// `src/androidterm.rs` 201-207 shape, body-identical.
#[cfg (feature = "use_gtk")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}
/* ==============================================================================================
   C 1297-2281: the second slice.  It opens with the dead `#if 0` block's text (recorded as a
   comment, P-86 ①) and runs through `xg_set_frame_icon`'s `#endif` - the next live definition is
   `get_dialog_title` (C 2288), which the dialog section (C 2282's banner) opens.
   ============================================================================================== */

/* C 1298-1300: C's comment above the dead block:
   "Handle height/width changes (i.e. add/remove/move menu/toolbar).
    The policy is to keep the number of editable lines."  */

/* C 1301-1311: `#if 0` / `static void xg_height_or_width_changed (struct frame *f)` / its body /
   `#endif` - dead text C itself excludes (both of C's arms are this text and the empty `#else`).
   Recorded as a comment; every line is C's own.  */
/*
#if 0
static void
xg_height_or_width_changed (struct frame *f)
{
  gtk_window_resize (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)),
                     FRAME_TOTAL_PIXEL_WIDTH (f),
                     FRAME_TOTAL_PIXEL_HEIGHT (f));
  f->output_data.xp->hint_flags = 0;
  x_wm_set_size_hint (f, 0, 0);
}
#endif
*/

/** Move and resize the outer window of frame F.  WIDTH and HEIGHT are
    the new native pixel sizes of F.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_frame_set_size_and_position (f: *mut frame, mut width: c_int,
							 mut height: c_int)
{                                                        // C 1316-1391
  let mut outer_height: c_int
    = height + FRAME_TOOLBAR_HEIGHT! (f) + FRAME_MENUBAR_HEIGHT! (f);
  let mut outer_width: c_int = width + FRAME_TOOLBAR_WIDTH! (f);
  let scale: c_int = xg_get_scale (f);
  let mut x: c_int = (*f).left_pos;
  let mut y: c_int = (*f).top_pos;
  let mut gwin: *mut GdkWindow = std::ptr::null_mut ();
  let mut flags: c_int = 0;

  if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    { gwin = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f)); }
  else
    { gwin = gtk_widget_get_window (FRAME_GTK_WIDGET! (f)); }

  outer_height /= scale;
  outer_width /= scale;
  height = height / scale * scale;
  width = width / scale * scale;

  x /= scale;
  y /= scale;

  /* Full force ahead.  For top-level frames the gravity will get reset
     to NorthWestGravity anyway.  */
  flags |= USSize as c_int;
  if (*f).win_gravity == 3 || (*f).win_gravity == 9
    { flags |= XNegative; }
  if (*f).win_gravity == 6 || (*f).win_gravity == 9
    { flags |= YNegative; }
  flags |= USPosition as c_int;

  xg_wm_set_size_hint (f, flags as c_long, true);

  #[cfg (not (feature = "have_pgtk"))]
  {
    gdk_window_move_resize (gwin, x, y, outer_width, outer_height);
    if ! FRAME_PARENT_FRAME (f).is_null ()
      {
	/* Resize all inner widgets and Cairo surface right away so the
	   next redisplay drawing isn't clipped to the old size.  */
	let mut alloc: GtkAllocation = GtkAllocation { x: 0, y: 0, width: outer_width,
						       height: outer_height };
	gtk_widget_size_allocate (FRAME_GTK_OUTER_WIDGET! (f), &raw mut alloc);
	#[cfg (feature = "use_cairo")]
	{ x_cr_update_surface_desired_size (f, width, height); }
      }
  }
  #[cfg (feature = "have_pgtk")]
  {
    if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      { gdk_window_move_resize (gwin, x, y, outer_width, outer_height); }
    else				/* PGTK child frame.  */
      { gtk_widget_set_size_request (FRAME_GTK_WIDGET! (f),
				     outer_width, outer_height); }
  }

  SET_FRAME_GARBAGED! (f);
  cancel_mouse_face (f);

  /* For child frames, don't wait for events — that would flush the X
     buffer and might show outdated contents in the frame.  Same for
     invisible frames: this way is faster and x_make_frame_visible will
     wait for event anyway.  */
  if FRAME_VISIBLE_P (f) && FRAME_PARENT_FRAME (f).is_null ()
    {
      /* Must call this to flush out events */
      gtk_events_pending ();
      gdk_flush ();
      #[cfg (not (feature = "have_pgtk"))]
      { x_wait_for_event (f, ConfigureNotify); }
    }
  else
    {
      adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),
			 5, false, Qxg_frame_set_char_size ());
    }
}

#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
/* Convert an X Window WSESC on display DPY to its corresponding GtkWidget.
   Must be done like this, because GtkWidget:s can have "hidden"
   X Window that aren't accessible.

   Return 0 if no widget match WDESC.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_win_to_widget (dpy: *mut Display, wdesc: Window) -> *mut GtkWidget
{                                                        // C 1401-1420
  let mut gdkwin: gpointer;
  let mut gwdesc: *mut GtkWidget = std::ptr::null_mut ();

  block_input ();

  gdkwin = gdk_x11_window_lookup_for_display (gdk_x11_lookup_xdisplay (dpy), wdesc);
  if ! gdkwin.is_null ()
    {
      let mut event: GdkEvent = GdkEvent { any: GdkEventAny {
	  r#type: GDK_NOTHING, window: std::ptr::null_mut (), send_event: 0 } };
      event.any.window = gdkwin as *mut GdkWindow;
      event.any.r#type = GDK_NOTHING;
      gwdesc = gtk_get_event_widget (&raw mut event);
    }

  unblock_input ();
  return gwdesc;
}

/* Set the background of widget W to PIXEL.  */

#[cfg (feature = "use_gtk")]
unsafe fn xg_set_widget_bg (f: *mut frame, w: *mut GtkWidget, pixel: c_ulong)   // C 1426-1458
{
  #[cfg (feature = "have_gtk3")]
  {
    let mut xbg: Emacs_Color = Emacs_Color { pixel: 0, red: 0, green: 0, blue: 0 };
    xbg.pixel = pixel;
    /* C 1431/1433: the two arms of `#ifndef HAVE_PGTK` fill `xbg` differently but share C 1441-
       1451's block.  C's X arm makes the block the body of `if (XQueryColor (...))`; the pgtk
       arm runs its six assignments and then the block unconditionally.  The port puts the arm's
       truth value in one `cond` and keeps the block once (the `xg_set_geometry` `take_body`
       shape, `## Deviations`).  */
    #[cfg (not (feature = "have_pgtk"))]
    let cond: bool = XQueryColor (FRAME_X_DISPLAY! (f), FRAME_X_COLORMAP! (f),
				  &raw mut xbg) != 0;
    #[cfg (feature = "have_pgtk")]
    let cond: bool = {
      xbg.red = ((pixel >> 16) & 0xff) as c_ushort;
      xbg.green = ((pixel >> 8) & 0xff) as c_ushort;
      xbg.blue = ((pixel >> 0) & 0xff) as c_ushort;
      xbg.red |= xbg.red << 8;
      xbg.green |= xbg.green << 8;
      xbg.blue |= xbg.blue << 8;
      true
    };
    if cond
      {
	/* C 1442-1443: `static char const format[] = "* { background-color: #%02x%02x%02x; }";` /
	   `char buffer[sizeof format + 3 * INT_STRLEN_BOUND (xbg.red)];` - `format` keeps C's name
	   and `sizeof format` is the array's size including the NUL, which a const byte-string's
	   length gives (`xg_set_geometry`'s 1113 shape).  */
	const format: &[u8] = b"* { background-color: #%02x%02x%02x; }";
	let mut buffer: [c_char; format.len () + 1 + 3 * INT_STRLEN_BOUND! (c_ushort)]
	  = [0; format.len () + 1 + 3 * INT_STRLEN_BOUND! (c_ushort)];
	sprintf (buffer.as_mut_ptr (), format.as_ptr () as *const c_char,
		 (xbg.red >> 8) as c_int, (xbg.green >> 8) as c_int, (xbg.blue >> 8) as c_int);
	let mut provider: *mut GtkCssProvider = gtk_css_provider_new ();
	gtk_css_provider_load_from_data (provider, buffer.as_ptr (), -1, std::ptr::null_mut ());
	gtk_style_context_add_provider (gtk_widget_get_style_context (w),
					GTK_STYLE_PROVIDER! (provider),
					GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
	g_clear_object! (&raw mut provider);
      }
  }
  #[cfg (not (feature = "have_gtk3"))]
  {
    let mut bg: GdkColor = GdkColor { pixel: 0, red: 0, green: 0, blue: 0 };
    let map: *mut GdkColormap = gtk_widget_get_colormap (w);
    gdk_colormap_query_color (map, pixel, &raw mut bg);
    gtk_widget_modify_bg (FRAME_GTK_WIDGET! (f), GTK_STATE_NORMAL, &raw mut bg);
  }
}

/* Apply dark mode preference to GTK window decorations.  */

#[cfg (all (feature = "use_gtk", feature = "have_pgtk", feature = "have_gsettings"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_gtk_theme_dark_mode (dark_mode_p: bool,
						     settings: *mut GtkSettings)
{                                                        // C 1464-1468
  g_object_set (settings, c"gtk-application-prefer-dark-theme".as_ptr (),
		(if dark_mode_p { TRUE } else { FALSE }) as c_int, std::ptr::null_mut ());
}

/* Update all frames' dark mode based on system setting.  */

#[cfg (all (feature = "use_gtk", feature = "have_pgtk", feature = "have_gsettings"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_update_dark_mode_for_all_displays (dark_mode_p: bool)
{                                                        // C 1473-1492
   let mut dpyinfo: *mut pgtk_display_info;
   dpyinfo = x_display_list;
   while ! dpyinfo.is_null ()
     {
       let screen: *mut GdkScreen
	 = gdk_display_get_default_screen ((*dpyinfo).gdpy);
       let settings: *mut GtkSettings
	 = gtk_settings_get_for_screen (screen);
       xg_set_gtk_theme_dark_mode (dark_mode_p, settings);
       dpyinfo = (*dpyinfo).next;
     }

   {
     let mut inev: input_event = std::mem::zeroed ();
     EVENT_INIT (&raw mut inev);
     input_event_bits! (kind, &raw mut inev, TOOLKIT_THEME_CHANGED_EVENT);
     inev.arg = if dark_mode_p { Qdark () } else { Qlight () };
     kbd_buffer_store_event (&raw mut inev as *mut c_void);
   }
}

/* Set initial dark mode for a new frame (called during frame
 * creation).  */

#[cfg (all (feature = "use_gtk", feature = "have_pgtk", feature = "have_gsettings"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_initial_dark_mode (f: *mut frame)   // C 1498-1502
{
  let dark_mode_p: bool = xg_get_system_dark_mode ();
  xg_update_dark_mode_for_all_displays (dark_mode_p);
}

/* Callback called when the gtk theme changes.
   We notify lisp code so it can fix faces used for region for example.  */

#[cfg (feature = "use_gtk")]
unsafe extern "C" fn style_changed_cb (go: *mut GObject, spec: *mut GParamSpec,
				       user_data: gpointer)   // C 1509-1559
{
  let mut event: input_event;
  let gdpy: *mut GdkDisplay = user_data as *mut GdkDisplay;
  let mut display_name: *const c_char = gdk_display_get_name (gdpy);
  #[cfg (not (feature = "have_pgtk"))]
  let dpy: *mut Display = GDK_DISPLAY_XDISPLAY! (gdpy);
  #[cfg (feature = "have_pgtk")]
  let dpy: *mut GdkDisplay = gdpy;

  #[cfg (not (feature = "have_pgtk"))]
  { if display_name.is_null () { display_name = c"".as_ptr (); } }

  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, CONFIG_CHANGED_EVENT);
  event.frame_or_window = build_string (display_name);
  event.arg = Qtheme_name ();
  kbd_buffer_store_event (&raw mut event as *mut c_void);

  update_theme_scrollbar_width ();
  update_theme_scrollbar_height ();

  /* If scroll bar width changed, we need set the new size on all frames
     on this display.  */
  if ! dpy.is_null ()
    {
      let mut rest: Lisp_Object;
      let mut frame: Lisp_Object;
      FOR_EACH_FRAME! (rest, frame, {
	  let f: *mut frame = XFRAME (frame);
	  /* C 1546/1548: `#ifndef HAVE_PGTK` tests `FRAME_X_P (f)`, the `#else` `FRAME_PGTK_P
	     (f)`; the arm's predicate is one `let` so C's `if` stays one statement.  */
	  #[cfg (not (feature = "have_pgtk"))]
	  let frame_p: bool = FRAME_X_P (f);
	  #[cfg (feature = "have_pgtk")]
	  let frame_p: bool = FRAME_PGTK_P (f);
	  if FRAME_LIVE_P (f)
	      && frame_p
	      && FRAME_X_DISPLAY! (f) == dpy
	    {
	      ((*FRAME_TERMINAL! (f)).set_scroll_bar_default_width_hook.unwrap_unchecked ())
		(f as *mut c_void);
	      ((*FRAME_TERMINAL! (f)).set_scroll_bar_default_height_hook.unwrap_unchecked ())
		(f as *mut c_void);
	      xg_frame_set_char_size (f, FRAME_PIXEL_WIDTH (f),
				      FRAME_PIXEL_HEIGHT (f));
	    }
	});
    }
}

/* Called when a delete-event occurs on WIDGET.  */

#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" fn delete_cb (widget: *mut GtkWidget, event: *mut GdkEvent,
				user_data: gpointer) -> gboolean   // C 1564-1570
{
  return TRUE;
}
/* Create and set up the GTK widgets for frame F.
   Return true if creation succeeded.  */

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_create_frame_widgets (f: *mut frame) -> bool   // C 1577-1831
{
  let mut wtop: *mut GtkWidget = std::ptr::null_mut ();
  let mut wvbox: *mut GtkWidget = std::ptr::null_mut ();
  let mut whbox: *mut GtkWidget = std::ptr::null_mut ();
  let mut wfixed: *mut GtkWidget = std::ptr::null_mut ();
  #[cfg (not (feature = "have_gtk3"))]
  let mut style: *mut GtkRcStyle = std::ptr::null_mut ();
  #[cfg (not (feature = "have_pgtk"))]
  let mut imc: *mut GtkIMContext = std::ptr::null_mut ();
  let mut r#type: GtkWindowType = GTK_WINDOW_TOPLEVEL;
  let mut title: *mut c_char = std::ptr::null_mut ();

  block_input ();

  #[cfg (not (feature = "have_pgtk"))]  /* gtk_plug not found. */
  {
    if FRAME_X_EMBEDDED_P! (f)
      {
	let gdpy: *mut GdkDisplay = gdk_x11_lookup_xdisplay (FRAME_X_DISPLAY! (f));
	wtop = gtk_plug_new_for_display (gdpy,
					 (*((*f).output_data.x as *mut xp_output)).parent_desc);
      }
    else
      { wtop = gtk_window_new (r#type); }
  }
  #[cfg (feature = "have_pgtk")]
  {
    if (*f).tooltip
      { r#type = GTK_WINDOW_POPUP; }
    wtop = gtk_window_new (r#type);
    gtk_widget_add_events (wtop, GDK_ALL_EVENTS_MASK);
  }

  gtk_widget_set_app_paintable (wtop, ((*f).alpha_background != 1.0) as gboolean);
  #[cfg (feature = "gtk_check_version_ge_3_10")]
  {
    g_signal_connect! (G_OBJECT! (wtop), c"style-updated".as_ptr (), xg_widget_style_updated,
		       f as gpointer);
  }

  /* gtk_window_set_has_resize_grip is a Gtk+ 3.0 function but Ubuntu
     has backported it to Gtk+ 2.0 and they add the resize grip for
     Gtk+ 2.0 applications also.  But it has a bug that makes Emacs loop
     forever, so disable the grip.  */
  #[cfg (all (not (feature = "have_gtk3"), feature = "have_gtk_window_set_has_resize_grip"))]
  { gtk_window_set_has_resize_grip (GTK_WINDOW! (wtop), FALSE); }

  xg_set_screen (wtop, f);

  wvbox = gtk_box_new! (GTK_ORIENTATION_VERTICAL, 0);
  whbox = gtk_box_new! (GTK_ORIENTATION_HORIZONTAL, 0);
  gtk_box_set_homogeneous (GTK_BOX! (wvbox), FALSE);
  gtk_box_set_homogeneous (GTK_BOX! (whbox), FALSE);

  #[cfg (feature = "have_gtk3")]
  { wfixed = emacs_fixed_new (f as *mut c_void) as *mut GtkWidget; }
  #[cfg (not (feature = "have_gtk3"))]
  { wfixed = gtk_fixed_new (); }

  if wtop.is_null () || wvbox.is_null () || whbox.is_null () || wfixed.is_null ()
    {
      if ! wtop.is_null () { gtk_widget_destroy (wtop); }
      if ! wvbox.is_null () { gtk_widget_destroy (wvbox); }
      if ! whbox.is_null () { gtk_widget_destroy (whbox); }
      if ! wfixed.is_null () { gtk_widget_destroy (wfixed); }

      unblock_input ();
      return false;
    }

  /* Use same names as the Xt port does.  I.e. Emacs.pane.emacs by default */
  gtk_widget_set_name (wtop, EMACS_CLASS.as_ptr ());
  gtk_widget_set_name (wvbox, c"pane".as_ptr ());
  gtk_widget_set_name (wfixed, SSDATA (Vx_resource_name));

  /* If this frame has a title or name, set it in the title bar.  */
  if ! NILP ((*f).title)
    { title = SSDATA (ENCODE_UTF_8 ((*f).title)); }
  else if ! NILP ((*f).name)
    { title = SSDATA (ENCODE_UTF_8 ((*f).name)); }

  if ! title.is_null ()
    { gtk_window_set_title (GTK_WINDOW! (wtop), title); }

  if FRAME_UNDECORATED (f) != 0
    {
      gtk_window_set_decorated (GTK_WINDOW! (wtop), FALSE);
      store_frame_param (f, Qundecorated (), Qt ());
    }

  FRAME_GTK_OUTER_WIDGET! (f) = wtop;
  FRAME_GTK_WIDGET! (f) = wfixed;
  (*((*f).output_data.x as *mut xp_output)).vbox_widget = wvbox;
  (*((*f).output_data.x as *mut xp_output)).hbox_widget = whbox;

  gtk_widget_set_has_window (wfixed, TRUE);

  gtk_container_add (GTK_CONTAINER! (wtop), wvbox);
  gtk_box_pack_start (GTK_BOX! (wvbox), whbox, TRUE, TRUE, 0);
  gtk_box_pack_start (GTK_BOX! (whbox), wfixed, TRUE, TRUE, 0);

  if FRAME_EXTERNAL_TOOL_BAR (f)
    { update_frame_tool_bar (f); }

  /* We don't want this widget double buffered, because we draw on it
     with regular X drawing primitives, so from a GTK/GDK point of
     view, the widget is totally blank.  When an expose comes, this
     will make the widget blank, and then Emacs redraws it.  This flickers
     a lot, so we turn off double buffering.  */

  #[cfg (not (feature = "have_pgtk"))]
  { gtk_widget_set_double_buffered (wfixed, FALSE); }

  #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
  {
    gtk_window_set_wmclass (GTK_WINDOW! (wtop),
			    SSDATA (Vx_resource_name),
			    SSDATA (Vx_resource_class));
  }

  #[cfg (not (feature = "have_pgtk"))]
  {
    /* Add callback to do nothing on WM_DELETE_WINDOW.  The default in
       GTK is to destroy the widget.  We want Emacs to do that instead.  */
    g_signal_connect! (G_OBJECT! (wtop), c"delete-event".as_ptr (), delete_cb, f as gpointer);
  }

  /* Convert our geometry parameters into a geometry string
     and specify it.
     GTK will itself handle calculating the real position this way.  */
  xg_set_geometry (f);
  (*f).win_gravity
    = gtk_window_get_gravity (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)));

  /* C 1714-1730: C's one `gtk_widget_add_events` splits by arm around `GDK_EXPOSURE_MASK`
     (`#ifndef HAVE_PGTK`) and the three scroll/touch masks (`#ifdef HAVE_PGTK`); the port adds
     the arm's terms to one `events` and calls once (the arm-chooser shape).  */
  let mut events: c_int = GDK_POINTER_MOTION_MASK;
  #[cfg (not (feature = "have_pgtk"))]
  { events |= GDK_EXPOSURE_MASK; }
  events |= GDK_BUTTON_PRESS_MASK
    | GDK_BUTTON_RELEASE_MASK
    | GDK_KEY_PRESS_MASK
    | GDK_ENTER_NOTIFY_MASK
    | GDK_LEAVE_NOTIFY_MASK
    | GDK_FOCUS_CHANGE_MASK
    | GDK_STRUCTURE_MASK;
  #[cfg (feature = "have_pgtk")]
  {
    events |= GDK_SCROLL_MASK
      | GDK_SMOOTH_SCROLL_MASK
      | GDK_TOUCH_MASK;
  }
  events |= GDK_VISIBILITY_NOTIFY_MASK;
  gtk_widget_add_events (wfixed, events);

  let screen: *mut GdkScreen = gtk_widget_get_screen (wtop);

  #[cfg (not (feature = "have_pgtk"))]
  let visual: *mut GdkVisual
    = gdk_x11_screen_lookup_visual (screen,
				    XVisualIDFromVisual (FRAME_X_VISUAL! (f)));
  #[cfg (feature = "have_pgtk")]
  let visual: *mut GdkVisual = gdk_screen_get_rgba_visual (screen);

  if visual.is_null ()
    { emacs_abort (); }

  gtk_widget_set_visual (wtop, visual);
  gtk_widget_set_visual (wfixed, visual);

  #[cfg (not (feature = "have_pgtk"))]
  {
    /* Must realize the windows so the X window gets created.  It is used
       by callers of this function.  */
    gtk_widget_realize (wfixed);
    FRAME_X_WINDOW! (f) = GTK_WIDGET_TO_X_WIN! (wfixed);
    initial_set_up_x_back_buffer (f);
  }

  /* Since GTK clears its window by filling with the background color,
     we must keep X and GTK background in sync.  */
  xg_set_widget_bg (f, wfixed, FRAME_BACKGROUND_PIXEL (f));

  #[cfg (not (feature = "have_gtk3"))]
  {
    /* Also, do not let any background pixmap to be set, this looks very
       bad as Emacs overwrites the background pixmap with its own idea
       of background color.  */
    style = gtk_widget_get_modifier_style (wfixed);

    /* Must use g_strdup because gtk_widget_modify_style does g_free.  */
    (*style).bg_pixmap_name[GTK_STATE_NORMAL] = g_strdup (c"<none>".as_ptr ());
    gtk_widget_modify_style (wfixed, style);
    gtk_widget_set_can_focus (wfixed, TRUE);
  }
  #[cfg (feature = "have_gtk3")]
  {
    gtk_widget_set_can_focus (wfixed, TRUE);
    #[cfg (feature = "have_pgtk")]
    { gtk_widget_grab_focus (wfixed); }
    gtk_window_set_resizable (GTK_WINDOW! (wtop), TRUE);
  }

  if FRAME_OVERRIDE_REDIRECT (f) != 0
    {
      let gwin: *mut GdkWindow = gtk_widget_get_window (wtop);

      if ! gwin.is_null ()
	{ gdk_window_set_override_redirect (gwin, TRUE); }
    }

  /* Steal a tool tip window we can move ourselves.  */
  (*((*f).output_data.x as *mut xp_output)).ttip_widget = std::ptr::null_mut ();
  (*((*f).output_data.x as *mut xp_output)).ttip_lbl = std::ptr::null_mut ();
  (*((*f).output_data.x as *mut xp_output)).ttip_window = std::ptr::null_mut ();
  #[cfg (not (feature = "have_pgtk"))]
  {
    gtk_widget_set_tooltip_text (wtop, c"Dummy text".as_ptr ());
    g_signal_connect! (wtop, c"query-tooltip".as_ptr (), qttip_cb, f as gpointer);

    imc = gtk_im_multicontext_new ();
    gtk_im_context_set_use_preedit (imc, TRUE);

    g_signal_connect! (G_OBJECT! (imc), c"commit".as_ptr (), xg_im_context_commit, f as gpointer);
    g_signal_connect! (G_OBJECT! (imc), c"preedit-changed".as_ptr (),
		       xg_im_context_preedit_changed, std::ptr::null_mut ());
    g_signal_connect! (G_OBJECT! (imc), c"preedit-end".as_ptr (),
		       xg_im_context_preedit_end, std::ptr::null_mut ());
    (*(FRAME_X_OUTPUT! (f) as *mut xp_output)).im_context = imc;

    g_signal_connect! (G_OBJECT! (wfixed), c"key-press-event".as_ptr (),
		       xg_widget_key_press_event_cb, std::ptr::null_mut ());
  }

  {
    let gs: *mut GtkSettings = gtk_settings_get_for_screen (screen);
    /* Only connect this signal once per screen.  */
    if g_signal_handler_find (G_OBJECT! (gs) as gpointer,
			      G_SIGNAL_MATCH_FUNC,
			      0, 0, std::ptr::null_mut (),
			      core::mem::transmute::<GCallback, gpointer> (G_CALLBACK! (style_changed_cb)),
			      std::ptr::null_mut ()) == 0
      {
        g_signal_connect! (G_OBJECT! (gs), c"notify::gtk-theme-name".as_ptr (),
			   style_changed_cb, gdk_screen_get_display (screen) as gpointer);
      }
  }

  #[cfg (all (feature = "have_pgtk", feature = "have_gsettings"))]
  { xg_set_initial_dark_mode (f); }

  unblock_input ();

  return true;
}

#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_create_frame_outer_widgets (f: *mut frame)   // C 1835-1932
{
  let mut wtop: *mut GtkWidget;
  let mut wvbox: *mut GtkWidget;
  let mut whbox: *mut GtkWidget;
  let mut r#type: GtkWindowType = GTK_WINDOW_TOPLEVEL;
  let mut title: *mut c_char = std::ptr::null_mut ();

  block_input ();

  wtop = gtk_window_new (r#type);
  gtk_widget_add_events (wtop, GDK_ALL_EVENTS_MASK);

  xg_set_screen (wtop, f);

  wvbox = gtk_box_new! (GTK_ORIENTATION_VERTICAL, 0);
  whbox = gtk_box_new! (GTK_ORIENTATION_HORIZONTAL, 0);
  gtk_box_set_homogeneous (GTK_BOX! (wvbox), FALSE);
  gtk_box_set_homogeneous (GTK_BOX! (whbox), FALSE);

  /* Use same names as the Xt port does.  I.e. Emacs.pane.emacs by default */
  gtk_widget_set_name (wtop, EMACS_CLASS.as_ptr ());
  gtk_widget_set_name (wvbox, c"pane".as_ptr ());

  /* If this frame has a title or name, set it in the title bar.  */
  if ! NILP ((*f).title)
    { title = SSDATA (ENCODE_UTF_8 ((*f).title)); }
  else if ! NILP ((*f).name)
    { title = SSDATA (ENCODE_UTF_8 ((*f).name)); }

  if ! title.is_null ()
    { gtk_window_set_title (GTK_WINDOW! (wtop), title); }

  if FRAME_UNDECORATED (f) != 0
    {
      gtk_window_set_decorated (GTK_WINDOW! (wtop), FALSE);
      store_frame_param (f, Qundecorated (), Qt ());
    }

  FRAME_GTK_OUTER_WIDGET! (f) = wtop;
  (*((*f).output_data.x as *mut xp_output)).vbox_widget = wvbox;
  (*((*f).output_data.x as *mut xp_output)).hbox_widget = whbox;

  gtk_container_add (GTK_CONTAINER! (wtop), wvbox);
  gtk_box_pack_start (GTK_BOX! (wvbox), whbox, TRUE, TRUE, 0);

  if FRAME_EXTERNAL_TOOL_BAR (f)
    { update_frame_tool_bar (f); }

  #[cfg (not (feature = "gtk_check_version_ge_3_22"))]
  {
    gtk_window_set_wmclass (GTK_WINDOW! (wtop),
			    SSDATA (Vx_resource_name),
			    SSDATA (Vx_resource_class));
  }

  /* Convert our geometry parameters into a geometry string
     and specify it.
     GTK will itself handle calculating the real position this way.  */
  xg_set_geometry (f);
  (*f).win_gravity
    = gtk_window_get_gravity (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)));

  gtk_window_set_resizable (GTK_WINDOW! (wtop), TRUE);

  if FRAME_OVERRIDE_REDIRECT (f) != 0
    {
      let gwin: *mut GdkWindow = gtk_widget_get_window (wtop);

      if ! gwin.is_null ()
	{ gdk_window_set_override_redirect (gwin, TRUE); }
    }

  /* Steal a tool tip window we can move ourselves.  */
  (*((*f).output_data.x as *mut xp_output)).ttip_widget = std::ptr::null_mut ();
  (*((*f).output_data.x as *mut xp_output)).ttip_lbl = std::ptr::null_mut ();
  (*((*f).output_data.x as *mut xp_output)).ttip_window = std::ptr::null_mut ();

  {
    let screen: *mut GdkScreen = gtk_widget_get_screen (wtop);
    let gs: *mut GtkSettings = gtk_settings_get_for_screen (screen);
    /* Only connect this signal once per screen.  */
    if g_signal_handler_find (G_OBJECT! (gs) as gpointer,
			      G_SIGNAL_MATCH_FUNC,
			      0, 0, std::ptr::null_mut (),
			      core::mem::transmute::<GCallback, gpointer> (G_CALLBACK! (style_changed_cb)),
			      std::ptr::null_mut ()) == 0
      {
        g_signal_connect! (G_OBJECT! (gs), c"notify::gtk-theme-name".as_ptr (),
			   style_changed_cb, gdk_screen_get_display (screen) as gpointer);
      }
  }

  #[cfg (feature = "have_gsettings")]
  { xg_set_initial_dark_mode (f); }

  unblock_input ();
}

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_free_frame_widgets (f: *mut frame)   // C 1936-1976
{
  if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    {
      let x: *mut xp_output = (*f).output_data.x as *mut xp_output;
      let tbinfo: *mut xg_frame_tb_info
        = g_object_get_data (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)),
			     TB_INFO_KEY.as_ptr ()) as *mut xg_frame_tb_info;
      if ! tbinfo.is_null ()
        { xfree (tbinfo as *mut c_void); }

      if ! (*x).toolbar_widget.is_null () && ! (*x).toolbar_is_packed
	{
	  gtk_widget_destroy ((*x).toolbar_widget);
	  (*x).toolbar_widget = std::ptr::null_mut ();
	}

      /* x_free_frame_resources should have taken care of it */
      #[cfg (not (feature = "have_pgtk"))]
      {
	#[cfg (feature = "have_xdbe")]
	{ eassert! (! FRAME_X_DOUBLE_BUFFERED_P! (f)); }
	g_object_unref ((*(FRAME_X_OUTPUT! (f) as *mut xp_output)).im_context as gpointer);
      }
      gtk_widget_destroy (FRAME_GTK_OUTER_WIDGET! (f));
      /* C 1961: `FRAME_X_WINDOW (f) = 0;` - a `Window` (`c_ulong`) in the X arm and a
	 `GtkWidget *` in the pgtk one, which the pgtk mirror now types, so the port splits the
	 arm (the arm-chooser shape, `## Deviations`).  */
      #[cfg (not (feature = "have_pgtk"))]
      { FRAME_X_WINDOW! (f) = 0; }
      #[cfg (feature = "have_pgtk")]
      { FRAME_X_WINDOW! (f) = std::ptr::null_mut (); }
      #[cfg (not (feature = "have_pgtk"))]
      { FRAME_X_RAW_DRAWABLE! (f) = 0; }
      FRAME_GTK_OUTER_WIDGET! (f) = std::ptr::null_mut ();
      if ! (*x).ttip_widget.is_null ()
        {
          /* Remove ttip_lbl from ttip_widget's custom slot before
             destroying it, to avoid double-free (Bug#41239).  */
          gtk_tooltip_set_custom ((*x).ttip_widget, std::ptr::null_mut ());
          g_object_unref (G_OBJECT! ((*x).ttip_widget) as gpointer);
        }
      if ! (*x).ttip_lbl.is_null ()
        { gtk_widget_destroy ((*x).ttip_lbl); }
    }
}

/* Set the normal size hints for the window manager, for frame F.
   FLAGS is the flags word to use--or 0 meaning preserve the flags
   that the window now has.
   If USER_POSITION, set the User Position
   flag (this is useful when FLAGS is 0).  */

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_wm_set_size_hint (f: *mut frame, mut flags: c_long,
					       user_position: bool)   // C 1985-2108
{
  /* Must use GTK routines here, otherwise GTK resets the size hints
     to its own defaults.  */
  let mut size_hints: GdkGeometry = std::mem::zeroed ();  /* C 1989: uninitialized (0.1-13) */
  let mut hint_flags: gint = 0;
  let mut base_width: c_int;
  let mut base_height: c_int;
  let win_gravity: c_int = (*f).win_gravity;
  let mut fs_state: Lisp_Object;
  let mut frame: Lisp_Object;
  let scale: c_int = xg_get_scale (f);

  /* Don't set size hints during initialization; that apparently leads
     to a race condition.  See the thread at
     https://lists.gnu.org/r/emacs-devel/2008-10/msg00033.html  */
  if NILP (Vafter_init_time)
      || FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      || ! FRAME_PARENT_FRAME (f).is_null ()
    { return; }

  XSETFRAME! (frame, f);
  fs_state = Fframe_parameter (frame, Qfullscreen ());
  /* C 2006-2011: the fold's `#ifndef HAVE_PGTK` adds the two `x_wm_supports` tests to the
     maximized/fullscreen test; the port hoists the arm's term (the arm-chooser shape).  */
  let mut suppressed: bool = EQ (fs_state, Qmaximized ()) || EQ (fs_state, Qfullboth ());
  #[cfg (not (feature = "have_pgtk"))]
  {
    suppressed = suppressed
      && (x_wm_supports (f, (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_state)
	  || x_wm_supports (f, (*FRAME_DISPLAY_INFO! (f)).Xatom_net_wm_state_fullscreen));
  }
  if suppressed
    {
      /* Don't set hints when maximized or fullscreen.  Apparently KWin and
         Gtk3 don't get along and the frame shrinks (!).
      */
      return;
    }

  if flags != 0
    {
      memset (&raw mut size_hints as *mut c_void, 0, core::mem::size_of::<GdkGeometry> ());
      (*((*f).output_data.x as *mut xp_output)).size_hints = size_hints;
      (*((*f).output_data.x as *mut xp_output)).hint_flags = hint_flags as c_long;
    }
  else
    { flags = (*f).size_hint_flags as c_long; }

  size_hints = (*((*f).output_data.x as *mut xp_output)).size_hints;
  hint_flags = (*((*f).output_data.x as *mut xp_output)).hint_flags as c_int;

  hint_flags |= GDK_HINT_RESIZE_INC | GDK_HINT_MIN_SIZE;
  size_hints.width_inc = if frame_resize_pixelwise { 1 } else { FRAME_COLUMN_WIDTH (f) };
  size_hints.height_inc = if frame_resize_pixelwise { 1 } else { FRAME_LINE_HEIGHT (f) };

  hint_flags |= GDK_HINT_BASE_SIZE;
  /* Use one row/col here so base_height/width does not become zero.
     Gtk+ and/or Unity on Ubuntu 12.04 can't handle it.
     Obviously this makes the row/col value displayed off by 1.  */
  base_width = FRAME_TEXT_COLS_TO_PIXEL_WIDTH (f, 1) + FRAME_TOOLBAR_WIDTH! (f);
  base_height = FRAME_TEXT_LINES_TO_PIXEL_HEIGHT (f, 1)
    + FRAME_MENUBAR_HEIGHT! (f) + FRAME_TOOLBAR_HEIGHT! (f);

  size_hints.base_width = base_width;
  size_hints.base_height = base_height;
  size_hints.min_width  = base_width;
  size_hints.min_height = base_height;

  /* These currently have a one to one mapping with the X values, but I
     don't think we should rely on that.  */
  if ! FRAME_PARENT_FRAME (f).is_null ()
    {
      hint_flags |= GDK_HINT_WIN_GRAVITY;
      size_hints.win_gravity = 0;
      if win_gravity == NorthWestGravity
	{ size_hints.win_gravity = GDK_GRAVITY_NORTH_WEST; }
      else if win_gravity == NorthGravity
	{ size_hints.win_gravity = GDK_GRAVITY_NORTH; }
      else if win_gravity == NorthEastGravity
	{ size_hints.win_gravity = GDK_GRAVITY_NORTH_EAST; }
      else if win_gravity == WestGravity
	{ size_hints.win_gravity = GDK_GRAVITY_WEST; }
      else if win_gravity == CenterGravity
	{ size_hints.win_gravity = GDK_GRAVITY_CENTER; }
      else if win_gravity == EastGravity
	{ size_hints.win_gravity = GDK_GRAVITY_EAST; }
      else if win_gravity == SouthWestGravity
	{ size_hints.win_gravity = GDK_GRAVITY_SOUTH_WEST; }
      else if win_gravity == SouthGravity
	{ size_hints.win_gravity = GDK_GRAVITY_SOUTH; }
      else if win_gravity == SouthEastGravity
	{ size_hints.win_gravity = GDK_GRAVITY_SOUTH_EAST; }
      else if win_gravity == StaticGravity
	{ size_hints.win_gravity = GDK_GRAVITY_STATIC; }
    }
  else
    { size_hints.win_gravity = GDK_GRAVITY_NORTH_WEST; }

  if (flags & PPosition) != 0
    { hint_flags |= GDK_HINT_POS; }
  if (flags & USPosition) != 0
    { hint_flags |= GDK_HINT_USER_POS; }
  if (flags & USSize) != 0
    { hint_flags |= GDK_HINT_USER_SIZE; }

  if user_position
    {
      hint_flags &= ! GDK_HINT_POS;
      hint_flags |= GDK_HINT_USER_POS;
    }

  size_hints.base_width /= scale;
  size_hints.base_height /= scale;
  size_hints.width_inc /= scale;
  size_hints.height_inc /= scale;

  if hint_flags as c_long != (*((*f).output_data.x as *mut xp_output)).hint_flags
      || memcmp (&raw const size_hints as *const c_void,
		 &raw const (*((*f).output_data.x as *mut xp_output)).size_hints as *const c_void,
		 core::mem::size_of::<GdkGeometry> ()) != 0
    {
      block_input ();
      gtk_window_set_geometry_hints (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
                                     std::ptr::null_mut (), &raw mut size_hints, hint_flags);
      (*((*f).output_data.x as *mut xp_output)).size_hints = size_hints;
      (*((*f).output_data.x as *mut xp_output)).hint_flags = hint_flags as c_long;
      unblock_input ();
    }
}

/* Change background color of a frame.
   Since GTK uses the background color to clear the window, we must
   keep the GTK and X colors in sync.
   F is the frame to change,
   BG is the pixel value to change to.  */

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_background_color (f: *mut frame, bg: c_ulong)   // C 2117-2138
{
  if ! FRAME_GTK_WIDGET! (f).is_null ()
    {
      block_input ();
      xg_set_widget_bg (f, FRAME_GTK_WIDGET! (f), FRAME_BACKGROUND_PIXEL (f));

      #[cfg (feature = "use_toolkit_scroll_bars")]
      {
	let mut bar: Lisp_Object;
	bar = FRAME_SCROLL_BARS (f);
	while ! NILP (bar)
	  {
	    let scrollbar: *mut GtkWidget
	      = xg_get_widget_from_map ((*XSCROLL_BAR! (bar)).x_window, FRAME_X_DISPLAY! (f));
	    let webox: *mut GtkWidget = gtk_widget_get_parent (scrollbar);
	    xg_set_widget_bg (f, webox, FRAME_BACKGROUND_PIXEL (f));
	    bar = (*XSCROLL_BAR! (bar)).next;
	  }
      }
      unblock_input ();
    }
}

/* Change the frame's decoration (title bar + resize borders).  This
   might not work with all window managers.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_undecorated (f: *mut frame, undecorated: Lisp_Object)
{                                                        // C 2143-2156
  #[cfg (feature = "have_pgtk")]
  { if FRAME_GTK_OUTER_WIDGET! (f).is_null () { return; } }
  if ! FRAME_GTK_WIDGET! (f).is_null ()
    {
      block_input ();
      gtk_window_set_decorated (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
				if NILP (undecorated) { TRUE } else { FALSE });
      unblock_input ();
    }
}


/* Restack F1 below F2, above if ABOVE_FLAG is true.  This might not
   work with all window managers.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_frame_restack (f1: *mut frame, f2: *mut frame, above_flag: bool)
{                                                        // C 2162-2182
  block_input ();
  if ! FRAME_GTK_OUTER_WIDGET! (f1).is_null () && ! FRAME_GTK_OUTER_WIDGET! (f2).is_null ()
    {
      let gwin1: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f1));
      let gwin2: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f2));
      let mut frame1: Lisp_Object;
      let mut frame2: Lisp_Object;

      XSETFRAME! (frame1, f1);
      XSETFRAME! (frame2, f2);

      gdk_window_restack (gwin1, gwin2, above_flag as gboolean);
      #[cfg (not (feature = "have_pgtk"))]
      { XSync (FRAME_X_DISPLAY! (f1), False); }
      #[cfg (feature = "have_pgtk")]
      { gdk_flush (); }
    }
  unblock_input ();
}


/* Don't show frame in taskbar, don't ALT-TAB to it.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_skip_taskbar (f: *mut frame, skip_taskbar: Lisp_Object)
{                                                        // C 2187-2202
  block_input ();
  /* C 2190/2196: C's `#ifndef HAVE_PGTK` tests `FRAME_GTK_WIDGET (f)`, the `#else`
     `FRAME_GTK_OUTER_WIDGET (f)`, around one shared body (the arm-chooser shape).  */
  #[cfg (not (feature = "have_pgtk"))]
  let cond: bool = ! FRAME_GTK_WIDGET! (f).is_null ();
  #[cfg (feature = "have_pgtk")]
  let cond: bool = ! FRAME_GTK_OUTER_WIDGET! (f).is_null ();
  if cond
    {
      gdk_window_set_skip_taskbar_hint
	(gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f)),
	 if NILP (skip_taskbar) { FALSE } else { TRUE });
    }
  unblock_input ();
}


/* Don't give frame focus.  */
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_no_focus_on_map (f: *mut frame, no_focus_on_map: Lisp_Object)
{                                                        // C 2207-2222
  #[cfg (feature = "have_pgtk")]
  { if FRAME_GTK_OUTER_WIDGET! (f).is_null () { return; } }
  block_input ();
  if ! FRAME_GTK_WIDGET! (f).is_null ()
    {
      let gwin: *mut GtkWindow = GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f));
      let g_no_focus_on_map: gboolean = if NILP (no_focus_on_map) { TRUE } else { FALSE };

      gtk_window_set_focus_on_map (gwin, g_no_focus_on_map);
    }
  unblock_input ();
}


#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_no_accept_focus (f: *mut frame, no_accept_focus: Lisp_Object)
{                                                        // C 2226-2244
  let g_no_accept_focus: gboolean = if NILP (no_accept_focus) { TRUE } else { FALSE };
  #[cfg (feature = "have_pgtk")]
  {
    if FRAME_GTK_OUTER_WIDGET! (f).is_null ()
      {
	if ! FRAME_WIDGET! (f).is_null ()
	  { gtk_widget_set_can_focus (FRAME_WIDGET! (f), g_no_accept_focus); }
	return;
      }
  }
  block_input ();
  if ! FRAME_GTK_WIDGET! (f).is_null ()
    {
      let gwin: *mut GtkWindow = GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f));
      gtk_window_set_accept_focus (gwin, g_no_accept_focus);
    }
  unblock_input ();
}

#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_override_redirect (f: *mut frame,
						   override_redirect: Lisp_Object)
{                                                        // C 2247-2259
  block_input ();

  if ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
    {
      let gwin: *mut GdkWindow = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f));

      gdk_window_set_override_redirect (gwin, if NILP (override_redirect) { FALSE } else { TRUE });
    }

  unblock_input ();
}

#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
/* Set the frame icon to ICON_PIXMAP/MASK.  This must be done with GTK
   functions so GTK does not overwrite the icon.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_set_frame_icon (f: *mut frame, icon_pixmap: Pixmap,
					    icon_mask: Pixmap)   // C 2266-2277
{
  #[cfg (feature = "have_pgtk")]
  { if FRAME_GTK_OUTER_WIDGET! (f).is_null () { return; } }
  let gp: *mut GdkPixbuf = xg_get_pixbuf_from_pix_and_mask (f, icon_pixmap, icon_mask);
  if ! gp.is_null ()
    { gtk_window_set_icon (GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)), gp); }
}

/* C 2282-2284's "Dialog functions" banner, carried as C's own text at its position.  */
/* C: /***********************************************************************
   C:                       Dialog functions
   C:  ***********************************************************************/  */

/* C 2285-2286's comment above `get_dialog_title`, carried as C's own text.  */
/* C: /* Return the dialog title to use for a dialog of type KEY.
   C:    This is the encoding used by lwlib.  We use the same for GTK.  */  */

/// C 2288-2316: `static const char *get_dialog_title (char key)` - C's `switch` over the ten
/// letters becomes a `match` whose fall-through pairs are `|` alternatives (PORTING.md 2.2), so
/// the `switch`/`case`/`break` tokens have no Rust counterpart
/// (`docs/ports/macro-body-notes.tsv`'s row); C's `title` initialiser, the five literals and the
/// final `return` are unchanged.
/// C: static const char *
/// C: get_dialog_title (char key)
#[cfg (feature = "use_gtk")]
unsafe fn get_dialog_title (key: c_char) -> *const c_char
{
  let mut title: *const c_char = c"".as_ptr ();                        // C 2291

  match key as u8
    {
    b'E' | b'e' =>                                                    // C 2294
      { title = c"Error".as_ptr (); }                                 // C 2295

    b'I' | b'i' =>                                                    // C 2298
      { title = c"Information".as_ptr (); }                           // C 2299

    b'L' | b'l' =>                                                    // C 2302
      { title = c"Prompt".as_ptr (); }                                // C 2303

    b'P' | b'p' =>                                                    // C 2306
      { title = c"Prompt".as_ptr (); }                                // C 2307

    b'Q' | b'q' =>                                                    // C 2310
      { title = c"Question".as_ptr (); }                              // C 2311

    _ => {}
    }

  title                                                               // C 2315
}

/* C 2318-2327's comment above `dialog_delete_callback`, carried as C's own text.  */
/* C: /* Callback for dialogs that get WM_DELETE_WINDOW.  We pop down
   C:    the dialog, but return TRUE so the event does not propagate further
   C:    in GTK.  This prevents GTK from destroying the dialog widget automatically
   C:    and we can always destroy the widget manually, regardless of how
   C:    it was popped down (button press or WM_DELETE_WINDOW).
   C:    W is the dialog widget.
   C:    EVENT is the GdkEvent that represents WM_DELETE_WINDOW (not used).
   C:    user_data is NULL (not used).

   C:    Returns TRUE to end propagation of event.  */  */

/// C 2329-2334: `static gboolean dialog_delete_callback (GtkWidget *w, GdkEvent *event,
/// gpointer user_data)` - `static` in C, but C 2398 takes its address for `g_signal_connect`
/// (through `G_CALLBACK`), so the port defines it `unsafe extern "C" fn` (the `delete_cb` shape
/// above).
/// C: static gboolean
/// C: dialog_delete_callback (GtkWidget *w, GdkEvent *event, gpointer user_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn dialog_delete_callback (w: *mut GtkWidget, event: *mut GdkEvent,
					     user_data: gpointer) -> gboolean
{
  gtk_widget_unmap (w);                                                 // C 2332
  TRUE                                                                  // C 2333
}

/* C 2336-2341's comment above `create_dialog`, carried as C's own text.  */
/* C: /* Create a popup dialog window.  See also xg_create_widget below.
   C:    WV is a widget_value describing the dialog.
   C:    SELECT_CB is the callback to use when a button has been pressed.
   C:    DEACTIVATE_CB is the callback to use when the dialog pops down.

   C:    Returns the GTK dialog widget.  */  */

/// C 2343-2457: `static GtkWidget *create_dialog (widget_value *wv, GCallback select_cb,
/// GCallback deactivate_cb)` - C 2363-2372's `#if GTK_CHECK_VERSION (3, 12, 0)` pair is the
/// `gtk_check_version_ge_3_12` feature (the two arms assign the same `cur_box`), and C 2419-2424's
/// `#if GTK_CHECK_VERSION (3, 14, 0)` pair is `gtk_check_version_ge_3_14`.  C 2361's `bool` local,
/// C 2430's `strlen` conversion and C 2445's `++button_nr` are the port's `bool`, `as c_int` and
/// two statements.
/// C: static GtkWidget *
/// C: create_dialog (widget_value *wv,
/// C:                GCallback select_cb,
/// C:                GCallback deactivate_cb)
#[cfg (feature = "use_gtk")]
unsafe fn create_dialog (wv: *mut widget_value, select_cb: GCallback,
			 deactivate_cb: GCallback) -> *mut GtkWidget
{
  let title: *const c_char = get_dialog_title (*(*wv).name as c_char);  // C 2348
  let mut total_buttons: c_int = *(*wv).name.offset (1) as c_int - b'0' as c_int;   // C 2349
  let mut right_buttons: c_int = *(*wv).name.offset (4) as c_int - b'0' as c_int;   // C 2350
  let left_buttons: c_int;                                             // C 2351
  let mut button_nr: c_int = 0;                                        // C 2352
  let mut button_spacing: c_int = 10;                                  // C 2353
  let wdialog: *mut GtkWidget = gtk_dialog_new ();                     // C 2354
  let wd: *mut GtkDialog = GTK_DIALOG! (wdialog);                      // C 2355
  let mut item: *mut widget_value;                                     // C 2356
  /* C leaves `whbox_down` uninitialised in its `else` arm (PORTING.md 1.5); C 2448 reads it only
     under `make_two_rows`, which is exactly the arm that stores it.  */
  let mut whbox_down: *mut GtkWidget = std::ptr::null_mut ();          // C 2357

  /* C 2359-2360's note, carried as C's own text at its position.  */
  /* C: /* If the number of buttons is greater than 4, make two rows of buttons
     C:    instead.  This looks better.  */  */
  let make_two_rows: bool = total_buttons > 4;                         // C 2361

  /* C 2363's `#if GTK_CHECK_VERSION (3, 12, 0)` - the `gtk_check_version_ge_3_12` feature.  */
  let mut cur_box: *mut GtkBox;                                        // C 2368/2371
  #[cfg (feature = "gtk_check_version_ge_3_12")]
  {
    let gbld: *mut GtkBuilder = gtk_builder_new ();                    // C 2364
    let go: *mut GObject
      = gtk_buildable_get_internal_child (GTK_BUILDABLE! (wd), gbld,
					  c"action_area".as_ptr ());       // C 2365-2367
    cur_box = GTK_BOX! (go);                                           // C 2368
    g_object_unref (G_OBJECT! (gbld));                                  // C 2369
  }
  #[cfg (not (feature = "gtk_check_version_ge_3_12"))]
  { cur_box = GTK_BOX! (gtk_dialog_get_action_area (wd)); }             // C 2371

  if right_buttons == 0 { right_buttons = total_buttons / 2; }          // C 2374
  left_buttons = total_buttons - right_buttons;                         // C 2375

  gtk_window_set_title (GTK_WINDOW! (wdialog), title);                  // C 2377
  gtk_widget_set_name (wdialog, c"emacs-dialog".as_ptr ());             // C 2378


  if make_two_rows                                                      // C 2381
    {
      let wvbox: *mut GtkWidget = gtk_box_new! (GTK_ORIENTATION_VERTICAL, button_spacing);   // C 2383
      let whbox_up: *mut GtkWidget = gtk_box_new! (GTK_ORIENTATION_HORIZONTAL, 0);   // C 2384
      gtk_box_set_homogeneous (GTK_BOX! (wvbox), TRUE);                 // C 2385
      gtk_box_set_homogeneous (GTK_BOX! (whbox_up), FALSE);             // C 2386
      whbox_down = gtk_box_new! (GTK_ORIENTATION_HORIZONTAL, 0);        // C 2387
      gtk_box_set_homogeneous (GTK_BOX! (whbox_down), FALSE);           // C 2388

      gtk_box_pack_start (cur_box, wvbox, FALSE, FALSE, 0);             // C 2390
      gtk_box_pack_start (GTK_BOX! (wvbox), whbox_up, FALSE, FALSE, 0); // C 2391
      gtk_box_pack_start (GTK_BOX! (wvbox), whbox_down, FALSE, FALSE, 0);   // C 2392

      cur_box = GTK_BOX! (whbox_up);                                    // C 2394
    }

  g_signal_connect! (G_OBJECT! (wdialog), c"delete-event".as_ptr (),    // C 2397
		    dialog_delete_callback, std::ptr::null_mut ());     // C 2398

  if deactivate_cb.is_some ()                                           // C 2400
    {
      /* C 2402/2403 pass the *variable* `deactivate_cb`, which the port's `g_signal_connect!`
	 macro would wrap a second time in `G_CALLBACK!` - and that macro needs a function item,
	 not a `GCallback` value (measured: `E0605`, this slice's first arm run).  The two calls
	 therefore stand in the macro's own expansion, handler and all.  */
      g_signal_connect_data (G_OBJECT! (wdialog) as gpointer, c"close".as_ptr (),
			     deactivate_cb, std::ptr::null_mut (), None,
			     0 as GConnectFlags);                          // C 2402
      g_signal_connect_data (G_OBJECT! (wdialog) as gpointer, c"response".as_ptr (),
			     deactivate_cb, std::ptr::null_mut (), None,
			     0 as GConnectFlags);                          // C 2403
    }

  item = (*wv).contents;                                                // C 2406
  while !item.is_null ()
    {
      let utf8_label: *mut c_char = get_utf8_string ((*item).value);    // C 2408
      let mut w: *mut GtkWidget;                                        // C 2409
      let mut req: GtkRequisition = GtkRequisition { width: 0, height: 0 };   // C 2410

      if !(*item).name.is_null () && strcmp ((*item).name, c"message".as_ptr ()) == 0   // C 2412
	{
	  let wvbox: *mut GtkBox = GTK_BOX! (gtk_dialog_get_content_area (wd));   // C 2414
	  /* This is the text part of the dialog.  */                       // C 2415
	  w = gtk_label_new (utf8_label);                                   // C 2416
	  gtk_box_pack_start (wvbox, gtk_label_new (c"".as_ptr ()), FALSE, FALSE, 0);   // C 2417
	  gtk_box_pack_start (wvbox, w, TRUE, TRUE, 0);                     // C 2418
	  /* C 2419's `#if GTK_CHECK_VERSION (3, 14, 0)` - the `gtk_check_version_ge_3_14` feature.  */
	  #[cfg (feature = "gtk_check_version_ge_3_14")]
	  {
	    gtk_widget_set_halign (w, GTK_ALIGN_START);                     // C 2420
	    gtk_widget_set_valign (w, GTK_ALIGN_CENTER);                    // C 2421
	  }
	  #[cfg (not (feature = "gtk_check_version_ge_3_14"))]
	  { gtk_misc_set_alignment (GTK_MISC! (w), 0.1, 0.5); }             // C 2423
	  /* C 2425-2426's note, carried as C's own text at its position.  */
	  /* C: /* Try to make dialog look better.  Must realize first so
	     C:    the widget can calculate the size it needs.  */  */
	  gtk_widget_realize (w);                                           // C 2427
	  gtk_widget_get_preferred_size (w, std::ptr::null_mut (), &raw mut req);   // C 2428
	  gtk_box_set_spacing (wvbox, req.height);                          // C 2429
	  if !(*item).value.is_null () && strlen ((*item).value) > 0        // C 2430
	    { button_spacing = 2 * req.width / strlen ((*item).value) as c_int; }   // C 2431
	  if button_spacing < 10 { button_spacing = 10; }                    // C 2432
	}
      else
	{
	  /* This is one button to add to the dialog.  */                   // C 2436
	  w = gtk_button_new_with_label (utf8_label);                       // C 2437
	  if !(*item).enabled                                               // C 2438
	    { gtk_widget_set_sensitive (w, FALSE); }                        // C 2439
	  if select_cb.is_some ()                                           // C 2440
	    {
	      /* C 2441-2442's `select_cb` is the variable, so this call takes the macro's own
		 expansion as above.  */
	      g_signal_connect_data (G_OBJECT! (w) as gpointer, c"clicked".as_ptr (),
				     select_cb, (*item).call_data, None,
				     0 as GConnectFlags);                   // C 2441-2442
	    }

	  gtk_box_pack_start (cur_box, w, TRUE, TRUE, button_spacing as guint);   // C 2444
	  button_nr += 1;                                                    // C 2445
	  if button_nr == left_buttons
	    {
	      if make_two_rows
		{ cur_box = GTK_BOX! (whbox_down); }                         // C 2448
	    }
	}

     if !utf8_label.is_null ()                                           // C 2452
       { g_free (utf8_label as *mut c_void); }                           // C 2453
     item = (*item).next;                                                // C 2406 (the loop's step)
    }

  wdialog                                                               // C 2456
}

/* C 2459-2465: `struct xg_dialog_data` - the record the three callbacks below share.  C's `loop`
   member is the Rust keyword, so the port spells it `r#loop` (C's own name; the
   `lib/include/sfnt.rs` 1994 device).  */
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct xg_dialog_data
{
  pub r#loop: *mut GMainLoop,             // C 2461
  pub response: c_int,                    // C 2462
  pub w: *mut GtkWidget,                  // C 2463
  pub timerid: guint,                     // C 2464
}

/* C 2467-2469's comment above `xg_dialog_response_cb`, carried as C's own text.  */
/* C: /* Function that is called when the file or font dialogs pop down.
   C:    W is the dialog widget, RESPONSE is the response code.
   C:    USER_DATA is what we passed in to g_signal_connect.  */  */

/// C 2471-2479: `static void xg_dialog_response_cb (GtkDialog *w, gint response,
/// gpointer user_data)` - `static` in C and C 2545 takes its address through `G_CALLBACK`, so
/// the port defines it `unsafe extern "C" fn`; C 2476's `void *` to `struct xg_dialog_data *`
/// conversion is the plain cast.
/// C: static void
/// C: xg_dialog_response_cb (GtkDialog *w, gint response, gpointer user_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn xg_dialog_response_cb (w: *mut GtkDialog, response: gint,
					    user_data: gpointer)
{
  let dd: *mut xg_dialog_data = user_data as *mut xg_dialog_data;       // C 2476
  (*dd).response = response;                                            // C 2477
  g_main_loop_quit ((*dd).r#loop);                                       // C 2478
}


/* C 2482's comment above `pop_down_dialog`, carried as C's own text.  */
/* C: /*  Destroy the dialog.  This makes it pop down.  */  */

/// C 2484-2497: `static void pop_down_dialog (void *arg)` - the unwind function C 2551 registers
/// with `record_unwind_protect_ptr`, so the port defines it `unsafe extern "C" fn` with the
/// mirror's `Option<unsafe extern "C" fn (*mut c_void)>` signature; C 2490/2491's two truth tests
/// are the port's `! … is_null ()` and `!= 0`.
/// C: static void
/// C: pop_down_dialog (void *arg)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn pop_down_dialog (arg: *mut c_void)
{
  let dd: *mut xg_dialog_data = arg as *mut xg_dialog_data;             // C 2487

  block_input ();                                                       // C 2489
  if !(*dd).w.is_null () { gtk_widget_destroy ((*dd).w); }              // C 2490
  if (*dd).timerid != 0 { g_source_remove ((*dd).timerid); }            // C 2491

  g_main_loop_quit ((*dd).r#loop);                                       // C 2493
  g_main_loop_unref ((*dd).r#loop);                                      // C 2494

  unblock_input ();                                                     // C 2496
}

/* C 2499-2500's comment above `xg_maybe_add_timer`, carried as C's own text.  */
/* C: /* If there are any emacs timers pending, add a timeout to main loop in DATA.
   C:    Pass DATA as gpointer so we can use this as a callback.  */  */

/* Use-site declarations (P-88 (1)): the one name this slice calls and no mirror in the unit's
   include graph carries.  C: keyboard.h 521: `extern struct timespec timer_check (void);` - the
   `src/haikumenu.rs` 116 spelling.  */
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C: keyboard.h 521: `extern struct timespec timer_check (void);` - C 2506's call.
  pub fn timer_check () -> timespec;
}

/// C 2502-2519: `static gboolean xg_maybe_add_timer (gpointer data)` - `static` in C, and its
/// address is C 2516's `g_timeout_add` argument, so an `unsafe extern "C" fn`; C 2513's
/// `TIMESPEC_HZ / 1000` is `lib/include/timespec.rs` 34's constant, C 2514's `int ms` and
/// C 2515's `(guint) -1` are the port's `c_int` and `-1 as guint`.
/// C: static gboolean
/// C: xg_maybe_add_timer (gpointer data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn xg_maybe_add_timer (data: gpointer) -> gboolean
{
  let dd: *mut xg_dialog_data = data as *mut xg_dialog_data;            // C 2505
  let next_time: timespec = timer_check ();                             // C 2506

  (*dd).timerid = 0;                                                    // C 2508

  if timespec_valid_p (next_time)                                       // C 2510
    {
      let s: time_t = next_time.tv_sec;                                 // C 2512
      let per_ms: c_int = TIMESPEC_HZ / 1000;                           // C 2513
      let ms: c_int = ((next_time.tv_nsec + per_ms as c_long - 1)
		       / per_ms as c_long) as c_int;                    // C 2514
      /* C 2515's `((guint) -1 - ms) / 1000`: the `(guint) -1` cast is the port's `! 0 as guint`
	 (the `lib/include/w32.c` device for a negative `DWORD`), and the whole expression is
	 converted to `time_t` for the comparison with `s`, exactly as C's arithmetic conversions
	 do (the RHS is a `guint` and `s` is a `time_t`).  */
      if s <= (((! 0 as guint) - ms as guint) / 1000) as c_long         // C 2515
	{
	  (*dd).timerid
	    = g_timeout_add ((s * 1000 + ms as c_long) as guint,
			     Some (xg_maybe_add_timer), dd as gpointer);   // C 2516
	}
    }
  FALSE                                                                 // C 2518
}


/* C 2522-2524's comment above `xg_dialog_run`, carried as C's own text.  */
/* C: /* Pops up a modal dialog W and waits for response.
   C:    We don't use gtk_dialog_run because we want to process emacs timers.
   C:    The dialog W is not destroyed when this function returns.  */  */

/// C 2526-2560: `static int xg_dialog_run (struct frame *f, GtkWidget *w)` - `static` in C with
/// one call site, so a plain `unsafe fn`; C 2529's `SPECPDL_INDEX` and C 2551's
/// `record_unwind_protect_ptr` are `include/lisp.rs` 2624/4094's rows, C 2553's `(void)` cast is
/// the port's bare call, and C 2556/2539's `0`/`GTK_RESPONSE_CANCEL` are the null pointer and the
/// mirror's placeholder constant.
/// C: static int
/// C: xg_dialog_run (struct frame *f, GtkWidget *w)
#[cfg (feature = "use_gtk")]
unsafe fn xg_dialog_run (f: *mut frame, w: *mut GtkWidget) -> c_int
{
  let count: specpdl_ref = SPECPDL_INDEX ();                            // C 2529
  /* C leaves the record uninitialised and fills its four members at C 2538-2541; Rust's
     definite-initialisation analysis rejects field-by-field assignment, so the port zeroes it
     first (PORTING.md 1.5) and then writes the same four members.  */
  let mut dd: xg_dialog_data = core::mem::zeroed ();                    // C 2530

  xg_set_screen (w, f);                                                 // C 2532
  gtk_window_set_transient_for (GTK_WINDOW! (w),                        // C 2533
				GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)));   // C 2534
  gtk_window_set_destroy_with_parent (GTK_WINDOW! (w), TRUE);           // C 2535
  gtk_window_set_modal (GTK_WINDOW! (w), TRUE);                         // C 2536

  dd.r#loop = g_main_loop_new (std::ptr::null_mut (), FALSE);            // C 2538
  dd.response = GTK_RESPONSE_CANCEL;                                    // C 2539
  dd.w = w;                                                             // C 2540
  dd.timerid = 0;                                                       // C 2541

  g_signal_connect! (G_OBJECT! (w),                                     // C 2543
		     c"response".as_ptr (),                             // C 2544
		     xg_dialog_response_cb,                             // C 2545
		     (&raw mut dd) as gpointer);                         // C 2546
  /* Don't destroy the widget if closed by the window manager close button.  */   // C 2547
  g_signal_connect! (G_OBJECT! (w), c"delete-event".as_ptr (),
		     gtk_true, std::ptr::null_mut ());                  // C 2548
  gtk_widget_show (w);                                                  // C 2549

  record_unwind_protect_ptr (Some (pop_down_dialog), (&raw mut dd) as *mut c_void);   // C 2551

  xg_maybe_add_timer ((&raw mut dd) as gpointer);                       // C 2553 (C's `(void)` cast)
  g_main_loop_run (dd.r#loop);                                          // C 2554

  dd.w = std::ptr::null_mut ();                                         // C 2556
  unbind_to (count, Qnil ());                                           // C 2557

  dd.response                                                           // C 2559
}

/* C 2562's form feed and C 2563-2565's "File dialog functions" banner, carried as C's own text.  */
/* C: /***********************************************************************
   C:                       File dialog functions
   C:  ***********************************************************************/  */

/* C 2566's comment above `xg_uses_old_file_dialog`, carried as C's own text.  */
/* C: /* Return true if the old file selection dialog is being used.  */  */

/// C 2568-2576: `bool xg_uses_old_file_dialog (void)` - non-`static`, so `#[unsafe(no_mangle)]`
/// with the C ABI; C 2571-2575's `#ifdef HAVE_GTK_FILE_SELECTION_NEW` pair is the
/// `have_gtk_file_selection_new` feature (registered by this slice), and C's `x_gtk_…` object is
/// the Lisp `bool` `include/globals.rs` 7193 carries.
/// C: bool
/// C: xg_uses_old_file_dialog (void)
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_uses_old_file_dialog () -> bool
{
  /* C 2571's `#ifdef HAVE_GTK_FILE_SELECTION_NEW`.  */
  #[cfg (feature = "have_gtk_file_selection_new")]
  { return x_gtk_use_old_file_dialog; }                                // C 2572
  #[cfg (not (feature = "have_gtk_file_selection_new"))]
  { return false; }                                                    // C 2574
}


/// C 2579: `typedef char * (*xg_get_file_func) (GtkWidget *);` - the type the three
/// `*func = …` stores below and C 2838's `(*func) (w)` call use.  The mirror's callback shape
/// (`Option<unsafe extern "C" fn …>`, PORTING.md 1.4) is what the port's `xg_get_file_func` is.
#[cfg (feature = "use_gtk")]
pub type xg_get_file_func = Option<unsafe extern "C" fn (w: *mut GtkWidget) -> *mut c_char>;

/* C 2581-2582's comment above `xg_get_file_name_from_chooser`, carried as C's own text.  */
/* C: /* Return the selected file for file chooser dialog W.
   C:    The returned string must be free:d.  */  */

/// C 2584-2588: `static char *xg_get_file_name_from_chooser (GtkWidget *w)` - `static` in C, and
/// C 2735 stores its address in the caller's `func`, so `unsafe extern "C" fn`.
/// C: static char *
/// C: xg_get_file_name_from_chooser (GtkWidget *w)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn xg_get_file_name_from_chooser (w: *mut GtkWidget) -> *mut c_char
{
  gtk_file_chooser_get_filename (GTK_FILE_CHOOSER! (w))                // C 2587
}

/* C 2590-2591's comment above `xg_toggle_visibility_cb`, carried as C's own text.  */
/* C: /* Callback called when the "Show hidden files" toggle is pressed.
   C:    WIDGET is the toggle widget, DATA is the file chooser dialog.  */  */

/// C 2593-2600: `static void xg_toggle_visibility_cb (GtkWidget *widget, gpointer data)` - a
/// callback C 2681's `g_signal_connect` takes the address of (through `G_CALLBACK`), so
/// `unsafe extern "C" fn`; C's `&visible` is the port's `&raw mut visible` and C 2599's `!visible`
/// is the port's `(visible == 0) as gboolean` (C's `!` on an int is 0/1).
/// C: static void
/// C: xg_toggle_visibility_cb (GtkWidget *widget, gpointer data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn xg_toggle_visibility_cb (widget: *mut GtkWidget, data: gpointer)
{
  let dialog: *mut GtkFileChooser = GTK_FILE_CHOOSER! (data);          // C 2596
  let mut visible: gboolean = 0;                                       // C 2597
  g_object_get (G_OBJECT! (dialog) as gpointer, c"show-hidden".as_ptr (),
		&raw mut visible, std::ptr::null_mut::<c_void> ());    // C 2598
  g_object_set (G_OBJECT! (dialog) as gpointer, c"show-hidden".as_ptr (),
		(visible == 0) as gboolean,
		std::ptr::null_mut::<c_void> ());                      // C 2599
}


/* C 2603-2607's comment above `xg_toggle_notify_cb`, carried as C's own text.  */
/* C: /* Callback called when a property changes in a file chooser.
   C:    GOBJECT is the file chooser dialog, ARG1 describes the property.
   C:    USER_DATA is the toggle widget in the file chooser dialog.
   C:    We use this to update the "Show hidden files" toggle when the user
   C:    changes that property by right clicking in the file list.  */  */

/// C 2609-2629: `static void xg_toggle_notify_cb (GObject *gobject, GParamSpec *arg1,
/// gpointer user_data)` - C 2683 takes its address, so `unsafe extern "C" fn`; C 2620's
/// `!!visible != !!toggle_on` is the port's two `!= 0` tests, and C 2622's `(gpointer) G_CALLBACK
/// (…)` cast is the `GCallback`-to-`gpointer` transmute this file already uses (C 2344's device).
/// C: static void
/// C: xg_toggle_notify_cb (GObject *gobject, GParamSpec *arg1, gpointer user_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn xg_toggle_notify_cb (gobject: *mut GObject, arg1: *mut GParamSpec,
					 user_data: gpointer)
{
  if strcmp ((*arg1).name, c"show-hidden".as_ptr ()) == 0             // C 2612
    {
      let wtoggle: *mut GtkWidget = GTK_WIDGET! (user_data);           // C 2614
      let mut visible: gboolean = 0;                                   // C 2615
      let mut toggle_on: gboolean = 0;

      g_object_get (G_OBJECT! (gobject) as gpointer, c"show-hidden".as_ptr (),
		    &raw mut visible, std::ptr::null_mut::<c_void> ());   // C 2617
      toggle_on = gtk_toggle_button_get_active (GTK_TOGGLE_BUTTON! (wtoggle));   // C 2618

      if (visible != 0) != (toggle_on != 0)                            // C 2620
	{
	  let cb: gpointer
	    = core::mem::transmute::<GCallback, gpointer> (
		G_CALLBACK! (xg_toggle_visibility_cb));                // C 2622
	  /* C 2623/2625 pass C's `gpointer cb` where the prototype takes a `GCallback`; the port
	     transmutes the stored pointer back (C's own implicit conversion).  */
	  g_signal_handlers_block_by_func (G_OBJECT! (wtoggle) as gpointer,
					   core::mem::transmute::<gpointer, GCallback> (cb),
					   gobject as gpointer);        // C 2623
	  gtk_toggle_button_set_active (GTK_TOGGLE_BUTTON! (wtoggle), visible);   // C 2624
	  g_signal_handlers_unblock_by_func (G_OBJECT! (wtoggle) as gpointer,
					     core::mem::transmute::<gpointer, GCallback> (cb),
					     gobject as gpointer);      // C 2625
	}
      x_gtk_show_hidden_files = visible != 0;                          // C 2627
    }
}

/* C 2631-2640's comment above `xg_get_file_with_chooser`, carried as C's own text.  */
/* C: /* Read a file name from the user using a file chooser dialog.
   C:    F is the current frame.
   C:    PROMPT is a prompt to show to the user.  May not be NULL.
   C:    DEFAULT_FILENAME is a default selection to be displayed.  May be NULL.
   C:    If MUSTMATCH_P, the returned file name must be an existing
   C:    file.  (Actually, this only has cosmetic effects, the user can
   C:    still enter a non-existing file.)  *FUNC is set to a function that
   C:    can be used to retrieve the selected file name from the returned widget.

   C:    Returns the created widget.  */  */

/// C 2642-2737: `static GtkWidget *xg_get_file_with_chooser (struct frame *f, char *prompt,
/// char *default_filename, bool mustmatch_p, bool only_dir_p, xg_get_file_func *func)` -
/// `static` in C with one call site, so a plain `unsafe fn`; C 2649's 1024-byte `msgbuf` is the
/// port's array, C 2652's `UNINIT` member is zeroed instead (PORTING.md 1.5), C 2654-2656 and
/// C 2658's `action` is the mirror's enumeration, C 2687's `char *z = msgbuf` is the port's
/// pointer into the array, and C 2735's `*func = …` stores the item through the caller's pointer.
/// C: static GtkWidget *
/// C: xg_get_file_with_chooser (struct frame *f,
/// C: 			  char *prompt,
/// C: 			  char *default_filename,
/// C: 			  bool mustmatch_p, bool only_dir_p,
/// C: 			  xg_get_file_func *func)
#[cfg (feature = "use_gtk")]
unsafe fn xg_get_file_with_chooser (f: *mut frame, prompt: *mut c_char,
				    default_filename: *mut c_char,
				    mustmatch_p: bool, only_dir_p: bool,
				    func: *mut xg_get_file_func) -> *mut GtkWidget
{
  let mut msgbuf: [c_char; 1024] = [0; 1024];                           // C 2649

  let filewin: *mut GtkWidget;                                         // C 2651
  let wtoggle: *mut GtkWidget;
  let wbox: *mut GtkWidget;
  /* C leaves `wmessage` UNINIT (PORTING.md 1.5); C 2696 stores it under C 2685's guard, which is
     exactly C 2701's guard, so the null is never read.  */
  let mut wmessage: *mut GtkWidget = std::ptr::null_mut ();             // C 2652
  let gwin: *mut GtkWindow = GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f));   // C 2653
  let mut action: GtkFileChooserAction = if mustmatch_p                 // C 2654
    { GTK_FILE_CHOOSER_ACTION_OPEN } else { GTK_FILE_CHOOSER_ACTION_SAVE };   // C 2655-2656

  if only_dir_p                                                         // C 2658
    { action = GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER; }                 // C 2659

  filewin = gtk_file_chooser_dialog_new (prompt, gwin, action,          // C 2661-2662
					 XG_TEXT_CANCEL! ().as_ptr (), GTK_RESPONSE_CANCEL,
					 (if mustmatch_p || only_dir_p   // C 2663
					  { XG_TEXT_OPEN! ().as_ptr () }
					  else { XG_TEXT_OK! ().as_ptr () }),   // C 2664
					 GTK_RESPONSE_OK,
					 std::ptr::null_mut::<c_void> ());   // C 2666
  gtk_file_chooser_set_local_only (GTK_FILE_CHOOSER! (filewin), TRUE);  // C 2667

  wbox = gtk_box_new! (GTK_ORIENTATION_VERTICAL, 0);                    // C 2669
  gtk_box_set_homogeneous (GTK_BOX! (wbox), FALSE);                     // C 2670
  gtk_widget_show (wbox);                                               // C 2671
  wtoggle = gtk_check_button_new_with_label (c"Show hidden files.".as_ptr ());   // C 2672

  g_object_set (G_OBJECT! (filewin) as gpointer, c"show-hidden".as_ptr (),   // C 2674
		x_gtk_show_hidden_files as gboolean,
		std::ptr::null_mut::<c_void> ());
  gtk_toggle_button_set_active (GTK_TOGGLE_BUTTON! (wtoggle),           // C 2676
				x_gtk_show_hidden_files as gboolean);   // C 2677

  gtk_widget_show (wtoggle);                                            // C 2679
  g_signal_connect! (G_OBJECT! (wtoggle), c"clicked".as_ptr (),         // C 2680
		     xg_toggle_visibility_cb, filewin as gpointer);     // C 2681
  g_signal_connect! (G_OBJECT! (filewin), c"notify".as_ptr (),          // C 2682
		     xg_toggle_notify_cb, wtoggle as gpointer);         // C 2683

  if x_gtk_file_dialog_help_text                                        // C 2685
    {
      let mut z: *mut c_char = msgbuf.as_mut_ptr ();                     // C 2687
      /* C 2688-2689's note, carried as C's own text at its position.  */
      /* C: /* Gtk+ 2.10 has the file name text entry box integrated in the dialog.
	 C:    Show the C-l help text only for versions < 2.10.  */  */
      if gtk_check_version (2, 10, 0) != 0 && action != GTK_FILE_CHOOSER_ACTION_SAVE   // C 2690
	{ z = stpcpy (z, c"\nType C-l to display a file name text entry box.\n".as_ptr ()); }   // C 2691
      /* C 2692-2694's three literals are one string in C; the port writes the concatenation
	 as one `c"…"` (the port has no adjacent-literal concatenation).  */
      strcpy (z, c"\nIf you don't like this file selector, use the corresponding\nkey binding or customize use-file-dialog to turn it off.".as_ptr ());   // C 2692-2694

      wmessage = gtk_label_new (msgbuf.as_ptr ());                       // C 2696
      gtk_widget_show (wmessage);                                        // C 2697
    }

  gtk_box_pack_start (GTK_BOX! (wbox), wtoggle, FALSE, FALSE, 0);        // C 2700
  if x_gtk_file_dialog_help_text                                         // C 2701
    { gtk_box_pack_start (GTK_BOX! (wbox), wmessage, FALSE, FALSE, 0); }   // C 2702
  gtk_file_chooser_set_extra_widget (GTK_FILE_CHOOSER! (filewin), wbox);   // C 2703

  if !default_filename.is_null ()                                        // C 2705
    {
      let mut file: Lisp_Object;                                         // C 2707
      let mut utf8_filename: *mut c_char;                                // C 2708

      file = build_string (default_filename);                            // C 2710

      /* C 2712-2713's note, carried as C's own text at its position.  */
      /* C: /* File chooser does not understand ~/... in the file name.  It must be
	 C:    an absolute name starting with /.  */  */
      if *default_filename != b'/' as c_char                             // C 2714
	{ file = Fexpand_file_name (file, Qnil ()); }                    // C 2715

      utf8_filename = SSDATA (ENCODE_UTF_8 (file));                      // C 2717
      if !NILP (Ffile_directory_p (file))                                // C 2718
	{
	  gtk_file_chooser_set_current_folder (GTK_FILE_CHOOSER! (filewin),   // C 2719
					       utf8_filename);           // C 2720
	}
      else
	{
	  gtk_file_chooser_set_filename (GTK_FILE_CHOOSER! (filewin),      // C 2723
					 utf8_filename);                 // C 2724
	  if action == GTK_FILE_CHOOSER_ACTION_SAVE                        // C 2725
	    {
	      let mut cp: *mut c_char = strrchr (utf8_filename, b'/' as c_int);   // C 2727
	      if !cp.is_null () { cp = cp.add (1); }                         // C 2728
	      else { cp = utf8_filename; }                                   // C 2729
	      gtk_file_chooser_set_current_name (GTK_FILE_CHOOSER! (filewin), cp);   // C 2730
	    }
	}
    }

  *func = Some (xg_get_file_name_from_chooser);                          // C 2735
  filewin                                                               // C 2736
}

/* C 2739's `#ifdef HAVE_GTK_FILE_SELECTION_NEW` - the `have_gtk_file_selection_new` feature (the
   same one C 2571's `xg_uses_old_file_dialog` pair carries).  */
/* C 2741-2742's comment above `xg_get_file_name_from_selector`, carried as C's own text.  */
/* C: /* Return the selected file for file selector dialog W.
   C:    The returned string must be free:d.  */  */

/// C 2744-2749: `static char *xg_get_file_name_from_selector (GtkWidget *w)` - `static` in C, and
/// C 2784 stores its address, so `unsafe extern "C" fn`; C 2748's `xstrdup` is `include/lisp.rs`
/// 4226's row.
/// C: static char *
/// C: xg_get_file_name_from_selector (GtkWidget *w)
#[cfg (all (feature = "use_gtk", feature = "have_gtk_file_selection_new"))]
unsafe extern "C" fn xg_get_file_name_from_selector (w: *mut GtkWidget) -> *mut c_char
{
  let filesel: *mut GtkFileSelection = GTK_FILE_SELECTION! (w);         // C 2747
  xstrdup (gtk_file_selection_get_filename (filesel))                   // C 2748
}

/* C 2751-2759's comment above `xg_get_file_with_selection`, carried as C's own text.  */
/* C: /* Create a file selection dialog.
   C:    F is the current frame.
   C:    PROMPT is a prompt to show to the user.  May not be NULL.
   C:    DEFAULT_FILENAME is a default selection to be displayed.  May be NULL.
   C:    If MUSTMATCH_P, the returned file name must be an existing
   C:    file.  *FUNC is set to a function that can be used to retrieve the
   C:    selected file name from the returned widget.

   C:    Returns the created widget.  */  */

/// C 2761-2787: `static GtkWidget *xg_get_file_with_selection (struct frame *f, char *prompt,
/// char *default_filename, bool mustmatch_p, bool only_dir_p, xg_get_file_func *func)` -
/// `static` in C with one call site (C 2823), so a plain `unsafe fn`; C 2775's single-statement
/// `if` is the port's braced one, C 2780's `filesel->selection_entry` is the member the mirror
/// names (its offset is UNVERIFIED, `## Owed` 6), and C 2784's `*func = …` stores the item.
/// C: static GtkWidget *
/// C: xg_get_file_with_selection (struct frame *f,
/// C:                             char *prompt,
/// C:                             char *default_filename,
/// C:                             bool mustmatch_p, bool only_dir_p,
/// C:                             xg_get_file_func *func)
#[cfg (all (feature = "use_gtk", feature = "have_gtk_file_selection_new"))]
unsafe fn xg_get_file_with_selection (f: *mut frame, prompt: *mut c_char,
				      default_filename: *mut c_char,
				      mustmatch_p: bool, only_dir_p: bool,
				      func: *mut xg_get_file_func) -> *mut GtkWidget
{
  let filewin: *mut GtkWidget;                                          // C 2768
  let filesel: *mut GtkFileSelection;                                   // C 2769

  filewin = gtk_file_selection_new (prompt);                            // C 2771
  filesel = GTK_FILE_SELECTION! (filewin);                              // C 2772

  if !default_filename.is_null ()                                       // C 2774
    { gtk_file_selection_set_filename (filesel, default_filename); }    // C 2775

  if mustmatch_p                                                        // C 2777
    {
      /* C 2779's note, carried as C's own text at its position.  */
      /* C: /* The selection_entry part of filesel is not documented.  */  */
      gtk_widget_set_sensitive ((*filesel).selection_entry, FALSE);      // C 2780
      gtk_file_selection_hide_fileop_buttons (filesel);                  // C 2781
    }

  *func = Some (xg_get_file_name_from_selector);                         // C 2784

  filewin                                                               // C 2786
}

/* C 2790-2801's comment above `xg_get_file_name`, carried as C's own text.  */
/* C: /* Read a file name from the user using a file dialog, either the old
   C:    file selection dialog, or the new file chooser dialog.  Which to use
   C:    depends on what the GTK version used has, and what the value of
   C:    gtk-use-old-file-dialog.
   C:    F is the current frame.
   C:    PROMPT is a prompt to show to the user.  May not be NULL.
   C:    DEFAULT_FILENAME is a default selection to be displayed.  May be NULL.
   C:    If MUSTMATCH_P, the returned file name must be an existing
   C:    file.

   C:    Returns a file name or NULL if no file was selected.
   C:    The returned string must be freed by the caller.  */  */

/// C 2803-2842: `char *xg_get_file_name (struct frame *f, char *prompt, char *default_filename,
/// bool mustmatch_p, bool only_dir_p)` - non-`static`, so `#[unsafe(no_mangle)]` with the C ABI.
/// C 2815-2818's `#ifdef HAVE_PGTK` guard is the `have_pgtk` feature and C 2817's `error` is
/// `include/lisp.rs` 3824's variadic; C 2820-2832's `#ifdef HAVE_GTK_FILE_SELECTION_NEW` pair is
/// the same feature as the two functions above; C 2810/2811's `0` are the port's null pointer and
/// null string, C 2813's `func` is `None` until one of the two arms stores an item, and C 2838's
/// `(*func) (w)` is the port's `(func.unwrap ()) (w)`.
/// C: char *
/// C: xg_get_file_name (struct frame *f,
/// C:                   char *prompt,
/// C:                   char *default_filename,
/// C:                   bool mustmatch_p,
/// C:                   bool only_dir_p)
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_get_file_name (f: *mut frame, prompt: *mut c_char,
					   default_filename: *mut c_char,
					   mustmatch_p: bool, only_dir_p: bool) -> *mut c_char
{
  let mut w: *mut GtkWidget = std::ptr::null_mut ();                    // C 2810
  let mut fn_: *mut c_char = std::ptr::null_mut ();                     // C 2811
  let mut filesel_done: c_int = 0;                                      // C 2812
  let mut func: xg_get_file_func = None;                                // C 2813

  /* C 2815's `#ifdef HAVE_PGTK`.  */
  #[cfg (feature = "have_pgtk")]
  { if FRAME_GTK_OUTER_WIDGET! (f).is_null ()                           // C 2816
      { error (c"Can't open dialog from child frames".as_ptr ()); } }   // C 2817

  /* C 2820's `#ifdef HAVE_GTK_FILE_SELECTION_NEW`.  */
  #[cfg (feature = "have_gtk_file_selection_new")]
  {
    if xg_uses_old_file_dialog ()                                       // C 2822
      {
	w = xg_get_file_with_selection (f, prompt, default_filename,    // C 2823
					mustmatch_p, only_dir_p, (&raw mut func));   // C 2824
      }
    else
      {
	w = xg_get_file_with_chooser (f, prompt, default_filename,      // C 2826
				      mustmatch_p, only_dir_p, (&raw mut func));   // C 2827
      }
  }
  #[cfg (not (feature = "have_gtk_file_selection_new"))]
  {
    w = xg_get_file_with_chooser (f, prompt, default_filename,          // C 2830
				  mustmatch_p, only_dir_p, (&raw mut func));   // C 2831
  }

  gtk_widget_set_name (w, c"emacs-filedialog".as_ptr ());               // C 2834

  filesel_done = xg_dialog_run (f, w);                                  // C 2836
  if filesel_done == GTK_RESPONSE_OK                                    // C 2837
    { fn_ = (func.unwrap ()) (w); }                                     // C 2838

  gtk_widget_destroy (w);                                               // C 2840
  fn_                                                                   // C 2841
}

/* C 2844-2846's "GTK font chooser" banner, carried as C's own text.  */
/* C: /***********************************************************************
   C:                       GTK font chooser
   C:  ***********************************************************************/  */

/* C 2848's `#ifdef HAVE_FREETYPE`: the whole font-chooser half below sits inside it (its `#endif`
   is C 3002), so every item here carries `have_freetype` beside the file gate - the way C 1666's
   own `xg_get_font` declaration does.  */
/* C 2850's `#ifdef HAVE_GTK3` opens here and C 2881's `#endif` closes it; the names of that arm
   are the mirror's GTK3/Pango surface (`include/gtk.rs`, P-90 (2)).  C 2859's
   `#if PANGO_VERSION_CHECK (1, 36, 7)` had no name, so this slice registers
   `pango_version_ge_1_36_7`; it decides one step of C 2856-2869's chain, so the function stands as
   the tree's two-`fn` arm pair (Deviation 3's shape), each arm keeping C's chain whole.  */

/// C 2852-2870: `static Lisp_Object xg_weight_to_symbol (PangoWeight w)` - `static` in C with one
/// call site (C 2976), so a plain `unsafe fn`; C's chained conditional is the port's `if`/`else if`
/// chain with the same steps in the same order, and C's own trailing comments name each step's
/// nominal weight.  The twelve `PANGO_WEIGHT_*` names are the mirror's placeholders (no Pango tree
/// on this machine).
/// C: static
/// C: Lisp_Object xg_weight_to_symbol (PangoWeight w)
#[cfg (all (feature = "use_gtk", feature = "have_freetype", feature = "have_gtk3",
	    feature = "pango_version_ge_1_36_7"))]
unsafe fn xg_weight_to_symbol (w: PangoWeight) -> Lisp_Object
{
  if w <= PANGO_WEIGHT_THIN { Qthin () }                          // C 2856 /* 100 */
  else if w <= PANGO_WEIGHT_ULTRALIGHT { Qultra_light () }        // C 2857 /* 200 */
  else if w <= PANGO_WEIGHT_LIGHT { Qlight () }                   // C 2858 /* 300 */
  else if w <= PANGO_WEIGHT_SEMILIGHT { Qsemi_light () }          // C 2860 /* 350 */
  else if w <= PANGO_WEIGHT_BOOK { Qbook () }                     // C 2862 /* 380 */
  else if w <= PANGO_WEIGHT_NORMAL { Qnormal () }                 // C 2863 /* 400 */
  else if w <= PANGO_WEIGHT_MEDIUM { Qmedium () }                 // C 2864 /* 500 */
  else if w <= PANGO_WEIGHT_SEMIBOLD { Qsemi_bold () }            // C 2865 /* 600 */
  else if w <= PANGO_WEIGHT_BOLD { Qbold () }                     // C 2866 /* 700 */
  else if w <= PANGO_WEIGHT_ULTRABOLD { Qultra_bold () }          // C 2867 /* 800 */
  else if w <= PANGO_WEIGHT_HEAVY { Qblack () }                   // C 2868 /* 900 */
  else { Qultra_heavy () }                                        // C 2869 /* 1000 */
}

/// C 2852-2870's other arm: C 2859-2861's `PANGO_WEIGHT_SEMILIGHT` step is the only text the
/// version condition removes, so this arm is C's chain without it.
#[cfg (all (feature = "use_gtk", feature = "have_freetype", feature = "have_gtk3"))]
#[cfg (not (feature = "pango_version_ge_1_36_7"))]
unsafe fn xg_weight_to_symbol (w: PangoWeight) -> Lisp_Object
{
  if w <= PANGO_WEIGHT_THIN { Qthin () }                          // C 2856 /* 100 */
  else if w <= PANGO_WEIGHT_ULTRALIGHT { Qultra_light () }        // C 2857 /* 200 */
  else if w <= PANGO_WEIGHT_LIGHT { Qlight () }                   // C 2858 /* 300 */
  else if w <= PANGO_WEIGHT_BOOK { Qbook () }                     // C 2862 /* 380 */
  else if w <= PANGO_WEIGHT_NORMAL { Qnormal () }                 // C 2863 /* 400 */
  else if w <= PANGO_WEIGHT_MEDIUM { Qmedium () }                 // C 2864 /* 500 */
  else if w <= PANGO_WEIGHT_SEMIBOLD { Qsemi_bold () }            // C 2865 /* 600 */
  else if w <= PANGO_WEIGHT_BOLD { Qbold () }                     // C 2866 /* 700 */
  else if w <= PANGO_WEIGHT_ULTRABOLD { Qultra_bold () }          // C 2867 /* 800 */
  else if w <= PANGO_WEIGHT_HEAVY { Qblack () }                   // C 2868 /* 900 */
  else { Qultra_heavy () }                                        // C 2869 /* 1000 */
}

/// C 2872-2879: `static Lisp_Object xg_style_to_symbol (PangoStyle s)` - `static` in C with one
/// call site (C 2977), so a plain `unsafe fn`; C's chain is the port's `if`/`else if`, and C 2878's
/// fall-through `Qnormal` is the port's final `else`.
/// C: static
/// C: Lisp_Object xg_style_to_symbol (PangoStyle s)
#[cfg (all (feature = "use_gtk", feature = "have_freetype", feature = "have_gtk3"))]
unsafe fn xg_style_to_symbol (s: PangoStyle) -> Lisp_Object
{
  if s == PANGO_STYLE_OBLIQUE { Qoblique () }                     // C 2876
  else if s == PANGO_STYLE_ITALIC { Qitalic () }                  // C 2877
  else { Qnormal () }                                             // C 2878
}

/* C 2881's `#endif /* HAVE_GTK3 */` and C 2883's second blank line.  */

/// C 2884: `static char *x_last_font_name;` - the file-scope object `extract-abi.py` counts under
/// `## File-scope objects` and the one this slice retires from the ledger's
/// `missing file-scope object` rows.  C's static storage starts zeroed, which is the port's null
/// initialiser (PORTING.md 1.5's static-storage shape).
#[cfg (all (feature = "use_gtk", feature = "have_freetype"))]
static mut x_last_font_name: *mut c_char = std::ptr::null_mut ();   // C 2884

/* C 2886's `#ifdef HAVE_GTK3` (its `#endif` is C 2899).  */

/* C 2895's `font_is_ignored` (font.h 861: `extern bool font_is_ignored (const char *name,
   ptrdiff_t namelen);`) - the port defines it in `src/font.rs` 3592, which `include!`s the
   font.h mirror (`src/font.rs` 87), so the mirror cannot carry the prototype without a
   duplicate item in that unit (`dupstring`'s class above); the declaration stands here and
   the ledger's Owed names it.  */
#[cfg (all (feature = "use_gtk", feature = "have_freetype", feature = "have_gtk3"))]
unsafe extern "C" { fn font_is_ignored (name: *const c_char, namelen: ptrdiff_t) -> bool; }

/// C 2887-2898: `static gboolean xg_font_filter (const PangoFontFamily *family, const
/// PangoFontFace *face, gpointer data)` - `static` in C, but C 2927 passes its address to
/// `gtk_font_chooser_set_filter_func`, so the port defines it `unsafe extern "C" fn` (the
/// `dialog_delete_callback` shape above).  C 2892's `(PangoFontFamily *)family` cast is the
/// prototype's own non-const parameter, C 2893's `strlen` is `lib/include/string.rs` 28's and its
/// `ptrdiff_t` result is `include/lisp.rs` 55's `isize`, C 2895-2897's `if (…) return FALSE;
/// return TRUE;` is the port's `if`/`else` expression, and the two parameters C's body does not
/// read keep C's names (the `dialog_delete_callback` shape).
/// C: static gboolean
/// C: xg_font_filter (const PangoFontFamily *family,
/// C:                 const PangoFontFace *face,
/// C:                 gpointer data)
#[cfg (all (feature = "use_gtk", feature = "have_freetype", feature = "have_gtk3"))]
unsafe extern "C" fn xg_font_filter (family: *const PangoFontFamily,
				     face: *const PangoFontFace,   // C 2889
				     data: gpointer) -> gboolean   // C 2890
{
  let name: *const c_char
    = pango_font_family_get_name (family as *mut PangoFontFamily);   // C 2892
  let namelen: ptrdiff_t = strlen (name) as ptrdiff_t;               // C 2893

  if font_is_ignored (name, namelen) { FALSE } else { TRUE }         // C 2895-2897
}

/* C 2980's `dupstring` (lisp.h 5555, `alloc.c`) - C 2980 is this file's only call site;
   it is a same-program function of `alloc.c` that no mirror carries yet, so the four
   units that already call it declare it themselves (`src/term.rs` 384-385's shape).  The
   declaration belongs in `include/lisp.rs`; the ledger's Owed names that.  */
#[cfg (all (feature = "use_gtk", feature = "have_freetype"))]
unsafe extern "C" { fn dupstring (ptr: *mut *mut c_char, string: *const c_char); }

/* C 2901-2909's comment above `xg_get_font`, carried as C's own text.  */
/* C: /* Pop up a GTK font selector and return the name of the font the user
   C:    selects, as a C string.  The returned font name follows GTK's own
   C:    format:

   C:    `FAMILY [VALUE1 VALUE2] SIZE'

   C:    This can be parsed using font_parse_fcname in font.c.
   C:    DEFAULT_NAME, if non-null, is the default font name;
   C:    it might be updated in place.  */  */

/// C 2911-3001: `Lisp_Object xg_get_font (struct frame *f, char *default_name)` - non-`static`, so
/// `#[unsafe(no_mangle)]` with the C ABI, and C 2929/2943's assignment to the parameter is the
/// port's `mut`.  C 2918-2921's `#ifdef HAVE_PGTK` guard is the `have_pgtk` feature and C 2920's
/// `error` is `include/lisp.rs` 3824's variadic; C 2923-2924's `gtk_font_chooser_dialog_new` is
/// the mirror's macro form (GTK3's function behind it, the GTK2 arm's C 110-112 macro in this
/// unit) and C 2924's `GTK_WINDOW` is the mirror's cast; C 2926-2928, C 2947-2954 and C 2961-2996
/// are the `have_gtk3` pairs - C 2953 is the GTK2 arm's `gtk_font_chooser_set_font` macro (C
/// 115-117) and C 2987-2988 the GTK2 `gtk_font_selection_dialog_get_font_name`; C 2931-2932's note
/// is carried as C's own text; C 2933/2937/2939's byte arguments are C's `'-'`, its digit test and
/// its `' '` (`strrchr`, `c_isdigit`, the store); C 2942-2943's read and C 2980's
/// `dupstring (&x_last_font_name, …)` are the file-scope object above, whose address the port
/// takes with `&raw mut` (its `static mut` form).  C 2980's `dupstring` is lisp.h 5555's
/// (`alloc.c`), which no mirror carries yet - the four units that already call it declare it
/// themselves (`src/term.rs` 384-385's shape); the declaration stands above and the ledger's
/// Owed names it.
/// C: Lisp_Object
/// C: xg_get_font (struct frame *f, char *default_name)
#[cfg (all (feature = "use_gtk", feature = "have_freetype"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_get_font (f: *mut frame, mut default_name: *mut c_char)
  -> Lisp_Object
{
  let w: *mut GtkWidget;                                               // C 2914
  let mut done: c_int = 0;                                             // C 2915
  let mut font: Lisp_Object = Qnil ();                                 // C 2916

  /* C 2918's `#ifdef HAVE_PGTK`.  */
  #[cfg (feature = "have_pgtk")]
  { if FRAME_GTK_OUTER_WIDGET! (f).is_null ()                          // C 2919
      { error (c"Can't open dialog from child frames".as_ptr ()); } }  // C 2920

  w = gtk_font_chooser_dialog_new! (c"Pick a font".as_ptr (),          // C 2923
				    GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)));   // C 2924

  /* C 2926's `#ifdef HAVE_GTK3`.  */
  #[cfg (feature = "have_gtk3")]
  { gtk_font_chooser_set_filter_func (GTK_FONT_CHOOSER! (w),           // C 2927
				      /* C's second argument is `xg_font_filter`, whose own
					 declarator makes the two pointers const where GTK's
					 `PangoFontFilterFunc` does not; C converts silently, so the
					 port spells the conversion out (PORTING.md 1.4).  */
				      Some (core::mem::transmute::<
					      *const (),
					      unsafe extern "C" fn (
						*mut PangoFontFamily, *mut PangoFontFace,
						gpointer) -> gboolean> (xg_font_filter as *const ())),
				      std::ptr::null_mut (), None); }      // C 2927
  if !default_name.is_null ()                                          // C 2929
    {
      /* Convert fontconfig names to Gtk names, i.e. remove - before
	 number */                                                     // C 2931-2932
      let p: *mut c_char = strrchr (default_name, b'-' as c_int);      // C 2933
      if !p.is_null ()                                                 // C 2934
	{
	  let mut ep: *mut c_char = p.offset (1);                      // C 2936
	  while c_isdigit (*ep as c_int)                               // C 2937
	    { ep = ep.add (1); }                                       // C 2938
	  if *ep == 0 { *p = b' ' as c_char; }                         // C 2939
	}
    }
  else if !x_last_font_name.is_null ()                                 // C 2942
    { default_name = x_last_font_name; }                               // C 2943

  if !default_name.is_null ()                                          // C 2945
    {
      #[cfg (feature = "have_gtk3")]
      {
	let desc: *mut PangoFontDescription
	  = pango_font_description_from_string (default_name);         // C 2948-2949
	gtk_font_chooser_set_font_desc (GTK_FONT_CHOOSER! (w), desc);  // C 2950
	pango_font_description_free (desc);                            // C 2951
      }
      #[cfg (not (feature = "have_gtk3"))]
      { gtk_font_chooser_set_font! (GTK_FONT_CHOOSER! (w), default_name); }   // C 2953
    }

  gtk_widget_set_name (w, c"emacs-fontdialog".as_ptr ());              // C 2957
  done = xg_dialog_run (f, w);                                         // C 2958
  if done == GTK_RESPONSE_OK                                           // C 2959
    {
      /* C 2961's `#ifdef HAVE_GTK3`.  */
      #[cfg (feature = "have_gtk3")]
      {
	/* Use the GTK3 font chooser.  */                              // C 2962
	let desc: *mut PangoFontDescription
	  = gtk_font_chooser_get_font_desc (GTK_FONT_CHOOSER! (w));    // C 2963-2964

	if !desc.is_null ()                                            // C 2966
	  {
	    let family: *const c_char
	      = pango_font_description_get_family (desc);              // C 2968
	    let size: gint = pango_font_description_get_size (desc);    // C 2969
	    let weight: PangoWeight
	      = pango_font_description_get_weight (desc);               // C 2970
	    let style: PangoStyle
	      = pango_font_description_get_style (desc);                // C 2971

	    font = CALLN! (Ffont_spec,                                  // C 2973
			   QCfamily (), build_string (family),           // C 2974
			   QCsize (), make_float (pango_units_to_double (size)),   // C 2975
			   QCweight (), xg_weight_to_symbol (weight),    // C 2976
			   QCslant (), xg_style_to_symbol (style));      // C 2977

	    let font_desc_str: *mut c_char
	      = pango_font_description_to_string (desc);               // C 2979
	    dupstring ((&raw mut x_last_font_name), font_desc_str);    // C 2980
	    g_free (font_desc_str as gpointer);                        // C 2981
	    pango_font_description_free (desc);                        // C 2982
	  }
      }
      /* C 2985's `#else /* Use old font selector, which just returns the font name.  */`.  */
      #[cfg (not (feature = "have_gtk3"))]
      {
	let font_name: *mut c_char
	  = gtk_font_selection_dialog_get_font_name (GTK_FONT_CHOOSER! (w));   // C 2987-2988

	if !font_name.is_null ()                                       // C 2990
	  {
	    font = build_string (font_name);                           // C 2992
	    g_free (x_last_font_name as gpointer);                     // C 2993
	    x_last_font_name = font_name;                              // C 2994
	  }
      }
    }

  gtk_widget_destroy (w);                                              // C 2999
  font                                                                 // C 3000
}
/* C 3002's `#endif /* HAVE_FREETYPE */`.  */


/* C 3003-3004's two blanks and C 3005's form feed, carried at C's own position.  */
/* C 3006-3008's "Menu functions." banner, carried as C's own text.  */
/* C: /***********************************************************************
   C: 	                Menu functions.
   C:  ***********************************************************************/  */

/* C 3010-3012's comment above `MENU_ITEM_NAME`, carried as C's own text.  */
/* C: /* The name of menu items that can be used for customization.  Since GTK
   C:    RC files are very crude and primitive, we have to set this on all
   C:    menu item names so a user can easily customize menu items.  */  */

/// C 3014: `#define MENU_ITEM_NAME "emacs-menuitem"` - the name C 3230-3232 gives every widget
/// of a menu item.  C's macro is a string literal and the port keeps that form - the
/// `&CStr` shape this file already gives its own keys (`TB_INFO_KEY` at C 132 and
/// `EMACS_CLASS` at C 7), which is also what `extract-abi.py`'s constant comparison reads,
/// since the `*const c_char` the sibling units use for their keys is a pointer where C's is
/// the string's own bytes; the three call sites below therefore take C's argument as
/// `MENU_ITEM_NAME.as_ptr ()`.
#[cfg (feature = "use_gtk")]
pub const MENU_ITEM_NAME: &core::ffi::CStr = c"emacs-menuitem";

/* ----------------------------------------------------------------------------------------------
   The declarations this section takes from `gtkutil.h`, `xterm.h` and `termhooks.h`.

   `gtkutil.h`'s mirror (`femacs/src/include/gtkutil.rs`) is a placeholder: its ledger records
   the P-08 decision that the header's whole body (C 24-243) is dead text under the frozen
   configuration, so its names stand in the unit at their first use - the shape C 619's
   `xg_list_node` above already has, and the shape `src/pgtkmenu.rs` 147-170 and `src/xmenu.rs`
   822-846 already use for two of these three records.  P-72 rule 4 and the ledger's Owed name
   the move-back: one `include/gtkutil.rs` body carrying each name once, with the consumers'
   copies deleted.  That round also has to reconcile `src/xmenu.rs`'s copy of
   `xg_menu_cb_data`, whose first member is `f` where gtkutil.h 63-72 puts the `xg_list_node`
   `ptrs` head - a wrong layout for a record gtkutil.c itself allocates, and one the ledger's
   Owed records.
   ---------------------------------------------------------------------------------------------- */

/// C: gtkutil.h 49: `#define XG_ITEM_DATA "emacs_menuitem"` - the key C 3177 reads the menu
/// item's callback record with.
#[cfg (feature = "use_gtk")]
pub const XG_ITEM_DATA: *const c_char = c"emacs_menuitem".as_ptr ();

/// C: gtkutil.h 63-72: `typedef struct xg_menu_cb_data_ { xg_list_node ptrs; struct frame *f;
/// Lisp_Object menu_bar_vector; int menu_bar_items_used; GCallback highlight_cb; int ref_count;
/// } xg_menu_cb_data;` - C 3035-3052 writes every member, C 3073-3076 rewrites four of them and
/// C 3086-3093 reads `ref_count`.  The head member is this unit's C 619 copy of gtkutil.h
/// 52-56, and `GCallback` is `include/gtk.rs` 93's (glib's `gclosure.h` 92, on this machine).
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct xg_menu_cb_data
{
  pub ptrs: xg_list_node,                  // gtkutil.h 65: `xg_list_node  ptrs;`
  pub f: *mut frame,                       // gtkutil.h 67: `struct frame  *f;`
  pub menu_bar_vector: Lisp_Object,        // gtkutil.h 68: `Lisp_Object   menu_bar_vector;`
  pub menu_bar_items_used: c_int,          // gtkutil.h 69: `int           menu_bar_items_used;`
  pub highlight_cb: GCallback,             // gtkutil.h 71: `GCallback     highlight_cb;`
  pub ref_count: c_int,                    // gtkutil.h 72: `int           ref_count;`
}

/// C: gtkutil.h 75-84: `typedef struct xg_menu_item_cb_data_ { xg_list_node ptrs; gulong
/// select_id; Lisp_Object help; gpointer call_data; xg_menu_cb_data *cl_data; }
/// xg_menu_item_cb_data;` - C 3110/3153 use `ptrs`, C 3112-3113 and C 3180 read `help`, and
/// C 3180/3183 read `cl_data`.  `gulong` is `include/gtk.rs` 56's alias of `c_ulong`.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct xg_menu_item_cb_data
{
  pub ptrs: xg_list_node,                  // gtkutil.h 77: `xg_list_node  ptrs;`
  pub select_id: c_ulong,                  // gtkutil.h 79: `gulong        select_id;`
  pub help: Lisp_Object,                   // gtkutil.h 80: `Lisp_Object   help;`
  pub call_data: gpointer,                 // gtkutil.h 81: `gpointer	call_data;`
  pub cl_data: *mut xg_menu_cb_data,       // gtkutil.h 82: `xg_menu_cb_data *cl_data;`
}

/* C 3017-3018's comment above the first list, carried as C's own text.  */
/* C: /* Linked list of all allocated struct xg_menu_cb_data.  Used for marking
   C:    during GC.  The next member points to the items.  */  */
/// C 3019: `static xg_list_node xg_menu_cb_list;` - the object the ledger's
/// `missing file-scope object` rows name, and the list C 3047 inserts into and C 3091 removes
/// from.  C's static storage starts zeroed, which the port writes out (PORTING.md 1.5's
/// static-storage shape).
#[cfg (feature = "use_gtk")]
static mut xg_menu_cb_list: xg_list_node                                    // C 3019
  = xg_list_node { prev: std::ptr::null_mut (), next: std::ptr::null_mut () };

/* C 3021-3022's comment above the second list, carried as C's own text.  */
/* C: /* Linked list of all allocated struct xg_menu_item_cb_data.  Used for marking
   C:    during GC.  The next member points to the items.  */  */
/// C 3023: `static xg_list_node xg_menu_item_cb_list;` - C 3154 removes from it and C 3339 (the
/// next slice) inserts into it.
#[cfg (feature = "use_gtk")]
static mut xg_menu_item_cb_list: xg_list_node                               // C 3023
  = xg_list_node { prev: std::ptr::null_mut (), next: std::ptr::null_mut () };

/* C 3025-3033's comment above `make_cl_data`, carried as C's own text.  */
/* C: /* Allocate and initialize CL_DATA if NULL, otherwise increase ref_count.
   C:    F is the frame CL_DATA will be initialized for.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.

   C:    The menu bar and all sub menus under the menu bar in a frame
   C:    share the same structure, hence the reference count.

   C:    Returns CL_DATA if CL_DATA is not NULL,  or a pointer to a newly
   C:    allocated xg_menu_cb_data if CL_DATA is NULL.  */  */

/// C 3035-3053: `static xg_menu_cb_data *make_cl_data (xg_menu_cb_data *cl_data, struct frame
/// *f, GCallback highlight_cb)` - `static` in C and its address is never taken, so a plain
/// `unsafe fn`.  C 3038's `if (! cl_data)` writes the parameter, which is the port's `mut`;
/// C 3040's `sizeof *cl_data` is the port's `size_of`, C 3047's two `&` operands are the
/// `&raw mut` forms of the two file-scope/record places, and C 3050's `cl_data->ref_count++`
/// is the port's `+= 1`.
/// C: static xg_menu_cb_data *
/// C: make_cl_data (xg_menu_cb_data *cl_data, struct frame *f, GCallback highlight_cb)
#[cfg (feature = "use_gtk")]
unsafe fn make_cl_data (mut cl_data: *mut xg_menu_cb_data, f: *mut frame,
			highlight_cb: GCallback) -> *mut xg_menu_cb_data
{
  if cl_data.is_null ()                                                 // C 3038
    {
      cl_data = xmalloc (core::mem::size_of::<xg_menu_cb_data> ())      // C 3040
	as *mut xg_menu_cb_data;
      (*cl_data).f = f;                                                 // C 3041
      (*cl_data).menu_bar_vector = (*f).menu_bar_vector;                // C 3042
      (*cl_data).menu_bar_items_used = (*f).menu_bar_items_used;        // C 3043
      (*cl_data).highlight_cb = highlight_cb;                           // C 3044
      (*cl_data).ref_count = 0;                                         // C 3045

      xg_list_insert ((&raw mut xg_menu_cb_list),
		      (&raw mut (*cl_data).ptrs));                      // C 3047
    }

  (*cl_data).ref_count += 1;                                            // C 3050

  cl_data                                                               // C 3052
}

/* C 3055-3064's comment above `update_cl_data`, carried as C's own text.  */
/* C: /* Update CL_DATA with values from frame F and with HIGHLIGHT_CB.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.

   C:    When the menu bar is updated, menu items may have been added and/or
   C:    removed, so menu_bar_vector and menu_bar_items_used change.  We must
   C:    then update CL_DATA since it is used to determine which menu
   C:    item that is invoked in the menu.
   C:    HIGHLIGHT_CB could change, there is no check that the same
   C:    function is given when modifying a menu bar as was given when
   C:    creating the menu bar.  */  */

/// C 3066-3078: `static void update_cl_data (xg_menu_cb_data *cl_data, struct frame *f,
/// GCallback highlight_cb)` - `static`, address never taken, so a plain `unsafe fn`; C 3071's
/// guard keeps the four stores off a null pointer in both C and the port.
/// C: static void
/// C: update_cl_data (xg_menu_cb_data *cl_data,
/// C:                 struct frame *f,
/// C:                 GCallback highlight_cb)
#[cfg (feature = "use_gtk")]
unsafe fn update_cl_data (cl_data: *mut xg_menu_cb_data, f: *mut frame, highlight_cb: GCallback)
{
  if ! cl_data.is_null ()                                               // C 3071
    {
      (*cl_data).f = f;                                                 // C 3073
      (*cl_data).menu_bar_vector = (*f).menu_bar_vector;                // C 3074
      (*cl_data).menu_bar_items_used = (*f).menu_bar_items_used;        // C 3075
      (*cl_data).highlight_cb = highlight_cb;                           // C 3076
    }
}

/* C 3080-3081's comment above `unref_cl_data`, carried as C's own text.  */
/* C: /* Decrease reference count for CL_DATA.
   C:    If reference count is zero, free CL_DATA.  */  */

/// C 3083-3095: `static void unref_cl_data (xg_menu_cb_data *cl_data)` - `static,` address
/// never taken, so a plain `unsafe fn`; C 3086's two tests are the port's null check and the
/// same `> 0`, C 3088's `--` is `-= 1`, and C 3092's `xfree` takes the `*mut c_void` the
/// port's allocator declares.
/// C: static void
/// C: unref_cl_data (xg_menu_cb_data *cl_data)
#[cfg (feature = "use_gtk")]
unsafe fn unref_cl_data (cl_data: *mut xg_menu_cb_data)
{
  if ! cl_data.is_null () && (*cl_data).ref_count > 0                   // C 3086
    {
      (*cl_data).ref_count -= 1;                                        // C 3088
      if (*cl_data).ref_count == 0                                      // C 3089
	{
	  xg_list_remove ((&raw mut xg_menu_cb_list),
			  (&raw mut (*cl_data).ptrs));                  // C 3091
	  xfree (cl_data as *mut c_void);                                   // C 3092
	}
    }
}

/* C 3097's comment above `xg_mark_data`, carried as C's own text.  */
/* C: /* Function that marks all lisp data during GC.  */  */

/// C 3099-3142: `void xg_mark_data (void)` - non-`static` (gtkutil.h 232 declares it and
/// `alloc.c`'s GC calls it), so `#[unsafe(no_mangle)]` with the C ABI.  C 3105/3108's two
/// `for` heads are the port's `while` with C's update step at the loop's end (PORTING.md 2.2),
/// C 3106/3110's casts are the port's `as`, C 3116's `FOR_EACH_FRAME` is `include/lisp.rs`'s
/// macro form, C 3120's bare pointer test is the port's null check, C 3122-3124's
/// `g_object_get_data` result takes C's own conversion, and C 3134-3139's `#ifndef HAVE_PGTK`
/// arm reads the quit event whose `.kind` bitfield is the port's `input_event_bits!` accessor
/// (PORTING.md P-80) over the object declared below.
/// C: void
/// C: xg_mark_data (void)
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_mark_data ()
{
  let mut iter: *mut xg_list_node;                                      // C 3102
  let mut rest: Lisp_Object;                                            // C 3103
  let mut frame: Lisp_Object;

  iter = xg_menu_cb_list.next;                                          // C 3105
  while ! iter.is_null ()                                               // C 3105
    {
      mark_object ((*(iter as *mut xg_menu_cb_data)).menu_bar_vector);   // C 3106
      iter = (*iter).next;                                              // C 3105 (the `for` step)
    }

  iter = xg_menu_item_cb_list.next;                                     // C 3108
  while ! iter.is_null ()                                               // C 3108
    {
      let cb_data: *mut xg_menu_item_cb_data
	= iter as *mut xg_menu_item_cb_data;                            // C 3110

      if ! NILP ((*cb_data).help)                                       // C 3112
	{ mark_object ((*cb_data).help); }                              // C 3113
      iter = (*iter).next;                                              // C 3108 (the `for` step)
    }

  FOR_EACH_FRAME! (rest, frame, {                                       // C 3116
      let f: *mut frame = XFRAME (frame);                               // C 3118

      if (FRAME_X_P (f) || FRAME_PGTK_P (f))                            // C 3120
	  && ! FRAME_GTK_OUTER_WIDGET! (f).is_null ()
	{
	  let tbinfo: *mut xg_frame_tb_info
	    = g_object_get_data (G_OBJECT! (FRAME_GTK_OUTER_WIDGET! (f)),
				 TB_INFO_KEY.as_ptr ()) as *mut xg_frame_tb_info;   // C 3122-3124
	  if ! tbinfo.is_null ()                                        // C 3125
	    {
	      mark_object ((*tbinfo).last_tool_bar);                    // C 3127
	      mark_object ((*tbinfo).style);                            // C 3128
	    }
	}
    });

  /* C 3133's `#ifndef HAVE_PGTK`.  */
  #[cfg (not (feature = "have_pgtk"))]
  {
    if input_event_bits! (kind, (&raw mut xg_pending_quit_event)) != NO_EVENT   // C 3134
      {
	eassert! (input_event_bits! (kind, (&raw mut xg_pending_quit_event))
		  == ASCII_KEYSTROKE_EVENT);                            // C 3136

	mark_object (xg_pending_quit_event.frame_or_window);            // C 3138
	mark_object (xg_pending_quit_event.arg);                        // C 3139
      }
  }
}

/* `mark_object` (lisp.h 4436, `alloc.c`) and `xg_pending_quit_event` (xterm.h 2012, defined in
   `src/xterm.rs` 769 - `xterm.c` 815) are same-program declarations no mirror carries:
   `include/lisp.rs` carries `mark_objects` alone, and the xterm.h mirror leaves the object to
   the unit that defines it (its own `xterm_globals` module).  Six units already declare
   `mark_object` this way (`src/eval.rs` 77, `src/bytecode.rs` 104 and the rest) and the
   ledger's Owed names both gaps.  */
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn mark_object (obj: Lisp_Object);                                // lisp.h 4436
  #[cfg (not (feature = "have_pgtk"))]
  pub static mut xg_pending_quit_event: input_event;                    // xterm.h 2012
}

/* `input_event_bits!` (termhooks.h's `input_event` bitfield accessor, PORTING.md P-80): every
   terminal unit carries its own copy of the four-arm accessor over
   `include/termhooks.rs`'s `INPUT_EVENT_KIND_MASK`/`INPUT_EVENT_PART_SHIFT` (`src/xterm.rs`
   1650-1658's shape), and C 3134/3136 are this unit's two reads.  */
#[cfg (feature = "use_gtk")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* C 3144-3146's comment above `menuitem_destroy_callback`, carried as C's own text.  */
/* C: /* Callback called when a menu item is destroyed.  Used to free data.
   C:    W is the widget that is being destroyed (not used).
   C:    CLIENT_DATA points to the xg_menu_item_cb_data associated with the W.  */  */

/// C 3148-3157: `static void menuitem_destroy_callback (GtkWidget *w, gpointer client_data)` -
/// `static` in C, but C 3393 (the next slice) takes its address for `g_signal_connect`, so the
/// port defines it `unsafe extern "C" fn` (the `dialog_delete_callback` shape above).  C 3153's
/// implicit `gpointer` conversion is the port's `as`, C 3151's pointer test is the null check,
/// and the unread parameter keeps C's name.
/// C: static void
/// C: menuitem_destroy_callback (GtkWidget *w, gpointer client_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn menuitem_destroy_callback (w: *mut GtkWidget, client_data: gpointer)
{
  if ! client_data.is_null ()                                           // C 3151
    {
      let data: *mut xg_menu_item_cb_data
	= client_data as *mut xg_menu_item_cb_data;                     // C 3153
      xg_list_remove ((&raw mut xg_menu_item_cb_list),
		      (&raw mut (*data).ptrs));                         // C 3154
      xfree (data as *mut c_void);                                       // C 3155
    }
}

/* C 3159-3164's comment above `menuitem_highlight_callback`, carried as C's own text.  */
/* C: /* Callback called when the pointer enters/leaves a menu item.
   C:    W is the parent of the menu item.
   C:    EVENT is either an enter event or leave event.
   C:    CLIENT_DATA is not used.

   C:    Returns FALSE to tell GTK to keep processing this event.  */  */

/// C 3166-3189: `static gboolean menuitem_highlight_callback (GtkWidget *w, GdkEventCrossing
/// *event, gpointer client_data)` - `static`, but C 3418 (the next slice) takes its address for
/// `g_signal_connect`, so `unsafe extern "C" fn`.  C 3171's uninitialised `GdkEvent ev` is the
/// port's zeroed local (PORTING.md 1.5), C 3175's `&ev` is `&raw mut`, C 3177's
/// `g_object_get_data` result takes C's conversion, C 3182's `0` is the null pointer and C
/// 3183's cast is the `GCallback`-to-`GtkCallback` transmute this file already uses for the
/// reverse direction (C 2344's device); C 3184's `(*func) (…)` is the port's `unwrap ()` call,
/// as C 2838's is.
/// C: static gboolean
/// C: menuitem_highlight_callback (GtkWidget *w,
/// C:                              GdkEventCrossing *event,
/// C:                              gpointer client_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn menuitem_highlight_callback (w: *mut GtkWidget,
						  event: *mut GdkEventCrossing,
						  client_data: gpointer) -> gboolean
{
  let mut ev: GdkEvent = core::mem::zeroed ();                          // C 3171
  let subwidget: *mut GtkWidget;                                        // C 3172
  let data: *mut xg_menu_item_cb_data;                                  // C 3173

  ev.crossing = *event;                                                  // C 3175
  subwidget = gtk_get_event_widget ((&raw mut ev));                     // C 3176
  data = g_object_get_data (G_OBJECT! (subwidget), XG_ITEM_DATA)        // C 3177
    as *mut xg_menu_item_cb_data;
  if ! data.is_null ()                                                  // C 3178
    {
      if ! NILP ((*data).help)                                          // C 3180
	  && (*((*data).cl_data)).highlight_cb.is_some ()
	{
	  let call_data: gpointer
	    = if (*event).r#type == GDK_LEAVE_NOTIFY
		{ std::ptr::null_mut () } else { data as gpointer };    // C 3182
	  let func: GtkCallback
	    = core::mem::transmute::<GCallback, GtkCallback> ((*((*data).cl_data)).highlight_cb);   // C 3183
	  (func.unwrap ()) (subwidget, call_data);                      // C 3184
	}
    }

  FALSE                                                                 // C 3188
}

/* C 3191-3193's comment above `menu_destroy_callback`, carried as C's own text.  */
/* C: /* Callback called when a menu is destroyed.  Used to free data.
   C:    W is the widget that is being destroyed (not used).
   C:    CLIENT_DATA points to the xg_menu_cb_data associated with W.  */  */

/// C 3195-3199: `static void menu_destroy_callback (GtkWidget *w, gpointer client_data)` -
/// `static`, but C 3468 (a later slice) takes its address for `g_signal_connect`, so
/// `unsafe extern "C" fn`.  C 3198's implicit conversion to `xg_menu_cb_data *` is the port's
/// `as`, and both parameters keep C's names.
/// C: static void
/// C: menu_destroy_callback (GtkWidget *w, gpointer client_data)
#[cfg (feature = "use_gtk")]
unsafe extern "C" fn menu_destroy_callback (w: *mut GtkWidget, client_data: gpointer)
{
  unref_cl_data (client_data as *mut xg_menu_cb_data);                  // C 3198
}

/* ----------------------------------------------------------------------------------------------
   The GTK/GDK/GLib names the two builders below need and no mirror carries.

   Four units that `include!` `include/gtk.rs` already carry their own copies of them
   (`src/pgtkmenu.rs` 212-227, `src/xmenu.rs` 776-791, `src/xwidget.rs` 300/408 and
   `src/pgtkterm.rs`), so the mirror cannot take these names without an `E0428` in those
   translation units; the move-back to `include/gtk.rs` with all five copies deleted is one
   round, and the ledger's Owed names it (P-90 ①/②).  **No GTK or GDK tree exists on this
   machine**, so every prototype here is `UNVERIFIED` and every enumerator is a placeholder
   (P-88 (2)) - except `GSList`, which is GLib's and therefore real.
   ---------------------------------------------------------------------------------------------- */

/// C: gtk/gtk.h: `typedef void (*GtkCallback) (GtkWidget *widget, gpointer data);` - the type
/// C 3183 casts the highlight callback to and C 3184 calls through.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GtkCallback = Option<unsafe extern "C" fn (widget: *mut GtkWidget, data: gpointer)>;
/// C: gtk/gtkcheckmenuitem.h: `typedef struct _GtkCheckMenuItem GtkCheckMenuItem;` - the type of
/// `GTK_CHECK_MENU_ITEM`'s result and of `gtk_check_menu_item_set_active`'s parameter
/// (C 3274/3282).  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkCheckMenuItem
{
  _private: [u8; 0],
}
/// C: gtk/gtkradiomenuitem.h: `typedef struct _GtkRadioMenuItem GtkRadioMenuItem;` - the type of
/// `GTK_RADIO_MENU_ITEM`'s result and of `gtk_radio_menu_item_get_group`'s parameter (C 3280).
/// **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkRadioMenuItem
{
  _private: [u8; 0],
}
/// C: gtk/gtkcheckmenuitem.h and gtk/gtkradiomenuitem.h: `#define GTK_CHECK_MENU_ITEM(obj)` /
/// `#define GTK_RADIO_MENU_ITEM(obj)` - C 3274/3280/3282's two checked casts, each keeping the
/// macro's name and its result type (the `src/pgtkmenu.rs` 220-227 and `src/xmenu.rs` 788-791
/// shapes); the `G_TYPE_CHECK_INSTANCE_CAST` chain is owed, as the mirror's other casts are.
#[cfg (feature = "use_gtk")]
macro_rules! GTK_CHECK_MENU_ITEM {
  ($o:expr) => { ($o) as *mut GtkCheckMenuItem };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_RADIO_MENU_ITEM {
  ($o:expr) => { ($o) as *mut GtkRadioMenuItem };
}
/// C: glib/gslist.h 39 and 41-45: `typedef struct _GSList GSList;` and `struct _GSList {
/// gpointer data; GSList *next; };` - the radio group C 3255's `GSList **group` threads through
/// the builders.  **VERIFIED**: GLib is on this machine
/// (`/opt/homebrew/include/glib-2.0/glib/gslist.h`), so the record is the header's own layout
/// and the four assertions are the `_GList` shape's (`include/gtk.rs` 60-75).
#[cfg (feature = "use_gtk")]
pub type GSList = _GSList;
#[cfg (feature = "use_gtk")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _GSList
{
  pub data: gpointer,        // gslist.h 43: `gpointer data;`
  pub next: *mut _GSList,    // gslist.h 44: `GSList *next;`
}
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<_GSList> () == 16);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::align_of::<_GSList> () == 8);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GSList, data) == 0);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GSList, next) == 8);
/// C: gdk/gdktypes.h: `GDK_LEAVE_NOTIFY` - the `GdkEventType` member C 3182's conditional tests.
/// **No GDK tree exists on this machine**, so the value is a **placeholder**; it is distinct
/// from every other member of `include/gtk.rs`'s set and it is the value `src/xwidget.rs` 300
/// already carries for the same enumerator (the `src/xwidget.rs` 495-520 rule).
#[cfg (feature = "use_gtk")]
pub const GDK_LEAVE_NOTIFY: GdkEventType = 8;         // placeholder, not GDK's value

/// C: gtk/gtkmenuitem.h, gtk/gtkcheckmenuitem.h and gtk/gtkradiomenuitem.h: the nine
/// constructors, accessors and the container call C 3272-3291 spends.  Every prototype is
/// `UNVERIFIED`.
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 3272: `if (utf8_key) w = gtk_check_menu_item_new ();`
  /// C: gtk/gtkcheckmenuitem.h: `GtkWidget *gtk_check_menu_item_new (void);`
  pub fn gtk_check_menu_item_new () -> *mut GtkWidget;
  /// C 3273: `else w = gtk_check_menu_item_new_with_label (utf8_label);`
  /// C: gtk/gtkcheckmenuitem.h: `GtkWidget *gtk_check_menu_item_new_with_label (const gchar
  /// *label);`
  pub fn gtk_check_menu_item_new_with_label (label: *const gchar) -> *mut GtkWidget;
  /// C 3274/3282: `gtk_check_menu_item_set_active (GTK_CHECK_MENU_ITEM (w), …);`
  /// C: gtk/gtkcheckmenuitem.h: `void gtk_check_menu_item_set_active (GtkCheckMenuItem
  /// *check_menu_item, gboolean is_active);`
  pub fn gtk_check_menu_item_set_active (check_menu_item: *mut GtkCheckMenuItem,
					 is_active: gboolean);
  /// C 3278: `if (utf8_key) w = gtk_radio_menu_item_new (*group);`
  /// C: gtk/gtkradiomenuitem.h: `GtkWidget *gtk_radio_menu_item_new (GSList *group);`
  pub fn gtk_radio_menu_item_new (group: *mut GSList) -> *mut GtkWidget;
  /// C 3279: `else w = gtk_radio_menu_item_new_with_label (*group, utf8_label);`
  /// C: gtk/gtkradiomenuitem.h: `GtkWidget *gtk_radio_menu_item_new_with_label (GSList *group,
  /// const gchar *label);`
  pub fn gtk_radio_menu_item_new_with_label (group: *mut GSList,
					     label: *const gchar) -> *mut GtkWidget;
  /// C 3280: `*group = gtk_radio_menu_item_get_group (GTK_RADIO_MENU_ITEM (w));`
  /// C: gtk/gtkradiomenuitem.h: `GSList *gtk_radio_menu_item_get_group (GtkRadioMenuItem
  /// *radio_menu_item);`
  pub fn gtk_radio_menu_item_get_group (radio_menu_item: *mut GtkRadioMenuItem)
    -> *mut GSList;
  /// C 3287: `if (utf8_key) w = gtk_menu_item_new ();`
  /// C: gtk/gtkmenuitem.h: `GtkWidget *gtk_menu_item_new (void);`
  pub fn gtk_menu_item_new () -> *mut GtkWidget;
  /// C 3288: `else w = gtk_menu_item_new_with_label (utf8_label);`
  /// C: gtk/gtkmenuitem.h: `GtkWidget *gtk_menu_item_new_with_label (const gchar *label);`
  pub fn gtk_menu_item_new_with_label (label: *const gchar) -> *mut GtkWidget;
}

/* C 3201-3204's comment above `make_widget_for_menu_item`, carried as C's own text.  */
/* C: /* Make a GTK widget that contains both UTF8_LABEL and UTF8_KEY (both
   C:    must be non-NULL) and can be inserted into a menu item.

   C:    Returns the GtkHBox.  */  */

/// C 3206-3235: `static GtkWidget *make_widget_for_menu_item (const char *utf8_label, const char
/// *utf8_key)` - `static` in C and its address is never taken, so a plain `unsafe fn`.  C
/// 3218-3226's `#if GTK_CHECK_VERSION (3, 14, 0)` pair is the `gtk_check_version_ge_3_14`
/// feature (C 2419's twin, registered by the third slice), the two `gtk_misc_set_alignment`
/// arguments C writes as `0.0, 0.5` are the mirror's `f32` parameters (glib.h 198's `gfloat`),
/// and C 3230-3232's `MENU_ITEM_NAME` is the constant above.
/// C: static GtkWidget *
/// C: make_widget_for_menu_item (const char *utf8_label, const char *utf8_key)
#[cfg (feature = "use_gtk")]
unsafe fn make_widget_for_menu_item (utf8_label: *const c_char, utf8_key: *const c_char)
  -> *mut GtkWidget
{
  let wlbl: *mut GtkWidget;                                             // C 3209
  let wkey: *mut GtkWidget;                                             // C 3210
  let wbox: *mut GtkWidget;                                             // C 3211

  wbox = gtk_box_new! (GTK_ORIENTATION_HORIZONTAL, 0);                  // C 3213
  gtk_box_set_homogeneous (GTK_BOX! (wbox), FALSE);                     // C 3214
  wlbl = gtk_label_new (utf8_label);                                    // C 3215
  wkey = gtk_label_new (utf8_key);                                      // C 3216

  /* C 3218's `#if GTK_CHECK_VERSION (3, 14, 0)`.  */
  #[cfg (feature = "gtk_check_version_ge_3_14")]
  {
    gtk_widget_set_halign (wlbl, GTK_ALIGN_START);                      // C 3219
    gtk_widget_set_valign (wlbl, GTK_ALIGN_CENTER);                     // C 3220
    gtk_widget_set_halign (wkey, GTK_ALIGN_START);                      // C 3221
    gtk_widget_set_valign (wkey, GTK_ALIGN_CENTER);                     // C 3222
  }
  #[cfg (not (feature = "gtk_check_version_ge_3_14"))]
  {
    gtk_misc_set_alignment (GTK_MISC! (wlbl), 0.0, 0.5);                // C 3224
    gtk_misc_set_alignment (GTK_MISC! (wkey), 0.0, 0.5);                // C 3225
  }
  gtk_box_pack_start (GTK_BOX! (wbox), wlbl, TRUE, TRUE, 0);            // C 3227
  gtk_box_pack_start (GTK_BOX! (wbox), wkey, FALSE, FALSE, 0);          // C 3228

  gtk_widget_set_name (wlbl, MENU_ITEM_NAME.as_ptr ());                // C 3230
  gtk_widget_set_name (wkey, MENU_ITEM_NAME.as_ptr ());                // C 3231
  gtk_widget_set_name (wbox, MENU_ITEM_NAME.as_ptr ());                // C 3232

  wbox                                                                  // C 3234
}

/* C 3237-3249's comment above `make_menu_item`, carried as C's own text.  */
/* C: /* Make and return a menu item widget with the key to the right.
   C:    UTF8_LABEL is the text for the menu item (GTK uses UTF8 internally).
   C:    UTF8_KEY is the text representing the key binding.
   C:    ITEM is the widget_value describing the menu item.

   C:    GROUP is an in/out parameter.  If the menu item to be created is not
   C:    part of any radio menu group, *GROUP contains NULL on entry and exit.
   C:    If the menu item to be created is part of a radio menu group, on entry
   C:    *GROUP contains the group to use, or NULL if this is the first item
   C:    in the group.  On exit, *GROUP contains the radio item group.

   C:    Unfortunately, keys don't line up as nicely as in Motif,
   C:    but the macOS version doesn't either, so I guess that is OK.  */  */

/// C 3251-3300: `static GtkWidget *make_menu_item (const char *utf8_label, const char
/// *utf8_key, widget_value *item, GSList **group)` - `static`, address never taken, so a plain
/// `unsafe fn`.  C 3264's write to the parameter is the port's `mut` and its `" "` literal is
/// the port's one-element `c" "`, C 3258's `= 0` is the null pointer, C 3274's and C 3282's
/// `item->selected` is the mirror's `bool` promoted to `gboolean` (C's own usual conversion),
/// and C 3294-3297's `#ifdef HAVE_PGTK` arm reads `widget_value`'s help through `NILP`/`SSDATA`
/// as the file's other arms do.
/// C: static GtkWidget *
/// C: make_menu_item (const char *utf8_label,
/// C:                 const char *utf8_key,
/// C:                 widget_value *item,
/// C:                 GSList **group)
#[cfg (feature = "use_gtk")]
unsafe fn make_menu_item (mut utf8_label: *const c_char, utf8_key: *const c_char,
			  item: *mut widget_value, group: *mut *mut GSList) -> *mut GtkWidget
{
  let w: *mut GtkWidget;                                                // C 3257
  let mut wtoadd: *mut GtkWidget = std::ptr::null_mut ();               // C 3258

  /* It has been observed that some menu items have a NULL name field.
     This will lead to this function being called with a NULL utf8_label.
     GTK crashes on that so we set a blank label.  Why there is a NULL
     name remains to be investigated.  */                               // C 3260-3263
  if utf8_label.is_null () { utf8_label = c" ".as_ptr (); }            // C 3264

  if ! utf8_key.is_null ()                                              // C 3266
    { wtoadd = make_widget_for_menu_item (utf8_label, utf8_key); }      // C 3267

  if (*item).button_type == BUTTON_TYPE_TOGGLE                          // C 3269
    {
      *group = std::ptr::null_mut ();                                   // C 3271
      if ! utf8_key.is_null () { w = gtk_check_menu_item_new (); }       // C 3272
      else { w = gtk_check_menu_item_new_with_label (utf8_label); }      // C 3273
      gtk_check_menu_item_set_active (GTK_CHECK_MENU_ITEM! (w),          // C 3274
				      (*item).selected as gboolean);
    }
  else if (*item).button_type == BUTTON_TYPE_RADIO                       // C 3276
    {
      if ! utf8_key.is_null () { w = gtk_radio_menu_item_new (*group); }   // C 3278
      else { w = gtk_radio_menu_item_new_with_label (*group, utf8_label); }   // C 3279
      *group = gtk_radio_menu_item_get_group (GTK_RADIO_MENU_ITEM! (w));   // C 3280
      if (*item).selected                                               // C 3281
	{ gtk_check_menu_item_set_active (GTK_CHECK_MENU_ITEM! (w), TRUE); }   // C 3282
    }
  else
    {
      *group = std::ptr::null_mut ();                                   // C 3286
      if ! utf8_key.is_null () { w = gtk_menu_item_new (); }             // C 3287
      else { w = gtk_menu_item_new_with_label (utf8_label); }            // C 3288
    }

  if ! wtoadd.is_null ()                                                // C 3291
    { gtk_container_add (GTK_CONTAINER! (w), wtoadd); }
  if ! (*item).enabled                                                  // C 3292
    { gtk_widget_set_sensitive (w, FALSE); }

  /* C 3294's `#ifdef HAVE_PGTK`.  */
  #[cfg (feature = "have_pgtk")]
  { if ! NILP ((*item).help)                                            // C 3295
      { gtk_widget_set_tooltip_text (w, SSDATA ((*item).help)); } }      // C 3296

  w                                                                     // C 3299
}

/* C 3301's blank and C 3302-3314's comment above `xg_create_one_menuitem`, carried as C's own
   text.  (The row table's slice-6 entry ends at C 3301: this comment belongs to the function
   below and is carried here.)  */
/* C: /* Create a menu item widget, and connect the callbacks.
   C:    ITEM describes the menu item.
   C:    F is the frame the created menu belongs to.
   C:    SELECT_CB is the callback to use when a menu item is selected.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.
   C:    CL_DATA points to the callback data to be used for this menu.
   C:    GROUP is an in/out parameter.  If the menu item to be created is not
   C:    part of any radio menu group, *GROUP contains NULL on entry and exit.
   C:    If the menu item to be created is part of a radio menu group, on entry
   C:    *GROUP contains the group to use, or NULL if this is the first item
   C:    in the group.  On exit, *GROUP contains the radio item group.

   C:    Returns the created GtkWidget.  */  */

/// C 3316-3363: `static GtkWidget *xg_create_one_menuitem (widget_value *item, struct frame *f,
/// GCallback select_cb, GCallback highlight_cb, xg_menu_cb_data *cl_data, GSList **group)` -
/// `static` and its address is never taken, so a plain `unsafe fn`.  C 3329/3330's
/// `get_utf8_string` is this file's own (C 655-729), C 3334/3335's `g_free` takes C's pointer
/// through the port's `*mut c_void`, C 3337's `sizeof *cb_data` is the port's `size_of`,
/// C 3346-3348's four-argument `g_signal_connect` is the mirror's `g_signal_connect!` with C's
/// `cb_data` cast to the `gpointer` its data argument takes, and C 3358-3359's `g_signal_connect`
/// passes the *variable* `select_cb` - which the port's macro would wrap a second time in
/// `G_CALLBACK!` (that macro needs a function item: `E0605`, measured in slice 3) - so that one
/// call stands in the macro's own expansion, its `gulong` result being what `select_id` holds.
/// C: static GtkWidget *
/// C: xg_create_one_menuitem (widget_value *item,
/// C:                         struct frame *f,
/// C:                         GCallback select_cb,
/// C:                         GCallback highlight_cb,
/// C:                         xg_menu_cb_data *cl_data,
/// C:                         GSList **group)
#[cfg (feature = "use_gtk")]
unsafe fn xg_create_one_menuitem (item: *mut widget_value, f: *mut frame, select_cb: GCallback,
				  highlight_cb: GCallback, cl_data: *mut xg_menu_cb_data,
				  group: *mut *mut GSList) -> *mut GtkWidget
{
  let utf8_label: *mut c_char;                                          // C 3324
  let utf8_key: *mut c_char;                                            // C 3325
  let w: *mut GtkWidget;                                                // C 3326
  let cb_data: *mut xg_menu_item_cb_data;                               // C 3327

  utf8_label = get_utf8_string ((*item).name);                          // C 3329
  utf8_key = get_utf8_string ((*item).key);                             // C 3330

  w = make_menu_item (utf8_label, utf8_key, item, group);               // C 3332

  if ! utf8_label.is_null () { g_free (utf8_label as gpointer); }       // C 3334
  if ! utf8_key.is_null () { g_free (utf8_key as gpointer); }           // C 3335

  cb_data = xmalloc (core::mem::size_of::<xg_menu_item_cb_data> ())     // C 3337
    as *mut xg_menu_item_cb_data;

  xg_list_insert ((&raw mut xg_menu_item_cb_list),
		  (&raw mut (*cb_data).ptrs));                          // C 3339

  (*cb_data).select_id = 0;                                             // C 3341
  (*cb_data).help = (*item).help;                                       // C 3342
  (*cb_data).cl_data = cl_data;                                         // C 3343
  (*cb_data).call_data = (*item).call_data;                             // C 3344

  g_signal_connect! (G_OBJECT! (w), c"destroy".as_ptr (),               // C 3346-3348
		     menuitem_destroy_callback,
		     cb_data as gpointer);                              // C 3349

  /* Put cb_data in widget, so we can get at it when modifying menubar  */   // C 3351
  g_object_set_data (G_OBJECT! (w), XG_ITEM_DATA, cb_data as gpointer);   // C 3352

  /* final item, not a submenu  */                                      // C 3354
  if ! (*item).call_data.is_null () && (*item).contents.is_null ()      // C 3355
    {
      if select_cb.is_some ()                                           // C 3357
	{
	  (*cb_data).select_id                                          // C 3358
	    = g_signal_connect_data (G_OBJECT! (w) as gpointer,
				     c"activate".as_ptr (), select_cb,
				     cb_data as gpointer, None,
				     0 as GConnectFlags);                   // C 3359
	}
    }

  w                                                                     // C 3362
}

/* C 3365's `#ifdef HAVE_PGTK` (its `#endif` is C 3382).  */
/// C 3366-3381: `static gboolean menu_bar_button_pressed_cb (GtkWidget *widget, GdkEvent *event,
/// gpointer user_data)` - `static` in C, but C 3447-3448 takes its address for
/// `g_signal_connect`, so the port defines it `unsafe extern "C" fn` (the callback shape above).
/// C 3370's `struct frame *f = user_data` is C's implicit conversion, C 3372-3373's
/// `event->button.button`/`event->button.window` are the mirror union's `button` member
/// (`include/gtk.rs` 1131-1146), C 3374's `!popup_activated ()` is the port's `== 0` over
/// the int dispextern.h 3879 declares, and C 3376/3377's true are C's own stdbool
/// values (the two declarations take `bool`), and C 3380's `false` is the `gboolean` the
/// function returns.
/// C: static gboolean
/// C: menu_bar_button_pressed_cb (GtkWidget *widget, GdkEvent *event,
/// C: 			    gpointer user_data)
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
unsafe extern "C" fn menu_bar_button_pressed_cb (widget: *mut GtkWidget, event: *mut GdkEvent,
						 user_data: gpointer) -> gboolean
{
  let f: *mut frame = user_data as *mut frame;                          // C 3370

  if (*event).button.button < 4                                         // C 3372
      && (*event).button.window != gtk_widget_get_window (widget)       // C 3373
      && popup_activated () == 0                                        // C 3374
    {
      pgtk_menu_set_in_use (true);                                      // C 3376
      set_frame_menubar (f, true);                                      // C 3377
    }

  FALSE                                                                 // C 3380
}

/// C: gtkutil.h 46: `#define XG_FRAME_DATA "emacs_frame"` - the key C 3459 puts a menu's
/// callback data on its top widget under, and C 4116/4613's the two later reads use.  C's macro
/// is a string literal; this object form is the one the same header's other consumers already
/// use for the two data keys (`src/xmenu.rs` 825, `src/pgtkmenu.rs` 150), and the name is not in
/// this file's own constant inventory.
#[cfg (feature = "use_gtk")]
pub const XG_FRAME_DATA: *const c_char = c"emacs_frame".as_ptr ();

/* C 3384-3402's comment above `create_menus`, carried as C's own text.  */
/* C: /* Create a full menu tree specified by DATA.
   C:    F is the frame the created menu belongs to.
   C:    SELECT_CB is the callback to use when a menu item is selected.
   C:    DEACTIVATE_CB is the callback to use when a sub menu is not shown anymore.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.
   C:    If POP_UP_P, create a popup menu.
   C:    If MENU_BAR_P, create a menu bar.
   C:    TOPMENU is the topmost GtkWidget that others shall be placed under.
   C:    It may be NULL, in that case we create the appropriate widget
   C:    (menu bar or menu item depending on POP_UP_P and MENU_BAR_P)
   C:    CL_DATA is the callback data we shall use for this menu, or NULL
   C:    if we haven't set the first callback yet.
   C:    NAME is the name to give to the top level menu if this function
   C:    creates it.  May be NULL to not set any name.

   C:    Returns the top level GtkWidget.  This is TOPLEVEL if TOPLEVEL is
   C:    not NULL.

   C:    This function calls itself to create submenus.  */  */

/// C 3404-3532: `static GtkWidget *create_menus (widget_value *data, struct frame *f, GCallback
/// select_cb, GCallback deactivate_cb, GCallback highlight_cb, bool pop_up_p, bool menu_bar_p,
/// GtkWidget *topmenu, xg_menu_cb_data *cl_data, const char *name)` - `static` and recursive
/// (C 3513 calls itself), so a plain `unsafe fn` whose own name is in scope.  C 3406-3414's two
/// `bool` parameters are the port's `bool`, C 3418's `= NULL` is the null pointer, C 3440-3444's
/// `#ifndef HAVE_GTK3` pair is the `have_gtk3` feature and C 3446-3449's `#ifdef HAVE_PGTK` the
/// `have_pgtk` one, C 3458's `cl_data = make_cl_data (…)` writes the parameter (the port's
/// `mut`), C 3459/3460's `cl_data` enters a `gpointer` argument by C's conversion,
/// C 3467/3471's `deactivate_cb` is a *variable* - so those two signals stand in the
/// `g_signal_connect!` macro's own expansion, as C 3359's does - C 3477's `for` head is the
/// port's `while` with C's step at the loop's end, C 3481/3482's three tests are the port's null
/// checks and the local `menu_separator_name_p` declaration below, C 3504's
/// `item->contents ? 0 : select_cb` is the port's `if`/`else` over C's null `GCallback`, and
/// C 3513-3522's recursive call passes C's three `0` arguments as the port's `false`, `false`
/// and null pointer plus the null `name`.
/// C: static GtkWidget *
/// C: create_menus (widget_value *data,
/// C:               struct frame *f,
/// C:               GCallback select_cb,
/// C:               GCallback deactivate_cb,
/// C:               GCallback highlight_cb,
/// C:               bool pop_up_p,
/// C:               bool menu_bar_p,
/// C:               GtkWidget *topmenu,
/// C:               xg_menu_cb_data *cl_data,
/// C:               const char *name)
#[cfg (feature = "use_gtk")]
unsafe fn create_menus (data: *mut widget_value, f: *mut frame, select_cb: GCallback,
			deactivate_cb: GCallback, highlight_cb: GCallback, pop_up_p: bool,
			menu_bar_p: bool, topmenu: *mut GtkWidget,
			mut cl_data: *mut xg_menu_cb_data, name: *const c_char) -> *mut GtkWidget
{
  let mut item: *mut widget_value;                                      // C 3416
  let mut wmenu: *mut GtkWidget = topmenu;                              // C 3417
  let mut group: *mut GSList = std::ptr::null_mut ();                   // C 3418

  if topmenu.is_null ()                                                 // C 3420
    {
      if ! menu_bar_p                                                   // C 3422
	{
	  wmenu = gtk_menu_new ();                                      // C 3424
	  xg_set_screen (wmenu, f);                                     // C 3425
	  /* Connect this to the menu instead of items so we get enter/leave for
	     disabled items also.  TODO:  Still does not get enter/leave for
	     disabled items in detached menus.  */                      // C 3426-3428
	  g_signal_connect! (G_OBJECT! (wmenu),                         // C 3429-3431
			     c"enter-notify-event".as_ptr (),
			     menuitem_highlight_callback,
			     std::ptr::null_mut ());                    // C 3432
	  g_signal_connect! (G_OBJECT! (wmenu),                         // C 3433-3435
			     c"leave-notify-event".as_ptr (),
			     menuitem_highlight_callback,
			     std::ptr::null_mut ());                    // C 3436
	}
      else
	{
	  /* C 3440's `#ifndef HAVE_GTK3`: the GTK2 arm's `gtk_menu_bar_new` and the GTK3 arm's
	     `emacs_menu_bar_new` (this file's own, C 182).  */
	  #[cfg (not (feature = "have_gtk3"))]
	  { wmenu = gtk_menu_bar_new (); }                              // C 3441
	  #[cfg (feature = "have_gtk3")]
	  { wmenu = emacs_menu_bar_new (); }                            // C 3443

	  /* C 3446's `#ifdef HAVE_PGTK`.  */
	  #[cfg (feature = "have_pgtk")]
	  { g_signal_connect! (G_OBJECT! (wmenu),
			       c"button-press-event".as_ptr (),        // C 3447
			       menu_bar_button_pressed_cb, f as gpointer); }   // C 3448

	  /* Set width of menu bar to a small value so it doesn't enlarge
	     a small initial frame size.  The width will be set to the
	     width of the frame later on when it is added to a container.
	     height -1: Natural height.  */                             // C 3450-3453
	  gtk_widget_set_size_request (wmenu, 1, -1);                   // C 3454
	}

      /* Put cl_data on the top menu for easier access.  */            // C 3457
      cl_data = make_cl_data (cl_data, f, highlight_cb);                // C 3458
      g_object_set_data (G_OBJECT! (wmenu), XG_FRAME_DATA,              // C 3459
			 cl_data as gpointer);
      g_signal_connect! (G_OBJECT! (wmenu), c"destroy".as_ptr (),       // C 3460
			 menu_destroy_callback, cl_data as gpointer);   // C 3461

      if ! name.is_null ()                                              // C 3463
	{ gtk_widget_set_name (wmenu, name); }                          // C 3464

      /* C 3466-3474: the `#ifndef HAVE_PGTK` pair, whose two arms differ in the signal name
	 and nowhere else; both pass the *variable* `deactivate_cb`, so both stand in the
	 `g_signal_connect!` macro's own expansion and C's final `0` is the null data.  */
      #[cfg (not (feature = "have_pgtk"))]
      {
	if deactivate_cb.is_some ()                                     // C 3467
	  { g_signal_connect_data (G_OBJECT! (wmenu) as gpointer,
				   c"selection-done".as_ptr (), deactivate_cb,
				   std::ptr::null_mut (), None,
				   0 as GConnectFlags); }               // C 3469
      }
      #[cfg (feature = "have_pgtk")]
      {
	if deactivate_cb.is_some ()                                     // C 3471
	  { g_signal_connect_data (G_OBJECT! (wmenu) as gpointer,
				   c"deactivate".as_ptr (), deactivate_cb,
				   std::ptr::null_mut (), None,
				   0 as GConnectFlags); }               // C 3473
      }
    }

  item = data;                                                          // C 3477
  while ! item.is_null ()                                               // C 3477
    {
      let mut w: *mut GtkWidget;                                        // C 3479

      if pop_up_p && (*item).contents.is_null ()                        // C 3481
	  && (*item).call_data.is_null ()                               // C 3481
	  && ! menu_separator_name_p ((*item).name)                     // C 3482
	{
	  let utf8_label: *mut c_char;                                  // C 3484
	  /* A title for a popup.  We do the same as GTK does when
	     creating titles, but it does not look good.  */            // C 3485-3486
	  group = std::ptr::null_mut ();                                // C 3487
	  utf8_label = get_utf8_string ((*item).name);                  // C 3488

	  w = gtk_menu_item_new_with_label (utf8_label);                // C 3490
	  gtk_widget_set_sensitive (w, FALSE);                          // C 3491
	  if ! utf8_label.is_null ()
	    { g_free (utf8_label as gpointer); }                        // C 3492
	}
      else if menu_separator_name_p ((*item).name)                      // C 3494
	{
	  group = std::ptr::null_mut ();                                // C 3496
	  /* GTK only have one separator type.  */                      // C 3497
	  w = gtk_separator_menu_item_new ();                           // C 3498
	}
      else
	{
	  w = xg_create_one_menuitem (item, f,                          // C 3502-3503
				      if ! (*item).contents.is_null ()   // C 3504
				      { select_cb } else { None },
				      highlight_cb, cl_data,
				      (&raw mut group));                 // C 3507

	  /* Create a possibly empty submenu for menu bar items, since some
	     themes don't highlight items correctly without it. */      // C 3509-3510
	  if ! (*item).contents.is_null () || menu_bar_p                 // C 3511
	    {
	      let submenu: *mut GtkWidget
		= create_menus ((*item).contents, f,                    // C 3513
				select_cb, deactivate_cb, highlight_cb,
				false, false, std::ptr::null_mut (),    // C 3518-3520
				cl_data, std::ptr::null ());            // C 3521-3522
	      gtk_menu_item_set_submenu (GTK_MENU_ITEM! (w), submenu);   // C 3523
	    }
	}

      gtk_menu_shell_append (GTK_MENU_SHELL! (wmenu), w);                // C 3527
      gtk_widget_set_name (w, MENU_ITEM_NAME.as_ptr ());                 // C 3528
      item = (*item).next;                                              // C 3477 (the `for` step)
    }

  wmenu                                                                 // C 3531
}

/* The three same-program declarations this slice's two functions call and no mirror carries:
   `menu_separator_name_p` (keyboard.h 473, defined in `keyboard.c` - `src/androidmenu.rs` 91,
   `src/haikumenu.rs` 107 and `src/w32menu.rs` 874 already declare it the same way),
   `popup_activated` (dispextern.h 3879, defined in `keyboard.c` - `src/androidterm.rs` 219's
   shape) and `set_frame_menubar` (frame.h 1881, defined in `xdisp.c` - `src/w32fns.rs` 2645's
   shape).  The ledger's Owed names the three mirror gaps.  */
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn menu_separator_name_p (name: *const c_char) -> bool;           // keyboard.h 473
  pub fn popup_activated () -> c_int;                                  // dispextern.h 3879
  pub fn set_frame_menubar (f: *mut frame, deep_p: bool);               // frame.h 1881
}

/* C 3534-3547's comment above `xg_create_widget`, carried as C's own text.  */
/* C: /* Create a menubar, popup menu or dialog, depending on the TYPE argument.
   C:    TYPE can be "menubar", "popup" for popup menu, or "dialog" for a dialog
   C:    with some text and buttons.
   C:    F is the frame the created item belongs to.
   C:    NAME is the name to use for the top widget.
   C:    VAL is a widget_value structure describing items to be created.
   C:    SELECT_CB is the callback to use when a menu item is selected or
   C:    a dialog button is pressed.
   C:    DEACTIVATE_CB is the callback to use when an item is deactivated.
   C:    For a menu, when a sub menu is not shown anymore, for a dialog it is
   C:    called when the dialog is popped down.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.

   C:    Returns the widget created.  */  */

/// C 3549-3597: `GtkWidget *xg_create_widget (const char *type, const char *name, struct frame
/// *f, widget_value *val, GCallback select_cb, GCallback deactivate_cb, GCallback highlight_cb)`
/// - non-`static` (gtkutil.h 96 declares it and `xmenu.c` calls it), so `#[unsafe(no_mangle)]`
/// with the C ABI; C's `type` parameter is the port's `type_` (Rust's keyword).  C 3555/3556/3558's
/// three `strcmp` tests are `lib/include/string.rs` 30's, the two locals C 3555-3556 holds are
/// the port's `bool` from `== 0`, C 3554's `= 0` is the null pointer, C 3570-3579's recursive
/// call passes C's two `0` arguments as the port's null pointers (topmenu and cl_data) plus the
/// two `bool`s and C's `name`, C 3587's `FRAME_DISPLAY_INFO (f)->xg_cursor` is the mirror's field
/// (`include/xterm.rs` 431) through the macro form this file already uses, and C 3592's
/// `fprintf` is `lib/include/stdio.rs`'s with C's own format string.
/// C: GtkWidget *
/// C: xg_create_widget (const char *type, const char *name, struct frame *f,
/// C: 		  widget_value *val, GCallback select_cb,
/// C: 		  GCallback deactivate_cb, GCallback highlight_cb)
#[cfg (feature = "use_gtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_create_widget (type_: *const c_char, name: *const c_char,
					   f: *mut frame, val: *mut widget_value,
					   select_cb: GCallback, deactivate_cb: GCallback,
					   highlight_cb: GCallback) -> *mut GtkWidget
{
  let mut w: *mut GtkWidget = std::ptr::null_mut ();                    // C 3554
  let menu_bar_p: bool = strcmp (type_, c"menubar".as_ptr ()) == 0;     // C 3555
  let pop_up_p: bool = strcmp (type_, c"popup".as_ptr ()) == 0;         // C 3556

  if strcmp (type_, c"dialog".as_ptr ()) == 0                           // C 3558
    {
      w = create_dialog (val, select_cb, deactivate_cb);                // C 3560
      xg_set_screen (w, f);                                             // C 3561
      gtk_window_set_transient_for (GTK_WINDOW! (w),                    // C 3562
				    GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)));   // C 3563
      gtk_window_set_destroy_with_parent (GTK_WINDOW! (w), TRUE);       // C 3564
      gtk_widget_set_name (w, c"emacs-dialog".as_ptr ());               // C 3565
      gtk_window_set_modal (GTK_WINDOW! (w), TRUE);                     // C 3566
    }
  else if menu_bar_p || pop_up_p                                         // C 3568
    {
      w = create_menus ((*val).contents, f,                             // C 3570
			select_cb, deactivate_cb, highlight_cb,
			pop_up_p, menu_bar_p,                           // C 3575-3576
			std::ptr::null_mut (),                          // C 3577
			std::ptr::null_mut (),                          // C 3578
			name);                                          // C 3579

      /* Set the cursor to an arrow for popup menus when they are mapped.
	 This is done by default for menu bar menus.  */                // C 3581-3582
      if pop_up_p                                                       // C 3583
	{
	  /* Must realize so the GdkWindow inside the widget is created.  */   // C 3585
	  gtk_widget_realize (w);                                       // C 3586
	  xg_set_cursor (w, (*FRAME_DISPLAY_INFO! (f)).xg_cursor);      // C 3587
	}
    }
  else
    {
      fprintf (stderr,
	       c"bad type in xg_create_widget: %s, doing nothing\n".as_ptr (),
	       type_);                                                  // C 3592-3593
    }

  w                                                                     // C 3596
}

/* C 3599's comment above `xg_get_menu_item_label`, carried as C's own text.  */
/* C: /* Return the label for menu item WITEM.  */  */

/// C 3601-3606: `static const char *xg_get_menu_item_label (GtkMenuItem *witem)` - `static` and
/// its address is never taken, so a plain `unsafe fn`.  C 3604's `GTK_LABEL (XG_BIN_CHILD
/// (witem))` is the mirror's cast over this file's own `XG_BIN_CHILD!` (C 127), and C 3605's
/// `gtk_label_get_label` is the mirror declaration this slice adds.
/// C: static const char *
/// C: xg_get_menu_item_label (GtkMenuItem *witem)
#[cfg (feature = "use_gtk")]
unsafe fn xg_get_menu_item_label (witem: *mut GtkMenuItem) -> *const c_char
{
  let wlabel: *mut GtkLabel = GTK_LABEL! (XG_BIN_CHILD! (witem));       // C 3604
  gtk_label_get_label (wlabel)                                          // C 3605
}

/* C 3608's comment above `xg_item_label_same_p`, carried as C's own text.  */
/* C: /* Return true if the menu item WITEM has the text LABEL.  */  */

/// C 3610-3623: `static bool xg_item_label_same_p (GtkMenuItem *witem, const char *label)` -
/// `static`, address never taken, so a plain `unsafe fn`.  C 3614's `witem ? … : 0` is the port's
/// `if`/`else` over the same null test with C's `0` as the null `const char *`, and C 3616-3618's
/// nested conditional is the port's `if`/`else` whose two arms are C's own - the `strcmp` test
/// and the `!utf8_label` fall-through; C 3620's `g_free` takes C's pointer through the port's
/// `*mut c_void`.
/// C: static bool
/// C: xg_item_label_same_p (GtkMenuItem *witem, const char *label)
#[cfg (feature = "use_gtk")]
unsafe fn xg_item_label_same_p (witem: *mut GtkMenuItem, label: *const c_char) -> bool
{
  let utf8_label: *mut c_char = get_utf8_string (label);                // C 3613
  let old_label: *const c_char
    = if ! witem.is_null () { xg_get_menu_item_label (witem) }          // C 3614
      else { std::ptr::null () };

  let is_same: bool
    = if ! old_label.is_null ()                                         // C 3616
      { ! utf8_label.is_null ()                                         // C 3617
	  && strcmp (utf8_label, old_label) == 0 }
      else { utf8_label.is_null () };                                   // C 3618

  if ! utf8_label.is_null () { g_free (utf8_label as gpointer); }       // C 3620

  is_same                                                               // C 3622
}

/* C 3625's comment above `xg_destroy_widgets`, carried as C's own text.  */
/* C: /* Destroy widgets in LIST.  */  */

/// C 3627-3639: `static void xg_destroy_widgets (GList *list)` - `static`, address never taken,
/// so a plain `unsafe fn`.  C 3632's `for` head is the port's `while` with C's `g_list_next`
/// step at the loop's end (PORTING.md 2.2), C 3634's `GTK_WIDGET (iter->data)` is the mirror's
/// cast macro over the `gpointer` member (`include/gtk.rs` 68), and C 3637's `gtk_widget_destroy`
/// is the mirror entry point the other destroy sites use.
/// C: static void
/// C: xg_destroy_widgets (GList *list)
#[cfg (feature = "use_gtk")]
unsafe fn xg_destroy_widgets (list: *mut GList)
{
  let mut iter: *mut GList;                                             // C 3630

  iter = list;                                                          // C 3632
  while ! iter.is_null ()                                               // C 3632
    {
      let w: *mut GtkWidget = GTK_WIDGET! ((*iter).data);               // C 3634

      /* Destroying the widget will remove it from the container it is in.  */   // C 3636
      gtk_widget_destroy (w);                                           // C 3637
      iter = g_list_next! (iter);                                       // C 3632 (the `for` step)
    }
}

/* C 3641-3651's comment above `xg_update_menubar`, carried as C's own text.  */
/* C: /* Update the top level names in MENUBAR (i.e. not submenus).
   C:    F is the frame the menu bar belongs to.
   C:    *LIST is a list with the current menu bar names (menu item widgets).
   C:    ITER is the item within *LIST that shall be updated.
   C:    POS is the numerical position, starting at 0, of ITER in *LIST.
   C:    VAL describes what the menu bar shall look like after the update.
   C:    SELECT_CB is the callback to use when a menu item is selected.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.
   C:    CL_DATA points to the callback data to be used for this menu bar.

   C:    This function calls itself to walk through the menu bar names.  */  */

/// C 3653-3830: `static void xg_update_menubar (GtkWidget *menubar, struct frame *f, GList **list,
/// GList *iter, int pos, widget_value *val, GCallback select_cb, GCallback deactivate_cb,
/// GCallback highlight_cb, xg_menu_cb_data *cl_data)` - `static` and recursive (C 3828 calls
/// itself), so a plain `unsafe fn`; C writes `iter`, `pos` and `val` (C 3677/3678/3694-3696 and
/// the four branches below), which is the port's `mut`.  C 3665/3667/3680/3691's four tests are
/// the port's null checks and the slice-8 `xg_item_label_same_p`; C 3674's `""` is the port's
/// one-element `c""`; C 3684's `0, 1, menubar, cl_data, 0` is `false, true, menubar, cl_data` and
/// the null `name`; C 3702/3703's two `bool` locals are the port's `bool` from C's `0`;
/// C 3708-3710 and C 3718's `for` heads are the port's `while` with C's update step at the
/// loop's end (PORTING.md 2.2); C 3737/3799/3820's `while (nr-- > 0)` keeps C's post-decrement
/// inside the condition, because that decrement happens on the test that fails too (the
/// `src/xfns.rs` 4526-4528 shape); C 3730/3812's `g_object_ref` and C 3816's `g_object_unref`
/// take C's `GObject *` through the port's `gpointer`; and C 3736/3798/3819's chained
/// `*list = iter = …` is two stores of one call, which the port writes as the store and a read
/// of it back, since Rust has no chained assignment.
/// C: static void
/// C: xg_update_menubar (GtkWidget *menubar,
/// C: 		   struct frame *f,
/// C: 		   GList **list,
/// C: 		   GList *iter,
/// C: 		   int pos,
/// C: 		   widget_value *val,
/// C: 		   GCallback select_cb,
/// C: 		   GCallback deactivate_cb,
/// C: 		   GCallback highlight_cb,
/// C: 		   xg_menu_cb_data *cl_data)
#[cfg (feature = "use_gtk")]
unsafe fn xg_update_menubar (menubar: *mut GtkWidget, f: *mut frame, list: *mut *mut GList,
			     mut iter: *mut GList, mut pos: c_int,
			     mut val: *mut widget_value, select_cb: GCallback,
			     deactivate_cb: GCallback, highlight_cb: GCallback,
			     cl_data: *mut xg_menu_cb_data)
{
  if iter.is_null () && val.is_null ()                                  // C 3665
    { return; }                                                         // C 3666
  else if ! iter.is_null () && val.is_null ()                           // C 3667
    {
      /* Item(s) have been removed.  Remove all remaining items.  */     // C 3669
      xg_destroy_widgets (iter);                                        // C 3670

      /* Add a blank entry so the menubar doesn't collapse to nothing. */   // C 3672
      gtk_menu_shell_insert (GTK_MENU_SHELL! (menubar),                 // C 3673
			     gtk_menu_item_new_with_label (c"".as_ptr ()),   // C 3674
			     0);                                        // C 3675
      /* All updated.  */                                               // C 3676
      val = std::ptr::null_mut ();                                      // C 3677
      iter = std::ptr::null_mut ();                                     // C 3678
    }
  else if iter.is_null () && ! val.is_null ()                           // C 3680
    {
      /* Item(s) added.  Add all new items in one call.  */             // C 3682
      create_menus (val, f, select_cb, deactivate_cb, highlight_cb,     // C 3683
		    false, true, menubar, cl_data, std::ptr::null ());  // C 3684

      /* All updated.  */                                               // C 3686
      val = std::ptr::null_mut ();                                      // C 3687
      iter = std::ptr::null_mut ();                                     // C 3688
    }
  /* Below this neither iter or val is NULL */                          // C 3690
  else if xg_item_label_same_p (GTK_MENU_ITEM! ((*iter).data),
				(*val).name)                            // C 3691
    {
      /* This item is still the same, check next item.  */              // C 3693
      val = (*val).next;                                                // C 3694
      iter = g_list_next! (iter);                                       // C 3695
      pos += 1;                                                         // C 3696
    }
  else /* This item is changed.  */                                     // C 3698
    {
      let witem: *mut GtkMenuItem = GTK_MENU_ITEM! ((*iter).data);      // C 3700
      let mut witem2: *mut GtkMenuItem = std::ptr::null_mut ();         // C 3701
      let mut val_in_menubar: bool = false;                             // C 3702
      let mut iter_in_new_menubar: bool = false;                        // C 3703
      let mut iter2: *mut GList;                                        // C 3704
      let mut cur: *mut widget_value;                                   // C 3705

      /* See if the changed entry (val) is present later in the menu bar  */   // C 3707
      iter2 = iter;                                                     // C 3708
      while ! iter2.is_null () && ! val_in_menubar                      // C 3709
	{
	  witem2 = GTK_MENU_ITEM! ((*iter2).data);                      // C 3712
	  val_in_menubar = xg_item_label_same_p (witem2, (*val).name);   // C 3713
	  iter2 = g_list_next! (iter2);                                 // C 3710 (the `for` step)
	}

      /* See if the current entry (iter) is present later in the
	 specification for the new menu bar.  */                        // C 3716-3717
      cur = val;                                                        // C 3718
      while ! cur.is_null () && ! iter_in_new_menubar                   // C 3718
	{
	  iter_in_new_menubar = xg_item_label_same_p (witem, (*cur).name);   // C 3719
	  cur = (*cur).next;                                            // C 3718 (the `for` step)
	}

      if val_in_menubar && ! iter_in_new_menubar                        // C 3721
	{
	  let mut nr: c_int = pos;                                      // C 3723

	  /*  This corresponds to:
	        Current:  A B C
	        New:      A C
	      Remove B.  */                                             // C 3725-3728

	  g_object_ref (G_OBJECT! (witem) as gpointer);                 // C 3730
	  gtk_container_remove (GTK_CONTAINER! (menubar), GTK_WIDGET! (witem));   // C 3731
	  gtk_widget_destroy (GTK_WIDGET! (witem));                     // C 3732

	  /* Must get new list since the old changed.  */               // C 3734
	  g_list_free (*list);                                          // C 3735
	  *list = gtk_container_get_children (GTK_CONTAINER! (menubar));   // C 3736
	  iter = *list;
	  while { nr -= 1; nr + 1 > 0 }                                 // C 3737
	    { iter = g_list_next! (iter); }
	}
      else if ! val_in_menubar && ! iter_in_new_menubar                 // C 3739
	{
	  /*  This corresponds to:
	        Current:  A B C
	        New:      A X C
	      Rename B to X.  This might seem to be a strange thing to do,
	      since if there is a menu under B it will be totally wrong for X.
	      But consider editing a C file.  Then there is a C-mode menu
	      (corresponds to B above).
	      If then doing C-x C-f the minibuf menu (X above) replaces the
	      C-mode menu.  When returning from the minibuffer, we get
	      back the C-mode menu.  Thus we do:
	        Rename B to X (C-mode to minibuf menu)
	        Rename X to B (minibuf to C-mode menu).
	      If the X menu hasn't been invoked, the menu under B
	      is up to date when leaving the minibuffer.  */            // C 3741-3754
	  let wlabel: *mut GtkLabel = GTK_LABEL! (XG_BIN_CHILD! (witem));   // C 3755
	  let utf8_label: *mut c_char = get_utf8_string ((*val).name);  // C 3756

	  /* GTK menu items don't notice when their labels have been
	     changed from underneath them, so we have to explicitly
	     use g_object_notify to tell listeners (e.g., a GMenuModel
	     bridge that might be loaded) that the item's label has
	     changed.  */                                              // C 3758-3762
	  gtk_label_set_text (wlabel, utf8_label);                      // C 3763
	  g_object_notify (G_OBJECT! (witem), c"label".as_ptr ());      // C 3764
	  if ! utf8_label.is_null ()
	    { g_free (utf8_label as gpointer); }                        // C 3765
	  iter = g_list_next! (iter);                                   // C 3766
	  val = (*val).next;                                            // C 3767
	  pos += 1;                                                     // C 3768
	}
      else if ! val_in_menubar && iter_in_new_menubar                   // C 3770
	{
	  /*  This corresponds to:
	        Current:  A B C
	        New:      A X B C
	      Insert X.  */                                             // C 3772-3775

	  let mut nr: c_int = pos;                                      // C 3777
	  let mut group: *mut GSList = std::ptr::null_mut ();           // C 3778
	  let w: *mut GtkWidget
	    = xg_create_one_menuitem (val,                              // C 3779
				      f,
				      select_cb,
				      highlight_cb,
				      cl_data,
				      (&raw mut group));                // C 3784

	  /* Create a possibly empty submenu for menu bar items, since some
	     themes don't highlight items correctly without it. */     // C 3786-3787
	  let submenu: *mut GtkWidget
	    = create_menus (std::ptr::null_mut (), f,                   // C 3788
			    select_cb, deactivate_cb,
			    highlight_cb,
			    false, false, std::ptr::null_mut (),
			    cl_data, std::ptr::null ());            // C 3791

	  gtk_widget_set_name (w, MENU_ITEM_NAME.as_ptr ());            // C 3793
	  gtk_menu_shell_insert (GTK_MENU_SHELL! (menubar), w, pos);    // C 3794
	  gtk_menu_item_set_submenu (GTK_MENU_ITEM! (w), submenu);      // C 3795

	  g_list_free (*list);                                          // C 3797
	  *list = gtk_container_get_children (GTK_CONTAINER! (menubar));   // C 3798
	  iter = *list;
	  while { nr -= 1; nr + 1 > 0 }                                 // C 3799
	    { iter = g_list_next! (iter); }
	  iter = g_list_next! (iter);                                   // C 3800
	  val = (*val).next;                                            // C 3801
	  pos += 1;                                                     // C 3802
	}
      else /* if (val_in_menubar && iter_in_new_menubar) */             // C 3804
	{
	  let mut nr: c_int = pos;                                      // C 3806
	  /*  This corresponds to:
	        Current:  A B C
	        New:      A C B
	      Move C before B  */                                       // C 3807-3810

	  g_object_ref (G_OBJECT! (witem2) as gpointer);                // C 3812
	  gtk_container_remove (GTK_CONTAINER! (menubar), GTK_WIDGET! (witem2));   // C 3813
	  gtk_menu_shell_insert (GTK_MENU_SHELL! (menubar),             // C 3814
				 GTK_WIDGET! (witem2), pos);            // C 3815
	  g_object_unref (G_OBJECT! (witem2) as gpointer);              // C 3816

	  g_list_free (*list);                                          // C 3818
	  *list = gtk_container_get_children (GTK_CONTAINER! (menubar));   // C 3819
	  iter = *list;
	  while { nr -= 1; nr + 1 > 0 }                                 // C 3820
	    { iter = g_list_next! (iter); }
	  if ! iter.is_null () { iter = g_list_next! (iter); }          // C 3821
	  val = (*val).next;                                            // C 3822
	  pos += 1;                                                     // C 3823
	}
    }

  /* Update the rest of the menu bar.  */                               // C 3827
  xg_update_menubar (menubar, f, list, iter, pos, val,                  // C 3828
		     select_cb, deactivate_cb, highlight_cb, cl_data);  // C 3829
}

/* C 3832-3835's comment above `xg_update_menu_item`, carried as C's own text.  */
/* C: /* Update the menu item W so it corresponds to VAL.
   C:    SELECT_CB is the callback to use when a menu item is selected.
   C:    HIGHLIGHT_CB is the callback to call when entering/leaving menu items.
   C:    CL_DATA is the data to set in the widget for menu invocation.  */  */

/// C 3837-3945: `static void xg_update_menu_item (widget_value *val, GtkWidget *w, GCallback
/// select_cb, GCallback highlight_cb, xg_menu_cb_data *cl_data)` - `static` and its address is
/// never taken, so a plain `unsafe fn`.  C 3845-3850's five pointer locals are the port's nulls
/// (`GtkLabel *wlbl = 0`, the two `const char *` and C 3852's `false`), C 3859's `GTK_IS_BOX` is
/// the mirror declaration this slice adds, C 3863-3864/3888-3889's `list->data`/
/// `list->next->data` reads are the `GList` members (`include/gtk.rs` 68-70),
/// C 3870/3873's `g_object_ref`/`g_object_unref` take C's `GObject *` through the port's
/// `gpointer`, C 3897-3898's `gtk_label_get_label` is slice 8's mirror entry point, C 3900/3906's
/// two comparisons are C's own `strcmp` tests with the port's null checks for C's `! old_key`/
/// `! old_label`, C 3915/3917's `gtk_widget_get_sensitive` is this slice's second mirror
/// declaration and its `gboolean` result is the port's `!= 0`/`== 0`, C 3920's
/// `g_object_get_data` result takes C's conversion, C 3931/3936's `select_id` tests are the
/// port's `== 0`/`!= 0` over a `gulong`, and C 3933-3934's `g_signal_connect` passes the
/// *variable* `select_cb` - so it stands in the `g_signal_connect!` macro's own
/// `g_signal_connect_data` expansion, as C 3359's and C 3469's do.
/// C: static void
/// C: xg_update_menu_item (widget_value *val,
/// C:                      GtkWidget *w,
/// C:                      GCallback select_cb,
/// C:                      GCallback highlight_cb,
/// C:                      xg_menu_cb_data *cl_data)
#[cfg (feature = "use_gtk")]
unsafe fn xg_update_menu_item (val: *mut widget_value, w: *mut GtkWidget, select_cb: GCallback,
			       highlight_cb: GCallback, cl_data: *mut xg_menu_cb_data)
{
  let wchild: *mut GtkWidget;                                           // C 3844
  let mut wlbl: *mut GtkLabel = std::ptr::null_mut ();                  // C 3845
  let mut wkey: *mut GtkLabel = std::ptr::null_mut ();                  // C 3846
  let utf8_label: *mut c_char;                                          // C 3847
  let utf8_key: *mut c_char;                                            // C 3848
  let mut old_label: *const c_char = std::ptr::null ();                 // C 3849
  let mut old_key: *const c_char = std::ptr::null ();                   // C 3850
  let cb_data: *mut xg_menu_item_cb_data;                               // C 3851
  let mut label_changed: bool = false;                                  // C 3852

  wchild = XG_BIN_CHILD! (w);                                           // C 3854
  utf8_label = get_utf8_string ((*val).name);                           // C 3855
  utf8_key = get_utf8_string ((*val).key);                              // C 3856

  /* See if W is a menu item with a key.  See make_menu_item above.  */  // C 3858
  if GTK_IS_BOX! (wchild)                                               // C 3859
    {
      let list: *mut GList = gtk_container_get_children (GTK_CONTAINER! (wchild));   // C 3861

      wlbl = GTK_LABEL! ((*list).data);                                 // C 3863
      wkey = GTK_LABEL! ((*(*list).next).data);                         // C 3864
      g_list_free (list);                                               // C 3865

      if utf8_key.is_null ()                                            // C 3867
	{
	  /* Remove the key and keep just the label.  */                // C 3869
	  g_object_ref (G_OBJECT! (wlbl) as gpointer);                  // C 3870
	  gtk_container_remove (GTK_CONTAINER! (w), wchild);            // C 3871
	  gtk_container_add (GTK_CONTAINER! (w), GTK_WIDGET! (wlbl));   // C 3872
	  g_object_unref (G_OBJECT! (wlbl) as gpointer);                // C 3873
	  wkey = std::ptr::null_mut ();                                 // C 3874
	}

    }
  else /* Just a label.  */                                             // C 3878
    {
      wlbl = GTK_LABEL! (wchild);                                       // C 3880

      /* Check if there is now a key.  */                               // C 3882
      if ! utf8_key.is_null ()                                          // C 3883
	{
	  let wtoadd: *mut GtkWidget
	    = make_widget_for_menu_item (utf8_label, utf8_key);         // C 3885
	  let list: *mut GList = gtk_container_get_children (GTK_CONTAINER! (wtoadd));   // C 3886

	  wlbl = GTK_LABEL! ((*list).data);                             // C 3888
	  wkey = GTK_LABEL! ((*(*list).next).data);                     // C 3889
	  g_list_free (list);                                           // C 3890

	  gtk_container_remove (GTK_CONTAINER! (w), wchild);            // C 3892
	  gtk_container_add (GTK_CONTAINER! (w), wtoadd);               // C 3893
	}
    }

  if ! wkey.is_null () { old_key = gtk_label_get_label (wkey); }        // C 3897
  if ! wlbl.is_null () { old_label = gtk_label_get_label (wlbl); }      // C 3898

  if ! wkey.is_null () && ! utf8_key.is_null ()                         // C 3900
      && (old_key.is_null () || strcmp (utf8_key, old_key) != 0)
    {
      label_changed = true;                                             // C 3902
      gtk_label_set_text (wkey, utf8_key);                              // C 3903
    }

  if ! utf8_label.is_null ()                                            // C 3906
      && (old_label.is_null () || strcmp (utf8_label, old_label) != 0)
    {
      label_changed = true;                                             // C 3908
      gtk_label_set_text (wlbl, utf8_label);                            // C 3909
    }

  if ! utf8_key.is_null () { g_free (utf8_key as gpointer); }           // C 3912
  if ! utf8_label.is_null () { g_free (utf8_label as gpointer); }       // C 3913

  if ! (*val).enabled && gtk_widget_get_sensitive (w) != 0              // C 3915
    { gtk_widget_set_sensitive (w, FALSE); }                            // C 3916
  else if (*val).enabled && gtk_widget_get_sensitive (w) == 0           // C 3917
    { gtk_widget_set_sensitive (w, TRUE); }                             // C 3918

  cb_data = g_object_get_data (G_OBJECT! (w), XG_ITEM_DATA)             // C 3920
    as *mut xg_menu_item_cb_data;
  if ! cb_data.is_null ()                                               // C 3921
    {
      (*cb_data).call_data = (*val).call_data;                          // C 3923
      (*cb_data).help = (*val).help;                                    // C 3924
      (*cb_data).cl_data = cl_data;                                     // C 3925

      /* We assume the callback functions don't change.  */             // C 3927
      if ! (*val).call_data.is_null () && (*val).contents.is_null ()    // C 3928
	{
	  /* This item shall have a select callback.  */                // C 3930
	  if (*cb_data).select_id == 0                                  // C 3931
	    {
	      (*cb_data).select_id                                      // C 3932
		= g_signal_connect_data (G_OBJECT! (w) as gpointer,
					 c"activate".as_ptr (), select_cb,
					 cb_data as gpointer, None,
					 0 as GConnectFlags);               // C 3933-3934
	    }
	}
      else if (*cb_data).select_id != 0                                 // C 3936
	{
	  g_signal_handler_disconnect (w as gpointer, (*cb_data).select_id);   // C 3938
	  (*cb_data).select_id = 0;                                     // C 3939
	}
    }

  if label_changed /* See comment in xg_update_menubar.  */             // C 3943
    { g_object_notify (G_OBJECT! (w), c"label".as_ptr ()); }             // C 3944
}

/* C 3947's comment above `xg_update_toggle_item`, carried as C's own text.  */
/* C: /* Update the toggle menu item W so it corresponds to VAL.  */  */

/// C 3949-3953: `static void xg_update_toggle_item (widget_value *val, GtkWidget *w)` -
/// `static`, address never taken, so a plain `unsafe fn`.  C 3952's `val->selected` is the
/// `widget_value` mirror's `bool` and enters the `gboolean` parameter as `as gboolean` (C's own
/// usual conversion, the `xg_create_widget` row's class), and `GTK_CHECK_MENU_ITEM` is the
/// unit's own C 220 macro.
/// C: static void
/// C: xg_update_toggle_item (widget_value *val, GtkWidget *w)
#[cfg (feature = "use_gtk")]
unsafe fn xg_update_toggle_item (val: *mut widget_value, w: *mut GtkWidget)
{
  gtk_check_menu_item_set_active (GTK_CHECK_MENU_ITEM! (w),
				  (*val).selected as gboolean);         // C 3952
}

/* C 3955's comment above `xg_update_radio_item`, carried as C's own text.  */
/* C: /* Update the radio menu item W so it corresponds to VAL.  */  */

/// C 3957-3961: `static void xg_update_radio_item (widget_value *val, GtkWidget *w)` - C's own
/// text is the toggle function above with the comment changed, which the port carries as it
/// stands (the two are separate definitions in C, so they are separate `unsafe fn`s here).
/// C: static void
/// C: xg_update_radio_item (widget_value *val, GtkWidget *w)
#[cfg (feature = "use_gtk")]
unsafe fn xg_update_radio_item (val: *mut widget_value, w: *mut GtkWidget)
{
  gtk_check_menu_item_set_active (GTK_CHECK_MENU_ITEM! (w),
				  (*val).selected as gboolean);         // C 3960
}
