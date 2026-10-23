// femacs: port of src/w32menu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32menu.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 316, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac`
// 3022-3030's `HAVE_W32` test - the one whose 3030 `AC_DEFINE`s `HAVE_NTGUI`), and
// `docs/baseline/frozen-build-sources.txt` lists no `w32menu.o` for this target - so
// `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the ledger
// `docs/ports/w32menu.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  `HAVE_NTGUI` is the macro the
//     Windows GUI build defines; it is *not* `WINDOWSNT` (the Cygwin GUI build compiles this file
//     with `HAVE_NTGUI` and without `WINDOWSNT`, and C 43/143/191/1493/1519/1579/1838's
//     `NTGUI_UNICODE` is the arm that build takes), so the file's own conditionals are carried as
//     their own features *inside* the file gate, **both arms each**;
//   * `tools/check-cond-eval.py src/w32menu.c` reads **`15 conditions, 0 unknown, 1 disagreements
//     [NOT VERIFIED]`**: C 60's `#ifndef TRUE` is `True` to clang and `False` to the evaluator
//     (`TRUE` is in the frozen macro table), so this file is *not* on `cond-eval-verified.txt` and
//     the tool reads its conditions conservatively - both arms of every `#if` stay live, which is
//     also what P-86 ① requires;
//   * the six conditions are C 43 `#ifndef NTGUI_UNICODE` -> `not (feature = "ntgui_unicode")`,
//     C 49 `#ifndef makedev` -> `not (feature = "makedev")`, C 60 `#ifndef TRUE` ->
//     `not (feature = "true")`, C 83 `#ifndef MINGW_W64` -> `not (feature = "mingw_w64")`,
//     C 58's `#undef HAVE_DIALOGS` -> `not (feature = "have_dialogs")` (the file itself turns the
//     macro off, so C's `#ifdef HAVE_DIALOGS` arms are dead in *every* configuration and the
//     `#else` arms are the live ones), and C 1546/1767 `#ifdef MENU_DEBUG` ->
//     `feature = "menu_debug"`;
//   * **two of the file's arms cannot be compiled in C either.**  C 1055-1242's
//     `#ifdef HAVE_DIALOGS` body names `menu`, `dialog_id` and `widget_id_tick`, which the
//     reference declares nowhere this translation unit can reach (`src/xmenu.c` 1610 carries the
//     `widget_id_tick` definition behind a "FIXME: Move this to a .h file", and `dialog_id` /
//     `dialog_selection_callback` are that file's locals), and `f->output_data.w32->widget` (C 1199)
//     is not a member of `struct w32_output` (`src/w32term.h` 309-456 has `menubar_widget` only).
//     The arm is carried as C's own text with those names declared in this unit (P-88's
//     declaration-site rule) - and the ledger's `## Not claimed` says plainly that the arm is not
//     a configuration C itself can build;
//   * C 191-195's `#ifdef NTGUI_UNICODE` arm declares a **function-local** `pMultiByteToWideChar`
//     whose uses at C 207/209/237/240/259/263 sit *outside* that `#ifdef`: in the MinGW build the
//     name is `src/w32.c` 537's file-scope object (the one `src/systhread.c` 440 declares
//     `extern`).  Both are carried - the file-scope declaration for C's `#else` arm and the local
//     `static` for the `#ifdef` arm, which shadows it exactly as C's inner declaration does.
//
// **Acquisition note - this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is **no `<windows.h>`
// to compile or measure against** and **no arm flip on a Windows target and no layout
// measurement**: every record and every value the stand-in sections declare is `UNVERIFIED`
// (documented, not measured) and the layout assertions are owed.  What the round does carry is the
// frozen `build-tu` reading, the machine-checked coverage table, the `#if` verdicts, a per-item C
// citation, the `probe` reading of the `have_ntgui` + `windowsnt` arm, a free multi-configuration
// sweep of the same materialized text, and a mutation test over this unit's own lines.
//
// `docs/ports/w32menu.md` carries the C line ranges, the acceptance readings, every deviation (the
// in-place `<windows.h>`/`<commctrl.h>`/`<mbstring.h>` surfaces, the dropped forward declarations,
// the in-unit declarations no mirror carries, the `match`es that stand for C's `switch`es, the
// recorded `w32_output` bitfield reads, the wide-string literals) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-18 is the reference's opening block, dropped (PORTING.md 0.1-19); the port's first line is
   the revision line above.  */

include!("include/config.rs");                  // C 20: `#include <config.h>`

/* C 22-24: `<signal.h>`, `<stdio.h>` and `<setjmp.h>`.  The first two are mirrors; the third has
   no mirror in this tree (`src/image.rs` 78 records the same absence), and this file names nothing
   from it - the two edges C's text actually spends are carried, the third is recorded.  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/signal.rs");           // C 22: `#include <signal.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdio.rs");            // C 23: `#include <stdio.h>`
// C 24: `#include <setjmp.h>` - no mirror, and no name from it is used here.

/* C 26-33: the C headers.  `w32menu.o` is built only by a Windows GUI configuration, so the
   whole graph sits under the file gate (P-86 ⑧: with the includes outside it the frozen compile
   immediately eats `E0428`s).  C 32's `menu.h` reaches `../lwlib/lwlib-widget.h` at its own line 22,
   which is where `widget_value` and the `BUTTON_TYPE_*` members come from - the mirror
   `lwlib/include/lwlib-widget.rs` is landed, so this unit includes it at that position.  */
#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                    // C 26: `#include "lisp.h"`
#[cfg (feature = "have_ntgui")]
include!("include/keyboard.rs");                // C 27: `#include "keyboard.h"`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                   // C 28: `#include "frame.h"`
#[cfg (feature = "have_ntgui")]
include!("include/blockinput.rs");              // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_ntgui")]
include!("include/buffer.rs");                  // C 30: `#include "buffer.h"`
#[cfg (feature = "have_ntgui")]
include!("include/coding.rs");                  // C 31: `#include "coding.h"`	/* for ENCODE_SYSTEM */
#[cfg (feature = "have_ntgui")]
include!("include/menu.rs");                    // C 32: `#include "menu.h"`
#[cfg (feature = "have_ntgui")]
include!("../lwlib/include/lwlib-widget.rs");   // C 32 -> `menu.h` 22: `#include "../lwlib/lwlib-widget.h"`
#[cfg (feature = "have_ntgui")]
include!("include/pdumper.rs");                 // C 33: `#include "pdumper.h"`

/* C 35-37: "This may include sys/types.h, and that somehow loses if this is not done before the
   other system files."  C's comment is carried in C's own words where it stands:

     /* This may include sys/types.h, and that somehow loses
        if this is not done before the other system files.  */

   The include itself is `w32term.h`, whose mirror gates its whole body on `windowsnt` (the
   `include/w32term.rs` note records why); a Windows GUI build sets that feature too.  */
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                 // C 37: `#include "w32term.h"`

/* C 39-45: `#ifndef NTGUI_UNICODE / #include <mbstring.h> / #endif /* !NTGUI_UNICODE */`.
   `conf_post.h` 169-170 defines `NTGUI_UNICODE` only for `defined CYGWIN && defined HAVE_NTGUI`
   ("Cygwin runs only on UNICODE-supporting systems"), so C's live arm is the `#else` - the empty
   one - and the MinGW arm takes the include.  `<mbstring.h>` is a MinGW header with no file in the
   reference tree, so the three names C's `add_menu_item` takes from it stand in the section below
   (P-88 ①).  Both arms are carried.  */
// C 43-45: `#ifndef NTGUI_UNICODE` - the three `_mbs*` declarations are in the section below.

/* C 47-51: "Load sys/types.h if not already loaded.  In some systems loading it twice is
   suicidal."  `makedev` is in the frozen macro table (`docs/baseline/config/darwin-arm64.macros.tsv`
   7209: `(x,y) ((dev_t)(((x) << 24) | (y)))`), so C's arm is the dead one there; the `#ifndef`'s
   `#else` is empty.  Both arms are carried: the mirror is included when the SDK has not defined
   the guard.  */
#[cfg (all (feature = "have_ntgui", not (feature = "makedev")))]
include!("../lib/include/sys_types.in.rs");     // C 50: `#include <sys/types.h>`
// C 49-51's `#else` arm (a `makedev`-defining system) is empty.

include!("include/w32common.rs");               // C 53: `#include "w32common.h"`	/* for osinfo_cache */

/* ----------------------------------------------------------------------------------------------
   C 55: `#include "commctrl.h"`, and C 25/26's chain to `<windows.h>`.

   Neither header has a file in the reference tree (P-88 ①: the declaration site is C's own
   `#include` line - here C 55 for `commctrl.h`, and C 26's `lisp.h` -> `conf_post.h` ->
   `ms-w32.h` for `<windows.h>` - and no file is added under `src/include/`), so the names
   *this file* uses stand in the sections below, each with the SDK's own text above it.  The
   spellings repeat `src/w32term.rs`'s, `src/w32select.rs`'s and `src/w32font.rs`'s sections
   (`DWORD` is the 32-bit Windows unsigned whatever this target's `c_long` is, `HWND`/`HMENU`/
   `HINSTANCE`/`HMODULE`/`HANDLE` are `void *`, `BOOL` is C's `int`, `WINAPI`/`CALLBACK` are
   `extern "system"`), so that a Windows translation unit sees one set of names.

   **No measurement was possible**: this machine has no `<windows.h>` and no mingw toolchain (see
   the acquisition note), so every scalar is spelled from PORTING.md 1.1's fixed Windows widths and
   every record below is **UNVERIFIED** - its size, offsets and values are the documented ones and
   the layout assertions are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// `<windows.h>`'s scalar types (`windef.h`/`winnt.h`), which C's declarations and the mirrors
/// this unit includes both name.  `HRESULT` is `<winerror.h>`'s `long` and `S_OK` its zero.
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned int UINT;
/// C: typedef int BOOL; typedef unsigned long ULONG;
/// C: typedef UINT_PTR WPARAM; typedef LONG_PTR LPARAM; typedef LONG_PTR LRESULT;
/// C: typedef ULONG_PTR UINT_PTR; typedef long long LONG_PTR; typedef ULONG_PTR ULONG_PTR;
/// C: typedef long HRESULT; #define S_OK ((HRESULT) 0L)
#[cfg (feature = "have_ntgui")]
pub type BYTE = u8;
#[cfg (feature = "have_ntgui")]
pub type WORD = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type WCHAR = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type DWORD = u32;
#[cfg (feature = "have_ntgui")]
pub type LONG = i32;
#[cfg (feature = "have_ntgui")]
pub type ULONG = u32;
#[cfg (feature = "have_ntgui")]
pub type UINT = c_uint;
#[cfg (feature = "have_ntgui")]
pub type BOOL = c_int;
#[cfg (feature = "have_ntgui")]
pub type UINT_PTR = usize;
#[cfg (feature = "have_ntgui")]
pub type LONG_PTR = isize;
#[cfg (feature = "have_ntgui")]
pub type ULONG_PTR = usize;
#[cfg (feature = "have_ntgui")]
pub type WPARAM = UINT_PTR;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = LONG_PTR;
#[cfg (feature = "have_ntgui")]
pub type LRESULT = LONG_PTR;
#[cfg (feature = "have_ntgui")]
pub type HRESULT = c_long;
/// C 185's `return S_OK;` - `<winerror.h>`'s own value.
#[cfg (feature = "have_ntgui")]
pub const S_OK: HRESULT = 0;

/// `<windows.h>`'s pointer types.  Every handle is `void *` at the ABI (the `include/w32gui.rs`
/// reading); C's implicit conversions at the assignment sites need no cast in the port either.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *HMENU; typedef void *HINSTANCE;
/// C: typedef void *HMODULE; typedef void *HICON; typedef void *LPVOID;
/// C: typedef char *LPSTR; typedef const char *LPCSTR; typedef const WCHAR *LPCWSTR;
/// C: typedef WCHAR *LPWSTR; typedef const WCHAR *PCWSTR; typedef LPCWSTR PCWSTR;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HWND = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMENU = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HICON = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "have_ntgui")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "have_ntgui")]
pub type LPCWSTR = *const WCHAR;
#[cfg (feature = "have_ntgui")]
pub type LPWSTR = *mut WCHAR;
#[cfg (feature = "have_ntgui")]
pub type PCWSTR = *const WCHAR;

/// C's `WINAPI`/`CALLBACK` are `__stdcall`, which Rust spells `extern "system"` (the
/// `src/w32fns.rs` convention); no macro item is written for them.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define TRUE 1
/// C: #define FALSE 0
/// The two `FALSE`/`TRUE` names are `<windef.h>`'s; C 60-63's own `#ifndef TRUE` guard carries
/// them again under `not (feature = "true")`, the arm this one is the `#else` of.  Both arms hold
/// the same values, so a target that defines the name reads the same constants.
#[cfg (all (feature = "have_ntgui", feature = "true"))]
pub const FALSE: BOOL = 0;
#[cfg (all (feature = "have_ntgui", feature = "true"))]
pub const TRUE: BOOL = 1;

/// C 131's `TDM_ENABLE_BUTTON` is `(WM_USER+111)`, so `<winuser.h>`'s `WM_USER` must be named here
/// (nothing else in this crate declares it, and `include/w32term.rs` 907's `WM_EMACS_START` is
/// `WM_USER + 1`).  The rest are the window messages and menu flags C's text uses.
/// C: #define WM_USER 0x0400
/// C: #define WM_INITMENU 0x0116
/// C: #define MF_SEPARATOR 0x00000800L / #define MF_STRING 0x00000000L
/// C: #define MF_GRAYED 0x00000001L / #define MF_DISABLED 0x00000002L
/// C: #define MF_UNCHECKED 0x00000000L / #define MF_CHECKED 0x00000008L
/// C: #define MF_POPUP 0x00000010L / #define MF_OWNERDRAW 0x00000100L
/// C: #define MF_MOUSESELECT 0x00008000L / #define MF_HILITE 0x00000080L
/// C: #define MF_BYPOSITION 0x00000400L
#[cfg (feature = "have_ntgui")]
pub const WM_USER: c_int = 0x0400;
#[cfg (feature = "have_ntgui")]
pub const WM_INITMENU: UINT = 0x0116;
#[cfg (feature = "have_ntgui")]
pub const MF_SEPARATOR: UINT = 0x00000800;
#[cfg (feature = "have_ntgui")]
pub const MF_STRING: UINT = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const MF_GRAYED: UINT = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const MF_DISABLED: UINT = 0x00000002;
#[cfg (feature = "have_ntgui")]
pub const MF_UNCHECKED: UINT = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const MF_CHECKED: UINT = 0x00000008;
#[cfg (feature = "have_ntgui")]
pub const MF_POPUP: UINT = 0x00000010;
#[cfg (feature = "have_ntgui")]
pub const MF_OWNERDRAW: UINT = 0x00000100;
#[cfg (feature = "have_ntgui")]
pub const MF_MOUSESELECT: UINT = 0x00008000;
#[cfg (feature = "have_ntgui")]
pub const MF_HILITE: UINT = 0x00000080;
#[cfg (feature = "have_ntgui")]
pub const MF_BYPOSITION: UINT = 0x00000400;

/// `<windows.h>`/`<wingdi.h>`'s handle and colour types and `RECT`.  This unit's own text uses
/// `HMENU`/`HWND` only, but the mirrors it includes name the rest (`include/w32term.rs` 505's
/// `w32_output` has `HDC`, `HBITMAP`, `HCURSOR`, `COLORREF` members and `RECT` parameters) - a
/// Windows translation unit gets them from `<windows.h>` through the same chain, so the surface
/// stands here with `src/w32term.rs` 184-239's spellings.
/// C: typedef void *HDC; typedef void *HGDIOBJ; typedef void *HBITMAP; typedef void *HPALETTE;
/// C: typedef void *HCURSOR; typedef void *HFONT; typedef void *HBRUSH; typedef void *HRGN;
/// C: typedef DWORD COLORREF;
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
#[cfg (feature = "have_ntgui")]
pub type HDC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HBITMAP = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HPALETTE = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HCURSOR = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HFONT = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HBRUSH = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HRGN = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RECT
{
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}

/// `<windows.h>`'s `POINT` (C 770's `POINT pos;` and C 965-967) and the `MAKEINTRESOURCEW` form
/// C 133's `TD_INFORMATION_ICON` takes.
/// C: typedef struct tagPOINT { LONG x; LONG y; } POINT;
/// C: #define MAKEINTRESOURCEW(i) ((LPWSTR)((ULONG_PTR)((WORD)(i))))
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINT
{
  pub x: LONG,
  pub y: LONG,
}
#[cfg (feature = "have_ntgui")]
macro_rules! MAKEINTRESOURCEW { ($i:expr) => { (($i) as WORD as ULONG_PTR) as LPWSTR }; }

/// C 1323-1376's message-box type and answers (`<winuser.h>`), and C 1570's
/// `VER_PLATFORM_WIN32_NT` (`<winver.h>`), which the `MENU_DEBUG`-free `#ifndef NTGUI_UNICODE`
/// fallback reads through `osinfo_cache`.
/// C: #define MB_YESNO 0x00000004L / #define MB_YESNOCANCEL 0x00000003L
/// C: #define MB_ICONQUESTION 0x00000020L / #define MB_ICONINFORMATION 0x00000040L
/// C: #define IDOK 1 / #define IDCANCEL 2 / #define IDYES 6 / #define IDNO 7
/// C: #define VER_PLATFORM_WIN32s 0 / #define VER_PLATFORM_WIN32_WINDOWS 1
/// C: #define VER_PLATFORM_WIN32_NT 2
#[cfg (feature = "have_ntgui")]
pub const MB_YESNOCANCEL: UINT = 0x00000003;
#[cfg (feature = "have_ntgui")]
pub const MB_YESNO: UINT = 0x00000004;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONQUESTION: UINT = 0x00000020;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONINFORMATION: UINT = 0x00000040;
#[cfg (feature = "have_ntgui")]
pub const IDOK: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const IDCANCEL: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const IDYES: c_int = 6;
#[cfg (feature = "have_ntgui")]
pub const IDNO: c_int = 7;
#[cfg (feature = "have_ntgui")]
pub const VER_PLATFORM_WIN32_NT: DWORD = 2;

/// `<windows.h>`'s UTF-8 code page (C 207/237/259) and the four entry points C's text calls:
/// `SendMessage` (C 182/970), `ClientToScreen` (C 967), `GetModuleHandle` (C 1834/1840) and the
/// message box (C 1376).  Each is the SDK's A/W alias - C's text writes the alias and the port
/// keeps C's name, so a Windows target supplies whichever entry point its own `#define` picks
/// (PORTING.md 0.1's rename convention).
/// C: #define CP_UTF8 65001
/// C: LRESULT SendMessageA (HWND, UINT, WPARAM, LPARAM);
/// C: BOOL ClientToScreen (HWND, POINT *);
/// C: HMODULE GetModuleHandleA (LPCSTR);
/// C: int MessageBoxA (HWND, LPCSTR, LPCSTR, UINT);
#[cfg (feature = "have_ntgui")]
pub const CP_UTF8: UINT = 65001;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn SendMessage (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn ClientToScreen (hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
  pub fn GetModuleHandle (lpModuleName: LPCSTR) -> HMODULE;
  pub fn MessageBox (hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR,
                     uType: UINT) -> c_int;
}

/// `<windows.h>`'s menu entry points (C 688, 961, 737-740, 683, 982, 1754, 1789) and the two
/// dynamic-loading pairs C's `get_menu_item_info`/`set_menu_item_info` typedefs name - all
/// `WINAPI`, so `extern "system"`.
/// C: HMENU CreateMenu (void); HMENU CreatePopupMenu (void);
/// C: BOOL DestroyMenu (HMENU); BOOL DeleteMenu (HMENU, UINT, UINT);
/// C: BOOL SetMenu (HWND, HMENU); HMENU GetMenu (HWND); int GetMenuItemCount (HMENU);
/// C: BOOL AppendMenuA (HMENU, UINT, UINT_PTR, LPCSTR);
/// C: BOOL GetMenuItemInfoA (HMENU, UINT, BOOL, LPMENUITEMINFOA);
/// C: BOOL SetMenuItemInfoA (HMENU, UINT, BOOL, LPCMENUITEMINFOA);
/// C: int MessageBoxW (HWND, const WCHAR *, const WCHAR *, UINT);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn CreateMenu () -> HMENU;
  pub fn CreatePopupMenu () -> HMENU;
  pub fn DestroyMenu (hMenu: HMENU) -> BOOL;
  pub fn DeleteMenu (hMenu: HMENU, uPosition: UINT, uFlags: UINT) -> BOOL;
  pub fn SetMenu (hWnd: HWND, hMenu: HMENU) -> BOOL;
  pub fn GetMenu (hWnd: HWND) -> HMENU;
  pub fn GetMenuItemCount (hMenu: HMENU) -> c_int;
  pub fn AppendMenu (hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT_PTR,
                     lpNewItem: LPCSTR) -> BOOL;
  pub fn GetMenuItemInfoA (hMenu: HMENU, uItem: UINT, fByPosition: BOOL,
                           lpmii: LPMENUITEMINFOA) -> BOOL;
  pub fn SetMenuItemInfoA (hMenu: HMENU, uItem: UINT, fByPosition: BOOL,
                           lpmii: LPCMENUITEMINFOA) -> BOOL;
  pub fn MessageBoxW (hWnd: HWND, lpText: *const WCHAR, lpCaption: *const WCHAR,
                      uType: UINT) -> c_int;
}

/* C 55's `commctrl.h`: the menu-item record and the three flag families C's `add_menu_item`,
   `w32_menu_display_help` and `w32_free_submenu_strings` fill (`MIIM_*` for `fMask`, `MFT_*` for
   `fType`, `MFS_*` for `fState`), plus the two `TASKDIALOG`-adjacent names the section below
   needs.  */
