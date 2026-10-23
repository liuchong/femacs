// femacs: port of src/pgtkselect.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C definitions that are not `static` carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`s except the six whose address C
// hands to another function (`pgtk_pop_current_selection_request`,
// `pgtk_selection_request_lisp_error`, `wait_for_property_change_unwind`, `pgtk_cancel_atimer`,
// `pgtk_display_selection_waiting_message` and `pgtk_display_selection_waiting_message`'s atimer
// callback shape), which are spelled `unsafe extern "C" fn` (PORTING.md 1.4/P-12: one definition
// per linkage unit, which is what C's `static` gives too).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `pgtkselect.o`
// sits in `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`; `configure.ac` 3878 fills
// it - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` - in the branch
// `window_system = pgtk`, whose `AC_DEFINE` is `configure.ac` 3176's `HAVE_PGTK`), and
// `docs/baseline/frozen-build-sources.txt` (106 lines of sources) lists no pgtk object: the file is
// not in this target's source list at all.  P-86 ① therefore applies - this is *transcription*, and
// the **file gate** is `feature = "have_pgtk"`, on every item **and on the include graph** (P-86 ⑧:
// an `include!` outside the gate breaks the frozen build at once, which is exactly why
// `build-tu src/pgtkselect.c` reads a 336-byte empty object - see the ledger's Evidence ①).  The C
// body sits under no `#if` of its own (the gate is the build system's), so
// `python3 tools/check-cond-eval.py src/pgtkselect.c` reads `3 conditions, 0 unknown, 0
// disagreements [VERIFIED]` and the file is on `docs/baseline/config/cond-eval-verified.txt` 345;
// the three conditions are the file's own `#if 0` blocks, which are dead in *every* configuration
// (`docs/status.md` 3.55) and are therefore recorded at their positions with no arm of their own -

// `prop_location_identifier` counter) and C 864-882 (`expect_property_change`'s definition).  The
// INCR machinery they belong to is what C 29-31's note says this file deliberately omits.
//
// The unit's own conditions are exactly those three `#if 0`s: there is no `#ifdef`, no `#else` and
// no `#elif` anywhere in the 1975 lines, so this port's only gate is the file gate (P-86 ⑥'s
// "file gate + every sub-gate" reduces to the file gate here, and the ledger says so).
//
// **The GTK/GDK surface's single home is `src/include/gtk.rs`** (P-90 ①②).  This file's C
// text does not itself `#include <gtk/gtk.h>`: the names arrive through C 23's `"pgtkterm.h"`, whose
// 31 is `#include <gtk/gtk.h>` under its own `HAVE_PGTK`.  That edge is owed by
// `include/pgtkterm.rs` (`## Owed` of `docs/ports/include-pgtkterm.md`), so - the `src/pgtkfns.rs`

// `include/gtk.rs` through it.  What `include/gtk.rs` does *not* carry is the GDK *selection*
// surface this file is made of: the atoms, `GdkPropMode`, `GtkTargetEntry`/`GtkTargetList` and the
// twenty entry points below are declared in-unit at C's `#include` position (PORTING.md P-88 ① - a
// unit-local segment, not a file under `src/include/`, which mirrors the reference's own
// headers), and their move into the mirror is owed (P-90 ③; `## Owed` of the ledger).
//
// **No GTK/GDK tree exists on this machine** (`ls -d /opt/homebrew/include/gtk-3.0
// /opt/homebrew/include/gtk-4.0` -> both absent; `pkg-config` is not installed; no `gtk/gtk.h` or
// `gdk/gdk.h` exists under `/opt/homebrew`, `/usr/local` or `/Library`), so the GDK segment's
// citations name the **header alone, no line**, every one of its values is a placeholder
// (`// placeholder, not GDK's value`) and no layout assertion is written for `GtkTargetEntry` - the
// `include/gtk.rs` 20-30 rule.  **GLib and cairo are on this machine**
// (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0), so `G_MAXUINT32` below quotes the header and the
// line it was read from (`glib/gtypes.h` 98) - a reading, not a memory.  The three records this
// file *defines* (`struct selection_data`, `struct prop_location`, `struct pgtk_selection_request`)
// are C's own and their layouts are clang readings on this machine (the ledger's Evidence ②:
// `clang -o /tmp/p layout.c && /tmp/p`), so they carry `const _:` assertions like
// `src/xselect.rs`'s.
//
// Two gaps stay the mirrors': `include/gtk.rs` declares `GdkEventProperty` and `GdkEventSelection`
// as zero-sized placeholders (pgtkterm.h 571-572 only ever took a pointer to them), while C 953-975
// and C 1791-1801 *read* `event->window`/`->atom`/`->state` and `event->selection`/`->property`, so
// those records have to grow the members their first member-reader touches (the status 3.481
// `GdkEventKey` shape); and `include/frame.rs` 861's `FRAME_PGTK_P` carries the frozen (non-pgtk)
// arm.  Both are mirror rows, named in the ledger's `## Owed` and `## Deviations`.
//
// `docs/ports/pgtkselect.md` carries the C line-range table, the acceptance readings, every
// deviation and the items the integrator owes.  No behaviour of this file was executed anywhere:
// there is no GTK tree to compile the arm against and no PGTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 22-27: the file's own include list.  Every edge carries the file gate (P-86 ⑧); C's lines stand
   as these comments where the mirror *is* what C would have included.  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (memcpy, memset)
#[cfg (feature = "have_pgtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>` (SHRT_MIN/MAX, LONG_WIDTH)
#[cfg (feature = "have_pgtk")]
include!("include/lisp.rs");
/* C 23's chain, second half: `"pgtkterm.h"` 31's `#include <gtk/gtk.h>` (under its `HAVE_PGTK`).
   The surface's single home is `include/gtk.rs` and this unit's stand-in edge stands here, at the C
   position that makes the names visible (the `src/pgtkfns.rs` C 28 shape); it drops when
   `include/pgtkterm.rs`'s own C 31 edge lands (P-88 ⑤).  */
#[cfg (feature = "have_pgtk")]
include!("include/gtk.rs");                    // pgtkterm.h 31: `#include <gtk/gtk.h>`
#[cfg (feature = "have_pgtk")]
include!("include/pgtkterm.rs");
#[cfg (feature = "have_pgtk")]
// [xtask dedupe] /Users/liu/projects/src/github.com/liuchong/femacs/femacs/src/include/termhooks.rs already included
#[cfg (feature = "have_pgtk")]
include!("include/keyboard.rs");
#[cfg (feature = "have_pgtk")]
include!("include/atimer.rs");
#[cfg (feature = "have_pgtk")]
include!("include/blockinput.rs");

/* C 29-31: "This file deliberately does not implement INCR, since it adds a bunch of extra code for
   no real gain, as PGTK isn't supposed to support X11 anyway."  Recorded; it is the reason the three
   `#if 0` blocks below carry no arm.  */

/* C 33-35: "Advance declaration of structs." and the two tag declarations `struct selection_data;`
   / `struct prop_location;`.  Rust has no incomplete-type tags for this shape - the two records are
   defined at their C positions (C 323-337, C 424-433) and are only ever named through pointers
   (PORTING.md P-72 ③).  */

/* C 37-56: the ten static forward declarations, dropped (PORTING.md 0.1-12: "C's own forward
   declarations have no Rust counterpart"); each definition stays at its C position.  C 41-44 is the
   `#if 0` around `static struct prop_location *expect_property_change (GdkDisplay *, GdkWindow *,
   GdkAtom, int);` - dead text in every configuration, recorded here and not transcribed.  */

/* ----------------------------------------------------------------------------------------------
   C 23's chain's GDK selection surface: the names this file uses and no mirror carries.  The
   surface's home is `include/gtk.rs` (P-90 ①②); this segment is declared in-unit at C's `#include`
   position (PORTING.md P-88 ①) because the mirror has not landed these names yet - the move is owed
   (`## Owed` of `docs/ports/pgtkselect.md`).  **No GDK header exists on this machine** (see the head
   note), so GDK's citations below name the header alone and every value is a placeholder; the
   prototypes are UNVERIFIED in the same sense (P-88 ④: "this segment is a stand-in of an external
   header; the reference tree has no file to pair it with").
   ---------------------------------------------------------------------------------------------- */

/// C: gdk/gdktypes.h: `#define GDK_NONE ((GdkAtom) 0)` - C 123/354/600/1227/1777 and the two
/// `cs->property == GDK_NONE` tests.  **UNVERIFIED** (a placeholder: no GDK header here).
#[cfg (feature = "have_pgtk")]
pub const GDK_NONE: GdkAtom = std::ptr::null_mut ();
/// C: gdk/gdktypes.h: `#define GDK_SELECTION_PRIMARY ((GdkAtom) 1)` - C 126.  **UNVERIFIED**:
/// placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_SELECTION_PRIMARY: GdkAtom = 1 as GdkAtom;
/// C: gdk/gdktypes.h: `#define GDK_SELECTION_SECONDARY ((GdkAtom) 2)` - C 128.  **UNVERIFIED**:
/// placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_SELECTION_SECONDARY: GdkAtom = 2 as GdkAtom;
/// C: gdk/gdktypes.h: `#define GDK_SELECTION_CLIPBOARD ((GdkAtom) 69)` - C 130.  **UNVERIFIED**:
/// placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_SELECTION_CLIPBOARD: GdkAtom = 69 as GdkAtom;
/// C: gdk/gdktypes.h: `#define GDK_SELECTION_TYPE_ATOM ((GdkAtom) 4)` - C 694, 1132, 1304.  The
/// placeholder is X11's own `XA_ATOM`; on Wayland the atom is interned and the value differs - the
/// point of the UNVERIFIED note.  **UNVERIFIED**: placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_SELECTION_TYPE_ATOM: GdkAtom = 4 as GdkAtom;
/// C: gdk/gdktypes.h: `#define GDK_SELECTION_TYPE_INTEGER ((GdkAtom) 19)` - C 1341, 1345, 1347,
/// 1368, 1372, 1374.  **UNVERIFIED**: placeholder (X11's `XA_INTEGER`).
#[cfg (feature = "have_pgtk")]
pub const GDK_SELECTION_TYPE_INTEGER: GdkAtom = 19 as GdkAtom;
/// C: gdk/gdktypes.h: `#define GDK_CURRENT_TIME 0L` - C 163, 579, 750.  **UNVERIFIED**: placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_CURRENT_TIME: guint32 = 0;
/// C: gdk/gdktypes.h: `#define GDK_PROP_MODE_APPEND ((GdkPropMode) 2)` - C 485.  **UNVERIFIED**:
/// placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_PROP_MODE_APPEND: GdkPropMode = 2;
/// C: gdk/gdktypes.h: `#define GDK_PROP_MODE_REPLACE ((GdkPropMode) 0)` - C 606, 642, 1222.
/// **UNVERIFIED**: placeholder.
#[cfg (feature = "have_pgtk")]
pub const GDK_PROP_MODE_REPLACE: GdkPropMode = 0;
/// C: gdk/gdktypes.h: `typedef enum { GDK_PROP_MODE_REPLACE, GDK_PROP_MODE_PREPEND,
/// GDK_PROP_MODE_APPEND } GdkPropMode;` - the type of `gdk_property_change`'s fifth parameter.
/// **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
pub type GdkPropMode = c_int;

/// C: gtk/gtkselection.h: `typedef struct { gchar *target; guint flags; guint info; }
/// GtkTargetEntry;` - C 159's `GtkTargetEntry *gtargets` and C 219-221's `.target` write.
/// `include/gtk.rs` does not carry it.  The three members are GDK/GTK's declaration and the layout
/// is therefore GDK/GTK's, so - the mirror's rule - no layout assertion is written here;
/// **UNVERIFIED** (no GTK header on this machine).
#[cfg (feature = "have_pgtk")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct GtkTargetEntry
{
  pub target: *mut gchar,   // C: gtkselection.h: `gchar *target;` (the only member C touches)
  pub flags: guint,         // C: gtkselection.h: `guint flags;`
  pub info: guint,          // C: gtkselection.h: `guint info;`
}
/// C: gtk/gtkselection.h: `typedef struct _GtkTargetList GtkTargetList;` - C 1814's `GtkTargetList
/// *list`.  **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct GtkTargetList
{
  _private: [u8; 0],
}

/// C: glib/gtypes.h 98: `#define G_MAXUINT32 ((guint32) 0xffffffff)` - C 1400's `G_MAXUINT32 <=
/// INTMAX_MAX` and `min (G_MAXUINT32, INTMAX_MAX)`.  **Measured on this machine** from
/// `/opt/homebrew/include/glib-2.0/glib/gtypes.h` (GLib 2.90.0): the value is `0xffffffff`.
#[cfg (feature = "have_pgtk")]
pub const G_MAXUINT32: guint32 = 0xffffffff;

