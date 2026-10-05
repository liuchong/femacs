// femacs: port of src/w32xfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the
// C `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32xfns.o`
// joins `W32_OBJ` only in `configure.ac` 3051 - the non-Cygwin arm of the split inside the
// `HAVE_W32` test (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` - and
// `docs/baseline/frozen-build-sources.txt` lists no `w32xfns.o` for this target, so
// `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the ledger
// `docs/ports/w32xfns.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*
//     (P-86 ⑧: with the includes outside the gate the frozen compile stops producing its empty
//     object).  It is `WINDOWSNT`, not `HAVE_NTGUI`: `HAVE_NTGUI` is on for *both* the Cygwin and
//     the native Windows GUI build (3024), while `w32xfns.o` is added to `W32_OBJ` only in the
//     non-Cygwin arm (3051) - the build whose `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.
//     The units of that same arm (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`,
//     `src/w32console.rs`) already landed with this gate, so this one does too;
//   * C 26-38, 49-51, 81-83 and 433-438's `#ifdef WINDOWSNT` is the *same* condition as the file
//     gate, so those four blocks carry `windowsnt` and nothing more; C 26-38's `#else` does not
//     exist in C, and the two other `#ifdef`s are one-liners with no `#else` either;
//   * C 31's `#ifndef MINGW_W64` is `not (feature = "mingw_w64")` - the feature names the
//     non-frozen side (P-51), the `src/w32fns.rs` 151 reading of the same guard;
//   * C 66-79 and 110-125's `#if HAVE_W32NOTIFY` is `feature = "have_w32notify"`; the macro is
//     `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so C's arm is dead in the frozen
//     configuration, and C has no `#else` for either (its own text is carried as the arm - P-51);
//   * C 364-368's `#ifdef CYGWIN` is `feature = "cygwin"` (`undef` in the same table); C's
//     `#else` does not exist, and the arm is the `PostThreadMessage` wakeup below.
//
// **Acquisition note - this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wingdi.h`), so there is **no
// `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
// measurement: every record, every value and every prototype of the `<windows.h>` stand-in section
// below is `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are
// owed.  What the round does carry is the frozen `build-tu` reading, the machine-checked coverage
// table, the eight `#if` verdicts, a per-item C citation and the per-file error census of the
// `windowsnt` arm.
//
// `docs/ports/w32xfns.md` carries the C line ranges, the acceptance readings, every deviation (the
// `<windows.h>` stand-in surface, the two cast sites the UINT/`c_int` boundary needs, the twelve
// mirror rows this unit's definitions collide with) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-18 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's own
   first line is the revision line above.  C 19 is the blank line under it.  */

include!("include/config.rs");                  // C 20: `#include <config.h>`
/* C 20's AH_BOTTOM: `#include <conf_post.h>` (`DebPrint`, C 78's use).  */
#[cfg (feature = "windowsnt")]
include!("include/conf_post.rs");

/* C 21-24: the system headers.  None of them is in the build the frozen configuration reaches, so
   the whole graph sits under the file gate (P-86 ⑧).  C 23's `<windows.h>` and C 24's
   `<windowsx.h>` have no file in the reference tree, so this file's own names for them stand in
   the two sections below (P-88 ①: the declaration site is C's own `#include` line, and no file is
   added under `femacs/src/include/`).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/signal.rs");           // C 21: `#include <signal.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");            // C 22: `#include <stdio.h>`
// C 23: `#include <windows.h>`
// C 24: `#include <windowsx.h>`

/* ----------------------------------------------------------------------------------------------
   C 23: `#include <windows.h>`.

   The names this file takes from it stand here, each with the SDK's own text above it.  The
   spellings follow the units of this same arm (`src/w32term.rs` 150-300, `src/w32fns.rs` 226-350,
   `src/w32inevt.rs` 120-300) so that a Windows translation unit sees one set of names: `DWORD` is
   the 32-bit `u32` the Windows ABI fixes, `LONG` is `i32`, `BOOL` is C's `int`, and every handle
   is `void *` at the ABI, which the port spells `*mut c_void`.

   **No measurement was possible** (see the file note): every scalar, record, constant and
   prototype below is **UNVERIFIED** - its size, offsets and values are the documented ones and the
   layout assertions are owed (the ledger's `## Owed`).  The names `include/w32term.rs` itself
   declares (`W32Msg`, `w32_output`, `notifications_set`, `CRITICAL_SECTION`'s two callers …) are
   not repeated here; a second declaration of one name in one crate is `E0428`.
   ---------------------------------------------------------------------------------------------- */

