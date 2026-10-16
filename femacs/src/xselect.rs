// femacs: port of src/xselect.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md
// 0.2); the C `static` helpers stay plain `unsafe fn`s, except the four whose address C hands to
// another function (`x_selection_transfer_timeout`, `wait_for_property_change_unwind`,
// `x_free_selection_data`, `x_cancel_atimer`), which are spelled `unsafe extern "C" fn` (PORTING.md
// P-12: one definition per linkage unit, which is what C's `static` gives too).
//
// **`src/xselect.c` is a P-86 unit**: the frozen configuration compiles nothing from it.  Its
// object belongs to `XOBJ`, and `configure.ac` 7196 fills that variable with
// `xterm.o xfns.o xselect.o xrdb.o xsmfns.o xsettings.o` only under `HAVE_X_WINDOWS`; the macro is
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1720, `build/emacs-ref/src/Makefile`
// 214 is the comment that names the same variable, `docs/baseline/frozen-build-sources.txt` lists
// no `xselect.o`, and the empty `WINDOW_SYSTEM_OBJ=`/`HAIKU_OBJ =`/`MSDOS_OBJ =`/`CYGWIN_OBJ=` of
// that Makefile are the other half of the same fact.  P-86 ① therefore applies: this is
// *transcription*, both arms of every `#if` are carried, and the file gate `have_x_windows` goes on
// every item **and on the include graph** (P-86 ⑧ - an include outside the gate fails the frozen
// build at once, which `build-tu src/xselect.c`'s 336-byte empty object measures).
// `src/xselect.c` *is* in `docs/baseline/config/cond-eval-verified.txt` (422), so `extract-abi.py`
// reads its `#if`s; the file's own body sits under no `#if` of its own (the gate is the build's -
// the `lwlib-Xlw.c` shape), so the checklist is *not* empty and the ledger `docs/ports/xselect.md`
// carries both P-86 ②'s C line-range table and the readings.
//
// The file's four `#if`s beyond the include guards are:
//   * C 24's `#if 10 <= __GNUC__`, whose body is one `# pragma GCC diagnostic ignored
//     "-Wanalyzer-null-dereference"` - the `src/pgtkterm.rs` 69 / `src/timefns.rs` 79 shape: a
//     pragma has no Rust spelling and clang's `__GNUC__` is 4 (macros.tsv 6264), so the guard is
//     false on the frozen target; C's text is kept as a comment and no item is gated on it.
//   * C 30's `#ifdef HAVE_SYS_TYPES_H`, whose body is `#include <sys/types.h>`.  `HAVE_SYS_TYPES_H`
//     is 1 (macros.tsv 1637), so that arm is live and the edge carries
//     `feature = "have_sys_types_h"` (a name this lane introduces and cannot register itself -
//     `## Owed` 1).
//   * C 72's `#ifdef TRACE_SELECTION`, the trace macros.  `TRACE_SELECTION` is undefined in the
//     frozen macro table, so the `(void) 0` arm is the live one and the `fprintf` arm is dead text
//     carried under `feature = "trace_selection"` (P-51; a name this lane introduces - `## Owed` 1).
//   * C 1708's `#if false`, a block C itself deadens in every configuration (the `src/xfns.rs` 958
//     precedent): recorded at its position, no item gated on it.
//   * C 2764 and C 2791's `#ifdef HAVE_XFIXES`.  `HAVE_XFIXES` is `undef` (macros.tsv 1695), so both
//     arms are carried under `feature = "have_xfixes"` (already registered,
//     `femacs/xtask/Cargo.toml` 305) and the `X_INVALID_WINDOW`/`x_find_selection_owner` names of the
//     dead arm are declared at their use sites.
//
// The Xlib/Xproto surface this unit names is declared in the unit at C's `#include` lines (PORTING.md
// P-88 ①: a unit-local segment, *not* a file under `femacs/src/include/`, which mirrors the
// reference's own headers): `<X11/Xlib.h>`'s four event records C reads a member of, its fourteen
// entry points, `<X11/Xatom.h>`'s atoms and `<X11/X.h>`'s constants that `include/xlib.rs`'s
// `x11_constants` module does not carry.  Every layout below is a clang reading from this machine's
// headers (`/opt/homebrew/include`: `xorgproto` 2025.1, libX11; the probe and its output are in the
// ledger's `## Evidence`), not memory (P-88 ②).  That surface has more than one consumer in the tree
// (`src/xterm.c`, `src/xsettings.c`, ...), so P-90 ① wants a dedicated mirror: the ledger's
// `## Owed` 2-4 name the move, and when it lands this segment disappears from here (P-90 ③ /
// P-88 ⑤).
//
// The `XEvent` union `include/xlib.rs` 1164 declares is an opaque 192-byte record whose 33 member
// records are owed in `docs/ports/include-xlib.md`, so C's `reply_base.xselection` / `message.xselection`
// / `event->xclient` / `event->xproperty` member is reached through a conversion to the measured
// records below (P-72 clause 3; the `src/xsettings.rs` 176 and `src/widget.rs` 575 precedents).
//
// Names the mirrors this unit includes do not carry are declared at the use site with the reference
// header's line - one citation per line (`xterm.h`'s entry points, `lisp.h` 4825's
// `record_unwind_protect_void`, `eval.h`'s `internal_condition_case_1`, `process.h`'s
// `wait_reading_process_output`, `xdisp.c`'s `message3_nolog`, `alloc.c`'s `mark_object`), and
// `## Owed` 5-9 name each and the mirror it belongs in.  C's own forward declarations (C 48-68) have
// no Rust counterpart (PORTING.md 0.1-12): they are dropped and the definitions stay at C's
// positions (`## Deviations` 2).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>` (every unit includes it)

/* C 23-26: the GCC-bug-102671 comment and `#if 10 <= __GNUC__` / `# pragma GCC diagnostic ignored
   "-Wanalyzer-null-dereference"` / `#endif`.  A pragma has no Rust spelling and clang's `__GNUC__`
   is 4, so the guard is false on the frozen target; C's text is kept here and no arm is carried (the
   `src/pgtkterm.rs` 69 / `src/timefns.rs` 79 precedent).  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/limits.in.rs");       // C 28: `#include <limits.h>` (INT_MAX, PTRDIFF_MAX)
#[cfg (all (feature = "have_x_windows", feature = "have_sys_types_h"))]
include!("../lib/include/sys_types.in.rs");    // C 31: `#include <sys/types.h>` (C 30's `#ifdef`)
#[cfg (feature = "have_x_windows")]
include!("../lib/include/unistd.in.rs");       // C 34: `#include <unistd.h>` (getpid, for TRACE)
#[cfg (feature = "have_x_windows")]
include!("include/lisp.rs");                   // C 36: `#include "lisp.h"`
#[cfg (feature = "have_x_windows")]
include!("include/xlib.rs");                   // C 37: `xterm.h` -> `<X11/Xlib.h>` (P-90 mirror)
#[cfg (feature = "have_x_windows")]
include!("include/xterm.rs");                  // C 37: `#include "xterm.h"`
#[cfg (feature = "have_x_windows")]
include!("include/frame.rs");                  // C 38: `#include "frame.h"`
#[cfg (feature = "have_x_windows")]
include!("include/blockinput.rs");             // C 39: `#include "blockinput.h"`
#[cfg (feature = "have_x_windows")]
include!("include/sysstdio.rs");               // C 40: `#include "sysstdio.h"` (TRACE_SELECTION)
#[cfg (feature = "have_x_windows")]
include!("include/termhooks.rs");              // C 41: `#include "termhooks.h"`
#[cfg (feature = "have_x_windows")]
include!("include/keyboard.rs");               // C 42: `#include "keyboard.h"`
#[cfg (feature = "have_x_windows")]
include!("include/pdumper.rs");                // C 43: `#include "pdumper.h"`
#[cfg (feature = "have_x_windows")]
include!("include/atimer.rs");                 // C 44: `#include "atimer.h"`
/* C 46: `#include <X11/Xproto.h>`.  Every name it declares that this file reads (`BadWindow`) is in
   the unit-local segment below, so it gets no edge of its own (the `src/xsettings.rs` record).  */
/* C 37's `xterm.h` reaches `<string.h>` through `lisp.h` 30, and this unit calls `memcpy`,
   `memset`, `strcmp` through C 1880/3335/3291 (the `src/xfont.rs` 93 edge).  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`

/* ----------------------------------------------------------------------------------------------
   C 28/30/34/46: the system and X11 SDK surfaces this unit names and no mirror carries.

   `include/limits.in.rs` (C 28) carries `INT_MAX` and `LONG_MAX`; `<unistd.h>`'s `getpid` is in the
   unistd mirror; `<X11/Xproto.h>`'s `BadWindow` and
   the `<X11/Xlib.h>`/`<X11/X.h>`/`<X11/Xatom.h>` names `include/xlib.rs`'s `x11_constants` module
   does not carry stand below, each with its header line.  Every number is a clang reading on this
   machine (the ledger's Evidence ②).  */

/* `SIZE_MAX`/`PTRDIFF_MAX` (C 753/1789/1919) come from `lib/include/stdint.in.rs` through
   `include/lisp.rs` (lisp.h 32's `<inttypes.h>`), so they are not re-declared here (a second
   crate-root `SIZE_MAX` would be E0428).  */

/// C: <X11/X.h> 191: `#define SelectionNotify 31` - the `type` C 1097 writes and 796/840 compare.
#[cfg (feature = "have_x_windows")]
pub const SelectionNotify: c_int = 31;
/// C: <X11/X.h> 445: `#define PropertyNewValue 0` - the `desired_state`/`state` comparison.
#[cfg (feature = "have_x_windows")]
pub const PropertyNewValue: c_int = 0;
/// C: <X11/X.h> 446: `#define PropertyDelete 1` - the same.
#[cfg (feature = "have_x_windows")]
pub const PropertyDelete: c_int = 1;
/// C: <X11/Xproto.h> 110: `#define BadWindow 3L` - C 1051's `error->error_code` comparison.
#[cfg (feature = "have_x_windows")]
pub const BadWindow: c_int = 3;
/// C: <X11/X.h> 233: `#define CurrentTime 0L` - C 1183/1351/3212's sentinel.
#[cfg (feature = "have_x_windows")]
pub const CurrentTime: Time = 0;
/// C: <X11/Xlib.h> 1085: `#define PointerWindow 0L` - C 3294's `wdest` value.
#[cfg (feature = "have_x_windows")]
pub const PointerWindow: Window = 0;
/// C: <X11/Xlib.h> 1086: `#define InputFocus 1L` - C 3296's `wdest` value.
#[cfg (feature = "have_x_windows")]
pub const InputFocus: Window = 1;
/// C: <X11/Xatom.h> 10: `#define XA_SECONDARY ((Atom) 2)` - `symbol_to_x_atom`'s second arm.
#[cfg (feature = "have_x_windows")]
pub const XA_SECONDARY: Atom = 2;
/// C: <X11/Xatom.h> 39: `#define XA_STRING ((Atom) 31)` - the third arm.
#[cfg (feature = "have_x_windows")]
pub const XA_STRING: Atom = 31;
/// C: <X11/Xatom.h> 27: `#define XA_INTEGER ((Atom) 19)` - the fourth arm.
#[cfg (feature = "have_x_windows")]
pub const XA_INTEGER: Atom = 19;

/* `X_LONG_MAX`/`X_LONG_MIN`/`X_ULONG_MAX`/`X_SHRT_MIN`/`X_SHRT_MAX` (C 1791/1860/2294/2378/2384)
   are already crate-root items of `include/xterm.rs` 940-953 (xterm.h 1330-1335), so they are not
   re-declared here.  */
/* C 2795's `X_INVALID_WINDOW` (xterm.h 418) is already a crate-root item of `include/xterm.rs`
   392, so it is not re-declared here (the `xtask probe` reading of the first version measured the
   E0428 that duplicate produced - the ledger's Evidence ③).  */

/// C: <X11/Xlib.h> 841's record opens at 826: `typedef struct { int type; unsigned long serial;
/// Bool send_event; Display *display; Window window; Atom atom; Time time; int state; }
/// XPropertyEvent;` - measured: `sizeof` **64**, `atom`@40, `time`@48, `state`@56.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
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
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XPropertyEvent> () == 64);
  core::assert! (core::mem::offset_of! (XPropertyEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XPropertyEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XPropertyEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XPropertyEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XPropertyEvent, window) == 32);
  core::assert! (core::mem::offset_of! (XPropertyEvent, atom) == 40);
  core::assert! (core::mem::offset_of! (XPropertyEvent, time) == 48);
  core::assert! (core::mem::offset_of! (XPropertyEvent, state) == 56);
};

/// C: <X11/Xlib.h> 876's record opens at 866: `typedef struct { int type; unsigned long serial;
/// Bool send_event; Display *display; Window requestor; Atom selection; Atom target; Atom property;
/// Time time; } XSelectionEvent;` - measured: `sizeof` **72**, `requestor`@32, `selection`@40,
/// `target`@48, `property`@56, `time`@64.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XSelectionEvent
{
  pub r#type: c_int,              // Xlib.h 870
  pub serial: c_ulong,            // Xlib.h 871
  pub send_event: Bool,           // Xlib.h 872
  pub display: *mut Display,      // Xlib.h 873
  pub requestor: Window,          // Xlib.h 874
  pub selection: Atom,            // Xlib.h 875
  pub target: Atom,               // Xlib.h 876
  pub property: Atom,             // Xlib.h 877
  pub time: Time,                 // Xlib.h 878
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XSelectionEvent> () == 72);
  core::assert! (core::mem::offset_of! (XSelectionEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XSelectionEvent, serial) == 8);
  core::assert! (core::mem::offset_of! (XSelectionEvent, send_event) == 16);
  core::assert! (core::mem::offset_of! (XSelectionEvent, display) == 24);
  core::assert! (core::mem::offset_of! (XSelectionEvent, requestor) == 32);
  core::assert! (core::mem::offset_of! (XSelectionEvent, selection) == 40);
  core::assert! (core::mem::offset_of! (XSelectionEvent, target) == 48);
  core::assert! (core::mem::offset_of! (XSelectionEvent, property) == 56);
  core::assert! (core::mem::offset_of! (XSelectionEvent, time) == 64);
};

/// C: <X11/Xlib.h> 906's record opens at 895: `typedef struct { int type; unsigned long serial;
/// Bool send_event; Display *display; Window window; Atom message_type; int format; union { char
/// b[20]; short s[10]; long l[5]; } data; } XClientMessageEvent;` - measured: `sizeof` **96**,
/// `message_type`@40, `format`@48, `data`@56.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
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
/// C: <X11/Xlib.h> 908-911: `union { char b[20]; short s[10]; long l[5]; } data;` - measured
/// `sizeof` **40** (the `long l[5]` member).
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub union _XClientMessageEvent_data
{
  pub b: [c_char; 20],      // Xlib.h 909
  pub s: [c_short; 10],     // Xlib.h 910
  pub l: [c_long; 5],       // Xlib.h 911
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XClientMessageEvent> () == 96);
  core::assert! (core::mem::offset_of! (XClientMessageEvent, message_type) == 40);
  core::assert! (core::mem::offset_of! (XClientMessageEvent, format) == 48);
  core::assert! (core::mem::offset_of! (XClientMessageEvent, data) == 56);
  core::assert! (core::mem::size_of::<_XClientMessageEvent_data> () == 40);
};

