// femacs: port of src/w32term.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32term.o`
// sits in `W32_OBJ` under `W32_OBJ=@W32_OBJ@` (`emacs/src/Makefile.in` 319, filled by
// `configure.ac` under the `HAVE_W32` test that also `AC_DEFINE`s `HAVE_NTGUI`), and
// `docs/baseline/frozen-build-sources.txt` lists no `w32term.o` for this target - so
// `extract-abi.py`'s checklist for the file is empty (P-86 clause 2) and the ledger
// `docs/ports/w32term.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.  This batch is P-86 clause 4's "first slice":
// **C 1-1002**, the whole items up to the nearest boundary of C 1000 (C 1000 is inside
// `w32_define_fringe_bitmap`, C 989-1002; the ledger records the choice and the neighbours).
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  `HAVE_NTGUI` is the macro the
//     Windows GUI build defines; it is *not* `WINDOWSNT` (the Cygwin GUI build compiles this file
//     with `HAVE_NTGUI` and without `WINDOWSNT`, and C 51/55/214 gate on the latter);
//   * C 34's `#ifdef CYGWIN`, C 51's `#ifdef WINDOWSNT`, C 55's `#ifndef WINDOWSNT`, C 214's
//     `#ifdef CYGWIN` and C 255's `#ifdef GLYPH_DEBUG` are the features `cygwin`, `windowsnt` and
//     `glyph_debug` - each inside the file gate, **both arms carried**;
//   * C 98's `#if _WIN32_WINNT < 0x0500 && !defined(MINGW_W64)` and C 129's
//     `#if _WIN32_WINNT < 0x0601` are the two conditions
//     `tools/check-cond-eval.py src/w32term.c` leaves unknown (`41 conditions, 2 unknown,
//     0 disagreements [VERIFIED]`), so both arms stay live text: the derived features are
//     `win32_winnt_lt_0500` (with `mingw_w64` for the second half of the first predicate) and
//     `win32_winnt_lt_0601`;
//   * the `#ifndef X` blocks (C 79, 165, 170, 176, 179) are "define it if the SDK has not" guards:
//     the port carries the constant with C's value and no `cfg`;
//   * the `#if 0` regions (C 64-66, 134-139, 248-250, 264-289, 312-319) are dead text in every
//     configuration; each is carried as C's own text inside a comment (the `cm.rs` 86 reading).
//
// `w32term.h`'s mirror (`include/w32term.rs`) gates its whole body on `feature = "windowsnt"`,
// because the header is only ever reached from a Windows GUI unit; the records this file's items
// name (`w32_display_info`, `w32_output`, `W32Msg`, the `MS`-level entry points) therefore arrive
// with that feature, and a Windows GUI build sets both it and `HAVE_NTGUI` - the interaction
// `docs/ports/w32fns.md` records for the same file pair.
//
// **Acquisition note - this batch is `text-only` (P-86 clause 3-2).**  The machine has no Windows
// and no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is
// empty, no Homebrew `mingw-w64`, and no `windows.h` anywhere under the toolchain roots), so there
// is no `<windows.h>` to compile or measure against and **no arm flip on a Windows target and no
// layout measurement**: the `<windows.h>` section below is spelled from C's own member text and
// PORTING.md 1.1's fixed widths, every `#[repr(C)]` record in it is `UNVERIFIED` here, and the
// readings the ledger carries are the frozen `build-tu`, the multi-configuration sweep of this
// unit's own text, and the arm-reach pre-flight over patched `/tmp` copies of the mirrors - which
// are `/tmp` harness readings, **not** a Windows-target compile.
//
// `docs/ports/w32term.md` carries the C line ranges, the acceptance readings, every deviation (the
// `<windows.h>` section, the dropped forward declarations, the two mirror collisions, the five
// mirror names this unit has to declare itself) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 20-66: the include graph.  Every header sits inside the file gate, as C's own build has it (no
   translation unit the frozen configuration builds reaches this file).  */
// C 20: `#include <config.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/signal.rs");           // C 21: `#include <signal.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdio.rs");            // C 22: `#include <stdio.h>`
#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                    // C 23: `#include "lisp.h"`
#[cfg (feature = "have_ntgui")]
include!("include/blockinput.rs");              // C 24: `#include "blockinput.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                 // C 25: `#include "w32term.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32common.rs");               // C 26: `#include "w32common.h"`	/* for OS version info */
/* C 27-28: `<wtypes.h>` and `<gdiplus.h>` - SDK headers with no file in the reference tree (the
   P-88 shape for a header with no mirror); this slice names nothing from either.  */
// C 27: `#include <wtypes.h>`
// C 28: `#include <gdiplus.h>`
#[cfg (feature = "have_ntgui")]
include!("include/w32gdiplus.rs");              // C 29: `#include "w32gdiplus.h"`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/ctype.rs");            // C 31: `#include <ctype.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/errno.rs");            // C 32: `#include <errno.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/sys_stat.in.rs");      // C 33: `#include <sys/stat.h>`
#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
include!("../lib/include/fcntl.in.rs");         // C 35: `#include <fcntl.h>`	/* for O_RDWR */
// C 37: `#include <imm.h>` - an SDK header with no file in the reference tree; this slice names
// nothing from it (`w32fns.c`'s own `<imm.h>` names are that unit's).
#[cfg (feature = "have_ntgui")]
include!("../lib/include/math.rs");             // C 38: `#include <math.h>` (floor)
#[cfg (feature = "have_ntgui")]
include!("include/coding.rs");                  // C 40: `#include "coding.h"`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                   // C 41: `#include "frame.h"`
#[cfg (feature = "have_ntgui")]
include!("include/fontset.rs");                 // C 42: `#include "fontset.h"`
#[cfg (feature = "have_ntgui")]
include!("include/termhooks.rs");               // C 43: `#include "termhooks.h"`
#[cfg (feature = "have_ntgui")]
include!("include/termopts.rs");                // C 44: `#include "termopts.h"`
#[cfg (feature = "have_ntgui")]
include!("include/termchar.rs");                // C 45: `#include "termchar.h"`
#[cfg (feature = "have_ntgui")]
include!("include/buffer.rs");                  // C 46: `#include "buffer.h"`
#[cfg (feature = "have_ntgui")]
include!("include/window.rs");                  // C 47: `#include "window.h"`
#[cfg (feature = "have_ntgui")]
include!("include/keyboard.rs");                // C 48: `#include "keyboard.h"`
#[cfg (feature = "have_ntgui")]
include!("include/menu.rs");                    // C 49: `#include "menu.h"`	/* for w32_menu_show */
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32.rs");                     // C 52: `#include "w32.h"`	/* for filename_from_utf16, filename_from_ansi */
/* C 55-57: `#ifndef WINDOWSNT` / `#include <io.h>` / `#endif` - an SDK header with no file in the
   reference tree ("for get_osfhandle"); this slice names nothing from it.  */
// C 56: `#include <io.h>` /* for get_osfhandle */
// C 59: `#include <shellapi.h>` - an SDK header with no file in the reference tree; the entry
// points it declares are named by the units that call them.
#[cfg (feature = "have_ntgui")]
include!("include/font.rs");                    // C 61: `#include "font.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32font.rs");                 // C 62: `#include "w32font.h"`

/* C 64-66 is `#if 0` ("TODO: stipple"): C's own preprocessor never compiles it, so the port keeps
   C's text as a comment (the `cm.rs` 86 reading).

#if 0	/* TODO: stipple */
#include "bitmaps/gray.xbm"
#endif
*/

/* ----------------------------------------------------------------------------------------------
   `<windows.h>` and the SDK's own names.

   C reaches `<windows.h>` through C 25's chain (`w32term.h` 22 -> `w32gui.h` 21), which the
   `include/w32gui.rs` and `include/w32term.rs` notes record; it has no file in the reference tree,
   so **the names this slice uses stand here** (the `w32fns.rs` 199 section's reading of the same
   header: P-88 clause 1's declaration site is the chain's include line, and no file is added under
   `femacs/src/include/`).

   **No measurement was possible**: this machine has no `windows.h` at all (see the file note), so
   every scalar below is spelled from PORTING.md 1.1's fixed widths - `DWORD`/`LONG`/`ULONG` are
   32-bit on Windows whatever this target's `c_long` is - and every record's size is
   **UNVERIFIED**; the ledger's owed list carries the layout assertions.
   ---------------------------------------------------------------------------------------------- */