/// C 67-76's two `LPMENUITEMINFOA`/`LPCMENUITEMINFOA` parameter types and C 1617's record.
/// C: typedef struct tagMENUITEMINFOA { UINT cbSize; UINT fMask; UINT fType; UINT fState;
/// C:   UINT wID; HMENU hSubMenu; HBITMAP hbmpChecked; HBITMAP hbmpUnchecked;
/// C:   ULONG_PTR dwItemData; LPSTR dwTypeData; UINT cch;
/// C:   HBITMAP hbmpItem; } MENUITEMINFOA, *LPMENUITEMINFOA;
/// C: typedef const MENUITEMINFOA *LPCMENUITEMINFOA;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct MENUITEMINFO
{
  pub cbSize: UINT,
  pub fMask: UINT,
  pub fType: UINT,
  pub fState: UINT,
  pub wID: UINT,
  pub hSubMenu: HMENU,
  pub hbmpChecked: HANDLE,
  pub hbmpUnchecked: HANDLE,
  pub dwItemData: ULONG_PTR,
  pub dwTypeData: LPSTR,
  pub cch: UINT,
  pub hbmpItem: HANDLE,
}
#[cfg (feature = "have_ntgui")]
pub type LPMENUITEMINFOA = *mut MENUITEMINFO;
#[cfg (feature = "have_ntgui")]
pub type LPCMENUITEMINFOA = *const MENUITEMINFO;

/* C 55's `commctrl.h`, `MINGW_W64` arm: with that macro defined C 83's `#ifndef` is skipped and
   the task-dialog surface comes from this header instead, so the same names stand here under the
   other arm of the pair (P-86 ①'s both-arms rule for the include graph's own header).  The values
   are `<commctrl.h>`'s and identical to C 83-135's own copy, so either arm reads one set.  */
/// C: typedef int TASKDIALOG_COMMON_BUTTON_FLAGS; typedef int TASKDIALOG_FLAGS;
/// C: typedef HRESULT (CALLBACK *PFTASKDIALOGCALLBACK) (HWND, UINT, WPARAM, LPARAM, LONG_PTR);
/// C: typedef struct _TASKDIALOG_BUTTON { int nButtonID; PCWSTR pszButtonText; }
/// C:   TASKDIALOG_BUTTON;
/// C: typedef struct _TASKDIALOGCONFIG { ... } TASKDIALOGCONFIG;
/// C: # define TDN_CREATED 0
/// C: # define TDM_ENABLE_BUTTON (WM_USER+111)
/// C: # define TDF_ALLOW_DIALOG_CANCELLATION 0x8
/// C: # define TD_INFORMATION_ICON MAKEINTRESOURCEW (-3)
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub type TASKDIALOG_COMMON_BUTTON_FLAGS = c_int;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub type TASKDIALOG_FLAGS = c_int;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub type PFTASKDIALOGCALLBACK
  = Option<unsafe extern "system" fn (HWND, UINT, WPARAM, LPARAM, LONG_PTR) -> HRESULT>;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
#[repr(C)]
pub struct _TASKDIALOG_BUTTON
{
  pub nButtonID: c_int,
  pub pszButtonText: PCWSTR,
}
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub type TASKDIALOG_BUTTON = _TASKDIALOG_BUTTON;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DUMMYUNIONNAME
{
  pub hMainIcon: HICON,
  pub pszMainIcon: PCWSTR,
}
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DUMMYUNIONNAME2
{
  pub hFooterIcon: HICON,
  pub pszFooterIcon: PCWSTR,
}
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
#[repr(C)]
pub struct _TASKDIALOGCONFIG
{
  pub cbSize: UINT,
  pub hwndParent: HWND,
  pub hInstance: HINSTANCE,
  pub dwFlags: TASKDIALOG_FLAGS,
  pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
  pub pszWindowTitle: PCWSTR,
  pub DUMMYUNIONNAME: DUMMYUNIONNAME,
  pub pszMainInstruction: PCWSTR,
  pub pszContent: PCWSTR,
  pub cButtons: UINT,
  pub pButtons: *const TASKDIALOG_BUTTON,
  pub nDefaultButton: c_int,
  pub cRadioButtons: UINT,
  pub pRadioButtons: *const TASKDIALOG_BUTTON,
  pub nDefaultRadioButton: c_int,
  pub pszVerificationText: PCWSTR,
  pub pszExpandedInformation: PCWSTR,
  pub pszExpandedControlText: PCWSTR,
  pub pszCollapsedControlText: PCWSTR,
  pub DUMMYUNIONNAME2: DUMMYUNIONNAME2,
  pub pszFooter: PCWSTR,
  pub pfCallback: PFTASKDIALOGCALLBACK,
  pub lpCallbackData: LONG_PTR,
  pub cxWidth: UINT,
}
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub type TASKDIALOGCONFIG = _TASKDIALOGCONFIG;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub const TDN_CREATED: UINT = 0;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
macro_rules! TDM_ENABLE_BUTTON { () => { (WM_USER + 111) as UINT }; }
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
pub const TDF_ALLOW_DIALOG_CANCELLATION: c_int = 0x8;
#[cfg (all (feature = "have_ntgui", feature = "mingw_w64"))]
macro_rules! TD_INFORMATION_ICON { () => { MAKEINTRESOURCEW (-3) }; }

/// C 1620/1643/1760's `fMask` bits, C 1644's `fType` bits and C 1646/1765's `fState` bit /
/// `fType` test.
/// C: #define MIIM_STATE 0x00000001 / #define MIIM_TYPE 0x00000010
/// C: #define MIIM_DATA 0x00000020 / #define MIIM_SUBMENU 0x00000004
/// C: #define MFT_STRING 0x00000000 / #define MFT_RADIOCHECK 0x00000200
/// C: #define MFS_CHECKED 0x00000008 / #define MFS_UNCHECKED 0x00000000
#[cfg (feature = "have_ntgui")]
pub const MIIM_STATE: UINT = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const MIIM_SUBMENU: UINT = 0x00000004;
#[cfg (feature = "have_ntgui")]
pub const MIIM_TYPE: UINT = 0x00000010;
#[cfg (feature = "have_ntgui")]
pub const MIIM_DATA: UINT = 0x00000020;
#[cfg (feature = "have_ntgui")]
pub const MFT_STRING: UINT = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const MFT_RADIOCHECK: UINT = 0x00000200;
#[cfg (feature = "have_ntgui")]
pub const MFS_CHECKED: UINT = 0x00000008;
#[cfg (feature = "have_ntgui")]
pub const MFS_UNCHECKED: UINT = 0x00000000;

/* C 39-45's `<mbstring.h>`: the MinGW multibyte helpers `add_menu_item` uses when
   `NTGUI_UNICODE` is off.  C's own text names them at C 1498/1500/1524/1527-1529.  */
/// C: unsigned char *_mbsinc (const unsigned char *);
/// C: unsigned int _mbsnextc (const unsigned char *);
/// C: unsigned char *_mbsncpy (unsigned char *, const unsigned char *, size_t);
#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
unsafe extern "C" {
  pub fn _mbsinc (current: *const u8) -> *mut u8;
  pub fn _mbsnextc (string: *const u8) -> c_uint;
  pub fn _mbsncpy (dest: *mut u8, src: *const u8, count: usize) -> *mut u8;
}

