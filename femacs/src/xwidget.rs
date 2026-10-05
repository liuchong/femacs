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
//   * the **file gate** is `feature = "use_gtk"` (the mandate's ruling; C 37's `#ifdef USE_GTK` is
//     the file's first and widest one, and `USE_GTK` is `undef` at
//     `docs/baseline/config/darwin-arm64.macros.tsv` 3861), on every item *and on the include
//     graph* (P-86 ⑧ - an `include!` outside the gate makes the frozen build fail instead of
//     producing the empty object the class expects);
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
#[cfg (feature = "use_gtk")]
include!("include/lisp.rs");                   // C 25: `#include "lisp.h"`
/* C 25's `"lisp.h"` chain (lisp.h 30/33): `<string.h>` for `strcmp` (C 595) and `<limits.h>` for
   `INT_MAX` (C 554).  `include/lisp.rs` carries neither mirror (its own note records the
   intprops/sys-limits omission), so the two edges are here, at the C position that makes the
   names visible - the `src/gtkutil.rs` 96-99 shape.  */
#[cfg (feature = "use_gtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
#[cfg (feature = "use_gtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>`

#[cfg (feature = "use_gtk")]
include!("include/buffer.rs");                 // C 22: `#include "buffer.h"`
#[cfg (feature = "use_gtk")]
include!("include/coding.rs");                 // C 23: `#include "coding.h"`
#[cfg (feature = "use_gtk")]
include!("include/xwidget.rs");                // C 24: `#include "xwidget.h"`
#[cfg (feature = "use_gtk")]
include!("include/blockinput.rs");             // C 26: `#include "blockinput.h"`
#[cfg (feature = "use_gtk")]
include!("include/dispextern.rs");             // C 27: `#include "dispextern.h"`
#[cfg (feature = "use_gtk")]
include!("include/frame.rs");                  // C 28: `#include "frame.h"`
#[cfg (feature = "use_gtk")]
include!("include/keyboard.rs");               // C 29: `#include "keyboard.h"`
#[cfg (feature = "use_gtk")]
include!("include/gtkutil.rs");                // C 30: `#include "gtkutil.h"`
/* C 30's `"gtkutil.h"` chain: gtkutil.h 32's `#include "xterm.h"` (the non-PGTK arm) and
   gtkutil.h 29's `#include "pgtkterm.h"` (the PGTK arm).  `include/gtkutil.rs` is a placeholder
   (it carries the include guard alone), so the two edges stand here, at the C position that makes
   the names visible: this slice reads `FRAME_GTK_WIDGET`/`FRAME_GTK_OUTER_WIDGET`/`FRAME_X_OUTPUT`
   /`FRAME_DISPLAY_INFO` and `x_display_info`'s four `*_mod_mask` members through them.  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
include!("include/xterm.rs");                  // gtkutil.h 32: `#include "xterm.h"`
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // gtkutil.h 29: `#include "pgtkterm.h"`
#[cfg (feature = "use_gtk")]
include!("include/sysstdio.rs");               // C 31: `#include "sysstdio.h"`
#[cfg (feature = "use_gtk")]
include!("include/termhooks.rs");              // C 32: `#include "termhooks.h"`
#[cfg (feature = "use_gtk")]
include!("include/window.rs");                 // C 33: `#include "window.h"`
#[cfg (feature = "use_gtk")]
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
#[cfg (feature = "use_gtk")]
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
/// C: gtypes.h 52/55/56/60/61/109: `typedef char gchar;` / `typedef int gint;` / `typedef gint
/// gboolean;` / `typedef unsigned long gulong;` / `typedef unsigned int guint;` / `typedef void*
/// gpointer;`
#[cfg (feature = "use_gtk")]
pub type gchar = c_char;
#[cfg (feature = "use_gtk")]
pub type gint = c_int;
#[cfg (feature = "use_gtk")]
pub type gboolean = c_int;
#[cfg (feature = "use_gtk")]
pub type gulong = c_ulong;
#[cfg (feature = "use_gtk")]
pub type guint = c_uint;
#[cfg (feature = "use_gtk")]
pub type gpointer = *mut c_void;
/// C: glibconfig.h 55/81 (the generated header): `typedef unsigned int guint32;` / `typedef
/// unsigned long gsize;`
#[cfg (feature = "use_gtk")]
pub type guint32 = c_uint;
#[cfg (feature = "use_gtk")]
pub type gsize = c_ulong;
/// C: gmacros.h: `#define TRUE 1` / `#define FALSE 0`.  C 341/393/663 read `TRUE`.  Measured
/// `sizeof_gboolean=4` (Evidence ⑤).
#[cfg (feature = "use_gtk")]
pub const TRUE: gboolean = 1;
#[cfg (feature = "use_gtk")]
pub const FALSE: gboolean = 0;
/// C: glist.h 41-46: `struct _GList { gpointer data; GList *next; GList *prev; };`.  C 885
/// declares `GList *devices, *tem;` and reads `tem->data`, `tem->next`.  Measured
/// `sizeof_GList=24 data=0 next=8 prev=16` (Evidence ⑤).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct _GList
{
  pub data: gpointer,        // C: glist.h 43: `gpointer data;`
  pub next: *mut _GList,     // C: glist.h 44: `GList *next;`
  pub prev: *mut _GList,     // C: glist.h 45: `GList *prev;`
}
#[cfg (feature = "use_gtk")]
pub type GList = _GList;
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<GList> () == 24);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (GList, data) == 0);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (GList, next) == 8);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (GList, prev) == 16);
/// C: gslist.h 41-45: `struct _GSList { gpointer data; GSList *next; };`.  C 1132 declares
/// `GSList *filenames;` and `GSList *tem;`, and C 1167-1168 reads `tem->data`, `tem->next`.
/// Measured `sizeof_GSList=16 data=0 next=8` (Evidence ⑤).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct _GSList
{
  pub data: gpointer,        // C: gslist.h 43: `gpointer data;`
  pub next: *mut _GSList,    // C: gslist.h 44: `GSList *next;`
}
#[cfg (feature = "use_gtk")]
pub type GSList = _GSList;
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<GSList> () == 16);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (GSList, data) == 0);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (GSList, next) == 8);
/// C: gobject.h: `typedef struct _GObject GObject;` - C 437/660 hand its pointers to
/// `g_object_set_data`/`G_OBJECT_GET_CLASS`.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GObject
{
  _private: [u8; 0],
}
/// C: gclosure.h: `typedef struct _GClosure GClosure;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GClosure
{
  _private: [u8; 0],
}
/// C: gclosure.h: `typedef void (*GClosureNotify) (gpointer data, GClosure *closure);`
#[cfg (feature = "use_gtk")]
pub type GClosureNotify =
  Option<unsafe extern "C" fn (data: gpointer, closure: *mut GClosure)>;
/// C: gsignal.h 184-206: `typedef enum { G_CONNECT_DEFAULT = 0, G_CONNECT_AFTER = 1 << 0,
/// G_CONNECT_SWAPPED = 1 << 1 } GConnectFlags;`.  `g_signal_connect`'s own body passes
/// `(GConnectFlags) 0`.
#[cfg (feature = "use_gtk")]
pub type GConnectFlags = c_int;
/// C: gsignal.h: `typedef void (*GCallback) (void);`.  GTK's function-pointer convention is
/// `Option<unsafe extern "C" fn …>` (PORTING.md P-27), which is what `G_CALLBACK` produces.
#[cfg (feature = "use_gtk")]
pub type GCallback = Option<unsafe extern "C" fn ()>;
/// C: gobject.h 54: `#define G_OBJECT(object) (G_TYPE_CHECK_INSTANCE_CAST ((object),
/// G_TYPE_OBJECT, GObject))`, whose own body is `((GObject *) g_type_check_instance_cast
/// ((GTypeInstance *) (object), (GType) (g_type)))` (gtype.h 528/2668-2672).  The port keeps the
/// macro's name and its result type; GLib's checked-cast chain is recorded as a deviation (the
/// `src/pgtkterm.rs` 288-296 treatment of the same macro).
#[cfg (feature = "use_gtk")]
macro_rules! G_OBJECT {
  ($object:expr) => { ($object) as *mut GObject };
}
/// C: gtk/gtk.h: `#define G_CALLBACK(f) ((GCallback) (f))` - GTK's, not GLib's, so the citation
/// is the header and not a line.  C's cast reinterprets the function pointer; Rust refuses a
/// signature-changing `as` cast between fn pointers (`error: non-primitive cast`, measured in
/// `src/pgtkterm.rs`), so the port spells C's cast as the item address plus a pointer transmute,
/// and every callback C passes through it is declared `unsafe extern "C" fn` (PORTING.md 1.4).
#[cfg (feature = "use_gtk")]
macro_rules! G_CALLBACK {
  ($f:expr) => {
    Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn ()> ($f as *const ()) })
  };
}
/// C: gsignal.h 554-556: `#define g_signal_connect(instance, detailed_signal, c_handler, data)
/// g_signal_connect_data ((instance), (detailed_signal), (c_handler), (data), NULL,
/// (GConnectFlags) 0)` - GLib's own body.  C 361/427/429/431/442/446/452/457/461/465/471 are the
/// eleven call sites.
#[cfg (feature = "use_gtk")]
macro_rules! g_signal_connect {
  ($instance:expr, $detailed_signal:expr, $c_handler:expr, $data:expr) => {
    g_signal_connect_data ($instance as gpointer, $detailed_signal, $c_handler,
                           $data as gpointer, None, 0 as GConnectFlags)
  };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 361/427/429/431/442/446/452/457/461/465/471: `g_signal_connect (G_OBJECT! (ctx),
  /// "download-started", G_CALLBACK (webkit_download_cb), xw);`
  /// C: gsignal.h 440-446: `gulong g_signal_connect_data (gpointer instance, const gchar
  /// *detailed_signal, GCallback c_handler, gpointer data, GClosureNotify destroy_data,
  /// GConnectFlags connect_flags);`
  pub fn g_signal_connect_data (instance: gpointer, detailed_signal: *const gchar,
                                c_handler: GCallback, data: gpointer,
                                destroy_data: GClosureNotify,
                                connect_flags: GConnectFlags) -> gulong;
  /// C 1165's `alloca` is the port's stack pool (PORTING.md P-05, `src/editfns.rs` 2477) rather
  /// than this declaration; the name below is C 1175's `g_free`.
  /// C: gmem.h 74: `void (g_free) (gpointer mem);`
  pub fn g_free (mem: gpointer);
  /// C 906: `g_list_free (devices);`
  /// C: glist.h 53: `void g_list_free (GList *list);`
  pub fn g_list_free (list: *mut GList);
  /// C 1164: `len = g_slist_length (filenames);`
  /// C: gslist.h 125: `guint g_slist_length (GSList *list);`
  pub fn g_slist_length (list: *mut GSList) -> guint;
  /// C 1171: `g_slist_free (filenames);`
  /// C: gslist.h 52: `void g_slist_free (GSList *list);`
  pub fn g_slist_free (list: *mut GSList);
  /// C 359/393/? : `g_object_unref (ctx as gpointer);`
  /// C: gobject.h 515: `void g_object_unref (gpointer object);`
  pub fn g_object_unref (object: gpointer);
  /// C 220/638: `g_object_ref (eventcopy->any.window);` / `g_object_ref (xg_event->any.window);`
  /// C: gobject.h 513: `gpointer g_object_ref (gpointer object);`
  pub fn g_object_ref (object: gpointer) -> gpointer;
  /// C 393/394: `g_object_set (G_OBJECT! (settings), "enable-developer-extras", TRUE, NULL);`
  /// C: gobject.h 455-457: `void g_object_set (gpointer object, const gchar *first_property_name,
  /// ...);`
  pub fn g_object_set (object: gpointer, first_property_name: *const gchar, ...);
  /// C 436/437: `g_object_set_data (G_OBJECT! (xw->widget_osr), XG_XWIDGET, xw);`
  /// C: gobject.h 597-599: `void g_object_set_data (GObject *object, const gchar *key, gpointer
  /// data);`
  pub fn g_object_set_data (object: *mut GObject, key: *const gchar, data: gpointer);
}

/* ----------------------------------------------------------------------------------------------
   GDK.  No GDK tree on this machine, so each citation names the header alone and every record
   below is either the never-dereferenced `[u8; 0]` shape (`src/xfns.rs` 1004-1009's `GtkWidget`)
   or a record carrying exactly the members C reads, with its layout **UNVERIFIED** (P-88 ②; the
   `src/gtkutil.rs` 494-561 treatment of the same surface, owed to the same mirror).
   ---------------------------------------------------------------------------------------------- */
