// femacs: port of src/xmenu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md
// 0.2); the C `static` helpers stay plain `unsafe fn`s, except the ones C hands to another function
// as a value (`pop_down_menu`, `leave_toolkit_menu`, `cleanup_widget_value_tree`,
// `x_menu_wait_for_event`, `x_menu_dispatch_event`, `x_menu_translate_generic_event`,
// `menu_help_callback`, `server_timestamp_predicate`, `popup_selection_callback`,
// `popup_deactivate_callback`, `menu_highlight_callback`, `menubar_selection_callback`,
// `dialog_selection_callback`, `popup_activate_callback`), which are spelled `unsafe extern "C" fn`
// (PORTING.md P-12: one definition per linkage unit, which is what C's `static` gives too).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `xmenu.o`
// sits in `XMENU_OBJ` (`emacs/src/Makefile.in` 211-212: "## xmenu.o if HAVE_X_WINDOWS, else
// empty."), which `configure.ac` 7189-7195 fills with `xmenu.o` only under
// `test "${HAVE_X_WINDOWS}" = "yes"` (the same line `AC_DEFINE`s `HAVE_X_WINDOWS`, 7193-7194), and
// `docs/baseline/frozen-build-sources.txt` (106 sources, this target's `make -n`) lists no
// `xmenu.o`.  P-86 ① therefore applies: this is *transcription*, both arms of every `#if` are
// carried, and
//
//   * the **file gate** is `feature = "have_x_windows"`, on every item *and on the include graph*
//     (P-86 ⑧: with the includes outside the gate the frozen compile eats `E0428`s), which is why
//     the frozen configuration compiles nothing here and `build-tu` measures a 336-byte empty
//     object;
//   * the file's own conditions are carried as their own features *inside* the file gate, **both
//     arms each**: `use_x_toolkit` (C 75/120/158/311/326/353/377/477/492/534/667/931/947/1197/
//     1222/1233/1255/1281/1286/1302/1316/1332/1354/1393/1600/1801/1825/1847/1858/1873/2167/2238/
//     2911/2915/2927), `use_gtk` (C 95/101/103/147/189/219/230/296/578/654/663/699/713/747/782/
//     830/855/1153/1179/1250/1367/1393/1600/2167/2211/2267/2911/2915), `use_lucid` (C 82/90/504/
//     1197/1222/1233/1354? /1761/1850/2238 — the `USE_LUCID` arms), `use_motif` (C 91/147/353/415/
//     534/1286/1302/1316/1332/1354/1723/1754/1825), `have_gtk3` (C 103/219/1421/1429/1460/1510/
//     1533/1556), `have_xinput2` (C 54/113/230/326/377/477/492/522/667/1620/1685/1723/1754/1801/
//     1847/1858/1873/2519/2542/2778/2790), `have_xinput2_4` (C 1685), `have_xaw3d` (C 85),
//     `msdos` (C 50/145/306/2525/2768/2881), and `not (feature = "have_multilingual_menu")` for
//     C 1961/2011/2073's `#ifndef HAVE_MULTILINGUAL_MENU` (the file `#undef`s it at C 74 itself, so
//     C's `#ifndef` arm is the live one in every configuration and the `#else` arm is empty - both
//     are carried);
//   * `tools/check-cond-eval.py src/xmenu.c` is the second reading of those same conditions
//     (see the ledger's Evidence ⑬).
//
// **Acquisition: this piece is partly `text-only` (P-86 ③⑵).**  The machine has libX11 and
// xorgproto under `/opt/homebrew/include` (`Xlib.h`, `X.h`, `Xatom.h`, `keysym.h`, and
// `extensions/XI2.h`), so the Xlib/Xproto/XI2-constant surface below is **measured** (clang
// readings; the probe and its output are the ledger's Evidence ②).  It has **no** libXt
// (`X11/Intrinsic.h`, `X11/Shell.h`, `X11/StringDefs.h`), **no** Xaw (`X11/Xaw/Paned.h`), **no**
// Xaw3d, **no** Motif, **no** GTK3 and **no** libXi (`X11/extensions/XInput2.h`): the `use_x_toolkit`
// and `use_gtk` arms therefore name a surface this machine cannot compile or measure, and the
// declarations below for those headers are `UNVERIFIED` (documented, not measured) - which is the
// `## Not claimed` line of this file's ledger.  What the round does carry is the frozen `build-tu`
// reading, the machine-checked coverage table, the `#if` verdicts, a per-item C citation, the
// measured Xlib/XI2 values, and the `probe` reading of the frozen and one flipped arm.
//
// Names the mirrors this unit includes do not carry are declared at the use site with the reference
// header's line - one citation per line (P-88 ①).  The reference-tree headers of this family
// (`src/xterm.h`, `src/widget.h`, `src/menu.h`, `lwlib/lwlib.h`, `lwlib/xlwmenu.h`,
// `oldXMenu/XMenu.h`) all have mirrors in this tree and are included below at C's own lines; the
// SDK headers (`<X11/...>`, `<gtk/gtk.h>`) have none, so their surface stands in the section after
// the include graph.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

/* C 1-23 is the reference's opening block (its one-line title and the three authors' names),
   dropped (PORTING.md 0.1-19): the port's first line is the revision line above and no line of
   that block is copied here.
   C 25-32 is the file's own prose - a one-line title comment (dropped with the block above) and
   two notes on the file's history, carried where they stand:

     the file's title comment;
     "Modified by Fred Pierresteguy on December 93 to make the popup menus and menubar use the
      Xt.";
     "Rewritten for clarity and GC protection by rms in Feb 94."

   */

include!("include/config.rs");                 // C 34: `#include <config.h>` (every unit includes it)

/* C 36: `#include <stdio.h>`.  The file reaches no name from it directly (`fprintf` sits in C
   2821's `#ifdef XDEBUG` arm), so the edge is carried under the file gate for C's text's sake.  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdio.rs");           // C 36: `#include <stdio.h>`

/* C 38-48: the C headers.  `xmenu.o` is built only by an X configuration, so the whole graph sits
   under the file gate (P-86 ⑧).  C 42's `termhooks.h` and C 43's `window.h` reach
   `dispextern.h`/`lisp.h` through their own bodies; the mirrors are landed.  */
#[cfg (feature = "have_x_windows")]
include!("include/lisp.rs");                   // C 38: `#include "lisp.h"`
#[cfg (feature = "have_x_windows")]
include!("include/keyboard.rs");               // C 39: `#include "keyboard.h"`
#[cfg (feature = "have_x_windows")]
include!("include/frame.rs");                  // C 40: `#include "frame.h"`
#[cfg (feature = "have_x_windows")]
include!("include/systime.rs");                // C 41: `#include "systime.h"`
#[cfg (feature = "have_x_windows")]
include!("include/termhooks.rs");              // C 42: `#include "termhooks.h"`
#[cfg (feature = "have_x_windows")]
include!("include/window.rs");                 // C 43: `#include "window.h"`
#[cfg (feature = "have_x_windows")]
include!("include/blockinput.rs");             // C 44: `#include "blockinput.h"`
#[cfg (feature = "have_x_windows")]
include!("include/buffer.rs");                 // C 45: `#include "buffer.h"`
#[cfg (feature = "have_x_windows")]
include!("include/coding.rs");                 // C 46: `#include "coding.h"`
#[cfg (feature = "have_x_windows")]
include!("include/sysselect.rs");              // C 47: `#include "sysselect.h"`
#[cfg (feature = "have_x_windows")]
include!("include/pdumper.rs");                // C 48: `#include "pdumper.h"`

/* C 50-52: `#ifdef MSDOS` / `#include "msdos.h"` / `#endif`.  Both arms are carried; the `#else`
   arm is empty.  (`MSDOS` is a *sub*-condition here, not the file gate: `xmenu.o` is not in
   `MSDOS_OBJ` either.)  */
#[cfg (all (feature = "have_x_windows", feature = "msdos"))]
include!("include/msdos.rs");                  // C 51: `#include "msdos.h"`

/* C 54-57: `#ifdef HAVE_XINPUT2` / `#include <math.h>` / `#include <X11/extensions/XInput2.h>` /
   `#endif`.  `<math.h>` has a mirror (`lrint`, C 258/394/...) and `<X11/extensions/XInput2.h>`
   (libXi) has none: the names C takes from it stand in the SDK section below.  Both arms are
   carried; the `#else` arm is empty.  */
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
include!("../lib/include/math.rs");            // C 55: `#include <math.h>`

/* C 59-63: `#ifdef HAVE_X_WINDOWS` / the "This may include sys/types.h" comment / `xterm.h` /
   `#endif`.  The include is the reference's own header, whose mirror is landed.  */
#[cfg (feature = "have_x_windows")]
include!("include/xterm.rs");                  // C 62: `#include "xterm.h"`

/* C 65-69: "Load sys/types.h if not already loaded.  In some systems loading it twice is
   suicidal."  and C 67's `#ifndef makedev` / `#include <sys/types.h>` / `#endif`.  `makedev` is in
   the frozen macro table, so C's arm is the dead one there; the `#else` arm is empty.  Both arms
   are carried (the `src/xselect.rs` 80 edge).  */
#[cfg (all (feature = "have_x_windows", not (feature = "makedev")))]
include!("../lib/include/sys_types.in.rs");    // C 68: `#include <sys/types.h>`

/* C 71-99: `#ifdef HAVE_X_WINDOWS` and the toolkit include chain inside it.  C 72-74's comment and
   `#undef HAVE_MULTILINGUAL_MENU` are carried where they stand, in C's words:

     /*  Defining HAVE_MULTILINGUAL_MENU would mean that the toolkit menu
         code accepts the Emacs internal encoding.  */
     #undef HAVE_MULTILINGUAL_MENU

   `#undef` has no Rust counterpart and needs none: the macro is not a feature this unit sets, and
   every C 1961/2011/2073 `#ifndef` reads it as unset.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
include!("include/widget.rs");                 // C 76: `#include "widget.h"`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
include!("include/xlib.rs");                   // C 77: `#include <X11/Xlib.h>`
/* C 78-81: `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`, `<X11/StringDefs.h>`, `<X11/Shell.h>` - libXt and
   Xaw headers this machine has not got and this tree has no mirror for; the names C takes from them
   stand in the SDK section below.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
include!("include/xsettings.rs");              // C 83: `#include "xsettings.h"`
/* C reaches `lwlib-widget.h` through three chains - `xlwmenu.h` -> `lwlib.h` 43 (this line),
   `gtkutil.h` 27 (C 102) and `menu.h` 22 (C 108) - and C's include guard makes whichever chain
   the *live* configuration takes the first.  `xtask`'s materializer follows every `include!`
   textually and keeps the first occurrence, so the edge is placed once, at the earliest C
   position, under the file gate (which is the only gate that covers all three arms); the
   ledger's `## Deviations` records the shape.  */
#[cfg (feature = "have_x_windows")]
include!("../lwlib/include/lwlib-widget.rs");  // C 84 (-> `lwlib.h` 43) / C 102 / C 108: `#include "../lwlib/lwlib-widget.h"`
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
include!("../lwlib/include/xlwmenu.rs");       // C 84: `#include "../lwlib/xlwmenu.h"`
/* C 85-89: `#ifdef HAVE_XAW3D` / `#include <X11/Xaw3d/Paned.h>` / `#else` / `#include
   <X11/Xaw/Paned.h>` / `#endif`.  Both arms are carried; neither header is on this machine and
   neither has a mirror, so neither arm names a name this unit reads (C reaches `Paned.h`'s
   `XtNinternalBorderWidth` indirectly, through the resource name, which is Xt's).  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_motif"))]
include!("../lwlib/include/lwlib.rs");         // C 92: `#include "../lwlib/lwlib.h"`
/* C 94-98: `#else /* not USE_X_TOOLKIT */` / `#ifndef USE_GTK` / `#include "../oldXMenu/XMenu.h"` /
   `#endif` / `#endif /* not USE_X_TOOLKIT */`.  Both arms of both conditions are carried; the
   reference's own `oldXMenu/XMenu.h` has a mirror, which is where the XMenu surface comes from.  */
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]
include!("../oldXMenu/include/XMenu.rs");      // C 96: `#include "../oldXMenu/XMenu.h"`

/* C 101-106: `#ifdef USE_GTK` / `#include "gtkutil.h"` / `#ifdef HAVE_GTK3` / `#include
   "xgselect.h"` / `#endif` / `#endif`.  Both arms of both conditions are carried; both headers are
   the reference's own and both mirrors are landed.  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
include!("include/gtkutil.rs");                // C 102: `#include "gtkutil.h"`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
include!("include/gtk.rs");                    // C 102 -> `gtkutil.h` 26: `#include <gtk/gtk.h>`
#[cfg (all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3"))]
include!("include/xgselect.rs");               // C 104: `#include "xgselect.h"`

/* C 108: `#include "menu.h"` - ungated in C, carried here under the file gate like the rest of the
   graph.  `menu.h` 22 reaches `../lwlib/lwlib-widget.h`, which is where `widget_value` and the
   `BUTTON_TYPE_*` members come from, so its mirror is included at that position (the
   `src/w32menu.rs` 79 edge).  */
#[cfg (feature = "have_x_windows")]
include!("include/menu.rs");                   // C 108: `#include "menu.h"`
#[cfg (feature = "have_x_windows")]
include!("../lwlib/include/lwlib-widget.rs");  // C 108 -> `menu.h` 22: `#include "../lwlib/lwlib-widget.h"`

/* ==============================================================================================
   C 77/C 78-81/C 102: the SDK surfaces this unit names and no mirror carries (PORTING.md
   P-88 ①: a unit-local segment, *not* a file under `femacs/src/include/`).

   (a) `<X11/Xlib.h>`, `<X11/X.h>`, `<X11/Xatom.h>` and `<X11/keysym.h>`: **measured**, from the
   headers this machine has under `/opt/homebrew/include` (`libx11` 1.8.13 and `xorgproto` 2025.1;
   the clang probe and its output are the ledger's Evidence ②).  `Display`, `Window`, `XID`,
   `Atom`, `KeySym`, `KeyCode`, `Bool`, `XPointer`, `XrmDatabase` and the opaque 192-byte `XEvent`
   are already crate-root items of `include/xlib.rs`, and `Time` of `include/systime.rs`; they are
   not re-declared here (a second crate-root item would be `E0428`).

   C's `event.xbutton.serial = …` reads a *member of the XEvent union*.  `include/xlib.rs`'s
   `XEvent` is the measured 192-byte record with an opaque body (its 33 member records are owed in
   `docs/ports/include-xlib.md`), so the port reaches the member the way `src/xselect.rs` 850 and
   3666 do (P-72 clause 3): a pointer view of the same storage
   (`let xbutton: *mut XButtonEvent = &raw mut event as *mut XButtonEvent;`), which keeps C's own
   member name in the text.  Every record below is asserted at its measured offsets.  */

/// C: <X11/Xlib.h> 568: `typedef XKeyEvent XKeyPressedEvent;`
#[cfg (feature = "have_x_windows")]
pub type XKeyPressedEvent = XKeyEvent;
/// C: <X11/Xlib.h> 586: `typedef XButtonEvent XButtonPressedEvent;` - C 1749's
/// `XButtonPressedEvent *event` (C's `&(dummy.xbutton)`).
#[cfg (feature = "have_x_windows")]
pub type XButtonPressedEvent = XButtonEvent;

/// C: <X11/Xlib.h> 567 closes the record opened at 556:
/// `typedef struct { int type; unsigned long serial; Bool send_event; Display *display; Window
/// window; Window root; Window subwindow; Time time; int x, y; int x_root, y_root; unsigned int
/// state; unsigned int button; Bool same_screen; } XKeyEvent;` - measured: `sizeof` **96**,
/// `serial`@8, `send_event`@16, `display`@24, `window`@32, `root`@40, `subwindow`@48, `time`@56,
/// `x`@64, `y`@68, `x_root`@72, `y_root`@76, `state`@80, `keycode`@84, `same_screen`@88.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XKeyEvent
{
  pub r#type: c_int,          // Xlib.h 557
  pub serial: c_ulong,        // Xlib.h 558
  pub send_event: Bool,       // Xlib.h 559
  pub display: *mut Display,  // Xlib.h 560
  pub window: Window,         // Xlib.h 561
  pub root: Window,           // Xlib.h 562
  pub subwindow: Window,      // Xlib.h 563
  pub time: Time,             // Xlib.h 564
  pub x: c_int,               // Xlib.h 565
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,          // Xlib.h 566
  pub keycode: c_uint,
  pub same_screen: Bool,
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XKeyEvent> () == 96);
  core::assert! (core::mem::offset_of! (XKeyEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XKeyEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XKeyEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XKeyEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XKeyEvent, window) == 32);
  core::assert! (core::mem::offset_of! (XKeyEvent, root) == 40);
  core::assert! (core::mem::offset_of! (XKeyEvent, subwindow) == 48);
  core::assert! (core::mem::offset_of! (XKeyEvent, time) == 56);
  core::assert! (core::mem::offset_of! (XKeyEvent, x) == 64);
  core::assert! (core::mem::offset_of! (XKeyEvent, y) == 68);
  core::assert! (core::mem::offset_of! (XKeyEvent, x_root) == 72);
  core::assert! (core::mem::offset_of! (XKeyEvent, y_root) == 76);
  core::assert! (core::mem::offset_of! (XKeyEvent, state) == 80);
  core::assert! (core::mem::offset_of! (XKeyEvent, keycode) == 84);
  core::assert! (core::mem::offset_of! (XKeyEvent, same_screen) == 88);
};

/// C: <X11/Xlib.h> 585 closes the record opened at 570: `… unsigned int state; unsigned int
/// button; Bool same_screen; } XButtonEvent;` - same layout as `XKeyEvent` through `state`@80,
/// `button`@84, `same_screen`@88; measured `sizeof` **96**.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XButtonEvent
{
  pub r#type: c_int,          // Xlib.h 571
  pub serial: c_ulong,        // Xlib.h 572
  pub send_event: Bool,       // Xlib.h 573
  pub display: *mut Display,  // Xlib.h 574
  pub window: Window,         // Xlib.h 575
  pub root: Window,           // Xlib.h 576
  pub subwindow: Window,      // Xlib.h 577
  pub time: Time,             // Xlib.h 578
  pub x: c_int,               // Xlib.h 579
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,          // Xlib.h 580
  pub button: c_uint,
  pub same_screen: Bool,
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XButtonEvent> () == 96);
  core::assert! (core::mem::offset_of! (XButtonEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XButtonEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XButtonEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XButtonEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XButtonEvent, window) == 32);
  core::assert! (core::mem::offset_of! (XButtonEvent, root) == 40);
  core::assert! (core::mem::offset_of! (XButtonEvent, subwindow) == 48);
  core::assert! (core::mem::offset_of! (XButtonEvent, time) == 56);
  core::assert! (core::mem::offset_of! (XButtonEvent, x) == 64);
  core::assert! (core::mem::offset_of! (XButtonEvent, y) == 68);
  core::assert! (core::mem::offset_of! (XButtonEvent, x_root) == 72);
  core::assert! (core::mem::offset_of! (XButtonEvent, y_root) == 76);
  core::assert! (core::mem::offset_of! (XButtonEvent, state) == 80);
  core::assert! (core::mem::offset_of! (XButtonEvent, button) == 84);
  core::assert! (core::mem::offset_of! (XButtonEvent, same_screen) == 88);
};

/// C: <X11/Xlib.h> 936 closes the record opened at 933: `typedef struct { int type; unsigned long
/// serial; Bool send_event; Display *display; Window window; } XAnyEvent;` - measured: `sizeof`
/// **40**, `window`@32 (C 459/1269/1815's `copy.xany.display`).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XAnyEvent
{
  pub r#type: c_int,          // Xlib.h 934
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,         // Xlib.h 935
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XAnyEvent> () == 40);
  core::assert! (core::mem::offset_of! (XAnyEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XAnyEvent, window) == 32);
};

/// C: <X11/Xlib.h> 667 closes the record opened at 657: `typedef struct { int type; unsigned long
/// serial; Bool send_event; Display *display; Window window; int x, y; int width, height; int
/// count; } XExposeEvent;` - measured: `sizeof` **64**, `x`@40, `y`@44, `width`@48, `height`@52,
/// `count`@56.  C 302 reads `event->xexpose.display` through it.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XExposeEvent
{
  pub r#type: c_int,          // Xlib.h 658
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,               // Xlib.h 663
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,           // Xlib.h 666
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XExposeEvent> () == 64);
  core::assert! (core::mem::offset_of! (XExposeEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XExposeEvent, x) == 40);
  core::assert! (core::mem::offset_of! (XExposeEvent, width) == 48);
  core::assert! (core::mem::offset_of! (XExposeEvent, count) == 56);
};

/// C: <X11/Xlib.h> 841 closes the record opened at 826: `typedef struct { int type; unsigned long
/// serial; Bool send_event; Display *display; Window window; Atom atom; Time time; int state; }
/// XPropertyEvent;` - measured: `sizeof` **64**, `window`@32, `atom`@40, `time`@48, `state`@56
/// (C 1732's `xevent->xproperty.window`/`.atom`).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XPropertyEvent
{
  pub r#type: c_int,          // Xlib.h 833
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,         // Xlib.h 837
  pub atom: Atom,             // Xlib.h 838
  pub time: Time,             // Xlib.h 839
  pub state: c_int,           // Xlib.h 840
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XPropertyEvent> () == 64);
  core::assert! (core::mem::offset_of! (XPropertyEvent, window) == 32);
  core::assert! (core::mem::offset_of! (XPropertyEvent, atom) == 40);
  core::assert! (core::mem::offset_of! (XPropertyEvent, time) == 48);
  core::assert! (core::mem::offset_of! (XPropertyEvent, state) == 56);
};

/// C: <X11/Xlib.h> 952 closes the record opened at 943: `typedef struct { int type; unsigned long
/// serial; Bool send_event; Display *display; int extension; int evtype; } XGenericEvent;` -
/// measured: `sizeof` **40**, `extension`@32, `evtype`@36.  C 245/377 read
/// `event.xgeneric.extension`/`.display`/`.evtype`.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGenericEvent
{
  pub r#type: c_int,          // Xlib.h 944
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,       // Xlib.h 948
  pub evtype: c_int,          // Xlib.h 951
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XGenericEvent> () == 40);
  core::assert! (core::mem::offset_of! (XGenericEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XGenericEvent, extension) == 32);
  core::assert! (core::mem::offset_of! (XGenericEvent, evtype) == 36);
};

/// C: <X11/Xlib.h> 963 closes the record opened at 955: `typedef struct { int type; unsigned long
/// serial; Bool send_event; Display *display; int extension; int evtype; unsigned int cookie;
/// void *data; } XGenericEventCookie;` - measured: `sizeof` **56**, `cookie`@40, `data`@48 (C
/// 395/503's `event.xcookie.data`, C 512's `event.xcookie.evtype`, and C 258's
/// `eassert (!event->xcookie.data)`).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XGenericEventCookie
{
  pub r#type: c_int,          // Xlib.h 956
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub cookie: c_uint,         // Xlib.h 961
  pub data: *mut c_void,      // Xlib.h 962
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XGenericEventCookie> () == 56);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, evtype) == 36);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, cookie) == 40);
  core::assert! (core::mem::offset_of! (XGenericEventCookie, data) == 48);
};

