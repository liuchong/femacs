// femacs: port of src/xwidget.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `xwidget.o`
// joins the object list only when xwidgets are enabled: `configure.ac` 4422-4474 sets
// `HAVE_XWIDGETS=no` first, then `XWIDGETS_OBJ="xwidget.o"` in the GTK branch (4434, under the
// `USE_GTK` arm whose 4433 reads `HAVE_XWIDGETS=$HAVE_WEBKIT`) and again in the Cocoa branch
// (4463), and `AC_DEFINE ([HAVE_XWIDGETS], [1])` is 4474; `docs/baseline/frozen-build-sources.txt`
// lists no `xwidget.o` and `docs/baseline/config/darwin-arm64.macros.tsv` 1717 has
// `HAVE_XWIDGETS undef`, 3861 `USE_GTK undef` - so the text can never be *compiled* here.
// `python3 tools/check-cond-eval.py src/xwidget.c` reads `121 conditions, 0 unknown, 0
// disagreements [VERIFIED]` (the file is `docs/baseline/config/cond-eval-verified.txt` 428), so
// every arm below is *decided* and `tools/extract-abi.py`'s checklist is **not** empty: the
// `--check` reading in the ledger's `## Evidence` is not vacuous.  What the ledger adds is P-86 ②'s
// substitute shape - the **C line-range table**, accounting for every line of the reference -
// beside the `## Evidence` readings and the `## Deviations`.
//
// **This batch is P-86 ④'s first slice: C 1-1180.**  It ends at the `}` of `run_file_chooser_cb`
// (C 1122-1180); **the next slice opens at C 1182's `#ifdef HAVE_X_WINDOWS`** and its first
// definition is `xv_drag_begin_cb` (C 1184-1200).  C 1181 is the blank the table needs no row for.
//
//   * the **file gate** is `all (feature = "use_gtk", feature = "have_gtk3")`, on every item *and
//     on the include graph* (P-86 ⑧ - an `include!` outside the gate makes the frozen build fail
//     instead of producing the empty object the class expects).  **Corrected 2026-10-05 (status
//     3.596)**: this used to be `use_gtk` alone, read off C 37's `#ifdef USE_GTK` - but that is the
//     file's first and widest *content* guard, not the condition that builds the object.
//     `configure.ac` 4424-4434 adds `XWIDGETS_OBJ="xwidget.o"` inside BOTH `if test
//     "$with_xwidgets" != "no"` and `if test "$USE_GTK_TOOLKIT" = "GTK3" && test "$window_system"
//     != "none"` (a second arm at 4463 covers Cocoa), and `USE_GTK` is defined for *any* GTK
//     (3826) while `USE_GTK_TOOLKIT="GTK3"` is the same block as `AC_DEFINE([HAVE_GTK3])`
//     (3756-3759) - so a GTK2 build compiles no `xwidget.o` at all, and gating on `use_gtk` alone
//     compiled the file in a configuration C does not.  The two `ns_impl_cocoa` items are left as
//     they are (their file gate would be the Cocoa arm's, not GTK's); all `USE_GTK` *content*
//     guards below are unchanged and now sit under the corrected file gate;
//   * the slice's own conditions are C 41's `#ifndef HAVE_PGTK` (feature `have_pgtk`), C 47's
//     `#ifdef HAVE_XINPUT2`, C 50's `#elif defined NS_IMPL_COCOA`, C 61/98/298/329/493/513/527/
//     739's `#ifdef USE_GTK`, C 62/230/1027's `#ifdef HAVE_X_WINDOWS`, C 64/339/396's
//     `WEBKIT_CHECK_VERSION` (below), C 130/451/1064's `#ifdef HAVE_PGTK`, C 423's
//     `#if !defined HAVE_XINPUT2 && !defined HAVE_PGTK`, C 519/537/624/684's `#ifdef HAVE_XINPUT2`,
//     C 572/604/774/813/850's `#ifndef HAVE_PGTK` and C 644's `#ifndef HAVE_X_WINDOWS` - all
//     decided by the frozen table, and **both arms of every one of them are carried**;
//   * `WEBKIT_CHECK_VERSION (major, minor, micro)` is WebKitGTK's own macro, which the reference
//     tree does not define anywhere (`grep -rn WEBKIT_CHECK_VERSION emacs/src/*.h` is empty), so
//     in the frozen configuration `clang` reports `function-like macro 'WEBKIT_CHECK_VERSION' is
//     not defined` and takes no branch, and the evaluator agrees (measured: the probe below,
//     Evidence ⑤).  The port spells each arm with a derived-condition feature after the
//     `android_api_le_11`/`gtk_check_version_ge_3_10` naming: **`webkit_check_version_ge_2_16`**,
//     **`_ge_2_26`**, **`_ge_2_34`** - **none registered in `femacs/xtask/Cargo.toml`**
//     (`## Owed` 1);
//   * the include graph reaches **placeholders**: `include/gtkutil.rs` carries the guard alone, so
//     the two `xg_*` names this slice reads (`xg_gtk_initialized`, `XG_XWIDGET`) and the
//     `xwidget.h` entry points are declared where C's text uses them (P-72 rule 4), and
//     `docs/ports/xwidget.md` owes those moves.
//
// **The toolkit surface is declared in this unit (P-88 ①, which P-90 ① keeps for this round), and
// this batch is `text-only` (P-86 ③⑵).**  Measured on this machine:
// `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0
// /opt/homebrew/include/webkit2gtk-4.0` -> all `No such file or directory`; `pkg-config` is not
// installed; no `gtk/gtk.h`, `gdk/gdk.h`, `gdktypes.h` or `JavaScriptCore` anywhere under
// `/opt/homebrew`, `/usr/local` or `/Library`.  So **GDK, GTK and WebKit have no tree to compile
// or measure against**: every citation of theirs names the header alone (no line, no probe) and
// every size, offset, enum width and enumerator *value* of theirs is **UNVERIFIED** in the port -
// no arm flip is possible and no layout assertion is written for a GDK/GTK/WebKit record.
// **GLib/GObject, cairo and X11 *are* on this machine** (`/opt/homebrew/include/glib-2.0`, GLib
// 2.90.0, with the generated `glibconfig.h` under
// `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`; `/opt/homebrew/include/cairo`; 
// `/opt/homebrew/include/X11`), so the GLib declarations quote the header and line they were read
// from and `GList`/`GSList`'s layouts are real clang readings (Evidence ⑤).  P-88 ⑤/P-90 ③: the
// move into a dedicated SDK mirror (`femacs/src/include/gtk.rs`, PORTING.md P-90) is owed
// (`## Owed` 2), and this unit's copy plus the segment copies in `src/gtkutil.rs` and
// `src/pgtkterm.rs` are the ones to drop in that commit.
//
// `docs/ports/xwidget.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.  No behaviour of this file was executed anywhere: there is no GTK
// tree to compile the arm against and no GTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>`

/* C 22-34: the Emacs headers, in C's order except that `"lisp.h"` is first (PORTING.md 0.1-14:
   `config.rs` -> `lisp.rs` -> the rest, because `buffer.rs`, `coding.rs` and `xwidget.rs` expand
   `eassert!`/`XUNTAG!`/`CHECK_TYPE` textually inside their bodies), and each `include!` carries
   the file gate (P-86 ⑧).  C writes this list outside any `#if`, but the reference's build only
   compiles `xwidget.o` when xwidgets are on (configure.ac 4424-4474), so the gate is the file's
   own - a recorded deviation (Deviations 3).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/lisp.rs");                   // C 25: `#include "lisp.h"`
/* C 25's `"lisp.h"` chain (lisp.h 30/33): `<string.h>` for `strcmp` (C 595) and `<limits.h>` for
   `INT_MAX` (C 554).  `include/lisp.rs` carries neither mirror (its own note records the
   intprops/sys-limits omission), so the two edges are here, at the C position that makes the
   names visible - the `src/gtkutil.rs` 96-99 shape.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>`

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/buffer.rs");                 // C 22: `#include "buffer.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/coding.rs");                 // C 23: `#include "coding.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/xwidget.rs");                // C 24: `#include "xwidget.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/blockinput.rs");             // C 26: `#include "blockinput.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/dispextern.rs");             // C 27: `#include "dispextern.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/frame.rs");                  // C 28: `#include "frame.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/keyboard.rs");               // C 29: `#include "keyboard.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/gtkutil.rs");                // C 30: `#include "gtkutil.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/gtk.rs");                   // C 30's chain: `"gtkutil.h"` 26's
                                              // `<gtk/gtk.h>` (P-90 ②, the SDK surface's
                                              // one mirror); the in-unit segment that
                                              // duplicated it is gone (3.481)
/* C 30's `"gtkutil.h"` chain: gtkutil.h 32's `#include "xterm.h"` (the non-PGTK arm) and
   gtkutil.h 29's `#include "pgtkterm.h"` (the PGTK arm).  `include/gtkutil.rs` is a placeholder
   (it carries the include guard alone), so the two edges stand here, at the C position that makes
   the names visible: this slice reads `FRAME_GTK_WIDGET`/`FRAME_GTK_OUTER_WIDGET`/`FRAME_X_OUTPUT`
   /`FRAME_DISPLAY_INFO` and `x_display_info`'s four `*_mod_mask` members through them.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
include!("include/xterm.rs");                  // gtkutil.h 32: `#include "xterm.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // gtkutil.h 29: `#include "pgtkterm.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/sysstdio.rs");               // C 31: `#include "sysstdio.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/termhooks.rs");              // C 32: `#include "termhooks.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/window.rs");                 // C 33: `#include "window.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/process.rs");                // C 34: `#include "process.h"`

/* ----------------------------------------------------------------------------------------------
   C 36-54: the third-party include chain - `/* Include xwidget bottom end headers.  */` (C 36),
   then C 37-52's `#ifdef USE_GTK`/`#elif defined NS_IMPL_COCOA` pair and C 54's `<math.h>`.
   `nsxwidget.h` (C 51, the `NS_IMPL_COCOA` arm) has a mirror - `include/nsxwidget.rs` - but it is
   the Cocoa lane's, not this slice's, so the arm is C's text plus the arm feature below.
   `<cairo.h>`/`<cairo-xlib.h>` (C 40/42) supply no name this slice uses, and the X11 headers
   (C 43/48) are mirrored by `include/xlib.rs` (which `include/xterm.rs` brings in, above).
   ---------------------------------------------------------------------------------------------- */
/* C 37-52: `#ifdef USE_GTK` (the file gate) / `#ifndef HAVE_PGTK` (C 41, the X11 arm) / `#ifdef
   HAVE_XINPUT2` (C 47) / `#elif defined NS_IMPL_COCOA` (C 50).  Each arm's text is the header it
   names; the `#endif`s are the gate boundaries this comment records.  */

/* C 54: `#include <math.h>` - `lrint` (C 148/149/...).  No mirror carries `<math.h>` (`ls
   femacs/lib/include | grep -i math` is empty), so the one name this slice takes from it stands
   here (P-88 ①).  */
/// C: math.h: `long int lrint (double x);` (the C library's, present on this machine).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn lrint (x: c_double) -> c_long;
}

/* ----------------------------------------------------------------------------------------------
   C 37-40/45's chain: `<gtk/gtk.h>` and the WebKitGTK headers, plus the GLib/GObject and GDK
   surfaces those pull in.  Third-party headers with no file in the reference tree, so the
   declarations this slice takes from them stand here (P-88 ①; the move into
   `femacs/src/include/gtk.rs` is P-90 ③'s owed item - `## Owed` 2).  GLib/GObject is on this
   machine, so its lines are cited from the headers themselves and its records are measured
   (Evidence ⑤); **GDK, GTK and WebKit are not** - see the file note - so their citations name the
   header alone, no layout assertion is written for a GDK/GTK/WebKit record, and every size, offset
   and enumerator value of theirs is **UNVERIFIED** (P-88 ②).
   ---------------------------------------------------------------------------------------------- */

/* GLib and GObject: `/opt/homebrew/include/glib-2.0` (GLib 2.90.0), `glibconfig.h` from
   `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::size_of::<GList> () == 24);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::offset_of! (GList, data) == 0);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::offset_of! (GList, next) == 8);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::offset_of! (GList, prev) == 16);
/// C: gslist.h 41-45: `struct _GSList { gpointer data; GSList *next; };`.  C 1132 declares
/// `GSList *filenames;` and `GSList *tem;`, and C 1167-1168 reads `tem->data`, `tem->next`.
/// Measured `sizeof_GSList=16 data=0 next=8` (Evidence ⑤).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct _GSList
{
  pub data: gpointer,        // C: gslist.h 43: `gpointer data;`
  pub next: *mut _GSList,    // C: gslist.h 44: `GSList *next;`
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GSList = _GSList;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::size_of::<GSList> () == 16);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::offset_of! (GSList, data) == 0);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
const _: () = core::assert! (core::mem::offset_of! (GSList, next) == 8);
/// C: gobject.h 54: `#define G_OBJECT(object) (G_TYPE_CHECK_INSTANCE_CAST ((object),
/// G_TYPE_OBJECT, GObject))`, whose own body is `((GObject *) g_type_check_instance_cast
/// ((GTypeInstance *) (object), (GType) (g_type)))` (gtype.h 528/2668-2672).  The port keeps the
/// macro's name and its result type; GLib's checked-cast chain is recorded as a deviation (the
/// `src/pgtkterm.rs` 288-296 treatment of the same macro).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_OBJECT {
  ($object:expr) => { ($object) as *mut GObject };
}
/// C: gtk/gtk.h: `#define G_CALLBACK(f) ((GCallback) (f))` - GTK's, not GLib's, so the citation
/// is the header and not a line.  C's cast reinterprets the function pointer; Rust refuses a
/// signature-changing `as` cast between fn pointers (`error: non-primitive cast`, measured in
/// `src/pgtkterm.rs`), so the port spells C's cast as the item address plus a pointer transmute,
/// and every callback C passes through it is declared `unsafe extern "C" fn` (PORTING.md 1.4).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_CALLBACK {
  ($f:expr) => {
    Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn ()> ($f as *const ()) })
  };
}
/// C: gsignal.h 554-556: `#define g_signal_connect(instance, detailed_signal, c_handler, data)
/// g_signal_connect_data ((instance), (detailed_signal), (c_handler), (data), NULL,
/// (GConnectFlags) 0)` - GLib's own body.  C 361/427/429/431/442/446/452/457/461/465/471 are the
/// eleven call sites.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! g_signal_connect {
  ($instance:expr, $detailed_signal:expr, $c_handler:expr, $data:expr) => {
    g_signal_connect_data ($instance as gpointer, $detailed_signal, $c_handler,
                           $data as gpointer, None, 0 as GConnectFlags)
  };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 1164: `len = g_slist_length (filenames);`
  /// C: gslist.h 125: `guint g_slist_length (GSList *list);`
  pub fn g_slist_length (list: *mut GSList) -> guint;
  /// C 1171: `g_slist_free (filenames);`
  /// C: gslist.h 52: `void g_slist_free (GSList *list);`
  pub fn g_slist_free (list: *mut GSList);
}

/* ----------------------------------------------------------------------------------------------
   GDK.  No GDK tree on this machine, so each citation names the header alone and every record
   below is either the never-dereferenced `[u8; 0]` shape (`src/xfns.rs` 1004-1009's `GtkWidget`)
   or a record carrying exactly the members C reads, with its layout **UNVERIFIED** (P-88 ②; the
   `src/gtkutil.rs` 494-561 treatment of the same surface, owed to the same mirror).
   ---------------------------------------------------------------------------------------------- */
/// C: gdk/gdktypes.h: `typedef enum { GDK_CROSSING_NORMAL, GDK_CROSSING_SELECTION,
/// GDK_CROSSING_GTK_GRAB, GDK_CROSSING_GTK_UNGRAB, GDK_CROSSING_STATE_CHANGED,
/// GDK_CROSSING_TOUCH_BEGIN, GDK_CROSSING_TOUCH_END, GDK_CROSSING_DEVICE_SWITCH } GdkCrossingMode;`
/// - the type C 271/275 declare parameters of.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkCrossingMode = c_int;
/// C: gdk/gdktypes.h: `typedef enum { GDK_SHIFT_MASK = 1 << 0, GDK_LOCK_MASK = 1 << 1,
/// GDK_CONTROL_MASK = 1 << 2, GDK_MOD1_MASK = 1 << 3, … GDK_SUPER_MASK = 1 << 26,
/// GDK_HYPER_MASK = 1 << 27, GDK_META_MASK = 1 << 28 } GdkModifierType;` - the six C 249-263 ORs
/// into `mods`.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkModifierType = c_int;
/// C: gdk/gdkseat.h: `typedef enum { GDK_SEAT_CAPABILITY_NONE, GDK_SEAT_CAPABILITY_POINTER,
/// GDK_SEAT_CAPABILITY_TOUCH, GDK_SEAT_CAPABILITY_TABLET_STYLUS, GDK_SEAT_CAPABILITY_KEYBOARD,
/// GDK_SEAT_CAPABILITY_TABLET_PAD, GDK_SEAT_CAPABILITY_ALL_POINTING, GDK_SEAT_CAPABILITY_ALL }
/// GdkSeatCapabilities;` - the flag C 891 asks `gdk_seat_get_slaves` for.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkSeatCapabilities = c_uint;
/// C: gtk/gtkwidget.h / gdk/gdkdevice.h / gdk/gdkseat.h: `#define GDK_DEVICE(object)
/// (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DEVICE, GdkDevice))` - C 899's cast keeps the
/// macro's name and its result type; GDK's cast chain is GLib's and owed (as above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GDK_DEVICE {
  ($o:expr) => { ($o) as *mut GdkDevice };
}

/* The GDK/GTK/WebKit enumerators and flag masks this slice names.  **Their values are the
   toolkit's, and this machine has no GDK/GTK/WebKit tree to read them from** (the header-only
   rule above), so the port does not transcribe them: each name below carries a *placeholder*
   whose only job is to keep the reference's arms distinct - every `switch` case stays a different
   value and every `& MASK` still tests a different bit - because PORTING.md forbids inventing
   behaviour and these values are not measurable here.  The real values are owed (`## Owed` 2) and
   each of them is **UNVERIFIED**.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_BUTTON_PRESS: GdkEventType = 1;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_BUTTON_RELEASE: GdkEventType = 2;       // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_2BUTTON_PRESS: GdkEventType = 3;        // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_3BUTTON_PRESS: GdkEventType = 4;        // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL: GdkEventType = 5;               // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_MOTION_NOTIFY: GdkEventType = 6;        // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_ENTER_NOTIFY: GdkEventType = 7;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_LEAVE_NOTIFY: GdkEventType = 8;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_KEY_PRESS: GdkEventType = 9;            // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_KEY_RELEASE: GdkEventType = 10;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_MOD1_MASK: GdkModifierType = 1 << 0;    // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_META_MASK: GdkModifierType = 1 << 1;    // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SUPER_MASK: GdkModifierType = 1 << 2;   // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_HYPER_MASK: GdkModifierType = 1 << 3;   // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_CONTROL_MASK: GdkModifierType = 1 << 4; // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SHIFT_MASK: GdkModifierType = 1 << 5;   // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SEAT_CAPABILITY_ALL_POINTING: GdkSeatCapabilities = 1 << 0;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SOURCE_TOUCHPAD: GdkInputSource = 0;    // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 216: `eventcopy = gdk_event_copy (event);`
  /// C: gdk/gdkevents.h: `GdkEvent *gdk_event_copy (const GdkEvent *event);`
  pub fn gdk_event_copy (event: *const GdkEvent) -> *mut GdkEvent;
  /// C 223/682: `gdk_event_free (eventcopy);`
  /// C: gdk/gdkevents.h: `void gdk_event_free (GdkEvent *event);`
  pub fn gdk_event_free (event: *mut GdkEvent);
  /// C 636: `xg_event = gdk_event_new (GDK_KEY_PRESS);`
  /// C: gdk/gdkevents.h: `GdkEvent *gdk_event_new (GdkEventType type);`
  pub fn gdk_event_new (type_: GdkEventType) -> *mut GdkEvent;
  /// C 676: `gdk_event_set_device (xg_event, find_suitable_keyboard (SELECTED_FRAME ()));`
  /// C: gdk/gdkevents.h: `void gdk_event_set_device (GdkEvent *event, GdkDevice *device);`
  pub fn gdk_event_set_device (event: *mut GdkEvent, device: *mut GdkDevice);
  /// C 956: `twidth = gdk_window_get_width (window);`
  /// C: gdk/gdkwindow.h: `int gdk_window_get_width (GdkWindow *window);`
  pub fn gdk_window_get_width (window: *mut GdkWindow) -> c_int;
  /// C 957: `theight = gdk_window_get_height (window);`
  /// C: gdk/gdkwindow.h: `int gdk_window_get_height (GdkWindow *window);`
  pub fn gdk_window_get_height (window: *mut GdkWindow) -> c_int;
  /// C 976: `gdk_window_get_position (window, &tx, &ty);`
  /// C: gdk/gdkwindow.h: `void gdk_window_get_position (GdkWindow *window, int *x, int *y);`
  pub fn gdk_window_get_position (window: *mut GdkWindow, x: *mut c_int, y: *mut c_int);
  /// C 982: `window = gdk_window_get_parent (window);`
  /// C: gdk/gdkwindow.h: `GdkWindow *gdk_window_get_parent (GdkWindow *window);`
  pub fn gdk_window_get_parent (window: *mut GdkWindow) -> *mut GdkWindow;
  /// C 883/914: `gdk_display_get_default_seat (gtk_widget_get_display (FRAME_GTK_WIDGET (f)))`
  /// C: gdk/gdkdisplay.h: `GdkSeat *gdk_display_get_default_seat (GdkDisplay *display);`
  pub fn gdk_display_get_default_seat (display: *mut GdkDisplay) -> *mut GdkSeat;
  /// C 908: `return !tem ? gdk_seat_get_pointer (seat) : device;`
  /// C: gdk/gdkseat.h: `GdkDevice *gdk_seat_get_pointer (GdkSeat *seat);`
  pub fn gdk_seat_get_pointer (seat: *mut GdkSeat) -> *mut GdkDevice;
  /// C 920: `return gdk_seat_get_keyboard (seat);`
  /// C: gdk/gdkseat.h: `GdkDevice *gdk_seat_get_keyboard (GdkSeat *seat);`
  pub fn gdk_seat_get_keyboard (seat: *mut GdkSeat) -> *mut GdkDevice;
  /// C 641: `keycode = gdk_unicode_to_keyval (character);`
  /// C: gdk/gdkkeyuni.h: `guint gdk_unicode_to_keyval (guint32 wc);`
  pub fn gdk_unicode_to_keyval (wc: guint32) -> guint;
  /// C 546/629/688/779: `gdk_offscreen_window_set_embedder (osw, embedder);`
  /// C: gdk/gdkwindow.h: `void gdk_offscreen_window_set_embedder (GdkWindow *window, GdkWindow
  /// *embedder);`
  pub fn gdk_offscreen_window_set_embedder (window: *mut GdkWindow, embedder: *mut GdkWindow);
}

/* ----------------------------------------------------------------------------------------------
   GTK.  Same treatment as GDK above: no GTK tree on this machine, so each citation names the
   header alone, the `GtkWidget`-family pointees are the `[u8; 0]` shape, and the records C reads
   members of carry exactly those members with their layout **UNVERIFIED** (P-88 ②; owed).
   ---------------------------------------------------------------------------------------------- */
