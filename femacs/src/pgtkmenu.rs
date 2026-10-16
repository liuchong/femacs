// femacs: port of src/pgtkmenu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md
// 0.2); the C `static` helpers stay plain `unsafe fn`s except the ones whose address C hands to
// another function (`popup_deactivate_callback`, `menu_highlight_callback`,
// `menubar_selection_callback`, `popup_selection_callback`, `pop_down_menu`,
// `cleanup_widget_value_tree`, `dialog_selection_callback`, `syms_of_pgtkmenu_for_pdumper`), which
// are spelled `unsafe extern "C" fn` (PORTING.md 1.4/P-12: one definition per linkage unit, which is
// what C's `static` gives too).
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `pgtkmenu.o`
// sits in `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`; `configure.ac` 3875-3881
// fills it - `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` - in the
// branch `window_system = pgtk`, whose `AC_DEFINE` is `configure.ac` 3176's `HAVE_PGTK`), and
// `docs/baseline/frozen-build-sources.txt` (106 source lines) lists no pgtk object: the file is not
// in this target's source list at all.  P-86 ① therefore applies - this is *transcription* - and
// the **file gate** is `feature = "have_pgtk"`, on every item **and on the include graph** (P-86 ⑧:
// an `include!` outside the gate breaks the frozen build at once, which is exactly why
// `build-tu src/pgtkmenu.c` reads a 336-byte empty object - see the ledger's Evidence ①).
//
// The file's own conditions are exactly three `#ifndef HAVE_MULTILINGUAL_MENU` blocks (C 655-661,
// C 704-716, C 758-761), carried as `not (feature = "have_multilingual_menu")`; nothing in the
// reference tree defines that macro, so the guarded arm is the live text in every configuration and
// the `#else` arm is empty (both are carried, the `#else` as its own nothing).
// `python3 tools/check-cond-eval.py src/pgtkmenu.c` reads `3 conditions, 0 unknown, 0
// disagreements [VERIFIED]` and the file is on `docs/baseline/config/cond-eval-verified.txt` 344.
//
// **The GTK/GDK surface's single home is `femacs/src/include/gtk.rs`** (P-90 ①②).  C 36's
// `"gtkutil.h"` reaches `<gtk/gtk.h>` at gtkutil.h 26 and `"pgtkterm.h"` at its 29 (under
// `HAVE_PGTK`), and this file's C 37 is its own `#include <gtk/gtk.h>`; the mirror edge stands at
// that position below.  What the mirrors do not carry stands in-unit at C's `#include` positions
// (PORTING.md P-88 ①/P-72 rule 4) and its move is owed (P-90 ③; `## Owed` of the ledger): the eight
// `"gtk/gtk.h"` names of the toolkit half (C 82/86/198-199/547-560), `"gtkutil.h"` 49/52-56/63-72/
// 75-84/96-110/112 (`XG_ITEM_DATA`, `xg_list_node`, `xg_menu_cb_data`, `xg_menu_item_cb_data`,
// `xg_create_widget`, `xg_modify_menubar_widgets`, `xg_update_frame_menubar`) and `"menu.h"` 38-45's
// `HAVE_EXT_MENU_BAR` block (live in a pgtk build, lisp.h 3930-3934, and absent from
// `include/menu.rs`'s frozen reading).
//
// **Acquisition: this piece is `text-only` (P-86 ③⑵).**  Measured on this machine: `ls -d
// /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` -> both `No such file or directory`;
// `pkg-config` is not installed; no `gtk/gtk.h` or `gdk/gdk.h` under `/opt/homebrew`, `/usr/local`
// or `/Library`.  There is no GTK/GDK tree to compile the arm against, so every GTK/GDK citation
// below names the header alone (no line), every one of its values is a placeholder, and no layout
// assertion is written for a GTK/GDK record (the `include/gtk.rs` 20-30 rule).  **GLib and cairo are
// present** (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0).  The records this file names from
// `"gtkutil.h"` are reference-tree declarations, transcribed from their C text (their layout cannot
// be measured without a GTK tree either - `gtkutil.h` includes `<gtk/gtk.h>`).
//
// `docs/ports/pgtkmenu.md` carries the C line-range table, the acceptance readings, every deviation
// and the items the integrator owes.  No behaviour of this file was executed anywhere: there is no
// GTK tree to compile the arm against and no PGTK target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 19: `#include <config.h>` (every unit includes it)

/* C 21-37: the file's own include list.  No translation unit the frozen configuration builds
   reaches any of it, so the whole graph sits under the file gate (P-86 ⑧).  C's lines stand as
   these comments where the mirror *is* what C would have included.  */
#[cfg (feature = "have_pgtk")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (memcpy, strcmp)
#[cfg (feature = "have_pgtk")]
include!("../lib/include/limits.in.rs");       // lisp.h 33: `#include <limits.h>`
#[cfg (feature = "have_pgtk")]
include!("include/lisp.rs");                   // C 21: `#include "lisp.h"`
#[cfg (feature = "have_pgtk")]
include!("include/frame.rs");                  // C 22: `#include "frame.h"`
#[cfg (feature = "have_pgtk")]
include!("include/window.rs");                 // C 23: `#include "window.h"`
#[cfg (feature = "have_pgtk")]
include!("include/character.rs");              // C 24: `#include "character.h"`
#[cfg (feature = "have_pgtk")]
include!("include/buffer.rs");                 // C 25: `#include "buffer.h"`
#[cfg (feature = "have_pgtk")]
include!("include/keymap.rs");                 // C 26: `#include "keymap.h"`
#[cfg (feature = "have_pgtk")]
include!("include/coding.rs");                 // C 27: `#include "coding.h"`
#[cfg (feature = "have_pgtk")]
include!("include/commands.rs");               // C 28: `#include "commands.h"`
#[cfg (feature = "have_pgtk")]
include!("include/blockinput.rs");             // C 29: `#include "blockinput.h"` (block_input)
#[cfg (feature = "have_pgtk")]
include!("include/termhooks.rs");              // C 30: `#include "termhooks.h"`
#[cfg (feature = "have_pgtk")]
include!("include/keyboard.rs");               // C 31: `#include "keyboard.h"`
#[cfg (feature = "have_pgtk")]
include!("include/menu.rs");                   // C 32: `#include "menu.h"`
#[cfg (feature = "have_pgtk")]
include!("../lwlib/include/lwlib-widget.rs");  // menu.h 22: `#include "../lwlib/lwlib-widget.h"` (widget_value)
#[cfg (feature = "have_pgtk")]
include!("include/pdumper.rs");                // C 33: `#include "pdumper.h"`
#[cfg (feature = "have_pgtk")]
include!("include/xgselect.rs");               // C 34: `#include "xgselect.h"`
#[cfg (feature = "have_pgtk")]
include!("include/gtkutil.rs");                // C 36: `#include "gtkutil.h"`
/* C 36's chain, second half: `"gtkutil.h"` 26's `#include <gtk/gtk.h>` - the surface's single home
   is `include/gtk.rs` and this edge stands at the C position that makes the names visible (the
   `src/pgtkfns.rs` C 28 shape); it drops when `include/gtkutil.rs`'s own C 26 edge lands (P-88 ⑤). */
#[cfg (feature = "have_pgtk")]
include!("include/gtk.rs");                    // gtkutil.h 26: `#include <gtk/gtk.h>`
/* C 36's chain, third part: `"gtkutil.h"` 28-30's `#ifdef HAVE_PGTK` / `#include "pgtkterm.h"` -
   `FRAME_X_OUTPUT`, `FRAME_DISPLAY_INFO`, `FRAME_GTK_WIDGET` and `pgtk_set_z_group` arrive here. */
#[cfg (feature = "have_pgtk")]
include!("include/pgtkterm.rs");               // gtkutil.h 29: `#include "pgtkterm.h"`
/* C 37: `#include <gtk/gtk.h>`, the same edge as C 36's chain's (already included, so this is the
   reference's own second reach at it; `[xtask dedupe]` would collapse it).  */

