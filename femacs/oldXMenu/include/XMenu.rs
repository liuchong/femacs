// femacs: mirror of oldXMenu/XMenu.h @ fd7afced403.
//
// The public interface of the MIT XMenu library: the error/selection constants, the window
// hierarchy records (`XMWindow`, `XMPane`, `XMSelect`), the menu record `XMenu`, its style and
// mode enums, the two callback typedefs and the entry points Emacs's X menu code calls.  The
// frozen configuration builds no window system - `USE_X_TOOLKIT`, `HAVE_X11` and
// `HAVE_X_WINDOWS` are all `undef` in docs/baseline/config/darwin-arm64.macros.tsv - so no
// translation unit includes it and the mirror is never compiled; the text gate
// `tools/extract-abi.py --check` is its whole acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Display`, `Window`, `Cursor`, `GC`, `XFontStruct`,
// `Pixmap`, `XEvent` - come from <X11/Xlib.h> and <X11/Xutil.h>; the port has no mirror for
// those headers, so they keep C's spelling and are left for the unit that eventually uses
// this header to supply (PORTING.md P-26/P-72 (3)).

/*
 * XMenu:	MIT Project Athena, X Window system menu package
 *
 *	XMenu.h -	Include file for the MIT Project Athena
 *			XMenu X window system menu package.
 *
 *	Author:		Tony Della Fera, DEC
 *			August, 1984
 */

/* C 37: `#include <stdlib.h>` - the port has a <stdlib.h> mirror (`lib/include/stdlib.rs`),
   but this header uses nothing from it, so it is not pulled in (P-26).  C 38's
   `#include <X11/Xutil.h>` has no mirror at all.  */
include!("X10.rs");   // XMenu.h 39: `#include "X10.h"`

// XMenu.h 34-35: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _XMenu_h_: bool = true;

/// XMenu.h 41: `#define FAILURE			-1`
pub const FAILURE: c_int = -1;
/// XMenu.h 42: `#define SUCCESS			1`
pub const SUCCESS: c_int = 1;
/// XMenu.h 43: `#define POST_ERROR		-1`
pub const POST_ERROR: c_int = -1;
/// XMenu.h 44: `#define NO_SELECTION		-1`
pub const NO_SELECTION: c_int = -1;

/// XMenu.h 46: `#define XM_FAILURE		-1`
pub const XM_FAILURE: c_int = -1;
/// XMenu.h 47: `#define XM_SUCCESS		1`
pub const XM_SUCCESS: c_int = 1;
/// XMenu.h 48: `#define XM_NO_SELECT		2`
pub const XM_NO_SELECT: c_int = 2;
/// XMenu.h 49: `#define XM_IA_SELECT		3`
pub const XM_IA_SELECT: c_int = 3;

/// XMenu.h 51: `#define XME_CODE_COUNT		17`
pub const XME_CODE_COUNT: c_int = 17;

/// XMenu.h 53: `XME_NO_ERROR		0`
pub const XME_NO_ERROR: c_int = 0;
/// XMenu.h 54: `XME_NOT_INIT		1`
pub const XME_NOT_INIT: c_int = 1;
/// XMenu.h 55: `XME_ARG_BOUNDS		2`
pub const XME_ARG_BOUNDS: c_int = 2;
/// XMenu.h 56: `XME_P_NOT_FOUND		3`
pub const XME_P_NOT_FOUND: c_int = 3;
/// XMenu.h 57: `XME_S_NOT_FOUND		4`
pub const XME_S_NOT_FOUND: c_int = 4;
/// XMenu.h 58: `XME_STYLE_PARAM		5`
pub const XME_STYLE_PARAM: c_int = 5;
/// XMenu.h 59: `XME_GRAB_MOUSE		6`
pub const XME_GRAB_MOUSE: c_int = 6;
/// XMenu.h 60: `XME_INTERP_LOC		7`
pub const XME_INTERP_LOC: c_int = 7;
/// XMenu.h 61: `XME_CALLOC		8`
pub const XME_CALLOC: c_int = 8;
/// XMenu.h 62: `XME_CREATE_ASSOC	9`
pub const XME_CREATE_ASSOC: c_int = 9;
/// XMenu.h 63: `XME_STORE_BITMAP	10`
pub const XME_STORE_BITMAP: c_int = 10;
/// XMenu.h 64: `XME_MAKE_TILES		11`
pub const XME_MAKE_TILES: c_int = 11;
/// XMenu.h 65: `XME_MAKE_PIXMAP	12`
pub const XME_MAKE_PIXMAP: c_int = 12;
/// XMenu.h 66: `XME_CREATE_CURSOR	13`
pub const XME_CREATE_CURSOR: c_int = 13;
/// XMenu.h 67: `XME_OPEN_FONT		14`
pub const XME_OPEN_FONT: c_int = 14;
/// XMenu.h 68: `XME_CREATE_WINDOW	15`
pub const XME_CREATE_WINDOW: c_int = 15;
/// XMenu.h 69: `XME_CREATE_TRANSP	16`
pub const XME_CREATE_TRANSP: c_int = 16;