/* The GDK and GTK entry points this file calls.  Each `/// C:` line quotes the call site and the
   header declaration it was read from; the declarations themselves are UNVERIFIED (P-88 ④).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C 135: `return gdk_atom_intern (SSDATA (SYMBOL_NAME (sym)), FALSE);`
  /// C: gdk/gdkatom.h: `GdkAtom gdk_atom_intern (const gchar *atom_name, gboolean
  /// only_if_exists);`
  pub fn gdk_atom_intern (atom_name: *const gchar, only_if_exists: gboolean) -> GdkAtom;
  /// C 1132/1135/1321/1332: `type == gdk_atom_intern_static_string ("ATOM_PAIR")` (and "NULL",
  /// "COMPOUND_TEXT", "UTF8_STRING").
  /// C: gdk/gdkatom.h: `GdkAtom gdk_atom_intern_static_string (const gchar *atom_name);`
  pub fn gdk_atom_intern_static_string (atom_name: *const gchar) -> GdkAtom;
  /// C 141: `return intern (gdk_atom_name (atom));`
  /// C: gdk/gdkatom.h: `gchar *gdk_atom_name (GdkAtom atom);`
  pub fn gdk_atom_name (atom: GdkAtom) -> *mut gchar;
  /// C 170-174: `!gdk_selection_owner_set_for_display (dpyinfo->display, FRAME_GDK_WINDOW (f),
  /// selection_atom, timestamp, TRUE)` and C 811-816/834-837's `NULL` owner.
  /// C: gdk/gdkselection.h: `gboolean gdk_selection_owner_set_for_display (GdkDisplay *display,
  /// GdkWindow *owner, GdkAtom selection, guint32 time_, gboolean send_event);`
  pub fn gdk_selection_owner_set_for_display (display: *mut GdkDisplay, owner: *mut GdkWindow,
                                              selection: GdkAtom, time_: guint32,
                                              send_event: gboolean) -> gboolean;
  /// C 317-320: `gdk_selection_send_notify (SELECTION_EVENT_REQUESTOR (event), ...);`
  /// C: gdk/gdkselection.h: `void gdk_selection_send_notify (GdkWindow *requestor, GdkAtom
  /// selection, GdkAtom target, GdkAtom property, guint32 time_);`
  pub fn gdk_selection_send_notify (requestor: *mut GdkWindow, selection: GdkAtom,
                                    target: GdkAtom, property: GdkAtom, time_: guint32);
  /// C 1042-1043: `gdk_selection_convert (requestor_window, selection_atom, type_atom,
  /// requestor_time);`
  /// C: gdk/gdkselection.h: `void gdk_selection_convert (GdkWindow *requestor, GdkAtom selection,
  /// GdkAtom target, guint32 time_);`
  pub fn gdk_selection_convert (requestor: *mut GdkWindow, selection: GdkAtom, target: GdkAtom,
                                time_: guint32);
  /// C 1208/1781: `gdk_selection_owner_get_for_display (display, selection_atom)`.
  /// C: gdk/gdkselection.h: `GdkWindow *gdk_selection_owner_get_for_display (GdkDisplay *display,
  /// GdkAtom selection);`
  pub fn gdk_selection_owner_get_for_display (display: *mut GdkDisplay, selection: GdkAtom)
                                              -> *mut GdkWindow;
  /// C 1117-1119: `length = gdk_selection_property_get (window, &data, &actual_type,
  /// &actual_format);`
  /// C: gdk/gdkselection.h: `gint gdk_selection_property_get (GdkWindow *requestor, guchar **data,
  /// GdkAtom *prop_type, gint *prop_format);`
  pub fn gdk_selection_property_get (requestor: *mut GdkWindow, data: *mut *mut guchar,
                                     prop_type: *mut GdkAtom, prop_format: *mut gint) -> gint;
  /* C 958's `gdk_window_get_display` used to be declared here; the row is
     `include/gtk.rs`'s now (P-26's one spelling per construct, status 3.725) and the call site at
     C 958 is unchanged.  */
  /// C 518: `gdk_window_set_events (window, 0);`
  /// C: gdk/gdkwindow.h: `void gdk_window_set_events (GdkWindow *window, GdkEventMask
  /// event_mask);`
  pub fn gdk_window_set_events (window: *mut GdkWindow, event_mask: c_int);
  /// C 483-486 / 605-606 / 631-634: `gdk_property_change (window, cs->property, cs->type,
  /// cs->format, GDK_PROP_MODE_APPEND|REPLACE, cs->data, cs->size);`
  /// C: gdk/gdkwindow.h: `void gdk_property_change (GdkWindow *window, GdkAtom property, GdkAtom
  /// type, gint format, GdkPropMode mode, const guchar *data, gint nelements);`
  pub fn gdk_property_change (window: *mut GdkWindow, property: GdkAtom, type_: GdkAtom,
                              format: gint, mode: GdkPropMode, data: *const guchar,
                              nelements: gint);
  /// C 1227-1228: `gdk_property_delete (window, property);`
  /// C: gdk/gdkwindow.h: `void gdk_property_delete (GdkWindow *window, GdkAtom property);`
  pub fn gdk_property_delete (window: *mut GdkWindow, property: GdkAtom);
  /// C 495: `gdk_display_flush (display);`
  /// C: gdk/gdkdisplay.h: `void gdk_display_flush (GdkDisplay *display);`
  pub fn gdk_display_flush (display: *mut GdkDisplay);
  /// C 523: `gdk_display_sync (display);`
  /// C: gdk/gdkdisplay.h: `void gdk_display_sync (GdkDisplay *display);`
  pub fn gdk_display_sync (display: *mut GdkDisplay);

  /* C 157's `gtk_get_current_event_time` used to be declared here; the row is
     `include/gtk.rs`'s now (P-26's one spelling per construct, status 3.727) and the call site at
     C 157 is unchanged.  */
  /// C 212/226-228: `gtk_selection_clear_targets (FRAME_GTK_WIDGET (f), selection_atom);` /
  /// `gtk_selection_add_targets (FRAME_GTK_WIDGET (f), selection_atom, gtargets, ntargets);`
  /// C: gtk/gtkselection.h: `void gtk_selection_clear_targets (GtkWidget *widget, GdkAtom
  /// selection);` / `void gtk_selection_add_targets (GtkWidget *widget, GdkAtom selection, const
  /// GtkTargetEntry *targets, guint ntargets);`
  pub fn gtk_selection_clear_targets (widget: *mut GtkWidget, selection: GdkAtom);
  pub fn gtk_selection_add_targets (widget: *mut GtkWidget, selection: GdkAtom,
                                    targets: *const GtkTargetEntry, ntargets: guint);
  /// C 1848: `list = gtk_target_list_new (entries, n);`
  /// C: gtk/gtkselection.h: `GtkTargetList *gtk_target_list_new (const GtkTargetEntry *targets,
  /// guint ntargets);`
  pub fn gtk_target_list_new (targets: *const GtkTargetEntry, ntargets: guint) -> *mut GtkTargetList;
  /// C 1849: `gtk_drag_dest_set_target_list (FRAME_GTK_WIDGET (f), list);`
  /// C: gtk/gtkselection.h: `void gtk_drag_dest_set_target_list (GtkWidget *widget,
  /// GtkTargetList *target_list);`
  pub fn gtk_drag_dest_set_target_list (widget: *mut GtkWidget, target_list: *mut GtkTargetList);
  /// C 1850: `gtk_target_list_unref (list);`
  /// C: gtk/gtkselection.h: `void gtk_target_list_unref (GtkTargetList *list);`
  pub fn gtk_target_list_unref (list: *mut GtkTargetList);
}

/* The `lisp.h`/`frame.h`/`dispextern.h` entry points this file calls that no included mirror
   carries, declared at their first use site with the reference header's line (P-88 ①; the ledger's
   `## Owed` names the move into each mirror).  */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C 587/588: `record_unwind_protect_void (pgtk_pop_current_selection_request);`
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C 939-940 / 1062-1063: `wait_reading_process_output (secs, nsecs, 0, false, ...);`
  /// C: lisp.h 5181: `extern int wait_reading_process_output (intmax_t, int, int, bool, Lisp_Object,
  /// struct Lisp_Process *, int);`
  pub fn wait_reading_process_output (time_limit: intmax_t, nsecs: c_int, read_kbd: c_int,
                                      do_display: bool, wait_for_cell: Lisp_Object,
                                      wait_proc: *mut c_void, just_wait_proc: c_int) -> c_int;
  /// C 773: `redisplay_preserve_echo_area (20);`
  /// C: dispextern.h 3589: `void redisplay_preserve_echo_area (int);`
  

  /// C 1823: `f = decode_window_system_frame (frame);`
  /// C: frame.h 1475: `extern struct frame *decode_window_system_frame (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C 96: `struct terminal *t = decode_live_terminal (object);`
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  /// (`include/termhooks.rs` 444 records that this prototype is one of the ten termhooks.h
  /// declares for the units that call them, not for the mirror.)
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
}

/* C 541: `ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH;`, the bit-field member C 779 reads
   (`event->kind != SELECTION_REQUEST_EVENT`).  `include/pgtkterm.rs` carries the storage unit
   (`selection_input_event::bits`) but no accessor for it; the mirror that owns the record is where
   this belongs (`## Owed`), and the body is P-80's member-named shape - the same body
   `include/xterm.rs` 1497 and `src/pgtkterm.rs` 190 give their own consumers.  */
#[cfg (feature = "have_pgtk")]
macro_rules! selection_input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
}

/* C's `max`/`min` (lisp.h 71-72) reach this file as macros over any integer type; the port's
   lisp.rs mirror carries them as ordinary `fn`s over `isize`, and C 933/1058's two folds are
   over `EMACS_INT`.  The same folds are therefore defined here as macros of the same name (a
   macro and a function share a name in Rust) - the `src/xselect.rs` 414-419 / `src/alloc.rs`
   124 shape.  */
#[cfg (feature = "have_pgtk")]
macro_rules! max {
  ($a:expr, $b:expr) => { if ($a) > ($b) { ($a) } else { ($b) } };
}
#[cfg (feature = "have_pgtk")]
macro_rules! min {
  ($a:expr, $b:expr) => { if ($a) < ($b) { ($a) } else { ($b) } };
}

/* C 58-59: "Allocate an array of NITEMS items, each of positive size ITEM_SIZE.  Make room for an
   extra byte at the end, as GDK sometimes needs that."  */

/// C 61-67: `pgtk_nalloc`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_nalloc (nitems: ptrdiff_t, item_size: ptrdiff_t) -> *mut c_void   // C 61-67
{
  /* To pacify gcc --Wanalyzer-allocation-size, make room for an extra
     item at the end instead of just the extra byte GDK sometimes needs.  */
  return xnmalloc (nitems + 1, item_size);
}

/* C 69-73: the `frame_for_pgtk_selection` comment.  */

/// C 75-114: `frame_for_pgtk_selection`.
#[cfg (feature = "have_pgtk")]
unsafe fn frame_for_pgtk_selection (object: Lisp_Object) -> *mut frame      // C 75-114
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  if NILP (object)
    {
      f = XFRAME (selected_frame);
      if FRAME_PGTK_P (f) && FRAME_LIVE_P (f)
	{ return f; }

      FOR_EACH_FRAME! (tail, frame,
	{
	  f = XFRAME (frame);
	  if FRAME_PGTK_P (f) && FRAME_LIVE_P (f)
	    { return f; }
	});
    }
  else if TERMINALP (object)
    {
      let t: *mut terminal = decode_live_terminal (object);

      if (*t).r#type == output_pgtk
	{ FOR_EACH_FRAME! (tail, frame,
	    {
	      f = XFRAME (frame);
	      if FRAME_LIVE_P (f) && (*f).terminal == t
		{ return f; }
	    }); }
    }
  else if FRAMEP (object)
    {
      f = XFRAME (object);
      if FRAME_PGTK_P (f) && FRAME_LIVE_P (f)
	{ return f; }
    }

  return std::ptr::null_mut ();
}

/// C 116-117: `#define LOCAL_SELECTION(selection_symbol, dpyinfo)
/// assq_no_quit (selection_symbol, dpyinfo->terminal->Vselection_alist)`
#[cfg (feature = "have_pgtk")]
macro_rules! LOCAL_SELECTION {
  ($selection_symbol:expr, $dpyinfo:expr) => {
    assq_no_quit ($selection_symbol, (*(*$dpyinfo).terminal).Vselection_alist)
  };
}

/// C 119-136: `symbol_to_gdk_atom`.
#[cfg (feature = "have_pgtk")]
unsafe fn symbol_to_gdk_atom (sym: Lisp_Object) -> GdkAtom                 // C 119-136
{
  if NILP (sym)
    { return GDK_NONE; }

  if EQ (sym, QPRIMARY ())
    { return GDK_SELECTION_PRIMARY; }
  if EQ (sym, QSECONDARY ())
    { return GDK_SELECTION_SECONDARY; }
  if EQ (sym, QCLIPBOARD ())
    { return GDK_SELECTION_CLIPBOARD; }

  if !SYMBOLP (sym)
    { emacs_abort (); }

  return gdk_atom_intern (SSDATA (SYMBOL_NAME (sym)) as *const gchar, FALSE);
}