/* ----------------------------------------------------------------------------------------------
   C 32's chain: `"menu.h"` 38-45's `#ifdef HAVE_EXT_MENU_BAR` block.  A pgtk build defines
   `HAVE_EXT_MENU_BAR` (lisp.h 3930-3934, through `USE_GTK`), so its five declarations are live
   there, and `include/menu.rs` - the frozen configuration's reading of the header - does not carry
   them.  They stand here at C's `#include` position (PORTING.md P-88 ①/P-72 rule 4) and their move
   into the mirror is owed (`## Owed` of `docs/ports/pgtkmenu.md`).
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: src/menu.h 43: `extern widget_value *make_widget_value (const char *, char *, bool,
  /// Lisp_Object);` - C 335/401/414/616/673/718/752/753/763/925/960/980.
  pub fn make_widget_value (name: *const c_char, value: *mut c_char, enabled: bool,
                            help: Lisp_Object) -> *mut widget_value;
  /// C: src/menu.h 39: `extern void free_menubar_widget_value_tree (widget_value *);` - C
  /// 368/461/584/941/955.
  pub fn free_menubar_widget_value_tree (wv: *mut widget_value);
  /// C: src/menu.h 40: `extern void update_submenu_strings (widget_value *);` - C 391.
  pub fn update_submenu_strings (wv: *mut widget_value);
  /// C: src/menu.h 44: `extern widget_value *digest_single_submenu (int, int, bool);` - C 342.
  pub fn digest_single_submenu (start: c_int, end: c_int,
                                top_level_items: bool) -> *mut widget_value;
  /// C: src/menu.h 41-42: `extern void find_and_call_menu_selection (struct frame *, int,
  /// Lisp_Object, void *);` - C 213-216.
  pub fn find_and_call_menu_selection (f: *mut frame, items: c_int, vector: Lisp_Object,
                                       client_data: *mut c_void);
}

/* ----------------------------------------------------------------------------------------------
   C 36's chain: the `"gtkutil.h"` names the mirror `include/gtkutil.rs` does not carry, at C's
   `#include` position (PORTING.md P-88 ①/P-72 rule 4).  The declarations are the reference
   header's own text; their layout cannot be measured here (the header reaches `<gtk/gtk.h>`), so
   each record is transcribed and its move into the mirror is owed.
   ---------------------------------------------------------------------------------------------- */
/// C: src/gtkutil.h 49: `#define XG_ITEM_DATA "emacs_menuitem"` - C 156's `g_object_get_data`.
/// **UNVERIFIED** as to the mirror's ownership (the move above); the value is C's own text.
#[cfg (feature = "have_pgtk")]
pub const XG_ITEM_DATA: *const c_char = c"emacs_menuitem".as_ptr ();
/// C: src/gtkutil.h 52-56: `typedef struct xg_list_node_ { struct xg_list_node_ *prev; struct
/// xg_list_node_ *next; } xg_list_node;` - the head member of the two records below.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct xg_list_node
{
  pub prev: *mut xg_list_node,                       // gtkutil.h 54: `struct xg_list_node_ *prev;`
  pub next: *mut xg_list_node,                       // gtkutil.h 55: `struct xg_list_node_ *next;`
}
/// C: src/gtkutil.h 63-72: `typedef struct xg_menu_cb_data_ { xg_list_node ptrs; struct frame *f;
/// Lisp_Object menu_bar_vector; int menu_bar_items_used; GCallback highlight_cb; int ref_count;
/// } xg_menu_cb_data;` - C 168/190/213-215 read `f`, `menu_bar_vector`, `menu_bar_items_used`.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct xg_menu_cb_data
{
  pub ptrs: xg_list_node,                            // gtkutil.h 65: `xg_list_node  ptrs;`
  pub f: *mut frame,                                 // gtkutil.h 67: `struct frame  *f;`
  pub menu_bar_vector: Lisp_Object,                  // gtkutil.h 68: `Lisp_Object   menu_bar_vector;`
  pub menu_bar_items_used: c_int,                    // gtkutil.h 69: `int           menu_bar_items_used;`
  pub highlight_cb: GCallback,                       // gtkutil.h 70: `GCallback     highlight_cb;`
  pub ref_count: c_int,                              // gtkutil.h 71: `int           ref_count;`
}
/// C: src/gtkutil.h 75-84: `typedef struct xg_menu_item_cb_data_ { xg_list_node ptrs; gulong
/// select_id; Lisp_Object help; gpointer call_data; xg_menu_cb_data *cl_data; }
/// xg_menu_item_cb_data;` - C 153/185/156-160/507/512 read `help`, `call_data`, `cl_data`.
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct xg_menu_item_cb_data
{
  pub ptrs: xg_list_node,                            // gtkutil.h 77: `xg_list_node  ptrs;`
  pub select_id: c_ulong,                            // gtkutil.h 79: `gulong        select_id;`
  pub help: Lisp_Object,                             // gtkutil.h 80: `Lisp_Object   help;`
  pub call_data: gpointer,                           // gtkutil.h 81: `gpointer      call_data;`
  pub cl_data: *mut xg_menu_cb_data,                 // gtkutil.h 82: `xg_menu_cb_data *cl_data;`
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: src/gtkutil.h 96-102: `extern GtkWidget *xg_create_widget (const char *type, const char
  /// *name, struct frame *f, struct _widget_value *val, GCallback select_cb, GCallback
  /// deactivate_cb, GCallback highlight_cb);` - C 453/537/868.
  pub fn xg_create_widget (r#type: *const c_char, name: *const c_char, f: *mut frame,
                           val: *mut widget_value, select_cb: GCallback, deactivate_cb: GCallback,
                           highlight_cb: GCallback) -> *mut GtkWidget;
  /// C: src/gtkutil.h 104-110: `extern void xg_modify_menubar_widgets (GtkWidget *menubar, struct
  /// frame *f, struct _widget_value *val, bool deep_p, GCallback select_cb, GCallback
  /// deactivate_cb, GCallback highlight_cb);` - C 442.
  pub fn xg_modify_menubar_widgets (menubar: *mut GtkWidget, f: *mut frame,
                                    val: *mut widget_value, deep_p: bool, select_cb: GCallback,
                                    deactivate_cb: GCallback, highlight_cb: GCallback);
  /// C: src/gtkutil.h 112: `extern void xg_update_frame_menubar (struct frame *f);` - C 225.
  pub fn xg_update_frame_menubar (f: *mut frame);
}

/* ----------------------------------------------------------------------------------------------
   C 37: `#include <gtk/gtk.h>`.  The eight names below are GTK's, not the mirror's, and
   `include/gtk.rs` does not carry them; they stand here at C's `#include` position (PORTING.md
   P-88 ①) and their move into the mirror is owed (P-90 ③).  **No GTK header exists on this
   machine** (see the head note), so the citations name the header alone and every declaration is
   UNVERIFIED (P-88 ④).
   ---------------------------------------------------------------------------------------------- */
/// C: gtk/gtkmenuitem.h: `typedef struct _GtkCheckMenuItem GtkCheckMenuItem;` - the type of
/// `GTK_CHECK_MENU_ITEM`'s result (C 199).  **UNVERIFIED** (no GTK header here).
#[cfg (feature = "have_pgtk")]
#[repr (C)]
pub struct GtkCheckMenuItem
{
  _private: [u8; 0],
}
/// C: gtk/gtkcheckmenuitem.h: `#define GTK_CHECK_MENU_ITEM(obj) (G_TYPE_CHECK_INSTANCE_CAST
/// ((obj), GTK_TYPE_CHECK_MENU_ITEM, GtkCheckMenuItem))` - C 199's cast.  The type check itself is
/// not measurable here, so the port keeps GTK's name, its argument position and its
/// `GtkCheckMenuItem *` result and spells the cast alone (the `include/gtk.rs` 640-682 family).
/// **UNVERIFIED**.
#[cfg (feature = "have_pgtk")]
macro_rules! GTK_CHECK_MENU_ITEM {
  ($o:expr) => { ($o) as *mut GtkCheckMenuItem };
}
/// C: gtk/gtkcheckmenuitem.h: `#define GTK_IS_RADIO_MENU_ITEM(obj) (G_TYPE_CHECK_INSTANCE_TYPE
/// ((obj), GTK_TYPE_RADIO_MENU_ITEM))` - C 198's test.  The port spells it as the
/// `include/gtk.rs` 668-682 type-check family does (`! (obj).is_null ()`); the real type test is
/// **UNVERIFIED** here.
#[cfg (feature = "have_pgtk")]
macro_rules! GTK_IS_RADIO_MENU_ITEM {
  ($o:expr) => { (! ($o).is_null ()) };
}
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: gtk/gtkcheckmenuitem.h: `gboolean gtk_check_menu_item_get_active (GtkCheckMenuItem
  /// *check_menu_item);` - C 199.
  pub fn gtk_check_menu_item_get_active (check_menu_item: *mut GtkCheckMenuItem) -> gboolean;
  /// C: gtk/gtkwidget.h: `gboolean gtk_widget_get_mapped (GtkWidget *widget);` - C 565.
  pub fn gtk_widget_get_mapped (widget: *mut GtkWidget) -> gboolean;
  /// C: gobject/gsignal.h: `void g_signal_emit_by_name (gpointer instance, const gchar
  /// *detailed_signal, ...);` - C 86.
  pub fn g_signal_emit_by_name (instance: gpointer, signal: *const c_char, ...);
  /// C: gtk/gtkmenu.h: `void gtk_menu_popup_at_pointer (GtkMenu *menu, const GdkEvent
  /// *trigger_event);` - C 547.
  pub fn gtk_menu_popup_at_pointer (menu: *mut GtkMenu, trigger_event: *const GdkEvent);
  /// C: gtk/gtkmenu.h: `void gtk_menu_popup_at_rect (GtkMenu *menu, GdkWindow *rect_window, const
  /// GdkRectangle *rect, GdkGravity rect_anchor, GdkGravity menu_anchor, const GdkEvent
  /// *trigger_event);` - C 556-560.
  pub fn gtk_menu_popup_at_rect (menu: *mut GtkMenu, rect_window: *mut GdkWindow,
                                 rect: *const GdkRectangle, rect_anchor: GdkGravity,
                                 menu_anchor: GdkGravity, trigger_event: *const GdkEvent);
}