unsafe extern "C" {
  /*
   * XMenu error code and error list definitions.
   */
  /// XMenu.h 74: `extern int _XMErrorCode;`
  pub static mut _XMErrorCode: c_int;
  /// XMenu.h 75: `extern char const *const _XMErrorList[];` (an incomplete array; the port
  /// declares it with length 0, the lisp.h `my_edata` shape).
  pub static _XMErrorList: [*const c_char; 0];
}

/*
 * Define the XMWindow datatypes.
 *
 * An XMWindow is either an XMPane or an XMSelect.
 *
 * XMWindow is wrapper used to identify the constant window
 * information that makes up XMPane and XMSelect objects.
 *
 * An XMPane is a menu pane made up of one or more XMSelect and a label.
 *
 * An XMSelect is a menu selection object with a label and a data pointer.
 */
/// XMenu.h 89: `typedef enum _xmwintype {PANE, SELECTION, PL_HEADER, SL_HEADER, SEPARATOR}
/// XMWType;` - one constant per member (PORTING.md 1.3).
pub type XMWType = c_int;
/// XMenu.h 89: `PANE` (the first member: 0)
pub const PANE: XMWType = 0;
/// XMenu.h 89: `SELECTION` (1)
pub const SELECTION: XMWType = 1;
/// XMenu.h 89: `PL_HEADER` (2)
pub const PL_HEADER: XMWType = 2;
/// XMenu.h 89: `SL_HEADER` (3)
pub const SL_HEADER: XMWType = 3;
/// XMenu.h 89: `SEPARATOR` (4)
pub const SEPARATOR: XMWType = 4;

/// XMenu.h 91-109: `typedef struct _xmwindow { ... } XMWindow;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _xmwindow
{
  pub next: *mut _xmwindow,	/* Next obj pointer (for emacs_insque). */
  pub prev: *mut _xmwindow,	/* Prev obj pointer (for emacs_insque). */
  pub r#type: XMWType,		/* Type of window. */
  pub window: Window,		/* X Window Id. */
  pub window_x: c_int,		/* Window upper left X coordinate. */
  pub window_y: c_int,		/* Window upper left y coordinate. */
  pub window_w: c_int,		/* Window width. */
  pub window_h: c_int,		/* Window height. */
  pub active: c_int,		/* Window active? */
  pub activated: c_int,		/* Window activated? */
  pub pad_l1: c_int,		/* ---- */
  pub pad_l2: *mut c_char,	/* ---- */
  pub pad_l3: c_int,		/* ---- */
  pub pad_l4: c_int,		/* ---- */
  pub pad_l5: c_int,		/* ---- */
  pub pad_l6: c_int,		/* ---- */
  pub pad_l7: c_int,		/* ---- */
  pub pad_l8: c_int,		/* ---- */
  pub pad_l9: *mut _xmwindow,	/* ---- */
  pub pad_l10: *mut c_char,	/* ---- */
  pub pad_l11: *mut _xmwindow,	/* ---- */
}
/// XMenu.h 109: `typedef struct _xmwindow { ... } XMWindow;`
pub type XMWindow = _xmwindow;

