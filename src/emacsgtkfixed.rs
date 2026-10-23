// femacs: port of src/emacsgtkfixed.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, local), the same control flow and the same error model.  The two
// non-`static` C definitions (`XSetWMSizeHints`, `XSetWMNormalHints`) carry
// `#[unsafe(no_mangle)] pub unsafe extern "C"` (PORTING.md 0.2); the C `static` helpers stay plain
// `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The build
// puts `emacsgtkfixed.o` in `GTK_OBJ` only in the GTK3 branch (`configure.ac` 3756
// `AC_DEFINE ([HAVE_GTK3], [1])` beside 3757 `GTK_OBJ=emacsgtkfixed.o`) and the object list uses
// `GTK_OBJ` only when the toolkit compiles (3818-3819 empties it otherwise, 3826
// `AC_DEFINE ([USE_GTK], [1])` beside 3827 `GTK_OBJ="gtkutil.o $GTK_OBJ"`; `emacs/src/Makefile.in`
// 182 takes it from `@GTK_OBJ@` and 452 puts `$(GTK_OBJ)` in the object list).  Both macros are
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` (HAVE_GTK3 1385, USE_GTK 3861), and
// `docs/baseline/frozen-build-sources.txt` lists no `emacsgtkfixed.o` at all, so the text can never
// be *compiled* here.  What the ledger carries instead is P-86 ②'s substitute shape - the C
// line-range table over all 210 lines - beside `## Evidence` and `## Deviations`.
//
//   * the **file gate** is `all (feature = "use_gtk", feature = "have_gtk3")` (configure.ac 3826 /
//     3756), on every item *and on C 23-30's include graph* (P-86 ⑧) - the same pair
//     `src/gtkutil.rs` 123 gates the very same `"emacsgtkfixed.h"` edge with;
//   * the file's one inner condition is C 25's `#ifdef HAVE_PGTK`, and **both arms are carried**
//     (P-86 ①): the `#ifdef` arm is the PGTK one (C 26/101-104/119-122, feature `have_pgtk`) and the
//     `#else` arm the X11/GTK3 one (C 27-28/105-109/123-127/131-210).  A GTK3-on-X11 build has
//     `HAVE_X_WINDOWS` (configure.ac 3138-3142's `x11` case) and `include/xterm.rs`'s records are
//     gated with it, so an X11 flip must set `have_x_windows` as well or the mirror's records are
//     absent (the `docs/ports/gtkutil.md` Evidence ② arm-A `--cfg` set);
//   * **the GTK surface is the SDK mirror's** (`include/gtk.rs`, P-90 ②); this unit reaches it at
//     C 30, the position where `"emacsgtkfixed.h"` 24's `<gtk/gtk.h>` enters C's chain.  The names
//     that mirror does not carry yet stand here at their C position (P-88 ①) and are owed back to
//     it (`## Owed` 1).  **GDK and GTK are not on this machine** (measured in `## Evidence`), so
//     nothing of theirs is measured and the records they own are UNVERIFIED (P-88 ②); GLib, GObject
//     and X11 *are*, so those declarations quote their header and line.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         unused_parens, static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`

/* C 23-30: the six `#include`s, every edge carrying the file gate (P-86 ⑧).  C 25-29's
   `#ifdef HAVE_PGTK` picks the terminal header; both arms are spelled here.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/lisp.rs");                   // C 23: `#include "lisp.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/frame.rs");                  // C 24: `#include "frame.h"`
/* `"xterm.h"` 23's `<X11/Xlib.h>` and `"pgtkterm.h"` 52's `<X11/Xresource.h>`: the X11 SDK
   surface's single home is `include/xlib.rs` (P-90 ②) and the edge is spelled **ungated**, the
   P-89 workaround ("write the edge ungated and let the target carry its own content gates"): the
   materializer gives a file one expansion slot, and with both terminal headers reaching it through
   a `#[cfg]` the pgtk-gated copy would claim it and leave the X11 arm without `x11_constants` and
   the `XID`/`Display` aliases.  Every item in that mirror carries `have_x_windows` itself
   (measured), so the edge compiles nothing in the frozen configuration.  */