/// C 23's `<windows.h>`: the scalar typedefs (`<windef.h>`/`<winnt.h>`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned long DWORD;
/// C: typedef long LONG; typedef unsigned long ULONG; typedef unsigned int UINT; typedef int BOOL;
/// C: typedef short SHORT; typedef unsigned short USHORT;
#[cfg (feature = "windowsnt")]
pub type BYTE = u8;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub type LONG = i32;
#[cfg (feature = "windowsnt")]
pub type ULONG = u32;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type SHORT = c_short;
#[cfg (feature = "windowsnt")]
pub type USHORT = c_ushort;

/// C 23's handles, each `void *` at the ABI.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *HDC; typedef void *HBITMAP;
/// C: typedef void *HGDIOBJ; typedef void *HMENU; typedef void *HCURSOR; typedef void *HPALETTE;
/// C: typedef void *HINSTANCE;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HWND = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HDC = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HBITMAP = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMENU = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HCURSOR = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HPALETTE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HINSTANCE = *mut c_void;

/// C 23's three derived scalars and pointer typedefs.
/// C: typedef DWORD COLORREF; typedef BYTE *LPBYTE; typedef RECT *LPRECT;
/// C: typedef UINT_PTR WPARAM; typedef LONG_PTR LPARAM; typedef LONG_PTR LRESULT;
#[cfg (feature = "windowsnt")]
pub type COLORREF = DWORD;
#[cfg (feature = "windowsnt")]
pub type LPBYTE = *mut BYTE;
#[cfg (feature = "windowsnt")]
pub type LPRECT = *mut RECT;
#[cfg (feature = "windowsnt")]
pub type WPARAM = usize;
#[cfg (feature = "windowsnt")]
pub type LPARAM = isize;
#[cfg (feature = "windowsnt")]
pub type LRESULT = isize;
/// C 23's `<windows.h>` also supplies the two scalars `include/w32term.rs`'s own record and
/// function-pointer aliases name (`AppendMenuW_Proc`'s `UINT_PTR`/`LPCWSTR`, C 872-873), so the
/// stand-in carries them (the `src/w32fns.rs` 226-262 spellings of the same pair).
/// C: typedef unsigned long long UINT_PTR; typedef const wchar_t *LPCWSTR;
#[cfg (feature = "windowsnt")]
pub type UINT_PTR = usize;
#[cfg (feature = "windowsnt")]
pub type LPCWSTR = *const c_ushort;

/// C 23's two value macros (`<windef.h>`'s booleans), carried with C's values.
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;

/// C 23's `WINAPI` is the `__stdcall` convention, which Rust spells `extern "system"`; the note
/// stays on the declarations below that use it.
/// C: #define WINAPI __stdcall

/// C 23's `POINT` and `RECT` (`<windef.h>`), spelled from the members this file reads.
/// C: typedef struct tagPOINT { LONG x; LONG y; } POINT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINT
{
  pub x: LONG,
  pub y: LONG,
}
/// C: typedef struct tagRECT { LONG left; LONG top; LONG right; LONG bottom; } RECT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RECT
{
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}

/// C 23's `MSG` (`<winuser.h>`) - `w32term.h` 736-741's `W32Msg` embeds it **by value** and this
/// file's queue nodes hold `W32Msg`s, so the record is spelled here (the `include/w32term.rs` note
/// leaves `MSG` to the unit that supplies `<windows.h>`).
/// C: typedef struct tagMSG { HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam; DWORD time;
/// C: POINT pt; ... } MSG;
#[cfg (feature = "windowsnt")]
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

/// C 23's `PALETTEENTRY` (`<wingdi.h>`) and `WINDOWPLACEMENT` (`<winuser.h>`), both members of
/// `include/w32term.rs`'s records (`w32_palette_entry.entry`, `w32_output.normal_placement`).
/// C: typedef struct tagPALETTEENTRY { BYTE peRed; BYTE peGreen; BYTE peBlue; BYTE peFlags; }
/// C: PALETTEENTRY;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PALETTEENTRY
{
  pub peRed: BYTE,
  pub peGreen: BYTE,
  pub peBlue: BYTE,
  pub peFlags: BYTE,
}
/// C: typedef struct tagWINDOWPLACEMENT { UINT length; UINT flags; UINT showCmd; POINT
/// C: ptMinPosition; POINT ptMaxPosition; RECT rcNormalPosition; } WINDOWPLACEMENT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOWPLACEMENT
{
  pub length: UINT,
  pub flags: UINT,
  pub showCmd: UINT,
  pub ptMinPosition: POINT,
  pub ptMaxPosition: POINT,
  pub rcNormalPosition: RECT,
}