/// C 130-141, 145-155 and 121/159-160's parameter lists name these; `windows.h` gives each one.
/// C: typedef unsigned char BYTE; typedef unsigned short WCHAR; typedef unsigned short USHORT;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned int UINT;
/// C: typedef int BOOL; typedef ULONG_PTR unsigned long;
#[cfg (feature = "have_ntgui")]
pub type BYTE = u8;
#[cfg (feature = "have_ntgui")]
pub type WCHAR = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type USHORT = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type DWORD = u32;
#[cfg (feature = "have_ntgui")]
pub type LONG = i32;
#[cfg (feature = "have_ntgui")]
pub type UINT = c_uint;
#[cfg (feature = "have_ntgui")]
pub type BOOL = c_int;
#[cfg (feature = "have_ntgui")]
pub type ULONG_PTR = usize;
/// C's handle names are all `void *` at the ABI (the `include/w32gui.rs` 16-19 reading).  `HGDIOBJ`
/// is the GDI object handle C 367/538-541's `SelectObject` returns and C 368's `DeleteObject`
/// takes; `HPEN`, `HBRUSH` and `HBITMAP` are the three concrete ones this slice creates, each an
/// alias of it so that C's implicit conversion at the assignment sites needs no cast.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *HDC; typedef void *HBITMAP;
/// C: typedef void *HBRUSH; typedef void *HPEN; typedef void *HRGN; typedef void *HCURSOR;
/// C: typedef void *HGDIOBJ; typedef COLORREF DWORD; typedef BYTE *LPBYTE;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HWND = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HDC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HBITMAP = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HBRUSH = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HPEN = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HRGN = HGDIOBJ;
#[cfg (feature = "have_ntgui")]
pub type HCURSOR = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
/// C 91's `LPCTSTR` is the SDK's `const TCHAR *`; the port's `char` is PORTING.md 1.1's `c_char`.
/// C: typedef const char *LPCTSTR;
#[cfg (feature = "have_ntgui")]
pub type LPCTSTR = *const c_char;
/// C 693/782's `SendMessageTimeout` parameter pair and its result.
/// C: typedef UINT_PTR WPARAM; typedef LONG_PTR LPARAM; typedef LONG_PTR LRESULT;
#[cfg (feature = "have_ntgui")]
pub type WPARAM = usize;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = isize;
#[cfg (feature = "have_ntgui")]
pub type LRESULT = isize;
/// C's `WINAPI` is `__stdcall`, which Rust spells `extern "system"` (the `create.zig`/`w32fns.rs`
/// convention); the note stays on the typedefs and declarations that use it below.
/// C: #define WINAPI __stdcall
/// C's `WINAPI` is `__stdcall`, which Rust spells `extern "system"` (the `w32fns.rs` convention);
/// the note stays on the declarations that use it below.  C 668's `FALSE` is the SDK's
/// (`<windef.h>`'s two names), carried with C's values and types.
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;

/// C 124 and C 461/546's record and C 558's local: `typedef struct tagPOINT { LONG x, y; }` and
/// `typedef struct tagRECT { LONG left, top, right, bottom; }`.
/// C: typedef struct tagPOINT { LONG x; LONG y; } POINT;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINT
{
  pub x: LONG,
  pub y: LONG,
}
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
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
/// C 124/536's pointer spellings and C 693's out parameter.
/// C: typedef POINT *LPPOINT; typedef RECT *LPRECT; typedef DWORD_PTR *PDWORD_PTR;
#[cfg (feature = "have_ntgui")]
pub type LPPOINT = *mut POINT;
#[cfg (feature = "have_ntgui")]
pub type LPRECT = *mut RECT;
#[cfg (feature = "have_ntgui")]
pub type PDWORD_PTR = *mut DWORD_PTR;

/* C 336/344/341, 395-400, 414-419, 484-485, 501/505, 525-526, 536, 550-551, 559, 611/619,
   626/635, 384-385 and 693/782's callees: the `<windows.h>`/`<wingdi.h>`/`<winuser.h>` entry
   points this slice calls, one `unsafe extern "system"` block per header group as the plane
   header comment describes.  C's `WINAPI` is the block's ABI; each declaration keeps C's own
   prototype in a `/// C:` line above it.  */
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  /// C 336: `raw_dc = GetDC (output->window_desc);`
  /// C: HDC WINAPI GetDC (HWND hWnd);
  pub fn GetDC (hWnd: HWND) -> HDC;
  /// C 344: `ReleaseDC (output->window_desc, raw_dc);`
  /// C: int WINAPI ReleaseDC (HWND hWnd, HDC hDC);
  pub fn ReleaseDC (hWnd: HWND, hDC: HDC) -> c_int;
  /// C 341-343: `BitBlt (raw_dc, 0, 0, …, output->paint_dc, 0, 0, SRCCOPY);`
  /// C: BOOL WINAPI BitBlt (HDC hdcDest, int x, int y, int cx, int cy, HDC hdcSrc, int x1,
  /// C:                     int y1, DWORD rop);
  pub fn BitBlt (hdcDest: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC,
                 x1: c_int, y1: c_int, rop: DWORD) -> BOOL;
  /// C 363-368 and 527-528/538-539: `SelectObject`/`DeleteObject`.
  /// C: HGDIOBJ WINAPI SelectObject (HDC hdc, HGDIOBJ h);
  /// C: BOOL WINAPI DeleteObject (HGDIOBJ ho);
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
  /// C 367: `DeleteDC (FRAME_OUTPUT_DATA (f)->paint_dc);`
  /// C: BOOL WINAPI DeleteDC (HDC hdc);
  pub fn DeleteDC (hdc: HDC) -> BOOL;
  /// C 395: `HRGN clip_region = CreateRectRgnIndirect (rect);`
  /// C: HRGN WINAPI CreateRectRgnIndirect (const RECT *lprect);
  pub fn CreateRectRgnIndirect (lprect: *const RECT) -> HRGN;
  /// C 396/400: `SelectClipRgn (hdc, clip_region);` and `SelectClipRgn (hdc, NULL);`
  /// C: int WINAPI SelectClipRgn (HDC hdc, HRGN hrgn);
  pub fn SelectClipRgn (hdc: HDC, hrgn: HRGN) -> c_int;
  /// C 416: `if (CombineRgn (clip1, clip1, clip2, RGN_OR) != ERROR)`
  /// C: int WINAPI CombineRgn (HRGN hrgnDst, HRGN hrgnSrc1, HRGN hrgnSrc2, int iMode);
  pub fn CombineRgn (hrgnDst: HRGN, hrgnSrc1: HRGN, hrgnSrc2: HRGN, iMode: c_int) -> c_int;
  /// C 484/526: `hp = CreatePen (PS_SOLID, thickness, color);` / `CreatePen (PS_SOLID, 0, …)`
  /// C: HPEN WINAPI CreatePen (int iStyle, int cWidth, COLORREF color);
  pub fn CreatePen (iStyle: c_int, cWidth: c_int, color: COLORREF) -> HPEN;
  /// C 525/550: `hb = CreateSolidBrush (gc->background);` / `CreateSolidBrush (pix);`
  /// C: HBRUSH WINAPI CreateSolidBrush (COLORREF color);
  pub fn CreateSolidBrush (color: COLORREF) -> HBRUSH;
  /// C 501/505: `MoveToEx (s->hdc, x1, y1, NULL);` / `LineTo (s->hdc, x2, y2);`
  /// C: BOOL WINAPI MoveToEx (HDC hdc, int x, int y, LPPOINT lppt);
  /// C: BOOL WINAPI LineTo (HDC hdc, int x, int y);
  pub fn MoveToEx (hdc: HDC, x: c_int, y: c_int, lppt: LPPOINT) -> BOOL;
  pub fn LineTo (hdc: HDC, x: c_int, y: c_int) -> BOOL;
  /// C 536: `Rectangle (hdc, x, y, x + width + 1, y + height + 1);`
  /// C: BOOL WINAPI Rectangle (HDC hdc, int left, int top, int right, int bottom);
  pub fn Rectangle (hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
  /// C 551: `FillRect (hdc, lprect, hb);`
  /// C: int WINAPI FillRect (HDC hDC, const RECT *lprc, HBRUSH hbr);
  pub fn FillRect (hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
  /// C 564: `GetClientRect (FRAME_W32_WINDOW (f), &rect);`
  /// C: BOOL WINAPI GetClientRect (HWND hWnd, LPRECT lpRect);
  pub fn GetClientRect (hWnd: HWND, lpRect: LPRECT) -> BOOL;
  /// C 611/619: `ex_style = GetWindowLong (window, GWL_EXSTYLE);` and
  /// `SetWindowLong (window, GWL_EXSTYLE, ex_style);`
  /// C: LONG WINAPI GetWindowLongA (HWND hWnd, int nIndex);
  /// C: LONG WINAPI SetWindowLongA (HWND hWnd, int nIndex, LONG dwNewLong);
  pub fn GetWindowLong (hWnd: HWND, nIndex: c_int) -> LONG;
  pub fn SetWindowLong (hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
  /// C 624/631/638/641: `GetSystemMetrics (SM_CYVIRTUALSCREEN)` and its three siblings.
  /// C: int WINAPI GetSystemMetrics (int nIndex);
  pub fn GetSystemMetrics (nIndex: c_int) -> c_int;
  /// C 384: `SystemParametersInfo (SPI_GETWHEELSCROLLLINES, 0, &scroll_lines, 0);`
  /// C: BOOL WINAPI SystemParametersInfoA (UINT uiAction, UINT uiParam, PVOID pvParam,
  /// C:                                 UINT fWinIni);
  pub fn SystemParametersInfo (uiAction: UINT, uiParam: UINT, pvParam: *mut c_void,
                               fWinIni: UINT) -> BOOL;
  /// C 693/782: `SendMessageTimeout (w32_system_caret_hwnd, WM_EMACS_HIDE_CARET, 0, 0, 0, 6000,
  /// NULL);`
  /// C: LRESULT WINAPI SendMessageTimeoutA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam,
  /// C:                                    UINT fuFlags, UINT uTimeout, PDWORD_PTR lpdwResult);
  pub fn SendMessageTimeout (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
                             fuFlags: UINT, uTimeout: UINT, lpdwResult: PDWORD_PTR) -> LRESULT;
  /// C 1001: `fringe_bmp[which] = CreateBitmap (wd, h, 1, 1, bits);`
  /// C: HBITMAP WINAPI CreateBitmap (int nWidth, int nHeight, UINT nPlanes, UINT nBitCount,
  /// C:                              const void *lpBits);
  pub fn CreateBitmap (nWidth: c_int, nHeight: c_int, nPlanes: UINT, nBitCount: UINT,
                       lpBits: *const c_void) -> HBITMAP;
  /// C 937: `compat_hdc = CreateCompatibleDC (hdc);`
  /// C: HDC WINAPI CreateCompatibleDC (HDC hdc);
  pub fn CreateCompatibleDC (hdc: HDC) -> HDC;
  /// C 940/981: `SaveDC (hdc);` and `RestoreDC (hdc, -1);`
  /// C: int WINAPI SaveDC (HDC hdc);
  /// C: BOOL WINAPI RestoreDC (HDC hdc, int nSavedDC);
  pub fn SaveDC (hdc: HDC) -> c_int;
  pub fn RestoreDC (hdc: HDC, nSavedDC: c_int) -> BOOL;
  /// C 949-950 and 969: `SetTextColor (hdc, BLACK_PIX_DEFAULT (f));` / `SetBkColor (hdc, …);`
  /// C: COLORREF WINAPI SetTextColor (HDC hdc, COLORREF color);
  /// C: COLORREF WINAPI SetBkColor (HDC hdc, COLORREF color);
  pub fn SetTextColor (hdc: HDC, color: COLORREF) -> COLORREF;
  pub fn SetBkColor (hdc: HDC, color: COLORREF) -> COLORREF;
}

/// C's `HIWORD` (`<windef.h>`): `#define HIWORD(l) ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))`.
/// C 80's macro body names it and nothing in this slice expands `GET_WHEEL_DELTA_WPARAM`, so the
/// definition's tokens are C's and its real one arrives with the target's `<windows.h>` (a
/// `macro_rules!` body is not resolved until it is expanded - the `include/w32gui.rs` 28 reading).
/// C: #define HIWORD(l) ((WORD) (((DWORD_PTR) (l) >> 16) & 0xffff))
#[cfg (feature = "have_ntgui")]
macro_rules! HIWORD {
  ($l:expr) => { ((($l) as DWORD_PTR >> 16) & 0xffff) as USHORT };
}

/// C 296/298's mask constants (`dispextern.h` 76-77's, reached through C 25's chain) and the SDK's
/// GDI/ROP/region constants this slice names.
/// C: #define SRCCOPY 0x00CC0020
/// C: #define DSTINVERT 0x00550009
/// C: #define PS_SOLID 0
/// C: #define RGN_OR 2
/// C: #define ERROR 0
#[cfg (feature = "have_ntgui")]
pub const SRCCOPY: DWORD = 0x00CC0020;
#[cfg (feature = "have_ntgui")]
pub const DSTINVERT: DWORD = 0x00550009;
#[cfg (feature = "have_ntgui")]
pub const PS_SOLID: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const RGN_OR: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const ERROR: c_int = 0;
/// C 611/619's `GWL_EXSTYLE` (`<winuser.h>`), C 384's `SPI_GETWHEELSCROLLLINES` (`<winuser.h>`)
/// and C 624-641's four `SM_*` indices, three of which C 176-181 guards in this file.
/// C: #define GWL_EXSTYLE (-20)
/// C: #define SPI_GETWHEELSCROLLLINES 0x0068
/// C: #define SM_CXSCREEN 0 / #define SM_CYSCREEN 1
#[cfg (feature = "have_ntgui")]
pub const GWL_EXSTYLE: c_int = -20;
#[cfg (feature = "have_ntgui")]
pub const SPI_GETWHEELSCROLLLINES: UINT = 0x0068;
#[cfg (feature = "have_ntgui")]
pub const SM_CXSCREEN: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SM_CYSCREEN: c_int = 1;

/* C 68-71: fringe bitmaps.  */
#[cfg (feature = "have_ntgui")]
static mut max_fringe_bmp: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut fringe_bmp: *mut HBITMAP = 0 as *mut HBITMAP;

/* C 73-76: "Temporary variables for w32_read_socket.  "  */
#[cfg (feature = "have_ntgui")]
static mut last_mousemove_x: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut last_mousemove_y: c_int = 0;

/* C 78-81: "Define GET_WHEEL_DELTA_WPARAM macro if system headers don't."  The `#ifndef` half is
   the guard (carried, no `cfg`); C's `(short)` cast is the port's `as c_short` (PORTING.md 1.1's
   `c_short` row).  Nothing in this slice expands it.  */
/// C: #define GET_WHEEL_DELTA_WPARAM(wparam) ((short)HIWORD (wparam))
#[cfg (feature = "have_ntgui")]
macro_rules! GET_WHEEL_DELTA_WPARAM {
  ($wparam:expr) => { ((HIWORD ($wparam)) as c_short) };
}

/* C 83-86: "Non-zero means that a HELP_EVENT has been generated since Emacs start."  C's
   tentative definition has no initializer; Rust's needs one, so the port zeroes it.  */
#[cfg (feature = "have_ntgui")]
static mut any_help_event_p: c_int = 0;

/// C 88: `extern unsigned int msh_mousewheel;` - declared here, defined by `w32fns.c`; a
/// declaration stays an `unsafe extern "C"` block (PORTING.md 0.1-3).
/// C: extern unsigned int msh_mousewheel;
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub static mut msh_mousewheel: c_uint;
}

/// C 90-91: the two entry points `w32fns.c` owns.
/// C: extern int w32_codepage_for_font (char *fontname);
/// C: extern HCURSOR w32_load_cursor (LPCTSTR name);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn w32_codepage_for_font (fontname: *mut c_char) -> c_int;
  pub fn w32_load_cursor (name: LPCTSTR) -> HCURSOR;
}

