// femacs: port of lwlib/lwlib-Xaw.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the
// `static` helpers whose address Xt takes are `unsafe extern "C" fn` (PORTING.md 1.4).
//
// **The file gate is `feature = "use_lucid"`** (P-86 clause 8, the build's condition rather than
// one the C text spells): `lwlib/Makefile.in` 52 lists `lwlib-Xaw.o` in
// `LUCID_OBJS = lwlib-Xlw.o xlwmenu.o lwlib-Xaw.o`, 56 selects that list as `TOOLKIT_OBJS` and 58
// puts it into `OBJS`, with `configure.ac` 7231's `AC_DEFINE ([USE_LUCID], [1], [Define to 1 if
// using the Lucid X toolkit.])` as the configure condition.  `USE_LUCID`, `USE_X_TOOLKIT` and
// `HAVE_X11` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so the frozen
// configuration compiles none of this file and the port's frozen object is the empty one.  This is
// the same gate its sibling `lwlib/lwlib-Xlw.rs` carries.
//
// **This file is `text-only`** (P-86 clause 3(2)): this machine has no Xt, Xaw or Xft header
// (`/opt/homebrew/include/X11` carries `xorgproto` 2025.1's Xlib half - `<X11/Xlib.h>`,
// `<X11/Xatom.h>` - and nothing of `<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`,
// `<X11/Shell.h>`, `<X11/StringDefs.h>` or `<X11/Xaw{3d}/…>`), so the one Xlib record this file
// reads element by element (`XColor`) was **measured with clang** against that header and every
// Xt/Xaw/Xft width, offset, enum value and resource name is C's spelling with the value
// **UNVERIFIED** (the `include/gtk.rs` `placeholder, not GDK's value` practice, status 3.481).
//
// The three sub-conditions the C text spells keep their own gates (P-86 clause 1): C 36-50's
// `HAVE_XAW3D` pair is `feature = "have_xaw3d"`, C 54's `#if defined USE_CAIRO || defined
// HAVE_XFT` is `feature = "use_cairo_or_have_xft"` (the derived name the `include/xlwmenuP.rs`
// mirror already uses for that condition) and the C 55 `#ifdef USE_CAIRO` inside it is
// `feature = "use_cairo"`.
//
// The `<X11/*>`, `<X11/Xaw*>` and `<X11/Xft/*>` surface stands in this unit (P-88 clause 1: no
// mirror in this tree carries any of those headers, and `lwlib/include/lwlib.rs` names `Widget`,
// `Boolean` and `XEvent` and leaves them to the unit that includes it).  C 36-50's two arms name
// the same six Athena headers under two directories, so the port has one declaration of the six
// classes they carry and C's own `#include` pair stands as the comment below (the `src/xfns.rs`
// 125-129 reading of the same pair).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, unused_mut, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, clippy::all)]

include!("../src/include/config.rs");            // C 21: `#include <config.h>`
#[cfg (feature = "use_lucid")]
include!("../lib/include/stdcountof.rs");         // C 23: `#include <stdcountof.h>` (`countof`)
#[cfg (feature = "use_lucid")]
include!("../lib/include/stdio.rs");             // C 24: `#include <stdio.h>` (`sprintf`)
/* C 25's `#include <setjmp.h>` has no edge of its own: nothing in this file names a type that
   header declares, and the mirror that carries them is `lisp.rs`, which C 27 includes next.  */
#[cfg (feature = "use_lucid")]
include!("../src/include/lisp.rs");              // C 27: `#include <lisp.h>`
#[cfg (feature = "use_lucid")]
include!("../lib/include/string.rs");            // lisp.h 30: `#include <string.h>`
                                                 // (`strlen`/`strchr`/`strcmp`)
#[cfg (feature = "use_lucid")]
include!("include/lwlib-Xaw.rs");                // C 29: `#include "lwlib-Xaw.h"`
/* C 31-34's `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/CoreP.h>` and `<X11/Shell.h>`, C
   36-50's `#ifdef HAVE_XAW3D` pair (`<X11/Xaw3d/{Scrollbar,Paned,Dialog,Form,Command,Label}.h>`
   against `<X11/Xaw/{…}.h>`) and C 52's `<X11/Xatom.h>` are the surface declared below: no header
   in this tree carries any of them and this file is their only consumer here (P-88 clause 1).  */
/* C 56's `#include <stdlib.h>` sits inside the `#ifdef USE_CAIRO` arm, yet C 594 calls `calloc`
   from the region both arms share: the `HAVE_XFT` arm reaches the declaration through its own
   header (C 59) instead, so the port includes the mirror for both arms and the ledger records the
   reading (the `## Deviations` entry - the reference's own text does not include this header in
   that arm).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
include!("../lib/include/stdlib.rs");            // C 56: `#include <stdlib.h>` (`calloc`)
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
include!("include/lwlib-utils.rs");              // C 57: `#include "lwlib-utils.h"` (the seven
                                                 // `crxft_*` renamings and `XftFont`/`XftColor`/
                                                 // `XGlyphInfo`/`XftDraw`)
/* C 58-60's `#else /* HAVE_XFT */` arm is `#include <X11/Xft/Xft.h>`: the `USE_CAIRO` arm above
   reaches the same four records and seven entry points through `lwlib-utils.h`'s renamings, and
   this arm needs the header's own, so they are declared under `have_xft` (and only there, so the
   two arms never both define them) in the surface section below.  */