/// C 23's `CRITICAL_SECTION` (`<synchapi.h>`'s `_RTL_CRITICAL_SECTION`), the record `C 47`'s
/// global and the two `enter_crit`/`leave_crit` calls name.
/// C: typedef struct _RTL_CRITICAL_SECTION { PRTL_CRITICAL_SECTION_DEBUG DebugInfo; LONG
/// C: LockCount; LONG RecursionCount; HANDLE OwningThread; HANDLE LockSemaphore; ULONG_PTR
/// C: SpinCount; } CRITICAL_SECTION, *PCRITICAL_SECTION, *LPCRITICAL_SECTION;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION
{
  pub DebugInfo: *mut c_void,
  pub LockCount: LONG,
  pub RecursionCount: LONG,
  pub OwningThread: HANDLE,
  pub LockSemaphore: HANDLE,
  pub SpinCount: usize,
}
#[cfg (feature = "windowsnt")]
pub type LPCRITICAL_SECTION = *mut CRITICAL_SECTION;

/// C 23's `<windows.h>` constants this file names: `<winbase.h>`'s `INFINITE`/`GPTR`,
/// `<winuser.h>`'s `PM_REMOVE`/`WM_USER`/`WM_MOUSELAST`/`WM_PAINT` and `<wingdi.h>`'s
/// `GDI_ERROR`.  `WM_USER` and `WM_MOUSELAST` are *not* this file's own names: the
/// `include/w32term.rs` constants `WM_EMACS_START` and `WM_MOUSEWHEEL` read them, and no reachable
/// mirror declares them (the `src/w32menu.rs` 245 reading of `WM_USER`), so the unit that includes
/// that mirror supplies them.
/// C: #define INFINITE 0xFFFFFFFF
/// C: #define GPTR 0x0040
/// C: #define PM_REMOVE 0x0001
/// C: #define WM_USER 0x0400
/// C: #define WM_MOUSELAST 0x020D
/// C: #define WM_PAINT 0x000F
/// C: #define GDI_ERROR (0xFFFFFFFFL)
#[cfg (feature = "windowsnt")]
pub const INFINITE: DWORD = 0xFFFFFFFF;
#[cfg (feature = "windowsnt")]
pub const GPTR: UINT = 0x0040;
#[cfg (feature = "windowsnt")]
pub const PM_REMOVE: UINT = 0x0001;
#[cfg (feature = "windowsnt")]
pub const WM_USER: c_int = 0x0400;
#[cfg (feature = "windowsnt")]
pub const WM_MOUSELAST: c_int = 0x020D;
/// `WM_PAINT` is compared with `MSG.message` (a `UINT`) at C 303/312, so the port's type is that
/// field's; `WM_USER`/`WM_MOUSELAST` are `c_int` because `include/w32term.rs`'s own constants add
/// to them in that type.
#[cfg (feature = "windowsnt")]
pub const WM_PAINT: UINT = 0x000F;
/// C's `GDI_ERROR` is `(0xFFFFFFFFL)`; `RealizePalette` returns `UINT` and C's comparison
/// promotes both sides, so the port's type is the callee's return type.
#[cfg (feature = "windowsnt")]
pub const GDI_ERROR: UINT = 0xFFFFFFFF;

/// C 23's `<windows.h>` entry points this file calls: `<synchapi.h>`'s critical-section pair and
/// the `enter_crit`/`leave_crit` pair the `include/w32term.rs` 1028-1044 macros expand to,
/// `<synchapi.h>`'s event entries, `<winbase.h>`'s `WaitForSingleObject`, and `<winuser.h>`'s
/// message-loop entries.  C's `WINAPI` is the block's ABI; each declaration keeps C's own
/// prototype in a `/// C:` line above it.
/// C: void WINAPI InitializeCriticalSection (LPCRITICAL_SECTION);
/// C: void WINAPI DeleteCriticalSection (LPCRITICAL_SECTION);
/// C: void WINAPI EnterCriticalSection (LPCRITICAL_SECTION);
/// C: void WINAPI LeaveCriticalSection (LPCRITICAL_SECTION);
/// C: HANDLE WINAPI CreateEventA (LPSECURITY_ATTRIBUTES, BOOL, BOOL, LPCSTR);
/// C: BOOL WINAPI SetEvent (HANDLE); BOOL WINAPI ResetEvent (HANDLE);
/// C: BOOL WINAPI PulseEvent (HANDLE); BOOL WINAPI CloseHandle (HANDLE);
/// C: DWORD WINAPI WaitForSingleObject (HANDLE, DWORD);
/// C: BOOL WINAPI PeekMessageA (LPMSG, HWND, UINT, UINT, UINT);
/// C: BOOL WINAPI TranslateMessage (const MSG *); LRESULT WINAPI DispatchMessageA (const MSG *);
/// C: BOOL WINAPI PostThreadMessageA (DWORD, UINT, WPARAM, LPARAM);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn InitializeCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn DeleteCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn EnterCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn LeaveCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  /// C's `LPSECURITY_ATTRIBUTES` and `LPCSTR` are pointer types this tree has no mirror for and
  /// every call passes `NULL`, so the two parameters are P-72 rule 3's untyped pointer.
  pub fn CreateEvent (attributes: *mut c_void, manual_reset: BOOL, initial_state: BOOL,
                      name: *mut c_void) -> HANDLE;
  pub fn SetEvent (event: HANDLE) -> BOOL;
  pub fn ResetEvent (event: HANDLE) -> BOOL;
  pub fn PulseEvent (event: HANDLE) -> BOOL;
  pub fn CloseHandle (object: HANDLE) -> BOOL;
  pub fn WaitForSingleObject (handle: HANDLE, milliseconds: DWORD) -> DWORD;
  pub fn PeekMessage (msg: *mut MSG, hwnd: HWND, filter_min: UINT, filter_max: UINT,
                      remove_msg: UINT) -> BOOL;
  pub fn TranslateMessage (msg: *const MSG) -> BOOL;
  pub fn DispatchMessage (msg: *const MSG) -> LRESULT;
  pub fn PostThreadMessage (thread_id: DWORD, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> BOOL;
}