/// C: gtk/gtktextview.h: `typedef struct _GtkTextView GtkTextView;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkTextView
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilechoosernative.h: `typedef struct _GtkFileChooserNative GtkFileChooserNative;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkFileChooserNative
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilefilter.h: `typedef struct _GtkFileFilter GtkFileFilter;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkFileFilter
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilechooser.h: `typedef struct _GtkFileChooser GtkFileChooser;` (an interface;
/// `GTK_FILE_CHOOSER` casts to it).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkFileChooser
{
  _private: [u8; 0],
}
/// C: gtk/gtknativedialog.h: `typedef struct _GtkNativeDialog GtkNativeDialog;` (a class in GTK3;
/// `GTK_NATIVE_DIALOG` casts to it).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkNativeDialog
{
  _private: [u8; 0],
}
/// C: gtk/gtkcontainer.h: `typedef void (*GtkCallback) (GtkWidget *widget, gpointer data);` - the
/// type of `gtk_container_forall`'s second parameter (C 1002/662).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GtkCallback = Option<unsafe extern "C" fn (widget: *mut GtkWidget, data: gpointer)>;
/// C: gtk/gtkcontainer.h: `struct _GtkContainerClass { GtkWidgetClass parent_class; … void
/// (*forall) (GtkContainer *container, gboolean include_internals, GtkCallback callback, gpointer
/// callback_data); … };` - C 660-663 reads `forall` through `GTK_CONTAINER_CLASS`.  The port
/// carries that one member and owes the record's other members and the offset (`src/xfns.rs`
/// 1034-1045's `GtkWidgetClass` precedent).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkContainerClass
{
  pub forall: GtkContainerClass_forall,  // C: gtk/gtkcontainer.h: the four-parameter member
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GtkContainerClass_forall =
  Option<unsafe extern "C" fn (container: *mut GtkContainer, include_internals: gboolean,
                               callback: GtkCallback, callback_data: gpointer)>;
/// C: gtk/gtktypes.h / gtk/gtkenums.h: `GtkResponseType` and `GtkFileChooserAction` - the types
/// of C 1153/1155's `response` and C 1145's action argument.  C's enum ABI is `c_int`
/// (PORTING.md 1.3); **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GtkResponseType = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GtkFileChooserAction = c_int;
/// C: gtk/gtk.h: the checked-cast wrappers C 345/347/406/408/412/413/417/549/660/1144/1146/
/// 1150/1152/1153/1157/1163 uses, each keeping the macro's name and its result type (the GLib
/// cast chains are owed, as above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_WIDGET {
  ($o:expr) => { ($o) as *mut GtkWidget };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_WINDOW {
  ($o:expr) => { ($o) as *mut GtkWindow };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_CONTAINER {
  ($o:expr) => { ($o) as *mut GtkContainer };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_CONTAINER_CLASS {
  ($o:expr) => { ($o) as *mut GtkContainerClass };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_FILE_CHOOSER {
  ($o:expr) => { ($o) as *mut GtkFileChooser };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_NATIVE_DIALOG {
  ($o:expr) => { ($o) as *mut GtkNativeDialog };
}
/// C: gtk/gtk.h: the four type-check macros C 499/655/665/992 asks, each keeping the macro's name
/// and its result type (`gboolean`).  C's bodies are `G_TYPE_CHECK_INSTANCE_TYPE` chains, which
/// need the type system the port has not got (the `src/gtkutil.rs` 876-893 treatment).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_IS_TEXT_VIEW {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_IS_WIDGET {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_IS_CONTAINER {
  ($o:expr) => { (! ($o).is_null ()) };
}
/// C: gtk/gtkenums.h: `GTK_RESPONSE_ACCEPT` (C 1155) and `GTK_FILE_CHOOSER_ACTION_OPEN` (C 1145).
/// Their values are GTK's; placeholders here (the enumerator note above).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_RESPONSE_ACCEPT: GtkResponseType = 0;               // placeholder, not GTK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_FILE_CHOOSER_ACTION_OPEN: GtkFileChooserAction = 0; // placeholder, not GTK's value
/// C: gobject/gobject.h: `#define G_OBJECT_GET_CLASS(object) (G_TYPE_INSTANCE_GET_CLASS ((object),
/// G_TYPE_OBJECT, GObjectClass))` - C 660's cast; the port keeps the macro's name and its result
/// type (`gpointer`), and GLib's chain is owed (as above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_OBJECT_GET_CLASS {
  ($o:expr) => { ($o) as gpointer };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 830-832/867-869/1036-1037/1053-1054: `gtk_widget_translate_coordinates (…, &xoff, &yoff);`
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_translate_coordinates (GtkWidget *src_widget,
  /// GtkWidget *dest_widget, int src_x, int src_y, int *dest_x, int *dest_y);`
  pub fn gtk_widget_translate_coordinates (src_widget: *mut GtkWidget, dest_widget: *mut GtkWidget,
                                           src_x: c_int, src_y: c_int, dest_x: *mut c_int,
                                           dest_y: *mut c_int) -> gboolean;
  /// C 940: `if (gtk_widget_get_has_window (widget))`
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_get_has_window (GtkWidget *widget);`
  pub fn gtk_widget_get_has_window (widget: *mut GtkWidget) -> gboolean;
  /// C 408: `gtk_widget_queue_allocate (GTK_WIDGET (xw->widget_osr));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_queue_allocate (GtkWidget *widget);`
  pub fn gtk_widget_queue_allocate (widget: *mut GtkWidget);
  /// C 222/679/681: `gtk_main_do_event (eventcopy);`
  /// C: gtk/gtkmain.h: `void gtk_main_do_event (GdkEvent *event);`
  pub fn gtk_main_do_event (event: *mut GdkEvent);
  /// C 1001-1002: `gtk_container_forall (GTK_CONTAINER (widget), find_widget_cb, &new_data);`
  /// C: gtk/gtkcontainer.h: `void gtk_container_forall (GtkContainer *container, GtkCallback
  /// callback, gpointer callback_data);`
  pub fn gtk_container_forall (container: *mut GtkContainer, callback: GtkCallback,
                               callback_data: gpointer);
  /// C 347: `gtk_container_check_resize (GTK_CONTAINER (xw->widgetwindow_osr));`
  /// C: gtk/gtkcontainer.h: `void gtk_container_check_resize (GtkContainer *container);`
  pub fn gtk_container_check_resize (container: *mut GtkContainer);
  /// C 549: `widget = gtk_window_get_focus (GTK_WINDOW (xw->widgetwindow_osr));`
  /// C: gtk/gtkwindow.h: `GtkWidget *gtk_window_get_focus (GtkWindow *window);`
  pub fn gtk_window_get_focus (window: *mut GtkWindow) -> *mut GtkWidget;
  /// C 670: `gtk_bindings_activate (G_OBJECT! (temp), keycode, modifiers);`
  /// C: gtk/gtkbindings.h: `gboolean gtk_bindings_activate (GObject *object, guint keyval,
  /// GdkModifierType modifiers);`
  pub fn gtk_bindings_activate (object: *mut GObject, keyval: guint,
                                modifiers: GdkModifierType) -> gboolean;
  /// C 344: `xw->widgetwindow_osr = gtk_offscreen_window_new ();`
  /// C: gtk/gtkwindow.h: `GtkWidget *gtk_offscreen_window_new (void);`
  pub fn gtk_offscreen_window_new () -> *mut GtkWidget;
  /// C 1143-1146: `chooser = gtk_file_chooser_native_new ("Select file", GTK_WINDOW (…),
  /// GTK_FILE_CHOOSER_ACTION_OPEN, "Select", "Cancel");`
  /// C: gtk/gtkfilechoosernative.h: `GtkFileChooserNative *gtk_file_chooser_native_new (const char
  /// *title, GtkWindow *parent, GtkFileChooserAction action, const char *accept_label, const char
  /// *cancel_label);`
  pub fn gtk_file_chooser_native_new (title: *const c_char, parent: *mut GtkWindow,
                                      action: GtkFileChooserAction, accept_label: *const c_char,
                                      cancel_label: *const c_char) -> *mut GtkFileChooserNative;
  /// C 1150: `gtk_file_chooser_set_select_multiple (GTK_FILE_CHOOSER (chooser),
  /// select_multiple_p);`
  /// C: gtk/gtkfilechooser.h: `void gtk_file_chooser_set_select_multiple (GtkFileChooser *chooser,
  /// gboolean select_multiple);`
  pub fn gtk_file_chooser_set_select_multiple (chooser: *mut GtkFileChooser,
                                               select_multiple: gboolean);
  /// C 1152: `gtk_file_chooser_add_filter (GTK_FILE_CHOOSER (chooser), filter);`
  /// C: gtk/gtkfilechooser.h: `void gtk_file_chooser_add_filter (GtkFileChooser *chooser,
  /// GtkFileFilter *filter);`
  pub fn gtk_file_chooser_add_filter (chooser: *mut GtkFileChooser, filter: *mut GtkFileFilter);
  /// C 1163: `filenames = gtk_file_chooser_get_filenames (GTK_FILE_CHOOSER (chooser));`
  /// C: gtk/gtkfilechooser.h: `GSList *gtk_file_chooser_get_filenames (GtkFileChooser *chooser);`
  pub fn gtk_file_chooser_get_filenames (chooser: *mut GtkFileChooser) -> *mut GSList;
  /// C 1153: `response = gtk_native_dialog_run (GTK_NATIVE_DIALOG (chooser));`
  /// C: gtk/gtknativedialog.h: `gint gtk_native_dialog_run (GtkNativeDialog *self);`
  pub fn gtk_native_dialog_run (self_: *mut GtkNativeDialog) -> gint;
  /// C 1157/1177: `gtk_native_dialog_destroy (GTK_NATIVE_DIALOG (chooser));`
  /// C: gtk/gtknativedialog.h: `void gtk_native_dialog_destroy (GtkNativeDialog *self);`
  pub fn gtk_native_dialog_destroy (self_: *mut GtkNativeDialog);
}

/* ----------------------------------------------------------------------------------------------
   WebKitGTK.  No WebKit tree on this machine, so each citation names the header alone, every
   record is the `[u8; 0]` shape (the port only ever holds their pointers) and every enumerator is
   a placeholder (the note above).  **UNVERIFIED** (P-88 ②; owed).
   ---------------------------------------------------------------------------------------------- */
/// C: webkit2/WebKitWebView.h: `typedef struct _WebKitWebView WebKitWebView;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitWebView
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitWebContext.h: `typedef struct _WebKitWebContext WebKitWebContext;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitWebContext
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitSettings.h: `typedef struct _WebKitSettings WebKitSettings;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitSettings
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitHitTestResult.h: `typedef struct _WebKitHitTestResult WebKitHitTestResult;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitHitTestResult
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitFileChooserRequest.h: `typedef struct _WebKitFileChooserRequest
/// WebKitFileChooserRequest;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitFileChooserRequest
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitScriptDialog.h: `typedef struct _WebKitScriptDialog WebKitScriptDialog;` -
/// the type C 71's forward declaration names.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitScriptDialog
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitPolicyDecision.h: `typedef struct _WebKitPolicyDecision
/// WebKitPolicyDecision;` - the type C 109's forward declaration names.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitPolicyDecision
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitNavigationAction.h: `typedef struct _WebKitNavigationAction
/// WebKitNavigationAction;` - the type C 106's forward declaration names.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitNavigationAction
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitBackForwardList.h: `typedef struct _WebKitBackForwardList
/// WebKitBackForwardList;` - C 3150's `list` (slice 4).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitBackForwardList
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitBackForwardListItem.h: `typedef struct _WebKitBackForwardListItem
/// WebKitBackForwardListItem;` - C 3151's `it` (slice 4).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitBackForwardListItem
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitDownload.h: `typedef struct _WebKitDownload WebKitDownload;` - the type
/// C 105's forward declaration names.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitDownload
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitCookieManager.h: `typedef struct _WebKitCookieManager
/// WebKitCookieManager;` - C 3879's `manager` (slice 7).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitCookieManager
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitCookieManager.h: `typedef enum { WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT,
/// WEBKIT_COOKIE_PERSISTENT_STORAGE_SQLITE } WebKitCookiePersistentStorage;` - the value C 3892
/// passes.  **UNVERIFIED** (a distinct placeholder; no WebKit tree on this machine).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT: c_int = 0;
/// C: webkit2/WebKitFindController.h: `typedef struct _WebKitFindController
/// WebKitFindController;` - C 3571/3621/3654/3687's `controller` (slice 6).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitFindController
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitFindController.h: `typedef enum { WEBKIT_FIND_OPTIONS_NONE, … }
/// WebKitFindOptions;` - the type C 3572's `opt` is OR'd into (slice 6).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitFindOptions = c_int;
/// C: webkit2/WebKitPolicyDecision.h: `typedef enum { WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION,
/// WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION, WEBKIT_POLICY_DECISION_TYPE_RESPONSE }
/// WebKitPolicyDecisionType;` - the type C 110's forward declaration names.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitPolicyDecisionType = c_int;
/// C: webkit2/WebKitWebView.h: `typedef enum { WEBKIT_LOAD_STARTED, WEBKIT_LOAD_REDIRECTED,
/// WEBKIT_LOAD_COMMITTED, WEBKIT_LOAD_FINISHED } WebKitLoadEvent;` - the type C 100's forward
/// declaration names.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitLoadEvent = c_int;
/// C: webkit2/WebKitSettings.h: `typedef enum { WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND,
/// WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS, WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER }
/// WebKitHardwareAccelerationPolicy;` - the type of C 402's argument.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitHardwareAccelerationPolicy = c_int;
/// C: webkit2/WebKitWebView.h: `#define WEBKIT_WEB_VIEW(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj),
/// WEBKIT_TYPE_WEB_VIEW, WebKitWebView))` - C 380/387/392/413's casts keep the macro's name and
/// its result type; WebKit's cast chain is GLib's and owed (as above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! WEBKIT_WEB_VIEW {
  ($o:expr) => { ($o) as *mut WebKitWebView };
}
/// C: webkit2/WebKitWebView.h: `#define WEBKIT_IS_WEB_VIEW(obj) (G_TYPE_CHECK_INSTANCE_TYPE
/// ((obj), WEBKIT_TYPE_WEB_VIEW))` - C 655's test; the port keeps the macro's name and its result
/// type (`gboolean`), the `GTK_IS_*` treatment above.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! WEBKIT_IS_WEB_VIEW {
  ($o:expr) => { (! ($o).is_null ()) };
}
/// C: webkit2/WebKitHitTestResult.h: `WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT`,
/// `…_LINK`, `…_IMAGE`, `…_MEDIA`, `…_EDITABLE`, `…_SELECTION`, `…_SCROLLBAR` - the six C
/// 1070-1078 tests.  Bit flags in WebKit's header; the values are WebKit's and are placeholders
/// here (the enumerator note above).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE: guint = 1 << 0;   // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION: guint = 1 << 1;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT: guint = 1 << 2;   // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR: guint = 1 << 3;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK: guint = 1 << 4;       // placeholder
/// C: webkit2/WebKitSettings.h: `WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER` (C 402).
/// **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER:
  WebKitHardwareAccelerationPolicy = 0;                              // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 337: `WebKitWebContext *webkit_context = webkit_web_context_get_default ();`
  /// C: webkit2/WebKitWebContext.h: `WebKitWebContext *webkit_web_context_get_default (void);`
  pub fn webkit_web_context_get_default () -> *mut WebKitWebContext;
  /// C 340: `if (!webkit_web_context_get_sandbox_enabled (webkit_context))`
  /// C: webkit2/WebKitWebContext.h: `gboolean webkit_web_context_get_sandbox_enabled
  /// (WebKitWebContext *context);`
  pub fn webkit_web_context_get_sandbox_enabled (context: *mut WebKitWebContext) -> gboolean;
  /// C 341: `webkit_web_context_set_sandbox_enabled (webkit_context, TRUE);`
  /// C: webkit2/WebKitWebContext.h: `void webkit_web_context_set_sandbox_enabled
  /// (WebKitWebContext *context, gboolean enabled);`
  pub fn webkit_web_context_set_sandbox_enabled (context: *mut WebKitWebContext, enabled: gboolean);
  /// C 357: `WebKitWebContext *ctx = webkit_web_context_new ();`
  /// C: webkit2/WebKitWebContext.h: `WebKitWebContext *webkit_web_context_new (void);`
  pub fn webkit_web_context_new () -> *mut WebKitWebContext;
  /// C 358: `xw->widget_osr = webkit_web_view_new_with_context (ctx);`
  /// C: webkit2/WebKitWebView.h: `GtkWidget *webkit_web_view_new_with_context (WebKitWebContext
  /// *context);`
  pub fn webkit_web_view_new_with_context (context: *mut WebKitWebContext) -> *mut WebKitWebView;
  /// C 388: `xw->widget_osr = webkit_web_view_new_with_related_view (related_view);`
  /// C: webkit2/WebKitWebView.h: `GtkWidget *webkit_web_view_new_with_related_view
  /// (WebKitWebView *web_view);`
  pub fn webkit_web_view_new_with_related_view (web_view: *mut WebKitWebView)
    -> *mut WebKitWebView;
  /// C 380: `webkit_web_view_load_uri (WEBKIT_WEB_VIEW (xw->widget_osr), "about:blank");`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_load_uri (WebKitWebView *web_view, const
  /// gchar *uri);`
  pub fn webkit_web_view_load_uri (web_view: *mut WebKitWebView, uri: *const gchar);
  /// C 392: `settings = webkit_web_view_get_settings (WEBKIT_WEB_VIEW (xw->widget_osr));`
  /// C: webkit2/WebKitWebView.h: `WebKitSettings *webkit_web_view_get_settings (WebKitWebView
  /// *web_view);`
  pub fn webkit_web_view_get_settings (web_view: *mut WebKitWebView) -> *mut WebKitSettings;
  /// C 401-402: `webkit_settings_set_hardware_acceleration_policy (settings,
  /// WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER);`
  /// C: webkit2/WebKitSettings.h: `void webkit_settings_set_hardware_acceleration_policy
  /// (WebKitSettings *settings, WebKitHardwareAccelerationPolicy policy);`
  pub fn webkit_settings_set_hardware_acceleration_policy (settings: *mut WebKitSettings,
                                                           policy:
                                                             WebKitHardwareAccelerationPolicy);
  /// C 1090: `xw->hit_result = webkit_hit_test_result_get_context (res);`
  /// C: webkit2/WebKitHitTestResult.h: `guint webkit_hit_test_result_get_context
  /// (WebKitHitTestResult *hit_test_result);`
  pub fn webkit_hit_test_result_get_context (hit_test_result: *mut WebKitHitTestResult) -> guint;
  /// C 1147: `filter = webkit_file_chooser_request_get_mime_types_filter (request);`
  /// C: webkit2/WebKitFileChooserRequest.h: `GtkFileFilter *
  /// webkit_file_chooser_request_get_mime_types_filter (WebKitFileChooserRequest *request);`
  pub fn webkit_file_chooser_request_get_mime_types_filter (request:
                                                              *mut WebKitFileChooserRequest)
    -> *mut GtkFileFilter;
  /// C 1148: `select_multiple_p = webkit_file_chooser_request_get_select_multiple (request);`
  /// C: webkit2/WebKitFileChooserRequest.h: `gboolean
  /// webkit_file_chooser_request_get_select_multiple (WebKitFileChooserRequest *request);`
  pub fn webkit_file_chooser_request_get_select_multiple (request: *mut WebKitFileChooserRequest)
    -> gboolean;
  /// C 1158: `webkit_file_chooser_request_cancel (request);`
  /// C: webkit2/WebKitFileChooserRequest.h: `void webkit_file_chooser_request_cancel
  /// (WebKitFileChooserRequest *request);`
  pub fn webkit_file_chooser_request_cancel (request: *mut WebKitFileChooserRequest);
  /// C 1172: `webkit_file_chooser_request_select_files (request, (const gchar **) files);`
  /// C: webkit2/WebKitFileChooserRequest.h: `void webkit_file_chooser_request_select_files
  /// (WebKitFileChooserRequest *request, const gchar * const *files);`
  pub fn webkit_file_chooser_request_select_files (request: *mut WebKitFileChooserRequest,
                                                   files: *const *const gchar);
  /// C 3061: `const gchar *uri = webkit_web_view_get_uri (wkwv);`
  /// C: webkit2/WebKitWebView.h: `const gchar *webkit_web_view_get_uri (WebKitWebView *web_view);`
  pub fn webkit_web_view_get_uri (web_view: *mut WebKitWebView) -> *const gchar;
  /// C 3079: `const gchar *title = webkit_web_view_get_title (wkwv);`
  /// C: webkit2/WebKitWebView.h: `const gchar *webkit_web_view_get_title (WebKitWebView
  /// *web_view);`
  pub fn webkit_web_view_get_title (web_view: *mut WebKitWebView) -> *const gchar;
  /// C 3107: `value = webkit_web_view_get_estimated_load_progress (webview);`
  /// C: webkit2/WebKitWebView.h: `gdouble webkit_web_view_get_estimated_load_progress
  /// (WebKitWebView *web_view);` - GLib's `gdouble` is C's `double`, which the unit spells
  /// `c_double` (the `WebKitHardwareAccelerationPolicy`/`gboolean` spellings above keep GLib's
  /// names because they are types the header itself defines; `gdouble` is a typedef of `double`).
  pub fn webkit_web_view_get_estimated_load_progress (web_view: *mut WebKitWebView) -> c_double;
  /// C 3154: `webkit_web_view_reload (wkwv);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_reload (WebKitWebView *web_view);`
  pub fn webkit_web_view_reload (web_view: *mut WebKitWebView);
  /// C 3157: `list = webkit_web_view_get_back_forward_list (wkwv);`
  /// C: webkit2/WebKitWebView.h: `WebKitBackForwardList *webkit_web_view_get_back_forward_list
  /// (WebKitWebView *web_view);`
  pub fn webkit_web_view_get_back_forward_list (web_view: *mut WebKitWebView)
    -> *mut WebKitBackForwardList;
  /// C 3158: `it = webkit_back_forward_list_get_nth_item (list, XFIXNUM (rel_pos));`
  /// C: webkit2/WebKitBackForwardList.h: `WebKitBackForwardListItem
  /// *webkit_back_forward_list_get_nth_item (WebKitBackForwardList *back_forward_list,
  /// gint index);`
  pub fn webkit_back_forward_list_get_nth_item (back_forward_list: *mut WebKitBackForwardList,
						index: c_int) -> *mut WebKitBackForwardListItem;
  /// C 3163: `webkit_web_view_go_to_back_forward_list_item (wkwv, it);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_go_to_back_forward_list_item (WebKitWebView
  /// *web_view, WebKitBackForwardListItem *list_item);`
  pub fn webkit_web_view_go_to_back_forward_list_item (web_view: *mut WebKitWebView,
						       list_item: *mut WebKitBackForwardListItem);
  /// C 3182-3185: `webkit_web_view_set_zoom_level (WEBKIT_WEB_VIEW (xw->widget_osr),
  /// webkit_web_view_get_zoom_level (WEBKIT_WEB_VIEW (xw->widget_osr)) + zoom_change);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_set_zoom_level (WebKitWebView *web_view,
  /// gdouble zoom_level);`
  pub fn webkit_web_view_set_zoom_level (web_view: *mut WebKitWebView, zoom_level: c_double);
  /// C 3184-3185: `… webkit_web_view_get_zoom_level (WEBKIT_WEB_VIEW (xw->widget_osr)) …`
  /// C: webkit2/WebKitWebView.h: `gdouble webkit_web_view_get_zoom_level (WebKitWebView
  /// *web_view);`
  pub fn webkit_web_view_get_zoom_level (web_view: *mut WebKitWebView) -> c_double;
  /// C 3241-3245: `webkit_web_view_run_javascript (WEBKIT_WEB_VIEW (xw->widget_osr),
  /// script_string, NULL, webkit_javascript_finished_cb, (gpointer) idx);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_run_javascript (WebKitWebView *web_view,
  /// const gchar *script, GCancellable *cancellable, GAsyncReadyCallback callback, gpointer
  /// user_data);`
  pub fn webkit_web_view_run_javascript (web_view: *mut WebKitWebView, script: *const gchar,
					 cancellable: *mut c_void,
					 callback: GAsyncReadyCallback, user_data: gpointer);
  /// C 3601/3634/3667/3700: `controller = webkit_web_view_get_find_controller (webview);`
  /// C: webkit2/WebKitWebView.h: `WebKitFindController *webkit_web_view_get_find_controller
  /// (WebKitWebView *web_view);`
  pub fn webkit_web_view_get_find_controller (web_view: *mut WebKitWebView)
    -> *mut WebKitFindController;
  /// C 3602: `webkit_find_controller_search (controller, g_query, opt, G_MAXUINT);`
  /// C: webkit2/WebKitFindController.h: `void webkit_find_controller_search
  /// (WebKitFindController *find_controller, const gchar *search_text, guint32 find_options,
  /// guint max_match_count);`
  pub fn webkit_find_controller_search (find_controller: *mut WebKitFindController,
					search_text: *const gchar, find_options: guint32,
					max_match_count: guint);
  /// C 3635: `webkit_find_controller_search_next (controller);`
  /// C: webkit2/WebKitFindController.h: `void webkit_find_controller_search_next
  /// (WebKitFindController *find_controller);`
  pub fn webkit_find_controller_search_next (find_controller: *mut WebKitFindController);
  /// C 3668: `webkit_find_controller_search_previous (controller);`
  /// C: webkit2/WebKitFindController.h: `void webkit_find_controller_search_previous
  /// (WebKitFindController *find_controller);`
  pub fn webkit_find_controller_search_previous (find_controller: *mut WebKitFindController);
  /// C 3701: `webkit_find_controller_search_finish (controller);`
  /// C: webkit2/WebKitFindController.h: `void webkit_find_controller_search_finish
  /// (WebKitFindController *find_controller);`
  pub fn webkit_find_controller_search_finish (find_controller: *mut WebKitFindController);
  /// C 3765: `webkit_web_view_load_html (webview, data, uri);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_load_html (WebKitWebView *web_view, const
  /// gchar *content, const gchar *base_uri);`
  pub fn webkit_web_view_load_html (web_view: *mut WebKitWebView, content: *const gchar,
				    base_uri: *const gchar);
  /// C 3814: `item = webkit_back_forward_list_get_current_item (list);`
  /// C: webkit2/WebKitBackForwardList.h: `WebKitBackForwardListItem
  /// *webkit_back_forward_list_get_current_item (WebKitBackForwardList *back_forward_list);`
  pub fn webkit_back_forward_list_get_current_item (back_forward_list: *mut WebKitBackForwardList)
    -> *mut WebKitBackForwardListItem;
  /// C 3819/3832/3850: `item_title = webkit_back_forward_list_item_get_title (item);`
  /// C: webkit2/WebKitBackForwardListItem.h: `const gchar *webkit_back_forward_list_item_get_title
  /// (WebKitBackForwardListItem *item);`
  pub fn webkit_back_forward_list_item_get_title (item: *mut WebKitBackForwardListItem)
    -> *const gchar;
  /// C 3820/3833/3851: `item_uri = webkit_back_forward_list_item_get_uri (item);`
  /// C: webkit2/WebKitBackForwardListItem.h: `const gchar *webkit_back_forward_list_item_get_uri
  /// (WebKitBackForwardListItem *item);`
  pub fn webkit_back_forward_list_item_get_uri (item: *mut WebKitBackForwardListItem)
    -> *const gchar;
  /// C 3825: `parent = webkit_back_forward_list_get_back_list_with_limit (list, lim);`
  /// C: webkit2/WebKitBackForwardList.h: `GList
  /// *webkit_back_forward_list_get_back_list_with_limit (WebKitBackForwardList *back_forward_list,
  /// guint limit);`
  pub fn webkit_back_forward_list_get_back_list_with_limit
    (back_forward_list: *mut WebKitBackForwardList, limit: c_uint) -> *mut GList;
  /// C 3843: `parent = webkit_back_forward_list_get_forward_list_with_limit (list, lim);`
  /// C: webkit2/WebKitBackForwardList.h: `GList
  /// *webkit_back_forward_list_get_forward_list_with_limit (WebKitBackForwardList
  /// *back_forward_list, guint limit);`
  pub fn webkit_back_forward_list_get_forward_list_with_limit
    (back_forward_list: *mut WebKitBackForwardList, limit: c_uint) -> *mut GList;
  /// C 3888: `context = webkit_web_view_get_context (webview);`
  /// C: webkit2/WebKitWebView.h: `WebKitWebContext *webkit_web_view_get_context (WebKitWebView
  /// *web_view);`
  pub fn webkit_web_view_get_context (web_view: *mut WebKitWebView) -> *mut WebKitWebContext;
  /// C 3889: `manager = webkit_web_context_get_cookie_manager (context);`
  /// C: webkit2/WebKitWebContext.h: `WebKitCookieManager *webkit_web_context_get_cookie_manager
  /// (WebKitWebContext *context);`
  pub fn webkit_web_context_get_cookie_manager (context: *mut WebKitWebContext)
    -> *mut WebKitCookieManager;
  /// C 3890-3892: `webkit_cookie_manager_set_persistent_storage (manager, SSDATA (ENCODE_UTF_8
  /// (file)), WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT);`
  /// C: webkit2/WebKitCookieManager.h: `void webkit_cookie_manager_set_persistent_storage
  /// (WebKitCookieManager *cookie_manager, const gchar *filename,
  /// WebKitCookiePersistentStorage storage);`
  pub fn webkit_cookie_manager_set_persistent_storage (cookie_manager: *mut WebKitCookieManager,
						       filename: *const gchar,
						       storage: c_int);
  /// C 3918: `webkit_web_view_stop_loading (webview);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_stop_loading (WebKitWebView *web_view);`
  pub fn webkit_web_view_stop_loading (web_view: *mut WebKitWebView);
}

/* C 56-59: the file's four file-scope objects.  The first three are C's tentative definitions
   (zero words, i.e. `Qnil`); `xwidget_counter`'s initializer is C's own.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
static mut id_to_xwidget_map: Lisp_Object = lisp_h_Qnil ();           // C 56 (tentative, i.e. Qnil)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
static mut internal_xwidget_view_list: Lisp_Object = lisp_h_Qnil ();  // C 57 (tentative, i.e. Qnil)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
static mut internal_xwidget_list: Lisp_Object = lisp_h_Qnil ();       // C 58 (tentative, i.e. Qnil)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
static mut xwidget_counter: u32 = 0;                                  // C 59

/* C 61-77: `#ifdef USE_GTK` with C 62's `#ifdef HAVE_X_WINDOWS` and C 64's
   `#if WEBKIT_CHECK_VERSION (2, 34, 0)` inside it.  C 68-76 are seven forward declarations;
   C's prototype has no Rust counterpart when the definition follows (PORTING.md 0.1-12), so they
   are dropped and the definitions stay at C's position - `record_osr_embedder` (C 766),
   `from_embedder` (C 806), `to_embedder` (C 843), `find_suitable_keyboard` (C 911) and
   `pick_embedded_child` (C 740) are inside this slice; `offscreen_damage_event` (C 2247),
   `synthesize_focus_in_event` (C 2110) and `webkit_script_dialog_cb` (C 2647) are in the
   remainder and stay as C's call sites (the `docs/ports/gtkutil.md` Deviations-3 shape).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
static mut x_window_to_xwv_map: Lisp_Object = lisp_h_Qnil ();         // C 63 (tentative, i.e. Qnil)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows",
            feature = "webkit_check_version_ge_2_34"))]
static mut dummy_tooltip_string: Lisp_Object = lisp_h_Qnil ();        // C 65 (tentative, i.e. Qnil)

/* C 79-83: `ALLOCATE_PSEUDOVECTOR` expands to a `allocate_pseudovector` call with `VECSIZE`/
   `PSEUDOVECSIZE` of the record's marker field; `include/lisp.rs` carries the macro.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn allocate_xwidget () -> *mut xwidget                         // C 79-83
{
  return ALLOCATE_PSEUDOVECTOR! (xwidget, script_callbacks, pvec_type::PVEC_XWIDGET);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn allocate_xwidget_view () -> *mut xwidget_view               // C 85-89
{
  return ALLOCATE_PSEUDOVECTOR! (xwidget_view, w, pvec_type::PVEC_XWIDGET_VIEW);
}

/* C 91-92: the two setters.  C writes them as macros over `XSETPSEUDOVECTOR`; the port keeps the
   names and the bodies (PORTING.md 0.2).  */
/// C: #define XSETXWIDGET(a, b) XSETPSEUDOVECTOR (a, b, PVEC_XWIDGET)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! XSETXWIDGET {
  ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_XWIDGET) };
}
/// C: #define XSETXWIDGET_VIEW(a, b) XSETPSEUDOVECTOR (a, b, PVEC_XWIDGET_VIEW)
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! XSETXWIDGET_VIEW {
  ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_XWIDGET_VIEW) };
}

/* C 94-96: `xwidget_view_lookup` and `kill_xwidget`'s forward declarations - dropped
   (PORTING.md 0.1-12); both definitions are in the remainder (C 4081 and C 3727).  */

/* C 98-128: `#ifdef USE_GTK`.  C 99-116 are seven more forward declarations (dropped, the
   `webkit_*_cb` definitions are in the remainder: C 2427/2520/2561/2407/2576 and
   `run_file_chooser_cb` C 1122, inside this slice), C 118-125 is the record below and C 127 the
   `find_widget` declaration (its definition is C 929, inside this slice).  */
/// C 118-125: `struct widget_search_data { int x; int y; bool foundp; bool first; GtkWidget
/// *data; };`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct widget_search_data
{
  pub x: c_int,                  // C 120: `int x;`
  pub y: c_int,                  // C 121: `int y;`
  pub foundp: bool,              // C 122: `bool foundp;`
  pub first: bool,               // C 123: `bool first;`
  pub data: *mut GtkWidget,      // C 124: `GtkWidget *data;`
}

/* C 130-228: `#ifdef HAVE_PGTK`.  C 131-132 is the `mouse_target_changed` forward declaration
   (dropped; its definition is C 1113, inside this slice).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe fn xw_forward_event_translate (event: *mut GdkEvent, xv: *mut xwidget_view,
                                      xw: *mut xwidget) -> c_int   // C 134-202
{
  let mut widget: *mut GtkWidget;
  let mut new_x: c_int = 0;
  let mut new_y: c_int = 0;

  match (*event).r#type
    {
    GDK_BUTTON_PRESS | GDK_BUTTON_RELEASE | GDK_2BUTTON_PRESS | GDK_3BUTTON_PRESS =>
      {
        widget = find_widget_at_pos ((*xw).widgetwindow_osr,
                                     lrint ((*event).button.x - (*xv).clip_left as c_double)
                                       as c_int,
                                     lrint ((*event).button.y - (*xv).clip_top as c_double)
                                       as c_int,
                                     &raw mut new_x, &raw mut new_y, false,
                                     std::ptr::null_mut ());
        if ! widget.is_null ()
          {
            (*event).any.window = gtk_widget_get_window (widget);
            (*event).button.x = new_x as c_double;
            (*event).button.y = new_y as c_double;
            return 1;
          }
        return 0;
      }
    GDK_SCROLL =>
      {
        widget = find_widget_at_pos ((*xw).widgetwindow_osr,
                                     lrint ((*event).scroll.x - (*xv).clip_left as c_double)
                                       as c_int,
                                     lrint ((*event).scroll.y - (*xv).clip_top as c_double)
                                       as c_int,
                                     &raw mut new_x, &raw mut new_y, false,
                                     std::ptr::null_mut ());
        if ! widget.is_null ()
          {
            (*event).any.window = gtk_widget_get_window (widget);
            (*event).scroll.x = new_x as c_double;
            (*event).scroll.y = new_y as c_double;
            return 1;
          }
        return 0;
      }
    GDK_MOTION_NOTIFY =>
      {
        widget = find_widget_at_pos ((*xw).widgetwindow_osr,
                                     lrint ((*event).motion.x - (*xv).clip_left as c_double)
                                       as c_int,
                                     lrint ((*event).motion.y - (*xv).clip_top as c_double)
                                       as c_int,
                                     &raw mut new_x, &raw mut new_y, false,
                                     std::ptr::null_mut ());
        if ! widget.is_null ()
          {
            (*event).any.window = gtk_widget_get_window (widget);
            (*event).motion.x = new_x as c_double;
            (*event).motion.y = new_y as c_double;
            return 1;
          }
        return 0;
      }
    GDK_ENTER_NOTIFY | GDK_LEAVE_NOTIFY =>
      {
        widget = find_widget_at_pos ((*xw).widgetwindow_osr,
                                     lrint ((*event).crossing.x - (*xv).clip_left as c_double)
                                       as c_int,
                                     lrint ((*event).crossing.y - (*xv).clip_top as c_double)
                                       as c_int,
                                     &raw mut new_x, &raw mut new_y, false,
                                     std::ptr::null_mut ());
        if ! widget.is_null ()
          {
            (*event).any.window = gtk_widget_get_window (widget);
            (*event).crossing.x = new_x as c_double;
            (*event).crossing.y = new_y as c_double;
            return 1;
          }
        return 0;
      }
    _ =>
      {
        return 0;
      }
    }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe extern "C" fn xw_forward_event_from_view (widget: *mut GtkWidget, event: *mut GdkEvent,
                                                 user_data: gpointer) -> gboolean  // C 204-227
{
  let xv: *mut xwidget_view = user_data as *mut xwidget_view;
  let xw: *mut xwidget = XXWIDGET ((*xv).model);
  let mut eventcopy: *mut GdkEvent;
  let mut translated_p: bool;

  if NILP ((*xw).buffer)
    { return TRUE; }

  eventcopy = gdk_event_copy (event);
  translated_p = xw_forward_event_translate (eventcopy, xv, xw) != 0;
  record_osr_embedder (xv);

  g_object_ref ((*eventcopy).any.window as gpointer);
  if translated_p
    { gtk_main_do_event (eventcopy); }
  gdk_event_free (eventcopy);

  /* Don't propagate this event further.  */
  return TRUE;
}
/* C 228: `#endif` (the `HAVE_PGTK` arm).  */

/* C 230-278: `#ifdef HAVE_X_WINDOWS`.  */
/// C 231-236: `enum xw_crossing_mode { XW_CROSSING_LEFT, XW_CROSSING_ENTERED, XW_CROSSING_NONE };`
/// - C's own enumeration values.  C's enum ABI is `c_int` (PORTING.md 1.3).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub type xw_crossing_mode = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const XW_CROSSING_LEFT: xw_crossing_mode = 0;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const XW_CROSSING_ENTERED: xw_crossing_mode = 1;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const XW_CROSSING_NONE: xw_crossing_mode = 2;

/* C 260/262: the two X11 masks this slice reads.  X11 *is* on this machine, so the values are the
   header's own text (`X11/X.h` 221/223); the mirror `include/xlib.rs` does not carry them
   (P-72 rule 4; the two rows are owed there).  */
/// C: X11/X.h 221: `#define ShiftMask (1<<0)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const ShiftMask: c_uint = 1 << 0;
/// C: X11/X.h 223: `#define ControlMask (1<<2)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const ControlMask: c_uint = 1 << 2;

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_translate_x_modifiers (dpyinfo: *mut x_display_info,
                                    modifiers: c_uint) -> guint    // C 238-266
{
  let mut mods: guint = 0;

  if modifiers & (*dpyinfo).meta_mod_mask != 0
    {
      /* GDK always assumes Mod1 is alt, but that's no reason for
	 us to make that mistake as well.  */
      if (*dpyinfo).alt_mod_mask == 0
	{ mods |= GDK_MOD1_MASK as guint; }
      else
	{ mods |= GDK_META_MASK as guint; }
    }

  if modifiers & (*dpyinfo).alt_mod_mask != 0
    { mods |= GDK_MOD1_MASK as guint; }
  if modifiers & (*dpyinfo).super_mod_mask != 0
    { mods |= GDK_SUPER_MASK as guint; }
  if modifiers & (*dpyinfo).hyper_mod_mask != 0
    { mods |= GDK_HYPER_MASK as guint; }
  if modifiers & ControlMask != 0
    { mods |= GDK_CONTROL_MASK as guint; }
  if modifiers & ShiftMask != 0
    { mods |= GDK_SHIFT_MASK as guint; }

  return mods;
}

/* C 268-277: the three forward declarations `xw_maybe_synthesize_crossing`,
   `xw_notify_virtual_upwards_until` and `window_coords_from_toplevel` - dropped
   (PORTING.md 0.1-12); their definitions are C 1769/1650/1578, in the remainder.  */
/* C 278: `#endif` (the `HAVE_X_WINDOWS` arm).  */

/* C 296-301: the first `#ifdef USE_GTK` block inside `Fmake_xwidget`.  `xg_gtk_initialized` is
   gtkutil.c's, declared `extern` at gtkutil.h 242; `include/gtkutil.rs` carries the guard alone
   (P-72 rule 4; the declaration is owed there), so the `extern` stands here.  */
/// C: gtkutil.h 242: `extern bool xg_gtk_initialized;`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub static mut xg_gtk_initialized: bool;
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("make-xwidget", Fmake_xwidget, Smake_xwidget, 4, 7, 0,
       doc: "Make an xwidget of TYPE.\nIf BUFFER is nil, use the current buffer.\nIf BUFFER is a string and no such buffer exists, create it.\nTYPE is a symbol which can take one of the following values:\n\n- webkit\n\nRELATED is nil, or an xwidget.  When constructing a WebKit widget, it\nwill share the same settings and internal subprocess as RELATED.\nReturns the newly constructed xwidget, or nil if construction\nfails.",
       (r#type: Lisp_Object, title: Lisp_Object, width: Lisp_Object, height: Lisp_Object,
        arguments: Lisp_Object, buffer: Lisp_Object, related: Lisp_Object),
       {
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  if ! xg_gtk_initialized
    {
      error (c"make-xwidget: GTK has not been initialized".as_ptr ());
    }
  CHECK_SYMBOL (r#type);
  CHECK_FIXNAT (width);
  CHECK_FIXNAT (height);

  if ! EQ (r#type, Qwebkit ())
    {
      error (c"Bad xwidget type".as_ptr ());
    }

  Frequire (Qxwidget (), Qnil (), Qnil ());

  let xw: *mut xwidget = allocate_xwidget ();
  let mut val: Lisp_Object;
  (*xw).r#type = r#type;
  (*xw).title = title;
  (*xw).buffer = (if NILP (buffer) { Fcurrent_buffer () }
                  else { Fget_buffer_create (buffer, Qnil ()) });
  (*xw).height = XFIXNAT (height) as c_int;
  (*xw).width = XFIXNAT (width) as c_int;
  /* C 99's `bool_bf kill_without_query : 1` is recorded, not packed, by `include/xwidget.rs`
     (P-80's member-dispatch accessor is the fix), so the arm compile reports `no field
     kill_without_query` for the line below - a registered mirror-gap class (Deviations 6).  */
  (*xw).kill_without_query = false;
  XSETXWIDGET! (val, xw);
  internal_xwidget_list = Fcons (val, internal_xwidget_list);
  Vxwidget_list = Fcopy_sequence (internal_xwidget_list);
  (*xw).plist = Qnil ();
  xwidget_counter += 1;
  (*xw).xwidget_id = xwidget_counter;
  (*xw).find_text = std::ptr::null_mut ();

  Fputhash (make_fixnum ((*xw).xwidget_id as EMACS_INT), val, id_to_xwidget_map);

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  (*xw).widgetwindow_osr = std::ptr::null_mut ();
  (*xw).widget_osr = std::ptr::null_mut ();
  (*xw).hit_result = 0;
  if EQ ((*xw).r#type, Qwebkit ())
    {
      block_input ();
      let mut settings: *mut WebKitSettings;
      let webkit_context: *mut WebKitWebContext = webkit_web_context_get_default ();

      #[cfg (feature = "webkit_check_version_ge_2_26")]
      if webkit_web_context_get_sandbox_enabled (webkit_context) == 0
	{ webkit_web_context_set_sandbox_enabled (webkit_context, TRUE); }

      (*xw).widgetwindow_osr = gtk_offscreen_window_new ();
      gtk_window_resize (GTK_WINDOW! ((*xw).widgetwindow_osr), (*xw).width, (*xw).height);
      gtk_container_check_resize (GTK_CONTAINER! ((*xw).widgetwindow_osr));

      if EQ ((*xw).r#type, Qwebkit ())
        {
	  let mut related_view: *mut WebKitWebView;

	  if NILP (related)
	      || ! XWIDGETP (related)
	      || ! EQ ((*XXWIDGET (related)).r#type, Qwebkit ())
	    {
	      let ctx: *mut WebKitWebContext = webkit_web_context_new ();
	      (*xw).widget_osr = webkit_web_view_new_with_context (ctx) as *mut GtkWidget;
	      g_object_unref (ctx as gpointer);

	      g_signal_connect! (G_OBJECT! (ctx),
				 c"download-started".as_ptr (),
				 G_CALLBACK! (webkit_download_cb), xw);
	      /* In process.c:init_process_emacs(), we determine whether GLib
		 overrides Emacs' SIGCHLD handler (based on Glib version
		 and kernel; see process.c for details).

		 webkit_web_view_load_uri() is potentially affected by this
		 (through GSubProcess), so as a workaround we load a minimal
		 about:blank url and restore the SIGCHLD handler afterward.

		 With PGTK, this workaround has the unfortunate side-effect of
		 making the first load of an URI not work immediately (it needs
		 a refresh), so avoid the workaround when we can.

		 See thread
		 <https://lists.gnu.org/archive/html/emacs-devel/2026-04/msg00810.html>.  */
	      if glib_installs_sigchld_handler
		{
		  webkit_web_view_load_uri (WEBKIT_WEB_VIEW! ((*xw).widget_osr),
					    c"about:blank".as_ptr ());
		  catch_child_signal ();
		}
	    }
	  else
	    {
	      related_view = WEBKIT_WEB_VIEW! ((*XXWIDGET (related)).widget_osr);
	      (*xw).widget_osr = webkit_web_view_new_with_related_view (related_view)
		as *mut GtkWidget;
	    }

	  /* Enable the developer extras.  */
	  settings = webkit_web_view_get_settings (WEBKIT_WEB_VIEW! ((*xw).widget_osr));
	  g_object_set (G_OBJECT! (settings) as gpointer, c"enable-developer-extras".as_ptr (), TRUE,
			std::ptr::null_mut ());
	  g_object_set (G_OBJECT! (settings) as gpointer, c"enable-javascript".as_ptr (),
			(! xwidget_webkit_disable_javascript) as gboolean,
			std::ptr::null_mut ());
	  #[cfg (feature = "webkit_check_version_ge_2_16")]
	  {
	  /* The webkit view is created on an *offscreen* window, on
	     which hardware acceleration is not supported; we need to
	     tell webkitgtk _not_ to try that, since it will fail & abort
	     with recent versions.  */
	  webkit_settings_set_hardware_acceleration_policy
	    (settings, WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER);
	  }
	}

      gtk_widget_set_size_request (GTK_WIDGET! ((*xw).widget_osr), (*xw).width, (*xw).height);
      gtk_widget_queue_allocate (GTK_WIDGET! ((*xw).widget_osr));

      if EQ ((*xw).r#type, Qwebkit ())
        {
          gtk_container_add (GTK_CONTAINER! ((*xw).widgetwindow_osr),
                             GTK_WIDGET! (WEBKIT_WEB_VIEW! ((*xw).widget_osr)));
        }
      else
        {
          gtk_container_add (GTK_CONTAINER! ((*xw).widgetwindow_osr),
                             (*xw).widget_osr);
        }

      gtk_widget_show ((*xw).widget_osr);
      gtk_widget_show ((*xw).widgetwindow_osr);
      #[cfg (all (not (feature = "have_xinput2"), not (feature = "have_pgtk")))]
      synthesize_focus_in_event ((*xw).widgetwindow_osr);

      g_signal_connect! (G_OBJECT! (gtk_widget_get_window ((*xw).widgetwindow_osr)),
			 c"from-embedder".as_ptr (), G_CALLBACK! (from_embedder),
			 std::ptr::null_mut ());
      g_signal_connect! (G_OBJECT! (gtk_widget_get_window ((*xw).widgetwindow_osr)),
			 c"to-embedder".as_ptr (), G_CALLBACK! (to_embedder),
			 std::ptr::null_mut ());
      g_signal_connect! (G_OBJECT! (gtk_widget_get_window ((*xw).widgetwindow_osr)),
			 c"pick-embedded-child".as_ptr (), G_CALLBACK! (pick_embedded_child),
			 std::ptr::null_mut ());

      /* Store some xwidget data in the gtk widgets for convenient
         retrieval in the event handlers.  */
      g_object_set_data (G_OBJECT! ((*xw).widget_osr), XG_XWIDGET.as_ptr (), xw as gpointer);
      g_object_set_data (G_OBJECT! ((*xw).widgetwindow_osr), XG_XWIDGET.as_ptr (), xw as gpointer);

      /* signals */
      if EQ ((*xw).r#type, Qwebkit ())
        {
          g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
                             c"load-changed".as_ptr (),
                             G_CALLBACK! (webkit_view_load_changed_cb), xw);

          g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
                             c"decide-policy".as_ptr (),
                             G_CALLBACK! (webkit_decide_policy_cb),
                             xw);
	  #[cfg (feature = "have_pgtk")]
	  g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
			     c"mouse-target-changed".as_ptr (),
			     G_CALLBACK! (mouse_target_changed),
			     xw);
	  g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
			     c"create".as_ptr (),
			     G_CALLBACK! (webkit_create_cb),
			     xw);
	  g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
			     c"script-dialog".as_ptr (),
			     G_CALLBACK! (webkit_script_dialog_cb),
			     std::ptr::null_mut ());
	  g_signal_connect! (G_OBJECT! ((*xw).widget_osr),
			     c"run-file-chooser".as_ptr (),
			     G_CALLBACK! (run_file_chooser_cb),
			     std::ptr::null_mut ());
        }

      g_signal_connect! (G_OBJECT! ((*xw).widgetwindow_osr), c"damage-event".as_ptr (),
			 G_CALLBACK! (offscreen_damage_event), xw);

      unblock_input ();
    }
  }
  /* C 476's `#elif defined NS_IMPL_COCOA` arm.  C's `#elif` negates the file gate it belongs to,
     but the port keeps the file gate on every item (P-86 ⑧), so the arm carries both names - a
     recorded deviation (Deviations 4).  `include/nsxwidget.rs` is the Cocoa lane's mirror, so
     `nsxwidget_init` is declared here at C's use site (P-72 rule 4).  */
  /// C: nsxwidget.h 53: `void nsxwidget_init (struct xwidget *xw);`
  #[cfg (all (feature = "use_gtk", feature = "ns_impl_cocoa"))]
  unsafe extern "C" {
    pub fn nsxwidget_init (xw: *mut xwidget);
  }
  #[cfg (all (feature = "use_gtk", feature = "ns_impl_cocoa"))]
  nsxwidget_init (xw);

  return val;
});

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-live-p", Fxwidget_live_p, Sxwidget_live_p, 1, 1, 0,
       doc: "Return t if OBJECT is an xwidget that has not been killed.\nValue is nil if OBJECT is not an xwidget or if it has been killed.",
       (object: Lisp_Object),
       {
  return (if XWIDGETP (object)
	    && ! NILP ((*XXWIDGET (object)).buffer)
	  { Qt () } else { Qnil () });
});