/// C: <X11/Xlib.h> 928's record opens at 918: `typedef struct { int type; Display *display; XID
/// resourceid; unsigned long serial; unsigned char error_code; unsigned char request_code; unsigned
/// char minor_code; } XErrorEvent;` - measured: `sizeof` **40**, `error_code`@32.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct XErrorEvent
{
  pub r#type: c_int,                // Xlib.h 922
  pub display: *mut Display,        // Xlib.h 923
  pub resourceid: XID,              // Xlib.h 924
  pub serial: c_ulong,              // Xlib.h 925
  pub error_code: c_uchar,          // Xlib.h 926
  pub request_code: c_uchar,        // Xlib.h 927
  pub minor_code: c_uchar,          // Xlib.h 928
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<XErrorEvent> () == 40);
  core::assert! (core::mem::offset_of! (XErrorEvent, r#type) == 0);
  core::assert! (core::mem::offset_of! (XErrorEvent, display) == 8);
  core::assert! (core::mem::offset_of! (XErrorEvent, resourceid) == 16);
  core::assert! (core::mem::offset_of! (XErrorEvent, serial) == 24);
  core::assert! (core::mem::offset_of! (XErrorEvent, error_code) == 32);
  core::assert! (core::mem::offset_of! (XErrorEvent, request_code) == 33);
  core::assert! (core::mem::offset_of! (XErrorEvent, minor_code) == 34);
};

/* The Xlib entry points this file calls that `include/xlib.rs` does not carry (its note: ten Xlib
   procedures).  Each keeps C's prototype in its `/// C:` line; every layout/width above is a clang
   reading on this machine.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: Xlib.h 1496: `extern char *XGetAtomName (Display*, Atom);`
  pub fn XGetAtomName (display: *mut Display, atom: Atom) -> *mut c_char;
  /// C: Xlib.h 1532: `extern Atom XInternAtom (Display*, _Xconst char*, Bool only_if_exists);`
  pub fn XInternAtom (display: *mut Display, atom_name: *const c_char,
                      only_if_exists: Bool) -> Atom;
  /// C: Xlib.h 1628: `extern Window XGetSelectionOwner (Display*, Atom);`
  pub fn XGetSelectionOwner (display: *mut Display, selection: Atom) -> Window;
  /// C: Xlib.h 1709: `extern long XMaxRequestSize (Display*);`
  pub fn XMaxRequestSize (display: *mut Display) -> c_long;
  /// C: Xlib.h 1712: `extern long XExtendedMaxRequestSize (Display*);`
  pub fn XExtendedMaxRequestSize (display: *mut Display) -> c_long;
  /// C: Xlib.h 2078: `extern int XChangeProperty (Display*, Window, Atom property, Atom type,
  /// int format, int mode, _Xconst unsigned char*, int nelements);`
  pub fn XChangeProperty (display: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar,
                          nelements: c_int) -> c_int;
  /// C: Xlib.h 2185: `extern int XConvertSelection (Display*, Atom, Atom target, Atom property,
  /// Window requestor, Time time);`
  pub fn XConvertSelection (display: *mut Display, selection: Atom, target: Atom,
                            property: Atom, requestor: Window, time: Time) -> c_int;
  /// C: Xlib.h 2247: `extern int XDeleteProperty (Display*, Window, Atom property);`
  pub fn XDeleteProperty (display: *mut Display, w: Window, property: Atom) -> c_int;
  /// C: Xlib.h 2517: `extern int XFlush (Display*);`
  pub fn XFlush (display: *mut Display) -> c_int;
  /// C: Xlib.h 2526: `extern int XFree (void*);`
  pub fn XFree (data: *mut c_void) -> c_int;
  /// C: Xlib.h 2686: `extern int XGetWindowProperty (Display*, Window, Atom property,
  /// long long_offset, long long_length, Bool delete, Atom req_type, Atom *actual_type_return,
  /// int *actual_format_return, unsigned long *nitems_return,
  /// unsigned long *bytes_after_return, unsigned char **prop_return);`
  pub fn XGetWindowProperty (display: *mut Display, w: Window, property: Atom,
                             long_offset: c_long, long_length: c_long, delete: Bool,
                             req_type: Atom, actual_type_return: *mut Atom,
                             actual_format_return: *mut c_int,
                             nitems_return: *mut c_ulong,
                             bytes_after_return: *mut c_ulong,
                             prop_return: *mut *mut c_uchar) -> c_int;
  /// C: Xlib.h 3139: `extern int XSelectInput (Display*, Window, long event_mask);`
  pub fn XSelectInput (display: *mut Display, w: Window, event_mask: c_long) -> c_int;
  /// C: Xlib.h 3145: `extern Status XSendEvent (Display*, Window, Bool propagate, long event_mask,
  /// XEvent *event_send);`
  pub fn XSendEvent (display: *mut Display, w: Window, propagate: Bool, event_mask: c_long,
                     event_send: *mut XEvent) -> c_int;
  /// C: Xlib.h 3302: `extern int XSetSelectionOwner (Display*, Atom selection, Window owner,
  /// Time time);`
  pub fn XSetSelectionOwner (display: *mut Display, selection: Atom, owner: Window,
                             time: Time) -> c_int;
}

/* C 48-49: `struct prop_location;` and `struct selection_data;` - C's forward declarations have no
   Rust counterpart (PORTING.md 0.1-12), so both tags are declared where C defines them (C 487 and
   C 585) and only these two lines are recorded here.  */
/* C 51-68: the eleven `static` prototypes.  Dropped for the same reason (PORTING.md 0.1-12); the
   definitions stay at C's positions below.  */

/* C 70-86: C's `Printing traces to stderr.` banner and the TRACE0-TRACE3 pair under
   `#ifdef TRACE_SELECTION`.  `TRACE_SELECTION` is undefined in the frozen macro table, so the
   `(void) 0` arm is live; the `fprintf` arm is dead text carried under `feature = "trace_selection"`
   (P-51) and it names `stderr`/`fprintf` (the sysstdio mirror) and `PRIdMAX`/`getpid`.  */
/// C: <inttypes.h> 443: `# define PRIdMAX "ld"` - measured: `intmax_t` is `long long` on this
/// target, so the frozen spelling is `"j" "d"` (`docs/ports/dbusbind.rs` 765's reading).
#[cfg (feature = "have_x_windows")]
macro_rules! PRIdMAX { () => { "jd" } }
/// C: lisp.h 150: `#define pD "t"` - the length modifier of the three `%"pD"d` format strings this
/// file's trace calls carry (C 1854, 1967, 1971).  The lisp.rs mirror carries `pD` as a `&str`
/// constant, which `concat!` cannot take, so the macro form is declared here (the
/// `docs/ports/dbusbind.rs` 769 `PRIdMAX!` shape - `## Owed` 10).
#[cfg (feature = "have_x_windows")]
macro_rules! pD { () => { "t" } }
/* The `(void) 0` arm C 82-85 carries is the **live** one (`TRACE_SELECTION` is undefined),
   and it is written **first** here: `extract-abi.py`'s `macro_body_of` reads a unit's first
   `macro_rules!` definition of a name, so putting the `fprintf` arm (C 73-80) first would
   hide the live body's `0` token from the body check.  Both arms are carried (P-51).  */
#[cfg (feature = "have_x_windows")]
macro_rules! TRACE0 { ($fmt:expr) => { 0 }; }
#[cfg (feature = "have_x_windows")]
macro_rules! TRACE1 { ($fmt:expr, $a0:expr) => { 0 }; }
#[cfg (feature = "have_x_windows")]
macro_rules! TRACE2 { ($fmt:expr, $a0:expr, $a1:expr) => { 0 }; }
#[cfg (feature = "have_x_windows")]
macro_rules! TRACE3 { ($fmt:expr, $a0:expr, $a1:expr, $a2:expr) => { 0 }; }
/* C 73-80: the `#ifdef TRACE_SELECTION` arm - dead text carried under its own feature.  */
#[cfg (all (feature = "have_x_windows", feature = "trace_selection"))]
macro_rules! TRACE0 {
  ($fmt:expr) => {
    fprintf (stderr, concat! ("%", PRIdMAX! (), ": ", $fmt, "\n").as_ptr () as *const c_char,
             getpid () as i64)
  };
}
#[cfg (all (feature = "have_x_windows", feature = "trace_selection"))]
macro_rules! TRACE1 {
  ($fmt:expr, $a0:expr) => {
    fprintf (stderr, concat! ("%", PRIdMAX! (), ": ", $fmt, "\n").as_ptr () as *const c_char,
             getpid () as i64, $a0)
  };
}
#[cfg (all (feature = "have_x_windows", feature = "trace_selection"))]
macro_rules! TRACE2 {
  ($fmt:expr, $a0:expr, $a1:expr) => {
    fprintf (stderr, concat! ("%", PRIdMAX! (), ": ", $fmt, "\n").as_ptr () as *const c_char,
             getpid () as i64, $a0, $a1)
  };
}
#[cfg (all (feature = "have_x_windows", feature = "trace_selection"))]
macro_rules! TRACE3 {
  ($fmt:expr, $a0:expr, $a1:expr, $a2:expr) => {
    fprintf (stderr, concat! ("%", PRIdMAX! (), ": ", $fmt, "\n").as_ptr () as *const c_char,
             getpid () as i64, $a0, $a1, $a2)
  };
}

/* C's `max`/`min` (lisp.h 71-72) reach this file as macros over any integer type; the port's
   lisp.rs mirror carries them as ordinary `fn`s over `isize`, which would truncate the `c_long`
/`usize` folds C 104-120 makes.  The same folds are therefore defined here as macros of the same
   name (a macro and a function share a name in Rust) - the `src/alloc.rs` 124 shape.  */
#[cfg (feature = "have_x_windows")]
macro_rules! max {
  ($a:expr, $b:expr) => { if ($a) > ($b) { ($a) } else { ($b) } };
}
#[cfg (feature = "have_x_windows")]
macro_rules! min {
  ($a:expr, $b:expr) => { if ($a) < ($b) { ($a) } else { ($b) } };
}

/* C 88-89: the comment on `X_LONG_SIZE`.  */
/// C 90: `#define X_LONG_SIZE 4`
#[cfg (feature = "have_x_windows")]
pub const X_LONG_SIZE: c_int = 4;

/* C 92-103: the `MAX_SELECTION_QUANTUM` comment.  */
/// C 104-106: `#define MAX_SELECTION_QUANTUM ((int) min (0xFFFFFF, (min (INT_MAX, min (PTRDIFF_MAX,
/// SIZE_MAX) - 1) / max (X_LONG_SIZE, sizeof (long)))))`
#[cfg (feature = "have_x_windows")]
macro_rules! MAX_SELECTION_QUANTUM {
  () => {
    (min! (0xFFFFFF,
           (min! (INT_MAX as usize,
                  (min! (PTRDIFF_MAX as usize, SIZE_MAX) - 1))
            / max! (X_LONG_SIZE as usize, std::mem::size_of::<c_long> ()))) as c_int)
  };
}

/* C 108-121: `selection_quantum`.  */
#[cfg (feature = "have_x_windows")]
unsafe fn selection_quantum (display: *mut Display) -> c_int          // C 108-121
{
  let mut mrs: c_long;

  mrs = XExtendedMaxRequestSize (display);

  if mrs == 0
    { mrs = XMaxRequestSize (display); }

  return if mrs < (MAX_SELECTION_QUANTUM! () / X_LONG_SIZE) as c_long + 25
    { ((mrs - 25) * X_LONG_SIZE as c_long) as c_int }
    else
    { MAX_SELECTION_QUANTUM! () };
}

/// C 123-124: `#define LOCAL_SELECTION(selection_symbol, dpyinfo)
/// assq_no_quit (selection_symbol, dpyinfo->terminal->Vselection_alist)`
#[cfg (feature = "have_x_windows")]
macro_rules! LOCAL_SELECTION {
  ($selection_symbol:expr, $dpyinfo:expr) => {
    assq_no_quit ($selection_symbol, (*(*$dpyinfo).terminal).Vselection_alist)
  };
}

/* The `xterm.h` entry points C 37's include brings and `include/xterm.rs` does not carry (its own
   note: it declares the records and statics, and declares no entry point).  Declared here at their
   first use site, each with the reference header's line (P-88 ①); the ledger's `## Owed` 5 names
   the move into the mirror.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: xterm.h 1841: `extern Atom x_intern_cached_atom (struct x_display_info *, const char *,
  /// bool);`
  pub fn x_intern_cached_atom (dpyinfo: *mut x_display_info, name: *const c_char,
                               only_if_exists: bool) -> Atom;
  /// C: xterm.h 1752: `extern void x_catch_errors (Display *);`
  pub fn x_catch_errors (display: *mut Display);
  /// C: xterm.h 1764: `extern void x_uncatch_errors (void);`
  pub fn x_uncatch_errors ();
  /// C: xterm.h 1844: `extern char *x_get_atom_name (struct x_display_info *, Atom, bool *);`
  pub fn x_get_atom_name (dpyinfo: *mut x_display_info, atom: Atom,
                          need_sync: *mut bool) -> *mut c_char;
  /// C: xterm.h 1762: `extern bool x_had_errors_p (Display *);`
  pub fn x_had_errors_p (display: *mut Display) -> bool;
  /// C: xterm.h 1765: `extern void x_uncatch_errors_after_check (void);`
  pub fn x_uncatch_errors_after_check ();
  /// C: xterm.h 1003: `extern struct x_display_info *x_display_info_for_display (Display *);`
  pub fn x_display_info_for_display (display: *mut Display) -> *mut x_display_info;
  /// C: xterm.h 1766: `extern void x_ignore_errors_for_next_request (struct x_display_info *,
  /// unsigned int);`
  pub fn x_ignore_errors_for_next_request (dpyinfo: *mut x_display_info, serial: c_uint);
  /// C: xterm.h 1768: `extern void x_stop_ignoring_errors (struct x_display_info *);`
  pub fn x_stop_ignoring_errors (dpyinfo: *mut x_display_info);
  /// C: xterm.h 1760: `extern void x_check_errors (Display *, const char *);`
  pub fn x_check_errors (display: *mut Display, format: *const c_char);
  /// C: xterm.h 1755: `extern void x_catch_errors_for_lisp (struct x_display_info *);`
  pub fn x_catch_errors_for_lisp (dpyinfo: *mut x_display_info);
  /// C: xterm.h 1757: `extern void x_check_errors_for_lisp (struct x_display_info *, const char *);`
  pub fn x_check_errors_for_lisp (dpyinfo: *mut x_display_info, format: *const c_char);
  /// C: xterm.h 1756: `extern void x_uncatch_errors_for_lisp (struct x_display_info *);`
  pub fn x_uncatch_errors_for_lisp (dpyinfo: *mut x_display_info);
  /// C: xterm.h 1803: `extern void x_wait_for_cell_change (Lisp_Object, struct timespec);`
  pub fn x_wait_for_cell_change (cell: Lisp_Object, timeout: timespec);
  /// C: xterm.h 1843: `extern void x_intern_atoms (struct x_display_info *, char **, int, Atom *);`
  pub fn x_intern_atoms (dpyinfo: *mut x_display_info, names: *mut *mut c_char, n: c_int,
                         atoms: *mut Atom);
  /// C: xterm.h 1737: `extern void x_relative_mouse_position (struct frame *, int *, int *);`
  pub fn x_relative_mouse_position (f: *mut frame, x: *mut c_int, y: *mut c_int);
  /// C: xterm.h 1833: `extern void x_translate_coordinates (struct frame *, int, int, int *,
  /// int *);`
  pub fn x_translate_coordinates (f: *mut frame, src_x: c_int, src_y: c_int, dest_x: *mut c_int,
                                  dest_y: *mut c_int);
  /// C: xterm.h 1967: `extern void x_preserve_selections (struct x_display_info *, Lisp_Object,
  /// Lisp_Object);`
  pub fn x_preserve_selections (dpyinfo: *mut x_display_info, selections: Lisp_Object,
                                frame: Lisp_Object);
  /// C: dispextern.h 3993: `extern Display_Info *check_x_display_info (Lisp_Object);`
  /// (`include/dispextern.rs` carries the `Display_Info` alias and not this prototype.)
  pub fn check_x_display_info (object: Lisp_Object) -> *mut x_display_info;
  /// C: frame.h 1475: `extern struct frame *decode_window_system_frame (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: frame.h 1475-1476: `extern struct frame *decode_live_frame (Lisp_Object);`
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  /// (`include/termhooks.rs` 444 records that this prototype is declared at its use sites.)
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: frame.h 1504: `extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);`
  pub fn check_window_system (f: *mut frame);
  /* C 72's XFIXES arm calls `x_find_selection_owner` (xterm.h 1822) - declared under that gate
     only, because `xterm.h`'s own declaration is inside `#ifndef USE_GTK`.  */
  #[cfg (feature = "have_xfixes")]
  pub fn x_find_selection_owner (dpyinfo: *mut x_display_info, selection: Atom) -> Window;
}

/* C 128-131: symbol_to_x_atom's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol_to_x_atom (dpyinfo: *mut x_display_info, sym: Lisp_Object) -> Atom
{
  use crate::x11_constants::{XA_ATOM, XA_PRIMARY};

  let mut val: Atom;                                            // C 134
  if NILP (sym)
    { return 0; }
  if EQ (sym, QPRIMARY ())
    { return XA_PRIMARY; }
  if EQ (sym, QSECONDARY ())
    { return XA_SECONDARY; }
  if EQ (sym, QSTRING ())
    { return XA_STRING; }
  if EQ (sym, QINTEGER ())
    { return XA_INTEGER; }
  if EQ (sym, QATOM ())
    { return XA_ATOM; }
  if EQ (sym, QCLIPBOARD ())
    { return (*dpyinfo).Xatom_CLIPBOARD; }
  if EQ (sym, QTIMESTAMP ())
    { return (*dpyinfo).Xatom_TIMESTAMP; }
  if EQ (sym, QTEXT ())
    { return (*dpyinfo).Xatom_TEXT; }
  if EQ (sym, QCOMPOUND_TEXT ())
    { return (*dpyinfo).Xatom_COMPOUND_TEXT; }
  if EQ (sym, QUTF8_STRING ())
    { return (*dpyinfo).Xatom_UTF8_STRING; }
  if EQ (sym, QDELETE ())
    { return (*dpyinfo).Xatom_DELETE; }
  if EQ (sym, QMULTIPLE ())
    { return (*dpyinfo).Xatom_MULTIPLE; }
  if EQ (sym, QINCR ())
    { return (*dpyinfo).Xatom_INCR; }
  if EQ (sym, Q_EMACS_TMP_ ())
    { return (*dpyinfo).Xatom_EMACS_TMP; }
  if EQ (sym, QTARGETS ())
    { return (*dpyinfo).Xatom_TARGETS; }
  if EQ (sym, QNULL ())
    { return (*dpyinfo).Xatom_NULL; }
  if EQ (sym, QXdndSelection ())
    { return (*dpyinfo).Xatom_XdndSelection; }
  if EQ (sym, QXmTRANSFER_SUCCESS ())
    { return (*dpyinfo).Xatom_XmTRANSFER_SUCCESS; }
  if EQ (sym, QXmTRANSFER_FAILURE ())
    { return (*dpyinfo).Xatom_XmTRANSFER_FAILURE; }
  if EQ (sym, QXdndDirectSave0 ())
    { return (*dpyinfo).Xatom_XdndDirectSave0; }
  if EQ (sym, Qtext_plain ())
    { return (*dpyinfo).Xatom_text_plain; }
  if EQ (sym, QXdndActionDirectSave ())
    { return (*dpyinfo).Xatom_XdndActionDirectSave; }

  if !SYMBOLP (sym)                                             // C 182
    { emacs_abort (); }

  TRACE1! (" XInternAtom %s", SSDATA (SYMBOL_NAME (sym)));      // C 185
  block_input ();
  val = x_intern_cached_atom (dpyinfo, SSDATA (SYMBOL_NAME (sym)), false);
  unblock_input ();
  return val;
}

/* C 193-194: x_atom_to_symbol's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_atom_to_symbol (dpyinfo: *mut x_display_info, atom: Atom) -> Lisp_Object
{
  use crate::x11_constants::{XA_ATOM, XA_PRIMARY};

  let mut str_: *mut c_char;                                    // C 199
  let val: Lisp_Object;

  if atom == 0                                                 // C 202
    { return Qnil (); }

  match atom                                                   // C 205-217
    {
      XA_PRIMARY => { return QPRIMARY (); }
      XA_SECONDARY => { return QSECONDARY (); }
      XA_STRING => { return QSTRING (); }
      XA_INTEGER => { return QINTEGER (); }
      XA_ATOM => { return QATOM (); }
      _ => { }
    }

  if dpyinfo.is_null ()                                        // C 219
    { return Qnil (); }
  if atom == (*dpyinfo).Xatom_CLIPBOARD
    { return QCLIPBOARD (); }
  if atom == (*dpyinfo).Xatom_TIMESTAMP
    { return QTIMESTAMP (); }
  if atom == (*dpyinfo).Xatom_TEXT
    { return QTEXT (); }
  if atom == (*dpyinfo).Xatom_COMPOUND_TEXT
    { return QCOMPOUND_TEXT (); }
  if atom == (*dpyinfo).Xatom_UTF8_STRING
    { return QUTF8_STRING (); }
  if atom == (*dpyinfo).Xatom_DELETE
    { return QDELETE (); }
  if atom == (*dpyinfo).Xatom_MULTIPLE
    { return QMULTIPLE (); }
  if atom == (*dpyinfo).Xatom_INCR
    { return QINCR (); }
  if atom == (*dpyinfo).Xatom_EMACS_TMP
    { return Q_EMACS_TMP_ (); }
  if atom == (*dpyinfo).Xatom_TARGETS
    { return QTARGETS (); }
  if atom == (*dpyinfo).Xatom_NULL
    { return QNULL (); }
  if atom == (*dpyinfo).Xatom_XdndSelection
    { return QXdndSelection (); }
  if atom == (*dpyinfo).Xatom_XmTRANSFER_SUCCESS
    { return QXmTRANSFER_SUCCESS (); }
  if atom == (*dpyinfo).Xatom_XmTRANSFER_FAILURE
    { return QXmTRANSFER_FAILURE (); }
  if atom == (*dpyinfo).Xatom_XdndDirectSave0
    { return QXdndDirectSave0 (); }
  if atom == (*dpyinfo).Xatom_text_plain
    { return Qtext_plain (); }
  if atom == (*dpyinfo).Xatom_XdndActionDirectSave
    { return QXdndActionDirectSave (); }

  x_catch_errors ((*dpyinfo).display);                          // C 256
  str_ = x_get_atom_name (dpyinfo, atom, std::ptr::null_mut ());
  x_uncatch_errors ();

  TRACE0! ("XGetAtomName --> NULL");                            // C 260
  if str_.is_null ()
    { return Qnil (); }
  TRACE1! ("XGetAtomName --> %s", str_);

  val = intern (str_);                                          // C 265
  xfree (str_ as *mut c_void);
  return val;
}

/* C 270-277: x_own_selection's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_own_selection (selection_name: Lisp_Object, selection_value: Lisp_Object,
                                          frame: Lisp_Object, mut dnd_data: Lisp_Object,
                                          mut timestamp: Time)
{
  let f: *mut frame = XFRAME (frame);                          // C 283
  let selecting_window: Window = FRAME_X_WINDOW! (f);
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let display: *mut Display = (*dpyinfo).display;
  let selection_atom: Atom = symbol_to_x_atom (dpyinfo, selection_name);

  if timestamp == 0                                             // C 289
    { timestamp = (*dpyinfo).last_user_time; }

  block_input ();                                               // C 292
  XSetSelectionOwner (display, selection_atom, selecting_window, timestamp);
  unblock_input ();

  /* Now update the local cache */                               // C 297
  {
    let selection_data: Lisp_Object;
    let mut prev_value: Lisp_Object;

    prev_value = LOCAL_SELECTION! (selection_name, dpyinfo);    // C 302

    /* If the previous value contains alternative data for responding to
       programs that do not understand a specialized drag-and-drop
       protocol and DND_DATA is unset, transfer it to the new value.  */
    if NILP (dnd_data) && !NILP (prev_value)                    // C 307
      { dnd_data = XCAR (XCDR (XCDR (XCDR (XCDR (prev_value))))); }

    selection_data = list5 (selection_name, selection_value,    // C 310
                            INT_TO_INTEGER (timestamp as EMACS_INT), frame,
                            dnd_data);

    tset_selection_alist                                        // C 314
      ((*dpyinfo).terminal,
       Fcons (selection_data, (*(*dpyinfo).terminal).Vselection_alist));

    /* If we already owned the selection, remove the old selection
       data.  Don't use Fdelq as that may quit.  */
    if !NILP (prev_value)                                       // C 320
      {
        /* We know it's not the CAR, so it's easy.  */
        let mut rest: Lisp_Object = (*(*dpyinfo).terminal).Vselection_alist;
        while CONSP (rest)                                      // C 324
          {
            if EQ (prev_value, CAR (XCDR (rest)))
              {
                XSETCDR (rest, XCDR (XCDR (rest)));
                break;
              }
            rest = XCDR (rest);
          }
      }
  }
}
/* C 334-347: x_get_local_selection's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_get_local_selection (selection_symbol: Lisp_Object, target_type: Lisp_Object,
                                 local_request: bool, dpyinfo: *mut x_display_info,
                                 mut local_value: Lisp_Object,
                                 need_alternate: bool) -> Lisp_Object      // C 349-443
{
  let mut tem: Lisp_Object = Qnil ();          // C 354 (uninitialized in C; PORTING.md 1.5)
  let mut handler_fn: Lisp_Object = Qnil ();   // C 355
  let mut value: Lisp_Object = Qnil ();        // C 355
  let mut check: Lisp_Object = Qnil ();        // C 355
  let mut may_quit: bool;
  let mut count: specpdl_ref = SPECPDL_INDEX ();       // C 357 (assigned at C 380)

  may_quit = false;                                             // C 359

  if NILP (local_value)                                         // C 361
    { local_value = LOCAL_SELECTION! (selection_symbol, dpyinfo); }
  else
    { may_quit = true; }

  if NILP (local_value)                                         // C 366
    { return Qnil (); }

  /* TIMESTAMP is a special case.  */                           // C 369
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
      count = SPECPDL_INDEX ();                                 // C 380

      if !may_quit                                               // C 382
	{ specbind (Qinhibit_quit (), Qt ()); }

      CHECK_SYMBOL (target_type);                               // C 385
      handler_fn = CDR (Fassq (target_type, Vselection_converter_alist));

      if CONSP (handler_fn)                                     // C 388
	{ handler_fn = XCDR (handler_fn); }

      if !need_alternate                                        // C 391
	{ tem = XCAR (XCDR (local_value)); }
      else
	{ tem = XCAR (XCDR (XCDR (XCDR (XCDR (local_value))))); }

      if STRINGP (tem)                                          // C 396
	{
	  local_value = Fget_text_property (make_fixnum (0),
					    target_type, tem);

	  if !NILP (local_value)                                // C 401
	    { tem = local_value; }
	}

      if !NILP (handler_fn)                                     // C 405
	{ value = calln! (handler_fn, selection_symbol,
			  if local_request && NILP (Vx_treat_local_requests_remotely)
			    { Qnil () }
			    else
			    { target_type },
			  tem); }
      else
	{ value = Qnil (); }
      value = unbind_to (count, value);                         // C 414
    }

  /* Make sure this value is of a type that we could transmit
     to another X client.  */

  check = value;                                                // C 420
  if CONSP (value)
      && SYMBOLP (XCAR (value))
    { check = XCDR (value); }

  if STRINGP (check)                                            // C 425
      || VECTORP (check)
      || SYMBOLP (check)
      || INTEGERP (check)
      || NILP (value)
    { return value; }
  /* Check for a value that CONS_TO_INTEGER could handle.  */
  else if CONSP (check)                                         // C 432
	   && INTEGERP (XCAR (check))
	   && (INTEGERP (XCDR (check))
	       || (CONSP (XCDR (check))
		   && INTEGERP (XCAR (XCDR (check)))
		   && NILP (XCDR (XCDR (check)))))
    { return value; }

  signal_error (c"Invalid data returned by selection-conversion function".as_ptr (),
		list2 (handler_fn, value));                     // C 441
}

