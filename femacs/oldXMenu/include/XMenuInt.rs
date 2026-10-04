// femacs: mirror of oldXMenu/XMenuInt.h @ fd7afced403.
//
// The XMenu library's internal interface: the `min`/`max`/`abs` helpers, the private
// success/failure codes, the `Pixel` spelling and the entry points the eighteen `.c` files of
// the library call each other with.  The frozen configuration builds no window system -
// `USE_X_TOOLKIT`, `HAVE_X11` and `HAVE_X_WINDOWS` are all `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes it and the
// mirror is never compiled; the text gate `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Display`, `XID`, `XEvent` - come from <X11/Xlib.h>; the
// port has no mirror for that header, so they keep C's spelling and are left for the unit that
// eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

/*
 * XMenu:	MIT Project Athena, X Window system menu package
 *
 *	XMenuInternal.h - Internal menu system include file for the
 *			MIT Project Athena XMenu X window system
 *			menu package.
 *
 *	Author:		Tony Della Fera, DEC
 *			October, 1985
 */

/* Avoid warnings about redefining NULL by including <stdio.h> first;
   the other file which wants to define it (<stddef.h> on Ultrix
   systems) can deal if NULL is already defined, but <stdio.h> can't.  */
/* C 38's `#include <config.h>`, C 40's `#include <attribute.h>` and C 45's
   `#include <stdio.h>`: this header uses nothing from any of them (the C comment below says
   why `<stdio.h>` is there at all), so none of their mirrors is pulled in (PORTING.md P-26).
   C 46's `#include <X11/Xlib.h>` has no mirror.  */
include!("X10.rs");   // XMenuInt.h 47: `#include "X10.h"`
include!("XMenu.rs"); // XMenuInt.h 48: `#include "XMenu.h"`

// XMenuInt.h 35-36: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _XMenuInternal_h_: bool = true;

/// XMenuInt.h 50: `#define min(x, y)	((x) <= (y) ? (x) : (y))` - C evaluates both
/// arguments twice, so the port's macro does too (PORTING.md 2.1/P-16).
macro_rules! min {
  ($x:expr, $y:expr) => { if $x <= $y { $x } else { $y } };
}
/// XMenuInt.h 51: `#define max(x, y)	((x) >= (y) ? (x) : (y))`
macro_rules! max {
  ($x:expr, $y:expr) => { if $x >= $y { $x } else { $y } };
}
/// XMenuInt.h 52: `#define abs(a)		((a) < 0 ? -(a) : (a))`
macro_rules! abs {
  ($a:expr) => { if $a < 0 { -$a } else { $a } };
}

/// XMenuInt.h 54: `#define _X_FAILURE	-1`
pub const _X_FAILURE: c_int = -1;

/// XMenuInt.h 56: `#define _SUCCESS	1`
pub const _SUCCESS: c_int = 1;
/// XMenuInt.h 57: `#define _FAILURE	-1`
pub const _FAILURE: c_int = -1;

/*
 * XMenu internal event handler variable.
 */
unsafe extern "C" {
  /// XMenuInt.h 62: `extern int (*_XMEventHandler)(XEvent*);` - the variable holds a
  /// nullable C function pointer, so the port stores an `Option` (P-27).
  pub static mut _XMEventHandler: Option<unsafe extern "C" fn (event: *mut XEvent) -> c_int>;
}

/* XMenuInt.h 64-66: `#ifndef Pixel` / `#define Pixel unsigned long` is a type spelling; the
   port writes the alias and every use site keeps the name (the `MINI_GMP_LIMB_TYPE` row,
   docs/ports/excluded-macros.txt).  */
/// XMenuInt.h 65: `#define Pixel unsigned long`
pub type Pixel = c_ulong;

/*
 * Internal routine declarations.
 */
