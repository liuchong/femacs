// femacs: port of src/xsettings.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, file-scope object, local), the same control flow and the same error model.  The
// non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the `static` ones stay
// plain `unsafe fn`s.
//
// **The file gate is `any (feature = "have_x_windows", feature = "have_pgtk")`.**  No `#if` in the
// file covers it: C builds `xsettings.o` into `XOBJ` when `HAVE_X_WINDOWS` is set (`configure.ac`
// 7196's `XOBJ="xterm.o xfns.o xselect.o xrdb.o xsmfns.o xsettings.o"`) and into `PGTK_OBJ` when
// the window system is `pgtk` (`configure.ac` 3878) - the *build membership* is the gate, the
// `src/pgtkfns.rs` shape with a two-armed condition instead of one.  Both are off in the frozen
// configuration (`docs/baseline/config/darwin-arm64.macros.tsv` 1685/1489;
// `docs/baseline/frozen-build-sources.txt` lists no `xsettings.o`), so this target's `build-tu`
// object is the 336-byte empty one - while the text is carried **in full** (P-86 ①), because an X11
// or a pgtk build compiles it.  The gate goes on every item *and on the include graph* (P-86 ⑧).
//
// The conditions the text carries, both arms each, named by the lowercased macro (P-86 ①, status
// 3.171's rule): `have_pgtk` (C 29/41/221/235/246/273/445), `have_gsettings` (C 48/226/1093/1375),
// `have_gconf` (C 53/1203/1379/1411), the `use_cairo || have_xft` family
// (C 57/60/109/128/188/208/415/487/513/529/631/680/732/806/867/1028/1139/1228/1409),
// `cairo_has_ft_font`/`cairo_has_fc_font` (C 64/868), `use_lucid` (C 1303), `fc_hint_style`
// (C 748/894/935) and the two GLib version probes (C 251's `GLIB_VERSION_2_32` ->
// `glib_version_2_32`; C 1097/1206's `! GLIB_CHECK_VERSION (2, 36, 0)` ->
// `glib_version_lt_2_36`, derived by the caller from `glib_check_version_lt_2_36`, the feature
// `src/gfilenotify.rs` 297 already uses).  `have_gconf`, `cairo_has_ft_font`, `cairo_has_fc_font`,
// `fc_hint_style` and `glib_version_2_32` are names for `xtask/Cargo.toml`'s `[features]`
// table that this lane introduces and cannot register itself (`## Owed` 1).
//
// **The third-party surface stands in this unit** (P-88 ①), at the C `#include` line it belongs to:
// the Xlib/Xproto bits (`include/xlib.rs` is a slice - it carries ten Xlib entry points, not the six
// this file calls - and no mirror carries `CARD16`/`CARD32`), the three `XEvent` member records C
// reads (the `src/widget.rs` 575 `XExposeEvent` shape: the mirror's `XEvent` is an opaque 192-byte
// record, so a member is reached through a measured conversion), the GIO/GSettings half (whose home
// would be an `include/gio.rs`), the GConf half (no header on this machine at all - **UNVERIFIED**),
// the fontconfig half (`src/ftfont.rs` 95-160 declares the same surface for its own consumers, and
// `## Owed` 4 names the shared mirror both lanes want) and the cairo font-options half.  Fontconfig,
// cairo, Xlib and GIO *are* on this machine (`/opt/homebrew/include`; the measurements are in
// `docs/ports/xsettings.md`'s `## Evidence` ②), so those records' layouts are clang readings; GDK,
// GTK and GConf are not, so their declarations stay header-only and **UNVERIFIED**.
//
// C 34's `#include "xsettings.h"` has **no Rust spelling in this unit**: the mirror
// `src/include/xsettings.rs` declares the three functions this unit defines, and a
// same-named `extern "C"` declaration beside its definition is `E0428` (measured) - the P-39 shape,
// whose fix (moving the mirror's declarations into a `pub mod` with a glob import) belongs to the
// mirror and is outside this lane's write scope (`## Owed` 2).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens, clippy::all)]

use std::ffi::CStr;                            // C 193's `XSETTINGS_TOOL_BAR_STYLE` (the `src/ftfont.rs` 90 shape)

include!("include/config.rs");                  // C 20: `#include <config.h>`
/* C 22-24: `#include <float.h>`, `<limits.h>` and `<fcntl.h>`.  `<limits.h>` is where C 843's
   `LONG_MAX` comes from (`lib/include/limits.in.rs` 45); `<float.h>` and `<fcntl.h>` are named by
   nothing in this file's live or dead text (no `DBL_*`/`FLT_*`, no `open`/`O_*`), so they get the
   `src/xsmfns.rs` 383 record - named, no edge.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("../lib/include/limits.in.rs");        // C 23: `#include <limits.h>` (`LONG_MAX`, C 843)
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("../lib/include/byteswap.in.rs");      // C 26: `#include <byteswap.h>` (C 645/667/695/704)
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/lisp.rs");                    // C 28: `#include "lisp.h"`
/* **The edges below are in the materializer's order, not C's** (P-86 ⑧; `## Deviations` 3): xtask's
   dedupe keys a mirror on its *file*, and the first edge in the walk claims its one expansion slot,
   so a file this unit's own text needs in *both* arms must be claimed by an edge that both arms
   expand - before C 30/C 32's conditional chains, whose own nested edges would otherwise take the
   slot and leave the file out of the other configuration's graph.  Each comment keeps C's own line,
   and every `#[cfg]` above an `include!` is on **one line**: xtask's `attr_run` reads only the
   current line's start, so a wrapped attribute left the edge ungated (measured: the wrapped
   `all (any (…), not (…))` above C 30's edge made `xterm.rs`'s whole chain "ungated reachable").
   The mirrors a chain reaches *inside* itself (C 30's `dispextern.rs`/`termhooks.rs`/`xlib.rs`,
   C 32's `pgtkterm.rs` chain) stay with that chain.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/frame.rs");                   // C 35: `#include "frame.h"`
/* C 35's chain and the three records the mirrors below name but do not declare: `frame.h`'s
   `struct frame` (`include/xterm.rs` 228's prototype and its `*mut frame` fields),
   `dispextern.h`'s `*mut window` and `*mut font` (its 1627/1652) and `keyboard.h`'s
   `*mut buffer` (its 248/249).  Each edge is the mirror the record's own header owns; the
   `src/xsmfns.rs` 396 `include/font.rs` reading is the precedent.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/font.rs");                    // C 30's xterm.h chain: `struct font *font`
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/buffer.rs");                  // C 36's keyboard.h chain: `*mut buffer`
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/keyboard.rs");                // C 36: `#include "keyboard.h"` (`kbd_buffer_store_event`)
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/blockinput.rs");              // C 37: `#include "blockinput.h"` (`block_input`)
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
include!("include/pdumper.rs");                 // C 39: `#include "pdumper.h"` (`PDUMPER_IGNORE`)
#[cfg (any (feature = "have_gsettings", feature = "have_gconf", feature = "have_pgtk",
           feature = "use_cairo", feature = "use_gtk"))]
include!("include/gtk.rs");                     // C 49: `<glib-object.h>`'s GLib/GObject half
/* C 49's `<glib-object.h>` edge, plus C 57/64's `<cairo.h>` names (the same mirror's cairo
   section, its 236-320): `include/xterm.rs` 53's *ungated* `include!("gtk.rs")` (C 69's `#ifdef
   USE_GTK`) would otherwise claim the surface's one expansion slot - and a `use_cairo`-without-GTK
   configuration (`--with-cairo --with-x-toolkit=lucid`) reaches that mirror only through C 57's
   `<fontconfig/fontconfig.h>`; the widths are recorded in `## Owed` 4.  */
/* C 29-33: `#ifndef HAVE_PGTK / #include "xterm.h" / #else / #include "gtkutil.h" / #endif`.
   `include/gtkutil.rs`'s body is recorded rather than transcribed (its `USE_GTK` gate is false in
   the frozen configuration, P-08), and its own C 29 edge (`<gtkutil.h>` 28-29's `#include
   "pgtkterm.h"`) is owed in `docs/ports/include-gtkutil.md`, so this unit spells that edge here -
   the `src/xgselect.rs` 31 reading.  C 38's `termhooks.h` edge and C 35's `window.h` are already
   claimed above (frame.rs 24-25's own edges, live in both arms).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
include!("include/xterm.rs");                   // C 30: `#include "xterm.h"` (`Display_Info`)
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
include!("include/gtkutil.rs");                 // C 32: `#include "gtkutil.h"`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
include!("include/pgtkterm.rs");                // C 32's chain: gtkutil.h 29's `#include "pgtkterm.h"`

/* ----------------------------------------------------------------------------------------------
   C 41-46: `#ifndef HAVE_PGTK / #include <X11/Xproto.h> / #else / typedef unsigned short CARD16;
   typedef unsigned int CARD32; / #endif`.  The two CARD names are Xmd.h's (Xproto.h includes it);
   no mirror in this tree carries either, so both arms stand here (P-88 ①) with the headers' own
   lines.  Measured on this machine (`/opt/homebrew/include`, xorgproto 2025.1 + libX11 1.8.13):
   `sizeof_CARD16=2 sizeof_CARD32=4` (Evidence ②).  */
/// C: Xmd.h: `typedef unsigned short CARD16;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub type CARD16 = c_ushort;
/// C: Xmd.h: `typedef unsigned int CARD32;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub type CARD32 = c_uint;
/// C 44: `typedef unsigned short CARD16;` (the `#else` arm)
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
pub type CARD16 = c_ushort;
/// C 45: `typedef unsigned int CARD32;` (the `#else` arm)
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
pub type CARD32 = c_uint;

/* The six Xlib entry points C 552-558, 840-845, 553-554 and 851 call and `include/xlib.rs` does
   not carry yet (it declares its own ten, the `USE_CAIRO`-era slice); the header's line is quoted
   and every prototype is the header's own text.  Their home is `include/xlib.rs`
   (`## Owed` 3).  Measured on this machine (libX11 1.8.13, Evidence ②).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe extern "C" {
/// C: Xlib.h 2565-2568: `extern int XGrabServer (Display *display);`   (C 552)
  pub fn XGrabServer (display: *mut Display) -> c_int;
  /// C: Xlib.h 2571-2573: `extern int XUngrabServer (Display *display);`   (C 561)
  pub fn XUngrabServer (display: *mut Display) -> c_int;
  /// C: Xlib.h 1647-1650: `extern Window XGetSelectionOwner (Display *display, Atom selection);`
  /// (C 553-554)
  pub fn XGetSelectionOwner (display: *mut Display, selection: Atom) -> Window;
  /// C: Xlib.h 3001-3004: `extern int XSelectInput (Display *display, Window w, long event_mask);`
  /// (C 558-559, 1273)
  pub fn XSelectInput (display: *mut Display, w: Window, event_mask: c_long) -> c_int;
  /// C: Xlib.h 3020-3029: `extern int XGetWindowProperty (Display *display, Window w, Atom
  /// property, long long_offset, long long_length, Bool delete, Atom req_type, Atom *actual_type,
  /// int *actual_format, unsigned long *nitems, unsigned long *bytes_after, unsigned char
  /// **prop);`   (C 840-845)
  pub fn XGetWindowProperty (display: *mut Display, w: Window, property: Atom,
                             long_offset: c_long, long_length: c_long, delete: Bool,
                             req_type: Atom, actual_type: *mut Atom, actual_format: *mut c_int,
                             nitems: *mut c_ulong, bytes_after: *mut c_ulong,
                             prop: *mut *mut c_uchar) -> c_int;
  /// C: Xlib.h 1936-1939: `extern int XFree (void *data);`   (C 851)
  pub fn XFree (data: *mut c_void) -> c_int;
}

/* The X.h / Xlib.h constants this file names and `include/xlib.rs`'s `x11_constants` module does
   not carry: X.h's three event `type`/`state` values and Xlib.h's two byte orders.  Measured
   (Evidence ②): `PropertyNewValue=0 DestroyNotify=17 PropertyNotify=28 LSBFirst=0 MSBFirst=1`
   (X.h 132/137/140, Xlib.h 79-80).  Their home is the same module (`## Owed` 3).  */
/// C: X.h 140: `#define PropertyNewValue 0`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const PropertyNewValue: c_int = 0;
/// C: X.h 137: `#define DestroyNotify 17`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const DestroyNotify: c_int = 17;
/// C: X.h 132: `#define PropertyNotify 28`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const PropertyNotify: c_int = 28;
/// C: Xlib.h 79: `#define LSBFirst 0`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const LSBFirst: c_int = 0;
/// C: Xlib.h 80: `#define MSBFirst 1`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const MSBFirst: c_int = 1;

/* The three `XEvent` member records C 1050-1067 reads.  `include/xlib.rs` 1164 declares `XEvent`
   as an opaque 192-byte record whose 33 member records are owed in
   `docs/ports/include-xlib.md`, so C's `event->xany` / `->xclient` / `->xproperty` member is
   reached through a conversion to the record below (P-72 clause 3; the `src/widget.rs` 575
   `XExposeEvent` precedent).  Layouts measured on this machine (libX11 1.8.13, Evidence ②).  */
/// C: Xlib.h 924-936: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; } XAnyEvent;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
#[repr (C)]
pub struct XAnyEvent
{
  pub r#type: c_int,              // Xlib.h 930
  pub serial: c_ulong,            // Xlib.h 931
  pub send_event: Bool,           // Xlib.h 932
  pub display: *mut Display,      // Xlib.h 933
  pub window: Window,             // Xlib.h 934
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::size_of::<XAnyEvent> () == 40);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, r#type) == 0);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, serial) == 8);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, send_event) == 16);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, display) == 24);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, window) == 32);