/* C 445-448: the "Subroutines of x_reply_selection_request" banner and
   x_decline_selection_request's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_decline_selection_request (event: *mut selection_input_event)  // C 450-483
{
  use crate::x11_constants::None;

  let mut reply_base: XEvent = std::mem::zeroed ();             // C 453
  let reply: *mut XSelectionEvent;
  let dpy: *mut Display;
  let dpyinfo: *mut x_display_info;

  reply = &raw mut reply_base as *mut XSelectionEvent;          // C 458 (C's `reply_base.xselection`)
  dpy = SELECTION_EVENT_DISPLAY (event);                        // C 459
  dpyinfo = x_display_info_for_display (dpy);

  if dpyinfo.is_null ()                                         // C 462
    { return; }

  (*reply).r#type = SelectionNotify;                            // C 465
  (*reply).display = dpy;
  (*reply).requestor = SELECTION_EVENT_REQUESTOR! (event);
  (*reply).selection = SELECTION_EVENT_SELECTION! (event);
  (*reply).time = SELECTION_EVENT_TIME! (event);
  (*reply).target = SELECTION_EVENT_TARGET! (event);
  (*reply).property = None as Atom;

  /* The reason for the error may be that the receiver has
     died in the meantime.  Handle that case.  */
  block_input ();                                               // C 475
  x_ignore_errors_for_next_request (dpyinfo, 0);
  XSendEvent ((*dpyinfo).display, (*reply).requestor,
	      false as Bool, 0, &raw mut reply_base);
  x_stop_ignoring_errors (dpyinfo);

  XFlush ((*dpyinfo).display);                                  // C 481
  unblock_input ();
}

/* C 485-486: the "Raw selection data" comment.  */

/// C 487-513: `struct selection_data`.  Measured (the ledger's Evidence ②): `sizeof` **56**,
/// `data`@0, `string`@8, `size`@16, `format`@24, `type`@32, `property`@40, `next`@48.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct selection_data
{
  pub data: *mut c_uchar,                 // C 490 - "Pointer to the selection data."
  pub string: Lisp_Object,                // C 495
  pub size: ptrdiff_t,                    // C 499
  pub format: c_int,                      // C 502
  pub r#type: Atom,                       // C 505
  pub property: Atom,                     // C 508
  pub next: *mut selection_data,          // C 512
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<selection_data> () == 56);
  core::assert! (core::mem::offset_of! (selection_data, data) == 0);
  core::assert! (core::mem::offset_of! (selection_data, string) == 8);
  core::assert! (core::mem::offset_of! (selection_data, size) == 16);
  core::assert! (core::mem::offset_of! (selection_data, format) == 24);
  core::assert! (core::mem::offset_of! (selection_data, r#type) == 32);
  core::assert! (core::mem::offset_of! (selection_data, property) == 40);
  core::assert! (core::mem::offset_of! (selection_data, next) == 48);
};

/* C 515-517: the `struct transfer` comment.  */

/// C 518-544: `struct transfer`.  Measured: `sizeof` **120**, `requestor`@0, `offset`@8,
/// `items_per_request`@16, `dpyinfo`@24, `data`@32, `next`@88, `last`@96, `timeout`@104,
/// `serial`@112, `flags`@116.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct transfer
{
  pub requestor: Window,                  // C 521
  pub offset: ptrdiff_t,                  // C 525
  pub items_per_request: ptrdiff_t,       // C 525
  pub dpyinfo: *mut x_display_info,       // C 528
  pub data: selection_data,               // C 531
  pub next: *mut transfer,                // C 534
  pub last: *mut transfer,                // C 534
  pub timeout: *mut atimer,               // C 537
  pub serial: c_uint,                     // C 540
  pub flags: c_int,                       // C 543
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<transfer> () == 120);
  core::assert! (core::mem::offset_of! (transfer, requestor) == 0);
  core::assert! (core::mem::offset_of! (transfer, offset) == 8);
  core::assert! (core::mem::offset_of! (transfer, items_per_request) == 16);
  core::assert! (core::mem::offset_of! (transfer, dpyinfo) == 24);
  core::assert! (core::mem::offset_of! (transfer, data) == 32);
  core::assert! (core::mem::offset_of! (transfer, next) == 88);
  core::assert! (core::mem::offset_of! (transfer, last) == 96);
  core::assert! (core::mem::offset_of! (transfer, timeout) == 104);
  core::assert! (core::mem::offset_of! (transfer, serial) == 112);
  core::assert! (core::mem::offset_of! (transfer, flags) == 116);
};

/// C 546: `#define SELECTED_EVENTS 1`
#[cfg (feature = "have_x_windows")]
pub const SELECTED_EVENTS: c_int = 1;

/// C 548-567: `struct x_selection_request`.  Measured: `sizeof` **40**, `last`@0, `dpyinfo`@8,
/// `request`@16, `converted_selections`@24, `serial`@32, `converted`@36.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct x_selection_request
{
  pub last: *mut x_selection_request,         // C 551
  pub dpyinfo: *mut x_display_info,           // C 554
  pub request: *mut selection_input_event,    // C 557
  pub converted_selections: *mut selection_data, // C 560
  pub serial: c_uint,                         // C 563
  pub converted: bool,                        // C 566
}
#[cfg (feature = "have_x_windows")]
const _: () = {
  core::assert! (core::mem::size_of::<x_selection_request> () == 40);
  core::assert! (core::mem::offset_of! (x_selection_request, last) == 0);
  core::assert! (core::mem::offset_of! (x_selection_request, dpyinfo) == 8);
  core::assert! (core::mem::offset_of! (x_selection_request, request) == 16);
  core::assert! (core::mem::offset_of! (x_selection_request, converted_selections) == 24);
  core::assert! (core::mem::offset_of! (x_selection_request, serial) == 32);
  core::assert! (core::mem::offset_of! (x_selection_request, converted) == 36);
};

/* C 569-570: the "Stack of selections currently being processed" comment.  */
#[cfg (feature = "have_x_windows")]
static mut selection_request_stack: *mut x_selection_request = std::ptr::null_mut ();   // C 572

/* C 574-575: the "List of all outstanding selection transfers" comment.  */
#[cfg (feature = "have_x_windows")]
static mut outstanding_transfers: transfer = unsafe { std::mem::zeroed () };            // C 577

/* C 579: "A counter for selection serials."  */
#[cfg (feature = "have_x_windows")]
static mut selection_serial: c_uint = 0;                                                // C 581

/// C 585-594: `struct prop_location`.  Measured: `sizeof` **48**, `identifier`@0, `display`@8,
/// `window`@16, `property`@24, `desired_state`@32, `arrived`@36, `next`@40.
#[cfg (feature = "have_x_windows")]
#[repr(C)]
pub struct prop_location
{
  pub identifier: c_int,                // C 587
  pub display: *mut Display,            // C 588
  pub window: Window,                   // C 589
  pub property: Atom,                   // C 590
  pub desired_state: c_int,             // C 591
  pub arrived: bool,                    // C 592
  pub next: *mut prop_location,         // C 593
}
#[cfg (feature = "have_x_windows")]
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

#[cfg (feature = "have_x_windows")]
static mut prop_location_identifier: c_int = 0;                        // C 596

#[cfg (feature = "have_x_windows")]
static mut property_change_reply: Lisp_Object = lisp_h_Qnil ();        // C 598

#[cfg (feature = "have_x_windows")]
static mut property_change_reply_object: *mut prop_location = std::ptr::null_mut ();  // C 600

#[cfg (feature = "have_x_windows")]
static mut property_change_wait_list: *mut prop_location = std::ptr::null_mut ();     // C 602

#[cfg (feature = "have_x_windows")]
unsafe fn set_property_change_object (location: *mut prop_location)    // C 604-614
{
  /* Input must be blocked so we don't get the event before we set
     these.  */
  if !input_blocked_p ()                                       // C 609
    { emacs_abort (); }

  XSETCAR (property_change_reply, Qnil ());                     // C 612
  property_change_reply_object = location;
}

#[cfg (feature = "have_x_windows")]
unsafe fn x_push_current_selection_request (se: *mut selection_input_event,
                                            dpyinfo: *mut x_display_info)          // C 618-632
{
  let frame: *mut x_selection_request;

  frame = xmalloc (std::mem::size_of::<x_selection_request> ()) as *mut x_selection_request;
  (*frame).converted = false;                                   // C 625
  (*frame).last = selection_request_stack;
  (*frame).request = se;
  (*frame).dpyinfo = dpyinfo;
  (*frame).converted_selections = std::ptr::null_mut ();

  selection_request_stack = frame;                              // C 631
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_pop_current_selection_request ()     // C 634-643
{
  let tem: *mut x_selection_request;

  tem = selection_request_stack;                                // C 639
  selection_request_stack = (*selection_request_stack).last;

  xfree (tem as *mut c_void);
}

/* C 645-647: the "unwind-protect clause" comment.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_selection_request_lisp_error ()      // C 649-668
{
  let mut cs: *mut selection_data;
  let mut next: *mut selection_data = std::ptr::null_mut ();
  let frame: *mut x_selection_request;

  frame = selection_request_stack;                              // C 655

  cs = (*frame).converted_selections;                           // C 657
  while !cs.is_null ()
    {
      next = (*cs).next;
      if !(*cs).data.is_null ()
	{ xfree ((*cs).data as *mut c_void); }
      xfree (cs as *mut c_void);
      cs = next;
    }
  (*frame).converted_selections = std::ptr::null_mut ();        // C 664

  if !(*frame).converted && !(*(*frame).dpyinfo).display.is_null ()   // C 666
    { x_decline_selection_request ((*frame).request); }
}

/* C's `ckd_mul`/`ckd_add` (C 765/772) reach this file through lisp.h's `<stdckdint.h>`; the
   port's mirror carries the frozen arm's `overflowing_*` macros (the `src/xsmfns.rs` 440 edge).  */
#[cfg (feature = "have_x_windows")]
include!("../lib/include/stdckdint.in.rs");    // lisp.h's `<stdckdint.h>`

#[cfg (feature = "have_x_windows")]
unsafe fn c_size_for_format (format: c_int) -> ptrdiff_t           // C 672-688
{
  match format
    {
      8 => { return std::mem::size_of::<c_char> () as ptrdiff_t; }      // C 678
      16 => { return std::mem::size_of::<c_short> () as ptrdiff_t; }    // C 681
      32 => { return std::mem::size_of::<c_long> () as ptrdiff_t; }     // C 684
      _ => { }
    }

  emacs_abort ();                                               // C 687
}

#[cfg (feature = "have_x_windows")]
unsafe fn x_size_for_format (format: c_int) -> ptrdiff_t           // C 690-706
{
  match format
    {
      8 => { return 1; }
      16 => { return 2; }
      32 => { return 4; }
      _ => { }
    }

  emacs_abort ();                                               // C 705
}

/* C 708-711: selection_data_for_offset's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn selection_data_for_offset (data: *mut selection_data, offset: ptrdiff_t,
                                     remaining: *mut ptrdiff_t) -> *mut c_uchar   // C 713-740
{
  let mut base: *mut c_uchar;
  let mut size: ptrdiff_t;

  if !NILP ((*data).string)                                     // C 720
    {
      base = SDATA ((*data).string);
      size = SBYTES ((*data).string);
    }
  else
    {
      base = (*data).data;
      size = (*data).size;
    }

  if offset >= size                                             // C 731
    {
      *remaining = 0;
      return std::ptr::null_mut ();
    }

  base = base.offset (offset * c_size_for_format ((*data).format));     // C 737
  *remaining = size - offset;
  return base;
}

/* C 742-747: selection_data_size's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn selection_data_size (data: *mut selection_data) -> ptrdiff_t    // C 749-781
{
  let mut scratch: ptrdiff_t = 0;                                       // C 752
  let max_selection_size: ptrdiff_t = min! (min! (PTRDIFF_MAX as usize, SIZE_MAX) as ptrdiff_t,
                                            X_ULONG_MAX as ptrdiff_t);

  if !NILP ((*data).string)                                     // C 756
    { return SBYTES ((*data).string); }

  match (*data).format                                          // C 759
    {
      8 => { return min! ((*data).size, max_selection_size); }         // C 762
      16 =>                                                           // C 764
        {
          /* C 765-766: `ckd_mul (&scratch, data->size, 2) || ...` - the frozen arm's
             `__builtin_mul_overflow` always stores the wrapped product, so the port writes the
             `overflowing_mul` pair and the store at the site (P-15).  */
          let (scratch_v, overflow) = (*data).size.overflowing_mul (2);
          scratch = scratch_v;
          if overflow
              || max_selection_size - max_selection_size % 2 < scratch
            { return max_selection_size - max_selection_size % 2; }

          return scratch;
        }
      32 =>                                                           // C 771
        {
          let (scratch_v, overflow) = (*data).size.overflowing_mul (4);   // C 772-773
          scratch = scratch_v;
          if overflow
              || max_selection_size - max_selection_size % 4 < scratch
            { return max_selection_size - max_selection_size % 4; }

          return scratch;
        }
      _ => { }
    }

  /* The specified format is invalid.  */
  emacs_abort ();                                               // C 780
}

/* C 783-784: transfer_selecting_event's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn transfer_selecting_event (dpyinfo: *mut x_display_info, requestor: Window) -> bool
                                                                        // C 786-801
{
  let mut next: *mut transfer;

  next = outstanding_transfers.next;                            // C 792
  while std::ptr::eq (next, &raw const outstanding_transfers) == false
    {
      if (*next).requestor == requestor                         // C 795
	  && (*next).dpyinfo == dpyinfo
	{ return true; }
      next = (*next).next;
    }

  return false;                                                 // C 800
}

/* C 803-804: x_cancel_selection_transfer's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_cancel_selection_transfer (transfer: *mut transfer)   // C 806-835
{
  use crate::x11_constants::NoEventMask;

  xfree ((*transfer).data.data as *mut c_void);                  // C 809

  if !(*transfer).next.is_null ()                               // C 811
    {
      (*(*transfer).next).last = (*transfer).last;
      (*(*transfer).last).next = (*transfer).next;
    }

  if ((*transfer).flags & SELECTED_EVENTS) != 0                 // C 817
      && !transfer_selecting_event ((*transfer).dpyinfo, (*transfer).requestor)
      /* This can be called from x_delete_display.  */
      && !(*(*transfer).dpyinfo).display.is_null ()
    {
      /* Ignore errors generated by the change window request in case
	 the window has gone away.  */
      block_input ();                                           // C 825
      x_ignore_errors_for_next_request ((*transfer).dpyinfo, 0);
      XSelectInput ((*(*transfer).dpyinfo).display, (*transfer).requestor, NoEventMask as c_long);
      x_stop_ignoring_errors ((*transfer).dpyinfo);
      unblock_input ();
    }

  cancel_atimer ((*transfer).timeout);                          // C 833
  xfree (transfer as *mut c_void);
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_selection_transfer_timeout (atimer: *mut atimer)   // C 837-843
{
  let transfer: *mut transfer;

  transfer = (*atimer).client_data as *mut transfer;            // C 842
  x_cancel_selection_transfer (transfer);
}

/* C 846-849: x_start_selection_transfer's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_start_selection_transfer (dpyinfo: *mut x_display_info, requestor: Window,
                                      data: *mut selection_data)      // C 851-975
{
  use crate::x11_constants::PropModeReplace;

  let transfer: *mut transfer;
  let mut timeout: intmax_t;
  let secs: intmax_t;
  let nsecs: c_int;
  let mut remaining: ptrdiff_t = 0;   // C 859 (uninitialized in C; PORTING.md 1.5)
  let max_size: ptrdiff_t;
  let xdata: *mut c_uchar;
  let mut data_size: c_ulong;

  timeout = max! (0 as EMACS_INT, x_selection_timeout) as intmax_t;          // C 863
  secs = timeout / 1000;
  nsecs = ((timeout % 1000) * 1000000) as c_int;

  transfer = xzalloc (std::mem::size_of::<transfer> ()) as *mut transfer;   // C 867
  (*transfer).requestor = requestor;
  (*transfer).dpyinfo = dpyinfo;

  (*transfer).timeout = start_atimer (ATIMER_RELATIVE,         // C 871
				      make_timespec (secs, nsecs as i64),
				      Some (x_selection_transfer_timeout),
				      transfer as *mut c_void);

  /* Note that DATA is copied into transfer.  DATA->data is then set
     to NULL, giving the struct transfer ownership over the selection
     data.  */

  (*transfer).data = *data;   // C 880
  (*data).data = std::ptr::null_mut ();

  /* Finally, transfer now holds a reference to data->string, if it is
     present.  GC cannot be allowed to happen until this function
     returns.  */
  (*data).string = Qnil ();   // C 886

  /* Now, try to write the selection data.  ... */

  max_size = selection_quantum ((*dpyinfo).display) as ptrdiff_t;   // C 893

  let seldata_size: ptrdiff_t = selection_data_size (&raw mut (*transfer).data);   // C 895
  TRACE3! (" x_start_selection_transfer: transferring to 0x%lx.  \
transfer consists of %tu bytes, quantum being %tu",
	   requestor, seldata_size, max_size);   // C 896-898

  if max_size < seldata_size   // C 900
    {
      /* Begin incremental selection transfer.  First, calculate how
	 many elements it is ok to write for every ChangeProperty
	 request.  */
      (*transfer).items_per_request
	= max_size / x_size_for_format ((*transfer).data.format);   // C 905
      TRACE1! (" x_start_selection_transfer: starting incremental\
 selection transfer, with %tu items per request",
	       (*transfer).items_per_request);                  // C 907-909

      /* Next, link the transfer onto the list of pending selection
	 transfers.  */
      (*transfer).next = outstanding_transfers.next;            // C 913
      (*transfer).last = &raw mut outstanding_transfers;
      (*(*transfer).next).last = transfer;
      (*(*transfer).last).next = transfer;

      /* Find a valid (non-zero) serial for the selection transfer.  */
      selection_serial = selection_serial.wrapping_add (1);     // C 920 (C's `++selection_serial`)
      (*transfer).serial = if selection_serial != 0
	{ selection_serial }
	else
	{
	  selection_serial = selection_serial.wrapping_add (1);
	  selection_serial
	};   // C 921-923

      /* Now, write the INCR property to begin incremental selection
	 transfer.  offset is currently 0.  */

      data_size = seldata_size as c_ulong;   // C 928

      /* Set SELECTED_EVENTS before the actual XSelectInput
	 request.  */
      (*transfer).flags |= SELECTED_EVENTS;   // C 932

      x_ignore_errors_for_next_request (dpyinfo, (*transfer).serial);   // C 934
      XChangeProperty ((*dpyinfo).display, requestor,
		       (*transfer).data.property,
		       (*dpyinfo).Xatom_INCR, 32, PropModeReplace,
		       &raw mut data_size as *mut c_uchar, 1);

      /* This assumes that Emacs is not selecting for any other events
	 from the requestor!  */
      XSelectInput ((*dpyinfo).display, requestor, PropertyChangeMask as c_long);   // C 946
      x_stop_ignoring_errors (dpyinfo);
    }
  else
    {
      /* Write the property data now.  */
      xdata = selection_data_for_offset (&raw mut (*transfer).data,   // C 952
					 0, &raw mut remaining);
      eassert! (remaining <= INT_MAX as ptrdiff_t);

      TRACE1! (" x_start_selection_transfer:  writing\
 %tu elements directly to requestor window",
	       remaining);   // C 956-958

      x_ignore_errors_for_next_request (dpyinfo, 0);   // C 960
      XChangeProperty ((*dpyinfo).display, requestor,
		       (*transfer).data.property,
		       (*transfer).data.r#type,
		       (*transfer).data.format,
		       PropModeReplace, xdata, remaining as c_int);
      x_stop_ignoring_errors (dpyinfo);

      /* Next, get rid of the transfer.  */
      x_cancel_selection_transfer (transfer);   // C 969
    }
}