/* ----------------------------------------------------------------------------------------------
   The reference-tree declarations the mirrors above do not carry, each at its C header's line
   (P-72 rule 4: a mirror reaches what it needs through its own `include!`s; P-39: another unit
   defines these, so they are declared, never defined).  Their move into the mirrors is owed.
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "have_pgtk")]
unsafe extern "C" {
  /// C: src/lisp.h 4786: `extern Lisp_Object quit (void);` - C 839/1043.
  pub fn quit () -> Lisp_Object;
  /// C: src/lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));` - C 1061.
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: src/lisp.h 4998: `extern void record_unwind_save_match_data (void);` - C 272.
  pub fn record_unwind_save_match_data ();
  /// C: src/lisp.h 5087: `extern Lisp_Object menu_bar_items (Lisp_Object);` - C 287/479.
  pub fn menu_bar_items (menu_bar_items: Lisp_Object) -> Lisp_Object;
  /// C: src/lisp.h 5097: `extern void safe_run_hooks (Lisp_Object);` - C 282/286.
  pub fn safe_run_hooks (hook: Lisp_Object);
  /// C: src/keyboard.h 385: `extern void unuse_menu_items (void);` - C 1061.
  pub fn unuse_menu_items ();
  /// C: src/frame.h 1475: `INLINE struct frame * decode_window_system_frame (Lisp_Object);` - C 73.
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: src/frame.h 1504: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);`
  /// - C 1056.
  pub fn check_window_system (f: *mut frame);
}

/* ==============================================================================================
   C 39-116.  */

/* Flag which when set indicates a dialog or menu has been posted by
   GTK on behalf of one of the widget sets.  */                    // C 39-40
#[cfg (feature = "have_pgtk")]
static mut popup_activated_flag: c_int = 0;                         // C 41 (`static int`, tentative)

/* Set menu_items_inuse so no other popup menu or dialog is created.  */   // C 43

/// C: void pgtk_menu_set_in_use (bool in_use)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_menu_set_in_use (in_use: bool)       // C 45-63
{
  let mut frames: Lisp_Object;                                      // C 48
  let mut frame: Lisp_Object;

  menu_items_inuse = in_use;                                        // C 50
  popup_activated_flag = in_use as c_int;                           // C 51

  /* Don't let frames in `above' z-group obscure popups.  */
  FOR_EACH_FRAME! (frames, frame, {                                 // C 54-62
      let f: *mut frame = XFRAME (frame);                           // C 56

      if in_use && FRAME_Z_GROUP_ABOVE (f)                          // C 58
	{ pgtk_set_z_group (f, Qabove_suspended (), Qabove ()); }
      else if !in_use && FRAME_Z_GROUP_ABOVE_SUSPENDED (f)          // C 60
	{ pgtk_set_z_group (f, Qabove (), Qabove_suspended ()); }
    });
}

/* C 65-66: `DEFUN ("x-menu-bar-open-internal", …)` - the `doc` field is C's `SKIP:` text.  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("x-menu-bar-open-internal", Fx_menu_bar_open_internal,
	Sx_menu_bar_open_internal, 0, 1, "i",
	doc: "SKIP: real doc in USE_GTK definition in xmenu.c.",    // C 66
	(frame: Lisp_Object),                                       // C 67
	{
  let mut menubar: *mut GtkWidget;                              // C 69
  let mut f: *mut frame;                                        // C 70

  block_input ();                                               // C 72
  f = decode_window_system_frame (frame);                       // C 73

  if FRAME_EXTERNAL_MENU_BAR (f)                                // C 75
    { set_frame_menubar (f, true); }                            // C 76

  menubar = (*FRAME_X_OUTPUT! (f)).menubar_widget;              // C 78
  if !menubar.is_null ()                                        // C 79
    {
      /* Activate the first menu.  */
      let children: *mut GList
	= gtk_container_get_children (GTK_CONTAINER! (menubar));  // C 82

      if !children.is_null ()                                   // C 84
	{
	  g_signal_emit_by_name ((*children).data, c"activate_item".as_ptr ());   // C 86
	  g_list_free (children);                                   // C 87
	}
    }
  unblock_input ();                                             // C 90

  return Qnil ();                                               // C 92
});

/* Loop util popup_activated_flag is set to zero in a callback.
   Used for popup menus and dialogs. */                            // C 95-96

/// C: static void popup_widget_loop (bool do_timers, GtkWidget *widget)
#[cfg (feature = "have_pgtk")]
unsafe fn popup_widget_loop (do_timers: bool, widget: *mut GtkWidget)   // C 98-106
{
  popup_activated_flag += 1;                                    // C 101

  /* Process events in the Gtk event loop until done.  */
  while popup_activated_flag != 0                               // C 104
    { gtk_main_iteration (); }                                  // C 105
}

/// C: void pgtk_activate_menubar (struct frame *f)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_activate_menubar (f: *mut frame)   // C 108-116
{
  set_frame_menubar (f, true);                                  // C 111

  popup_activated_flag = 1;                                     // C 113

  /* C 115: `f->output_data.pgtk->menubar_active = 1;` is C's own comment, not code.  */
}

/* This callback is invoked when a dialog or menu is finished being
   used and has been unposted.  */                                 // C 118-119

/// C: static void popup_deactivate_callback (GtkWidget *widget, gpointer client_data)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn popup_deactivate_callback (widget: *mut GtkWidget,
                                                client_data: gpointer)   // C 121-125
{
  pgtk_menu_set_in_use (false);                                 // C 124
}

/* Function that finds the frame for WIDGET and shows the HELP text
   for that widget.
   F is the frame if known, or NULL if not known.  */              // C 127-129
/// C: static void show_help_event (struct frame *f, GtkWidget *widget, Lisp_Object help)
#[cfg (feature = "have_pgtk")]
unsafe fn show_help_event (f: *mut frame, widget: *mut GtkWidget, help: Lisp_Object)   // C 130-142
{
  /* Don't show help echo on PGTK, as tooltips are always transient
     for the main widget, so on Wayland the menu will display above
     and obscure the tooltip.  FIXME: this is some low hanging fruit
     for fixing.  After you fix Fx_show_tip in pgtkterm.c so that it
     can display tooltips above menus, copy the definition of this
     function from xmenu.c.

     As a workaround, GTK is used to display menu tooltips, outside
     the Emacs help echo machinery.  */                            // C 133-141
}

/* Callback called when menu items are highlighted/unhighlighted
   while moving the mouse over them.  WIDGET is the menu bar or menu
   popup widget.  ID is its LWLIB_ID.  CALL_DATA contains a pointer to
   the data structure for the menu item, or null in case of
   unhighlighting.  */                                             // C 144-148

/// C: static void menu_highlight_callback (GtkWidget *widget, gpointer call_data)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn menu_highlight_callback (widget: *mut GtkWidget,
                                              call_data: gpointer)   // C 150-170
{
  let mut cb_data: *mut xg_menu_item_cb_data;                   // C 153
  let mut help: Lisp_Object;                                    // C 154

  cb_data = g_object_get_data (G_OBJECT! (widget), XG_ITEM_DATA)
              as *mut xg_menu_item_cb_data;                     // C 156
  if cb_data.is_null ()                                         // C 157 (`if (! cb_data)`)
    { return; }

  help = if !call_data.is_null () { (*cb_data).help } else { Qnil () };   // C 160

  /* If popup_activated_flag is greater than 1 we are in a popup menu.
     Don't pass the frame to show_help_event for those.
     Passing frame creates an Emacs event.  As we are looping in
     popup_widget_loop, it won't be handled.  Passing NULL shows the tip
     directly without using an Emacs event.  This is what the Lucid code
     does below.  */                                              // C 162-167
  show_help_event (if popup_activated_flag <= 1 { (*(*cb_data).cl_data).f }
		   else { std::ptr::null_mut () },
		   widget, help);                                 // C 168-169
}