/* ------------------------------------------------------------------------------------------
   `<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`, `<X11/Shell.h>`,
   `<X11/StringDefs.h>`, `<X11/Xatom.h>` and the six Athena headers of C 36-50 (both arms name
   the same names).  **C's spellings, not measurements** except where a reading is named - this
   machine has no Xt/Xaw/Xft header, so every width, offset, enum value and resource-type string
   below is `UNVERIFIED` (P-86 clause 3(2); the readings are in `docs/ports/lwlib-Xaw.md`).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "use_lucid")]
pub type XtPointer = *mut c_void;                     // C: `typedef char *XtPointer;`
#[cfg (feature = "use_lucid")]
pub type String = *mut c_char;                        // C: `typedef char *String;`
#[cfg (feature = "use_lucid")]
pub type Boolean = c_int;                             // C: `typedef int Boolean;`
#[cfg (feature = "use_lucid")]
pub const False: Boolean = 0;                         // C: `#define False 0`
#[cfg (feature = "use_lucid")]
pub const True: Boolean = 1;                          // C: `#define True 1`
/// C: `<stddef.h>`: `#define NULL ((void *) 0)` - the null pointer C 538, 559, 579, 596, 733 and
/// 800 write; the token is kept (the `src/widget.rs` 396-400 spelling of the same macro).
#[cfg (feature = "use_lucid")]
pub const NULL: *mut c_void = core::ptr::null_mut ();
#[cfg (feature = "use_lucid")]
pub type Cardinal = c_uint;                           // C: `typedef unsigned int Cardinal;`
#[cfg (feature = "use_lucid")]
pub type Dimension = c_ushort;                         // C: `typedef unsigned short Dimension;`
#[cfg (feature = "use_lucid")]
pub type Position = c_short;                          // C: `typedef short Position;`
/// C: X.h 66-88: `typedef unsigned long XID;` and `typedef XID Window;`/`typedef XID Pixmap;`/
/// `typedef XID Colormap;` - the three handles this file passes to Xlib.
#[cfg (feature = "use_lucid")]
pub type XID = c_ulong;
#[cfg (feature = "use_lucid")]
pub type Window = XID;
#[cfg (feature = "use_lucid")]
pub type Pixmap = XID;
#[cfg (feature = "use_lucid")]
pub type Colormap = XID;
#[cfg (feature = "use_lucid")]
pub type Atom = c_ulong;                              // C: X.h: `typedef unsigned long Atom;`
#[cfg (feature = "use_lucid")]
pub type Pixel = c_ulong;                             // C: X.h: `typedef unsigned long Pixel;`
/// C: X.h 115: `#define None 0L` - "universal null resource or null atom", which C 458 writes as
/// the `data->p` sentinel.  It lives in a module rather than at this unit's top level: a top-level
/// `None` would shadow Rust's prelude `Option::None` for the whole unit (the
/// `include/xlib.rs` 29-31 `x11_constants` reading of the same macro), and the two functions that
/// write C's `None` import it where they use it (`src/xterm.rs` 1898's shape).
#[cfg (feature = "use_lucid")]
pub mod xaw_x11_constants {
  use core::ffi::c_long;

  pub const None: c_long = 0;
}
/// C: Xatom.h: `#define XA_ATOM ((Atom) 4)` (measured on this machine's xorgproto 2025.1).
#[cfg (feature = "use_lucid")]
pub const XA_ATOM: Atom = 4;
/// C: X.h: `#define PropModeAppend 2` (measured on this machine's xorgproto 2025.1).
#[cfg (feature = "use_lucid")]
pub const PropModeAppend: c_int = 2;
/// C: X.h: `#define ButtonPress 4` - declared beside the events the port names, the
/// `lwlib-Xlw.rs` spelling.
#[cfg (feature = "use_lucid")]
pub const ButtonPress: c_int = 4;
/// C: Xlib.h: `typedef struct _XDisplay Display;`
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Display { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XScreen Screen;` (`Xlib.h`'s `Screen` is the screen record the
/// four `Default*OfScreen` macros read).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Screen { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XVisual Visual;` - C 193's `DefaultVisual` result.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Visual { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct _XFontStruct XFontStruct;` - `XLoadQueryFont`'s result, which C 584
/// only Tests and then frees.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XFontStruct { pub _private: [u8; 0] }
/// C: Xlib.h: `typedef struct { unsigned long pixel; unsigned short red, green, blue; char flags;
/// char pad; } XColor;` - **measured with clang** against this machine's
/// `/opt/homebrew/include/X11/Xlib.h` (xorgproto 2025.1): `sizeof` 16, `pixel` 0, `red` 8,
/// `green` 10, `blue` 12, `flags` 14, `pad` 15.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XColor
{
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}
/// C: Intrinsic.h: `typedef struct _WidgetRec *Widget;` - the record carries the six members this
/// file reads, in C's own order (`parent`, `being_destroyed`, then `x`, `y`, `width`, `height`);
/// the members between them are omitted, so **every offset below is UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec
{
  pub core: CorePart,
}
#[cfg (feature = "use_lucid")]
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
}
#[cfg (feature = "use_lucid")]
pub type Widget = *mut WidgetRec;
#[cfg (feature = "use_lucid")]
pub type WidgetClass = *mut c_void;                   // C: `typedef struct _WidgetClassRec *WidgetClass;`
/// C: Intrinsic.h: `typedef char *XtTranslations;` (`XtParseTranslationTable`'s result, C 538 and
/// 553).
#[cfg (feature = "use_lucid")]
pub type XtTranslations = *mut c_char;
/// C: Intrinsic.h: `typedef struct _XtAppStruct *XtAppContext;` - C 530's handle.
#[cfg (feature = "use_lucid")]
pub type XtAppContext = *mut c_void;
/// C: Intrinsic.h: `typedef void (*XtActionProc) (Widget, XEvent*, String*, Cardinal*);`
#[cfg (feature = "use_lucid")]
pub type XtActionProc =
  Option<unsafe extern "C" fn (Widget, *mut XEvent, *mut String, *mut Cardinal)>;
/// C: Intrinsic.h: `typedef struct { String name; XtActionProc proc; } XtActionsRec;`
#[cfg (feature = "use_lucid")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XtActionsRec
{
  pub name: String,
  pub proc: XtActionProc,
}
/// C: Intrinsic.h: `typedef void (*XtCallbackProc) (Widget, XtPointer, XtPointer);`
#[cfg (feature = "use_lucid")]
pub type XtCallbackProc = Option<unsafe extern "C" fn (Widget, XtPointer, XtPointer)>;
/// C: Intrinsic.h: `typedef struct { String resource_name; String resource_class;
/// String resource_type; Cardinal resource_size; Cardinal resource_offset; String default_type;
/// XtPointer default_addr; } XtResource;` - the seven columns C 576-578 initialize positionally.
#[cfg (feature = "use_lucid")]
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
/// C: XLIB / Xlib.h: `typedef union _XEvent { int type; XAnyEvent xany; XButtonEvent xbutton; … }
/// XEvent;` - every `XEvent *` this file declares (C 399's parameter and the three
/// `XtActionProc`s') is passed on and never dereferenced, so the port carries `type` plus a
/// padding member and nothing else.  The members C does not name are **UNVERIFIED**; the padding
/// is the **measured** size and alignment of this target's XEvent (192 bytes, align 8 - the
/// `[c_ulong; 24]` element carries both, `## Evidence` in docs/ports/lwlib-Xaw.md), the
/// `lwlib-Xlw.rs`/`lwlib-utils.rs` union shape.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent
{
  pub r#type: c_int,
  pub _padding: [c_ulong; 24],
}
/// C: StringDefs.h: the resource names this file passes to `XtVaGetValues`/`XtVaSetValues`/
/// `XtAddCallback`/`XtRemoveAllCallbacks` and the two class strings of the `XtResource` row
/// (C 576-578).  C's spelling is kept and the values are **UNVERIFIED** - they are the names the X
/// resource-manager convention gives those resources, not a reading of that header (the
/// `include/widget.rs` 112-146 treatment of the same header).
#[cfg (feature = "use_lucid")]
pub const XtNbackground: &core::ffi::CStr = c"background";
#[cfg (feature = "use_lucid")]
pub const XtNforeground: &core::ffi::CStr = c"foreground";
#[cfg (feature = "use_lucid")]
pub const XtNbitmap: &core::ffi::CStr = c"bitmap";
#[cfg (feature = "use_lucid")]
pub const XtNcornerRoundPercent: &core::ffi::CStr = c"cornerRoundPercent";
#[cfg (feature = "use_lucid")]
pub const XtNlabel: &core::ffi::CStr = c"label";
#[cfg (feature = "use_lucid")]
pub const XtNborderWidth: &core::ffi::CStr = c"borderWidth";
#[cfg (feature = "use_lucid")]
pub const XtNjustify: &core::ffi::CStr = c"justify";
#[cfg (feature = "use_lucid")]
pub const XtNinternalHeight: &core::ffi::CStr = c"internalHeight";
#[cfg (feature = "use_lucid")]
pub const XtNhighlightThickness: &core::ffi::CStr = c"highlightThickness";
#[cfg (feature = "use_lucid")]
pub const XtNcallback: &core::ffi::CStr = c"callback";
#[cfg (feature = "use_lucid")]
pub const XtNtitle: &core::ffi::CStr = c"title";
#[cfg (feature = "use_lucid")]
pub const XtNallowShellResize: &core::ffi::CStr = c"allowShellResize";
#[cfg (feature = "use_lucid")]
pub const XtNgeometry: &core::ffi::CStr = c"geometry";
#[cfg (feature = "use_lucid")]
pub const XtNnumChildren: &core::ffi::CStr = c"numChildren";
#[cfg (feature = "use_lucid")]
pub const XtNchildren: &core::ffi::CStr = c"children";
#[cfg (feature = "use_lucid")]
pub const XtNdestroyCallback: &core::ffi::CStr = c"destroyCallback";
#[cfg (feature = "use_lucid")]
pub const XtNfromHoriz: &core::ffi::CStr = c"fromHoriz";
#[cfg (feature = "use_lucid")]
pub const XtNleft: &core::ffi::CStr = c"left";
#[cfg (feature = "use_lucid")]
pub const XtNright: &core::ffi::CStr = c"right";
#[cfg (feature = "use_lucid")]
pub const XtNtop: &core::ffi::CStr = c"top";
#[cfg (feature = "use_lucid")]
pub const XtNbottom: &core::ffi::CStr = c"bottom";
#[cfg (feature = "use_lucid")]
pub const XtNresizable: &core::ffi::CStr = c"resizable";
#[cfg (feature = "use_lucid")]
pub const XtNhorizDistance: &core::ffi::CStr = c"horizDistance";
#[cfg (feature = "use_lucid")]
pub const XtNx: &core::ffi::CStr = c"x";
#[cfg (feature = "use_lucid")]
pub const XtNy: &core::ffi::CStr = c"y";
/// C: StringDefs.h: `#define XtRString "String"` - the resource *type* C 577 passes twice (the
/// `include/widget.rs` 146-148 spelling of the same name).
#[cfg (feature = "use_lucid")]
pub const XtRString: &core::ffi::CStr = c"String";
/// C: Command.h: `typedef enum { XtJustifyLeft, XtJustifyCenter, XtJustifyRight } XtJustify;` -
/// C 346 passes the middle member; the value is a **placeholder, not Xaw's value** (there is no
/// Xaw header here to read it from).
#[cfg (feature = "use_lucid")]
pub const XtJustifyCenter: c_int = 1;
/// C: Form.h: `typedef enum { XtRubber, XtChainLeft, XtChainRight, XtChainTop, XtChainBottom,
/// XtChainCenter } XtEdgeType;` - C 615-618 and 648-651 pass these four through `XtSetArg`; the
/// values are **placeholders, not Xaw's values**.
#[cfg (feature = "use_lucid")]
pub const XtChainLeft: c_int = 1;
#[cfg (feature = "use_lucid")]
pub const XtChainRight: c_int = 2;
#[cfg (feature = "use_lucid")]
pub const XtChainBottom: c_int = 4;
/// C: Shell.h: `typedef enum { XtGrabNone, XtGrabNonexclusive, XtGrabExclusive } XtGrabKind;` -
/// C 437 passes the middle member; the value is a **placeholder, not Xt's value**.
#[cfg (feature = "use_lucid")]
pub const XtGrabNonexclusive: c_int = 1;
/* C: the six Athena headers of C 36-50 (both arms carry the same names): `Scrollbar.h`'s
   `scrollbarWidgetClass`, `Dialog.h`'s `dialogWidgetClass`, `Command.h`'s `commandWidgetClass`,
   `Label.h`'s `labelWidgetClass`, `Paned.h`'s `panedWidgetClass`, and `<X11/Shell.h>`'s
   `transientShellWidgetClass`/`shellWidgetClass`.  A `///` here is an `unused doc comment`
   (rustdoc generates nothing for an `extern` block), so the citation is a plain comment.  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  pub static mut scrollbarWidgetClass: WidgetClass;
  pub static mut dialogWidgetClass: WidgetClass;
  pub static mut commandWidgetClass: WidgetClass;
  pub static mut labelWidgetClass: WidgetClass;
  pub static mut panedWidgetClass: WidgetClass;
  pub static mut transientShellWidgetClass: WidgetClass;
  pub static mut shellWidgetClass: WidgetClass;
}

/// C: Intrinsic.h: `typedef struct { String name; XtArgVal value; } Arg, *ArgList;` - C's macro
/// `XtSetArg (arg, n, d)` is `((arg).name = (n), (arg).value = (XtArgVal) (d))`, so the port
/// carries `XtArgVal` as `XtPointer` and spells the macro with the same two assignments (the
/// `lwlib-Xlw.rs` 149-165 shape).
#[cfg (feature = "use_lucid")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct Arg
{
  pub name: String,
  pub value: XtPointer,
}
#[cfg (feature = "use_lucid")]
macro_rules! XtSetArg {
  ($arg:expr, $name:expr, $value:expr) => {
    { ($arg).name = ($name) as String; ($arg).value = ($value) as XtPointer; }
  };
}

#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /* C: Intrinsic.h: `Boolean XtIsSubclass (Widget, WidgetClass);`
     C: Intrinsic.h: `void XtVaSetValues (Widget, ...);`
     C: Intrinsic.h: `void XtVaGetValues (Widget, ...);`
     C: Intrinsic.h: `void XtVaGetSubresources (Widget, XtPointer, const char *, const char *,
     XtResourceList, Cardinal, ...);`
     C: Intrinsic.h: `void XtSetValues (Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `void XtSetSensitive (Widget, Boolean);`
     C: Intrinsic.h: `void XtAddCallback (Widget, const char *, XtCallbackProc, XtPointer);`
     C: Intrinsic.h: `void XtRemoveAllCallbacks (Widget, const char *);`
     C: Intrinsic.h: `void XtOverrideTranslations (Widget, XtTranslations);`
     C: Intrinsic.h: `XtTranslations XtParseTranslationTable (const char *);`
     C: Intrinsic.h: `XtAppContext XtWidgetToApplicationContext (Widget);`
     C: Intrinsic.h: `void XtAppAddActions (XtAppContext, XtActionList, Cardinal);` (`XtActionList`
     is `XtActionsRec *`)
     C: Intrinsic.h: `Widget XtCreateWidget (const char *, WidgetClass, Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `Widget XtCreateManagedWidget (const char *, WidgetClass, Widget, ArgList,
     Cardinal);`
     C: Intrinsic.h: `Widget XtCreatePopupShell (const char *, WidgetClass, Widget, ArgList,
     Cardinal);`
     C: Intrinsic.h: `void XtDestroyWidget (Widget);`
     C: Intrinsic.h: `void XtManageChild (Widget);`
     C: Intrinsic.h: `void XtUnmanageChild (Widget);`
     C: Intrinsic.h: `void XtPopup (Widget, XtGrabKind);`
     C: Intrinsic.h: `Widget XtParent (Widget);`
     C: Intrinsic.h: `String XtName (Widget);`
     C: Intrinsic.h: `Display *XtDisplay (Widget);`
     C: Intrinsic.h: `Screen *XtScreen (Widget);`
     C: Intrinsic.h: `Window XtWindow (Widget);`
     C: Intrinsic.h: `Boolean XtIsRealized (Widget);` (a macro in C, the `XtIsShell`
     `lwlib-Xlw.rs` declares)
     C: Xlib.h: `void XQueryColors (Display *, Colormap, XColor *, int);`
     C: Xlib.h: `Pixmap XCreatePixmap (Display *, Drawable, unsigned int, unsigned int,
     unsigned int);`
     C: Xlib.h: `int XFreePixmap (Display *, Pixmap);`
     C: Xlib.h: `Atom XInternAtom (Display *, const char *, Bool);`
     C: Xlib.h: `int XChangeProperty (Display *, Window, Atom, Atom, int, int,
     const unsigned char *, int);`
     C: Xlib.h: `int XScreenNumberOfScreen (Screen *);`
     C: Xlib.h: `XFontStruct *XLoadQueryFont (Display *, const char *);`
     C: Xlib.h: `int XFreeFont (Display *, XFontStruct *);`
     C: Xlib.h: `#define DefaultColormapOfScreen(s) (ScreenOfDisplay (…)->cmap)`
     C: Xlib.h: `#define DefaultVisual(dpy, scr) (…->visual)`
     C: Xlib.h: `#define DefaultDepthOfScreen(s) (ScreenOfDisplay (…)->root_depth)`  */
  pub fn XtIsSubclass (widget: Widget, widget_class: WidgetClass) -> Boolean;
  pub fn XtVaSetValues (widget: Widget, ...);
  pub fn XtVaGetValues (widget: Widget, ...);
  pub fn XtVaGetSubresources (widget: Widget, base: XtPointer, name: *const c_char,
                              class: *const c_char, resources: *mut XtResource,
                              num_resources: Cardinal, ...);
  pub fn XtSetValues (widget: Widget, args: *mut Arg, num_args: Cardinal);
  pub fn XtSetSensitive (widget: Widget, sensitive: Boolean);
  pub fn XtAddCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                        client_data: XtPointer);
  pub fn XtRemoveAllCallbacks (widget: Widget, name: *const c_char);
  pub fn XtOverrideTranslations (widget: Widget, translations: XtTranslations);
  pub fn XtParseTranslationTable (table: *const c_char) -> XtTranslations;
  pub fn XtWidgetToApplicationContext (widget: Widget) -> XtAppContext;
  pub fn XtAppAddActions (app_context: XtAppContext, actions: *mut XtActionsRec,
                          num_actions: Cardinal);
  pub fn XtCreateWidget (name: *const c_char, widget_class: WidgetClass, parent: Widget,
                         args: *mut Arg, num_args: Cardinal) -> Widget;
  pub fn XtCreateManagedWidget (name: *const c_char, widget_class: WidgetClass, parent: Widget,
                                args: *mut Arg, num_args: Cardinal) -> Widget;
  pub fn XtCreatePopupShell (name: *const c_char, widget_class: WidgetClass, parent: Widget,
                             args: *mut Arg, num_args: Cardinal) -> Widget;
  pub fn XtDestroyWidget (widget: Widget);
  pub fn XtManageChild (widget: Widget);
  pub fn XtUnmanageChild (widget: Widget);
  pub fn XtPopup (widget: Widget, grab_kind: c_int);
  pub fn XtParent (widget: Widget) -> Widget;
  pub fn XtName (widget: Widget) -> String;
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  pub fn XtScreen (widget: Widget) -> *mut Screen;
  pub fn XtWindow (widget: Widget) -> Window;
  pub fn XtIsRealized (widget: Widget) -> Boolean;
  pub fn XQueryColors (dpy: *mut Display, cmap: Colormap, colors: *mut XColor, ncolors: c_int);
  pub fn XCreatePixmap (dpy: *mut Display, drawable: c_ulong, width: c_uint, height: c_uint,
                        depth: c_uint) -> Pixmap;
  pub fn XFreePixmap (dpy: *mut Display, pixmap: Pixmap) -> c_int;
  pub fn XInternAtom (dpy: *mut Display, name: *const c_char, only_if_exists: Boolean) -> Atom;
  pub fn XChangeProperty (dpy: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int)
    -> c_int;
  pub fn XScreenNumberOfScreen (screen: *mut Screen) -> c_int;
  pub fn XLoadQueryFont (dpy: *mut Display, name: *const c_char) -> *mut XFontStruct;
  pub fn XFreeFont (dpy: *mut Display, font_struct: *mut XFontStruct) -> c_int;
  pub fn DefaultColormapOfScreen (screen: *mut Screen) -> Colormap;
  pub fn DefaultVisual (dpy: *mut Display, screen_number: c_int) -> *mut Visual;
  pub fn DefaultDepthOfScreen (screen: *mut Screen) -> c_int;
}

