// femacs: port of oldXMenu/Activate.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The three non-`static` functions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2);
// the four file-scope `static` objects stay plain `static mut`.
//
// **The file gate is `feature = "have_x_windows"`** (P-86 clause 8, the build's condition rather
// than one the C text spells).  `configure.ac` 7287-7295 selects this library - the arm that sets
// `HAVE_OLDXMENU=yes` and `LIBXMENU='$(oldXMenudir)/libXMenu11.a'` is `HAVE_X11 = yes` with
// `HAVE_GTK != yes` and `USE_X_TOOLKIT = none` - and `oldXMenu/Makefile.in`'s `OBJS` block (60-85)
// lists every object of the directory unconditionally, so the reference text carries **no macro of
// its own** for this file: the condition is a configure result and the feature is the repo's name
// for the X11 window system (`HAVE_X_WINDOWS`), the gate `src/xfns.rs`'s unit uses.  P-86 clause 8
// asks for the reference's own macro lower-cased; this file has none, which the ledger's
// `## Deviations` 1 records.
//
// `HAVE_X11`, `HAVE_X_WINDOWS` and `USE_X_TOOLKIT` are `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles not one
// line of this unit and `xtask build-tu` leaves the empty object - and this machine has no X11
// *program* to run it against.  The `<X11/*>` declarations below are therefore measured against
// `/opt/homebrew/include` (xorgproto 2025.1 and libX11 1.8.13) with clang rather than taken from a
// build; every value and offset this unit carries is one of those readings, and the commands are in
// the ledger's `## Evidence`.
//
// The `<X11/Xlib.h>` surface this file reads is the shared mirror `include/xlib.rs` where that
// mirror carries the name (P-90 clause 1); what it does not carry yet is declared in this unit at
// the use site, with C's own prototype in a `/// C:` comment (P-88 clause 1 / P-55).  The ledger's
// `## Owed` names the move back into the mirror.
//
// The one sub-gate the reference spells - `HAVE_XINPUT2` at C 529 - is carried as
// `feature = "have_xinput2"` (P-86 clause 1: both arms are kept; its `#else` arm is empty).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_mut, unused_parens,
         unused_comparisons, clippy::all)]

/* C 1-115: the file's opening block - the two notices and the XMenuActivate description comment -
   is not transcribed (PORTING.md 0.1-19; the ledger's `## C line ranges` records the range).  */

/* C 116: `#include "XMenuInt.h"`.  Its mirror holds the declarations of the twelve internal
   functions this unit calls, and Rust's "declaration + definition" of one name in one scope is
   `E0428`, while an explicit item shadows a glob import - which is C's pair (PORTING.md 0.1-10 /
   P-21, the `oldXMenu/Internal.rs` 44-53 shape).  So the mirror is included inside a module that is
   glob-imported, and the `#[macro_use]` is what carries `min`/`max`/`abs` across (a glob import
   does not import `macro_rules!` items).  */
#[cfg (feature = "have_x_windows")]
#[macro_use]
mod xmenu_int {
  use super::*;
  include!("include/XMenuInt.rs");             // C 116: `#include "XMenuInt.h"`
}
#[cfg (feature = "have_x_windows")]
use xmenu_int::*;

/* XMenuInt.h 46: `#include <X11/Xlib.h>` - the header this file reaches the X11 surface through.
   The mirror that owns that surface is `include/xlib.rs` (P-90 clause 1), and the names below it
   lacks are declared in this unit at their use sites (P-88 clause 1).  */
#[cfg (feature = "have_x_windows")]
include!("../src/include/xlib.rs");            // XMenuInt.h 46: `#include <X11/Xlib.h>`

/// The Xlib names this unit's own text spells.  `include/xlib.rs` ends with `use xlib_surface::*;`,
/// but that line is a *private* import of whatever module the mirror expands into, and the
/// include-graph dedupe (PORTING.md 0.1-4) expands the mirror inside `mod xmenu_int` above: the
/// crate-root `include!` on the line before is the later reach and becomes a no-op, so this unit
/// takes the names it needs from the module (PORTING.md 0.1-9's rule, the `use x11_constants::{…}`
/// shape `oldXMenu/Internal.rs` 60-63 uses).  Every one of them is declared by that mirror.
#[cfg (feature = "have_x_windows")]
use xlib_surface::{Cursor, Display, KeySym, Pixmap, Window, XEvent};

/* C 117: `#include <X11/keysym.h>` - the header the two keysyms C 506-507 compare come from.  It
   has no mirror of its own, and the two names the file reads are declared below at their use site
   (P-88 clause 1).  */

/* C reaches `malloc`/`free` through `XMenu.h` 37's `#include <stdlib.h>` (`XMenuInt.h` 48 includes
   `XMenu.h`), which that header's mirror deliberately does not pull in because the header itself
   uses nothing from it.  This unit does, so it includes the mirror at that edge's position.  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdlib.rs");          // XMenu.h 37: `#include <stdlib.h>` (`malloc`/`free`)

/* The C fixed-width spellings.  Every unit that includes `lisp.h` gets them from that mirror's
   crate-root `use std::ffi::{…}`; XMenu's headers include no `lisp.h`, so this unit brings the
   whole set itself, the shape `oldXMenu/Internal.rs` 63-66 uses - and the `include!`d mirrors
   below (`xlib.rs`, `XMenu.rs`, `XMenuInt.rs`, `X10.rs`) read these names at the crate root.
   `c_double` is here for `lib/include/stdlib.rs` alone, whose `strtod` names it (the
   `lib-src/hexl.rs` 23 and `exec/exec1.rs` 21 sets carry it for that same declaration).  */