/* Gtk calls callbacks just because we tell it what item should be
   selected in a radio group.  If this variable is set to a non-zero
   value, we are creating menus and don't want callbacks right now.
*/                                                               // C 172-175
#[cfg (feature = "have_pgtk")]
static mut xg_crazy_callback_abort: bool = false;               // C 176 (`static bool`, tentative)

/* This callback is called from the menu bar pulldown menu
   when the user makes a selection.
   Figure out what the user chose
   and put the appropriate events into the keyboard buffer.  */    // C 178-181
/// C: static void menubar_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn menubar_selection_callback (widget: *mut GtkWidget,
                                                 client_data: gpointer)   // C 182-217
{
  let cb_data: *mut xg_menu_item_cb_data
    = client_data as *mut xg_menu_item_cb_data;                   // C 185

  if xg_crazy_callback_abort                                     // C 187
    { return; }

  if cb_data.is_null () || (*cb_data).cl_data.is_null ()
      || (*(*cb_data).cl_data).f.is_null ()                      // C 190
    { return; }

  /* For a group of radio buttons, GTK calls the selection callback first
     for the item that was active before the selection and then for the one that
     is active after the selection.  For C-h k this means we get the help on
     the deselected item and then the selected item is executed.  Prevent that
     by ignoring the non-active item.  */                          // C 193-197
  if GTK_IS_RADIO_MENU_ITEM! (widget)
      && gtk_check_menu_item_get_active (GTK_CHECK_MENU_ITEM! (widget)) == 0   // C 198-199
    { return; }

  /* When a menu is popped down, X generates a focus event (i.e. focus
     goes back to the frame below the menu).  Since GTK buffers events,
     we force it out here before the menu selection event.  Otherwise
     sit-for will exit at once if the focus event follows the menu selection
     event.  */                                                   // C 202-206

  block_input ();                                                // C 208
  while gtk_events_pending () != 0                               // C 209
    { gtk_main_iteration (); }                                   // C 210
  unblock_input ();                                              // C 211

  find_and_call_menu_selection ((*(*cb_data).cl_data).f,           // C 213-216
				(*(*cb_data).cl_data).menu_bar_items_used,
				(*(*cb_data).cl_data).menu_bar_vector,
				(*cb_data).call_data as *mut c_void);
}

/* Recompute all the widgets of frame F, when the menu bar has been
   changed.  */                                                    // C 219-220

/// C: static void update_frame_menubar (struct frame *f)
#[cfg (feature = "have_pgtk")]
unsafe fn update_frame_menubar (f: *mut frame)                   // C 222-226
{
  xg_update_frame_menubar (f);                                   // C 225
}

/* Set the contents of the menubar widgets of frame F.
   The argument FIRST_TIME is currently ignored;
   it is set the first time this is called, from initialize_frame_menubar.  */   // C 228-230

/// C: void set_frame_menubar (struct frame *f, bool deep_p)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_frame_menubar (f: *mut frame, mut deep_p: bool)   // C 232-467
{
  let mut menubar_widget: *mut GtkWidget;                        // C 235
  let mut items: Lisp_Object;                                    // C 236
  let mut wv: *mut widget_value;                                 // C 237
  let mut first_wv: *mut widget_value;
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();
  let mut i: c_int;                                              // C 238
  let mut submenu_start: *mut c_int;                             // C 239
  let mut submenu_end: *mut c_int;
  let mut submenu_top_level_items: *mut bool;                    // C 240
  let mut submenu_n_panes: *mut c_int;                           // C 241


  menubar_widget
    = (*((*f).output_data.pgtk as *mut pgtk_output)).menubar_widget;   // C 244

  XSETFRAME! (Vmenu_updating_frame, f);                           // C 246

  if menubar_widget.is_null ()                                   // C 248 (`if (! menubar_widget)`)
    { deep_p = true; }

  if deep_p                                                      // C 251
    {
      let prev: *mut buffer = (current_buffer! ());                     // C 253
      let mut buffer: Lisp_Object;                                // C 254
      let specpdl_count: specpdl_ref = SPECPDL_INDEX ();           // C 255
      let mut previous_menu_items_used: c_int = (*f).menu_bar_items_used;   // C 256
      /* C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision).  */
      let mut previous_items: *mut Lisp_Object
	= sa_stack_alloc (previous_menu_items_used as usize
			  * core::mem::size_of::<Lisp_Object> ()) as *mut Lisp_Object;   // C 257-258
      let mut subitems: c_int;                                    // C 259

      /* If we are making a new widget, its contents are empty,
         do always reinitialize them.  */
      if menubar_widget.is_null ()                                // C 263
	{ previous_menu_items_used = 0; }

      buffer = (*XWINDOW (FRAME_SELECTED_WINDOW (f))).contents;   // C 266
      specbind (Qinhibit_quit (), Qt ());                          // C 267
      /* Don't let the debugger step into this code
         because it is not reentrant.  */
      specbind (Qdebug_on_next_call (), Qnil ());                  // C 270

      record_unwind_save_match_data ();                           // C 272
      if NILP (Voverriding_local_map_menu_flag)                   // C 273
	{
	  specbind (Qoverriding_terminal_local_map (), Qnil ());       // C 275
	  specbind (Qoverriding_local_map (), Qnil ());                // C 276
	}

      set_buffer_internal_1 (XBUFFER (buffer));                   // C 279

      /* Run the Lucid hook.  */
      safe_run_hooks (Qactivate_menubar_hook ());                  // C 282

      /* If it has changed current-menubar from previous value,
         really recompute the menubar from the value.  */
      safe_run_hooks (Qmenu_bar_update_hook ());                   // C 286
      fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 287

      items = FRAME_MENU_BAR_ITEMS (f);                           // C 289

      /* Save the frame's previous menu bar contents data.  */
      if previous_menu_items_used != 0                            // C 292
	{ memcpy (previous_items as *mut c_void,
		  xvector_contents ((*f).menu_bar_vector) as *const c_void,
		  previous_menu_items_used as usize * word_size as usize); }   // C 293-294

      /* Fill in menu_items with the current menu bar contents.
         This can evaluate Lisp code.  */
      save_menu_items ();                                         // C 298

      menu_items = (*f).menu_bar_vector;                           // C 300
      menu_items_allocated = if VECTORP (menu_items) { ASIZE (menu_items) as c_int } else { 0 };   // C 301
      subitems = (ASIZE (items) / 4) as c_int;                     // C 302
      submenu_start = sa_stack_alloc (((subitems + 1) as usize)
				      * core::mem::size_of::<c_int> ()) as *mut c_int;   // C 303
      submenu_end = sa_stack_alloc ((subitems as usize)
				    * core::mem::size_of::<c_int> ()) as *mut c_int;   // C 304
      submenu_n_panes = sa_stack_alloc ((subitems as usize)
					* core::mem::size_of::<c_int> ()) as *mut c_int;   // C 305
      submenu_top_level_items = sa_stack_alloc ((subitems as usize)
					* core::mem::size_of::<bool> ()) as *mut bool;   // C 306-307
      init_menu_items ();                                         // C 308
      i = 0;                                                      // C 309
      while i < subitems                                           // C 309-327
	{
	  let mut key: Lisp_Object;                                  // C 311
	  let mut string: Lisp_Object;
	  let mut maps: Lisp_Object;

	  key = AREF (items, (4 * i) as isize);                      // C 313
	  string = AREF (items, (4 * i + 1) as isize);               // C 314
	  maps = AREF (items, (4 * i + 2) as isize);                 // C 315
	  if NILP (string)                                           // C 316
	    { break; }

	  *submenu_start.offset (i as isize) = menu_items_used;      // C 319

	  menu_items_n_panes = 0;                                    // C 321
	  *submenu_top_level_items.offset (i as isize)
	    = parse_single_submenu (key, string, maps);              // C 322-323
	  *submenu_n_panes.offset (i as isize) = menu_items_n_panes;   // C 324

	  *submenu_end.offset (i as isize) = menu_items_used;        // C 326
	  i += 1;
	}

      *submenu_start.offset (i as isize) = -1;                     // C 329
      finish_menu_items ();                                       // C 330

      /* Convert menu_items into widget_value trees
         to display the menu.  This cannot evaluate Lisp code.  */

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());   // C 335
      (*wv).button_type = BUTTON_TYPE_NONE;                        // C 336
      first_wv = wv;                                               // C 337

      i = 0;                                                       // C 339
      while *submenu_start.offset (i as isize) >= 0                // C 339-352
	{
	  menu_items_n_panes = *submenu_n_panes.offset (i as isize);   // C 341
	  wv = digest_single_submenu (*submenu_start.offset (i as isize),
				      *submenu_end.offset (i as isize),
				      *submenu_top_level_items.offset (i as isize));   // C 342-343
	  if !prev_wv.is_null ()                                       // C 344
	    { (*prev_wv).next = wv; }
	  else
	    { (*first_wv).contents = wv; }
	  /* Don't set wv->name here; GC during the loop might relocate it.  */
	  (*wv).enabled = true;                                        // C 349
	  (*wv).button_type = BUTTON_TYPE_NONE;                        // C 350
	  prev_wv = wv;                                                // C 351
	  i += 1;
	}

      set_buffer_internal_1 (prev);                                // C 354

      /* If there has been no change in the Lisp-level contents
         of the menu bar, skip redisplaying it.  Just exit.  */

      /* Compare the new menu items with the ones computed last time.  */
      i = 0;                                                       // C 360
      while i < previous_menu_items_used                            // C 360-363
	{
	  if menu_items_used == i
	      || !EQ (*previous_items.offset (i as isize), AREF (menu_items, i as isize))
	    { break; }
	  i += 1;
	}
      if i == menu_items_used && i == previous_menu_items_used && i != 0   // C 364
	{
	  /* The menu items have not changed.  Don't bother updating
	     the menus in any form, since it would be a no-op.  */
	  free_menubar_widget_value_tree (first_wv);                   // C 368
	  discard_menu_items ();                                       // C 369
	  unbind_to (specpdl_count, Qnil ());                          // C 370
	  return;                                                      // C 371
	}

      /* The menu items are different, so store them in the frame.  */
      fset_menu_bar_vector (f, menu_items);                        // C 375
      (*f).menu_bar_items_used = menu_items_used;                  // C 376

      /* This undoes save_menu_items.  */
      unbind_to (specpdl_count, Qnil ());                          // C 379

      /* Now GC cannot happen during the lifetime of the widget_value,
         so it's safe to store data from a Lisp_String.  */
      wv = (*first_wv).contents;                                   // C 383
      i = 0;                                                       // C 384
      while i < ASIZE (items) as c_int                             // C 384-393
	{
	  let mut string: Lisp_Object;                                // C 386
	  string = AREF (items, i as isize + 1);                      // C 387
	  if NILP (string)                                           // C 388
	    { break; }
	  (*wv).name = SSDATA (string);                                // C 390
	  update_submenu_strings ((*wv).contents);                     // C 391
	  wv = (*wv).next;                                             // C 392
	  i += 4;
	}

    }
  else                                                           // C 396
    {
      /* Make a widget-value tree containing
         just the top level menu bar strings.  */

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());   // C 401
      (*wv).button_type = BUTTON_TYPE_NONE;                        // C 402
      first_wv = wv;                                               // C 403

      items = FRAME_MENU_BAR_ITEMS (f);                            // C 405
      i = 0;                                                       // C 406
      while i < ASIZE (items) as c_int                             // C 406-427
	{
	  let mut string: Lisp_Object;                                // C 408

	  string = AREF (items, i as isize + 1);                      // C 410
	  if NILP (string)                                           // C 411
	    { break; }

	  wv = make_widget_value (SSDATA (string), std::ptr::null_mut (), true, Qnil ());   // C 414
	  (*wv).button_type = BUTTON_TYPE_NONE;                        // C 415
	  /* This prevents lwlib from assuming this
	     menu item is really supposed to be empty.  */
	  /* The intptr_t cast avoids a warning.
	     This value just has to be different from small integers.  */
	  (*wv).call_data = -1 as intptr_t as *mut c_void;            // C 420

	  if !prev_wv.is_null ()                                       // C 422
	    { (*prev_wv).next = wv; }
	  else
	    { (*first_wv).contents = wv; }
	  prev_wv = wv;                                                // C 426
	  i += 4;
	}

      /* Forget what we thought we knew about what is in the
         detailed contents of the menu bar menus.
         Changing the top level always destroys the contents.  */
      (*f).menu_bar_items_used = 0;                                // C 432
    }

  block_input ();                                              // C 435

  xg_crazy_callback_abort = true;                              // C 437
  if !menubar_widget.is_null ()                                // C 438
    {
      /* The fourth arg is DEEP_P, which says to consider the entire
         menu trees we supply, rather than just the menu bar item names.  */
      xg_modify_menubar_widgets (menubar_widget,                // C 442-448
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
	= xg_create_widget (c"menubar".as_ptr (), c"menubar".as_ptr (), f, first_wv,   // C 452-456
			    G_CALLBACK! (menubar_selection_callback),
			    G_CALLBACK! (popup_deactivate_callback),
			    G_CALLBACK! (menu_highlight_callback));

      (*((*f).output_data.pgtk as *mut pgtk_output)).menubar_widget = menubar_widget;   // C 458
    }

  free_menubar_widget_value_tree (first_wv);                   // C 461
  update_frame_menubar (f);                                    // C 462

  xg_crazy_callback_abort = false;                             // C 464

  unblock_input ();                                            // C 466
}