/* C 54-73: the `#if defined USE_CAIRO || defined HAVE_XFT` region's four records.  The
   `USE_CAIRO` arm reaches `XftFont`, `XftDraw`, `XftColor` and `XGlyphInfo` through
   `lwlib-utils.h`'s renamings (the mirror included at C 57); the `HAVE_XFT` arm includes
   `<X11/Xft/Xft.h>` itself (C 59), and that header has no mirror here, so its own four records
   and seven entry points are declared for that arm alone - the two arms can never both define
   them.  Every width and offset below is **UNVERIFIED** (no Xft header on this machine).  */

/* C 62-70: `struct widget_xft_data { Widget widget; XftFont *xft_font; XftDraw *xft_draw;
   XftColor xft_fg, xft_bg; int p_width, p_height; Pixmap p; };`  C's tag is the name, and this
   file is the unit that completes it.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct widget_xft_data
{
  pub widget: Widget,
  pub xft_font: *mut XftFont,
  pub xft_draw: *mut XftDraw,
  pub xft_fg: XftColor,
  pub xft_bg: XftColor,
  pub p_width: c_int,
  pub p_height: c_int,
  pub p: Pixmap,
}

/// C: Xft.h: `typedef struct _XftFont { int ascent, descent, height, max_advance_width;
/// FcCharSet *charset; FcPattern *pattern; } XftFont;` - the two members this file reads are
/// `ascent` and `height`; the other two `int`s are carried to keep them where Xft puts them, and
/// the offsets are **UNVERIFIED** (`charset`/`pattern` are opaque pointers, the `FcCharSet`/
/// `FcPattern` records themselves not being named).
#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XftFont
{
  pub ascent: c_int,
  pub descent: c_int,
  pub height: c_int,
  pub max_advance_width: c_int,
  pub charset: *mut c_void,
  pub pattern: *mut c_void,
}
/// C: Xft.h: `typedef struct _XftDraw XftDraw;` - the drawable-holder C 118/129 points at and
/// every `XftDraw*` entry point takes.
#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XftDraw { pub _private: [u8; 0] }
/// C: Xft.h: `typedef struct _XftColor { unsigned long pixel; struct { unsigned short red,
/// green, blue, alpha; } color; } XftColor;` - the same record `include/lwlib-utils.rs` carries
/// for the cairo arm (its nested anonymous record is named `<type>_<member>`, PORTING.md
/// 1.3/P-67); the offsets are **UNVERIFIED**.
#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XftColor
{
  pub pixel: c_ulong,
  pub color: XftColor_color,
}
#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XftColor_color
{
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub alpha: c_ushort,
}
/// C: Xft.h: `typedef struct { unsigned short width, height; short x, y, xOff, yOff; }
/// XGlyphInfo;` - the `#else` arm of that header's `HAVE_XRENDER` test, which is the arm this
/// machine's Xft takes (`XA_ATOM`-class data; the offsets are **UNVERIFIED**).
#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XGlyphInfo
{
  pub width: c_ushort,
  pub height: c_ushort,
  pub x: c_short,
  pub y: c_short,
  pub xOff: c_short,
  pub yOff: c_short,
}
/// C: fontconfig.h: `typedef unsigned char FcChar8;` - the byte type C 195/205 casts a `char *`
/// to.
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
pub type FcChar8 = c_uchar;
/// C: X.h: `typedef XID Drawable;` - the `crxft_draw_create` parameter (C 191-196 passes
/// `data->p`) and the `XCreatePixmap` one.
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
pub type Drawable = XID;
/// C: cairo.h: `typedef struct _cairo cairo_t;` / `typedef struct _cairo_surface cairo_surface_t;`
/// / `typedef struct _cairo_scaled_font cairo_scaled_font_t;` - the three names the
/// `lwlib-utils.h` mirror's renaming declarations spell and leaves to the unit that includes it
/// (its own comment; `lwlib/lwlib-utils.rs` declares the same three for the same reason).
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_t { pub _private: [u8; 0] }
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_surface_t { pub _private: [u8; 0] }
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct cairo_scaled_font_t { pub _private: [u8; 0] }

#[cfg (all (feature = "use_lucid", feature = "have_xft", not (feature = "use_cairo")))]
unsafe extern "C" {
  /* C: Xft.h: `XftFont *XftFontOpenName (Display *, int, const char *);`
     C: Xft.h: `void XftFontClose (Display *, XftFont *);`
     C: Xft.h: `XftDraw *XftDrawCreate (Display *, Drawable, Visual *, Colormap);`
     C: Xft.h: `void XftDrawDestroy (XftDraw *);`
     C: Xft.h: `void XftDrawRect (XftDraw *, const XftColor *, int, int, unsigned int,
     unsigned int);`
     C: Xft.h: `void XftDrawStringUtf8 (XftDraw *, const XftColor *, XftFont *, int, int,
     const FcChar8 *, int);`
     C: Xft.h: `void XftTextExtentsUtf8 (Display *, XftFont *, const FcChar8 *, int,
     XGlyphInfo *);`  */
  pub fn XftFontOpenName (dpy: *mut Display, screen: c_int, name: *const c_char)
    -> *mut XftFont;
  pub fn XftFontClose (dpy: *mut Display, pub_: *mut XftFont);
  pub fn XftDrawCreate (dpy: *mut Display, drawable: c_ulong, visual: *mut Visual,
                        colormap: Colormap) -> *mut XftDraw;
  pub fn XftDrawDestroy (draw: *mut XftDraw);
  pub fn XftDrawRect (draw: *mut XftDraw, color: *const XftColor, x: c_int, y: c_int,
                      width: c_uint, height: c_uint);
  pub fn XftDrawStringUtf8 (draw: *mut XftDraw, color: *const XftColor, font: *mut XftFont,
                            x: c_int, y: c_int, string: *const FcChar8, len: c_int);
  pub fn XftTextExtentsUtf8 (dpy: *mut Display, font: *mut XftFont, string: *const FcChar8,
                             len: c_int, extents: *mut XGlyphInfo);
}