/// C 138-142: `gdk_atom_to_symbol`.
#[cfg (feature = "have_pgtk")]
unsafe fn gdk_atom_to_symbol (atom: GdkAtom) -> Lisp_Object                // C 138-142
{
  return intern (gdk_atom_name (atom));
}

/* C 146-149: the `pgtk_own_selection` comment.  */

/// C 151-233: `pgtk_own_selection` (C's `static void`; its one caller is
/// `Fpgtk_own_selection_internal` below).
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_own_selection (selection_name: Lisp_Object, selection_value: Lisp_Object,
                              frame: Lisp_Object)                          // C 151-233
{
  let f: *mut frame = XFRAME (frame);
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  let mut timestamp: guint32 = gtk_get_current_event_time ();
  let selection_atom: GdkAtom = symbol_to_gdk_atom (selection_name);
  let targets: Lisp_Object;
  let mut i: ptrdiff_t;
  let mut ntargets: ptrdiff_t;
  let mut gtargets: *mut GtkTargetEntry = std::ptr::null_mut ();   // 1.5: C leaves it unset

  if timestamp == GDK_CURRENT_TIME
    { timestamp = (*dpyinfo).last_user_time; }

  /* Assert ownership over the selection.  Ideally we would use only
     the GDK selection API for this, but it just doesn't work on
     Wayland.  */

  if gdk_selection_owner_set_for_display ((*dpyinfo).gdpy,
					  FRAME_GDK_WINDOW! (f),
					  selection_atom,
					  timestamp, TRUE) == 0
    { signal_error (c"Could not assert ownership over selection".as_ptr (), selection_name); }

  /* Update the local cache */
  {
    let selection_data: Lisp_Object;
    let prev_value: Lisp_Object;

    selection_data = list4 (selection_name, selection_value,
			    INT_TO_INTEGER (timestamp as EMACS_INT), frame);
    prev_value = LOCAL_SELECTION! (selection_name, dpyinfo);

    tset_selection_alist
      ((*dpyinfo).terminal,
       Fcons (selection_data, (*(*dpyinfo).terminal).Vselection_alist));

    /* If we already owned the selection, remove the old selection
       data.  Don't use Fdelq as that may quit.  */
    if !NILP (prev_value)
      {
	/* We know it's not the CAR, so it's easy.  */
	let mut rest: Lisp_Object = (*(*dpyinfo).terminal).Vselection_alist;
	while CONSP (rest)
	  {
	    if EQ (prev_value, Fcar (XCDR (rest)))
	      {
		XSETCDR (rest, XCDR (XCDR (rest)));
		break;
	      }
	    rest = XCDR (rest);
	  }
      }
  }

  /* Announce the targets to the display server.  This isn't required
     on X, but is on Wayland.  */

  targets = pgtk_get_local_selection (selection_name, QTARGETS (),
				      true, dpyinfo);

  /* GC must not happen inside this segment.  */
  block_input ();
  gtk_selection_clear_targets (FRAME_GTK_WIDGET! (f), selection_atom);

  if VECTORP (targets)
    {
      gtargets = xcalloc (ASIZE (targets) as usize,
			  std::mem::size_of::<GtkTargetEntry> ()) as *mut GtkTargetEntry;
      ntargets = 0;

      i = 0;
      while i < ASIZE (targets)
	{
	  if SYMBOLP (AREF (targets, i))
	    {
	      (*gtargets.offset (ntargets as isize)).target
		= SSDATA (SYMBOL_NAME (AREF (targets, i)));
	      ntargets += 1;
	    }
	  i += 1;
	}

      gtk_selection_add_targets (FRAME_GTK_WIDGET! (f),
				 selection_atom, gtargets,
				 ntargets as guint);

      xfree (gtargets as *mut c_void);
    }
  unblock_input ();
}

/// C 235-312: `pgtk_get_local_selection`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_local_selection (selection_symbol: Lisp_Object, target_type: Lisp_Object,
                                    local_request: bool, dpyinfo: *mut pgtk_display_info)
                                    -> Lisp_Object
{
  let mut local_value: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut handler_fn: Lisp_Object;
  let mut value: Lisp_Object;
  let mut check: Lisp_Object;

  local_value = LOCAL_SELECTION! (selection_symbol, dpyinfo);

  if NILP (local_value) { return Qnil (); }

  /* TIMESTAMP is a special case.  */
  if EQ (target_type, QTIMESTAMP ())
    {
      handler_fn = Qnil ();
      value = XCAR (XCDR (XCDR (local_value)));
    }
  else
    {
      /* Don't allow a quit within the converter.
	 When the user types C-g, he would be surprised
	 if by luck it came during a converter.  */
      let count: specpdl_ref = SPECPDL_INDEX ();
      specbind (Qinhibit_quit (), Qt ());

      CHECK_SYMBOL (target_type);
      handler_fn = Fcdr (Fassq (target_type, Vselection_converter_alist));

      if CONSP (handler_fn)
	{ handler_fn = XCDR (handler_fn); }

      tem = XCAR (XCDR (local_value));

      if STRINGP (tem)
	{
	  local_value = Fget_text_property (make_fixnum (0),
					    target_type, tem);

	  if !NILP (local_value)
	    { tem = local_value; }
	}

      if !NILP (handler_fn)
	{ value = calln! (handler_fn, selection_symbol,
			  if local_request { Qnil () } else { target_type },
			  tem); }
      else
	{ value = Qnil (); }
      value = unbind_to (count, value);
    }

  /* Make sure this value is of a type that we could transmit
     to another client.  */

  check = value;
  if CONSP (value)
      && SYMBOLP (XCAR (value))
    { check = XCDR (value); }

  if STRINGP (check)
      || VECTORP (check)
      || SYMBOLP (check)
      || INTEGERP (check)
      || NILP (value)
    { return value; }
  /* Check for a value that CONS_TO_INTEGER could handle.  */
  else if CONSP (check)
	   && INTEGERP (XCAR (check))
	   && (INTEGERP (XCDR (check))
	       || (CONSP (XCDR (check))
		   && INTEGERP (XCAR (XCDR (check)))
		   && NILP (XCDR (XCDR (check)))))
    { return value; }

  signal_error (c"Invalid data returned by selection-conversion function".as_ptr (),
		list2 (handler_fn, value));
}

/// C 314-321: `pgtk_decline_selection_request`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_decline_selection_request (event: *mut selection_input_event)   // C 314-321
{
  gdk_selection_send_notify (SELECTION_EVENT_REQUESTOR! (event),
			     SELECTION_EVENT_SELECTION! (event),
			     SELECTION_EVENT_TARGET! (event),
			     GDK_NONE, SELECTION_EVENT_TIME! (event));
}

/* C 323-337: `struct selection_data`.  Measured on this machine (`clang` probe, the ledger's
   Evidence ②): `sizeof` **64**, `data`@0, `size`@8, `format`@16, `type`@24, `nofree`@32,
   `property`@40, `wait_object`@48, `next`@56.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct selection_data
{
  pub data: *mut c_uchar,                 // C 325: `unsigned char *data;`
  pub size: ptrdiff_t,                    // C 326: `ptrdiff_t size;`
  pub format: c_int,                      // C 327: `int format;`
  pub r#type: GdkAtom,                    // C 328: `GdkAtom type;`
  pub nofree: bool,                       // C 329: `bool nofree;`
  pub property: GdkAtom,                  // C 330: `GdkAtom property;`
  /// This can be set to non-NULL during x_reply_selection_request, if the selection is waiting
  /// for an INCR transfer to complete.  Don't free these; that's done by
  /// `unexpect_property_change`.
  pub wait_object: *mut prop_location,    // C 335: `struct prop_location *wait_object;`
  pub next: *mut selection_data,          // C 336: `struct selection_data *next;`
}
#[cfg (feature = "have_pgtk")]
const _: () = {
  core::assert! (core::mem::size_of::<selection_data> () == 64);
  core::assert! (core::mem::offset_of! (selection_data, data) == 0);
  core::assert! (core::mem::offset_of! (selection_data, size) == 8);
  core::assert! (core::mem::offset_of! (selection_data, format) == 16);
  core::assert! (core::mem::offset_of! (selection_data, r#type) == 24);
  core::assert! (core::mem::offset_of! (selection_data, nofree) == 32);
  core::assert! (core::mem::offset_of! (selection_data, property) == 40);
  core::assert! (core::mem::offset_of! (selection_data, wait_object) == 48);
  core::assert! (core::mem::offset_of! (selection_data, next) == 56);
};

/* C 339-358: `struct pgtk_selection_request`.  Measured: `sizeof` **48**, `last`@0, `dpyinfo`@8,
   `request`@16, `converted_selections`@24, `conversion_fail_tag`@32, `converted`@40.  */
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct pgtk_selection_request
{
  pub last: *mut pgtk_selection_request,  // C 342: `struct pgtk_selection_request *last;`
  pub dpyinfo: *mut pgtk_display_info,    // C 345: `struct pgtk_display_info *dpyinfo;`
  pub request: *mut selection_input_event,   // C 348: `struct selection_input_event *request;`
  pub converted_selections: *mut selection_data,   // C 351
  pub conversion_fail_tag: GdkAtom,       // C 354: `GdkAtom conversion_fail_tag;`
  pub converted: bool,                    // C 357: `bool converted;`
}
#[cfg (feature = "have_pgtk")]
const _: () = {
  core::assert! (core::mem::size_of::<pgtk_selection_request> () == 48);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, last) == 0);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, dpyinfo) == 8);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, request) == 16);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, converted_selections) == 24);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, conversion_fail_tag) == 32);
  core::assert! (core::mem::offset_of! (pgtk_selection_request, converted) == 40);
};

/* C 360-361: "Stack of selections currently being processed.  NULL if all requests have been fully
   processed."  */
#[cfg (feature = "have_pgtk")]
static mut selection_request_stack: *mut pgtk_selection_request = std::ptr::null_mut ();   // C 363

/// C 365-380: `pgtk_push_current_selection_request`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_push_current_selection_request (se: *mut selection_input_event,
                                               dpyinfo: *mut pgtk_display_info)   // C 365-380
{
  let frame: *mut pgtk_selection_request;

  frame = xmalloc (std::mem::size_of::<pgtk_selection_request> ()) as *mut pgtk_selection_request;
  (*frame).converted = false;
  (*frame).last = selection_request_stack;
  (*frame).request = se;
  (*frame).dpyinfo = dpyinfo;
  (*frame).converted_selections = std::ptr::null_mut ();
  (*frame).conversion_fail_tag = GDK_NONE;

  selection_request_stack = frame;
}

/// C 382-391: `pgtk_pop_current_selection_request` (its address goes to
/// `record_unwind_protect_void`, so `unsafe extern "C" fn`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_pop_current_selection_request ()               // C 382-391
{
  let tem: *mut pgtk_selection_request;

  tem = selection_request_stack;
  selection_request_stack = (*selection_request_stack).last;

  xfree (tem as *mut c_void);
}

/* C 393-395: the "Used as an unwind-protect clause" comment.  */

/// C 397-420: `pgtk_selection_request_lisp_error` (its address goes to
/// `record_unwind_protect_void`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_selection_request_lisp_error ()                // C 397-420
{
  let mut cs: *mut selection_data;
  let mut next: *mut selection_data;
  let frame: *mut pgtk_selection_request;

  frame = selection_request_stack;

  cs = (*frame).converted_selections;
  while !cs.is_null ()
    {
      next = (*cs).next;
      if !(*cs).nofree && !(*cs).data.is_null ()
	{ xfree ((*cs).data as *mut c_void); }
      xfree (cs as *mut c_void);
      cs = next;
    }
  (*frame).converted_selections = std::ptr::null_mut ();

  if !(*frame).converted && !(*(*frame).dpyinfo).gdpy.is_null ()
    { pgtk_decline_selection_request ((*frame).request); }
}

/* C 418-420: "This stuff is so that INCR selections are reentrant (that is, so we can be servicing
   multiple INCR selection requests simultaneously.)  I haven't actually tested that yet."  */

/* C 422: "Keep a list of the property changes that are awaited."  */