/* Called from Fx_create_frame to create the initial menubar of a frame
   before it is mapped, so that the window is mapped with the menubar already
   there instead of us tacking it on later and thrashing the window after it
   is visible.  */                                                 // C 469-472

/// C: void initialize_frame_menubar (struct frame *f)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_frame_menubar (f: *mut frame)   // C 474-481
{
  /* This function is called before the first chance to redisplay
     the frame.  It has to be, so the frame will have the right size.  */
  fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 479
  set_frame_menubar (f, true);                                    // C 480
}

/* x_menu_show actually displays a menu using the panes and items in menu_items
   and returns the value selected from it.
   There are two versions of x_menu_show, one for Xt and one for Xlib.
   Both assume input is blocked by the caller.  */                 // C 484-487

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
   (We return nil on failure, but the value doesn't actually matter.)  */   // C 489-499

/* The item selected in the popup menu.  */
#[cfg (feature = "have_pgtk")]
static mut menu_item_selection: *mut Lisp_Object = std::ptr::null_mut ();   // C 502

/// C: static void popup_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn popup_selection_callback (widget: *mut GtkWidget,
                                               client_data: gpointer)   // C 504-513
{
  let cb_data: *mut xg_menu_item_cb_data
    = client_data as *mut xg_menu_item_cb_data;                   // C 507

  if xg_crazy_callback_abort                                     // C 509
    { return; }
  if !cb_data.is_null ()                                         // C 511 (`if (cb_data)`)
    { menu_item_selection = (*cb_data).call_data as *mut Lisp_Object; }   // C 512
}

/// C: static void pop_down_menu (void *arg)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn pop_down_menu (arg: *mut c_void)            // C 515-522
{
  popup_activated_flag = 0;                                      // C 518
  block_input ();                                                // C 519
  gtk_widget_destroy (GTK_WIDGET! (arg));                        // C 520
  unblock_input ();                                              // C 521
}

/* Pop up the menu for frame F defined by FIRST_WV at X/Y and loop until the
   menu pops down.
   menu_item_selection will be set to the selection.  */           // C 524-526

/// C: static void create_and_show_popup_menu (struct frame *f, widget_value * first_wv, int x,
/// int y, bool for_click)
#[cfg (feature = "have_pgtk")]
unsafe fn create_and_show_popup_menu (f: *mut frame, first_wv: *mut widget_value, x: c_int,
                                      y: c_int, for_click: bool)    // C 527-579
{
  let mut menu: *mut GtkWidget;                                  // C 531
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();              // C 532

  eassert! (FRAME_PGTK_P (f));                                   // C 534

  xg_crazy_callback_abort = true;                                // C 536
  menu = xg_create_widget (c"popup".as_ptr (), (*first_wv).name, f, first_wv,
			   G_CALLBACK! (popup_selection_callback),
			   G_CALLBACK! (popup_deactivate_callback),
			   G_CALLBACK! (menu_highlight_callback));   // C 537-540
  xg_crazy_callback_abort = false;                               // C 541

  /* Display the menu.  */
  gtk_widget_show_all (menu);                                    // C 544

  if for_click                                                   // C 546
    { gtk_menu_popup_at_pointer (GTK_MENU! (menu),
				 (*FRAME_DISPLAY_INFO! (f)).last_click_event); }   // C 547-548
  else
    {
      /* C 551-555's `GdkRectangle rect;` and its four field assignments are one struct literal:
         this toolchain rejects partial initialization of an uninitialised binding (E0381), and
	 the `src/pgtkim.rs` 491 spelling of the same record is the literal (## Deviations).  */
      let mut rect: GdkRectangle
	= GdkRectangle { x: x, y: y, width: 1, height: 1 };        // C 551-555
      gtk_menu_popup_at_rect (GTK_MENU! (menu),                   // C 556-560
			      gtk_widget_get_window (FRAME_GTK_WIDGET! (f)),
			      &rect,
			      GDK_GRAVITY_NORTH_WEST, GDK_GRAVITY_NORTH_WEST,
			      (*FRAME_DISPLAY_INFO! (f)).last_click_event);
    }

  record_unwind_protect_ptr (Some (pop_down_menu), menu as *mut c_void);   // C 563

  if gtk_widget_get_mapped (menu) != 0                           // C 565
    {
      /* Set this to one.  popup_widget_loop increases it by one, so it becomes
         two.  show_help_echo uses this to detect popup menus.  */
      popup_activated_flag = 1;                                  // C 569
      /* Process events that apply to the menu.  */
      popup_widget_loop (true, menu);                            // C 571
    }

  unbind_to (specpdl_count, Qnil ());                            // C 574

  /* Must reset this manually because the button release event is not passed
     to Emacs event loop. */
  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                        // C 578
}