/// C: <X11/X.h> - the event-type codes and masks C compares against (`event.type == ButtonRelease`
/// C 339, `== KeyPress` C 350, `GenericEvent` C 375, `PropertyNotify` C 1731; the `state` masks C
/// 354/418 and the button numbers/masks C 253/261/1720/1794).  Measured (X.h 139-259).
#[cfg (feature = "have_x_windows")]
pub const ButtonPress: c_int = 4;          // X.h 183
#[cfg (feature = "have_x_windows")]
pub const ButtonRelease: c_int = 5;        // X.h 184
#[cfg (feature = "have_x_windows")]
pub const KeyPress: c_int = 2;             // X.h 181
#[cfg (feature = "have_x_windows")]
pub const PropertyNotify: c_int = 28;      // X.h 207
#[cfg (feature = "have_x_windows")]
pub const GenericEvent: c_int = 35;        // X.h 214
#[cfg (feature = "have_x_windows")]
pub const ControlMask: c_uint = 1 << 2;    // X.h 223
#[cfg (feature = "have_x_windows")]
pub const Button1: c_uint = 1;             // X.h 259
#[cfg (feature = "have_x_windows")]
pub const Button1Mask: c_uint = 1 << 8;    // X.h 246
#[cfg (feature = "have_x_windows")]
pub const ButtonReleaseMask: c_uint = 1 << 3;   // X.h 245
#[cfg (feature = "have_x_windows")]
pub const CurrentTime: Time = 0;           // X.h 139
#[cfg (feature = "have_x_windows")]
pub const PropModeReplace: c_int = 0;      // X.h 475
#[cfg (feature = "have_x_windows")]
pub const XA_ATOM: Atom = 4;               // Xatom.h 12
#[cfg (feature = "have_x_windows")]
pub const XK_g: KeySym = 0x0067;           // keysym.h 696
#[cfg (feature = "have_x_windows")]
pub const XK_Escape: KeySym = 0xff1b;      // keysym.h 208
#[cfg (feature = "have_x_windows")]
#[cfg (feature = "have_x_windows")]
/// C: <X11/Xlib.h> 927: `typedef int Status;` (XGetGeometry's return).
#[cfg (feature = "have_x_windows")]
pub type Status = c_int;

/// C: <X11/Xlib.h> 97-98: `#define DisplayWidth(dpy, scr) (ScreenOfDisplay(dpy,scr)->width)` and
/// `#define DisplayHeight(dpy, scr) (ScreenOfDisplay(dpy,scr)->height)` (C 2747-2748), and C 87's
/// `#define ConnectionNumber(dpy) (((_XPrivDisplay)(dpy))->fd)` (C 208).  Measured (the macros are
/// Xlib.h's own; their values are the `Screen` record's members this mirror carries at
/// `include/xlib.rs` 975).
#[cfg (feature = "have_x_windows")]
macro_rules! DisplayWidth {
  ($dpy:expr, $scr:expr) => { (*ScreenOfDisplay ($dpy, $scr)).width };
}
#[cfg (feature = "have_x_windows")]
macro_rules! DisplayHeight {
  ($dpy:expr, $scr:expr) => { (*ScreenOfDisplay ($dpy, $scr)).height };
}
#[cfg (feature = "have_x_windows")]
macro_rules! ConnectionNumber {
  ($dpy:expr) => { (*($dpy as *mut _XPrivDisplay)).fd };
}
/// C: <X11/Xlib.h> 2763/2891/2517/1692/2908/3456/2078/2751/3496/3491/3486/2630/4014/4009/
/// <X11/Xresource.h> 204 - the entry points this unit calls, measured from the headers on this
/// machine (each line of the block above is the header's own prototype).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XLookupKeysym (event: *mut XKeyEvent, index: c_int) -> KeySym;           // Xlib.h 1692
  pub fn XPending (display: *mut Display) -> c_int;                               // Xlib.h 2891
  pub fn XFlush (display: *mut Display) -> c_int;                                 // Xlib.h 2517
  pub fn XTranslateCoordinates (display: *mut Display, src_w: Window, dest_w: Window,
                                src_x: c_int, src_y: c_int, dest_x_return: *mut c_int,
                                dest_y_return: *mut c_int,
                                child_return: *mut Window) -> Bool;               // Xlib.h 3456
  pub fn XPutBackEvent (display: *mut Display, event: *mut XEvent) -> c_int;      // Xlib.h 2908
  pub fn XChangeProperty (display: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar,
                          nelements: c_int) -> c_int;                             // Xlib.h 2078
  pub fn XIfEvent (display: *mut Display, event_return: *mut XEvent,
                   predicate: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, XPointer) -> Bool>,
                   arg: XPointer) -> c_int;                                       // Xlib.h 2763
  pub fn XGrabServer (display: *mut Display) -> c_int;                            // Xlib.h 2751
  pub fn XUngrabServer (display: *mut Display) -> c_int;                          // Xlib.h 3496
  pub fn XUngrabPointer (display: *mut Display, time: Time) -> c_int;             // Xlib.h 3491
  pub fn XUngrabKeyboard (display: *mut Display, time: Time) -> c_int;            // Xlib.h 3486
  pub fn XFreeEventData (display: *mut Display, cookie: *mut XGenericEventCookie); // Xlib.h 4014
  pub fn XGetEventData (display: *mut Display, cookie: *mut XGenericEventCookie) -> Bool; // Xlib.h 4009
  pub fn XrmPutStringResource (database: *mut XrmDatabase, resource: *const c_char,
                               value: *const c_char);                             // Xresource.h 204
}
/// C: <X11/Xlib.h> 2630: `extern Status XGetGeometry (Display *display, Drawable d, Window
/// *root_return, int *x_return, int *y_return, unsigned int *width_return, unsigned int
/// *height_return, unsigned int *border_width_return, unsigned int *depth_return);` (C 2673).
/// The declaration sits in a module with a glob import rather than at this unit's crate root:
/// `include/msdos.rs` - which C 50-52's `#ifdef MSDOS` puts on this unit's include graph when
/// `msdos` is set - declares its own `XGetGeometry` for the MSDOS build, and a unit-side
/// redeclaration is the copy `tools/check-mirror-dupes.py` reports.  The module is PORTING.md
/// P-21/P-39's shape for exactly this (`pub mod` + `use mod::*;`): the declaration is a different
/// scope and the call sites keep C's name.  */
#[cfg (feature = "have_x_windows")]
pub mod x_menu_xlib_prototypes
{
  /* A module does not inherit the crate root's imports, and the crate root's `c_int`-family
     bindings come from `include/lisp.rs`'s own `use` (a binding, not an item), so the module
     carries both its own `use` and a glob of the crate root's items (`Display`, `Window`,
     `Status`, `XEvent`, ... are items).  */
  use super::*;
  use std::ffi::{c_char, c_int, c_uint, c_void};
  unsafe extern "C"
  {
    pub fn XGetGeometry (display: *mut Display, d: Window, root_return: *mut Window,
                         x_return: *mut c_int, y_return: *mut c_int,
                         width_return: *mut c_uint, height_return: *mut c_uint,
                         border_width_return: *mut c_uint,
                         depth_return: *mut c_uint) -> Status;                  // Xlib.h 2630
  }
}
#[cfg (feature = "have_x_windows")]
use x_menu_xlib_prototypes::*;
/// C: <X11/Xlib.h> 97-98's `ScreenOfDisplay` and 85's `_XPrivDisplay`, which the two `DisplayWidth`
/// macros above spend.  Measured (`ScreenOfDisplay` is Xlib.h 96's macro over the display record's
/// `screens`/`nscreens`; this unit reads only the two width/height members).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn ScreenOfDisplay (dpy: *mut Display, scr: c_int) -> *mut Screen;          // Xlib.h 96
}
/// C: <X11/Xlib.h> 86: `typedef struct _XPrivDisplay { … int fd; … } *_XPrivDisplay;` - the
/// `ConnectionNumber` macro's own cast target (C 208).  Measured: Xlib.h 86's record, whose `fd`
/// is at the head of the Xlib-private display record.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct _XPrivDisplay
{
  pub fd: c_int,               // Xlib.h 86 (the `fd` member ConnectionNumber reads)
}

/// C: <X11/extensions/XI2.h> - the XI2 event types and mask bounds C 240-293/375-458/1660-1700/
/// 2784-2800 name.  Measured from this machine's `xorgproto` 2025.1 (`XI2.h` 187-223).
#[cfg (feature = "have_x_windows")]
pub const XI_ButtonPress: c_int = 4;           // XI2.h 194
#[cfg (feature = "have_x_windows")]
pub const XI_ButtonRelease: c_int = 5;         // XI2.h 195
#[cfg (feature = "have_x_windows")]
pub const XI_KeyPress: c_int = 2;              // XI2.h 192
#[cfg (feature = "have_x_windows")]
pub const XI_Motion: c_int = 6;                // XI2.h 196
#[cfg (feature = "have_x_windows")]
pub const XI_Enter: c_int = 7;                 // XI2.h 197
#[cfg (feature = "have_x_windows")]
pub const XI_Leave: c_int = 8;                 // XI2.h 198
#[cfg (feature = "have_x_windows")]
pub const XI_HierarchyChanged: c_int = 11;     // XI2.h 201
#[cfg (feature = "have_x_windows")]
pub const XI_DeviceChanged: c_int = 1;         // XI2.h 191
#[cfg (feature = "have_x_windows")]
pub const XI_GesturePinchBegin: c_int = 27;    // XI2.h 217
#[cfg (feature = "have_x_windows")]
pub const XI_GesturePinchUpdate: c_int = 28;   // XI2.h 218
#[cfg (feature = "have_x_windows")]
pub const XI_GesturePinchEnd: c_int = 29;      // XI2.h 219
/// C: <X11/extensions/XI2.h> 223: `#define XI_LASTEVENT XI_GestureSwipeEnd` - measured **32**.
#[cfg (feature = "have_x_windows")]
pub const XI_LASTEVENT: c_int = 32;
/// C: <X11/extensions/XI2.h> 188: `#define XIAllMasterDevices 1` - measured.
#[cfg (feature = "have_x_windows")]
pub const XIAllMasterDevices: c_int = 1;

/* (b) `<X11/extensions/XInput2.h>` (libXi) and `<X11/Intrinsic.h>`, `<X11/Shell.h>`,
   `<X11/StringDefs.h>`, `<X11/CoreP.h>`, `<X11/Xaw/Paned.h>`, `<X11/Xaw3d/Paned.h>`, `<gtk/gtk.h>`
   and `<glib.h>`: **no such header on this machine** (no libXi, no libXt, no Xaw, no Motif, no
   GTK3 - `ls /opt/homebrew/include/X11` has libX11 and xorgproto only), so the surface below is
   `UNVERIFIED`: it is the C headers' own contract, needed so these arms are *text* rather than a
   name the compiler has to invent, and the ledger's `## Owed` names each one for a machine that
   has the SDK.  Nothing here compiles on this target in any case: every arm below is inside
   `use_x_toolkit`/`use_gtk`/`have_xinput2`, and the file gate is already false.  */

/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget; typedef char *XtPointer; typedef
/// long Cardinal; typedef short Position; typedef unsigned int Dimension;
/// typedef struct { String name; XtArgVal value; } Arg, *ArgList; typedef Widget *WidgetList;
/// typedef struct _XtTranslations *XtTranslations; typedef int Boolean;
/// typedef struct _XtAppContextRec *XtAppContext;` - the types C 472/536/700/711/1746/2509 spend.
#[cfg (feature = "have_x_windows")]
pub type Widget = *mut c_void;
#[cfg (feature = "have_x_windows")]
pub type WidgetClass = *mut c_void;
#[cfg (feature = "have_x_windows")]
pub type XtPointer = *mut c_void;
#[cfg (feature = "have_x_windows")]
pub type Cardinal = c_ulong;
#[cfg (feature = "have_x_windows")]
pub type Position = c_short;
#[cfg (feature = "have_x_windows")]
pub type Dimension = c_uint;
#[cfg (feature = "have_x_windows")]
pub type Boolean = c_int;
#[cfg (feature = "have_x_windows")]
pub type XtAppContext = *mut c_void;
#[cfg (feature = "have_x_windows")]
pub type XtTranslations = *mut c_void;
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct Arg
{
  pub name: *const c_char,
  pub value: *mut c_void,
}
#[cfg (feature = "have_x_windows")]
pub type ArgList = *mut Arg;
#[cfg (feature = "have_x_windows")]
pub type WidgetList = *mut Widget;
/// C: `<X11/Intrinsic.h>`: `void XtAppNextEvent (XtAppContext, XEvent *); Boolean XtAppPending
/// (XtAppContext); void XtDispatchEvent (XEvent *); Window XtWindow (Widget); void XtSetArg (Arg,
/// String, XtArgVal); …` with `<X11/StringDefs.h>`'s resource names C 536-543/1240/1255/1261/1310/
/// 1317/1775 spend.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XtAppNextEvent (app_context: XtAppContext, event_return: *mut XEvent);
  pub fn XtAppPending (app_context: XtAppContext) -> Boolean;
  pub fn XtDispatchEvent (event: *mut XEvent) -> Boolean;
  pub fn XtWindow (widget: Widget) -> Window;
  pub fn XtGetValues (widget: Widget, args: ArgList, num_args: Cardinal);
  pub fn XtSetValues (widget: Widget, args: ArgList, num_args: Cardinal);
  pub fn XtVaGetValues (widget: Widget, ...);
  pub fn XtVaSetValues (widget: Widget, ...);
  pub fn XtUnmanageChild (widget: Widget);
  pub fn XtManageChild (widget: Widget);
  pub fn XtMapWidget (widget: Widget);
  pub fn XtRealizeWidget (widget: Widget);
  pub fn XtIsManaged (widget: Widget) -> Boolean;
  pub fn XtParseTranslationTable (table: *const c_char) -> XtTranslations;
  pub fn XtOverrideTranslations (widget: Widget, table: XtTranslations);
  pub fn XtDatabase (display: *mut Display) -> XrmDatabase;
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  pub fn XtLastTimestampProcessed (display: *mut Display) -> Time;
}
/// C: `<X11/IntrinsicP.h>` (libXt/Xaw - **not on this machine**): `typedef struct { … Dimension
/// width; Dimension height; Dimension border_width; … } CorePart;` and
/// `typedef struct _WidgetRec { CorePart core; … } *Widget;` - C 1230/1236 read
/// `menubar_widget->core.height` and `->core.border_width` through them.  The record below keeps
/// C's three members and their declared order; its *offsets* are `UNVERIFIED` (no libXt here) and
/// the ledger's `## Owed` names the machine that has to measure them.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct CorePart
{
  pub width: Dimension,           // IntrinsicP.h: `Dimension width;`
  pub height: Dimension,          // `Dimension height;`
  pub border_width: Dimension,    // `Dimension border_width;`
}
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct _WidgetRec
{
  pub core: CorePart,             // IntrinsicP.h: `_WidgetRec`'s first member is its `CorePart core`
}
/// C: `<X11/Intrinsic.h>`: `void XtSetArg (Arg arg, String name, XtArgVal value)` - a macro in the
/// header, kept as a macro here so C's own call text (`XtSetArg (al[0], XtNchildren, &list)`, C
/// 538/539/1766/1795) reads the same.
#[cfg (feature = "have_x_windows")]
macro_rules! XtSetArg {
  ($arg:expr, $name:expr, $val:expr) => {
    { $arg.name = $name.as_ptr (); $arg.value = $val as XtPointer; }
  };
}
/// C: `<X11/StringDefs.h>`: `#define XtNchildren "children"`, `XtNnumChildren "numChildren"`,
/// `XtNmappedWhenManaged "mappedWhenManaged"`, `XtNgeometry "geometry"`,
/// `XtNinternalBorderWidth "internalBorderWidth"`, `XtNx "x"`, `XtNy "y"` - the resource names C
/// 538-543/1241/1242/1310/1317/1766/1795 spend.  The strings are libXt's and are `UNVERIFIED`
/// here (no libXt on this machine).
#[cfg (feature = "have_x_windows")]
pub const XtNchildren: &core::ffi::CStr = c"children";
#[cfg (feature = "have_x_windows")]
pub const XtNnumChildren: &core::ffi::CStr = c"numChildren";
#[cfg (feature = "have_x_windows")]
pub const XtNmappedWhenManaged: &core::ffi::CStr = c"mappedWhenManaged";
#[cfg (feature = "have_x_windows")]
#[cfg (feature = "have_x_windows")]
#[cfg (feature = "have_x_windows")]
pub const XtNx: &core::ffi::CStr = c"x";
#[cfg (feature = "have_x_windows")]
pub const XtNy: &core::ffi::CStr = c"y";

/// C: <X11/extensions/XInput2.h> (libXi - not on this machine): `typedef struct { int deviceid;
/// int mask_len; unsigned char *mask; } XIEventMask;`, `typedef struct { … } XIDeviceEvent;` with
/// the members C 249-268/396-412/2761-2766 read (`event_x`, `event_y`, `root_x`, `root_y`,
/// `deviceid`, `detail`, `serial`, `send_event`, `event`, `root`, `child`, `time`).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct XIEventMask
{
  pub deviceid: c_int,
  pub mask_len: c_int,
  pub mask: *mut c_uchar,
}
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct XIDeviceEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub deviceid: c_int,
  pub sourceid: c_int,
  pub detail: c_int,
  pub flags: c_int,
  pub root: Window,
  pub event: Window,
  pub child: Window,
  pub root_x: f64,
  pub root_y: f64,
  pub event_x: f64,
  pub event_y: f64,
  pub time: Time,
}
/// C: <X11/extensions/XInput2.h> (libXi): `#define XISetMask(ptr, event) (((unsigned char*)(ptr))
/// [(event)>>3] |= (1 << ((event)&7)))` and `#define XIMaskLen(event) (((event) >> 3) + 1)` - the
/// two macros C 1626/1668/2774 spend.  `UNVERIFIED` (no libXi here).
#[cfg (feature = "have_x_windows")]
macro_rules! XISetMask {
  ($ptr:expr, $event:expr) => {
    { *($ptr as *mut u8).offset (($event >> 3) as isize) |= (1u8 << ($event & 7)); }
  };
}
#[cfg (feature = "have_x_windows")]
macro_rules! XIMaskLen {
  ($event:expr) => { (($event >> 3) + 1) };
}
/// C: <X11/extensions/XInput2.h> (libXi): `int XISelectEvents (Display *, Window, XIEventMask *,
/// int);` and `void XIUngrabDevice (Display *, int, Time);` - C 1680/2813/2836/2799/1760.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn XISelectEvents (display: *mut Display, win: Window, masks: *mut XIEventMask,
                         num_masks: c_int) -> c_int;
  pub fn XIUngrabDevice (display: *mut Display, deviceid: c_int, time: Time) -> c_int;
}

/// C: `<gtk/gtk.h>` (no GTK3 on this machine): the names `include/gtk.rs` does not carry, which C
/// 796/800/804/1523/2189 spends.  `UNVERIFIED`.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub type GtkCheckMenuItem = GtkWidget;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub type GtkRadioMenuItem = GtkWidget;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub type GtkMenuPositionFunc = Option<unsafe extern "C" fn (*mut GtkMenu, *mut c_int,
                                                           *mut c_int, *mut gboolean,
                                                           gpointer)>;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
macro_rules! GTK_IS_RADIO_MENU_ITEM {
  ($w:expr) => { true };
}
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
macro_rules! GTK_CHECK_MENU_ITEM {
  ($w:expr) => { ($w as *mut GtkCheckMenuItem) };
}
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn gtk_check_menu_item_get_active (check_menu_item: *mut GtkCheckMenuItem) -> gboolean;
  pub fn gtk_widget_get_mapped (widget: *mut GtkWidget) -> gboolean;
  pub fn g_signal_emit_by_name (instance: gpointer, signal: *const c_char, ...);
  pub fn gtk_menu_popup (menu: *mut GtkMenu, parent_menu_shell: *mut c_void,
                         parent_menu_item: *mut c_void, func: GtkMenuPositionFunc,
                         data: gpointer, button: c_uint, activate_time: c_uint);
}

/* (c) The reference-tree names whose *mirrors* do not carry them (P-72 rule 4: a mirror reaches
   what it needs through its own `include!`s).  Each line cites the reference header it comes from;
   the ledger records each as a gap for the mirror's own round (the `src/xselect.rs` `## Owed`
   shape).  */
/// C: src/gtkutil.h 96: `extern GtkWidget *xg_create_widget (const char *type, …)`; 104:
/// `xg_modify_menubar_widgets`; 112: `xg_update_frame_menubar (struct frame *f)`; 168:
/// `xg_win_to_widget (Display *dpy, Window wdesc)`; 170: `xg_get_scale (struct frame *f)`.
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" {
  pub fn xg_create_widget (r#type: *const c_char, name: *const c_char, f: *mut frame,
                           val: *mut widget_value,
                           select_cb: GCallback, deactivate_cb: GCallback,
                           highlight_cb: GCallback) -> *mut GtkWidget;      // gtkutil.h 96
  pub fn xg_modify_menubar_widgets (menubar: *mut GtkWidget, f: *mut frame,
                                    val: *mut widget_value, deep_p: bool,
                                    select_cb: GCallback, deactivate_cb: GCallback,
                                    highlight_cb: GCallback);              // gtkutil.h 104
  pub fn xg_update_frame_menubar (f: *mut frame);                         // gtkutil.h 112
  pub fn xg_win_to_widget (dpy: *mut Display, wdesc: Window) -> *mut GtkWidget; // gtkutil.h 168
  pub fn xg_get_scale (f: *mut frame) -> c_int;                           // gtkutil.h 170
}
/// C: src/gtkutil.h 49: `#define XG_ITEM_DATA "emacs_menuitem"` (C 754) and 63-84's
/// `xg_menu_cb_data`/`xg_menu_item_cb_data` (C 752/758/794/800/1481/1485).
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub const XG_ITEM_DATA: *const c_char = c"emacs_menuitem".as_ptr ();
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[repr(C)]
pub struct xg_menu_cb_data
{
  pub f: *mut frame,                        // gtkutil.h 65
  pub menu_bar_vector: Lisp_Object,         // gtkutil.h 68
  pub menu_bar_items_used: c_int,           // gtkutil.h 69
  pub highlight_cb: GCallback,              // gtkutil.h 71
}
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[repr(C)]
pub struct xg_menu_item_cb_data
{
  pub id: c_ulong,                          // gtkutil.h 77
  pub call_data: Lisp_Object,               // gtkutil.h 79
  pub help: Lisp_Object,                    // gtkutil.h 80
  pub cl_data: *mut xg_menu_cb_data,        // gtkutil.h 82
}
/* C 754's `g_object_get_data` is a crate-root item of `include/gtk.rs` (its own `gobject.h` 594-595
   citation), which the edge above brings in, so it is not re-declared here.  */