/* C 94-96: "This is display since w32 does not support multiple ones."  Both are tentative
   definitions in C, so both are zero-initialized (PORTING.md 0.1-13's reading of a C object with
   no initializer).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut one_w32_display_info: w32_display_info = unsafe { core::mem::zeroed () };
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut x_display_list: *mut w32_display_info = 0 as *mut w32_display_info;

/* C 98-118: the pre-Windows-2000 `WCRANGE`/`GLYPHSET` pair.  `_WIN32_WINNT < 0x0500 &&
   !defined(MINGW_W64)` is one of the two conditions the evaluator leaves unknown, so **both arms
   are carried**: the arm below is the text, and the empty `#else` arm is the note that C's guard
   has no `#else` at all.  */
/// C: #if _WIN32_WINNT < 0x0500 && !defined(MINGW_W64)
/// C: #endif /* compiling for pre-Win2k */
#[cfg (all (feature = "have_ntgui", feature = "win32_winnt_lt_0500", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagWCRANGE
{
  pub wcLow: WCHAR,
  pub cGlyphs: USHORT,
}
/// C: typedef struct tagWCRANGE { WCHAR wcLow; USHORT cGlyphs; } WCRANGE;
#[cfg (all (feature = "have_ntgui", feature = "win32_winnt_lt_0500", not (feature = "mingw_w64")))]
pub type WCRANGE = tagWCRANGE;
/// C: typedef struct tagGLYPHSET { DWORD cbThis; DWORD flAccel; DWORD cGlyphsSupported; DWORD
/// C: cRanges; WCRANGE ranges[1]; } GLYPHSET;
#[cfg (all (feature = "have_ntgui", feature = "win32_winnt_lt_0500", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagGLYPHSET
{
  pub cbThis: DWORD,
  pub flAccel: DWORD,
  pub cGlyphsSupported: DWORD,
  pub cRanges: DWORD,
  pub ranges: [WCRANGE; 1],
}
#[cfg (all (feature = "have_ntgui", feature = "win32_winnt_lt_0500", not (feature = "mingw_w64")))]
pub type GLYPHSET = tagGLYPHSET;

/* C 120-124: "Dynamic linking to SetLayeredWindowAttribute (only since 2000)."  Both are C
   function pointers, so both are `Option<unsafe extern "system" fn …>` (PORTING.md P-27; the
   `w32fns.rs` 715 spelling of the same shape) and their C NULL is `None`.  */
/// C: static BOOL (WINAPI *pfnSetLayeredWindowAttributes) (HWND, COLORREF, BYTE, DWORD);
#[cfg (feature = "have_ntgui")]
pub type SetLayeredWindowAttributes_proc
  = unsafe extern "system" fn (HWND, COLORREF, BYTE, DWORD) -> BOOL;
#[cfg (feature = "have_ntgui")]
static mut pfnSetLayeredWindowAttributes: Option<SetLayeredWindowAttributes_proc> = None;
/// C: static BOOL (WINAPI *pfnPlgBlt) (HDC, const POINT *, HDC, int, int, int, int, HBITMAP, int,
/// C: int);
#[cfg (feature = "have_ntgui")]
pub type PlgBlt_proc
  = unsafe extern "system" fn (HDC, *const POINT, HDC, c_int, c_int, c_int, c_int, HBITMAP,
                              c_int, c_int) -> BOOL;
#[cfg (feature = "have_ntgui")]
static mut pfnPlgBlt: Option<PlgBlt_proc> = None;

/* C 126-156: "Define required types and constants on systems with older headers lest they be
   absent."  `_WIN32_WINNT < 0x0601` is the evaluator's second unknown condition, and the block is
   a *"define it if the SDK has not got it"* guard - the same shape as the five `#ifndef` ones
   above and as `include/w32term.rs`'s nine, so the port carries C's values and text with the
   guard recorded rather than a `cfg`: the block's *other* arm is `<winuser.h>`'s own declarations
   (C 143-155 is the pre-Win7 spelling of `TOUCHINPUT`/`PTOUCHINPUT`), and this target has no such
   header to take them from - which is also why the unguarded typedef at C 159-160, whose second
   parameter is `PTOUCHINPUT`, resolves here.  The `#if 0` inside the guard (C 134-139) is dead in
   every configuration and is carried as C's own text.  */