/// C: static void cleanup_widget_value_tree (void *arg)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn cleanup_widget_value_tree (arg: *mut c_void)   // C 581-585
{
  free_menubar_widget_value_tree (arg as *mut widget_value);     // C 584
}

/// C: Lisp_Object pgtk_menu_show (struct frame *f, int x, int y, int menuflags, Lisp_Object
/// title, const char **error_name)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_menu_show (f: *mut frame, x: c_int, y: c_int, menuflags: c_int,
					 title: Lisp_Object,
					 error_name: *mut *const c_char) -> Lisp_Object   // C 587-845
{
  let mut i: c_int;                                              // C 591
  let mut wv: *mut widget_value;                                 // C 592
  let mut save_wv: *mut widget_value = std::ptr::null_mut ();
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();
  let mut submenu_stack: *mut *mut widget_value = std::ptr::null_mut ();   // C 593
  let mut subprefix_stack: *mut Lisp_Object = std::ptr::null_mut ();   // C 594
  let mut submenu_depth: c_int = 0;                              // C 595

  eassert! (FRAME_PGTK_P (f));                                   // C 597

  *error_name = std::ptr::null ();                               // C 599

  if menu_items_used <= MENU_ITEMS_PANE_LENGTH                   // C 601
    {
      *error_name = c"Empty menu".as_ptr ();                     // C 603
      return Qnil ();                                            // C 604
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                         // C 607
  SAFE_NALLOCA! (sa_avail, submenu_stack, 1, menu_items_used);   // C 608
  SAFE_NALLOCA! (sa_avail, subprefix_stack, 1, menu_items_used);   // C 609
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();              // C 610

  block_input ();                                                // C 612

  /* Create a tree of widget_value objects
     representing the panes and their items.  */
  wv = make_widget_value (c"menu".as_ptr (), std::ptr::null_mut (), true, Qnil ());   // C 616
  (*wv).button_type = BUTTON_TYPE_NONE;                          // C 617
  first_wv = wv;                                                 // C 618
  let mut first_pane: bool = true;                               // C 619

  /* Loop over all panes and items, filling in the tree.  */
  i = 0;                                                         // C 622
  while i < menu_items_used                                      // C 623-746
    {
      if NILP (AREF (menu_items, i as isize))                     // C 625
	{
	  *submenu_stack.offset (submenu_depth as isize) = save_wv;   // C 627
	  submenu_depth += 1;
	  save_wv = prev_wv;                                         // C 628
	  prev_wv = std::ptr::null_mut ();                           // C 629
	  first_pane = true;                                         // C 630
	  i += 1;                                                    // C 631
	}
      else if EQ (AREF (menu_items, i as isize), Qlambda ())       // C 633
	{
	  prev_wv = save_wv;                                         // C 635
	  submenu_depth -= 1;
	  save_wv = *submenu_stack.offset (submenu_depth as isize);   // C 636
	  first_pane = false;                                        // C 637
	  i += 1;                                                    // C 638
	}
      else if EQ (AREF (menu_items, i as isize), Qt ()) && submenu_depth != 0   // C 640
	{ i += MENU_ITEMS_PANE_LENGTH; }                             // C 641
      /* Ignore a nil in the item list.
         It's meaningful only for dialog boxes.  */
      else if EQ (AREF (menu_items, i as isize), Qquote ())        // C 644
	{ i += 1; }                                                  // C 645
      else if EQ (AREF (menu_items, i as isize), Qt ())            // C 646
	{
	  /* Create a new pane.  */
	  let mut pane_name: Lisp_Object;                            // C 649
	  let mut prefix: Lisp_Object;
	  let mut pane_string: *const c_char;                        // C 650

	  pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);   // C 652
	  prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);    // C 653

	  #[cfg (not (feature = "have_multilingual_menu"))]          // C 655-661
	  if STRINGP (pane_name) && STRING_MULTIBYTE (pane_name)
	    {
	      pane_name = ENCODE_MENU_STRING (pane_name);             // C 658
	      ASET (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize, pane_name);   // C 659
	    }
	  pane_string = if NILP (pane_name) { c"".as_ptr () } else { SSDATA (pane_name) };   // C 662
	  /* If there is just one top-level pane, put all its items directly
	     under the top-level menu.  */
	  if menu_items_n_panes == 1                                 // C 665
	    { pane_string = c"".as_ptr (); }                          // C 666

	  /* If the pane has a meaningful name,
	     make the pane a top-level menu item
	     with its items as a submenu beneath it.  */
	  if (menuflags & MENU_KEYMAPS) == 0 && strcmp (pane_string, c"".as_ptr ()) != 0   // C 671
	    {
	      wv = make_widget_value (pane_string, std::ptr::null_mut (), true, Qnil ());   // C 673
	      if !save_wv.is_null ()                                  // C 674
		{ (*save_wv).next = wv; }
	      else
		{ (*first_wv).contents = wv; }                        // C 677
	      if (menuflags & MENU_KEYMAPS) != 0 && !NILP (prefix)    // C 678
		{ (*wv).name = (*wv).name.offset (1); }               // C 679
	      (*wv).button_type = BUTTON_TYPE_NONE;                   // C 680
	      save_wv = wv;                                           // C 681
	      prev_wv = std::ptr::null_mut ();                        // C 682
	    }
	  else if first_pane                                          // C 684
	    {
	      save_wv = wv;                                           // C 686
	      prev_wv = std::ptr::null_mut ();                        // C 687
	    }
	  first_pane = false;                                        // C 689
	  i += MENU_ITEMS_PANE_LENGTH;                               // C 690
	}
      else
	{
	  /* Create a new item within current pane.  */
	  let mut item_name: Lisp_Object;                            // C 695
	  let mut enable: Lisp_Object;
	  let mut descrip: Lisp_Object;
	  let mut def: Lisp_Object;
	  let mut r#type: Lisp_Object;
	  let mut selected: Lisp_Object;
	  let mut help: Lisp_Object;
	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 696
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 697
	  descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 698
	  def = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_DEFINITION as isize);   // C 699
	  r#type = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_TYPE as isize);      // C 700
	  selected = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_SELECTED as isize);   // C 701
	  help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);        // C 702

	  #[cfg (not (feature = "have_multilingual_menu"))]          // C 704-716
	  {
	    if STRINGP (item_name) && STRING_MULTIBYTE (item_name)     // C 705
	      {
		item_name = ENCODE_MENU_STRING (item_name);            // C 707
		ASET (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize, item_name);   // C 708
	      }

	    if STRINGP (descrip) && STRING_MULTIBYTE (descrip)         // C 711
	      {
		descrip = ENCODE_MENU_STRING (descrip);                // C 713
		ASET (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize, descrip);   // C 714
	      }
	  }

	  wv = make_widget_value (SSDATA (item_name), std::ptr::null_mut (), !NILP (enable),
				  if STRINGP (help) { help } else { Qnil () });   // C 718-719
	  if !prev_wv.is_null ()                                     // C 720
	    { (*prev_wv).next = wv; }
	  else if !save_wv.is_null ()                                // C 722
	    { (*save_wv).contents = wv; }
	  if !NILP (descrip)                                         // C 724
	    { (*wv).key = SSDATA (descrip); }                        // C 725
	  /* If this item has a null value,
	     make the call_data null so that it won't display a box
	     when the mouse is on it.  */
	  (*wv).call_data = if !NILP (def)
			      { aref_addr (menu_items, i as isize) as *mut c_void }
			    else { std::ptr::null_mut () };           // C 729

	  if NILP (r#type)                                           // C 731
	    { (*wv).button_type = BUTTON_TYPE_NONE; }
	  else if EQ (r#type, QCtoggle ())                            // C 733
	    { (*wv).button_type = BUTTON_TYPE_TOGGLE; }
	  else if EQ (r#type, QCradio ())                             // C 735
	    { (*wv).button_type = BUTTON_TYPE_RADIO; }
	  else
	    { emacs_abort (); }                                      // C 738

	  (*wv).selected = !NILP (selected);                          // C 740

	  prev_wv = wv;                                              // C 742

	  i += MENU_ITEMS_ITEM_LENGTH;                               // C 744
	}
    }

  /* Deal with the title, if it is non-nil.  */
  if !NILP (title)                                               // C 749
    {
      let mut wv_title: *mut widget_value;                        // C 751
      let mut wv_sep1: *mut widget_value
	= make_widget_value (c"--".as_ptr (), std::ptr::null_mut (), false, Qnil ());   // C 752
      let mut wv_sep2: *mut widget_value
	= make_widget_value (c"--".as_ptr (), std::ptr::null_mut (), false, Qnil ());   // C 753

      (*wv_sep2).next = (*first_wv).contents;                     // C 755
      (*wv_sep1).next = wv_sep2;                                  // C 756

      #[cfg (not (feature = "have_multilingual_menu"))]           // C 758-761
      let mut title: Lisp_Object = title;
      #[cfg (not (feature = "have_multilingual_menu"))]
      if STRING_MULTIBYTE (title)                                 // C 759
	{ title = ENCODE_MENU_STRING (title); }                    // C 760

      wv_title = make_widget_value (SSDATA (title), std::ptr::null_mut (), true, Qnil ());   // C 763
      (*wv_title).button_type = BUTTON_TYPE_NONE;                 // C 764
      (*wv_title).next = wv_sep1;                                 // C 765
      (*first_wv).contents = wv_title;                            // C 766
    }

  /* No selection has been chosen yet.  */
  menu_item_selection = std::ptr::null_mut ();                   // C 770

  /* Make sure to free the widget_value objects we used to specify the
     contents even with longjmp.  */
  record_unwind_protect_ptr (Some (cleanup_widget_value_tree), first_wv as *mut c_void);   // C 774

  /* Actually create and show the menu until popped down.  */
  create_and_show_popup_menu (f, first_wv, x, y, (menuflags & MENU_FOR_CLICK) != 0);   // C 777

  unbind_to (specpdl_count, Qnil ());                            // C 779

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if !menu_item_selection.is_null ()                             // C 783
    {
      let mut prefix: Lisp_Object;                               // C 785
      let mut entry: Lisp_Object;

      prefix = Qnil (); entry = Qnil ();                         // C 787
      i = 0;                                                     // C 788
      while i < menu_items_used                                 // C 789-833
	{
	  if NILP (AREF (menu_items, i as isize))                 // C 791
	    {
	      *subprefix_stack.offset (submenu_depth as isize) = prefix;   // C 793
	      submenu_depth += 1;
	      prefix = entry;                                        // C 794
	      i += 1;                                                // C 795
	    }
	  else if EQ (AREF (menu_items, i as isize), Qlambda ())     // C 797
	    {
	      submenu_depth -= 1;
	      prefix = *subprefix_stack.offset (submenu_depth as isize);   // C 799
	      i += 1;                                                // C 800
	    }
	  else if EQ (AREF (menu_items, i as isize), Qt ())          // C 802
	    {
	      prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);   // C 804
	      i += MENU_ITEMS_PANE_LENGTH;                            // C 805
	    }
	  /* Ignore a nil in the item list.
	     It's meaningful only for dialog boxes.  */
	  else if EQ (AREF (menu_items, i as isize), Qquote ())      // C 809
	    { i += 1; }                                              // C 810
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 813
	      if menu_item_selection == aref_addr (menu_items, i as isize)   // C 814
		{
		  if (menuflags & MENU_KEYMAPS) != 0                 // C 816
		    {
		      let mut j: c_int;                              // C 818

		      entry = list1 (entry);                         // C 820
		      if !NILP (prefix)                              // C 821
			{ entry = Fcons (prefix, entry); }            // C 822
		      j = submenu_depth - 1;                         // C 823
		      while j >= 0
			{
			  if !NILP (*subprefix_stack.offset (j as isize))   // C 824
			    { entry = Fcons (*subprefix_stack.offset (j as isize), entry); }   // C 825
			  j -= 1;
			}
		    }
		  unblock_input ();                                  // C 827
		  SAFE_FREE! (sa_count, sa_mark);                             // C 828
		  return entry;                                      // C 829
		}
	      i += MENU_ITEMS_ITEM_LENGTH;                           // C 831
	    }
	}
    }
  else if (menuflags & MENU_FOR_CLICK) == 0                      // C 835
    {
      unblock_input ();                                          // C 837
      /* Make "Cancel" equivalent to C-g.  */
      quit ();                                                   // C 839
    }

  unblock_input ();                                              // C 842
  SAFE_FREE! (sa_count, sa_mark);                                         // C 843
  return Qnil ();                                                // C 844
}