include!("include/xlib.rs");                   // C 28's chain: `"xterm.h"` 23
/* The chain edges C 24's `"frame.h"` and C 25-29's terminal header reach: `"termhooks.h"` 27/28
   (`#include "dispextern.h"`, `#include "systime.h" /* for Time */`) and `"frame.h"`'s own 22's
   `"termhooks.h"`.  `Emacs_Color`, `text_cursor_kinds` and `Mouse_HLInfo` are dispextern.h's,
   `Time` is systime.h's, and `font` (the `*mut font` member of the mirror's `x_output`) is
   font.h's; the mirrors name them and P-72 ④ says the edge is spelled by the consumer that needs
   them (the `src/gfilenotify.rs` 36-44 and `src/xgselect.rs` 55-62 readings).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/dispextern.rs");             // termhooks.h 27
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/systime.rs");                // termhooks.h 28: `Time`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/font.rs");                   // xterm.h 1131's `struct font *font`
/* `"dispextern.h"`'s and `"lisp.h"`'s character helpers (`MAX_CHAR`, `CHAR_VALID_P`,
   `STRING_CHAR`, `UNIBYTE_TO_CHAR`, `BYTES_BY_CHAR_HEAD`, `string_char_and_length`,
   `raw_prev_char_len`): `include/character.rs` is their home and the mirrors' inline bodies call
   them (`docs/ports/gfilenotify.md`'s same four-edge reading).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/character.rs");              // lisp.h/character.h's character helpers
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // C 26: `#include "pgtkterm.h"`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
include!("include/xterm.rs");                  // C 28: `#include "xterm.h"`

/* C 28's chain: `"xterm.h"` 23's `#include <X11/Xlib.h>`, 46's `<X11/Xutil.h>` and 48's
   `<X11/Xatom.h>`.  X11 *is* on this machine, so every value and every offset below is a clang
   measurement (see `## Evidence`), and each declaration is owed to the X11 surface's single home
   `include/xlib.rs` (P-90 ②) - except the two `xterm.h` prototypes, which are owed to
   `include/xterm.rs`.  `## Owed` 2/3 carry both moves.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
pub const PMinSize: c_long = 1 << 4;           // C: Xutil.h 107: `#define PMinSize (1L << 4)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
pub const PResizeInc: c_long = 1 << 6;         // C: Xutil.h 109: `#define PResizeInc (1L << 6)`
/// C: Xatom.h 48: `#define XA_WM_NORMAL_HINTS ((Atom) 40)` (C 207's argument).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
pub const XA_WM_NORMAL_HINTS: Atom = 40;
/// C: Xatom.h 49: `#define XA_WM_SIZE_HINTS ((Atom) 41)` (C 196's argument).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
pub const XA_WM_SIZE_HINTS: Atom = 41;
/// C: Xutil.h 87-90: the unnamed `struct { int x;   /* numerator */ int y;   /* denominator */ }`
/// the two aspects share.  Rust has no nested record definition, so the port names it after the
/// members it serves (PORTING.md P-67's anonymous-record rule; `include/dispextern.rs` 356's
/// `<union>_<member>` spelling).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSizeHints_aspect
{
  pub x: c_int,                                // C: `int x;   /* numerator */`
  pub y: c_int,                                // C: `int y;   /* denominator */`
}
/// C: Xutil.h 81-94: `typedef struct { long flags; int x, y; int width, height; int min_width,
/// min_height; int max_width, max_height; int width_inc, height_inc; struct { … } min_aspect,
/// max_aspect; int base_width, base_height; int win_gravity; } XSizeHints;` - the eighteen
/// members C 146-163 copies into `long data[18]`, in that order.  Measured (Evidence ③):
/// `sizeof` 80, align 8, and the eighteen offsets asserted below.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSizeHints
{
  pub flags: c_long,
  pub x: c_int, pub y: c_int,
  pub width: c_int, pub height: c_int,
  pub min_width: c_int, pub min_height: c_int,
  pub max_width: c_int, pub max_height: c_int,
  pub width_inc: c_int, pub height_inc: c_int,
  pub min_aspect: XSizeHints_aspect, pub max_aspect: XSizeHints_aspect,
  pub base_width: c_int, pub base_height: c_int,
  pub win_gravity: c_int,
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::size_of::<XSizeHints> () == 80);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::align_of::<XSizeHints> () == 8);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, flags) == 0);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, x) == 8);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, y) == 12);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, width) == 16);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, height) == 20);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, min_width) == 24);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, min_height) == 28);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, max_width) == 32);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, max_height) == 36);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, width_inc) == 40);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, height_inc) == 44);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, min_aspect) == 48);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, max_aspect) == 56);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, base_width) == 64);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, base_height) == 68);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XSizeHints, win_gravity) == 72);
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
unsafe extern "C" {
  /// C 196: `XChangeProperty (d, w, prop, XA_WM_SIZE_HINTS, 32, PropModeReplace,
  /// (unsigned char *) data, 18);`
  /// C: Xlib.h 2078-2087: `extern int XChangeProperty (Display *, Window, Atom, Atom, int, int,
  /// _Xconst unsigned char *, int);`
  pub fn XChangeProperty (display: *mut Display, w: Window, property: Atom, type_: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int)
			  -> c_int;
  /// C 143: `struct x_display_info *dpyinfo = x_display_info_for_display (d);`
  /// C: xterm.h 1003: `extern struct x_display_info *x_display_info_for_display (Display *);`
  pub fn x_display_info_for_display (dpy: *mut Display) -> *mut x_display_info;
  /// C 144: `struct frame *f = x_top_window_to_frame (dpyinfo, w);` (C's second argument is
  /// `Window`; the prototype takes C's `int`, so the call site carries the narrowing - Deviation 4)
  /// C: xterm.h 1004: `extern struct frame *x_top_window_to_frame (struct x_display_info *, int);`
  pub fn x_top_window_to_frame (dpyinfo: *mut x_display_info, w: c_int) -> *mut frame;
}

