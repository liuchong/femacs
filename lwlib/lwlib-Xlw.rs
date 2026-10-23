// femacs: port of lwlib/lwlib-Xlw.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2).
//
// **The file gate is `feature = "use_lucid"`** (P-86 clause 8): the reference builds
// `lwlib-Xlw.o` only in the Lucid toolkit's object list (`lwlib/Makefile.in` 52:
// `LUCID_OBJS = lwlib-Xlw.o xlwmenu.o lwlib-Xaw.o`, with `configure.ac` 7231's
// `AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])`).  `USE_LUCID`,
// `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so
// the frozen configuration compiles none of this file, and **this machine has no X11 headers**:
// the file is `text-only` (P-86 clause 3).
//
// C 34-58's `#if 0` block (`x_print_complete_resource_name`) is dead text and is **not**
// transcribed (the `src/xfns.rs` 958 precedent); `docs/ports/lwlib-Xlw.md` records it.  C's
// `#ifdef emacs` (C 140-144) is the build's own `-Demacs`, which is always defined for this file,
// so its four `XtSetArg` lines are live here.
//
// The `<X11/*>` surface this file reads is declared in this unit (P-88 clause 1: no mirror in
// this tree carries it and this file is its only consumer), with the shapes **UNVERIFIED** -
// there is no X11 header here to measure them against (the `src/haiku.rs` / `include/gtk.rs`
// reading).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, unused_mut,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("../src/include/config.rs");           // C 21: `#include <config.h>`
#[cfg (feature = "use_lucid")]
include!("../src/include/lisp.rs");             // C 24: `#include "lisp.h"`
#[cfg (feature = "use_lucid")]
include!("include/lwlib-Xlw.rs");               // C 26: `#include "lwlib-Xlw.h"`
#[cfg (feature = "use_lucid")]
include!("include/lwlib.rs");                   // C 30: `#include "lwlib.h"` (`widget_instance`, `widget_value`)
#[cfg (feature = "use_lucid")]
include!("include/lwlib-int.rs");               // lwlib.h's own chain: `widget_creation_entry`, `lw_get_widget_value_for_widget`
#[cfg (feature = "use_lucid")]
include!("include/xlwmenu.rs");                 // C 32: `#include "xlwmenu.h"`
#[cfg (feature = "use_lucid")]
include!("include/xlwmenuP.rs");                // xlwmenu.h's own chain: `_XlwMenuRec`, `_XlwMenuClassRec`
/* C 27-31's `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/CompositeP.h>`
   and `<X11/Shell.h>` are the surface below: no mirror in this tree carries them (the `src/xfns.rs`
   /`src/xterm.rs` X11 units declare the same names for their own translation units).  */

/* ------------------------------------------------------------------------------------------
   `<X11/Intrinsic.h>` and friends.  **C's spellings, not measurements** - no X11 header is on
   this machine, so every width and layout below is `UNVERIFIED` (P-86 clause 3(2)).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "use_lucid")]
pub type XtPointer = *mut c_void;                     // C: `typedef char *XtPointer;`
#[cfg (feature = "use_lucid")]
pub type String = *mut c_char;                        // C: `typedef char *String;` (X11's)
#[cfg (feature = "use_lucid")]
pub type Boolean = c_int;                             // C: `typedef int Boolean;`
#[cfg (feature = "use_lucid")]
pub const False: Boolean = 0;
#[cfg (feature = "use_lucid")]
pub const True: Boolean = 1;
/* The X11 scalars and opaque handles the lwlib and xlwmenu mirrors name and leave to their
   unit (their own header comments say so): `Window` (above), `Pixmap`, `Cursor`, `GC`, `Pixel`,
   `Position`, `Dimension` and the opaque `XFontStruct`/`CoreClassPart`/`XtCallbackList`.  */