/* C 967-969: x_continue_selection_transfer's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_continue_selection_transfer (transfer: *mut transfer)      // C 977-1026
{
  use crate::x11_constants::PropModeReplace;

  let mut remaining: ptrdiff_t = 0;   // C 979 (uninitialized in C; PORTING.md 1.5)
  let xdata: *mut c_uchar;

  xdata = selection_data_for_offset (&raw mut (*transfer).data,   // C 983
				     (*transfer).offset,
				     &raw mut remaining);
  remaining = min! (remaining, (*transfer).items_per_request);

  if remaining == 0
    {
      /* The transfer is finished.  Write zero-length property data to
	 signal EOF and remove the transfer.  */
      TRACE0! (" x_continue_selection_transfer: writing 0 items to\
 indicate EOF");   // C 992-993
      x_ignore_errors_for_next_request ((*transfer).dpyinfo, 0);   // C 994
      XChangeProperty ((*(*transfer).dpyinfo).display,
		       (*transfer).requestor,
		       (*transfer).data.property,
		       (*transfer).data.r#type,
		       (*transfer).data.format,
		       PropModeReplace,
		       std::ptr::null_mut (), 0);
      x_stop_ignoring_errors ((*transfer).dpyinfo);
      TRACE0! (" x_continue_selection_transfer: done sending incrementally");   // C 1003

      x_cancel_selection_transfer (transfer);   // C 1005
    }
  else
    {
      TRACE2! (" x_continue_selection_transfer: writing %tu items\
; current offset is %tu", remaining, (*transfer).offset);   // C 1009-1010
      eassert! (remaining <= INT_MAX as ptrdiff_t);   // C 1011

      (*transfer).offset += remaining;   // C 1013

      x_ignore_errors_for_next_request ((*transfer).dpyinfo,
					(*transfer).serial);   // C 1015
      XChangeProperty ((*(*transfer).dpyinfo).display,
		       (*transfer).requestor,
		       (*transfer).data.property,
		       (*transfer).data.r#type,
		       (*transfer).data.format,
		       PropModeReplace, xdata,
		       remaining as c_int);
      x_stop_ignoring_errors ((*transfer).dpyinfo);
    }
}

/* The four callbacks C hands to `record_unwind_protect_*` and `atimer` keep C's linkage class
   (`static`, i.e. one definition), so their Rust spellings are `unsafe extern "C" fn` (PORTING.md
   1.4/P-12).  `record_unwind_protect_void` (lisp.h 4825) is not in the mirrors this unit includes
   (the `src/window.rs` 235 declaration is the shape); declared here at its first use site.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
}

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_remove_selection_transfers (dpyinfo: *mut x_display_info)  // C 1029-1047
{
  let mut next: *mut transfer;
  let mut last: *mut transfer;

  next = outstanding_transfers.next;   // C 1033
  while !std::ptr::eq (next, &raw const outstanding_transfers)
    {
      last = next;
      next = (*next).next;

      if (*last).dpyinfo == dpyinfo   // C 1039
	{ x_cancel_selection_transfer (last); }
    }
}

/* C 1049-1051: x_handle_selection_error's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_handle_selection_error (serial: c_uint,
                                                   error: *mut XErrorEvent)   // C 1049-1078
{
  let mut next: *mut transfer;
  let mut last: *mut transfer;

  if (*error).error_code as c_int != BadWindow   // C 1053
    /* The error was not caused by the window going away. ... */
    { return; }

  next = outstanding_transfers.next;                           // C 1060
  while !std::ptr::eq (next, &raw const outstanding_transfers)
    {
      last = next;
      next = (*next).next;

      if (*last).serial == serial   // C 1066
	{
	  /* Clear SELECTED_EVENTS, so x_cancel_selection_transfer
	     will not make X requests.  ...  */
	  (*last).flags &= !SELECTED_EVENTS;   // C 1072
	  x_cancel_selection_transfer (last);
	}
    }
}

/* C 1080: x_reply_selection_request's comment ("Send the reply to a selection request event
   EVENT.").  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_reply_selection_request (event: *mut selection_input_event,
                                     dpyinfo: *mut x_display_info)     // C 1080-1120
{
  use crate::x11_constants::{NoEventMask, None};

  let mut message: XEvent = std::mem::zeroed ();   // C 1084
  let mut cs: *mut selection_data;
  let frame: *mut x_selection_request;

  block_input ();   // C 1088
  frame = selection_request_stack;

  (*(&raw mut message as *mut XSelectionEvent)).r#type = SelectionNotify;  // C 1091
  (*(&raw mut message as *mut XSelectionEvent)).display = (*dpyinfo).display;
  (*(&raw mut message as *mut XSelectionEvent)).requestor = SELECTION_EVENT_REQUESTOR! (event);
  (*(&raw mut message as *mut XSelectionEvent)).selection = SELECTION_EVENT_SELECTION! (event);
  (*(&raw mut message as *mut XSelectionEvent)).time = SELECTION_EVENT_TIME! (event);
  (*(&raw mut message as *mut XSelectionEvent)).target = SELECTION_EVENT_TARGET! (event);
  (*(&raw mut message as *mut XSelectionEvent)).property = SELECTION_EVENT_PROPERTY! (event);

  if (*(&raw mut message as *mut XSelectionEvent)).property == None as Atom   // C 1099
    { (*(&raw mut message as *mut XSelectionEvent)).property =
	(*(&raw mut message as *mut XSelectionEvent)).target; }

  /* For each of the converted selections, start a write transfer from
     Emacs to the requestor.  */
  cs = (*frame).converted_selections;   // C 1104
  while !cs.is_null ()
    {
      x_start_selection_transfer (dpyinfo,
				  SELECTION_EVENT_REQUESTOR! (event),
				  cs);
      cs = (*cs).next;
    }

  /* Send the SelectionNotify event to the requestor, telling it that
     the property data has arrived.  */
  x_ignore_errors_for_next_request (dpyinfo, 0);   // C 1112
  XSendEvent ((*dpyinfo).display, SELECTION_EVENT_REQUESTOR! (event),
	      false as Bool, NoEventMask as c_long, &raw mut message);
  x_stop_ignoring_errors (dpyinfo);
  unblock_input ();
}

/* C 1122-1123: x_handle_selection_request and its comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_handle_selection_request (event: *mut selection_input_event)   // C 1122-1279
{
  use crate::x11_constants::{None, PropModeReplace, XA_ATOM};

  let mut local_selection_time: Time = 0;                       // C 1125 (uninitialized in C; PORTING.md 1.5)
  let dpyinfo: *mut x_display_info = SELECTION_EVENT_DPYINFO! (event);
  let selection: Atom = SELECTION_EVENT_SELECTION! (event);
  let mut selection_symbol: Lisp_Object = x_atom_to_symbol (dpyinfo, selection);
  let target: Atom = SELECTION_EVENT_TARGET! (event);
  let target_symbol: Lisp_Object = x_atom_to_symbol (dpyinfo, target);
  let mut property: Atom = SELECTION_EVENT_PROPERTY! (event);
  let local_selection_data: Lisp_Object;
  let mut success: bool = false;                                // C 1133
  let count: specpdl_ref = SPECPDL_INDEX ();                    // C 1134
  let mut pushed: bool;                                         // C 1135
  let mut use_alternate: bool;                                  // C 1135
  let mut alias: Lisp_Object;                                   // C 1136
  let mut tem: Lisp_Object;                                     // C 1136

  alias = Vx_selection_alias_alist;                             // C 1138

  FOR_EACH_TAIL_SAFE! (alias,                                   // C 1140-1154
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

  pushed = false;                                               // C 1156

  'really_done: loop
    {
      if dpyinfo.is_null ()                                     // C 1158
	{ break 'really_done; }                                 // C's `goto REALLY_DONE`

      local_selection_data = LOCAL_SELECTION! (selection_symbol, dpyinfo);   // C 1161

      'done: loop
	{
	  /* Decline if we don't own any selections.  */
	  if NILP (local_selection_data)                        // C 1164
	    { break 'done; }                                    // C's `goto DONE`

	  /* Decline requests issued prior to our acquiring the selection.  */
	  CONS_TO_INTEGER! (XCAR (XCDR (XCDR (local_selection_data))),
			    local_selection_time);              // C 1167-1168
	  if SELECTION_EVENT_TIME! (event) != CurrentTime        // C 1169
	      && local_selection_time > SELECTION_EVENT_TIME! (event)
	    { break 'done; }

	  use_alternate = false;                                // C 1173

	  /* This is how the XDND protocol recommends dropping text onto a
	     target that doesn't support XDND.  */
	  if (*dpyinfo).pending_dnd_time != 0                   // C 1177
	      && ((SELECTION_EVENT_TIME! (event) == (*dpyinfo).pending_dnd_time + 1)
		  || (SELECTION_EVENT_TIME! (event) == (*dpyinfo).pending_dnd_time + 2))
	    { use_alternate = true; }

	  block_input ();                                       // C 1184
	  pushed = true;
	  x_push_current_selection_request (event, dpyinfo);
	  record_unwind_protect_void (Some (x_pop_current_selection_request));
	  record_unwind_protect_void (Some (x_selection_request_lisp_error));
	  unblock_input ();

	  TRACE2! ("x_handle_selection_request: selection=%s, target=%s",
		   SDATA (SYMBOL_NAME (selection_symbol)),
		   SDATA (SYMBOL_NAME (target_symbol)));        // C 1191-1193

	  if EQ (target_symbol, QMULTIPLE ())                   // C 1195
	    {
	      /* For MULTIPLE targets, the event property names a list of atom
		 pairs; the first atom names a target and the second names a
		 non-None property.  */
	      let requestor: Window = SELECTION_EVENT_REQUESTOR! (event);
	      let multprop: Lisp_Object;
	      let mut j: ptrdiff_t;
	      let nselections: ptrdiff_t;
	      let mut cs: selection_data = std::mem::zeroed ();

	      if property == None as Atom                                // C 1205
		{ break 'done; }

	      multprop
		= x_get_window_property_as_lisp_data (dpyinfo, requestor, property,
						      QMULTIPLE (), selection, true);

	      if !VECTORP (multprop) || ASIZE (multprop) % 2 != 0     // C 1212
		{ break 'done; }

	      nselections = ASIZE (multprop) / 2;               // C 1215
	      /* Perform conversions.  This can signal.  */
	      j = 0;                                            // C 1217
	      while j < nselections
		{
		  let subtarget: Lisp_Object = AREF (multprop, (2 * j) as isize);
		  let subproperty: Atom = symbol_to_x_atom (dpyinfo,
							    AREF (multprop, (2 * j + 1) as isize));
		  let mut subsuccess: bool = false;             // C 1222

		  if subproperty != None as Atom                        // C 1224
		    { subsuccess = x_convert_selection (selection_symbol, subtarget,
						       subproperty, dpyinfo,
						       use_alternate); }
		  if !subsuccess                                // C 1228
		    { ASET (multprop, (2 * j + 1) as isize, Qnil ()); }
		  j += 1;
		}

	      /* Save conversion results */
	      lisp_data_to_selection_data (dpyinfo, multprop, &raw mut cs);   // C 1233

	      /* If cs.type is ATOM, change it to ATOM_PAIR.  ...  */
	      if cs.r#type == XA_ATOM                           // C 1238
		{ cs.r#type = (*dpyinfo).Xatom_ATOM_PAIR; }

	      XChangeProperty ((*dpyinfo).display, requestor, property,
			       cs.r#type, cs.format, PropModeReplace,
			       cs.data, cs.size as c_int);      // C 1241-1243
	      success = true;                                   // C 1244

	      xfree (cs.data as *mut c_void);                   // C 1246
	    }
	  else
	    {
	      if property == None as Atom                               // C 1250
		{ property = SELECTION_EVENT_TARGET! (event); }

	      success = x_convert_selection (selection_symbol,
					     target_symbol, property,
					     dpyinfo, use_alternate);   // C 1253-1255
	    }

	  break 'done;                                          // falls into the `DONE:` section
	}

      /* DONE: */                                               // C 1258

      if success                                                // C 1260
	{ x_reply_selection_request (event, dpyinfo); }
      else
	{ x_decline_selection_request (event); }

      if pushed                                                 // C 1265
	{ (*selection_request_stack).converted = true; }

      /* Run the `x-sent-selection-functions' abnormal hook.  */
      if !NILP (Vx_sent_selection_functions)                    // C 1269
	  && !BASE_EQ (Vx_sent_selection_functions, Qunbound ())
	{ CALLN! (Frun_hook_with_args, Qx_sent_selection_functions (),
		  selection_symbol, target_symbol, if success { Qt () } else { Qnil () }); }

      break 'really_done;                                       // C's `REALLY_DONE:` falls out here
    }

  unbind_to (count, Qnil ());                                   // C 1277
  return;
}

/* C 1281-1285: x_convert_selection's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_convert_selection (selection_symbol: Lisp_Object, target_symbol: Lisp_Object,
                               property: Atom, dpyinfo: *mut x_display_info,
                               use_alternate: bool) -> bool        // C 1287-1321
{
  let lisp_selection: Lisp_Object;
  let cs: *mut selection_data;
  let frame: *mut x_selection_request;

  lisp_selection
    = x_get_local_selection (selection_symbol, target_symbol,
			     false, dpyinfo, Qnil (), use_alternate);   // C 1296-1298

  frame = selection_request_stack;                              // C 1300

  /* A nil return value means we can't perform the conversion.  */
  if NILP (lisp_selection)                                      // C 1303
      || (CONSP (lisp_selection) && NILP (XCDR (lisp_selection)))
    { return false; }

  /* Otherwise, record the converted selection to binary.  */
  cs = xmalloc (std::mem::size_of::<selection_data> ()) as *mut selection_data;   // C 1307
  (*cs).data = std::ptr::null_mut ();
  (*cs).string = Qnil ();
  (*cs).property = property;
  (*cs).next = (*frame).converted_selections;
  (*frame).converted_selections = cs;

  lisp_data_to_selection_data (dpyinfo, lisp_selection, cs);    // C 1317
  return true;
}

/* The `lisp.h`/`dispextern.h` entry points this file calls that no included mirror carries
   (declared at their first use site with the reference header's line; `## Owed` 6-9).  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: dispextern.h 3589: `void redisplay_preserve_echo_area (int);`
  pub fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: lisp.h 5181: `extern int wait_reading_process_output (intmax_t, int, int, bool,
  /// Lisp_Object, struct Lisp_Process *, int);`
  pub fn wait_reading_process_output (time_limit: intmax_t, nsecs: c_int, read_kbd: c_int,
                                      do_display: bool, wait_for_cell: Lisp_Object,
                                      wait_proc: *mut c_void, just_wait_proc: c_int) -> c_int;
}

#[cfg (feature = "have_x_windows")]
unsafe fn x_handle_selection_clear (event: *mut selection_input_event)   // C 1323-1379
{
  let selection: Atom = SELECTION_EVENT_SELECTION! (event);      // C 1326
  let changed_owner_time: Time = SELECTION_EVENT_TIME! (event);
  let mut selection_symbol: Lisp_Object;                        // C 1329
  let mut local_selection_data: Lisp_Object;
  let mut local_selection_time: Time = 0;                       // C 1330
  let dpyinfo: *mut x_display_info = SELECTION_EVENT_DPYINFO! (event);
  let mut Vselection_alist: Lisp_Object;                        // C 1332

  TRACE0! ("x_handle_selection_clear");                         // C 1334

  if dpyinfo.is_null () { return; }                             // C 1336

  selection_symbol = x_atom_to_symbol (dpyinfo, selection);      // C 1338
  local_selection_data = LOCAL_SELECTION! (selection_symbol, dpyinfo);

  /* Well, we already believe that we don't own it, so that's just fine.  */
  if NILP (local_selection_data) { return; }                     // C 1342

  CONS_TO_INTEGER! (XCAR (XCDR (XCDR (local_selection_data))),
		    local_selection_time);                      // C 1344-1345

  /* We have reasserted the selection since this SelectionClear was
     generated, so we can disregard it.  */
  if changed_owner_time != CurrentTime                          // C 1349
      && local_selection_time > changed_owner_time
    { return; }

  /* Otherwise, really clear.  Don't use Fdelq as that may quit.  */
  Vselection_alist = (*(*dpyinfo).terminal).Vselection_alist;   // C 1354
  if EQ (local_selection_data, CAR (Vselection_alist))
    { Vselection_alist = XCDR (Vselection_alist); }
  else
    {
      let mut rest: Lisp_Object;
      rest = Vselection_alist;                                  // C 1360
      while CONSP (rest)
	{
	  if EQ (local_selection_data, CAR (XCDR (rest)))       // C 1361
	    {
	      XSETCDR (rest, XCDR (XCDR (rest)));
	      break;
	    }
	  rest = XCDR (rest);
	}
    }
  tset_selection_alist ((*dpyinfo).terminal, Vselection_alist);  // C 1367

  /* Run the `x-lost-selection-functions' abnormal hook.  */
  CALLN! (Frun_hook_with_args, Qx_lost_selection_functions (), selection_symbol);

  /* If Emacs lost ownership of XdndSelection during drag-and-drop,
     there is no point in continuing the drag-and-drop session.  */
  if x_dnd_in_progress                                       // C 1374
      && EQ (selection_symbol, QXdndSelection ())
    { error (c"Lost ownership of XdndSelection".as_ptr ()); }

  redisplay_preserve_echo_area (20);                            // C 1378
}

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_handle_selection_event (event: *mut selection_input_event)  // C 1381-1389
{
  TRACE0! ("x_handle_selection_event");                          // C 1384
  if selection_input_event_bits! (kind, event) != SELECTION_REQUEST_EVENT
    { x_handle_selection_clear (event); }
  else
    { x_handle_selection_request (event); }
}

#[cfg (feature = "have_x_windows")]
unsafe fn x_should_preserve_selection (selection: Lisp_Object) -> bool   // C 1391-1410
{
  let mut tem: Lisp_Object;

  tem = Vx_auto_preserve_selections;                            // C 1396

  if CONSP (Vx_auto_preserve_selections)                        // C 1398
    {
      FOR_EACH_TAIL_SAFE! (tem,                                  // C 1400-1404
	{
	  if EQ (XCAR (tem), selection)
	    { return true; }
	});

      return false;                                             // C 1406
    }

  return !NILP (tem);                                           // C 1409
}

/* C 1412-1413: x_clear_frame_selections's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_clear_frame_selections (f: *mut frame)      // C 1415-1460
{
  let mut frame: Lisp_Object;                                   // C 1418
  let mut rest: Lisp_Object;
  let mut lost: Lisp_Object;
  let mut selection: Lisp_Object;
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let t: *mut terminal = (*dpyinfo).terminal;

  XSETFRAME! (frame, f);                                        // C 1422
  lost = Qnil ();

  /* Delete elements from the beginning of Vselection_alist.  */
  while CONSP ((*t).Vselection_alist)                           // C 1426
	&& EQ (frame, XCAR (XCDR (XCDR (XCDR (XCAR ((*t).Vselection_alist))))))
    {
      selection = CAR (CAR ((*t).Vselection_alist));

      if !x_should_preserve_selection (selection)               // C 1431
	/* Run the `x-lost-selection-functions' abnormal hook.  */
	{ CALLN! (Frun_hook_with_args, Qx_lost_selection_functions (),
		 selection); }
      else
	{ lost = Fcons (CAR ((*t).Vselection_alist), lost); }

      tset_selection_alist (t, XCDR ((*t).Vselection_alist));   // C 1438
    }

  /* Delete elements after the beginning of Vselection_alist.  */
  rest = (*t).Vselection_alist;                                  // C 1442
  while CONSP (rest)
    {
      if CONSP (XCDR (rest))
	  && EQ (frame, XCAR (XCDR (XCDR (XCDR (XCAR (XCDR (rest)))))))
	{
	  selection = XCAR (XCAR (XCDR (rest)));                 // C 1446

	  if !x_should_preserve_selection (selection)
	    { CALLN! (Frun_hook_with_args, Qx_lost_selection_functions (),
		      selection); }
	  else
	    { lost = Fcons (XCAR (XCDR (rest)), lost); }

	  XSETCDR (rest, XCDR (XCDR (rest)));                    // C 1454
	  break;
	}
      rest = XCDR (rest);
    }

  if !NILP (lost)                                               // C 1458
    { x_preserve_selections (dpyinfo, lost, frame); }
}

