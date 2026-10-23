// femacs: port of lwlib/lwlib-Xm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the
// `static` helpers whose address an Xt/Motif callback takes are `unsafe extern "C" fn`
// (PORTING.md 1.4).
//
// **The file gate is `feature = "use_motif"`** (P-86 clause 8, the build's condition rather than
// one the C text spells): `lwlib/Makefile.in` 53 lists `lwlib-Xm.o` in `MOTIF_OBJS = lwlib-Xm.o`,
// 56 selects `$(@X_TOOLKIT_TYPE@_OBJS)` as `TOOLKIT_OBJS` and 58 puts that list into `OBJS`,
// with `configure.ac` 7233's `AC_DEFINE ([USE_MOTIF], [1], [Define to 1 if using the Motif X
// toolkit.])` as the configure condition (`configure.ac` 3147 sets `USE_X_TOOLKIT=MOTIF`, and
// 4264 tests for it).  `USE_MOTIF`, `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of
// this file and the port's frozen object is the empty one.
//
// **This file is `text-only`** (P-86 clause 3(2)): this machine has no Xt and no Motif header
// (`/opt/homebrew/include/X11` carries `xorgproto` 2025.1's Xlib half - `<X11/Xlib.h>`,
// `<X11/Xatom.h>` - and nothing of `<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`,
// `<X11/CoreP.h>`, `<X11/CompositeP.h>`, `<X11/StringDefs.h>` or the twenty-four `<Xm/*.h>` of
// C 38-61).  The Xlib records this file addresses element by element (`XButtonPressedEvent`'s
// fifteen members, `XKeyEvent`'s `display`/`keycode`, `XEvent`) and the eleven Xlib scalars are
// **measured with clang** against that header (`## Evidence` 6 of docs/ports/lwlib-Xm.md);
// every Xt and Motif width, offset, enum value and resource name below is C's spelling with the
// value **UNVERIFIED**.
//
// The sub-conditions the C text spells keep their own gates (P-86 clause 1 and 6): C 500's
// `#ifndef LESSTIF_VERSION` is `feature = "not_lesstif_version"`, C 709's `#ifdef XmNpositionIndex`
// is `feature = "xm_position_index"`, C 1546's `#ifdef ENERGIZE` is `feature = "energize"`, and
// C 1728's `#if XmVersion < 1002 || (defined LESSTIF_VERSION && LESSTIF_VERSION < 84)` is
// `feature = "xm_version_lt_1002_or_lesstif_lt_84"`.  None of the four is registered in
// `xtask/Cargo.toml` today, so no configuration sets them and the ledger names the owner
// (`## Owed` in docs/ports/lwlib-Xm.md).  C 123-128 and C 131-153 are `#if 0`: dead in every
// configuration, so the port records the region instead of carrying it (the `src/character.rs`
// 91-97 precedent).
//
// The `<X11/*>` and `<Xm/*>` surface stands in this unit (P-88 clause 1: no mirror in this tree
// carries any of those headers and this file is their only consumer here);
// `lwlib/include/lwlib.rs` names `Widget`, `Boolean` and `XEvent` and leaves them to the unit
// that includes it.
//
// C's static forward declarations (C 77-121) are **deleted**: Rust items are order-independent
// (PORTING.md 0.1-12 / checklist 10).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, unused_mut, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, clippy::all)]

include!("../src/include/config.rs");            // C 21: `#include <config.h>`
/* C 23's `#include <unistd.h>` has no edge of its own: nothing in this file names a declaration
   that header carries.  */
#[cfg (feature = "use_motif")]
include!("../lib/include/stdio.rs");             // C 24: `#include <stdio.h>` (`sprintf`)
/* C 25's `#include <setjmp.h>` has no edge of its own either; the mirror that carries what it
   declares is `lisp.rs`, which C 33 includes next.  */
#[cfg (feature = "use_motif")]
include!("../src/include/lisp.rs");              // C 33: `#include <lisp.h>`
                                                 // (`xmalloc`/`xstrdup`/`xfree`/`emacs_abort`)
#[cfg (feature = "use_motif")]
include!("../lib/include/string.rs");            // lisp.h 30: `#include <string.h>` (`strcmp`)
#[cfg (feature = "use_motif")]
include!("include/lwlib-Xm.rs");                 // C 35: `#include "lwlib-Xm.h"`
#[cfg (feature = "use_motif")]
include!("include/lwlib-utils.rs");              // C 36: `#include "lwlib-utils.h"`
                                                 // (`XtCompositeChildren`)
/* C 27-31's `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/CoreP.h>`,
   `<X11/CompositeP.h>` and C 38-61's twenty-four `<Xm/…>` headers are the surface declared
   below: no header in this tree carries any of them and this file is their only consumer here
   (P-88 clause 1).  C's own include list, kept as C writes it:

     C 38  <Xm/BulletinB.h>       C 45  <Xm/MainW.h>        C 52  <Xm/SelectioB.h>
     C 39  <Xm/CascadeB.h>        C 46  <Xm/MenuShell.h>    C 53  <Xm/Text.h>
     C 40  <Xm/CascadeBG.h>       C 47  <Xm/MessageB.h>     C 54  <Xm/TextF.h>
     C 41  <Xm/DrawingA.h>        C 48  <Xm/PanedW.h>       C 55  <Xm/ToggleB.h>
     C 42  <Xm/FileSB.h>          C 49  <Xm/PushB.h>        C 56  <Xm/ToggleBG.h>
     C 43  <Xm/Label.h>           C 50  <Xm/PushBG.h>       C 57  <Xm/RowColumn.h>
     C 44  <Xm/List.h>            C 51  <Xm/ArrowB.h>       C 58  <Xm/ScrolledW.h>
                                                             C 59  <Xm/Separator.h>
                                                             C 60  <Xm/DialogS.h>
                                                             C 61  <Xm/Form.h>            */

/* ------------------------------------------------------------------------------------------
   The Xt half (`<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/CoreP.h>`,
   `<X11/CompositeP.h>`, `<X11/StringDefs.h>`) and the Xlib half (`<X11/Xlib.h>`,
   `<X11/X.h>`).  **C's spellings, not measurements** except the records marked measured: this
   machine has no Xt header, so `WidgetRec`/`CorePart`/`CompositePart` carry the members this
   file reads in C's own relative order with the members between them omitted - their offsets
   are knowingly not the real ones - and every resource name and Xt/Xlib enum below is a
   `UNVERIFIED` value (`## Owed` 5 of docs/ports/lwlib-Xm.md names the toolchain).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "use_motif")]
pub type XtPointer = *mut c_void;                     // C: `typedef char *XtPointer;`
#[cfg (feature = "use_motif")]
pub type String = *mut c_char;                        // C: `typedef char *String;`
#[cfg (feature = "use_motif")]
pub type Boolean = c_int;                             // C: `typedef int Boolean;`
#[cfg (feature = "use_motif")]
pub const False: Boolean = 0;                         // C: `#define False 0`
#[cfg (feature = "use_motif")]
pub const True: Boolean = 1;                          // C: `#define True 1`
/// C: `<stddef.h>`: `#define NULL ((void *) 0)` - the null pointer C 156, 241, 273, 663, 728,
/// 1001, 1522, 1655 and 1812 write; the token is kept (the `lwlib-Xaw.rs` 92-95 spelling).
#[cfg (feature = "use_motif")]
pub const NULL: *mut c_void = core::ptr::null_mut ();
#[cfg (feature = "use_motif")]
pub type Cardinal = c_uint;                           // C: `typedef unsigned int Cardinal;`
#[cfg (feature = "use_motif")]
pub type Dimension = c_ushort;                        // C: `typedef unsigned short Dimension;`
#[cfg (feature = "use_motif")]
pub type Position = c_short;                          // C: `typedef short Position;`
/// C: X.h 66-88: `typedef unsigned long XID;` and `typedef XID Window;`.
#[cfg (feature = "use_motif")]
pub type XID = c_ulong;
#[cfg (feature = "use_motif")]
pub type Window = XID;
/// C: Xlib.h: `typedef struct _XDisplay Display;`
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Display { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XScreen Screen;` (`WidthOfScreen`/`HeightOfScreen`'s argument).
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Screen { pub _private: [u8; 0] }
/// C: X.h: `typedef unsigned long KeySym;` (measured: 8 bytes on this target).
#[cfg (feature = "use_motif")]
pub type KeySym = c_ulong;
/// C: X.h: `typedef unsigned char KeyCode;`
#[cfg (feature = "use_motif")]
pub type KeyCode = c_uchar;
/// C: Intrinsic.h: `typedef unsigned int Modifiers;` (Xt's, not read here: C 1014's
/// `modif_ret` is only filled by `XtTranslateKeycode`).
#[cfg (feature = "use_motif")]
pub type Modifiers = c_uint;
/// C: X.h: `typedef unsigned long Time;` (measured: 8 bytes on this target).
#[cfg (feature = "use_motif")]
pub type Time = c_ulong;

/// C: Xlib.h: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Window root; Window subwindow; Time time; int x, y; int x_root,
/// y_root; unsigned int state; unsigned int button; Bool same_screen; } XButtonEvent;` -
/// **measured with clang** against this machine's xorgproto 2025.1: `sizeof` 96, `type` 0,
/// `serial` 8, `send_event` 16, `display` 24, `window` 32, `root` 40, `subwindow` 48, `time` 56,
/// `x` 64, `y` 68, `x_root` 72, `y_root` 76, `state` 80, `button` 84, `same_screen` 88.  C 1705
/// declares its `dummy` as `XButtonPressedEvent`, which that header defines as this record.
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XButtonEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
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
  pub same_screen: c_int,
}
/// C: X.h: `#define XButtonPressedEvent XButtonEvent` - the name C 1705 uses.
#[cfg (feature = "use_motif")]
pub type XButtonPressedEvent = XButtonEvent;

/// C: Xlib.h: `typedef struct { int type; unsigned long serial; Bool send_event; Display
/// *display; Window window; Window root; Window subwindow; Time time; unsigned int keycode;
/// unsigned int state; int x, y; int x_root, y_root; Bool same_screen; } XKeyEvent;` -
/// **measured with clang** (the same probe): `sizeof` 96, `display` 24, `keycode` 84.  C 1016
/// reads the first two of those through `event->xkey`.
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XKeyEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: c_int,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub keycode: c_uint,
  pub state: c_uint,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub same_screen: c_int,
}
/// C: Xlib.h: `typedef union _XEvent { int type; XAnyEvent xany; XKeyEvent xkey; XButtonEvent
/// xbutton; … } XEvent;` - the two members this file addresses (`type` at C 1722, `xkey` at
/// C 1016, `xbutton` at C 1735-1739) plus the **measured** size/alignment of this target's
/// `XEvent` (192 bytes, align 8) as the `[c_ulong; 24]` element; the members C does not name
/// are **UNVERIFIED** (the `lwlib-Xaw.rs` 240-254 union shape).
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent
{
  pub r#type: c_int,
  pub xkey: XKeyEvent,
  pub xbutton: XButtonEvent,
  pub _padding: [c_ulong; 24],
}
/// C: Intrinsic.h: `typedef struct _WidgetRec *Widget;` - the record carries `core` and
/// `composite` (its `CompositePart`), the two members this file reaches through the widget
/// pointer; the members between them are omitted, so **both offsets are UNVERIFIED**.
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec
{
  pub core: CorePart,
  pub composite: CompositePart,
}
/// C: CoreP.h 92-96: `typedef struct { Widget parent; WidgetClass class; String name; Cardinal
/// ref_count; ArgList …; Boolean being_destroyed; …; Position x, y; Dimension width, height;
/// …; Window window; … } CorePart;` - C 787, C 1517, C 1521, C 1803 and C 1355-1374 read the
/// seven members below; C's order is `parent`, `being_destroyed`, `x`/`y`, `width`/`height`,
/// `window`, with everything between them omitted (**UNVERIFIED**
/// offsets).
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CorePart
{
  pub parent: Widget,                                 // C: `Widget parent;`
  pub being_destroyed: Boolean,                       // C: `Boolean being_destroyed;`
  pub x: Position,                                    // C: `Position x, y;`
  pub y: Position,
  pub width: Dimension,                               // C: `Dimension width, height;`
  pub height: Dimension,
  pub window: Window,                                 // C: `Window window;`
}
/// C: CompositeP.h 92-96: `typedef struct { Cardinal num_children; WidgetList children; … }
/// CompositePart;` - `first_child` (C 188) and C 949/C 952 read the two members in C's own
/// order and with no member omitted, so their two offsets are right; the record's offset inside
/// `WidgetRec` above is not (**UNVERIFIED**).
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CompositePart
{
  pub num_children: Cardinal,                          // C: `Cardinal num_children;`
  pub children: *mut Widget,                            // C: `WidgetList children;`
}
#[cfg (feature = "use_motif")]
pub type Widget = *mut WidgetRec;
/// C: Intrinsic.h: `typedef struct _WidgetRec *CompositeWidget;` - the same pointer type.
#[cfg (feature = "use_motif")]
pub type CompositeWidget = *mut WidgetRec;
#[cfg (feature = "use_motif")]
pub type WidgetClass = *mut c_void;                   // C: `typedef struct _WidgetClassRec *WidgetClass;`
/// C: Intrinsic.h: `typedef char *XtTranslations;` (`XtParseTranslationTable`'s result, C 491).
#[cfg (feature = "use_motif")]
pub type XtTranslations = *mut c_char;
/// C: Intrinsic.h: `typedef void (*XtCallbackProc) (Widget, XtPointer, XtPointer);`
#[cfg (feature = "use_motif")]
pub type XtCallbackProc = Option<unsafe extern "C" fn (Widget, XtPointer, XtPointer)>;
/// C: Intrinsic.h: `typedef void (*XtEventHandler) (Widget, XtPointer, XEvent *, Boolean *);`
/// - the type of C 1007-1030's `dialog_key_cb` and of the handler C 1118 gives
/// `XtAddEventHandler`.
#[cfg (feature = "use_motif")]
pub type XtEventHandler =
  Option<unsafe extern "C" fn (Widget, XtPointer, *mut XEvent, *mut Boolean)>;
/// C: Intrinsic.h: `typedef struct { String resource_name; String resource_class; String
/// resource_type; Cardinal resource_size; Cardinal resource_offset; String default_type;
/// XtPointer default_addr; } XtResource;` - the seven columns C 206-212 initializes by name.
#[cfg (feature = "use_motif")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XtResource
{
  pub resource_name: String,
  pub resource_class: String,
  pub resource_type: String,
  pub resource_size: Cardinal,
  pub resource_offset: Cardinal,
  pub default_type: String,
  pub default_addr: XtPointer,
}
/// C: StringDefs.h: the resource names this file passes to `XtSetArg`/`XtGetValues`/
/// `XtVaSetValues`/`XtVaGetValues`/`XtAddCallback`/`XtRemoveAllCallbacks`/`XtRemoveCallback`,
/// plus the resource *type* C 211 and the two class strings C 207-208 of `resource_motif_string`
/// name.  C's spelling is kept and the values are **UNVERIFIED** (they are the names the X
/// resource-manager convention gives those resources, not a reading of that header).
#[cfg (feature = "use_motif")]
pub const XtNwidth: &core::ffi::CStr = c"width";
#[cfg (feature = "use_motif")]
pub const XtNheight: &core::ffi::CStr = c"height";
#[cfg (feature = "use_motif")]
pub const XtNborderWidth: &core::ffi::CStr = c"borderWidth";
#[cfg (feature = "use_motif")]
pub const XtNx: &core::ffi::CStr = c"x";
#[cfg (feature = "use_motif")]
pub const XtNy: &core::ffi::CStr = c"y";
#[cfg (feature = "use_motif")]
pub const XtNdestroyCallback: &core::ffi::CStr = c"destroyCallback";
#[cfg (feature = "use_motif")]
pub const XtNallowShellResize: &core::ffi::CStr = c"allowShellResize";
/// C: StringDefs.h: `#define XtRImmediate "Immediate"` - the resource type C 211 gives
/// `default_type`.
#[cfg (feature = "use_motif")]
pub const XtRImmediate: &core::ffi::CStr = c"Immediate";

/// C: Intrinsic.h: `typedef struct { String name; XtArgVal value; } Arg, *ArgList;` - C's macro
/// `XtSetArg (arg, n, d)` is `((arg).name = (n), (arg).value = (XtArgVal) (d))`, so the port
/// carries `XtArgVal` as `XtPointer` and spells the macro with the same two assignments (the
/// `lwlib-Xaw.rs` 348-365 shape).
#[cfg (feature = "use_motif")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct Arg
{
  pub name: String,
  pub value: XtPointer,
}
#[cfg (feature = "use_motif")]
macro_rules! XtSetArg {
  ($arg:expr, $name:expr, $value:expr) => {
    { ($arg).name = ($name) as String; ($arg).value = ($value) as XtPointer; }
  };
}

#[cfg (feature = "use_motif")]
pub mod lwlib_xm_x11_constants {
  use core::ffi::{c_int, c_uint, c_ulong};

  /// C: X.h: `#define ButtonPress 4` (measured on this machine's xorgproto 2025.1).
  pub const ButtonPress: c_int = 4;
  /// C: X.h: `#define ButtonRelease 5` (measured).
  pub const ButtonRelease: c_int = 5;
  /// C: X.h: `#define MotionNotify 6` (measured).
  pub const MotionNotify: c_int = 6;
  /// C: X.h: `#define EnterNotify 7` (measured).
  pub const EnterNotify: c_int = 7;
  /// C: X.h: `#define KeyPressMask (1L<<0)` (measured).
  pub const KeyPressMask: c_ulong = 1;
  /// C: X.h: `#define Button1Mask (1<<8)` … `#define Button5Mask (1<<12)` (measured:
  /// 256/512/1024/2048/4096).
  pub const Button1Mask: c_uint = 256;
  pub const Button2Mask: c_uint = 512;
  pub const Button3Mask: c_uint = 1024;
  pub const Button4Mask: c_uint = 2048;
  pub const Button5Mask: c_uint = 4096;
}

/// C: X.h: the eleven Xlib scalars the module above holds (measured on this machine's
/// xorgproto 2025.1) - imported whole so the call sites write C's own names.
#[cfg (feature = "use_motif")]
use lwlib_xm_x11_constants::*;

#[cfg (feature = "use_motif")]
unsafe extern "C" {
  /* C: Intrinsic.h: `Boolean XtIsSubclass (Widget, WidgetClass);`
     C: Intrinsic.h: `void XtSetValues (Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `void XtVaSetValues (Widget, ...);`
     C: Intrinsic.h: `void XtVaGetValues (Widget, ...);`
     C: Intrinsic.h: `void XtGetValues (Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `void XtSetSensitive (Widget, Boolean);`
     C: Intrinsic.h: `void XtAddCallback (Widget, const char *, XtCallbackProc, XtPointer);`
     C: Intrinsic.h: `void XtRemoveAllCallbacks (Widget, const char *);`
     C: Intrinsic.h: `void XtRemoveCallback (Widget, const char *, XtCallbackProc, XtPointer);`
     C: Intrinsic.h: `void XtCallCallbacks (Widget, const char *, XtPointer);`
     C: Intrinsic.h: `void XtAddEventHandler (Widget, EventMask, Boolean, XtEventHandler,
     XtPointer);`
     C: Intrinsic.h: `void XtOverrideTranslations (Widget, XtTranslations);`
     C: Intrinsic.h: `XtTranslations XtParseTranslationTable (const char *);`
     C: Intrinsic.h: `void XtDestroyWidget (Widget);`
     C: Intrinsic.h: `void XtManageChild (Widget);`
     C: Intrinsic.h: `void XtManageChildren (WidgetList, Cardinal);`
     C: Intrinsic.h: `void XtUnmanageChild (Widget);`
     C: Intrinsic.h: `void XtUnmanageChildren (WidgetList, Cardinal);`
     C: Intrinsic.h: `Widget XtParent (Widget);`
     C: Intrinsic.h: `String XtName (Widget);`
     C: Intrinsic.h: `Widget XtNameToWidget (Widget, const char *);`
     C: Intrinsic.h: `WidgetClass XtClass (Widget);`
     C: Intrinsic.h: `Display *XtDisplay (Widget);`
     C: Intrinsic.h: `Screen *XtScreen (Widget);`
     C: Intrinsic.h: `Window XtWindow (Widget);`
     C: Intrinsic.h: `char *XtMalloc (Cardinal);`
     C: Intrinsic.h: `void XtFree (char *);`
     C: Intrinsic.h: `void XtSetKeyboardFocus (Widget, Widget);`
     C: Intrinsic.h: `void XtInstallAccelerators (Widget, Widget);`
     C: Intrinsic.h: `void XtTranslateCoords (Widget, Position, Position, Position *, Position *);`
     C: Intrinsic.h: `void XtPopdown (Widget);`
     C: Intrinsic.h: `void XtRealizeWidget (Widget);`
     C: Intrinsic.h: `void XtGetSubresources (Widget, XtPointer, const char *, const char *,
     XtResourceList, Cardinal, ...);`
     C: Intrinsic.h: `Boolean XtTranslateKeycode (Display *, KeyCode, Modifiers, Modifiers *,
     KeySym *);`  */
  pub fn XtIsSubclass (widget: Widget, widget_class: WidgetClass) -> Boolean;
  pub fn XtSetValues (widget: Widget, args: *mut Arg, num_args: Cardinal);
  pub fn XtVaSetValues (widget: Widget, ...);
  pub fn XtVaGetValues (widget: Widget, ...);
  pub fn XtGetValues (widget: Widget, args: *mut Arg, num_args: Cardinal);
  pub fn XtSetSensitive (widget: Widget, sensitive: Boolean);
  pub fn XtAddCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                        client_data: XtPointer);
  pub fn XtRemoveAllCallbacks (widget: Widget, name: *const c_char);
  pub fn XtRemoveCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                           client_data: XtPointer);
  pub fn XtCallCallbacks (widget: Widget, name: *const c_char, call_data: XtPointer);
  pub fn XtAddEventHandler (widget: Widget, event_mask: c_ulong, nonmaskable: Boolean,
                            handler: XtEventHandler, client_data: XtPointer);
  pub fn XtOverrideTranslations (widget: Widget, translations: XtTranslations);
  pub fn XtParseTranslationTable (table: *const c_char) -> XtTranslations;
  pub fn XtDestroyWidget (widget: Widget);
  pub fn XtManageChild (widget: Widget);
  pub fn XtManageChildren (children: *mut Widget, num_children: Cardinal);
  pub fn XtUnmanageChild (widget: Widget);
  pub fn XtUnmanageChildren (children: *mut Widget, num_children: Cardinal);
  pub fn XtParent (widget: Widget) -> Widget;
  pub fn XtName (widget: Widget) -> String;
  pub fn XtNameToWidget (reference: Widget, names: *const c_char) -> Widget;
  pub fn XtClass (widget: Widget) -> WidgetClass;
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  pub fn XtScreen (widget: Widget) -> *mut Screen;
  pub fn XtWindow (widget: Widget) -> Window;
  pub fn XtMalloc (size: Cardinal) -> *mut c_char;
  pub fn XtFree (ptr: *mut c_char);
  pub fn XtSetKeyboardFocus (widget: Widget, descendant: Widget);
  pub fn XtInstallAccelerators (destination: Widget, source: Widget);
  pub fn XtTranslateCoords (widget: Widget, x: Position, y: Position, root_x: *mut Position,
                            root_y: *mut Position);
  pub fn XtPopdown (shell: Widget);
  pub fn XtRealizeWidget (widget: Widget);
  pub fn XtGetSubresources (widget: Widget, base: XtPointer, name: *const c_char,
                            class: *const c_char, resources: *mut XtResource,
                            num_resources: Cardinal, ...);
  pub fn XtTranslateKeycode (display: *mut Display, keycode: KeyCode, modifiers: Modifiers,
                             modifiers_return: *mut Modifiers, keysym_return: *mut KeySym)
    -> Boolean;
}

