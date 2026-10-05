// femacs: port of src/w32fns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// This file is the largest of P-86's class: the frozen configuration compiles no `w32fns.o` -
// `configure.ac` 3023 puts the name in `W32_OBJ` under `HAVE_W32`, which is the same test that
// `AC_DEFINE`s `HAVE_NTGUI` (3024), and `docs/baseline/frozen-build-sources.txt` lists no
// `w32fns.o` for this target - so `extract-abi.py`'s checklist for the file is empty and
// `docs/ports/w32fns.md` carries P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
// readings and the `## Deviations`.  This batch is P-86 ④'s "first slice": **C 1-884**, which is
// the largest run of whole items that ends at or before C 900 (C 897-900 is the middle of a call
// expression inside `add_system_logical_colors_to_map`; the ledger records the choice).
//
//   * the **file gate** is `feature = "have_ntgui"`, on every item *and on C 27-97's include
//     graph* - which is why the frozen configuration compiles nothing here.  `HAVE_NTGUI` is the
//     macro `configure.ac` 3024 defines for the native MS Windows GUI, i.e. exactly the condition
//     that puts `w32fns.o` in `W32_OBJ`; it is *not* `WINDOWSNT` (the Cygwin GUI build compiles
//     this file with `HAVE_NTGUI` and without `WINDOWSNT`, and C 61/310/344 gate on the latter);
//   * C 38's `#if !defined MINGW_W64 && !defined CYGWIN`, C 61's `#ifdef WINDOWSNT`, C 74's
//     `#if CYGWIN`, C 292's `#ifdef GLYPH_DEBUG`, C 310's `#ifdef WINDOWSNT`, C 334's `#ifdef
//     HAVE_NTGUI` and C 344's `#ifdef WINDOWSNT` are the features `mingw_w64`, `cygwin`,
//     `windowsnt` and `glyph_debug` - each nested inside the file gate, both arms carried - and
//     C 167's `#if _WIN32_WINDOWS >= 0x0410` is `win32_windows_ge_0410`, named after C's own
//     condition the way `lib/include/boot-time-aux.rs` names `_WIN32_WINNT`'s predicate
//     `win32_vista`.  `tools/check-cond-eval.py src/w32fns.c` reads that file as `117 conditions,
//     3 unknown, 3 disagreements [NOT VERIFIED]` (C 167, C 8286 and C 8300 are `_WIN32_WINDOWS`/
//     `_WIN32_WINNT` value tests the evaluator keeps live where clang says False), which is why
//     both of C 167's arms stay text here;
//   * the `#ifndef X` blocks (C 105, 109, 113, 121, 145, 148, 151, 154, 180, 345, 352) are
//     "define it if the SDK has not" guards: the port carries the constant with C's value and no
//     `cfg`, which is the `include/w32term.rs` reading of its nine guards.
//
// **Acquisition note - this batch is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc` is empty, no Homebrew
// `mingw-w64`), so no `<windows.h>` exists to compile against and **no arm flip and no layout
// measurement is possible**: every `#[cfg (feature = "have_ntgui")]` item below is text that
// rustc never sees on this target, and the `<windows.h>` names are spelled from PORTING.md's
// table (`HANDLE`/`HWND`/`HKEY`/... are `*mut c_void`, `BOOL` is `c_int`, `DWORD` is the 32-bit
// `u32` the Windows ABI fixes - the `sysdep.rs` 1056 / `pdumper.rs` 4968 reading) rather than
// measured.  What the batch does carry: the frozen `build-tu` reading, the C line-range table
// (`check-dead-file-coverage.py`), the `#if` verdicts above, a per-item C citation, and an
// arm-reach pre-flight that compiles this unit's own text (ledger `## Evidence` ②).
//
// `docs/ports/w32fns.md` carries the C line ranges, the acceptance readings, every deviation
// (the `<windows.h>` section, the deferred `SYSTEM_INFO`/`OSVERSIONINFO` pair, the five mirror
// names that collide with this unit's definitions, the two features the tree does not register)
// and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 22-25: "Override API version to get the latest functionality."  The two directives have no
   Rust counterpart (there is no preprocessor); the value is what the `<windows.h>` section below
   spells its SDK entries from, so it is carried as a constant.  */
// C 24: `#undef _WIN32_WINNT`
// C 25: `#define _WIN32_WINNT 0x0600`
#[cfg (feature = "have_ntgui")]
pub const _WIN32_WINNT: u32 = 0x0600;

/* C 27-35: the system headers.  Every one of them sits inside the file gate, as C's own build does
   (no translation unit the frozen configuration builds reaches this file).  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/signal.rs");           // C 27: `#include <signal.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdio.rs");            // C 28: `#include <stdio.h>` (sscanf)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/limits.in.rs");        // C 29: `#include <limits.h>` (MAX_PATH via lisp.h)
#[cfg (feature = "have_ntgui")]
include!("../lib/include/errno.rs");            // C 30: `#include <errno.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/math.rs");             // C 31: `#include <math.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/fcntl.in.rs");         // C 32: `#include <fcntl.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/unistd.in.rs");        // C 33: `#include <unistd.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/c-ctype.rs");          // C 35: `#include <c-ctype.h>` (c_isdigit)
/* lisp.h 28 includes `<stdcountof.h>`; the mirror is the one `alloc.rs` includes for C's `countof`,
   and C 848's `countof (w32_color_map)` is this unit's use of it.  */
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdcountof.rs");       // lisp.h 28: `#include <stdcountof.h>` (countof)

/// C 37: `#define COBJMACROS /* Ask for C definitions for COM.  */` - the macro selects the
/// `IUnknown_*` C spellings in the COM headers; the port has no COM mirror, and the name is kept
/// as the definition record.  The `<windows.h>` section below declares the entries this slice
/// names from that block.
/// C: #define COBJMACROS /* Ask for C definitions for COM.  */
#[cfg (feature = "have_ntgui")]
pub const COBJMACROS: () = ();

/// C 38-40: the GUID initializer is needed by MinGW's and Cygwin's headers only.  Both arms are
/// carried; the feature names the non-frozen side (`mingw_w64`/`cygwin`), the `mingw_w64` one
/// being a name this tree does not carry yet (the ledger's owed list).
/// C: #if !defined MINGW_W64 && !defined CYGWIN
/// C: # define INITGUID
/// C: #endif
#[cfg (all (feature = "have_ntgui", not (feature = "mingw_w64"), not (feature = "cygwin")))]
pub const INITGUID: () = ();

/* C 41-46: the COM/GUID headers.  None of them has a file in the reference tree, and this slice
   names nothing from them (the COM interfaces arrive at C 3000+, past the slice), so the section
   is the C include list itself - the P-88 shape for a header with no mirror (each line carries C's
   own `#include`, and a declaration belongs under the line that would have declared it).  */
// C 41: `#include <initguid.h>`
// C 42: `#include <shlobj.h>`
// C 43: `#include <shobjidl.h>`
// C 44: `#include <oleidl.h>`
// C 45: `#include <objidl.h>`
// C 46: `#include <ole2.h>`

/* C 48-59: the Emacs headers, in C's order (PORTING.md 0.1-14: `lisp.rs` first, because the other
   mirrors expand its macros textually).  All of them sit inside the file gate.  */
#[cfg (feature = "have_ntgui")]
include!("include/lisp.rs");                    // C 48: `#include "lisp.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                 // C 49: `#include "w32term.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32font.rs");                 // C 50: `#include "w32font.h"`
#[cfg (feature = "have_ntgui")]
include!("include/frame.rs");                   // C 51: `#include "frame.h"`
#[cfg (feature = "have_ntgui")]
include!("include/window.rs");                  // C 52: `#include "window.h"`
#[cfg (feature = "have_ntgui")]
include!("include/character.rs");               // buffer.h's CHAR_STRING family (PORTING.md 0.1-4)
#[cfg (feature = "have_ntgui")]
include!("include/buffer.rs");                  // C 53: `#include "buffer.h"`
#[cfg (feature = "have_ntgui")]
include!("include/keyboard.rs");                // C 54: `#include "keyboard.h"`
#[cfg (feature = "have_ntgui")]
include!("include/blockinput.rs");              // C 55: `#include "blockinput.h"`
#[cfg (feature = "have_ntgui")]
include!("include/coding.rs");                  // C 56: `#include "coding.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32common.rs");               // C 58: `#include "w32common.h"`
#[cfg (feature = "have_ntgui")]
include!("include/w32inevt.rs");                // C 59: `#include "w32inevt.h"`

/* C 61-72: `#ifdef WINDOWSNT`, the MinGW/Cygwin-Windows half.  C 63-67 is the `#ifndef MINGW_W64`
   arm (mingw.org's headers "mistakenly omit this enumeration"), C 69-71 the three SDK headers.  */
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "mingw_w64")))]
/// C: typedef enum _WTS_VIRTUAL_CLASS { WTSVirtualClientData, WTSVirtualFileHandle }
/// C: WTS_VIRTUAL_CLASS;
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum _WTS_VIRTUAL_CLASS
{
  WTSVirtualClientData = 0,
  WTSVirtualFileHandle = 1,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "mingw_w64")))]
pub type WTS_VIRTUAL_CLASS = _WTS_VIRTUAL_CLASS;
/* C 69-71: `<mbstring.h>`, `<mbctype.h>` ("for _getmbcp"), `<wtsapi32.h>` ("for
   WTS(Un)RegisterSessionNotification") - no file in the reference tree (P-88 ①), and this slice
   names nothing from the first two; `<wtsapi32.h>`'s two entry points are named by the typedefs at
   C 221-224 and used at C 3100+, past the slice.  */
// C 69: `#include <mbstring.h>`
// C 70: `#include <mbctype.h>`	/* for _getmbcp */
// C 71: `#include <wtsapi32.h>`	/* for WTS(Un)RegisterSessionNotification */

/* C 74-78: the Cygwin/Windows split: `cygw32.h` for the Cygwin GUI build, `w32.h` otherwise.  Both
   mirrors exist, and each arm's include carries the file gate **and** its own gate.  */
#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
include!("include/cygw32.rs");                  // C 75: `#include "cygw32.h"`
#[cfg (all (feature = "have_ntgui", not (feature = "cygwin")))]
include!("include/w32.rs");                     // C 77: `#include "w32.h"`

#[cfg (feature = "have_ntgui")]
include!("include/pdumper.rs");                 // C 80: `#include "pdumper.h"`

/* C 82-95: the Windows SDK headers, none of which has a file in the reference tree.  This slice
   names nothing from `<commctrl.h>`, `<commdlg.h>`, `<shellapi.h>`, `<shlwapi.h>`, `<ctype.h>`,
   `<winspool.h>`, `<objbase.h>` or `<windowsx.h>`, and the entries it does name from `<imm.h>`
   and `<dlgs.h>` stand under those lines further down.  */
// C 82: `#include <basetyps.h>`
// C 83: `#include <unknwn.h>`
// C 84: `#include <commctrl.h>`
// C 85: `#include <commdlg.h>`
// C 86: `#include <shellapi.h>`
// C 87: `#include <shlwapi.h>`
// C 88: `#include <ctype.h>`
// C 89: `#include <winspool.h>`
// C 90: `#include <objbase.h>`
// C 92: `#include <dlgs.h>`
// C 94: `#include <windowsx.h>`
// C 95: `#include <gdiplus.h>`

/* `<windows.h>` - reached through C 49's chain (`w32term.h` 22 -> `w32gui.h` 21), the header the
   `w32gui.rs` and `w32term.rs` notes say supplies the target's spellings.  It has no file in the
   reference tree, so **the names this slice uses stand here** (P-88 ①: the section's declaration
   site is the chain's include line, and no file is added under `femacs/src/include/`).

   **No measurement was possible**: this machine has no `windows.h` at all (see the header), so
   every `#[repr(C)]` record below is spelled from **C's own member text** - each member's C line
   is in a `/// C:` line above it - and its size is **UNVERIFIED here**; the ledger's owed list
   carries the layout assertions.  The scalar widths are the Windows ABI's, which PORTING.md 1.1
   fixes independently of this target's `c_long`: `LONG`/`ULONG` are 32-bit on Windows, so
   `RECT`'s four members are 4 bytes each and not this target's `c_long`.  */
// C 89's chain: `#include <windows.h>` (w32gui.h 21), `#include <winspool.h>`, `#include <objbase.h>`
#[cfg (feature = "have_ntgui")]
pub type BYTE = u8;
#[cfg (feature = "have_ntgui")]
pub type WORD = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type DWORD = u32;
/* C's `DWORD_PTR` (`syspage_mask`'s type, C 321) is the one `<windows.h>` name this tree already
   carries: `include/w32common.rs` 42 declares it ("a Windows target that grows a windows.h mirror
   must retire this copy rather than declare it twice"), and a second declaration here is `E0428`
   (measured in the arm pre-flight below), so the port uses the mirror's copy.  */
#[cfg (feature = "have_ntgui")]
pub type ULONG = u32;
#[cfg (feature = "have_ntgui")]
pub type LONG = i32;
#[cfg (feature = "have_ntgui")]
pub type BOOL = c_int;
#[cfg (feature = "have_ntgui")]
pub type UINT = c_uint;
#[cfg (feature = "have_ntgui")]
pub type WPARAM = usize;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = isize;
#[cfg (feature = "have_ntgui")]
pub type LRESULT = isize;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HWND = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HKEY = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMENU = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HHOOK = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HDC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMONITOR = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HIMC = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type COLORREF = DWORD;
#[cfg (feature = "have_ntgui")]
pub type LPBYTE = *mut BYTE;
#[cfg (feature = "have_ntgui")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPCVOID = *const c_void;
#[cfg (feature = "have_ntgui")]
pub type LPCWSTR = *const c_ushort;
#[cfg (feature = "have_ntgui")]
pub type PCWSTR = *const c_ushort;
#[cfg (feature = "have_ntgui")]
pub type UINT_PTR = usize;
/// `HRESULT` is `<winerror.h>`'s `typedef LONG HRESULT;` (`<dwmapi.h>`/`<shobjidl.h>`'s entry
/// points below return it).
/// C: typedef LONG HRESULT;
#[cfg (feature = "have_ntgui")]
pub type HRESULT = LONG;
/// C's `WINAPI`/`CALLBACK` are the `__stdcall` convention, which Rust spells `extern "system"`;
/// the two names appear in the `/// C:` lines of the typedefs below that use them.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define FALSE 0
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
/// C: #define TRUE 1
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;
/// C: #define HWND_DESKTOP ((HWND)0)
#[cfg (feature = "have_ntgui")]
pub const HWND_DESKTOP: HWND = core::ptr::null_mut ();
/// C: #define SM_CXSCREEN 0 / #define SM_CYSCREEN 1 / #define SM_CXMAXIMIZED 61 / #define
/// C: SM_CYMAXIMIZED 62
#[cfg (feature = "have_ntgui")]
pub const SM_CXSCREEN: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SM_CYSCREEN: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const SM_CXMAXIMIZED: c_int = 61;
#[cfg (feature = "have_ntgui")]
pub const SM_CYMAXIMIZED: c_int = 62;

/// C 473: `GetWindowRect (FRAME_W32_WINDOW (f), &rect);` - `<winuser.h>`.
/// C: WINUSERAPI BOOL WINAPI GetWindowRect (HWND hWnd, LPRECT lpRect);
/// C: WINUSERAPI int WINAPI MapWindowPoints (HWND hWndFrom, HWND hWndTo, LPPOINT lpPoints,
/// C:                                      UINT cPoints);
/// C: int WINAPI GetSystemMetrics (int nIndex);
#[cfg (feature = "have_ntgui")]
pub type LPRECT = *mut RECT;
#[cfg (feature = "have_ntgui")]
pub type LPPOINT = *mut POINT;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetWindowRect (hwnd: HWND, lpRect: LPRECT) -> BOOL;
  pub fn MapWindowPoints (hwndFrom: HWND, hwndTo: HWND, lpPoints: LPPOINT,
                          cPoints: UINT) -> c_int;
  pub fn GetSystemMetrics (nIndex: c_int) -> c_int;
}

/// C 871: `lstrcmpi (SSDATA (tem), colorname) == 0` - `<winbase.h>`'s case-insensitive compare.
/// `lstrcmpi` is the SDK's `lstrcmpiA`/`lstrcmpiW` alias (UNICODE selects the wide one); C's text
/// writes the alias and the port keeps C's name, so a Windows target supplies whichever entry
/// point its own `#define` picks (PORTING.md 0.1's rename convention).
/// C: WINBASEAPI int WINAPI lstrcmpiA (LPCSTR lpString1, LPCSTR lpString2);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn lstrcmpi (lpString1: *const c_char, lpString2: *const c_char) -> c_int;
}

/// C's `POINT` and `RECT` (both `<windef.h>`) - spelled from the members C's text reads.
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
/// C: `typedef struct tagMSG { HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam; DWORD
/// time; POINT pt; ... } MSG;` - `w32term.h` 736-741's `W32Msg` embeds this record **by value**,
/// and this slice stores two `W32Msg`s, so the record is spelled here (the `include/w32term.rs`
/// note leaves `MSG` to the unit that supplies `<windows.h>`).
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

/// `TITLEBARINFO` and its two macros (C 168-169's `#if` arm names all three).  `<winuser.h>`'s
/// record, spelled from its member list like the two above.
/// C: #define CCHILDREN_TITLEBAR 5
/// C: typedef struct tagTITLEBARINFO { DWORD cbSize; RECT rcTitleBar; DWORD
/// C: rgstate[CCHILDREN_TITLEBAR + 1]; } TITLEBARINFO, *PTITLEBARINFO;
#[cfg (feature = "have_ntgui")]
pub const CCHILDREN_TITLEBAR: usize = 5;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TITLEBARINFO
{
  pub cbSize: DWORD,
  pub rcTitleBar: RECT,
  pub rgstate: [DWORD; CCHILDREN_TITLEBAR + 1],
}
#[cfg (feature = "have_ntgui")]
pub type PTITLEBARINFO = *mut TITLEBARINFO;

/* C 92-93: `<dlgs.h>`'s `CCHDEVICENAME` guard (C 180-182) and `<imm.h>`'s IME entries, which the
   typedefs at C 203-211 name.  */
#[cfg (feature = "have_ntgui")]
include!("include/w32gdiplus.rs");              // C 97: `#include "w32gdiplus.h"`

/*
  Internal/undocumented constants for Windows Dark mode.
  See: https://github.com/microsoft/WindowsAppSDK/issues/41
*/
/// C 103: `#define DARK_MODE_APP_NAME L"DarkMode_Explorer"` - a **wide** string literal, which
/// Rust has no syntax for, so the port spells the same UTF-16 code units in an array (host-endian,
/// as C's `L"..."` is; the trailing `0x0000` is C's terminator).  Its one use is C 2513's
/// `SetWindowTheme (hwnd, DARK_MODE_APP_NAME, L"Explorer")`, past this slice.
/// C: #define DARK_MODE_APP_NAME L"DarkMode_Explorer"
#[cfg (feature = "have_ntgui")]
pub const DARK_MODE_APP_NAME: [u16; 18] =
  [0x0044, 0x0061, 0x0072, 0x006B, 0x004D, 0x006F, 0x0064, 0x0065, 0x005F,
   0x0045, 0x0078, 0x0070, 0x006C, 0x006F, 0x0072, 0x0065, 0x0072, 0x0000];

/* C 104-115: the three `#ifndef` SDK guards.  Each is "define it if the SDK has not" - the port
   carries the constant with C's value and no `cfg`, the `include/w32term.rs` reading of its nine
   guards.  */
/* For Windows 10 version 1809, 1903, 1909. */
/// C: #ifndef DWMWA_USE_IMMERSIVE_DARK_MODE_OLD / #define DWMWA_USE_IMMERSIVE_DARK_MODE_OLD 19
/// C: #endif
#[cfg (feature = "have_ntgui")]
pub const DWMWA_USE_IMMERSIVE_DARK_MODE_OLD: DWORD = 19;
/* For Windows 10 version 2004 and higher, and Windows 11. */
/// C: #ifndef DWMWA_USE_IMMERSIVE_DARK_MODE / #define DWMWA_USE_IMMERSIVE_DARK_MODE 20 / #endif
#[cfg (feature = "have_ntgui")]
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWORD = 20;

/// C: #ifndef FOF_NO_CONNECTED_ELEMENTS / #define FOF_NO_CONNECTED_ELEMENTS 0x2000 / #endif
#[cfg (feature = "have_ntgui")]
pub const FOF_NO_CONNECTED_ELEMENTS: DWORD = 0x2000;

/* C 117-119: the three entries of this file's neighbours.  Each definition is in its own unit
   (`w32proc.c`'s / `w32menu.c`'s), so the declarations are the `extern` form (PORTING.md 0.2).  */
/// C: extern int w32_console_toggle_lock_key (int, Lisp_Object);
/// C: extern void w32_menu_display_help (HWND, HMENU, UINT, UINT);
/// C: extern void w32_free_menu_strings (HWND);
/// C 117's `w32_console_toggle_lock_key` declaration has **no** item here: C declares it at
/// C 117 and defines it in this same file at C 10113, so the declaration and the definition are
/// one C "extern then define" pair and only the definition belongs in the port (PORTING.md
/// 0.1-12; the `is_debugger_present` reading above) - `tools/extract-abi.py --check` reported the
/// declaration as "not static in C but has no `#[unsafe(no_mangle)]`" (docs/ports/w32fns.md,
/// slice 2's Deviations).  The other two are `w32menu.c`'s definitions and stay declarations.
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn w32_menu_display_help (arg0: HWND, arg1: HMENU, arg2: UINT, arg3: UINT);
  pub fn w32_free_menu_strings (arg0: HWND);
}

/// C 121-123's guard names a `<winuser.h>` entry point the SDK may not have; C's value is the
/// resource-id-as-pointer cast macro, whose own `<winuser.h>` text is in the `/// C:` line above
/// the macro (P-88 ② keeps the header's line).
/// C: #define MAKEINTRESOURCEA(i) ((LPSTR)((ULONG_PTR)((WORD)(i))))
#[cfg (feature = "have_ntgui")]
pub type ULONG_PTR = usize;
#[cfg (feature = "have_ntgui")]
pub type LPSTR = *mut c_char;

/* C 563's `RGB (...)` and the 240 `PALETTERGB (...)` initializers are `<wingdi.h>`'s two value
   macros (`#define PALETTERGB(r,g,b) (0x02000000 | RGB(r,g,b))` and `#define RGB(r,g,b)
   ((COLORREF)(((BYTE)(r)|((WORD)((BYTE)(g))<<8))|(((DWORD)(BYTE)(b))<<16)))`).  The port defines
   both with C's expansion - the `docs/ports/androidmenu.md` `ANDROID_DELETE_LOCAL_REF` reading (a
   macro whose *value* is needed in a constant expression cannot just be named the way a macro
   body's call target can) - so the 240 initializers keep C's own spelling.  They stand here with
   the other `<windows.h>` names because a `macro_rules!` must be defined before its first use
   (PORTING.md 0.1-18; C 563 is the first).  */