/* C 493-502: `#ifdef USE_GTK` - the `forall` callback C 662 hands to `gtk_container_forall`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn set_widget_if_text_view (widget: *mut GtkWidget, data: *mut c_void)
{
  let pointer: *mut *mut GtkWidget = data as *mut *mut GtkWidget;

  if GTK_IS_TEXT_VIEW! (widget)
    { *pointer = widget; }
}

/* C 504-694: `Fxwidget_perform_lispy_event`.  The seven declarations C 514-522 writes inside its
   `#ifdef USE_GTK`/`#ifdef HAVE_XINPUT2` pair carry those gates here (a `let` may carry a
   `#[cfg]`, which is the `src/gtkutil.rs` 2106 shape for the same construction).  Two names this
   body calls are not in any mirror the file includes, so the `extern` declarations stand at their
   C use site (P-72 rule 4; both records are owed): `decode_window_system_frame` (frame.h 1475) and
   `parse_modifiers` (keyboard.h 461).  */
/// C: frame.h 1475: `extern struct frame *decode_window_system_frame (Lisp_Object);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}
/// C: keyboard.h 461: `extern Lisp_Object parse_modifiers (Lisp_Object);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn parse_modifiers (symbol: Lisp_Object) -> Lisp_Object;
}
/// C: keyboard.h 525: `extern const char *const lispy_function_keys[];` - C's incomplete array.
/// `src/keyboard.rs` 5975 *defines* the 256-slot X11 table (C 5513's arm, the one this TU's
/// configuration compiles), so the port declares those 256 slots rather than an incomplete array
/// the loop at C 592-602 could not index; the definition there carries no `#[unsafe(no_mangle)]`
/// yet, which a real GTK build needs (Owed 3).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub static mut lispy_function_keys: [*const c_char; 256];
}
/// C: xterm.h 1809: `extern int x_emacs_to_x_modifiers (struct x_display_info *, intmax_t);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
unsafe extern "C" {
  pub fn x_emacs_to_x_modifiers (dpyinfo: *mut x_display_info, modifiers: intmax_t) -> c_int;
}
/// C: pgtkterm.h 678: `extern int pgtk_emacs_to_gtk_modifiers (struct pgtk_display_info *, int);`
/// (`pgtk_display_info` itself is in `include/pgtkterm.rs`'s placeholder - the recorded mirror
/// gap, Deviations 6).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe extern "C" {
  pub fn pgtk_emacs_to_gtk_modifiers (dpyinfo: *mut pgtk_display_info, modifiers: c_int)
    -> c_int;
}

/// C: process.h 303: `extern void catch_child_signal (void);` - `include/process.rs` does not
/// carry it (P-72 rule 4), and `src/process.rs` 8661 defines it without `#[unsafe(no_mangle)]`
/// yet, which a real GTK build needs (Owed 3).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn catch_child_signal ();
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-perform-lispy-event",
       Fxwidget_perform_lispy_event, Sxwidget_perform_lispy_event, 2, 3, 0,
       doc: "Send a lispy event to XWIDGET.\nEVENT should be the event that will be sent.  FRAME should be the\nframe which generated the event, and defaults to the selected frame.\nOn X11, modifier keys will not be processed if FRAME is nil and the\nselected frame is not an X-Windows frame.",
       (xwidget: Lisp_Object, event: Lisp_Object, frame: Lisp_Object),
       {
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut f: *mut frame = std::ptr::null_mut ();
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut xg_event: *mut GdkEvent;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut klass: *mut GtkContainerClass;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut widget: *mut GtkWidget;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut temp: *mut GtkWidget = std::ptr::null_mut ();
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_xinput2"))]
  let mut embedder: *mut GdkWindow;
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_xinput2"))]
  let mut osw: *mut GdkWindow;

  CHECK_LIVE_XWIDGET (xwidget);

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  if ! NILP (frame)
    { f = decode_window_system_frame (frame); }
  else if FRAME_WINDOW_P (SELECTED_FRAME ())
    { f = SELECTED_FRAME (); }

  let mut character: c_int = -1;
  let mut keycode: c_int = -1;
  let mut modifiers: c_int = 0;
  let xw: *mut xwidget = XXWIDGET (xwidget);

  #[cfg (feature = "have_xinput2")]
  {
  /* XI2 GDK devices crash if we try this without an embedder set.  */
  if f.is_null ()
    { return Qnil (); }

  block_input ();
  osw = gtk_widget_get_window ((*xw).widgetwindow_osr);
  embedder = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! (f));

  gdk_offscreen_window_set_embedder (osw, embedder);
  unblock_input ();
  }
  widget = gtk_window_get_focus (GTK_WINDOW! ((*xw).widgetwindow_osr));

  if widget.is_null ()
    { widget = (*xw).widget_osr; }

  if RANGED_FIXNUMP (0, event, INT_MAX as EMACS_INT)
    {
      character = XFIXNUM (event) as c_int;

      if character < 32
	{ modifiers |= ctrl_modifier; }

      modifiers |= character & meta_modifier;
      modifiers |= character & hyper_modifier;
      modifiers |= character & super_modifier;
      modifiers |= character & shift_modifier;
      modifiers |= character & ctrl_modifier;

      character = character & ! (1 << 21);

      if character < 32
	{ character += '_' as c_int; }

      #[cfg (not (feature = "have_pgtk"))]
      if ! f.is_null ()
	{ modifiers = x_emacs_to_x_modifiers (FRAME_DISPLAY_INFO! (f), modifiers as intmax_t); }
      else
	{ modifiers = 0; }
      #[cfg (feature = "have_pgtk")]
      if ! f.is_null ()
	{ modifiers = pgtk_emacs_to_gtk_modifiers (FRAME_DISPLAY_INFO! (f), modifiers); }
      else
	{ modifiers = 0; }
    }
  else if SYMBOLP (event)
    {
      let decoded: Lisp_Object = parse_modifiers (event);
      let decoded_name: Lisp_Object = SYMBOL_NAME (XCAR (decoded));

      let mut off: c_int = 0;
      let mut found: bool = false;

      while off < 256
	{
	  if ! lispy_function_keys[off as usize].is_null ()
	      && strcmp (lispy_function_keys[off as usize],
			 SSDATA (decoded_name) as *const c_char) == 0
	    {
	      found = true;
	      break;
	    }
	  off += 1;
	}

      #[cfg (not (feature = "have_pgtk"))]
      if ! f.is_null ()
	{ modifiers = x_emacs_to_x_modifiers (FRAME_DISPLAY_INFO! (f),
					      XFIXNUM (XCAR (XCDR (decoded))) as intmax_t); }
      else
	{ modifiers = 0; }
      #[cfg (feature = "have_pgtk")]
      if ! f.is_null ()
	{ modifiers = pgtk_emacs_to_gtk_modifiers (FRAME_DISPLAY_INFO! (f),
						   XFIXNUM (XCAR (XCDR (decoded))) as c_int); }
      else
	{ modifiers = 0; }

      if found
	{ keycode = off + 0xff00; }
    }

  if character == -1 && keycode == -1
    {
      #[cfg (feature = "have_xinput2")]
      {
      block_input ();
      if ! (*xw).embedder_view.is_null ()
	{ record_osr_embedder ((*xw).embedder_view as *mut xwidget_view); }
      else
	{ gdk_offscreen_window_set_embedder (osw, std::ptr::null_mut ()); }
      unblock_input ();
      }
      return Qnil ();
    }

  block_input ();
  xg_event = gdk_event_new (GDK_KEY_PRESS);
  (*xg_event).any.window = gtk_widget_get_window ((*xw).widget_osr);
  g_object_ref ((*xg_event).any.window as gpointer);

  if character > -1
    { keycode = gdk_unicode_to_keyval (character as guint32) as c_int; }

  (*xg_event).key.keyval = keycode as c_uint;
  #[cfg (not (feature = "have_x_windows"))]
  { (*xg_event).key.state = modifiers as c_uint; }
  #[cfg (feature = "have_x_windows")]
  if ! f.is_null ()
    { (*xg_event).key.state = xw_translate_x_modifiers (FRAME_DISPLAY_INFO! (f),
							modifiers as c_uint); }

  if keycode > -1
    {
      /* WebKitGTK internals abuse follows.  */
      if WEBKIT_IS_WEB_VIEW! (widget)
	{
	  /* WebKitGTK relies on an internal GtkTextView object to
	     "translate" keys such as backspace.  We must find that
	     widget and activate its binding to this key if any.  */
	  klass = GTK_CONTAINER_CLASS! (G_OBJECT_GET_CLASS! (widget));

	  (*klass).forall.unwrap () (GTK_CONTAINER! ((*xw).widget_osr), TRUE,
				     Some (set_widget_if_text_view), &raw mut temp as gpointer);

	  if GTK_IS_WIDGET! (temp)
	    {
	      if gtk_widget_get_realized (temp) == 0
		{ gtk_widget_realize (temp); }

	      gtk_bindings_activate (G_OBJECT! (temp), keycode as guint,
				     modifiers as GdkModifierType);
	    }
	}
    }

  if ! f.is_null ()
    { gdk_event_set_device (xg_event,
			    find_suitable_keyboard (SELECTED_FRAME ())); }

  gtk_main_do_event (xg_event);
  (*xg_event).r#type = GDK_KEY_RELEASE;
  gtk_main_do_event (xg_event);
  gdk_event_free (xg_event);

  #[cfg (feature = "have_xinput2")]
  {
  if ! (*xw).embedder_view.is_null ()
    { record_osr_embedder ((*xw).embedder_view as *mut xwidget_view); }
  else
    { gdk_offscreen_window_set_embedder (osw, std::ptr::null_mut ()); }
  }
  unblock_input ();
  }

  return Qnil ();
});

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("get-buffer-xwidgets", Fget_buffer_xwidgets, Sget_buffer_xwidgets, 1, 1, 0,
       doc: "Return a list of xwidgets associated with BUFFER.\nBUFFER may be a buffer or the name of one.",
       (buffer: Lisp_Object),
       {
  let mut xw: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut xw_list: Lisp_Object;
  /* C reassigns its `buffer` parameter (C 706); Rust needs the rebinding (the
     `src/androidmenu.rs` `contents` shape).  */
  let mut buffer = buffer;

  if NILP (buffer)
    { return Qnil (); }
  buffer = Fget_buffer (buffer);
  if NILP (buffer)
    { return Qnil (); }

  xw_list = Qnil ();

  tail = internal_xwidget_list;
  while CONSP (tail)
    {
      xw = XCAR (tail);
      if XWIDGETP (xw) && EQ (Fxwidget_buffer (xw), buffer)
        { xw_list = Fcons (xw, xw_list); }
      tail = XCDR (tail);
    }
  return xw_list;
});

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_hidden (xv: *mut xwidget_view) -> bool                 // C 721-725
{
  return (*xv).hidden;
}