/* C 30: `#include "emacsgtkfixed.h"` and, through its 24's `<gtk/gtk.h>`, the SDK surface.  Both
   edges carry the file gate (P-86 ⑧).  The header mirror is a mirror: its text is written for the
   *crate root* (it names `c_void`, which `include/lisp.rs` imports there), so it is `include!`d
   here directly - wrapping it in a module leaves every such name unresolved (measured: a nested
   module does not see the unit's imports).  Its one top-level prototype, `emacs_fixed_new`, is
   therefore in this unit's crate root together with this unit's *definition* of that function,
   which is `E0428`; P-39's remedy (the prototype into a `pub mod …_decls` + glob import, the
   `include/xterm.rs` 212 shape) is a change *inside* that mirror and is owed (`## Owed` 4).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/gtk.rs");                    // C 30's chain: `"emacsgtkfixed.h"` 24
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
include!("include/emacsgtkfixed.rs");          // C 30: `#include "emacsgtkfixed.h"`

/* GLib/GObject names C 30's chain reaches and `include/gtk.rs` does not carry yet (P-88 ①; GLib
   *is* on this machine, so every line below is read from the header).  `## Owed` 1 owes them.  */
/// C: gobject/gtype.h 451-455: `struct _GTypeClass { GType g_type; };`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GTypeClass
{
  pub g_type: GType,
}
/// C: gobject/gtype.h 461-465: `struct _GTypeInstance { GTypeClass *g_class; };`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct GTypeInstance
{
  pub g_class: *mut GTypeClass,
}
/// C: gobject/gtype.h 528: `#define G_TYPE_CHECK_INSTANCE_CAST(instance, g_type, c_type)
/// (_G_TYPE_CIC ((instance), (g_type), c_type))`, whose own body is gtype.h 2672's
/// `_G_TYPE_CIC(ip, gt, ct) ((ct*) (void *) g_type_check_instance_cast ((GTypeInstance*) ip,
/// gt))`.  C 61 is this file's one use; the macro keeps C's name, its three arguments and its
/// result type.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_TYPE_CHECK_INSTANCE_CAST {
  ($instance:expr, $g_type:expr, $c_type:ty) => {
    g_type_check_instance_cast ($instance as *mut GTypeInstance, $g_type) as *mut $c_type
  };
}
/// C: gobject/gtype.h 688: `#define G_TYPE_INSTANCE_GET_PRIVATE(instance, g_type, c_type)
/// ((c_type*) g_type_instance_get_private ((GTypeInstance*) (instance), (g_type)))` - the same
/// treatment.  C 80 is this file's one use.
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
macro_rules! G_TYPE_INSTANCE_GET_PRIVATE {
  ($instance:expr, $g_type:expr, $c_type:ty) => {
    g_type_instance_get_private ($instance as *mut GTypeInstance, $g_type) as *mut $c_type
  };
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  /// C 61: `G_TYPE_CHECK_INSTANCE_CAST (widget, emacs_fixed_get_type (), EmacsFixed)`
  /// C: gobject/gtype.h 2633-2634: `GTypeInstance *g_type_check_instance_cast (GTypeInstance
  /// *instance, GType iface_type);`
  pub fn g_type_check_instance_cast (instance: *mut GTypeInstance, iface_type: GType)
				     -> *mut GTypeInstance;
  /// C 80: `G_TYPE_INSTANCE_GET_PRIVATE (fixed, emacs_fixed_get_type (), EmacsFixedPrivate)`
  /// C: gobject/gtype.h 1515-1516: `gpointer g_type_instance_get_private (GTypeInstance
  /// *instance, GType private_type);`
  pub fn g_type_instance_get_private (instance: *mut GTypeInstance, private_type: GType)
				      -> gpointer;
  /// C 74: `g_type_class_add_private (klass, sizeof (EmacsFixedPrivate));`
  /// C: gobject/gtype.h 1509-1510: `void g_type_class_add_private (gpointer g_class, gsize
  /// private_size);`
  pub fn g_type_class_add_private (g_class: gpointer, private_size: gsize);
}