/// C: #define RGB(r,g,b) ((COLORREF)(((BYTE)(r)|((WORD)((BYTE)(g))<<8))|(((DWORD)(BYTE)(b))<<16)))
/// C: #define PALETTERGB(r,g,b) (0x02000000 | RGB(r,g,b))
#[cfg (feature = "have_ntgui")]
macro_rules! RGB {
  ($r:expr, $g:expr, $b:expr) =>
    { (((($r) as u8 as u16) as u32)
       | (((($g) as u8 as u16) as u32) << 8)
       | (((($b) as u8 as u16) as u32) << 16)) as COLORREF };
}
#[cfg (feature = "have_ntgui")]
macro_rules! PALETTERGB {
  ($r:expr, $g:expr, $b:expr) => { (0x02000000 | RGB! ($r, $g, $b)) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! MAKEINTRESOURCE {
  ($i:expr) => { (($i as ULONG_PTR) as LPSTR) };
}
/// C: #ifndef IDC_HAND / #define IDC_HAND MAKEINTRESOURCE(32649) / #endif
#[cfg (feature = "have_ntgui")]
pub const IDC_HAND: LPSTR = MAKEINTRESOURCE! (32649);

/* Prefix for system colors.  */
/// C: #define SYSTEM_COLOR_PREFIX "System"
#[cfg (feature = "have_ntgui")]
pub const SYSTEM_COLOR_PREFIX: &str = "System";
/// C 127: `#define SYSTEM_COLOR_PREFIX_LEN (sizeof (SYSTEM_COLOR_PREFIX) - 1)` - C's `sizeof` of
/// a string literal minus its `NUL` is the port's `.len ()` at the use point (PORTING.md 2.1).
/// C: #define SYSTEM_COLOR_PREFIX_LEN (sizeof (SYSTEM_COLOR_PREFIX) - 1)
#[cfg (feature = "have_ntgui")]
macro_rules! SYSTEM_COLOR_PREFIX_LEN {
  () => { SYSTEM_COLOR_PREFIX.len () };
}

/* State variables for emulating a three button mouse. */
/// C: #define LMOUSE 1
#[cfg (feature = "have_ntgui")]
pub const LMOUSE: c_int = 1;
/// C: #define MMOUSE 2
#[cfg (feature = "have_ntgui")]
pub const MMOUSE: c_int = 2;
/// C: #define RMOUSE 4
#[cfg (feature = "have_ntgui")]
pub const RMOUSE: c_int = 4;

#[cfg (feature = "have_ntgui")]
static mut button_state: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut saved_mouse_button_msg: W32Msg = unsafe { core::mem::zeroed () };
#[cfg (feature = "have_ntgui")]
static mut mouse_button_timer: c_uint = 0;	/* non-zero when timer is active */
#[cfg (feature = "have_ntgui")]
static mut saved_mouse_move_msg: W32Msg = unsafe { core::mem::zeroed () };
#[cfg (feature = "have_ntgui")]
static mut mouse_move_timer: c_uint = 0;

/* Window that is tracking the mouse.  */
#[cfg (feature = "have_ntgui")]
static mut track_mouse_window: HWND = core::ptr::null_mut ();

/* Multi-monitor API definitions that are not pulled from the headers
   since we are compiling for NT 4.  */
/// C: #ifndef MONITOR_DEFAULT_TO_NEAREST / #define MONITOR_DEFAULT_TO_NEAREST 2 / #endif
#[cfg (feature = "have_ntgui")]
pub const MONITOR_DEFAULT_TO_NEAREST: DWORD = 2;
/// C: #ifndef MONITORINFOF_PRIMARY / #define MONITORINFOF_PRIMARY 1 / #endif
#[cfg (feature = "have_ntgui")]
pub const MONITORINFOF_PRIMARY: DWORD = 1;
/// C: #ifndef SM_XVIRTUALSCREEN / #define SM_XVIRTUALSCREEN 76 / #endif
#[cfg (feature = "have_ntgui")]
pub const SM_XVIRTUALSCREEN: c_int = 76;
/// C: #ifndef SM_YVIRTUALSCREEN / #define SM_YVIRTUALSCREEN 77 / #endif
#[cfg (feature = "have_ntgui")]
pub const SM_YVIRTUALSCREEN: c_int = 77;

/* MinGW headers define MONITORINFO unconditionally, but MSVC ones don't.
   To avoid a compile error on one or the other, redefine with a new name.  */
/// C 159-165.  `<winuser.h>`'s record is *not* used (`MONITOR_INFO` is this file's own name), and
/// its members are all scalar-or-`RECT`, so the spelling is the member list C writes.
/// C: struct MONITOR_INFO { DWORD cbSize; RECT rcMonitor; RECT rcWork; DWORD dwFlags; };
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO
{
  pub cbSize: DWORD,
  pub rcMonitor: RECT,
  pub rcWork: RECT,
  pub dwFlags: DWORD,
}

/// C 167-178: the `TITLEBAR_INFO` pair.  `tools/check-cond-eval.py src/w32fns.c` records C 167 as
/// `DISAGREE(unknown) ... clang False, evaluator keeps the branch live`, so the file is **not** on
/// `cond-eval-verified.txt` and both arms stay text (P-66).  The feature names C's condition the
/// way `lib/include/boot-time-aux.rs` names `_WIN32_WINNT`'s predicate `win32_vista`; the value
/// condition itself cannot be decided here - the machine has no SDK to define `_WIN32_WINDOWS`.
/// C: #if _WIN32_WINDOWS >= 0x0410
/// C: #define C_CHILDREN_TITLEBAR CCHILDREN_TITLEBAR
/// C: typedef TITLEBARINFO TITLEBAR_INFO;
/// C: #else
/// C: #define C_CHILDREN_TITLEBAR 5
/// C: typedef struct { DWORD cbSize; RECT rcTitleBar; DWORD rgstate[C_CHILDREN_TITLEBAR+1]; }
/// C: TITLEBAR_INFO, *PTITLEBAR_INFO;
/// C: #endif
#[cfg (all (feature = "have_ntgui", feature = "win32_windows_ge_0410"))]
pub const C_CHILDREN_TITLEBAR: usize = CCHILDREN_TITLEBAR;
#[cfg (all (feature = "have_ntgui", feature = "win32_windows_ge_0410"))]
pub type TITLEBAR_INFO = TITLEBARINFO;
#[cfg (all (feature = "have_ntgui", not (feature = "win32_windows_ge_0410")))]
pub const C_CHILDREN_TITLEBAR: usize = 5;
#[cfg (all (feature = "have_ntgui", not (feature = "win32_windows_ge_0410")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TITLEBAR_INFO
{
  pub cbSize: DWORD,
  pub rcTitleBar: RECT,
  pub rgstate: [DWORD; C_CHILDREN_TITLEBAR + 1],
}
#[cfg (all (feature = "have_ntgui", not (feature = "win32_windows_ge_0410")))]
pub type PTITLEBAR_INFO = *mut TITLEBAR_INFO;

/// C 180-182's `#ifndef CCHDEVICENAME` guard, used by C 189's `szDevice`.
/// C: #ifndef CCHDEVICENAME / #define CCHDEVICENAME 32 / #endif
#[cfg (feature = "have_ntgui")]
pub const CCHDEVICENAME: usize = 32;

/// C 183-190.  The device name is C's `char szDevice[CCHDEVICENAME]` - a byte array, and the
/// record's members are the scalar-or-`RECT` kind, so the spelling is C's member list.
/// C: struct MONITOR_INFO_EX { DWORD cbSize; RECT rcMonitor; RECT rcWork; DWORD dwFlags;
/// C: char szDevice[CCHDEVICENAME]; };
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MONITOR_INFO_EX
{
  pub cbSize: DWORD,
  pub rcMonitor: RECT,
  pub rcWork: RECT,
  pub dwFlags: DWORD,
  pub szDevice: [c_char; CCHDEVICENAME],
}

/* C 192-247: the lazily bound SDK entry points.  C's `WINAPI` is `extern "system"` in Rust, and a
   C function pointer may be NULL: the typedef stays C's plain pointer type and the *storage*
   below is `Option<…>` (the `include/w32term.rs` 1237-1245 / `lib/include/boot-time-aux.rs` 411
   reading).  Parameter types that the port only ever passes as a pointer degrade per P-72 rule 3
   (`LPTRACKMOUSEEVENT`, `COMPOSITIONFORM *`, `LPLOGFONTW`, `PFLASHWINFO` are `*mut c_void`; C's
   spelling is in the `/// C:` line above each typedef).  */
/// C: typedef BOOL (WINAPI * TrackMouseEvent_Proc) (IN OUT LPTRACKMOUSEEVENT lpEventTrack);
#[cfg (feature = "have_ntgui")]
pub type TrackMouseEvent_Proc = unsafe extern "system" fn (lpEventTrack: *mut c_void) -> BOOL;
/// C: typedef LONG (WINAPI * ImmGetCompositionString_Proc) (IN HIMC context, IN DWORD index,
/// C: IN LPVOID buffer, IN DWORD bufLen);
#[cfg (feature = "have_ntgui")]
pub type ImmGetCompositionString_Proc
  = unsafe extern "system" fn (context: HIMC, index: DWORD, buffer: LPVOID, bufLen: DWORD) -> LONG;
/// C: typedef HIMC (WINAPI * ImmGetContext_Proc) (IN HWND window);
#[cfg (feature = "have_ntgui")]
pub type ImmGetContext_Proc = unsafe extern "system" fn (window: HWND) -> HIMC;
/// C: typedef BOOL (WINAPI * ImmReleaseContext_Proc) (IN HWND wnd, IN HIMC context);
#[cfg (feature = "have_ntgui")]
pub type ImmReleaseContext_Proc = unsafe extern "system" fn (wnd: HWND, context: HIMC) -> BOOL;
/// C: typedef BOOL (WINAPI * ImmSetCompositionWindow_Proc) (IN HIMC context, IN COMPOSITIONFORM
/// C: *form);
#[cfg (feature = "have_ntgui")]
pub type ImmSetCompositionWindow_Proc
  = unsafe extern "system" fn (context: HIMC, form: *mut c_void) -> BOOL;
/* For toggling IME status.  */
/// C: typedef BOOL (WINAPI * ImmGetOpenStatus_Proc) (IN HIMC);
#[cfg (feature = "have_ntgui")]
pub type ImmGetOpenStatus_Proc = unsafe extern "system" fn (context: HIMC) -> BOOL;
/// C: typedef BOOL (WINAPI * ImmSetOpenStatus_Proc) (IN HIMC, IN BOOL);
#[cfg (feature = "have_ntgui")]
pub type ImmSetOpenStatus_Proc = unsafe extern "system" fn (context: HIMC, open: BOOL) -> BOOL;

/* Set IME font.  */
/// C: typedef BOOL (WINAPI * ImmSetCompositionFont_Proc) (IN HIMC, LPLOGFONTW lplf);
#[cfg (feature = "have_ntgui")]
pub type ImmSetCompositionFont_Proc
  = unsafe extern "system" fn (context: HIMC, lplf: *mut c_void) -> BOOL;

/// C: typedef HMONITOR (WINAPI * MonitorFromPoint_Proc) (IN POINT pt, IN DWORD flags);
#[cfg (feature = "have_ntgui")]
pub type MonitorFromPoint_Proc
  = unsafe extern "system" fn (pt: POINT, flags: DWORD) -> HMONITOR;
/// C: typedef BOOL (WINAPI * GetMonitorInfo_Proc) (IN HMONITOR monitor, OUT struct MONITOR_INFO*
/// C: info);
#[cfg (feature = "have_ntgui")]
pub type GetMonitorInfo_Proc
  = unsafe extern "system" fn (monitor: HMONITOR, info: *mut MONITOR_INFO) -> BOOL;
/// C: typedef HMONITOR (WINAPI * MonitorFromWindow_Proc) (IN HWND hwnd, IN DWORD dwFlags);
#[cfg (feature = "have_ntgui")]
pub type MonitorFromWindow_Proc
  = unsafe extern "system" fn (hwnd: HWND, dwFlags: DWORD) -> HMONITOR;
/// C: typedef BOOL CALLBACK (* MonitorEnum_Proc) (IN HMONITOR monitor, IN HDC hdc, IN RECT
/// C: *rcMonitor, IN LPARAM dwData);
#[cfg (feature = "have_ntgui")]
pub type MonitorEnum_Proc
  = unsafe extern "system" fn (monitor: HMONITOR, hdc: HDC, rcMonitor: *mut RECT,
                               dwData: LPARAM) -> BOOL;
/// C: typedef BOOL (WINAPI * EnumDisplayMonitors_Proc) (IN HDC hdc, IN RECT *rcClip, IN
/// C: MonitorEnum_Proc fnEnum, IN LPARAM dwData);
#[cfg (feature = "have_ntgui")]
pub type EnumDisplayMonitors_Proc
  = unsafe extern "system" fn (hdc: HDC, rcClip: *mut RECT, fnEnum: MonitorEnum_Proc,
                               dwData: LPARAM) -> BOOL;
/// C: typedef BOOL (WINAPI * GetTitleBarInfo_Proc) (IN HWND hwnd, OUT TITLEBAR_INFO* info);
#[cfg (feature = "have_ntgui")]
pub type GetTitleBarInfo_Proc
  = unsafe extern "system" fn (hwnd: HWND, info: *mut TITLEBAR_INFO) -> BOOL;

/// C: typedef BOOL (WINAPI *IsDebuggerPresent_Proc) (void);
#[cfg (feature = "have_ntgui")]
pub type IsDebuggerPresent_Proc = unsafe extern "system" fn () -> BOOL;
/// C: typedef HRESULT (WINAPI *SetThreadDescription_Proc) (HANDLE hThread, PCWSTR
/// C: lpThreadDescription);
#[cfg (feature = "have_ntgui")]
pub type SetThreadDescription_Proc
  = unsafe extern "system" fn (hThread: HANDLE, lpThreadDescription: PCWSTR) -> HRESULT;

/// C: typedef HRESULT (WINAPI * SetWindowTheme_Proc) (IN HWND hwnd, IN LPCWSTR pszSubAppName, IN
/// C: LPCWSTR pszSubIdList);
#[cfg (feature = "have_ntgui")]
pub type SetWindowTheme_Proc
  = unsafe extern "system" fn (hwnd: HWND, pszSubAppName: LPCWSTR, pszSubIdList: LPCWSTR) -> HRESULT;
/// C: typedef HRESULT (WINAPI * DwmSetWindowAttribute_Proc) (HWND hwnd, DWORD dwAttribute, IN
/// C: LPCVOID pvAttribute, DWORD cbAttribute);
#[cfg (feature = "have_ntgui")]
pub type DwmSetWindowAttribute_Proc
  = unsafe extern "system" fn (hwnd: HWND, dwAttribute: DWORD, pvAttribute: LPCVOID,
                               cbAttribute: DWORD) -> HRESULT;

/// C: typedef BOOL (WINAPI * WTSRegisterSessionNotification_Proc) (HWND hwnd, DWORD dwFlags);
#[cfg (feature = "have_ntgui")]
pub type WTSRegisterSessionNotification_Proc
  = unsafe extern "system" fn (hwnd: HWND, dwFlags: DWORD) -> BOOL;
/// C: typedef BOOL (WINAPI * WTSUnRegisterSessionNotification_Proc) (HWND hwnd);
#[cfg (feature = "have_ntgui")]
pub type WTSUnRegisterSessionNotification_Proc
  = unsafe extern "system" fn (hwnd: HWND) -> BOOL;

/// C: typedef BOOL (WINAPI * RegisterTouchWindow_proc) (HWND, ULONG);
#[cfg (feature = "have_ntgui")]
pub type RegisterTouchWindow_proc = unsafe extern "system" fn (HWND, ULONG) -> BOOL;

/* Types for gesture recognition are documented by Microsoft but appear
   not to be defined anywhere in MinGW's includes.  */

/// C 237-242.  C's typedef names both the record and a pointer to it; the members are C's.
/// C: typedef struct Emacs_GESTURECONFIG { DWORD dwID; DWORD dwWant; DWORD dwBlock; }
/// C: Emacs_GESTURECONFIG, *Emacs_PGESTURECONFIG;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Emacs_GESTURECONFIG
{
  pub dwID: DWORD,
  pub dwWant: DWORD,
  pub dwBlock: DWORD,
}
#[cfg (feature = "have_ntgui")]
pub type Emacs_PGESTURECONFIG = *mut Emacs_GESTURECONFIG;

/// C: typedef BOOL (WINAPI * SetGestureConfig_proc) (HWND, DWORD, UINT, Emacs_PGESTURECONFIG,
/// C: UINT);
#[cfg (feature = "have_ntgui")]
pub type SetGestureConfig_proc
  = unsafe extern "system" fn (HWND, DWORD, UINT, Emacs_PGESTURECONFIG, UINT) -> BOOL;

/// C: typedef BOOL (WINAPI * FlashWindowEx_Proc) (PFLASHWINFO pfwi);
#[cfg (feature = "have_ntgui")]
pub type FlashWindowEx_Proc = unsafe extern "system" fn (pfwi: *mut c_void) -> BOOL;

#[cfg (feature = "have_ntgui")]
static mut track_mouse_event_fn: Option<TrackMouseEvent_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut get_composition_string_fn: Option<ImmGetCompositionString_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut get_ime_context_fn: Option<ImmGetContext_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut get_ime_open_status_fn: Option<ImmGetOpenStatus_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut set_ime_open_status_fn: Option<ImmSetOpenStatus_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut set_ime_composition_font_fn: Option<ImmSetCompositionFont_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut release_ime_context_fn: Option<ImmReleaseContext_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut set_ime_composition_window_fn: Option<ImmSetCompositionWindow_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut monitor_from_point_fn: Option<MonitorFromPoint_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut get_monitor_info_fn: Option<GetMonitorInfo_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut monitor_from_window_fn: Option<MonitorFromWindow_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut enum_display_monitors_fn: Option<EnumDisplayMonitors_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut get_title_bar_info_fn: Option<GetTitleBarInfo_Proc> = None;
/* C 262-265: C declares the object `extern` and then defines it in the same file.  Rust has one
   item per name, so the definition stays at C's position and the `extern` line has no counterpart
   (PORTING.md 0.1-12, which names exactly this pair for functions; the ledger records it).  The
   two symbols are read by other Windows units (`w32proc.c`).
   C: extern IsDebuggerPresent_Proc is_debugger_present;
   C: IsDebuggerPresent_Proc is_debugger_present = NULL;  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut is_debugger_present: Option<IsDebuggerPresent_Proc> = None;
/* C: extern SetThreadDescription_Proc set_thread_description;
   C: SetThreadDescription_Proc set_thread_description = NULL;  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut set_thread_description: Option<SetThreadDescription_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut SetWindowTheme_fn: Option<SetWindowTheme_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut DwmSetWindowAttribute_fn: Option<DwmSetWindowAttribute_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut WTSUnRegisterSessionNotification_fn: Option<WTSUnRegisterSessionNotification_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut WTSRegisterSessionNotification_fn: Option<WTSRegisterSessionNotification_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut RegisterTouchWindow_fn: Option<RegisterTouchWindow_proc> = None;
#[cfg (feature = "have_ntgui")]
static mut SetGestureConfig_fn: Option<SetGestureConfig_proc> = None;
#[cfg (feature = "have_ntgui")]
static mut flash_window_ex_fn: Option<FlashWindowEx_Proc> = None;

/* C 274: declared here and defined by `w32menu.c`.  The port spells C's nullable function pointer
   as the mirror's own `Option<…>` alias (`w32term.h` 869-873), which the mirror carries under
   `windowsnt` - so this declaration carries the same gate, and the ledger records that the
   mirror's umbrella should be `have_ntgui` (w32term.h has no platform `#if` of its own).  */
/// C: extern AppendMenuW_Proc unicode_append_menu;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut unicode_append_menu: AppendMenuW_Proc;
}

/* Flag to selectively ignore WM_IME_CHAR messages.  */
#[cfg (feature = "have_ntgui")]
static mut ignore_ime_char: c_int = 0;

/* W95 mousewheel handler */
/* C 280-281: the same `extern`-then-define pair as C 262-265 (no header declares
   `msh_mousewheel`), so the definition is the single item.
   C: extern unsigned int msh_mousewheel;
   C: unsigned int msh_mousewheel = 0;  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut msh_mousewheel: c_uint = 0;

/* Timers */
/// C: #define MOUSE_BUTTON_ID	1
#[cfg (feature = "have_ntgui")]
pub const MOUSE_BUTTON_ID: c_uint = 1;
/// C: #define MOUSE_MOVE_ID	2
#[cfg (feature = "have_ntgui")]
pub const MOUSE_MOVE_ID: c_uint = 2;
/// C: #define MENU_FREE_ID 3
#[cfg (feature = "have_ntgui")]
pub const MENU_FREE_ID: c_uint = 3;
/* The delay (milliseconds) before a menu is freed after WM_EXITMENULOOP
   is received.  */
/// C: #define MENU_FREE_DELAY 1000
#[cfg (feature = "have_ntgui")]
pub const MENU_FREE_DELAY: c_uint = 1000;
#[cfg (feature = "have_ntgui")]
static mut menu_free_timer: c_uint = 0;

/* C 292-294: the reference-count debug aid, dead text under the frozen configuration
   (`GLYPH_DEBUG` is `undef` in docs/baseline/config/darwin-arm64.macros.tsv) - carried under the
   feature that names the non-frozen side (P-51; `glyph_debug` is registered and
   `xtask dead-arms`-flippable).  */
/// C: #ifdef GLYPH_DEBUG / static int dpyinfo_refcount; / #endif
#[cfg (feature = "glyph_debug")]
static mut dpyinfo_refcount: c_int = 0;

#[cfg (feature = "have_ntgui")]
static mut w32_visible_system_caret_hwnd: HWND = core::ptr::null_mut ();

#[cfg (feature = "have_ntgui")]
static mut w32_unicode_gui: c_int = 0;

#[cfg (feature = "have_ntgui")]
static mut w32_selection_dialog_open: bool = false;

/* From w32menu.c  */
/* C 303: `w32term.h` 283 declares this object for the other Windows units, and this file is its
   definition - the declaration/definition pair the port cannot spell twice (E0428, PORTING.md
   0.1-10); the definition is here and the mirror's copy is one of the five names the ledger's
   owed list asks the integrator to move into a `*_decls` module.  */
/// C: int menubar_in_use = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut menubar_in_use: c_int = 0;

/* From w32uniscribe.c  */
/// C: extern void syms_of_w32uniscribe (void);
/// C: extern int uniscribe_available;
/// C: extern int harfbuzz_available;
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn syms_of_w32uniscribe ();
  pub static mut uniscribe_available: c_int;
  pub static mut harfbuzz_available: c_int;
}

/// C 310-313: the `WINDOWSNT` arm (`w32inevt.c` defines `faked_key`; a Cygwin GUI build does not
/// have it).
/// C: #ifdef WINDOWSNT / /* From w32inevt.c */ extern int faked_key; / #endif
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut faked_key: c_int;
}

/* This gives us the page size and the size of the allocation unit on NT.  */
/* C 315-319: **deferred, not written** - `include/w32common.rs` 35-36 records the same decision
   for the same two records ("C declares a `<sysinfoapi.h>`/`<winver.h>` record *by value*, the
   port has no mirror of either, and no compiler on this target could measure them - so no Rust
   spelling is written rather than an unmeasured one"), and this file is where both objects are
   defined.  The ledger's Deviations and its owed list carry the pair.  */
/// C: SYSTEM_INFO sysinfo_cache;
/// C: OSVERSIONINFO osinfo_cache;

/* C 321-326: the four globals `w32common.h` 37-41 declares (this file defines them; see the
   collision note above).  The C objects have no initializer, i.e. C's tentative definitions
   (zero).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut syspage_mask: DWORD_PTR = 0;

/* The major and minor versions of NT.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_major_version: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_minor_version: c_int = 0;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut w32_build_number: c_int = 0;

/* If the OS is set to use dark mode.  */
#[cfg (feature = "have_ntgui")]
static mut w32_darkmode: BOOL = FALSE;

/* Distinguish between Windows NT and Windows 95.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut os_subtype: c_int = 0;

/* C 334-336: the application instance.  `w32gui.h` 57 declares it and this file defines it (the
   `hinst` collision in the ledger); the C text is inside `#ifdef HAVE_NTGUI`, which is the file
   gate itself.  */
/// C: #ifdef HAVE_NTGUI / HINSTANCE hinst = NULL; / #endif
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut hinst: HINSTANCE = core::ptr::null_mut ();

#[cfg (feature = "have_ntgui")]
static mut sound_type: c_uint = 0xFFFFFFFF;
/// C: #define MB_EMACS_SILENT (0xFFFFFFFF - 1)
#[cfg (feature = "have_ntgui")]
pub const MB_EMACS_SILENT: DWORD = 0xFFFFFFFF - 1;

/* Special virtual key code for indicating "any" key.  */
/// C: #define VK_ANY 0xFF
#[cfg (feature = "have_ntgui")]
pub const VK_ANY: c_int = 0xFF;

/// C 344-350: the `WINDOWSNT` arm, whose body is another "the SDK has not got it" guard (32-bit
/// MinGW's headers lack both names).
/// C: #ifdef WINDOWSNT
/// C: # ifndef WM_WTSSESSION_CHANGE
/// C: #  define WM_WTSSESSION_CHANGE  0x02B1
/// C: #  define WTS_SESSION_LOCK      0x7
/// C: # endif
/// C: #endif
#[cfg (feature = "windowsnt")]
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
#[cfg (feature = "windowsnt")]
pub const WTS_SESSION_LOCK: UINT = 0x7;

/// C 352-354's guard.  C's `0x08000000L` is a `long` literal; on Windows a `long` is 32 bits, so
/// the port's `LONG` is the same 32-bit signed type (PORTING.md 1.1's `long double`/`float` rows
/// keep C's widths, and this target's own `c_long` is 64-bit - the ledger records the choice).
/// C: #ifndef WS_EX_NOACTIVATE / #define WS_EX_NOACTIVATE 0x08000000L / #endif
#[cfg (feature = "have_ntgui")]
pub const WS_EX_NOACTIVATE: LONG = 0x08000000;