/// C: lisp.h 4432: `extern AVOID memory_full (size_t);` - the one name C 597 calls that the
/// mirrors this unit includes do not carry (`lisp.rs` declares `memory_full_up`, and `lisp.h`
/// 5710's `memory_full` macro wraps it); declared at the use site (P-72 rule 3).
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" {
  pub fn memory_full (size: size_t) -> !;
}

/* C 219-221: the `#ifdef USE_CAIRO` pair `cairo_surface_flush (cairo_get_target (…))` - the two
   cairo names that region reads beyond the `lwlib-utils.h` renamings.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo"))]
unsafe extern "C" {
  /// C: cairo.h: `cairo_surface_t *cairo_get_target (cairo_t *);`
  pub fn cairo_get_target (cr: *mut cairo_t) -> *mut cairo_surface_t;
  /// C: cairo.h: `void cairo_surface_flush (cairo_surface_t *);`
  pub fn cairo_surface_flush (surface: *mut cairo_surface_t);
  /// C: cairo.h: `void cairo_destroy (cairo_t *);` - the header's `XftDrawDestroy` renaming (C
  /// 130) expands to this entry (`lwlib-utils.h` 51).
  pub fn cairo_destroy (cr: *mut cairo_t);
}

/* ------------------------------------------------------------------------------------------
   The file's definitions, in C's order.
   ------------------------------------------------------------------------------------------ */

/* C 75-77: `static void xaw_generic_callback (Widget, XtPointer, XtPointer);` - C's forward
   declaration, deleted (PORTING.md 0.1-12/checklist 10: Rust items are order-independent).
   C 476-479's `static void wm_delete_window (…);` is deleted the same way.  */