/// C: Xlib.h 895-906: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Atom message_type; int format; union { char b[20]; short s[10]; long
/// l[5]; } data; } XClientMessageEvent;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XClientMessageEvent
{
  pub r#type: c_int,                      // Xlib.h 901
  pub serial: c_ulong,                    // Xlib.h 902
  pub send_event: Bool,                   // Xlib.h 903
  pub display: *mut Display,              // Xlib.h 904
  pub window: Window,                     // Xlib.h 905
  pub message_type: Atom,                 // Xlib.h 906
  pub format: c_int,                      // Xlib.h 907
  pub data: _XClientMessageEvent_data,    // Xlib.h 908-911
}
/// C: Xlib.h 908-911: `union { char b[20]; short s[10]; long l[5]; } data;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub union _XClientMessageEvent_data
{
  pub b: [c_char; 20],      // Xlib.h 909
  pub s: [c_short; 10],     // Xlib.h 910
  pub l: [c_long; 5],       // Xlib.h 911
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::size_of::<XClientMessageEvent> () == 96);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XClientMessageEvent, message_type) == 40);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XClientMessageEvent, format) == 48);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XClientMessageEvent, data) == 56);

/// C: Xlib.h 826-841: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Atom atom; Time time; int state; } XPropertyEvent;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XPropertyEvent
{
  pub r#type: c_int,              // Xlib.h 833
  pub serial: c_ulong,            // Xlib.h 834
  pub send_event: Bool,           // Xlib.h 835
  pub display: *mut Display,      // Xlib.h 836
  pub window: Window,             // Xlib.h 837
  pub atom: Atom,                 // Xlib.h 838
  pub time: Time,                 // Xlib.h 839
  pub state: c_int,               // Xlib.h 840
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::size_of::<XPropertyEvent> () == 64);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XPropertyEvent, atom) == 40);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XPropertyEvent, time) == 48);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
const _: () = core::assert! (core::mem::offset_of! (XPropertyEvent, state) == 56);

/* ----------------------------------------------------------------------------------------------
   C 47-51: `#ifdef HAVE_GSETTINGS / #include <glib-object.h> / #include <gio/gio.h> / #endif`.
   `<glib-object.h>`'s and `<gobject.h>`'s names (`GObject`, `G_OBJECT`, `G_CALLBACK`, `g_free`,
   `g_signal_connect`'s macro) live in `include/gtk.rs` (P-90 ②: the GLib/GObject surface's one
   home); `<gio/gio.h>`'s do not, so the GSettings/GVariant half stands here (P-88 ①; the
   `src/gfilenotify.rs` 37 shape for the same header).  **GLib and GIO are on this machine**
   (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0 + the generated `glibconfig.h`), so every
   prototype quotes the header and the line, and the records' sizes are clang readings
   (Evidence ②).  A dedicated `include/gio.rs` is where these belong (`## Owed` 4).  */
/* C 49's `<glib-object.h>` edge: claimed *before* C 30/C 32's chains, because `include/xterm.rs` 53
   has an ungated `include!("gtk.rs")` of its own (C 69's `#ifdef USE_GTK`) which would take the
   surface's one expansion slot - and the file's own edge is the one live in the settings and pgtk
   configurations (`## Deviations` 3).  */
#[cfg (any (feature = "have_gsettings", feature = "have_gconf", feature = "have_pgtk",
           feature = "use_gtk"))]