/* Keyboard hook state data.  */
/// C 357-374: C's record here is an **anonymous** `static struct { … } kbdhook;`, which Rust needs
/// a name for (PORTING.md 1.3: an anonymous struct must be named; the `font_expand_range`,
/// `map_keymap_s` and `read_non_regular_s` readings), so the port names it after the object.
/// C: static struct { int hook_count; HHOOK hook; HWND console; HWND notified_wnd; int lwindown;
/// C: int rwindown; int winsdown; int send_win_up; int suppress_lone; int winseen; char
/// C: alt_hooked[256]; char lwin_hooked[256]; char rwin_hooked[256]; } kbdhook;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct kbdhook_s
{
  pub hook_count: c_int, /* counter, if several windows are created */
  pub hook: HHOOK,     /* hook handle */
  pub console: HWND,   /* console window handle */
  pub notified_wnd: HWND, /* window that receives session notifications */

  pub lwindown: c_int,      /* Left Windows key currently pressed (and hooked) */
  pub rwindown: c_int,      /* Right Windows key currently pressed (and hooked) */
  pub winsdown: c_int,      /* Number of handled keys currently pressed */
  pub send_win_up: c_int,   /* Pass through the keyup for this Windows key press? */
  pub suppress_lone: c_int, /* Suppress simulated Windows keydown-keyup for this press? */
  pub winseen: c_int,       /* Windows keys seen during this press? */

  pub alt_hooked: [c_char; 256],  /* hook Alt+[this key]? */
  pub lwin_hooked: [c_char; 256], /* hook left Win+[this key]? */
  pub rwin_hooked: [c_char; 256], /* hook right Win+[this key]? */
}
#[cfg (feature = "have_ntgui")]
static mut kbdhook: kbdhook_s = unsafe { core::mem::zeroed () };
/// C: typedef HWND (WINAPI *GetConsoleWindow_Proc) (void);
#[cfg (feature = "have_ntgui")]
pub type GetConsoleWindow_Proc = unsafe extern "system" fn () -> HWND;

/* stdin, from w32console.c */
/// C: extern HANDLE keyboard_handle;
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub static mut keyboard_handle: HANDLE;
}

/* C 380-383: three forward declarations of `static` functions defined later in the file.  Rust has
   no forward declarations (PORTING.md 0.1-12: a declaration plus a definition is `E0428`, and
   items are order-independent), so the three lines have no counterpart here; their definitions
   stand at C's positions (C 1007-1009 for `my_post_msg`/`w32_get_modifiers`,
   C 9581-9586 for `w32_display_info_for_name`), past this slice.
   C: static struct w32_display_info *w32_display_info_for_name (Lisp_Object);
   C: static void my_post_msg (W32Msg *, HWND, UINT, WPARAM, LPARAM);
   C: static unsigned int w32_get_modifiers (void);  */

/* Let the user specify a display with a frame.
   nil stands for the selected frame--or, if that is not a w32 frame,
   the first display on the list.  */

/* C 385-422.  C's prototype lives in `dispextern.h` 3993 (`extern Display_Info
   *check_x_display_info (Lisp_Object);`), which `include/dispextern.rs` does not carry yet - the
   ledger's owed list names it.  `t->display_info.w32` is `*mut c_void` in the port's
   `termhooks.rs` (the C member points at a record that header cannot name, P-72 rule 3), so the
   one return needs C's implicit conversion written out as a cast.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_x_display_info (object: Lisp_Object) -> *mut w32_display_info
{
  if NILP (object)
    {
      let mut sf: *mut frame = XFRAME (selected_frame);

      if FRAME_W32_P (sf) && FRAME_LIVE_P (sf)
	{ return FRAME_DISPLAY_INFO! (sf); }
      else
	{ return &raw mut one_w32_display_info; }
    }
  else if TERMINALP (object)
    {
      let mut t: *mut terminal = decode_live_terminal (object);

      if (*t).r#type != output_w32
	{ error (c"Terminal %d is not a W32 display".as_ptr (), (*t).id); }

      return (*t).display_info.w32 as *mut w32_display_info;
    }
  else if STRINGP (object)
    { return w32_display_info_for_name (object); }
  else
    {
      let mut f: *mut frame;

      CHECK_LIVE_FRAME! (object);
      f = XFRAME (object);
      if ! FRAME_W32_P (f)
	{ error (c"Non-W32 frame used".as_ptr ()); }
      return FRAME_DISPLAY_INFO! (f);
    }
}

/* `termhooks.h` 963: `extern struct terminal *decode_live_terminal (Lisp_Object);` - the mirror
   does not carry it ("each is declared at its use site", termhooks.rs 434), so this unit declares
   it here (P-72 rule 4).  */
/// C: extern struct terminal *decode_live_terminal (Lisp_Object);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
}

/* Return the Emacs frame-object corresponding to an w32 window.
   It could be the frame's main window or an icon window.  */

/* C 424-443.  `w32term.h` 230 declares this entry point and this file defines it - one of the five
   names the ledger asks the integrator to move into a `w32term_decls` module (E0428 otherwise; the
   `include/window.rs` 302-367 and `include/androidterm.rs` 544-644 precedent).  C's `return 0;`
   for a pointer is written as C's own null-pointer constant `0` cast to the pointee, so C's token
   survives (the ledger records it).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_window_to_frame (dpyinfo: *mut w32_display_info,
                                              wdesc: HWND) -> *mut frame
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  FOR_EACH_FRAME! (tail, frame, {
    f = XFRAME (frame);
    if ! FRAME_W32_P (f) || FRAME_DISPLAY_INFO! (f) != dpyinfo
      { continue; }

    if FRAME_W32_WINDOW! (f) == wdesc
      { return f; }
  });
  return 0 as *mut frame;
}

/* C 445-461: eleven more forward declarations of `static` functions and of the two other units'
   entries; like C 380-383 they have no Rust counterpart (PORTING.md 0.1-12), and their definitions
   stand at C's positions (past this slice).
   C: static Lisp_Object unwind_create_frame (Lisp_Object);
   C: static void unwind_create_tip_frame (Lisp_Object);
   C: static void my_create_window (struct frame *);
   C: static void my_create_tip_window (struct frame *);
   C: static void w32_set_foreground_color (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_background_color (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_mouse_color (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_border_color (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_cursor_color (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_cursor_type (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_icon_type (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_icon_name (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_explicitly_set_name (struct frame *, Lisp_Object, Lisp_Object);
   C: static void w32_set_title (struct frame *, Lisp_Object, Lisp_Object);  */

/* Store the screen positions of frame F into XPTR and YPTR.
   These are the positions of the containing window manager window,
   not Emacs's own window.  */

/* C 463-488.  `w32term.h` 232 declares it and this file defines it (the mirror collision listed in
   the ledger).  C's `(LPPOINT) &rect` is the port's `as LPPOINT` on the address, C's
   `FRAME_PARENT_FRAME` is the mirror's inline function (frame.rs 413), and the two handles are the
   macros `w32term.rs` 560-563 carries.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_real_positions (f: *mut frame, xptr: *mut c_int, yptr: *mut c_int)
{
  let mut rect: RECT = core::mem::zeroed ();

  /* Get the bounds of the WM window.  */
  GetWindowRect (FRAME_W32_WINDOW! (f), &raw mut rect);

  if ! FRAME_PARENT_FRAME (f).is_null ()
    {
      /* For a child window we have to get its coordinates wrt its
	 parent.  */
      let mut parent_hwnd: HWND = FRAME_W32_WINDOW! (FRAME_PARENT_FRAME (f));

      if ! parent_hwnd.is_null ()
	{ MapWindowPoints (HWND_DESKTOP, parent_hwnd, (&raw mut rect) as LPPOINT, 2); }
    }

  *xptr = rect.left;
  *yptr = rect.top;
}

/* Returns the window rectangle appropriate for the given fullscreen mode.
   The normal rect parameter was the window's rectangle prior to entering
   fullscreen mode.  If multiple monitor support is available, the nearest
   monitor to the window is chosen.  */

/* C 490-541.  C's `struct MONITOR_INFO mi = { sizeof(mi) };` sets the first member and leaves the
   rest zero, which Rust spells out (PORTING.md 0.1-13), and C's two function-pointer tests are the
   port's `Option` tests with C's `.unwrap ()` call shape at the two call sites.  C's `switch` is
   the port's `match`: the three `case`/`break` pairs become arms, and the `switch`/`case`/`break`/
   `default` tokens are the row the ledger asks the integrator to add to
   `docs/ports/macro-body-notes.tsv` (the `set_internal` row is the precedent).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_fullscreen_rect (hwnd: HWND, fsmode: c_int, normal: RECT,
                                              rect: *mut RECT)
{
  let mut mi: MONITOR_INFO
    = MONITOR_INFO { cbSize: core::mem::size_of::<MONITOR_INFO> () as DWORD,
		     rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
		     rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
		     dwFlags: 0 };
  if monitor_from_window_fn.is_some () && get_monitor_info_fn.is_some ()
    {
      let mut monitor: HMONITOR =
	(monitor_from_window_fn.unwrap ()) (hwnd, MONITOR_DEFAULT_TO_NEAREST);
      (get_monitor_info_fn.unwrap ()) (monitor, &raw mut mi);
    }
  else
    {
      mi.rcMonitor.left = 0;
      mi.rcMonitor.top = 0;
      mi.rcMonitor.right = GetSystemMetrics (SM_CXSCREEN);
      mi.rcMonitor.bottom = GetSystemMetrics (SM_CYSCREEN);
      mi.rcWork.left = 0;
      mi.rcWork.top = 0;
      mi.rcWork.right = GetSystemMetrics (SM_CXMAXIMIZED);
      mi.rcWork.bottom = GetSystemMetrics (SM_CYMAXIMIZED);
    }

  match fsmode
    {
    FULLSCREEN_BOTH =>
      {
	(*rect).left = mi.rcMonitor.left;
	(*rect).top = mi.rcMonitor.top;
	(*rect).right = mi.rcMonitor.right;
	(*rect).bottom = mi.rcMonitor.bottom;
      }
    FULLSCREEN_WIDTH =>
      {
	(*rect).left = mi.rcWork.left;
	(*rect).top = normal.top;
	(*rect).right = mi.rcWork.right;
	(*rect).bottom = normal.bottom;
      }
    FULLSCREEN_HEIGHT =>
      {
	(*rect).left = normal.left;
	(*rect).top = mi.rcWork.top;
	(*rect).right = normal.right;
	(*rect).bottom = mi.rcWork.bottom;
      }
    _ =>
      {
	*rect = normal;
      }
    }
}


#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-define-rgb-color", Fw32_define_rgb_color,
       Sw32_define_rgb_color, 4, 4, 0,
       doc: "Convert RGB numbers to a Windows color reference and associate with NAME.\nThis adds or updates a named color to `w32-color-map', making it\navailable for use.  The original entry's RGB ref is returned, or nil\nif the entry is new.",
       (red: Lisp_Object, green: Lisp_Object, blue: Lisp_Object, name: Lisp_Object),
       {
  let mut rgb: Lisp_Object;
  let mut oldrgb: Lisp_Object = Qnil ();
  let mut entry: Lisp_Object;

  CHECK_FIXNUM (red);
  CHECK_FIXNUM (green);
  CHECK_FIXNUM (blue);
  CHECK_STRING (name);

  /* C's implicit `COLORREF` (a 32-bit `DWORD`) to `EMACS_INT` conversion, written out: the port's
     `make_fixnum` takes an `EMACS_INT` (lisp.rs 724), and C's own promotion is value-preserving
     here.  */
  XSETINT! (rgb, RGB! (XUFIXNUM (red), XUFIXNUM (green), XUFIXNUM (blue)) as EMACS_INT);

  block_input ();

  /* replace existing entry in w32-color-map or add new entry. */
  entry = Fassoc (name, Vw32_color_map, Qnil ());
  if NILP (entry)
    {
      entry = Fcons (name, rgb);
      Vw32_color_map = Fcons (entry, Vw32_color_map);
    }
  else
    {
      oldrgb = Fcdr (entry);
      Fsetcdr (entry, rgb);
    }

  unblock_input ();

  return (oldrgb);
});


/// C 585-589: C's `typedef struct colormap_t { const char *name; COLORREF colorref; }`.
/// C: typedef struct colormap_t { const char *name; COLORREF colorref; } colormap_t;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct colormap_t
{
  pub name: *const c_char,
  pub colorref: COLORREF,
}

/* C 591-833: "The default colors for the w32 color map".  C's own `static` (internal linkage) is
   the port's plain `static mut`, C's 240 initializers are kept verbatim - each `colormap_t` names
   C's string and C's `PALETTERGB (...)` arguments line by line (`NAME! (...)` is how this tree
   spells every C macro call) - and C's empty array bound is the initializer count.  */
