// femacs: port of lwlib/lwlib.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct member, file-scope object, local), the same control flow.  The non-`static`
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the `static` helper whose address Xt
// takes as a callback (`mark_widget_destroyed`) is `unsafe extern "C" fn` (PORTING.md 1.4).
//
// **The file gate is `feature = "use_x_toolkit"`** (P-86 clause 8, the build's condition rather
// than one the C text spells): `configure.ac` 7226-7229 sets `WIDGET_OBJ` and does
// `AC_DEFINE ([USE_X_TOOLKIT], [1], [Define to 1 if using an X toolkit.])` under
// `if test "${USE_X_TOOLKIT}" != "none"`, and 7296 puts `LIBXMENU='$(lwlibdir)/liblw.a'` in that
// same block; `lwlib/Makefile.in` 58 is the library's object list,
// `OBJS = lwlib.o $(TOOLKIT_OBJS) lwlib-utils.o`.  The gate is the toolkit switch itself rather
// than one toolkit's name (the `lwlib-Xlw.rs`/`lwlib-Xm.rs` choice) because `lwlib.o` is in `OBJS`
// for *both* toolkits, and because `include/lwlib-widget.rs` gates `widget_value`'s `change`,
// `this_one_change`, `toolkit_data` and `free_toolkit_data` - which the readers below use - on
// `use_x_toolkit`.  `USE_X_TOOLKIT`, `USE_LUCID`, `USE_MOTIF` and `HAVE_X11` are `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of this
// file and the port's frozen object is the empty one.
//
// **This file is `text-only`** (P-86 clause 3(2)): this machine carries no Xt, Xaw, Motif or X11
// header to measure this file's surface against, so every width, offset and resource name below is
// C's spelling with the value **UNVERIFIED**.
//
// The sub-conditions the C text spells keep their own gates (P-86 clause 1 and 6): C 33's
// `#if defined (USE_LUCID)` is `feature = "use_lucid"`, C 36's `#if defined (USE_MOTIF)` is
// `feature = "use_motif"`, C 39-41's derived `#define USE_XAW` is `feature = "use_xaw"`, C 44's
// `#ifdef HAVE_XAW3D` is `feature = "have_xaw3d"`, and C 52-54's `#error` is the `compile_error!`
// below.  C 884-886's `#ifdef USE_MOTIF` and C 903-909's are `feature = "use_motif"` too.
//
// C 60-94's static forward declarations are **deleted**: Rust items are order-independent
// (PORTING.md 0.1-12 / checklist 10), and `## C line ranges` names the region.  C 316-329's
// `#if 0` arm of `EXPLAIN` and C 443-447's `#if 0` inside `merge_widget_value` are dead in every
// configuration and are not transcribed (the `src/character.rs` 91-97 precedent).
//
// The `<X11/*>` surface stands in this unit (P-88 clause 1: no mirror in this tree carries
// `<X11/Intrinsic.h>`, `<X11/Xlib.h>`, `<X11/StringDefs.h>` or `<X11/{Xaw,Xaw3d}/Paned.h>`, and
// `include/lwlib.rs` names `Widget`, `Boolean`, `XEvent` and `Window` and leaves them to the unit
// that includes it).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, unused_mut, unused_assignments,
         unused_parens, static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("../src/include/config.rs");            // C 21: `#include <config.h>`
/* C 23's `#include <setjmp.h>` has no edge of its own: nothing in this file names a type that
   header declares, and the mirror that carries them is `lisp.rs`, which C 24 includes next.  */
#[cfg (feature = "use_x_toolkit")]
include!("../src/include/lisp.rs");              // C 24: `#include <lisp.h>`
                                                 // (`xmalloc`/`xzalloc`/`xstrdup`/`xfree`/`emacs_abort`/`Qnil`/`EQ`)
#[cfg (feature = "use_x_toolkit")]
include!("../lib/include/c-strcase.rs");         // C 25: `#include <c-strcase.h>` (`c_strcasecmp`)
/* C 27's `#include <sys/types.h>` has no edge of its own either: nothing here names a declaration
   that header carries.  */
#[cfg (feature = "use_x_toolkit")]
include!("../lib/include/stdio.rs");             // C 28: `#include <stdio.h>` (`printf`)
#[cfg (feature = "use_x_toolkit")]
include!("include/lwlib-int.rs");                // C 29: `#include "lwlib-int.h"`
                                                 // (`widget_info`/`widget_instance`, its own chain: `lwlib.rs`)
#[cfg (feature = "use_x_toolkit")]
include!("include/lwlib-utils.rs");              // C 30: `#include "lwlib-utils.h"` (`XtWidgetBeingDestroyedP`)
/* C 31's `<X11/StringDefs.h>` is the surface declared below: no mirror in this tree carries it.  */
/* C 32-42's toolkit block, arm by arm.  C 39-41's `#else /* not USE_MOTIF */` arm *defines*
   `USE_XAW` when `USE_LUCID` is on, and the port writes that derived name as the feature the
   arms below are gated on (the `use_cairo_or_have_xft` reading of a condition C derives).  */
#[cfg (feature = "use_xaw")]
macro_rules! USE_XAW { () => {} }
#[cfg (feature = "use_lucid")]
include!("include/lwlib-Xlw.rs");                // C 33-35: `#if defined (USE_LUCID)`
#[cfg (feature = "use_motif")]
include!("include/lwlib-Xm.rs");                 // C 36-38: `#if defined (USE_MOTIF)`
/* C 43-49's `#if defined (USE_XAW)` / `#ifdef HAVE_XAW3D` pair names the same six Athena headers
   under two directories (`lwlib-Xaw.h` is the mirror, and `<X11/{Xaw,Xaw3d}/Paned.h>` is the one
   name this file needs from either, in the surface below).  */
#[cfg (feature = "use_xaw")]
include!("include/lwlib-Xaw.rs");                // C 49: `#include "lwlib-Xaw.h"`

/* ------------------------------------------------------------------------------------------
   `<X11/StringDefs.h>`, `<X11/Intrinsic.h>`, `<X11/Xlib.h>` and `<X11/{Xaw,Xaw3d}/Paned.h>`
   (C 31, C 43-49).  **C's spellings, not measurements** - no X11 header is on this machine, so
   every width and layout below is `UNVERIFIED` (P-86 clause 3(2)).
   ------------------------------------------------------------------------------------------ */
