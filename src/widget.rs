// femacs: port of src/widget.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct member, file-scope object, local), the same control flow.  The four non-`static` C
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain
// `unsafe fn`, and the seven whose address the class record or Xt takes are `unsafe extern "C" fn`
// (PORTING.md 1.4).
//
// **The file gate is `feature = "use_x_toolkit"`.**  C's text has no `#ifdef` around its body at
// all (C 26-514 is unconditional text); the condition is the *build's*: `configure.ac` 7227-7229
// sets `WIDGET_OBJ=widget.o` and `AC_DEFINE ([USE_X_TOOLKIT], [1], …)` under
// `if test "${USE_X_TOOLKIT}" != "none"`, and `src/Makefile.in` 298/499 puts `$(WIDGET_OBJ)` into
// `otherobj`.  `USE_X_TOOLKIT` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 3873
// and `docs/baseline/frozen-build-sources.txt` lists no `widget.o`, so the frozen configuration
// compiles nothing from this file and the port's frozen object is the empty one (the
// `src/haiku.c`/`src/androidmenu.c` reading of the same situation).
//
// **This file is `text-only`** (P-86 clause 3(2)): this machine has no Xt and no Xaw header at all
// (`/opt/X11/include/X11` does not exist; `/opt/homebrew/include/X11` carries `xorgproto` 2025.1's
// `<X11/Xlib.h>`, `<X11/Xutil.h>`, `<X11/Xresource.h>`, `<X11/Xregion.h>` and `<X11/cursorfont.h>`,
// but no `<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`, `<X11/Shell.h>`,
// `<X11/ShellP.h>`, `<X11/ObjectP.h>` or `<X11/StringDefs.h>`).  The Xlib half of the surface is
// therefore **measured** (`clang` against those headers; the numbers are in `docs/ports/widget.md`
// `## Evidence`), and every Xt width, offset, enum value and resource-type string is C's spelling
// with the value **UNVERIFIED** - the `include/gtk.rs` `placeholder, not GDK's value` practice
// (status 3.481), which keeps the arms and the bit tests distinct instead of inventing behaviour.
//
// The X11/Xt surface stands in this unit (P-88 clause 1: no header in the reference tree declares
// any of it and this file is its first consumer; `include/xlib.rs` is the port's mirror of the Xlib
// half only).  The dedicated SDK mirror P-90 clause 1 asks for - an Xt/Xaw mirror
// `src/include/<sdk>.rs`, which `src/xfns.rs`'s `USE_X_TOOLKIT` block, `src/xterm.rs` 775 and
// the four `lwlib/` units will all want - is owed, and this unit's copy is the one to move then
// (P-88 clause 5 / P-90 clause 3).  `docs/ports/widget.md` names every name's C header and what the
// move requires.
//
// **Two of the mirrors this unit includes do not carry C's records for the widget yet**, and this
// unit says so rather than working around the substitution silently:
//
//   * `include/widget.rs` 154-157 spells `EmacsFrame`/`EmacsFrameClass` as `*mut c_void` (P-55's
//     untyped pointer).  C's `EmacsFrame` points at `_EmacsFrameRec` (`widget.h` 90 plus
//     `widgetprv.h` 56-59), and `widgetprv.rs` derives its `EmacsFramePart` from that same header,
//     so the unit declares the two records under C's own **tag** names
//     (`_EmacsFrameRec`/`_EmacsFrameClassRec`, which no mirror claims) and converts at each use
//     site (P-72 clause 3's "the use site converts explicitly").
//   * `include/widgetprv.rs` 114-166 has the same three records, but with the **by-value**
//     `CorePart`/`CoreClassPart` replaced by `*mut c_void` (its ledger's deferral).  A pointer
//     where C has an inline part cannot express C's `ew->core.width` reads or the class record's
//     thirty-two-value initializer, so those two Xt parts are declared here as well.
//     `docs/ports/widget.md` `## Owed` carries the mirror fix (the `lwlib/include/xlwmenuP.rs`
//     12-19 convention: keep C's spelling and let the including unit supply `CorePart`), and the
//     mirror's own `EmacsFrameRec`/`EmacsFrameClassRec` stay unused until then.
//
// The four entry points `include/widget.rs` declares at its top level (C 146-155, 320-324,
// 474-491, 507-514) are *defined* here.  A crate-root definition plus a crate-root declaration is
// `E0428` (P-39), and the fix is the mirror's (`include/emacsgtkfixed.rs` 49-56's `_decls` module,
// status 3.496 ②⑺); the unit records it in `## Owed` instead of moving the mirror's text.
//
// `docs/ports/widget.md` carries the C line-range table, the acceptance readings, every deviation
// and the items the integrator owes.  Nothing in this file runs anywhere: there is no X toolkit on
// this machine, no Xt header to compile the arm against and no X target to run it on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 26: `#include <config.h>` (every unit includes it)

/* C 27-34: the file's own include list.  Every `include!` carries the file gate (P-86 clause 8);
   `lisp.h` is C's own order (PORTING.md 0.1-14).  */
#[cfg (feature = "use_x_toolkit")]
include!("include/widget.rs");                 // C 27: `#include "widget.h"`
#[cfg (feature = "use_x_toolkit")]
include!("../lib/include/stdlib.rs");          // C 29: `#include <stdlib.h>`
#[cfg (feature = "use_x_toolkit")]
include!("include/lisp.rs");                   // C 31: `#include "lisp.h"`
/* The `<string.h>` edge C reaches through `"lisp.h"` supplies `memcpy`/`memcmp` (C 287/316);
   `include/lisp.rs` carries neither mirror (its own note records the omission), so the edge is
   here, at the C position that makes the names visible (the `src/gtkutil.rs` 29-33 shape).  */
#[cfg (feature = "use_x_toolkit")]
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
#[cfg (feature = "use_x_toolkit")]
include!("include/sysstdio.rs");               // C 32: `#include "sysstdio.h"` (`fputs`, `stderr`)
#[cfg (feature = "use_x_toolkit")]
include!("include/xterm.rs");                  // C 33: `#include "xterm.h"` (brings `xlib.rs`)
/* `xterm.h` 129 includes `dispextern.h`, whose C 25 includes `character.h`; the character.h mirror
   is not reachable from `include/dispextern.rs` (its own `include!` list omits it), so the edge is
   here, at the C position that makes its names visible (the `src/dispnew.rs` 47 reading of the same
   line).  */
#[cfg (feature = "use_x_toolkit")]
include!("include/character.rs");              // dispextern.h 25: `#include "character.h"`
#[cfg (feature = "use_x_toolkit")]
include!("include/frame.rs");                  // C 34: `#include "frame.h"`

/* ==========================================================================================
   C 36-38: `#include <X11/StringDefs.h>`, `#include <X11/IntrinsicP.h>` and
   `#include <X11/cursorfont.h>`.

   None of the three has a mirror in this tree.  The declarations below stand in this unit
   (P-88 clause 1) and are split by the header C takes the name from; `docs/ports/widget.md`
   owes the move into the Xt mirror P-90 clause 1 asks for.

   `IntrinsicP.h` includes `Intrinsic.h`, which includes `<X11/Xlib.h>`, `<X11/Xutil.h>`,
   `<X11/Xresource.h>` and `<X11/Shell.h>`; the Xlib names this file needs that
   `include/xlib.rs` does not carry yet (`XSetWindowAttributes`, `XExposeEvent`,
   `XSizeHints`, `Region` and eleven macro values) are therefore declared here too, each
   with the header and line it was **measured** from on this machine's `xorgproto` 2025.1
   (`include/xlib.rs` is the home they are owed to - the `include/xterm.rs` 60-69 `True`/
   `False` reading of the same situation).
   ========================================================================================== */

/* C 36: `#include <X11/StringDefs.h>` - the resource-name and resource-type strings Xt looks a
   widget up by.  `widget.h`'s own 35 `XtN*`/`XtC*` strings are `include/widget.rs`'s; the ones
   below are the names this file uses that the header does not have.  C's spelling is kept, and
   the values are **UNVERIFIED** (no Xt header on this machine): they are the names the X
   resource-manager convention gives those resources, not a reading of Xt's text.  */
/// C: `<X11/StringDefs.h>`: `#define XtNiconic "iconic"` - C 59's resource name.
#[cfg (feature = "use_x_toolkit")]
pub const XtNiconic: &core::ffi::CStr = c"iconic";
/// C: `<X11/StringDefs.h>`: `#define XtCIconic "Iconic"` - C 59's resource class.
#[cfg (feature = "use_x_toolkit")]
pub const XtCIconic: &core::ffi::CStr = c"Iconic";
/// C: `<X11/StringDefs.h>`: `#define XtNbaseWidth "baseWidth"` - C 302's argument name.
#[cfg (feature = "use_x_toolkit")]
pub const XtNbaseWidth: &core::ffi::CStr = c"baseWidth";
/// C: `<X11/StringDefs.h>`: `#define XtNbaseHeight "baseHeight"` - C 303.
#[cfg (feature = "use_x_toolkit")]
pub const XtNbaseHeight: &core::ffi::CStr = c"baseHeight";
/// C: `<X11/StringDefs.h>`: `#define XtNwidthInc "widthInc"` - C 304.
#[cfg (feature = "use_x_toolkit")]
pub const XtNwidthInc: &core::ffi::CStr = c"widthInc";
/// C: `<X11/StringDefs.h>`: `#define XtNheightInc "heightInc"` - C 305.
#[cfg (feature = "use_x_toolkit")]
pub const XtNheightInc: &core::ffi::CStr = c"heightInc";
/// C: `<X11/StringDefs.h>`: `#define XtNminWidth "minWidth"` - C 306.
#[cfg (feature = "use_x_toolkit")]
pub const XtNminWidth: &core::ffi::CStr = c"minWidth";
/// C: `<X11/StringDefs.h>`: `#define XtNminHeight "minHeight"` - C 307.
#[cfg (feature = "use_x_toolkit")]
pub const XtNminHeight: &core::ffi::CStr = c"minHeight";

/* The six resource *type* names C 57-85's `resource_type`/`default_type` columns use.  Xt's
   `StringDefs.h` spells them the same way; the port keeps C's `String` (`char *`) at the use
   sites, the choice `include/widget.rs` 22-135 made for the resource-name strings.  */