/// C: gio/gsettings.h 60: `typedef struct _GSettings GSettings;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub struct _GSettings { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub type GSettings = _GSettings;
/// C: gio/gsettingsschema.h 33: `typedef struct _GSettingsSchema GSettingsSchema;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub struct _GSettingsSchema { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub type GSettingsSchema = _GSettingsSchema;
/// C: gio/gsettingsschema.h 35: `typedef struct _GSettingsSchemaSource GSettingsSchemaSource;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub struct _GSettingsSchemaSource { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub type GSettingsSchemaSource = _GSettingsSchemaSource;
/// C: glib/gvariant.h 31: `typedef struct _GVariant GVariant;` - the record C 287/294's
/// `g_settings_get_value` returns (opaque; the port never allocates or sizes one).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub struct _GVariant { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub type GVariant = _GVariant;
/// C: glib/gvarianttype.h 34: `typedef struct _GVariantType GVariantType;` - the type
/// `G_VARIANT_TYPE_STRING` below points at.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub struct _GVariantType { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub type GVariantType = _GVariantType;
/// C: glib/gvarianttype.h 122: `#define G_VARIANT_TYPE_STRING ((const GVariantType *) "s")` -
/// measured: the macro's bytes are `"s"` (Evidence ②).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
pub const G_VARIANT_TYPE_STRING: *const GVariantType = c"s".as_ptr () as *const GVariantType;
/// C: gobject/gtype.h 737: `void g_type_init (void);` - deprecated since 2.36, so C 1098/1207 take
/// the `! GLIB_CHECK_VERSION (2, 36, 0)` arm.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_gsettings"))]
unsafe extern "C" {
  pub fn g_type_init ();
  /* C 255-257's `g_object_get` is `include/gtk.rs` 186's (C: gobject/gobject.h 459-462); the
     mirror's declaration is the one this unit calls (P-26's one spelling per construct).  */
  /// C: gobject/gobject.h 509-510: `gpointer g_object_ref_sink (gpointer object);` (C 1125)
  pub fn g_object_ref_sink (object: gpointer) -> gpointer;
  /// C: gio/gsettings.h 80: `GSettings *g_settings_new (const gchar *schema_id);` (C 1123)
  pub fn g_settings_new (schema_id: *const gchar) -> *mut GSettings;
  /// C: gio/gsettings.h 76: `const gchar * const *g_settings_list_schemas (void);` (C 1115)
  pub fn g_settings_list_schemas () -> *const *const gchar;
  /// C: gio/gsettings.h 112-114: `GVariant *g_settings_get_value (GSettings *settings, const
  /// gchar *key);` (C 293/330/363/403/418/432/465/1129/1140/1149/1180)
  pub fn g_settings_get_value (settings: *mut GSettings, key: *const gchar) -> *mut GVariant;
  /// C: gio/gsettingsschema.h 38: `GSettingsSchemaSource *g_settings_schema_source_get_default
  /// (void);` (C 1103-1104)
  pub fn g_settings_schema_source_get_default () -> *mut GSettingsSchemaSource;
  /// C: gio/gsettingsschema.h 51-54: `GSettingsSchema *g_settings_schema_source_lookup
  /// (GSettingsSchemaSource *source, const gchar *schema_id, gboolean recursive);` (C 1107-1108)
  pub fn g_settings_schema_source_lookup (source: *mut GSettingsSchemaSource,
                                          schema_id: *const gchar,
                                          recursive: gboolean) -> *mut GSettingsSchema;
  /// C: gio/gsettingsschema.h 68: `void g_settings_schema_unref (GSettingsSchema *schema);`
  /// (C 263, 1110)
  pub fn g_settings_schema_unref (schema: *mut GSettingsSchema);
  /// C: gio/gsettingsschema.h 78-80: `gboolean g_settings_schema_has_key (GSettingsSchema
  /// *schema, const gchar *key);` (C 262)
  pub fn g_settings_schema_has_key (schema: *mut GSettingsSchema, key: *const gchar) -> gboolean;
  /// C: glib/gvariant.h 61: `void g_variant_unref (GVariant *value);` (C 316/349/387/412/427/441/476/1136/1146/1155/1188/1191)
  pub fn g_variant_unref (value: *mut GVariant);
  /// C: glib/gvariant.h 65: `GVariant *g_variant_ref_sink (GVariant *value);` (C 297/334/368/406/421/435/468/1132/1143/1152/1183)
  pub fn g_variant_ref_sink (value: *mut GVariant) -> *mut GVariant;
  /// C: glib/gvariant.h 76-78: `gboolean g_variant_is_of_type (GVariant *value, const GVariantType
  /// *type);` (C 299/335/370/407/422/436/469/1133/1144/1153/1184)
  pub fn g_variant_is_of_type (value: *mut GVariant, r#type: *const GVariantType) -> gboolean;
  /// C: glib/gvariant.h 158-160: `const gchar *g_variant_get_string (GVariant *value, gsize
  /// *length);` (C 301/337/372/409/424/438/471/1135/1145/1154/1186)
  pub fn g_variant_get_string (value: *mut GVariant, length: *mut gsize) -> *const gchar;
}

/* ----------------------------------------------------------------------------------------------
   C 52-55: `#ifdef HAVE_GCONF / #include <gconf/gconf-client.h> / #endif`.  **No GConf header is on
   this machine** (`ls -d /opt/homebrew/include/gconf` -> absent), so every declaration below is
   written from the library's published interface and is **UNVERIFIED** (P-88 ②/P-90 ④) - the
   segment's reason is `## Deviations` 8, and a GConf toolchain is what can drop the mark.  */
/// C: gconf/gconf-client.h: `typedef struct _GConfClient GConfClient;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub struct _GConfClient { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub type GConfClient = _GConfClient;
/// C: gconf/gconf-value.h: `typedef struct _GConfEntry GConfEntry;` - opaque; only
/// `gconf_entry_get_value`/`_get_key` below read it.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub struct _GConfEntry { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub type GConfEntry = _GConfEntry;
/// C: gconf/gconf-value.h: `typedef enum { GCONF_VALUE_INVALID, GCONF_VALUE_STRING, GCONF_VALUE_INT,
/// GCONF_VALUE_FLOAT, GCONF_VALUE_BOOL, GCONF_VALUE_SCHEMA, GCONF_VALUE_LIST, GCONF_VALUE_PAIR }
/// GConfValueType;`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub type GConfValueType = c_int;
/// C: gconf/gconf-value.h's enum member - **UNVERIFIED**
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub const GCONF_VALUE_STRING: GConfValueType = 1;
/// C: gconf/gconf-value.h: `struct _GConfValue { GConfValueType type; GConfValueTypeList list_type;
/// union { gchar *str; gint int_value; … } value; };` - **UNVERIFIED**; the port states only the
/// member C 507 reads (the union tail is not spelled, and nothing here sizes the record).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
#[repr (C)]
pub struct GConfValue
{
  pub r#type: GConfValueType,     // C: GConfValueType type;
}
/// C: gconf/gconf-client.h: `#define GCONF_CLIENT_HANDLE_NONE 0` - **UNVERIFIED**
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub const GCONF_CLIENT_HANDLE_NONE: c_int = 0;
/// C: gconf/gconf-client.h: `#define GCONF_CLIENT_PRELOAD_ONELEVEL 2` - **UNVERIFIED**
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub const GCONF_CLIENT_PRELOAD_ONELEVEL: c_int = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
unsafe extern "C" {
/// C: gconf/gconf-client.h and gconf/gconf-value.h: the eight entry points C 504-506, 510, 1210-1256
/// call - **UNVERIFIED**
  pub fn gconf_client_get_default () -> *mut GConfClient;
  pub fn gconf_client_set_error_handling (client: *mut GConfClient, err_handling: c_int);
  pub fn gconf_client_add_dir (client: *mut GConfClient, dir: *const c_char,
                               preload: c_int, err: *mut c_void);
  pub fn gconf_client_notify_add (client: *mut GConfClient, dir: *const c_char,
                                  func: Option<unsafe extern "C" fn (*mut GConfClient, guint,
                                                                     *mut GConfEntry,
                                                                     gpointer)>,
                                  user_data: gpointer, destroy: *mut c_void, err: *mut c_void);
  pub fn gconf_client_get_string (client: *mut GConfClient, key: *const c_char,
                                  err: *mut c_void) -> *mut c_char;
  pub fn gconf_entry_get_value (entry: *mut GConfEntry) -> *mut GConfValue;
  pub fn gconf_entry_get_key (entry: *mut GConfEntry) -> *const c_char;
  pub fn gconf_value_get_string (value: *mut GConfValue) -> *const c_char;
}

/* ----------------------------------------------------------------------------------------------
   C 56-62: `#ifdef USE_CAIRO / #include <fontconfig/fontconfig.h> / #include "ftfont.h" / #elif
   defined HAVE_XFT / #include <X11/Xft/Xft.h> / #endif`.  Fontconfig is on this machine
   (`/opt/homebrew/include/fontconfig`, 2.18.3), so every record, prototype and value below is a
   header reading with its line (Evidence ②); **Xft is not** (`/opt/homebrew/include/X11/Xft/Xft.h`
   is absent), so its two entry points stay header-only and **UNVERIFIED** (`## Deviations` 9).
   `src/ftfont.rs` 95-160 declares the same fontconfig surface for its own consumers; both move into
   one `include/fontconfig.rs` when it lands (`## Owed` 4).  */
/// C: fontconfig.h 48: `typedef int FcBool;` - measured `sizeof_FcBool=4` (Evidence ②).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub type FcBool = c_int;
/// C: fontconfig.h 47/49: `typedef unsigned char FcChar8;` / `typedef unsigned int FcChar32;`
/// (measured 1 and 4).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub type FcChar8 = c_uchar;
/// C: fontconfig.h 531: `typedef struct _FcPattern FcPattern;` - opaque in the port's use (only
/// pointers are made, passed and destroyed).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub struct _FcPattern { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub type FcPattern = _FcPattern;
/// C: fontconfig.h 40: `typedef int FcResult;` - `FcPatternGetInteger`'s return type; C 892-899
/// discards it.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub type FcResult = c_int;
/// C: fontconfig.h 98/99/100/116/117/132: `#define FC_ANTIALIAS "antialias"` / `FC_HINTING
/// "hinting"` / `FC_HINT_STYLE "hintstyle"` / `FC_DPI "dpi"` / `FC_RGBA "rgba"` /
/// `FC_LCD_FILTER "lcdfilter"` - measured bytes (Evidence ②).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_ANTIALIAS: *const c_char = c"antialias".as_ptr ();
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINTING: *const c_char = c"hinting".as_ptr ();
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINT_STYLE: *const c_char = c"hintstyle".as_ptr ();
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_DPI: *const c_char = c"dpi".as_ptr ();
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA: *const c_char = c"rgba".as_ptr ();
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_LCD_FILTER: *const c_char = c"lcdfilter".as_ptr ();
/// C: fontconfig.h 208-211: `#define FC_HINT_NONE 0` / `FC_HINT_SLIGHT 1` / `FC_HINT_MEDIUM 2` /
/// `FC_HINT_FULL 3` (measured 0/1/2/3).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINT_NONE: c_int = 0;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINT_SLIGHT: c_int = 1;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINT_MEDIUM: c_int = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_HINT_FULL: c_int = 3;
/// C: fontconfig.h 201-205: `#define FC_RGBA_RGB 1` / `FC_RGBA_BGR 2` / `FC_RGBA_VRGB 3` /
/// `FC_RGBA_VBGR 4` / `FC_RGBA_NONE 5` (measured 1/2/3/4/5).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA_RGB: c_int = 1;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA_BGR: c_int = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA_VRGB: c_int = 3;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA_VBGR: c_int = 4;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_RGBA_NONE: c_int = 5;
/* C 531-542: `#ifndef FC_LCD_NONE / #define FC_LCD_NONE 0 / #endif`, and the same pair for
   `FC_LCD_DEFAULT` (1) and `FC_LCD_FILTER` ("lcdfilter").  The header has all three, so the three
   `#ifndef`s are no-ops and the port declares each name once - the `src/ftfont.rs` 2793 reading of
   the same two `#ifndef`s.  Measured 0/1/"lcdfilter" (Evidence ②).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_LCD_NONE: c_int = 0;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FC_LCD_DEFAULT: c_int = 1;
/// C: fontconfig.h 1034: the eleven entry points C 879-1002 calls.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
unsafe extern "C" {
  /// C: fontconfig.h 941: `FcPattern *FcPatternCreate (void);` (C 879)
  pub fn FcPatternCreate () -> *mut FcPattern;
  /// C: fontconfig.h 962: `void FcPatternDestroy (FcPattern *p);` (C 990, 1002)
  pub fn FcPatternDestroy (p: *mut FcPattern);
  /// C: fontconfig.h 989: `void FcPatternDel (FcPattern *p, const char *object);` (C 903/912/919/929/939/972)
  pub fn FcPatternDel (p: *mut FcPattern, object: *const c_char);
  /// C: fontconfig.h 1010-1011: `FcBool FcPatternAddBool (FcPattern *p, const char *object, FcBool
  /// b);` (C 904/913)
  pub fn FcPatternAddBool (p: *mut FcPattern, object: *const c_char, b: FcBool) -> FcBool;
  /// C: fontconfig.h 1034-1035: `FcBool FcPatternGetBool (const FcPattern *p, const char *object,
  /// int n, FcBool *b);` (C 892/893)
  pub fn FcPatternGetBool (p: *const FcPattern, object: *const c_char, n: c_int,
                           b: *mut FcBool) -> FcResult;
  /// C: fontconfig.h 995: `FcBool FcPatternAddInteger (FcPattern *p, const char *object, int i);`
  /// (C 920/930/940)
  pub fn FcPatternAddInteger (p: *mut FcPattern, object: *const c_char, i: c_int) -> FcBool;
  /// C: fontconfig.h 1019-1020: `FcBool FcPatternGetInteger (const FcPattern *p, const char
  /// *object, int n, int *i);` (C 895/897/898)
  pub fn FcPatternGetInteger (p: *const FcPattern, object: *const c_char, n: c_int,
                              i: *mut c_int) -> FcResult;
  /// C: fontconfig.h 998: `FcBool FcPatternAddDouble (FcPattern *p, const char *object, double d);`
  /// (C 973)
  pub fn FcPatternAddDouble (p: *mut FcPattern, object: *const c_char, d: c_double) -> FcBool;
  /// C: fontconfig.h 1022-1023: `FcBool FcPatternGetDouble (const FcPattern *p, const char
  /// *object, int n, double *d);` (C 899)
  pub fn FcPatternGetDouble (p: *const FcPattern, object: *const c_char, n: c_int,
                             d: *mut c_double) -> FcResult;
  /// C: fontconfig.h 544-546: `FcBool FcConfigSubstitute (FcConfig *config, FcPattern *p,
  /// FcMatchKind kind);` (C 885)
  pub fn FcConfigSubstitute (config: *mut c_void, p: *mut FcPattern, kind: c_int) -> FcBool;
  /// C: fontconfig.h 668: `void FcDefaultSubstitute (FcPattern *pattern);` (C 890)
  pub fn FcDefaultSubstitute (pattern: *mut FcPattern);
}
/// C: ftfont.h 89-90: `extern void ftcrfont_get_default_font_options (struct x_display_info *,
/// cairo_font_options_t *);` - the Cairo arm's one entry point (C 887).  `include/ftfont.rs` 71
/// records it as a comment rather than a declaration (there is no Rust item for it there), so the
/// unit spells it at the use site (P-88 ①; `## Owed` 3).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "use_cairo", feature = "cairo_has_ft_font", feature = "cairo_has_fc_font",
           not (feature = "have_xft")))]
unsafe extern "C" {
  pub fn ftcrfont_get_default_font_options (dpyinfo: *mut c_void,
                                            options: *mut cairo_font_options_t);
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_xft"))]
unsafe extern "C" {
/// C: Xft.h: `extern void XftDefaultSubstitute (Display *dpy, int screen, FcPattern *pattern);` -
/// **UNVERIFIED** (no Xft header on this machine).
  pub fn XftDefaultSubstitute (dpy: *mut Display, screen: c_int, pattern: *mut FcPattern);
  /// C: Xft.h: `extern void XftDefaultSet (Display *dpy, FcPattern *defaults);` - **UNVERIFIED**
  /// (C 988).
  pub fn XftDefaultSet (dpy: *mut Display, defaults: *mut FcPattern);
}

/* ----------------------------------------------------------------------------------------------
   C 63-66: `#if defined USE_CAIRO && defined CAIRO_HAS_FT_FONT / #include <cairo/cairo-ft.h> /
   #endif`.  `<cairo.h>` is on this machine (`/opt/homebrew/include/cairo`, cairo 1.18.4), so the
   record, the six entry points and the three enums below are header readings with their lines
   (Evidence ②).  The font-options surface is reachable from the pgtk arm too (`gtkutil.h`'s
   `<gtk/gtk.h>` pulls `<cairo.h>` in), which is why it carries `use_cairo` *or* `have_pgtk`.
   `include/gtk.rs` 240-320 declares cairo's *other* half (surfaces, patterns, formats) for the
   pixbuf helpers; the two halves move into one `include/cairo.rs` when it lands (`## Owed` 4).  */
/// C: cairo.h 1462: `typedef struct _cairo_font_options cairo_font_options_t;` - opaque; only
/// pointers are made, copied and destroyed.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub struct _cairo_font_options { _private: [u8; 0] }
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub type cairo_font_options_t = _cairo_font_options;
/// C: cairo.h 750-765: `typedef enum _cairo_antialias { CAIRO_ANTIALIAS_DEFAULT,
/// CAIRO_ANTIALIAS_NONE, CAIRO_ANTIALIAS_GRAY, CAIRO_ANTIALIAS_SUBPIXEL, … } cairo_antialias_t;` -
/// measured `NONE=1 GRAY=2 SUBPIXEL=3` (Evidence ②).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub type cairo_antialias_t = c_int;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_ANTIALIAS_NONE: cairo_antialias_t = 1;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_ANTIALIAS_GRAY: cairo_antialias_t = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_ANTIALIAS_SUBPIXEL: cairo_antialias_t = 3;
/// C: cairo.h 1364-1394: `typedef enum _cairo_hint_style { CAIRO_HINT_STYLE_DEFAULT,
/// CAIRO_HINT_STYLE_NONE, CAIRO_HINT_STYLE_SLIGHT, CAIRO_HINT_STYLE_MEDIUM,
/// CAIRO_HINT_STYLE_FULL } cairo_hint_style_t;` - measured `NONE=1 SLIGHT=2 MEDIUM=3 FULL=4`.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub type cairo_hint_style_t = c_int;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_HINT_STYLE_NONE: cairo_hint_style_t = 1;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_HINT_STYLE_SLIGHT: cairo_hint_style_t = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_HINT_STYLE_MEDIUM: cairo_hint_style_t = 3;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_HINT_STYLE_FULL: cairo_hint_style_t = 4;
/// C: cairo.h 1349-1362: `typedef enum _cairo_subpixel_order { CAIRO_SUBPIXEL_ORDER_DEFAULT,
/// CAIRO_SUBPIXEL_ORDER_RGB, CAIRO_SUBPIXEL_ORDER_BGR, CAIRO_SUBPIXEL_ORDER_VRGB,
/// CAIRO_SUBPIXEL_ORDER_VBGR } cairo_subpixel_order_t;` - measured `RGB=1 BGR=2 VRGB=3 VBGR=4`.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub type cairo_subpixel_order_t = c_int;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_SUBPIXEL_ORDER_RGB: cairo_subpixel_order_t = 1;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_SUBPIXEL_ORDER_BGR: cairo_subpixel_order_t = 2;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_SUBPIXEL_ORDER_VRGB: cairo_subpixel_order_t = 3;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
pub const CAIRO_SUBPIXEL_ORDER_VBGR: cairo_subpixel_order_t = 4;
/// C: cairo.h and cairo-ft.h: the seven entry points C 886-889, 1161-1164 and 1321-1325 call.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_pgtk")))]
unsafe extern "C" {
  /// C: cairo.h 1465: `cairo_font_options_t *cairo_font_options_create (void);` (C 886, 1161, 1325)
  pub fn cairo_font_options_create () -> *mut cairo_font_options_t;
  /// C: cairo.h 1471: `void cairo_font_options_destroy (cairo_font_options_t *options);` (C 889)
  pub fn cairo_font_options_destroy (options: *mut cairo_font_options_t);
  /// C: cairo.h 1468: `cairo_font_options_t *cairo_font_options_copy (const cairo_font_options_t
  /// *original);` (C 1322)
  pub fn cairo_font_options_copy (original: *const cairo_font_options_t) -> *mut cairo_font_options_t;
  /// C: cairo.h 1487-1488: `void cairo_font_options_set_antialias (cairo_font_options_t *options,
  /// cairo_antialias_t antialias);` (C 340/343/346)
  pub fn cairo_font_options_set_antialias (options: *mut cairo_font_options_t,
                                           antialias: cairo_antialias_t);
  /// C: cairo.h 1493-1495: `void cairo_font_options_set_subpixel_order (cairo_font_options_t
  /// *options, cairo_subpixel_order_t subpixel_order);` (C 375/378/381/384)
  pub fn cairo_font_options_set_subpixel_order (options: *mut cairo_font_options_t,
                                                subpixel_order: cairo_subpixel_order_t);
  /// C: cairo.h 1499-1500: `void cairo_font_options_set_hint_style (cairo_font_options_t *options,
  /// cairo_hint_style_t hint_style);` (C 304/307/310/313)
  pub fn cairo_font_options_set_hint_style (options: *mut cairo_font_options_t,
                                            hint_style: cairo_hint_style_t);
  /// C: cairo-ft.h 107-108: `void cairo_ft_font_options_substitute (const cairo_font_options_t
  /// *options, FcPattern *pattern);` (C 888)
  pub fn cairo_ft_font_options_substitute (options: *const cairo_font_options_t,
                                           pattern: *mut FcPattern);
}

/* C 30 and C 32's chains: `Display_Info` is dispextern.h 115/152's
   `typedef struct <platform>_display_info Display_Info;`.  The mirror `include/dispextern.rs` 698
   carries an alias of that name for the X arm, but it points at *its own module's opaque stand-in*
   for C 115's tag (`include/dispextern.rs` 2757-2758's `pub enum x_display_info {}`), which is what
   the header's own declarations need and not the record this unit dereferences; its pgtk arm (711)
   is a recorded comment with no alias at all.  A unit that needs the complete record declares its
   own (the `src/frame.rs` 6059-6076 precedent), and because C's two arms are two different records
   the port keeps one name for both through a module - the P-21/P-39 shape (PORTING.md 0.1-10).  The
   `src/frame.rs` 6059 unit spells its six aliases at the top level because it does not include
   `dispextern.h`; this unit does, through C 30's `xterm.h`, so a top-level alias would be `E0428`
   against the mirror's (measured, Evidence ③).  */
pub mod xsettings_dpi {
  /// C: dispextern.h 115: `typedef struct x_display_info Display_Info;`
  #[cfg (feature = "have_x_windows")]
  pub type Display_Info = super::x_display_info;
  /// C: dispextern.h 152: `typedef struct pgtk_display_info Display_Info;`
  #[cfg (feature = "have_pgtk")]
  pub type Display_Info = super::pgtk_display_info;
}

/* C 68-71: the four file-scope objects, all `static` (file linkage) and all initialised in
   `syms_of_xsettings` (C 1369-1374), which is why the port declares them without an initialiser -
   the `src/xsmfns.rs` 44-67 shape.  */
/* C 68: `static char *current_mono_font;`  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
static mut current_mono_font: *mut c_char = core::ptr::null_mut ();
/* C 69: `static char *current_font;`  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
static mut current_font: *mut c_char = core::ptr::null_mut ();
/* C 70: `static Display_Info *first_dpyinfo;`  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
static mut first_dpyinfo: *mut xsettings_dpi::Display_Info = core::ptr::null_mut ();
/* C 71: `static Lisp_Object current_tool_bar_style;`  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
static mut current_tool_bar_style: Lisp_Object = lisp_h_Qnil ();

/* termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
   `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80), so C 80's `event.kind =
   CONFIG_CHANGED_EVENT;` keeps C's field name in a per-unit macro - the `src/inotify.rs` 285-293
   shape, body-identical to `src/xsmfns.rs` 455-463 and `src/kqueue.rs` 156-165.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/* C 73-105: `#if defined HAVE_GSETTINGS || defined HAVE_GCONF || !defined HAVE_PGTK` ... `#endif`.  */
/* C 75-87: `static void store_config_changed_event (Lisp_Object arg, Lisp_Object display_name)` -
   "Store a config changed event in to the event queue."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "have_gsettings", feature = "have_gconf", not (feature = "have_pgtk"))))]
unsafe fn store_config_changed_event (arg: Lisp_Object, display_name: Lisp_Object)
{
  /* C's `struct input_event event; EVENT_INIT (event);` - C's own compound literal zero-fills the
     record (the `src/dbusbind.rs` 2528 shape).  */
  let mut event: input_event = core::mem::zeroed ();
  EVENT_INIT (&raw mut event);                                  // C 79
  input_event_bits! (kind, &raw mut event, CONFIG_CHANGED_EVENT);   // C 80
  event.frame_or_window = display_name;                         // C 81
  event.arg = arg;                                             // C 82
  kbd_buffer_store_event (&raw mut event as *mut c_void);       // C 83
}