/// XMenu.h 111-134: `typedef struct _xmpane { ... } XMPane;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _xmpane
{
  pub next: *mut _xmpane,	/* Next obj pointer (for emacs_insque). */
  pub prev: *mut _xmpane,	/* Prev obj pointer (for emacs_insque). */
  pub r#type: XMWType,		/* Type of window. */
  pub window: Window,		/* X Window Id. */
  pub window_x: c_int,		/* Window upper left X coordinate. */
  pub window_y: c_int,		/* Window upper left y coordinate. */
  pub window_w: c_int,		/* Window width. */
  pub window_h: c_int,		/* Window height. */
  pub active: c_int,		/* Window active? */
  pub activated: c_int,		/* Window activated? */
  pub serial: c_int,		/* -- Pane serial number. */
  pub label: *const c_char,	/* -- Pane label. */
  pub label_width: c_int,	/* -- Pane label width in pixels. */
  pub label_length: c_int,	/* -- Pane label length in chars. */
  pub label_x: c_int,		/* -- Pane label X offset. */
  pub label_uy: c_int,		/* -- Pane label upper Y offset. */
  pub label_ly: c_int,		/* -- Pane label lower Y offset. */
  pub s_count: c_int,		/* -- Selections in this pane. */
  pub s_list: *mut _xmselect,	/* -- Selection  window list. */
  pub pad_l10: *mut c_char,	/* ---- */
  pub pad_l11: *mut _xmwindow,	/* ---- */
}
/// XMenu.h 134: `typedef struct _xmpane { ... } XMPane;`
pub type XMPane = _xmpane;

/// XMenu.h 136-161: `typedef struct _xmselect { ... } XMSelect;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _xmselect
{
  pub next: *mut _xmselect,	/* Next obj pointer (for emacs_insque). */
  pub prev: *mut _xmselect,	/* Prev obj pointer (for emacs_insque). */
  pub r#type: XMWType,		/* Type of window. */
  pub window: Window,		/* X Window Id. */
  pub parent: Window,		/* X Window id of parent window. */
  pub window_x: c_int,		/* Window upper left X coordinate. */
  pub window_y: c_int,		/* Window upper left y coordinate. */
  pub window_w: c_int,		/* Window width. */
  pub window_h: c_int,		/* Window height. */
  pub active: c_int,		/* Window active? */
  pub activated: c_int,		/* Window activated? */
  pub serial: c_int,		/* -- Selection serial number. */
  pub label: *mut c_char,	/* -- Selection label. */
  pub label_width: c_int,	/* -- Selection label width in pixels. */
  pub label_length: c_int,	/* -- Selection label length in chars. */
  pub label_x: c_int,		/* -- Selection label X offset. */
  pub label_y: c_int,		/* -- Selection label Y offset. */
  pub pad_l7: c_int,		/* ---- */
  pub pad_l8: c_int,		/* ---- */
  pub pad_l9: *mut _xmwindow,	/* ---- */
  pub data: *mut c_char,	/* -- Selection data pointer. */
  pub parent_p: *mut _xmpane,	/* -- Selection parent pane structure. */
  pub help_string: *const c_char,	/* -- Help string or null.  */
}
/// XMenu.h 161: `typedef struct _xmselect { ... } XMSelect;`
pub type XMSelect = _xmselect;

/*
 * Define the XMStyle datatype.
 *
 * Menu presentation style information.
 *
 */
/// XMenu.h 172: `typedef enum _xmstyle { LEFT, RIGHT, CENTER } XMStyle;`
pub type XMStyle = c_int;
/// XMenu.h 173: `LEFT` (the first member: 0) - /* Left oriented object. */
pub const LEFT: XMStyle = 0;
/// XMenu.h 174: `RIGHT` (1) - /* Right oriented object. */
pub const RIGHT: XMStyle = 1;
/// XMenu.h 175: `CENTER` (2) - /* Center oriented object. */
pub const CENTER: XMStyle = 2;