/* C 80-85: `lw_xaw_widget_p`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_xaw_widget_p (widget: Widget) -> Boolean
{
  return (XtIsSubclass (widget, scrollbarWidgetClass) != 0
          || XtIsSubclass (widget, dialogWidgetClass) != 0) as Boolean;
}

/* C 88-120: `fill_xft_data` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn fill_xft_data (data: *mut widget_xft_data, widget: Widget, font: *mut XftFont)
{
  use crate::xaw_x11_constants::None;

  let mut bg: Pixel = 0;
  let mut fg: Pixel = 0;
  let mut colors: [XColor; 2] = [core::mem::zeroed (); 2];

  (*data).widget = widget;
  (*data).xft_font = font;
  XtVaGetValues (widget, XtNbackground.as_ptr (), &raw mut bg,
                 XtNforeground.as_ptr (), &raw mut fg, NULL);

  (*data).xft_fg.pixel = fg;
  colors[0].pixel = fg;
  (*data).xft_bg.pixel = bg;
  colors[1].pixel = bg;
  XQueryColors (XtDisplay (widget), DefaultColormapOfScreen (XtScreen (widget)),
                colors.as_mut_ptr (), 2);

  (*data).xft_fg.color.alpha = 0xFFFF;
  (*data).xft_fg.color.red = colors[0].red;
  (*data).xft_fg.color.green = colors[0].green;
  (*data).xft_fg.color.blue = colors[0].blue;
  (*data).xft_bg.color.alpha = 0xFFFF;
  (*data).xft_bg.color.red = colors[1].red;
  (*data).xft_bg.color.green = colors[1].green;
  (*data).xft_bg.color.blue = colors[1].blue;

  (*data).p = None as Pixmap;
  (*data).xft_draw = core::ptr::null_mut ();
  (*data).p_width = 0;
  (*data).p_height = 0;
}

/* C 122-137: `destroy_xft_data` (static; its address goes to `XtAddCallback` at C 600, so it is
   an `unsafe extern "C" fn`, PORTING.md 1.4).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn destroy_xft_data (widget: Widget, closure: XtPointer, _call_data: XtPointer)
{
  use crate::xaw_x11_constants::None;

  let xft_data: *mut widget_xft_data = closure as *mut widget_xft_data;
  let mut i: c_int = 0;

  while !(*xft_data.offset (i as isize)).widget.is_null ()
    {
      if !(*xft_data.offset (i as isize)).xft_draw.is_null ()
        {
          #[cfg (feature = "use_cairo")]
          { XftDrawDestroy! ((*xft_data.offset (i as isize)).xft_draw); }
          #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
          { XftDrawDestroy ((*xft_data.offset (i as isize)).xft_draw); }
        }
      if (*xft_data.offset (i as isize)).p != None as Pixmap
        { XFreePixmap (XtDisplay (widget), (*xft_data.offset (i as isize)).p); }
      i += 1;
    }
  if !(*xft_data).xft_font.is_null ()
    {
      #[cfg (feature = "use_cairo")]
      { XftFontClose! (XtDisplay (widget), (*xft_data).xft_font); }
      #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
      { XftFontClose (XtDisplay (widget), (*xft_data).xft_font); }
    }
  xfree (xft_data as *mut c_void);
}

/* C 139-160: `openFont` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn openFont (widget: Widget, name: *mut c_char) -> *mut XftFont
{
  let mut fname: *mut c_char = name;
  let screen: c_int = XScreenNumberOfScreen (XtScreen (widget));
  let len: c_int = strlen (fname) as c_int;
  let mut i: c_int = len - 1;
  let fn_: *mut XftFont;

  /* Try to convert Gtk-syntax (Sans 9) to Xft syntax Sans-9.  */
  while i > 0 && (*fname.offset (i as isize) as c_int) >= (b'0' as c_int)
        && (*fname.offset (i as isize) as c_int) <= (b'9' as c_int)
    { i -= 1; }
  if *fname.offset (i as isize) == b' ' as c_char
    {
      fname = xstrdup (name);
      *fname.offset (i as isize) = b'-' as c_char;
    }

  #[cfg (feature = "use_cairo")]
  { fn_ = XftFontOpenName! (XtDisplay (widget), screen, fname); }
  #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
  { fn_ = XftFontOpenName (XtDisplay (widget), screen, fname); }
  if fname != name { xfree (fname as *mut c_void); }

  return fn_;
}

/* C 162-185: `get_text_width_and_height` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn get_text_width_and_height (widget: Widget, text: *mut c_char, xft_font: *mut XftFont,
                                     height: *mut c_int) -> c_int
{
  let mut w: c_int = 0;
  let mut h: c_int = 0;
  let mut bp: *mut c_char = text;

  while !bp.is_null () && *bp != 0
    {
      let mut gi: XGlyphInfo = core::mem::zeroed ();
      let cp: *mut c_char = strchr (bp, b'\n' as c_int);
      #[cfg (feature = "use_cairo")]
      { XftTextExtentsUtf8! (XtDisplay (widget), xft_font, bp as *mut FcChar8,
                             (if !cp.is_null () { cp.offset_from (bp) as c_int }
                              else { strlen (bp) as c_int }),
                             &raw mut gi); }
      #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
      { XftTextExtentsUtf8 (XtDisplay (widget), xft_font, bp as *mut FcChar8,
                            (if !cp.is_null () { cp.offset_from (bp) as c_int }
                             else { strlen (bp) as c_int }),
                            &raw mut gi); }
      bp = if !cp.is_null () { cp.offset (1) } else { core::ptr::null_mut () };
      h += (*xft_font).height;
      if w < gi.xOff as c_int { w = gi.xOff as c_int; }
    }

  *height = h;
  return w;
}

/* C 187-223: `draw_text` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn draw_text (data: *mut widget_xft_data, lbl: *mut c_char, inverse: c_int)
{
  let sc: *mut Screen = XtScreen ((*data).widget);
  let screen: c_int = XScreenNumberOfScreen (sc);
  let mut y: c_int = (*(*data).xft_font).ascent;
  let x: c_int = if inverse != 0 { 0 } else { 2 };
  let mut bp: *mut c_char = lbl;

  if (*data).xft_draw.is_null ()
    {
      #[cfg (feature = "use_cairo")]
      { (*data).xft_draw = XftDrawCreate! (XtDisplay ((*data).widget),
                                           (*data).p,
                                           DefaultVisual (XtDisplay ((*data).widget), screen),
                                           DefaultColormapOfScreen (sc)); }
      #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
      { (*data).xft_draw = XftDrawCreate (XtDisplay ((*data).widget),
                                          (*data).p,
                                          DefaultVisual (XtDisplay ((*data).widget), screen),
                                          DefaultColormapOfScreen (sc)); }
    }
  #[cfg (feature = "use_cairo")]
  { XftDrawRect! ((*data).xft_draw,
                  if inverse != 0 { &raw const (*data).xft_fg }
                  else { &raw const (*data).xft_bg },
                  0, 0, (*data).p_width as c_uint, (*data).p_height as c_uint); }
  #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
  { XftDrawRect ((*data).xft_draw,
                 if inverse != 0 { &raw const (*data).xft_fg }
                 else { &raw const (*data).xft_bg },
                 0, 0, (*data).p_width as c_uint, (*data).p_height as c_uint); }

  if inverse == 0 { y += 2; }
  while !bp.is_null () && *bp != 0
    {
      let cp: *mut c_char = strchr (bp, b'\n' as c_int);
      #[cfg (feature = "use_cairo")]
      { XftDrawStringUtf8! ((*data).xft_draw,
                            if inverse != 0 { &raw const (*data).xft_bg }
                            else { &raw const (*data).xft_fg },
                            (*data).xft_font, x, y, bp as *mut FcChar8,
                            (if !cp.is_null () { cp.offset_from (bp) as c_int }
                             else { strlen (bp) as c_int })); }
      #[cfg (all (feature = "use_cairo_or_have_xft", not (feature = "use_cairo")))]
      { XftDrawStringUtf8 ((*data).xft_draw,
                           if inverse != 0 { &raw const (*data).xft_bg }
                           else { &raw const (*data).xft_fg },
                           (*data).xft_font, x, y, bp as *mut FcChar8,
                           (if !cp.is_null () { cp.offset_from (bp) as c_int }
                            else { strlen (bp) as c_int })); }
      bp = if !cp.is_null () { cp.offset (1) } else { core::ptr::null_mut () };
      /* 1.2 gives reasonable line spacing.  */
      y += (((*(*data).xft_font).height as c_double) * 1.2) as c_int;
    }
  #[cfg (feature = "use_cairo")]
  { cairo_surface_flush (cairo_get_target ((*data).xft_draw)); }

}