#[cfg (feature = "use_motif")]
unsafe extern "C" {
  /* C: Intrinsic.h: `void XtIsShell (Widget);` is a macro (`XtIsSubclass (w, shellWidgetClass)`)
     and the port declares the predicate C 1023-1026 calls, the `lwlib-Xlw.rs` reading of the
     same name.  `XtIsManaged` likewise (`XtIsSubclass (w, compositeWidgetClass)`).
     C: Xlib.h: `Bool XQueryPointer (Display *, Window, Window *, Window *, int *, int *, int *,
     int *, unsigned int *);`
     C: Xlib.h: `int XFlush (Display *);`
     C: Xlib.h: `#define WidthOfScreen(s) (…)` / `#define HeightOfScreen(s) (…)` - the two
     macros C 1346-1347 reads.  */
  pub fn XtIsShell (widget: Widget) -> Boolean;
  pub fn XtIsManaged (widget: Widget) -> Boolean;
  pub fn XQueryPointer (display: *mut Display, w: Window, root_return: *mut Window,
                        child_return: *mut Window, root_x_return: *mut c_int,
                        root_y_return: *mut c_int, win_x_return: *mut c_int,
                        win_y_return: *mut c_int, mask_return: *mut c_uint) -> c_int;
  pub fn XFlush (display: *mut Display) -> c_int;
  pub fn WidthOfScreen (screen: *mut Screen) -> c_int;
  pub fn HeightOfScreen (screen: *mut Screen) -> c_int;
}

/* ------------------------------------------------------------------------------------------
   The Motif half (the twenty-four `<Xm/…>` headers of C 38-61).  **C's spellings, not
   measurements**: there is no Motif header on this machine, so every record, resource name,
   enum value and entry point below is `UNVERIFIED` (`## Owed` 5 of docs/ports/lwlib-Xm.md).
   ------------------------------------------------------------------------------------------ */
/// C: Xm/Xm.h: `typedef struct _XmStringRec *XmString;` - opaque, and C 204/321-323/342/351 only
/// ever holds one and passes it on.
#[cfg (feature = "use_motif")]
pub type XmString = *mut c_void;
/// C: Xm/PushB.h: `typedef struct { int reason; XEvent *event; Widget widget; … }
/// XmPushButtonCallbackStruct;` - C 272 reads `reason` and `event`; the members after them are
/// omitted (**UNVERIFIED** offsets).
#[cfg (feature = "use_motif")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XmPushButtonCallbackStruct
{
  pub reason: c_int,
  pub event: *mut XEvent,
}