/// C 23's `<windows.h>` graphics entries this file calls: `<wingdi.h>`'s DC/palette pair,
/// `<wingdi.h>`'s bitmap/DC pair, `<wingdi.h>`'s object pair, `UnionRect`/`SetRectEmpty` and
/// `<wingdi.h>`'s `GDI_ERROR` producer `RealizePalette`.
/// C: HDC WINAPI GetDC (HWND); int WINAPI ReleaseDC (HWND, HDC);
/// C: HBITMAP WINAPI CreateCompatibleBitmap (HDC, int, int); HDC WINAPI CreateCompatibleDC (HDC);
/// C: BOOL WINAPI DeleteObject (HGDIOBJ); HGDIOBJ WINAPI SelectObject (HDC, HGDIOBJ);
/// C: HPALETTE WINAPI SelectPalette (HDC, HPALETTE, BOOL); UINT WINAPI RealizePalette (HDC);
/// C: BOOL WINAPI UnionRect (LPRECT, const RECT *, const RECT *);
/// C: BOOL WINAPI SetRectEmpty (LPRECT);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetDC (hwnd: HWND) -> HDC;
  pub fn ReleaseDC (hwnd: HWND, hdc: HDC) -> c_int;
  pub fn CreateCompatibleBitmap (hdc: HDC, cx: c_int, cy: c_int) -> HBITMAP;
  pub fn CreateCompatibleDC (hdc: HDC) -> HDC;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn SelectPalette (hdc: HDC, hpal: HPALETTE, force_background: BOOL) -> HPALETTE;
  pub fn RealizePalette (hdc: HDC) -> UINT;
  pub fn UnionRect (dst: LPRECT, src1: *const RECT, src2: *const RECT) -> BOOL;
  pub fn SetRectEmpty (rect: LPRECT) -> BOOL;
}

/// C 24's `<windowsx.h>`: the two allocation wrappers behind C 44-45's `myalloc`/`myfree`.
/// `<windowsx.h>` declares them as macros over `<winbase.h>`'s `GlobalAlloc`/`GlobalFree`; C's own
/// text is kept in the `/// C:` lines and the port declares the two entry points with the
/// signature C's use sites need (a `void *` result, which C casts at C 374/403).
/// C: #define GlobalAllocPtr(flags, cb) (GlobalAlloc(GMEM_MOVEABLE|(flags), (cb)))
/// C: #define GlobalFreePtr(lp) (GlobalUnlockPtr(lp), (HANDLE)GlobalFree((HGLOBAL)GlobalHandle(lp)))
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GlobalAllocPtr (flags: UINT, bytes: usize) -> *mut c_void;
  pub fn GlobalFreePtr (lp: *mut c_void) -> *mut c_void;
}

/* C 26-38: the `WINDOWSNT` arm - the API-version override and the `wtsapi32.h` include.  It is the
   file gate's own condition, so the items carry `windowsnt`; C 27-30's two preprocessor directives
   have no Rust counterpart and the value they fix is carried as a constant (the `src/w32fns.rs`
   63-66 reading of its own pair).  C 31-36's `#ifndef MINGW_W64` carries the non-frozen side.  */