/* C 727-737: `xwidget_from_id` is one of the file's non-`static` definitions (xwidget.h 206
   declares it `extern`), so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2); `include/xwidget.rs`
   declares the same name in `xwidget_decls` and its glob import is shadowed by this definition
   (P-17/P-21, the note at that mirror's C 184-230).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_from_id (id: u32) -> *mut xwidget
{
  let key: Lisp_Object = make_fixnum (id as EMACS_INT);
  let xwidget: Lisp_Object = Fgethash (key, id_to_xwidget_map, Qnil ());

  if NILP (xwidget)
    { emacs_abort (); }

  return XXWIDGET (xwidget);
}

/* C 739-2281: the file's widest `#ifdef USE_GTK` - everything from here to the `?` file's GTK
   body.  This slice carries C 739-1180 of it; the next slice opens at C 1182's
   `#ifdef HAVE_X_WINDOWS`.  */

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn pick_embedded_child (window: *mut GdkWindow, x: c_double, y: c_double,
                                          user_data: gpointer) -> *mut GdkWindow  // C 740-764
{
  let mut widget: *mut GtkWidget;
  let mut child: *mut GtkWidget;
  /* C declares `GdkEvent event;` and lets the assignments below initialise it - Rust does not
     support partial initialisation (PORTING.md 0.1-13), so the port zeroes it first.  */
  let mut event: GdkEvent = unsafe { std::mem::zeroed () };
  let mut xout: c_int = 0;
  let mut yout: c_int = 0;

  event.any.window = window;
  event.any.r#type = GDK_NOTHING;

  widget = gtk_get_event_widget (&raw mut event);

  if widget.is_null ()
    { return std::ptr::null_mut (); }

  child = find_widget_at_pos (widget, lrint (x) as c_int, lrint (y) as c_int,
                              &raw mut xout, &raw mut yout, false, std::ptr::null_mut ());

  if child.is_null ()
    { return std::ptr::null_mut (); }

  return gtk_widget_get_window (child);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn record_osr_embedder (view: *mut xwidget_view)                  // C 766-782
{
  let mut xw: *mut xwidget;
  let mut window: *mut GdkWindow;
  let mut embedder: *mut GdkWindow;

  xw = XXWIDGET ((*view).model);
  window = gtk_widget_get_window ((*xw).widgetwindow_osr);
  #[cfg (not (feature = "have_pgtk"))]
  { embedder = gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET! ((*view).frame as *mut frame)); }
  #[cfg (feature = "have_pgtk")]
  { embedder = gtk_widget_get_window ((*view).widget); }
  gdk_offscreen_window_set_embedder (window, embedder);
  (*xw).embedder = (*view).frame;
  (*xw).embedder_view = view as *mut c_void;
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn find_xwidget_for_offscreen_window (window: *mut GdkWindow) -> *mut xwidget  // C 784-804
{
  let mut tem: Lisp_Object;
  let mut xw: *mut xwidget;
  let mut w: *mut GdkWindow;

  tem = internal_xwidget_list;
  while CONSP (tem)
    {
      if XWIDGETP (XCAR (tem))
	{
	  xw = XXWIDGET (XCAR (tem));
	  w = gtk_widget_get_window ((*xw).widgetwindow_osr);

	  if w == window
	    { return xw; }
	}
      tem = XCDR (tem);
    }

  return std::ptr::null_mut ();
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn from_embedder (window: *mut GdkWindow, x: c_double, y: c_double,
                                    x_out_ptr: gpointer, y_out_ptr: gpointer,
                                    user_data: gpointer)                  // C 806-841
{
  let xout: *mut c_double = x_out_ptr as *mut c_double;
  let yout: *mut c_double = y_out_ptr as *mut c_double;
  #[cfg (not (feature = "have_pgtk"))]
  {
  let xw: *mut xwidget = find_xwidget_for_offscreen_window (window);
  let mut xvw: *mut xwidget_view;
  let mut xoff: gint = 0;
  let mut yoff: gint = 0;

  if xw.is_null ()
    { emacs_abort (); }

  xvw = (*xw).embedder_view as *mut xwidget_view;

  if xvw.is_null ()
    {
      *xout = x;
      *yout = y;
    }
  else
    {
      gtk_widget_translate_coordinates (FRAME_GTK_WIDGET! ((*xvw).frame as *mut frame),
					FRAME_GTK_OUTER_WIDGET! ((*xvw).frame as *mut frame),
					0, 0, &raw mut xoff, &raw mut yoff);

      *xout = x - (*xvw).x as c_double - xoff as c_double;
      *yout = y - (*xvw).y as c_double - yoff as c_double;
    }
  }
  #[cfg (feature = "have_pgtk")]
  {
  *xout = x;
  *yout = y;
  }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn to_embedder (window: *mut GdkWindow, x: c_double, y: c_double,
                                  x_out_ptr: gpointer, y_out_ptr: gpointer,
                                  user_data: gpointer)                    // C 843-878
{
  let xout: *mut c_double = x_out_ptr as *mut c_double;
  let yout: *mut c_double = y_out_ptr as *mut c_double;
  #[cfg (not (feature = "have_pgtk"))]
  {
  let xw: *mut xwidget = find_xwidget_for_offscreen_window (window);
  let mut xvw: *mut xwidget_view;
  let mut xoff: gint = 0;
  let mut yoff: gint = 0;

  if xw.is_null ()
    { emacs_abort (); }

  xvw = (*xw).embedder_view as *mut xwidget_view;

  if xvw.is_null ()
    {
      *xout = x;
      *yout = y;
    }
  else
    {
      gtk_widget_translate_coordinates (FRAME_GTK_WIDGET! ((*xvw).frame as *mut frame),
					FRAME_GTK_OUTER_WIDGET! ((*xvw).frame as *mut frame),
					0, 0, &raw mut xoff, &raw mut yoff);

      *xout = x + (*xvw).x as c_double + xoff as c_double;
      *yout = y + (*xvw).y as c_double + yoff as c_double;
    }
  }
  #[cfg (feature = "have_pgtk")]
  {
  *xout = x;
  *yout = y;
  }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn find_suitable_pointer (f: *mut frame, need_smooth: bool) -> *mut GdkDevice  // C 880-909
{
  let seat: *mut GdkSeat = gdk_display_get_default_seat
    (gtk_widget_get_display (FRAME_GTK_WIDGET! (f)));
  let mut devices: *mut GList;
  let mut tem: *mut GList;
  let mut device: *mut GdkDevice;

  if seat.is_null ()
    { return std::ptr::null_mut (); }

  devices = gdk_seat_get_slaves (seat, GDK_SEAT_CAPABILITY_ALL_POINTING);
  device = std::ptr::null_mut ();
  tem = std::ptr::null_mut ();

  if need_smooth
    {
      tem = devices;
      while ! tem.is_null ()
	{
	  device = GDK_DEVICE! ((*tem).data);

	  if gdk_device_get_source (device) == GDK_SOURCE_TOUCHPAD
	    { break; }
	  tem = (*tem).next;
	}
    }

  g_list_free (devices);

  return if tem.is_null () { gdk_seat_get_pointer (seat) } else { device };
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn find_suitable_keyboard (f: *mut frame) -> *mut GdkDevice       // C 911-921
{
  let seat: *mut GdkSeat = gdk_display_get_default_seat
    (gtk_widget_get_display (FRAME_GTK_WIDGET! (f)));

  if seat.is_null ()
    { return std::ptr::null_mut (); }

  return gdk_seat_get_keyboard (seat);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn find_widget_cb (widget: *mut GtkWidget, user: *mut c_void)  // C 923-927
{
  find_widget (widget, user as *mut widget_search_data);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn find_widget (widget: *mut GtkWidget,
                       data: *mut widget_search_data)                    // C 929-1018
{
  /* C 933's `GtkAllocation new_allocation;` is filled by the call below (PORTING.md 0.1-13:
     Rust has no partial initialisation).  */
  let mut new_allocation: GtkAllocation = unsafe { std::mem::zeroed () };
  let mut window: *mut GdkWindow;
  let mut x_offset: c_int = 0;
  let mut y_offset: c_int = 0;

  gtk_widget_get_allocation (widget, &raw mut new_allocation);

  if gtk_widget_get_has_window (widget) != 0
    {
      new_allocation.x = 0;
      new_allocation.y = 0;
    }

  if ! gtk_widget_get_parent (widget).is_null () && ! (*data).first
    {
      window = gtk_widget_get_window (widget);
      while window != gtk_widget_get_window (gtk_widget_get_parent (widget))
	{
	  let mut tx: gint = 0;
	  let mut ty: gint = 0;
	  let mut twidth: gint;
	  let mut theight: gint;

	  if window.is_null ()
	    { return; }

	  twidth = gdk_window_get_width (window);
	  theight = gdk_window_get_height (window);

	  if new_allocation.x < 0
	    {
	      new_allocation.width += new_allocation.x;
	      new_allocation.x = 0;
	    }

	  if new_allocation.y < 0
	    {
	      new_allocation.height += new_allocation.y;
	      new_allocation.y = 0;
	    }

	  if new_allocation.x + new_allocation.width > twidth
	    { new_allocation.width = twidth - new_allocation.x; }
	  if new_allocation.y + new_allocation.height > theight
	    { new_allocation.height = theight - new_allocation.y; }

	  gdk_window_get_position (window, &raw mut tx, &raw mut ty);
	  new_allocation.x += tx;
	  x_offset += tx;
	  new_allocation.y += ty;
	  y_offset += ty;

	  window = gdk_window_get_parent (window);
	}
    }

  if ((*data).x >= new_allocation.x) && ((*data).y >= new_allocation.y)
      && ((*data).x < new_allocation.x + new_allocation.width)
      && ((*data).y < new_allocation.y + new_allocation.height)
    {
      /* First, check if the drag is in a valid drop site in one of
	 our children.	*/
      if GTK_IS_CONTAINER! (widget)
	{
	  let mut new_data: widget_search_data = *data;

	  new_data.x -= x_offset;
	  new_data.y -= y_offset;
	  new_data.foundp = false;
	  new_data.first = false;

	  gtk_container_forall (GTK_CONTAINER! (widget),
				Some (find_widget_cb), &raw mut new_data as gpointer);

	  (*data).foundp = new_data.foundp;
	  if (*data).foundp
	    { (*data).data = new_data.data; }
	}

      /* If not, and this widget is registered as a drop site, check
	 to emit "drag_motion" to check if we are actually in a drop
	 site.	*/
      if ! (*data).foundp
	{
	  (*data).foundp = true;
	  (*data).data = widget;
	}
    }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn find_widget_at_pos (w: *mut GtkWidget, x: c_int, y: c_int,
                              new_x: *mut c_int, new_y: *mut c_int,
                              pointer_grabs: bool,
                              vw: *mut xwidget_view) -> *mut GtkWidget   // C 1020-1062
{
  /* C's `struct widget_search_data data;` is initialised field by field below (PORTING.md
     0.1-13: Rust has no partial initialisation).  */
  let mut data: widget_search_data = widget_search_data {
    x: 0, y: 0, foundp: false, first: false, data: std::ptr::null_mut () };
  #[cfg (feature = "have_x_windows")]
  {
  let mut grab: *mut GtkWidget = std::ptr::null_mut ();

  if pointer_grabs
    {
      grab = (*vw).passive_grab;

      if ! grab.is_null () && ! gtk_widget_get_window (grab).is_null ()
	{
	  gtk_widget_translate_coordinates (w, grab, x,
					    y, new_x, new_y);

	  return grab;
	}
    }
  }

  data.x = x;
  data.y = y;
  data.foundp = false;
  data.first = true;

  find_widget (w, &raw mut data);

  if data.foundp
    {
      gtk_widget_translate_coordinates (w, data.data, x,
					y, new_x, new_y);
      return data.data;
    }

  *new_x = x;
  *new_y = y;

  return std::ptr::null_mut ();
}

/* C 1064-1120: `#ifdef HAVE_PGTK`.  `Emacs_Cursor` is `pgtkgui.rs`'s `*mut c_void` under
   `have_pgtk` (P-55's stand-in for GDK's `GdkCursor *`), which is why C 1106's argument needs the
   `as *mut GdkCursor` the explicit-cast deviation records.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe fn cursor_for_hit (result: guint, frame: *mut frame) -> Emacs_Cursor  // C 1065-1082
{
  let mut cursor: Emacs_Cursor = (*FRAME_OUTPUT_DATA! (frame) as *mut pgtk_output).nontext_cursor;

  if (result & WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE) != 0
      || (result & WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION) != 0
      || (result & WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT) != 0
    { cursor = (*FRAME_X_OUTPUT! (frame) as *mut pgtk_output).text_cursor; }

  if (result & WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR) != 0
    { cursor = (*FRAME_X_OUTPUT! (frame) as *mut pgtk_output).vertical_drag_cursor; }

  if (result & WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK) != 0
    { cursor = (*FRAME_X_OUTPUT! (frame) as *mut pgtk_output).hand_cursor; }

  return cursor;
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe fn define_cursors (xw: *mut xwidget, res: *mut WebKitHitTestResult)  // C 1084-1111
{
  let mut xvw: *mut xwidget_view;
  let mut wdesc: *mut GdkWindow;

  (*xw).hit_result = webkit_hit_test_result_get_context (res);

  let mut tem: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tem)
    {
      if XWIDGET_VIEW_P (XCAR (tem))
	{
	  xvw = XXWIDGET_VIEW (XCAR (tem));

	  if XXWIDGET ((*xvw).model) == xw
	    {
	      (*xvw).cursor = cursor_for_hit ((*xw).hit_result, (*xvw).frame as *mut frame);

	      if gtk_widget_get_realized ((*xvw).widget) != 0
		{
		  wdesc = gtk_widget_get_window ((*xvw).widget);
		  gdk_window_set_cursor (wdesc, (*xvw).cursor as *mut GdkCursor);
		}
	    }
	}
      tem = XCDR (tem);
    }
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe extern "C" fn mouse_target_changed (webview: *mut WebKitWebView,
                                           hitresult: *mut WebKitHitTestResult,
                                           modifiers: guint, xw: gpointer)  // C 1113-1119
{
  define_cursors (xw as *mut xwidget, hitresult);
}
/* C 1120: `#endif` (the `HAVE_PGTK` arm).  */

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn run_file_chooser_cb (webview: *mut WebKitWebView,
                                          request: *mut WebKitFileChooserRequest,
                                          user_data: gpointer) -> gboolean  // C 1122-1180
{
  let f: *mut frame = SELECTED_FRAME ();
  let mut chooser: *mut GtkFileChooserNative;
  let mut filter: *mut GtkFileFilter;
  let mut select_multiple_p: bool;
  let mut response: guint;
  let mut filenames: *mut GSList;
  let mut tem: *mut GSList;
  let mut i: c_int;
  let mut len: c_int;
  let mut files: *mut *mut gchar;

  /* Return TRUE to prevent WebKit from showing the default script
     dialog in the offscreen window, which runs a nested main loop
     Emacs can't respond to, and as such can't pass X events to.  */
  if ! FRAME_WINDOW_P (f)
    { return TRUE; }

  chooser = gtk_file_chooser_native_new (c"Select file".as_ptr (),
					 GTK_WINDOW! (FRAME_GTK_OUTER_WIDGET! (f)),
					 GTK_FILE_CHOOSER_ACTION_OPEN, c"Select".as_ptr (),
					 c"Cancel".as_ptr ());
  filter = webkit_file_chooser_request_get_mime_types_filter (request);
  select_multiple_p = webkit_file_chooser_request_get_select_multiple (request) != 0;

  gtk_file_chooser_set_select_multiple (GTK_FILE_CHOOSER! (chooser),
					select_multiple_p as gboolean);
  gtk_file_chooser_add_filter (GTK_FILE_CHOOSER! (chooser), filter);
  response = gtk_native_dialog_run (GTK_NATIVE_DIALOG! (chooser)) as guint;

  if response != GTK_RESPONSE_ACCEPT as guint
    {
      gtk_native_dialog_destroy (GTK_NATIVE_DIALOG! (chooser));
      webkit_file_chooser_request_cancel (request);

      return TRUE;
    }

  filenames = gtk_file_chooser_get_filenames (GTK_FILE_CHOOSER! (chooser));
  len = g_slist_length (filenames) as c_int;
  /* C's bare `alloca` is the port's stack pool (PORTING.md P-05; the `src/editfns.rs` 2477
     shape).  */
  files = sa_stack_alloc (std::mem::size_of::<*mut gchar> () * (len + 1) as usize)
    as *mut *mut gchar;

  tem = filenames;
  i = 0;
  while ! tem.is_null ()
    {
      *files.offset (i as isize) = (*tem).data as *mut gchar;
      tem = (*tem).next;
      i += 1;
    }
  *files.offset (len as isize) = std::ptr::null_mut ();

  g_slist_free (filenames);
  webkit_file_chooser_request_select_files (request, files as *const *const gchar);

  i = 0;
  while i < len
    {
      g_free (*files.offset (i as isize) as gpointer);
      i += 1;
    }

  gtk_native_dialog_destroy (GTK_NATIVE_DIALOG! (chooser));

  return TRUE;
}
/* C 1181 is the blank line the next slice's `#ifdef HAVE_X_WINDOWS` (C 1182) follows.  */

/* ----------------------------------------------------------------------------------------------
   C 1182-2716's newly-read toolkit surface: the `HAVE_X_WINDOWS` drag/button/motion/crossing
   machinery, the WebKit callbacks and the script dialog.  Same treatment as the surface at the
   top of this file (P-88 ①, which P-90 ① keeps for this round): the names stand here until the
   P-90 ③ move into `femacs/src/include/gtk.rs`, each citation naming its header and the C line
   that reads it.  **GDK, GTK, WebKit and XInput2 have no tree on this machine** (the file note
   above), so every record below carries exactly the members C reads, its layout is
   **UNVERIFIED**, and every enumerator is a *distinct placeholder* whose only job is to keep C's
   arms distinct; the real values are owed (`## Owed` 2).  **X11 is on this machine**, so the
   `XMoveWindow` family's prototypes below are the header's own.
   ---------------------------------------------------------------------------------------------- */

/* GDK, continued.  */
/// C: gdk/gdktypes.h: `typedef enum { GDK_NOTIFY_ANCESTOR, GDK_NOTIFY_VIRTUAL,
/// GDK_NOTIFY_INFERIOR, GDK_NOTIFY_NONLINEAR, GDK_NOTIFY_NONLINEAR_VIRTUAL } GdkNotifyType;` -
/// `xi_translate_notify_detail`'s return type (C 1557) and the type of the `detail` C 1855-1861
/// and C 1885-1891 write.  **UNVERIFIED**: the five names are distinct placeholders.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkNotifyType = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_NOTIFY_ANCESTOR: GdkNotifyType = 0;           // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_NOTIFY_VIRTUAL: GdkNotifyType = 1;            // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_NOTIFY_INFERIOR: GdkNotifyType = 2;           // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_NOTIFY_NONLINEAR: GdkNotifyType = 3;          // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_NOTIFY_NONLINEAR_VIRTUAL: GdkNotifyType = 4;  // placeholder, not GDK's value
/// C: gdk/gdktypes.h: `typedef enum { GDK_CROSSING_NORMAL, GDK_CROSSING_SELECTION,
/// GDK_CROSSING_GTK_GRAB, GDK_CROSSING_GTK_UNGRAB, GDK_CROSSING_STATE_CHANGED } GdkCrossingMode;`
/// - the four names C 1344/2056/2068/2096 pass as `entry_crossing`/`exit_crossing`.  The type
/// itself is declared at C 244-263's surface above.  **UNVERIFIED**: distinct placeholders.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_CROSSING_NORMAL: GdkCrossingMode = 0;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_CROSSING_GTK_GRAB: GdkCrossingMode = 1;       // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_CROSSING_GTK_UNGRAB: GdkCrossingMode = 2;     // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_CROSSING_STATE_CHANGED: GdkCrossingMode = 3;  // placeholder, not GDK's value
/// C: gdk/gdktypes.h: `GDK_BUTTON1_MASK = 1 << 8` (GLib's `GdkModifierType`) - the three C 2038-
/// 2046 ORs into `xg_event->crossing.state`.  **UNVERIFIED**: distinct placeholder bits.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_BUTTON1_MASK: GdkModifierType = 1 << 8;       // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_BUTTON2_MASK: GdkModifierType = 1 << 9;       // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_BUTTON3_MASK: GdkModifierType = 1 << 10;      // placeholder, not GDK's value
/// C: gdk/gdktypes.h: `typedef enum { GDK_SCROLL_UP, GDK_SCROLL_DOWN, GDK_SCROLL_LEFT,
/// GDK_SCROLL_RIGHT, GDK_SCROLL_SMOOTH } GdkScrollDirection;` - C 1370-1377's four writes and
/// C 1463's.  **UNVERIFIED**: distinct placeholders.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkScrollDirection = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL_UP: GdkScrollDirection = 0;            // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL_DOWN: GdkScrollDirection = 1;          // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL_LEFT: GdkScrollDirection = 2;          // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL_RIGHT: GdkScrollDirection = 3;         // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_SCROLL_SMOOTH: GdkScrollDirection = 4;        // placeholder, not GDK's value
/// C: gdk/gdktypes.h: `typedef enum { GDK_TOUCHPAD_GESTURE_PHASE_BEGIN,
/// GDK_TOUCHPAD_GESTURE_PHASE_UPDATE, GDK_TOUCHPAD_GESTURE_PHASE_END,
/// GDK_TOUCHPAD_GESTURE_PHASE_CANCELED } GdkTouchpadGesturePhase;` - C 1537-1547's three writes.
/// **UNVERIFIED**: distinct placeholders.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GdkTouchpadGesturePhase = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_TOUCHPAD_GESTURE_PHASE_BEGIN: GdkTouchpadGesturePhase = 0;   // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_TOUCHPAD_GESTURE_PHASE_UPDATE: GdkTouchpadGesturePhase = 1;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_TOUCHPAD_GESTURE_PHASE_END: GdkTouchpadGesturePhase = 2;     // placeholder
/// C: gdk/gdktypes.h: `GDK_FOCUS_CHANGE` / `GDK_TOUCHPAD_PINCH` - the two `GdkEventType` members
/// C 2121 and C 1521 ask `gdk_event_new` for.  Distinct placeholders (the values above C's list
/// at C 278-296 stops at 10).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_FOCUS_CHANGE: GdkEventType = 11;              // placeholder, not GDK's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GDK_TOUCHPAD_PINCH: GdkEventType = 12;            // placeholder, not GDK's value
/// C: glib/gmacros.h: `#define G_MAXINT32 2147483647` - C 2483's bounds test on `dlen`.  The
/// value is `INT32_MAX`, the header's own expression.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const G_MAXINT32: c_int = 2147483647;
/// C: glib/gmacros.h: `#define G_MAXUINT ((guint) 0xffffffff)` - C 3602's fourth argument to
/// `webkit_find_controller_search`.  The header's own expression.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const G_MAXUINT: guint = 4294967295;
/// C: glib/gasyncresult.h: `typedef struct _GAsyncResult GAsyncResult;` - `webkit_web_view_
/// run_javascript_finish`'s second parameter (C 2533).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GAsyncResult
{
  _private: [u8; 0],
}
/// C: glib/gasyncresult.h: `typedef void (*GAsyncReadyCallback) (GObject *source_object,
/// GAsyncResult *res, gpointer user_data);` - C 3244's `webkit_javascript_finished_cb` argument
/// (slice 4); the `include/gtk.rs` 93-96 `GCallback`/`GClosureNotify` shape.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GAsyncReadyCallback =
  Option<unsafe extern "C" fn (source_object: *mut GObject, res: *mut GAsyncResult,
			       user_data: gpointer)>;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 1585: `children = g_list_prepend (children, window);`
  /// C: glib/glist.h: `GList *g_list_prepend (GList *list, gpointer data);`
  pub fn g_list_prepend (list: *mut GList, data: gpointer) -> *mut GList;
  /// C 1092/1603/1934: `g_clear_pointer (&view->last_crossing_window, g_object_unref);`
  /// C: glib/gmem.h 135-145's `g_clear_pointer` macro - modelled as a `macro_rules!` (below),
  /// because C's name is a macro and calling it as a function would name a symbol GLib does not
  /// export (the mirror's `g_clear_object` precedent, `include/gtk.rs` 813).
  /// C 1585/1591/1654/1686/1698/1730: `gdk_window_get_parent (…);`
  /// C: gdk/gdkwindow.h: `GdkWindow *gdk_window_get_parent (GdkWindow *window);` (already
  /// declared above at C 982's surface).
  /// C 1795/1803: `gdk_window_is_destroyed (…);`
  /// C: gdk/gdkwindow.h: `gboolean gdk_window_is_destroyed (GdkWindow *window);`
  pub fn gdk_window_is_destroyed (window: *mut GdkWindow) -> gboolean;
  /// C 1589: `gdk_window_coords_from_parent (l->data, x, y, &x_out, &y_out);`
  /// C: gdk/gdkwindow.h: `void gdk_window_coords_from_parent (GdkWindow *window, gdouble
  /// parent_x, gdouble parent_y, gdouble *x, gdouble *y);`
  pub fn gdk_window_coords_from_parent (window: *mut GdkWindow, parent_x: c_double,
                                        parent_y: c_double, x: *mut c_double, y: *mut c_double);
  /// C 1743: `xg_cursor = gdk_window_get_cursor (crossing_window);`
  /// C: gdk/gdkwindow.h: `GdkCursor *gdk_window_get_cursor (GdkWindow *window);`
  pub fn gdk_window_get_cursor (window: *mut GdkWindow) -> *mut GdkCursor;
  /// C 2499: `memory_full (n);` - lisp.h's `extern AVOID memory_full (size_t);` (alloc.c).  No
  /// mirror carries it (`include/lisp.rs` has only `memory_full_up`, used at C 2483), so the
  /// prototype stands here (P-72 rule 4) and is owed (`## Owed` 3).
  pub fn memory_full (size: usize) -> !;
  /// C 2478: `g_strfreev (properties_names);`
  /// C: glib/gstrfuncs.h: `void g_strfreev (gchar **str_array);`
  pub fn g_strfreev (str_array: *mut *mut gchar);
  /// C 2477: `guint n = g_strv_length (properties_names);`
  /// C: glib/gstrfuncs.h: `guint g_strv_length (gchar **str_array);`
  pub fn g_strv_length (str_array: *mut *mut gchar) -> guint;
}
/// C: glib/gmem.h 135-145: `#define g_clear_pointer(pp, destroy)` - take the pointer, null the
/// slot, then `destroy` the old value when it was non-NULL (the `__GNUC__` arm this machine
/// takes; the mirror's `g_clear_object` is the same body, `include/gtk.rs` 813).  C 1934's
/// `destroy` is `g_object_unref`.
/* The two `gdk/gdkx.h` entry points C 1747-1749 calls: X11-specific (they name `Cursor` and
   `Display`, which this tree carries only under `have_x_windows`), so their declarations carry
   that arm's gate.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" {
  /// C 1747: `cursor = gdk_x11_cursor_get_xcursor (xg_cursor);`
  /// C: gdk/gdkx.h: `Cursor gdk_x11_cursor_get_xcursor (GdkCursor *cursor);`
  pub fn gdk_x11_cursor_get_xcursor (cursor: *mut GdkCursor) -> Cursor;
  /// C 1749: `if (gdk_x11_cursor_get_xdisplay (xg_cursor) == xv->dpy)`
  /// C: gdk/gdkx.h: `Display *gdk_x11_cursor_get_xdisplay (GdkCursor *cursor);`
  pub fn gdk_x11_cursor_get_xdisplay (cursor: *mut GdkCursor) -> *mut Display;
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
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

/* GTK, continued.  */
/// C: gtk/gtkwindow.h: `typedef struct _GtkOffscreenWindow GtkOffscreenWindow;` (a `GtkBin`;
/// `GTK_OFFSCREEN_WINDOW` casts to it) - C 2185/2226 and C 2230.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkOffscreenWindow
{
  _private: [u8; 0],
}
/// C: gtk/gtkoffscreenwindow.h: `#define GTK_OFFSCREEN_WINDOW(obj) (G_TYPE_CHECK_INSTANCE_CAST
/// ((obj), GTK_TYPE_OFFSCREEN_WINDOW, GtkOffscreenWindow))` - the `GTK_WIDGET` treatment
/// (C 429): the port keeps the macro's name and its result type.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_OFFSCREEN_WINDOW {
  ($o:expr) => { ($o) as *mut GtkOffscreenWindow };
}
/// C: gtk/gtkentry.h: `typedef struct _GtkEntry GtkEntry;` (a `GtkWidget`; `GTK_ENTRY` casts to
/// it) - C 2704-2713's entry.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkEntry
{
  _private: [u8; 0],
}
/// C: gtk/gtkentry.h: `#define GTK_ENTRY(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ENTRY,
/// GtkEntry))` - the `GTK_WIDGET` treatment.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_ENTRY {
  ($o:expr) => { ($o) as *mut GtkEntry };
}
/// C: gtk/gtkdialog.h: `typedef struct _GtkDialog GtkDialog;` (a `GtkWindow`; `GTK_DIALOG` casts
/// to it) - C 2686/2687/2698/2699/2713.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct GtkDialog
{
  _private: [u8; 0],
}
/// C: gtk/gtkdialog.h: `#define GTK_DIALOG(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_DIALOG,
/// GtkDialog))` - the `GTK_WIDGET` treatment.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_DIALOG {
  ($o:expr) => { ($o) as *mut GtkDialog };
}
/// C: gtk/gtkenums.h: `GTK_DIALOG_MODAL = 1 << 0` (one of `GtkDialogFlags`) - C 2687/2699's
/// flags argument.  **UNVERIFIED**: the value is the header's, the flag's task being to differ
/// from `GTK_DIALOG_DESTROY_WITH_PARENT`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_DIALOG_MODAL: c_int = 1 << 0;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 2186/2227: `wnd = GTK_OFFSCREEN_WINDOW (w->widgetwindow_osr);`
  /// C: gtk/gtkoffscreenwindow.h: `cairo_surface_t *gtk_offscreen_window_get_surface
  /// (GtkOffscreenWindow *offscreen_window);`
  pub fn gtk_offscreen_window_get_surface (offscreen_window: *mut GtkOffscreenWindow)
    -> *mut cairo_surface_t;
  /// C 1226/1231: `g_signal_connect (G_OBJECT (view->passive_grab), "destroy", G_CALLBACK
  /// (gtk_widget_destroyed), &view->passive_grab);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_destroyed (GtkWidget *widget, GtkWidget **widget_pointer);`
  pub fn gtk_widget_destroyed (widget: *mut GtkWidget, widget_pointer: *mut *mut GtkWidget);
  /// C 2268: `gtk_widget_queue_draw (view->widget);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_queue_draw (GtkWidget *widget);`
  pub fn gtk_widget_queue_draw (widget: *mut GtkWidget);
  /// C 2686: `dialog = gtk_dialog_new_with_buttons ("Alert", GTK_WINDOW (widget), GTK_DIALOG_MODAL,
  /// "Dismiss", 1, NULL);`
  /// C: gtk/gtkdialog.h: `GtkWidget *gtk_dialog_new_with_buttons (const gchar *title, GtkWindow
  /// *parent, GtkDialogFlags flags, const gchar *first_button_text, ...);`
  pub fn gtk_dialog_new_with_buttons (title: *const gchar, parent: *mut GtkWindow, flags: c_int,
                                      first_button_text: *const gchar, ...) -> *mut GtkWidget;
  /// C 2693: `content_area = gtk_dialog_get_content_area (GTK_DIALOG (dialog));`
  /// C: gtk/gtkdialog.h: `GtkWidget *gtk_dialog_get_content_area (GtkDialog *dialog);`
  pub fn gtk_dialog_get_content_area (dialog: *mut GtkDialog) -> *mut GtkWidget;
  /// C 2704: `result = gtk_dialog_run (GTK_DIALOG (dialog));`
  /// C: gtk/gtkdialog.h: `gint gtk_dialog_run (GtkDialog *dialog);`
  pub fn gtk_dialog_run (dialog: *mut GtkDialog) -> gint;
  /// C 2705: `entry = gtk_entry_new ();`
  /// C: gtk/gtkentry.h: `GtkWidget *gtk_entry_new (void);`
  pub fn gtk_entry_new () -> *mut GtkWidget;
  /// C 2708: `gtk_entry_set_text (GTK_ENTRY (entry), message);`
  /// C: gtk/gtkentry.h: `void gtk_entry_set_text (GtkEntry *entry, const gchar *text);`
  pub fn gtk_entry_set_text (entry: *mut GtkEntry, text: *const gchar);
  /// C 2711-2712: `webkit_script_dialog_prompt_set_text (script_dialog, gtk_entry_get_text
  /// (GTK_ENTRY (entry)));`
  /// C: gtk/gtkentry.h: `const gchar *gtk_entry_get_text (GtkEntry *entry);`
  pub fn gtk_entry_get_text (entry: *mut GtkEntry) -> *const gchar;
  /// C 2702: `gtk_box_pack_end (GTK_BOX (box), entry, TRUE, TRUE, 0);`
  /// C: gtk/gtkbox.h: `void gtk_box_pack_end (GtkBox *box, GtkWidget *child, gboolean expand,
  /// gboolean fill, guint padding);`
  pub fn gtk_box_pack_end (box_: *mut GtkBox, child: *mut GtkWidget, expand: gboolean,
                           fill: gboolean, padding: guint);
}

/* WebKit, continued.  */
/// C: webkit2/WebKitJavascriptResult.h: `typedef struct _WebKitJavascriptResult
/// WebKitJavascriptResult;` - C 2530/2537/2552's value.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitJavascriptResult
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitURIRequest.h: `typedef struct _WebKitURIRequest WebKitURIRequest;` - C 2565
/// and C 2601/2608/2637's request.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitURIRequest
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitNavigationPolicyDecision.h: `typedef struct _WebKitNavigationPolicyDecision
/// WebKitNavigationPolicyDecision;` (a `WebKitPolicyDecision`) - C 2586 and C 2624.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitNavigationPolicyDecision
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitResponsePolicyDecision.h: `typedef struct _WebKitResponsePolicyDecision
/// WebKitResponsePolicyDecision;` (a `WebKitPolicyDecision`) - C 2582.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct WebKitResponsePolicyDecision
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitNavigationAction.h: `#define WEBKIT_NAVIGATION_POLICY_DECISION(obj)
/// (G_TYPE_CHECK_INSTANCE_CAST ((obj), WEBKIT_TYPE_NAVIGATION_POLICY_DECISION,
/// WebKitNavigationPolicyDecision))` - the `WEBKIT_WEB_VIEW` treatment (C 637): the port keeps
/// the macro's name and its result type.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! WEBKIT_NAVIGATION_POLICY_DECISION {
  ($o:expr) => { ($o) as *mut WebKitNavigationPolicyDecision };
}
/// C: webkit2/WebKitResponsePolicyDecision.h: `#define WEBKIT_RESPONSE_POLICY_DECISION(obj)
/// (G_TYPE_CHECK_INSTANCE_CAST ((obj), WEBKIT_TYPE_RESPONSE_POLICY_DECISION,
/// WebKitResponsePolicyDecision))` - the `WEBKIT_WEB_VIEW` treatment.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! WEBKIT_RESPONSE_POLICY_DECISION {
  ($o:expr) => { ($o) as *mut WebKitResponsePolicyDecision };
}
/// C: webkit2/WebKitNavigationAction.h: `typedef enum { WEBKIT_NAVIGATION_TYPE_LINK_CLICKED,
/// WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED, WEBKIT_NAVIGATION_TYPE_BACK_FORWARD,
/// WEBKIT_NAVIGATION_TYPE_RELOAD, WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED,
/// WEBKIT_NAVIGATION_TYPE_OTHER } WebKitNavigationType;` - C 2415-2422's five cases.  The six
/// names are distinct placeholders.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitNavigationType = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_LINK_CLICKED: WebKitNavigationType = 0;      // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED: WebKitNavigationType = 1;    // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_BACK_FORWARD: WebKitNavigationType = 2;      // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_RELOAD: WebKitNavigationType = 3;            // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED: WebKitNavigationType = 4;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_NAVIGATION_TYPE_OTHER: WebKitNavigationType = 5;             // placeholder
/// C: webkit2/WebKitScriptDialog.h: `typedef enum { WEBKIT_SCRIPT_DIALOG_ALERT,
/// WEBKIT_SCRIPT_DIALOG_CONFIRM, WEBKIT_SCRIPT_DIALOG_PROMPT,
/// WEBKIT_SCRIPT_DIALOG_BEFORE_UNLOAD_CONFIRM } WebKitScriptDialogType;` - C 2683/2701/2709's
/// four tests.  Distinct placeholders.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type WebKitScriptDialogType = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_SCRIPT_DIALOG_ALERT: WebKitScriptDialogType = 0;                   // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_SCRIPT_DIALOG_CONFIRM: WebKitScriptDialogType = 1;                 // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_SCRIPT_DIALOG_PROMPT: WebKitScriptDialogType = 2;                  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_SCRIPT_DIALOG_BEFORE_UNLOAD_CONFIRM: WebKitScriptDialogType = 3;   // placeholder
/// C: webkit2/WebKitWebView.h: the four `WebKitLoadEvent` members C 2436-2449 switches over
/// (the type itself is declared at C 627's surface above).  Distinct placeholders.  **UNVERIFIED**.
/// C: webkit2/WebKitFindController.h: the four `WebKitFindOptions` members C 3586-3594 OR's
/// together (the type itself is declared at C 627's surface above).  Distinct placeholders, and
/// the bit *positions* are what C's `|=` needs, so they keep C's shifts.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_FIND_OPTIONS_NONE: WebKitFindOptions = 0;          // C's `0`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE: WebKitFindOptions = 1 << 0;  // C's `1 << 0`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_FIND_OPTIONS_BACKWARDS: WebKitFindOptions = 1 << 3;         // C's `1 << 3`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_FIND_OPTIONS_WRAP_AROUND: WebKitFindOptions = 1 << 4;       // C's `1 << 4`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_LOAD_STARTED: WebKitLoadEvent = 0;      // placeholder, not WebKit's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_LOAD_REDIRECTED: WebKitLoadEvent = 1;   // placeholder, not WebKit's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_LOAD_COMMITTED: WebKitLoadEvent = 2;    // placeholder, not WebKit's value
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_LOAD_FINISHED: WebKitLoadEvent = 3;     // placeholder, not WebKit's value
/// C: webkit2/WebKitPolicyDecision.h: the three `WebKitPolicyDecisionType` members C 2579-2621
/// switches over (the type itself is declared at C 622's surface above).  Distinct placeholders.
/// **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION: WebKitPolicyDecisionType = 0;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION: WebKitPolicyDecisionType = 1;  // placeholder
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const WEBKIT_POLICY_DECISION_TYPE_RESPONSE: WebKitPolicyDecisionType = 2;           // placeholder
/// C: javascriptcore/JSValue.h: `typedef struct _JSCValue JSCValue;` - `webkit_js_to_lisp`'s
/// parameter (C 2454) and C 2544's value.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr (C)]
pub struct JSCValue
{
  _private: [u8; 0],
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 2531-2535: `webkit_web_view_run_javascript_finish (WEBKIT_WEB_VIEW (webview), result,
  /// &error);`
  /// C: webkit2/WebKitWebView.h: `WebKitJavascriptResult *webkit_web_view_run_javascript_finish
  /// (WebKitWebView *web_view, GAsyncResult *result, GError **error);`
  pub fn webkit_web_view_run_javascript_finish (web_view: *mut WebKitWebView,
                                                result: *mut GAsyncResult,
                                                error: *mut *mut GError)
    -> *mut WebKitJavascriptResult;
  /// C 2544: `JSCValue *value = webkit_javascript_result_get_js_value (js_result);`
  /// C: webkit2/WebKitJavascriptResult.h: `JSCValue *webkit_javascript_result_get_js_value
  /// (WebKitJavascriptResult *js_result);`
  pub fn webkit_javascript_result_get_js_value (js_result: *mut WebKitJavascriptResult)
    -> *mut JSCValue;
  /// C 2552: `webkit_javascript_result_unref (js_result);`
  /// C: webkit2/WebKitJavascriptResult.h: `void webkit_javascript_result_unref
  /// (WebKitJavascriptResult *js_result);`
  pub fn webkit_javascript_result_unref (js_result: *mut WebKitJavascriptResult);
  /// C 2564: `WebKitWebView *view = webkit_download_get_web_view (arg1);`
  /// C: webkit2/WebKitDownload.h: `WebKitWebView *webkit_download_get_web_view (WebKitDownload
  /// *download);`
  pub fn webkit_download_get_web_view (download: *mut WebKitDownload) -> *mut WebKitWebView;
  /// C 2565: `WebKitURIRequest *request = webkit_download_get_request (arg1);`
  /// C: webkit2/WebKitDownload.h: `WebKitURIRequest *webkit_download_get_request (WebKitDownload
  /// *download);`
  pub fn webkit_download_get_request (download: *mut WebKitDownload) -> *mut WebKitURIRequest;
  /// C 2571: `webkit_uri_request_get_uri (request)` - and C 2637-2638's.
  /// C: webkit2/WebKitURIRequest.h: `const gchar *webkit_uri_request_get_uri (WebKitURIRequest
  /// *request);`
  pub fn webkit_uri_request_get_uri (request: *mut WebKitURIRequest) -> *const gchar;
  /// C 2584-2585: `if (!webkit_response_policy_decision_is_mime_type_supported (response))`
  /// C: webkit2/WebKitResponsePolicyDecision.h: `gboolean
  /// webkit_response_policy_decision_is_mime_type_supported (WebKitResponsePolicyDecision
  /// *decision);`
  pub fn webkit_response_policy_decision_is_mime_type_supported
    (decision: *mut WebKitResponsePolicyDecision) -> gboolean;
  /// C 2587: `webkit_policy_decision_download (decision);`
  /// C: webkit2/WebKitPolicyDecision.h: `void webkit_policy_decision_download (WebKitPolicyDecision
  /// *decision);`
  pub fn webkit_policy_decision_download (decision: *mut WebKitPolicyDecision);
  /// C 2590-2591: `webkit_navigation_policy_decision_get_navigation_action (navigation_decision);`
  /// C: webkit2/WebKitNavigationPolicyDecision.h: `WebKitNavigationAction *
  /// webkit_navigation_policy_decision_get_navigation_action (WebKitNavigationPolicyDecision
  /// *decision);`
  pub fn webkit_navigation_policy_decision_get_navigation_action
    (decision: *mut WebKitNavigationPolicyDecision) -> *mut WebKitNavigationAction;
  /// C 2415: `switch (webkit_navigation_action_get_navigation_type (nav_action))`
  /// C: webkit2/WebKitNavigationAction.h: `WebKitNavigationType
  /// webkit_navigation_action_get_navigation_type (WebKitNavigationAction *navigation_action);`
  pub fn webkit_navigation_action_get_navigation_type (navigation_action: *mut WebKitNavigationAction)
    -> WebKitNavigationType;
  /// C 2602: `WebKitURIRequest *request = webkit_navigation_action_get_request (navigation_action);`
  /// C: webkit2/WebKitNavigationAction.h: `WebKitURIRequest *webkit_navigation_action_get_request
  /// (WebKitNavigationAction *navigation_action);`
  pub fn webkit_navigation_action_get_request (navigation_action: *mut WebKitNavigationAction)
    -> *mut WebKitURIRequest;
  /// C 2617: `webkit_web_view_load_request (newview, request);`
  /// C: webkit2/WebKitWebView.h: `void webkit_web_view_load_request (WebKitWebView *web_view,
  /// WebKitURIRequest *request);`
  pub fn webkit_web_view_load_request (web_view: *mut WebKitWebView, request: *mut WebKitURIRequest);
  /// C 2683: `type = webkit_script_dialog_get_dialog_type (script_dialog);`
  /// C: webkit2/WebKitScriptDialog.h: `WebKitScriptDialogType webkit_script_dialog_get_dialog_type
  /// (WebKitScriptDialog *dialog);`
  pub fn webkit_script_dialog_get_dialog_type (dialog: *mut WebKitScriptDialog)
    -> WebKitScriptDialogType;
  /// C 2685: `content = webkit_script_dialog_get_message (script_dialog);`
  /// C: webkit2/WebKitScriptDialog.h: `const gchar *webkit_script_dialog_get_message
  /// (WebKitScriptDialog *dialog);`
  pub fn webkit_script_dialog_get_message (dialog: *mut WebKitScriptDialog) -> *const gchar;
  /// C 2706: `message = webkit_script_dialog_prompt_get_default_text (script_dialog);`
  /// C: webkit2/WebKitScriptDialog.h: `const gchar *webkit_script_dialog_prompt_get_default_text
  /// (WebKitScriptDialog *dialog);`
  pub fn webkit_script_dialog_prompt_get_default_text (dialog: *mut WebKitScriptDialog)
    -> *const gchar;
  /// C 2711-2712: `webkit_script_dialog_prompt_set_text (script_dialog, …);`
  /// C: webkit2/WebKitScriptDialog.h: `void webkit_script_dialog_prompt_set_text
  /// (WebKitScriptDialog *dialog, const gchar *text);`
  pub fn webkit_script_dialog_prompt_set_text (dialog: *mut WebKitScriptDialog, text: *const gchar);
  /// C 2709-2710: `webkit_script_dialog_confirm_set_confirmed (script_dialog, result == 0);`
  /// C: webkit2/WebKitScriptDialog.h: `void webkit_script_dialog_confirm_set_confirmed
  /// (WebKitScriptDialog *dialog, gboolean confirmed);`
  pub fn webkit_script_dialog_confirm_set_confirmed (dialog: *mut WebKitScriptDialog,
                                                     confirmed: gboolean);
}

/* JavaScriptCore: the eleven `jsc_value_*` entry points `webkit_js_to_lisp` calls (C 2455-2490).
   No JavaScriptCore tree on this machine, so each citation names the header alone and every
   return/argument shape is **UNVERIFIED**.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 2455: `if (jsc_value_is_string (value))`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_is_string (JSCValue *value);`
  pub fn jsc_value_is_string (value: *mut JSCValue) -> gboolean;
  /// C 2457: `gchar *str_value = jsc_value_to_string (value);`
  /// C: javascriptcore/JSValue.h: `gchar *jsc_value_to_string (JSCValue *value);`
  pub fn jsc_value_to_string (value: *mut JSCValue) -> *mut gchar;
  /// C 2464: `else if (jsc_value_is_boolean (value))`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_is_boolean (JSCValue *value);`
  pub fn jsc_value_is_boolean (value: *mut JSCValue) -> gboolean;
  /// C 2466: `return (jsc_value_to_boolean (value)) ? Qt : Qnil;`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_to_boolean (JSCValue *value);`
  pub fn jsc_value_to_boolean (value: *mut JSCValue) -> gboolean;
  /// C 2468: `else if (jsc_value_is_number (value))`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_is_number (JSCValue *value);`
  pub fn jsc_value_is_number (value: *mut JSCValue) -> gboolean;
  /// C 2470: `return make_fixnum (jsc_value_to_int32 (value));`
  /// C: javascriptcore/JSValue.h: `gint32 jsc_value_to_int32 (JSCValue *value);`
  pub fn jsc_value_to_int32 (value: *mut JSCValue) -> c_int;
  /// C 2472: `else if (jsc_value_is_array (value))`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_is_array (JSCValue *value);`
  pub fn jsc_value_is_array (value: *mut JSCValue) -> gboolean;
  /// C 2474: `JSCValue *len = jsc_value_object_get_property (value, "length");`
  /// C: javascriptcore/JSValue.h: `JSCValue *jsc_value_object_get_property (JSCValue *object,
  /// const char *name);`
  pub fn jsc_value_object_get_property (object: *mut JSCValue, name: *const c_char) -> *mut JSCValue;
  /// C 2489: `webkit_js_to_lisp (jsc_value_object_get_property_at_index (value, i));`
  /// C: javascriptcore/JSValue.h: `JSCValue *jsc_value_object_get_property_at_index (JSCValue
  /// *object, guint index);`
  pub fn jsc_value_object_get_property_at_index (object: *mut JSCValue, index: guint)
    -> *mut JSCValue;
  /// C 2493: `else if (jsc_value_is_object (value))`
  /// C: javascriptcore/JSValue.h: `gboolean jsc_value_is_object (JSCValue *value);`
  pub fn jsc_value_is_object (value: *mut JSCValue) -> gboolean;
  /// C 2495: `char **properties_names = jsc_value_object_enumerate_properties (value);`
  /// C: javascriptcore/JSValue.h: `char **jsc_value_object_enumerate_properties (JSCValue
  /// *object);`
  pub fn jsc_value_object_enumerate_properties (object: *mut JSCValue) -> *mut *mut gchar;
}

/* XInput2.  No XInput2 header on this machine, so the two records carry the members C reads and
   their layouts are **UNVERIFIED**; the block's four `HAVE_XINPUT2` arms are carried as text
   (P-51) but never flipped (`## Owed` 6).  */
/// C: X11/extensions/XInput2.h: `typedef struct { int type; unsigned long serial; Bool
/// send_event; Display *display; Window window; Window root; Window subwindow; Time time; int x,
/// y; int x_root, y_root; int mode; int detail; Window same_screen; Window focus; unsigned int
/// state; unsigned int buttons_len; XIAddData mods; ... } XIEnterEvent;` - C 1993-2035 reads
/// `event_x`/`event_y`/`root_x`/`root_y`/`time`/`focus`/`mode`/`detail`/`mods.effective`/
/// `buttons.mask_len`/`buttons.mask`.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
#[repr (C)]
pub struct XIEnterEvent
{
  pub r#type: c_int,                    // C: XInput2.h: `int type;`
  pub serial: c_ulong,                  // C: XInput2.h: `unsigned long serial;`
  pub send_event: c_int,                // C: XInput2.h: `Bool send_event;`
  pub display: *mut Display,            // C: XInput2.h: `Display *display;`
  pub window: Window,                   // C: XInput2.h: `Window window;`
  pub root: Window,                     // C: XInput2.h: `Window root;`
  pub subwindow: Window,                // C: XInput2.h: `Window subwindow;`
  pub time: Time,                       // C: XInput2.h: `Time time;`
  pub x: c_int,                         // C: XInput2.h: `int x, y;`
  pub y: c_int,
  pub root_x: c_int,                    // C: XInput2.h: `int root_x, root_y;` (C 2018-2019)
  pub root_y: c_int,
  pub mode: c_int,                      // C: XInput2.h: `int mode;`
  pub detail: c_int,                    // C: XInput2.h: `int detail;`
  pub same_screen: Window,              // C: XInput2.h: `Window same_screen;`
  pub focus: c_int,                     // C: XInput2.h: `Bool focus;`
  pub state: c_uint,                    // C: XInput2.h: `unsigned int state;`
  pub buttons_len: c_uint,              // C: XInput2.h: `unsigned int buttons_len;`
  pub mods: XIAddData,                  // C: XInput2.h: `XIAddData mods;`
  // C 2023-2024 reads `xev->buttons.mask_len` / `xev->buttons.mask`, which the real header spells
  // through `XIButtonState`; the two members stand here (see the file note: no XInput2 header).
  pub buttons: XIButtonState,
  // C: XInput2.h: `int event_x, event_y;` (C 2001-2002)
  pub event_x: c_double,
  pub event_y: c_double,
}
/// C: X11/extensions/XInput2.h: `typedef struct { int effective; int mask; ... } XIAddData;` -
/// the `mods` member above (C 2030's `xev->mods.effective`).  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
#[repr (C)]
pub struct XIAddData
{
  pub effective: c_int,                 // C: XInput2.h: `int effective;`
  pub mask: c_int,                      // C: XInput2.h: `int mask;`
}
/// C: X11/extensions/XInput2.h: `typedef struct { int mask_len; unsigned char *mask; }
/// XIButtonState;` - C 2023-2024's `xev->buttons.mask_len`/`buttons.mask`.  **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
#[repr (C)]
pub struct XIButtonState
{
  pub mask_len: c_int,                  // C: XInput2.h: `int mask_len;`
  pub mask: *mut c_uchar,               // C: XInput2.h: `unsigned char *mask;`
}
/// C: X11/extensions/XInput2.h: `typedef struct { int type; unsigned long serial; Bool
/// send_event; Display *display; Window window; Window root; ... Time time; int event_x, event_y;
/// ... double delta_x, delta_y; double delta_angle; double scale; ... Time root_x, root_y? ...;
/// XIAddData mods; int evtype; ... } XIGesturePinchEvent;` - C 1497-1507 reads `event_x`/
/// `event_y`/`delta_x`/`delta_y`/`delta_angle`/`scale`/`root_x`/`root_y`/`mods.effective`/`evtype`.
/// **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2",
            feature = "have_xinput2_4"))]
#[repr (C)]
pub struct XIGesturePinchEvent
{
  pub r#type: c_int,                    // C: XInput2.h: `int type;`
  pub serial: c_ulong,                  // C: XInput2.h: `unsigned long serial;`
  pub send_event: c_int,                // C: XInput2.h: `Bool send_event;`
  pub display: *mut Display,            // C: XInput2.h: `Display *display;`
  pub window: Window,                   // C: XInput2.h: `Window window;`
  pub root: Window,                     // C: XInput2.h: `Window root;`
  pub subwindow: Window,                // C: XInput2.h: `Window subwindow;`
  pub time: Time,                       // C: XInput2.h: `Time time;`
  pub event_x: c_double,                // C: XInput2.h: `double event_x, event_y;`
  pub event_y: c_double,
  pub root_x: c_double,                 // C: XInput2.h: `double root_x, root_y;`
  pub root_y: c_double,
  pub delta_x: c_double,                // C: XInput2.h: `double delta_x, delta_y;`
  pub delta_y: c_double,
  pub delta_angle: c_double,            // C: XInput2.h: `double delta_angle;`
  pub scale: c_double,                  // C: XInput2.h: `double scale;`
  pub mods: XIAddData,                  // C: XInput2.h: `XIAddData mods;`
  pub evtype: c_int,                    // C: XInput2.h: `int evtype;`
}
/// C: X11/extensions/XInput2.h 183: `#define XIMaskIsSet(ptr, event) (((unsigned char*)(ptr))
/// [(event)>>3] & (1 << ((event) & 7)))` - the header's own body (XInput2.h 183, cited from the
/// `X11/extensions/XI2.h` this machine has).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
macro_rules! XIMaskIsSet {
  ($ptr:expr, $event:expr) => {
    ((* ($ptr as *const c_uchar).offset ((($event) >> 3) as isize) as c_int)
     & (1 << (($event) & 7))) };
}
/// C: X11/extensions/XI2.h 197-198 and 217-219: `XI_Enter 7`, `XI_Leave 8`,
/// `XI_GesturePinchBegin 27`, `XI_GesturePinchUpdate 28`, `XI_GesturePinchEnd 29` - the values
/// are the header's (measured on this machine's `/opt/homebrew/include/X11/extensions/XI2.h`).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_Enter: c_int = 7;                 // XI2.h 197
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_Leave: c_int = 8;                 // XI2.h 198
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchBegin: c_int = 27;    // XI2.h 217
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchUpdate: c_int = 28;   // XI2.h 218
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XI_GesturePinchEnd: c_int = 29;      // XI2.h 219

/* X11, continued: the six entry points C 1219/1225/2160/2170/2188/2193 calls.  X11 *is* on this
   machine, so the prototypes are the header's own; `include/xlib.rs` carries only the ten
   procedures its own consumers read, so these stand here (P-72 rule 4).  */
/// C: X11/X.h 216-217: `#define MotionNotify 6` / `#define EnterNotify 7` / `#define LeaveNotify
/// 8`; X11/X.h 223: `#define GenericEvent 35` - the four event types `xwidget_motion_or_crossing`
/// compares (C 1951-2065).  Values measured on this machine.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const MotionNotify: c_int = 6;             // X.h 216
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const EnterNotify: c_int = 7;              // X.h 217
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const LeaveNotify: c_int = 8;              // X.h 218
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
pub const GenericEvent: c_int = 35;            // X.h 223
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" {
  /// C 2160: `XMoveWindow (xv->dpy, xv->wdesc, xv->x + xv->clip_left, xv->y + xv->clip_top);`
  /// C: X11/Xlib.h: `int XMoveWindow (Display *display, Window w, int x, int y);`
  pub fn XMoveWindow (display: *mut Display, w: Window, x: c_int, y: c_int) -> c_int;
  /// C 2161: `XMapWindow (xv->dpy, xv->wdesc);`
  /// C: X11/Xlib.h: `int XMapWindow (Display *display, Window w);`
  pub fn XMapWindow (display: *mut Display, w: Window) -> c_int;
  /// C 2171: `XUnmapWindow (xv->dpy, xv->wdesc);`
  /// C: X11/Xlib.h: `int XUnmapWindow (Display *display, Window w);`
  pub fn XUnmapWindow (display: *mut Display, w: Window) -> c_int;
  /// C 2162/2172: `XFlush (xv->dpy);`
  /// C: X11/Xlib.h: `int XFlush (Display *display);`
  pub fn XFlush (display: *mut Display) -> c_int;
  /// C 1758: `XDefineCursor (xv->dpy, xv->wdesc, xv->cursor);`
  /// C: X11/Xlib.h: `int XDefineCursor (Display *display, Window w, Cursor cursor);`
  pub fn XDefineCursor (display: *mut Display, w: Window, cursor: Cursor) -> c_int;
  /// C 2188: `XClearWindow (xw->dpy, xw->wdesc);`
  /// C: X11/Xlib.h: `int XClearWindow (Display *display, Window w);`
  pub fn XClearWindow (display: *mut Display, w: Window) -> c_int;
}