/* C 1462-1463: waiting_for_other_props_on_window's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn waiting_for_other_props_on_window (display: *mut Display, window: Window) -> bool
                                                                        // C 1465-1474
{
  let mut p: *mut prop_location = property_change_wait_list;     // C 1468
  while !p.is_null ()
    {
      if (*p).display == display && (*p).window == window
	{ return true; }
      p = (*p).next;
    }

  return false;                                                 // C 1473
}

/* C 1476-1478: expect_property_change's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn expect_property_change (display: *mut Display, window: Window,
                                  property: Atom, state: c_int) -> *mut prop_location
                                                                        // C 1479-1496
{
  let pl: *mut prop_location = xmalloc (std::mem::size_of::<prop_location> ()) as *mut prop_location;
  prop_location_identifier += 1;   // C 1484
  (*pl).identifier = prop_location_identifier;
  (*pl).display = display;
  (*pl).window = window;
  (*pl).property = property;
  (*pl).desired_state = state;
  (*pl).next = property_change_wait_list;
  (*pl).arrived = false;
  property_change_wait_list = pl;
  return pl;
}

/* C 1498-1499: unexpect_property_change's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn unexpect_property_change (location: *mut prop_location)      // C 1498-1516
{
  let mut prop: *mut prop_location;
  let mut pprev: *mut *mut prop_location = &raw mut property_change_wait_list;   // C 1501

  prop = property_change_wait_list;
  while !prop.is_null ()
    {
      if prop == location   // C 1505
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

/* C 1518: wait_for_property_change_unwind's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn wait_for_property_change_unwind (loc: *mut c_void)   // C 1518-1529
{
  let location: *mut prop_location = loc as *mut prop_location;

  unexpect_property_change (location);   // C 1523
  if location == property_change_reply_object
    { property_change_reply_object = std::ptr::null_mut (); }
}

/* C 1531-1532: wait_for_property_change's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn wait_for_property_change (location: *mut prop_location)      // C 1531-1569
{
  let count: specpdl_ref = SPECPDL_INDEX ();                    // C 1534

  /* Make sure to do unexpect_property_change if we quit or err.  */
  record_unwind_protect_ptr (Some (wait_for_property_change_unwind), location as *mut c_void);

  /* See comment in x_reply_selection_request about setting
     property_change_reply.  Do not do it here.  */

  /* If the event we are waiting for arrives beyond here, it will set
     property_change_reply, because property_change_reply_object says so.  */
  if !(*location).arrived   // C 1544
    {
      let timeout: intmax_t = max! (0 as EMACS_INT, x_selection_timeout) as intmax_t;
      let secs: intmax_t = timeout / 1000;
      let nsecs: c_int = ((timeout % 1000) * 1000000) as c_int;
      TRACE2! (concat! ("  Waiting %", PRIdMAX! (), " secs, %d nsecs"), secs, nsecs);  // C 1549

      if !input_blocked_p ()                                    // C 1551
	{ wait_reading_process_output (secs, nsecs, 0, false,
				       property_change_reply, std::ptr::null_mut (), 0); }
      else
	{ x_wait_for_cell_change (property_change_reply,
				  make_timespec (secs, nsecs as i64)); }

      if NILP (XCAR (property_change_reply))   // C 1558
	{
	  TRACE0! ("  Timed out");
	  error (c"Timed out waiting for property-notify event".as_ptr ());
	}
    }

  unbind_to (count, Qnil ());                                   // C 1567
}

/* C 1571: x_handle_property_notify's comment ("Called from XTread_socket in response to a
   PropertyNotify event.").  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_handle_property_notify (event: *const XPropertyEvent)  // C 1571-1627
{
  let mut rest: *mut prop_location;                             // C 1573
  let mut next: *mut transfer;
  #[cfg (feature = "trace_selection")]
  let mut name: *mut c_char;                                    // C 1576

  rest = property_change_wait_list;                             // C 1578
  while !rest.is_null ()
    {
      if !(*rest).arrived                                       // C 1580
	  && (*rest).property == (*event).atom
	  && (*rest).window == (*event).window
	  && (*rest).display == (*event).display
	  && (*rest).desired_state == (*event).state
	{
	  #[cfg (feature = "trace_selection")]
	  {
	    name = XGetAtomName ((*event).display, (*event).atom);

	    TRACE2! ("Expected %s of property %s",
		     if (*event).state == PropertyDelete { "deletion" } else { "change" },
		     if name.is_null () { c"unknown".as_ptr () } else { name });

	    if !name.is_null ()
	      { XFree (name as *mut c_void); }
	  }

	  (*rest).arrived = true;                               // C 1598

	  /* If this is the one wait_for_property_change is waiting for,
	     tell it to wake up.  */
	  if rest == property_change_reply_object                // C 1602
	    { XSETCAR (property_change_reply, Qt ()); }

	  return;
	}
      rest = (*rest).next;
    }

  /* Look for a property change for an outstanding selection
     transfer.  */
  next = outstanding_transfers.next;                            // C 1613
  while !std::ptr::eq (next, &raw const outstanding_transfers)
    {
      if (*(*next).dpyinfo).display == (*event).display
	  && (*next).requestor == (*event).window
	  && (*next).data.property == (*event).atom
	  && (*event).state == PropertyDelete
	{
	  TRACE1! ("Expected PropertyDelete event arrived from the\
 requestor window %lx", (*next).requestor);                 // C 1620-1621

	  x_continue_selection_transfer (next);                 // C 1623
	  return;
	}

      next = (*next).next;
    }
}

/* The two `lisp.h` names C 1637/1716 and C 1719 use that no included mirror carries.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: lisp.h 4393: `extern void message3_nolog (Lisp_Object);`
  pub fn message3_nolog (m: Lisp_Object);
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_display_selection_waiting_message (timer: *mut atimer)   // C 1630-1639
{
  let val: Lisp_Object;
  let atimerp: *mut *mut atimer = (*timer).client_data as *mut *mut atimer;   // C 1634

  val = build_string (c"Waiting for reply from selection owner...".as_ptr ());   // C 1636
  message3_nolog (val);
  *atimerp = std::ptr::null_mut ();
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_cancel_atimer (atimerp: *mut c_void)            // C 1641-1650
{
  let atimer: *mut *mut atimer = atimerp as *mut *mut atimer;
  if !(*atimer).is_null ()
    {
      cancel_atimer (*atimer);
      *atimer = std::ptr::null_mut ();
    }
}

/* C 1653: C's `Variables for communication with x_handle_selection_notify.` banner.  */
#[cfg (feature = "have_x_windows")]
static mut reading_which_selection: Atom = 0;                  // C 1654
#[cfg (feature = "have_x_windows")]
static mut reading_selection_reply: Lisp_Object = lisp_h_Qnil ();   // C 1655
#[cfg (feature = "have_x_windows")]
static mut reading_selection_window: Window = 0;               // C 1656

/* C 1658-1660: x_get_foreign_selection's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_get_foreign_selection (selection_symbol: Lisp_Object, target_type: Lisp_Object,
                                   time_stamp: Lisp_Object, frame: Lisp_Object) -> Lisp_Object
                                                                        // C 1662-1762
{
  let f: *mut frame = XFRAME (frame);                          // C 1666
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let display: *mut Display = (*dpyinfo).display;
  let requestor_window: Window = FRAME_X_WINDOW! (f);
  let mut requestor_time: Time = (*dpyinfo).last_user_time;      // C 1670
  let target_property: Atom = (*dpyinfo).Xatom_EMACS_TMP;
  let selection_atom: Atom = symbol_to_x_atom (dpyinfo, selection_symbol);
  let type_atom: Atom = if CONSP (target_type)                  // C 1673-1675
    { symbol_to_x_atom (dpyinfo, XCAR (target_type)) }
    else
    { symbol_to_x_atom (dpyinfo, target_type) };
  let mut delayed_message: *mut atimer = std::ptr::null_mut ();  // C 1676
  let message_interval: timespec;                               // C 1677
  let count: specpdl_ref;                                       // C 1678

  count = SPECPDL_INDEX ();                                     // C 1680

  if !FRAME_LIVE_P (f)                                          // C 1682
    { return unbind_to (count, Qnil ()); }

  if !NILP (time_stamp)                                         // C 1685
    { CONS_TO_INTEGER! (time_stamp, requestor_time); }

  block_input ();                                               // C 1688
  TRACE2! ("Get selection %s, type %s",
	   XGetAtomName (display, type_atom),
	   XGetAtomName (display, target_property));            // C 1689-1691

  x_catch_errors (display);                                     // C 1693
  XConvertSelection (display, selection_atom, type_atom, target_property,
		     requestor_window, requestor_time);
  x_check_errors (display, c"Can't convert selection: %s".as_ptr ());
  x_uncatch_errors_after_check ();

  /* Prepare to block until the reply has been read.  */
  reading_selection_window = requestor_window;                  // C 1700
  reading_which_selection = selection_atom;
  XSETCAR (reading_selection_reply, Qnil ());

  /* It should not be necessary to stop handling selection requests
     during this time.  In fact, the SAVE_TARGETS mechanism requires
     us to handle a clipboard manager's requests before it returns
     SelectionNotify. */
  /* C 1708-1711 is `#if false`, dead in every configuration: `x_start_queuing_selection_requests
     (); record_unwind_protect_void (x_stop_queuing_selection_requests);` - recorded, not
     transcribed (the `src/xfns.rs` 958 precedent).  */

  unblock_input ();                                             // C 1713

  message_interval = make_timespec (1, 0);                      // C 1715
  delayed_message = start_atimer (ATIMER_RELATIVE, message_interval,
				  Some (x_display_selection_waiting_message),
				  &raw mut delayed_message as *mut c_void);
  record_unwind_protect_ptr (Some (x_cancel_atimer), &raw mut delayed_message as *mut c_void);

  /* This allows quits.  Also, don't wait forever.  */
  let timeout: intmax_t = max! (0 as EMACS_INT, x_selection_timeout) as intmax_t;   // C 1722
  let secs: intmax_t = timeout / 1000;
  let nsecs: c_int = ((timeout % 1000) * 1000000) as c_int;
  TRACE1! (concat! ("  Start waiting %", PRIdMAX! (), " secs for SelectionNotify."), secs);

  if input_blocked_p ()                                         // C 1727
    { TRACE0! ("    Input is blocked."); }
  else
    { TRACE1! ("    Waiting for %d nsecs in addition.", nsecs); }

  /* This function can be called with input blocked inside Xt or GTK
     timeouts run inside popup menus, so use a function that works
     when input is blocked.  Prefer wait_reading_process_output
     otherwise, or the toolkit might not get some events.
     (bug#22214) */
  if !input_blocked_p ()                                        // C 1737
    { wait_reading_process_output (secs, nsecs, 0, false,
				   reading_selection_reply, std::ptr::null_mut (), 0); }
  else
    { x_wait_for_cell_change (reading_selection_reply,
			      make_timespec (secs, nsecs as i64)); }
  TRACE1! ("  Got event = %s", if !NILP (XCAR (reading_selection_reply))
	     {
	       if SYMBOLP (XCAR (reading_selection_reply))
		 { SSDATA (SYMBOL_NAME (XCAR (reading_selection_reply))) }
	       else
		 { c"YES".as_ptr () as *mut c_char }
	     }
	     else
	     { c"NO".as_ptr () as *mut c_char });                // C 1743-1747

  if NILP (XCAR (reading_selection_reply))                      // C 1749
    { error (c"Timed out waiting for reply from selection owner".as_ptr ()); }
  if EQ (XCAR (reading_selection_reply), Qlambda ())            // C 1751
    { return unbind_to (count, Qnil ()); }

  /* Otherwise, the selection is waiting for us on the requested property.  */
  return unbind_to (count,
		    x_get_window_property_as_lisp_data (dpyinfo,
							requestor_window,
							target_property,
							target_type,
							selection_atom,
							false));   // C 1755-1761
}