// C 26: `#ifdef WINDOWSNT`
// C 27: `/* Override API version to get the required functionality.  */`
// C 28: `# undef _WIN32_WINNT`
// C 29: `# define _WIN32_WINNT 0x0501`
#[cfg (feature = "windowsnt")]
pub const _WIN32_WINNT: u32 = 0x0501;
/// C 31-36: "mingw.org's MinGW headers mistakenly omit this enumeration".  `MINGW_W64` is `undef`
/// in the frozen table, so this arm is the live side of its condition.
/// C: # ifndef MINGW_W64
/// C: typedef enum _WTS_VIRTUAL_CLASS { WTSVirtualClientData, WTSVirtualFileHandle }
/// C: WTS_VIRTUAL_CLASS;
// C: # endif
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum _WTS_VIRTUAL_CLASS
{
  WTSVirtualClientData = 0,
  WTSVirtualFileHandle = 1,
}
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
pub type WTS_VIRTUAL_CLASS = _WTS_VIRTUAL_CLASS;
/// C 37's `<wtsapi32.h>` - "for WM_WTSSESSION_CHANGE, WTS_SESSION_LOCK".  The header has no file
/// in the reference tree (P-88 ①), and the two names it is named for stand here; each takes the
/// type of its own use site at C 434-435 (the `src/w32fns.rs` 920-922 pair, which types the first
/// `UINT` for a `switch` the same way - see the ledger's Deviations).
/// C 37: `#include <wtsapi32.h>	/* for WM_WTSSESSION_CHANGE, WTS_SESSION_LOCK */`
/// C: #define WM_WTSSESSION_CHANGE 0x02B1
/// C: #define WTS_SESSION_LOCK 0x7
#[cfg (feature = "windowsnt")]
pub const WM_WTSSESSION_CHANGE: c_int = 0x02B1;
#[cfg (feature = "windowsnt")]
pub const WTS_SESSION_LOCK: WPARAM = 0x7;
// C 38: `#endif	/* WINDOWSNT */`

/* C 40-43: the Emacs headers, in C's order (PORTING.md 0.1-14: `lisp.rs` first, because the other
   mirrors expand its macros textually).  `../lib/include/stdlib.rs` is lisp.h's own `<stdlib.h>`
   edge: `include/lisp.rs` pulls `<string.h>` (its C 30) but not `<stdlib.h>`, and this file's
   `malloc`/`free` (C 70, 120, 121, 124) come from exactly that line, so the mirror is included
   under the `lisp.h` line it belongs to (the `src/w32inevt.rs` 432 reading).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");           // lisp.h's `<stdlib.h>` (C 70's `malloc`, C 120's `free`)
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                    // C 40: `#include "lisp.h"`
#[cfg (feature = "windowsnt")]
include!("include/frame.rs");                   // C 41: `#include "frame.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32term.rs");                 // C 42: `#include "w32term.h"`

/* C 44-45: the two allocation macros.  C's bodies keep their own tokens; `GPTR` is the `<winbase.h>`
   flag name declared above.  */
/// C: #define myalloc(cb) GlobalAllocPtr (GPTR, cb)
#[cfg (feature = "windowsnt")]
macro_rules! myalloc {
  ($cb:expr) => { GlobalAllocPtr (GPTR, $cb) };
}
/// C: #define myfree(lp) GlobalFreePtr (lp)
#[cfg (feature = "windowsnt")]
macro_rules! myfree {
  ($lp:expr) => { GlobalFreePtr ($lp) };
}

/// C 47: `CRITICAL_SECTION critsect;` - a non-`static` definition (PORTING.md 0.2), and C's
/// tentative definition is zero-initialized (PORTING.md 1.5's zeroed form).
/// C: CRITICAL_SECTION critsect;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut critsect: CRITICAL_SECTION = CRITICAL_SECTION {
  DebugInfo: core::ptr::null_mut (), LockCount: 0, RecursionCount: 0,
  OwningThread: core::ptr::null_mut (), LockSemaphore: core::ptr::null_mut (), SpinCount: 0,
};

/* C 49-51: "keyboard_handle", `w32console.c`'s own object - C's `extern` declaration, the
   `src/w32inevt.rs` 464 shape (no reachable mirror carries it).  */
// C 49: `#ifdef WINDOWSNT`
/// C: extern HANDLE keyboard_handle;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut keyboard_handle: HANDLE;
}
// C 51: `#endif /* WINDOWSNT */`

/// C 53: `static HANDLE input_available = NULL;` - C's `static`, so a plain `static mut`.
/// C: static HANDLE input_available = NULL;
#[cfg (feature = "windowsnt")]
static mut input_available: HANDLE = core::ptr::null_mut ();
/// C 54-55: `extern HANDLE interrupt_handle; HANDLE interrupt_handle = NULL;` - C's own prototype
/// plus the definition's zero value (PORTING.md 0.2/1.5; the `src/w32console.rs` 1736 reading).
/// C: extern HANDLE interrupt_handle;
/// C: HANDLE interrupt_handle = NULL;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut interrupt_handle: HANDLE = core::ptr::null_mut ();