/// C: Intrinsic.h: `typedef char *XtPointer;`
#[cfg (feature = "use_x_toolkit")]
pub type XtPointer = *mut c_void;
/// C: Intrinsic.h: `typedef char *String;`
#[cfg (feature = "use_x_toolkit")]
pub type String = *mut c_char;
/// C: Intrinsic.h: `typedef int Boolean;`
#[cfg (feature = "use_x_toolkit")]
pub type Boolean = c_int;
#[cfg (feature = "use_x_toolkit")]
pub const False: Boolean = 0;                         // C: Intrinsic.h: `#define False 0`
#[cfg (feature = "use_x_toolkit")]
pub const True: Boolean = 1;                          // C: Intrinsic.h: `#define True 1`
/// C: `<stddef.h>`: `#define NULL ((void *) 0)` - the null pointer C 62, 248, 280, 493 and 893
/// carry (the `lwlib-Xm.rs` 101-104 reading of the same name).
#[cfg (feature = "use_x_toolkit")]
pub const NULL: *mut c_void = core::ptr::null_mut ();
/// C: X.h 66-88: `typedef unsigned long XID;` and `typedef XID Window;`
#[cfg (feature = "use_x_toolkit")]
pub type XID = c_ulong;
#[cfg (feature = "use_x_toolkit")]
pub type Window = XID;
/// C: X.h: `typedef unsigned long Pixel;` (C 1129-1130's two caches).
#[cfg (feature = "use_x_toolkit")]
pub type Pixel = c_ulong;
/// C: Xlib.h: `typedef struct _XDisplay Display;` - only ever a pointer here.
#[cfg (feature = "use_x_toolkit")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct Display { pub _private: [u8; 0] }
/// C: Intrinsic.h: `typedef struct _WidgetRec *Widget;` - this file never dereferences one, so the
/// record is opaque here (the `include/gtk.rs` `placeholder, not GDK's value` practice, 3.481).
#[cfg (feature = "use_x_toolkit")]
#[derive (Copy, Clone)]
#[repr (C)]
pub struct WidgetRec { pub _private: [u8; 0] }
#[cfg (feature = "use_x_toolkit")]
pub type Widget = *mut WidgetRec;
/// C: Intrinsic.h: `typedef void (*XtCallbackProc) (Widget, XtPointer, XtPointer);`
#[cfg (feature = "use_x_toolkit")]
pub type XtCallbackProc = Option<unsafe extern "C" fn (Widget, XtPointer, XtPointer)>;
/// C: Xlib.h: `typedef union _XEvent { int type; … } XEvent;` - C 969 only passes one on, and the
/// union keeps this target's **measured** `XEvent` size (192 bytes, align 8) as the `[c_ulong; 24]`
/// element (the `lwlib-Xm.rs` 206-214 union shape); the members C does not name are `UNVERIFIED`.
#[cfg (feature = "use_x_toolkit")]
#[derive (Copy, Clone)]
#[repr (C)]
pub union XEvent
{
  pub r#type: c_int,
  pub _padding: [c_ulong; 24],
}
/// C: `<X11/StringDefs.h>`: `#define XtNdestroyCallback "destroyCallback"` (C 217, 795),
/// `#define XtNforeground "foreground"` and `#define XtNbackground "background"` (C 1136-1141).
#[cfg (feature = "use_x_toolkit")]
pub const XtNdestroyCallback: &core::ffi::CStr = c"destroyCallback";
#[cfg (feature = "use_x_toolkit")]
pub const XtNforeground: &core::ffi::CStr = c"foreground";
#[cfg (feature = "use_x_toolkit")]
pub const XtNbackground: &core::ffi::CStr = c"background";

#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  /* C: Intrinsic.h: `String XtName (Widget);` */
  pub fn XtName (widget: Widget) -> String;
  /* C: Intrinsic.h: `Widget XtNameToWidget (Widget, const char *);` */
  pub fn XtNameToWidget (reference: Widget, names: *const c_char) -> Widget;
  /* C: Intrinsic.h: `void XtAddCallback (Widget, const char *, XtCallbackProc, XtPointer);` */
  pub fn XtAddCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                        client_data: XtPointer);
  /* C: Intrinsic.h: `void XtRemoveCallback (Widget, const char *, XtCallbackProc, XtPointer);` */
  pub fn XtRemoveCallback (widget: Widget, name: *const c_char, callback: XtCallbackProc,
                           client_data: XtPointer);
  /* C: Intrinsic.h: `Boolean XtIsManaged (Widget);` (a macro in C: `XtIsSubclass (w, …)`) */
  pub fn XtIsManaged (widget: Widget) -> Boolean;
  /* C: Intrinsic.h: `Display *XtDisplay (Widget);` */
  pub fn XtDisplay (widget: Widget) -> *mut Display;
  /* C: Intrinsic.h: `Window XtWindow (Widget);` */
  pub fn XtWindow (widget: Widget) -> Window;
  /* C: Intrinsic.h: `Widget XtParent (Widget);` */
  pub fn XtParent (widget: Widget) -> Widget;
  /* C: Intrinsic.h: `void XtRealizeWidget (Widget);` */
  pub fn XtRealizeWidget (widget: Widget);
  /* C: Intrinsic.h: `void XtSetKeyboardFocus (Widget, Widget);` */
  pub fn XtSetKeyboardFocus (widget: Widget, descendant: Widget);
  /* C: Intrinsic.h: `void XtManageChild (Widget);` and `void XtUnmanageChild (Widget);`
     (C 1175/1177's Motif arm).  */
  pub fn XtManageChild (widget: Widget);
  pub fn XtUnmanageChild (widget: Widget);
  /* C: Intrinsic.h: `void XtVaGetValues (Widget, …);` and `void XtVaSetValues (Widget, …);` */
  pub fn XtVaGetValues (widget: Widget, ...);
  pub fn XtVaSetValues (widget: Widget, ...);
  /* C: Intrinsic.h: `void XtFree (char *);` */
  pub fn XtFree (ptr: *mut c_char);
  /* C: Intrinsic.h: `Widget XtWindowToWidget (Display *, Window);` (C 1192-1193) */
  pub fn XtWindowToWidget (display: *mut Display, window: Window) -> Widget;
  /* C: Xlib.h: `int XMapRaised (Display *, Window);` (C 914) */
  pub fn XMapRaised (display: *mut Display, window: Window) -> c_int;
  /* C: `<X11/Xaw/Paned.h>` and `<X11/Xaw3d/Paned.h>`: `void XawPanedSetRefigureMode (Widget,
     Boolean);` (C 46/47, C 1171) - the two arms C 43-48 names declare the same name.  */
  pub fn XawPanedSetRefigureMode (w: Widget, mode: Boolean);
}

/* Two names the mirrors on this unit's include graph do not carry.  `dupstring` is `lisp.h`
   5555's and `include/lisp.rs` does not spell it (the `lwlib-Xm.rs` 2828 reading); `strnlen` is
   `<string.h>`'s and `lib/include/string.rs` carries the `str*` family without it (the
   `lib-src/pop.rs` 307 shape).  */
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" {
  /// C: lisp.h 5555: `extern void dupstring (char **, char const *);`
  pub fn dupstring (ptr: *mut *mut c_char, string: *const c_char);
  /// C: `<string.h>`: `size_t strnlen (const char *s, size_t maxlen);`
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
}

/* C 52-54: `#if !defined (USE_LUCID) && !defined (USE_MOTIF)` … `#error At least one of USE_LUCID
   or USE_MOTIF must be defined.` - a configuration C refuses, refused the same way (`compile_error!`
   is the C `#error` counterpart, the `include/frame.rs` 320 precedent).  */
#[cfg (all (feature = "use_x_toolkit", not (feature = "use_lucid"), not (feature = "use_motif")))]
compile_error! ("At least one of USE_LUCID or USE_MOTIF must be defined.");

/* C 56-58: `#ifndef max` … `#define max(x, y) ((x) > (y) ? (x) : (y))` … `#endif` - C's fold over
   `change`/`change_type` below, written as a macro of C's name so the eight sites keep C's token
   (the `src/alloc.rs` 129 shape; the tokens `?` and `:` are the note in
   docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "use_x_toolkit")]
macro_rules! max {
  ($x:expr, $y:expr) => { if ($x) > ($y) { ($x) } else { ($y) } };
}

/* List of all widgets managed by the library. */
#[cfg (feature = "use_x_toolkit")]
static mut all_widget_info: *mut widget_info = NULL as *mut widget_info;

/* C 64-94: C's static forward declarations, deleted (PORTING.md 0.1-12 / checklist 10); the
   ledger's `## C line ranges` names the region.  */