/// C: `<X11/StringDefs.h>`: `#define XtRString "String"` - C 57's `resource_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRString: &core::ffi::CStr = c"String";
/// C: `<X11/StringDefs.h>`: `#define XtRImmediate "Immediate"` - C 60's `default_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRImmediate: &core::ffi::CStr = c"Immediate";
/// C: `<X11/StringDefs.h>`: `#define XtRBoolean "Boolean"` - C 59's `resource_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRBoolean: &core::ffi::CStr = c"Boolean";
/// C: `<X11/StringDefs.h>`: `#define XtRInt "Int"` - C 66's `resource_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRInt: &core::ffi::CStr = c"Int";
/// C: `<X11/StringDefs.h>`: `#define XtRPixel "Pixel"` - C 76's `resource_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRPixel: &core::ffi::CStr = c"Pixel";
/// C: `<X11/StringDefs.h>`: `#define XtRPointer "Pointer"` - C 63's `resource_type`.
#[cfg (feature = "use_x_toolkit")]
pub const XtRPointer: &core::ffi::CStr = c"Pointer";

/* `<X11/IntrinsicP.h>`'s scalar typedefs.  `Boolean`, `Cardinal` and `Pixel` are the substitutions
   `include/widgetprv.rs` 124-148 already writes inside its own record (`c_int`/`c_uint`/`c_ulong`),
   kept here with the same widths and named UNVERIFIED.  */
/// C: `<X11/Intrinsic.h>`: `typedef int Boolean;`
#[cfg (feature = "use_x_toolkit")]
pub type Boolean = c_int;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned int Cardinal;`
#[cfg (feature = "use_x_toolkit")]
pub type Cardinal = c_uint;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned short Dimension;`
#[cfg (feature = "use_x_toolkit")]
pub type Dimension = c_ushort;
/// C: `<X11/Intrinsic.h>`: `typedef short Position;`
#[cfg (feature = "use_x_toolkit")]
pub type Position = c_short;
/// C: `<X11/Intrinsic.h>`: `typedef char *String;` - the resource-name type.
#[cfg (feature = "use_x_toolkit")]
pub type String = *mut c_char;
/// C: `<X11/Intrinsic.h>`: `typedef char *XtPointer;`
#[cfg (feature = "use_x_toolkit")]
pub type XtPointer = *mut c_char;
/// C: `<X11/Intrinsic.h>`: `typedef long XtArgVal;` - a varargs `XtVaSetValues` argument (C 302).
#[cfg (feature = "use_x_toolkit")]
pub type XtArgVal = c_long;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned long XtValueMask;` - C 383's `*mask`.
#[cfg (feature = "use_x_toolkit")]
pub type XtValueMask = c_ulong;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned long XtGeometryMask;` - `XtWidgetGeometry.request_mode`.
#[cfg (feature = "use_x_toolkit")]
pub type XtGeometryMask = c_ulong;
/// C: `<X11/Intrinsic.h>`: `typedef int XtEnum;`
#[cfg (feature = "use_x_toolkit")]
pub type XtEnum = c_int;
/// C: `<X11/Intrinsic.h>`: `typedef unsigned long XtVersionType;` - `CoreClassPart.version`.
#[cfg (feature = "use_x_toolkit")]
pub type XtVersionType = c_ulong;

/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget;` - the pointee is Xt's own record,
/// whose layout nothing here reads, so it keeps the `include/xlib.rs` 881-883 `_XDisplay` shape
/// (an opaque record) and `Widget` is a real pointer type rather than `*mut c_void` (P-55).
#[cfg (feature = "use_x_toolkit")]
pub struct WidgetRec
{
  pub _private: [u8; 0],
}
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget;`
#[cfg (feature = "use_x_toolkit")]
pub type Widget = *mut WidgetRec;

/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetClassRec *WidgetClass;` - Xt's base class
/// record; C 152-153 takes its address (`&widgetClassRec`) and nothing reads its members.
#[cfg (feature = "use_x_toolkit")]
pub struct WidgetClassRec
{
  pub _private: [u8; 0],
}
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetClassRec *WidgetClass;`
#[cfg (feature = "use_x_toolkit")]
pub type WidgetClass = *mut WidgetClassRec;

/// C 152-153: `emacsFrameClassRec.core_class.superclass = &widgetClassRec;`
/// C: `<X11/Intrinsic.h>`: `extern WidgetClassRec widgetClassRec;` - Xt's base class record.
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  pub static mut widgetClassRec: WidgetClassRec;
}

/// C: `<X11/Intrinsic.h>`: `typedef struct { String name; XtArgVal value; } Arg;` and
/// `typedef Arg *ArgList;` - C 358's `dum1` is the only use, and it is never read.
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct Arg
{
  pub name: String,
  pub value: XtArgVal,
}
/// C: `<X11/Intrinsic.h>`: `typedef Arg *ArgList;`
#[cfg (feature = "use_x_toolkit")]
pub type ArgList = *mut Arg;

/// C: `<X11/Intrinsic.h>`: `typedef struct { String resource_name; String resource_class;
/// String resource_type; Cardinal resource_size; Cardinal resource_offset; String default_type;
/// XtPointer default_addr; } XtResource;` - C 57-85's twelve rows initialize exactly these seven
/// columns *positionally*, so the port's field names and order are the ones C's own initializer
/// implies (column 5 is the `offset (field)` byte offset, column 7 the default address).  The
/// record's layout is **UNVERIFIED** (no Xt header).
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
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

/// C: `<X11/Intrinsic.h>`: `typedef unsigned long XtGeometryMask;` and `typedef struct {
/// XtGeometryMask request_mode; Position x, y; Dimension width, height, border_width; Widget
/// sibling; int stack_mode; } XtWidgetGeometry;` - C 448-468 reads `request_mode`, `width` and
/// `height` only, so those three are carried in C's order and the rest are owed (the Xt header is
/// not on this machine, so the record's offsets are **UNVERIFIED** - the `src/haiku.rs` 87-120
/// "members the reference reads" reading).
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct XtWidgetGeometry
{
  pub request_mode: XtGeometryMask,
  pub width: Dimension,
  pub height: Dimension,
}

/// C: `<X11/Intrinsic.h>`: `typedef enum { XtGeometrySuccess, XtGeometryYes, XtGeometryNo,
/// XtGeometryAlmost, XtGeometryDone } XtGeometryResult;` - the port spells C's enum as `c_int`
/// with one constant per member (PORTING.md 1.3, the `include/lwlib.rs` 64-86 `menu_separator`
/// shape).  C 470's ternary only needs the two values to be *distinct*; the members' numbers are
/// **UNVERIFIED** and are recorded as placeholders.
#[cfg (feature = "use_x_toolkit")]
pub type XtGeometryResult = c_int;
/// C: `<X11/Intrinsic.h>`: `XtGeometrySuccess` - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtGeometrySuccess: XtGeometryResult = 0;
/// C: `<X11/Intrinsic.h>`: `XtGeometryYes` - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtGeometryYes: XtGeometryResult = 1;
/// C: `<X11/Intrinsic.h>`: `XtGeometryNo` - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtGeometryNo: XtGeometryResult = 2;
/// C: `<X11/Intrinsic.h>`: `XtGeometryAlmost` - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtGeometryAlmost: XtGeometryResult = 3;
/// C: `<X11/Intrinsic.h>`: `XtGeometryDone` - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtGeometryDone: XtGeometryResult = 4;

/* The Xt callback/proc types whose signatures this file's own prototypes give (C 45-51, C 372-376).
   The nullable ones are `Option` of an `extern "C"` fn (P-27, the `include/lwlib.rs` 89-90
   `lw_callback` shape), because C 114-141's zeros are null procedure slots.  */
/// C 45: `static void EmacsFrameInitialize (Widget, Widget, ArgList, Cardinal *);`
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtInitProc)(Widget, Widget, ArgList, Cardinal *);`
#[cfg (feature = "use_x_toolkit")]
pub type XtInitProc = Option<unsafe extern "C" fn (Widget, Widget, ArgList, *mut Cardinal)>;
/// C 46: `static void EmacsFrameDestroy (Widget);`
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtWidgetProc)(Widget);`
#[cfg (feature = "use_x_toolkit")]
pub type XtWidgetProc = Option<unsafe extern "C" fn (Widget)>;
/// C 47: `static void EmacsFrameRealize (Widget, XtValueMask *, XSetWindowAttributes *);`
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtRealizeProc)(Widget, XtValueMask *, XSetWindowAttributes *);`
#[cfg (feature = "use_x_toolkit")]
pub type XtRealizeProc =
  Option<unsafe extern "C" fn (Widget, *mut XtValueMask, *mut XSetWindowAttributes)>;
/// C 49: `static void EmacsFrameExpose (Widget, XEvent *, Region);`
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtExposeProc)(Widget, XEvent *, Region);`
#[cfg (feature = "use_x_toolkit")]
pub type XtExposeProc = Option<unsafe extern "C" fn (Widget, *mut XEvent, Region)>;
/// C 50-51: `static XtGeometryResult EmacsFrameQueryGeometry (Widget, XtWidgetGeometry *,
/// XtWidgetGeometry *);`
/// C: `<X11/Intrinsic.h>`: `typedef XtGeometryResult (*XtGeometryHandler)(Widget,
/// XtWidgetGeometry *, XtWidgetGeometry *);`
#[cfg (feature = "use_x_toolkit")]
pub type XtGeometryHandler =
  Option<unsafe extern "C" fn (Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry)
                               -> XtGeometryResult>;
