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
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn w32_console_toggle_lock_key (arg0: c_int, arg1: Lisp_Object) -> c_int;
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