/// C: Xm/Xm.h (and the per-class headers): the resource names this file passes to `XtSetArg`/
/// `XtVaSetValues`/`XtVaGetValues`/`XtAddCallback`/`XtRemoveAllCallbacks`.  C's spelling is
/// kept and the values are **UNVERIFIED** - they are the names Motif registers, not a reading
/// of those headers.
#[cfg (feature = "use_motif")]
pub const XmNacceleratorText: &core::ffi::CStr = c"acceleratorText";
#[cfg (feature = "use_motif")]
pub const XmNactivateCallback: &core::ffi::CStr = c"activateCallback";
#[cfg (feature = "use_motif")]
pub const XmNadjustLast: &core::ffi::CStr = c"adjustLast";
#[cfg (feature = "use_motif")]
pub const XmNalignment: &core::ffi::CStr = c"alignment";
#[cfg (feature = "use_motif")]
pub const XmNarmCallback: &core::ffi::CStr = c"armCallback";
#[cfg (feature = "use_motif")]
pub const XmNautoUnmanage: &core::ffi::CStr = c"autoUnmanage";
#[cfg (feature = "use_motif")]
pub const XmNbottomAttachment: &core::ffi::CStr = c"bottomAttachment";
#[cfg (feature = "use_motif")]
pub const XmNbottomOffset: &core::ffi::CStr = c"bottomOffset";
#[cfg (feature = "use_motif")]
pub const XmNbottomWidget: &core::ffi::CStr = c"bottomWidget";
#[cfg (feature = "use_motif")]
pub const XmNcascadingCallback: &core::ffi::CStr = c"cascadingCallback";
#[cfg (feature = "use_motif")]
pub const XmNcolumns: &core::ffi::CStr = c"columns";
#[cfg (feature = "use_motif")]
pub const XmNdefaultActionCallback: &core::ffi::CStr = c"defaultActionCallback";
#[cfg (feature = "use_motif")]
pub const XmNdefaultButton: &core::ffi::CStr = c"defaultButton";
#[cfg (feature = "use_motif")]
pub const XmNdeleteResponse: &core::ffi::CStr = c"deleteResponse";
#[cfg (feature = "use_motif")]
pub const XmNdisarmCallback: &core::ffi::CStr = c"disarmCallback";
#[cfg (feature = "use_motif")]
pub const XmNentryCallback: &core::ffi::CStr = c"entryCallback";
#[cfg (feature = "use_motif")]
pub const XmNheight: &core::ffi::CStr = c"height";
#[cfg (feature = "use_motif")]
pub const XmNhighlightThickness: &core::ffi::CStr = c"highlightThickness";
#[cfg (feature = "use_motif")]
pub const XmNindicatorType: &core::ffi::CStr = c"indicatorType";
#[cfg (feature = "use_motif")]
pub const XmNisAligned: &core::ffi::CStr = c"isAligned";
#[cfg (feature = "use_motif")]
pub const XmNlabelString: &core::ffi::CStr = c"labelString";
#[cfg (feature = "use_motif")]
pub const XmNlabelType: &core::ffi::CStr = c"labelType";
#[cfg (feature = "use_motif")]
pub const XmNleftAttachment: &core::ffi::CStr = c"leftAttachment";
#[cfg (feature = "use_motif")]
pub const XmNleftOffset: &core::ffi::CStr = c"leftOffset";
#[cfg (feature = "use_motif")]
pub const XmNleftWidget: &core::ffi::CStr = c"leftWidget";
#[cfg (feature = "use_motif")]
pub const XmNmappedWhenManaged: &core::ffi::CStr = c"mappedWhenManaged";
#[cfg (feature = "use_motif")]
pub const XmNmappingDelay: &core::ffi::CStr = c"mappingDelay";
#[cfg (feature = "use_motif")]
pub const XmNmarginHeight: &core::ffi::CStr = c"marginHeight";
#[cfg (feature = "use_motif")]
pub const XmNmarginWidth: &core::ffi::CStr = c"marginWidth";
#[cfg (feature = "use_motif")]
pub const XmNmenuAccelerator: &core::ffi::CStr = c"menuAccelerator";
#[cfg (feature = "use_motif")]
pub const XmNmenuHelpWidget: &core::ffi::CStr = c"menuHelpWidget";
#[cfg (feature = "use_motif")]
pub const XmNmenuPost: &core::ffi::CStr = c"menuPost";
#[cfg (feature = "use_motif")]
pub const XmNminHeight: &core::ffi::CStr = c"minHeight";
#[cfg (feature = "use_motif")]
pub const XmNminWidth: &core::ffi::CStr = c"minWidth";
#[cfg (feature = "use_motif")]
pub const XmNnavigationType: &core::ffi::CStr = c"navigationType";
#[cfg (feature = "use_motif")]
pub const XmNnumColumns: &core::ffi::CStr = c"numColumns";
#[cfg (feature = "use_motif")]
pub const XmNorientation: &core::ffi::CStr = c"orientation";
#[cfg (feature = "use_motif")]
pub const XmNpacking: &core::ffi::CStr = c"packing";
#[cfg (feature = "use_motif")]
pub const XmNpopdownCallback: &core::ffi::CStr = c"popdownCallback";
#[cfg (feature = "use_motif")]
pub const XmNpopupCallback: &core::ffi::CStr = c"popupCallback";
#[cfg (feature = "use_motif")]
pub const XmNradioBehavior: &core::ffi::CStr = c"radioBehavior";
#[cfg (feature = "use_motif")]
pub const XmNrightAttachment: &core::ffi::CStr = c"rightAttachment";
#[cfg (feature = "use_motif")]
pub const XmNrightOffset: &core::ffi::CStr = c"rightOffset";
#[cfg (feature = "use_motif")]
pub const XmNrowColumnType: &core::ffi::CStr = c"rowColumnType";
#[cfg (feature = "use_motif")]
pub const XmNsensitive: &core::ffi::CStr = c"sensitive";
#[cfg (feature = "use_motif")]
pub const XmNseparatorType: &core::ffi::CStr = c"separatorType";
#[cfg (feature = "use_motif")]
pub const XmNset: &core::ffi::CStr = c"set";
#[cfg (feature = "use_motif")]
pub const XmNshowAsDefault: &core::ffi::CStr = c"showAsDefault";
#[cfg (feature = "use_motif")]
pub const XmNsingleSelectionCallback: &core::ffi::CStr = c"singleSelectionCallback";
#[cfg (feature = "use_motif")]
pub const XmNspacing: &core::ffi::CStr = c"spacing";
#[cfg (feature = "use_motif")]
pub const XmNsubMenuId: &core::ffi::CStr = c"subMenuId";
#[cfg (feature = "use_motif")]
pub const XmNtitle: &core::ffi::CStr = c"title";
#[cfg (feature = "use_motif")]
pub const XmNtopAttachment: &core::ffi::CStr = c"topAttachment";
#[cfg (feature = "use_motif")]
pub const XmNtopOffset: &core::ffi::CStr = c"topOffset";
#[cfg (feature = "use_motif")]
pub const XmNtopWidget: &core::ffi::CStr = c"topWidget";
#[cfg (feature = "use_motif")]
pub const XmNuserData: &core::ffi::CStr = c"userData";
#[cfg (feature = "use_motif")]
pub const XmNvalueChangedCallback: &core::ffi::CStr = c"valueChangedCallback";
#[cfg (feature = "use_motif")]
pub const XmNvisibleItemCount: &core::ffi::CStr = c"visibleItemCount";
#[cfg (feature = "use_motif")]
pub const XmNvisibleWhenOff: &core::ffi::CStr = c"visibleWhenOff";
#[cfg (feature = "use_motif")]
pub const XmNwidth: &core::ffi::CStr = c"width";
/// C: Xm/RowColumn.h: `#define XmNpositionIndex "positionIndex"` - the resource name C 711 uses
/// inside its own `#ifdef XmNpositionIndex`; declared under the same gate's lowercased feature
/// (P-86 clause 1) so the gated statement is the only reader.  **UNVERIFIED.**
#[cfg (feature = "xm_position_index")]
pub const XmNpositionIndex: &core::ffi::CStr = c"positionIndex";
/// C: Xm/Xm.h and Xt's `StringDefs.h`: `#define TRUE 1` / `#define FALSE 0` - the two spellings
/// C 1029-1177 passes as `Boolean` arguments (Xt's own `True`/`False` are carried separately).
/// **UNVERIFIED**: neither header is on this machine.
#[cfg (feature = "use_motif")]
pub const TRUE: Boolean = 1;
#[cfg (feature = "use_motif")]
pub const FALSE: Boolean = 0;
/// C: Xm/Xm.h: `#define XmNallowShellResize "allowShellResize"` - the spelling C 1570 passes
/// inside the `#ifdef ENERGIZE` arm's `make_one` (the same resource Xt calls
/// `XtNallowShellResize`).  **UNVERIFIED.**
#[cfg (feature = "use_motif")]
pub const XmNallowShellResize: &core::ffi::CStr = c"allowShellResize";
/// C: Xm/Xm.h: `#define XmCXmString "XmString"` - the resource *class* C 207 assigns.
#[cfg (feature = "use_motif")]
pub const XmCXmString: &core::ffi::CStr = c"XmString";
/// C: Xm/Xm.h: `#define XmRXmString "XmString"` - the resource *type* C 208 assigns.
#[cfg (feature = "use_motif")]
pub const XmRXmString: &core::ffi::CStr = c"XmString";

/// C: Xm/Xm.h and the per-class headers: the enumeration values this file passes as arguments.
/// Each is a **placeholder, not Motif's value** (no Motif header here to read them from), and
/// the constant keeps C's name and its declaring enumeration's width.
#[cfg (feature = "use_motif")]
pub const XmALIGNMENT_BEGINNING: c_int = 0;
#[cfg (feature = "use_motif")]
pub const XmALIGNMENT_CENTER: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmATTACH_NONE: c_int = 0;
#[cfg (feature = "use_motif")]
pub const XmATTACH_FORM: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmATTACH_WIDGET: c_int = 2;
#[cfg (feature = "use_motif")]
pub const XmHORIZONTAL: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmVERTICAL: c_int = 0;
#[cfg (feature = "use_motif")]
pub const XmPACK_COLUMN: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmPACK_TIGHT: c_int = 0;
/// C: Xm/RowColumn.h: `#define XmMENU_BAR ((unsigned char) 1)` - the three `XmRowColumnType`
/// values C 510/512/1898-1900 compare with an `unsigned char`; the port declares them at C's
/// own width for that reason.  **Placeholders.**
#[cfg (feature = "use_motif")]
pub const XmMENU_BAR: c_uchar = 1;
#[cfg (feature = "use_motif")]
pub const XmMENU_PULLDOWN: c_uchar = 2;
#[cfg (feature = "use_motif")]
pub const XmMENU_POPUP: c_uchar = 3;
#[cfg (feature = "use_motif")]
pub const XmTAB_GROUP: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmUNMAP: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmN_OF_MANY: c_int = 1;
#[cfg (feature = "use_motif")]
pub const XmONE_OF_MANY: c_int = 0;
/// C: Xm/Label.h: `#define XmSTRING ((unsigned char) 0)` - C 346 passes it as `XmNlabelType`;
/// C's own type is `unsigned char` and the port declares the value as `c_int`, the width
/// `XtSetArg` casts to `XtArgVal` (PORTING.md 1.1).  **Placeholder.**
#[cfg (feature = "use_motif")]
pub const XmSTRING: c_int = 0;
/// C: Xm/Label.h: `#define XmPIXMAP ((unsigned char) 1)` - C 1167 passes it as `XmNlabelType`;
/// the same `c_int` reading.  **Placeholder.**
#[cfg (feature = "use_motif")]
pub const XmPIXMAP: c_int = 1;
/// C: Xm/Xm.h: `#define XmCR_DISARM 2` - C 286's `cbs->reason` test; **placeholder**.
#[cfg (feature = "use_motif")]
pub const XmCR_DISARM: c_int = 2;
/// C: Xm/Xm.h: `#define osfXK_Cancel 0x1006FF0D` - C 1019's `sym` test; **placeholder** (the
/// value is the OSF keysym the Motif headers define).
#[cfg (feature = "use_motif")]
pub const osfXK_Cancel: KeySym = 0x1006FF0D;