#[cfg (feature = "have_ntgui")]
static mut w32_color_map: [colormap_t; 240] =
[
  colormap_t { name: c"snow".as_ptr (), colorref: PALETTERGB! (255,250,250) },   // C 593
  colormap_t { name: c"ghost white".as_ptr (), colorref: PALETTERGB! (248,248,255) },   // C 594
  colormap_t { name: c"GhostWhite".as_ptr (), colorref: PALETTERGB! (248,248,255) },   // C 595
  colormap_t { name: c"white smoke".as_ptr (), colorref: PALETTERGB! (245,245,245) },   // C 596
  colormap_t { name: c"WhiteSmoke".as_ptr (), colorref: PALETTERGB! (245,245,245) },   // C 597
  colormap_t { name: c"gainsboro".as_ptr (), colorref: PALETTERGB! (220,220,220) },   // C 598
  colormap_t { name: c"floral white".as_ptr (), colorref: PALETTERGB! (255,250,240) },   // C 599
  colormap_t { name: c"FloralWhite".as_ptr (), colorref: PALETTERGB! (255,250,240) },   // C 600
  colormap_t { name: c"old lace".as_ptr (), colorref: PALETTERGB! (253,245,230) },   // C 601
  colormap_t { name: c"OldLace".as_ptr (), colorref: PALETTERGB! (253,245,230) },   // C 602
  colormap_t { name: c"linen".as_ptr (), colorref: PALETTERGB! (250,240,230) },   // C 603
  colormap_t { name: c"antique white".as_ptr (), colorref: PALETTERGB! (250,235,215) },   // C 604
  colormap_t { name: c"AntiqueWhite".as_ptr (), colorref: PALETTERGB! (250,235,215) },   // C 605
  colormap_t { name: c"papaya whip".as_ptr (), colorref: PALETTERGB! (255,239,213) },   // C 606
  colormap_t { name: c"PapayaWhip".as_ptr (), colorref: PALETTERGB! (255,239,213) },   // C 607
  colormap_t { name: c"blanched almond".as_ptr (), colorref: PALETTERGB! (255,235,205) },   // C 608
  colormap_t { name: c"BlanchedAlmond".as_ptr (), colorref: PALETTERGB! (255,235,205) },   // C 609
  colormap_t { name: c"bisque".as_ptr (), colorref: PALETTERGB! (255,228,196) },   // C 610
  colormap_t { name: c"peach puff".as_ptr (), colorref: PALETTERGB! (255,218,185) },   // C 611
  colormap_t { name: c"PeachPuff".as_ptr (), colorref: PALETTERGB! (255,218,185) },   // C 612
  colormap_t { name: c"navajo white".as_ptr (), colorref: PALETTERGB! (255,222,173) },   // C 613
  colormap_t { name: c"NavajoWhite".as_ptr (), colorref: PALETTERGB! (255,222,173) },   // C 614
  colormap_t { name: c"moccasin".as_ptr (), colorref: PALETTERGB! (255,228,181) },   // C 615
  colormap_t { name: c"cornsilk".as_ptr (), colorref: PALETTERGB! (255,248,220) },   // C 616
  colormap_t { name: c"ivory".as_ptr (), colorref: PALETTERGB! (255,255,240) },   // C 617
  colormap_t { name: c"lemon chiffon".as_ptr (), colorref: PALETTERGB! (255,250,205) },   // C 618
  colormap_t { name: c"LemonChiffon".as_ptr (), colorref: PALETTERGB! (255,250,205) },   // C 619
  colormap_t { name: c"seashell".as_ptr (), colorref: PALETTERGB! (255,245,238) },   // C 620
  colormap_t { name: c"honeydew".as_ptr (), colorref: PALETTERGB! (240,255,240) },   // C 621
  colormap_t { name: c"mint cream".as_ptr (), colorref: PALETTERGB! (245,255,250) },   // C 622
  colormap_t { name: c"MintCream".as_ptr (), colorref: PALETTERGB! (245,255,250) },   // C 623
  colormap_t { name: c"azure".as_ptr (), colorref: PALETTERGB! (240,255,255) },   // C 624
  colormap_t { name: c"alice blue".as_ptr (), colorref: PALETTERGB! (240,248,255) },   // C 625
  colormap_t { name: c"AliceBlue".as_ptr (), colorref: PALETTERGB! (240,248,255) },   // C 626
  colormap_t { name: c"lavender".as_ptr (), colorref: PALETTERGB! (230,230,250) },   // C 627
  colormap_t { name: c"lavender blush".as_ptr (), colorref: PALETTERGB! (255,240,245) },   // C 628
  colormap_t { name: c"LavenderBlush".as_ptr (), colorref: PALETTERGB! (255,240,245) },   // C 629
  colormap_t { name: c"misty rose".as_ptr (), colorref: PALETTERGB! (255,228,225) },   // C 630
  colormap_t { name: c"MistyRose".as_ptr (), colorref: PALETTERGB! (255,228,225) },   // C 631
  colormap_t { name: c"white".as_ptr (), colorref: PALETTERGB! (255,255,255) },   // C 632
  colormap_t { name: c"black".as_ptr (), colorref: PALETTERGB! (  0,  0,  0) },   // C 633
  colormap_t { name: c"dark slate gray".as_ptr (), colorref: PALETTERGB! ( 47, 79, 79) },   // C 634
  colormap_t { name: c"DarkSlateGray".as_ptr (), colorref: PALETTERGB! ( 47, 79, 79) },   // C 635
  colormap_t { name: c"dark slate grey".as_ptr (), colorref: PALETTERGB! ( 47, 79, 79) },   // C 636
  colormap_t { name: c"DarkSlateGrey".as_ptr (), colorref: PALETTERGB! ( 47, 79, 79) },   // C 637
  colormap_t { name: c"dim gray".as_ptr (), colorref: PALETTERGB! (105,105,105) },   // C 638
  colormap_t { name: c"DimGray".as_ptr (), colorref: PALETTERGB! (105,105,105) },   // C 639
  colormap_t { name: c"dim grey".as_ptr (), colorref: PALETTERGB! (105,105,105) },   // C 640
  colormap_t { name: c"DimGrey".as_ptr (), colorref: PALETTERGB! (105,105,105) },   // C 641
  colormap_t { name: c"slate gray".as_ptr (), colorref: PALETTERGB! (112,128,144) },   // C 642
  colormap_t { name: c"SlateGray".as_ptr (), colorref: PALETTERGB! (112,128,144) },   // C 643
  colormap_t { name: c"slate grey".as_ptr (), colorref: PALETTERGB! (112,128,144) },   // C 644
  colormap_t { name: c"SlateGrey".as_ptr (), colorref: PALETTERGB! (112,128,144) },   // C 645
  colormap_t { name: c"light slate gray".as_ptr (), colorref: PALETTERGB! (119,136,153) },   // C 646
  colormap_t { name: c"LightSlateGray".as_ptr (), colorref: PALETTERGB! (119,136,153) },   // C 647
  colormap_t { name: c"light slate grey".as_ptr (), colorref: PALETTERGB! (119,136,153) },   // C 648
  colormap_t { name: c"LightSlateGrey".as_ptr (), colorref: PALETTERGB! (119,136,153) },   // C 649
  colormap_t { name: c"gray".as_ptr (), colorref: PALETTERGB! (190,190,190) },   // C 650
  colormap_t { name: c"grey".as_ptr (), colorref: PALETTERGB! (190,190,190) },   // C 651
  colormap_t { name: c"light grey".as_ptr (), colorref: PALETTERGB! (211,211,211) },   // C 652
  colormap_t { name: c"LightGrey".as_ptr (), colorref: PALETTERGB! (211,211,211) },   // C 653
  colormap_t { name: c"light gray".as_ptr (), colorref: PALETTERGB! (211,211,211) },   // C 654
  colormap_t { name: c"LightGray".as_ptr (), colorref: PALETTERGB! (211,211,211) },   // C 655
  colormap_t { name: c"midnight blue".as_ptr (), colorref: PALETTERGB! ( 25, 25,112) },   // C 656
  colormap_t { name: c"MidnightBlue".as_ptr (), colorref: PALETTERGB! ( 25, 25,112) },   // C 657
  colormap_t { name: c"navy".as_ptr (), colorref: PALETTERGB! (  0,  0,128) },   // C 658
  colormap_t { name: c"navy blue".as_ptr (), colorref: PALETTERGB! (  0,  0,128) },   // C 659
  colormap_t { name: c"NavyBlue".as_ptr (), colorref: PALETTERGB! (  0,  0,128) },   // C 660
  colormap_t { name: c"cornflower blue".as_ptr (), colorref: PALETTERGB! (100,149,237) },   // C 661
  colormap_t { name: c"CornflowerBlue".as_ptr (), colorref: PALETTERGB! (100,149,237) },   // C 662
  colormap_t { name: c"dark slate blue".as_ptr (), colorref: PALETTERGB! ( 72, 61,139) },   // C 663
  colormap_t { name: c"DarkSlateBlue".as_ptr (), colorref: PALETTERGB! ( 72, 61,139) },   // C 664
  colormap_t { name: c"slate blue".as_ptr (), colorref: PALETTERGB! (106, 90,205) },   // C 665
  colormap_t { name: c"SlateBlue".as_ptr (), colorref: PALETTERGB! (106, 90,205) },   // C 666
  colormap_t { name: c"medium slate blue".as_ptr (), colorref: PALETTERGB! (123,104,238) },   // C 667
  colormap_t { name: c"MediumSlateBlue".as_ptr (), colorref: PALETTERGB! (123,104,238) },   // C 668
  colormap_t { name: c"light slate blue".as_ptr (), colorref: PALETTERGB! (132,112,255) },   // C 669
  colormap_t { name: c"LightSlateBlue".as_ptr (), colorref: PALETTERGB! (132,112,255) },   // C 670
  colormap_t { name: c"medium blue".as_ptr (), colorref: PALETTERGB! (  0,  0,205) },   // C 671
  colormap_t { name: c"MediumBlue".as_ptr (), colorref: PALETTERGB! (  0,  0,205) },   // C 672
  colormap_t { name: c"royal blue".as_ptr (), colorref: PALETTERGB! ( 65,105,225) },   // C 673
  colormap_t { name: c"RoyalBlue".as_ptr (), colorref: PALETTERGB! ( 65,105,225) },   // C 674
  colormap_t { name: c"blue".as_ptr (), colorref: PALETTERGB! (  0,  0,255) },   // C 675
  colormap_t { name: c"dodger blue".as_ptr (), colorref: PALETTERGB! ( 30,144,255) },   // C 676
  colormap_t { name: c"DodgerBlue".as_ptr (), colorref: PALETTERGB! ( 30,144,255) },   // C 677
  colormap_t { name: c"deep sky blue".as_ptr (), colorref: PALETTERGB! (  0,191,255) },   // C 678
  colormap_t { name: c"DeepSkyBlue".as_ptr (), colorref: PALETTERGB! (  0,191,255) },   // C 679
  colormap_t { name: c"sky blue".as_ptr (), colorref: PALETTERGB! (135,206,235) },   // C 680
  colormap_t { name: c"SkyBlue".as_ptr (), colorref: PALETTERGB! (135,206,235) },   // C 681
  colormap_t { name: c"light sky blue".as_ptr (), colorref: PALETTERGB! (135,206,250) },   // C 682
  colormap_t { name: c"LightSkyBlue".as_ptr (), colorref: PALETTERGB! (135,206,250) },   // C 683
  colormap_t { name: c"steel blue".as_ptr (), colorref: PALETTERGB! ( 70,130,180) },   // C 684
  colormap_t { name: c"SteelBlue".as_ptr (), colorref: PALETTERGB! ( 70,130,180) },   // C 685
  colormap_t { name: c"light steel blue".as_ptr (), colorref: PALETTERGB! (176,196,222) },   // C 686
  colormap_t { name: c"LightSteelBlue".as_ptr (), colorref: PALETTERGB! (176,196,222) },   // C 687
  colormap_t { name: c"light blue".as_ptr (), colorref: PALETTERGB! (173,216,230) },   // C 688
  colormap_t { name: c"LightBlue".as_ptr (), colorref: PALETTERGB! (173,216,230) },   // C 689
  colormap_t { name: c"powder blue".as_ptr (), colorref: PALETTERGB! (176,224,230) },   // C 690
  colormap_t { name: c"PowderBlue".as_ptr (), colorref: PALETTERGB! (176,224,230) },   // C 691
  colormap_t { name: c"pale turquoise".as_ptr (), colorref: PALETTERGB! (175,238,238) },   // C 692
  colormap_t { name: c"PaleTurquoise".as_ptr (), colorref: PALETTERGB! (175,238,238) },   // C 693
  colormap_t { name: c"dark turquoise".as_ptr (), colorref: PALETTERGB! (  0,206,209) },   // C 694
  colormap_t { name: c"DarkTurquoise".as_ptr (), colorref: PALETTERGB! (  0,206,209) },   // C 695
  colormap_t { name: c"medium turquoise".as_ptr (), colorref: PALETTERGB! ( 72,209,204) },   // C 696
  colormap_t { name: c"MediumTurquoise".as_ptr (), colorref: PALETTERGB! ( 72,209,204) },   // C 697
  colormap_t { name: c"turquoise".as_ptr (), colorref: PALETTERGB! ( 64,224,208) },   // C 698
  colormap_t { name: c"cyan".as_ptr (), colorref: PALETTERGB! (  0,255,255) },   // C 699
  colormap_t { name: c"light cyan".as_ptr (), colorref: PALETTERGB! (224,255,255) },   // C 700
  colormap_t { name: c"LightCyan".as_ptr (), colorref: PALETTERGB! (224,255,255) },   // C 701
  colormap_t { name: c"cadet blue".as_ptr (), colorref: PALETTERGB! ( 95,158,160) },   // C 702
  colormap_t { name: c"CadetBlue".as_ptr (), colorref: PALETTERGB! ( 95,158,160) },   // C 703
  colormap_t { name: c"medium aquamarine".as_ptr (), colorref: PALETTERGB! (102,205,170) },   // C 704
  colormap_t { name: c"MediumAquamarine".as_ptr (), colorref: PALETTERGB! (102,205,170) },   // C 705
  colormap_t { name: c"aquamarine".as_ptr (), colorref: PALETTERGB! (127,255,212) },   // C 706
  colormap_t { name: c"dark green".as_ptr (), colorref: PALETTERGB! (  0,100,  0) },   // C 707
  colormap_t { name: c"DarkGreen".as_ptr (), colorref: PALETTERGB! (  0,100,  0) },   // C 708
  colormap_t { name: c"dark olive green".as_ptr (), colorref: PALETTERGB! ( 85,107, 47) },   // C 709
  colormap_t { name: c"DarkOliveGreen".as_ptr (), colorref: PALETTERGB! ( 85,107, 47) },   // C 710
  colormap_t { name: c"dark sea green".as_ptr (), colorref: PALETTERGB! (143,188,143) },   // C 711
  colormap_t { name: c"DarkSeaGreen".as_ptr (), colorref: PALETTERGB! (143,188,143) },   // C 712
  colormap_t { name: c"sea green".as_ptr (), colorref: PALETTERGB! ( 46,139, 87) },   // C 713
  colormap_t { name: c"SeaGreen".as_ptr (), colorref: PALETTERGB! ( 46,139, 87) },   // C 714
  colormap_t { name: c"medium sea green".as_ptr (), colorref: PALETTERGB! ( 60,179,113) },   // C 715
  colormap_t { name: c"MediumSeaGreen".as_ptr (), colorref: PALETTERGB! ( 60,179,113) },   // C 716
  colormap_t { name: c"light sea green".as_ptr (), colorref: PALETTERGB! ( 32,178,170) },   // C 717
  colormap_t { name: c"LightSeaGreen".as_ptr (), colorref: PALETTERGB! ( 32,178,170) },   // C 718
  colormap_t { name: c"pale green".as_ptr (), colorref: PALETTERGB! (152,251,152) },   // C 719
  colormap_t { name: c"PaleGreen".as_ptr (), colorref: PALETTERGB! (152,251,152) },   // C 720
  colormap_t { name: c"spring green".as_ptr (), colorref: PALETTERGB! (  0,255,127) },   // C 721
  colormap_t { name: c"SpringGreen".as_ptr (), colorref: PALETTERGB! (  0,255,127) },   // C 722
  colormap_t { name: c"lawn green".as_ptr (), colorref: PALETTERGB! (124,252,  0) },   // C 723
  colormap_t { name: c"LawnGreen".as_ptr (), colorref: PALETTERGB! (124,252,  0) },   // C 724
  colormap_t { name: c"green".as_ptr (), colorref: PALETTERGB! (  0,255,  0) },   // C 725
  colormap_t { name: c"chartreuse".as_ptr (), colorref: PALETTERGB! (127,255,  0) },   // C 726
  colormap_t { name: c"medium spring green".as_ptr (), colorref: PALETTERGB! (  0,250,154) },   // C 727
  colormap_t { name: c"MediumSpringGreen".as_ptr (), colorref: PALETTERGB! (  0,250,154) },   // C 728
  colormap_t { name: c"green yellow".as_ptr (), colorref: PALETTERGB! (173,255, 47) },   // C 729
  colormap_t { name: c"GreenYellow".as_ptr (), colorref: PALETTERGB! (173,255, 47) },   // C 730
  colormap_t { name: c"lime green".as_ptr (), colorref: PALETTERGB! ( 50,205, 50) },   // C 731
  colormap_t { name: c"LimeGreen".as_ptr (), colorref: PALETTERGB! ( 50,205, 50) },   // C 732
  colormap_t { name: c"yellow green".as_ptr (), colorref: PALETTERGB! (154,205, 50) },   // C 733
  colormap_t { name: c"YellowGreen".as_ptr (), colorref: PALETTERGB! (154,205, 50) },   // C 734
  colormap_t { name: c"forest green".as_ptr (), colorref: PALETTERGB! ( 34,139, 34) },   // C 735
  colormap_t { name: c"ForestGreen".as_ptr (), colorref: PALETTERGB! ( 34,139, 34) },   // C 736
  colormap_t { name: c"olive drab".as_ptr (), colorref: PALETTERGB! (107,142, 35) },   // C 737
  colormap_t { name: c"OliveDrab".as_ptr (), colorref: PALETTERGB! (107,142, 35) },   // C 738
  colormap_t { name: c"dark khaki".as_ptr (), colorref: PALETTERGB! (189,183,107) },   // C 739
  colormap_t { name: c"DarkKhaki".as_ptr (), colorref: PALETTERGB! (189,183,107) },   // C 740
  colormap_t { name: c"khaki".as_ptr (), colorref: PALETTERGB! (240,230,140) },   // C 741
  colormap_t { name: c"pale goldenrod".as_ptr (), colorref: PALETTERGB! (238,232,170) },   // C 742
  colormap_t { name: c"PaleGoldenrod".as_ptr (), colorref: PALETTERGB! (238,232,170) },   // C 743
  colormap_t { name: c"light goldenrod yellow".as_ptr (), colorref: PALETTERGB! (250,250,210) },   // C 744
  colormap_t { name: c"LightGoldenrodYellow".as_ptr (), colorref: PALETTERGB! (250,250,210) },   // C 745
  colormap_t { name: c"light yellow".as_ptr (), colorref: PALETTERGB! (255,255,224) },   // C 746
  colormap_t { name: c"LightYellow".as_ptr (), colorref: PALETTERGB! (255,255,224) },   // C 747
  colormap_t { name: c"yellow".as_ptr (), colorref: PALETTERGB! (255,255,  0) },   // C 748
  colormap_t { name: c"gold".as_ptr (), colorref: PALETTERGB! (255,215,  0) },   // C 749
  colormap_t { name: c"light goldenrod".as_ptr (), colorref: PALETTERGB! (238,221,130) },   // C 750
  colormap_t { name: c"LightGoldenrod".as_ptr (), colorref: PALETTERGB! (238,221,130) },   // C 751
  colormap_t { name: c"goldenrod".as_ptr (), colorref: PALETTERGB! (218,165, 32) },   // C 752
  colormap_t { name: c"dark goldenrod".as_ptr (), colorref: PALETTERGB! (184,134, 11) },   // C 753
  colormap_t { name: c"DarkGoldenrod".as_ptr (), colorref: PALETTERGB! (184,134, 11) },   // C 754
  colormap_t { name: c"rosy brown".as_ptr (), colorref: PALETTERGB! (188,143,143) },   // C 755
  colormap_t { name: c"RosyBrown".as_ptr (), colorref: PALETTERGB! (188,143,143) },   // C 756
  colormap_t { name: c"indian red".as_ptr (), colorref: PALETTERGB! (205, 92, 92) },   // C 757
  colormap_t { name: c"IndianRed".as_ptr (), colorref: PALETTERGB! (205, 92, 92) },   // C 758
  colormap_t { name: c"saddle brown".as_ptr (), colorref: PALETTERGB! (139, 69, 19) },   // C 759
  colormap_t { name: c"SaddleBrown".as_ptr (), colorref: PALETTERGB! (139, 69, 19) },   // C 760
  colormap_t { name: c"sienna".as_ptr (), colorref: PALETTERGB! (160, 82, 45) },   // C 761
  colormap_t { name: c"peru".as_ptr (), colorref: PALETTERGB! (205,133, 63) },   // C 762
  colormap_t { name: c"burlywood".as_ptr (), colorref: PALETTERGB! (222,184,135) },   // C 763
  colormap_t { name: c"beige".as_ptr (), colorref: PALETTERGB! (245,245,220) },   // C 764
  colormap_t { name: c"wheat".as_ptr (), colorref: PALETTERGB! (245,222,179) },   // C 765
  colormap_t { name: c"sandy brown".as_ptr (), colorref: PALETTERGB! (244,164, 96) },   // C 766
  colormap_t { name: c"SandyBrown".as_ptr (), colorref: PALETTERGB! (244,164, 96) },   // C 767
  colormap_t { name: c"tan".as_ptr (), colorref: PALETTERGB! (210,180,140) },   // C 768
  colormap_t { name: c"chocolate".as_ptr (), colorref: PALETTERGB! (210,105, 30) },   // C 769
  colormap_t { name: c"firebrick".as_ptr (), colorref: PALETTERGB! (178,34, 34) },   // C 770
  colormap_t { name: c"brown".as_ptr (), colorref: PALETTERGB! (165,42, 42) },   // C 771
  colormap_t { name: c"dark salmon".as_ptr (), colorref: PALETTERGB! (233,150,122) },   // C 772
  colormap_t { name: c"DarkSalmon".as_ptr (), colorref: PALETTERGB! (233,150,122) },   // C 773
  colormap_t { name: c"salmon".as_ptr (), colorref: PALETTERGB! (250,128,114) },   // C 774
  colormap_t { name: c"light salmon".as_ptr (), colorref: PALETTERGB! (255,160,122) },   // C 775
  colormap_t { name: c"LightSalmon".as_ptr (), colorref: PALETTERGB! (255,160,122) },   // C 776
  colormap_t { name: c"orange".as_ptr (), colorref: PALETTERGB! (255,165,  0) },   // C 777
  colormap_t { name: c"dark orange".as_ptr (), colorref: PALETTERGB! (255,140,  0) },   // C 778
  colormap_t { name: c"DarkOrange".as_ptr (), colorref: PALETTERGB! (255,140,  0) },   // C 779
  colormap_t { name: c"coral".as_ptr (), colorref: PALETTERGB! (255,127, 80) },   // C 780
  colormap_t { name: c"light coral".as_ptr (), colorref: PALETTERGB! (240,128,128) },   // C 781
  colormap_t { name: c"LightCoral".as_ptr (), colorref: PALETTERGB! (240,128,128) },   // C 782
  colormap_t { name: c"tomato".as_ptr (), colorref: PALETTERGB! (255, 99, 71) },   // C 783
  colormap_t { name: c"orange red".as_ptr (), colorref: PALETTERGB! (255, 69,  0) },   // C 784
  colormap_t { name: c"OrangeRed".as_ptr (), colorref: PALETTERGB! (255, 69,  0) },   // C 785
  colormap_t { name: c"red".as_ptr (), colorref: PALETTERGB! (255,  0,  0) },   // C 786
  colormap_t { name: c"hot pink".as_ptr (), colorref: PALETTERGB! (255,105,180) },   // C 787
  colormap_t { name: c"HotPink".as_ptr (), colorref: PALETTERGB! (255,105,180) },   // C 788
  colormap_t { name: c"deep pink".as_ptr (), colorref: PALETTERGB! (255, 20,147) },   // C 789
  colormap_t { name: c"DeepPink".as_ptr (), colorref: PALETTERGB! (255, 20,147) },   // C 790
  colormap_t { name: c"pink".as_ptr (), colorref: PALETTERGB! (255,192,203) },   // C 791
  colormap_t { name: c"light pink".as_ptr (), colorref: PALETTERGB! (255,182,193) },   // C 792
  colormap_t { name: c"LightPink".as_ptr (), colorref: PALETTERGB! (255,182,193) },   // C 793
  colormap_t { name: c"pale violet red".as_ptr (), colorref: PALETTERGB! (219,112,147) },   // C 794
  colormap_t { name: c"PaleVioletRed".as_ptr (), colorref: PALETTERGB! (219,112,147) },   // C 795
  colormap_t { name: c"maroon".as_ptr (), colorref: PALETTERGB! (176, 48, 96) },   // C 796
  colormap_t { name: c"medium violet red".as_ptr (), colorref: PALETTERGB! (199, 21,133) },   // C 797
  colormap_t { name: c"MediumVioletRed".as_ptr (), colorref: PALETTERGB! (199, 21,133) },   // C 798
  colormap_t { name: c"violet red".as_ptr (), colorref: PALETTERGB! (208, 32,144) },   // C 799
  colormap_t { name: c"VioletRed".as_ptr (), colorref: PALETTERGB! (208, 32,144) },   // C 800
  colormap_t { name: c"magenta".as_ptr (), colorref: PALETTERGB! (255,  0,255) },   // C 801
  colormap_t { name: c"violet".as_ptr (), colorref: PALETTERGB! (238,130,238) },   // C 802
  colormap_t { name: c"plum".as_ptr (), colorref: PALETTERGB! (221,160,221) },   // C 803
  colormap_t { name: c"orchid".as_ptr (), colorref: PALETTERGB! (218,112,214) },   // C 804
  colormap_t { name: c"medium orchid".as_ptr (), colorref: PALETTERGB! (186, 85,211) },   // C 805
  colormap_t { name: c"MediumOrchid".as_ptr (), colorref: PALETTERGB! (186, 85,211) },   // C 806
  colormap_t { name: c"dark orchid".as_ptr (), colorref: PALETTERGB! (153, 50,204) },   // C 807
  colormap_t { name: c"DarkOrchid".as_ptr (), colorref: PALETTERGB! (153, 50,204) },   // C 808
  colormap_t { name: c"dark violet".as_ptr (), colorref: PALETTERGB! (148,  0,211) },   // C 809
  colormap_t { name: c"DarkViolet".as_ptr (), colorref: PALETTERGB! (148,  0,211) },   // C 810
  colormap_t { name: c"blue violet".as_ptr (), colorref: PALETTERGB! (138, 43,226) },   // C 811
  colormap_t { name: c"BlueViolet".as_ptr (), colorref: PALETTERGB! (138, 43,226) },   // C 812
  colormap_t { name: c"purple".as_ptr (), colorref: PALETTERGB! (160, 32,240) },   // C 813
  colormap_t { name: c"medium purple".as_ptr (), colorref: PALETTERGB! (147,112,219) },   // C 814
  colormap_t { name: c"MediumPurple".as_ptr (), colorref: PALETTERGB! (147,112,219) },   // C 815
  colormap_t { name: c"thistle".as_ptr (), colorref: PALETTERGB! (216,191,216) },   // C 816
  colormap_t { name: c"gray0".as_ptr (), colorref: PALETTERGB! (  0,  0,  0) },   // C 817
  colormap_t { name: c"grey0".as_ptr (), colorref: PALETTERGB! (  0,  0,  0) },   // C 818
  colormap_t { name: c"dark grey".as_ptr (), colorref: PALETTERGB! (169,169,169) },   // C 819
  colormap_t { name: c"DarkGrey".as_ptr (), colorref: PALETTERGB! (169,169,169) },   // C 820
  colormap_t { name: c"dark gray".as_ptr (), colorref: PALETTERGB! (169,169,169) },   // C 821
  colormap_t { name: c"DarkGray".as_ptr (), colorref: PALETTERGB! (169,169,169) },   // C 822
  colormap_t { name: c"dark blue".as_ptr (), colorref: PALETTERGB! (  0,  0,139) },   // C 823
  colormap_t { name: c"DarkBlue".as_ptr (), colorref: PALETTERGB! (  0,  0,139) },   // C 824
  colormap_t { name: c"dark cyan".as_ptr (), colorref: PALETTERGB! (  0,139,139) },   // C 825
  colormap_t { name: c"DarkCyan".as_ptr (), colorref: PALETTERGB! (  0,139,139) },   // C 826
  colormap_t { name: c"dark magenta".as_ptr (), colorref: PALETTERGB! (139,  0,139) },   // C 827
  colormap_t { name: c"DarkMagenta".as_ptr (), colorref: PALETTERGB! (139,  0,139) },   // C 828
  colormap_t { name: c"dark red".as_ptr (), colorref: PALETTERGB! (139,  0,  0) },   // C 829
  colormap_t { name: c"DarkRed".as_ptr (), colorref: PALETTERGB! (139,  0,  0) },   // C 830
  colormap_t { name: c"light green".as_ptr (), colorref: PALETTERGB! (144,238,144) },   // C 831
  colormap_t { name: c"LightGreen".as_ptr (), colorref: PALETTERGB! (144,238,144) },   // C 832
];


/* C 835-854.  C's `for (i = 0; i < countof (w32_color_map); pc++, i++)` is the port's `while`
   with C's two update expressions at the foot of the body (`countof` is the tree's `countof!`),
   and `pc`'s C declaration `colormap_t *pc = w32_color_map;` is C's array-to-pointer decay.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_default_color_map () -> Lisp_Object
{
  let mut i: c_int = 0;
  let mut pc: *mut colormap_t = &raw mut w32_color_map as *mut colormap_t;
  let mut cmap: Lisp_Object;

  block_input ();

  cmap = Qnil ();

  while i < countof! (w32_color_map) as c_int
    {
      cmap = Fcons (Fcons (build_string ((*pc).name),
			   make_fixnum ((*pc).colorref as EMACS_INT)),
		    cmap);
      pc = pc.add (1);
      i += 1;
    }

  unblock_input ();

  return (cmap);
}

/* C 856-884.  C's `for` runs its update expression on the `continue` path too, so the port's
   `tail = XCDR (tail)` stands before that `continue` and at the foot of the loop (the
   `casefiddle.rs` 485 / `ccl.rs` 1877 readings of the same shape).  C's `register` on the two
   locals is PORTING.md 1.1's no-op.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_color_map_lookup (colorname: *const c_char) -> Lisp_Object
{
  let mut tail: Lisp_Object;
  let mut ret: Lisp_Object = Qnil ();

  block_input ();

  tail = Vw32_color_map;
  while CONSP (tail)
    {
      let mut elt: Lisp_Object;
      let mut tem: Lisp_Object;

      elt = XCAR (tail);
      if ! CONSP (elt) { tail = XCDR (tail); continue; }

      tem = XCAR (elt);

      if lstrcmpi (SSDATA (tem), colorname) == 0
	{
	  ret = Fcdr (elt);
	  break;
	}

      maybe_quit ();

      tail = XCDR (tail);
    }

  unblock_input ();

  return ret;
}


/* ==============================================================================================
   Slice 2: C 887-2536 (`add_system_logical_colors_to_map` through `process_dropfiles`).

   It continues at the C line after the first slice's last row (C 856-884
   `w32_color_map_lookup`, whose `}` is C 884) and stops at a whole-item boundary: C 2536 is the
   `}` of `process_dropfiles`, and C 2541 is `w32_process_dnd_data` (C 2538-2539 is its comment).
   The slice carries the colour/palette family, the twenty-nine `w32_set_*` frame-parameter
   setters, the tool/tab/menu-bar height family, the window-class, cursor and scroll-bar helpers,
   `w32_applytheme` and `process_dropfiles`.  It stops before the COM drop-target half (C
   2538-2700) because that surface has a second consumer in this tree and P-90 ① puts it in a
   dedicated SDK mirror, a decision the next lane's ledger entry records; the 2,500-3,000 C lines
   a lane is asked for are not reached, and `docs/ports/w32fns.md`'s slice-2 section says so.
   The C range table there is the coverage evidence (P-86 ④).

   The file gate (`have_ntgui`) is on every item, and the sub-gates are the ledger's
   `## Gate mapping`: `windowsnt` for C 34x's arms, `glyph_debug`, `cygwin`, `mingw_w64`,
   `win32_windows_ge_0410`.  Nothing here is measured on a Windows target: the machine has no
   Windows SDK and no mingw toolchain (the first slice's `text-only` note), so the `<windows.h>`
   records below are spelled from C's own member lists and from the SDK's published signatures,
   with each name's C line in a `/// C:` line, and every layout is `UNVERIFIED`.
   ============================================================================================== */

/* C 82-95's `<windows.h>`/`<winreg.h>`/`<wingdi.h>` section, continued: the SDK names this
   slice's functions call, in C's use order.  C's own text stands in the `/// C:` line above each
   item (P-88 ①/②; the section's first half is at C 82-95 in the first slice).  */

/// C 891: `char full_name_buffer[MAX_PATH + SYSTEM_COLOR_PREFIX_LEN];` - `<windef.h>`'s path
/// limit (`MAX_PATH`), which `lib/include/limits.in.rs` does not carry and `include/w32term.rs`
/// neither.
/// C: #define MAX_PATH 260
#[cfg (feature = "have_ntgui")]
pub const MAX_PATH: c_int = 260;

/// C 943: `if (parse_color_spec (colorname, &r, &g, &b))` - `dispextern.h` 3798-3800 declares it
/// and `src/xfaces.c` defines it; `include/dispextern.rs` does not carry the declaration yet
/// (the `haikufns.rs` 311 shape declares it at the use site: P-72 rule 4 / P-88 ①).
/// C: dispextern.h 3798-3800: `bool parse_color_spec (const char *, unsigned short *,
/// C: unsigned short *, unsigned short *);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn parse_color_spec (spec: *const c_char, r: *mut c_ushort, g: *mut c_ushort,
                           b: *mut c_ushort) -> bool;
}

/// C 963: `sscanf (color_buffer, " %u %u %u", &r, &g, &b)` - `<stdio.h>`'s entry point and a
/// variadic one; `lib/include/stdio.rs` carries the header's other names but not this one, so the
/// unit declares it (P-88 ①).
/// C: int sscanf (const char *s, const char *format, ...);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn sscanf (s: *const c_char, format: *const c_char, ...) -> c_int;
}

/// C 977/963: the two string helpers this slice uses are `lib/include/string.rs`'s (`strlen`,
/// `strcpy`), reached through `include/lisp.rs` (C 48 includes `lisp.h`, which includes
/// `<string.h>`); no declaration is needed here.