/* C 89-105: "Return true if DPYINFO is still valid."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "have_gsettings", feature = "have_gconf", not (feature = "have_pgtk"))))]
unsafe fn dpyinfo_valid (dpyinfo: *mut xsettings_dpi::Display_Info) -> bool
{
  let mut found = false;
  if !dpyinfo.is_null ()                                       // C 92
    {
      /* C 95-101's `for (d = x_display_list; !found && d; d = d->next)`: the head, the test and
         the step, in C's order.  */
      let mut d: *mut xsettings_dpi::Display_Info = x_display_list;
      while !found && !d.is_null ()
        {
          #[cfg (not (feature = "have_pgtk"))]
          { found = d == dpyinfo && (*d).display == (*dpyinfo).display; }   // C 98
          #[cfg (feature = "have_pgtk")]
          { found = d == dpyinfo && (*d).gdpy == (*dpyinfo).gdpy; }         // C 100
          d = (*d).next;
        }
    }
  found                                                        // C 104
}

/* C 108-124: `#if (defined USE_CAIRO || defined HAVE_XFT) && (defined HAVE_GSETTINGS || defined
   HAVE_GCONF)`, "Store a monospace font change event if the monospaced font changed."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft"),
           any (feature = "have_gsettings", feature = "have_gconf")))]
unsafe fn store_monospaced_changed (newfont: *const c_char)
{
  if !current_mono_font.is_null () && strcmp (newfont, current_mono_font) == 0   // C 113
    { return; }                                                 /* No change. */
  dupstring (&raw mut current_mono_font, newfont);              // C 116
  if dpyinfo_valid (first_dpyinfo) && use_system_font           // C 118
    {
      store_config_changed_event (Qmonospace_font_name (),       // C 120-121
                                  XCAR ((*first_dpyinfo).name_list_element));
    }
}

/* C 127-143: `#if (defined USE_CAIRO || defined HAVE_XFT) && (defined HAVE_GSETTINGS || defined
   HAVE_GCONF || !defined HAVE_PGTK)`, "Store a font name change event if the font name changed."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft"),
           any (feature = "have_gsettings", feature = "have_gconf", not (feature = "have_pgtk"))))]
unsafe fn store_font_name_changed (newfont: *const c_char)
{
  if !current_font.is_null () && strcmp (newfont, current_font) == 0     // C 132
    { return; }                                                 /* No change. */
  dupstring (&raw mut current_font, newfont);                   // C 135
  if dpyinfo_valid (first_dpyinfo)                              // C 137
    {
      store_config_changed_event (Qfont_name (),                 // C 139-140
                                  XCAR ((*first_dpyinfo).name_list_element));
    }
}

/* C 145-185: the same `#if` region: `map_tool_bar_style` and `store_tool_bar_style_changed`.  */
/* C 145-167: "Map TOOL_BAR_STYLE from a string to its corresponding Lisp value.  Return Qnil if
   TOOL_BAR_STYLE is not known."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "have_gsettings", feature = "have_gconf", not (feature = "have_pgtk"))))]
unsafe fn map_tool_bar_style (tool_bar_style: *const c_char) -> Lisp_Object
{
  let mut style = Qnil ();                                      // C 149
  if !tool_bar_style.is_null ()                                 // C 150
    {
      if strcmp (tool_bar_style, c"both".as_ptr ()) == 0        // C 152
        { style = Qboth (); }
      else if strcmp (tool_bar_style, c"both-horiz".as_ptr ()) == 0   // C 154
        { style = Qboth_horiz (); }
      else if strcmp (tool_bar_style, c"icons".as_ptr ()) == 0  // C 156
        { style = Qimage (); }
      else if strcmp (tool_bar_style, c"text".as_ptr ()) == 0   // C 158
        { style = Qtext (); }
    }
  style                                                         // C 162
}

/* C 169-185: "Store a tool bar style change event if the tool bar style changed."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "have_gsettings", feature = "have_gconf", not (feature = "have_pgtk"))))]
unsafe fn store_tool_bar_style_changed (newstyle: *const c_char, dpyinfo: *mut xsettings_dpi::Display_Info)
{
  let style = map_tool_bar_style (newstyle);                    // C 174
  if EQ (current_tool_bar_style, style)                         // C 175
    { return; }                                                 /* No change. */
  current_tool_bar_style = style;                               // C 178
  if dpyinfo_valid (dpyinfo)                                    // C 179
    {
      store_config_changed_event (Qtool_bar_style (),            // C 181-182
                                  XCAR ((*dpyinfo).name_list_element));
    }
}

/* C 187-194: `#ifndef HAVE_PGTK` ... `#endif`: the three XSettings property names (the first three
   only when `USE_CAIRO || HAVE_XFT`).  C's `#define`s are string macros, so the port spells them as
   the constants C's text stands for (the `src/ftfont.rs` 1156-1235 shape).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const XSETTINGS_FONT_NAME: *const c_char = c"Gtk/FontName".as_ptr ();              // C 189
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const XSETTINGS_GDK_DPI_NAME: *const c_char = c"Gdk/UnscaledDPI".as_ptr ();        // C 190
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const XSETTINGS_GDK_WSCALE_NAME: *const c_char = c"Gdk/WindowScalingFactor".as_ptr ();  // C 191
/// C 193: `#define XSETTINGS_TOOL_BAR_STYLE  "Gtk/ToolbarStyle"` - the one macro C *defines* in
/// this file's live text, so `extract-abi.py`'s constants rule compares its bytes: the port keeps
/// it as the literal itself (`&CStr`) and `.as_ptr ()` at the two call sites (the
/// `include/ftfont.rs` 1194 `FC_HINT_STYLE` shape).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
pub const XSETTINGS_TOOL_BAR_STYLE: &CStr = c"Gtk/ToolbarStyle";     // C 193

/* C 196-205: the anonymous `enum { SEEN_AA = 0x01, … SEEN_TB_STYLE = 0x80 }`.  C's enum ABI is
   `c_int`, but every reader is `settings->seen` (`unsigned`), so the port types the members with the
   field's type (PORTING.md 1.3; the `src/xterm.rs` 550-556 shape).  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_AA: c_uint = 0x01;               // C 197
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_HINTING: c_uint = 0x02;          // C 198
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_RGBA: c_uint = 0x04;             // C 199
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_LCDFILTER: c_uint = 0x08;        // C 200
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_HINTSTYLE: c_uint = 0x10;        // C 201
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_DPI: c_uint = 0x20;              // C 202
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_FONT: c_uint = 0x40;             // C 203
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
pub const SEEN_TB_STYLE: c_uint = 0x80;         // C 204

/* C 206-219: `struct xsettings`, both arms of its own `#if defined USE_CAIRO || defined HAVE_XFT`
   (C 208-214).  Layout assertions are written for the frozen arm (C's `char *tb_style; unsigned
   seen;`) and for the cairo arm's own field sequence.  */
/// C: struct xsettings { #if USE_CAIRO || HAVE_XFT FcBool aa, hinting; int rgba, lcdfilter,
/// hintstyle; double dpi; char *font; #endif char *tb_style; unsigned seen; };
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[repr (C)]
pub struct xsettings
{
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub aa: FcBool,                       // C 209: `FcBool aa, hinting;`
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub hinting: FcBool,
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub rgba: c_int,                      // C 210: `int rgba, lcdfilter, hintstyle;`
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub lcdfilter: c_int,
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub hintstyle: c_int,
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub dpi: c_double,                    // C 211: `double dpi;`
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  pub font: *mut c_char,                // C 213: `char *font;`
  pub tb_style: *mut c_char,            // C 216: `char *tb_style;`
  pub seen: c_uint,                     // C 218: `unsigned seen;`
}
/* The frozen arm: `char *tb_style; unsigned seen;` - measured offsets 0 and 8, size 16.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           not (any (feature = "use_cairo", feature = "have_xft"))))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, tb_style) == 0);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           not (any (feature = "use_cairo", feature = "have_xft"))))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, seen) == 8);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           not (any (feature = "use_cairo", feature = "have_xft"))))]
const _: () = core::assert! (core::mem::size_of::<xsettings> () == 16);
/* The `USE_CAIRO || HAVE_XFT` arm: the seven members of C 208-214 then `tb_style` and `seen`
   (`FcBool` is 4 bytes, `double` is 8 - measured, Evidence ②).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, dpi) == 24);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, font) == 32);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, tb_style) == 40);
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
const _: () = core::assert! (core::mem::offset_of! (xsettings, seen) == 48);

/* C 221-224: `#ifdef HAVE_PGTK` - "The cairo font_options as obtained using gsettings."  */
/* C 223: `static cairo_font_options_t *font_options;`  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
static mut font_options: *mut cairo_font_options_t = core::ptr::null_mut ();

/* C 226-240: `#ifdef HAVE_GSETTINGS` - the schema and key names.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings"))]
pub const GSETTINGS_SCHEMA: *const c_char = c"org.gnome.desktop.interface".as_ptr ();   // C 227
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings"))]
pub const GSETTINGS_TOOL_BAR_STYLE: *const c_char = c"toolbar-style".as_ptr ();         // C 228
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           any (feature = "use_cairo", feature = "have_xft")))]
pub const GSETTINGS_MONO_FONT: *const c_char = c"monospace-font-name".as_ptr ();        // C 231
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           any (feature = "use_cairo", feature = "have_xft")))]
pub const GSETTINGS_FONT_NAME: *const c_char = c"font-name".as_ptr ();                  // C 232
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           feature = "have_pgtk"))]
pub const GSETTINGS_FONT_ANTIALIASING: *const c_char = c"font-antialiasing".as_ptr ();  // C 236
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           feature = "have_pgtk"))]
pub const GSETTINGS_FONT_RGBA_ORDER: *const c_char = c"font-rgba-order".as_ptr ();      // C 237
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           feature = "have_pgtk"))]
pub const GSETTINGS_FONT_HINTING: *const c_char = c"font-hinting".as_ptr ();            // C 238
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           feature = "have_pgtk"))]
pub const GSETTINGS_COLOR_SCHEME: *const c_char = c"color-scheme".as_ptr ();            // C 239
/* C 242-244: "The single GSettings instance, or NULL if not connected to GSettings."  */
/* C 244: `static GSettings *gsettings_client;`  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings"))]
static mut gsettings_client: *mut GSettings = core::ptr::null_mut ();

/* C 246-271: `#ifdef HAVE_PGTK` - `xg_settings_key_valid_p`.  */
/// C 246-271: `static bool xg_settings_key_valid_p (GSettings *settings, const char *key)`.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings",
           feature = "have_pgtk"))]
unsafe fn xg_settings_key_valid_p (settings: *mut GSettings, key: *const c_char) -> bool
{
  /* C 251-268: `#ifdef GLIB_VERSION_2_32` - both arms carried (P-51); the feature names the
     non-frozen side, and the frozen path is C's own `return false`.  */
  #[cfg (feature = "glib_version_2_32")]
  {
    /* C 252-265: `GSettingsSchema *schema; bool rc;` ...  */
    let mut schema: *mut GSettingsSchema = core::ptr::null_mut ();   /* PORTING.md 1.5 */
    let rc: bool;

    g_object_get (G_OBJECT! (settings) as gpointer, c"settings-schema".as_ptr (),   // C 255-257
                  &raw mut schema,
                  core::ptr::null_mut::<c_void> ());
    if schema.is_null ()                                          // C 259
      { return false; }
    rc = g_settings_schema_has_key (schema, key) != 0;             // C 262
    g_settings_schema_unref (schema);                              // C 263
    return rc;                                                     // C 265
  }
  #[cfg (not (feature = "glib_version_2_32"))]
  { return false; }                                                // C 267
}

/* C 273-390: the same `#ifdef HAVE_PGTK` region: `store_font_options_changed` and the three
   `apply_gsettings_font_*` helpers.  */
/// C 274-281: "Store an event for re-rendering of the fonts."
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe fn store_font_options_changed ()
{
  if dpyinfo_valid (first_dpyinfo)                                 // C 278
    {
      store_config_changed_event (Qfont_render (),                // C 279-280
                                  XCAR ((*first_dpyinfo).name_list_element));
    }
}

/// C 283-318: "Apply changes in the hinting system setting."
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe fn apply_gsettings_font_hinting (settings: *mut GSettings)
{
  let mut val: *mut GVariant;
  let mut hinting: *const c_char;

  if !xg_settings_key_valid_p (settings, GSETTINGS_FONT_HINTING)   // C 290
    { return; }

  val = g_settings_get_value (settings, GSETTINGS_FONT_HINTING);   // C 293

  if !val.is_null ()                                               // C 295
    {
      g_variant_ref_sink (val);                                    // C 297

      if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0    // C 299
        {
          hinting = g_variant_get_string (val, core::ptr::null_mut ());   // C 301

          if strcmp (hinting, c"full".as_ptr ()) == 0              // C 303
            { cairo_font_options_set_hint_style (font_options, CAIRO_HINT_STYLE_FULL); }
          else if strcmp (hinting, c"medium".as_ptr ()) == 0       // C 306
            { cairo_font_options_set_hint_style (font_options, CAIRO_HINT_STYLE_MEDIUM); }
          else if strcmp (hinting, c"slight".as_ptr ()) == 0       // C 309
            { cairo_font_options_set_hint_style (font_options, CAIRO_HINT_STYLE_SLIGHT); }
          else if strcmp (hinting, c"none".as_ptr ()) == 0         // C 312
            { cairo_font_options_set_hint_style (font_options, CAIRO_HINT_STYLE_NONE); }
        }
      g_variant_unref (val);                                       // C 316
    }
}