/* C 1764: the "Subroutines of x_get_window_property_as_lisp_data" banner.  */
/* C 1766: the "Use xfree, not XFree" comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_get_window_property (display: *mut Display, window: Window, property: Atom,
                                 data_ret: *mut *mut c_uchar, bytes_ret: *mut ptrdiff_t,
                                 actual_type_ret: *mut Atom, actual_format_ret: *mut c_int,
                                 actual_size_ret: *mut c_ulong)          // C 1768-1900
{
  use crate::x11_constants::{AnyPropertyType, None, Success};

  let mut total_size: ptrdiff_t;                               // C 1774
  let mut bytes_remaining: c_ulong = 0;                        // C 1775 (uninitialized in C; PORTING.md 1.5)
  let mut offset: ptrdiff_t = 0;                               // C 1776
  let mut data: *mut c_uchar = std::ptr::null_mut ();           // C 1777
  let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();       // C 1778
  let mut result: c_int;                                       // C 1779
  let buffer_size: c_int = selection_quantum (display);         // C 1780

  /* Wide enough to avoid overflow in expressions using it.  */
  let x_long_size: ptrdiff_t = X_LONG_SIZE as ptrdiff_t;         // C 1783

  /* Maximum value for TOTAL_SIZE.  ...  */
  let total_size_max: ptrdiff_t                                // C 1788-1791
    = if (min! (PTRDIFF_MAX as usize, SIZE_MAX) - 1) / (x_long_size as usize) < LONG_MAX as usize
      { (min! (PTRDIFF_MAX as usize, SIZE_MAX) - 1) as ptrdiff_t }
      else
      { (LONG_MAX as ptrdiff_t) * x_long_size };

  'overflow: loop
    {
      'done: loop
	{
	  block_input ();                                      // C 1793

	  /* First probe the thing to find out how big it is.  */
	  result = XGetWindowProperty (display, window, property,
				       0, 0, false as Bool, AnyPropertyType as Atom,
				       actual_type_ret, actual_format_ret,
				       actual_size_ret,
				       &raw mut bytes_remaining, &raw mut tmp_data);   // C 1796-1800
	  if result != Success                                // C 1801
	    { break 'done; }                                  // C's `goto done`

	  /* This was allocated by Xlib, so use XFree.  */
	  XFree (tmp_data as *mut c_void);                     // C 1805

	  if *actual_type_ret == None as Atom || *actual_format_ret == 0   // C 1807
	    { break 'done; }

	  if total_size_max < bytes_remaining as ptrdiff_t     // C 1810
	    { break 'overflow; }                              // C's `goto size_overflow`
	  total_size = bytes_remaining as ptrdiff_t;            // C 1812
	  data = xmalloc ((total_size + 1) as usize) as *mut c_uchar;

	  /* Now read, until we've gotten it all.  */
	  while bytes_remaining != 0                           // C 1816
	    {
	      let mut bytes_gotten: ptrdiff_t;                  // C 1818
	      let bytes_per_item: c_int;                        // C 1819
	      result
		= XGetWindowProperty (display, window, property,
				      (offset / X_LONG_SIZE as ptrdiff_t) as c_long,
				      buffer_size as c_long / X_LONG_SIZE as c_long,
				      false as Bool,
				      AnyPropertyType as Atom,
				      actual_type_ret, actual_format_ret,
				      actual_size_ret, &raw mut bytes_remaining,
				      &raw mut tmp_data);       // C 1820-1827

	      /* If this doesn't return Success at this point, ...  */
	      if result != Success                              // C 1832
		{ break; }

	      bytes_per_item = *actual_format_ret >> 3;         // C 1835
	      eassert! (*actual_size_ret <= (buffer_size / bytes_per_item) as c_ulong);   // C 1836

	      /* The man page for XGetWindowProperty says: ...  */
	      bytes_gotten = *actual_size_ret as ptrdiff_t;     // C 1851
	      bytes_gotten *= bytes_per_item as ptrdiff_t;

	      TRACE2! (concat! ("Read %", pD! (), "d bytes from property %s"),
		       bytes_gotten, XGetAtomName (display, property));   // C 1854-1855

	      if total_size - offset < bytes_gotten             // C 1857
		{
		  let data1: *mut c_uchar;                      // C 1859
		  let remaining_lim: ptrdiff_t = total_size_max - offset - bytes_gotten;   // C 1860
		  if remaining_lim < 0 || remaining_lim < bytes_remaining as ptrdiff_t
		    { break 'overflow; }                        // C 1862
		  total_size = offset + bytes_gotten + bytes_remaining as ptrdiff_t;   // C 1863
		  data1 = xrealloc (data as *mut c_void, (total_size + 1) as usize) as *mut c_uchar;
		  data = data1;
		}

	      if LONG_WIDTH > 32 && *actual_format_ret == 32     // C 1868
		{
		  let mut i: c_ulong;                           // C 1870
		  let idata: *mut c_int = data.offset (offset) as *mut c_int;   // C 1871
		  let ldata: *const c_long = tmp_data as *const c_long;         // C 1872

		  i = 0;
		  while i < *actual_size_ret                     // C 1874
		    {
		      *idata.offset (i as isize) = *ldata.offset (i as isize) as c_int;
		      i += 1;
		    }
		}
	      else
		{ memcpy (data.offset (offset) as *mut c_void, tmp_data as *const c_void,
			  bytes_gotten as usize); }             // C 1878

	      offset += bytes_gotten;                           // C 1880

	      /* This was allocated by Xlib, so use XFree.  */
	      XFree (tmp_data as *mut c_void);                  // C 1883
	    }

	  XFlush (display);                                     // C 1886
	  *data.offset (offset) = 0;                            // C 1887

	  break 'done;
	}

      /* done: */
      unblock_input ();                                         // C 1890
      *data_ret = data;                                        // C 1891
      *bytes_ret = offset;                                     // C 1892
      return;                                                  // C 1893
    }

  /* size_overflow: */
  if !data.is_null ()                                          // C 1896
    { xfree (data as *mut c_void); }
  unblock_input ();                                            // C 1898
  memory_full_up ();                                           // C 1899
}

/* C 1902: the "Use xfree, not XFree" comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn receive_incremental_selection (dpyinfo: *mut x_display_info,
                                         window: Window, property: Atom,
                                         target_type: Lisp_Object,
                                         min_size_bytes: c_uint,
                                         data_ret: *mut *mut c_uchar,
                                         size_bytes_ret: *mut ptrdiff_t,
                                         type_ret: *mut Atom, format_ret: *mut c_int,
                                         size_ret: *mut c_ulong,
                                         real_bytes_ret: *mut ptrdiff_t)   // C 1904-2021
{
  use crate::x11_constants::PropertyChangeMask;   // STANDARD_EVENT_SET is xterm.h's (mirror)

  let mut offset: ptrdiff_t = 0;                                // C 1915
  let mut wait_object: *mut prop_location;                      // C 1916
  let display: *mut Display = (*dpyinfo).display;                // C 1917

  if min! (PTRDIFF_MAX as usize, SIZE_MAX) < min_size_bytes as usize   // C 1919
    { memory_full_up (); }
  *data_ret = xmalloc (min_size_bytes as usize) as *mut c_uchar; // C 1921
  *size_bytes_ret = min_size_bytes as ptrdiff_t;                 // C 1922

  TRACE1! ("Read %u bytes incrementally", min_size_bytes);       // C 1924

  /* At this point, we have read an INCR property.  ...  */
  block_input ();                                               // C 1938
  XSelectInput (display, window, STANDARD_EVENT_SET | PropertyChangeMask as c_long);   // C 1939
  TRACE1! ("  Delete property %s",
	   SDATA (SYMBOL_NAME (x_atom_to_symbol (dpyinfo, property))));   // C 1940-1941
  XDeleteProperty (display, window, property);                   // C 1942
  TRACE1! ("  Expect new value of property %s",
	   SDATA (SYMBOL_NAME (x_atom_to_symbol (dpyinfo, property))));   // C 1943-1944
  wait_object = expect_property_change (display, window, property,
					PropertyNewValue);       // C 1945-1946
  XFlush (display);                                              // C 1947
  /* See comment in x_reply_selection_request about property_change_reply.  */
  set_property_change_object (wait_object);                       // C 1949
  unblock_input ();

  while true                                                     // C 1952
    {
      let mut tmp_data: *mut c_uchar = std::ptr::null_mut ();
      let mut tmp_size_bytes: ptrdiff_t = 0;

      TRACE0! ("  Wait for property change");                     // C 1956
      wait_for_property_change (wait_object);                     // C 1957

      /* expect it again immediately, because x_get_window_property may
	 .. no it won't, I don't get it.
	 .. Ok, I get it now, the Xt code that implements INCR is broken. */
      TRACE0! ("  Get property value");                           // C 1962
      x_get_window_property (display, window, property,
			     &raw mut tmp_data, &raw mut tmp_size_bytes,
			     type_ret, format_ret, size_ret);     // C 1963-1965

      TRACE1! (concat! ("  Read increment of %", pD! (), "d bytes"), tmp_size_bytes);   // C 1967

      if tmp_size_bytes == 0 /* we're done */                    // C 1969
	{
	  TRACE1! (concat! ("Done reading incrementally; total bytes: %", pD! (), "d"),
		   *size_bytes_ret);                             // C 1971-1972

	  if !waiting_for_other_props_on_window (display, window)   // C 1974
	    { XSelectInput (display, window, STANDARD_EVENT_SET); }
	  /* Use xfree, not XFree, because x_get_window_property
	     calls xmalloc itself.  */
	  xfree (tmp_data as *mut c_void);                        // C 1978
	  break;
	}

      block_input ();                                           // C 1981
      TRACE1! ("  ACK by deleting property %s",
	       XGetAtomName (display, property));               // C 1982-1983
      XDeleteProperty (display, window, property);              // C 1984
      wait_object = expect_property_change (display, window, property,
					    PropertyNewValue);   // C 1985-1986
      /* See comment in x_reply_selection_request about
	 property_change_reply.  */
      set_property_change_object (wait_object);                 // C 1989
      XFlush (display);                                         // C 1990
      unblock_input ();

      if *size_bytes_ret - offset < tmp_size_bytes              // C 1993
	{ *data_ret = xpalloc (*data_ret as *mut c_void, size_bytes_ret,
			       tmp_size_bytes - (*size_bytes_ret - offset),
			       -1, 1) as *mut c_uchar; }

      memcpy ((*data_ret).offset (offset) as *mut c_void, tmp_data as *const c_void,
	      tmp_size_bytes as usize);                          // C 2002
      offset += tmp_size_bytes;                                 // C 2003

      /* *size_bytes_ret is not really the size of the data inside the
	 buffer; it is the size of the buffer allocated by xpalloc.  ...  */
      *real_bytes_ret += tmp_size_bytes;                        // C 2021

      /* Use xfree, not XFree, because x_get_window_property
	 calls xmalloc itself.  */
      xfree (tmp_data as *mut c_void);                          // C 2024
    }
}

/* C 2029-2030: x_free_selection_data's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_free_selection_data (data: *mut c_void)         // C 2023-2034
{
  let ptr: *mut *mut c_uchar;

  ptr = data as *mut *mut c_uchar;                              // C 2029
  xfree (*ptr as *mut c_void);                                  // C 2030
}

/* C 2027-2028 and C 2036-2038: the two comments above x_get_window_property_as_lisp_data.  */

#[cfg (feature = "have_x_windows")]
unsafe fn x_get_window_property_as_lisp_data (dpyinfo: *mut x_display_info,
                                              window: Window, property: Atom,
                                              target_type: Lisp_Object,
                                              selection_atom: Atom,
                                              for_multiple: bool) -> Lisp_Object
                                                                        // C 2036-2156
{
  let mut actual_type: Atom = 0;                                // C 2045
  let mut actual_format: c_int = 0;
  let mut actual_size: c_ulong = 0;
  let mut data: *mut c_uchar = std::ptr::null_mut ();            // C 2048
  let mut bytes: ptrdiff_t = 0;                                 // C 2049
  let mut array_bytes: ptrdiff_t;                               // C 2049
  let val: Lisp_Object;
  let display: *mut Display = (*dpyinfo).display;
  let count: specpdl_ref;

  /* array_bytes is only used as an argument to xpalloc.  ...  */
  array_bytes = 0;                                              // C 2057

  TRACE0! ("Reading selection data");                           // C 2059

  x_get_window_property (display, window, property, &raw mut data, &raw mut bytes,
			 &raw mut actual_type, &raw mut actual_format, &raw mut actual_size);
  if data.is_null ()                                            // C 2063
    {
      if for_multiple
	{ return Qnil (); }
      block_input ();
      let there_is_a_selection_owner
	= XGetSelectionOwner (display, selection_atom) != 0;    // C 2068
      unblock_input ();
      if there_is_a_selection_owner                             // C 2070
	{
	  AUTO_STRING! (format, c"Selection owner couldn't convert: %s".as_ptr ());
	  CALLN! (Fmessage, format,
		  if actual_type != 0
		    { list2 (target_type, x_atom_to_symbol (dpyinfo, actual_type)) }
		    else
		    { target_type });                           // C 2071-2077
	  return Qnil ();
	}
      else
	{
	  AUTO_STRING! (format, c"No selection: %s".as_ptr ());
	  CALLN! (Fmessage, format, x_atom_to_symbol (dpyinfo, selection_atom));
	  return Qnil ();                                     // C 2083-2086
	}
    }

  /* Make sure DATA is freed even if `receive_incremental_connection'
     quits. ...  */

  count = SPECPDL_INDEX ();                                     // C 2094
  record_unwind_protect_ptr (Some (x_free_selection_data), &raw mut data as *mut c_void);

  if !for_multiple && actual_type == (*dpyinfo).Xatom_INCR      // C 2096
    {
      /* That wasn't really the data, just the beginning.  */

      let min_size_bytes: c_uint = *(data as *mut c_uint);      // C 2100
      block_input ();
      /* Use xfree, not XFree, because x_get_window_property
	 calls xmalloc itself.  */
      xfree (data as *mut c_void);                              // C 2104

      /* In case quitting happens below.  */
      data = std::ptr::null_mut ();                             // C 2107
      unblock_input ();

      /* Clear bytes again.  ...  */
      bytes = 0;                                                // C 2113
      receive_incremental_selection (dpyinfo, window, property, target_type,
				     min_size_bytes, &raw mut data, &raw mut array_bytes,
				     &raw mut actual_type, &raw mut actual_format,
				     &raw mut actual_size, &raw mut bytes);   // C 2114-2117
    }

  if !for_multiple                                            // C 2120
    {
      block_input ();
      TRACE1! ("  Delete property %s", XGetAtomName (display, property));   // C 2122
      XDeleteProperty (display, window, property);
      XFlush (display);
      unblock_input ();
    }

  /* It's been read.  Now convert it to a lisp object in some semi-rational
     manner.  */
  val = selection_data_to_lisp_data (dpyinfo, data, bytes,
				     actual_type, actual_format);   // C 2132-2133

  /* This will also free `data'.  */
  return unbind_to (count, val);                                // C 2136
}

/* C 2138-2156: the "These functions convert from ..." table comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn selection_data_to_lisp_data (dpyinfo: *mut x_display_info,
                                       data: *const c_uchar,
                                       size: ptrdiff_t, r#type: Atom,
                                       format: c_int) -> Lisp_Object     // C 2158-2288
{
  use crate::x11_constants::XA_ATOM;

  if r#type == (*dpyinfo).Xatom_NULL                             // C 2163
    { return QNULL (); }
  /* Convert any 8-bit data to a string, for compactness.  */
  else if format == 8                                            // C 2167
    {
      let str_: Lisp_Object;
      let lispy_type: Lisp_Object;

      str_ = make_unibyte_string (data as *const c_char, size);   // C 2171
      /* Indicate that this string is from foreign selection by a text
	 property `foreign-selection' so that the caller of
	 x-get-selection-internal (usually x-get-selection) can know
	 that the string must be decode.  */
      if r#type == (*dpyinfo).Xatom_COMPOUND_TEXT   // C 2176
	{ lispy_type = QCOMPOUND_TEXT (); }
      else if r#type == (*dpyinfo).Xatom_UTF8_STRING
	{ lispy_type = QUTF8_STRING (); }
      else
	{ lispy_type = QSTRING (); }
      Fput_text_property (make_fixnum (0), make_fixnum (size as EMACS_INT),
			  Qforeign_selection (), lispy_type, str_);   // C 2182-2183
      return str_;
    }
  /* Convert a single atom to a Lisp_Symbol.  Convert a set of atoms to
     a vector of symbols.  */
  else if r#type == XA_ATOM   // C 2188
	   /* Treat ATOM_PAIR type similar to list of atoms.  */
	   || r#type == (*dpyinfo).Xatom_ATOM_PAIR
    {
      let mut i: ptrdiff_t;   // C 2192
      /* On a 64 bit machine sizeof(Atom) == sizeof(long) == 8. ... */
      let idata: *const c_int = data as *const c_int;   // C 2197

      if size == std::mem::size_of::<c_int> () as ptrdiff_t   // C 2199
	{ return x_atom_to_symbol (dpyinfo, *idata as Atom); }
      else
	{
	  let v: Lisp_Object = make_nil_vector (size / std::mem::size_of::<c_int> () as ptrdiff_t);

	  i = 0;
	  while i < size / std::mem::size_of::<c_int> () as ptrdiff_t   // C 2205
	    {
	      ASET (v, i as isize,
		    x_atom_to_symbol (dpyinfo, *idata.offset (i as isize) as Atom));
	      i += 1;
	    }
	  return v;
	}
    }

  /* Convert a single 16-bit number or a small 32-bit number to a Lisp_Int. ... */
  else if format == 32 && size == std::mem::size_of::<c_int> () as ptrdiff_t   // C 2218
    {
      if r#type == XA_INTEGER                                   // C 2220
	{ return INT_TO_INTEGER (*(data as *const c_int) as EMACS_INT); }
      else
	{ return INT_TO_INTEGER (*(data as *const c_uint) as EMACS_INT); }
    }
  else if format == 16 && size == std::mem::size_of::<c_short> () as ptrdiff_t   // C 2225
    {
      if r#type == XA_INTEGER                                   // C 2227
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

      if r#type == XA_INTEGER
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
      let v: Lisp_Object = make_nil_vector (size / X_LONG_SIZE as ptrdiff_t);

      if r#type == XA_INTEGER
	{
	  i = 0;
	  while i < size / X_LONG_SIZE as ptrdiff_t
	    {
	      let j: c_int = *(data as *const c_int).offset (i as isize);
	      ASET (v, i as isize, INT_TO_INTEGER (j as EMACS_INT));
	      i += 1;
	    }
	}
      else
	{
	  i = 0;
	  while i < size / X_LONG_SIZE as ptrdiff_t
	    {
	      let j: c_uint = *(data as *const c_uint).offset (i as isize);
	      ASET (v, i as isize, INT_TO_INTEGER (j as EMACS_INT));
	      i += 1;
	    }
	}
      return v;
    }
}

/* C 2290-2296: cons_to_x_long's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn cons_to_x_long (obj: Lisp_Object) -> c_ulong
{
  if X_ULONG_MAX as u64 <= INTMAX_MAX as u64
      || NILP (Fnatnump (if CONSP (obj) { XCAR (obj) } else { obj }))
    { return cons_to_signed (obj, X_LONG_MIN as i64,
			     min! (X_ULONG_MAX as u64, INTMAX_MAX as u64) as i64) as c_ulong; }
  else
    { return cons_to_unsigned (obj, X_ULONG_MAX as u64) as c_ulong; }
}

/* C 2295-2297: the "Use xfree, not XFree" comment above lisp_data_to_selection_data.  */

#[cfg (feature = "have_x_windows")]
unsafe fn lisp_data_to_selection_data (dpyinfo: *mut x_display_info,
                                       mut obj: Lisp_Object,
                                       cs: *mut selection_data)       // C 2302-2472
{
  let mut r#type: Lisp_Object;                                  // C 2306
  let mut name_buffer: *mut *mut c_char = std::ptr::null_mut ();   // C 2307

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                        // C 2309

  r#type = Qnil ();                                             // C 2311

  eassert! (!cs.is_null ());                                    // C 2313

  if CONSP (obj) && SYMBOLP (XCAR (obj))                        // C 2315
    {
      r#type = XCAR (obj);
      obj = XCDR (obj);
      if CONSP (obj) && NILP (XCDR (obj))
	{ obj = XCAR (obj); }
    }

  /* This is not the same as declining.  */

  if EQ (obj, QNULL ()) || EQ (r#type, QNULL ())                 // C 2325
    {
      (*cs).format = 32;
      (*cs).size = 0;
      (*cs).data = std::ptr::null_mut ();
      r#type = QNULL ();
    }
  else if STRINGP (obj)                                         // C 2332
    {
      if SCHARS (obj) < SBYTES (obj)                            // C 2334
	/* OBJ is a multibyte string containing a non-ASCII char.  */
	{ signal_error (c"Non-ASCII string must be encoded in advance".as_ptr (), obj); }

      if NILP (r#type)                                          // C 2338
	{ r#type = QSTRING (); }

      (*cs).format = 8;
      (*cs).size = -1;
      (*cs).data = std::ptr::null_mut ();
      (*cs).string = obj;
    }
  else if SYMBOLP (obj)                                         // C 2346
    {
      let data: *mut c_void = xmalloc (std::mem::size_of::<Atom> () + 1);   // C 2348
      let x_atom_ptr: *mut Atom = data as *mut Atom;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 32;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<Atom> () as isize) = 0;   // C 2353
      *x_atom_ptr = symbol_to_x_atom (dpyinfo, obj);
      if NILP (r#type) { r#type = QATOM (); }                    // C 2355
    }
  else if RANGED_FIXNUMP (X_SHRT_MIN as EMACS_INT, obj, X_SHRT_MAX as EMACS_INT)   // C 2357
    {
      let data: *mut c_void = xmalloc (std::mem::size_of::<c_short> () + 1);
      let short_ptr: *mut c_short = data as *mut c_short;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 16;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<c_short> () as isize) = 0;   // C 2364
      *short_ptr = XFIXNUM (obj) as c_short;
      if NILP (r#type) { r#type = QINTEGER (); }                 // C 2366
    }
  else if INTEGERP (obj)                                        // C 2368
	   || (CONSP (obj) && INTEGERP (XCAR (obj))
	       && (FIXNUMP (XCDR (obj))
		   || (CONSP (XCDR (obj))
		       && FIXNUMP (XCAR (XCDR (obj))))))
    {
      let data: *mut c_void = xmalloc (std::mem::size_of::<c_ulong> () + 1);   // C 2374
      let x_long_ptr: *mut c_ulong = data as *mut c_ulong;
      (*cs).data = data as *mut c_uchar;
      (*cs).format = 32;
      (*cs).size = 1;
      *(*cs).data.offset (std::mem::size_of::<c_ulong> () as isize) = 0;   // C 2379
      *x_long_ptr = cons_to_x_long (obj);
      if NILP (r#type) { r#type = QINTEGER (); }                 // C 2381
    }
  else if VECTORP (obj)                                         // C 2383
    {
      /* Lisp_Vectors may represent a set of ATOMs; ... */
      let mut i: ptrdiff_t;                                      // C 2389
      let size: ptrdiff_t = ASIZE (obj);                          // C 2390

      if size == 0                                               // C 2392
	{
	  /* This vector is empty and of unknown type.  ...  */
	  (*cs).data = std::ptr::null_mut ();
	  (*cs).format = 32;
	  (*cs).size = 0;
	  r#type = QINTEGER ();
	}
      else if SYMBOLP (AREF (obj, 0))                            // C 2402
	/* This vector is an ATOM set */
	{
	  let data: *mut c_void;                                  // C 2405
	  let x_atoms: *mut Atom;                                 // C 2406
	  if NILP (r#type) { r#type = QATOM (); }                  // C 2407
	  i = 0;                                                  // C 2408
	  while i < size
	    {
	      if !SYMBOLP (AREF (obj, i as isize))
		{ signal_error (c"All elements of selection vector must have same type".as_ptr (),
				obj); }
	      i += 1;
	    }

	  data = xnmalloc (size, std::mem::size_of::<Atom> () as isize);   // C 2412
	  (*cs).data = data as *mut c_uchar;
	  x_atoms = data as *mut Atom;
	  (*cs).format = 32;
	  (*cs).size = size;

	  if size == 1                                            // C 2417
	    { *x_atoms = symbol_to_x_atom (dpyinfo, AREF (obj, i as isize)); }
	  else
	    {
	      SAFE_NALLOCA! (sa_avail, name_buffer,
			     std::mem::size_of::<Atom> () as isize, size);   // C 2421

	      i = 0;                                              // C 2423
	      while i < size
		{
		  *name_buffer.offset (i as isize) = SSDATA (SYMBOL_NAME (AREF (obj, i as isize)));
		  i += 1;
		}

	      x_intern_atoms (dpyinfo, name_buffer, size as c_int, x_atoms);   // C 2426-2427
	    }
	}
      else
	/* This vector is an INTEGER set, or something like it */
	{
	  let mut format: c_int = 16;                              // C 2433
	  let mut data_size: c_int = std::mem::size_of::<c_short> () as c_int;   // C 2434
	  let data: *mut c_void;                                   // C 2435
	  let x_atoms: *mut c_ulong;                               // C 2436
	  let shorts: *mut c_short;                                // C 2437
	  if NILP (r#type) { r#type = QINTEGER (); }                // C 2438
	  i = 0;                                                  // C 2439
	  while i < size
	    {
	      if !RANGED_FIXNUMP (X_SHRT_MIN as EMACS_INT, AREF (obj, i as isize),
				  X_SHRT_MAX as EMACS_INT)
		{
		  /* Use sizeof (long) even if it is more than 32 bits. ...  */
		  data_size = std::mem::size_of::<c_long> () as c_int;   // C 2447
		  format = 32;
		  break;
		}
	      i += 1;
	    }
	  data = xnmalloc (size, data_size as isize);               // C 2452
	  (*cs).data = data as *mut c_uchar;
	  x_atoms = data as *mut c_ulong;
	  shorts = data as *mut c_short;
	  (*cs).format = format;
	  (*cs).size = size;
	  i = 0;                                                  // C 2457
	  while i < size
	    {
	      if format == 32
		{ *x_atoms.offset (i as isize) = cons_to_x_long (AREF (obj, i as isize)); }
	      else
		{ *shorts.offset (i as isize) = XFIXNUM (AREF (obj, i as isize)) as c_short; }
	      i += 1;
	    }
	}
    }
  else
    { signal_error (c"Unrecognized selection data".as_ptr (), obj); }   // C 2467

  (*cs).r#type = symbol_to_x_atom (dpyinfo, r#type);              // C 2469

  SAFE_FREE! (sa_count, sa_mark);                                        // C 2471
}

#[cfg (feature = "have_x_windows")]
unsafe fn clean_local_selection_data (mut obj: Lisp_Object) -> Lisp_Object   // C 2474-2506
{
  if CONSP (obj)                                                // C 2477
      && INTEGERP (XCAR (obj))
      && CONSP (XCDR (obj))
      && FIXNUMP (XCAR (XCDR (obj)))
      && NILP (XCDR (XCDR (obj)))
    { obj = Fcons (XCAR (obj), XCDR (obj)); }

  if CONSP (obj)                                                // C 2484
      && INTEGERP (XCAR (obj))
      && FIXNUMP (XCDR (obj))
    {
      if BASE_EQ (XCAR (obj), make_fixnum (0))                  // C 2488
	{ return XCDR (obj); }
      if BASE_EQ (XCAR (obj), make_fixnum (-1))                 // C 2490
	{ return make_fixnum (-XFIXNUM (XCDR (obj))); }
    }
  if VECTORP (obj)                                              // C 2493
    {
      let mut i: ptrdiff_t;
      let size: ptrdiff_t = ASIZE (obj);
      let copy: Lisp_Object;
      if size == 1                                              // C 2498
	{ return clean_local_selection_data (AREF (obj, 0)); }
      copy = make_nil_vector (size);                            // C 2500
      i = 0;                                                    // C 2501
      while i < size
	{
	  ASET (copy, i as isize, clean_local_selection_data (AREF (obj, i as isize)));
	  i += 1;
	}
      return copy;
    }
  return obj;                                                   // C 2505
}

/* C 2508-2511: x_handle_selection_notify's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_handle_selection_notify (event: *const XSelectionEvent)   // C 2513-2524
{
  if (*event).requestor != reading_selection_window             // C 2516
    { return; }
  if (*event).selection != reading_which_selection               // C 2518
    { return; }

  TRACE1! ("Received SelectionNotify: %d", (*event).property as c_int);   // C 2521
  XSETCAR (reading_selection_reply,
	   if (*event).property != 0 { Qt () } else { Qlambda () });   // C 2522-2523
}

/* C 2527-2532: frame_for_x_selection's comment.  */

#[cfg (feature = "have_x_windows")]
unsafe fn frame_for_x_selection (object: Lisp_Object) -> *mut frame      // C 2533-2572
{
  let mut tail: Lisp_Object;                                    // C 2536
  let mut frame: Lisp_Object;
  let mut f: *mut frame;                                        // C 2537

  if NILP (object)                                              // C 2539
    {
      f = XFRAME (selected_frame);
      if FRAME_X_P (f) && FRAME_LIVE_P (f)
	{ return f; }

      FOR_EACH_FRAME! (tail, frame,                              // C 2545-2550
	{
	  f = XFRAME (frame);
	  if FRAME_X_P (f) && FRAME_LIVE_P (f)
	    { return f; }
	});
    }
  else if TERMINALP (object)                                    // C 2552
    {
      let t: *mut terminal = decode_live_terminal (object);      // C 2554

      if (*t).r#type == output_x_window                          // C 2556
	{ FOR_EACH_FRAME! (tail, frame,
	    {
	      f = XFRAME (frame);
	      if FRAME_LIVE_P (f) && (*f).terminal == t
		{ return f; }
	    }); }
    }
  else if FRAMEP (object)                                       // C 2564
    {
      f = XFRAME (object);
      if FRAME_X_P (f) && FRAME_LIVE_P (f)
	{ return f; }
    }

  return std::ptr::null_mut ();                                 // C 2571
}

/* C 2574: the blank and the DEFUN.  */

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-own-selection-internal", Fx_own_selection_internal,
	Sx_own_selection_internal, 2, 3, 0,                     // C 2575-2576
	doc: "Assert an X selection of type SELECTION and value VALUE.\nSELECTION is a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nVALUE is typically a string, or a cons of two markers, but may be\nanything that the functions on `selection-converter-alist' know about.\n\nFRAME should be a frame that should own the selection.  If omitted or\nnil, it defaults to the selected frame.\n\nOn Nextstep, FRAME is unused.",
	(selection: Lisp_Object, value: Lisp_Object, frame: Lisp_Object),
	{
	  /* C 2589-2595 reassigns both `frame` and `selection`, so each parameter is shadowed by a
	     mutable local of the same name (the `src/data.rs` 1097 reading).  */
	  let mut frame = frame;
	  let mut selection = selection;
	  if NILP (frame) { frame = selected_frame; }               // C 2589
	  if !FRAME_LIVE_P (XFRAME (frame)) || !FRAME_X_P (XFRAME (frame))
	    { error (c"X selection unavailable for this frame".as_ptr ()); }

	  CHECK_SYMBOL (selection);                                 // C 2593
	  if NILP (value) { error (c"VALUE may not be nil".as_ptr ()); }
	  x_own_selection (selection, value, frame, Qnil (), 0);    // C 2595
	  return value;
	});

/* C 2600-2602: x-get-selection-internal's comment.  */

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-get-selection-internal", Fx_get_selection_internal,
	Sx_get_selection_internal, 2, 4, 0,                     // C 2604-2605
	doc: "Return text selected from some X window.\nSELECTION-SYMBOL is typically `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nTARGET-TYPE is the type of data desired, typically `STRING'.\n\nTIME-STAMP is the time to use in the XConvertSelection call for foreign\nselections.  If omitted, defaults to the time for the last event.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.\n\nOn Nextstep, TIME-STAMP and TERMINAL are unused.",
	(selection_symbol: Lisp_Object, target_type: Lisp_Object,
	 time_stamp: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection_symbol = selection_symbol;   // C 2640 reassigns the parameter
	  let mut val: Lisp_Object = Qnil ();                       // C 2624
	  let maybe_alias: Lisp_Object;                             // C 2625
	  let f: *mut frame = frame_for_x_selection (terminal);      // C 2626

	  CHECK_SYMBOL (selection_symbol);                           // C 2628
	  CHECK_SYMBOL (target_type);

	  if EQ (target_type, QMULTIPLE ())                          // C 2631
	    { error (c"Retrieving MULTIPLE selections is currently unimplemented".as_ptr ()); }
	  if f.is_null ()                                            // C 2633
	    { error (c"X selection unavailable for this frame".as_ptr ()); }

	  /* Quitting inside this function is okay, ...  */
	  maybe_alias = Fassq (selection_symbol, Vx_selection_alias_alist);   // C 2637

	  if !NILP (maybe_alias)                                     // C 2639
	    {
	      selection_symbol = XCDR (maybe_alias);
	      CHECK_SYMBOL (selection_symbol);
	    }

	  val = x_get_local_selection (selection_symbol, target_type, true,
				       FRAME_DISPLAY_INFO! (f), Qnil (), false);   // C 2645-2646

	  if NILP (val) && FRAME_LIVE_P (f)                          // C 2648
	    {
	      let mut frame: Lisp_Object;
	      XSETFRAME! (frame, f);
	      return x_get_foreign_selection (selection_symbol, target_type,
					      time_stamp, frame);
	    }

	  if CONSP (val) && SYMBOLP (XCAR (val))                     // C 2656
	    {
	      val = XCDR (val);
	      if CONSP (val) && NILP (XCDR (val))
		{ val = XCAR (val); }
	    }
	  return clean_local_selection_data (val);                    // C 2662
	});

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-disown-selection-internal", Fx_disown_selection_internal,
	Sx_disown_selection_internal, 1, 3, 0,                  // C 2664-2665
	doc: "If we own the selection SELECTION, disown it.\nDisowning it means there is no such selection.\n\nSets the last-change time for the selection to TIME-OBJECT (by default\nthe time of the last event).\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.\n\nOn Nextstep, the TIME-OBJECT and TERMINAL arguments are unused.\nOn MS-DOS, all this does is return non-nil if we own the selection.",
	(selection: Lisp_Object, time_object: Lisp_Object, terminal: Lisp_Object),
	{
	  let timestamp: Time;                                      // C 2698
	  let selection_atom: Atom;
	  let mut event: selection_input_event = std::mem::zeroed ();   // C 2700
	  let f: *mut frame = frame_for_x_selection (terminal);
	  let dpyinfo: *mut x_display_info;

	  if f.is_null ()                                           // C 2704
	    { return Qnil (); }

	  dpyinfo = FRAME_DISPLAY_INFO! (f);                        // C 2707
	  CHECK_SYMBOL (selection);

	  /* Don't disown the selection when we're not the owner.  */
	  if NILP (LOCAL_SELECTION! (selection, dpyinfo))           // C 2711
	    { return Qnil (); }

	  selection_atom = symbol_to_x_atom (dpyinfo, selection);   // C 2714

	  block_input ();                                          // C 2716
	  if NILP (time_object)                                    // C 2717
	    { timestamp = (*dpyinfo).last_user_time; }
	  else
	    { CONS_TO_INTEGER! (time_object, timestamp); }
	  XSetSelectionOwner ((*dpyinfo).display, selection_atom, 0, timestamp);
	  unblock_input ();

	  /* It doesn't seem to be guaranteed that a SelectionClear event will be
	     generated for a window which owns the selection when that window sets
	     the selection owner to None.  ...  */
	  SELECTION_EVENT_DPYINFO! (&raw mut event) = dpyinfo;       // C 2739
	  SELECTION_EVENT_SELECTION! (&raw mut event) = selection_atom;
	  SELECTION_EVENT_TIME! (&raw mut event) = timestamp;
	  x_handle_selection_clear (&raw mut event);                // C 2742

	  return Qt ();                                            // C 2744
	});

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-selection-owner-p", Fx_selection_owner_p, Sx_selection_owner_p,
	0, 2, 0,                                                // C 2719-2720
	doc: "Whether the current Emacs process owns the given X Selection.\nThe arg should be the name of the selection in question, typically one of\nthe symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.\n(Those are literal upper-case symbol names, since that's what X expects.)\nFor convenience, the symbol nil is the same as `PRIMARY',\nand t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.\n\nOn Nextstep, TERMINAL is unused.",
	(selection: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection = selection;   // C 2742-2743 reassigns the parameter
	  let f: *mut frame = frame_for_x_selection (terminal);      // C 2739

	  CHECK_SYMBOL (selection);                                 // C 2741
	  if NILP (selection) { selection = QPRIMARY (); }           // C 2742
	  if EQ (selection, Qt ()) { selection = QSECONDARY (); }     // C 2743

	  if !f.is_null () && !NILP (LOCAL_SELECTION! (selection, FRAME_DISPLAY_INFO! (f)))
	    { return Qt (); }                                       // C 2746-2747
	  else
	    { return Qnil (); }
	});

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-selection-exists-p", Fx_selection_exists_p, Sx_selection_exists_p,
	0, 2, 0,                                                // C 2747-2748
	doc: "Whether there is an owner for the given X selection.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', `CLIPBOARD', or\n`CLIPBOARD_MANAGER' (X expects these literal upper-case names.)  The\nsymbol nil is the same as `PRIMARY', and t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.\n\nOn Nextstep, TERMINAL is unused.",
	(selection: Lisp_Object, terminal: Lisp_Object),
	{
	  let mut selection = selection;   // C 2775-2779 reassigns the parameter
	  let mut owner: Window;                                    // C 2769
	  let atom: Atom;
	  #[cfg (feature = "have_xfixes")]
	  let mut temp_owner: Window;
	  let f: *mut frame = frame_for_x_selection (terminal);
	  let dpyinfo: *mut x_display_info;

	  CHECK_SYMBOL (selection);                                 // C 2773

	  if NILP (selection)                                       // C 2775
	    { selection = QPRIMARY (); }

	  if EQ (selection, Qt ())                                   // C 2778
	    { selection = QSECONDARY (); }

	  if f.is_null ()                                           // C 2781
	    { return Qnil (); }

	  dpyinfo = FRAME_DISPLAY_INFO! (f);                        // C 2784

	  if !NILP (LOCAL_SELECTION! (selection, dpyinfo))           // C 2786
	    { return Qt (); }

	  atom = symbol_to_x_atom (dpyinfo, selection);             // C 2788

	  if atom == 0                                              // C 2790
	    { return Qnil (); }

	  #[cfg (feature = "have_xfixes")]
	  {
	    /* See if this information can be obtained without a roundtrip.  */
	    temp_owner = x_find_selection_owner (dpyinfo, atom);    // C 2793

	    if temp_owner != X_INVALID_WINDOW                       // C 2795
	      { return if temp_owner != 0 { Qt () } else { Qnil () }; }
	  }

	  block_input ();                                          // C 2800
	  owner = XGetSelectionOwner ((*dpyinfo).display, atom);
	  unblock_input ();

	  return if owner != 0 { Qt () } else { Qnil () };          // C 2803
	});

/* The `lisp.h`/`eval.c`/`lisp.h`-4389 names the clipboard-manager and DND functions use that no
   included mirror carries.  */
#[cfg (feature = "have_x_windows")]
unsafe extern "C" {
  /// C: lisp.h 4806: `extern Lisp_Object internal_condition_case_1 (Lisp_Object (*) (Lisp_Object),
  /// Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  pub fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
                                    arg: Lisp_Object, handlers: Lisp_Object,
                                    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  /// C: lisp.h 4389: `extern void message (const char *, ...);`
  pub fn message (format: *const c_char, ...);
  /// C: lisp.h 4436: `extern void mark_object (Lisp_Object);`
  pub fn mark_object (obj: Lisp_Object);
}

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-get-local-selection", Fx_get_local_selection, Sx_get_local_selection,
	0, 2, 0,                                                // C 2806-2807
	doc: "Run selection converters for VALUE, and return the result.\nTARGET is the selection target that is used to find a suitable\nconverter.  VALUE is a list of 4 values NAME, SELECTION-VALUE,\nTIMESTAMP and FRAME.  NAME is the name of the selection that will be\npassed to selection converters, SELECTION-VALUE is the value of the\nselection used by the converter, TIMESTAMP is not meaningful (but must\nbe a number that fits in an X timestamp), and FRAME is the frame\ndescribing the terminal for which the selection converter will be\nrun.",
	(value: Lisp_Object, target: Lisp_Object),
	{
	  let mut time: Time = 0;                                   // C 2837
	  let name: Lisp_Object;
	  let timestamp: Lisp_Object;
	  let frame: Lisp_Object;
	  let result: Lisp_Object;

	  CHECK_SYMBOL (target);

	  /* Check that VALUE has 4 elements, for x_get_local_selection.  */
	  let mut v: Lisp_Object = value; CHECK_CONS (v);
	  name = XCAR (v); v = XCDR (v); CHECK_CONS (v);
	  v = XCDR (v); CHECK_CONS (v);
	  timestamp = XCAR (v); v = XCDR (v); CHECK_CONS (v);
	  frame = XCAR (v);

	  CHECK_SYMBOL (name);
	  CONS_TO_INTEGER! (timestamp, time);
	  check_window_system (decode_live_frame (frame));

	  result = x_get_local_selection (name, target, true,
					  std::ptr::null_mut (), value, false);

	  if CONSP (result) && SYMBOLP (XCAR (result))
	    {
	      let mut result = result;
	      result = XCDR (result);

	      if CONSP (result) && NILP (XCDR (result))
		{ result = XCAR (result); }
	      return clean_local_selection_data (result);
	    }

	  return clean_local_selection_data (result);
	});

/* C 2871-2872: the "Send clipboard manager a SAVE_TARGETS request" comment.  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_clipboard_manager_save (frame: Lisp_Object) -> Lisp_Object   // C 2853-2869
{
  use crate::x11_constants::PropModeReplace;

  let f: *mut frame = XFRAME (frame);
  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
  let mut data: Atom = (*dpyinfo).Xatom_UTF8_STRING;

  XChangeProperty (FRAME_X_DISPLAY! (f), FRAME_X_WINDOW! (f),
		   (*dpyinfo).Xatom_EMACS_TMP,
		   (*dpyinfo).Xatom_ATOM, 32, PropModeReplace,
		   &raw mut data as *mut c_uchar, 1);
  x_get_foreign_selection (QCLIPBOARD_MANAGER (), QSAVE_TARGETS (),
			   Qnil (), frame);
  return Qt ();
}

/* C 2871: "Error handler for x_clipboard_manager_save_frame."  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_clipboard_manager_error_1 (err: Lisp_Object) -> Lisp_Object   // C 2871-2881
{
  AUTO_STRING! (format, c"X clipboard manager error: %s\nIf the problem persists, set `%s' to nil.".as_ptr ());
  AUTO_STRING! (varname, c"x-select-enable-clipboard-manager".as_ptr ());
  CALLN! (Fmessage, format, CAR (CDR (err)), varname);
  return Qnil ();
}

/* C 2883: "Error handler for x_clipboard_manager_save_all."  */

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn x_clipboard_manager_error_2 (err: Lisp_Object) -> Lisp_Object   // C 2883-2896
{
  fputs (c"Error saving to X clipboard manager.\nIf the problem persists, set 'x-select-enable-clipboard-manager' to nil.\n".as_ptr (),
	 stderr);
  return Qnil ();
}

/* C 2897-2900: x_clipboard_manager_save_frame's comment ("Called from delete_frame: ...").  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_clipboard_manager_save_frame (frame: Lisp_Object)   // C 2897-2923
{
  let f: *mut frame;

  if !NILP (Vx_select_enable_clipboard_manager)
      && FRAMEP (frame)
      && ({ f = XFRAME (frame); FRAME_X_P (f) })
      && FRAME_LIVE_P (f)
    {
      let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);
      let local_selection: Lisp_Object
	= LOCAL_SELECTION! (QCLIPBOARD (), dpyinfo);

      if !NILP (local_selection)
	  && EQ (frame, XCAR (XCDR (XCDR (XCDR (local_selection)))))
	  && XGetSelectionOwner ((*dpyinfo).display,
				 (*dpyinfo).Xatom_CLIPBOARD_MANAGER) != 0
	{ internal_condition_case_1 (Some (x_clipboard_manager_save), frame, Qt (),
				     Some (x_clipboard_manager_error_1)); }
    }
}

/* C 2924-2926: x_clipboard_manager_save_all's comment ("Called from Fkill_emacs: ...").  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_clipboard_manager_save_all ()   // C 2924-2960
{
  /* Loop through all X displays, saving owned clipboards.  */
  let mut dpyinfo: *mut x_display_info;
  let mut local_selection: Lisp_Object;
  let mut local_frame: Lisp_Object;

  if NILP (Vx_select_enable_clipboard_manager)
    { return; }

  dpyinfo = x_display_list;
  while !dpyinfo.is_null ()
    {
      local_selection = LOCAL_SELECTION! (QCLIPBOARD (), dpyinfo);
      if NILP (local_selection)
	  || XGetSelectionOwner ((*dpyinfo).display,
				 (*dpyinfo).Xatom_CLIPBOARD_MANAGER) == 0
	{ dpyinfo = (*dpyinfo).next; continue; }

      local_frame = XCAR (XCDR (XCDR (XCDR (local_selection))));
      if FRAME_LIVE_P (XFRAME (local_frame))
	{
	  message (c"Saving clipboard to X clipboard manager...".as_ptr ());
	  internal_condition_case_1 (Some (x_clipboard_manager_save), local_frame,
				     Qt (), Some (x_clipboard_manager_error_2));
	}
      dpyinfo = (*dpyinfo).next;
    }
}