/// C: src/xsettings.h 43: `extern const char *xsettings_get_system_normal_font (void);` (C 898/911).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn xsettings_get_system_normal_font () -> *const c_char;            // xsettings.h 43
}
/// C: src/xterm.h 1001/1752/1762/1765/1788/1789/1801/1806/1834/1857/1863/1864/2022/2024/2027 -
/// the X11 entry points the mirror `include/xterm.rs` does not carry, at C's own positions.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_dpyinfo (display: *mut Display) -> *mut x_display_info;             // xterm.h 1001
  pub fn x_catch_errors (display: *mut Display);                               // xterm.h 1752
  pub fn x_had_errors_p (display: *mut Display) -> bool;                       // xterm.h 1762
  pub fn x_uncatch_errors_after_check ();                                      // xterm.h 1765
  pub fn x_activate_timeout_atimer ();                                         // xterm.h 1789
  pub fn x_mouse_leave (dpyinfo: *mut x_display_info);                         // xterm.h 1801
  pub fn x_dispatch_event (event: *mut XEvent, display: *mut Display) -> c_int; // xterm.h 1806
  pub fn x_translate_coordinates_to_root (f: *mut frame, x: c_int, y: c_int,
                                          dest_x: *mut c_int,
                                          dest_y: *mut c_int);                 // xterm.h 1834
  pub fn x_handle_pending_selection_requests ();                               // xterm.h 1857
  pub fn xi_device_from_id (dpyinfo: *mut x_display_info,
                            id: c_int) -> *mut xi_device_t;                    // xterm.h 2022
  pub fn xi_frame_selected_for (f: *mut frame, event: c_ulong) -> bool;        // xterm.h 2024
  pub fn xi_convert_event_state (xev: *mut XIDeviceEvent) -> c_uint;           // xterm.h 2027
  pub static mut Xt_app_con: XtAppContext;                                     // xterm.h 1788
}
/// C: src/xterm.h 55: `typedef Widget xt_or_gtk_widget;` (the `USE_X_TOOLKIT` arm) and 83:
/// `typedef GtkWidget *xt_or_gtk_widget;` (the `USE_GTK` arm) - the type C 930/1285 and
/// `show_help_event`'s parameter spell.  Both arms are carried.
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
pub type xt_or_gtk_widget = Widget;
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
pub type xt_or_gtk_widget = *mut GtkWidget;
/// C: src/frame.h 1475: `INLINE struct frame * decode_window_system_frame (Lisp_Object);` (C 477)
/// and frame.h 1504: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);`
/// (C 2441) - the frame.h mirror does not carry either.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;   // frame.h 1475
                            // frame.h 1504
}
/// C: src/xterm.h 1855-1856: `extern void x_defer_selection_requests (void);` and
/// `extern void x_release_selection_requests_and_flush (void);` - the two the mirror's
/// `DEFER_SELECTIONS!` expands to (`include/xterm.rs` 1556-1563, xterm.h 1851-1854); their
/// definitions are `src/xterm.c` 900/953, so the names are declared here (P-39: another unit
/// defines them, so they cannot be crate-root `pub fn` definitions here).
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn x_defer_selection_requests ();                                  // xterm.h 1855
  pub fn x_release_selection_requests_and_flush ();                      // xterm.h 1856
}
/// C: src/menu.h 39/40/42/43/44 and src/keyboard.h 344-358 - the menu-item bookkeeping C's two
/// `x_menu_show`s and `x_dialog_show` spend, which `include/menu.rs` and `include/keyboard.rs` do
/// not carry.  `menu_items`, `menu_items_used`, `menu_items_allocated`, `menu_items_n_panes` and
/// `menu_items_inuse` are in `include/keyboard.rs` 262-266 and are not re-declared here.
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  pub fn make_widget_value (name: *const c_char, value: *mut c_char, enabled: bool,
                            help: Lisp_Object) -> *mut widget_value;              // menu.h 43
  pub fn free_menubar_widget_value_tree (wv: *mut widget_value);                 // menu.h 39
  pub fn update_submenu_strings (wv: *mut widget_value);                         // menu.h 40
  pub fn digest_single_submenu (start: c_int, end: c_int,
                                top_level_items: bool) -> *mut widget_value;      // menu.h 44
                                                    // keyboard.h 385
  pub fn find_and_call_menu_selection (f: *mut frame, items: c_int,
                                       vector: Lisp_Object,
                                       client_data: *mut c_void);                // keyboard.h
  pub fn x_set_z_group (f: *mut frame, r#type: Lisp_Object, value: Lisp_Object);  // frame.h
    // keyboard.h
                                      // xdisp.h
  pub fn safe_run_hooks (hook: Lisp_Object);                                     // keyboard.h
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);  // lisp.h 4825
  pub fn record_unwind_save_match_data ();                                       // search.h
  // frame.h
                  // frame.h
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;      // frame.h
  pub fn menu_bar_items (menu_bar_items: Lisp_Object) -> Lisp_Object;            // keyboard.h
  pub fn timer_check () -> timespec;                                             // keyboard.h
  pub fn quit () -> !;                                                           // lisp.h 4786
}

/* ==============================================================================================
   C 111-306.  */

/* Flag which when set indicates a dialog or menu has been posted by
   Xt on behalf of one of the widget sets.  */                    // C 111-112
#[cfg (all (feature = "have_x_windows", not (feature = "have_xinput2")))]
static mut popup_activated_flag: c_int = 0;                    // C 113-114 (`static int`)
#[cfg (all (feature = "have_x_windows", feature = "have_xinput2"))]
#[unsafe(no_mangle)]
pub static mut popup_activated_flag: c_int = 0;                // C 116 (`int`, the `#else` arm)

/* C 120-143: `#ifdef USE_X_TOOLKIT` / the widget-id counter, `menubar_id_to_frame` / `#endif`.
   Both arms are carried; the `#else` arm is empty.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
static mut next_menubar_widget_id: LWLIB_ID = 0;               // C 122

/* Return the frame whose ->output_data.x->id equals ID, or 0 if none.  */
/// C: struct frame *menubar_id_to_frame (LWLIB_ID id)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" fn menubar_id_to_frame (id: LWLIB_ID) -> *mut frame   // C 126-142
{
  let mut tail: Lisp_Object = Qnil ();                         // C 128
  let mut frame: Lisp_Object = Qnil ();
  let mut f: *mut frame;

  FOR_EACH_FRAME! (tail, frame, {                              // C 131-140
      f = XFRAME (frame);
      if !FRAME_WINDOW_P (f)
        { continue; }
      if (*(FRAME_X_OUTPUT! (f) as *mut x_output)).id == id as c_int
        { return f; }
    });
  return std::ptr::null_mut ();                                // C 141 (`return 0;`)
}

/* C 145-306: `#ifndef MSDOS` … `#endif /* ! MSDOS */`.  Both arms are carried; the `#else` arm is
   empty.  */

/* C 147-174: `#if defined USE_GTK || defined USE_MOTIF` and `x_menu_set_in_use` with C 158-161's
   `#ifdef USE_X_TOOLKIT` arm inside it.  Both arms of both conditions are carried.  */

/* Set menu_items_inuse so no other popup menu or dialog is created.  */
/// C: void x_menu_set_in_use (bool in_use)
#[cfg (all (feature = "have_x_windows", not (feature = "msdos"),
            any (feature = "use_gtk", feature = "use_motif")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_menu_set_in_use (in_use: bool)      // C 152-173
{
  let mut frames: Lisp_Object = Qnil ();                       // C 154
  let mut frame: Lisp_Object = Qnil ();

  menu_items_inuse = in_use;                                   // C 156
  popup_activated_flag = in_use as c_int;
  #[cfg (feature = "use_x_toolkit")]
  if popup_activated_flag != 0                                 // C 158-161
    { x_activate_timeout_atimer (); }

  /* Don't let frames in `above' z-group obscure popups.  */
  FOR_EACH_FRAME! (frames, frame, {                            // C 163-171
      let f: *mut frame = XFRAME (frame);                      // C 165

      if in_use && FRAME_Z_GROUP_ABOVE (f)                    // C 167
        { x_set_z_group (f, Qabove_suspended (), Qabove ()); }
      else if !in_use && FRAME_Z_GROUP_ABOVE_SUSPENDED (f)    // C 169
        { x_set_z_group (f, Qabove (), Qabove_suspended ()); }
    });
}

/* Wait for an X event to arrive or for a timer to expire.  */

/// C: void x_menu_wait_for_event (void *data)
#[cfg (all (feature = "have_x_windows", not (feature = "msdos")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_menu_wait_for_event (data: *mut c_void)   // C 179-228
{
  /* Another way to do this is to register a timer callback, that can be
     done in GTK and Xt.  But we have to do it like this when using only X
     anyway, and with callbacks we would have three variants for timer handling
     instead of the small ifdefs below.  */                     // C 181-185

  /* C 187-193: the three texts of the `while` condition, one per arm:
       `#if defined USE_X_TOOLKIT` `! (data ? XPending (data) : XtAppPending (Xt_app_con))`
       `#elif defined USE_GTK` `! gtk_events_pending ()`
       `#else` `! XPending (data)` `#endif`.  Rust cannot put a `#[cfg]` inside an expression, so
     the three texts are the three arms of one local macro that the `while` expands (the condition
     is still evaluated at the same point in each iteration; the ledger records the shape).  */
  #[cfg (feature = "use_x_toolkit")]
  macro_rules! x_menu_wait_for_event_pending {
    () => { ! (if !data.is_null () { XPending (data as *mut Display) != 0 }
               else { XtAppPending (Xt_app_con) != 0 }) };
  }
  #[cfg (all (not (feature = "use_x_toolkit"), feature = "use_gtk"))]
  macro_rules! x_menu_wait_for_event_pending {
    () => { ! (gtk_events_pending () != 0) };
  }
  #[cfg (all (not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]
  macro_rules! x_menu_wait_for_event_pending {
    () => { ! (XPending (data as *mut Display) != 0) };
  }

  while x_menu_wait_for_event_pending! ()                     // C 187-227
    {
      let mut next_time: timespec = timer_check ();             // C 195
      let mut ntp: *mut timespec;
      let mut read_fds: fd_set = core::mem::zeroed ();
      let mut dpyinfo: *mut x_display_info;
      let mut n: c_int = 0;                                     // C 199

      /* ISTM that if timer_check is okay, this should be too, since
	 both can run random Lisp.  */
      x_handle_pending_selection_requests ();                   // C 203

      FD_ZERO (&raw mut read_fds);                              // C 205
      dpyinfo = x_display_list;                                 // C 206
      while !dpyinfo.is_null ()                                 // C 206-214
        {
          let fd = ConnectionNumber! ((*dpyinfo).display) as c_int;   // C 208
          FD_SET (fd, &raw mut read_fds);
          if fd > n { n = fd; }
          XFlush ((*dpyinfo).display) as c_int;                 // C 212 (C discards the int)
          dpyinfo = (*dpyinfo).next;
        }

      if !timespec_valid_p (next_time)                          // C 216 (`! timespec_valid_p`)
        { ntp = std::ptr::null_mut (); }
      else
        { ntp = &raw mut next_time as *mut timespec; }

      #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]  // C 219-226
      /* Gtk3 have arrows on menus when they don't fit.  When the
	 pointer is over an arrow, a timeout scrolls it a bit.  Use
	 xg_select so that timeout gets triggered.  */
      { xg_select (n + 1, &raw mut read_fds as *mut c_void, std::ptr::null_mut (),
                   std::ptr::null_mut (),
                   ntp as *mut c_void, std::ptr::null_mut ()) as c_int; }
      #[cfg (not (all (feature = "use_gtk", feature = "have_gtk3")))]
      { pselect (n + 1, &raw mut read_fds, std::ptr::null_mut (), std::ptr::null_mut (),
                 ntp, std::ptr::null_mut ()) as c_int; }
    }
}

/* C 230-294: `#if !defined USE_GTK && !defined USE_X_TOOLKIT && defined HAVE_XINPUT2` /
   `x_menu_translate_generic_event` / `#endif`.  Both arms are carried; the `#else` arm is empty.
   The body reads the `XEvent` union's `xgeneric`/`xcookie`/`xbutton` members through the pointer
   views declared in the SDK section (P-72 clause 3).  */
/// C: static void x_menu_translate_generic_event (XEvent *event)
#[cfg (all (feature = "have_x_windows", not (feature = "msdos"),
            not (feature = "use_gtk"), not (feature = "use_x_toolkit"),
            feature = "have_xinput2"))]
unsafe fn x_menu_translate_generic_event (event: *mut XEvent)   // C 232-293
{
  let mut dpyinfo: *mut x_display_info;
  let mut device: *mut xi_device_t;
  let mut copy: XEvent;
  let mut xev: *mut XIDeviceEvent;
  let xgeneric: *mut XGenericEvent = event as *mut XGenericEvent;      // C 245/... `event->xgeneric`
  let xcookie: *mut XGenericEventCookie = event as *mut XGenericEventCookie;  // `event->xcookie`

  dpyinfo = x_dpyinfo ((*xgeneric).display);                   // C 239

  if (*xgeneric).extension == (*dpyinfo).xi2_opcode           // C 243
    {
      eassert! ((*xcookie).data.is_null ());                   // C 245 (`eassert (!event->xcookie.data)`)

      match (*xcookie).evtype                                  // C 247 (`switch (event->xcookie.evtype)`)
        {
          XI_ButtonPress | XI_ButtonRelease =>                 // C 250-251 (both labels share the arm)
            {
              if XGetEventData ((*dpyinfo).display, xcookie) == 0   // C 254
                { return; }

              xev = (*xcookie).data as *mut XIDeviceEvent;      // C 257
              let xbutton: *mut XButtonEvent = &raw mut copy as *mut XButtonEvent;
              (*xbutton).r#type = if (*xcookie).evtype == XI_ButtonPress   // C 258-259
                                    { ButtonPress } else { ButtonRelease };
              (*xbutton).serial = (*xev).serial;                // C 260
              (*xbutton).send_event = (*xev).send_event;
              (*xbutton).display = (*dpyinfo).display;
              (*xbutton).window = (*xev).event;
              (*xbutton).root = (*xev).root;
              (*xbutton).subwindow = (*xev).child;
              (*xbutton).time = (*xev).time;
              (*xbutton).x = lrint ((*xev).event_x) as c_int;   // C 268
              (*xbutton).y = lrint ((*xev).event_y) as c_int;
              (*xbutton).x_root = lrint ((*xev).root_x) as c_int;
              (*xbutton).y_root = lrint ((*xev).root_y) as c_int;
              (*xbutton).state = xi_convert_event_state (xev);  // C 272
              (*xbutton).button = (*xev).detail as c_uint;      // C 273
              (*xbutton).same_screen = True;                    // C 274

              device = xi_device_from_id (dpyinfo, (*xev).deviceid);   // C 276

              /* I don't know the repercussions of changing
	         device->grab on XI_ButtonPress events, so be safe and
	         only do what is necessary to prevent the grab from
	         being left invalid as XMenuActivate swallows
	         events.  */                                   // C 278-282
              if !device.is_null () && (*xev).evtype == XI_ButtonRelease
                { (*device).grab &= !(1 << (*xev).detail); }  // C 286-287

              XPutBackEvent ((*dpyinfo).display, &raw mut copy);   // C 289
              XFreeEventData ((*dpyinfo).display, xcookie);     // C 290
            }
          XI_HierarchyChanged | XI_DeviceChanged =>             // C 292-293
            /* These events must always be handled.  */
            { x_dispatch_event (event, (*dpyinfo).display) as c_int; }
        }
    }
}

/* C 296-305: `#if !defined USE_X_TOOLKIT && !defined USE_GTK` / `x_menu_dispatch_event` /
   `#endif`.  Both arms are carried; the `#else` arm is empty.  */
/// C: static int x_menu_dispatch_event (XEvent *event)
#[cfg (all (feature = "have_x_windows", not (feature = "msdos"),
            not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]
unsafe extern "C" fn x_menu_dispatch_event (event: *mut XEvent) -> c_int   // C 298-304
{
  let xexpose: *mut XExposeEvent = event as *mut XExposeEvent;   // C 300 (`event->xexpose`)
  x_dispatch_event (event, (*xexpose).display) as c_int;

  /* The return doesn't really matter.  */
  return 0;                                                     // C 303
}

/* ==============================================================================================
   C 308-1369: `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` … `#endif`.

   C 309 is the arm selector for the whole group; every item below carries it, and each `#ifdef`
   inside it carries its own `#[cfg]` with both arms.  */