/// C 320-351: "Apply changes in the antialiasing system setting."
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe fn apply_gsettings_font_antialias (settings: *mut GSettings)
{
  let mut val: *mut GVariant;
  let mut antialias: *const c_char;

  if !xg_settings_key_valid_p (settings, GSETTINGS_FONT_ANTIALIASING)   // C 327
    { return; }

  val = g_settings_get_value (settings, GSETTINGS_FONT_ANTIALIASING);   // C 330

  if !val.is_null ()                                               // C 332
    {
      g_variant_ref_sink (val);                                    // C 334
      if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0    // C 335
        {
          antialias = g_variant_get_string (val, core::ptr::null_mut ());   // C 337

          if strcmp (antialias, c"none".as_ptr ()) == 0            // C 339
            { cairo_font_options_set_antialias (font_options, CAIRO_ANTIALIAS_NONE); }
          else if strcmp (antialias, c"grayscale".as_ptr ()) == 0  // C 342
            { cairo_font_options_set_antialias (font_options, CAIRO_ANTIALIAS_GRAY); }
          else if strcmp (antialias, c"rgba".as_ptr ()) == 0       // C 345
            { cairo_font_options_set_antialias (font_options, CAIRO_ANTIALIAS_SUBPIXEL); }
        }
      g_variant_unref (val);                                       // C 349
    }
}

/// C 353-389: "Apply the settings for the rgb element ordering."
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe fn apply_gsettings_font_rgba_order (settings: *mut GSettings)
{
  let mut val: *mut GVariant;
  let mut rgba_order: *const c_char;

  if !xg_settings_key_valid_p (settings, GSETTINGS_FONT_RGBA_ORDER)   // C 360
    { return; }

  val = g_settings_get_value (settings,                                         // C 363-364
                              GSETTINGS_FONT_RGBA_ORDER);

  if !val.is_null ()                                               // C 366
    {
      g_variant_ref_sink (val);                                    // C 368

      if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0    // C 370
        {
          rgba_order = g_variant_get_string (val, core::ptr::null_mut ());   // C 372

          if strcmp (rgba_order, c"rgb".as_ptr ()) == 0            // C 374
            { cairo_font_options_set_subpixel_order (font_options, CAIRO_SUBPIXEL_ORDER_RGB); }
          else if strcmp (rgba_order, c"bgr".as_ptr ()) == 0       // C 377
            { cairo_font_options_set_subpixel_order (font_options, CAIRO_SUBPIXEL_ORDER_BGR); }
          else if strcmp (rgba_order, c"vrgb".as_ptr ()) == 0      // C 380
            { cairo_font_options_set_subpixel_order (font_options, CAIRO_SUBPIXEL_ORDER_VRGB); }
          else if strcmp (rgba_order, c"vbgr".as_ptr ()) == 0      // C 383
            { cairo_font_options_set_subpixel_order (font_options, CAIRO_SUBPIXEL_ORDER_VBGR); }
        }
      g_variant_unref (val);                                       // C 387
    }
}

/* The four names C's own headers declare and the mirrors in this unit's graph do not carry yet.
   They stand at the use site with the reference header's line (P-55/0.1-2), and their homes are
   named in `## Owed` 5; each carries the gate of the arm that reaches it.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
unsafe extern "C" {
  /// C: lisp.h 5555: `extern void dupstring (char **, char const *);` (C 116, 135, 729, 735,
  /// 1034, 1145, 1154, 1232, 1238)
  pub fn dupstring (s: *mut *mut c_char, str_: *const c_char);
}
/// C: lisp.h 4542-4543: `extern Lisp_Object make_formatted_string (const char *, ...)
/// ATTRIBUTE_FORMAT_PRINTF (1, 2);` (C 995) - C-variadic, so the declaration is too.
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
unsafe extern "C" {
  pub fn make_formatted_string (format: *const c_char, ...) -> Lisp_Object;
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe extern "C" {
/// C: xterm.h 1752: `extern void x_catch_errors (Display *);` (C 839) and C: xterm.h 1764:
/// `extern void x_uncatch_errors (void);` (C 853).
  pub fn x_catch_errors (dpy: *mut Display);
  pub fn x_uncatch_errors ();
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe extern "C" {
/// C: string.h: `extern char *strstr (const char *, const char *);` (C 473, 1187).
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
unsafe extern "C" {
/// C: gtkutil.h 190: `extern void xg_update_dark_mode_for_all_displays (bool dark_mode_p);`
/// (C 474).  `include/gtkutil.rs` records the header's prototype block as a comment rather than a
/// declaration, so the unit spells it at the use site (P-88 ①; `## Owed` 5).
  pub fn xg_update_dark_mode_for_all_displays (dark_mode_p: bool);
}

/* C 392-483: `#ifdef HAVE_GSETTINGS`'s `something_changed_gsettingsCB` - "Callback called when
   something changed in GSettings."  Its address goes into `g_signal_connect` (C 1126-1127), so it
   is an `unsafe extern "C" fn` (PORTING.md 0.1-6).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gsettings"))]
unsafe extern "C" fn something_changed_gsettingsCB (settings: *mut GSettings, key: *mut gchar,
                                                    user_data: gpointer)
{
  let mut val: *mut GVariant;

  if strcmp (key, GSETTINGS_TOOL_BAR_STYLE) == 0                       // C 401
    {
      val = g_settings_get_value (settings, GSETTINGS_TOOL_BAR_STYLE);   // C 403
      if !val.is_null ()                                               // C 404
        {
          g_variant_ref_sink (val);                                    // C 406
          if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0     // C 407
            {
              /* C 409: `const gchar *newstyle = g_variant_get_string (val, NULL);`  */
              let newstyle = g_variant_get_string (val, core::ptr::null_mut ());
              store_tool_bar_style_changed (newstyle, first_dpyinfo);    // C 410
            }
          g_variant_unref (val);                                       // C 412
        }
    }
  else
    {
      /* C 415-444's `#if defined USE_CAIRO || defined HAVE_XFT` and C 445-480's `#ifdef
         HAVE_PGTK` are C's own `else if` arms; each carries its gate and the chain restarts, which
         is the `src/termcap.rs` 313-330 shape (the tested keys are distinct constants, so the
         flattened chain is C's).  */
      #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
      if strcmp (key, GSETTINGS_MONO_FONT) == 0                        // C 416
        {
          val = g_settings_get_value (settings, GSETTINGS_MONO_FONT);   // C 418
          if !val.is_null ()                                           // C 419
            {
              g_variant_ref_sink (val);                                // C 421
              if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 422
                {
                  /* C 424: `const gchar *newfont = g_variant_get_string (val, NULL);`  */
                  let newfont = g_variant_get_string (val, core::ptr::null_mut ());
                  store_monospaced_changed (newfont);                  // C 425
                }
              g_variant_unref (val);                                   // C 427
            }
        }
      #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
      if strcmp (key, GSETTINGS_FONT_NAME) == 0                   // C 430
        {
          val = g_settings_get_value (settings, GSETTINGS_FONT_NAME);   // C 432
          if !val.is_null ()                                           // C 433
            {
              g_variant_ref_sink (val);                                // C 435
              if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 436
                {
                  /* C 438: `const gchar *newfont = g_variant_get_string (val, NULL);`  */
                  let newfont = g_variant_get_string (val, core::ptr::null_mut ());
                  store_font_name_changed (newfont);                   // C 439
                }
              g_variant_unref (val);                                   // C 441
            }
        }
      #[cfg (feature = "have_pgtk")]
      if strcmp (key, GSETTINGS_FONT_ANTIALIASING) == 0                // C 446
        {
          apply_gsettings_font_antialias (settings);                   // C 448
          store_font_options_changed ();                               // C 449
        }
      #[cfg (feature = "have_pgtk")]
      if strcmp (key, GSETTINGS_FONT_HINTING) == 0                // C 451
        {
          apply_gsettings_font_hinting (settings);                     // C 453
          store_font_options_changed ();                               // C 454
        }
      #[cfg (feature = "have_pgtk")]
      if strcmp (key, GSETTINGS_FONT_RGBA_ORDER) == 0             // C 456
        {
          apply_gsettings_font_rgba_order (settings);                  // C 458
          store_font_options_changed ();                               // C 459
        }
      #[cfg (feature = "have_pgtk")]
      if strcmp (key, GSETTINGS_COLOR_SCHEME) == 0                // C 461
        {
          if xg_settings_key_valid_p (settings, GSETTINGS_COLOR_SCHEME)   // C 463
            {
              val = g_settings_get_value (settings, GSETTINGS_COLOR_SCHEME);   // C 465
              if !val.is_null ()                                       // C 466
                {
                  g_variant_ref_sink (val);                            // C 468
                  if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 469
                    {
                      /* C 471: `const char *color_scheme = g_variant_get_string (val, NULL);`  */
                      let color_scheme = g_variant_get_string (val, core::ptr::null_mut ());
                      /* Check dark mode preference and update all frames.  */
                      let dark_mode_p = !(strstr (color_scheme, c"dark".as_ptr ())).is_null ();   // C 473
                      xg_update_dark_mode_for_all_displays (dark_mode_p);   // C 474
                    }
                  g_variant_unref (val);                               // C 476
                }
            }
        }
    }
}

/* C 485-527: `#ifdef HAVE_GCONF` - the schema keys, the client and the callback.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
pub const GCONF_TOOL_BAR_STYLE: *const c_char = c"/desktop/gnome/interface/toolbar_style".as_ptr ();   // C 486
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf",
           any (feature = "use_cairo", feature = "have_xft")))]
pub const GCONF_MONO_FONT: *const c_char = c"/desktop/gnome/interface/monospace_font_name".as_ptr ();   // C 488
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf",
           any (feature = "use_cairo", feature = "have_xft")))]
pub const GCONF_FONT_NAME: *const c_char = c"/desktop/gnome/interface/font_name".as_ptr ();   // C 489
/* C 492-494: "The single GConf instance, or NULL if not connected to GConf."  */
/* C 494: `static GConfClient *gconf_client;`  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
static mut gconf_client: *mut GConfClient = core::ptr::null_mut ();

/// C 496-525: "Callback called when something changed in GConf that we care about." - its address
/// goes into `gconf_client_notify_add` (C 1216-1219, 1245-1256), so it is an
/// `unsafe extern "C" fn` (PORTING.md 0.1-6).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_gconf"))]
unsafe extern "C" fn something_changed_gconfCB (client: *mut GConfClient, cnxn_id: guint,
                                                entry: *mut GConfEntry, user_data: gpointer)
{
  let v = gconf_entry_get_value (entry);                       // C 504
  let key = gconf_entry_get_key (entry);                       // C 505

  if v.is_null () || (*v).r#type != GCONF_VALUE_STRING || key.is_null ()   // C 507
    { return; }
  if strcmp (key, GCONF_TOOL_BAR_STYLE) == 0                   // C 508
    {
      let value = gconf_value_get_string (v);                  // C 510
      store_tool_bar_style_changed (value, first_dpyinfo);     // C 511
    }
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  if strcmp (key, GCONF_MONO_FONT) == 0                   // C 514
    {
      let value = gconf_value_get_string (v);                  // C 516
      store_monospaced_changed (value);                        // C 517
    }
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  if strcmp (key, GCONF_FONT_NAME) == 0                   // C 519
    {
      let value = gconf_value_get_string (v);                  // C 521
      store_font_name_changed (value);                         // C 522
    }
}

/* C 529-542: `#if defined USE_CAIRO || defined HAVE_XFT`'s three `#ifndef FC_LCD_*` fallback
   definitions.  The header has all three names, so the `#ifndef`s are no-ops and the three
   constants are declared once in the fontconfig segment above (C 531-542, the `src/ftfont.rs`
   2793 reading of the same `#ifndef` pair).  */

/* C 544-563: `#ifndef HAVE_PGTK`'s `get_prop_window` - "Find the window that contains the
   XSETTINGS property values."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn get_prop_window (dpyinfo: *mut xsettings_dpi::Display_Info)
{
  use x11_constants::None;                                      // C 555/558's X11 `None`

  let dpy = (*dpyinfo).display;                                 // C 550

  XGrabServer (dpy);                                            // C 552
  (*dpyinfo).xsettings_window = XGetSelectionOwner (dpy,        // C 553-554
                                                    (*dpyinfo).Xatom_xsettings_sel);
  if (*dpyinfo).xsettings_window != None as Window              // C 555
    /* Select events so we can detect if window is deleted or if settings
       are changed.  */
    { XSelectInput (dpy, (*dpyinfo).xsettings_window,           // C 558-559
                    (PropertyChangeMask | StructureNotifyMask) as c_long); }

  XUngrabServer (dpy);                                          // C 561
}

/* C 565-821: `#ifndef HAVE_PGTK`'s `PAD` and `parse_settings`.  */
/// C 567: `#define PAD(nr)    (((nr) + 3) & ~3)`
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
macro_rules! PAD {
  ($nr:expr) => { ((($nr) + 3) & !3) };
}