/*
 * Define the XMMode datatype.
 *
 * Menu presentation mode information.
 *
 */
/// XMenu.h 186: `typedef enum _xmmode { BOX, INVERT } XMMode;`
pub type XMMode = c_int;
/// XMenu.h 187: `BOX` (the first member: 0) - /* BOXed graphic rendition. */
pub const BOX: XMMode = 0;
/// XMenu.h 188: `INVERT` (1) - /* INVERTed graphic rendition. */
pub const INVERT: XMMode = 1;

/*
 * Define the XMenu datatype.
 *
 * All dimensions are in pixels unless otherwise noted.
 */
/// XMenu.h 197-254: `typedef struct _xmenu { ... } XMenu;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _xmenu
{
  /* -------------------- Menu data -------------------- */
  pub menu_style: XMStyle,	/* Menu display style. */
  pub menu_mode: XMMode,	/* Menu display mode. */
  pub freeze: c_int,		/* Freeze server mode? */
  pub aeq: c_int,		/* Asynchronous Event Queuing mode? */
  pub recompute: c_int,		/* Recompute menu dependencies? */
  pub parent: Window,		/* Menu's parent window. */
  pub width: c_int,		/* Overall menu width. */
  pub height: c_int,		/* Overall menu height. */
  pub x_pos: c_int,		/* Overall menu origin. */
  pub y_pos: c_int,		/* Overall menu origin. */
  pub mouse_cursor: Cursor,	/* Mouse cursor raster. */
  pub assoc_tab: *mut XAssocTable,	/* XMWindow association table. */
  pub p_list: *mut XMPane,	/* List of XMPanes. */
  /* -------------------- Pane window data -------------------- */
  pub p_style: XMStyle,		/* Pane display style. */
  pub p_events: c_int,		/* Pane window X events. */
  pub p_fnt_info: *mut XFontStruct,	/* Flag font info structure. */
  pub pane_GC: GC,		/* Pane graphics context. */
  pub p_fnt_pad: c_int,		/* Fixed flag font padding in pixels. */
  pub p_spread: f64,		/* Pane spread in flag height fractions. */
  pub p_bdr_width: c_int,	/* Pane border width. */
  pub flag_height: c_int,	/* Flag height. */
  pub p_width: c_int,		/* Menu pane width. */
  pub p_height: c_int,		/* Menu pane height. */
  pub p_x_off: c_int,		/* Pane window X offset. */
  pub p_y_off: c_int,		/* Pane window Y offset. */
  pub p_count: c_int,		/* Number of panes per menu. */
  /* -------------------- Selection window data -------------------- */
  pub s_style: XMStyle,		/* Selection display style. */
  pub s_events: c_int,		/* Selection window X events. */
  pub s_fnt_info: *mut XFontStruct,	/* Body font info structure. */
  pub s_fnt_pad: c_int,		/* Fixed body font padding in pixels. */
  pub s_spread: f64,		/* Select spread in line height fractions. */
  pub s_bdr_width: c_int,	/* Select border width. */
  pub s_width: c_int,		/* Selection window width. */
  pub s_height: c_int,		/* Selection window height. */
  pub s_x_off: c_int,		/* Selection window X offset. */
  pub s_y_off: c_int,		/* Selection window Y offset. */
  pub s_count: c_int,		/* Maximum number of selections per pane. */
  pub normal_select_GC: GC,	/* GC used for inactive selections. */
  pub inverse_select_GC: GC,	/* GC used for active (current) selection. */
  pub inact_GC: GC,		/* GC used for inactive selections and */
				/* panes headers. */
  /* -------------------- Color data -------------------- */
  pub p_bdr_color: c_ulong,	/* Color of pane border pixmap. */
  pub s_bdr_color: c_ulong,	/* Color of selection border pixmap. */
  pub p_frg_color: c_ulong,	/* Color of pane foreground pixmap. */
  pub s_frg_color: c_ulong,	/* Color of selection pixmap. */
  pub bkgnd_color: c_ulong,	/* Color of menu background pixmap. */
  /* -------------------- Pixmap data -------------------- */
  pub p_bdr_pixmap: Pixmap,	/* Pane border pixmap. */
  pub s_bdr_pixmap: Pixmap,	/* Selection border pixmap. */
  pub p_frg_pixmap: Pixmap,	/* Pane foreground pixmap. */
  pub s_frg_pixmap: Pixmap,	/* Selection foreground pixmap. */
  pub bkgnd_pixmap: Pixmap,	/* Menu background pixmap. */
  pub inact_pixmap: Pixmap,	/* Menu inactive pixmap. */
}
/// XMenu.h 254: `typedef struct _xmenu { ... } XMenu;`
pub type XMenu = _xmenu;

