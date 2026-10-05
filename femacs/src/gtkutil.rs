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

/* C 29-32: the four Emacs headers this slice's records and calls come from.  Each `include!`
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

/* C 42-47: the rest of the Emacs headers.  `gtkutil.h`'s mirror is a placeholder (it carries the
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
   C 44/49's chain: `<gtk/gtk.h>` (through `"gtkutil.h"` 26) and `<gdk/gdkkeysyms.h>`, plus the
   GLib/GObject and cairo surfaces those pull in.  Third-party headers with no file in the
   reference tree, so the declarations this slice takes from them stand here (P-88 ①; the move into
   `femacs/src/include/gtk.rs` is P-90 ③'s owed item).  GLib, cairo and X11 are on this machine, so
   their lines are cited from the headers themselves and their records are measured (Evidence ②);
   **GTK and GDK are not**, so their citations name the header alone and no layout assertion is
   written for a GTK or GDK record (P-88 ②: UNVERIFIED).  */

/* GLib and GObject: `/opt/homebrew/include/glib-2.0` (GLib 2.90.0), `glibconfig.h` from
   `/opt/homebrew/Cellar/glib/2.90.0/lib/glib-2.0/include`.  */
/// C: gtypes.h 52/55/56/58/60/61: `typedef char gchar;` / `typedef int gint;` / `typedef gint
/// gboolean;` / `typedef unsigned char guchar;` / `typedef unsigned long gulong;` / `typedef
/// unsigned int guint;`
#[cfg (feature = "use_gtk")]
pub type gchar = c_char;
#[cfg (feature = "use_gtk")]
pub type gint = c_int;
#[cfg (feature = "use_gtk")]
pub type gboolean = c_int;
#[cfg (feature = "use_gtk")]
pub type guchar = c_uchar;
#[cfg (feature = "use_gtk")]
pub type gulong = c_ulong;
#[cfg (feature = "use_gtk")]
pub type guint = c_uint;
/// C: gtypes.h 109: `typedef void* gpointer;`
#[cfg (feature = "use_gtk")]
pub type gpointer = *mut c_void;
/// C: glibconfig.h 55/81 (the generated header): `typedef unsigned int guint32;` / `typedef
/// unsigned long gsize;`
#[cfg (feature = "use_gtk")]
pub type guint32 = c_uint;
#[cfg (feature = "use_gtk")]
pub type gsize = c_ulong;
/// C: gquark.h 38: `typedef guint32 GQuark;`
#[cfg (feature = "use_gtk")]
pub type GQuark = guint32;
/// C: gtype.h 427: `typedef gsize GType;` (the applicable arm of C 425-429's three-way choice on
/// this machine).  Measured `sizeof_GType=8` (Evidence ②).
#[cfg (feature = "use_gtk")]
pub type GType = gsize;
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<GType> () == 8);
/// C: gmacros.h 966-972: `#ifndef FALSE / #define FALSE (0) / #endif` and its `TRUE` pair.
#[cfg (feature = "use_gtk")]
pub const FALSE: gboolean = 0;
#[cfg (feature = "use_gtk")]
pub const TRUE: gboolean = 1;
/// C: glist.h 39-46: `typedef struct _GList GList; struct _GList { gpointer data; GList *next;
/// GList *prev; };`.  Measured on this machine: `sizeof_GList=24 align=8 data=0 next=8 prev=16`
/// (Evidence ②).  C 606/615/616/869/871 read `->next` and `->data`.
#[cfg (feature = "use_gtk")]
pub type GList = _GList;
#[cfg (feature = "use_gtk")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct _GList
{
  pub data: gpointer,        // C: `gpointer data;`
  pub next: *mut _GList,     // C: `GList *next;`
  pub prev: *mut _GList,     // C: `GList *prev;`
}
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<_GList> () == 24);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::align_of::<_GList> () == 8);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GList, data) == 0);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GList, next) == 8);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GList, prev) == 16);
/// C: gobject.h 192: `typedef struct _GObject GObject;` - the checked-cast chain's target type
/// (`G_OBJECT` below); the port never allocates or dereferences one (the `src/xfns.rs` 1027-1032
/// shape).  A `use_gtk` build reaches it through `gtkutil.h` 26's `<gtk/gtk.h>`.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GObject
{
  _private: [u8; 0],
}
/// C: gclosure.h 77/92: `typedef struct _GClosure GClosure;` / `typedef void (*GCallback)
/// (void);`.  C's function-pointer convention is `Option<unsafe extern "C" fn …>` (PORTING.md
/// P-27), which is what `G_CALLBACK` below produces.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GClosure
{
  _private: [u8; 0],
}
#[cfg (feature = "use_gtk")]
pub type GCallback = Option<unsafe extern "C" fn ()>;
/// C: gclosure.h: `typedef void (*GClosureNotify) (gpointer data, GClosure *closure);`
#[cfg (feature = "use_gtk")]
pub type GClosureNotify =
  Option<unsafe extern "C" fn (data: gpointer, closure: *mut GClosure)>;
/// C: gsignal.h 184-206: `typedef enum { G_CONNECT_DEFAULT = 0, G_CONNECT_AFTER = 1 << 0,
/// G_CONNECT_SWAPPED = 1 << 1 } GConnectFlags;`
#[cfg (feature = "use_gtk")]
pub type GConnectFlags = c_int;
/// C: gerror.h 43/47-52: `typedef struct _GError GError;` and its three members.  C 677/681/707
/// declare one and read `err->code`; measured `sizeof_GError=16 domain=0 code=4 message=8`
/// (Evidence ②).
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct _GError
{
  pub domain: GQuark,        // C: `GQuark domain;`
  pub code: gint,            // C: `gint code;`
  pub message: *mut gchar,   // C: `gchar *message;`
}
#[cfg (feature = "use_gtk")]
pub type GError = _GError;
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::size_of::<_GError> () == 16);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GError, code) == 4);
#[cfg (feature = "use_gtk")]
const _: () = core::assert! (core::mem::offset_of! (_GError, message) == 8);
/// C: gconvert.h 53-66: the `GConvertError` enumeration; C 681/707 compare `err->code` against
/// `G_CONVERT_ERROR_ILLEGAL_SEQUENCE` (member 59, value 1 by C's own text - measured 1).
/// C's enum ABI is `c_int` (PORTING.md 1.3), so no record measurement is owed.
#[cfg (feature = "use_gtk")]
pub type GConvertError = c_int;
#[cfg (feature = "use_gtk")]
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: GConvertError = 1;
/// C: gmessages.h 59-71: `G_LOG_FLAG_RECURSION = 1 << 0`, `G_LOG_FLAG_FATAL = 1 << 1`,
/// `G_LOG_LEVEL_WARNING = 1 << 4`, `} G_GNUC_FLAG_ENUM GLogLevelFlags;`.  C 1028/1128-1129 read
/// all three; measured `sizeof_GLogLevelFlags=4` with values 1/2/16 (Evidence ②).
#[cfg (feature = "use_gtk")]
pub type GLogLevelFlags = c_int;
#[cfg (feature = "use_gtk")]
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1 << 0;
#[cfg (feature = "use_gtk")]
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 1 << 1;
#[cfg (feature = "use_gtk")]
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 1 << 4;
/// C: gmessages.h 76: `typedef void (*GLogFunc) (const gchar *log_domain, GLogLevelFlags
/// log_level, const gchar *message, gpointer user_data);` - the type `my_log_handler`'s address is
/// handed to `g_log_set_handler` as.
#[cfg (feature = "use_gtk")]
pub type GLogFunc = Option<unsafe extern "C" fn (log_domain: *const gchar,
                                                log_level: GLogLevelFlags,
                                                message: *const gchar,
                                                user_data: gpointer)>;