/* ------------------------------------------------------------------------------------------
   C 311-575: `#ifdef USE_X_TOOLKIT` … `#endif /* USE_X_TOOLKIT */`.

   **The union-member pattern.**  C reads `event.xbutton.serial`, `event.xkey.state`,
   `event.xany.display`, `event.xgeneric.evtype`, `event.xcookie.data`, `event.xexpose.display`
   and `event.xproperty.window`; `include/xlib.rs`'s `XEvent` is the measured 192-byte record with
   an opaque body (its 33 member records are owed in `docs/ports/include-xlib.md`), so the port
   reaches each member exactly the way `src/xselect.rs` 850/1507/3666 does (P-72 clause 3): a
   pointer view bound to a local, and the local is named after C's *member* (`xbutton`, `xkey`,
   `xany`, `xgeneric`, `xcookie`, `xexpose`, `xproperty`) so the C member name stays in the text;
   where two objects need the same member the binding is repeated (shadowed) at the second C
   expression's position.  `copy = event` - C's struct assignment - is the port's
   `copy_nonoverlapping`, because `XEvent` carries no `Copy`.  */

/* Loop in Xt until the menu pulldown or dialog popup has been
   popped down (deactivated).  This is used for x-popup-menu
   and x-popup-dialog; it is not used for the menu bar.

   NOTE: All calls to popup_get_selection should be protected
   with BLOCK_INPUT, UNBLOCK_INPUT wrappers.  */                     // C 313-319
/// C: static void popup_get_selection (XEvent *initial_event, struct x_display_info *dpyinfo,
/// LWLIB_ID id, bool do_timers)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
unsafe extern "C" fn popup_get_selection (mut initial_event: *mut XEvent,
                                          dpyinfo: *mut x_display_info,
                                          id: LWLIB_ID, do_timers: bool)  // C 321-462
{
  let mut event: XEvent = core::mem::zeroed ();                // C 323
  let mut copy: XEvent = core::mem::zeroed ();                 // C 324
  #[cfg (feature = "have_xinput2")]
  let mut cookie_claimed_p: bool = false;                      // C 326
  #[cfg (feature = "have_xinput2")]
  let mut xev: *mut XIDeviceEvent;
  #[cfg (feature = "have_xinput2")]
  let mut device: *mut xi_device_t;

  while popup_activated_flag != 0                              // C 331
    {
      if !initial_event.is_null ()                             // C 333
        {
          /* C 335's `copy = event = *initial_event;` - two assignments, and Rust's assignment
             yields `()`.  */
          core::ptr::copy_nonoverlapping (&raw const *initial_event, &raw mut event, 1);
          core::ptr::copy_nonoverlapping (&raw const event, &raw mut copy, 1);
          initial_event = std::ptr::null_mut ();
        }
      else
        {
          if do_timers { x_menu_wait_for_event (std::ptr::null_mut ()); }   // C 342
          XtAppNextEvent (Xt_app_con, &raw mut event);          // C 343
          core::ptr::copy_nonoverlapping (&raw const event, &raw mut copy, 1);  // C 344
        }

      /* Make sure we don't consider buttons grabbed after menu goes.
         And make sure to deactivate for any ButtonRelease,
         even if XtDispatchEvent doesn't do that.  */           // C 347-349
      let xany: *mut XAnyEvent = &raw mut event as *mut XAnyEvent;
      let xbutton: *mut XButtonEvent = &raw mut event as *mut XButtonEvent;
      if (*xany).r#type == ButtonRelease                        // C 350
          && (*dpyinfo).display == (*xbutton).display
        {
          (*dpyinfo).grabbed &= !(1 << (*xbutton).button);      // C 353
          #[cfg (feature = "use_motif")]                        // C 353-362
          /* Pretending that the event came from a
             Btn1Down seems the only way to convince Motif to
             activate its callbacks; setting the XmNmenuPost
             isn't working. --marcus@sysc.pdx.edu.  */
            {
              (*xbutton).button = 1;
              /*  Motif only pops down menus when no Ctrl, Alt or Mod
                  key is pressed and the button is released.  So reset key state
                  so Motif thinks this is the case.  */
              (*xbutton).state = 0;
            }
          core::ptr::copy_nonoverlapping (&raw const event, &raw mut copy, 1);   // C 363
        }
      /* Pop down on C-g and Escape.  */
      else if (*xany).r#type == KeyPress                        // C 366-367
               && (*dpyinfo).display == (*xbutton).display
        {
          let xkey: *mut XKeyEvent = &raw mut event as *mut XKeyEvent;
          let keysym: KeySym = XLookupKeysym (xkey, 0);         // C 369

          if (keysym == XK_g && ((*xkey).state & ControlMask) != 0)
              || keysym == XK_Escape /* Any escape, ignore modifiers.  */
            { popup_activated_flag = 0; }                       // C 372-373

          core::ptr::copy_nonoverlapping (&raw const event, &raw mut copy, 1);   // C 375
        }
      else
        {
          #[cfg (feature = "have_xinput2")]                     // C 377-465
          {
            let xgeneric: *mut XGenericEvent = &raw mut event as *mut XGenericEvent;
            if (*xgeneric).r#type == GenericEvent
	       && (*dpyinfo).supports_xi2
	       && (*xgeneric).display == (*dpyinfo).display
	       && (*xgeneric).extension == (*dpyinfo).xi2_opcode
	      {
	        let xcookie: *mut XGenericEventCookie
		  = &raw mut event as *mut XGenericEventCookie;
	        if (*xcookie).data.is_null ()
	            && XGetEventData ((*dpyinfo).display, xcookie) != 0
	          { cookie_claimed_p = true; }                      // C 383-384

	        if !(*xcookie).data.is_null ()                      // C 386
	          {
	            match (*xgeneric).evtype                        // C 388
		      {
		        XI_ButtonRelease =>                          // C 390
		          {
		            xev = (*xcookie).data as *mut XIDeviceEvent;    // C 392
		            device = xi_device_from_id (dpyinfo, (*xev).deviceid);  // C 393

		            (*dpyinfo).grabbed &= !(1 << (*xev).detail);    // C 395
		            (*device).grab &= !(1 << (*xev).detail);        // C 396

		            let xbutton: *mut XButtonEvent
			      = &raw mut copy as *mut XButtonEvent;     // C 398-412
		            (*xbutton).r#type = ButtonRelease;
		            (*xbutton).serial = (*xev).serial;
		            (*xbutton).send_event = (*xev).send_event;
		            (*xbutton).display = (*dpyinfo).display;
		            (*xbutton).window = (*xev).event;
		            (*xbutton).root = (*xev).root;
		            (*xbutton).subwindow = (*xev).child;
		            (*xbutton).time = (*xev).time;
		            (*xbutton).x = lrint ((*xev).event_x) as c_int;
		            (*xbutton).y = lrint ((*xev).event_y) as c_int;
		            (*xbutton).x_root = lrint ((*xev).root_x) as c_int;
		            (*xbutton).y_root = lrint ((*xev).root_y) as c_int;
		            (*xbutton).state = xi_convert_event_state (xev);
		            (*xbutton).button = (*xev).detail as c_uint;
		            (*xbutton).same_screen = True;

		            #[cfg (feature = "use_motif")]              // C 415-424
		            /* Pretending that the event came from a
		               Btn1Down seems the only way to convince Motif to
		               activate its callbacks; setting the XmNmenuPost
		               isn't working. --marcus@sysc.pdx.edu.  */
		              {
		                (*xbutton).button = 1;
		                /*  Motif only pops down menus when no Ctrl, Alt or Mod
		                    key is pressed and the button is released.  So reset key state
		                    so Motif thinks this is the case.  */
		                (*xbutton).state = 0;
		              }
		          }
		        XI_KeyPress =>                                // C 426
		          {
		            let mut keysym: KeySym;

		            xev = (*xcookie).data as *mut XIDeviceEvent;  // C 431

		            let xkey: *mut XKeyEvent = &raw mut copy as *mut XKeyEvent;  // C 433-447
		            (*xkey).r#type = KeyPress;
		            (*xkey).serial = (*xev).serial;
		            (*xkey).send_event = (*xev).send_event;
		            (*xkey).display = (*dpyinfo).display;
		            (*xkey).window = (*xev).event;
		            (*xkey).root = (*xev).root;
		            (*xkey).subwindow = (*xev).child;
		            (*xkey).time = (*xev).time;
		            (*xkey).x = lrint ((*xev).event_x) as c_int;
		            (*xkey).y = lrint ((*xev).event_y) as c_int;
		            (*xkey).x_root = lrint ((*xev).root_x) as c_int;
		            (*xkey).y_root = lrint ((*xev).root_y) as c_int;
		            (*xkey).state = xi_convert_event_state (xev);
		            (*xkey).keycode = (*xev).detail as c_uint;
		            (*xkey).same_screen = True;

		            keysym = XLookupKeysym (xkey, 0);           // C 449

		            if (keysym == XK_g && ((*xkey).state & ControlMask) != 0)
		                || keysym == XK_Escape /* Any escape, ignore modifiers.  */
		              { popup_activated_flag = 0; }             // C 453-454
		          }
		        _ => {}                                    // C 388's other cases
		      }
	          }
	      }

            if cookie_claimed_p                                    // C 468-469
	      { XFreeEventData ((*dpyinfo).display, xcookie); }
          }
        }

      let xany: *mut XAnyEvent = &raw mut copy as *mut XAnyEvent;   // C 471 (`copy.xany.display`)
      x_dispatch_event (&raw mut copy, (*xany).display);
    }
}

#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit"))]
DEFUN! ("x-menu-bar-open-internal", Fx_menu_bar_open_internal,
       Sx_menu_bar_open_internal, 0, 1, "i",
       doc: "SKIP: real doc in USE_GTK definition in xmenu.c.",   // C 472-473
       (frame: Lisp_Object),                                     // C 474
       {
  let mut ev: XEvent = core::mem::zeroed ();                     // C 476
  let f: *mut frame = decode_window_system_frame (frame);        // C 477
  #[cfg (feature = "have_xinput2")]
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);    // C 479
  let mut menubar: Widget;                                        // C 481
  block_input ();                                                // C 482

  if FRAME_EXTERNAL_MENU_BAR (f)                                 // C 484
    { set_frame_menubar (f, true); }

  menubar = (*(FRAME_X_OUTPUT! (f) as *mut x_output)).menubar_widget;   // C 486
  if !menubar.is_null ()                                         // C 487
    {
      /* C 489-490's `Window child; bool error_p = false;` - `child` is written by
         `XTranslateCoordinates` below and read by nothing; C leaves it uninitialised, the port
         zeroes it (PORTING.md 1.5).  */
      let mut child: Window = 0;
      let mut error_p: bool = false;

      #[cfg (feature = "have_xinput2")]                          // C 492-515
      /* Clear the XI2 grab so Motif or lwlib can set a core grab.
	 Otherwise some versions of Motif will emit a warning and hang,
	 and lwlib will fail to destroy the menu window.  */
      if (*dpyinfo).supports_xi2
	  && xi_frame_selected_for (f, XI_ButtonPress as c_ulong)
	{
	  let mut i: c_int = 0;                                      // C 499
	  while i < (*dpyinfo).num_devices
	    {
	      /* The keyboard grab matters too, in this specific
		 case.  */                                       // C 503-504
	      #[cfg (not (feature = "use_lucid"))]                   // C 504-506
	      if (*(*dpyinfo).devices.offset (i as isize)).grab != 0
		{
		  XIUngrabDevice ((*dpyinfo).display,
				  (*(*dpyinfo).devices.offset (i as isize)).device_id,
				  CurrentTime) as c_int;
		  (*(*dpyinfo).devices.offset (i as isize)).grab = 0;
		}
	      #[cfg (feature = "use_lucid")]
		{
		  XIUngrabDevice ((*dpyinfo).display,
				  (*(*dpyinfo).devices.offset (i as isize)).device_id,
				  CurrentTime) as c_int;
		  (*(*dpyinfo).devices.offset (i as isize)).grab = 0;
		}
	      i += 1;
	    }
	}

      x_catch_errors (FRAME_X_DISPLAY! (f));                     // C 517
      memset (&raw mut ev as *mut c_void, 0, core::mem::size_of::<XEvent> ());   // C 518
      let xbutton: *mut XButtonEvent = &raw mut ev as *mut XButtonEvent;
      (*xbutton).display = FRAME_X_DISPLAY! (f);                 // C 519
      (*xbutton).window = XtWindow (menubar);                    // C 520
      (*xbutton).root = (*FRAME_DISPLAY_INFO! (f)).root_window;  // C 521
      #[cfg (not (feature = "have_xinput2"))]                    // C 522-524
      { (*xbutton).time = XtLastTimestampProcessed (FRAME_X_DISPLAY! (f)); }
      #[cfg (feature = "have_xinput2")]                          // C 524-529
      { (*xbutton).time = if (*dpyinfo).supports_xi2
	                     && xi_frame_selected_for (f, XI_KeyPress as c_ulong)
	                       { (*dpyinfo).last_user_time }
	                     else { XtLastTimestampProcessed ((*dpyinfo).display) }; }
      (*xbutton).button = Button1;                               // C 530
      (*xbutton).y = FRAME_MENUBAR_HEIGHT! (f) / 2;              // C 531 (C's `x = y = …`)
      (*xbutton).x = (*xbutton).y;
      (*xbutton).same_screen = True;                             // C 532

      #[cfg (feature = "use_motif")]                             // C 534-544
      {
        let mut al: [Arg; 2] = core::mem::zeroed ();
        let mut list: WidgetList;
        let mut nr: Cardinal;
        XtSetArg! (al[0], XtNchildren, &raw mut list);
        XtSetArg! (al[1], XtNnumChildren, &raw mut nr);
        XtGetValues (menubar, &raw mut al as ArgList, 2);
        (*xbutton).window = XtWindow (*list);
      }

      XTranslateCoordinates (FRAME_X_DISPLAY! (f),
                             /* From-window, to-window.  */
                             (*xbutton).window, (*xbutton).root,

                             /* From-position, to-position.  */
                             (*xbutton).x, (*xbutton).y,
                             &raw mut (*xbutton).x_root, &raw mut (*xbutton).y_root,

                             /* Child of win.  */
                             &raw mut child) as c_int;           // C 546-557
      error_p = x_had_errors_p (FRAME_X_DISPLAY! (f));           // C 558
      x_uncatch_errors_after_check ();                           // C 559

      if !error_p                                                // C 561
        {
          (*(&raw mut ev as *mut XAnyEvent)).r#type = ButtonPress;   // C 563
          (*xbutton).state = 0;                                  // C 564

          XtDispatchEvent (&raw mut ev) as c_int;                // C 566
          (*xbutton).r#type = ButtonRelease;                     // C 567 (`ev.xbutton.type`)
          (*xbutton).state = Button1Mask;                        // C 568
          XtDispatchEvent (&raw mut ev) as c_int;                // C 569
        }
    }

  unblock_input ();                                             // C 572

  return Qnil ();                                               // C 574
});

/* C 578-630: `#ifdef USE_GTK` / the menu-bar `DEFUN`'s GTK arm and `popup_widget_loop` /
   `#endif`.  Both arms are carried (the Xt arm above is the `#else`).  */
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
DEFUN! ("x-menu-bar-open-internal", Fx_menu_bar_open_internal,
       Sx_menu_bar_open_internal, 0, 1, "i",
       doc: "Start key navigation of the menu bar in FRAME.\nThis initially opens the first menu bar item and you can then navigate with the\narrow keys, select a menu entry with the return key or cancel with the\nescape key.  If FRAME has no menu bar this function does nothing.\n\nIf FRAME is nil or not given, use the selected frame.",   // C 580-585
       (frame: Lisp_Object),
       {
  let mut menubar: *mut GtkWidget;                              // C 588
  let mut f: *mut frame;

  block_input ();                                               // C 591
  f = decode_window_system_frame (frame);                       // C 592

  if FRAME_EXTERNAL_MENU_BAR (f)                                // C 594
    { set_frame_menubar (f, true); }

  menubar = (*(FRAME_X_OUTPUT! (f) as *mut x_output)).menubar_widget;   // C 596
  if !menubar.is_null ()                                        // C 597
    {
      /* Activate the first menu.  */
      let mut children: *mut GList
	= gtk_container_get_children (GTK_CONTAINER! (menubar));  // C 600

      if !children.is_null ()                                   // C 602
        {
          g_signal_emit_by_name ((*children).data, c"activate_item".as_ptr ());   // C 604
          popup_activated_flag = 1;                             // C 605
          g_list_free (children);                               // C 606
        }
    }
  unblock_input ();                                             // C 609

  return Qnil ();                                               // C 611
});

/* Loop util popup_activated_flag is set to zero in a callback.
   Used for popup menus and dialogs. */                            // C 613-614
/// C: static void popup_widget_loop (bool do_timers, GtkWidget *widget)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe fn popup_widget_loop (do_timers: bool, widget: *mut GtkWidget)   // C 618-628
{
  popup_activated_flag += 1;                                    // C 620

  /* Process events in the Gtk event loop until done.  */
  while popup_activated_flag != 0                               // C 623
    {
      if do_timers { x_menu_wait_for_event (std::ptr::null_mut ()); }   // C 625
      gtk_main_iteration () as c_int;                           // C 626
    }
}

/* C 632-698: `x_activate_menubar`.  C's `#ifdef USE_GTK` (C 654-658) and
   `#if defined USE_X_TOOLKIT && defined HAVE_XINPUT2` (C 667-684) both carry both arms.  */

/* Activate the menu bar of frame F.
   This is called from keyboard.c when it gets the
   MENU_BAR_ACTIVATE_EVENT out of the Emacs event queue.

   To activate the menu bar, we use the X button-press event
   that was saved in saved_menu_event.
   That makes the toolkit do its thing.

   But first we recompute the menu bar contents (the whole tree).

   The reason for saving the button event until here, instead of
   passing it to the toolkit right away, is that we can safely
   execute Lisp code.  */                                         // C 632-646
/// C: void x_activate_menubar (struct frame *f)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_activate_menubar (f: *mut frame)     // C 647-697
{
  eassert! (FRAME_X_P (f));                                     // C 649

  let saved_menu_event: *mut XEvent
    = (*(FRAME_X_OUTPUT! (f) as *mut x_output)).saved_menu_event;
  let xany: *mut XAnyEvent = saved_menu_event as *mut XAnyEvent;
  if (*xany).r#type == 0                                        // C 651
    { return; }

  #[cfg (feature = "use_gtk")]                                  // C 654-658
  if xg_win_to_widget (FRAME_X_DISPLAY! (f), (*xany).window).is_null ()
    { return; }

  set_frame_menubar (f, true);                                  // C 660
  block_input ();                                               // C 661
  popup_activated_flag = 1;                                     // C 662
  #[cfg (feature = "use_gtk")]                                  // C 664-665
  { XPutBackEvent ((*FRAME_DISPLAY_INFO! (f)).display, saved_menu_event) as c_int; }
  #[cfg (not (feature = "use_gtk"))]
  {
    #[cfg (feature = "have_xinput2")]                           // C 667-684
    {
      /* C 668's `struct x_display_info *dpyinfo = FRAME_DISPLAY_INFO (f);` */
      let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
      /* Clear the XI2 grab so Motif or lwlib can set a core grab.
	 Otherwise some versions of Motif will emit a warning and hang,
	 and lwlib will fail to destroy the menu window.  */

      if (*dpyinfo).supports_xi2
	  && xi_frame_selected_for (f, XI_ButtonPress as c_ulong)
	{
	  let mut i: c_int = 0;
	  while i < (*dpyinfo).num_devices
	    {
	      if (*(*dpyinfo).devices.offset (i as isize)).grab != 0
		{ XIUngrabDevice ((*dpyinfo).display,
				  (*(*dpyinfo).devices.offset (i as isize)).device_id,
				  CurrentTime) as c_int; }
	      i += 1;
	    }
	}
    }
    /* The cascade button might have been deleted, so don't activate the
       popup if it no widget was found to dispatch to.  */
    popup_activated_flag = XtDispatchEvent (saved_menu_event) as c_int;   // C 686-687
  }
  unblock_input ();                                             // C 688

  /* Ignore this if we get it a second time.  */
  (*xany).r#type = 0;                                           // C 691 (`saved_menu_event->type`)
}

/* This callback is invoked when the user selects a menubar cascade
   pushbutton, but before the pulldown menu is posted.  */        // C 693-694
/// C: static void popup_activate_callback (Widget widget, LWLIB_ID id, XtPointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]   // C 696-706
unsafe extern "C" fn popup_activate_callback (widget: Widget, id: LWLIB_ID,
                                              client_data: XtPointer)   // C 701-705
{
  popup_activated_flag = 1;
  x_activate_timeout_atimer ();
}

/* This callback is invoked when a dialog or menu is finished being
   used and has been unposted.  */                               // C 708-709
/// C: static void popup_deactivate_callback (GtkWidget *widget, gpointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]   // C 712-718
unsafe extern "C" fn popup_deactivate_callback (widget: *mut GtkWidget,
                                                client_data: gpointer)   // C 712-722
{
  popup_activated_flag = 0;                                     // C 720
}
/// C: static void popup_deactivate_callback (Widget widget, LWLIB_ID id, XtPointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe extern "C" fn popup_deactivate_callback (widget: Widget, id: LWLIB_ID,
                                                client_data: XtPointer)   // C 712-722
{
  popup_activated_flag = 0;                                     // C 720
}

/* Function that finds the frame for WIDGET and shows the HELP text
   for that widget.
   F is the frame if known, or NULL if not known.  */             // C 724-727
/// C: static void show_help_event (struct frame *f, xt_or_gtk_widget widget, Lisp_Object help)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
unsafe fn show_help_event (f: *mut frame, widget: xt_or_gtk_widget,
                           help: Lisp_Object)                   // C 728-741
{
  let mut frame: Lisp_Object = Qnil ();                          // C 730

  if !f.is_null ()                                              // C 732
    {
      XSETFRAME! (frame, f);
      kbd_buffer_store_help_event (frame, help);                 // C 735
    }
  else
    { show_help_echo (help, Qnil (), Qnil (), Qnil ()); }         // C 738
}

/* Callback called when menu items are highlighted/unhighlighted
   while moving the mouse over them.  WIDGET is the menu bar or menu
   popup widget.  ID is its LWLIB_ID.  CALL_DATA contains a pointer to
   the data structure for the menu item, or null in case of
   unhighlighting.  */                                            // C 743-747
/// C: static void menu_highlight_callback (GtkWidget *widget, gpointer call_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]   // C 748-766
unsafe extern "C" fn menu_highlight_callback (widget: *mut GtkWidget,
                                              call_data: gpointer)
{
  let mut cb_data: *mut xg_menu_item_cb_data;                    // C 751
  let mut help: Lisp_Object;

  cb_data = g_object_get_data (G_OBJECT! (widget), XG_ITEM_DATA)
              as *mut xg_menu_item_cb_data;                      // C 754
  if cb_data.is_null () { return; }                              // C 755 (`if (! cb_data) return;`)

  help = if !call_data.is_null () { (*cb_data).help } else { Qnil () };   // C 757

  /* If popup_activated_flag is greater than 1 we are in a popup menu.
     Don't pass the frame to show_help_event for those.
     Passing frame creates an Emacs event.  As we are looping in
     popup_widget_loop, it won't be handled.  Passing NULL shows the tip
     directly without using an Emacs event.  This is what the Lucid code
     does below.  */                                             // C 759-765
  show_help_event (if popup_activated_flag <= 1 { (*(*cb_data).cl_data).f }
                   else { std::ptr::null_mut () },
                   widget as xt_or_gtk_widget, help);
}
/// C: static void menu_highlight_callback (Widget widget, LWLIB_ID id, void *call_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]   // C 769-779
unsafe extern "C" fn menu_highlight_callback (widget: Widget, id: LWLIB_ID,
                                              call_data: *mut c_void)
{
  let wv: *mut widget_value = call_data as *mut widget_value;    // C 771
  let help: Lisp_Object = if !wv.is_null () { (*wv).help } else { Qnil () };   // C 772

  /* Determine the frame for the help event.  */
  let f: *mut frame = menubar_id_to_frame (id);                   // C 775

  show_help_event (f, widget as xt_or_gtk_widget, help);          // C 777
}

/* C 782-847: `#ifdef USE_GTK` (the `xg_crazy_callback_abort` flag and the GTK
   `menubar_selection_callback`) / `#else /* not USE_GTK */` (the Xt one) / `#endif`.  Both arms are
   carried.  */

#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]   // C 782-785
/* Gtk calls callbacks just because we tell it what item should be
   selected in a radio group.  If this variable is set to a non-zero
   value, we are creating menus and don't want callbacks right now.
*/
static mut xg_crazy_callback_abort: bool = false;               // C 787

/// C: static void menubar_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn menubar_selection_callback (widget: *mut GtkWidget,
                                                 client_data: gpointer)   // C 794-828
{
  let cb_data: *mut xg_menu_item_cb_data = client_data as *mut xg_menu_item_cb_data;   // C 796

  if xg_crazy_callback_abort                                   // C 798
    { return; }

  if cb_data.is_null () || (*cb_data).cl_data.is_null () || (*(*cb_data).cl_data).f.is_null ()
    { return; }                                                // C 800-801

  /* For a group of radio buttons, GTK calls the selection callback first
     for the item that was active before the selection and then for the one that
     is active after the selection.  For C-h k this means we get the help on
     the deselected item and then the selected item is executed.  Prevent that
     by ignoring the non-active item.  */                         // C 803-808
  if GTK_IS_RADIO_MENU_ITEM! (widget)
      && gtk_check_menu_item_get_active (GTK_CHECK_MENU_ITEM! (widget)) == 0
    { return; }

  /* When a menu is popped down, X generates a focus event (i.e. focus
     goes back to the frame below the menu).  Since GTK buffers events,
     we force it out here before the menu selection event.  Otherwise
     sit-for will exit at once if the focus event follows the menu selection
     event.  */                                                   // C 810-816

  block_input ();                                                // C 818
  while gtk_events_pending () != 0
    { gtk_main_iteration () as c_int; }
  unblock_input ();                                              // C 821

  find_and_call_menu_selection ((*(*cb_data).cl_data).f,             // C 823-826
                                (*(*cb_data).cl_data).menu_bar_items_used,
                                (*(*cb_data).cl_data).menu_bar_vector,
                                (*cb_data).call_data as *mut c_void);
}

/* This callback is called from the menu bar pulldown menu
   when the user makes a selection.
   Figure out what the user chose
   and put the appropriate events into the keyboard buffer.  */   // C 831-834
/// C: static void menubar_selection_callback (Widget widget, LWLIB_ID id, XtPointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]   // C 837-845
unsafe extern "C" fn menubar_selection_callback (widget: Widget, id: LWLIB_ID,
                                                 client_data: XtPointer)
{
  let mut f: *mut frame;

  f = menubar_id_to_frame (id);                                  // C 840
  if f.is_null ()                                                // C 841
    { return; }
  find_and_call_menu_selection (f, (*f).menu_bar_items_used,     // C 843-844
                                (*f).menu_bar_vector, client_data);
}

/* Recompute all the widgets of frame F, when the menu bar has been
   changed.  */                                                   // C 849-850
/// C: static void update_frame_menubar (struct frame *f)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
unsafe fn update_frame_menubar (f: *mut frame)                   // C 853-890
{
  #[cfg (feature = "use_gtk")]                                   // C 855-857
  xg_update_frame_menubar (f);
  #[cfg (not (feature = "use_gtk"))]
  {
    let mut x: *mut x_output;                                     // C 859

    eassert! (FRAME_X_P (f));                                     // C 861

    x = FRAME_X_OUTPUT! (f) as *mut x_output;                     // C 863

    if (*x).menubar_widget.is_null () || XtIsManaged ((*x).menubar_widget) != 0
      { return; }                                                 // C 865-866

    block_input ();                                               // C 868

    /* Do the voodoo which means "I'm changing lots of things, don't try
       to refigure sizes until I'm done."  */
    lw_refigure_widget ((*x).column_widget, False as Boolean);    // C 872

    /* The order in which children are managed is the top to bottom
       order in which they are displayed in the paned window.  First,
       remove the text-area widget.  */
    XtUnmanageChild ((*x).edit_widget);                           // C 876

    /* Remove the menubar that is there now, and put up the menubar that
       should be there.  */
    XtManageChild ((*x).menubar_widget);                          // C 880
    XtMapWidget ((*x).menubar_widget);                            // C 881
    XtVaSetValues ((*x).menubar_widget, XtNmappedWhenManaged, 1,
		   std::ptr::null_mut::<c_void> ());   // C 882

    /* Re-manage the text-area widget, and then thrash the sizes.  */
    XtManageChild ((*x).edit_widget);                             // C 885
    lw_refigure_widget ((*x).column_widget, True as Boolean);     // C 886

    /* Force the pane widget to resize itself.  */
    adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ());   // C 889
    unblock_input ();                                             // C 890
  }
}

/* C 892-923: `#ifdef USE_LUCID` / `apply_systemfont_to_dialog`, `apply_systemfont_to_menu` /
   `#endif`.  Both arms are carried; the `#else` arm is empty.  */
/// C: static void apply_systemfont_to_dialog (Widget w)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
unsafe fn apply_systemfont_to_dialog (w: Widget)                 // C 896-905
{
  let fn_: *const c_char = xsettings_get_system_normal_font ();   // C 898
  if !fn_.is_null ()                                             // C 899
    {
      let mut db: XrmDatabase = XtDatabase (XtDisplay (w));        // C 901
      if !db.is_null ()                                           // C 902
        { XrmPutStringResource (&raw mut db, c"*dialog.font".as_ptr (), fn_); }   // C 903
    }
}

/// C: static void apply_systemfont_to_menu (struct frame *f, Widget w)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", feature = "use_lucid"))]
unsafe fn apply_systemfont_to_menu (f: *mut frame, w: Widget)     // C 908-923
{
  let fn_: *const c_char = xsettings_get_system_normal_font ();   // C 911

  if !fn_.is_null ()                                             // C 913
    {
      let mut db: XrmDatabase = XtDatabase (XtDisplay (w));        // C 915
      if !db.is_null ()                                           // C 916
        {
          XrmPutStringResource (&raw mut db, c"*menubar*font".as_ptr (), fn_);   // C 918
          XrmPutStringResource (&raw mut db, c"*popup*font".as_ptr (), fn_);     // C 919
        }
    }
}