/* C: Xm/Xm.h and the per-class headers: the widget classes this file compares `XtClass`'s
   result with.  A Rustdoc comment is not emitted for an `extern` static, so the citation is a
   plain comment (the `lwlib-Xaw.rs` 332-346 reading of the same shape).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" {
  pub static mut xmDialogShellWidgetClass: WidgetClass;    // C: Xm/DialogS.h
  pub static mut xmLabelWidgetClass: WidgetClass;          // C: Xm/Label.h
  pub static mut xmPushButtonWidgetClass: WidgetClass;     // C: Xm/PushB.h
  pub static mut xmArrowButtonWidgetClass: WidgetClass;    // C: Xm/ArrowB.h
  pub static mut xmCascadeButtonWidgetClass: WidgetClass;  // C: Xm/CascadeB.h
  pub static mut xmToggleButtonWidgetClass: WidgetClass;   // C: Xm/ToggleB.h
  pub static mut xmToggleButtonGadgetClass: WidgetClass;   // C: Xm/ToggleBG.h
  pub static mut xmRowColumnWidgetClass: WidgetClass;      // C: Xm/RowColumn.h
  pub static mut xmTextWidgetClass: WidgetClass;           // C: Xm/Text.h
  pub static mut xmTextFieldWidgetClass: WidgetClass;      // C: Xm/TextF.h
  pub static mut xmListWidgetClass: WidgetClass;           // C: Xm/List.h
}

#[cfg (feature = "use_motif")]
unsafe extern "C" {
  /* C: Xm/Xm.h: the five class predicates (`XmIsPrimitive`, `XmIsManager`, `XmIsGadget`,
     `XmIsRowColumn`, `XmIsCascadeButton`) are macros in C (`XtIsSubclass (w, …)`), and the port
     declares the predicate the C text calls, the `lwlib-Xlw.rs` reading of `XtIsShell`.  */
  pub fn XmIsPrimitive (widget: Widget) -> Boolean;
  pub fn XmIsManager (widget: Widget) -> Boolean;
  pub fn XmIsGadget (widget: Widget) -> Boolean;
  pub fn XmIsRowColumn (widget: Widget) -> Boolean;
  pub fn XmIsCascadeButton (widget: Widget) -> Boolean;
  /* C: Xm/DialogS.h: `Widget XmCreateDialogShell (Widget, char *, ArgList, Cardinal);`
     and the rest of the creators: `Widget XmCreate{Form, RowColumn, PushButton, Label,
     Separator, TextField, RadioBox, ToggleButtonGadget, ScrolledList, PulldownMenu,
     CascadeButton, ToggleButton, MenuBar, PopupMenu, MainWindow} (Widget, char *, ArgList,
     Cardinal);` - `XmCreateRadioBox`, `XmCreateScrolledList` and `XmCreateMainWindow` are
     convenience macros in C and the port declares the call the C text makes.  */
  pub fn XmCreateDialogShell (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateForm (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal) -> Widget;
  pub fn XmCreateRowColumn (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreatePushButton (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateLabel (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal) -> Widget;
  pub fn XmCreateSeparator (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateTextField (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateRadioBox (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateToggleButtonGadget (parent: Widget, name: *mut c_char, al: *mut Arg,
                                     ac: Cardinal) -> Widget;
  pub fn XmCreateScrolledList (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreatePulldownMenu (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateCascadeButton (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateToggleButton (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateMenuBar (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreatePopupMenu (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  pub fn XmCreateMainWindow (parent: Widget, name: *mut c_char, al: *mut Arg, ac: Cardinal)
    -> Widget;
  /* C: Xm/Xm.h: `XmString XmStringCreateLocalized (char *);` / `void XmStringFree (XmString);`
     C: Xm/List.h: `void XmListAddItem (Widget, XmString, int);` / `void XmListSelectPos (Widget,
     int, Boolean);` / `Boolean XmListGetSelectedPos (Widget, int **, int *);`
     C: Xm/Text.h: `void XmTextSetString (Widget, char *);` / `char *XmTextGetString (Widget);`
     C: Xm/TextF.h: `void XmTextFieldSetString (Widget, char *);` / `char *XmTextFieldGetString
     (Widget);`
     C: Xm/MenuShell.h: `void XmRemoveFromPostFromList (Widget, Widget);`
     C: Xm/Xm.h: `void XmMenuPosition (Widget, XButtonPressedEvent *);` / `void
     XmProcessTraversal (Widget, int);`
     C: Xm/MainW.h: `void XmMainWindowSetAreas (Widget, Widget, Widget, Widget, Widget,
     Widget);`  */
  pub fn XmStringCreateLocalized (string: *mut c_char) -> XmString;
  pub fn XmStringFree (string: XmString);
  pub fn XmListAddItem (widget: Widget, item: XmString, position: c_int);
  pub fn XmListSelectPos (widget: Widget, position: c_int, notify: Boolean);
  pub fn XmListGetSelectedPos (widget: Widget, positions: *mut *mut c_int,
                               count: *mut c_int) -> Boolean;
  pub fn XmTextSetString (widget: Widget, value: *mut c_char);
  pub fn XmTextGetString (widget: Widget) -> *mut c_char;
  pub fn XmTextFieldSetString (widget: Widget, value: *mut c_char);
  pub fn XmTextFieldGetString (widget: Widget) -> *mut c_char;
  pub fn XmRemoveFromPostFromList (menu: Widget, post_from: Widget);
  pub fn XmMenuPosition (widget: Widget, event: *mut XButtonPressedEvent);
  pub fn XmProcessTraversal (widget: Widget, direction: c_int);
  pub fn XmMainWindowSetAreas (parent: Widget, menubar: Widget, command: Widget,
                               hscroll: Widget, vscroll: Widget, work_area: Widget);
}

/* ==========================================================================================
   C's text, in C's order.
   ========================================================================================== */

/* C 63: `enum do_call_type { pre_activate, selection, no_selection, post_activate };` - one
   constant per member (PORTING.md 1.3), in C's declaration order.  */
#[cfg (feature = "use_motif")]
pub type do_call_type = c_int;
#[cfg (feature = "use_motif")]
pub const pre_activate: do_call_type = 0;
#[cfg (feature = "use_motif")]
pub const selection: do_call_type = 1;
#[cfg (feature = "use_motif")]
pub const no_selection: do_call_type = 2;
#[cfg (feature = "use_motif")]
pub const post_activate: do_call_type = 3;

/* C 66-75: `typedef struct _destroyed_instance { char* name; char* type; Widget widget; Widget
   parent; Boolean pop_up_p; struct _destroyed_instance* next; } destroyed_instance;`  */
#[cfg (feature = "use_motif")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct _destroyed_instance
{
  pub name: *mut c_char,
  pub r#type: *mut c_char,
  pub widget: Widget,
  pub parent: Widget,
  pub pop_up_p: Boolean,
  pub next: *mut _destroyed_instance,
}
#[cfg (feature = "use_motif")]
pub type destroyed_instance = _destroyed_instance;

/* C 66's comment `/* Structures to keep destroyed instances */` and C 77-121's static forward
   declarations.  The declarations are **deleted**: Rust items are order-independent, so
   nothing takes their place (PORTING.md 0.1-12 / checklist 10).  C names them:
   `make_destroyed_instance`, `free_destroyed_instance`, `resource_motif_string`,
   `destroy_all_children`, `xm_update_label`, `xm_update_list`, `xm_update_pushbutton`,
   `xm_update_cascadebutton`, `xm_update_toggle`, `xm_update_radiobox`, `make_menu_in_widget`,
   `update_one_menu_entry`, `xm_update_menu`, `xm_update_text`, `xm_update_text_field`,
   `activate_button`, `make_dialog`, `find_matching_instance`, `mark_dead_instance_destroyed`,
   `recenter_widget`, `recycle_instance`, `make_menubar`, `remove_grabs`, `make_popup_menu`,
   `make_main`, `set_min_dialog_size`, `do_call`, `xm_generic_callback`, `xm_nosel_callback`,
   `xm_pull_down_callback`, `xm_pop_down_callback`, `xm_pop_up_callback`,
   `xm_internal_update_other_instances`, `xm_arm_callback` (C 124's `xm_update_one_widget` and
   C 126's `xm_pop_instance` are inside its own `#if 0` below).  */

/* C 123-128: `#if 0` around three declarations (`xm_update_one_widget`, `xm_pop_instance`,
   `xm_manage_resizing` - all three are declared in `lwlib-Xm.h`).  Dead in every configuration,
   so the port records the region instead of carrying it (the `src/character.rs` 91-97
   precedent).  */
/* C 131-153: `#if 0` around `void x_print_complete_resource_name (Widget widget)` (C 136-151)
   and the `#endif /* 0 */` at C 153.  The same: recorded, not carried.  */

/* C 156: `static destroyed_instance *all_destroyed_instances = NULL;`  */
#[cfg (feature = "use_motif")]
pub static mut all_destroyed_instances: *mut destroyed_instance =
  NULL as *mut destroyed_instance;

/* C 158-174: `make_destroyed_instance` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn make_destroyed_instance (name: *mut c_char, r#type: *mut c_char, widget: Widget,
                                   parent: Widget, pop_up_p: Boolean)
  -> *mut destroyed_instance
{
  let instance: *mut destroyed_instance =
    xmalloc (core::mem::size_of::<destroyed_instance> ()) as *mut destroyed_instance;
  (*instance).name = xstrdup (name);
  (*instance).r#type = xstrdup (r#type);
  (*instance).widget = widget;
  (*instance).parent = parent;
  (*instance).pop_up_p = pop_up_p;
  (*instance).next = NULL as *mut _destroyed_instance;
  return instance;
}

/* C 176-182: `free_destroyed_instance` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn free_destroyed_instance (instance: *mut destroyed_instance)
{
  xfree ((*instance).name as *mut c_void);
  xfree ((*instance).r#type as *mut c_void);
  xfree (instance as *mut c_void);
}

/* C 184-189: `motif utility functions` and `first_child` (C 81 declares it non-static, so it
   keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn first_child (widget: Widget) -> Widget
{
  return *(*(widget as CompositeWidget)).composite.children.offset (0);
}

/* C 191-197: `lw_motif_widget_p` (declared in `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_motif_widget_p (widget: Widget) -> Boolean
{
  return (XtClass (widget) == xmDialogShellWidgetClass
          || XmIsPrimitive (widget) != 0 || XmIsManager (widget) != 0
          || XmIsGadget (widget) != 0) as Boolean;
}

/* C 199-217: `resource_motif_string` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn resource_motif_string (widget: Widget, name: *mut c_char) -> XmString
{
  let mut resource: XtResource = core::mem::zeroed ();
  let mut result: XmString = 0 as XmString;

  resource.resource_name = name;
  resource.resource_class = XmCXmString.as_ptr () as String;
  resource.resource_type = XmRXmString.as_ptr () as String;
  resource.resource_size = core::mem::size_of::<XmString> () as Cardinal;
  resource.resource_offset = 0;
  resource.default_type = XtRImmediate.as_ptr () as String;
  resource.default_addr = 0 as XtPointer;

  XtGetSubresources (widget, (&raw mut result) as XtPointer, c"dialogString".as_ptr (),
                     c"DialogString".as_ptr (), &raw mut resource, 1, NULL, 0);
  return result;
}

/* C 219-259: `destroy_all_children` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn destroy_all_children (widget: Widget, first_child_to_destroy: c_int)
{
  let mut children: *mut Widget;
  let mut number: c_uint = 0;
  let mut i: c_int;

  children = XtCompositeChildren (widget, &raw mut number);
  if !children.is_null ()
    {
      XtUnmanageChildren (children.offset (first_child_to_destroy as isize),
                          number - (first_child_to_destroy as c_uint));

      /* Unmanage all children and destroy them.  They will only be really destroyed when we get
	 out of DispatchEvent.  */
      i = first_child_to_destroy;
      while (i as c_uint) < number
	{
	  let mut al: [Arg; 2] = [core::mem::zeroed (); 2];
	  let mut submenu: Widget = 0 as Widget;
	  /* Cascade buttons have submenus,and these submenus need to be freed.  But they are not
	     included in XtCompositeChildren.  So get it out of the cascade button and free it.  If
	     this child is not a cascade button, then submenu should remain unchanged.  */
	  XtSetArg! (al[0], XmNsubMenuId.as_ptr (), &raw mut submenu);
	  XtGetValues (*children.offset (i as isize), al.as_mut_ptr (), 1);
	  if !submenu.is_null ()
	    {
	      destroy_all_children (submenu, 0);
	      XtDestroyWidget (submenu);
	    }
	  XtDestroyWidget (*children.offset (i as isize));
	  i += 1;
	}

      XtFree (children as *mut c_char);
    }
}

/* C 263-295: `xm_arm_callback` (static; the `XtAddCallback` `XtCallbackProc` of C 573-584 and
   C 596, so it is `extern "C"`).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_arm_callback (w: Widget, client_data: XtPointer,
                                      mut call_data: XtPointer)
{
  let cbs: *mut XmPushButtonCallbackStruct = call_data as *mut XmPushButtonCallbackStruct;
  let mut wv: *mut widget_value = NULL as *mut widget_value;
  let instance: *mut widget_instance = client_data as *mut widget_instance;
  let mut user_data: XtPointer = NULL;
  let mut al: [Arg; 2] = [core::mem::zeroed (); 2];
  let mut ac: c_int = 0;

  XtSetArg! (al[ac as usize], XmNuserData.as_ptr (), &raw mut user_data);
  ac += 1;
  XtGetValues (w, al.as_mut_ptr (), ac as Cardinal);
  wv = user_data as *mut widget_value;

  if !wv.is_null ()
    {
      if (*(*instance).info).highlight_cb.is_some ()
	  && ((*cbs).reason == XmCR_DISARM
	      || (!(*cbs).event.is_null ()
		  && ((*(*cbs).event).r#type == EnterNotify
		      || (*(*cbs).event).r#type == MotionNotify)))
	{
	  call_data = if (*cbs).reason == XmCR_DISARM { NULL }
		      else { wv as XtPointer };
	  ((*(*instance).info).highlight_cb.unwrap ()) (w, (*(*instance).info).id, call_data);
	}
    }
}

/* C 299-314: `xm_update_label`'s comment.  Menus: Emacs fills VAL->name with the text to
   display in the menu, and sets VAL->value to null; `make_menu_in_widget` creates widgets with
   VAL->name as resource name, which works because the Label widget uses its resource name for
   display if no XmNlabelString is set.  Dialogs: VAL->name is again set to the resource name,
   but VAL->value is not null and contains the label string to display.  */

/* C 316-363: `xm_update_label` (static; WIDGET_INSTANCE is unused, as C 300 says).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_label (_instance: *mut widget_instance, widget: Widget,
                           val: *mut widget_value)
{
  let mut res_string: XmString = 0 as XmString;
  let mut built_string: XmString = 0 as XmString;
  let mut key_string: XmString = 0 as XmString;
  let mut al: [Arg; 256] = [core::mem::zeroed (); 256];
  let mut ac: c_int;

  ac = 0;

  if !(*val).value.is_null ()
    {
      /* A label string is specified, i.e. we are in a dialog.  First see if it is overridden
	 by something from the resource file.  */
      res_string = resource_motif_string (widget, (*val).value);

      if !res_string.is_null ()
	{
	  XtSetArg! (al[ac as usize], XmNlabelString.as_ptr (), res_string);
	  ac += 1;
	}
      else
	{
	  built_string = XmStringCreateLocalized ((*val).value);
	  XtSetArg! (al[ac as usize], XmNlabelString.as_ptr (), built_string);
	  ac += 1;
	}

      XtSetArg! (al[ac as usize], XmNlabelType.as_ptr (), XmSTRING as XtPointer);
      ac += 1;
    }

  if !(*val).key.is_null ()
    {
      key_string = XmStringCreateLocalized ((*val).key);
      XtSetArg! (al[ac as usize], XmNacceleratorText.as_ptr (), key_string);
      ac += 1;
    }

  if ac != 0
    { XtSetValues (widget, al.as_mut_ptr (), ac as Cardinal); }

  if !built_string.is_null ()
    { XmStringFree (built_string); }

  if !key_string.is_null ()
    { XmStringFree (key_string); }
}

/* C 365-386: `update of list` and `xm_update_list` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_list (instance: *mut widget_instance, widget: Widget,
                          val: *mut widget_value)
{
  let mut cur: *mut widget_value;
  let mut i: c_int;
  XtRemoveAllCallbacks (widget, XmNsingleSelectionCallback.as_ptr ());
  XtAddCallback (widget, XmNsingleSelectionCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);
  cur = (*val).contents;
  i = 0;
  while !cur.is_null ()
    {
      if !(*cur).value.is_null ()
	{
	  let xmstr: XmString = XmStringCreateLocalized ((*cur).value);
	  i += 1;
	  XmListAddItem (widget, xmstr, 0);
	  if (*cur).selected
	    { XmListSelectPos (widget, i, False); }
	  XmStringFree (xmstr);
	}
      cur = (*cur).next;
    }
}

/* C 388-397: `update of buttons` and `xm_update_pushbutton` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_pushbutton (instance: *mut widget_instance, widget: Widget,
                                _val: *mut widget_value)
{
  XtVaSetValues (widget, XmNalignment.as_ptr (), XmALIGNMENT_CENTER,
		 NULL);
  XtRemoveAllCallbacks (widget, XmNactivateCallback.as_ptr ());
  XtAddCallback (widget, XmNactivateCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);
}

/* C 399-408: `xm_update_cascadebutton` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_cascadebutton (instance: *mut widget_instance, widget: Widget,
                                   _val: *mut widget_value)
{
  /* Should also rebuild the menu by calling ...update_menu... */
  XtRemoveAllCallbacks (widget, XmNcascadingCallback.as_ptr ());
  XtAddCallback (widget, XmNcascadingCallback.as_ptr (), Some (xm_pull_down_callback),
		 instance as XtPointer);
}

/* C 410-421: `update toggle and radiobox` and `xm_update_toggle` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_toggle (instance: *mut widget_instance, widget: Widget,
                            val: *mut widget_value)
{
  XtRemoveAllCallbacks (widget, XmNvalueChangedCallback.as_ptr ());
  XtAddCallback (widget, XmNvalueChangedCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);
  XtVaSetValues (widget, XmNset.as_ptr (), (*val).selected as c_int,
		 XmNalignment.as_ptr (), XmALIGNMENT_BEGINNING, NULL);
}

/* C 423-462: `xm_update_radiobox` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_radiobox (instance: *mut widget_instance, widget: Widget,
                              val: *mut widget_value)
{
  let mut toggle: Widget;
  let mut cur: *mut widget_value;

  /* update the callback */
  XtRemoveAllCallbacks (widget, XmNentryCallback.as_ptr ());
  XtAddCallback (widget, XmNentryCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);

  /* first update all the toggles */
  /* Energize kernel interface is currently bad.  It sets the selected widget with the selected
     flag but returns it by its name.  So we currently have to support both setting the
     selection with the selected slot of val contents and setting it with the "value" slot of
     val.  The latter has a higher priority.  This to be removed when the kernel is fixed. */
  cur = (*val).contents;
  while !cur.is_null ()
    {
      toggle = XtNameToWidget (widget, (*cur).value);
      if !toggle.is_null ()
	{
	  XtSetSensitive (toggle, (*cur).enabled as Boolean);
	  if (*val).value.is_null () && (*cur).selected
	    { XtVaSetValues (toggle, XmNset.as_ptr (), (*cur).selected as c_int,
			     NULL); }
	  if !(*val).value.is_null () && strcmp ((*val).value, (*cur).value) != 0
	    { XtVaSetValues (toggle, XmNset.as_ptr (), False, NULL); }
	}
      cur = (*cur).next;
    }

  /* The selected was specified by the value slot */
  if !(*val).value.is_null ()
    {
      toggle = XtNameToWidget (widget, (*val).value);
      if !toggle.is_null ()
	{ XtVaSetValues (toggle, XmNset.as_ptr (), True, NULL); }
    }
}

/* C 464-468: `update a popup menu, pulldown menu or a menubar`; `KEEP_FIRST_CHILDREN gives the
   number of initial children to keep.`  */
/* C 469-632: `make_menu_in_widget` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn make_menu_in_widget (instance: *mut widget_instance, widget: Widget,
                               val: *mut widget_value, keep_first_children: c_int)
{
  let mut children: *mut Widget = 0 as *mut Widget;
  let mut num_children: c_int;
  let mut child_index: c_int;
  let mut cur: *mut widget_value;
  let mut button: Widget = 0 as Widget;
  let mut menu: Widget;
  let mut al: [Arg; 256] = [core::mem::zeroed (); 256];
  let mut ac: c_int;
  let mut menubar_p: Boolean;
  let mut r#type: c_uchar;

  let mut old_children: *mut Widget;
  let mut old_num_children: c_uint = 0;

  /* Disable drag and drop for labels in menu bar.  C's `static char overrideTrans[]` is the
     port's `&CStr` of the same bytes and the pointer C passes is the same (the `lwlib-Xaw.rs`
     232-234 reading of the same shape).  */
  let overrideTrans: &core::ffi::CStr = c"<Btn2Down>: Noop()";
  let mut override_: XtTranslations = XtParseTranslationTable (overrideTrans.as_ptr ());

  old_children = XtCompositeChildren (widget, &raw mut old_num_children);

  /* Allocate the children array */
  num_children = 0;
  cur = val;
  while !cur.is_null ()
    {
      num_children += 1;
      cur = (*cur).next;
    }
  children = XtMalloc (((num_children as usize) * core::mem::size_of::<Widget> ()) as Cardinal)
    as *mut Widget;

  #[cfg (feature = "not_lesstif_version")]
  /* WIDGET should be a RowColumn.  */
  if XmIsRowColumn (widget) == 0
    { emacs_abort (); }

  /* Determine whether WIDGET is a menu bar.  */
  r#type = (-1 as c_int) as c_uchar;
  XtSetArg! (al[0], XmNrowColumnType.as_ptr (), &raw mut r#type);
  XtGetValues (widget, al.as_mut_ptr (), 1);
  if r#type != XmMENU_BAR && r#type != XmMENU_PULLDOWN && r#type != XmMENU_POPUP
    { emacs_abort (); }
  menubar_p = (r#type == XmMENU_BAR) as Boolean;

  /* Add a callback to popups and pulldowns that is called when it is made invisible again.  */
  if menubar_p == 0
    {
      XtAddCallback (XtParent (widget), XmNpopdownCallback.as_ptr (),
		     Some (xm_pop_down_callback), instance as XtPointer);
      XtAddCallback (XtParent (widget), XmNpopupCallback.as_ptr (),
		     Some (xm_pop_up_callback), instance as XtPointer);
    }

  /* Preserve the first KEEP_FIRST_CHILDREN old children.  */
  child_index = 0;
  cur = val;
  while child_index < keep_first_children
    {
      *children.offset (child_index as isize) = *old_children.offset (child_index as isize);
      child_index += 1;
      cur = (*cur).next;
    }

  /* Check that those are all we have (the caller should have deleted the rest).  */
  if old_num_children != keep_first_children as c_uint
    { emacs_abort (); }

  /* Create the rest.  */
  child_index = keep_first_children;
  while !cur.is_null ()
    {
      let mut separator: menu_separator = 0;

      ac = 0;
      XtSetArg! (al[ac as usize], XmNsensitive.as_ptr (), (*cur).enabled as c_int as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNalignment.as_ptr (), XmALIGNMENT_BEGINNING as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNuserData.as_ptr (), cur as XtPointer);
      ac += 1;

      if (*instance).pop_up_p != 0 && (*cur).contents.is_null () && (*cur).call_data.is_null ()
	  && lw_separator_p ((*cur).name, &raw mut separator, 1) == 0
	{
	  ac = 0;
	  XtSetArg! (al[ac as usize], XmNalignment.as_ptr (), XmALIGNMENT_CENTER as XtPointer);
	  ac += 1;
	  button = XmCreateLabel (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal);
	}
      else if lw_separator_p ((*cur).name, &raw mut separator, 1) != 0
	{
	  ac = 0;
	  XtSetArg! (al[ac as usize], XmNseparatorType.as_ptr (), separator as XtPointer);
	  ac += 1;
	  button = XmCreateSeparator (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal);
	}
      else if (*cur).contents.is_null ()
	{
	  if menubar_p != 0
	    { button = XmCreateCascadeButton (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal); }
	  else if (*cur).call_data.is_null ()
	    { button = XmCreateLabel (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal); }
	  else if (*cur).button_type == BUTTON_TYPE_TOGGLE
		   || (*cur).button_type == BUTTON_TYPE_RADIO
	    {
	      XtSetArg! (al[ac as usize], XmNset.as_ptr (), (*cur).selected as c_int as XtPointer);
	      ac += 1;
	      XtSetArg! (al[ac as usize], XmNvisibleWhenOff.as_ptr (), True as XtPointer);
	      ac += 1;
	      XtSetArg! (al[ac as usize], XmNindicatorType.as_ptr (),
			 (if (*cur).button_type == BUTTON_TYPE_TOGGLE { XmN_OF_MANY }
			  else { XmONE_OF_MANY }) as XtPointer);
	      ac += 1;
	      button = XmCreateToggleButton (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal);
	      XtAddCallback (button, XmNarmCallback.as_ptr (), Some (xm_arm_callback),
			     instance as XtPointer);
	      XtAddCallback (button, XmNdisarmCallback.as_ptr (), Some (xm_arm_callback),
			     instance as XtPointer);
	    }
	  else
	    {
	      button = XmCreatePushButton (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal);
	      XtAddCallback (button, XmNarmCallback.as_ptr (), Some (xm_arm_callback),
			     instance as XtPointer);
	      XtAddCallback (button, XmNdisarmCallback.as_ptr (), Some (xm_arm_callback),
			     instance as XtPointer);
	    }

	  xm_update_label (instance, button, cur);

	  /* Add a callback that is called when the button is selected.  Toggle buttons don't
	     support XmNactivateCallback, we use XmNvalueChangedCallback in that case.  Don't add a
	     callback to a simple label.  */
	  if (*cur).button_type != 0
	    { xm_update_toggle (instance, button, cur); }
	  else if !(*cur).call_data.is_null ()
	    { XtAddCallback (button, XmNactivateCallback.as_ptr (), Some (xm_generic_callback),
			     instance as XtPointer); }
	}
      else
	{
	  menu = XmCreatePulldownMenu (widget, (*cur).name, core::ptr::null_mut (), 0);

	  make_menu_in_widget (instance, menu, (*cur).contents, 0);
	  XtSetArg! (al[ac as usize], XmNsubMenuId.as_ptr (), menu as XtPointer);
	  ac += 1;
	  button = XmCreateCascadeButton (widget, (*cur).name, al.as_mut_ptr (), ac as Cardinal);

	  xm_update_label (instance, button, cur);

	  XtAddCallback (button, XmNcascadingCallback.as_ptr (), Some (xm_pull_down_callback),
			 instance as XtPointer);
	  XtOverrideTranslations (button, override_);
	}

      *children.offset (child_index as isize) = button;
      child_index += 1;
      cur = (*cur).next;
    }

  /* Last entry is the help button.  The original comment read "Has to be done after managing
     the buttons otherwise the menubar is only 4 pixels high."  This is no longer true, and to
     make XmNmenuHelpWidget work, we need to set it before managing the children.. --gerd.  */
  if !button.is_null ()
    { XtVaSetValues (widget, XmNmenuHelpWidget.as_ptr (), button, NULL); }

  if num_children != 0
    { XtManageChildren (children, num_children as Cardinal); }

  XtFree (children as *mut c_char);
  if !old_children.is_null ()
    { XtFree (old_children as *mut c_char); }
}

/* C 634-734: `update_one_menu_entry` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn update_one_menu_entry (instance: *mut widget_instance, widget: Widget,
                                 val: *mut widget_value, deep_p: Boolean)
{
  let mut al: [Arg; 256] = [core::mem::zeroed (); 256];
  let mut ac: c_int;
  let mut menu: Widget;
  let mut contents: *mut widget_value;

  if (*val).this_one_change == NO_CHANGE
    { return; }

  /* update the sensitivity and userdata */
  /* Common to all widget types */
  XtSetSensitive (widget, (*val).enabled as Boolean);
  XtVaSetValues (widget, XmNuserData.as_ptr (), val as XtPointer, NULL);

  /* update the menu button as a label. */
  if (*val).this_one_change >= VISIBLE_CHANGE
    {
      xm_update_label (instance, widget, val);
      if (*val).button_type != 0
	{ xm_update_toggle (instance, widget, val); }
    }

  /* update the pulldown/pullaside as needed */
  ac = 0;
  menu = NULL as Widget;
  XtSetArg! (al[ac as usize], XmNsubMenuId.as_ptr (), &raw mut menu);
  ac += 1;
  XtGetValues (widget, al.as_mut_ptr (), ac as Cardinal);

  contents = (*val).contents;

  if menu.is_null ()
    {
      if !contents.is_null ()
	{
	  let mut old_num_children: c_uint = 0;
	  let mut i: c_uint;
	  let parent: Widget;
	  let mut widget_list: *mut Widget;

	  parent = XtParent (widget);
	  widget_list = XtCompositeChildren (parent, &raw mut old_num_children);

	  /* Find the widget position within the parent's widget list.  */
	  i = 0;
	  while i < old_num_children
	    {
	      if strcmp (XtName (*widget_list.offset (i as isize)), XtName (widget)) == 0
		{ break; }
	      i += 1;
	    }
	  if i == old_num_children
	    { emacs_abort (); }
	  if XmIsCascadeButton (*widget_list.offset (i as isize)) != 0
	    {
	      menu = XmCreatePulldownMenu (parent, XtName (widget) as *mut c_char,
					   core::ptr::null_mut (), 0);
	      make_menu_in_widget (instance, menu, contents, 0);
	      ac = 0;
	      XtSetArg! (al[ac as usize], XmNsubMenuId.as_ptr (), menu as XtPointer);
	      ac += 1;
	      XtSetValues (widget, al.as_mut_ptr (), ac as Cardinal);
	    }
	  else
	    {
	      let mut button: Widget;

	      /* The current menuitem is a XmPushButtonGadget, it needs to be replaced by a
		 CascadeButtonGadget */
	      XtDestroyWidget (*widget_list.offset (i as isize));
	      menu = XmCreatePulldownMenu (parent, (*val).name, core::ptr::null_mut (), 0);
	      make_menu_in_widget (instance, menu, contents, 0);
	      ac = 0;
	      XtSetArg! (al[ac as usize], XmNsubMenuId.as_ptr (), menu as XtPointer);
	      ac += 1;
	      /* Non-zero values don't work reliably in conjunction with Emacs's event loop */
	      XtSetArg! (al[ac as usize], XmNmappingDelay.as_ptr (), 0 as XtPointer);
	      ac += 1;
	      #[cfg (feature = "xm_position_index")]
	      /* Tell Motif to put it in the right place */
	      { XtSetArg! (al[ac as usize], XmNpositionIndex.as_ptr (), i as c_int as XtPointer);
		ac += 1; }
	      button = XmCreateCascadeButton (parent, (*val).name, al.as_mut_ptr (),
					      ac as Cardinal);
	      xm_update_label (instance, button, val);

	      XtAddCallback (button, XmNcascadingCallback.as_ptr (),
			     Some (xm_pull_down_callback), instance as XtPointer);
	      XtManageChild (button);
	    }

	  if !widget_list.is_null ()
	    { XtFree (widget_list as *mut c_char); }
	}
    }
  else if contents.is_null ()
    {
      ac = 0;
      XtSetArg! (al[ac as usize], XmNsubMenuId.as_ptr (), core::ptr::null_mut ());
      ac += 1;
      XtSetValues (widget, al.as_mut_ptr (), ac as Cardinal);
      XtDestroyWidget (menu);
    }
  else if deep_p != 0 && (*contents).change != NO_CHANGE
    { xm_update_menu (instance, menu, val, 1); }
}

/* C 736-805: `xm_update_menu` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_menu (instance: *mut widget_instance, widget: Widget,
                          val: *mut widget_value, deep_p: Boolean)
{
  let mut children: *mut Widget;
  let mut num_children: c_uint = 0;
  let mut num_children_to_keep: c_int = 0;
  let mut i: c_int;
  let mut cur: *mut widget_value;

  children = XtCompositeChildren (widget, &raw mut num_children);

  /* Widget is a RowColumn widget whose contents have to be updated to reflect the list of items
     in val->contents */

  /* See how many buttons we can keep, and how many we must completely replace.  */
  if (*val).contents.is_null ()
    { num_children_to_keep = 0; }
  else if (*(*val).contents).change == STRUCTURAL_CHANGE
    {
      if !children.is_null ()
	{
	  i = 0;
	  cur = (*val).contents;
	  while (i as c_uint) < num_children && !cur.is_null ()
	    { /* how else to ditch unwanted children ?? - mgd */
	      if (*cur).this_one_change == STRUCTURAL_CHANGE
		{ break; }
	      i += 1;
	      cur = (*cur).next;
	    }

	  num_children_to_keep = i;
	}
    }
  else
    { num_children_to_keep = num_children as c_int; }

  /* Update all the buttons of the RowColumn, in order, except for those we are going to
     replace entirely.  */
  if !children.is_null ()
    {
      i = 0;
      cur = (*val).contents;
      while i < num_children_to_keep
	{
	  if cur.is_null ()
	    { num_children_to_keep = i; break; }
	  if (**children.offset (i as isize)).core.being_destroyed != 0
	      || strcmp (XtName (*children.offset (i as isize)), (*cur).name) != 0
	    { i += 1; continue; }
	  update_one_menu_entry (instance, *children.offset (i as isize), cur, deep_p);
	  cur = (*cur).next;
	  i += 1;
	}
    }

  /* Now replace from scratch all the buttons after the last place that the top-level structure
     changed.  */
  if !(*val).contents.is_null () && (*(*val).contents).change == STRUCTURAL_CHANGE
    {
      destroy_all_children (widget, num_children_to_keep);
      make_menu_in_widget (instance, widget, (*val).contents, num_children_to_keep);
    }

  XtFree (children as *mut c_char);
}

/* C 807-821: `update text widgets` and `xm_update_text` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_text (instance: *mut widget_instance, widget: Widget,
                          val: *mut widget_value)
{
  XmTextSetString (widget, if !(*val).value.is_null () { (*val).value }
			  else { c"".as_ptr () as *mut c_char });
  XtRemoveAllCallbacks (widget, XmNactivateCallback.as_ptr ());
  XtAddCallback (widget, XmNactivateCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);
  XtRemoveAllCallbacks (widget, XmNvalueChangedCallback.as_ptr ());
  XtAddCallback (widget, XmNvalueChangedCallback.as_ptr (),
		 Some (xm_internal_update_other_instances), instance as XtPointer);
}

/* C 823-834: `xm_update_text_field` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn xm_update_text_field (instance: *mut widget_instance, widget: Widget,
                                val: *mut widget_value)
{
  XmTextFieldSetString (widget, if !(*val).value.is_null () { (*val).value }
			       else { c"".as_ptr () as *mut c_char });
  XtRemoveAllCallbacks (widget, XmNactivateCallback.as_ptr ());
  XtAddCallback (widget, XmNactivateCallback.as_ptr (), Some (xm_generic_callback),
		 instance as XtPointer);
  XtRemoveAllCallbacks (widget, XmNvalueChangedCallback.as_ptr ());
  XtAddCallback (widget, XmNvalueChangedCallback.as_ptr (),
		 Some (xm_internal_update_other_instances), instance as XtPointer);
}

/* C 836-900: `update a motif widget` and `xm_update_one_widget` (declared in `lwlib-Xm.h`, so
   it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_update_one_widget (instance: *mut widget_instance, widget: Widget,
                                               val: *mut widget_value, deep_p: Boolean)
{
  let mut class: WidgetClass;

  /* Mark as not edited */
  (*val).edited = false;

  /* Common to all widget types */
  XtSetSensitive (widget, (*val).enabled as Boolean);
  XtVaSetValues (widget, XmNuserData.as_ptr (), val as XtPointer, NULL);

  /* Common to all label like widgets */
  if XtIsSubclass (widget, xmLabelWidgetClass) != 0
    { xm_update_label (instance, widget, val); }

  class = XtClass (widget);
  /* Class specific things */
  if class == xmPushButtonWidgetClass || class == xmArrowButtonWidgetClass
    {
      xm_update_pushbutton (instance, widget, val);
    }
  else if class == xmCascadeButtonWidgetClass
    {
      xm_update_cascadebutton (instance, widget, val);
    }
  else if class == xmToggleButtonWidgetClass || class == xmToggleButtonGadgetClass
    {
      xm_update_toggle (instance, widget, val);
    }
  else if class == xmRowColumnWidgetClass
    {
      let mut radiobox: Boolean = 0;
      let mut ac: c_int = 0;
      let mut al: [Arg; 1] = [core::mem::zeroed (); 1];

      XtSetArg! (al[ac as usize], XmNradioBehavior.as_ptr (), &raw mut radiobox);
      ac += 1;
      XtGetValues (widget, al.as_mut_ptr (), ac as Cardinal);

      if radiobox != 0
	{ xm_update_radiobox (instance, widget, val); }
      else
	{ xm_update_menu (instance, widget, val, deep_p); }
    }
  else if class == xmTextWidgetClass
    {
      xm_update_text (instance, widget, val);
    }
  else if class == xmTextFieldWidgetClass
    {
      xm_update_text_field (instance, widget, val);
    }
  else if class == xmListWidgetClass
    {
      xm_update_list (instance, widget, val);
    }
}

/* C 902-986: `getting the value back` and `xm_update_one_value` (declared in `lwlib-Xm.h`, so
   it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_update_one_value (instance: *mut widget_instance, widget: Widget,
                                              val: *mut widget_value)
{
  let class: WidgetClass = XtClass (widget);
  let mut old_wv: *mut widget_value;

  /* copy the call_data slot into the "return" widget_value */
  old_wv = (*(*(*instance).info).val).contents;
  while !old_wv.is_null ()
    {
      if strcmp ((*val).name, (*old_wv).name) == 0
	{
	  (*val).call_data = (*old_wv).call_data;
	  break;
	}
      old_wv = (*old_wv).next;
    }

  if class == xmToggleButtonWidgetClass || class == xmToggleButtonGadgetClass
    {
      XtVaGetValues (widget, XmNset.as_ptr (), &raw mut (*val).selected,
		     NULL);
      (*val).edited = true;
    }
  else if class == xmTextWidgetClass
    {
      xfree ((*val).value as *mut c_void);
      (*val).value = XmTextGetString (widget);
      (*val).edited = true;
    }
  else if class == xmTextFieldWidgetClass
    {
      xfree ((*val).value as *mut c_void);
      (*val).value = XmTextFieldGetString (widget);
      (*val).edited = true;
    }
  else if class == xmRowColumnWidgetClass
    {
      let mut radiobox: Boolean = 0;
      let mut ac: c_int = 0;
      let mut al: [Arg; 1] = [core::mem::zeroed (); 1];

      XtSetArg! (al[ac as usize], XmNradioBehavior.as_ptr (), &raw mut radiobox);
      ac += 1;
      XtGetValues (widget, al.as_mut_ptr (), ac as Cardinal);

      if radiobox != 0
	{
	  let radio: CompositeWidget = widget;
	  let mut i: c_int = 0;
	  while (i as c_uint) < (*radio).composite.num_children
	    {
	      let mut set: c_int = False;
	      let toggle: Widget = *(*radio).composite.children.offset (i as isize);

	      XtVaGetValues (toggle, XmNset.as_ptr (), &raw mut set,
			     NULL);
	      if set != 0
		{ dupstring (&raw mut (*val).value, XtName (toggle)); }
	      i += 1;
	    }
	  (*val).edited = true;
	}
    }
  else if class == xmListWidgetClass
    {
      let mut pos_cnt: c_int = 0;
      let mut pos_list: *mut c_int = core::ptr::null_mut ();
      if XmListGetSelectedPos (widget, &raw mut pos_list, &raw mut pos_cnt) != 0
	{
	  let mut i: c_int;
	  let mut cur: *mut widget_value;
	  cur = (*val).contents;
	  i = 0;
	  while !cur.is_null ()
	    {
	      if !(*cur).value.is_null ()
		{
		  let mut j: c_int;
		  (*cur).selected = false;
		  i += 1;
		  j = 0;
		  while j < pos_cnt
		    {
		      if *pos_list.offset (j as isize) == i
			{
			  (*cur).selected = true;
			  (*val).value = xstrdup ((*cur).name);
			}
		      j += 1;
		    }
		}
	      cur = (*cur).next;
	    }
	  (*val).edited = true;
	  XtFree (pos_list as *mut c_char);
	}
    }
}

/* C 988-1002: `activate_button`'s comment and the static itself (`XtAddCallback`'s
   `XtCallbackProc` at C 1248).  This function is for activating a button from a program.  It's
   wrong because we pass a NULL argument in the call_data which is not Motif compatible.  This
   is used from the XmNdefaultAction callback of the List widgets to have a double-click put
   down a dialog box like the button would do.  I could not find a way to do that with
   accelerators.  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn activate_button (_widget: Widget, closure: XtPointer,
                                      _call_data: XtPointer)
{
  let button: Widget = closure as Widget;
  XtCallCallbacks (button, XmNactivateCallback.as_ptr (), NULL);
}

/* C 1004-1030: `creation functions`, and `dialog_key_cb` (static; an `XtEventHandler` at
   C 1118-1119).  Called for key press in dialogs.  Used to pop down dialog on ESC.  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn dialog_key_cb (_widget: Widget, closure: XtPointer, event: *mut XEvent,
                                    continue_to_dispatch: *mut Boolean)
{
  let mut sym: KeySym = 0;
  let mut modif_ret: Modifiers = 0;

  XtTranslateKeycode ((*event).xkey.display, (*event).xkey.keycode as KeyCode, 0,
		      &raw mut modif_ret, &raw mut sym);

  if sym == osfXK_Cancel
    {
      let mut w: Widget = *(closure as *mut Widget);

      while !w.is_null () && XtIsShell (w) == 0
	{ w = XtParent (w); }

      if XtIsShell (w) != 0 { XtPopdown (w); }
    }

  *continue_to_dispatch = TRUE;
}

/* C 1032-1293: `dialogs` and `make_dialog` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn make_dialog (name: *mut c_char, parent: Widget, pop_up_p: Boolean,
                       shell_title: *mut c_char, icon_name: *mut c_char,
                       text_input_slot: Boolean, radio_box: Boolean, list: Boolean,
                       left_buttons: c_int, right_buttons: c_int) -> Widget
{
  let mut result: Widget;
  let mut form: Widget;
  let mut row: Widget;
  let mut icon: Widget;
  let mut icon_separator: Widget;
  let mut message_label: Widget;
  let mut value: Widget = 0 as Widget;
  let mut separator: Widget;
  let mut button: Widget = 0 as Widget;
  let mut children: [Widget; 16] = [core::ptr::null_mut (); 16];  /* for the final XtManageChildren */
  let mut n_children: c_int;
  let mut al: [Arg; 64] = [core::mem::zeroed (); 64];             /* Arg List */
  let mut ac: c_int;                                              /* Arg Count */
  let mut i: c_int;

  if pop_up_p != 0
    {
      ac = 0;
      XtSetArg! (al[ac as usize], XmNtitle.as_ptr (), shell_title as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XtNallowShellResize.as_ptr (), True as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNdeleteResponse.as_ptr (), XmUNMAP as XtPointer);
      ac += 1;
      result = XmCreateDialogShell (parent, c"dialog".as_ptr () as *mut c_char,
				    al.as_mut_ptr (), ac as Cardinal);
      ac = 0;
      XtSetArg! (al[ac as usize], XmNautoUnmanage.as_ptr (), FALSE as XtPointer);
      ac += 1;
      /*      XtSetArg(al[ac], XmNautoUnmanage, TRUE); ac++; */ /* ####is this ok? */
      XtSetArg! (al[ac as usize], XmNnavigationType.as_ptr (), XmTAB_GROUP as XtPointer);
      ac += 1;
      form = XmCreateForm (result, shell_title, al.as_mut_ptr (), ac as Cardinal);
    }
  else
    {
      ac = 0;
      XtSetArg! (al[ac as usize], XmNautoUnmanage.as_ptr (), FALSE as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNnavigationType.as_ptr (), XmTAB_GROUP as XtPointer);
      ac += 1;
      form = XmCreateForm (parent, shell_title, al.as_mut_ptr (), ac as Cardinal);
      result = form;
    }

  n_children = left_buttons + right_buttons + 1;
  ac = 0;
  XtSetArg! (al[ac as usize], XmNpacking.as_ptr (),
	     (if n_children == 3 { XmPACK_COLUMN } else { XmPACK_TIGHT }) as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNorientation.as_ptr (),
	     (if n_children == 3 { XmVERTICAL } else { XmHORIZONTAL }) as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNnumColumns.as_ptr (),
	     (left_buttons + right_buttons + 1) as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNmarginWidth.as_ptr (), 0 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNmarginHeight.as_ptr (), 0 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNspacing.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNadjustLast.as_ptr (), False as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNalignment.as_ptr (), XmALIGNMENT_CENTER as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNisAligned.as_ptr (), True as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  row = XmCreateRowColumn (form, c"row".as_ptr () as *mut c_char, al.as_mut_ptr (),
			   ac as Cardinal);

  n_children = 0;
  i = 0;
  while i < left_buttons
    {
      let mut button_name: [c_char; 16] = [0; 16];
      sprintf (button_name.as_mut_ptr (), c"button%d".as_ptr (), i + 1);
      ac = 0;
      if i == 0
	{
	  XtSetArg! (al[ac as usize], XmNhighlightThickness.as_ptr (), 1 as XtPointer);
	  ac += 1;
	  XtSetArg! (al[ac as usize], XmNshowAsDefault.as_ptr (), TRUE as XtPointer);
	  ac += 1;
	}
      XtSetArg! (al[ac as usize], XmNmarginWidth.as_ptr (), 10 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNnavigationType.as_ptr (), XmTAB_GROUP as XtPointer);
      ac += 1;
      children[n_children as usize] = XmCreatePushButton (row, button_name.as_mut_ptr (),
							  al.as_mut_ptr (), ac as Cardinal);
      XtAddEventHandler (children[n_children as usize], KeyPressMask, False,
			 Some (dialog_key_cb), result as XtPointer);

      if i == 0
	{
	  button = children[n_children as usize];
	  ac = 0;
	  XtSetArg! (al[ac as usize], XmNdefaultButton.as_ptr (), button as XtPointer);
	  ac += 1;
	  XtSetValues (row, al.as_mut_ptr (), ac as Cardinal);
	}

      n_children += 1;
      i += 1;
    }

  /* invisible separator button */
  ac = 0;
  XtSetArg! (al[ac as usize], XmNmappedWhenManaged.as_ptr (), FALSE as XtPointer);
  ac += 1;
  children[n_children as usize] = XmCreateLabel (row, c"separator_button".as_ptr ()
						      as *mut c_char,
						 al.as_mut_ptr (), ac as Cardinal);
  n_children += 1;

  i = 0;
  while i < right_buttons
    {
      let mut button_name: [c_char; 16] = [0; 16];
      sprintf (button_name.as_mut_ptr (), c"button%d".as_ptr (), left_buttons + i + 1);
      ac = 0;
      XtSetArg! (al[ac as usize], XmNmarginWidth.as_ptr (), 10 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNnavigationType.as_ptr (), XmTAB_GROUP as XtPointer);
      ac += 1;
      children[n_children as usize] = XmCreatePushButton (row, button_name.as_mut_ptr (),
							  al.as_mut_ptr (), ac as Cardinal);
      XtAddEventHandler (children[n_children as usize], KeyPressMask, False,
			 Some (dialog_key_cb), result as XtPointer);

      if button.is_null () { button = children[n_children as usize]; }
      n_children += 1;
      i += 1;
    }

  XtManageChildren (children.as_mut_ptr (), n_children as Cardinal);

  ac = 0;
  XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (), row as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 0 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 0 as XtPointer);
  ac += 1;
  separator = XmCreateSeparator (form, c"".as_ptr () as *mut c_char, al.as_mut_ptr (),
				 ac as Cardinal);

  ac = 0;
  XtSetArg! (al[ac as usize], XmNlabelType.as_ptr (), XmPIXMAP as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  icon = XmCreateLabel (form, icon_name, al.as_mut_ptr (), ac as Cardinal);

  ac = 0;
  XtSetArg! (al[ac as usize], XmNmappedWhenManaged.as_ptr (), FALSE as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopOffset.as_ptr (), 6 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopWidget.as_ptr (), icon as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 6 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (), separator as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
  ac += 1;
  icon_separator = XmCreateLabel (form, c"".as_ptr () as *mut c_char, al.as_mut_ptr (),
				  ac as Cardinal);

  if text_input_slot != 0
    {
      ac = 0;
      XtSetArg! (al[ac as usize], XmNcolumns.as_ptr (), 50 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (), separator as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftWidget.as_ptr (), icon as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      value = XmCreateTextField (form, c"value".as_ptr () as *mut c_char, al.as_mut_ptr (),
				 ac as Cardinal);
    }
  else if radio_box != 0
    {
      let mut radio_butt: Widget;
      ac = 0;
      XtSetArg! (al[ac as usize], XmNmarginWidth.as_ptr (), 0 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNmarginHeight.as_ptr (), 0 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNspacing.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNalignment.as_ptr (), XmALIGNMENT_CENTER as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNorientation.as_ptr (), XmHORIZONTAL as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (), separator as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftWidget.as_ptr (), icon as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      value = XmCreateRadioBox (form, c"radiobutton1".as_ptr () as *mut c_char,
				al.as_mut_ptr (), ac as Cardinal);
      ac = 0;
      i = 0;
      radio_butt = XmCreateToggleButtonGadget (value, c"radio1".as_ptr () as *mut c_char,
					       al.as_mut_ptr (), ac as Cardinal);
      children[i as usize] = radio_butt; i += 1;
      radio_butt = XmCreateToggleButtonGadget (value, c"radio2".as_ptr () as *mut c_char,
					       al.as_mut_ptr (), ac as Cardinal);
      children[i as usize] = radio_butt; i += 1;
      radio_butt = XmCreateToggleButtonGadget (value, c"radio3".as_ptr () as *mut c_char,
					       al.as_mut_ptr (), ac as Cardinal);
      children[i as usize] = radio_butt; i += 1;
      XtManageChildren (children.as_mut_ptr (), i as Cardinal);
    }
  else if list != 0
    {
      ac = 0;
      XtSetArg! (al[ac as usize], XmNvisibleItemCount.as_ptr (), 5 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_NONE as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (), separator as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNleftWidget.as_ptr (), icon as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
      ac += 1;
      XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 13 as XtPointer);
      ac += 1;
      value = XmCreateScrolledList (form, c"list".as_ptr () as *mut c_char, al.as_mut_ptr (),
				    ac as Cardinal);

      /* this is the easiest way I found to have the dble click in the list activate the
	 default button */
      XtAddCallback (value, XmNdefaultActionCallback.as_ptr (), Some (activate_button),
		     button as XtPointer);
    }

  ac = 0;
  XtSetArg! (al[ac as usize], XmNalignment.as_ptr (), XmALIGNMENT_BEGINNING as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNtopOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNbottomWidget.as_ptr (),
	     (if text_input_slot != 0 || radio_box != 0 || list != 0 { value }
	      else { separator }) as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftAttachment.as_ptr (), XmATTACH_WIDGET as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNleftWidget.as_ptr (), icon as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightAttachment.as_ptr (), XmATTACH_FORM as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNrightOffset.as_ptr (), 13 as XtPointer);
  ac += 1;
  message_label = XmCreateLabel (form, c"message".as_ptr () as *mut c_char, al.as_mut_ptr (),
				 ac as Cardinal);

  if list != 0
    { XtManageChild (value); }

  i = 0;
  children[i as usize] = row; i += 1;
  children[i as usize] = separator; i += 1;
  if text_input_slot != 0 || radio_box != 0
    {
      children[i as usize] = value; i += 1;
    }
  children[i as usize] = message_label; i += 1;
  children[i as usize] = icon; i += 1;
  children[i as usize] = icon_separator; i += 1;
  XtManageChildren (children.as_mut_ptr (), i as Cardinal);

  if text_input_slot != 0 || list != 0
    {
      XtInstallAccelerators (value, button);
      XtSetKeyboardFocus (result, value);
    }
  else
    {
      XtInstallAccelerators (form, button);
      XtSetKeyboardFocus (result, button);
    }

  return result;
}

/* C 1295-1330: `find_matching_instance` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn find_matching_instance (instance: *mut widget_instance) -> *mut destroyed_instance
{
  let mut cur: *mut destroyed_instance;
  let mut prev: *mut destroyed_instance;
  let r#type: *mut c_char = (*(*instance).info).r#type;
  let name: *mut c_char = (*(*instance).info).name;

  prev = NULL as *mut destroyed_instance;
  cur = all_destroyed_instances;
  while !cur.is_null ()
    {
      if strcmp ((*cur).name, name) == 0
	  && strcmp ((*cur).r#type, r#type) == 0
	  && (*cur).parent == (*instance).parent
	  && (*cur).pop_up_p == (*instance).pop_up_p
	{
	  if !prev.is_null ()
	    { (*prev).next = (*cur).next; }
	  else
	    { all_destroyed_instances = (*cur).next; }
	  return cur;
	}
      /* do some cleanup */
      else if (*cur).widget.is_null ()
	{
	  if !prev.is_null ()
	    { (*prev).next = (*cur).next; }
	  else
	    { all_destroyed_instances = (*cur).next; }
	  free_destroyed_instance (cur);
	  cur = if !prev.is_null () { prev } else { all_destroyed_instances };
	}
      prev = cur;
      cur = (*cur).next;
    }
  return NULL as *mut destroyed_instance;
}

/* C 1332-1339: `mark_dead_instance_destroyed` (static; an `XtCallbackProc` at C 1689).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn mark_dead_instance_destroyed (_widget: Widget, closure: XtPointer,
                                                   _call_data: XtPointer)
{
  let instance: *mut destroyed_instance = closure as *mut destroyed_instance;
  (*instance).widget = NULL as Widget;
}

/* C 1341-1375: `recenter_widget` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn recenter_widget (widget: Widget)
{
  let parent: Widget = XtParent (widget);
  let screen: *mut Screen = XtScreen (widget);
  let screen_width: Dimension = WidthOfScreen (screen) as Dimension;
  let screen_height: Dimension = HeightOfScreen (screen) as Dimension;
  let mut parent_width: Dimension = 0;
  let mut parent_height: Dimension = 0;
  let mut child_width: Dimension = 0;
  let mut child_height: Dimension = 0;
  let mut x: Position;
  let mut y: Position;

  XtVaGetValues (widget, XtNwidth.as_ptr (), &raw mut child_width, XtNheight.as_ptr (),
		 &raw mut child_height, NULL);
  XtVaGetValues (parent, XtNwidth.as_ptr (), &raw mut parent_width, XtNheight.as_ptr (),
		 &raw mut parent_height, NULL);

  x = ((parent_width as Position) - (child_width as Position)) / 2;
  y = ((parent_height as Position) - (child_height as Position)) / 2;

  XtTranslateCoords (parent, x, y, &raw mut x, &raw mut y);

  if x as c_int + child_width as c_int > screen_width as c_int
    { x = (screen_width - child_width) as Position; }
  if x < 0
    { x = 0; }

  if y as c_int + child_height as c_int > screen_height as c_int
    { y = (screen_height - child_height) as Position; }
  if y < 0
    { y = 0; }

  XtVaSetValues (widget, XtNx.as_ptr (), x as c_int, XtNy.as_ptr (),
		 y as c_int, NULL);
}

/* C 1377-1411: `recycle_instance` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn recycle_instance (instance: *mut destroyed_instance) -> Widget
{
  let mut widget: Widget = (*instance).widget;

  /* widget is NULL if the parent was destroyed. */
  if !widget.is_null ()
    {
      let mut focus: Widget;
      let mut separator: Widget;

      /* Remove the destroy callback as the instance is not in the list anymore */
      XtRemoveCallback ((*instance).parent, XtNdestroyCallback.as_ptr (),
			Some (mark_dead_instance_destroyed), instance as XtPointer);

      /* Give the focus to the initial item */
      focus = XtNameToWidget (widget, c"*value".as_ptr ());
      if focus.is_null ()
	{ focus = XtNameToWidget (widget, c"*button1".as_ptr ()); }
      if !focus.is_null ()
	{ XtSetKeyboardFocus (widget, focus); }

      /* shrink the separator label back to their original size */
      separator = XtNameToWidget (widget, c"*separator_button".as_ptr ());
      if !separator.is_null ()
	{ XtVaSetValues (separator, XtNwidth.as_ptr (), 5, XtNheight.as_ptr (), 5,
			 NULL); }

      /* Center the dialog in its parent */
      recenter_widget (widget);
    }
  free_destroyed_instance (instance);
  return widget;
}

/* C 1413-1488: `xm_create_dialog` (declared in `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_create_dialog (instance: *mut widget_instance) -> Widget
{
  let name: *mut c_char = (*(*instance).info).r#type;
  let parent: Widget = (*instance).parent;
  let mut widget: Widget;
  let pop_up_p: Boolean = (*instance).pop_up_p;
  let mut shell_name: *mut c_char = 0 as *mut c_char;
  let mut icon_name: *mut c_char = 0 as *mut c_char;
  let mut text_input_slot: Boolean = False;
  let mut radio_box: Boolean = False;
  let mut list: Boolean = False;
  let mut total_buttons: c_int;
  let mut left_buttons: c_int = 0;
  let mut right_buttons: c_int = 1;
  let mut dead_one: *mut destroyed_instance;

  /* try to find a widget to recycle */
  dead_one = find_matching_instance (instance);
  if !dead_one.is_null ()
    {
      let recycled_widget: Widget = recycle_instance (dead_one);
      if !recycled_widget.is_null ()
	{ return recycled_widget; }
    }

  /* C's `switch (name [0])` has five shared bodies and no `default:`; Rust needs an exhaustive
     `match`, so the port adds the `_ => { }` arm that leaves the five locals exactly as C's
     fall-through does (PORTING.md 2.2; the `lwlib-Xaw.rs` `xaw_create_dialog` reading of the
     same shape, and docs/ports/macro-body-notes.tsv's `xm_create_dialog` row).  */
  match *name as u8
    {
      b'E' | b'e' =>
	{
	  icon_name = c"dbox-error".as_ptr () as *mut c_char;
	  shell_name = c"Error".as_ptr () as *mut c_char;
	}

      b'I' | b'i' =>
	{
	  icon_name = c"dbox-info".as_ptr () as *mut c_char;
	  shell_name = c"Information".as_ptr () as *mut c_char;
	}

      b'L' | b'l' =>
	{
	  list = True;
	  icon_name = c"dbox-question".as_ptr () as *mut c_char;
	  shell_name = c"Prompt".as_ptr () as *mut c_char;
	}

      b'P' | b'p' =>
	{
	  text_input_slot = True;
	  icon_name = c"dbox-question".as_ptr () as *mut c_char;
	  shell_name = c"Prompt".as_ptr () as *mut c_char;
	}

      b'Q' | b'q' =>
	{
	  icon_name = c"dbox-question".as_ptr () as *mut c_char;
	  shell_name = c"Question".as_ptr () as *mut c_char;
	}

      _ => { }
    }

  total_buttons = (*name.offset (1) as c_int) - (b'0' as c_int);

  if *name.offset (3) == b'T' as c_char || *name.offset (3) == b't' as c_char
    {
      text_input_slot = False;
      radio_box = True;
    }
  else if *name.offset (3) != 0
    { right_buttons = (*name.offset (4) as c_int) - (b'0' as c_int); }

  left_buttons = total_buttons - right_buttons;

  widget = make_dialog (name, parent, pop_up_p, shell_name, icon_name, text_input_slot,
			radio_box, list, left_buttons, right_buttons);

  XtAddCallback (widget, XmNpopdownCallback.as_ptr (), Some (xm_nosel_callback),
		 instance as XtPointer);

  return widget;
}

/* C 1490-1502: `Create a menu bar.  We turn off the f10 key because we have not yet managed to
   make it work right in Motif.` and `make_menubar` (static; the table's
   `widget_creation_function`).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn make_menubar (instance: *mut widget_instance) -> Widget
{
  let mut al: [Arg; 3] = [core::mem::zeroed (); 3];
  let mut ac: c_int;

  ac = 0;
  XtSetArg! (al[ac as usize], XmNmenuAccelerator.as_ptr (), 0 as XtPointer);
  ac += 1;
  return XmCreateMenuBar ((*instance).parent, (*(*instance).info).name, al.as_mut_ptr (),
			  ac as Cardinal);
}

/* C 1504-1511: `remove_grabs` (static; the `XtCallbackProc` of C 1523).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn remove_grabs (_shell: Widget, closure: XtPointer, _call_data: XtPointer)
{
  let menu: Widget = closure as Widget;
  XmRemoveFromPostFromList (menu, XtParent (XtParent (menu)));
}

/* C 1513-1527: `make_popup_menu` (static; the table's `widget_creation_function`).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn make_popup_menu (instance: *mut widget_instance) -> Widget
{
  let parent: Widget = (*instance).parent;
  let parent_window: Window = (*parent).core.window;
  let mut result: Widget;

  /* sets the parent window to 0 to fool Motif into not generating a grab */
  (*parent).core.window = 0;
  result = XmCreatePopupMenu (parent, (*(*instance).info).name, core::ptr::null_mut (), 0);
  XtAddCallback (XtParent (result), XmNpopdownCallback.as_ptr (), Some (remove_grabs),
		 result as XtPointer);
  (*parent).core.window = parent_window;
  return result;
}