#[cfg (feature = "use_lucid")]
pub type XID = c_ulong;                               // C: X.h 66: `typedef unsigned long XID;`
#[cfg (feature = "use_lucid")]
pub type Pixmap = XID;                                // C: X.h 78: `typedef XID Pixmap;`
#[cfg (feature = "use_lucid")]
pub type Cursor = XID;                                // C: X.h: `typedef XID Cursor;`
#[cfg (feature = "use_lucid")]
pub type GC = *mut c_void;                            // C: Xlib.h: `typedef struct _XGC *GC;`
#[cfg (feature = "use_lucid")]
pub type Pixel = c_ulong;                             // C: X.h: `typedef unsigned long Pixel;`
#[cfg (feature = "use_lucid")]
pub type Position = c_int;                            // C: X.h: `typedef int Position;`
#[cfg (feature = "use_lucid")]
pub type Dimension = c_ushort;                        // C: X.h: `typedef unsigned short Dimension;`
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XFontStruct { pub _private: [u8; 0] }      // C: Xlib.h: `typedef struct _XFontStruct { … } XFontStruct;`
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CoreClassPart { pub _private: [u8; 0] }    // C: IntrinsicP.h's `CoreClassPart`
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XtCallbackRec
{                                                     // C: Intrinsic.h: `typedef struct _XtCallbackRec { … } XtCallbackRec;`
  pub callback: XtCallbackProc,
  pub closure: XtPointer,
}
#[cfg (feature = "use_lucid")]
pub type XtCallbackList = *mut XtCallbackRec;         // C: `typedef struct _XtCallbackRec *XtCallbackList;`

#[cfg (feature = "use_lucid")]
pub const ButtonPress: c_int = 4;                     // C: X.h's `ButtonPress`
#[cfg (feature = "use_lucid")]
pub const CurrentTime: c_ulong = 0;                   // C: X.h's `#define CurrentTime 0L` (Rust has no `L` suffix)

/// C: X.h: `typedef XID Window;` (`typedef unsigned long XID;`) - `lwlib.rs` names `Window` and
/// leaves it to the unit that includes it (its own header comment).
#[cfg (feature = "use_lucid")]
pub type Window = c_ulong;
/// C: Intrinsic.h: `typedef struct _WidgetRec *Widget;` - the record carries only the members
/// this file reads (`core.being_destroyed`), in C's order, and its offsets are **UNVERIFIED**.
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
  pub being_destroyed: Boolean,                       // C: `Boolean being_destroyed;`
}
#[cfg (feature = "use_lucid")]
pub type Widget = *mut WidgetRec;
#[cfg (feature = "use_lucid")]
pub type WidgetClass = *mut c_void;                   // C: `typedef struct _WidgetClassRec *WidgetClass;`
/// C: Intrinsic.h: `typedef struct { XtResourceList ... } CompositePart;` - the members this file
/// reads are `num_children` and `children`; the record's offsets are **UNVERIFIED**.
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CompositePart
{
  pub children: *mut Widget,                          // C: `WidgetList children;`
  pub num_children: c_uint,                           // C: `Cardinal num_children;`
}
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct CompositeRec
{
  pub core: CorePart,
  pub composite: CompositePart,
}
#[cfg (feature = "use_lucid")]
pub type CompositeWidget = *mut CompositeRec;
/// C: Shell.h: `extern WidgetClass overrideShellWidgetClass;`
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  pub static mut overrideShellWidgetClass: WidgetClass;
}
/// C: Intrinsic.h: `typedef struct { String name; XtArgVal value; } Arg, *ArgList;` - C's macro
/// `XtSetArg (arg, n, d)` is `((arg).name = (n), (arg).value = (XtArgVal) (d))`, so the port
/// carries `XtArgVal` as `XtPointer`.
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

/// C: Xlib.h: `typedef struct { int type; ... } XButtonPressedEvent;` - the members this file
/// writes are carried in C's order (`type`, `serial`, `send_event`, `display`, `window`, `root`,
/// `subwindow`, `time`, `x`, `y`, `x_root`, `y_root`, `state`, `button`), **UNVERIFIED**.
/// C: Xlib.h: `typedef union _XEvent { int type; XAnyEvent xany; XButtonEvent xbutton; … }
/// XEvent;` - the port carries the union with the two members this file touches (`type` is only
/// ever written by other units, `xbutton` is what C 248 reaches) plus a padding member that keeps
/// the union the 192 bytes the frozen target's Xlib has; the members C does not name are
/// **UNVERIFIED** and owed (`## Owed`).
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent
{
  pub r#type: c_int,
  pub xbutton: XButtonPressedEvent,
  pub _padding: [u8; 192],
}
#[cfg (feature = "use_lucid")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct XButtonPressedEvent
{
  pub r#type: c_int,
  pub serial: c_ulong,
  pub send_event: Boolean,
  pub display: *mut c_void,
  pub window: c_ulong,
  pub root: c_ulong,
  pub subwindow: c_ulong,
  pub time: c_ulong,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub button: c_uint,
  pub same_screen: Boolean,
}