/* termhooks.h's two `HAVE_XWIDGETS` `enum event_kind` members.  The mirror carries the frozen
   configuration's members and stops before the gated ones (`include/termhooks.rs` 27-58), so
   their values in a build that *has* xwidgets are declared here (P-88 ①, the `src/gfilenotify.rs`
   161-169 `FILE_NOTIFY_EVENT` precedent): `SAVE_SESSION_EVENT` (26) is followed by
   `CONFIG_CHANGED_EVENT` (27, the frozen `HAVE_DBUS`/`THREADS_ENABLED` arms absent), so
   `XWIDGET_EVENT` is 28 and `XWIDGET_DISPLAY_EVENT` 29.  Owed to the mirror (`## Owed` 3).  */
/// C: termhooks.h 273-281: `#ifdef HAVE_XWIDGETS … , XWIDGET_EVENT … , XWIDGET_DISPLAY_EVENT`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const XWIDGET_EVENT: c_int = 28;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const XWIDGET_DISPLAY_EVENT: c_int = 29;
/// termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
/// `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80).  C 2285/2300/2320/2341 write
/// `event.kind = …`, so the port keeps C's field name in a per-unit macro - the
/// `src/gfilenotify.rs` 183-189 shape, body-identical.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}
/// lisp.h 71-73's `min`/`max` macros (the `src/w32console.rs` 346 shape): C 2069's
/// `min (5, event->xcrossing.detail)`.  `include/lisp.rs` carries only the `isize` *functions*
/// of the same names, which the macro coexists with (the `src/alloc.rs` 127 note).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("../lib/include/min-max.rs");

/* gtkutil.h's tooltip preparer (gtkutil.h 207), read by C 1786.  `include/gtkutil.rs` is a
   placeholder that carries the guard alone, so the prototype stands here (P-72 rule 4) and is
   owed (`## Owed` 3).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" {
  /// C 1786: `xg_prepare_tooltip (view->frame, dummy_tooltip_string, &tooltip_width,
  /// &tooltip_height);`
  /// C: gtkutil.h 207-208: `extern bool xg_prepare_tooltip (struct frame *f, Lisp_Object string,
  /// int *width, int *height);`
  pub fn xg_prepare_tooltip (f: *mut frame, string: Lisp_Object, width: *mut c_int,
                             height: *mut c_int) -> bool;
}

/* X11, continued: the `XEvent` union's member records `xwidget_motion_or_crossing` reads
   (C 1945-2105).  `include/xlib.rs`'s `XEvent` is the measured 192-byte record with an opaque
   body (its 33 member records are owed, `docs/ports/include-xlib.md`), so the port casts the
   `XEvent *` to the member record C names - the `src/xmenu.rs` 1086 `event->xcookie` shape.
   **X11 is on this machine**, so each layout below is measured (Evidence ⑥).  */
/// C: X11/Xlib.h 395-403: `typedef struct { int type; unsigned long serial; Bool send_event;
/// Display *display; Window window; } XAnyEvent;` - C 1951-2065's `event->type`.  Measured:
/// `sizeof` 40, `type`@0, `serial`@8, `send_event`@16, `display`@24, `window`@32.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[repr (C)]
pub struct XAnyEvent
{
  pub r#type: c_int,                    // C: Xlib.h: `int type;`
  pub serial: c_ulong,                  // C: Xlib.h: `unsigned long serial;`
  pub send_event: c_int,                // C: Xlib.h: `Bool send_event;`
  pub display: *mut Display,            // C: Xlib.h: `Display *display;`
  pub window: Window,                   // C: Xlib.h: `Window window;`
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XAnyEvent> () == 40);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, window) == 32);
/// C: X11/Xlib.h 527-547: the `XMotionEvent` record - C 1963-1987/2078-2085's `event->xmotion`.
/// Measured: `sizeof` 96, `time`@56, `x`@64, `y`@68, `x_root`@72, `y_root`@76, `state`@80.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[repr (C)]
pub struct XMotionEvent
{
  pub r#type: c_int,                    // C: Xlib.h: `int type;`
  pub serial: c_ulong,                  // C: Xlib.h: `unsigned long serial;`
  pub send_event: c_int,                // C: Xlib.h: `Bool send_event;`
  pub display: *mut Display,            // C: Xlib.h: `Display *display;`
  pub window: Window,                   // C: Xlib.h: `Window window;`
  pub root: Window,                     // C: Xlib.h: `Window root;`
  pub subwindow: Window,                // C: Xlib.h: `Window subwindow;`
  pub time: Time,                       // C: Xlib.h: `Time time;`
  pub x: c_int,                         // C: Xlib.h: `int x, y;`
  pub y: c_int,
  pub x_root: c_int,                    // C: Xlib.h: `int x_root, y_root;`
  pub y_root: c_int,
  pub state: c_uint,                    // C: Xlib.h: `unsigned int state;`
  pub is_hint: c_char,                  // C: Xlib.h: `char is_hint;`
  pub same_screen: c_int,               // C: Xlib.h: `Bool same_screen;`
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XMotionEvent> () == 96);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XMotionEvent, x) == 64);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XMotionEvent, state) == 80);
/// C: X11/Xlib.h 564-593: the `XCrossingEvent` record - C 1964-1969/2053-2065's
/// `event->xcrossing`.  Measured: `sizeof` 104, `time`@56, `x`@64, `mode`@80, `detail`@84,
/// `focus`@92, `state`@96.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[repr (C)]
pub struct XCrossingEvent
{
  pub r#type: c_int,                    // C: Xlib.h: `int type;`
  pub serial: c_ulong,                  // C: Xlib.h: `unsigned long serial;`
  pub send_event: c_int,                // C: Xlib.h: `Bool send_event;`
  pub display: *mut Display,            // C: Xlib.h: `Display *display;`
  pub window: Window,                   // C: Xlib.h: `Window window;`
  pub root: Window,                     // C: Xlib.h: `Window root;`
  pub subwindow: Window,                // C: Xlib.h: `Window subwindow;`
  pub time: Time,                       // C: Xlib.h: `Time time;`
  pub x: c_int,                         // C: Xlib.h: `int x, y;`
  pub y: c_int,
  pub x_root: c_int,                    // C: Xlib.h: `int x_root, y_root;`
  pub y_root: c_int,
  pub mode: c_int,                      // C: Xlib.h: `int mode;`
  pub detail: c_int,                    // C: Xlib.h: `int detail;`
  pub same_screen: c_int,               // C: Xlib.h: `Bool same_screen;`
  pub focus: c_int,                     // C: Xlib.h: `Bool focus;`
  pub state: c_uint,                    // C: Xlib.h: `unsigned int state;`
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XCrossingEvent> () == 104);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XCrossingEvent, mode) == 80);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XCrossingEvent, state) == 96);
/// C: X11/Xlib.h 1168-1180: the `XGenericEventCookie` record - C 1993-1995's
/// `event->xcookie.evtype`/`data`.  Measured: `sizeof` 56, `evtype`@36, `data`@48 (the same
/// record `src/xmenu.rs` 417-437 declares).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[repr (C)]
pub struct XGenericEventCookie
{
  pub r#type: c_int,                    // C: Xlib.h: `int type;`
  pub serial: c_ulong,                  // C: Xlib.h: `unsigned long serial;`
  pub send_event: c_int,                // C: Xlib.h: `Bool send_event;`
  pub display: *mut Display,            // C: Xlib.h: `Display *display;`
  pub extension: c_int,                 // C: Xlib.h: `int extension;`
  pub evtype: c_int,                    // C: Xlib.h: `int evtype;`
  pub cookie: c_uint,                   // C: Xlib.h: `unsigned int cookie;`
  pub data: *mut c_void,                // C: Xlib.h: `void *data;`
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XGenericEventCookie> () == 56);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XGenericEventCookie, evtype) == 36);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XGenericEventCookie, data) == 48);
/// C: X11/extensions/XI2.h 57-61: `#define XINotifyAncestor 0`, `XINotifyVirtual 1`,
/// `XINotifyInferior 2`, `XINotifyNonlinear 3`, `XINotifyNonlinearVirtual 4` - the five values
/// `xi_translate_notify_detail` maps (C 1560-1572).  Measured on this machine's
/// `/opt/homebrew/include/X11/extensions/XI2.h`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XINotifyAncestor: c_int = 0;             // XI2.h 57
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XINotifyVirtual: c_int = 1;              // XI2.h 58
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XINotifyInferior: c_int = 2;             // XI2.h 59
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XINotifyNonlinear: c_int = 3;            // XI2.h 60
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
pub const XINotifyNonlinearVirtual: c_int = 4;     // XI2.h 61

/* cairo, continued: the three entry points `xv_do_draw`/`xwidget_view_draw_cb` call (C 2191-2199,
   C 2232-2238).  cairo *is* on this machine; the prototypes are the header's own, and
   `include/gtk.rs` carries `cairo_paint`/`cairo_set_source_surface`/`cairo_set_operator` but not
   these three.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 2191/2233: `cairo_save (xw->cr_context);`
  /// C: cairo.h: `void cairo_save (cairo_t *cr);`
  pub fn cairo_save (cr: *mut cairo_t);
  /// C 2199/2238: `cairo_restore (xw->cr_context);`
  /// C: cairo.h: `void cairo_restore (cairo_t *cr);`
  pub fn cairo_restore (cr: *mut cairo_t);
  /// C 2193/2235: `cairo_translate (xw->cr_context, -xw->clip_left, -xw->clip_top);`
  /// C: cairo.h: `void cairo_translate (cairo_t *cr, double tx, double ty);`
  pub fn cairo_translate (cr: *mut cairo_t, tx: c_double, ty: c_double);
}

/* ==============================================================================================
   C 1182-2716: the `HAVE_X_WINDOWS` drag/button/motion/crossing machinery, the WebKit callbacks
   and the script dialog.  C's statements in C's order, C's names; `->` is `.`, C's implicit
   conversions are explicit `as` casts (Deviation 5).  Where C reads a member of a union whose
   body this tree does not transcribe (`XEvent`, `GdkEvent`), the pointer is cast to the member
   record C names first (the `src/xmenu.rs` 1086 shape) and the gap is recorded (`## Owed`).
   ============================================================================================== */

/* C 1182-2107: `#ifdef HAVE_X_WINDOWS`.  */
/// C 1184-1200: the `"drag-begin"` handler `xwidget_button_1` connects.  `view = user_data`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" fn xv_drag_begin_cb (widget: *mut GtkWidget, context: *mut GdkDragContext,
                                       user_data: gpointer)  // C 1184-1200
{
  let view: *mut xwidget_view = user_data as *mut xwidget_view;

  if ! (*view).passive_grab.is_null ()
    {
      g_signal_handler_disconnect ((*view).passive_grab as gpointer,
				   (*view).passive_grab_destruction_signal as gulong);
      g_signal_handler_disconnect ((*view).passive_grab as gpointer,
				   (*view).passive_grab_drag_signal as gulong);
      (*view).passive_grab = std::ptr::null_mut ();
    }
}

/// C 1202-1326: the press/release synthesiser `xwidget_button` shares with the scroll arm.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xwidget_button_1 (view: *mut xwidget_view, down_p: bool, mut x: c_int, mut y: c_int,
                            button: c_int, modifier_state: c_int, time: Time)  // C 1202-1326
{
  let mut xg_event: *mut GdkEvent;
  let model: *mut xwidget = XXWIDGET ((*view).model);
  let mut target: *mut GtkWidget;
  let mut ungrab_target: *mut GtkWidget;
  let mut toplevel: *mut GdkWindow;
  let mut target_window: *mut GdkWindow;
  let mut view_x: c_int;
  let mut view_y: c_int;

  /* X and Y should be relative to the origin of view->wdesc.  */
  x += (*view).clip_left;
  y += (*view).clip_top;

  view_x = x;
  view_y = y;

  target = find_widget_at_pos ((*model).widgetwindow_osr, x, y, &raw mut x, &raw mut y,
			       true, view);

  if target.is_null ()
    { target = (*model).widget_osr; }

  xg_event = gdk_event_new (if down_p { GDK_BUTTON_PRESS } else { GDK_BUTTON_RELEASE });

  (*xg_event).any.window = gtk_widget_get_window (target);
  g_object_ref ((*xg_event).any.window as gpointer); /* The window will be unrefed
							later by gdk_event_free.  */

  (*xg_event).button.x = x as c_double;
  (*xg_event).button.x_root = x as c_double;
  (*xg_event).button.y = y as c_double;
  (*xg_event).button.y_root = y as c_double;
  (*xg_event).button.button = button as c_uint;
  (*xg_event).button.state = modifier_state as c_uint;
  (*xg_event).button.time = time as c_uint;
  (*xg_event).button.device = find_suitable_pointer ((*view).frame as *mut frame, false);

  gtk_main_do_event (xg_event);
  gdk_event_free (xg_event);


  if down_p && (*view).passive_grab.is_null ()
    {
      (*view).passive_grab = target;
      (*view).passive_grab_destruction_signal =
	g_signal_connect! (G_OBJECT! ((*view).passive_grab),
			   c"destroy".as_ptr (), G_CALLBACK! (gtk_widget_destroyed),
			   &raw mut (*view).passive_grab) as guint;
      (*view).passive_grab_drag_signal =
	g_signal_connect! (G_OBJECT! ((*view).passive_grab),
			   c"drag-begin".as_ptr (), G_CALLBACK! (xv_drag_begin_cb),
			   view as gpointer) as guint;
    }
  else
    {
      ungrab_target = find_widget_at_pos ((*model).widgetwindow_osr,
					  view_x, view_y, &raw mut x, &raw mut y,
					  false, std::ptr::null_mut ());

      if ! (*view).last_crossing_window.is_null () && ! ungrab_target.is_null ()
	{
	  xw_maybe_synthesize_crossing (view, gtk_widget_get_window (ungrab_target),
					view_x, view_y, XW_CROSSING_NONE,
					time, modifier_state as c_uint, GDK_CROSSING_GTK_UNGRAB,
					GDK_CROSSING_GTK_UNGRAB);
	}
      else
	{
	  toplevel = gtk_widget_get_window ((*model).widgetwindow_osr);
	  xg_event = gdk_event_new (GDK_LEAVE_NOTIFY);
	  target_window = gtk_widget_get_window (target);
	  window_coords_from_toplevel (target_window, toplevel, view_x,
				       view_y, &raw mut x, &raw mut y);

	  (*xg_event).crossing.x = x as c_double;
	  (*xg_event).crossing.y = y as c_double;
	  (*xg_event).crossing.time = time as c_uint;
	  (*xg_event).crossing.focus = FALSE;
	  (*xg_event).crossing.detail = GDK_NOTIFY_ANCESTOR;
	  (*xg_event).crossing.mode = GDK_CROSSING_GTK_UNGRAB;
	  (*xg_event).crossing.window = g_object_ref (target_window as gpointer) as *mut GdkWindow;
	  gdk_event_set_device (xg_event,
				find_suitable_pointer ((*view).frame as *mut frame, false));

	  gtk_main_do_event (xg_event);
	  gdk_event_free (xg_event);

	  xw_notify_virtual_upwards_until (view, target_window, toplevel, toplevel,
					   modifier_state as c_uint, view_x, view_y, time,
					   GDK_LEAVE_NOTIFY, false,
					   GDK_CROSSING_GTK_UNGRAB);

	  if target_window != toplevel
	    {
	      xg_event = gdk_event_new (GDK_LEAVE_NOTIFY);

	      (*xg_event).crossing.x = view_y as c_double;
	      (*xg_event).crossing.y = view_y as c_double;
	      (*xg_event).crossing.time = time as c_uint;
	      (*xg_event).crossing.focus = FALSE;
	      (*xg_event).crossing.detail = GDK_NOTIFY_VIRTUAL;
	      (*xg_event).crossing.mode = GDK_CROSSING_GTK_UNGRAB;
	      (*xg_event).crossing.window = g_object_ref (toplevel as gpointer) as *mut GdkWindow;

	      gdk_event_set_device (xg_event,
				    find_suitable_pointer ((*view).frame as *mut frame, false));
	      gtk_main_do_event (xg_event);
	      gdk_event_free (xg_event);
	    }

	}

      if ! (*view).passive_grab.is_null ()
	{
	  g_signal_handler_disconnect ((*view).passive_grab as gpointer,
				       (*view).passive_grab_destruction_signal as gulong);
	  g_signal_handler_disconnect ((*view).passive_grab as gpointer,
				       (*view).passive_grab_drag_signal as gulong);
	  (*view).passive_grab = std::ptr::null_mut ();
	}
    }
}

/* C 1328-1385: `xwidget_button` is one of the file's non-`static` definitions (xwidget.h 217
   declares it `extern`), so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_button (view: *mut xwidget_view, down_p: bool, mut x: c_int,
                                         mut y: c_int, button: c_int, modifier_state: c_int,
                                         time: Time)  // C 1328-1385
{
  if NILP ((*XXWIDGET ((*view).model)).buffer)
    { return; }

  record_osr_embedder (view);

  if button < 4 || button > 8
    { xwidget_button_1 (view, down_p, x, y, button, modifier_state, time); }
  else
    {
      if ! down_p
	{
	  let mut xg_event: *mut GdkEvent = gdk_event_new (GDK_SCROLL);
	  let model: *mut xwidget = XXWIDGET ((*view).model);
	  let mut target: *mut GtkWidget;

	  x += (*view).clip_left;
	  y += (*view).clip_top;

	  target = find_widget_at_pos ((*model).widgetwindow_osr, x, y, &raw mut x, &raw mut y,
				       true, view);

	  if target.is_null ()
	    { target = (*model).widget_osr; }

	  (*xg_event).any.window = gtk_widget_get_window (target);
	  g_object_ref ((*xg_event).any.window as gpointer); /* The window will be unrefed
								later by gdk_event_free.  */
	  if button == 4
	    { (*xg_event).scroll.direction = GDK_SCROLL_UP; }
	  else if button == 5
	    { (*xg_event).scroll.direction = GDK_SCROLL_DOWN; }
	  else if button == 6
	    { (*xg_event).scroll.direction = GDK_SCROLL_LEFT; }
	  else
	    { (*xg_event).scroll.direction = GDK_SCROLL_RIGHT; }

	  (*xg_event).scroll.device = find_suitable_pointer ((*view).frame as *mut frame,
							     false);

	  (*xg_event).scroll.x = x as c_double;
	  (*xg_event).scroll.x_root = x as c_double;
	  (*xg_event).scroll.y = y as c_double;
	  (*xg_event).scroll.y_root = y as c_double;
	  (*xg_event).scroll.state = modifier_state as c_uint;
	  (*xg_event).scroll.time = time as c_uint;

	  (*xg_event).scroll.delta_x = 0 as c_double;
	  (*xg_event).scroll.delta_y = 0 as c_double;

	  gtk_main_do_event (xg_event);
	  gdk_event_free (xg_event);
	}
    }
}

/* C 1387-1553: `#ifdef HAVE_XINPUT2` - `xwidget_motion_notify`, `xwidget_scroll` and (inside
   C 1489's `#ifdef HAVE_XINPUT2_4` and C 1493's `#if GTK_CHECK_VERSION (3, 18, 0)`)
   `xwidget_pinch`.  The three are the file's non-`static` definitions (xwidget.h 222-227), so
   they carry `#[unsafe(no_mangle)]`; the XInput2 text is carried but never flipped here
   (`## Owed` 6).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_motion_notify (view: *mut xwidget_view, x: c_double, y: c_double,
                                                 root_x: c_double, root_y: c_double,
                                                 state: c_uint, time: Time)  // C 1389-1440
{
  let mut xg_event: *mut GdkEvent;
  let mut target: *mut GtkWidget;
  let model: *mut xwidget = XXWIDGET ((*view).model);
  let mut target_x: c_int;
  let mut target_y: c_int;

  if NILP ((*model).buffer)
    { return; }

  record_osr_embedder (view);

  target = find_widget_at_pos ((*model).widgetwindow_osr,
			       lrint (x + (*view).clip_left as c_double) as c_int,
			       lrint (y + (*view).clip_top as c_double) as c_int,
			       &raw mut target_x, &raw mut target_y,
			       true, view);

  if target.is_null ()
    {
      target_x = lrint (x + (*view).clip_left as c_double) as c_int;
      target_y = lrint (y + (*view).clip_top as c_double) as c_int;
      target = (*model).widget_osr;
    }
  else if xw_maybe_synthesize_crossing (view, gtk_widget_get_window (target),
					(x + (*view).clip_left as c_double) as c_int,
					(y + (*view).clip_top as c_double) as c_int,
					XW_CROSSING_NONE, time, state,
					(if ! (*view).passive_grab.is_null ()
					 { GDK_CROSSING_GTK_GRAB }
					 else { GDK_CROSSING_NORMAL }),
					GDK_CROSSING_NORMAL)
    { return; }

  xg_event = gdk_event_new (GDK_MOTION_NOTIFY);
  (*xg_event).any.window = gtk_widget_get_window (target);
  (*xg_event).motion.x = target_x as c_double;
  (*xg_event).motion.y = target_y as c_double;
  (*xg_event).motion.x_root = root_x;
  (*xg_event).motion.y_root = root_y;
  (*xg_event).motion.time = time as c_uint;
  (*xg_event).motion.state = state;
  (*xg_event).motion.device = find_suitable_pointer ((*view).frame as *mut frame, false);

  g_object_ref ((*xg_event).any.window as gpointer);

  gtk_main_do_event (xg_event);
  gdk_event_free (xg_event);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_scroll (view: *mut xwidget_view, x: c_double, y: c_double,
                                         dx: c_double, dy: c_double, state: c_uint, time: Time,
                                         stop_p: bool)  // C 1442-1487
{
  let mut xg_event: *mut GdkEvent;
  let mut target: *mut GtkWidget;
  let model: *mut xwidget = XXWIDGET ((*view).model);
  let mut target_x: c_int;
  let mut target_y: c_int;

  if NILP ((*model).buffer)
    { return; }

  record_osr_embedder (view);

  target = find_widget_at_pos ((*model).widgetwindow_osr,
			       lrint (x + (*view).clip_left as c_double) as c_int,
			       lrint (y + (*view).clip_top as c_double) as c_int,
			       &raw mut target_x, &raw mut target_y,
			       true, view);

  if target.is_null ()
    {
      target_x = lrint (x) as c_int;
      target_y = lrint (y) as c_int;
      target = (*model).widget_osr;
    }

  xg_event = gdk_event_new (GDK_SCROLL);
  (*xg_event).any.window = gtk_widget_get_window (target);
  (*xg_event).scroll.direction = GDK_SCROLL_SMOOTH;
  (*xg_event).scroll.x = target_x as c_double;
  (*xg_event).scroll.y = target_y as c_double;
  (*xg_event).scroll.x_root = lrint (x) as c_double;
  (*xg_event).scroll.y_root = lrint (y) as c_double;
  (*xg_event).scroll.time = time as c_uint;
  (*xg_event).scroll.state = state;
  (*xg_event).scroll.delta_x = dx;
  (*xg_event).scroll.delta_y = dy;
  (*xg_event).scroll.device = find_suitable_pointer ((*view).frame as *mut frame, true);
  (*xg_event).scroll.is_stop = stop_p;

  g_object_ref ((*xg_event).any.window as gpointer);

  gtk_main_do_event (xg_event);
  gdk_event_free (xg_event);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2",
	    feature = "have_xinput2_4"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_pinch (view: *mut xwidget_view,
                                        xev: *mut XIGesturePinchEvent)  // C 1491-1550
{
  #[cfg (feature = "gtk_check_version_ge_3_18")]
  {
    let mut xg_event: *mut GdkEvent;
    let mut target: *mut GtkWidget;
    let model: *mut xwidget = XXWIDGET ((*view).model);
    let mut target_x: c_int;
    let mut target_y: c_int;
    let x: c_double = (*xev).event_x;
    let y: c_double = (*xev).event_y;

    if NILP ((*model).buffer)
      { return; }

    record_osr_embedder (view);

    target = find_widget_at_pos ((*model).widgetwindow_osr,
				 lrint (x + (*view).clip_left as c_double) as c_int,
				 lrint (y + (*view).clip_top as c_double) as c_int,
				 &raw mut target_x, &raw mut target_y,
				 true, view);

    if target.is_null ()
      {
	target_x = lrint (x) as c_int;
	target_y = lrint (y) as c_int;
	target = (*model).widget_osr;
      }

    xg_event = gdk_event_new (GDK_TOUCHPAD_PINCH);
    (*xg_event).any.window = gtk_widget_get_window (target);
    (*xg_event).touchpad_pinch.x = target_x as c_double;
    (*xg_event).touchpad_pinch.y = target_y as c_double;
    (*xg_event).touchpad_pinch.dx = (*xev).delta_x;
    (*xg_event).touchpad_pinch.dy = (*xev).delta_y;
    (*xg_event).touchpad_pinch.angle_delta = (*xev).delta_angle;
    (*xg_event).touchpad_pinch.scale = (*xev).scale;
    (*xg_event).touchpad_pinch.x_root = (*xev).root_x;
    (*xg_event).touchpad_pinch.y_root = (*xev).root_y;
    (*xg_event).touchpad_pinch.state = (*xev).mods.effective as c_uint;
    (*xg_event).touchpad_pinch.n_fingers = 2;

    match (*xev).evtype
      {
      XI_GesturePinchBegin =>
	{
	  (*xg_event).touchpad_pinch.phase = GDK_TOUCHPAD_GESTURE_PHASE_BEGIN;
	}
      XI_GesturePinchUpdate =>
	{
	  (*xg_event).touchpad_pinch.phase = GDK_TOUCHPAD_GESTURE_PHASE_UPDATE;
	}
      XI_GesturePinchEnd =>
	{
	  (*xg_event).touchpad_pinch.phase = GDK_TOUCHPAD_GESTURE_PHASE_END;
	}
      _ => {}
      }

    gdk_event_set_device (xg_event, find_suitable_pointer ((*view).frame as *mut frame, false));

    g_object_ref ((*xg_event).any.window as gpointer);
    gtk_main_do_event (xg_event);
    gdk_event_free (xg_event);
  }
}

/* C 1555-1575: `#ifdef HAVE_XINPUT2` - the XI notify-detail translator, `GdkNotifyType`'s one
   producer.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows", feature = "have_xinput2"))]
unsafe fn xi_translate_notify_detail (detail: c_int) -> GdkNotifyType  // C 1557-1573
{
  match detail
    {
    XINotifyInferior =>
      {
	return GDK_NOTIFY_INFERIOR;
      }
    XINotifyAncestor =>
      {
	return GDK_NOTIFY_ANCESTOR;
      }
    XINotifyVirtual =>
      {
	return GDK_NOTIFY_VIRTUAL;
      }
    XINotifyNonlinear =>
      {
	return GDK_NOTIFY_NONLINEAR;
      }
    XINotifyNonlinearVirtual =>
      {
	return GDK_NOTIFY_NONLINEAR_VIRTUAL;
      }
    _ =>
      {
	emacs_abort ();
      }
    }
}

/// C 1578-1607: the window-coordinate walk the crossing machinery shares.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn window_coords_from_toplevel (mut window: *mut GdkWindow, toplevel: *mut GdkWindow,
                                       x: c_int, y: c_int, out_x: *mut c_int,
                                       out_y: *mut c_int)  // C 1578-1607
{
  let mut parent: *mut GdkWindow;
  let mut children: *mut GList;
  let mut l: *mut GList;
  let mut x_out: c_double = 0 as c_double;
  let mut y_out: c_double = 0 as c_double;

  if window == toplevel
    {
      *out_x = x;
      *out_y = y;
      return;
    }

  children = std::ptr::null_mut ();
  parent = gdk_window_get_parent (window);
  while parent != toplevel
    {
      children = g_list_prepend (children, window as gpointer);
      window = parent;
      parent = gdk_window_get_parent (window);
    }

  l = children;
  while ! l.is_null ()
    {
      gdk_window_coords_from_parent ((*l).data as *mut GdkWindow, x as c_double, y as c_double,
				     &raw mut x_out, &raw mut y_out);
      l = (*l).next;
    }

  g_list_free (children);

  *out_x = x_out as c_int;
  *out_y = y_out as c_int;
}

/// C 1609-1648: the common-ancestor search `xw_maybe_synthesize_crossing` uses.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_find_common_ancestor (window: *mut GdkWindow, other: *mut GdkWindow,
                                   toplevel: *mut GdkWindow) -> *mut GdkWindow  // C 1609-1648
{
  let mut tem: *mut GdkWindow;
  let mut l1: *mut GList = std::ptr::null_mut ();
  let mut l2: *mut GList = std::ptr::null_mut ();
  let mut i1: *mut GList;
  let mut i2: *mut GList;

  tem = window;
  while ! tem.is_null () && tem != toplevel
    {
      l1 = g_list_prepend (l1, tem as gpointer);
      tem = gdk_window_get_parent (tem);
    }

  tem = other;
  while ! tem.is_null () && tem != toplevel
    {
      l2 = g_list_prepend (l2, tem as gpointer);
      tem = gdk_window_get_parent (tem);
    }

  tem = std::ptr::null_mut ();
  i1 = l1;
  i2 = l2;

  while ! i1.is_null () && ! i2.is_null () && ((*i1).data == (*i2).data)
    {
      tem = (*i1).data as *mut GdkWindow;
      i1 = (*i1).next;
      i2 = (*i2).next;
    }

  g_list_free (l1);
  g_list_free (l2);

  return tem;
}

/// C 1650-1687: the virtual-notify walk towards the toplevel.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_notify_virtual_upwards_until (xv: *mut xwidget_view, window: *mut GdkWindow,
                                           until: *mut GdkWindow, toplevel: *mut GdkWindow,
                                           state: c_uint, x: c_int, y: c_int, time: Time,
                                           type_: GdkEventType, nonlinear_p: bool,
                                           crossing: GdkCrossingMode)  // C 1650-1687
{
  let mut xg_event: *mut GdkEvent;
  let mut tem: *mut GdkWindow;
  let mut cx: c_int;
  let mut cy: c_int;

  tem = gdk_window_get_parent (window);
  while ! tem.is_null () && (tem != until)
    {
      xg_event = gdk_event_new (type_);

      gdk_event_set_device (xg_event,
			    find_suitable_pointer ((*xv).frame as *mut frame, false));
      window_coords_from_toplevel (tem, toplevel, x, y, &raw mut cx, &raw mut cy);
      (*xg_event).crossing.x = cx as c_double;
      (*xg_event).crossing.y = cy as c_double;
      (*xg_event).crossing.time = time as c_uint;
      (*xg_event).crossing.focus = FALSE;
      (*xg_event).crossing.detail = (if nonlinear_p
				     { GDK_NOTIFY_NONLINEAR_VIRTUAL }
				     else { GDK_NOTIFY_VIRTUAL });
      (*xg_event).crossing.mode = crossing;
      (*xg_event).crossing.window = g_object_ref (tem as gpointer) as *mut GdkWindow;

      gtk_main_do_event (xg_event);
      gdk_event_free (xg_event);

      tem = gdk_window_get_parent (tem);
    }
}

/// C 1689-1736: the same walk, collected into a path first so it runs top-down.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_notify_virtual_downwards_until (xv: *mut xwidget_view, window: *mut GdkWindow,
                                             until: *mut GdkWindow, toplevel: *mut GdkWindow,
                                             state: c_uint, x: c_int, y: c_int, time: Time,
                                             type_: GdkEventType, nonlinear_p: bool,
                                             crossing: GdkCrossingMode)  // C 1689-1736
{
  let mut xg_event: *mut GdkEvent;
  let mut tem: *mut GdkWindow;
  let mut cx: c_int;
  let mut cy: c_int;
  let mut path: *mut GList = std::ptr::null_mut ();
  let mut it: *mut GList;

  tem = gdk_window_get_parent (window);

  while ! tem.is_null () && tem != until
    {
      path = g_list_prepend (path, tem as gpointer);
      tem = gdk_window_get_parent (tem);
    }

  it = path;
  while ! it.is_null ()
    {
      tem = (*it).data as *mut GdkWindow;
      xg_event = gdk_event_new (type_);

      gdk_event_set_device (xg_event,
			    find_suitable_pointer ((*xv).frame as *mut frame, false));
      window_coords_from_toplevel (tem, toplevel, x, y, &raw mut cx, &raw mut cy);
      (*xg_event).crossing.x = cx as c_double;
      (*xg_event).crossing.y = cy as c_double;
      (*xg_event).crossing.time = time as c_uint;
      (*xg_event).crossing.focus = FALSE;
      (*xg_event).crossing.detail = (if nonlinear_p
				     { GDK_NOTIFY_NONLINEAR_VIRTUAL }
				     else { GDK_NOTIFY_VIRTUAL });
      (*xg_event).crossing.mode = crossing;
      (*xg_event).crossing.window = g_object_ref (tem as gpointer) as *mut GdkWindow;

      gtk_main_do_event (xg_event);
      gdk_event_free (xg_event);

      it = (*it).next;
    }

  g_list_free (path);
}

/// C 1738-1759: the `"notify::cursor"` handler's worker.  C's `Cursor cursor;` is assigned on
/// both paths before use; the port seeds it for Rust's definite-initialisation (PORTING.md
/// 0.1-13).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_update_cursor_for_view (xv: *mut xwidget_view,
                                     crossing_window: *mut GdkWindow)  // C 1738-1759
{
  use x11_constants::None;

  let mut xg_cursor: *mut GdkCursor;
  let mut cursor: Cursor = 0;

  xg_cursor = gdk_window_get_cursor (crossing_window);

  if ! xg_cursor.is_null ()
    {
      cursor = gdk_x11_cursor_get_xcursor (xg_cursor);

      if gdk_x11_cursor_get_xdisplay (xg_cursor) == (*xv).dpy
	{ (*xv).cursor = cursor; }
    }
  else
    { (*xv).cursor = (* (FRAME_OUTPUT_DATA! ((*xv).frame as *mut frame) as *mut x_output))
		       .nontext_cursor; }

  if (*xv).wdesc != None as Window
    { XDefineCursor ((*xv).dpy, (*xv).wdesc, (*xv).cursor); }
}

/// C 1761-1767: the `"notify::cursor"` handler `xw_maybe_synthesize_crossing` connects.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" fn xw_last_crossing_cursor_cb (window: *mut GdkWindow, spec: *mut GParamSpec,
                                                 user_data: gpointer)  // C 1761-1767
{
  xw_update_cursor_for_view (user_data as *mut xwidget_view, window);
}

/// C 1769-1943: the crossing synthesiser - the file's widest helper, and C 1945's
/// `xwidget_motion_or_crossing`'s worker.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe fn xw_maybe_synthesize_crossing (view: *mut xwidget_view,
                                        current_window: *mut GdkWindow,
                                        x: c_int, y: c_int, crossing: c_int,
                                        time: Time, state: c_uint,
                                        entry_crossing: GdkCrossingMode,
                                        exit_crossing: GdkCrossingMode) -> bool  // C 1769-1943
{
  let mut last_crossing: *mut GdkWindow;
  let mut toplevel: *mut GdkWindow;
  let mut ancestor: *mut GdkWindow;
  let mut xg_event: *mut GdkEvent;
  let mut cx: c_int;
  let mut cy: c_int;
  let mut nonlinear_p: bool;
  let mut retention_flag: bool;

  #[cfg (feature = "webkit_check_version_ge_2_34")]
  {
    /* Work around a silly bug in WebKitGTK+ that tries to make tooltip
       windows transient for our offscreen window.  */
    let mut tooltip_width: c_int = 0;
    let mut tooltip_height: c_int = 0;

    xg_prepare_tooltip ((*view).frame as *mut frame, dummy_tooltip_string,
			&raw mut tooltip_width, &raw mut tooltip_height);
  }

  toplevel = gtk_widget_get_window ((*XXWIDGET ((*view).model)).widgetwindow_osr);
  retention_flag = false;

  if crossing == XW_CROSSING_LEFT
      && (! (*view).last_crossing_window.is_null ()
	  && gdk_window_is_destroyed ((*view).last_crossing_window) == 0)
    {
      xw_notify_virtual_upwards_until (view, (*view).last_crossing_window,
				       toplevel, toplevel,
				       state, x, y, time,
				       GDK_LEAVE_NOTIFY, false,
				       exit_crossing);
    }

  if ! (*view).last_crossing_window.is_null ()
      && (gdk_window_is_destroyed ((*view).last_crossing_window) != 0
	  || crossing == XW_CROSSING_LEFT)
    {
      if gdk_window_is_destroyed ((*view).last_crossing_window) == 0
	  && (*view).last_crossing_window != toplevel
	{
	  xg_event = gdk_event_new (GDK_LEAVE_NOTIFY);
	  window_coords_from_toplevel ((*view).last_crossing_window,
				       toplevel, x, y, &raw mut cx, &raw mut cy);

	  (*xg_event).crossing.x = cx as c_double;
	  (*xg_event).crossing.y = cy as c_double;
	  (*xg_event).crossing.time = time as c_uint;
	  (*xg_event).crossing.focus = FALSE;
	  (*xg_event).crossing.detail = GDK_NOTIFY_ANCESTOR;
	  (*xg_event).crossing.mode = exit_crossing;
	  (*xg_event).crossing.window =
	    g_object_ref ((*view).last_crossing_window as gpointer) as *mut GdkWindow;
	  gdk_event_set_device (xg_event,
				find_suitable_pointer ((*view).frame as *mut frame, false));

	  gtk_main_do_event (xg_event);
	  gdk_event_free (xg_event);

	  xw_notify_virtual_upwards_until (view, (*view).last_crossing_window,
					   gdk_window_get_parent (toplevel),
					   toplevel, state, x, y, time,
					   GDK_LEAVE_NOTIFY, false, exit_crossing);
	  retention_flag = true;
	}

      g_signal_handler_disconnect ((*view).last_crossing_window as gpointer,
				   (*view).last_crossing_cursor_signal as gulong);
      g_clear_pointer! (&raw mut (*view).last_crossing_window, g_object_unref);
    }
  last_crossing = (*view).last_crossing_window;

  if last_crossing.is_null ()
    {
      if ! current_window.is_null ()
	{
	  (*view).last_crossing_window = g_object_ref (current_window as gpointer)
					   as *mut GdkWindow;
	  xw_update_cursor_for_view (view, current_window);
	  (*view).last_crossing_cursor_signal =
	    g_signal_connect! (G_OBJECT! (current_window), c"notify::cursor".as_ptr (),
			       G_CALLBACK! (xw_last_crossing_cursor_cb), view as gpointer) as guint;

	  xw_notify_virtual_downwards_until (view, current_window,
					     toplevel, toplevel,
					     state, x, y, time,
					     GDK_ENTER_NOTIFY,
					     false, entry_crossing);
	}
      return retention_flag;
    }

  if last_crossing != current_window
    {
      (*view).last_crossing_window = g_object_ref (current_window as gpointer)
				       as *mut GdkWindow;
      g_signal_handler_disconnect (last_crossing as gpointer,
				   (*view).last_crossing_cursor_signal as gulong);

      xw_update_cursor_for_view (view, current_window);
      (*view).last_crossing_cursor_signal =
	g_signal_connect! (G_OBJECT! (current_window), c"notify::cursor".as_ptr (),
			   G_CALLBACK! (xw_last_crossing_cursor_cb), view as gpointer) as guint;

      ancestor = xw_find_common_ancestor (last_crossing, current_window, toplevel);

      if ancestor.is_null ()
	{ emacs_abort (); }

      nonlinear_p = (last_crossing != ancestor) && (current_window != ancestor);

      if nonlinear_p || (last_crossing != ancestor)
	{
	  xw_notify_virtual_upwards_until (view, last_crossing,
					   ancestor, toplevel,
					   state, x, y, time,
					   GDK_LEAVE_NOTIFY,
					   nonlinear_p,
					   exit_crossing);
	}

      xg_event = gdk_event_new (GDK_LEAVE_NOTIFY);
      gdk_event_set_device (xg_event,
			    find_suitable_pointer ((*view).frame as *mut frame, false));
      window_coords_from_toplevel (last_crossing, toplevel,
				   x, y, &raw mut cx, &raw mut cy);
      (*xg_event).crossing.x = cx as c_double;
      (*xg_event).crossing.y = cy as c_double;
      (*xg_event).crossing.time = time as c_uint;
      (*xg_event).crossing.focus = FALSE;
      (*xg_event).crossing.state = state;
      (*xg_event).crossing.detail = (if nonlinear_p
				     { GDK_NOTIFY_NONLINEAR }
				     else { (if last_crossing == ancestor
					       { GDK_NOTIFY_INFERIOR }
					       else { GDK_NOTIFY_ANCESTOR }) });
      (*xg_event).crossing.mode = exit_crossing;
      (*xg_event).crossing.window = g_object_ref (last_crossing as gpointer) as *mut GdkWindow;

      gtk_main_do_event (xg_event);
      gdk_event_free (xg_event);

      if nonlinear_p || (current_window != ancestor)
	{
	  xw_notify_virtual_downwards_until (view, current_window,
					     ancestor, toplevel,
					     state, x, y, time,
					     GDK_ENTER_NOTIFY,
					     nonlinear_p,
					     entry_crossing);
	}

      xg_event = gdk_event_new (GDK_ENTER_NOTIFY);
      gdk_event_set_device (xg_event,
			    find_suitable_pointer ((*view).frame as *mut frame, false));
      window_coords_from_toplevel (current_window, toplevel,
				   x, y, &raw mut cx, &raw mut cy);
      (*xg_event).crossing.x = cx as c_double;
      (*xg_event).crossing.y = cy as c_double;
      (*xg_event).crossing.time = time as c_uint;
      (*xg_event).crossing.focus = FALSE;
      (*xg_event).crossing.state = state;
      (*xg_event).crossing.detail = (if nonlinear_p
				     { GDK_NOTIFY_NONLINEAR }
				     else { (if current_window == ancestor
					       { GDK_NOTIFY_INFERIOR }
					       else { GDK_NOTIFY_ANCESTOR }) });
      (*xg_event).crossing.mode = entry_crossing;
      (*xg_event).crossing.window = g_object_ref (current_window as gpointer) as *mut GdkWindow;

      gtk_main_do_event (xg_event);
      gdk_event_free (xg_event);
      g_object_unref (last_crossing as gpointer);

      return true;
    }

  return false;
}