/// C 372-376: `static void resize_cb (Widget widget, XtPointer closure, XEvent *event, Boolean
/// *continue_to_dispatch)` - the handler C 402 hands to `XtAddRawEventHandler`.
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtEventHandler)(Widget, XtPointer, XEvent *, Boolean *);`
#[cfg (feature = "use_x_toolkit")]
pub type XtEventHandler =
  Option<unsafe extern "C" fn (Widget, XtPointer, *mut XEvent, *mut Boolean)>;

/* The procedure types whose signature the reference does *not* give: C 111-135's zeros and Xt's own
   `XtInherit*` globals.  They keep Xt's typedef names, with the pointee untyped (P-55), because
   this machine has no Xt header to read the real signatures from.  */
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtProc)(void);` - C 111's zero slot.
#[cfg (feature = "use_x_toolkit")]
pub type XtProc = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtWidgetClassProc)(WidgetClass);` - C 112's zero slot.
#[cfg (feature = "use_x_toolkit")]
pub type XtWidgetClassProc = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtArgsProc)(Widget, XtPointer, XtPointer);` - C 115 and
/// C 135's zero slots.
#[cfg (feature = "use_x_toolkit")]
pub type XtArgsProc = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtArgsFunc)(Widget, XtPointer, XtPointer);` - C 133.
#[cfg (feature = "use_x_toolkit")]
pub type XtArgsFunc = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtSetValuesFunc)(Widget, Widget, Widget, ArgList,
/// Cardinal *);` - C 132's zero slot.
#[cfg (feature = "use_x_toolkit")]
pub type XtSetValuesFunc = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef XtActionList XtActionsList;` - C 117's zero slot.
#[cfg (feature = "use_x_toolkit")]
pub type XtActionList = *mut c_void;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtAlmostProc)(Widget, Widget, XtWidgetGeometry *,
/// XtWidgetGeometry *);` - C 134's `XtInheritSetValuesAlmost`, whose declaration below carries
/// that signature (the value is a function, so the field's type has to be a real one for the class
/// record to compile; the parameter list is Xt's documented one, **UNVERIFIED**).
#[cfg (feature = "use_x_toolkit")]
pub type XtAlmostProc =
  Option<unsafe extern "C" fn (Widget, Widget, *mut XtWidgetGeometry, *mut XtWidgetGeometry)>;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtAcceptFocusProc)(Widget, Time *);` - C 136's
/// `XtInheritAcceptFocus`.
#[cfg (feature = "use_x_toolkit")]
pub type XtAcceptFocusProc = Option<unsafe extern "C" fn (Widget, *mut Time)>;
/// C: `<X11/Intrinsic.h>`: `typedef void (*XtStringProc)(Widget, String);` - C 141's
/// `XtInheritDisplayAccelerator`.
#[cfg (feature = "use_x_toolkit")]
pub type XtStringProc = Option<unsafe extern "C" fn (Widget, String)>;
/// C: `<X11/Intrinsic.h>`: `typedef XtResource *XtResourceList;` - C 119's `resources`.
#[cfg (feature = "use_x_toolkit")]
pub type XtResourceList = *mut XtResource;

/// C 113/122-125: `FALSE` / `TRUE` in the class record.
/// C: `<X11/Intrinsic.h>`: `#define TRUE 1` (its sibling `#define FALSE 0`) - Xt's boolean pair,
/// which is not Xlib's `True`/`False` pair (`include/xterm.rs` 63-69).
#[cfg (feature = "use_x_toolkit")]
pub const TRUE: Boolean = 1;
/// C: `<X11/Intrinsic.h>`: `#define FALSE 0`
#[cfg (feature = "use_x_toolkit")]
pub const FALSE: Boolean = 0;

/// C 302-308: `XtVaSetValues ((Widget) wmshell, …, NULL);`
/// C: `<X11/Intrinsic.h>`: `extern void XtVaSetValues (Widget, ...);` - C's variable-argument
/// setter, declared as C's own C-variadic prototype (Rust allows the declaration and the call).
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  pub fn XtVaSetValues (w: Widget, ...);
}

/// C: `<stddef.h>`: `#define NULL ((void *) 0)` - C 308 and C 402 pass it.  `stddef.h` is not in
/// this unit's include graph, and the port's spelling of a null pointer is `core::ptr::null*`
/// (`lib/include/stddef.in.rs` 75-80's note).
#[cfg (feature = "use_x_toolkit")]
pub const NULL: *mut c_void = core::ptr::null_mut ();

/* C 37 (`<X11/IntrinsicP.h>`): Xt's two by-value instance parts.  C 331-346 reads
   `ew->core.background_pixel`/`ew->core.border_pixel`/`ew->core.width`/`ew->core.height`, and
   C 264-265/340-341/434/456-457 write the width and the height.  Only the members this file reads
   are carried (the `src/haiku.rs` 87-120 reading), so the record's size - and with it
   `offsetof (_EmacsFrameRec, emacs_frame)` - is **not** Xt's; that consequence is named in
   `docs/ports/widget.md` (`## Deviations` 3 and `## Owed` 1).  */
/// C: `<X11/IntrinsicP.h>`: `typedef struct _CorePart { Widget self; WidgetClass widget_class;
/// Widget parent; XrmName xrm_name; Boolean being_destroyed; … Dimension width, height; …
/// Pixel border_pixel; Pixmap background_pixmap; Pixel background_pixel; … } CorePart;` - the
/// four members C 264-265/296-299/331-346/428/456-457 read or write, in C's order as far as this
/// machine can tell.
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct CorePart
{
  pub width: Dimension,
  pub height: Dimension,
  pub background_pixel: Pixel,
  pub border_pixel: Pixel,
}

/// C: `<X11/IntrinsicP.h>`: `typedef struct { WidgetClass superclass; String class_name;
/// Cardinal widget_size; XtProc class_initialize; … XtPointer extension; } CoreClassPart;` - the
/// member names and their order are the ones **C's own initializer carries as comments**
/// (C 107-142 names every column: `superclass` … `extension`), which is the best evidence
/// available when Xt's header is not on the machine; the *types* are Xt's documented typedefs and
/// the layout is **UNVERIFIED**.
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct CoreClassPart
{
  pub superclass: WidgetClass,             // C 108
  pub class_name: String,                  // C 109
  pub widget_size: Cardinal,               // C 110
  pub class_initialize: XtProc,            // C 111
  pub class_part_initialize: XtWidgetClassProc,   // C 112
  pub class_inited: Boolean,               // C 113
  pub initialize: XtInitProc,              // C 114
  pub initialize_hook: XtArgsProc,         // C 115
  pub realize: XtRealizeProc,              // C 116
  pub actions: XtActionList,               // C 117
  pub num_actions: Cardinal,               // C 118
  pub resources: XtResourceList,           // C 119
  pub resource_count: Cardinal,            // C 120
  pub xrm_class: XrmQuark,                 // C 121
  pub compress_motion: Boolean,            // C 122
  pub compress_exposure: XtEnum,           // C 123
  pub compress_enterleave: Boolean,        // C 124
  pub visible_interest: Boolean,           // C 125
  pub destroy: XtWidgetProc,               // C 126
  pub resize: XtWidgetProc,                // C 127
  pub expose: XtExposeProc,                // C 128
  pub set_values: XtSetValuesFunc,         // C 132
  pub set_values_hook: XtArgsFunc,         // C 133
  pub set_values_almost: XtAlmostProc,     // C 134
  pub get_values_hook: XtArgsProc,         // C 135
  pub accept_focus: XtAcceptFocusProc,     // C 136
  pub version: XtVersionType,              // C 137
  pub callback_private: XtPointer,         // C 138
  pub tm_table: String,                    // C 139
  pub query_geometry: XtGeometryHandler,   // C 140
  pub display_accelerator: XtStringProc,   // C 141
  pub extension: XtPointer,                // C 142
}

/* C 137/123/134/136/141: the five Xt-supplied values C's initializer names.  All **UNVERIFIED**
   (no Xt header on this machine): `XtVersion` is a *placeholder*, and the three `XtInherit*`
   globals keep the pointer's size with the pointee untyped (P-55).  The three Xt entry points
   C 301-308 and C 398-402 call follow.  */
/// C 137: `/* version */ XtVersion,`
/// C: `<X11/Intrinsic.h>`: `#define XtVersion <Xt's own release marker>` - version-dependent, so
/// the port carries the placeholder 0 rather than a number written from memory (P-88 clause 2).
#[cfg (feature = "use_x_toolkit")]
pub const XtVersion: XtVersionType = 0;   // placeholder, not Xt's value
/// C 123: `/* compress_exposure */ XtExposeNoCompress,`
/// C: `<X11/Intrinsic.h>`: `XtExposeNoCompress` (an `XtEnum` member) - placeholder, not Xt's value.
#[cfg (feature = "use_x_toolkit")]
pub const XtExposeNoCompress: XtEnum = 0;   // placeholder, not Xt's value
/// C 121: `/* xrm_class */ NULLQUARK,`
/// C: `<X11/Xresource.h>` 85: `#define NULLQUARK ((XrmQuark) 0)` - measured on this machine's
/// `xorgproto`; `XrmQuark` is `include/xlib.rs` 203's.  Its home is that mirror's `x11_constants`
/// module (P-72 clause 1), and the move is owed.
#[cfg (feature = "use_x_toolkit")]
pub const NULLQUARK: XrmQuark = 0;
/// C 134: `/* set_values_almost */ XtInheritSetValuesAlmost,`
/// C: `<X11/Intrinsic.h>`: `extern void XtInheritSetValuesAlmost (Widget, Widget,
/// XtWidgetGeometry *, XtWidgetGeometry *);` - Xt's "inherit the superclass's procedure"
/// *function*, so the class record takes its **address** (`Some (…)`); a `static` would not be
/// readable in a `static` initializer (`E0013`).  **UNVERIFIED**: no Xt header on this machine.
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  pub fn XtInheritSetValuesAlmost (w: Widget, new: Widget,
                                   request: *mut XtWidgetGeometry, reply: *mut XtWidgetGeometry);
  /// C 136: `/* accept_focus */ XtInheritAcceptFocus,`
  /// C: `<X11/Intrinsic.h>`: `extern void XtInheritAcceptFocus (Widget, Time *);`
  pub fn XtInheritAcceptFocus (w: Widget, time: *mut Time);
  /// C 141: `/* display_accelerator */ XtInheritDisplayAccelerator,`
  /// C: `<X11/Intrinsic.h>`: `extern void XtInheritDisplayAccelerator (Widget, String);`
  pub fn XtInheritDisplayAccelerator (w: Widget, string: String);
}