/// C: gdk/gdkwindow.h: `typedef struct _GdkWindow GdkWindow;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GdkWindow
{
  _private: [u8; 0],
}
/// C: gdk/gdkdevice.h: `typedef struct _GdkDevice GdkDevice;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GdkDevice
{
  _private: [u8; 0],
}
/// C: gdk/gdkseat.h: `typedef struct _GdkSeat GdkSeat;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GdkSeat
{
  _private: [u8; 0],
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_NOTHING = -1, GDK_DELETE, GDK_DESTROY, GDK_EXPOSE,
/// GDK_MOTION_NOTIFY, GDK_BUTTON_PRESS, GDK_2BUTTON_PRESS, GDK_DOUBLE_BUTTON_PRESS,
/// GDK_3BUTTON_PRESS, …, GDK_ENTER_NOTIFY, …, GDK_LEAVE_NOTIFY, …, GDK_SCROLL, …, GDK_KEY_PRESS,
/// …, GDK_KEY_RELEASE, … } GdkEventType;` - the type C 141's `switch (event->type)` reads.  C's
/// enum ABI is `c_int` (PORTING.md 1.3); no layout assertion is written and the values below are
/// placeholders (see the enumerator note).
#[cfg (feature = "use_gtk")]
pub type GdkEventType = c_int;
/// C: gdk/gdktypes.h: `typedef union _GdkEvent GdkEvent;` - the members C reads.  GDK declares
/// thirteen union members; the port carries the six the reference's text touches (`any`,
/// `button`, `scroll`, `motion`, `crossing`, `key`) plus the bare `type` member C 680 writes.
/// **UNVERIFIED**: no GDK header on this machine.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventAny
{
  pub r#type: GdkEventType,        // C: gdk/gdktypes.h: `GdkEventType type;`
  pub window: *mut GdkWindow,      // C: gdk/gdktypes.h: `GdkWindow *window;`
  pub send_event: i8,              // C: gdk/gdktypes.h: `gint8 send_event;`
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventButton GdkEventButton;` - C 148-155 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventButton
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,                 // C: gdk/gdktypes.h: `gdouble x;`
  pub y: c_double,                 // C: gdk/gdktypes.h: `gdouble y;`
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventScroll GdkEventScroll;` - C 161-168 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventScroll
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventMotion GdkEventMotion;` - C 174-181 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventMotion
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventCrossing GdkEventCrossing;` - C 188-195 reads and
/// writes `x`/`y`; **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventCrossing
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub subwindow: *mut GdkWindow,
  pub time: u32,
  pub x: c_double,
  pub y: c_double,
}
/// C: gdk/gdktypes.h: `typedef struct _GdkEventKey GdkEventKey;` - C 643-648 writes `keyval` and
/// `state`; **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GdkEventKey
{
  pub r#type: GdkEventType,
  pub window: *mut GdkWindow,
  pub send_event: i8,
  pub time: u32,
  pub state: c_uint,               // C: gdk/gdktypes.h: `guint state;`
  pub keyval: c_uint,              // C: gdk/gdktypes.h: `guint keyval;`
}
#[cfg (feature = "use_gtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub union GdkEvent
{
  pub r#type: GdkEventType,        // C: gdk/gdktypes.h: `GdkEventType type;`
  pub any: GdkEventAny,            // C: gdk/gdktypes.h: `GdkEventAny any;`
  pub button: GdkEventButton,      // C: gdk/gdktypes.h: `GdkEventButton button;`
  pub scroll: GdkEventScroll,      // C: gdk/gdktypes.h: `GdkEventScroll scroll;`
  pub motion: GdkEventMotion,      // C: gdk/gdktypes.h: `GdkEventMotion motion;`
  pub crossing: GdkEventCrossing,  // C: gdk/gdktypes.h: `GdkEventCrossing crossing;`
  pub key: GdkEventKey,            // C: gdk/gdktypes.h: `GdkEventKey key;`
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_CROSSING_NORMAL, GDK_CROSSING_SELECTION,
/// GDK_CROSSING_GTK_GRAB, GDK_CROSSING_GTK_UNGRAB, GDK_CROSSING_STATE_CHANGED,
/// GDK_CROSSING_TOUCH_BEGIN, GDK_CROSSING_TOUCH_END, GDK_CROSSING_DEVICE_SWITCH } GdkCrossingMode;`
/// - the type C 271/275 declare parameters of.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GdkCrossingMode = c_int;
/// C: gdk/gdktypes.h: `typedef enum { GDK_SHIFT_MASK = 1 << 0, GDK_LOCK_MASK = 1 << 1,
/// GDK_CONTROL_MASK = 1 << 2, GDK_MOD1_MASK = 1 << 3, … GDK_SUPER_MASK = 1 << 26,
/// GDK_HYPER_MASK = 1 << 27, GDK_META_MASK = 1 << 28 } GdkModifierType;` - the six C 249-263 ORs
/// into `mods`.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GdkModifierType = c_int;
/// C: gdk/gdkseat.h: `typedef enum { GDK_SEAT_CAPABILITY_NONE, GDK_SEAT_CAPABILITY_POINTER,
/// GDK_SEAT_CAPABILITY_TOUCH, GDK_SEAT_CAPABILITY_TABLET_STYLUS, GDK_SEAT_CAPABILITY_KEYBOARD,
/// GDK_SEAT_CAPABILITY_TABLET_PAD, GDK_SEAT_CAPABILITY_ALL_POINTING, GDK_SEAT_CAPABILITY_ALL }
/// GdkSeatCapabilities;` - the flag C 891 asks `gdk_seat_get_slaves` for.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GdkSeatCapabilities = c_uint;
/// C: gdk/gdkdevice.h: `typedef enum { GDK_SOURCE_MOUSE, GDK_SOURCE_PEN, GDK_SOURCE_ERASER,
/// GDK_SOURCE_CURSOR, GDK_SOURCE_KEYBOARD, GDK_SOURCE_TOUCHSCREEN, GDK_SOURCE_TOUCHPAD,
/// GDK_SOURCE_TRACKPOINT, GDK_SOURCE_TABLET_PAD } GdkInputSource;` - the type
/// `gdk_device_get_source` returns, compared at C 901.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GdkInputSource = c_int;
/// C: gtk/gtkwidget.h / gdk/gdkdevice.h / gdk/gdkseat.h: `#define GDK_DEVICE(object)
/// (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DEVICE, GdkDevice))` - C 899's cast keeps the
/// macro's name and its result type; GDK's cast chain is GLib's and owed (as above).
#[cfg (feature = "use_gtk")]
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
#[cfg (feature = "use_gtk")]
pub const GDK_NOTHING: GdkEventType = 0;              // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_BUTTON_PRESS: GdkEventType = 1;         // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_BUTTON_RELEASE: GdkEventType = 2;       // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_2BUTTON_PRESS: GdkEventType = 3;        // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_3BUTTON_PRESS: GdkEventType = 4;        // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_SCROLL: GdkEventType = 5;               // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_MOTION_NOTIFY: GdkEventType = 6;        // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_ENTER_NOTIFY: GdkEventType = 7;         // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_LEAVE_NOTIFY: GdkEventType = 8;         // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_KEY_PRESS: GdkEventType = 9;            // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_KEY_RELEASE: GdkEventType = 10;         // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_MOD1_MASK: GdkModifierType = 1 << 0;    // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_META_MASK: GdkModifierType = 1 << 1;    // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_SUPER_MASK: GdkModifierType = 1 << 2;   // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_HYPER_MASK: GdkModifierType = 1 << 3;   // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_CONTROL_MASK: GdkModifierType = 1 << 4; // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_SHIFT_MASK: GdkModifierType = 1 << 5;   // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
pub const GDK_SEAT_CAPABILITY_ALL_POINTING: GdkSeatCapabilities = 1 << 0;  // placeholder
#[cfg (feature = "use_gtk")]
pub const GDK_SOURCE_TOUCHPAD: GdkInputSource = 0;    // placeholder, not GDK's value
#[cfg (feature = "use_gtk")]
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
  /// C 1106: `gdk_window_set_cursor (wdesc, xvw->cursor);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_cursor (GdkWindow *window, GdkCursor *cursor);`
  pub fn gdk_window_set_cursor (window: *mut GdkWindow, cursor: *mut GdkCursor);
  /// C 883/914: `gdk_display_get_default_seat (gtk_widget_get_display (FRAME_GTK_WIDGET (f)))`
  /// C: gdk/gdkdisplay.h: `GdkSeat *gdk_display_get_default_seat (GdkDisplay *display);`
  pub fn gdk_display_get_default_seat (display: *mut GdkDisplay) -> *mut GdkSeat;
  /// C 891: `devices = gdk_seat_get_slaves (seat, GDK_SEAT_CAPABILITY_ALL_POINTING);`
  /// C: gdk/gdkseat.h: `GList *gdk_seat_get_slaves (GdkSeat *seat, GdkSeatCapabilities
  /// capabilities);`
  pub fn gdk_seat_get_slaves (seat: *mut GdkSeat, capabilities: GdkSeatCapabilities)
    -> *mut GList;
  /// C 908: `return !tem ? gdk_seat_get_pointer (seat) : device;`
  /// C: gdk/gdkseat.h: `GdkDevice *gdk_seat_get_pointer (GdkSeat *seat);`
  pub fn gdk_seat_get_pointer (seat: *mut GdkSeat) -> *mut GdkDevice;
  /// C 920: `return gdk_seat_get_keyboard (seat);`
  /// C: gdk/gdkseat.h: `GdkDevice *gdk_seat_get_keyboard (GdkSeat *seat);`
  pub fn gdk_seat_get_keyboard (seat: *mut GdkSeat) -> *mut GdkDevice;
  /// C 901: `if (gdk_device_get_source (device) == GDK_SOURCE_TOUCHPAD)`
  /// C: gdk/gdkdevice.h: `GdkInputSource gdk_device_get_source (GdkDevice *device);`
  pub fn gdk_device_get_source (device: *mut GdkDevice) -> GdkInputSource;
  /// C 641: `keycode = gdk_unicode_to_keyval (character);`
  /// C: gdk/gdkkeyuni.h: `guint gdk_unicode_to_keyval (guint32 wc);`
  pub fn gdk_unicode_to_keyval (wc: guint32) -> guint;
  /// C 546/629/688/779: `gdk_offscreen_window_set_embedder (osw, embedder);`
  /// C: gdk/gdkwindow.h: `void gdk_offscreen_window_set_embedder (GdkWindow *window, GdkWindow
  /// *embedder);`
  pub fn gdk_offscreen_window_set_embedder (window: *mut GdkWindow, embedder: *mut GdkWindow);
}
/// C: gdk/gdkdisplay.h: `typedef struct _GdkDisplay GdkDisplay;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GdkDisplay
{
  _private: [u8; 0],
}
/// C: gdk/gdkcursor.h: `typedef struct _GdkCursor GdkCursor;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GdkCursor
{
  _private: [u8; 0],
}

/* ----------------------------------------------------------------------------------------------
   GTK.  Same treatment as GDK above: no GTK tree on this machine, so each citation names the
   header alone, the `GtkWidget`-family pointees are the `[u8; 0]` shape, and the records C reads
   members of carry exactly those members with their layout **UNVERIFIED** (P-88 ②; owed).
   ---------------------------------------------------------------------------------------------- */
/// C: gtk/gtkwidget.h: `typedef struct _GtkWidget GtkWidget;` (`G_DECLARE_DERIVABLE_TYPE` in GTK3).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkWidget
{
  _private: [u8; 0],
}
/// C: gtk/gtkwindow.h: `typedef struct _GtkWindow GtkWindow;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkWindow
{
  _private: [u8; 0],
}
/// C: gtk/gtkcontainer.h: `typedef struct _GtkContainer GtkContainer;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkContainer
{
  _private: [u8; 0],
}
/// C: gtk/gtktextview.h: `typedef struct _GtkTextView GtkTextView;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkTextView
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilechoosernative.h: `typedef struct _GtkFileChooserNative GtkFileChooserNative;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkFileChooserNative
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilefilter.h: `typedef struct _GtkFileFilter GtkFileFilter;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkFileFilter
{
  _private: [u8; 0],
}
/// C: gtk/gtkfilechooser.h: `typedef struct _GtkFileChooser GtkFileChooser;` (an interface;
/// `GTK_FILE_CHOOSER` casts to it).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkFileChooser
{
  _private: [u8; 0],
}
/// C: gtk/gtknativedialog.h: `typedef struct _GtkNativeDialog GtkNativeDialog;` (a class in GTK3;
/// `GTK_NATIVE_DIALOG` casts to it).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkNativeDialog
{
  _private: [u8; 0],
}
/// C: gtk/gtktypes.h: `typedef struct { gint x, y; gint width, height; } GtkAllocation;`.  C 933
/// declares one and C 938-988 reads and writes all four members; **UNVERIFIED** (the
/// `src/gtkutil.rs` 794-801 declaration of the same record, whose size/offsets that ledger owes
/// too).
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkAllocation
{
  pub x: gint,          // C: gtk/gtktypes.h: `gint x;`
  pub y: gint,          // C: gtk/gtktypes.h: `gint y;`
  pub width: gint,      // C: gtk/gtktypes.h: `gint width;`
  pub height: gint,     // C: gtk/gtktypes.h: `gint height;`
}
/// C: gtk/gtkcontainer.h: `typedef void (*GtkCallback) (GtkWidget *widget, gpointer data);` - the
/// type of `gtk_container_forall`'s second parameter (C 1002/662).
#[cfg (feature = "use_gtk")]
pub type GtkCallback = Option<unsafe extern "C" fn (widget: *mut GtkWidget, data: gpointer)>;
/// C: gtk/gtkcontainer.h: `struct _GtkContainerClass { GtkWidgetClass parent_class; … void
/// (*forall) (GtkContainer *container, gboolean include_internals, GtkCallback callback, gpointer
/// callback_data); … };` - C 660-663 reads `forall` through `GTK_CONTAINER_CLASS`.  The port
/// carries that one member and owes the record's other members and the offset (`src/xfns.rs`
/// 1034-1045's `GtkWidgetClass` precedent).  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct GtkContainerClass
{
  pub forall: GtkContainerClass_forall,  // C: gtk/gtkcontainer.h: the four-parameter member
}
#[cfg (feature = "use_gtk")]
pub type GtkContainerClass_forall =
  Option<unsafe extern "C" fn (container: *mut GtkContainer, include_internals: gboolean,
                               callback: GtkCallback, callback_data: gpointer)>;
/// C: gtk/gtktypes.h / gtk/gtkenums.h: `GtkResponseType` and `GtkFileChooserAction` - the types
/// of C 1153/1155's `response` and C 1145's action argument.  C's enum ABI is `c_int`
/// (PORTING.md 1.3); **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type GtkResponseType = c_int;
#[cfg (feature = "use_gtk")]
pub type GtkFileChooserAction = c_int;
/// C: gtk/gtk.h: the checked-cast wrappers C 345/347/406/408/412/413/417/549/660/1144/1146/
/// 1150/1152/1153/1157/1163 uses, each keeping the macro's name and its result type (the GLib
/// cast chains are owed, as above).
#[cfg (feature = "use_gtk")]
macro_rules! GTK_WIDGET {
  ($o:expr) => { ($o) as *mut GtkWidget };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_WINDOW {
  ($o:expr) => { ($o) as *mut GtkWindow };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_CONTAINER {
  ($o:expr) => { ($o) as *mut GtkContainer };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_CONTAINER_CLASS {
  ($o:expr) => { ($o) as *mut GtkContainerClass };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_FILE_CHOOSER {
  ($o:expr) => { ($o) as *mut GtkFileChooser };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_NATIVE_DIALOG {
  ($o:expr) => { ($o) as *mut GtkNativeDialog };
}
/// C: gtk/gtk.h: the four type-check macros C 499/655/665/992 asks, each keeping the macro's name
/// and its result type (`gboolean`).  C's bodies are `G_TYPE_CHECK_INSTANCE_TYPE` chains, which
/// need the type system the port has not got (the `src/gtkutil.rs` 876-893 treatment).
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IS_TEXT_VIEW {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IS_WIDGET {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IS_CONTAINER {
  ($o:expr) => { (! ($o).is_null ()) };
}
/// C: gtk/gtkenums.h: `GTK_RESPONSE_ACCEPT` (C 1155) and `GTK_FILE_CHOOSER_ACTION_OPEN` (C 1145).
/// Their values are GTK's; placeholders here (the enumerator note above).  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub const GTK_RESPONSE_ACCEPT: GtkResponseType = 0;               // placeholder, not GTK's value
#[cfg (feature = "use_gtk")]
pub const GTK_FILE_CHOOSER_ACTION_OPEN: GtkFileChooserAction = 0; // placeholder, not GTK's value
/// C: gobject/gobject.h: `#define G_OBJECT_GET_CLASS(object) (G_TYPE_INSTANCE_GET_CLASS ((object),
/// G_TYPE_OBJECT, GObjectClass))` - C 660's cast; the port keeps the macro's name and its result
/// type (`gpointer`), and GLib's chain is owed (as above).
#[cfg (feature = "use_gtk")]
macro_rules! G_OBJECT_GET_CLASS {
  ($o:expr) => { ($o) as gpointer };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 153/166/179/193/? : `event->any.window = gtk_widget_get_window (widget);`
  /// C: gtk/gtkwidget.h: `GdkWindow *gtk_widget_get_window (GtkWidget *widget);`
  pub fn gtk_widget_get_window (widget: *mut GtkWidget) -> *mut GdkWindow;
  /// C 752: `widget = gtk_get_event_widget (&event);`
  /// C: gtk/gtkmain.h: `GtkWidget *gtk_get_event_widget (GdkEvent *event);`
  pub fn gtk_get_event_widget (event: *mut GdkEvent) -> *mut GtkWidget;
  /// C 830-832/867-869/1036-1037/1053-1054: `gtk_widget_translate_coordinates (…, &xoff, &yoff);`
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_translate_coordinates (GtkWidget *src_widget,
  /// GtkWidget *dest_widget, int src_x, int src_y, int *dest_x, int *dest_y);`
  pub fn gtk_widget_translate_coordinates (src_widget: *mut GtkWidget, dest_widget: *mut GtkWidget,
                                           src_x: c_int, src_y: c_int, dest_x: *mut c_int,
                                           dest_y: *mut c_int) -> gboolean;
  /// C 938: `gtk_widget_get_allocation (widget, &new_allocation);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_get_allocation (GtkWidget *widget, GtkAllocation
  /// *allocation);`
  pub fn gtk_widget_get_allocation (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
  /// C 940: `if (gtk_widget_get_has_window (widget))`
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_get_has_window (GtkWidget *widget);`
  pub fn gtk_widget_get_has_window (widget: *mut GtkWidget) -> gboolean;
  /// C 946/949: `if (gtk_widget_get_parent (widget) && …)`
  /// C: gtk/gtkwidget.h: `GtkWidget *gtk_widget_get_parent (GtkWidget *widget);`
  pub fn gtk_widget_get_parent (widget: *mut GtkWidget) -> *mut GtkWidget;
  /// C 884/915: `gtk_widget_get_display (FRAME_GTK_WIDGET (f))`
  /// C: gtk/gtkwidget.h: `GdkDisplay *gtk_widget_get_display (GtkWidget *widget);`
  pub fn gtk_widget_get_display (widget: *mut GtkWidget) -> *mut GdkDisplay;
  /// C 667/1103: `if (!gtk_widget_get_realized (temp))` / `if (gtk_widget_get_realized (xvw->widget))`
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_get_realized (GtkWidget *widget);`
  pub fn gtk_widget_get_realized (widget: *mut GtkWidget) -> gboolean;
  /// C 406: `gtk_widget_set_size_request (GTK_WIDGET (xw->widget_osr), xw->width, xw->height);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_size_request (GtkWidget *widget, gint width, gint
  /// height);`
  pub fn gtk_widget_set_size_request (widget: *mut GtkWidget, width: gint, height: gint);
  /// C 408: `gtk_widget_queue_allocate (GTK_WIDGET (xw->widget_osr));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_queue_allocate (GtkWidget *widget);`
  pub fn gtk_widget_queue_allocate (widget: *mut GtkWidget);
  /// C 421/422: `gtk_widget_show (xw->widget_osr);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_show (GtkWidget *widget);`
  pub fn gtk_widget_show (widget: *mut GtkWidget);
  /// C 668: `gtk_widget_realize (temp);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_realize (GtkWidget *widget);`
  pub fn gtk_widget_realize (widget: *mut GtkWidget);
  /// C 222/679/681: `gtk_main_do_event (eventcopy);`
  /// C: gtk/gtkmain.h: `void gtk_main_do_event (GdkEvent *event);`
  pub fn gtk_main_do_event (event: *mut GdkEvent);
  /// C 1001-1002: `gtk_container_forall (GTK_CONTAINER (widget), find_widget_cb, &new_data);`
  /// C: gtk/gtkcontainer.h: `void gtk_container_forall (GtkContainer *container, GtkCallback
  /// callback, gpointer callback_data);`
  pub fn gtk_container_forall (container: *mut GtkContainer, callback: GtkCallback,
                               callback_data: gpointer);
  /// C 412/417: `gtk_container_add (GTK_CONTAINER (xw->widgetwindow_osr), …);`
  /// C: gtk/gtkcontainer.h: `void gtk_container_add (GtkContainer *container, GtkWidget *widget);`
  pub fn gtk_container_add (container: *mut GtkContainer, widget: *mut GtkWidget);
  /// C 347: `gtk_container_check_resize (GTK_CONTAINER (xw->widgetwindow_osr));`
  /// C: gtk/gtkcontainer.h: `void gtk_container_check_resize (GtkContainer *container);`
  pub fn gtk_container_check_resize (container: *mut GtkContainer);
  /// C 549: `widget = gtk_window_get_focus (GTK_WINDOW (xw->widgetwindow_osr));`
  /// C: gtk/gtkwindow.h: `GtkWidget *gtk_window_get_focus (GtkWindow *window);`
  pub fn gtk_window_get_focus (window: *mut GtkWindow) -> *mut GtkWidget;
  /// C 345: `gtk_window_resize (GTK_WINDOW (xw->widgetwindow_osr), xw->width, xw->height);`
  /// C: gtk/gtkwindow.h: `void gtk_window_resize (GtkWindow *window, gint width, gint height);`
  pub fn gtk_window_resize (window: *mut GtkWindow, width: gint, height: gint);
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
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitWebView
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitWebContext.h: `typedef struct _WebKitWebContext WebKitWebContext;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitWebContext
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitSettings.h: `typedef struct _WebKitSettings WebKitSettings;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitSettings
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitHitTestResult.h: `typedef struct _WebKitHitTestResult WebKitHitTestResult;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitHitTestResult
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitFileChooserRequest.h: `typedef struct _WebKitFileChooserRequest
/// WebKitFileChooserRequest;`
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitFileChooserRequest
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitScriptDialog.h: `typedef struct _WebKitScriptDialog WebKitScriptDialog;` -
/// the type C 71's forward declaration names.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitScriptDialog
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitPolicyDecision.h: `typedef struct _WebKitPolicyDecision
/// WebKitPolicyDecision;` - the type C 109's forward declaration names.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitPolicyDecision
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitNavigationAction.h: `typedef struct _WebKitNavigationAction
/// WebKitNavigationAction;` - the type C 106's forward declaration names.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitNavigationAction
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitDownload.h: `typedef struct _WebKitDownload WebKitDownload;` - the type
/// C 105's forward declaration names.
#[cfg (feature = "use_gtk")]
#[repr (C)]
pub struct WebKitDownload
{
  _private: [u8; 0],
}
/// C: webkit2/WebKitPolicyDecision.h: `typedef enum { WEBKIT_POLICY_DECISION_TYPE_NAVIGATION_ACTION,
/// WEBKIT_POLICY_DECISION_TYPE_NEW_WINDOW_ACTION, WEBKIT_POLICY_DECISION_TYPE_RESPONSE }
/// WebKitPolicyDecisionType;` - the type C 110's forward declaration names.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type WebKitPolicyDecisionType = c_int;
/// C: webkit2/WebKitWebView.h: `typedef enum { WEBKIT_LOAD_STARTED, WEBKIT_LOAD_REDIRECTED,
/// WEBKIT_LOAD_COMMITTED, WEBKIT_LOAD_FINISHED } WebKitLoadEvent;` - the type C 100's forward
/// declaration names.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type WebKitLoadEvent = c_int;
/// C: webkit2/WebKitSettings.h: `typedef enum { WEBKIT_HARDWARE_ACCELERATION_POLICY_ON_DEMAND,
/// WEBKIT_HARDWARE_ACCELERATION_POLICY_ALWAYS, WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER }
/// WebKitHardwareAccelerationPolicy;` - the type of C 402's argument.  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub type WebKitHardwareAccelerationPolicy = c_int;
/// C: webkit2/WebKitWebView.h: `#define WEBKIT_WEB_VIEW(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj),
/// WEBKIT_TYPE_WEB_VIEW, WebKitWebView))` - C 380/387/392/413's casts keep the macro's name and
/// its result type; WebKit's cast chain is GLib's and owed (as above).
#[cfg (feature = "use_gtk")]
macro_rules! WEBKIT_WEB_VIEW {
  ($o:expr) => { ($o) as *mut WebKitWebView };
}
/// C: webkit2/WebKitWebView.h: `#define WEBKIT_IS_WEB_VIEW(obj) (G_TYPE_CHECK_INSTANCE_TYPE
/// ((obj), WEBKIT_TYPE_WEB_VIEW))` - C 655's test; the port keeps the macro's name and its result
/// type (`gboolean`), the `GTK_IS_*` treatment above.
#[cfg (feature = "use_gtk")]
macro_rules! WEBKIT_IS_WEB_VIEW {
  ($o:expr) => { (! ($o).is_null ()) };
}
/// C: webkit2/WebKitHitTestResult.h: `WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT`,
/// `…_LINK`, `…_IMAGE`, `…_MEDIA`, `…_EDITABLE`, `…_SELECTION`, `…_SCROLLBAR` - the six C
/// 1070-1078 tests.  Bit flags in WebKit's header; the values are WebKit's and are placeholders
/// here (the enumerator note above).  **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE: guint = 1 << 0;   // placeholder
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION: guint = 1 << 1;  // placeholder
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT: guint = 1 << 2;   // placeholder
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR: guint = 1 << 3;  // placeholder
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK: guint = 1 << 4;       // placeholder
/// C: webkit2/WebKitSettings.h: `WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER` (C 402).
/// **UNVERIFIED**.
#[cfg (feature = "use_gtk")]
pub const WEBKIT_HARDWARE_ACCELERATION_POLICY_NEVER:
  WebKitHardwareAccelerationPolicy = 0;                              // placeholder
#[cfg (feature = "use_gtk")]
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
}

/* C 56-59: the file's four file-scope objects.  The first three are C's tentative definitions
   (zero words, i.e. `Qnil`); `xwidget_counter`'s initializer is C's own.  */
#[cfg (feature = "use_gtk")]
static mut id_to_xwidget_map: Lisp_Object = lisp_h_Qnil ();           // C 56 (tentative, i.e. Qnil)
#[cfg (feature = "use_gtk")]
static mut internal_xwidget_view_list: Lisp_Object = lisp_h_Qnil ();  // C 57 (tentative, i.e. Qnil)
#[cfg (feature = "use_gtk")]
static mut internal_xwidget_list: Lisp_Object = lisp_h_Qnil ();       // C 58 (tentative, i.e. Qnil)
#[cfg (feature = "use_gtk")]
static mut xwidget_counter: u32 = 0;                                  // C 59

/* C 61-77: `#ifdef USE_GTK` with C 62's `#ifdef HAVE_X_WINDOWS` and C 64's
   `#if WEBKIT_CHECK_VERSION (2, 34, 0)` inside it.  C 68-76 are seven forward declarations;
   C's prototype has no Rust counterpart when the definition follows (PORTING.md 0.1-12), so they
   are dropped and the definitions stay at C's position - `record_osr_embedder` (C 766),
   `from_embedder` (C 806), `to_embedder` (C 843), `find_suitable_keyboard` (C 911) and
   `pick_embedded_child` (C 740) are inside this slice; `offscreen_damage_event` (C 2247),
   `synthesize_focus_in_event` (C 2110) and `webkit_script_dialog_cb` (C 2647) are in the
   remainder and stay as C's call sites (the `docs/ports/gtkutil.md` Deviations-3 shape).  */
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
static mut x_window_to_xwv_map: Lisp_Object = lisp_h_Qnil ();         // C 63 (tentative, i.e. Qnil)
#[cfg (all (feature = "use_gtk", feature = "have_x_windows",
            feature = "webkit_check_version_ge_2_34"))]
static mut dummy_tooltip_string: Lisp_Object = lisp_h_Qnil ();        // C 65 (tentative, i.e. Qnil)

/* C 79-83: `ALLOCATE_PSEUDOVECTOR` expands to a `allocate_pseudovector` call with `VECSIZE`/
   `PSEUDOVECSIZE` of the record's marker field; `include/lisp.rs` carries the macro.  */
#[cfg (feature = "use_gtk")]
unsafe fn allocate_xwidget () -> *mut xwidget                         // C 79-83
{
  return ALLOCATE_PSEUDOVECTOR! (xwidget, script_callbacks, pvec_type::PVEC_XWIDGET);
}

#[cfg (feature = "use_gtk")]
unsafe fn allocate_xwidget_view () -> *mut xwidget_view               // C 85-89
{
  return ALLOCATE_PSEUDOVECTOR! (xwidget_view, w, pvec_type::PVEC_XWIDGET_VIEW);
}

/* C 91-92: the two setters.  C writes them as macros over `XSETPSEUDOVECTOR`; the port keeps the
   names and the bodies (PORTING.md 0.2).  */
/// C: #define XSETXWIDGET(a, b) XSETPSEUDOVECTOR (a, b, PVEC_XWIDGET)
#[cfg (feature = "use_gtk")]
macro_rules! XSETXWIDGET {
  ($a:expr, $b:expr) => { XSETPSEUDOVECTOR! ($a, $b, pvec_type::PVEC_XWIDGET) };
}
/// C: #define XSETXWIDGET_VIEW(a, b) XSETPSEUDOVECTOR (a, b, PVEC_XWIDGET_VIEW)
#[cfg (feature = "use_gtk")]
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
#[cfg (feature = "use_gtk")]
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
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
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

#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
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
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub type xw_crossing_mode = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub const XW_CROSSING_LEFT: xw_crossing_mode = 0;
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub const XW_CROSSING_ENTERED: xw_crossing_mode = 1;
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub const XW_CROSSING_NONE: xw_crossing_mode = 2;

/* C 260/262: the two X11 masks this slice reads.  X11 *is* on this machine, so the values are the
   header's own text (`X11/X.h` 221/223); the mirror `include/xlib.rs` does not carry them
   (P-72 rule 4; the two rows are owed there).  */
/// C: X11/X.h 221: `#define ShiftMask (1<<0)`
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub const ShiftMask: c_uint = 1 << 0;
/// C: X11/X.h 223: `#define ControlMask (1<<2)`
#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
pub const ControlMask: c_uint = 1 << 2;

#[cfg (all (feature = "use_gtk", feature = "have_x_windows"))]
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
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub static mut xg_gtk_initialized: bool;
}

#[cfg (feature = "use_gtk")]
DEFUN! ("make-xwidget", Fmake_xwidget, Smake_xwidget, 4, 7, 0,
       doc: "Make an xwidget of TYPE.\nIf BUFFER is nil, use the current buffer.\nIf BUFFER is a string and no such buffer exists, create it.\nTYPE is a symbol which can take one of the following values:\n\n- webkit\n\nRELATED is nil, or an xwidget.  When constructing a WebKit widget, it\nwill share the same settings and internal subprocess as RELATED.\nReturns the newly constructed xwidget, or nil if construction\nfails.",
       (r#type: Lisp_Object, title: Lisp_Object, width: Lisp_Object, height: Lisp_Object,
        arguments: Lisp_Object, buffer: Lisp_Object, related: Lisp_Object),
       {
  #[cfg (feature = "use_gtk")]
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

  #[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
DEFUN! ("xwidget-live-p", Fxwidget_live_p, Sxwidget_live_p, 1, 1, 0,
       doc: "Return t if OBJECT is an xwidget that has not been killed.\nValue is nil if OBJECT is not an xwidget or if it has been killed.",
       (object: Lisp_Object),
       {
  return (if XWIDGETP (object)
	    && ! NILP ((*XXWIDGET (object)).buffer)
	  { Qt () } else { Qnil () });
});

/* C 493-502: `#ifdef USE_GTK` - the `forall` callback C 662 hands to `gtk_container_forall`.  */
#[cfg (feature = "use_gtk")]
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
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}
/// C: keyboard.h 461: `extern Lisp_Object parse_modifiers (Lisp_Object);`
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn parse_modifiers (symbol: Lisp_Object) -> Lisp_Object;
}
/// C: keyboard.h 525: `extern const char *const lispy_function_keys[];` - C's incomplete array.
/// `src/keyboard.rs` 5975 *defines* the 256-slot X11 table (C 5513's arm, the one this TU's
/// configuration compiles), so the port declares those 256 slots rather than an incomplete array
/// the loop at C 592-602 could not index; the definition there carries no `#[unsafe(no_mangle)]`
/// yet, which a real GTK build needs (Owed 3).
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub static mut lispy_function_keys: [*const c_char; 256];
}
/// C: xterm.h 1809: `extern int x_emacs_to_x_modifiers (struct x_display_info *, intmax_t);`
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" {
  pub fn x_emacs_to_x_modifiers (dpyinfo: *mut x_display_info, modifiers: intmax_t) -> c_int;
}
/// C: pgtkterm.h 678: `extern int pgtk_emacs_to_gtk_modifiers (struct pgtk_display_info *, int);`
/// (`pgtk_display_info` itself is in `include/pgtkterm.rs`'s placeholder - the recorded mirror
/// gap, Deviations 6).
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
unsafe extern "C" {
  pub fn pgtk_emacs_to_gtk_modifiers (dpyinfo: *mut pgtk_display_info, modifiers: c_int)
    -> c_int;
}

/// C: process.h 303: `extern void catch_child_signal (void);` - `include/process.rs` does not
/// carry it (P-72 rule 4), and `src/process.rs` 8661 defines it without `#[unsafe(no_mangle)]`
/// yet, which a real GTK build needs (Owed 3).
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  pub fn catch_child_signal ();
}

#[cfg (feature = "use_gtk")]
DEFUN! ("xwidget-perform-lispy-event",
       Fxwidget_perform_lispy_event, Sxwidget_perform_lispy_event, 2, 3, 0,
       doc: "Send a lispy event to XWIDGET.\nEVENT should be the event that will be sent.  FRAME should be the\nframe which generated the event, and defaults to the selected frame.\nOn X11, modifier keys will not be processed if FRAME is nil and the\nselected frame is not an X-Windows frame.",
       (xwidget: Lisp_Object, event: Lisp_Object, frame: Lisp_Object),
       {
  #[cfg (feature = "use_gtk")]
  let mut f: *mut frame = std::ptr::null_mut ();
  #[cfg (feature = "use_gtk")]
  let mut xg_event: *mut GdkEvent;
  #[cfg (feature = "use_gtk")]
  let mut klass: *mut GtkContainerClass;
  #[cfg (feature = "use_gtk")]
  let mut widget: *mut GtkWidget;
  #[cfg (feature = "use_gtk")]
  let mut temp: *mut GtkWidget = std::ptr::null_mut ();
  #[cfg (all (feature = "use_gtk", feature = "have_xinput2"))]
  let mut embedder: *mut GdkWindow;
  #[cfg (all (feature = "use_gtk", feature = "have_xinput2"))]
  let mut osw: *mut GdkWindow;

  CHECK_LIVE_XWIDGET (xwidget);

  #[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
unsafe fn xwidget_hidden (xv: *mut xwidget_view) -> bool                 // C 721-725
{
  return (*xv).hidden;
}

/* C 727-737: `xwidget_from_id` is one of the file's non-`static` definitions (xwidget.h 206
   declares it `extern`), so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2); `include/xwidget.rs`
   declares the same name in `xwidget_decls` and its glob import is shadowed by this definition
   (P-17/P-21, the note at that mirror's C 184-230).  */
#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
unsafe fn find_suitable_keyboard (f: *mut frame) -> *mut GdkDevice       // C 911-921
{
  let seat: *mut GdkSeat = gdk_display_get_default_seat
    (gtk_widget_get_display (FRAME_GTK_WIDGET! (f)));

  if seat.is_null ()
    { return std::ptr::null_mut (); }

  return gdk_seat_get_keyboard (seat);
}

#[cfg (feature = "use_gtk")]
unsafe extern "C" fn find_widget_cb (widget: *mut GtkWidget, user: *mut c_void)  // C 923-927
{
  find_widget (widget, user as *mut widget_search_data);
}

#[cfg (feature = "use_gtk")]
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

#[cfg (feature = "use_gtk")]
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
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
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

#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
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

#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
unsafe extern "C" fn mouse_target_changed (webview: *mut WebKitWebView,
                                           hitresult: *mut WebKitHitTestResult,
                                           modifiers: guint, xw: gpointer)  // C 1113-1119
{
  define_cursors (xw as *mut xwidget, hitresult);
}
/* C 1120: `#endif` (the `HAVE_PGTK` arm).  */

#[cfg (feature = "use_gtk")]
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