/* C 1945-2105: the motion/crossing entry point.  C's `#ifdef`-split `if`/`else if`/`else`
   (C 1951-1961 and C 1990-1997) is the port's `match` on `event->type`, its middle arm gated by
   `have_xinput2` (Deviation 9); the `XEvent` union's members are read through casts to the member
   records (the file note above, and `docs/ports/include-xlib.md`).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_motion_or_crossing (view: *mut xwidget_view,
                                                     event: *const XEvent)  // C 1945-2105
{
  let mut xg_event: *mut GdkEvent;
  let model: *mut xwidget = XXWIDGET ((*view).model);
  let mut x: c_int;
  let mut y: c_int;
  let mut toplevel_x: c_int;
  let mut toplevel_y: c_int;
  let mut target: *mut GtkWidget;
  #[cfg (feature = "have_xinput2")]
  let mut xev: *mut XIEnterEvent = std::ptr::null_mut ();
  /* C reads members of the `XEvent` union; `include/xlib.rs`'s `XEvent` is opaque, so the port
     casts the pointer to the member record C names (the `src/xmenu.rs` 1086 shape).  */
  let xany: *const XAnyEvent = event as *const XAnyEvent;
  let xmotion: *const XMotionEvent = event as *const XMotionEvent;
  let xcrossing: *const XCrossingEvent = event as *const XCrossingEvent;
  let xcookie: *const XGenericEventCookie = event as *const XGenericEventCookie;

  if NILP ((*model).buffer)
    { return; }

  /* C 1951-1976's `#ifdef HAVE_XINPUT2`-split `if (event->type != GenericEvent) … else …` is
     the port's `match` on `event->type`, its `GenericEvent` arm gated by `have_xinput2`
     (Deviation 9): the C build without XI2 has one unconditional block, the XI2 build adds the
     `else`.  */
  match (*xany).r#type
    {
      #[cfg (feature = "have_xinput2")]
      GenericEvent =>
	{
	  eassert! ((*xcookie).evtype == XI_Enter
		    || (*xcookie).evtype == XI_Leave);

	  xev = (*xcookie).data as *mut XIEnterEvent;
	  xg_event = gdk_event_new (if (*xany).r#type == XI_Enter
				    { GDK_ENTER_NOTIFY }
				    else { GDK_LEAVE_NOTIFY });
	  toplevel_x = lrint ((*xev).event_x + (*view).clip_left as c_double) as c_int;
	  toplevel_y = lrint ((*xev).event_y + (*view).clip_top as c_double) as c_int;
	  target = find_widget_at_pos ((*model).widgetwindow_osr,
				       toplevel_x,
				       toplevel_y,
				       &raw mut x, &raw mut y, true, view);
	}
      _ =>
	{
	  xg_event = gdk_event_new (if (*xany).r#type == MotionNotify
				    { GDK_MOTION_NOTIFY }
				    else { (if (*xany).r#type == LeaveNotify
					      { GDK_LEAVE_NOTIFY }
					      else { GDK_ENTER_NOTIFY }) });
	  toplevel_x = (if (*xany).r#type == MotionNotify
			{ (*xmotion).x + (*view).clip_left }
			else { (*xcrossing).x + (*view).clip_left });
	  toplevel_y = (if (*xany).r#type == MotionNotify
			{ (*xmotion).y + (*view).clip_top }
			else { (*xcrossing).y + (*view).clip_top });
	  target = find_widget_at_pos ((*model).widgetwindow_osr,
				       toplevel_x, toplevel_y, &raw mut x, &raw mut y,
				       true, view);
	}
    }

  if target.is_null ()
    { target = (*model).widget_osr; }

  record_osr_embedder (view);
  (*xg_event).any.window = gtk_widget_get_window (target);
  g_object_ref ((*xg_event).any.window as gpointer); /* The window will be unrefed
							later by gdk_event_free.  */

  /* C 1977-2105's event-kind dispatch: `if (MotionNotify) … #ifdef XI2 else if (GenericEvent) …
     #endif else …` is the port's `match`, the middle arm gated (Deviation 9).  */
  match (*xany).r#type
    {
      MotionNotify =>
	{
	  if ! xw_maybe_synthesize_crossing (view, (*xg_event).any.window,
					     toplevel_x, toplevel_y,
					     XW_CROSSING_NONE, (*xmotion).time,
					     (*xmotion).state,
					     (if ! (*view).passive_grab.is_null ()
					      { GDK_CROSSING_GTK_GRAB }
					      else { GDK_CROSSING_NORMAL }),
					     GDK_CROSSING_NORMAL)
	    {
	      (*xg_event).motion.x = x as c_double;
	      (*xg_event).motion.y = y as c_double;
	      (*xg_event).motion.x_root = (*xmotion).x_root as c_double;
	      (*xg_event).motion.y_root = (*xmotion).y_root as c_double;
	      (*xg_event).motion.time = (*xmotion).time as c_uint;
	      (*xg_event).motion.state = (*xmotion).state;
	      (*xg_event).motion.device
		= find_suitable_pointer ((*view).frame as *mut frame, false);
	    }
	  else
	    {
	      gdk_event_free (xg_event);
	      return;
	    }
	}
      #[cfg (feature = "have_xinput2")]
      GenericEvent =>
	{
	  (*xg_event).crossing.x = x as c_double;
	  (*xg_event).crossing.y = y as c_double;
	  (*xg_event).crossing.x_root = (*xev).root_x as c_double;
	  (*xg_event).crossing.y_root = (*xev).root_y as c_double;
	  (*xg_event).crossing.time = (*xev).time as c_uint;
	  (*xg_event).crossing.focus = (*xev).focus;
	  (*xg_event).crossing.mode = (*xev).mode;
	  (*xg_event).crossing.detail = xi_translate_notify_detail ((*xev).detail);
	  (*xg_event).crossing.state = (*xev).mods.effective;

	  if (*xev).buttons.mask_len != 0
	    {
	      if XIMaskIsSet! ((*xev).buttons.mask, 1) != 0
		{ (*xg_event).crossing.state |= GDK_BUTTON1_MASK as c_uint; }
	      if XIMaskIsSet! ((*xev).buttons.mask, 2) != 0
		{ (*xg_event).crossing.state |= GDK_BUTTON2_MASK as c_uint; }
	      if XIMaskIsSet! ((*xev).buttons.mask, 3) != 0
		{ (*xg_event).crossing.state |= GDK_BUTTON3_MASK as c_uint; }
	    }

	  if ! (*view).passive_grab.is_null ()
	      || xw_maybe_synthesize_crossing (view, (*xg_event).any.window,
					       toplevel_x, toplevel_y,
					       (if (*xev).r#type == XI_Enter
						{ XW_CROSSING_ENTERED }
						else { XW_CROSSING_LEFT }),
					       (*xev).time, (*xg_event).crossing.state,
					       (if ! (*view).passive_grab.is_null ()
						{ GDK_CROSSING_GTK_GRAB }
						else { GDK_CROSSING_NORMAL }),
					       GDK_CROSSING_NORMAL)
	    {
	      gdk_event_free (xg_event);
	      return;
	    }

	  gdk_event_set_device (xg_event,
				find_suitable_pointer ((*view).frame as *mut frame, false));
	}
      _ =>
	{
	  if ! (*view).passive_grab.is_null ()
	      || xw_maybe_synthesize_crossing (view, (*xg_event).any.window,
					       toplevel_x, toplevel_y,
					       (if (*xany).r#type == EnterNotify
						{ XW_CROSSING_ENTERED }
						else { XW_CROSSING_LEFT }),
					       (*xcrossing).time,
					       (*xcrossing).state,
					       (if ! (*view).passive_grab.is_null ()
						{ GDK_CROSSING_GTK_GRAB }
						else { GDK_CROSSING_NORMAL }),
					       GDK_CROSSING_NORMAL)
	    {
	      gdk_event_free (xg_event);
	      return;
	    }

	  (*xg_event).crossing.detail = min! (5, (*xcrossing).detail);
	  (*xg_event).crossing.time = (*xcrossing).time as c_uint;
	  (*xg_event).crossing.x = x as c_double;
	  (*xg_event).crossing.y = y as c_double;
	  (*xg_event).crossing.x_root = (*xcrossing).x_root as c_double;
	  (*xg_event).crossing.y_root = (*xcrossing).y_root as c_double;
	  (*xg_event).crossing.focus = (*xcrossing).focus;
	  gdk_event_set_device (xg_event,
				find_suitable_pointer ((*view).frame as *mut frame, false));
	}
    }

  gtk_main_do_event (xg_event);
  gdk_event_free (xg_event);
}
/* C 2107: `#endif /* HAVE_X_WINDOWS */` — the HAVE_X_WINDOWS arm ends here.  */

/* C 2110-2133: `synthesize_focus_in_event` (inside the C 739 `USE_GTK` block).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn synthesize_focus_in_event (offscreen_window: *mut GtkWidget)  // C 2110-2133
{
  let mut wnd: *mut GdkWindow;
  let mut focus_event: *mut GdkEvent;

  if gtk_widget_get_realized (offscreen_window) == 0
    { gtk_widget_realize (offscreen_window); }

  wnd = gtk_widget_get_window (offscreen_window);

  focus_event = gdk_event_new (GDK_FOCUS_CHANGE);
  (*focus_event).focus_change.window = wnd;
  (*focus_event).focus_change.r#in = TRUE;

  if FRAME_WINDOW_P (SELECTED_FRAME ())
    { gdk_event_set_device (focus_event,
			    find_suitable_pointer (SELECTED_FRAME (), false)); }

  g_object_ref (wnd as gpointer);

  gtk_main_do_event (focus_event);
  gdk_event_free (focus_event);
}

/* C 2135-2147: `#ifdef HAVE_X_WINDOWS`.  `xwidget_view_from_window` is the header's `extern`
   definition (xwidget.h 209), so it carries `#[unsafe(no_mangle)]`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_view_from_window (wdesc: Window) -> *mut xwidget_view  // C 2137-2145
{
  let key: Lisp_Object = make_fixnum (wdesc as EMACS_INT);
  let xwv: Lisp_Object = Fgethash (key, x_window_to_xwv_map, Qnil ());

  if NILP (xwv)
    { return std::ptr::null_mut (); }

  return XXWIDGET_VIEW (xwv);
}

/// C 2150-2167: the two arms C 2153's `#ifdef HAVE_X_WINDOWS` splits.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_show_view (xv: *mut xwidget_view)  // C 2150-2167
{
  (*xv).hidden = false;
  #[cfg (feature = "have_x_windows")]
  {
    XMoveWindow ((*xv).dpy, (*xv).wdesc,
		 (*xv).x + (*xv).clip_left,
		 (*xv).y + (*xv).clip_top);
    XMapWindow ((*xv).dpy, (*xv).wdesc);
    XFlush ((*xv).dpy);
  }
  #[cfg (not (feature = "have_x_windows"))]
  {
    gtk_fixed_move (GTK_FIXED! (FRAME_GTK_WIDGET! ((*xv).frame as *mut frame)),
		    (*xv).widget, (*xv).x + (*xv).clip_left,
		    (*xv).y + (*xv).clip_top);
    gtk_widget_show_all ((*xv).widget);
  }
}

/// C 2169-2178: the same split for hiding.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_hide_view (xv: *mut xwidget_view)  // C 2169-2178
{
  (*xv).hidden = true;
  #[cfg (feature = "have_x_windows")]
  {
    XUnmapWindow ((*xv).dpy, (*xv).wdesc);
    XFlush ((*xv).dpy);
  }
  #[cfg (not (feature = "have_x_windows"))]
  {
    gtk_widget_hide ((*xv).widget);
  }
}

/* C 2180-2242: `#ifndef HAVE_PGTK`'s `xv_do_draw` and `#else`'s `xwidget_view_draw_cb`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
unsafe fn xv_do_draw (xw: *mut xwidget_view, w: *mut xwidget)  // C 2182-2210
{
  let mut wnd: *mut GtkOffscreenWindow;
  let mut surface: *mut cairo_surface_t;

  if (*xw).just_resized != 0
    { return; }

  if NILP ((*w).buffer)
    {
      XClearWindow ((*xw).dpy, (*xw).wdesc);
      return;
    }

  block_input ();
  wnd = GTK_OFFSCREEN_WINDOW! ((*w).widgetwindow_osr);
  surface = gtk_offscreen_window_get_surface (wnd);

  cairo_save ((*xw).cr_context);
  if ! surface.is_null ()
    {
      cairo_translate ((*xw).cr_context, -((*xw).clip_left as c_double),
		       -((*xw).clip_top as c_double));
      cairo_set_source_surface ((*xw).cr_context, surface, 0 as c_double, 0 as c_double);
      cairo_set_operator ((*xw).cr_context, CAIRO_OPERATOR_SOURCE);
      cairo_paint ((*xw).cr_context);
    }
  cairo_restore ((*xw).cr_context);

  unblock_input ();
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
unsafe extern "C" fn xwidget_view_draw_cb (widget: *mut GtkWidget, cr: *mut cairo_t,
                                           data: gpointer)  // C 2214-2240
{
  let view: *mut xwidget_view = data as *mut xwidget_view;
  let w: *mut xwidget = XXWIDGET ((*view).model);
  let mut wnd: *mut GtkOffscreenWindow;
  let mut surface: *mut cairo_surface_t;

  if NILP ((*w).buffer)
    { return; }

  block_input ();
  wnd = GTK_OFFSCREEN_WINDOW! ((*w).widgetwindow_osr);
  surface = gtk_offscreen_window_get_surface (wnd);

  cairo_save (cr);
  if ! surface.is_null ()
    {
      cairo_translate (cr, -((*view).clip_left as c_double),
		       -((*view).clip_top as c_double));
      cairo_set_source_surface (cr, surface, 0 as c_double, 0 as c_double);
      cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);
      cairo_paint (cr);
    }
  cairo_restore (cr);

  unblock_input ();
}

/* C 2247-2270: the off-screen master view's damage handler, with C 2258's
   `#ifdef HAVE_X_WINDOWS` arm.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn offscreen_damage_event (widget: *mut GtkWidget, event: *mut GdkEvent,
                                             xwidget: gpointer) -> gboolean  // C 2247-2270
{
  block_input ();

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
	{
	  let view: *mut xwidget_view = XXWIDGET_VIEW (XCAR (tail));
	  #[cfg (feature = "have_x_windows")]
	  if (*view).wdesc != 0 && XXWIDGET ((*view).model) == xwidget as *mut xwidget
	    { xv_do_draw (view, XXWIDGET ((*view).model)); }
	  #[cfg (not (feature = "have_x_windows"))]
	  gtk_widget_queue_draw ((*view).widget);
	}
      tail = XCDR (tail);
    }

  unblock_input ();

  return FALSE;
}

/* C 2272-2280: `#ifdef HAVE_X_WINDOWS` around `xwidget_expose`, the header's `extern` definition
   (xwidget.h 210).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_expose (xv: *mut xwidget_view)  // C 2274-2279
{
  let xw: *mut xwidget = XXWIDGET ((*xv).model);

  xv_do_draw (xv, xw);
}
/* C 2281: `#endif /* USE_GTK */` — the C 739 block ends here.  */

/* C 2284-2331: the three event-string store helpers, all of them the header's `extern`
   definitions (xwidget.h 193-204), so each carries `#[unsafe(no_mangle)]`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_xwidget_event_string (xw: *mut xwidget, eventname: *const c_char,
                                                     eventstr: *const c_char)  // C 2284-2296
{
  let mut event: input_event = std::mem::zeroed ();
  let mut xwl: Lisp_Object;
  XSETXWIDGET! (xwl, xw);
  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, XWIDGET_EVENT);
  event.frame_or_window = Qnil ();
  event.arg = list3 (intern (eventname), xwl, build_string (eventstr));
  kbd_buffer_store_event (&raw mut event as *mut c_void);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_xwidget_download_callback_event (xw: *mut xwidget,
                                                                url: *const c_char,
                                                                mimetype: *const c_char,
                                                                filename: *const c_char)  // C 2298-2316
{
  let mut event: input_event = std::mem::zeroed ();
  let mut xwl: Lisp_Object;
  XSETXWIDGET! (xwl, xw);
  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, XWIDGET_EVENT);
  event.frame_or_window = Qnil ();
  event.arg = list5 (Qdownload_callback (),
                     xwl,
                     build_string (url),
                     build_string (mimetype),
                     build_string (filename));
  kbd_buffer_store_event (&raw mut event as *mut c_void);
}

#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_xwidget_js_callback_event (xw: *mut xwidget, proc_: Lisp_Object,
                                                          argument: Lisp_Object)  // C 2318-2331
{
  let mut event: input_event = std::mem::zeroed ();
  let mut xwl: Lisp_Object;
  XSETXWIDGET! (xwl, xw);
  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, XWIDGET_EVENT);
  event.frame_or_window = Qnil ();
  event.arg = list4 (Qjavascript_callback (), xwl, proc_, argument);
  kbd_buffer_store_event (&raw mut event as *mut c_void);
}

/* C 2333-2716: `#ifdef USE_GTK` - the display-event store and the six WebKit callbacks.  */
/// C 2335-2349: the display-event store (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn store_xwidget_display_event (xw: *mut xwidget, src: *mut xwidget)  // C 2335-2349
{
  let mut evt: input_event = std::mem::zeroed ();
  let mut val: Lisp_Object;
  let mut src_val: Lisp_Object;

  XSETXWIDGET! (val, xw);
  XSETXWIDGET! (src_val, src);
  EVENT_INIT (&raw mut evt);
  input_event_bits! (kind, &raw mut evt, XWIDGET_DISPLAY_EVENT);
  evt.frame_or_window = Qnil ();
  evt.arg = list2 (val, src_val);
  kbd_buffer_store_event (&raw mut evt as *mut c_void);
}

/// C 2351-2379: the `"ready-to-show"` handler `webkit_create_cb_1` connects.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn webkit_ready_to_show (new_view: *mut WebKitWebView,
                                           user_data: gpointer)  // C 2351-2379
{
  let mut tem: Lisp_Object;
  let mut xw: *mut xwidget;
  let src: *mut xwidget;

  src = find_xwidget_for_offscreen_window (GDK_WINDOW! (user_data));

  tem = internal_xwidget_list;
  while CONSP (tem)
    {
      if XWIDGETP (XCAR (tem))
	{
	  xw = XXWIDGET (XCAR (tem));

	  if EQ ((*xw).r#type, Qwebkit ())
	      && WEBKIT_WEB_VIEW! ((*xw).widget_osr) == new_view
	    {
	      /* The source widget was destroyed before we had a
		 chance to display the new widget.  */
	      if src.is_null ()
		{ kill_xwidget (xw); }
	      else
		{ store_xwidget_display_event (xw, src); }
	    }
	}
      tem = XCDR (tem);
    }
}

/// C 2381-2405: the `"create"` handler's first half (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn webkit_create_cb_1 (webview: *mut WebKitWebView, xv: *mut xwidget) -> *mut GtkWidget  // C 2381-2405
{
  let mut related: Lisp_Object;
  let mut xwidget: Lisp_Object;
  let mut widget: *mut GtkWidget;

  XSETXWIDGET! (related, xv);
  xwidget = Fmake_xwidget (Qwebkit (), Qnil (), make_fixnum (0),
			   make_fixnum (0), Qnil (),
			   build_string (c" *detached xwidget buffer*".as_ptr ()),
			   related);

  if NILP (xwidget)
    { return std::ptr::null_mut (); }

  widget = (*XXWIDGET (xwidget)).widget_osr;

  g_signal_connect! (G_OBJECT! (widget), c"ready-to-show".as_ptr (),
		     G_CALLBACK! (webkit_ready_to_show),
		     gtk_widget_get_window ((*xv).widgetwindow_osr) as gpointer);

  return widget;
}

/// C 2407-2425: the `"create"` handler itself (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn webkit_create_cb (webview: *mut WebKitWebView,
                            nav_action: *mut WebKitNavigationAction,
                            user_data: gpointer) -> *mut GtkWidget  // C 2407-2425
{
  match webkit_navigation_action_get_navigation_type (nav_action)
    {
      WEBKIT_NAVIGATION_TYPE_OTHER =>
	{
	  return webkit_create_cb_1 (webview, user_data as *mut xwidget);
	}
      WEBKIT_NAVIGATION_TYPE_BACK_FORWARD
      | WEBKIT_NAVIGATION_TYPE_RELOAD
      | WEBKIT_NAVIGATION_TYPE_FORM_SUBMITTED
      | WEBKIT_NAVIGATION_TYPE_FORM_RESUBMITTED
      | WEBKIT_NAVIGATION_TYPE_LINK_CLICKED
      | _ =>
	{
	  return std::ptr::null_mut ();
	}
    }
}

/* C 2427-2451: the `"load-changed"` handler.  C's `switch` without `default` is the port's
   `match` with an empty `_` arm (C's own fall-through; Deviation 9), and the definition is
   non-`static`, so it carries `#[unsafe(no_mangle)]`.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webkit_view_load_changed_cb (webkitwebview: *mut WebKitWebView,
                                                      load_event: WebKitLoadEvent,
                                                      data: gpointer)  // C 2427-2451
{
  let xw: *mut xwidget = g_object_get_data (G_OBJECT! (webkitwebview),
					    XG_XWIDGET.as_ptr ()) as *mut xwidget;

  match load_event
    {
      WEBKIT_LOAD_FINISHED =>
	{
	  store_xwidget_event_string (xw, c"load-changed".as_ptr (),
				      c"load-finished".as_ptr ());
	}
      WEBKIT_LOAD_STARTED =>
	{
	  store_xwidget_event_string (xw, c"load-changed".as_ptr (),
				      c"load-started".as_ptr ());
	}
      WEBKIT_LOAD_REDIRECTED =>
	{
	  store_xwidget_event_string (xw, c"load-changed".as_ptr (),
				      c"load-redirected".as_ptr ());
	}
      WEBKIT_LOAD_COMMITTED =>
	{
	  store_xwidget_event_string (xw, c"load-changed".as_ptr (),
				      c"load-committed".as_ptr ());
	}
      _ => {}
    }
}

/* Recursively convert a JavaScript value to a Lisp value. */
/// C 2453-2518: `webkit_js_to_lisp` (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn webkit_js_to_lisp (value: *mut JSCValue) -> Lisp_Object  // C 2453-2518
{
  if jsc_value_is_string (value) != 0
    {
      let mut str_value: *mut gchar = jsc_value_to_string (value);
      let ret: Lisp_Object = build_string (str_value);
      g_free (str_value as gpointer);

      return ret;
    }
  else if jsc_value_is_boolean (value) != 0
    {
      return (if jsc_value_to_boolean (value) != 0 { Qt () } else { Qnil () });
    }
  else if jsc_value_is_number (value) != 0
    {
      return make_fixnum (jsc_value_to_int32 (value) as EMACS_INT);
    }
  else if jsc_value_is_array (value) != 0
    {
      let len: *mut JSCValue = jsc_value_object_get_property (value, c"length".as_ptr ());
      let dlen: c_int = jsc_value_to_int32 (len);

      let mut obj: Lisp_Object;
      if ! (0 <= dlen && dlen < G_MAXINT32)
	{ memory_full_up (); }

      let n: ptrdiff_t = dlen as ptrdiff_t;
      let p: *mut Lisp_Vector = allocate_nil_vector (n);

      let mut i: ptrdiff_t = 0;
      while i < n
	{
	  FAM_ELEM! ((*p).contents, i) =
	    webkit_js_to_lisp (jsc_value_object_get_property_at_index (value, i as guint));
	  i += 1;
	}
      XSETVECTOR! (obj, p);
      return obj;
    }
  else if jsc_value_is_object (value) != 0
    {
      let properties_names: *mut *mut gchar = jsc_value_object_enumerate_properties (value);
      let mut n: guint = g_strv_length (properties_names);

      let mut obj: Lisp_Object;
      if PTRDIFF_MAX < n as isize
	{ memory_full (n as usize); }
      let p: *mut Lisp_Vector = allocate_nil_vector (n as isize);

      let mut i: ptrdiff_t = 0;
      while i < n as isize
	{
	  let name: *const c_char = *properties_names.offset (i as isize);
	  let property: *mut JSCValue = jsc_value_object_get_property (value, name);

	  FAM_ELEM! ((*p).contents, i) =
	    Fcons (build_string (name), webkit_js_to_lisp (property));
	  i += 1;
	}

      g_strfreev (properties_names);

      XSETVECTOR! (obj, p);
      return obj;
    }

  return Qnil ();
}

/// C 2520-2559: the `run-javascript` completion callback (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
/* C 3244 passes this function as `GAsyncReadyCallback`, whose type is
   `unsafe extern "C" fn (...)`; C's `static void webkit_javascript_finished_cb (...)` has that
   type, so the port spells it `extern "C"` (the `init_alloc_once_for_pdumper` correction,
   `femacs/src/alloc.rs` 7994).  */
unsafe extern "C" fn webkit_javascript_finished_cb (webview: *mut GObject,
						    result: *mut GAsyncResult,
						    arg: gpointer)  // C 2520-2559
{
  let mut error: *mut GError = std::ptr::null_mut ();
  let xw: *mut xwidget = g_object_get_data (G_OBJECT! (webview),
					    XG_XWIDGET.as_ptr ()) as *mut xwidget;

  let script_idx: ptrdiff_t = arg as intptr_t as ptrdiff_t;
  let script_callback: Lisp_Object = AREF ((*xw).script_callbacks, script_idx);
  ASET ((*xw).script_callbacks, script_idx, Qnil ());
  if ! NILP (script_callback)
    { xfree (xmint_pointer (XCAR (script_callback))); }

  let js_result: *mut WebKitJavascriptResult =
    webkit_web_view_run_javascript_finish
    (WEBKIT_WEB_VIEW! (webview), result, &raw mut error);

  if js_result.is_null ()
    {
      if ! error.is_null ()
	{ g_error_free (error); }
      return;
    }

  if ! NILP (script_callback) && ! NILP (XCDR (script_callback))
    {
      let value: *mut JSCValue = webkit_javascript_result_get_js_value (js_result);

      let lisp_value: Lisp_Object = webkit_js_to_lisp (value);

      /* Register an xwidget event here, which then runs the callback.
	 This ensures that the callback runs in sync with the Emacs
	 event loop.  */
      store_xwidget_js_callback_event (xw, XCDR (script_callback), lisp_value);
    }

  webkit_javascript_result_unref (js_result);
}

/// C 2561-2574: the `"download-started"` handler.  Non-`static`, so `#[unsafe(no_mangle)]`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn webkit_download_cb (webkitwebcontext: *mut WebKitWebContext,
                                             arg1: *mut WebKitDownload,
                                             data: gpointer) -> gboolean  // C 2561-2574
{
  let view: *mut WebKitWebView = webkit_download_get_web_view (arg1);
  let request: *mut WebKitURIRequest = webkit_download_get_request (arg1);
  let xw: *mut xwidget = g_object_get_data (G_OBJECT! (view),
					    XG_XWIDGET.as_ptr ()) as *mut xwidget;

  store_xwidget_event_string (xw, c"download-started".as_ptr (),
			      webkit_uri_request_get_uri (request));
  return FALSE;
}