/// C 569-820: "Parse xsettings and extract those that deal with Xft."  The long comment at
/// C 569-617 documents the property's layout; C 619-623 is the definition and C 820 the closing
/// brace.  C's `switch (type)` becomes a `match` whose `_` arm is C's `default:` (PORTING.md 2.2;
/// the row in `docs/ports/macro-body-notes.tsv` records the tokens).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn parse_settings (prop: *mut c_uchar, bytes: c_ulong,
                          settings: *mut xsettings) -> c_int
{
  let int1: c_int = 1;                                          // C 624
  /* C's `*(char *) &int1 == 1 ? LSBFirst : MSBFirst`: the byte-order probe, kept as C's cast of
     the first byte of `int1`.  */
  let my_bo: c_int = if *((&raw const int1) as *const c_char) == 1   // C 625
                       { LSBFirst } else { MSBFirst };
  let that_bo: c_int = *prop as c_int;                          // C 626
  let mut n_settings: CARD32 = 0;   /* PORTING.md 1.5: C leaves it indeterminate */
  let mut bytes_parsed: c_int = 0;                              // C 628
  let mut settings_seen: c_int = 0;                             // C 629
  let mut i: c_int = 0;                                         // C 630
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  let mut gdk_unscaled_dpi: c_int = 0;                          // C 637
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  let mut gdk_window_scale: c_int = 0;                          // C 638

  /* First 4 bytes is a serial number, skip that.  */

  if bytes < 12 { return settings_seen; }                       // C 643
  memcpy (&raw mut n_settings as *mut c_void,                    // C 644
          prop.offset (8) as *const c_void, 4);
  if my_bo != that_bo { n_settings = bswap_32 (n_settings); }    // C 645
  bytes_parsed = 12;                                            // C 646

  memset (settings as *mut c_void, 0, core::mem::size_of::<xsettings> ());   // C 648

  while ((bytes_parsed + 4) as c_ulong) < bytes && settings_seen < 7 && i < n_settings as c_int   // C 650-651
    {
      let r#type: c_int = *prop.offset (bytes_parsed as isize) as c_int;   // C 653
      bytes_parsed += 1;
      let mut nlen: CARD16 = 0;     /* PORTING.md 1.5 */
      let mut vlen: CARD32 = 0;     /* PORTING.md 1.5 */
      let mut ival: CARD32 = 0;                                 // C 655
      let mut name: [c_char; 128] = [0; 128];                   // C 656
      let mut sval: [c_char; 128] = [0; 128];                   // C 657
      let mut want_this: bool;
      let mut to_cpy: c_int;

      sval[0] = 0;                                              // C 661
      i += 1;                                                   // C 662
      bytes_parsed += 1;                                        /* Padding */   // C 663

      memcpy (&raw mut nlen as *mut c_void,                      // C 665
              prop.offset (bytes_parsed as isize) as *const c_void, 2);
      bytes_parsed += 2;                                        // C 666
      if my_bo != that_bo { nlen = bswap_16 (nlen); }            // C 667
      if (bytes_parsed as c_ulong) + nlen as c_ulong > bytes { return settings_seen; }   // C 668
      to_cpy = min (nlen as isize,                               // C 669
                    core::mem::size_of::<[c_char; 128]> () as isize - 1) as c_int;
      memcpy (name.as_mut_ptr () as *mut c_void,                 // C 670
              prop.offset (bytes_parsed as isize) as *const c_void, to_cpy as usize);
      name[to_cpy as usize] = 0;                                 // C 671

      bytes_parsed += nlen as c_int;                             // C 673
      bytes_parsed = PAD! (bytes_parsed);                        // C 674

      bytes_parsed += 4; /* Skip serial for this value */        // C 676
      if (bytes_parsed as c_ulong) > bytes { return settings_seen; }   // C 677

      want_this = strcmp (XSETTINGS_TOOL_BAR_STYLE.as_ptr (), name.as_ptr ()) == 0;   // C 679
      #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
      if (nlen > 6 && memcmp (name.as_ptr () as *const c_void, c"Xft/".as_ptr () as *const c_void, 4) == 0)   // C 681
          || strcmp (XSETTINGS_FONT_NAME, name.as_ptr ()) == 0
          || strcmp (XSETTINGS_GDK_DPI_NAME, name.as_ptr ()) == 0
          || strcmp (XSETTINGS_GDK_WSCALE_NAME, name.as_ptr ()) == 0
        { want_this = true; }                                    // C 685

      /* C 688-723: C's `switch (type)` - the three arms and its `default:` (the `_` arm returns
         C's value).  */
      match r#type
        {
        0 =>                                                     /* Integer, C 690 */
          {
            if (bytes_parsed + 4) as c_ulong > bytes { return settings_seen; }   // C 691
            if want_this                                          // C 692
              {
                memcpy (&raw mut ival as *mut c_void,             // C 694
                        prop.offset (bytes_parsed as isize) as *const c_void, 4);
                if my_bo != that_bo { ival = bswap_32 (ival); }    // C 695
              }
            bytes_parsed += 4;                                    // C 697
          }
        1 =>                                                     /* String, C 700 */
          {
            if (bytes_parsed + 4) as c_ulong > bytes { return settings_seen; }   // C 701
            memcpy (&raw mut vlen as *mut c_void,                 // C 702
                    prop.offset (bytes_parsed as isize) as *const c_void, 4);
            bytes_parsed += 4;                                    // C 703
            if my_bo != that_bo { vlen = bswap_32 (vlen); }        // C 704
            if want_this                                          // C 705
              {
                to_cpy = min (vlen as isize,                      // C 707
                              core::mem::size_of::<[c_char; 128]> () as isize - 1) as c_int;
                memcpy (sval.as_mut_ptr () as *mut c_void,        // C 708
                        prop.offset (bytes_parsed as isize) as *const c_void, to_cpy as usize);
                sval[to_cpy as usize] = 0;                        // C 709
              }
            bytes_parsed += vlen as c_int;                        // C 711
            bytes_parsed = PAD! (bytes_parsed);                   // C 712
          }
        2 =>                                                     /* RGB value, C 715 */
          {
            /* No need to parse this */
            if (bytes_parsed + 8) as c_ulong > bytes { return settings_seen; }   // C 717
            bytes_parsed += 8; /* 4 values (r, b, g, alpha), 2 bytes each.  */    // C 718
          }
        _ => /* Parse Error, C 721 */
          { return settings_seen; }                               // C 722
        }

      if want_this                                                // C 725
        {
          if strcmp (name.as_ptr (), XSETTINGS_TOOL_BAR_STYLE.as_ptr ()) == 0   // C 727
            {
              dupstring (&raw mut (*settings).tb_style, sval.as_mut_ptr ());   // C 729
              (*settings).seen |= SEEN_TB_STYLE;                  // C 730
            }
          else
            {
              /* C 732-799's `#if defined USE_CAIRO || defined HAVE_XFT` arms and C 800-801's
                 `else want_this = false;`, which closes the last arm of whichever configuration is
                 built: the chain is C's inside each gate (the `src/termcap.rs` 313-330 shape), and
                 C 748-763's nested `# ifdef FC_HINT_STYLE` keeps its arm with the body gated - a
                 Rust `#[cfg]` cannot sit between an `else` and its `if` (`error: expected
                 expression, found keyword 'else'`, measured), and the arm's *absence* in C is
                 exactly the trailing `else`'s behaviour for that name (`## Deviations` 6).  */
              #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
              {
                if strcmp (name.as_ptr (), XSETTINGS_FONT_NAME) == 0      // C 733
                  {
                    dupstring (&raw mut (*settings).font, sval.as_mut_ptr ());   // C 735
                    (*settings).seen |= SEEN_FONT;              // C 736
                  }
                else if strcmp (name.as_ptr (), c"Xft/Antialias".as_ptr ()) == 0   // C 738
                  {
                    (*settings).seen |= SEEN_AA;                // C 740
                    (*settings).aa = (ival != 0) as FcBool;     // C 741
                  }
                else if strcmp (name.as_ptr (), c"Xft/Hinting".as_ptr ()) == 0   // C 743
                  {
                    (*settings).seen |= SEEN_HINTING;           // C 745
                    (*settings).hinting = (ival != 0) as FcBool;   // C 746
                  }
                else if strcmp (name.as_ptr (), c"Xft/HintStyle".as_ptr ()) == 0   // C 749
                  {
                    /* C 748-763's `# ifdef FC_HINT_STYLE` arm; the `#else` side is the trailing
                       `else` for this name.  */
                    #[cfg (feature = "fc_hint_style")]
                    {
                      (*settings).seen |= SEEN_HINTSTYLE;       // C 751
                      if strcmp (sval.as_ptr (), c"hintnone".as_ptr ()) == 0      // C 752
                        { (*settings).hintstyle = FC_HINT_NONE; }
                      else if strcmp (sval.as_ptr (), c"hintslight".as_ptr ()) == 0   // C 754
                        { (*settings).hintstyle = FC_HINT_SLIGHT; }
                      else if strcmp (sval.as_ptr (), c"hintmedium".as_ptr ()) == 0   // C 756
                        { (*settings).hintstyle = FC_HINT_MEDIUM; }
                      else if strcmp (sval.as_ptr (), c"hintfull".as_ptr ()) == 0     // C 758
                        { (*settings).hintstyle = FC_HINT_FULL; }
                      else
                        { (*settings).seen &= !SEEN_HINTSTYLE; }   // C 761
                    }
                    #[cfg (not (feature = "fc_hint_style"))]
                    { want_this = false; }                      // C 800-801's `else`
                  }
                else if strcmp (name.as_ptr (), c"Xft/RGBA".as_ptr ()) == 0   // C 764
                  {
                    (*settings).seen |= SEEN_RGBA;              // C 766
                    if strcmp (sval.as_ptr (), c"none".as_ptr ()) == 0          // C 767
                      { (*settings).rgba = FC_RGBA_NONE; }
                    else if strcmp (sval.as_ptr (), c"rgb".as_ptr ()) == 0      // C 769
                      { (*settings).rgba = FC_RGBA_RGB; }
                    else if strcmp (sval.as_ptr (), c"bgr".as_ptr ()) == 0      // C 771
                      { (*settings).rgba = FC_RGBA_BGR; }
                    else if strcmp (sval.as_ptr (), c"vrgb".as_ptr ()) == 0     // C 773
                      { (*settings).rgba = FC_RGBA_VRGB; }
                    else if strcmp (sval.as_ptr (), c"vbgr".as_ptr ()) == 0     // C 775
                      { (*settings).rgba = FC_RGBA_VBGR; }
                    else
                      { (*settings).seen &= !SEEN_RGBA; }       // C 778
                  }
                else if strcmp (name.as_ptr (), c"Xft/DPI".as_ptr ()) == 0     // C 780
                        && ival != CARD32::MAX                  /* C's `(CARD32) -1` */
                  {
                    (*settings).seen |= SEEN_DPI;               // C 782
                    (*settings).dpi = ival as c_double / 1024.0;   // C 783
                  }
                else if strcmp (name.as_ptr (), XSETTINGS_GDK_DPI_NAME) == 0   // C 785
                  { gdk_unscaled_dpi = ival as c_int; }
                else if strcmp (name.as_ptr (), XSETTINGS_GDK_WSCALE_NAME) == 0   // C 787
                  { gdk_window_scale = ival as c_int; }
                else if strcmp (name.as_ptr (), c"Xft/lcdfilter".as_ptr ()) == 0   // C 789
                  {
                    (*settings).seen |= SEEN_LCDFILTER;         // C 791
                    if strcmp (sval.as_ptr (), c"none".as_ptr ()) == 0         // C 792
                      { (*settings).lcdfilter = FC_LCD_NONE; }
                    else if strcmp (sval.as_ptr (), c"lcddefault".as_ptr ()) == 0   // C 794
                      { (*settings).lcdfilter = FC_LCD_DEFAULT; }
                    else
                      { (*settings).seen &= !SEEN_LCDFILTER; }  // C 797
                  }
                else
                  { want_this = false; }                        // C 800-801
              }
              #[cfg (not (any (feature = "use_cairo", feature = "have_xft")))]
              { want_this = false; }                            // C 800-801
            }
          settings_seen += want_this as c_int;                  // C 802
        }
    }

  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  if gdk_unscaled_dpi > 0 && gdk_window_scale > 0               // C 807
    {
      /* Override any previous DPI settings.  GDK ones are intended to
	 be authoritative.
	 See
	 https://mail.gnome.org/archives/commits-list/2013-June/msg06726.html
       */
      (*settings).seen |= SEEN_DPI;                             // C 814
      (*settings).dpi = gdk_window_scale as c_double * gdk_unscaled_dpi as c_double / 1024.0;   // C 815
    }

  settings_seen                                                 // C 819
}

/* The names C 885 (FcMatchPattern), 988 (XftDefaultSet's Xft.h), 1897-1900 (XScreenNumberOfScreen)
   and the `HAVE_XFT` arm name, and that no mirror in this graph carries; they stand at the use
   site with the reference header's line (P-55/0.1-2) and are named in `## Owed` 5.  */
/// C: fontconfig.h 336: `FcMatchPattern,` - the `FcMatchKind` member C 885 passes (measured 0,
/// Evidence ②).
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           any (feature = "use_cairo", feature = "have_xft")))]
pub const FcMatchPattern: c_int = 0;
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_xft"))]
unsafe extern "C" {
/// C: Xlib.h 1839-1840: `extern int XScreenNumberOfScreen (Screen *);` - measured on this machine
/// (Evidence ②).  X11's `Screen` record has no mirror in this tree (`include/xterm.rs` 402 names
/// it and `docs/ports/include-xterm.md` owes the record), so the parameter keeps the P-72 clause 3
/// stand-in and the call site casts.
  pub fn XScreenNumberOfScreen (screen: *mut c_void) -> c_int;
}

/* C 823-857: `#ifndef HAVE_PGTK`'s `read_settings` - "Read settings from the XSettings property
   window on display for DPYINFO.  Store settings read in SETTINGS.  Return true if successful."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn read_settings (dpyinfo: *mut xsettings_dpi::Display_Info, settings: *mut xsettings) -> bool
{
  use x11_constants::{AnyPropertyType, Success};

  let mut act_type: Atom = 0;           /* PORTING.md 1.5: C leaves them indeterminate */
  let mut act_form: c_int = 0;         /* PORTING.md 1.5 */
  let mut nitems: c_ulong = 0;         /* PORTING.md 1.5 */
  let mut bytes_after: c_ulong = 0;    /* PORTING.md 1.5 */
  let mut prop: *mut c_uchar = core::ptr::null_mut ();          // C 834
  let dpy = (*dpyinfo).display;                                 // C 835
  let rc: c_int;
  let mut got_settings = false;                                 // C 837

  x_catch_errors (dpy);                                         // C 839
  rc = XGetWindowProperty (dpy,                                 // C 840-845
                           (*dpyinfo).xsettings_window,
                           (*dpyinfo).Xatom_xsettings_prop,
                           0, LONG_MAX as c_long, False, AnyPropertyType as Atom,
                           &raw mut act_type, &raw mut act_form, &raw mut nitems,
                           &raw mut bytes_after, &raw mut prop);

  if rc == Success && !prop.is_null () && act_form == 8 && nitems > 0       // C 847-848
      && act_type == (*dpyinfo).Xatom_xsettings_prop
    { got_settings = parse_settings (prop, nitems, settings) != 0; }        // C 849

  XFree (prop as *mut c_void);                                  // C 851

  x_uncatch_errors ();                                          // C 853

  got_settings                                                  // C 855
}