/// C: static void dialog_selection_callback (GtkWidget *widget, gpointer client_data)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn dialog_selection_callback (widget: *mut GtkWidget,
                                                client_data: gpointer)   // C 847-856
{
  /* Treat the pointer as an integer.  There's no problem
     as long as pointers have enough bits to hold small integers.  */
  if client_data as intptr_t != -1                               // C 852
    { menu_item_selection = client_data as *mut Lisp_Object; }   // C 853

  popup_activated_flag = 0;                                      // C 855
}

/* Pop up the dialog for frame F defined by FIRST_WV and loop until the
   dialog pops down.
   menu_item_selection will be set to the selection.  */            // C 858-860

/// C: static void create_and_show_dialog (struct frame *f, widget_value *first_wv)
#[cfg (feature = "have_pgtk")]
unsafe fn create_and_show_dialog (f: *mut frame, first_wv: *mut widget_value)   // C 861-885
{
  let mut menu: *mut GtkWidget;                                  // C 864

  eassert! (FRAME_PGTK_P (f));                                   // C 866

  menu = xg_create_widget (c"dialog".as_ptr (), (*first_wv).name, f, first_wv,
			   G_CALLBACK! (dialog_selection_callback),
			   G_CALLBACK! (popup_deactivate_callback), None);   // C 868-870

  if !menu.is_null ()                                            // C 872
    {
      let specpdl_count: specpdl_ref = SPECPDL_INDEX ();           // C 874
      record_unwind_protect_ptr (Some (pop_down_menu), menu as *mut c_void);   // C 875

      /* Display the menu.  */
      gtk_widget_show_all (menu);                                // C 878

      /* Process events that apply to the menu.  */
      popup_widget_loop (true, menu);                            // C 881

      unbind_to (specpdl_count, Qnil ());                        // C 883
    }
}

#[cfg (feature = "have_pgtk")]
static mut button_names: [*const c_char; 10] = [                 // C 887-890
  c"button1".as_ptr (), c"button2".as_ptr (), c"button3".as_ptr (), c"button4".as_ptr (),
  c"button5".as_ptr (), c"button6".as_ptr (), c"button7".as_ptr (), c"button8".as_ptr (),
  c"button9".as_ptr (), c"button10".as_ptr () ];