/* C 226-243: `set_text` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn set_text (data: *mut widget_xft_data, toplevel: Widget, lbl: *mut c_char,
                    margin: c_int)
{
  let mut width: c_int;
  let mut height: c_int = 0;

  width = get_text_width_and_height ((*data).widget, lbl, (*data).xft_font, &raw mut height);
  (*data).p_width = width + margin;
  (*data).p_height = height + margin;

  (*data).p = XCreatePixmap (XtDisplay ((*data).widget),
                             XtWindow (toplevel),
                             (*data).p_width as c_uint,
                             (*data).p_height as c_uint,
                             DefaultDepthOfScreen (XtScreen ((*data).widget)) as c_uint);
  draw_text (data, lbl, 0);
  XtVaSetValues ((*data).widget, XtNbitmap.as_ptr (), (*data).p, NULL);
}

/* C 245-266: `find_xft_data` (static).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe fn find_xft_data (widget: Widget) -> *mut widget_xft_data
{
  let mut inst: *mut widget_instance = core::ptr::null_mut ();
  let mut parent: Widget = XtParent (widget);
  let mut data: *mut widget_xft_data = core::ptr::null_mut ();
  let mut nr: c_int;
  while !parent.is_null () && inst.is_null ()
    {
      inst = lw_get_widget_instance (parent);
      parent = XtParent (parent);
    }
  if inst.is_null () || (*inst).xft_data.is_null () || (*(*inst).xft_data).xft_font.is_null ()
    { return core::ptr::null_mut (); }

  nr = 0;
  while data.is_null () && !(*((*inst).xft_data).offset (nr as isize)).widget.is_null ()
    {
      if (*((*inst).xft_data).offset (nr as isize)).widget == widget
        { data = ((*inst).xft_data).offset (nr as isize); }
      nr += 1;
    }

  return data;
}

/* C 268-284: `command_press` (static; an `XtActionProc` of C 486-490's table).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn command_press (widget: Widget, _event: *mut XEvent, _params: *mut String,
                                    _num_params: *mut Cardinal)
{
  let data: *mut widget_xft_data = find_xft_data (widget);
  if !data.is_null ()
    {
      let mut lbl: *mut c_char = core::ptr::null_mut ();
      /* Since this isn't used for rectangle buttons, use it to for armed.  */
      XtVaSetValues (widget, XtNcornerRoundPercent.as_ptr (), 1, NULL);

      XtVaGetValues (widget, XtNlabel.as_ptr (), &raw mut lbl, NULL);
      draw_text (data, lbl, 1);
    }
}

/* C 286-305: `command_reset` (static; the other `XtActionProc`).  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
unsafe extern "C" fn command_reset (widget: Widget, _event: *mut XEvent, _params: *mut String,
                                    _num_params: *mut Cardinal)
{
  let data: *mut widget_xft_data = find_xft_data (widget);
  if !data.is_null ()
    {
      let mut cr: Dimension = 0;
      XtVaGetValues (widget, XtNcornerRoundPercent.as_ptr (), &raw mut cr, NULL);
      if cr == 1
        {
          let mut lbl: *mut c_char = core::ptr::null_mut ();
          XtVaSetValues (widget, XtNcornerRoundPercent.as_ptr (), 0, NULL);
          XtVaGetValues (widget, XtNlabel.as_ptr (), &raw mut lbl, NULL);
          draw_text (data, lbl, 0);
        }
    }
}

/* C 308: the region's `#endif`.  */

/* C 311-375: `xaw_update_one_widget`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_update_one_widget (instance: *mut widget_instance, widget: Widget,
                                                val: *mut widget_value, _deep_p: Boolean)
{
  if XtIsSubclass (widget, dialogWidgetClass) != 0
    {
      #[cfg (feature = "use_cairo_or_have_xft")]
      if !(*instance).xft_data.is_null () && !(*(*instance).xft_data).xft_font.is_null ()
        {
          set_text ((*instance).xft_data,
                    (*instance).parent,
                    (*(*val).contents).value, 10);
        }
      XtVaSetValues (widget, XtNlabel.as_ptr (), (*(*val).contents).value, NULL);
    }
  else if XtIsSubclass (widget, commandWidgetClass) != 0
    {
      let mut bw: Dimension = 0;
      let mut al: [Arg; 10] = [core::mem::zeroed (); 10];
      let mut ac: c_int = 0;

      XtVaGetValues (widget, XtNborderWidth.as_ptr (), &raw mut bw, NULL);
      if bw == 0
	/* Don't let buttons end up with 0 borderwidth, that's ugly...
	   Yeah, all this should really be done through app-defaults files
	   or fallback resources, but that's a whole different can of worms
	   that I don't feel like opening right now.  Making Athena widgets
	   not look like shit is just entirely too much work.
	 */
	{
	  XtSetArg! (al[0], XtNborderWidth.as_ptr (), 1);
	  XtSetValues (widget, al.as_mut_ptr (), 1);
	}

      XtSetSensitive (widget, (*val).enabled as Boolean);
      XtSetArg! (al[ac as usize], XtNlabel.as_ptr (), (*val).value); ac += 1;
      /* Force centered button text.  Se above. */
      XtSetArg! (al[ac as usize], XtNjustify.as_ptr (), XtJustifyCenter); ac += 1;
      #[cfg (feature = "use_cairo_or_have_xft")]
      if !(*instance).xft_data.is_null () && !(*(*instance).xft_data).xft_font.is_null ()
        {
          let mut th: c_int = 0;
          let mut nr: c_int;
          nr = 0;
          while !(*((*instance).xft_data).offset (nr as isize)).widget.is_null ()
            {
              if (*((*instance).xft_data).offset (nr as isize)).widget == widget
                { break; }
              nr += 1;
            }
          if !(*((*instance).xft_data).offset (nr as isize)).widget.is_null ()
            {
              set_text (((*instance).xft_data).offset (nr as isize),
                        (*instance).parent, (*val).value, 6);

              /* Must set internalHeight to twice the highlight thickness,
                 or else it gets overwritten by our pixmap.  Probably a bug.  */
              XtVaGetValues (widget, XtNhighlightThickness.as_ptr (), &raw mut th, NULL);
              XtSetArg! (al[ac as usize], XtNinternalHeight.as_ptr (), 2 * th); ac += 1;
            }
        }
      XtSetValues (widget, al.as_mut_ptr (), ac as Cardinal);
      XtRemoveAllCallbacks (widget, XtNcallback.as_ptr ());
      XtAddCallback (widget, XtNcallback.as_ptr (), Some (xaw_generic_callback),
                     instance as XtPointer);
    }
}

/* C 378-385: `xaw_update_one_value`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_update_one_value (_instance: *mut widget_instance, _widget: Widget,
                                               _val: *mut widget_value)
{
  /* This function is not used by the scrollbars and those are the only
     Athena widget implemented at the moment so do nothing. */
  return;
}

/* C 388-395: `xaw_destroy_instance`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_destroy_instance (instance: *mut widget_instance)
{
  if XtIsSubclass ((*instance).widget, dialogWidgetClass) != 0
    /* Need to destroy the Shell too. */
    { XtDestroyWidget (XtParent ((*instance).widget)); }
  else
    { XtDestroyWidget ((*instance).widget); }
}

/* C 398-402: `xaw_popup_menu`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_popup_menu (_widget: Widget, _event: *mut XEvent)
{
  /* An Athena menubar has not been implemented. */
  return;
}