/* C 96-125: `free_widget_value_tree` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn free_widget_value_tree (wv: *mut widget_value)
{
  if wv.is_null ()
    { return; }

  xfree ((*wv).name as *mut c_void);
  xfree ((*wv).value as *mut c_void);
  xfree ((*wv).key as *mut c_void);

  /* C's chained `wv->name = wv->value = wv->key = (char *) 0xDEADBEEF;`, one assignment per line
     (the same three stores in the same order).  */
  (*wv).name = 0xDEADBEEF as *mut c_char;
  (*wv).value = 0xDEADBEEF as *mut c_char;
  (*wv).key = 0xDEADBEEF as *mut c_char;

  if !(*wv).toolkit_data.is_null () && (*wv).free_toolkit_data
    {
      XtFree ((*wv).toolkit_data as *mut c_char);
      (*wv).toolkit_data = 0xDEADBEEF as *mut c_void;
    }

  if !(*wv).contents.is_null () && (*wv).contents != 1 as *mut widget_value
    {
      free_widget_value_tree ((*wv).contents);
      (*wv).contents = 0xDEADBEEF as *mut widget_value;
    }
  if !(*wv).next.is_null ()
    {
      free_widget_value_tree ((*wv).next);
      (*wv).next = 0xDEADBEEF as *mut widget_value;
    }
  xfree (wv as *mut c_void);
}

/* C 127-155: `copy_widget_value_tree` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn copy_widget_value_tree (val: *mut widget_value, change: change_type) -> *mut widget_value
{
  let copy: *mut widget_value;

  if val.is_null ()
    { return NULL as *mut widget_value; }
  if val == 1 as *mut widget_value
    { return val; }

  copy = xmalloc (core::mem::size_of::<widget_value> ()) as *mut widget_value;
  /* C's chained `copy->lname = copy->lkey = Qnil;`.  */
  (*copy).lname = Qnil ();
  (*copy).lkey = Qnil ();
  (*copy).name = xstrdup ((*val).name);
  (*copy).value = if !(*val).value.is_null () { xstrdup ((*val).value) } else { NULL as *mut c_char };
  (*copy).key = if !(*val).key.is_null () { xstrdup ((*val).key) } else { NULL as *mut c_char };
  (*copy).help = (*val).help;
  (*copy).enabled = (*val).enabled;
  (*copy).button_type = (*val).button_type;
  (*copy).selected = (*val).selected;
  (*copy).edited = false;
  (*copy).change = change;
  (*copy).this_one_change = change;
  (*copy).contents = copy_widget_value_tree ((*val).contents, change);
  (*copy).call_data = (*val).call_data;
  (*copy).next = copy_widget_value_tree ((*val).next, change);
  (*copy).toolkit_data = NULL as *mut c_void;
  (*copy).free_toolkit_data = false;
  return copy;
}

/* C 157-183: `allocate_widget_info` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn allocate_widget_info (r#type: *const c_char, name: *const c_char, id: LWLIB_ID,
                                val: *mut widget_value, pre_activate_cb: lw_callback,
                                selection_cb: lw_callback, post_activate_cb: lw_callback,
                                highlight_cb: lw_callback) -> *mut widget_info
{
  let info: *mut widget_info = xmalloc (core::mem::size_of::<widget_info> ()) as *mut widget_info;
  (*info).r#type = xstrdup (r#type);
  (*info).name = xstrdup (name);
  (*info).id = id;
  (*info).val = copy_widget_value_tree (val, STRUCTURAL_CHANGE);
  (*info).busy = False;
  (*info).pre_activate_cb = pre_activate_cb;
  (*info).selection_cb = selection_cb;
  (*info).post_activate_cb = post_activate_cb;
  (*info).highlight_cb = highlight_cb;
  (*info).instances = NULL as *mut widget_instance;

  (*info).next = all_widget_info;
  all_widget_info = info;

  return info;
}

/* C 185-193: `free_widget_info` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn free_widget_info (info: *mut widget_info)
{
  xfree ((*info).r#type as *mut c_void);
  xfree ((*info).name as *mut c_void);
  free_widget_value_tree ((*info).val);
  /* C's `memset (ptr, 0xDEADBEEF, n)`: the literal does not fit C's `int` either (it is
     converted, the same low byte), and the port spells that conversion the way rustc asks
     for it instead of taking the literal's `i32` default.  */
  memset (info as *mut c_void, 0xDEADBEEF_u32 as c_int, core::mem::size_of::<widget_info> ());
  xfree (info as *mut c_void);
}

/* C 195-203: `mark_widget_destroyed` (static; Xt takes its address as the destroy callback).  */
#[cfg (feature = "use_x_toolkit")]
unsafe extern "C" fn mark_widget_destroyed (widget: Widget, closure: XtPointer,
                                            call_data: XtPointer)
{
  let instance: *mut widget_instance = closure as *mut widget_instance;

  /* be very conservative */
  if (*instance).widget == widget
    { (*instance).widget = NULL as Widget; }
}

/* C 205-220: `allocate_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn allocate_widget_instance (info: *mut widget_info, parent: Widget,
                                    pop_up_p: Boolean) -> *mut widget_instance
{
  let instance: *mut widget_instance =
    xzalloc (core::mem::size_of::<widget_instance> ()) as *mut widget_instance;
  (*instance).parent = parent;
  (*instance).pop_up_p = pop_up_p;
  (*instance).info = info;
  (*instance).next = (*info).instances;
  (*info).instances = instance;

  instantiate_widget_instance (instance);

  XtAddCallback ((*instance).widget, XtNdestroyCallback.as_ptr (),
		 Some (mark_widget_destroyed), instance as XtPointer);
  return instance;
}

/* C 222-227: `free_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn free_widget_instance (instance: *mut widget_instance)
{
  memset (instance as *mut c_void, 0xDEADBEEF_u32 as c_int, core::mem::size_of::<widget_instance> ());
  xfree (instance as *mut c_void);
}

/* C 229-249: `get_widget_info` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn get_widget_info (id: LWLIB_ID, remove_p: Boolean) -> *mut widget_info
{
  let mut info: *mut widget_info;
  let mut prev: *mut widget_info;
  prev = NULL as *mut widget_info;
  info = all_widget_info;
  while !info.is_null ()
    {
      if (*info).id == id
	{
	  if remove_p != 0
	    {
	      if !prev.is_null ()
		{ (*prev).next = (*info).next; }
	      else
		{ all_widget_info = (*info).next; }
	    }
	  return info;
	}
      prev = info;
      info = (*info).next;
    }
  return NULL as *mut widget_info;
}

/* Internal function used by the library dependent implementation to get the
   widget_value for a given widget in an instance */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_widget_info (id: LWLIB_ID) -> *mut widget_info
{
  return get_widget_info (id, 0);
}

/* C 259-281: `get_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn get_widget_instance (widget: Widget, remove_p: Boolean) -> *mut widget_instance
{
  let mut info: *mut widget_info;
  let mut instance: *mut widget_instance;
  let mut prev: *mut widget_instance;
  info = all_widget_info;
  while !info.is_null ()
    {
      prev = NULL as *mut widget_instance;
      instance = (*info).instances;
      while !instance.is_null ()
	{
	  if (*instance).widget == widget
	    {
	      if remove_p != 0
		{
		  if !prev.is_null ()
		    { (*prev).next = (*instance).next; }
		  else
		    { (*info).instances = (*instance).next; }
		}
	      return instance;
	    }
	  prev = instance;
	  instance = (*instance).next;
	}
      info = (*info).next;
    }
  return 0 as *mut widget_instance;
}

/* Value is a pointer to the widget_instance corresponding to
   WIDGET, or null if WIDGET is not a lwlib widget.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_widget_instance (widget: Widget) -> *mut widget_instance
{
  return get_widget_instance (widget, False);
}

/* C 292-304: `find_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn find_instance (id: LWLIB_ID, parent: Widget, pop_up_p: Boolean) -> *mut widget_instance
{
  let info: *mut widget_info = get_widget_info (id, False);
  let mut instance: *mut widget_instance;

  if !info.is_null ()
    {
      instance = (*info).instances;
      while !instance.is_null ()
	{
	  if (*instance).parent == parent && (*instance).pop_up_p == pop_up_p
	    { return instance; }
	  instance = (*instance).next;
	}
    }

  return NULL as *mut widget_instance;
}

/* utility function for widget_value */
#[cfg (feature = "use_x_toolkit")]
unsafe fn safe_strcmp (s1: *const c_char, s2: *const c_char) -> Boolean
{
  if ((!s1.is_null ()) as c_int ^ (!s2.is_null ()) as c_int) != 0 { return True; }
  return if !s1.is_null () && !s2.is_null () { strcmp (s1, s2) }
	 else if !s1.is_null () { False }
	 else { (!s2.is_null ()) as Boolean };
}