/* C 1529-1542: `make_main` (static; the table's `widget_creation_function`).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn make_main (instance: *mut widget_instance) -> Widget
{
  let parent: Widget = (*instance).parent;
  let mut result: Widget;
  let mut al: [Arg; 2] = [core::mem::zeroed (); 2];
  let mut ac: c_int;

  ac = 0;
  XtSetArg! (al[ac as usize], XtNborderWidth.as_ptr (), 0 as XtPointer);
  ac += 1;
  XtSetArg! (al[ac as usize], XmNspacing.as_ptr (), 0 as XtPointer);
  ac += 1;
  result = XmCreateMainWindow (parent, (*(*instance).info).name, al.as_mut_ptr (),
			       ac as Cardinal);
  return result;
}

/* C 1544-1644: `Table of functions to create widgets` (C 1544) and the `#ifdef ENERGIZE` arm -
   the XDesigner-generated creators, the `widget_maker` typedef and the ten wrappers.  */
#[cfg (feature = "energize")]
pub type widget_maker = Option<unsafe extern "C" fn (parent: Widget) -> Widget>;

/* C 1550-1559: `interface with the XDesigner generated functions`.  */
#[cfg (feature = "energize")]
unsafe extern "C" {
  pub fn create_project_p_sheet (parent: Widget) -> Widget;
  pub fn create_debugger_p_sheet (parent: Widget) -> Widget;
  pub fn create_breaklist_p_sheet (parent: Widget) -> Widget;
  pub fn create_le_browser_p_sheet (parent: Widget) -> Widget;
  pub fn create_class_browser_p_sheet (parent: Widget) -> Widget;
  pub fn create_call_browser_p_sheet (parent: Widget) -> Widget;
  pub fn create_build_dialog (parent: Widget) -> Widget;
  pub fn create_editmode_dialog (parent: Widget) -> Widget;
  pub fn create_search_dialog (parent: Widget) -> Widget;
  pub fn create_project_display_dialog (parent: Widget) -> Widget;
}