/* C 57-58: "This only applies to OS versions prior to Vista.  #undef HAVE_DIALOGS /* TODO:
   Implement native dialogs.  */".  The `#undef` is why C 159/324/1055's `HAVE_DIALOGS` arms are
   dead in every configuration; the port's feature is `have_dialogs` and the *negation* is the live
   arm (PORTING.md P-51's convention: the feature names the non-frozen side).  */
// C: #undef HAVE_DIALOGS /* TODO: Implement native dialogs.  */

/* C 60-63: `#ifndef TRUE / #define TRUE 1 / #define FALSE 0 / #endif /* no TRUE */`.  The frozen
   macro table has `TRUE`, so C's own arm is the dead one there and the `<windows.h>` arm above is
   live; `tools/check-cond-eval.py src/w32menu.c` reads this guard as clang's `True` and the
   evaluator's `False`, which is why the file is not on `cond-eval-verified.txt`.  Both arms are
   carried.  */
#[cfg (all (feature = "have_ntgui", not (feature = "true")))]
pub const TRUE: BOOL = 1;                        // C 61
#[cfg (all (feature = "have_ntgui", not (feature = "true")))]
pub const FALSE: BOOL = 0;                       // C 62

/* C 65: `HMENU current_popup_menu;` - a tentative C definition, i.e. zero.  `w32term.h` 225
   declares the same object (`include/w32term.rs` 289 carries that declaration at crate root), so
   this definition collides with the mirror's declaration as `E0428` under `windowsnt` - the
   `docs/ports/w32select.md` shape; the ledger's `## Owed` names the mirror row that fixes it.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut current_popup_menu: HMENU = std::ptr::null_mut ();

/* C 67-81: the three `WINAPI` proc typedefs whose entry points are looked up at run time
   (`globals_of_w32menu`).  C's `IN`/`OUT` are empty parameter annotations, dropped with the
   comment (PORTING.md 0.1-9); `WINAPI` is the `extern "system"` ABI and a C function pointer is
   nullable, so each typedef is an `Option<...>` (PORTING.md P-27).  The `_Proc` parameter names
   are C's own (`window`/`text`/`caption`/`type`).  */
/// C: typedef BOOL (WINAPI * GetMenuItemInfoA_Proc) (IN HMENU, IN UINT, IN BOOL,
/// C:   IN OUT LPMENUITEMINFOA);
/// C: typedef BOOL (WINAPI * SetMenuItemInfoA_Proc) (IN HMENU, IN UINT, IN BOOL,
/// C:   IN LPCMENUITEMINFOA);
/// C: typedef int (WINAPI * MessageBoxW_Proc) (IN HWND window, IN const WCHAR *text,
/// C:   IN const WCHAR *caption, IN UINT type);
#[cfg (feature = "have_ntgui")]
pub type GetMenuItemInfoA_Proc
  = Option<unsafe extern "system" fn (HMENU, UINT, BOOL, LPMENUITEMINFOA) -> BOOL>;
#[cfg (feature = "have_ntgui")]
pub type SetMenuItemInfoA_Proc
  = Option<unsafe extern "system" fn (HMENU, UINT, BOOL, LPCMENUITEMINFOA) -> BOOL>;
#[cfg (feature = "have_ntgui")]
pub type MessageBoxW_Proc
  = Option<unsafe extern "system" fn (HWND, *const WCHAR, *const WCHAR, UINT) -> c_int>;

/* C 83-135: `#ifndef MINGW_W64` - "mingw.org's MinGW doesn't have this in its header files", so
   C declares the task-dialog surface itself.  The `#else` arm is `<commctrl.h>`'s own copy, which
   `MINGW_W64`'s toolchain supplies; both arms are carried, and both declare the same names with
   the same values, so a target that defines `MINGW_W64` reads the same constants (the
   `docs/ports/w32select.md` `CF_DIBV5` reading of the same guard class).  */
/// C: typedef int TASKDIALOG_COMMON_BUTTON_FLAGS; typedef int TASKDIALOG_FLAGS;
/// C: typedef HRESULT (CALLBACK *PFTASKDIALOGCALLBACK) (HWND, UINT, WPARAM, LPARAM, LONG_PTR);
/// C: typedef struct _TASKDIALOG_BUTTON { int nButtonID; PCWSTR pszButtonText; }
/// C:   TASKDIALOG_BUTTON;
/// C: typedef struct _TASKDIALOGCONFIG { UINT cbSize; HWND hwndParent; HINSTANCE hInstance;
/// C:   TASKDIALOG_FLAGS dwFlags; TASKDIALOG_COMMON_BUTTON_FLAGS dwCommonButtons;
/// C:   PCWSTR pszWindowTitle; union { HICON hMainIcon; PCWSTR pszMainIcon; } DUMMYUNIONNAME;
/// C:   PCWSTR pszMainInstruction; PCWSTR pszContent; UINT cButtons;
/// C:   const TASKDIALOG_BUTTON *pButtons; int nDefaultButton; UINT cRadioButtons;
/// C:   const TASKDIALOG_BUTTON *pRadioButtons; int nDefaultRadioButton;
/// C:   PCWSTR pszVerificationText; PCWSTR pszExpandedInformation;
/// C:   PCWSTR pszExpandedControlText; PCWSTR pszCollapsedControlText;
/// C:   union { HICON hFooterIcon; PCWSTR pszFooterIcon; } DUMMYUNIONNAME2; PCWSTR pszFooter;
/// C:   PFTASKDIALOGCALLBACK pfCallback; LONG_PTR lpCallbackData; UINT cxWidth; }
/// C:   TASKDIALOGCONFIG;
/// C: typedef HRESULT (WINAPI *TaskDialogIndirect_Proc) (IN const TASKDIALOGCONFIG *pTaskConfig,
/// C:   OUT int *pnButton, OUT int *pnRadioButton, OUT BOOL *pfVerificationFlagChecked);
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type TASKDIALOG_COMMON_BUTTON_FLAGS = c_int;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type TASKDIALOG_FLAGS = c_int;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type PFTASKDIALOGCALLBACK
  = Option<unsafe extern "system" fn (HWND, UINT, WPARAM, LPARAM, LONG_PTR) -> HRESULT>;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct _TASKDIALOG_BUTTON
{
  pub nButtonID: c_int,
  pub pszButtonText: PCWSTR,
}
/// C 95: `} TASKDIALOG_BUTTON;`
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type TASKDIALOG_BUTTON = _TASKDIALOG_BUTTON;

/// C 104-107 and C 120-123's two unions.  The SDK defines `DUMMYUNIONNAME` as `u` and
/// `DUMMYUNIONNAME2` as `u2` (`<winnt.h>`'s compatibility macros for compilers without anonymous
/// unions), which is why C's text may write the same member name twice; the port declares the two
/// records under those substitutions and reaches their members through the member name.
/// C: #define DUMMYUNIONNAME u
/// C: #define DUMMYUNIONNAME2 u2
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DUMMYUNIONNAME
{
  pub hMainIcon: HICON,
  pub pszMainIcon: PCWSTR,
}
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DUMMYUNIONNAME2
{
  pub hFooterIcon: HICON,
  pub pszFooterIcon: PCWSTR,
}

#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
#[repr(C)]
pub struct _TASKDIALOGCONFIG
{
  pub cbSize: UINT,
  pub hwndParent: HWND,
  pub hInstance: HINSTANCE,
  pub dwFlags: TASKDIALOG_FLAGS,
  pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
  pub pszWindowTitle: PCWSTR,
  pub DUMMYUNIONNAME: DUMMYUNIONNAME,
  pub pszMainInstruction: PCWSTR,
  pub pszContent: PCWSTR,
  pub cButtons: UINT,
  pub pButtons: *const TASKDIALOG_BUTTON,
  pub nDefaultButton: c_int,
  pub cRadioButtons: UINT,
  pub pRadioButtons: *const TASKDIALOG_BUTTON,
  pub nDefaultRadioButton: c_int,
  pub pszVerificationText: PCWSTR,
  pub pszExpandedInformation: PCWSTR,
  pub pszExpandedControlText: PCWSTR,
  pub pszCollapsedControlText: PCWSTR,
  pub DUMMYUNIONNAME2: DUMMYUNIONNAME2,
  pub pszFooter: PCWSTR,
  pub pfCallback: PFTASKDIALOGCALLBACK,
  pub lpCallbackData: LONG_PTR,
  pub cxWidth: UINT,
}
/// C 128: `} TASKDIALOGCONFIG;`
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub type TASKDIALOGCONFIG = _TASKDIALOGCONFIG;

/// C 130-133: the four macros of the `#ifndef MINGW_W64` arm.  Three are literal-valued
/// constants; `TD_INFORMATION_ICON` is an expression over `MAKEINTRESOURCEW`, so it stays a
/// macro with C's own body.
/// C: # define TDN_CREATED 0
/// C: # define TDM_ENABLE_BUTTON (WM_USER+111)
/// C: # define TDF_ALLOW_DIALOG_CANCELLATION 0x8
/// C: # define TD_INFORMATION_ICON MAKEINTRESOURCEW (-3)
/* C's `# define` has no type, and both uses want a `UINT`: the `match` below is over the
   callback's `UINT msg`, and `SendMessage`'s `Msg` parameter is a `UINT`.  `WM_USER` stays a
   `c_int` because `include/w32term.rs` 907's `WM_EMACS_START` is `WM_USER + 1` in that type.  */
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub const TDN_CREATED: UINT = 0;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
macro_rules! TDM_ENABLE_BUTTON { () => { (WM_USER + 111) as UINT }; }
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
pub const TDF_ALLOW_DIALOG_CANCELLATION: c_int = 0x8;
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64")))]
macro_rules! TD_INFORMATION_ICON { () => { MAKEINTRESOURCEW (-3) }; }

/// C 137-141: `TaskDialogIndirect_Proc`, the entry point `globals_of_w32menu` looks up on
/// `comctl32.dll`.  C 89's typedef and this one are the two halves of the same surface, and this
/// one sits *outside* C 83's guard, so it carries the file gate alone.
/// C: typedef HRESULT (WINAPI *TaskDialogIndirect_Proc) (IN const TASKDIALOGCONFIG *pTaskConfig,
/// C:   OUT int *pnButton, OUT int *pnRadioButton, OUT BOOL *pfVerificationFlagChecked);
#[cfg (feature = "have_ntgui")]
pub type TaskDialogIndirect_Proc
  = Option<unsafe extern "system" fn (*const TASKDIALOGCONFIG, *mut c_int, *mut c_int,
                                      *mut BOOL) -> HRESULT>;

/* C 143-155: `#ifdef NTGUI_UNICODE` / `#else /* !NTGUI_UNICODE */` - the six file-scope objects.
   The `#if` arm starts three of them at the SDK entry point and the `#else` arm at `NULL`; C
   declares `unicode_append_menu` twice (an `extern` prototype and the definition), and the
   prototype is dropped (PORTING.md 0.1-12).  C's `NULL` is `core::ptr::null_mut ()` and the
   function's address is `Some (...)`.  Both arms are carried.  */
/// C: static GetMenuItemInfoA_Proc get_menu_item_info = GetMenuItemInfoA;
/// C: static SetMenuItemInfoA_Proc set_menu_item_info = SetMenuItemInfoA;
/// C: static MessageBoxW_Proc unicode_message_box = MessageBoxW;
/// C: static GetMenuItemInfoA_Proc get_menu_item_info = NULL;
/// C: static SetMenuItemInfoA_Proc set_menu_item_info = NULL;
/// C: static MessageBoxW_Proc unicode_message_box = NULL;
/// C: extern AppendMenuW_Proc unicode_append_menu;
/// C: AppendMenuW_Proc unicode_append_menu = AppendMenuW;    (and `= NULL` in the `#else` arm)
#[cfg (all (feature = "have_ntgui", feature = "ntgui_unicode"))]
static mut get_menu_item_info: GetMenuItemInfoA_Proc = Some (GetMenuItemInfoA);
#[cfg (all (feature = "have_ntgui", feature = "ntgui_unicode"))]
static mut set_menu_item_info: SetMenuItemInfoA_Proc = Some (SetMenuItemInfoA);
#[cfg (all (feature = "have_ntgui", feature = "ntgui_unicode"))]
static mut unicode_message_box: MessageBoxW_Proc = Some (MessageBoxW);
#[cfg (all (feature = "have_ntgui", feature = "ntgui_unicode"))]
#[unsafe(no_mangle)]
pub static mut unicode_append_menu: AppendMenuW_Proc = Some (AppendMenuW);

#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
static mut get_menu_item_info: GetMenuItemInfoA_Proc = None;
#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
static mut set_menu_item_info: SetMenuItemInfoA_Proc = None;
#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
static mut unicode_message_box: MessageBoxW_Proc = None;
#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
#[unsafe(no_mangle)]
pub static mut unicode_append_menu: AppendMenuW_Proc = None;

/// C 157: `static TaskDialogIndirect_Proc task_dialog_indirect;` - a tentative `static`, i.e.
/// `NULL`.
#[cfg (feature = "have_ntgui")]
static mut task_dialog_indirect: TaskDialogIndirect_Proc = None;

/* C 159-164: the two arms' forward declarations of the three static dialog helpers.  C declares
   them because the definitions follow later; the port's items are order-independent, so they are
   dropped (PORTING.md 0.1-12) and their text stays here:

     #ifdef HAVE_DIALOGS
     static Lisp_Object w32_dialog_show (struct frame *, Lisp_Object, Lisp_Object, char **);
     #else
     static bool is_simple_dialog (Lisp_Object);
     static Lisp_Object simple_dialog_show (struct frame *, Lisp_Object, Lisp_Object);
     #endif

   C 166-169's three more (`utf8to16`, `fill_in_menu`, `w32_free_menu_strings`) are the same
   shape:

     static void utf8to16 (unsigned char *, int, WCHAR *);
     static int fill_in_menu (HMENU, widget_value *);
     void w32_free_menu_strings (HWND);
*/

/// C 171: `#define TASK_DIALOG_MAX_BUTTONS 10` - C 181's loop bound, C 212-213's array bound and
/// C 219's limit.  The port's arrays take a `usize`, which is why two sites spell `as usize`.
#[cfg (feature = "have_ntgui")]
pub const TASK_DIALOG_MAX_BUTTONS: c_int = 10;

/* C 191-195's `MultiByteToWideChar` surface - hoisted to file scope because the `#else` arm's
   object below has to be typed by it, and because C's uses at C 207/209/237/240/259/263 sit
   outside the `#ifdef` that declares the local.  C's local typedef keeps C's parameter list.  */
/// C: typedef int (WINAPI *MultiByteToWideChar_Proc)(UINT,DWORD,LPCSTR,int,LPWSTR, int);
#[cfg (feature = "have_ntgui")]
pub type MultiByteToWideChar_Proc
  = Option<unsafe extern "system" fn (UINT, DWORD, LPCSTR, c_int, LPWSTR, c_int) -> c_int>;
/// C 207's `pMultiByteToWideChar` in the `#else` arm is `src/w32.c` 537's file-scope object (the
/// one `src/systhread.c` 440 declares `extern`): C's `#ifdef NTGUI_UNICODE` arm declares a local
/// `static` of the same name, and the `#else` arm is left with whatever is in scope - which, for
/// the MinGW build, is that object.
/// C: int (WINAPI *pMultiByteToWideChar)(UINT,DWORD,LPCSTR,int,LPWSTR,int);
#[cfg (all (feature = "have_ntgui", not (feature = "ntgui_unicode")))]
unsafe extern "C" {
  pub static mut pMultiByteToWideChar: MultiByteToWideChar_Proc;
}
/// `<windows.h>`'s `MultiByteToWideChar`, which C 194's local `static` starts from.
/// C: int MultiByteToWideChar (UINT, DWORD, LPCSTR, int, LPWSTR, int);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn MultiByteToWideChar (CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCSTR,
                              cbMultiByte: c_int, lpWideCharStr: LPWSTR,
                              cchWideChar: c_int) -> c_int;
}

/// C 287/290 and C 1354/1359's `L"Question"`/`L"Information"` and C 1336's `L""`: a C wide string
/// literal has no Rust spelling (Rust's `c"..."` is a narrow `&CStr`), so the port writes the array
/// each literal denotes - the code units of C's ASCII text plus C's terminator - and takes its
/// address, which is what C's literal decays to.  The ledger's `## Deviations` records the three.
/// C: config.pszWindowTitle = L"Question";
/// C: title = L"Question";
#[cfg (feature = "have_ntgui")]
static WIDE_Question: [WCHAR; 9]
  = [0x0051, 0x0075, 0x0065, 0x0073, 0x0074, 0x0069, 0x006f, 0x006e, 0x0000];
/// C: config.pszWindowTitle = L"Information";
/// C: title = L"Information";
#[cfg (feature = "have_ntgui")]
static WIDE_Information: [WCHAR; 12]
  = [0x0049, 0x006e, 0x0066, 0x006f, 0x0072, 0x006d, 0x0061, 0x0074, 0x0069, 0x006f, 0x006e,
     0x0000];
/// C: text = (WCHAR *)L"";
#[cfg (feature = "have_ntgui")]
static WIDE_EMPTY: [WCHAR; 1] = [0x0000];

/* C 65-171's remaining in-unit declarations.  Each is a name C reaches through the headers the
   include graph above carries, and each is declared here because no mirror in this graph carries
   it yet (P-72 rule ④: a mirror declares only what its own `include!`s give it).  Every line names
   the C header it comes from and the ledger's `## Owed` records the move; until it lands, the unit
   supplies the declaration at the use site - the `src/androidmenu.rs` 92-104 shape.  */
/// C: extern Lisp_Object menu_bar_items (Lisp_Object);                  (lisp.h 5087)
/// C: extern void record_unwind_save_match_data (void);                 (lisp.h 4998)
/// C: extern void discard_mouse_events (void);                          (lisp.h 5090)
/// C: extern void safe_run_hooks (Lisp_Object);                         (lisp.h 5097)
/// C: extern void quit (void) NO_RETURN;                                (lisp.h 4786)
/// C: extern void show_help_echo (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object);
/// C: extern void kbd_buffer_store_help_event (Lisp_Object, Lisp_Object);
/// C: extern WINDOW_SYSTEM_RETURN void check_window_system (struct frame *);  (frame.h 1504)
/// C: extern void adjust_frame_size (struct frame *, int, int, int, bool, Lisp_Object);
/// C: extern bool menu_separator_name_p (const char *);                 (keyboard.h 473)
/// C: extern widget_value *make_widget_value (const char *, char *, bool, Lisp_Object);
/// C: extern void free_menubar_widget_value_tree (widget_value *);
/// C: extern widget_value *digest_single_submenu (int, int, bool);
/// C: extern void update_submenu_strings (widget_value *);
/// C: LWLIB_ID widget_id_tick;  (src/xmenu.c 1610-1611, "FIXME: Move this to a .h file")
/// C: int dialog_id; Lisp_Object *dialog_selection_callback;  (C 1197-1200's names)
/// C: void popup_get_selection (XEvent *, struct w32_display_info *, LWLIB_ID);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {









  pub fn menu_separator_name_p (name: *const c_char) -> bool;
  pub fn make_widget_value (name: *const c_char, value: *mut c_char, enabled: bool,
                            help: Lisp_Object) -> *mut widget_value;
  pub fn free_menubar_widget_value_tree (wv: *mut widget_value);
  pub fn digest_single_submenu (start: c_int, end: c_int,
                                top_level_items: bool) -> *mut widget_value;
  pub fn update_submenu_strings (first_wv: *mut widget_value);
}

/// C 1197-1213's names.  `widget_id_tick` is `LWLIB_ID` (lwlib.h 30's `typedef unsigned long
/// LWLIB_ID`), `dialog_id` and `menu` are `xmenu.c`'s locals that C's text here reads as if they
/// were objects, and `dialog_selection_callback` is that file's callback.  C's `HAVE_DIALOGS` arm
/// cannot be compiled in C either (the file `#undef`s the macro at C 58 and the names are not
/// declared anywhere reachable); the ledger's `## Not claimed` says so.
/// C: dialog_id = widget_id_tick++;
/// C: menu = lw_create_widget (first_wv->name, "dialog", dialog_id, first_wv, ...
/// C:   dialog_selection_callback, 0);
#[cfg (all (feature = "have_ntgui", feature = "have_dialogs"))]
unsafe extern "C" {
  pub static mut widget_id_tick: c_ulong;
  pub static mut dialog_id: c_ulong;
  pub static mut menu: *mut c_void;
  pub static mut dialog_selection_callback: *mut c_void;
  pub fn popup_get_selection (initial_event: *mut c_void, dpyinfo: *mut w32_display_info,
                              id: c_ulong);
}

/// `lwlib.h` 81-96's four widget entry points C 1198-1215 calls.  They are `lwlib/lwlib.c`'s
/// definitions; C's `HAVE_DIALOGS` arm is the only user in this file.
/// C: Widget lw_create_widget (const char* type, const char* name, LWLIB_ID id, widget_value* val,
/// C:   Widget parent, int pop_up_p, void *pre_activate_cb, void *selection_cb, void *post_activate_cb);
/// C: void lw_modify_all_widgets (LWLIB_ID id, widget_value* val, Boolean deep_p);
/// C: void lw_pop_up_all_widgets (LWLIB_ID id); void lw_destroy_all_widgets (LWLIB_ID id);
#[cfg (all (feature = "have_ntgui", feature = "have_dialogs"))]
unsafe extern "C" {
  pub fn lw_create_widget (type_: *const c_char, name: *const c_char, id: c_ulong,
                           val: *mut widget_value, parent: *mut c_void, pop_up_p: c_int,
                           pre_activate_cb: *mut c_void, selection_cb: *mut c_void,
                           post_activate_cb: *mut c_void) -> *mut c_void;
  pub fn lw_modify_all_widgets (id: c_ulong, val: *mut widget_value, deep_p: c_int);
  pub fn lw_pop_up_all_widgets (id: c_ulong);
  pub fn lw_destroy_all_widgets (id: c_ulong);
}

/// `MENU_DEBUG`'s printer (C 1547/1768).  C's `<windows.h>` declares `DebPrint` as a no-op
/// macro under `NDEBUG` and as `printf`-like output otherwise; the feature is the non-frozen side
/// (PORTING.md P-51), so the port declares what its arm calls.
/// C: void DebPrint (const char *fmt, ...);
#[cfg (all (feature = "have_ntgui", feature = "menu_debug"))]
unsafe extern "C" {
  pub fn DebPrint (fmt: *const c_char, ...);
}

/// termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
/// `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80).  C 437/445/452/459/466 write
/// `buf.kind = MENU_BAR_EVENT;`, so the port keeps C's field name in a per-unit macro - the
/// `src/gfilenotify.rs` 174-183 shape, body-identical.
#[cfg (feature = "have_ntgui")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/* C 374/462/470/487/986/1806's `f->output_data.w32->menubar_active`.  Every use writes C's own
   field path - `(*( (*f).output_data.w32 as *mut w32_output ))` is `src/w32term.rs` 1553's spelling
   of `(*f).output_data.w32` - which is also what hands the body token gate `output_data`, `w32`
   and `menubar_active`.  The member itself is one of the bitfields `include/w32term.rs` 393-400
   *records* rather than packs (PORTING.md 1.3 / P-80 need the real compiler's byte dump before a
   storage unit may stand for a bitfield, and this record cannot be compiled on this target), so
   it does not resolve yet; the ledger's `## Owed` carries the mirror row that gives it a unit.  */

/// `<winerror.h>`'s success test, which C 298's `if (!SUCCEEDED (...))` uses.
/// C: #define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#[cfg (feature = "have_ntgui")]
macro_rules! SUCCEEDED { ($hr:expr) => { (($hr) as HRESULT) >= 0 }; }

/* C 173-186: `task_dialog_callback` - the `TaskDialogIndirect` callback C 286 hands to the
   dialog.  C's `switch (msg)` is the `match` below and its `break` the end of the arm
   (PORTING.md 2.2/P-04; the ledger's note row records the three tokens).  */
/// C: static HRESULT CALLBACK task_dialog_callback (HWND hwnd, UINT msg, WPARAM wParam,
/// C:   LPARAM lParam, LONG_PTR callback_data)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn task_dialog_callback (hwnd: HWND, msg: UINT, wParam: WPARAM,
                                                lParam: LPARAM, callback_data: LONG_PTR)
                                                -> HRESULT
{
  match msg                                                   // C 177
    {
      TDN_CREATED =>                                          // C 179
        {
          /* Disable all buttons with ID >= 2000  */
          let mut i: c_int = 0;                               // C 181
          while i < TASK_DIALOG_MAX_BUTTONS
            {
              SendMessage (hwnd, TDM_ENABLE_BUTTON! (), (2000 + i) as WPARAM,
                           FALSE as LPARAM);                  // C 182
              i += 1;
            }
        }
      _ => {}                                                 // C 177's other cases: none
    }
  return S_OK;                                                // C 185
}

/* C 188-366: `w32_popup_dialog`.  C's `TASKDIALOG_BUTTON buttons[…]` and
   `Lisp_Object button_values[…]` are C automatic arrays C leaves uninitialised on the entries it
   does not fill; the port's are zero-initialised (PORTING.md 1.5), and `Lisp_Object`'s zero is
   `Qnil` (`NIL_IS_ZERO`).  C's bare `alloca` at C 208/239/262 is the port's stack pool
   (PORTING.md P-05's decision, the `src/xrdb.rs` 63 reading), which the ledger's note row
   records.  */