/* C 32-36: the GCC-bug-683906 comment and the version gate

     #if GNUC_PREREQ (4, 7, 0) && ! GLIB_CHECK_VERSION (2, 35, 7)
     # pragma GCC diagnostic push
     # pragma GCC diagnostic ignored "-Wunused-local-typedefs"
     #endif

   A `#pragma GCC diagnostic` has no Rust spelling and silences a diagnostic Rust cannot emit
   (`-Wunused-local-typedefs` on a local typedef), so C's text is kept and **no arm is written**
   (the `src/pgtkterm.rs` 69, `src/pdumper.rs` 224, `src/timefns.rs` 79 and `src/ccl.rs` 913
   readings).  The condition is *decided*, not guessed: `GNUC_PREREQ` is conf_post.h's
   `__GNUC__`/`__GNUC_MINOR__` test and the frozen table has 4 and 2
   (`docs/baseline/config/darwin-arm64.macros.tsv` 6264/6259), so `GNUC_PREREQ (4, 7, 0)` is
   **false** on this target and the whole guard is dead here.  The tree's registered
   `glib_check_version_lt_2_36` (`xtask/Cargo.toml`, `src/gfilenotify.rs` 298-305's
   `#if ! GLIB_CHECK_VERSION (2, 36, 0)`) is **not** this condition (2.36.0, not 2.35.7) and must
   not be reused for it; no feature is needed while no arm is written (`## Owed` 5).  */

/* C 38-39: the two typedefs of the header's records.  Rust has no `typedef`, so each is an alias
   of the tag the mirror (or the copy below) declares.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
type EmacsFixed = _EmacsFixed;
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
type EmacsFixedClass = _EmacsFixedClass;

/* C 41-44: `struct _EmacsFixedPrivate { struct frame *f; };` - the one record this file defines
   itself (the header only typedefs the tag, `emacsgtkfixed.h` 37).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[repr(C)]
pub struct _EmacsFixedPrivate
{
  pub f: *mut frame,                       // C 43: `struct frame *f;`
}
/// C: emacsgtkfixed.h 37: `typedef struct _EmacsFixedPrivate EmacsFixedPrivate;` (the header's
/// one typedef of this tag; C 74/80/89 spell the typedef name).
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
type EmacsFixedPrivate = _EmacsFixedPrivate;

/* `emacsgtkfixed.h` 39-50: `struct _EmacsFixed { GtkFixed container; EmacsFixedPrivate *priv; }`
   and `struct _EmacsFixedClass { GtkFixedClass parent_class; }`.  The header mirror carries them
   under its own `#[cfg (feature = "have_pgtk")]`, but the header's two records are **not** inside
   any `#ifdef` (its 30-33 `HAVE_PGTK` block holds only the two type macros) - so this unit states
   them for the arm that mirror withholds them from, with that mirror's own member spellings and
   names (the two GTK members are records no compiler here can measure, P-88 ②/UNVERIFIED, and the
   priv pointer is `*mut c_void` there too).  `## Deviations` 1 and `## Owed` 4 carry the reading.  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[repr(C)]
pub struct _EmacsFixed
{
  /* GtkFixed container;  */
  pub container: *mut c_void,
  /* EmacsFixedPrivate *priv;  */
  pub r#priv: *mut c_void,
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[repr(C)]
pub struct _EmacsFixedClass
{
  /* GtkFixedClass parent_class;  */
  pub parent_class: *mut c_void,
}