/* C 859-1005: `#ifndef HAVE_PGTK`'s `apply_xft_settings` - "Apply Xft settings in SETTINGS to the
   Xft library.  Store a Lisp event that Xft settings changed."  C's whole body is one
   `#if defined HAVE_XFT || (defined USE_CAIRO && defined CAIRO_HAS_FC_FONT && defined
   CAIRO_HAS_FT_FONT)` (C 867-1003), so a configuration without any of those has C's empty body -
   which is what the arm below is, not a stub.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn apply_xft_settings (dpyinfo: *mut xsettings_dpi::Display_Info, settings: *mut xsettings)
{
  #[cfg (any (feature = "have_xft",
              all (feature = "use_cairo", feature = "cairo_has_fc_font",
                   feature = "cairo_has_ft_font")))]
  {
    let pat: *mut FcPattern;
    let mut oldsettings: xsettings = core::mem::zeroed ();
    let mut changed = false;                                    // C 872
    #[cfg (not (feature = "have_xft"))]
    let mut options: *mut cairo_font_options_t;

    memset (&raw mut oldsettings as *mut c_void, 0,             // C 878
            core::mem::size_of::<xsettings> ());
    pat = FcPatternCreate ();                                   // C 879
    #[cfg (feature = "have_xft")]
    { XftDefaultSubstitute ((*dpyinfo).display,                  // C 881-883
                            XScreenNumberOfScreen ((*dpyinfo).screen as *mut c_void),
                            pat); }
    #[cfg (not (feature = "have_xft"))]
    {
      FcConfigSubstitute (core::ptr::null_mut (), pat, FcMatchPattern);   // C 885
      options = cairo_font_options_create ();                   // C 886
      ftcrfont_get_default_font_options (dpyinfo as *mut c_void, options);   // C 887
      cairo_ft_font_options_substitute (options, pat);          // C 888
      cairo_font_options_destroy (options);                     // C 889
      FcDefaultSubstitute (pat);                                // C 890
    }
    FcPatternGetBool (pat, FC_ANTIALIAS, 0, &raw mut oldsettings.aa);         // C 892
    FcPatternGetBool (pat, FC_HINTING, 0, &raw mut oldsettings.hinting);      // C 893
    #[cfg (feature = "fc_hint_style")]
    { FcPatternGetInteger (pat, FC_HINT_STYLE, 0, &raw mut oldsettings.hintstyle); }   // C 895
    FcPatternGetInteger (pat, FC_LCD_FILTER, 0, &raw mut oldsettings.lcdfilter);   // C 897
    FcPatternGetInteger (pat, FC_RGBA, 0, &raw mut oldsettings.rgba);         // C 898
    FcPatternGetDouble (pat, FC_DPI, 0, &raw mut oldsettings.dpi);            // C 899

    if ((*settings).seen & SEEN_AA) != 0 && oldsettings.aa != (*settings).aa   // C 901
      {
        FcPatternDel (pat, FC_ANTIALIAS);                       // C 903
        FcPatternAddBool (pat, FC_ANTIALIAS, (*settings).aa);   // C 904
        changed = true;                                         // C 905
        oldsettings.aa = (*settings).aa;                        // C 906
      }

    if ((*settings).seen & SEEN_HINTING) != 0                   // C 909
        && oldsettings.hinting != (*settings).hinting
      {
        FcPatternDel (pat, FC_HINTING);                         // C 912
        FcPatternAddBool (pat, FC_HINTING, (*settings).hinting);   // C 913
        changed = true;                                         // C 914
        oldsettings.hinting = (*settings).hinting;              // C 915
      }
    if ((*settings).seen & SEEN_RGBA) != 0 && oldsettings.rgba != (*settings).rgba   // C 917
      {
        FcPatternDel (pat, FC_RGBA);                            // C 919
        FcPatternAddInteger (pat, FC_RGBA, (*settings).rgba);   // C 920
        oldsettings.rgba = (*settings).rgba;                    // C 921
        changed = true;                                         // C 922
      }

    /* Older fontconfig versions don't have FC_LCD_FILTER. */
    if ((*settings).seen & SEEN_LCDFILTER) != 0                 // C 926
        && oldsettings.lcdfilter != (*settings).lcdfilter
      {
        FcPatternDel (pat, FC_LCD_FILTER);                      // C 929
        FcPatternAddInteger (pat, FC_LCD_FILTER, (*settings).lcdfilter);   // C 930
        changed = true;                                         // C 931
        oldsettings.lcdfilter = (*settings).lcdfilter;          // C 932
      }

    #[cfg (feature = "fc_hint_style")]
    if ((*settings).seen & SEEN_HINTSTYLE) != 0                 // C 936
        && oldsettings.hintstyle != (*settings).hintstyle
      {
        FcPatternDel (pat, FC_HINT_STYLE);                      // C 939
        FcPatternAddInteger (pat, FC_HINT_STYLE, (*settings).hintstyle);   // C 940
        changed = true;                                         // C 941
        oldsettings.hintstyle = (*settings).hintstyle;          // C 942
      }

    #[cfg (feature = "use_cairo")]
    {
      /* When Cairo is being used, set oldsettings.dpi to dpyinfo->resx.
	 This is a gross hack, but seeing as Cairo fails to report
	 anything reasonable, just use it to avoid config-changed events
	 being sent at startup.  */
      oldsettings.dpi = (*dpyinfo).resx;                        // C 951
    }

    if ((*settings).seen & SEEN_DPI) != 0                       // C 954
        && (*settings).dpi > 0.0
	/* The following conjunct avoids setting `changed' to true when
	   old and new dpi settings do not differ "substantially".
	   Otherwise, the dynamic-setting Elisp code may process all sorts
	   of unrelated settings that override users' font customizations,
	   among others.  C 962-963 cites two mailing-list threads here (the
	   2016-05 `emacs-devel` message and the 2016-12 tracker one); their
	   URLs carry the reference's bug address, which this repository does
	   not repeat (`## Deviations` 12).

	   As soon as the dynamic-settings code has been tested and
	   verified, this Emacs 25.2 workaround should be removed.  */
        && (((oldsettings.dpi >= (*settings).dpi)                // C 967
	     && (oldsettings.dpi - (*settings).dpi) > 2.0)
	    || (((*settings).dpi > oldsettings.dpi)
	        && ((*settings).dpi - oldsettings.dpi) > 2.0))
      {
        FcPatternDel (pat, FC_DPI);                             // C 972
        FcPatternAddDouble (pat, FC_DPI, (*settings).dpi);      // C 973
        changed = true;                                         // C 974
        oldsettings.dpi = (*settings).dpi;                      // C 975

        /* Changing the DPI on this display affects all frames on it.
	   Check FRAME_RES_X and FRAME_RES_Y in frame.h to see how.  */
        (*dpyinfo).resx = (*settings).dpi;                      // C 979 (C's chain, right to left)
        (*dpyinfo).resy = (*settings).dpi;
      }

    if changed                                                  // C 982
      {
        let format: *const c_char
          = c"Antialias: %d, Hinting: %d, RGBA: %d, LCDFilter: %d, Hintstyle: %d, DPI: %f".as_ptr ();   // C 984-986
        #[cfg (feature = "have_xft")]
        { XftDefaultSet ((*dpyinfo).display, pat); }             // C 988
        #[cfg (not (feature = "have_xft"))]
        { FcPatternDestroy (pat); }                             // C 990
        store_config_changed_event (Qfont_render (),            // C 992-993
                                    XCAR ((*dpyinfo).name_list_element));
        Vxft_settings
          = make_formatted_string (format,                       // C 994-998
                                   oldsettings.aa as c_int, oldsettings.hinting as c_int,
                                   oldsettings.rgba, oldsettings.lcdfilter,
                                   oldsettings.hintstyle, oldsettings.dpi);
      }
    else
      { FcPatternDestroy (pat); }                               // C 1002
  }
}

/* C 1007-1039: `#ifndef HAVE_PGTK`'s `read_and_apply_settings`.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn read_and_apply_settings (dpyinfo: *mut xsettings_dpi::Display_Info, send_event_p: bool)
{
  let mut settings: xsettings = core::mem::zeroed ();            // C 1014

  if !read_settings (dpyinfo, &raw mut settings)                // C 1016
    { return; }

  apply_xft_settings (dpyinfo, &raw mut settings);              // C 1019
  if (settings.seen & SEEN_TB_STYLE) != 0                       // C 1020
    {
      if send_event_p
        { store_tool_bar_style_changed (settings.tb_style, dpyinfo); }   // C 1023
      else
        { current_tool_bar_style = map_tool_bar_style (settings.tb_style); }   // C 1025
      xfree (settings.tb_style as *mut c_void);                 // C 1026
    }
  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  if (settings.seen & SEEN_FONT) != 0                           // C 1029
    {
      if send_event_p
        { store_font_name_changed (settings.font); }            // C 1032
      else
        { dupstring (&raw mut current_font, settings.font); }   // C 1034
      xfree (settings.font as *mut c_void);                     // C 1035
    }
}

/* C 1041-1086: `#ifndef HAVE_PGTK`'s `xft_settings_event` - "Check if EVENT for the display in
   DPYINFO is XSettings related.  Return true if it is, after performing associated side
   effects."  Non-`static`, so `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xft_settings_event (dpyinfo: *mut xsettings_dpi::Display_Info, event: *const XEvent) -> bool
{
  use x11_constants::{None, ClientMessage};   /* `ClientMessage` is X.h 212's, in that module */

  let mut check_window_p = false;
  let mut apply_settings_p = false;                             // C 1048

  /* C 1050-1070: C's `switch (event->type)` with its three `case`s and no `default`; the `_` arm is
     C's fall-through (PORTING.md 2.2), and each `break` is the end of a `match` arm.  `event->type`
     and the three members are read through `include/xlib.rs`'s opaque `XEvent` record (the
     `src/widget.rs` 575 shape).  */
  /* C's three union members keep their names as the local each `case` reads: `event->type` is the
     `xany` member's (C 1050), `event->xclient`'s (C 1058-1060) and `event->xproperty`'s (C 1065-1067)
     conversions - the `src/widget.rs` 575 `XExposeEvent` shape.  */
  let xany = event as *const XAnyEvent;
  match (*xany).r#type
    {
    DestroyNotify =>                                             // C 1052
      {
        if (*dpyinfo).xsettings_window == (*xany).window         // C 1053
          { check_window_p = true; }
      }
    ClientMessage =>                                             // C 1057
      {
        let xclient = event as *const XClientMessageEvent;
        if (*xclient).message_type == (*dpyinfo).Xatom_xsettings_mgr       // C 1058
            && (*xclient).data.l[1] as c_ulong == (*dpyinfo).Xatom_xsettings_sel   // C 1059
            && (*xclient).window == (*dpyinfo).root_window       // C 1060
          { check_window_p = true; }
      }
    PropertyNotify =>                                            // C 1064
      {
        let xproperty = event as *const XPropertyEvent;
        if (*xproperty).window == (*dpyinfo).xsettings_window    // C 1065
            && (*xproperty).state == PropertyNewValue            // C 1066
            && (*xproperty).atom == (*dpyinfo).Xatom_xsettings_prop   // C 1067
          { apply_settings_p = true; }
      }
    _ => { }
    }

  if check_window_p                                             // C 1073
    {
      (*dpyinfo).xsettings_window = None as Window;              // C 1075
      get_prop_window (dpyinfo);                                 // C 1076
      if (*dpyinfo).xsettings_window != None as Window           // C 1077
        { apply_settings_p = true; }
    }

  if apply_settings_p                                           // C 1081
    { read_and_apply_settings (dpyinfo, true); }                 // C 1082

  check_window_p || apply_settings_p                            // C 1084
}