/// C 890-932: `<winreg.h>`'s registry entries (`RegOpenKeyEx`/`RegEnumValueA`/`RegCloseKey`) and
/// the four names the calls read.  `RegOpenKeyEx` is the SDK's `RegOpenKeyExA`/`RegOpenKeyExW`
/// alias (C's text writes the alias and the port keeps C's name - PORTING.md 0.1's rename
/// convention); the ANSI form is the one C's `char` buffers want.
/// C: #define HKEY_CURRENT_USER ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000001))
/// C: #define HKEY_LOCAL_MACHINE ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000002))
/// C: #define KEY_READ ((STANDARD_RIGHTS_READ | KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS |
/// C: KEY_NOTIFY) & (~SYNCHRONIZE))
/// C: #define ERROR_SUCCESS 0L
/// C: WINADVAPI LONG WINAPI RegOpenKeyExA (HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions, REGSAM
/// C: samDesired, PHKEY phkResult);
/// C: WINADVAPI LONG WINAPI RegEnumValueA (HKEY hKey, DWORD dwIndex, LPSTR lpValueName, LPDWORD
/// C: lpcchValueName, LPDWORD lpReserved, LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
/// C: WINADVAPI LONG WINAPI RegCloseKey (HKEY hKey);
#[cfg (feature = "have_ntgui")]
pub type REGSAM = DWORD;
#[cfg (feature = "have_ntgui")]
pub type PHKEY = *mut HKEY;
/// C's `(HKEY) (ULONG_PTR) ((INT_PTR) 0x80000001)`: the handle constants are the pointer-sized
/// unsigned values the SDK defines, cast twice, which is what the port writes.
#[cfg (feature = "have_ntgui")]
pub const HKEY_CURRENT_USER: HKEY = (0x80000001 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const HKEY_LOCAL_MACHINE: HKEY = (0x80000002 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const KEY_READ: REGSAM = 0x20019;
#[cfg (feature = "have_ntgui")]
pub const ERROR_SUCCESS: LONG = 0;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn RegOpenKeyEx (hKey: HKEY, lpSubKey: *const c_char, ulOptions: DWORD,
                       samDesired: REGSAM, phkResult: PHKEY) -> LONG;
  pub fn RegEnumValueA (hKey: HKEY, dwIndex: DWORD, lpValueName: LPSTR,
                        lpcchValueName: *mut DWORD, lpReserved: *mut DWORD,
                        lpType: *mut DWORD, lpData: LPBYTE, lpcbData: *mut DWORD) -> LONG;
  pub fn RegCloseKey (hKey: HKEY) -> LONG;
}

/// C 990-1013: `<wingdi.h>`'s palette record and its two entry points.  `PALETTEENTRY`,
/// `HPALETTE` and `w32_palette_entry` are `include/w32term.rs`'s (C 486-490 read them there).
/// C: typedef struct tagLOGPALETTE { WORD palVersion; WORD palNumEntries;
/// C: PALETTEENTRY palPalEntry[1]; } LOGPALETTE;
/// C: HPALETTE WINAPI CreatePalette (const LOGPALETTE *);
/// C: BOOL WINAPI DeleteObject (HGDIOBJ);
#[cfg (feature = "have_ntgui")]
pub type HGDIOBJ = *mut c_void;
/// C's trailing `PALETTEENTRY palPalEntry[1]` is the pre-C99 flexible-array idiom: the record is
/// allocated with room for `num_colors` entries (C 993-995) and indexed past the first, so the
/// port keeps the declared member and reaches the entries by pointer arithmetic at the write
/// site (C's own `palPalEntry[i]`), because a Rust array index would be bounds-checked at 1.
/// C: typedef struct tagLOGPALETTE { WORD palVersion; WORD palNumEntries;
/// C: PALETTEENTRY palPalEntry[1]; } LOGPALETTE;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGPALETTE
{
  pub palVersion: WORD,
  pub palNumEntries: WORD,
  pub palPalEntry: [PALETTEENTRY; 1],
}
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn CreatePalette (lplgpl: *const LOGPALETTE) -> HPALETTE;
  pub fn DeleteObject (ho: HGDIOBJ) -> BOOL;
}

/* C 887-932.  C's `int index` is passed to `RegEnumValueA`'s `DWORD dwIndex` (C's implicit
   conversion, written out), `NULL, NULL` are the two unused `LPDWORD` outputs (`null_mut ()`,
   which the token check folds to C's `NULL`), and `sizeof (full_name_buffer)` is the port's
   `size_of_val` on the same place expression (the `sizeof` family, `token-aliases.tsv`).  C's
   `char *name_buffer = full_name_buffer + SYSTEM_COLOR_PREFIX_LEN;` is the array-to-pointer
   decay C writes, i.e. the port's `add` on the buffer's pointer.  */
#[cfg (feature = "have_ntgui")]
unsafe fn add_system_logical_colors_to_map (system_colors: *mut Lisp_Object)
{
  let mut colors_key: HKEY = core::ptr::null_mut ();

  /* Other registry operations are done with input blocked.  */
  block_input ();

  /* Look for "Control Panel/Colors" under User and Machine registry
     settings.  */
  if RegOpenKeyEx (HKEY_CURRENT_USER, c"Control Panel\\Colors".as_ptr (), 0,
		   KEY_READ, &raw mut colors_key) == ERROR_SUCCESS
      || RegOpenKeyEx (HKEY_LOCAL_MACHINE, c"Control Panel\\Colors".as_ptr (), 0,
		       KEY_READ, &raw mut colors_key) == ERROR_SUCCESS
    {
      /* List all keys.  */
      let mut color_buffer: [c_char; 64] = [0; 64];
      let mut full_name_buffer: [c_char; MAX_PATH as usize + SYSTEM_COLOR_PREFIX_LEN! ()]
	= [0; MAX_PATH as usize + SYSTEM_COLOR_PREFIX_LEN! ()];
      let mut index: c_int = 0;
      let mut name_size: DWORD = 0;
      let mut color_size: DWORD = 0;
      let name_buffer: *mut c_char
	= full_name_buffer.as_mut_ptr ().add (SYSTEM_COLOR_PREFIX_LEN! ());

      name_size = (core::mem::size_of_val (&full_name_buffer) - SYSTEM_COLOR_PREFIX_LEN! ()) as DWORD;
      color_size = core::mem::size_of_val (&color_buffer) as DWORD;

      strcpy (full_name_buffer.as_mut_ptr (), SYSTEM_COLOR_PREFIX.as_ptr () as *const c_char);

      while RegEnumValueA (colors_key, index as DWORD, name_buffer, &raw mut name_size,
			   core::ptr::null_mut (), core::ptr::null_mut (),
			   color_buffer.as_mut_ptr () as LPBYTE, &raw mut color_size)
	     == ERROR_SUCCESS
	{
	  let mut r: c_uint = 0;
	  let mut g: c_uint = 0;
	  let mut b: c_uint = 0;
	  if sscanf (color_buffer.as_ptr (), c" %u %u %u".as_ptr (),
		     &raw mut r, &raw mut g, &raw mut b) == 3
	    { *system_colors = Fcons (Fcons (build_string (full_name_buffer.as_ptr ()),
					     make_fixnum (RGB! (r, g, b) as EMACS_INT)),
				      *system_colors); }

	  name_size = (core::mem::size_of_val (&full_name_buffer) - SYSTEM_COLOR_PREFIX_LEN! ()) as DWORD;
	  color_size = core::mem::size_of_val (&color_buffer) as DWORD;
	  index += 1;
	}
      RegCloseKey (colors_key);
    }

  unblock_input ();
}

/* C 934-978.  C's `unsigned short r, g, b` promote to `int` in the return expression
   (`(b & 0xff00) << 8 | (g & 0xff00) | r >> 8`), which the port writes out because a `u16`
   shift by 8 would overflow in Rust; `make_fixnum` then takes C's implicit `int` -> `EMACS_INT`.
   C's `char *ptr, *approx = alloca (len + 1);` is the port's stack pool (`sa_stack_alloc`,
   PORTING.md P-05 - C's bare `alloca` at C 961; the row is in
   `docs/ports/macro-body-notes.tsv`), and `*ptr-- = '\0'` is C's two statements
   (`*ptr = '\0'; ptr--;`).  `c_isdigit` is `lib/include/c-ctype.rs`'s `bool` function, so C's
   integer test is written out.  */
#[cfg (feature = "have_ntgui")]
unsafe fn x_to_w32_color (colorname: *const c_char) -> Lisp_Object
{
  let mut ret: Lisp_Object = Qnil ();

  block_input ();

  let mut r: c_ushort = 0;
  let mut g: c_ushort = 0;
  let mut b: c_ushort = 0;
  if parse_color_spec (colorname, &raw mut r, &raw mut g, &raw mut b)
    {
      unblock_input ();
      /* Throw away the low 8 bits and return 0xBBGGRR.  */
      return make_fixnum ((((b as c_int) & 0xff00) << 8
			   | ((g as c_int) & 0xff00)
			   | ((r as c_int) >> 8)) as EMACS_INT);
    }

  /* I am not going to attempt to handle any of the CIE color schemes
     or TekHVC, since I don't know the algorithms for conversion to
     RGB.  */

  /* If we fail to lookup the color name in w32_color_map, then check the
     colorname to see if it can be crudely approximated: If the X color
     ends in a number (e.g., "darkseagreen2"), strip the number and
     return the result of looking up the base color name.  */
  ret = w32_color_map_lookup (colorname);
  if NILP (ret)
    {
      let len: c_int = strlen (colorname) as c_int;

      if c_isdigit (colorname[(len - 1) as usize] as c_int)
	{
	  let mut ptr: *mut c_char;
	  let approx: *mut c_char = sa_stack_alloc ((len + 1) as usize) as *mut c_char;

	  strcpy (approx, colorname);
	  ptr = approx.add ((len - 1) as usize);
	  while ptr > approx && c_isdigit (*ptr as c_int)
	      { *ptr = 0; ptr = ptr.offset (-1); }

	  ret = w32_color_map_lookup (approx);
	}
    }

  unblock_input ();
  return ret;
}

/* C 979-1016.  C's `if (! FRAME_DISPLAY_INFO (f)->has_palette)` is the port's `== 0` comparison
   (the `w32xfns.rs` 560 spelling of the same test).  C's `(LOGPALETTE *) alloca (...)` is the
   stack pool (`sa_stack_alloc`, PORTING.md P-05 - the second `alloca` row of this file), and
   `i++, list = list->next` in the `for` header is written at the foot of the port's `while`.
   `palNumEntries` is a `WORD` (C 486-490's `num_colors` is an `unsigned`), so C's implicit
   narrowing is written out.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_regenerate_palette (f: *mut frame)
{
  let mut list: *mut w32_palette_entry;
  let mut log_palette: *mut LOGPALETTE;
  let mut new_palette: HPALETTE;
  let mut i: c_int;

  /* don't bother trying to create palette if not supported */
  if (*FRAME_DISPLAY_INFO! (f)).has_palette == 0
    { return; }

  log_palette = sa_stack_alloc (core::mem::size_of::<LOGPALETTE> ()
				+ ((*FRAME_DISPLAY_INFO! (f)).num_colors as usize)
				  * core::mem::size_of::<PALETTEENTRY> ()) as *mut LOGPALETTE;
  (*log_palette).palVersion = 0x300;
  (*log_palette).palNumEntries = (*FRAME_DISPLAY_INFO! (f)).num_colors as WORD;

  list = (*FRAME_DISPLAY_INFO! (f)).color_list;
  i = 0;
  while i < (*FRAME_DISPLAY_INFO! (f)).num_colors as c_int
    {
      *(*log_palette).palPalEntry.as_mut_ptr ().add (i as usize) = (*list).entry;
      list = (*list).next;
      i += 1;
    }

  new_palette = CreatePalette (log_palette);

  enter_crit! ();

  if (*FRAME_DISPLAY_INFO! (f)).palette != core::ptr::null_mut ()
    { DeleteObject ((*FRAME_DISPLAY_INFO! (f)).palette); }
  (*FRAME_DISPLAY_INFO! (f)).palette = new_palette;

  /* Realize display palette and garbage all frames. */
  release_frame_dc (f, get_frame_dc (f));

  leave_crit! ();
}

/* C 1019-1093: the `<wingdi.h>` channel accessors and C's two value macros, then the `#if 0`
   region they were kept for.  */

/// C 1019-1027's two accessors: `W32_COLOR` reads the three channels of a `PALETTEENTRY` and
/// `SET_W32_COLOR` is C's `do { … } while (0)` statement macro, which the port writes as a block
/// expression (PORTING.md 2.1).  Both are named by the `#if 0` text below and by
/// `w32_defined_color` (C 1126-1142).
/// C: #define W32_COLOR(pe)  RGB (pe.peRed, pe.peGreen, pe.peBlue)
/// C: #define SET_W32_COLOR(pe, color) do { pe.peRed = GetRValue (color); pe.peGreen =
/// C: GetGValue (color); pe.peBlue = GetBValue (color); pe.peFlags = 0; } while (0)
#[cfg (feature = "have_ntgui")]
macro_rules! W32_COLOR {
  ($pe:expr) => { RGB! (($pe).peRed, ($pe).peGreen, ($pe).peBlue) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! SET_W32_COLOR {
  ($pe:expr, $color:expr) => { {
    ($pe).peRed = GetRValue! ($color);
    ($pe).peGreen = GetGValue! ($color);
    ($pe).peBlue = GetBValue! ($color);
    ($pe).peFlags = 0;
  } };
}

/// C 1020-1027's three `<wingdi.h>` channel accessors, which `W32_COLOR`/`SET_W32_COLOR` and
/// C 1182-1185 expand (`w32term.rs` 1645 declares the same three for its own unit - each unit is
/// its own crate, P-90's seventh clause).
/// C: #define GetRValue(rgb) ((BYTE)(rgb))
/// C: #define GetGValue(rgb) ((BYTE)(((WORD)(rgb)) >> 8))
/// C: #define GetBValue(rgb) ((BYTE)((rgb) >> 16))
#[cfg (feature = "have_ntgui")]
macro_rules! GetRValue { ($rgb:expr) => { (($rgb) as BYTE) }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GetGValue { ($rgb:expr) => { (((($rgb) as WORD) >> 8) as BYTE) }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GetBValue { ($rgb:expr) => { ((($rgb) >> 16) as BYTE) }; }

/// C 1102-1104/1236/1257: `update_face_from_frame_parameter` (`dispextern.h` 3794) and
/// `redraw_frame` (`dispextern.h` 3896) are declared in headers whose mirrors do not carry them,
/// so the unit declares them at the use site (P-88 ①).
/// C: dispextern.h 3794-3795: `void update_face_from_frame_parameter (struct frame *,
/// C: Lisp_Object, Lisp_Object);`
/// C: dispextern.h 3896: `extern void redraw_frame (struct frame *);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn update_face_from_frame_parameter (f: *mut frame, prop: Lisp_Object, new_value: Lisp_Object);
  pub fn redraw_frame (f: *mut frame);
}

/// C 1252/2166/2174/2273/2275: `<winuser.h>`'s window-attribute writer (`SetWindowLong` is the
/// SDK's `SetWindowLongA`/`SetWindowLongW` alias; C's text writes the alias and the port keeps
/// C's name - PORTING.md 0.1's rename convention).
/// C: LONG WINAPI SetWindowLongA (HWND hWnd, int nIndex, LONG dwNewLong);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn SetWindowLong (hwnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;
}

/// frame.h 1375-1376: `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL` are C's plain
/// `foreground_pixel`/`background_pixel` fields.  The mirror has the readers; this unit also
/// writes them (C 1232's `FRAME_FOREGROUND_PIXEL (f) = fg;` and C 1248's assignment), so the
/// two-line pair is declared here at its use site (P-72 rule 3, the `dispnew.rs` 7367 shape).
macro_rules! FRAME_FOREGROUND_PIXEL { ($f:expr) => { (*$f).foreground_pixel }; }
macro_rules! FRAME_BACKGROUND_PIXEL { ($f:expr) => { (*$f).background_pixel }; }

/* C 1029-1093 is `#if 0` ("Keep these around in case we ever want to track color usage."): C's
   own preprocessor never compiles it, so the port keeps C's text as a comment (the `w32term.rs`
   33-34/133-139 reading of its own `#if 0` regions).

#if 0
/* Keep these around in case we ever want to track color usage. */
void
w32_map_color (struct frame *f, COLORREF color)
{
  struct w32_palette_entry * list = FRAME_DISPLAY_INFO (f)->color_list;

  if (NILP (Vw32_enable_palette))
    return;

  /* check if color is already mapped */
  while (list)
    {
      if (W32_COLOR (list->entry) == color)
	{
	  ++list->refcount;
	  return;
	}
      list = list->next;
    }

  /* not already mapped, so add to list and recreate Windows palette */
  list = xmalloc (sizeof (struct w32_palette_entry));
  SET_W32_COLOR (list->entry, color);
  list->refcount = 1;
  list->next = FRAME_DISPLAY_INFO (f)->color_list;
  FRAME_DISPLAY_INFO (f)->color_list = list;
  FRAME_DISPLAY_INFO (f)->num_colors++;

  /* set flag that palette must be regenerated */
  FRAME_DISPLAY_INFO (f)->regen_palette = TRUE;
}

void
w32_unmap_color (struct frame *f, COLORREF color)
{
  struct w32_palette_entry * list = FRAME_DISPLAY_INFO (f)->color_list;
  struct w32_palette_entry **prev = &FRAME_DISPLAY_INFO (f)->color_list;

  if (NILP (Vw32_enable_palette))
    return;

  /* check if color is already mapped */
  while (list)
    {
      if (W32_COLOR (list->entry) == color)
	{
	  if (--list->refcount == 0)
	    {
	      *prev = list->next;
	      xfree (list);
	      FRAME_DISPLAY_INFO (f)->num_colors--;
	      break;
	    }
	  else
	    return;
	}
      prev = &list->next;
      list = list->next;
    }

  /* set flag that palette must be regenerated */
  FRAME_DISPLAY_INFO (f)->regen_palette = TRUE;
}
#endif
*/

/* C 1096-1107.  C's `f->gamma` is a `double` (frame.h), so C's truth test is written as the
   `!= 0.0` comparison the tree uses for doubles; C's `GetRValue (*color) / 255.0` mixes a `BYTE`
   with a double, which the port writes out, and `PALETTERGB`'s arguments are C's `double`
   results, narrowed by the macro's own casts (`include/w32term.rs`'s PALETTERGB reading).  */
/* Gamma-correct COLOR on frame F.  */

#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gamma_correct (f: *mut frame, color: *mut COLORREF)
{
  if (*f).gamma != 0.0
    {
      *color = PALETTERGB! (
	pow (GetRValue! (*color) as c_double / 255.0, (*f).gamma) * 255.0 + 0.5,
	pow (GetGValue! (*color) as c_double / 255.0, (*f).gamma) * 255.0 + 0.5,
	pow (GetBValue! (*color) as c_double / 255.0, (*f).gamma) * 255.0 + 0.5);
    }
}

/* C 1110-1186.  C's `{`-truth tests on the pointer and on the two Lisp values are the port's
   `is_null`/`NILP` forms; `XUFIXNUM`'s `EMACS_UINT` narrows to `COLORREF` where C assigns it to a
   `DWORD` and widens where C compares it with `W32_COLOR`'s `COLORREF` (C's own promotions,
   written out - both measured classes in this tree).  C's `XSETINT (tem, w32_color_ref)` is the
   mirror's `XSETINT!`, whose argument is an `EMACS_INT`.  C's `one_w32_display_info` is
   `include/w32term.rs` 1489's declaration (`w32term.c`'s definition).  C's `return 1;`/`return 0;`
   for its `bool` are the port's `true`/`false` (a `macro-body-notes.tsv` row records the two
   integer tokens); `xmalloc`'s `*mut c_void` is cast to the record C allocates.  */
/* Decide if color named COLOR is valid for the display associated with
   the selected frame; if so, return the rgb values in COLOR_DEF.
   If ALLOC is nonzero, allocate a new colormap cell.  */

#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_defined_color (f: *mut frame, color: *const c_char,
					     color_def: *mut Emacs_Color, alloc_p: bool,
					     _makeIndex: bool) -> bool
{
  let mut tem: Lisp_Object;
  let mut w32_color_ref: COLORREF;

  tem = x_to_w32_color (color);

  if ! NILP (tem)
    {
      if ! f.is_null ()
	{
	  /* Apply gamma correction.  */
	  w32_color_ref = XUFIXNUM (tem) as COLORREF;
	  gamma_correct (f, &raw mut w32_color_ref);
	  XSETINT! (tem, w32_color_ref as EMACS_INT);
	}

      /* Map this color to the palette if it is enabled. */
      if ! NILP (Vw32_enable_palette)
	{
	  let mut entry: *mut w32_palette_entry = one_w32_display_info.color_list;
	  let mut prev: *mut *mut w32_palette_entry
	    = &raw mut one_w32_display_info.color_list;

	  /* check if color is already mapped */
	  while ! entry.is_null ()
	    {
	      if W32_COLOR! ((*entry).entry) as EMACS_UINT == XUFIXNUM (tem)
		{ break; }
	      prev = &raw mut (*entry).next;
	      entry = (*entry).next;
	    }

	  if entry.is_null () && alloc_p
	    {
	      /* not already mapped, so add to list */
	      entry = xmalloc (core::mem::size_of::<w32_palette_entry> ()) as *mut w32_palette_entry;
	      SET_W32_COLOR! ((*entry).entry, XUFIXNUM (tem) as COLORREF);
	      (*entry).next = core::ptr::null_mut ();
	      *prev = entry;
	      one_w32_display_info.num_colors += 1;

	      /* set flag that palette must be regenerated */
	      one_w32_display_info.regen_palette = TRUE;
	    }
	}
      /* Ensure COLORREF value is snapped to nearest color in (default)
	 palette by simulating the PALETTERGB macro.  This works whether
	 or not the display device has a palette. */
      w32_color_ref = (XUFIXNUM (tem) | 0x2000000) as COLORREF;

      (*color_def).pixel = w32_color_ref as c_ulong;
      (*color_def).red = (GetRValue! (w32_color_ref) as c_int * 256) as c_ushort;
      (*color_def).green = (GetGValue! (w32_color_ref) as c_int * 256) as c_ushort;
      (*color_def).blue = (GetBValue! (w32_color_ref) as c_int * 256) as c_ushort;

      return true;
    }
  else
    {
      return false;
    }
}

/* C 1187-1208.  C's `strcmp` is `lib/include/string.rs`'s and the two literals are `c"…"`
   (PORTING.md 2.1's string rule); `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` are
   `include/w32term.rs` 70-78's macros (their `PALETTERGB (...)` bodies are the mirror's owed
   defect - reported, not changed here), and C's `n_planes * n_cbits` arithmetic is `int` on both
   sides, which the port keeps.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_decode_color (f: *mut frame, arg: Lisp_Object, def: c_int) -> c_int
{
  let mut cdef: Emacs_Color = unsafe { core::mem::zeroed () };

  CHECK_STRING (arg);

  if strcmp (SSDATA (arg), c"black".as_ptr ()) == 0
    { return BLACK_PIX_DEFAULT! (f); }
  else if strcmp (SSDATA (arg), c"white".as_ptr ()) == 0
    { return WHITE_PIX_DEFAULT! (f); }

  if ((*FRAME_DISPLAY_INFO! (f)).n_planes * (*FRAME_DISPLAY_INFO! (f)).n_cbits) == 1
    { return def; }

  /* w32_defined_color is responsible for coping with failures
     by looking for a near-miss.  */
  if w32_defined_color (f, SSDATA (arg), &raw mut cdef, true, false)
    { return cdef.pixel as c_int; }

  /* defined_color failed; return an ultimate default.  */
  return def;
}


/* Functions called only from `gui_set_frame_parameters'
   to set individual parameters.

   If FRAME_W32_WINDOW (f) is 0,
   the frame is being created and its window does not exist yet.
   In that case, just record the parameter's new value
   in the standard place; do not attempt to change the window.  */

/* C 1221-1242.  C's `struct w32_output *x = f->output_data.w32;` is the port's cast of the
   union member (`frame.rs` 39 spells it `*mut c_void`, P-72 rule 3), and `x->cursor_gc->background`
   is the `Emacs_GC` member `dispextern.rs` declares.  `PIX_TYPE` is `include/w32term.rs` 360-366's
   type macro, so C's declaration `PIX_TYPE fg, old_fg;` is the port's `PIX_TYPE! ()`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_foreground_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let x: *mut w32_output = (*f).output_data.w32 as *mut w32_output;
  let mut fg: PIX_TYPE! ();
  let mut old_fg: PIX_TYPE! ();

  fg = w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));
  old_fg = FRAME_FOREGROUND_PIXEL! (f);
  FRAME_FOREGROUND_PIXEL! (f) = fg;

  if FRAME_W32_WINDOW! (f) != 0
    {
      if (*x).cursor_pixel == old_fg
	{
	  (*x).cursor_pixel = fg;
	  (*(*x).cursor_gc).background = fg;
	}

      update_face_from_frame_parameter (f, Qforeground_color (), arg);
      if FRAME_VISIBLE_P (f)
	{ redraw_frame (f); }
    }
}

/* C 1245-1260.  C's `SetWindowLong` is `<winuser.h>`'s (declared above) and takes C's `LONG`, so
   the `COLORREF` pixel C passes is the identical 32-bit unsigned value; C's assignment to
   `FRAME_BACKGROUND_PIXEL (f)` is the macro pair declared above (the mirror's reader is a
   function, so the write needs the place expression).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_background_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  FRAME_BACKGROUND_PIXEL! (f)
    = w32_decode_color (f, arg, WHITE_PIX_DEFAULT! (f)) as c_ulong;

  if FRAME_W32_WINDOW! (f) != 0
    {
      SetWindowLong (FRAME_W32_WINDOW! (f), WND_BACKGROUND_INDEX,
		     FRAME_BACKGROUND_PIXEL! (f) as LONG);

      update_face_from_frame_parameter (f, Qbackground_color (), arg);

      if FRAME_VISIBLE_P (f)
	{ redraw_frame (f); }
    }
}

/// C 1486/1503: `set_frame_cursor_types` (`lisp.h` 4409) and `gui_update_cursor`
/// (`dispextern.h` 3656) are declared in headers whose mirrors do not carry them, so the unit
/// declares them at their use site (P-88 ①).
/// C: lisp.h 4409: `void set_frame_cursor_types (struct frame *, Lisp_Object);`
/// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn set_frame_cursor_types (f: *mut frame, arg: Lisp_Object);
  pub fn gui_update_cursor (f: *mut frame, on_p: bool);
}

/* C 1263-1420.  C's two `#if 0` regions (C 1265-1268's declarations and C 1278-1417's
   "TODO : Mouse cursor customization") are dead in every configuration and are carried as C's own
   text below.  The live half is C 1269-1277: `f->output_data.w32->mouse_pixel` is the union
   member `frame.rs` 39 spells `*mut c_void` (P-72 rule 3), `mask_color` is an `int` compared with
   the `COLORREF` member, and C's two single-statement `if` bodies are the port's one-line blocks.
   The final `update_face_from_frame_parameter (f, Qmouse_color, arg)` is inside the dead region in
   C's text (C 1416 is past the second `#endif`? no: it stands at C 1416 inside `#if 0`).  */