/* C 405-466: `xaw_pop_instance`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_pop_instance (instance: *mut widget_instance, up: Boolean)
{
  let widget: Widget = (*instance).widget;

  if up != 0
    {
      if XtIsSubclass (widget, dialogWidgetClass) != 0
	{
	  /* For dialogs, we need to call XtPopup on the parent instead
	     of calling XtManageChild on the widget.
	     Also we need to hack the shell's WM_PROTOCOLS to get it to
	     understand what the close box is supposed to do!!
	   */
	  let dpy: *mut Display = XtDisplay (widget);
	  let shell: Widget = XtParent (widget);
	  let mut props: [Atom; 2] = [0; 2];
	  let mut i: c_int = 0;
	  props[i as usize] = XInternAtom (dpy, c"WM_DELETE_WINDOW".as_ptr (), False);
	  i += 1;
	  XChangeProperty (dpy, XtWindow (shell),
			   XInternAtom (dpy, c"WM_PROTOCOLS".as_ptr (), False),
			   XA_ATOM, 32, PropModeAppend,
			   props.as_mut_ptr () as *mut c_uchar, i);

	  /* Center the widget in its parent.  Why isn't this kind of crap
	     done automatically?  I thought toolkits were supposed to make
	     life easier?
	   */
	  {
	    let mut x: c_uint = 0;
	    let mut y: c_uint = 0;
	    let w: c_uint;
	    let h: c_uint;
	    let mut topmost: Widget = (*instance).parent;
	    let mut args: [Arg; 2] = [core::mem::zeroed (); 2];

	    w = (*shell).core.width as c_uint;
	    h = (*shell).core.height as c_uint;
	    while !(*topmost).core.parent.is_null () && XtIsRealized ((*topmost).core.parent) != 0
	      { topmost = (*topmost).core.parent; }
	    if ((*topmost).core.width as c_uint) < w { x = (*topmost).core.x as c_uint; }
	    else { x = (*topmost).core.x as c_uint
	           + (((*topmost).core.width as c_uint - w) / 2); }
	    if ((*topmost).core.height as c_uint) < h { y = (*topmost).core.y as c_uint; }
	    else { y = (*topmost).core.y as c_uint
	           + (((*topmost).core.height as c_uint - h) / 2); }
	    /* Using XtMoveWidget caused the widget to come
	       out in the wrong place with vtwm.
	       Question of virtual vs real coords, perhaps.  */
	    XtSetArg! (args[0], XtNx.as_ptr (), x);
	    XtSetArg! (args[1], XtNy.as_ptr (), y);
	    XtSetValues (shell, args.as_mut_ptr (), 2);
	  }

	  /* Finally, pop it up. */
	  XtPopup (shell, XtGrabNonexclusive);
	}
      else
	{ XtManageChild (widget); }
    }
  else
    {
      if XtIsSubclass (widget, dialogWidgetClass) != 0
	{ XtUnmanageChild (XtParent (widget)); }
      else
	{ XtUnmanageChild (widget); }
    }
}

/* C 471-474: the two translation tables.  C's `static char X [] = "…"` carries the bytes and the
   port's `&CStr` the same bytes (the initializer's array length is C's own bookkeeping).  */
#[cfg (feature = "use_lucid")]
static overrideTrans: &core::ffi::CStr =
  c"<Message>WM_PROTOCOLS: lwlib_delete_dialog()";
/* Dialogs pop down on any key press */
#[cfg (feature = "use_lucid")]
static dialogOverride: &core::ffi::CStr =
  c"<KeyPress>Escape:\tlwlib_delete_dialog()";

/* C 480-482: `xaw_actions` - `wm_delete_window`'s only reference, so its `unsafe extern "C"`
   signature is the one C's `XtActionsRec.proc` requires.  */
#[cfg (feature = "use_lucid")]
static mut xaw_actions: [XtActionsRec; 1] =
  [XtActionsRec { name: c"lwlib_delete_dialog".as_ptr () as String,
                  proc: Some (wm_delete_window) }];
/* C 483: `static Boolean actions_initted = False;` */
#[cfg (feature = "use_lucid")]
static mut actions_initted: Boolean = False;

/* C 485-495: the same region's second table and its translations string.  */
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
static mut button_actions: [XtActionsRec; 2] =
  [XtActionsRec { name: c"my_reset".as_ptr () as String, proc: Some (command_reset) },
   XtActionsRec { name: c"my_press".as_ptr () as String, proc: Some (command_press) }];
#[cfg (all (feature = "use_lucid", feature = "use_cairo_or_have_xft"))]
static buttonTrans: &core::ffi::CStr =
  c"<Leave>: reset() my_reset()\n<Btn1Down>: set() my_press()\n<Btn1Up>:  my_reset() notify() unset()\n";

/* C 497-673: `make_dialog` (static).  */
#[cfg (feature = "use_lucid")]
unsafe fn make_dialog (name: *mut c_char, parent: Widget, pop_up_p: Boolean,
                       shell_title: *mut c_char, icon_name: *mut c_char,
                       text_input_slot: Boolean, radio_box: Boolean, list: Boolean,
                       left_buttons: c_int, right_buttons: c_int,
                       instance: *mut widget_instance) -> Widget
{
  let mut av: [Arg; 20] = [core::mem::zeroed (); 20];
  let mut ac: c_int = 0;
  let mut i: c_int = 0;
  let mut bc: c_int = 0;
  let mut button_name: [c_char; 255] = [0; 255];
  let mut shell: Widget;
  let mut dialog: Widget;
  let mut button: Widget;
  let mut override_: XtTranslations;
  #[cfg (feature = "use_cairo_or_have_xft")]
  let mut xft_font: *mut XftFont = core::ptr::null_mut ();
  #[cfg (feature = "use_cairo_or_have_xft")]
  let mut button_override: XtTranslations;
  if pop_up_p == 0 { emacs_abort (); } /* not implemented */
  if text_input_slot != 0 { emacs_abort (); } /* not implemented */
  if radio_box != 0 { emacs_abort (); } /* not implemented */
  if list != 0 { emacs_abort (); } /* not implemented */

  if actions_initted == 0
    {
      let app: XtAppContext = XtWidgetToApplicationContext (parent);
      XtAppAddActions (app, &raw const xaw_actions as *mut XtActionsRec,
                       countof! (xaw_actions) as Cardinal);
      #[cfg (feature = "use_cairo_or_have_xft")]
      { XtAppAddActions (app, &raw const button_actions as *mut XtActionsRec,
                         countof! (button_actions) as Cardinal); }
      actions_initted = True;
    }

  override_ = XtParseTranslationTable (overrideTrans.as_ptr ());

  ac = 0;
  XtSetArg! (av[ac as usize], XtNtitle.as_ptr (), shell_title); ac += 1;
  XtSetArg! (av[ac as usize], XtNallowShellResize.as_ptr (), True); ac += 1;

  /* Don't allow any geometry request from the user.  */
  XtSetArg! (av[ac as usize], XtNgeometry.as_ptr (), 0); ac += 1;

  shell = XtCreatePopupShell (c"dialog".as_ptr (), transientShellWidgetClass,
			      parent, av.as_mut_ptr (), ac as Cardinal);
  XtOverrideTranslations (shell, override_);

  ac = 0;
  dialog = XtCreateManagedWidget (name, dialogWidgetClass, shell, av.as_mut_ptr (),
                                  ac as Cardinal);
  override_ = XtParseTranslationTable (dialogOverride.as_ptr ());
  XtOverrideTranslations (dialog, override_);

  #[cfg (feature = "use_cairo_or_have_xft")]
  {
    let mut num: c_int = 0;
    let mut ch: *mut Widget = NULL as *mut Widget;
    let mut w_: Widget = core::ptr::null_mut ();
    XtVaGetValues (dialog,
                   XtNnumChildren.as_ptr (), &raw mut num,
                   XtNchildren.as_ptr (), &raw mut ch, NULL);
    i = 0;
    while i < num
      {
        if XtIsSubclass (ch.offset (i as isize) as Widget, commandWidgetClass) == 0
            && XtIsSubclass (ch.offset (i as isize) as Widget, labelWidgetClass) != 0
          {
            w_ = ch.offset (i as isize) as Widget;
            break;
          }
        i += 1;
      }
    (*instance).xft_data = core::ptr::null_mut ();
    if !w_.is_null ()
      {
        let rec: [XtResource; 1] =
          [XtResource { resource_name: c"font".as_ptr () as String,
                        resource_class: c"Font".as_ptr () as String,
                        resource_type: XtRString.as_ptr () as String,
                        resource_size: core::mem::size_of::<String> () as Cardinal,
                        resource_offset: 0,
                        default_type: XtRString.as_ptr () as String,
                        default_addr: c"Sans-10".as_ptr () as XtPointer }];
        let mut fontName: *mut c_char = core::ptr::null_mut ();
        XtVaGetSubresources (dialog, &raw mut fontName as XtPointer,
                             c"Dialog".as_ptr (), c"dialog".as_ptr (),
                             rec.as_ptr () as *mut XtResource, 1, NULL as String);
        if !fontName.is_null ()
          {
            let xfn: *mut XFontStruct = XLoadQueryFont (XtDisplay (dialog), fontName);
            if xfn.is_null ()
              { xft_font = openFont (dialog, fontName); }
            else
              { XFreeFont (XtDisplay (dialog), xfn); }
          }

        if !xft_font.is_null ()
          {
            let nr_xft_data: c_int = left_buttons + right_buttons + 1;
            (*instance).xft_data = calloc ((nr_xft_data + 1) as usize,
                                           core::mem::size_of::<widget_xft_data> ())
                                   as *mut widget_xft_data;
	    if (*instance).xft_data.is_null ()
	      { memory_full (((nr_xft_data + 1) as usize)
	                     * core::mem::size_of::<widget_xft_data> ()); }

            fill_xft_data ((*instance).xft_data, w_, xft_font);
	    XtAddCallback (dialog, XtNdestroyCallback.as_ptr (), Some (destroy_xft_data),
			   (*instance).xft_data as XtPointer);
          }
      }

    button_override = XtParseTranslationTable (buttonTrans.as_ptr ());
  }

  bc = 0;
  button = core::ptr::null_mut ();
  i = 0;
  while i < left_buttons
    {
      ac = 0;
      XtSetArg! (av[ac as usize], XtNfromHoriz.as_ptr (), button); ac += 1;
      XtSetArg! (av[ac as usize], XtNleft.as_ptr (), XtChainLeft); ac += 1;
      XtSetArg! (av[ac as usize], XtNright.as_ptr (), XtChainLeft); ac += 1;
      XtSetArg! (av[ac as usize], XtNtop.as_ptr (), XtChainBottom); ac += 1;
      XtSetArg! (av[ac as usize], XtNbottom.as_ptr (), XtChainBottom); ac += 1;
      XtSetArg! (av[ac as usize], XtNresizable.as_ptr (), True); ac += 1;
      #[cfg (feature = "have_xaw3d")]
      if DefaultDepthOfScreen (XtScreen (dialog)) >= 16
        {
          /* Turn of dithered shadow if we can.  Looks bad */
          XtSetArg! (av[ac as usize], c"beNiceToColormap".as_ptr (), False); ac += 1;
        }
      sprintf (button_name.as_mut_ptr (), c"button%d".as_ptr (), { bc += 1; bc });
      button = XtCreateManagedWidget (button_name.as_ptr (), commandWidgetClass,
				      dialog, av.as_mut_ptr (), ac as Cardinal);
      #[cfg (feature = "use_cairo_or_have_xft")]
      if !xft_font.is_null ()
        {
          fill_xft_data ((*instance).xft_data.offset (bc as isize), button, xft_font);
          XtOverrideTranslations (button, button_override);
        }
      i += 1;
    }

  i = 0;
  while i < right_buttons
    {
      ac = 0;
      XtSetArg! (av[ac as usize], XtNfromHoriz.as_ptr (), button); ac += 1;
      if i == 0
        {
          /* Separator to the other buttons. */
          XtSetArg! (av[ac as usize], XtNhorizDistance.as_ptr (), 30); ac += 1;
        }
      XtSetArg! (av[ac as usize], XtNleft.as_ptr (), XtChainRight); ac += 1;
      XtSetArg! (av[ac as usize], XtNright.as_ptr (), XtChainRight); ac += 1;
      XtSetArg! (av[ac as usize], XtNtop.as_ptr (), XtChainBottom); ac += 1;
      XtSetArg! (av[ac as usize], XtNbottom.as_ptr (), XtChainBottom); ac += 1;
      XtSetArg! (av[ac as usize], XtNresizable.as_ptr (), True); ac += 1;
      #[cfg (feature = "have_xaw3d")]
      if DefaultDepthOfScreen (XtScreen (dialog)) >= 16
        {
          /* Turn of dithered shadow if we can.  Looks bad */
          XtSetArg! (av[ac as usize], c"beNiceToColormap".as_ptr (), False); ac += 1;
        }
      sprintf (button_name.as_mut_ptr (), c"button%d".as_ptr (), { bc += 1; bc });
      button = XtCreateManagedWidget (button_name.as_ptr (), commandWidgetClass,
				      dialog, av.as_mut_ptr (), ac as Cardinal);
      #[cfg (feature = "use_cairo_or_have_xft")]
      if !xft_font.is_null ()
        {
          fill_xft_data ((*instance).xft_data.offset (bc as isize), button, xft_font);
          XtOverrideTranslations (button, button_override);
        }
      i += 1;
    }

  return dialog;
}