/// C 424-433: `struct prop_location`.  Measured: `sizeof` **48**, `identifier`@0, `display`@8,
/// `window`@16, `property`@24, `desired_state`@32, `arrived`@36, `next`@40.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct prop_location
{
  pub identifier: c_int,                // C 426: `int identifier;`
  pub display: *mut GdkDisplay,         // C 427: `GdkDisplay *display;`
  pub window: *mut GdkWindow,           // C 428: `GdkWindow *window;`
  pub property: GdkAtom,                // C 429: `GdkAtom property;`
  pub desired_state: c_int,             // C 430: `int desired_state;`
  pub arrived: bool,                    // C 431: `bool arrived;`
  pub next: *mut prop_location,         // C 432: `struct prop_location *next;`
}
#[cfg (feature = "have_pgtk")]
const _: () = {
  core::assert! (core::mem::size_of::<prop_location> () == 48);
  core::assert! (core::mem::offset_of! (prop_location, identifier) == 0);
  core::assert! (core::mem::offset_of! (prop_location, display) == 8);
  core::assert! (core::mem::offset_of! (prop_location, window) == 16);
  core::assert! (core::mem::offset_of! (prop_location, property) == 24);
  core::assert! (core::mem::offset_of! (prop_location, desired_state) == 32);
  core::assert! (core::mem::offset_of! (prop_location, arrived) == 36);
  core::assert! (core::mem::offset_of! (prop_location, next) == 40);
};

/* C 435-439 is `#if 0` / `static int prop_location_identifier;` / `#endif`: C's own dead text in
   every configuration (`docs/status.md` 3.55), the counter `expect_property_change` would have used.
   Recorded, not transcribed.  */

#[cfg (feature = "have_pgtk")]
static mut property_change_reply: Lisp_Object = lisp_h_Qnil ();        // C 441

#[cfg (feature = "have_pgtk")]
static mut property_change_reply_object: *mut prop_location = std::ptr::null_mut ();   // C 443

#[cfg (feature = "have_pgtk")]
static mut property_change_wait_list: *mut prop_location = std::ptr::null_mut ();      // C 445

/// C 447-456: `set_property_change_object`.
#[cfg (feature = "have_pgtk")]
unsafe fn set_property_change_object (location: *mut prop_location)      // C 447-456
{
  /* Input must be blocked so we don't get the event before we set these.  */
  if !input_blocked_p ()
    { emacs_abort (); }

  XSETCAR (property_change_reply, Qnil ());
  property_change_reply_object = location;
}

/* C 459: "Send the reply to a selection request event EVENT."  */

/// C 461-525: `pgtk_reply_selection_request`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_reply_selection_request (event: *mut selection_input_event,
                                        dpyinfo: *mut pgtk_display_info)   // C 461-525
{
  let display: *mut GdkDisplay = SELECTION_EVENT_DISPLAY (event);
  let window: *mut GdkWindow = SELECTION_EVENT_REQUESTOR! (event);
  let mut cs: *mut selection_data;
  let frame: *mut pgtk_selection_request;

  frame = selection_request_stack;

  block_input ();
  /* Loop over converted selections, storing them in the requested
     properties.  If data is large, only store the first N bytes
     (section 2.7.2 of ICCCM).  Note that we store the data for a
     MULTIPLE request in the opposite order; the ICCM says only that
     the conversion itself must be done in the same order. */
  cs = (*frame).converted_selections;
  while !cs.is_null ()
    {
      'step: {
	if (*cs).property == GDK_NONE
	  { break 'step; }        /* C 481: `continue;` (the stepper follows the body).  */

	gdk_property_change (window, (*cs).property,
			     (*cs).r#type, (*cs).format,
			     GDK_PROP_MODE_APPEND,
			     (*cs).data, (*cs).size as c_int);
      }
      cs = (*cs).next;
    }

  /* Now issue the SelectionNotify event.  */
  gdk_selection_send_notify (window,
			     SELECTION_EVENT_SELECTION! (event),
			     SELECTION_EVENT_TARGET! (event),
			     SELECTION_EVENT_PROPERTY! (event),
			     SELECTION_EVENT_TIME! (event));
  gdk_display_flush (display);

  /* Finish sending the rest of each of the INCR values.  This should
     be improved; there's a chance of deadlock if more than one
     subtarget in a MULTIPLE selection requires an INCR transfer, and
     the requestor and Emacs loop waiting on different transfers.  */
  cs = (*frame).converted_selections;
  while !cs.is_null ()
    {
      if !(*cs).wait_object.is_null ()
	{
	  /* Must set this inside block_input ().  unblock_input may read
	     events and setting property_change_reply in
	     wait_for_property_change is then too late.  */
	  set_property_change_object ((*cs).wait_object);
	  unblock_input ();

	  /* Wait for the requestor to ack by deleting the property.
	     This can run Lisp code (process handlers) or signal.  */
	  wait_for_property_change ((*cs).wait_object);

	  /* Now write a zero-length chunk to the property to tell the
	     requestor that we're done.  */
	  block_input ();
	  if !waiting_for_other_props_on_window (display, window)
	    { gdk_window_set_events (window, 0); }
	  gdk_property_change (window, (*cs).property, (*cs).r#type, (*cs).format,
			       GDK_PROP_MODE_REPLACE, (*cs).data, 0);
	}
      cs = (*cs).next;
    }

  gdk_display_sync (display);
  unblock_input ();
}

/* C 529-530: the `pgtk_handle_selection_request` comment.  */