#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  /* C: Intrinsic.h: `String XtName (Widget);`
     C: Intrinsic.h: `Widget XtParent (Widget);`
     C: Intrinsic.h: `WidgetClass XtClass (Widget);`
     C: Intrinsic.h: `Display *XtDisplay (Widget);`
     C: Intrinsic.h: `Window XtWindow (Widget);`
     C: Intrinsic.h: `Widget XtCreateWidget (const char *, WidgetClass, Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `Widget XtCreateManagedWidget (const char *, WidgetClass, Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `Widget XtCreatePopupShell (const char *, WidgetClass, Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `void XtAddCallback (Widget, const char *, XtCallbackProc, XtPointer);`
     C: Intrinsic.h: `void XtSetValues (Widget, ArgList, Cardinal);`
     C: Intrinsic.h: `void XtDestroyWidget (Widget);`
     C: Intrinsic.h: `void XtCallActionProc (Widget, const char *, XEvent *, String *, Cardinal);`
     C: Intrinsic.h: `Boolean XtIsShell (Widget);` (a macro in C: `XtIsSubclass (w, shellWidgetClass)`)
     C: Xlib.h: `int XQueryPointer (Display *, Window, Window *, Window *, int *, int *, int *, int *, unsigned int *);`  */
  pub fn XtName (widget: Widget) -> String;
  pub fn XtParent (widget: Widget) -> Widget;
  pub fn XtClass (widget: Widget) -> WidgetClass;
  pub fn XtDisplay (widget: Widget) -> *mut c_void;
  pub fn XtWindow (widget: Widget) -> c_ulong;
  pub fn XtCreateWidget (name: *const c_char, class: WidgetClass, parent: Widget,
                         args: *mut Arg, num_args: c_uint) -> Widget;
  pub fn XtCreateManagedWidget (name: *const c_char, class: WidgetClass, parent: Widget,
                                args: *mut Arg, num_args: c_uint) -> Widget;
  pub fn XtCreatePopupShell (name: *const c_char, class: WidgetClass, parent: Widget,
                             args: *mut Arg, num_args: c_uint) -> Widget;
  pub fn XtAddCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                        client_data: XtPointer);
  pub fn XtSetValues (widget: Widget, args: *mut Arg, num_args: c_uint);
  pub fn XtDestroyWidget (widget: Widget);
  pub fn XtCallActionProc (widget: Widget, action: *const c_char, event: *mut c_void,
                           params: *mut String, num_params: c_uint);
  pub fn XtIsShell (widget: Widget) -> Boolean;
  pub fn XQueryPointer (display: *mut c_void, window: c_ulong, root: *mut c_ulong,
                        child: *mut c_ulong, root_x: *mut c_int, root_y: *mut c_int,
                        win_x: *mut c_int, win_y: *mut c_int, mask: *mut c_uint) -> Boolean;
}
/// C: Intrinsic.h: `typedef void (*XtCallbackProc) (Widget, XtPointer, XtPointer);`
#[cfg (feature = "use_lucid")]
pub type XtCallbackProc = Option<unsafe extern "C" fn (Widget, XtPointer, XtPointer)>;
/// C: Shell.h: `extern WidgetClass shellWidgetClass;` - the class `XtIsShell` compares against.
#[cfg (feature = "use_lucid")]
unsafe extern "C" {
  pub static mut shellWidgetClass: WidgetClass;
}

/* ------------------------------------------------------------------------------------------
   The file's definitions, in C's order.
   ------------------------------------------------------------------------------------------ */

/* C 34-58: `#if 0` ... `#endif /* 0 */` - `x_print_complete_resource_name` is dead text and is
   not transcribed (the `src/xfns.rs` 958 precedent); the ledger records it.  */

/* C 61-70: `highlight_hook` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn highlight_hook (w: Widget, client_data: XtPointer, call_data: XtPointer)
{
  let instance: *mut widget_instance = client_data as *mut widget_instance;

  if (*((*instance).info)).highlight_cb.is_some ()
      && (*w).core.being_destroyed == 0
    { ((*((*instance).info)).highlight_cb.unwrap ()) (w, (*((*instance).info)).id, call_data); }
}

/* C 72-77: `enter_hook` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn enter_hook (w: Widget, client_data: XtPointer, call_data: XtPointer)
{
  highlight_hook (w, client_data, call_data);
}

/* C 79-87: `leave_hook` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn leave_hook (w: Widget, client_data: XtPointer, call_data: XtPointer)
{
  highlight_hook (w, client_data, core::ptr::null_mut ());
}

/* C 90-105: `pre_hook` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn pre_hook (w: Widget, client_data: XtPointer, call_data: XtPointer)
{
  let instance: *mut widget_instance = client_data as *mut widget_instance;
  let val: *mut widget_value;

  if (*w).core.being_destroyed != 0
    { return; }

  val = lw_get_widget_value_for_widget (instance, w);
  if (*((*instance).info)).pre_activate_cb.is_some ()
    {
      ((*((*instance).info)).pre_activate_cb.unwrap ())
        (w, (*((*instance).info)).id,
         if !val.is_null () { (*val).call_data } else { core::ptr::null_mut () });
    }
}

/* C 107-128: `pick_hook` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn pick_hook (w: Widget, client_data: XtPointer, call_data: XtPointer)
{
  let instance: *mut widget_instance = client_data as *mut widget_instance;
  let contents_val: *mut widget_value = call_data as *mut widget_value;
  let widget_val: *mut widget_value;
  let widget_arg: XtPointer;

  if (*w).core.being_destroyed != 0
    { return; }

  if (*((*instance).info)).selection_cb.is_some () && !contents_val.is_null ()
      && (*contents_val).enabled
      && (*contents_val).contents.is_null ()
    {
      ((*((*instance).info)).selection_cb.unwrap ())
        (w, (*((*instance).info)).id, (*contents_val).call_data);
    }

  widget_val = lw_get_widget_value_for_widget (instance, w);
  widget_arg = if !widget_val.is_null () { (*widget_val).call_data } else { core::ptr::null_mut () };
  if (*((*instance).info)).post_activate_cb.is_some ()
    { ((*((*instance).info)).post_activate_cb.unwrap ()) (w, (*((*instance).info)).id, widget_arg); }

}

/* C 130-157: `xlw_create_menubar` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn xlw_create_menubar (instance: *mut widget_instance) -> Widget
{
  let mut widget: Widget;
  let mut al: [Arg; 5] = [core::mem::zeroed (); 5];
  let mut ac: c_int = 0;

  XtSetArg! (al[ac as usize], XtNmenu.as_ptr (), (*((*instance).info)).val); ac += 1;
  /* C 140-144: `#ifdef emacs` - the build's own `-Demacs`, always defined for this file.  */
  XtSetArg! (al[ac as usize], XtNshowGrip.as_ptr (), core::ptr::null_mut ()); ac += 1;
  XtSetArg! (al[ac as usize], XtNresizeToPreferred.as_ptr (), 1 as XtPointer); ac += 1;
  XtSetArg! (al[ac as usize], XtNallowResize.as_ptr (), 1 as XtPointer); ac += 1;

  /* This used to use XtVaCreateWidget, but an old Xt version has a bug in XtVaCreateWidget that
     frees instance->info->name.  */
  widget = XtCreateWidget ((*((*instance).info)).name, xlwMenuWidgetClass,
                           (*instance).parent, al.as_mut_ptr (), ac as c_uint);

  XtAddCallback (widget, XtNopen.as_ptr (), Some (pre_hook), instance as XtPointer);
  XtAddCallback (widget, XtNselect.as_ptr (), Some (pick_hook), instance as XtPointer);
  XtAddCallback (widget, XtNleaveCallback.as_ptr (), Some (leave_hook), instance as XtPointer);
  XtAddCallback (widget, XtNenterCallback.as_ptr (), Some (enter_hook), instance as XtPointer);
  return widget;
}