/* C 57-92: `init_crit`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_crit ()
{
  InitializeCriticalSection (&raw mut critsect);

  /* For safety, input_available should only be reset by get_next_msg
     when the input queue is empty, so make it a manual reset event. */
  input_available = CreateEvent (core::ptr::null_mut (), TRUE, FALSE, core::ptr::null_mut ());

  #[cfg (feature = "have_w32notify")]
  {
    /* Initialize the linked list of notifications sets that will be
       used to communicate between the watching worker threads and the
       main thread.  */
    notifications_set_head = malloc (core::mem::size_of::<notifications_set> ())
      as *mut notifications_set;
    if ! notifications_set_head.is_null ()
      {
        memset (notifications_set_head as *mut c_void, 0,
                core::mem::size_of::<notifications_set> ());
        (*notifications_set_head).prev = notifications_set_head;
        (*notifications_set_head).next = notifications_set_head;
      }
    else
      { DebPrint! (("Out of memory: can't initialize notifications sets.")); }
  }

  keyboard_handle = input_available;

  /* interrupt_handle is signaled when quit (C-g) is detected, so that
     blocking system calls can be interrupted.  We make it a manual
     reset event, so that if we should ever have multiple threads
     performing system calls, they will all be interrupted (I'm guessing
     that would the right response).  Note that we use PulseEvent to
     signal this event, so that it never remains signaled.  */
  interrupt_handle = CreateEvent (core::ptr::null_mut (), TRUE, FALSE, core::ptr::null_mut ());
}

/* C 94-126: `delete_crit`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_crit ()
{
  DeleteCriticalSection (&raw mut critsect);

  if ! input_available.is_null ()
    {
      CloseHandle (input_available);
      input_available = core::ptr::null_mut ();
    }
  if ! interrupt_handle.is_null ()
    {
      CloseHandle (interrupt_handle);
      interrupt_handle = core::ptr::null_mut ();
    }

  #[cfg (feature = "have_w32notify")]
  {
    if ! notifications_set_head.is_null ()
      {
        /* Free any remaining notifications set that could be left over.  */
        while (*notifications_set_head).next != notifications_set_head
          {
            let ns: *mut notifications_set = (*notifications_set_head).next;
            (*notifications_set_head).next = (*ns).next;
            (*(*ns).next).prev = notifications_set_head;
            if ! (*ns).notifications.is_null ()
              { free ((*ns).notifications as *mut c_void); }
            free (ns as *mut c_void);
          }
      }
    free (notifications_set_head as *mut c_void);
  }
}

/* C 128-134: `signal_quit`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_quit ()
{
  /* Make sure this event never remains signaled; if the main thread
     isn't in a blocking call, then this should do nothing.  */
  PulseEvent (interrupt_handle);
}

/* C 136-161: `select_palette`.  `FOR_EACH_FRAME`'s body is the macro's last argument (the
   `include/frame.rs` 793 shape); the two loop variables are C's own block locals.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn select_palette (f: *mut frame, hdc: HDC)
{
  let display_info: *mut w32_display_info = FRAME_DISPLAY_INFO! (f);

  if (*display_info).has_palette == 0
    { return; }

  if (*display_info).palette.is_null ()
    { return; }

  if ! NILP (Vw32_enable_palette)
    { (*((*f).output_data.w32 as *mut w32_output)).old_palette
        = SelectPalette (hdc, (*display_info).palette, FALSE); }
  else
    { (*((*f).output_data.w32 as *mut w32_output)).old_palette = core::ptr::null_mut (); }

  if RealizePalette (hdc) != GDI_ERROR
    {
      let mut frame: Lisp_Object;
      let mut framelist: Lisp_Object;
      FOR_EACH_FRAME! (framelist, frame, {
        SET_FRAME_GARBAGED! (XFRAME (frame));
      });
    }
}

/* C 163-168: `deselect_palette`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn deselect_palette (f: *mut frame, hdc: HDC)
{
  if ! (*((*f).output_data.w32 as *mut w32_output)).old_palette.is_null ()
    { SelectPalette (hdc, (*((*f).output_data.w32 as *mut w32_output)).old_palette, FALSE); }
}

/* C 170-241: `get_frame_dc` (C 170-171's comment above it).  */
/* Get a DC for frame and select palette for drawing; force an update of
   all frames if palette's mapping changes.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_frame_dc (f: *mut frame) -> HDC
{
  let hdc: HDC;
  let paint_dc: HDC;
  let back_buffer: HBITMAP;
  let obj: HGDIOBJ;
  let output: *mut w32_output;

  if ! FRAME_W32_P (f)
    { emacs_abort (); }

  enter_crit! ();
  output = FRAME_OUTPUT_DATA! (f) as *mut w32_output;

  if ! (*output).paint_dc.is_null ()
    {
      if (*output).paint_buffer_width != FRAME_PIXEL_WIDTH (f)
	  || (*output).paint_buffer_height != FRAME_PIXEL_HEIGHT (f)
	  || w32_disable_double_buffering
	{ w32_release_paint_buffer (f); }
      else
	{
	  (*output).paint_buffer_dirty = 1;
	  return (*output).paint_dc;
	}
    }

  hdc = GetDC ((*output).window_desc);

  /* If this gets called during startup before the frame is valid,
     there is a chance of corrupting random data or crashing. */
  if ! hdc.is_null ()
    {
      select_palette (f, hdc);

      if ! w32_disable_double_buffering
	  && (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).want_paint_buffer != 0
	{
	  back_buffer
	    = CreateCompatibleBitmap (hdc, FRAME_PIXEL_WIDTH (f),
				      FRAME_PIXEL_HEIGHT (f));

	  if ! back_buffer.is_null ()
	    {
	      paint_dc = CreateCompatibleDC (hdc);

	      if paint_dc.is_null ()
		{ DeleteObject (back_buffer); }
	      else
		{
		  obj = SelectObject (paint_dc, back_buffer);

		  (*output).paint_dc_object = obj;
		  (*output).paint_dc = paint_dc;
		  (*output).paint_buffer_handle = hdc;
		  (*output).paint_buffer = back_buffer;
		  (*output).paint_buffer_width = FRAME_PIXEL_WIDTH (f);
		  (*output).paint_buffer_height = FRAME_PIXEL_HEIGHT (f);
		  (*output).paint_buffer_dirty = 1;

		  SET_FRAME_GARBAGED! (f);

		  return paint_dc;
		}
	    }
	}
    }

  return hdc;
}