/// C: #if _WIN32_WINNT < 0x0601
/// C: #endif /* _WIN32_WINNT < 0x0601 */
#[cfg (feature = "have_ntgui")]
pub const TOUCHEVENTF_DOWN: c_int = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const TOUCHEVENTF_UP: c_int = 0x0004;
/* C 133-139 is `#if 0` ("These are currently unused; prevent compiler warnings.").

#if 0
#define TOUCHEVENTF_MOVE			       0x0002
#define TOUCHEVENTMASKF_CONTACTAREA		       0x0004
#define TOUCHEVENTMASKF_EXTRAINFO		       0x0002
#define TOUCHEVENTMASKF_TIMEFROMSYSTEM		       0x0001
#endif	/* 0 */
*/
/// C: #define WM_TOUCHMOVE 576
#[cfg (feature = "have_ntgui")]
pub const WM_TOUCHMOVE: c_int = 576;
/// C: typedef struct _TOUCHINPUT { LONG x; LONG y; HANDLE hSource; DWORD dwID; DWORD dwFlags;
/// C: DWORD dwMask; DWORD dwTime; ULONG_PTR dwExtraInfo; DWORD cxContact; DWORD cyContact; }
/// C: TOUCHINPUT, *PTOUCHINPUT;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _TOUCHINPUT
{
  pub x: LONG,
  pub y: LONG,
  pub hSource: HANDLE,
  pub dwID: DWORD,
  pub dwFlags: DWORD,
  pub dwMask: DWORD,
  pub dwTime: DWORD,
  pub dwExtraInfo: ULONG_PTR,
  pub cxContact: DWORD,
  pub cyContact: DWORD,
}
#[cfg (feature = "have_ntgui")]
pub type TOUCHINPUT = _TOUCHINPUT;
#[cfg (feature = "have_ntgui")]
pub type PTOUCHINPUT = *mut TOUCHINPUT;

/* C 158-163: "Functions that extract data from touch-screen events."  */
/// C: typedef BOOL (WINAPI * CloseTouchInputHandle_proc) (HANDLE);
/// C: typedef BOOL (WINAPI * GetTouchInputInfo_proc) (HANDLE, UINT, PTOUCHINPUT, int);
/// C: static CloseTouchInputHandle_proc pfnCloseTouchInputHandle;
/// C: static GetTouchInputInfo_proc pfnGetTouchInputInfo;
#[cfg (feature = "have_ntgui")]
pub type CloseTouchInputHandle_proc = unsafe extern "system" fn (HANDLE) -> BOOL;
#[cfg (feature = "have_ntgui")]
pub type GetTouchInputInfo_proc
  = unsafe extern "system" fn (HANDLE, UINT, PTOUCHINPUT, c_int) -> BOOL;
#[cfg (feature = "have_ntgui")]
static mut pfnCloseTouchInputHandle: Option<CloseTouchInputHandle_proc> = None;
#[cfg (feature = "have_ntgui")]
static mut pfnGetTouchInputInfo: Option<GetTouchInputInfo_proc> = None;

/* C 165-181: the five `#ifndef` guards.  Each is "define it if the SDK has not", so the port
   carries the constant with C's value and no `cfg` (the `include/w32term.rs` reading of its own
   nine guards).  */
/// C: #define LWA_ALPHA 0x02
#[cfg (feature = "have_ntgui")]
pub const LWA_ALPHA: DWORD = 0x02;
/// C: #define WS_EX_LAYERED 0x80000
#[cfg (feature = "have_ntgui")]
pub const WS_EX_LAYERED: LONG = 0x80000;
/// C: #define SM_CXVIRTUALSCREEN 78
#[cfg (feature = "have_ntgui")]
pub const SM_CXVIRTUALSCREEN: c_int = 78;
/// C: #define SM_CYVIRTUALSCREEN 79
#[cfg (feature = "have_ntgui")]
pub const SM_CYVIRTUALSCREEN: c_int = 79;

/// C 183: `struct font *w32_system_remap_font;` - a tentative definition.
/// C: struct font *w32_system_remap_font;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_remap_font: *mut font = 0 as *mut font;

/* C 185-196: "The handle of the frame that currently owns the system caret."  C 193-196 carry
   explicit initializers (`= 0`, `= NULL`), the rest are tentative definitions.  */
/// C: HWND w32_system_caret_hwnd; int w32_system_caret_height; int w32_system_caret_x; int
/// C: w32_system_caret_y; struct window *w32_system_caret_window; int
/// C: w32_system_caret_hdr_height; int w32_system_caret_mode_height;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_hwnd: HWND = 0 as HWND;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_height: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_x: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_y: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_window: *mut window = 0 as *mut window;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_hdr_height: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_system_caret_mode_height: c_int = 0;
/// C: DWORD dwWindowsThreadId = 0; HANDLE hWindowsThread = NULL; DWORD dwMainThreadId = 0;
/// C: HANDLE hMainThread = NULL;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut dwWindowsThreadId: DWORD = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut hWindowsThread: HANDLE = core::ptr::null_mut ();
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut dwMainThreadId: DWORD = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut hMainThread: HANDLE = core::ptr::null_mut ();

/* C 198-205: the scroll-bar geometry.  All six are `static`, and C 205's `last_scroll_bar_drag_pos`
   with them; each is a tentative definition.  */
/// C: static int vertical_scroll_bar_min_handle; static int horizontal_scroll_bar_min_handle;
/// C: static int vertical_scroll_bar_top_border; static int vertical_scroll_bar_bottom_border;
/// C: static int horizontal_scroll_bar_left_border; static int horizontal_scroll_bar_right_border;
/// C: static int last_scroll_bar_drag_pos;
#[cfg (feature = "have_ntgui")]
static mut vertical_scroll_bar_min_handle: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut horizontal_scroll_bar_min_handle: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut vertical_scroll_bar_top_border: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut vertical_scroll_bar_bottom_border: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut horizontal_scroll_bar_left_border: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut horizontal_scroll_bar_right_border: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut last_scroll_bar_drag_pos: c_int = 0;

/* C 207-208: "Keyboard code page - may be changed by language-change events."  */
/// C: int w32_keyboard_codepage;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_keyboard_codepage: c_int = 0;

/* C 210-212: "The number of screen lines to scroll for the default mouse-wheel scroll amount,
   given by WHEEL_DELTA."  */
/// C: static UINT w32_wheel_scroll_lines;
#[cfg (feature = "have_ntgui")]
static mut w32_wheel_scroll_lines: UINT = 0;

/* C 214-216: the Cygwin arm of the message file descriptor.  */
/// C: #ifdef CYGWIN / int w32_message_fd = -1; / #endif /* CYGWIN */
#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
#[unsafe(no_mangle)]
pub static mut w32_message_fd: c_int = -1;

/* ----------------------------------------------------------------------------------------------
   C 218-257: the forward-declaration block.  C's `static` prototype + definition pairs in one
   translation unit have no Rust counterpart (PORTING.md 0.1-12), so every line of the block is
   recorded here and each definition keeps C's place in the slice that reaches it.
   ---------------------------------------------------------------------------------------------- */
/* C 218-219: `w32_handle_tab_bar_click` - `static`, defined outside this slice, called by nothing
   here: dropped.  */
/* C 220-221: `w32_handle_tool_bar_click` - `static`, outside this slice: dropped.  */
/* C 222: `w32_define_cursor (Window, Emacs_Cursor)` - `static`, outside this slice: dropped.  */
/* C 224-226: `w32_scroll_bar_clear`, `w32_raise_frame`, `w32_lower_frame` - `static`, outside this
   slice: dropped.  */
/* C 227: `w32_initialize` - `static`, outside this slice: dropped.  */
/* C 228-229: `w32_update_end` (defined at C 790) and `w32_frame_up_to_date` (C 804) - `static`
   prototypes of two functions this slice defines: dropped.  */
/* C 230-232: `w32_clear_frame`, `w32_frame_highlight`, `w32_frame_unhighlight` - outside: dropped.  */
/* C 233-240: `w32_new_focus_frame`, `w32_focus_changed`, `w32_detect_focus_change`,
   `w32_frame_rehighlight`, `w32_frame_rehighlight_1` - outside (the focus section): dropped.  */
/* C 241-243: `w32_draw_hollow_cursor`, `w32_draw_bar_cursor` - outside: dropped.  */
/* C 244-245: `w32_clip_to_row` - `static`, defined outside this slice (C 1739) and **called by
   this slice** at C 938: dropped with its definition named, so the call site reaches a name whose
   definition is the next slice's (the `image.rs` 706 reading of the same situation).  */
/* C 246-247: `my_show_window`, `my_set_window_pos` - outside: dropped.  */
/* C 248-250: `#if 0` / `my_set_focus` / `#endif` - dead in every configuration: dropped.  */
/* C 251-252: `my_set_foreground_window`, `my_destroy_window` - outside: dropped.  */
/* C 253: `w32fullscreen_hook` - outside: dropped.  */
/* C 255-257: `#ifdef GLYPH_DEBUG` / `w32_check_font` / `#endif` - the declaration is dropped with
   the others; the feature itself is reached by this slice's `IF_DEBUG (w32_check_font (…))` sites
   (C 1001-1002 of the `w32_set_cursor_gc` body and its sibling), which keeps C's token.  */