/// C: Lisp_Object w32_popup_dialog (struct frame *f, Lisp_Object header, Lisp_Object contents)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_popup_dialog (f: *mut frame, header: Lisp_Object,
                                           contents: Lisp_Object) -> Lisp_Object
{
  #[cfg (feature = "ntgui_unicode")]
  static mut pMultiByteToWideChar: MultiByteToWideChar_Proc = Some (MultiByteToWideChar);  // C 194
  check_window_system (f);                                    // C 196

  if task_dialog_indirect.is_some ()                          // C 198
    {
      let mut wide_len: c_int;                                // C 200

      CHECK_CONS (contents);                                  // C 202

      /* Get the title as an UTF-16 string.  */
      let title: *mut c_char = SSDATA (ENCODE_UTF_8 (XCAR (contents)));   // C 205
      wide_len = (core::mem::size_of::<WCHAR> () as c_int)                // C 206-207
	* pMultiByteToWideChar.unwrap () (CP_UTF8, 0, title, -1, std::ptr::null_mut (), 0);
      let title_w: *mut WCHAR = sa_stack_alloc (wide_len as usize) as *mut WCHAR;   // C 208
      pMultiByteToWideChar.unwrap () (CP_UTF8, 0, title, -1, title_w, wide_len);    // C 209

      /* Prepare the arrays with the dialog's buttons and return values.  */
      let mut buttons: [TASKDIALOG_BUTTON; TASK_DIALOG_MAX_BUTTONS as usize]
	= core::mem::zeroed ();                               // C 212
      let mut button_values: [Lisp_Object; TASK_DIALOG_MAX_BUTTONS as usize]
	= core::mem::zeroed ();                               // C 213
      let mut button_count: c_int = 0;                        // C 214
      let mut b: Lisp_Object = XCDR (contents);               // C 215

      while !NILP (b)                                         // C 217
	{
	  if button_count >= TASK_DIALOG_MAX_BUTTONS           // C 219
	    {
	      /* We have too many buttons.  We ignore the rest.  */
	      break;
	    }

	  let item: Lisp_Object = XCAR (b);                     // C 225

	  if CONSP (item)                                       // C 227
	    {
	      /* A normal item (text . value)  */
	      let mut item_name: Lisp_Object = XCAR (item);     // C 230
	      let item_value: Lisp_Object = XCDR (item);        // C 231

	      CHECK_STRING (item_name);                         // C 233

	      item_name = ENCODE_UTF_8 (item_name);             // C 235
	      wide_len = (core::mem::size_of::<WCHAR> () as c_int)      // C 236-238
		* pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (item_name), -1,
						  std::ptr::null_mut (), 0);
	      buttons[button_count as usize].pszButtonText
		= sa_stack_alloc (wide_len as usize) as PCWSTR;          // C 239
	      pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (item_name), -1,   // C 240-243
			    buttons[button_count as usize].pszButtonText as LPWSTR,
			    wide_len);
	      buttons[button_count as usize].nButtonID = 1000 + button_count;       // C 244
	      button_values[button_count as usize] = item_value;                     // C 245
	      button_count += 1;
	    }
	  else if NILP (item)                                   // C 247
	    {
	      /* A nil item means to put all following items on the
		 right.  We ignore this.  */
	    }
	  else if STRINGP (item)                                // C 252
	    {
	      /* A string item means an unselectable button.  We add a
	       button, and then need to disable it on the callback.  We
	       use ids based on 2000 to mark these buttons.  */
	      let item_name: Lisp_Object = ENCODE_UTF_8 (item); // C 257
	      wide_len = (core::mem::size_of::<WCHAR> () as c_int)      // C 258-261
		* pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (item_name), -1,
						  std::ptr::null_mut (), 0);
	      buttons[button_count as usize].pszButtonText
		= sa_stack_alloc (wide_len as usize) as PCWSTR;          // C 262
	      pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (item_name), -1,   // C 263-266
			    buttons[button_count as usize].pszButtonText as LPWSTR,
			    wide_len);
	      buttons[button_count as usize].nButtonID = 2000 + button_count;       // C 267
	      button_values[button_count as usize] = Qnil ();                        // C 268
	      button_count += 1;
	    }
	  else
	    {
	      error (c"Incorrect dialog button specification".as_ptr ());   // C 272
	      return Qnil ();
	    }

	  b = XCDR (b);                                         // C 276
	}

      let mut pressed_button: c_int = 0;                      // C 279

      let mut config: TASKDIALOGCONFIG = core::mem::zeroed ();   // C 281
      config.hwndParent = FRAME_W32_WINDOW! (f);              // C 282
      config.cbSize = core::mem::size_of::<TASKDIALOGCONFIG> () as UINT;   // C 283
      config.hInstance = hinst;                               // C 284
      config.dwFlags = TDF_ALLOW_DIALOG_CANCELLATION;         // C 285
      config.pfCallback = Some (task_dialog_callback);        // C 286
      config.pszWindowTitle = WIDE_Question.as_ptr ();        // C 287
      if !NILP (header)                                       // C 288
	{
	  config.pszWindowTitle = WIDE_Information.as_ptr ();   // C 290
	  config.DUMMYUNIONNAME.pszMainIcon = TD_INFORMATION_ICON! ();   // C 291
	}

      config.pszMainInstruction = title_w;                    // C 294
      config.pButtons = buttons.as_ptr ();                    // C 295
      config.cButtons = button_count as UINT;                 // C 296

      if !SUCCEEDED! (task_dialog_indirect.unwrap () (&raw mut config,   // C 298-299
						     &raw mut pressed_button,
						     std::ptr::null_mut (),
						     std::ptr::null_mut ()))
	{ quit (); }                                          // C 300

      let button_index: c_int;                                // C 302
      match pressed_button                                    // C 303
	{
	  IDOK =>                                             // C 305
	    {
	      /* This can only happen if no buttons were provided.  The OK
		 button is automatically added by TaskDialogIndirect in that
		 case.  */
	      return Qt ();                                   // C 309
	    }
	  IDCANCEL =>                                         // C 310
	    {
	      /* The user closed the dialog without using the buttons.  */
	      return quit ();                                 // C 312
	    }
	  _ =>                                                // C 313's `default:`
	    {
	      /* One of the specified buttons.  */
	      button_index = pressed_button - 1000;           // C 315
	      if button_index >= 0 && button_index < button_count    // C 316
		{ return button_values[button_index as usize]; }     // C 317
	      return quit ();                                 // C 318
	    }
	}
    }

  /* If we get here, TaskDialog is not supported.  Use MessageBox/Menu.  */

  /* C 324-329: `#ifndef HAVE_DIALOGS` - the live arm on every configuration this file reaches
     (C 58 `#undef`s the macro).  */
  #[cfg (not (feature = "have_dialogs"))]
  {
    /* Handle simple Yes/No choices as MessageBox popups.  */
    if is_simple_dialog (contents)                            // C 326
      { return simple_dialog_show (f, contents, header); }     // C 327
    else
      { return Qunsupported__w32_dialog (); }                  // C 329
  }
  /* C 330-352: `#else /* HAVE_DIALOGS */` - the arm C's text has but cannot build (C 58 turns
     the macro off and the names `w32_dialog_show` needs are `src/xmenu.c`'s).  */
  #[cfg (feature = "have_dialogs")]
  {
    let mut title: Lisp_Object;                               // C 332
    let mut error_name: *mut c_char;                          // C 333
    let selection: Lisp_Object;                               // C 334

    /* Decode the dialog items from what was specified.  */
    title = Fcar (contents);                                  // C 337
    CHECK_STRING (title);                                     // C 338

    list_of_panes (Fcons (contents, Qnil ()));                // C 340

    /* Display them in a dialog box.  */
    block_input ();                                           // C 343
    selection = w32_dialog_show (f, title, header, &raw mut error_name);   // C 344
    unblock_input ();                                         // C 345

    discard_menu_items ();                                    // C 347
    (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                   // C 348

    if !error_name.is_null () { error (error_name); }         // C 350
    return selection;                                         // C 351
  }
}

/* C 356-366: "Activate the menu bar of frame F.  …" - carried in C's own words where it stands:

   /* Activate the menu bar of frame F.
      This is called from keyboard.c when it gets the
      MENU_BAR_ACTIVATE_EVENT out of the Emacs event queue.

      To activate the menu bar, we signal to the input thread that it can
      return from the WM_INITMENU message, allowing the normal Windows
      processing of the menus.

      But first we recompute the menu bar contents (the whole tree).

      This way we can safely execute Lisp code.  */
*/

/// C: void w32_activate_menubar (struct frame *f)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_activate_menubar (f: *mut frame)     // C 368-376
{
  set_frame_menubar (f, true);                                // C 371

  /* Lock out further menubar changes while active.  */
  (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 1;                              // C 374

  /* Signal input thread to return from WM_INITMENU.  */
  complete_deferred_msg (FRAME_W32_WINDOW! (f), WM_INITMENU, 0);   // C 377
}

/* C 379-384: the callback's banner comment ("This callback is called from the menu bar pulldown
   menu when the user makes a selection. …") and its forward declaration.  The declaration is
   dropped (PORTING.md 0.1-12); C's text stays here:

     /* This callback is called from the menu bar pulldown menu
        when the user makes a selection.
        Figure out what the user chose
        and put the appropriate events into the keyboard buffer.  */
     void menubar_selection_callback (struct frame *, void *);
*/

/* C 386-473: `menubar_selection_callback`.  C's `alloca (f->menu_bar_items_used * word_size)` is
   the port's stack pool (PORTING.md P-05); `struct input_event buf` is zero-initialised then
   `EVENT_INIT`ed exactly as C's two statements do (the `src/dbusbind.rs` 2530-2533 shape), and
   C's `buf.kind = MENU_BAR_EVENT;` keeps C's field name through `input_event_bits!`.  */
/// C: void menubar_selection_callback (struct frame *f, void * client_data)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn menubar_selection_callback (f: *mut frame, client_data: *mut c_void)
{
  let mut prefix: Lisp_Object;                                // C 389
  let mut entry: Lisp_Object;                                 // C 389
  let vector: Lisp_Object;                                    // C 390
  let subprefix_stack: *mut Lisp_Object;                      // C 391
  let mut submenu_depth: c_int = 0;                           // C 392
  let mut i: c_int;                                           // C 393

  if f.is_null ()                                             // C 395
    { return; }
  entry = Qnil ();                                            // C 397
  subprefix_stack = sa_stack_alloc (((*f).menu_bar_items_used as isize * word_size)
				    as usize) as *mut Lisp_Object;    // C 398
  vector = (*f).menu_bar_vector;                              // C 399
  prefix = Qnil ();                                           // C 400
  i = 0;                                                      // C 401
  while i < (*f).menu_bar_items_used                           // C 402
    {
      if NILP (AREF (vector, i as isize))                      // C 404
	{
	  *subprefix_stack.offset (submenu_depth as isize) = prefix;   // C 406
	  submenu_depth += 1;
	  prefix = entry;
	  i += 1;
	}
      else if EQ (AREF (vector, i as isize), Qlambda ())       // C 408
	{
	  submenu_depth -= 1;
	  prefix = *subprefix_stack.offset (submenu_depth as isize);   // C 410
	  i += 1;
	}
      else if EQ (AREF (vector, i as isize), Qt ())            // C 412
	{
	  prefix = AREF (vector, i as isize + MENU_ITEMS_PANE_PREFIX as isize);   // C 414
	  i += MENU_ITEMS_PANE_LENGTH;                          // C 415
	}
      else
	{
	  entry = AREF (vector, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 419
	  /* The UINT_PTR cast avoids a warning.  There's no problem
	     as long as pointers have enough bits to hold small integers.  */
	  if client_data as UINT_PTR as c_int == i                 // C 422
	    {
	      let mut j: c_int;                                  // C 424
	      let mut buf: input_event = std::mem::zeroed ();     // C 425
	      let mut frame: Lisp_Object;                        // C 426
	      EVENT_INIT (&raw mut buf);                         // C 427

	      XSETFRAME! (frame, f);                             // C 429
	      input_event_bits! (kind, &raw mut buf, MENU_BAR_EVENT);   // C 430
	      buf.frame_or_window = frame;                       // C 431
	      buf.arg = frame;                                   // C 432
	      kbd_buffer_store_event (&raw mut buf as *mut c_void);    // C 433

	      j = 0;                                             // C 438
	      while j < submenu_depth
		{
		  if !NILP (*subprefix_stack.offset (j as isize))   // C 439
		    {
		      input_event_bits! (kind, &raw mut buf, MENU_BAR_EVENT);   // C 441
		      buf.frame_or_window = frame;                   // C 442
		      buf.arg = *subprefix_stack.offset (j as isize);   // C 443
		      kbd_buffer_store_event (&raw mut buf as *mut c_void);   // C 444
		    }
		  j += 1;
		}

	      if !NILP (prefix)                                  // C 447
		{
		  input_event_bits! (kind, &raw mut buf, MENU_BAR_EVENT);   // C 449
		  buf.frame_or_window = frame;                     // C 450
		  buf.arg = prefix;                                // C 451
		  kbd_buffer_store_event (&raw mut buf as *mut c_void);   // C 452
		}

	      input_event_bits! (kind, &raw mut buf, MENU_BAR_EVENT);   // C 455
	      buf.frame_or_window = frame;                         // C 456
	      buf.arg = entry;                                     // C 457
	      /* Free memory used by owner-drawn and help-echo strings.  */
	      w32_free_menu_strings (FRAME_W32_WINDOW! (f));       // C 459
	      kbd_buffer_store_event (&raw mut buf as *mut c_void);   // C 460

	      (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 0;                       // C 462
	      return;
	    }
	  i += MENU_ITEMS_ITEM_LENGTH;                             // C 465
	}
    }
  /* Free memory used by owner-drawn and help-echo strings.  */
  w32_free_menu_strings (FRAME_W32_WINDOW! (f));               // C 469
  (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 0;                               // C 470
}

/* C 474-475: "Set the contents of the menubar widgets of frame F." - C's banner comment:

     /* Set the contents of the menubar widgets of frame F.  */

   C 476-712: `set_frame_menubar`.  C's four `alloca`s (C 503-505, 546-549) are the port's stack
   pool (PORTING.md P-05, the ledger's note row); `f->menu_bar_items_used` and
   `->menu_bar_vector` are `struct frame`'s own members, read through the mirror's record.  */
/// C: void set_frame_menubar (struct frame *f, bool deep_p)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_frame_menubar (f: *mut frame, mut deep_p: bool)
{
  let mut menubar_widget: HMENU = (*( (*f).output_data.w32 as *mut w32_output )).menubar_widget;    // C 479
  let mut items: Lisp_Object;                                   // C 480
  let mut wv: *mut widget_value;                                // C 481
  let mut first_wv: *mut widget_value;                          // C 481
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();   // C 481
  let mut i: c_int;                                             // C 482
  let mut last_i: c_int = 0;                                    // C 482
  let submenu_start: *mut c_int;                                // C 483
  let submenu_end: *mut c_int;                                  // C 483
  let submenu_top_level_items: *mut c_int;                       // C 484
  let submenu_n_panes: *mut c_int;                               // C 484

  /* We must not change the menubar when actually in use.  */
  if (*( (*f).output_data.w32 as *mut w32_output )).menubar_active != 0                            // C 487
    { return; }

  XSETFRAME! (Vmenu_updating_frame, f);                        // C 490

  if menubar_widget.is_null ()                                 // C 492
    { deep_p = true; }

  if deep_p                                                    // C 495
    {
      /* Make a widget-value tree representing the entire menu trees.  */

      let prev: *mut buffer = current_buffer;                  // C 499
      let mut buffer: Lisp_Object;                             // C 500
      let specpdl_count: specpdl_ref = SPECPDL_INDEX ();        // C 501
      let mut previous_menu_items_used: c_int = (*f).menu_bar_items_used;   // C 502
      let previous_items: *mut Lisp_Object
	= sa_stack_alloc ((previous_menu_items_used as isize * word_size) as usize)
	  as *mut Lisp_Object;                                 // C 503-505

      /* If we are making a new widget, its contents are empty,
	 do always reinitialize them.  */
      if menubar_widget.is_null ()                             // C 509
	{ previous_menu_items_used = 0; }

      buffer = (*XWINDOW (FRAME_SELECTED_WINDOW (f))).contents;   // C 512
      specbind (Qinhibit_quit (), Qt ());                      // C 513
      /* Don't let the debugger step into this code
	 because it is not reentrant.  */
      specbind (Qdebug_on_next_call (), Qnil ());              // C 516

      record_unwind_save_match_data ();                        // C 518

      if NILP (Voverriding_local_map_menu_flag)                // C 520
	{
	  specbind (Qoverriding_terminal_local_map (), Qnil ());   // C 522
	  specbind (Qoverriding_local_map (), Qnil ());            // C 523
	}

      set_buffer_internal_1 (XBUFFER (buffer));                // C 526

      /* Run the hooks.  */
      safe_run_hooks (Qactivate_menubar_hook ());              // C 529
      safe_run_hooks (Qmenu_bar_update_hook ());               // C 530
      fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 531

      items = FRAME_MENU_BAR_ITEMS (f);                        // C 533

      /* Save the frame's previous menu bar contents data.  */
      if previous_menu_items_used != 0                         // C 536
	{ memcpy (previous_items as *mut c_void,                // C 537-538
		  (*XVECTOR ((*f).menu_bar_vector)).contents.as_ptr () as *const c_void,
		  (previous_menu_items_used as isize * word_size) as usize); }

      /* Fill in menu_items with the current menu bar contents.
	 This can evaluate Lisp code.  */
      save_menu_items ();                                      // C 542

      menu_items = (*f).menu_bar_vector;                       // C 544
      menu_items_allocated = if VECTORP (menu_items)           // C 545
	{ ASIZE (menu_items) as c_int } else { 0 };
      submenu_start = sa_stack_alloc ((ASIZE (items) * core::mem::size_of::<c_int> () as isize)
				      as usize) as *mut c_int;   // C 546
      submenu_end = sa_stack_alloc ((ASIZE (items) * core::mem::size_of::<c_int> () as isize)
				    as usize) as *mut c_int;     // C 547
      submenu_n_panes = sa_stack_alloc ((ASIZE (items) * core::mem::size_of::<c_int> () as isize)
					as usize) as *mut c_int; // C 548
      submenu_top_level_items
	= sa_stack_alloc ((ASIZE (items) * core::mem::size_of::<c_int> () as isize)
			  as usize) as *mut c_int;               // C 549
      init_menu_items ();                                      // C 550
      i = 0;                                                   // C 551
      while i < ASIZE (items) as c_int
	{
	  let mut key: Lisp_Object;                              // C 553
	  let string: Lisp_Object;                               // C 553
	  let maps: Lisp_Object;                                 // C 553

	  last_i = i;                                            // C 555

	  key = AREF (items, i as isize);                         // C 557
	  string = AREF (items, i as isize + 1);                  // C 558
	  maps = AREF (items, i as isize + 2);                    // C 559
	  if NILP (string)                                        // C 560
	    { break; }

	  *submenu_start.offset (i as isize) = menu_items_used;    // C 563

	  menu_items_n_panes = 0;                                 // C 565
	  *submenu_top_level_items.offset (i as isize)
	    = parse_single_submenu (key, string, maps) as c_int;   // C 566-567
	  *submenu_n_panes.offset (i as isize) = menu_items_n_panes;   // C 568

	  *submenu_end.offset (i as isize) = menu_items_used;      // C 570
	  i += 4;                                                 // C 551's `i += 4`
	}

      finish_menu_items ();                                    // C 573

      /* Convert menu_items into widget_value trees
	 to display the menu.  This cannot evaluate Lisp code.  */

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());  // C 578
      (*wv).button_type = BUTTON_TYPE_NONE;                    // C 579
      first_wv = wv;                                           // C 580

      i = 0;                                                   // C 582
      while i < last_i
	{
	  menu_items_n_panes = *submenu_n_panes.offset (i as isize);   // C 584
	  wv = digest_single_submenu (*submenu_start.offset (i as isize),   // C 585-586
				      *submenu_end.offset (i as isize),
				      *submenu_top_level_items.offset (i as isize) != 0);
	  if !prev_wv.is_null ()                                   // C 587
	    { (*prev_wv).next = wv; }                              // C 588
	  else
	    { (*first_wv).contents = wv; }                         // C 590
	  /* Don't set wv->name here; GC during the loop might relocate it.  */
	  (*wv).enabled = true;                                    // C 592
	  (*wv).button_type = BUTTON_TYPE_NONE;                    // C 593
	  prev_wv = wv;                                            // C 594
	  i += 4;                                                  // C 582's `i += 4`
	}

      set_buffer_internal_1 (prev);                            // C 597

      /* If there has been no change in the Lisp-level contents
	 of the menu bar, skip redisplaying it.  Just exit.  */

      i = 0;                                                   // C 602
      while i < previous_menu_items_used
	{
	  if menu_items_used == i
	    || !EQ (*previous_items.offset (i as isize),
		    AREF (menu_items, i as isize))
	    { break; }
	  i += 1;
	}
      if i == menu_items_used && i == previous_menu_items_used && i != 0   // C 606
	{
	  free_menubar_widget_value_tree (first_wv);              // C 608
	  discard_menu_items ();                                  // C 609
	  unbind_to (specpdl_count, Qnil ());                      // C 610
	  return;
	}

      fset_menu_bar_vector (f, menu_items);                      // C 614
      (*f).menu_bar_items_used = menu_items_used;                // C 615

      /* This undoes save_menu_items.  */
      unbind_to (specpdl_count, Qnil ());                        // C 618

      /* Now GC cannot happen during the lifetime of the widget_value,
	 so it's safe to store data from a Lisp_String, as long as
	 local copies are made when the actual menu is created.
	 Windows takes care of this for normal string items, but
	 not for owner-drawn items or additional item-info.  */
      wv = (*first_wv).contents;                                  // C 625
      i = 0;                                                      // C 626
      while i < ASIZE (items) as c_int
	{
	  let string: Lisp_Object = AREF (items, i as isize + 1);   // C 628-629
	  if NILP (string)                                          // C 630
	    { break; }
	  (*wv).name = SSDATA (string);                             // C 632
	  update_submenu_strings ((*wv).contents);                  // C 633
	  wv = (*wv).next;                                          // C 634
	  i += 4;                                                   // C 626's `i += 4`
	}
    }
  else
    {
      /* Make a widget-value tree containing
	 just the top level menu bar strings.  */

      wv = make_widget_value (c"menubar".as_ptr (), std::ptr::null_mut (), true, Qnil ());  // C 642
      (*wv).button_type = BUTTON_TYPE_NONE;                     // C 643
      first_wv = wv;                                            // C 644

      items = FRAME_MENU_BAR_ITEMS (f);                         // C 646
      i = 0;                                                    // C 647
      while i < ASIZE (items) as c_int
	{
	  let string: Lisp_Object = AREF (items, i as isize + 1);   // C 649-651
	  if NILP (string)                                        // C 652
	    { break; }

	  wv = make_widget_value (SSDATA (string), std::ptr::null_mut (), true, Qnil ());  // C 655
	  (*wv).button_type = BUTTON_TYPE_NONE;                   // C 656
	  /* This prevents lwlib from assuming this
	     menu item is really supposed to be empty.  */
	  /* The EMACS_INT cast avoids a warning.
	     This value just has to be different from small integers.  */
	  (*wv).call_data = (-1 as EMACS_INT) as *mut c_void;      // C 661

	  if !prev_wv.is_null ()                                   // C 663
	    { (*prev_wv).next = wv; }                              // C 664
	  else
	    { (*first_wv).contents = wv; }                         // C 666
	  prev_wv = wv;                                            // C 667
	  i += 4;                                                  // C 647's `i += 4`
	}

      /* Forget what we thought we knew about what is in the
	 detailed contents of the menu bar menus.
	 Changing the top level always destroys the contents.  */
      (*f).menu_bar_items_used = 0;                              // C 673
    }

  /* Create or update the menu bar widget.  */

  block_input ();                                                // C 678

  if !menubar_widget.is_null ()                                  // C 680
    {
      /* Empty current menubar, rather than creating a fresh one.  */
      while DeleteMenu (menubar_widget, 0, MF_BYPOSITION) != 0    // C 683
	{
	  /* C 684 is C's own empty statement (`;`): the call in the condition is the whole
	     body, and the loop re-tests it until the menubar is empty.  */
	}
    }
  else
    {
      menubar_widget = CreateMenu ();                             // C 688
    }
  fill_in_menu (menubar_widget, (*first_wv).contents);            // C 690

  free_menubar_widget_value_tree (first_wv);                      // C 692

  {
    let old_widget: HMENU = (*( (*f).output_data.w32 as *mut w32_output )).menubar_widget;             // C 695

    (*( (*f).output_data.w32 as *mut w32_output )).menubar_widget = menubar_widget;                    // C 697
    SetMenu (FRAME_W32_WINDOW! (f), (*( (*f).output_data.w32 as *mut w32_output )).menubar_widget);    // C 698
    /* Causes flicker when menu bar is updated
    DrawMenuBar (FRAME_W32_WINDOW (f)); */

    /* Force the window size to be recomputed so that the frame's text
       area remains the same, if menubar has just been created.  */
    if old_widget.is_null ()                                      // C 704
      {
	windows_or_buffers_changed = 23;                           // C 706
	adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ());   // C 707
      }
  }

  unblock_input ();                                               // C 711
}

/// C 714-717's comment ("Called from Fx_create_frame to create the initial menubar of a frame
/// before it is mapped, …") is carried in C's own words where it stands:

   /* Called from Fx_create_frame to create the initial menubar of a frame
      before it is mapped, so that the window is mapped with the menubar already
      there instead of us tacking it on later and thrashing the window after it
      is visible.  */

/// C: void initialize_frame_menubar (struct frame *f)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_frame_menubar (f: *mut frame)   // C 719-726
{
  /* This function is called before the first chance to redisplay
     the frame.  It has to be, so the frame will have the right size.  */
  fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));   // C 724
  set_frame_menubar (f, true);                                      // C 725
}

/// C 728-729's comment ("Get rid of the menu bar of frame F, and free its storage. …"):

   /* Get rid of the menu bar of frame F, and free its storage.
      This is used when deleting a frame, and when turning off the menu bar.  */

/// C: void free_frame_menubar (struct frame *f)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_frame_menubar (f: *mut frame)       // C 731-744
{
  block_input ();                                                   // C 734

  {
    let old: HMENU = GetMenu (FRAME_W32_WINDOW! (f));                // C 737
    SetMenu (FRAME_W32_WINDOW! (f), std::ptr::null_mut ());          // C 738
    (*( (*f).output_data.w32 as *mut w32_output )).menubar_widget = std::ptr::null_mut ();               // C 739
    DestroyMenu (old);                                              // C 740
  }

  unblock_input ();                                                 // C 743
}

/* C 746-749 and C 751-761: the two banner comments of `w32_menu_show` ("w32_menu_show actually
   displays a menu using the panes and items in menu_items and returns the value selected from it;
   we assume input is blocked by the caller." and the parameter list).  Carried in C's own words
   where they stand:

     /* w32_menu_show actually displays a menu using the panes and items in
        menu_items and returns the value selected from it; we assume input
        is blocked by the caller.  */

     /* F is the frame the menu is for.
        X and Y are the frame-relative specified position,
        relative to the inside upper left corner of the frame F.
        Bitfield MENUFLAGS bits are:
        MENU_FOR_CLICK is set if this menu was invoked for a mouse click.
        MENU_KEYMAPS is set if this menu was specified with keymaps;
         in that case, we return a list containing the chosen item's value
         and perhaps also the pane's prefix.
        TITLE is the specified menu title.
        ERROR is a place to store an error message string in case of failure.
        (We return nil on failure, but the value doesn't actually matter.)  */
*/

/* C 763-1052: `w32_menu_show`.  C's `USE_SAFE_ALLOCA`/`SAFE_ALLOCA`/`SAFE_FREE` pair is the
   mirror's macros (PORTING.md P-05: Rust's hygiene needs the two names spelled at the call site);
   `pos` is zero-initialised before C 965's two stores (PORTING.md 1.5).  */
/// C: Lisp_Object w32_menu_show (struct frame *f, int x, int y, int menuflags,
/// C:   Lisp_Object title, const char **error)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_menu_show (f: *mut frame, x: c_int, y: c_int, menuflags: c_int,
                                        title: Lisp_Object, error: *mut *const c_char)
                                        -> Lisp_Object
{
  let mut i: c_int;                                             // C 767
  let mut menu_item_selection: c_int;                            // C 768
  let mut menu: HMENU;                                           // C 769
  let mut pos: POINT = core::mem::zeroed ();                     // C 770
  let mut wv: *mut widget_value;                                 // C 771
  let mut save_wv: *mut widget_value = std::ptr::null_mut ();    // C 771
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();   // C 771
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();    // C 771
  let submenu_stack: *mut *mut widget_value;                     // C 772
  let subprefix_stack: *mut Lisp_Object;                         // C 773
  let mut submenu_depth: c_int = 0;                              // C 774
  let mut first_pane: bool;                                      // C 775

  *error = std::ptr::null ();                                    // C 777

  if menu_items_n_panes == 0                                      // C 779
    { return Qnil (); }

  if menu_items_used <= MENU_ITEMS_PANE_LENGTH                    // C 782
    {
      *error = c"Empty menu".as_ptr ();                           // C 784
      return Qnil ();
    }

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                          // C 788

  submenu_stack = SAFE_ALLOCA! (sa_avail, menu_items_used as isize
				* core::mem::size_of::<*mut widget_value> () as isize)
		  as *mut *mut widget_value;                      // C 790
  subprefix_stack = SAFE_ALLOCA! (sa_avail, menu_items_used as isize * word_size)
		    as *mut Lisp_Object;                          // C 791

  block_input ();                                                 // C 793

  /* Create a tree of widget_value objects
     representing the panes and their items.  */
  wv = make_widget_value (c"menu".as_ptr (), std::ptr::null_mut (), true, Qnil ());  // C 797
  (*wv).button_type = BUTTON_TYPE_NONE;                           // C 798
  first_wv = wv;                                                  // C 799
  first_pane = true;                                              // C 800

  /* Loop over all panes and items, filling in the tree.  */
  i = 0;                                                          // C 803
  while i < menu_items_used                                       // C 804
    {
      if NILP (AREF (menu_items, i as isize))                     // C 806
	{
	  *submenu_stack.offset (submenu_depth as isize) = save_wv;   // C 808
	  submenu_depth += 1;
	  save_wv = prev_wv;                                      // C 809
	  prev_wv = std::ptr::null_mut ();                        // C 810
	  first_pane = false;                                     // C 811
	  i += 1;                                                 // C 812
	}
      else if EQ (AREF (menu_items, i as isize), Qlambda ())      // C 814
	{
	  prev_wv = save_wv;                                      // C 816
	  submenu_depth -= 1;
	  save_wv = *submenu_stack.offset (submenu_depth as isize);   // C 817
	  first_pane = false;                                     // C 818
	  i += 1;                                                 // C 819
	}
      else if EQ (AREF (menu_items, i as isize), Qt ())           // C 821
	       && submenu_depth != 0                              // C 822
	{ i += MENU_ITEMS_PANE_LENGTH; }                          // C 823
      /* Ignore a nil in the item list.
	 It's meaningful only for dialog boxes.  */
      else if EQ (AREF (menu_items, i as isize), Qquote ())       // C 826
	{ i += 1; }                                               // C 827
      else if EQ (AREF (menu_items, i as isize), Qt ())           // C 828
	{
	  /* Create a new pane.  */
	  let mut pane_name: Lisp_Object;                         // C 831
	  let prefix: Lisp_Object;                                // C 831
	  let mut pane_string: *const c_char;                     // C 832
	  pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);   // C 833
	  prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);    // C 834

	  if STRINGP (pane_name)                                  // C 836
	    {
	      if unicode_append_menu.is_some ()                   // C 838
		{ pane_name = ENCODE_UTF_8 (pane_name); }           // C 839
	      else if STRING_MULTIBYTE (pane_name)                // C 840
		{ pane_name = ENCODE_SYSTEM (pane_name); }          // C 841

	      ASET (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize, pane_name);   // C 843
	    }

	  pane_string = if NILP (pane_name)                       // C 846-847
	    { c"".as_ptr () } else { SSDATA (pane_name) };
	  /* If there is just one top-level pane, put all its items directly
	     under the top-level menu.  */
	  if menu_items_n_panes == 1                              // C 850
	    { pane_string = c"".as_ptr (); }                      // C 851

	  /* If the pane has a meaningful name,
	     make the pane a top-level menu item
	     with its items as a submenu beneath it.  */
	  if (menuflags & MENU_KEYMAPS) == 0 && strcmp (pane_string, c"".as_ptr ()) != 0   // C 856
	    {
	      wv = make_widget_value (pane_string, std::ptr::null_mut (), true, Qnil ());  // C 858
	      if !save_wv.is_null ()                              // C 859
		{ (*save_wv).next = wv; }                         // C 860
	      else
		{ (*first_wv).contents = wv; }                    // C 862
	      if (menuflags & MENU_KEYMAPS) != 0 && !NILP (prefix)   // C 863
		{ (*wv).name = (*wv).name.offset (1); }           // C 864
	      (*wv).button_type = BUTTON_TYPE_NONE;               // C 865
	      save_wv = wv;                                       // C 866
	      prev_wv = std::ptr::null_mut ();                    // C 867
	    }
	  else if first_pane                                      // C 869
	    {
	      save_wv = wv;                                       // C 871
	      prev_wv = std::ptr::null_mut ();                    // C 872
	    }
	  first_pane = false;                                     // C 874
	  i += MENU_ITEMS_PANE_LENGTH;                            // C 875
	}
      else
	{
	  /* Create a new item within current pane.  */
	  let mut item_name: Lisp_Object;                         // C 880
	  let enable: Lisp_Object;                                // C 880
	  let mut descrip: Lisp_Object;                           // C 880
	  let def: Lisp_Object;                                   // C 880
	  let type_: Lisp_Object;                                 // C 880
	  let selected: Lisp_Object;                              // C 880
	  let help: Lisp_Object;                                  // C 880

	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 882
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 883
	  descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 884
	  def = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_DEFINITION as isize);   // C 885
	  type_ = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_TYPE as isize);       // C 886
	  selected = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_SELECTED as isize);   // C 887
          help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);        // C 888

          if STRINGP (item_name)                                  // C 890
	    {
	      if unicode_append_menu.is_some ()                   // C 892
		{ item_name = ENCODE_UTF_8 (item_name); }           // C 893
	      else if STRING_MULTIBYTE (item_name)                // C 894
		{ item_name = ENCODE_SYSTEM (item_name); }          // C 895

	      ASET (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize, item_name);   // C 897
	    }

	  if STRINGP (descrip) && STRING_MULTIBYTE (descrip)       // C 900
            {
	      descrip = ENCODE_SYSTEM (descrip);                   // C 902
	      ASET (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize, descrip);   // C 903
	    }

	  wv = make_widget_value (SSDATA (item_name), std::ptr::null_mut (), !NILP (enable),   // C 906
				  if STRINGP (help) { help } else { Qnil () });   // C 907
	  if !prev_wv.is_null ()                                   // C 908
	    { (*prev_wv).next = wv; }                              // C 909
	  else
	    { (*save_wv).contents = wv; }                          // C 911
	  if !NILP (descrip)                                       // C 912
	    { (*wv).key = SSDATA (descrip); }                      // C 913
	  /* Use the contents index as call_data, since we are
             restricted to 16-bits.  */
	  (*wv).call_data = if !NILP (def)                          // C 916
	    { i as UINT_PTR as *mut c_void } else { std::ptr::null_mut () };

	  if NILP (type_)                                          // C 918
	    { (*wv).button_type = BUTTON_TYPE_NONE; }              // C 919
	  else if EQ (type_, QCtoggle ())                          // C 920
	    { (*wv).button_type = BUTTON_TYPE_TOGGLE; }            // C 921
	  else if EQ (type_, QCradio ())                           // C 922
	    { (*wv).button_type = BUTTON_TYPE_RADIO; }             // C 923
	  else
	    { emacs_abort (); }                                    // C 925

	  (*wv).selected = !NILP (selected);                       // C 927

	  prev_wv = wv;                                            // C 929

	  i += MENU_ITEMS_ITEM_LENGTH;                             // C 931
	}
    }

  /* Deal with the title, if it is non-nil.  */
  if !NILP (title)                                             // C 936
    {
      let wv_title: *mut widget_value;                          // C 938
      let wv_sep: *mut widget_value
	= make_widget_value (c"--".as_ptr (), std::ptr::null_mut (), false, Qnil ());  // C 939

      /* Maybe replace this separator with a bitmap or owner-draw item
	 so that it looks better.  Having two separators looks odd.  */
      (*wv_sep).next = (*first_wv).contents;                    // C 943

      let mut title = title;                                    // C 945 rebinds the parameter
      if unicode_append_menu.is_some ()                         // C 945
	{ title = ENCODE_UTF_8 (title); }                       // C 946
      else if STRING_MULTIBYTE (title)                          // C 947
	{ title = ENCODE_SYSTEM (title); }                      // C 948

      wv_title = make_widget_value (SSDATA (title), std::ptr::null_mut (), true, Qnil ());  // C 950
      (*wv_title).title = TRUE != 0;                            // C 951
      (*wv_title).button_type = BUTTON_TYPE_NONE;               // C 952
      (*wv_title).next = wv_sep;                                // C 953
      (*first_wv).contents = wv_title;                          // C 954
    }

  /* No selection has been chosen yet.  */
  menu_item_selection = 0;                                      // C 958

  /* Actually create the menu.  */
  menu = CreatePopupMenu ();                                    // C 961
  current_popup_menu = menu;
  fill_in_menu (menu, (*first_wv).contents);                    // C 962

  /* Adjust coordinates to be root-window-relative.  */
  pos.x = x;                                                    // C 965
  pos.y = y;                                                    // C 966
  ClientToScreen (FRAME_W32_WINDOW! (f), &raw mut pos);          // C 967

  /* Display the menu.  */
  menu_item_selection = SendMessage (FRAME_W32_WINDOW! (f),      // C 970-972
				     WM_EMACS_TRACKPOPUPMENU as UINT,
				     menu as WPARAM, (&raw mut pos) as LPARAM) as c_int;

  /* Clean up extraneous mouse events which might have been generated
     during the call. */
  discard_mouse_events ();                                      // C 976
  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;                        // C 977

  /* Free the widget_value objects we used to specify the contents.  */
  free_menubar_widget_value_tree (first_wv);                     // C 980

  DestroyMenu (menu);                                            // C 982

  /* Free the owner-drawn and help-echo menu strings.  */
  w32_free_menu_strings (FRAME_W32_WINDOW! (f));                 // C 985
  (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 0;                      // C 986

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if menu_item_selection != 0                                    // C 990
    {
      let mut prefix: Lisp_Object;                               // C 992
      let mut entry: Lisp_Object;                                // C 992

      prefix = Qnil ();                                          // C 994
      entry = prefix;
      i = 0;                                                     // C 995
      while i < menu_items_used                                 // C 996
	{
	  if NILP (AREF (menu_items, i as isize))                 // C 998
	    {
	      *subprefix_stack.offset (submenu_depth as isize) = prefix;   // C 1000
	      submenu_depth += 1;
	      prefix = entry;                                     // C 1001
	      i += 1;                                             // C 1002
	    }
	  else if EQ (AREF (menu_items, i as isize), Qlambda ())   // C 1004
	    {
	      submenu_depth -= 1;
	      prefix = *subprefix_stack.offset (submenu_depth as isize);   // C 1006
	      i += 1;                                             // C 1007
	    }
	  else if EQ (AREF (menu_items, i as isize), Qt ())        // C 1009
	    {
	      prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);   // C 1011
	      i += MENU_ITEMS_PANE_LENGTH;                        // C 1012
	    }
	  /* Ignore a nil in the item list.
	     It's meaningful only for dialog boxes.  */
	  else if EQ (AREF (menu_items, i as isize), Qquote ())    // C 1016
	    { i += 1; }                                           // C 1017
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 1020
	      if menu_item_selection == i                         // C 1021
		{
		  if (menuflags & MENU_KEYMAPS) != 0              // C 1023
		    {
		      let mut j: c_int;                            // C 1025

		      entry = Fcons (entry, Qnil ());              // C 1027
		      if !NILP (prefix)                            // C 1028
			{ entry = Fcons (prefix, entry); }         // C 1029
		      j = submenu_depth - 1;                       // C 1030
		      while j >= 0
			{
			  if !NILP (*subprefix_stack.offset (j as isize))   // C 1031
			    { entry = Fcons (*subprefix_stack.offset (j as isize), entry); }   // C 1032
			  j -= 1;
			}
		    }
		  unblock_input ();                                // C 1034
		  SAFE_FREE! (sa_count, sa_mark);                           // C 1035
		  return entry;                                    // C 1036
		}
	      i += MENU_ITEMS_ITEM_LENGTH;                         // C 1038
	    }
	}
    }
  else if (menuflags & MENU_FOR_CLICK) == 0                    // C 1042
    {
      unblock_input ();                                        // C 1044
      /* Make "Cancel" equivalent to C-g.  */
      quit ();                                                 // C 1046
    }

  unblock_input ();                                            // C 1049
  SAFE_FREE! (sa_count, sa_mark);                                       // C 1050
  return Qnil ();                                              // C 1051
}

/* C 1055-1077: `#ifdef HAVE_DIALOGS` and the "TODO: On Windows, there are two ways of defining a
   dialog." design comment.  C's text is carried in C's own words where it stands:

     #ifdef HAVE_DIALOGS
     /* TODO: On Windows, there are two ways of defining a dialog.

        1. Create a predefined dialog resource and include it in nt/emacs.rc.
           Using this method, we could then set the titles and make unneeded
           buttons invisible before displaying the dialog.  Everything would
           be a fixed size though, so there is a risk that text does not
           fit on a button.
        2. Create the dialog template in memory on the fly.  This allows us
           to size the dialog and buttons dynamically, probably giving more
           natural looking results for dialogs with few buttons, and eliminating
           the problem of text overflowing the buttons.  But the API for this is
           quite complex - structures have to be allocated in particular ways,
           text content is tacked onto the end of structures in variable length
           arrays with further structures tacked on after these, there are
           certain alignment requirements for all this, and we have to
           measure all the text and convert to "dialog coordinates" to figure
           out how big to make everything.

           For now, we'll just stick with menus for dialogs that are more
           complicated than simple yes/no type questions for which we can use
           the MessageBox function.
     */

   **This arm cannot be built in C either**: C 58 `#undef`s `HAVE_DIALOGS` ("TODO: Implement native
   dialogs"), so no configuration reaches the text, and the names it writes (`dialog_id`,
   `widget_id_tick`, `dialog_selection_callback`, `popup_get_selection`, `menu`, and
   `f->output_data.w32->widget`) are declared nowhere C reaches.  It is carried as C's own text
   with those names declared in this unit (see the declarations above), and the ledger's
   `## Not claimed` says plainly that nothing here was compiled or executed.  */