/// C 532-662: `pgtk_handle_selection_request`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_handle_selection_request (event: *mut selection_input_event)   // C 532-662
{
  let mut local_selection_time: guint32 = 0;   // 1.5: C leaves it unset
  let dpyinfo: *mut pgtk_display_info = SELECTION_EVENT_DPYINFO! (event);
  let selection: GdkAtom = SELECTION_EVENT_SELECTION! (event);
  let mut selection_symbol: Lisp_Object = gdk_atom_to_symbol (selection);
  let target: GdkAtom = SELECTION_EVENT_TARGET! (event);
  let target_symbol: Lisp_Object = gdk_atom_to_symbol (target);
  let mut property: GdkAtom = SELECTION_EVENT_PROPERTY! (event);
  let local_selection_data: Lisp_Object;
  let mut success: bool = false;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut pushed: bool;
  let mut alias: Lisp_Object;
  let mut tem: Lisp_Object;

  alias = Vpgtk_selection_alias_alist;

  FOR_EACH_TAIL_SAFE! (alias,
    {
      tem = Qnil ();

      if CONSP (alias)
	{ tem = XCAR (alias); }

      if CONSP (tem)
	  && EQ (XCAR (tem), selection_symbol)
	  && SYMBOLP (XCDR (tem))
	{
	  selection_symbol = XCDR (tem);
	  break;
	}
    });

  pushed = false;

  'done: loop
    {
      if dpyinfo.is_null ()
	{ break 'done; }        /* C 569: `goto DONE;`  */

      local_selection_data = LOCAL_SELECTION! (selection_symbol, dpyinfo);

      /* Decline if we don't own any selections.  */
      if NILP (local_selection_data) { break 'done; }

      /* Decline requests issued prior to our acquiring the selection.  */
      /* C 577-578's `CONS_TO_INTEGER (..., guint32, local_selection_time)` is the unsigned
	 helper with `TYPE_MAXIMUM (guint32)` (lisp.h 4188-4195).  */
      local_selection_time
	= cons_to_unsigned (XCAR (XCDR (XCDR (local_selection_data))),
			    G_MAXUINT32 as uintmax_t) as guint32;
      if SELECTION_EVENT_TIME! (event) != GDK_CURRENT_TIME
	  && local_selection_time > SELECTION_EVENT_TIME! (event)
	{ break 'done; }

      block_input ();
      pushed = true;
      pgtk_push_current_selection_request (event, dpyinfo);
      record_unwind_protect_void (Some (pgtk_pop_current_selection_request));
      record_unwind_protect_void (Some (pgtk_selection_request_lisp_error));
      unblock_input ();

      if EQ (target_symbol, QMULTIPLE ())
	{
	  /* For MULTIPLE targets, the event property names a list of atom
	     pairs; the first atom names a target and the second names a
	     non-GDK_NONE property.  */
	  let requestor: *mut GdkWindow = SELECTION_EVENT_REQUESTOR! (event);
	  let multprop: Lisp_Object;
	  let mut j: ptrdiff_t;
	  let nselections: ptrdiff_t;
	  let mut cs: selection_data = std::mem::zeroed ();

	  if property == GDK_NONE
	    { break 'done; }

	  multprop
	    = pgtk_get_window_property_as_lisp_data (dpyinfo,
						     requestor,
						     property,
						     QMULTIPLE (),
						     selection,
						     true);

	  if !VECTORP (multprop) || ASIZE (multprop) % 2 != 0
	    { break 'done; }

	  nselections = ASIZE (multprop) / 2;
	  /* Perform conversions.  This can signal.  */
	  j = 0;
	  while j < nselections
	    {
	      let subtarget: Lisp_Object = AREF (multprop, 2 * j);
	      let subproperty: GdkAtom = symbol_to_gdk_atom (AREF (multprop, 2 * j + 1));
	      let mut subsuccess: bool = false;

	      if subproperty != GDK_NONE
		{ subsuccess = pgtk_convert_selection (selection_symbol, subtarget,
						       subproperty, true, dpyinfo); }
	      if !subsuccess
		{ ASET (multprop, 2 * j + 1, Qnil ()); }
	      j += 1;
	    }
	  /* Save conversion results */
	  lisp_data_to_selection_data (dpyinfo, multprop, &raw mut cs);
	  gdk_property_change (requestor, property,
			       cs.r#type, cs.format,
			       GDK_PROP_MODE_REPLACE,
			       cs.data, cs.size as c_int);
	  success = true;
	}
      else
	{
	  if property == GDK_NONE
	    { property = SELECTION_EVENT_TARGET! (event); }

	  success = pgtk_convert_selection (selection_symbol,
					    target_symbol, property,
					    false, dpyinfo);
	}

      break 'done;              /* C 645's `DONE:` - the section below is what C jumps to.  */
    }

  /* DONE: */

  if pushed
    { (*selection_request_stack).converted = true; }

  if success
    { pgtk_reply_selection_request (event, dpyinfo); }
  else
    { pgtk_decline_selection_request (event); }

  /* Run the `pgtk-sent-selection-functions' abnormal hook.  */
  if !NILP (Vpgtk_sent_selection_functions)
      && !BASE_EQ (Vpgtk_sent_selection_functions, Qunbound ())
    { CALLN! (Frun_hook_with_args, Qpgtk_sent_selection_functions (),
	      selection_symbol, target_symbol, if success { Qt () } else { Qnil () }); }

  unbind_to (count, Qnil ());
}

/* C 664-669: the `pgtk_convert_selection` comment.  */

/// C 671-718: `pgtk_convert_selection`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_convert_selection (selection_symbol: Lisp_Object, target_symbol: Lisp_Object,
                                  property: GdkAtom, for_multiple: bool,
                                  dpyinfo: *mut pgtk_display_info) -> bool   // C 671-718
{
  let lisp_selection: Lisp_Object;
  let cs: *mut selection_data;
  let frame: *mut pgtk_selection_request;

  lisp_selection
    = pgtk_get_local_selection (selection_symbol, target_symbol,
				false, dpyinfo);

  frame = selection_request_stack;

  /* A nil return value means we can't perform the conversion.  */
  if NILP (lisp_selection)
      || (CONSP (lisp_selection) && NILP (XCDR (lisp_selection)))
    {
      if for_multiple
	{
	  cs = xmalloc (std::mem::size_of::<selection_data> ()) as *mut selection_data;
	  (*cs).data = (&raw mut (*selection_request_stack).conversion_fail_tag)
	               as *mut c_uchar;
	  (*cs).size = 1;
	  (*cs).format = 32;
	  (*cs).r#type = GDK_SELECTION_TYPE_ATOM;
	  (*cs).nofree = true;
	  (*cs).property = property;
	  (*cs).wait_object = std::ptr::null_mut ();
	  (*cs).next = (*frame).converted_selections;
	  (*frame).converted_selections = cs;
	}

      return false;
    }

  /* Otherwise, record the converted selection to binary.  */
  cs = xmalloc (std::mem::size_of::<selection_data> ()) as *mut selection_data;
  (*cs).data = std::ptr::null_mut ();
  (*cs).nofree = true;
  (*cs).property = property;
  (*cs).wait_object = std::ptr::null_mut ();
  (*cs).next = (*frame).converted_selections;
  (*frame).converted_selections = cs;
  lisp_data_to_selection_data (dpyinfo, lisp_selection, cs);
  return true;
}

/* C 722-724: the `pgtk_handle_selection_clear` comment.  */

/// C 726-774: `pgtk_handle_selection_clear`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_handle_selection_clear (event: *mut selection_input_event)   // C 726-774
{
  let selection: GdkAtom = SELECTION_EVENT_SELECTION! (event);
  let changed_owner_time: guint32 = SELECTION_EVENT_TIME! (event);

  let mut selection_symbol: Lisp_Object;
  let mut local_selection_data: Lisp_Object;
  let mut local_selection_time: guint32 = 0;   // 1.5: C leaves it unset
  let dpyinfo: *mut pgtk_display_info = SELECTION_EVENT_DPYINFO! (event);
  let mut Vselection_alist: Lisp_Object;

  if dpyinfo.is_null () { return; }

  selection_symbol = gdk_atom_to_symbol (selection);
  local_selection_data = LOCAL_SELECTION! (selection_symbol, dpyinfo);

  /* Well, we already believe that we don't own it, so that's just fine.  */
  if NILP (local_selection_data) { return; }

  /* C 745-746's `CONS_TO_INTEGER (..., guint32, local_selection_time)`: the same unsigned
     helper with `TYPE_MAXIMUM (guint32)`.  */
  local_selection_time
    = cons_to_unsigned (XCAR (XCDR (XCDR (local_selection_data))),
			G_MAXUINT32 as uintmax_t) as guint32;

  /* We have reasserted the selection since this SelectionClear was
     generated, so we can disregard it.  */
  if changed_owner_time != GDK_CURRENT_TIME
      && local_selection_time > changed_owner_time
    { return; }

  /* Otherwise, really clear.  Don't use Fdelq as that may quit.  */
  Vselection_alist = (*(*dpyinfo).terminal).Vselection_alist;
  if EQ (local_selection_data, CAR (Vselection_alist))
    { Vselection_alist = XCDR (Vselection_alist); }
  else
    {
      let mut rest: Lisp_Object;
      rest = Vselection_alist;
      while CONSP (rest)
	{
	  if EQ (local_selection_data, CAR (XCDR (rest)))
	    {
	      XSETCDR (rest, XCDR (XCDR (rest)));
	      break;
	    }
	  rest = XCDR (rest);
	}
    }
  tset_selection_alist ((*dpyinfo).terminal, Vselection_alist);

  /* Run the `pgtk-lost-selection-functions' abnormal hook.  */
  CALLN! (Frun_hook_with_args, Qpgtk_lost_selection_functions (), selection_symbol);

  redisplay_preserve_echo_area (20);
}

/// C 776-783: `pgtk_handle_selection_event`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_handle_selection_event (event: *mut selection_input_event)   // C 776-783
{
  if selection_input_event_bits! (kind, event) != SELECTION_REQUEST_EVENT
    { pgtk_handle_selection_clear (event); }
  else
    { pgtk_handle_selection_request (event); }
}

/* C 785-786: the `pgtk_clear_frame_selections` comment.  */

/// C 788-842: `pgtk_clear_frame_selections`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_clear_frame_selections (f: *mut frame)      // C 788-842
{
  let mut frame: Lisp_Object;
  let mut rest: Lisp_Object;
  let mut timestamp: Lisp_Object;
  let mut symbol: Lisp_Object;
  let mut time: guint32 = 0;   // 1.5: C leaves it unset
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  let t: *mut terminal = (*dpyinfo).terminal;

  XSETFRAME! (frame, f);

  /* Delete elements from the beginning of Vselection_alist.  */
  while CONSP ((*t).Vselection_alist)
	&& EQ (frame, XCAR (XCDR (XCDR (XCDR (XCAR ((*t).Vselection_alist))))))
    {
      symbol = Fcar (Fcar ((*t).Vselection_alist));

      /* Run the `pgtk-lost-selection-functions' abnormal hook.  */
      CALLN! (Frun_hook_with_args, Qpgtk_lost_selection_functions (),
	      symbol);

      timestamp = Fcar (Fcdr (Fcdr (Fcar ((*t).Vselection_alist))));
      /* C 808's `CONS_TO_INTEGER (timestamp, guint32, time)`.  */
      time = cons_to_unsigned (timestamp, G_MAXUINT32 as uintmax_t) as guint32;

      /* On Wayland, GDK will still ask the (now non-existent) frame for
	 selection data, even though we no longer think the selection is
	 owned by us.  Manually relinquish ownership of the selection.  */
      gdk_selection_owner_set_for_display ((*dpyinfo).gdpy,
					   std::ptr::null_mut (),
					   symbol_to_gdk_atom (symbol),
					   time, TRUE);

      tset_selection_alist (t, XCDR ((*t).Vselection_alist));
    }

  /* Delete elements after the beginning of Vselection_alist.  */
  rest = (*t).Vselection_alist;
  while CONSP (rest)
    {
      if CONSP (XCDR (rest))
	  && EQ (frame, XCAR (XCDR (XCDR (XCDR (XCAR (XCDR (rest)))))))
	{
	  symbol = XCAR (XCAR (XCDR (rest)));
	  CALLN! (Frun_hook_with_args, Qpgtk_lost_selection_functions (),
		  symbol);

	  timestamp = XCAR (XCDR (XCDR (XCAR (XCDR (rest)))));
	  /* C 833's `CONS_TO_INTEGER (timestamp, guint32, time)`.  */
	  time = cons_to_unsigned (timestamp, G_MAXUINT32 as uintmax_t) as guint32;

	  gdk_selection_owner_set_for_display ((*dpyinfo).gdpy,
					       std::ptr::null_mut (),
					       symbol_to_gdk_atom (symbol),
					       time, TRUE);

	  XSETCDR (rest, XCDR (XCDR (rest)));
	  break;
	}
      rest = XCDR (rest);
    }
}

/* C 844-845: the `waiting_for_other_props_on_window` comment.  */

/// C 847-854: `waiting_for_other_props_on_window`.
#[cfg (feature = "have_pgtk")]
unsafe fn waiting_for_other_props_on_window (display: *mut GdkDisplay,
                                             window: *mut GdkWindow) -> bool   // C 847-854
{
  let mut p: *mut prop_location = property_change_wait_list;
  while !p.is_null ()
    {
      if (*p).display == display && (*p).window == window
	{ return true; }
      p = (*p).next;
    }
  return false;
}

/* C 856-862: the `expect_property_change` comment and its "Currently unused -- uncomment later if
   we decide to implement INCR transfer for X." note, then C 864-882's `#if 0` block holding the
   whole definition.  C's own dead text in every configuration; recorded, not transcribed.  */

/// C 884-885: the `unexpect_property_change` comment.
#[cfg (feature = "have_pgtk")]
unsafe fn unexpect_property_change (location: *mut prop_location)      // C 887-903
{
  let mut prop: *mut prop_location;
  let mut pprev: *mut *mut prop_location = &raw mut property_change_wait_list;

  prop = property_change_wait_list;
  while !prop.is_null ()
    {
      if prop == location
	{
	  *pprev = (*prop).next;
	  xfree (prop as *mut c_void);
	  break;
	}
      else
	{ pprev = &raw mut (*prop).next; }
      prop = *pprev;
    }
}

/// C 905: "Remove the property change expectation element for IDENTIFIER."
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn wait_for_property_change_unwind (loc: *mut c_void)   // C 907-915
{
  let location: *mut prop_location = loc as *mut prop_location;

  unexpect_property_change (location);
  if location == property_change_reply_object
    { property_change_reply_object = std::ptr::null_mut (); }
}

/* C 917-918: the `wait_for_property_change` comment.  */

/// C 920-947: `wait_for_property_change`.
#[cfg (feature = "have_pgtk")]
unsafe fn wait_for_property_change (location: *mut prop_location)      // C 920-947
{
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* Make sure to do unexpect_property_change if we quit or err.  */
  record_unwind_protect_ptr (Some (wait_for_property_change_unwind), location as *mut c_void);

  /* See comment in x_reply_selection_request about setting
     property_change_reply.  Do not do it here.  */

  /* If the event we are waiting for arrives beyond here, it will set
     property_change_reply, because property_change_reply_object says so.  */
  if !(*location).arrived
    {
      let timeout: intmax_t = max! (0 as EMACS_INT, pgtk_selection_timeout) as intmax_t;
      let secs: intmax_t = timeout / 1000;
      let nsecs: c_int = ((timeout % 1000) * 1000000) as c_int;

      wait_reading_process_output (secs, nsecs, 0, false,
				   property_change_reply, std::ptr::null_mut (), 0);

      if NILP (XCAR (property_change_reply))
	{ error (c"Timed out waiting for property-notify event".as_ptr ()); }
    }

  unbind_to (count, Qnil ());
}

/* C 949-950: "Called from the big filter in response to a PropertyNotify event."  */

/// C 952-978: `pgtk_handle_property_notify`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_handle_property_notify (event: *mut GdkEventProperty)   // C 952-978
{
  let mut rest: *mut prop_location;
  let dpy: *mut GdkDisplay;

  dpy = gdk_window_get_display ((*event).window);

  rest = property_change_wait_list;
  while !rest.is_null ()
    {
      if !(*rest).arrived
	  && (*rest).property == (*event).atom
	  && (*rest).window == (*event).window
	  && (*rest).display == dpy
	  && (*rest).desired_state == (*event).state
	{
	  (*rest).arrived = true;

	  /* If this is the one wait_for_property_change is waiting for,
	     tell it to wake up.  */
	  if rest == property_change_reply_object
	    { XSETCAR (property_change_reply, Qt ()); }

	  return;
	}
      rest = (*rest).next;
    }
}

/// C 980-989: `pgtk_display_selection_waiting_message` (its address goes to `start_atimer`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_display_selection_waiting_message (timer: *mut atimer)   // C 980-989
{
  let val: Lisp_Object;
  let atimerp: *mut *mut atimer = (*timer).client_data as *mut *mut atimer;

  val = build_string (c"Waiting for reply from selection owner...".as_ptr ());
  message3_nolog (val);
  *atimerp = std::ptr::null_mut ();
}

/// C 991-1000: `pgtk_cancel_atimer` (its address goes to `record_unwind_protect_ptr`).
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pgtk_cancel_atimer (atimerp: *mut c_void)         // C 991-1000
{
  let atimer: *mut *mut atimer = atimerp as *mut *mut atimer;
  if !(*atimer).is_null ()
    {
      cancel_atimer (*atimer);
      *atimer = std::ptr::null_mut ();
    }
}

/* C 1003: "Variables for communication with pgtk_handle_selection_notify."  */
#[cfg (feature = "have_pgtk")]
static mut reading_which_selection: GdkAtom = std::ptr::null_mut ();   // C 1004
#[cfg (feature = "have_pgtk")]
static mut reading_selection_reply: Lisp_Object = lisp_h_Qnil ();      // C 1005
#[cfg (feature = "have_pgtk")]
static mut reading_selection_window: *mut GdkWindow = std::ptr::null_mut ();   // C 1006

/* C 1008-1010: the `pgtk_get_foreign_selection` comment.  */

/// C 1012-1078: `pgtk_get_foreign_selection`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_foreign_selection (selection_symbol: Lisp_Object, target_type: Lisp_Object,
                                      time_stamp: Lisp_Object, frame: Lisp_Object)
                                      -> Lisp_Object
{
  let f: *mut frame = XFRAME (frame);
  let dpyinfo: *mut pgtk_display_info = FRAME_DISPLAY_INFO! (f);
  let requestor_window: *mut GdkWindow = FRAME_GDK_WINDOW! (f);
  let mut requestor_time: guint32 = (*dpyinfo).last_user_time;
  let selection_atom: GdkAtom = symbol_to_gdk_atom (selection_symbol);
  let type_atom: GdkAtom = if CONSP (target_type)
    { symbol_to_gdk_atom (XCAR (target_type)) }
    else
    { symbol_to_gdk_atom (target_type) };
  let mut delayed_message: *mut atimer = std::ptr::null_mut ();   // 1.5: C leaves it unset
  let message_interval: timespec;
  let count: specpdl_ref;

  count = SPECPDL_INDEX ();

  if !FRAME_LIVE_P (f)
    { return unbind_to (count, Qnil ()); }

  if !NILP (time_stamp)
    /* C 1033's `CONS_TO_INTEGER (time_stamp, guint32, requestor_time)`.  */
    { requestor_time = cons_to_unsigned (time_stamp, G_MAXUINT32 as uintmax_t) as guint32; }

  block_input ();
  /* Prepare to block until the reply has been read.  */
  reading_selection_window = requestor_window;
  reading_which_selection = selection_atom;
  XSETCAR (reading_selection_reply, Qnil ());

  gdk_selection_convert (requestor_window, selection_atom,
			 type_atom, requestor_time);
  unblock_input ();

  /* It should not be necessary to stop handling selection requests
     during this time.  In fact, the SAVE_TARGETS mechanism requires
     us to handle a clipboard manager's requests before it returns
     GDK_SELECTION_NOTIFY. */

  message_interval = make_timespec (1, 0);
  delayed_message = start_atimer (ATIMER_RELATIVE, message_interval,
				  Some (pgtk_display_selection_waiting_message),
				  &raw mut delayed_message as *mut c_void);
  record_unwind_protect_ptr (Some (pgtk_cancel_atimer),
			     &raw mut delayed_message as *mut c_void);

  /* This allows quits.  Also, don't wait forever.  */
  let timeout: intmax_t = max! (0 as EMACS_INT, pgtk_selection_timeout) as intmax_t;
  let secs: intmax_t = timeout / 1000;
  let nsecs: c_int = ((timeout % 1000) * 1000000) as c_int;

  wait_reading_process_output (secs, nsecs, 0, false,
			       reading_selection_reply, std::ptr::null_mut (), 0);

  if NILP (XCAR (reading_selection_reply))
    { error (c"Timed out waiting for reply from selection owner".as_ptr ()); }
  if EQ (XCAR (reading_selection_reply), Qlambda ())
    { return unbind_to (count, Qnil ()); }

  /* Otherwise, the selection is waiting for us on the requested property.  */
  return unbind_to (count,
		    pgtk_get_window_property_as_lisp_data (dpyinfo,
							   requestor_window,
							   GDK_NONE,
							   target_type,
							   selection_atom,
							   false));
}

/* C 1080: "Subroutines of pgtk_get_window_property_as_lisp_data"  */

/// C 1082-1097: `pgtk_size_for_format`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_size_for_format (format: gint) -> ptrdiff_t               // C 1082-1097
{
  match format
    {
      8 => { return std::mem::size_of::<c_uchar> () as ptrdiff_t; }
      16 => { return std::mem::size_of::<c_ushort> () as ptrdiff_t; }
      32 => { return std::mem::size_of::<c_ulong> () as ptrdiff_t; }
      _ => { }
    }

  emacs_abort ();
}

/* C 1099: "Use xfree, not g_free, to free the data obtained with this function."  */

/// C 1101-1183: `pgtk_get_window_property`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_window_property (window: *mut GdkWindow, data_ret: *mut *mut c_uchar,
                                    bytes_ret: *mut ptrdiff_t, actual_type_ret: *mut GdkAtom,
                                    actual_format_ret: *mut c_int,
                                    actual_size_ret: *mut c_ulong)   // C 1101-1183
{
  let mut length: gint = 0;   // 1.5: C leaves it unset
  let mut actual_format: gint = 0;   // 1.5
  let mut data: *mut c_uchar;
  let element_size: ptrdiff_t;
  let mut xdata: *mut c_void;
  let mut actual_type: GdkAtom = std::ptr::null_mut ();   // 1.5: C leaves it unset
  let mut i: c_ulong;
  let mut idata: *mut c_uint;
  let mut ldata: *mut c_ulong;

  data = std::ptr::null_mut ();

  length = gdk_selection_property_get (window, &raw mut data,
				       &raw mut actual_type,
				       &raw mut actual_format);

  if data.is_null ()
    {
      *data_ret = std::ptr::null_mut ();
      *actual_type_ret = GDK_NONE;
      *bytes_ret = 0;
      *actual_format_ret = 8;
      *actual_size_ret = 0;

      return;
    }

  if actual_type == GDK_SELECTION_TYPE_ATOM
      || actual_type == gdk_atom_intern_static_string (c"ATOM_PAIR".as_ptr ())
    {
      /* GDK should not allow anything else.  */
      eassert! (actual_format == 32);

      length = (length as usize / std::mem::size_of::<GdkAtom> ()) as c_int;
      xdata = pgtk_nalloc (length as ptrdiff_t, std::mem::size_of::<GdkAtom> () as ptrdiff_t);
      memcpy (xdata, data as *const c_void,
	      (1 + length as usize * std::mem::size_of::<GdkAtom> ()) as usize);

      g_free (data as *mut c_void);

      *data_ret = xdata as *mut c_uchar;
      *actual_type_ret = actual_type;
      *bytes_ret = length as ptrdiff_t * std::mem::size_of::<GdkAtom> () as ptrdiff_t;
      *actual_format_ret = 32;
      *actual_size_ret = length as c_ulong;

      return;
    }

  element_size = pgtk_size_for_format (actual_format);
  length = (length as ptrdiff_t / element_size) as c_int;
  xdata = pgtk_nalloc (length as ptrdiff_t, element_size);
  memcpy (xdata, data as *const c_void,
	  (1 + element_size * length as ptrdiff_t) as usize);

  if actual_format == 32 && LONG_WIDTH > 32
    {
      ldata = data as *mut c_ulong;
      idata = xdata as *mut c_uint;

      i = 0;
      while i < length as c_ulong
	{
	  *idata.offset (i as isize) = *ldata.offset (i as isize) as c_uint;
	  i += 1;
	}

      /* There is always enough space in idata.  */
      *idata.offset (length as isize) = 0;
      *bytes_ret = std::mem::size_of::<c_uint> () as ptrdiff_t * length as ptrdiff_t;
    }
  else
    /* I think GDK itself prevents element_size from exceeding the
       length at which this computation fails.  */
    { *bytes_ret = element_size * length as ptrdiff_t; }

  /* Now free the original `data' allocated by GDK.  */
  g_free (data as *mut c_void);

  *data_ret = xdata as *mut c_uchar;
  *actual_type_ret = GDK_NONE;
  *actual_size_ret = length as c_ulong;
  *actual_format_ret = actual_format;
  *actual_type_ret = actual_type;
}

/// C 1185-1239: `pgtk_get_window_property_as_lisp_data`.
#[cfg (feature = "have_pgtk")]
unsafe fn pgtk_get_window_property_as_lisp_data (dpyinfo: *mut pgtk_display_info,
                                                 window: *mut GdkWindow, property: GdkAtom,
                                                 target_type: Lisp_Object,
                                                 selection_atom: GdkAtom,
                                                 for_multiple: bool) -> Lisp_Object   // C 1185-1239
{
  let mut actual_type: GdkAtom = std::ptr::null_mut ();   // 1.5: C leaves it unset
  let mut actual_format: c_int = 0;   // 1.5
  let mut actual_size: c_ulong = 0;   // 1.5
  let mut data: *mut c_uchar = std::ptr::null_mut ();
  let mut bytes: ptrdiff_t = 0;
  let val: Lisp_Object;
  let display: *mut GdkDisplay = (*dpyinfo).gdpy;

  pgtk_get_window_property (window, &raw mut data, &raw mut bytes,
			    &raw mut actual_type, &raw mut actual_format,
			    &raw mut actual_size);

  if data.is_null ()
    {
      if for_multiple
	{ return Qnil (); }

      if !gdk_selection_owner_get_for_display (display, selection_atom).is_null ()
	{
	  AUTO_STRING! (format, c"Selection owner couldn't convert: %s".as_ptr ());
	  CALLN! (Fmessage, format,
		  if !actual_type.is_null ()
		  { list2 (target_type, gdk_atom_to_symbol (actual_type)) }
		  else
		  { target_type });
	  return Qnil ();
	}
      else
	{
	  AUTO_STRING! (format, c"No selection: %s".as_ptr ());
	  CALLN! (Fmessage, format,
		  gdk_atom_to_symbol (selection_atom));
	  return Qnil ();
	}
    }

  if !for_multiple && property != GDK_NONE
    { gdk_property_delete (window, property); }

  /* It's been read.  Now convert it to a lisp object in some semi-rational
     manner.  */
  val = selection_data_to_lisp_data (dpyinfo, data, bytes,
				     actual_type, actual_format);

  /* Use xfree, not g_free, because pgtk_get_window_property calls
     xmalloc itself.  */
  xfree (data as *mut c_void);
  return val;
}

/* C 1243-1264: the conversion-table comment ("These functions convert from the selection data read
   from the server into something that we can use from Lisp, and vice versa." plus the type table
   and the "Important: When format is 32, data should contain an array of int, not an array of long
   as GDK returns" note).  */

/// C 1266-1386: `selection_data_to_lisp_data`.
#[cfg (feature = "have_pgtk")]
unsafe fn selection_data_to_lisp_data (dpyinfo: *mut pgtk_display_info, data: *const c_uchar,
                                       size: ptrdiff_t, r#type: GdkAtom, format: c_int)
                                       -> Lisp_Object
{
  if r#type == gdk_atom_intern_static_string (c"NULL".as_ptr ())
    { return QNULL (); }
  /* Convert any 8-bit data to a string, for compactness.  */
  else if format == 8
    {
      let str_: Lisp_Object;
      let lispy_type: Lisp_Object;

      str_ = make_unibyte_string (data as *const c_char, size);
      /* Indicate that this string is from foreign selection by a text
	 property `foreign-selection' so that the caller of
	 x-get-selection-internal (usually x-get-selection) can know
	 that the string must be decode.  */
      if r#type == gdk_atom_intern_static_string (c"COMPOUND_TEXT".as_ptr ())
	{ lispy_type = QCOMPOUND_TEXT (); }
      else if r#type == gdk_atom_intern_static_string (c"UTF8_STRING".as_ptr ())
	{ lispy_type = QUTF8_STRING (); }
      else
	{ lispy_type = QSTRING (); }

      Fput_text_property (make_fixnum (0), make_fixnum (size as EMACS_INT),
			  Qforeign_selection (), lispy_type, str_);
      return str_;
    }
  /* Convert a single atom to a Lisp_Symbol.  Convert a set of atoms to
     a vector of symbols.  */
  else if format == 32
	   && (r#type == GDK_SELECTION_TYPE_ATOM
	       /* Treat ATOM_PAIR type similar to list of atoms.  */
	       || r#type == gdk_atom_intern_static_string (c"ATOM_PAIR".as_ptr ()))
    {
      let mut i: ptrdiff_t;
      let idata: *const GdkAtom = data as *const GdkAtom;

      if size == std::mem::size_of::<GdkAtom> () as ptrdiff_t
	{ return gdk_atom_to_symbol (*idata); }
      else
	{
	  let v: Lisp_Object = make_nil_vector (size / std::mem::size_of::<GdkAtom> () as ptrdiff_t);

	  i = 0;
	  while i < size / std::mem::size_of::<GdkAtom> () as ptrdiff_t
	    {
	      ASET (v, i as isize, gdk_atom_to_symbol (*idata.offset (i as isize)));
	      i += 1;
	    }
	  return v;
	}
    }

  /* Convert a single 16-bit number or a small 32-bit number to a Lisp_Int.
     If the number is 32 bits and won't fit in a Lisp_Int, convert it
     to a bignum.

     INTEGER is a signed type, CARDINAL is unsigned.
     Assume any other types are unsigned as well.
   */
  else if format == 32 && size == std::mem::size_of::<c_int> () as ptrdiff_t
    {
      if r#type == GDK_SELECTION_TYPE_INTEGER
	{ return INT_TO_INTEGER (*(data as *const c_int) as EMACS_INT); }
      else
	{ return INT_TO_INTEGER (*(data as *const c_uint) as EMACS_INT); }
    }
  else if format == 16 && size == std::mem::size_of::<c_short> () as ptrdiff_t
    {
      if r#type == GDK_SELECTION_TYPE_INTEGER
	{ return make_fixnum (*(data as *const c_short) as EMACS_INT); }
      else
	{ return make_fixnum (*(data as *const c_ushort) as EMACS_INT); }
    }
  /* Convert any other kind of data to a vector of numbers, represented
     as above (as an integer, or a cons of two 16 bit integers.)
   */
  else if format == 16
    {
      let mut i: ptrdiff_t;
      let v: Lisp_Object = make_uninit_vector (size / 2);

      if r#type == GDK_SELECTION_TYPE_INTEGER
	{
	  i = 0;
	  while i < size / 2
	    {
	      let j: c_short = *(data as *const c_short).offset (i as isize);
	      ASET (v, i as isize, make_fixnum (j as EMACS_INT));
	      i += 1;
	    }
	}
      else
	{
	  i = 0;
	  while i < size / 2
	    {
	      let j: c_ushort = *(data as *const c_ushort).offset (i as isize);
	      ASET (v, i as isize, make_fixnum (j as EMACS_INT));
	      i += 1;
	    }
	}
      return v;
    }
  else
    {
      let mut i: ptrdiff_t;
      let v: Lisp_Object = make_nil_vector (size / std::mem::size_of::<gint> () as ptrdiff_t);

      if r#type == GDK_SELECTION_TYPE_INTEGER
	{
	  i = 0;
	  while i < size / std::mem::size_of::<gint> () as ptrdiff_t
	    {
	      let j: c_int = *(data as *const gint).offset (i as isize);
	      ASET (v, i as isize, INT_TO_INTEGER (j as EMACS_INT));
	      i += 1;
	    }
	}
      else
	{
	  i = 0;
	  while i < size / std::mem::size_of::<gint> () as ptrdiff_t
	    {
	      let j: c_uint = *(data as *const c_uint).offset (i as isize);
	      ASET (v, i as isize, INT_TO_INTEGER (j as EMACS_INT));
	      i += 1;
	    }
	}
      return v;
    }
}

/* C 1388-1393: the `cons_to_gdk_long` comment.  */

/// C 1394-1402: `cons_to_gdk_long`.
#[cfg (feature = "have_pgtk")]
unsafe fn cons_to_gdk_long (obj: Lisp_Object) -> c_ulong                  // C 1394-1402
{
  if G_MAXUINT32 as intmax_t <= INTMAX_MAX
      || NILP (Fnatnump (if CONSP (obj) { XCAR (obj) } else { obj }))
    { return cons_to_signed (obj, 0,
			     min! (G_MAXUINT32 as intmax_t, INTMAX_MAX)) as c_ulong; }
  else
    { return cons_to_unsigned (obj, G_MAXUINT32 as uintmax_t) as c_ulong; }
}

/* C 1404: "Use xfree, not XFree, to free the data obtained with this function."  */

/// C 1406-1544: `lisp_data_to_selection_data`.
#[cfg (feature = "have_pgtk")]
unsafe fn lisp_data_to_selection_data (dpyinfo: *mut pgtk_display_info, mut obj: Lisp_Object,
                                       cs: *mut selection_data)          // C 1406-1544
{
  let mut r#type: Lisp_Object = Qnil ();

  eassert! (!cs.is_null ());
  (*cs).nofree = false;

  if CONSP (obj) && SYMBOLP (XCAR (obj))
    {
      r#type = XCAR (obj);
      obj = XCDR (obj);
      if CONSP (obj) && NILP (XCDR (obj))
	{ obj = XCAR (obj); }
    }

  if EQ (obj, QNULL ()) || EQ (r#type, QNULL ())
    {				/* This is not the same as declining */
      (*cs).format = 32;
      (*cs).size = 0;
      (*cs).data = std::ptr::null_mut ();
      r#type = QNULL ();
    }
  else if STRINGP (obj)
    {
      if SCHARS (obj) < SBYTES (obj)
	/* OBJ is a multibyte string containing a non-ASCII char.  */
	{ signal_error (c"Non-ASCII string must be encoded in advance".as_ptr (), obj); }
      if NILP (r#type)
	{ r#type = QSTRING (); }
      (*cs).format = 8;
      (*cs).size = SBYTES (obj);
      (*cs).data = SDATA (obj);
      (*cs).nofree = true;
    }
  else if SYMBOLP (obj)
    {
      let data: *mut c_void = pgtk_nalloc (1, std::mem::size_of::<GdkAtom> () as ptrdiff_t);
      let x_atom_ptr: *mut GdkAtom = data as *mut GdkAtom;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 32;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<GdkAtom> () as isize) = 0;
      *x_atom_ptr = symbol_to_gdk_atom (obj);
      if NILP (r#type) { r#type = QATOM (); }
    }
  else if RANGED_FIXNUMP (SHRT_MIN as EMACS_INT, obj, SHRT_MAX as EMACS_INT)
    {
      let data: *mut c_void = pgtk_nalloc (1, std::mem::size_of::<c_short> () as ptrdiff_t);
      let short_ptr: *mut c_short = data as *mut c_short;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 16;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<c_short> () as isize) = 0;
      *short_ptr = XFIXNUM (obj) as c_short;
      if NILP (r#type) { r#type = QINTEGER (); }
    }
  else if INTEGERP (obj)
	   || (CONSP (obj) && INTEGERP (XCAR (obj))
	       && (FIXNUMP (XCDR (obj))
		   || (CONSP (XCDR (obj))
		       && FIXNUMP (XCAR (XCDR (obj))))))
    {
      let data: *mut c_void = pgtk_nalloc (1, std::mem::size_of::<c_ulong> () as ptrdiff_t);
      let x_long_ptr: *mut c_ulong = data as *mut c_ulong;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 32;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<c_ulong> () as isize) = 0;
      *x_long_ptr = cons_to_gdk_long (obj);
      if NILP (r#type) { r#type = QINTEGER (); }
    }
  else if VECTORP (obj)
    {
      /* Lisp_Vectors may represent a set of ATOMs;
	 a set of 16 or 32 bit INTEGERs;
	 or a set of ATOM_PAIRs (represented as [[A1 A2] [A3 A4] ...]
       */
      let mut i: ptrdiff_t;
      let size: ptrdiff_t = ASIZE (obj);

      if SYMBOLP (AREF (obj, 0))
	/* This vector is an ATOM set */
	{
	  let data: *mut c_void;
	  let x_atoms: *mut GdkAtom;
	  if NILP (r#type) { r#type = QATOM (); }
	  i = 0;
	  while i < size
	    {
	      if !SYMBOLP (AREF (obj, i))
		{ signal_error (c"All elements of selection vector must have same type".as_ptr (),
				obj); }
	      i += 1;
	    }

	  data = xnmalloc (size, std::mem::size_of::<GdkAtom> () as isize);
	  (*cs).data = data as *mut c_uchar;
	  x_atoms = data as *mut GdkAtom;
	  (*cs).format = 32;
	  (*cs).size = size;
	  i = 0;
	  while i < size
	    {
	      *x_atoms.offset (i as isize) = symbol_to_gdk_atom (AREF (obj, i));
	      i += 1;
	    }
	}
      else
	/* This vector is an INTEGER set, or something like it */
	{
	  let mut format: c_int = 16;
	  let mut data_size: c_int = std::mem::size_of::<c_short> () as c_int;
	  let data: *mut c_void;
	  let x_atoms: *mut c_ulong;
	  let shorts: *mut c_short;
	  if NILP (r#type) { r#type = QINTEGER (); }
	  i = 0;
	  while i < size
	    {
	      if !RANGED_FIXNUMP (SHRT_MIN as EMACS_INT, AREF (obj, i),
				  SHRT_MAX as EMACS_INT)
		{
		  /* Use sizeof (long) even if it is more than 32 bits.
		     See comment in x_get_window_property and
		     x_fill_property_data.  */
		  data_size = std::mem::size_of::<c_long> () as c_int;
		  format = 32;
		  break;
		}
	      i += 1;
	    }
	  data = xnmalloc (size, data_size as isize);
	  (*cs).data = data as *mut c_uchar;
	  x_atoms = data as *mut c_ulong;
	  shorts = data as *mut c_short;
	  (*cs).format = format;
	  (*cs).size = size;
	  i = 0;
	  while i < size
	    {
	      if format == 32
		{ *x_atoms.offset (i as isize) = cons_to_gdk_long (AREF (obj, i)); }
	      else
		{ *shorts.offset (i as isize) = XFIXNUM (AREF (obj, i)) as c_short; }
	      i += 1;
	    }
	}
    }
  else
    { signal_error (/* Qselection_error */ c"Unrecognized selection data".as_ptr (), obj); }

  (*cs).r#type = symbol_to_gdk_atom (r#type);
}

/// C 1546-1578: `clean_local_selection_data`.
#[cfg (feature = "have_pgtk")]
unsafe fn clean_local_selection_data (mut obj: Lisp_Object) -> Lisp_Object   // C 1546-1578
{
  if CONSP (obj)
      && INTEGERP (XCAR (obj))
      && CONSP (XCDR (obj))
      && FIXNUMP (XCAR (XCDR (obj)))
      && NILP (XCDR (XCDR (obj)))
    { obj = Fcons (XCAR (obj), XCDR (obj)); }

  if CONSP (obj)
      && INTEGERP (XCAR (obj))
      && FIXNUMP (XCDR (obj))
    {
      if BASE_EQ (XCAR (obj), make_fixnum (0))
	{ return XCDR (obj); }
      if BASE_EQ (XCAR (obj), make_fixnum (-1))
	{ return make_fixnum (-XFIXNUM (XCDR (obj))); }
    }
  if VECTORP (obj)
    {
      let mut i: ptrdiff_t;
      let size: ptrdiff_t = ASIZE (obj);
      let copy: Lisp_Object;
      if size == 1
	{ return clean_local_selection_data (AREF (obj, 0)); }
      copy = make_nil_vector (size);
      i = 0;
      while i < size
	{
	  ASET (copy, i as isize, clean_local_selection_data (AREF (obj, i as isize)));
	  i += 1;
	}
      return copy;
    }
  return obj;
}

/* C 1580-1604: `DEFUN ("pgtk-own-selection-internal", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-own-selection-internal", Fpgtk_own_selection_internal,
	Spgtk_own_selection_internal, 2, 3, 0,
	doc: "Assert a selection of type SELECTION and value VALUE.\nSELECTION is a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what GDK expects.)\nVALUE is typically a string, or a cons of two markers, but may be\nanything that the functions on `selection-converter-alist' know about.\n\nFRAME should be a frame that should own the selection.  If omitted or\nnil, it defaults to the selected frame.",
	(selection: Lisp_Object, value: Lisp_Object, frame: Lisp_Object),
	{
	  /* C 1598-1600 reassigns both `frame` and `selection`, so each parameter is shadowed by a
	     mutable local of the same name (the `src/xselect.rs` 3013 shape).  */
	  let mut frame = frame;
	  let mut selection = selection;
	  if NILP (frame) { frame = selected_frame; }
	  if !FRAME_LIVE_P (XFRAME (frame)) || !FRAME_PGTK_P (XFRAME (frame))
	    { error (c"GDK selection unavailable for this frame".as_ptr ()); }

	  CHECK_SYMBOL (selection);
	  if NILP (value) { error (c"VALUE may not be nil".as_ptr ()); }
	  pgtk_own_selection (selection, value, frame);
	  return value;
	});

/* C 1606-1676: `DEFUN ("pgtk-get-selection-internal", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-get-selection-internal", Fpgtk_get_selection_internal,
	Spgtk_get_selection_internal, 2, 4, 0,
	doc: "Return text selected from some X window.\nSELECTION-SYMBOL is typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nTARGET-TYPE is the type of data desired, typically `STRING'.\n\nTIME-STAMP is the time to use in the XConvertSelection call for foreign\nselections.  If omitted, defaults to the time for the last event.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.",
	(selection_symbol: Lisp_Object, target_type: Lisp_Object,
	 time_stamp: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection_symbol = selection_symbol;
	  let mut val: Lisp_Object = Qnil ();
	  let maybe_alias: Lisp_Object;
	  let f: *mut frame = frame_for_pgtk_selection (terminal);

	  CHECK_SYMBOL (selection_symbol);
	  CHECK_SYMBOL (target_type);

	  if EQ (target_type, QMULTIPLE ())
	    { error (c"Retrieving MULTIPLE selections is currently unimplemented".as_ptr ()); }
	  if f.is_null ()
	    { error (c"GDK selection unavailable for this frame".as_ptr ()); }

	  /* Quitting inside this function is okay, so we don't have to use
	     FOR_EACH_TAIL_SAFE.  */
	  maybe_alias = Fassq (selection_symbol, Vpgtk_selection_alias_alist);

	  if !NILP (maybe_alias)
	    {
	      selection_symbol = XCDR (maybe_alias);
	      CHECK_SYMBOL (selection_symbol);
	    }

	  val = pgtk_get_local_selection (selection_symbol, target_type, true,
					  FRAME_DISPLAY_INFO! (f));

	  if NILP (val) && FRAME_LIVE_P (f)
	    {
	      let mut frame: Lisp_Object;
	      let mut val: Lisp_Object;
	      XSETFRAME! (frame, f);

	      val = pgtk_get_foreign_selection (selection_symbol, target_type,
						time_stamp, frame);

	      /* A window property holding just one item is indistinguishable
		 from an array of one element, and is always decoded as the
		 former, producing issues with programs that expect the TARGETS
		 property always to return vectors, even when the toolkit
		 reports just one data type.  Though X sidesteps this ambiguity
		 by defining TARGETS as returning at least two properties
		 TARGETS and MULTIPLE, GTK knows no such scruples, and therefore
		 symbol values (or nil) should be enclosed in vectors when
		 TARGETS is being requested.  (bug#72254) */
	      if EQ (target_type, QTARGETS ()) && (NILP (val) || SYMBOLP (val))
		{ val = make_vector (if NILP (val) { 0 } else { 1 }, val); }
	      return val;
	    }

	  if CONSP (val) && SYMBOLP (XCAR (val))
	    {
	      val = XCDR (val);
	      if CONSP (val) && NILP (XCDR (val))
		{ val = XCAR (val); }
	    }
	  return clean_local_selection_data (val);
	});

/* C 1678-1719: `DEFUN ("pgtk-disown-selection-internal", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-disown-selection-internal", Fpgtk_disown_selection_internal,
	Spgtk_disown_selection_internal, 1, 3, 0,
	doc: "If we own the selection SELECTION, disown it.\nDisowning it means there is no such selection.\n\nSets the last-change time for the selection to TIME-OBJECT (by default\nthe time of the last event).\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.",
	(selection: Lisp_Object, time_object: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut timestamp: guint32 = 0;   // 1.5: C leaves it unset
	  let selection_atom: GdkAtom;
	  let f: *mut frame = frame_for_pgtk_selection (terminal);
	  let dpyinfo: *mut pgtk_display_info;

	  if f.is_null ()
	    { return Qnil (); }

	  dpyinfo = FRAME_DISPLAY_INFO! (f);
	  CHECK_SYMBOL (selection);

	  /* Don't disown the selection when we're not the owner.  */
	  if NILP (LOCAL_SELECTION! (selection, dpyinfo))
	    { return Qnil (); }

	  selection_atom = symbol_to_gdk_atom (selection);

	  block_input ();
	  if NILP (time_object)
	    { timestamp = (*dpyinfo).last_user_time; }
	  else
	    /* C 1711's `CONS_TO_INTEGER (time_object, guint32, timestamp)`.  */
	    { timestamp = cons_to_unsigned (time_object, G_MAXUINT32 as uintmax_t) as guint32; }
	  gdk_selection_owner_set_for_display ((*dpyinfo).gdpy,
					       std::ptr::null_mut (),
					       selection_atom, timestamp,
					       TRUE);
	  unblock_input ();

	  return Qt ();
	});

/* C 1721-1745: `DEFUN ("pgtk-selection-owner-p", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-selection-owner-p", Fpgtk_selection_owner_p, Spgtk_selection_owner_p,
	0, 2, 0,
	doc: "Whether the current Emacs process owns the given selection.\nThe arg should be the name of the selection in question, typically one of\nthe symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what GDK expects.)\nFor convenience, the symbol nil is the same as `PRIMARY',\nand t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the GDK\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.",
	(selection: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection = selection;
	  let f: *mut frame = frame_for_pgtk_selection (terminal);

	  CHECK_SYMBOL (selection);
	  if NILP (selection) { selection = QPRIMARY (); }
	  if EQ (selection, Qt ()) { selection = QSECONDARY (); }

	  if !f.is_null () && !NILP (LOCAL_SELECTION! (selection, FRAME_DISPLAY_INFO! (f)))
	    { return Qt (); }
	  else
	    { return Qnil (); }
	});

/* C 1747-1789: `DEFUN ("pgtk-selection-exists-p", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-selection-exists-p", Fpgtk_selection_exists_p, Spgtk_selection_exists_p,
	0, 2, 0,
	doc: "Whether there is an owner for the given selection.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', `CLIPBOARD', or\n`CLIPBOARD_MANAGER' (GDK expects these literal upper-case names.)  The\nsymbol nil is the same as `PRIMARY', and t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the GDK\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.",
	(selection: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection = selection;
	  let owner: *mut GdkWindow;
	  let atom: GdkAtom;
	  let f: *mut frame = frame_for_pgtk_selection (terminal);
	  let dpyinfo: *mut pgtk_display_info;

	  CHECK_SYMBOL (selection);
	  if NILP (selection) { selection = QPRIMARY (); }
	  if EQ (selection, Qt ()) { selection = QSECONDARY (); }

	  if f.is_null ()
	    { return Qnil (); }

	  dpyinfo = FRAME_DISPLAY_INFO! (f);

	  if !NILP (LOCAL_SELECTION! (selection, dpyinfo))
	    { return Qt (); }

	  atom = symbol_to_gdk_atom (selection);
	  if atom.is_null () { return Qnil (); }
	  block_input ();
	  owner = gdk_selection_owner_get_for_display ((*dpyinfo).gdpy, atom);
	  unblock_input ();
	  return if !owner.is_null () { Qt () } else { Qnil () };
	});

/* C 1785-1789: the `pgtk_handle_selection_notify` comment.  */

/// C 1790-1801: `pgtk_handle_selection_notify`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_handle_selection_notify (event: *mut GdkEventSelection)   // C 1790-1801
{
  /* GDK doesn't populate event->requestor, contrary to what the ICCCM
     says should be done with SelectionNotify events.  */

  if (*event).selection != reading_which_selection
    { return; }

  XSETCAR (reading_selection_reply,
	   if (*event).property != GDK_NONE { Qt () } else { Qlambda () });
}

/* C 1804-1806: the "Drag and drop support" banner.  */

/* C 1808-1855: `DEFUN ("pgtk-register-dnd-targets", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-register-dnd-targets", Fpgtk_register_dnd_targets,
	Spgtk_register_dnd_targets, 2, 2, 0,
	doc: "Register TARGETS on FRAME.\nTARGETS should be a list of strings describing data types (selection\ntargets) that can be dropped on top of FRAME.",
	(frame: Lisp_Object, targets: Lisp_Object),
	{
	  let f: *mut frame;
	  let mut entries: *mut GtkTargetEntry = std::ptr::null_mut ();
	  let list: *mut GtkTargetList;
	  let mut length: ptrdiff_t;
	  let mut n: ptrdiff_t;
	  let mut tem: Lisp_Object;
	  let mut t: Lisp_Object;
	  let mut buf: *mut c_char = std::ptr::null_mut ();
	  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

	  f = decode_window_system_frame (frame);
	  CHECK_LIST (targets);
	  length = list_length (targets);
	  n = 0;
	  SAFE_NALLOCA! (sa_avail, entries, 1, length);
	  memset (entries as *mut c_void, 0,
		  std::mem::size_of::<GtkTargetEntry> () * length as usize);
	  tem = targets;

	  FOR_EACH_TAIL! (tem,
	    {
	      'body: {
		if !CONSP (tem)
		  { break 'body; }   /* C 1843: `continue;` (the FOR_EACH_TAIL stepper follows
					the body, so C's jump to the step is this exit).  */

		t = XCAR (tem);

		CHECK_STRING (t);
		SAFE_ALLOCA_STRING! (sa_avail, buf, t);

		(*entries.offset (n as isize)).target = buf;
		n += 1;
	      }
	    });
	  CHECK_LIST_END (tem, targets);

	  if n != length
	    { emacs_abort (); }

	  list = gtk_target_list_new (entries, n as guint);
	  gtk_drag_dest_set_target_list (FRAME_GTK_WIDGET! (f), list);
	  gtk_target_list_unref (list);

	  SAFE_FREE! (sa_count, sa_mark);

	  return Qnil ();
	});