/* C 316-332: `#if 0` … `#define EXPLAIN(name, oc, nc, desc, a1, a2) printf (…)` … `#else` …
   `#define EXPLAIN(name, oc, nc, desc, a1, a2) ((void) 0)` … `#endif`.  The `#if 0` arm is dead
   text and is not transcribed; the live arm is the port's macro of C's name (the tokens `void`,
   `0` are the note in docs/ports/macro-body-notes.tsv).  */
#[cfg (feature = "use_x_toolkit")]
macro_rules! EXPLAIN {
  ($name:expr, $oc:expr, $nc:expr, $desc:expr, $a1:expr, $a2:expr) => { () };
}

/* C 334-485: `merge_widget_value` (static).  C 443-447's `#if 0` block inside it is dead in every
   configuration and is not transcribed.  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn merge_widget_value (val1: *mut widget_value, val2: *mut widget_value, level: c_int,
                              change_p: *mut c_int) -> *mut widget_value
{
  let mut change: change_type;
  let mut this_one_change: change_type;
  let mut merged_next: *mut widget_value;
  let mut merged_contents: *mut widget_value;

  if val1.is_null ()
    {
      if !val2.is_null ()
	{
	  *change_p = 1;
	  return copy_widget_value_tree (val2, STRUCTURAL_CHANGE);
	}
      else
	{ return NULL as *mut widget_value; }
    }
  if val2.is_null ()
    {
      *change_p = 1;
      free_widget_value_tree (val1);
      return NULL as *mut widget_value;
    }

  change = NO_CHANGE;

  if safe_strcmp ((*val1).name, (*val2).name) != 0
    {
      EXPLAIN! ((*val1).name, change, STRUCTURAL_CHANGE, "name change",
		(*val1).name, (*val2).name);
      change = max! (change, STRUCTURAL_CHANGE);
      dupstring (&raw mut (*val1).name, (*val2).name);
    }
  if safe_strcmp ((*val1).value, (*val2).value) != 0
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "value change",
		(*val1).value, (*val2).value);
      change = max! (change, VISIBLE_CHANGE);
      dupstring (&raw mut (*val1).value, (*val2).value);
    }
  if safe_strcmp ((*val1).key, (*val2).key) != 0
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "key change",
		(*val1).key, (*val2).key);
      change = max! (change, VISIBLE_CHANGE);
      dupstring (&raw mut (*val1).key, (*val2).key);
    }
  if !EQ ((*val1).help, (*val2).help)
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "help change",
		(*val1).help, (*val2).help);
      change = max! (change, VISIBLE_CHANGE);
      (*val1).help = (*val2).help;
    }
  if (*val1).enabled != (*val2).enabled
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "enablement change",
		(*val1).enabled, (*val2).enabled);
      change = max! (change, VISIBLE_CHANGE);
      (*val1).enabled = (*val2).enabled;
    }
  if (*val1).button_type != (*val2).button_type
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "button type change",
		(*val1).button_type, (*val2).button_type);
      change = max! (change, VISIBLE_CHANGE);
      (*val1).button_type = (*val2).button_type;
    }
  if (*val1).selected != (*val2).selected
    {
      EXPLAIN! ((*val1).name, change, VISIBLE_CHANGE, "selection change",
		(*val1).selected, (*val2).selected);
      change = max! (change, VISIBLE_CHANGE);
      (*val1).selected = (*val2).selected;
    }
  if (*val1).call_data != (*val2).call_data
    {
      EXPLAIN! ((*val1).name, change, INVISIBLE_CHANGE, "call-data change",
		(*val1).call_data, (*val2).call_data);
      change = max! (change, INVISIBLE_CHANGE);
      (*val1).call_data = (*val2).call_data;
    }

  if level > 0
    {
      merged_contents = merge_widget_value ((*val1).contents, (*val2).contents, level - 1,
					    change_p);

      if !(*val1).contents.is_null () && merged_contents.is_null ()
	{
	  /* This used to say INVISIBLE_CHANGE,
	     but it is visible and vitally important when
	     the contents of the menu bar itself are entirely deleted.

	     But maybe it doesn't matter.  This fails to fix the bug.  */
	  EXPLAIN! ((*val1).name, change, STRUCTURAL_CHANGE, "(contents gone)", 0, 0);
	  change = max! (change, STRUCTURAL_CHANGE);
	}
      else if !merged_contents.is_null () && (*merged_contents).change != NO_CHANGE
	{
	  EXPLAIN! ((*val1).name, change, INVISIBLE_CHANGE, "(contents change)", 0, 0);
	  change = max! (change, INVISIBLE_CHANGE);
	  /* C 443-447: `#if 0` … `#ifdef USE_MOTIF` … `#endif` … `#endif` - "This was replaced by
	     the August 9 1996 change in lwlib-Xm.c."  Dead in every configuration.  */
	}

      (*val1).contents = merged_contents;
    }

  this_one_change = change;

  merged_next = merge_widget_value ((*val1).next, (*val2).next, level, change_p);

  if !(*val1).next.is_null () && merged_next.is_null ()
    {
      EXPLAIN! ((*val1).name, change, STRUCTURAL_CHANGE, "(following gone)", 0, 0);
      change = max! (change, STRUCTURAL_CHANGE);
    }
  else if !merged_next.is_null ()
    {
      if (*merged_next).change != 0
	{ EXPLAIN! ((*val1).name, change, (*merged_next).change, "(following change)", 0, 0); }
      change = max! (change, (*merged_next).change);
    }

  (*val1).next = merged_next;

  (*val1).this_one_change = this_one_change;
  (*val1).change = change;

  if change > NO_CHANGE && !(*val1).toolkit_data.is_null ()
    {
      *change_p = 1;
      if (*val1).free_toolkit_data
	{ XtFree ((*val1).toolkit_data as *mut c_char); }
      (*val1).toolkit_data = NULL as *mut c_void;
    }

  return val1;
}

/* modifying the widgets */
/* C 488-511: `name_to_widget` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn name_to_widget (instance: *mut widget_instance, name: *const c_char) -> Widget
{
  let mut widget: Widget = NULL as Widget;

  if (*instance).widget.is_null ()
    { return NULL as Widget; }

  if strcmp (XtName ((*instance).widget), name) == 0
    { widget = (*instance).widget; }
  else
    {
      let length: c_int = strlen (name) as c_int + 2;
      let real_name: *mut c_char = xmalloc (length as usize) as *mut c_char;
      *real_name = b'*' as c_char;
      strcpy (real_name.offset (1), name);

      widget = XtNameToWidget ((*instance).widget, real_name);

      xfree (real_name as *mut c_void);
    }
  return widget;
}

/* C 513-533: `set_one_value` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn set_one_value (instance: *mut widget_instance, val: *mut widget_value, deep_p: Boolean)
{
  let widget: Widget = name_to_widget (instance, (*val).name);

  if !widget.is_null ()
    {
      #[cfg (feature = "use_lucid")]
      if lw_lucid_widget_p ((*instance).widget) != 0
	{ xlw_update_one_widget (instance, widget, val, deep_p); }
      #[cfg (feature = "use_motif")]
      if lw_motif_widget_p ((*instance).widget) != 0
	{ xm_update_one_widget (instance, widget, val, deep_p); }
      #[cfg (feature = "use_xaw")]
      if lw_xaw_widget_p ((*instance).widget) != 0
	{ xaw_update_one_widget (instance, widget, val, deep_p); }
    }
}

/* C 535-547: `update_one_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn update_one_widget_instance (instance: *mut widget_instance, deep_p: Boolean)
{
  let mut val: *mut widget_value;

  if (*instance).widget.is_null ()
    /* the widget was destroyed */
    { return; }

  val = (*(*instance).info).val;
  while !val.is_null ()
    {
      if (*val).change != NO_CHANGE
	{ set_one_value (instance, val, deep_p); }
      val = (*val).next;
    }
}