/// C: static char * button_names [] = { "button1", "button2", "button3", "button4", "button5",
/// C:   "button6", "button7", "button8", "button9", "button10" };
#[cfg (all (feature = "have_ntgui", feature = "have_dialogs"))]
static mut button_names: [*mut c_char; 10] = [
  c"button1".as_ptr () as *mut c_char, c"button2".as_ptr () as *mut c_char,
  c"button3".as_ptr () as *mut c_char, c"button4".as_ptr () as *mut c_char,
  c"button5".as_ptr () as *mut c_char, c"button6".as_ptr () as *mut c_char,
  c"button7".as_ptr () as *mut c_char, c"button8".as_ptr () as *mut c_char,
  c"button9".as_ptr () as *mut c_char, c"button10".as_ptr () as *mut c_char,
];

/// C: static Lisp_Object w32_dialog_show (struct frame *f, Lisp_Object title,
/// C:   Lisp_Object header, char **error)
#[cfg (all (feature = "have_ntgui", feature = "have_dialogs"))]
unsafe fn w32_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
                           error: *mut *mut c_char) -> Lisp_Object   // C 1083-1242
{
  let mut i: c_int;                                              // C 1087
  let mut nb_buttons: c_int = 0;                                 // C 1087
  let mut dialog_name: [c_char; 6] = core::mem::zeroed ();        // C 1088
  let mut menu_item_selection: c_int;                             // C 1089

  let mut wv: *mut widget_value;                                  // C 1091
  let mut first_wv: *mut widget_value = std::ptr::null_mut ();    // C 1091
  let mut prev_wv: *mut widget_value = std::ptr::null_mut ();     // C 1091

  /* Number of elements seen so far, before boundary.  */
  let mut left_count: c_int = 0;                                  // C 1094
  /* true means we've seen the boundary between left-hand elts and
     right-hand.  */
  let mut boundary_seen: bool = false;                            // C 1097

  *error = std::ptr::null_mut ();                                 // C 1099

  if menu_items_n_panes > 1                                       // C 1101
    {
      *error = c"Multiple panes in dialog box".as_ptr () as *mut c_char;   // C 1103
      return Qnil ();
    }

  /* Create a tree of widget_value objects
     representing the text label and buttons.  */
  {
    let pane_name: Lisp_Object;                                   // C 1110
    let pane_string: *const c_char;                               // C 1111
    pane_name = AREF (menu_items, MENU_ITEMS_PANE_NAME as isize);  // C 1112
    pane_string = if NILP (pane_name)                              // C 1113-1114
      { c"".as_ptr () } else { SSDATA (pane_name) };
    prev_wv = make_widget_value (c"message".as_ptr (), pane_string as *mut c_char,
				 true, Qnil ());                  // C 1115
    first_wv = prev_wv;                                           // C 1116

    /* Loop over all panes and items, filling in the tree.  */
    i = MENU_ITEMS_PANE_LENGTH;                                   // C 1119
    while i < menu_items_used                                     // C 1120
      {

	/* Create a new item within current pane.  */
	let item_name: Lisp_Object;                                  // C 1124
	let enable: Lisp_Object;                                     // C 1124
	let descrip: Lisp_Object;                                    // C 1124
	let help: Lisp_Object;                                       // C 1124

	item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);   // C 1126
	enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);    // C 1127
	descrip = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize);   // C 1128
        help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);        // C 1129

	if NILP (item_name)                                          // C 1131
	  {
	    free_menubar_widget_value_tree (first_wv);                // C 1133
	    *error = c"Submenu in dialog items".as_ptr () as *mut c_char;   // C 1134
	    return Qnil ();
	  }
	if EQ (item_name, Qquote ())                                  // C 1137
	  {
	    /* This is the boundary between left-side elts
	       and right-side elts.  Stop incrementing right_count.  */
	    boundary_seen = true;                                     // C 1141
	    i += 1;                                                   // C 1142
	    continue;
	  }
	if nb_buttons >= 9                                            // C 1145
	  {
	    free_menubar_widget_value_tree (first_wv);                // C 1147
	    *error = c"Too many dialog items".as_ptr () as *mut c_char;   // C 1148
	    return Qnil ();
	  }

	wv = make_widget_value (button_names[nb_buttons as usize],   // C 1152
				SSDATA (item_name),                  // C 1153
				!NILP (enable), Qnil ());            // C 1154
	(*prev_wv).next = wv;                                         // C 1155
	if !NILP (descrip)                                            // C 1156
	  { (*wv).key = SSDATA (descrip); }                           // C 1157
	(*wv).call_data = aref_addr (menu_items, i as isize) as *mut c_void;   // C 1158
	prev_wv = wv;                                                 // C 1159

	if !boundary_seen                                             // C 1161
	  { left_count += 1; }                                        // C 1162

	nb_buttons += 1;                                              // C 1164
	i += MENU_ITEMS_ITEM_LENGTH;                                  // C 1165
      }

    /* If the boundary was not specified,
       by default put half on the left and half on the right.  */
    if !boundary_seen                                             // C 1170
      { left_count = nb_buttons - nb_buttons / 2; }               // C 1171

    wv = make_widget_value (dialog_name.as_ptr (), std::ptr::null_mut (),
			    false, Qnil ());                      // C 1173

    /*  Frame title: 'Q' = Question, 'I' = Information.
        Can also have 'E' = Error if, one day, we want
        a popup for errors. */
    if NILP (header)                                              // C 1178
      { dialog_name[0] = b'Q' as c_char; }                        // C 1179
    else
      { dialog_name[0] = b'I' as c_char; }                        // C 1181

    /* Dialog boxes use a really stupid name encoding
       which specifies how many buttons to use
       and how many buttons are on the right. */
    dialog_name[1] = b'0' as c_char + nb_buttons as c_char;        // C 1186
    dialog_name[2] = b'B' as c_char;                              // C 1187
    dialog_name[3] = b'R' as c_char;                              // C 1188
    /* Number of buttons to put on the right.  */
    dialog_name[4] = b'0' as c_char + (nb_buttons - left_count) as c_char;   // C 1190
    dialog_name[5] = 0;                                           // C 1191
    (*wv).contents = first_wv;                                    // C 1192
    first_wv = wv;                                                // C 1193
  }

  /* Actually create the dialog.  */
  dialog_id = widget_id_tick;                                     // C 1197
  widget_id_tick += 1;
  /* C 1199's `f->output_data.w32->widget` is **not a member of `struct w32_output`**
     (`src/w32term.h` 309-456 declares `menubar_widget` and no `widget`), so C's own text here
     names something its headers do not define - one of the reasons this arm cannot be built in C
     (the ledger's `## Deviations` records it).  The port keeps C's path; nothing resolves it
     because `feature = "have_dialogs"` is never set.  */
  menu = lw_create_widget ((*first_wv).name, c"dialog".as_ptr (), dialog_id, first_wv,
			   (*( (*f).output_data.w32 as *mut w32_output )).widget,
			   true as c_int, std::ptr::null_mut (),
			   dialog_selection_callback, std::ptr::null_mut ());   // C 1198-1200
  lw_modify_all_widgets (dialog_id, (*first_wv).contents, TRUE);   // C 1201

  /* Free the widget_value objects we used to specify the contents.  */
  free_menubar_widget_value_tree (first_wv);                      // C 1204

  /* No selection has been chosen yet.  */
  menu_item_selection = 0;                                        // C 1207

  /* Display the menu.  */
  lw_pop_up_all_widgets (dialog_id);                              // C 1210

  /* Process events that apply to the menu.  */
  /* C 1213's first argument is `(XEvent *) 0` - an X11 type from `lwlib/lwlib.h` this
     translation unit cannot reach (a C-level defect of the arm, which C 58's `#undef` keeps dead);
     the null pointer the cast produces is what the port passes, and the mirror's row in
     docs/ports/macro-body-notes.tsv records the `XEvent` token.  */
  popup_get_selection (std::ptr::null_mut (), FRAME_DISPLAY_INFO! (f), dialog_id);   // C 1213

  lw_destroy_all_widgets (dialog_id);                             // C 1215

  /* Find the selected item, and its pane, to return
     the proper value.  */
  if menu_item_selection != 0                                     // C 1219
    {
      i = 0;                                                      // C 1221
      while i < menu_items_used                                  // C 1222
	{
	  let entry: Lisp_Object;                                  // C 1224

	  if EQ (AREF (menu_items, i as isize), Qt ())              // C 1226
	    { i += MENU_ITEMS_PANE_LENGTH; }                        // C 1227
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);   // C 1230
	      if menu_item_selection == i                            // C 1231
		{ return entry; }                                    // C 1232
	      i += MENU_ITEMS_ITEM_LENGTH;                           // C 1233
	    }
	}
    }
  else
    /* Make "Cancel" equivalent to C-g.  */
    { quit (); }                                                 // C 1239

  return Qnil ();                                                // C 1241
}