/* C 1857-1869: `DEFUN ("pgtk-drop-finish", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-drop-finish", Fpgtk_drop_finish, Spgtk_drop_finish, 3, 3, 0,
	doc: "Finish the drag-n-drop event that happened at TIMESTAMP.\nSUCCESS is whether or not the drop was successful, i.e. the action\nchosen in the last call to `pgtk-update-drop-status' was performed.\nTIMESTAMP is the time associated with the drag-n-drop event that is\nbeing finished.\nDELETE is whether or not the action was `move'.",
	(success: Lisp_Object, timestamp: Lisp_Object, delete: Lisp_Object),
	{
	  pgtk_finish_drop (success, timestamp, delete);

	  return Qnil ();
	});

/* C 1871-1881: `DEFUN ("pgtk-update-drop-status", …)`.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("pgtk-update-drop-status", Fpgtk_update_drop_status,
	Spgtk_update_drop_status, 2, 2, 0,
	doc: "Update the status of the current drag-and-drop operation.\nACTION is the action the drop source should take.\nTIMESTAMP is the same as in `pgtk-drop-finish'.",
	(action: Lisp_Object, timestamp: Lisp_Object),
	{
	  pgtk_update_drop_status (action, timestamp);

	  return Qnil ();
	});

/* C 1906-1913: the eight DEFUNs' `S*` records are emitted by the `DEFUN!` macro above (PORTING.md
   0.1-3), and C 1955-1965's `DEFVAR_INT`, C 1915/1919/1927/1941/1961's `DEFVAR_LISP` variables are
   defined at this unit's file scope so it is their single definition point (P-21: the mirror's
   `globals.rs` `variables` module declares them under a glob import, which a definition shadows).  */
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut Vselection_converter_alist: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut Vpgtk_lost_selection_functions: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut Vpgtk_sent_selection_functions: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut Vpgtk_sent_selection_hooks: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut pgtk_selection_timeout: EMACS_INT = 0;
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)] pub static mut Vpgtk_selection_alias_alist: Lisp_Object = lisp_h_Qnil ();