/* C 258-262 is the section banner (`Debugging`): a comment in C, carried as one here.  */

/* C 264-289 is `#if 0`: an unported debugging recorder.  C's own preprocessor never compiles it,
   so the port keeps C's text as a comment (the `cm.rs` 86 reading).

#if 0

/* This is a function useful for recording debugging information about
   the sequence of occurrences in this file.  */

struct record
{
  char *locus;
  int type;
};

struct record event_record[100];

int event_record_index;

record_event (char *locus, int type)
{
  if (event_record_index == countof (event_record))
    event_record_index = 0;

  event_record[event_record_index].locus = locus;
  event_record[event_record_index].type = type;
  event_record_index++;
}

#endif /* 0 */
*/

/* C 291-300: the X-compatibility GC setters.  C's `if (mask & GCForeground)` is a truth test, so
   the port writes C's comparison out; the mask constants are `dispextern.rs`'s (`c_int`), which is
   why each is cast to `unsigned long` - C's own conversion, written out.  */
#[cfg (feature = "have_ntgui")]
unsafe fn XChangeGC (ignore: *mut c_void, gc: *mut Emacs_GC, mask: c_ulong, egc: *mut Emacs_GC)
{
  if (mask & GCForeground as c_ulong) != 0
    { (*gc).foreground = (*egc).foreground; }
  if (mask & GCBackground as c_ulong) != 0
    { (*gc).background = (*egc).background; }
}

/* C 302-310.  C's `xzalloc` returns `void *`, so the port's cast is the one C's assignment from
   `void *` performs implicitly (PORTING.md 1.4).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn XCreateGC (ignore: *mut c_void, wignore: HWND, mask: c_ulong,
                                   egc: *mut Emacs_GC) -> *mut Emacs_GC
{
  let gc: *mut Emacs_GC = xzalloc (core::mem::size_of::<Emacs_GC> ()) as *mut Emacs_GC;

  XChangeGC (ignore, gc, mask, egc);

  return gc;
}

/* C 312-319 is `#if 0` ("unused for now, see w32_draw_image_glyph_string below"): C's own
   preprocessor never compiles it, so the port keeps C's text as a comment.

#if 0	/* unused for now, see w32_draw_image_glyph_string below */
static void
XGetGCValues (void *ignore, XGCValues *gc,
	      unsigned long mask, XGCValues *xgcv)
{
  XChangeGC (ignore, xgcv, mask, gc);
}
#endif
*/