/* C 159-184: `xlw_create_popup_menu` (static).  */
#[cfg (feature = "use_lucid")]
unsafe extern "C" fn xlw_create_popup_menu (instance: *mut widget_instance) -> Widget
{
  let popup_shell: Widget
    = XtCreatePopupShell ((*((*instance).info)).name, overrideShellWidgetClass,
                          (*instance).parent, core::ptr::null_mut (), 0);

  let mut widget: Widget;
  let mut al: [Arg; 2] = [core::mem::zeroed (); 2];
  let mut ac: c_int = 0;

  XtSetArg! (al[ac as usize], XtNmenu.as_ptr (), (*((*instance).info)).val); ac += 1;
  XtSetArg! (al[ac as usize], XtNhorizontal.as_ptr (), False as XtPointer); ac += 1;

  /* This used to use XtVaManagedCreateWidget, but an old Xt version has a bug in
     XtVaManagedCreateWidget that frees instance->info->name.  */
  widget = XtCreateManagedWidget (c"popup".as_ptr (), xlwMenuWidgetClass,
                                  popup_shell, al.as_mut_ptr (), ac as c_uint);

  XtAddCallback (widget, XtNselect.as_ptr (), Some (pick_hook), instance as XtPointer);
  XtAddCallback (widget, XtNleaveCallback.as_ptr (), Some (leave_hook), instance as XtPointer);
  XtAddCallback (widget, XtNenterCallback.as_ptr (), Some (enter_hook), instance as XtPointer);

  return popup_shell;
}