/* C 549-560: `update_all_widget_values` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn update_all_widget_values (info: *mut widget_info, deep_p: Boolean)
{
  let mut instance: *mut widget_instance;
  let mut val: *mut widget_value;

  instance = (*info).instances;
  while !instance.is_null ()
    {
      update_one_widget_instance (instance, deep_p);
      instance = (*instance).next;
    }

  val = (*info).val;
  while !val.is_null ()
    {
      (*val).change = NO_CHANGE;
      val = (*val).next;
    }
}

/* C 562-612: `lw_modify_all_widgets`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_modify_all_widgets (id: LWLIB_ID, val: *mut widget_value,
						deep_p: Boolean) -> c_int
{
  let info: *mut widget_info = get_widget_info (id, False);
  let mut new_val: *mut widget_value;
  let mut next_new_val: *mut widget_value;
  let mut cur: *mut widget_value;
  let mut prev: *mut widget_value;
  let mut next: *mut widget_value;
  let mut found: c_int;
  let mut change_p: c_int = 0;

  if info.is_null ()
    { return 0; }

  new_val = val;
  while !new_val.is_null ()
    {
      next_new_val = (*new_val).next;
      (*new_val).next = NULL as *mut widget_value;
      found = False;
      prev = NULL as *mut widget_value;
      cur = (*info).val;
      while !cur.is_null ()
	{
	  if strcmp ((*cur).name, (*new_val).name) == 0
	    {
	      found = True;
	      next = (*cur).next;
	      (*cur).next = NULL as *mut widget_value;
	      cur = merge_widget_value (cur, new_val, if deep_p != 0 { 1000 } else { 1 },
					&raw mut change_p);
	      if !prev.is_null ()
		{ (*prev).next = if !cur.is_null () { cur } else { next }; }
	      else
		{ (*info).val = if !cur.is_null () { cur } else { next }; }
	      if !cur.is_null ()
		{ (*cur).next = next; }
	      break;
	    }
	  prev = cur;
	  cur = (*cur).next;
	}
      if found == 0
	{
	  /* Could not find it, add it */
	  if !prev.is_null ()
	    { (*prev).next = copy_widget_value_tree (new_val, STRUCTURAL_CHANGE); }
	  else
	    { (*info).val = copy_widget_value_tree (new_val, STRUCTURAL_CHANGE); }
	  change_p = 1;
	}
      new_val = next_new_val;
    }

  update_all_widget_values (info, deep_p);
  return change_p;
}

/* creating the widgets */

/* C 617-629: `initialize_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn initialize_widget_instance (instance: *mut widget_instance)
{
  let mut val: *mut widget_value;

  val = (*(*instance).info).val;
  while !val.is_null ()
    {
      (*val).change = STRUCTURAL_CHANGE;
      val = (*val).next;
    }

  update_one_widget_instance (instance, True);

  val = (*(*instance).info).val;
  while !val.is_null ()
    {
      (*val).change = NO_CHANGE;
      val = (*val).next;
    }
}

/* C 632-640: `find_in_table` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn find_in_table (r#type: *const c_char,
			 table: *const widget_creation_entry) -> widget_creation_function
{
  let mut cur: *const widget_creation_entry;
  cur = table;
  while !(*cur).r#type.is_null ()
    {
      if c_strcasecmp (r#type, (*cur).r#type) == 0
	{ return (*cur).function; }
      cur = cur.offset (1);
    }
  return None;
}

/* C 642-673: `dialog_spec_p` (static).  C's `switch`/`case` chain is the port's `match` over
   `*name as u8` (the `lwlib/lwlib-Xaw.rs` `xaw_create_dialog` shape and its
   docs/ports/macro-body-notes.tsv row: `case`/`break` have no counterpart, and Rust needs the
   `_ =>` arm C's `default:` spells).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn dialog_spec_p (name: *const c_char) -> Boolean
{
  /* return True if name matches [EILPQeilpq][1-9][Bb] or
     [EILPQeilpq][1-9][Bb][Rr][1-9] */
  if name.is_null ()
    { return False; }

  match *name as u8
    {
      b'E' | b'I' | b'L' | b'P' | b'Q' | b'e' | b'i' | b'l' | b'p' | b'q' =>
	{
	  if *name.offset (1) >= b'0' as c_char && *name.offset (1) <= b'9' as c_char
	    {
	      if *name.offset (2) != b'B' as c_char && *name.offset (2) != b'b' as c_char
		{ return False; }
	      if *name.offset (3) == 0
		{ return True; }
	      if (*name.offset (3) == b'T' as c_char || *name.offset (3) == b't' as c_char)
		  && *name.offset (4) == 0
		{ return True; }
	      if (*name.offset (3) == b'R' as c_char || *name.offset (3) == b'r' as c_char)
		  && *name.offset (4) >= b'0' as c_char && *name.offset (4) <= b'9' as c_char
		  && *name.offset (5) == 0
		{ return True; }
	      return False;
	    }
	  else
	    { return False; }
	}

      _ =>
	{ return False; }
    }
}

/* C 675-724: `instantiate_widget_instance` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn instantiate_widget_instance (instance: *mut widget_instance)
{
  let mut function: widget_creation_function = None;

  #[cfg (feature = "use_lucid")]
  if function.is_none ()
    {
      function = find_in_table ((*(*instance).info).r#type,
				&raw const xlw_creation_table as *const widget_creation_entry);
    }
  #[cfg (feature = "use_motif")]
  if function.is_none ()
    {
      function = find_in_table ((*(*instance).info).r#type,
				&raw const xm_creation_table as *const widget_creation_entry);
    }
  #[cfg (feature = "use_xaw")]
  if function.is_none ()
    {
      function = find_in_table ((*(*instance).info).r#type,
				&raw const xaw_creation_table as *const widget_creation_entry);
    }

  if function.is_none ()
    {
      if dialog_spec_p ((*(*instance).info).r#type) != 0
	{
	  /* C 697-699: `#if defined (USE_LUCID)` … `/* not yet */` … `#endif` - the Lucid arm is a
	     comment and nothing else.  */
	  #[cfg (feature = "use_motif")]
	  if function.is_none ()
	    { function = Some (xm_create_dialog); }
	  #[cfg (feature = "use_xaw")]
	  if function.is_none ()
	    { function = Some (xaw_create_dialog); }
	}
    }

  if function.is_none ()
    {
      printf (c"No creation function for widget type %s\n".as_ptr (),
	      (*(*instance).info).r#type);
      emacs_abort ();
    }

  (*instance).widget = (function.unwrap ()) (instance);

  if (*instance).widget.is_null ()
    { emacs_abort (); }

  /*   XtRealizeWidget (instance->widget);*/
}