/* C 2961-2966: the Drag-and-drop banner and x_check_property_data's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_check_property_data (data: Lisp_Object) -> c_int   // C 2961-2997
{
  let mut iter: Lisp_Object;
  let mut size: c_int = 0;

  iter = data;
  while CONSP (iter)
    {
      let o: Lisp_Object = XCAR (iter);

      if !NUMBERP (o) && !STRINGP (o) && !CONSP (o)
	{ return -1; }
      else if CONSP (o)
	  && (!NUMBERP (XCAR (o)) || !NUMBERP (XCDR (o)))
	{ return -1; }
      if size == INT_MAX
	{ return -1; }
      size += 1;
      iter = XCDR (iter);
    }

  return size;
}

/* C 2988-2997: x_fill_property_data's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_fill_property_data (dpyinfo: *mut x_display_info, data: Lisp_Object,   // C 2998-3072
                                               ret: *mut c_void, nelements_max: c_int,
                                               format: c_int)
{
  let mut val: c_ulong;
  let mut d32: *mut c_ulong = ret as *mut c_ulong;
  let mut d16: *mut c_ushort = ret as *mut c_ushort;
  let mut d08: *mut c_uchar = ret as *mut c_uchar;
  let mut nelements: c_int;
  let mut iter: Lisp_Object;

  iter = data;
  nelements = 0;
  while CONSP (iter) && nelements < nelements_max
    {
      let o: Lisp_Object = XCAR (iter);

      if NUMBERP (o) || CONSP (o)
	{
	  if CONSP (o)
	      && RANGED_FIXNUMP ((X_LONG_MIN >> 16) as EMACS_INT, XCAR (o),
				 (X_LONG_MAX >> 16) as EMACS_INT)
	      && RANGED_FIXNUMP (-(1 << 15) as EMACS_INT, XCDR (o), -1)
	    {
	      /* cons_to_x_long does not handle negative values for v2. ...  */
	      let v1: c_ulong = XFIXNUM (XCAR (o)) as c_ulong & 0xffff;
	      let v2: c_ulong = XFIXNUM (XCDR (o)) as c_ulong & 0xffff;
	      val = (v1 << 16) | v2;
	    }
	  else
	    { val = cons_to_x_long (o); }
	}
      else if STRINGP (o)
	{
	  block_input ();
	  val = x_intern_cached_atom (dpyinfo, SSDATA (o), false) as c_ulong;
	  unblock_input ();
	}
      else
	{ error (c"Wrong type, must be string, number or cons".as_ptr ()); }

      if format == 8
	{
	  if (1 << 8) < val && val <= X_ULONG_MAX - (1 << 7)
	    { error (c"Out of `char' range".as_ptr ()); }
	  *d08 = val as c_uchar;
	  d08 = d08.offset (1);
	}
      else if format == 16
	{
	  if (1 << 16) < val && val <= X_ULONG_MAX - (1 << 15)
	    { error (c"Out of `short' range".as_ptr ()); }
	  *d16 = val as c_ushort;
	  d16 = d16.offset (1);
	}
      else
	{ *d32 = val; d32 = d32.offset (1); }

      iter = XCDR (iter);
      nelements += 1;
    }
}

/* C 3050-3071: x_property_data_to_lisp's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_property_data_to_lisp (f: *mut frame, data: *const c_uchar,   // C 3073-3083
                                                  r#type: Atom, format: c_int,
                                                  size: c_ulong) -> Lisp_Object
{
  let format_bytes: ptrdiff_t = (format >> 3) as ptrdiff_t;
  let mut data_bytes: ptrdiff_t = 0;
  /* C 3078: `if (ckd_mul (&data_bytes, size, format_bytes)) memory_full_up ();` (P-15).  */
  let (data_bytes_v, overflow) = (size as ptrdiff_t).overflowing_mul (format_bytes);
  data_bytes = data_bytes_v;
  if overflow
    { memory_full_up (); }
  return selection_data_to_lisp_data (FRAME_DISPLAY_INFO! (f), data,
				      data_bytes, r#type, format);
}

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-get-atom-name", Fx_get_atom_name, Sx_get_atom_name,   // C 3085-3121
	1, 2, 0,
	doc: "Return the X atom name for VALUE as a string.\nVALUE may be a number or a cons where the car is the upper 16 bits and\nthe cdr is the lower 16 bits of a 32 bit value.\nUse the display for FRAME or the current frame if FRAME is not given or nil.\n\nIf the value is 0 or the atom is not known, return the empty string.",
	(value: Lisp_Object, frame: Lisp_Object),
	{
	  let f: *mut frame = decode_window_system_frame (frame);
	  let dpy: *mut Display = FRAME_X_DISPLAY! (f);
	  let dpyinfo: *mut x_display_info;
	  let mut atom: Atom = 0;
	  let mut had_errors_p: bool = false;
	  let mut need_sync: bool = false;
	  let mut name: *mut c_char;
	  let mut ret: Lisp_Object;

	  dpyinfo = FRAME_DISPLAY_INFO! (f);
	  CONS_TO_INTEGER! (value, atom);

	  x_catch_errors (dpy);
	  name = x_get_atom_name (dpyinfo, atom, &raw mut need_sync);
	  had_errors_p = need_sync && x_had_errors_p (dpy);
	  x_uncatch_errors_after_check ();

	  ret = empty_unibyte_string;

	  if !name.is_null ()
	    {
	      if !had_errors_p
		{ ret = build_string (name); }
	      xfree (name as *mut c_void);
	    }

	  return ret;
	});

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-register-dnd-atom", Fx_register_dnd_atom, Sx_register_dnd_atom,   // C 3123-3160
	1, 2, 0,
	doc: "Request that dnd events are made for ClientMessages with ATOM.\nATOM can be a symbol or a string.  The ATOM is interned on the display that\nFRAME is on.  If FRAME is nil, the selected frame is used.",
	(atom: Lisp_Object, frame: Lisp_Object),
	{
	  let x_atom: Atom;
	  let f: *mut frame = decode_window_system_frame (frame);
	  let mut i: ptrdiff_t;
	  let dpyinfo: *mut x_display_info = FRAME_DISPLAY_INFO! (f);

	  if SYMBOLP (atom)
	    { x_atom = symbol_to_x_atom (dpyinfo, atom); }
	  else if STRINGP (atom)
	    {
	      block_input ();
	      x_atom = x_intern_cached_atom (dpyinfo, SSDATA (atom),
					     false);
	      unblock_input ();
	    }
	  else
	    { error (c"ATOM must be a symbol or a string".as_ptr ()); }

	  i = 0;
	  while i < (*dpyinfo).x_dnd_atoms_length
	    {
	      if *(*dpyinfo).x_dnd_atoms.offset (i as isize) == x_atom
		{ return Qnil (); }
	      i += 1;
	    }

	  if (*dpyinfo).x_dnd_atoms_length == (*dpyinfo).x_dnd_atoms_size
	    { (*dpyinfo).x_dnd_atoms =
		xpalloc ((*dpyinfo).x_dnd_atoms as *mut c_void, &raw mut (*dpyinfo).x_dnd_atoms_size,
			 1, -1, std::mem::size_of::<Atom> () as ptrdiff_t) as *mut Atom; }

	  *(*dpyinfo).x_dnd_atoms.offset ((*dpyinfo).x_dnd_atoms_length as isize) = x_atom;
	  (*dpyinfo).x_dnd_atoms_length += 1;
	  return Qnil ();
	});