#[cfg (feature = "have_x_windows")]
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/// Xlib's `Success` and the two button event names come from the mirror's own `x11_constants`
/// module; the three this file reads are imported here (PORTING.md 0.1-9: a unit takes the names it
/// needs from the module rather than glob-importing it).  `None` - the fourth name this file reads
/// - is imported inside `XMenuActivate` instead, because a crate-root `None` would shadow the
/// prelude's for the whole unit (the `src/xsettings.rs` 1291 shape).  */
#[cfg (feature = "have_x_windows")]
use x11_constants::{ButtonPress, ButtonRelease, Success};

/* ------------------------------------------------------------------------------------------
   The X11 surface `include/xlib.rs` does not carry yet (P-88 clause 1: no mirror holds these
   names, and this file is their only consumer today).  Every number below is measured with clang
   against this machine's headers; each `/// C:` line keeps the header's own text and line.
   ------------------------------------------------------------------------------------------ */

/// C: Xlib.h 80-81: `#define True 1` / `#define False 0` (the header spells them with `#define`,
/// and `Bool` is Xlib.h 78's `int`).  Measured: one and zero.  The two names `XMenu.h` 264/276 and
/// this file's own `Bool` locals read; the mirror carries `Bool` but neither value (the
/// `oldXMenu/SetPane.rs` 43-50 shape).
#[cfg (feature = "have_x_windows")]
pub const True: Bool = 1;
#[cfg (feature = "have_x_windows")]
pub const False: Bool = 0;

/// C: X.h 77: `typedef unsigned long Time;` - measured `sizeof (Time) == 8`.  `XMenu.h` 261-283's
/// prototypes spell it, and C 279/288/554/555 pass `CurrentTime` to them.
#[cfg (feature = "have_x_windows")]
pub type Time = c_ulong;
/// C: X.h 139: `#define CurrentTime 0L` - "special Time".  Measured: zero.
#[cfg (feature = "have_x_windows")]
pub const CurrentTime: Time = 0;

/// C: X.h 181-191 and 214: the six `XEvent` type codes this file's two `switch`es dispatch on.
/// Measured: `KeyPress` 2, `KeyRelease` 3, `EnterNotify` 7, `LeaveNotify` 8, `Expose` 12,
/// `GenericEvent` 35.  (`ButtonPress` 4 and `ButtonRelease` 5 are the mirror's `x11_constants`
/// rows.)  `GenericEvent`'s only use is inside C 529-533's arm; the macro itself is unconditional
/// in C, so its declaration is too.
#[cfg (feature = "have_x_windows")]
pub const KeyPress: c_int = 2;
#[cfg (feature = "have_x_windows")]
pub const KeyRelease: c_int = 3;
#[cfg (feature = "have_x_windows")]
pub const EnterNotify: c_int = 7;
#[cfg (feature = "have_x_windows")]
pub const LeaveNotify: c_int = 8;
#[cfg (feature = "have_x_windows")]
pub const Expose: c_int = 12;
#[cfg (feature = "have_x_windows")]
pub const GenericEvent: c_int = 35;

/// C: X.h 223: `#define ControlMask (1<<2)` - the `state` bit C 506 tests.  Measured: 4.
#[cfg (feature = "have_x_windows")]
pub const ControlMask: c_uint = 1 << 2;
/// C: X.h 319: `#define GrabModeAsync 1` - the two modes C 275-276 and 287-288 pass with
/// `True`/`False` as `owner_events`; measured: 1.
#[cfg (feature = "have_x_windows")]
pub const GrabModeAsync: c_int = 1;

/// C: keysym.h 696 (`keysymdef.h`): `#define XK_g 0x0067` - measured `0x67`.
#[cfg (feature = "have_x_windows")]
pub const XK_g: KeySym = 0x0067;
/// C: keysym.h 208 (`keysymdef.h`): `#define XK_Escape 0xff1b` - measured `0xff1b`.
#[cfg (feature = "have_x_windows")]
pub const XK_Escape: KeySym = 0xff1b;

/* The `XEvent` union's member records C's `switch`es read.  `include/xlib.rs`'s `XEvent` is the
   measured 192-byte record with an opaque body (its 33 member records are owed in
   `docs/ports/include-xlib.md`), so the port reaches each member the way `src/xmenu.rs` 1163-1172
   and `src/xsettings.rs` 1788-1792 do (P-72 clause 3): a pointer view of the same storage, bound to
   a local named after C's member.  Every record below is asserted at its measured offsets.  */