/* Set the mouse color of frame F to ARG.  */

#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_mouse_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  /* C 1265-1268: (dead text, `#if 0`)
     Cursor cursor, nontext_cursor, mode_cursor, hand_cursor;
     int count;  */
  let mut mask_color: c_int;

  if ! NILP (arg)
    { (*(*f).output_data.w32 as *mut w32_output).mouse_pixel
	= w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as COLORREF; }
  mask_color = FRAME_BACKGROUND_PIXEL! (f) as c_int;

  /* Don't let pointers be invisible.  */
  if mask_color == (*(*f).output_data.w32 as *mut w32_output).mouse_pixel as c_int
	&& mask_color == FRAME_BACKGROUND_PIXEL! (f) as c_int
    { (*(*f).output_data.w32 as *mut w32_output).mouse_pixel
	= FRAME_FOREGROUND_PIXEL! (f) as COLORREF; }

/* C 1278-1417 is `#if 0 /* TODO : Mouse cursor customization.  */` and C's own preprocessor never
   compiles it; the port keeps C's text as a comment (the `w32term.rs` 33-34 reading).

#if 0 /* TODO : Mouse cursor customization.  */
  block_input ();

  /* It's not okay to crash if the user selects a screwy cursor.  */
  count = x_catch_errors (FRAME_W32_DISPLAY (f));

  if (!NILP (Vx_pointer_shape))
    {
      CHECK_FIXNUM (Vx_pointer_shape);
      cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XFIXNUM (Vx_pointer_shape));
    }
  else
    cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_xterm);
  x_check_errors (FRAME_W32_DISPLAY (f), "bad text pointer cursor: %s");

  if (!NILP (Vx_nontext_pointer_shape))
    {
      CHECK_FIXNUM (Vx_nontext_pointer_shape);
      nontext_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f),
					  XFIXNUM (Vx_nontext_pointer_shape));
    }
  else
    nontext_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_left_ptr);
  x_check_errors (FRAME_W32_DISPLAY (f), "bad nontext pointer cursor: %s");

  if (!NILP (Vx_hourglass_pointer_shape))
    {
      CHECK_FIXNUM (Vx_hourglass_pointer_shape);
      hourglass_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f),
					    XFIXNUM (Vx_hourglass_pointer_shape));
    }
  else
    hourglass_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_watch);
  x_check_errors (FRAME_W32_DISPLAY (f), "bad busy pointer cursor: %s");

  x_check_errors (FRAME_W32_DISPLAY (f), "bad nontext pointer cursor: %s");
  if (!NILP (Vx_mode_pointer_shape))
    {
      CHECK_FIXNUM (Vx_mode_pointer_shape);
      mode_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f),
				       XFIXNUM (Vx_mode_pointer_shape));
    }
  else
    mode_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_xterm);
  x_check_errors (FRAME_W32_DISPLAY (f), "bad modeline pointer cursor: %s");

  if (!NILP (Vx_sensitive_text_pointer_shape))
    {
      CHECK_FIXNUM (Vx_sensitive_text_pointer_shape);
      hand_cursor
	= XCreateFontCursor (FRAME_W32_DISPLAY (f),
			     XFIXNUM (Vx_sensitive_text_pointer_shape));
    }
  else
    hand_cursor = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_crosshair);

  if (!NILP (Vx_window_horizontal_drag_shape))
    {
      CHECK_FIXNUM (Vx_window_horizontal_drag_shape);
      horizontal_drag_cursor
	= XCreateFontCursor (FRAME_W32_DISPLAY (f),
			     XFIXNUM (Vx_window_horizontal_drag_shape));
    }
  else
    horizontal_drag_cursor
      = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_sb_h_double_arrow);

  if (!NILP (Vx_window_vertical_drag_shape))
    {
      CHECK_FIXNUM (Vx_window_vertical_drag_shape);
      vertical_drag_cursor
	= XCreateFontCursor (FRAME_W32_DISPLAY (f),
			     XFIXNUM (Vx_window_vertical_drag_shape));
    }
  else
    vertical_drag_cursor
      = XCreateFontCursor (FRAME_W32_DISPLAY (f), XC_sb_v_double_arrow);

  /* Check and report errors with the above calls.  */
  x_check_errors (FRAME_W32_DISPLAY (f), "can't set cursor shape: %s");
  x_uncatch_errors (FRAME_W32_DISPLAY (f), count);

  {
    XColor fore_color, back_color;

    fore_color.pixel = f->output_data.w32->mouse_pixel;
    back_color.pixel = mask_color;
    XQueryColor (FRAME_W32_DISPLAY (f),
		 DefaultColormap (FRAME_W32_DISPLAY (f),
				  DefaultScreen (FRAME_W32_DISPLAY (f))),
		 &fore_color);
    XQueryColor (FRAME_W32_DISPLAY (f),
		 DefaultColormap (FRAME_W32_DISPLAY (f),
				  DefaultScreen (FRAME_W32_DISPLAY (f))),
		 &back_color);
    XRecolorCursor (FRAME_W32_DISPLAY (f), cursor,
		    &fore_color, &back_color);
    XRecolorCursor (FRAME_W32_DISPLAY (f), nontext_cursor,
		    &fore_color, &back_color);
    XRecolorCursor (FRAME_W32_DISPLAY (f), mode_cursor,
		    &fore_color, &back_color);
    XRecolorCursor (FRAME_W32_DISPLAY (f), hand_cursor,
		    &fore_color, &back_color);
    XRecolorCursor (FRAME_W32_DISPLAY (f), hourglass_cursor,
		    &fore_color, &back_color);
  }

  if (FRAME_W32_WINDOW (f) != 0)
    XDefineCursor (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f), cursor);

  if (cursor != f->output_data.w32->text_cursor && f->output_data.w32->text_cursor != 0)
    XFreeCursor (FRAME_W32_DISPLAY (f), f->output_data.w32->text_cursor);
  f->output_data.w32->text_cursor = cursor;

  if (nontext_cursor != f->output_data.w32->nontext_cursor
      && f->output_data.w32->nontext_cursor != 0)
    XFreeCursor (FRAME_W32_DISPLAY (f), f->output_data.w32->nontext_cursor);
  f->output_data.w32->nontext_cursor = nontext_cursor;

  if (hourglass_cursor != f->output_data.w32->hourglass_cursor
      && f->output_data.w32->hourglass_cursor != 0)
    XFreeCursor (FRAME_W32_DISPLAY (f), f->output_data.w32->hourglass_cursor);
  f->output_data.w32->hourglass_cursor = hourglass_cursor;

  if (mode_cursor != f->output_data.w32->modeline_cursor
      && f->output_data.w32->modeline_cursor != 0)
    XFreeCursor (FRAME_W32_DISPLAY (f), f->output_data.w32->modeline_cursor);
  f->output_data.w32->modeline_cursor = mode_cursor;

  if (hand_cursor != f->output_data.w32->hand_cursor
      && f->output_data.w32->hand_cursor != 0)
    XFreeCursor (FRAME_W32_DISPLAY (f), f->output_data.w32->hand_cursor);
  f->output_data.w32->hand_cursor = hand_cursor;

  XFlush (FRAME_W32_DISPLAY (f));
  unblock_input ();

  update_face_from_frame_parameter (f, Qmouse_color, arg);
#endif /* TODO */
*/
}

/* C 1423-1463.  `Vx_cursor_fore_pixel` is `globals.rs`'s, the two `output_data.w32` members are
   `COLORREF` (the `cursor_foreground_pixel`/`cursor_pixel` pair of `include/w32term.rs`), and
   C's two `gui_update_cursor (f, 0)`/`(f, 1)` `int`-as-`bool` arguments are the port's
   `false`/`true` (a `macro-body-notes.tsv` row records the two integer tokens).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_cursor_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut fore_pixel: c_ulong;
  let mut pixel: c_ulong;

  if ! NILP (Vx_cursor_fore_pixel)
    { fore_pixel = w32_decode_color (f, Vx_cursor_fore_pixel,
				     WHITE_PIX_DEFAULT! (f)) as c_ulong; }
  else
    { fore_pixel = FRAME_BACKGROUND_PIXEL! (f); }

  pixel = w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as c_ulong;

  /* Make sure that the cursor color differs from the background color.  */
  if pixel == FRAME_BACKGROUND_PIXEL! (f)
    {
      pixel = (*(*f).output_data.w32 as *mut w32_output).mouse_pixel as c_ulong;
      if pixel == fore_pixel
	{ fore_pixel = FRAME_BACKGROUND_PIXEL! (f); }
    }

  (*(*f).output_data.w32 as *mut w32_output).cursor_foreground_pixel = fore_pixel as COLORREF;
  (*(*f).output_data.w32 as *mut w32_output).cursor_pixel = pixel as COLORREF;

  if FRAME_W32_WINDOW! (f) != 0
    {
      block_input ();
      /* Update frame's cursor_gc.  */
      (*(*(*f).output_data.w32 as *mut w32_output).cursor_gc).foreground = fore_pixel;
      (*(*(*f).output_data.w32 as *mut w32_output).cursor_gc).background = pixel;

      unblock_input ();

      if FRAME_VISIBLE_P (f)
	{
	  gui_update_cursor (f, false);
	  gui_update_cursor (f, true);
	}
    }

  update_face_from_frame_parameter (f, Qcursor_color (), arg);
}

/* Set the border-color of frame F to pixel value PIX.
   Note that this does not fully take effect if done before
   F has a window.  */

/* C 1470-1480.  `border_pixel` is `include/w32term.rs`'s member (`COLORREF`) and C's
   `int pix` narrows into it, which the port writes out.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_border_pixel (f: *mut frame, pix: c_int)
{

  (*(*f).output_data.w32 as *mut w32_output).border_pixel = pix as COLORREF;

  if FRAME_W32_WINDOW! (f) != 0 && (*f).border_width > 0
    {
      if FRAME_VISIBLE_P (f)
	{ redraw_frame (f); }
    }
}

/* Set the border-color of frame F to value described by ARG.
   ARG can be a string naming a color.
   The border-color is used for the border that is drawn by the server.
   Note that this does not fully take effect if done before
   F has a window; it must be redone when the window is created.  */

/* C 1489-1497.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_border_color (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut pix: c_int;

  CHECK_STRING (arg);
  pix = w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f));
  w32_set_border_pixel (f, pix);
  update_face_from_frame_parameter (f, Qborder_color (), arg);
}

/* C 1501-1504.  */

#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_cursor_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  set_frame_cursor_types (f, arg);
}

/* C 1507-1531.  C's `error ("No icon window available")` is the mirror's variadic `error` with a
   `c"…"` literal (PORTING.md 2.1), and the terminal hook is the `pgtkfns.rs` 1211 shape (the
   `Option<…>` is `unwrap`ped and the frame crosses as `*mut c_void`).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_icon_type (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut result: bool;

  if NILP (arg) && NILP (oldval)
    { return; }

  if STRINGP (arg) && STRINGP (oldval)
      && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
    { return; }

  if SYMBOLP (arg) && SYMBOLP (oldval) && EQ (arg, oldval)
    { return; }

  block_input ();

  result = (*FRAME_TERMINAL! (f)).set_bitmap_icon_hook.unwrap () (f as *mut c_void, arg);
  if result
    {
      unblock_input ();
      error (c"No icon window available".as_ptr ());
    }

  unblock_input ();
}

/* C 1534-1578.  C's second half is `#if 0` (C 1552-1583), kept as C's own text below;
   `fset_icon_name` is `include/frame.rs`'s.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_icon_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  if STRINGP (arg)
    {
      if STRINGP (oldval) && BASE_EQ (Fstring_equal (oldval, arg), Qt ())
	{ return; }
    }
  else if ! NILP (arg) || NILP (oldval)
    { return; }

  fset_icon_name (f, arg);

/* C 1552-1583 is `#if 0`: C's own preprocessor never compiles it, so the port keeps C's text as a
   comment (the `w32term.rs` 33-34 reading).

#if 0
  if (f->output_data.w32->icon_bitmap != 0)
    return;

  block_input ();

  result = x_text_icon (f,
			SSDATA ((!NILP (f->icon_name)
				 ? f->icon_name
				 : !NILP (f->title)
				 ? f->title
				 : f->name)));

  if (result)
    {
      unblock_input ();
      error ("No icon window available");
    }

  /* If the window was unmapped (and its icon was mapped),
     the new icon is not mapped, so map the window in its stead.  */
  if (FRAME_VISIBLE_P (f))
    {
#ifdef USE_X_TOOLKIT
      XtPopup (f->output_data.w32->widget, XtGrabNone);
#endif
      XMapWindow (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f));
    }

  XFlush (FRAME_W32_DISPLAY (f));
  unblock_input ();
#endif
*/
}

/// C 1610-1615/1757/1793/1819: the five display helpers `dispextern.h` 3901-3940 and `lisp.h` 5120
/// declare (their mirrors do not carry them) plus the menu-bar family `frame.h` 1881-1883 and
/// `lisp.h` 5117 declare; the unit declares them at the use site (P-88 ①), and `adjust_frame_size`
/// is `frame.h` 1511's.
/// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
/// C: dispextern.h 3906-3907: `void clear_glyph_matrix (struct glyph_matrix *); void
/// C: clear_current_matrices (struct frame *f);`
/// C: dispextern.h 3940: `extern void clear_frame (struct frame *);`
/// C: lisp.h 5120: `extern Lisp_Object get_frame_param (struct frame *, Lisp_Object);`
/// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
/// C: Lisp_Object);`
/// C: frame.h 1881/1883: `extern void set_frame_menubar (struct frame *f, bool deep_p); extern
/// C: void free_frame_menubar (struct frame *);`
/// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn adjust_frame_glyphs (f: *mut frame);
  pub fn clear_glyph_matrix (matrix: *mut glyph_matrix);
  pub fn clear_current_matrices (f: *mut frame);
  pub fn clear_frame (f: *mut frame);
  pub fn get_frame_param (f: *mut frame, prop: Lisp_Object) -> Lisp_Object;
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                            inhibit: c_int, inhibit_2: bool, parameter: Lisp_Object);
  pub fn set_frame_menubar (f: *mut frame, deep_p: bool);
  pub fn free_frame_menubar (f: *mut frame);
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
}

/// frame.h 1078-1083: the menu-bar flag's two arms, and the two plain members below it.  The
/// mirror has the readers as functions; this unit also writes them (C 1731-1733, 1802-1803), so
/// the write forms stand here at their use site (P-72 rule 3, the `dispnew.rs` 7367 and
/// `haikufns.rs` 390 shapes).  `lisp.h` 3933 defines `HAVE_EXT_MENU_BAR` as true for every
/// window-system build, which is the arm that writes the flag; **the flag's bit number for the
/// w32 arm is unmeasured** (`include/frame.rs` 405-418 numbers only arms B and C, and 329 is the
/// `compile_error!` that says so) - recorded in the ledger's owed list.
/// C: frame.h 1078-1083: `#ifdef HAVE_EXT_MENU_BAR` / `#define FRAME_EXTERNAL_MENU_BAR(f)
/// C: (f)->external_menu_bar` / `#else` / `#define FRAME_EXTERNAL_MENU_BAR(f) false` / `#endif`
/// C: frame.h 1086-1087: `#define FRAME_MENU_BAR_LINES(f) (f)->menu_bar_lines` /
/// C: `#define FRAME_MENU_BAR_HEIGHT(f) (f)->menu_bar_height`
/// C: frame.h 1093-1098: `#define FRAME_TAB_BAR_LINES(f) (f)->tab_bar_lines` /
/// C: `#define FRAME_TAB_BAR_HEIGHT(f) (f)->tab_bar_height`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_EXTERNAL_MENU_BAR {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT, FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT,
		    ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_MENU_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_lines = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_MENU_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).menu_bar_height = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TAB_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).tab_bar_lines = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).tab_bar_height = $v };
}

/* C 1587-1634.  C's `lookup_basic_face (NULL, f, …)` passes a null `struct window *`, so the
   port's first argument is the null pointer typed by the mirror's declaration; the ternary chain
   is C's, the `FACE_FROM_ID_OR_NULL` call is the mirror's, and `face->background` is
   `dispextern.rs`'s `unsigned long`.  `w32_fill_area`/`w32_clear_area` are `include/w32term.rs`
   879-925's statement macros (C's own `do { … } while (0)` blocks).  */
/**
 * w32_clear_under_internal_border:
 *
 * Clear area of frame F's internal border.  If the internal border face
 * of F has been specified (is not null), fill the area with that face.
 */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_clear_under_internal_border (f: *mut frame)
{
  let border: c_int = FRAME_INTERNAL_BORDER_WIDTH! (f);

  /* Clear border if it's larger than before.  */
  if border != 0
    {
      let width: c_int = FRAME_PIXEL_WIDTH! (f);
      let height: c_int = FRAME_PIXEL_HEIGHT! (f);
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT! (f);
      let face_id: c_int = (if ! FRAME_PARENT_FRAME (f).is_null ()
			    { if ! NILP (Vface_remapping_alist)
				{ lookup_basic_face (core::ptr::null_mut (), f,
						     CHILD_FRAME_BORDER_FACE_ID) }
			      else
				{ CHILD_FRAME_BORDER_FACE_ID } }
			    else
			    { if ! NILP (Vface_remapping_alist)
				{ lookup_basic_face (core::ptr::null_mut (), f,
						     INTERNAL_BORDER_FACE_ID) }
			      else
				{ INTERNAL_BORDER_FACE_ID } });
      let face: *mut face = FACE_FROM_ID_OR_NULL (f, face_id);

      block_input ();
      let hdc: HDC = get_frame_dc (f);
      if ! face.is_null ()
	{
	  /* Fill border with internal border face.  */
	  let color: c_ulong = (*face).background;

	  w32_fill_area! (f, hdc, color, 0, FRAME_TOP_MARGIN_HEIGHT! (f),
			  width, border);
	  w32_fill_area! (f, hdc, color, 0, 0, border, height);
	  w32_fill_area! (f, hdc, color, width - border, 0, border, height);
	  w32_fill_area! (f, hdc, color, 0, height - bottom_margin - border,
			  width, border);
	}
      else
	{
	  w32_clear_area! (f, hdc, 0, FRAME_TOP_MARGIN_HEIGHT! (f),
			   width, border);
	  w32_clear_area! (f, hdc, 0, 0, border, height);
	  w32_clear_area! (f, hdc, width - border, 0, border, height);
	  w32_clear_area! (f, hdc, 0, height - bottom_margin - border,
			   width, border);
	}
      release_frame_dc (f, hdc);
      unblock_input ();
    }
}

/**
 * w32_set_child_frame_border_width:
 *
 * Set width of child frame F's internal border to ARG pixels.
 * ARG < 0 is treated like ARG = 0.
 */
/* C 1643-1666.  C's `adjust_frame_size (f, -1, -1, 3, false, …)` is `frame.h` 1511's (declared
   above); `FRAME_NATIVE_WINDOW` is `include/w32term.rs`'s macro and C's `if (…) w32_clear_…`
   single statement is the port's one-line block.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_child_frame_border_width (f: *mut frame, arg: Lisp_Object,
					    oldval: Lisp_Object)
{
  let mut border: c_int;

  if NILP (arg)
    { border = -1; }
  else if RANGED_FIXNUMP (0, arg, INT_MAX)
    { border = XFIXNAT (arg) as c_int; }
  else
    { signal_error (c"Invalid child frame border width".as_ptr (), arg); }

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH! (f)
    {
      (*f).child_frame_border_width = border;

      if FRAME_NATIVE_WINDOW! (f) != 0
	{
	  adjust_frame_size (f, -1, -1, 3, false, Qchild_frame_border_width ());

	  if FRAME_VISIBLE_P (f)
	    { w32_clear_under_internal_border (f); }
	}
    }
}

/**
 * w32_set_internal_border_width:
 *
 * Set width of frame F's internal border to ARG pixels.  ARG < 0 is
 * treated like ARG = 0.
 */
/* C 1676-1693.  `check_integer_range`'s result is `intmax_t` (`lisp.h` 5143), which C narrows
   into its `int argval`, and C's `max (argval, 0)` is the mirror's `max` function over `isize`
   (the `abi_shim.rs` 2475 / `bidi.rs` 412 shape).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_internal_border_width (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let argval: c_int = check_integer_range (arg, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let border: c_int = max (argval as isize, 0) as c_int;

  if border != FRAME_INTERNAL_BORDER_WIDTH! (f)
    {
      (*f).internal_border_width = border;

      if FRAME_NATIVE_WINDOW! (f) != 0
	{
	  adjust_frame_size (f, -1, -1, 3, false, Qinternal_border_width ());

	  if FRAME_VISIBLE_P (f)
	    { w32_clear_under_internal_border (f); }
	}
    }
}

/**
 * w32_set_menu_bar_lines:
 *
 * Set number of lines of frame F's menu bar to VALUE.  An integer
 * greater zero specifies 1 line and turns the menu bar on if it was off
 * before.  Any other value specifies 0 lines and turns the menu bar off
 * if it was on before.
 */
/* C 1705-1750.  C's `boolean` is `lisp.h`'s `typedef bool boolean`, `FIXNUMP`'s result is the
   port's `bool` function, and C's two writes to `FRAME_EXTERNAL_MENU_BAR (f)`/`FRAME_MENU_BAR_LINES
   (f)`/`FRAME_MENU_BAR_HEIGHT (f)` are the macro pair declared above; `FRAME_W32_WINDOW (f)` is a
   pointer test, so C's bare `if (…)` is the `!= 0` comparison.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  /* Right now, menu bars don't work properly in minibuf-only frames;
     most of the commands try to apply themselves to the minibuffer
     frame itself, and get an error because you can't switch buffers in
     or split the minibuffer window.  Child frames don't like menu bars
     either.  */
  if ! FRAME_MINIBUF_ONLY_P (f) && FRAME_PARENT_FRAME (f).is_null ()
    {
      let old: bool = FRAME_EXTERNAL_MENU_BAR (f);
      let new: bool = if FIXNUMP (value) && XFIXNUM (value) > 0 { true } else { false };

      FRAME_MENU_BAR_LINES! (f, 0);
      FRAME_MENU_BAR_HEIGHT! (f, 0);

      if old != new
	{
	  FRAME_EXTERNAL_MENU_BAR! (f, new);

	  if ! old
	    /* Make menu bar when there was none.  Emacs 25 waited until
	       the next redisplay for this to take effect.  */
	    { set_frame_menubar (f, true); }
	  else
	    {
	      /* Remove menu bar.  */
	      free_frame_menubar (f);

	      /* Adjust the frame size so that the client (text) dimensions
		 remain the same.  Note that we resize twice: The first time
		 upon a request from the window manager who wants to keep
		 the height of the outer rectangle (including decorations)
		 unchanged, and a second time because we want to keep the
		 height of the inner rectangle (without the decorations
		 unchanged).  */
	      adjust_frame_size (f, -1, -1, 2, false, Qmenu_bar_lines ());
	    }

	  if FRAME_W32_WINDOW! (f) != 0
	    { w32_clear_under_internal_border (f); }

	  /* Don't store anything but 1 or 0 in the parameter.  */
	  store_frame_param (f, Qmenu_bar_lines (), make_fixnum (if new { 1 } else { 0 }));
	}
    }
}