/* C 243-261: `release_frame_dc`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_frame_dc (f: *mut frame, hdc: HDC) -> c_int
{
  let ret: c_int;

  /* Avoid releasing the double-buffered DC here, since it'll be
     released upon the next buffer flip instead.  */
  if hdc != (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc
    {
      deselect_palette (f, hdc);
      ret = ReleaseDC ((*((*f).output_data.w32 as *mut w32_output)).window_desc, hdc);
    }
  else
    { ret = 0; }

  leave_crit! ();

  return ret;
}

/// C 263-267: the queue node.
/// C: typedef struct int_msg { W32Msg w32msg; struct int_msg *lpNext; } int_msg;
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct int_msg
{
  pub w32msg: W32Msg,
  pub lpNext: *mut int_msg,
}

/// C 269-271: the two ends of the queue and its length - C's `static` objects.
/// C: static int_msg *lpHead = NULL; static int_msg *lpTail = NULL; static int nQueue = 0;
#[cfg (feature = "windowsnt")]
static mut lpHead: *mut int_msg = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut lpTail: *mut int_msg = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut nQueue: c_int = 0;

/* C 273-353: `get_next_msg`.  The inner block's `lpCur` and the WM_PAINT consolidation's three
   pointers are C's own locals; C's `memcpy` keeps its `sizeof (W32Msg)` as `size_of` (PORTING.md
   2.1).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_next_msg (lpmsg: *mut W32Msg, bWait: BOOL) -> BOOL
{
  let mut bRet: BOOL = FALSE;

  enter_crit! ();

  /* The while loop takes care of multiple sets */

  while nQueue == 0 && bWait != 0
    {
      leave_crit! ();
      WaitForSingleObject (input_available, INFINITE);
      enter_crit! ();
    }

  if nQueue != 0
    {
      memcpy (lpmsg as *mut c_void, &raw const (*lpHead).w32msg as *const c_void,
	      core::mem::size_of::<W32Msg> ());

      {
	let lpCur: *mut int_msg = lpHead;

	lpHead = (*lpHead).lpNext;

	myfree! (lpCur as *mut c_void);
      }

      nQueue -= 1;
      /* Consolidate WM_PAINT messages to optimize redrawing.  */
      if (*lpmsg).msg.message == WM_PAINT && nQueue != 0
        {
          let mut lpCur: *mut int_msg = lpHead;
          let mut lpPrev: *mut int_msg = core::ptr::null_mut ();
          let mut lpNext: *mut int_msg = core::ptr::null_mut ();

          while ! lpCur.is_null () && nQueue != 0
            {
              lpNext = (*lpCur).lpNext;
              if (*lpCur).w32msg.msg.message == WM_PAINT
                {
                  /* Remove this message from the queue.  */
                  if ! lpPrev.is_null ()
                    { (*lpPrev).lpNext = lpNext; }
                  else
                    { lpHead = lpNext; }

                  if lpCur == lpTail
                    { lpTail = lpPrev; }

                  /* Adjust clip rectangle to cover both.  */
                  if UnionRect (&raw mut (*lpmsg).rect, &raw const (*lpmsg).rect,
                                &raw const (*lpCur).w32msg.rect) == 0
                    {
                      SetRectEmpty (&raw mut (*lpmsg).rect);
                    }

                  myfree! (lpCur as *mut c_void);

                  nQueue -= 1;

                  lpCur = lpNext;
                }
              else
                {
                  lpPrev = lpCur;
                  lpCur = lpNext;
                }
            }
        }

      bRet = TRUE;
    }

  if nQueue == 0
    { ResetEvent (input_available); }

  leave_crit! ();

  return bRet;
}