/// C: Xlib.h 930-936: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; } XAnyEvent;` - the view C's `event.type` (the union's common initial
/// member) is read through.  Measured `[sizeof=40, align=8]`, `serial` 8, `send_event` 16,
/// `display` 24, `window` 32.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XAnyEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XAnyEvent> () == 40);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, display) == 24);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XAnyEvent, window) == 32);

/// C: Xlib.h 1028-1046: `typedef struct { … XCharStruct max_bounds; … } XFontStruct;` -
/// `XMenu.h` 274/288's `*mut XFontStruct` (and, as the record `XLoadQueryFont` returns, the one
/// `XLoadQueryFont`'s own callers keep).  This unit only ever spells the *pointer*, so it declares
/// the tag opaquely, the `lwlib/lwlib-Xaw.rs` 153-158 spelling for the same name; the measured
/// record (96 bytes, `max_bounds` 68, `ascent` 88, `descent` 92, with `XCharStruct` and
/// `XFontProp`) is what `oldXMenu/Internal.rs` 105-183 carries, and the ledger's `## Owed` names
/// the move of *that* text into `include/xlib.rs`.
#[cfg (feature = "have_x_windows")]
#[derive (Clone, Copy)]
#[repr (C)]
pub struct XFontStruct { pub _private: [u8; 0] }

/// C: Xlib.h 553-567: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Window root; Window subwindow; Time time; int x, y; int x_root, y_root;
/// unsigned int state; unsigned int keycode; Bool same_screen; } XKeyEvent;` - measured
/// `[sizeof=96, align=8]`, `window` 32, `state` 80 (C 503's `&event.xkey` and C 506's
/// `event.xkey.state`).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XKeyEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: Bool,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XKeyEvent> () == 96);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XKeyEvent, window) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XKeyEvent, state) == 80);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XKeyEvent, same_screen) == 88);

/// C: Xlib.h 571-585: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Window root; Window subwindow; Time time; int x, y; int x_root, y_root;
/// unsigned int state; unsigned int button; Bool same_screen; } XButtonEvent;` - measured
/// `[sizeof=96, align=8]`, `window` 32, `button` 84 (C 605's `event.xbutton.window`).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XButtonEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub button: c_uint,
  pub same_screen: Bool,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XButtonEvent> () == 96);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XButtonEvent, window) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XButtonEvent, button) == 84);

/// C: Xlib.h 589-626: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Window root; Window subwindow; Time time; int x, y; int x_root, y_root;
/// int mode; int detail; Bool same_screen; Bool focus; unsigned int state; } XCrossingEvent;` -
/// measured `[sizeof=104, align=8]`, `window` 32, `mode` 80, `focus` 92, `state` 96 (C 366/450's
/// `event.xcrossing.window`).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XCrossingEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub mode: c_int,
  pub detail: c_int,
  pub same_screen: Bool,
  pub focus: Bool,
  pub state: c_uint,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XCrossingEvent> () == 104);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XCrossingEvent, window) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XCrossingEvent, same_screen) == 88);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XCrossingEvent, state) == 96);

/// C: Xlib.h 658-667: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; int x, y; int width, height; int count; } XExposeEvent;` - measured
/// `[sizeof=64, align=8]`, `window` 32, `x` 40, `width` 48, `count` 56 (C 326's
/// `event.xexpose.window`).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XExposeEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::size_of::<XExposeEvent> () == 64);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XExposeEvent, window) == 32);
#[cfg (feature = "have_x_windows")]
const _: () = core::assert! (core::mem::offset_of! (XExposeEvent, count) == 56);