/* C 726-739: `lw_register_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_register_widget (r#type: *const c_char, name: *const c_char,
					     id: LWLIB_ID, val: *mut widget_value,
					     pre_activate_cb: lw_callback,
					     selection_cb: lw_callback,
					     post_activate_cb: lw_callback,
					     highlight_cb: lw_callback)
{
  if get_widget_info (id, False).is_null ()
    { allocate_widget_info (r#type, name, id, val, pre_activate_cb, selection_cb,
			    post_activate_cb, highlight_cb); }
}

/* C 741-748: `lw_get_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_widget (id: LWLIB_ID, parent: Widget, pop_up_p: Boolean) -> Widget
{
  let instance: *mut widget_instance;

  instance = find_instance (id, parent, pop_up_p);
  return if instance.is_null () { NULL as Widget } else { (*instance).widget };
}

/* C 750-768: `lw_make_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_make_widget (id: LWLIB_ID, parent: Widget, pop_up_p: Boolean) -> Widget
{
  let mut instance: *mut widget_instance;
  let info: *mut widget_info;

  instance = find_instance (id, parent, pop_up_p);
  if instance.is_null ()
    {
      info = get_widget_info (id, False);
      if info.is_null ()
	{ return NULL as Widget; }
      instance = allocate_widget_instance (info, parent, pop_up_p);
      initialize_widget_instance (instance);
    }
  if (*instance).widget.is_null ()
    { emacs_abort (); }
  return (*instance).widget;
}

/* C 770-779: `lw_create_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_create_widget (r#type: *const c_char, name: *const c_char,
					   id: LWLIB_ID, val: *mut widget_value, parent: Widget,
					   pop_up_p: Boolean, pre_activate_cb: lw_callback,
					   selection_cb: lw_callback, post_activate_cb: lw_callback,
					   highlight_cb: lw_callback) -> Widget
{
  lw_register_widget (r#type, name, id, val, pre_activate_cb, selection_cb, post_activate_cb,
		      highlight_cb);
  return lw_make_widget (id, parent, pop_up_p);
}

/* destroying the widgets */
/* C 783-824: `destroy_one_instance` (static).  C 803-820's if-then-else chain gates each link with
   its own `#if`, and Rust cannot put an attribute on an `else`: the port spells each link as C's
   `else` chain does - the first link the preprocessor leaves in place, then the remaining links
   nested under `not (...)` of the arm just taken - which reproduces C's control flow exactly and
   keeps C 818-820's empty compound statement.  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn destroy_one_instance (instance: *mut widget_instance)
{
  /* Remove the destroy callback on the widget; that callback will try to
     dereference the instance object (to set its widget slot to 0, since the
     widget is dead.)  Since the instance is now dead, we don't have to worry
     about the fact that its widget is dead too.

     This happens in the Phase2Destroy of the widget, so this callback would
     not have been run until arbitrarily long after the instance was freed.
   */
  if !(*instance).widget.is_null ()
    {
      XtRemoveCallback ((*instance).widget, XtNdestroyCallback.as_ptr (),
			Some (mark_widget_destroyed), instance as XtPointer);
    }

  if !(*instance).widget.is_null ()
    {
      /* The else are pretty tricky here, including the empty statement
	 at the end because it would be very bad to destroy a widget
	 twice. */
      #[cfg (feature = "use_lucid")]
      if lw_lucid_widget_p ((*instance).widget) != 0
	{ xlw_destroy_instance (instance); }
      #[cfg (not (feature = "use_lucid"))]
      {
	#[cfg (feature = "use_motif")]
	if lw_motif_widget_p ((*instance).widget) != 0
	  { xm_destroy_instance (instance); }
	#[cfg (not (feature = "use_motif"))]
	{
	  #[cfg (feature = "use_xaw")]
	  if lw_xaw_widget_p ((*instance).widget) != 0
	    { xaw_destroy_instance (instance); }
	  #[cfg (not (feature = "use_xaw"))]
	  {
	    /* Empty compound statement to terminate if-then-else chain.  */
	  }
	}
      }
    }

  free_widget_instance (instance);
}

/* C 826-840: `lw_destroy_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_destroy_widget (w: Widget)
{
  let instance: *mut widget_instance = get_widget_instance (w, True);

  if !instance.is_null ()
    {
      let info: *mut widget_info = (*instance).info;
      /* instance has already been removed from the list; free it */
      destroy_one_instance (instance);
      /* if there are no instances left, free the info too */
      if (*info).instances.is_null ()
	{ lw_destroy_all_widgets ((*info).id); }
    }
}

/* C 842-859: `lw_destroy_all_widgets`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_destroy_all_widgets (id: LWLIB_ID)
{
  let info: *mut widget_info = get_widget_info (id, True);
  let mut instance: *mut widget_instance;
  let mut next: *mut widget_instance;

  if !info.is_null ()
    {
      instance = (*info).instances;
      while !instance.is_null ()
	{
	  next = (*instance).next;
	  destroy_one_instance (instance);
	  instance = next;
	}
      free_widget_info (info);
    }
}

/* C 861-866: `lw_destroy_everything`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_destroy_everything ()
{
  while !all_widget_info.is_null ()
    { lw_destroy_all_widgets ((*all_widget_info).id); }
}

/* C 868-882: `lw_destroy_all_pop_ups`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_destroy_all_pop_ups ()
{
  let mut info: *mut widget_info;
  let mut next: *mut widget_info;
  let mut instance: *mut widget_instance;

  info = all_widget_info;
  while !info.is_null ()
    {
      next = (*info).next;
      instance = (*info).instances;
      if !instance.is_null () && (*instance).pop_up_p != 0
	{ lw_destroy_all_widgets ((*info).id); }
      info = next;
    }
}

/* C 884-886: `#ifdef USE_MOTIF` / `extern Widget first_child (Widget);	/* garbage */` / `#endif`
   - `lwlib-Xm.c` defines it and `lwlib-Xm.h` does not declare it, so C declares it here.  */
#[cfg (feature = "use_motif")]
unsafe extern "C" {
  pub fn first_child (widget: Widget) -> Widget;
}

/* C 888-919: `lw_raise_all_pop_up_widgets`.  C 903-909's `#ifdef USE_MOTIF` sits in the *middle* of
   the test, which Rust cannot spell; the port holds the preprocessor's two spellings of the test in
   one local of its own name (the `xlwmenu.rs` `frobbed_ok` shape for a split condition).  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_raise_all_pop_up_widgets () -> Widget
{
  let mut info: *mut widget_info;
  let mut instance: *mut widget_instance;
  let mut result: Widget = NULL as Widget;

  info = all_widget_info;
  while !info.is_null ()
    {
      instance = (*info).instances;
      while !instance.is_null ()
	{
	  if (*instance).pop_up_p != 0
	    {
	      let widget: Widget = (*instance).widget;
	      if !widget.is_null ()
		{
		  /* C 902-910: `if (XtIsManaged (widget)` … `#ifdef USE_MOTIF` … `#endif` … `)`.  */
		  #[cfg (feature = "use_motif")]
		  let managed_p: Boolean =
		    (XtIsManaged (widget) != 0
		     /* What a complete load of crap!!!!
			When a dialogShell is on the screen, it is not managed!  */
		     || (lw_motif_widget_p ((*instance).widget) != 0
			 && XtIsManaged (first_child (widget)) != 0)) as Boolean;
		  #[cfg (not (feature = "use_motif"))]
		  let managed_p: Boolean = (XtIsManaged (widget) != 0) as Boolean;
		  if managed_p != 0
		    {
		      if result.is_null ()
			{ result = widget; }
		      XMapRaised (XtDisplay (widget), XtWindow (widget));
		    }
		}
	    }
	  instance = (*instance).next;
	}
      info = (*info).next;
    }
  return result;
}