/* C 47-52: the two `static` forward declarations of the preferred-size callbacks.  PORTING.md
   0.1-12 drops them (Rust has no forward declarations and its items are order-independent); their
   definitions stay at C 94-110 / C 112-128.  C 53-55's `#ifndef HAVE_PGTK` /
   `static GType emacs_fixed_get_type (void);` / `#endif` is the other declaration there, and under
   `HAVE_PGTK` it is `emacsgtkfixed.h` 52-54's `extern GType emacs_fixed_get_type (void);`.  The
   *definition* is C 56's `G_DEFINE_TYPE` expansion (owed), so the name is declared here - in a
   module with a glob import, so the definition shadows it instead of colliding with it (PORTING.md
   0.1-10, P-39's shape, the `include/xterm.rs` 212 `xterm_bitmap_decls` precedent).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
mod emacs_fixed_get_type_decls {
  use super::*;

  unsafe extern "C" {
    /// C 54 / `emacsgtkfixed.h` 53: `GType emacs_fixed_get_type (void);`
    pub fn emacs_fixed_get_type () -> GType;
  }
}
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
use emacs_fixed_get_type_decls::*;

/* C 56: `G_DEFINE_TYPE (EmacsFixed, emacs_fixed, GTK_TYPE_FIXED)`.  GLib's own text is gtype.h
   1840's `#define G_DEFINE_TYPE(TN, t_n, T_P) G_DEFINE_TYPE_EXTENDED (TN, t_n, T_P, 0, {})`, and
   its 2293-2348 expansion (`_G_DEFINE_TYPE_EXTENDED_BEGIN_PRE`/`_REGISTER`/`_END`) is the type
   registration boilerplate: `GTK_TYPE_FIXED`, `sizeof (EmacsFixed)`, `sizeof (EmacsFixedClass)`,
   `GTypeInfo` and `g_type_register_static_simple`.  **PORTING.md has no entry for an
   item-defining macro of a third-party header** (its §11 owes one), and GTK is not on this
   machine, so the invocation is carried as C's text here and the expansion - which *defines*
   `emacs_fixed_get_type` and `emacs_fixed_parent_class`, forward-declares `emacs_fixed_init` and
   `emacs_fixed_class_init` (C 65-83) and registers the class - is owed (`## Owed` 6; the
   `src/gtkutil.rs` 313-320 treatment of the very macro this file is the sibling of).  */

// C 58-63: `static EmacsFixed * EMACS_FIXED (GtkWidget *widget)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe fn EMACS_FIXED (widget: *mut GtkWidget) -> *mut EmacsFixed
{
  return G_TYPE_CHECK_INSTANCE_CAST! (widget, emacs_fixed_get_type (), EmacsFixed);
}

// C 65-75: `static void emacs_fixed_class_init (EmacsFixedClass *klass)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn emacs_fixed_class_init (klass: *mut EmacsFixedClass)
{
  let widget_class: *mut GtkWidgetClass;

  widget_class = klass as *mut GtkWidgetClass;   // C 70: `(GtkWidgetClass *) klass`

  (*widget_class).get_preferred_width = Some (emacs_fixed_get_preferred_width);
  (*widget_class).get_preferred_height = Some (emacs_fixed_get_preferred_height);
  g_type_class_add_private (klass as gpointer,
			       core::mem::size_of::<EmacsFixedPrivate> () as gsize);
}