/* The Xlib entry points this file calls, each with the header's own prototype (P-88 clause 1).
   The one macro among them is C 581's `QLength`, whose body reads Xlib's private display record
   (`_XPrivDisplay`, Xlib.h 85-119) that no mirror here carries: the port calls `XQLength`, the
   entry point that reads the same field (Xlib.h 2926, measured present as `_XQLength` in
   `/opt/homebrew/lib/libX11.6.dylib`) - the reading `oldXMenu/Internal.rs` 246-251 takes for
   `DisplayWidth`/`DisplayHeight` (the ledger's `## Deviations` 5).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1692-1695: `extern KeySym XLookupKeysym (XKeyEvent*, int);`
  pub fn XLookupKeysym (event: *mut XKeyEvent, index: c_int) -> KeySym;
  /// C: Xlib.h 2730-2737: `extern int XGrabKeyboard (Display*, Window, Bool, int, int, Time);`
  pub fn XGrabKeyboard (display: *mut Display, grab_window: Window, owner_events: Bool,
                        pointer_mode: c_int, keyboard_mode: c_int, time: Time) -> c_int;
  /// C: Xlib.h 2739-2749: `extern int XGrabPointer (Display*, Window, Bool, unsigned int, int, int,
  /// Window, Cursor, Time);`
  pub fn XGrabPointer (display: *mut Display, grab_window: Window, owner_events: Bool,
                       event_mask: c_uint, pointer_mode: c_int, keyboard_mode: c_int,
                       confine_to: Window, cursor: Cursor, time: Time) -> c_int;
  /// C: Xlib.h 2811-2814: `extern int XMapSubwindows (Display*, Window);`
  pub fn XMapSubwindows (display: *mut Display, w: Window) -> c_int;
  /// C: Xlib.h 2816-2819: `extern int XMapWindow (Display*, Window);`
  pub fn XMapWindow (display: *mut Display, w: Window) -> c_int;
  /// C: Xlib.h 2851-2854: `extern int XNextEvent (Display*, XEvent*);`
  pub fn XNextEvent (display: *mut Display, event_return: *mut XEvent) -> c_int;
  /// C: Xlib.h 2875-2878: `extern int XPeekEvent (Display*, XEvent*);`
  pub fn XPeekEvent (display: *mut Display, event_return: *mut XEvent) -> c_int;
  /// C: Xlib.h 2891-2893: `extern int XPending (Display*);`
  pub fn XPending (display: *mut Display) -> c_int;
  /// C: Xlib.h 2908-2911: `extern int XPutBackEvent (Display*, XEvent*);`
  pub fn XPutBackEvent (display: *mut Display, event: *mut XEvent) -> c_int;
  /// C: Xlib.h 2926-2928: `extern int XQLength (Display*);` - `QLength`'s entry point (C 581).
  pub fn XQLength (display: *mut Display) -> c_int;
  /// C: Xlib.h 2993-3005: `extern Bool XQueryPointer (Display*, Window, Window*, Window*, int*,
  /// int*, int*, int*, unsigned int*);`
  pub fn XQueryPointer (display: *mut Display, w: Window, root_return: *mut Window,
                        child_return: *mut Window, root_x_return: *mut c_int,
                        root_y_return: *mut c_int, win_x_return: *mut c_int,
                        win_y_return: *mut c_int, mask_return: *mut c_uint) -> Bool;
  /// C: Xlib.h 3036-3039: `extern int XRaiseWindow (Display*, Window);`
  pub fn XRaiseWindow (display: *mut Display, w: Window) -> c_int;
  /// C: Xlib.h 3343-3347: `extern int XSetWindowBackground (Display*, Window, unsigned long);`
  pub fn XSetWindowBackground (display: *mut Display, w: Window,
                               background_pixel: c_ulong) -> c_int;
  /// C: Xlib.h 3349-3353: `extern int XSetWindowBackgroundPixmap (Display*, Window, Pixmap);`
  pub fn XSetWindowBackgroundPixmap (display: *mut Display, w: Window,
                                     background_pixmap: Pixmap) -> c_int;
  /// C: Xlib.h 3419-3421: `extern int XSync (Display*, Bool);`
  pub fn XSync (display: *mut Display, discard: Bool) -> c_int;
  /// C: Xlib.h 3486-3489: `extern int XUngrabKeyboard (Display*, Time);`
  pub fn XUngrabKeyboard (display: *mut Display, time: Time) -> c_int;
  /// C: Xlib.h 3491-3494: `extern int XUngrabPointer (Display*, Time);`
  pub fn XUngrabPointer (display: *mut Display, time: Time) -> c_int;
  /// C: Xlib.h 3515-3518: `extern int XUnmapWindow (Display*, Window);`
  pub fn XUnmapWindow (display: *mut Display, w: Window) -> c_int;
}

/* C 119: "For debug, set this to 0 to not grab the keyboard on menu popup".  C's file-scope
   `static` has internal linkage, so no `#[unsafe(no_mangle)]` and no `pub` (PORTING.md 0.2).  */
/// C 120: `static int x_menu_grab_keyboard = 1;`
#[cfg (feature = "have_x_windows")]
static mut x_menu_grab_keyboard: c_int = 1;

/// C 122: `static Wait_func wait_func;` - `Wait_func` is already an `Option` (XMenu.h 256), so C's
/// uninitialised `static` is that type's `None` (0.1-13: Rust has no uninitialised `static`).
#[cfg (feature = "have_x_windows")]
static mut wait_func: Wait_func = None;
/// C 123: `static void* wait_data;` - a zero-initialised C `static` pointer is the null pointer.
#[cfg (feature = "have_x_windows")]
static mut wait_data: *mut c_void = core::ptr::null_mut ();
/// C 124: `static Translate_func translate_func = NULL;` - C's `NULL` is `None` for the `Option`
/// this pointer type is (P-27).
#[cfg (feature = "have_x_windows")]
static mut translate_func: Translate_func = None;

/// C 126-131: `void XMenuActivateSetWaitFunction (Wait_func func, void *data) { wait_func = func;
/// wait_data = data; }` - non-`static`, so `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuActivateSetWaitFunction (func: Wait_func, data: *mut c_void)
{
  wait_func = func;
  wait_data = data;
}

/// C 133-137: `void XMenuActivateSetTranslateFunction (Translate_func func) { translate_func =
/// func; }`
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuActivateSetTranslateFunction (func: Translate_func)
{
  translate_func = func;
}