/* C 921-954: `lw_pop_all_widgets` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn lw_pop_all_widgets (id: LWLIB_ID, up: Boolean)
{
  let info: *mut widget_info = get_widget_info (id, False);
  let mut instance: *mut widget_instance;

  if !info.is_null ()
    {
      instance = (*info).instances;
      while !instance.is_null ()
	{
	  if (*instance).pop_up_p != 0 && !(*instance).widget.is_null ()
	    {
	      #[cfg (feature = "use_lucid")]
	      if lw_lucid_widget_p ((*instance).widget) != 0
		{
		  XtRealizeWidget ((*instance).widget);
		  xlw_pop_instance (instance, up);
		}
	      #[cfg (feature = "use_motif")]
	      if lw_motif_widget_p ((*instance).widget) != 0
		{
		  XtRealizeWidget ((*instance).widget);
		  xm_pop_instance (instance, up);
		}
	      #[cfg (feature = "use_xaw")]
	      if lw_xaw_widget_p ((*instance).widget) != 0
		{
		  XtRealizeWidget (XtParent ((*instance).widget));
		  XtRealizeWidget ((*instance).widget);
		  xaw_pop_instance (instance, up);
		}
	    }
	  instance = (*instance).next;
	}
    }
}

/* C 956-960: `lw_pop_up_all_widgets`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_pop_up_all_widgets (id: LWLIB_ID)
{
  lw_pop_all_widgets (id, True);
}

/* C 962-966: `lw_pop_down_all_widgets`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_pop_down_all_widgets (id: LWLIB_ID)
{
  lw_pop_all_widgets (id, False);
}

/* C 968-983: `lw_popup_menu`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_popup_menu (widget: Widget, event: *mut XEvent)
{
  #[cfg (feature = "use_lucid")]
  if lw_lucid_widget_p (widget) != 0
    { xlw_popup_menu (widget, event); }
  #[cfg (feature = "use_motif")]
  if lw_motif_widget_p (widget) != 0
    { xm_popup_menu (widget, event); }
  #[cfg (feature = "use_xaw")]
  if lw_xaw_widget_p (widget) != 0
    { xaw_popup_menu (widget, event); }
}

/* get the values back */
/* C 986-1009: `get_one_value` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn get_one_value (instance: *mut widget_instance, val: *mut widget_value) -> Boolean
{
  let widget: Widget = name_to_widget (instance, (*val).name);

  if !widget.is_null ()
    {
      #[cfg (feature = "use_lucid")]
      if lw_lucid_widget_p ((*instance).widget) != 0
	{ xlw_update_one_value (instance, widget, val); }
      #[cfg (feature = "use_motif")]
      if lw_motif_widget_p ((*instance).widget) != 0
	{ xm_update_one_value (instance, widget, val); }
      #[cfg (feature = "use_xaw")]
      if lw_xaw_widget_p ((*instance).widget) != 0
	{ xaw_update_one_value (instance, widget, val); }
      return True;
    }
  else
    { return False; }
}

/* C 1011-1031: `lw_get_some_values`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_some_values (id: LWLIB_ID, val_out: *mut widget_value) -> Boolean
{
  let info: *mut widget_info = get_widget_info (id, False);
  let mut instance: *mut widget_instance;
  let mut val: *mut widget_value;
  let mut result: Boolean = False;

  if info.is_null ()
    { return False; }

  instance = (*info).instances;
  if instance.is_null ()
    { return False; }

  val = val_out;
  while !val.is_null ()
    {
      if get_one_value (instance, val) != 0
	{ result = True; }
      val = (*val).next;
    }

  return result;
}

/* C 1033-1044: `lw_get_all_values`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_all_values (id: LWLIB_ID) -> *mut widget_value
{
  let info: *mut widget_info = get_widget_info (id, False);
  if !info.is_null ()
    {
      let val: *mut widget_value = (*info).val;
      if lw_get_some_values (id, val) != 0
	{ return val; }
    }
  return NULL as *mut widget_value;
}

/* internal function used by the library dependent implementation to get the
   widget_value for a given widget in an instance */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_widget_value_for_widget (instance: *mut widget_instance,
							 w: Widget) -> *mut widget_value
{
  let name: *const c_char = XtName (w);
  let mut cur: *mut widget_value;
  cur = (*(*instance).info).val;
  while !cur.is_null ()
    {
      if strcmp ((*cur).name, name) == 0
	{ return cur; }
      cur = (*cur).next;
    }
  return NULL as *mut widget_value;
}

/* update other instances value when one thing changed */

/* To forbid recursive calls */
#[cfg (feature = "use_x_toolkit")]
static mut lwlib_updating: Boolean = 0;

/* This function can be used as an XtCallback for the widgets that get
   modified to update other instances of the widgets.  Closure should be the
   widget_instance. */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_internal_update_other_instances (widget: Widget, closure: XtPointer,
							     call_data: XtPointer)
{
  let instance: *mut widget_instance = closure as *mut widget_instance;
  let name: *const c_char = XtName (widget);
  let mut info: *mut widget_info;
  let mut cur: *mut widget_instance;
  let mut val: *mut widget_value;

  /* Avoid possibly infinite recursion.  */
  if lwlib_updating != 0
    { return; }

  /* protect against the widget being destroyed */
  if XtWidgetBeingDestroyedP (widget) != 0
    { return; }

  /* Return immediately if there are no other instances */
  info = (*instance).info;
  if (*(*info).instances).next.is_null ()
    { return; }

  lwlib_updating = True;

  val = (*info).val;
  while !val.is_null () && strcmp ((*val).name, name) != 0
    { val = (*val).next; }

  if !val.is_null () && get_one_value (instance, val) != 0
    {
      cur = (*info).instances;
      while !cur.is_null ()
	{
	  if cur != instance
	    { set_one_value (cur, val, True); }
	  cur = (*cur).next;
	}
    }

  lwlib_updating = False;
}

/* get the id */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_get_widget_id (w: Widget) -> LWLIB_ID
{
  let instance: *mut widget_instance = get_widget_instance (w, False);

  return if instance.is_null () { 0 } else { (*(*instance).info).id };
}

/* set the keyboard focus */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_set_keyboard_focus (parent: Widget, w: Widget)
{
  /* C 1118-1122: `#if defined (USE_MOTIF)` … `#else` … `#endif`, both arms.  */
  #[cfg (feature = "use_motif")]
  { xm_set_keyboard_focus (parent, w); }
  #[cfg (not (feature = "use_motif"))]
  { XtSetKeyboardFocus (parent, w); }
}

/* Show busy */
/* C 1126-1143: `show_one_widget_busy` (static).  */
#[cfg (feature = "use_x_toolkit")]
unsafe fn show_one_widget_busy (w: Widget, flag: Boolean)
{
  let mut foreground: Pixel = 0;
  let mut background: Pixel = 1;
  let mut widget_to_invert: Widget = XtNameToWidget (w, c"*sheet".as_ptr ());
  if widget_to_invert.is_null ()
    { widget_to_invert = w; }

  XtVaGetValues (widget_to_invert,
		 XtNforeground.as_ptr (), &raw mut foreground,
		 XtNbackground.as_ptr (), &raw mut background,
		 NULL);
  XtVaSetValues (widget_to_invert,
		 XtNforeground.as_ptr (), background,
		 XtNbackground.as_ptr (), foreground,
		 NULL);
}

/* C 1145-1163: `lw_show_busy`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_show_busy (w: Widget, busy: Boolean)
{
  let instance: *mut widget_instance = get_widget_instance (w, False);
  let mut info: *mut widget_info;
  let mut next: *mut widget_instance;

  if !instance.is_null ()
    {
      info = (*instance).info;
      if (*info).busy != busy
	{
	  next = (*info).instances;
	  while !next.is_null ()
	    {
	      if !(*next).widget.is_null ()
		{ show_one_widget_busy ((*next).widget, busy); }
	      next = (*next).next;
	    }
	  (*info).busy = busy;
	}
    }
}

/* This hack exists because Lucid/Athena need to execute the strange
   function below to support geometry management. */
/* C 1167-1179: `lw_refigure_widget`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_refigure_widget (w: Widget, doit: Boolean)
{
  #[cfg (feature = "use_xaw")]
  { XawPanedSetRefigureMode (w, doit); }
  /* C 1173-1178: `#if defined (USE_MOTIF)` around the whole `if`/`else`, so the attribute below
     is on one statement (Rust has no attribute for an `else`).  */
  #[cfg (feature = "use_motif")]
  if doit != 0
    { XtManageChild (w); }
  else
    { XtUnmanageChild (w); }
}