/// C: Lisp_Object pgtk_dialog_show (struct frame *f, Lisp_Object title, Lisp_Object header,
/// const char **error_name)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
					    error_name: *mut *const c_char) -> Lisp_Object   // C 892-1046
{
  let mut i: c_int;                                              // C 896
  let mut nb_buttons: c_int = 0;
  let mut dialog_name: [c_char; 6] = [0; 6];                     // C 897

  let mut wv: *mut widget_value;                                 // C 899
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();

  /* Number of elements seen so far, before boundary.  */
  let mut left_count: c_int = 0;                                 // C 902
  /* Whether we've seen the boundary between left-hand elts and right-hand.  */
  let mut boundary_seen: bool = false;                           // C 904

  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();              // C 906

  eassert! (FRAME_PGTK_P (f));                                   // C 908

  *error_name = std::ptr::null ();                               // C 910

  if menu_items_n_panes > 1                                      // C 912
    {
      *error_name = c"Multiple panes in dialog box".as_ptr ();    // C 914
      return Qnil ();                                            // C 915
    }

  /* Create a tree of widget_value objects
     representing the text label and buttons.  */
  {
    let mut pane_name: Lisp_Object;                              // C 921
    let mut pane_string: *const c_char;                          // C 922
    pane_name = AREF (menu_items, MENU_ITEMS_PANE_NAME as isize);   // C 923
    pane_string = if NILP (pane_name) { c"".as_ptr () } else { SSDATA (pane_name) };   // C 924
    prev_wv = make_widget_value (c"message".as_ptr (), pane_string as *mut c_char, true,
				 Qnil ());                       // C 925
    first_wv = prev_wv;                                          // C 926

    /* Loop over all panes and items, filling in the tree.  */
    i = MENU_ITEMS_PANE_LENGTH;                                  // C 929
    while i < menu_items_used                                    // C 930-973
      {

	/* Create a new item within current pane.  */
	let mut item_name: Lisp_Object;                             // C 934
	let mut enable: Lisp_Object;
	let mut descrip: Lisp_Object;
	item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 935
	enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 936
	descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 937

	if NILP (item_name)                                          // C 939
	  {
	    free_menubar_widget_value_tree (first_wv);                // C 941
	    *error_name = c"Submenu in dialog items".as_ptr ();       // C 942
	    return Qnil ();                                          // C 943
	  }
	if EQ (item_name, Qquote ())                                 // C 945
	  {
	    /* This is the boundary between left-side elts
	       and right-side elts.  Stop incrementing right_count.  */
	    boundary_seen = true;                                    // C 949
	    i += 1;                                                  // C 950
	    continue;                                                // C 951
	  }
	if nb_buttons >= 9                                           // C 953
	  {
	    free_menubar_widget_value_tree (first_wv);                // C 955
	    *error_name = c"Too many dialog items".as_ptr ();         // C 956
	    return Qnil ();                                          // C 957
	  }

	wv = make_widget_value (button_names[nb_buttons as usize],   // C 960-961
				SSDATA (item_name), !NILP (enable), Qnil ());
	(*prev_wv).next = wv;                                        // C 962
	if !NILP (descrip)                                           // C 963
	  { (*wv).key = SSDATA (descrip); }                          // C 964
	(*wv).call_data = aref_addr (menu_items, i as isize) as *mut c_void;   // C 965
	prev_wv = wv;                                                // C 966

	if !boundary_seen                                           // C 968
	  { left_count += 1; }                                       // C 969

	nb_buttons += 1;                                            // C 971
	i += MENU_ITEMS_ITEM_LENGTH;                                // C 972
      }

    /* If the boundary was not specified,
       by default put half on the left and half on the right.  */
    if !boundary_seen                                            // C 977
      { left_count = nb_buttons - nb_buttons / 2; }               // C 978

    wv = make_widget_value (dialog_name.as_mut_ptr (), std::ptr::null_mut (), false, Qnil ());   // C 980

    /*  Frame title: 'Q' = Question, 'I' = Information.
       Can also have 'E' = Error if, one day, we want
       a popup for errors. */
    if NILP (header)                                             // C 985
      { dialog_name[0] = b'Q' as c_char; }                        // C 986
    else
      { dialog_name[0] = b'I' as c_char; }                        // C 988

    /* Dialog boxes use a really stupid name encoding
       which specifies how many buttons to use
       and how many buttons are on the right. */
    dialog_name[1] = (b'0' as c_int + nb_buttons) as c_char;     // C 993
    dialog_name[2] = b'B' as c_char;                             // C 994
    dialog_name[3] = b'R' as c_char;                             // C 995
    /* Number of buttons to put on the right.  */
    dialog_name[4] = (b'0' as c_int + nb_buttons - left_count) as c_char;   // C 997
    dialog_name[5] = 0;                                          // C 998
    (*wv).contents = first_wv;                                   // C 999
    first_wv = wv;                                               // C 1000
  }

  /* No selection has been chosen yet.  */
  menu_item_selection = std::ptr::null_mut ();                   // C 1004

  /* Make sure to free the widget_value objects we used to specify the
     contents even with longjmp.  */
  record_unwind_protect_ptr (Some (cleanup_widget_value_tree), first_wv as *mut c_void);   // C 1008

  /* Actually create and show the dialog.  */
  create_and_show_dialog (f, first_wv);                          // C 1011

  unbind_to (specpdl_count, Qnil ());                            // C 1013

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if !menu_item_selection.is_null ()                             // C 1017
    {
      i = 0;                                                     // C 1019
      while i < menu_items_used                                 // C 1020-1039
	{
	  let mut entry: Lisp_Object;                               // C 1022

	  if EQ (AREF (menu_items, i as isize), Qt ())              // C 1024
	    { i += MENU_ITEMS_PANE_LENGTH; }
	  else if EQ (AREF (menu_items, i as isize), Qquote ())      // C 1026
	    {
	      /* This is the boundary between left-side elts and
	         right-side elts.  */
	      i += 1;                                                // C 1030
	    }
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 1034
	      if menu_item_selection == aref_addr (menu_items, i as isize)   // C 1035
		{ return entry; }                                     // C 1036
	      i += MENU_ITEMS_ITEM_LENGTH;                            // C 1037
	    }
	}
    }
  else
    /* Make "Cancel" equivalent to C-g.  */
    { quit (); }                                                 // C 1043

  return Qnil ();                                                // C 1045
}

/// C: Lisp_Object pgtk_popup_dialog (struct frame *f, Lisp_Object header, Lisp_Object contents)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pgtk_popup_dialog (f: *mut frame, header: Lisp_Object,
					    mut contents: Lisp_Object) -> Lisp_Object   // C 1048-1082
{
  let mut title: Lisp_Object;                                    // C 1051
  let mut error_name: *const c_char = std::ptr::null ();         // C 1052 (C leaves it unset; the
                                                                 // callee writes it - see ## Deviations)

  let mut selection: Lisp_Object;                                // C 1053
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();              // C 1054

  check_window_system (f);                                       // C 1056

  /* Decode the dialog items from what was specified.  */
  title = Fcar (contents);                                       // C 1059
  CHECK_STRING (title);                                          // C 1060
  record_unwind_protect_void (Some (unuse_menu_items));          // C 1061

  if NILP (Fcar (Fcdr (contents)))                               // C 1063
    /* No buttons specified, add an "Ok" button so users can pop down
       the dialog.  Also, the lesstif/motif version crashes if there are
       no buttons.  */
    { contents = list2 (title, Fcons (build_string (c"Ok".as_ptr ()), Qt ())); }   // C 1067

  list_of_panes (list1 (contents));                              // C 1069

  /* Display them in a dialog box.  */
  block_input ();                                                // C 1072
  selection = pgtk_dialog_show (f, title, header, &mut error_name);   // C 1073
  unblock_input ();                                              // C 1074

  unbind_to (specpdl_count, Qnil ());                            // C 1076
  discard_menu_items ();                                         // C 1077

  if !error_name.is_null ()                                      // C 1079
    { error (c"%s".as_ptr (), error_name); }                     // C 1080
  return selection;                                              // C 1081
}

/* Detect if a dialog or menu has been posted.  MSDOS has its own
   implementation on msdos.c.  */                                 // C 1084-1085

/// C: int popup_activated (void)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn popup_activated () -> c_int             // C 1087-1091
{
  return popup_activated_flag;                                   // C 1090
}

/* The following is used by delayed window autoselection.  */      // C 1093

/* C 1095-1097: `DEFUN ("menu-or-popup-active-p", …)` - C 1097's `\(` is C's own escaping of the
   docstring's parenthesis (make-docfile turns it into `(`).  */
#[cfg (feature = "have_pgtk")]
DEFUN! ("menu-or-popup-active-p", Fmenu_or_popup_active_p, Smenu_or_popup_active_p, 0, 0, 0,
	doc: "Return t if a menu or popup dialog is active.\n(On MS Windows, this refers to the selected frame.)",   // C 1096-1097
	(),                                                         // C 1098
	{
  return if popup_activated () != 0 { Qt () } else { Qnil () };   // C 1100
});

/* C 1103's `static void syms_of_pgtkmenu_for_pdumper (void);` - C's forward declaration, dropped
   (PORTING.md 0.1-12); the definition is at its C position below.  */

/// C: void syms_of_pgtkmenu (void)
#[cfg (feature = "have_pgtk")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_pgtkmenu ()                    // C 1105-1118
{
  DEFSYM! (Qdebug_on_next_call, c"debug-on-next-call");         // C 1108
  defsubr (&raw mut Smenu_or_popup_active_p);                   // C 1109

  DEFSYM! (Qframe_monitor_workarea, c"frame-monitor-workarea");   // C 1111

  defsubr (&raw mut Sx_menu_bar_open_internal);                 // C 1113
  Ffset (intern_c_string (c"accelerate-menu".as_ptr ()),           // C 1114
	 intern_c_string (Sx_menu_bar_open_internal.s.symbol_name));   // C 1115

  pdumper_do_now_and_after_load (Some (syms_of_pgtkmenu_for_pdumper));   // C 1117
}

/// C: static void syms_of_pgtkmenu_for_pdumper (void)
#[cfg (feature = "have_pgtk")]
unsafe extern "C" fn syms_of_pgtkmenu_for_pdumper ()            // C 1120-1123
{
}