/* Set the number of lines used for the tab bar of frame F to VALUE.
   VALUE not an integer, or < 0 means set the lines to zero.  OLDVAL
   is the old number of tab bar lines.  This function changes the
   height of all windows on frame F to match the new tab bar height.
   The frame's height doesn't change.  */

/* C 1760-1775.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let mut nlines: c_int;

  /* Treat tab bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an int >= 0.  */
  if RANGED_FIXNUMP (0, value, INT_MAX)
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  w32_change_tab_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

/* Set the pixel height of the tab bar of frame F to HEIGHT.  */
/* C 1780-1848.  `FRAME_TAB_BAR_HEIGHT (f)`/`FRAME_TAB_BAR_LINES (f)` are written at C 1802-1803,
   so the two write macros stand above (the mirror's readers are functions); C's
   `WINDOWP (f->tab_bar_window)` is the mirror's `bool` function and `XWINDOW (…)`'s result is
   `*mut window`; C's `SET_FRAME_GARBAGED (f)` is `include/frame.rs`'s macro and
   `adjust_frame_size`'s six arguments are `frame.h` 1511's.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_change_tab_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TAB_BAR_HEIGHT (f);

  /* This differs from the tool bar code in that the tab bar height is
     not rounded up.  Otherwise, if redisplay_tab_bar decides to grow
     the tab bar by even 1 pixel, FRAME_TAB_BAR_LINES will be changed,
     leading to the tab bar height being incorrectly set upon the next
     call to x_set_font.  (bug#59285) */
  let mut lines: c_int = height / unit;

  /* Even so, HEIGHT might be less than unit if the tab bar face is
     not so tall as the frame's font height; which if true lines will
     be set to 0 and the tab bar will thus vanish.  */

  if lines == 0 && height != 0
    { lines = 1; }

  /* Make sure we redisplay all windows in this frame.  */
  fset_redisplay (f);

  /* Recalculate tab bar and frame text sizes.  */
  FRAME_TAB_BAR_HEIGHT! (f, height);
  FRAME_TAB_BAR_LINES! (f, lines);
  /* Store the `tab-bar-lines' and `height' frame parameters.  */
  store_frame_param (f, Qtab_bar_lines (), make_fixnum (lines as EMACS_INT));
  store_frame_param (f, Qheight (), make_fixnum (FRAME_LINES (f) as EMACS_INT));

  /* We also have to make sure that the internal border at the top of
     the frame, below the menu bar or tab bar, is redrawn when the
     tab bar disappears.  This is so because the internal border is
     below the tab bar if one is displayed, but is below the menu bar
     if there isn't a tab bar.  The tab bar draws into the area
     below the menu bar.  */
  if FRAME_W32_WINDOW! (f) != 0 && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*(XWINDOW ((*f).tab_bar_window) as *mut window)).current_matrix); }

  if ! (*f).tab_bar_resized
    {
      let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

      /* As long as tab_bar_resized is false, effectively try to change
	 F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
 { adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
			   1, false, Qtab_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtab_bar_lines ()); }

      (*f).tab_bar_resized = (*f).tab_bar_redisplayed;
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtab_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
  if FRAME_W32_WINDOW! (f) != 0
    { w32_clear_under_internal_border (f); }
}

/// C 1999-2018: the UTF-16 conversion half of `w32_set_title_bar_text`.  `CP_UTF8` is
/// `<winnls.h>`'s code page, `WCHAR`/`LPWSTR` are `<winnt.h>`'s 16-bit wide character and its
/// pointer, `SetWindowTextW`/`SetWindowTextA` are `<winuser.h>`'s two entry points (C's text names
/// both explicitly, so both stand), and `pMultiByteToWideChar` is `src/w32.c` 537's file-scope
/// object the `include/w32.rs` 525 block declares (`src/w32menu.rs` 809 declares the same object
/// for its own unit - each unit is its own crate, P-90's seventh clause).
/// C: #define CP_UTF8 65001
/// C: typedef unsigned short WCHAR; typedef WCHAR *LPWSTR; typedef const WCHAR *LPCWSTR;
/// C: WINUSERAPI BOOL WINAPI SetWindowTextA (HWND hWnd, LPCSTR lpString);
/// C: WINUSERAPI BOOL WINAPI SetWindowTextW (HWND hWnd, LPCWSTR lpString);
/// C: int (WINAPI *pMultiByteToWideChar)(UINT,DWORD,LPCSTR,int,LPWSTR,int);
#[cfg (feature = "have_ntgui")]
pub const CP_UTF8: UINT = 65001;
#[cfg (feature = "have_ntgui")]
pub type WCHAR = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type LPWSTR = *mut WCHAR;
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub static mut pMultiByteToWideChar: MultiByteToWideChar_Proc;
}
/// `<windows.h>`'s `MultiByteToWideChar` spelling of the same signature (`include/w32.rs` 347's
/// `MultiByteToWideChar_Proc`); the Cygwin half of the function below is the one that names it
/// through `GUI_FN`.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn SetWindowTextA (hwnd: HWND, lpString: *const c_char) -> BOOL;
  pub fn SetWindowTextW (hwnd: HWND, lpString: *const c_ushort) -> BOOL;
}

/// frame.h 1107-1108's two more writers (`FRAME_TOOL_BAR_LINES`/`FRAME_TOOL_BAR_HEIGHT` are the
/// mirror's readers; C 1957-1958 writes them).
/// C: frame.h 1107-1108: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines` /
/// C: `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).tool_bar_lines = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).tool_bar_height = $v };
}

/* C 1859-1874.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_tool_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let mut nlines: c_int;

  /* Treat tool bars like menu bars.  */
  if FRAME_MINIBUF_ONLY_P (f)
    { return; }

  /* Use VALUE only if an integer >= 0.  */
  if FIXNUMP (value) && XFIXNUM (value) >= 0
    { nlines = XFIXNAT (value) as c_int; }
  else
    { nlines = 0; }

  w32_change_tool_bar_height (f, nlines * FRAME_LINE_HEIGHT (f));
}

/* C 1877-1908.  C's `error ("Tool bar position must be either `top' or `bottom'")` is the
   mirror's variadic `error` with a `c"…"` literal; `fset_tool_bar_position` is `include/frame.rs`
   1544's.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_tool_bar_position (f: *mut frame,
				     new_value: Lisp_Object,
				     old_value: Lisp_Object)
{
  if ! EQ (new_value, Qtop ()) && ! EQ (new_value, Qbottom ())
    { error (c"Tool bar position must be either `top' or `bottom'".as_ptr ()); }

  if EQ (new_value, old_value)
    { return; }

  /* Set the tool bar position.  */
  fset_tool_bar_position (f, new_value);

  /* Now reconfigure frame glyphs to place the tool bar at the
     bottom.  While the inner height has not changed, call
     `resize_frame_windows' to place each of the windows at its
     new position.  */

  adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_position ());
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);

  if FRAME_W32_WINDOW! (f) != 0
    { w32_clear_under_internal_border (f); }
}

/* Enable or disable double buffering on frame F.

   When double buffering is enabled, all drawing happens on a back
   buffer (a bitmap), which is then displayed as a single operation
   after redisplay is complete.  This avoids flicker caused by the
   results of an incomplete redisplay becoming visible.  */

/* C 1910-1930.  C's `FRAME_OUTPUT_DATA (f)->want_paint_buffer = 1;` writes one of the two packed
   members `include/w32term.rs` 552-603 names: the record is reached through `FRAME_OUTPUT_DATA`,
   and the bit is set by `w32_output_bits!` - the integrator's 3.575 spelling, used, not
   re-invented.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_inhibit_double_buffering (f: *mut frame,
					    new_value: Lisp_Object,
					    /* This parameter is unused.  */
					    old_value: Lisp_Object)
{
  block_input ();

  if NILP (new_value)
    { w32_output_bits! (want_paint_buffer, FRAME_OUTPUT_DATA! (f), 1); }
  else
    {
      w32_output_bits! (want_paint_buffer, FRAME_OUTPUT_DATA! (f), 0);
      w32_release_paint_buffer (f);

      SET_FRAME_GARBAGED! (f);
    }

  unblock_input ();
}

/* Set the pixel height of the tool bar of frame F to HEIGHT.  */
/* C 1932-1980.  C's `windows_or_buffers_changed = 23;` is `include/window.rs` 282's global;
   C's two writes to `FRAME_TOOL_BAR_HEIGHT (f)`/`FRAME_TOOL_BAR_LINES (f)` use the macro pair
   above, and `XWINDOW (f->tool_bar_window)->current_matrix` is the mirror's accessor pair.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_change_tool_bar_height (f: *mut frame, height: c_int)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);
  let old_height: c_int = FRAME_TOOL_BAR_HEIGHT (f);
  let lines: c_int = (height + unit - 1) / unit;
  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

  /* Make sure we redisplay all windows in this frame.  */
  windows_or_buffers_changed = 23;

  /* Recalculate tool bar and frame text sizes.  */
  FRAME_TOOL_BAR_HEIGHT! (f, height);
  FRAME_TOOL_BAR_LINES! (f, lines);
  /* Store `tool-bar-lines' and `height' frame parameters.  */
  store_frame_param (f, Qtool_bar_lines (), make_fixnum (lines as EMACS_INT));
  store_frame_param (f, Qheight (), make_fixnum (FRAME_LINES (f) as EMACS_INT));

  if FRAME_W32_WINDOW! (f) != 0 && FRAME_TOOL_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*(XWINDOW ((*f).tool_bar_window) as *mut window)).current_matrix); }

  if ! (*f).tool_bar_resized
    {
      /* As long as tool_bar_resized is false, effectively try to change
	 F's native height.  */
      if NILP (fullscreen) || EQ (fullscreen, Qfullwidth ())
	{ adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
			     1, false, Qtool_bar_lines ()); }
      else
	{ adjust_frame_size (f, -1, -1, 4, false, Qtool_bar_lines ()); }

      (*f).tool_bar_resized = (*f).tool_bar_redisplayed;
    }
  else
    /* Any other change may leave the native size of F alone.  */
    { adjust_frame_size (f, -1, -1, 3, false, Qtool_bar_lines ()); }

  /* adjust_frame_size might not have done anything, garbage frame
     here.  */
  adjust_frame_glyphs (f);
  SET_FRAME_GARBAGED! (f);
  if FRAME_W32_WINDOW! (f) != 0
    { w32_clear_under_internal_border (f); }
}

/* C 1983-2025.  C's `#ifdef __CYGWIN__` arm (C 1988-1990) and its `#else` arm (C 1991-2021) are
   both carried, gated `cygwin`; the Cygwin arm's `GUI_FN`/`GUI_SDATA`/`GUI_ENCODE_SYSTEM` are
   `include/w32term.rs` 1363-1375's macros (their own `windowsnt` gate is the mirror's reading of
   C's `#ifdef WINDOWSNT` - recorded in the ledger).  C's `alloca ((tlen + 1) * sizeof (wchar_t))`
   is this file's third bare `alloca` (the stack pool, PORTING.md P-05, a `macro-body-notes.tsv`
   row), `wchar_t` is the Windows 16-bit type (`include/w32.rs` 31's reading, i.e. `WCHAR`), and
   `L'\0'` is written as its code unit `0x0000` (a C wide literal has no Rust spelling - the
   `src/w32menu.rs` 820 note).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_title_bar_text (f: *mut frame, name: Lisp_Object)
{
  if FRAME_W32_WINDOW! (f) != 0
    {
      block_input ();
      /* C 1988-1990: `#ifdef __CYGWIN__` */
      #[cfg (feature = "cygwin")]
      { GUI_FN! (SetWindowText) (FRAME_W32_WINDOW! (f),
				 GUI_SDATA! (GUI_ENCODE_SYSTEM! () (name))); }

      /* C 1991-2021: `#else` */
      #[cfg (not (feature = "cygwin"))]
      {
      /* The frame's title many times shows the name of the file
	 visited in the selected window's buffer, so it makes sense to
	 support non-ASCII characters outside of the current system
	 codepage in the title.  */
      if w32_unicode_filenames
	{
	  let encoded_title: Lisp_Object = ENCODE_UTF_8 (name);
	  let mut title_w: *mut c_ushort;
	  let mut tlen: c_int = pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (encoded_title),
								-1, core::ptr::null_mut (), 0);

	  if tlen > 0
	    {
	      /* Windows truncates the title text beyond what fits on
		 a single line, so we can limit the length to some
		 reasonably large value, and use alloca.  */
	      if tlen > 10000
		{ tlen = 10000; }
	      title_w = sa_stack_alloc (((tlen + 1) as usize)
					* core::mem::size_of::<c_ushort> ()) as *mut c_ushort;
	      pMultiByteToWideChar.unwrap () (CP_UTF8, 0, SSDATA (encoded_title), -1,
					      title_w, tlen);
	      *title_w.add (tlen as usize) = 0x0000;
	      SetWindowTextW (FRAME_W32_WINDOW! (f), title_w);
	    }
	  else	/* Conversion to UTF-16 failed, so we punt.  */
	    { SetWindowTextA (FRAME_W32_WINDOW! (f),
			      SSDATA (ENCODE_SYSTEM (name))); }
	}
      else
	{ SetWindowTextA (FRAME_W32_WINDOW! (f), SSDATA (ENCODE_SYSTEM (name))); }
      }
      unblock_input ();
    }
}

/// C 2155-2175/2270-2290/2384-2390: `<winuser.h>`'s window-attribute and window-state entry
/// points with the five constants they are called with.  C's text names `GetWindowLong`/
/// `LoadImage`/`GetModuleHandle` (the SDK's `…A`/`…W` aliases; PORTING.md 0.1's rename
/// convention), and `HINSTANCE NULL` is the module handle of the running image.
/// C: #define GWL_STYLE (-16)
/// C: #define GWL_EXSTYLE (-20)
/// C: #define WS_THICKFRAME 0x00040000L / #define WS_CAPTION 0x00C00000L / #define WS_BORDER
/// C: 0x00800000L / #define WS_MAXIMIZEBOX 0x00010000L / #define WS_MINIMIZEBOX 0x00020000L /
/// C: #define WS_SYSMENU 0x00080000L
/// C: #define SW_HIDE 0 / #define SW_SHOWNOACTIVATE 4
/// C: #define HWND_TOP ((HWND)0) / #define HWND_NOTOPMOST ((HWND)-2) / #define HWND_TOPMOST
/// C: ((HWND)-1)
/// C: #define SWP_NOSIZE 0x0001 / #define SWP_NOMOVE 0x0002 / #define SWP_NOZORDER 0x0004 /
/// C: #define SWP_NOACTIVATE 0x0010 / #define SWP_FRAMECHANGED 0x0020 / #define
/// C: SWP_NOOWNERZORDER 0x0200
/// C: #define IMAGE_CURSOR 2 / #define LR_DEFAULTCOLOR 0x00000000 / #define LR_DEFAULTSIZE
/// C: 0x00000040 / #define LR_SHARED 0x00008000
/// C: LONG WINAPI GetWindowLongA (HWND hWnd, int nIndex);
/// C: BOOL WINAPI SetWindowPos (HWND hWnd, HWND hWndInsertAfter, int X, int Y, int cx, int cy,
/// C: UINT uFlags);
/// C: BOOL WINAPI ShowWindow (HWND hWnd, int nCmdShow);
/// C: HWND WINAPI SetParent (HWND hWndChild, HWND hWndNewParent);
/// C: HANDLE WINAPI LoadImageA (HINSTANCE hinst, LPCSTR lpszName, UINT uType, int cxDesired, int
/// C: cyDesired, UINT fuLoad);
#[cfg (feature = "have_ntgui")]
pub type LPCTSTR = *const c_char;
#[cfg (feature = "have_ntgui")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub const GWL_STYLE: c_int = -16;
#[cfg (feature = "have_ntgui")]
pub const GWL_EXSTYLE: c_int = -20;
#[cfg (feature = "have_ntgui")]
pub const WS_THICKFRAME: DWORD = 0x00040000;
#[cfg (feature = "have_ntgui")]
pub const WS_CAPTION: DWORD = 0x00C00000;
#[cfg (feature = "have_ntgui")]
pub const WS_BORDER: DWORD = 0x00800000;
#[cfg (feature = "have_ntgui")]
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
#[cfg (feature = "have_ntgui")]
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
#[cfg (feature = "have_ntgui")]
pub const WS_SYSMENU: DWORD = 0x00080000;
#[cfg (feature = "have_ntgui")]
pub const SW_HIDE: c_int = 0;
#[cfg (feature = "have_ntgui")]
pub const SW_SHOWNOACTIVATE: c_int = 4;
#[cfg (feature = "have_ntgui")]
pub const HWND_TOP: HWND = 0 as HWND;
#[cfg (feature = "have_ntgui")]
pub const HWND_NOTOPMOST: HWND = (-2) as isize as HWND;
#[cfg (feature = "have_ntgui")]
pub const HWND_TOPMOST: HWND = (-1) as isize as HWND;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOSIZE: UINT = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOMOVE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOZORDER: UINT = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOACTIVATE: UINT = 0x0010;
#[cfg (feature = "have_ntgui")]
pub const SWP_FRAMECHANGED: UINT = 0x0020;
#[cfg (feature = "have_ntgui")]
pub const SWP_NOOWNERZORDER: UINT = 0x0200;
#[cfg (feature = "have_ntgui")]
pub const IMAGE_CURSOR: UINT = 2;
#[cfg (feature = "have_ntgui")]
pub const LR_DEFAULTCOLOR: UINT = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const LR_DEFAULTSIZE: UINT = 0x00000040;
#[cfg (feature = "have_ntgui")]
pub const LR_SHARED: UINT = 0x00008000;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetWindowLong (hwnd: HWND, nIndex: c_int) -> LONG;
  pub fn SetWindowPos (hwnd: HWND, hwndInsertAfter: HWND, x: c_int, y: c_int, cx: c_int,
		       cy: c_int, uFlags: UINT) -> BOOL;
  pub fn ShowWindow (hwnd: HWND, nCmdShow: c_int) -> BOOL;
  pub fn SetParent (hwndChild: HWND, hwndNewParent: HWND) -> HWND;
  pub fn LoadImage (hinst: HINSTANCE, lpszName: LPCTSTR, uType: UINT, cxDesired: c_int,
		    cyDesired: c_int, fuLoad: UINT) -> HANDLE;
}

/// frame.h's packed flags this slice writes: the mirror has the readers (functions), and the
/// write forms stand here with the unit's own `frame_set_flag` (P-72 rule 3, the `haikufns.rs`
/// 408-448 shapes).  A `bool_bf` write goes through `frame_set_flag`; the two-bit `z_group` needs
/// the mask.
/// C: frame.h 417's `bool_bf explicit_name : 1`, C 428's `bool_bf undecorated : 1`, C 441's
/// C: `bool_bf skip_taskbar : 1`, C 446's `bool_bf no_focus_on_map : 1`, C 451's
/// C: `bool_bf no_accept_focus : 1`, C 482's `z_group : 2`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_EXPLICIT_NAME {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_UNDECORATED {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_SKIP_TASKBAR {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_SKIP_TASKBAR_UNIT, FRAME_SKIP_TASKBAR_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_NO_FOCUS_ON_MAP {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_NO_ACCEPT_FOCUS {
  ($f:expr, $v:expr) => {
    frame_set_flag ($f, FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT, ($v) as u32) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_Z_GROUP {
  ($f:expr, $v:expr) => {
    (*$f).bits2 = ((*$f).bits2 & ! (FRAME_Z_GROUP_MASK << FRAME_Z_GROUP_FLAG_SHIFT))
		  | (((($v) as u32) & FRAME_Z_GROUP_MASK) << FRAME_Z_GROUP_FLAG_SHIFT) };
}

/* C 2039-2080.  C's `f->explicit_name` is the `bool_bf` frame.h 417 packs, so the port reads it
   with its own `frame_flag` (`FRAME_EXPLICIT_NAME` above is the write form) and `! NILP (name)`
   is the bool C stores; `w32_id_name` is `w32_display_info`'s member and `strcmp` the mirror's.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_name (f: *mut frame, name: Lisp_Object, explicit: bool)
{
  /* Make sure that requests from lisp code override requests from
     Emacs redisplay code.  */
  if explicit
    {
      /* If we're switching from explicit to implicit, we had better
	 update the mode lines and thereby update the title.  */
      if frame_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT) && NILP (name)
	{ update_mode_lines = 25; }

      FRAME_EXPLICIT_NAME! (f, ! NILP (name));
    }
  else if frame_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT)
    { return; }

  /* If NAME is nil, set the name to the w32_id_name.  */
  if NILP (name)
    {
      /* Check for no change needed in this very common case
	 before we do any consing.  */
      if strcmp ((*FRAME_DISPLAY_INFO! (f)).w32_id_name,
		 SSDATA ((*f).name)) == 0
	{ return; }
      name = build_string ((*FRAME_DISPLAY_INFO! (f)).w32_id_name);
    }
  else
    { CHECK_STRING (name); }

  /* Don't change the name if it's already NAME.  */
  if ! NILP (Fstring_equal (name, (*f).name))
    { return; }

  fset_name (f, name);

  /* For setting the frame title, the title parameter should override
     the name parameter.  */
  if ! NILP ((*f).title)
    { name = (*f).title; }

  w32_set_title_bar_text (f, name);
}

/* This function should be called when the user's lisp code has
   specified a name for the frame; the name will override any set by the
   redisplay code.  */
/* C 2086-2089.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_explicitly_set_name (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  w32_set_name (f, arg, true);
}

/* This function should be called by Emacs redisplay code to set the
   name; names set this way will never override names set by the user's
   lisp code.  */
/* C 2095-2098.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_implicitly_set_name (f: *mut frame, arg: Lisp_Object,
						  oldval: Lisp_Object)
{
  w32_set_name (f, arg, false);
}

/* Change the title of frame F to NAME.
   If NAME is nil, use the frame name as the title.  */

/* C 2104-2118.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_title (f: *mut frame, name: Lisp_Object, old_name: Lisp_Object)
{
  /* Don't change the title if it's already NAME.  */
  if EQ (name, (*f).title)
    { return; }

  update_mode_lines = 26;

  fset_title (f, name);

  if NILP (name)
    { name = (*f).name; }

  w32_set_title_bar_text (f, name);
}

/* C 2121-2128.  `GetSystemMetrics` is `include/w32term.rs`'s declaration (C 473's sibling), and
   `FRAME_CONFIG_SCROLL_BAR_WIDTH`/`COLS` are written at C 2125-2127, so the write forms stand
   here (the mirror's readers are functions).  */
/// C: frame.h 1200-1203: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f)
/// C: (FRAME_PARAM_FACES (f) [SCROLL_BAR_FACE_ID].width)`-family - the port's readers are
/// C: functions, and C 2125/2127/2137/2139 write the two members.
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr, $v:expr) => { frame_set_config_scroll_bar_width ($f, $v) };
}
#[cfg (feature = "have_ntgui")]
unsafe fn frame_set_config_scroll_bar_width (f: *mut frame, v: c_int)
{
  (*f).config_scroll_bar_width = v;
}
/// C: frame.h 1204-1206: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) (f)->config_scroll_bar_cols`
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr, $v:expr) => { (*($f)).config_scroll_bar_cols = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr, $v:expr) => { (*($f)).config_scroll_bar_height = $v };
}
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr, $v:expr) => { (*($f)).config_scroll_bar_lines = $v };
}

#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_set_scroll_bar_default_width (f: *mut frame)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);

  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, GetSystemMetrics (SM_CXVSCROLL));
  FRAME_CONFIG_SCROLL_BAR_COLS! (f, (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + unit - 1) / unit);
}

/* C 2132-2139.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_set_scroll_bar_default_height (f: *mut frame)
{
  let unit: c_int = FRAME_LINE_HEIGHT (f);

  FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f, GetSystemMetrics (SM_CXHSCROLL));
  FRAME_CONFIG_SCROLL_BAR_LINES! (f, (FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) + unit - 1) / unit);
}

/**
 * w32_set_undecorated:
 *
 * Set frame F's `undecorated' parameter.  If non-nil, F's window-system
 * window is drawn without decorations, title, minimize/maximize boxes
 * and external borders.  This usually means that the window cannot be
 * dragged, resized, iconified, maximized or deleted with the mouse.  If
 * nil, draw the frame with all the elements listed above unless these
 * have been suspended via window manager settings.
 *
 * Some window managers may not honor this parameter.
 */