/* Toolkit independent way of determining if an event window is in the
   menubar. */
/* C 1183-1196: `lw_window_is_in_menubar`.  C's expression is one `return`, with each toolkit's
   `&&` half behind its own `#if` and a `;` inside the first arm's text; the port returns the arm
   the preprocessor leaves (the `## Deviations` entry says why C's text is only well formed when at
   most one of the two is defined, which is exactly what C 52-54's `#error` requires).  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_window_is_in_menubar (win: Window, menubar_widget: Widget) -> Boolean
{
  #[cfg (feature = "use_lucid")]
  return ((!menubar_widget.is_null ()) && XtWindow (menubar_widget) == win) as Boolean;
  #[cfg (feature = "use_motif")]
  return ((!menubar_widget.is_null ())
	  && ((XtWindow (menubar_widget) == win)
	      || (!XtWindowToWidget (XtDisplay (menubar_widget), win).is_null ()
		  && (XtParent (XtWindowToWidget (XtDisplay (menubar_widget), win))
		      == menubar_widget)))) as Boolean;
}

/* Motif hack to set the main window areas. */
/* C 1199-1205: `lw_set_main_areas`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_set_main_areas (parent: Widget, menubar: Widget, work_area: Widget)
{
  #[cfg (feature = "use_motif")]
  { xm_set_main_areas (parent, menubar, work_area); }
}

/* Manage resizing for Motif.  This disables resizing when the menubar
   is about to be modified. */
/* C 1209-1215: `lw_allow_resizing`.  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_allow_resizing (w: Widget, flag: Boolean)
{
  #[cfg (feature = "use_motif")]
  { xm_manage_resizing (w, flag); }
}

/* Value is non-zero if LABEL is a menu separator.  If it is, *TYPE is
   set to an appropriate enumerator of type enum menu_separator.
   MOTIF_P non-zero means map separator types not supported by Motif
   to similar ones that are supported.  */

/* C 1223-1330: `lw_separator_p`.  C's two function-local `struct separator_table` types and their
   two `static` tables are the two block-local ones below (Rust items may sit in a block).  */
#[cfg (feature = "use_x_toolkit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lw_separator_p (mut label: *const c_char,
					 r#type: *mut menu_separator, motif_p: c_int) -> c_int
{
  let mut separator_p: c_int = 0;

  if strncmp (label, c"--:".as_ptr (), 3) == 0
    {
      struct separator_table
      {
	name: *const c_char,
	r#type: menu_separator,
      }
      /* C's function-local `static` table; read-only here, so the port spells the one copy
         C's storage class asks for as a `const` of the same name (`*const c_char` is not
         `Sync`, which an immutable `static` requires).  */
      const separator_names: [separator_table; 15] =
      [
	separator_table { name: c"space".as_ptr (), r#type: SEPARATOR_NO_LINE },
	separator_table { name: c"noLine".as_ptr (), r#type: SEPARATOR_NO_LINE },
	separator_table { name: c"singleLine".as_ptr (), r#type: SEPARATOR_SINGLE_LINE },
	separator_table { name: c"doubleLine".as_ptr (), r#type: SEPARATOR_DOUBLE_LINE },
	separator_table { name: c"singleDashedLine".as_ptr (),
			  r#type: SEPARATOR_SINGLE_DASHED_LINE },
	separator_table { name: c"doubleDashedLine".as_ptr (),
			  r#type: SEPARATOR_DOUBLE_DASHED_LINE },
	separator_table { name: c"shadowEtchedIn".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_IN },
	separator_table { name: c"shadowEtchedOut".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_OUT },
	separator_table { name: c"shadowEtchedInDash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_IN_DASH },
	separator_table { name: c"shadowEtchedOutDash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_OUT_DASH },
	separator_table { name: c"shadowDoubleEtchedIn".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_IN },
	separator_table { name: c"shadowDoubleEtchedOut".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT },
	separator_table { name: c"shadowDoubleEtchedInDash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH },
	separator_table { name: c"shadowDoubleEtchedOutDash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH },
	separator_table { name: core::ptr::null (), r#type: 0 },
      ];

      let mut i: c_int;

      label = label.offset (3);
      i = 0;
      while !separator_names[i as usize].name.is_null ()
	{
	  if strcmp (label, separator_names[i as usize].name) == 0
	    {
	      separator_p = 1;
	      *r#type = separator_names[i as usize].r#type;

	      /* If separator type is not supported under Motif,
		 use a similar one.  */
	      if motif_p != 0 && *r#type >= SEPARATOR_SHADOW_DOUBLE_ETCHED_IN
		{ *r#type -= 4; }
	      break;
	    }
	  i += 1;
	}
    }
  else if strnlen (label, 4) == 4
	   && memcmp (label as *const c_void, c"--".as_ptr () as *const c_void, 2) == 0
	   && *label.offset (2) != b'-' as c_char
    {
      /* Alternative, more Emacs-style names.  */
      struct separator_table
      {
	name: *const c_char,
	r#type: menu_separator,
      }
      /* C's function-local `static` table; read-only here, so the port spells the one copy
         C's storage class asks for as a `const` of the same name (`*const c_char` is not
         `Sync`, which an immutable `static` requires).  */
      const separator_names: [separator_table; 15] =
      [
	separator_table { name: c"space".as_ptr (), r#type: SEPARATOR_NO_LINE },
	separator_table { name: c"no-line".as_ptr (), r#type: SEPARATOR_NO_LINE },
	separator_table { name: c"single-line".as_ptr (), r#type: SEPARATOR_SINGLE_LINE },
	separator_table { name: c"double-line".as_ptr (), r#type: SEPARATOR_DOUBLE_LINE },
	separator_table { name: c"single-dashed-line".as_ptr (),
			  r#type: SEPARATOR_SINGLE_DASHED_LINE },
	separator_table { name: c"double-dashed-line".as_ptr (),
			  r#type: SEPARATOR_DOUBLE_DASHED_LINE },
	separator_table { name: c"shadow-etched-in".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_IN },
	separator_table { name: c"shadow-etched-out".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_OUT },
	separator_table { name: c"shadow-etched-in-dash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_IN_DASH },
	separator_table { name: c"shadow-etched-out-dash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_ETCHED_OUT_DASH },
	separator_table { name: c"shadow-double-etched-in".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_IN },
	separator_table { name: c"shadow-double-etched-out".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT },
	separator_table { name: c"shadow-double-etched-in-dash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_IN_DASH },
	separator_table { name: c"shadow-double-etched-out-dash".as_ptr (),
			  r#type: SEPARATOR_SHADOW_DOUBLE_ETCHED_OUT_DASH },
	separator_table { name: core::ptr::null (), r#type: 0 },
      ];

      let mut i: c_int;

      label = label.offset (2);
      i = 0;
      while !separator_names[i as usize].name.is_null ()
	{
	  if strcmp (label, separator_names[i as usize].name) == 0
	    {
	      separator_p = 1;
	      *r#type = separator_names[i as usize].r#type;

	      /* If separator type is not supported under Motif,
		 use a similar one.  */
	      if motif_p != 0 && *r#type >= SEPARATOR_SHADOW_DOUBLE_ETCHED_IN
		{ *r#type -= 4; }
	      break;
	    }
	  i += 1;
	}
    }
  else
    {
      /* Old-style separator, maybe.  It's a separator if it contains
	 only dashes.  */
      if *label == b'-' as c_char
	{
	  while *label == b'-' as c_char
	    { label = label.offset (1); }
	  separator_p = (*label == 0) as c_int;

	  *r#type = SEPARATOR_SHADOW_ETCHED_IN;
	}
    }

  return separator_p;
}