/* C 1561-1582: `make_one` (static).  */
#[cfg (feature = "energize")]
unsafe extern "C" fn make_one (instance: *mut widget_instance, fn_: widget_maker) -> Widget
{
  let mut result: Widget;
  let mut al: [Arg; 64] = [core::mem::zeroed (); 64];
  let mut ac: c_int = 0;

  if (*instance).pop_up_p != 0
    {
      XtSetArg! (al[ac as usize], XmNallowShellResize.as_ptr (), TRUE as XtPointer);
      ac += 1;
      result = XmCreateDialogShell ((*instance).parent, c"dialog".as_ptr () as *mut c_char,
				    core::ptr::null_mut (), 0);
      XtAddCallback (result, XmNpopdownCallback.as_ptr (), Some (xm_nosel_callback),
		     instance as XtPointer);
      (fn_.unwrap ()) (result);
    }
  else
    {
      result = (fn_.unwrap ()) ((*instance).parent);
      XtRealizeWidget (result);
    }
  return result;
}

/* C 1584-1642: the ten wrappers.  */
#[cfg (feature = "energize")]
unsafe extern "C" fn make_project_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_project_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_debugger_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_debugger_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_breaklist_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_breaklist_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_le_browser_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_le_browser_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_class_browser_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_class_browser_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_call_browser_p_sheet (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_call_browser_p_sheet)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_build_dialog (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_build_dialog)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_editmode_dialog (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_editmode_dialog)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_search_dialog (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_search_dialog)); }
#[cfg (feature = "energize")]
unsafe extern "C" fn make_project_display_dialog (instance: *mut widget_instance) -> Widget
{ return make_one (instance, Some (create_project_display_dialog)); }