/* Set the contents of the menubar widgets of frame F.  */         // C 926
/// C: void set_frame_menubar (struct frame *f, bool deep_p)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_frame_menubar (f: *mut frame, mut deep_p: bool)   // C 928-1250
{
  let mut menubar_widget: xt_or_gtk_widget;                      // C 930
  let mut old_widget: xt_or_gtk_widget;
  #[cfg (feature = "use_x_toolkit")]
  let mut id: LWLIB_ID;                                          // C 932
  let mut items: Lisp_Object;                                    // C 933
  let mut wv: *mut widget_value;                                 // C 934
  let mut first_wv: *mut widget_value;
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();
  let mut i: c_int;                                              // C 935
  let mut submenu_start: *mut c_int;                             // C 936
  let mut submenu_end: *mut c_int;
  let mut submenu_top_level_items: *mut bool;                     // C 937
  let mut submenu_n_panes: *mut c_int;

  eassert! (FRAME_X_P (f));                                      // C 939

  let x_output_: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;   // C 941
  menubar_widget = (*x_output_).menubar_widget;
  old_widget = menubar_widget;

  XSETFRAME! (Vmenu_updating_frame, f);                          // C 943

  #[cfg (feature = "use_x_toolkit")]                             // C 945-951
  {
    if (*x_output_).id == 0
      { (*x_output_).id = next_menubar_widget_id as c_int; next_menubar_widget_id += 1; }
    id = (*x_output_).id as LWLIB_ID;
  }

  if menubar_widget.is_null ()                                   // C 953
    { deep_p = true; }
  /* Make the first call for any given frame always go deep.  */
  else if (*x_output_).saved_menu_event.is_null () && !deep_p    // C 955-961
    {
      deep_p = true;
      (*x_output_).saved_menu_event = xmalloc (core::mem::size_of::<XEvent> ()) as *mut XEvent;
      (*( (*x_output_).saved_menu_event as *mut XAnyEvent )).r#type = 0;
    }

  if deep_p                                                      // C 963
    {
      /* Make a widget-value tree representing the entire menu trees.  */

      let prev: *mut buffer = (current_buffer! ());                     // C 967
      let mut buffer: Lisp_Object;                                // C 968
      let specpdl_count: specpdl_ref = SPECPDL_INDEX ();           // C 969
      let mut previous_menu_items_used: c_int = (*f).menu_bar_items_used;   // C 970
      /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision).  */
      let mut previous_items: *mut Lisp_Object
	= sa_stack_alloc (previous_menu_items_used as usize
			  * core::mem::size_of::<Lisp_Object> ()) as *mut Lisp_Object;   // C 971-972
      let mut subitems: c_int;                                    // C 973

      /* If we are making a new widget, its contents are empty,
	 do always reinitialize them.  */
      if menubar_widget.is_null ()                                // C 977
	{ previous_menu_items_used = 0; }

      buffer = (*XWINDOW (FRAME_SELECTED_WINDOW (f))).contents;   // C 979
      specbind (Qinhibit_quit (), Qt ());                           // C 980
      /* Don't let the debugger step into this code
	 because it is not reentrant.  */
      specbind (Qdebug_on_next_call (), Qnil ());                   // C 983

      record_unwind_save_match_data ();                           // C 985
      if NILP (Voverriding_local_map_menu_flag)                   // C 986
	{
	  specbind (Qoverriding_terminal_local_map (), Qnil ());
	  specbind (Qoverriding_local_map (), Qnil ());
	}

      set_buffer_internal_1 (XBUFFER (buffer));                   // C 992

      /* Run the Lucid hook.  */
      safe_run_hooks (Qactivate_menubar_hook ());                   // C 995

      /* If it has changed current-menubar from previous value,
	 really recompute the menubar from the value.  */
      safe_run_hooks (Qmenu_bar_update_hook ());                    // C 999
      fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 1000

      items = FRAME_MENU_BAR_ITEMS (f);                           // C 1002

      /* Save the frame's previous menu bar contents data.  */
      if previous_menu_items_used != 0                            // C 1005
	{ memcpy (previous_items as *mut c_void,
		  xvector_contents ((*f).menu_bar_vector) as *const c_void,
		  previous_menu_items_used as usize * word_size as usize); }

      /* Fill in menu_items with the current menu bar contents.
	 This can evaluate Lisp code.  */
      save_menu_items ();                                         // C 1012

      menu_items = (*f).menu_bar_vector;                          // C 1014
      menu_items_allocated = if VECTORP (menu_items) { ASIZE (menu_items) as c_int } else { 0 };
      subitems = (ASIZE (items) / 4) as c_int;                     // C 1016
      submenu_start = sa_stack_alloc (((subitems + 1) as usize)
				      * core::mem::size_of::<c_int> ()) as *mut c_int;   // C 1017
      submenu_end = sa_stack_alloc ((subitems as usize)
				    * core::mem::size_of::<c_int> ()) as *mut c_int;     // C 1018
      submenu_n_panes = sa_stack_alloc ((subitems as usize)
					* core::mem::size_of::<c_int> ()) as *mut c_int; // C 1019
      submenu_top_level_items
	= sa_stack_alloc ((subitems as usize)
			  * core::mem::size_of::<bool> ()) as *mut bool;            // C 1020-1021
      init_menu_items ();                                         // C 1022
      i = 0;                                                      // C 1023
      while i < subitems                                        // C 1023-1041
	{
	  let mut key: Lisp_Object;                                 // C 1025
	  let mut string: Lisp_Object;
	  let mut maps: Lisp_Object;

	  key = AREF (items, 4 * i as isize);                        // C 1027
	  string = AREF (items, 4 * i as isize + 1);
	  maps = AREF (items, 4 * i as isize + 2);
	  if NILP (string)                                           // C 1030
	    { break; }

	  *submenu_start.offset (i as isize) = menu_items_used;      // C 1032

	  menu_items_n_panes = 0;                                    // C 1034
	  *submenu_top_level_items.offset (i as isize)
	    = parse_single_submenu (key, string, maps) as bool;      // C 1035-1036
	  *submenu_n_panes.offset (i as isize) = menu_items_n_panes;  // C 1037

	  *submenu_end.offset (i as isize) = menu_items_used;        // C 1039
	  i += 1;
	}

      *submenu_start.offset (i as isize) = -1;                     // C 1043
      finish_menu_items ();                                       // C 1044

      /* Convert menu_items into widget_value trees
	 to display the menu.  This cannot evaluate Lisp code.  */

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());  // C 1049
      (*wv).button_type = BUTTON_TYPE_NONE;
      first_wv = wv;                                              // C 1051

      i = 0;                                                      // C 1053
      while *submenu_start.offset (i as isize) >= 0               // C 1053-1066
	{
	  menu_items_n_panes = *submenu_n_panes.offset (i as isize);
	  wv = digest_single_submenu (*submenu_start.offset (i as isize),
				      *submenu_end.offset (i as isize),
				      *submenu_top_level_items.offset (i as isize));
	  if !prev_wv.is_null ()                                    // C 1059
	    { (*prev_wv).next = wv; }
	  else
	    { (*first_wv).contents = wv; }
	  /* Don't set wv->name here; GC during the loop might relocate it.  */
	  (*wv).enabled = true;                                     // C 1063
	  (*wv).button_type = BUTTON_TYPE_NONE;
	  prev_wv = wv;
	  i += 1;
	}

      set_buffer_internal_1 (prev);                               // C 1068

      /* If there has been no change in the Lisp-level contents
	 of the menu bar, skip redisplaying it.  Just exit.  */

      /* Compare the new menu items with the ones computed last time.  */
      i = 0;                                                      // C 1075
      while i < previous_menu_items_used                          // C 1075-1077
	{
	  if menu_items_used == i
	      || !EQ (*previous_items.offset (i as isize), AREF (menu_items, i as isize))
	    { break; }
	  i += 1;
	}
      if i == menu_items_used && i == previous_menu_items_used && i != 0   // C 1078
	{
	  /* The menu items have not changed.  Don't bother updating
	     the menus in any form, since it would be a no-op.  */
	  free_menubar_widget_value_tree (first_wv);
	  discard_menu_items ();
	  unbind_to (specpdl_count, Qnil ()) as Lisp_Object;
	  return;
	}

      /* The menu items are different, so store them in the frame.  */
      fset_menu_bar_vector (f, menu_items);                        // C 1088
      (*f).menu_bar_items_used = menu_items_used;                  // C 1089

      /* This undoes save_menu_items.  */
      unbind_to (specpdl_count, Qnil ()) as Lisp_Object;           // C 1092

      /* Now GC cannot happen during the lifetime of the widget_value,
	 so it's safe to store data from a Lisp_String.  */
      wv = (*first_wv).contents;                                  // C 1096
      i = 0;                                                      // C 1097
      while i < ASIZE (items) as c_int                            // C 1097-1105
	{
	  let mut string: Lisp_Object;
	  string = AREF (items, i as isize + 1);                      // C 1100
	  if NILP (string)                                           // C 1101
            { break; }
          (*wv).name = SSDATA (string);                             // C 1103
          update_submenu_strings ((*wv).contents);                  // C 1104
          wv = (*wv).next;
	  i += 4;
	}
    }
  else
    {
      /* Make a widget-value tree containing
	 just the top level menu bar strings.  */                 // C 1110-1112

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());  // C 1114
      (*wv).button_type = BUTTON_TYPE_NONE;
      first_wv = wv;

      items = FRAME_MENU_BAR_ITEMS (f);                           // C 1118
      i = 0;                                                      // C 1119
      while i < ASIZE (items) as c_int                            // C 1119-1139
	{
	  let mut string: Lisp_Object;

	  string = AREF (items, i as isize + 1);                      // C 1123
	  if NILP (string)                                           // C 1124
	    { break; }

	  wv = make_widget_value (SSDATA (string), std::ptr::null_mut (), true, Qnil ());   // C 1126
	  (*wv).button_type = BUTTON_TYPE_NONE;
	  /* This prevents lwlib from assuming this
	     menu item is really supposed to be empty.  */
	  /* The intptr_t cast avoids a warning.
	     This value just has to be different from small integers.  */
	  (*wv).call_data = -1 as intptr_t as *mut c_void;            // C 1132

	  if !prev_wv.is_null ()                                     // C 1134
	    { (*prev_wv).next = wv; }
	  else
	    { (*first_wv).contents = wv; }
	  prev_wv = wv;
	  i += 4;
	}

      /* Forget what we thought we knew about what is in the
	 detailed contents of the menu bar menus.
	 Changing the top level always destroys the contents.  */
      (*f).menu_bar_items_used = 0;                               // C 1143
    }

  /* Create or update the menu bar widget.  */

  block_input ();                                                // C 1148

  #[cfg (feature = "use_gtk")]                                   // C 1150-1177
  {
    xg_crazy_callback_abort = true;
    if !menubar_widget.is_null ()
      {
	/* The fourth arg is DEEP_P, which says to consider the entire
	   menu trees we supply, rather than just the menu bar item names.  */
	xg_modify_menubar_widgets (menubar_widget as *mut GtkWidget,
				   f,
				   first_wv,
				   deep_p,
				   G_CALLBACK! (menubar_selection_callback),
				   G_CALLBACK! (popup_deactivate_callback),
				   G_CALLBACK! (menu_highlight_callback));
      }
    else
      {
	menubar_widget
	  = xg_create_widget (c"menubar".as_ptr (), c"menubar".as_ptr (), f, first_wv,
			      G_CALLBACK! (menubar_selection_callback),
			      G_CALLBACK! (popup_deactivate_callback),
			      G_CALLBACK! (menu_highlight_callback)) as xt_or_gtk_widget;

	(*x_output_).menubar_widget = menubar_widget;
      }
  }
  #[cfg (not (feature = "use_gtk"))]                             // C 1179-1248
  {
    if !menubar_widget.is_null ()
      {
	/* Disable resizing (done for Motif!) */
	lw_allow_resizing ((*x_output_).widget, False as Boolean);   // C 1183

	/* The third arg is DEEP_P, which says to consider the entire
	   menu trees we supply, rather than just the menu bar item names.  */
	lw_modify_all_widgets (id, first_wv, deep_p as Boolean);     // C 1187

	/* Re-enable the edit widget to resize.  */
	lw_allow_resizing ((*x_output_).widget, True as Boolean);    // C 1190
      }
    else
      {
	let menuOverride: *const c_char
	  = c"Ctrl<KeyPress>g: MenuGadgetEscape()".as_ptr ();        // C 1194 (C's `char menuOverride[]`)
	let override_: XtTranslations
	  = XtParseTranslationTable (menuOverride);                  // C 1195

	#[cfg (feature = "use_lucid")]                             // C 1197-1199
	apply_systemfont_to_menu (f, (*x_output_).column_widget);
	menubar_widget = lw_create_widget (c"menubar".as_ptr (), c"menubar".as_ptr (), id,
					   first_wv,
					   (*x_output_).column_widget,
					   false as Boolean,
					   Some (popup_activate_callback),
					   Some (menubar_selection_callback),
					   Some (popup_deactivate_callback),
					   Some (menu_highlight_callback)) as xt_or_gtk_widget;   // C 1199-1214
	(*x_output_).menubar_widget = menubar_widget;

	/* Make menu pop down on C-g.  */
	XtOverrideTranslations (menubar_widget, override_);         // C 1217
      }

    {
      let mut menubar_size: c_int;                                 // C 1221
      if !(*x_output_).menubar_widget.is_null ()                   // C 1223
	{ XtRealizeWidget ((*x_output_).menubar_widget); }

      menubar_size                                                // C 1227-1240
	= if !(*x_output_).menubar_widget.is_null ()
	    {
	      /* C reads `menubar_widget->core.height` and, unless `USE_LUCID`, also
	         `->core.border_width`; `Widget` is a pointer to libXt's `_WidgetRec`, whose first
	         member is the `CorePart core` this unit declares (the record is `UNVERIFIED`: no
	         libXt on this machine - see the ledger's `## Owed`).  */
	      let menubar_widget_core: *mut CorePart
		= &raw mut (*( (*x_output_).menubar_widget as *mut _WidgetRec )).core;
	      #[cfg (not (feature = "use_lucid"))]
	      { (*menubar_widget_core).height as c_int
		+ (*menubar_widget_core).border_width as c_int }
	      #[cfg (feature = "use_lucid")]
	      { (*menubar_widget_core).height as c_int }
	    }
	  else { 0 };

      #[cfg (feature = "use_lucid")]                             // C 1233-1246
      /* Experimentally, we now get the right results
	 for -geometry -0-0 without this.  24 Aug 96, rms.
         Maybe so, but the menu bar size is missing the pixels so the
         WM size hints are off by these pixels.  Jan D, oct 2009.  */
      if FRAME_EXTERNAL_MENU_BAR (f)
	{
	  let mut ibw: Dimension = 0;

	  XtVaGetValues ((*x_output_).column_widget,
			 XtNinternalBorderWidth, &raw mut ibw, std::ptr::null_mut ());
	  menubar_size += ibw as c_int;
	}

      FRAME_MENUBAR_HEIGHT! (f) = menubar_size;                   // C 1244
    }
  }

  free_menubar_widget_value_tree (first_wv);                      // C 1250
  update_frame_menubar (f);                                       // C 1251

  #[cfg (feature = "use_gtk")]                                    // C 1253-1255
  { xg_crazy_callback_abort = false; }

  unblock_input ();                                               // C 1257
}

/* Called from Fx_create_frame to create the initial menubar of a frame
   before it is mapped, so that the window is mapped with the menubar already
   there instead of us tacking it on later and thrashing the window after it
   is visible.  */                                                 // C 1259-1263
/// C: void initialize_frame_menubar (struct frame *f)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_frame_menubar (f: *mut frame)   // C 1268-1276
{
  /* This function is called before the first chance to redisplay
     the frame.  It has to be, so the frame will have the right size.  */
  fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 1273
  set_frame_menubar (f, true);                                    // C 1274
}

/* Get rid of the menu bar of frame F, and free its storage.
   This is used when deleting a frame, and when turning off the menu bar.
   For GTK this function is in gtkutil.c.  */                      // C 1279-1281
/// C: void free_frame_menubar (struct frame *f)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]   // C 1283-1367
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_frame_menubar (f: *mut frame)
{
  let mut menubar_widget: Widget;                                 // C 1285
  #[cfg (feature = "use_motif")]                                  // C 1286-1292
  /* Motif automatically shrinks the frame in lw_destroy_all_widgets.
     If we want to preserve the old height, calculate it now so we can
     restore it below.  */
  let mut old_width: c_int = FRAME_TEXT_WIDTH (f);
  #[cfg (feature = "use_motif")]
  let mut old_height: c_int = FRAME_TEXT_HEIGHT (f) + FRAME_MENUBAR_HEIGHT! (f);

  eassert! (FRAME_X_P (f));                                       // C 1294

  let x_output_: *mut x_output = FRAME_X_OUTPUT! (f) as *mut x_output;
  menubar_widget = (*x_output_).menubar_widget;                    // C 1296

  FRAME_MENUBAR_HEIGHT! (f) = 0;                                  // C 1298

  if !menubar_widget.is_null ()                                   // C 1300
    {
      #[cfg (feature = "use_motif")]                              // C 1302
      let mut x0: Position = 0;
      #[cfg (feature = "use_motif")]
      let mut y0: Position = 0;
      #[cfg (feature = "use_motif")]
      let mut x1: Position = 0;
      #[cfg (feature = "use_motif")]
      let mut y1: Position = 0;

      block_input ();                                             // C 1305

      #[cfg (feature = "use_motif")]                              // C 1307-1312
      if !(*x_output_).widget.is_null ()
	{ XtVaGetValues ((*x_output_).widget, XtNx, &raw mut x0, XtNy, &raw mut y0,
			 std::ptr::null_mut ()); }

      lw_destroy_all_widgets ((*x_output_).id as LWLIB_ID);        // C 1314
      (*x_output_).menubar_widget = std::ptr::null_mut ();         // C 1315

      /* When double-buffering is enabled and the frame shall not be
	 resized either because resizing is inhibited or the frame is
	 fullheight, some (usually harmless) display artifacts like a
	 doubled mode line may show up.  Sometimes the configuration
	 gets messed up in a more serious fashion though and you may
	 have to resize the frame to get it back in a normal state.  */
      if !(*x_output_).widget.is_null ()                           // C 1322
	{
	  #[cfg (feature = "use_motif")]                           // C 1324-1350
	  {
	    XtVaGetValues ((*x_output_).widget, XtNx, &raw mut x1, XtNy, &raw mut y1,
			   std::ptr::null_mut ());
	    if x1 == 0 && y1 == 0
	      { XtVaSetValues ((*x_output_).widget, XtNx, x0, XtNy, y0,
			       std::ptr::null_mut ()); }
	    /* When resizing is inhibited and a normal Motif frame is not
	       fullheight, we have to explicitly request its old sizes
	       here since otherwise turning off the menu bar will shrink
	       the frame but turning them on again will not resize it
	       back.  For a fullheight frame we let the window manager
	       deal with this problem.  */
	    if frame_inhibit_resize (f, false, Qmenu_bar_lines ())
		&& !EQ (get_frame_param (f, Qfullscreen ()), Qfullheight ())
	      { adjust_frame_size (f, old_width, old_height, 1, false, Qmenu_bar_lines ()); }
	    else
	      { adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ()); }
	  }
	  #[cfg (not (feature = "use_motif"))]                     // C 1348-1350
	  { adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ()); }
	}
      else
	{
	  #[cfg (feature = "use_motif")]                           // C 1354-1361
	  if WINDOWP (FRAME_ROOT_WINDOW (f))
	      /* See comment above.  */
	      && frame_inhibit_resize (f, false, Qmenu_bar_lines ())
	      && !EQ (get_frame_param (f, Qfullscreen ()), Qfullheight ())
	    { adjust_frame_size (f, old_width, old_height, 1, false, Qmenu_bar_lines ()); }
	}

      unblock_input ();                                           // C 1364
    }
}

/* ==============================================================================================
   C 1388-2879: `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` … `#else /* not USE_X_TOOLKIT &&
   not USE_GTK */` (C 2468) … `#endif` (C 2879).

   The `#if` arm - C 1388-2467 - is below; the `#else` arm (the oldXMenu build) follows it.  */

/* C 1389-1391: `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` and the volatile pointer C's
   `popup_selection_callback` writes and `x_menu_show` reads.  C's `volatile` guards the value
   across `longjmp`; Rust has no `volatile` on a `static mut` (every read is a plain load), so the
   qualifier is recorded in the ledger rather than spelled.  */

/* The item selected in the popup menu.  */
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
static mut menu_item_selection: *mut Lisp_Object = std::ptr::null_mut ();   // C 1391

/* ------------------------------------------------------------------------------------------
   C 1393-1598: `#ifdef USE_GTK` - `struct next_popup_x_y`, `menu_position_func`, the GTK
   `popup_selection_callback` / `pop_down_menu` and `create_and_show_popup_menu`.  */

/* C 1395-1404: `struct next_popup_x_y { struct frame *f; int x; int y; };`.  */
/// C: struct next_popup_x_y { struct frame *f; int x; int y; };
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
#[repr(C)]
pub struct next_popup_x_y
{
  pub f: *mut frame,              // C 1397
  pub x: c_int,                   // C 1398
  pub y: c_int,                   // C 1399
}

/* The menu position function to use if we are not putting a popup
   menu where the pointer is.
   MENU is the menu to pop up.
   X and Y shall on exit contain x/y where the menu shall pop up.
   PUSH_IN is not documented in the GTK manual.
   USER_DATA is any data passed in when calling gtk_menu_popup.
   Here it points to a struct next_popup_x_y where the coordinates
   to store in *X and *Y are as well as the frame for the popup.

   Here only X and Y are used.  */                                // C 1406-1413
/// C: static void menu_position_func (GtkMenu *menu, gint *x, gint *y, gboolean *push_in,
/// gpointer user_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn menu_position_func (menu: *mut GtkMenu, x: *mut gint, y: *mut gint,
                                         push_in: *mut gboolean,
                                         user_data: gpointer)   // C 1415-1471
{
  let data: *mut next_popup_x_y = user_data as *mut next_popup_x_y;   // C 1417
  let mut req: GtkRequisition = core::mem::zeroed ();               // C 1418
  let mut max_x: c_int = -1;                                       // C 1419
  let mut max_y: c_int = -1;                                       // C 1420
  #[cfg (feature = "have_gtk3")]                                   // C 1421-1423
  let mut scale: c_int;

  let mut frame: Lisp_Object;                                      // C 1425
  let mut workarea: Lisp_Object;

  XSETFRAME! (frame, (*data).f);                                   // C 1427

  #[cfg (feature = "have_gtk3")]                                   // C 1429-1431
  { scale = xg_get_scale ((*data).f); }
  /* TODO: Get the monitor workarea directly without calculating other
     items in x-display-monitor-attributes-list. */                // C 1432-1433
  workarea = calln! (Qframe_monitor_workarea (),                   // C 1434-1437
                    Qnil (),
                    make_fixnum ((*data).x as EMACS_INT),
                    make_fixnum ((*data).y as EMACS_INT));

  if CONSP (workarea)                                             // C 1439
    {
      let mut min_x: c_int;                                         // C 1441
      let mut min_y: c_int;

      min_x = XFIXNUM (XCAR (workarea)) as c_int;                   // C 1443
      min_y = XFIXNUM (Fnth (make_fixnum (1), workarea)) as c_int;   // C 1444
      max_x = min_x + XFIXNUM (Fnth (make_fixnum (2), workarea)) as c_int;   // C 1445
      max_y = min_y + XFIXNUM (Fnth (make_fixnum (3), workarea)) as c_int;   // C 1446
    }

  if max_x < 0 || max_y < 0                                          // C 1449
    {
      /* C 1451's `struct x_display_info *dpyinfo = FRAME_DISPLAY_INFO (data->f);`  */
      let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! ((*data).f);

      /* `include/gtk.rs` 1402 declares the two with the P-55 stand-in parameter
         (`*mut c_void`), so the call site converts (P-72 rule 3).  */
      max_x = x_display_pixel_width (dpyinfo as *mut c_void);       // C 1453
      max_y = x_display_pixel_height (dpyinfo as *mut c_void);      // C 1454
    }

  /* frame-monitor-workarea and {x,y}_display_pixel_width/height all
     return device pixels, but GTK wants scaled pixels.  The positions
     passed in via data were already scaled for us.  */             // C 1456-1459
  #[cfg (feature = "have_gtk3")]                                   // C 1460-1463
  {
    max_x /= scale;
    max_y /= scale;
  }
  *x = (*data).x;                                                  // C 1465
  *y = (*data).y;                                                  // C 1466

  /* Check if there is room for the menu.  If not, adjust x/y so that
     the menu is fully visible.  gtk_widget_get_preferred_size returns
     scaled pixels, so there is no need to apply the scaling
     factor.  */                                                   // C 1468-1472
  gtk_widget_get_preferred_size (GTK_WIDGET! (menu), std::ptr::null_mut (), &raw mut req);   // C 1473
  if (*data).x + req.width > max_x                                  // C 1474
    { *x -= (*data).x + req.width - max_x; }
  if (*data).y + req.height > max_y                                 // C 1476
    { *y -= (*data).y + req.height - max_y; }
}