/// C 203-207: `XtParent (w)`, `XtParent (wmshell)` (twice).
/// C: `<X11/Intrinsic.h>`: `extern Widget XtParent (Widget);` - Xt's parent accessor; the port
/// spells it as the function Xt's header declares it as, **UNVERIFIED** (the header is not here).
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  pub fn XtParent (w: Widget) -> Widget;
  /// C 204: `!XtIsWMShell (wmshell)`.
  /// C: `<X11/Intrinsic.h>`: `extern Boolean XtIsWMShell (Widget);` - Xt may spell this one as a
  /// macro over `XtIsSubclass`/`wmShellWidgetClass`, which is not readable here (**UNVERIFIED**),
  /// so the unit declares the accessor C's text calls.
  pub fn XtIsWMShell (w: Widget) -> Boolean;
  /// C 398: `XtCreateWindow (widget, InputOutput, (Visual *) CopyFromParent, *mask, attrs);`
  /// C: `<X11/Intrinsic.h>`: `extern void XtCreateWindow (Widget, unsigned int, Visual *,
  /// XtValueMask, XSetWindowAttributes *);`
  pub fn XtCreateWindow (widget: Widget, window_class: c_uint, visual: *mut Visual,
                         value_mask: XtValueMask, attributes: *mut XSetWindowAttributes);
  /// C 402: `XtAddRawEventHandler (widget, StructureNotifyMask, False, resize_cb, NULL);`
  /// C: `<X11/Intrinsic.h>`: `extern void XtAddRawEventHandler (Widget, EventMask, Boolean,
  /// XtEventHandler, XtPointer);`
  pub fn XtAddRawEventHandler (widget: Widget, event_mask: c_long, nonmaskable: Boolean,
                               handler: XtEventHandler, client_data: XtPointer);
}

/* C 36-37: the Xlib surface `include/xlib.rs` does not carry yet - the four types and the seven
   macro values this file names.  Every number below is **measured** on this machine's `xorgproto`
   2025.1 with `clang` (`docs/ports/widget.md` `## Evidence` ③); `XrmQuark`, `Visual`, `XWMHints`,
   `XID`, `Pixmap`, `Colormap`, `Cursor`, `Bool` and `Display` are that mirror's and are not
   repeated (P-26).  */
/// C 136: the `XtAcceptFocusProc` parameter `Time`.
/// C: `<X11/X.h>` 77: `typedef unsigned long Time;` - **measured** (8 bytes on this machine).
/// `include/xlib.rs` does not carry it yet (its ledger's `## Owed` names `Time` among the bare X11
/// names an X11 unit reaches), and `<X11/Intrinsic.h>` includes `<X11/Xlib.h>`, so an Xt build has
/// this gate too.
#[cfg (feature = "have_x_windows")]
pub type Time = c_ulong;

/// C: `<X11/Intrinsic.h>`: `typedef unsigned long Pixel;` - **UNVERIFIED**: `Pixel` is Xt's, not
/// Xlib's (this machine's `<X11/Xlib.h>` has no such typedef), so its width is the one
/// `include/widgetprv.rs` 139-142 already substitutes (`c_ulong`).
#[cfg (feature = "use_x_toolkit")]
pub type Pixel = c_ulong;

/// C 383-399: `XSetWindowAttributes *attrs` and `attrs->event_mask`/`attrs->bit_gravity`.
/// C: `<X11/Xlib.h>` 288-302: `typedef struct { Pixmap background_pixmap; unsigned long
/// background_pixel; Pixmap border_pixmap; unsigned long border_pixel; int bit_gravity; int
/// win_gravity; int backing_store; unsigned long backing_planes; unsigned long backing_pixel;
/// Bool save_under; long event_mask; long do_not_propagate_mask; Bool override_redirect; Colormap
/// colormap; Cursor cursor; } XSetWindowAttributes;` - all fifteen members are carried, because
/// the two C reads are at offsets the members before them fix (measured
/// `[sizeof=112, event_mask=72, bit_gravity=32]`).
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XSetWindowAttributes
{
  pub background_pixmap: Pixmap,           // Xlib.h 290
  pub background_pixel: c_ulong,           // Xlib.h 291
  pub border_pixmap: Pixmap,               // Xlib.h 292
  pub border_pixel: c_ulong,               // Xlib.h 293
  pub bit_gravity: c_int,                  // Xlib.h 294
  pub win_gravity: c_int,                  // Xlib.h 295
  pub backing_store: c_int,                // Xlib.h 296
  pub backing_planes: c_ulong,             // Xlib.h 297
  pub backing_pixel: c_ulong,              // Xlib.h 298
  pub save_under: Bool,                    // Xlib.h 299
  pub event_mask: c_long,                  // Xlib.h 300
  pub do_not_propagate_mask: c_long,       // Xlib.h 301
  pub override_redirect: Bool,             // Xlib.h 302
  pub colormap: Colormap,                  // Xlib.h 303
  pub cursor: Cursor,                      // Xlib.h 304
}

/// C 499-501: `event->xexpose.x`, `.y`, `.width`, `.height`, `.count`.
/// C: `<X11/Xlib.h>` 656-667: `typedef struct { int type; unsigned long serial; Bool send_event;
/// Display *display; Window window; int x, y; int width, height; int count; } XExposeEvent;` -
/// `include/xlib.rs` 1164 declares `XEvent` as an opaque 192-byte record, so C 499-501's
/// `event->xexpose` member is reached through a conversion to this record (P-72 clause 3).
/// Measured `[sizeof=64, x=40, y=44, width=48, height=52, count=56]`.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XExposeEvent
{
  pub r#type: c_int,                       // Xlib.h 658
  pub serial: c_ulong,                     // Xlib.h 659
  pub send_event: Bool,                    // Xlib.h 660
  pub display: *mut Display,               // Xlib.h 661
  pub window: Window,                      // Xlib.h 662
  pub x: c_int,                            // Xlib.h 663
  pub y: c_int,                            // Xlib.h 664
  pub width: c_int,                        // Xlib.h 665
  pub height: c_int,                       // Xlib.h 666
  pub count: c_int,                        // Xlib.h 667
}

/// C 283/287-288/316-317: `sizeof wmshell->wm.size_hints` and its address.
/// C: `<X11/Xutil.h>` 81-94: the ICCCM normal-hints record (`long flags; int x, y, width, height,
/// min_width, min_height, max_width, max_height, width_inc, height_inc; struct { int x, y; }
/// min_aspect, max_aspect; int base_width, base_height, win_gravity;`).  Measured
/// `[sizeof=80, width_inc=40, height_inc=44, min_width=24, base_width=64]`.
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XSizeHints
{
  pub flags: c_long,                       // Xutil.h 82
  pub x: c_int,                            // Xutil.h 83
  pub y: c_int,                            // Xutil.h 83
  pub width: c_int,                        // Xutil.h 84
  pub height: c_int,                       // Xutil.h 84
  pub min_width: c_int,                    // Xutil.h 85
  pub min_height: c_int,                   // Xutil.h 85
  pub max_width: c_int,                    // Xutil.h 86
  pub max_height: c_int,                   // Xutil.h 86
  pub width_inc: c_int,                    // Xutil.h 87
  pub height_inc: c_int,                   // Xutil.h 87
  pub min_aspect: XSizeHints_aspect,       // Xutil.h 88-91
  pub max_aspect: XSizeHints_aspect,       // Xutil.h 88-91
  pub base_width: c_int,                   // Xutil.h 92
  pub base_height: c_int,                  // Xutil.h 92
  pub win_gravity: c_int,                  // Xutil.h 93
}

/// C: `<X11/Xutil.h>` 88-91: `struct { int x; int y; } min_aspect, max_aspect;` - C's two nested
/// anonymous records, named after their members (PORTING.md 1.3's "嵌套匿名 struct 必须显式命名嵌入").
#[cfg (feature = "have_x_windows")]
#[repr (C)]
pub struct XSizeHints_aspect
{
  pub x: c_int,                            // Xutil.h 89
  pub y: c_int,                            // Xutil.h 90
}

/// C 49/494: `Region` - the third parameter of `EmacsFrameExpose`, which C 494 never reads.
/// C: `<X11/Xutil.h>` 273: `typedef struct _XRegion *Region;`
#[cfg (feature = "have_x_windows")]
pub type Region = *mut _XRegion;
/// C: `<X11/Xregion.h>` 75-80: `typedef struct _XRegion { long size; long numRects; BOX *rects;
/// BOX extents; } REGION;` - the pointee is never read through this type, so it is opaque
/// (`include/xlib.rs` 881-883's `_XDisplay` shape).
#[cfg (feature = "have_x_windows")]
pub struct _XRegion
{
  pub _private: [u8; 0],
}

/// C: `<X11/Xutil.h>` 103: `#define USSize (1L << 1)` - the `XSizeHints.flags` bit the `#if 0`
/// text at C 218 names; carried so the record's flag family is complete (measured 2).
#[cfg (feature = "have_x_windows")]
pub const USSize: c_long = 2;

/* The five `<X11/X.h>` macro values `EmacsFrameRealize` and `EmacsFrameQueryGeometry` use, each
   measured on this machine's `xorgproto` (the file line is the header's).  `PropertyChangeMask`,
   `StructureNotifyMask` and `STANDARD_EVENT_SET` come through `include/xterm.rs` and are not
   repeated.  */
/// C 392-394: `attrs->event_mask = (STANDARD_EVENT_SET | PropertyChangeMask |
/// SubstructureNotifyMask);`
/// C: `<X11/X.h>` 170: `#define SubstructureNotifyMask (1L<<19)`
#[cfg (feature = "have_x_windows")]
pub const SubstructureNotifyMask: c_long = 1 << 19;
/// C 396: `*mask |= CWEventMask;`
/// C: `<X11/X.h>` 403: `#define CWEventMask (1L<<11)`
#[cfg (feature = "have_x_windows")]
pub const CWEventMask: c_long = 1 << 11;
/// C 397: `*mask |= CWBitGravity;`
/// C: `<X11/X.h>` 396: `#define CWBitGravity (1L<<4)`
#[cfg (feature = "have_x_windows")]
pub const CWBitGravity: c_long = 1 << 4;
/// C 450/456/459/461: `mask & CWWidth`
/// C: `<X11/X.h>` 412: `#define CWWidth (1<<2)`
#[cfg (feature = "have_x_windows")]
pub const CWWidth: c_int = 1 << 2;
/// C 450/457/464/466: `mask & CWHeight`
/// C: `<X11/X.h>` 413: `#define CWHeight (1<<3)`
#[cfg (feature = "have_x_windows")]
pub const CWHeight: c_int = 1 << 3;
/// C 398: `XtCreateWindow (widget, InputOutput, …)`
/// C: `<X11/X.h>` 387: `#define InputOutput 1`
#[cfg (feature = "have_x_windows")]
pub const InputOutput: c_int = 1;
/// C 395: `attrs->bit_gravity = NorthWestGravity;`
/// C: `<X11/X.h>` 422: `#define NorthWestGravity 1`
#[cfg (feature = "have_x_windows")]
pub const NorthWestGravity: c_int = 1;