unsafe extern "C" {
  /// XMenuInt.h 71: `void _XMWinQueInit(void);`
  pub fn _XMWinQueInit ();
  /// XMenuInt.h 72: `int _XMWinQueAddPane(Display *display, XMenu *menu, XMPane *p_ptr);`
  pub fn _XMWinQueAddPane (display: *mut Display, menu: *mut XMenu,
                           p_ptr: *mut XMPane) -> c_int;
  /// XMenuInt.h 73: `int _XMWinQueAddSelection(Display *display, XMenu *menu, XMSelect *s_ptr);`
  pub fn _XMWinQueAddSelection (display: *mut Display, menu: *mut XMenu,
                                s_ptr: *mut XMSelect) -> c_int;
  /// XMenuInt.h 74: `int _XMWinQueFlush(Display *display, XMenu *menu, XMPane *pane,
  /// XMSelect *select);`
  pub fn _XMWinQueFlush (display: *mut Display, menu: *mut XMenu, pane: *mut XMPane,
                         select: *mut XMSelect) -> c_int;
  /// XMenuInt.h 75: `XMPane *_XMGetPanePtr(XMenu *menu, int p_num);`
  pub fn _XMGetPanePtr (menu: *mut XMenu, p_num: c_int) -> *mut XMPane;
  /// XMenuInt.h 76: `XMSelect *_XMGetSelectionPtr(XMPane *p_ptr, int s_num);`
  pub fn _XMGetSelectionPtr (p_ptr: *mut XMPane, s_num: c_int) -> *mut XMSelect;
  /// XMenuInt.h 77: `void _XMRecomputeGlobals(Display *display, XMenu *menu);`
  pub fn _XMRecomputeGlobals (display: *mut Display, menu: *mut XMenu);
  /// XMenuInt.h 78: `int _XMRecomputePane(Display *display, XMenu *menu, XMPane *p_ptr,
  /// int p_num);`
  pub fn _XMRecomputePane (display: *mut Display, menu: *mut XMenu, p_ptr: *mut XMPane,
                           p_num: c_int) -> c_int;
  /// XMenuInt.h 79: `int _XMRecomputeSelection(Display *display, XMenu *menu, XMSelect *s_ptr,
  /// int s_num);`
  pub fn _XMRecomputeSelection (display: *mut Display, menu: *mut XMenu, s_ptr: *mut XMSelect,
                                s_num: c_int) -> c_int;
  /// XMenuInt.h 80: `void _XMTransToOrigin(Display *display, XMenu *menu, XMPane *p_ptr,
  /// XMSelect *s_ptr, int x_pos, int y_pos, int *orig_x, int *orig_y);`
  pub fn _XMTransToOrigin (display: *mut Display, menu: *mut XMenu, p_ptr: *mut XMPane,
                           s_ptr: *mut XMSelect, x_pos: c_int, y_pos: c_int,
                           orig_x: *mut c_int, orig_y: *mut c_int);
  /// XMenuInt.h 81: `void _XMRefreshPane(Display *display, XMenu *menu, XMPane *pane);`
  pub fn _XMRefreshPane (display: *mut Display, menu: *mut XMenu, pane: *mut XMPane);
  /// XMenuInt.h 82: `void _XMRefreshSelection(Display *display, XMenu *menu, XMSelect *select);`
  pub fn _XMRefreshSelection (display: *mut Display, menu: *mut XMenu, select: *mut XMSelect);
  /// XMenuInt.h 83: `void emacs_insque (void *elem, void *prev);`
  pub fn emacs_insque (elem: *mut c_void, prev: *mut c_void);
  /// XMenuInt.h 84: `void emacs_remque (void *elem);`
  pub fn emacs_remque (elem: *mut c_void);
  /// XMenuInt.h 85: `void XDestroyAssocTable(XAssocTable *table);`
  pub fn XDestroyAssocTable (table: *mut XAssocTable);
  /// XMenuInt.h 86: `void XMakeAssoc(Display *dpy, XAssocTable *table, XID x_id, void *data);`
  pub fn XMakeAssoc (dpy: *mut Display, table: *mut XAssocTable, x_id: XID, data: *mut c_void);
  /// XMenuInt.h 87: `void XDeleteAssoc(Display *dpy, XAssocTable *table, XID x_id);`
  pub fn XDeleteAssoc (dpy: *mut Display, table: *mut XAssocTable, x_id: XID);
}

/* Don't add stuff after this #endif */