/// C: gobject.h 54: `#define G_OBJECT(object) (G_TYPE_CHECK_INSTANCE_CAST ((object), G_TYPE_OBJECT,
/// GObject))`, whose own body is `((GObject *) g_type_check_instance_cast ((GTypeInstance *)
/// (object), (GType) (g_type)))` (gtype.h 528/2668-2672).  The port keeps the macro's name and its
/// result type; GLib's checked-cast chain is recorded as a deviation (the `src/pgtkterm.rs` 288-296
/// treatment of the same macro).
#[cfg (feature = "use_gtk")]
macro_rules! G_OBJECT {
  ($object:expr) => { ($object) as *mut GObject };
}
/// C: gtk/gtk.h: `#define G_CALLBACK(f) ((GCallback) (f))` - GTK's, not GLib's, so the citation is
/// the header and not a line.  C's cast reinterprets the function pointer; Rust refuses a
/// signature-changing `as` cast between fn pointers (`error: non-primitive cast`, measured in
/// `src/pgtkterm.rs`), so the port spells C's cast as the item address plus a pointer transmute,
/// and every callback C passes through it is declared `unsafe extern "C" fn` (PORTING.md 1.4).
#[cfg (feature = "use_gtk")]
macro_rules! G_CALLBACK {
  ($f:expr) => {
    Some (unsafe { core::mem::transmute::<*const (), unsafe extern "C" fn ()> ($f as *const ()) })
  };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 663/679/705: `if (!g_utf8_validate (str, -1, NULL))` / `while (! (cp = g_locale_to_utf8
  /// ((char *)p, -1, &bytes_read, &bytes_written, &err)) …`
  /// C: gunicode.h 1052-1054: `gboolean g_utf8_validate (const gchar *str, gssize max_len, const
  /// gchar **end);`
  pub fn g_utf8_validate (str: *const gchar, max_len: isize,
                          end: *mut *const gchar) -> gboolean;
  /// C: gconvert.h 129-133: `gchar* g_locale_to_utf8 (const gchar *opsysstring, gssize len,
  /// gsize *bytes_read, gsize *bytes_written, GError **error);`
  pub fn g_locale_to_utf8 (opsysstring: *const gchar, len: isize,
                           bytes_read: *mut gsize, bytes_written: *mut gsize,
                           error: *mut *mut GError) -> *mut gchar;
  /// C 666: `return g_strdup (str);`
  /// C: gstrfuncs.h 284: `gchar* (g_strdup) (const gchar *str) G_GNUC_MALLOC;` (the function the
  /// 324's `#define g_strdup(x) g_strdup_inline (x)` wrapper calls).
  pub fn g_strdup (str: *const gchar) -> *mut gchar;
  /// C 694/720: `if (cp) g_free (cp);` / `g_free (cp);`
  /// C: gmem.h 74: `void (g_free) (gpointer mem);`
  pub fn g_free (mem: gpointer);
  /// C 685/691/713/724: `g_error_free (err);`
  /// C: gerror.h 208: `void g_error_free (GError *error);`
  pub fn g_error_free (error: *mut GError);
  /// C 182/554/563/586/860: `g_object_new (emacs_menu_bar_get_type (), NULL)` /
  /// `g_object_unref (G_OBJECT (icon_buf))` / `g_object_ref (G_OBJECT (tooltip))`
  /// C: gobject.h 432-434: `gpointer g_object_new (GType object_type, const gchar
  /// *first_property_name, ...);`
  pub fn g_object_new (object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
  /// C 858/920/985: `g_object_set (G_OBJECT (widget), "has-tooltip", FALSE, NULL)` /
  /// `g_object_set (settings, "gtk-enable-tooltips", FALSE, NULL)`
  /// C: gobject.h 455-457: `void g_object_set (gpointer object, const gchar *first_property_name,
  /// ...);`
  pub fn g_object_set (object: gpointer, first_property_name: *const gchar, ...);
  /// C 917: `g_object_get (settings, "gtk-enable-tooltips", &tt_enabled, NULL);`
  /// C: gobject.h 459-461: `void g_object_get (gpointer object, const gchar
  /// *first_property_name, ...);`
  pub fn g_object_get (object: gpointer, first_property_name: *const gchar, ...);
  /// C 860/862: `g_object_ref (G_OBJECT (tooltip));` / `g_object_ref (G_OBJECT (x->ttip_lbl));`
  /// C: gobject.h 513: `gpointer g_object_ref (gpointer object);`
  pub fn g_object_ref (object: gpointer) -> gpointer;
  /// C: gobject.h 515: `void g_object_unref (gpointer object);`
  pub fn g_object_unref (object: gpointer);
  /// C 922: `g_object_set_data (G_OBJECT (x->ttip_window), "restore-tt", (gpointer)f);`
  /// C: gobject.h 597-599: `void g_object_set_data (GObject *object, const gchar *key, gpointer
  /// data);`
  pub fn g_object_set_data (object: *mut GObject, key: *const gchar, data: gpointer);
  /// C 980: `if (g_object_get_data (G_OBJECT (win), "restore-tt"))`
  /// C: gobject.h 594-595: `gpointer g_object_get_data (GObject *object, const gchar *key);`
  pub fn g_object_get_data (object: *mut GObject, key: *const gchar) -> gpointer;
  /// C 875: `g_list_free (list);`
  /// C: glist.h 53: `void g_list_free (GList *list);`
  pub fn g_list_free (list: *mut GList);
  /// C: glist.h 175: `#define g_list_next(list) ((list) ? (((GList *)(list))->next) : NULL)` -
  /// GLib's own body, which C 615/869 use as the update step of a `for` loop.
  /// C: gsignal.h 554-556: `#define g_signal_connect(instance, detailed_signal, c_handler, data)
  /// g_signal_connect_data ((instance), (detailed_signal), (c_handler), (data), NULL,
  /// (GConnectFlags) 0)` - GLib's own body; C 883-884 is the one call site.
  /// C: gsignal.h 440-446: `gulong g_signal_connect_data (gpointer instance, const gchar
  /// *detailed_signal, GCallback c_handler, gpointer data, GClosureNotify destroy_data,
  /// GConnectFlags connect_flags);`
  pub fn g_signal_connect_data (instance: gpointer, detailed_signal: *const gchar,
                                c_handler: GCallback, data: gpointer,
                                destroy_data: GClosureNotify,
                                connect_flags: GConnectFlags) -> gulong;
  /// C 1128: `id = g_log_set_handler ("Gtk", G_LOG_LEVEL_WARNING | G_LOG_FLAG_FATAL |
  /// G_LOG_FLAG_RECURSION, my_log_handler, NULL);`
  /// C: gmessages.h 84-87: `guint g_log_set_handler (const gchar *log_domain, GLogLevelFlags
  /// log_levels, GLogFunc log_func, gpointer user_data);`
  pub fn g_log_set_handler (log_domain: *const gchar, log_levels: GLogLevelFlags,
                            log_func: GLogFunc, user_data: gpointer) -> guint;
  /// C 1135: `g_log_remove_handler ("Gtk", id);`
  /// C: gmessages.h 95: `void g_log_remove_handler (const gchar *log_domain, guint handler_id);`
  pub fn g_log_remove_handler (log_domain: *const gchar, handler_id: guint);
}
#[cfg (feature = "use_gtk")]
macro_rules! g_list_next {
  ($list:expr) => {
    { if !($list).is_null () { (*($list as *mut GList)).next } else { std::ptr::null_mut () } }
  };
}
#[cfg (feature = "use_gtk")]
macro_rules! g_signal_connect {
  ($instance:expr, $signal:expr, $handler:expr, $data:expr) => {
    g_signal_connect_data (($instance) as gpointer, ($signal), G_CALLBACK! ($handler), ($data),
                           None, 0 as GConnectFlags)
  };
}

/* cairo and cairo-gobject: `/opt/homebrew/include/cairo` (cairo 1.18.4).  C 428-476's
   `xg_get_pixbuf_from_surface` and C 512-570's two cairo arms are the users; `img->cr_data` is
   C's `void *` in the mirror (`include/dispextern.rs` 2314), so the two calls that need the real
   type take C's pointer cast at the use site (P-72 rule 3).  */
/// C: cairo.h 135/164/229: `typedef struct _cairo cairo_t;` / `typedef struct _cairo_surface
/// cairo_surface_t;` / `typedef struct _cairo_pattern cairo_pattern_t;`
#[cfg (feature = "use_gtk")]
pub enum cairo_t {}
#[cfg (feature = "use_gtk")]
pub enum cairo_surface_t {}
#[cfg (feature = "use_gtk")]
pub enum cairo_pattern_t {}
/// C: cairo.h 431-441: `typedef enum _cairo_format { CAIRO_FORMAT_INVALID = -1, CAIRO_FORMAT_ARGB32
/// = 0, … } cairo_format_t;` - measured `CAIRO_FORMAT_ARGB32=0`,
/// `sizeof_cairo_format_t=4` (Evidence ②).  C's enum ABI is `c_int` (PORTING.md 1.3).
#[cfg (feature = "use_gtk")]
pub type cairo_format_t = c_int;
#[cfg (feature = "use_gtk")]
pub const CAIRO_FORMAT_ARGB32: cairo_format_t = 0;
/// C: cairo.h 662-696: `typedef enum _cairo_operator { CAIRO_OPERATOR_CLEAR,
/// CAIRO_OPERATOR_SOURCE, … } cairo_operator_t;` - measured `CAIRO_OPERATOR_SOURCE=1`,
/// `sizeof_cairo_operator_t=4` (Evidence ②).
#[cfg (feature = "use_gtk")]
pub type cairo_operator_t = c_int;
#[cfg (feature = "use_gtk")]
pub const CAIRO_OPERATOR_SOURCE: cairo_operator_t = 1;
/// C: cairo.h 2990-2997: `typedef enum _cairo_pattern_type { CAIRO_PATTERN_TYPE_SOLID,
/// CAIRO_PATTERN_TYPE_SURFACE, … } cairo_pattern_type_t;` - measured
/// `CAIRO_PATTERN_TYPE_SURFACE=1`, `sizeof_cairo_pattern_type_t=4` (Evidence ②).
#[cfg (feature = "use_gtk")]
pub type cairo_pattern_type_t = c_int;
#[cfg (feature = "use_gtk")]
pub const CAIRO_PATTERN_TYPE_SURFACE: cairo_pattern_type_t = 1;
/// C: cairo.h 325-374: `typedef enum _cairo_status { … } cairo_status_t;` - the return type of
/// `cairo_pattern_get_surface` below, which C 546 ignores (a statement call).
#[cfg (feature = "use_gtk")]
pub type cairo_status_t = c_int;
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 441-442: `cairo_image_surface_create_for_data (pixels, CAIRO_FORMAT_ARGB32, width, height,
  /// rowstride);`
  /// C: cairo.h 2718-2724: `cairo_surface_t *cairo_image_surface_create_for_data (unsigned char
  /// *data, cairo_format_t format, int width, int height, int stride);`
  pub fn cairo_image_surface_create_for_data (data: *mut c_uchar, format: cairo_format_t,
                                              width: c_int, height: c_int,
                                              stride: c_int) -> *mut cairo_surface_t;
  /// C 432-433: `cairo_image_surface_get_width (surface)` / `…_get_height (surface)`
  /// C: cairo.h 2731-2735: `int cairo_image_surface_get_width (cairo_surface_t *surface);` /
  /// `int cairo_image_surface_get_height (cairo_surface_t *surface);`
  pub fn cairo_image_surface_get_width (surface: *mut cairo_surface_t) -> c_int;
  pub fn cairo_image_surface_get_height (surface: *mut cairo_surface_t) -> c_int;
  /// C 443: `cairo_t *cr = cairo_create (icon_surface);`
  /// C: cairo.h 539-540: `cairo_t *cairo_create (cairo_surface_t *target);`
  pub fn cairo_create (target: *mut cairo_surface_t) -> *mut cairo_t;
  /// C 444: `cairo_surface_destroy (icon_surface);`
  /// C: cairo.h 2489: `void cairo_surface_destroy (cairo_surface_t *surface);`
  pub fn cairo_surface_destroy (surface: *mut cairo_surface_t);
  /// C 445: `cairo_set_source_surface (cr, surface, 0, 0);`
  /// C: cairo.h 712-716: `void cairo_set_source_surface (cairo_t *cr, cairo_surface_t *surface,
  /// double x, double y);`
  pub fn cairo_set_source_surface (cr: *mut cairo_t, surface: *mut cairo_surface_t,
                                   x: c_double, y: c_double);
  /// C 446: `cairo_set_operator (cr, CAIRO_OPERATOR_SOURCE);`
  /// C: cairo.h 699: `void cairo_set_operator (cairo_t *cr, cairo_operator_t op);`
  pub fn cairo_set_operator (cr: *mut cairo_t, op: cairo_operator_t);
  /// C 447: `cairo_paint (cr);`
  /// C: cairo.h 973: `void cairo_paint (cairo_t *cr);`
  pub fn cairo_paint (cr: *mut cairo_t);
  /// C 448: `cairo_destroy (cr);`
  /// C: cairo.h 546: `void cairo_destroy (cairo_t *cr);`
  pub fn cairo_destroy (cr: *mut cairo_t);
  /// C 545: `if (cairo_pattern_get_type (img->cr_data) == CAIRO_PATTERN_TYPE_SURFACE)`
  /// C: cairo.h 2999-3000: `cairo_pattern_type_t cairo_pattern_get_type (cairo_pattern_t
  /// *pattern);`
  pub fn cairo_pattern_get_type (pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
  /// C 546: `cairo_pattern_get_surface (img->cr_data, &surface);`
  /// C: cairo.h 3135-3137: `cairo_status_t cairo_pattern_get_surface (cairo_pattern_t *pattern,
  /// cairo_surface_t **surface);`
  pub fn cairo_pattern_get_surface (pattern: *mut cairo_pattern_t,
                                    surface: *mut *mut cairo_surface_t) -> cairo_status_t;
}

/* ----------------------------------------------------------------------------------------------
   GDK.  No GDK tree on this machine, so each citation names the header alone and every one of the
   records below is the never-dereferenced `[u8; 0]` shape (`src/xfns.rs` 1004-1009's `GtkWidget`):
   the port only ever passes their pointers to GDK's and GTK's own entry points.  The three records
   C reads *members* of (`GdkRGBA` at C 787-789 and `GdkColor` at C 802-805, both GTK-version arms
   of `xg_check_special_colors`) carry exactly those members, and their layout is **UNVERIFIED**
   (P-88 ②: no header to measure; owed in the ledger).
   ---------------------------------------------------------------------------------------------- */
/// C: gdk/gdkdisplay.h: `typedef struct _GdkDisplay GdkDisplay;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkDisplay
{
  _private: [u8; 0],
}
/// C: gdk/gdkscreen.h: `typedef struct _GdkScreen GdkScreen;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkScreen
{
  _private: [u8; 0],
}
/// C: gdk/gdkcursor.h: `typedef struct _GdkCursor GdkCursor;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkCursor
{
  _private: [u8; 0],
}
/// C: gdk/gdkwindow.h: `typedef struct _GdkWindow GdkWindow;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkWindow
{
  _private: [u8; 0],
}
/// C: gdk-pixbuf/gdk-pixbuf.h: `typedef struct _GdkPixbuf GdkPixbuf;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkPixbuf
{
  _private: [u8; 0],
}
/// C: gdk/gdkrgba.h: `typedef struct { double red; double green; double blue; double alpha; }
/// GdkRGBA;`.  C 745 declares one, C 783 assigns `col = *c;` and C 787-789 reads `col.red`,
/// `col.green`, `col.blue`.  **UNVERIFIED**: no GDK header on this machine.
#[cfg (feature = "use_gtk")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct GdkRGBA
{
  pub red: c_double,        // C: gdk/gdkrgba.h: `double red;`
  pub green: c_double,      // C: gdk/gdkrgba.h: `double green;`
  pub blue: c_double,       // C: gdk/gdkrgba.h: `double blue;`
  pub alpha: c_double,      // C: gdk/gdkrgba.h: `double alpha;`
}
/// C: gdk/gdkcolor.h (GTK2's GDK): `typedef struct { guint32 pixel; guint16 red; guint16 green;
/// guint16 blue; } GdkColor;`.  C 752 declares `GdkColor *grgb;` and C 802-805 reads four members.
/// **UNVERIFIED**: no GDK header on this machine (the record's width is unmeasured too).
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct GdkColor
{
  pub pixel: guint32,        // C: gdk/gdkcolor.h: `guint32 pixel;`
  pub red: c_ushort,         // C: gdk/gdkcolor.h: `guint16 red;`
  pub green: c_ushort,       // C: gdk/gdkcolor.h: `guint16 green;`
  pub blue: c_ushort,        // C: gdk/gdkcolor.h: `guint16 blue;`
}
/// C: gdk/gdktypes.h: `typedef enum { GDK_COLORSPACE_RGB } GdkColorspace;` (C 404/434 pass it to
/// `gdk_pixbuf_new`).  C's enum ABI is `c_int` (PORTING.md 1.3); **UNVERIFIED** for the same
/// reason as the records above.
#[cfg (feature = "use_gtk")]
pub type GdkColorspace = c_int;
#[cfg (feature = "use_gtk")]
pub const GDK_COLORSPACE_RGB: GdkColorspace = 0;
/// C: gdk/gdkcursor.h: `typedef enum { … GDK_LEFT_PTR, … } GdkCursorType;` (C 375 passes it to
/// `gdk_cursor_new_for_display`).  Its value is GTK's, **UNVERIFIED** here.
#[cfg (feature = "use_gtk")]
pub type GdkCursorType = c_int;
#[cfg (feature = "use_gtk")]
pub const GDK_LEFT_PTR: GdkCursorType = 0;
/// C: gtk/gtkwidget.h: `#define GDK_WINDOW(window) ((GdkWindow *) (window))`-family checked cast
/// (C 616 converts `children->data`), and gdk/gdkx.h's `#define GDK_DISPLAY_XDISPLAY(display)` (C
/// 260).  Both keep their names and their result types; the cast chains are GLib's and owed (the
/// `src/xfns.rs` 1019-1025 treatment of `GTK_WIDGET_CLASS`).
#[cfg (feature = "use_gtk")]
macro_rules! GDK_WINDOW {
  ($o:expr) => { ($o) as *mut GdkWindow };
}
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
macro_rules! GDK_DISPLAY_XDISPLAY {
  ($d:expr) => { gdk_x11_display_get_xdisplay ($d) };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 247/249: `gdpy = gdk_display_open (display_name);` and the `HAVE_PGTK` arm's
  /// `gdk_display_open (strlen (display_name) == 0 ? NULL : display_name)`
  /// C: gdk/gdkdisplay.h: `GdkDisplay *gdk_display_open (const gchar *display_name);`
  pub fn gdk_display_open (display_name: *const gchar) -> *mut GdkDisplay;
  /// C 255/322/346: `gdk_display_manager_set_default_display (gdk_display_manager_get (), gdpy);`
  /// C: gdk/gdkdisplaymanager.h: `GdkDisplayManager *gdk_display_manager_get (void);` and
  /// `void gdk_display_manager_set_default_display (GdkDisplayManager *manager, GdkDisplay
  /// *display);`
  pub fn gdk_display_manager_get () -> *mut GdkDisplayManager;
  pub fn gdk_display_manager_set_default_display (manager: *mut GdkDisplayManager,
                                                  display: *mut GdkDisplay);
  /// C 208/218: `gdk_display_get_default_screen (gdpy)` / `… (FRAME_X_DISPLAY (f))`
  /// C: gdk/gdkdisplay.h: `GdkScreen *gdk_display_get_default_screen (GdkDisplay *display);`
  pub fn gdk_display_get_default_screen (display: *mut GdkDisplay) -> *mut GdkScreen;
  /// C 312/336: `if (gdk_display_get_default () == gdpy)`
  /// C: gdk/gdkdisplay.h: `GdkDisplay *gdk_display_get_default (void);`
  pub fn gdk_display_get_default () -> *mut GdkDisplay;
  /// C 329/353: `gdk_display_close (gdpy);`
  /// C: gdk/gdkdisplay.h: `void gdk_display_close (GdkDisplay *display);`
  pub fn gdk_display_close (display: *mut GdkDisplay);
  /// C 375: `return gdk_cursor_new_for_display (gdpy, GDK_LEFT_PTR);`
  /// C: gdk/gdkcursor.h: `GdkCursor *gdk_cursor_new_for_display (GdkDisplay *display,
  /// GdkCursorType cursor_type);`
  pub fn gdk_cursor_new_for_display (display: *mut GdkDisplay,
                                     cursor_type: GdkCursorType) -> *mut GdkCursor;
  /// C 404/434: `icon_buf = gdk_pixbuf_new (GDK_COLORSPACE_RGB, TRUE, 8, width, height);`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `GdkPixbuf *gdk_pixbuf_new (GdkColorspace colorspace, gboolean
  /// has_alpha, int bits_per_sample, int width, int height);`
  pub fn gdk_pixbuf_new (colorspace: GdkColorspace, has_alpha: gboolean,
                         bits_per_sample: c_int, width: c_int, height: c_int) -> *mut GdkPixbuf;
  /// C 407/438: `guchar *pixels = gdk_pixbuf_get_pixels (icon_buf);`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `guchar *gdk_pixbuf_get_pixels (const GdkPixbuf *pixbuf);`
  pub fn gdk_pixbuf_get_pixels (pixbuf: *const GdkPixbuf) -> *mut guchar;
  /// C 408/439: `int rowjunkwidth = gdk_pixbuf_get_rowstride (icon_buf) - width * 4;`
  /// C: gdk-pixbuf/gdk-pixbuf.h: `int gdk_pixbuf_get_rowstride (const GdkPixbuf *pixbuf);`
  pub fn gdk_pixbuf_get_rowstride (pixbuf: *const GdkPixbuf) -> c_int;
  /// C 606: `GList *children = gdk_window_peek_children (window);`
  /// C: gdk/gdkwindow.h: `GList *gdk_window_peek_children (GdkWindow *window);`
  pub fn gdk_window_peek_children (window: *mut GdkWindow) -> *mut GList;
  /// C 608/616: `gdk_window_set_cursor (window, cursor);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_cursor (GdkWindow *window, GdkCursor *cursor);`
  pub fn gdk_window_set_cursor (window: *mut GdkWindow, cursor: *mut GdkCursor);
  /// C 915/983: `screen = gdk_window_get_screen (gwin);`
  /// C: gdk/gdkwindow.h: `GdkScreen *gdk_window_get_screen (GdkWindow *window);`
  pub fn gdk_window_get_screen (window: *mut GdkWindow) -> *mut GdkScreen;
  /// C 784: `gdk_rgba_free (c);`
  /// C: gdk/gdkrgba.h: `void gdk_rgba_free (GdkRGBA *rgba);`
  pub fn gdk_rgba_free (rgba: *mut GdkRGBA);
  /// C 1272: `gdk_flush ();`
  /// C: gdk/gdkwindow.h: `void gdk_flush (void);`
  pub fn gdk_flush ();
  /// C 1231: `gdk_window_resize (gtk_widget_get_window (FRAME_GTK_OUTER_WIDGET (f)), outer_width,
  /// outer_height);`
  /// C: gdk/gdkwindow.h: `void gdk_window_resize (GdkWindow *window, gint width, gint height);`
  pub fn gdk_window_resize (window: *mut GdkWindow, width: gint, height: gint);
}
/// C: gdk/gdkdisplaymanager.h: `typedef struct _GdkDisplayManager GdkDisplayManager;` - the
/// opaque the two `*_default_display` entry points hand back.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GdkDisplayManager
{
  _private: [u8; 0],
}

/* ----------------------------------------------------------------------------------------------
   GTK.  Same treatment as GDK above: no GTK tree on this machine, so each citation names the
   header alone, the `GtkWidget`-family pointees are the `[u8; 0]` shape, and the records C reads
   members of carry exactly those members with their layout **UNVERIFIED** (P-88 ②; owed).
   `GtkWidgetClass` is the `src/xfns.rs` 1034-1045 precedent: C reads one member of it, the port
   carries that member and owes the record's other members and the offset.
   ---------------------------------------------------------------------------------------------- */
/// C: gtk/gtkwidget.h: `typedef struct _GtkWidget GtkWidget;` (`G_DECLARE_DERIVABLE_TYPE` in GTK3).
/// Also the type `include/xterm.rs`'s `x_output.widget`/`menubar_widget`/`ttip_lbl` members name
/// bare (xterm.rs 378 lists it among that mirror's un-mirrored names), which is why it is declared
/// here rather than only where this file uses it.
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkWidget
{
  _private: [u8; 0],
}
/// C: gtk/gtkwindow.h: `typedef struct _GtkWindow GtkWindow;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkWindow
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenu.h: `typedef struct _GtkMenu GtkMenu;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkMenu
{
  _private: [u8; 0],
}
/// C: gtk/gtkimage.h: `typedef struct _GtkImage GtkImage;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkImage
{
  _private: [u8; 0],
}
/// C: gtk/gtkbin.h: `typedef struct _GtkBin GtkBin;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkBin
{
  _private: [u8; 0],
}
/// C: gtk/gtkcontainer.h: `typedef struct _GtkContainer GtkContainer;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkContainer
{
  _private: [u8; 0],
}
/// C: gtk/gtklabel.h: `typedef struct _GtkLabel GtkLabel;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkLabel
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenushell.h: `typedef struct _GtkMenuShell GtkMenuShell;`
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkMenuShell
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenubar.h: `typedef struct _GtkMenuBar GtkMenuBar;` (`G_DECLARE_DERIVABLE_TYPE`, so
/// `struct _GtkMenuBar` is GTK's own and its size is GTK's).  C 86 embeds one **by value** in this
/// file's `EmacsMenuBar`; the port's `[u8; 0]` is the placeholder for a record it cannot lay out
/// (SIZE **UNVERIFIED**, owed - `## Deviations`).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkMenuBar
{
  _private: [u8; 0],
}
/// C: gtk/gtkmenubar.h: `typedef struct _GtkMenuBarClass GtkMenuBarClass;` - C 91 embeds one by
/// value in `EmacsMenuBarClass` (same UNVERIFIED placeholder as above).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkMenuBarClass
{
  _private: [u8; 0],
}
/// C: gtk/gtkwidget.h: `GtkWidgetClass`.  C 163 writes and C 173 calls exactly one member of it;
/// the port carries that member and owes the record's other members and this one's offset to a GTK
/// toolchain measurement (P-88 ②; the `src/xfns.rs` 1034-1045 precedent).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkWidgetClass
{
  /// C: gtk/gtkwidget.h: `void (*get_preferred_width) (GtkWidget *widget, gint *minimum, gint
  /// *natural);`
  pub get_preferred_width:
    Option<unsafe extern "C" fn (widget: *mut GtkWidget, minimum: *mut gint, natural: *mut gint)>,
}
/// C: gtk/gtktooltip.h: `typedef struct _GtkTooltip GtkTooltip;`
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[repr(C)]
pub struct GtkTooltip
{
  _private: [u8; 0],
}
/// C: gtk/gtksettings.h: `typedef struct _GtkSettings GtkSettings;`
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[repr(C)]
pub struct GtkSettings
{
  _private: [u8; 0],
}
/// C: gtk/gtkstylecontext.h: `typedef struct _GtkStyleContext GtkStyleContext;` (GTK3's).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GtkStyleContext
{
  _private: [u8; 0],
}
/// C: gtk/gtkstyle.h (GTK2's): `struct _GtkStyle { … GdkColor fg[…]; GdkColor bg[…]; … }`.  C 751
/// declares `GtkStyle *gsty;` and C 798-800 reads `gsty->bg[GTK_STATE_SELECTED]` and
/// `gsty->fg[GTK_STATE_SELECTED]` - **the two members and the state enumerator are GTK2's and
/// there is no GTK2 tree on this machine**, so the port carries the record's opaque head and
/// records those two member reads as owed (SIZE and BOUNDS UNVERIFIED, P-88 ②).
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
#[repr(C)]
pub struct GtkStyle
{
  _private: [u8; 0],
}
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
unsafe extern "C" {
  /// C 798: `gsty = gtk_widget_get_style (FRAME_GTK_WIDGET (f));`
  /// C: gtk/gtkwidget.h (GTK2): `GtkStyle *gtk_widget_get_style (GtkWidget *widget);`
  pub fn gtk_widget_get_style (widget: *mut GtkWidget) -> *mut GtkStyle;
}
/// C: gtk/gtktypes.h: `typedef struct { gint x, y; gint width, height; } GtkAllocation;`.  C 1197
/// declares one, C 1199-1200 reads `alloc.width`/`alloc.height` and C 1235 initializes one with
/// four values; **UNVERIFIED** (the `src/pgtkterm.rs` 431-445 declaration of the same record, whose
/// size/offsets that ledger owes too).
#[cfg (feature = "use_gtk")]
#[repr(C)]
pub struct GtkAllocation
{
  pub x: gint,          // C: gtk/gtktypes.h: `gint x;`
  pub y: gint,          // C: gtk/gtktypes.h: `gint y;`
  pub width: gint,      // C: gtk/gtktypes.h: `gint width;`
  pub height: gint,     // C: gtk/gtktypes.h: `gint height;`
}
/// C: gtk/gtkrequisition.h: `typedef struct { gint width; gint height; } GtkRequisition;`.  C 905
/// declares one and C 935-937 reads both members with `gtk_widget_get_preferred_size`;
/// **UNVERIFIED** for the same reason.
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
#[repr(C)]
pub struct GtkRequisition
{
  pub width: gint,      // C: gtk/gtkrequisition.h: `gint width;`
  pub height: gint,     // C: gtk/gtkrequisition.h: `gint height;`
}
/// C: gtk/gtktypes.h: `typedef enum { GTK_TEXT_DIR_NONE, GTK_TEXT_DIR_LTR, GTK_TEXT_DIR_RTL }
/// GtkTextDirection;` - the type of C 139's `struct xg_frame_tb_info.dir`.  C's enum ABI is
/// `c_int` (PORTING.md 1.3); the width is **UNVERIFIED** (no GTK header here).
#[cfg (feature = "use_gtk")]
pub type GtkTextDirection = c_int;
/// C: gtk/gtktypes.h / gtk/gtkstylecontext.h: `GtkStateFlags` (GTK3's) and `GtkStateType` (GTK2's,
/// used with `gtk_widget_get_style`/`GtkStyle`), plus the two GTK3 flag values C 770 ORs (the
/// `GtkStyleContext` arm) and the property name C 780 asks for.  All GTK's; **UNVERIFIED**.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub type GtkStateFlags = c_int;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_STATE_FLAG_SELECTED: GtkStateFlags = 1 << 4;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_STATE_FLAG_FOCUSED: GtkStateFlags = 1 << 3;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
pub const GTK_STYLE_PROPERTY_BACKGROUND_COLOR: *const c_char = c"background-color".as_ptr ();
/// C: gtk/gtkenums.h (GTK2's): `GTK_ORIENTATION_HORIZONTAL` (C 120's `gtk_box_new` body),
/// `GTK_STATE_SELECTED` (C 799-800) and gtk/gtkstock.h's three stock-id macros C 75-77 aliases.
/// **None of the six can be transcribed**: no GTK2 tree exists here (`## Owed`).  The GTK3-version
/// `GtkOrientation`/`GtkStateType` are the same names in GTK3, and this file only reaches them
/// through the GTK2 arm.
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
pub type GtkOrientation = c_int;
#[cfg (all (feature = "use_gtk", not (feature = "have_gtk3")))]
pub type GtkStateType = c_int;
/// C: gtk/gtk.h: the eight checked-cast wrappers C 211-223/533-594/866/871/873/914/928/978/1041
/// uses, each keeping the macro's name and its result type (the GLib cast chains are owed, as
/// above).
#[cfg (feature = "use_gtk")]
macro_rules! GTK_WIDGET {
  ($o:expr) => { ($o) as *mut GtkWidget };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_WINDOW {
  ($o:expr) => { ($o) as *mut GtkWindow };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_MENU {
  ($o:expr) => { ($o) as *mut GtkMenu };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IMAGE {
  ($o:expr) => { ($o) as *mut GtkImage };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_BIN {
  ($o:expr) => { ($o) as *mut GtkBin };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_CONTAINER {
  ($o:expr) => { ($o) as *mut GtkContainer };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_LABEL {
  ($o:expr) => { ($o) as *mut GtkLabel };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_MENU_SHELL {
  ($o:expr) => { ($o) as *mut GtkMenuShell };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! GTK_WIDGET_CLASS {
  ($o:expr) => { ($o) as *mut GtkWidgetClass };
}
/// C: gtk/gtk.h: the four type-check macros C 210/220/833/1039-1046 asks, each keeping the macro's
/// name and its result type (`gboolean`).
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IS_MENU {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (feature = "use_gtk")]
macro_rules! GTK_IS_WINDOW {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
macro_rules! GTK_IS_LABEL {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_xinput2"))]
macro_rules! GTK_IS_MENU_BAR {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (feature = "use_gtk")]
unsafe extern "C" {
  /// C 211/221: `gtk_menu_set_screen (GTK_MENU (w), gscreen);`
  /// C: gtk/gtkmenu.h: `void gtk_menu_set_screen (GtkMenu *menu, GdkScreen *screen);`
  pub fn gtk_menu_set_screen (menu: *mut GtkMenu, screen: *mut GdkScreen);
  /// C 213/223: `gtk_window_set_screen (GTK_WINDOW (w), gscreen);`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_screen (GtkWindow *window, GdkScreen *screen);`
  pub fn gtk_window_set_screen (window: *mut GtkWindow, screen: *mut GdkScreen);
  /// C 292: `return gtk_widget_get_scale_factor (FRAME_GTK_WIDGET (f));`
  /// C: gtk/gtkwidget.h: `gint gtk_widget_get_scale_factor (GtkWidget *widget);`
  pub fn gtk_widget_get_scale_factor (widget: *mut GtkWidget) -> gint;
  /// C 533/535: `old_widget = GTK_IMAGE (gtk_image_new_from_file (encoded_file));` /
  /// `gtk_image_set_from_file (old_widget, encoded_file);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_file (const gchar *filename);` / `void
  /// gtk_image_set_from_file (GtkImage *image, const gchar *filename);`
  pub fn gtk_image_new_from_file (filename: *const gchar) -> *mut GtkWidget;
  pub fn gtk_image_set_from_file (image: *mut GtkImage, filename: *const gchar);
  /// C 554/556: `gtk_image_new_from_surface (surface)` / `gtk_image_set_from_surface (old_widget,
  /// surface);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_surface (cairo_surface_t *surface);` / `void
  /// gtk_image_set_from_surface (GtkImage *image, cairo_surface_t *surface);`
  pub fn gtk_image_new_from_surface (surface: *mut cairo_surface_t) -> *mut GtkWidget;
  pub fn gtk_image_set_from_surface (image: *mut GtkImage, surface: *mut cairo_surface_t);
  /// C 563/565/586/588: `gtk_image_new_from_pixbuf (icon_buf)` /
  /// `gtk_image_set_from_pixbuf (old_widget, icon_buf);`
  /// C: gtk/gtkimage.h: `GtkWidget *gtk_image_new_from_pixbuf (GdkPixbuf *pixbuf);` / `void
  /// gtk_image_set_from_pixbuf (GtkImage *image, GdkPixbuf *pixbuf);`
  pub fn gtk_image_new_from_pixbuf (pixbuf: *mut GdkPixbuf) -> *mut GtkWidget;
  pub fn gtk_image_set_from_pixbuf (image: *mut GtkImage, pixbuf: *mut GdkPixbuf);
  /// C 866/1041: `gtk_bin_get_child (GTK_BIN (gwdesc))` (the `XG_BIN_CHILD` macro below is its
  /// other spelling)
  /// C: gtk/gtkbin.h: `GtkWidget *gtk_bin_get_child (GtkBin *bin);`
  pub fn gtk_bin_get_child (bin: *mut GtkBin) -> *mut GtkWidget;
  /// C 878: `gtk_window_set_title (x->ttip_window, "");`
  /// C: gtk/gtkwindow.h: `void gtk_window_set_title (GtkWindow *window, const gchar *title);`
  pub fn gtk_window_set_title (window: *mut GtkWindow, title: *const gchar);
  /// C 880-881: `gtk_widget_realize (GTK_WIDGET (x->ttip_window));` / `gtk_widget_realize
  /// (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_realize (GtkWidget *widget);`
  pub fn gtk_widget_realize (widget: *mut GtkWidget);
  /// C 831/864: `GtkWidget *top = gtk_widget_get_toplevel (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `GtkWidget *gtk_widget_get_toplevel (GtkWidget *widget);`
  pub fn gtk_widget_get_toplevel (widget: *mut GtkWidget) -> *mut GtkWidget;
  /// C 605/914/982/1231: `GdkWindow *window = gtk_widget_get_window (w);` /
  /// `gwin = gtk_widget_get_window (GTK_WIDGET (x->ttip_window));`
  /// C: gtk/gtkwidget.h: `GdkWindow *gtk_widget_get_window (GtkWidget *widget);`
  pub fn gtk_widget_get_window (widget: *mut GtkWidget) -> *mut GdkWindow;
  /// C 957/959: `gtk_widget_show (GTK_WIDGET (x->ttip_window));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_show (GtkWidget *widget);`
  pub fn gtk_widget_show (widget: *mut GtkWidget);
  /// C 834/978: `gtk_widget_hide (previous_toplevel);` / `gtk_widget_hide (GTK_WIDGET (win));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_hide (GtkWidget *widget);`
  pub fn gtk_widget_hide (widget: *mut GtkWidget);
  /// C 955/960/1099: `gtk_window_move (x->ttip_window, root_x / xg_get_scale (f), …)` /
  /// `gtk_window_move (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), f->left_pos / scale, …);`
  /// C: gtk/gtkwindow.h: `void gtk_window_move (GtkWindow *window, gint x, gint y);`
  pub fn gtk_window_move (window: *mut GtkWindow, x: gint, y: gint);
  /// C 1187/1192: `gtk_window_get_size (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), &gwidth,
  /// &gheight);`
  /// C: gtk/gtkwindow.h: `void gtk_window_get_size (GtkWindow *window, gint *width, gint
  /// *height);`
  pub fn gtk_window_get_size (window: *mut GtkWindow, width: *mut gint, height: *mut gint);
  /// C 1241/1245: `gtk_window_resize (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), outer_width,
  /// outer_height);`
  /// C: gtk/gtkwindow.h: `void gtk_window_resize (GtkWindow *window, gint width, gint height);`
  pub fn gtk_window_resize (window: *mut GtkWindow, width: gint, height: gint);
  /// C 1248: `gtk_widget_set_size_request (FRAME_GTK_WIDGET (f), outer_width, outer_height);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_size_request (GtkWidget *widget, gint width, gint
  /// height);`
  pub fn gtk_widget_set_size_request (widget: *mut GtkWidget, width: gint, height: gint);
  /// C 1198: `gtk_widget_get_allocation (FRAME_GTK_WIDGET (f), &alloc);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_get_allocation (GtkWidget *widget, GtkAllocation
  /// *allocation);`
  pub fn gtk_widget_get_allocation (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
  /// C 1236: `gtk_widget_size_allocate (FRAME_GTK_OUTER_WIDGET (f), &alloc);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_size_allocate (GtkWidget *widget, GtkAllocation
  /// *allocation);`
  pub fn gtk_widget_size_allocate (widget: *mut GtkWidget, allocation: *mut GtkAllocation);
  /// C 1001-1005/1009-1010: `gtk_widget_set_tooltip_text (FRAME_GTK_OUTER_WIDGET (f) ?
  /// FRAME_GTK_OUTER_WIDGET (f) : FRAME_GTK_WIDGET (f), SSDATA (encoded_string));`
  /// C: gtk/gtkwidget.h: `void gtk_widget_set_tooltip_text (GtkWidget *widget, const gchar
  /// *text);`
  pub fn gtk_widget_set_tooltip_text (widget: *mut GtkWidget, text: *const gchar);
  /// C 1131: `if (!gtk_window_parse_geometry (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)), geom_str))`
  /// C: gtk/gtkwindow.h: `gboolean gtk_window_parse_geometry (GtkWindow *window, const gchar
  /// *geometry);`
  pub fn gtk_window_parse_geometry (window: *mut GtkWindow,
                                    geometry: *const gchar) -> gboolean;
  /// C 1045: `gtk_menu_shell_get_parent_shell (GTK_MENU_SHELL (fw))`
  /// C: gtk/gtkmenushell.h: `GtkWidget *gtk_menu_shell_get_parent_shell (GtkMenuShell *menu_shell);`
  pub fn gtk_menu_shell_get_parent_shell (menu_shell: *mut GtkMenuShell) -> *mut GtkWidget;
  /// C 1271: `(void)gtk_events_pending ();`
  /// C: gtk/gtkmain.h: `gboolean gtk_events_pending (void);`
  pub fn gtk_events_pending () -> gboolean;
}
/* The GTK/GDK entry points whose C *uses* sit inside an arm: C 207-221 and the three
   display functions' X branches call the two `gdk_x11_*` lookups and the two
   `x_display_pixel_*` measurements only under `#ifndef HAVE_PGTK`, and C 824-942's tooltip
   callbacks/preparer are that arm's too - so their declarations carry that gate rather than the
   file gate alone (C's own structure: the call is inside the arm).  */
#[cfg (all (feature = "use_gtk", not (feature = "have_pgtk")))]
unsafe extern "C" {
  /// C 207/307/321/373: `gdk_x11_lookup_xdisplay (FRAME_X_DISPLAY (f))` / `… (dpy)`
  /// C: gdk/gdkx.h: `GdkDisplay *gdk_x11_lookup_xdisplay (Display *xdisplay);`
  pub fn gdk_x11_lookup_xdisplay (xdisplay: *mut Display) -> *mut GdkDisplay;
  /// C 260: `GDK_DISPLAY_XDISPLAY (gdpy)`
  /// C: gdk/gdkx.h: `Display *gdk_x11_display_get_xdisplay (GdkDisplay *display);`
  pub fn gdk_x11_display_get_xdisplay (display: *mut GdkDisplay) -> *mut Display;
  /// C 867: `p = gtk_widget_get_parent (x->ttip_lbl);`
  /// C: gtk/gtkwidget.h: `GtkWidget *gtk_widget_get_parent (GtkWidget *widget);`
  pub fn gtk_widget_get_parent (widget: *mut GtkWidget) -> *mut GtkWidget;
  /// C 868: `list = gtk_container_get_children (GTK_CONTAINER (p));`
  /// C: gtk/gtkcontainer.h: `GList *gtk_container_get_children (GtkContainer *container);`
  pub fn gtk_container_get_children (container: *mut GtkContainer) -> *mut GList;
  /// C 861: `x->ttip_lbl = gtk_label_new ("");`
  /// C: gtk/gtklabel.h: `GtkWidget *gtk_label_new (const gchar *str);`
  pub fn gtk_label_new (str: *const gchar) -> *mut GtkWidget;
  /// C 873: `gtk_label_set_line_wrap (GTK_LABEL (w), FALSE);`
  /// C: gtk/gtklabel.h: `void gtk_label_set_line_wrap (GtkLabel *label, gboolean wrap);`
  pub fn gtk_label_set_line_wrap (label: *mut GtkLabel, wrap: gboolean);
  /// C 863/933: `gtk_tooltip_set_custom (tooltip, x->ttip_lbl);`
  /// C: gtk/gtktooltip.h: `void gtk_tooltip_set_custom (GtkTooltip *tooltip, GtkWidget *custom);`
  pub fn gtk_tooltip_set_custom (tooltip: *mut GtkTooltip, custom: *mut GtkWidget);
  /// C 916/984: `settings = gtk_settings_get_for_screen (screen);`
  /// C: gtk/gtksettings.h: `GtkSettings *gtk_settings_get_for_screen (GdkScreen *screen);`
  pub fn gtk_settings_get_for_screen (screen: *mut GdkScreen) -> *mut GtkSettings;
  /// C 928: `gtk_widget_get_display (GTK_WIDGET (x->ttip_window))`
  /// C: gtk/gtkwidget.h: `GdkDisplay *gtk_widget_get_display (GtkWidget *widget);`
  pub fn gtk_widget_get_display (widget: *mut GtkWidget) -> *mut GdkDisplay;
  /// C 934: `gtk_tooltip_set_text (x->ttip_widget, SSDATA (encoded_string));`
  /// C: gtk/gtktooltip.h: `void gtk_tooltip_set_text (GtkTooltip *tooltip, const gchar *text);`
  pub fn gtk_tooltip_set_text (tooltip: *mut GtkTooltip, text: *const gchar);
  /// C 935: `gtk_widget_get_preferred_size (GTK_WIDGET (x->ttip_window), NULL, &req);`
  /// C: gtk/gtkwidget.h: `void gtk_widget_get_preferred_size (GtkWidget *widget, GtkRequisition
  /// *minimum_size, GtkRequisition *natural_size);` (xterm.h 1434-1445's GTK2 arm redefines the
  /// name as `gtk_widget_size_request`, which `include/xterm.rs` 1016-1021 carries).
  pub fn gtk_widget_get_preferred_size (widget: *mut GtkWidget,
                                        minimum_size: *mut GtkRequisition,
                                        natural_size: *mut GtkRequisition);
  /// C 1062's `f->size_hint_flags & (USPosition | PPosition)`, C 1076/1082's two display
  /// measurements, C 1099/1131's two GTK calls that take the widget, and C 1280-1281's two frame
  /// utilities with their frame.h lines.
  /// C: xterm.h 1863-1864: `extern int x_display_pixel_height (struct x_display_info *);` /
  /// `extern int x_display_pixel_width (struct x_display_info *);`
  pub fn x_display_pixel_width (dpyinfo: *mut x_display_info) -> c_int;
  pub fn x_display_pixel_height (dpyinfo: *mut x_display_info) -> c_int;
}
/* C 768-800's `#ifdef HAVE_GTK3` arm of `xg_check_special_colors`: the three GTK3 style calls
   are that arm's, so they carry the version gate.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 769: `gsty = gtk_widget_get_style_context (FRAME_GTK_OUTER_WIDGET (f));`
  /// C: gtk/gtkstylecontext.h: `GtkStyleContext *gtk_widget_get_style_context (GtkWidget
  /// *widget);`
  pub fn gtk_widget_get_style_context (widget: *mut GtkWidget) -> *mut GtkStyleContext;
  /// C 773: `gtk_style_context_get_color (gsty, state, &col);`
  /// C: gtk/gtkstylecontext.h: `void gtk_style_context_get_color (GtkStyleContext *context,
  /// GtkStateFlags state, GdkRGBA *color);`
  pub fn gtk_style_context_get_color (context: *mut GtkStyleContext, state: GtkStateFlags,
                                      color: *mut GdkRGBA);
  /// C 780-782: `gtk_style_context_get (gsty, state, GTK_STYLE_PROPERTY_BACKGROUND_COLOR, &c,
  /// NULL);`
  /// C: gtk/gtkstylecontext.h: `void gtk_style_context_get (GtkStyleContext *context,
  /// GtkStateFlags state, ...);`
  pub fn gtk_style_context_get (context: *mut GtkStyleContext, state: GtkStateFlags, ...);
}

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
pub const TB_INFO_KEY: &CStr = c"xg_frame_tb_info";
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
/* C 1088/1094 (the `HAVE_PGTK` arm of `xg_set_geometry`) and C 795 (the pgtk arm of
   `xg_check_special_colors`): `pgtkterm.h`'s two pixel measurements and its colour parser,
   reached in C through `"gtkutil.h"` 29's `#include "pgtkterm.h"` (whose mirror is a
   placeholder, so they stand here - P-72 rule 4).  Only the pgtk arms name them, so the block
   carries that gate alone.  */
#[cfg (all (feature = "use_gtk", feature = "have_pgtk"))]
unsafe extern "C" {
  /// C: pgtkterm.h 601-602: `extern int pgtk_display_pixel_height (struct pgtk_display_info *);`
  /// / `extern int pgtk_display_pixel_width (struct pgtk_display_info *);`
  pub fn pgtk_display_pixel_width (dpyinfo: *mut pgtk_display_info) -> c_int;
  pub fn pgtk_display_pixel_height (dpyinfo: *mut pgtk_display_info) -> c_int;
  /// C: pgtkterm.h 592: `extern int pgtk_parse_color (struct frame *, const char *, Emacs_Color
  /// *);`
  pub fn pgtk_parse_color (f: *mut frame, color_name: *const c_char,
                           color: *mut Emacs_Color) -> c_int;
}
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
				     { x_display_pixel_width (FRAME_DISPLAY_INFO! (f)) })
				  - FRAME_PIXEL_WIDTH (f) + (*f).left_pos); }

	    if ((*f).size_hint_flags & YNegative) != 0
	      { (*f).top_pos = ((if ! FRAME_PARENT_FRAME (f).is_null ()
				    { FRAME_PIXEL_HEIGHT (FRAME_PARENT_FRAME (f)) }
				  else
				    { x_display_pixel_height (FRAME_DISPLAY_INFO! (f)) })
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