/* C 321-350.  C's `struct w32_output *output;` is written as the declaration's type (PORTING.md
   1.5's zeroed local is not needed here: C assigns it before any read); `FRAME_OUTPUT_DATA`'s
   member is still `*mut c_void` in `frame.rs` (P-72 rule 3), so the one cast the mirror's own
   `FRAME_W32_WINDOW` spells is written here too.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_show_back_buffer (f: *mut frame)
{
  let output: *mut w32_output;
  let raw_dc: HDC;

  output = FRAME_OUTPUT_DATA! (f) as *mut w32_output;

  if (*output).want_paint_buffer == 0 || w32_disable_double_buffering
    { return; }

  enter_crit! ();

  if ! (*output).paint_buffer.is_null ()
    {
      raw_dc = GetDC ((*output).window_desc);

      if raw_dc.is_null ()
	{ emacs_abort (); }

      BitBlt (raw_dc, 0, 0, FRAME_PIXEL_WIDTH (f),
	      FRAME_PIXEL_HEIGHT (f),
	      (*output).paint_dc, 0, 0, SRCCOPY);
      ReleaseDC ((*output).window_desc, raw_dc);

      (*output).paint_buffer_dirty = 0;
    }

  leave_crit! ();
}

/* C 352-375.  C's `NULL` assignments to three pointer members are the port's `core::ptr::null_mut`
   in C's order.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_release_paint_buffer (f: *mut frame)
{
  /* Delete the back buffer so it gets created
     again the next time we ask for the DC.  */

  enter_crit! ();
  if ! (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer.is_null ()
    {
      deselect_palette (f, (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_handle);

      SelectObject ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc,
		    (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc_object);
      ReleaseDC ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).window_desc,
		 (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_handle);
      DeleteDC ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc);
      DeleteObject ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer);

      (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer = core::ptr::null_mut ();
      (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc = core::ptr::null_mut ();
      (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_handle = core::ptr::null_mut ();
    }
  leave_crit! ();
}

/* C 377-388.  C 383's `UINT scroll_lines;` is C's "declared then written by the callee"
   (PORTING.md 1.5), so the port zeroes it; C's `if (ret)` is a truth test on a `BOOL`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_get_mouse_wheel_vertical_delta ()
{
  if os_subtype != OS_SUBTYPE_NT
    { return; }

  let mut scroll_lines: UINT = 0;
  let ret: BOOL = SystemParametersInfo (SPI_GETWHEELSCROLLLINES, 0,
				   &raw mut scroll_lines as *mut c_void, 0);
  if ret != 0
    { w32_wheel_scroll_lines = scroll_lines; }
}

/* C 390-401.  C's `if (rect)` / `SelectClipRgn (hdc, NULL)` are the port's null test and
   `core::ptr::null_mut`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_clip_rectangle (hdc: HDC, rect: *mut RECT)
{
  if ! rect.is_null ()
    {
      let clip_region: HRGN = CreateRectRgnIndirect (rect);
      SelectClipRgn (hdc, clip_region);
      DeleteObject (clip_region);
    }
  else
    { SelectClipRgn (hdc, core::ptr::null_mut ()); }
}

/* C 403-421: "Restore clipping rectangle in S".  C's `s->clip` is `NativeRectangle clip[2]`
   (dispextern.h 1497), whose mirror is the degraded `[c_int; 2]` (`include/dispextern.rs` 1539
   records why), so the port's pointer is the array's address cast to `RECT *` and C's `r + 1` is
   the same pointer one `RECT` further on.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_restore_glyph_string_clip (s: *mut glyph_string)
{
  let r: *mut RECT = (&raw mut (*s).clip) as *mut RECT;
  let n: c_int = (*s).num_clips;

  if n == 1
    { w32_set_clip_rectangle ((*s).hdc as HDC, r); }
  else if n > 1
    {
      let clip1: HRGN = CreateRectRgnIndirect (r);
      let clip2: HRGN = CreateRectRgnIndirect (r.add (1));
      if CombineRgn (clip1, clip1, clip2, RGN_OR) != ERROR
        { SelectClipRgn ((*s).hdc as HDC, clip1); }
      DeleteObject (clip1);
      DeleteObject (clip2);
    }
}

/* C 423-437.  `dpyinfo->resx`/`resy` are `double` in `w32term.h` 142 and `base_res` is an `int`,
   so C's comparisons and divisions mix the two; the port writes C's conversions out
   (`base_res as f64`), and the assignment to an `int` is C's implicit `double` -> `int`
   conversion, also written out (`as c_int`).  C's `*scale_x = *scale_y = 1;` is one statement per
   assignment in C's order (the recorded chained-assignment deviation).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_get_scale_factor (dpyinfo: *mut w32_display_info, scale_x: *mut c_int,
                               scale_y: *mut c_int)
{
  const base_res: c_int = 96;

  *scale_x = 1;
  *scale_y = 1;

  if ! dpyinfo.is_null ()
    {
      if (*dpyinfo).resx > base_res as f64
	{ *scale_x = floor ((*dpyinfo).resx / base_res as f64) as c_int; }
      if (*dpyinfo).resy > base_res as f64
	{ *scale_y = floor ((*dpyinfo).resy / base_res as f64) as c_int; }
    }
}

/* C 868's `FACE_FROM_ID_OR_NULL` and the `FRAME_FACE_CACHE` its C body reads: both are `INLINE`
   in the reference (`frame.h` 1379 and 1656), `include/frame.rs` carries the pair inside
   `pub mod frame_face_accessors` (a module it does not glob-import), and `include/dispextern.rs`
   carries `FACE_FROM_ID`'s `eassert` form but not the NULL-returning sibling - so the unit
   declares its own copy at the use site, exactly as `xdisp.rs` 963-1010 and `alloc.rs` 3456-3463
   do (the `term.rs`/`window.rs` copies are the same four lines, and `## Deviations` records the
   sharing).  The record's own `face_cache` is **not** repeated here: the mirror's top-level copy
   is the one this unit reads (`include/dispextern.rs` 2187, behind `dispextern_face_records`),
   which is the arm the two face records live behind.  */
/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`.  The `frame` mirror carries
/// the field as `*mut c_void` (the record has no mirror), so the cast is the use-site conversion
/// P-72 rule 3.
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_FACE_CACHE {
  ($f:expr) => { (*$f).face_cache as *mut face_cache };
}

/* Two of C's reads are packed 1-bit members, and the port spells each through a macro **named
   after the member** so that C's own token survives (`frame.rs` 310's `garbaged`, `dispnew.rs`
   396's `window_bits!`); both live in units this file does not include (C's `w32term.c` includes
   neither `frame.c` nor `dispnew.c`), so each is declared here at its use site and the ledger
   records the gap - the `dispnew.rs` 450 note for its own two copies.  */
/// C 826: `!f->garbaged` - `frame.h` 398's `bool_bf garbaged : 1`, one of the two measured 4-byte
/// units (`include/frame.rs` names the shift; C's `FRAME_GARBAGED_P` is `(f)->garbaged`).
/// C: #define FRAME_GARBAGED_P(f) (f)->garbaged
#[cfg (feature = "have_ntgui")]
macro_rules! garbaged {
  ($f:expr) => { (*$f).bits1 >> FRAME_GARBAGED_SHIFT & 1 != 0 };
}
/// C 851: `!w->pseudo_window_p` - `window.h` 438's `bool_bf pseudo_window_p : 1`, bit 11 of
/// `window::bits` (`include/window.rs` 195 names the shift).
/// C: #define WINDOW_PSEUDO_P(w) ((w)->pseudo_window_p)
#[cfg (feature = "have_ntgui")]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr) => { ((*($w)).bits >> WINDOW_PSEUDO_WINDOW_P_SHIFT) & 1 != 0 };
}

/// C: frame.h 1656: `INLINE struct face *FACE_FROM_ID_OR_NULL (struct frame *f, int id);`
#[cfg (feature = "have_ntgui")]
unsafe fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = (*FRAME_FACE_CACHE! (f)).used;
  eassume! (0 <= used);
  return if 0 <= id && id < used
    { *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize) }
    else { core::ptr::null_mut () };
}

/* Five functions this slice calls have no prototype in the port's mirrors: `dispextern.h` is
   where C declares all five (C 25's chain reaches it through `w32term.h` 22), and
   `include/dispextern.rs` does not carry them yet, so the unit declares them at the use sites the
   way `androidterm.rs` 160-175 and `dispnew.rs` 257-259 do (P-72 rule 4: a mirror must reach what
   it needs through its own `include!`s).  Each line names the C header it comes from, and
   `docs/ports/w32term.md` records the gap for the integrator.  `gui_define_fringe_bitmap` is the
   one of the five whose *definition* already exists in this tree (`abi_shim.rs` 2240 exports it
   for the two layers that call it), so the declaration keeps C's prototype and no stand-in.  */
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  /// C 478: `get_glyph_string_clip_rect (s, &w32_string_clip);`
  /// C: dispextern.h 3663: `extern void get_glyph_string_clip_rect (struct glyph_string *s,
  /// C: NativeRectangle *r);`
  pub fn get_glyph_string_clip_rect (s: *mut glyph_string, r: *mut RECT);
  /// C 481: `if (!gui_intersect_rectangles (&wave_clip, &string_clip, &final_clip))`
  /// C: dispextern.h 3676: `extern bool gui_intersect_rectangles (const Emacs_Rectangle *r1, const
  /// C: Emacs_Rectangle *r2, Emacs_Rectangle *out);`
  pub fn gui_intersect_rectangles (r1: *const Emacs_Rectangle, r2: *const Emacs_Rectangle,
                                   out: *mut Emacs_Rectangle) -> bool;
  /// C 935: `gui_define_fringe_bitmap (f, p->which);`
  /// C: dispextern.h 3711: `void gui_define_fringe_bitmap (struct frame *f, int n);`
  pub fn gui_define_fringe_bitmap (f: *mut frame, n: c_int);
  /// C 808 and 830: `!buffer_flipping_blocked_p ()`
  /// C: dispextern.h 3719: `extern bool buffer_flipping_blocked_p (void);`
  pub fn buffer_flipping_blocked_p () -> bool;
  /// C 868: `lookup_basic_face (NULL, f, INTERNAL_BORDER_FACE_ID)`
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);`
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
}

/* C 439-448 is the ASCII-wave picture above `w32_draw_underwave`: a comment in C, carried as one
   here.
   Draw a wavy line under S. The wave fills wave_height pixels from y0.

                    x0         wave_length = 2
                                 --
                y0   *   *   *   *   *
                     |* * * * * * * * *
    wave_height = 3  | *   *   *   *
*/

/* C 450-515.  The twelve uninitialized locals of C 459 and the two records of C 461-462 are
   zeroed and named (PORTING.md 1.5), and `CONVERT_TO_EMACS_RECT`/`CONVERT_FROM_EMACS_RECT` are
   `include/w32gui.rs`'s macros (C's own uses of the same two).  C's four comma expressions
   (`x1  = x2, y1 = y2`, `x2 += dx, y2 = …`) are one statement per assignment in C's order, and
   `odd = !odd` on an `int` is the port's `(odd == 0) as c_int` (the recorded deviations).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_underwave (s: *mut glyph_string, color: COLORREF)
{
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO! ((*s).f);

  let mut scale_x: c_int = 0;
  let mut scale_y: c_int = 0;
  w32_get_scale_factor (dpyinfo, &raw mut scale_x, &raw mut scale_y);

  let wave_height: c_int = 3 * scale_y;
  let wave_length: c_int = 2 * scale_x;
  let thickness: c_int = scale_y;
  let mut dx: c_int = 0;
  let mut dy: c_int = 0;
  let mut x0: c_int = 0;
  let mut y0: c_int = 0;
  let mut width: c_int = 0;
  let mut x1: c_int = 0;
  let mut y1: c_int = 0;
  let mut x2: c_int = 0;
  let mut y2: c_int = 0;
  let mut odd: c_int = 0;
  let mut xmax: c_int = 0;
  let mut wave_clip: Emacs_Rectangle = core::mem::zeroed ();
  let mut string_clip: Emacs_Rectangle = core::mem::zeroed ();
  let mut final_clip: Emacs_Rectangle = core::mem::zeroed ();
  let mut w32_final_clip: RECT = core::mem::zeroed ();
  let mut w32_string_clip: RECT = core::mem::zeroed ();
  let mut hp: HPEN = 0 as HPEN;
  let mut oldhp: HPEN = 0 as HPEN;

  dx = wave_length;
  dy = wave_height - 1;
  x0 = (*s).x;
  y0 = (*s).ybase + wave_height / 2 - scale_y;
  width = (*s).width;
  xmax = x0 + width;

  /* Find and set clipping rectangle */

  wave_clip.x = x0;
  wave_clip.y = y0;
  wave_clip.width = width as c_uint;
  wave_clip.height = wave_height as c_uint;

  get_glyph_string_clip_rect (s, &raw mut w32_string_clip);
  CONVERT_TO_EMACS_RECT! (string_clip, w32_string_clip);

  if ! gui_intersect_rectangles (&wave_clip, &string_clip, &raw mut final_clip)
    { return; }

  hp = CreatePen (PS_SOLID, thickness, color);
  oldhp = SelectObject ((*s).hdc as HDC, hp);
  CONVERT_FROM_EMACS_RECT! (final_clip, w32_final_clip);
  w32_set_clip_rectangle ((*s).hdc as HDC, &raw mut w32_final_clip);

  /* Draw the waves */

  x1 = x0 - (x0 % dx);
  x2 = x1 + dx;
  odd = (x1/dx) % 2;
  y1 = y0;
  y2 = y0;

  if odd != 0
    { y1 += dy; }
  else
    { y2 += dy; }

  MoveToEx ((*s).hdc as HDC, x1, y1, core::ptr::null_mut ());

  while (x1 <= xmax)
    {
      LineTo ((*s).hdc as HDC, x2, y2);
      x1  = x2; y1 = y2;
      x2 += dx; y2 = y0 + odd*dy;
      odd = (odd == 0) as c_int;
    }

  /* Restore previous pen and clipping rectangle(s) */
  w32_restore_glyph_string_clip (s);
  SelectObject ((*s).hdc as HDC, oldhp);
  DeleteObject (hp);
}

/* C 517-542: "Draw a hollow rectangle at the specified position."  C's `SelectObject` results are
   `HGDIOBJ` and the three concrete handle types are aliases of it (the `<windows.h>` section
   above), so C's implicit conversions need no cast.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_rectangle (hdc: HDC, gc: *mut Emacs_GC, x: c_int, y: c_int,
                             width: c_int, height: c_int)
{
  let mut hb: HBRUSH = 0 as HBRUSH;
  let mut oldhb: HBRUSH = 0 as HBRUSH;
  let mut hp: HPEN = 0 as HPEN;
  let mut oldhp: HPEN = 0 as HPEN;

  hb = CreateSolidBrush ((*gc).background as COLORREF);
  hp = CreatePen (PS_SOLID, 0, (*gc).foreground as COLORREF);
  oldhb = SelectObject (hdc, hb);
  oldhp = SelectObject (hdc, hp);

  /* We enlarge WIDTH and HEIGHT by 1 to be bug-compatible to the
     brain-dead design of XDrawRectangle, which draws a rectangle that
     is 1 pixel wider and higher than its arguments WIDTH and HEIGHT.
     This allows us to keep the code that calls this function similar
     to the corresponding code in xterm.c.  For the details, see
     https://lists.gnu.org/r/emacs-devel/2014-10/msg00546.html.  */
  Rectangle (hdc, x, y, x + width + 1, y + height + 1);

  SelectObject (hdc, oldhb);
  SelectObject (hdc, oldhp);
  DeleteObject (hb);
  DeleteObject (hp);
}

/* C 544-553: "Draw a filled rectangle at the specified position."  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_fill_rect (f: *mut frame, hdc: HDC, pix: COLORREF, lprect: *mut RECT)
{
  let mut hb: HBRUSH = 0 as HBRUSH;

  hb = CreateSolidBrush (pix);
  FillRect (hdc, lprect, hb);
  DeleteObject (hb);
}

/* C 555-571.  C's `HDC hdc = get_frame_dc (f);` is kept as the declaration's initializer.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_clear_window (f: *mut frame)
{
  let mut rect: RECT = core::mem::zeroed ();
  let hdc: HDC = get_frame_dc (f);

  /* Under certain conditions, this can be called at startup with
     a console frame pointer before the GUI frame is created. An HDC
     of 0 indicates this. */
  if ! hdc.is_null ()
    {
      GetClientRect (FRAME_W32_WINDOW! (f), &raw mut rect);
      w32_clear_rect! (f, hdc, &raw mut rect);
    }

  release_frame_dc (f, hdc);
}

/// C 573-574: `#define OPAQUE_FRAME 255`.
/// C: #define OPAQUE_FRAME 255
#[cfg (feature = "have_ntgui")]
pub const OPAQUE_FRAME: c_int = 255;

/* C 576-619.  `FLOATP`/`FIXNUMP` are `lisp.rs`'s predicates; C's `XFLOAT_DATA`/`XFIXNUM` reads and
   its `double` arithmetic are written as C has them, and the `BYTE opac = alpha * OPAQUE_FRAME;`
   narrowing is C's implicit conversion written out (`as BYTE`).  C 610's `GetWindowLong`/C 618's
   `SetWindowLong` are the `<windows.h>` entry points declared above; `ex_style` is a `LONG`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_frame_alpha (f: *mut frame)
{
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO! (f);
  let mut alpha: c_double = 1.0;
  let mut alpha_min: c_double = 1.0;
  let opac: BYTE;
  let mut ex_style: LONG;
  let window: HWND = FRAME_W32_WINDOW! (f);

  /* Older versions of Windows do not support transparency.  */
  if pfnSetLayeredWindowAttributes.is_none ()
    { return; }

  if (*dpyinfo).w32_focus_frame == f
    { alpha = (*f).alpha[0]; }
  else
    { alpha = (*f).alpha[1]; }

  if FLOATP (Vframe_alpha_lower_limit)
    { alpha_min = XFLOAT_DATA (Vframe_alpha_lower_limit); }
  else if FIXNUMP (Vframe_alpha_lower_limit)
    { alpha_min = (XFIXNUM (Vframe_alpha_lower_limit)) as c_double / 100.0; }

  if alpha < 0.0
    { return; }
  else if alpha > 1.0
    { alpha = 1.0; }
  else if alpha < alpha_min && alpha_min <= 1.0
    { alpha = alpha_min; }

  opac = (alpha * OPAQUE_FRAME as c_double) as BYTE;

  ex_style = GetWindowLong (window, GWL_EXSTYLE);

  if opac == OPAQUE_FRAME as BYTE
    { ex_style &= !WS_EX_LAYERED; }
  else
    { ex_style |= WS_EX_LAYERED; }

  SetWindowLong (window, GWL_EXSTYLE, ex_style);

  if opac != OPAQUE_FRAME as BYTE
    { pfnSetLayeredWindowAttributes.unwrap () (window, 0, opac, LWA_ALPHA); }
}