/* C 1646-1665: `xm_creation_table`.  C's `#ifdef ENERGIZE` pair sits *inside* the initializer,
   and Rust cannot put an attribute on an array element, so the two arms are the two gated
   definitions below - exactly one of them exists in any configuration (P-86 clause 1).  */
#[cfg (all (feature = "use_motif", not (feature = "energize")))]
#[unsafe(no_mangle)]
pub static mut xm_creation_table: [widget_creation_entry; 4] =
  [widget_creation_entry { r#type: c"menubar".as_ptr () as *mut c_char,
			   function: Some (make_menubar) },
   widget_creation_entry { r#type: c"popup".as_ptr () as *mut c_char,
			   function: Some (make_popup_menu) },
   widget_creation_entry { r#type: c"main".as_ptr () as *mut c_char,
			   function: Some (make_main) },
   widget_creation_entry { r#type: core::ptr::null_mut (), function: None }];
#[cfg (all (feature = "use_motif", feature = "energize"))]
#[unsafe(no_mangle)]
pub static mut xm_creation_table: [widget_creation_entry; 14] =
  [widget_creation_entry { r#type: c"menubar".as_ptr () as *mut c_char,
			   function: Some (make_menubar) },
   widget_creation_entry { r#type: c"popup".as_ptr () as *mut c_char,
			   function: Some (make_popup_menu) },
   widget_creation_entry { r#type: c"main".as_ptr () as *mut c_char,
			   function: Some (make_main) },
   widget_creation_entry { r#type: c"project_p_sheet".as_ptr () as *mut c_char,
			   function: Some (make_project_p_sheet) },
   widget_creation_entry { r#type: c"debugger_p_sheet".as_ptr () as *mut c_char,
			   function: Some (make_debugger_p_sheet) },
   widget_creation_entry { r#type: c"breaklist_psheet".as_ptr () as *mut c_char,
			   function: Some (make_breaklist_p_sheet) },
   widget_creation_entry { r#type: c"leb_psheet".as_ptr () as *mut c_char,
			   function: Some (make_le_browser_p_sheet) },
   widget_creation_entry { r#type: c"class_browser_psheet".as_ptr () as *mut c_char,
			   function: Some (make_class_browser_p_sheet) },
   widget_creation_entry { r#type: c"ctree_browser_psheet".as_ptr () as *mut c_char,
			   function: Some (make_call_browser_p_sheet) },
   widget_creation_entry { r#type: c"build".as_ptr () as *mut c_char,
			   function: Some (make_build_dialog) },
   widget_creation_entry { r#type: c"editmode".as_ptr () as *mut c_char,
			   function: Some (make_editmode_dialog) },
   widget_creation_entry { r#type: c"search".as_ptr () as *mut c_char,
			   function: Some (make_search_dialog) },
   widget_creation_entry { r#type: c"project_display".as_ptr () as *mut c_char,
			   function: Some (make_project_display_dialog) },
   widget_creation_entry { r#type: core::ptr::null_mut (), function: None }];

/* C 1667-1699: `Destruction of instances` and `xm_destroy_instance` (declared in `lwlib-Xm.h`,
   so it keeps C's symbol).  C's `0 &&` guard is carried verbatim: the recycling arm has been
   disabled since 1996 (`Disable the recycling until we can find a way to have the dialog box get
   reasonable layout after we modify its contents.`).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_destroy_instance (instance: *mut widget_instance)
{
  let widget: Widget = (*instance).widget;
  if 0 != 0 && XtClass (widget) == xmDialogShellWidgetClass
    {
      let dead_instance: *mut destroyed_instance =
	make_destroyed_instance ((*(*instance).info).name, (*(*instance).info).r#type,
				 (*instance).widget, (*instance).parent, (*instance).pop_up_p);
      (*dead_instance).next = all_destroyed_instances;
      all_destroyed_instances = dead_instance;
      XtUnmanageChild (first_child ((*instance).widget));
      XFlush (XtDisplay ((*instance).widget));
      XtAddCallback ((*instance).parent, XtNdestroyCallback.as_ptr (),
		     Some (mark_dead_instance_destroyed), dead_instance as XtPointer);
    }
  else
    {
      /* This might not be necessary now that the nosel is attached to popdown instead of
	 destroy, but it can't hurt. */
      XtRemoveCallback ((*instance).widget, XtNdestroyCallback.as_ptr (),
			Some (xm_nosel_callback), instance as XtPointer);
      XtDestroyWidget ((*instance).widget);
    }
}

/* C 1701-1748: `popup utility` and `xm_popup_menu` (declared in `lwlib-Xm.h`, so it keeps C's
   symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_popup_menu (widget: Widget, mut event: *mut XEvent)
{
  let mut dummy: XButtonPressedEvent = core::mem::zeroed ();

  if event.is_null ()
    {
      dummy.r#type = ButtonPress;
      dummy.serial = 0;
      dummy.send_event = 0;
      dummy.display = XtDisplay (widget);
      dummy.window = XtWindow (XtParent (widget));
      dummy.time = 0;
      dummy.button = 0;
      XQueryPointer (dummy.display, dummy.window, &raw mut dummy.root, &raw mut dummy.subwindow,
		     &raw mut dummy.x_root, &raw mut dummy.y_root, &raw mut dummy.x,
		     &raw mut dummy.y, &raw mut dummy.state);
      event = (&raw mut dummy) as *mut XEvent;
    }

  if (*event).r#type == ButtonPress || (*event).r#type == ButtonRelease
    {
      /* Setting the menuPost resource only required by Motif 1.1 and LessTif 0.84 and earlier.
	 With later versions of LessTif, setting menuPost is unnecessary and may cause problems,
	 so don't do it.  */
      #[cfg (feature = "xm_version_lt_1002_or_lesstif_lt_84")]
      {
	/* This is so totally ridiculous: there's NO WAY to tell Motif that *any* button can
	   select a menu item.  Only one button can have that honor.  */

	let mut trans: *mut c_char = 0 as *mut c_char;
	if ((*event).xbutton.state & Button5Mask) != 0
	  { trans = c"<Btn5Down>".as_ptr () as *mut c_char; }
	else if ((*event).xbutton.state & Button4Mask) != 0
	  { trans = c"<Btn4Down>".as_ptr () as *mut c_char; }
	else if ((*event).xbutton.state & Button3Mask) != 0
	  { trans = c"<Btn3Down>".as_ptr () as *mut c_char; }
	else if ((*event).xbutton.state & Button2Mask) != 0
	  { trans = c"<Btn2Down>".as_ptr () as *mut c_char; }
	else if ((*event).xbutton.state & Button1Mask) != 0
	  { trans = c"<Btn1Down>".as_ptr () as *mut c_char; }
	if !trans.is_null ()
	  { XtVaSetValues (widget, XmNmenuPost.as_ptr (), trans, NULL); }
      }

      XmMenuPosition (widget, event as *mut XButtonPressedEvent);
    }

  XtManageChild (widget);
}