/* C 675-736: `xaw_create_dialog`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xaw_create_dialog (instance: *mut widget_instance) -> Widget
{
  let name: *mut c_char = (*(*instance).info).r#type;
  let parent: Widget = (*instance).parent;
  let widget: Widget;
  let pop_up_p: Boolean = (*instance).pop_up_p;
  let mut shell_name: *mut c_char = core::ptr::null_mut ();
  let mut icon_name: *mut c_char = core::ptr::null_mut ();
  let mut text_input_slot: Boolean = False;
  let mut radio_box: Boolean = False;
  let mut list: Boolean = False;
  let mut total_buttons: c_int;
  let mut left_buttons: c_int = 0;
  let mut right_buttons: c_int = 1;

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
      /* C's `switch` has no `default` arm: a name C does not list leaves all five locals
         unchanged (the `match` arm Rust requires for exhaustiveness).  */
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

  widget = make_dialog (name, parent, pop_up_p,
			shell_name, icon_name, text_input_slot, radio_box,
			list, left_buttons, right_buttons, instance);
  return widget;
}

/* C 739-778: `xaw_generic_callback` (static; `XtAddCallback`'s `XtCallbackProc` at C 370).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn xaw_generic_callback (widget: Widget, closure: XtPointer,
                                           call_data: XtPointer)
{
  let instance: *mut widget_instance = closure as *mut widget_instance;
  let instance_widget: Widget;
  let id: LWLIB_ID;
  let user_data: XtPointer;

  lw_internal_update_other_instances (widget, closure, call_data);

  if instance.is_null ()
    { return; }
  if (*widget).core.being_destroyed != 0
    { return; }

  instance_widget = (*instance).widget;
  if instance_widget.is_null ()
    { return; }

  id = (*(*instance).info).id;

  /* Damn!  Athena doesn't give us a way to hang our own data on the
     buttons, so we have to go find it...  I guess this assumes that
     all instances of a button have the same call data. */
  {
    let mut val: *mut widget_value = (*(*(*instance).info).val).contents;
    let name: *const c_char = XtName (widget);
    while !val.is_null ()
      {
	if !(*val).name.is_null () && strcmp ((*val).name, name) == 0
	  { break; }
	val = (*val).next;
      }
    if val.is_null () { emacs_abort (); }
    user_data = (*val).call_data;
  }

  if (*(*instance).info).selection_cb.is_some ()
    { ((*(*instance).info).selection_cb.unwrap ()) (widget, id, user_data); }
}

/* C 780-822: `wm_delete_window` (static; the `XtActionProc` of C 480-482).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn wm_delete_window (w: Widget, _event: *mut XEvent, _params: *mut String,
                                       _num_params: *mut Cardinal)
{
  let id: LWLIB_ID;
  let mut nkids: Cardinal = 0;
  let mut i: c_int;
  let mut kids: *mut Widget = core::ptr::null_mut ();
  let mut widget: Widget = core::ptr::null_mut ();
  let shell: Widget;

  if XtIsSubclass (w, dialogWidgetClass) != 0
    { shell = XtParent (w); }
  else
    { shell = w; }

  if XtIsSubclass (shell, shellWidgetClass) == 0
    { emacs_abort (); }
  XtVaGetValues (shell, XtNnumChildren.as_ptr (), &raw mut nkids, NULL);
  XtVaGetValues (shell, XtNchildren.as_ptr (), &raw mut kids, NULL);
  if kids.is_null () || (*kids).is_null ()
    { emacs_abort (); }
  i = 0;
  while (i as c_uint) < nkids
    {
      widget = *kids.offset (i as isize);
      if XtIsSubclass (widget, dialogWidgetClass) != 0
	{ break; }
      i += 1;
    }
  if widget.is_null () { return; }

  id = lw_get_widget_id (widget);
  if id == 0 { emacs_abort (); }

  {
    let info: *mut widget_info = lw_get_widget_info (id);
    if info.is_null () { emacs_abort (); }
    if (*info).selection_cb.is_some ()
      { ((*info).selection_cb.unwrap ()) (widget, id, (-1 as c_int) as XtPointer); }
  }

  lw_destroy_all_widgets (id);
}

/* C 826-837: `xaw_create_main` (static; the one `widget_creation_function` of C 839-844).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn xaw_create_main (instance: *mut widget_instance) -> Widget
{
  let mut al: [Arg; 1] = [core::mem::zeroed (); 1];
  let mut ac: c_int;

  /* Create a vertical Paned to hold menubar */
  ac = 0;
  XtSetArg! (al[ac as usize], XtNborderWidth.as_ptr (), 0); ac += 1;
  return XtCreateWidget ((*(*instance).info).name, panedWidgetClass,
			 (*instance).parent, al.as_mut_ptr (), ac as Cardinal);
}

/* C 839-844: `xaw_creation_table`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub static mut xaw_creation_table: [widget_creation_entry; 2] =
  [widget_creation_entry { r#type: c"main".as_ptr () as *mut c_char,
                           function: Some (xaw_create_main) },
   widget_creation_entry { r#type: core::ptr::null_mut (), function: None }];