/* C 139-642: `XMenuActivate` - "Maps a given menu to the display and activates the menu for user
   selection."  The C 140-149 parameter list is carried below; `register` is C's storage hint and
   has no Rust counterpart (PORTING.md 1.2), and the `help_callback` type is XMenu.h 283's
   `void (*) (char const *, int, int)` (a nullable C function pointer, so an `Option` - P-27).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XMenuActivate (display: *mut Display, menu: *mut XMenu,
                                        p_num: *mut c_int, s_num: *mut c_int,
                                        x_pos: c_int, y_pos: c_int, event_mask: c_uint,
                                        data: *mut *mut c_char,
                                        help_callback: Option<unsafe extern "C" fn (arg0: *const c_char,
                                                                                    arg1: c_int,
                                                                                    arg2: c_int)>) -> c_int
{
  let mut status: c_int = 0;                    /* X routine call status. */
  let mut orig_x: c_int = 0;                    /* Upper left menu origin X coord. */
  let mut orig_y: c_int = 0;                    /* Upper left menu origin Y coord. */
  let mut ret_val: c_int = 0;                   /* Return value. */

  let mut p_ptr: *mut XMPane;                   /* Current XMPane. */
  let mut event_xmp: *mut XMPane;               /* Event XMPane pointer. */
  let mut cur_p: *mut XMPane;                   /* Current pane. */
  let mut cur_s: *mut XMSelect;                 /* Current selection. */
  let mut event_xmw: *mut XMWindow;             /* Event XMWindow pointer. */
  let mut event: XEvent = core::mem::zeroed ();         /* X input event. */
  let mut peek_event: XEvent = core::mem::zeroed ();    /* X input peek ahead event. */

  let mut selection: Bool = False;              /* Selection has been made. */
  let mut forward: Bool = True;                 /* Moving forward in the pane list. */

  let mut root: Window = 0;                     /* XQueryPointer's root_return. */
  let mut child: Window = 0;                    /* XQueryPointer's child_return. */
  let mut root_x: c_int = 0;                    /* XQueryPointer's root x. */
  let mut root_y: c_int = 0;                    /* XQueryPointer's root y. */
  let mut win_x: c_int = 0;                     /* XQueryPointer's window x. */
  let mut win_y: c_int = 0;                     /* XQueryPointer's window y. */
  let mut mask: c_uint = 0;                     /* XQueryPointer's mask_return. */
  let mut keysym: KeySym = 0;                   /* Keysym of the last key event. */

  /* C 172-176: "Define and allocate a foreign event queue to hold events that don't belong to
     XMenu.  These events are later restored to the X event queue."  C's record is a `typedef`
     inside the function body - a type has no linkage, so the port's is here too (the
     `oldXMenu/Internal.rs` 329-345 `_xmwinquedef` shape).  */
  /// C 177-180: `typedef struct _xmeventque { XEvent event; struct _xmeventque *next; }
  /// XMEventQue;`
  #[repr (C)]
  struct _xmeventque
  {
    event: XEvent,
    next: *mut _xmeventque,
  }
  /// C 177-180: the typedef name.
  type XMEventQue = _xmeventque;

  let mut feq: *mut XMEventQue = core::ptr::null_mut ();   /* Foreign event queue. */
  let mut feq_tmp: *mut XMEventQue;                        /* Foreign event queue temporary. */

  /* C 185-188: "If there are no panes in the menu then return failure because the menu is not
     initialized."  */
  if (*menu).p_count == 0
    {
      _XMErrorCode = XME_NOT_INIT;
      return XM_FAILURE;
    }

  /* C 194-197: "Find the desired current pane."  */
  cur_p = _XMGetPanePtr (menu, *p_num);
  if cur_p.is_null ()
    {
      return XM_FAILURE;
    }
  (*cur_p).activated = (*cur_p).active;

  /* C 203-209: "Find the desired current selection.  If the current selection index is out of
     range a null current selection will be assumed and the cursor will be placed in the current
     pane header."  */
  cur_s = _XMGetSelectionPtr (cur_p, *s_num);

  /* C 211-219: "Compute origin of menu so that cursor is in Correct pane and selection."  */
  _XMTransToOrigin (display,
                    menu,
                    cur_p, cur_s,
                    x_pos, y_pos,
                    &raw mut orig_x, &raw mut orig_y);
  (*menu).x_pos = orig_x;               /* C 220: "Store X and Y coords of menu." */
  (*menu).y_pos = orig_y;

  /* C 223-225: `if (XMenuRecompute (display, menu) == XM_FAILURE) return (XM_FAILURE);`
     `XMenuRecompute` is declared by `XMenu.h` 277, whose mirror `XMenuInt.rs` 42 includes.  */
  if XMenuRecompute (display, menu) == XM_FAILURE
    {
      return XM_FAILURE;
    }

  /* C 227-232: "Flush the window creation queue.  This batches all window creates since lazy
     evaluation is more efficient than individual evaluation.  This routine also does an
     XFlush()."  */
  if _XMWinQueFlush (display, menu, cur_p, cur_s) == _FAILURE
    {
      return XM_FAILURE;
    }

  /* C 237-240: "Make sure windows are in correct order (in case we were passed an already created
     menu in incorrect order.)"  */
  p_ptr = (*(*menu).p_list).next;
  while p_ptr != cur_p
    {
      XRaiseWindow (display, (*p_ptr).window);
      p_ptr = (*p_ptr).next;
    }
  p_ptr = (*(*menu).p_list).prev;
  while p_ptr != (*cur_p).prev
    {
      XRaiseWindow (display, (*p_ptr).window);
      p_ptr = (*p_ptr).prev;
    }

  /* C 246-248: "Make sure all selection windows are mapped."  */
  p_ptr = (*(*menu).p_list).next;
  while p_ptr != (*menu).p_list
    {
      XMapSubwindows (display, (*p_ptr).window);
      p_ptr = (*p_ptr).next;
    }

  /* C 257-263: "Synchronize the X buffers and the event queue.  From here on, all events in the
     queue that don't belong to XMenu are sent back to the application via an application provided
     event handler or discarded if the application has not provided an event handler."  */
  XSync (display, 0);

  /* C 266-268: "Grab the mouse for menu input."  */
  status = XGrabPointer (display,
                         (*menu).parent,
                         True,
                         event_mask,
                         GrabModeAsync,
                         GrabModeAsync,
                         x11_constants::None as Window,   /* X.h 115: `None` is the null Window. */

                         (*menu).mouse_cursor,
                         CurrentTime);
  if status == Success && x_menu_grab_keyboard != 0
    {
      status = XGrabKeyboard (display,
                              (*menu).parent,
                              False,
                              GrabModeAsync,
                              GrabModeAsync,
                              CurrentTime);
      if status != Success
        {
          XUngrabPointer (display, CurrentTime);
        }
    }

  if status == _X_FAILURE
    {
      _XMErrorCode = XME_GRAB_MOUSE;
      return XM_FAILURE;
    }

  /* C 298-300: "Map the menu panes."  */
  XMapWindow (display, (*cur_p).window);
  p_ptr = (*(*menu).p_list).next;
  while p_ptr != cur_p
    {
      XMapWindow (display, (*p_ptr).window);
      p_ptr = (*p_ptr).next;
    }
  p_ptr = (*cur_p).next;
  while p_ptr != (*menu).p_list
    {
      XMapWindow (display, (*p_ptr).window);
      p_ptr = (*p_ptr).next;
    }

  XRaiseWindow (display, (*cur_p).window);      /* C 311-312: "Make sure current */
                                                /* pane is on top." */

  cur_s = core::ptr::null_mut ();               /* C 314: "Clear current selection." */

  /* C 316-318: "Begin event processing loop."  */
  loop
    {
      if let Some (func) = wait_func
        {
          /* C 320: `if (wait_func) (*wait_func) (wait_data);`  */
          (func) (wait_data);
        }
      XNextEvent (display, &raw mut event);     /* C 321: "Get next event." */

      /* C 322: `switch (event.type)` - C's `event.type` is the union's common initial member,
         read through the `XAnyEvent` view of the same storage (P-72 clause 3).  */
      let xany: *mut XAnyEvent = &raw mut event as *mut XAnyEvent;
      match (*xany).r#type
        {
          Expose =>
            {
              let xexpose: *mut XExposeEvent = &raw mut event as *mut XExposeEvent;
              event_xmp = XLookUpAssoc (display,
                                        (*menu).assoc_tab,
                                        (*xexpose).window) as *mut XMPane;
              if event_xmp.is_null ()
                {
                  /* C 328-330: "If AEQ mode is enabled then queue the event."  */
                  if (*menu).aeq != 0
                    {
                      feq_tmp = malloc (core::mem::size_of::<XMEventQue> ()) as *mut XMEventQue;
                      if feq_tmp.is_null ()
                        {
                          _XMErrorCode = XME_CALLOC;
                          return XM_FAILURE;
                        }
                      core::ptr::copy_nonoverlapping (&raw const event,
                                                       &raw mut (*feq_tmp).event, 1);
                      (*feq_tmp).next = feq;
                      feq = feq_tmp;
                    }
                  else if let Some (handler) = _XMEventHandler
                    {
                      (handler) (&raw mut event);
                    }
                  /* C 342: the `break;` that ends this arm.  */
                }
              else
                {
                  if (*event_xmp).activated != 0
                    {
                      XSetWindowBackground (display,
                                            (*event_xmp).window,
                                            (*menu).bkgnd_color);
                    }
                  else
                    {
                      XSetWindowBackgroundPixmap (display,
                                                  (*event_xmp).window,
                                                  (*menu).inact_pixmap);
                    }
                  _XMRefreshPane (display, menu, event_xmp);
                }
            }
          EnterNotify =>
            {
              /* C 356-362: "First wait a small period of time, and see if another EnterNotify
                 event follows hard on the heels of this one. i.e., the user is simply 'passing
                 through'. If so, ignore this one."  C's `break;` out of the `switch` is
                 `break 'arm` out of this labelled arm block (PORTING.md 2.2).  */
              'arm: {
                let xcrossing: *mut XCrossingEvent = &raw mut event as *mut XCrossingEvent;
                event_xmw = XLookUpAssoc (display,
                                          (*menu).assoc_tab,
                                          (*xcrossing).window) as *mut XMWindow;
                if event_xmw.is_null () { break 'arm; }
                if (*event_xmw).r#type == SELECTION
                  {
                    /* C 369-371: "We have entered a selection."; C 372-378's `XPending` pair
                       ("if (XPending (display) == 0) usleep (150000);" is the C comment).  */
                    if XPending (display) != 0
                      {
                        XPeekEvent (display, &raw mut peek_event);
                        let xany: *mut XAnyEvent = &raw mut peek_event as *mut XAnyEvent;
                        if (*xany).r#type == LeaveNotify
                          {
                            break 'arm;
                          }
                      }
                    cur_s = event_xmw as *mut XMSelect;
                    (help_callback.unwrap_unchecked ()) ((*cur_s).help_string,
                                                         (*cur_p).serial, (*cur_s).serial);

                    /* C 383-387: "If the pane we are in is active and the selection entered is
                       active then activate the selection."  */
                    if (*cur_p).active != 0 && (*cur_s).active > 0
                      {
                        (*cur_s).activated = 1;
                        _XMRefreshSelection (display, menu, cur_s);
                      }
                  }
                else
                  {
                    /* C 394-396: "We have entered a pane."  */
                    if XPending (display) != 0
                      {
                        XPeekEvent (display, &raw mut peek_event);
                        let xany: *mut XAnyEvent = &raw mut peek_event as *mut XAnyEvent;
                        if (*xany).r#type == EnterNotify { break 'arm; }
                      }
                    XQueryPointer (display,
                                   (*menu).parent,
                                   &raw mut root, &raw mut child,
                                   &raw mut root_x, &raw mut root_y,
                                   &raw mut win_x, &raw mut win_y,
                                   &raw mut mask);
                    event_xmp = XLookUpAssoc (display,
                                              (*menu).assoc_tab,
                                              child) as *mut XMPane;
                    if event_xmp.is_null () { break 'arm; }
                    if event_xmp == cur_p { break 'arm; }
                    if (*event_xmp).serial > (*cur_p).serial { forward = True; }
                    else { forward = False; }
                    p_ptr = cur_p;
                    while p_ptr != event_xmp
                      {
                        if forward != 0 { p_ptr = (*p_ptr).next; }
                        else { p_ptr = (*p_ptr).prev; }
                        XRaiseWindow (display, (*p_ptr).window);
                      }
                    if (*cur_p).activated != 0
                      {
                        (*cur_p).activated = False;
                        XSetWindowBackgroundPixmap (display,
                                                    (*cur_p).window,
                                                    (*menu).inact_pixmap);
                        _XMRefreshPane (display, menu, cur_p);
                      }
                    if (*event_xmp).active != 0 { (*event_xmp).activated = True; }
                    /* C 429-442: `#if 1` … `#endif` - the arm is live in every configuration, so
                       it is carried as text rather than behind a `cfg` (P-86 clause 1).  */
                    /* C 430-437: "i suspect the we don't get an EXPOSE event when backing store is
                       enabled; the menu windows content is probably not drawn in when it should be
                       in that case.  in that case, this is probably an ugly fix!  i hope someone
                       more familiar with this code would take it from here.  -- caveh@eng.sun.com."  */
                    XSetWindowBackground (display,
                                          (*event_xmp).window,
                                          (*menu).bkgnd_color);
                    _XMRefreshPane (display, menu, event_xmp);
                    cur_p = event_xmp;
                  }
              }
            }
          LeaveNotify =>
            {
              'arm: {
                let xcrossing: *mut XCrossingEvent = &raw mut event as *mut XCrossingEvent;
                event_xmw = XLookUpAssoc (display,
                                          (*menu).assoc_tab,
                                          (*xcrossing).window) as *mut XMWindow;
                if event_xmw.is_null () { break 'arm; }
                if cur_s.is_null () { break 'arm; }

                /* C 455-461: "If the current selection was activated then deactivate it.  Emacs
                   specific, HELP_STRING cannot be validly NULL in the real XMenu library."  */
                (help_callback.unwrap_unchecked ()) (core::ptr::null (),
                                                     (*cur_p).serial, (*cur_s).serial);
                if (*cur_s).activated != 0
                  {
                    (*cur_s).activated = False;
                    _XMRefreshSelection (display, menu, cur_s);
                  }
                cur_s = core::ptr::null_mut ();
              }
            }
          ButtonPress | ButtonRelease =>
            {
              *p_num = (*cur_p).serial;
              /* C 472-474: "Check to see if there is a current selection."  */
              if !cur_s.is_null ()
                {
                  /* C 476-479: "Set the selection number to the current selection."  */
                  *s_num = (*cur_s).serial;
                  /* C 480-484: "If the current selection was activated then we have a valid
                     selection otherwise we have an inactive selection."  */
                  if (*cur_s).activated != 0
                    {
                      *data = (*cur_s).data;
                      ret_val = XM_SUCCESS;
                    }
                  else
                    {
                      ret_val = XM_IA_SELECT;
                    }
                }
              else
                {
                  /* C 494-497: "No selection was current."  */
                  ret_val = XM_NO_SELECT;
                }
              selection = True;
            }
          KeyPress | KeyRelease =>
            {
              /* C's `&event.xkey` and `event.xkey.state` are two reads of the union's `xkey`
                 member; the port views the same storage through a pointer named after the member
                 (P-72 clause 3).  */
              let xkey: *mut XKeyEvent = &raw mut event as *mut XKeyEvent;
              keysym = XLookupKeysym (xkey, 0);

              /* C 505-507: "Pop down on C-g and Escape.  Any escape, ignore modifiers."  */
              if ((keysym == XK_g && ((*xkey).state & ControlMask) != 0)
                  || keysym == XK_Escape)
                {
                  ret_val = XM_NO_SELECT;
                  selection = True;
                }
            }
          #[cfg (feature = "have_xinput2")]
          GenericEvent =>
            {
              /* C 530-532: `case GenericEvent: if (translate_func) translate_func (&event);`.
                 C's `case` sits *after* its `default:` textually (C 529-533); Rust's `match`
                 reaches the first matching arm, so this one is placed before the `_` arm C's
                 `default:` becomes.  Both arms keep their `cfg`, so the frozen reading is
                 unchanged (the ledger's `## Deviations` 6).  */
              if let Some (func) = translate_func
                {
                  (func) (&raw mut event);
                }
            }
          _ =>
            {
              /* C 514-516: "If AEQ mode is enabled then queue the event."  */
              if (*menu).aeq != 0
                {
                  feq_tmp = malloc (core::mem::size_of::<XMEventQue> ()) as *mut XMEventQue;
                  if feq_tmp.is_null ()
                    {
                      _XMErrorCode = XME_CALLOC;
                      return XM_FAILURE;
                    }
                  core::ptr::copy_nonoverlapping (&raw const event,
                                                   &raw mut (*feq_tmp).event, 1);
                  (*feq_tmp).next = feq;
                  feq = feq_tmp;
                }
              else if let Some (handler) = _XMEventHandler
                {
                  (handler) (&raw mut event);
                }
            }
        }
      /* C 535-538: "If a selection has been made, break out of the event loop."  */
      if selection == True { break; }
    }

  /* C 541-543: "Unmap the menu."  */
  p_ptr = (*(*menu).p_list).next;
  while p_ptr != (*menu).p_list
    {
      XUnmapWindow (display, (*p_ptr).window);
      p_ptr = (*p_ptr).next;
    }

  /* C 551-553: "Ungrab the mouse."  */
  XUngrabPointer (display, CurrentTime);
  XUngrabKeyboard (display, CurrentTime);

  /* C 562-564: "If there is a current selection deactivate it."  */
  if !cur_s.is_null () { (*cur_s).activated = 0; }

  /* C 567-569: "Deactivate the current pane."  */
  (*cur_p).activated = 0;
  XSetWindowBackgroundPixmap (display, (*cur_p).window, (*menu).inact_pixmap);

  /* C 573-575: "Synchronize the X buffers and the X event queue."  */
  XSync (display, 0);

  /* C 578-580: "Dispatch any events remaining on the queue."  */
  while XQLength (display) != 0
    {
      /* C 582-584: "Fetch the next event."  */
      XNextEvent (display, &raw mut event);

      /* C 587-609: "Discard any events left on the queue that belong to XMenu.  All others are
         held and then returned to the event queue."  C's `FALLTHROUGH;` (C 608) sends the five
         named cases on into its `default:`, so the port merges them into that one arm (PORTING.md
         2.2); the case names are kept in the pattern, and C's `continue` is Rust's.  */
      let xany: *mut XAnyEvent = &raw mut event as *mut XAnyEvent;
      match (*xany).r#type
        {
          Expose | EnterNotify | LeaveNotify | ButtonPress | ButtonRelease | _ =>
            {
              /* C 597-601: "Does this event belong to one of XMenu's windows?  If so, discard it
                 and process the next event.  If not fall through and treat it as a foreign
                 event."  */
              let xbutton: *mut XButtonEvent = &raw mut event as *mut XButtonEvent;
              event_xmp = XLookUpAssoc (display,
                                        (*menu).assoc_tab,
                                        (*xbutton).window) as *mut XMPane;
              if !event_xmp.is_null () { continue; }

              /* C 610-613: "This is a foreign event.  Queue it for later return to the X event
                 queue."  */
              feq_tmp = malloc (core::mem::size_of::<XMEventQue> ()) as *mut XMEventQue;
              if feq_tmp.is_null ()
                {
                  _XMErrorCode = XME_CALLOC;
                  return XM_FAILURE;
                }
              core::ptr::copy_nonoverlapping (&raw const event,
                                               &raw mut (*feq_tmp).event, 1);
              (*feq_tmp).next = feq;
              feq = feq_tmp;
            }
        }
    }
  /* C 624-626: "Return any foreign events that were queued to the X event queue."  */
  while !feq.is_null ()
    {
      feq_tmp = feq;
      XPutBackEvent (display, &raw mut (*feq_tmp).event);
      feq = (*feq_tmp).next;
      free (feq_tmp as *mut c_char as *mut c_void);
    }

  wait_func = None;                     /* C 634: `wait_func = 0;` - the null function pointer. */

  /* C 636-638: "Return successfully."  */
  _XMErrorCode = XME_NO_ERROR;
  return ret_val;
}