/// C: static void popup_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn popup_selection_callback (widget: *mut GtkWidget,
                                               client_data: gpointer)   // C 1479-1485
{
  let cb_data: *mut xg_menu_item_cb_data = client_data as *mut xg_menu_item_cb_data;   // C 1481

  if xg_crazy_callback_abort { return; }                           // C 1483
  if !cb_data.is_null () { menu_item_selection = (*cb_data).call_data as *mut Lisp_Object; }   // C 1484
}

/// C: static void pop_down_menu (void *arg)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn pop_down_menu (arg: *mut c_void)            // C 1488-1498
{
  popup_activated_flag = 0;                                       // C 1490
  block_input ();                                                 // C 1491
  gtk_widget_destroy (GTK_WIDGET! (arg));                          // C 1492
  unblock_input ();                                               // C 1493
}

/* Pop up the menu for frame F defined by FIRST_WV at X/Y and loop until the
   menu pops down.
   menu_item_selection will be set to the selection.  */           // C 1495-1497
/// C: static void create_and_show_popup_menu (struct frame *f, widget_value *first_wv, int x,
/// int y, bool for_click)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe fn create_and_show_popup_menu (f: *mut frame, first_wv: *mut widget_value,
                                      mut x: c_int, mut y: c_int,
                                      for_click: bool)           // C 1500-1598
{
  let mut i: c_int;                                                // C 1502
  let mut menu: *mut GtkWidget;                                    // C 1503
  let mut pos_func: GtkMenuPositionFunc = None;  /* Pop up at pointer.  */   // C 1504
  let mut popup_x_y: next_popup_x_y = core::mem::zeroed ();         // C 1505
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                // C 1506
  let mut use_pos_func: bool = !for_click;                          // C 1507

  #[cfg (feature = "have_gtk3")]                                   // C 1509-1514
  /* Always use position function for Gtk3.  Otherwise menus may become
     too small to show anything.  */
  { use_pos_func = true; }

  eassert! (FRAME_X_P (f));                                        // C 1516

  xg_crazy_callback_abort = true;                                  // C 1518
  menu = xg_create_widget (c"popup".as_ptr (), (*first_wv).name, f, first_wv,   // C 1519-1522
                           G_CALLBACK! (popup_selection_callback),
                           G_CALLBACK! (popup_deactivate_callback),
                           G_CALLBACK! (menu_highlight_callback));
  xg_crazy_callback_abort = false;                                 // C 1523

  if use_pos_func                                            // C 1525-1538
    {
      /* Not invoked by a click.  pop up at x/y.  */
      pos_func = Some (menu_position_func);                        // C 1528

      /* Adjust coordinates to be root-window-relative.  */
      block_input ();                                              // C 1531
      x_translate_coordinates_to_root (f, x, y, &raw mut x, &raw mut y);   // C 1532
      #[cfg (feature = "have_gtk3")]                               // C 1533-1538
      /* Use window scaling factor to adjust position for scaled
	 outputs.  */
      {
	x /= xg_get_scale (f);
	y /= xg_get_scale (f);
      }
      unblock_input ();                                            // C 1539
      popup_x_y.x = x;                                             // C 1540
      popup_x_y.y = y;                                             // C 1541
      popup_x_y.f = f;                                             // C 1542

      i = 0;  /* gtk_menu_popup needs this to be 0 for a non-button popup.  */   // C 1544
    }

  if for_click                                                     // C 1547
    {
      i = 0;                                                       // C 1549
      while i < 5
        {
          if (*FRAME_DISPLAY_INFO! (f)).grabbed & (1 << i) != 0     // C 1550
            { break; }
          i += 1;
        }
      /* If keys aren't grabbed (i.e., a mouse up event), use 0.  */
      if i == 5 { i = 0; }                                          // C 1553
    }

  #[cfg (all (not (feature = "have_gtk3"), feature = "have_xinput2"))]   // C 1556-1572
  if (*FRAME_DISPLAY_INFO! (f)).supports_xi2
      && xi_frame_selected_for (f, XI_ButtonPress as c_ulong)
    {
      let mut i: c_int = 0;
      while i < (*FRAME_DISPLAY_INFO! (f)).num_devices
	{
	  if (*(*FRAME_DISPLAY_INFO! (f)).devices.offset (i as isize)).grab != 0
	    {
	      (*(*FRAME_DISPLAY_INFO! (f)).devices.offset (i as isize)).grab = 0;

	      XIUngrabDevice (FRAME_X_DISPLAY! (f),
			      (*(*FRAME_DISPLAY_INFO! (f)).devices.offset (i as isize)).device_id,
			      CurrentTime) as c_int;
	    }
	  i += 1;
	}
    }

  DEFER_SELECTIONS! ();                                                // C 1574

  /* Display the menu.  */
  gtk_widget_show_all (menu);                                      // C 1577

  gtk_menu_popup (GTK_MENU! (menu), std::ptr::null_mut (), std::ptr::null_mut (),   // C 1579-1580
		  pos_func, &raw mut popup_x_y as gpointer, i as c_uint,
		  (*FRAME_DISPLAY_INFO! (f)).last_user_time as c_uint);

  record_unwind_protect_ptr (Some (pop_down_menu), menu as *mut c_void);   // C 1582

  if gtk_widget_get_mapped (menu) != 0                             // C 1584
    {
      /* Set this to one.  popup_widget_loop increases it by one, so it becomes
         two.  show_help_echo uses this to detect popup menus.  */
      popup_activated_flag = 1;                                     // C 1588
      /* Process events that apply to the menu.  */
      popup_widget_loop (true, menu);                               // C 1590
    }

  unbind_to (specpdl_count, Qnil ()) as Lisp_Object;               // C 1593

  /* Must reset this manually because the button release event is not passed
     to Emacs event loop. */
  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                          // C 1597
}

/* ------------------------------------------------------------------------------------------
   C 1600-1884: `#else /* not USE_GTK */` - the Xt arm: `widget_id_tick`, the Xt
   `popup_selection_callback`, the two `HAVE_XINPUT2` helpers, `pop_down_menu`,
   `server_timestamp_predicate` and the Xt `create_and_show_popup_menu`.  */

/* We need a unique id for each widget handled by the Lucid Widget
   library.

   For the main windows, and popup menus, we use this counter, which we
   increment each time after use.  This starts from WIDGET_ID_TICK_START.

   For menu bars, we use numbers starting at 0, counted in
   next_menubar_widget_id.  */                                     // C 1602-1608
/// C: extern LWLIB_ID widget_id_tick; /* FIXME: Move this to a .h file.  */
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]   // C 1610-1611
#[unsafe(no_mangle)]
pub static mut widget_id_tick: LWLIB_ID = 0;

/// C: static void popup_selection_callback (Widget widget, LWLIB_ID id, XtPointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe extern "C" fn popup_selection_callback (widget: Widget, id: LWLIB_ID,
                                               client_data: XtPointer)   // C 1614-1618
{
  menu_item_selection = client_data as *mut Lisp_Object;            // C 1616
}

#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit",
            not (feature = "use_gtk"), feature = "have_xinput2"))]                            // C 1620-1710
/// C: static void prepare_for_entry_into_toolkit_menu (struct frame *f)
unsafe extern "C" fn prepare_for_entry_into_toolkit_menu (f: *mut frame)   // C 1623-1657
{
  let mut mask: XIEventMask = core::mem::zeroed ();                 // C 1625
  let l: ptrdiff_t = XIMaskLen! (XI_LASTEVENT) as ptrdiff_t;        // C 1626
  let mut m: *mut c_uchar;
  let mut tail: Lisp_Object;                                        // C 1628
  let mut frame: Lisp_Object;
  let mut dpyinfo: *mut x_display_info;

  dpyinfo = FRAME_DISPLAY_INFO! (f);                                // C 1631

  if !(*dpyinfo).supports_xi2                                       // C 1633
    { return; }

  mask.mask = sa_stack_alloc (l as usize) as *mut c_uchar;          // C 1635-1636
  m = mask.mask;
  memset (m as *mut c_void, 0, l as usize);                          // C 1637
  mask.mask_len = l as c_int;                                        // C 1638

  mask.deviceid = XIAllMasterDevices;                                // C 1640

  XISetMask! (m, XI_Motion);                                         // C 1642
  XISetMask! (m, XI_Enter);                                          // C 1643
  XISetMask! (m, XI_Leave);                                          // C 1644

  FOR_EACH_FRAME! (tail, frame, {                                    // C 1646-1655
      f = XFRAME (frame);                                            // C 1648

      if FRAME_X_P (f)
	  && FRAME_DISPLAY_INFO! (f) == dpyinfo
	  && !FRAME_TOOLTIP_P (f)
	{ XISelectEvents (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
			  &raw mut mask, 1) as c_int; }
    });
}

/// C: static void leave_toolkit_menu (void *data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit",
            not (feature = "use_gtk"), feature = "have_xinput2"))]
unsafe extern "C" fn leave_toolkit_menu (data: *mut c_void)       // C 1659-1706
{
  let mut mask: XIEventMask = core::mem::zeroed ();                 // C 1661
  let l: ptrdiff_t = XIMaskLen! (XI_LASTEVENT) as ptrdiff_t;        // C 1662
  let mut m: *mut c_uchar;
  let mut tail: Lisp_Object;                                        // C 1664
  let mut frame: Lisp_Object;
  let mut dpyinfo: *mut x_display_info;
  let mut f: *mut frame;                                            // C 1667

  dpyinfo = FRAME_DISPLAY_INFO! (data as *mut frame);                // C 1669

  if !(*dpyinfo).supports_xi2                                       // C 1671
    { return; }

  mask.mask = sa_stack_alloc (l as usize) as *mut c_uchar;          // C 1673-1674
  m = mask.mask;
  memset (m as *mut c_void, 0, l as usize);                          // C 1675
  mask.mask_len = l as c_int;                                        // C 1676

  mask.deviceid = XIAllMasterDevices;                                // C 1678

  XISetMask! (m, XI_ButtonPress);                                    // C 1680
  XISetMask! (m, XI_ButtonRelease);                                  // C 1681
  XISetMask! (m, XI_Motion);                                         // C 1682
  XISetMask! (m, XI_Enter);                                          // C 1683
  XISetMask! (m, XI_Leave);                                          // C 1684

  #[cfg (feature = "have_xinput2_4")]                                // C 1685-1696
  /* Select for gesture events.  Emacs selects for gesture events from
     all master devices on non-GTK3 builds, so that event mask is also
     clobbered by prepare_for_entry_into_toolkit_menu.  (bug#65129) */

  if (*dpyinfo).xi2_version >= 4
    {
      XISetMask! (m, XI_GesturePinchBegin);
      XISetMask! (m, XI_GesturePinchUpdate);
      XISetMask! (m, XI_GesturePinchEnd);
    }

  FOR_EACH_FRAME! (tail, frame, {                                    // C 1698-1705
      f = XFRAME (frame);                                            // C 1700

      if FRAME_X_P (f)
	  && FRAME_DISPLAY_INFO! (f) == dpyinfo
	  && !FRAME_TOOLTIP_P (f)
	{ XISelectEvents (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
			  &raw mut mask, 1) as c_int; }
    });
}

/* ID is the LWLIB ID of the dialog box.  */                        // C 1712
/// C: static void pop_down_menu (int id)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe extern "C" fn pop_down_menu (id: c_int)                    // C 1714-1721
{
  block_input ();                                                  // C 1716
  lw_destroy_all_widgets (id as LWLIB_ID);                          // C 1717
  unblock_input ();                                                // C 1718
  popup_activated_flag = 0;                                        // C 1719
}

/// C: static Bool server_timestamp_predicate (Display *display, XEvent *xevent, XPointer arg)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit",
            not (feature = "use_gtk"),
            feature = "have_xinput2", feature = "use_motif"))]     // C 1723-1738
unsafe extern "C" fn server_timestamp_predicate (display: *mut Display,
                                                 xevent: *mut XEvent,
                                                 arg: XPointer) -> Bool
{
  let args: *mut XID = arg as *mut XID;                             // C 1727
  let xproperty: *mut XPropertyEvent = xevent as *mut XPropertyEvent;

  if (*xevent as *mut XAnyEvent).r#type == PropertyNotify
      && (*xproperty).window == *args.offset (0)                    // C 1731
      && (*xproperty).atom == *args.offset (1)                      // C 1732
    { return True; }

  return False;                                                     // C 1735
}

/* Pop up the menu for frame F defined by FIRST_WV at X/Y and loop until the
   menu pops down.
   menu_item_selection will be set to the selection.  */            // C 1740-1742
/// C: static void create_and_show_popup_menu (struct frame *f, widget_value *first_wv, int x,
/// int y, bool for_click)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe fn create_and_show_popup_menu (f: *mut frame, first_wv: *mut widget_value,
                                      mut x: c_int, mut y: c_int,
                                      for_click: bool)           // C 1744-1883
{
  let mut i: c_int;                                                 // C 1746
  let mut av: [Arg; 2] = core::mem::zeroed ();                       // C 1747
  let mut ac: c_int = 0;                                            // C 1748
  let mut dummy: XEvent = core::mem::zeroed ();                      // C 1749
  let mut event: *mut XButtonPressedEvent
    = &raw mut (*(&raw mut dummy as *mut XButtonEvent));             // C 1750
  let mut menu_id: LWLIB_ID;                                        // C 1751
  let mut menu: Widget;                                             // C 1752
  #[cfg (all (feature = "have_xinput2", feature = "use_motif"))]     // C 1754-1757
  let mut property_dummy: XEvent = core::mem::zeroed ();
  #[cfg (all (feature = "have_xinput2", feature = "use_motif"))]
  let mut property_atom: Atom;

  eassert! (FRAME_X_P (f));                                         // C 1759

  #[cfg (feature = "use_lucid")]                                    // C 1761-1763
  apply_systemfont_to_menu (f, (*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget);

  menu_id = widget_id_tick; widget_id_tick += 1;                     // C 1765
  menu = lw_create_widget (c"popup".as_ptr (), (*first_wv).name, menu_id, first_wv,   // C 1766-1770
                           (*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget, True as Boolean, None,
                           Some (popup_selection_callback),
                           Some (popup_deactivate_callback),
                           Some (menu_highlight_callback));

  (*event).r#type = ButtonPress;                                     // C 1772
  (*event).serial = 0;                                               // C 1773
  (*event).send_event = False;                                       // C 1774
  (*event).display = FRAME_X_DISPLAY! (f);                            // C 1775
  (*event).time = CurrentTime;                                        // C 1776
  (*event).root = (*FRAME_DISPLAY_INFO! (f)).root_window;             // C 1777
  (*event).window = (*event).root; (*event).subwindow = (*event).root;   // C 1778
  (*event).x = x;                                                    // C 1779
  (*event).y = y;                                                    // C 1780

  /* Adjust coordinates to be root-window-relative.  */
  block_input ();                                                    // C 1783
  x += FRAME_LEFT_SCROLL_BAR_AREA_WIDTH (f);                          // C 1784
  x_translate_coordinates_to_root (f, x, y, &raw mut x, &raw mut y);   // C 1785
  unblock_input ();                                                  // C 1786

  (*event).x_root = x;                                               // C 1788
  (*event).y_root = y;                                               // C 1789

  (*event).state = 0;                                                // C 1791
  (*event).button = 0;                                               // C 1792
  i = 0;                                                             // C 1793
  while i < 5
    {
      if (*FRAME_DISPLAY_INFO! (f)).grabbed & (1 << i) != 0
	{ (*event).button = i as c_uint; }
      i += 1;
    }

  /* Don't allow any geometry request from the user.  */
  XtSetArg! (av[ac as usize], XtNgeometry, 0); ac += 1;               // C 1798
  XtSetValues (menu, &raw mut av as ArgList, ac as Cardinal);         // C 1799

  #[cfg (feature = "have_xinput2")]                                  // C 1801-1845
  {
    let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);

    /* Clear the XI2 grab, and if any XI2 grab was set, place a core
       grab on the frame's edit widget.  */
    if (*dpyinfo).supports_xi2
      { XGrabServer ((*dpyinfo).display) as c_int; }                  // C 1806

    if (*dpyinfo).supports_xi2
	&& xi_frame_selected_for (f, XI_ButtonPress as c_ulong)
      {
	let mut i: c_int = 0;
	while i < (*dpyinfo).num_devices
	  {
	    if (*(*dpyinfo).devices.offset (i as isize)).grab != 0
	      {
		(*(*dpyinfo).devices.offset (i as isize)).grab = 0;

		XIUngrabDevice ((*dpyinfo).display,
				(*(*dpyinfo).devices.offset (i as isize)).device_id,
				CurrentTime) as c_int;
	      }
	    i += 1;
	  }
      }

    #[cfg (feature = "use_motif")]                                  // C 1825-1844
    if (*dpyinfo).supports_xi2
      {
	/* Dispatch a PropertyNotify to Xt with the current server time.
	   Motif tries to set a grab with the timestamp of the last event
	   processed by Xt, but Xt doesn't consider GenericEvents, so the
	   timestamp is always less than the last grab time.  */

	property_atom = (*dpyinfo).Xatom_EMACS_SERVER_TIME_PROP;       // C 1832

	XChangeProperty ((*dpyinfo).display, FRAME_OUTER_WINDOW! (f),   // C 1834-1836
			 property_atom, XA_ATOM, 32,
			 PropModeReplace, &raw mut property_atom as *mut c_uchar, 1) as c_int;

	let mut args: [XID; 2] = [FRAME_OUTER_WINDOW! (f), property_atom];
	XIfEvent ((*dpyinfo).display, &raw mut property_dummy,
		  Some (server_timestamp_predicate),
		  &raw mut args as XPointer) as c_int;                  // C 1838-1839

	XtDispatchEvent (&raw mut property_dummy) as c_int;            // C 1841
      }
  }

  #[cfg (feature = "have_xinput2")]                                  // C 1847-1861
  {
    prepare_for_entry_into_toolkit_menu (f);

    #[cfg (feature = "use_lucid")]                                   // C 1850-1853
    if (*FRAME_DISPLAY_INFO! (f)).supports_xi2
      { x_mouse_leave (FRAME_DISPLAY_INFO! (f)); }
  }
  /* Display the menu.  */
  lw_popup_menu (menu, &raw mut dummy);                              // C 1857

  #[cfg (feature = "have_xinput2")]                                   // C 1858-1861
  if (*FRAME_DISPLAY_INFO! (f)).supports_xi2
    { XUngrabServer (FRAME_X_DISPLAY! (f)) as c_int; }

  popup_activated_flag = 1;                                           // C 1864

  x_activate_timeout_atimer ();                                       // C 1866

  {                                                                   // C 1868-1881
    let specpdl_count: specpdl_ref = SPECPDL_INDEX ();

    DEFER_SELECTIONS! ();

    record_unwind_protect_int (Some (pop_down_menu), menu_id as c_int);   // C 1874
    #[cfg (feature = "have_xinput2")]                                 // C 1875-1877
    record_unwind_protect_ptr (Some (leave_toolkit_menu), f as *mut c_void);

    /* Process events that apply to the menu.  */
    popup_get_selection (std::ptr::null_mut (), FRAME_DISPLAY_INFO! (f), menu_id, true);   // C 1880

    unbind_to (specpdl_count, Qnil ()) as Lisp_Object;                 // C 1882
  }
}

#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]   // C 1886-1890
/// C: static void cleanup_widget_value_tree (void *arg)
unsafe extern "C" fn cleanup_widget_value_tree (arg: *mut c_void)
{
  free_menubar_widget_value_tree (arg as *mut widget_value);       // C 1888
}

/* x_menu_show actually displays a menu using the panes and items in menu_items
   and returns the value selected from it.
   There are two versions of x_menu_show, one for Xt and one for Xlib.
   Both assume input is blocked by the caller.  */                  // C 1371-1374

/* F is the frame the menu is for.
   X and Y are the frame-relative specified position,
   relative to the inside upper left corner of the frame F.
   Bitfield MENUFLAGS bits are:
   MENU_FOR_CLICK is set if this menu was invoked for a mouse click.
   MENU_KEYMAPS is set if this menu was specified with keymaps;
    in that case, we return a list containing the chosen item's value
    and perhaps also the pane's prefix.
   TITLE is the specified menu title.
   ERROR is a place to store an error message string in case of failure.
   (We return nil on failure, but the value doesn't actually matter.)  */   // C 1376-1386
/// C: Lisp_Object x_menu_show (struct frame *f, int x, int y, int menuflags, Lisp_Object title,
/// const char **error_name)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_menu_show (f: *mut frame, mut x: c_int, y: c_int,
                                      menuflags: c_int, title: Lisp_Object,
                                      error_name: *mut *const c_char) -> Lisp_Object   // C 1892-2166
{
  let mut i: c_int;                                                 // C 1894
  let mut wv: *mut widget_value;                                    // C 1895
  let mut save_wv: *mut widget_value = std::ptr::null_mut ();
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();
  let mut submenu_stack: *mut *mut widget_value = std::ptr::null_mut ();   // C 1896
  let mut subprefix_stack: *mut Lisp_Object = std::ptr::null_mut ();   // C 1897
  let mut submenu_depth: c_int = 0;                                 // C 1898
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                            // C 1899
  SAFE_NALLOCA! (sa_avail, submenu_stack, 1, menu_items_used);      // C 1900
  SAFE_NALLOCA! (sa_avail, subprefix_stack, 1, menu_items_used);    // C 1901
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                // C 1902

  eassert! (FRAME_X_P (f));                                         // C 1904

  *error_name = std::ptr::null ();                                  // C 1906

  if menu_items_used <= MENU_ITEMS_PANE_LENGTH                      // C 1908
    {
      *error_name = c"Empty menu".as_ptr ();                         // C 1910
      SAFE_FREE! (sa_count, sa_mark);                                        // C 1911
      return Qnil ();
    }

  block_input ();                                                   // C 1914

  /* Create a tree of widget_value objects
     representing the panes and their items.  */
  wv = make_widget_value (c"menu".as_ptr (), std::ptr::null_mut (), true, Qnil ());   // C 1918
  (*wv).button_type = BUTTON_TYPE_NONE;
  first_wv = wv;                                                    // C 1920
  let mut first_pane: bool = true;                                  // C 1921

  /* Loop over all panes and items, filling in the tree.  */
  i = 0;                                                            // C 1924
  while i < menu_items_used                                       // C 1924-2090
    {
      if NILP (AREF (menu_items, i as isize))                       // C 1926
	{
	  *submenu_stack.offset (submenu_depth as isize) = save_wv;    // C 1928
	  submenu_depth += 1;
	  save_wv = prev_wv;
	  prev_wv = std::ptr::null_mut ();
	  first_pane = true;
	  i += 1;
	}
      else if EQ (AREF (menu_items, i as isize), Qlambda ())        // C 1933
	{
	  prev_wv = save_wv;                                          // C 1935
	  submenu_depth -= 1;
	  save_wv = *submenu_stack.offset (submenu_depth as isize);
	  first_pane = false;
	  i += 1;
	}
      else if EQ (AREF (menu_items, i as isize), Qt ()) && submenu_depth != 0   // C 1940-1941
	{ i += MENU_ITEMS_PANE_LENGTH; }
      /* Ignore a nil in the item list.
	 It's meaningful only for dialog boxes.  */
      else if EQ (AREF (menu_items, i as isize), Qquote ())         // C 1944
	{ i += 1; }
      else if EQ (AREF (menu_items, i as isize), Qt ())             // C 1946
	{
	  /* Create a new pane.  */
	  let mut pane_name: Lisp_Object;                              // C 1949
	  let mut prefix: Lisp_Object;
	  let mut pane_string: *const c_char;

	  pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);   // C 1951
	  prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);    // C 1952

	  #[cfg (not (feature = "have_multilingual_menu"))]            // C 1954-1961
	  if STRINGP (pane_name) && STRING_MULTIBYTE (pane_name)
	    {
	      pane_name = ENCODE_MENU_STRING (pane_name);
	      ASET (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize, pane_name);
	    }
	  pane_string = if NILP (pane_name) { c"".as_ptr () } else { SSDATA (pane_name) };   // C 1963-1964
	  /* If there is just one top-level pane, put all its items directly
	     under the top-level menu.  */
	  if menu_items_n_panes == 1                                   // C 1967
	    { pane_string = c"".as_ptr (); }

	  /* If the pane has a meaningful name,
	     make the pane a top-level menu item
	     with its items as a submenu beneath it.  */
	  if (menuflags & MENU_KEYMAPS) == 0 && strcmp (pane_string, c"".as_ptr ()) != 0   // C 1973
	    {
	      wv = make_widget_value (pane_string, std::ptr::null_mut (), true, Qnil ());   // C 1975
	      if !save_wv.is_null ()
		{ (*save_wv).next = wv; }
	      else
		{ (*first_wv).contents = wv; }
	      if (menuflags & MENU_KEYMAPS) != 0 && !NILP (prefix)      // C 1979
		{ (*wv).name = (*wv).name.offset (1); }
	      (*wv).button_type = BUTTON_TYPE_NONE;                     // C 1981
	      save_wv = wv;
	      prev_wv = std::ptr::null_mut ();
	    }
	  else if first_pane                                          // C 1984
	    {
	      save_wv = wv;
	      prev_wv = std::ptr::null_mut ();
	    }
	  first_pane = false;                                          // C 1989
	  i += MENU_ITEMS_PANE_LENGTH;
	}
      else
	{
	  /* Create a new item within current pane.  */
	  let mut item_name: Lisp_Object;                              // C 1994
	  let mut enable: Lisp_Object;
	  let mut descrip: Lisp_Object;
	  let mut def: Lisp_Object;
	  let mut r#type: Lisp_Object;
	  let mut selected: Lisp_Object;
	  let mut help: Lisp_Object;
	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 1995
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);
	  descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);
	  def = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_DEFINITION as isize);
	  r#type = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_TYPE as isize);
	  selected = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_SELECTED as isize);
	  help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);

	  #[cfg (not (feature = "have_multilingual_menu"))]            // C 2003-2023
	  {
            if STRINGP (item_name) && STRING_MULTIBYTE (item_name)
	      {
	        item_name = ENCODE_MENU_STRING (item_name);
	        ASET (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize, item_name);
	      }

            if STRINGP (descrip) && STRING_MULTIBYTE (descrip)
	      {
	        descrip = ENCODE_MENU_STRING (descrip);
	        ASET (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize, descrip);
	      }
	  }

	  wv = make_widget_value (SSDATA (item_name), std::ptr::null_mut (), !NILP (enable),   // C 2025-2026
				  if STRINGP (help) { help } else { Qnil () });
	  if !prev_wv.is_null ()                                       // C 2027
	    { (*prev_wv).next = wv; }
	  else if save_wv.is_null ()                                   // C 2029
	    {
	      /* This emacs_abort call pacifies gcc 11.2.1 when Emacs
		 is configured with --enable-gcc-warnings.  FIXME: If
		 save_wv can be null, do something better; otherwise,
		 explain why save_wv cannot be null.  */
	      emacs_abort ();                                          // C 2035
	    }
	  else
	    { (*save_wv).contents = wv; }
	  if !NILP (descrip)                                           // C 2039
	    { (*wv).key = SSDATA (descrip); }
	  /* If this item has a null value,
	     make the call_data null so that it won't display a box
	     when the mouse is on it.  */
	  (*wv).call_data = if !NILP (def) { aref_addr (menu_items, i as isize) as *mut c_void }
			    else { std::ptr::null_mut () };             // C 2043

	  if NILP (r#type)                                             // C 2045
	    { (*wv).button_type = BUTTON_TYPE_NONE; }
	  else if EQ (r#type, QCtoggle ())
	    { (*wv).button_type = BUTTON_TYPE_TOGGLE; }
	  else if EQ (r#type, QCradio ())
	    { (*wv).button_type = BUTTON_TYPE_RADIO; }
	  else
	    { emacs_abort (); }                                        // C 2053

	  (*wv).selected = !NILP (selected);                            // C 2055

	  prev_wv = wv;                                                // C 2057

	  i += MENU_ITEMS_ITEM_LENGTH;                                 // C 2059
	}
    }

  /* Deal with the title, if it is non-nil.  */
  if !NILP (title)                                                 // C 2063
    {
      let mut wv_title: *mut widget_value;                          // C 2065
      let mut wv_sep1: *mut widget_value
	= make_widget_value (c"--".as_ptr (), std::ptr::null_mut (), false, Qnil ());
      let mut wv_sep2: *mut widget_value
	= make_widget_value (c"--".as_ptr (), std::ptr::null_mut (), false, Qnil ());   // C 2066-2067

      (*wv_sep2).next = (*first_wv).contents;                       // C 2069
      (*wv_sep1).next = wv_sep2;                                    // C 2070

      #[cfg (not (feature = "have_multilingual_menu"))]            // C 2072-2076
      let mut title: Lisp_Object = title;
      #[cfg (not (feature = "have_multilingual_menu"))]
      if STRING_MULTIBYTE (title)
	{ title = ENCODE_MENU_STRING (title); }

      wv_title = make_widget_value (SSDATA (title), std::ptr::null_mut (), true, Qnil ());   // C 2078
      (*wv_title).button_type = BUTTON_TYPE_NONE;
      (*wv_title).next = wv_sep1;                                   // C 2080
      (*first_wv).contents = wv_title;                              // C 2081
    }

  /* No selection has been chosen yet.  */
  menu_item_selection = std::ptr::null_mut ();                      // C 2085

  /* Make sure to free the widget_value objects we used to specify the
     contents even with longjmp.  */
  record_unwind_protect_ptr (Some (cleanup_widget_value_tree), first_wv as *mut c_void);   // C 2089

  /* Actually create and show the menu until popped down.  */
  create_and_show_popup_menu (f, first_wv, x, y, (menuflags & MENU_FOR_CLICK) != 0);   // C 2092-2093

  unbind_to (specpdl_count, Qnil ()) as Lisp_Object;                 // C 2095

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if !menu_item_selection.is_null ()                                // C 2099
    {
      let mut prefix: Lisp_Object;                                   // C 2101
      let mut entry: Lisp_Object;

      prefix = Qnil (); entry = Qnil ();                             // C 2103
      i = 0;                                                         // C 2104
      while i < menu_items_used                                    // C 2104-2140
	{
	  if NILP (AREF (menu_items, i as isize))                     // C 2106
	    {
	      *subprefix_stack.offset (submenu_depth as isize) = prefix;   // C 2108
	      submenu_depth += 1;
	      prefix = entry;
	      i += 1;
	    }
	  else if EQ (AREF (menu_items, i as isize), Qlambda ())       // C 2112
	    {
	      submenu_depth -= 1;
	      prefix = *subprefix_stack.offset (submenu_depth as isize);   // C 2114
	      i += 1;
	    }
	  else if EQ (AREF (menu_items, i as isize), Qt ())            // C 2116
	    {
	      prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);   // C 2118
	      i += MENU_ITEMS_PANE_LENGTH;
	    }
	  /* Ignore a nil in the item list.
	     It's meaningful only for dialog boxes.  */
	  else if EQ (AREF (menu_items, i as isize), Qquote ())        // C 2122
	    { i += 1; }
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 2126
	      if menu_item_selection == aref_addr (menu_items, i as isize)   // C 2127
		{
		  if (menuflags & MENU_KEYMAPS) != 0                   // C 2129
		    {
		      let mut j: c_int;

		      entry = list1 (entry);                            // C 2133
		      if !NILP (prefix)
			{ entry = Fcons (prefix, entry); }
		      j = submenu_depth - 1;                            // C 2136
		      while j >= 0
			{
			  if !NILP (*subprefix_stack.offset (j as isize))
			    { entry = Fcons (*subprefix_stack.offset (j as isize), entry); }
			  j -= 1;
			}
		    }
		  unblock_input ();                                     // C 2143

		  SAFE_FREE! (sa_count, sa_mark);                                // C 2145
		  return entry;
		}
	      i += MENU_ITEMS_ITEM_LENGTH;                             // C 2148
	    }
	}
    }
  else if (menuflags & MENU_FOR_CLICK) == 0                         // C 2152
    {
      unblock_input ();                                             // C 2154
      /* Make "Cancel" equivalent to C-g.  */
      quit ();                                                      // C 2156
    }

  unblock_input ();                                                 // C 2159

  SAFE_FREE! (sa_count, sa_mark);                                            // C 2161
  return Qnil ();                                                   // C 2162
}

/* C 2167-2267: `#ifdef USE_GTK` (the GTK dialog callback and `create_and_show_dialog`) /
   `#else /* not USE_GTK */` (the Xt pair, with C 2238-2240's `#ifdef USE_LUCID` arm inside) /
   `#endif`.  Both arms are carried.  */
/// C: static void dialog_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe extern "C" fn dialog_selection_callback (widget: *mut GtkWidget,
                                                client_data: gpointer)   // C 2169-2181
{
  /* Treat the pointer as an integer.  There's no problem
     as long as pointers have enough bits to hold small integers.  */
  if client_data as intptr_t != -1                                 // C 2172-2173
    { menu_item_selection = client_data as *mut Lisp_Object; }

  popup_activated_flag = 0;                                        // C 2176
}