/* C 398's second argument and C 301's first: the two Xlib/Xt names this file takes from
   `include/xlib.rs`'s `x11_constants` module rather than from the crate root (that module is not
   glob-imported, because its `None` would shadow the prelude's - `include/xlib.rs` 33-36).  */
/// C 398: `(Visual *) CopyFromParent`
/// C: `<X11/X.h>` 121: `#define CopyFromParent 0L` (`include/xlib.rs` 70's constant).
#[cfg (feature = "have_x_windows")]
use x11_constants::CopyFromParent;

#[cfg (feature = "use_x_toolkit")]
include!("include/widgetprv.rs");              // C 39: `#include "widgetprv.h"`

/* C 40-42: `#include <X11/ObjectP.h>`, `#include <X11/Shell.h>` and `#include <X11/ShellP.h>`.
   `<X11/ObjectP.h>` declares no name this file uses; `<X11/ShellP.h>` declares the three the
   shell half of the file walks (`get_wm_shell`, `update_wm_hints`).  They have no mirror here, so
   they stand in this unit (P-88 clause 1) with the members the reference reads - `WMShellRec.wm`'s
   offset is therefore not Xt's (C's `CompositePart`/`ShellPart` come before it and are not
   readable here), and `docs/ports/widget.md` names that consequence.  */
/// C: `<X11/ShellP.h>`: `typedef struct { String title; int wm_timeout; Boolean wait_for_wm;
/// Boolean transient; XSizeHints size_hints; XWMHints wm_hints; … } WMShellPart;` - the two
/// members C 283-288/313-317 read and copy, with `wm_hints`'s type coming from
/// `include/xlib.rs` 1100's measured `XWMHints`.
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct WMShellPart
{
  pub size_hints: XSizeHints,
  pub wm_hints: XWMHints,
}
/// C: `<X11/ShellP.h>`: `typedef struct { CorePart core; CompositePart composite; ShellPart
/// shell; WMShellPart wm; } WMShellRec, *WMShellWidget;` - `core` is C's first member (that is
/// what makes C 296-299's `wmshell->core.width` an inline read) and `wm` the last one; the two
/// parts between them are not carried, so `wm`'s offset is **not** Xt's.
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct WMShellRec
{
  pub core: CorePart,
  pub wm: WMShellPart,
}
/// C: `<X11/ShellP.h>`: `typedef struct { … } WMShellRec, *WMShellWidget;`
#[cfg (feature = "use_x_toolkit")]
pub type WMShellWidget = *mut WMShellRec;

#[cfg (feature = "use_x_toolkit")]
include!("../lwlib/include/lwlib.rs");         // C 43: `#include "../lwlib/lwlib.h"`

/* C 45-51: the six `static` prototypes.  Rust has no `static` prototype (PORTING.md 0.1-12,
   P-91 clause 1): C's forward declarations are deleted and each of the six is defined at its own C
   position below.  C's own text, so the signatures they fix stay readable:

     static void EmacsFrameInitialize (Widget, Widget, ArgList, Cardinal *);
     static void EmacsFrameDestroy (Widget);
     static void EmacsFrameRealize (Widget, XtValueMask *, XSetWindowAttributes *);
     static void EmacsFrameResize (Widget);
     static void EmacsFrameExpose (Widget, XEvent *, Region);
     static XtGeometryResult EmacsFrameQueryGeometry (Widget, XtWidgetGeometry *,
                                                      XtWidgetGeometry *);
*/

/// C 54: `#define offset(field) offsetof (EmacsFrameRec, emacs_frame.field)` - the record C names
/// is the tag `_EmacsFrameRec` this unit declares (see the file note), so both the macro and its
/// call sites keep C's spelling with that one substitution.  `offset_of!` is C's `offsetof`
/// (PORTING.md's `PSEUDOVECSIZE`/`VECSIZE` row).
#[cfg (feature = "use_x_toolkit")]
macro_rules! offset {
  ($field:tt) => {
    core::mem::offset_of! (_EmacsFrameRec, emacs_frame.$field) as Cardinal
  };
}