/* C 1243-1248: `#else /* !HAVE_DIALOGS */` and its comment.  This is the *live* arm on every
   configuration this file reaches (C 58's `#undef`), so `is_simple_dialog` and
   `simple_dialog_show` are the two functions the frozen liveness map keeps.  C's comment:

     #else /* !HAVE_DIALOGS  */

     /* Currently we only handle Yes No dialogs (y-or-n-p and yes-or-no-p) as
        simple dialogs.  We could handle a few more, but I'm not aware of
        anywhere in Emacs that uses the other specific dialog choices that
        MessageBox provides.  */
*/

/// C: static bool is_simple_dialog (Lisp_Object contents)
#[cfg (all (feature = "have_ntgui", not (feature = "have_dialogs")))]
unsafe fn is_simple_dialog (contents: Lisp_Object) -> bool     // C 1250-1295
{
  let mut options: Lisp_Object;                                 // C 1253
  let mut name: Lisp_Object;                                    // C 1254
  let mut yes: Lisp_Object;                                     // C 1254
  let mut no: Lisp_Object;                                      // C 1254
  let mut other: Lisp_Object;                                   // C 1254

  if !CONSP (contents)                                          // C 1256
    { return false; }
  options = XCDR (contents);                                    // C 1258

  yes = build_string (c"Yes".as_ptr ());                        // C 1260
  no = build_string (c"No".as_ptr ());                          // C 1261

  if !CONSP (options)                                           // C 1263
    { return false; }

  name = XCAR (options);                                        // C 1266
  if !CONSP (name)                                              // C 1267
    { return false; }
  name = XCAR (name);                                           // C 1269

  if !NILP (Fstring_equal (name, yes))                          // C 1271
    { other = no; }                                             // C 1272
  else if !NILP (Fstring_equal (name, no))                      // C 1273
    { other = yes; }                                            // C 1274
  else
    { return false; }                                           // C 1276

  options = XCDR (options);                                     // C 1278
  if !CONSP (options)                                           // C 1279
    { return false; }

  name = XCAR (options);                                        // C 1282
  if !CONSP (name)                                              // C 1283
    { return false; }
  name = XCAR (name);                                           // C 1285
  if NILP (Fstring_equal (name, other))                         // C 1286
    { return false; }

  /* Check there are no more options.

     (FIXME: Since we use MB_YESNOCANCEL, we could also consider
     dialogs with 3 options: Yes/No/Cancel as "simple".  */
  options = XCDR (options);                                     // C 1293
  return !(CONSP (options));                                    // C 1294
}

/// C: static Lisp_Object simple_dialog_show (struct frame *f, Lisp_Object contents,
/// C:   Lisp_Object header)
#[cfg (all (feature = "have_ntgui", not (feature = "have_dialogs")))]
unsafe fn simple_dialog_show (f: *mut frame, contents: Lisp_Object,
                              header: Lisp_Object) -> Lisp_Object   // C 1297-1407
{
  let mut answer: c_int;                                        // C 1300
  let mut type_: UINT;                                          // C 1301
  let mut lispy_answer: Lisp_Object = Qnil ();                   // C 1302
  let mut temp: Lisp_Object = XCAR (contents);                   // C 1302

  /* We use MB_YESNOCANCEL to allow the user the equivalent of C-g
     when the Yes/No question is asked vya y-or-n-p or
     yes-or-no-p.  */
  if w32_yes_no_dialog_show_cancel                            // C 1307
    { type_ = MB_YESNOCANCEL; }                                 // C 1308
  else
    { type_ = MB_YESNO; }                                       // C 1310

  /* Since we only handle Yes/No dialogs, and we already checked
     is_simple_dialog, we don't need to worry about checking contents
     to see what type of dialog to use.  */

  /* Use Unicode if possible, so any language can be displayed.  */
  if unicode_message_box.is_some ()                             // C 1317
    {
      let mut text: *mut WCHAR;                                  // C 1319
      let title: *const WCHAR;                                   // C 1320
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                     // C 1321

      if STRINGP (temp)                                          // C 1323
	{
	  let utf8_text: *mut c_char = SSDATA (ENCODE_UTF_8 (temp));   // C 1325
	  /* Be pessimistic about the number of characters needed.
	     Remember characters outside the BMP will take more than
	     one utf16 word, so we cannot simply use the character
	     length of temp.  */
	  let utf8_len: c_int = strlen (utf8_text) as c_int;        // C 1330
	  text = SAFE_ALLOCA! (sa_avail, (utf8_len + 1) as isize
			       * core::mem::size_of::<WCHAR> () as isize)
		 as *mut WCHAR;                                    // C 1331
	  utf8to16 (utf8_text as *mut u8, utf8_len, text);          // C 1332
	}
      else
	{
	  text = WIDE_EMPTY.as_ptr () as *mut WCHAR;               // C 1336
	}

      if NILP (header)                                          // C 1339
	{
	  title = WIDE_Question.as_ptr ();                          // C 1341
	  type_ |= MB_ICONQUESTION;                                 // C 1342
	}
      else
	{
	  title = WIDE_Information.as_ptr ();                       // C 1346
	  type_ |= MB_ICONINFORMATION;                              // C 1347
	}

      answer = unicode_message_box.unwrap () (FRAME_W32_WINDOW! (f), text, title, type_);   // C 1350
      SAFE_FREE! (sa_count, sa_mark);                                     // C 1351
    }
  else
    {
      let mut text: *const c_char;                               // C 1355
      let title: *const c_char;                                  // C 1355

      /* Fall back on ANSI message box, but at least use system
	 encoding so questions representable by the system codepage
	 are encoded properly.  */
      if STRINGP (temp)                                          // C 1360
	{ text = SSDATA (ENCODE_SYSTEM (temp)); }                 // C 1361
      else
	{ text = c"".as_ptr (); }                                 // C 1363

      if NILP (header)                                          // C 1365
	{
	  title = c"Question".as_ptr ();                            // C 1367
	  type_ |= MB_ICONQUESTION;                                 // C 1368
	}
      else
	{
	  title = c"Information".as_ptr ();                         // C 1372
	  type_ |= MB_ICONINFORMATION;                              // C 1373
	}

      answer = MessageBox (FRAME_W32_WINDOW! (f), text, title, type_);   // C 1376
    }

  if answer == IDYES                                           // C 1379
    { lispy_answer = build_string (c"Yes".as_ptr ()); }          // C 1380
  else if answer == IDNO                                       // C 1381
    { lispy_answer = build_string (c"No".as_ptr ()); }           // C 1382
  else
    { quit (); }                                               // C 1384

  temp = XCDR (contents);                                      // C 1386
  while CONSP (temp)                                           // C 1386
    {
      let item: Lisp_Object;                                    // C 1388
      let mut name: Lisp_Object;                                // C 1388
      let value: Lisp_Object;                                   // C 1388
      item = XCAR (temp);                                       // C 1389
      if CONSP (item)                                           // C 1390
	{
	  name = XCAR (item);                                     // C 1392
	  value = XCDR (item);                                    // C 1393
	}
      else
	{
	  name = item;                                            // C 1397
	  value = Qnil ();                                        // C 1398
	}

      if !NILP (Fstring_equal (name, lispy_answer))              // C 1401
	{
	  return value;                                          // C 1403
	}

      temp = XCDR (temp);                                       // C 1386's `temp = XCDR (temp)`
    }
  return quit ();                                               // C 1406
}
// C 1408: `#endif  /* !HAVE_DIALOGS  */`

/* C 1411: "UTF8: 0xxxxxxx, 110xxxxx 10xxxxxx, 1110xxxx, 10xxxxxx, 10xxxxxx" - C's own text:

     /* UTF8: 0xxxxxxx, 110xxxxx 10xxxxxx, 1110xxxx, 10xxxxxx, 10xxxxxx */

   C 1412-1448: `utf8to16`.  */
/// C: static void utf8to16 (unsigned char * src, int len, WCHAR * dest)
#[cfg (feature = "have_ntgui")]
unsafe fn utf8to16 (src: *mut u8, mut len: c_int, dest: *mut WCHAR)
{
  let mut src = src;                                          // C 1414 rebinds the parameter
  let mut dest = dest;
  while len > 0                                                // C 1415
    {
      if *src < 0x80                                           // C 1417
	{
	  *dest = *src as WCHAR;                                  // C 1419
	  dest = dest.offset (1); src = src.offset (1); len -= 1;
	}
      /* Since we might get >3 byte sequences which we don't handle, ignore the extra parts.  */
      else if *src < 0xC0                                      // C 1424
	{
	  src = src.offset (1); len -= 1;
	}
      /* 2 char UTF-8 sequence.  */
      else if *src < 0xE0                                      // C 1429
	{
	  *dest = (((*src & 0x1f) as WCHAR) << 6                  // C 1431-1432
		   | (*src.offset (1) & 0x3f) as WCHAR);
	  src = src.offset (2); len -= 2; dest = dest.offset (1);
	}
      else if *src < 0xF0                                      // C 1434
	{
	  *dest = (((*src & 0x0f) as WCHAR) << 12                 // C 1436-1438
		   | ((*src.offset (1) & 0x3f) as WCHAR) << 6
		   | (*src.offset (2) & 0x3f) as WCHAR);
	  src = src.offset (3); len -= 3; dest = dest.offset (1);
	}
      else /* Not encodable. Insert Unicode Substitution char.  */   // C 1440
	{
	  *dest = 0xfffd;                                         // C 1442
	  src = src.offset (1); len -= 1; dest = dest.offset (1);
	}
    }
  *dest = 0;                                                   // C 1446
}

/* C 1450-1656: `add_menu_item`.  C's `USE_SAFE_ALLOCA`/`SAFE_ALLOCA`/`SAFE_FREE` pair is the
   mirror's macros; `local_alloc`/`local_free` are `include/w32gui.rs` 43-48's macros over
   `GetProcessHeap`/`HeapAlloc`/`HeapFree`.  C's two `#ifndef NTGUI_UNICODE` arms (C 1493-1504 and
   C 1519-1531) and the `#ifdef MENU_DEBUG` arm (C 1546-1548) are carried; `MENUITEMINFO info` is
   zero-initialised and `memset` right after it, exactly as C 1617-1618 has it.  */