/* C 1088-1170: `init_gsettings` - "Initialize GSettings and read startup values."  C's whole body
   is `#ifdef HAVE_GSETTINGS`, so a configuration without it has C's empty body (the arm below).  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
unsafe fn init_gsettings ()
{
  #[cfg (feature = "have_gsettings")]
  {
    let mut val: *mut GVariant;
    let mut schema_found = false;                               // C 1095

    #[cfg (feature = "glib_check_version_lt_2_36")]
    { g_type_init (); }                                         // C 1098

    /* C 1101-1120: `#if GLIB_CHECK_VERSION (2, 32, 0)` - both arms carried (P-51); the two
       conditions C writes this way (`#ifdef GLIB_VERSION_2_32` at C 251 and this one) are the same
       "GLib 2.32 or newer", so both carry `glib_version_2_32`.  */
    #[cfg (feature = "glib_version_2_32")]
    {
      let source = g_settings_schema_source_get_default ();      // C 1103-1104
      if !source.is_null ()                                      // C 1105
        {
          let sc = g_settings_schema_source_lookup (source, GSETTINGS_SCHEMA, TRUE);   // C 1107-1108
          schema_found = !sc.is_null ();                          // C 1109
          if !sc.is_null () { g_settings_schema_unref (sc); }     // C 1110
        }
    }
    #[cfg (not (feature = "glib_version_2_32"))]
    {
      let mut schemas = g_settings_list_schemas ();               // C 1115
      if schemas.is_null () { return; }                           // C 1116
      while !schema_found && !(*schemas).is_null ()               // C 1117
        {
          schema_found = strcmp (*schemas, GSETTINGS_SCHEMA) == 0;   // C 1118
          schemas = schemas.offset (1);                           /* C's `*schemas++` */
        }
    }
    if !schema_found { return; }                                  // C 1121

    gsettings_client = g_settings_new (GSETTINGS_SCHEMA);         // C 1123
    if gsettings_client.is_null () { return; }                    // C 1124
    g_object_ref_sink (G_OBJECT! (gsettings_client) as gpointer);   // C 1125
    /* C's `G_CALLBACK (something_changed_gsettingsCB)` is what `g_signal_connect!` applies itself
       (`include/gtk.rs` 229-233), so the handler is passed raw (the `src/gfilenotify.rs` 343 note).  */
    g_signal_connect! (G_OBJECT! (gsettings_client), c"changed".as_ptr (),    // C 1126-1127
                       something_changed_gsettingsCB,
                       core::ptr::null_mut::<c_void> ());

    val = g_settings_get_value (gsettings_client, GSETTINGS_TOOL_BAR_STYLE);   // C 1129
    if !val.is_null ()                                            // C 1130
      {
        g_variant_ref_sink (val);                                 // C 1132
        if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0  // C 1133
          {
            current_tool_bar_style
              = map_tool_bar_style (g_variant_get_string (val,    // C 1134-1135
                                                         core::ptr::null_mut ()));
          }
        g_variant_unref (val);                                    // C 1136
      }

    #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
    {
      val = g_settings_get_value (gsettings_client, GSETTINGS_MONO_FONT);   // C 1140
      if !val.is_null ()                                          // C 1141
        {
          g_variant_ref_sink (val);                               // C 1143
          if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 1144
            {
              dupstring (&raw mut current_mono_font,               // C 1145
                         g_variant_get_string (val, core::ptr::null_mut ()));
            }
          g_variant_unref (val);                                  // C 1146
        }

      val = g_settings_get_value (gsettings_client, GSETTINGS_FONT_NAME);   // C 1149
      if !val.is_null ()                                          // C 1150
        {
          g_variant_ref_sink (val);                               // C 1152
          if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 1153
            {
              dupstring (&raw mut current_font,                   // C 1154
                         g_variant_get_string (val, core::ptr::null_mut ()));
            }
          g_variant_unref (val);                                  // C 1155
        }

      /* Only use the gsettings font entries for the Cairo backend
	 running on PGTK.  */
      #[cfg (feature = "have_pgtk")]
      {
        font_options = cairo_font_options_create ();              // C 1161
        apply_gsettings_font_antialias (gsettings_client);        // C 1162
        apply_gsettings_font_hinting (gsettings_client);          // C 1163
        apply_gsettings_font_rgba_order (gsettings_client);       // C 1164
      }
    }
  }
}

/* C 1172-1196: `#if defined HAVE_PGTK && defined HAVE_GSETTINGS` - "Get current system dark mode
   state."  Non-`static`, so `#[unsafe(no_mangle)]`.  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"),
           feature = "have_pgtk", feature = "have_gsettings"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xg_get_system_dark_mode () -> bool
{
  if !gsettings_client.is_null () && xg_settings_key_valid_p (gsettings_client, GSETTINGS_COLOR_SCHEME)   // C 1178
    {
      let val = g_settings_get_value (gsettings_client, GSETTINGS_COLOR_SCHEME);   // C 1180
      if !val.is_null ()                                          // C 1181
        {
          g_variant_ref_sink (val);                               // C 1183
          if g_variant_is_of_type (val, G_VARIANT_TYPE_STRING) != 0   // C 1184
            {
              let color_scheme = g_variant_get_string (val, core::ptr::null_mut ());   // C 1186
              let dark_mode_p = !(strstr (color_scheme, c"dark".as_ptr ())).is_null ();   // C 1187
              g_variant_unref (val);                              // C 1188
              return dark_mode_p;                                 // C 1189
            }
          g_variant_unref (val);                                  // C 1191
        }
    }
  false                                                          // C 1194
}

/* C 1198-1259: `init_gconf` - "Init GConf and read startup values."  C's whole body is
   `#if defined (HAVE_GCONF)`, so a configuration without it has C's empty body (the arm below).  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
unsafe fn init_gconf ()
{
  #[cfg (feature = "have_gconf")]
  {
    let mut s: *mut c_char;                                       // C 1204

    #[cfg (feature = "glib_check_version_lt_2_36")]
    { g_type_init (); }                                          // C 1207

    gconf_client = gconf_client_get_default ();                    // C 1210
    gconf_client_set_error_handling (gconf_client, GCONF_CLIENT_HANDLE_NONE);   // C 1211
    gconf_client_add_dir (gconf_client,                            // C 1212-1215
                          GCONF_TOOL_BAR_STYLE,
                          GCONF_CLIENT_PRELOAD_ONELEVEL,
                          core::ptr::null_mut ());
    gconf_client_notify_add (gconf_client,                         // C 1216-1219
                             GCONF_TOOL_BAR_STYLE,
                             Some (something_changed_gconfCB),
                             core::ptr::null_mut (), core::ptr::null_mut (),
                             core::ptr::null_mut ());

    s = gconf_client_get_string (gconf_client, GCONF_TOOL_BAR_STYLE, core::ptr::null_mut ());   // C 1221
    if !s.is_null ()                                               // C 1222
      {
        current_tool_bar_style = map_tool_bar_style (s);            // C 1224
        g_free (s as *mut c_void);                                 // C 1225
      }

    #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
    {
      s = gconf_client_get_string (gconf_client, GCONF_MONO_FONT, core::ptr::null_mut ());   // C 1229
      if !s.is_null ()                                             // C 1230
        {
          dupstring (&raw mut current_mono_font, s);                 // C 1232
          g_free (s as *mut c_void);                               // C 1233
        }
      s = gconf_client_get_string (gconf_client, GCONF_FONT_NAME, core::ptr::null_mut ());   // C 1235
      if !s.is_null ()                                             // C 1236
        {
          dupstring (&raw mut current_font, s);                      // C 1238
          g_free (s as *mut c_void);                               // C 1239
        }
      gconf_client_add_dir (gconf_client,                          // C 1241-1244
                            GCONF_MONO_FONT,
                            GCONF_CLIENT_PRELOAD_ONELEVEL,
                            core::ptr::null_mut ());
      gconf_client_notify_add (gconf_client,                       // C 1245-1248
                               GCONF_MONO_FONT,
                               Some (something_changed_gconfCB),
                               core::ptr::null_mut (), core::ptr::null_mut (),
                               core::ptr::null_mut ());
      gconf_client_add_dir (gconf_client,                          // C 1249-1252
                            GCONF_FONT_NAME,
                            GCONF_CLIENT_PRELOAD_ONELEVEL,
                            core::ptr::null_mut ());
      gconf_client_notify_add (gconf_client,                       // C 1253-1256
                               GCONF_FONT_NAME,
                               Some (something_changed_gconfCB),
                               core::ptr::null_mut (), core::ptr::null_mut (),
                               core::ptr::null_mut ());
    }
  }
}

/* C 1261-1281: `#ifndef HAVE_PGTK`'s `init_xsettings` - "Init Xsettings and read startup
   values."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), not (feature = "have_pgtk")))]
unsafe fn init_xsettings (dpyinfo: *mut xsettings_dpi::Display_Info)
{
  use x11_constants::None;

  let dpy = (*dpyinfo).display;                                 // C 1267

  block_input ();                                               // C 1269

  /* Select events so we can detect client messages sent when selection
     owner changes.  */
  XSelectInput (dpy, (*dpyinfo).root_window,                    // C 1273
                StructureNotifyMask as c_long);

  get_prop_window (dpyinfo);                                    // C 1275
  if (*dpyinfo).xsettings_window != None as Window              // C 1276
    { read_and_apply_settings (dpyinfo, false); }               // C 1277

  unblock_input ();                                             // C 1279
}

/* C 1283-1292: `xsettings_initialize` - non-`static`, so `#[unsafe(no_mangle)]`.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xsettings_initialize (dpyinfo: *mut xsettings_dpi::Display_Info)
{
  if first_dpyinfo.is_null () { first_dpyinfo = dpyinfo; }       // C 1286
  init_gconf ();                                                // C 1287
  #[cfg (not (feature = "have_pgtk"))]
  { init_xsettings (dpyinfo); }                                 // C 1289
  init_gsettings ();                                            // C 1291
}

/* C 1294-1301: "Return the system monospaced font.  May be NULL if not known."  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xsettings_get_system_font () -> *const c_char
{
  current_mono_font                                             // C 1300
}

/* C 1303-1312: `#ifdef USE_LUCID` - "Return the system font.  May be NULL if not known."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "use_lucid"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xsettings_get_system_normal_font () -> *const c_char
{
  current_font                                                  // C 1310
}

/* C 1314-1327: `#ifdef HAVE_PGTK` - "Return the cairo font options, updated from the gsettings
   font config entries.  The caller should call cairo_font_options_destroy on the result."  */
#[cfg (all (any (feature = "have_x_windows", feature = "have_pgtk"), feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xsettings_get_font_options () -> *mut cairo_font_options_t
{
  if !font_options.is_null ()                                    // C 1321
    { cairo_font_options_copy (font_options) }                   // C 1322
  else
    /* GSettings is not configured.  */
    { cairo_font_options_create () }                             // C 1325
}

/* C 1329-1364: the file's three subrs.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
DEFUN! ("font-get-system-normal-font", Ffont_get_system_normal_font,
       Sfont_get_system_normal_font,
       0, 0, 0,
       doc: "Get the system default application font.\nThe font is returned as either a font-spec or font name.",
       (),
       {
  if current_font.is_null () { Qnil () } else { build_string (current_font) }   // C 1336
});

#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
DEFUN! ("font-get-system-font", Ffont_get_system_font, Sfont_get_system_font,
       0, 0, 0,
       doc: "Get the system default fixed width font.\nThe font is returned as either a font-spec or font name.",
       (),
       {
  if current_mono_font.is_null () { Qnil () } else { build_string (current_mono_font) }   // C 1345
});

#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
DEFUN! ("tool-bar-get-system-style", Ftool_bar_get_system_style,
       Stool_bar_get_system_style, 0, 0, 0,
       doc: "Get the system tool bar style.\nIf no system tool bar style is known, return `tool-bar-style' if set to a\nknown style.  Otherwise return image.",
       (),
       {
  if EQ (Vtool_bar_style, Qimage ())                              // C 1355
      || EQ (Vtool_bar_style, Qtext ())
      || EQ (Vtool_bar_style, Qboth ())
      || EQ (Vtool_bar_style, Qboth_horiz ())
      || EQ (Vtool_bar_style, Qtext_image_horiz ())
    { return Vtool_bar_style; }                                   // C 1360
  if !NILP (current_tool_bar_style)                               // C 1361
    { return current_tool_bar_style; }                            // C 1362
  Qimage ()                                                       // C 1363
});

/* C 1366-1421: `syms_of_xsettings`.  The two variables C declares through `DEFVAR_BOOL`/`DEFVAR_LISP`
   live in `struct emacs_globals`, so the unit that owns them defines them (P-39's
   `tools/check-globals.py`): `include/globals.rs` 6917/7142 declares both and the definitions stand
   here in C's order - the `src/xsmfns.rs` 1015-1020 shape.  */
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub static mut Vxft_settings: Lisp_Object = lisp_h_Qnil ();
#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub static mut use_system_font: bool = false;

#[cfg (any (feature = "have_x_windows", feature = "have_pgtk"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xsettings ()
{
  current_mono_font = core::ptr::null_mut ();                    // C 1369
  PDUMPER_IGNORE! (current_mono_font);                           // C 1370
  current_font = core::ptr::null_mut ();                         // C 1371
  PDUMPER_IGNORE! (current_font);                                // C 1372
  first_dpyinfo = core::ptr::null_mut ();                        // C 1373
  PDUMPER_IGNORE! (first_dpyinfo);                               // C 1374
  #[cfg (feature = "have_gsettings")]
  {
    gsettings_client = core::ptr::null_mut ();                    // C 1376
    PDUMPER_IGNORE! (gsettings_client);                          // C 1377
  }
  #[cfg (feature = "have_gconf")]
  {
    gconf_client = core::ptr::null_mut ();                        // C 1380
    PDUMPER_IGNORE! (gconf_client);                              // C 1381
  }
  #[cfg (feature = "have_pgtk")]
  {
    font_options = core::ptr::null_mut ();                        // C 1384
    PDUMPER_IGNORE! (font_options);                              // C 1385
  }

  DEFSYM! (Qmonospace_font_name, "monospace-font-name");          // C 1388
  DEFSYM! (Qfont_name, "font-name");                             // C 1389
  DEFSYM! (Qfont_render, "font-render");                         // C 1390
  DEFSYM! (Qdynamic_setting, "dynamic-setting");                 // C 1391
  DEFSYM! (Qfont_render_setting, "font-render-setting");         // C 1392
  DEFSYM! (Qsystem_font_setting, "system-font-setting");         // C 1393

  defsubr (&raw mut Sfont_get_system_font);                      // C 1395
  defsubr (&raw mut Sfont_get_system_normal_font);               // C 1396

  DEFVAR_BOOL! (c"font-use-system-font", use_system_font,         // C 1398-1402
    "Non-nil means to apply the system defined font dynamically.\nWhen this is non-nil and the system defined fixed width font changes, we\nupdate frames dynamically.\nIf this variable is nil, Emacs ignores system font changes.");
  use_system_font = false;                                       // C 1403

  DEFVAR_LISP! (c"xft-settings", Vxft_settings,                   // C 1405-1406
                "Font settings applied to Xft.");
  Vxft_settings = empty_unibyte_string;                          // C 1407

  #[cfg (any (feature = "use_cairo", feature = "have_xft"))]
  {
    Fprovide (Qfont_render_setting (), Qnil ());                  // C 1410
    #[cfg (any (feature = "have_gconf", feature = "have_gsettings"))]
    { Fprovide (Qsystem_font_setting (), Qnil ()); }              // C 1412
  }

  current_tool_bar_style = Qnil ();                               // C 1416
  DEFSYM! (Qtool_bar_style, "tool-bar-style");                    // C 1417
  defsubr (&raw mut Stool_bar_get_system_style);                  // C 1418

  Fprovide (Qdynamic_setting (), Qnil ());                        // C 1420
}