/// C 56-86: `static XtResource resources[] = { … };` - twelve rows in C's order.  The seven
/// `XtResource` columns are the ones C's own positional initializer fills (C 57-85: name, class,
/// type, `sizeof`, the `offset (field)` byte offset, default type, default address), and each
/// value is C's: `sizeof (String)`/`sizeof (Boolean)`/`sizeof (int)`/`sizeof (Pixel)`/
/// `sizeof (XtPointer)` stay `size_of`, and the six `XtR*` names are this unit's `&CStr`
/// constants.  The record's layout is **UNVERIFIED** (no Xt header).
#[cfg (feature = "use_x_toolkit")]
static mut resources: [XtResource; 12] =
[
  /* C 57-58 */ XtResource { resource_name: XtNgeometry.as_ptr () as String,
                             resource_class: XtCGeometry.as_ptr () as String,
                             resource_type: XtRString.as_ptr () as String,
                             resource_size: core::mem::size_of::<String> () as Cardinal,
                             resource_offset: offset! (geometry),
                             default_type: XtRString.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 59-60 */ XtResource { resource_name: XtNiconic.as_ptr () as String,
                             resource_class: XtCIconic.as_ptr () as String,
                             resource_type: XtRBoolean.as_ptr () as String,
                             resource_size: core::mem::size_of::<Boolean> () as Cardinal,
                             resource_offset: offset! (iconic),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: False as XtPointer },
  /* C 62-64 */ XtResource { resource_name: XtNemacsFrame.as_ptr () as String,
                             resource_class: XtCEmacsFrame.as_ptr () as String,
                             resource_type: XtRPointer.as_ptr () as String,
                             resource_size: core::mem::size_of::<XtPointer> () as Cardinal,
                             resource_offset: offset! (frame),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 66-67 */ XtResource { resource_name: XtNminibuffer.as_ptr () as String,
                             resource_class: XtCMinibuffer.as_ptr () as String,
                             resource_type: XtRInt.as_ptr () as String,
                             resource_size: core::mem::size_of::<c_int> () as Cardinal,
                             resource_offset: offset! (minibuffer),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 68-70 */ XtResource { resource_name: XtNunsplittable.as_ptr () as String,
                             resource_class: XtCUnsplittable.as_ptr () as String,
                             resource_type: XtRBoolean.as_ptr () as String,
                             resource_size: core::mem::size_of::<Boolean> () as Cardinal,
                             resource_offset: offset! (unsplittable),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 71-73 */ XtResource { resource_name: XtNinternalBorderWidth.as_ptr () as String,
                             resource_class: XtCInternalBorderWidth.as_ptr () as String,
                             resource_type: XtRInt.as_ptr () as String,
                             resource_size: core::mem::size_of::<c_int> () as Cardinal,
                             resource_offset: offset! (internal_border_width),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 4 as XtPointer },
  /* C 74-75 */ XtResource { resource_name: XtNinterline.as_ptr () as String,
                             resource_class: XtCInterline.as_ptr () as String,
                             resource_type: XtRInt.as_ptr () as String,
                             resource_size: core::mem::size_of::<c_int> () as Cardinal,
                             resource_offset: offset! (interline),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 76-77 */ XtResource { resource_name: XtNforeground.as_ptr () as String,
                             resource_class: XtCForeground.as_ptr () as String,
                             resource_type: XtRPixel.as_ptr () as String,
                             resource_size: core::mem::size_of::<Pixel> () as Cardinal,
                             resource_offset: offset! (foreground_pixel),
                             default_type: XtRString.as_ptr () as String,
                             default_addr: c"XtDefaultForeground".as_ptr () as XtPointer },
  /* C 78-79 */ XtResource { resource_name: XtNcursorColor.as_ptr () as String,
                             resource_class: XtCForeground.as_ptr () as String,
                             resource_type: XtRPixel.as_ptr () as String,
                             resource_size: core::mem::size_of::<Pixel> () as Cardinal,
                             resource_offset: offset! (cursor_color),
                             default_type: XtRString.as_ptr () as String,
                             default_addr: c"XtDefaultForeground".as_ptr () as XtPointer },
  /* C 80-81 */ XtResource { resource_name: XtNbarCursor.as_ptr () as String,
                             resource_class: XtCBarCursor.as_ptr () as String,
                             resource_type: XtRBoolean.as_ptr () as String,
                             resource_size: core::mem::size_of::<Boolean> () as Cardinal,
                             resource_offset: offset! (bar_cursor),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 82-83 */ XtResource { resource_name: XtNvisualBell.as_ptr () as String,
                             resource_class: XtCVisualBell.as_ptr () as String,
                             resource_type: XtRBoolean.as_ptr () as String,
                             resource_size: core::mem::size_of::<Boolean> () as Cardinal,
                             resource_offset: offset! (visual_bell),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
  /* C 84-85 */ XtResource { resource_name: XtNbellVolume.as_ptr () as String,
                             resource_class: XtCBellVolume.as_ptr () as String,
                             resource_type: XtRInt.as_ptr () as String,
                             resource_size: core::mem::size_of::<c_int> () as Cardinal,
                             resource_offset: offset! (bell_volume),
                             default_type: XtRImmediate.as_ptr () as String,
                             default_addr: 0 as XtPointer },
];

/* C 88: `#undef offset` - C ends the macro's life here and this file no longer spells `offset`
   after C 85.  Rust's `macro_rules!` has no `#undef` and needs none: the macro has exactly the
   twelve call sites above, and the ledger records the reading.  */

/* C 90-104: C's commented-out actions table and translations string.  Every line is C's own
   (the class record's C 117-118 and C 139 comments name the two objects).

/*
static XtActionsRec
emacsFrameActionsTable [] = {
  {"keypress",  key_press},
  {"focus_in",  emacs_frame_focus_handler},
  {"focus_out", emacs_frame_focus_handler},
};

static char
emacsFrameTranslations [] = "\
<KeyPress>: keypress()\n\
<FocusIn>:  focus_in()\n\
<FocusOut>: focus_out()\n";
*/
*/

/* C 56-59 of `src/widgetprv.h`: `typedef struct _EmacsFrameRec { CorePart core; EmacsFramePart
   emacs_frame; } EmacsFrameRec;` - declared under C's own **tag**, because `include/widget.rs`
   154-155 spells `EmacsFrame` (this tag's typedef) as `*mut c_void`, and `include/widgetprv.rs`
   150-158 keeps the typedef name itself with `core` substituted by a pointer.  `EmacsFramePart`
   is that mirror's record; `CorePart` is this unit's.  */
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct _EmacsFrameRec
{
  pub core: CorePart,
  pub emacs_frame: EmacsFramePart,
}

/* C 65-68 of `src/widgetprv.h`: `typedef struct _EmacsFrameClassRec { CoreClassPart core_class;
   EmacsFrameClassPart emacs_frame_class; } EmacsFrameClassRec;` - C's tag for the same reason.
   `EmacsFrameClassPart` is `include/widgetprv.rs` 52-56's (the mirror declares it ungated).  */
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct _EmacsFrameClassRec
{
  pub core_class: CoreClassPart,
  pub emacs_frame_class: EmacsFrameClassPart,
}

/* C 106-144: `static EmacsFrameClassRec emacsFrameClassRec = { { … } };` - C's outer brace
   initializes the record's `emacs_frame_class` to zero and the inner one carries `CoreClassPart`'s
   thirty-two positional values, translated to Rust's field-list form.  The `/* … */` comments are
   C's own (they are the only member-name evidence this machine has - see `CoreClassPart`).  */
#[cfg (feature = "use_x_toolkit")]
static mut emacsFrameClassRec: _EmacsFrameClassRec =
  _EmacsFrameClassRec
  {
    core_class: CoreClassPart
    {
      /* core fields */
      /* superclass           */ superclass: core::ptr::null_mut (),                     // C 108
      /* class_name           */ class_name: c"EmacsFrame".as_ptr () as String,          // C 109
      /* widget_size          */ widget_size: core::mem::size_of::<_EmacsFrameRec> () as Cardinal,  // C 110
      /* class_initialize     */ class_initialize: core::ptr::null_mut (),                     // C 111
      /* class_part_initialize*/ class_part_initialize: core::ptr::null_mut (),                     // C 112
      /* class_inited         */ class_inited: FALSE,                                      // C 113
      /* initialize           */ initialize: Some (EmacsFrameInitialize),                // C 114
      /* initialize_hook      */ initialize_hook: core::ptr::null_mut (),                     // C 115
      /* realize              */ realize: Some (EmacsFrameRealize),                   // C 116
      /* actions              */ actions: core::ptr::null_mut (),   /*emacsFrameActionsTable*/   // C 117
      /* num_actions          */ num_actions: 0,  /*XtNumber (emacsFrameActionsTable)*/   // C 118
      /* resources            */ resources: (&raw mut resources) as XtResourceList,     // C 119
      /* resource_count       */ resource_count: (core::mem::size_of::<[XtResource; 12]> ()
                                  / core::mem::size_of::<XtResource> ()) as Cardinal,   // C 120
      /* xrm_class            */ xrm_class: NULLQUARK,                                  // C 121
      /* compress_motion      */ compress_motion: TRUE,                                       // C 122
      /* compress_exposure    */ compress_exposure: XtExposeNoCompress,                         // C 123
      /* compress_enterleave  */ compress_enterleave: TRUE,                                       // C 124
      /* visible_interest     */ visible_interest: FALSE,                                      // C 125
      /* destroy              */ destroy: Some (EmacsFrameDestroy),                   // C 126
      /* resize               */ resize: Some (EmacsFrameResize),                    // C 127
      /* expose               */ expose: Some (EmacsFrameExpose),                    // C 128

      /* Emacs never does XtSetvalues on this widget, so we have no code
         for it. */
      /* set_values           */ set_values: core::ptr::null_mut (), /* Not supported */   // C 132
      /* set_values_hook      */ set_values_hook: core::ptr::null_mut (),                     // C 133
      /* set_values_almost    */ set_values_almost: Some (XtInheritSetValuesAlmost),            // C 134
      /* get_values_hook      */ get_values_hook: core::ptr::null_mut (),                     // C 135
      /* accept_focus         */ accept_focus: Some (XtInheritAcceptFocus),                // C 136
      /* version              */ version: XtVersion,                                  // C 137
      /* callback_private     */ callback_private: core::ptr::null_mut (),                     // C 138
      /* tm_table             */ tm_table: core::ptr::null_mut (), /*emacsFrameTranslations*/   // C 139
      /* query_geometry       */ query_geometry: Some (EmacsFrameQueryGeometry),             // C 140
      /* display_accelerator  */ display_accelerator: Some (XtInheritDisplayAccelerator),         // C 141
      /* extension            */ extension: core::ptr::null_mut (),                     // C 142
    },
    emacs_frame_class: EmacsFrameClassPart { dummy: 0 },   /* C 143's outer `}` */
  };

/* ----------------------------------------------------------------------------------------------
   The calls this file makes whose declarations the mirrors it includes do not carry (P-72 rule 4:
   the unit declares them where C's text uses them, and `docs/ports/widget.md` owes the move to the
   mirror each one belongs to).  Each keeps C's prototype on its `/// C:` line, read from the
   header that declares it.  ------------------------------------------------------------------- */
/// C 365: `exit (1);`
/// C: `<stdlib.h>`: `_Noreturn void exit (int);` - `lib/include/stdlib.rs` carries the allocator and
/// the `strto*` family but not `exit` (the `src/emacs.rs` 320-321 reading).
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
  /// C 434: `change_frame_size (f, ew->core.width, ew->core.height, false, true, false);`
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                            pretend: bool, delay: bool, safe: bool);
  /// C 441: `cancel_mouse_face (f);`
  /// C: dispextern.h 3690: `extern void cancel_mouse_face (struct frame *);`
  pub fn cancel_mouse_face (f: *mut frame);



  /// C 489-490: `x_set_window_size (f, 0, …);`
  /// C: xterm.h 1770: `extern void x_set_window_size (struct frame *, bool, int, int);`
  pub fn x_set_window_size (f: *mut frame, change_gravity: bool,
                            width: c_int, height: c_int);
  /// C 499: `expose_frame (f, event->xexpose.x, …);`
  /// C: dispextern.h 3675: `extern void expose_frame (struct frame *, int, int, int, int);`
  pub fn expose_frame (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);
  /// C 502: `x_clear_under_internal_border (f);`
  /// C: xterm.h 1900: `extern void x_clear_under_internal_border (struct frame *f);`
  pub fn x_clear_under_internal_border (f: *mut frame);
}

/// C 146-155: `WidgetClass emacsFrameClass (void)` - the file's first non-`static` definition, so
/// it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  `widget.h` 93's `extern WidgetClass
/// emacsFrameClass (void);` is `include/widget.rs` 159-164's declaration; a crate-root definition
/// next to it is `E0428` (P-39) and the fix is the mirror's, recorded in `## Owed`.
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacsFrameClass () -> WidgetClass
{
  /* Set the superclass here rather than relying on static
     initialization, to work around an unexelf.c bug on x86 platforms
     that use the GNU Gold linker (Bug#27248).  */
  emacsFrameClassRec.core_class.superclass = &raw mut widgetClassRec;   // C 152

  return (&raw mut emacsFrameClassRec) as WidgetClass;                  // C 154
}

/// C 157-164: `static void get_default_char_pixel_size (EmacsFrame ew, int *pixel_width, int
/// *pixel_height)` - `ew->emacs_frame.frame`, the read every function in this file opens with, is
/// written `(*(ew as *mut _EmacsFrameRec))` at each site: `EmacsFrame` is `include/widget.rs`
/// 154-155's untyped pointer, and C's record is this unit's `_EmacsFrameRec` (the file note).
#[cfg (feature = "use_x_toolkit")]
unsafe fn get_default_char_pixel_size (ew: EmacsFrame, pixel_width: *mut c_int,
                                       pixel_height: *mut c_int)
{
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 160

  *pixel_width = FRAME_COLUMN_WIDTH (f);                               // C 162
  *pixel_height = FRAME_LINE_HEIGHT (f);                               // C 163
}

/// C 166-174: `static void pixel_to_char_size (EmacsFrame ew, Dimension pixel_width, Dimension
/// pixel_height, int *char_width, int *char_height)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn pixel_to_char_size (ew: EmacsFrame, pixel_width: Dimension, pixel_height: Dimension,
                              char_width: *mut c_int, char_height: *mut c_int)
{
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 170

  *char_width = FRAME_PIXEL_WIDTH_TO_TEXT_COLS (f, pixel_width as c_int);       // C 172
  *char_height = FRAME_PIXEL_HEIGHT_TO_TEXT_LINES (f, pixel_height as c_int);   // C 173
}

/// C 176-184: `static void char_to_pixel_size (EmacsFrame ew, int char_width, int char_height,
/// Dimension *pixel_width, Dimension *pixel_height)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn char_to_pixel_size (ew: EmacsFrame, char_width: c_int, char_height: c_int,
                              pixel_width: *mut Dimension, pixel_height: *mut Dimension)
{
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 180

  *pixel_width = FRAME_TEXT_COLS_TO_PIXEL_WIDTH (f, char_width) as Dimension;    // C 182
  *pixel_height = FRAME_TEXT_LINES_TO_PIXEL_HEIGHT (f, char_height) as Dimension; // C 183
}

/// C 186-196: `static void round_size_to_char (EmacsFrame ew, Dimension in_width, Dimension
/// in_height, Dimension *out_width, Dimension *out_height)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn round_size_to_char (ew: EmacsFrame, in_width: Dimension, in_height: Dimension,
                              out_width: *mut Dimension, out_height: *mut Dimension)
{
  let mut char_width: c_int = 0;      // C 190 (C leaves it uninitialized; PORTING.md 1.5)
  let mut char_height: c_int = 0;     // C 191
  pixel_to_char_size (ew, in_width, in_height,
                      &raw mut char_width, &raw mut char_height);     // C 192-193
  char_to_pixel_size (ew, char_width, char_height,
                      out_width, out_height);                         // C 194-195
}

/// C 198-208: `static WMShellWidget get_wm_shell (Widget w)` - C's `for (wmshell = XtParent (w);
/// wmshell && !XtIsWMShell (wmshell); wmshell = XtParent (wmshell));` has an empty body, so the
/// init is hoisted and the loop is a `while`.
#[cfg (feature = "use_x_toolkit")]
unsafe fn get_wm_shell (w: Widget) -> WMShellWidget
{
  let mut wmshell: Widget = XtParent (w);                              // C 201-203

  while !wmshell.is_null () && XtIsWMShell (wmshell) == 0              // C 204
    { wmshell = XtParent (wmshell); }                                  // C 205

  return wmshell as WMShellWidget;                                     // C 207
}

/* C 210-221: `#if 0` / `static void mark_shell_size_user_specified (Widget wmshell)` / its body /
   `#endif` - dead text C itself excludes (C's other arm is the empty `#else`).  Recorded as a
   comment; every line is C's own.

#if 0 /* Currently not used.  */

static void
mark_shell_size_user_specified (Widget wmshell)
{
  if (! XtIsWMShell (wmshell)) emacs_abort ();
  /* This is kind of sleazy, but I can't see how else to tell it to make it
     mark the WM_SIZE_HINTS size as user specified when appropriate. */
  ((WMShellWidget) wmshell)->wm.size_hints.flags |= USSize;
}

#endif
*/

/// C 224-270: `static void set_frame_size (EmacsFrame ew)` - C 227-260's ASCII widget-hierarchy
/// note is C's own text and stands below.
#[cfg (feature = "use_x_toolkit")]
unsafe fn set_frame_size (ew: EmacsFrame)
{
  /* The widget hierarchy is

	argv[0]			emacsShell	pane	Frame-NAME
	ApplicationShell	EmacsShell	Paned	EmacsFrame

     We accept geometry specs in this order:

	*Frame-NAME.geometry
	*EmacsFrame.geometry
	Emacs.geometry

     Other possibilities for widget hierarchies might be

	argv[0]			frame		pane	Frame-NAME
	ApplicationShell	EmacsShell	Paned	EmacsFrame
     or
	argv[0]			Frame-NAME	pane	Frame-NAME
	ApplicationShell	EmacsShell	Paned	EmacsFrame
     or
	argv[0]			Frame-NAME	pane	emacsTextPane
	ApplicationShell	EmacsFrame	Paned	EmacsTextPane

     With the current setup, the text-display-area is the part which is
     an emacs "frame", since that's the only part managed by emacs proper
     (the menubar and the parent of the menubar and all that sort of thing
     are managed by lwlib.)

     The EmacsShell widget is simply a replacement for the Shell widget
     which is able to deal with using an externally-supplied window instead
     of always creating its own.  It is not actually emacs specific, and
     should possibly have class "Shell" instead of "EmacsShell" to simplify
     the resources.

   */

  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 262

  (*(ew as *mut _EmacsFrameRec)).core.width = FRAME_PIXEL_WIDTH (f) as Dimension;    // C 264
  (*(ew as *mut _EmacsFrameRec)).core.height = FRAME_PIXEL_HEIGHT (f) as Dimension;  // C 265

  if CONSP (frame_size_history)                                        // C 267
    { frame_size_history_plain
        (f, build_string (c"set_frame_size".as_ptr ())); }             // C 268-269
}

/// C 272-318: `static bool update_wm_hints (WMShellWidget wmshell, EmacsFrame ew)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn update_wm_hints (wmshell: WMShellWidget, ew: EmacsFrame) -> bool
{
  let mut cw: c_int = 0;                        // C 275 (C leaves the eight locals uninitialized)
  let mut ch: c_int = 0;                        // C 276
  let mut rounded_width: Dimension = 0;         // C 277
  let mut rounded_height: Dimension = 0;        // C 278
  let mut char_width: c_int = 0;                // C 279
  let mut char_height: c_int = 0;               // C 280
  let mut base_width: c_int = 0;                // C 281
  let mut base_height: c_int = 0;               // C 282
  let mut buffer: [c_char; core::mem::size_of::<XSizeHints> ()] =
    [0; core::mem::size_of::<XSizeHints> ()];   // C 283
  let mut hints_ptr: *mut c_char = std::ptr::null_mut ();   // C 284

  /* Copy the old size hints to the buffer.  */
  memcpy (buffer.as_mut_ptr () as *mut c_void,                          // C 287
          (&raw const (*wmshell).wm.size_hints) as *const c_void,       // C 287
          core::mem::size_of::<XSizeHints> ());                         // C 288

  pixel_to_char_size (ew, (*(ew as *mut _EmacsFrameRec)).core.width,
                      (*(ew as *mut _EmacsFrameRec)).core.height,
                      &raw mut char_width, &raw mut char_height);       // C 290-291
  char_to_pixel_size (ew, char_width, char_height,
                      &raw mut rounded_width, &raw mut rounded_height); // C 292-293
  get_default_char_pixel_size (ew, &raw mut cw, &raw mut ch);           // C 294

  base_width = ((*wmshell).core.width as c_int)                         // C 296
		- ((*(ew as *mut _EmacsFrameRec)).core.width as c_int)  // C 296
		+ (rounded_width as c_int - (char_width * cw));         // C 297
  base_height = ((*wmshell).core.height as c_int)                       // C 298
		 - ((*(ew as *mut _EmacsFrameRec)).core.height as c_int) // C 298
		 + (rounded_height as c_int - (char_height * ch));      // C 299

  XtVaSetValues (wmshell as Widget,                                     // C 301
		 XtNbaseWidth.as_ptr (), base_width as XtArgVal,        // C 302
		 XtNbaseHeight.as_ptr (), base_height as XtArgVal,      // C 303
		 XtNwidthInc.as_ptr (),
                 (if frame_resize_pixelwise { 1 } else { cw }) as XtArgVal,   // C 304
		 XtNheightInc.as_ptr (),
                 (if frame_resize_pixelwise { 1 } else { ch }) as XtArgVal,   // C 305
		 XtNminWidth.as_ptr (), base_width as XtArgVal,         // C 306
		 XtNminHeight.as_ptr (), base_height as XtArgVal,       // C 307
		 NULL);                                                 // C 308

  /* Return if size hints really changed.  If they did not, then Xt
     probably didn't set them either (or take the flags into
     account.)  */
  hints_ptr = (&raw const (*wmshell).wm.size_hints) as *mut c_char;     // C 313

  /* Skip flags, which is unsigned long.  */
  return 0 != memcmp (hints_ptr.add (core::mem::size_of::<c_long> ()) as *const c_void,  // C 316
                      buffer.as_ptr ().add (core::mem::size_of::<c_long> ()) as *const c_void,
                      core::mem::size_of_val (&(*wmshell).wm.wm_hints)
                        - core::mem::size_of::<c_long> ());             // C 317
}

/// C 320-324: `bool widget_update_wm_size_hints (Widget widget, Widget frame)`
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widget_update_wm_size_hints (widget: Widget, frame: Widget) -> bool
{
  return update_wm_hints (widget as WMShellWidget, frame as EmacsFrame);   // C 323
}

/// C 326-332: `static void update_various_frame_slots (EmacsFrame ew)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn update_various_frame_slots (ew: EmacsFrame)
{
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 329

  (*f).internal_border_width =
    (*(ew as *mut _EmacsFrameRec)).emacs_frame.internal_border_width;   // C 331
}

/// C 334-354: `static void update_from_various_frame_slots (EmacsFrame ew)`
#[cfg (feature = "use_x_toolkit")]
unsafe fn update_from_various_frame_slots (ew: EmacsFrame)
{
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 337
  let x = (*f).output_data.x as *mut x_output;                          // C 338

  (*(ew as *mut _EmacsFrameRec)).core.height =
    (FRAME_PIXEL_HEIGHT (f) - (*x).menubar_height) as Dimension;        // C 340
  (*(ew as *mut _EmacsFrameRec)).core.width = FRAME_PIXEL_WIDTH (f) as Dimension;  // C 341
  (*(ew as *mut _EmacsFrameRec)).core.background_pixel = FRAME_BACKGROUND_PIXEL (f);  // C 342
  (*(ew as *mut _EmacsFrameRec)).emacs_frame.internal_border_width =
    (*f).internal_border_width;                                         // C 343
  (*(ew as *mut _EmacsFrameRec)).emacs_frame.foreground_pixel = FRAME_FOREGROUND_PIXEL (f);  // C 344
  (*(ew as *mut _EmacsFrameRec)).emacs_frame.cursor_color = (*x).cursor_pixel;  // C 345
  (*(ew as *mut _EmacsFrameRec)).core.border_pixel = (*x).border_pixel;         // C 346

  if CONSP (frame_size_history)                                         // C 348
    { frame_size_history_extra
        (f, build_string (c"update_from_various_frame_slots".as_ptr ()),  // C 349-350
         FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),                 // C 351
         (*(ew as *mut _EmacsFrameRec)).core.width as c_int,            // C 352
         (*(ew as *mut _EmacsFrameRec)).core.height as c_int,           // C 352
         (*f).new_width, (*f).new_height); }                            // C 353
}