// C 77-83: `static void emacs_fixed_init (EmacsFixed *fixed)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn emacs_fixed_init (fixed: *mut EmacsFixed)
{
  (*fixed).r#priv = G_TYPE_INSTANCE_GET_PRIVATE! (fixed, emacs_fixed_get_type (),
						  EmacsFixedPrivate) as *mut c_void;
  /* C 82: `fixed->priv->f = 0;` - C's literal is kept (the `null` family of
     `docs/ports/token-aliases.tsv` is C's `NULL`, not this `0`).  */
  (*((*fixed).r#priv as *mut EmacsFixedPrivate)).f = 0 as *mut frame;
}

/* C 85-92: `GtkWidget *emacs_fixed_new (struct frame *f)`.  The one non-`static` definition above
   the arms gets PORTING.md 0.2's export (P-52): `#[unsafe(no_mangle)] pub unsafe extern "C"`.
   `struct frame` is a complete type in `include/frame.rs`, so the parameter keeps it (P-55's
   `*mut c_void` is for the tags no mirror defines).  */
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_fixed_new (f: *mut frame) -> *mut GtkWidget
{
  let fixed: *mut EmacsFixed = g_object_new (emacs_fixed_get_type (), core::ptr::null ())
			       as *mut EmacsFixed;
  let r#priv: *mut EmacsFixedPrivate = (*fixed).r#priv as *mut EmacsFixedPrivate;
  (*r#priv).f = f;
  return GTK_WIDGET! (fixed);
}

// C 94-110: `static void emacs_fixed_get_preferred_width (GtkWidget *widget, gint *minimum, gint *natural)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn emacs_fixed_get_preferred_width (widget: *mut GtkWidget, minimum: *mut gint,
						      natural: *mut gint)
{
  let fixed: *mut EmacsFixed = EMACS_FIXED (widget);
  let r#priv: *mut EmacsFixedPrivate = (*fixed).r#priv as *mut EmacsFixedPrivate;
  /* C 101-109: the two arms of `#ifdef HAVE_PGTK` - PGTK takes the frame's preferred width from
     the PGTK output record, the X11/GTK3 arm uses the X size hint for both (P-86 ①).  */
  #[cfg (feature = "have_pgtk")]
  let w: c_int = (*((*(*r#priv).f).output_data.pgtk as *mut pgtk_output)).size_hints.min_width;
  #[cfg (not (feature = "have_pgtk"))]
  let w: c_int = (*((*(*r#priv).f).output_data.x as *mut x_output)).size_hints.min_width;
  if ! minimum.is_null () { *minimum = w; }
  #[cfg (feature = "have_pgtk")]
  if ! natural.is_null ()
    { *natural = (*((*(*r#priv).f).output_data.pgtk as *mut pgtk_output)).preferred_width; }
  #[cfg (not (feature = "have_pgtk"))]
  if ! natural.is_null () { *natural = w; }
}

// C 112-128: `static void emacs_fixed_get_preferred_height (GtkWidget *widget, gint *minimum, gint *natural)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" fn emacs_fixed_get_preferred_height (widget: *mut GtkWidget, minimum: *mut gint,
						       natural: *mut gint)
{
  let fixed: *mut EmacsFixed = EMACS_FIXED (widget);
  let r#priv: *mut EmacsFixedPrivate = (*fixed).r#priv as *mut EmacsFixedPrivate;
  #[cfg (feature = "have_pgtk")]
  let h: c_int = (*((*(*r#priv).f).output_data.pgtk as *mut pgtk_output)).size_hints.min_height;
  #[cfg (not (feature = "have_pgtk"))]
  let h: c_int = (*((*(*r#priv).f).output_data.x as *mut x_output)).size_hints.min_height;
  if ! minimum.is_null () { *minimum = h; }
  #[cfg (feature = "have_pgtk")]
  if ! natural.is_null ()
    { *natural = (*((*(*r#priv).f).output_data.pgtk as *mut pgtk_output)).preferred_height; }
  #[cfg (not (feature = "have_pgtk"))]
  if ! natural.is_null () { *natural = h; }
}

/* C 131: `#ifndef HAVE_PGTK` - the X11/GTK3 arm's two exported functions, both carried with C's
   statements.  Its closing `#endif` is C 210.  */
/* C 133-135: "Override the X function so we can intercept Gtk+ 3 calls.  Use our values for
   min_width/height so that KDE don't freak out (Bug#8919), and so users can resize our frames as
   they wish."  */

// C 137-198: `void XSetWMSizeHints (Display *d, Window w, XSizeHints *hints, Atom prop)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XSetWMSizeHints (d: *mut Display, w: Window, hints: *mut XSizeHints,
					   prop: Atom)
{
  use x11_constants::PropModeReplace;    // C 196's `PropModeReplace` (X.h 475, the mirror's)

  let dpyinfo: *mut x_display_info = x_display_info_for_display (d);
  let f: *mut frame = x_top_window_to_frame (dpyinfo, w as c_int);
  /* C 145: `long data[18];` - C leaves it uninitialized and writes all eighteen members below, so
     the port starts it zeroed (PORTING.md 0.1-13's zero-value rule).  */
  let mut data: [c_long; 18] = [0; 18];
  data[0] = (*hints).flags;
  data[1] = (*hints).x as c_long;              // C: `int` -> `long` (PORTING.md 1.1)
  data[2] = (*hints).y as c_long;
  data[3] = (*hints).width as c_long;
  data[4] = (*hints).height as c_long;
  data[5] = (*hints).min_width as c_long;
  data[6] = (*hints).min_height as c_long;
  data[7] = (*hints).max_width as c_long;
  data[8] = (*hints).max_height as c_long;
  data[9] = (*hints).width_inc as c_long;
  data[10] = (*hints).height_inc as c_long;
  data[11] = (*hints).min_aspect.x as c_long;
  data[12] = (*hints).min_aspect.y as c_long;
  data[13] = (*hints).max_aspect.x as c_long;
  data[14] = (*hints).max_aspect.y as c_long;
  data[15] = (*hints).base_width as c_long;
  data[16] = (*hints).base_height as c_long;
  data[17] = (*hints).win_gravity as c_long;

  if ((*hints).flags & PMinSize) != 0 && ! f.is_null ()
    {
      /* C 167-173's comment: overriding the size hints with our own min_width/min_height used to
	 work but now only makes frames resize unpredictably and emit GTK warnings while the two
	 sides fight over the frame's size, so C respects the hints GTK set - and only makes sure
	 they are an integer multiple of the resize increments so that bug#8919 stays fixed.  The
	 two commented-out lines C 175-179 carries are kept below.  */

      /* int w = f->output_data.x->size_hints.min_width;
         int h = f->output_data.x->size_hints.min_height;

	 data[5] = w;
	 data[6] = h; */

      /* C 181-182: "Make sure min_width and min_height are multiples of width_inc and
	 height_inc."  */
      if ((*hints).flags & PResizeInc) != 0
	{
	  /* C 186-187: some GTK versions set `PResizeInc` even when the increments are at their
	     initial values.  */
	  if (*hints).width_inc != 0 && data[5] % (*hints).width_inc as c_long != 0
	    { data[5] += (*hints).width_inc as c_long - (data[5] % (*hints).width_inc as c_long); }
	  if (*hints).height_inc != 0 && data[6] % (*hints).height_inc as c_long != 0
	    { data[6] += (*hints).height_inc as c_long - (data[6] % (*hints).height_inc as c_long); }
	}
    }

  XChangeProperty (d, w, prop, XA_WM_SIZE_HINTS, 32, PropModeReplace,
		   data.as_ptr () as *const c_uchar, 18);
}

/* C 200-202: "Override this X11 function.  This function is in the same X11 file as the one
   above.  So we must provide it also."  */

// C 204-208: `void XSetWMNormalHints (Display *d, Window w, XSizeHints *hints)`
#[cfg (all (feature = "use_gtk", feature = "have_gtk3", not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XSetWMNormalHints (d: *mut Display, w: Window, hints: *mut XSizeHints)
{
  XSetWMSizeHints (d, w, hints, XA_WM_NORMAL_HINTS);
}

// C 210: `#endif` - the end of C 131's `#ifndef HAVE_PGTK` arm.