/* Pop up the dialog for frame F defined by FIRST_WV and loop until the
   dialog pops down.
   menu_item_selection will be set to the selection.  */            // C 2178-2180
/// C: static void create_and_show_dialog (struct frame *f, widget_value *first_wv)
#[cfg (all (feature = "have_x_windows", feature = "use_gtk"))]
unsafe fn create_and_show_dialog (f: *mut frame, first_wv: *mut widget_value)   // C 2183-2210
{
  let mut menu: *mut GtkWidget;                                     // C 2185

  eassert! (FRAME_X_P (f));                                         // C 2187

  menu = xg_create_widget (c"dialog".as_ptr (), (*first_wv).name, f, first_wv,   // C 2189-2192
                           G_CALLBACK! (dialog_selection_callback),
                           G_CALLBACK! (popup_deactivate_callback),
                           None);

  if !menu.is_null ()                                               // C 2194
    {
      let specpdl_count: specpdl_ref = SPECPDL_INDEX ();            // C 2196

      DEFER_SELECTIONS! ();                                             // C 2198
      record_unwind_protect_ptr (Some (pop_down_menu), menu as *mut c_void);   // C 2199

      /* Display the menu.  */
      gtk_widget_show_all (menu);                                   // C 2202

      /* Process events that apply to the menu.  */
      popup_widget_loop (true, menu);                               // C 2205

      unbind_to (specpdl_count, Qnil ()) as Lisp_Object;             // C 2207
    }
}

/// C: static void dialog_selection_callback (Widget widget, LWLIB_ID id, XtPointer client_data)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe extern "C" fn dialog_selection_callback (widget: Widget, id: LWLIB_ID,
                                                client_data: XtPointer)   // C 2213-2228
{
  /* Treat the pointer as an integer.  There's no problem
     as long as pointers have enough bits to hold small integers.  */
  if client_data as intptr_t != -1                                 // C 2216-2217
    { menu_item_selection = client_data as *mut Lisp_Object; }

  block_input ();                                                   // C 2219
  lw_destroy_all_widgets (id);                                      // C 2220
  unblock_input ();                                                 // C 2221
  popup_activated_flag = 0;                                         // C 2222
}

/* Pop up the dialog for frame F defined by FIRST_WV and loop until the
   dialog pops down.
   menu_item_selection will be set to the selection.  */            // C 2224-2226
/// C: static void create_and_show_dialog (struct frame *f, widget_value *first_wv)
#[cfg (all (feature = "have_x_windows", feature = "use_x_toolkit", not (feature = "use_gtk")))]
unsafe fn create_and_show_dialog (f: *mut frame, first_wv: *mut widget_value)   // C 2230-2266
{
  let mut dialog_id: LWLIB_ID;                                      // C 2232

  eassert! (FRAME_X_P (f));                                         // C 2234

  dialog_id = widget_id_tick; widget_id_tick += 1;                   // C 2236
  #[cfg (feature = "use_lucid")]                                    // C 2238-2240
  apply_systemfont_to_dialog ((*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget);
  lw_create_widget ((*first_wv).name, c"dialog".as_ptr (), dialog_id, first_wv,   // C 2241-2244
                    (*(FRAME_X_OUTPUT! (f) as *mut x_output)).widget, True as Boolean, None,
                    Some (dialog_selection_callback), None, None);
  lw_modify_all_widgets (dialog_id, (*first_wv).contents, True as Boolean);   // C 2245
  /* Display the dialog box.  */
  lw_pop_up_all_widgets (dialog_id);                                // C 2247
  popup_activated_flag = 1;                                         // C 2248
  x_activate_timeout_atimer ();                                      // C 2249

  /* Process events that apply to the dialog box.
     Also handle timers.  */
  {                                                                 // C 2253-2263
    let count: specpdl_ref = SPECPDL_INDEX ();

    DEFER_SELECTIONS! ();

    /* xdialog_show_unwind is responsible for popping the dialog box down.  */

    record_unwind_protect_int (Some (pop_down_menu), dialog_id as c_int);   // C 2261

    popup_get_selection (std::ptr::null_mut (), FRAME_DISPLAY_INFO! (f), dialog_id, true);   // C 2263

    unbind_to (count, Qnil ()) as Lisp_Object;
  }
}

#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
static mut button_names: [*const c_char; 10] = [                  // C 2269-2271
  c"button1".as_ptr (), c"button2".as_ptr (), c"button3".as_ptr (), c"button4".as_ptr (),
  c"button5".as_ptr (), c"button6".as_ptr (), c"button7".as_ptr (), c"button8".as_ptr (),
  c"button9".as_ptr (), c"button10".as_ptr () ];

/// C: static Lisp_Object x_dialog_show (struct frame *f, Lisp_Object title, Lisp_Object header,
/// const char **error_name)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
unsafe fn x_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
                         error_name: *mut *const c_char) -> Lisp_Object   // C 2273-2432
{
  let mut i: c_int;                                                 // C 2275
  let mut nb_buttons: c_int = 0;
  let mut dialog_name: [c_char; 6] = [0; 6];                        // C 2276

  let mut wv: *mut widget_value;                                    // C 2278
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();

  /* Number of elements seen so far, before boundary.  */
  let mut left_count: c_int = 0;                                    // C 2281
  /* Whether we've seen the boundary between left-hand elts and right-hand.  */
  let mut boundary_seen: bool = false;                              // C 2283

  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                 // C 2285

  eassert! (FRAME_X_P (f));                                          // C 2287

  *error_name = std::ptr::null ();                                   // C 2289

  if menu_items_n_panes > 1                                          // C 2291
    {
      *error_name = c"Multiple panes in dialog box".as_ptr ();        // C 2293
      return Qnil ();
    }

  /* Create a tree of widget_value objects
     representing the text label and buttons.  */
  {                                                                  // C 2298-2371
    let mut pane_name: Lisp_Object;                                   // C 2300
    let mut pane_string: *const c_char;
    pane_name = AREF (menu_items, MENU_ITEMS_PANE_NAME as isize);     // C 2302
    pane_string = if NILP (pane_name) { c"".as_ptr () } else { SSDATA (pane_name) };   // C 2303-2304
    prev_wv = make_widget_value (c"message".as_ptr (), pane_string as *mut c_char,
				 true, Qnil ());                      // C 2305
    first_wv = prev_wv;

    /* Loop over all panes and items, filling in the tree.  */
    i = MENU_ITEMS_PANE_LENGTH;                                       // C 2309
    while i < menu_items_used                                      // C 2310-2364
      {

	/* Create a new item within current pane.  */
	let mut item_name: Lisp_Object;                                 // C 2314
	let mut enable: Lisp_Object;
	let mut descrip: Lisp_Object;
	item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 2315
	enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 2316
	descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 2317-2318

	if NILP (item_name)                                           // C 2320
	  {
	    free_menubar_widget_value_tree (first_wv);
	    *error_name = c"Submenu in dialog items".as_ptr ();         // C 2323
	    return Qnil ();
	  }
	if EQ (item_name, Qquote ())                                   // C 2327
	  {
	    /* This is the boundary between left-side elts
	       and right-side elts.  Stop incrementing right_count.  */
	    boundary_seen = true;                                       // C 2332
	    i += 1;
	    continue;
	  }
	if nb_buttons >= 9                                             // C 2336
	  {
	    free_menubar_widget_value_tree (first_wv);
	    *error_name = c"Too many dialog items".as_ptr ();            // C 2339
	    return Qnil ();
	  }

	wv = make_widget_value (button_names[nb_buttons as usize],   // C 2342-2344
				SSDATA (item_name) as *mut c_char,
				!NILP (enable), Qnil ());
	(*prev_wv).next = wv;                                          // C 2345
	if !NILP (descrip)                                             // C 2346
	  { (*wv).key = SSDATA (descrip); }
	(*wv).call_data = aref_addr (menu_items, i as isize) as *mut c_void;   // C 2348
	prev_wv = wv;                                                  // C 2349

	if !boundary_seen                                             // C 2351
	  { left_count += 1; }

	nb_buttons += 1;                                               // C 2354
	i += MENU_ITEMS_ITEM_LENGTH;                                   // C 2355
      }

    /* If the boundary was not specified,
       by default put half on the left and half on the right.  */
    if !boundary_seen                                             // C 2359
      { left_count = nb_buttons - nb_buttons / 2; }

    wv = make_widget_value (dialog_name.as_ptr (), std::ptr::null_mut (), false, Qnil ());   // C 2361

    /*  Frame title: 'Q' = Question, 'I' = Information.
        Can also have 'E' = Error if, one day, we want
        a popup for errors. */
    if NILP (header)                                              // C 2366
      { dialog_name[0] = b'Q' as c_char; }
    else
      { dialog_name[0] = b'I' as c_char; }

    /* Dialog boxes use a really stupid name encoding
       which specifies how many buttons to use
       and how many buttons are on the right. */
    dialog_name[1] = (b'0' as c_int + nb_buttons) as c_char;        // C 2374
    dialog_name[2] = b'B' as c_char;                                // C 2375
    dialog_name[3] = b'R' as c_char;                                // C 2376
    /* Number of buttons to put on the right.  */
    dialog_name[4] = (b'0' as c_int + nb_buttons - left_count) as c_char;   // C 2378
    dialog_name[5] = 0;                                             // C 2379
    (*wv).contents = first_wv;                                      // C 2380
    first_wv = wv;                                                  // C 2381
  }

  /* No selection has been chosen yet.  */
  menu_item_selection = std::ptr::null_mut ();                       // C 2385

  /* Make sure to free the widget_value objects we used to specify the
     contents even with longjmp.  */
  record_unwind_protect_ptr (Some (cleanup_widget_value_tree), first_wv as *mut c_void);   // C 2389

  /* Actually create and show the dialog.  */
  create_and_show_dialog (f, first_wv);                              // C 2392

  unbind_to (specpdl_count, Qnil ()) as Lisp_Object;                  // C 2394

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if !menu_item_selection.is_null ()                                 // C 2398
    {
      i = 0;                                                          // C 2400
      while i < menu_items_used                                    // C 2400-2421
	{
	  let mut entry: Lisp_Object;

	  if EQ (AREF (menu_items, i as isize), Qt ())                 // C 2404
	    { i += MENU_ITEMS_PANE_LENGTH; }
	  else if EQ (AREF (menu_items, i as isize), Qquote ())        // C 2406
	    {
	      /* This is the boundary between left-side elts and
		 right-side elts.  */
	      i += 1;
	    }
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 2415
	      if menu_item_selection == aref_addr (menu_items, i as isize)   // C 2416
		{ return entry; }
	      i += MENU_ITEMS_ITEM_LENGTH;                             // C 2418
	    }
	}
    }
  else
    /* Make "Cancel" equivalent to C-g.  */
    { quit (); }                                                   // C 2425

  return Qnil ();                                                  // C 2427
}