/// C 356-370: `static void EmacsFrameInitialize (Widget request, Widget new, ArgList dum1,
/// Cardinal *dum2)` - the class record's `initialize` slot.
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameInitialize (request: Widget, new: Widget, dum1: ArgList,
                                           dum2: *mut Cardinal)
{
  let ew = new as EmacsFrame;                                           // C 360

  if (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame.is_null ()        // C 362
    {
      fputs (c"can't create an emacs frame widget without a frame\n".as_ptr (), stderr);  // C 364
      exit (1);                                                         // C 365
    }

  update_from_various_frame_slots (ew);                                 // C 368
  set_frame_size (ew);                                                  // C 369
}

/// C 372-379: `static void resize_cb (Widget widget, XtPointer closure, XEvent *event, Boolean
/// *continue_to_dispatch)` - C 402 hands it to Xt as a raw handler, so its type is `extern "C"`
/// (PORTING.md 1.4).
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn resize_cb (widget: Widget, closure: XtPointer, event: *mut XEvent,
                                continue_to_dispatch: *mut Boolean)
{
  EmacsFrameResize (widget);                                            // C 378
}

/// C 382-410: `static void EmacsFrameRealize (Widget widget, XtValueMask *mask,
/// XSetWindowAttributes *attrs)`
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameRealize (widget: Widget, mask: *mut XtValueMask,
                                        attrs: *mut XSetWindowAttributes)
{
  let ew = widget as EmacsFrame;                                        // C 386
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 387

  /* This used to contain SubstructureRedirectMask, but this turns out
     to be a problem with XIM on Solaris, and events from that mask
     don't seem to be used.  Let's check that.  */
  (*attrs).event_mask = (STANDARD_EVENT_SET                             // C 392
			       | PropertyChangeMask                     // C 393
			       | SubstructureNotifyMask);               // C 394
  (*attrs).bit_gravity = NorthWestGravity;                              // C 395
  *mask |= CWEventMask as XtValueMask;                                  // C 396
  *mask |= CWBitGravity as XtValueMask;                                 // C 397
  XtCreateWindow (widget, InputOutput as c_uint, CopyFromParent as *mut Visual, *mask,
		  attrs);                                               // C 398-399
  /* Some ConfigureNotify events does not end up in EmacsFrameResize so
     make sure we get them all.  Seen with xfcwm4 for example.  */
  XtAddRawEventHandler (widget, StructureNotifyMask, False, Some (resize_cb),
			NULL as XtPointer);                             // C 402

  if CONSP (frame_size_history)                                         // C 404
    { frame_size_history_plain
        (f, build_string (c"EmacsFrameRealize".as_ptr ())); }           // C 405-406

  if !get_wm_shell (widget).is_null ()                                  // C 408
    { update_wm_hints (get_wm_shell (widget), ew); }                    // C 409
}

/// C 412-416: `static void EmacsFrameDestroy (Widget widget)`
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameDestroy (widget: Widget)
{
  /* All GCs are now freed in x_free_frame_resources.  */               // C 415
}

/// C 418-442: `static void EmacsFrameResize (Widget widget)`
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameResize (widget: Widget)
{
  let ew = widget as EmacsFrame;                                        // C 421
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 422

  if CONSP (frame_size_history)                                         // C 424
    { frame_size_history_extra
        (f, build_string (c"EmacsFrameResize".as_ptr ()),               // C 425-426
	 FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),                 // C 427
	 (*(ew as *mut _EmacsFrameRec)).core.width as c_int,            // C 428
	 (*(ew as *mut _EmacsFrameRec)).core.height as c_int,           // C 428
	 (*f).new_width, (*f).new_height); }                            // C 429

  if !FRAME_PARENT_FRAME (f).is_null () 	/* Size always up to date.  */    // C 431
    { return; }                                                         // C 432

  change_frame_size (f, (*(ew as *mut _EmacsFrameRec)).core.width as c_int,
		     (*(ew as *mut _EmacsFrameRec)).core.height as c_int,
		     false, true, false);                               // C 434-435

  if !get_wm_shell (widget).is_null ()                                  // C 437
    { update_wm_hints (get_wm_shell (widget), ew); }                    // C 438
  update_various_frame_slots (ew);                                      // C 439

  cancel_mouse_face (f);                                                // C 441
}

