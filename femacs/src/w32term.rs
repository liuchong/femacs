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
/* C 20's AH_BOTTOM: `#include <conf_post.h>` (`DebPrint`, C 1807/2145/2353's uses; the
   `w32dwrite.rs` 415-417 reading of the same AH_BOTTOM).  */
#[cfg (feature = "have_ntgui")]
include!("include/conf_post.rs");

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

  if w32_output_bits! (want_paint_buffer, output) == 0 || w32_disable_double_buffering
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

      w32_output_bits! (paint_buffer_dirty, output, 0);
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
  /* C 418's `bool_bf phys_cursor_on_p : 1`, which slice 17's `w32_draw_window_cursor` writes
     (C 6677); the shift is the mirror's `WINDOW_PHYS_CURSOR_ON_P_SHIFT` (window.rs 191).  */
  (phys_cursor_on_p, $w:expr) => { ((*($w)).bits >> WINDOW_PHYS_CURSOR_ON_P_SHIFT) & 1 != 0 };
  (phys_cursor_on_p, $w:expr, $v:expr) => {
    (*($w)).bits = ((*($w)).bits & ! (1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT))
		   | (((($v) as u32) & 1) << WINDOW_PHYS_CURSOR_ON_P_SHIFT) };
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
      && w32_output_bits! (paint_buffer_dirty, FRAME_OUTPUT_DATA! (f)) != 0
    { w32_show_back_buffer (f); }
}

/* C 813-818.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_buffer_flipping_unblocked_hook (f: *mut frame)
{
  if w32_output_bits! (paint_buffer_dirty, FRAME_OUTPUT_DATA! (f)) != 0
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
      && w32_output_bits! (paint_buffer_dirty, FRAME_OUTPUT_DATA! (f)) != 0
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
			    { (*( (*f).output_data.w32 as *mut w32_output )).cursor_pixel }
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

/* ==============================================================================================
   **Slice 2: C 1004-1949.**  The `Display Iterator` page (its banner is C 1015-1017), the glyph
   string drawing that fills it, and the relief/box drawing C 1709-1949 that follows - down to
   `w32_draw_glyph_string_box`'s closing brace at **C 1949**.  The ledger `docs/ports/w32term.md`
   records the boundary and its neighbours: the next definition is **`w32_image_rotations_p`**
   (C 1952-1955), the first item of the image page, which the next lane's slice opens.

   Everything below carries the same readings the file note above records - the file gate is
   `feature = "have_ntgui"`, both arms of every `#if` are carried, the `#if 0` regions are C's own
   text inside a comment, the `<windows.h>` declarations are spelled (this machine has no
   `windows.h` to measure against) and every `#[repr(C)]` record stays **UNVERIFIED** here.  The
   ledger's `## Evidence` is the frozen `build-tu`, the `extract-abi --check` reading and the two
   inventory gates.
   ============================================================================================== */

/* ----------------------------------------------------------------------------------------------
   The `<windows.h>` / `<wingdi.h>` names **this slice** adds to the section C 25's chain opened
   above, at the same declaration site and by the same reading (P-88 clause 1: the site is the
   chain's include line, and no file is added under `femacs/src/include/`).  `PALETTERGB` and the
   three `Get*Value` accessors are the macros `include/w32term.rs`'s `BLACK_PIX_DEFAULT` /
   `WHITE_PIX_DEFAULT` already *expand to* (the ledger's owed 7); this slice is the first that
   expands them, so the pair stands here.  No measurement was possible: every value is
   `<wingdi.h>`'s own text and every scalar PORTING.md 1.1's fixed width.
   ---------------------------------------------------------------------------------------------- */

/// C 1637-1659's `PALETTERGB`/`Get*Value` family needs `<windef.h>`'s `WORD` (`typedef
/// unsigned short WORD;`); the unit's type section above carries the other scalar typedefs.
/// C: typedef unsigned short WORD;
#[cfg (feature = "have_ntgui")]
pub type WORD = c_ushort;

/// C 1637-1639 and 1644-1646: `RGB` and `PALETTERGB` (`<wingdi.h>`), the two macros the four
/// `PALETTERGB (…)` calls of `w32_alloc_lighter_color` expand through - and the ones
/// `include/w32term.rs`'s `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` bodies name (owed 7).
/// C: #define RGB(r,g,b) ((COLORREF)(((BYTE)(r)|((WORD)((BYTE)(g))<<8))|(((DWORD)(BYTE)(b))<<16)))
/// C: #define PALETTERGB(r,g,b) (0x02000000 | RGB(r,g,b))
#[cfg (feature = "have_ntgui")]
macro_rules! RGB {
  ($r:expr, $g:expr, $b:expr) =>
    { ((((($r) as BYTE) as DWORD) | ((((($g) as BYTE) as WORD) as DWORD) << 8))
       | (((($b) as BYTE) as DWORD) << 16)) as COLORREF };
}
#[cfg (feature = "have_ntgui")]
macro_rules! PALETTERGB {
  ($r:expr, $g:expr, $b:expr) => { 0x02000000 | RGB! ($r, $g, $b) };
}

/// C 1637-1659's three channel accessors (`<wingdi.h>`'s `GetRValue`/`GetGValue`/`GetBValue`).
/// C: #define GetRValue(rgb) ((BYTE)(rgb))
/// C: #define GetGValue(rgb) ((BYTE)(((WORD)(rgb)) >> 8))
/// C: #define GetBValue(rgb) ((BYTE)((rgb) >> 16))
#[cfg (feature = "have_ntgui")]
macro_rules! GetRValue { ($rgb:expr) => { (($rgb) as BYTE) }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GetGValue { ($rgb:expr) => { (((($rgb) as WORD) >> 8) as BYTE) }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GetBValue { ($rgb:expr) => { ((($rgb) >> 16) as BYTE) }; }

/// C's font handle (`<wingdi.h>`'s `DECLARE_HANDLE (HFONT)`, i.e. a `void *` object handle);
/// `w32_draw_glyph_string_foreground` and `w32_draw_glyphless_glyph_string_foreground` walk it
/// through `FONT_HANDLE` (`include/w32font.rs` 80, whose `hfont` member names this type) and
/// hand it to `SelectObject`.  It is an alias of `HGDIOBJ` so C's implicit conversion at those
/// call sites needs no cast, the `HBITMAP`/`HBRUSH` reading of the section above.
/// C: DECLARE_HANDLE (HFONT);
#[cfg (feature = "have_ntgui")]
pub type HFONT = HGDIOBJ;

/// C 1300-1320/1370-1420/1531-1600's two `<wingdi.h>` entry points and the two text-alignment
/// flags of C 1390/1431/1541's `SetTextAlign (s->hdc, TA_BASELINE | TA_LEFT);`.  `FillRect`
/// (C 1314/1535) is **not** declared here: slice 1's own section above already carries it (its
/// C 551 site), and a second declaration of the same name would be `E0428`.
/// C: HBRUSH WINAPI CreatePatternBrush (HBITMAP hbm);
/// C: UINT WINAPI SetTextAlign (HDC hdc, UINT align);
/// C: #define TA_LEFT 0
/// C: #define TA_BASELINE 24
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn CreatePatternBrush (hbm: HBITMAP) -> HBRUSH;
  pub fn SetTextAlign (hdc: HDC, align: UINT) -> UINT;
}
#[cfg (feature = "have_ntgui")]
pub const TA_LEFT: UINT = 0;
#[cfg (feature = "have_ntgui")]
pub const TA_BASELINE: UINT = 24;

/// C 1329-1363/1368-1421/1520-1601's `min`/`max`: `lisp.h`'s two object-like macros (lisp.h
/// 71-73, `#undef min`/`#undef max` then the two `?:` definitions).  `include/lisp.rs` does not
/// carry them and the only mirror that does - `lib/include/min-max.rs`, the mirror of
/// `lib/min-max.h` - is not on this unit's include graph, so the pair is declared here at the
/// use site and the ledger's `## Owed` records it.  The bodies are C's tokens: each expands its
/// argument twice, which the call sites here all survive (they pass plain members and literals).
/// C: #define max(a, b) ((a) > (b) ? (a) : (b))
/// C: #define min(a, b) ((a) < (b) ? (a) : (b))
#[cfg (feature = "have_ntgui")]
macro_rules! max { ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } }; }
#[cfg (feature = "have_ntgui")]
macro_rules! min { ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } }; }

/// C 1754-1760/1887-1900's `s->face->use_box_color_for_shadows_p`, `->box` and `->overstrike`:
/// `struct face`'s bitfield run (dispextern.h 1893-1935), which `include/dispextern.rs` stores as
/// one `bits: u32` (PORTING.md 1.3/P-80) but does not name.  `src/xfaces.rs` 351-435 is the
/// sibling copy of the same accessors; the masks below are that unit's measured positions and
/// are repeated here (a unit may only name what it declares or includes, P-72), and the ledger
/// records the sharing.  The member name stays the macro's first argument, so the ported text
/// keeps C's token.
/// C: ENUM_BF (face_box_type) box : 2;   (dispextern.h 1893)
/// C: bool_bf use_box_color_for_shadows_p : 1;   (dispextern.h 1900)
/// C: bool_bf overstrike : 1;   (dispextern.h 1913)
#[cfg (feature = "have_ntgui")]
pub const FACE_BOX_SHIFT: u32 = 0;
#[cfg (feature = "have_ntgui")]
pub const FACE_BOX_MASK: u32 = 0x3;
#[cfg (feature = "have_ntgui")]
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
#[cfg (feature = "have_ntgui")]
pub const FACE_OVERSTRIKE: u32 = 1 << 21;
#[cfg (feature = "have_ntgui")]
/* The eight `struct face` members slice 4's `w32_draw_glyph_string` text reads and this unit's
   copy did not name yet, at the positions dispextern.h 1893-1935 gives them (the `src/xterm.rs`
   9850-9870 row of the same record, whose values this copy repeats; `include/dispextern.rs`'s own
   copy is behind `dispextern_face_records`, so the mirror row is owed - P-26/P-80).
   C: unsigned underline : 3; then the `overline_p`/`strike_through_p` bits, the two
   `*_defaulted_p` and `underline_at_descent_line_p` bits and the 3-bit `underline` field.  */
#[cfg (feature = "have_ntgui")]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (feature = "have_ntgui")]
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
#[cfg (feature = "have_ntgui")]
pub const FACE_OVERLINE_P: u32 = 1 << 6;
#[cfg (feature = "have_ntgui")]
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
#[cfg (feature = "have_ntgui")]
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
#[cfg (feature = "have_ntgui")]
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
#[cfg (feature = "have_ntgui")]
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
#[cfg (feature = "have_ntgui")]
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;

macro_rules! face_bits {
  (box, $f:expr) =>
    { (((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT) as c_int };
  (use_box_color_for_shadows_p, $f:expr) =>
    { ((*$f).bits & FACE_USE_BOX_COLOR_FOR_SHADOWS_P) != 0 };
  /* The seven arms C 2820-2971's text reads (the `xterm.rs` 9875-9886 copy of the same
     members).  */
  (underline, $f:expr) =>
    { (((*$f).bits & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT) as c_int };
  (overline_p, $f:expr) => { ((*$f).bits & FACE_OVERLINE_P) != 0 };
  (strike_through_p, $f:expr) => { ((*$f).bits & FACE_STRIKE_THROUGH_P) != 0 };
  (underline_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_DEFAULTED_P) != 0 };
  (overline_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_OVERLINE_COLOR_DEFAULTED_P) != 0 };
  (strike_through_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P) != 0 };
  (underline_at_descent_line_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_AT_DESCENT_LINE_P) != 0 };
  (overstrike, $f:expr) => { ((*$f).bits & FACE_OVERSTRIKE) != 0 };
}

/// C 1440/1448-1456/1570-1584's `glyph->u.cmp.automatic` and `glyph->u.glyphless.*`: the two
/// `glyph` union members' bitfield units (dispextern.h 596-602 and 624-634).  The mirror carries
/// the units and their masks (`include/dispextern.rs` 417-443) but no accessor, so the pair is
/// declared here at the use site; `src/term.rs` 840-851 is the sibling copy of the first (the
/// `glyph_u_cmp_bits!` shape) and the ledger records the sharing.
/// C: bool_bf automatic : 1; unsigned id : 31;   (dispextern.h 597-598)
/// C: unsigned method : 2; bool_bf for_no_font : 1; unsigned len : 4; unsigned ch : 25;
/// C:                                                          (dispextern.h 626-632)
#[cfg (feature = "have_ntgui")]
macro_rules! glyph_u_cmp_bits {
  (automatic, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_AUTOMATIC) != 0) };
  (id, $g:expr) => { (((*$g).u.cmp.bits >> GLYPH_U_CMP_ID_SHIFT) as ptrdiff_t) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/// C 1197/1331/1789/1853's five display entry points the mirrors do not carry yet: the unit
/// declares each at its use site (the `androidterm.rs` 160-175 / `dispins.rs` 257-259 precedent,
/// and slice 1's five of the same class) and the ledger records the gap.
/// C: dispextern.h 3581: `int window_box_right (struct window *, enum glyph_row_area);`
/// C: dispextern.h 3661-3662: `extern int get_glyph_string_clip_rects (struct glyph_string *,
/// C:                                        NativeRectangle *, int);`
/// C: dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);`
/// C: dispextern.h 3769-3770: `RGB_PIXEL_COLOR image_background (struct image *, struct frame *,
/// C:                                  Emacs_Pix_Context img);`
/// C: dispextern.h 3771-3772: `int image_background_transparent (struct image *, struct frame *,
/// C:                                  Emacs_Pix_Context mask);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int;
  pub fn get_glyph_string_clip_rects (s: *mut glyph_string, nr: *mut RECT, n: c_int) -> c_int;
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
  pub fn image_background (image: *mut image, f: *mut frame, img: Emacs_Pix_Context)
    -> RGB_PIXEL_COLOR;
  pub fn image_background_transparent (image: *mut image, f: *mut frame, mask: Emacs_Pix_Context)
    -> c_int;
}

/// dispextern.h 136-139's `HAVE_NTGUI` arm of the `Emacs_Pix_*` trio: C's `typedef XImage
/// *Emacs_Pix_Container;` (X11's `XImage`, absent here), `typedef HDC Emacs_Pix_Context;` and
/// `#define PIX_CONTAINER_TO_CONTEXT(c) ((Emacs_Pix_Context) (c))`.  The mirror carries the other
/// four arms and gates this one away (its `Emacs_Pix_Context` is `not (have_ntgui)`), so the two
/// names stand here.
/// C: typedef HDC Emacs_Pix_Context;   (dispextern.h 138)
/// C: #define RGB_PIXEL_COLOR COLORREF   (dispextern.h 3766)
#[cfg (feature = "have_ntgui")]
pub type Emacs_Pix_Context = HDC;
#[cfg (feature = "have_ntgui")]
pub type RGB_PIXEL_COLOR = COLORREF;

/// C 1758-1760's `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT` (dispextern.h 3860-3872, the
/// close of the `HAVE_WINDOW_SYSTEM` block at 3874).  The mirror keeps the record's three masks
/// but not the two macros, so each is defined here at the use site - the `image.rs` 2197-2214 and
/// `haikuterm.rs` 215-233 copies of the same pair.
/// C: #define IMAGE_BACKGROUND(img, f, ximg) ((img)->background_valid ? (img)->background : image_background (img, f, ximg))
/// C: #define IMAGE_BACKGROUND_TRANSPARENT(img, f, mask) ((img)->background_transparent_valid ? (img)->background_transparent : image_background_transparent (img, f, mask))
#[cfg (feature = "have_ntgui")]
macro_rules! IMAGE_BACKGROUND {
  ($img:expr, $f:expr, $ximg:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_VALID_MASK != 0
	 { (*$img).background }
       else
	 { image_background ($img, $f, $ximg) as c_ulong })
    };
}
#[cfg (feature = "have_ntgui")]
macro_rules! IMAGE_BACKGROUND_TRANSPARENT {
  ($img:expr, $f:expr, $mask:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK != 0
	 { ((*$img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int }
       else
	 { image_background_transparent ($img, $f, $mask) })
    };
}

/* C 1004-1013: the close of the fringe-bitmap pair C 989 opened - `w32_define_fringe_bitmap` is
   the last item of slice 1, this is the first of slice 2.  C's `fringe_bmp[which]` truth test is
   the port's null test, and the `0` C stores is the port's `0 as HBITMAP` (the same value).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_destroy_fringe_bitmap (which: c_int)
{
  if which >= max_fringe_bmp
    { return; }

  if ! (*fringe_bmp.add (which as usize)).is_null ()
    { DeleteObject (*fringe_bmp.add (which as usize)); }
  *fringe_bmp.add (which as usize) = 0 as HBITMAP;
}

/* C 1015-1017 is the page banner (`Display Iterator`): a comment in C, carried as one here.  */

/* C 1019-1046 is "Function prototypes of this page." - C's frozen `static` prototype block, the
   same class as C 218-257 above: dropped (PORTING.md 0.1-12), each line recorded here with the
   place its definition lands.  Fifteen of the twenty name functions this slice defines, so the
   definition below is the only declaration they get; the five that do not are named as such and
   are *not* declared here, so this slice's call sites - `w32_draw_glyph_string_box`'s
   `w32_draw_box_rect`/`w32_draw_relief_rect` are the only intra-slice ones - never reach them.

   * C 1021 `w32_set_glyph_string_clipping`  - defined at C 1195 (this slice);
   * C 1022 `w32_set_glyph_string_gc`        - defined at C 1153 (this slice);
   * C 1023 `w32_draw_glyph_string_background` - defined at C 1329 (this slice);
   * C 1025 `w32_draw_glyph_string_foreground` - defined at C 1368 (this slice);
   * C 1026 `w32_draw_composite_glyph_string_foreground` - defined at C 1425 (this slice);
   * C 1027 `w32_draw_glyph_string_box`      - defined at C 1886 (this slice);
   * C 1028 `w32_draw_glyph_string`          - **outside**: defined at C 2720, the next lane's;
   * C 1029 `w32_set_cursor_gc`               - defined at C 1052 (this slice);
   * C 1030 `w32_set_mode_line_face_gc`       - defined at C 1142 (this slice);
   * C 1031 `w32_set_mouse_face_gc`           - defined at C 1106 (this slice);
   * C 1032 `w32_alloc_lighter_color`         - defined at C 1625 (this slice);
   * C 1033 `w32_setup_relief_color`          - defined at C 1716 (this slice);
   * C 1035 `w32_setup_relief_colors`         - defined at C 1752 (this slice);
   * C 1036 `w32_draw_image_glyph_string`     - **outside**: defined at C 2417;
   * C 1037 `w32_draw_image_relief`           - **outside**: defined at C 2207;
   * C 1038 `w32_draw_image_foreground`       - **outside**: defined at C 1975;
   * C 1039 `w32_draw_image_foreground_1`     - **outside**: defined at C 2299;
   * C 1040 `w32_clear_glyph_string_rect`     - defined at C 1270 (this slice);
   * C 1042 `w32_draw_relief_rect`            - defined at C 1787 (this slice);
   * C 1045 `w32_draw_box_rect`               - defined at C 1851 (this slice).

   The four "outside" ones are the image page's own statics; none of them is called by this
   slice, so no name here reaches a definition the port does not yet carry (`w32_clip_to_row`'s
   case, slice 1's `## Owed`/`## Deviations`, is the only one of that shape and it stays there).  */

/* C 1049-1101: "Set S->gc to a suitable GC for drawing glyph string S in cursor face."  C's
   `s->f->output_data.w32->cursor_gc` chain is written the way the reference writes it (the
   `w32_output` pointer is the use-site cast P-72 rule 3, as in slice 1's C 970 read), `s->cmp`
   is the port's null test, and the `unsigned long` members of `Emacs_GC` (dispextern.h 74-75)
   take C's implicit widening from `COLORREF`/`int` written out (`as c_ulong`).  C's `NULL` is
   `core::ptr::null_mut ()` and `mask`'s two mask constants are `dispextern.rs`'s `c_int`s, so the
   one conversion C performs is spelled `as c_ulong`.  `IF_DEBUG (w32_check_font (…))` keeps C's
   tokens; `w32_check_font`'s definition is C 7747's, in a later slice, and its C 256 prototype is
   dropped with the C 218-257 block - the same open name `w32_clip_to_row` is (slice 1's
   `## Owed`/`## Deviations`), so this slice names it and does not declare it.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_cursor_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;

  if (*s).font == FRAME_FONT! ((*s).f)
      && (*face).background == FRAME_BACKGROUND_PIXEL ((*s).f)
      && (*face).foreground == FRAME_FOREGROUND_PIXEL ((*s).f)
      /* Sometimes we are not called for each change in the default
	 face's background color (e.g., bug#26851), so the additional
	 test in the next line gives us a chance to resync.  */
      && (*(*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_gc).foreground
	 == (*face).background
      && (*s).cmp.is_null ()
    { (*s).gc = (*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_gc; }
  else
    {
      /* Cursor on non-default face: must merge.  */
      let mut egc: Emacs_GC = core::mem::zeroed ();
      let mut mask: c_ulong = 0;

      egc.background
	= (*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_pixel as c_ulong;
      egc.foreground = (*face).background;

      /* If the glyph would be invisible, try a different foreground.  */
      if egc.foreground == egc.background
	{ egc.foreground = (*face).foreground; }
      if egc.foreground == egc.background
	{
	  egc.foreground
	    = (*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_foreground_pixel
	      as c_ulong;
	}
      if egc.foreground == egc.background
	{ egc.foreground = (*face).foreground; }

      /* Make sure the cursor is distinct from text in this face.  */
      if egc.background == (*face).background
	  && egc.foreground == (*face).foreground
	{
	  egc.background = (*face).foreground;
	  egc.foreground = (*face).background;
	}

      IF_DEBUG! (w32_check_font ((*s).f, (*s).font));
      mask = (GCForeground | GCBackground) as c_ulong;

      if ! (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()
	{
	  XChangeGC (core::ptr::null_mut (),
		     (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc, mask, &raw mut egc);
	}
      else
	{
	  (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc
	    = XCreateGC (core::ptr::null_mut (), FRAME_W32_WINDOW! ((*s).f), mask,
			 &raw mut egc);
	}

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc;
    }
}

/* C 1104-1135: "Set up S->gc of glyph string S for drawing text in mouse face."  The same
   scratch-GC pair as above, with `s->face->font`'s `*mut c_void` mirror member cast to C's
   `struct font *`, and C's `eassert (s->gc != 0)` written as the null-pointer comparison the
   0 spells (`0 as *mut Emacs_GC`, the `indent.rs` 730 shape).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_mouse_face_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;

  /* If font in this face is same as S->font, use it.  */
  if (*s).font == (*face).font as *mut font
    { (*s).gc = (*face).gc; }
  else
    {
      /* Otherwise construct scratch_cursor_gc with values from FACE
	 but font FONT.  */
      let mut egc: Emacs_GC = core::mem::zeroed ();
      let mut mask: c_ulong = 0;

      egc.background = (*face).background;
      egc.foreground = (*face).foreground;
      IF_DEBUG! (w32_check_font ((*s).f, (*s).font));
      mask = (GCForeground | GCBackground) as c_ulong;

      if ! (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()
	{
	  XChangeGC (core::ptr::null_mut (),
		     (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc, mask, &raw mut egc);
	}
      else
	{
	  (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc
	    = XCreateGC (core::ptr::null_mut (), FRAME_W32_WINDOW! ((*s).f), mask,
			 &raw mut egc);
	}

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc;
    }

  eassert! ((*s).gc != 0 as *mut Emacs_GC);
}

/* C 1138-1146: "Set S->gc of glyph string S to a GC suitable for drawing a mode line. …"  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_mode_line_face_gc (s: *mut glyph_string)
{
  (*s).gc = (*( (*s).face as *mut face )).gc;
}

/* C 1149-1189: "Set S->gc of glyph string S for drawing that glyph string.  Set S->stippled_p to
   a non-zero value if the face of S has a stipple pattern."  The five `s->hl` comparisons are C's
   `enum draw_glyphs_face` against `dispextern.rs`'s constants; the four `s->stippled_p` writes
   are the record's bitfield run, so each is `glyph_string_bits!` named after the member; the
   `s->face->stipple != 0` test keeps C's `0` (`stipple` is a `ptrdiff_t` member of the mirror's
   `face`), and C's final `emacs_abort ()` is the port's.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_glyph_string_gc (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;

  prepare_face_for_display ((*s).f, face);

  if (*s).hl == DRAW_NORMAL_TEXT
    {
      (*s).gc = (*face).gc;
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_INVERSE_VIDEO
    {
      w32_set_mode_line_face_gc (s);
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_CURSOR
    {
      w32_set_cursor_gc (s);
      glyph_string_bits! (stippled_p, (*s), false);
    }
  else if (*s).hl == DRAW_MOUSE_FACE
    {
      w32_set_mouse_face_gc (s);
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else if (*s).hl == DRAW_IMAGE_RAISED
	   || (*s).hl == DRAW_IMAGE_SUNKEN
    {
      (*s).gc = (*face).gc;
      glyph_string_bits! (stippled_p, (*s), (*face).stipple != 0);
    }
  else
    { emacs_abort (); }

  /* GC must have been set.  */
  eassert! ((*s).gc != 0 as *mut Emacs_GC);
}

/* C 1192-1213: "Set clipping for output of glyph string S.  S may be part of a mode line or menu
   if we don't have X toolkit support."  C's `RECT *r = s->clip;` reads the mirror's degraded
   `NativeRectangle clip[2]` (`[c_int; 2]`) through the same cast slice 1's
   `w32_restore_glyph_string_clip` uses; `r + 1` is `r.add (1)`, `s->hdc` is the mirror's untyped
   member cast to `HDC`, and `s->num_clips = n` is C's trailing assignment (outside the `if`).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_glyph_string_clipping (s: *mut glyph_string)
{
  let r: *mut RECT = (&raw mut (*s).clip) as *mut RECT;
  let n: c_int = get_glyph_string_clip_rects (s, r, 2);

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
  (*s).num_clips = n;
}

/* C 1215-1232: "Set SRC's clipping for output of glyph string DST. …"  The `dst->clip[0] = r;`
   store goes through the same degraded-array cast as the read above, and C's `[0]` subscript
   is the pointer's `.add (0)`, so the source keeps C's own index.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_glyph_string_clipping_exactly (src: *mut glyph_string, dst: *mut glyph_string)
{
  let mut r: RECT = core::mem::zeroed ();

  r.left = (*src).x;
  r.right = r.left + (*src).width;
  r.top = (*src).y;
  r.bottom = r.top + (*src).height;
  *((&raw mut (*dst).clip) as *mut RECT).add (0) = r;
  (*dst).num_clips = 1;
  w32_set_clip_rectangle ((*dst).hdc as HDC, &raw mut r);
}

/* C 1234-1266: "RIF: Compute left and right overhang of glyph string S."  `metrics` is C's
   uninitialized local, zeroed and named (PORTING.md 1.5); `s->first_glyph->type` is the glyph
   record's bitfield run read through `glyph_bits!`, `s->font_not_found_p` the glyph string's;
   the two `font->driver->text_extents (…)`/`->draw (…)` calls are the two hops of C's chain, cast
   at the use site (`include/font.rs` 133 keeps `driver` untyped, P-72 rule 3).  `s->cmp` is
   `*mut composition` and the three metric members are `short`s, so C's assignment to the `int`
   overhang members widens with `as c_int`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_compute_glyph_string_overhangs (s: *mut glyph_string)
{
  if (*s).cmp.is_null ()
    {
      let mut metrics: font_metrics = core::mem::zeroed ();

      if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH
	  && ! glyph_string_bits! (font_not_found_p, (*s))
	{
	  let font: *mut font = (*s).font;
	  ((*((*font).driver as *const font_driver)).text_extents.unwrap ())
	    (font, (*s).char2b, (*s).nchars, &raw mut metrics);
	  (*s).right_overhang = (if metrics.rbearing > metrics.width
				 { (metrics.rbearing - metrics.width) as c_int }
				 else { 0 });
	  (*s).left_overhang = (if metrics.lbearing < 0
				{ (- metrics.lbearing) as c_int } else { 0 });
	}
      else if glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH
	{
	  let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);

	  composition_gstring_width (gstring, (*s).cmp_from as ptrdiff_t,
				     (*s).cmp_to as ptrdiff_t, &raw mut metrics as *mut c_void);
	  (*s).right_overhang = (if metrics.rbearing > metrics.width
				 { (metrics.rbearing - metrics.width) as c_int }
				 else { 0 });
	  (*s).left_overhang = (if metrics.lbearing < 0
				{ (- metrics.lbearing) as c_int } else { 0 });
	}
    }
  else
    {
      (*s).right_overhang = ((*( (*s).cmp as *mut composition )).rbearing - (*( (*s).cmp as *mut composition )).pixel_width) as c_int;
      (*s).left_overhang = (- (*( (*s).cmp as *mut composition )).lbearing) as c_int;
    }
}

/* C 1268-1292: "Fill rectangle X, Y, W, H with background color of glyph string S."  The four
   `real_*` locals are C's uninitialized (but always assigned) copies; C 1274-1287 is `#if 0`
   ("Take clipping into account."), dead in every configuration and carried as C's own text.  C's
   `s->gc->background` is `unsigned long` while `w32_fill_area`'s pixel parameter is a `COLORREF`
   (`DWORD`, the unit's `u32`), so C narrows implicitly - the ledger's recorded cast class.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_clear_glyph_string_rect (s: *mut glyph_string, x: c_int, y: c_int, w: c_int,
				       h: c_int)
{
  let real_x: c_int = x;
  let real_y: c_int = y;
  let real_w: c_int = w;
  let real_h: c_int = h;
/* C 1274-1287 is `#if 0` ("Take clipping into account."): C's own preprocessor never compiles it,
   so the port keeps C's text as a comment (the `cm.rs` 86 reading).

#if 0
  /* Take clipping into account.  */
  if (s->gc->clip_mask == Rect)
    {
      real_x = max (real_x, s->gc->clip_rectangle.left);
      real_y = max (real_y, s->gc->clip_rectangle.top);
      real_w = min (real_w, s->gc->clip_rectangle.right
                    - s->gc->clip_rectangle.left);
      real_h = min (real_h, s->gc->clip_rectangle.bottom
                    - s->gc->clip_rectangle.top);
    }
#endif
*/
  w32_fill_area! ((*s).f, (*s).hdc as HDC, (*(*s).gc).background as COLORREF,
		  real_x, real_y, real_w, real_h);
}

/* C 1294-1321: "Fill background with bitmap pattern from S at specified position given by X and
   Y. …"  C's `FRAME_DISPLAY_INFO (s->f)->bitmaps[s->face->stipple - 1].stipple` is the display
   info's bitmap table read at the face's stipple index (the `s->face->stipple - 1` subscript is
   `ptrdiff_t`, so the pointer's `add` takes it widened), and the two `Emacs_GC` members widen to
   `COLORREF` (`SetTextColor`/`SetBkColor` take it) the way the section above records.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_fill_stipple_pattern (hdc: HDC, s: *mut glyph_string, gc: *mut Emacs_GC,
				    x: c_int, y: c_int, width: c_uint, height: c_uint)
{
  SetTextColor (hdc, (*gc).foreground as COLORREF);
  SetBkColor (hdc, (*gc).background as COLORREF);

  let mut r: RECT = core::mem::zeroed ();
  let bm: Emacs_Pixmap;
  let hb: HBRUSH;

  r.left = x;
  r.top = y;
  r.right = x + width as c_int;
  r.bottom = y + height as c_int;

  bm = (*(*FRAME_DISPLAY_INFO! ((*s).f)).bitmaps
	  .add ((((*( (*s).face as *mut face )).stipple - 1) as usize))).stipple;

  hb = CreatePatternBrush (bm);
  FillRect (hdc, &raw mut r, hb);

  DeleteObject (hb);
}

/* C 1323-1363: "Draw the background of glyph_string S.  If S->background_filled_p is non-zero
   don't draw it.  FORCE_P non-zero means draw the background even if it wouldn't be drawn
   normally. …"  `s->background_filled_p`/`s->stippled_p`/`s->font_not_found_p`/
   `s->extends_to_end_of_line_p` are the record's bitfield run, each through `glyph_string_bits!`
   named after the member; `box_line_width` is C's `max (…, 0)`; `FONT_HEIGHT`/`FONT_TOO_HIGH` are
   the two macros `include/font.rs` 231 and `include/dispextern.rs` 2601 carry.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyph_string_background (s: *mut glyph_string, force_p: bool)
{
  let face: *mut face = (*s).face as *mut face;

  /* Nothing to do if background has already been drawn or if it
     shouldn't be drawn in the first place.  */
  if ! glyph_string_bits! (background_filled_p, (*s))
    {
      let box_line_width: c_int = max! ((*face).box_horizontal_line_width, 0);

      if glyph_string_bits! (stippled_p, (*s))
	{
	  /* Fill background with a stipple pattern.  */
	  w32_fill_stipple_pattern ((*s).hdc as HDC, s, (*s).gc, (*s).x,
				    (*s).y + box_line_width,
				    (*s).background_width as c_uint,
				    ((*s).height - 2 * box_line_width) as c_uint);
	  glyph_string_bits! (background_filled_p, (*s), true);
	}
      else if FONT_HEIGHT! ((*s).font) < (*s).height - 2 * box_line_width
	       /* When xdisp.c ignores FONT_HEIGHT, we cannot trust
		  font dimensions, since the actual glyphs might be
		  much smaller.  So in that case we always clear the
		  rectangle with background color.  */
	       || FONT_TOO_HIGH! ((*s).font)
	       || glyph_string_bits! (font_not_found_p, (*s))
	       || glyph_string_bits! (extends_to_end_of_line_p, (*s))
	       || force_p
	{
	  w32_clear_glyph_string_rect (s, (*s).x, (*s).y + box_line_width,
				       (*s).background_width,
				       (*s).height - 2 * box_line_width);
	  glyph_string_bits! (background_filled_p, (*s), true);
	}
    }
}

/* C 1366-1421: "Draw the foreground of glyph string S."  The glyph walk is C's `for (i = 0;
   i < s->nchars; ++i)` written as the header's statement plus a `while` whose last statement is
   C's increment (`w32_define_fringe_bitmap`'s C 997-1001 reading); `g->pixel_width` is a `short`
   that C's arithmetic widens; `s->for_overlaps`/`s->background_filled_p` are the bitfield run,
   `s->face->overstrike` is the `face_bits!` accessor declared above, and the four
   `font->driver->draw (…)` calls are C's two-hop chain cast at the use site.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyph_string_foreground (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let mut i: c_int = 0;
  let mut x: c_int = 0;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max! ((*face).box_vertical_line_width, 0); }
  else
    { x = (*s).x; }

  SetTextColor ((*s).hdc as HDC, (*(*s).gc).foreground as COLORREF);
  SetBkColor ((*s).hdc as HDC, (*(*s).gc).background as COLORREF);
  SetTextAlign ((*s).hdc as HDC, TA_BASELINE | TA_LEFT);

  /* Draw characters of S as rectangles if S's font could not be
     loaded. */
  if glyph_string_bits! (font_not_found_p, (*s))
    {
      i = 0;
      while i < (*s).nchars
        {
          let g: *mut glyph = (*s).first_glyph.add (i as usize);

          w32_draw_rectangle ((*s).hdc as HDC, (*s).gc, x, (*s).y,
			      (*g).pixel_width as c_int - 1, (*s).height - 1);
          x += (*g).pixel_width as c_int;
          i += 1;
        }
    }
  else
    {
      let font: *mut font = (*s).font;
      let mut boff: c_int = (*font).baseline_offset;
      let y: c_int;
      let old_font: HFONT;

      old_font = SelectObject ((*s).hdc as HDC, FONT_HANDLE! (font));

      if (*font).vertical_centering
	{ boff = VCENTER_BASELINE_OFFSET! ((*font), (*s).f) - boff; }

      y = (*s).ybase - boff;
      if glyph_string_bits! (for_overlaps, (*s)) != 0
	  || (glyph_string_bits! (background_filled_p, (*s)) && (*s).hl != DRAW_CURSOR)
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, false);
	}
      else
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x, y, true);
	}
      if face_bits! (overstrike, face)
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, (*s).nchars, x + 1, y, false);
	}

      SelectObject ((*s).hdc as HDC, old_font);
    }
}

/* C 1423-1515: "Draw the foreground of composite glyph string S."  `s->first_glyph->u.cmp.automatic`
   is the glyph union's first bitfield unit (`glyph_u_cmp_bits!` above), `COMPOSITION_GLYPH` is
   `include/composite.rs`'s function of the same name and its `'\t'` test is C's TAB code point
   written as `9`; `s->cmp->offsets[j * 2]`/`[j * 2 + 1]` are the record's `short` table read
   through `add`; `LGLYPH_*` are `composite.h`'s own accessors (`include/composite.rs` 323-393),
   returning `EMACS_INT`s that C's `int` locals narrow; `gstring`/`glyph` are C's
   `Lisp_Object`s and the loop headers are C's `for (i = 0, j = s->cmp_from; …; i++, j++)` and
   `for (i = j = s->cmp_from; …; i++)` written as their statements plus a `while`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_composite_glyph_string_foreground (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let font: *mut font = (*s).font;
  let mut i: c_int = 0;
  let mut j: c_int = 0;
  let mut x: c_int = 0;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if ! (*s).face.is_null ()
      && face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max! ((*face).box_vertical_line_width, 0); }
  else
    { x = (*s).x; }

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */

  SetTextColor ((*s).hdc as HDC, (*(*s).gc).foreground as COLORREF);
  SetBkColor ((*s).hdc as HDC, (*(*s).gc).background as COLORREF);
  SetTextAlign ((*s).hdc as HDC, TA_BASELINE | TA_LEFT);

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */
  if glyph_string_bits! (font_not_found_p, (*s))
    {
      if (*s).cmp_from == 0
        {
	  w32_draw_rectangle ((*s).hdc as HDC, (*s).gc, x, (*s).y,
			      (*s).width - 1, (*s).height - 1);
	}
    }
  else if ! glyph_u_cmp_bits! (automatic, (*s).first_glyph)
    {
      let mut y: c_int = (*s).ybase;
      let old_font: HFONT;

      old_font = SelectObject ((*s).hdc as HDC, FONT_HANDLE! (font));

      i = 0;
      j = (*s).cmp_from;
      while i < (*s).nchars
	{
	  /* TAB in a composition means display glyphs with padding
	     space on the left or right.  */
	  if COMPOSITION_GLYPH ((*s).cmp as *mut composition, j as ptrdiff_t) != 9
	    {
	      let xx: c_int = x + (*(*( (*s).cmp as *mut composition )).offsets.add ((j * 2) as usize)) as c_int;
	      let yy: c_int = y - (*(*( (*s).cmp as *mut composition )).offsets.add ((j * 2 + 1) as usize)) as c_int;

	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, j, j + 1, xx, yy, false);
	      if face_bits! (overstrike, face)
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, j + 1, xx + 1, yy, false);
		}
	    }
	  i += 1;
	  j += 1;
	}
      SelectObject ((*s).hdc as HDC, old_font);
    }
  else
    {
      let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);
      let mut glyph: Lisp_Object = core::mem::zeroed ();
      let mut y: c_int = (*s).ybase;
      let mut width: c_int = 0;
      let old_font: HFONT;

      old_font = SelectObject ((*s).hdc as HDC, FONT_HANDLE! (font));

      j = (*s).cmp_from;
      i = j;
      while i < (*s).cmp_to
	{
	  glyph = LGSTRING_GLYPH (gstring, i as ptrdiff_t);
	  if NILP (LGLYPH_ADJUSTMENT (glyph))
	    { width += LGLYPH_WIDTH (glyph) as c_int; }
	  else
	    {
	      let mut xoff: c_int = 0;
	      let mut yoff: c_int = 0;
	      let mut wadjust: c_int = 0;

	      if j < i
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ())
		    (s, j, i, x, y, false);
		  x += width;
		}
	      xoff = LGLYPH_XOFF (glyph) as c_int;
	      yoff = LGLYPH_YOFF (glyph) as c_int;
	      wadjust = LGLYPH_WADJUST (glyph) as c_int;
	      ((*((*font).driver as *const font_driver)).draw.unwrap ())
		(s, i, i + 1, x + xoff, y + yoff, false);
	      x += wadjust;
	      j = i + 1;
	      width = 0;
	    }
	  i += 1;
	}
      if j < i
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, j, i, x, y, false);
	}

      SelectObject ((*s).hdc as HDC, old_font);
    }
}

/* C 1518-1601: "Draw the foreground of glyph string S for glyphless characters."  C's function
   local `static unsigned char2b[8]` is a `static mut` array in the port (the value `s->char2b`
   points at for the duration of the call, which is C's lifetime too); `buf` and `str` are C's
   uninitialized `char buf[7], *str = NULL` pair, so `buf` is zeroed and `str` is the null pointer
   (PORTING.md 1.5).  `glyph->u.glyphless.*` is the glyph union's second bitfield unit
   (`glyphless_bits!` above), `glyph->slice.glyphless.*` the four `short` offsets of that union
   member, and the acronym branch is the same accessor sequence `src/xdisp.rs` 9286-9305 spells
   for `lookup_glyphless_char_display` (`CHAR_TABLE_REF`'s no-font arm is `FAM_ELEM!` on
   `extras[0]`).  `sprintf`'s format is `<stdio.h>`'s own, the loop header is C's
   `for (i = 0; i < s->nchars; i++, glyph++)` written as statements plus a `while` (the increment
   pair last, C's order), and `with_background` is C's `(…) == 0` truth test on the `int` its
   `||` builds, which the port writes against `false`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyphless_glyph_string_foreground (s: *mut glyph_string)
{
  static mut char2b: [c_uint; 8] = [0; 8];

  let face: *mut face = (*s).face as *mut face;
  let mut glyph: *mut glyph = (*s).first_glyph;
  let mut x: c_int = 0;
  let mut i: c_int = 0;
  let mut j: c_int = 0;
  let mut with_background: bool = false;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */
  if face_bits! (box, face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
    { x = (*s).x + max! ((*face).box_vertical_line_width, 0); }
  else
    { x = (*s).x; }

  SetTextColor ((*s).hdc as HDC, (*(*s).gc).foreground as COLORREF);
  SetBkColor ((*s).hdc as HDC, (*(*s).gc).background as COLORREF);
  SetTextAlign ((*s).hdc as HDC, TA_BASELINE | TA_LEFT);

  (*s).char2b = &raw mut char2b as *mut c_uint;
  with_background = (glyph_string_bits! (for_overlaps, (*s)) != 0
		     || (glyph_string_bits! (background_filled_p, (*s))
			 && (*s).hl != DRAW_CURSOR)) == false;
  i = 0;
  while i < (*s).nchars
    {
      let mut buf: [c_char; 7] = [0; 7];
      let mut str: *mut c_char = core::ptr::null_mut ();
      let len: c_int = glyphless_bits! (len, glyph);

      if glyphless_bits! (method, glyph) == GLYPHLESS_DISPLAY_ACRONYM
	{
	  if len > 0
	      && CHAR_TABLE_P (Vglyphless_char_display)
	      && (CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display))
		  >= 1)
	    {
	      let mut acronym: Lisp_Object
		= (if ! glyphless_bits! (for_no_font, glyph)
		   { CHAR_TABLE_REF (Vglyphless_char_display,
				     glyphless_bits! (ch, glyph) as isize) }
		   else { FAM_ELEM! ((*XCHAR_TABLE (Vglyphless_char_display)).extras, 0) });
	      if CONSP (acronym)
		{ acronym = XCAR (acronym); }
	      if STRINGP (acronym)
		{ str = SSDATA (acronym); }
	    }
	}
      else if glyphless_bits! (method, glyph) == GLYPHLESS_DISPLAY_HEX_CODE
	{
	  sprintf (&raw mut buf as *mut c_char, c"%0*X".as_ptr (),
		   (if glyphless_bits! (ch, glyph) < 0x10000 { 4 } else { 6 }) as c_int,
		   glyphless_bits! (ch, glyph) as c_uint & 0xffffff);
	  str = &raw mut buf as *mut c_char;
	}

      if glyphless_bits! (method, glyph) != GLYPHLESS_DISPLAY_THIN_SPACE
	{
	  w32_draw_rectangle ((*s).hdc as HDC, (*s).gc,
			      x, (*s).ybase - (*glyph).ascent as c_int,
			      (*glyph).pixel_width as c_int - 1,
			      ((*glyph).ascent + (*glyph).descent) as c_int - 1);
	}
      if ! str.is_null ()
	{
	  let font: *mut font = (*s).font;
	  let upper_len: c_int = (len + 1) / 2;
	  let old_font: HFONT;

	  old_font = SelectObject ((*s).hdc as HDC, FONT_HANDLE! (font));
	  /* It is certain that all LEN characters in STR are ASCII.  */
	  j = 0;
	  while j < len
            {
	      char2b[j as usize]
		= ((*((*font).driver as *const font_driver)).encode_char.unwrap ())
		    (font, (*str.add (j as usize)) as c_int) & 0xFFFF;
	      j += 1;
	    }
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, upper_len, x + (*glyph).slice.glyphless.upper_xoff as c_int,
	     (*s).ybase + (*glyph).slice.glyphless.upper_yoff as c_int, with_background);
	  ((*((*font).driver as *const font_driver)).draw.unwrap ())
	    (s, upper_len, len, x + (*glyph).slice.glyphless.lower_xoff as c_int,
	     (*s).ybase + (*glyph).slice.glyphless.lower_yoff as c_int, with_background);
	  SelectObject ((*s).hdc as HDC, old_font);
	}
      x += (*glyph).pixel_width as c_int;
      i += 1;
      glyph = glyph.add (1);
   }
}

/* C 1603-1615: "Brightness beyond which a color won't have its highlight brightness boosted. …"
   - C's comment carried as C's own text, then the object-like macro's one value.  `w32_alloc_lighter_color`'s
   `long bright` is the only reader, so the constant is a `c_long` (C's `187` has type `int`, and
   C's comparison widens it; the port writes the width the comparison needs).  */
/* C 1604-1614: the comment above the macro, verbatim.
   Brightness beyond which a color won't have its highlight brightness
   boosted.

   Nominally, highlight colors for `3d' faces are calculated by
   brightening an object's color by a constant scale factor, but this
   doesn't yield good results for dark colors, so for colors whose
   brightness is less than this value (on a scale of 0-255) have to
   use an additional additive factor.

   The value here is set so that the default menu-bar/mode-line color
   (grey75) will not have its highlights changed at all.  */
/// C 1615: `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 187`
/// C: #define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 187
#[cfg (feature = "have_ntgui")]
pub const HIGHLIGHT_COLOR_DARK_BOOST_LIMIT: c_long = 187;

/* C 1618-1680: "Allocate a color which is lighter or darker than *COLOR by FACTOR or DELTA. …"
   The four `PALETTERGB` calls and their `min`/`max` nests are the two macros declared above:
   C's `min (0xff, factor * GetRValue (*color))` mixes an `int` with a `double`, so the int side
   is written `0xff as c_double` and each channel widens with `as c_double` (the recorded cast
   class); the two `int`-valued nests take `as c_int` on the channel side only.  `bright`'s
   expression is C's `int` arithmetic summed before the `/ 6` and widened after it, and C's
   `dimness` is the `double` its division produces.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_alloc_lighter_color (f: *mut frame, color: *mut COLORREF, factor: c_double,
				  delta: c_int) -> c_int
{
  let mut new: COLORREF = 0;
  let mut bright: c_long = 0;

  /* On Windows, RGB values are 0-255, not 0-65535, so scale delta. */
  delta /= 256;

  /* Change RGB values by specified FACTOR.  Avoid overflow!  */
  eassert! (factor >= 0 as c_double);
  new = PALETTERGB! (min! (0xff as c_double, factor * GetRValue (*color) as c_double),
                    min! (0xff as c_double, factor * GetGValue (*color) as c_double),
                    min! (0xff as c_double, factor * GetBValue (*color) as c_double));

  /* Calculate brightness of COLOR.  */
  bright = ((2 * GetRValue (*color) as c_int + 3 * GetGValue (*color) as c_int
	     + GetBValue (*color) as c_int) / 6) as c_long;

  /* We only boost colors that are darker than
     HIGHLIGHT_COLOR_DARK_BOOST_LIMIT.  */
  if bright < HIGHLIGHT_COLOR_DARK_BOOST_LIMIT
    /* Make an additive adjustment to NEW, because it's dark enough so
       that scaling by FACTOR alone isn't enough.  */
    {
      /* How far below the limit this color is (0 - 1, 1 being darker).  */
      let dimness: c_double
	= 1 as c_double - bright as c_double / HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_double;
      /* The additive adjustment.  */
      let min_delta: c_int = (delta as c_double * dimness * factor / 2 as c_double) as c_int;

      if factor < 1 as c_double
        { new = PALETTERGB! (max! (0, min! (0xff, min_delta - GetRValue (*color) as c_int)),
                            max! (0, min! (0xff, min_delta - GetGValue (*color) as c_int)),
                            max! (0, min! (0xff, min_delta - GetBValue (*color) as c_int))); }
      else
        { new = PALETTERGB! (max! (0, min! (0xff, min_delta + GetRValue (*color) as c_int)),
                            max! (0, min! (0xff, min_delta + GetGValue (*color) as c_int)),
                            max! (0, min! (0xff, min_delta + GetBValue (*color) as c_int))); }
    }

  if new == *color
    { new = PALETTERGB! (max! (0, min! (0xff, delta + GetRValue (*color) as c_int)),
                        max! (0, min! (0xff, delta + GetGValue (*color) as c_int)),
                        max! (0, min! (0xff, delta + GetBValue (*color) as c_int))); }

  /* TODO: Map to palette and retry with delta if same? */
  /* TODO: Free colors (if using palette)? */

  if new == *color
    { return 0; }

  *color = new;

  return 1;
}

/* C 1682-1698: "On frame F, translate pixel colors to RGB values for the NCOLORS colors in
   COLORS.  On W32, we no longer try to map colors to a palette."  `colors[i].pixel` narrows from
   `unsigned long` to C's local `DWORD`, and each channel widens from the `BYTE` its accessor
   returns to the `int` of C's `* 257`, then narrows into the record's `unsigned short`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_query_colors (f: *mut frame, colors: *mut Emacs_Color, ncolors: c_int)
{
  let mut i: c_int;

  i = 0;
  while i < ncolors
    {
      let pixel: DWORD = (*colors.add (i as usize)).pixel as DWORD;
      /* Convert to a 16 bit value in range 0 - 0xffff. */
      (*colors.add (i as usize)).red = (GetRValue (pixel) as c_int * 257) as c_ushort;
      (*colors.add (i as usize)).green = (GetGValue (pixel) as c_int * 257) as c_ushort;
      (*colors.add (i as usize)).blue = (GetBValue (pixel) as c_int * 257) as c_ushort;
      i += 1;
    }
}

/* C 1700-1707: "Store F's background color into *BGCOLOR."  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_query_frame_background_color (f: *mut frame,
							  bgcolor: *mut Emacs_Color)
{
  (*bgcolor).pixel = FRAME_BACKGROUND_PIXEL (f);
  w32_query_colors (f, bgcolor, 1);
}

/* C 1709-1747: "Set up the foreground color for drawing relief lines of glyph string S. …"  The
   two `#if 0` regions C 1719-1721 and 1731-1734 are dead in every configuration and are carried
   as C's own text; `f->output_data.w32` is the use-site cast; `relief->pixel` is the record's
   `unsigned long` member, so C's widening from `COLORREF` is written `as c_ulong`; the
   `relief->gc == 0` test is the null-pointer comparison the `0` spells; and C's chained
   `egc.foreground = relief->pixel = pixel;` is one statement per assignment in C's right-to-left
   order (PORTING.md 0.3, slice 1's recorded reading).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_setup_relief_color (f: *mut frame, relief: *mut relief, factor: c_double,
				  delta: c_int, default_pixel: COLORREF)
{
  let mut egc: Emacs_GC = core::mem::zeroed ();
  let di: *mut w32_output = (*f).output_data.w32 as *mut w32_output;
  let mut mask: c_ulong = GCForeground as c_ulong;
  let mut pixel: COLORREF = 0;
  let background: COLORREF = (*di).relief_background as COLORREF;
/* C 1719-1721 is `#if 0` (the `dpyinfo` local the stipple code below would read):

#if 0
  struct w32_display_info *dpyinfo = FRAME_DISPLAY_INFO (f);
#endif
*/
  /* TODO: Free colors (if using palette)? */

  /* Allocate new color.  */
  egc.foreground = default_pixel as c_ulong;
  pixel = background;
  if w32_alloc_lighter_color (f, &raw mut pixel, factor, delta) != 0
    {
      (*relief).pixel = pixel as c_ulong;
      egc.foreground = (*relief).pixel;
    }

  if (*relief).gc == 0 as *mut Emacs_GC
    {
/* C 1731-1734 is `#if 0 /* TODO: stipple */` (the two statements an unported stipple path would
   add); C's own preprocessor never compiles them, so the port keeps C's text as a comment.

#if 0 /* TODO: stipple */
      egc.stipple = dpyinfo->gray;
      mask |= GCStipple;
#endif
*/
      (*relief).gc = XCreateGC (core::ptr::null_mut (), FRAME_W32_WINDOW! (f), mask,
				&raw mut egc);
    }
  else
    { XChangeGC (core::ptr::null_mut (), (*relief).gc, mask, &raw mut egc); }
}

/* C 1750-1776: "Set up colors for the relief lines around glyph string S."  The three-arm colour
   choice is C's `if`/`else if`/`else`: `use_box_color_for_shadows_p` is the `face_bits!` accessor,
   `s->first_glyph->type` is `glyph_bits!`, `s->img->pixmap` is the null test C's truth test
   spells, and the two image macros are the pair declared above (their third argument is C's `0`,
   an `Emacs_Pix_Context` for this configuration).  `di->white_relief.gc == 0` is the same
   null-pointer comparison `w32_setup_relief_color` uses, and C's `color != di->relief_background`
   widens the `COLORREF` to the member's `unsigned long`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_setup_relief_colors (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let di: *mut w32_output = (*(*s).f).output_data.w32 as *mut w32_output;
  let mut color: COLORREF = 0;

  if face_bits! (use_box_color_for_shadows_p, face)
    { color = (*face).box_color as COLORREF; }
  else if glyph_bits! (type, (*(*s).first_glyph)) == IMAGE_GLYPH
	   && ! (*(*s).img).pixmap.is_null ()
	   && IMAGE_BACKGROUND_TRANSPARENT! ((*s).img, (*s).f, 0 as Emacs_Pix_Context) == 0
    { color = IMAGE_BACKGROUND! ((*s).img, (*s).f, 0 as Emacs_Pix_Context) as COLORREF; }
  else
    { color = (*(*s).gc).background as COLORREF; }

  if (*di).white_relief.gc == 0 as *mut Emacs_GC
      || color as c_ulong != (*di).relief_background
    {
      (*di).relief_background = color as c_ulong;
      w32_setup_relief_color ((*s).f, &raw mut (*di).white_relief, 1.2, 0x8000,
                              WHITE_PIX_DEFAULT! ((*s).f));
      w32_setup_relief_color ((*s).f, &raw mut (*di).black_relief, 0.6, 0x4000,
                              BLACK_PIX_DEFAULT! ((*s).f));
    }
}

/* C 1779-1841: "Draw a relief on frame F inside the rectangle given by LEFT_X, TOP_Y, RIGHT_X,
   and BOTTOM_Y. …"  The four side blocks are C's `for (i = 0; i < hwidth; ++i)` (and `vwidth`)
   written as the header's assignment plus a `while` whose last statement is C's increment; the
   other seven parameters are C's `int` flags, so each test is C's `!= 0`; `clip_rect` is C's
   `RECT *` and `NULL` the port's null pointer.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_relief_rect (f: *mut frame, left_x: c_int, top_y: c_int, right_x: c_int,
				bottom_y: c_int, hwidth: c_int, vwidth: c_int,
				raised_p: c_int, top_p: c_int, bot_p: c_int,
				left_p: c_int, right_p: c_int, clip_rect: *mut RECT)
{
  let mut i: c_int;
  let mut gc: Emacs_GC = core::mem::zeroed ();
  let hdc: HDC = get_frame_dc (f);

  if raised_p != 0
    { gc.foreground
	= (*(*( (*f).output_data.w32 as *mut w32_output )).white_relief.gc).foreground; }
  else
    { gc.foreground
	= (*(*( (*f).output_data.w32 as *mut w32_output )).black_relief.gc).foreground; }

  w32_set_clip_rectangle (hdc, clip_rect);

  /* Top.  */
  if top_p != 0
    {
      i = 0;
      while i < hwidth
	{
	  w32_fill_area! (f, hdc, gc.foreground as COLORREF,
			  left_x + i * left_p, top_y + i,
			  right_x - left_x - i * (left_p + right_p ) + 1, 1);
	  i += 1;
	}
    }

  /* Left.  */
  if left_p != 0
    {
      i = 0;
      while i < vwidth
	{
	  w32_fill_area! (f, hdc, gc.foreground as COLORREF,
			  left_x + i, top_y + (i + 1) * top_p, 1,
			  bottom_y - top_y - (i + 1) * (bot_p + top_p) + 1);
	  i += 1;
	}
    }

  if raised_p != 0
    { gc.foreground
	= (*(*( (*f).output_data.w32 as *mut w32_output )).black_relief.gc).foreground; }
  else
    { gc.foreground
	= (*(*( (*f).output_data.w32 as *mut w32_output )).white_relief.gc).foreground; }

  /* Bottom.  */
  if bot_p != 0
    {
      i = 0;
      while i < hwidth
	{
	  w32_fill_area! (f, hdc, gc.foreground as COLORREF,
			  left_x + i * left_p, bottom_y - i,
			  right_x - left_x - i * (left_p + right_p) + 1, 1);
	  i += 1;
	}
    }

  /* Right.  */
  if right_p != 0
    {
      i = 0;
      while i < vwidth
	{
	  w32_fill_area! (f, hdc, gc.foreground as COLORREF,
			  right_x - i, top_y + (i + 1) * top_p, 1,
			  bottom_y - top_y - (i + 1) * (bot_p + top_p) + 1);
	  i += 1;
	}
    }

  w32_set_clip_rectangle (hdc, core::ptr::null_mut ());

  release_frame_dc (f, hdc);
}

/* C 1844-1881: "Draw a box on frame F inside the rectangle given by LEFT_X, TOP_Y, RIGHT_X, and
   BOTTOM_Y. …"  The four `w32_fill_area` calls take the face's `box_color`, an `unsigned long`
   that C narrows into the `COLORREF` parameter.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_box_rect (s: *mut glyph_string, left_x: c_int, top_y: c_int, right_x: c_int,
			     bottom_y: c_int, hwidth: c_int, vwidth: c_int, left_p: bool,
			     right_p: bool, clip_rect: *mut RECT)
{
  let face: *mut face = (*s).face as *mut face;

  w32_set_clip_rectangle ((*s).hdc as HDC, clip_rect);

  /* Top.  */
  w32_fill_area! ((*s).f, (*s).hdc as HDC, (*face).box_color as COLORREF,
		  left_x, top_y, right_x - left_x + 1, hwidth);

  /* Left.  */
  if left_p
    {
      w32_fill_area! ((*s).f, (*s).hdc as HDC, (*face).box_color as COLORREF,
                     left_x, top_y, vwidth, bottom_y - top_y + 1);
    }

  /* Bottom.  */
  w32_fill_area! ((*s).f, (*s).hdc as HDC, (*face).box_color as COLORREF,
                 left_x, bottom_y - hwidth + 1, right_x - left_x + 1, hwidth);

  /* Right.  */
  if right_p
    {
      w32_fill_area! ((*s).f, (*s).hdc as HDC, (*face).box_color as COLORREF,
                     right_x - vwidth + 1, top_y, vwidth, bottom_y - top_y + 1);
    }

  w32_set_clip_rectangle ((*s).hdc as HDC, core::ptr::null_mut ());
}

/* C 1884-1949: "Draw a box around glyph string S." - the last item of this slice, and the point
   the ledger's boundary is drawn at.  `last_x`'s choice is C's `?:` over the row's
   `full_width_p` bit and the window's `pseudo_window_p` bit (the mirror's `glyph_row_bits!` and
   the `window_bits!` slice 1 declared); the automatic-composition walk is C's
   `for (last_glyph = g++; …; last_glyph = g++)` with an empty body, written as a `while` whose
   body is empty and whose last two statements are C's two assignments in C's order (`last_glyph
   = g` then `g = g + 1`, which is what `last_glyph = g++` does); `g->u.cmp.id` is the union
   member's second accessor arm; `eabs` is `lisp.rs`'s macro; `min` the one declared above; and
   the last three tests are `face_bits! (box, …)` against `FACE_RAISED_BOX`/`FACE_SIMPLE_BOX`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyph_string_box (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let mut hwidth: c_int;
  let mut vwidth: c_int;
  let left_x: c_int;
  let right_x: c_int;
  let top_y: c_int;
  let bottom_y: c_int;
  let mut last_x: c_int;
  let mut left_p: bool;
  let mut right_p: bool;
  let raised_p: bool;
  let mut last_glyph: *mut glyph;
  let mut clip_rect: RECT = core::mem::zeroed ();

  last_x = (if glyph_row_bits! (full_width_p, (*s).row)
	       && ! window_bits! (pseudo_window_p, (*s).w)
	    { WINDOW_RIGHT_EDGE_X! ((*s).w) }
	    else { window_box_right ((*s).w, (*s).area) });

  /* The glyph that may have a right box line.  For static
     compositions and images, the right-box flag is on the first glyph
     of the glyph string; for other types it's on the last glyph.  */
  if ! (*s).cmp.is_null () || ! (*s).img.is_null ()
    { last_glyph = (*s).first_glyph; }
  else if glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH
	   && glyph_u_cmp_bits! (automatic, (*s).first_glyph)
    {
      /* For automatic compositions, we need to look up the last glyph
	 in the composition.  */
      let end: *mut glyph
	= (*(*s).row).glyphs[(*s).area as usize].add ((*(*s).row).used[(*s).area as usize]
						      as usize);
      let mut g: *mut glyph = (*s).first_glyph;
      last_glyph = g;
      g = g.add (1);
      while g < end
	  && glyph_u_cmp_bits! (automatic, g)
	  && glyph_u_cmp_bits! (id, g) == (*s).cmp_id
	  && (*g).slice.cmp.to < (*s).cmp_to
	{
	  last_glyph = g;
	  g = g.add (1);
	}
    }
  else
    { last_glyph = (*s).first_glyph.add (((*s).nchars - 1) as usize); }

  vwidth = eabs! ((*face).box_vertical_line_width);
  hwidth = eabs! ((*face).box_horizontal_line_width);
  raised_p = face_bits! (box, face) == FACE_RAISED_BOX;
  left_x = (*s).x;
  right_x = (if glyph_row_bits! (full_width_p, (*s).row)
		&& glyph_string_bits! (extends_to_end_of_line_p, (*s))
	     { last_x - 1 }
	     else { min! (last_x, (*s).x + (*s).background_width) - 1 });
  top_y = (*s).y;
  bottom_y = top_y + (*s).height - 1;

  left_p = (glyph_bits! (left_box_line_p, (*(*s).first_glyph))
	    || ((*s).hl == DRAW_MOUSE_FACE
		&& ((*s).prev.is_null () || (*(*s).prev).hl != (*s).hl)));
  right_p = (glyph_bits! (right_box_line_p, (*last_glyph))
	     || ((*s).hl == DRAW_MOUSE_FACE
		 && ((*s).next.is_null () || (*(*s).next).hl != (*s).hl)));

  get_glyph_string_clip_rect (s, &raw mut clip_rect);

  if face_bits! (box, face) == FACE_SIMPLE_BOX
    { w32_draw_box_rect (s, left_x, top_y, right_x, bottom_y, hwidth, vwidth, left_p, right_p,
			 &raw mut clip_rect); }
  else
    {
      w32_setup_relief_colors (s);
      w32_draw_relief_rect ((*s).f, left_x, top_y, right_x, bottom_y, hwidth, vwidth, raised_p
			    as c_int, 1, 1, left_p as c_int, right_p as c_int,
			    &raw mut clip_rect);
    }
}

/* ----------------------------------------------------------------------------------------------
   The `<windows.h>` / `<wingdi.h>` / `<gdiplus.h>` names **this slice** adds to the section C 25's
   chain opened above, at the same declaration site and by the same reading (P-88 clause 1: the
   site is the chain's include line, and no file is added under `femacs/src/include/`).  The image
   page that C 1950 opens is where `w32term.c` starts reading the GDI bitmap records and the GDI+
   entry points, so its section is the first one with `#[repr(C)]` members of its own.  No
   measurement was possible: this machine has no `windows.h`, no `gdiplus.h` and no mingw toolchain
   (the file note above), so every scalar below is PORTING.md 1.1's fixed width, every record keeps
   `<windows.h>`'s own declaration order, and each size is **UNVERIFIED** (the ledger's owed layout
   list).
   ---------------------------------------------------------------------------------------------- */

/// C 1964's `0.5f` and C 3247's `XFORM`: `<windows.h>`'s `typedef float FLOAT;`.
/// C: typedef float FLOAT;
#[cfg (feature = "have_ntgui")]
pub type FLOAT = f32;

/// C 1978's `&s->img->xform` and the three `transform` calls: `<windows.h>`'s `tagXFORM` (six
/// `FLOAT`s, 24 bytes).  `struct image`'s member of this type is the mirror's recorded deviation
/// (`include/dispextern.rs` 2412-2416: it stores a `*mut c_void` where C reads a record by value),
/// so every site that names the member is the use-site cast `as *mut XFORM` - this unit is the
/// first reader of that member and the ledger records it.
/// C: typedef struct tagXFORM { FLOAT eM11; FLOAT eM12; FLOAT eM21; FLOAT eM22; FLOAT eDx;
/// C:                         FLOAT eDy; } XFORM;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XFORM
{
  pub eM11: FLOAT,
  pub eM12: FLOAT,
  pub eM21: FLOAT,
  pub eM22: FLOAT,
  pub eDx: FLOAT,
  pub eDy: FLOAT,
}

/// C 2009's `BITMAP bmp = dib.dsBm;` and C 2195's `BITMAP bm_info;`: `<wingdi.h>`'s `tagBITMAP`.
/// C: typedef struct tagBITMAP { LONG bmType; LONG bmWidth; LONG bmHeight; LONG bmWidthBytes;
/// C:                           WORD bmPlanes; WORD bmBitsPixel; LPVOID bmBits; } BITMAP;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAP
{
  pub bmType: LONG,
  pub bmWidth: LONG,
  pub bmHeight: LONG,
  pub bmWidthBytes: LONG,
  pub bmPlanes: WORD,
  pub bmBitsPixel: WORD,
  /// C: `LPVOID bmBits;`
  pub bmBits: *mut c_void,
}

/// C 2003's `DIBSECTION dib;` embeds `<wingdi.h>`'s `tagBITMAPINFOHEADER` by value, so the record
/// stands here with it (`sizeof (dib)` is C 2008's second argument to `GetObject`, which is this
/// record's size whatever the member values are).
/// C: typedef struct tagBITMAPINFOHEADER { DWORD biSize; LONG biWidth; LONG biHeight;
/// C:      WORD biPlanes; WORD biBitCount; DWORD biCompression; DWORD biSizeImage;
/// C:      LONG biXPelsPerMeter; LONG biYPelsPerMeter; DWORD biClrUsed; DWORD biClrImportant; }
/// C:      BITMAPINFOHEADER;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPINFOHEADER
{
  pub biSize: DWORD,
  pub biWidth: LONG,
  pub biHeight: LONG,
  pub biPlanes: WORD,
  pub biBitCount: WORD,
  pub biCompression: DWORD,
  pub biSizeImage: DWORD,
  pub biXPelsPerMeter: LONG,
  pub biYPelsPerMeter: LONG,
  pub biClrUsed: DWORD,
  pub biClrImportant: DWORD,
}

/// C 2003/2009/2198's `DIBSECTION`: `<wingdi.h>`'s `tagDIBSECTION`, whose first member is the
/// `BITMAP` that `GetObject` fills and whose `dsBm` C 2009 reads.
/// C: typedef struct tagDIBSECTION { BITMAP dsBm; BITMAPINFOHEADER dsBmih; DWORD dsBitfields[3];
/// C:                               HANDLE dshSection; DWORD dsOffset; } DIBSECTION;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIBSECTION
{
  pub dsBm: BITMAP,
  pub dsBmih: BITMAPINFOHEADER,
  pub dsBitfields: [DWORD; 3],
  pub dshSection: HANDLE,
  pub dsOffset: DWORD,
}

/// C 2008/2198's `GetObject`, `<wingdi.h>`'s `GetObjectA`.  Its second argument is C's
/// `sizeof (dib)` / `sizeof (BITMAP)`, written out with `size_of`.
/// C: int WINAPI GetObjectA (HANDLE h, int c, LPVOID pv);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetObject (h: HANDLE, c: c_int, pv: *mut c_void) -> c_int;
}

/// C 2110/2155-2157/2199 and 2055's `StretchBlt` family, C 2153/2159's `SetStretchBltMode` and
/// C 2154's `SetBrushOrgEx` (`<wingdi.h>`), and C 2124-2125's `CopyImage`/`GetCurrentObject`
/// (`<winuser.h>`/`<wingdi.h>`).
/// C: BOOL WINAPI StretchBlt (HDC hdcDest, int xDest, int yDest, int wDest, int hDest,
/// C:                          HDC hdcSrc, int xSrc, int ySrc, int wSrc, int hSrc, DWORD rop);
/// C: int WINAPI SetStretchBltMode (HDC hdc, int mode);
/// C: BOOL WINAPI SetBrushOrgEx (HDC hdc, int x, int y, LPPOINT lppt);
/// C: HANDLE WINAPI CopyImage (HANDLE h, UINT type, int cx, int cy, UINT flags);
/// C: HGDIOBJ WINAPI GetCurrentObject (HDC hdc, UINT type);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn StretchBlt (hdcDest: HDC, xDest: c_int, yDest: c_int, wDest: c_int, hDest: c_int,
                     hdcSrc: HDC, xSrc: c_int, ySrc: c_int, wSrc: c_int, hSrc: c_int,
                     rop: DWORD) -> BOOL;
  pub fn SetStretchBltMode (hdc: HDC, mode: c_int) -> c_int;
  pub fn SetBrushOrgEx (hdc: HDC, x: c_int, y: c_int, lppt: LPPOINT) -> BOOL;
  pub fn CopyImage (h: HANDLE, r#type: UINT, cx: c_int, cy: c_int, flags: UINT) -> HANDLE;
  pub fn GetCurrentObject (hdc: HDC, r#type: UINT) -> HGDIOBJ;
}

/// C 2110-2111/2155-2157's four raster-operation codes and C 2153/2159's HALFTONE stretch mode
/// (`<wingdi.h>`), and C 2124-2125's two object-type constants.
/// C: #define SRCINVERT 0x00660046
/// C: #define SRCAND 0x008800C6
/// C: #define HALFTONE 4
/// C: #define OBJ_BITMAP 7
/// C: #define IMAGE_BITMAP 0
#[cfg (feature = "have_ntgui")]
pub const SRCINVERT: DWORD = 0x00660046;
#[cfg (feature = "have_ntgui")]
pub const SRCAND: DWORD = 0x008800C6;
#[cfg (feature = "have_ntgui")]
pub const HALFTONE: c_int = 4;
#[cfg (feature = "have_ntgui")]
pub const OBJ_BITMAP: UINT = 7;
#[cfg (feature = "have_ntgui")]
pub const IMAGE_BITMAP: UINT = 0;

/// C 2117-2145's GDI+ names (`<gdiplus.h>`), the `src/w32image.rs` 244-293 reading of the same
/// header and the same reason: `include/w32gdiplus.rs` leaves the SDK's GDI+ types to its
/// includer (its own head note says so), so the unit declares the ones it names.  `GpStatus` is
/// the SDK's `Status` enum and the three constants are C's own values.
/// C: typedef INT GpStatus; typedef enum Status { Ok = 0, GenericError, InvalidParameter, ... };
/// C: typedef struct GpImage GpImage; typedef struct GpGraphics GpGraphics;
/// C: typedef struct GpBitmap GpBitmap; typedef INT GpUnit; typedef INT InterpolationMode;
/// C: typedef enum InterpolationMode { InterpolationModeInvalid = QualityModeInvalid, ...,
/// C:                                  InterpolationModeHighQualityBilinear, ... };
/// C: typedef enum GpUnit { UnitWorld = 0, UnitDisplay, UnitPixel, ... };
/// C: typedef VOID (*DrawImageAbort) (VOID *);
#[cfg (feature = "have_ntgui")]
pub type GpStatus = c_int;
#[cfg (feature = "have_ntgui")]
pub type GpImage = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type GpGraphics = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type GpBitmap = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type GpUnit = c_int;
#[cfg (feature = "have_ntgui")]
pub type InterpolationMode = c_int;
#[cfg (feature = "have_ntgui")]
pub type DrawImageAbort = Option<unsafe extern "system" fn (*mut c_void) -> BOOL>;
#[cfg (feature = "have_ntgui")]
pub const Ok: GpStatus = 0;
#[cfg (feature = "have_ntgui")]
pub const UnitPixel: GpUnit = 2;
#[cfg (feature = "have_ntgui")]
pub const InterpolationModeHighQualityBilinear: InterpolationMode = 6;

/// C 1978/2213/2304's `image_ascent`.  `dispextern.h` 3774 declares it and `src/image.rs`
/// defines it (`image.rs` 2808, `#[unsafe(no_mangle)]`), so the declaration keeps C's prototype
/// and no stand-in (the `gui_define_fringe_bitmap` reading above).  C's `&s->slice` is C's
/// address-of on a `struct glyph_slice`, the port's `&raw mut`.
/// C: dispextern.h 3774: `int image_ascent (struct image *, struct face *, struct glyph_slice *);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn image_ascent (img: *mut image, face: *mut face, slice: *mut glyph_slice) -> c_int;
}

/// C 2552/2564/2616's three display entry points of the same class as the five above
/// (`dispextern.h` declares them, no mirror in this unit's graph carries them), each declared at
/// its use site; the ledger records the gap for the integrator.
/// C: dispextern.h 3579: `int window_box_left (struct window *, enum glyph_row_area);`
/// C: dispextern.h 3580: `int window_box_left_offset (struct window *, enum glyph_row_area);`
/// C: dispextern.h 3692: `extern bool cursor_in_mouse_face_p (struct window *w);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  pub fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
  pub fn cursor_in_mouse_face_p (w: *mut window) -> bool;
}

/* ==============================================================================================
   **Slice 3: C 1950-….**  The image page C 1951-2415 (its first definition, `w32_image_rotations_p`
   at C 1952-1955, was the boundary the previous slice stopped short of), then the glyph-string
   drawing C 2417-… that follows it.  The ledger `docs/ports/w32term.md` records the boundary and
   its neighbours.

   Everything below carries the same readings the file note above records - the file gate is
   `feature = "have_ntgui"`, both arms of every `#if` are carried, the `#if 0` regions are C's own
   text inside a comment, the `<windows.h>` declarations are spelled (this machine has no
   `windows.h` to measure against) and every `#[repr(C)]` record stays **UNVERIFIED** here.  The
   ledger's `## Evidence` is the frozen `build-tu`, the `extract-abi --check` reading and the
   inventory gates.
   ============================================================================================== */

/* C 1951-1955: "Return true if the GDI+ `PlgBlt` entry point is available."  C's `pfnPlgBlt !=
   NULL` is the `Option`'s `is_some ()` (the `pfnSetLayeredWindowAttributes` reading above).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_image_rotations_p () -> bool
{
  return pfnPlgBlt.is_some ();
}

/* C 1957-1969: `transform` - the affine `XFORM` applied to one corner of the image.  C's four
   `FLOAT` members are `f32`, so the whole expression is C's `float` arithmetic and the
   assignment to the `POINT`'s `LONG` members is C's implicit `float` -> `long` conversion, written
   out (`as c_int`, PORTING.md 1.4); C's `0.5f` is a `FLOAT` literal.  `struct image`'s `xform`
   member is the mirror's recorded deviation (`include/dispextern.rs` 2412-2416: a `*mut c_void`
   where C holds the record by value), so every read is the use-site cast at the call sites.  */
#[cfg (feature = "have_ntgui")]
unsafe fn transform (x0: c_int, y0: c_int, x: c_int, y: c_int, xform: *mut XFORM) -> POINT
{
  let mut pt: POINT = core::mem::zeroed ();

  /* See https://docs.microsoft.com/en-us/windows/desktop/api/Wingdi/nf-wingdi-setworldtransform */
  pt.x
    = (x0 as FLOAT + (x - x0) as FLOAT * (*xform).eM11 + (y - y0) as FLOAT * (*xform).eM21
       + (*xform).eDx + 0.5f32) as c_int;
  pt.y
    = (y0 as FLOAT + (x - x0) as FLOAT * (*xform).eM12 + (y - y0) as FLOAT * (*xform).eM22
       + (*xform).eDy + 0.5f32) as c_int;

  return pt;
}


/* C 1972-2190: "Draw foreground of image glyph string S."  The three `s->slice` reads are the
   record's four 16-bit members (`glyph_slice_bits!`, `include/dispextern.rs` 599-608) and every
   one of them lands in an `int` parameter or variable, so each is C's `unsigned` -> `int`
   conversion written out; `s->gc->foreground`/`background` widen to `COLORREF` the way the
   `<windows.h>` section above records; `s->img->xform` is the use-site cast
   `(*(*s).img).xform as *mut XFORM`; and C's `(pmode = SetStretchBltMode (…)) != 0` is the block
   operand the `terminal.rs` 479 reading gives an assignment inside a condition.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_image_foreground (s: *mut glyph_string)
{
  let mut x: c_int = (*s).x;
  let mut y: c_int
    = (*s).ybase - image_ascent ((*s).img, (*s).face as *mut face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max! ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*(*s).img).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*(*s).img).vmargin; }

  SaveDC ((*s).hdc as HDC);

  if ! (*(*s).img).pixmap.is_null ()
    {
      let compat_hdc: HDC = CreateCompatibleDC ((*s).hdc as HDC);
      let fg_brush: HBRUSH = CreateSolidBrush ((*(*s).gc).foreground as COLORREF);
      let orig_brush: HBRUSH = SelectObject ((*s).hdc as HDC, fg_brush);
      let orig_obj: HGDIOBJ = SelectObject (compat_hdc, (*(*s).img).pixmap);
      let mut orig_width: LONG = 0;
      let mut orig_height: LONG = 0;
      let mut dib: DIBSECTION = core::mem::zeroed ();
      SetBkColor (compat_hdc, RGB! (255, 255, 255));
      SetTextColor ((*s).hdc as HDC, RGB! (0, 0, 0));
      w32_set_glyph_string_clipping (s);
      /* Extract the original dimensions of the bitmap.  */
      if GetObject ((*(*s).img).pixmap, core::mem::size_of::<DIBSECTION> () as c_int,
		    &raw mut dib as *mut c_void) > 0
	{
	  let bmp: BITMAP = dib.dsBm;
	  orig_width = bmp.bmWidth;
	  orig_height = bmp.bmHeight;
	}
      else
	{
	  DebPrint! (("w32_draw_image_foreground: GetObject(pixmap) failed!\n"));
	  orig_width = glyph_slice_bits! (width, (*s).slice) as LONG;
	  orig_height = glyph_slice_bits! (height, (*s).slice) as LONG;
	}

      let mut w_factor: c_double = 1.0;
      let mut h_factor: c_double = 1.0;
      let mut scaled: bool = false;
      let mut need_xform: bool = false;
      let mut orig_slice_width: c_int = glyph_slice_bits! (width, (*s).slice) as c_int;
      let mut orig_slice_height: c_int = glyph_slice_bits! (height, (*s).slice) as c_int;
      let mut orig_slice_x: c_int = glyph_slice_bits! (x, (*s).slice) as c_int;
      let mut orig_slice_y: c_int = glyph_slice_bits! (y, (*s).slice) as c_int;
      let mut corner: [POINT; 3] = core::mem::zeroed ();
      if ((*( (*(*s).img).xform as *mut XFORM )).eM12 != 0 as FLOAT
	  || (*( (*(*s).img).xform as *mut XFORM )).eM21 != 0 as FLOAT
	  || (*( (*(*s).img).xform as *mut XFORM )).eDx != 0 as FLOAT
	  || (*( (*(*s).img).xform as *mut XFORM )).eDy != 0 as FLOAT)
	  /* PlgBlt is not available on Windows 9X.  */
	  && pfnPlgBlt.is_some ()
	{
	  need_xform = true;
	  corner[0] = transform (x, y, x, y, (*(*s).img).xform as *mut XFORM);
	  corner[1] = transform (x, y, x + orig_width, y, (*(*s).img).xform as *mut XFORM);
	  corner[2] = transform (x, y, x, y + orig_height, (*(*s).img).xform as *mut XFORM);
	}
      else if (*(*s).img).width != orig_width || (*(*s).img).height != orig_height
	{
	  /* For scaled images we need to restore the original slice's
	     dimensions and origin coordinates, from before the scaling.  */
	  scaled = true;
	  w_factor = orig_width as c_double / (*(*s).img).width as c_double;
	  h_factor = orig_height as c_double / (*(*s).img).height as c_double;
	  orig_slice_width
	    = (glyph_slice_bits! (width, (*s).slice) as c_double * w_factor + 0.5) as c_int;
	  orig_slice_height
	    = (glyph_slice_bits! (height, (*s).slice) as c_double * h_factor + 0.5) as c_int;
	  orig_slice_x
	    = (glyph_slice_bits! (x, (*s).slice) as c_double * w_factor + 0.5) as c_int;
	  orig_slice_y
	    = (glyph_slice_bits! (y, (*s).slice) as c_double * h_factor + 0.5) as c_int;
	}

      if ! (*(*s).img).mask.is_null ()
	{
	  let mask_dc: HDC = CreateCompatibleDC ((*s).hdc as HDC);
	  let mask_orig_obj: HGDIOBJ = SelectObject (mask_dc, (*(*s).img).mask);

	  SetTextColor ((*s).hdc as HDC, RGB! (255, 255, 255));
	  SetBkColor ((*s).hdc as HDC, RGB! (0, 0, 0));
	  if need_xform
	    {
	      if pfnPlgBlt.unwrap () ((*s).hdc as HDC, &raw mut corner as *mut POINT, compat_hdc,
					glyph_slice_bits! (x, (*s).slice) as c_int,
					glyph_slice_bits! (y, (*s).slice) as c_int,
					orig_width, orig_height, (*(*s).img).mask,
					glyph_slice_bits! (x, (*s).slice) as c_int,
					glyph_slice_bits! (y, (*s).slice) as c_int) == 0
		{ DebPrint! (("PlgBlt failed!")); }
	    }
	  else if ! scaled
	    {
	      BitBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		      glyph_slice_bits! (height, (*s).slice) as c_int,
		      compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
		      glyph_slice_bits! (y, (*s).slice) as c_int, SRCINVERT);
	      BitBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		      glyph_slice_bits! (height, (*s).slice) as c_int,
		      mask_dc, glyph_slice_bits! (x, (*s).slice) as c_int,
		      glyph_slice_bits! (y, (*s).slice) as c_int, SRCAND);
	      BitBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		      glyph_slice_bits! (height, (*s).slice) as c_int,
		      compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
		      glyph_slice_bits! (y, (*s).slice) as c_int, SRCINVERT);
	    }
	  else
	    {
	      let mut pmode: c_int = 0;
	      /* HALFTONE produces better results, especially when
		 scaling to a larger size, but Windows 9X doesn't
		 support HALFTONE.  */
	      if os_subtype == OS_SUBTYPE_NT
		  && {
		       pmode = SetStretchBltMode ((*s).hdc as HDC, HALFTONE);
		       pmode != 0
		     }
		{ SetBrushOrgEx ((*s).hdc as HDC, 0, 0, core::ptr::null_mut ()); }
	      StretchBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
			  glyph_slice_bits! (height, (*s).slice) as c_int,
			  compat_hdc, orig_slice_x, orig_slice_y,
			  orig_slice_width, orig_slice_height, SRCINVERT);
	      StretchBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
			  glyph_slice_bits! (height, (*s).slice) as c_int,
			  mask_dc, orig_slice_x, orig_slice_y,
			  orig_slice_width, orig_slice_height, SRCAND);
	      StretchBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
			  glyph_slice_bits! (height, (*s).slice) as c_int,
			  compat_hdc, orig_slice_x, orig_slice_y,
			  orig_slice_width, orig_slice_height, SRCINVERT);
	      if pmode != 0
		{ SetStretchBltMode ((*s).hdc as HDC, pmode); }
	    }
	  SelectObject (mask_dc, mask_orig_obj);
	  DeleteDC (mask_dc);
	}
      else
	{
	  SetTextColor ((*s).hdc as HDC, (*(*s).gc).foreground as COLORREF);
	  SetBkColor ((*s).hdc as HDC, (*(*s).gc).background as COLORREF);
	  if need_xform
	    {
	      if pfnPlgBlt.unwrap () ((*s).hdc as HDC, &raw mut corner as *mut POINT, compat_hdc,
					glyph_slice_bits! (x, (*s).slice) as c_int,
					glyph_slice_bits! (y, (*s).slice) as c_int,
					orig_width, orig_height,
					core::ptr::null_mut (), 0, 0) == 0
		{ DebPrint! (("PlgBlt failed!")); }
	    }
	  else if ! scaled
	    {
	      BitBlt ((*s).hdc as HDC, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		      glyph_slice_bits! (height, (*s).slice) as c_int,
		      compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
		      glyph_slice_bits! (y, (*s).slice) as c_int, SRCCOPY);
	    }
	  else
	    {
/* C 2114-2148 is `#ifdef WINDOWSNT` around the GDI+ smoothing arm and C 2147-2149's `#else`
   is the HALFTONE/`StretchBlt` arm; both are carried, and because Rust cannot attach a `cfg`
   to an `else` clause C 2148-2160's block stands once as the GDI+ test's `else` and once as
   the whole arm (the `hbfont.rs` 870 two-arm reading; `docs/ports/w32term.md` records it).
   C's own text:

#ifdef WINDOWSNT
	      if (s->img->smoothing && w32_gdiplus_startup ())
		{
		  GpGraphics *graphics;
		  if (GdipCreateFromHDC (s->hdc, &graphics) == Ok)
		    {
		      GpBitmap *gp_bitmap;
		      /* Can't create a GpBitmap from a HBITMAP that was
			 ever selected into a DC, so we need to copy.  */
		      HBITMAP copy
			= CopyImage (GetCurrentObject (compat_hdc, OBJ_BITMAP),
				     IMAGE_BITMAP, 0, 0, 0);
		      if (GdipCreateBitmapFromHBITMAP (copy, NULL,
						       &gp_bitmap) == Ok)
			{
			  GdipSetInterpolationMode (graphics,
						    InterpolationModeHighQualityBilinear);
			  GdipDrawImageRectRectI (graphics,
						  gp_bitmap, x, y,
						  s->slice.width,
						  s->slice.height,
						  orig_slice_x,
						  orig_slice_y,
						  orig_slice_width,
						  orig_slice_height,
						  UnitPixel,
						  NULL, NULL, NULL);
			  GdipDisposeImage (gp_bitmap);
			}
		      DeleteObject (copy);
		      GdipDeleteGraphics (graphics);
		    }
		}
	      else
#endif
*/
	      #[cfg (feature = "windowsnt")]
	      {
		if (*(*s).img).smoothing && w32_gdiplus_startup ()
		  {
		    let mut graphics: *mut GpGraphics = core::ptr::null_mut ();
		    if GdipCreateFromHDC! ().unwrap () ((*s).hdc as HDC, &raw mut graphics) == Ok
		      {
			let mut gp_bitmap: *mut GpBitmap = core::ptr::null_mut ();
			/* Can't create a GpBitmap from a HBITMAP that was
			   ever selected into a DC, so we need to copy.  */
			let copy: HBITMAP
			  = CopyImage (GetCurrentObject (compat_hdc, OBJ_BITMAP),
				       IMAGE_BITMAP, 0, 0, 0);
			if GdipCreateBitmapFromHBITMAP! ().unwrap () (copy, core::ptr::null_mut (),
								 &raw mut gp_bitmap) == Ok
			  {
			    GdipSetInterpolationMode! ().unwrap () (graphics,
							  InterpolationModeHighQualityBilinear);
			    GdipDrawImageRectRectI! ().unwrap () (graphics,
							gp_bitmap, x, y,
							glyph_slice_bits! (width, (*s).slice)
							  as c_int,
							glyph_slice_bits! (height, (*s).slice)
							  as c_int,
							orig_slice_x,
							orig_slice_y,
							orig_slice_width,
							orig_slice_height,
							UnitPixel,
							core::ptr::null (),
							None, core::ptr::null_mut ());
			    GdipDisposeImage! ().unwrap () (gp_bitmap);
			  }
			DeleteObject (copy);
			GdipDeleteGraphics! ().unwrap () (graphics);
		      }
		  }
		else
		  {
		    let mut pmode: c_int = 0;
		    /* Windows 9X doesn't support HALFTONE.  */
		    if os_subtype == OS_SUBTYPE_NT
			&& {
			     pmode = SetStretchBltMode ((*s).hdc as HDC, HALFTONE);
			     pmode != 0
			   }
		      { SetBrushOrgEx ((*s).hdc as HDC, 0, 0, core::ptr::null_mut ()); }
		    StretchBlt ((*s).hdc as HDC, x, y,
				glyph_slice_bits! (width, (*s).slice) as c_int,
				glyph_slice_bits! (height, (*s).slice) as c_int,
				compat_hdc, orig_slice_x, orig_slice_y,
				orig_slice_width, orig_slice_height, SRCCOPY);
		    if pmode != 0
		      { SetStretchBltMode ((*s).hdc as HDC, pmode); }
		  }
	      }
	      #[cfg (not (feature = "windowsnt"))]
	      {
		let mut pmode: c_int = 0;
		/* Windows 9X doesn't support HALFTONE.  */
		if os_subtype == OS_SUBTYPE_NT
		    && {
			 pmode = SetStretchBltMode ((*s).hdc as HDC, HALFTONE);
			 pmode != 0
		       }
		  { SetBrushOrgEx ((*s).hdc as HDC, 0, 0, core::ptr::null_mut ()); }
		StretchBlt ((*s).hdc as HDC, x, y,
			    glyph_slice_bits! (width, (*s).slice) as c_int,
			    glyph_slice_bits! (height, (*s).slice) as c_int,
			    compat_hdc, orig_slice_x, orig_slice_y,
			    orig_slice_width, orig_slice_height, SRCCOPY);
		if pmode != 0
		  { SetStretchBltMode ((*s).hdc as HDC, pmode); }
	      }
	    }

	  /* When the image has a mask, we can expect that at
	     least part of a mouse highlight or a block cursor will
	     be visible.  If the image doesn't have a mask, make
	     a block cursor visible by drawing a rectangle around
	     the image.  I believe it's looking better if we do
	     nothing here for mouse-face.  */
	  if (*s).hl == DRAW_CURSOR
	    {
	      let mut r: c_int = (*(*s).img).relief;
	      if r < 0 { r = -r; }
	      w32_draw_rectangle ((*s).hdc as HDC, (*s).gc, x - r, y - r,
				  glyph_slice_bits! (width, (*s).slice) as c_int + r * 2 - 1,
				  glyph_slice_bits! (height, (*s).slice) as c_int + r * 2 - 1);
	    }
	}

      w32_set_clip_rectangle ((*s).hdc as HDC, core::ptr::null_mut ());
      SelectObject ((*s).hdc as HDC, orig_brush);
      DeleteObject (fg_brush);
      SelectObject (compat_hdc, orig_obj);
      DeleteDC (compat_hdc);
    }
  else
    { w32_draw_rectangle ((*s).hdc as HDC, (*s).gc, x, y,
			  glyph_slice_bits! (width, (*s).slice) as c_int - 1,
			  glyph_slice_bits! (height, (*s).slice) as c_int - 1); }

  RestoreDC ((*s).hdc as HDC, -1);
}

/* C 2192-2201: `w32_image_size` - the `GetObject`-reported size of PIXMAP in bytes.  C's
   `sizeof (BITMAP)` is the second argument, written out; C's `size_t rv = 0` is the zeroed local
   (PORTING.md 1.5) and the whole expression is C's arithmetic on `LONG`/`WORD` members widened to
   the `size_t` result.  The definition is the `w32gui.h` 53 declaration `include/w32gui.rs`
   carries inside `w32gui_fn_decls` (a crate-root definition shadows the glob-imported one).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_image_size (pixmap: Emacs_Pixmap) -> size_t
{
  let mut bm_info: BITMAP = core::mem::zeroed ();
  let mut rv: size_t = 0;

  if GetObject (pixmap, core::mem::size_of::<BITMAP> () as c_int,
	       &raw mut bm_info as *mut c_void) != 0
    {
      rv = (bm_info.bmWidth * bm_info.bmHeight * bm_info.bmBitsPixel as LONG / 8) as size_t;
    }
  return rv;
}


/* C 2204-2293: "Draw a relief around the image glyph string S."  C's three chained assignments
   (`extra_x = extra_y = 0`, `top_p = bot_p = left_p = right_p = 0`) are one statement per
   assignment (the recorded chained-assignment deviation), C's two comma expressions
   (`x -= thick + extra_x, left_p = 1` and its three siblings) are one statement per assignment in
   C's order, `raised_p` is C's `s->hl == DRAW_IMAGE_RAISED` (a `bool` here, so each use is `as
   c_int`), and `s->slice` again reads through `glyph_slice_bits!` with C's `unsigned` -> `int`
   conversions written out.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_image_relief (s: *mut glyph_string)
{
  let mut x1: c_int = 0;
  let mut y1: c_int = 0;
  let mut thick: c_int = 0;
  let mut raised_p: c_int = 0;
  let mut top_p: c_int = 0;
  let mut bot_p: c_int = 0;
  let mut left_p: c_int = 0;
  let mut right_p: c_int = 0;
  let mut extra_x: c_int = 0;
  let mut extra_y: c_int = 0;
  let mut r: RECT = core::mem::zeroed ();
  let mut x: c_int = (*s).x;
  let mut y: c_int
    = (*s).ybase - image_ascent ((*s).img, (*s).face as *mut face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max! ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*(*s).img).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*(*s).img).vmargin; }

  if (*s).hl == DRAW_IMAGE_SUNKEN
      || (*s).hl == DRAW_IMAGE_RAISED
    {
      if (*( (*s).face as *mut face )).id == TAB_BAR_FACE_ID
	{ thick = (if tab_bar_button_relief < 0
		     { DEFAULT_TAB_BAR_BUTTON_RELIEF }
		     else { min! (tab_bar_button_relief, 1000000) as c_int }); }
      else
	{ thick = (if tool_bar_button_relief < 0
		     { DEFAULT_TOOL_BAR_BUTTON_RELIEF }
		     else { min! (tool_bar_button_relief, 1000000) as c_int }); }
      raised_p = ((*s).hl == DRAW_IMAGE_RAISED) as c_int;
    }
  else
    {
      thick = eabs! ((*(*s).img).relief);
      raised_p = ((*(*s).img).relief > 0) as c_int;
    }

  x1 = x + glyph_slice_bits! (width, (*s).slice) as c_int - 1;
  y1 = y + glyph_slice_bits! (height, (*s).slice) as c_int - 1;

  extra_x = 0;
  extra_y = 0;
  if (*( (*s).face as *mut face )).id == TAB_BAR_FACE_ID
    {
      if CONSP (Vtab_bar_button_margin)
	  && FIXNUMP (XCAR (Vtab_bar_button_margin))
	  && FIXNUMP (XCDR (Vtab_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtab_bar_button_margin)) as c_int - thick;
	  extra_y = XFIXNUM (XCDR (Vtab_bar_button_margin)) as c_int - thick;
	}
      else if FIXNUMP (Vtab_bar_button_margin)
	{
	  extra_x = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;
	  extra_y = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;
	}
    }

  if (*( (*s).face as *mut face )).id == TOOL_BAR_FACE_ID
    {
      if CONSP (Vtool_bar_button_margin)
	  && FIXNUMP (XCAR (Vtool_bar_button_margin))
	  && FIXNUMP (XCDR (Vtool_bar_button_margin))
	{
	  extra_x = XFIXNUM (XCAR (Vtool_bar_button_margin)) as c_int;
	  extra_y = XFIXNUM (XCDR (Vtool_bar_button_margin)) as c_int;
	}
      else if FIXNUMP (Vtool_bar_button_margin)
	{
	  extra_x = XFIXNUM (Vtool_bar_button_margin) as c_int;
	  extra_y = XFIXNUM (Vtool_bar_button_margin) as c_int;
	}
    }

  top_p = 0;
  bot_p = 0;
  left_p = 0;
  right_p = 0;

  if glyph_slice_bits! (x, (*s).slice) == 0
    {
      x -= thick + extra_x; left_p = 1;
    }
  if glyph_slice_bits! (y, (*s).slice) == 0
    {
      y -= thick + extra_y; top_p = 1;
    }
  if glyph_slice_bits! (x, (*s).slice) + glyph_slice_bits! (width, (*s).slice)
     == (*(*s).img).width as c_uint
    {
      x1 += thick + extra_x; right_p = 1;
    }
  if glyph_slice_bits! (y, (*s).slice) + glyph_slice_bits! (height, (*s).slice)
     == (*(*s).img).height as c_uint
    {
      y1 += thick + extra_y; bot_p = 1;
    }

  w32_setup_relief_colors (s);
  get_glyph_string_clip_rect (s, &raw mut r);
  w32_draw_relief_rect ((*s).f, x, y, x1, y1, thick, thick, raised_p,
			top_p, bot_p, left_p, right_p, &raw mut r);
}


/* C 2296-2383: "Draw the foreground of image glyph string S to PIXMAP."  The same three
   `s->slice` reads and the two `Emacs_GC` members' `COLORREF` conversions as
   `w32_draw_image_foreground`; C's `x = 0` and its `y` are that function's pair offset by
   `s->y`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_image_foreground_1 (s: *mut glyph_string, pixmap: HBITMAP)
{
  let hdc: HDC = CreateCompatibleDC ((*s).hdc as HDC);
  let orig_hdc_obj: HGDIOBJ = SelectObject (hdc, pixmap);
  let mut x: c_int = 0;
  let mut y: c_int = (*s).ybase - (*s).y
    - image_ascent ((*s).img, (*s).face as *mut face, &raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))
      && glyph_slice_bits! (x, (*s).slice) == 0
    { x += max! ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */
  if glyph_slice_bits! (x, (*s).slice) == 0
    { x += (*(*s).img).hmargin; }
  if glyph_slice_bits! (y, (*s).slice) == 0
    { y += (*(*s).img).vmargin; }

  /* FIXME (maybe): The below doesn't support image scaling.  But it
     seems to never be called, because the conditions for its call in
     w32_draw_image_glyph_string are never fulfilled (they will be if
     the #ifdef'ed away part of that function is ever activated).  */
  if ! (*(*s).img).pixmap.is_null ()
    {
      let compat_hdc: HDC = CreateCompatibleDC (hdc);
      let fg_brush: HBRUSH = CreateSolidBrush ((*(*s).gc).foreground as COLORREF);
      let orig_brush: HBRUSH = SelectObject (hdc, fg_brush);
      let orig_obj: HGDIOBJ = SelectObject (compat_hdc, (*(*s).img).pixmap);

      if ! (*(*s).img).mask.is_null ()
	{
	  let mask_dc: HDC = CreateCompatibleDC (hdc);
	  let mask_orig_obj: HGDIOBJ = SelectObject (mask_dc, (*(*s).img).mask);

	  SetTextColor (hdc, RGB! (0, 0, 0));
	  SetBkColor (hdc, RGB! (255, 255, 255));
	  BitBlt (hdc, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		  glyph_slice_bits! (height, (*s).slice) as c_int,
		  compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
		  glyph_slice_bits! (y, (*s).slice) as c_int, SRCINVERT);
	  BitBlt (hdc, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		  glyph_slice_bits! (height, (*s).slice) as c_int,
		  mask_dc, glyph_slice_bits! (x, (*s).slice) as c_int,
		  glyph_slice_bits! (y, (*s).slice) as c_int, SRCAND);
	  BitBlt (hdc, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
		  glyph_slice_bits! (height, (*s).slice) as c_int,
		  compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
		  glyph_slice_bits! (y, (*s).slice) as c_int, SRCINVERT);

	  SelectObject (mask_dc, mask_orig_obj);
	  DeleteDC (mask_dc);
	}
      else
	{
	  SetTextColor (hdc, (*(*s).gc).foreground as COLORREF);
	  SetBkColor (hdc, (*(*s).gc).background as COLORREF);

          BitBlt (hdc, x, y, glyph_slice_bits! (width, (*s).slice) as c_int,
                  glyph_slice_bits! (height, (*s).slice) as c_int,
                  compat_hdc, glyph_slice_bits! (x, (*s).slice) as c_int,
                  glyph_slice_bits! (y, (*s).slice) as c_int, SRCCOPY);

	  /* When the image has a mask, we can expect that at
	     least part of a mouse highlight or a block cursor will
	     be visible.  If the image doesn't have a mask, make
	     a block cursor visible by drawing a rectangle around
	     the image.  I believe it's looking better if we do
	     nothing here for mouse-face.  */
	  if (*s).hl == DRAW_CURSOR
	    {
	      let mut r: c_int = (*(*s).img).relief;
	      if r < 0 { r = -r; }
	      w32_draw_rectangle (hdc, (*s).gc, x - r, y - r,
				  glyph_slice_bits! (width, (*s).slice) as c_int + r * 2 - 1,
				  glyph_slice_bits! (height, (*s).slice) as c_int + r * 2 - 1);
	    }
	}

      SelectObject (hdc, orig_brush);
      DeleteObject (fg_brush);
      SelectObject (compat_hdc, orig_obj);
      DeleteDC (compat_hdc);
    }
  else
    { w32_draw_rectangle (hdc, (*s).gc, x, y,
			  glyph_slice_bits! (width, (*s).slice) as c_int - 1,
			  glyph_slice_bits! (height, (*s).slice) as c_int - 1); }

  SelectObject (hdc, orig_hdc_obj);
  DeleteDC (hdc);
}


/* C 2386-2399: "Draw part of the background of glyph string S.  X, Y, W, and H give the rectangle
   to draw."  `s->stippled_p` is the record's bitfield run (`glyph_string_bits!`) and
   `w32_fill_stipple_pattern`'s last two parameters are C's `unsigned int`, so C's implicit
   conversion from the `int` arguments is written out.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyph_string_bg_rect (s: *mut glyph_string, x: c_int, y: c_int, w: c_int,
					 h: c_int)
{
  if glyph_string_bits! (stippled_p, (*s))
    {
      /* Fill background with a stipple pattern.  */
      w32_fill_stipple_pattern ((*s).hdc as HDC, s, (*s).gc, x, y, w as c_uint, h as c_uint);
    }
  else
    { w32_clear_glyph_string_rect (s, x, y, w, h); }
}


/* C 2402-2532: "Draw image glyph string S." and the picture of its geometry C carries above it (the
   `ASCII wave picture` reading of slice 1):

            s->y
   s->x      +-------------------------
	     |   s->face->box
	     |
	     |     +-------------------------
	     |     |  s->img->vmargin
	     |     |
	     |     |       +-------------------
	     |     |       |  the image

   `s->stippled_p`
   and `s->background_filled_p` are the record's bitfield run (the read and the write arms of
   `glyph_string_bits!`); the `#if 0` region C 2455-2494 is dead text in every configuration and is
   carried as C's own text; and `pixmap` keeps C's `HBITMAP pixmap = 0` as a null pointer.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_image_glyph_string (s: *mut glyph_string)
{
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let box_line_hwidth: c_int = max! ((*( (*s).face as *mut face )).box_vertical_line_width, 0);
  let box_line_vwidth: c_int = max! ((*( (*s).face as *mut face )).box_horizontal_line_width, 0);
  let mut height: c_int = 0;
  let mut width: c_int = 0;
  let mut pixmap: HBITMAP = 0 as HBITMAP;

  height = (*s).height;
  if glyph_slice_bits! (y, (*s).slice) == 0
    { height -= box_line_vwidth; }
  if glyph_slice_bits! (y, (*s).slice) + glyph_slice_bits! (height, (*s).slice)
     >= (*(*s).img).height as c_uint
    { height -= box_line_vwidth; }

  /* Fill background with face under the image.  Do it only if row is
     taller than image or if image has a clip mask to reduce
     flickering.  */
  glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);
  if height > glyph_slice_bits! (height, (*s).slice) as c_int
      || (*(*s).img).hmargin != 0
      || (*(*s).img).vmargin != 0
      || ! (*(*s).img).mask.is_null ()
      || (*(*s).img).pixmap.is_null ()
      || (*s).width != (*s).background_width
    {
      width = (*s).background_width;
      x = (*s).x;
      if glyph_bits! (left_box_line_p, (*(*s).first_glyph))
	  && glyph_slice_bits! (x, (*s).slice) == 0
	{
	  x += box_line_hwidth;
	  width -= box_line_hwidth;
	}

      y = (*s).y;
      if glyph_slice_bits! (y, (*s).slice) == 0
	{ y += box_line_vwidth; }

/* C 2455-2494 is `#if 0 /* TODO: figure out if we need to do this on Windows.  */` - the X11
   pixmap path C never compiles; C's own text:

#if 0 /* TODO: figure out if we need to do this on Windows.  */
      if (s->img->mask)
	{
	  /* Create a pixmap as large as the glyph string.  Fill it
	     with the background color.  Copy the image to it, using
	     its mask.  Copy the temporary pixmap to the display.  */
	  Screen *screen = FRAME_X_SCREEN (s->f);
	  int depth = DefaultDepthOfScreen (screen);

	  /* Create a pixmap as large as the glyph string.  */
	  pixmap = XCreatePixmap (s->display, FRAME_W32_WINDOW (s->f),
				  s->background_width,
				  s->height, depth);

	  /* Don't clip in the following because we're working on the
	     pixmap.  */
	  XSetClipMask (s->display, s->gc, None);

	  /* Fill the pixmap with the background color/stipple.  */
	  if (s->stippled_p)
	    {
	      /* Fill background with a stipple pattern.  */
	      XSetFillStyle (s->display, s->gc, FillOpaqueStippled);
	      XFillRectangle (s->display, pixmap, s->gc,
			      0, 0, s->background_width, s->height);
	      XSetFillStyle (s->display, s->gc, FillSolid);
	    }
	  else
	    {
	      XGCValues xgcv;
	      XGetGCValues (s->display, s->gc, GCForeground | GCBackground,
			    &xgcv);
	      XSetForeground (s->display, s->gc, xgcv.background);
	      XFillRectangle (s->display, pixmap, s->gc,
			      0, 0, s->background_width, s->height);
	      XSetForeground (s->display, s->gc, xgcv.foreground);
	    }
	}
      else
#endif
*/
	w32_draw_glyph_string_bg_rect (s, x, y, width, height);

      glyph_string_bits! (background_filled_p, (*s), true);
    }

  /* Draw the foreground.  */
  if ! pixmap.is_null ()
    {
      w32_draw_image_foreground_1 (s, pixmap);
      w32_set_glyph_string_clipping (s);
      {
        let compat_hdc: HDC = CreateCompatibleDC ((*s).hdc as HDC);
        let fg_brush: HBRUSH = CreateSolidBrush ((*(*s).gc).foreground as COLORREF);
        let orig_brush: HBRUSH = SelectObject ((*s).hdc as HDC, fg_brush);
        let orig_obj: HGDIOBJ = SelectObject (compat_hdc, pixmap);

        SetTextColor ((*s).hdc as HDC, (*(*s).gc).foreground as COLORREF);
        SetBkColor ((*s).hdc as HDC, (*(*s).gc).background as COLORREF);
        BitBlt ((*s).hdc as HDC, (*s).x, (*s).y, (*s).background_width, (*s).height,
                compat_hdc, 0, 0, SRCCOPY);

        SelectObject ((*s).hdc as HDC, orig_brush);
        DeleteObject (fg_brush);
        SelectObject (compat_hdc, orig_obj);
        DeleteDC (compat_hdc);
      }
      DeleteObject (pixmap);
      pixmap = 0 as HBITMAP;
    }
  else
    { w32_draw_image_foreground (s); }

  /* If we must draw a relief around the image, do it.  */
  if (*(*s).img).relief != 0
      || (*s).hl == DRAW_IMAGE_RAISED
      || (*s).hl == DRAW_IMAGE_SUNKEN
    { w32_draw_image_relief (s); }
}


/* C 2534-2631: "Draw stretch glyph string S."  `s->row`'s three members are the record's bitfield
   run (`glyph_row_bits!`, `include/dispextern.rs` 1107), `s->area` is C's `enum glyph_row_area`
   (C's `TEXT_AREA`), `FRAME_COLUMN_WIDTH` is the mirror's `pub fn` and C's `min`/`max` the pair
   declared above; `window_box_left_offset`, `window_box_left` and `cursor_in_mouse_face_p` are
   `dispextern.h` 3579/3580/3692's entry points, which no mirror carries, so the unit declares them
   at their use sites (the reading the five `dispextern.h` names above share).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_stretch_glyph_string (s: *mut glyph_string)
{
  eassert! (glyph_bits! (type, (*(*s).first_glyph)) == STRETCH_GLYPH);

  if (*s).hl == DRAW_CURSOR
      && ! x_stretch_cursor_p
    {
      /* If `x-stretch-cursor' is nil, don't draw a block cursor as
	 wide as the stretch glyph.  */
      let mut width: c_int = 0;
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;

      if ! glyph_row_bits! (reversed_p, (*s).row)
	{
	  let left_x: c_int = window_box_left_offset ((*s).w, TEXT_AREA);

	  if x < left_x
	    {
	      background_width -= left_x - x;
	      x = left_x;
	    }
	}
      else
	{
	  /* In R2L rows, draw the cursor on the right edge of the
	     stretch glyph.  */
	  let right_x: c_int = window_box_right ((*s).w, TEXT_AREA);

	  if x + background_width > right_x
	    { background_width -= x - right_x; }
	  x += background_width;
	}
      width = min! (FRAME_COLUMN_WIDTH ((*s).f), background_width);
      if glyph_row_bits! (reversed_p, (*s).row)
	{ x -= width; }

      /* Draw cursor.  */
      w32_draw_glyph_string_bg_rect (s, x, (*s).y, width, (*s).height);

      /* Clear rest using the GC of the original non-cursor face.  */
      if width < background_width
	{
	  let mut gc: *mut Emacs_GC = (*( (*s).face as *mut face )).gc;
	  let y: c_int = (*s).y;
	  let w: c_int = background_width - width;
	  let h: c_int = (*s).height;
	  let mut r: RECT = core::mem::zeroed ();
          let hdc: HDC = (*s).hdc as HDC;

	  if ! glyph_row_bits! (reversed_p, (*s).row)
	    { x += width; }
	  else
	    { x = (*s).x; }
	  if glyph_row_bits! (mouse_face_p, (*s).row)
	      && cursor_in_mouse_face_p ((*s).w)
	    {
	      w32_set_mouse_face_gc (s);
	      gc = (*s).gc;
	    }
	  else
	    { gc = (*( (*s).face as *mut face )).gc; }

	  get_glyph_string_clip_rect (s, &raw mut r);
	  w32_set_clip_rectangle (hdc, &raw mut r);

	  if (*( (*s).face as *mut face )).stipple != 0
	    {
	      /* Fill background with a stipple pattern.  */
	      w32_fill_stipple_pattern ((*s).hdc as HDC, s, gc, x, y, w as c_uint, h as c_uint);
	    }
	  else
            {
              w32_fill_area! ((*s).f, (*s).hdc as HDC, (*gc).background as COLORREF,
			      x, y, w, h);
            }
        }
    }
  else if ! glyph_string_bits! (background_filled_p, (*s))
    {
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;
      let text_left_x: c_int = window_box_left ((*s).w, TEXT_AREA);

      /* Don't draw into left fringe or scrollbar area except for
         header line and mode line.  */
      if (*s).area == TEXT_AREA
	  && x < text_left_x && ! glyph_row_bits! (mode_line_p, (*s).row)
	{
	  background_width -= text_left_x - x;
	  x = text_left_x;
	}
      if background_width > 0
	{ w32_draw_glyph_string_bg_rect (s, x, (*s).y, background_width, (*s).height); }
    }

  glyph_string_bits! (background_filled_p, (*s), true);
}

/* C 2633-2674: "Draw a dashed underline of thickness THICKNESS and width WIDTH onto F at a
   vertical offset of OFFSET …"  C's `char segment` is a `c_char` here, so every arithmetic use
   writes C's own promotion to `int` out (`as c_int`); `which` is C's `int` truth value, so
   `which = !which` is the recorded `(which == 0) as c_int`; and C's `for` header plus its
   comma-expression increment is the header's assignment plus a `while` whose last two statements
   are C's increment in C's order (`x += length` then `length = segment`).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_dash (f: *mut frame, s: *mut glyph_string, foreground: COLORREF,
			 width: c_int, segment: c_char, offset: c_int, thickness: c_int)
{
  let mut y_base: c_int = 0;
  let mut which: c_int = 0;
  let mut length: c_int = 0;
  let mut x: c_int = 0;
  let mut doffset: c_int = 0;
  let hdc: HDC = (*s).hdc as HDC;

  /* A pen with PS_DASH (or PS_DOT) is unsuitable for two reasons: first
     that PS_DASH does not accept width values greater than 1, with
     itself considered equivalent to PS_SOLID if such a value be
     specified, and second that it does not provide for an offset to be
     applied to the pattern, absent which Emacs cannot align dashes that
     are displayed at locations not multiples of each other.  I can't be
     bothered to research this matter further, so, for want of a better
     option, draw the specified pattern manually.  */

  y_base = (*s).ybase + offset;

  /* Remove redundant portions of OFFSET.  */
  doffset = (*s).x % (segment as c_int * 2);

  /* Set which to the phase of the first dash that ought to be drawn and
     length to its length.  */
  which = (doffset < segment as c_int) as c_int;
  length = segment as c_int - (*s).x % segment as c_int;

  /* Begin drawing this dash.  */
  x = (*s).x;
  while x < (*s).x + width
    {
      if which != 0
	{ w32_fill_area! (f, hdc, foreground, x, y_base, length, thickness); }

      which = (which == 0) as c_int;
      x += length; length = segment as c_int;
    }
}

/* C 2676-2715: "Draw an underline of STYLE onto F at an offset of POSITION from the baseline of
   the glyph string S, in the color FOREGROUND that is THICKNESS in height."  C's `switch` is the
   port's `match` over the same labels; the `FACE_UNDERLINE_DOTS` arm's `FALLTHROUGH` is recorded
   in the arm (PORTING.md 2.2, the `ccl.rs` 1535 reading), and C's `case FACE_NO_UNDERLINE:` /
   `case FACE_UNDERLINE_WAVE:` / `default:` run `emacs_abort` in either case.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_fill_underline (f: *mut frame, s: *mut glyph_string, foreground: COLORREF,
			      style: face_underline_type, position: c_int, thickness: c_int)
{
  let mut segment: c_int = 0;

  segment = thickness * 3;

  match style
    {
      /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
	 the second line will be filled by another invocation of this
	 function.  */
    FACE_UNDERLINE_SINGLE | FACE_UNDERLINE_DOUBLE_LINE =>
      {
	w32_fill_area! ((*s).f, (*s).hdc as HDC, foreground, (*s).x,
			(*s).ybase + position, (*s).width, thickness);
      }

    FACE_UNDERLINE_DOTS =>
      {
	segment = thickness;
	/* C's `FALLTHROUGH;` sends this arm on into the `FACE_UNDERLINE_DASHES` arm's
	   statement, which the port therefore writes here too (PORTING.md 2.2).  */
	w32_draw_dash (f, s, foreground, (*s).width, segment as c_char, position, thickness);
      }

    FACE_UNDERLINE_DASHES =>
      {
	w32_draw_dash (f, s, foreground, (*s).width, segment as c_char, position, thickness);
      }

    FACE_NO_UNDERLINE | FACE_UNDERLINE_WAVE =>
      { emacs_abort (); }
    _ =>
      { emacs_abort (); }
    }
}

/* C 2717-3038: "Draw glyph string S."  The file's glyph painter: C 2724-2744's successor-first
   background pass, C 2747's GC/clipping setup, C 2749-2772's relief-or-clip choice, C 2774-2811's
   `switch (s->first_glyph->type)`, and C 2813-3037's `if (!s->for_overlaps)` block - the relief,
   underline (with C 2838-2908's previous-style reuse and the two locale values C 2856-2873 reads),
   overline, strike-through and the two overhang-repair loops - ending with C 3035-3037's clip
   reset.  The sibling ports of the same algorithm are `src/xterm.rs`'s `x_draw_glyph_string` and
   `src/haikuterm.rs`'s, whose idioms this slice follows where the C text is the same.  */
/* C's `<font.h>` entry C 2852 calls: no mirror in this unit's graph carries it (the
   `w32font.h`/`font.h` declaration set the slices so far needed is narrower), so it stands at the
   use site (P-88 1) and the ledger records the gap.  */
/// C: struct font *font_for_underline_metrics (struct glyph_string *s);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn font_for_underline_metrics (s: *mut glyph_string) -> *mut font;
}

/// C: /* Draw glyph string S.  */
/// C: static void
/// C: w32_draw_glyph_string (struct glyph_string *s)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_glyph_string (s: *mut glyph_string)
{
  /* C reads `s->face` (a `struct face *`) through its own member access; the port binds it once
     (the `x_draw_glyph_string` reading) because the bitfield macros take the record's pointer.  */
  let face: *mut face = (*s).face as *mut face;
  let mut relief_drawn_p: bool = false;                              // C 2722

  /* If S draws into the background of its successor, draw the
     background of the successor first so that S can draw into it.
     This makes S->next use XDrawString instead of XDrawImageString.  */
  if !(*s).next.is_null () && (*s).right_overhang != 0               // C 2727
      && glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut width: c_int;                                          // C 2729
      let mut next: *mut glyph_string;                               // C 2730

      width = 0; next = (*s).next;                                   // C 2731
      while !next.is_null () && width < (*s).right_overhang          // C 2731-2732
	{
	  if glyph_bits! (type, (*(*next).first_glyph)) != IMAGE_GLYPH   // C 2734
	    {
	      w32_set_glyph_string_gc (next);                            // C 2736
	      w32_set_glyph_string_clipping (next);                      // C 2737
	      if glyph_bits! (type, (*(*next).first_glyph)) == STRETCH_GLYPH   // C 2738
		{ w32_draw_stretch_glyph_string (next); }                // C 2739
	      else
		{ w32_draw_glyph_string_background (next, true); }       // C 2741
	      (*next).num_clips = 0;                                     // C 2742
	    }
	  /* C's `for` increment clause (`width += next->width, next = next->next`).  */
	  width += (*next).width;                                        // C 2733
	  next = (*next).next;
	}
    }

  /* Set up S->gc, set clipping and draw S.  */
  w32_set_glyph_string_gc (s);                                       // C 2747

  /* Draw relief (if any) in advance for char/composition so that the
     glyph string can be drawn over it.  */
  if glyph_string_bits! (for_overlaps, (*s)) == 0                    // C 2751
      && face_bits! (box, face) != FACE_NO_BOX                       // C 2752
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH      // C 2753
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 2754
    {
      w32_set_glyph_string_clipping (s);                             // C 2757
      w32_draw_glyph_string_background (s, true);                    // C 2758
      w32_draw_glyph_string_box (s);                                 // C 2759
      w32_set_glyph_string_clipping (s);                             // C 2760
      relief_drawn_p = true;                                         // C 2761
    }
  else if (*s).clip_head.is_null ()                                  // C 2763
      && (*s).clip_tail.is_null ()                                   // C 2764
      && ((!(*s).prev.is_null () && (*(*s).prev).hl != (*s).hl
	   && (*s).left_overhang != 0)
	  || (!(*s).next.is_null () && (*(*s).next).hl != (*s).hl
	      && (*s).right_overhang != 0))                          // C 2765-2766
    /* We must clip just this glyph.  left_overhang part has already
       drawn when s->prev was drawn, and right_overhang part will be
       drawn later when s->next is drawn. */
    { w32_set_glyph_string_clipping_exactly (s, s); }                // C 2770
  else
    { w32_set_glyph_string_clipping (s); }                           // C 2772

  match glyph_bits! (type, (*(*s).first_glyph))                      // C 2774
    {
    IMAGE_GLYPH =>                                                   // C 2776
      { w32_draw_image_glyph_string (s); }                           // C 2777

    STRETCH_GLYPH =>                                                 // C 2780
      { w32_draw_stretch_glyph_string (s); }                         // C 2781

    CHAR_GLYPH =>                                                    // C 2784
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0               // C 2785
	  { glyph_string_bits! (background_filled_p, (*s), true); }  // C 2786
	else
	  { w32_draw_glyph_string_background (s, false); }           // C 2788
	w32_draw_glyph_string_foreground (s);                        // C 2789
      }

    COMPOSITE_GLYPH =>                                               // C 2792
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0               // C 2793
	    || ((*s).cmp_from > 0
		&& ! glyph_u_cmp_bits! (automatic, (*s).first_glyph))    // C 2794
	  { glyph_string_bits! (background_filled_p, (*s), true); }  // C 2795
	else
	  { w32_draw_glyph_string_background (s, true); }            // C 2797
	w32_draw_composite_glyph_string_foreground (s);              // C 2798
      }

    GLYPHLESS_GLYPH =>                                               // C 2801
      {
	if glyph_string_bits! (for_overlaps, (*s)) != 0               // C 2802
	  { glyph_string_bits! (background_filled_p, (*s), true); }  // C 2803
	else
	  { w32_draw_glyph_string_background (s, false); }           // C 2805
	w32_draw_glyphless_glyph_string_foreground (s);              // C 2806
      }

    /* C's `default:` runs `emacs_abort` for every other glyph type (C 2809-2810).  */
    _ =>
      { emacs_abort (); }                                            // C 2810
    }

  if glyph_string_bits! (for_overlaps, (*s)) == 0                    // C 2813
    {
      /* Draw relief if not yet drawn.  */
      if !relief_drawn_p && face_bits! (box, face) != FACE_NO_BOX    // C 2816
	{ w32_draw_glyph_string_box (s); }                           // C 2817

      /* Draw underline.  */
      if face_bits! (underline, face) != 0                           // C 2820
	{
	  if face_bits! (underline, face) == FACE_UNDERLINE_WAVE       // C 2822
	    {
	      let mut color: COLORREF;                                   // C 2824

	      if face_bits! (underline_defaulted_p, face)              // C 2826
		{ color = (*(*s).gc).foreground as COLORREF; }           // C 2827
	      else
		{ color = (*face).underline_color as COLORREF; }         // C 2829

	      w32_draw_underwave (s, color);                             // C 2831
	    }
	  else if face_bits! (underline, face) >= FACE_UNDERLINE_SINGLE   // C 2833
	    {
	      let mut thickness: c_ulong;                                // C 2835
	      let mut position: c_ulong;
	      let mut foreground: COLORREF;                              // C 2836

	      if !(*s).prev.is_null ()                                   // C 2838
		  && (face_bits! (underline, (*((*s).prev)).face as *mut face)
		      != FACE_UNDERLINE_WAVE
		      && face_bits! (underline, (*((*s).prev)).face as *mut face)
			 >= FACE_UNDERLINE_SINGLE)                       // C 2839-2840
		  && (face_bits! (underline_at_descent_line_p,
				  (*((*s).prev)).face as *mut face)
		      == face_bits! (underline_at_descent_line_p, face))   // C 2841-2842
		  && ((*( (*((*s).prev)).face as *mut face )).underline_pixels_above_descent_line
		      == (*face).underline_pixels_above_descent_line)   // C 2843-2844
		{
		  /* We use the same underline style as the previous one.  */
		  thickness = (*(*s).prev).underline_thickness as c_ulong;   // C 2847
		  position = (*(*s).prev).underline_position as c_ulong;     // C 2848
		}
	      else
		{
		  let font: *mut font = font_for_underline_metrics (s);  // C 2852
		  let mut minimum_offset: c_ulong;                        // C 2853
		  let mut underline_at_descent_line: bool;                // C 2854
		  let mut use_underline_position_properties: bool;        // C 2855
		  let mut val: Lisp_Object
		    = (WINDOW_BUFFER_LOCAL_VALUE (Qunderline_minimum_offset (),
						  (*s).w));              // C 2856-2857

		  if FIXNUMP (val)                                        // C 2859
		    { minimum_offset = max! (0, XFIXNUM (val) as c_int) as c_ulong; }   // C 2860
		  else
		    { minimum_offset = 1; }                              // C 2862

		  val = (WINDOW_BUFFER_LOCAL_VALUE (Qx_underline_at_descent_line (),
						    (*s).w));         // C 2864-2865
		  underline_at_descent_line
		    = (!(NILP (val) || BASE_EQ (val, Qunbound ()))
		       || face_bits! (underline_at_descent_line_p, face));   // C 2866-2868

		  val = (WINDOW_BUFFER_LOCAL_VALUE (Qx_use_underline_position_properties (),
						    (*s).w));         // C 2870-2871
		  use_underline_position_properties
		    = !(NILP (val) || BASE_EQ (val, Qunbound ()));       // C 2872-2873

		  /* Get the underline thickness.  Default is 1 pixel.  */
		  if !font.is_null () && (*font).underline_thickness > 0   // C 2876
		    { thickness = (*font).underline_thickness as c_ulong; }   // C 2877
		  else
		    { thickness = 1; }                                   // C 2879
		  if underline_at_descent_line || font.is_null ()        // C 2880-2881
		    {
		      position = (((*s).height - thickness as c_int)
				  - ((*s).ybase - (*s).y)
				  - (*face).underline_pixels_above_descent_line)
				 as c_ulong;                        // C 2882-2884
		    }
		  else
		    {
		      /* Get the underline position.  This is the
			 recommended vertical offset in pixels from
			 the baseline to the top of the underline.
			 This is a signed value according to the
			 specs, and its default is

			 ROUND ((maximum_descent) / 2), with
			 ROUND (x) = floor (x + 0.5)  */

		      if use_underline_position_properties             // C 2896
			  && (*font).underline_position >= 0
			{ position = (*font).underline_position as c_ulong; }   // C 2898
		      else
			{ position = (((*font).descent + 1) / 2) as c_ulong; }   // C 2900
		    }

		  if !(face_bits! (underline_at_descent_line_p, face)   // C 2903-2906
		       /* Ignore minimum_offset if the amount of pixels
			  was explicitly specified.  */
		       && (*face).underline_pixels_above_descent_line != 0)
		    { position = max! (position as isize, minimum_offset as isize) as c_ulong; }   // C 2907
		}
	      /* Check the sanity of thickness and position.  We should
		 avoid drawing underline out of the current line area.  */
	      if (((*s).y + (*s).height) as c_ulong)
		  <= ((*s).ybase as c_ulong).wrapping_add (position)   // C 2911
		{ position = (((*s).height - 1) - ((*s).ybase - (*s).y)) as c_ulong; }   // C 2912
	      if (((*s).y + (*s).height) as c_ulong)
		  < ((*s).ybase as c_ulong).wrapping_add (position).wrapping_add (thickness)   // C 2913
		{
		  thickness = (((*s).y + (*s).height) as c_ulong)
			      .wrapping_sub ((*s).ybase as c_ulong + position);   // C 2914
		}
	      (*s).underline_thickness = thickness as c_int;           // C 2915
	      (*s).underline_position = position as c_int;             // C 2916

	      if face_bits! (underline_defaulted_p, face)              // C 2918
		{ foreground = (*(*s).gc).foreground as COLORREF; }      // C 2919
	      else
		{ foreground = (*face).underline_color as COLORREF; }    // C 2921

	      w32_fill_underline ((*s).f, s, foreground,
				  face_bits! (underline, face),
				  position as c_int, thickness as c_int);   // C 2923-2924

	      /* Place a second underline above the first if this was
		 requested in the face specification.  */

	      if face_bits! (underline, face) == FACE_UNDERLINE_DOUBLE_LINE   // C 2929
		{
		  /* Compute the position of the second underline.  */
		  position = position.wrapping_sub (thickness).wrapping_sub (1);   // C 2932
		  w32_fill_underline ((*s).f, s, foreground,
				      face_bits! (underline, face),
				      position as c_int, thickness as c_int);   // C 2933-2934
		}
	    }
	}
      /* Draw overline.  */
      if face_bits! (overline_p, face)                               // C 2939
	{
	  let dy: c_ulong = 0;                                           // C 2941
	  let h: c_ulong = 1;

	  if face_bits! (overline_color_defaulted_p, face)             // C 2943
	    {
	      w32_fill_area! ((*s).f, (*s).hdc as HDC,
			      (*(*s).gc).foreground as COLORREF, (*s).x,
			      (*s).y + dy as c_int, (*s).width, h as c_int);   // C 2945-2946
	    }
	  else
	    {
	      w32_fill_area! ((*s).f, (*s).hdc as HDC,
			      (*face).overline_color as COLORREF, (*s).x,
			      (*s).y + dy as c_int, (*s).width, h as c_int);   // C 2950-2951
	    }
	}

      /* Draw strike-through.  */
      if face_bits! (strike_through_p, face)                         // C 2956
	  && FONT_TEXTMETRIC! ((*s).font).tmStruckOut == 0           // C 2957
	{
	  /* Y-coordinate and height of the glyph string's first
	     glyph.  We cannot use s->y and s->height because those
	     could be larger if there are taller display elements
	     (e.g., characters displayed with a larger font) in the
	     same glyph row.  */
	  let glyph_y: c_int
	    = (*s).ybase - (*(*s).first_glyph).ascent as c_int;      // C 2964
	  let glyph_height: c_int
	    = (*(*s).first_glyph).ascent as c_int
	      + (*(*s).first_glyph).descent as c_int;                // C 2965
	  /* Strike-through width and offset from the glyph string's
	     top edge.  */
	  let h: c_ulong = 1;                                        // C 2968
	  let dy: c_ulong = ((glyph_height as c_ulong).wrapping_sub (h)) / 2;   // C 2969

	  if face_bits! (strike_through_color_defaulted_p, face)     // C 2971
	    {
	      w32_fill_area! ((*s).f, (*s).hdc as HDC,
			      (*(*s).gc).foreground as COLORREF, (*s).x,
			      glyph_y + dy as c_int, (*s).width, h as c_int);   // C 2973-2974
	    }
	  else
	    {
	      w32_fill_area! ((*s).f, (*s).hdc as HDC,
			      (*face).strike_through_color as COLORREF, (*s).x,
			      glyph_y + dy as c_int, (*s).width, h as c_int);   // C 2978-2979
	    }
	}

      if !(*s).prev.is_null ()                                       // C 2983
	{
	  let mut prev: *mut glyph_string;                             // C 2985

	  prev = (*s).prev;                                            // C 2987
	  while !prev.is_null ()
	    {
	      if (*prev).hl != (*s).hl
		  && (*prev).x + (*prev).width + (*prev).right_overhang > (*s).x   // C 2988-2989
		{
		  /* As prev was drawn while clipped to its own area, we
		     must draw the right_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*prev).hl;             // C 2993

		  (*prev).hl = (*s).hl;                                // C 2995
		  w32_set_glyph_string_gc (prev);                      // C 2996
		  w32_set_glyph_string_clipping_exactly (s, prev);     // C 2997
		  if glyph_bits! (type, (*(*prev).first_glyph)) == CHAR_GLYPH   // C 2998
		    { w32_draw_glyph_string_foreground (prev); }       // C 2999
		  else
		    { w32_draw_composite_glyph_string_foreground (prev); }   // C 3001
		  w32_set_clip_rectangle ((*prev).hdc, core::ptr::null_mut ());   // C 3002
		  (*prev).hl = save;                                   // C 3003
		  (*prev).num_clips = 0;                               // C 3004
		}
	      prev = (*prev).prev;                                     // C 2987 (C's increment clause)
	    }
	}

      if !(*s).next.is_null ()                                       // C 3008
	{
	  let mut next: *mut glyph_string;                             // C 3010

	  next = (*s).next;                                            // C 3012
	  while !next.is_null ()
	    {
	      if (*next).hl != (*s).hl
		  && (*next).x - (*next).left_overhang < (*s).x + (*s).width   // C 3013-3014
		{
		  /* As next will be drawn while clipped to its own area,
		     we must draw the left_overhang part using s->hl now.  */
		  let save: draw_glyphs_face = (*next).hl;             // C 3018

		  (*next).hl = (*s).hl;                                // C 3020
		  w32_set_glyph_string_gc (next);                      // C 3021
		  w32_set_glyph_string_clipping_exactly (s, next);     // C 3022
		  if glyph_bits! (type, (*(*next).first_glyph)) == CHAR_GLYPH   // C 3023
		    { w32_draw_glyph_string_foreground (next); }       // C 3024
		  else
		    { w32_draw_composite_glyph_string_foreground (next); }   // C 3026
		  w32_set_clip_rectangle ((*next).hdc, core::ptr::null_mut ());   // C 3027
		  (*next).hl = save;                                   // C 3028
		  (*next).num_clips = 0;                               // C 3029
		  (*next).clip_head = (*s).next;                       // C 3030
		}
	      next = (*next).next;                                     // C 3012 (C's increment clause)
	    }
	}
    }

  /* Reset clipping.  */
  w32_set_clip_rectangle ((*s).hdc, core::ptr::null_mut ());          // C 3036
  (*s).num_clips = 0;                                                 // C 3037
}

/* C's `<windows.h>`/`<wingdi.h>` surface C 3049-3234 reads and this unit does not carry yet: the
   four GDI/user entries (`CreateRectRgn` and `EqualRgn` on `<wingdi.h>`, `FlashWindow`,
   `ScrollWindowEx` and `SW_INVALIDATE` on `<winuser.h>`, `Sleep` on `<winbase.h>`) and the two
   display-side declarations the tree's mirrors do not reach (`gui_clear_cursor`, dispextern.h
   3657, and `window_box`, dispextern.h 3570-3571).  Every name stands at the use site (P-88 (1));
   the SDK records are `UNVERIFIED` (no Windows SDK on this machine) and the two prototypes above
   are read from the reference's own headers.  */
/// C: HRGN WINAPI CreateRectRgn (int x1, int y1, int x2, int y2);
/// C: BOOL WINAPI EqualRgn (HRGN hrgn1, HRGN hrgn2);
/// C: BOOL WINAPI FlashWindow (HWND hWnd, BOOL bInvert);
/// C: int WINAPI ScrollWindowEx (HWND hWnd, int XAmount, int YAmount, const RECT *prcScroll,
/// C:   const RECT *prcClip, HRGN hrgnUpdate, LPRECT prcUpdate, UINT flags);
/// C: #define SW_INVALIDATE 0x0002
/// C: void WINAPI Sleep (DWORD dwMilliseconds);
/// C: extern void gui_clear_cursor (struct window *);        [dispextern.h 3657]
/// C: void window_box (struct window *, enum glyph_row_area, int *, int *, int *, int *);
/// C:                                                        [dispextern.h 3570-3571]
#[cfg (feature = "have_ntgui")]
pub const SW_INVALIDATE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn CreateRectRgn (x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> HRGN;
  pub fn EqualRgn (hrgn1: HRGN, hrgn2: HRGN) -> BOOL;
  pub fn FlashWindow (hWnd: HWND, bInvert: BOOL) -> BOOL;
  pub fn ScrollWindowEx (hWnd: HWND, XAmount: c_int, YAmount: c_int, prcScroll: *const RECT,
			 prcClip: *const RECT, hrgnUpdate: HRGN, prcUpdate: *mut RECT,
			 flags: UINT) -> c_int;
  pub fn Sleep (dwMilliseconds: DWORD);
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn gui_clear_cursor (w: *mut window);
  pub fn window_box (w: *mut window, area: glyph_row_area, box_x: *mut c_int,
		     box_y: *mut c_int, box_width: *mut c_int, box_height: *mut c_int);
}

/* C 3041-3054: "Shift display to make room for inserted glyphs." - the `get_frame_dc` /
   `BitBlt` / `release_frame_dc` trio C 3049-3053 writes.  */
/// C: /* Shift display to make room for inserted glyphs.   */
/// C: static void
/// C: w32_shift_glyphs_for_insert (struct frame *f, int x, int y,
/// C: 			     int width, int height, int shift_by)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_shift_glyphs_for_insert (f: *mut frame, x: c_int, y: c_int,
				       width: c_int, height: c_int, shift_by: c_int)
{
  let mut hdc: HDC;                                                  // C 3047

  hdc = get_frame_dc (f);                                            // C 3049
  BitBlt (hdc, x + shift_by, y, width, height,
	  hdc, x, y, SRCCOPY);                                       // C 3050-3051

  release_frame_dc (f, hdc);                                         // C 3053
}

/* C 3057-3067: "Delete N glyphs at the nominal cursor position.  Not implemented for X frames."
   - the same `FRAME_W32_P` guard and `emacs_abort` body as C 3123-3133 below.  */
/// C: /* Delete N glyphs at the nominal cursor position.  Not implemented
/// C:    for X frames.  */
/// C: static void
/// C: w32_delete_glyphs (struct frame *f, register int n)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_delete_glyphs (f: *mut frame, n: c_int)
{
  if ! FRAME_W32_P (f)                                               // C 3063
    { return; }                                                      // C 3064

  emacs_abort ();                                                    // C 3066
}

/* C 3070-3091: "Clear entire frame." - the cursor bookkeeping (C 3080), the input block and the
   two clears.  */
/// C: /* Clear entire frame.  */
/// C: static void
/// C: w32_clear_frame (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_clear_frame (f: *mut frame)
{
  if ! FRAME_W32_P (f)                                               // C 3075
    { return; }                                                      // C 3076

  /* Clearing the frame will erase any cursor, so mark them all as no
     longer visible.  */
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));         // C 3080

  block_input ();                                                    // C 3082

  w32_clear_window (f);                                              // C 3084

  /* We have to clear the scroll bars, too.  If we have changed
     colors or something like that, then they should be notified.  */
  w32_scroll_bar_clear (f);                                          // C 3088

  unblock_input ();                                                  // C 3090
}

/* C 3094-3117: "Make audible bell." - C's five-`FlashWindow` flash when `visible_bell` is set,
   the `w32_sys_ring_bell` fallback, and the input block C 3099/3116 wraps both in.  */
/// C: /* Make audible bell.  */
/// C: static void
/// C: w32_ring_bell (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_ring_bell (f: *mut frame)
{
  block_input ();                                                    // C 3099

  if FRAME_W32_P (f) && visible_bell                                 // C 3101
    {
      let mut i: c_int;                                              // C 3103
      let hwnd: HWND = FRAME_W32_WINDOW (f);                          // C 3104

      i = 0;                                                         // C 3106
      while i < 5
	{
	  FlashWindow (hwnd, TRUE);                                  // C 3108
	  Sleep (10);                                                // C 3109
	  i += 1;                                                    // C 3106 (C's increment clause)
	}
      FlashWindow (hwnd, FALSE);                                     // C 3111
    }
  else
    { w32_sys_ring_bell (f); }                                       // C 3114

  unblock_input ();                                                  // C 3116
}

/* C 3119-3133: the "Line Dance" banner (C 3119-3121) and C's insert/delete-lines entry, which
   this frame type never implements (the `emacs_abort` tail).  */
/// C 3119-3121: the section banner, carried as C's own text at its position.
/// C: /***********************************************************************
/// C: 			      Line Dance
/// C:  ***********************************************************************/
/// C 3123-3124's comment: "Perform an insert-lines or delete-lines operation, inserting N lines or
/// deleting -N lines at vertical position VPOS."
/// C: static void
/// C: w32_ins_del_lines (struct frame *f, int vpos, int n)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_ins_del_lines (f: *mut frame, vpos: c_int, n: c_int)
{
  if ! FRAME_W32_P (f)                                               // C 3129
    { return; }                                                      // C 3130

  emacs_abort ();                                                    // C 3132
}

/* C 3136-3235: "Scroll part of the display as described by RUN." - C 3150's `window_box`, the
   two bounding-box branches C 3156-3189 (each computing the copied height and the region C
   3175/3187 registers when double buffering is off), C 3191/3230's input block, C 3194's
   `gui_clear_cursor`, the BitBlt arm C 3195-3199 and the `ScrollWindowEx` arm C 3201-3228 with
   its dirty-region check (C 3218-3227) and C 3232-3234's `expect_dirty` release.  */
/// C: /* Scroll part of the display as described by RUN.  */
/// C: static void
/// C: w32_scroll_run (struct window *w, struct run *run)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_run (w: *mut window, run: *mut run)
{
  let f: *mut frame = XFRAME ((*w).frame);                           // C 3141
  let mut x: c_int;                                                  // C 3142
  let mut y: c_int;
  let mut width: c_int;
  let mut height: c_int;
  let mut from_y: c_int;
  let mut to_y: c_int;
  let mut bottom_y: c_int;
  let mut hdc: HDC;                                                  // C 3143
  let hwnd: HWND = FRAME_W32_WINDOW (f);                              // C 3144
  let mut expect_dirty: HRGN = core::ptr::null_mut ();                // C 3145

  /* Get frame-relative bounding box of the text display area of W,
     without mode lines.  Include in this box the left and right
     fringes of W.  */
  window_box (w, ANY_AREA, &raw mut x, &raw mut y, &raw mut width,
	      &raw mut height);                                      // C 3150

  from_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).current_y);            // C 3152
  to_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).desired_y);              // C 3153
  bottom_y = y + height;                                             // C 3154

  if to_y < from_y                                                    // C 3156
    {
      /* Scrolling up.  Make sure we don't copy part of the mode
	 line at the bottom.  */
      if from_y + (*run).height > bottom_y                            // C 3160
	{ height = bottom_y - from_y; }                               // C 3161
      else
	{ height = (*run).height; }                                   // C 3163

      /* Don't draw over the display parts above if destination is off
         the top of the current window.  */
      if to_y < y                                                     // C 3167
	{
	  let d: c_int = y - to_y;                                    // C 3169
	  height -= d;                                                // C 3170
	  to_y += d;                                                  // C 3171
	  from_y += d;                                                // C 3172
	}

      if w32_disable_double_buffering                                 // C 3175
	{ expect_dirty = CreateRectRgn (x, y + height, x + width, bottom_y); }   // C 3176
    }
  else
    {
      /* Scrolling down.  Make sure we don't copy over the mode line.
	 at the bottom.  */
      if to_y + (*run).height > bottom_y                              // C 3182
	{ height = bottom_y - to_y; }                                 // C 3183
      else
	{ height = (*run).height; }                                   // C 3185

      if w32_disable_double_buffering                                 // C 3187
	{ expect_dirty = CreateRectRgn (x, y, x + width, to_y); }     // C 3188
    }

  block_input ();                                                     // C 3191

  /* Cursor off.  Will be switched on again in gui_update_window_end.  */
  gui_clear_cursor (w);                                               // C 3194
  if !w32_disable_double_buffering                                    // C 3195
    {
      hdc = get_frame_dc (f);                                         // C 3197
      BitBlt (hdc, x, to_y, width, height, hdc, x, from_y, SRCCOPY);   // C 3198
      release_frame_dc (f, hdc);                                      // C 3199
    }
  else
    {
      /* C declares both `RECT`s uninitialised and fills every member below
	 (PORTING.md 1.5's reading); Rust has to start them zeroed.  */
      let mut from: RECT = core::mem::zeroed ();                      // C 3203
      let mut to: RECT = core::mem::zeroed ();                        // C 3204
      let dirty: HRGN = CreateRectRgn (0, 0, 0, 0);                   // C 3205
      let combined: HRGN = CreateRectRgn (0, 0, 0, 0);                // C 3206

      /* C's chained `from.left = to.left = x;` (and the `from.right = to.right = x + width;`
	 below) assign the inner target first, so the port writes the two statements in that
	 order.  */
      to.left = x;                                                    // C 3208
      from.left = x;
      to.right = x + width;                                           // C 3209
      from.right = x + width;
      from.top = from_y;                                              // C 3210
      from.bottom = from_y + height;                                  // C 3211
      to.top = y;                                                     // C 3212
      to.bottom = bottom_y;                                           // C 3213

      ScrollWindowEx (hwnd, 0, to_y - from_y, &raw const from, &raw const to, dirty,
		      core::ptr::null_mut (), SW_INVALIDATE);         // C 3215-3216

      /* Combine this with what we expect to be dirty. This covers the
	 case where not all of the region we expect is actually dirty.  */
      CombineRgn (combined, dirty, expect_dirty, RGN_OR);             // C 3220

      /* If the dirty region is not what we expected, redraw the entire frame.  */
      if EqualRgn (combined, expect_dirty) == 0                       // C 3223
	{ SET_FRAME_GARBAGED! (f); }                                  // C 3224

      DeleteObject (dirty as HGDIOBJ);                                // C 3226
      DeleteObject (combined as HGDIOBJ);                             // C 3227
    }

  unblock_input ();                                                   // C 3230

  if w32_disable_double_buffering
      && !expect_dirty.is_null ()                                     // C 3232-3233
    { DeleteObject (expect_dirty as HGDIOBJ); }                       // C 3234
}

/* C's `<windows.h>`/`<winnls.h>` surface C 3246-3428 reads and this unit does not carry yet: the
   two focus messages, the key-name entry, the locale entries C 3427 reads and the CRT's `atoi`;
   plus `gui_update_cursor` (dispextern.h 3656), which no mirror in this unit's graph declares.
   Every name stands at the use site (P-88 (1)); the SDK records are UNVERIFIED.  */
/// C: #define WM_SETFOCUS 0x0007
/// C: #define WM_KILLFOCUS 0x0008
/// C: WINUSERAPI int WINAPI GetKeyNameTextA (LONG lParam, LPSTR lpString, int cchSize);
/// C: typedef DWORD LCID; typedef DWORD LCTYPE;
/// C: #define LOCALE_IDEFAULTANSICODEPAGE 0x00001004
/// C: #define CP_ACP 0
/// C: WINBASEAPI int WINAPI GetLocaleInfoA (LCID Locale, LCTYPE LCType, LPSTR lpLCData, int cchData);
/// C: int atoi (const char *);
/// C: extern void gui_update_cursor (struct frame *, bool);        [dispextern.h 3656]
#[cfg (feature = "have_ntgui")]
pub type LCID = DWORD;
#[cfg (feature = "have_ntgui")]
pub type LCTYPE = DWORD;
#[cfg (feature = "have_ntgui")]
pub const WM_SETFOCUS: UINT = 0x0007;
#[cfg (feature = "have_ntgui")]
pub const WM_KILLFOCUS: UINT = 0x0008;
#[cfg (feature = "have_ntgui")]
pub const LOCALE_IDEFAULTANSICODEPAGE: LCTYPE = 0x00001004;
#[cfg (feature = "have_ntgui")]
pub const CP_ACP: UINT = 0;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetKeyNameText (lParam: LONG, lpString: LPSTR, cchSize: c_int) -> c_int;
  pub fn GetLocaleInfo (Locale: LCID, LCType: LCTYPE, lpLCData: LPSTR, cchData: c_int) -> c_int;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn atoi (s: *const c_char) -> c_int;
  pub fn gui_update_cursor (f: *mut frame, arg: bool);
}

/* frame.h 405/408's two bit members this slice reads (`auto_raise`, `auto_lower`).  The mirror
   declares the shifts (`include/frame.rs` 463-479, per configuration) but the accessors live in
   the units that read them (`src/xterm.rs` 22696-22700, `src/haikuterm.rs` 722-728), so the two
   read arms stand here at the use site.  */
/// C: bool_bf auto_raise : 1;   bool_bf auto_lower : 1;   (frame.h 405/408)
/* termhooks.h 375-380's event record carries `kind` and `part` in one 32-bit unit
   (`include/termhooks.rs` 116's `bits: u32`, its own recorded deviation), so the two writes C
   3301/3318 makes go through the accessor this tree spells per unit (`src/xterm.rs` 22985-22999's
   copy, `src/androidterm.rs` 259's).  Only the `kind` write arm is needed here; the masks are
   `include/termhooks.rs`'s.  */
/// C: enum input_event_kind kind : 16; unsigned part : 16;   (termhooks.h 377-378)
#[cfg (feature = "have_ntgui")]
macro_rules! input_event_bits {
  /* The read arm slice 16's `w32_read_socket` needs - C 6355/6362 compare the field and C 6085
     reads a queued event's - which is `src/keyboard.rs` 4063's copy of the same pair.  */
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK)
      | (($v) as u32 & INPUT_EVENT_KIND_MASK); };
  /* The `part` write arm slices 14's two scroll-bar click handlers need (termhooks.h 377-378's
     16+16-bit unit; the `src/xterm.rs` 1650-1658 copy of the same pair).  */
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
      | ((($v) as u32 & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

#[cfg (feature = "have_ntgui")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "have_ntgui")]
macro_rules! frame_bits {
  (auto_raise, $f:expr) => { (((*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT) & 1) != 0 };
  (auto_lower, $f:expr) => { (((*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT) & 1) != 0 };
  /* The two `mouse_moved` arms slices 9's text reads and writes (frame.h 434's `bool_bf
     mouse_moved : 1`; the `src/xterm.rs` 22681/22696-22700 copy of the same pair).  */
  (mouse_moved, $f:expr) => { (((*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT) & 1) != 0 };
  (mouse_moved, $f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
  /* frame.h 494's `bool_bf after_make_frame : 1`, which slice 20's `w32_set_window_size` reads
     (C 7198).  Arm A puts it in unit 2 at bit 14 (`docs/baseline/frame-bits.tsv`), so the read is
     the mirror's `frame_field` - the `want_fullscreen!` shape above.  */
  (after_make_frame, $f:expr) =>
    { frame_field ($f, FRAME_AFTER_MAKE_FRAME_UNIT, FRAME_AFTER_MAKE_FRAME_SHIFT, 1) != 0 };
}

/* C 3239-3241 is the "Exposure Events" banner, carried as C's own text at its position.  */
/// C: /***********************************************************************
/// C: 			   Exposure Events
/// C:  ***********************************************************************/

/// C 3243-3248: `static void w32_frame_highlight (struct frame *f)` - the two-call body (the
/// cursor update and the alpha re-read C's `w32_set_frame_alpha` redoes; the same body is C
/// 3250-3255's `w32_frame_unhighlight`).
/// C: static void
/// C: w32_frame_highlight (struct frame *f)
/// C:   gui_update_cursor (f, 1); w32_set_frame_alpha (f);   (the body's two calls)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_highlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                       // C 3246
  w32_set_frame_alpha (f);                                           // C 3247
}

/// C 3250-3255: `static void w32_frame_unhighlight (struct frame *f)`.
/// C: static void
/// C: w32_frame_unhighlight (struct frame *f)
/// C:   gui_update_cursor (f, 1); w32_set_frame_alpha (f);   (the body's two calls)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_unhighlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                       // C 3253
  w32_set_frame_alpha (f);                                           // C 3254
}

/// C 3257-3284: "The focus has changed. …" - C 3266's saved focus, C 3268-3281's update (the
/// `auto_lower`/`auto_raise` reads and the pending-autoraise record) and C 3283's
/// `w32_frame_rehighlight_1`.
/// C: /* The focus has changed.  Update the frames as necessary to reflect
/// C:    the new situation.  Note that we can't change the selected frame
/// C:    here, because the Lisp code we are interrupting might become confused.
/// C:    Each event gets marked with the frame in which it occurred, so the
/// C:    Lisp code can tell when the switch took place by examining the events.  */
/// C: static void
/// C: w32_new_focus_frame (struct w32_display_info *dpyinfo, struct frame *frame)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_new_focus_frame (dpyinfo: *mut w32_display_info, frame: *mut frame)
{
  let old_focus: *mut frame = (*dpyinfo).w32_focus_frame;            // C 3266

  if frame != (*dpyinfo).w32_focus_frame                              // C 3268
    {
      /* Set this before calling other routines, so that they see
	 the correct value of w32_focus_frame.  */
      (*dpyinfo).w32_focus_frame = frame;                             // C 3272

      if !old_focus.is_null () && frame_bits! (auto_lower, old_focus)   // C 3274
	{ w32_lower_frame (old_focus); }                              // C 3275

      if !(*dpyinfo).w32_focus_frame.is_null ()
	  && frame_bits! (auto_raise, (*dpyinfo).w32_focus_frame)      // C 3277
	{ (*dpyinfo).w32_pending_autoraise_frame = (*dpyinfo).w32_focus_frame; }   // C 3278
      else
	{ (*dpyinfo).w32_pending_autoraise_frame = core::ptr::null_mut (); }   // C 3280
    }

  w32_frame_rehighlight_1 (dpyinfo);                                  // C 3283
}

/// C 3287-3324: "Handle FocusIn and FocusOut state changes for FRAME. …" - the `WM_SETFOCUS` arm
/// (C 3297-3307, with C 3301-3302's event fill and C 3305's `focus_state` set) and the
/// `WM_KILLFOCUS` arm (C 3313-3322, which clears the bit and the focus-event frame).
/// C: /* Handle FocusIn and FocusOut state changes for FRAME.
/// C:    If FRAME has focus and there exists more than one frame, puts
/// C:    a FOCUS_IN_EVENT into *BUFP.  */
/// C: static void
/// C: w32_focus_changed (int type, int state, struct w32_display_info *dpyinfo,
/// C: 		 struct frame *frame, struct input_event *bufp)
/// C:   dpyinfo->w32_focus_event_frame = 0;
/// C:   w32_new_focus_frame (dpyinfo, 0);
#[cfg (feature = "have_ntgui")]
unsafe fn w32_focus_changed (type_: c_int, state: c_int, dpyinfo: *mut w32_display_info,
			     frame: *mut frame, bufp: *mut input_event)
{
  if type_ == WM_SETFOCUS as c_int                                   // C 3295
    {
      if (*dpyinfo).w32_focus_event_frame != frame                   // C 3297
	{
	  w32_new_focus_frame (dpyinfo, frame);                       // C 3299
	  (*dpyinfo).w32_focus_event_frame = frame;                   // C 3300
	  input_event_bits! (kind, bufp, FOCUS_IN_EVENT);            // C 3301
	  XSETFRAME! ((*bufp).frame_or_window, frame);                // C 3302
	}

      (*( (*frame).output_data.x as *mut x_output )).focus_state |= state;   // C 3305

      /* TODO: IME focus?  */
    }
  else if type_ == WM_KILLFOCUS as c_int                             // C 3309
    {
      (*( (*frame).output_data.x as *mut x_output )).focus_state &= !state;   // C 3311

      if (*dpyinfo).w32_focus_event_frame == frame                    // C 3313
	{
	  (*dpyinfo).w32_focus_event_frame = core::ptr::null_mut ();    // C 3315
	  w32_new_focus_frame (dpyinfo, core::ptr::null_mut ());        // C 3316

	  input_event_bits! (kind, bufp, FOCUS_OUT_EVENT);           // C 3318
	  XSETFRAME! ((*bufp).frame_or_window, frame);                // C 3319
	}

      /* TODO: IME focus?  */
    }
}

/// C 3327-3347: "The focus may have changed. …" - C 3338's frame lookup, C 3339-3340's guard and
/// C 3343-3346's call with C 3344-3345's `? :` choosing the focus kind.
/// C: /* The focus may have changed.  Figure out if it is a real focus change,
/// C:    by checking both FocusIn/Out and Enter/LeaveNotify events.
/// C:
/// C:    Returns FOCUS_IN_EVENT event in *BUFP. */
/// C: static void
/// C: w32_detect_focus_change (struct w32_display_info *dpyinfo, W32Msg *event,
/// C: 			 struct input_event *bufp)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_detect_focus_change (dpyinfo: *mut w32_display_info, event: *mut W32Msg,
				   bufp: *mut input_event)
{
  let frame: *mut frame;                                              // C 3336

  frame = w32_window_to_frame (dpyinfo, (*event).msg.hwnd);           // C 3338
  if frame.is_null ()                                                 // C 3339
    { return; }                                                       // C 3340

  /* On w32, this is only called from focus events, so no switch needed.  */
  w32_focus_changed ((*event).msg.message as c_int,                   // C 3343
		     if (*event).msg.message == WM_KILLFOCUS          // C 3344
		       { FOCUS_IMPLICIT } else { FOCUS_EXPLICIT },    // C 3345
		     dpyinfo, frame, bufp);                           // C 3346
}

/* C 3350-3358: the `#if 0 /* unused */` block (`w32_mouse_leave`), carried as C's own text
   (P-51; C itself excludes it in every configuration).
   #if 0	/* unused */
     /* Handle an event saying the mouse has moved out of an Emacs frame.  */

     static void
     w32_mouse_leave (struct w32_display_info *dpyinfo)
     {
       w32_new_focus_frame (dpyinfo, dpyinfo->w32_focus_event_frame);
     }
   #endif  */

/// C 3360-3374: "The focus has changed, or we have redirected a frame's focus …" - the
/// `FRAME_W32_P` guard and the `FRAME_DISPLAY_INFO` call.
/// C: /* The focus has changed, or we have redirected a frame's focus to
/// C:    another frame (this happens when a frame uses a surrogate
/// C:    mini-buffer frame).  Shift the highlight as appropriate.
/// C:
/// C:    The FRAME argument doesn't necessarily have anything to do with which
/// C:    frame is being highlighted or un-highlighted; we only use it to find
/// C:    the appropriate X display info.  */
/// C: static void
/// C: w32_frame_rehighlight (struct frame *frame)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_rehighlight (frame: *mut frame)
{
  if !FRAME_W32_P (frame)                                            // C 3371
    { return; }                                                      // C 3372
  w32_frame_rehighlight_1 (FRAME_DISPLAY_INFO! (frame));              // C 3373
}

/// C 3376-3403: `static void w32_frame_rehighlight_1 (struct w32_display_info *dpyinfo)` - C
/// 3379's saved highlight, C 3381-3394's choice (C 3384-3386's `FRAMEP (FRAME_FOCUS_FRAME …)`
/// test with the dead-frame repair C 3387-3391 runs) and C 3396-3402's two re-highlight calls.
/// C: static void
/// C: w32_frame_rehighlight_1 (struct w32_display_info *dpyinfo)
/// C:   dpyinfo->highlight_frame = 0;
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_rehighlight_1 (dpyinfo: *mut w32_display_info)
{
  let old_highlight: *mut frame = (*dpyinfo).highlight_frame;         // C 3379

  if !(*dpyinfo).w32_focus_frame.is_null ()                           // C 3381
    {
      (*dpyinfo).highlight_frame
	= (if FRAMEP (FRAME_FOCUS_FRAME ((*dpyinfo).w32_focus_frame)) // C 3384
	     { XFRAME (FRAME_FOCUS_FRAME ((*dpyinfo).w32_focus_frame)) }   // C 3385
	   else
	     { (*dpyinfo).w32_focus_frame });                         // C 3386
      if !FRAME_LIVE_P ((*dpyinfo).highlight_frame)                   // C 3387
	{
	  fset_focus_frame ((*dpyinfo).w32_focus_frame, Qnil ());     // C 3389
	  (*dpyinfo).highlight_frame = (*dpyinfo).w32_focus_frame;    // C 3390
	}
    }
  else
    { (*dpyinfo).highlight_frame = core::ptr::null_mut (); }          // C 3394

  if (*dpyinfo).highlight_frame != old_highlight                      // C 3396
    {
      if !old_highlight.is_null ()                                    // C 3398
	{ w32_frame_unhighlight (old_highlight); }                    // C 3399
      if !(*dpyinfo).highlight_frame.is_null ()                       // C 3400
	{ w32_frame_highlight ((*dpyinfo).highlight_frame); }         // C 3401
    }
}

/* C 3405-3420: "Keyboard processing - modifier keys, etc." with "Convert a keysym to its name."
   - C's `static char value[100]` is a function-local static (the `sys_select` reading), and the
   entry is non-`static`, so it is exported (PORTING.md 0.2).  */
/// C: /* Keyboard processing - modifier keys, etc. */
/// C: /* Convert a keysym to its name.  */
/// C: char *
/// C: get_keysym_name (int keysym)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_keysym_name (keysym: c_int) -> *mut c_char
{
  /* Make static so we can always return it */
  static mut value: [c_char; 100] = [0; 100];                         // C 3413

  block_input ();                                                    // C 3415
  GetKeyNameText (keysym, value.as_mut_ptr (), 100);                 // C 3416
  unblock_input ();                                                  // C 3417

  return value.as_mut_ptr ();                                        // C 3419
}

/// C 3422-3431: `static int codepage_for_locale (LCID locale)` - C 3425's twenty-byte buffer, the
/// `GetLocaleInfo` read of `LOCALE_IDEFAULTANSICODEPAGE` and C 3428/3430's two answers.
/// C: static int
/// C: codepage_for_locale (LCID locale)
#[cfg (feature = "have_ntgui")]
unsafe fn codepage_for_locale (locale: LCID) -> c_int
{
  let mut cp: [c_char; 20] = [0; 20];                                 // C 3425

  if GetLocaleInfo (locale, LOCALE_IDEFAULTANSICODEPAGE, cp.as_mut_ptr (), 20) > 0   // C 3427
    { return atoi (cp.as_ptr ()); }                                   // C 3428
  else
    { return CP_ACP as c_int; }                                       // C 3430
}

/* C's `<windows.h>` surface C 3445-3624 reads and this unit does not carry yet, plus the `MSG`
   record `W32Msg`'s `msg` member names (the mirror's `W32Msg` leaves it to its includer, the
   `src/w32fns.rs` 379-388 / `src/w32proc.rs` slice-25 reading).  Every name stands at the use
   site (P-88 (1)); the SDK records are UNVERIFIED.  */
/// C: #define WM_LBUTTONDOWN 0x0201 / #define WM_LBUTTONUP 0x0202
/// C: #define WM_RBUTTONDOWN 0x0204 / #define WM_RBUTTONUP 0x0205
/// C: #define WM_MBUTTONDOWN 0x0207 / #define WM_MBUTTONUP 0x0208
/// C: #define WHEEL_DELTA 120
/// C: WINUSERAPI BOOL WINAPI ScreenToClient (HWND hWnd, LPPOINT lpPoint);
/// C: typedef struct tagMSG { HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam; DWORD time;
/// C:   POINT pt; } MSG;
#[cfg (feature = "have_ntgui")]
pub const WM_LBUTTONDOWN: c_int = 0x0201;
#[cfg (feature = "have_ntgui")]
pub const WM_LBUTTONUP: c_int = 0x0202;
#[cfg (feature = "have_ntgui")]
pub const WM_RBUTTONDOWN: c_int = 0x0204;
#[cfg (feature = "have_ntgui")]
pub const WM_RBUTTONUP: c_int = 0x0205;
#[cfg (feature = "have_ntgui")]
pub const WM_MBUTTONDOWN: c_int = 0x0207;
#[cfg (feature = "have_ntgui")]
pub const WM_MBUTTONUP: c_int = 0x0208;
#[cfg (feature = "have_ntgui")]
pub const WHEEL_DELTA: c_int = 120;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG
{
  pub hwnd: HWND,
  pub message: UINT,
  pub wParam: WPARAM,
  pub lParam: LPARAM,
  pub time: DWORD,
  pub pt: POINT,
}
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn ScreenToClient (hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
}

/* C's `<windef.h>` macro the two event constructors call: no mirror in this unit's graph declares
   it (the unit's own `HIWORD` at C 79-82 is the other half), so it stands here.  */
/// C: #define LOWORD(l) ((WORD) (((DWORD_PTR) (l)) & 0xffff))
#[cfg (feature = "have_ntgui")]
macro_rules! LOWORD {
  ($l:expr) => { (($l) as DWORD_PTR & 0xffff) as WORD };
}

/* C 3433-3434: "Mouse clicks and mouse movement.  Rah." - the section comment, carried at its
   position.  */
/// C: /* Mouse clicks and mouse movement.  Rah.  */

/// C 3436-3499: "Parse a button MESSAGE. …" - C 3442-3443's two locals, C 3445-3493's
/// `switch (message)` over the eight message labels (with C 3456-3473's two `Vw32_swap_mouse_buttons`
/// branches), C 3491-3492's `default:` (which returns `FALSE`) and C 3495-3498's two
/// write-through-pointer stores and `TRUE` answer.  The entry is non-`static`, so it is exported
/// (PORTING.md 0.2).
/// C: /* Parse a button MESSAGE. The button index is returned in PBUTTON, and
/// C:    the state in PUP. XBUTTON provides extra information for extended mouse
/// C:    button messages. Returns FALSE if unable to parse the message.  */
/// C: BOOL
/// C: parse_button (int message, int xbutton, int * pbutton, int * pup)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_button (message: c_int, xbutton: c_int, pbutton: *mut c_int,
				       pup: *mut c_int) -> BOOL
{
  let mut button: c_int = 0;                                         // C 3442
  let mut up: c_int = 0;                                             // C 3443

  match message                                                      // C 3445
    {
    WM_LBUTTONDOWN =>                                                // C 3447
      {
	button = 0; up = 0;                                          // C 3448-3449
      }

    WM_LBUTTONUP =>                                                  // C 3451
      {
	button = 0; up = 1;                                          // C 3452-3453
      }

    WM_MBUTTONDOWN =>                                                // C 3455
      {
	if NILP (Vw32_swap_mouse_buttons)                         // C 3456
	  { button = 1; }                                            // C 3457
	else
	  { button = 2; }                                            // C 3459
	up = 0;                                                      // C 3460
      }

    WM_MBUTTONUP =>                                                  // C 3462
      {
	if NILP (Vw32_swap_mouse_buttons)                         // C 3463
	  { button = 1; }                                            // C 3464
	else
	  { button = 2; }                                            // C 3466
	up = 1;                                                      // C 3467
      }

    WM_RBUTTONDOWN =>                                                // C 3469
      {
	if NILP (Vw32_swap_mouse_buttons)                         // C 3470
	  { button = 2; }                                            // C 3471
	else
	  { button = 1; }                                            // C 3473
	up = 0;                                                      // C 3474
      }

    WM_RBUTTONUP =>                                                  // C 3476
      {
	if NILP (Vw32_swap_mouse_buttons)                         // C 3477
	  { button = 2; }                                            // C 3478
	else
	  { button = 1; }                                            // C 3480
	up = 1;                                                      // C 3481
      }

    WM_XBUTTONDOWN =>                                                // C 3483
      {
	button = xbutton + 2; up = 0;                                // C 3484-3485
      }

    WM_XBUTTONUP =>                                                  // C 3487
      {
	button = xbutton + 2; up = 1;                                // C 3488-3489
      }

    /* C's `default:` returns FALSE for every other message (C 3491-3492).  */
    _ =>
      { return FALSE; }                                              // C 3492
    }

  if !pup.is_null () { *pup = up; }                                  // C 3495
  if !pbutton.is_null () { *pbutton = button; }                      // C 3496

  return TRUE;                                                       // C 3498
}

/// C 3501-3532: "Prepare a mouse-event in *RESULT for placement in the input queue. …" - C
/// 3511-3512's two locals, C 3514-3515's `parse_button` (with C 3514's `HIWORD` of the `wParam`),
/// C 3519-3525's event fill and C 3527-3530's four setters.
/// C: /* Prepare a mouse-event in *RESULT for placement in the input queue.
/// C:
/// C:    If the event is a button press, then note that we have grabbed
/// C:    the mouse.  */
/// C: static Lisp_Object
/// C: w32_construct_mouse_click (struct input_event *result, W32Msg *msg,
/// C:                            struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_construct_mouse_click (result: *mut input_event, msg: *mut W32Msg,
				     f: *mut frame) -> Lisp_Object
{
  let mut button: c_int = 0;                                         // C 3511
  let mut up: c_int = 0;                                             // C 3512

  parse_button ((*msg).msg.message as c_int,                         // C 3514
		HIWORD! ((*msg).msg.wParam) as c_int,
		&raw mut button, &raw mut up);                       // C 3515

  /* Make the event type NO_EVENT; we'll change that when we decide
     otherwise.  */
  input_event_bits! (kind, result, MOUSE_CLICK_EVENT);               // C 3519
  (*result).code = button as c_uint;                                 // C 3520
  (*result).timestamp = (*msg).msg.time as Time;                     // C 3521
  (*result).modifiers = (*msg).dwModifiers | (if up != 0                   // C 3522-3525
					       { up_modifier as c_uint }
					     else { down_modifier as c_uint });

  XSETINT! ((*result).x, LOWORD! ((*msg).msg.lParam) as EMACS_INT);  // C 3527
  XSETINT! ((*result).y, HIWORD! ((*msg).msg.lParam) as EMACS_INT);  // C 3528
  XSETFRAME! ((*result).frame_or_window, f);                         // C 3529
  (*result).arg = Qnil ();                                           // C 3530
  return Qnil ();                                                    // C 3531
}

/// C 3534-3629: `static Lisp_Object w32_construct_mouse_wheel (struct input_event *result, W32Msg
/// *msg, struct frame *f)` - C 3538-3540's point, delta and `sum_delta_y` (a function-local
/// static, the slice-17 reading), C 3542-3546's event fill, C 3551-3556's `GET_WHEEL_DELTA_WPARAM`
/// and zero-delta guard, C 3560-3561's two `short` casts, C 3563-3616's high-precision arm (with
/// C 3576-3586's `window_from_coordinates`/pixel-height fallback, C 3590-3607's coalescing
/// accumulator, C 3603-3615's two rate expressions and C 3613-3615's `list3`), C 3620-3621's
/// modifier choice and C 3624-3627's `ScreenToClient` + three setters.
/// C: static Lisp_Object
/// C: w32_construct_mouse_wheel (struct input_event *result, W32Msg *msg,
/// C:                            struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_construct_mouse_wheel (result: *mut input_event, msg: *mut W32Msg,
				     f: *mut frame) -> Lisp_Object
{
  /* C declares the `POINT` uninitialised and fills both members below (PORTING.md 1.5).  */
  let mut p: POINT = core::mem::zeroed ();                           // C 3538
  let mut delta: c_int;                                              // C 3539
  static mut sum_delta_y: c_int = 0;                                 // C 3540

  input_event_bits! (kind, result,                                  // C 3542-3543
		     if (*msg).msg.message == WM_MOUSEHWHEEL as c_uint
		       { HORIZ_WHEEL_EVENT } else { WHEEL_EVENT });
  (*result).code = 0;                                                // C 3544
  (*result).timestamp = (*msg).msg.time as Time;                     // C 3545
  (*result).arg = Qnil ();                                           // C 3546

  /* A WHEEL_DELTA positive value indicates that the wheel was rotated
     forward, away from the user (up); a negative value indicates that
     the wheel was rotated backward, toward the user (down).  */
  delta = GET_WHEEL_DELTA_WPARAM! ((*msg).msg.wParam) as c_int;      // C 3551
  if delta == 0                                                      // C 3552
    {
      input_event_bits! (kind, result, NO_EVENT);                    // C 3554
      return Qnil ();                                                // C 3555
    }

  /* With multiple monitors, we can legitimately get negative
     coordinates, so cast to short to interpret them correctly.  */
  p.x = LOWORD! ((*msg).msg.lParam) as c_short as LONG;              // C 3560
  p.y = HIWORD! ((*msg).msg.lParam) as c_short as LONG;              // C 3561

  if eabs! (delta) < WHEEL_DELTA                                     // C 3563
    {
      /* This is high-precision mouse wheel, which sends
	 fine-resolution wheel events.  Produce a wheel event only if
	 the conditions for sending such an event are fulfilled.  */
      let mut scroll_unit: c_int = max! (w32_wheel_scroll_lines, 1) as c_int;   // C 3568
      let mut nlines: c_int;
      let mut value_to_report: c_double;                             // C 3569

      /* w32_wheel_scroll_lines == UINT_MAX means the user asked for
	 "entire page" to be the scroll unit.  We interpret that as
	 the height of the window under the mouse pointer.  */
      if w32_wheel_scroll_lines == UINT_MAX                          // C 3574
	{
	  let window: Lisp_Object
	    = window_from_coordinates (f as *mut c_void, p.x, p.y,     // C 3576
				       core::ptr::null_mut (), false, false, false);
	  if !WINDOWP (window)                                       // C 3578
	    {
	      input_event_bits! (kind, result, NO_EVENT);            // C 3580
	      return Qnil ();                                        // C 3581
	    }
	  scroll_unit = (*XWINDOW (window)).pixel_height;             // C 3583
	  if scroll_unit < 1	/* paranoia */                          // C 3584
	    { scroll_unit = 1; }                                     // C 3585
	}

      /* If mwheel-coalesce-scroll-events is non-nil, report a wheel event
	 only when we have accumulated enough delta's for WHEEL_DELTA.  */
      if mwheel_coalesce_scroll_events                               // C 3590
	{
	  /* If the user changed the direction, reset the accumulated
	     deltas. */
	  if (delta > 0) != (sum_delta_y > 0)                        // C 3594
	    { sum_delta_y = 0; }                                     // C 3595
	  sum_delta_y += delta;                                      // C 3596
	  /* https://docs.microsoft.com/en-us/previous-versions/ms997498(v=msdn.10) */
	  if eabs! (sum_delta_y) < WHEEL_DELTA                       // C 3598
	    {
	      input_event_bits! (kind, result, NO_EVENT);            // C 3600
	      return Qnil ();                                        // C 3601
	    }
	  value_to_report
	    = (FRAME_LINE_HEIGHT (f) as c_double * scroll_unit as c_double)
	      / (WHEEL_DELTA as c_double / sum_delta_y as c_double);   // C 3603-3605
	  sum_delta_y = 0;                                           // C 3606
	}
      else
	{
	  value_to_report
	    = (FRAME_LINE_HEIGHT (f) as c_double * scroll_unit as c_double)
	      / (WHEEL_DELTA as c_double / delta as c_double);       // C 3609-3611
	}
      nlines = (value_to_report / FRAME_LINE_HEIGHT (f) as c_double + 0.5) as c_int;   // C 3612
      (*result).arg = list3 (make_fixnum (eabs! (nlines) as EMACS_INT),   // C 3613-3615
			     make_float (0.0),
			     make_float (value_to_report));
    }

  /* The up and down modifiers indicate if the wheel was rotated up or
     down based on WHEEL_DELTA value.  */
  (*result).modifiers = (*msg).dwModifiers                                // C 3620-3621
    | (if delta < 0 { down_modifier as c_uint } else { up_modifier as c_uint });

  /* For the case that F's w32 window is not msg->msg.hwnd.  */
  ScreenToClient (FRAME_W32_WINDOW (f), &raw mut p);                 // C 3624
  XSETINT! ((*result).x, p.x as EMACS_INT);                           // C 3625
  XSETINT! ((*result).y, p.y as EMACS_INT);                           // C 3626
  XSETFRAME! ((*result).frame_or_window, f);                         // C 3627
  return Qnil ();                                                    // C 3628
}

/* C 3631-3763: the `#if HAVE_W32NOTIFY` block (C 3632) with its inner `#ifdef WINDOWSNT`
   (C 3672): `w32_lispy_file_action` (C 3636-3670) and `w32_queue_notifications` (C 3676-3761).
   The gate is the registered `have_w32notify` feature plus the file gate (the `src/w32xfns.rs`
   66-79 reading of the same macro; `HAVE_W32NOTIFY` is `undef` in
   `docs/baseline/config/darwin-arm64.macros.tsv` 1675 and a Windows build's own configure).  */
/// C: #if HAVE_W32NOTIFY
/// C: /* File event notifications (see w32notify.c).  */

/* C's `<winbase.h>` names this block reads that no mirror in this unit's graph carries: the
   notification record with its `FileName[1]` tail, the `…ACTION…` values C's switch compares and
   `termhooks.h`'s event kind.  Every name stands at the use site (P-88 (1)); the SDK records are
   UNVERIFIED.  */
/// C: typedef struct _FILE_NOTIFY_INFORMATION { DWORD NextEntryOffset; DWORD Action;
/// C:   DWORD FileNameLength; WCHAR FileName[1]; } FILE_NOTIFY_INFORMATION;
/// C: typedef FILE_NOTIFY_INFORMATION *PFILE_NOTIFY_INFORMATION;
/// C: #define FILE_ACTION_ADDED 0x00000001 / #define FILE_ACTION_REMOVED 0x00000002
/// C: #define FILE_ACTION_MODIFIED 0x00000003 / #define FILE_ACTION_RENAMED_OLD_NAME 0x00000004
/// C: #define FILE_ACTION_RENAMED_NEW_NAME 0x00000005
/// C: termhooks.h 291: `FILE_NOTIFY_EVENT` (an `enum input_event_kind` member; the `src/inotify.rs`
/// C:   284 reading of the same enumerator, value 28)
/// C: #define INT_STRLEN_BOUND(t) ((sizeof (t) * CHAR_BIT - 1) * 146 / 485 + 1)
/// C:                                                    [gnulib intprops.h; `src/xterm.rs` 19085]
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_INFORMATION
{
  pub NextEntryOffset: DWORD,
  pub Action: DWORD,
  pub FileNameLength: DWORD,
  pub FileName: [WCHAR; 1],
}
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub type PFILE_NOTIFY_INFORMATION = *mut FILE_NOTIFY_INFORMATION;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_ACTION_ADDED: DWORD = 0x00000001;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_ACTION_REMOVED: DWORD = 0x00000002;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_ACTION_MODIFIED: DWORD = 0x00000003;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_ACTION_RENAMED_OLD_NAME: DWORD = 0x00000004;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_ACTION_RENAMED_NEW_NAME: DWORD = 0x00000005;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
pub const FILE_NOTIFY_EVENT: c_int = 28;
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => { ((core::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1 };
}

/// C 3636-3670: `Lisp_Object w32_lispy_file_action (DWORD action)` - C 3639's function-local
/// `static char unknown_fmt[]`, C 3642-3667's `switch (action)` over the five `FILE_ACTION_*`
/// values with C 3659-3666's `default:` (which `sprintf`s the format into C's
/// `sizeof`-sized buffer and interns it) and C 3669's answer.  The entry is non-`static`, so it is
/// exported (PORTING.md 0.2).
/// C: /* File event notifications (see w32notify.c).  */
/// C: Lisp_Object
/// C: w32_lispy_file_action (DWORD action)
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_lispy_file_action (action: DWORD) -> Lisp_Object
{
  /* C's `static char unknown_fmt[] = "unknown-action(%d)";` is the tree's element-list device
     (`src/json.rs` 359, `src/fns.rs` 6421), with C's terminating NUL and the array's own length
     (19) kept; `c_char` is `i8` here, so the byte-string form would be an `E0308`.  */
  static mut unknown_fmt: [c_char; 19]
    = [b'u' as c_char, b'n' as c_char, b'k' as c_char, b'n' as c_char, b'o' as c_char,
       b'w' as c_char, b'n' as c_char, b'-' as c_char, b'a' as c_char, b'c' as c_char,
       b't' as c_char, b'i' as c_char, b'o' as c_char, b'n' as c_char, b'(' as c_char,
       b'%' as c_char, b'd' as c_char, b')' as c_char, 0];          // C 3639
  let mut retval: Lisp_Object;                                       // C 3640

  match action                                                       // C 3642
    {
    FILE_ACTION_ADDED =>                                             // C 3644
      { retval = Qadded (); }                                        // C 3645

    FILE_ACTION_REMOVED =>                                           // C 3647
      { retval = Qremoved (); }                                      // C 3648

    FILE_ACTION_MODIFIED =>                                          // C 3650
      { retval = Qmodified (); }                                     // C 3651

    FILE_ACTION_RENAMED_OLD_NAME =>                                  // C 3653
      { retval = Qrenamed_from (); }                                 // C 3654

    FILE_ACTION_RENAMED_NEW_NAME =>                                  // C 3656
      { retval = Qrenamed_to (); }                                   // C 3657

    /* C's `default:` builds `char buf[sizeof(unknown_fmt) - 1 + INT_STRLEN_BOUND (DWORD)]`,
       formats C's own `unknown_fmt` into it and interns the result (C 3659-3666).  */
    _ =>
      {
	let mut buf: [c_char; (19 - 1 + INT_STRLEN_BOUND! (DWORD)) as usize]
	  = [0; (19 - 1 + INT_STRLEN_BOUND! (DWORD)) as usize];      // C 3661

	sprintf (buf.as_mut_ptr (), unknown_fmt.as_ptr (), action);    // C 3663
	retval = intern (buf.as_ptr ());                               // C 3664
      }
    }

  return retval;                                                     // C 3669
}

/// C 3676-3761: "Put file notifications into the Emacs input event queue. …" - C 3680-3682's three
/// locals, C 3686-3687's initialisation guard, C 3689's `XSETFRAME`, C 3691-3758's drain loop (the
/// critical section, the list unlink, C 3711-3717's record walk, C 3723-3724's size/object guard,
/// C 3728-3752's per-entry event build with C 3747-3751's advance and C 3755-3756's two `xfree`s)
/// and C 3760's final `NO_EVENT`.
/// C: #ifdef WINDOWSNT
/// C: /* Put file notifications into the Emacs input event queue.  This
/// C:    function runs when the WM_EMACS_FILENOTIFY message arrives from a
/// C:    watcher thread.  */
/// C: static void
/// C: w32_queue_notifications (struct input_event *event, W32Msg *msg,
/// C:                          struct frame *f, int *evcount)
#[cfg (all (feature = "have_ntgui", feature = "have_w32notify", feature = "windowsnt"))]
unsafe fn w32_queue_notifications (event: *mut input_event, msg: *mut W32Msg,
				   f: *mut frame, evcount: *mut c_int)
{
  let mut ns: *mut notifications_set = core::ptr::null_mut ();       // C 3680
  let mut frame: Lisp_Object;                                        // C 3681
  let mut done: c_int = 0;                                           // C 3682

  /* We cannot process notification before Emacs is fully initialized,
     since we need the UTF-16LE coding-system to be set up.  */
  if !initialized                                                  // C 3686
    { return; }                                                      // C 3687

  XSETFRAME! (frame, f);                                             // C 3689

  while done == 0                                                   // C 3691
    {
      ns = core::ptr::null_mut ();                                   // C 3693

      /* Find out if there is a record available in the linked list of
	 notifications sets.  If so, unlink the set from the linked
	 list.  Use critical section.  */
      enter_crit ();                                                 // C 3698
      if (*notifications_set_head).next != notifications_set_head   // C 3699
	{
	  ns = (*notifications_set_head).next;                         // C 3701
	  (*(*ns).prev).next = (*ns).next;                             // C 3702
	  (*(*ns).next).prev = (*ns).prev;                             // C 3703
	}
      else
	{ done = 1; }                                                // C 3706
      leave_crit ();                                                 // C 3707

      if !ns.is_null ()                                              // C 3709
	{
	  let mut p: *mut BYTE = (*ns).notifications;                  // C 3711
	  let mut fni: *mut FILE_NOTIFY_INFORMATION
	    = p as PFILE_NOTIFY_INFORMATION;                           // C 3712
	  let min_size: DWORD
	    = core::mem::offset_of! (FILE_NOTIFY_INFORMATION, FileName) as DWORD
	      + core::mem::size_of::<wchar_t> () as DWORD;             // C 3713-3714
	  let mut info_size: DWORD = (*ns).size;                       // C 3715
	  let cs: Lisp_Object = Qutf_16le ();                          // C 3716
	  let obj: Lisp_Object = w32_get_watch_object ((*ns).desc);    // C 3717

	  /* notifications size could be zero when the buffer of
	     notifications overflowed on the OS level, or when the
	     directory being watched was itself deleted.  Do nothing in
	     that case.  */
	  if info_size != 0                                            // C 3723
	      && !NILP (obj) && CONSP (obj)                            // C 3724
	    {
	      let callback: Lisp_Object = XCDR (obj);                  // C 3726

	      while info_size >= min_size                              // C 3728
		{
		  let utf_16_fn: Lisp_Object
		    = make_unibyte_string ((*fni).FileName.as_ptr () as *const c_char,
					   (*fni).FileNameLength as ptrdiff_t);   // C 3730-3732
		  /* Note: mule-conf is preloaded, so utf-16le must
		     already be defined at this point.  */
		  let fname: Lisp_Object
		    = code_convert_string_norecord (utf_16_fn, cs, false);   // C 3735-3736
		  let action: Lisp_Object = w32_lispy_file_action ((*fni).Action);   // C 3737

		  input_event_bits! (kind, event, FILE_NOTIFY_EVENT);   // C 3739
		  (*event).timestamp = (*msg).msg.time as Time;         // C 3740
		  (*event).modifiers = 0;                              // C 3741
		  (*event).frame_or_window = callback;                 // C 3742
		  (*event).arg = list3 (make_pointer_integer ((*ns).desc),   // C 3743-3744
					action, fname);
		  kbd_buffer_store_event (event as *mut c_void);       // C 3745
		  *evcount += 1;                                       // C 3746
		  if (*fni).NextEntryOffset == 0                       // C 3747
		    { break; }                                         // C 3748
		  p = p.add ((*fni).NextEntryOffset as usize);         // C 3749
		  fni = p as PFILE_NOTIFY_INFORMATION;                 // C 3750
		  info_size -= (*fni).NextEntryOffset;                 // C 3751
		}
	    }
	  /* Free this notifications set. */
	  xfree ((*ns).notifications as *mut c_void);                  // C 3755
	  xfree (ns as *mut c_void);                                   // C 3756
	}
    }
  /* We've stuffed all the events ourselves, so w32_read_socket shouldn't.  */
  input_event_bits! (kind, event, NO_EVENT);                         // C 3760
}
/* C 3762-3763: the two block closers, carried as C's own text.
   #endif	/* WINDOWSNT */
   #endif	/* HAVE_W32NOTIFY */  */

/// C 3766-3819: "Function to report a mouse movement to the mainstream Emacs code. …" - C
/// 3777-3780's four locals (C 3778-3779's two `LOWORD`/`HIWORD` reads of the `lParam`), C
/// 3782-3783's output guard, C 3785-3789's five display-info stores, C 3791-3798's
/// off-frame arm and C 3801-3818's glyph-rectangle test (C 3806-3815's five statements).
/// C: /* Function to report a mouse movement to the mainstream Emacs code.
/// C:    The input handler calls this.
/// C:
/// C:    We have received a mouse movement event, which is given in *event.
/// C:    If the mouse is over a different glyph than it was last time, tell
/// C:    the mainstream emacs code by setting mouse_moved.  If not, ask for
/// C:    another motion event, so we can check again the next time it moves.  */
/// C: static int
/// C: w32_note_mouse_movement (struct frame *frame, MSG *msg)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_note_mouse_movement (frame: *mut frame, msg: *mut MSG) -> c_int
{
  let mut dpyinfo: *mut w32_display_info;                            // C 3777
  let mouse_x: c_int = LOWORD! ((*msg).lParam) as c_int;             // C 3778
  let mouse_y: c_int = HIWORD! ((*msg).lParam) as c_int;             // C 3779
  let mut r: *mut RECT;                                              // C 3780

  if FRAME_OUTPUT_DATA! (frame).is_null ()                           // C 3782
    { return 0; }                                                    // C 3783

  dpyinfo = FRAME_DISPLAY_INFO! (frame);                             // C 3785
  (*dpyinfo).last_mouse_movement_time = (*msg).time as Time;         // C 3786
  (*dpyinfo).last_mouse_motion_frame = frame;                        // C 3787
  (*dpyinfo).last_mouse_motion_x = mouse_x;                          // C 3788
  (*dpyinfo).last_mouse_motion_y = mouse_y;                          // C 3789

  if (*msg).hwnd != FRAME_W32_WINDOW (frame)                         // C 3791
    {
      frame_bits! (mouse_moved, frame, true);                        // C 3793
      (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();     // C 3794
      note_mouse_highlight (frame, -1, -1);                          // C 3795
      (*dpyinfo).last_mouse_glyph_frame = core::ptr::null_mut ();    // C 3796
      return 1;                                                      // C 3797
    }

  /* Has the mouse moved off the glyph it was on at the last sighting?  */
  r = &raw mut (*dpyinfo).last_mouse_glyph;                          // C 3801
  if frame != (*dpyinfo).last_mouse_glyph_frame                      // C 3802
      || mouse_x < (*r).left || mouse_x >= (*r).right                // C 3803
      || mouse_y < (*r).top  || mouse_y >= (*r).bottom               // C 3804
    {
      frame_bits! (mouse_moved, frame, true);                        // C 3806
      (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();     // C 3807
      note_mouse_highlight (frame, mouse_x, mouse_y);                // C 3808
      /* Remember the mouse position here, as w32_mouse_position only
	 gets called when mouse tracking is enabled but we also need
	 to keep track of the mouse for help_echo and highlighting at
	 other times.  */
      remember_mouse_glyph (frame, mouse_x, mouse_y, r);             // C 3813
      (*dpyinfo).last_mouse_glyph_frame = frame;                     // C 3814
      return 1;                                                      // C 3815
    }

  return 0;                                                          // C 3818
}

/* C 3822-3824 is the "Mouse Face" banner and C 3826-3834 are the file's own three forward
   declarations (`w32_window_to_scroll_bar`, `w32_scroll_bar_report_motion`,
   `w32_horizontal_scroll_bar_report_motion`) - dropped by PORTING.md 0.1-12 (Rust resolves
   forward references), with C's text carried at its position.  */
/* C: /************************************************************************
   C: 			      Mouse Face
   C:  ************************************************************************/
   C: static struct scroll_bar *w32_window_to_scroll_bar (Window, int);
   C: static void w32_scroll_bar_report_motion (struct frame **, Lisp_Object *,
   C:                                           enum scroll_bar_part *,
   C:                                           Lisp_Object *, Lisp_Object *,
   C:                                           Time *);
   C: static void w32_horizontal_scroll_bar_report_motion (struct frame **, Lisp_Object *,
   C:                                                      enum scroll_bar_part *,
   C:                                                      Lisp_Object *, Lisp_Object *,
   C:                                                      Time *);  */

/* C's `<winuser.h>` entries C 3838/3904/3911/3936 call and the two reference-side prototypes this
   slice needs that no mirror in this unit's graph declares: `PostMessage` (the `…A` export),
   `WindowFromPoint`, `ChildWindowFromPoint`, `GetCursorPos`, `remember_mouse_glyph`
   (dispextern.h 3586, whose `NativeRectangle *` is this unit's `RECT *`) and `gui_mouse_grabbed`
   (dispextern.h 4006, `Display_Info *` being `w32_display_info *` for this term).  Every name
   stands at the use site (P-88 (1)); the SDK records are UNVERIFIED.  */
/// C: WINUSERAPI BOOL WINAPI PostMessageA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: WINUSERAPI HWND WINAPI WindowFromPoint (POINT Point);
/// C: WINUSERAPI HWND WINAPI ChildWindowFromPoint (HWND hWndParent, POINT Point);
/// C: WINUSERAPI BOOL WINAPI GetCursorPos (LPPOINT lpPoint);
/// C: extern void remember_mouse_glyph (struct frame *, int, int, NativeRectangle *);
/// C: extern bool gui_mouse_grabbed (Display_Info *);            [dispextern.h 3586 / 4006]
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn PostMessage (hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
  pub fn WindowFromPoint (Point: POINT) -> HWND;
  pub fn ChildWindowFromPoint (hWndParent: HWND, Point: POINT) -> HWND;
  pub fn GetCursorPos (lpPoint: *mut POINT) -> BOOL;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn remember_mouse_glyph (f: *mut frame, x: c_int, y: c_int, rectangle: *mut RECT);
  pub fn gui_mouse_grabbed (dpyinfo: *mut w32_display_info) -> bool;
}

/// C 3835-3839: `static void w32_define_cursor (Window window, Emacs_Cursor cursor)` - the one
/// `PostMessage` with C 3838's two casts.
/// C: static void
/// C: w32_define_cursor (Window window, Emacs_Cursor cursor)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_define_cursor (window: Window, cursor: Emacs_Cursor)
{
  PostMessage (window, WM_EMACS_SETCURSOR as UINT, cursor as WPARAM, 0);   // C 3838
}

/// C 3841-3970: "Return the current position of the mouse. …" - C 3866's display-info read, C
/// 3868's input block, C 3870-3878's scroll-bar arm (C 3874's `horizontal` choice), C 3881-3883's
/// four locals, C 3886-3887's `FOR_EACH_FRAME` clear, C 3891's `GetCursorPos`, C 3900-3943's
/// innermost-window search (C 3931-3940's scroll-bar lookup and C 3942-3943's fallback), C
/// 3945-3966's store block and C 3969's `unblock_input`.
/// C: /* Return the current position of the mouse.
/// C:    *fp should be a frame which indicates which display to ask about.
/// C:
/// C:    If the mouse movement started in a scroll bar, set *fp, *bar_window,
/// C:    and *part to the frame, window, and scroll bar part that the mouse
/// C:    is over.  Set *x and *y to the portion and whole of the mouse's
/// C:    position on the scroll bar.
/// C:
/// C:    If the mouse movement started elsewhere, set *fp to the frame the
/// C:    mouse is on, *bar_window to nil, and *x and *y to the character cell
/// C:    the mouse is over.
/// C:
/// C:    Set *time to the server time-stamp for the time at which the mouse
/// C:    was at this position.
/// C:
/// C:    Don't store anything if we don't have a valid set of values to report.
/// C:
/// C:    This clears the mouse_moved flag, so we can wait for the next mouse
/// C:    movement.  */
/// C: static void
/// C: w32_mouse_position (struct frame **fp, int insist, Lisp_Object *bar_window,
/// C: 		    enum scroll_bar_part *part, Lisp_Object *x, Lisp_Object *y,
/// C: 		    Time *time)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_mouse_position (fp: *mut *mut frame, insist: c_int,
			      bar_window: *mut Lisp_Object, part: *mut scroll_bar_part,
			      x: *mut Lisp_Object, y: *mut Lisp_Object, time: *mut Time)
{
  let mut dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO! (*fp);   // C 3866

  block_input ();                                                    // C 3868

  if !(*dpyinfo).last_mouse_scroll_bar.is_null () && insist == 0       // C 3870
    {
      let bar: *mut scroll_bar = (*dpyinfo).last_mouse_scroll_bar;     // C 3872

      if (*bar).horizontal                                             // C 3874
	{ w32_horizontal_scroll_bar_report_motion (fp, bar_window, part, x, y, time); }   // C 3875
      else
	{ w32_scroll_bar_report_motion (fp, bar_window, part, x, y, time); }   // C 3877
    }
  else
    {
      /* C declares the `POINT` uninitialised and fills it with `GetCursorPos` (PORTING.md 1.5).  */
      let mut pt: POINT = core::mem::zeroed ();                        // C 3881
      let mut frame: Lisp_Object;                                      // C 3882
      let mut tail: Lisp_Object;
      let mut f1: *mut frame = core::ptr::null_mut ();                 // C 3883

      /* Clear the mouse-moved flag for every frame on this display.  */
      FOR_EACH_FRAME! (tail, frame, {                                  // C 3886
	frame_bits! (mouse_moved, XFRAME (frame), false); });          // C 3887
      (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();       // C 3889

      GetCursorPos (&raw mut pt);                                      // C 3891

      /* Now we have a position on the root; find the innermost window
	 containing the pointer.  */

      /* If mouse was grabbed on a frame and we are not dropping,
	 give coords for that frame even if the mouse is now outside
	 it.  Otherwise check for window under mouse on one of our
	 frames.  */
      if gui_mouse_grabbed (dpyinfo)
	  && !EQ (track_mouse, Qdropping ())                           // C 3900
	{ f1 = (*dpyinfo).last_mouse_frame; }                          // C 3901
      else
	{
	  let wfp: HWND = WindowFromPoint (pt);                        // C 3904

	  if !wfp.is_null ()                                           // C 3906
	    {
	      f1 = w32_window_to_frame (dpyinfo, wfp);                 // C 3908
	      if !f1.is_null ()                                        // C 3909
		{
		  let cwfp: HWND = ChildWindowFromPoint (wfp, pt);     // C 3911

		  if !cwfp.is_null ()                                  // C 3913
		    {
		      let f2: *mut frame = w32_window_to_frame (dpyinfo, cwfp);   // C 3915

		      /* If a child window was found, make sure that its
			 frame is a child frame (Bug#26615, maybe).  */
		      if !f2.is_null () && !FRAME_PARENT_FRAME (f2).is_null ()   // C 3919
			{ f1 = f2; }                                   // C 3920
		    }
		}
	    }
	}

      if f1.is_null () || FRAME_TOOLTIP_P (f1)                         // C 3926
	/* Don't use a tooltip frame.  */
	{
	  f1 = if EQ (track_mouse, Qdropping ()) && gui_mouse_grabbed (dpyinfo)   // C 3928
		 { (*dpyinfo).last_mouse_frame }                       // C 3929
	       else
		 { core::ptr::null_mut () };                           // C 3930
	}

      /* If not, is it one of our scroll bars?  */
      if f1.is_null ()                                                 // C 3933
	{
	  let bar: *mut scroll_bar
	    = w32_window_to_scroll_bar (WindowFromPoint (pt), 2);     // C 3935-3936

	  if !bar.is_null ()                                           // C 3938
	    { f1 = XFRAME (WINDOW_FRAME (XWINDOW ((*bar).window))); }  // C 3939
	}

      if f1.is_null () && insist > 0                                   // C 3942
	{ f1 = SELECTED_FRAME (); }                                    // C 3943

      if !f1.is_null ()                                                // C 3945
	{
	  /* Ok, we found a frame.  Store all the values.
	     last_mouse_glyph is a rectangle used to reduce the
	     generation of mouse events.  To not miss any motion
	     events, we must divide the frame into rectangles of the
	     size of the smallest character that could be displayed
	     on it, i.e. into the same rectangles that matrices on
	     the frame are divided into.  */

	  dpyinfo = FRAME_DISPLAY_INFO! (f1);                             // C 3955
	  ScreenToClient (FRAME_W32_WINDOW (f1), &raw mut pt);             // C 3956
	  remember_mouse_glyph (f1, pt.x, pt.y, &raw mut (*dpyinfo).last_mouse_glyph);   // C 3957
	  (*dpyinfo).last_mouse_glyph_frame = f1;                          // C 3958

	  *bar_window = Qnil ();                                          // C 3960
	  *part = scroll_bar_above_handle;                                // C 3961
	  *fp = f1;                                                       // C 3962
	  XSETINT! (*x, pt.x as EMACS_INT);                               // C 3963
	  XSETINT! (*y, pt.y as EMACS_INT);                               // C 3964
	  *time = (*dpyinfo).last_mouse_movement_time;                   // C 3965
	}
    }

  unblock_input ();                                                   // C 3969
}

/* C's `<winuser.h>`/`<dispextern.h>` surface C 3988-4154 reads that this unit does not carry yet:
   the two click handlers' own prototypes (dispextern.h 3668-3671) and the scroll-bar info block
   (`SCROLLINFO` with its two entry points and the three `SIF_*` bits and `SB_CTL`).  Every name
   stands at the use site (P-88 (1)); the SDK records are UNVERIFIED.  */
/// C: extern Lisp_Object handle_tab_bar_click (struct frame *, int, int, bool, int);
/// C: extern void handle_tool_bar_click (struct frame *, int, int, bool, int);
/// C: typedef struct tagSCROLLINFO { UINT cbSize; UINT fMask; int nMin; int nMax; UINT nPage;
/// C:   int nPos; int nTrackPos; } SCROLLINFO;
/// C: #define SIF_RANGE 0x0001 / #define SIF_PAGE 0x0002 / #define SIF_POS 0x0004
/// C: #define SB_CTL 2
/// C: WINUSERAPI BOOL WINAPI GetScrollInfo (HWND hwnd, int nBar, LPSCROLLINFO lpsi);
/// C: WINUSERAPI int WINAPI SetScrollInfo (HWND hwnd, int nBar, LPCSCROLLINFO lpsi, BOOL redraw);
/// C:                                                        [dispextern.h 3668-3671; winuser.h]
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCROLLINFO
{
  pub cbSize: UINT,
  pub fMask: UINT,
  pub nMin: c_int,
  pub nMax: c_int,
  pub nPage: UINT,
  pub nPos: c_int,
  pub nTrackPos: c_int,
}
#[cfg (feature = "have_ntgui")]
pub const SIF_RANGE: UINT = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const SIF_PAGE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const SIF_POS: UINT = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const SB_CTL: c_int = 2;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetScrollInfo (hwnd: HWND, nBar: c_int, lpsi: *mut SCROLLINFO) -> BOOL;
  pub fn SetScrollInfo (hwnd: HWND, nBar: c_int, lpsi: *const SCROLLINFO, redraw: BOOL) -> c_int;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn handle_tab_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
			       ts: c_int) -> Lisp_Object;
  pub fn handle_tool_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
				ts: c_int);
}

/* C 3973-3975 is the "Tab-bars" banner and C 4000-4002 the tool-bar one, both carried as C's own
   text at their positions.  */
/* C: /***********************************************************************
   C: 			       Tab-bars
   C:  ***********************************************************************/  */

/// C 3977-3992: "Handle mouse button event on the tab-bar of frame F, …" - C 3984-3985's two
/// `XFIXNAT` reads, C 3987's modifier test and C 3988/3990-3991's two handler calls.
/// C: /* Handle mouse button event on the tab-bar of frame F, at
/// C:    frame-relative coordinates X/Y.  EVENT_TYPE is either ButtonPress
/// C:    or ButtonRelease.  */
/// C: static Lisp_Object
/// C: w32_handle_tab_bar_click (struct frame *f, struct input_event *button_event)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_handle_tab_bar_click (f: *mut frame,
				    button_event: *mut input_event) -> Lisp_Object
{
  let x: c_int = XFIXNAT ((*button_event).x) as c_int;               // C 3984
  let y: c_int = XFIXNAT ((*button_event).y) as c_int;               // C 3985

  if (*button_event).modifiers & down_modifier as c_uint != 0         // C 3987
    { return handle_tab_bar_click (f, x, y, true, 0); }              // C 3988
  else
    { return handle_tab_bar_click (f, x, y, false,                   // C 3990
				   ((*button_event).modifiers
				    & ! (up_modifier as c_uint)) as c_int); }   // C 3991
}

/* C: /***********************************************************************
   C: 			       Tool-bars
   C:  ***********************************************************************/  */

/// C 4000-4015: "Handle mouse button event on the tool-bar of frame F, …" - the same two
/// `XFIXNAT` reads and modifier test as the tab-bar twin, with C 4011/4013-4014's two calls (the
/// second one returns nothing).
/// C: static void
/// C: w32_handle_tool_bar_click (struct frame *f, struct input_event *button_event)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_handle_tool_bar_click (f: *mut frame, button_event: *mut input_event)
{
  let x: c_int = XFIXNAT ((*button_event).x) as c_int;               // C 4007
  let y: c_int = XFIXNAT ((*button_event).y) as c_int;               // C 4008

  if (*button_event).modifiers & down_modifier as c_uint != 0         // C 4010
    { handle_tool_bar_click (f, x, y, true, 0); }                    // C 4011
  else
    { handle_tool_bar_click (f, x, y, false,                         // C 4013
			     ((*button_event).modifiers
			      & ! (up_modifier as c_uint)) as c_int); }   // C 4014
}

/*: /***********************************************************************
   C: 			       Scroll bars
   C:  ***********************************************************************/  */
/* C 4023-4027's two comments: "Scroll bar support." and "Given a window ID, find the struct
   scroll_bar which manages it vertically.  This can be called in GC, so we have to make sure to
   strip off mark bits."  */

/// C 4029-4056: `static struct scroll_bar *w32_window_to_scroll_bar (Window window_id, int
/// type)` - C 4032's two list variables, C 4034's `FOR_EACH_FRAME`, C 4040-4046's two-list
/// initialisation and the trick condition, C 4047's increment clause, C 4048-4051's four-way match
/// and C 4055's null answer.
/// C: static struct scroll_bar *
/// C: w32_window_to_scroll_bar (Window window_id, int type)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_window_to_scroll_bar (window_id: Window, type_: c_int) -> *mut scroll_bar
{
  let mut tail: Lisp_Object;                                         // C 4032
  let mut frame: Lisp_Object;

  FOR_EACH_FRAME! (tail, frame, {                                     // C 4034
    let mut bar: Lisp_Object;                                         // C 4036
    let mut condemned: Lisp_Object;

    /* Scan this frame's scroll bar list for a scroll bar with the
       right window ID.  */
    condemned = FRAME_CONDEMNED_SCROLL_BARS (XFRAME (frame));          // C 4040
    /* C's `for` (C 4041-4047) is the initialisation plus a `while` whose condition is C's own
       "trick allows us to search both the ordinary and condemned scroll bar lists with one
       loop" (C 4042-4046), with C's increment clause written at the loop's end.  */
    bar = FRAME_SCROLL_BARS (XFRAME (frame));                          // C 4041
    while !NILP (bar)                                                 // C 4044
	|| { bar = condemned; condemned = Qnil (); !NILP (bar) }       // C 4044-4046
      {
	if SCROLL_BAR_W32_WINDOW! (XSCROLL_BAR! (bar)) == window_id    // C 4048
	    && (type_ == 2                                              // C 4049
		|| (type_ == 1 && (*XSCROLL_BAR! (bar)).horizontal)     // C 4050
		|| (type_ == 0 && !(*XSCROLL_BAR! (bar)).horizontal))   // C 4051
	  { return XSCROLL_BAR! (bar); }                               // C 4052
	bar = (*XSCROLL_BAR! (bar)).next;                              // C 4047
      }
  });

  return core::ptr::null_mut ();                                      // C 4055
}

/// C 4059-4132: "Set the thumb size and position of vertical scroll bar BAR. …" - C 4067's window
/// handle, C 4071-4072's range expression, C 4074's `draggingp`, C 4085-4096's drag-time
/// `GetScrollInfo` probe, C 4098-4118's two position branches (with C 4105/4110-4111's `range` and
/// `whole` arithmetic) and C 4120-4131's `SetScrollInfo`.
/// C: /* Set the thumb size and position of vertical scroll bar BAR.  We are currently
/// C:    displaying PORTION out of a whole WHOLE, and our position POSITION.  */
/// C: static void
/// C: w32_set_scroll_bar_thumb (struct scroll_bar *bar,
/// C: 			  int portion, int position, int whole)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int, position: c_int,
				    whole: c_int)
{
  let w: Window = SCROLL_BAR_W32_WINDOW! (bar);                       // C 4067
  /* We use the whole scroll-bar height in the calculations below, to
     avoid strange effects like scrolling backwards when just clicking
     on the handle (without moving it).  */
  /* C 4071 passes C's dead macro argument `f`: `VERTICAL_SCROLL_BAR_INSIDE_HEIGHT` never expands
     it (w32term.h 581-582), so the name never reaches a scope and this function has none of its
     own - C compiles because a macro argument is unexpanded text, and a Rust `macro_rules!`
     argument that no expansion uses is the same (measured by slice 14's arm reading, which is why
     the port keeps C's own `f` here rather than a stand-in).  */
  let range: c_double = VERTICAL_SCROLL_BAR_TOP_RANGE! (f, (*bar).height)
    as c_double                                                       // C 4071
    + VERTICAL_SCROLL_BAR_MIN_HANDLE! () as c_double;                 // C 4072
  let mut sb_page: c_int;                                             // C 4073
  let mut sb_pos: c_int;
  let draggingp: BOOL = if (*bar).dragging != 0 { TRUE } else { FALSE };   // C 4074
  /* C declares the `SCROLLINFO` uninitialised and fills it below (PORTING.md 1.5).  */
  let mut si: SCROLLINFO = core::mem::zeroed ();                      // C 4075

  /* We used to change the nPage setting while dragging the handle,
     but that had very strange effects (such as scrolling backwards
     while dragging downwards).

     Now, we don't change the nPage setting while dragging unless we
     get near to the end of the buffer, in which case we often have to
     resize the handle to "go all the way".  */

  if draggingp != 0                                                   // C 4085
    {
      let mut near_bottom_p: c_int;                                   // C 4087
      block_input ();                                                 // C 4088
      si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;        // C 4089
      si.fMask = SIF_POS | SIF_PAGE;                                  // C 4090
      GetScrollInfo (w, SB_CTL, &raw mut si);                         // C 4091
      near_bottom_p = (si.nPos + si.nPage as c_int >= range as c_int) as c_int;   // C 4092
      unblock_input ();                                               // C 4093
      if near_bottom_p == 0                                           // C 4094
	{ return; }                                                   // C 4095
    }

  if whole != 0                                                       // C 4098
    {
      /* Position scroll bar at rock bottom if the bottom of the
         buffer is visible. This avoids shrinking the thumb away
         to nothing if it is held at the bottom of the buffer.  */
      if position + portion >= whole && draggingp == 0                 // C 4103
	{
	  sb_page = (range * (whole - position) as c_double / whole as c_double) as c_int;   // C 4105
	  sb_pos = range as c_int;                                    // C 4106
	}
      else
	{
	  sb_pos = (position as c_double * range / whole as c_double) as c_int;   // C 4110
	  sb_page = (min! (portion, whole - position) as c_double * range
		     / whole as c_double) as c_int;                   // C 4111
	}
    }
  else
    {
      sb_page = range as c_int;                                       // C 4116
      sb_pos = 0;                                                     // C 4117
    }

  sb_page = max! (sb_page, VERTICAL_SCROLL_BAR_MIN_HANDLE! ()) as c_int;   // C 4120

  block_input ();                                                     // C 4122

  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;            // C 4124
  si.fMask = SIF_PAGE | SIF_POS;                                      // C 4125
  si.nPage = sb_page as UINT;                                         // C 4126
  si.nPos = sb_pos;                                                   // C 4127

  SetScrollInfo (w, SB_CTL, &raw const si, TRUE);                     // C 4129

  unblock_input ();                                                   // C 4131
}

/// C 4134-4157: "Set the thumb size and position of horizontal scroll bar BAR. …" - the same
/// shape without the drag probe: C 4141's window handle, C 4146-4153's six `SCROLLINFO` fields and
/// C 4154's `SetScrollInfo`.
/// C: /* Set the thumb size and position of horizontal scroll bar BAR.  We are currently
/// C:    displaying PORTION out of a whole WHOLE, and our position POSITION.  */
/// C: static void
/// C: w32_set_horizontal_scroll_bar_thumb (struct scroll_bar *bar,
/// C: 				     int portion, int position, int whole)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_horizontal_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
					       position: c_int, whole: c_int)
{
  let w: Window = SCROLL_BAR_W32_WINDOW! (bar);                       // C 4141
  /* C declares the `SCROLLINFO` uninitialised and fills it below (PORTING.md 1.5).  */
  let mut si: SCROLLINFO = core::mem::zeroed ();                      // C 4142

  block_input ();                                                     // C 4144

  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;            // C 4146
  si.fMask = SIF_PAGE | SIF_POS | SIF_RANGE;                          // C 4147
  si.nMin = 0;                                                        // C 4148
  si.nMax = whole;                                                    // C 4149
  /* Allow nPage to be one larger than nPos so we don't allow the scrolling
     of an already fully visible buffer.  */
  si.nPage = (min! (portion, si.nMax) + 1) as UINT;                   // C 4152
  si.nPos = min! (position, si.nMax);                                 // C 4153
  SetScrollInfo (w, SB_CTL, &raw const si, TRUE);                     // C 4154

  unblock_input ();                                                   // C 4156
}

/* C's `<winuser.h>` names C 4167-4239 read that this unit does not carry yet: the two message
   entries (the non-timeout one the two `my_create_*scrollbar` wrappers call) and the
   `WINDOWPOS` record `my_set_window_pos` fills.  C 4197-4209's `#else` arm names `SetWindowPos`
   and C 4189's names `ShowWindow`; both are dead here (`ATTACH_THREADS` is C 4180's commented-out
   `#define`), so C's own text carries them and no declaration is needed (P-51).  Every name stands
   at the use site (P-88 (1)); the SDK records are UNVERIFIED.  */
/// C: #define SIF_ALL (SIF_RANGE | SIF_PAGE | SIF_POS | SIF_TRACKPOS)
/// C: WINUSERAPI LRESULT WINAPI SendMessageA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: typedef struct tagWINDOWPOS { HWND hwnd; HWND hwndInsertAfter; int x; int y; int cx;
/// C:   int cy; UINT flags; } WINDOWPOS;
#[cfg (feature = "have_ntgui")]
pub const SIF_ALL: UINT = 0x0017;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOWPOS
{
  pub hwnd: HWND,
  pub hwndInsertAfter: HWND,
  pub x: c_int,
  pub y: c_int,
  pub cx: c_int,
  pub cy: c_int,
  pub flags: UINT,
}
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn SendMessage (hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
}

/* C 4160-4162 is the "Scroll bars, general" banner, carried as C's own text at its position.  */
/* C: /************************************************************************
   C: 			 Scroll bars, general
   C:  ************************************************************************/  */

/// C 4164-4170: `static HWND my_create_vscrollbar (struct frame * f, struct scroll_bar * bar)` -
/// the one `SendMessage` with C 4168-4169's two casts.
/// C: static HWND
/// C: my_create_vscrollbar (struct frame * f, struct scroll_bar * bar)
#[cfg (feature = "have_ntgui")]
unsafe fn my_create_vscrollbar (f: *mut frame, bar: *mut scroll_bar) -> HWND
{
  return SendMessage (FRAME_W32_WINDOW (f),                            // C 4167
		      WM_EMACS_CREATEVSCROLLBAR as UINT, bar as WPARAM,
		      f as LPARAM) as HWND;                            // C 4168-4169
}

/// C 4172-4178: `static HWND my_create_hscrollbar (struct frame * f, struct scroll_bar * bar)` -
/// the same pair with C 4176-4177's two casts.
/// C: static HWND
/// C: my_create_hscrollbar (struct frame * f, struct scroll_bar * bar)
#[cfg (feature = "have_ntgui")]
unsafe fn my_create_hscrollbar (f: *mut frame, bar: *mut scroll_bar) -> HWND
{
  return SendMessage (FRAME_W32_WINDOW (f),                            // C 4175
		      WM_EMACS_CREATEHSCROLLBAR as UINT, bar as WPARAM,
		      f as LPARAM) as HWND;                            // C 4176-4177
}

/* C 4180's `/*#define ATTACH_THREADS*/` is C's own commented-out `#define`, carried here; with it
   undefined, C 4185's and C 4197's `#ifndef` arms are the live ones and the two `#else` arms below
   are dead text (P-51).  */
/// C 4182-4191: `static BOOL my_show_window (struct frame *f, HWND hwnd, int how)`.
/// C: /*#define ATTACH_THREADS*/
/// C: static BOOL
/// C: my_show_window (struct frame *f, HWND hwnd, int how)
/// C: #ifndef ATTACH_THREADS
/// C:   return SendMessageTimeout (FRAME_W32_WINDOW (f), WM_EMACS_SHOWWINDOW,
/// C: 			     (WPARAM) hwnd, (LPARAM) how, 0, 6000, NULL);
/// C: #else
/// C:   return ShowWindow (hwnd, how);
/// C: #endif
#[cfg (feature = "have_ntgui")]
unsafe fn my_show_window (f: *mut frame, hwnd: HWND, how: c_int) -> BOOL
{
  return SendMessageTimeout (FRAME_W32_WINDOW (f), WM_EMACS_SHOWWINDOW as UINT,   // C 4186
			     hwnd as WPARAM, how as LPARAM, 0, 6000,
			     core::ptr::null_mut ()) as BOOL;          // C 4187
}

/// C 4193-4210: `static void my_set_window_pos (HWND hwnd, HWND hwndAfter, int x, int y, int cx,
/// int cy, UINT flags)` - C 4198's `WINDOWPOS` (six field stores) and C 4205-4206's
/// `SendMessageTimeout`.
/// C: static void
/// C: my_set_window_pos (HWND hwnd, HWND hwndAfter,
/// C: 		   int x, int y, int cx, int cy, UINT flags)
/// C: #ifndef ATTACH_THREADS
/// C:   WINDOWPOS pos;
/// C:   pos.hwndInsertAfter = hwndAfter; pos.x = x; pos.y = y; pos.cx = cx; pos.cy = cy;
/// C:   pos.flags = flags;
/// C:   SendMessageTimeout (hwnd, WM_EMACS_SETWINDOWPOS, (WPARAM) &pos, 0, 0, 6000, NULL);
/// C: #else
/// C:   SetWindowPos (hwnd, hwndAfter, x, y, cx, cy, flags);
/// C: #endif
#[cfg (feature = "have_ntgui")]
unsafe fn my_set_window_pos (hwnd: HWND, hwndAfter: HWND, x: c_int, y: c_int, cx: c_int,
			     cy: c_int, flags: UINT)
{
  /* C declares the `WINDOWPOS` uninitialised and fills its six members below (PORTING.md 1.5).  */
  let mut pos: WINDOWPOS = core::mem::zeroed ();                      // C 4198
  pos.hwndInsertAfter = hwndAfter;                                    // C 4199
  pos.x = x;                                                          // C 4200
  pos.y = y;                                                          // C 4201
  pos.cx = cx;                                                        // C 4202
  pos.cy = cy;                                                        // C 4203
  pos.flags = flags;                                                  // C 4204
  SendMessageTimeout (hwnd, WM_EMACS_SETWINDOWPOS as UINT,            // C 4205
		      &raw mut pos as WPARAM, 0, 0, 6000,
		      core::ptr::null_mut ());                           // C 4206
}

/* C 4212-4219: the `#if 0` block holding `my_set_focus`, carried as C's own text (P-51).
   #if 0
   static void
   my_set_focus (struct frame * f, HWND hwnd)
   {
     SendMessageTimeout (FRAME_W32_WINDOW (f), WM_EMACS_SETFOCUS,
			 (WPARAM) hwnd, 0, 0, 6000, NULL);
   }
   #endif  */

/// C 4221-4226: `static void my_set_foreground_window (HWND hwnd)` - the one timeout message.
/// C: static void
/// C: my_set_foreground_window (HWND hwnd)
#[cfg (feature = "have_ntgui")]
unsafe fn my_set_foreground_window (hwnd: HWND)
{
  SendMessageTimeout (hwnd, WM_EMACS_SETFOREGROUND as UINT,           // C 4224
		      hwnd as WPARAM, 0, 0, 6000, core::ptr::null_mut ());   // C 4225
}

/// C 4229-4234: `static void my_destroy_window (struct frame * f, HWND hwnd)`.
/// C: static void
/// C: my_destroy_window (struct frame * f, HWND hwnd)
#[cfg (feature = "have_ntgui")]
unsafe fn my_destroy_window (f: *mut frame, hwnd: HWND)
{
  SendMessageTimeout (FRAME_W32_WINDOW (f), WM_EMACS_DESTROYWINDOW as UINT,   // C 4232
		      hwnd as WPARAM, 0, 0, 6000, core::ptr::null_mut ());   // C 4233
}

/// C 4236-4241: `static void my_bring_window_to_top (HWND hwnd)`.
/// C: static void
/// C: my_bring_window_to_top (HWND hwnd)
#[cfg (feature = "have_ntgui")]
unsafe fn my_bring_window_to_top (hwnd: HWND)
{
  SendMessageTimeout (hwnd, WM_EMACS_BRINGTOTOP as UINT,              // C 4239
		      hwnd as WPARAM, 0, 0, 6000, core::ptr::null_mut ());   // C 4240
}

/// C 4243-4305: "Create a scroll bar and return the scroll bar vector for it. …" - C 4252's
/// frame, C 4255-4256's `ALLOCATE_PSEUDOVECTOR`, C 4259's input block, C 4261-4269's nine field
/// stores, C 4273-4276's window creation, C 4278-4290's `SCROLLINFO` fill (with C 4282-4286's
/// horizontal/vertical range pair) and C 4292-4300's list insert with C 4299-4300's
/// `SET_SCROLL_BAR_W32_WINDOW`/`XSETVECTOR` pair.
/// C: /* Create a scroll bar and return the scroll bar vector for it.  W is
/// C:    the Emacs window on which to create the scroll bar. TOP, LEFT,
/// C:    WIDTH and HEIGHT are the pixel coordinates and dimensions of the
/// C:    scroll bar. */
/// C: static struct scroll_bar *
/// C: w32_scroll_bar_create (struct window *w, int left, int top,
/// C:                        int width, int height, bool horizontal)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_bar_create (w: *mut window, left: c_int, top: c_int, width: c_int,
				 height: c_int, horizontal: bool) -> *mut scroll_bar
{
  let f: *mut frame = XFRAME (WINDOW_FRAME (w));                      // C 4252
  let mut hwnd: HWND;                                                 // C 4253
  let mut si: SCROLLINFO;                                             // C 4254
  let mut bar: *mut scroll_bar
    = ALLOCATE_PSEUDOVECTOR! (scroll_bar, w32_widget_high, pvec_type::PVEC_OTHER);   // C 4255-4256
  let mut barobj: Lisp_Object;                                        // C 4257

  block_input ();                                                     // C 4259

  XSETWINDOW! ((*bar).window, w);                                     // C 4261
  (*bar).top = top;                                                   // C 4262
  (*bar).left = left;                                                 // C 4263
  (*bar).width = width;                                               // C 4264
  (*bar).height = height;                                             // C 4265
  (*bar).start = 0;                                                   // C 4266
  (*bar).end = 0;                                                     // C 4267
  (*bar).dragging = 0;                                                // C 4268
  (*bar).horizontal = horizontal;                                     // C 4269

  /* Requires geometry to be set before call to create the real window */

  if horizontal                                                       // C 4273
    { hwnd = my_create_hscrollbar (f, bar); }                         // C 4274
  else
    { hwnd = my_create_vscrollbar (f, bar); }                         // C 4276

  /* C declares the `SCROLLINFO` uninitialised and fills it below (PORTING.md 1.5).  */
  si = core::mem::zeroed ();                                          // C 4254
  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;            // C 4278
  si.fMask = SIF_ALL;                                                 // C 4279
  si.nMin = 0;                                                        // C 4280
  if horizontal                                                       // C 4281
    {
      si.nMax = HORIZONTAL_SCROLL_BAR_LEFT_RANGE! (f, width)          // C 4282
	+ HORIZONTAL_SCROLL_BAR_MIN_HANDLE! ();                       // C 4283
    }
  else
    {
      si.nMax = VERTICAL_SCROLL_BAR_TOP_RANGE! (f, height)            // C 4285
	+ VERTICAL_SCROLL_BAR_MIN_HANDLE! ();                         // C 4286
    }
  si.nPage = si.nMax as UINT;                                         // C 4287
  si.nPos = 0;                                                        // C 4288

  SetScrollInfo (hwnd, SB_CTL, &raw const si, FALSE);                 // C 4290

  SET_SCROLL_BAR_W32_WINDOW! (bar, hwnd);                             // C 4292

  /* Add bar to its frame's list of scroll bars.  */
  (*bar).next = FRAME_SCROLL_BARS (f);                                // C 4295
  (*bar).prev = Qnil ();                                              // C 4296
  XSETVECTOR! (barobj, bar);                                          // C 4297
  fset_scroll_bars (f, barobj);                                       // C 4298
  if !NILP ((*bar).next)                                              // C 4299
    { XSETVECTOR! ((*(XSCROLL_BAR! ((*bar).next))).prev, bar); }      // C 4300

  unblock_input ();                                                   // C 4302

  return bar;                                                         // C 4304
}

/// C 4308-4328: "Destroy scroll bar BAR, and set its Emacs window's scroll bar to nil." - C
/// 4314's frame, C 4319's `my_destroy_window` and C 4322-4325's two `wset_*_scroll_bar` calls.
/// C: /* Destroy scroll bar BAR, and set its Emacs window's scroll bar to
/// C:    nil. */
/// C: static void
/// C: w32_scroll_bar_remove (struct scroll_bar *bar)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_bar_remove (bar: *mut scroll_bar)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME (XWINDOW ((*bar).window)));   // C 4314

  block_input ();                                                     // C 4316

  /* Destroy the window.  */
  my_destroy_window (f, SCROLL_BAR_W32_WINDOW! (bar));                // C 4319

  /* Dissociate this scroll bar from its window.  */
  if (*bar).horizontal                                                // C 4322
    { wset_horizontal_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }   // C 4323
  else
    { wset_vertical_scroll_bar (XWINDOW ((*bar).window), Qnil ()); }   // C 4325

  unblock_input ();                                                   // C 4327
}

/* C's `<winuser.h>` names C 4383-4518 reads and this unit does not carry yet: the two show
   commands, the bottom-of-z-order handle, the frame-changed flag, and the two window entries
   (`InvalidateRect` and `SetWindowPos`).  Every name stands at the use site (P-88 (1)); the SDK
   records are UNVERIFIED.  */
/// C: #define SW_HIDE 0 / #define SW_NORMAL 1
/// C: #define HWND_BOTTOM ((HWND) 1)
/// C: #define SWP_FRAMECHANGED 0x0020
/// C: WINUSERAPI BOOL WINAPI InvalidateRect (HWND hWnd, const RECT *lpRect, BOOL bErase);
/// C: WINUSERAPI BOOL WINAPI SetWindowPos (HWND hWnd, HWND hWndInsertAfter, int X, int Y,
/// C:   int cx, int cy, UINT uFlags);
#[cfg (feature = "have_ntgui")]
pub const SW_HIDE: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SW_NORMAL: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const HWND_BOTTOM: HWND = 1 as HWND;
#[cfg (feature = "have_ntgui")]
pub const SWP_FRAMECHANGED: UINT = 0x0020;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn InvalidateRect (hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
  pub fn SetWindowPos (hWnd: HWND, hWndInsertAfter: HWND, X: c_int, Y: c_int, cx: c_int,
		       cy: c_int, uFlags: UINT) -> BOOL;
}

/// C 4330-4432: "Set the handle of the vertical scroll bar for WINDOW …" - C 4338-4342's locals,
/// C 4345-4351's geometry reads, C 4354-4367's create branch (with C 4357-4364's clear when the
/// area is non-empty), C 4368-4428's move branch (C 4377-4385's "already positioned" redraw, C
/// 4391-4427's four-part move: the clear, the hide, C 4406-4407's `SetWindowPos`, the `SCROLLINFO`
/// fill C 4409-4415 and C 4421-4424's four remembered settings) and C 4429-4431's thumb update.
/// C: /* Set the handle of the vertical scroll bar for WINDOW to indicate that
/// C:    we are displaying PORTION characters out of a total of WHOLE
/// C:    characters, starting at POSITION.  If WINDOW has no vertical scroll
/// C:    bar, create one.  */
/// C: static void
/// C: w32_set_vertical_scroll_bar (struct window *w,
/// C: 			     int portion, int whole, int position)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_vertical_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
				       position: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);                            // C 4338
  let mut barobj: Lisp_Object;                                        // C 4339
  let mut bar: *mut scroll_bar;                                       // C 4340
  let mut top: c_int;                                                 // C 4341
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  let mut window_y: c_int;                                            // C 4342
  let mut window_height: c_int;

  /* Get window dimensions.  */
  window_box (w, ANY_AREA, core::ptr::null_mut (), &raw mut window_y,   // C 4345
	      core::ptr::null_mut (), &raw mut window_height);
  top  = window_y;                                                    // C 4346
  height = window_height;                                             // C 4347

  /* Compute the left edge and the width of the scroll bar area.  */
  left = WINDOW_SCROLL_BAR_AREA_X (w);                                // C 4350
  width = WINDOW_SCROLL_BAR_AREA_WIDTH (w);                           // C 4351

  /* Does the scroll bar exist yet?  */
  if NILP ((*w).vertical_scroll_bar)                                  // C 4354
    {
      let mut hdc: HDC;                                               // C 4356
      block_input ();                                                 // C 4357
      if width > 0 && height > 0                                      // C 4358
	{
	  hdc = get_frame_dc (f);                                     // C 4360
	  w32_clear_area! (f, hdc, left, top, width, height);          // C 4361
	  release_frame_dc (f, hdc);                                   // C 4362
	}
      unblock_input ();                                               // C 4364

      bar = w32_scroll_bar_create (w, left, top, width, height, false);   // C 4366
    }
  else
    {
      /* It may just need to be moved and resized.  */
      let mut hwnd: HWND;                                             // C 4371

      bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                   // C 4373
      hwnd = SCROLL_BAR_W32_WINDOW! (bar);                             // C 4374

      /* If already correctly positioned, do nothing.  */
      if (*bar).left == left                                          // C 4377
	  && (*bar).top == top                                        // C 4378
	  && (*bar).width == width                                    // C 4379
	  && (*bar).height == height                                  // C 4380
	{
	  /* Redraw after clear_frame. */
	  if my_show_window (f, hwnd, SW_NORMAL) == 0                  // C 4383
	    { InvalidateRect (hwnd, core::ptr::null (), FALSE); }      // C 4384
	}
      else
	{
	  /* C declares the `SCROLLINFO` uninitialised and fills it below
	     (PORTING.md 1.5).  */
	  let mut hdc: HDC;                                            // C 4388
	  let mut si: SCROLLINFO = core::mem::zeroed ();               // C 4389

	  block_input ();                                              // C 4391
	  if width != 0 && height != 0                                 // C 4392
	    {
	      hdc = get_frame_dc (f);                                  // C 4394
	      /* Since Windows scroll bars are smaller than the space reserved
		 for them on the frame, we have to clear "under" them.  */
	      w32_clear_area! (f, hdc, left, top, width, height);       // C 4397
	      release_frame_dc (f, hdc);                               // C 4398
	      w32_clear_under_internal_border (f);                     // C 4399
	    }
	  /* Make sure scroll bar is "visible" before moving, to ensure the
	     area of the parent window now exposed will be refreshed.  */
	  my_show_window (f, hwnd, SW_HIDE);                           // C 4403
	  /* Try to not draw over child frames.  */
	  SetWindowPos (hwnd, HWND_BOTTOM, left, top, width,
			max! (height, 1), SWP_FRAMECHANGED);           // C 4406-4407

	  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;      // C 4409
	  si.fMask = SIF_RANGE;                                        // C 4410
	  si.nMin = 0;                                                 // C 4411
	  si.nMax = VERTICAL_SCROLL_BAR_TOP_RANGE! (f, height)          // C 4412
	    + VERTICAL_SCROLL_BAR_MIN_HANDLE! ();                      // C 4413

	  SetScrollInfo (hwnd, SB_CTL, &raw const si, FALSE);           // C 4415

	  my_show_window (f, hwnd, SW_NORMAL);                          // C 4417
	  /* InvalidateRect (w, NULL, FALSE);  */

	  /* Remember new settings.  */
	  (*bar).left = left;                                           // C 4421
	  (*bar).top = top;                                             // C 4422
	  (*bar).width = width;                                         // C 4423
	  (*bar).height = height;                                       // C 4424

	  unblock_input ();                                             // C 4426
	}
    }
  w32_set_scroll_bar_thumb (bar, portion, position, whole);            // C 4429
  XSETVECTOR! (barobj, bar);                                          // C 4430
  wset_vertical_scroll_bar (w, barobj);                               // C 4431
}

/// C 4434-4536: "Set the handle of the horizontal scroll bar for WINDOW …" - the same shape as the
/// vertical twin with C 4447-4448's two extra `clear_*` locals, C 4451-4455's five geometry reads
/// and C 4512-4517's page/pos fill.
/// C: /* Set the handle of the horizontal scroll bar for WINDOW to indicate
/// C:    that we are displaying PORTION characters out of a total of WHOLE
/// C:    characters, starting at POSITION.  If WINDOW has no horizontal scroll
/// C:    bar, create one.  */
/// C: static void
/// C: w32_set_horizontal_scroll_bar (struct window *w,
/// C: 			       int portion, int whole, int position)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_horizontal_scroll_bar (w: *mut window, portion: c_int, whole: c_int,
					 position: c_int)
{
  let f: *mut frame = XFRAME ((*w).frame);                            // C 4442
  let mut barobj: Lisp_Object;                                        // C 4443
  let mut bar: *mut scroll_bar;                                       // C 4444
  let mut top: c_int;                                                 // C 4445
  let mut height: c_int;
  let mut left: c_int;
  let mut width: c_int;
  let mut window_x: c_int;                                            // C 4446
  let mut window_width: c_int;
  let clear_left: c_int = WINDOW_LEFT_EDGE_X (w);                     // C 4447
  let clear_width: c_int
    = WINDOW_PIXEL_WIDTH (w) - WINDOW_RIGHT_DIVIDER_WIDTH (w);         // C 4448

  /* Get window dimensions.  */
  window_box (w, ANY_AREA, &raw mut window_x, core::ptr::null_mut (),   // C 4451
	      &raw mut window_width, core::ptr::null_mut ());
  left  = window_x;                                                   // C 4452
  height = WINDOW_SCROLL_BAR_AREA_HEIGHT (w);                         // C 4453
  width = window_width;                                               // C 4454
  top = WINDOW_SCROLL_BAR_AREA_Y (w);                                 // C 4455

  /* Does the scroll bar exist yet?  */
  if NILP ((*w).horizontal_scroll_bar)                                // C 4458
    {
      let mut hdc: HDC;                                               // C 4460
      block_input ();                                                 // C 4461
      if width > 0 && height > 0                                      // C 4462
	{
	  hdc = get_frame_dc (f);                                     // C 4464
	  w32_clear_area! (f, hdc, clear_left, top, clear_width, height);   // C 4465
	  release_frame_dc (f, hdc);                                   // C 4466
	}
      unblock_input ();                                               // C 4468

      bar = w32_scroll_bar_create (w, left, top, width, height, true);   // C 4470
    }
  else
    {
      /* It may just need to be moved and resized.  */
      let mut hwnd: HWND;                                             // C 4475

      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                 // C 4477
      hwnd = SCROLL_BAR_W32_WINDOW! (bar);                             // C 4478

      /* If already correctly positioned, do nothing.  */
      if (*bar).left == left && (*bar).top == top                     // C 4481
	  && (*bar).width == width && (*bar).height == height         // C 4482
	{
	  /* Redraw after clear_frame. */
	  if my_show_window (f, hwnd, SW_NORMAL) == 0                  // C 4485
	    { InvalidateRect (hwnd, core::ptr::null (), FALSE); }      // C 4486
	}
      else
	{
	  /* C declares the `SCROLLINFO` uninitialised and fills it below
	     (PORTING.md 1.5).  */
	  let mut hdc: HDC;                                            // C 4490
	  let mut si: SCROLLINFO = core::mem::zeroed ();               // C 4491

	  block_input ();                                              // C 4493
	  if width != 0 && height != 0                                 // C 4494
	    {
	      hdc = get_frame_dc (f);                                  // C 4496
	      /* Since Windows scroll bars are smaller than the space reserved
		 for them on the frame, we have to clear "under" them.  */
	      w32_clear_area! (f, hdc, clear_left, top, clear_width, height);   // C 4499
	      release_frame_dc (f, hdc);                               // C 4500
	      w32_clear_under_internal_border (f);                     // C 4501
	    }
	  /* Make sure scroll bar is "visible" before moving, to ensure the
	     area of the parent window now exposed will be refreshed.  */
	  my_show_window (f, hwnd, SW_HIDE);                           // C 4505
	  /* Try to not draw over child frames.  */
	  SetWindowPos (hwnd, HWND_BOTTOM, left, top, max! (width, 1),
			height, SWP_FRAMECHANGED);                     // C 4508-4509

	  /* +++ SetScrollInfo +++ */
	  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;      // C 4512
	  si.fMask = SIF_PAGE | SIF_POS | SIF_RANGE;                   // C 4513
	  si.nMin = 0;                                                 // C 4514
	  si.nMax = whole;                                             // C 4515
	  si.nPage = (min! (portion, si.nMax) + 1) as UINT;            // C 4516
	  si.nPos = min! (position, si.nMax);                          // C 4517
	  SetScrollInfo (hwnd, SB_CTL, &raw const si, FALSE);           // C 4518

	  my_show_window (f, hwnd, SW_NORMAL);                          // C 4520
	  /* InvalidateRect (w, NULL, FALSE);  */

	  /* Remember new settings.  */
	  (*bar).left = left;                                           // C 4524
	  (*bar).top = top;                                             // C 4525
	  (*bar).width = width;                                         // C 4526
	  (*bar).height = height;                                       // C 4527

	  unblock_input ();                                             // C 4529
	}
    }

  w32_set_horizontal_scroll_bar_thumb (bar, portion, position, whole);   // C 4533
  XSETVECTOR! (barobj, bar);                                          // C 4534
  wset_horizontal_scroll_bar (w, barobj);                             // C 4535
}

/* C 4539-4545's comment ("The following three hooks are used when we're doing a thorough redisplay
   of the frame. …"), carried as C's own text at its position.  */
/* C: /* The following three hooks are used when we're doing a thorough
   C:    redisplay of the frame.  We don't explicitly know which scroll bars
   C:    are going to be deleted, because keeping track of when windows go
   C:    away is a real pain - "Can you say set-window-configuration, boys
   C:    and girls?"  Instead, we just assert at the beginning of redisplay
   C:    that *all* scroll bars are to be removed, and then save a scroll bar
   C:    from the fiery pit when we actually redisplay its window.  */  */

/// C 4547-4571: "Arrange for all scroll bars on FRAME to be removed at the next call to
/// `*judge_scroll_bars_hook'. …" - C 4554's guard, C 4556-4566's prepend (the two-list walk and
/// the two pointer stores) and C 4568-4569's two moves.
/// C: /* Arrange for all scroll bars on FRAME to be removed at the next call
/// C:    to `*judge_scroll_bars_hook'.  A scroll bar may be spared if
/// C:    `*redeem_scroll_bar_hook' is applied to its window before the judgment.  */
/// C: static void
/// C: w32_condemn_scroll_bars (struct frame *frame)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_condemn_scroll_bars (frame: *mut frame)
{
  if !NILP (FRAME_SCROLL_BARS (frame))                                // C 4554
    {
      if !NILP (FRAME_CONDEMNED_SCROLL_BARS (frame))                  // C 4556
	{
	  /* Prepend scrollbars to already condemned ones.  */
	  let mut last: Lisp_Object = FRAME_SCROLL_BARS (frame);       // C 4559

	  while !NILP ((*XSCROLL_BAR! (last)).next)                    // C 4561
	    { last = (*XSCROLL_BAR! (last)).next; }                   // C 4562

	  (*XSCROLL_BAR! (last)).next = FRAME_CONDEMNED_SCROLL_BARS (frame);   // C 4564
	  (*XSCROLL_BAR! (FRAME_CONDEMNED_SCROLL_BARS (frame))).prev = last;   // C 4565
	}

      fset_condemned_scroll_bars (frame, FRAME_SCROLL_BARS (frame));   // C 4568
      fset_scroll_bars (frame, Qnil ());                               // C 4569
    }
}

/// C 4574-4656: "Un-mark WINDOW's scroll bar for deletion in this judgment cycle. …" - C
/// 4580-4582's three locals, C 4585-4586's guard, C 4588-4620's vertical arm (the condemned-list
/// unlink with C 4597-4606's three-way front test - whose C 4599 is the `goto horizontal` the
/// labelled block below serves - and C 4614-4619's re-insert) and C 4623-4655's horizontal twin
/// (whose C 4634 returns directly, C's own `return`).
/// C: /* Un-mark WINDOW's scroll bar for deletion in this judgment cycle.
/// C:    Note that WINDOW isn't necessarily condemned at all.  */
/// C: static void
/// C: w32_redeem_scroll_bar (struct window *w)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_redeem_scroll_bar (w: *mut window)
{
  let mut bar: *mut scroll_bar;                                       // C 4580
  let mut barobj: Lisp_Object;                                        // C 4581
  let mut f: *mut frame;                                              // C 4582

  /* We can't redeem this window's scroll bar if it doesn't have one.  */
  if NILP ((*w).vertical_scroll_bar) && NILP ((*w).horizontal_scroll_bar)   // C 4585
    { emacs_abort (); }                                               // C 4586

  /* C 4622's `horizontal:` label's block: C 4599's `goto horizontal` leaves it, so the vertical
     arm and the label's own line are one labelled block and the horizontal arm follows it (the
     `w32proc` `new_child` reading of the forward-jump device).  */
  'horizontal: {
    if !NILP ((*w).vertical_scroll_bar)                               // C 4588
	&& WINDOW_HAS_VERTICAL_SCROLL_BAR (w)
      {
	bar = XSCROLL_BAR! ((*w).vertical_scroll_bar);                // C 4590
	/* Unlink it from the condemned list.  */
	f = XFRAME (WINDOW_FRAME (w));                                // C 4592
	if NILP ((*bar).prev)                                         // C 4593
	  {
	    /* If the prev pointer is nil, it must be the first in one of
	       the lists.  */
	    if EQ (FRAME_SCROLL_BARS (f), (*w).vertical_scroll_bar)   // C 4597
	      /* It's not condemned.  Everything's fine.  */
	      { break 'horizontal; }                                  // C 4599
	    else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),              // C 4600-4601
			(*w).vertical_scroll_bar)
	      { fset_condemned_scroll_bars (f, (*bar).next); }        // C 4602
	    else
	      /* If its prev pointer is nil, it must be at the front of
		 one or the other!  */
	      { emacs_abort (); }                                     // C 4606
	  }
	else
	  { (*(XSCROLL_BAR! ((*bar).prev))).next = (*bar).next; }     // C 4609

	if !NILP ((*bar).next)                                        // C 4611
	  { (*(XSCROLL_BAR! ((*bar).next))).prev = (*bar).prev; }     // C 4612

	(*bar).next = FRAME_SCROLL_BARS (f);                          // C 4614
	(*bar).prev = Qnil ();                                        // C 4615
	XSETVECTOR! (barobj, bar);                                    // C 4616
	fset_scroll_bars (f, barobj);                                 // C 4617
	if !NILP ((*bar).next)                                        // C 4618
	  { XSETVECTOR! ((*(XSCROLL_BAR! ((*bar).next))).prev, bar); }   // C 4619
      }
  }

  if !NILP ((*w).horizontal_scroll_bar)                               // C 4623
      && WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)
    {
      bar = XSCROLL_BAR! ((*w).horizontal_scroll_bar);                // C 4625
      /* Unlink it from the condemned list.  */
      f = XFRAME (WINDOW_FRAME (w));                                  // C 4627
      if NILP ((*bar).prev)                                           // C 4628
	{
	  /* If the prev pointer is nil, it must be the first in one of
	     the lists.  */
	  if EQ (FRAME_SCROLL_BARS (f), (*w).horizontal_scroll_bar)   // C 4632
	    /* It's not condemned.  Everything's fine.  */
	    { return; }                                               // C 4634
	  else if EQ (FRAME_CONDEMNED_SCROLL_BARS (f),                // C 4635-4636
		      (*w).horizontal_scroll_bar)
	    { fset_condemned_scroll_bars (f, (*bar).next); }          // C 4637
	  else
	    /* If its prev pointer is nil, it must be at the front of
	       one or the other!  */
	    { emacs_abort (); }                                       // C 4641
	}
      else
	{ (*(XSCROLL_BAR! ((*bar).prev))).next = (*bar).next; }       // C 4644

      if !NILP ((*bar).next)                                          // C 4646
	{ (*(XSCROLL_BAR! ((*bar).next))).prev = (*bar).prev; }       // C 4647

      (*bar).next = FRAME_SCROLL_BARS (f);                            // C 4649
      (*bar).prev = Qnil ();                                          // C 4650
      XSETVECTOR! (barobj, bar);                                      // C 4651
      fset_scroll_bars (f, barobj);                                   // C 4652
      if !NILP ((*bar).next)                                          // C 4653
	{ XSETVECTOR! ((*(XSCROLL_BAR! ((*bar).next))).prev, bar); }   // C 4654
    }
}

/// C 4658-4684: "Remove all scroll bars on FRAME that haven't been saved since the last call to
/// `*condemn_scroll_bars_hook'." - C 4666's read, C 4670's clear, C 4672-4680's removal walk (with
/// C 4678-4679's two stores) and C 4682-4683's comment.
/// C: /* Remove all scroll bars on FRAME that haven't been saved since the
/// C:    last call to `*condemn_scroll_bars_hook'.  */
/// C: static void
/// C: w32_judge_scroll_bars (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_judge_scroll_bars (f: *mut frame)
{
  let mut bar: Lisp_Object;                                           // C 4664
  let mut next: Lisp_Object;

  bar = FRAME_CONDEMNED_SCROLL_BARS (f);                              // C 4666

  /* Clear out the condemned list now so we won't try to process any
     more events on the hapless scroll bars.  */
  fset_condemned_scroll_bars (f, Qnil ());                            // C 4670

  /* C's `for (; ! NILP (bar); bar = next)` (C 4672-4673) is the initialisation C 4666 made, the
     condition and C's increment clause.  */
  while !NILP (bar)                                                   // C 4672
    {
      let b: *mut scroll_bar = XSCROLL_BAR! (bar);                    // C 4674

      w32_scroll_bar_remove (b);                                     // C 4676

      next = (*b).next;                                              // C 4678
      (*b).next = Qnil ();                                           // C 4679
      (*b).prev = Qnil ();
      bar = next;                                                     // C 4672 (C's increment clause)
    }

  /* Now there should be no references to the condemned scroll bars,
     and they should get garbage-collected.  */
}

/* C's `<winuser.h>` scroll-bar values C 4716-4905 reads and this unit does not carry yet: the nine
   `SB_*` commands (`SB_LINEUP`/`SB_LINEDOWN`/`SB_PAGEUP`/`SB_PAGEDOWN`/`SB_THUMBPOSITION`/
   `SB_THUMBTRACK`/`SB_TOP`/`SB_BOTTOM`/`SB_ENDSCROLL` and their horizontal counterparts
   `SB_LINELEFT`/`SB_LINERIGHT`/`SB_PAGELEFT`/`SB_PAGERIGHT`/`SB_LEFT`/`SB_RIGHT`, whose numeric
   values the SDK shares with the vertical set) and `SIF_TRACKPOS`.  Every name stands at the use
   site (P-88 (1)); the SDK values are UNVERIFIED.  */
/// C: #define SB_LINEUP 0 / #define SB_LINELEFT 0
/// C: #define SB_LINEDOWN 1 / #define SB_LINERIGHT 1
/// C: #define SB_PAGEUP 2 / #define SB_PAGELEFT 2
/// C: #define SB_PAGEDOWN 3 / #define SB_PAGERIGHT 3
/// C: #define SB_THUMBPOSITION 4 / #define SB_THUMBTRACK 5
/// C: #define SB_TOP 6 / #define SB_LEFT 6
/// C: #define SB_BOTTOM 7 / #define SB_RIGHT 7
/// C: #define SB_ENDSCROLL 8
/// C: #define SIF_TRACKPOS 0x0010
#[cfg (feature = "have_ntgui")]
pub const SB_LINEUP: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SB_LINELEFT: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SB_LINEDOWN: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const SB_LINERIGHT: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const SB_PAGEUP: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const SB_PAGELEFT: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const SB_PAGEDOWN: c_int = 3;
#[cfg (feature = "have_ntgui")]
pub const SB_PAGERIGHT: c_int = 3;
#[cfg (feature = "have_ntgui")]
pub const SB_THUMBPOSITION: c_int = 4;
#[cfg (feature = "have_ntgui")]
pub const SB_THUMBTRACK: c_int = 5;
#[cfg (feature = "have_ntgui")]
pub const SB_TOP: c_int = 6;
#[cfg (feature = "have_ntgui")]
pub const SB_LEFT: c_int = 6;
#[cfg (feature = "have_ntgui")]
pub const SB_BOTTOM: c_int = 7;
#[cfg (feature = "have_ntgui")]
pub const SB_RIGHT: c_int = 7;
#[cfg (feature = "have_ntgui")]
pub const SB_ENDSCROLL: c_int = 8;
#[cfg (feature = "have_ntgui")]
pub const SIF_TRACKPOS: UINT = 0x0010;

/// C 4686-4800: "Handle a mouse click on the vertical scroll bar BAR. …" - C 4697-4698's guard, C
/// 4700-4706's six event fields, C 4709-4713's five locals, C 4715-4725's `SCROLLINFO` probe (with
/// C 4721's `GetScrollInfo` and C 4722-4725's position choice), C 4727-4729's dragging reset and
/// the display-info store, C 4731-4793's `switch (sb_event)` over C's fourteen labels (C 4759-4770's
/// nested position store, C 4776-4787's drag tail and C 4788-4792's `FALLTHROUGH` into `default`)
/// and C 4795-4798's two setters and `TRUE` answer.
/// C: /* Handle a mouse click on the vertical scroll bar BAR.  If
/// C:    *EMACS_EVENT's kind is set to something other than NO_EVENT, it is
/// C:    enqueued.
/// C:
/// C:    This may be called from a signal handler, so we have to ignore GC
/// C:    mark bits.  */
/// C: static int
/// C: w32_scroll_bar_handle_click (struct scroll_bar *bar, W32Msg *msg,
/// C: 			     struct input_event *emacs_event)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_bar_handle_click (bar: *mut scroll_bar, msg: *mut W32Msg,
				       emacs_event: *mut input_event) -> c_int
{
  if !WINDOWP ((*bar).window)                                         // C 4697
    { emacs_abort (); }                                               // C 4698

  input_event_bits! (kind, emacs_event, SCROLL_BAR_CLICK_EVENT);      // C 4700
  (*emacs_event).code = 0;                                            // C 4701
  /* not really meaningful to distinguish up/down */
  (*emacs_event).modifiers = (*msg).dwModifiers;                      // C 4703
  (*emacs_event).frame_or_window = (*bar).window;                     // C 4704
  (*emacs_event).arg = Qnil ();                                       // C 4705
  (*emacs_event).timestamp = (*msg).msg.time as Time;                 // C 4706

  {
    /* C 4709 passes C's dead macro argument `f` (`VERTICAL_SCROLL_BAR_INSIDE_HEIGHT` never
       expands it), and C 4728's own `struct frame *f;  /* Value is not used.  */` declaration
       below is what makes the name visible to C 4729's `FRAME_DISPLAY_INFO` - which is this
       unit's `(&one_w32_display_info)` macro and ignores it the same way.  */
    let top_range: c_int = VERTICAL_SCROLL_BAR_TOP_RANGE! (f, (*bar).height);   // C 4709
    let mut y: c_int;                                                 // C 4710
    let dragging: c_int = (*bar).dragging;                            // C 4711
    let mut si: SCROLLINFO;                                           // C 4712
    let sb_event: c_int = LOWORD! ((*msg).msg.wParam) as c_int;       // C 4713

    si = core::mem::zeroed ();                                        // C 4712 (C's uninitialised `SCROLLINFO`)
    si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;          // C 4715
    if sb_event == SB_THUMBTRACK                                      // C 4716
      { si.fMask = SIF_TRACKPOS; }                                    // C 4717
    else
      { si.fMask = SIF_POS; }                                         // C 4719

    GetScrollInfo ((*msg).msg.lParam as HWND, SB_CTL, &raw mut si);   // C 4721
    if sb_event == SB_THUMBTRACK                                      // C 4722
      { y = si.nTrackPos; }                                           // C 4723
    else
      { y = si.nPos; }                                                // C 4725

    (*bar).dragging = 0;                                              // C 4727
    let mut f: *mut frame;	/* Value is not used.  */                 // C 4728
    (*FRAME_DISPLAY_INFO! (f)).last_mouse_scroll_bar_pos
      = (*msg).msg.wParam as c_int;                                   // C 4729

    match sb_event                                                    // C 4731
      {
      SB_LINEDOWN =>                                                  // C 4733
	{ input_event_bits! (part, emacs_event, scroll_bar_down_arrow); }   // C 4734

      SB_LINEUP =>                                                    // C 4736
	{ input_event_bits! (part, emacs_event, scroll_bar_up_arrow); }   // C 4737

      SB_PAGEUP =>                                                    // C 4739
	{ input_event_bits! (part, emacs_event, scroll_bar_above_handle); }   // C 4740

      SB_PAGEDOWN =>                                                  // C 4742
	{ input_event_bits! (part, emacs_event, scroll_bar_below_handle); }   // C 4743

      SB_TOP =>                                                       // C 4745
	{
	  input_event_bits! (part, emacs_event, scroll_bar_handle);    // C 4746
	  y = 0;                                                      // C 4747
	}

      SB_BOTTOM =>                                                    // C 4749
	{
	  input_event_bits! (part, emacs_event, scroll_bar_handle);    // C 4750
	  y = top_range;                                              // C 4751
	}

      SB_THUMBTRACK | SB_THUMBPOSITION =>                             // C 4753-4754
	{
	  (*bar).dragging = 1; /* ??????? */                           // C 4755
	  input_event_bits! (part, emacs_event, scroll_bar_handle);    // C 4756

	  /* "Silently" update current position.  */
	  {
	    /* C declares this inner `SCROLLINFO` uninitialised and fills it below
	       (PORTING.md 1.5).  */
	    let mut si: SCROLLINFO = core::mem::zeroed ();             // C 4760

	    si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;   // C 4762
	    si.fMask = SIF_POS;                                       // C 4763
	    si.nPos = y;                                              // C 4764
	    /* Remember apparent position (we actually lag behind the real
	       position, so don't set that directly).  */
	    last_scroll_bar_drag_pos = y;                             // C 4767

	    SetScrollInfo (SCROLL_BAR_W32_WINDOW! (bar), SB_CTL, &raw const si, FALSE);   // C 4769
	  }
	}

      SB_ENDSCROLL =>                                                 // C 4772
	{
	  /* If this is the end of a drag sequence, then reset the scroll
	     handle size to normal and do a final redraw.  Otherwise do
	     nothing.  */
	  if dragging != 0                                            // C 4776
	    {
	      /* C declares this inner `SCROLLINFO` uninitialised and fills it below
		 (PORTING.md 1.5).  */
	      let mut si: SCROLLINFO = core::mem::zeroed ();           // C 4778
	      let start: c_int = (*bar).start;                         // C 4779
	      let end: c_int = (*bar).end;                             // C 4780

	      si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;   // C 4782
	      si.fMask = SIF_PAGE | SIF_POS;                           // C 4783
	      si.nPage = (end - start + VERTICAL_SCROLL_BAR_MIN_HANDLE! ()) as UINT;   // C 4784
	      si.nPos = last_scroll_bar_drag_pos;                      // C 4785
	      SetScrollInfo (SCROLL_BAR_W32_WINDOW! (bar), SB_CTL, &raw const si, TRUE);   // C 4786
	    }
	  /* C's `FALLTHROUGH;` sends this arm on into the `default:` arm's two statements, which
	     the port therefore writes here too (PORTING.md 2.2).  */
	  input_event_bits! (kind, emacs_event, NO_EVENT);             // C 4791
	  return FALSE;                                                // C 4792
	}

      /* C's `default:` (C 4790-4792).  */
      _ =>
	{
	  input_event_bits! (kind, emacs_event, NO_EVENT);             // C 4791
	  return FALSE;                                                // C 4792
	}
      }

    XSETINT! ((*emacs_event).x, y as EMACS_INT);                       // C 4795
    XSETINT! ((*emacs_event).y, top_range as EMACS_INT);               // C 4796

    return TRUE;                                                       // C 4798
  }
}

/// C 4802-4915: "Handle a mouse click on the horizontal scroll bar BAR. …" - the horizontal twin of
/// C 4686-4800: C 4825's `left_range`, C 4826's extra `y`, C 4832-4842's `SIF_TRACKPOS | SIF_PAGE |
/// SIF_RANGE` probe with C 4842's `y = si.nMax - si.nPage`, C 4848-4908's `switch (sb_event)` over
/// C's thirteen labels (C 4881's `min (x, XWINDOW (bar->window)->hscroll_whole - 1)` and C
/// 4899-4900's min of the remembered position) and C 4910-4913's two setters and `TRUE`.
/// C: static int
/// C: w32_horizontal_scroll_bar_handle_click (struct scroll_bar *bar, W32Msg *msg,
/// C: 					struct input_event *emacs_event)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_horizontal_scroll_bar_handle_click (bar: *mut scroll_bar, msg: *mut W32Msg,
						  emacs_event: *mut input_event) -> c_int
{
  if !WINDOWP ((*bar).window)                                         // C 4813
    { emacs_abort (); }                                               // C 4814

  input_event_bits! (kind, emacs_event, HORIZONTAL_SCROLL_BAR_CLICK_EVENT);   // C 4816
  (*emacs_event).code = 0;                                            // C 4817
  /* not really meaningful to distinguish left/right */
  (*emacs_event).modifiers = (*msg).dwModifiers;                      // C 4819
  (*emacs_event).frame_or_window = (*bar).window;                     // C 4820
  (*emacs_event).arg = Qnil ();                                       // C 4821
  (*emacs_event).timestamp = (*msg).msg.time as Time;                 // C 4822

  {
    /* C 4825 and C 4846 pass C's dead macro argument `f` and C 4845 declares the same unused
       `struct frame *f;` as the vertical twin above.  */
    let left_range: c_int = HORIZONTAL_SCROLL_BAR_LEFT_RANGE! (f, (*bar).width);   // C 4825
    let mut x: c_int;                                                 // C 4826
    let mut y: c_int;
    let dragging: c_int = (*bar).dragging;                            // C 4827
    let mut si: SCROLLINFO;                                           // C 4828
    let sb_event: c_int = LOWORD! ((*msg).msg.wParam) as c_int;       // C 4829

    si = core::mem::zeroed ();                                        // C 4828 (C's uninitialised `SCROLLINFO`)
    si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;          // C 4831
    if sb_event == SB_THUMBTRACK                                      // C 4832
      { si.fMask = SIF_TRACKPOS | SIF_PAGE | SIF_RANGE; }             // C 4833
    else
      { si.fMask = SIF_POS | SIF_PAGE | SIF_RANGE; }                  // C 4835

    GetScrollInfo ((*msg).msg.lParam as HWND, SB_CTL, &raw mut si);   // C 4837
    if sb_event == SB_THUMBTRACK                                      // C 4838
      { x = si.nTrackPos; }                                           // C 4839
    else
      { x = si.nPos; }                                                // C 4841
    y = si.nMax - si.nPage as c_int;                                  // C 4842

    (*bar).dragging = 0;                                              // C 4844
    let mut f: *mut frame;	/* Value is not used.  */                 // C 4845
    (*FRAME_DISPLAY_INFO! (f)).last_mouse_scroll_bar_pos
      = (*msg).msg.wParam as c_int;                                   // C 4846

    match sb_event                                                    // C 4848
      {
      SB_LINELEFT =>                                                  // C 4850
	{ input_event_bits! (part, emacs_event, scroll_bar_left_arrow); }   // C 4851

      SB_LINERIGHT =>                                                 // C 4853
	{ input_event_bits! (part, emacs_event, scroll_bar_right_arrow); }   // C 4854

      SB_PAGELEFT =>                                                  // C 4856
	{ input_event_bits! (part, emacs_event, scroll_bar_before_handle); }   // C 4857

      SB_PAGERIGHT =>                                                 // C 4859
	{ input_event_bits! (part, emacs_event, scroll_bar_after_handle); }   // C 4860

      SB_LEFT =>                                                      // C 4862
	{
	  input_event_bits! (part, emacs_event, scroll_bar_horizontal_handle);   // C 4863
	  x = 0;                                                      // C 4864
	}

      SB_RIGHT =>                                                     // C 4866
	{
	  input_event_bits! (part, emacs_event, scroll_bar_horizontal_handle);   // C 4867
	  x = left_range;                                             // C 4868
	}

      SB_THUMBTRACK | SB_THUMBPOSITION =>                             // C 4870-4871
	{
	  (*bar).dragging = 1;                                          // C 4872
	  input_event_bits! (part, emacs_event, scroll_bar_horizontal_handle);   // C 4873

	  /* "Silently" update current position.  */
	  {
	    /* C declares this inner `SCROLLINFO` uninitialised and fills it below
	       (PORTING.md 1.5).  */
	    let mut si: SCROLLINFO = core::mem::zeroed ();             // C 4877

	    si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;   // C 4879
	    si.fMask = SIF_POS;                                       // C 4880
	    si.nPos = min! (x, (*XWINDOW ((*bar).window)).hscroll_whole as c_int - 1) as c_int;   // C 4881
	    /* Remember apparent position (we actually lag behind the real
	       position, so don't set that directly).  */
	    last_scroll_bar_drag_pos = x;                              // C 4884

	    SetScrollInfo (SCROLL_BAR_W32_WINDOW! (bar), SB_CTL, &raw const si, FALSE);   // C 4886
	  }
	}

      SB_ENDSCROLL =>                                                 // C 4889
	{
	  /* If this is the end of a drag sequence, then reset the scroll
	     handle size to normal and do a final redraw.  Otherwise do
	     nothing.  */
	  if dragging != 0                                            // C 4893
	    {
	      /* C declares this inner `SCROLLINFO` uninitialised and fills it below
		 (PORTING.md 1.5).  */
	      let mut si: SCROLLINFO = core::mem::zeroed ();           // C 4895

	      si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;   // C 4897
	      si.fMask = SIF_POS;                                      // C 4898
	      si.nPos = min! (last_scroll_bar_drag_pos,
			      (*XWINDOW ((*bar).window)).hscroll_whole as c_int - 1) as c_int;   // C 4899-4900
	      SetScrollInfo (SCROLL_BAR_W32_WINDOW! (bar), SB_CTL, &raw const si, TRUE);   // C 4901
	    }
	  /* C's `FALLTHROUGH;` sends this arm on into the `default:` arm's two statements, which
	     the port therefore writes here too (PORTING.md 2.2).  */
	  input_event_bits! (kind, emacs_event, NO_EVENT);             // C 4906
	  return FALSE;                                                // C 4907
	}

      /* C's `default:` (C 4905-4907).  */
      _ =>
	{
	  input_event_bits! (kind, emacs_event, NO_EVENT);             // C 4906
	  return FALSE;                                                // C 4907
	}
      }

    XSETINT! ((*emacs_event).x, x as EMACS_INT);                       // C 4910
    XSETINT! ((*emacs_event).y, y as EMACS_INT);                       // C 4911

    return TRUE;                                                       // C 4913
  }
}


/* C 4917-4918's banner comment, carried as C's own text at its position.  */
/* C: /* Return information to the user about the current position of the mouse
   C:    on the vertical scroll bar.  */  */

/// C 4919-4965: `static void w32_scroll_bar_report_motion (struct frame **fp, Lisp_Object
/// *bar_window, enum scroll_bar_part *part, Lisp_Object *x, Lisp_Object *y, Time *time)` - C
/// 4925's `FRAME_DISPLAY_INFO` read, C 4926's `last_mouse_scroll_bar`, C 4927's
/// `SCROLL_BAR_W32_WINDOW`, C 4928's `XFRAME (WINDOW_FRAME (XWINDOW …))`, C 4929's `pos` and C
/// 4930's `top_range` from `VERTICAL_SCROLL_BAR_TOP_RANGE`, C 4931's `SCROLLINFO` and C 4932's
/// `LOWORD (…)` read; C 4934's `block_input`, C 4936-4937's two stores, C 4939-4943's
/// `cbSize`/`fMask` choice, C 4945's `GetScrollInfo` with C 4946-4949's position choice and C
/// 4950's range recompute, C 4952's `scroll_bar_handle` with C 4953-4954's `SB_LINEDOWN` step, C
/// 4956-4957's two `XSETINT`s, C 4959-4960's flag clear and handle drop, C 4962's time store and
/// C 4964's `unblock_input`.
/// C: static void
/// C: w32_scroll_bar_report_motion (struct frame **fp, Lisp_Object *bar_window,
/// C: 			    enum scroll_bar_part *part,
/// C: 			    Lisp_Object *x, Lisp_Object *y,
/// C: 			    Time *time)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_bar_report_motion (fp: *mut *mut frame, bar_window: *mut Lisp_Object,
					part: *mut scroll_bar_part, x: *mut Lisp_Object,
					y: *mut Lisp_Object, time: *mut Time)
{
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO! (*fp);      // C 4925
  let bar: *mut scroll_bar = (*dpyinfo).last_mouse_scroll_bar;         // C 4926
  let w: Window = SCROLL_BAR_W32_WINDOW! (bar);                        // C 4927
  let f: *mut frame = XFRAME (WINDOW_FRAME (XWINDOW ((*bar).window)));  // C 4928
  let mut pos: c_int;                                                  // C 4929
  let mut top_range: c_int = VERTICAL_SCROLL_BAR_TOP_RANGE! (f, (*bar).height);   // C 4930
  /* C declares the `SCROLLINFO` uninitialised and fills it below (PORTING.md 1.5).  */
  let mut si: SCROLLINFO = core::mem::zeroed ();                       // C 4931
  let sb_event: c_int = LOWORD! ((*dpyinfo).last_mouse_scroll_bar_pos) as c_int;   // C 4932

  block_input ();                                                      // C 4934

  *fp = f;                                                             // C 4936
  *bar_window = (*bar).window;                                         // C 4937

  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;             // C 4939
  if sb_event == SB_THUMBTRACK                                        // C 4940
    { si.fMask = SIF_TRACKPOS | SIF_PAGE | SIF_RANGE; }               // C 4941
  else
    { si.fMask = SIF_POS | SIF_PAGE | SIF_RANGE; }                    // C 4943

  GetScrollInfo (w, SB_CTL, &raw mut si);                              // C 4945
  if sb_event == SB_THUMBTRACK                                        // C 4946
    { pos = si.nTrackPos; }                                           // C 4947
  else
    { pos = si.nPos; }                                                // C 4949
  top_range = si.nMax - si.nPage as c_int + 1;                         // C 4950

  *part = scroll_bar_handle;                                           // C 4952
  if sb_event == SB_LINEDOWN                                          // C 4953
    { pos += 1; }                                                     // C 4954

  XSETINT! (*x, pos as EMACS_INT);                                     // C 4956
  XSETINT! (*y, top_range as EMACS_INT);                               // C 4957

  frame_bits! (mouse_moved, f, false);                                 // C 4959
  (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();           // C 4960

  *time = (*dpyinfo).last_mouse_movement_time;                         // C 4962

  unblock_input ();                                                    // C 4964
}

/* C 4967-4968's banner comment, carried as C's own text at its position.  */
/* C: /* Return information to the user about the current position of the mouse
   C:    on the horizontal scroll bar.  */  */

/// C 4969-5016: `static void w32_horizontal_scroll_bar_report_motion (struct frame **fp,
/// Lisp_Object *bar_window, enum scroll_bar_part *part, Lisp_Object *x, Lisp_Object *y,
/// Time *time)` - the horizontal twin, line for line: C 4980's `left_range` from
/// `HORIZONTAL_SCROLL_BAR_LEFT_RANGE`, C 5000's range recompute, C 5003's `SB_LINERIGHT` step and
/// C 5006's blank line (C's own, carried), then C 5007-5008's two `XSETINT`s in C's `y`-then-`x`
/// order.
/// C: static void
/// C: w32_horizontal_scroll_bar_report_motion (struct frame **fp, Lisp_Object *bar_window,
/// C:                                          enum scroll_bar_part *part,
/// C:                                          Lisp_Object *x, Lisp_Object *y,
/// C:                                          Time *time)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_horizontal_scroll_bar_report_motion (fp: *mut *mut frame,
						   bar_window: *mut Lisp_Object,
						   part: *mut scroll_bar_part,
						   x: *mut Lisp_Object, y: *mut Lisp_Object,
						   time: *mut Time)
{
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO! (*fp);      // C 4975
  let bar: *mut scroll_bar = (*dpyinfo).last_mouse_scroll_bar;         // C 4976
  let w: Window = SCROLL_BAR_W32_WINDOW! (bar);                        // C 4977
  let f: *mut frame = XFRAME (WINDOW_FRAME (XWINDOW ((*bar).window)));  // C 4978
  let mut pos: c_int;                                                  // C 4979
  let mut left_range: c_int = HORIZONTAL_SCROLL_BAR_LEFT_RANGE! (f, (*bar).width);   // C 4980
  /* C declares the `SCROLLINFO` uninitialised and fills it below (PORTING.md 1.5).  */
  let mut si: SCROLLINFO = core::mem::zeroed ();                       // C 4981
  let sb_event: c_int = LOWORD! ((*dpyinfo).last_mouse_scroll_bar_pos) as c_int;   // C 4982

  block_input ();                                                      // C 4984

  *fp = f;                                                             // C 4986
  *bar_window = (*bar).window;                                         // C 4987

  si.cbSize = core::mem::size_of::<SCROLLINFO> () as UINT;             // C 4989
  if sb_event == SB_THUMBTRACK                                        // C 4990
    { si.fMask = SIF_TRACKPOS | SIF_PAGE | SIF_RANGE; }               // C 4991
  else
    { si.fMask = SIF_POS | SIF_PAGE | SIF_RANGE; }                    // C 4993

  GetScrollInfo (w, SB_CTL, &raw mut si);                              // C 4995
  if sb_event == SB_THUMBTRACK                                        // C 4996
    { pos = si.nTrackPos; }                                           // C 4997
  else
    { pos = si.nPos; }                                                // C 4999
  left_range = si.nMax - si.nPage as c_int + 1;                        // C 5000

  *part = scroll_bar_handle;                                           // C 5002
  if sb_event == SB_LINERIGHT                                         // C 5003
    { pos += 1; }                                                     // C 5004

  /* C 5006 is blank.  */

  XSETINT! (*y, pos as EMACS_INT);                                     // C 5007
  XSETINT! (*x, left_range as EMACS_INT);                              // C 5008

  frame_bits! (mouse_moved, f, false);                                 // C 5010
  (*dpyinfo).last_mouse_scroll_bar = core::ptr::null_mut ();           // C 5011

  *time = (*dpyinfo).last_mouse_movement_time;                         // C 5013

  unblock_input ();                                                    // C 5015
}

/* C 5019-5022's banner comment, carried as C's own text at its position.  */
/* C: /* The screen has been cleared so we may have changed foreground or
   C:    background colors, and the scroll bars may need to be redrawn.
   C:    Clear out the scroll bars, and ask for expose events, so we can
   C:    redraw them.  */  */

/// C 5023-5061: `static void w32_scroll_bar_clear (struct frame *f)` - C 5027's `bar`, C
/// 5029-5035's double-buffering guard (C 5034's `want_paint_buffer` bit read the
/// `src/w32xfns.rs` 639 way, from the C 207-208 pair it copies), C 5037-5041's scroll-bar-kind
/// guard and C 5042-5060's `for` sweep (C 5045-5047's window/DC/`RECT`, C 5049-5051's hide, C
/// 5053-5057's client rect, palette and clear, C 5059's `ReleaseDC`), with C's increment clause
/// written at the loop's end (`w32_window_to_scroll_bar`'s shape above).
/// C: static void
/// C: w32_scroll_bar_clear (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_scroll_bar_clear (f: *mut frame)
{
  let mut bar: Lisp_Object;                                           // C 5027

  /* Return if double buffering is enabled, since clearing a frame
     actually clears just the back buffer, so avoid clearing all of
     the scroll bars, since that causes the scroll bars to
     flicker.  */
  if !w32_disable_double_buffering                                    // C 5033
    && w32_output_bits! (want_paint_buffer, FRAME_OUTPUT_DATA! (f)) != 0   // C 5034
    { return; }                                                       // C 5035

  /* We can have scroll bars even if this is 0,
     if we just turned off scroll bar mode.
     But in that case we should not clear them.  */
  if FRAME_HAS_VERTICAL_SCROLL_BARS (f) != 0                          // C 5040
    || FRAME_HAS_HORIZONTAL_SCROLL_BARS (f) != 0                      // C 5041
    {
      bar = FRAME_SCROLL_BARS (f);                                    // C 5042
      while VECTORP (bar)                                             // C 5043
	{
	  let window: HWND = SCROLL_BAR_W32_WINDOW! (XSCROLL_BAR! (bar));   // C 5045
	  let hdc: HDC = GetDC (window);                                // C 5046
	  /* C declares the `RECT` uninitialised and fills it with `GetClientRect`
	     (PORTING.md 1.5).  */
	  let mut rect: RECT = core::mem::zeroed ();                    // C 5047

	  /* Hide scroll bar until ready to repaint.  x_scroll_bar_move
	     arranges to refresh the scroll bar if hidden.  */
	  my_show_window (f, window, SW_HIDE);                          // C 5051

	  GetClientRect (window, &raw mut rect);                        // C 5053
	  select_palette (f, hdc);                                      // C 5054
	  w32_clear_rect! (f, hdc, &raw mut rect);                      // C 5055
	  w32_clear_under_internal_border (f);                          // C 5056
	  deselect_palette (f, hdc);                                    // C 5057

	  ReleaseDC (window, hdc);                                      // C 5059

	  bar = (*XSCROLL_BAR! (bar)).next;                             // C 5043
	}
    }
}

/* C 5063-5076: the socket reader's banner and buffer comments with the five `static` objects C
   5068-5076 declares for it, carried as C's own text at their positions.  All five are C
   `static`, so the port writes them without `pub` (the `last_mousemove_x/y` shape above).  */
/* C: /* The main W32 event-reading loop - w32_read_socket.  */
   C:
   C: /* Record the last 100 characters stored
   C:    to help debug the loss-of-chars-during-GC problem.  */
   C:
   C: /* Temporarily store lead byte of DBCS input sequences.  */
   C:
   C: /* Temporarily store pending UTF-16 high surrogate unit and the modifiers.  */  */
/// C: static int temp_index;
/// C: static short temp_buffer[100];
/// C: static char dbcs_lead = 0;
/// C: static unsigned short utf16_high;
/// C: static DWORD utf16_high_modifiers;
#[cfg (feature = "have_ntgui")]
static mut temp_index: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut temp_buffer: [c_short; 100] = [0; 100];
#[cfg (feature = "have_ntgui")]
static mut dbcs_lead: c_char = 0;
#[cfg (feature = "have_ntgui")]
static mut utf16_high: c_ushort = 0;
#[cfg (feature = "have_ntgui")]
static mut utf16_high_modifiers: DWORD = 0;

/* C 5078-5085's doxygen comment, carried as C's own text at its position.  */
/* C: /**
   C:   mouse_or_wdesc_frame: When not dropping and the mouse was grabbed
   C:   for DPYINFO, return the frame where the mouse was seen last.  If
   C:   there's no such frame, return the frame according to WDESC.  When
   C:   dropping, return the frame according to WDESC.  If there's no such
   C:   frame and the mouse was grabbed for DPYINFO, return the frame where
   C:   the mouse was seen last.  In either case, never return a tooltip
   C:   frame.  */  */

/// C 5086-5107: `static struct frame *mouse_or_wdesc_frame (struct w32_display_info *dpyinfo,
/// HWND wdesc)` - C 5089-5091's `gui_mouse_grabbed`/`last_mouse_frame` ternary (an `if`
/// expression), C 5093's drop test, C 5097's `w32_window_to_frame`, C 5100's tooltip guard with C
/// 5101's ternary and C 5105's answer.  C's `static`, so a plain `unsafe fn`; the rendering is
/// `src/xterm.rs` 29989's copy of the X-side function of the same shape (which names its own
/// `src/androidterm.rs` 768-790 sibling), minus the reference's `Qdrag_source` clause, which its
/// version of C 5093 does not have.
/// C: static struct frame *
/// C: mouse_or_wdesc_frame (struct w32_display_info *dpyinfo, HWND wdesc)
#[cfg (feature = "have_ntgui")]
unsafe fn mouse_or_wdesc_frame (dpyinfo: *mut w32_display_info, wdesc: HWND) -> *mut frame
{
  let lm_f: *mut frame = if gui_mouse_grabbed (dpyinfo)              // C 5089-5091
    { (*dpyinfo).last_mouse_frame } else { core::ptr::null_mut () };

  if !lm_f.is_null () && !EQ (track_mouse, Qdropping ())               // C 5093
    { return lm_f; }                                                   // C 5094
  else
    {
      let w_f: *mut frame = w32_window_to_frame (dpyinfo, wdesc);      // C 5097

      /* Do not return a tooltip frame.  */
      if w_f.is_null () || FRAME_TOOLTIP_P (w_f)                       // C 5100
	{ return if EQ (track_mouse, Qdropping ()) { lm_f } else { core::ptr::null_mut () }; }   // C 5101
      else
	/* When dropping it would be probably nice to raise w_f
	   here.  */
	{ return w_f; }                                                // C 5105
    }
}

/* C 5109-5126's comment on the socket reader's repost discipline, carried as C's own text at its
   position.  */
/* C: /* Read events coming from the W32 shell.
   C:    This routine is called by the SIGIO handler.
   C:    We return as soon as there are no more events to be read.
   C:
   C:    For an overview of how Emacs input works on MS-Windows, see the
   C:    commentary before w32_msg_pump in w32fns.c.
   C:
   C:    We return the number of characters stored into the buffer,
   C:    thus pretending to be `read'.
   C:
   C:    Some of these messages are reposted back to the message queue since the
   C:    system calls the windows proc directly in a context where we cannot return
   C:    the data nor can we guarantee the state we are in.  So if we dispatch them
   C:    we will get into an infinite loop.  To prevent this from ever happening we
   C:    will set a variable to indicate we are in the read_socket call and indicate
   C:    which message we are processing since the windows proc gets called
   C:    recursively with different messages by the system.
   C: */  */

/// C 5128: `extern void menubar_selection_callback (struct frame *, void *);` - the socket
/// reader's own forward declaration, so the port carries it as a declaration at the use site
/// (P-88 (1)); C's definition is `w32menu.c` 386's, ported at `src/w32menu.rs` 1216 (whose own
/// comment keeps C's text and drops the declaration per PORTING.md 0.1-12).
/// C: extern void menubar_selection_callback (struct frame *, void *);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn menubar_selection_callback (f: *mut frame, client_data: *mut c_void);
}

/* C 5130-5131's `static int w32_read_socket (struct terminal *terminal, struct input_event
   *hold_quit)` opens the next slice.  */

/* C 5274/5300/5415 use `countof`, which C 23's `lisp.h` (28) reaches through
   `<stdcountof.h>`; the mirror chain this unit includes does not carry it, so the unit includes the
   mirror where C reaches it (the `src/cm.rs` 33 shape).  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdcountof.rs");       // lisp.h 28: `#include <stdcountof.h>` (`countof`)

/* dispextern.h 3019-3023's three `bool_bf : 1` members of `struct Mouse_HLInfo` are the mirror's
   single `bits` byte with three masks, so `hlinfo->mouse_face_hidden` is a pack access - the
   `src/xterm.rs` 31041 `mouse_face_hidden!` pair, brought to this unit because C 5266/5292 read it
   and C 5271/5297 write it.  */
#[cfg (feature = "have_ntgui")]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* frame.h 460's `ENUM_BF (fullscreen_type) want_fullscreen : 4;` is one of the flag run's packed
   fields (arm A's word 2, bits 0-3 - `docs/baseline/frame-bits.tsv`), and `include/frame.rs` has
   the mask/shift/unit and the reader but no accessor, so the pair stands here (the
   `src/xterm.rs` 16556 copy).  C 5769 reads it against `FULLSCREEN_WAIT`; the write arm is C's own
   masked write on `bits2`, which the later slices' `w32fullscreen_hook` needs.  */
#[cfg (feature = "have_ntgui")]
macro_rules! want_fullscreen {
  ($f:expr) => {
	 frame_field ($f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT,
		     FRAME_WANT_FULLSCREEN_MASK) as fullscreen_type };
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & !(FRAME_WANT_FULLSCREEN_MASK << FRAME_WANT_FULLSCREEN_SHIFT))
                  | (((($v) as u32) & FRAME_WANT_FULLSCREEN_MASK) << FRAME_WANT_FULLSCREEN_SHIFT) };
}

/* C's `<winuser.h>` surface `w32_read_socket` (C 5130-6458) reads and this unit does not carry,
   plus the three `termhooks.h` members C writes inside platform `#if`s.  Every name stands at the
   use site (P-88 (1)); the SDK records are UNVERIFIED, and the three event kinds follow the
   `src/dbusbind.rs` 572 `DBUS_EVENT` shape - a mirror gap with the configuration's own value
   (owed, see the ledger).  */
/// C: #define WM_KEYDOWN 0x0100 / #define WM_SYSKEYDOWN 0x0104
/// C: #define WM_CHAR 0x0102 / #define WM_SYSCHAR 0x0106
/// C: #define WM_INPUTLANGCHANGE 0x0051 / #define WM_SETTINGCHANGE 0x001A
/// C: #define WM_MOUSEMOVE 0x0200 / #define WM_APPCOMMAND 0x0319
/// C: #define WM_HSCROLL 0x0114 / #define WM_VSCROLL 0x0115
/// C: #define WM_WINDOWPOSCHANGED 0x0047 / #define WM_ACTIVATE 0x0006
/// C: #define WM_ACTIVATEAPP 0x001C / #define WM_MOVE 0x0003
/// C: #define WM_SHOWWINDOW 0x0018 / #define WM_SIZE 0x0005 / #define WM_CLOSE 0x0010
/// C: #define WM_ENDSESSION 0x0016 / #define WM_INITMENU 0x0116 / #define WM_COMMAND 0x0111
/// C: #define WM_DISPLAYCHANGE 0x007E / #define WM_TOUCH 0x0240
/// C: #define SPI_SETWHEELSCROLLLINES 0x0073
/// C: #define SIZE_RESTORED 0 / #define SIZE_MINIMIZED 1 / #define SIZE_MAXIMIZED 2
/// C: WINUSERAPI HDC WINAPI GetWindowDC (HWND hWnd);
/// C: WINUSERAPI int WINAPI GetClipBox (HDC hdc, LPRECT lprc);
/// C: WINUSERAPI int WINAPI MultiByteToWideChar (UINT CodePage, DWORD dwFlags,
/// C:   LPCCH lpMultiByteStr, int cbMultiByte, LPWSTR lpWideCharStr, int cchWideChar);
/// C: WINUSERAPI BOOL WINAPI IsDBCSLeadByteEx (UINT CodePage, BYTE TestChar);
/// C: termhooks.h 152-158: `LANGUAGE_CHANGE_EVENT` (inside `#ifdef HAVE_NTGUI`)
/// C: termhooks.h 189-191: `END_SESSION_EVENT` (inside `#ifdef HAVE_NTGUI`)
/// C: termhooks.h 258-271: `MULTIMEDIA_KEY_EVENT` (inside `#ifdef HAVE_NTGUI`)
#[cfg (feature = "have_ntgui")]
pub const WM_KEYDOWN: c_int = 0x0100;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSKEYDOWN: c_int = 0x0104;
#[cfg (feature = "have_ntgui")]
pub const WM_CHAR: c_int = 0x0102;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSCHAR: c_int = 0x0106;
#[cfg (feature = "have_ntgui")]
pub const WM_INPUTLANGCHANGE: c_int = 0x0051;
#[cfg (feature = "have_ntgui")]
pub const WM_SETTINGCHANGE: c_int = 0x001A;
#[cfg (feature = "have_ntgui")]
pub const WM_MOUSEMOVE: c_int = 0x0200;
#[cfg (feature = "have_ntgui")]
pub const WM_HSCROLL: c_int = 0x0114;
#[cfg (feature = "have_ntgui")]
pub const WM_VSCROLL: c_int = 0x0115;
#[cfg (feature = "have_ntgui")]
pub const WM_WINDOWPOSCHANGED: c_int = 0x0047;
#[cfg (feature = "have_ntgui")]
pub const WM_ACTIVATE: c_int = 0x0006;
#[cfg (feature = "have_ntgui")]
pub const WM_ACTIVATEAPP: c_int = 0x001C;
#[cfg (feature = "have_ntgui")]
pub const WM_MOVE: c_int = 0x0003;
#[cfg (feature = "have_ntgui")]
pub const WM_SHOWWINDOW: c_int = 0x0018;
#[cfg (feature = "have_ntgui")]
pub const WM_SIZE: c_int = 0x0005;
#[cfg (feature = "have_ntgui")]
pub const WM_CLOSE: c_int = 0x0010;
#[cfg (feature = "have_ntgui")]
pub const WM_ENDSESSION: c_int = 0x0016;
#[cfg (feature = "have_ntgui")]
pub const WM_INITMENU: c_int = 0x0116;
#[cfg (feature = "have_ntgui")]
pub const WM_COMMAND: c_int = 0x0111;
#[cfg (feature = "have_ntgui")]
pub const WM_DISPLAYCHANGE: c_int = 0x007E;
#[cfg (feature = "have_ntgui")]
pub const WM_TOUCH: c_int = 0x0240;
#[cfg (feature = "have_ntgui")]
pub const SPI_SETWHEELSCROLLLINES: c_int = 0x0073;
#[cfg (feature = "have_ntgui")]
pub const SIZE_RESTORED: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SIZE_MINIMIZED: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const SIZE_MAXIMIZED: c_int = 2;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetWindowDC (hWnd: HWND) -> HDC;
  pub fn GetClipBox (hdc: HDC, lprc: LPRECT) -> c_int;
  pub fn MultiByteToWideChar (CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: *const c_char,
			      cbMultiByte: c_int, lpWideCharStr: *mut c_ushort,
			      cchWideChar: c_int) -> c_int;
  pub fn IsDBCSLeadByteEx (CodePage: UINT, TestChar: BYTE) -> BOOL;
}
/* C's own chain with `HAVE_NTGUI` defined places `LANGUAGE_CHANGE_EVENT` at 8 (`HORIZ_WHEEL_EVENT`
   is 7) and `END_SESSION_EVENT` at 14 (`DELETE_WINDOW_EVENT` is 13), and `MULTIMEDIA_KEY_EVENT` at
   28 (`CONFIG_CHANGED_EVENT` is 27); `include/termhooks.rs` carries the frozen configuration's 37
   members and none of these three, so each carries the value its own configuration gives (the
   `dbusbind.rs` 572 reading of the same gap - a `have_ntgui` build therefore reads them as the
   frozen `SCROLL_BAR_CLICK_EVENT`, `ICONIFY_EVENT` and `SLEEP_EVENT` until the mirror is extended;
   recorded as owed, not claimed).  */
#[cfg (feature = "have_ntgui")]
pub const LANGUAGE_CHANGE_EVENT: event_kind = 8;
#[cfg (feature = "have_ntgui")]
pub const END_SESSION_EVENT: event_kind = 14;
#[cfg (feature = "have_ntgui")]
pub const MULTIMEDIA_KEY_EVENT: event_kind = 28;

/// C's socket-reader entry points this unit calls but no mirror in its include graph declares (the
/// header and line of each prototype is in its own `/// C:` line below): `store_frame_param`,
/// `frame_ancestor_p`, `cancel_mouse_face`, `gui_redo_mouse_highlight`, `gen_help_event`,
/// `expose_frame`, `clear_mouse_face`, `change_frame_size`, `get_frame_param`, `gui_set_fullscreen`
/// and `note_mouse_highlight`, plus `w32_process_dnd_data` (defined at `src/w32fns.rs` 4394).  Each
/// name stands at the use site (P-88 (1)).
/// C: extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);   [lisp.h 5117]
/// C: extern bool frame_ancestor_p (struct frame *, struct frame *);              [frame.h 1911]
/// C: extern void cancel_mouse_face (struct frame *);                       [dispextern.h 3690]
/// C: extern void gui_redo_mouse_highlight (Display_Info *);                [dispextern.h 4007]
/// C: extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object,
/// C:   ptrdiff_t);                                                          [keyboard.h 513]
/// C: extern void expose_frame (struct frame *, int, int, int, int);        [dispextern.h 3675]
/// C: extern bool clear_mouse_face (Mouse_HLInfo *);                        [dispextern.h 3691]
/// C: extern void change_frame_size (struct frame *, int, int, bool, bool, bool);
/// C:                                                                    [dispextern.h 3924]
/// C: extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);          [lisp.h 5120]
/// C: extern void gui_set_fullscreen (struct frame *, Lisp_Object, Lisp_Object);
/// C:                                                                          [frame.h 1844]
/// C: extern void note_mouse_highlight (struct frame *, int, int);          [dispextern.h 3688]
/// C: Lisp_Object w32_process_dnd_data (int, void *);                          [w32fns.c 2541]
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
  pub fn frame_ancestor_p (a: *mut frame, b: *mut frame) -> bool;
  pub fn cancel_mouse_face (f: *mut frame);
  pub fn gui_redo_mouse_highlight (dpyinfo: *mut w32_display_info);
  pub fn gen_help_event (string: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
			 object: Lisp_Object, pos: isize);
  pub fn w32_process_dnd_data (format: c_int, hGlobal: *mut c_void) -> Lisp_Object;
  pub fn expose_frame (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);
  pub fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    pretend: bool, delay: bool, safe: bool);
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  pub fn gui_set_fullscreen (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object);
  pub fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
}

/// C 5130-6458: `static int w32_read_socket (struct terminal *terminal, struct input_event
/// *hold_quit)` - the socket reader: C 5134-5139's five locals, C 5141's `block_input`, C 5144's
/// `drain_message_queue`, C 5147-6399's `while (get_next_msg (&msg, FALSE))` body (the per-event
/// `inev`/`do_help`/`ignore_dirty_back_buffer`, the `EVENT_INIT` at C 5163-5165, the `switch
/// (msg.msg.message)` over C 5167-6353's twenty-five arms with C 6344's `default:`, C 6355-6359's
/// event store, C 6361-6389's help block and C 6395-6398's back-buffer flush), C 6403-6407's
/// autoraise, C 6409-6454's visibility sweep with `FOR_EACH_FRAME` and C 6456-6457's
/// `unblock_input`/`return count`.
/// C: static int
/// C: w32_read_socket (struct terminal *terminal,
/// C: 		 struct input_event *hold_quit)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_read_socket (terminal: *mut terminal, hold_quit: *mut input_event) -> c_int
{
  let mut count: c_int = 0;                                            // C 5134
  let mut check_visibility: c_int = 0;                                 // C 5135
  /* C declares the `W32Msg` uninitialised and fills it with `get_next_msg` (PORTING.md 1.5).  */
  let mut msg: W32Msg = core::mem::zeroed ();                          // C 5136
  let mut f: *mut frame;                                               // C 5137
  let dpyinfo: *mut w32_display_info = &raw mut one_w32_display_info;   // C 5138
  let hlinfo: *mut Mouse_HLInfo = &raw mut (*dpyinfo).mouse_highlight;  // C 5139

  block_input ();                                                      // C 5141

  /* Process any incoming thread messages.  */
  drain_message_queue ();                                              // C 5144

  /* TODO: ghostscript integration. */
  while get_next_msg (&raw mut msg, FALSE) != 0                             // C 5147
    {
      let mut inev: input_event = core::mem::zeroed ();                 // C 5149
      let mut do_help: c_int = 0;                                      // C 5150

      /* WM_WINDOWPOSCHANGED makes the buffer dirty, but there's no
	 reason to flush the back buffer after receiving such an
	 event, and that also causes flicker.  */
      let mut ignore_dirty_back_buffer: bool = false;                  // C 5155

      /* C 5157-5159's commented-out three-line `DebPrint` call, carried as C's own text:

	   /* DebPrint (("w32_read_socket: %s time:%u\n", */
	   /*            w32_name_of_message (msg.msg.message), */
	   /*            msg.msg.time)); */
      */

      f = core::ptr::null_mut ();                                      // C 5161

      EVENT_INIT (&raw mut inev);                                      // C 5163
      input_event_bits! (kind, &raw mut inev, NO_EVENT);               // C 5164
      inev.arg = Qnil ();                                              // C 5165

      /* C's `switch` reaches its end at C 6353 and C's mid-arm `break`s leave it there, so the
	 match below is the labelled block PORTING.md 2.2 asks for.  */
      'dispatch:
      {
      match msg.msg.message as c_int
	{
	  WM_EMACS_PAINT =>                                            // C 5169
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5170

	      if !f.is_null ()                                         // C 5172
		{
		  if msg.rect.right == msg.rect.left             // C 5174
		      || msg.rect.bottom == msg.rect.top         // C 5175
		    {
		      /* We may get paint messages even though the client
			 area is clipped - these are not expose events. */
		      DebPrint! (("clipped frame %p (%s) got WM_PAINT - ignored\n", f,
				  SDATA ((*f).name)));                 // C 5179-5180
		    }
		  else if !FRAME_VISIBLE_P (f)                     // C 5182
		    {
		      let iconified: bool = FRAME_ICONIFIED_P (f);     // C 5184

		      /* Definitely not obscured, so mark as visible.  */
		      SET_FRAME_VISIBLE! (f, 1);                        // C 5187
		      SET_FRAME_ICONIFIED (f, 0);                  // C 5188
		      SET_FRAME_GARBAGED! (f);                          // C 5189
		      if w32_output_bits! (asked_for_visible, FRAME_OUTPUT_DATA! (f)) == 0   // C 5190
			{ DebPrint! (("frame %p (%s) reexposed by WM_PAINT\n", f,
				      SDATA ((*f).name))); }           // C 5191-5192

		      /* WM_PAINT serves as MapNotify as well, so report
			 visibility changes properly.  */
		      if iconified                                      // C 5196
			{
			  input_event_bits! (kind, &raw mut inev, DEICONIFY_EVENT);   // C 5198
			  XSETFRAME! (inev.frame_or_window, f);         // C 5199
			}
		    }
		  else
		    {
		      if w32_disable_double_buffering                   // C 5204
			  || (*((*f).output_data.w32 as *mut w32_output)).paint_buffer.is_null ()   // C 5205
			{
			  /* Erase background again for safety.  But don't do
			     that if the frame's 'garbaged' flag is set, since
			     in that case expose_frame will do nothing, and if
			     the various redisplay flags happen to be unset,
			     we are left with a blank frame.  */

			  if !FRAME_GARBAGED_P (f) || !FRAME_PARENT_FRAME (f).is_null ()   // C 5213
			    {
			      let hdc: HDC = get_frame_dc (f);          // C 5215

			      w32_clear_rect! (f, hdc, &raw mut msg.rect);   // C 5217
			      release_frame_dc (f, hdc);                // C 5218
			    }

			  expose_frame (f,                              // C 5221
					msg.rect.left,               // C 5222
					msg.rect.top,                // C 5223
					msg.rect.right - msg.rect.left,   // C 5224
					msg.rect.bottom - msg.rect.top);   // C 5225
			  w32_clear_under_internal_border (f);          // C 5226
			}
		      else
			{ w32_show_back_buffer (f); }                   // C 5229
		    }
		}
	      /* C 5232's `break` is the arm's end.  */
	    }

	  WM_INPUTLANGCHANGE =>                                        // C 5234
	    {
	      /* Generate a language change event.  */
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5236

	      /* lParam contains the input language ID in its low 16 bits.
		 Use it to update our record of the keyboard codepage.  */
	      w32_keyboard_codepage = codepage_for_locale ((msg.msg.lParam
							    & 0xffff) as LCID);   // C 5240-5241

	      if !f.is_null ()                                         // C 5243
		{
		  input_event_bits! (kind, &raw mut inev, LANGUAGE_CHANGE_EVENT);   // C 5245
		  XSETFRAME! (inev.frame_or_window, f);                 // C 5246
		  inev.code = w32_keyboard_codepage as c_uint;          // C 5247
		  inev.modifiers = (msg.msg.lParam & 0xffff) as c_uint;   // C 5248
		}
	    }

	  WM_SETTINGCHANGE =>                                          // C 5252
	    {
	      /* We are only interested in changes of the number of lines
		 to scroll when the vertical mouse wheel is moved.  This
		 is only supported on NT.  */
	      if msg.msg.wParam == SPI_SETWHEELSCROLLLINES as usize   // C 5256
		{ w32_get_mouse_wheel_vertical_delta (); }              // C 5257
	    }

	  WM_KEYDOWN | WM_SYSKEYDOWN =>                                // C 5260-5261
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5262

	      if !f.is_null () && !FRAME_ICONIFIED_P (f)               // C 5264
		{
		  if !mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 5266
		      && !EQ ((*f).tab_bar_window, (*hlinfo).mouse_face_window)   // C 5267
		      && !EQ ((*f).tool_bar_window, (*hlinfo).mouse_face_window)   // C 5268
		    {
		      clear_mouse_face (hlinfo);                        // C 5270
		      mouse_face_hidden! (hlinfo, true);                // C 5271
		    }

		  if temp_index == countof! (temp_buffer) as c_int      // C 5274
		    { temp_index = 0; }                                 // C 5275
		  temp_buffer[temp_index as usize] = msg.msg.wParam as c_short;   // C 5276
		  temp_index += 1;
		  input_event_bits! (kind, &raw mut inev, NON_ASCII_KEYSTROKE_EVENT);   // C 5277
		  inev.code = msg.msg.wParam as c_uint;               // C 5278
		  inev.modifiers = msg.dwModifiers;                   // C 5279
		  XSETFRAME! (inev.frame_or_window, f);                  // C 5280
		  inev.timestamp = msg.msg.time as Time;              // C 5281
		}
	    }

	  WM_UNICHAR | WM_SYSCHAR | WM_CHAR =>                          // C 5285-5287
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5288

	      if !f.is_null () && !FRAME_ICONIFIED_P (f)               // C 5290
		{
		  if !mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 5292
		      && !EQ ((*f).tab_bar_window, (*hlinfo).mouse_face_window)   // C 5293
		      && !EQ ((*f).tool_bar_window, (*hlinfo).mouse_face_window)   // C 5294
		    {
		      clear_mouse_face (hlinfo);                        // C 5296
		      mouse_face_hidden! (hlinfo, true);                // C 5297
		    }

		  if temp_index == countof! (temp_buffer) as c_int      // C 5300
		    { temp_index = 0; }                                 // C 5301
		  temp_buffer[temp_index as usize] = msg.msg.wParam as c_short;   // C 5302
		  temp_index += 1;

		  inev.modifiers = msg.dwModifiers;                   // C 5304
		  XSETFRAME! (inev.frame_or_window, f);                  // C 5305
		  inev.timestamp = msg.msg.time as Time;              // C 5306

		  if utf16_high != 0                                    // C 5308
		      && (msg.msg.message != WM_UNICHAR as c_uint    // C 5309
			  || UTF_16_HIGH_SURROGATE_P (msg.msg.wParam as c_int) != 0)   // C 5310
		    {
		      /* Flush the pending high surrogate if the low one
			 isn't coming.  (This should never happen, but I
			 have paranoia about this stuff.)  */
		      let mut inev1: input_event = core::mem::zeroed ();   // C 5315
		      inev1.modifiers = utf16_high_modifiers;            // C 5316
		      inev1.code = utf16_high as c_uint;                 // C 5317
		      inev1.timestamp = inev.timestamp;                  // C 5318
		      inev1.arg = Qnil ();                               // C 5319
		      kbd_buffer_store_event_hold (&raw mut inev1, hold_quit);   // C 5320
		      utf16_high = 0;                                    // C 5321
		      utf16_high_modifiers = 0;                          // C 5322
		    }

		  if msg.msg.message == WM_UNICHAR as c_uint          // C 5325
		    {
		      /* Handle UTF-16 encoded codepoint above the BMP.
			 This is needed to support Emoji input from input
			 panel popped up by "Win+." shortcut.  */
		      if UTF_16_HIGH_SURROGATE_P (msg.msg.wParam as c_int) != 0   // C 5330
			{
			  utf16_high = msg.msg.wParam as c_ushort;     // C 5332
			  utf16_high_modifiers = inev.modifiers;          // C 5333
			  input_event_bits! (kind, &raw mut inev, NO_EVENT);   // C 5334
			  break 'dispatch;                                // C 5335
			}
		      else if UTF_16_LOW_SURROGATE_P (msg.msg.wParam as c_int) != 0   // C 5337
			  && utf16_high != 0                          // C 5338
			{
			  inev.code = surrogates_to_codepoint (msg.msg.wParam as c_int,
					       utf16_high as c_int) as c_uint;   // C 5340-5341
			  utf16_high = 0;                                 // C 5342
			  utf16_high_modifiers = 0;                       // C 5343
			}
		      else
			{ inev.code = msg.msg.wParam as c_uint; }      // C 5346
		    }
		  else if msg.msg.wParam < 256                         // C 5348
		    {
		      let mut code: c_ushort;                             // C 5350
		      let mut dbcs: [c_char; 2] = [0; 2];                 // C 5351
		      dbcs[0] = 0;                                        // C 5352
		      dbcs[1] = msg.msg.wParam as c_char;              // C 5353

		      if dbcs_lead != 0                                   // C 5355
			{
			  dbcs[0] = dbcs_lead;                            // C 5357
			  dbcs_lead = 0;                                  // C 5358
			  if MultiByteToWideChar (w32_keyboard_codepage as UINT, 0,
						  &raw const dbcs[0], 2, &raw mut code, 1) == 0   // C 5359-5360
			    {
			      /* Garbage */
			      DebPrint! (("Invalid DBCS sequence: %d %d\n",
					  dbcs[0], dbcs[1]));             // C 5363-5364
			      input_event_bits! (kind, &raw mut inev, NO_EVENT);   // C 5365
			      break 'dispatch;                            // C 5366
			    }
			}
		      else if IsDBCSLeadByteEx (w32_keyboard_codepage as UINT,
						msg.msg.wParam as BYTE) != 0   // C 5369-5370
			{
			  dbcs_lead = msg.msg.wParam as c_char;         // C 5372
			  input_event_bits! (kind, &raw mut inev, NO_EVENT);   // C 5373
			  break 'dispatch;                                // C 5374
			}
		      else
			{
			  if MultiByteToWideChar (w32_keyboard_codepage as UINT, 0,
						  &raw const dbcs[1], 1, &raw mut code, 1) == 0   // C 5378-5379
			    {
			      /* What to do with garbage? */
			      DebPrint! (("Invalid character: %d\n", dbcs[1]));   // C 5382
			      input_event_bits! (kind, &raw mut inev, NO_EVENT);   // C 5383
			      break 'dispatch;                            // C 5384
			    }
			}
		      inev.code = code as c_uint;                          // C 5387
		    }
		  else
		    {
		      /* Windows shouldn't generate WM_CHAR events above 0xFF
			 in non-Unicode message handlers.  */
		      DebPrint! (("Non-byte WM_CHAR: %d\n", msg.msg.wParam));   // C 5393
		      input_event_bits! (kind, &raw mut inev, NO_EVENT);   // C 5394
		      break 'dispatch;                                    // C 5395
		    }
		  input_event_bits! (kind, &raw mut inev,                 // C 5397
				     if (inev.code as c_int) < 128
				       { ASCII_KEYSTROKE_EVENT }
				     else { MULTIBYTE_CHAR_KEYSTROKE_EVENT });   // C 5397-5398
		}
	    }

	  WM_APPCOMMAND =>                                             // C 5402
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5403

	      if !f.is_null () && !FRAME_ICONIFIED_P (f)                // C 5405
		{
		  if !mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 5407
		      && !EQ ((*f).tab_bar_window, (*hlinfo).mouse_face_window)   // C 5408
		      && !EQ ((*f).tool_bar_window, (*hlinfo).mouse_face_window)   // C 5409
		    {
		      clear_mouse_face (hlinfo);                        // C 5411
		      mouse_face_hidden! (hlinfo, true);                // C 5412
		    }

		  if temp_index == countof! (temp_buffer) as c_int      // C 5415
		    { temp_index = 0; }                                 // C 5416
		  temp_buffer[temp_index as usize] = msg.msg.wParam as c_short;   // C 5417
		  temp_index += 1;
		  input_event_bits! (kind, &raw mut inev, MULTIMEDIA_KEY_EVENT);   // C 5418
		  inev.code = GET_APPCOMMAND_LPARAM! (msg.msg.lParam) as c_uint;   // C 5419
		  inev.modifiers = msg.dwModifiers;                   // C 5420
		  XSETFRAME! (inev.frame_or_window, f);                  // C 5421
		  inev.timestamp = msg.msg.time as Time;              // C 5422
		}
	    }

	  WM_MOUSEMOVE =>                                              // C 5426
	    {
	      /* Ignore non-movement.  */
	      {
		let x: c_int = LOWORD! (msg.msg.lParam) as c_int;     // C 5429
		let y: c_int = HIWORD! (msg.msg.lParam) as c_int;     // C 5430
		if x == last_mousemove_x && y == last_mousemove_y     // C 5431
		  { break 'dispatch; }                               // C 5432
		last_mousemove_x = x;                                // C 5433
		last_mousemove_y = y;                                // C 5434
	      }

	      previous_help_echo_string = help_echo_string;          // C 5437
	      help_echo_string = Qnil ();                            // C 5438

	      if mouse_face_hidden! (hlinfo)                         // C 5440
		{
		  mouse_face_hidden! (hlinfo, false);                  // C 5442
		  clear_mouse_face (hlinfo);                           // C 5443
		}

	      f = mouse_or_wdesc_frame (dpyinfo, msg.msg.hwnd);   // C 5446
	      if !f.is_null ()                                       // C 5447
		{
		  /* Maybe generate SELECT_WINDOW_EVENTs for
		     `mouse-autoselect-window'.  */
		  if !NILP (Vmouse_autoselect_window)                 // C 5451
		      && (f == XFRAME (selected_frame)                // C 5452
			  /* Switch to f from another frame iff
			     focus_follows_mouse is set and f accepts
			     focus.  */
			  || (!NILP (focus_follows_mouse)             // C 5456
			      && FRAME_NO_ACCEPT_FOCUS (f) == 0))     // C 5457
		    {
		      let window: Lisp_Object = window_from_coordinates (f as *mut c_void,
									 LOWORD! (msg.msg.lParam) as c_int,
									 HIWORD! (msg.msg.lParam) as c_int,
									 core::ptr::null_mut (), false, false, false);   // C 5459-5460

		      /* Window will be selected only when it is not
			 selected now and last mouse movement event was
			 not in it.  Minibuffer window will be selected
			 only when it is active.  */
		      if WINDOWP (window)                             // C 5466
			  && !EQ (window, last_mouse_window)          // C 5467
			  && !EQ (window, selected_window)            // C 5468
			{
			  input_event_bits! (kind, &raw mut inev, SELECT_WINDOW_EVENT);   // C 5470
			  inev.frame_or_window = window;              // C 5471
			}

		      /* Remember the last window where we saw the mouse.  */
		      last_mouse_window = window;                     // C 5475
		    }

		  if w32_note_mouse_movement (f, &raw mut msg.msg) == 0   // C 5478
		    { help_echo_string = previous_help_echo_string; }   // C 5479
		}
	      else
		{
		  /* If we move outside the frame, then we're
		     certainly no longer on any text in the frame.  */
		  clear_mouse_face (hlinfo);                          // C 5485
		}

	      /* If the contents of the global variable help_echo_string
		 has changed, generate a HELP_EVENT.  */
	      /* C 5490-5499 is `#if 0` / `#else` / `#endif`; the arm this configuration compiles is
		 C's `#else` (the `xterm.c` one), and C's dead arm is carried as C's own text:

	  #if 0 /* The below is an invalid comparison when CHECK_LISP_OBJECT_TYPE.
		   But it was originally changed to this to fix a bug, so I
		   have not removed it completely in case the bug is still there.  */
		  if (help_echo_string != previous_help_echo_string ||
		      (!NILP (help_echo_string) && !STRINGP (help_echo_string) && f->mouse_moved))
		#else /* This is what xterm.c does.  */
	      */
	      if !NILP (help_echo_string)                             // C 5496
		  || !NILP (previous_help_echo_string)                // C 5497
		{ do_help = 1; }                                      // C 5498
	    }

	  WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDOWN | WM_MBUTTONUP   // C 5502-5505
	  | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_XBUTTONDOWN | WM_XBUTTONUP =>   // C 5506-5509
	    {
	      /* If we decide we want to generate an event to be seen
		 by the rest of Emacs, we put it here.  */
	      let mut tab_bar_arg: Lisp_Object = Qnil ();             // C 5513
	      let mut tab_bar_p: bool = false;                        // C 5514
	      let mut tool_bar_p: bool = false;                       // C 5515
	      let mut button: c_int = 0;                              // C 5516
	      let mut up: c_int = 0;                                  // C 5517

	      f = mouse_or_wdesc_frame (dpyinfo, msg.msg.hwnd);    // C 5519
	      if !f.is_null ()                                        // C 5520
		{
		  w32_construct_mouse_click (&raw mut inev, &raw mut msg, f);   // C 5522

		  /* Is this in the tab-bar?  */
		  if WINDOWP ((*f).tab_bar_window)                    // C 5525
		      && WINDOW_TOTAL_LINES (XWINDOW ((*f).tab_bar_window)) != 0   // C 5526
		    {
		      let window: Lisp_Object;                        // C 5528
		      let x: c_int = XFIXNAT (inev.x) as c_int;       // C 5529
		      let y: c_int = XFIXNAT (inev.y) as c_int;       // C 5530

		      window = window_from_coordinates (f as *mut c_void, x, y,
							core::ptr::null_mut (), true, true, true);   // C 5532

		      if EQ (window, (*f).tab_bar_window)             // C 5534
			{
			  tab_bar_arg = w32_handle_tab_bar_click (f, &raw mut inev);   // C 5536
			  tab_bar_p = true;                           // C 5537
			}
		    }

		  if (tab_bar_p && NILP (tab_bar_arg))                // C 5541
		      || (!(*dpyinfo).w32_focus_frame.is_null ()      // C 5542
			  && f != (*dpyinfo).w32_focus_frame          // C 5543
			  /* This does not help when the click happens in
			     a grand-parent frame.  */
			  && !frame_ancestor_p (f, (*dpyinfo).w32_focus_frame))   // C 5546
		    { input_event_bits! (kind, &raw mut inev, NO_EVENT); }   // C 5547

		  if !NILP (tab_bar_arg)                              // C 5549
		    { inev.arg = tab_bar_arg; }                       // C 5550

		  /* Is this in the tool-bar?  */
		  if WINDOWP ((*f).tool_bar_window)                   // C 5553
		      && WINDOW_TOTAL_LINES (XWINDOW ((*f).tool_bar_window)) != 0   // C 5554
		    {
		      let window: Lisp_Object;                        // C 5556
		      let x: c_int = XFIXNAT (inev.x) as c_int;       // C 5557
		      let y: c_int = XFIXNAT (inev.y) as c_int;       // C 5558

		      window = window_from_coordinates (f as *mut c_void, x, y,
							core::ptr::null_mut (), true, true, true);   // C 5560

		      if EQ (window, (*f).tool_bar_window)            // C 5562
			  /* Make sure the tool bar was previously
			     pressed, otherwise an event that started
			     outside of the tool bar will not be handled
			     correctly when the mouse button is
			     released.  For example, start dragging to
			     select some buffer text, drag the mouse to
			     the tool bar, and release the mouse button
			     -- this should not consider the release
			     event as a tool-bar click.  */
			  && ((inev.modifiers & down_modifier as c_uint) != 0    // C 5572
			      || (*f).last_tool_bar_item != -1)       // C 5573
			{
			  w32_handle_tool_bar_click (f, &raw mut inev);   // C 5575
			  tool_bar_p = true;                          // C 5576
			}
		    }

		  if tool_bar_p                                        // C 5580
		      || (!(*dpyinfo).w32_focus_frame.is_null ()      // C 5581
			  && f != (*dpyinfo).w32_focus_frame          // C 5582
			  /* This does not help when the click happens in
			     a grand-parent frame.  */
			  && !frame_ancestor_p (f, (*dpyinfo).w32_focus_frame))   // C 5585
		    { input_event_bits! (kind, &raw mut inev, NO_EVENT); }   // C 5586
		}

	      parse_button (msg.msg.message as c_int, HIWORD! (msg.msg.wParam) as c_int,   // C 5589
			    &raw mut button, &raw mut up);            // C 5590

	      if up != 0                                              // C 5592
		{
		  (*dpyinfo).grabbed &= ! (1 << button);              // C 5594
		}
	      else
		{
		  (*dpyinfo).grabbed |= (1 << button);                // C 5598
		  (*dpyinfo).last_mouse_frame = f;                    // C 5599
		  /* Ignore any mouse motion that happened
		     before this event; any subsequent mouse-movement
		     Emacs events should reflect only motion after
		     the ButtonPress.  */
		  if !f.is_null ()                                    // C 5604
		    {
		      frame_bits! (mouse_moved, f, false);            // C 5606
		      if !tab_bar_p                                  // C 5607
			{ (*f).last_tab_bar_item = -1; }             // C 5608
		      if !tool_bar_p                                 // C 5609
			{ (*f).last_tool_bar_item = -1; }            // C 5610
		    }
		}
	    }

	  WM_MOUSEWHEEL | WM_MOUSEHWHEEL =>                           // C 5616-5617
	    {
	      f = mouse_or_wdesc_frame (dpyinfo, msg.msg.hwnd);    // C 5619
	      if !f.is_null ()                                        // C 5620
		{
		  if (*dpyinfo).w32_focus_frame.is_null ()            // C 5622
		      || f == (*dpyinfo).w32_focus_frame              // C 5623
		    /* Emit an Emacs wheel-up/down event.  */
		    {
		      w32_construct_mouse_wheel (&raw mut inev, &raw mut msg, f);   // C 5626

		      /* Ignore any mouse motion that happened before this
			 event; any subsequent mouse-movement Emacs events
			 should reflect only motion after the ButtonPress.  */
		      frame_bits! (mouse_moved, f, false);             // C 5631
		      (*f).last_tab_bar_item = -1;                      // C 5632
		      (*f).last_tool_bar_item = -1;                     // C 5633
		      (*dpyinfo).last_mouse_frame = f;                  // C 5634
		    }
		  else if FRAME_NO_ACCEPT_FOCUS (f) != 0               // C 5636
		      && !gui_mouse_grabbed (dpyinfo)                   // C 5637
		    {
		      let frame1: Lisp_Object = get_frame_param (f, Qmouse_wheel_frame ());   // C 5639
		      let f1: *mut frame = if FRAMEP (frame1) { XFRAME (frame1) } else { core::ptr::null_mut () };   // C 5640

		      if !f1.is_null () && FRAME_LIVE_P (f1) && FRAME_W32_P (f1)   // C 5642
			{
			  w32_construct_mouse_wheel (&raw mut inev, &raw mut msg, f1);   // C 5644
			  frame_bits! (mouse_moved, f1, false);         // C 5645
			  (*f1).last_tab_bar_item = -1;                 // C 5646
			  (*f1).last_tool_bar_item = -1;                // C 5647
			  (*dpyinfo).last_mouse_frame = f1;             // C 5648
			}
		      else
			{ (*dpyinfo).last_mouse_frame = f; }            // C 5651
		    }
		  else
		    { (*dpyinfo).last_mouse_frame = f; }                // C 5654
		}
	      else
		{ (*dpyinfo).last_mouse_frame = f; }                    // C 5657
	    }

	  WM_EMACS_DROP =>                                             // C 5661
	    {
	      let format: c_int = msg.msg.wParam as c_int;           // C 5663
	      let drop_object: Lisp_Object
		= w32_process_dnd_data (format, msg.msg.lParam as *mut c_void);   // C 5664-5665

	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5667
	      if f.is_null () || NILP (drop_object)                   // C 5668
		{ break 'dispatch; }                                   // C 5669

	      XSETFRAME! (inev.frame_or_window, f);                   // C 5671
	      input_event_bits! (kind, &raw mut inev, DRAG_N_DROP_EVENT);   // C 5672
	      inev.code = 0;                                          // C 5673
	      inev.timestamp = msg.msg.time as Time;               // C 5674
	      inev.modifiers = msg.dwModifiers;                    // C 5675
	      ScreenToClient (msg.msg.hwnd, &raw mut msg.msg.pt);   // C 5676
	      XSETINT! (inev.x, msg.msg.pt.x as EMACS_INT);         // C 5677
	      XSETINT! (inev.y, msg.msg.pt.y as EMACS_INT);         // C 5678
	      inev.arg = drop_object;                                 // C 5679
	    }

	  WM_EMACS_DRAGOVER =>                                         // C 5683
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5685
	      if f.is_null ()                                          // C 5686
		{ break 'dispatch; }                                   // C 5687
	      XSETFRAME! (inev.frame_or_window, f);                    // C 5688
	      input_event_bits! (kind, &raw mut inev, DRAG_N_DROP_EVENT);   // C 5689
	      inev.code = 0;                                           // C 5690
	      inev.timestamp = msg.msg.time as Time;                // C 5691
	      inev.modifiers = msg.dwModifiers;                     // C 5692
	      ScreenToClient (msg.msg.hwnd, &raw mut msg.msg.pt);   // C 5693
	      XSETINT! (inev.x, msg.msg.pt.x as EMACS_INT);         // C 5694
	      XSETINT! (inev.y, msg.msg.pt.y as EMACS_INT);         // C 5695
	      /* This is a drag movement.  */
	      inev.arg = Qnil ();                                     // C 5697
	      /* C 5698's `break` is the arm's end.  */
	    }

	  WM_HSCROLL =>                                                // C 5701
	    {
	      let bar: *mut scroll_bar
		= w32_window_to_scroll_bar (msg.msg.lParam as HWND, 1);   // C 5703-5704

	      if !bar.is_null ()                                       // C 5706
		{ w32_horizontal_scroll_bar_handle_click (bar, &raw mut msg, &raw mut inev); }   // C 5707
	    }

	  WM_VSCROLL =>                                                // C 5711
	    {
	      let bar: *mut scroll_bar
		= w32_window_to_scroll_bar (msg.msg.lParam as HWND, 0);   // C 5713-5714

	      if !bar.is_null ()                                       // C 5716
		{ w32_scroll_bar_handle_click (bar, &raw mut msg, &raw mut inev); }   // C 5717
	    }

	  WM_WINDOWPOSCHANGED =>                                       // C 5721
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5722
	      ignore_dirty_back_buffer = true;                          // C 5723

	      if !f.is_null ()                                          // C 5725
		{
		  /* C declares the `RECT` uninitialised and fills it with `GetClientRect`
		     (PORTING.md 1.5).  */
		  let mut rect: RECT = core::mem::zeroed ();                // C 5727
		  /* C 5728's `int /* rows, columns, */ width, height;` - the comment is C's own.  */
		  let mut width: c_int;
		  let mut height: c_int;

		  if GetClientRect (msg.msg.hwnd, &raw mut rect) != 0   // C 5730
		      /* GetClientRect evidently returns (0, 0, 0, 0) if
			 called on a minimized frame.  Such "dimensions"
			 aren't useful anyway.  */
		      && !(rect.bottom == 0                            // C 5734
			   && rect.top == 0                             // C 5735
			   && rect.left == 0                            // C 5736
			   && rect.right == 0)                          // C 5737
		    {
		      height = rect.bottom - rect.top;                   // C 5739
		      width = rect.right - rect.left;                    // C 5740
		      if width != FRAME_PIXEL_WIDTH (f)                  // C 5741
			  || height != FRAME_PIXEL_HEIGHT (f)            // C 5742
			{
			  change_frame_size (f, width, height, false, true, false);   // C 5744-5745
			  SET_FRAME_GARBAGED! (f);                        // C 5746
			  cancel_mouse_face (f);                          // C 5747
			  (*f).win_gravity = NorthWestGravity;            // C 5748
			}
		    }
		}

	      check_visibility = 1;                                     // C 5753
	    }

	  WM_ACTIVATE | WM_ACTIVATEAPP =>                              // C 5756-5757
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5758
	      if !f.is_null ()                                          // C 5759
		{
		  /* Run the full-screen hook function also when we are
		     being activated, to actually install the required
		     size in effect, if the WAIT flag is set.  This
		     because when the hook is run from gui_set_fullscreen,
		     the frame might not yet be visible, if that call is a
		     result of make-frame, and in that case the hook just
		     sets the WAIT flag.  */
		  if ((msg.msg.message == WM_WINDOWPOSCHANGED as c_uint   // C 5768
		       || msg.msg.wParam != 0)                       // C 5768
		      && (want_fullscreen! (f) & FULLSCREEN_WAIT) != 0)   // C 5769
		    {
		      /* Must set visibility right here since otherwise
			 w32fullscreen_hook returns immediately.  */
		      SET_FRAME_VISIBLE! (f, 1);                        // C 5773
		      w32fullscreen_hook (f);                           // C 5774
		    }
		}

	      check_visibility = 1;                                     // C 5778
	    }

	  WM_MOVE =>                                                    // C 5781
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5782

	      if !f.is_null () && FRAME_VISIBLE_P (f) && !FRAME_ICONIFIED_P (f)   // C 5784
		  && !FRAME_TOOLTIP_P (f)                               // C 5785
		{
		  w32_real_positions (f, &raw mut (*f).left_pos, &raw mut (*f).top_pos);   // C 5787
		  input_event_bits! (kind, &raw mut inev, MOVE_FRAME_EVENT);   // C 5788
		  XSETFRAME! (inev.frame_or_window, f);                 // C 5789
		}

	      check_visibility = 1;                                     // C 5792
	    }

	  WM_SHOWWINDOW =>                                              // C 5795
	    {
	      /* wParam non-zero means Window is about to be shown, 0 means
		 about to be hidden.  */
	      /* Redo the mouse-highlight after the tooltip has gone.  */
	      if msg.msg.wParam == 0 && msg.msg.hwnd == tip_window   // C 5799
		{
		  tip_window = core::ptr::null_mut ();                    // C 5801
		  gui_redo_mouse_highlight (dpyinfo);                     // C 5802
		}

	      /* If window has been obscured or exposed by another window
		 being maximized or minimized/restored, then recheck
		 visibility of all frames.  Direct changes to our own
		 windows get handled by WM_SIZE.  */
	      /* C 5809-5817 is `#if 0`, carried as C's own text:

	  #if 0
		  if (msg.msg.lParam != 0)
		    check_visibility = 1;
		  else
		    {
		      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);
		      f->async_visible = msg.msg.wParam;
		    }
	  #endif
	      */

	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5819
	      if !f.is_null ()                                          // C 5820
		{ w32_clear_under_internal_border (f); }                // C 5821

	      check_visibility = 1;                                     // C 5823
	    }

	  WM_SIZE =>                                                   // C 5826
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5827

	      /* Inform lisp of whether frame has been iconified etc. */
	      if !f.is_null ()                                          // C 5830
		{
		  match msg.msg.wParam as c_int                      // C 5832
		    {
		      SIZE_MINIMIZED =>                                 // C 5834
			{
			  SET_FRAME_VISIBLE! (f, 0);                    // C 5835
			  SET_FRAME_ICONIFIED (f, 1);                   // C 5836

			  input_event_bits! (kind, &raw mut inev, ICONIFY_EVENT);   // C 5838
			  XSETFRAME! (inev.frame_or_window, f);         // C 5839
			}

		      SIZE_MAXIMIZED =>                                 // C 5842
			{
			  let iconified: bool = FRAME_ICONIFIED_P (f);   // C 5844
			  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());   // C 5845

			  SET_FRAME_VISIBLE! (f, 1);                    // C 5847
			  SET_FRAME_ICONIFIED (f, 0);                   // C 5848

			  /* wait_reading_process_output will notice this
			     and update the frame's display structures.  */
			  SET_FRAME_GARBAGED! (f);                      // C 5852

			  if iconified                                  // C 5854
			    {
			      let mut x: c_int;
			      let mut y: c_int;

			      /* Reset top and left positions of the Window
				 here since Windows sends a WM_MOVE message
				 BEFORE telling us the Window is minimized
				 when the Window is iconified, with 3000,3000
				 as the co-ords. */
			      w32_real_positions (f, &raw mut x, &raw mut y);   // C 5863
			      (*f).left_pos = x;                        // C 5864
			      (*f).top_pos = y;                         // C 5865

			      input_event_bits! (kind, &raw mut inev, DEICONIFY_EVENT);   // C 5867
			      XSETFRAME! (inev.frame_or_window, f);     // C 5868
			    }

			  /* Windows can send us a SIZE_MAXIMIZED message even
			     when fullscreen is fullboth.  The following is a
			     simple hack to check that based on the fact that
			     only a maximized fullscreen frame should have top
			     or left outside the screen.  */
			  if EQ (fullscreen, Qfullwidth ()) || EQ (fullscreen, Qfullheight ())   // C 5876
			      || NILP (fullscreen)                      // C 5877
			    {
			      let mut x: c_int;
			      let mut y: c_int;

			      w32_real_positions (f, &raw mut x, &raw mut y);   // C 5881
			      if x < 0 || y < 0                         // C 5882
				{ store_frame_param (f, Qfullscreen (), Qmaximized ()); }   // C 5883
			    }
			}

		      SIZE_RESTORED =>                                  // C 5889
			{
			  let iconified: bool = FRAME_ICONIFIED_P (f);   // C 5891

			  /* The following was made unconditional in a
			     pathetic attempt to fix bug#16967 in revision
			     116716 but, considered counterproductive was made
			     conditional again in revision 116727.  martin */
			  if iconified                                  // C 5897
			    { SET_FRAME_VISIBLE! (f, 1); }               // C 5898
			  SET_FRAME_ICONIFIED (f, 0);                   // C 5899

			  /* wait_reading_process_output will notice this
			     and update the frame's display structures.  */
			  SET_FRAME_GARBAGED! (f);                      // C 5903

			  if iconified                                  // C 5905
			    {
			      /* Reset top and left positions of the Window
				 here since Windows sends a WM_MOVE message
				 BEFORE telling us the Window is minimized
				 when the Window is iconified, with 3000,3000
				 as the co-ords.  */
			      w32_real_positions (f, &raw mut (*f).left_pos, &raw mut (*f).top_pos);   // C 5912

			      input_event_bits! (kind, &raw mut inev, DEICONIFY_EVENT);   // C 5914
			      XSETFRAME! (inev.frame_or_window, f);     // C 5915
			    }
			}

		      /* C's `switch` has no `default` (PORTING.md 2.2).  */
		      _ => { }
		    }

		  if EQ (get_frame_param (f, Qfullscreen ()), Qmaximized ())   // C 5919
		    { store_frame_param (f, Qfullscreen (), Qnil ()); }   // C 5920
		}

	      if !f.is_null () && !FRAME_ICONIFIED_P (f)                // C 5926
		  && msg.msg.wParam != SIZE_MINIMIZED as usize       // C 5926
		{
		  /* C declares the `RECT` uninitialised and fills it with `GetClientRect`
		     (PORTING.md 1.5).  */
		  let mut rect: RECT = core::mem::zeroed ();                // C 5928
		  /* C 5929's `int /* rows, columns, */ width, height;` - the comment is C's own.  */
		  let mut width: c_int;
		  let mut height: c_int;

		  if GetClientRect (msg.msg.hwnd, &raw mut rect) != 0   // C 5931
		      /* GetClientRect evidently returns (0, 0, 0, 0) if
			 called on a minimized frame.  Such "dimensions"
			 aren't useful anyway.  */
		      && !(rect.bottom == 0                            // C 5935
			   && rect.top == 0                             // C 5936
			   && rect.left == 0                            // C 5937
			   && rect.right == 0)                          // C 5938
		    {
		      height = rect.bottom - rect.top;                   // C 5940
		      width = rect.right - rect.left;                    // C 5941

		      if width != FRAME_PIXEL_WIDTH (f)                  // C 5943
			  || height != FRAME_PIXEL_HEIGHT (f)            // C 5944
			{
			  w32_release_paint_buffer (f);                  // C 5946

			  change_frame_size (f, width, height, false, true, false);   // C 5948-5949
			  SET_FRAME_GARBAGED! (f);                       // C 5950
			  cancel_mouse_face (f);                         // C 5951
			  (*f).win_gravity = NorthWestGravity;           // C 5952
			}
		    }
		}

	      check_visibility = 1;                                     // C 5957
	    }

	  WM_MOUSELEAVE =>                                             // C 5960
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5961
	      if !f.is_null ()                                          // C 5962
		{
		  if f == (*hlinfo).mouse_face_mouse_frame as *mut frame              // C 5964
		    {
		      /* If we move outside the frame, then we're
			 certainly no longer on any text in the frame.  */
		      clear_mouse_face (hlinfo);                        // C 5968
		      (*hlinfo).mouse_face_mouse_frame = core::ptr::null_mut ();   // C 5969
		    }

		  /* Generate a nil HELP_EVENT to cancel a help-echo.
		     Do it only if there's something to cancel.
		     Otherwise, the startup message is cleared when
		     the mouse leaves the frame.  */
		  if any_help_event_p != 0                              // C 5976
		    { do_help = -1; }                                   // C 5977
		}
	    }

	  /* The mirror declares `WM_SETFOCUS` as `UINT` (C 3343-3344's comparisons, slice 10's
	     block), so the arm is a guard over C's own token with the scrutinee's cast.  */
	  x if x == WM_SETFOCUS as c_int =>                            // C 5982
	    {
	      w32_detect_focus_change (dpyinfo, &raw mut msg, &raw mut inev);   // C 5983

	      (*dpyinfo).grabbed = 0;                                   // C 5985
	      check_visibility = 1;                                     // C 5986
	    }

	  x if x == WM_KILLFOCUS as c_int =>                           // C 5989
	    {
	      w32_detect_focus_change (dpyinfo, &raw mut msg, &raw mut inev);   // C 5990
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 5991

	      if !f.is_null ()                                          // C 5993
		{
		  if f == (*hlinfo).mouse_face_mouse_frame as *mut frame              // C 5995
		    {
		      /* If we move outside the frame, then we're
			 certainly no longer on any text in the frame.  */
		      clear_mouse_face (hlinfo);                        // C 5999
		      (*hlinfo).mouse_face_mouse_frame = core::ptr::null_mut ();   // C 6000
		    }

		  /* Generate a nil HELP_EVENT to cancel a help-echo.
		     Do it only if there's something to cancel.
		     Otherwise, the startup message is cleared when
		     the mouse leaves the frame.  */
		  if any_help_event_p != 0                              // C 6007
		    { do_help = -1; }                                   // C 6008
		}

	      (*dpyinfo).grabbed = 0;                                   // C 6011
	      check_visibility = 1;                                     // C 6012
	    }

	  WM_CLOSE =>                                                  // C 6015
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6016

	      if !f.is_null ()                                          // C 6018
		{
		  input_event_bits! (kind, &raw mut inev, DELETE_WINDOW_EVENT);   // C 6020
		  XSETFRAME! (inev.frame_or_window, f);                 // C 6021
		}
	    }

	  WM_ENDSESSION =>                                             // C 6025
	    {
	      input_event_bits! (kind, &raw mut inev, END_SESSION_EVENT);   // C 6026
	    }

	  WM_INITMENU =>                                               // C 6029
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6030

	      if !f.is_null ()                                          // C 6032
		{
		  input_event_bits! (kind, &raw mut inev, MENU_BAR_ACTIVATE_EVENT);   // C 6034
		  XSETFRAME! (inev.frame_or_window, f);                 // C 6035
		}
	    }

	  WM_COMMAND =>                                                // C 6039
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6040

	      if !f.is_null ()                                          // C 6042
		{
		  menubar_selection_callback (f, msg.msg.wParam as *mut c_void);   // C 6044
		}

	      check_visibility = 1;                                     // C 6047
	    }

	  WM_DISPLAYCHANGE =>                                          // C 6050
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6051

	      if !f.is_null ()                                          // C 6053
		{
		  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());   // C 6055

		  (*dpyinfo).n_cbits = msg.msg.wParam as c_int;         // C 6057
		  /* The new display could have a different resolution, in
		     which case we must reconsider what fullscreen means.
		     The following code is untested yet.  */
		  if !NILP (fullscreen)                                 // C 6061
		    {
		      gui_set_fullscreen (f, fullscreen, fullscreen);   // C 6063
		      w32fullscreen_hook (f);                           // C 6064
		    }

		  DebPrint! (("display change: %d %d\n",                 // C 6067
			      LOWORD! (msg.msg.lParam) as c_short,    // C 6068
			      HIWORD! (msg.msg.lParam) as c_short));   // C 6069
		}

	      /* According to the MS documentation, this message is sent
		 to each window whenever a monitor is added, removed, or
		 has its resolution change.  Detect duplicate events when
		 there are multiple frames by ensuring only one event is
		 put in the keyboard buffer at any given time.  */
	      {
		let ev: *mut buffered_input_event;                       // C 6078

		ev = if kbd_store_ptr == (&raw mut kbd_buffer as *mut buffered_input_event)
		  { (&raw mut kbd_buffer as *mut buffered_input_event)
		      .add (KBD_BUFFER_SIZE as usize - 1) }             // C 6080-6081
		else
		  { kbd_store_ptr.offset (-1) };                        // C 6082

		if kbd_store_ptr != kbd_fetch_ptr                       // C 6084
		    && input_event_bits! (kind, &raw mut (*ev).ie) == MONITORS_CHANGED_EVENT   // C 6085
		    && XTERMINAL ((*ev).ie.arg) == (*dpyinfo).terminal   // C 6086
		  /* Don't store a MONITORS_CHANGED_EVENT if there is
		     already an undelivered event on the queue.  */
		  { break 'dispatch; }                                  // C 6089

		input_event_bits! (kind, &raw mut inev, MONITORS_CHANGED_EVENT);   // C 6091
		XSETTERMINAL! (inev.arg, (*dpyinfo).terminal);           // C 6092
	      }

	      check_visibility = 1;                                     // C 6095
	    }

	  WM_EMACS_SET_TOOLKIT_THEME =>                                // C 6098
	    {
	      input_event_bits! (kind, &raw mut inev, TOOLKIT_THEME_CHANGED_EVENT);   // C 6100
	      inev.arg = if msg.msg.wParam != 0 { Qdark () } else { Qlight () };   // C 6101
	    }

	  /* C 6105-6111 is `#if HAVE_W32NOTIFY` - the file gate's second arm (`have_w32notify`,
	     the `docs/ports/w32term.md` table's `all (have_ntgui, have_w32notify)` row), so the arm
	     carries the feature.  */
	  #[cfg (feature = "have_w32notify")]
	  WM_EMACS_FILENOTIFY =>                                       // C 6106
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6107
	      if !f.is_null ()                                          // C 6108
		{ w32_queue_notifications (&raw mut inev, &raw mut msg, f, &raw mut count); }   // C 6109
	    }

	  /* C 6113-6125 carries a dead `#if 0` and an `#ifdef WM_TOUCHMOVE` / `#else` pair whose two
	     labels are the same number (`src/w32term.c` 141 defines `WM_TOUCHMOVE` 576 under
	     `_WIN32_WINNT < 0x0601`, and `<winuser.h>`'s `WM_TOUCH` is 0x0240 = 576), so the live
	     label is the file's own name and C's two dead arms are carried as C's own text:

	  #if 0
		  /* These messages existed in prerelease versions of Windows 7,
		     yet, though superseded by just WM_TOUCHMOVE (renamed
		     WM_TOUCH) in the release, are still defined by MinGW's
		     winuser.h.  */
		case WM_TOUCHDOWN:
		case WM_TOUCHUP:
	  #endif /* 0 */
	  #ifdef WM_TOUCHMOVE
		case WM_TOUCHMOVE:
	  #else /* not WM_TOUCHMOVE */
		case WM_TOUCH:
	  #endif /* not WM_TOUCHMOVE */
	      */
	  WM_TOUCHMOVE =>                                              // C 6122
	    {
	      f = w32_window_to_frame (dpyinfo, msg.msg.hwnd);       // C 6126

	      /* WM_TOUCH should never be received when touch input
		 functions are unavailable.  */
	      if pfnGetTouchInputInfo.is_none ()                        // C 6130
		{ break 'dispatch; }                                   // C 6131

	      if !f.is_null ()                                          // C 6133
		{
		  let points: *mut TOUCHINPUT;                              // C 6135
		  let mut i: c_int;
		  /* C 6136's `int x UNINIT` is `= {0,}` in conf_post.h 369, so the port zeroes it.  */
		  let mut x: c_int = 0;
		  let mut px: c_int;
		  let mut py: c_int;
		  let mut pt: POINT;                                       // C 6137

		  /* C's bare `alloca` is the port's stack pool (PORTING.md P-05).  */
		  points = sa_stack_alloc (core::mem::size_of::<TOUCHINPUT> ()
					   * LOWORD! (msg.msg.wParam) as usize) as *mut TOUCHINPUT;   // C 6139
		  if (pfnGetTouchInputInfo.unwrap ()) (msg.msg.lParam as HANDLE,   // C 6140-6141
						       LOWORD! (msg.msg.wParam) as UINT,
						       points,
						       core::mem::size_of::<TOUCHINPUT> () as c_int) != 0   // C 6142
		    {
		      let mut movement_p: bool = false;                    // C 6144
		      let base: EMACS_INT = (*((*f).output_data.w32 as *mut w32_output)).touch_base;   // C 6145

		      /* Iterate over the list of touch points in the
			 structure, and for each, enter or remove
			 information into and from F->touch_ids, and
			 generate events correspondingly.  */
		      /* C's `for` performs its update on `continue` too, so the port's `while` writes
		 C's own `++i` at both sites (PORTING.md 2.2's `fringe.rs` shape), and C's two
		 `goto touch_located` are the forward jumps into the labelled block below.  */
		      i = 0;
		      /* Rust forbids an unlabelled `continue` whose loop is outside the labelled block
			 (`E0695`, measured), so C's own `while` head carries the label C's `for` does not
			 need - the label C's own two `goto touch_located` already imply.  */
		      'touch_points: while i < LOWORD! (msg.msg.wParam) as c_int   // C 6151
			{
			  'touch_located: {
			    if (*points.offset (i as isize)).dwID == 0     // C 6153
			      { i += 1; continue 'touch_points; }          // C 6154

			    /* Skip to `touch_located' if the point is
			       reserved for the tool bar, and hasn't just been
			       placed.  */
			    if (*points.offset (i as isize)).dwID          // C 6159
				== (*((*f).output_data.w32 as *mut w32_output)).tool_bar_dwID   // C 6160
			      {
				if ((*points.offset (i as isize)).dwFlags & TOUCHEVENTF_UP as DWORD) != 0   // C 6162
				  { break 'touch_located; }                // C 6163

				/* Other like events should be simply
				   discarded.  */
				i += 1; continue 'touch_points;            // C 6167
			      }

			    /* Search for a slot in touch_ids that is either
			       empty or matches dwID.  */
			    x = 0;
			    while x < MAX_TOUCH_POINTS as c_int                     // C 6172
			      {
				if (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize]
				    == (*points.offset (i as isize)).dwID as c_int   // C 6174-6175
				  { break; }                               // C 6176
				x += 1;
			      }

			    if x < MAX_TOUCH_POINTS as c_int                        // C 6179
			      { break 'touch_located; }                    // C 6180

			    x = 0;
			    while x < MAX_TOUCH_POINTS as c_int                     // C 6182
			      {
				if (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize] == -1   // C 6184
				  { break; }                               // C 6185
				x += 1;
			      }

			    if x == MAX_TOUCH_POINTS as c_int                       // C 6188
			      { i += 1; continue 'touch_points; }          // C 6189
			  }

			  /* C 6191's `touch_located:` station.  */
			  /* X and Y are fractional values.  */
			  pt.x = (*points.offset (i as isize)).x / 100;    // C 6193
			  pt.y = (*points.offset (i as isize)).y / 100;    // C 6194

			  /* Convert them from screen values to client
			     values.  */
			  ScreenToClient (msg.msg.hwnd, &raw mut pt);   // C 6198
			  px = pt.x;                                       // C 6199
			  py = pt.y;                                       // C 6200

			  if ((*points.offset (i as isize)).dwFlags & TOUCHEVENTF_UP as DWORD) != 0   // C 6202
			    {
			      if (*points.offset (i as isize)).dwID
				  == (*((*f).output_data.w32 as *mut w32_output)).tool_bar_dwID   // C 6204-6205
				{
				  (*((*f).output_data.w32 as *mut w32_output)).tool_bar_dwID = -1i32 as DWORD;   // C 6207
				  if (*f).last_tool_bar_item != -1         // C 6208
				    { handle_tool_bar_click (f, px, py, false, 0); }   // C 6209

				  /* Cancel any outstanding mouse highlight.  */
				  note_mouse_highlight (f, -1, -1);        // C 6212
				  i += 1; continue;                        // C 6213
				}

			      /* Clear the entry in touch_ids and report the
				 change.  Unless, of course, the entry be
				 empty.  */
			      if (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize] == -1   // C 6219
				{ i += 1; continue; }                      // C 6220
			      (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize] = -1;   // C 6221

			      input_event_bits! (kind, &raw mut inev, TOUCHSCREEN_END_EVENT);   // C 6223
			      inev.timestamp = msg.msg.time as Time;    // C 6224
			      XSETFRAME! (inev.frame_or_window, f);        // C 6225
			      XSETINT! (inev.x, px as EMACS_INT);          // C 6226
			      XSETINT! (inev.y, py as EMACS_INT);          // C 6227
			      XSETINT! (inev.arg, (x as EMACS_INT + base) as EMACS_INT);   // C 6228
			      kbd_buffer_store_event (&raw mut inev as *mut c_void);   // C 6229
			      EVENT_INIT (&raw mut inev);                  // C 6230
			    }
			  else if ((*points.offset (i as isize)).dwFlags
				   & TOUCHEVENTF_DOWN as DWORD) != 0   // C 6232
			    {
			      let mut recorded_p: bool;                    // C 6234
			      let mut window: Lisp_Object;                 // C 6235

			      recorded_p
				= (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize] != -1;   // C 6237-6238

			      /* Update the local record of its
				 position.  */
			      (*((*f).output_data.w32 as *mut w32_output)).touch_x[x as usize] = px;   // C 6242
			      (*((*f).output_data.w32 as *mut w32_output)).touch_y[x as usize] = py;   // C 6243

			      if recorded_p                              // C 6245
				{
				  movement_p = true;                       // C 6247
				  i += 1; continue;                        // C 6248
				}

			      /* This event might have landed above the tool
				 bar, which if true its dwID should be
				 reserved for manipulation of the tool bar.  */
			      window = window_from_coordinates (f as *mut c_void, px, py,
								core::ptr::null_mut (),
								true, true, true);   // C 6254-6255
			      if EQ (window, (*f).tool_bar_window)         // C 6256
				{
				  if !NILP (Vmouse_highlight)              // C 6258
				    {
				      note_mouse_highlight (f, px, py);    // C 6260

				      /* Always allow future mouse motion to
					 update the mouse highlight, no matter
					 where it is.  */
				      memset (&raw mut (*dpyinfo).last_mouse_glyph as *mut c_void, 0,
					      core::mem::size_of_val (&(*dpyinfo).last_mouse_glyph));   // C 6265-6266
				      (*dpyinfo).last_mouse_glyph_frame = f;   // C 6267
				    }

				  handle_tool_bar_click (f, px, py, true, 0);   // C 6270
				  (*((*f).output_data.w32 as *mut w32_output)).tool_bar_dwID
				    = (*points.offset (i as isize)).dwID;      // C 6271-6272
				  i += 1; continue;                        // C 6273
				}

			      /* Report and record (if not already recorded)
				 the addition.  */
			      (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize]
				= (*points.offset (i as isize)).dwID as c_int;      // C 6278

			      input_event_bits! (kind, &raw mut inev, TOUCHSCREEN_BEGIN_EVENT);   // C 6280
			      inev.timestamp = msg.msg.time as Time;    // C 6281
			      XSETFRAME! (inev.frame_or_window, f);        // C 6282
			      XSETINT! (inev.x, px as EMACS_INT);          // C 6283
			      XSETINT! (inev.y, py as EMACS_INT);          // C 6284
			      XSETINT! (inev.arg, (x as EMACS_INT + base) as EMACS_INT);   // C 6285
			      kbd_buffer_store_event (&raw mut inev as *mut c_void);   // C 6286
			      EVENT_INIT (&raw mut inev);                  // C 6287
			    }
			  else
			    {
			      let recorded_p
				= (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize] != -1;   // C 6291-6292

			      if !recorded_p                               // C 6294
				{ i += 1; continue; }                      // C 6295

			      if (*((*f).output_data.w32 as *mut w32_output)).touch_x[x as usize] != px   // C 6297
				  || (*((*f).output_data.w32 as *mut w32_output)).touch_y[x as usize] != py   // C 6298
				{
				  movement_p = true;                       // C 6300
				  (*((*f).output_data.w32 as *mut w32_output)).touch_ids[x as usize]
				    = (*points.offset (i as isize)).dwID as c_int;   // C 6301-6302
				  (*((*f).output_data.w32 as *mut w32_output)).touch_x[x as usize] = px;   // C 6303
				  (*((*f).output_data.w32 as *mut w32_output)).touch_y[x as usize] = py;   // C 6304
				}
			    }

			  i += 1;                                          // C 6151
			}

		      /* Report updated positions of touchpoints if some
			 changed.  */
		      if movement_p                                        // C 6311
			{
			  let mut arg: Lisp_Object;                        // C 6313

			  input_event_bits! (kind, &raw mut inev, TOUCHSCREEN_UPDATE_EVENT);   // C 6315
			  inev.timestamp = msg.msg.time as Time;        // C 6316
			  XSETFRAME! (inev.frame_or_window, f);            // C 6317
			  arg = Qnil ();                                   // C 6318

			  i = 0;
			  while i < MAX_TOUCH_POINTS as c_int                       // C 6320
			    {
			      if (*((*f).output_data.w32 as *mut w32_output)).touch_ids[i as usize] == -1   // C 6322
				{ i += 1; continue; }                      // C 6323

			      arg
				= Fcons (list3i ((*((*f).output_data.w32 as *mut w32_output)).touch_x[i as usize] as EMACS_INT,
						 (*((*f).output_data.w32 as *mut w32_output)).touch_y[i as usize] as EMACS_INT,
						 (i as EMACS_INT + base) as EMACS_INT),   // C 6326-6328
				 arg);                                     // C 6329
			      i += 1;                                      // C 6320
			    }

			  inev.arg = arg;                                  // C 6332

			  /* Don't generate events if they would be empty.  */
			  if NILP (arg)                                    // C 6335
			    { EVENT_INIT (&raw mut inev); }                // C 6336
			}
		    }
		}

	      (pfnCloseTouchInputHandle.unwrap ()) (msg.msg.lParam as HANDLE);   // C 6341
	    }

	  /* C's `default:` (C 6344-6352).  */
	  _ =>
	    {
	      /* Check for messages registered at runtime.  */
	      if msg.msg.message == msh_mousewheel                // C 6346
		{
		  /* Forward MSH_MOUSEWHEEL as WM_MOUSEWHEEL.  */
		  msg.msg.message = WM_MOUSEWHEEL as c_uint;           // C 6349
		  prepend_msg (&raw mut msg);                            // C 6350
		}
	    }
	}
      }

      if input_event_bits! (kind, &raw mut inev) != NO_EVENT              // C 6355
	{
	  kbd_buffer_store_event_hold (&raw mut inev, hold_quit);        // C 6357
	  count += 1;                                                    // C 6358
	}

      if do_help != 0                                                   // C 6361
	  && !(!hold_quit.is_null ()
	       && input_event_bits! (kind, hold_quit) != NO_EVENT)       // C 6362
	{
	  let mut frame: Lisp_Object;                                     // C 6364

	  if !f.is_null ()                                                // C 6366
	    { XSETFRAME! (frame, f); }                                    // C 6367
	  else
	    { frame = Qnil (); }                                          // C 6369

	  if do_help > 0                                                  // C 6371
	    {
	      if NILP (help_echo_string)                                  // C 6373
		{
		  help_echo_object = Qnil ();                             // C 6375
		  help_echo_window = Qnil ();
		  help_echo_pos = -1;                                     // C 6376
		}

	      any_help_event_p = 1;                                       // C 6379
	      gen_help_event (help_echo_string, frame, help_echo_window,   // C 6380
			      help_echo_object, help_echo_pos as isize);   // C 6381
	    }
	  else
	    {
	      help_echo_string = Qnil ();                                 // C 6385
	      gen_help_event (Qnil (), frame, Qnil (), Qnil (), 0);        // C 6386
	    }
	  count += 1;                                                     // C 6388
	}

      /* Event processing might have drawn to F outside redisplay.  If
	 that is the case, flush any changes that have been made to
	 the front buffer.  */

      if !f.is_null () && !w32_disable_double_buffering                 // C 6395
	  && w32_output_bits! (paint_buffer_dirty, FRAME_OUTPUT_DATA! (f)) != 0   // C 6396
	  && !FRAME_GARBAGED_P (f) && !ignore_dirty_back_buffer                   // C 6397
	{ w32_show_back_buffer (f); }                                     // C 6398
    }

  /* If the focus was just given to an autoraising frame,
     raise it now.  FIXME: handle more than one such frame.  */
  if !(*dpyinfo).w32_pending_autoraise_frame.is_null ()                 // C 6403
    {
      w32_raise_frame ((*dpyinfo).w32_pending_autoraise_frame);         // C 6405
      (*dpyinfo).w32_pending_autoraise_frame = core::ptr::null_mut ();   // C 6406
    }

  /* Check which frames are still visible, if we have enqueued any user
     events or been notified of events that may affect visibility.  We
     do this here because there doesn't seem to be any direct
     notification from Windows that the visibility of a window has
     changed (at least, not in all cases).  */
  if count > 0 || check_visibility != 0                                 // C 6414
    {
      let mut tail: Lisp_Object;                                        // C 6416
      let mut frame: Lisp_Object;

      FOR_EACH_FRAME! (tail, frame, {                                   // C 6418
	let f: *mut frame = XFRAME (frame);                              // C 6420
	/* The tooltip has been drawn already.  Avoid the
	   SET_FRAME_GARBAGED below.  */
	if FRAME_TOOLTIP_P (f)                                           // C 6423
	  { continue; }                                                  // C 6424

	/* Check "visible" frames and mark each as visible or not.
	   Note that visible is nonzero for unobscured and obscured
	   frames, but zero for hidden and iconified frames.  */
	if FRAME_W32_P (f) && FRAME_VISIBLE_P (f)                        // C 6429
	  {
	    /* C declares the `RECT` uninitialised and fills it with `GetClipBox`
	       (PORTING.md 1.5).  */
	    let mut clipbox: RECT = core::mem::zeroed ();                 // C 6431
	    let mut hdc: HDC;                                            // C 6432

	    enter_crit ();                                               // C 6434
	    /* Query clipping rectangle for the entire window area
	       (GetWindowDC), not just the client portion (GetDC).
	       Otherwise, the scrollbars and menubar aren't counted as
	       part of the visible area of the frame, and we may think
	       the frame is obscured when really a scrollbar is still
	       visible and gets WM_PAINT messages above.  */
	    hdc = GetWindowDC (FRAME_W32_WINDOW (f));                   // C 6441
	    GetClipBox (hdc, &raw mut clipbox);                          // C 6442
	    ReleaseDC (FRAME_W32_WINDOW (f), hdc);                      // C 6443
	    leave_crit ();                                               // C 6444

	    if !(clipbox.right == clipbox.left                           // C 6446
		 || clipbox.bottom == clipbox.top)                       // C 6447
	      {
		/* Frame is not obscured, so mark it as such.  */
		SET_FRAME_VISIBLE! (f, 1);                               // C 6450
	      }
	  }
      });                                                                // C 6454
    }

  unblock_input ();                                                      // C 6456
  return count;                                                          // C 6457
}

/* C 6462-6464 is the "Text Cursor" banner, C 6466-6471 the note above `w32_clip_to_row` and C
   6473-6489 that function - the next slice's text.  */

/* The five functions C 6506-6733 calls have no prototype in the port's mirrors: `dispextern.h`
   declares all five (`get_phys_cursor_glyph` 3688's neighbourhood, `get_phys_cursor_geometry`
   3652, `draw_phys_cursor_glyph` 3649, `erase_phys_cursor` 3654, `draw_fringe_bitmap` 3703) and
   C 25's chain reaches them through `w32term.h` 22, so the unit declares them at the use sites the
   way `src/xterm.rs` 38828-38833 does for the same three cursor entries (P-72 rule 4).
   `FrameRect` is `<wingdi.h>`'s, and `WM_IME_STARTCOMPOSITION` is `<imm.h>`'s (C 37's include).
   The SDK records are UNVERIFIED.  */
/// C: WINUSERAPI HBRUSH WINAPI CreateSolidBrush (COLORREF color);
/// C: WINUSERAPI int WINAPI FrameRect (HDC hDC, const RECT *lprc, HBRUSH hbr);
/// C: #define WM_IME_STARTCOMPOSITION 0x010D
/// C: extern struct glyph *get_phys_cursor_glyph (struct window *w);
/// C: extern void get_phys_cursor_geometry (struct window *, struct glyph_row *, struct glyph *,
/// C:   int *, int *, int *);
/// C: extern void draw_phys_cursor_glyph (struct window *, struct glyph_row *,
/// C:   enum draw_glyphs_face);
/// C: extern void erase_phys_cursor (struct window *);
/// C: void draw_fringe_bitmap (struct window *, struct glyph_row *, int);   [dispextern.h 3703]
#[cfg (feature = "have_ntgui")]
pub const WM_IME_STARTCOMPOSITION: c_int = 0x010D;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn FrameRect (hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn get_phys_cursor_glyph (w: *mut window) -> *mut glyph;
  pub fn get_phys_cursor_geometry (w: *mut window, row: *mut glyph_row, glyph: *mut glyph,
				   xp: *mut c_int, yp: *mut c_int, heightp: *mut c_int);
  pub fn draw_phys_cursor_glyph (w: *mut window, row: *mut glyph_row, draw: draw_glyphs_face);
  pub fn erase_phys_cursor (w: *mut window);
  pub fn draw_fringe_bitmap (w: *mut window, row: *mut glyph_row, left_p: c_int);
}

/* C 1656's `FACE_FROM_ID` is dispextern.h's second `INLINE` face accessor: C pastes it into every
   unit that includes the header, and the port's live form is one copy per including unit (the
   `src/xterm.rs` 38843 block, whose shape this is; the sibling `FACE_FROM_ID_OR_NULL` copy is this
   unit's own at C 868).  C 6569 reads it.  */
/// C: INLINE struct face *FACE_FROM_ID (struct frame *f, int id)
#[cfg (feature = "have_ntgui")]
#[inline(always)]
unsafe fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < (*FRAME_FACE_CACHE! (f)).used);
  return *(*FRAME_FACE_CACHE! (f)).faces_by_id.add (id as usize);      // C 1656
}

/* C 6462-6464 is the "Text Cursor" banner, carried as C's own text at its position.  */
/* C: /***********************************************************************
   C: 			     Text Cursor
   C:  ***********************************************************************/  */

/* C 6466-6471's note above `w32_clip_to_row`, carried as C's own text.  */
/* C: /* Set clipping for output in glyph row ROW.  W is the window in which
   C:    we operate.  GC is the graphics context to set clipping in.
   C:
   C:    ROW may be a text row or, e.g., a mode line.  Text rows must be
   C:    clipped to the interior of the window dedicated to text display,
   C:    mode lines must be clipped to the whole window.  */  */

/// C 6473-6489: `static void w32_clip_to_row (struct window *w, struct glyph_row *row, enum
/// glyph_row_area area, HDC hdc)` - C 6477's `RECT`, C 6478's three locals, C 6480's `window_box`
/// and C 6482-6486's five member writes, closed by C 6488's `w32_set_clip_rectangle`.
/// C: static void
/// C: w32_clip_to_row (struct window *w, struct glyph_row *row,
/// C: 		 enum glyph_row_area area, HDC hdc)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_clip_to_row (w: *mut window, row: *mut glyph_row, area: glyph_row_area, hdc: HDC)
{
  /* C declares the `RECT` uninitialised and fills every member below (PORTING.md 1.5).  */
  let mut clip_rect: RECT = core::mem::zeroed ();                       // C 6477
  /* C declares these three uninitialised and fills them with `window_box` (PORTING.md 1.5).  */
  let mut window_x: c_int = 0;                                         // C 6478
  let mut window_y: c_int = 0;
  let mut window_width: c_int = 0;

  window_box (w, area, &raw mut window_x, &raw mut window_y, &raw mut window_width,
	      core::ptr::null_mut ());                                 // C 6480

  clip_rect.left = window_x;                                           // C 6482
  clip_rect.top = WINDOW_TO_FRAME_PIXEL_Y! (w, (*row).y);              // C 6483
  clip_rect.top = max! (clip_rect.top, window_y);                      // C 6484
  clip_rect.right = clip_rect.left + window_width;                     // C 6485
  clip_rect.bottom = clip_rect.top + (*row).visible_height;            // C 6486

  w32_set_clip_rectangle (hdc, &raw mut clip_rect);                    // C 6488
}

/* C 6492's note above `w32_draw_hollow_cursor`, carried as C's own text.  */
/* C: /* Draw a hollow box cursor on window W in glyph row ROW.  */  */

/// C 6494-6533: `static void w32_draw_hollow_cursor (struct window *w, struct glyph_row *row)` - C
/// 6497-6502's seven locals (with the brush C 6502 creates), C 6506-6511's glyph lookup and its
/// early return, C 6514-6524's geometry (with C 6519-6521's R2L shift), C 6526-6532's DC, clipped
/// `FrameRect` and reset.
/// C: static void
/// C: w32_draw_hollow_cursor (struct window *w, struct glyph_row *row)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_hollow_cursor (w: *mut window, row: *mut glyph_row)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                      // C 6497
  let mut hdc: HDC;                                                    // C 6498
  /* C declares the `RECT` uninitialised and fills every member below (PORTING.md 1.5).  */
  let mut rect: RECT = core::mem::zeroed ();                           // C 6499
  /* C declares these three uninitialised and fills them with `get_phys_cursor_geometry`
     (PORTING.md 1.5).  */
  let mut left: c_int = 0;                                             // C 6500
  let mut top: c_int = 0;
  let mut h: c_int = 0;
  let mut cursor_glyph: *mut glyph;                                    // C 6501
  let hb: HBRUSH
    = CreateSolidBrush ((*((*f).output_data.w32 as *mut w32_output)).cursor_pixel);   // C 6502

  /* Get the glyph the cursor is on.  If we can't tell because
     the current matrix is invalid or such, give up.  */
  cursor_glyph = get_phys_cursor_glyph (w);                            // C 6506
  if cursor_glyph.is_null ()                                           // C 6507
    {
      DeleteObject (hb as HGDIOBJ);                                    // C 6509
      return;                                                          // C 6510
    }

  /* Compute frame-relative coordinates for phys cursor.  */
  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut left, &raw mut top, &raw mut h);   // C 6514
  rect.left = left;                                                    // C 6515
  /* When on R2L character, show cursor at the right edge of the
     glyph, unless the cursor box is as wide as the glyph or wider
     (the latter happens when x-stretch-cursor is non-nil).  */
  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0          // C 6519
      && ((*cursor_glyph).pixel_width as c_int) > (*w).phys_cursor_width   // C 6520
    { rect.left += (*cursor_glyph).pixel_width as c_int - (*w).phys_cursor_width; }   // C 6521
  rect.top = top;                                                      // C 6522
  rect.bottom = rect.top + h;                                          // C 6523
  rect.right = rect.left + (*w).phys_cursor_width;                     // C 6524

  hdc = get_frame_dc (f);                                              // C 6526
  /* Set clipping, draw the rectangle, and reset clipping again.  */
  w32_clip_to_row (w, row, TEXT_AREA, hdc);                            // C 6528
  FrameRect (hdc, &raw const rect, hb);                               // C 6529
  DeleteObject (hb as HGDIOBJ);                                        // C 6530
  w32_set_clip_rectangle (hdc, core::ptr::null_mut ());                // C 6531
  release_frame_dc (f, hdc);                                           // C 6532
}

/* C 6536-6541's note above `w32_draw_bar_cursor`, carried as C's own text.  */
/* C: /* Draw a bar cursor on window W in glyph row ROW.
   C:
   C:    Implementation note: One would like to draw a bar cursor with an
   C:    angle equal to the one given by the font property XA_ITALIC_ANGLE.
   C:    Unfortunately, I didn't find a font yet that has this property set.
   C:    --gerd.  */  */

/// C 6543-6626: `static void w32_draw_bar_cursor (struct window *w, struct glyph_row *row, int
/// width, enum text_cursor_kinds kind)` - C 6547's frame, C 6553-6555's glyph lookup with its bare
/// `return`, C 6560-6565's image arm (the `MATRIX_ROW` of the current matrix and the cursor glyph)
/// and C 6566-6625's bar arm (the colour choice of C 6578-6579, C 6581's x, C 6584's clip, the two
/// `BAR_CURSOR`/`HBAR_CURSOR` branches with their `min`/`max` arithmetic and C 6623-6624's reset).
/// C: static void
/// C: w32_draw_bar_cursor (struct window *w, struct glyph_row *row,
/// C: 		   int width, enum text_cursor_kinds kind)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_bar_cursor (w: *mut window, row: *mut glyph_row, mut width: c_int,
			       kind: text_cursor_kinds)
{
  let f: *mut frame = XFRAME ((*w).frame);                             // C 6547
  let mut cursor_glyph: *mut glyph;                                    // C 6548

  /* If cursor is out of bounds, don't draw garbage.  This can happen
     in mini-buffer windows when switching between echo area glyphs
     and mini-buffer.  */
  cursor_glyph = get_phys_cursor_glyph (w);                            // C 6553
  if cursor_glyph.is_null ()                                           // C 6554
    { return; }                                                        // C 6555

  /* If on an image, draw like a normal cursor.  That's usually better
     visible than drawing a bar, esp. if the image is large so that
     the bar might not be in the window.  */
  if glyph_bits! (type, (*cursor_glyph)) == IMAGE_GLYPH                 // C 6560
    {
      let row: *mut glyph_row;
      row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).phys_cursor.vpos);   // C 6563
      draw_phys_cursor_glyph (w, row, DRAW_CURSOR);                     // C 6564
    }
  else
    {
      let mut cursor_color: COLORREF
	= (*((*f).output_data.w32 as *mut w32_output)).cursor_pixel;     // C 6568
      let face: *mut face = FACE_FROM_ID (f, glyph_bits! (face_id, (*cursor_glyph)));   // C 6569
      let mut x: c_int;                                                 // C 6570
      let mut hdc: HDC;                                                 // C 6571

      /* If the glyph's background equals the color we normally draw
	 the bar cursor in, the bar cursor in its normal color is
	 invisible.  Use the glyph's foreground color instead in this
	 case, on the assumption that the glyph's colors are chosen so
	 that the glyph is legible.  */
      if (*face).background == cursor_color as c_ulong                  // C 6578
	{ cursor_color = (*face).foreground as COLORREF; }              // C 6579

      x = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);        // C 6581

      hdc = get_frame_dc (f);                                           // C 6583
      w32_clip_to_row (w, row, TEXT_AREA, hdc);                         // C 6584

      if kind == BAR_CURSOR                                             // C 6586
	{
	  if width < 0                                                  // C 6588
	    { width = FRAME_CURSOR_WIDTH (f); }                         // C 6589
	  width = min! ((*cursor_glyph).pixel_width as c_int, width);       // C 6590

	  (*w).phys_cursor_width = width;                               // C 6592

	  /* If the character under cursor is R2L, draw the bar cursor
	     on the right of its glyph, rather than on the left.  */
	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0   // C 6596
	    { x += (*cursor_glyph).pixel_width as c_int - width; }      // C 6597

	  w32_fill_area! (f, hdc, cursor_color, x,                        // C 6599
			  WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y),   // C 6600
			  width, (*row).height);                        // C 6601
	}
      else	/* HBAR_CURSOR */                                       // C 6603
	{
	  /* C declares these three uninitialised and fills them with `get_phys_cursor_geometry`
	     (PORTING.md 1.5).  */
	  let mut dummy_x: c_int = 0;                                   // C 6605
	  let mut dummy_y: c_int = 0;
	  let mut dummy_h: c_int = 0;

	  if width < 0                                                  // C 6607
	    { width = (*row).height; }                                  // C 6608

	  width = min! ((*row).height, width);                            // C 6610

	  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut dummy_x,
				    &raw mut dummy_y, &raw mut dummy_h);   // C 6612-6613
	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0   // C 6614
	      && ((*cursor_glyph).pixel_width as c_int) > (*w).phys_cursor_width   // C 6615
	    { x += (*cursor_glyph).pixel_width as c_int - (*w).phys_cursor_width; }   // C 6616
	  w32_fill_area! (f, hdc, cursor_color, x,                        // C 6617
			  WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y + (*row).height - width),   // C 6618-6619
			  (*w).phys_cursor_width, width);                 // C 6620
	}

      w32_set_clip_rectangle (hdc, core::ptr::null_mut ());             // C 6623
      release_frame_dc (f, hdc);                                       // C 6624
    }
}

/* C 6629's note above `w32_define_frame_cursor`, carried as C's own text.  */
/* C: /* RIF: Define cursor CURSOR on frame F.  */  */

/// C 6631-6635: `static void w32_define_frame_cursor (struct frame *f, Emacs_Cursor cursor)` - the
/// one `w32_define_cursor` call.
/// C: static void
/// C: w32_define_frame_cursor (struct frame *f, Emacs_Cursor cursor)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_define_frame_cursor (f: *mut frame, cursor: Emacs_Cursor)
{
  w32_define_cursor (FRAME_W32_WINDOW! (f), cursor);                   // C 6634
}

/* C 6638's note above `w32_clear_frame_area`, carried as C's own text.  */
/* C: /* RIF: Clear area on frame F.  */  */

/// C 6640-6648: `static void w32_clear_frame_area (struct frame *f, int x, int y, int width, int
/// height)` - the DC, C 6646's `w32_clear_area` and C 6647's release.
/// C: static void
/// C: w32_clear_frame_area (struct frame *f, int x, int y, int width, int height)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_clear_frame_area (f: *mut frame, x: c_int, y: c_int, width: c_int, height: c_int)
{
  let hdc: HDC;                                                        // C 6643

  hdc = get_frame_dc (f);                                              // C 6645
  w32_clear_area! (f, hdc, x, y, width, height);                       // C 6646
  release_frame_dc (f, hdc);                                           // C 6647
}

/// C 6650-6752: `static void w32_draw_window_cursor (struct window *w, struct glyph_row
/// *glyph_row, int x, int y, enum text_cursor_kinds cursor_type, int cursor_width, bool on_p,
/// bool active_p)` - C 6657's `on_p` guard, C 6659-6675's system-caret choice (with C 6667's
/// `erase_phys_cursor` and the chained store of C 6669), C 6677's flag, C 6682-6714's active-cursor
/// block (the face lookup, the four `w32_system_caret_*` stores, C 6699-6700's font choice and C
/// 6702-6713's four `PostMessage` calls), C 6716-6724's fringe branch and C 6726-6750's
/// `switch (cursor_type)` with its five labels and C's own `default:` abort.  C's `x` and `y`
/// parameters are unread in C's body too (`-Wunused-parameter`; the file's `unused_variables`
/// covers them), so the port keeps C's names and no use.
/// C: static void
/// C: w32_draw_window_cursor (struct window *w, struct glyph_row *glyph_row,
/// C: 			int x, int y, enum text_cursor_kinds cursor_type,
/// C: 			int cursor_width, bool on_p, bool active_p)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_draw_window_cursor (w: *mut window, glyph_row: *mut glyph_row,
				  x: c_int, y: c_int, mut cursor_type: text_cursor_kinds,
				  cursor_width: c_int, on_p: bool, active_p: bool)
{
  if on_p                                                              // C 6657
    {
      /* If the user wants to use the system caret, make sure our own
	 cursor remains invisible.  */
      if w32_use_visible_system_caret                                  // C 6661
	{
	  /* Call to erase_phys_cursor here seems to use the
	     wrong values of w->phys_cursor, as they have been
	     overwritten before this function was called. */
	  if (*w).phys_cursor_type != NO_CURSOR                        // C 6666
	    { erase_phys_cursor (w); }                                 // C 6667

	  /* C's chained `cursor_type = w->phys_cursor_type = NO_CURSOR;` is its own two stores.  */
	  (*w).phys_cursor_type = NO_CURSOR;                          // C 6669
	  cursor_type = NO_CURSOR;                                    // C 6669
	  (*w).phys_cursor_width = -1;                                // C 6670
	}
      else
	{
	  (*w).phys_cursor_type = cursor_type;                        // C 6674
	}

      window_bits! (phys_cursor_on_p, w, true);                       // C 6677

      /* If this is the active cursor, we need to track it with the
	 system caret, so third party software like screen magnifiers
	 and speech synthesizers can follow the cursor.  */
      if active_p                                                     // C 6682
	{
	  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                // C 6684
	  let face_id: c_int = lookup_basic_face (w, f, DEFAULT_FACE_ID);   // C 6685
	  let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);        // C 6686
	  let hwnd: HWND = FRAME_W32_WINDOW! (f);                         // C 6687

	  w32_system_caret_x
	    = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);      // C 6689-6690
	  w32_system_caret_y
	    = (WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y)           // C 6692
	       + (*glyph_row).ascent as c_int - (*w).phys_cursor_ascent);   // C 6693
	  w32_system_caret_window = w;                                    // C 6694
	  w32_system_caret_hdr_height = WINDOW_TAB_LINE_HEIGHT! (w)       // C 6695
	    + WINDOW_HEADER_LINE_HEIGHT! (w);                             // C 6696
	  w32_system_caret_mode_height = WINDOW_MODE_LINE_HEIGHT! (w);    // C 6697

	  w32_system_remap_font
	    = if !face.is_null () && !(*face).font.is_null ()
		{ (*face).font as *mut font } else { FRAME_FONT! (f) as *mut font };   // C 6699-6700

	  PostMessage (hwnd, WM_IME_STARTCOMPOSITION as UINT, 0, 0);      // C 6702

	  /* If the size of the active cursor changed, destroy the old
	     system caret.  */
	  if !w32_system_caret_hwnd.is_null ()                           // C 6706
	      && w32_system_caret_height != (*w).phys_cursor_height      // C 6707
	    { PostMessage (hwnd, WM_EMACS_DESTROY_CARET as UINT, 0, 0); }   // C 6708

	  w32_system_caret_height = (*w).phys_cursor_height;              // C 6710

	  /* Move the system caret.  */
	  PostMessage (hwnd, WM_EMACS_TRACK_CARET as UINT, 0, 0);         // C 6713
	}

      if glyph_row_bits! (exact_window_width_line_p, glyph_row)       // C 6716
	  && (if glyph_row_bits! (reversed_p, glyph_row)                  // C 6717
		{ (*w).phys_cursor.hpos < 0 }                            // C 6718
	      else { (*w).phys_cursor.hpos >= (*glyph_row).used[TEXT_AREA as usize] as c_int })   // C 6719
	{
	  glyph_row_bits! (cursor_in_fringe_p, glyph_row, true);          // C 6721
	  draw_fringe_bitmap (w, glyph_row, glyph_row_bits! (reversed_p, glyph_row) as c_int);   // C 6722
	  return;                                                        // C 6723
	}

      match cursor_type                                               // C 6726
	{
	  HOLLOW_BOX_CURSOR =>                                        // C 6728
	    { w32_draw_hollow_cursor (w, glyph_row); }                // C 6729

	  FILLED_BOX_CURSOR =>                                        // C 6732
	    { draw_phys_cursor_glyph (w, glyph_row, DRAW_CURSOR); }    // C 6733

	  BAR_CURSOR =>                                               // C 6736
	    { w32_draw_bar_cursor (w, glyph_row, cursor_width, BAR_CURSOR); }   // C 6737

	  HBAR_CURSOR =>                                              // C 6740
	    { w32_draw_bar_cursor (w, glyph_row, cursor_width, HBAR_CURSOR); }   // C 6741

	  NO_CURSOR =>                                                // C 6744
	    { (*w).phys_cursor_width = 0; }                           // C 6745

	  /* C's `default:` (C 6748-6749).  */
	  _ => { emacs_abort (); }
	}
    }
}

/* C 6755-6758's "Icons" banner and `static bool w32_bitmap_icon (struct frame *f, Lisp_Object
   icon)` open the next slice.  */
/* The SDK surface this slice's `w32_bitmap_icon` and `w32_new_font` read: `<winuser.h>`'s icon
   entry points and their constants, `frame.h` 1840's `EMACS_CLASS`, `dispextern.h` 3631's
   `get_font_ascent_descent` and `frame.h` 1511-1512's `adjust_frame_size` (whose definition is
   `src/frame.rs` 1221).  Every name stands at the use site (P-88 (1)); the SDK records are
   UNVERIFIED.  Three `frame.h` accessors this slice *writes* get the two-arm form the unit carries
   per site (the `src/androidterm.rs` 2739-2757 shape; `include/frame.rs` has only the reader).  */
/// C: typedef void *HINSTANCE; typedef char *LPSTR; typedef const char *LPCSTR;
/// C: #define MAKEINTRESOURCEA(i) ((LPSTR)((ULONG_PTR)((WORD)(i))))
/// C: #define IMAGE_ICON 1 / #define LR_DEFAULTSIZE 0x00000040 / #define LR_LOADFROMFILE 0x00000010
/// C: #define SM_CXSMICON 49 / #define SM_CYSMICON 50
/// C: #define WM_SETICON 0x0080 / #define ICON_SMALL 0 / #define ICON_BIG 1
/// C: #define IDI_APPLICATION MAKEINTRESOURCE(32512) / #define IDI_HAND MAKEINTRESOURCE(32513)
/// C: #define IDI_QUESTION MAKEINTRESOURCE(32514) / #define IDI_EXCLAMATION MAKEINTRESOURCE(32515)
/// C: #define IDI_ASTERISK MAKEINTRESOURCE(32516) / #define IDI_WINLOGO MAKEINTRESOURCE(32517)
/// C: WINUSERAPI HICON WINAPI LoadIconA (HINSTANCE hInstance, LPCSTR lpIconName);
/// C: WINUSERAPI HANDLE WINAPI LoadImageA (HINSTANCE hinst, LPCSTR lpszName, UINT uType,
/// C:   int cxDesired, int cyDesired, UINT fuLoad);
/// C: frame.h 1840: `#define EMACS_CLASS "Emacs"`
/// C: extern void get_font_ascent_descent (struct font *, int *, int *);   [dispextern.h 3631]
/// C: extern void adjust_frame_size (struct frame *, int, int, int, bool, Lisp_Object);
/// C:   [frame.h 1511-1512]
#[cfg (feature = "have_ntgui")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "have_ntgui")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "have_ntgui")]
macro_rules! MAKEINTRESOURCE {
  ($i:expr) => { (($i as ULONG_PTR) as LPSTR) };
}
#[cfg (feature = "have_ntgui")]
pub const IMAGE_ICON: UINT = 1;
#[cfg (feature = "have_ntgui")]
pub const LR_DEFAULTSIZE: UINT = 0x00000040;
#[cfg (feature = "have_ntgui")]
pub const LR_LOADFROMFILE: UINT = 0x00000010;
#[cfg (feature = "have_ntgui")]
pub const SM_CXSMICON: c_int = 49;
#[cfg (feature = "have_ntgui")]
pub const SM_CYSMICON: c_int = 50;
#[cfg (feature = "have_ntgui")]
pub const WM_SETICON: c_int = 0x0080;
#[cfg (feature = "have_ntgui")]
pub const ICON_SMALL: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const ICON_BIG: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const IDI_APPLICATION: LPSTR = MAKEINTRESOURCE! (32512);
#[cfg (feature = "have_ntgui")]
pub const IDI_HAND: LPSTR = MAKEINTRESOURCE! (32513);
#[cfg (feature = "have_ntgui")]
pub const IDI_QUESTION: LPSTR = MAKEINTRESOURCE! (32514);
#[cfg (feature = "have_ntgui")]
pub const IDI_EXCLAMATION: LPSTR = MAKEINTRESOURCE! (32515);
#[cfg (feature = "have_ntgui")]
pub const IDI_ASTERISK: LPSTR = MAKEINTRESOURCE! (32516);
#[cfg (feature = "have_ntgui")]
pub const IDI_WINLOGO: LPSTR = MAKEINTRESOURCE! (32517);
#[cfg (feature = "have_ntgui")]
pub const EMACS_CLASS: &core::ffi::CStr = c"Emacs";
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn LoadIcon (hInstance: HINSTANCE, lpIconName: LPCSTR) -> HANDLE;
  pub fn LoadImage (hinst: HINSTANCE, lpszName: LPCSTR, uType: UINT, cxDesired: c_int,
		    cyDesired: c_int, fuLoad: UINT) -> HANDLE;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  pub fn adjust_frame_size (f: *mut frame, new_text_width: c_int, new_text_height: c_int,
			    inhibit: c_int, pretend: bool, parameter: Lisp_Object);
}

/// C: frame.h 1091: `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
/// C: frame.h 1355: `#define FRAME_COLUMN_WIDTH(f) ((f)->column_width)`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_COLUMN_WIDTH {
  ($f:expr) => { FRAME_COLUMN_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).column_width = $v };
}
/// C: frame.h 1286: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f) ((f)->config_scroll_bar_width)`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_width = $v };
}
/// C: frame.h 1296: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) ((f)->config_scroll_bar_cols)`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_COLS ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}
/// C: frame.h 1357: `#define FRAME_LINE_HEIGHT(f) ((f)->line_height)`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_LINE_HEIGHT {
  ($f:expr) => { FRAME_LINE_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).line_height = $v };
}

/* C 6756's "Icons." banner, carried as C's own text at its position.  */
/* C: /* Icons.  */  */

/// C 6758-6816: `static bool w32_bitmap_icon (struct frame *f, Lisp_Object icon)` - C 6761-6762's
/// two handles, C 6764-6765's window guard, C 6767-6802's four-way icon choice (the default icon,
/// the two `LoadImage` calls of the `STRINGP` arm, the six `IDI_*` symbols of the `SYMBOLP` arm and
/// C 6801-6802's `return 1`), C 6804-6805's NULL guard and C 6807-6815's two `PostMessage` calls
/// with the `return 0`.
/// C: static bool
/// C: w32_bitmap_icon (struct frame *f, Lisp_Object icon)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_bitmap_icon (f: *mut frame, icon: Lisp_Object) -> bool
{
  let mut main_icon: HANDLE;                                           // C 6761
  let mut small_icon: HANDLE = core::ptr::null_mut ();                 // C 6762

  if FRAME_W32_WINDOW! (f).is_null ()                                  // C 6764
    { return true; }                                                   // C 6765

  if NILP (icon)                                                       // C 6767
    { main_icon = LoadIcon (hinst, EMACS_CLASS.as_ptr ()); }           // C 6768
  else if STRINGP (icon)                                               // C 6769
    {
      /* Load the main icon from the named file.  */
      main_icon = LoadImage (core::ptr::null_mut (), SDATA (icon) as LPCTSTR, IMAGE_ICON, 0, 0,   // C 6772
			     LR_DEFAULTSIZE | LR_LOADFROMFILE);        // C 6773
      /* Try to load a small icon to go with it.  */
      small_icon = LoadImage (core::ptr::null_mut (), SDATA (icon) as LPCSTR, IMAGE_ICON,   // C 6775
			      GetSystemMetrics (SM_CXSMICON),          // C 6776
			      GetSystemMetrics (SM_CYSMICON),          // C 6777
			      LR_LOADFROMFILE);                        // C 6778
    }
  else if SYMBOLP (icon)                                               // C 6780
    {
      let name: LPCTSTR;                                               // C 6782

      if EQ (icon, Qapplication ())                                    // C 6784
	{ name = IDI_APPLICATION as LPCTSTR; }                         // C 6785
      else if EQ (icon, Qhand ())                                      // C 6786
	{ name = IDI_HAND as LPCTSTR; }                                // C 6787
      else if EQ (icon, Qquestion ())                                  // C 6788
	{ name = IDI_QUESTION as LPCTSTR; }                            // C 6789
      else if EQ (icon, Qexclamation ())                               // C 6790
	{ name = IDI_EXCLAMATION as LPCTSTR; }                         // C 6791
      else if EQ (icon, Qasterisk ())                                  // C 6792
	{ name = IDI_ASTERISK as LPCTSTR; }                            // C 6793
      else if EQ (icon, Qwinlogo ())                                   // C 6794
	{ name = IDI_WINLOGO as LPCTSTR; }                             // C 6795
      else
	{ return true; }                                               // C 6797

      main_icon = LoadIcon (core::ptr::null_mut (), name);             // C 6799
    }
  else
    { return true; }                                                   // C 6802

  if main_icon.is_null ()                                              // C 6804
    { return true; }                                                   // C 6805

  PostMessage (FRAME_W32_WINDOW! (f), WM_SETICON as UINT, (ICON_BIG as WPARAM),   // C 6807
	       (main_icon as LPARAM));                                    // C 6808

  /* If there is a small icon that goes with it, set that too.  */
  if !small_icon.is_null ()                                            // C 6811
    { PostMessage (FRAME_W32_WINDOW! (f), WM_SETICON as UINT, (ICON_SMALL as WPARAM),   // C 6812
		   (small_icon as LPARAM)); }                          // C 6813

  return false;                                                        // C 6815
}

/* C 6818-6841: the "Handling X errors" banner and the commented-out list of the X-only display
   error functions, carried as C's own text at their position.  */
/* C: /************************************************************************
   C: 			  Handling X errors
   C:  ************************************************************************/  */

   /* Display Error Handling functions not used on W32. Listing them here
      helps diff stay in step when comparing w32term.c with xterm.c.

   x_error_catcher (display, error)
   x_catch_errors (dpy)
   x_catch_errors_unwind (old_val)
   x_check_errors (dpy, format)
   x_fully_uncatch_errors ()
   x_had_errors_p (dpy)
   x_clear_errors (dpy)
   x_uncatch_errors (dpy, count)
   x_trace_wire ()
   x_connection_signal (signalnum)
   x_connection_closed (dpy, error_message)
   x_error_quitter (display, error)
   x_error_handler (display, error)
   x_io_error_quitter (display)

    */

/* C 6843's banner above `w32_new_font`, carried as C's own text at its position.  */
/* C: /* Changing the font of the frame.  */  */

/// C 6846-6891: `static Lisp_Object w32_new_font (struct frame *f, Lisp_Object font_object, int
/// fontset)` - C 6849's `XFONT_OBJECT`, C 6850's three locals, C 6852-6853's fontset lookup, C
/// 6854's store, C 6855-6858's already-set early return, C 6860-6864's five stores (with C 6862's
/// chained assignment and C 6863's `get_font_ascent_descent`), C 6866-6876's scroll-bar column
/// arithmetic, C 6878's tab-bar height and C 6883-6886's `adjust_frame_size`.
/// C: static Lisp_Object
/// C: w32_new_font (struct frame *f, Lisp_Object font_object, int fontset)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_new_font (f: *mut frame, font_object: Lisp_Object, mut fontset: c_int) -> Lisp_Object
{
  let font: *mut font = XFONT_OBJECT (font_object);                    // C 6849
  let mut unit: c_int;                                                 // C 6850
  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;

  if fontset < 0                                                       // C 6852
    { fontset = fontset_from_font (font_object); }                     // C 6853
  FRAME_FONTSET! (f, fontset);                                         // C 6854
  if FRAME_FONT! (f) as *mut font == font                              // C 6855
    /* This font is already set in frame F.  There's nothing more to
       do.  */
    { return font_object; }                                            // C 6858

  FRAME_FONT! (f, font as *mut c_void);                                // C 6860
  FRAME_BASELINE_OFFSET! (f, (*font).baseline_offset);                 // C 6861
  /* C's chained `FRAME_COLUMN_WIDTH (f) = unit = font->average_width;` is its own two stores,
     the inner one first.  */
  unit = (*font).average_width;                                        // C 6862
  FRAME_COLUMN_WIDTH! (f, unit);                                       // C 6862
  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);   // C 6863
  FRAME_LINE_HEIGHT! (f, font_ascent + font_descent);                  // C 6864

  /* Compute number of scrollbar columns.  */
  unit = FRAME_COLUMN_WIDTH! (f);                                      // C 6867
  if FRAME_CONFIG_SCROLL_BAR_WIDTH! (f) > 0                            // C 6868
    {
      FRAME_CONFIG_SCROLL_BAR_COLS! (f, (FRAME_CONFIG_SCROLL_BAR_WIDTH! (f) + unit - 1) / unit);   // C 6869-6870
    }
  else
    {
      FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + unit - 1) / unit);       // C 6873
      FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, FRAME_CONFIG_SCROLL_BAR_COLS! (f) * unit);   // C 6874-6875
    }

  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT! (f));   // C 6878

/* Don't change the size of a tip frame; there's no point in
   doing it because it's done in Fx_show_tip, and it leads to
   problems because the tip frame has no widget.  */
  if !FRAME_NATIVE_WINDOW! (f).is_null () && !FRAME_TOOLTIP_P (f)      // C 6883
    {
      adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH! (f),   // C 6884-6885
			 FRAME_LINES (f) * FRAME_LINE_HEIGHT! (f), 3, false, Qfont ());   // C 6886
    }

  /* X version sets font of input methods here also.  */

  return font_object;                                                  // C 6890
}

/* C 6894-6906: the Input Methods banner and the commented-out list of the X-only IME functions,
   carried as C's own text at their position.  */
/* C: /***********************************************************************
   C: 	TODO: W32 Input Methods
   C:  ***********************************************************************/  */

   /* Listing missing functions from xterm.c helps diff stay in step.

   xim_destroy_callback (xim, client_data, call_data)
   xim_open_dpy (dpyinfo, resource_name)
   struct xim_inst_t
   xim_instantiate_callback (display, client_data, call_data)
   xim_initialize (dpyinfo, resource_name)
   xim_close_dpy (dpyinfo)

    */

/* C 6909-6912's note and `static void w32_calc_absolute_position (struct frame *f)` open the next
   slice.  */

/* The SDK surface this slice's three functions read: `<winuser.h>`'s placement and window-state
   entry points with their constants, plus dispnew.c's `do_pending_window_change` (defined at
   `src/dispnew.rs` 7434).  Every name stands at the use site (P-88 (1)); the SDK records are
   UNVERIFIED.  `XNegative`/`YNegative` and `FRAME_UNDECORATED` are already in this unit's include
   graph (`include/w32gui.rs` 146-147, `include/frame.rs`), and `w32_wm_set_size_hint` (defined
   later in this file, C 7477) is the second call of that class this lane carries.  */
/// C: WINUSERAPI BOOL WINAPI GetWindowPlacement (HWND hWnd, WINDOWPLACEMENT *lpwndpl);
/// C: WINUSERAPI BOOL WINAPI SetWindowPlacement (HWND hWnd, const WINDOWPLACEMENT *lpwndpl);
/// C: WINUSERAPI BOOL WINAPI ShowWindow (HWND hWnd, int nCmdShow);
/// C: #define GWL_STYLE (-16) / #define HWND_TOP ((HWND)0)
/// C: #define WS_OVERLAPPEDWINDOW (WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
/// C:   | WS_MINIMIZEBOX | WS_MAXIMIZEBOX)   /* 0x00cf0000 */
/// C: #define SW_SHOWNORMAL 1 / #define SW_MAXIMIZE 3
/// C: #define SWP_NOSIZE 0x0001 / #define SWP_NOZORDER 0x0004 / #define SWP_NOACTIVATE 0x0010
/// C: #define SWP_NOOWNERZORDER 0x0200
/// C: extern void do_pending_window_change (bool);            [dispnew.c]
#[cfg (feature = "have_ntgui")]
pub const GWL_STYLE: c_int = -16;
#[cfg (feature = "have_ntgui")]
pub const HWND_TOP: HWND = 0 as HWND;
#[cfg (feature = "have_ntgui")]
pub const WS_OVERLAPPEDWINDOW: DWORD = 0x00cf0000;
#[cfg (feature = "have_ntgui")]
pub const SW_SHOWNORMAL: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const SW_MAXIMIZE: c_int = 3;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOSIZE: UINT = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOZORDER: UINT = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOACTIVATE: UINT = 0x0010;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetWindowPlacement (hWnd: HWND, lpwndpl: *mut WINDOWPLACEMENT) -> BOOL;
  pub fn SetWindowPlacement (hWnd: HWND, lpwndpl: *const WINDOWPLACEMENT) -> BOOL;
  pub fn ShowWindow (hWnd: HWND, nCmdShow: c_int) -> BOOL;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn do_pending_window_change (safe: bool);
}

/* C 6909-6910's note above `w32_calc_absolute_position`, carried as C's own text.  */
/* C: /* Calculate the absolute position in frame F
   C:    from its current recorded position values and gravity.  */  */

/// C 6912-7021: `static void w32_calc_absolute_position (struct frame *f)` - C 6915's `flags`, C
/// 6920's two border widths, C 6926-6927's two zeroed records, C 6929-6945's placement probe with
/// its default arm, C 6951-6978's negative-coordinate monitor scan (`Fw32_display_monitor_attributes_list`,
/// the `CONSP` walk, `Fassoc`/`Fnth` and the two `min`s), C 6988-7016's two negative branches and C
/// 7020's flag clear.
/// C: static void
/// C: w32_calc_absolute_position (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_calc_absolute_position (f: *mut frame)
{
  let flags: c_int = (*f).size_hint_flags;                             // C 6915

  /* The sum of the widths of the frame's left and right borders, and
     the sum of the heights of the frame's top and bottom borders (in
     pixels) drawn by Windows.  */
  let mut left_right_borders_width: c_uint;                            // C 6920
  let mut top_bottom_borders_height: c_uint;

  /* Try to get the actual values of these two variables.  We compute
     the border width (height) by subtracting the width (height) of
     the frame's client area from the width (height) of the frame's
     entire window.  */
  /* C's `{ 0 }` initialisers are explicit zero fills (PORTING.md 1.5).  */
  let mut wp: WINDOWPLACEMENT = core::mem::zeroed ();                  // C 6926
  let mut client_rect: RECT = core::mem::zeroed ();                    // C 6927

  if GetWindowPlacement (FRAME_W32_WINDOW! (f), &raw mut wp) != 0       // C 6929
      && GetClientRect (FRAME_W32_WINDOW! (f), &raw mut client_rect) != 0   // C 6930
    {
      left_right_borders_width =
	(wp.rcNormalPosition.right - wp.rcNormalPosition.left) as c_uint     // C 6932-6933
	- (client_rect.right - client_rect.left) as c_uint;            // C 6934

      top_bottom_borders_height =
	(wp.rcNormalPosition.bottom - wp.rcNormalPosition.top) as c_uint   // C 6936-6937
	- (client_rect.bottom - client_rect.top) as c_uint;            // C 6938
    }
  else
    {
      /* Use sensible default values.  */
      left_right_borders_width = 8;                                    // C 6943
      top_bottom_borders_height = 32;                                  // C 6944
    }

  /* With multiple monitors, we can legitimately get negative
     coordinates (for monitors above or to the left of the primary
     monitor).  Find the display origin to ensure negative positions
     are computed correctly (Bug#21173).  */
  let mut display_left: c_int = 0;                                     // C 6951
  let mut display_top: c_int = 0;                                      // C 6952
  let p: *mut frame = FRAME_PARENT_FRAME (f);                          // C 6953

  if p.is_null () && (flags & (XNegative | YNegative)) != 0            // C 6955
    {
      let mut list: Lisp_Object;                                       // C 6957

      list = Fw32_display_monitor_attributes_list (Qnil ());           // C 6959
      while CONSP (list)                                               // C 6960
	{
	  let attributes: Lisp_Object = CAR (list);                     // C 6962
	  let mut geometry: Lisp_Object;                                // C 6963
	  let mut monitor_left: Lisp_Object;                            // C 6964
	  let mut monitor_top: Lisp_Object;

	  list = CDR (list);                                           // C 6966

	  geometry = Fassoc (Qgeometry (), attributes, Qnil ());        // C 6968
	  if !NILP (geometry)                                          // C 6969
	    {
	      monitor_left = Fnth (make_fixnum (1), geometry);          // C 6971
	      monitor_top = Fnth (make_fixnum (2), geometry);           // C 6972

	      display_left = min! (display_left, XFIXNUM (monitor_left) as c_int);   // C 6974
	      display_top = min! (display_top, XFIXNUM (monitor_top) as c_int);   // C 6975
	    }
	}
    }

  /* Treat negative positions as relative to the rightmost bottommost
     position that fits on the screen or parent frame.

     I see no need for subtracting 1 from the border widths - is there
     any on the remaining platforms?  Here these subtractions did put
     the last pixel line/column of a frame off-display when, for
     example, a (set-frame-parameter nil 'left '(- 0)) specification was
     used - martin 20017-05-05. */
  if (flags & XNegative) != 0                                          // C 6988
    {
      if !p.is_null ()                                                 // C 6990
	{
	  (*f).left_pos = (FRAME_PIXEL_WIDTH (p)                        // C 6991
			   - FRAME_PIXEL_WIDTH (f)                      // C 6992
			   + (*f).left_pos                              // C 6993
			   - left_right_borders_width as c_int);        // C 6994
	}
      else
	{
	  (*f).left_pos = (w32_display_pixel_width (FRAME_DISPLAY_INFO! (f))   // C 6996
			   + display_left                               // C 6997
			   - FRAME_PIXEL_WIDTH (f)                      // C 6998
			   + (*f).left_pos                              // C 6999
			   - left_right_borders_width as c_int);        // C 7000
	}
    }

  if (flags & YNegative) != 0                                          // C 7003
    {
      if !p.is_null ()                                                 // C 7005
	{
	  (*f).top_pos = (FRAME_PIXEL_HEIGHT (p)                       // C 7006
			  - FRAME_PIXEL_HEIGHT (f)                     // C 7007
			  + (*f).top_pos                               // C 7008
			  - top_bottom_borders_height as c_int);        // C 7009
	}
      else
	{
	  (*f).top_pos = (w32_display_pixel_height (FRAME_DISPLAY_INFO! (f))   // C 7011
			  + display_top                                // C 7012
			  - FRAME_PIXEL_HEIGHT (f)                     // C 7013
			  + (*f).top_pos                               // C 7014
			  - top_bottom_borders_height as c_int);        // C 7015
	}
    }

  /* The left_pos and top_pos are now relative to the top and left
     screen edges, so the flags should correspond.  */
  (*f).size_hint_flags &= ! (XNegative | YNegative);                   // C 7020
}

/* C 7023-7027's note above `w32_set_offset`, carried as C's own text.  */
/* C: /* CHANGE_GRAVITY is 1 when calling from Fset_frame_position,
   C:    to really change the position, and 0 when calling from
   C:    w32_make_frame_visible (in that case, XOFF and YOFF are the current
   C:    position values).  It is -1 when calling from gui_set_frame_parameters,
   C:    which means, do adjust for borders but don't change the gravity.  */  */

/// C 7029-7065: `static void w32_set_offset (struct frame *f, register int xoff, register int
/// yoff, int change_gravity)` - C 7033's two locals, C 7035-7045's `change_gravity` block (the two
/// stores, the flag clear, C 7040-7043's two conditional flag sets and C 7044's gravity), C 7046's
/// `w32_calc_absolute_position`, C 7048-7049's `block_input`/`w32_wm_set_size_hint`, C 7051-7052's
/// two locals and C 7054-7064's two `my_set_window_pos` calls.  C's two `register` keywords have
/// no Rust counterpart and the port keeps the names (PORTING.md 0.1).
/// C: static void
/// C: w32_set_offset (struct frame *f, register int xoff, register int yoff,
/// C:                 int change_gravity)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_gravity: c_int)
{
  let mut modified_top: c_int;                                         // C 7033
  let mut modified_left: c_int;

  if change_gravity > 0                                                // C 7035
    {
      (*f).top_pos = yoff;                                             // C 7037
      (*f).left_pos = xoff;                                            // C 7038
      (*f).size_hint_flags &= ! (XNegative | YNegative);               // C 7039
      if xoff < 0                                                      // C 7040
	{ (*f).size_hint_flags |= XNegative; }                         // C 7041
      if yoff < 0                                                      // C 7042
	{ (*f).size_hint_flags |= YNegative; }                         // C 7043
      (*f).win_gravity = NorthWestGravity;                             // C 7044
    }
  w32_calc_absolute_position (f);                                      // C 7046

  block_input ();                                                      // C 7048
  w32_wm_set_size_hint (f, 0 as c_long, false);                        // C 7049

  modified_left = (*f).left_pos;                                       // C 7051
  modified_top = (*f).top_pos;                                         // C 7052

  if FRAME_PARENT_FRAME (f).is_null ()                                 // C 7054
    {
      my_set_window_pos (FRAME_W32_WINDOW! (f), core::ptr::null_mut (),   // C 7055
			 modified_left, modified_top,                  // C 7056
			 0, 0,                                         // C 7057
			 SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);   // C 7058
    }
  else
    {
      my_set_window_pos (FRAME_W32_WINDOW! (f), HWND_TOP,              // C 7060
			 modified_left, modified_top,                  // C 7061
			 0, 0,                                         // C 7062
			 SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);   // C 7063
    }
  unblock_input ();                                                    // C 7064
}

/// C 7067-7146: `static void w32fullscreen_hook (struct frame *f)` - C 7070's `FRAME_VISIBLE_P`
/// guard, C 7072-7075's four locals, C 7077-7078's `block_input` and the `FULLSCREEN_WAIT` clear,
/// C 7080-7091's three placement restores, C 7093's `FRAME_PREV_FSMODE` store, C 7095-7133's five
/// fullscreen arms (the `SW_SHOWNORMAL`/`SW_MAXIMIZE` pair, the `FULLSCREEN_BOTH` arm with its
/// `w32_fullscreen_rect` + `SetWindowLong` + `SetWindowPos` + `change_frame_size`, the `else` arm
/// with the same four and the two `SWP_*` sets), C 7135-7136's store and `unblock_input` and C
/// 7138-7141's `do_pending_window_change`, with C 7144-7145's `else` writing the WAIT flag.
/// C: static void
/// C: w32fullscreen_hook (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32fullscreen_hook (f: *mut frame)
{
  if FRAME_VISIBLE_P (f)                                               // C 7070
    {
      let hwnd: HWND = FRAME_W32_WINDOW! (f);                          // C 7072
      let dwStyle: DWORD = GetWindowLong (hwnd, GWL_STYLE) as DWORD;   // C 7073
      /* C declares the `RECT` uninitialised and fills it with `w32_fullscreen_rect`
	 (PORTING.md 1.5).  */
      let mut rect: RECT = core::mem::zeroed ();                       // C 7074
      let prev_fsmode: fullscreen_type = FRAME_PREV_FSMODE! (f);       // C 7075

      block_input ();                                                  // C 7077
      want_fullscreen! (f, want_fullscreen! (f) & ! FULLSCREEN_WAIT);   // C 7078

      if FRAME_PREV_FSMODE! (f) == FULLSCREEN_NONE                     // C 7080
	{ GetWindowPlacement (hwnd, &raw mut FRAME_NORMAL_PLACEMENT! (f)); }   // C 7081

      if FRAME_PREV_FSMODE! (f) == FULLSCREEN_BOTH                     // C 7083
	{
	  if FRAME_UNDECORATED (f) == 0                                    // C 7085
	    { SetWindowLong (hwnd, GWL_STYLE, (dwStyle | WS_OVERLAPPEDWINDOW) as LONG); }   // C 7086
	  SetWindowPlacement (hwnd, &raw const FRAME_NORMAL_PLACEMENT! (f));   // C 7087
	}
      else if FRAME_PREV_FSMODE! (f) == FULLSCREEN_HEIGHT              // C 7089
	       || FRAME_PREV_FSMODE! (f) == FULLSCREEN_WIDTH           // C 7090
	{ SetWindowPlacement (hwnd, &raw const FRAME_NORMAL_PLACEMENT! (f)); }   // C 7091

      FRAME_PREV_FSMODE! (f, want_fullscreen! (f));                    // C 7093

      if want_fullscreen! (f) == FULLSCREEN_NONE                       // C 7095
	{ ShowWindow (hwnd, SW_SHOWNORMAL); }                          // C 7096
      else if want_fullscreen! (f) == FULLSCREEN_MAXIMIZED             // C 7097
	{
	  if prev_fsmode == FULLSCREEN_BOTH                            // C 7099
	      || prev_fsmode == FULLSCREEN_WIDTH                       // C 7100
	      || prev_fsmode == FULLSCREEN_HEIGHT                      // C 7101
	    /* Make window normal since otherwise the subsequent
	       maximization might fail in some cases.  */
	    { ShowWindow (hwnd, SW_SHOWNORMAL); }                      // C 7104
	  ShowWindow (hwnd, SW_MAXIMIZE);                              // C 7105
	}
      else if want_fullscreen! (f) == FULLSCREEN_BOTH                  // C 7107
	{
	  w32_fullscreen_rect
	    (hwnd, want_fullscreen! (f),                                 // C 7110
	     FRAME_NORMAL_PLACEMENT! (f).rcNormalPosition, &raw mut rect);   // C 7111
	  if FRAME_UNDECORATED (f) == 0                                    // C 7112
	    { SetWindowLong (hwnd, GWL_STYLE, (dwStyle & ! WS_OVERLAPPEDWINDOW) as LONG); }   // C 7113
	  SetWindowPos (hwnd, HWND_TOP, rect.left, rect.top,              // C 7114
			rect.right - rect.left, rect.bottom - rect.top,   // C 7115
			(SWP_NOOWNERZORDER | SWP_FRAMECHANGED) as UINT);   // C 7116
	  change_frame_size
	    (f, rect.right - rect.left, rect.bottom - rect.top,           // C 7117-7118
	     false, true, false);                                      // C 7119
	}
      else
	{
	  ShowWindow (hwnd, SW_SHOWNORMAL);                            // C 7123
	  w32_fullscreen_rect
	    (hwnd, want_fullscreen! (f),                                 // C 7125
	     FRAME_NORMAL_PLACEMENT! (f).rcNormalPosition, &raw mut rect);   // C 7126
	  SetWindowPos (hwnd, HWND_TOP, rect.left, rect.top,              // C 7127
			rect.right - rect.left, rect.bottom - rect.top, 0);   // C 7128

	  change_frame_size
	    (f, rect.right - rect.left, rect.bottom - rect.top,           // C 7130-7131
	     false, true, false);                                      // C 7132
	}

      want_fullscreen! (f, FULLSCREEN_NONE);                           // C 7135
      unblock_input ();                                                // C 7136

      if want_fullscreen! (f) == FULLSCREEN_BOTH                       // C 7138
	  || want_fullscreen! (f) == FULLSCREEN_WIDTH                  // C 7139
	  || want_fullscreen! (f) == FULLSCREEN_HEIGHT                 // C 7140
	{ do_pending_window_change (false); }                          // C 7141
    }
  else
    { want_fullscreen! (f, want_fullscreen! (f) | FULLSCREEN_WAIT); }   // C 7145
}

/* C 7148's note and `static void w32_set_window_size (struct frame *f, ...)` open the next slice.
   `w32_wm_set_size_hint` (C 7049's callee) is defined at C 7477, still uncarried.  */

/* The SDK surface these eight items read: `<winuser.h>`'s menu-bar, window-rectangle, mouse-warp
   and deferred-window-position entries with their constants, plus `MENUBARINFO` (the record
   `src/w32fns.rs` 14014 declares the same way for its own menu-bar probe).  Every name stands at
   the use site (P-88 (1)); the SDK records are UNVERIFIED, and the two `SPI_*`/`SM_CYMENUSIZE`
   values follow the records `src/w32fns.rs` 13990/13992/6986 already carry.  */
/// C: typedef struct tagMENUBARINFO { DWORD cbSize; RECT rcBar; HMENU hMenu; HWND hwndMenu;
/// C:   BOOL fBarFocused : 1; BOOL fFocused : 1; } MENUBARINFO, *PMENUBARINFO;
/// C: WINUSERAPI BOOL WINAPI GetMenuBarInfo (HWND hwnd, LONG idObject, LONG idItem,
/// C:   PMENUBARINFO pmbi);
/// C: WINUSERAPI HDC WINAPI AdjustWindowRect (LPRECT lpRect, DWORD dwStyle, BOOL bMenu);
/// C: WINUSERAPI BOOL WINAPI GetWindowRect (HWND hWnd, LPRECT lpRect);
/// C: WINUSERAPI BOOL WINAPI ClientToScreen (HWND hWnd, LPPOINT lpPoint);
/// C: WINUSERAPI BOOL WINAPI SetCursorPos (int X, int Y);
/// C: WINUSERAPI HWND WINAPI GetForegroundWindow (void);
/// C: WINUSERAPI HDWP WINAPI BeginDeferWindowPos (int nNumWindows);
/// C: WINUSERAPI HDWP WINAPI DeferWindowPos (HDWP hWinPosInfo, HWND hWnd, HWND hWndInsertAfter,
/// C:   int x, int y, int cx, int cy, UINT uFlags);
/// C: WINUSERAPI BOOL WINAPI EndDeferWindowPos (HDWP hWinPosInfo);
/// C: #define SWP_NOMOVE 0x0002 / #define SM_CYMENUSIZE 51
/// C: #define SPI_GETMOUSETRAILS 0x005E / #define SPI_SETMOUSETRAILS 0x005D
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENUBARINFO
{
  pub cbSize: DWORD,
  pub rcBar: RECT,
  pub hMenu: HMENU,
  pub hwndMenu: HWND,
  /* C's two `BOOL : 1` members share one 4-byte unit (the `src/w32fns.rs` 14014 record).  */
  pub bits: DWORD,
}
#[cfg (feature = "have_ntgui")]
pub type HDWP = HANDLE;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOMOVE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const SM_CYMENUSIZE: c_int = 51;
#[cfg (feature = "have_ntgui")]
pub const SPI_GETMOUSETRAILS: UINT = 0x005E;
#[cfg (feature = "have_ntgui")]
pub const SPI_SETMOUSETRAILS: UINT = 0x005D;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetMenuBarInfo (hwnd: HWND, idObject: LONG, idItem: LONG, pmbi: *mut MENUBARINFO) -> BOOL;
  pub fn AdjustWindowRect (lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
  pub fn GetWindowRect (hWnd: HWND, lpRect: LPRECT) -> BOOL;
  pub fn ClientToScreen (hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
  pub fn SetCursorPos (X: c_int, Y: c_int) -> BOOL;
  pub fn GetForegroundWindow () -> HWND;
  pub fn BeginDeferWindowPos (nNumWindows: c_int) -> HDWP;
  pub fn DeferWindowPos (hWinPosInfo: HDWP, hWnd: HWND, hWndInsertAfter: HWND, x: c_int, y: c_int,
			 cx: c_int, cy: c_int, uFlags: UINT) -> HDWP;
  pub fn EndDeferWindowPos (hWinPosInfo: HDWP) -> BOOL;
}

/* C 7148-7151's note above `w32_set_window_size`, carried as C's own text.  */
/* C: /* Change the size of frame F's Windows window to WIDTH and HEIGHT
   C:    pixels.  If CHANGE_GRAVITY, change to top-left-corner window gravity
   C:    for this size change and subsequent size changes.  Otherwise leave
   C:    the window gravity unchanged.  */  */

/// C 7152-7256: `static void w32_set_window_size (struct frame *f, bool change_gravity, int
/// width, int height)` - C 7156-7159's four locals, C 7161's `block_input`, C 7166-7169's
/// `GetMenuBarInfo` probe, C 7171-7187's wrapped-menu-bar adjustment, C 7189-7194's gravity,
/// size-hint and `RECT` fill, C 7196's `AdjustWindowRect`, C 7198-7222's fullscreen-parameter
/// block with `GetWindowRect`, C 7224-7251's resize block and C 7253-7255's `unblock_input` +
/// `do_pending_window_change`.  C's unused `change_gravity` parameter has no Rust counterpart use
/// and keeps C's name.
/// C: static void
/// C: w32_set_window_size (struct frame *f, bool change_gravity,
/// C: 			 int width, int height)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_window_size (f: *mut frame, change_gravity: bool, mut width: c_int,
			       mut height: c_int)
{
  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());   // C 7156
  /* C declares these uninitialised and fills them below (PORTING.md 1.5).  */
  let mut rect: RECT = core::mem::zeroed ();                           // C 7157
  let mut info: MENUBARINFO = core::mem::zeroed ();                    // C 7158
  let mut menu_bar_height: c_int;                                      // C 7159

  block_input ();                                                      // C 7161

  /* Get the height of the menu bar here.  It's used below to detect
     whether the menu bar is wrapped.  It's also used to specify the
     third argument for AdjustWindowRect.  See bug#22105.  */
  info.cbSize = core::mem::size_of::<MENUBARINFO> () as DWORD;         // C 7166
  info.rcBar.bottom = 0;                                               // C 7167
  info.rcBar.top = 0;                                                  // C 7167
  GetMenuBarInfo (FRAME_W32_WINDOW! (f), 0xFFFFFFFD, 0, &raw mut info);   // C 7168
  menu_bar_height = info.rcBar.bottom - info.rcBar.top;                // C 7169

  if w32_add_wrapped_menu_bar_lines                                    // C 7171
    {
      /* When the menu bar wraps sending a SetWindowPos shrinks the
	 height of the frame then the wrapped menu bar lines are not
	 accounted for (Bug#15174 and Bug#18720).  Here we add these
	 extra lines to the frame height.  */
      let mut default_menu_bar_height: c_int;                          // C 7177

      /* Why is (apparently) SM_CYMENUSIZE needed here instead of
	 SM_CYMENU ??  */
      default_menu_bar_height = GetSystemMetrics (SM_CYMENUSIZE);      // C 7181

      if (default_menu_bar_height > 0)                                 // C 7183
	  && (menu_bar_height > default_menu_bar_height)               // C 7184
	  && ((menu_bar_height % default_menu_bar_height) == 0)        // C 7185
	{ height = height + menu_bar_height - default_menu_bar_height; }   // C 7186
    }

  (*f).win_gravity = NorthWestGravity;                                 // C 7189
  w32_wm_set_size_hint (f, 0 as c_long, false);                        // C 7190

  rect.top = 0;                                                        // C 7192
  rect.left = 0;                                                       // C 7192
  rect.right = width;                                                  // C 7193
  rect.bottom = height;                                                // C 7194

  AdjustWindowRect (&raw mut rect, (*((*f).output_data.w32 as *mut w32_output)).dwStyle,
		    (menu_bar_height > 0) as BOOL);                    // C 7196

  if !(frame_bits! (after_make_frame, f))                              // C 7198
      && (want_fullscreen! (f) & FULLSCREEN_WAIT) == 0                 // C 7199
      && FRAME_VISIBLE_P (f)                                           // C 7200
    {
      /* C declares the `RECT` uninitialised and fills it with `GetWindowRect` (PORTING.md 1.5).  */
      let mut window_rect: RECT = core::mem::zeroed ();                // C 7202

      GetWindowRect (FRAME_W32_WINDOW! (f), &raw mut window_rect);      // C 7204

      if EQ (fullscreen, Qmaximized ())                                // C 7206
	  || EQ (fullscreen, Qfullboth ())                             // C 7207
	  || EQ (fullscreen, Qfullwidth ())                            // C 7208
	{
	  rect.left = window_rect.left;                                // C 7210
	  rect.right = window_rect.right;                              // C 7211
	  width = -1;                                                  // C 7212
	}
      if EQ (fullscreen, Qmaximized ())                                // C 7214
	  || EQ (fullscreen, Qfullboth ())                             // C 7215
	  || EQ (fullscreen, Qfullheight ())                           // C 7216
	{
	  rect.top = window_rect.top;                                  // C 7218
	  rect.bottom = window_rect.bottom;                            // C 7219
	  height = -1;                                                 // C 7220
	}
    }

  if width > 0 || height > 0                                           // C 7224
    {
      if FRAME_PARENT_FRAME (f).is_null ()                             // C 7226
	{
	  my_set_window_pos (FRAME_W32_WINDOW! (f), core::ptr::null_mut (),   // C 7227
			     0, 0,                                     // C 7228
			     rect.right - rect.left,                   // C 7229
			     rect.bottom - rect.top,                   // C 7230
			     (SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE) as UINT);   // C 7231
	}
      else
	{
	  my_set_window_pos (FRAME_W32_WINDOW! (f), HWND_TOP,              // C 7233
			     0, 0,                                     // C 7234
			     rect.right - rect.left,                   // C 7235
			     rect.bottom - rect.top,                   // C 7236
			     (SWP_NOMOVE | SWP_NOACTIVATE) as UINT);   // C 7237
	}

      change_frame_size (f, width, height, false, true, false);        // C 7239
      SET_FRAME_GARBAGED! (f);                                         // C 7240

      /* If cursor was outside the new size, mark it as off.  */
      mark_window_cursors_off (XWINDOW ((*f).root_window));            // C 7243

      /* Clear out any recollection of where the mouse highlighting was,
	 since it might be in a place that's outside the new frame size.
	 Actually checking whether it is outside is a pain in the neck,
	 so don't try--just let the highlighting be done afresh with new
	 size.  */
      cancel_mouse_face (f);                                           // C 7250
    }

  unblock_input ();                                                    // C 7253

  do_pending_window_change (false);                                    // C 7255
}

/* C 7258-7260's note above `w32_set_window_size_and_position`, carried as C's own text.  */
/* C: /* Change the size of frame F's Windows window to WIDTH and HEIGHT
   C:    pixels and its position to those stored in f->left_pos and
   C:    f->top_pos.  */  */

/// C 7261-7335: `static void w32_set_window_size_and_position (struct frame *f, int width, int
/// height)` - the same probe and wrapped-menu-bar block as the function above, C 7296-7304's
/// gravity/size-hint/`RECT` fill and `AdjustWindowRect`, C 7306-7317's two `my_set_window_pos`
/// calls and C 7319-7334's tail.
/// C: static void
/// C: w32_set_window_size_and_position (struct frame *f, int width, int height)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_window_size_and_position (f: *mut frame, mut width: c_int, mut height: c_int)
{
  /* C declares these uninitialised and fills them below (PORTING.md 1.5).  */
  let mut rect: RECT = core::mem::zeroed ();                           // C 7264
  let mut info: MENUBARINFO = core::mem::zeroed ();                    // C 7265
  let mut menu_bar_height: c_int;                                      // C 7266

  block_input ();                                                      // C 7268

  /* Get the height of the menu bar here.  It's used below to detect
     whether the menu bar is wrapped.  It's also used to specify the
     third argument for AdjustWindowRect.  See bug#22105.  */
  info.cbSize = core::mem::size_of::<MENUBARINFO> () as DWORD;         // C 7273
  info.rcBar.bottom = 0;                                               // C 7274
  info.rcBar.top = 0;                                                  // C 7274
  GetMenuBarInfo (FRAME_W32_WINDOW! (f), 0xFFFFFFFD, 0, &raw mut info);   // C 7275
  menu_bar_height = info.rcBar.bottom - info.rcBar.top;                // C 7276

  if w32_add_wrapped_menu_bar_lines                                    // C 7278
    {
      /* When the menu bar wraps sending a SetWindowPos shrinks the
	 height of the frame then the wrapped menu bar lines are not
	 accounted for (Bug#15174 and Bug#18720).  Here we add these
	 extra lines to the frame height.  */
      let mut default_menu_bar_height: c_int;                          // C 7284

      /* Why is (apparently) SM_CYMENUSIZE needed here instead of
	 SM_CYMENU ??  */
      default_menu_bar_height = GetSystemMetrics (SM_CYMENUSIZE);      // C 7288

      if (default_menu_bar_height > 0)                                 // C 7290
	  && (menu_bar_height > default_menu_bar_height)               // C 7291
	  && ((menu_bar_height % default_menu_bar_height) == 0)        // C 7292
	{ height = height + menu_bar_height - default_menu_bar_height; }   // C 7293
    }

  (*f).win_gravity = NorthWestGravity;                                 // C 7296
  w32_wm_set_size_hint (f, 0 as c_long, false);                        // C 7297

  rect.left = (*f).left_pos;                                           // C 7299
  rect.top = (*f).top_pos;                                             // C 7300
  rect.right = rect.left + width;                                      // C 7301
  rect.bottom = rect.top + height;                                     // C 7302

  AdjustWindowRect (&raw mut rect, (*((*f).output_data.w32 as *mut w32_output)).dwStyle,
		    (menu_bar_height > 0) as BOOL);                    // C 7304

  if FRAME_PARENT_FRAME (f).is_null ()                                 // C 7306
    {
      my_set_window_pos (FRAME_W32_WINDOW! (f), core::ptr::null_mut (),   // C 7307
			 (*f).left_pos, (*f).top_pos,                  // C 7308
			 rect.right - rect.left,                       // C 7309
			 rect.bottom - rect.top,                       // C 7310
			 (SWP_NOZORDER | SWP_NOACTIVATE) as UINT);     // C 7311
    }
  else
    {
      my_set_window_pos (FRAME_W32_WINDOW! (f), HWND_TOP,              // C 7313
			 (*f).left_pos, (*f).top_pos,                  // C 7314
			 rect.right - rect.left,                       // C 7315
			 rect.bottom - rect.top,                       // C 7316
			 SWP_NOACTIVATE as UINT);                      // C 7317
    }

  change_frame_size (f, width, height, false, true, false);            // C 7319
  SET_FRAME_GARBAGED! (f);                                             // C 7320

  /* If cursor was outside the new size, mark it as off.  */
  mark_window_cursors_off (XWINDOW ((*f).root_window));                // C 7323

  /* Clear out any recollection of where the mouse highlighting was,
     since it might be in a place that's outside the new frame size.
     Actually checking whether it is outside is a pain in the neck,
     so don't try--just let the highlighting be done afresh with new
     size.  */
  cancel_mouse_face (f);                                               // C 7330

  unblock_input ();                                                    // C 7332

  do_pending_window_change (false);                                    // C 7334
}

/* C 7337's "Mouse warping." banner, carried as C's own text at its position.  */
/* C: /* Mouse warping.  */  */

/// C 7339-7365: `void frame_set_mouse_pixel_position (struct frame *f, int pix_x, int pix_y)` -
/// C's one non-`static` definition of this slice, so the port's is `#[unsafe(no_mangle)] pub
/// unsafe extern "C" fn` (P-52).  C 7342-7345's four locals, C 7347's `block_input`, C 7349-7352's
/// client-rect and `ClientToScreen` pair, C 7357-7362's mouse-trail block and C 7364's
/// `unblock_input`.
/// C: void
/// C: frame_set_mouse_pixel_position (struct frame *f, int pix_x, int pix_y)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_mouse_pixel_position (f: *mut frame, pix_x: c_int, pix_y: c_int)
{
  let mut trail_num: UINT = 0;                                         // C 7342
  let mut ret: BOOL = FALSE;                                           // C 7343
  /* C declares these uninitialised and fills them below (PORTING.md 1.5).  */
  let mut rect: RECT = core::mem::zeroed ();                           // C 7344
  let mut pt: POINT = core::mem::zeroed ();                            // C 7345

  block_input ();                                                      // C 7347

  GetClientRect (FRAME_W32_WINDOW! (f), &raw mut rect);                // C 7349
  pt.x = rect.left + pix_x;                                            // C 7350
  pt.y = rect.top + pix_y;                                             // C 7351
  ClientToScreen (FRAME_W32_WINDOW! (f), &raw mut pt);                 // C 7352

  /* When "mouse trails" are in effect, moving the mouse cursor
     sometimes leaves behind an annoying "ghost" of the pointer.
     Avoid that by momentarily switching off mouse trails.  */
  if os_subtype == OS_SUBTYPE_NT                                    // C 7357
      && w32_major_version + w32_minor_version >= 6                 // C 7358
    {
      ret = SystemParametersInfo (SPI_GETMOUSETRAILS, 0, &raw mut trail_num as *mut c_void, 0);   // C 7359
    }
  SetCursorPos (pt.x, pt.y);                                           // C 7360
  if ret != 0                                                          // C 7361
    {
      SystemParametersInfo (SPI_SETMOUSETRAILS, trail_num, core::ptr::null_mut (), 0);   // C 7362
    }

  unblock_input ();                                                    // C 7364
}

/// C 7367-7379: `static Lisp_Object w32_get_focus_frame (struct frame *f)` - C 7370's local, C
/// 7372's `dpyinfo->w32_focus_frame` read, C 7374-7375's null guard and C 7377-7378's
/// `XSETFRAME`/`return`.
/// C: static Lisp_Object
/// C: w32_get_focus_frame (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_get_focus_frame (f: *mut frame) -> Lisp_Object
{
  let mut lisp_focus: Lisp_Object;                                     // C 7370

  let focus: *mut frame = (*FRAME_DISPLAY_INFO! (f)).w32_focus_frame;   // C 7372

  if focus.is_null ()                                                  // C 7374
    { return Qnil (); }                                                // C 7375

  XSETFRAME! (lisp_focus, focus);                                      // C 7377
  return lisp_focus;                                                   // C 7378
}

/* C 7381-7386: the "Focus shifting, raising and lowering." banner and the note above
   `w32_focus_frame`, carried as C's own text at their positions.  */
/* C: /* Focus shifting, raising and lowering.  */  */

   /* The NOACTIVATE argument has no effect on Windows.  According to the
      Windows API: An application cannot activate an inactive window
      without also bringing it to the top of the Z order.  */

/// C 7388-7405: `static void w32_focus_frame (struct frame *f, bool noactivate)` - C 7391-7393's
/// `#if 0` declaration (carried as C's text), C 7396's `block_input`, C 7397-7402's `#if 0` block
/// (also carried) and the live `my_set_foreground_window` of C 7403 with C 7404's `unblock_input`.
/// C: static void
/// C: w32_focus_frame (struct frame *f, bool noactivate)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_focus_frame (f: *mut frame, noactivate: bool)
{
  /* C 7391-7393 is `#if 0`:

  #if 0
    struct w32_display_info *dpyinfo = &one_w32_display_info;
  #endif
  */

  /* Give input focus to frame.  */
  block_input ();                                                      // C 7396
  /* C 7397-7402 is `#if 0`, carried as C's own text:

  #if 0
    /* Try not to change its Z-order if possible.  */
    if (w32_window_to_frame (dpyinfo, GetForegroundWindow ()))
      my_set_focus (f, FRAME_W32_WINDOW (f));
    else
  #endif
  */
  my_set_foreground_window (FRAME_W32_WINDOW! (f));                    // C 7403
  unblock_input ();                                                    // C 7404
}

/* C 7407's note above `w32_raise_frame`, carried as C's own text.  */
/* C: /* Raise frame F.  */  */

/// C 7408-7467: `static void w32_raise_frame (struct frame *f)` - C 7411's `block_input`, C
/// 7413-7430's long note (carried), C 7432-7460's `Vw32_grab_focus_on_raise` arm (the three-step
/// `BeginDeferWindowPos`/`DeferWindowPos`/`EndDeferWindowPos` chain), C 7461-7464's `else` with
/// `my_bring_window_to_top` and C 7466's `unblock_input`.  **This is the definition of slice 16's
/// second owed call** (C 6405 called it).
/// C: static void
/// C: w32_raise_frame (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_raise_frame (f: *mut frame)
{
  block_input ();                                                      // C 7411

  /* Strictly speaking, raise-frame should only change the frame's Z
     order, leaving input focus unchanged.  This is reasonable behavior
     on X where the usual policy is point-to-focus.  However, this
     behavior would be very odd on Windows where the usual policy is
     click-to-focus.

     On X, if the mouse happens to be over the raised frame, it gets
     input focus anyway (so the window with focus will never be
     completely obscured) - if not, then just moving the mouse over it
     is sufficient to give it focus.  On Windows, the user must actually
     click on the frame (preferably the title bar so as not to move
     point), which is more awkward.  Also, no other Windows program
     raises a window to the top but leaves another window (possibly now
     completely obscured) with input focus.

     Because there is a system setting on Windows that allows the user
     to choose the point to focus policy, we make the strict semantics
     optional, but by default we grab focus when raising.  */

  if NILP (Vw32_grab_focus_on_raise)                                // C 7432
    {
      /* The obvious call to my_set_window_pos doesn't work if Emacs is
	 not already the foreground application: the frame is raised
	 above all other frames belonging to us, but not above the
	 current top window.  To achieve that, we have to resort to this
	 more cumbersome method.  */

      let mut handle: HDWP = BeginDeferWindowPos (2);                  // C 7440
      if !handle.is_null ()                                            // C 7441
	{
	  handle = DeferWindowPos (handle,                             // C 7443
				   FRAME_W32_WINDOW! (f),              // C 7444
				   HWND_TOP,                           // C 7445
				   0, 0, 0, 0,                          // C 7446
				   (SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE) as UINT);   // C 7447
	  if !handle.is_null ()                                        // C 7448
	    {
	      handle = DeferWindowPos (handle,                         // C 7450
				       GetForegroundWindow (),         // C 7451
				       FRAME_W32_WINDOW! (f),          // C 7452
				       0, 0, 0, 0,                      // C 7453
				       (SWP_NOSIZE | SWP_NOMOVE |      // C 7454
					SWP_NOACTIVATE) as UINT);      // C 7455
	      if !handle.is_null ()                                    // C 7456
		{ EndDeferWindowPos (handle); }                        // C 7457
	    }
	}
    }
  else
    {
      my_bring_window_to_top (FRAME_W32_WINDOW! (f));                  // C 7463
    }

  unblock_input ();                                                    // C 7466
}

/* C 7469's note above `w32_lower_frame`, carried as C's own text.  */
/* C: /* Lower frame F.  */  */

/// C 7470-7479: `static void w32_lower_frame (struct frame *f)` - the one `my_set_window_pos` with
/// `HWND_BOTTOM`.
/// C: static void
/// C: w32_lower_frame (struct frame *f)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_lower_frame (f: *mut frame)
{
  block_input ();                                                      // C 7473
  my_set_window_pos (FRAME_W32_WINDOW! (f),                            // C 7474
		     HWND_BOTTOM,                                      // C 7475
		     0, 0, 0, 0,                                       // C 7476
		     (SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE) as UINT);   // C 7477
  unblock_input ();                                                    // C 7478
}

/// C 7481-7491: `static void w32_frame_raise_lower (struct frame *f, bool raise_flag)` - the
/// `FRAME_W32_P` guard and the two calls.
/// C: static void
/// C: w32_frame_raise_lower (struct frame *f, bool raise_flag)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_raise_lower (f: *mut frame, raise_flag: bool)
{
  if !FRAME_W32_P (f)                                                  // C 7484
    { return; }                                                        // C 7485

  if raise_flag                                                        // C 7487
    { w32_raise_frame (f); }                                           // C 7488
  else
    { w32_lower_frame (f); }                                           // C 7490
}

/* C 7493's "Change of visibility." banner, C 7495-7501's note and C 7503-7504's
   `void w32_make_frame_visible (struct frame *f)` open the next slice.  */