/// C 2576-2645: the `"decide-policy"` handler (static).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn webkit_decide_policy_cb (web_view: *mut WebKitWebView,
                                   decision: *mut WebKitPolicyDecision,
                                   type_: WebKitPolicyDecisionType,
                                   user_data: gpointer) -> gboolean  // C 2576-2645
{
  match type_
    {
      WEBKIT_POLICY_DECISION_TYPE_RESPONSE =>
	{
	  /* This function makes webkit send a download signal for all unknown
	     mime types.  TODO: Defer the decision to Lisp, so that it's
	     possible to make Emacs handle mime text for instance.  */
	  let response: *mut WebKitResponsePolicyDecision =
	    WEBKIT_RESPONSE_POLICY_DECISION! (decision);
	  if webkit_response_policy_decision_is_mime_type_supported (response) == 0
	    {
	      webkit_policy_decision_download (decision);
	      return TRUE;
	    }
	  else
	    { return FALSE; }
	}
      WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION =>
	{
	  let navigation_decision: *mut WebKitNavigationPolicyDecision =
	    WEBKIT_NAVIGATION_POLICY_DECISION! (decision);
	  let navigation_action: *mut WebKitNavigationAction =
	    webkit_navigation_policy_decision_get_navigation_action (navigation_decision);
	  let request: *mut WebKitURIRequest =
	    webkit_navigation_action_get_request (navigation_action);
	  let mut newview: *mut WebKitWebView;
	  let xw: *mut xwidget = g_object_get_data (G_OBJECT! (web_view),
						    XG_XWIDGET.as_ptr ()) as *mut xwidget;
	  let mut val: Lisp_Object;
	  let mut new_xwidget: Lisp_Object;

	  XSETXWIDGET! (val, xw);

	  new_xwidget = Fmake_xwidget (Qwebkit (), Qnil (), make_fixnum (0),
				       make_fixnum (0), Qnil (),
				       build_string (c" *detached xwidget buffer*".as_ptr ()),
				       val);

	  if NILP (new_xwidget)
	    { return FALSE; }

	  newview = WEBKIT_WEB_VIEW! ((*XXWIDGET (new_xwidget)).widget_osr);
	  webkit_web_view_load_request (newview, request);

	  store_xwidget_display_event (XXWIDGET (new_xwidget), xw);
	  return TRUE;
	}
      WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION =>
	{
	  let navigation_decision: *mut WebKitNavigationPolicyDecision =
	    WEBKIT_NAVIGATION_POLICY_DECISION! (decision);
	  let navigation_action: *mut WebKitNavigationAction =
	    webkit_navigation_policy_decision_get_navigation_action (navigation_decision);
	  let request: *mut WebKitURIRequest =
	    webkit_navigation_action_get_request (navigation_action);

	  let xw: *mut xwidget = g_object_get_data (G_OBJECT! (web_view),
						    XG_XWIDGET.as_ptr ()) as *mut xwidget;
	  store_xwidget_event_string (xw, c"decide-policy".as_ptr (),
				      webkit_uri_request_get_uri (request));
	  return FALSE;
	}
      _ =>
	{
	  return FALSE;
	}
    }
}

/// C 2647-2714: the `"script-dialog"` handler (static).  `entry` is C's uninitialised local,
/// assigned before its only reads; Rust seeds it (PORTING.md 0.1-13).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn webkit_script_dialog_cb (webview: *mut WebKitWebView,
                                   script_dialog: *mut WebKitScriptDialog,
                                   user: gpointer) -> gboolean  // C 2647-2714
{
  let f: *mut frame = SELECTED_FRAME ();
  let mut type_: WebKitScriptDialogType;
  let widget: *mut GtkWidget;
  let mut dialog: *mut GtkWidget;
  let mut entry: *mut GtkWidget = std::ptr::null_mut ();
  let mut content_area: *mut GtkWidget;
  let mut box_: *mut GtkWidget;
  let mut label: *mut GtkWidget;
  let content: *const gchar;
  let mut message: *const gchar;
  let mut result: gint;

  /* Return TRUE to prevent WebKit from showing the default script
     dialog in the offscreen window, which runs a nested main loop
     Emacs can't respond to, and as such can't pass X events to.  */
  if ! FRAME_WINDOW_P (f)
    { return TRUE; }

  type_ = webkit_script_dialog_get_dialog_type (script_dialog);
  widget = FRAME_GTK_OUTER_WIDGET! (f);
  content = webkit_script_dialog_get_message (script_dialog);

  if type_ == WEBKIT_SCRIPT_DIALOG_ALERT
    {
      dialog = gtk_dialog_new_with_buttons (c"Alert".as_ptr (), GTK_WINDOW! (widget),
					    GTK_DIALOG_MODAL,
					    c"Dismiss".as_ptr (), 1, std::ptr::null_mut::<c_void> ());
    }
  else
    {
      dialog = gtk_dialog_new_with_buttons (c"Question".as_ptr (), GTK_WINDOW! (widget),
					    GTK_DIALOG_MODAL,
					    c"OK".as_ptr (), 0, c"Cancel".as_ptr (), 1,
					    std::ptr::null_mut::<c_void> ());
    }

  box_ = gtk_box_new (GTK_ORIENTATION_VERTICAL, 8);
  label = gtk_label_new (content);
  content_area = gtk_dialog_get_content_area (GTK_DIALOG! (dialog));
  gtk_container_add (GTK_CONTAINER! (content_area), box_);

  gtk_widget_show (box_);
  gtk_widget_show (label);

  gtk_box_pack_start (GTK_BOX! (box_), label, TRUE, TRUE, 0);

  if type_ == WEBKIT_SCRIPT_DIALOG_PROMPT
    {
      entry = gtk_entry_new ();
      message = webkit_script_dialog_prompt_get_default_text (script_dialog);

      gtk_widget_show (entry);
      gtk_entry_set_text (GTK_ENTRY! (entry), message);
      gtk_box_pack_end (GTK_BOX! (box_), entry, TRUE, TRUE, 0);
    }

  result = gtk_dialog_run (GTK_DIALOG! (dialog));

  if type_ == WEBKIT_SCRIPT_DIALOG_CONFIRM
      || type_ == WEBKIT_SCRIPT_DIALOG_BEFORE_UNLOAD_CONFIRM
    { webkit_script_dialog_confirm_set_confirmed (script_dialog, (result == 0) as gboolean); }

  if type_ == WEBKIT_SCRIPT_DIALOG_PROMPT
    {
      webkit_script_dialog_prompt_set_text (script_dialog,
					    gtk_entry_get_text (GTK_ENTRY! (entry)));
    }

  gtk_widget_destroy (GTK_WIDGET! (dialog));

  return TRUE;
}
/* C 2716: `#endif /* USE_GTK */` — the second `USE_GTK` block ends here; C 2718's
   `xwidget_init_view` is the next lane's first definition.  */

/* Three names lisp.h declares and this slice's bodies call, which the mirrors this unit reaches
   do not carry yet (P-72 rule 4: a mirror must reach what it needs through its own `include!`s).
   Each line names the C header it comes from, and the ledger's Owed list records them.  */
/// C: lisp.h 4245: `extern Lisp_Object larger_vector (Lisp_Object, ptrdiff_t, ptrdiff_t);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn larger_vector (vec: Lisp_Object, eltcnt: ptrdiff_t, eltbytes: ptrdiff_t)
    -> Lisp_Object;
}
/// C: lisp.h 5553: `extern char *xlispstrdup (Lisp_Object) ATTRIBUTE_MALLOC;`
/// (alloc.c 842 defines it).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn xlispstrdup (string: Lisp_Object) -> *mut c_char;
}
/// C: xdisp.c 17073: `redisplay (void)` (dispextern.h declares it; the `keyboard.rs` 680
/// `unsafe extern "C" { fn redisplay (); }` shape).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn redisplay ();
}
/// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (eval.c defines it; the
/// `androidselect.rs` 145-152 shape - lisp.h also spells it as a macro at 3772, and the port's
/// `alloc.rs` 142-146 carries that form for its own arm).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn FUNCTIONP (object: Lisp_Object) -> bool;
}

/* C 2716: `#endif /* USE_GTK */`.  This slice is C 2717-2790 and then C 3042-3116; C 2791-3041
   (C 2793's `x_draw_xwidget_glyph_string`) is the unported remainder between them and C 3117-4304
   the remainder after them - `docs/ports/xwidget.md`'s C line-range table carries the rows.  */

/* Initializes and does initial placement of an xwidget view on screen.  */

/// C: 2720-2723: `static struct xwidget_view *` / `xwidget_init_view (struct xwidget *xww,
/// struct glyph_string *s, int x, int y)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_init_view (xww: *mut xwidget, s: *mut glyph_string, x: c_int, y: c_int)
  -> *mut xwidget_view
{
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  if !xg_gtk_initialized
    { error (c"xwidget_init_view: GTK has not been initialized".as_ptr ()); }

  let xv: *mut xwidget_view = allocate_xwidget_view ();
  let mut val: Lisp_Object;

  XSETXWIDGET_VIEW! (val, xv);
  internal_xwidget_view_list = Fcons (val, internal_xwidget_view_list);
  Vxwidget_view_list = Fcopy_sequence (internal_xwidget_view_list);

  XSETWINDOW! ((*xv).w, (*s).w);
  XSETXWIDGET! ((*xv).model, xww);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
  {
  /* C 2752's `None` is X11/X.h 115's `#define None 0L`; the mirror carries it in
     `include/xlib.rs`'s `x11_constants` (reached through `include/xterm.rs`), and the unit
     imports it by name the way C 614's own `xv->wdesc = None` does at line 3445.  */
  use x11_constants::None;
  (*xv).dpy = FRAME_X_DISPLAY! ((*s).f);

  (*xv).x = x;
  (*xv).y = y;

  (*xv).clip_left = 0;
  (*xv).clip_right = (*xww).width;
  (*xv).clip_top = 0;
  (*xv).clip_bottom = (*xww).height;

  (*xv).wdesc = None as Window;
  /* `xwidget_view.frame` is the mirror's `*mut c_void` (P-55: the record cannot name `struct
     frame`), and `just_resized` is the mirror's `c_int` for C's `bool` - so C's `false` is 0
     (the read side, `xwidget.c` 3933, is the port's `!= 0`).  */
  (*xv).frame = (*s).f as *mut c_void;
  (*xv).cursor = (*(FRAME_OUTPUT_DATA! ((*s).f) as *mut x_output)).nontext_cursor;
  (*xv).just_resized = 0;
  (*xv).last_crossing_window = core::ptr::null_mut ();
  (*xv).passive_grab = core::ptr::null_mut ();
  }

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
  {
  (*xv).dpyinfo = FRAME_DISPLAY_INFO! ((*s).f);
  (*xv).widget = gtk_drawing_area_new ();
  gtk_widget_set_app_paintable ((*xv).widget, TRUE);
  gtk_widget_add_events ((*xv).widget, GDK_ALL_EVENTS_MASK);
  gtk_container_add (GTK_CONTAINER! (FRAME_GTK_WIDGET! ((*s).f)), (*xv).widget);

  g_signal_connect! ((*xv).widget, c"draw".as_ptr (),
		     G_CALLBACK! (xwidget_view_draw_cb), xv as *mut c_void);
  g_signal_connect! ((*xv).widget, c"event".as_ptr (),
		     G_CALLBACK! (xw_forward_event_from_view), xv as *mut c_void);

  g_object_set_data (G_OBJECT! ((*xv).widget), XG_XWIDGET_VIEW.as_ptr (), xv as *mut c_void);

  (*xv).x = x;
  (*xv).y = y;

  (*xv).clip_left = 0;
  (*xv).clip_right = (*xww).width;
  (*xv).clip_top = 0;
  (*xv).clip_bottom = (*xww).height;

  (*xv).frame = (*s).f as *mut c_void;
  (*xv).cursor = cursor_for_hit ((*xww).hit_result, (*s).f);
  (*xv).just_resized = 0;
  }

  /* C 2784's `#elif defined NS_IMPL_COCOA` arm.  The port keeps the file gate on every item
     (P-86 ⑧), so the arm carries both names - a recorded deviation.  `include/nsxwidget.rs` is
     the Cocoa lane's mirror, so the two are declared at the use site (P-72 rule 4).  */
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  /// C: nsxwidget.h 67-70: `void nsxwidget_init_view (struct xwidget_view *xv, struct xwidget
  /// *xww, struct glyph_string *s, int x, int y);` - the third pointer is `struct glyph_string`,
  /// which no arm but the window-system ones can name, so the declaration takes `*mut c_void`
  /// (P-72 rule 3) and the call site passes C's own pointer.
  unsafe extern "C" {
    pub fn nsxwidget_init_view (xv: *mut xwidget_view, xww: *mut xwidget, s: *mut c_void,
				x: c_int, y: c_int);
  }
  /// C: nsxwidget.h 75-76: `void nsxwidget_resize_view (struct xwidget_view *xv, int widget, int
  /// height);` - C 2786 spells the second argument `xww->width` and the third `xww->height`.
  unsafe extern "C" {
    pub fn nsxwidget_resize_view (xv: *mut xwidget_view, widget: c_int, height: c_int);
  }
  nsxwidget_init_view (xv, xww, s as *mut c_void, x, y);
  nsxwidget_resize_view (xv, (*xww).width, (*xww).height);
  }

  return xv;
}

/* C 2791-3041 is the unported remainder: C 2793's `x_draw_xwidget_glyph_string`.  */

/* C 3042-3044:
   `#define CHECK_WEBKIT_WIDGET(xw)						\
      if (NILP (xw->buffer) || !EQ (xw->type, Qwebkit))			\
        error ("Not a WebKit widget")`  */

/// C: 3042-3044: the `CHECK_WEBKIT_WIDGET` macro.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! CHECK_WEBKIT_WIDGET {
  ($xw:expr) => {
    if NILP ((*$xw).buffer) || !EQ ((*$xw).r#type, Qwebkit ())
      { error (c"Not a WebKit widget".as_ptr ()); }
  };
}

/* Macro that checks xwidget hold webkit web view first.  */

/// C: 3047-3050: `#define WEBKIT_FN_INIT() CHECK_LIVE_XWIDGET (xwidget); struct xwidget *xw =
/// XXWIDGET (xwidget); CHECK_WEBKIT_WIDGET (xw)` - C's macro *declares* `xw` at its expansion
/// site, which Rust macro hygiene cannot hand out (the `ITREE_FOREACH` class in
/// `docs/ports/macro-body-notes.tsv`), so the port's form runs C's three statements and answers
/// `xw`; each of the three callers binds it with C's name (`let xw = WEBKIT_FN_INIT! (xwidget);`).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! WEBKIT_FN_INIT {
  ($xwidget:expr) => {{
    CHECK_LIVE_XWIDGET ($xwidget);
    let xw: *mut xwidget = XXWIDGET ($xwidget);
    CHECK_WEBKIT_WIDGET! (xw);
    xw
  }};
}

/// C: 3052-3055: `DEFUN ("xwidget-webkit-uri", Fxwidget_webkit_uri, Sxwidget_webkit_uri, 1, 1,
/// 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-uri", Fxwidget_webkit_uri, Sxwidget_webkit_uri, 1, 1, 0,
       doc: "Get the current URL of XWIDGET webkit.",
       (xwidget: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  let wkwv: *mut WebKitWebView = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  let uri: *const gchar = webkit_web_view_get_uri (wkwv);
  if uri.is_null ()
    { return build_string (c"".as_ptr ()); }
  return build_string (uri);
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  return nsxwidget_webkit_uri (xw);
  }
});

/// C 3065-3067's `#elif defined NS_IMPL_COCOA` arm of `F_xwidget_webkit_uri`: C's `#elif` negates
/// its own file gate, and the port keeps the gate on every item (P-86 ⑧, the recorded shape).
/// C: nsxwidget.h 35: `Lisp_Object nsxwidget_webkit_uri (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_uri (xw: *mut xwidget) -> Lisp_Object;
}

/// C: 3070-3073: `DEFUN ("xwidget-webkit-title", Fxwidget_webkit_title, Sxwidget_webkit_title,
/// 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-title", Fxwidget_webkit_title, Sxwidget_webkit_title, 1, 1, 0,
       doc: "Get the current title of XWIDGET webkit.",
       (xwidget: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  let wkwv: *mut WebKitWebView = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  let title: *const gchar = webkit_web_view_get_title (wkwv);

  return build_string (if title.is_null () { c"".as_ptr () } else { title });
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  return nsxwidget_webkit_title (xw);
  }
});

/// C 3082-3084's `#elif defined NS_IMPL_COCOA` arm of `F_xwidget_webkit_title`.
/// C: nsxwidget.h 36: `Lisp_Object nsxwidget_webkit_title (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_title (xw: *mut xwidget) -> Lisp_Object;
}

/// C: 3087-3091: `DEFUN ("xwidget-webkit-estimated-load-progress",
/// Fxwidget_webkit_estimated_load_progress, Sxwidget_webkit_estimated_load_progress, 1, 1, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-estimated-load-progress", Fxwidget_webkit_estimated_load_progress,
       Sxwidget_webkit_estimated_load_progress, 1, 1, 0,
       doc: "Get the estimated load progress of XWIDGET, a WebKit widget.\nReturn a value ranging from 0.0 to 1.0, based on how close XWIDGET\nis to completely loading its page.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  let mut webview: *mut WebKitWebView;
  let mut value: c_double;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  block_input ();
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  value = webkit_web_view_get_estimated_load_progress (webview);
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  value = nsxwidget_webkit_estimated_load_progress (xw);
  }

  unblock_input ();

  return make_float (value);
});

/// C 3108-3110's `#elif defined NS_IMPL_COCOA` arm of `F_xwidget_webkit_estimated_load_progress`.
/// C: nsxwidget.h 39: `double nsxwidget_webkit_estimated_load_progress (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_estimated_load_progress (xw: *mut xwidget) -> c_double;
}

/* C 3117's `DEFUN ("xwidget-webkit-goto-uri", …)` on: slice 4.  The C line-range table's row
   3117-4304 is still the owed remainder, and this slice lands its first six items (C 3117-3300);
   C 3301 on is the next slice's.  */

/// C 3117-3120: `DEFUN ("xwidget-webkit-goto-uri", Fxwidget_webkit_goto_uri,
/// Sxwidget_webkit_goto_uri, 2, 2, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-goto-uri", Fxwidget_webkit_goto_uri, Sxwidget_webkit_goto_uri, 2, 2, 0,
       doc: "Make the xwidget webkit instance referenced by XWIDGET browse URI.",
       (xwidget: Lisp_Object, uri: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);
  /* C 3125 assigns its own `uri` parameter (`uri = ENCODE_FILE (uri);`); a Rust parameter is
     not a place, so the port rebinds it (the `window.rs` 4267 shape).  */
  let mut uri: Lisp_Object = uri;
  CHECK_STRING (uri);
  uri = ENCODE_FILE (uri);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  webkit_web_view_load_uri (WEBKIT_WEB_VIEW! ((*xw).widget_osr), SSDATA (uri));
  catch_child_signal ();
  }
  /* C 3130's `#elif defined NS_IMPL_COCOA` arm; the port keeps the file gate on every item
     (P-86 ⑧), so the arm carries the name - the same shape as slice 3's Cocoa arms.  */
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_webkit_goto_uri (xw, SSDATA (uri));
  }
  return Qnil ();
});

/// C 3129-3130's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 37: `void nsxwidget_webkit_goto_uri (struct xwidget *xw, const char *uri);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_goto_uri (xw: *mut xwidget, uri: *mut c_char);
}

/// C 3135-3142: `DEFUN ("xwidget-webkit-goto-history", Fxwidget_webkit_goto_history,
/// Sxwidget_webkit_goto_history, 2, 2, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-goto-history", Fxwidget_webkit_goto_history,
       Sxwidget_webkit_goto_history, 2, 2, 0,
       doc: "Make the XWIDGET webkit the REL-POSth element in load history.\n\nIf REL-POS is 0, the widget will be just reload the current element in\nhistory.  If REL-POS is more or less than 0, the widget will load the\nREL-POSth element around the current spot in the load history.",
       (xwidget: Lisp_Object, rel_pos: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);
  CHECK_FIXNUM (rel_pos);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  let wkwv: *mut WebKitWebView = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  let mut list: *mut WebKitBackForwardList;
  let mut it: *mut WebKitBackForwardListItem;

  if XFIXNUM (rel_pos) == 0
    { webkit_web_view_reload (wkwv); }
  else
    {
      list = webkit_web_view_get_back_forward_list (wkwv);
      it = webkit_back_forward_list_get_nth_item (list, XFIXNUM (rel_pos) as c_int);

      if it.is_null ()
	{ error (c"There is no item at this index".as_ptr ()); }

      webkit_web_view_go_to_back_forward_list_item (wkwv, it);
    }
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_webkit_goto_history (xw, XFIXNAT (rel_pos) as c_int);
  }
  return Qnil ();
});

/// C 3165-3166's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 38: `void nsxwidget_webkit_goto_history (struct xwidget *xw, int rel_pos);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_goto_history (xw: *mut xwidget, rel_pos: c_int);
}

/// C 3171-3174: `DEFUN ("xwidget-webkit-zoom", Fxwidget_webkit_zoom, Sxwidget_webkit_zoom, 2, 2,
/// 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-zoom", Fxwidget_webkit_zoom, Sxwidget_webkit_zoom, 2, 2, 0,
       doc: "Change the zoom factor of the xwidget webkit instance referenced by XWIDGET.",
       (xwidget: Lisp_Object, factor: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);
  if FLOATP (factor)
    {
      let zoom_change: c_double = XFLOAT_DATA (factor);

      #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
      {
      webkit_web_view_set_zoom_level
        (WEBKIT_WEB_VIEW! ((*xw).widget_osr),
         webkit_web_view_get_zoom_level
         (WEBKIT_WEB_VIEW! ((*xw).widget_osr)) + zoom_change);
      }
      #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
      {
      nsxwidget_webkit_zoom (xw, zoom_change);
      }
    }
  return Qnil ();
});

/// C 3186-3187's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 41: `void nsxwidget_webkit_zoom (struct xwidget *xw, double zoom_change);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_zoom (xw: *mut xwidget, zoom_change: c_double);
}

/* C 3193's `#ifdef USE_GTK`; its `#endif` is C 3215.  */

/* Save script and fun in the script/callback save vector and return
   its index.  */

/// C: 3196-3197: `static ptrdiff_t` / `save_script_callback (struct xwidget *xw, Lisp_Object
/// script, Lisp_Object fun)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn save_script_callback (xw: *mut xwidget, script: Lisp_Object, fun: Lisp_Object)
  -> ptrdiff_t
{
  let mut cbs: Lisp_Object = (*xw).script_callbacks;
  if NILP (cbs)
    { cbs = make_nil_vector (32); (*xw).script_callbacks = cbs; }

  /* Find first free index.  */
  let mut idx: ptrdiff_t;
  idx = 0;
  while !NILP (AREF (cbs, idx))
    {
      if idx + 1 == ASIZE (cbs)
	{
	  cbs = larger_vector (cbs, 1 as ptrdiff_t, -1 as ptrdiff_t);
	  (*xw).script_callbacks = cbs;
	  break;
	}
      idx += 1;
    }

  ASET (cbs, idx, Fcons (make_mint_ptr (xlispstrdup (script) as *mut c_void), fun));
  return idx;
}

/// C 3217-3222: `DEFUN ("xwidget-webkit-execute-script", Fxwidget_webkit_execute_script,
/// Sxwidget_webkit_execute_script, 2, 3, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-execute-script", Fxwidget_webkit_execute_script,
       Sxwidget_webkit_execute_script, 2, 3, 0,
       doc: "Make the Webkit XWIDGET execute JavaScript SCRIPT.\nIf FUN is provided, feed the JavaScript return value to the single\nargument procedure FUN.",
       (xwidget: Lisp_Object, script: Lisp_Object, fun: Lisp_Object),
       {
  let xw: *mut xwidget = WEBKIT_FN_INIT! (xwidget);
  /* C 3230 assigns its own `script` parameter (`script = ENCODE_SYSTEM (script);`).  */
  let mut script: Lisp_Object = script;
  CHECK_STRING (script);
  if !NILP (fun) && !FUNCTIONP (fun)
    { wrong_type_argument (Qinvalid_function (), fun); }

  script = ENCODE_SYSTEM (script);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  /* Protect script and fun during GC.  */
  let idx: intptr_t = save_script_callback (xw, script, fun) as intptr_t;

  /* JavaScript execution happens asynchronously.  If an elisp
     callback function is provided we pass it to the C callback
     procedure that retrieves the return value.  */
  let script_string: *mut c_char
    = xmint_pointer (XCAR (AREF ((*xw).script_callbacks, idx as ptrdiff_t))) as *mut c_char;
  webkit_web_view_run_javascript (WEBKIT_WEB_VIEW! ((*xw).widget_osr),
				  script_string,
                                  core::ptr::null_mut (), /* cancelable */
                                  Some (webkit_javascript_finished_cb),
				  idx as gpointer);
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_webkit_execute_script (xw, SSDATA (script), fun);
  }
  return Qnil ();
});

/// C 3246-3247's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 42-43: `void nsxwidget_webkit_execute_script (struct xwidget *xw, const char
/// *script, Lisp_Object fun);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_execute_script (xw: *mut xwidget, script: *mut c_char,
					  fun: Lisp_Object);
}

/// C 3252-3253: `DEFUN ("xwidget-resize", Fxwidget_resize, Sxwidget_resize, 3, 3, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-resize", Fxwidget_resize, Sxwidget_resize, 3, 3, 0,
       doc: "Resize XWIDGET to NEW_WIDTH, NEW_HEIGHT.",
       (xwidget: Lisp_Object, new_width: Lisp_Object, new_height: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  /* `check_integer_range`'s bounds are C's `intmax_t` (its own declaration, lisp.h 3773).  */
  let w: c_int = check_integer_range (new_width, 0 as intmax_t, INT_MAX as intmax_t) as c_int;
  let h: c_int = check_integer_range (new_height, 0 as intmax_t, INT_MAX as intmax_t) as c_int;
  let xw: *mut xwidget = XXWIDGET (xwidget);

  (*xw).width = w;
  (*xw).height = h;

  block_input ();

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
        {
          let xv: *mut xwidget_view = XXWIDGET_VIEW (XCAR (tail));
          if XXWIDGET ((*xv).model) == xw
            {
	      #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
	      {
	      (*xv).just_resized = 1;
	      SET_FRAME_GARBAGED! ((*xv).frame);
	      }
	      #[cfg (all (not (feature = "use_gtk"), feature = "have_gtk3"))]
	      {
	      wset_redisplay (XWINDOW ((*xv).w));
	      }
            }
        }
      tail = XCDR (tail);
    }

  redisplay ();

  /* If there is an offscreen widget resize it first.  */
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  if !(*xw).widget_osr.is_null ()
    {
      gtk_window_resize (GTK_WINDOW! ((*xw).widgetwindow_osr), (*xw).width,
                         (*xw).height);
      gtk_widget_set_size_request (GTK_WIDGET! ((*xw).widget_osr), (*xw).width,
                                   (*xw).height);

      gtk_widget_queue_allocate (GTK_WIDGET! ((*xw).widget_osr));
    }
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_resize (xw);
  }
  unblock_input ();
  return Qnil ();
});

/// C 3297-3298's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 55: `void nsxwidget_resize (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_resize (xw: *mut xwidget);
}

/* C 3308's `DEFUN ("xwidget-size-request", …)` on: slice 5.  The C line-range table's row
   3308-4304 is still the owed remainder, and this slice lands C 3308-3547; C 3548 on (the
   `xwidget-webkit-search` family, the store/kill helpers and `syms_of_xwidget`) is next.  */

/// C 3308-3313: `DEFUN ("xwidget-size-request", Fxwidget_size_request, Sxwidget_size_request, 1,
/// 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-size-request", Fxwidget_size_request, Sxwidget_size_request, 1, 1, 0,
       doc: "Return the desired size of the XWIDGET.\nThis can be used to read the xwidget desired size, and resizes the\nEmacs allocated area accordingly.",
       (xwidget: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  let mut requisition: GtkRequisition = core::mem::zeroed ();
  gtk_widget_size_request ((*XXWIDGET (xwidget)).widget_osr, &mut requisition);
  return list2i (requisition.width as EMACS_INT, requisition.height as EMACS_INT);
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  return nsxwidget_get_size (XXWIDGET (xwidget));
  }
});

/// C 3321-3322's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 47: `Lisp_Object nsxwidget_get_size (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_get_size (xw: *mut xwidget) -> Lisp_Object;
}

/// C 3326-3328: `DEFUN ("xwidgetp", Fxwidgetp, Sxwidgetp, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidgetp", Fxwidgetp, Sxwidgetp, 1, 1, 0,
       doc: "Return t if OBJECT is an xwidget.",
       (object: Lisp_Object),
       {
  return if XWIDGETP (object) { Qt () } else { Qnil () };
});

/// C 3335-3338: `DEFUN ("xwidget-view-p", Fxwidget_view_p, Sxwidget_view_p, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-view-p", Fxwidget_view_p, Sxwidget_view_p, 1, 1, 0,
       doc: "Return t if OBJECT is an xwidget-view.",
       (object: Lisp_Object),
       {
  return if XWIDGET_VIEW_P (object) { Qt () } else { Qnil () };
});

/// C 3344-3348: `DEFUN ("xwidget-info", Fxwidget_info, Sxwidget_info, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-info", Fxwidget_info, Sxwidget_info, 1, 1, 0,
       doc: "Return XWIDGET properties in a vector.\nCurrently [TYPE TITLE WIDTH HEIGHT].",
       (xwidget: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  let xw: *mut xwidget = XXWIDGET (xwidget);
  return CALLN! (Fvector, (*xw).r#type, (*xw).title,
		 make_fixed_natnum ((*xw).width as EMACS_INT),
		 make_fixed_natnum ((*xw).height as EMACS_INT));
});

/// C 3357-3361: `DEFUN ("xwidget-view-info", Fxwidget_view_info, Sxwidget_view_info, 1, 1, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-view-info", Fxwidget_view_info, Sxwidget_view_info, 1, 1, 0,
       doc: "Return properties of XWIDGET-VIEW in a vector.\nCurrently [X Y CLIP_RIGHT CLIP_BOTTOM CLIP_TOP CLIP_LEFT].",
       (xwidget_view: Lisp_Object),
       {
  CHECK_XWIDGET_VIEW (xwidget_view);
  let xv: *mut xwidget_view = XXWIDGET_VIEW (xwidget_view);
  return CALLN! (Fvector, make_fixnum ((*xv).x as EMACS_INT), make_fixnum ((*xv).y as EMACS_INT),
		 make_fixnum ((*xv).clip_right as EMACS_INT),
		 make_fixnum ((*xv).clip_bottom as EMACS_INT),
		 make_fixnum ((*xv).clip_top as EMACS_INT),
		 make_fixnum ((*xv).clip_left as EMACS_INT));
});

/// C 3371-3374: `DEFUN ("xwidget-view-model", Fxwidget_view_model, Sxwidget_view_model, 1, 1, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-view-model", Fxwidget_view_model, Sxwidget_view_model, 1, 1, 0,
       doc: "Return the model associated with XWIDGET-VIEW.",
       (xwidget_view: Lisp_Object),
       {
  CHECK_XWIDGET_VIEW (xwidget_view);
  return (*XXWIDGET_VIEW (xwidget_view)).model;
});

/// C 3381-3384: `DEFUN ("xwidget-view-window", Fxwidget_view_window, Sxwidget_view_window, 1, 1,
/// 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-view-window", Fxwidget_view_window, Sxwidget_view_window, 1, 1, 0,
       doc: "Return the window of XWIDGET-VIEW.",
       (xwidget_view: Lisp_Object),
       {
  CHECK_XWIDGET_VIEW (xwidget_view);
  return (*XXWIDGET_VIEW (xwidget_view)).w;
});

/// C 3392-3395: `DEFUN ("delete-xwidget-view", Fdelete_xwidget_view, Sdelete_xwidget_view, 1, 1,
/// 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("delete-xwidget-view", Fdelete_xwidget_view, Sdelete_xwidget_view, 1, 1, 0,
       doc: "Delete the XWIDGET-VIEW.",
       (xwidget_view: Lisp_Object),
       {
  CHECK_XWIDGET_VIEW (xwidget_view);
  let xv: *mut xwidget_view = XXWIDGET_VIEW (xwidget_view);

  block_input ();
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  let xw: *mut xwidget = XXWIDGET ((*xv).model);
  let mut w: *mut GdkWindow;
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
  {
  use x11_constants::None;
  if (*xv).wdesc != None as Window
    {
      cairo_destroy ((*xv).cr_context);
      cairo_surface_destroy ((*xv).cr_surface);
      XDestroyWindow ((*xv).dpy, (*xv).wdesc);
      Fremhash (make_fixnum ((*xv).wdesc as EMACS_INT), x_window_to_xwv_map);
    }

  if !(*xv).last_crossing_window.is_null ()
    {
      g_signal_handler_disconnect ((*xv).last_crossing_window as *mut c_void,
				   (*xv).last_crossing_cursor_signal as gulong);
    }
  g_clear_pointer! (&raw mut (*xv).last_crossing_window,
		    g_object_unref);

  if !(*xv).passive_grab.is_null ()
    {
      g_signal_handler_disconnect ((*xv).passive_grab as *mut c_void,
				   (*xv).passive_grab_destruction_signal as gulong);
      g_signal_handler_disconnect ((*xv).passive_grab as *mut c_void,
				   (*xv).passive_grab_drag_signal as gulong);
      (*xv).passive_grab = core::ptr::null_mut ();
    }
  }
  /* C 3429-3430's `#else`: USE_GTK *without* HAVE_X_WINDOWS.  The port spells that arm
     `have_pgtk`, because it is the arm in which the record's `widget` member exists
     (`include/xwidget.rs` 145-148 gates `dpyinfo`/`widget` on `all (use_gtk, have_pgtk)`) -
     the same "follow the mirror's own reading of the arm" shape Deviations 4 records.  */
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
  {
  gtk_widget_destroy ((*xv).widget);
  }

  if (*xw).embedder_view == xv as *mut c_void && !NILP ((*xw).buffer)
    {
      w = gtk_widget_get_window ((*xw).widgetwindow_osr);

      (*XXWIDGET ((*xv).model)).embedder_view = core::ptr::null_mut ();
      (*XXWIDGET ((*xv).model)).embedder = core::ptr::null_mut ();

      gdk_offscreen_window_set_embedder (w, core::ptr::null_mut ());
    }
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_delete_view (xv);
  }

  internal_xwidget_view_list = Fdelq (xwidget_view, internal_xwidget_view_list);
  Vxwidget_view_list = Fcopy_sequence (internal_xwidget_view_list);
  unblock_input ();
  return Qnil ();
});

/// C 3442-3443's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 48: `void nsxwidget_delete_view (struct xwidget_view *xv);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_delete_view (xv: *mut xwidget_view);
}

/// C 3392-3395's `XDestroyWindow`: X11/Xlib.h's declaration, at the use site (this unit's
/// X11 surface, P-72 rule 4; the `XMoveWindow` family at line 2694-2710 is the same shape).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" {
  /// C 3410: `XDestroyWindow (xv->dpy, xv->wdesc);`
  /// C: X11/Xlib.h: `int XDestroyWindow (Display *display, Window w);`
  pub fn XDestroyWindow (display: *mut Display, w: Window) -> c_int;
}

/// C 3452-3457: `DEFUN ("xwidget-view-lookup", Fxwidget_view_lookup, Sxwidget_view_lookup, 1, 2,
/// 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-view-lookup", Fxwidget_view_lookup, Sxwidget_view_lookup, 1, 2, 0,
       doc: "Return the xwidget-view associated with XWIDGET in WINDOW.\nIf WINDOW is unspecified or nil, use the selected window.\nReturn nil if no association is found.",
       (xwidget: Lisp_Object, window: Lisp_Object),
       {
  let mut window: Lisp_Object = window;
  CHECK_XWIDGET (xwidget);

  if NILP (window)
    { window = Fselected_window (); }
  CHECK_WINDOW (window);

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      let xwidget_view: Lisp_Object = XCAR (tail);
      if EQ (Fxwidget_view_model (xwidget_view), xwidget)
          && EQ (Fxwidget_view_window (xwidget_view), window)
        { return xwidget_view; }
      tail = XCDR (tail);
    }

  return Qnil ();
});

/// C 3478-3481: `DEFUN ("xwidget-plist", Fxwidget_plist, Sxwidget_plist, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-plist", Fxwidget_plist, Sxwidget_plist, 1, 1, 0,
       doc: "Return the plist of XWIDGET.",
       (xwidget: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  return (*XXWIDGET (xwidget)).plist;
});