/* C 621-643.  Both return C's `GetSystemMetrics` reading and keep C's fallback arm.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_display_pixel_height (dpyinfo: *mut w32_display_info) -> c_int
{
  let mut pixels: c_int = GetSystemMetrics (SM_CYVIRTUALSCREEN);

  if pixels == 0
    /* Fallback for Windows 95 or NT 4.0.  */
    { pixels = GetSystemMetrics (SM_CYSCREEN); }

  return pixels;
}

#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_display_pixel_width (dpyinfo: *mut w32_display_info) -> c_int
{
  let mut pixels: c_int = GetSystemMetrics (SM_CXVIRTUALSCREEN);

  if pixels == 0
    /* Fallback for Windows 95 or NT 4.0.  */
    { pixels = GetSystemMetrics (SM_CXSCREEN); }

  return pixels;
}

/* C 645-649 is the section banner (`Starting and ending an update`): a comment in C, carried as
   one here.  */

/* C 651-670: "Start an update of frame F. …"  C's `if (display_info->regen_palette)` is an
   `int` test, and `FALSE` is `<windows.h>`'s (declared above).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_update_begin (f: *mut frame)
{
  let display_info: *mut w32_display_info = FRAME_DISPLAY_INFO! (f);

  if ! FRAME_W32_P (f)
    { return; }

  /* Regenerate display palette before drawing if list of requested
     colors has changed. */
  if (*display_info).regen_palette != 0
  {
    w32_regenerate_palette (f);
    (*display_info).regen_palette = FALSE;
  }
}

/* C 672-685: "Start update of window W."  C's third `&&` operand is the `HWND` truth test.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_update_window_begin (w: *mut window)
{
  /* Hide the system caret during an update.  */
  if w32_use_visible_system_caret && ! w32_system_caret_hwnd.is_null ()
      && w == w32_system_caret_window
    {
      SendMessageTimeout (w32_system_caret_hwnd, WM_EMACS_HIDE_CARET as UINT, 0, 0,
			  0, 6000, core::ptr::null_mut ());
    }
}

/* C 687-710: "Draw a vertical window border from (x,y0) to (x,y1)".  `face->foreground` and
   `FRAME_FOREGROUND_PIXEL` are `unsigned long` while `w32_fill_rect`'s parameter is a `COLORREF`
   (`uint32_t`), so C's narrowing is written out at both call sites.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_vertical_window_border (w: *mut window, x: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut r: RECT = core::mem::zeroed ();
  let hdc: HDC;
  let face: *mut face;

  r.left = x;
  r.right = x + 1;
  r.top = y0;
  r.bottom = y1;

  hdc = get_frame_dc (f);
  face = FACE_FROM_ID_OR_NULL (f, VERTICAL_BORDER_FACE_ID);
  if ! face.is_null ()
    { w32_fill_rect (f, hdc, (*face).foreground as COLORREF, &raw mut r); }
  else
    { w32_fill_rect (f, hdc, FRAME_FOREGROUND_PIXEL (f) as COLORREF, &raw mut r); }

  release_frame_dc (f, hdc);
}

/* C 712-760: "Draw a window divider from (x0, y0) to (x1, y1)".
   C's three `?:` expressions on pointers are the port's `if`/`else` in C's order, and the six
   `w32_fill_area_abs` calls are `include/w32term.rs`'s macro (C 641-655's).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_window_divider (w: *mut window, x0: c_int, x1: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let hdc: HDC = get_frame_dc (f);
  let face: *mut face = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FACE_ID);
  let face_first: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);
  let face_last: *mut face
    = FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);
  let color: c_ulong = if ! face.is_null () { (*face).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let color_first: c_ulong = if ! face_first.is_null ()
			       { (*face_first).foreground }
			       else { FRAME_FOREGROUND_PIXEL (f) };
  let color_last: c_ulong = if ! face_last.is_null ()
			      { (*face_last).foreground }
			      else { FRAME_FOREGROUND_PIXEL (f) };

  if (y1 - y0 > x1 - x0) && (x1 - x0 >= 3)
    /* A vertical divider, at least three pixels wide: Draw first and
       last pixels differently.  */
    {
      w32_fill_area_abs! (f, hdc, color_first, x0, y0, x0 + 1, y1);
      w32_fill_area_abs! (f, hdc, color, x0 + 1, y0, x1 - 1, y1);
      w32_fill_area_abs! (f, hdc, color_last, x1 - 1, y0, x1, y1);
    }
  else if (x1 - x0 > y1 - y0) && (y1 - y0 >= 3)
    /* A horizontal divider, at least three pixels high: Draw first and
       last pixels differently.  */
    {
      w32_fill_area_abs! (f, hdc, color_first, x0, y0, x1, y0 + 1);
      w32_fill_area_abs! (f, hdc, color, x0, y0 + 1, x1, y1 - 1);
      w32_fill_area_abs! (f, hdc, color_last, x0, y1 - 1, x1, y1);
    }
  else
    /* In any other case do not draw the first and last pixels
       differently.  */
    { w32_fill_area_abs! (f, hdc, color, x0, y0, x1, y1); }

  release_frame_dc (f, hdc);
}

/* C 762-783: "End update of window W. …"  The third `&&` operand is the `HWND` truth test again.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_update_window_end (w: *mut window, cursor_on_p: bool,
                                mouse_face_overwritten_p: bool)
{
  /* Unhide the caret.  This won't actually show the cursor, unless it
     was visible before the corresponding call to HideCaret in
     w32_update_window_begin.  */
  if w32_use_visible_system_caret && ! w32_system_caret_hwnd.is_null ()
      && w == w32_system_caret_window
    {
      SendMessageTimeout (w32_system_caret_hwnd, WM_EMACS_SHOW_CARET as UINT, 0, 0,
			  0, 6000, core::ptr::null_mut ());
    }
}

/* C 785-797: "End update of frame F. …"  `MOUSE_HL_INFO` is `frame.rs`'s macro (C's own, from
   `frame.h`), and C's `false` assignment to its `bool` member is the port's `false`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_update_end (f: *mut frame)
{
  if ! FRAME_W32_P (f)
    { return; }

  /* Mouse highlight may be displayed again.  C's `->mouse_face_defer` is one of
     `Mouse_HLInfo`'s three `bool_bf : 1` members, which the mirror stores as `bits` with the
     three masks below (`include/dispextern.rs` 1945-1947; the `reset_mouse_highlight` reading at
     3486-3494 binds C's three names to the `false`s C assigns them), so C's `= false` write is
     the mask clear.  */
  let mouse_face_defer = false;
  let _ = mouse_face_defer;
  (*MOUSE_HL_INFO! (f)).bits &= !MOUSE_FACE_DEFER;
}