/* C 186-192: `xlw_creation_table`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub static mut xlw_creation_table: [widget_creation_entry; 3] =
  [widget_creation_entry { r#type: c"menubar".as_ptr () as *mut c_char,
                           function: Some (xlw_create_menubar) },
   widget_creation_entry { r#type: c"popup".as_ptr () as *mut c_char,
                           function: Some (xlw_create_popup_menu) },
   widget_creation_entry { r#type: core::ptr::null_mut (), function: None }];

/* C 194-205: `lw_lucid_widget_p`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_lucid_widget_p (widget: Widget) -> Boolean
{
  let the_class: WidgetClass = XtClass (widget);

  if the_class == xlwMenuWidgetClass
    { return True; }
  if the_class == overrideShellWidgetClass
    {
      return (XtClass (*((*(widget as CompositeWidget)).composite.children).offset (0))
              == xlwMenuWidgetClass) as Boolean;
    }
  return False;
}

/* C 207-218: `xlw_update_one_widget`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlw_update_one_widget (instance: *mut widget_instance, widget: Widget,
                                               val: *mut widget_value, _deep_p: Boolean)
{
  let mut al: [Arg; 1] = [core::mem::zeroed (); 1];

  /* This used to use XtVaSetValues, but some old Xt versions that have a bug in XtVaCreateWidget
     might have it here too.  */
  XtSetArg! (al[0], XtNmenu.as_ptr (), (*((*instance).info)).val);

  XtSetValues (widget, al.as_mut_ptr (), 1);
}

/* C 220-226: `xlw_update_one_value`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlw_update_one_value (_instance: *mut widget_instance, _widget: Widget,
                                              _val: *mut widget_value)
{
  return;
}

/* C 228-231: `xlw_pop_instance`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlw_pop_instance (_instance: *mut widget_instance, _up: Boolean)
{
}

/* C 233-263: `xlw_popup_menu`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlw_popup_menu (widget: Widget, event: *mut c_void)
{
  let mw: XlwMenuWidget;

  if XtIsShell (widget) == 0
    { return; }

  mw = (*((*(widget as CompositeWidget)).composite.children).offset (0)) as XlwMenuWidget;

  if !event.is_null ()
    { XtCallActionProc (mw as Widget, c"start".as_ptr (), event, core::ptr::null_mut (), 0); }
  else
    {
      let mut dummy: XEvent = core::mem::zeroed ();
      let bd: *mut XButtonPressedEvent = &raw mut dummy.xbutton;

      (*bd).r#type = ButtonPress;
      (*bd).serial = 0;
      (*bd).send_event = 0;
      (*bd).display = XtDisplay (widget);
      (*bd).window = XtWindow (XtParent (widget));
      (*bd).time = CurrentTime;
      (*bd).button = 0;
      XQueryPointer ((*bd).display, (*bd).window, &raw mut (*bd).root,
                     &raw mut (*bd).subwindow, &raw mut (*bd).x_root,
                     &raw mut (*bd).y_root, &raw mut (*bd).x, &raw mut (*bd).y,
                     &raw mut (*bd).state);

      XtCallActionProc (mw as Widget, c"start".as_ptr (), &raw mut dummy as *mut c_void,
                        core::ptr::null_mut (), 0);
    }
}

/* C 265-271: `xlw_destroy_instance`.  */
#[cfg (feature = "use_lucid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xlw_destroy_instance (instance: *mut widget_instance)
{
  if !(*instance).widget.is_null ()
    { XtDestroyWidget ((*instance).widget); }
}