/// C 1883-1975: `syms_of_pgtkselect`.
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_pgtkselect ()                          // C 1883-1975
{
  DEFSYM! (QCLIPBOARD, c"CLIPBOARD");
  DEFSYM! (QSECONDARY, c"SECONDARY");
  DEFSYM! (QTEXT, c"TEXT");
  DEFSYM! (QFILE_NAME, c"FILE_NAME");
  DEFSYM! (QSTRING, c"STRING");
  DEFSYM! (QINTEGER, c"INTEGER");
  DEFSYM! (QTIMESTAMP, c"TIMESTAMP");
  DEFSYM! (QTEXT, c"TEXT");
  DEFSYM! (QMULTIPLE, c"MULTIPLE");
  DEFSYM! (QNULL, c"NULL");
  DEFSYM! (QATOM, c"ATOM");
  DEFSYM! (QTARGETS, c"TARGETS");
  DEFSYM! (QUTF8_STRING, c"UTF8_STRING");
  DEFSYM! (QCOMPOUND_TEXT, c"COMPOUND_TEXT");

  DEFSYM! (Qforeign_selection, c"foreign-selection");

  DEFSYM! (Qpgtk_sent_selection_functions, c"pgtk-sent-selection-functions");
  DEFSYM! (Qpgtk_lost_selection_functions, c"pgtk-lost-selection-functions");

  defsubr (&raw mut Spgtk_disown_selection_internal);
  defsubr (&raw mut Spgtk_get_selection_internal);
  defsubr (&raw mut Spgtk_own_selection_internal);
  defsubr (&raw mut Spgtk_selection_exists_p);
  defsubr (&raw mut Spgtk_selection_owner_p);
  defsubr (&raw mut Spgtk_register_dnd_targets);
  defsubr (&raw mut Spgtk_update_drop_status);
  defsubr (&raw mut Spgtk_drop_finish);

  DEFVAR_LISP! (c"selection-converter-alist", Vselection_converter_alist,
		"SKIP: real doc in xselect.c.");
  Vselection_converter_alist = Qnil ();

  DEFVAR_LISP! (c"pgtk-lost-selection-functions", Vpgtk_lost_selection_functions,
		"A list of functions to be called when Emacs loses a selection.\n(This happens when some other client makes its own selection\nor when a Lisp program explicitly clears the selection.)\nThe functions are called with one argument, the selection type\n(a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD').");
  Vpgtk_lost_selection_functions = Qnil ();

  DEFVAR_LISP! (c"pgtk-sent-selection-functions", Vpgtk_sent_selection_functions,
		"A list of functions to be called when Emacs answers a selection request.\nThe functions are called with three arguments:\n  - the selection name (typically `PRIMARY', `SECONDARY', or `CLIPBOARD');\n  - the selection-type which Emacs was asked to convert the\n    selection into before sending (for example, `STRING' or `LENGTH');\n  - a flag indicating success or failure for responding to the request.\nWe might have failed (and declined the request) for any number of reasons,\nincluding being asked for a selection that we no longer own, or being asked\nto convert into a type that we don't know about or that is inappropriate.\nxThis hook doesn't let you change the behavior of Emacs's selection replies,\nit merely informs you that they have happened.");
  Vpgtk_sent_selection_functions = Qnil ();

  DEFVAR_LISP! (c"pgtk-sent-selection-hooks", Vpgtk_sent_selection_hooks,
		"A list of functions to be called when Emacs answers a selection request\nThe functions are called with four arguments:\n  - the selection name (typically `PRIMARY', `SECONDARY', or `CLIPBOARD');\n  - the selection-type which Emacs was asked to convert the\n    selection into before sending (for example, `STRING' or `LENGTH');\n  - a flag indicating success or failure for responding to the request.\nWe might have failed (and declined the request) for any number of reasons,\nincluding being asked for a selection that we no longer own, or being asked\nto convert into a type that we don't know about or that is inappropriate.\nThis hook doesn't let you change the behavior of Emacs's selection replies,\nit merely informs you that they have happened.");
  Vpgtk_sent_selection_hooks = Qnil ();

  DEFVAR_INT! (c"pgtk-selection-timeout", pgtk_selection_timeout,
	       "Number of milliseconds to wait for a selection reply.\nIf the selection owner doesn't reply in this time, we give up.\nA value of 0 means wait as long as necessary.");
  pgtk_selection_timeout = 0;

  DEFVAR_LISP! (c"pgtk-selection-alias-alist", Vpgtk_selection_alias_alist,
		"List of selections to alias to another.\nIt should be an alist of a selection name to another.  When a\nselection request arrives for the first selection, Emacs will respond\nas if the request was meant for the other.\n\nNote that this does not affect setting or owning selections.");
  Vpgtk_selection_alias_alist = Qnil ();

  reading_selection_reply = Fcons (Qnil (), Qnil ());
  staticpro (&raw const reading_selection_reply);

  property_change_reply = Fcons (Qnil (), Qnil ());
  staticpro (&raw const property_change_reply);
}