/// C: Lisp_Object xw_popup_dialog (struct frame *f, Lisp_Object header, Lisp_Object contents)
#[cfg (all (feature = "have_x_windows",
            any (feature = "use_x_toolkit", feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xw_popup_dialog (f: *mut frame, header: Lisp_Object,
                                          mut contents: Lisp_Object) -> Lisp_Object   // C 2434-2467
{
  let mut title: Lisp_Object;                                      // C 2436
  let mut error_name: *const c_char = std::ptr::null ();            // C 2437
  let mut selection: Lisp_Object;                                  // C 2438
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                // C 2439

  check_window_system (f);                                          // C 2441

  /* Decode the dialog items from what was specified.  */
  title = Fcar (contents);                                          // C 2444
  CHECK_STRING (title);                                             // C 2445
  record_unwind_protect_void (Some (unuse_menu_items));             // C 2446

  if NILP (Fcar (Fcdr (contents)))                                  // C 2448
    /* No buttons specified, add an "Ok" button so users can pop down
       the dialog.  Also, the lesstif/motif version crashes if there are
       no buttons.  */
    { contents = list2 (title, Fcons (build_string (c"Ok".as_ptr ()), Qt ())); }   // C 2452

  list_of_panes (list1 (contents));                                 // C 2454

  /* Display them in a dialog box.  */
  block_input ();                                                   // C 2457
  selection = x_dialog_show (f, title, header, &raw mut error_name);   // C 2458
  unblock_input ();                                                 // C 2459

  unbind_to (specpdl_count, Qnil ()) as Lisp_Object;                 // C 2461
  discard_menu_items ();                                            // C 2462

  if !error_name.is_null () { error (c"%s".as_ptr (), error_name); }   // C 2464
  return selection;                                                 // C 2465
}

/* ==============================================================================================
   C 2468-2879: `#else /* not USE_X_TOOLKIT && not USE_GTK */` - the oldXMenu build.  The `#if`
   arm (the toolkit/GTK one) is C 1388-2467 above.  */

/* The frame of the last activated non-toolkit menu bar.
   Used to generate menu help events.  */                          // C 2470-2471
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
            not (feature = "use_gtk")))]
static mut menu_help_frame: *mut frame = std::ptr::null_mut ();    // C 2473

/* Show help HELP_STRING, or clear help if HELP_STRING is null.

   PANE is the pane number, and ITEM is the menu item number in
   the menu (currently not used).

   This cannot be done with generating a HELP_EVENT because
   XMenuActivate contains a loop that doesn't let Emacs process
   keyboard events.  */                                            // C 2476-2483
/// C: static void menu_help_callback (char const *help_string, int pane, int item)
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
            not (feature = "use_gtk")))]
unsafe extern "C" fn menu_help_callback (help_string: *const c_char, pane: c_int,
                                         item: c_int)            // C 2486-2508
{
  let mut first_item: *mut Lisp_Object;                             // C 2488
  let mut pane_name: Lisp_Object;
  let mut menu_object: Lisp_Object;

  /* lisp.h 3944-3957's `XVECTOR`/`xvector_contents`: the mirror's `Lisp_Vector.contents` is the
     `[Lisp_Object; 0]` flexible array member (P-55), so C's `->contents` is its address.  */
  first_item = (*XVECTOR (menu_items)).contents.as_ptr () as *mut Lisp_Object;   // C 2492
  if EQ (*first_item.offset (0), Qt ())                             // C 2493
    { pane_name = *first_item.offset (MENU_ITEMS_PANE_NAME as isize); }
  else if EQ (*first_item.offset (0), Qquote ())                    // C 2495
    /* This shouldn't happen, see x_menu_show.  */
    { pane_name = empty_unibyte_string; }
  else
    { pane_name = *first_item.offset (MENU_ITEMS_ITEM_NAME as isize); }

  /* (menu-item MENU-NAME PANE-NUMBER)  */
  menu_object = list3 (Qmenu_item (), pane_name, make_fixnum (pane as EMACS_INT));   // C 2502
  show_help_echo (if !help_string.is_null () { build_string (help_string) } else { Qnil () },   // C 2503-2504
		  Qnil (), menu_object, make_fixnum (item as EMACS_INT));
}

/// C: struct pop_down_menu { struct frame *frame; XMenu *menu; };
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
            not (feature = "use_gtk")))]
#[repr(C)]
pub struct pop_down_menu
{
  pub frame: *mut frame,                  // C 2512
  pub menu: *mut XMenu,                   // C 2513
}

#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
            not (feature = "use_gtk")))]
/// C: static void pop_down_menu (void *arg)
unsafe extern "C" fn pop_down_menu (arg: *mut c_void)            // C 2517-2559
{
  let data: *mut pop_down_menu = arg as *mut pop_down_menu;         // C 2519
  let f: *mut frame = (*data).frame;                                 // C 2520
  let menu: *mut XMenu = (*data).menu;                               // C 2521
  #[cfg (feature = "have_xinput2")]                                // C 2522-2525
  let mut i: c_int;
  #[cfg (feature = "have_xinput2")]
  let mut device: *mut xi_device_t;

  block_input ();                                                   // C 2527
  #[cfg (not (feature = "msdos"))]                                  // C 2528-2531
  {
    XUngrabPointer (FRAME_X_DISPLAY! (f), CurrentTime) as c_int;    // C 2529
    XUngrabKeyboard (FRAME_X_DISPLAY! (f), CurrentTime) as c_int;   // C 2530
  }
  XMenuDestroy (FRAME_X_DISPLAY! (f), menu);               // C 2532

  #[cfg (feature = "have_x_windows")]                               // C 2534-2555
  {
    /* Assume the mouse has moved out of the X window.
       If it has actually moved in, we will get an EnterNotify.  */
    x_mouse_leave (FRAME_DISPLAY_INFO! (f));                        // C 2537

    /* State that no mouse buttons are now held.
       (The oldXMenu code doesn't track this info for us.)
       That is not necessarily true, but the fiction leads to reasonable
       results, and it is a pain to ask which are actually held now.  */
    (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                         // C 2543

    #[cfg (feature = "have_xinput2")]                               // C 2545-2551
    {
      /* Likewise for XI grabs when the mouse is released on top of the
         menu itself.  */

      i = 0;
      while i < (*FRAME_DISPLAY_INFO! (f)).num_devices
	{
	  device = &raw mut *(*FRAME_DISPLAY_INFO! (f)).devices.offset (i as isize);
	  (*device).grab = 0;
	  i += 1;
	}
    }

    /* Decrement the popup_activated_flag.  */
    popup_activated_flag = 0;                                       // C 2553
  }

  unblock_input ();                                                 // C 2557
}

/// C: Lisp_Object x_menu_show (struct frame *f, int x, int y, int menuflags, Lisp_Object title,
/// const char **error_name)
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
            not (feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_menu_show (f: *mut frame, mut x: c_int, mut y: c_int,
                                      menuflags: c_int, title: Lisp_Object,
                                      error_name: *mut *const c_char) -> Lisp_Object   // C 2562-2877
{
  let mut root: Window;                                             // C 2564
  let mut menu: *mut XMenu;                                         // C 2565
  let mut pane: c_int;                                              // C 2566
  let mut selidx: c_int;
  let mut lpane: c_int;
  let mut status: c_int;
  let mut entry: Lisp_Object = Qnil ();                             // C 2567
  let mut pane_prefix: Lisp_Object;                                 // C 2568
  let mut datap: *mut c_char;                                       // C 2569
  let mut ulx: c_int;                                               // C 2570
  let mut uly: c_int;
  let mut width: c_int;
  let mut height: c_int;
  let mut dispwidth: c_int;                                         // C 2572
  let mut dispheight: c_int;
  let mut i: c_int;                                                 // C 2573
  let mut j: c_int;
  let mut lines: c_int;
  let mut maxlines: c_int;
  let mut maxwidth: c_int;                                          // C 2574
  let mut dummy_int: c_int;                                         // C 2575
  let mut dummy_uint: c_uint;                                       // C 2576
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();                 // C 2577

  eassert! (FRAME_X_P (f) || FRAME_MSDOS_P (f));                     // C 2579

  *error_name = std::ptr::null ();                                   // C 2581
  if menu_items_n_panes == 0                                         // C 2582
    { return Qnil (); }

  if menu_items_used <= MENU_ITEMS_PANE_LENGTH                       // C 2584
    {
      *error_name = c"Empty menu".as_ptr ();                          // C 2586
      return Qnil ();
    }

  /* C's three `goto return_entry` jumps (C 2603/2644/2706) are the `break 'return_entry`s of
     this labeled block (PORTING.md 2.2's forward-jump shape; the `goto` token has no Rust
     counterpart and is recorded in `docs/ports/macro-body-notes.tsv`), and C 2874's label is
     the tail below the block.  The block opens before `USE_SAFE_ALLOCA!` because C's `sa_count`
     is a function-scope local that the label's own tail spends.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                             // C 2590
  block_input ();                                                    // C 2591
  'return_entry: {

  /* Figure out which root window F is on.  */
  XGetGeometry (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f), &raw mut root,   // C 2594-2596
		&raw mut dummy_int, &raw mut dummy_int, &raw mut dummy_uint, &raw mut dummy_uint,
		&raw mut dummy_uint, &raw mut dummy_uint) as c_int;

  /* Make the menu on that window.  */
  menu = XMenuCreate (FRAME_X_DISPLAY! (f), root, c"emacs".as_ptr ());   // C 2599
  if menu.is_null ()                                                 // C 2600
    {
      *error_name = c"Can't create menu".as_ptr ();                   // C 2602
      break 'return_entry;                                          // C 2603 (C's `goto return_entry`)
    }

  /* Don't GC while we prepare and show the menu,
     because we give the oldxmenu library pointers to the
     contents of strings.  */
  inhibit_garbage_collection ();                                     // C 2608

  #[cfg (feature = "have_x_windows")]                                // C 2610-2618
  { x_translate_coordinates_to_root (f, x, y, &raw mut x, &raw mut y); }
  #[cfg (not (feature = "have_x_windows"))]
  /* MSDOS without X support.  */
  {
    x += (*f).left_pos;
    y += (*f).top_pos;
  }

  /* Create all the necessary panes and their items.  */
  maxwidth = 0; maxlines = 0; lines = 0; i = 0;                       // C 2621
  lpane = XM_FAILURE;                                                 // C 2622
  while i < menu_items_used                                        // C 2623-2716
    {
      if EQ (AREF (menu_items, i as isize), Qt ())                    // C 2625
	{
	  /* Create a new pane.  */
	  let mut pane_name: Lisp_Object;                                // C 2628
	  let mut prefix: Lisp_Object;
	  let mut pane_string: *const c_char;

          maxlines = max (maxlines as isize, lines as isize) as c_int;   // C 2631
          lines = 0;
	  pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);   // C 2633
	  prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);    // C 2634
	  pane_string = if NILP (pane_name) { c"".as_ptr () } else { SSDATA (pane_name) };   // C 2635-2636
	  if (menuflags & MENU_KEYMAPS) != 0 && !NILP (prefix)          // C 2637
	    { pane_string = pane_string.offset (1); }

	  lpane = XMenuAddPane (FRAME_X_DISPLAY! (f), menu, pane_string, True) as c_int;   // C 2639
	  if lpane == XM_FAILURE                                        // C 2640
	    {
	      XMenuDestroy (FRAME_X_DISPLAY! (f), menu);         // C 2642
	      *error_name = c"Can't create pane".as_ptr ();               // C 2643
	      break 'return_entry;                                     // C 2644 (C's `goto return_entry`)
	    }
	  i += MENU_ITEMS_PANE_LENGTH;                                  // C 2646

	  /* Find the width of the widest item in this pane.  */
	  j = i;                                                        // C 2649
	  while j < menu_items_used                                  // C 2650-2663
	    {
	      let mut item: Lisp_Object;
	      item = AREF (menu_items, j as isize);                       // C 2652
	      if EQ (item, Qt ())
		{ break; }
	      if NILP (item)
		{
		  j += 1;
		  continue;
		}
	      width = SBYTES (item) as c_int;                             // C 2657
	      if width > maxwidth
		{ maxwidth = width; }

	      j += MENU_ITEMS_ITEM_LENGTH;                              // C 2661
	    }
	}
      /* Ignore a nil in the item list.
	 It's meaningful only for dialog boxes.  */
      else if EQ (AREF (menu_items, i as isize), Qquote ())           // C 2666
	{ i += 1; }
      else
	{
	  /* Create a new item within current pane.  */
	  let mut item_name: Lisp_Object;                               // C 2671
	  let mut enable: Lisp_Object;
	  let mut descrip: Lisp_Object;
	  let mut help: Lisp_Object;
	  let mut item_data: *mut c_char;                               // C 2673
	  let mut help_string: *const c_char;                           // C 2674

	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 2676
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 2677
	  descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 2678-2679
	  help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);        // C 2680
	  help_string = if STRINGP (help) { SSDATA (help) } else { std::ptr::null () };   // C 2681

	  if !NILP (descrip)                                           // C 2683
	    {
	      item_data = SAFE_ALLOCA! (sa_avail, maxwidth as EMACS_INT + SBYTES (descrip) + 1)   // C 2685
		            as *mut c_char;
	      memcpy (item_data as *mut c_void, SSDATA (item_name) as *const c_void,
		      SBYTES (item_name) as usize);                        // C 2686
	      j = SCHARS (item_name) as c_int;                             // C 2687
	      while j < maxwidth
		{
		  *item_data.offset (j as isize) = b' ' as c_char;          // C 2688
		  j += 1;
		}
	      memcpy (item_data.offset (j as isize) as *mut c_void,
		      SSDATA (descrip) as *const c_void, SBYTES (descrip) as usize);   // C 2690
	      *item_data.offset ((j + SBYTES (descrip) as c_int) as isize) = 0;   // C 2691
	    }
	  else
	    { item_data = SSDATA (item_name); }                         // C 2694

	  if lpane == XM_FAILURE
	      || XMenuAddSelection (FRAME_X_DISPLAY! (f),              // C 2696-2700
				    menu, lpane, std::ptr::null_mut (), item_data,
				    (!NILP (enable)) as c_int, help_string) as c_int
		  == XM_FAILURE
	    {
	      XMenuDestroy (FRAME_X_DISPLAY! (f), menu);        // C 2704
	      *error_name = c"Can't add selection to menu".as_ptr ();     // C 2705
	      break 'return_entry;                                     // C 2706 (C's `goto return_entry`)
	    }
	  i += MENU_ITEMS_ITEM_LENGTH;                                 // C 2708
          lines += 1;
	}
    }

  maxlines = max (maxlines as isize, lines as isize) as c_int;        // C 2713

  /* All set and ready to fly.  */
  XMenuRecompute (FRAME_X_DISPLAY! (f), menu) as c_int;              // C 2716
  dispwidth = DisplayWidth! (FRAME_X_DISPLAY! (f), FRAME_X_SCREEN_NUMBER! (f)) as c_int;   // C 2717
  dispheight = DisplayHeight! (FRAME_X_DISPLAY! (f), FRAME_X_SCREEN_NUMBER! (f)) as c_int;   // C 2718
  x = min (x as isize, dispwidth as isize) as c_int;                  // C 2719
  y = min (y as isize, dispheight as isize) as c_int;                 // C 2720
  x = max (x as isize, 1) as c_int;                                   // C 2721
  y = max (y as isize, 1) as c_int;                                   // C 2722
  XMenuLocate (FRAME_X_DISPLAY! (f), menu, 0, 0, x, y,               // C 2723-2724
	       &raw mut ulx, &raw mut uly, &raw mut width, &raw mut height) as c_int;
  if ulx + width > dispwidth                                         // C 2725
    {
      x -= (ulx + width) - dispwidth;
      ulx = dispwidth - width;
    }
  if uly + height > dispheight                                       // C 2730
    {
      y -= (uly + height) - dispheight;
      uly = dispheight - height;
    }
  #[cfg (not (feature = "have_x_windows"))]                          // C 2736-2752
  if FRAME_HAS_MINIBUF_P (f) && uly + height > dispheight - 1
    {
      /* Move the menu away of the echo area, to avoid overwriting the
	 menu with help echo messages or vice versa.  */
      if BUFFERP (*echo_area_buffer.offset (0)) && WINDOWP (echo_area_window)
	{
	  y -= WINDOW_TOTAL_LINES (XWINDOW (echo_area_window)) as c_int;
	  uly -= WINDOW_TOTAL_LINES (XWINDOW (echo_area_window)) as c_int;
	}
      else
	{
	  y -= 1;
	  uly -= 1;
	}
    }
  if ulx < 0 { x -= ulx; }                                           // C 2754
  if uly < 0 { y -= uly; }                                           // C 2755

  if (menuflags & MENU_FOR_CLICK) == 0                               // C 2757
    {
      /* If position was not given by a mouse click, adjust so upper left
         corner of the menu as a whole ends up at given coordinates.  This
         is what x-popup-menu says in its documentation.  */
      x += width / 2;                                                 // C 2762
      y += (1.5 * height as f64 / (maxlines + 2) as f64) as c_int;    // C 2763
    }

  XMenuSetFreeze (menu, True);                                        // C 2766
  pane = 0; selidx = 0;                                              // C 2767

  #[cfg (not (feature = "msdos"))]                                   // C 2768-2781
  {
    DEFER_SELECTIONS! ();

    XMenuActivateSetWaitFunction (Some (x_menu_wait_for_event),
				  FRAME_X_DISPLAY! (f) as *mut c_void);   // C 2773-2774
    XMenuEventHandler (Some (x_menu_dispatch_event));                 // C 2775

    /* When the input extension is in use, the owner_events grab will
       report extension events on frames, which the XMenu library does
       not normally understand.  */
    #[cfg (feature = "have_xinput2")]                                 // C 2777-2780
    { XMenuActivateSetTranslateFunction (Some (x_menu_translate_generic_event)); }
  }

  /* C's `&(struct pop_down_menu) {f, menu}` is a compound literal, which Rust has no spelling
     for; the port binds the record and passes its address.  */
  let mut down_data: pop_down_menu = pop_down_menu { frame: f, menu };
  record_unwind_protect_ptr (Some (pop_down_menu),                     // C 2784-2785
			     &raw mut down_data as *mut c_void);

  /* Help display under X won't work because XMenuActivate contains
     a loop that doesn't give Emacs a chance to process it.  */
  menu_help_frame = f;                                                // C 2788

  #[cfg (feature = "have_xinput2")]                                   // C 2790-2804
  {
    /* C 2791's `struct x_display_info *dpyinfo = FRAME_DISPLAY_INFO (f);`  */
    let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
    /* Clear the XI2 grab so a core grab can be set.  */

    if (*dpyinfo).supports_xi2
	&& xi_frame_selected_for (f, XI_ButtonPress as c_ulong)
      {
	let mut i: c_int = 0;
	while i < (*dpyinfo).num_devices
	  {
	    if (*(*dpyinfo).devices.offset (i as isize)).grab != 0
	      { XIUngrabDevice ((*dpyinfo).display,
				(*(*dpyinfo).devices.offset (i as isize)).device_id,
				CurrentTime) as c_int; }
	    i += 1;
	  }
      }
  }

  #[cfg (feature = "have_x_windows")]                                 // C 2806-2811
  /* Increment the popup flag; this prevents nested popups from being
     displayed by user Lisp code in help-echo callbacks, and also
     prevents mouse face from being displayed.  */
  { popup_activated_flag = 1; }
  status = XMenuActivate (FRAME_X_DISPLAY! (f), menu, &raw mut pane, &raw mut selidx,   // C 2813-2815
                          x, y, ButtonReleaseMask, &raw mut datap,
                          Some (menu_help_callback)) as c_int;
  pane_prefix = Qnil ();                                              // C 2816

  match status                                                       // C 2818-2870
    {
      XM_SUCCESS =>                                                 // C 2820
	{
	  #[cfg (feature = "xdebug")]                                 // C 2821-2823
	  fprintf (stderr, c"pane= %d line = %d\n".as_ptr (), panes, selidx);

	  /* Find the item number SELIDX in pane number PANE.  */
	  i = 0;                                                       // C 2827
	  while i < menu_items_used                                // C 2827-2860
	    {
	      if EQ (AREF (menu_items, i as isize), Qt ())              // C 2829
		{
		  if pane == 0
		    { pane_prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize); }   // C 2831-2832
		  pane -= 1;
		  i += MENU_ITEMS_PANE_LENGTH;
		}
	      else
		{
		  if pane == -1                                          // C 2838
		    {
		      if selidx == 0                                     // C 2840
			{
			  entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 2842
			  if (menuflags & MENU_KEYMAPS) != 0               // C 2844
			    {
			      entry = list1 (entry);
			      if !NILP (pane_prefix)
				{ entry = Fcons (pane_prefix, entry); }
			    }
			  break;                                        // C 2850
			}
		      selidx -= 1;                                      // C 2852
		    }
		  i += MENU_ITEMS_ITEM_LENGTH;                          // C 2854
		}
	    }
	}
      XM_FAILURE =>                                                  // C 2862
	{ *error_name = c"Can't activate menu".as_ptr (); }
      XM_IA_SELECT =>                                                // C 2864 (C falls through to here)
	{}
      XM_NO_SELECT =>                                                // C 2866
	/* Make "Cancel" equivalent to C-g unless FOR_CLICK (which means
	   the menu was invoked with a mouse event as POSITION).  */
	if (menuflags & MENU_FOR_CLICK) == 0
	  {
	    unblock_input ();                                          // C 2871
	    quit ();                                                   // C 2872
	  }
      _ => {}
    }

  }
  /* C's `return_entry:` label.  */
  unblock_input ();                                                   // C 2874
  return SAFE_FREE_UNBIND_TO! (sa_count, specpdl_count, entry, sa_mark);       // C 2875
}

/* C 2881-2890: `#ifndef MSDOS` / `popup_activated` / `#endif`.  Both arms are carried; the `#else`
   arm is empty (`msdos.c` has its own).  */

/* Detect if a dialog or menu has been posted.  MSDOS has its own
   implementation on msdos.c.  */                                   // C 2882-2883
/// C: int popup_activated (void)
#[cfg (all (feature = "have_x_windows", not (feature = "msdos")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn popup_activated () -> c_int                // C 2885-2888
{
  return popup_activated_flag;                                       // C 2887
}

/* The following is used by delayed window autoselection.  */        // C 2892
#[cfg (feature = "have_x_windows")]
DEFUN! ("menu-or-popup-active-p", Fmenu_or_popup_active_p, Smenu_or_popup_active_p, 0, 0, 0,
       doc: "Return t if a menu or popup dialog is active.\n(On MS Windows, this refers to the selected frame.)",   // C 2894-2896
       (),                                                           // C 2897
       {
  return if popup_activated () != 0 { Qt () } else { Qnil () };       // C 2900
});

/* C 2903's `static void syms_of_xmenu_for_pdumper (void);` - C's forward declaration, dropped
   (PORTING.md 0.1-12); the definition is at its C position below.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xmenu ()                           // C 2906-2922
{
  DEFSYM! (Qdebug_on_next_call, "debug-on-next-call");                // C 2908
  defsubr (&raw mut Smenu_or_popup_active_p);                         // C 2909

  #[cfg (feature = "use_gtk")]                                       // C 2911-2913
  { DEFSYM! (Qframe_monitor_workarea, "frame-monitor-workarea"); }

  #[cfg (all (feature = "have_x_windows",
              any (feature = "use_gtk", feature = "use_x_toolkit")))]   // C 2915-2919
  {
    defsubr (&raw mut Sx_menu_bar_open_internal);
    Ffset (intern_c_string (c"accelerate-menu".as_ptr ()),
	   intern_c_string (Sx_menu_bar_open_internal.s.symbol_name));
  }

  pdumper_do_now_and_after_load (Some (syms_of_xmenu_for_pdumper));   // C 2921
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn syms_of_xmenu_for_pdumper ()                    // C 2924-2931
{
  #[cfg (feature = "use_x_toolkit")]                                 // C 2927-2931
  {
    /* C 2928's `enum { WIDGET_ID_TICK_START = 1 << 16 };` */
    const WIDGET_ID_TICK_START: c_int = 1 << 16;
    widget_id_tick = WIDGET_ID_TICK_START as LWLIB_ID;
    next_menubar_widget_id = 1;
  }
}