/// C 3488-3491: `DEFUN ("xwidget-buffer", Fxwidget_buffer, Sxwidget_buffer, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-buffer", Fxwidget_buffer, Sxwidget_buffer, 1, 1, 0,
       doc: "Return the buffer of XWIDGET.",
       (xwidget: Lisp_Object),
       {
  CHECK_XWIDGET (xwidget);
  return (*XXWIDGET (xwidget)).buffer;
});

/// C 3498-3501: `DEFUN ("set-xwidget-buffer", Fset_xwidget_buffer, Sset_xwidget_buffer, 2, 2, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("set-xwidget-buffer", Fset_xwidget_buffer, Sset_xwidget_buffer, 2, 2, 0,
       doc: "Set XWIDGET's buffer to BUFFER.",
       (xwidget: Lisp_Object, buffer: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  CHECK_BUFFER (buffer);

  (*XXWIDGET (xwidget)).buffer = buffer;
  return Qnil ();
});

/// C 3511-3515: `DEFUN ("set-xwidget-plist", Fset_xwidget_plist, Sset_xwidget_plist, 2, 2, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("set-xwidget-plist", Fset_xwidget_plist, Sset_xwidget_plist, 2, 2, 0,
       doc: "Replace the plist of XWIDGET with PLIST.\nReturns PLIST.",
       (xwidget: Lisp_Object, plist: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  CHECK_LIST (plist);

  (*XXWIDGET (xwidget)).plist = plist;
  return plist;
});

/// C 3525-3531: `DEFUN ("set-xwidget-query-on-exit-flag", Fset_xwidget_query_on_exit_flag,
/// Sset_xwidget_query_on_exit_flag, 2, 2, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("set-xwidget-query-on-exit-flag", Fset_xwidget_query_on_exit_flag,
       Sset_xwidget_query_on_exit_flag, 2, 2, 0,
       doc: "Specify if query is needed for XWIDGET when Emacs is exited.\nIf the second argument FLAG is non-nil, Emacs will query the user before\nexiting or killing a buffer if XWIDGET is running.\nThis function returns FLAG.",
       (xwidget: Lisp_Object, flag: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  /* C 99's `bool_bf kill_without_query : 1` is recorded, not packed, by `include/xwidget.rs`
     (its own note; P-80 requires the record's byte dump before a storage unit may stand for a
     bitfield, and this record cannot be compiled here).  C's statement is carried as C writes
     it - neither stubbed nor dropped - and the arm compile reports `no field
     kill_without_query` for the line below - the registered mirror-gap class (Owed 9).  */
  (*XXWIDGET (xwidget)).kill_without_query = NILP (flag);
  return flag;
});

/// C 3539-3542: `DEFUN ("xwidget-query-on-exit-flag", Fxwidget_query_on_exit_flag,
/// Sxwidget_query_on_exit_flag, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-query-on-exit-flag", Fxwidget_query_on_exit_flag,
       Sxwidget_query_on_exit_flag, 1, 1, 0,
       doc: "Return the current value of the query-on-exit flag for XWIDGET.",
       (xwidget: Lisp_Object),
       {
  CHECK_LIVE_XWIDGET (xwidget);
  /* The same mirror gap as the setter above (Owed 9).  */
  return if (*XXWIDGET (xwidget)).kill_without_query { Qnil () } else { Qt () };
});

/* C 3549's `DEFUN ("xwidget-webkit-search", …)` on: slice 6 (C 3548-3770).  The C line-range
   table's owed row is now 3548-4304; this slice lands its first six items, and C 3771 on (the
   `xwidget-webkit-back-forward-list` and the two remaining WebKit subrs, then `syms_of_xwidget`
   and the store/kill helpers) is next.  */

/// C 3549-3565: `DEFUN ("xwidget-webkit-search", Fxwidget_webkit_search, Sxwidget_webkit_search,
/// 2, 5, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-search", Fxwidget_webkit_search, Sxwidget_webkit_search, 2, 5, 0,
       doc: "Begin an incremental search operation in an xwidget.\nQUERY should be a string containing the text to search for.  XWIDGET\nshould be a WebKit xwidget where the search will take place.  When the\nsearch operation is complete, callers should also call\n`xwidget-webkit-finish-search' to complete the search operation.\n\nCASE-INSENSITIVE, when non-nil, will cause the search to ignore the\ncase of characters inside QUERY.  BACKWARDS, when non-nil, will cause\nthe search to proceed towards the beginning of the widget's contents.\nWRAP-AROUND, when nil, will cause the search to stop upon hitting the\nend of the widget's contents.\n\nIt is OK to call this function even when a search is already in\nprogress.  In that case, the previous search query will be replaced\nwith QUERY.",
       (query: Lisp_Object, xwidget: Lisp_Object, case_insensitive: Lisp_Object,
	backwards: Lisp_Object, wrap_around: Lisp_Object),
       {
  /* C 3570-3574's `#ifdef USE_GTK` locals: this file's gate *is* `USE_GTK` (the ledger's
     `## Gate mapping`), so they stand at function scope with no inner `#[cfg]`.  */
  let mut webview: *mut WebKitWebView;
  let mut controller: *mut WebKitFindController;
  let mut opt: WebKitFindOptions;
  let mut xw: *mut xwidget;
  let mut g_query: *mut c_char;
  /* C 3585 assigns its own `query` parameter (`query = ENCODE_UTF_8 (query);`).  */
  let mut query: Lisp_Object = query;

  CHECK_STRING (query);
  CHECK_LIVE_XWIDGET (xwidget);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  query = ENCODE_UTF_8 (query);
  opt = WEBKIT_FIND_OPTIONS_NONE;
  g_query = xstrdup (SSDATA (query));

  if !NILP (case_insensitive)
    { opt |= WEBKIT_FIND_OPTIONS_CASE_INSENSITIVE; }
  if !NILP (backwards)
    { opt |= WEBKIT_FIND_OPTIONS_BACKWARDS; }
  if !NILP (wrap_around)
    { opt |= WEBKIT_FIND_OPTIONS_WRAP_AROUND; }

  if !(*xw).find_text.is_null ()
    { xfree ((*xw).find_text as *mut c_void); }
  (*xw).find_text = g_query;

  block_input ();
  controller = webkit_web_view_get_find_controller (webview);
  /* C 3602 passes its `WebKitFindOptions opt` to the `guint32 find_options` parameter.  */
  webkit_find_controller_search (controller, g_query, opt as guint32, G_MAXUINT as guint);
  unblock_input ();
  }

  return Qnil ();
});

/// C 3609-3615: `DEFUN ("xwidget-webkit-next-result", Fxwidget_webkit_next_result,
/// Sxwidget_webkit_next_result, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-next-result", Fxwidget_webkit_next_result,
       Sxwidget_webkit_next_result, 1, 1, 0,
       doc: "Show the next result matching the current search query.\n\nXWIDGET should be an xwidget that currently has a search query.\nBefore calling this function, you should start a search operation\nusing `xwidget-webkit-search'.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  /* C 3619-3622 / 3652-3655 / 3685-3688's `#ifdef USE_GTK` locals (the file gate, as above).  */
  let mut webview: *mut WebKitWebView;
  let mut controller: *mut WebKitFindController;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  if (*xw).find_text.is_null ()
    { error (c"Widget has no ongoing search operation".as_ptr ()); }

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  block_input ();
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  controller = webkit_web_view_get_find_controller (webview);
  webkit_find_controller_search_next (controller);
  unblock_input ();
  }

  return Qnil ();
});

/// C 3642-3648: `DEFUN ("xwidget-webkit-previous-result", Fxwidget_webkit_previous_result,
/// Sxwidget_webkit_previous_result, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-previous-result", Fxwidget_webkit_previous_result,
       Sxwidget_webkit_previous_result, 1, 1, 0,
       doc: "Show the previous result matching the current search query.\n\nXWIDGET should be an xwidget that currently has a search query.\nBefore calling this function, you should start a search operation\nusing `xwidget-webkit-search'.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  /* C 3619-3622 / 3652-3655 / 3685-3688's `#ifdef USE_GTK` locals (the file gate, as above).  */
  let mut webview: *mut WebKitWebView;
  let mut controller: *mut WebKitFindController;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  if (*xw).find_text.is_null ()
    { error (c"Widget has no ongoing search operation".as_ptr ()); }

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  block_input ();
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  controller = webkit_web_view_get_find_controller (webview);
  webkit_find_controller_search_previous (controller);
  unblock_input ();
  }

  return Qnil ();
});

/// C 3675-3681: `DEFUN ("xwidget-webkit-finish-search", Fxwidget_webkit_finish_search,
/// Sxwidget_webkit_finish_search, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-finish-search", Fxwidget_webkit_finish_search,
       Sxwidget_webkit_finish_search, 1, 1, 0,
       doc: "Finish XWIDGET's search operation.\n\nXWIDGET should be an xwidget that currently has a search query.\nBefore calling this function, you should start a search operation\nusing `xwidget-webkit-search'.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  /* C 3619-3622 / 3652-3655 / 3685-3688's `#ifdef USE_GTK` locals (the file gate, as above).  */
  let mut webview: *mut WebKitWebView;
  let mut controller: *mut WebKitFindController;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  if (*xw).find_text.is_null ()
    { error (c"Widget has no ongoing search operation".as_ptr ()); }

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  block_input ();
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  controller = webkit_web_view_get_find_controller (webview);
  webkit_find_controller_search_finish (controller);

  if !(*xw).find_text.is_null ()
    {
      xfree ((*xw).find_text as *mut c_void);
      (*xw).find_text = core::ptr::null_mut ();
    }
  unblock_input ();
  }

  return Qnil ();
});

/// C 3714-3718: `DEFUN ("kill-xwidget", Fkill_xwidget, Skill_xwidget, 1, 1, 0, doc: ...)`.  Its
/// call to `kill_xwidget` (C 4247's `static` definition, a later slice) is one of the names that
/// unit's `## Owed` carries until then - C's text is carried as it stands.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("kill-xwidget", Fkill_xwidget, Skill_xwidget, 1, 1, 0,
       doc: "Kill the specified XWIDGET.\nThis releases all window system resources associated with XWIDGET,\nremoves it from `xwidget-list', and detaches it from its buffer.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);

  block_input ();
  kill_xwidget (xw);
  unblock_input ();

  return Qnil ();
});


/* C 96's `static void kill_xwidget (struct xwidget *);` prototype and slice 6's use-site
   declaration are gone: C 4247's definition is carried by this file now (slice 8), so the
   name is defined, not declared (the P-91 shape).  */

/// C 3734-3741: `DEFUN ("xwidget-webkit-load-html", Fxwidget_webkit_load_html,
/// Sxwidget_webkit_load_html, 2, 3, 0, doc: ...)` - C 3733's `#ifdef USE_GTK` is this file's
/// gate (the ledger's `## Gate mapping`), so the DEFUN carries it like every other item.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-load-html", Fxwidget_webkit_load_html, Sxwidget_webkit_load_html,
       2, 3, 0,
       doc: "Make XWIDGET's WebKit widget render TEXT.\nXWIDGET should be a WebKit xwidget, that will receive TEXT.  TEXT\nshould be a string that will be displayed by XWIDGET as HTML markup.\nBASE-URI should be a string containing a URI that is used to locate\nresources with relative URLs, and if not specified, defaults\nto \"about:blank\".",
       (xwidget: Lisp_Object, text: Lisp_Object, base_uri: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  let mut webview: *mut WebKitWebView;
  let mut data: *mut c_char;
  let mut uri: *mut c_char;

  /* C 3751/3755-3756 assign their own `base_uri` and `text` parameters.  */
  let mut base_uri: Lisp_Object = base_uri;
  let mut text: Lisp_Object = text;

  CHECK_LIVE_XWIDGET (xwidget);
  CHECK_STRING (text);
  if NILP (base_uri)
    { base_uri = build_string (c"about:blank".as_ptr ()); }
  else
    { CHECK_STRING (base_uri); }

  base_uri = ENCODE_UTF_8 (base_uri);
  text = ENCODE_UTF_8 (text);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  data = SSDATA (text);
  uri = SSDATA (base_uri);
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);

  block_input ();
  webkit_web_view_load_html (webview, data, uri);
  unblock_input ();

  return Qnil ();
});

/* C 3771's `DEFUN ("xwidget-webkit-back-forward-list", …)` on: slice 7 (C 3771-4035).  This lands
   the last three WebKit subrs and `syms_of_xwidget` (C 3928-4026, which registers every subr this
   unit has ported); C 4028 on (`valid_xwidget_spec_p` and the store/kill helpers, C 4247's
   `kill_xwidget` among them) is the file's last slice.  */

/// C 3771-3786: `DEFUN ("xwidget-webkit-back-forward-list", Fxwidget_webkit_back_forward_list,
/// Sxwidget_webkit_back_forward_list, 1, 2, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-back-forward-list", Fxwidget_webkit_back_forward_list,
       Sxwidget_webkit_back_forward_list, 1, 2, 0,
       doc: "Return the navigation history of XWIDGET, a WebKit xwidget.\n\nReturn the history as a list of the form (BACK HERE FORWARD), where\nHERE is the current navigation item, while BACK and FORWARD are lists\nof history items of the form (IDX TITLE URI).  Here, IDX is an index\nthat can be passed to `xwidget-webkit-goto-history', TITLE is a string\ncontaining the human-readable title of the history item, and URI is\nthe URI of the history item.\n\nBACK, HERE, and FORWARD can all be nil depending on the state of the\nnavigation history.\n\nBACK and FORWARD will each not contain more elements than LIMIT.  If\nLIMIT is not specified or nil, it is treated as `50'.",
       (xwidget: Lisp_Object, limit: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  let mut back: Lisp_Object;
  let mut here: Lisp_Object;
  let mut forward: Lisp_Object;
  let mut webview: *mut WebKitWebView;
  let mut list: *mut WebKitBackForwardList;
  let mut item: *mut WebKitBackForwardListItem;
  let mut parent: *mut GList;
  let mut tem: *mut GList;
  let mut i: c_int;
  let mut lim: c_uint;
  let mut title: Lisp_Object;
  let mut uri: Lisp_Object;
  let mut item_title: *const gchar;
  let mut item_uri: *const gchar;

  /* C 3805 assigns its own `limit` parameter.  */
  let mut limit: Lisp_Object = limit;

  back = Qnil ();
  here = Qnil ();
  forward = Qnil ();

  if NILP (limit)
    { limit = make_fixnum (50); }
  else
    { CHECK_FIXNAT (limit); }

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);

  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  list = webkit_web_view_get_back_forward_list (webview);
  item = webkit_back_forward_list_get_current_item (list);
  lim = XFIXNAT (limit) as c_uint;

  if !item.is_null ()
    {
      item_title = webkit_back_forward_list_item_get_title (item);
      item_uri = webkit_back_forward_list_item_get_uri (item);
      here = list3 (make_fixnum (0),
		    build_string_from_utf8 (if item_title.is_null () { c"".as_ptr () }
					    else { item_title }),
		    build_string_from_utf8 (if item_uri.is_null () { c"".as_ptr () }
					    else { item_uri }));
    }
  parent = webkit_back_forward_list_get_back_list_with_limit (list, lim);

  if !parent.is_null ()
    {
      i = 1;
      tem = parent;
      while !tem.is_null ()
	{
	  item = (*tem).data as *mut WebKitBackForwardListItem;
	  item_title = webkit_back_forward_list_item_get_title (item);
	  item_uri = webkit_back_forward_list_item_get_uri (item);
	  title = build_string_from_utf8 (if item_title.is_null () { c"".as_ptr () }
					  else { item_title });
	  uri = build_string_from_utf8 (if item_uri.is_null () { c"".as_ptr () }
					else { item_uri });
	  back = Fcons (list3 (make_fixnum (-(i as EMACS_INT)), title, uri), back);
	  tem = (*tem).next;
	  i += 1;
	}
    }

  back = Fnreverse (back);
  g_list_free (parent);

  parent = webkit_back_forward_list_get_forward_list_with_limit (list, lim);

  if !parent.is_null ()
    {
      i = 1;
      tem = parent;
      while !tem.is_null ()
	{
	  item = (*tem).data as *mut WebKitBackForwardListItem;
	  item_title = webkit_back_forward_list_item_get_title (item);
	  item_uri = webkit_back_forward_list_item_get_uri (item);
	  title = build_string_from_utf8 (if item_title.is_null () { c"".as_ptr () }
					  else { item_title });
	  uri = build_string_from_utf8 (if item_uri.is_null () { c"".as_ptr () }
					else { item_uri });
	  forward = Fcons (list3 (make_fixnum (i as EMACS_INT), title, uri), forward);
	  tem = (*tem).next;
	  i += 1;
	}
    }

  forward = Fnreverse (forward);
  g_list_free (parent);

  return list3 (back, here, forward);
});

/// C 3866-3872: `DEFUN ("xwidget-webkit-set-cookie-storage-file",
/// Fxwidget_webkit_set_cookie_storage_file, Sxwidget_webkit_set_cookie_storage_file, 2, 2, 0,
/// doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-set-cookie-storage-file", Fxwidget_webkit_set_cookie_storage_file,
       Sxwidget_webkit_set_cookie_storage_file, 2, 2, 0,
       doc: "Make the WebKit widget XWIDGET load and store cookies in FILE.\n\nCookies will be stored as plain text in FILE, which must be an\nabsolute file name.  All xwidgets related to XWIDGET will also\nstore cookies in FILE and load them from there.",
       (xwidget: Lisp_Object, file: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  let mut webview: *mut WebKitWebView;
  let mut context: *mut WebKitWebContext;
  let mut manager: *mut WebKitCookieManager;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);
  CHECK_STRING (file);

  block_input ();
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  context = webkit_web_view_get_context (webview);
  manager = webkit_web_context_get_cookie_manager (context);
  webkit_cookie_manager_set_persistent_storage (manager,
						SSDATA (ENCODE_UTF_8 (file)),
						WEBKIT_COOKIE_PERSISTENT_STORAGE_TEXT);
  unblock_input ();

  return Qnil ();
});

/// C 3899-3903: `DEFUN ("xwidget-webkit-stop-loading", Fxwidget_webkit_stop_loading,
/// Sxwidget_webkit_stop_loading, 1, 1, 0, doc: ...)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
DEFUN! ("xwidget-webkit-stop-loading", Fxwidget_webkit_stop_loading,
       Sxwidget_webkit_stop_loading, 1, 1, 0,
       doc: "Stop loading data in the WebKit widget XWIDGET.\nThis will stop any data transfer that may still be in progress inside\nXWIDGET as part of loading a page.",
       (xwidget: Lisp_Object),
       {
  let mut xw: *mut xwidget;
  /* C 3907-3909's `#ifdef USE_GTK` local (the file gate, as slices 5/6 record).  */
  let mut webview: *mut WebKitWebView;

  CHECK_LIVE_XWIDGET (xwidget);
  xw = XXWIDGET (xwidget);
  CHECK_WEBKIT_WIDGET! (xw);

  block_input ();
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  webview = WEBKIT_WEB_VIEW! ((*xw).widget_osr);
  webkit_web_view_stop_loading (webview);
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_webkit_stop_loading (xw);
  }
  unblock_input ();

  return Qnil ();
});

/// C 3919-3920's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 40: `void nsxwidget_webkit_stop_loading (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_webkit_stop_loading (xw: *mut xwidget);
}

/// C 3996-4001's `DEFVAR_BOOL` carrier (P-97: a Bool carrier is the unit's own `bool`); the
/// declaration in `include/globals.rs` 7205 is the mirror's `extern`, and this definition is what
/// the macro dereferences (`#[unsafe(no_mangle)]` shadows the glob import, PORTING.md P-21).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub static mut xwidget_webkit_disable_javascript: bool = false;

/// C 3928: `void` / `syms_of_xwidget (void)` - non-`static`, so exported.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xwidget ()
{
  defsubr (&raw mut Smake_xwidget);
  defsubr (&raw mut Sxwidgetp);
  defsubr (&raw mut Sxwidget_live_p);
  DEFSYM! (Qxwidgetp, c"xwidgetp");
  DEFSYM! (Qxwidget_live_p, c"xwidget-live-p");
  defsubr (&raw mut Sxwidget_view_p);
  DEFSYM! (Qxwidget_view_p, c"xwidget-view-p");
  defsubr (&raw mut Sxwidget_info);
  defsubr (&raw mut Sxwidget_view_info);
  defsubr (&raw mut Sxwidget_resize);
  defsubr (&raw mut Sget_buffer_xwidgets);
  defsubr (&raw mut Sxwidget_view_model);
  defsubr (&raw mut Sxwidget_view_window);
  defsubr (&raw mut Sxwidget_view_lookup);
  defsubr (&raw mut Sxwidget_query_on_exit_flag);
  defsubr (&raw mut Sset_xwidget_query_on_exit_flag);

  defsubr (&raw mut Sxwidget_webkit_uri);
  defsubr (&raw mut Sxwidget_webkit_title);
  defsubr (&raw mut Sxwidget_webkit_goto_uri);
  defsubr (&raw mut Sxwidget_webkit_goto_history);
  defsubr (&raw mut Sxwidget_webkit_zoom);
  defsubr (&raw mut Sxwidget_webkit_execute_script);
  DEFSYM! (Qwebkit, c"webkit");

  defsubr (&raw mut Sxwidget_size_request);
  defsubr (&raw mut Sdelete_xwidget_view);

  defsubr (&raw mut Sxwidget_plist);
  defsubr (&raw mut Sxwidget_buffer);
  defsubr (&raw mut Sset_xwidget_plist);
  defsubr (&raw mut Sxwidget_perform_lispy_event);
  defsubr (&raw mut Sxwidget_webkit_search);
  defsubr (&raw mut Sxwidget_webkit_finish_search);
  defsubr (&raw mut Sxwidget_webkit_next_result);
  defsubr (&raw mut Sxwidget_webkit_previous_result);
  defsubr (&raw mut Sset_xwidget_buffer);
  defsubr (&raw mut Sxwidget_webkit_set_cookie_storage_file);
  defsubr (&raw mut Sxwidget_webkit_stop_loading);
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  defsubr (&raw mut Sxwidget_webkit_load_html);
  defsubr (&raw mut Sxwidget_webkit_back_forward_list);
  }

  defsubr (&raw mut Sxwidget_webkit_estimated_load_progress);
  defsubr (&raw mut Skill_xwidget);

  DEFSYM! (QCxwidget, c":xwidget");
  DEFSYM! (QCtitle, c":title");

  /* Do not forget to update the docstring of make-xwidget if you add
     new types.  */

  DEFSYM! (Qvertical, c"vertical");
  DEFSYM! (Qhorizontal, c"horizontal");

  DEFSYM! (QCplist, c":plist");

  DEFVAR_LISP! (c"xwidget-list", Vxwidget_list,
		"List of all xwidgets that have not been killed.");
  Vxwidget_list = Qnil ();

  DEFVAR_LISP! (c"xwidget-view-list", Vxwidget_view_list,
		"List of all xwidget views.");
  Vxwidget_view_list = Qnil ();

  DEFVAR_BOOL! (c"xwidget-webkit-disable-javascript", xwidget_webkit_disable_javascript,
    "If non-nil, disable execution of JavaScript in xwidget WebKit widgets.\nModifications to this setting do not take effect in existing WebKit\nwidgets; kill all xwidget-webkit buffers for changes in this setting\nto take effect.");
  xwidget_webkit_disable_javascript = false;

  Fprovide (intern (c"xwidget-internal".as_ptr ()), Qnil ());

  id_to_xwidget_map = CALLN! (Fmake_hash_table, QCtest (), Qeq (),
			      QCweakness (), Qvalue ());
  staticpro (&raw const id_to_xwidget_map);

  internal_xwidget_list = Qnil ();
  staticpro (&raw const internal_xwidget_list);
  internal_xwidget_view_list = Qnil ();
  staticpro (&raw const internal_xwidget_view_list);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
  {
  x_window_to_xwv_map = CALLN! (Fmake_hash_table, QCtest (), Qeq ());

  staticpro (&raw const x_window_to_xwv_map);

  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows",
	     feature = "webkit_check_version_ge_2_34"))]
  {
  dummy_tooltip_string = build_string (c"".as_ptr ());
  staticpro (&raw const dummy_tooltip_string);
  }
  }
  DEFSYM! (Qdownload_callback, c"download-callback");
  DEFSYM! (Qjavascript_callback, c"javascript-callback");
}

/* C 4028's comment on: slice 8, the file's last function block (C 4028-4304).  With it the
   unit's only owed text is C 2791-3041 (C 2793's `x_draw_xwidget_glyph_string`).  */

/* Value is non-zero if OBJECT is a valid Lisp xwidget specification.  A
   valid xwidget specification is a list whose car is the symbol
   `xwidget', and whose rest is a property list.  The property list must
   contain a value for key `:type'.  That value must be the name of a
   supported xwidget type.  The rest of the property list depends on the
   xwidget type.  */

/// C: 4036-4037: `bool` / `valid_xwidget_spec_p (Lisp_Object object)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn valid_xwidget_spec_p (object: Lisp_Object) -> bool
{
  return CONSP (object) && EQ (XCAR (object), Qxwidget ());
}

/* Find a value associated with key in spec.  */

/// C: 4044-4045: `static Lisp_Object` / `xwidget_spec_value (Lisp_Object spec, Lisp_Object key)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_spec_value (spec: Lisp_Object, key: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object;

  eassert! (valid_xwidget_spec_p (spec));

  tail = XCDR (spec);
  while CONSP (tail) && CONSP (XCDR (tail))
    {
      if EQ (XCAR (tail), key)
	{ return XCAR (XCDR (tail)); }
      tail = XCDR (XCDR (tail));
    }

  return Qnil ();
}

/// C: 4062-4063: `void` / `xwidget_view_delete_all_in_window (struct window *w)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_view_delete_all_in_window (w: *mut window)
{
  let mut xv: *mut xwidget_view = core::ptr::null_mut ();
  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
        {
          xv = XXWIDGET_VIEW (XCAR (tail));
          if XWINDOW ((*xv).w) == w
            {
              Fdelete_xwidget_view (XCAR (tail));
            }
        }
      tail = XCDR (tail);
    }
}

/// C: 4080-4081: `static struct xwidget_view *` / `xwidget_view_lookup (struct xwidget *xw,
/// struct window *w)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_view_lookup (xw: *mut xwidget, w: *mut window) -> *mut xwidget_view
{
  let mut xwidget: Lisp_Object;
  let mut window: Lisp_Object;
  let mut ret: Lisp_Object;
  XSETXWIDGET! (xwidget, xw);
  XSETWINDOW! (window, w);

  ret = Fxwidget_view_lookup (xwidget, window);

  return if NILP (ret) { core::ptr::null_mut () } else { XXWIDGET_VIEW (ret) };
}

/// C: 4092-4093: `struct xwidget *` / `lookup_xwidget (Lisp_Object spec)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_xwidget (spec: Lisp_Object) -> *mut xwidget
{
  /* When a xwidget lisp spec is found initialize the C struct that is
     used in the C code.  This is done by redisplay so values change
     if the spec changes.  So, take special care of one-shot events.  */
  let mut value: Lisp_Object;
  let mut xw: *mut xwidget;

  value = xwidget_spec_value (spec, QCxwidget ());
  xw = XXWIDGET (value);

  return xw;
}

/* Set up detection of touched xwidget.  */

/// C: 4108-4109: `static void` / `xwidget_start_redisplay (void)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_start_redisplay ()
{
  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
        { (*XXWIDGET_VIEW (XCAR (tail))).redisplayed = false; }
      tail = XCDR (tail);
    }
}

/* The xwidget was touched during redisplay, so it isn't a candidate
   for hiding.  */

/// C: 4121-4122: `static void` / `xwidget_touch (struct xwidget_view *xv)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_touch (xv: *mut xwidget_view)
{
  (*xv).redisplayed = true;
}

/// C: 4127-4128: `static bool` / `xwidget_touched (struct xwidget_view *xv)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn xwidget_touched (xv: *mut xwidget_view) -> bool
{
  return (*xv).redisplayed;
}

/* Redisplay has ended, now we should hide untouched xwidgets.  */

/// C: 4134-4135: `void` / `xwidget_end_redisplay (struct window *w, struct glyph_matrix *matrix)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xwidget_end_redisplay (w: *mut window, matrix: *mut glyph_matrix)
{
  let mut i: c_int;
  let mut area: c_int;

  xwidget_start_redisplay ();
  /* Iterate desired glyph matrix of window here, hide gtk widgets
     not in the desired matrix.

     This only takes care of xwidgets in active windows.  If a window
     goes away from the screen, xwidget views must be deleted.

     dump_glyph_matrix (matrix, 2);  */
  i = 0;
  while i < (*matrix).nrows
    {
      /* dump_glyph_row (MATRIX_ROW (matrix, i), i, glyphs); */
      let mut row: *mut glyph_row;
      row = MATRIX_ROW! (matrix, i);
      if glyph_row_bits! (enabled_p, row)
	{
	  area = LEFT_MARGIN_AREA;
	  while area < LAST_AREA
	    {
	      let mut glyph: *mut glyph = (*row).glyphs [area as usize];
	      let glyph_end: *mut glyph = glyph.add ((*row).used [area as usize] as usize);
	      while glyph < glyph_end
		{
		  if glyph_bits! (type, glyph) == XWIDGET_GLYPH
		    {
		      /* The only call to xwidget_end_redisplay is in dispnew.
			 xwidget_end_redisplay (w->current_matrix);  */
		      let xv: *mut xwidget_view
			= xwidget_view_lookup (xwidget_from_id ((*glyph).u.xwidget), w);

		      /* In NS xwidget, xv can be NULL for the second or
                         later views for a model, the result of 1 to 1
                         model view relation enforcement.  `xwidget_view_lookup'
			 has also been observed to return NULL here on X-Windows
			 at least once, so stay safe and only touch it if it's
			 not NULL.  */

                      if !xv.is_null ()
                        { xwidget_touch (xv); }
		    }
		  glyph = glyph.add (1);
		}
	      area += 1;
	    }
	}
      i += 1;
    }

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
        {
          let xv: *mut xwidget_view = XXWIDGET_VIEW (XCAR (tail));

          /* "touched" is only meaningful for the current window, so
             disregard other views.  */
          if XWINDOW ((*xv).w) == w
            {
              if xwidget_touched (xv)
                {
		  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
		  {
                  xwidget_show_view (xv);
		  }
		  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
		  {
                  nsxwidget_show_view (xv);
		  }
                }
              else
                {
		  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
		  {
                  xwidget_hide_view (xv);
		  }
		  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
		  {
                  nsxwidget_hide_view (xv);
		  }
                }
            }
        }
      tail = XCDR (tail);
    }
}

/// C 4192-4195 and C 4200-4203's two `#elif defined NS_IMPL_COCOA` arms' callees.
/// C: nsxwidget.h 73: `void nsxwidget_show_view (struct xwidget_view *xv);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_show_view (xv: *mut xwidget_view);
}
/// C: nsxwidget.h 74: `void nsxwidget_hide_view (struct xwidget_view *xv);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_hide_view (xv: *mut xwidget_view);
}

/// C: 4212-4213: `void` / `lower_frame_xwidget_views (struct frame *f)` - C 4211's
/// `#ifdef HAVE_X_WINDOWS`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lower_frame_xwidget_views (f: *mut frame)
{
  let mut xv: *mut xwidget_view;

  /* C 4221's `None` is X11/X.h 115's `#define None 0L`, imported by name (slice 3's note).  */
  use x11_constants::None;

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      xv = XXWIDGET_VIEW (XCAR (tail));
      if (*xv).frame == f as *mut c_void && (*xv).wdesc != None as Window
	{ XLowerWindow ((*xv).dpy, (*xv).wdesc); }
      tail = XCDR (tail);
    }
}

/// C 4221's `None` (X11/X.h 115) and C 4222's `XLowerWindow`, at the use site (the unit's X11
/// surface, P-72 rule 4; the `XDestroyWindow` shape above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_x_windows"))]
unsafe extern "C" {
  /// C 4222: `XLowerWindow (xv->dpy, xv->wdesc);`
  /// C: X11/Xlib.h: `int XLowerWindow (Display *display, Window w);`
  pub fn XLowerWindow (display: *mut Display, w: Window) -> c_int;
}

/// C: 4228-4229: `void` / `kill_frame_xwidget_views (struct frame *f)` - C 4227's
/// `#ifndef NS_IMPL_COCOA`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "ns_impl_cocoa")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kill_frame_xwidget_views (f: *mut frame)
{
  let mut rem: Lisp_Object = Qnil ();

  let mut tail: Lisp_Object = internal_xwidget_view_list;
  while CONSP (tail)
    {
      if XWIDGET_VIEW_P (XCAR (tail))
	  && (*XXWIDGET_VIEW (XCAR (tail))).frame == f as *mut c_void
	{ rem = Fcons (XCAR (tail), rem); }
      tail = XCDR (tail);
    }

  while CONSP (rem)
    {
      Fdelete_xwidget_view (XCAR (rem));
      rem = XCDR (rem);
    }
}

/// C: 4246-4247: `static void` / `kill_xwidget (struct xwidget *xw)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn kill_xwidget (xw: *mut xwidget)
{
  let mut val: Lisp_Object;
  XSETXWIDGET! (val, xw);

  internal_xwidget_list = Fdelq (val, internal_xwidget_list);
  Vxwidget_list = Fcopy_sequence (internal_xwidget_list);
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
  {
  (*xw).buffer = Qnil ();

  if !(*xw).widget_osr.is_null () && !(*xw).widgetwindow_osr.is_null ()
    {
      gtk_widget_destroy ((*xw).widget_osr);
      gtk_widget_destroy ((*xw).widgetwindow_osr);
    }

  if !(*xw).find_text.is_null ()
    { xfree ((*xw).find_text as *mut c_void); }

  if !NILP ((*xw).script_callbacks)
    {
      let mut idx: ptrdiff_t = 0;
      while idx < ASIZE ((*xw).script_callbacks)
	{
	  let cb: Lisp_Object = AREF ((*xw).script_callbacks, idx);
	  if !NILP (cb)
	    { xfree (xmint_pointer (XCAR (cb))); }
	  ASET ((*xw).script_callbacks, idx, Qnil ());
	  idx += 1;
	}
    }

  (*xw).widget_osr = core::ptr::null_mut ();
  (*xw).widgetwindow_osr = core::ptr::null_mut ();
  (*xw).find_text = core::ptr::null_mut ();

  catch_child_signal ();
  }
  #[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
  {
  nsxwidget_kill (xw);
  }
}

/// C 4282-4283's `#elif defined NS_IMPL_COCOA` arm's callee.
/// C: nsxwidget.h 52: `void nsxwidget_kill (struct xwidget *xw);`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "ns_impl_cocoa"))]
unsafe extern "C" {
  pub fn nsxwidget_kill (xw: *mut xwidget);
}

/* Kill all xwidget in BUFFER.  */

/// C: 4288-4289: `void` / `kill_buffer_xwidgets (Lisp_Object buffer)`.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kill_buffer_xwidgets (buffer: Lisp_Object)
{
  let mut tail: Lisp_Object;
  let mut xwidget: Lisp_Object;
  tail = Fget_buffer_xwidgets (buffer);
  while CONSP (tail)
    {
      xwidget = XCAR (tail);
      {
        CHECK_LIVE_XWIDGET (xwidget);
        let xw: *mut xwidget = XXWIDGET (xwidget);

	kill_xwidget (xw);
      }
      tail = XCDR (tail);
    }

  catch_child_signal ();
}