/// XMenu.h 256: `typedef void (*Wait_func)(void*);` - nullable in C, so it is an `Option`
/// of an `extern "C"` fn (P-27).
pub type Wait_func = Option<unsafe extern "C" fn (data: *mut c_void)>;

/* Function for translating GenericEvents.  It is should call
   XPutBackEvent on an equivalent artificial core event on any
   function it wants to translate.  */
/// XMenu.h 261: `typedef void (*Translate_func)(XEvent *);`
pub type Translate_func = Option<unsafe extern "C" fn (event: *mut XEvent)>;

/*
 * XMenu library routine declarations.
 */
unsafe extern "C" {
  /// XMenu.h 266: `XMenu *XMenuCreate(Display *display, Window parent, char const *def_env);`
  pub fn XMenuCreate (display: *mut Display, parent: Window,
                      def_env: *const c_char) -> *mut XMenu;
  /// XMenu.h 267: `int XMenuAddPane(Display *display, XMenu *menu, char const *label, int active);`
  pub fn XMenuAddPane (display: *mut Display, menu: *mut XMenu, label: *const c_char,
                       active: c_int) -> c_int;
  /// XMenu.h 268: `int XMenuAddSelection(Display *display, XMenu *menu, int p_num, char *data,
  /// char *label, int active, char const *help);`
  pub fn XMenuAddSelection (display: *mut Display, menu: *mut XMenu, p_num: c_int,
                            data: *mut c_char, label: *mut c_char, active: c_int,
                            help: *const c_char) -> c_int;
  /// XMenu.h 269: `int XMenuInsertPane(XMenu *menu, int p_num, char *label, int active);`
  pub fn XMenuInsertPane (menu: *mut XMenu, p_num: c_int, label: *mut c_char,
                          active: c_int) -> c_int;
  /// XMenu.h 270: `int XMenuInsertSelection(XMenu *menu, int p_num, int s_num, char *data,
  /// char *label, int active);`
  pub fn XMenuInsertSelection (menu: *mut XMenu, p_num: c_int, s_num: c_int,
                               data: *mut c_char, label: *mut c_char, active: c_int) -> c_int;
  /// XMenu.h 271: `int XMenuFindPane(XMenu *menu, char *label);`
  pub fn XMenuFindPane (menu: *mut XMenu, label: *mut c_char) -> c_int;
  /// XMenu.h 272: `int XMenuFindSelection(XMenu *menu, int p_num, char *label);`
  pub fn XMenuFindSelection (menu: *mut XMenu, p_num: c_int, label: *mut c_char) -> c_int;
  /// XMenu.h 273: `int XMenuChangePane(XMenu *menu, int p_num, char *label);`
  pub fn XMenuChangePane (menu: *mut XMenu, p_num: c_int, label: *mut c_char) -> c_int;
  /// XMenu.h 274: `int XMenuChangeSelection(Display *display, XMenu *menu, int p_num, int s_num,
  /// char *data, int data_sw, char *label, int label_sw);`
  pub fn XMenuChangeSelection (display: *mut Display, menu: *mut XMenu, p_num: c_int,
                               s_num: c_int, data: *mut c_char, data_sw: c_int,
                               label: *mut c_char, label_sw: c_int) -> c_int;
  /// XMenu.h 275: `int XMenuSetPane(XMenu *menu, int p_num, int active);`
  pub fn XMenuSetPane (menu: *mut XMenu, p_num: c_int, active: c_int) -> c_int;
  /// XMenu.h 276: `int XMenuSetSelection(XMenu *menu, int p_num, int s_num, int active);`
  pub fn XMenuSetSelection (menu: *mut XMenu, p_num: c_int, s_num: c_int,
                            active: c_int) -> c_int;
  /// XMenu.h 277: `int XMenuRecompute(Display *display, XMenu *menu);`
  pub fn XMenuRecompute (display: *mut Display, menu: *mut XMenu) -> c_int;
  /// XMenu.h 278: `void XMenuEventHandler(int (*handler) (XEvent *));`
  pub fn XMenuEventHandler (handler: Option<unsafe extern "C" fn (event: *mut XEvent) -> c_int>);
  /// XMenu.h 279: `int XMenuLocate(Display *display, XMenu *menu, int p_num, int s_num, int x_pos,
  /// int y_pos, int *ul_x, int *ul_y, int *width, int *height);`
  pub fn XMenuLocate (display: *mut Display, menu: *mut XMenu, p_num: c_int, s_num: c_int,
                      x_pos: c_int, y_pos: c_int, ul_x: *mut c_int, ul_y: *mut c_int,
                      width: *mut c_int, height: *mut c_int) -> c_int;
  /// XMenu.h 280: `void XMenuSetFreeze(XMenu *menu, int freeze);`
  pub fn XMenuSetFreeze (menu: *mut XMenu, freeze: c_int);
  /// XMenu.h 281: `void XMenuActivateSetWaitFunction(Wait_func func, void *data);`
  pub fn XMenuActivateSetWaitFunction (func: Wait_func, data: *mut c_void);
  /// XMenu.h 282: `void XMenuActivateSetTranslateFunction(Translate_func func);`
  pub fn XMenuActivateSetTranslateFunction (func: Translate_func);
  /// XMenu.h 283: `int XMenuActivate(Display *display, XMenu *menu, int *p_num, int *s_num,
  /// int x_pos, int y_pos, unsigned int event_mask, char **data,
  /// void (*help_callback) (char const *, int, int));`
  pub fn XMenuActivate (display: *mut Display, menu: *mut XMenu, p_num: *mut c_int,
                        s_num: *mut c_int, x_pos: c_int, y_pos: c_int, event_mask: c_uint,
                        data: *mut *mut c_char,
                        help_callback: Option<unsafe extern "C" fn (arg0: *const c_char,
                                                                     arg1: c_int,
                                                                     arg2: c_int)>) -> c_int;
  /// XMenu.h 284: `char *XMenuPost(Display *display, XMenu *menu, int *p_num, int *s_num,
  /// int x_pos, int y_pos, int event_mask);`
  pub fn XMenuPost (display: *mut Display, menu: *mut XMenu, p_num: *mut c_int,
                    s_num: *mut c_int, x_pos: c_int, y_pos: c_int,
                    event_mask: c_int) -> *mut c_char;
  /// XMenu.h 285: `int XMenuDeletePane(Display *display, XMenu *menu, int p_num);`
  pub fn XMenuDeletePane (display: *mut Display, menu: *mut XMenu, p_num: c_int) -> c_int;
  /// XMenu.h 286: `int XMenuDeleteSelection(Display *display, XMenu *menu, int p_num, int s_num);`
  pub fn XMenuDeleteSelection (display: *mut Display, menu: *mut XMenu, p_num: c_int,
                               s_num: c_int) -> c_int;
  /// XMenu.h 287: `void XMenuDestroy(Display *display, XMenu *menu);`
  pub fn XMenuDestroy (display: *mut Display, menu: *mut XMenu);
  /// XMenu.h 288: `char const *XMenuError(void);`
  pub fn XMenuError () -> *const c_char;
  /// XMenu.h 289: `void XMenuSetAEQ(XMenu *menu, int aeq);`
  pub fn XMenuSetAEQ (menu: *mut XMenu, aeq: c_int);
}
/* Don't add after this point. */