/// C: static int add_menu_item (HMENU menu, widget_value *wv, HMENU item)
#[cfg (feature = "have_ntgui")]
unsafe fn add_menu_item (menu: HMENU, wv: *mut widget_value, item: HMENU) -> c_int
{
  let mut fuFlags: UINT;                                       // C 1453
  let mut out_string: *mut c_char;                             // C 1454
  let mut p: *mut c_char;                                      // C 1454
  let mut q: *mut c_char;                                      // C 1454
  let mut return_value: c_int;                                 // C 1455
  let mut nlen: usize;                                         // C 1456
  let mut orig_len: usize;                                     // C 1456
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                        // C 1457

  if menu_separator_name_p ((*wv).name)                        // C 1459
    {
      fuFlags = MF_SEPARATOR;                                  // C 1461
      out_string = std::ptr::null_mut ();                       // C 1462
    }
  else
    {
      if (*wv).enabled                                         // C 1466
	{ fuFlags = MF_STRING; }                               // C 1467
      else
	{ fuFlags = MF_STRING | MF_GRAYED; }                    // C 1469

      if !(*wv).key.is_null ()                                 // C 1471
	{
	  out_string = SAFE_ALLOCA! (sa_avail, strlen ((*wv).name)                    // C 1473
				     + strlen ((*wv).key) + 2) as *mut c_char;
	  p = stpcpy (out_string, (*wv).name);                      // C 1474
	  p = stpcpy (p, c"\t".as_ptr ());                           // C 1475
	  strcpy (p, (*wv).key);                                    // C 1476
	}
      else
	{ out_string = (*wv).name; }                             // C 1479

      /* Quote any special characters within the menu item's text and
	 key binding.  */
      orig_len = strlen (out_string);                            // C 1483
      nlen = orig_len;
      if unicode_append_menu.is_some ()                          // C 1484
        {
          /* With UTF-8, & cannot be part of a multibyte character.  */
          p = out_string;                                        // C 1487
          while *p != 0
            {
              if *p == b'&' as c_char                             // C 1489
                { nlen += 1; }                                   // C 1490
              p = p.offset (1);
            }
        }
      /* C 1493-1504: `#ifndef NTGUI_UNICODE`'s `#else` - the multibyte arm.  */
      #[cfg (not (feature = "ntgui_unicode"))]
      {
        if unicode_append_menu.is_none ()
          {
            /* If encoded with the system codepage, use multibyte string
               functions in case of multibyte characters that contain '&'.  */
            p = out_string;                                      // C 1498
            while *p != 0
              {
                if _mbsnextc (p as *const u8) == b'&' as c_uint   // C 1500
                  { nlen += 1; }                                 // C 1501
                p = _mbsinc (p as *const u8) as *mut c_char;
              }
          }
      }

      if nlen > orig_len                                         // C 1506
        {
          p = out_string;                                        // C 1508
          out_string = SAFE_ALLOCA! (sa_avail, nlen + 1) as *mut c_char;   // C 1509
          q = out_string;                                        // C 1510
          while *p != 0
            {
              if unicode_append_menu.is_some ()                   // C 1513
                {
                  if *p == b'&' as c_char                         // C 1515
                    { *q = *p; q = q.offset (1); }                // C 1516
                  *q = *p; q = q.offset (1); p = p.offset (1);    // C 1517
                }
              /* C 1519-1531: `#ifndef NTGUI_UNICODE`'s `#else`.  */
              #[cfg (not (feature = "ntgui_unicode"))]
              {
                if unicode_append_menu.is_none ()
                  {
                    if _mbsnextc (p as *const u8) == b'&' as c_uint   // C 1522
                      {
                        _mbsncpy (q as *mut u8, p as *const u8, 1);   // C 1524
                        q = _mbsinc (q as *const u8) as *mut c_char;  // C 1525
                      }
                    _mbsncpy (q as *mut u8, p as *const u8, 1);       // C 1527
                    p = _mbsinc (p as *const u8) as *mut c_char;      // C 1528
                    q = _mbsinc (q as *const u8) as *mut c_char;      // C 1529
                  }
              }
            }
          *q = 0;                                                // C 1533
        }

      if !item.is_null ()                                        // C 1536
	{ fuFlags = MF_POPUP; }                                  // C 1537
      else if (*wv).title || ((*wv).call_data).is_null ()         // C 1538
	{
	  /* Only use MF_OWNERDRAW if GetMenuItemInfo is usable, since
	     we can't deallocate the memory otherwise.  */
	  if get_menu_item_info.is_some ()                        // C 1542
	    {
              out_string = local_alloc! (strlen ((*wv).name) + 1) as *mut c_char;   // C 1544
              strcpy (out_string, (*wv).name);                    // C 1545
              /* C 1546-1548: `#ifdef MENU_DEBUG`.  */
              #[cfg (feature = "menu_debug")]
              {
		DebPrint (c"Menu: allocating %ld for owner-draw".as_ptr (), out_string);   // C 1547
              }
	      fuFlags = MF_OWNERDRAW | MF_DISABLED;                // C 1549
	    }
	  else
	    { fuFlags = MF_DISABLED; }                            // C 1552
	}

      /* Draw radio buttons and tickboxes. */
      else if (*wv).selected && ((*wv).button_type == BUTTON_TYPE_TOGGLE       // C 1556-1557
				 || (*wv).button_type == BUTTON_TYPE_RADIO)
	{ fuFlags |= MF_CHECKED; }                                // C 1558
      else
	{ fuFlags |= MF_UNCHECKED; }                              // C 1560
    }

  if unicode_append_menu.is_some () && !out_string.is_null ()    // C 1563
    {
      /* Convert out_string from UTF-8 to UTF-16-LE.  */
      let utf8_len: c_int = strlen (out_string) as c_int;         // C 1566
      let utf16_string: *mut WCHAR;                               // C 1567
      if (fuFlags & MF_OWNERDRAW) != 0                            // C 1568
	{ utf16_string = local_alloc! ((utf8_len + 1) as usize
				      * core::mem::size_of::<WCHAR> ()) as *mut WCHAR; }   // C 1569
      else
	{ utf16_string = SAFE_ALLOCA! (sa_avail, (utf8_len + 1) as isize
				       * core::mem::size_of::<WCHAR> () as isize)
			 as *mut WCHAR; }                     // C 1571

      utf8to16 (out_string as *mut u8, utf8_len, utf16_string);   // C 1573
      return_value = unicode_append_menu.unwrap () (menu, fuFlags,   // C 1574-1577
					  if !item.is_null () { item as UINT_PTR }
					  else { (*wv).call_data as UINT_PTR },
					  utf16_string) as c_int;

      /* C 1579-1598: `#ifndef NTGUI_UNICODE /* Fallback does not apply when always UNICODE */`.  */
      #[cfg (not (feature = "ntgui_unicode"))]
      {
	if return_value == 0                                      // C 1580
	  {
	    /* On W9x/ME, Unicode menus are not supported, though AppendMenuW
	       apparently does exist at least in some cases and appears to be
	       stubbed out to do nothing.  out_string is UTF-8, but since
	       our standard menus are in English and this is only going to
	       happen the first time a menu is used, the encoding is
	       of minor importance compared with menus not working at all.  */
	    return_value = AppendMenu (menu, fuFlags,              // C 1588-1591
			if !item.is_null () { item as UINT_PTR }
			else { (*wv).call_data as UINT_PTR },
			out_string) as c_int;
	    /* Don't use Unicode menus in future, unless this is Windows
	       NT or later, where a failure of AppendMenuW does NOT mean
	       Unicode menus are unsupported.  */
	    if osinfo_cache.dwPlatformId != VER_PLATFORM_WIN32_NT   // C 1595
	      { unicode_append_menu = None; }                      // C 1596
	  }
      }

      if unicode_append_menu.is_some () && (fuFlags & MF_OWNERDRAW) != 0   // C 1600
	{ local_free! (out_string); }                              // C 1601
    }
  else
    {
      return_value =
	AppendMenu (menu,                                         // C 1606
		    fuFlags,
		    if !item.is_null () { item as UINT_PTR }
		    else { (*wv).call_data as UINT_PTR },
		    out_string) as c_int;                             // C 1609
    }

  /* This must be done after the menu item is created.  */
  if !(*wv).title && !((*wv).call_data).is_null ()                 // C 1613
    {
      if set_menu_item_info.is_some ()                             // C 1615
	{
	  let mut info: MENUITEMINFO = core::mem::zeroed ();         // C 1617
	  memset (&raw mut info as *mut c_void, 0, core::mem::size_of::<MENUITEMINFO> ());  // C 1618
	  info.cbSize = core::mem::size_of::<MENUITEMINFO> () as UINT;   // C 1619
	  info.fMask = MIIM_DATA;                                   // C 1620

	  /* Set help string for menu item.  Leave it as a pointer to
	     a Lisp_String until it is ready to be displayed, since GC
	     can happen while menus are active.  */
	  if !NILP ((*wv).help)                                     // C 1625
	    {
	      /* We use XUNTAG below because in a 32-bit build
		 --with-wide-int we cannot pass a Lisp_Object
		 via a DWORD member of MENUITEMINFO.  */
	      /* As of Jul-2012, w32api headers say that dwItemData
		 has DWORD type, but that's a bug: it should actually
		 be ULONG_PTR, which is correct for 32-bit and 64-bit
		 Windows alike.  MSVC headers get it right; hopefully,
		 MinGW headers will, too.  */
	      eassert! (STRINGP ((*wv).help));                        // C 1635
	      info.dwItemData = XUNTAG! ((*wv).help, Lisp_Type::Lisp_String,   // C 1636-1637
					 Lisp_String) as ULONG_PTR;
	    }
	  if (*wv).button_type == BUTTON_TYPE_RADIO                 // C 1639
	    {
	      /* CheckMenuRadioItem allows us to differentiate TOGGLE and
		 RADIO items, but is not available on NT 3.51 and earlier.  */
	      info.fMask |= MIIM_TYPE | MIIM_STATE;                 // C 1643
	      info.fType = MFT_RADIOCHECK | MFT_STRING;             // C 1644
	      info.dwTypeData = out_string;                         // C 1645
	      info.fState = if (*wv).selected                        // C 1646
		{ MFS_CHECKED } else { MFS_UNCHECKED };
	    }

	  set_menu_item_info.unwrap () (menu,                        // C 1649-1651
			if !item.is_null () { item as UINT_PTR as UINT }
			else { (*wv).call_data as UINT_PTR as UINT },
			FALSE, &raw const info);
	}
    }
  SAFE_FREE! (sa_count, sa_mark);                                        // C 1654
  return return_value;                                          // C 1655
}

/* C 1658: "Construct native Windows menu(bar) based on widget_value tree."  C 1659-1685:
   `fill_in_menu`.  */
/// C: static int fill_in_menu (HMENU menu, widget_value *wv)
#[cfg (feature = "have_ntgui")]
unsafe fn fill_in_menu (menu: HMENU, wv: *mut widget_value) -> c_int
{
  let mut wv = wv;                                              // C 1662 rebinds the parameter
  while !wv.is_null ()                                          // C 1662
    {
      if !(*wv).contents.is_null ()                              // C 1664
	{
	  let sub_menu: HMENU = CreatePopupMenu ();                 // C 1666

	  if sub_menu.is_null ()                                   // C 1668
	    { return 0; }

	  if fill_in_menu (sub_menu, (*wv).contents) == 0          // C 1671
	      || add_menu_item (menu, wv, sub_menu) == 0            // C 1672
	    {
	      DestroyMenu (sub_menu);                              // C 1674
	      return 0;
	    }
	}
      else
	{
	  if add_menu_item (menu, wv, std::ptr::null_mut ()) == 0    // C 1680
	    { return 0; }
	}
      wv = (*wv).next;                                          // C 1662's `wv = wv->next`
    }
  return 1;                                                     // C 1684
}

/* C 1687-1689's comment ("Display help string for currently pointed to menu item. Not supported on
   NT 3.51 and earlier, as GetMenuItemInfo is not available.") and C 1690's prototype of the
   function C 1692-1748 defines - the prototype is dropped (PORTING.md 0.1-12):

     /* Display help string for currently pointed to menu item. Not
        supported on NT 3.51 and earlier, as GetMenuItemInfo is not
        available. */
     void w32_menu_display_help (HWND, HMENU, UINT, UINT);

   C 1692-1748: `w32_menu_display_help`.  */
/// C: void w32_menu_display_help (HWND owner, HMENU menu, UINT item, UINT flags)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_menu_display_help (owner: HWND, menu: HMENU, item: UINT,
                                               flags: UINT)
{
  if get_menu_item_info.is_some ()                               // C 1695
    {
      let f: *mut frame = w32_window_to_frame (&raw mut one_w32_display_info, owner);   // C 1697
      let mut frame: Lisp_Object;                                 // C 1698
      let mut help: Lisp_Object;                                  // C 1698

      /* No help echo on owner-draw menu items, or when the keyboard
	 is used to navigate the menus, since tooltips are distracting
	 if they pop up elsewhere.  */
      if (flags & MF_OWNERDRAW) != 0 || (flags & MF_POPUP) != 0   // C 1703
	  || (flags & MF_MOUSESELECT) == 0                        // C 1704
	  /* Ignore any dwItemData for menu items whose flags don't
	     have the MF_HILITE bit set.  These are dwItemData that
	     Windows sends our way, but they aren't pointers to our
	     Lisp_String objects, so trying to create Lisp_Strings out
	     of them below and pass that to the keyboard queue will
	     crash Emacs when we try to display those "strings".  It
	     is unclear why we get these dwItemData, or what they are:
	     sometimes they point to a wchar_t string that is the menu
	     title, sometimes to something that doesn't look like text
	     at all.  (The problematic data also comes with the 0x0800
	     bit set, but this bit is not documented, so we don't want
	     to depend on it.)  */
	  || (flags & MF_HILITE) == 0                             // C 1717
	{ help = Qnil (); }                                       // C 1718
      else
	{
	  let mut info: MENUITEMINFO = core::mem::zeroed ();         // C 1721

	  memset (&raw mut info as *mut c_void, 0, core::mem::size_of::<MENUITEMINFO> ());  // C 1723
	  info.cbSize = core::mem::size_of::<MENUITEMINFO> () as UINT;   // C 1724
	  info.fMask = MIIM_DATA;                                   // C 1725
	  get_menu_item_info.unwrap () (menu, item, FALSE, &raw mut info);   // C 1726

	  help =                                                    // C 1728-1731
	    if info.dwItemData != 0
	    { make_lisp_ptr (info.dwItemData as *mut c_void, Lisp_Type::Lisp_String) }
	    else { Qnil () };
	}

      /* Store the help echo in the keyboard buffer as the X toolkit
	 version does, rather than directly showing it. This seems to
	 solve the GC problems that were present when we based the
	 Windows code on the non-toolkit version.  */
      if !f.is_null ()                                          // C 1738
	{
	  XSETFRAME! (frame, f);                                    // C 1740
	  kbd_buffer_store_help_event (frame, help);                // C 1741
	}
      else
	/* X version has a loop through frames here, which doesn't
	   appear to do anything, unless it has some side effect.  */
	{ show_help_echo (help, Qnil (), Qnil (), Qnil ()); }       // C 1746
    }
}

/* C 1750: "Free memory used by owner-drawn strings."  C 1751-1777:
   `w32_free_submenu_strings`.  */
/// C: static void w32_free_submenu_strings (HMENU menu)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_free_submenu_strings (menu: HMENU)
{
  let mut i: c_int;                                             // C 1754
  let num: c_int = GetMenuItemCount (menu);                      // C 1754
  i = 0;
  while i < num                                                  // C 1755
    {
      let mut info: MENUITEMINFO = core::mem::zeroed ();           // C 1757
      memset (&raw mut info as *mut c_void, 0, core::mem::size_of::<MENUITEMINFO> ());  // C 1758
      info.cbSize = core::mem::size_of::<MENUITEMINFO> () as UINT;   // C 1759
      info.fMask = MIIM_DATA | MIIM_TYPE | MIIM_SUBMENU;           // C 1760

      get_menu_item_info.unwrap () (menu, i as UINT, TRUE, &raw mut info);   // C 1762

      /* Owner-drawn names are held in dwItemData.  */
      if (info.fType & MF_OWNERDRAW) != 0 && info.dwItemData != 0   // C 1765
	{
	  /* C 1767-1769: `#ifdef MENU_DEBUG`.  */
	  #[cfg (feature = "menu_debug")]
	  {
	    DebPrint (c"Menu: freeing %ld for owner-draw".as_ptr (), info.dwItemData);   // C 1768
	  }
	  local_free! (info.dwItemData);                             // C 1770
	}

      /* Recurse down submenus.  */
      if !info.hSubMenu.is_null ()                               // C 1774
	{ w32_free_submenu_strings (info.hSubMenu); }             // C 1775
      i += 1;                                                    // C 1755's `i++`
    }
}

/// C: void w32_free_menu_strings (HWND hwnd)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_free_menu_strings (hwnd: HWND)      // C 1779-1796
{
  let mut menu: HMENU = current_popup_menu;                      // C 1782

  if get_menu_item_info.is_some ()                                // C 1784
    {
      /* If there is no popup menu active, free the strings from the frame's
	 menubar.  */
      if menu.is_null ()                                         // C 1788
	{ menu = GetMenu (hwnd); }                                // C 1789

      if !menu.is_null ()                                        // C 1791
	{ w32_free_submenu_strings (menu); }                       // C 1792
    }

  current_popup_menu = std::ptr::null_mut ();                     // C 1795
}

/* C 1798: "The following is used by delayed window autoselection."  C 1800-1807: the subr.  C's
   `f->output_data.w32->menubar_active` keeps C's field name through `w32_output_bits!` (the
   member is recorded, not packed - see the macro).  */
/// C: DEFUN ("menu-or-popup-active-p", Fmenu_or_popup_active_p, Smenu_or_popup_active_p, 0, 0, 0,
/// C:   doc: /* SKIP: real doc in xmenu.c.  */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("menu-or-popup-active-p", Fmenu_or_popup_active_p, Smenu_or_popup_active_p, 0, 0, 0,
       doc: "SKIP: real doc in xmenu.c.  ",
       (),
       {
  let f: *mut frame;                                             // C 1804
  f = SELECTED_FRAME ();                                         // C 1805
  return if (*( (*f).output_data.w32 as *mut w32_output )).menubar_active > 0 { Qt () } else { Qnil () };   // C 1806
});

/// C: void syms_of_w32menu (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32menu ()                      // C 1809-1821
{
  globals_of_w32menu ();                                         // C 1812

  current_popup_menu = std::ptr::null_mut ();                     // C 1814
  PDUMPER_IGNORE! (current_popup_menu);                           // C 1815

  DEFSYM! (Qdebug_on_next_call, "debug-on-next-call");             // C 1817
  DEFSYM! (Qunsupported__w32_dialog, "unsupported--w32-dialog");   // C 1818

  defsubr (&raw mut Smenu_or_popup_active_p);                      // C 1820
}

/* C 1823-1830: the banner comment of `globals_of_w32menu` - carried in C's own words:

   /*
   	globals_of_w32menu is used to initialize those global variables that
   	must always be initialized on startup even when the global variable
   	initialized is non zero (see the function main in emacs.c).
   	globals_of_w32menu is called from syms_of_w32menu when the global
   	variable initialized is 0 and directly from main when initialized
   	is non zero.
    */
*/

/// C: void globals_of_w32menu (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn globals_of_w32menu ()                   // C 1831-1850
{
  let comctrl32: HMODULE = GetModuleHandle (c"comctl32.dll".as_ptr ());   // C 1834
  task_dialog_indirect                                                       // C 1835-1836
    = core::mem::transmute::<VOIDFNPTR, TaskDialogIndirect_Proc> (
	get_proc_addr (comctrl32, c"TaskDialogIndirect".as_ptr ()));

  /* C 1838-1849: `#ifndef NTGUI_UNICODE` - "See if Get/SetMenuItemInfo functions are
     available."  */
  #[cfg (not (feature = "ntgui_unicode"))]
  {
    let user32: HMODULE = GetModuleHandle (c"user32.dll".as_ptr ());      // C 1840
    get_menu_item_info                                                     // C 1841-1842
      = core::mem::transmute::<VOIDFNPTR, GetMenuItemInfoA_Proc> (
	  get_proc_addr (user32, c"GetMenuItemInfoA".as_ptr ()));
    set_menu_item_info                                                     // C 1843-1844
      = core::mem::transmute::<VOIDFNPTR, SetMenuItemInfoA_Proc> (
	  get_proc_addr (user32, c"SetMenuItemInfoA".as_ptr ()));
    unicode_append_menu                                                    // C 1845-1846
      = core::mem::transmute::<VOIDFNPTR, AppendMenuW_Proc> (
	  get_proc_addr (user32, c"AppendMenuW".as_ptr ()));
    unicode_message_box                                                    // C 1847-1848
      = core::mem::transmute::<VOIDFNPTR, MessageBoxW_Proc> (
	  get_proc_addr (user32, c"MessageBoxW".as_ptr ()));
  }
}