/* C 799-811: "This function is called from various places in xdisp.c whenever a complete update
   has been performed."  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_up_to_date (f: *mut frame)
{
  FRAME_MOUSE_UPDATE! (f);

  if ! buffer_flipping_blocked_p ()
      && (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_dirty != 0
    { w32_show_back_buffer (f); }
}

/* C 813-818.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_buffer_flipping_unblocked_hook (f: *mut frame)
{
  if (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_dirty != 0
    { w32_show_back_buffer (f); }
}

/* C 820-832: "Flip buffers on F if drawing has happened. …"  The three pointer/int operands of
   C's `&&` chain are the port's null tests and `!= 0`; `f->garbaged` is the `bool` member C's
   `!` negates.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_flip_buffers_if_dirty (f: *mut frame)
{
  if FRAME_W32_P (f)	/* do nothing in TTY frames */
      && ! (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer.is_null ()
      && (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_dirty != 0
      && ! garbaged! (f) && ! buffer_flipping_blocked_p ()
    { w32_show_back_buffer (f); }
}

/* C 834-889: "Draw truncation mark bitmaps, …".  C 858-859's and C 868-869's two comma
   expressions are written as the statements C's effects require, in C's order and behind C's own
   short-circuit tests (the `haikuterm.rs` 888 reading of the same shape); C's `max` is
   `core::cmp::max` (which is what C's `max` macro is).  Three of the reads are `bool_bf` members,
   so each is the mirror's own accessor rather than C's `->` (PORTING.md 1.3): `->mode_line_p`,
   `->full_width_p` and `->redraw_fringe_bitmaps_p` are `glyph_row_bits!` (`include/dispextern.rs`
   965/988/990), and `w->pseudo_window_p` is `window.h`'s own `WINDOW_PSEUDO_P`
   (`include/window.rs` 391-392).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_after_update_window_line (w: *mut window, desired_row: *mut glyph_row)
{
  let mut f: *mut frame = 0 as *mut frame;
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  eassert! (w);

  if ! glyph_row_bits! (mode_line_p, desired_row) && ! window_bits! (pseudo_window_p, w)
    { glyph_row_bits! (redraw_fringe_bitmaps_p, desired_row, true); }

  /* When a window has disappeared, make sure that no rest of
     full-width rows stays visible in the internal border.  Could
     check here if updated window is the leftmost/rightmost window,
     but I guess it's not worth doing since vertically split windows
     are almost never used, internal border is rarely set, and the
     overhead is very small.  */
  if windows_or_buffers_changed != 0
      && glyph_row_bits! (full_width_p, desired_row)
      && {
	   f = XFRAME ((*w).frame);
	   width = FRAME_INTERNAL_BORDER_WIDTH (f);
	   width != 0
	 }
      && {
	   height = (*desired_row).visible_height;
	   height > 0
	 }
    {
      let y: c_int = WINDOW_TO_FRAME_PIXEL_Y! (w, core::cmp::max (0, (*desired_row).y));
      let face_id: c_int =
	if ! NILP (Vface_remapping_alist)
	{ lookup_basic_face (core::ptr::null_mut (), f, INTERNAL_BORDER_FACE_ID) }
	 else
	{ INTERNAL_BORDER_FACE_ID };
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

      block_input ();

      let hdc: HDC = get_frame_dc (f);
      if ! face.is_null ()
	{
	  /* Fill border with internal border face.  */
	  let color: c_ulong = (*face).background;

	  w32_fill_area! (f, hdc, color as COLORREF, 0, y, width, height);
	  w32_fill_area! (f, hdc, color as COLORREF, FRAME_PIXEL_WIDTH (f) - width,
			 y, width, height);
	}
      else
	{
	  w32_clear_area! (f, hdc, 0, y, width, height);
	  w32_clear_area! (f, hdc, FRAME_PIXEL_WIDTH (f) - width,
			  y, width, height);
	}
      release_frame_dc (f, hdc);

      unblock_input ();
    }
}

/* C 891-987: "Draw the bitmap WHICH in one of the left or right fringes of window W. …"
   `p->bx`, `p->which`, `p->overlay_p` and `p->cursor_p` are `draw_fringe_bitmap_params`'s (the
   first two plain members, the last two bitfields `include/dispextern.rs` records as its `flags`
   storage unit); `fringe_bmp[p->which]` is C's array read of this file's own object, and the
   three `BitBlt` calls' third literal (C 959's `0x2E064A`) is kept as C's number.

   **The record's name is the module path, not C's bare one.**  `include/dispextern.rs` carries
   the five records of C 2159-3200 (`bidi_it`, `draw_fringe_bitmap_params`, `composition_it`,
   `it`, `redisplay_interface`) inside `pub mod dispextern_unit_names`, "because every unit that
   includes the mirror declares its own copy of each" - and this unit is no C record's owner, so
   it names the type the way `abi_shim.rs` 2087 does, which is the measured reading there ("the
   unqualified name is `E0425` in the flip arm").  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_fringe_bitmap (w: *mut window, row: *mut glyph_row,
				  p: *mut dispextern_unit_names::draw_fringe_bitmap_params)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let hdc: HDC;
  let face: *mut face = (*p).face;

  hdc = get_frame_dc (f);

  /* Must clip because of partially visible lines.  */
  w32_clip_to_row (w, row, ANY_AREA, hdc);

  if (*p).bx >= 0 && (*p).overlay_p == 0
    { w32_fill_area! (f, hdc, (*face).background as COLORREF,
		   (*p).bx, (*p).by, (*p).nx, (*p).ny); }

  if (*p).which != 0
      && (*p).which < max_fringe_bmp
      && (*p).which < max_used_fringe_bitmap
    {
      let pixmap: HBITMAP = *fringe_bmp.add ((*p).which as usize);
      let compat_hdc: HDC;
      let horig_obj: HANDLE;

      if (*fringe_bmp.add ((*p).which as usize)).is_null ()
	{
	  /* This fringe bitmap is known to fringe.c, but lacks the
	     HBITMAP data which shadows that bitmap.  This is typical
	     to define-fringe-bitmap being called when the selected
	     frame was not a GUI frame, for example, when packages
	     that define fringe bitmaps are loaded by a daemon Emacs.
	     Create the missing HBITMAP now.  */
	  gui_define_fringe_bitmap (f, (*p).which);
	}

      compat_hdc = CreateCompatibleDC (hdc);

      SaveDC (hdc);

      horig_obj = SelectObject (compat_hdc, pixmap);

      /* Paint overlays transparently.  */
      if (*p).overlay_p != 0
	{
	  let mut h_brush: HBRUSH = 0 as HBRUSH;
	  let mut h_orig_brush: HBRUSH = 0 as HBRUSH;

	  SetTextColor (hdc, BLACK_PIX_DEFAULT! (f) as COLORREF);
	  SetBkColor (hdc, WHITE_PIX_DEFAULT! (f) as COLORREF);
	  h_brush = CreateSolidBrush ((*face).foreground as COLORREF);
	  h_orig_brush = SelectObject (hdc, h_brush);

	  BitBlt (hdc, (*p).x, (*p).y, (*p).wd, (*p).h,
		  compat_hdc, 0, (*p).dh,
		  DSTINVERT);
	  BitBlt (hdc, (*p).x, (*p).y, (*p).wd, (*p).h,
		  compat_hdc, 0, (*p).dh,
		  0x2E064A);
	  BitBlt (hdc, (*p).x, (*p).y, (*p).wd, (*p).h,
		  compat_hdc, 0, (*p).dh,
		  DSTINVERT);

	  SelectObject (hdc, h_orig_brush);
	  DeleteObject (h_brush);
	}
      else
	{
	  SetTextColor (hdc, (*face).background as COLORREF);
	  SetBkColor (hdc, (if (*p).cursor_p != 0
			    { (*(*f).output_data.w32 as *mut w32_output).cursor_pixel }
			    else { (*face).foreground as COLORREF }));

	  BitBlt (hdc, (*p).x, (*p).y, (*p).wd, (*p).h,
		  compat_hdc, 0, (*p).dh,
		  SRCCOPY);
	}

      SelectObject (compat_hdc, horig_obj);
      DeleteDC (compat_hdc);
      RestoreDC (hdc, -1);
    }

  w32_set_clip_rectangle (hdc, core::ptr::null_mut ());

  release_frame_dc (f, hdc);
}

/* C 989-1002: the last item of this slice.  C's `fringe_bmp` reallocation is `xrealloc`'s
   `void *` assignment written out (`as *mut HBITMAP`), and `fringe_bmp[i++] = 0` is C's
   post-increment loop, which the port keeps as the index then the step.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_define_fringe_bitmap (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)
{
  if which >= max_fringe_bmp
    {
      let mut i: c_int = max_fringe_bmp;
      max_fringe_bmp = which + 20;
      fringe_bmp = xrealloc (fringe_bmp as *mut c_void,
			     max_fringe_bmp as usize * core::mem::size_of::<HBITMAP> ())
	as *mut HBITMAP;
      while i < max_fringe_bmp
	{
	  *fringe_bmp.add (i as usize) = 0 as HBITMAP;
	  i += 1;
	}
    }

  *fringe_bmp.add (which as usize) = CreateBitmap (wd, h, 1, 1, bits as *const c_void);
}