/* C 3162: the "Convert an XClientMessageEvent to a Lisp event of type DRAG_N_DROP_EVENT." comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_handle_dnd_message (f: *mut frame, event: *const XClientMessageEvent,   // C 3162-3220
                                               dpyinfo: *mut x_display_info,
                                               bufp: *mut input_event,
                                               root_window_coords: bool, root_x: c_int,
                                               root_y: c_int) -> bool
{
  let vec: Lisp_Object;
  let frame: Lisp_Object;
  /* format 32 => size 5, format 16 => size 10, format 8 => size 20 */
  let size: c_ulong = (160 / (*event).format) as c_ulong;
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut data: *mut c_uchar = core::ptr::addr_of! ((*event).data.b) as *mut c_uchar;   // C 3171
  let mut idata: [c_int; 5] = [0; 5];                            // C 3172
  let mut i: ptrdiff_t;                                          // C 3173

  i = 0;                                                        // C 3175
  while i < (*dpyinfo).x_dnd_atoms_length
    {
      if *(*dpyinfo).x_dnd_atoms.offset (i as isize) == (*event).message_type
	{ break; }
      i += 1;
    }

  if i == (*dpyinfo).x_dnd_atoms_length { return false; }        // C 3177

  XSETFRAME! (frame, f);                                        // C 3179

  /* On a 64 bit machine, the event->data.l array members are 64 bits (long),
     but the x_property_data_to_lisp (or rather selection_data_to_lisp_data)
     function expects them to be of size int (i.e. 32).  ...  */

  if LONG_WIDTH > 32 && (*event).format == 32                    // C 3185
    {
      i = 0;                                                    // C 3187
      while i < 5 /* There are only 5 longs in a ClientMessage. */
	{
	  idata[i as usize] = (*event).data.l[i as usize] as c_int;
	  i += 1;
	}
      data = core::ptr::addr_of! (idata) as *mut c_uchar;         // C 3190
    }

  vec = make_nil_vector (4);                                    // C 3193
  ASET (vec, 0, SYMBOL_NAME (x_atom_to_symbol (FRAME_DISPLAY_INFO! (f),
					       (*event).message_type)));
  ASET (vec, 1, frame);
  ASET (vec, 2, make_fixnum ((*event).format as EMACS_INT));
  ASET (vec, 3, x_property_data_to_lisp (f,
					 data,
					 (*event).message_type,
					 (*event).format,
					 size));

  if !root_window_coords                                         // C 3205
    { x_relative_mouse_position (f, &raw mut x, &raw mut y); }
  else
    { x_translate_coordinates (f, root_x, root_y, &raw mut x, &raw mut y); }

  /* C 3210: `bufp->kind = DRAG_N_DROP_EVENT;` - `kind` is termhooks.h 369's 16-bit bitfield, and
     the port's `input_event` keeps the unit as `bits` (PORTING.md 1.3, P-80), so the store is the
     masked write `keyboard.rs` 4065's `SET_INPUT_EVENT_KIND!` spells.  */
  (*bufp).bits = ((*bufp).bits & !INPUT_EVENT_KIND_MASK)
    | ((DRAG_N_DROP_EVENT as u32) & INPUT_EVENT_KIND_MASK);
  (*bufp).frame_or_window = frame;                              // C 3211
  (*bufp).timestamp = CurrentTime;                              // C 3212
  (*bufp).x = make_fixnum (x as EMACS_INT);                     // C 3213
  (*bufp).y = make_fixnum (y as EMACS_INT);                     // C 3214
  (*bufp).arg = vec;                                            // C 3215
  (*bufp).modifiers = 0;                                        // C 3216

  return true;                                                  // C 3218
}

#[cfg (feature = "have_x_windows")]
DEFUN! ("x-send-client-message", Fx_send_client_message, Sx_send_client_message,
	6, 6, 0,                                                // C 3222-3223
	doc: "Send a client message of MESSAGE-TYPE to window DEST on DISPLAY.\n\nFor DISPLAY, specify either a frame or a display name (a string).\nIf DISPLAY is nil, that stands for the selected frame's display.\nDEST may be a number, in which case it is a Window id.  The value 0 may\nbe used to send to the root window of the DISPLAY.\nIf DEST is a cons, it is converted to a 32 bit number\nwith the high 16 bits from the car and the lower 16 bit from the cdr.  That\nnumber is then used as a window id.\nIf DEST is a frame the event is sent to the outer window of that frame.\nA value of nil means the currently selected frame.\nIf DEST is the string \"PointerWindow\" the event is sent to the window that\ncontains the pointer.  If DEST is the string \"InputFocus\" the event is\nsent to the window that has the input focus.\nFROM is the frame sending the event.  Use nil for currently selected frame.\nMESSAGE-TYPE is the name of an Atom as a string.\nFORMAT must be one of 8, 16 or 32 and determines the size of the values in\nbits.  VALUES is a list of numbers, cons and/or strings containing the values\nto send.  If a value is a string, it is converted to an Atom and the value of\nthe Atom is sent.  If a value is a cons, it is converted to a 32 bit number\nwith the high 16 bits from the car and the lower 16 bit from the cdr.\nIf more values than fits into the event is given, the excessive values\nare ignored.\n\nWait for the event to be sent and signal any error, unless\n`x-fast-protocol-requests' is non-nil, in which case errors will be\nsilently ignored.",
	(display: Lisp_Object, dest: Lisp_Object, from: Lisp_Object,
	 message_type: Lisp_Object, format: Lisp_Object, values: Lisp_Object),
	{
	  let dpyinfo: *mut x_display_info = check_x_display_info (display);   // C 3258

	  CHECK_STRING (message_type);                              // C 3260
	  x_send_client_event (display, dest, from,
			       XInternAtom ((*dpyinfo).display,
					    SSDATA (message_type),
					    false as Bool),
			       format, values);                 // C 3261-3265

	  return Qnil ();                                           // C 3267
	});

#[cfg (feature = "have_x_windows")]
unsafe fn x_send_client_event (display: Lisp_Object, dest: Lisp_Object, from: Lisp_Object,
                               message_type: Atom, format: Lisp_Object,
                               values: Lisp_Object)             // C 3266-3348
{
  use crate::x11_constants::ClientMessage;

  let dpyinfo: *mut x_display_info = check_x_display_info (display);   // C 3270
  let mut wdest: Window = 0;                                    // C 3271
  let mut event: XEvent = std::mem::zeroed ();                  // C 3272
  let f: *mut frame = decode_window_system_frame (from);         // C 3273
  let mut to_root: bool;                                        // C 3274

  CHECK_FIXNUM (format);                                        // C 3276
  CHECK_CONS (values);

  if x_check_property_data (values) == -1                       // C 3279
    { error (c"Bad data in VALUES, must be number, cons or string".as_ptr ()); }

  if XFIXNUM (format) != 8 && XFIXNUM (format) != 16 && XFIXNUM (format) != 32
    { error (c"FORMAT must be one of 8, 16 or 32".as_ptr ()); }  // C 3282

  /* C 3285-3286: C's `event.xclient`, reached through the measured record (P-72 ③).  */
  let xclient: *mut XClientMessageEvent = &raw mut event as *mut XClientMessageEvent;

  (*xclient).r#type = ClientMessage;                            // C 3285
  (*xclient).format = XFIXNUM (format) as c_int;                // C 3286

  if FRAMEP (dest) || NILP (dest)                               // C 3288
    {
      let fdest: *mut frame = decode_window_system_frame (dest);   // C 3290
      wdest = FRAME_OUTER_WINDOW! (fdest);                      // C 3291
    }
  else if STRINGP (dest)                                        // C 3293
    {
      if strcmp (SSDATA (dest), c"PointerWindow".as_ptr ()) == 0
	{ wdest = PointerWindow; }                              // C 3295
      else if strcmp (SSDATA (dest), c"InputFocus".as_ptr ()) == 0
	{ wdest = InputFocus; }                                 // C 3297
      else
	{ error (c"DEST as a string must be one of PointerWindow or InputFocus".as_ptr ()); }
    }
  else if NUMBERP (dest) || CONSP (dest)                        // C 3301
    { CONS_TO_INTEGER! (dest, wdest); }
  else
    { error (c"DEST must be a frame, nil, string, number or cons".as_ptr ()); }

  if wdest == 0 { wdest = (*dpyinfo).root_window; }              // C 3307
  to_root = wdest == (*dpyinfo).root_window;                     // C 3308

  block_input ();                                               // C 3310

  (*xclient).send_event = true as Bool;                         // C 3312
  (*xclient).serial = 0;                                        // C 3313
  (*xclient).message_type = message_type;                       // C 3314
  (*xclient).display = (*dpyinfo).display;                       // C 3315

  /* Some clients (metacity for example) expects sending window to be here
     when sending to the root window.  */
  (*xclient).window = if to_root { FRAME_OUTER_WINDOW! (f) } else { wdest };   // C 3319

  memset (core::ptr::addr_of_mut! ((*xclient).data.l) as *mut c_void, 0,
	  std::mem::size_of_val (&(*xclient).data.l));          // C 3321
  /* event.xclient.data can hold 20 chars, 10 shorts, or 5 longs.  */
  x_fill_property_data (dpyinfo, values,
			core::ptr::addr_of_mut! ((*xclient).data.b) as *mut c_void,
			5 * 32 / (*xclient).format,
			(*xclient).format);                     // C 3323-3325

  /* If event mask is 0 the event is sent to the client that created
     the destination window.  ...  */
  x_catch_errors_for_lisp (dpyinfo);
  {
    let propagate: bool = !to_root;
    let mask: c_long = if to_root { 0xffffff } else { 0 };

    XSendEvent ((*dpyinfo).display, wdest, propagate as Bool, mask, &raw mut event);
    XFlush ((*dpyinfo).display);
  }
  x_check_errors_for_lisp (dpyinfo, c"Failed to send client event: %s".as_ptr ());
  x_uncatch_errors_for_lisp (dpyinfo);
  unblock_input ();
}

/* C 3350-3351: x_timestamp_for_selection's comment.  */

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_timestamp_for_selection (dpyinfo: *mut x_display_info,   // C 3349-3364
                                                    selection: Lisp_Object) -> Lisp_Object

{
  let value: Lisp_Object;
  let local_value: Lisp_Object;

  local_value = LOCAL_SELECTION! (selection, dpyinfo);

  if NILP (local_value)
    { return Qnil (); }

  value = XCAR (XCDR (XCDR (local_value)));

  return value;
}

/* C 3366: the blank before `static void syms_of_xselect_for_pdumper (void);`.  That forward
   declaration has no Rust counterpart (PORTING.md 0.1-12) and the definition stays at C 3499.  */

/* C 3406-3410: the seven `DEFVAR_*` variables.  Each is defined here with `#[unsafe(no_mangle)]`
   so the unit is its single definition point (P-21: the mirror's `globals.rs` `variables` module
   declares them under a glob import, which a definition shadows).  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vselection_converter_alist: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vx_lost_selection_functions: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vx_sent_selection_functions: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vx_select_enable_clipboard_manager: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut x_selection_timeout: EMACS_INT = 0;
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vx_treat_local_requests_remotely: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)] pub static mut Vx_selection_alias_alist: Lisp_Object = lisp_h_Qnil ();

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xselect ()   // C 3367-3497
{
  defsubr (&raw mut Sx_get_selection_internal);
  defsubr (&raw mut Sx_own_selection_internal);
  defsubr (&raw mut Sx_disown_selection_internal);
  defsubr (&raw mut Sx_selection_owner_p);
  defsubr (&raw mut Sx_selection_exists_p);

  defsubr (&raw mut Sx_get_atom_name);
  defsubr (&raw mut Sx_send_client_message);
  defsubr (&raw mut Sx_register_dnd_atom);
  defsubr (&raw mut Sx_get_local_selection);

  reading_selection_reply = Fcons (Qnil (), Qnil ());
  staticpro (&raw const reading_selection_reply);
  staticpro (&raw const property_change_reply);

  outstanding_transfers.next = &raw mut outstanding_transfers;
  outstanding_transfers.last = &raw mut outstanding_transfers;

  DEFVAR_LISP! (c"selection-converter-alist", Vselection_converter_alist,
		"An alist associating X Windows selection-types with functions.\nThese functions are called to convert the selection, with three args:\nthe name of the selection (typically `PRIMARY', `SECONDARY', or\n`CLIPBOARD'); a desired type to which the selection should be\nconverted; and the local selection value (whatever was given to\n`x-own-selection-internal').\n\nOn X Windows, the function can also be a cons of (PREDICATE\n. FUNCTION), where PREDICATE determines whether or not the selection\ntype will appear in the list of selection types available to other\nprograms, and FUNCTION is the function which is actually called.\nPREDICATE is called with the same arguments as FUNCTION, and should\nreturn a non-nil value if the data type is to appear in that list.\n\nThe function should return the value to send to the X server\n(typically a string).  A return value of nil\nmeans that the conversion could not be done.\nA return value which is the symbol `NULL'\nmeans that a side-effect was executed,\nand there is no meaningful selection value.");
  Vselection_converter_alist = Qnil ();

  DEFVAR_LISP! (c"x-lost-selection-functions", Vx_lost_selection_functions,
		"A list of functions to be called when Emacs loses an X selection.\n(This happens when some other X client makes its own selection\nor when a Lisp program explicitly clears the selection.)\nThe functions are called with one argument, the selection type\n(a symbol, typically `PRIMARY', `SECONDARY', or `CLIPBOARD').");
  Vx_lost_selection_functions = Qnil ();

  DEFVAR_LISP! (c"x-sent-selection-functions", Vx_sent_selection_functions,
		"A list of functions to be called when Emacs answers a selection request.\nThe functions are called with three arguments:\n  - the selection name (typically `PRIMARY', `SECONDARY', or `CLIPBOARD');\n  - the selection-type which Emacs was asked to convert the\n    selection into before sending (for example, `STRING' or `LENGTH');\n  - a flag indicating success or failure for responding to the request.\nWe might have failed (and declined the request) for any number of reasons,\nincluding being asked for a selection that we no longer own, or being asked\nto convert into a type that we don't know about or that is inappropriate.\nThis hook doesn't let you change the behavior of Emacs's selection replies,\nit merely informs you that they have happened.");
  Vx_sent_selection_functions = Qnil ();                        // C 3429

  DEFVAR_LISP! (c"x-select-enable-clipboard-manager",
		Vx_select_enable_clipboard_manager,
		"Whether to enable X clipboard manager support.\nIf non-nil, then whenever Emacs is killed or an Emacs frame is deleted\nwhile owning the X clipboard, the clipboard contents are saved to the\nclipboard manager if one is present.");   // C 3431-3436
  Vx_select_enable_clipboard_manager = Qt ();                   // C 3437

  DEFVAR_INT! (c"x-selection-timeout", x_selection_timeout,
	       "Number of milliseconds to wait for a selection reply.\nIf the selection owner doesn't reply in this time, we give up.\nA value of 0 means wait as long as necessary.  This is initialized from the\n\"*selectionTimeout\" resource.");   // C 3439-3443
  x_selection_timeout = 0;                                      // C 3444

  DEFVAR_LISP! (c"x-treat-local-requests-remotely", Vx_treat_local_requests_remotely,
    "Whether to treat local selection requests as remote ones.\n\nIf non-nil, selection converters for string types (`STRING',\n`UTF8_STRING', `COMPOUND_TEXT', etc) will encode the strings, even\nwhen Emacs itself is converting the selection.");   // C 3446-3452
  Vx_treat_local_requests_remotely = Qnil ();                   // C 3453

  DEFVAR_LISP! (c"x-selection-alias-alist", Vx_selection_alias_alist,
    "List of selections to alias to another.\nIt should be an alist of a selection name to another.  When a\nselection request arrives for the first selection, Emacs will respond\nas if the request was meant for the other.\n\nNote that this does not affect setting or owning selections.");   // C 3455-3461
  Vx_selection_alias_alist = Qnil ();                           // C 3462

  /* QPRIMARY is defined in keyboard.c.  */
  DEFSYM! (QSECONDARY, c"SECONDARY");                           // C 3465
  DEFSYM! (QSTRING, c"STRING");
  DEFSYM! (QINTEGER, c"INTEGER");
  DEFSYM! (QCLIPBOARD, c"CLIPBOARD");
  DEFSYM! (QTIMESTAMP, c"TIMESTAMP");
  DEFSYM! (QTEXT, c"TEXT");

  /* These are types of selection.  */
  DEFSYM! (QCOMPOUND_TEXT, c"COMPOUND_TEXT");                   // C 3472
  DEFSYM! (QUTF8_STRING, c"UTF8_STRING");

  DEFSYM! (QDELETE, c"DELETE");                                 // C 3475
  DEFSYM! (QMULTIPLE, c"MULTIPLE");
  DEFSYM! (QINCR, c"INCR");
  DEFSYM! (Q_EMACS_TMP_, c"_EMACS_TMP_");
  DEFSYM! (QTARGETS, c"TARGETS");
  DEFSYM! (QATOM, c"ATOM");
  DEFSYM! (QCLIPBOARD_MANAGER, c"CLIPBOARD_MANAGER");
  DEFSYM! (QSAVE_TARGETS, c"SAVE_TARGETS");
  DEFSYM! (QNULL, c"NULL");
  DEFSYM! (QXdndDirectSave0, c"XdndDirectSave0");
  DEFSYM! (QXdndActionDirectSave, c"XdndActionDirectSave");
  DEFSYM! (Qtext_plain, c"text/plain");
  DEFSYM! (Qforeign_selection, c"foreign-selection");
  DEFSYM! (Qx_lost_selection_functions, c"x-lost-selection-functions");
  DEFSYM! (Qx_sent_selection_functions, c"x-sent-selection-functions");

  DEFSYM! (QXmTRANSFER_SUCCESS, c"XmTRANSFER_SUCCESS");         // C 3491
  DEFSYM! (QXmTRANSFER_FAILURE, c"XmTRANSFER_FAILURE");

  pdumper_do_now_and_after_load (Some (syms_of_xselect_for_pdumper));   // C 3494
}

#[cfg (feature = "have_x_windows")]
unsafe extern "C" fn syms_of_xselect_for_pdumper ()            // C 3499-3511
{
  outstanding_transfers.next = &raw mut outstanding_transfers;   // C 3501
  outstanding_transfers.last = &raw mut outstanding_transfers;

  reading_selection_window = 0;                                 // C 3504
  reading_which_selection = 0;
  property_change_wait_list = std::ptr::null_mut ();
  prop_location_identifier = 0;
  property_change_reply = Fcons (Qnil (), Qnil ());
}

#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_xselect ()                       // C 3513-3541
{
  let mut next: *mut transfer;                                 // C 3515
  let mut frame: *mut x_selection_request;                     // C 3516
  let mut cs: *mut selection_data;                             // C 3517

  /* Mark all the strings being used as selection data.  ...  */

  next = outstanding_transfers.next;                            // C 3525

  if next.is_null ()                                            // C 3527
    /* syms_of_xselect has not yet been called.  */
    { return; }

  while !std::ptr::eq (next, &raw const outstanding_transfers)   // C 3531
    {
      mark_object ((*next).data.string);                        // C 3533
      next = (*next).next;
    }

  frame = selection_request_stack;                              // C 3537
  while !frame.is_null ()
    {
      cs = (*frame).converted_selections;
      while !cs.is_null ()
	{
	  mark_object ((*cs).string);
	  cs = (*cs).next;
	}
      frame = (*frame).last;
    }
}