/* C 2154-2185.  C's `(… ? WS_BORDER : false)` is the port's `if`/`else` (Rust has no `?:`), and
   C's `false` there is the 0 the port writes (a `macro-body-notes.tsv` row records the word);
   `GetWindowLong`'s `LONG` narrows into C's `DWORD dwStyle`, which the port writes out, and
   `f->output_data.w32->dwStyle` is the `w32_output` member `include/w32term.rs` declares.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_undecorated (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  let hwnd: HWND = FRAME_W32_WINDOW! (f);
  let mut dwStyle: DWORD = GetWindowLong (hwnd, GWL_STYLE) as DWORD;
  let border_width: Lisp_Object = Fcdr (Fassq (Qborder_width (), (*f).param_alist));

  block_input ();
  if ! NILP (new_value) && frame_flag (f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT) == false
    {
      dwStyle = (dwStyle & ! WS_THICKFRAME & ! WS_CAPTION)
	| (if FIXNUMP (border_width) && XFIXNUM (border_width) > 0
	     { WS_BORDER } else { 0 });
      SetWindowLong (hwnd, GWL_STYLE, dwStyle as LONG);
      SetWindowPos (hwnd, HWND_TOP, 0, 0, 0, 0,
		    SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE
		    | SWP_FRAMECHANGED);
      FRAME_UNDECORATED! (f, true);
    }
  else if NILP (new_value) && frame_flag (f, FRAME_UNDECORATED_UNIT, FRAME_UNDECORATED_SHIFT)
    {
      SetWindowLong (hwnd, GWL_STYLE, (dwStyle | WS_THICKFRAME | WS_CAPTION
				       | WS_MAXIMIZEBOX | WS_MINIMIZEBOX | WS_SYSMENU) as LONG);
      SetWindowPos (hwnd, HWND_TOP, 0, 0, 0, 0,
		    SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE
		    | SWP_FRAMECHANGED);
      FRAME_UNDECORATED! (f, false);
    }

  (*(*f).output_data.w32 as *mut w32_output).dwStyle = GetWindowLong (hwnd, GWL_STYLE) as DWORD;

  unblock_input ();
}

/**
 * w32_set_parent_frame:
 *
 * Set frame F's `parent-frame' parameter.  If non-nil, make F a child
 * frame of the frame specified by that parameter.  Technically, this
 * makes F's window-system window a child window of the parent frame's
 * window-system window.  If nil, make F's window-system window a
 * top-level window--a child of its display's root window.
 *
 * A child frame is clipped at the native edges of its parent frame.
 * Its `left' and `top' parameters specify positions relative to the
 * top-left corner of its parent frame's native rectangle.  Usually,
 * moving a parent frame moves all its child frames too, keeping their
 * positions relative to the parent's native rectangle fixed.  The
 * parent frame may have decorations, while the child frame usually has
 * its `undecorated' parameter set.
 *
 * On Windows, if a child frame's redisplay is not finished,
 * `w32_set_parent_frame' might have to change the parent frame's
 * window accordingly.  This is the job of
 * `w32_set_parent_frame_by_w32_set_parent_frame' below.
 *
 * Some window managers may not honor this parameter.
 */
/* C 2216-2247.  `SetParent` is `<winuser.h>`'s (declared above); C's `p ? … : NULL` is the port's
   `if`/`else` (the `null_mut ()` spelling of C's `NULL`), `FRAMEP`/`FRAME_LIVE_P` are the
   mirror's `bool` functions, and `fset_parent_frame` is the mirror's frame.h 1697-family setter.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_parent_frame (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  let mut p: *mut frame = 0 as *mut frame;

  if ! NILP (new_value)
      && (! FRAMEP (new_value)
	  || { p = XFRAME (new_value); ! FRAME_LIVE_P (p) }
	  || ! FRAME_W32_P (p))
    {
      store_frame_param (f, Qparent_frame (), old_value);
      error (c"Invalid specification of `parent-frame'".as_ptr ());
    }

  if p != FRAME_PARENT_FRAME (f)
    {
      let hwnd: HWND = FRAME_W32_WINDOW! (f);
      let hwnd_parent: HWND = if ! p.is_null () { FRAME_W32_WINDOW! (p) } else { core::ptr::null_mut () };
      let mut hwnd_value: HWND;

      block_input ();
      hwnd_value = SetParent (hwnd, hwnd_parent);
      unblock_input ();

      if ! hwnd_value.is_null ()
	{ fset_parent_frame (f, new_value); }
      else
	{
	  store_frame_param (f, Qparent_frame (), old_value);
	  error (c"Reparenting frame failed".as_ptr ());
	}
    }
}

/**
 * w32_set_skip_taskbar:
 *
 * Set frame F's `skip-taskbar' parameter.  If non-nil, this should
 * remove F's icon from the taskbar associated with the display of F's
 * window-system window and inhibit switching to F's window via
 * <Alt>-<TAB>.  On Windows iconifying F will "roll in" its window at
 * the bottom of the desktop.  If nil, lift these restrictions.
 *
 * Some window managers may not honor this parameter.
 */
/* C 2261-2281.  C's `NULL` is the port's `null_mut ()` (the token check folds the two), and the
   `WS_EX_NOACTIVATE` constant is this file's own C 352 guard (`LONG`, so C's `exStyle |
   WS_EX_NOACTIVATE` widens it, which the port writes out).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_skip_taskbar (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if ! EQ (new_value, old_value)
    {
      let hwnd: HWND = FRAME_W32_WINDOW! (f);
      let exStyle: DWORD = GetWindowLong (hwnd, GWL_EXSTYLE) as DWORD;

      block_input ();
      /* Temporarily hide the window while changing its WS_EX_NOACTIVATE
	 setting.  */
      ShowWindow (hwnd, SW_HIDE);
      if ! NILP (new_value)
	{ SetWindowLong (hwnd, GWL_EXSTYLE, (exStyle | WS_EX_NOACTIVATE as DWORD) as LONG); }
      else
	{ SetWindowLong (hwnd, GWL_EXSTYLE, (exStyle & ! (WS_EX_NOACTIVATE as DWORD)) as LONG); }
      ShowWindow (hwnd, SW_SHOWNOACTIVATE);
      unblock_input ();

      FRAME_SKIP_TASKBAR! (f, ! NILP (new_value));
    }
}

/**
 * w32_set_no_focus_on_map:
 *
 * Set frame F's `no-focus-on-map' parameter which, if non-nil, means
 * that F's window-system window does not want to receive input focus
 * when it is mapped.  (A frame's window is mapped when the frame is
 * displayed for the first time and when the frame changes its state
 * from `iconified' or `invisible' to `visible'.)
 *
 * Some window managers may not honor this parameter.
 */
/* C 2295-2299.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_no_focus_on_map (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if ! EQ (new_value, old_value)
    { FRAME_NO_FOCUS_ON_MAP! (f, ! NILP (new_value)); }
}

/**
 * w32_set_no_accept_focus:
 *
 * Set frame F's `no-accept-focus' parameter which, if non-nil, hints
 * that F's window-system window does not want to receive input focus
 * via mouse clicks or by moving the mouse into it.
 *
 * If non-nil, this may have the unwanted side-effect that a user cannot
 * scroll a non-selected frame with the mouse.
 *
 * Some window managers may not honor this parameter.
 */
/* C 2314-2318.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_no_accept_focus (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  if ! EQ (new_value, old_value)
    { FRAME_NO_ACCEPT_FOCUS! (f, ! NILP (new_value)); }
}

/**
 * w32_set_z_group:
 *
 * Set frame F's `z-group' parameter.  If `above', F's window-system
 * window is displayed above all windows that do not have the `above'
 * property set.  If nil, F's window is shown below all windows that
 * have the `above' property set and above all windows that have the
 * `below' property set.  If `below', F's window is displayed below all
 * windows that do not have the `below' property set.
 *
 * Some window managers may not honor this parameter.  The value `below'
 * is not supported on Windows.
 *
 * Internally, this function also handles a value 'above-suspended'.
 * That value is used to temporarily remove F from the 'above' group
 * to make sure that it does not obscure the window of a dialog in
 * progress.
 */
/* C 2339-2374.  C's four `FRAME_Z_GROUP (f) = …` writes are the two-bit field's write macro
   above (`z_group_none`/`z_group_above`/`z_group_above_suspended` are `include/frame.rs`
   1589-1640's enumerators).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_set_z_group (f: *mut frame, new_value: Lisp_Object, old_value: Lisp_Object)
{
  let hwnd: HWND = FRAME_W32_WINDOW! (f);

  if NILP (new_value)
    {
      block_input ();
      SetWindowPos (hwnd, HWND_NOTOPMOST, 0, 0, 0, 0,
		    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
		    | SWP_NOOWNERZORDER);
      unblock_input ();
      FRAME_Z_GROUP! (f, z_group_none);
    }
  else if EQ (new_value, Qabove ())
    {
      block_input ();
      SetWindowPos (hwnd, HWND_TOPMOST, 0, 0, 0, 0,
		    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
		    | SWP_NOOWNERZORDER);
      unblock_input ();
      FRAME_Z_GROUP! (f, z_group_above);
    }
  else if EQ (new_value, Qabove_suspended ())
    {
      block_input ();
      SetWindowPos (hwnd, HWND_NOTOPMOST, 0, 0, 0, 0,
		    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE
		    | SWP_NOOWNERZORDER);
      unblock_input ();
      FRAME_Z_GROUP! (f, z_group_above_suspended);
    }
  else if EQ (new_value, Qbelow ())
    { error (c"Value `below' for z-group is not supported on Windows".as_ptr ()); }
  else
    { error (c"Invalid z-group specification".as_ptr ()); }
}

/* Subroutines for creating a frame.  */

/* C 2379-2379: C's `HCURSOR w32_load_cursor (LPCTSTR);` is a forward declaration of the
   definition below; it has no Rust counterpart (PORTING.md 0.1-12), and `w32_wnd_proc`'s
   (C 2396) belongs to the next slice.  */

/* C 2381-2394.  C's `GetModuleHandle (NULL)` passes the running image's handle (`include/w32.rs`
   410 declares it) and `HINSTANCE NULL` is the port's `null_mut ()`.  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_load_cursor (name: LPCTSTR) -> HCURSOR
{
  /* Try first to load cursor from application resource.  */
  let mut cursor: HCURSOR = LoadImage (GetModuleHandle (c"".as_ptr ()),
				       name, IMAGE_CURSOR, 0, 0,
				       LR_DEFAULTCOLOR | LR_DEFAULTSIZE | LR_SHARED) as HCURSOR;
  if cursor.is_null ()
    {
      /* Then try to load a shared predefined cursor.  */
      cursor = LoadImage (core::ptr::null_mut (), name, IMAGE_CURSOR, 0, 0,
			  LR_DEFAULTCOLOR | LR_DEFAULTSIZE | LR_SHARED) as HCURSOR;
    }
  return cursor;
}

/// C 2399-2406/2415-2422/2443-2450/2464-2471: `<winuser.h>`'s window-class record (both the
/// ANSI and the wide spelling, because C 2404/2418 declares both) and the six constants the
/// classes and the two scroll bars use.
/// C: typedef struct tagWNDCLASSA { UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int
/// C: cbWndExtra; HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground; LPCSTR
/// C: lpszMenuName; LPCSTR lpszClassName; } WNDCLASSA, *LPWNDCLASSA;
/// C: typedef struct tagWNDCLASSW { UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int
/// C: cbWndExtra; HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground;
/// C: LPCWSTR lpszMenuName; LPCWSTR lpszClassName; } WNDCLASSW, *LPWNDCLASSW;
/// C: typedef LRESULT (CALLBACK *WNDPROC) (HWND, UINT, WPARAM, LPARAM);
/// C: #define CS_VREDRAW 0x0001 / #define CS_HREDRAW 0x0002
/// C: #define SBS_VERT 0x0001 / #define SBS_HORZ 0x0000
/// C: #define WS_CHILD 0x40000000L / #define WS_VISIBLE 0x10000000L / #define WS_CLIPSIBLINGS
/// C: 0x04000000L
/// C: ATOM WINAPI RegisterClassA (const WNDCLASSA *lpWndClass);
/// C: ATOM WINAPI RegisterClassW (const WNDCLASSW *lpWndClass);
/// C: HICON WINAPI LoadIconA (HINSTANCE hInstance, LPCSTR lpIconName);
/// C: HWND WINAPI CreateWindowExA (DWORD dwExStyle, LPCSTR lpClassName, LPCSTR lpWindowName,
/// C: DWORD dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, HMENU hMenu,
/// C: HINSTANCE hInstance, LPVOID lpParam);
/// C: BOOL WINAPI PostThreadMessageA (DWORD idThread, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: #define IDC_ARROW MAKEINTRESOURCE(32512)
/// C: #define EMACS_CLASS "Emacs"
/// C: #define MAX_UTF8_PATH (MAXPATHLEN * 4)   [nt/inc/ms-w32.h 156]
/// C: typedef struct _DROPFILES { DWORD pFiles; POINT pt; BOOL fNC; BOOL fWide; } DROPFILES;
/// C: coding.h 737: `extern Lisp_Object from_unicode_buffer (const wchar_t *wstr);`
/// C: size_t wcslen (const wchar_t *s);
#[cfg (feature = "have_ntgui")]
pub type HICON = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HBRUSH = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type ATOM = WORD;
#[cfg (feature = "have_ntgui")]
pub type WNDPROC = Option<unsafe extern "system" fn (HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNDCLASS
{
  pub style: UINT,
  pub lpfnWndProc: WNDPROC,
  pub cbClsExtra: c_int,
  pub cbWndExtra: c_int,
  pub hInstance: HINSTANCE,
  pub hIcon: HICON,
  pub hCursor: HCURSOR,
  pub hbrBackground: HBRUSH,
  pub lpszMenuName: *const c_char,
  pub lpszClassName: *const c_char,
}
#[cfg (feature = "have_ntgui")]
pub type WNDCLASSW = WNDCLASS;
#[cfg (feature = "have_ntgui")]
pub const CS_VREDRAW: UINT = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const CS_HREDRAW: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const SBS_VERT: DWORD = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const SBS_HORZ: DWORD = 0x0000;
#[cfg (feature = "have_ntgui")]
pub const WS_CHILD: DWORD = 0x40000000;
#[cfg (feature = "have_ntgui")]
pub const WS_VISIBLE: DWORD = 0x10000000;
#[cfg (feature = "have_ntgui")]
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
#[cfg (feature = "have_ntgui")]
pub const EMACS_CLASS: &str = "Emacs";
#[cfg (feature = "have_ntgui")]
pub const IDC_ARROW: LPSTR = MAKEINTRESOURCE! (32512);
#[cfg (feature = "have_ntgui")]
pub const MAXPATHLEN: c_int = 1024;
#[cfg (feature = "have_ntgui")]
pub const MAX_UTF8_PATH: c_int = MAXPATHLEN * 4;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DROPFILES
{
  pub pFiles: DWORD,
  pub pt: POINT,
  pub fNC: BOOL,
  pub fWide: BOOL,
}
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn RegisterClassA (lpWndClass: *const WNDCLASS) -> ATOM;
  pub fn RegisterClassW (lpWndClass: *const WNDCLASS) -> ATOM;
  pub fn LoadIcon (hInstance: HINSTANCE, lpIconName: LPSTR) -> HICON;
  pub fn CreateWindow (dwExStyle: DWORD, lpClassName: *const c_char, lpWindowName: *const c_char,
		       dwStyle: DWORD, x: c_int, y: c_int, nWidth: c_int, nHeight: c_int,
		       hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE,
		       lpParam: LPVOID) -> HWND;
  pub fn PostThreadMessage (idThread: DWORD, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn from_unicode_buffer (wstr: *const c_ushort) -> Lisp_Object;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn wcslen (s: *const c_ushort) -> usize;
}

/// C 2398-2407's `INIT_WINDOW_CLASS(WC)`: C's nine-assignment `do`-less macro, written as the
/// port's block expression (PORTING.md 2.1).  `w32_wnd_proc` is *this file's* definition at
/// C 4528 (the next slice's, per P-91's "same file, C line known, not ported" class - the
/// probe's allowed finding), and `hinst` is C 336's own object (the prefix's).
/// C: #define INIT_WINDOW_CLASS(WC) (WC).style = CS_HREDRAW | CS_VREDRAW; \
/// C: (WC).lpfnWndProc = w32_wnd_proc; (WC).cbClsExtra = 0; (WC).cbWndExtra = WND_EXTRA_BYTES; \
/// C: (WC).hInstance = hinst; (WC).hIcon = LoadIcon (hinst, EMACS_CLASS); (WC).hCursor =
/// C: w32_load_cursor (IDC_ARROW); (WC).hbrBackground = NULL; (WC).lpszMenuName = NULL;
#[cfg (feature = "have_ntgui")]
macro_rules! INIT_WINDOW_CLASS {
  ($wc:expr) => { {
    ($wc).style = CS_HREDRAW | CS_VREDRAW;
    ($wc).lpfnWndProc = Some (w32_wnd_proc);
    ($wc).cbClsExtra = 0;
    ($wc).cbWndExtra = WND_EXTRA_BYTES;
    ($wc).hInstance = hinst;
    ($wc).hIcon = LoadIcon (hinst, EMACS_CLASS.as_ptr () as LPSTR);
    ($wc).hCursor = w32_load_cursor (IDC_ARROW);
    ($wc).hbrBackground = core::ptr::null_mut ();
    ($wc).lpszMenuName = core::ptr::null ();
  } };
}

/* C 2409-2428.  C's `if (w32_unicode_gui)` arm calls `RegisterClassW` with `L"Emacs"`; a C wide
   literal has no Rust spelling (the `src/w32menu.rs` 820 note), so the port writes the code units
   and takes their address - `EMACS_WIDE_CLASS` below.  */
/// C: wc.lpszClassName = L"Emacs";
#[cfg (feature = "have_ntgui")]
static EMACS_WIDE_CLASS: [WCHAR; 6] = [0x0045, 0x006d, 0x0061, 0x0063, 0x0073, 0x0000];

#[cfg (feature = "have_ntgui")]
unsafe fn w32_init_class (hinst: HINSTANCE) -> BOOL
{
  if w32_unicode_gui != 0
    {
      let mut uwc: WNDCLASSW = core::mem::zeroed ();
      INIT_WINDOW_CLASS! (uwc);
      uwc.lpszClassName = EMACS_WIDE_CLASS.as_ptr ();

      return RegisterClassW (&raw const uwc) as BOOL;
    }
  else
    {
      let mut wc: WNDCLASS = core::mem::zeroed ();
      INIT_WINDOW_CLASS! (wc);
      wc.lpszClassName = EMACS_CLASS.as_ptr () as *const c_char;

      return RegisterClassA (&raw const wc) as BOOL;
    }
}

/* Applies the Windows system theme (light or dark) to the window
   handle HWND.  */
/* C 2433-2456.  `SetWindowTheme_fn`/`DwmSetWindowAttribute_fn` are the prefix's `static mut
   Option<…>` objects (C 249-272's block) and `w32_follow_system_dark_mode` is `globals.rs`'s
   `bool`; C's `w32_darkmode ? 1 : 0` is the port's `if`/`else`.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_applytheme (hwnd: HWND)
{
  if w32_darkmode != 0 && w32_follow_system_dark_mode
    {
      /* Set window theme to that of a built-in Windows app (Explorer),
	 because it has dark scroll bars and other UI elements.  */
      if SetWindowTheme_fn.is_some ()
	{ SetWindowTheme_fn.unwrap () (hwnd, DARK_MODE_APP_NAME.as_ptr (), core::ptr::null ()); }

      /* Set the titlebar to system dark mode.  */
      if DwmSetWindowAttribute_fn.is_some ()
	{
	  /* Windows 10 version 2004 and up, Windows 11.  */
	  let mut attr: DWORD = DWMWA_USE_IMMERSIVE_DARK_MODE;
	  /* Windows 10 older than 2004.  */
	  if w32_build_number < 19041
	    { attr = DWMWA_USE_IMMERSIVE_DARK_MODE_OLD; }
	  DwmSetWindowAttribute_fn.unwrap () (hwnd, attr,
					      (&raw const w32_darkmode) as LPCVOID,
					      core::mem::size_of_val (&w32_darkmode) as DWORD);
	}
    }
  let dark_mode_p: WPARAM = if w32_darkmode != 0 { 1 } else { 0 };
  PostThreadMessage (dwMainThreadId, WM_EMACS_SET_TOOLKIT_THEME, dark_mode_p, 0);
}

/* C 2459-2473.  `bar`'s four members are `include/w32term.rs`'s `scroll_bar` (C 483-530), and
   `CreateWindow`'s class name and title are C's two `""` literals (`c""`).  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_createvscrollbar (f: *mut frame, bar: *mut scroll_bar) -> HWND
{
  let hwnd: HWND = CreateWindow (c"SCROLLBAR".as_ptr (), c"".as_ptr (),
				 /* Clip siblings so we don't draw over child
				    frames.  Apparently this is not always
				    sufficient so we also try to make bar windows
				    bottommost.  */
				 SBS_VERT | WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS,
				 /* Position and size of scroll bar.  */
				 (*bar).left, (*bar).top, (*bar).width, (*bar).height,
				 FRAME_W32_WINDOW! (f), core::ptr::null_mut (), hinst,
				 core::ptr::null_mut ());
  if ! hwnd.is_null ()
    { w32_applytheme (hwnd); }
  return hwnd;
}

/* C 2476-2490.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_createhscrollbar (f: *mut frame, bar: *mut scroll_bar) -> HWND
{
  let hwnd: HWND = CreateWindow (c"SCROLLBAR".as_ptr (), c"".as_ptr (),
				 /* Clip siblings so we don't draw over child
				    frames.  Apparently this is not always
				    sufficient so we also try to make bar windows
				    bottommost.  */
				 SBS_HORZ | WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS,
				 /* Position and size of scroll bar.  */
				 (*bar).left, (*bar).top, (*bar).width, (*bar).height,
				 FRAME_W32_WINDOW! (f), core::ptr::null_mut (), hinst,
				 core::ptr::null_mut ());
  if ! hwnd.is_null ()
    { w32_applytheme (hwnd); }
  return hwnd;
}

/* From the DROPFILES struct, extract the filenames and return as a list
   of strings.  */
/* C 2495-2536.  C's `#ifdef NTGUI_UNICODE` arm (C 2503-2510) and its `#else` arm (C 2512-2531)
   are both carried, the first under the `ntgui_unicode` feature (`src/w32menu.rs`'s reading of
   that macro) with C's inner `#ifdef CYGWIN` arm under `cygwin`; C's
   `(char *) files + files->pFiles` is typed pointer arithmetic, `wcslen` is `<wchar.h>`'s, and
   `filename_from_utf16`/`filename_from_ansi` are `include/w32.rs`'s declarations (C 537-558).  */
#[cfg (feature = "have_ntgui")]
unsafe fn process_dropfiles (files: *mut DROPFILES) -> Lisp_Object
{
  let start_of_files: *mut c_char = files as *mut c_char;
  let start_of_files: *mut c_char = start_of_files.add ((*files).pFiles as usize);
  /* C 2497-2499: `#ifndef NTGUI_UNICODE` */
  #[cfg (not (feature = "ntgui_unicode"))]
  let mut filename: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];
  let mut lisp_files: Lisp_Object = Qnil ();

  /* C 2503-2510: `#ifdef NTGUI_UNICODE` */
  #[cfg (feature = "ntgui_unicode")]
  {
  let mut p: *mut c_ushort = start_of_files as *mut c_ushort;
  while *p != 0
    {
      let mut fn_: Lisp_Object = from_unicode_buffer (p);
      /* C 2507-2509: `#ifdef CYGWIN` */
      #[cfg (feature = "cygwin")]
      { fn_ = Fcygwin_convert_file_name_to_windows (fn_, Qt ()); }
      lisp_files = Fcons (fn_, lisp_files);
      p = p.add (wcslen (p) + 1);
    }
  }
  /* C 2512-2531: `#else` */
  #[cfg (not (feature = "ntgui_unicode"))]
  {
  if (*files).fWide != 0
    {
      let mut p: *mut c_ushort = start_of_files as *mut c_ushort;
      while *p != 0
	{
	  filename_from_utf16 (p, filename.as_mut_ptr ());
	  lisp_files = Fcons (DECODE_FILE (build_unibyte_string (filename.as_ptr ())),
			      lisp_files);
	  p = p.add (wcslen (p) + 1);
	}
    }
  else
    {
      let mut p: *mut c_char = start_of_files;
      while *p != 0
	{
	  filename_from_ansi (p, filename.as_mut_ptr ());
	  lisp_files = Fcons (DECODE_FILE (build_unibyte_string (filename.as_ptr ())),
			      lisp_files);
	  p = p.add (strlen (p) + 1);
	}
    }
  }
  return lisp_files;
}