/* C 1750-1757: `set_min_dialog_size` (static).  */
#[cfg (feature = "use_motif")]
unsafe fn set_min_dialog_size (w: Widget)
{
  let mut width: c_short = 0;
  let mut height: c_short = 0;
  XtVaGetValues (w, XmNwidth.as_ptr (), &raw mut width, XmNheight.as_ptr (), &raw mut height,
		 NULL);
  XtVaSetValues (w, XmNminWidth.as_ptr (), width as c_int, XmNminHeight.as_ptr (),
		 height as c_int, NULL);
}

/* C 1759-1783: `xm_pop_instance` (declared in `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_pop_instance (instance: *mut widget_instance, up: Boolean)
{
  let widget: Widget = (*instance).widget;

  if XtClass (widget) == xmDialogShellWidgetClass
    {
      let widget_to_manage: Widget = first_child (widget);
      if up != 0
	{
	  XtManageChild (widget_to_manage);
	  set_min_dialog_size (widget);
	  XtSetKeyboardFocus ((*instance).parent, widget);
	}
      else
	{ XtUnmanageChild (widget_to_manage); }
    }
  else
    {
      if up != 0
	{ XtManageChild (widget); }
      else
	{ XtUnmanageChild (widget); }
    }
}

/* C 1785-1842: `motif callback` and `do_call` (static).  C's `switch (type)` has a `default:`
   arm (`emacs_abort ()`) and four constant arms, so the port's `match` is C's own control flow
   with no added arm; the `switch`/`case`/`break` tokens are the macro-body note
   (docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "use_motif")]
unsafe fn do_call (widget: Widget, closure: XtPointer, r#type: do_call_type)
{
  let mut al: [Arg; 256] = [core::mem::zeroed (); 256];
  let mut ac: c_int;
  let mut user_data: XtPointer;
  let instance: *mut widget_instance = closure as *mut widget_instance;
  let mut wv: *mut widget_value;
  let mut instance_widget: Widget;
  let id: LWLIB_ID;

  if instance.is_null ()
    { return; }
  if (*widget).core.being_destroyed != 0
    { return; }

  instance_widget = (*instance).widget;
  if instance_widget.is_null ()
    { return; }

  id = (*(*instance).info).id;
  ac = 0;
  user_data = NULL;
  XtSetArg! (al[ac as usize], XmNuserData.as_ptr (), &raw mut user_data);
  ac += 1;
  XtGetValues (widget, al.as_mut_ptr (), ac as Cardinal);
  wv = user_data as *mut widget_value;

  match r#type
    {
      pre_activate =>
	{
	  if (*(*instance).info).pre_activate_cb.is_some ()
	    { ((*(*instance).info).pre_activate_cb.unwrap ()) (widget, id,
	      if !wv.is_null () { (*wv).call_data } else { NULL }); }
	}

      selection =>
	{
	  if (*(*instance).info).selection_cb.is_some ()
	    { ((*(*instance).info).selection_cb.unwrap ()) (widget, id,
	      if !wv.is_null () { (*wv).call_data } else { NULL }); }
	}

      no_selection =>
	{
	  if (*(*instance).info).selection_cb.is_some ()
	    { ((*(*instance).info).selection_cb.unwrap ()) (widget, id,
	      (-1 as c_int) as XtPointer); }
	}

      post_activate =>
	{
	  if (*(*instance).info).post_activate_cb.is_some ()
	    { ((*(*instance).info).post_activate_cb.unwrap ()) (widget, id,
	      if !wv.is_null () { (*wv).call_data } else { NULL }); }
	}

      _ => { emacs_abort (); }
    }
}

/* C 1844-1861: `xm_internal_update_other_instances`'s comment - like
   `lw_internal_update_other_instances` except that it does not do anything if its shell parent
   is not managed; this is to protect `lw_internal_update_other_instances` from dereferencing
   freed memory if the widget was ``destroyed'' by caching it in the `all_destroyed_instances`
   list - and the static itself (an `XtCallbackProc` at C 820/833).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_internal_update_other_instances (widget: Widget, closure: XtPointer,
                                                         call_data: XtPointer)
{
  let mut parent: Widget;
  parent = widget;
  while !parent.is_null ()
    {
      if XtIsShell (parent) != 0
	{ break; }
      else if XtIsManaged (parent) == 0
	{ return; }
      parent = XtParent (parent);
    }
  lw_internal_update_other_instances (widget, closure, call_data);
}

/* C 1863-1870: `xm_generic_callback` (static; the `XtCallbackProc` of C 374, 396, 418, 434, 596,
   817, 830).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_generic_callback (widget: Widget, closure: XtPointer,
                                          call_data: XtPointer)
{
  lw_internal_update_other_instances (widget, closure, call_data);
  do_call (widget, closure, selection);
}

/* C 1872-1887: `xm_nosel_callback` (static; the `XtCallbackProc` of C 1484, 1572, 1696, 1764).
   This callback is only called when a dialog box is dismissed with the wm's destroy button
   (WM_DELETE_WINDOW.)  We want the dialog box to be destroyed in that case, not just unmapped,
   so that it releases its keyboard grabs.  But there are problems with running our callbacks
   while the widget is in the process of being destroyed, so we set XmNdeleteResponse to XmUNMAP
   instead of XmDESTROY and then destroy it ourself after having run the callback.  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_nosel_callback (widget: Widget, closure: XtPointer,
                                        _call_data: XtPointer)
{
  do_call (widget, closure, no_selection);
  XtDestroyWidget (widget);
}

/* C 1889-1903: `xm_pull_down_callback` (static; the `XtCallbackProc` of C 406, 609, 716).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_pull_down_callback (widget: Widget, closure: XtPointer,
                                            _call_data: XtPointer)
{
  let parent: Widget = XtParent (widget);

  if XmIsRowColumn (parent) != 0
    {
      let mut r#type: c_uchar = 0xff;
      XtVaGetValues (parent, XmNrowColumnType.as_ptr (), &raw mut r#type,
		     NULL);
      if r#type == XmMENU_BAR
	{ do_call (widget, closure, pre_activate); }
    }
}

/* C 1905-1922: `XmNpopdownCallback for MenuShell widgets.  WIDGET is the MenuShell, CLOSURE is
   a pointer to the widget_instance of the shell.  Note that this callback is called for each
   cascade button in a menu, whether or not its submenu is visible.` and
   `xm_pop_down_callback` (static; the `XtCallbackProc` of C 518).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_pop_down_callback (widget: Widget, closure: XtPointer,
                                           _call_data: XtPointer)
{
  let instance: *mut widget_instance = closure as *mut widget_instance;

  if ((*instance).pop_up_p == 0 && XtParent (widget) == (*instance).widget)
      || XtParent (widget) == (*instance).parent
    { do_call (widget, closure, post_activate); }
}

/* C 1924-1934: `xm_pop_up_callback` (static; the `XtCallbackProc` of C 520).  */
#[cfg (feature = "use_motif")]
unsafe extern "C" fn xm_pop_up_callback (widget: Widget, closure: XtPointer,
                                         _call_data: XtPointer)
{
  let instance: *mut widget_instance = closure as *mut widget_instance;

  if ((*instance).pop_up_p == 0 && XtParent (widget) == (*instance).widget)
      || XtParent (widget) == (*instance).parent
    { do_call (widget, closure, pre_activate); }
}

/* C 1936-1943: `set the keyboard focus` and `xm_set_keyboard_focus` (declared in
   `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_set_keyboard_focus (parent: Widget, w: Widget)
{
  XmProcessTraversal (w, 0);
  XtSetKeyboardFocus (parent, w);
}

/* C 1945-1957: `Motif hack to set the main window areas.` and `xm_set_main_areas` (declared in
   `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_set_main_areas (parent: Widget, menubar: Widget, work_area: Widget)
{
  XmMainWindowSetAreas (parent,
			menubar,	/* menubar (maybe 0) */
			0 as Widget,	/* command area (psheets) */
			0 as Widget,	/* horizontal scroll */
			0 as Widget,	/* vertical scroll */
			work_area);	/* work area */
}

/* C 1959-1964: `Motif hack to control resizing on the menubar.` and `xm_manage_resizing`
   (declared in `lwlib-Xm.h`, so it keeps C's symbol).  */
#[cfg (feature = "use_motif")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xm_manage_resizing (w: Widget, flag: Boolean)
{
  XtVaSetValues (w, XtNallowShellResize.as_ptr (), flag, NULL);
}

/* lisp.h 5555: `extern void dupstring (char **, char const *);` - C 956 calls it and the
   `lisp.rs` mirror does not carry it (it is `src/alloc.c`'s definition), so the unit declares
   it here, the `src/term.rs` 384-385 reading of the same entry.  */
#[cfg (feature = "use_motif")]
unsafe extern "C" {
  pub fn dupstring (ptr: *mut *mut c_char, string: *const c_char);
}