/// C 444-471: `static XtGeometryResult EmacsFrameQueryGeometry (Widget widget, XtWidgetGeometry
/// *request, XtWidgetGeometry *result)` - the class record's `query_geometry` slot.
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameQueryGeometry (widget: Widget, request: *mut XtWidgetGeometry,
					      result: *mut XtWidgetGeometry)
  -> XtGeometryResult
{
  let mut mask: c_int = (*request).request_mode as c_int;               // C 448

  if (mask & (CWWidth | CWHeight)) != 0 && !frame_resize_pixelwise      // C 450
    {
      let ew = widget as EmacsFrame;                                    // C 452
      let mut ok_width: Dimension = 0;                                  // C 453 (C: uninitialized)
      let mut ok_height: Dimension = 0;                                 // C 453

      round_size_to_char (ew,                                           // C 455
			  if (mask & CWWidth) != 0 { (*request).width }
			    else { (*(ew as *mut _EmacsFrameRec)).core.width },   // C 456
			  if (mask & CWHeight) != 0 { (*request).height }
			    else { (*(ew as *mut _EmacsFrameRec)).core.height },  // C 457
			  &raw mut ok_width, &raw mut ok_height);           // C 458
      if (mask & CWWidth) != 0 && ok_width != (*request).width          // C 459
	{
	  (*result).request_mode |= CWWidth as XtGeometryMask;          // C 461
	  (*result).width = ok_width;                                   // C 462
	}
      if (mask & CWHeight) != 0 && ok_height != (*request).height       // C 464
	{
	  (*result).request_mode |= CWHeight as XtGeometryMask;         // C 466
	  (*result).height = ok_height;                                 // C 467
	}
    }
  return if (*result).request_mode != 0 { XtGeometryAlmost } else { XtGeometryYes };   // C 470
}

/* Special entry points */                                              // C 473

/// C 474-491: `void EmacsFrameSetCharSize (Widget widget, int columns, int rows)` - non-`static`,
/// so `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EmacsFrameSetCharSize (widget: Widget, columns: c_int, rows: c_int)
{
  let ew = widget as EmacsFrame;                                        // C 477
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 478

  if CONSP (frame_size_history)                                         // C 480
    { frame_size_history_extra
        (f, build_string (c"EmacsFrameSetCharSize".as_ptr ()),          // C 481-482
	 FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),                 // C 483
	 columns, rows,                                                 // C 484
	 (*f).new_width, (*f).new_height); }                            // C 485

  if !frame_inhibit_resize (f, false, Qfont ())                            // C 487
      && !frame_inhibit_resize (f, true, Qfont ())                         // C 488
    { x_set_window_size (f, false, columns * FRAME_COLUMN_WIDTH (f),
			 rows * FRAME_LINE_HEIGHT (f)); }               // C 489-490
}

/* C 503: `flush_frame (f);` - frame.h 1914-1921's INLINE, whose port is `include/frame.rs`
   1946-1954's `flush_frame!`.  That macro cannot type-check in any unit: its body assigns
   `FRAME_RIF! (f)` - `include/termhooks.rs` 425, whose `rif` that mirror degrades to `*mut c_void`
   - to `*mut redisplay_interface`, and then calls the interface member with the same `*mut frame`
   while `include/dispextern.rs` 3703/3717 declares that member as `*mut c_void` and keeps the
   whole record behind `dispextern_face_records`, a switch no configuration sets.  The unit
   therefore writes C's own body as a unit-local `flush_frame` and declares the one member it reads,
   the `src/androidterm.rs` 111-145 shape (measured there as `E0308 expected *mut
   redisplay_interface, found *mut c_void`); the ten members C declares before `flush_display` are
   all pointers, so the prefix is an opaque array and the one offset that matters is pinned.  */
/// C: dispextern.h 3067-3200: `struct redisplay_interface` - **UNVERIFIED** beyond the one member
/// and the offset this file reads (the record is a unit-side use-site copy, P-72 rule 4).
#[cfg (feature = "use_x_toolkit")]
#[repr (C)]
pub struct redisplay_interface
{
  _prefix: [*mut c_void; 9],
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,
}
#[cfg (feature = "use_x_toolkit")]
const _: () = { assert! (core::mem::offset_of! (redisplay_interface, flush_display) == 72); };

/// C: frame.h 1914-1921: `INLINE void flush_frame (struct frame *f) { struct
/// redisplay_interface *rif = FRAME_RIF (f); if (rif && rif->flush_display) rif->flush_display
/// (f); }` - C's own body, written unit-local because the mirror's macro cannot expand.
#[cfg (feature = "use_x_toolkit")]
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface =
    (*(*f).terminal).rif as *mut redisplay_interface;   /* C 1916: `FRAME_RIF (f)`.  */

  if !rif.is_null () && (*rif).flush_display.is_some ()          // C 1918
    { ((*rif).flush_display).unwrap () (f); }                   // C 1919
}

/// C 493-504: `static void EmacsFrameExpose (Widget widget, XEvent *event, Region region)` - the
/// class record's `expose` slot.  `include/xlib.rs` 1164 declares `XEvent` as an opaque record, so
/// C 499-501's `event->xexpose` member is reached through a conversion to this unit's
/// `XExposeEvent` (P-72 clause 3).
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn EmacsFrameExpose (widget: Widget, event: *mut XEvent, region: Region)
{
  let ew = widget as EmacsFrame;                                        // C 496
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 497

  expose_frame (f, (*(event as *mut XExposeEvent)).x,                   // C 499
		(*(event as *mut XExposeEvent)).y,                      // C 499
		(*(event as *mut XExposeEvent)).width,                  // C 500
		(*(event as *mut XExposeEvent)).height);                // C 500
  if (*(event as *mut XExposeEvent)).count == 0                         // C 501
    { x_clear_under_internal_border (f); }                              // C 502
  flush_frame (f);                                                      // C 503
}

/// C 507-514: `void widget_store_internal_border (Widget widget)` - non-`static`, so
/// `#[unsafe(no_mangle)]` (PORTING.md 0.2).
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widget_store_internal_border (widget: Widget)
{
  let ew = widget as EmacsFrame;                                        // C 510
  let f = (*(ew as *mut _EmacsFrameRec)).emacs_frame.frame;             // C 511

  (*(ew as *mut _EmacsFrameRec)).emacs_frame.internal_border_width =
    (*f).internal_border_width;                                         // C 513
}
