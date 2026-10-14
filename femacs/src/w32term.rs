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
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK)
      | (($v) as u32 & INPUT_EVENT_KIND_MASK); };
}

#[cfg (feature = "have_ntgui")]
macro_rules! frame_bits {
  (auto_raise, $f:expr) => { (((*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT) & 1) != 0 };
  (auto_lower, $f:expr) => { (((*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT) & 1) != 0 };
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

/* C 3631-3632's `#if HAVE_W32NOTIFY` and C 3634's "File event notifications (see w32notify.c)."
   comment open the next slice, with `w32_lispy_file_action` (C 3636-3670).  */