/// C 355: `extern char * w32_strerror (int error_no);` - the declaration of `w32.c`'s entry point
/// at its own C position.  The two mirrors that carry it (`include/w32.rs` 149, `include/cygw32.rs`)
/// are not on this file's include graph, so the unit spells it (the `src/w32fns.rs` 117-119
/// reading of a neighbour's declaration).
/// C: extern char * w32_strerror (int error_no);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn w32_strerror (error_no: c_int) -> *mut c_char;
}

/* C 357-369: `notify_msg_ready` - the C `static` helper; C 364-368's `CYGWIN` arm is its own
   `feature = "cygwin"` (the frozen table has `CYGWIN` `undef`, so the arm is dead there).  */
/* Tell the main thread that we have input available; if the main
   thread is blocked in select(), we wake it up here.  */
#[cfg (feature = "windowsnt")]
unsafe fn notify_msg_ready ()
{
  SetEvent (input_available);

  #[cfg (feature = "cygwin")]
  {
    /* Wakes up the main thread, which is blocked select()ing for /dev/windows,
       among other files.  */
    let _ = PostThreadMessage (dwMainThreadId, WM_EMACS_INPUT_READY as UINT, 0, 0);
  }
}

/* C 371-398: `post_msg`.  C 374's `(int_msg *)` cast is the port's `as *mut int_msg`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn post_msg (lpmsg: *mut W32Msg) -> BOOL
{
  let lpNew: *mut int_msg = myalloc! (core::mem::size_of::<int_msg> ()) as *mut int_msg;

  if lpNew.is_null ()
    { return FALSE; }

  memcpy (&raw mut (*lpNew).w32msg as *mut c_void, lpmsg as *const c_void,
	  core::mem::size_of::<W32Msg> ());
  (*lpNew).lpNext = core::ptr::null_mut ();

  enter_crit! ();

  /* C's `if (nQueue++)` reads the value and steps it in one expression; Rust has no
     post-increment operator, so the pre-step value is bound first and the step follows
     (the `src/w32term.rs` 1573 reading of its own post-increment loop).  */
  let queued = nQueue;
  nQueue += 1;
  if queued != 0
    {
      (*lpTail).lpNext = lpNew;
    }
  else
    {
      lpHead = lpNew;
    }

  lpTail = lpNew;
  notify_msg_ready ();
  leave_crit! ();

  return TRUE;
}

/* C 400-419: `prepend_msg`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepend_msg (lpmsg: *mut W32Msg) -> BOOL
{
  let lpNew: *mut int_msg = myalloc! (core::mem::size_of::<int_msg> ()) as *mut int_msg;

  if lpNew.is_null ()
    { return FALSE; }

  memcpy (&raw mut (*lpNew).w32msg as *mut c_void, lpmsg as *const c_void,
	  core::mem::size_of::<W32Msg> ());

  enter_crit! ();

  nQueue += 1;
  (*lpNew).lpNext = lpHead;
  lpHead = lpNew;
  notify_msg_ready ();
  leave_crit! ();

  return TRUE;
}

/* C 421-447: `drain_message_queue` (C 421-422's comment above it).  C's `switch (msg.message)`
   is a `match`; the scrutinee's `UINT` meets the mirror's `c_int` message constants, so it is cast
   once at the scrutinee (the `src/w32menu.rs` 708-717 reading of a `UINT` message and a `c_int`
   constant) - see the ledger's Deviations.  */
/* Process all messages in the current thread's queue.  Value is 1 if
   one of these messages was WM_EMACS_FILENOTIFY, zero otherwise.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn drain_message_queue () -> c_int
{
  let mut msg: MSG = core::mem::zeroed ();
  let mut retval: c_int = 0;

  while PeekMessage (&raw mut msg, core::ptr::null_mut (), 0, 0, PM_REMOVE) != 0
    {
      match msg.message as c_int
	{
	  #[cfg (feature = "windowsnt")]
	  WM_WTSSESSION_CHANGE =>
	    { if msg.wParam == WTS_SESSION_LOCK { reset_w32_kbdhook_state (); } }
	  WM_EMACS_FILENOTIFY =>
	    { retval = 1; }
	  _ => {}
	}
      TranslateMessage (&raw const msg);
      DispatchMessage (&raw const msg);
    }
  return retval;
}
