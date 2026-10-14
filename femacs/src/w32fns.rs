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
#[cfg (feature = "have_ntgui")]
include!("include/conf_post.rs");               // config.h's AH_BOTTOM `#include <conf_post.h>` (`DebPrint`, C 3819)

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

/* C 48-59: the C headers, in C's order (PORTING.md 0.1-14: `lisp.rs` first, because the other
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
/// C: #define SM_CXVSCROLL 2 / #define SM_CXHSCROLL 4 - the two metrics C 2125/2127 and
/// C 6362-6363 read.  They were used from slice 5 on (C 2125/2127's
/// `w32_set_scroll_bar_default_width`/`_height`) without a definition anywhere in the tree,
/// which is what slice 8's name sweep found: the `<windows.h>` section spells them here in
/// `<winuser.h>`'s order, next to C 283-290's four neighbours.
#[cfg (feature = "have_ntgui")]
pub const SM_CXVSCROLL: c_int = 2;
#[cfg (feature = "have_ntgui")]
pub const SM_CXHSCROLL: c_int = 4;
/// C: #define HORZSIZE 4 / #define VERTSIZE 6 / #define HORZRES 8 / #define VERTRES 10 - the
/// four `<wingdi.h>` metrics C 6858-6859, C 6874-6875, C 6989-6990 and C 7056-7060 read.
#[cfg (feature = "have_ntgui")]
pub const HORZSIZE: c_int = 4;
#[cfg (feature = "have_ntgui")]
pub const VERTSIZE: c_int = 6;
#[cfg (feature = "have_ntgui")]
pub const HORZRES: c_int = 8;
#[cfg (feature = "have_ntgui")]
pub const VERTRES: c_int = 10;
/// C: #define SPI_GETWORKAREA 0x0030 - the action C 7071 passes to `SystemParametersInfo`; the
/// function itself is the declaration `src/w32term.rs` 326 carries for C 384 (this unit's
/// `<windows.h>` block has no copy, so one is added here with C's `<winuser.h>` line).
#[cfg (feature = "have_ntgui")]
pub const SPI_GETWORKAREA: UINT = 0x0030;

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
/* C 315-320: the two `<sysinfoapi.h>`/`<winver.h>` caches this file *defines*.  The pair used to
   be recorded here as deferred; `include/w32common.rs` 118-146 now carries both records (spelled
   from `<windows.h>` and marked UNVERIFIED there, since no `<windows.h>` exists on this machine)
   and declares both objects inside its `w32common_decls` module, so the two definitions land at
   C's own position here - the `syspage_mask` shape below (the mirror declares, the owning unit
   defines).  C's own text has no initialiser, i.e. a tentative definition (zero).  */
/// C: SYSTEM_INFO sysinfo_cache;
/// C: OSVERSIONINFO osinfo_cache;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub static mut sysinfo_cache: SYSTEM_INFO
  = SYSTEM_INFO {
      dwOemId: 0,
      dwPageSize: 0,
      lpMinimumApplicationAddress: core::ptr::null_mut (),
      lpMaximumApplicationAddress: core::ptr::null_mut (),
      dwActiveProcessorMask: 0,
      dwNumberOfProcessors: 0,
      dwProcessorType: 0,
      dwAllocationGranularity: 0,
      wProcessorLevel: 0,
      wProcessorRevision: 0,
    };
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub static mut osinfo_cache: OSVERSIONINFO
  = OSVERSIONINFO {
      dwOSVersionInfoSize: 0,
      dwMajorVersion: 0,
      dwMinorVersion: 0,
      dwBuildNumber: 0,
      dwPlatformId: 0,
      szCSDVersion: [0; 128],
    };

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
/// C: #define MB_OK 0x00000000 / #define MB_ICONHAND 0x00000010 / #define MB_ICONQUESTION
/// C: 0x00000020 / #define MB_ICONEXCLAMATION 0x00000030 / #define MB_ICONASTERISK
/// C: 0x00000040 - the five `<winuser.h>` ids C 7121-7131 gives `set-message-beep`, spelled
/// from the header (C 339's `MB_EMACS_SILENT` above is the file's own extra).
#[cfg (feature = "have_ntgui")]
pub const MB_OK: DWORD = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONHAND: DWORD = 0x00000010;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONQUESTION: DWORD = 0x00000020;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONEXCLAMATION: DWORD = 0x00000030;
#[cfg (feature = "have_ntgui")]
pub const MB_ICONASTERISK: DWORD = 0x00000040;

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
/// C: #define HKEY_CLASSES_ROOT ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000000))
/// C: #define HKEY_CURRENT_USER ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000001))
/// C: #define HKEY_LOCAL_MACHINE ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000002))
/// C: #define HKEY_USERS ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000003))
/// C: #define HKEY_CURRENT_CONFIG ((HKEY) (ULONG_PTR) ((INT_PTR) 0x80000005))
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
pub const HKEY_CLASSES_ROOT: HKEY = (0x80000000 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const HKEY_CURRENT_USER: HKEY = (0x80000001 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const HKEY_LOCAL_MACHINE: HKEY = (0x80000002 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const HKEY_USERS: HKEY = (0x80000003 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const HKEY_CURRENT_CONFIG: HKEY = (0x80000005 as ULONG_PTR) as HKEY;
#[cfg (feature = "have_ntgui")]
pub const KEY_READ: REGSAM = 0x20019;
#[cfg (feature = "have_ntgui")]
pub const ERROR_SUCCESS: LONG = 0;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn RegOpenKeyEx (hKey: HKEY, lpSubKey: *const c_char, ulOptions: DWORD,
                       samDesired: REGSAM, phkResult: PHKEY) -> LONG;
  /// C 890-932's block keeps C's name for the SDK's alias (`RegOpenKeyEx` above is the same case):
  /// C 10925/10942 write `RegQueryValueEx`, whose ANSI form is the one C's `char` buffers want.
  /// C: WINADVAPI LONG WINAPI RegQueryValueExA (HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved,
  /// C: LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
  pub fn RegQueryValueEx (hKey: HKEY, lpValueName: *const c_char, lpReserved: *mut DWORD,
			  lpType: *mut DWORD, lpData: LPBYTE, lpcbData: *mut DWORD) -> LONG;
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

      /* C's `colorname[len - 1]` is a pointer index, which Rust writes as `offset` (E0608).  */
      if c_isdigit (*colorname.offset ((len - 1) as isize) as c_int)
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

  fg = w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as COLORREF;
  /* `PIX_TYPE` is `COLORREF` (`DWORD`) while the `frame` member C reads and writes with
     `FRAME_FOREGROUND_PIXEL` is C's `unsigned long`, so C narrows on the read and widens on the
     write; the port writes both conversions out (the `w32_set_background_color` reading).  */
  old_fg = FRAME_FOREGROUND_PIXEL! (f) as COLORREF;
  FRAME_FOREGROUND_PIXEL! (f) = fg as c_ulong;

  if FRAME_W32_WINDOW! (f) != 0 as HWND
    {
      if (*x).cursor_pixel == old_fg
	{
	  (*x).cursor_pixel = fg;
	  (*(*x).cursor_gc).background = fg as c_ulong;
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

  if FRAME_W32_WINDOW! (f) != 0 as HWND
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
    { (*((*f).output_data.w32 as *mut w32_output)).mouse_pixel
	= w32_decode_color (f, arg, BLACK_PIX_DEFAULT! (f)) as COLORREF; }
  mask_color = FRAME_BACKGROUND_PIXEL! (f) as c_int;

  /* Don't let pointers be invisible.  */
  if mask_color == (*((*f).output_data.w32 as *mut w32_output)).mouse_pixel as c_int
	&& mask_color == FRAME_BACKGROUND_PIXEL! (f) as c_int
    { (*((*f).output_data.w32 as *mut w32_output)).mouse_pixel
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
      pixel = (*((*f).output_data.w32 as *mut w32_output)).mouse_pixel as c_ulong;
      if pixel == fore_pixel
	{ fore_pixel = FRAME_BACKGROUND_PIXEL! (f); }
    }

  (*((*f).output_data.w32 as *mut w32_output)).cursor_foreground_pixel = fore_pixel as COLORREF;
  (*((*f).output_data.w32 as *mut w32_output)).cursor_pixel = pixel as COLORREF;

  if FRAME_W32_WINDOW! (f) != 0 as HWND
    {
      block_input ();
      /* Update frame's cursor_gc.  */
      (*(*((*f).output_data.w32 as *mut w32_output)).cursor_gc).foreground = fore_pixel;
      (*(*((*f).output_data.w32 as *mut w32_output)).cursor_gc).background = pixel;

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

  (*((*f).output_data.w32 as *mut w32_output)).border_pixel = pix as COLORREF;

  if FRAME_W32_WINDOW! (f) != 0 as HWND && (*f).border_width > 0
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
  let border: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);

  /* Clear border if it's larger than before.  */
  if border != 0
    {
      let width: c_int = FRAME_PIXEL_WIDTH (f);
      let height: c_int = FRAME_PIXEL_HEIGHT (f);
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT (f);
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

	  w32_fill_area! (f, hdc, color, 0, FRAME_TOP_MARGIN_HEIGHT (f),
			  width, border);
	  w32_fill_area! (f, hdc, color, 0, 0, border, height);
	  w32_fill_area! (f, hdc, color, width - border, 0, border, height);
	  w32_fill_area! (f, hdc, color, 0, height - bottom_margin - border,
			  width, border);
	}
      else
	{
	  w32_clear_area! (f, hdc, 0, FRAME_TOP_MARGIN_HEIGHT (f),
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
  else if RANGED_FIXNUMP (0, arg, INT_MAX as isize)
    { border = XFIXNAT (arg) as c_int; }
  else
    { signal_error (c"Invalid child frame border width".as_ptr (), arg); }

  if border != FRAME_CHILD_FRAME_BORDER_WIDTH (f)
    {
      (*f).child_frame_border_width = border;

      if FRAME_NATIVE_WINDOW! (f) != 0 as HWND
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

  if border != FRAME_INTERNAL_BORDER_WIDTH (f)
    {
      (*f).internal_border_width = border;

      if FRAME_NATIVE_WINDOW! (f) != 0 as HWND
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

	  if !FRAME_W32_WINDOW! (f).is_null ()
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
  if RANGED_FIXNUMP (0, value, INT_MAX as isize)
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
  fset_redisplay (f as *mut c_void);

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
  if !FRAME_W32_WINDOW! (f).is_null () && FRAME_TAB_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tab_bar_window)
    { clear_glyph_matrix ((*(XWINDOW ((*f).tab_bar_window)
			     as *mut window)).current_matrix as *mut glyph_matrix); }

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
  if !FRAME_W32_WINDOW! (f).is_null ()
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

  if !FRAME_W32_WINDOW! (f).is_null ()
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

  if !FRAME_W32_WINDOW! (f).is_null () && FRAME_TOOL_BAR_HEIGHT (f) == 0
    {
      clear_frame (f);
      clear_current_matrices (f);
    }

  if height < old_height && WINDOWP ((*f).tool_bar_window)
    { clear_glyph_matrix ((*(XWINDOW ((*f).tool_bar_window)
			     as *mut window)).current_matrix as *mut glyph_matrix); }

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
  if !FRAME_W32_WINDOW! (f).is_null ()
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
  if !FRAME_W32_WINDOW! (f).is_null ()
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
unsafe fn w32_set_name (f: *mut frame, mut name: Lisp_Object, explicit: bool)
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
unsafe fn w32_set_title (f: *mut frame, mut name: Lisp_Object, old_name: Lisp_Object)
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

  (*((*f).output_data.w32 as *mut w32_output)).dwStyle = GetWindowLong (hwnd, GWL_STYLE) as DWORD;

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
/// C: #define CreateWindowA(lpClassName, lpWindowName, dwStyle, X, Y, nWidth, nHeight,
/// C: hWndParent, hMenu, hInstance, lpParam) CreateWindowExA (0L, lpClassName, lpWindowName,
/// C: dwStyle, X, Y, nWidth, nHeight, hWndParent, hMenu, hInstance, lpParam);
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
/// C: #define IDC_IBEAM MAKEINTRESOURCE(32513) / #define IDC_WAIT MAKEINTRESOURCE(32514) /
/// C: #define IDC_SIZENWSE MAKEINTRESOURCE(32642) / #define IDC_SIZENESW MAKEINTRESOURCE(32643) /
/// C: #define IDC_SIZEWE MAKEINTRESOURCE(32644) / #define IDC_SIZENS MAKEINTRESOURCE(32645)
/// - the six cursor ids C 6586-6600 hands to `w32_load_cursor` beyond `IDC_ARROW` (C 3689's
/// block) and `IDC_HAND` (C 465's `#ifndef` guard), spelled here from `<winuser.h>`, in the
/// header's order.  Slice 7's text used all six; slice 8's name sweep is what found them
/// undefined (see the lane ledger's Evidence).
#[cfg (feature = "have_ntgui")]
pub const IDC_IBEAM: LPSTR = MAKEINTRESOURCE! (32513);
#[cfg (feature = "have_ntgui")]
pub const IDC_WAIT: LPSTR = MAKEINTRESOURCE! (32514);
#[cfg (feature = "have_ntgui")]
pub const IDC_SIZENWSE: LPSTR = MAKEINTRESOURCE! (32642);
#[cfg (feature = "have_ntgui")]
pub const IDC_SIZENESW: LPSTR = MAKEINTRESOURCE! (32643);
#[cfg (feature = "have_ntgui")]
pub const IDC_SIZEWE: LPSTR = MAKEINTRESOURCE! (32644);
#[cfg (feature = "have_ntgui")]
pub const IDC_SIZENS: LPSTR = MAKEINTRESOURCE! (32645);
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
  pub fn CreateWindow (lpClassName: *const c_char, lpWindowName: *const c_char,
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

/* C 2410's parameter name `hinst` shadows the file-scope `hinst` (C 918), which Rust forbids
   (E0530: a parameter cannot shadow a static - the `post_character_message` `modifiers_`
   reading), so the port spells the parameter `hinst_` and the token check folds the underscore.
   `INIT_WINDOW_CLASS!`'s body writes `hinst` as C's `#define` does, and at this unit's only call
   site (C 2750) C passes the file-scope `hinst` - the same object - so both readings agree.  */
#[cfg (feature = "have_ntgui")]
unsafe fn w32_init_class (hinst_: HINSTANCE) -> BOOL
{
  if w32_unicode_gui != 0
    {
      let mut uwc: WNDCLASSW = core::mem::zeroed ();
      INIT_WINDOW_CLASS! (uwc);
      /* C 2416's `L"Emacs"` is the code-unit array above; the port's one `WNDCLASS` stands for
	 C's `WNDCLASS` *and* `WNDCLASSW`, whose `lpszClassName` members are `LPCSTR` and `LPCWSTR`,
	 so the wide pointer is cast to the field's type here (the `w32menu.rs` W/A reading).  */
      uwc.lpszClassName = EMACS_WIDE_CLASS.as_ptr () as *const c_char;

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
  PostThreadMessage (dwMainThreadId, WM_EMACS_SET_TOOLKIT_THEME as UINT, dark_mode_p, 0);
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


/* ==============================================================================================
   Slice 3: C 2538-4041 (`w32_process_dnd_data` through `post_character_message`).

   It continues where slice 2 stopped (C 2536, the `}` of `process_dropfiles`) and stops at a
   whole-item boundary: C 4041 is the `}` of `post_character_message`, and the next item's
   return type is C 4043.  The slice carries 1,504 C lines: the COM drop-target half (C
   2538-2700), `w32_createwindow` (C 2701), `my_post_msg` (C 2860), the keyboard-hook family
   (C 2871-3238), the modifier family and `map_keypad_keys` (C 3252-3575), the hot-key macros and
   `register_hot_keys`/`unregister_hot_keys` (C 3577-3615), `w32_name_of_message` (C 3617-3673,
   `EMACSDEBUG`), `w32_msg_pump` (C 3706-3838), the deferred-message family (C 3840-3960),
   `signal_user_input` (C 3962) and `post_character_message` (C 3976-4041).

   The COM surface (C 2538-2700) is the item slice 2 stopped in front of: its ledger entry asked
   for P-90 ①'s dedicated SDK mirror because this tree has a second consumer of `IDataObject`
   (`w32select.c`).  This lane cannot add a file under `femacs/src/include/` (its writable set),
   so the surface stands **in the unit** at C's own `#include` lines - P-88 ①'s rule, the same
   shape as this file's `<windows.h>`/`<winreg.h>`/`<wingdi.h>` sections - and
   `docs/ports/w32fns.md`'s owed list carries the move-back into `include/ole.rs` for the mirror
   round.  Every record there is spelled from the SDK's published member lists: no Windows SDK
   exists on this machine (`x86_64-w64-mingw32-gcc` is absent), so the layouts are `UNVERIFIED`.

   The file gate (`have_ntgui`) is on every item; the sub-gates this slice's C text names are
   `windowsnt` (C 2871-3154, C 3196-3239, C 3415-3418), `emacsdebug` (C 3617) and `msg_debug`
   (C 3816), the last two named after C's own macros (P-86 ①) and registered by the integrator.
   C 2834-2836's `#ifndef GC_ALLGESTURES` is an SDK guard, so the constant is carried with C's
   value and no `cfg` (the first slice's eleven-guard reading).
   ============================================================================================== */

/* C 41-46's `<oleidl.h>`/`<objidl.h>`/`<unknwn.h>`/`<ole2.h>` section, continued: the interfaces,
   the COM records and the result codes the drop-target half names.  The section's first half is
   C 41-46 (the include list itself); no file in the reference tree carries these names (P-88 ①)
   and P-90 ①'s dedicated SDK mirror is a mirror-round item - see the ledger.  */
/// C: typedef struct IUnknown { IUnknownVtbl *lpVtbl; } IUnknown;
/// C: HRESULT STDMETHODCALLTYPE QueryInterface (IUnknown *This, REFIID riid, void **ppvObject);
/// C: ULONG STDMETHODCALLTYPE AddRef (IUnknown *This);
/// C: ULONG STDMETHODCALLTYPE Release (IUnknown *This);
/// C: typedef /* [unique] */ IUnknown *LPUNKNOWN;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct IUnknown
{
  pub lpVtbl: *mut IUnknownVtbl,
}
/// C: typedef struct IUnknownVtbl {
/// C:   BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IUnknown *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IUnknown *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IUnknown *This);
/// C:   END_INTERFACE
/// C: } IUnknownVtbl;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IUnknownVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IUnknown, riid: REFIID,
                                                         ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IUnknown) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IUnknown) -> ULONG>,
}
/// C: typedef GUID IID; typedef const IID *REFIID;
/// C: typedef struct _GUID { unsigned long Data1; unsigned short Data2; unsigned short Data3;
/// C: unsigned char Data4[8]; } GUID;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID
{
  pub Data1: c_ulong,
  pub Data2: c_ushort,
  pub Data3: c_ushort,
  pub Data4: [c_uchar; 8],
}
#[cfg (feature = "have_ntgui")]
pub type IID = GUID;
#[cfg (feature = "have_ntgui")]
pub type REFIID = *const IID;
/// C: typedef struct tagPOINTL { LONG x; LONG y; } POINTL;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POINTL
{
  pub x: LONG,
  pub y: LONG,
}
/// C: typedef struct tagDVTARGETDEVICE { /* … */ } DVTARGETDEVICE;   (only ever a pointer here)
#[cfg (feature = "have_ntgui")]
pub type DVTARGETDEVICE = c_void;
/// C: typedef WORD CLIPFORMAT;
#[cfg (feature = "have_ntgui")]
pub type CLIPFORMAT = WORD;
/// C: typedef struct tagFORMATETC { CLIPFORMAT cfFormat; DVTARGETDEVICE *ptd; DWORD dwAspect;
/// C: LONG lindex; DWORD tymed; } FORMATETC;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FORMATETC
{
  pub cfFormat: CLIPFORMAT,
  pub ptd: *mut DVTARGETDEVICE,
  pub dwAspect: DWORD,
  pub lindex: LONG,
  pub tymed: DWORD,
}
/// C: typedef struct tagSTGMEDIUM { DWORD tymed; union { HBITMAP hBitmap; HMETAFILEPICT
/// C: hMetaFilePict; HENHMETAFILE hEnhMetaFile; HGLOBAL hGlobal; LPWSTR lpszFileName; IStream
/// C: *pstm; IStorage *pstg; } u; IUnknown *pUnkForRelease; } STGMEDIUM;
/// The union keeps C's name `u`; the union itself is named `<struct>_<member>` (the `glyph_slice`
/// convention `include/dispextern.rs`/`src/w32inevt.rs` use).  `IStream`/`IStorage` appear only as
/// pointers here, so they are the port's `*mut c_void` (0.1-2), with C's member text kept in the
/// `/// C:` lines above.
#[cfg (feature = "have_ntgui")]
pub type HBITMAP = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMETAFILEPICT = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HENHMETAFILE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGLOBAL = HANDLE;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union STGMEDIUM_u
{
  pub hBitmap: HBITMAP,
  pub hMetaFilePict: HMETAFILEPICT,
  pub hEnhMetaFile: HENHMETAFILE,
  pub hGlobal: HGLOBAL,
  pub lpszFileName: LPWSTR,
  pub pstm: *mut c_void,   /* C: IStream *pstm; */
  pub pstg: *mut c_void,   /* C: IStorage *pstg; */
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STGMEDIUM
{
  pub tymed: DWORD,
  pub u: STGMEDIUM_u,
  pub pUnkForRelease: *mut IUnknown,
}
/// C: typedef struct IDropTarget { IDropTargetVtbl *lpVtbl; } IDropTarget;
/// C: typedef struct IDataObject { IDataObjectVtbl *lpVtbl; } IDataObject;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct IDropTarget
{
  pub lpVtbl: *mut IDropTargetVtbl,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct IDataObject
{
  pub lpVtbl: *mut IDataObjectVtbl,
}
/// C: typedef struct IDropTargetVtbl {
/// C:   BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDropTarget *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDropTarget *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDropTarget *This);
/// C:   HRESULT (STDMETHODCALLTYPE *DragEnter) (IDropTarget *This, IDataObject *pDataObj,
/// C:   DWORD grfKeyState, POINTL pt, DWORD *pdwEffect);
/// C:   HRESULT (STDMETHODCALLTYPE *DragOver) (IDropTarget *This, DWORD grfKeyState, POINTL pt,
/// C:   DWORD *pdwEffect);
/// C:   HRESULT (STDMETHODCALLTYPE *DragLeave) (IDropTarget *This);
/// C:   HRESULT (STDMETHODCALLTYPE *Drop) (IDropTarget *This, IDataObject *pDataObj, DWORD
/// C:   grfKeyState, POINTL pt, DWORD *pdwEffect);
/// C:   END_INTERFACE
/// C: } IDropTargetVtbl;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDropTargetVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDropTarget, riid: REFIID,
                                                         ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDropTarget) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDropTarget) -> ULONG>,
  pub DragEnter: Option<unsafe extern "system" fn (This: *mut IDropTarget,
                                                   pDataObj: *mut IDataObject,
                                                   grfKeyState: DWORD, pt: POINTL,
                                                   pdwEffect: *mut DWORD) -> HRESULT>,
  pub DragOver: Option<unsafe extern "system" fn (This: *mut IDropTarget, grfKeyState: DWORD,
                                                  pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT>,
  pub DragLeave: Option<unsafe extern "system" fn (This: *mut IDropTarget) -> HRESULT>,
  pub Drop: Option<unsafe extern "system" fn (This: *mut IDropTarget,
                                              pDataObj: *mut IDataObject, grfKeyState: DWORD,
                                              pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT>,
}
/// C: typedef struct IDataObjectVtbl {
/// C:   BEGIN_INTERFACE
/// C:   HRESULT (STDMETHODCALLTYPE *QueryInterface) (IDataObject *This, REFIID riid, void **ppvObject);
/// C:   ULONG (STDMETHODCALLTYPE *AddRef) (IDataObject *This);
/// C:   ULONG (STDMETHODCALLTYPE *Release) (IDataObject *This);
/// C:   HRESULT (STDMETHODCALLTYPE *GetData) (IDataObject *This, FORMATETC *pformatetcIn,
/// C:   STGMEDIUM *pmedium);
/// C:   HRESULT (STDMETHODCALLTYPE *GetDataHere) (IDataObject *This, FORMATETC *pformatetc,
/// C:   STGMEDIUM *pmedium);
/// C:   HRESULT (STDMETHODCALLTYPE *QueryGetData) (IDataObject *This, FORMATETC *pformatetc);
/// C:   HRESULT (STDMETHODCALLTYPE *GetCanonicalFormatEtc) (IDataObject *This, FORMATETC
/// C:   *pformatectIn, FORMATETC *pformatetcOut);
/// C:   HRESULT (STDMETHODCALLTYPE *SetData) (IDataObject *This, FORMATETC *pformatetc,
/// C:   STGMEDIUM *pmedium, BOOL fRelease);
/// C:   HRESULT (STDMETHODCALLTYPE *EnumFormatEtc) (IDataObject *This, DWORD dwDirection,
/// C:   IEnumFORMATETC **ppenumFormatEtc);
/// C:   HRESULT (STDMETHODCALLTYPE *DAdvise) (IDataObject *This, FORMATETC *pformatetc, DWORD
/// C:   advf, IAdviseSink *pAdvSink, DWORD *pdwConnection);
/// C:   HRESULT (STDMETHODCALLTYPE *DUnadvise) (IDataObject *This, DWORD dwConnection);
/// C:   HRESULT (STDMETHODCALLTYPE *EnumDAdvise) (IDataObject *This, IEnumSTATDATA **ppenumAdvise);
/// C:   END_INTERFACE
/// C: } IDataObjectVtbl;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDataObjectVtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut IDataObject, riid: REFIID,
                                                         ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut IDataObject) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut IDataObject) -> ULONG>,
  pub GetData: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                 pformatetcIn: *mut FORMATETC,
                                                 pmedium: *mut STGMEDIUM) -> HRESULT>,
  pub GetDataHere: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                     pformatetc: *mut FORMATETC,
                                                     pmedium: *mut STGMEDIUM) -> HRESULT>,
  pub QueryGetData: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                      pformatetc: *mut FORMATETC) -> HRESULT>,
  pub GetCanonicalFormatEtc: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                               pformatectIn: *mut FORMATETC,
                                                               pformatetcOut: *mut FORMATETC)
                                                              -> HRESULT>,
  pub SetData: Option<unsafe extern "system" fn (This: *mut IDataObject, pformatetc: *mut FORMATETC,
                                                 pmedium: *mut STGMEDIUM,
                                                 fRelease: BOOL) -> HRESULT>,
  pub EnumFormatEtc: Option<unsafe extern "system" fn (This: *mut IDataObject, dwDirection: DWORD,
                                                      ppenumFormatEtc: *mut *mut c_void) -> HRESULT>,
  pub DAdvise: Option<unsafe extern "system" fn (This: *mut IDataObject, pformatetc: *mut FORMATETC,
                                                 advf: DWORD, pAdvSink: *mut c_void,
                                                 pdwConnection: *mut DWORD) -> HRESULT>,
  pub DUnadvise: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                   dwConnection: DWORD) -> HRESULT>,
  pub EnumDAdvise: Option<unsafe extern "system" fn (This: *mut IDataObject,
                                                     ppenumAdvise: *mut *mut c_void) -> HRESULT>,
}
/// C: #define S_OK ((HRESULT)0L)
/// C: #define E_NOINTERFACE ((HRESULT)0x80004002L)
/// C: #define DROPEFFECT_NONE 0 / #define DROPEFFECT_COPY 1 / #define DROPEFFECT_MOVE 2
/// C: #define DROPEFFECT_LINK 4
/// C: #define DVASPECT_CONTENT 1
/// C: #define TYMED_HGLOBAL 1
/// C: #define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
/// C: #define CF_TEXT 1 / #define CF_UNICODETEXT 13 / #define CF_HDROP 15
#[cfg (feature = "have_ntgui")]
pub const S_OK: HRESULT = 0;
#[cfg (feature = "have_ntgui")]
pub const E_NOINTERFACE: HRESULT = 0x80004002_u32 as HRESULT;
#[cfg (feature = "have_ntgui")]
pub const DROPEFFECT_NONE: DWORD = 0;
#[cfg (feature = "have_ntgui")]
pub const DROPEFFECT_COPY: DWORD = 1;
#[cfg (feature = "have_ntgui")]
pub const DROPEFFECT_MOVE: DWORD = 2;
#[cfg (feature = "have_ntgui")]
pub const DROPEFFECT_LINK: DWORD = 4;
#[cfg (feature = "have_ntgui")]
pub const DVASPECT_CONTENT: DWORD = 1;
#[cfg (feature = "have_ntgui")]
pub const TYMED_HGLOBAL: DWORD = 1;
#[cfg (feature = "have_ntgui")]
pub const CF_TEXT: c_int = 1;
#[cfg (feature = "have_ntgui")]
pub const CF_UNICODETEXT: c_int = 13;
#[cfg (feature = "have_ntgui")]
pub const CF_HDROP: c_int = 15;
/// C 2600: `SUCCEEDED (r)` - the SDK's macro over an `HRESULT`; its argument is evaluated once on
/// both sides (PORTING.md 2.1, the `eassert` reading) and its token survives the port.
/// C: #define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#[cfg (feature = "have_ntgui")]
macro_rules! SUCCEEDED {
  ($hr:expr) => { ($hr) >= 0 };
}
/// C 2607: `IDataObject_GetData (pDataObj, &formatetc, &stgmedium)` - `COBJMACROS`' C spelling of
/// the vtable call (C 37's `#define COBJMACROS`); the port spells the body and keeps the name.
/// C: #define IDataObject_GetData(This, pformatetcIn, pmedium) \
/// C: ((This)->lpVtbl->GetData (This, pformatetcIn, pmedium))
#[cfg (feature = "have_ntgui")]
macro_rules! IDataObject_GetData {
  ($obj:expr, $fmt:expr, $med:expr) => {
    (((*(*($obj)).lpVtbl).GetData).unwrap ()) (($obj), ($fmt), ($med)) };
}
/// C 2570: `HGLOBAL GlobalLock (HGLOBAL hMem); LPVOID GlobalLock (HGLOBAL hMem);` - `<winbase.h>`.
/// C: #define GlobalLock(hMem) GlobalLock (hMem)
/// C: LPVOID GlobalLock (HGLOBAL hMem);
/// C: BOOL GlobalUnlock (HGLOBAL hMem);
/// C: HGLOBAL GlobalFree (HGLOBAL hMem);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GlobalLock (hMem: HGLOBAL) -> LPVOID;
  pub fn GlobalUnlock (hMem: HGLOBAL) -> BOOL;
  pub fn GlobalFree (hMem: HGLOBAL) -> HGLOBAL;
}
/// C 2599/2604: `free (target->i_drop_target.lpVtbl); free (target);` and C 2736's
/// `malloc (sizeof (struct w32_drop_target))` - `<stdlib.h>`'s pair.  `include/lisp.rs` carries
/// `xmalloc`/`xfree` but not the bare pair, and `lib/include/stdlib.rs` is not in this unit's
/// include graph (the ledger records the declaration site).
/// C: void *malloc (size_t size);
/// C: void free (void *ptr);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn malloc (size: usize) -> *mut c_void;
  pub fn free (ptr: *mut c_void);
}
/// C 2570/2602: `HGLOBAL hg = hGlobal;` and C 2596's `WCHAR *text` - the wide character of this
/// unit's own `WCHAR` (C 1117's `<winnt.h>` reading, spelled in the second slice).

/* This function can be called ONLY between calls to
   block_input/unblock_input.  It is used in w32_read_socket.  */
/// C 2541-2575: `Lisp_Object w32_process_dnd_data (int format, void *hGlobal)` - a non-`static`
/// definition, so it carries `#[unsafe(no_mangle)]` (PORTING.md 0.2).  C's `switch (format)` has
/// no `default:`; the port writes the empty arm PORTING.md 2.2's table records for that shape
/// (`set_internal`'s precedent), and the `break` of each arm has no Rust spelling.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_process_dnd_data (format: c_int, hGlobal: *mut c_void) -> Lisp_Object
{
  let mut result: Lisp_Object = Qnil ();
  let hg: HGLOBAL = hGlobal as HGLOBAL;

  match format
    {
    CF_HDROP =>
      {
	let files: *mut DROPFILES = GlobalLock (hg) as *mut DROPFILES;
	if ! files.is_null ()
	  { result = process_dropfiles (files); }
	GlobalUnlock (hg);
      }
    CF_UNICODETEXT =>
      {
	let text: *mut WCHAR = GlobalLock (hg) as *mut WCHAR;
	result = from_unicode_buffer (text);
	GlobalUnlock (hg);
      }
    CF_TEXT =>
      {
	let text: *mut c_char = GlobalLock (hg) as *mut c_char;
	result = DECODE_SYSTEM (build_unibyte_string (text));
	GlobalUnlock (hg);
      }
    _ => {}
    }

  GlobalFree (hg);

  return result;
}

/* C 2577-2700.  The COM drop-target half: the record C 2538's own text names (`i_drop_target`
   must be the first member), the seven vtable entries C writes, and the two helpers.  */
/// C: struct w32_drop_target { IDropTarget i_drop_target; HWND hwnd; int ref_count; };
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct w32_drop_target
{
  pub i_drop_target: IDropTarget,   /* i_drop_target must be the first member.  */
  pub hwnd: HWND,
  pub ref_count: c_int,
}

/// C 2584-2589: `static HRESULT STDMETHODCALLTYPE w32_drop_target_QueryInterface (IDropTarget
/// *t, REFIID ri, void **r)`.  `STDMETHODCALLTYPE` is `__stdcall`, so the definition is
/// `unsafe extern "system"` (the `WNDPROC` reading above), and C's unused `t`/`ri`/`r` keep
/// their names.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_QueryInterface (t: *mut IDropTarget, ri: REFIID,
                                                          r: *mut *mut c_void) -> HRESULT
{
  return E_NOINTERFACE;
}

/// C 2590-2596: `static ULONG STDMETHODCALLTYPE w32_drop_target_AddRef (IDropTarget *This)` -
/// C's `return ++target->ref_count;` is an increment then a return of the new value.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_AddRef (This: *mut IDropTarget) -> ULONG
{
  let target: *mut w32_drop_target = This as *mut w32_drop_target;
  (*target).ref_count += 1;
  return (*target).ref_count as ULONG;
}

/// C 2597-2605: `static ULONG STDMETHODCALLTYPE w32_drop_target_Release (IDropTarget *This)` -
/// C's `if (--target->ref_count > 0)` is a decrement inside the test; the two `free`s keep C's
/// arguments and order.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_Release (This: *mut IDropTarget) -> ULONG
{
  let target: *mut w32_drop_target = This as *mut w32_drop_target;
  (*target).ref_count -= 1;
  if (*target).ref_count > 0
    { return (*target).ref_count as ULONG; }
  free ((*target).i_drop_target.lpVtbl as *mut c_void);
  free (target as *mut c_void);
  return 0;
}

/// C 2608-2620.
#[cfg (feature = "have_ntgui")]
unsafe fn w32_handle_drag_movement (This: *mut IDropTarget, pt: POINTL)
{
  let target: *mut w32_drop_target = This as *mut w32_drop_target;

  let mut msg: W32Msg = core::mem::zeroed ();   /* C: W32Msg msg = {0}; */
  msg.dwModifiers = w32_get_modifiers ();
  msg.msg.time = GetMessageTime () as DWORD;
  msg.msg.pt.x = pt.x;
  msg.msg.pt.y = pt.y;
  my_post_msg (&raw mut msg, (*target).hwnd, WM_EMACS_DRAGOVER as UINT, 0, 0);
}

/// C 2621-2633.  C's comment is kept; the four parameters of C's `DragEnter` keep their names.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_DragEnter (This: *mut IDropTarget,
                                                     pDataObj: *mut IDataObject,
                                                     grfKeyState: DWORD, pt: POINTL,
                                                     pdwEffect: *mut DWORD) -> HRESULT
{
  /* Possible 'effect' values are COPY, MOVE, LINK or NONE.  This choice
     changes the mouse pointer shape to inform the user of what will
     happen on drop.  We send COPY because our use cases don't modify
     or link to the original data.  */
  *pdwEffect = DROPEFFECT_COPY;
  w32_handle_drag_movement (This, pt);
  return S_OK;
}

/// C 2634-2643.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_DragOver (This: *mut IDropTarget, grfKeyState: DWORD,
                                                    pt: POINTL,
                                                    pdwEffect: *mut DWORD) -> HRESULT
{
  /* See comment in w32_drop_target_DragEnter.  */
  *pdwEffect = DROPEFFECT_COPY;
  w32_handle_drag_movement (This, pt);
  return S_OK;
}

/// C 2644-2649.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_DragLeave (This: *mut IDropTarget) -> HRESULT
{
  return S_OK;
}

/// C 2650-2664.  C's `STGMEDIUM stgmedium;` is an uninitialised local the callee fills
/// (PORTING.md 1.5: the port zero-initialises it and keeps C's statements), and C's `FORMATETC
/// formatetc = { format, NULL, DVASPECT_CONTENT, -1, TYMED_HGLOBAL }` keeps its member order.
#[cfg (feature = "have_ntgui")]
unsafe fn w32_try_get_data (pDataObj: *mut IDataObject, format: c_int) -> HGLOBAL
{
  let mut formatetc: FORMATETC = FORMATETC { cfFormat: format as CLIPFORMAT,
					     ptd: core::ptr::null_mut (),
					     dwAspect: DVASPECT_CONTENT,
					     lindex: -1,
					     tymed: TYMED_HGLOBAL };
  let mut stgmedium: STGMEDIUM = core::mem::zeroed ();
  let r: HRESULT = IDataObject_GetData! (pDataObj, &raw mut formatetc, &raw mut stgmedium);
  if SUCCEEDED! (r)
    {
      if stgmedium.tymed == TYMED_HGLOBAL
	{ return stgmedium.u.hGlobal; }
      ReleaseStgMedium (&raw mut stgmedium);
    }
  return core::ptr::null_mut ();
}

/// C 2665-2699.  C's `W32Msg msg = {0}` is the port's zeroed initialiser, the three
/// `w32_try_get_data` calls keep C's `format` sequence, and `(LPARAM) hGlobal` is C's cast.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_drop_target_Drop (This: *mut IDropTarget, pDataObj: *mut IDataObject,
                                                grfKeyState: DWORD, pt: POINTL,
                                                pdwEffect: *mut DWORD) -> HRESULT
{
  let target: *mut w32_drop_target = This as *mut w32_drop_target;
  *pdwEffect = DROPEFFECT_COPY;

  let mut msg: W32Msg = core::mem::zeroed ();
  msg.dwModifiers = w32_get_modifiers ();
  msg.msg.time = GetMessageTime () as DWORD;
  msg.msg.pt.x = pt.x;
  msg.msg.pt.y = pt.y;

  let mut format: c_int = CF_HDROP;
  let mut hGlobal: HGLOBAL = w32_try_get_data (pDataObj, format);

  if hGlobal.is_null ()
    {
      format = CF_UNICODETEXT;
      hGlobal = w32_try_get_data (pDataObj, format);
    }

  if hGlobal.is_null ()
    {
      format = CF_TEXT;
      hGlobal = w32_try_get_data (pDataObj, format);
    }

  if ! hGlobal.is_null ()
    { my_post_msg (&raw mut msg, (*target).hwnd, WM_EMACS_DROP as UINT, format as WPARAM,
		   hGlobal as LPARAM); }

  return S_OK;
}

/// C 2706: `static EMACS_INT touch_base;` - a **function-local** static in C (one object with
/// internal linkage and static storage duration).  Rust has no function-scope `static`, so the
/// declaration stands at file scope inside the same gate; C's implicit zero initialiser is what
/// the initialiser spells (the `frame.rs` 4813 `frame_float` reading).
/// C: static EMACS_INT touch_base;
#[cfg (feature = "have_ntgui")]
static mut touch_base: EMACS_INT = 0;

/// C 2701-2706/2783-2842: `<winuser.h>`'s remaining window-style constants and the two entry
/// points this function calls that the sections above do not carry (`AdjustWindowRect` is
/// `<winuser.h>`'s rect helper).  C 2834-2836's `#ifndef GC_ALLGESTURES` is one of the SDK's
/// "define it if the SDK has not" guards, so the constant is carried with C's value and no `cfg`
/// (the first slice's eleven-guard reading).
/// C: #define WS_POPUP 0x80000000L / #define WS_OVERLAPPEDWINDOW (WS_OVERLAPPED | WS_CAPTION |
/// C: WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX) / #define WS_CLIPCHILDREN
/// C: 0x02000000L / #define WS_EX_NOACTIVATE 0x08000000L
/// C: #ifndef GC_ALLGESTURES / #define GC_ALLGESTURES 0x00000001 / #endif /* GC_ALLGESTURES */
/// C: BOOL WINAPI AdjustWindowRect (LPRECT lpRect, DWORD dwStyle, BOOL bMenu);
/// C: HRESULT WINAPI RegisterDragDrop (HWND hwnd, LPDROPTARGET pDropTarget);
/// C: HRESULT WINAPI OleInitialize (LPVOID pvReserved);
#[cfg (feature = "have_ntgui")]
pub const WS_POPUP: DWORD = 0x80000000;
/// C: #define WS_DISABLED 0x08000000 - C 7564's `WS_BORDER | WS_POPUP | WS_DISABLED`.
#[cfg (feature = "have_ntgui")]
pub const WS_DISABLED: DWORD = 0x08000000;
#[cfg (feature = "have_ntgui")]
pub const WS_OVERLAPPED: DWORD = 0x00000000;
#[cfg (feature = "have_ntgui")]
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
                                    | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
#[cfg (feature = "have_ntgui")]
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
#[cfg (feature = "have_ntgui")]
pub const GC_ALLGESTURES: DWORD = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub type LPDROPTARGET = *mut IDropTarget;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn AdjustWindowRect (lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
  pub fn RegisterDragDrop (hwnd: HWND, pDropTarget: LPDROPTARGET) -> HRESULT;
  pub fn OleInitialize (pvReserved: LPVOID) -> HRESULT;
}

/* C 2701-2857.  `w32_createwindow`: C's `int top, left;` are the port's zero initialisers
   (PORTING.md 1.5, the `w32_real_positions` reading above), `f->output_data.w32->dwStyle` is the
   `w32_output` member the section above spells, and C's two tests
   `f->size_hint_flags & USPosition` write the usual-arithmetic conversion out because the
   mirror's `USPosition`/`PPosition` are `c_long` while the member is C's `int` (the
   `gtkutil.rs` 1488 reading).  C's `CreateWindow` is the SDK's `CreateWindowA` macro, whose
   eleven-argument shape the declaration above spells (the arity the four call sites of this file
   use).  */
/// C 2701-2857: `static void w32_createwindow (struct frame *f, int *coords)`
#[cfg (feature = "have_ntgui")]
unsafe fn w32_createwindow (f: *mut frame, coords: *mut c_int)
{
  let mut hwnd: HWND = core::ptr::null_mut ();
  let mut parent_hwnd: HWND = core::ptr::null_mut ();
  let mut rect: RECT = core::mem::zeroed ();
  let mut top: c_int = 0;
  let mut left: c_int = 0;
  let border_width: Lisp_Object = Fcdr (Fassq (Qborder_width (), (*f).param_alist));

  if ! FRAME_PARENT_FRAME (f).is_null () && FRAME_W32_P (FRAME_PARENT_FRAME (f))
    {
      parent_hwnd = FRAME_W32_WINDOW! (FRAME_PARENT_FRAME (f));
      (*((*f).output_data.w32 as *mut w32_output)).dwStyle = WS_CHILD | WS_CLIPSIBLINGS;

      if FRAME_UNDECORATED (f) != 0
	{
	  /* If we want a thin border, specify it here.  */
	  if FIXNUMP (border_width) && (XFIXNUM (border_width) > 0)
	    { (*((*f).output_data.w32 as *mut w32_output)).dwStyle |= WS_BORDER; }
	}
      else
	/* To decorate a child frame, list all needed elements.  */
	{ (*((*f).output_data.w32 as *mut w32_output)).dwStyle |= (WS_THICKFRAME | WS_CAPTION
								  | WS_MAXIMIZEBOX
								  | WS_MINIMIZEBOX
								  | WS_SYSMENU); }
    }
  else if FRAME_UNDECORATED (f) != 0
    {
      /* All attempts to start with ~WS_OVERLAPPEDWINDOW or overlapped
	 with all other style elements negated failed here.  */
      (*((*f).output_data.w32 as *mut w32_output)).dwStyle = WS_POPUP;

      /* If we want a thin border, specify it here.  */
      if FIXNUMP (border_width) && (XFIXNUM (border_width) > 0)
	{ (*((*f).output_data.w32 as *mut w32_output)).dwStyle |= WS_BORDER; }
    }
  else
    { (*((*f).output_data.w32 as *mut w32_output)).dwStyle = WS_OVERLAPPEDWINDOW; }

  /* Always clip children.  */
  (*((*f).output_data.w32 as *mut w32_output)).dwStyle |= WS_CLIPCHILDREN;

  rect.left = 0;
  rect.top = 0;
  rect.right = FRAME_PIXEL_WIDTH (f);
  rect.bottom = FRAME_PIXEL_HEIGHT (f);

  AdjustWindowRect (&raw mut rect, (*((*f).output_data.w32 as *mut w32_output)).dwStyle,
		    (FRAME_EXTERNAL_MENU_BAR (f) && parent_hwnd.is_null ()) as BOOL);

  /* Do first time app init */
  w32_init_class (hinst);

  if (*f).size_hint_flags as c_long & USPosition != 0
      || (*f).size_hint_flags as c_long & PPosition != 0
    {
      left = (*f).left_pos;
      top = (*f).top_pos;
    }
  else
    {
      left = *coords.offset (0);
      top = *coords.offset (1);
    }

  hwnd = CreateWindow (EMACS_CLASS.as_ptr () as *const c_char, (*f).namebuf,
		       (*((*f).output_data.w32 as *mut w32_output)).dwStyle,
		       left, top, rect.right - rect.left, rect.bottom - rect.top,
		       parent_hwnd, core::ptr::null_mut (), hinst, core::ptr::null_mut ());
  FRAME_W32_WINDOW! (f) = hwnd;

  if ! hwnd.is_null ()
    {
      let mut i: c_int;

      if FRAME_SKIP_TASKBAR (f) != 0
	{ SetWindowLong (hwnd, GWL_EXSTYLE,
			 (GetWindowLong (hwnd, GWL_EXSTYLE) | WS_EX_NOACTIVATE as LONG) as LONG); }

      SetWindowLong (hwnd, WND_FONTWIDTH_INDEX, FRAME_COLUMN_WIDTH (f));
      SetWindowLong (hwnd, WND_LINEHEIGHT_INDEX, FRAME_LINE_HEIGHT (f));
      SetWindowLong (hwnd, WND_BORDER_INDEX, FRAME_INTERNAL_BORDER_WIDTH (f));
      SetWindowLong (hwnd, WND_VSCROLLBAR_INDEX, FRAME_SCROLL_BAR_AREA_WIDTH (f));
      SetWindowLong (hwnd, WND_HSCROLLBAR_INDEX, FRAME_SCROLL_BAR_AREA_HEIGHT (f));
      SetWindowLong (hwnd, WND_BACKGROUND_INDEX, FRAME_BACKGROUND_PIXEL (f) as LONG);

      /* Enable drag-n-drop.  */
      let drop_target: *mut w32_drop_target
	= malloc (core::mem::size_of::<w32_drop_target> ()) as *mut w32_drop_target;

      if ! drop_target.is_null ()
	{
	  let vtbl: *mut IDropTargetVtbl
	    = malloc (core::mem::size_of::<IDropTargetVtbl> ()) as *mut IDropTargetVtbl;
	  if ! vtbl.is_null ()
	    {
	      (*drop_target).hwnd = hwnd;
	      (*drop_target).ref_count = 0;
	      (*drop_target).i_drop_target.lpVtbl = vtbl;
	      (*vtbl).QueryInterface = Some (w32_drop_target_QueryInterface);
	      (*vtbl).AddRef = Some (w32_drop_target_AddRef);
	      (*vtbl).Release = Some (w32_drop_target_Release);
	      (*vtbl).DragEnter = Some (w32_drop_target_DragEnter);
	      (*vtbl).DragOver = Some (w32_drop_target_DragOver);
	      (*vtbl).DragLeave = Some (w32_drop_target_DragLeave);
	      (*vtbl).Drop = Some (w32_drop_target_Drop);
	      RegisterDragDrop (hwnd, &raw mut (*drop_target).i_drop_target);
	    }
	  else
	    {
	      free (drop_target as *mut c_void);
	    }
	}

      /* Enable system light/dark theme.  */
      w32_applytheme (hwnd);

      /* Do this to discard the default setting specified by our parent. */
      ShowWindow (hwnd, SW_HIDE);

      /* Update frame positions. */
      GetWindowRect (hwnd, &raw mut rect);

      if ! parent_hwnd.is_null ()
	/* For a child window we have to get its coordinates wrt its
	   parent.  */
	{ MapWindowPoints (HWND_DESKTOP, parent_hwnd, (&raw mut rect) as LPPOINT, 2); }

      /* Enable touch-screen input.  */
      if RegisterTouchWindow_fn.is_some ()
	{
	  let mut cfg: Emacs_GESTURECONFIG = core::mem::zeroed ();

	  (RegisterTouchWindow_fn.unwrap ()) (hwnd, 0);

	  /* Disable Window's emulation of mouse events.  */
	  cfg.dwID = 0;
	  cfg.dwWant = 0;
	  cfg.dwBlock = GC_ALLGESTURES;
	  (SetGestureConfig_fn.unwrap ()) (hwnd, 0, 1, &raw mut cfg,
					   core::mem::size_of_val (&cfg) as UINT);
	}

      /* Reset F's touch point array.  */
      for i in 0..countof! ((*((*f).output_data.w32 as *mut w32_output)).touch_ids)
	{ (*((*f).output_data.w32 as *mut w32_output)).touch_ids[i as usize] = -1; }

      /* Assign an offset for touch points reported to F.  */
      if FIXNUM_OVERFLOW_P (touch_base + MAX_TOUCH_POINTS as EMACS_INT - 1)
	{ touch_base = 0; }
      (*((*f).output_data.w32 as *mut w32_output)).touch_base = touch_base;
      touch_base += MAX_TOUCH_POINTS as EMACS_INT;

      /* Reset the tool bar touch sequence identifier slot.  */
      (*((*f).output_data.w32 as *mut w32_output)).tool_bar_dwID = (-1 as LONG) as DWORD;

      (*f).left_pos = rect.left;
      (*f).top_pos = rect.top;
    }
}

/// C 2859-2869: `static void my_post_msg (W32Msg * wmsg, HWND hwnd, UINT msg, WPARAM wParam,
/// LPARAM lParam)` - C 380-383's forward declaration has no Rust counterpart (PORTING.md
/// 0.1-12), so the definition stands here and the calls above see it (Rust items are
/// order-independent).
#[cfg (feature = "have_ntgui")]
unsafe fn my_post_msg (wmsg: *mut W32Msg, hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM)
{
  (*wmsg).msg.hwnd = hwnd;
  (*wmsg).msg.message = msg;
  (*wmsg).msg.wParam = wParam;
  (*wmsg).msg.lParam = lParam;
  (*wmsg).msg.time = GetMessageTime () as DWORD;

  post_msg (wmsg);
}

/* C 2871-3066.  The keyboard hook callback and its two installers.  The whole family is the
   `windowsnt` arm of the file gate (C 2871's `#ifdef WINDOWSNT` ... C 3154's `#endif`), so every
   item carries both gates.  */
/// C 2877-3042/3434: `<wincon.h>`'s console records and `<winuser.h>`'s input record and hook
/// record, plus the constants the callback compares against.  C's text for each is in the
/// `/// C:` lines; the option bits are typed at the width of the port's member they meet (C's
/// `#define`s are untyped), which the ledger's Deviations record.
/// C: typedef struct _COORD { SHORT X; SHORT Y; } COORD;
/// C: typedef CHAR *LPCH, *PCH;
/// C: #define KEY_EVENT 0x0001
/// C: #define RIGHT_ALT_PRESSED 0x0001 / #define LEFT_ALT_PRESSED 0x0002 /
/// C: #define RIGHT_CTRL_PRESSED 0x0004 / #define LEFT_CTRL_PRESSED 0x0008 /
/// C: #define SHIFT_PRESSED 0x0010 / #define NUMLOCK_ON 0x0020 / #define SCROLLLOCK_ON 0x0040 /
/// C: #define CAPSLOCK_ON 0x0080 / #define ENHANCED_KEY 0x0100
/// C: #define KEYEVENTF_EXTENDEDKEY 0x0001 / #define KEYEVENTF_KEYUP 0x0002
/// C: #define SM_CXSCREEN 0 / #define SM_CYSCREEN 1
/// C: typedef struct tagKEYBDINPUT { WORD wVk; WORD wScan; DWORD dwFlags; DWORD time;
/// C: ULONG_PTR dwExtraInfo; } KEYBDINPUT;
/// C: typedef struct tagMOUSEINPUT { LONG dx; LONG dy; DWORD mouseData; DWORD dwFlags; DWORD time;
/// C: ULONG_PTR dwExtraInfo; } MOUSEINPUT;
/// C: typedef struct tagHARDWAREINPUT { DWORD uMsg; WORD wParamL; WORD wParamH; } HARDWAREINPUT;
/// C: typedef struct tagINPUT { DWORD type; union { MOUSEINPUT mi; KEYBDINPUT ki; HARDWAREINPUT
/// C: hi; } DUMMYUNIONNAME; } INPUT;
/// C: typedef struct tagKBDLLHOOKSTRUCT { DWORD vkCode; DWORD scanCode; DWORD flags; DWORD time;
/// C: ULONG_PTR dwExtraInfo; } KBDLLHOOKSTRUCT, *PKBDLLHOOKSTRUCT, *LPKBDLLHOOKSTRUCT;
/// C: #define LLKHF_EXTENDED (KF_EXTENDED >> 8) / #define LLKHF_INJECTED 0x00000010
/// C: typedef LRESULT (CALLBACK *HOOKPROC) (int code, WPARAM wParam, LPARAM lParam);
/// C: #define WH_KEYBOARD_LL 13
/// C: #define NOTIFY_FOR_THIS_SESSION 0
/// C: #define INPUT_KEYBOARD 1
/// C: typedef HKL *LPHKL;
#[cfg (feature = "have_ntgui")]
pub type SHORT = c_short;
#[cfg (feature = "have_ntgui")]
pub type CHAR = c_char;
#[cfg (feature = "have_ntgui")]
pub type HKL = *mut c_void;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEYBDINPUT
{
  pub wVk: WORD,
  pub wScan: WORD,
  pub dwFlags: DWORD,
  pub time: DWORD,
  pub dwExtraInfo: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MOUSEINPUT
{
  pub dx: LONG,
  pub dy: LONG,
  pub mouseData: DWORD,
  pub dwFlags: DWORD,
  pub time: DWORD,
  pub dwExtraInfo: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HARDWAREINPUT
{
  pub uMsg: DWORD,
  pub wParamL: WORD,
  pub wParamH: WORD,
}
/// C's anonymous union member keeps the SDK's own compatibility name `DUMMYUNIONNAME` (the
/// `w32menu.rs` 648-652 and `w32.rs` 640 readings: `<winnt.h>` defines it as `u` for compilers
/// without anonymous unions), and C's promoted accesses `inputs[0].ki.…` are written with it.
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DUMMYUNIONNAME
{
  pub mi: MOUSEINPUT,
  pub ki: KEYBDINPUT,
  pub hi: HARDWAREINPUT,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT
{
  pub r#type: DWORD,
  pub DUMMYUNIONNAME: DUMMYUNIONNAME,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDLLHOOKSTRUCT
{
  pub vkCode: DWORD,
  pub scanCode: DWORD,
  pub flags: DWORD,
  pub time: DWORD,
  pub dwExtraInfo: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
pub type HOOKPROC = Option<unsafe extern "system" fn (code: c_int, wParam: WPARAM,
                                                      lParam: LPARAM) -> LRESULT>;
/// C 3040-3042: `INPUT_RECORD rec;` and C 2940's `KEY_EVENT_RECORD` - `<wincon.h>`'s records,
/// the same shapes the `w32inevt.c` unit spells (its 159-232).
/// C: typedef struct _KEY_EVENT_RECORD { BOOL bKeyDown; WORD wRepeatCount; WORD wVirtualKeyCode;
/// C: WORD wVirtualScanCode; union { WCHAR UnicodeChar; CHAR AsciiChar; } uChar; DWORD
/// C: dwControlKeyState; } KEY_EVENT_RECORD;
/// C: typedef struct _COORD { SHORT X; SHORT Y; } COORD;
/// C: typedef struct _MOUSE_EVENT_RECORD { COORD dwMousePosition; DWORD dwButtonState; DWORD
/// C: dwControlKeyState; DWORD dwEventFlags; } MOUSE_EVENT_RECORD;
/// C: typedef struct _WINDOW_BUFFER_SIZE_RECORD { COORD dwSize; } WINDOW_BUFFER_SIZE_RECORD;
/// C: typedef struct _MENU_EVENT_RECORD { UINT dwCommandId; } MENU_EVENT_RECORD;
/// C: typedef struct _FOCUS_EVENT_RECORD { BOOL bSetFocus; } FOCUS_EVENT_RECORD;
/// C: typedef struct _INPUT_RECORD { WORD EventType; union { KEY_EVENT_RECORD KeyEvent;
/// C: MOUSE_EVENT_RECORD MouseEvent; WINDOW_BUFFER_SIZE_RECORD WindowBufferSizeEvent;
/// C: MENU_EVENT_RECORD MenuEvent; FOCUS_EVENT_RECORD FocusEvent; } Event; } INPUT_RECORD;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COORD
{
  pub X: SHORT,
  pub Y: SHORT,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_uChar
{
  pub UnicodeChar: WCHAR,
  pub AsciiChar: CHAR,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD
{
  pub bKeyDown: BOOL,
  pub wRepeatCount: WORD,
  pub wVirtualKeyCode: WORD,
  pub wVirtualScanCode: WORD,
  pub uChar: KEY_EVENT_RECORD_uChar,
  pub dwControlKeyState: DWORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MOUSE_EVENT_RECORD
{
  pub dwMousePosition: COORD,
  pub dwButtonState: DWORD,
  pub dwControlKeyState: DWORD,
  pub dwEventFlags: DWORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOW_BUFFER_SIZE_RECORD
{
  pub dwSize: COORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENU_EVENT_RECORD
{
  pub dwCommandId: UINT,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FOCUS_EVENT_RECORD
{
  pub bSetFocus: BOOL,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_RECORD_Event
{
  pub KeyEvent: KEY_EVENT_RECORD,
  pub MouseEvent: MOUSE_EVENT_RECORD,
  pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
  pub MenuEvent: MENU_EVENT_RECORD,
  pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD
{
  pub EventType: WORD,
  pub Event: INPUT_RECORD_Event,
}
#[cfg (feature = "have_ntgui")]
pub const KEY_EVENT: WORD = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const RIGHT_ALT_PRESSED: c_int = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const LEFT_ALT_PRESSED: c_int = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const RIGHT_CTRL_PRESSED: c_int = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const LEFT_CTRL_PRESSED: c_int = 0x0008;
#[cfg (feature = "have_ntgui")]
pub const SHIFT_PRESSED: c_int = 0x0010;
#[cfg (feature = "have_ntgui")]
pub const NUMLOCK_ON: c_int = 0x0020;
#[cfg (feature = "have_ntgui")]
pub const SCROLLLOCK_ON: c_int = 0x0040;
#[cfg (feature = "have_ntgui")]
pub const CAPSLOCK_ON: c_int = 0x0080;
#[cfg (feature = "have_ntgui")]
pub const ENHANCED_KEY: c_int = 0x0100;
#[cfg (feature = "have_ntgui")]
pub const KEYEVENTF_EXTENDEDKEY: DWORD = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const KEYEVENTF_KEYUP: DWORD = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const INPUT_KEYBOARD: DWORD = 1;
#[cfg (feature = "have_ntgui")]
pub const LLKHF_EXTENDED: DWORD = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const LLKHF_INJECTED: DWORD = 0x00000010;
#[cfg (feature = "have_ntgui")]
pub const WH_KEYBOARD_LL: c_int = 13;
#[cfg (feature = "have_ntgui")]
pub const NOTIFY_FOR_THIS_SESSION: DWORD = 0;
/// C 3028's `WH_KEYBOARD_LL` hook and C 2980's virtual-key codes - `<winuser.h>`'s (the mirror
/// carries `VK_LWIN`, `VK_RWIN`, `VK_APPS` and the `VK_NUMPAD_*` family, so only the rest are
/// here).
/// C: #define VK_LBUTTON 0x01 … #define VK_CLEAR 0x0C / #define VK_RETURN 0x0D /
/// C: #define VK_SHIFT 0x10 / #define VK_CONTROL 0x11 / #define VK_MENU 0x12 /
/// C: #define VK_CAPITAL 0x14 / #define VK_PRIOR 0x21 / #define VK_DOWN 0x28 /
/// C: #define VK_INSERT 0x2D / #define VK_DELETE 0x2E / #define VK_LSHIFT 0xA0 /
/// C: #define VK_RSHIFT 0xA1 / #define VK_LCONTROL 0xA2 / #define VK_RCONTROL 0xA3 /
/// C: #define VK_LMENU 0xA4 / #define VK_RMENU 0xA5 / #define VK_NUMLOCK 0x90 /
/// C: #define VK_SCROLL 0x91
#[cfg (feature = "have_ntgui")]
pub const VK_SHIFT: c_int = 0x10;
#[cfg (feature = "have_ntgui")]
pub const VK_CONTROL: c_int = 0x11;
#[cfg (feature = "have_ntgui")]
pub const VK_MENU: c_int = 0x12;
#[cfg (feature = "have_ntgui")]
pub const VK_CAPITAL: c_int = 0x14;
#[cfg (feature = "have_ntgui")]
pub const VK_LSHIFT: c_int = 0xA0;
#[cfg (feature = "have_ntgui")]
pub const VK_RSHIFT: c_int = 0xA1;
#[cfg (feature = "have_ntgui")]
pub const VK_LCONTROL: c_int = 0xA2;
#[cfg (feature = "have_ntgui")]
pub const VK_RCONTROL: c_int = 0xA3;
#[cfg (feature = "have_ntgui")]
pub const VK_LMENU: c_int = 0xA4;
#[cfg (feature = "have_ntgui")]
pub const VK_RMENU: c_int = 0xA5;
#[cfg (feature = "have_ntgui")]
pub const VK_CLEAR: c_int = 0x0C;
#[cfg (feature = "have_ntgui")]
pub const VK_RETURN: c_int = 0x0D;
#[cfg (feature = "have_ntgui")]
pub const VK_PRIOR: c_int = 0x21;
#[cfg (feature = "have_ntgui")]
pub const VK_DOWN: c_int = 0x28;
#[cfg (feature = "have_ntgui")]
pub const VK_INSERT: c_int = 0x2D;
#[cfg (feature = "have_ntgui")]
pub const VK_DELETE: c_int = 0x2E;
#[cfg (feature = "have_ntgui")]
pub const VK_NUMLOCK: c_int = 0x90;
#[cfg (feature = "have_ntgui")]
pub const VK_SCROLL: c_int = 0x91;
/// C 3028-3033: the SDK's keyboard/hook/console/WTS entry points this family calls.
/// C: BOOL WINAPI GetKeyboardState (PBYTE lpKeyState);
/// C: BOOL WINAPI SetKeyboardState (LPBYTE lpKeyState);
/// C: SHORT WINAPI GetKeyState (int nVirtKey);
/// C: SHORT WINAPI GetAsyncKeyState (int vKey);
/// C: HWND WINAPI GetFocus (void);
/// C: HWND WINAPI GetForegroundWindow (void);
/// C: BOOL WINAPI PostMessageA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: LRESULT CALLBACK CallNextHookEx (HHOOK hhk, int nCode, WPARAM wParam, LPARAM lParam);
/// C: UINT WINAPI SendInput (UINT cInputs, LPINPUT pInputs, int cbSize);
/// C: void WINAPI keybd_event (BYTE bVk, BYTE bScan, DWORD dwFlags, ULONG_PTR dwExtraInfo);
/// C: UINT WINAPI MapVirtualKeyA (UINT uCode, UINT uMapType);
/// C: HHOOK WINAPI SetWindowsHookExA (int idHook, HOOKPROC lpfn, HINSTANCE hmod,
/// C: DWORD dwThreadId);
/// C: BOOL WINAPI UnhookWindowsHookEx (HHOOK hhk);
/// C: DWORD WINAPI GetCurrentThreadId (void);
/// C: BOOL WINAPI WriteConsoleInputA (HANDLE hConsoleInput, const INPUT_RECORD *lpBuffer,
/// C: DWORD nLength, LPDWORD lpNumberOfEventsWritten);
/// C: BOOL WINAPI WriteConsoleInputW (HANDLE hConsoleInput, const INPUT_RECORD *lpBuffer,
/// C: DWORD nLength, LPDWORD lpNumberOfEventsWritten);
/// C: DWORD WINAPI GetModuleHandleA (LPCSTR lpModuleName);
/// C: HRESULT WINAPI CoCreateGuid (GUID *pguid);
/// C: int WINAPI StringFromGUID2 (REFGUID rguid, LPOLESTR lpsz, int cchMax);
/// C: DWORD WINAPI GetConsoleTitleW (LPWSTR lpConsoleTitle, DWORD nSize);
/// C: BOOL WINAPI SetConsoleTitleW (LPCWSTR lpConsoleTitle);
/// C: HWND WINAPI FindWindowW (LPCWSTR lpClassName, LPCWSTR lpWindowName);
/// C: HWND WINAPI FindWindowA (LPCSTR lpClassName, LPCSTR lpWindowName);
/// C: void WINAPI Sleep (DWORD dwMilliseconds);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetKeyboardState (lpKeyState: LPBYTE) -> BOOL;
  pub fn SetKeyboardState (lpKeyState: LPBYTE) -> BOOL;
  pub fn GetKeyState (nVirtKey: c_int) -> SHORT;
  pub fn GetAsyncKeyState (vKey: c_int) -> SHORT;
  pub fn GetFocus () -> HWND;
  pub fn GetForegroundWindow () -> HWND;
  pub fn PostMessage (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
  pub fn CallNextHookEx (hhk: HHOOK, nCode: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn SendInput (cInputs: UINT, pInputs: *const INPUT, cbSize: c_int) -> UINT;
  pub fn keybd_event (bVk: BYTE, bScan: BYTE, dwFlags: DWORD, dwExtraInfo: ULONG_PTR);
  pub fn MapVirtualKey (uCode: UINT, uMapType: UINT) -> UINT;
  pub fn SetWindowsHookEx (idHook: c_int, lpfn: HOOKPROC, hmod: HINSTANCE,
                           dwThreadId: DWORD) -> HHOOK;
  pub fn UnhookWindowsHookEx (hhk: HHOOK) -> BOOL;
  pub fn GetCurrentThreadId () -> DWORD;
  pub fn WriteConsoleInputA (hConsoleInput: HANDLE, lpBuffer: *const INPUT_RECORD,
                             nLength: DWORD, lpNumberOfEventsWritten: *mut DWORD) -> BOOL;
  pub fn WriteConsoleInputW (hConsoleInput: HANDLE, lpBuffer: *const INPUT_RECORD,
                             nLength: DWORD, lpNumberOfEventsWritten: *mut DWORD) -> BOOL;
  pub fn CoCreateGuid (pguid: *mut GUID) -> HRESULT;
  pub fn StringFromGUID2 (rguid: *const GUID, lpsz: LPWSTR, cchMax: c_int) -> c_int;
  pub fn GetConsoleTitleW (lpConsoleTitle: LPWSTR, nSize: DWORD) -> DWORD;
  pub fn SetConsoleTitleW (lpConsoleTitle: *const WCHAR) -> BOOL;
  pub fn FindWindowW (lpClassName: *const WCHAR, lpWindowName: *const WCHAR) -> HWND;
  pub fn FindWindowA (lpClassName: *const c_char, lpWindowName: *const c_char) -> HWND;
  pub fn Sleep (dwMilliseconds: DWORD);
}

/* The Windows keyboard hook callback.  */
/// C 2874-3066: `static LRESULT CALLBACK funhook (int code, WPARAM w, LPARAM l)`.  `CALLBACK` is
/// `__stdcall` (the `WNDPROC` reading above), C's `get_console != NULL` is the port's integer
/// test of a raw function pointer, and C's `GetKeyState`/`GetAsyncKeyState` results (Windows'
/// 16-bit `SHORT`) promote to `int` inside C's `&` expressions, which the port writes out.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" fn funhook (code: c_int, w: WPARAM, l: LPARAM) -> LRESULT
{
  let mut inputs: [INPUT; 2] = core::mem::zeroed ();
  let mut focus: HWND = GetFocus ();
  let mut console: c_int = 0;
  let hs: *const KBDLLHOOKSTRUCT = l as *const KBDLLHOOKSTRUCT;

  if code < 0 || (*hs).flags & LLKHF_INJECTED != 0
    { return CallNextHookEx (core::ptr::null_mut (), code, w, l); }

  /* The keyboard hook sees keyboard input on all processes (except
     elevated ones, when Emacs itself is not elevated).  As such,
     care must be taken to only filter out keyboard input when Emacs
     itself is on the foreground.

     GetFocus returns a non-NULL window if another application is active,
     and always for a console Emacs process.  For a console Emacs, determine
     focus by checking if the current foreground window is the process's
     console window.  */
  if focus.is_null () && ! kbdhook.console.is_null ()
    {
      if GetForegroundWindow () == kbdhook.console
	{
	  focus = kbdhook.console;
	  console = 1;
	}
    }

  /* First, check hooks for the left and right Windows keys.  */
  if (*hs).vkCode == VK_LWIN as DWORD || (*hs).vkCode == VK_RWIN as DWORD
    {
      if ! focus.is_null () && (w as c_int == WM_KEYDOWN || w as c_int == WM_SYSKEYDOWN)
	{
	  /* The key is being pressed in an Emacs window.  */
	  if (*hs).vkCode == VK_LWIN as DWORD && kbdhook.lwindown == 0
	    {
	      kbdhook.lwindown = 1;
	      kbdhook.winseen = 1;
	      kbdhook.winsdown += 1;
	    }
	  else if (*hs).vkCode == VK_RWIN as DWORD && kbdhook.rwindown == 0
	    {
	      kbdhook.rwindown = 1;
	      kbdhook.winseen = 1;
	      kbdhook.winsdown += 1;
	    }
	  /* Returning 1 here drops the keypress without further processing.
	     If the keypress was allowed to go through, the normal Windows
	     hotkeys would take over.  */
	  return 1;
	}
      else if kbdhook.winsdown > 0 && (w as c_int == WM_KEYUP || w as c_int == WM_SYSKEYUP)
	{
	  /* A key that has been captured earlier is being released now.  */
	  if (*hs).vkCode == VK_LWIN as DWORD && kbdhook.lwindown != 0
	    {
	      kbdhook.lwindown = 0;
	      kbdhook.winsdown -= 1;
	    }
	  else if (*hs).vkCode == VK_RWIN as DWORD && kbdhook.rwindown != 0
	    {
	      kbdhook.rwindown = 0;
	      kbdhook.winsdown -= 1;
	    }
	  if kbdhook.winsdown == 0 && kbdhook.winseen != 0
	    {
	      if kbdhook.suppress_lone == 0
		{
		  /* The Windows key was pressed, then released,
		     without any other key pressed simultaneously.
		     Normally, this opens the Start menu, but the user
		     can prevent this by setting the
		     w32-pass-[lr]window-to-system variable to
		     NIL.  */
		  if (*hs).vkCode == VK_LWIN as DWORD
		       && ! NILP (Vw32_pass_lwindow_to_system)
		     || (*hs).vkCode == VK_RWIN as DWORD
		       && ! NILP (Vw32_pass_rwindow_to_system)
		    {
		      /* Not prevented - Simulate the keypress to the system.  */
		      memset (inputs.as_mut_ptr () as *mut c_void, 0,
			      core::mem::size_of_val (&inputs));
		      inputs[0].r#type = INPUT_KEYBOARD;
		      inputs[0].DUMMYUNIONNAME.ki.wVk = (*hs).vkCode as WORD;
		      inputs[0].DUMMYUNIONNAME.ki.wScan = (*hs).vkCode as WORD;
		      inputs[0].DUMMYUNIONNAME.ki.dwFlags = KEYEVENTF_EXTENDEDKEY;
		      inputs[0].DUMMYUNIONNAME.ki.time = 0;
		      inputs[1].r#type = INPUT_KEYBOARD;
		      inputs[1].DUMMYUNIONNAME.ki.wVk = (*hs).vkCode as WORD;
		      inputs[1].DUMMYUNIONNAME.ki.wScan = (*hs).vkCode as WORD;
		      inputs[1].DUMMYUNIONNAME.ki.dwFlags
			= KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP;
		      inputs[1].DUMMYUNIONNAME.ki.time = 0;
		      SendInput (2, inputs.as_mut_ptr (), core::mem::size_of::<INPUT> () as c_int);
		    }
		  else if ! focus.is_null ()
		    {
		      /* When not passed to system, must simulate privately to Emacs.  */
		      PostMessage (focus, WM_SYSKEYDOWN as UINT, (*hs).vkCode as WPARAM, 0);
		      PostMessage (focus, WM_SYSKEYUP as UINT, (*hs).vkCode as WPARAM, 0);
		    }
		}
	    }
	  if kbdhook.winsdown == 0
	    {
	      /* No Windows keys pressed anymore - clear the state flags.  */
	      kbdhook.suppress_lone = 0;
	      kbdhook.winseen = 0;
	    }
	  if kbdhook.send_win_up == 0
	    {
	      /* Swallow this release message, as not to confuse
		 applications who did not get to see the original
		 WM_KEYDOWN message either.  */
	      return 1;
	    }
	  kbdhook.send_win_up = 0;
	}
    }
  else if kbdhook.winsdown > 0
    {
      /* Some other key was pressed while a captured Win key is down.
	 This is either an Emacs registered hotkey combination, or a
	 system hotkey.	 */
      if kbdhook.lwindown != 0 && kbdhook.lwin_hooked[(*hs).vkCode as usize] != 0
	  || kbdhook.rwindown != 0 && kbdhook.rwin_hooked[(*hs).vkCode as usize] != 0
	{
	  /* Hooked Win-x combination, do not pass the keypress to Windows.  */
	  kbdhook.suppress_lone = 1;
	}
      else if kbdhook.suppress_lone == 0
	{
	  /* Unhooked S-x combination; simulate the combination now
	     (will be seen by the system).  */
	  memset (inputs.as_mut_ptr () as *mut c_void, 0, core::mem::size_of_val (&inputs));
	  inputs[0].r#type = INPUT_KEYBOARD;
	  inputs[0].DUMMYUNIONNAME.ki.wVk
	    = (if kbdhook.lwindown != 0 { VK_LWIN } else { VK_RWIN }) as WORD;
	  inputs[0].DUMMYUNIONNAME.ki.wScan
	    = (if kbdhook.lwindown != 0 { VK_LWIN } else { VK_RWIN }) as WORD;
	  inputs[0].DUMMYUNIONNAME.ki.dwFlags = KEYEVENTF_EXTENDEDKEY;
	  inputs[0].DUMMYUNIONNAME.ki.time = 0;
	  inputs[1].r#type = INPUT_KEYBOARD;
	  inputs[1].DUMMYUNIONNAME.ki.wVk = (*hs).vkCode as WORD;
	  inputs[1].DUMMYUNIONNAME.ki.wScan = (*hs).scanCode as WORD;
	  inputs[1].DUMMYUNIONNAME.ki.dwFlags
	    = if (*hs).flags & LLKHF_EXTENDED != 0 { KEYEVENTF_EXTENDEDKEY } else { 0 };
	  inputs[1].DUMMYUNIONNAME.ki.time = 0;
	  SendInput (2, inputs.as_mut_ptr (), core::mem::size_of::<INPUT> () as c_int);
	  /* Stop processing of this Win sequence here; the
	     corresponding keyup messages will come through the normal
	     channel when the keys are released.  */
	  kbdhook.suppress_lone = 1;
	  kbdhook.send_win_up = 1;
	  /* Swallow the original keypress (as we want the Win key
	     down message simulated above to precede this real message).  */
	  return 1;
	}
    }

  /* Next, handle the registered Alt-* combinations.  */
  if (w as c_int == WM_SYSKEYDOWN || w as c_int == WM_KEYDOWN)
      && kbdhook.alt_hooked[(*hs).vkCode as usize] != 0
      && ! focus.is_null ()
      && (GetAsyncKeyState (VK_MENU) as c_int & 0x8000) != 0
    {
      /* Prevent the system from getting this Alt-* key - suppress the
	 message and post as a normal keypress to Emacs.  */
      if console != 0
	{
	  let mut rec: INPUT_RECORD = core::mem::zeroed ();
	  let mut n: DWORD = 0;
	  rec.EventType = KEY_EVENT;
	  rec.Event.KeyEvent.bKeyDown = TRUE;
	  rec.Event.KeyEvent.wVirtualKeyCode = (*hs).vkCode as WORD;
	  rec.Event.KeyEvent.wVirtualScanCode = (*hs).scanCode as WORD;
	  rec.Event.KeyEvent.uChar.UnicodeChar = 0;
	  rec.Event.KeyEvent.dwControlKeyState
	    = ((if GetAsyncKeyState (VK_LMENU) as c_int & 0x8000 != 0
		   { LEFT_ALT_PRESSED } else { 0 })
	       | (if GetAsyncKeyState (VK_RMENU) as c_int & 0x8000 != 0
		    { RIGHT_ALT_PRESSED } else { 0 })
	       | (if GetAsyncKeyState (VK_LCONTROL) as c_int & 0x8000 != 0
		    { LEFT_CTRL_PRESSED } else { 0 })
	       | (if GetAsyncKeyState (VK_RCONTROL) as c_int & 0x8000 != 0
		    { RIGHT_CTRL_PRESSED } else { 0 })
	       | (if GetAsyncKeyState (VK_SHIFT) as c_int & 0x8000 != 0
		    { SHIFT_PRESSED } else { 0 })
	       | (if (*hs).flags & LLKHF_EXTENDED != 0 { ENHANCED_KEY } else { 0 })) as DWORD;
	  if w32_console_unicode_input != 0
	    { WriteConsoleInputW (keyboard_handle, &raw const rec, 1, &raw mut n); }
	  else
	    { WriteConsoleInputA (keyboard_handle, &raw const rec, 1, &raw mut n); }
	}
      else
	{ PostMessage (focus, w as UINT, (*hs).vkCode as WPARAM, 1 | (1 << 29)); }
      return 1;
    }

  /* The normal case - pass the message through.  */
  return CallNextHookEx (core::ptr::null_mut (), code, w, l);
}

/// C 3096/3126: `GetModuleHandle ("kernel32.dll")` and `GetModuleHandle (NULL)` - the entry point
/// the first slice's `w32_load_cursor` (C 2381-2394) also calls; no mirror carries it, so the
/// single declaration stands here (P-88 ①), where C 3096 names it.
/// C: HMODULE WINAPI GetModuleHandleA (LPCSTR lpModuleName);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetModuleHandle (lpModuleName: *const c_char) -> HMODULE;
}

/* Set up the hook; can be called several times, with matching
   remove_w32_kbdhook calls.  */
/// C 3070-3137: `void setup_w32_kbdhook (HWND hwnd)` - a non-`static` definition, so it carries
/// `#[unsafe(no_mangle)]`.  C's `(GetConsoleWindow_Proc) get_proc_addr (…)` is the port's
/// `transmute` of the `VOIDFNPTR` the header returns (the `LOAD_DLL_FN` reading, `w32common.h`
/// 82-89), and C's `get_console != NULL` is a raw function pointer's integer test.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_w32_kbdhook (hwnd: HWND)
{
  kbdhook.hook_count += 1;

  /* This hook gets in the way of debugging, since when Emacs stops,
     its input thread stops, and there's nothing to process keyboard
     events, whereas this hook is global, and is invoked in the
     context of the thread that installed it.  So we don't install the
     hook if the process is being debugged. */
  if w32_kbdhook_active! ()
    {
      if is_debugger_present.is_some () && (is_debugger_present.unwrap ()) () != 0
	{ return; }
    }

  /* Hooking is only available on NT architecture systems, as
     indicated by the w32_kbdhook_active variable.  */
  if kbdhook.hook_count == 1 && w32_kbdhook_active! ()
    {
      /* Get the handle of the Emacs console window.  As the
	 GetConsoleWindow function is only available on Win2000+, a
	 hackish workaround described in Microsoft KB article 124103
	 (https://support.microsoft.com/en-us/kb/124103) is used for
	 NT 4 systems.  */
      let get_console: GetConsoleWindow_Proc
	= core::mem::transmute (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
					       c"GetConsoleWindow".as_ptr ()));

      if get_console as usize != 0
	{ kbdhook.console = get_console (); }
      else
	{
	  let mut guid: GUID = core::mem::zeroed ();
	  let oldTitle: *mut c_ushort
	    = malloc (1024 * core::mem::size_of::<c_ushort> ()) as *mut c_ushort;
	  let mut newTitle: [c_ushort; 64] = [0; 64];
	  let mut i: c_int;

	  CoCreateGuid (&raw mut guid);
	  if ! oldTitle.is_null () && StringFromGUID2 (&raw const guid, newTitle.as_mut_ptr (), 64) != 0
	    {
	      GetConsoleTitleW (oldTitle, 1024);
	      SetConsoleTitleW (newTitle.as_ptr ());
	      for i in 0..25
		{
		  Sleep (40);
		  kbdhook.console = FindWindowW (core::ptr::null (), newTitle.as_ptr ());
		  if ! kbdhook.console.is_null ()
		    { break; }
		}
	      SetConsoleTitleW (oldTitle);
	    }
	  free (oldTitle as *mut c_void);
	}

      /* Set the hook.  */
      kbdhook.hook = SetWindowsHookEx (WH_KEYBOARD_LL, Some (funhook),
				       GetModuleHandle (core::ptr::null ()), 0);

      /* Register session notifications so we get notified about the
	 computer being locked.  */
      kbdhook.notified_wnd = core::ptr::null_mut ();
      if ! hwnd.is_null () && WTSRegisterSessionNotification_fn.is_some ()
	{
	  (WTSRegisterSessionNotification_fn.unwrap ()) (hwnd, NOTIFY_FOR_THIS_SESSION);
	  kbdhook.notified_wnd = hwnd;
	}
    }
}

/* Remove the hook.  */
/// C 3140-3153: `void remove_w32_kbdhook (void)` - the second non-`static` definition of the
/// family.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_w32_kbdhook ()
{
  kbdhook.hook_count -= 1;
  if kbdhook.hook_count == 0 && w32_kbdhook_active! ()
    {
      UnhookWindowsHookEx (kbdhook.hook);
      if ! kbdhook.notified_wnd.is_null ()
	  && WTSUnRegisterSessionNotification_fn.is_some ()
	{ (WTSUnRegisterSessionNotification_fn.unwrap ()) (kbdhook.notified_wnd); }
      kbdhook.hook = core::ptr::null_mut ();
      kbdhook.notified_wnd = core::ptr::null_mut ();
    }
}

/* Mark a specific key combination as hooked, preventing it to be
   handled by the system.  */
/// C 3158-3194: `static void hook_w32_key (int hook, int modifier, int vkey)` - outside the
/// `WINDOWSNT` arm (C 3154's `#endif` closes it), so only the file gate applies.  C's `char *tbl`
/// takes the address of one of the record's three 256-byte tables, and its `(char)hook` casts
/// narrow the value to C's `char` before both stores (written out here).
#[cfg (feature = "have_ntgui")]
unsafe fn hook_w32_key (hook: c_int, modifier: c_int, vkey: c_int)
{
  let mut tbl: *mut c_char = core::ptr::null_mut ();

  match modifier
    {
    VK_MENU =>
      {
	tbl = kbdhook.alt_hooked.as_mut_ptr ();
      }
    VK_LWIN =>
      {
	tbl = kbdhook.lwin_hooked.as_mut_ptr ();
      }
    VK_RWIN =>
      {
	tbl = kbdhook.rwin_hooked.as_mut_ptr ();
      }
    _ => {}
    }

  if ! tbl.is_null () && vkey >= 0 && vkey <= 255
    {
       /* VK_ANY hooks all keys for this modifier */
       if vkey == VK_ANY
	 { memset (tbl as *mut c_void, hook as c_char as c_int, 256); }
       else
	 { *tbl.offset (vkey as isize) = hook as c_char; }
       /* Alt-<modifier>s should go through */
       kbdhook.alt_hooked[VK_MENU as usize] = 0;
       kbdhook.alt_hooked[VK_LMENU as usize] = 0;
       kbdhook.alt_hooked[VK_RMENU as usize] = 0;
       kbdhook.alt_hooked[VK_CONTROL as usize] = 0;
       kbdhook.alt_hooked[VK_LCONTROL as usize] = 0;
       kbdhook.alt_hooked[VK_RCONTROL as usize] = 0;
       kbdhook.alt_hooked[VK_SHIFT as usize] = 0;
       kbdhook.alt_hooked[VK_LSHIFT as usize] = 0;
       kbdhook.alt_hooked[VK_RSHIFT as usize] = 0;
    }
}

/* Check the current Win key pressed state.  */
/// C 3198-3223: `int check_w32_winkey_state (int vkey)` - inside C 3196's `#ifdef WINDOWSNT`
/// arm, so it carries both gates; C's comment is kept below C's own position.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_w32_winkey_state (vkey: c_int) -> c_int
{
  /* The hook code handles grabbing of the Windows keys and Alt-* key
     combinations reserved by the system.  Handling Alt is a bit
     easier, as Windows intends Alt-* shortcuts for application use in
     Windows; hotkeys such as Alt-tab and Alt-escape are special
     cases.  Win-* hotkeys, on the other hand, are primarily meant for
     system use.

     As a result, when we want Emacs to be able to grab the Win-*
     keys, we must swallow all Win key presses in a low-level keyboard
     hook.  Unfortunately, this means that the Emacs window procedure
     (and console input handler) never see the keypresses either.
     Thus, to check the modifier states properly, Emacs code must use
     the check_w32_winkey_state function that uses the flags directly
     updated by the hook callback.  */
  match vkey
    {
    VK_LWIN =>
      {
	return kbdhook.lwindown;
      }
    VK_RWIN =>
      {
	return kbdhook.rwindown;
      }
    _ => {}
    }
  return 0;
}

/* Reset the keyboard hook state.  Locking the workstation with Win-L
   leaves the Win key(s) "down" from the hook's point of view - the
   keyup event is never seen.  Thus, this function must be called when
   the system is locked.  */
/// C 3229-3238: `void reset_w32_kbdhook_state (void)` - the third non-`static` definition of the
/// family, inside the same `WINDOWSNT` arm.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_w32_kbdhook_state ()
{
  kbdhook.lwindown = 0;
  kbdhook.rwindown = 0;
  kbdhook.winsdown = 0;
  kbdhook.send_win_up = 0;
  kbdhook.suppress_lone = 0;
  kbdhook.winseen = 0;
}

/* GetKeyState and MapVirtualKey on Windows 95 do not actually distinguish
   between left and right keys as advertised.  We test for this
   support dynamically, and set a flag when the support is absent.  If
   absent, we keep track of the left and right control and alt keys
   ourselves.  This is particularly necessary on keyboards that rely
   upon the AltGr key, which is represented as having the left control
   and right alt keys pressed.  For these keyboards, we need to know
   when the left alt key has been pressed in addition to the AltGr key
   so that we can properly support M-AltGr-key sequences (such as M-@
   on Swedish keyboards).  */

/// C 3252-3255: the four indices of C's `modifiers` array.  C's `#define`s are untyped; the port
/// types them `usize` because every use is an array subscript (the `modifiers[i]` writes below
/// and the four reads of `modifier_set`).
/// C: #define EMACS_LCONTROL 0
/// C: #define EMACS_RCONTROL 1
/// C: #define EMACS_LMENU    2
/// C: #define EMACS_RMENU    3
#[cfg (feature = "have_ntgui")]
pub const EMACS_LCONTROL: usize = 0;
#[cfg (feature = "have_ntgui")]
pub const EMACS_RCONTROL: usize = 1;
#[cfg (feature = "have_ntgui")]
pub const EMACS_LMENU: usize = 2;
#[cfg (feature = "have_ntgui")]
pub const EMACS_RMENU: usize = 3;

/// C 3257-3259: the three file-scope `static int`s of the modifier family.
/// C: static int modifiers[4];
/// C: static int modifiers_recorded;
/// C: static int modifier_key_support_tested;
#[cfg (feature = "have_ntgui")]
static mut modifiers: [c_int; 4] = [0; 4];
#[cfg (feature = "have_ntgui")]
static mut modifiers_recorded: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut modifier_key_support_tested: c_int = 0;

/// C 3262-3283: `static void test_modifier_support (unsigned int wparam)`.  C's
/// `GetKeyState (l) & 0x8000` promotes the SDK's 16-bit `SHORT` to `int`, which the port writes
/// out (the `funhook` reading).
#[cfg (feature = "have_ntgui")]
unsafe fn test_modifier_support (wparam: c_uint)
{
  let mut l: c_uint = 0;
  let mut r: c_uint = 0;

  if wparam != VK_CONTROL as c_uint && wparam != VK_MENU as c_uint
    { return; }
  if wparam == VK_CONTROL as c_uint
    {
      l = VK_LCONTROL as c_uint;
      r = VK_RCONTROL as c_uint;
    }
  else
    {
      l = VK_LMENU as c_uint;
      r = VK_RMENU as c_uint;
    }
  if (GetKeyState (l as c_int) as c_int) & 0x8000 == 0
      && (GetKeyState (r as c_int) as c_int) & 0x8000 == 0
    { modifiers_recorded = 1; }
  else
    { modifiers_recorded = 0; }
  modifier_key_support_tested = 1;
}

/// C 3285-3302: `static void record_keydown (unsigned int wparam, unsigned int lparam)`.  C's
/// `int i` holds one of the four `EMACS_*` indices, so the port types it `usize`.
#[cfg (feature = "have_ntgui")]
unsafe fn record_keydown (wparam: c_uint, lparam: c_uint)
{
  let mut i: usize = 0;

  if modifier_key_support_tested == 0
    { test_modifier_support (wparam); }

  if (wparam != VK_CONTROL as c_uint && wparam != VK_MENU as c_uint)
      || modifiers_recorded == 0
    { return; }

  if wparam == VK_CONTROL as c_uint
    { i = if lparam & 0x1000000 != 0 { EMACS_RCONTROL } else { EMACS_LCONTROL }; }
  else
    { i = if lparam & 0x1000000 != 0 { EMACS_RMENU } else { EMACS_LMENU }; }

  modifiers[i] = 1;
}

/// C 3304-3318: `static void record_keyup (unsigned int wparam, unsigned int lparam)` - the same
/// index arithmetic, storing 0.
#[cfg (feature = "have_ntgui")]
unsafe fn record_keyup (wparam: c_uint, lparam: c_uint)
{
  let mut i: usize = 0;

  if (wparam != VK_CONTROL as c_uint && wparam != VK_MENU as c_uint) || modifiers_recorded == 0
    { return; }

  if wparam == VK_CONTROL as c_uint
    { i = if lparam & 0x1000000 != 0 { EMACS_RCONTROL } else { EMACS_LCONTROL }; }
  else
    { i = if lparam & 0x1000000 != 0 { EMACS_RMENU } else { EMACS_LMENU }; }

  modifiers[i] = 0;
}

/* Emacs can lose focus while a modifier key has been pressed.  When
   it regains focus, be conservative and clear all modifiers since
   we cannot reconstruct the left and right modifier state.  */
/// C 3323-3365: `static void reset_modifiers (void)`.  The `CURRENT_STATE` `#define` of C 3349 is
/// the port's `macro_rules!` at C's own position inside the block (its argument is evaluated once
/// on each side, PORTING.md 2.1), and C's `BYTE keystate[256]` meets a pointer parameter, so the
/// port passes `keystate.as_mut_ptr ()` where C's array decays.
#[cfg (feature = "have_ntgui")]
unsafe fn reset_modifiers ()
{
  let mut ctrl: SHORT = 0;
  let mut alt: SHORT = 0;

  if GetFocus ().is_null ()
    /* Emacs doesn't have keyboard focus.  Do nothing.  */
    { return; }

  ctrl = GetAsyncKeyState (VK_CONTROL);
  alt = GetAsyncKeyState (VK_MENU);

  if (ctrl as c_int) & 0x08000 == 0
    /* Clear any recorded control modifier state.  */
    { modifiers[EMACS_RCONTROL] = 0; modifiers[EMACS_LCONTROL] = 0; }

  if (alt as c_int) & 0x08000 == 0
    /* Clear any recorded alt modifier state.  */
    { modifiers[EMACS_RMENU] = 0; modifiers[EMACS_LMENU] = 0; }

  /* Update the state of all modifier keys, because modifiers used in
     hot-key combinations can get stuck on if Emacs loses focus as a
     result of a hot-key being pressed.  */
  {
    let mut keystate: [BYTE; 256] = [0; 256];

    /// C: #define CURRENT_STATE(key) ((GetAsyncKeyState (key) & 0x8000) >> 8)
    macro_rules! CURRENT_STATE {
      ($key:expr) => { ((GetAsyncKeyState ($key) as c_int & 0x8000) >> 8) };
    }

    memset (keystate.as_mut_ptr () as *mut c_void, 0, core::mem::size_of_val (&keystate));
    GetKeyboardState (keystate.as_mut_ptr ());
    keystate[VK_SHIFT as usize] = CURRENT_STATE! (VK_SHIFT) as BYTE;
    keystate[VK_CONTROL as usize] = CURRENT_STATE! (VK_CONTROL) as BYTE;
    keystate[VK_LCONTROL as usize] = CURRENT_STATE! (VK_LCONTROL) as BYTE;
    keystate[VK_RCONTROL as usize] = CURRENT_STATE! (VK_RCONTROL) as BYTE;
    keystate[VK_MENU as usize] = CURRENT_STATE! (VK_MENU) as BYTE;
    keystate[VK_LMENU as usize] = CURRENT_STATE! (VK_LMENU) as BYTE;
    keystate[VK_RMENU as usize] = CURRENT_STATE! (VK_RMENU) as BYTE;
    keystate[VK_LWIN as usize] = CURRENT_STATE! (VK_LWIN) as BYTE;
    keystate[VK_RWIN as usize] = CURRENT_STATE! (VK_RWIN) as BYTE;
    keystate[VK_APPS as usize] = CURRENT_STATE! (VK_APPS) as BYTE;
    SetKeyboardState (keystate.as_mut_ptr ());
  }
}

/* Synchronize modifier state with what is reported with the current
   keystroke.  Even if we cannot distinguish between left and right
   modifier keys, we know that, if no modifiers are set, then neither
   the left or right modifier should be set.  */
/// C 3371-3382.
#[cfg (feature = "have_ntgui")]
unsafe fn sync_modifiers ()
{
  if modifiers_recorded == 0
    { return; }

  if (GetKeyState (VK_CONTROL) as c_int) & 0x8000 == 0
    { modifiers[EMACS_RCONTROL] = 0; modifiers[EMACS_LCONTROL] = 0; }

  if (GetKeyState (VK_MENU) as c_int) & 0x8000 == 0
    { modifiers[EMACS_RMENU] = 0; modifiers[EMACS_LMENU] = 0; }
}

/// C 3384-3435: `static int modifier_set (int vkey)`.  C's comment is kept, and C 3415-3418's
/// `#ifdef WINDOWSNT` arm is the port's `#[cfg (feature = "windowsnt")]` block inside the body
/// (the file gate covers the item).
#[cfg (feature = "have_ntgui")]
unsafe fn modifier_set (vkey: c_int) -> c_int
{
  /* Warning: The fact that VK_NUMLOCK is not treated as the other 2
     toggle keys is not an omission!  If you want to add it, you will
     have to make changes in the default sub-case of the WM_KEYDOWN
     switch, because if the NUMLOCK modifier is set, the code there
     will directly convert any key that looks like an ASCII letter,
     and also downcase those that look like upper-case ASCII.  */
  if vkey == VK_CAPITAL
    {
      if NILP (Vw32_enable_caps_lock)
	{ return 0; }
      else
	{ return GetKeyState (vkey) as c_int & 0x1; }
    }
  if vkey == VK_SCROLL
    {
      if NILP (Vw32_scroll_lock_modifier)
	  /* w32-scroll-lock-modifier can be any non-nil value that is
	     not one of the modifiers, in which case it shall be ignored.  */
	  || ! (   EQ (Vw32_scroll_lock_modifier, Qhyper ())
		|| EQ (Vw32_scroll_lock_modifier, Qsuper ())
		|| EQ (Vw32_scroll_lock_modifier, Qmeta ())
		|| EQ (Vw32_scroll_lock_modifier, Qalt ())
		|| EQ (Vw32_scroll_lock_modifier, Qcontrol ())
		|| EQ (Vw32_scroll_lock_modifier, Qshift ()))
	{ return 0; }
      else
	{ return GetKeyState (vkey) as c_int & 0x1; }
    }
  #[cfg (feature = "windowsnt")]
  {
    if w32_kbdhook_active! () && (vkey == VK_LWIN || vkey == VK_RWIN)
      { return check_w32_winkey_state (vkey); }
  }

  if modifiers_recorded == 0
    { return GetKeyState (vkey) as c_int & 0x8000; }

  match vkey
    {
    VK_LCONTROL =>
      {
	return modifiers[EMACS_LCONTROL];
      }
    VK_RCONTROL =>
      {
	return modifiers[EMACS_RCONTROL];
      }
    VK_LMENU =>
      {
	return modifiers[EMACS_LMENU];
      }
    VK_RMENU =>
      {
	return modifiers[EMACS_RMENU];
      }
    _ => {}
    }
  return GetKeyState (vkey) as c_int & 0x8000;
}

/* Convert between the modifier bits W32 uses and the modifier bits
   Emacs uses.  */
/* C 3437-3439: C's `unsigned int w32_key_to_modifier (int);` forward declaration has no Rust
   counterpart (PORTING.md 0.1-12); the definition stands at C's position below.  */
/// C 3441-3487: `unsigned int w32_key_to_modifier (int key)` - a non-`static` definition, so it
/// carries `#[unsafe(no_mangle)]`.  C's `Lisp_Object key_mapping;` is assigned by every arm of
/// the `switch` (its `default:` too), which Rust accepts as a deferred initialiser, and C's
/// `int`-valued modifier constants convert to the declared `unsigned int` return, which the port
/// writes out.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_key_to_modifier (key: c_int) -> c_uint
{
  let mut key_mapping: Lisp_Object;

  match key
    {
    VK_LWIN =>
      {
	key_mapping = Vw32_lwindow_modifier;
      }
    VK_RWIN =>
      {
	key_mapping = Vw32_rwindow_modifier;
      }
    VK_APPS =>
      {
	key_mapping = Vw32_apps_modifier;
      }
    VK_SCROLL =>
      {
	key_mapping = Vw32_scroll_lock_modifier;
      }
    _ =>
      {
	key_mapping = Qnil ();
      }
    }

  /* NB. This code runs in the input thread, asynchronously to the lisp
     thread, so we must be careful to ensure access to lisp data is
     thread-safe.  The following code is safe because the modifier
     variable values are updated atomically from lisp and symbols are
     not relocated by GC.  Also, we don't have to worry about seeing GC
     markbits here.  */
  if EQ (key_mapping, Qhyper ())
    { return hyper_modifier as c_uint; }
  if EQ (key_mapping, Qsuper ())
    { return super_modifier as c_uint; }
  if EQ (key_mapping, Qmeta ())
    { return meta_modifier as c_uint; }
  if EQ (key_mapping, Qalt ())
    { return alt_modifier as c_uint; }
  if EQ (key_mapping, Qctrl ())
    { return ctrl_modifier as c_uint; }
  if EQ (key_mapping, Qcontrol ()) /* synonym for ctrl */
    { return ctrl_modifier as c_uint; }
  if EQ (key_mapping, Qshift ())
    { return shift_modifier as c_uint; }

  /* Don't generate any modifier if not explicitly requested.  */
  return 0;
}

/// C 3489-3500: `static unsigned int w32_get_modifiers (void)` - C 380-383's forward declaration
/// has no Rust counterpart (PORTING.md 0.1-12).
#[cfg (feature = "have_ntgui")]
unsafe fn w32_get_modifiers () -> c_uint
{
  return ((if modifier_set (VK_SHIFT) != 0 { shift_modifier as c_uint } else { 0 })
	  | (if modifier_set (VK_CONTROL) != 0 { ctrl_modifier as c_uint } else { 0 })
	  | (if modifier_set (VK_LWIN) != 0 { w32_key_to_modifier (VK_LWIN) } else { 0 })
	  | (if modifier_set (VK_RWIN) != 0 { w32_key_to_modifier (VK_RWIN) } else { 0 })
	  | (if modifier_set (VK_APPS) != 0 { w32_key_to_modifier (VK_APPS) } else { 0 })
	  | (if modifier_set (VK_SCROLL) != 0 { w32_key_to_modifier (VK_SCROLL) } else { 0 })
	  | (if modifier_set (VK_MENU) != 0
	     { if NILP (Vw32_alt_is_meta) { alt_modifier as c_uint } else { meta_modifier as c_uint } }
	     else { 0 }));
}

/* We map the VK_* modifiers into console modifier constants
   so that we can use the same routines to handle both console
   and window input.  */

/// C 3506-3525: `static int w32_construct_console_modifiers (void)`.
#[cfg (feature = "have_ntgui")]
unsafe fn w32_construct_console_modifiers () -> c_int
{
  let mut mods: c_int;

  mods = 0;
  mods |= if modifier_set (VK_SHIFT) != 0 { SHIFT_PRESSED } else { 0 };
  mods |= if modifier_set (VK_CAPITAL) != 0 { CAPSLOCK_ON } else { 0 };
  mods |= if modifier_set (VK_SCROLL) != 0 { SCROLLLOCK_ON } else { 0 };
  mods |= if modifier_set (VK_NUMLOCK) != 0 { NUMLOCK_ON } else { 0 };
  mods |= if modifier_set (VK_LCONTROL) != 0 { LEFT_CTRL_PRESSED } else { 0 };
  mods |= if modifier_set (VK_RCONTROL) != 0 { RIGHT_CTRL_PRESSED } else { 0 };
  mods |= if modifier_set (VK_LMENU) != 0 { LEFT_ALT_PRESSED } else { 0 };
  mods |= if modifier_set (VK_RMENU) != 0 { RIGHT_ALT_PRESSED } else { 0 };
  mods |= if modifier_set (VK_LWIN) != 0 { LEFT_WIN_PRESSED } else { 0 };
  mods |= if modifier_set (VK_RWIN) != 0 { RIGHT_WIN_PRESSED } else { 0 };
  mods |= if modifier_set (VK_APPS) != 0 { APPS_PRESSED } else { 0 };

  return mods;
}

/// C 3527-3536: `static int w32_get_key_modifiers (unsigned int wparam, unsigned int lparam)` -
/// `w32term.h` 262's declaration takes C's `DWORD`/`WORD`, so C's two `unsigned int` arguments
/// are narrowed at the call (the port writes C's implicit conversion out).
#[cfg (feature = "have_ntgui")]
unsafe fn w32_get_key_modifiers (wparam: c_uint, lparam: c_uint) -> c_int
{
  let mut mods: c_int;

  /* Convert to emacs modifiers.  */
  mods = w32_kbd_mods_to_emacs (w32_construct_console_modifiers () as DWORD, wparam as WORD);

  return mods;
}

/* C 3538-3540: C's `unsigned int map_keypad_keys (unsigned int, unsigned int);` forward
   declaration has no Rust counterpart (PORTING.md 0.1-12); the definition stands at C's
   position below.  */
/// C 3542-3566: `unsigned int map_keypad_keys (unsigned int virt_key, unsigned int extended)` -
/// a non-`static` definition, so it carries `#[unsafe(no_mangle)]`; the five `VK_NUMPAD_*`
/// constants come from the mirror and the SDK's `VK_*` from the section above, so C's `unsigned
/// int` comparisons write the conversions out.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_keypad_keys (virt_key: c_uint, extended: c_uint) -> c_uint
{
  if virt_key < VK_CLEAR as c_uint || virt_key > VK_DELETE as c_uint
    { return virt_key; }

  if virt_key == VK_RETURN as c_uint
    { return (if extended != 0 { VK_NUMPAD_ENTER } else { VK_RETURN }) as c_uint; }

  if virt_key >= VK_PRIOR as c_uint && virt_key <= VK_DOWN as c_uint
    { return (if extended == 0 { VK_NUMPAD_PRIOR + (virt_key as c_int - VK_PRIOR) }
	      else { virt_key as c_int }) as c_uint; }

  if virt_key == VK_INSERT as c_uint || virt_key == VK_DELETE as c_uint
    { return (if extended == 0 { VK_NUMPAD_INSERT + (virt_key as c_int - VK_INSERT) }
	      else { virt_key as c_int }) as c_uint; }

  if virt_key == VK_CLEAR as c_uint
    { return (if extended == 0 { VK_NUMPAD_CLEAR } else { virt_key as c_int }) as c_uint; }

  return virt_key;
}

/* List of special key combinations which w32 would normally capture,
   but Emacs should grab instead.  Not directly visible to lisp, to
   simplify synchronization.  Each item is an integer encoding a virtual
   key code and modifier combination to capture.
   Note: This code is not used if keyboard hooks are active
   (Windows 2000 and later).  */
/// C 3567: `static Lisp_Object w32_grabbed_keys;` - C's tentative definition with static storage
/// duration, whose zero initialiser is `Qnil` (the `alloc.rs` 8006 `lisp_h_Qnil ()` reading).
/// C: static Lisp_Object w32_grabbed_keys;
#[cfg (feature = "have_ntgui")]
static mut w32_grabbed_keys: Lisp_Object = lisp_h_Qnil ();

/// C 3569-3574/3577-3590: the seven `#define`s of the hot-key encoding and `register_hot_keys`.
/// Each macro's argument is evaluated once per use C makes of it (PORTING.md 2.1).
/// C: #define HOTKEY(vk, mods)      make_fixnum (((vk) & 255) | ((mods) << 8))
/// C: #define HOTKEY_ID(k)          (XFIXNAT (k) & 0xbfff)
/// C: #define HOTKEY_VK_CODE(k)     (XFIXNAT (k) & 255)
/// C: #define HOTKEY_MODIFIERS(k)   (XFIXNAT (k) >> 8)
/// C: #define RAW_HOTKEY_ID(k)        ((k) & 0xbfff)
/// C: #define RAW_HOTKEY_VK_CODE(k)   ((k) & 255)
/// C: #define RAW_HOTKEY_MODIFIERS(k) ((k) >> 8)
#[cfg (feature = "have_ntgui")]
macro_rules! HOTKEY {
  ($vk:expr, $mods:expr) => { make_fixnum ((($vk) & 255) | (($mods) << 8)) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! HOTKEY_ID {
  ($k:expr) => { (XFIXNAT ($k) & 0xbfff) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! HOTKEY_VK_CODE {
  ($k:expr) => { (XFIXNAT ($k) & 255) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! HOTKEY_MODIFIERS {
  ($k:expr) => { (XFIXNAT ($k) >> 8) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! RAW_HOTKEY_ID {
  ($k:expr) => { (($k) & 0xbfff) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! RAW_HOTKEY_VK_CODE {
  ($k:expr) => { (($k) & 255) };
}
#[cfg (feature = "have_ntgui")]
macro_rules! RAW_HOTKEY_MODIFIERS {
  ($k:expr) => { (($k) >> 8) };
}

/* Register hot-keys for reserved key combinations when Emacs has
   keyboard focus, since this is the only way Emacs can receive key
   combinations like Alt-Tab which are used by the system.  */

/// C 3581-3598: `static void register_hot_keys (HWND hwnd)`.  C's `Lisp_Object keylist;` and
/// `Lisp_Object key = XCAR (keylist);` keep their scope, and the three macro arguments are the
/// `RAW_HOTKEY_*`/`HOTKEY_*` families above.
#[cfg (feature = "have_ntgui")]
unsafe fn register_hot_keys (hwnd: HWND)
{
  let mut keylist: Lisp_Object;

  /* Use CONSP, since we are called asynchronously.  */
  keylist = w32_grabbed_keys;
  while CONSP (keylist)
    {
      let key: Lisp_Object = XCAR (keylist);

      /* Deleted entries get set to nil.  */
      if ! FIXNUMP (key)
	{
	  keylist = XCDR (keylist);
	  continue;
	}

      RegisterHotKey (hwnd, HOTKEY_ID! (key) as c_int, HOTKEY_MODIFIERS! (key) as UINT,
		      HOTKEY_VK_CODE! (key) as UINT);

      keylist = XCDR (keylist);
    }
}

/// C 3600-3615: `static void unregister_hot_keys (HWND hwnd)` - the same walk, with the one
/// `UnregisterHotKey` call.
#[cfg (feature = "have_ntgui")]
unsafe fn unregister_hot_keys (hwnd: HWND)
{
  let mut keylist: Lisp_Object;

  keylist = w32_grabbed_keys;
  while CONSP (keylist)
    {
      let key: Lisp_Object = XCAR (keylist);

      if ! FIXNUMP (key)
	{
	  keylist = XCDR (keylist);
	  continue;
	}

      UnregisterHotKey (hwnd, HOTKEY_ID! (key) as c_int);

      keylist = XCDR (keylist);
    }
}

/* C 3617-3673: `w32_name_of_message`, the `EMACSDEBUG` arm.  C 3617's `#if EMACSDEBUG` is the
   feature `emacsdebug` (P-86 ①: C's macro lowercased), inside the file gate.  */
/// C 3630-3632: C's function-local `static char buf[64]`, which the one `sprintf` writes and the
/// return hands out; Rust has no function-scope `static`, so it stands at file scope (the
/// `frame.rs` 4813 and `textconv.rs` 2026 readings, PORTING.md 1.5).
/// C: static char buf[64];
#[cfg (all (feature = "have_ntgui", feature = "emacsdebug"))]
static mut buf: [c_char; 64] = [0; 64];

/// C 3621-3671: `const char* w32_name_of_message (UINT msg)` - a non-`static` definition, so it
/// carries `#[unsafe(no_mangle)]`.  C's local `struct { UINT msg; const char* name; } msgnames[]`
/// is an anonymous record the port names after the object (P-67), its `#define M(msg) { msg, #
/// msg }` is the port's `macro_rules!` over `stringify!`, and C's `static const` array is a local
/// immutable array (the `kqueue.rs` 464 `nullts` reading - no address of it escapes).
#[cfg (all (feature = "have_ntgui", feature = "emacsdebug"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_name_of_message (msg: UINT) -> *const c_char
{
  let mut i: c_uint;

  /// C: static const struct { UINT msg; const char* name; } msgnames[] = { … };
  #[repr(C)]
  struct msgnames_s
  {
    msg: UINT,
    name: *const c_char,
  }

  /// C: #define M(msg) { msg, # msg }
  macro_rules! M {
    ($msg:ident) => {
      msgnames_s { msg: $msg as UINT,
		   name: concat! (stringify! ($msg), "\0").as_ptr () as *const c_char } };
  }

  let msgnames: [msgnames_s; 35] = [
    M! (WM_PAINT),
    M! (WM_TIMER),
    M! (WM_USER),
    M! (WM_MOUSEMOVE),
    M! (WM_LBUTTONUP),
    M! (WM_KEYDOWN),
    M! (WM_EMACS_KILL),
    M! (WM_EMACS_CREATEWINDOW),
    M! (WM_EMACS_DONE),
    M! (WM_EMACS_CREATEVSCROLLBAR),
    M! (WM_EMACS_CREATEHSCROLLBAR),
    M! (WM_EMACS_SHOWWINDOW),
    M! (WM_EMACS_SETWINDOWPOS),
    M! (WM_EMACS_DESTROYWINDOW),
    M! (WM_EMACS_TRACKPOPUPMENU),
    M! (WM_EMACS_SETFOCUS),
    M! (WM_EMACS_SETFOREGROUND),
    M! (WM_EMACS_SETLOCALE),
    M! (WM_EMACS_SETKEYBOARDLAYOUT),
    M! (WM_EMACS_REGISTER_HOT_KEY),
    M! (WM_EMACS_UNREGISTER_HOT_KEY),
    M! (WM_EMACS_TOGGLE_LOCK_KEY),
    M! (WM_EMACS_TRACK_CARET),
    M! (WM_EMACS_DESTROY_CARET),
    M! (WM_EMACS_SHOW_CARET),
    M! (WM_EMACS_HIDE_CARET),
    M! (WM_EMACS_SETCURSOR),
    M! (WM_EMACS_SHOWCURSOR),
    M! (WM_EMACS_PAINT),
    M! (WM_EMACS_IME_STATUS),
    M! (WM_CHAR),
    M! (WM_EMACS_DRAGOVER),
    M! (WM_EMACS_DROP),
    M! (WM_EMACS_SET_TOOLKIT_THEME),
    /* C's `#undef M` has no counterpart: the macro's scope ends with this block.  */
    msgnames_s { msg: 0, name: core::ptr::null () },
  ];

  i = 0;
  while ! msgnames[i as usize].name.is_null ()
    {
      if msgnames[i as usize].msg == msg
	{ return msgnames[i as usize].name; }
      i += 1;
    }

  sprintf (buf.as_mut_ptr (), c"message 0x%04x".as_ptr (), msg as c_uint);
  return buf.as_ptr ();
}

/* C 3675-3704: the overview comment ("Here's an overview of how Emacs input works in GUI
   sessions on MS-Windows.  …").  It names `w32_console_read_socket` (C 3677, in
   `src/w32inevt.c`), `w32_msg_pump` (C 3706), `w32_wnd_proc` (C 4528, this file's, not yet
   ported: P-91 ④'s class), `w32_read_socket` and `kbd_buffer_get_event`.  */

/* Here's an overview of how Emacs input works in GUI sessions on
   MS-Windows.  (For description of non-GUI input, see the commentary
   before w32_console_read_socket in w32inevt.c.)

   System messages are read and processed by w32_msg_pump below.  This
   function runs in a separate thread.  It handles a small number of
   custom WM_EMACS_* messages (posted by the main thread, look for
   PostMessage calls), and dispatches the rest to w32_wnd_proc, which
   is the main window procedure for the entire Emacs application.

   w32_wnd_proc also runs in the same separate input thread.  It
   handles some messages, mostly those that need GDI calls, by itself.
   For the others, it calls my_post_msg, which inserts the messages
   into the input queue serviced by w32_read_socket.

   w32_read_socket runs in the main (a.k.a. "Lisp") thread, and is
   called synchronously from keyboard.c when it is known or suspected
   that some input is available.  w32_read_socket either handles
   messages immediately, or converts them into Emacs input events and
   stuffs them into kbd_buffer, where kbd_buffer_get_event can get at
   them and process them when read_char and its callers require
   input.

   Under Cygwin with the W32 toolkit, the use of /dev/windows with
   select(2) takes the place of w32_read_socket.

   */

/// C 3706-3838: `w32_msg_pump`'s message ids (the SDK's `<winuser.h>` half) and the entry points
/// the three files of the family call.  C's `switch` compares them through C's `UINT msg.message`,
/// and the port's message ids are typed `c_int` (the mirror's `WM_EMACS_*` are `c_int`, so the
/// match's scrutinee writes C's conversion out); the ledger's Deviations record it.
/// C: #define WM_NULL 0x0000 / #define WM_PAINT 0x000F / #define WM_KEYDOWN 0x0100 /
/// C: #define WM_KEYUP 0x0101 / #define WM_CHAR 0x0102 / #define WM_SYSKEYDOWN 0x0104 /
/// C: #define WM_SYSKEYUP 0x0105 / #define WM_TIMER 0x0113 / #define WM_MOUSEMOVE 0x0200 /
/// C: #define WM_LBUTTONUP 0x0202 / #define WM_INITMENU 0x0116
/// C: BOOL WINAPI GetMessageA (LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax);
/// C: BOOL WINAPI GetMessageW (LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax);
/// C: LRESULT WINAPI DispatchMessageA (const MSG *lpMsg);
/// C: LRESULT WINAPI DispatchMessageW (const MSG *lpMsg);
/// C: LONG WINAPI GetMessageTime (void);
/// C: UINT WINAPI RegisterWindowMessageA (LPCSTR lpString);
/// C: BOOL WINAPI RegisterHotKey (HWND hWnd, int id, UINT fsModifiers, UINT vk);
/// C: BOOL WINAPI UnregisterHotKey (HWND hWnd, int id);
/// C: LCID WINAPI SetThreadLocale (LCID Locale);
/// C: HKL WINAPI ActivateKeyboardLayout (HKL hkl, UINT Flags);
#[cfg (feature = "have_ntgui")]
pub const WM_NULL: c_int = 0x0000;
#[cfg (feature = "have_ntgui")]
pub const WM_PAINT: c_int = 0x000F;
#[cfg (feature = "have_ntgui")]
pub const WM_KEYDOWN: c_int = 0x0100;
#[cfg (feature = "have_ntgui")]
pub const WM_KEYUP: c_int = 0x0101;
#[cfg (feature = "have_ntgui")]
pub const WM_CHAR: c_int = 0x0102;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSKEYDOWN: c_int = 0x0104;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSKEYUP: c_int = 0x0105;
#[cfg (feature = "have_ntgui")]
pub const WM_TIMER: c_int = 0x0113;
#[cfg (feature = "have_ntgui")]
pub const WM_INITMENU: c_int = 0x0116;
#[cfg (feature = "have_ntgui")]
pub const WM_MOUSEMOVE: c_int = 0x0200;
#[cfg (feature = "have_ntgui")]
pub const WM_LBUTTONUP: c_int = 0x0202;
#[cfg (feature = "have_ntgui")]
pub type LCID = DWORD;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetMessageA (lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT,
                      wMsgFilterMax: UINT) -> BOOL;
  pub fn GetMessageW (lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT,
                      wMsgFilterMax: UINT) -> BOOL;
  pub fn DispatchMessageA (lpMsg: *const MSG) -> LRESULT;
  pub fn DispatchMessageW (lpMsg: *const MSG) -> LRESULT;
  pub fn GetMessageTime () -> LONG;
  pub fn RegisterWindowMessage (lpString: *const c_char) -> UINT;
  pub fn RegisterHotKey (hWnd: HWND, id: c_int, fsModifiers: UINT, vk: UINT) -> BOOL;
  pub fn UnregisterHotKey (hWnd: HWND, id: c_int) -> BOOL;
  pub fn SetThreadLocale (Locale: LCID) -> LCID;
  pub fn ActivateKeyboardLayout (hkl: HKL, Flags: UINT) -> HKL;
}

/* Main message dispatch loop.  */

/// C 3706-3838: `static void w32_msg_pump (deferred_msg * msg_buf)`.  C's
/// `while ((w32_unicode_gui ? GetMessageW : GetMessageA) (&msg, NULL, 0, 0))` chooses the entry
/// point with C's conditional operator, which the port binds to a local function pointer and
/// tests per iteration; C 3827's `dispatch:` label is reached by the `else` branch and by the two
/// `goto dispatch` sites (C 3812 and C 3822), which the port writes as the `dispatch` flag the
/// ledger's Deviations and the `macro-body-notes.tsv` row record (PORTING.md 2.2's structured
/// spelling for a `goto` into the other arm).  `msg`, `result` and `focus_window` are C's
/// uninitialised locals (PORTING.md 1.5).
#[cfg (feature = "have_ntgui")]
unsafe fn w32_msg_pump (msg_buf: *mut deferred_msg)
{
  let mut msg: MSG = core::mem::zeroed ();
  let mut result: WPARAM = 0;
  let mut focus_window: HWND = core::ptr::null_mut ();

  msh_mousewheel = RegisterWindowMessage (MSH_MOUSEWHEEL.as_ptr ());

  let get_message: unsafe extern "system" fn (lpMsg: *mut MSG, hWnd: HWND,
					      wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL
    = if w32_unicode_gui != 0 { GetMessageW } else { GetMessageA };

  while get_message (&raw mut msg, core::ptr::null_mut (), 0, 0) != 0
    {

      /* DebPrint (("w32_msg_pump: %s time:%u\n", */
      /*            w32_name_of_message (msg.message), msg.time)); */

      let mut dispatch: bool = false;

      if msg.hwnd.is_null ()
	{
	  /* C's `switch` arms that `break` fall out of the switch and reach the end of the `if`
	     branch without dispatching; the label carries that exit (PORTING.md 2.2).  */
	  'message_switch:
	  {
	  match msg.message as c_int
	    {
	    WM_NULL =>
	      {
		/* Produced by complete_deferred_msg; just ignore.  */
	      }
	    WM_EMACS_CREATEWINDOW =>
	      {
		/* Initialize COM for this window.  Needed for RegisterDragDrop.
		   Some third party shell extensions can cause it to be used in
		   system dialogs, which causes a crash if it is not initialized.
		   This is a known bug in Windows, which was fixed long ago, but
		   the patch for XP is not publicly available until XP SP3,
		   and older versions will never be patched.  */
		OleInitialize (core::ptr::null_mut ());

		w32_createwindow (msg.wParam as *mut frame, msg.lParam as *mut c_int);
		if PostThreadMessage (dwMainThreadId, WM_EMACS_DONE as UINT, 0, 0) == 0
		  { emacs_abort (); }
	      }
	    WM_EMACS_SETLOCALE =>
	      {
		SetThreadLocale (msg.wParam as LCID);
		/* Reply is not expected.  */
	      }
	    WM_EMACS_SETKEYBOARDLAYOUT =>
	      {
		result = ActivateKeyboardLayout (msg.wParam as HKL, 0) as WPARAM;
		if PostThreadMessage (dwMainThreadId, WM_EMACS_DONE as UINT, result, 0) == 0
		  { emacs_abort (); }
	      }
	    WM_EMACS_REGISTER_HOT_KEY =>
	      {
		focus_window = GetFocus ();
		if ! focus_window.is_null ()
		  { RegisterHotKey (focus_window, RAW_HOTKEY_ID! (msg.wParam) as c_int,
				    RAW_HOTKEY_MODIFIERS! (msg.wParam) as UINT,
				    RAW_HOTKEY_VK_CODE! (msg.wParam) as UINT); }
		/* Reply is not expected.  */
	      }
	    WM_EMACS_UNREGISTER_HOT_KEY =>
	      {
		focus_window = GetFocus ();
		if ! focus_window.is_null ()
		  { UnregisterHotKey (focus_window, RAW_HOTKEY_ID! (msg.wParam) as c_int); }
		/* Mark item as erased.  NB: this code must be
		   thread-safe.  The next line is okay because the cons
		   cell is never made into garbage and is not relocated by
		   GC.  */
		XSETCAR (make_lisp_ptr (msg.lParam as *mut c_void, Lisp_Type::Lisp_Cons),
			 Qnil ());
		if PostThreadMessage (dwMainThreadId, WM_EMACS_DONE as UINT, 0, 0) == 0
		  { emacs_abort (); }
	      }
	    WM_EMACS_TOGGLE_LOCK_KEY =>
	      {
		let vk_code: c_int = msg.wParam as c_int;
		let mut cur_state: c_int = GetKeyState (vk_code) as c_int & 1;
		let new_state: c_int = msg.lParam as c_int;

		if new_state == -1
		    || ((new_state & 1) != cur_state)
		  {
		    one_w32_display_info.faked_key = vk_code;

		    keybd_event (vk_code as BYTE,
				 MapVirtualKey (vk_code as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
		    keybd_event (vk_code as BYTE,
				 MapVirtualKey (vk_code as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | 0, 0);
		    keybd_event (vk_code as BYTE,
				 MapVirtualKey (vk_code as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
		    cur_state = if cur_state == 0 { 1 } else { 0 };
		  }
		if PostThreadMessage (dwMainThreadId, WM_EMACS_DONE as UINT,
				      cur_state as WPARAM, 0) == 0
		  { emacs_abort (); }
	      }
	    WM_EMACS_IME_STATUS =>
	      {
		focus_window = GetFocus ();
		if set_ime_open_status_fn.is_none () || focus_window.is_null ()
		  /* C 3802-3803: `break` out of the switch - no dispatch.  */
		  { break 'message_switch; }

		let context: HIMC = (get_ime_context_fn.unwrap ()) (focus_window);
		if context.is_null ()
		  /* C 3806-3807: the same `break`.  */
		  { break 'message_switch; }

		(set_ime_open_status_fn.unwrap ()) (context, (msg.wParam != 0) as BOOL);
		(release_ime_context_fn.unwrap ()) (focus_window, context);

		/* C 3812: `goto dispatch`.  */
		dispatch = true;
	      }

	    _ =>
	      {
		#[cfg (feature = "msg_debug")]
		{
		  /* Broadcast messages make it here, so you need to be looking
		     for something in particular for this to be useful.  */
		  DebPrint! ((c"msg %x not expected by w32_msg_pump\n".as_ptr (), msg.message));
		}
		/* Handle extra events for compatibility, preventing not dispatch.  */
		/* C 3822: `goto dispatch`.  */
		dispatch = true;
	      }
	    }
	  }
	}
      else
	{
	  /* C 3827's `dispatch:` label; the `else` branch falls through to it.  */
	  dispatch = true;
	}

      if dispatch
	{
	  if w32_unicode_gui != 0
	    { DispatchMessageW (&raw const msg); }
	  else
	    { DispatchMessageA (&raw const msg); }
	}

      /* Exit nested loop when our deferred message has completed.  */
      if (*msg_buf).completed != 0
	{ break; }
    }
}

/// C 3840: `static deferred_msg * deferred_msg_head;` - C's tentative definition (a null pointer
/// until the first `send_deferred_msg`), which the extract-abi checklist asked for.
/// C: static deferred_msg * deferred_msg_head;
#[cfg (feature = "have_ntgui")]
static mut deferred_msg_head: *mut deferred_msg = core::ptr::null_mut ();

/// C 3842-3859: `static deferred_msg * find_deferred_msg (HWND hwnd, UINT msg)`.  C's
/// `for (item = …; item != NULL; item = item->next) … break;` is the port's `while` with C's
/// `break` (the step runs only when the loop body did not break), and the two `enter_crit ()`/
/// `leave_crit ()` calls are commented out in C's own text, which the port keeps as comments.
#[cfg (feature = "have_ntgui")]
unsafe fn find_deferred_msg (hwnd: HWND, msg: UINT) -> *mut deferred_msg
{
  let mut item: *mut deferred_msg;

  /* Don't actually need synchronization for read access, since
     modification of single pointer is always atomic.  */
  /* enter_crit (); */

  item = deferred_msg_head;
  while ! item.is_null ()
    {
      if (*item).w32msg.msg.hwnd == hwnd
	  && (*item).w32msg.msg.message == msg
	{ break; }
      item = (*item).next;
    }

  /* leave_crit (); */

  return item;
}

/// C 3861-3897: `static LRESULT send_deferred_msg (deferred_msg * msg_buf, HWND hwnd, UINT msg,
/// WPARAM wParam, LPARAM lParam)` - the nested `w32_msg_pump` call is the recursion into the
/// family's dispatcher.
#[cfg (feature = "have_ntgui")]
unsafe fn send_deferred_msg (msg_buf: *mut deferred_msg, hwnd: HWND, msg: UINT, wParam: WPARAM,
			     lParam: LPARAM) -> LRESULT
{
  /* Only input thread can send deferred messages.  */
  if GetCurrentThreadId () != dwWindowsThreadId
    { emacs_abort (); }

  /* It is an error to send a message that is already deferred.  */
  if ! find_deferred_msg (hwnd, msg).is_null ()
    { emacs_abort (); }

  /* Enforced synchronization is not needed because this is the only
     function that alters deferred_msg_head, and the following critical
     section is guaranteed to only be serially reentered (since only the
     input thread can call us).  */

  /* enter_crit (); */

  (*msg_buf).completed = 0;
  (*msg_buf).next = deferred_msg_head;
  deferred_msg_head = msg_buf;
  my_post_msg (&raw mut (*msg_buf).w32msg, hwnd, msg, wParam, lParam);

  /* leave_crit (); */

  /* Start a new nested message loop to process other messages until
     this one is completed.  */
  w32_msg_pump (msg_buf);

  deferred_msg_head = (*msg_buf).next;

  return (*msg_buf).result;
}

/// C 3899-3913: `void complete_deferred_msg (HWND hwnd, UINT msg, LRESULT result)` - a non-
/// `static` definition, so it carries `#[unsafe(no_mangle)]`.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn complete_deferred_msg (hwnd: HWND, msg: UINT, result: LRESULT)
{
  let msg_buf: *mut deferred_msg = find_deferred_msg (hwnd, msg);

  if msg_buf.is_null ()
    /* Message may have been canceled, so don't abort.  */
    { return; }

  (*msg_buf).result = result;
  (*msg_buf).completed = 1;

  /* Ensure input thread is woken so it notices the completion.  */
  PostThreadMessage (dwWindowsThreadId, WM_NULL as UINT, 0, 0);
}

/// C 3915-3933: `static void cancel_all_deferred_msgs (void)`.
#[cfg (feature = "have_ntgui")]
unsafe fn cancel_all_deferred_msgs ()
{
  let mut item: *mut deferred_msg;

  /* Don't actually need synchronization for read access, since
     modification of single pointer is always atomic.  */
  /* enter_crit (); */

  item = deferred_msg_head;
  while ! item.is_null ()
    {
      (*item).result = 0;
      (*item).completed = 1;
      item = (*item).next;
    }

  /* leave_crit (); */

  /* Ensure input thread is woken so it notices the completion.  */
  PostThreadMessage (dwWindowsThreadId, WM_NULL as UINT, 0, 0);
}

/// C 3942: `PeekMessage (&msg, NULL, 0, 0, PM_NOREMOVE)` - `<winuser.h>`'s non-blocking peek and
/// the flag C passes it.
/// C: BOOL WINAPI PeekMessageA (LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax,
/// C: UINT wRemoveMsg);
/// C: #define PM_NOREMOVE 0x0000 / #define PM_REMOVE 0x0001
#[cfg (feature = "have_ntgui")]
pub const PM_NOREMOVE: UINT = 0x0000;
#[cfg (feature = "have_ntgui")]
pub const PM_REMOVE: UINT = 0x0001;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn PeekMessage (lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
		      wRemoveMsg: UINT) -> BOOL;
}

/* C 3936: `DWORD WINAPI w32_msg_worker (void *);` - C's forward declaration, which has no Rust
   counterpart (PORTING.md 0.1-12); the definition stands at C 3938.  */

/// C 3938-3960: `DWORD WINAPI w32_msg_worker (void *arg)` - `WINAPI` is `__stdcall` and the
/// definition is non-`static`, so it is an `unsafe extern "system"` item with
/// `#[unsafe(no_mangle)]`.  C's `MSG msg;` and `deferred_msg dummy_buf;` are the two locals the
/// `PeekMessage` call and the `memset` fill (PORTING.md 1.5: `msg` keeps a zero initialiser, and
/// `dummy_buf`'s is the one C's `memset` writes anyway).
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn w32_msg_worker (arg: *mut c_void) -> DWORD
{
  let mut msg: MSG = core::mem::zeroed ();
  let mut dummy_buf: deferred_msg = core::mem::zeroed ();

  /* Ensure our message queue is created */

  PeekMessage (&raw mut msg, core::ptr::null_mut (), 0, 0, PM_NOREMOVE);

  if PostThreadMessage (dwMainThreadId, WM_EMACS_DONE as UINT, 0, 0) == 0
    { emacs_abort (); }

  memset (&raw mut dummy_buf as *mut c_void, 0, core::mem::size_of_val (&dummy_buf));
  dummy_buf.w32msg.msg.hwnd = core::ptr::null_mut ();
  dummy_buf.w32msg.msg.message = WM_NULL as UINT;

  /* This is the initial message loop which should only exit when the
     application quits.  */
  w32_msg_pump (&raw mut dummy_buf);

  return 0;
}

/// C 3962-3973: `static void signal_user_input (void)`.
#[cfg (feature = "have_ntgui")]
unsafe fn signal_user_input ()
{
  /* Interrupt any lisp that wants to be interrupted by input.  */
  if ! NILP (Vthrow_on_input)
    {
      Vquit_flag = Vthrow_on_input;
      /* Calling maybe_quit from this thread is a bad idea, since this
	 unwinds the stack of the Lisp thread, and the Windows runtime
	 rightfully barfs.  */
    }
}

/// C 4001: C's function-local `static int force_quit_count;` (the triple-C-g feature's counter) -
/// one object with static storage duration, which Rust spells at file scope (the `frame.rs` 4813
/// reading, PORTING.md 1.5).
/// C: static int force_quit_count;
#[cfg (feature = "have_ntgui")]
static mut force_quit_count: c_int = 0;

/// C 3976-4041: `static void post_character_message (HWND hwnd, UINT msg, WPARAM wParam, LPARAM
/// lParam, DWORD modifiers)`.  C's `int c = wParam;` narrows the parameter, `make_ctrl_char` is
/// `keyboard.c`'s (no mirror carries it: the declaration below), and C's `msg = WM_NULL;` assigns
/// the message id back through C's `UINT` parameter, which the port writes out.  C's parameter
/// name `modifiers` shadows the file-scope `modifiers` array of the modifier family, which Rust
/// forbids (E0530: a function parameter cannot shadow a static), so the port spells it with a
/// trailing underscore (the `alloc.rs` 4334 and `w32.rs` 3199 readings); the token check folds
/// the underscore.
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn make_ctrl_char (c: c_int) -> c_int;
}
#[cfg (feature = "have_ntgui")]
unsafe fn post_character_message (hwnd: HWND, mut msg: UINT, wParam: WPARAM, lParam: LPARAM,
				  modifiers_: DWORD)
{
  let mut wmsg: W32Msg = core::mem::zeroed ();

  wmsg.dwModifiers = modifiers_;

  /* Detect quit_char and set quit-flag directly.  Note that we
     still need to post a message to ensure the main thread will be
     woken up if blocked in sys_select, but we do NOT want to post
     the quit_char message itself (because it will usually be as if
     the user had typed quit_char twice).  Instead, we post a dummy
     message that has no particular effect.  */
  {
    let mut c: c_int = wParam as c_int;
    if c_isalpha (c) && wmsg.dwModifiers == ctrl_modifier as DWORD
      { c = make_ctrl_char (c) & 0o377; }
    if c == quit_char
	|| (wmsg.dwModifiers == 0
	    && w32_quit_key != 0 && wParam == w32_quit_key as WPARAM)
      {
	/* See keyboard.c:handle_interrupt.  We repeat the same code
           here to support the triple-C-g feature.  */
	let count: c_int = if NILP (Vquit_flag) { 1 } else { force_quit_count + 1 };
	force_quit_count = count;
	if count == 3
	  { Vinhibit_quit = Qnil (); }
	Vquit_flag = Qt ();

	/* The choice of message is somewhat arbitrary, as long as
	   the main thread handler just ignores it.  */
	msg = WM_NULL as UINT;

	/* Interrupt any blocking system calls.  */
	signal_quit ();

	/* As a safety precaution, forcibly complete any deferred
	   messages.  This is a kludge, but I don't see any particularly
	   clean way to handle the situation where a deferred message is
	   "dropped" in the lisp thread, and will thus never be
	   completed, eg. by the user trying to activate the menubar
	   when the lisp thread is busy, and then typing C-g when the
	   menubar doesn't open promptly (with the result that the
	   menubar never responds at all because the deferred
	   WM_INITMENU message is never completed).  Another problem
	   situation is when the lisp thread calls SendMessage (to send
	   a window manager command) when a message has been deferred;
	   the lisp thread gets blocked indefinitely waiting for the
	   deferred message to be completed, which itself is waiting for
	   the lisp thread to respond.

	   Note that we don't want to block the input thread waiting for
	   a response from the lisp thread (although that would at least
	   solve the deadlock problem above), because we want to be able
	   to receive C-g to interrupt the lisp thread.  */
	cancel_all_deferred_msgs ();
      }
    else
      { signal_user_input (); }
  }

  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
}

/* ============================================================================
   C 4043-5986: this unit's fourth slice (`docs/ports/w32fns.md`): `get_wm_chars` (C 4043),
   C 4135's `#ifdef DBG_WM_CHARS`, C 4141-4152's `after_deadkey`, `deliver_wm_chars` (C 4154),
   `maybe_pass_notification` (C 4490, C's `#ifdef WINDOWSNT` arm) and `w32_wnd_proc` (C 4528) -
   the window procedure `INIT_WINDOW_CLASS!`'s `(WC).lpfnWndProc = Some (w32_wnd_proc)` (C 2404)
   and `w32_msg_pump`'s `DispatchMessageW/A` path both name, i.e. the `static` P-91 ④ recorded as
   "same file, C line known, not ported yet" until this slice.  The C line-range table, the
   acceptance readings and the deviations are in that ledger.
   ============================================================================ */

/* The `<windows.h>`/`<winuser.h>`/`<wingdi.h>` entries this slice names.  C's own `#include`
   block for them is C 60-71 (slice 1's text); these are the ones *this* slice's text is the first
   in the unit to need, so they stand here the way slice 2's and slice 3's SDK sections do
   (P-88 ①: a header with no file in the reference tree is declared where its members are used),
   each with C's own prototype in a `/// C:` line.  The records are spelled from the SDK's
   published member lists - this machine has no Windows SDK and no mingw toolchain, so **no layout
   assertion exists for any of them** (P-86 ③⑵'s `text-only`, owed in the ledger).  */
/// C: typedef struct tagPAINTSTRUCT { HDC hdc; BOOL fErase; RECT rcPaint; BOOL fRestore;
/// C: BOOL fIncUpdate; BYTE rgbReserved[32]; } PAINTSTRUCT;
/// C: typedef struct tagLOGFONTA { LONG lfHeight; LONG lfWidth; LONG lfEscapement;
/// C: LONG lfOrientation; LONG lfWeight; BYTE lfItalic; BYTE lfUnderline; BYTE lfStrikeOut;
/// C: BYTE lfCharSet; BYTE lfOutPrecision; BYTE lfClipPrecision; BYTE lfQuality;
/// C: BYTE lfPitchAndFamily; CHAR lfFaceName[LF_FACESIZE]; } LOGFONTA;
/// C: typedef struct tagLOGFONTW { LONG lfHeight; LONG lfWidth; LONG lfEscapement;
/// C: LONG lfOrientation; LONG lfWeight; BYTE lfItalic; BYTE lfUnderline; BYTE lfStrikeOut;
/// C: BYTE lfCharSet; BYTE lfOutPrecision; BYTE lfClipPrecision; BYTE lfQuality;
/// C: BYTE lfPitchAndFamily; WCHAR lfFaceName[LF_FACESIZE]; } LOGFONTW;
/// C: typedef struct tagMEASUREITEMSTRUCT { UINT CtlType; UINT CtlID; UINT itemID; UINT itemWidth;
/// C: UINT itemHeight; ULONG_PTR itemData; } MEASUREITEMSTRUCT, *LPMEASUREITEMSTRUCT;
/// C: typedef struct tagDRAWITEMSTRUCT { UINT CtlType; UINT CtlID; UINT itemID; UINT itemAction;
/// C: UINT itemState; HWND hwndItem; HDC hDC; RECT rcItem; ULONG_PTR itemData; } DRAWITEMSTRUCT;
/// C: typedef struct tagSIZE { LONG cx; LONG cy; } SIZE, *LPSIZE;
/// C: typedef struct tagWINDOWPOS { HWND hwnd; HWND hwndInsertAfter; int x; int y; int cx;
/// C: int cy; UINT flags; } WINDOWPOS, *LPWINDOWPOS;
/// C: typedef struct tagMINMAXINFO { POINT ptReserved; POINT ptMaxSize; POINT ptMaxPosition;
/// C: POINT ptMinTrackSize; POINT ptMaxTrackSize; } MINMAXINFO, *PMINMAXINFO, *LPMINMAXINFO;
/// C: typedef HANDLE HFONT;
/// C: typedef struct _RTL_CRITICAL_SECTION { PRTL_CRITICAL_SECTION_DEBUG DebugInfo;
/// C: LONG LockCount; LONG RecursionCount; HANDLE OwningThread; HANDLE LockSemaphore;
/// C: ULONG_PTR SpinCount; } RTL_CRITICAL_SECTION, CRITICAL_SECTION;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION
{
  pub DebugInfo: *mut c_void,
  pub LockCount: LONG,
  pub RecursionCount: LONG,
  pub OwningThread: HANDLE,
  pub LockSemaphore: HANDLE,
  pub SpinCount: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
pub type HFONT = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;
#[cfg (feature = "have_ntgui")]
pub type LPSIZE = *mut SIZE;
#[cfg (feature = "have_ntgui")]
pub type LPMINMAXINFO = *mut MINMAXINFO;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PAINTSTRUCT
{
  pub hdc: HDC,
  pub fErase: BOOL,
  pub rcPaint: RECT,
  pub fRestore: BOOL,
  pub fIncUpdate: BOOL,
  pub rgbReserved: [BYTE; 32],
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGFONT
{
  pub lfHeight: LONG,
  pub lfWidth: LONG,
  pub lfEscapement: LONG,
  pub lfOrientation: LONG,
  pub lfWeight: LONG,
  pub lfItalic: BYTE,
  pub lfUnderline: BYTE,
  pub lfStrikeOut: BYTE,
  pub lfCharSet: BYTE,
  pub lfOutPrecision: BYTE,
  pub lfClipPrecision: BYTE,
  pub lfQuality: BYTE,
  pub lfPitchAndFamily: BYTE,
  pub lfFaceName: [CHAR; 32],
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGFONTW
{
  pub lfHeight: LONG,
  pub lfWidth: LONG,
  pub lfEscapement: LONG,
  pub lfOrientation: LONG,
  pub lfWeight: LONG,
  pub lfItalic: BYTE,
  pub lfUnderline: BYTE,
  pub lfStrikeOut: BYTE,
  pub lfCharSet: BYTE,
  pub lfOutPrecision: BYTE,
  pub lfClipPrecision: BYTE,
  pub lfQuality: BYTE,
  pub lfPitchAndFamily: BYTE,
  pub lfFaceName: [WCHAR; 32],
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEASUREITEMSTRUCT
{
  pub CtlType: UINT,
  pub CtlID: UINT,
  pub itemID: UINT,
  pub itemWidth: UINT,
  pub itemHeight: UINT,
  pub itemData: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRAWITEMSTRUCT
{
  pub CtlType: UINT,
  pub CtlID: UINT,
  pub itemID: UINT,
  pub itemAction: UINT,
  pub itemState: UINT,
  pub hwndItem: HWND,
  pub hDC: HDC,
  pub rcItem: RECT,
  pub itemData: ULONG_PTR,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIZE
{
  pub cx: LONG,
  pub cy: LONG,
}
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MINMAXINFO
{
  pub ptReserved: POINT,
  pub ptMaxSize: POINT,
  pub ptMaxPosition: POINT,
  pub ptMinTrackSize: POINT,
  pub ptMaxTrackSize: POINT,
}

/* The `<winuser.h>` message ids, the `<wincon.h>`/`<wingdi.h>` constants and the three `VK_*`
   this slice adds to slice 3's set.  C's own `#define` text is in the `/// C:` lines.  */
/// C: #define WM_KEYFIRST 0x0100 / #define WM_KEYLAST 0x0109 / #define WM_DEADCHAR 0x0103 /
/// C: #define WM_SYSCHAR 0x0106 / #define WM_SYSDEADCHAR 0x0107 / #define WM_HOTKEY 0x0312 /
/// C: #define WM_INPUTLANGCHANGE 0x0051 / #define WM_ERASEBKGND 0x0014 /
/// C: #define WM_PALETTECHANGED 0x0311 / #define WM_NCACTIVATE 0x0086 /
/// C: #define WM_MENUSELECT 0x011F / #define WM_MEASUREITEM 0x002C / #define WM_DRAWITEM 0x002B /
/// C: #define WM_MOUSEACTIVATE 0x0021 / #define WM_ACTIVATEAPP 0x001C / #define WM_ACTIVATE 0x0006 /
/// C: #define WM_WINDOWPOSCHANGED 0x0047 / #define WM_SHOWWINDOW 0x0018 /
/// C: #define WM_SETTINGCHANGE 0x001A / #define WM_SETFOCUS 0x0007 / #define WM_KILLFOCUS 0x0008 /
/// C: #define WM_COMMAND 0x0111 / #define WM_MOVE 0x0003 / #define WM_SIZE 0x0005 /
/// C: #define WM_CREATE 0x0001 / #define WM_DESTROY 0x0002 / #define WM_CLOSE 0x0010 /
/// C: #define WM_ENDSESSION 0x0016 / #define WM_WINDOWPOSCHANGING 0x0046 /
/// C: #define WM_GETMINMAXINFO 0x0024 / #define WM_SETCURSOR 0x0020 / #define WM_HSCROLL 0x0114 /
/// C: #define WM_VSCROLL 0x0115 / #define WM_LBUTTONDOWN 0x0201 / #define WM_RBUTTONDOWN 0x0204 /
/// C: #define WM_RBUTTONUP 0x0205 / #define WM_MBUTTONDOWN 0x0207 / #define WM_MBUTTONUP 0x0208 /
/// C: #define WM_MOUSEFIRST 0x0200 / #define WM_MOUSELAST 0x020E /
/// C: #define WM_IME_STARTCOMPOSITION 0x010D / #define WM_IME_ENDCOMPOSITION 0x010E
#[cfg (feature = "have_ntgui")]
pub const WM_KEYFIRST: c_int = 0x0100;
#[cfg (feature = "have_ntgui")]
pub const WM_KEYLAST: c_int = 0x0109;
#[cfg (feature = "have_ntgui")]
pub const WM_DEADCHAR: c_int = 0x0103;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSCHAR: c_int = 0x0106;
#[cfg (feature = "have_ntgui")]
pub const WM_SYSDEADCHAR: c_int = 0x0107;
#[cfg (feature = "have_ntgui")]
pub const WM_HOTKEY: c_int = 0x0312;
#[cfg (feature = "have_ntgui")]
pub const WM_INPUTLANGCHANGE: c_int = 0x0051;
#[cfg (feature = "have_ntgui")]
pub const WM_ERASEBKGND: c_int = 0x0014;
#[cfg (feature = "have_ntgui")]
pub const WM_PALETTECHANGED: c_int = 0x0311;
#[cfg (feature = "have_ntgui")]
pub const WM_NCACTIVATE: c_int = 0x0086;
#[cfg (feature = "have_ntgui")]
pub const WM_MENUSELECT: c_int = 0x011F;
#[cfg (feature = "have_ntgui")]
pub const WM_MEASUREITEM: c_int = 0x002C;
#[cfg (feature = "have_ntgui")]
pub const WM_DRAWITEM: c_int = 0x002B;
#[cfg (feature = "have_ntgui")]
pub const WM_MOUSEACTIVATE: c_int = 0x0021;
#[cfg (feature = "have_ntgui")]
pub const WM_ACTIVATEAPP: c_int = 0x001C;
#[cfg (feature = "have_ntgui")]
pub const WM_ACTIVATE: c_int = 0x0006;
#[cfg (feature = "have_ntgui")]
pub const WM_WINDOWPOSCHANGED: c_int = 0x0047;
#[cfg (feature = "have_ntgui")]
pub const WM_SHOWWINDOW: c_int = 0x0018;
#[cfg (feature = "have_ntgui")]
pub const WM_SETTINGCHANGE: c_int = 0x001A;
#[cfg (feature = "have_ntgui")]
pub const WM_SETFOCUS: c_int = 0x0007;
#[cfg (feature = "have_ntgui")]
pub const WM_KILLFOCUS: c_int = 0x0008;
#[cfg (feature = "have_ntgui")]
pub const WM_COMMAND: c_int = 0x0111;
#[cfg (feature = "have_ntgui")]
pub const WM_MOVE: c_int = 0x0003;
#[cfg (feature = "have_ntgui")]
pub const WM_SIZE: c_int = 0x0005;
#[cfg (feature = "have_ntgui")]
pub const WM_CREATE: c_int = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const WM_DESTROY: c_int = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const WM_CLOSE: c_int = 0x0010;
#[cfg (feature = "have_ntgui")]
pub const WM_ENDSESSION: c_int = 0x0016;
#[cfg (feature = "have_ntgui")]
pub const WM_WINDOWPOSCHANGING: c_int = 0x0046;
#[cfg (feature = "have_ntgui")]
pub const WM_GETMINMAXINFO: c_int = 0x0024;
#[cfg (feature = "have_ntgui")]
pub const WM_SETCURSOR: c_int = 0x0020;
#[cfg (feature = "have_ntgui")]
pub const WM_HSCROLL: c_int = 0x0114;
#[cfg (feature = "have_ntgui")]
pub const WM_VSCROLL: c_int = 0x0115;
#[cfg (feature = "have_ntgui")]
pub const WM_LBUTTONDOWN: c_int = 0x0201;
#[cfg (feature = "have_ntgui")]
pub const WM_RBUTTONDOWN: c_int = 0x0204;
#[cfg (feature = "have_ntgui")]
pub const WM_RBUTTONUP: c_int = 0x0205;
#[cfg (feature = "have_ntgui")]
pub const WM_MBUTTONDOWN: c_int = 0x0207;
#[cfg (feature = "have_ntgui")]
pub const WM_MBUTTONUP: c_int = 0x0208;
#[cfg (feature = "have_ntgui")]
pub const WM_MOUSEFIRST: c_int = 0x0200;
#[cfg (feature = "have_ntgui")]
pub const WM_MOUSELAST: c_int = 0x020E;
#[cfg (feature = "have_ntgui")]
pub const WM_IME_STARTCOMPOSITION: c_int = 0x010D;
#[cfg (feature = "have_ntgui")]
pub const WM_IME_ENDCOMPOSITION: c_int = 0x010E;
/// C 5716-5718: `#ifndef WM_TOUCH / #define WM_TOUCH 576 / #endif`, the "define it if the SDK has
/// not" guard of C 5714-5719's `#ifdef WM_TOUCHMOVE` arm (that arm is False for every
/// configuration of *this* file: `WM_TOUCHMOVE` is `w32term.c` 141's `#define`, another
/// translation unit - the ledger's Deviations record the reading).
/// C: #define WM_TOUCH 576
#[cfg (feature = "have_ntgui")]
pub const WM_TOUCH: c_int = 576;
/// C 5113-5115/5185-5187: `#ifndef MOUSEEVENTF_FROMTOUCH / #define MOUSEEVENTF_FROMTOUCH
/// 0xFF515700 / #endif` - the same guard shape (the value is the SDK's).
/// C: #define MOUSEEVENTF_FROMTOUCH 0xFF515700
#[cfg (feature = "have_ntgui")]
pub const MOUSEEVENTF_FROMTOUCH: DWORD = 0xFF515700;
/// C: #define PM_NOYIELD 0x0002
#[cfg (feature = "have_ntgui")]
pub const PM_NOYIELD: UINT = 0x0002;
/// C: #define HOVER_DEFAULT 0xFFFFFFFF
#[cfg (feature = "have_ntgui")]
pub const HOVER_DEFAULT: DWORD = 0xFFFFFFFF;
/// C: #define CFS_RECT 0x0001
#[cfg (feature = "have_ntgui")]
pub const CFS_RECT: DWORD = 0x0001;
/// C: #define GCS_RESULTSTR 0x0800
#[cfg (feature = "have_ntgui")]
pub const GCS_RESULTSTR: DWORD = 0x0800;
/* C's `UNICODE_NOCHAR` (0xFFFF, `#define UNICODE_NOCHAR 0xFFFF`) is `include/w32term.rs` 962's
   constant already, so it is not repeated here; C's `wParam == UNICODE_NOCHAR` compares the `UINT`
   with it, which the port writes out at the use site.  */
/// C: #define MA_NOACTIVATE 3
#[cfg (feature = "have_ntgui")]
pub const MA_NOACTIVATE: LRESULT = 3;
/// C: #define HTCLIENT 1
#[cfg (feature = "have_ntgui")]
pub const HTCLIENT: c_int = 1;
/// C: #define SM_CYMENUSIZE 51 / #define SM_CXMENUCHECK 71
#[cfg (feature = "have_ntgui")]
pub const SM_CYMENUSIZE: c_int = 51;
#[cfg (feature = "have_ntgui")]
pub const SM_CXMENUCHECK: c_int = 71;
/// C: #define FW_BOLD 700 / #define OBJ_FONT 6
#[cfg (feature = "have_ntgui")]
pub const FW_BOLD: c_int = 700;
#[cfg (feature = "have_ntgui")]
pub const OBJ_FONT: c_int = 6;
/// C: #define ETO_OPAQUE 0x0002 / #define ODT_MENU 1 / #define SRCCOPY 0x00CC0020
#[cfg (feature = "have_ntgui")]
pub const ETO_OPAQUE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const ODT_MENU: UINT = 1;
#[cfg (feature = "have_ntgui")]
pub const SRCCOPY: DWORD = 0x00CC0020;
/// C: #define TPM_CENTERALIGN 0x0004 / #define TPM_LEFTBUTTON 0x0000 / #define TPM_RIGHTBUTTON
/// C: 0x0002
#[cfg (feature = "have_ntgui")]
pub const TPM_CENTERALIGN: UINT = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const TPM_LEFTBUTTON: UINT = 0x0000;
#[cfg (feature = "have_ntgui")]
pub const TPM_RIGHTBUTTON: UINT = 0x0002;
/// C: #define VK_SPACE 0x20 / #define VK_CANCEL 0x03 / #define VK_PAUSE 0x13 /
/// C: #define VK_PROCESSKEY 0xE5 / #define VK_NUMPAD0 0x60 / #define VK_DIVIDE 0x6F
#[cfg (feature = "have_ntgui")]
pub const VK_SPACE: c_int = 0x20;
#[cfg (feature = "have_ntgui")]
pub const VK_CANCEL: c_int = 0x03;
#[cfg (feature = "have_ntgui")]
pub const VK_PAUSE: c_int = 0x13;
#[cfg (feature = "have_ntgui")]
pub const VK_PROCESSKEY: c_int = 0xE5;
#[cfg (feature = "have_ntgui")]
pub const VK_NUMPAD0: c_int = 0x60;
#[cfg (feature = "have_ntgui")]
pub const VK_DIVIDE: c_int = 0x6F;

/* The entry points of this slice's two functions (C's own prototypes).  */
/// C: BOOL WINAPI PeekMessageW (LPMSG lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax,
/// C: UINT wRemoveMsg);
/// C: BOOL WINAPI TranslateMessage (const MSG *lpMsg);
/// C: LPARAM WINAPI GetMessageExtraInfo (void);
/// C: HWND WINAPI SetCapture (HWND hWnd);
/// C: BOOL WINAPI ReleaseCapture (void);
/// C: UINT_PTR WINAPI SetTimer (HWND hWnd, UINT_PTR nIDEvent, UINT uElapse, TIMERPROC lpTimerFunc);
/// C: BOOL WINAPI KillTimer (HWND hWnd, UINT_PTR uIDEvent);
/// C: HWND WINAPI SetFocus (HWND hWnd);
/// C: BOOL WINAPI GetUpdateRect (HWND hWnd, LPRECT lpRect, BOOL bErase);
/// C: HDC WINAPI BeginPaint (HWND hWnd, LPPAINTSTRUCT lpPaint);
/// C: BOOL WINAPI EndPaint (HWND hWnd, const PAINTSTRUCT *lpPaint);
/// C: BOOL WINAPI UnionRect (LPRECT lprcDst, const RECT *lprcSrc1, const RECT *lprcSrc2);
/// C: BOOL WINAPI BitBlt (HDC hdc, int x, int y, int cx, int cy, HDC hdcSrc, int x1, int y1,
/// C: DWORD rop);
/// C: BOOL WINAPI GetClientRect (HWND hWnd, LPRECT lpRect);
/// C: HDC WINAPI GetDC (HWND hWnd);
/// C: int WINAPI ReleaseDC (HWND hWnd, HDC hDC);
/// C: HGDIOBJ WINAPI GetCurrentObject (HDC hdc, UINT type);
/// C: int WINAPI GetObject (HANDLE h, int c, LPVOID pv); /* the macro's GetObjectW arm is
/// C: GetObjectW (C 5047), used by the IME arm */
/// C: int WINAPI GetObjectW (HANDLE h, int c, LPVOID pv);
/// C: HFONT WINAPI CreateFontIndirect (const LOGFONTA *lplf);
/// C: HGDIOBJ WINAPI SelectObject (HDC hdc, HGDIOBJ h);
/// C: BOOL WINAPI GetTextExtentPoint32 (HDC hdc, LPCSTR lpString, int c, LPSIZE psizl);
/// C: BOOL WINAPI GetTextExtentPoint32W (HDC hdc, LPCWSTR lpString, int c, LPSIZE psizl);
/// C: BOOL WINAPI ExtTextOut (HDC hdc, int x, int y, UINT options, const RECT *lprect,
/// C: LPCSTR lpString, UINT c, const INT *lpDx);
/// C: BOOL WINAPI ExtTextOutW (HDC hdc, int x, int y, UINT options, const RECT *lprect,
/// C: LPCWSTR lpString, UINT c, const INT *lpDx);
/// C: BOOL WINAPI TrackPopupMenu (HMENU hMenu, UINT uFlags, int x, int y, int nReserved,
/// C: HWND hWnd, const RECT *prcRect);
/// C: ATOM WINAPI CreateCaret (HWND hWnd, HBITMAP hBitmap, int nWidth, int nHeight);
/// C: BOOL WINAPI SetCaretPos (int X, int Y);
/// C: BOOL WINAPI ShowCaret (HWND hWnd);
/// C: BOOL WINAPI HideCaret (HWND hWnd);
/// C: BOOL WINAPI DestroyCaret (void);
/// C: void WINAPI RevokeDragDrop (HWND hwnd);
/// C: BOOL WINAPI DestroyWindow (HWND hWnd);
/// C: DWORD WINAPI GetWindowThreadProcessId (HWND hWnd, LPDWORD lpdwProcessId);
/// C: BOOL WINAPI AttachThreadInput (DWORD idAttach, DWORD idAttachTo, BOOL fAttach);
/// C: BOOL WINAPI SetForegroundWindow (HWND hWnd);
/// C: BOOL WINAPI BringWindowToTop (HWND hWnd);
/// C: void WINAPI CoUninitialize (void);
/// C: HCURSOR WINAPI SetCursor (HCURSOR hCursor);
/// C: SHORT WINAPI VkKeyScanW (WCHAR ch);
/// C: LRESULT WINAPI DefWindowProcA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: LRESULT WINAPI DefWindowProcW (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn PeekMessageW (lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
		       wRemoveMsg: UINT) -> BOOL;
  pub fn TranslateMessage (lpMsg: *const MSG) -> BOOL;
  pub fn GetMessageExtraInfo () -> LPARAM;
  pub fn SetCapture (hWnd: HWND) -> HWND;
  pub fn ReleaseCapture () -> BOOL;
  pub fn SetTimer (hWnd: HWND, nIDEvent: UINT_PTR, uElapse: UINT, lpTimerFunc: *mut c_void)
    -> UINT_PTR;
  pub fn KillTimer (hWnd: HWND, uIDEvent: UINT_PTR) -> BOOL;
  pub fn SetFocus (hWnd: HWND) -> HWND;
  pub fn GetUpdateRect (hWnd: HWND, lpRect: LPRECT, bErase: BOOL) -> BOOL;
  pub fn BeginPaint (hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
  pub fn EndPaint (hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
  pub fn UnionRect (lprcDst: LPRECT, lprcSrc1: *const RECT, lprcSrc2: *const RECT) -> BOOL;
  pub fn BitBlt (hdc: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC, x1: c_int,
		 y1: c_int, rop: DWORD) -> BOOL;
  pub fn GetClientRect (hWnd: HWND, lpRect: LPRECT) -> BOOL;
  pub fn GetDC (hWnd: HWND) -> HDC;
  pub fn ReleaseDC (hWnd: HWND, hDC: HDC) -> c_int;
  pub fn GetDeviceCaps (hdc: HDC, nIndex: c_int) -> c_int;
  /// C 7691's reader (`<winuser.h>`): `BOOL GetCursorPos (LPPOINT lpPoint);`
  pub fn GetCursorPos (lpPoint: LPPOINT) -> BOOL;
  pub fn CreateDCA (pwszDriver: *const c_char, pwszDevice: *mut c_char, pszPort: *const c_char,
		    pdm: *mut c_void) -> HDC;
  pub fn DeleteDC (hdc: HDC) -> BOOL;
  /// C 7071's reader: the declaration `src/w32term.rs` 326 carries for C 384, added here
  /// because this unit's `<windows.h>` block has no copy of it.
  /// C: BOOL WINAPI SystemParametersInfoA (UINT uiAction, UINT uiParam, PVOID pvParam,
  /// C:                            UINT fWinIni);
  pub fn SystemParametersInfo (uiAction: UINT, uiParam: UINT, pvParam: *mut c_void,
			       fWinIni: UINT) -> BOOL;
  /// C 7156/7228's reader (and C 6121's, which this file has called since slice 6):
  /// `frame.c` 6180 defines it and no header declares it (the `src/xfns.rs` 6877 note).
  /// C: extern void validate_x_resource_name (void);
  pub fn validate_x_resource_name ();
  /// C 7790's reader: `frame.h` 1509's `extern Lisp_Object delete_frame (Lisp_Object, Lisp_Object);`
  /// (the `src/androidfns.rs` 117 declaration of the same function).
  pub fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object;
  /// C 8036's readers (`dispextern.h` 3573/3918, the `src/androidfns.rs` 2981-2984 pair):
  /// C: int try_window (Lisp_Object, struct text_pos, int);
  /// C: void update_single_window (struct window *);
  pub fn try_window (window: Lisp_Object, pos: text_pos, flags: c_int) -> c_int;
  pub fn update_single_window (w: *mut window);
  /// C 7237's reader: this unit's slice 1 already lists `fatal` among `lisp.h`'s entry points
  /// and never declared it; C's prototype is variadic.
  /// C: extern AVOID void fatal (const char *msgid, ...);
  pub fn fatal (msgid: *const c_char, ...) -> !;
  pub fn GetCurrentObject (hdc: HDC, r#type: UINT) -> HGDIOBJ;
  pub fn GetObject (h: HANDLE, c: c_int, pv: LPVOID) -> c_int;
  pub fn GetObjectW (h: HANDLE, c: c_int, pv: LPVOID) -> c_int;
  pub fn CreateFontIndirect (lplf: *const LOGFONT) -> HFONT;
  pub fn SelectObject (hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
  pub fn GetTextExtentPoint32 (hdc: HDC, lpString: *const c_char, c: c_int, psizl: LPSIZE)
    -> BOOL;
  pub fn GetTextExtentPoint32W (hdc: HDC, lpString: *const c_ushort, c: c_int, psizl: LPSIZE)
    -> BOOL;
  pub fn ExtTextOut (hdc: HDC, x: c_int, y: c_int, options: UINT, lprect: *const RECT,
		     lpString: *const c_char, c: UINT, lpDx: *const c_int) -> BOOL;
  pub fn ExtTextOutW (hdc: HDC, x: c_int, y: c_int, options: UINT, lprect: *const RECT,
		      lpString: *const c_ushort, c: UINT, lpDx: *const c_int) -> BOOL;
  pub fn TrackPopupMenu (hMenu: HMENU, uFlags: UINT, x: c_int, y: c_int, nReserved: c_int,
			 hWnd: HWND, prcRect: *const RECT) -> BOOL;
  pub fn CreateCaret (hWnd: HWND, hBitmap: HBITMAP, nWidth: c_int, nHeight: c_int) -> ATOM;
  pub fn SetCaretPos (X: c_int, Y: c_int) -> BOOL;
  pub fn ShowCaret (hWnd: HWND) -> BOOL;
  pub fn HideCaret (hWnd: HWND) -> BOOL;
  pub fn DestroyCaret () -> BOOL;
  pub fn RevokeDragDrop (hwnd: HWND);
  pub fn DestroyWindow (hWnd: HWND) -> BOOL;
  pub fn GetWindowThreadProcessId (hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
  pub fn AttachThreadInput (idAttach: DWORD, idAttachTo: DWORD, fAttach: BOOL) -> BOOL;
  pub fn SetForegroundWindow (hWnd: HWND) -> BOOL;
  pub fn BringWindowToTop (hWnd: HWND) -> BOOL;
  pub fn CoUninitialize ();
  pub fn SetCursor (hCursor: HCURSOR) -> HCURSOR;
  pub fn VkKeyScanW (ch: WCHAR) -> SHORT;
  pub fn DefWindowProcA (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn DefWindowProcW (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
}

/// `<windef.h>`'s two word extractors and `<windowsx.h>`'s two coordinate extractors, which
/// C 4727, 5042-5043, 5242-5243, 5270-5271, 5332-5333, 5620 and 5920 use.
/// C: #define LOWORD(l) ((WORD)(((DWORD_PTR)(l)) & 0xffff))
/// C: #define HIWORD(l) ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))
/// C: #define GET_X_LPARAM(lp) ((int)(short)LOWORD(lp))
/// C: #define GET_Y_LPARAM(lp) ((int)(short)HIWORD(lp))
#[cfg (feature = "have_ntgui")]
macro_rules! LOWORD { ($l:expr) => { (($l as usize) & 0xffff) as WORD }; }
#[cfg (feature = "have_ntgui")]
macro_rules! HIWORD { ($l:expr) => { ((($l as usize) >> 16) & 0xffff) as WORD }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GET_X_LPARAM { ($lp:expr) => { LOWORD! ($lp) as i16 as c_int }; }
#[cfg (feature = "have_ntgui")]
macro_rules! GET_Y_LPARAM { ($lp:expr) => { HIWORD! ($lp) as i16 as c_int }; }

/* C 5110's `#define MOUSEEVENTF_FROMTOUCH` and C 5348's `tme.dwHoverTime = HOVER_DEFAULT` are
   above; `TRACKMOUSEEVENT`/`TME_LEAVE`/`WM_MOUSELEAVE`/`WM_MOUSEWHEEL`/`WM_MOUSEHWHEEL`/
   `WM_XBUTTONDOWN`/`WM_XBUTTONUP`/`WM_APPCOMMAND`/`WM_UNICHAR` and the `VK_LWIN`/`VK_RWIN`/
   `VK_APPS`/`LEFT|RIGHT_WIN_PRESSED`/`APPS_PRESSED` family come from `include/w32term.rs`
   (C 664-686 and 831-861 of `w32term.h`), so they are not repeated here.  */

/* The `pointer_invisible` accessor: `frame.h` 437's bitfield is a `struct frame` member (C writes
   `f->pointer_invisible`, a field no header mirror carries an accessor for), so this unit defines
   its own per-unit macro over `include/frame.rs`'s shift constant - the `androidterm.rs` 240 /
   `frame.rs` 334 shape, body-identical.  `hourglass_p`/`menubar_active` are `w32_output` members
   instead and every use writes C's own field path (`w32menu.rs` 938's reading of the same two
   members), which is also what hands the body token gate `output_data`, `w32` and the member.  */
#[cfg (feature = "have_ntgui")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "have_ntgui")]
macro_rules! pointer_invisible {
  ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_POINTER_INVISIBLE_SHIFT, $v) };
}

/// keyboard.h 525: `extern const char *const lispy_function_keys[];`.  `keyboard.c`'s
/// `HAVE_NTGUI` arm is the 256-entry table of C 5212-5403, and `w32_wnd_proc` indexes it by
/// `wParam` (C 4712, 4881), so the declaration carries the reference's own length.
/// `include/keyboard.rs` does not carry it yet (the ledger's owed row), which is why it stands
/// here - the P-88 ① shape for a declaration the unit's include graph cannot supply.
/// C: #if defined HAVE_NTGUI || defined HAVE_X_WINDOWS || defined HAVE_PGTK
/// C: extern const char *const lispy_function_keys[];
/// C: #endif
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub static lispy_function_keys: [*const c_char; 256];
}

/* C 4043-4133: `get_wm_chars`.  The four message tests of the `while` condition compare C's
   `UINT msg.message` with the SDK ids, and the port's ids are `c_int` (slice 3's reading), so
   each comparison writes C's conversion out; `PM_NOREMOVE | PM_NOYIELD` is C's own bit-or of
   the two `UINT` constants.  C's `*buf++` is `*buf` plus `buf.add (1)`, C's `if (ctrl_cnt)` is
   the null test, and `code_unit` is C's uninitialised local (`int i = buflen, doubled = 0,
   code_unit;`, PORTING.md 1.5).  */
/// C: static int get_wm_chars (HWND aWnd, int *buf, int buflen, int ignore_ctrl, int ctrl,
/// C:                           int *ctrl_cnt, int *is_dead, int vk, int exp);
#[cfg (feature = "have_ntgui")]
unsafe fn get_wm_chars (aWnd: HWND, mut buf: *mut c_int, mut buflen: c_int, ignore_ctrl: c_int,
			ctrl: c_int, ctrl_cnt: *mut c_int, is_dead: *mut c_int, vk: c_int,
			exp: c_int) -> c_int
{
  let mut msg: MSG = core::mem::zeroed ();
  /* If doubled is at the end, ignore it.  */
  let mut i: c_int = buflen;
  let mut doubled: c_int = 0;
  /* PORTING.md 1.5: C's `int i = buflen, doubled = 0, code_unit;` leaves `code_unit`
     uninitialised.  */
  let mut code_unit: c_int = 0;

  if !ctrl_cnt.is_null ()
    { *ctrl_cnt = 0; }
  if !is_dead.is_null ()
    { *is_dead = -1; }
  eassert! (w32_unicode_gui != 0);
  while buflen != 0
	 /* Should be called only when w32_unicode_gui:  */
	 && PeekMessageW (&raw mut msg, aWnd, WM_KEYFIRST as UINT, WM_KEYLAST as UINT,
			  PM_NOREMOVE | PM_NOYIELD) != 0
	 && (msg.message as c_int == WM_CHAR || msg.message as c_int == WM_SYSCHAR
	     || msg.message as c_int == WM_DEADCHAR || msg.message as c_int == WM_SYSDEADCHAR
	     || msg.message as c_int == WM_UNICHAR)
    {
      /* We extract character payload, but in this call we handle only the
	 characters which come BEFORE the next keyup/keydown message.  */
      let mut dead: c_int;

      GetMessageW (&raw mut msg, aWnd, msg.message, msg.message);
      dead = if msg.message as c_int == WM_DEADCHAR || msg.message as c_int == WM_SYSDEADCHAR
	     { 1 } else { 0 };
      if !is_dead.is_null ()
	{ *is_dead = if dead != 0 { msg.wParam as c_int } else { -1 }; }
      if dead != 0
	{ continue; }
      code_unit = msg.wParam as c_int;
      if doubled != 0
	{
	  /* Had surrogate.  */
	  if msg.message as c_int == WM_UNICHAR
	      || code_unit < 0xDC00 || code_unit > 0xDFFF
	    { /* Mismatched first surrogate.
		 Pass both code units as if they were two characters.  */
	      *buf = doubled;
	      buf = buf.add (1);
	      buflen -= 1;
	      if buflen == 0
		{ return i; }	/* Drop the 2nd char if at the end of the buffer.  */
	    }
	  else	/* see https://en.wikipedia.org/wiki/UTF-16 */
	    { code_unit = (doubled << 10) + code_unit - 0x35FDC00; }
	  doubled = 0;
	}
      else if code_unit >= 0xD800 && code_unit <= 0xDBFF
	{
	  /* Handle mismatched 2nd surrogate the same as a normal character.  */
	  doubled = code_unit;
	  continue;
	}

      /* The only "fake" characters delivered by ToUnicode() or
	 TranslateMessage() are:
	 0x01 .. 0x1a for Ctrl-letter, Enter, Tab, Ctrl-Break, Esc, Backspace
	 0x00 and 0x1b .. 0x1f for Control- []\@^_
	 0x7f for Control-BackSpace
	 0x20 for Control-Space */
      if ignore_ctrl != 0
	  && (code_unit < 0x20 || code_unit == 0x7f
	      || (code_unit == 0x20 && ctrl != 0))
	{
	  /* Non-character payload in a WM_CHAR
	     (Ctrl-something pressed, see above).  Ignore, and report.  */
	  if !ctrl_cnt.is_null ()
	    { *ctrl_cnt += 1; }
	  continue;
	}
      /* Traditionally, Emacs would ignore the character payload of VK_NUMPAD*
	 keys, and would treat them later via `function-key-map'.  In addition
	 to usual 102-key NUMPAD keys, this map also treats `kp-'-variants of
	 space, tab, enter, separator, equal.  TAB  and EQUAL, apparently,
	 cannot be generated on Win-GUI branch.  ENTER is already handled
	 by the code above.  According to `lispy_function_keys', kp_space is
	 generated by not-extended VK_CLEAR.  (kp-tab !=  VK_OEM_NEC_EQUAL!).

	 We do similarly for backward-compatibility, but ignore only the
	 characters restorable later by `function-key-map'.  */
      if code_unit < 0x7f
	  && ((vk >= VK_NUMPAD0 && vk <= VK_DIVIDE)
	      || (exp != 0 && ((vk >= VK_PRIOR && vk <= VK_DOWN)
			       || vk == VK_INSERT || vk == VK_DELETE || vk == VK_CLEAR)))
	  && !strchr (c"0123456789/*-+.,".as_ptr (), code_unit).is_null ()
	{ continue; }
      *buf = code_unit;
      buf = buf.add (1);
      buflen -= 1;
    }
  return i - buflen;
}

/* C 4135-4139: `#ifdef DBG_WM_CHARS`.  C's `ARG` is the *argument list* of a call
   (`FPRINTF_WM_CHARS ((stderr, "…", …))`), so this `macro_rules!`'s pattern is C's own
   parenthesised list.  The `#else` arm is C's `(void)0`, which does **not** evaluate `ARG`
   (PORTING.md 2.1/P-16's `eassert` reading), so that body is the unit value.  `dbg_wm_chars` is
   a feature `femacs/xtask/Cargo.toml` does not register yet (the ledger's owed list), exactly
   as `emacsdebug`/`msg_debug` above and `w32_debug_display` below.  */
// C: #ifdef DBG_WM_CHARS
// C: #  define FPRINTF_WM_CHARS(ARG)	fprintf ARG
// C: #else
// C: #  define FPRINTF_WM_CHARS(ARG)	(void)0
// C: #endif
#[cfg (all (feature = "have_ntgui", feature = "dbg_wm_chars"))]
macro_rules! FPRINTF_WM_CHARS {
  (($($arg:tt)*)) => { fprintf ($($arg)*) };
}
#[cfg (all (feature = "have_ntgui", not (feature = "dbg_wm_chars")))]
macro_rules! FPRINTF_WM_CHARS {
  (($($arg:tt)*)) => { () };
}

/* This is a heuristic only.  This is supposed to track the state of the
   finite automaton in the language environment of Windows.

   However, separate windows (if with the same different language
   environments!) should  have different values.  Moreover, switching to a
   non-Emacs window with the same language environment, and using (dead)keys
   there would change the value stored in the kernel, but not this value.  */
/* A layout may emit deadkey=0.  It looks like this would reset the state
   of the kernel's finite automaton (equivalent to emitting 0-length string,
   which is otherwise impossible in the dead-key map of a layout).
   Be ready to treat the case when this delivers WM_(SYS)DEADCHAR. */
/// C 4152: `static int after_deadkey = -1;` - the two comments above are C's own text.
/// C: static int after_deadkey = -1;
#[cfg (feature = "have_ntgui")]
static mut after_deadkey: c_int = -1;

/// C 4154-4488: `static int deliver_wm_chars (int do_translate, HWND hwnd, UINT msg, UINT
/// wParam, UINT lParam, int legacy_alt_meta)`.  C's five locals of the first declaration are
/// its uninitialised ones (`int ctrl_cnt, buf[1024], count, is_dead, after_dead = …`; the port
/// zeroes the four - PORTING.md 1.5); `buf` is filled by `get_wm_chars` through its `int *buf`
/// parameter.  C's `MSG windows_msg = { hwnd, msg, wParam, lParam, 0, {0,0} };` is the four
/// field writes plus the zeroed tail the initialiser gives the two others, and the `while
/// (count--)` at the foot is the post-decrement loop the test spells out (`src/xdisp.rs`'s
/// deviation-86 shape: the decrement runs on the test that ends the loop too).
/// C: static int deliver_wm_chars (int do_translate, HWND hwnd, UINT msg, UINT wParam,
/// C:                           UINT lParam, int legacy_alt_meta);
#[cfg (feature = "have_ntgui")]
unsafe fn deliver_wm_chars (do_translate: c_int, hwnd: HWND, msg: UINT, wParam: UINT,
			    lParam: UINT, legacy_alt_meta: c_int) -> c_int
{
  /* An "old style" keyboard description may assign up to 125 UTF-16 code
     points to a keypress.
     (However, the "old style" TranslateMessage() would deliver at most 16 of
     them.)  Be on a safe side, and prepare to treat many more.  */
  let mut ctrl_cnt: c_int = 0;
  let mut buf: [c_int; 1024] = [0; 1024];
  let mut count: c_int = 0;
  let mut is_dead: c_int = 0;
  let mut after_dead: c_int = if after_deadkey > 0 { 1 } else { 0 };

  /* Since the keypress processing logic of Windows has a lot of state, it
     is important to call TranslateMessage() for every keyup/keydown, AND
     do it exactly once.  (The actual change of state is done by
     ToUnicode[Ex](), which is called by TranslateMessage().  So one can
     call ToUnicode[Ex]() instead.)

     The "usual" message pump calls TranslateMessage() for EVERY event.
     Emacs calls TranslateMessage() very selectively (is it needed for doing
     some tricky stuff with Win95???  With newer Windows, selectiveness is,
     most probably, not needed -- and harms a lot).

     So, with the usual message pump, the following call to TranslateMessage()
     is not needed (and is going to be VERY harmful).  With Emacs's message
     pump, the call is needed.  */
  if do_translate != 0
    {
      let mut windows_msg: MSG = core::mem::zeroed ();

      windows_msg.hwnd = hwnd;
      windows_msg.message = msg;
      windows_msg.wParam = wParam as WPARAM;
      windows_msg.lParam = lParam as LPARAM;

      windows_msg.time = GetMessageTime () as DWORD;
      TranslateMessage (&raw const windows_msg);
    }
  count = get_wm_chars (hwnd, buf.as_mut_ptr (), countof! (buf) as c_int, 1,
			/* The message may have been synthesized by
			   who knows what; be conservative.  */
			(if modifier_set (VK_LCONTROL) != 0 || modifier_set (VK_RCONTROL) != 0
			    || modifier_set (VK_CONTROL) != 0 { 1 } else { 0 }),
			&raw mut ctrl_cnt, &raw mut is_dead, wParam as c_int,
			(if lParam & 0x1000000 != 0 { 1 } else { 0 }));
  if count != 0
    {
      let mut wmsg: W32Msg = core::mem::zeroed ();
      let console_modifiers: DWORD = w32_construct_console_modifiers () as DWORD;
      let mut b: *mut c_int = buf.as_mut_ptr ();
      let mut strip_ExtraMods: c_int = 1;
      let mut hairy: c_int = 0;
      let mut type_CtrlAlt: *const c_char = core::ptr::null ();

      /*  XXXX In fact, there may be another case when we need to do the same:
	       What happens if the string defined in the LIGATURES has length
	       0?  Probably, we will get count==0, but the state of the finite
	       automaton would reset to 0???  */
      after_deadkey = -1;

      /* wParam is checked when converting CapsLock to Shift; this is a clone
	 of w32_get_key_modifiers ().  */
      wmsg.dwModifiers = w32_kbd_mods_to_emacs (console_modifiers, wParam as WORD) as DWORD;

      /* What follows is just heuristics; the correct treatment requires
	 non-destructive ToUnicode():
	   https://metacpan.org/dist/UI-KeyboardLayout/view/lib/UI/KeyboardLayout.pm#Can-an-application-on-Windows-accept-keyboard-events?-Part-IV:-application-specific-modifiers

	 What one needs to find is:
	   * which of the present modifiers AFFECT the resulting char(s)
	     (so should be stripped, since their EFFECT is "already
	      taken into account" in the string in buf), and
	   * which modifiers are not affecting buf, so should be reported to
	     the application for further treatment.

	 Example: assume that we know:
	   (A) lCtrl+rCtrl+rAlt modifiers with VK_A key produce a Latin "f"
	       ("may be logical" in JCUKEN-flavored Russian keyboard flavors);
	   (B) removing any of lCtrl, rCtrl, rAlt changes the produced char;
	   (C) Win-modifier is not affecting the produced character
	       (this is the common case: happens with all "standard" layouts).

	 Suppose the user presses Win+lCtrl+rCtrl+rAlt modifiers with VK_A.
	 What is the intent of the user?  We need to guess the intent to decide
	 which event to deliver to the application.

	 This looks like a reasonable logic: since Win- modifier doesn't affect
	 the output string, the user was pressing Win for SOME OTHER purpose.
	 So the user wanted to generate Win-SOMETHING event.  Now, what is
	 something?  If one takes the mantra that "character payload is more
	 important than the combination of keypresses which resulted in this
	 payload", then one should ignore lCtrl+rCtrl+rAlt, ignore VK_A, and
	 assume that the user wanted to generate Win-f.

	 Unfortunately, without non-destructive ToUnicode(), checking (B),(C)
	 is out of question.  So we use heuristics (hopefully, covering
	 99.9999% of cases).  */

      /* Another thing to watch for is a possibility to use AltGr-* and
	 Ctrl-Alt-* with different semantic.

	 Background: the layout defining the KLLF_ALTGR bit are treated
	 specially by the kernel: when VK_RMENU (=rightAlt, =AltGr) is pressed
	 (released), a press (release) of VK_LCONTROL is emulated (unless Ctrl
	 is already down).  As a result, any press/release of AltGr is seen
	 by applications as a press/release of lCtrl AND rAlt.  This is
	 applicable, in particular, to ToUnicode[Ex]().  (Keyrepeat is covered
	 the same way!)

	   NOTE: it IS possible to see bare rAlt even with KLLF_ALTGR; but this
	   requires a good finger coordination: doing (physically)
	     Down-lCtrl Down-rAlt Up-lCtrl Down-a
	   (doing quick enough, so that key repeat of rAlt [which would
	   generate new "fake" Down-lCtrl events] does not happens before 'a'
	   is down) results in no "fake" events, so the application will see
	   only rAlt down when 'a' is pressed.  (However, fake Up-lCtrl WILL
	   be generated when rAlt goes UP.)

	   In fact, note also that KLLF_ALTGR does not prohibit construction of
	   rCtrl-rAlt (just press them in this order!).

	 Moreover: "traditional" layouts do not define distinct modifier-masks
	 for VK_LMENU and VK_RMENU (same for VK_L/RCONTROL).  Instead, they
	 rely on the KLLF_ALTGR bit to make the behavior of VK_LMENU and
	 VK_RMENU distinct.  As a corollary, for such layouts, the produced
	 character is the same for AltGr-* (=rAlt-*) and Ctrl-Alt-* (in any
	 combination of handedness).  For description of masks, see

	   https://metacpan.org/dist/UI-KeyboardLayout/view/lib/UI/KeyboardLayout.pm#Keyboard-input-on-Windows,-Part-I:-what-is-the-kernel-doing?

	 By default, Emacs was using these coincidences via the following
	 heuristics: it was treating:
	  (*) keypresses with lCtrl-rAlt modifiers as if they are carrying
	      ONLY the character payload (no matter what the actual keyboard
	      was defining: if lCtrl-lAlt-b was delivering U+05df=beta, then
	      Emacs saw [beta]; if lCtrl-lAlt-b was undefined in the layout,
	      the keypress was completely ignored), and
	  (*) keypresses with the other combinations of handedness of Ctrl-Alt
	      modifiers (e.g., lCtrl-lAlt) as if they NEVER carry a character
	      payload (so they were reported "raw": if lCtrl-lAlt-b was
	      delivering beta, then Emacs saw event [C-A-b], and not [beta]).
	 This worked good for "traditional" layouts: users could type both
	 AltGr-x and Ctrl-Alt-x, and one was a character, another a bindable
	 event.

	 However, for layouts which deliver different characters for AltGr-x
	 and lCtrl-lAlt-x, this scheme makes the latter character inaccessible
	 in Emacs.  While it is easy to access functionality of [C-M-x] in
	 Emacs by other means (for example, by the `controlify' prefix, or
	 using lCtrl-rCtrl-x, or rCtrl-rAlt-x [in this order]), missing
	 characters cannot be reconstructed without a tedious manual work.  */

      /* These two cases are often going to be distinguishable, since at most
	 one of these character is defined with KBDCTRL | KBDMENU modifier
	 bitmap.  (This heuristic breaks if both lCtrl-lAlt- AND lCtrl-rAlt-
	 are translated to modifier bitmaps distinct from KBDCTRL | KBDMENU,
	 or in the cases when lCtrl-lAlt-* and lCtrl-rAlt-* are generally
	 different, but lCtrl-lAlt-x and lCtrl-rAlt-x happen to deliver the
	 same character.)

	 So we have 2 chunks of info:
	   (A) is it lCtrl-rAlt-, or lCtrl-lAlt, or some other combination?
	   (B) is the delivered character defined with KBDCTRL | KBDMENU bits?
	 Basing on (A) and (B), we should decide whether to ignore the
	 delivered character.  (Before, Emacs was completely ignoring (B), and
	 was treating the 3-state of (A) as a bit.)  This means that we have 6
	 bits of customization.

	 Additionally, a presence of two Ctrl down may be AltGr-rCtrl-.  */

      /* Strip all non-Shift modifiers if:
	- more than one UTF-16 code point delivered (can't call VkKeyScanW ())
	- or the character is a result of combining with a prefix key.  */
      if after_dead == 0 && count == 1 && *b < 0x10000
	{
	  if console_modifiers & (RIGHT_ALT_PRESSED | LEFT_ALT_PRESSED) as DWORD != 0
	      && console_modifiers & (RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED) as DWORD != 0
	    {
	      type_CtrlAlt = c"bB".as_ptr ();   /* generic bindable Ctrl-Alt- modifiers */
	      if console_modifiers & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) as DWORD
		  == (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) as DWORD
		/* double-Ctrl:
		   e.g. AltGr-rCtrl on some layouts (in this order!) */
		{ type_CtrlAlt = c"dD".as_ptr (); }
	      else if console_modifiers & (LEFT_CTRL_PRESSED | LEFT_ALT_PRESSED) as DWORD
		       == (LEFT_CTRL_PRESSED | LEFT_ALT_PRESSED) as DWORD
		{ type_CtrlAlt = c"lL".as_ptr (); } /* Ctrl-Alt- modifiers on the left */
	      else if !NILP (Vw32_recognize_altgr)
		       && console_modifiers & (RIGHT_ALT_PRESSED | LEFT_CTRL_PRESSED) as DWORD
			  == (RIGHT_ALT_PRESSED | LEFT_CTRL_PRESSED) as DWORD
		{ type_CtrlAlt = c"gG".as_ptr (); } /* modifiers as in AltGr */
	    }
	  else if wmsg.dwModifiers & (alt_modifier | meta_modifier) as DWORD != 0
		   || console_modifiers & (LEFT_WIN_PRESSED | RIGHT_WIN_PRESSED
					   | APPS_PRESSED | SCROLLLOCK_ON) as DWORD != 0
	    {
	      /* Pure Alt (or combination of Alt, Win, APPS, scrolllock.  */
	      type_CtrlAlt = c"aA".as_ptr ();
	    }
	  if !type_CtrlAlt.is_null ()
	    {
	      /* Out of bound bitmap:  */
	      let mut r: SHORT = VkKeyScanW (*b as WCHAR);
	      let mut bitmap: SHORT = 0x1FF;

	      FPRINTF_WM_CHARS! ((stderr, c"VkKeyScanW %#06x %#04x\n".as_ptr (), r as c_int,
				  wParam));
	      if r as c_int & 0xFF == wParam as c_int
		{ bitmap = r >> 8; } /* *b is reachable via simple interface */
	      else
		{
		  /* VkKeyScanW() (essentially) returns the FIRST key with
		     the specified character; so here the pressed key is the
		     SECONDARY key producing the character.

		     Essentially, we have no information about the "role" of
		     modifiers on this key: which contribute into the
		     produced character (so "are consumed"), and which are
		     "extra" (must attach to bindable events).

		     The default above would consume ALL modifiers, so the
		     character is reported "as is".  However, on many layouts
		     the ordering of the keys (in the layout table) is not
		     thought out well, so the "secondary" keys are often those
		     which the users would prefer to use with Alt-CHAR.
		     (Moreover - with e.g. Czech-QWERTY - the ASCII
		     punctuation is accessible from two equally [nu]preferable
		     AltGr-keys.)

		     SO:   Heuristic: if the reported char is ASCII, AND Meta
		     modifier is a candidate, behave as if Meta is present
		     (fallback to the legacy branch; bug#23251).

		     (This would break layouts
		     - delivering ASCII characters
		     - on SECONDARY keys
		     - with not Shift/AltGr-like modifier combinations.
		     All 3 conditions together must be pretty exotic
		     cases - and a workaround exists: use "primary" keys!) */
		  if *b < 0x80
		      && wmsg.dwModifiers & (alt_modifier | meta_modifier
					     | super_modifier | hyper_modifier) as DWORD != 0
		    { return 0; }
		}
	      if *type_CtrlAlt as c_int == 'a' as c_int /* Simple Alt seen */
		{
		  if bitmap as c_int & !1 == 0 /* 1: KBDSHIFT */
		    {
		      /* In "traditional" layouts, Alt without Ctrl does not
			 change the delivered character.  This detects this
			 situation; it is safe to report this as Alt-something
			  -- as opposed to delivering the reported character
			 without modifiers.  */
		      if legacy_alt_meta != 0
			  && *b > 0x7f && ('A' as UINT <= wParam && wParam <= 'Z' as UINT)
			/* For backward-compatibility with older Emacsen, let
			   this be processed by another branch below (which
			   would convert it to Alt-Latin char via wParam).  */
			{ return 0; }
		    }
		  else
		    { hairy = 1; }
		}
	      /* Check whether the delivered character(s) is accessible via
		 KBDCTRL | KBDALT ( | KBDSHIFT ) modifier mask (which is 7).  */
	      else if (bitmap as c_int & !1) != 6
		{
		  /* The character is not accessible via plain Ctrl-Alt(-Shift)
		     (which is, probably, same as AltGr) modifiers.
		     Either it was after a prefix key, or is combined with
		     modifier keys which we don't see, or there is an asymmetry
		     between left-hand and right-hand modifiers, or other hairy
		     stuff.  */
		  hairy = 1;
		}
	      /* The best solution is to delegate these tough (but rarely
		 needed) choices to the user.  Temporarily (???), it is
		 implemented as C macros.

		 Essentially, there are 3 things to do: return 0 (handle to the
		 legacy processing code [ignoring the character payload]; keep
		 some modifiers (so that they will be processed by the binding
		 system [on top of the character payload]; strip modifiers [so
		 that `self-insert' is going to be triggered with the character
		 payload]).

		 The default below should cover 99.9999% of cases:
		   (a) strip Alt- in the hairy case only;
		       (stripping = not ignoring)
		   (l) for lAlt-lCtrl, ignore the char in simple cases only;
		   (g) for what looks like AltGr, ignore the modifiers;
		   (d) for what looks like lCtrl-rCtrl-Alt (probably
		       AltGr-rCtrl), ignore the character in simple cases only;
		   (b) for other cases of Ctrl-Alt, ignore the character in
		       simple cases only.

		 Essentially, in all hairy cases, and in looks-like-AltGr case,
		 we keep the character, ignoring the modifiers.  In all the
		 other cases, we ignore the delivered character.  */
	      /* C 4453-4454: `#define S_TYPES_TO_IGNORE_CHARACTER_PAYLOAD "aldb"` and
		 `#define S_TYPES_TO_REPORT_CHARACTER_PAYLOAD_WITH_MODIFIERS ""`; the two
		 `strchr` calls below are C's two tests over them.  */
	      if !strchr (S_TYPES_TO_IGNORE_CHARACTER_PAYLOAD.as_ptr (),
			  *type_CtrlAlt.offset (hairy as isize) as c_int).is_null ()
		{ return 0; }
	      /* If in neither list, report all the modifiers we see COMBINED
		 WITH the reported character.  */
	      if !strchr (S_TYPES_TO_REPORT_CHARACTER_PAYLOAD_WITH_MODIFIERS.as_ptr (),
			  *type_CtrlAlt.offset (hairy as isize) as c_int).is_null ()
		{ strip_ExtraMods = 0; }
	    }
	}
      if strip_ExtraMods != 0
	{ wmsg.dwModifiers = wmsg.dwModifiers & shift_modifier as DWORD; }

      signal_user_input ();
      /* C's `while (count--)`: the post-decrement runs on the test that ends the loop
	 (deviation 86's rule, the `src/xdisp.rs` 15307 shape).  */
      loop
	{
	  let old: c_int = count;
	  count -= 1;
	  if old == 0 { break; }
	  FPRINTF_WM_CHARS! ((stderr, c"unichar %#06x\n".as_ptr (), *b));
	  my_post_msg (&raw mut wmsg, hwnd, WM_UNICHAR as UINT, *b as WPARAM,
				      lParam as LPARAM);
	  b = b.add (1);
	}
      if ctrl_cnt == 0 /* Process ALSO as ctrl */
	{ return 1; }
      else
	{ FPRINTF_WM_CHARS! ((stderr, c"extra ctrl char\n".as_ptr ())); }
      return -1;
    }
  else if is_dead >= 0
    {
      FPRINTF_WM_CHARS! ((stderr, c"dead %#06x\n".as_ptr (), is_dead));
      after_deadkey = is_dead;
      return 1;
    }
  return 0;
}

/// C 4453-4454 (`#define S_TYPES_TO_IGNORE_CHARACTER_PAYLOAD "aldb"` /
/// `#define S_TYPES_TO_REPORT_CHARACTER_PAYLOAD_WITH_MODIFIERS ""`).  The two are `strchr`
/// subjects, so their bytes must be C's - a `&CStr` keeps the terminator (`alloc.rs` 5493's
/// reading of the same choice).
/// C: #define S_TYPES_TO_IGNORE_CHARACTER_PAYLOAD "aldb"
#[cfg (feature = "have_ntgui")]
pub const S_TYPES_TO_IGNORE_CHARACTER_PAYLOAD: &core::ffi::CStr = c"aldb";
/// C: #define S_TYPES_TO_REPORT_CHARACTER_PAYLOAD_WITH_MODIFIERS ""
#[cfg (feature = "have_ntgui")]
pub const S_TYPES_TO_REPORT_CHARACTER_PAYLOAD_WITH_MODIFIERS: &core::ffi::CStr = c"";

/// C 4490-4523: `#ifdef WINDOWSNT`'s `static void maybe_pass_notification (HWND hwnd)` and its
/// `#endif` - the whole item is the `windowsnt` arm of the file gate (the Cygwin GUI build has
/// neither the function nor its callers).  C's `break` leaves the `FOR_EACH_FRAME` loop, which is
/// the plain `break` inside the mirror's macro body (the increment is at the body's end, so the
/// exit skips it - `frame.rs` 2836-2845, the `androidterm.rs` 282 reading).  `f` is C's
/// uninitialised local (PORTING.md 1.5) and `found_frame` guards its use.
/// C: static void maybe_pass_notification (HWND hwnd);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe fn maybe_pass_notification (hwnd: HWND)
{
  if hwnd == kbdhook.notified_wnd
      && kbdhook.hook_count > 0 && w32_kbdhook_active! ()
    {
      let mut tail: Lisp_Object;
      let mut frame: Lisp_Object;
      let mut f: *mut frame = core::ptr::null_mut ();
      let mut found_frame: bool = false;

      FOR_EACH_FRAME! (tail, frame,
	{
	  f = XFRAME (frame);
	  if FRAME_W32_P (f) && !FRAME_OUTPUT_DATA! (f).is_null ()
	      && FRAME_W32_WINDOW! (f) != hwnd
	    {
	      found_frame = true;
	      break;
	    }
	});

      if found_frame && WTSUnRegisterSessionNotification_fn.is_some ()
	  && WTSRegisterSessionNotification_fn.is_some ()
	{
	  /* There is another frame, pass on the session notification.  */
	  let next_wnd: HWND = FRAME_W32_WINDOW! (f);
	  (WTSUnRegisterSessionNotification_fn.unwrap ()) (hwnd);
	  (WTSRegisterSessionNotification_fn.unwrap ()) (next_wnd, NOTIFY_FOR_THIS_SESSION);
	  kbdhook.notified_wnd = next_wnd;
	}
    }
}

/* Main window procedure */

/// C 4525-5986: `static LRESULT CALLBACK w32_wnd_proc (HWND hwnd, UINT msg, WPARAM wParam,
/// LPARAM lParam)`.  C's `switch (msg)` is the `match` below and every `case` one arm; C's
/// `break` (which leaves the switch for the trailing `return 0`) is `break 'dispatch`, the label
/// this block carries (PORTING.md 2.2).  C's four shared labels are the structured spellings the
/// ledger's Deviations and the `macro-body-notes.tsv` rows record: `dflt:` is the
/// `DefWindowProcW/A` return each `goto dflt` writes in place, `translate:` is the `translate`
/// flag's forward jump inside the `WM_KEYDOWN` arm, `handle_plain_button:` and `command:` are the
/// two flags whose bodies run after the match in C's own order (C 5251 and C 5708), and
/// `disable_lock_key:` is the nested switch's own flag (`'wparam'`'s `break`).  `f`, `wmsg`,
/// `windows_translate` and `key` are C's uninitialised locals (PORTING.md 1.5).
/// C: static LRESULT CALLBACK w32_wnd_proc (HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_wnd_proc (hwnd: HWND, mut msg: UINT, mut wParam: WPARAM,
					lParam: LPARAM) -> LRESULT
{
  let mut f: *mut frame = core::ptr::null_mut ();
  let mut dpyinfo: *mut w32_display_info = &raw mut one_w32_display_info;
  let mut wmsg: W32Msg = core::mem::zeroed ();
  let mut windows_translate: c_int = 0;
  let mut key: c_int = 0;
  /* C's `handle_plain_button:` and `command:` labels (C 5251, C 5708): a `match` arm cannot be
     jumped into, so the two bodies run after the match with C's own statements.  */
  let mut handle_plain_button: bool = false;
  let mut command: bool = false;

  /* Note that it is okay to call w32_window_to_frame, even though we are
     not running in the main lisp thread, because frame deletion
     requires the lisp thread to synchronize with this thread.  Thus, if
     a frame struct is returned, it can be used without concern that the
     lisp thread might make it disappear while we are using it.

     NB. Walking the frame list in this thread is safe (as long as
     writes of Lisp_Object slots are atomic, which they are on Windows).
     Although delete-frame can destructively modify the frame list while
     we are walking it, a garbage collection cannot occur until after
     delete-frame has synchronized with this thread.

     It is also safe to use functions that make GDI calls, such as
     w32_clear_rect, because these functions must obtain a DC handle
     from the frame struct using get_frame_dc which is thread-aware.  */

  'dispatch:
  {
    match msg as c_int
      {
      WM_ERASEBKGND =>
	{
	  f = w32_window_to_frame (dpyinfo, hwnd);

	  enter_crit! ();
	  if !f.is_null () && (w32_disable_double_buffering
			       || (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output))
				  .paint_buffer.is_null ())
	    {
	      let hdc: HDC = get_frame_dc (f);
	      GetUpdateRect (hwnd, &raw mut wmsg.rect, FALSE);
	      w32_clear_rect! (f, hdc, &raw mut wmsg.rect);
	      release_frame_dc (f, hdc);

	      #[cfg (feature = "w32_debug_display")]
	      {
		DebPrint! ((c"WM_ERASEBKGND (frame %p): erasing %d,%d-%d,%d\n".as_ptr (),
			    f, wmsg.rect.left, wmsg.rect.top,
			    wmsg.rect.right, wmsg.rect.bottom));
	      }
	    }
	  leave_crit! ();
	  return 1;
	}
      WM_PALETTECHANGED =>
	{
	  /* ignore our own changes */
	  if wParam as HWND != hwnd
	    {
	      f = w32_window_to_frame (dpyinfo, hwnd);
	      if !f.is_null ()
		/* get_frame_dc will realize our palette and force all
		   frames to be redrawn if needed. */
		{ release_frame_dc (f, get_frame_dc (f)); }
	    }
	  return 0;
	}
      WM_PAINT =>
	{
	  let mut paintStruct: PAINTSTRUCT = core::mem::zeroed ();
	  let mut update_rect: RECT = core::mem::zeroed ();
	  memset (&raw mut update_rect as *mut c_void, 0,
		  core::mem::size_of_val (&update_rect));

	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if f.is_null ()
	    {
	      DebPrint! ((c"WM_PAINT received for unknown window %p\n".as_ptr (), hwnd));
	      return 0;
	    }

	  /* MSDN Docs say not to call BeginPaint if GetUpdateRect
	     fails.  Apparently this can happen under some
	     circumstances.  */
	  if GetUpdateRect (hwnd, &raw mut update_rect, FALSE) != 0 || !w32_strict_painting
	    {
	      enter_crit! ();
	      BeginPaint (hwnd, &raw mut paintStruct);

	      /* The rectangles returned by GetUpdateRect and BeginPaint
		 do not always match.  Play it safe by assuming both areas
		 are invalid.  */
	      UnionRect (&raw mut wmsg.rect, &raw const update_rect,
			 &raw const paintStruct.rcPaint);

	      #[cfg (feature = "w32_debug_display")]
	      {
		DebPrint! ((c"WM_PAINT (frame %p): painting %d,%d-%d,%d\n".as_ptr (),
			    f, wmsg.rect.left, wmsg.rect.top,
			    wmsg.rect.right, wmsg.rect.bottom));
		DebPrint! ((c"  [update region is %d,%d-%d,%d]\n".as_ptr (),
			    update_rect.left, update_rect.top,
			    update_rect.right, update_rect.bottom));
	      }
	      /* Under double-buffering, update the frame from the back
		 buffer, to prevent a "ghost" of the selection dialog to
		 be left on display while the user selects in the dialog.  */
	      if w32_selection_dialog_open
		  && !w32_disable_double_buffering
		  && !(*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc.is_null ()
		{ BitBlt ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_buffer_handle,
			  0, 0, FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),
			  (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).paint_dc, 0, 0, SRCCOPY); }

	      EndPaint (hwnd, &raw const paintStruct);
	      leave_crit! ();

	      /* Change the message type to prevent Windows from
		 combining WM_PAINT messages in the Lisp thread's queue,
		 since Windows assumes that each message queue is
		 dedicated to one frame and does not bother checking
		 that hwnd matches before combining them.  */
	      my_post_msg (&raw mut wmsg, hwnd, WM_EMACS_PAINT as UINT, wParam, lParam);

	      return 0;
	    }

	  /* If GetUpdateRect returns 0 (meaning there is no update
	     region), assume the whole window needs to be repainted.  */
	  GetClientRect (hwnd, &raw mut wmsg.rect);
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  return 0;
	}
      WM_INPUTLANGCHANGE =>
	{
	  /* Inform lisp thread of keyboard layout changes.  */
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);

	  /* The state of the finite automaton is separate per every input
	     language environment (so it does not change when one switches
	     to a different window with the same environment).  Moreover,
	     the experiments show that the state is not remembered when
	     one switches back to the pre-previous environment.  */
	  after_deadkey = -1;

	  /* XXXX??? What follows is a COMPLETE misunderstanding of Windows!  */

	  /* Clear dead keys in the keyboard state; for simplicity only
	     preserve modifier key states.  */
	  {
	    let mut i: c_int;
	    let mut keystate: [BYTE; 256] = [0; 256];

	    GetKeyboardState (keystate.as_mut_ptr ());
	    i = 0;
	    while i < 256
	      {
		if true
		    && i != VK_SHIFT
		    && i != VK_LSHIFT
		    && i != VK_RSHIFT
		    && i != VK_CAPITAL
		    && i != VK_NUMLOCK
		    && i != VK_SCROLL
		    && i != VK_CONTROL
		    && i != VK_LCONTROL
		    && i != VK_RCONTROL
		    && i != VK_MENU
		    && i != VK_LMENU
		    && i != VK_RMENU
		    && i != VK_LWIN
		    && i != VK_RWIN
		  { keystate[i as usize] = 0; }
		i += 1;
	      }
	    SetKeyboardState (keystate.as_mut_ptr ());
	  }
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_HOTKEY =>
	{
	  /* Synchronize hot keys with normal input.  */
	  PostMessage (hwnd, WM_KEYDOWN as UINT, HIWORD! (lParam) as WPARAM, 0);
	  return 0;
	}
      WM_KEYUP | WM_SYSKEYUP =>
	{
	  record_keyup (wParam as c_uint, lParam as c_uint);
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_KEYDOWN | WM_SYSKEYDOWN =>
	{
	  /* C's `goto translate` (C 4741) is this flag: it skips the rest of the keydown body and
	     lands on the `translate:` label below.  */
	  let mut translate: bool = false;

	  /* Ignore keystrokes we fake ourself; see below.  */
	  if (*dpyinfo).faked_key == wParam as c_int
	    {
	      (*dpyinfo).faked_key = 0;
	      /* Make sure TranslateMessage sees them though (as long as
		 they don't produce WM_CHAR messages).  This ensures that
		 indicator lights are toggled promptly on Windows 9x, for
		 example.  */
	      if wParam < 256 && !lispy_function_keys[wParam as usize].is_null ()
		{
		  windows_translate = 1;
		  translate = true;	/* C's `goto translate`.  */
		}
	      else
		{ return 0; }
	    }

	  if !translate
	    {
	      /* Synchronize modifiers with current keystroke.  */
	      sync_modifiers ();
	      record_keydown (wParam as c_uint, lParam as c_uint);
	      if w32_use_fallback_wm_chars_method
		{
		  wParam = map_keypad_keys (wParam as c_uint,
					    (if lParam & 0x1000000 != 0 { 1 } else { 0 })
					    ) as WPARAM;
		}

	      windows_translate = 0;

	      /* C's nested `switch (wParam)`; the `disable_lock_key:` label (C 4790) is the flag
		 the three lock-key arms `goto`.  */
	      {
		let mut disable_lock_key: bool = false;

		'wparam:
		{
		  match wParam as c_int
		    {
		    VK_LWIN =>
		      {
			if !w32_kbdhook_active! () && NILP (Vw32_pass_lwindow_to_system)
			  {
			    /* Prevent system from acting on keyup (which opens the
			       Start menu if no other key was pressed) by simulating a
			       press of Space which we will ignore.  */
			    if GetAsyncKeyState (wParam as c_int) as c_int & 1 != 0
			      {
				if FIXNUMP (Vw32_phantom_key_code)
				  { key = XUFIXNUM (Vw32_phantom_key_code) as c_int & 255; }
				else
				  { key = VK_SPACE; }
				(*dpyinfo).faked_key = key;
				keybd_event (key as BYTE,
					     MapVirtualKey (key as UINT, 0) as BYTE, 0, 0);
			      }
			  }
			if !NILP (Vw32_lwindow_modifier)
			  { return 0; }
		      }
		    VK_RWIN =>
		      {
			if !w32_kbdhook_active! () && NILP (Vw32_pass_rwindow_to_system)
			  {
			    if GetAsyncKeyState (wParam as c_int) as c_int & 1 != 0
			      {
				if FIXNUMP (Vw32_phantom_key_code)
				  { key = XUFIXNUM (Vw32_phantom_key_code) as c_int & 255; }
				else
				  { key = VK_SPACE; }
				(*dpyinfo).faked_key = key;
				keybd_event (key as BYTE,
					     MapVirtualKey (key as UINT, 0) as BYTE, 0, 0);
			      }
			  }
			if !NILP (Vw32_rwindow_modifier)
			  { return 0; }
		      }
		    VK_APPS =>
		      {
			if !NILP (Vw32_apps_modifier)
			  { return 0; }
		      }
		    VK_MENU =>
		      {
			if NILP (Vw32_pass_alt_to_system)
			  /* Prevent DefWindowProc from activating the menu bar if an
			     Alt key is pressed and released by itself.  */
			  { return 0; }
			windows_translate = 1;
		      }
		    VK_CAPITAL =>
		      {
			/* Decide whether to treat as modifier or function key.  */
			if NILP (Vw32_enable_caps_lock)
			  /* C's `goto disable_lock_key`.  */
			  { disable_lock_key = true; break 'wparam; }
			windows_translate = 1;
		      }
		    VK_NUMLOCK =>
		      {
			/* Decide whether to treat as modifier or function key.  */
			if NILP (Vw32_enable_num_lock)
			  { disable_lock_key = true; break 'wparam; }
			windows_translate = 1;
		      }
		    VK_SCROLL =>
		      {
			/* Decide whether to treat as modifier or function key.  */
			if NILP (Vw32_scroll_lock_modifier)
			  { disable_lock_key = true; break 'wparam; }
			windows_translate = 1;
		      }
		    VK_CONTROL | VK_SHIFT | VK_PROCESSKEY =>
		      {
			/* VK_PROCESSKEY is generated by IME.  */
			windows_translate = 1;
		      }
		    VK_CANCEL =>
		      {
			/* Windows maps Ctrl-Pause (aka Ctrl-Break) into VK_CANCEL,
			   which is confusing for purposes of key binding; convert
			   VK_CANCEL events into VK_PAUSE events.  */
			wParam = VK_PAUSE as WPARAM;
		      }
		    VK_PAUSE =>
		      {
			/* Windows maps Ctrl-NumLock into VK_PAUSE, which is confusing
			   for purposes of key binding; convert these back into
			   VK_NUMLOCK events, at least when we want to see NumLock key
			   presses.  (Note that there is never any possibility that
			   VK_PAUSE with Ctrl really is C-Pause as per above.)  */
			if NILP (Vw32_enable_num_lock) && modifier_set (VK_CONTROL) != 0
			  { wParam = VK_NUMLOCK as WPARAM; }
		      }
		    _ =>
		      {
			let mut break_out: bool = false;

			if w32_unicode_gui != 0 && !w32_use_fallback_wm_chars_method
			  {
			    /* If this event generates characters or deadkeys, do
			       not interpret it as a "raw combination of modifiers
			       and keysym".  Hide deadkeys, and use the generated
			       character(s) instead of the keysym.  (Backward
			       compatibility: exceptions for numpad keys generating
			       0-9 . , / * - +, and for extra-Alt combined with a
			       non-Latin char.)

			       Try to not report modifiers which have effect on
			       which character or deadkey is generated.

			       Example (contrived): if rightAlt-? generates f (on a
			       Cyrillic keyboard layout), and Ctrl, leftAlt do not
			       affect the generated character, one wants to report
			       Ctrl-leftAlt-f if the user presses
			       Ctrl-leftAlt-rightAlt-?.  */
			    let res: c_int;

			    /* C 4852-4867 is `#if 0` - C's own dead text (the commented-out
			       `deliver_wm_chars (0, …)` call and its five-line comment),
			       which the port records instead of carrying (the
			       `character.rs` 118 `#if 0` reading; C's own `#if 0` never
			       compiles in any configuration).  */
			    /* Processing the generated WM_CHAR messages *WHILE* we
			       handle KEYDOWN/UP event is the best choice, since
			       without any fuss, we know all 3 of: scancode, virtual
			       keycode, and expansion.  (Additionally, one knows
			       boundaries of expansion of different keypresses.)  */
			    res = deliver_wm_chars (1, hwnd, msg, wParam as UINT, lParam as UINT, 1);
			    windows_translate = -((res != 0) as c_int);
			    if res > 0 /* Bound to character(s) or a deadkey */
			      /* C's `break` out of the nested switch.  */
			      { break_out = true; }
			    /* deliver_wm_chars may make some branches after this vestigal.  */
			  }
			if !break_out
			  {
			    wParam = map_keypad_keys (wParam as c_uint,
						      (if lParam & 0x1000000 != 0 { 1 } else { 0 })
						      ) as WPARAM;
			    /* If not defined as a function key, change it to a WM_CHAR message.  */
			    if wParam > 255 || lispy_function_keys[wParam as usize].is_null ()
			      {
				let modifiers_: DWORD = w32_construct_console_modifiers () as DWORD;

				if !NILP (Vw32_recognize_altgr)
				    && modifier_set (VK_LCONTROL) != 0
				    && modifier_set (VK_RMENU) != 0
				  {
				    /* Always let TranslateMessage handle AltGr key chords;
				       for some reason, ToAscii doesn't always process AltGr
				       chords correctly.  */
				    windows_translate = 1;
				  }
				else if (modifiers_
					 & (!(SHIFT_PRESSED as DWORD)
					    & !(CAPSLOCK_ON as DWORD))) != 0
				  {
				    /* Handle key chords including any modifiers other
				       than shift directly, in order to preserve as much
				       modifier information as possible.  */
				    if 'A' as WPARAM <= wParam && wParam <= 'Z' as WPARAM
				      {
					/* Don't translate modified alphabetic keystrokes,
					   so the user doesn't need to constantly switch
					   layout to type control or meta keystrokes when
					   the normal layout translates alphabetic
					   characters to non-ascii characters.  */
					if modifier_set (VK_SHIFT) == 0
					  { wParam += ('a' as WPARAM - 'A' as WPARAM); }
					msg = WM_CHAR as UINT;
				      }
				    else
				      {
					/* Try to handle other keystrokes by determining the
					   base character (ie. translating the base key plus
					   shift modifier).  */
					let mut add: c_int;
					let mut key_record: KEY_EVENT_RECORD = core::mem::zeroed ();

					key_record.bKeyDown = TRUE;
					key_record.wRepeatCount = 1;
					key_record.wVirtualKeyCode = wParam as WORD;
					key_record.wVirtualScanCode = ((lParam & 0xFF0000) >> 16) as WORD;
					key_record.uChar.AsciiChar = 0;
					key_record.dwControlKeyState = modifiers_;

					add = w32_kbd_patch_key (&raw mut key_record,
								 w32_keyboard_codepage);
					/* 0 means an unrecognized keycode, negative means
					   dead key.  Ignore both.  */
					/* C's `while (--add >= 0)`: the pre-decrement runs on the
					   test, and the loop runs while the new value is ≥ 0.  */
					loop
					  {
					    add -= 1;
					    if add < 0 { break; }
					    /* Forward asciified character sequence.  */
					    post_character_message
					      (hwnd, WM_CHAR as UINT,
					       key_record.uChar.AsciiChar as u8 as WPARAM, lParam,
					       w32_get_key_modifiers (wParam as c_uint,
								      lParam as c_uint) as DWORD);
					    w32_kbd_patch_key (&raw mut key_record,
							       w32_keyboard_codepage);
					  }
					return 0;
				      }
				  }
				else
				  {
				    /* Let TranslateMessage handle everything else.  */
				    windows_translate = 1;
				  }
			      }
			  }
		      }
		    }
		}

		if disable_lock_key
		  {
		    /* C: `disable_lock_key:` - "Ensure the appropriate lock key state (and indicator
		       light) remains in the same state. We do this by faking another press of the
		       relevant key.  Apparently, this really is the only way to toggle the state of
		       the indicator lights."  */
		    (*dpyinfo).faked_key = wParam as c_int;
		    keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
		    keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | 0, 0);
		    keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
				 KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
		    /* Ensure indicator lights are updated promptly on Windows 9x
		       (TranslateMessage apparently does this), after forwarding
		       input event.  */
		    post_character_message (hwnd, msg, wParam, lParam,
					    w32_get_key_modifiers (wParam as c_uint,
								   lParam as c_uint) as DWORD);
		    windows_translate = 1;
		  }
	      }

	      if windows_translate == -1
		{ break 'dispatch; }
	    }

	  /* C: `translate:` (C 4869).  C's `MSG windows_msg = { hwnd, msg, wParam, lParam, 0,
	     {0,0} };` is the four field writes plus the zeroed tail.  */
	  if windows_translate != 0
	    {
	      let mut windows_msg: MSG = core::mem::zeroed ();

	      windows_msg.hwnd = hwnd;
	      windows_msg.message = msg;
	      windows_msg.wParam = wParam;
	      windows_msg.lParam = lParam;

	      windows_msg.time = GetMessageTime () as DWORD;
	      TranslateMessage (&raw const windows_msg);
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }
	  /* C's `FALLTHROUGH` into `case WM_SYSCHAR: case WM_CHAR:`'s body (C 4879-4898), which
	     the port writes here in C's own statements.  */
	  if wParam > 255
	    {
	      let mut wmsg: W32Msg = core::mem::zeroed ();

	      wmsg.dwModifiers = w32_get_key_modifiers (wParam as c_uint, lParam as c_uint) as DWORD;
	      signal_user_input ();
	      my_post_msg (&raw mut wmsg, hwnd, WM_UNICHAR as UINT, wParam, lParam);
	    }
	  else
	    { post_character_message (hwnd, msg, wParam, lParam,
				      w32_get_key_modifiers (wParam as c_uint,
							     lParam as c_uint) as DWORD); }
	  break 'dispatch;
	}

      WM_SYSCHAR | WM_CHAR =>
	{
	  if wParam > 255
	    {
	      let mut wmsg: W32Msg = core::mem::zeroed ();

	      wmsg.dwModifiers = w32_get_key_modifiers (wParam as c_uint, lParam as c_uint) as DWORD;
	      signal_user_input ();
	      my_post_msg (&raw mut wmsg, hwnd, WM_UNICHAR as UINT, wParam, lParam);
	    }
	  else
	    { post_character_message (hwnd, msg, wParam, lParam,
				      w32_get_key_modifiers (wParam as c_uint,
							     lParam as c_uint) as DWORD); }
	  break 'dispatch;
	}
      WM_UNICHAR =>
	{
	  /* WM_UNICHAR looks promising from the docs, but the exact
	     circumstances in which TranslateMessage sends it is one of those
	     Microsoft secret API things that EU and US courts are supposed
	     to have put a stop to already. Spy++ shows it being sent to Notepad
	     and other MS apps, but never to Emacs.

	     Some third party IMEs send it in accordance with the official
	     documentation though, so handle it here.

	     UNICODE_NOCHAR is used to test for support for this message.
	     TRUE indicates that the message is supported.  */
	  if wParam == UNICODE_NOCHAR as WPARAM
	    { return TRUE as LRESULT; }

	  {
	    let mut wmsg: W32Msg = core::mem::zeroed ();
	    wmsg.dwModifiers = w32_get_key_modifiers (wParam as c_uint, lParam as c_uint) as DWORD;
	    signal_user_input ();
	    my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  }
	  break 'dispatch;
	}
      WM_IME_CHAR =>
	{
	  /* If we can't get the IME result as Unicode, use default processing,
	     which will at least allow characters decodable in the system locale
	     get through.  */
	  if get_composition_string_fn.is_none ()
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }
	  else if ignore_ime_char == 0
	    {
	      /* C 5039's `wchar_t * buffer;` is the Windows 16-bit wide character, which this unit
		 spells `c_ushort` (the `process_dropfiles` `macro-body-notes.tsv` row's reading: the
		 tree's `wchar_t` is the host's 4-byte type).  */
	      let mut buffer: *mut c_ushort;
	      let mut size: c_int;
	      let mut i: c_int;
	      let mut wmsg: W32Msg = core::mem::zeroed ();
	      let context: HIMC = (get_ime_context_fn.unwrap ()) (hwnd);
	      wmsg.dwModifiers =
		if w32_ignore_modifiers_on_IME_input
		  { 0 }
		else
		  { w32_get_key_modifiers (wParam as c_uint, lParam as c_uint) as DWORD };
	      /* Get buffer size.  */
	      size = (get_composition_string_fn.unwrap ())
		(context, GCS_RESULTSTR, core::ptr::null_mut (), 0) as c_int;
	      buffer = sa_stack_alloc (size as usize) as *mut c_ushort;
	      size = (get_composition_string_fn.unwrap ())
		(context, GCS_RESULTSTR, buffer as *mut c_void, size as DWORD) as c_int;
	      (release_ime_context_fn.unwrap ()) (hwnd, context);

	      signal_user_input ();
	      i = 0;
	      while i < size / core::mem::size_of::<c_ushort> () as c_int
		{
		  my_post_msg (&raw mut wmsg, hwnd, WM_UNICHAR as UINT,
			       *buffer.offset (i as isize) as WPARAM, lParam);
		  i += 1;
		}
	      /* Ignore the messages for the rest of the
		 characters in the string that was output above.  */
	      ignore_ime_char = size / core::mem::size_of::<c_ushort> () as c_int - 1;
	    }
	  else
	    { ignore_ime_char -= 1; }

	  break 'dispatch;
	}
      WM_IME_STARTCOMPOSITION =>
	{
	  if set_ime_composition_window_fn.is_none ()
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }
	  else
	    {
	      let mut form: COMPOSITIONFORM = core::mem::zeroed ();
	      let mut lf: LOGFONTW = core::mem::zeroed ();
	      let mut context: HIMC;
	      let mut w: *mut window;

	      /* Implementation note: The code below does something that
		 one shouldn't do: it accesses the window object from a
		 separate thread, while the main (a.k.a. "Lisp") thread
		 runs and can legitimately delete and even GC it.  That is
		 why we are extra careful not to futz with a window that
		 is different from the one recorded when the system caret
		 coordinates were last modified.  That is also why we are
		 careful not to move the IME window if the window
		 described by W was deleted, as indicated by its buffer
		 field being reset to nil.  */
	      f = w32_window_to_frame (dpyinfo, hwnd);
	      if !(!f.is_null () && FRAME_LIVE_P (f))
		{
		  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
		}
	      w = XWINDOW (FRAME_SELECTED_WINDOW (f));
	      /* Punt if someone changed the frame's selected window
		 behind our back. */
	      if w != w32_system_caret_window as *mut window
		{
		  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
		}

	      form.dwStyle = CFS_RECT;
	      form.ptCurrentPos.x = w32_system_caret_x;
	      form.ptCurrentPos.y = w32_system_caret_y;

	      form.rcArea.left = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, 0);
	      form.rcArea.top = (WINDOW_TOP_EDGE_Y! (w)
				 + w32_system_caret_hdr_height);
	      form.rcArea.right = (WINDOW_BOX_RIGHT_EDGE_X! (w)
				   - WINDOW_RIGHT_MARGIN_WIDTH! (w)
				   - WINDOW_RIGHT_FRINGE_WIDTH! (w));
	      form.rcArea.bottom = (WINDOW_BOTTOM_EDGE_Y! (w)
				    - WINDOW_BOTTOM_DIVIDER_WIDTH! (w)
				    - w32_system_caret_mode_height);

	      /* Punt if the window was deleted behind our back.  */
	      if !BUFFERP ((*w).contents)
		{
		  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
		}

	      context = (get_ime_context_fn.unwrap ()) (hwnd);

	      if context.is_null ()
		{
		  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
		}

	      GetObjectW (FONT_HANDLE! (w32_system_remap_font), core::mem::size_of_val (&lf) as c_int,
			  &raw mut lf as *mut c_void);
	      (set_ime_composition_font_fn.unwrap ()) (context, &raw mut lf as *mut c_void);
	      (set_ime_composition_window_fn.unwrap ()) (context, &raw mut form as *mut c_void);
	      (release_ime_context_fn.unwrap ()) (hwnd, context);
	    }
	  /* FIXME: somehow "goto dflt" here instead of "break" causes
	     popup dialogs, such as the ones shown by File->Open File and
	     w32-select-font, to become hidden behind their parent frame,
	     when focus-follows-mouse is in effect.  See bug#11732.  But
	     if we don't "goto dflt", users of IME cannot type text
	     supported by the input method...  */
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_IME_ENDCOMPOSITION =>
	{
	  ignore_ime_char = 0;
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      /* Simulate middle mouse button events when left and right buttons
	 are used together, but only if user has two button mouse. */
      WM_LBUTTONDOWN | WM_RBUTTONDOWN =>
	{
	  /* Ignore mouse events produced by a touch screen.  */
	  /* C 5113-5115: `#ifndef MOUSEEVENTF_FROMTOUCH / #define MOUSEEVENTF_FROMTOUCH
	     0xFF515700 / #endif` (the constant is above).  */
	  if GetMessageExtraInfo () as DWORD & MOUSEEVENTF_FROMTOUCH != 0
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }

	  if w32_num_mouse_buttons > 2
	    { handle_plain_button = true; }	/* C's `goto handle_plain_button`.  */
	  else
	    {
	      let this: c_int = if msg as c_int == WM_LBUTTONDOWN { LMOUSE } else { RMOUSE };
	      let other: c_int = if msg as c_int == WM_LBUTTONDOWN { RMOUSE } else { LMOUSE };

	      if button_state & this != 0
		{ return 0; }

	      /* Don't capture mouse when dropping.  */
	      if button_state == 0 && !EQ (track_mouse, Qdropping ())
		{ SetCapture (hwnd); }

	      button_state |= this;

	      if button_state & other != 0
		{
		  if mouse_button_timer != 0
		    {
		      KillTimer (hwnd, mouse_button_timer as UINT_PTR);
		      mouse_button_timer = 0;

		      /* Generate middle mouse event instead. */
		      msg = WM_MBUTTONDOWN as UINT;
		      button_state |= MMOUSE;
		    }
		  else if button_state & MMOUSE != 0
		    {
		      /* Ignore button event if we've already generated a
			 middle mouse down event.  This happens if the
			 user releases and press one of the two buttons
			 after we've faked a middle mouse event. */
		      return 0;
		    }
		  else
		    {
		      /* Flush out saved message. */
		      post_msg (&raw mut saved_mouse_button_msg);
		    }
		  wmsg.dwModifiers = w32_get_modifiers ();
		  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
		  signal_user_input ();

		  /* Clear message buffer. */
		  saved_mouse_button_msg.msg.hwnd = core::ptr::null_mut ();
		}
	      else
		{
		  /* Hold onto message for now. */
		  mouse_button_timer =
		    SetTimer (hwnd, MOUSE_BUTTON_ID as UINT_PTR,
			      w32_mouse_button_tolerance as UINT, core::ptr::null_mut ()) as c_uint;
		  saved_mouse_button_msg.msg.hwnd = hwnd;
		  saved_mouse_button_msg.msg.message = msg;
		  saved_mouse_button_msg.msg.wParam = wParam;
		  saved_mouse_button_msg.msg.lParam = lParam;
		  saved_mouse_button_msg.msg.time = GetMessageTime () as DWORD;
		  saved_mouse_button_msg.dwModifiers = w32_get_modifiers ();
		}
	    }
	  return 0;
	}
      WM_LBUTTONUP | WM_RBUTTONUP =>
	{
	  /* Ignore mouse events produced by a touch screen.  */
	  /* C 5185-5187: the same `#ifndef MOUSEEVENTF_FROMTOUCH` guard.  */
	  if GetMessageExtraInfo () as DWORD & MOUSEEVENTF_FROMTOUCH != 0
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }

	  if w32_num_mouse_buttons > 2
	    { handle_plain_button = true; }	/* C's `goto handle_plain_button`.  */
	  else
	    {
	      let this: c_int = if msg as c_int == WM_LBUTTONUP { LMOUSE } else { RMOUSE };
	      let other: c_int = if msg as c_int == WM_LBUTTONUP { RMOUSE } else { LMOUSE };

	      if (button_state & this) == 0
		{ return 0; }

	      button_state &= !this;

	      if button_state & MMOUSE != 0
		{
		  /* Only generate event when second button is released. */
		  if (button_state & other) == 0
		    {
		      msg = WM_MBUTTONUP as UINT;
		      button_state &= !MMOUSE;

		      if button_state != 0 { emacs_abort (); }
		    }
		  else
		    { return 0; }
		}
	      else
		{
		  /* Flush out saved message if necessary. */
		  if !saved_mouse_button_msg.msg.hwnd.is_null ()
		    {
		      post_msg (&raw mut saved_mouse_button_msg);
		    }
		}
	      wmsg.dwModifiers = w32_get_modifiers ();
	      my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	      signal_user_input ();

	      /* Always clear message buffer and cancel timer. */
	      saved_mouse_button_msg.msg.hwnd = core::ptr::null_mut ();
	      KillTimer (hwnd, mouse_button_timer as UINT_PTR);
	      mouse_button_timer = 0;

	      if button_state == 0
		{ ReleaseCapture (); }
	    }
	  return 0;
	}
      WM_XBUTTONDOWN | WM_XBUTTONUP =>
	{
	  if w32_pass_extra_mouse_buttons_to_system
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }
	  /* else fall through and process them.  */
	  handle_plain_button = true;			/* C's `FALLTHROUGH` into `handle_plain_button`.  */
	}
      WM_MBUTTONDOWN | WM_MBUTTONUP =>
	{
	  handle_plain_button = true;			/* C: the two `case`s label `handle_plain_button`.  */
	}
      WM_MOUSEMOVE | WM_HSCROLL | WM_VSCROLL =>
	{
	  if msg as c_int == WM_MOUSEMOVE
	    {
	      f = w32_window_to_frame (dpyinfo, hwnd);
	      if !f.is_null ()
		{
		  /* Ignore mouse movements as long as the menu is active.
		     These movements are processed by the window manager
		     anyway, and it's wrong to handle them as if they happened
		     on the underlying frame.  */
		  if (*( (*f).output_data.w32 as *mut w32_output )).menubar_active != 0
		    { return 0; }

		  /* If the mouse moved, and the mouse pointer is invisible,
		     make it visible again.  We do this here so as to be able
		     to show the mouse pointer even when the main
		     (a.k.a. "Lisp") thread is busy doing something.  */
		  /* C 5328: C's function-local `static int last_x, last_y;` are the two
		     file-scope `last_x`/`last_y` above (the `force_quit_count` reading).  */
		  let x: c_int = GET_X_LPARAM! (lParam);
		  let y: c_int = GET_Y_LPARAM! (lParam);

		  if pointer_invisible! (f) && (x != last_x || y != last_y)
		    { pointer_invisible! (f, false); }

		  last_x = x;
		  last_y = y;
		}

	      /* If the mouse has just moved into the frame, start tracking
		 it, so we will be notified when it leaves the frame.  Mouse
		 tracking only works under W98 and NT4 and later. On earlier
		 versions, there is no way of telling when the mouse leaves the
		 frame, so we just have to put up with help-echo and mouse
		 highlighting remaining while the frame is not active.  */
	      if track_mouse_event_fn.is_some () && track_mouse_window.is_null ()
		  /* If the menu bar is active, turning on tracking of mouse
		     movement events might send these events to the tooltip
		     frame, if the user happens to move the mouse pointer over
		     the tooltip.  But since we don't process events for
		     tooltip frames, this causes Windows to present a
		     hourglass cursor, which is ugly and unexpected.  So don't
		     enable tracking mouse events in this case; they will be
		     restarted when the menu pops down.  (Confusingly, the
		     menubar_active member of f->output_data.w32, tested
		     above, is only set when a menu was popped up _not_ from
		     the frame's menu bar, but via x-popup-menu.)  */
		  && menubar_in_use == 0
		{
		  let mut tme: TRACKMOUSEEVENT = core::mem::zeroed ();
		  tme.cbSize = core::mem::size_of_val (&tme) as DWORD;
		  tme.dwFlags = TME_LEAVE as DWORD;
		  tme.hwndTrack = hwnd;
		  tme.dwHoverTime = HOVER_DEFAULT;

		  (track_mouse_event_fn.unwrap ()) (&raw mut tme as *mut c_void);
		  track_mouse_window = hwnd;
		}
	      /* C's `FALLTHROUGH` into `case WM_HSCROLL: case WM_VSCROLL:` (C 5349).  */
	    }
	  if w32_mouse_move_interval <= 0
	      || (msg as c_int == WM_MOUSEMOVE && button_state == 0)
	    {
	      wmsg.dwModifiers = w32_get_modifiers ();
	      my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	      return 0;
	    }

	  /* Hang onto mouse move and scroll messages for a bit, to avoid
	     sending such events to Emacs faster than it can process them.
	     If we get more events before the timer from the first message
	     expires, we just replace the first message. */

	  if saved_mouse_move_msg.msg.hwnd.is_null ()
	    {
	      mouse_move_timer =
		SetTimer (hwnd, MOUSE_MOVE_ID as UINT_PTR, w32_mouse_move_interval as UINT,
			  core::ptr::null_mut ()) as c_uint;
	    }

	  /* Hold onto message for now. */
	  saved_mouse_move_msg.msg.hwnd = hwnd;
	  saved_mouse_move_msg.msg.message = msg;
	  saved_mouse_move_msg.msg.wParam = wParam;
	  saved_mouse_move_msg.msg.lParam = lParam;
	  saved_mouse_move_msg.msg.time = GetMessageTime () as DWORD;
	  saved_mouse_move_msg.dwModifiers = w32_get_modifiers ();

	  return 0;
	}
      WM_MOUSEWHEEL =>
	{
	  wmsg.dwModifiers = w32_get_modifiers ();
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  signal_user_input ();
	  return 0;
	}
      WM_APPCOMMAND | WM_MOUSEHWHEEL =>
	{
	  if msg as c_int == WM_APPCOMMAND && w32_pass_multimedia_buttons_to_system
	    {
	      return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
		(hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	    }
	  /* Otherwise, pass to lisp, the same way we do with mousehwheel.  */
	  /* C's `FALLTHROUGH` into `case WM_MOUSEHWHEEL:` is the body below; C 5369-5371's
	     `FIXME!!!  This is never reached so what's the purpose?  If the non-zero return remark
	     below is right we're doing it wrong all the time.` is C's own note.  */
	  wmsg.dwModifiers = w32_get_modifiers ();
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  signal_user_input ();
	  /* Non-zero must be returned when WM_MOUSEHWHEEL messages are
	     handled, to prevent the system trying to handle it by faking
	     scroll bar events.  */
	  return 1;
	}
      WM_TIMER =>
	{
	  /* Flush out saved messages if necessary. */
	  if wParam as c_uint == mouse_button_timer
	    {
	      if !saved_mouse_button_msg.msg.hwnd.is_null ()
		{
		  post_msg (&raw mut saved_mouse_button_msg);
		  signal_user_input ();
		  saved_mouse_button_msg.msg.hwnd = core::ptr::null_mut ();
		}
	      KillTimer (hwnd, mouse_button_timer as UINT_PTR);
	      mouse_button_timer = 0;
	    }
	  else if wParam as c_uint == mouse_move_timer
	    {
	      if !saved_mouse_move_msg.msg.hwnd.is_null ()
		{
		  post_msg (&raw mut saved_mouse_move_msg);
		  saved_mouse_move_msg.msg.hwnd = core::ptr::null_mut ();
		}
	      KillTimer (hwnd, mouse_move_timer as UINT_PTR);
	      mouse_move_timer = 0;
	    }
	  else if wParam as c_uint == menu_free_timer
	    {
	      KillTimer (hwnd, menu_free_timer as UINT_PTR);
	      menu_free_timer = 0;
	      f = w32_window_to_frame (dpyinfo, hwnd);
	      /* If a popup menu is active, don't wipe its strings.  */
	      if menubar_in_use != 0
		  && current_popup_menu.is_null ()
		{
		  /* Free memory used by owner-drawn and help-echo strings.  */
		  w32_free_menu_strings (hwnd);
		  if !f.is_null ()
		    { (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 0; }
		  menubar_in_use = 0;
		}
	    }
	  return 0;
	}
      WM_NCACTIVATE =>
	{
	  /* Windows doesn't send us focus messages when putting up and
	     taking down a system popup dialog as for Ctrl-Alt-Del on Windows 95.
	     The only indication we get that something happened is receiving
	     this message afterwards.  So this is a good time to reset our
	     keyboard modifiers' state. */
	  reset_modifiers ();
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_INITMENU =>
	{
	  button_state = 0;
	  ReleaseCapture ();
	  /* We must ensure menu bar is fully constructed and up to date
	     before allowing user interaction with it.  To achieve this
	     we send this message to the lisp thread and wait for a
	     reply (whose value is not actually needed) to indicate that
	     the menu bar is now ready for use, so we can now return.

	     To remain responsive in the meantime, we enter a nested message
	     loop that can process all other messages.

	     However, we skip all this if the message results from calling
	     TrackPopupMenu - in fact, we must NOT attempt to send the lisp
	     thread a message because it is blocked on us at this point.  We
	     set menubar_active before calling TrackPopupMenu to indicate
	     this (there is no possibility of confusion with real menubar
	     being active).  */

	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null ()
	      && ((*( (*f).output_data.w32 as *mut w32_output )).menubar_active != 0
		  /* We can receive this message even in the absence of a
		     menubar (ie. when the system menu is activated) - in this
		     case we do NOT want to forward the message, otherwise it
		     will cause the menubar to suddenly appear when the user
		     had requested it to be turned off!  */
		  || (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).menubar_widget.is_null ())
	    { return 0; }

	  {
	    let mut msg_buf: deferred_msg = core::mem::zeroed ();

	    /* Detect if message has already been deferred; in this case
	       we cannot return any sensible value to ignore this.  */
	    if !find_deferred_msg (hwnd, msg).is_null ()
	      { emacs_abort (); }

	    menubar_in_use = 1;

	    return send_deferred_msg (&raw mut msg_buf, hwnd, msg, wParam, lParam);
	  }
	}
      WM_EXITMENULOOP =>
	{
	  f = w32_window_to_frame (dpyinfo, hwnd);

	  /* If a menu is still active, check again after a short delay,
	     since Windows often (always?) sends the WM_EXITMENULOOP
	     before the corresponding WM_COMMAND message.
	     Don't do this if a popup menu is active, since it is only
	     menubar menus that require cleaning up in this way.
	  */
	  if !f.is_null () && menubar_in_use != 0 && current_popup_menu.is_null ()
	    {
	      menu_free_timer = SetTimer (hwnd, MENU_FREE_ID as UINT_PTR,
					  MENU_FREE_DELAY as UINT,
					  core::ptr::null_mut ()) as c_uint;
	    }

	  /* If hourglass cursor should be displayed, display it now.  */
	  if !f.is_null () && (*( (*f).output_data.w32 as *mut w32_output )).hourglass_p != 0
	    { SetCursor ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).hourglass_cursor); }

	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_MENUSELECT =>
	{
	  /* Direct handling of help_echo in menus.  Should be safe now
	     that we generate the help_echo by placing a help event in the
	     keyboard buffer.  */
	  {
	    let menu: HMENU = lParam as HMENU;
	    let menu_item: UINT = LOWORD! (wParam) as UINT;
	    let flags: UINT = HIWORD! (wParam) as UINT;

	    w32_menu_display_help (hwnd, menu, menu_item, flags);
	  }
	  return 0;
	}
      WM_MEASUREITEM =>
	{
	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null ()
	    {
	      let pMis: *mut MEASUREITEMSTRUCT = lParam as *mut MEASUREITEMSTRUCT;

	      if (*pMis).CtlType == ODT_MENU
		{
		  /* Work out dimensions for popup menu titles. */
		  let title: *mut c_char = (*pMis).itemData as *mut c_char;
		  let hdc: HDC = GetDC (hwnd);
		  let mut menu_font: HFONT = GetCurrentObject (hdc, OBJ_FONT as UINT) as HFONT;
		  let mut menu_logfont: LOGFONT = core::mem::zeroed ();
		  let old_font: HFONT;
		  let mut size: SIZE = core::mem::zeroed ();

		  GetObject (menu_font as HANDLE, core::mem::size_of_val (&menu_logfont) as c_int,
			     &raw mut menu_logfont as *mut c_void);
		  menu_logfont.lfWeight = FW_BOLD;
		  menu_font = CreateFontIndirect (&raw const menu_logfont);
		  old_font = SelectObject (hdc, menu_font as HGDIOBJ) as HFONT;

		  (*pMis).itemHeight = GetSystemMetrics (SM_CYMENUSIZE) as UINT;
		  if !title.is_null ()
		    {
		      if unicode_append_menu.is_some ()
			{
			  GetTextExtentPoint32W (hdc, title as *const WCHAR,
						 wcslen (title as *const WCHAR) as c_int,
						 &raw mut size);
			}
		      else
			{
			  GetTextExtentPoint32 (hdc, title, strlen (title) as c_int,
						&raw mut size);
			}

		      (*pMis).itemWidth = size.cx as UINT;
		      if (*pMis).itemHeight < size.cy as UINT
			{ (*pMis).itemHeight = size.cy as UINT; }
		    }
		  else
		    { (*pMis).itemWidth = 0; }

		  SelectObject (hdc, old_font as HGDIOBJ);
		  DeleteObject (menu_font as HGDIOBJ);
		  ReleaseDC (hwnd, hdc);
		  return TRUE as LRESULT;
		}
	    }
	  return 0;
	}
      WM_DRAWITEM =>
	{
	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null ()
	    {
	      let pDis: *mut DRAWITEMSTRUCT = lParam as *mut DRAWITEMSTRUCT;

	      if (*pDis).CtlType == ODT_MENU
		{
		  /* Draw popup menu title. */
		  let title: *mut c_char = (*pDis).itemData as *mut c_char;
		  if !title.is_null ()
		    {
		      let hdc: HDC = (*pDis).hDC;
		      let mut menu_font: HFONT = GetCurrentObject (hdc, OBJ_FONT as UINT) as HFONT;
		      let mut menu_logfont: LOGFONT = core::mem::zeroed ();
		      let old_font: HFONT;

		      GetObject (menu_font as HANDLE, core::mem::size_of_val (&menu_logfont) as c_int,
				 &raw mut menu_logfont as *mut c_void);
		      menu_logfont.lfWeight = FW_BOLD;
		      menu_font = CreateFontIndirect (&raw const menu_logfont);
		      old_font = SelectObject (hdc, menu_font as HGDIOBJ) as HFONT;

		      /* Always draw title as if not selected.  */
		      if unicode_append_menu.is_some ()
			{
			  ExtTextOutW (hdc,
				       (*pDis).rcItem.left
				       + GetSystemMetrics (SM_CXMENUCHECK),
				       (*pDis).rcItem.top,
				       ETO_OPAQUE, &raw const (*pDis).rcItem,
				       title as *const WCHAR,
				       wcslen (title as *const WCHAR) as UINT, core::ptr::null ());
			}
		      else
			{
			  ExtTextOut (hdc,
				      (*pDis).rcItem.left
				      + GetSystemMetrics (SM_CXMENUCHECK),
				      (*pDis).rcItem.top,
				      ETO_OPAQUE, &raw const (*pDis).rcItem,
				      title, strlen (title) as UINT, core::ptr::null ());
			}

		      SelectObject (hdc, old_font as HGDIOBJ);
		      DeleteObject (menu_font as HGDIOBJ);
		    }
		  return TRUE as LRESULT;
		}
	    }
	  return 0;
	}
      WM_MOUSEACTIVATE =>
	{
	  /* WM_MOUSEACTIVATE is the only way on Windows to implement the
	     `no-accept-focus' frame parameter.  This means that one can't
	     use the mouse to scroll a window on a non-selected frame.  */

	  /* Still not right - can't distinguish between clicks in the
	     client area of the frame from clicks forwarded from the scroll
	     bars - may have to hook WM_NCHITTEST to remember the mouse
	     position and then check if it is in the client area
	     ourselves.  */

	  /* Discard the mouse click that activates a frame, allowing the
	     user to click anywhere without changing point (or worse!).
	     Don't eat mouse clicks on scrollbars though!!  */

	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null ()
	      && FRAME_NO_ACCEPT_FOCUS (f) != 0
	      /* Ignore child frames, they don't accept focus anyway.  */
	      && FRAME_PARENT_FRAME (f).is_null ()
	    {
	      let mut frame: Lisp_Object = Qnil ();

	      XSETFRAME! (frame, f);
	      if !EQ (selected_frame, frame)
		/* Don't discard the message, GTK doesn't either.  */
		{ return MA_NOACTIVATE; } /* ANDEAT; */
	    }
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_MOUSELEAVE | WM_ACTIVATEAPP | WM_ACTIVATE | WM_WINDOWPOSCHANGED | WM_SHOWWINDOW =>
	{
	  /* C's `case WM_MOUSELEAVE:` sets the tracker and `FALLTHROUGH`s into this body.  */
	  if msg as c_int == WM_MOUSELEAVE
	    {
	      /* No longer tracking mouse.  */
	      track_mouse_window = core::ptr::null_mut ();
	    }

	  /* Inform lisp thread that a frame might have just been obscured
	     or exposed, so should recheck visibility of all frames.  */
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_SETTINGCHANGE =>
	{
	  /* Inform the Lisp thread that some system-wide setting has
	     changed, so if Emacs is interested in some of them, it could
	     update its internal values.  */
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_SETFOCUS =>
	{
	  (*dpyinfo).faked_key = 0;
	  reset_modifiers ();
	  if !w32_kbdhook_active! ()
	    { register_hot_keys (hwnd); }
	  command = true;		/* C's `goto command`.  */
	}
      WM_KILLFOCUS =>
	{
	  if !w32_kbdhook_active! ()
	    { unregister_hot_keys (hwnd); }
	  button_state = 0;
	  ReleaseCapture ();
	  /* Relinquish the system caret.  */
	  if !w32_system_caret_hwnd.is_null ()
	    {
	      w32_visible_system_caret_hwnd = core::ptr::null_mut ();
	      w32_system_caret_hwnd = core::ptr::null_mut ();
	      DestroyCaret ();
	    }
	  command = true;		/* C's `goto command`.  */
	}
      WM_COMMAND =>
	{
	  menubar_in_use = 0;
	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null () && HIWORD! (wParam) == 0
	    {
	      if menu_free_timer != 0
		{
		  KillTimer (hwnd, menu_free_timer as UINT_PTR);
		  menu_free_timer = 0;
		}
	    }
	  command = true;		/* C's `FALLTHROUGH` into `case WM_MOVE:`.  */
	}
      WM_MOVE | WM_SIZE =>
	{
	  command = true;		/* C: the two `case`s label `command:`.  */
	}
      WM_TOUCH =>
	{
	  /* C 5712-5719's `#ifdef WM_TOUCHMOVE` arm is False for every configuration of this file
	     (the macro is `w32term.c` 141's `#define`, another translation unit), so C's own
	     `#else` arm - `#ifndef WM_TOUCH / #define WM_TOUCH 576 / #endif` and `case WM_TOUCH:` -
	     is what the port carries (the ledger's Deviations record it).  */
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  /* It is said that DefWindowProc will release the touch
	     information in the event.  */
	  return 0;
	}
      #[cfg (feature = "windowsnt")]
      WM_CREATE =>
	{
	  setup_w32_kbdhook (hwnd);
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_DESTROY =>
	{
	  #[cfg (feature = "windowsnt")]
	  {
	    maybe_pass_notification (hwnd);
	    remove_w32_kbdhook ();
	  }
	  CoUninitialize ();
	  return 0;
	}
      #[cfg (feature = "windowsnt")]
      _ if msg as c_uint == WM_WTSSESSION_CHANGE =>
	{
	  if wParam as UINT == WTS_SESSION_LOCK
	    { reset_w32_kbdhook_state (); }
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_CLOSE =>
	{
	  #[cfg (feature = "windowsnt")]
	  {
	    maybe_pass_notification (hwnd);
	  }
	  wmsg.dwModifiers = w32_get_modifiers ();
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  return 0;
	}
      WM_ENDSESSION | WM_WINDOWPOSCHANGING =>
	{
	  if msg as c_int == WM_ENDSESSION
	    {
	      my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	      /* Allow time for Emacs to attempt an orderly shutdown.  If we
		 return, the process will be terminated immediately.  FIXME:
		 1000 seconds is too long to sleep if the shutdown attempt
		 fails (see bug#25875).  But if it fails, we want to find out
		 about it, so let's leave 1000 for now.  */
	      sleep (1000);
	      /* C's `FALLTHROUGH` into `case WM_WINDOWPOSCHANGING:`.  */
	    }
	  /* Don't restrict the sizing of any kind of frames.  If the window
	     manager doesn't, there's no reason to do it ourselves.  */
	  return 0;
	}
      WM_GETMINMAXINFO =>
	{
	  /* Hack to allow resizing the Emacs frame above the screen size.
	     Note that Windows 9x limits coordinates to 16-bits.  */
	  (*(lParam as LPMINMAXINFO)).ptMaxTrackSize.x = 32767;
	  (*(lParam as LPMINMAXINFO)).ptMaxTrackSize.y = 32767;
	  return 0;
	}
      WM_SETCURSOR =>
	{
	  if LOWORD! (lParam) as c_int == HTCLIENT
	    {
	      f = w32_window_to_frame (dpyinfo, hwnd);
	      if !f.is_null ()
		{
		  if (*( (*f).output_data.w32 as *mut w32_output )).hourglass_p != 0
		      && menubar_in_use == 0 && current_popup_menu.is_null ()
		    { SetCursor ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).hourglass_cursor); }
		  else if pointer_invisible! (f)
		    { SetCursor (core::ptr::null_mut ()); }
		  else
		    { SetCursor ((*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).current_cursor); }
		}

	      return 0;
	    }
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);	/* C's `goto dflt`.  */
	}
      WM_EMACS_SETCURSOR =>
	{
	  let cursor: HCURSOR = wParam as HCURSOR;
	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null () && !cursor.is_null ()
	    {
	      (*(FRAME_OUTPUT_DATA! (f) as *mut w32_output)).current_cursor = cursor;
	      /* Don't change the cursor while menu-bar menu is in use.  */
	      if (*( (*f).output_data.w32 as *mut w32_output )).menubar_active == 0
		  && (*( (*f).output_data.w32 as *mut w32_output )).hourglass_p == 0
		{
		  if pointer_invisible! (f)
		    { SetCursor (core::ptr::null_mut ()); }
		  else
		    { SetCursor (cursor); }
		}
	    }
	  return 0;
	}
      WM_EMACS_SHOWCURSOR =>
	{
	  ShowCursor ((wParam != 0) as BOOL);

	  return 0;
	}
      WM_EMACS_CREATEVSCROLLBAR =>
	{
	  return w32_createvscrollbar (wParam as *mut frame,
				       lParam as *mut scroll_bar) as LRESULT;
	}
      WM_EMACS_CREATEHSCROLLBAR =>
	{
	  return w32_createhscrollbar (wParam as *mut frame,
				       lParam as *mut scroll_bar) as LRESULT;
	}
      WM_EMACS_SHOWWINDOW =>
	{
	  return ShowWindow (wParam as HWND, lParam as c_int) as LRESULT;
	}
      WM_EMACS_BRINGTOTOP | WM_EMACS_SETFOREGROUND =>
	{
	  let foreground_window: HWND;
	  let mut foreground_thread: DWORD;
	  let mut retval: DWORD;

	  /* On NT 5.0, and apparently Windows 98, it is necessary to
	     attach to the thread that currently has focus in order to
	     pull the focus away from it.  */
	  foreground_window = GetForegroundWindow ();
	  foreground_thread = GetWindowThreadProcessId (foreground_window, core::ptr::null_mut ());
	  if foreground_window.is_null ()
	      || foreground_thread == GetCurrentThreadId ()
	      || AttachThreadInput (GetCurrentThreadId (), foreground_thread, TRUE) == 0
	    { foreground_thread = 0; }

	  retval = SetForegroundWindow (wParam as HWND) as DWORD;
	  if msg as c_int == WM_EMACS_BRINGTOTOP
	    { retval = BringWindowToTop (wParam as HWND) as DWORD; }

	  /* Detach from the previous foreground thread.  */
	  if foreground_thread != 0
	    { AttachThreadInput (GetCurrentThreadId (), foreground_thread, FALSE); }

	  /* SetFocus to give/remove focus to/from a child window.  */
	  if msg as c_int == WM_EMACS_SETFOREGROUND
	    { SetFocus (wParam as HWND); }

	  return retval as LRESULT;
	}
      WM_EMACS_SETWINDOWPOS =>
	{
	  let pos: *mut WINDOWPOS = wParam as *mut WINDOWPOS;
	  return SetWindowPos (hwnd, (*pos).hwndInsertAfter,
			       (*pos).x, (*pos).y, (*pos).cx, (*pos).cy, (*pos).flags) as LRESULT;
	}
      WM_EMACS_DESTROYWINDOW =>
	{
	  RevokeDragDrop (wParam as HWND);
	  return DestroyWindow (wParam as HWND) as LRESULT;
	}
      WM_EMACS_HIDE_CARET =>
	{
	  return HideCaret (hwnd) as LRESULT;
	}
      WM_EMACS_SHOW_CARET =>
	{
	  return ShowCaret (hwnd) as LRESULT;
	}
      WM_EMACS_DESTROY_CARET =>
	{
	  w32_system_caret_hwnd = core::ptr::null_mut ();
	  w32_visible_system_caret_hwnd = core::ptr::null_mut ();
	  return DestroyCaret () as LRESULT;
	}
      WM_EMACS_TRACK_CARET =>
	{
	  /* If there is currently no system caret, create one.  */
	  if w32_system_caret_hwnd.is_null ()
	    {
	      /* Use the default caret width, and avoid changing it
		 unnecessarily, as it confuses screen reader software.  */
	      w32_system_caret_hwnd = hwnd;
	      CreateCaret (hwnd, core::ptr::null_mut (), 0,
			   w32_system_caret_height);
	    }

	  if SetCaretPos (w32_system_caret_x, w32_system_caret_y) == 0
	    { return 0; }
	  /* Ensure visible caret gets turned on when requested.  */
	  else if w32_use_visible_system_caret
		   && w32_visible_system_caret_hwnd != hwnd
	    {
	      w32_visible_system_caret_hwnd = hwnd;
	      return ShowCaret (hwnd) as LRESULT;
	    }
	  /* Ensure visible caret gets turned off when requested.  */
	  else if !w32_use_visible_system_caret
		   && !w32_visible_system_caret_hwnd.is_null ()
	    {
	      w32_visible_system_caret_hwnd = core::ptr::null_mut ();
	      return HideCaret (hwnd) as LRESULT;
	    }
	  else
	    { return 1; }
	}
      WM_EMACS_TRACKPOPUPMENU =>
	{
	  let mut flags: UINT;
	  let pos: *mut POINT;
	  let mut retval: c_int;
	  pos = lParam as *mut POINT;
	  flags = TPM_CENTERALIGN;
	  if button_state & LMOUSE != 0
	    { flags |= TPM_LEFTBUTTON; }
	  else if button_state & RMOUSE != 0
	    { flags |= TPM_RIGHTBUTTON; }

	  /* We may have done a SetCapture on the initial mouse down
	     event, so for safety, make sure the capture is canceled
	     now.  */
	  ReleaseCapture ();
	  button_state = 0;

	  /* Use menubar_active to indicate that WM_INITMENU is from
	     TrackPopupMenu below, and should be ignored.  */
	  f = w32_window_to_frame (dpyinfo, hwnd);
	  if !f.is_null ()
	    { (*( (*f).output_data.w32 as *mut w32_output )).menubar_active = 1; }

	  if TrackPopupMenu (wParam as HMENU, flags, (*pos).x, (*pos).y,
			     0, hwnd, core::ptr::null ()) != 0
	    {
	      let mut amsg: MSG = core::mem::zeroed ();
	      /* Eat any mouse messages during popupmenu */
	      while PeekMessage (&raw mut amsg, hwnd, WM_MOUSEFIRST as UINT,
				 WM_MOUSELAST as UINT, PM_REMOVE) != 0 { }
	      /* Get the menu selection, if any */
	      if PeekMessage (&raw mut amsg, hwnd, WM_COMMAND as UINT, WM_COMMAND as UINT,
			      PM_REMOVE) != 0
		{
		  retval = LOWORD! (amsg.wParam) as c_int;
		}
	      else
		{
		  retval = 0;
		}
	    }
	  else
	    {
	      retval = -1;
	    }

	  return retval as LRESULT;
	}
      WM_EMACS_FILENOTIFY =>
	{
	  my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	  return 1;
	}
      _ =>
	{
	  /* Check for messages registered at runtime. */
	  if msg == msh_mousewheel
	    {
	      wmsg.dwModifiers = w32_get_modifiers ();
	      my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	      signal_user_input ();
	      return 0;
	    }

	  /* dflt: */
	  return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	    (hwnd, msg, wParam, lParam);
	}
      }

    if handle_plain_button
      {
	/* C: `handle_plain_button:` (C 5251-5321) - the body `case WM_MBUTTONDOWN:` is labelled
	   with, and the target of the two `goto handle_plain_button`s above.  */
	let mut up: BOOL = 0;
	let mut button: c_int = 0;

	/* Ignore middle and extra buttons as long as the menu is active.  */
	f = w32_window_to_frame (dpyinfo, hwnd);
	if !f.is_null () && (*( (*f).output_data.w32 as *mut w32_output )).menubar_active != 0
	  { return 0; }

	if parse_button (msg as c_int, HIWORD! (wParam) as c_int, &raw mut button,
			 &raw mut up) != 0
	  {
	    if up != 0
	      { ReleaseCapture (); }
	    /* Don't capture mouse when dropping.  */
	    else if !EQ (track_mouse, Qdropping ())
	      { SetCapture (hwnd); }
	    button = if button == 0 { LMOUSE }
		     else if button == 1 { MMOUSE }
		     else { RMOUSE };
	    if up != 0
	      { button_state &= !button; }
	    else
	      { button_state |= button; }
	  }

	if !f.is_null () && (msg as c_int == WM_LBUTTONDOWN || msg as c_int == WM_RBUTTONDOWN
			     || msg as c_int == WM_MBUTTONDOWN || msg as c_int == WM_XBUTTONDOWN)
	    && FRAME_NO_ACCEPT_FOCUS (f) == 0
	  /* When clicking into a child frame or when clicking into a
	     parent frame with the child frame selected and
	     `no-accept-focus' is not set, select the clicked frame.  */
	  {
	    let p: *mut frame = FRAME_PARENT_FRAME (XFRAME (selected_frame));

	    if !FRAME_PARENT_FRAME (f).is_null () || f == p
	      {
		SetFocus (hwnd);
		SetWindowPos (hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
	      }
	  }

	wmsg.dwModifiers = w32_get_modifiers ();
	my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	signal_user_input ();

	/* Need to return true for XBUTTON messages, false for others,
	   to indicate that we processed the message.  */
	return if msg as c_int == WM_XBUTTONDOWN || msg as c_int == WM_XBUTTONUP { 1 } else { 0 };
      }
    if command
      {
	/* C: `command:` (C 5708-5712).  */
	wmsg.dwModifiers = w32_get_modifiers ();
	my_post_msg (&raw mut wmsg, hwnd, msg, wParam, lParam);
	return (if w32_unicode_gui != 0 { DefWindowProcW } else { DefWindowProcA })
	  (hwnd, msg, wParam, lParam);
      }
  }

  /* The most common default return code for handled messages is 0.  */
  0
}

/// C 5331: C's function-local `static int last_x, last_y;` inside `w32_wnd_proc`'s
/// `WM_MOUSEMOVE` arm are two objects with static storage duration, which Rust spells at file
/// scope (the `force_quit_count` reading above); C leaves them uninitialised, so their static
/// storage is the zero value.
/// C: static int last_x, last_y;
#[cfg (feature = "have_ntgui")]
static mut last_x: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut last_y: c_int = 0;

/// C 5988-6033: `my_create_window` - C's `static void` (a private `unsafe fn` here) plus the
/// function-local `static int coords[2]` C 5997 declares, which has static storage duration and is
/// therefore a file-scope `static mut` (the `force_quit_count`/`last_x` reading above).  The two
/// `gui_display_get_arg` calls are C 6015-6018's, with C's own `"left"`/`"Left"` strings.
/// C: static int coords[2];
/// C: static void my_create_window (struct frame * f);
#[cfg (feature = "have_ntgui")]
static mut coords: [c_int; 2] = [0, 0];
#[cfg (feature = "have_ntgui")]
unsafe fn my_create_window (f: *mut frame)
{
  let mut msg: MSG = core::mem::zeroed ();        /* C's uninitialised `MSG msg;`, PORTING.md 1.5 */
  let mut left: Lisp_Object; let mut top: Lisp_Object;
  let dpyinfo: *mut w32_display_info = &raw mut one_w32_display_info;

  /* If f->size_hint_flags is set, it means gui_figure_window_size
     already processed the 'top' and 'left' frame parameters and set
     f->top_pos and f->left_pos accordingly.  w32_createwindow will
     then use those value disregarding coords[].  So we don't need to
     compute coords[] in that case.  */
  if ! ((((*f).size_hint_flags as c_long) & USPosition != 0)
	|| (((*f).size_hint_flags as c_long) & PPosition != 0))
    {
      /* When called with RES_TYPE_NUMBER, and there's no 'top' or
	 'left' parameters in the frame's parameter alist,
	 gui_display_get_arg will return zero for anything that is
	 neither a number nor Qunbound.  If frame parameter alist does
	 have 'left' or 'top', they are interpreted by
	 gui_figure_window_size, which was already called, and which
	 sets f->size_hint_flags.  */
      left = gui_display_get_arg (dpyinfo, Qnil (), Qleft (), c"left".as_ptr (),
				  c"Left".as_ptr (), RES_TYPE_NUMBER);
      top = gui_display_get_arg (dpyinfo, Qnil (), Qtop (), c"top".as_ptr (),
				 c"Top".as_ptr (), RES_TYPE_NUMBER);
      if BASE_EQ (left, Qunbound ())
	{ coords[0] = CW_USEDEFAULT; }
      else
	{ coords[0] = XFIXNUM (left) as c_int; }
      if BASE_EQ (top, Qunbound ())
	{ coords[1] = CW_USEDEFAULT; }
      else
	{ coords[1] = XFIXNUM (top) as c_int; }
    }

  if PostThreadMessage (dwWindowsThreadId, WM_EMACS_CREATEWINDOW as UINT,
			f as WPARAM, coords.as_mut_ptr () as LPARAM) == 0
    { emacs_abort (); }
  let get_message: unsafe extern "system" fn (lpMsg: *mut MSG, hWnd: HWND,
					      wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL
    = if w32_unicode_gui != 0 { GetMessageW } else { GetMessageA };
  get_message (&raw mut msg, core::ptr::null_mut (), WM_EMACS_DONE as UINT,
	       WM_EMACS_DONE as UINT);
}


/// C 6036-6078: `my_create_tip_window` - C's `static void`, and C 6049/6051-6062's
/// `f->output_data.w32->` reads are this file's `(*f).output_data.w32 as *mut w32_output` cast
/// (C 1221-1242's reading).  C 6051-6052's `tip_window = FRAME_W32_WINDOW (f) = CreateWindow (…)`
/// is one assignment through the `FRAME_W32_WINDOW!` macro, so the window handle is stored twice.
/// C: static void my_create_tip_window (struct frame *f);
#[cfg (feature = "have_ntgui")]
unsafe fn my_create_tip_window (f: *mut frame)
{
  let mut rect: RECT = core::mem::zeroed ();     /* C's uninitialised `RECT rect;`, PORTING.md 1.5 */

  rect.left = 0; rect.top = 0;                   /* C's chained `rect.left = rect.top = 0;` */
  rect.right = FRAME_PIXEL_WIDTH (f);
  rect.bottom = FRAME_PIXEL_HEIGHT (f);

  AdjustWindowRect (&raw mut rect,
		   (*((*f).output_data.w32 as *mut w32_output)).dwStyle, FALSE);

  /* C 6051-6052's `tip_window = FRAME_W32_WINDOW (f) = CreateWindow (…)` nests one assignment in
     another; Rust's assignment yields `()`, so the port keeps C's two stores as two statements,
     in C's order (the window's own field first, then `tip_window`).  */
  let created: Window
    = CreateWindow (EMACS_CLASS.as_ptr () as *const c_char,
		    (*f).namebuf,
		    (*((*f).output_data.w32 as *mut w32_output)).dwStyle,
		    (*f).left_pos,
		    (*f).top_pos,
		    rect.right - rect.left,
		    rect.bottom - rect.top,
		    FRAME_W32_WINDOW! (SELECTED_FRAME ()), /* owner */
		    core::ptr::null_mut (),
		    hinst,
		    core::ptr::null_mut ());
  FRAME_W32_WINDOW! (f) = created;
  tip_window = created;

  if ! tip_window.is_null ()
    {
      SetWindowLong (tip_window, WND_FONTWIDTH_INDEX, FRAME_COLUMN_WIDTH (f) as LONG);
      SetWindowLong (tip_window, WND_LINEHEIGHT_INDEX, FRAME_LINE_HEIGHT (f) as LONG);
      SetWindowLong (tip_window, WND_BORDER_INDEX,
		     FRAME_INTERNAL_BORDER_WIDTH (f) as LONG);
      SetWindowLong (tip_window, WND_BACKGROUND_INDEX,
		     FRAME_BACKGROUND_PIXEL (f) as LONG);

      /* Tip frames have no scrollbars.  */
      SetWindowLong (tip_window, WND_VSCROLLBAR_INDEX, 0);
      SetWindowLong (tip_window, WND_HSCROLLBAR_INDEX, 0);

      /* Do this to discard the default setting specified by our parent. */
      ShowWindow (tip_window, SW_HIDE);
    }
}


/// C 6080-6121: `w32_window` - C's `static void`; C 6093's `f->namebuf = xlispstrdup (…)` is the
/// same field slice 2 wrote, and C 6103-6111's block is a plain scope whose three statements keep
/// C's order (the `explicit` save, the two `Qnil` writes through `fset_name`, and the `w32_set_name
/// (f, name, explicit)` call).
/// C: static void w32_window (struct frame *f, long window_prompting, bool minibuffer_only);
#[cfg (feature = "have_ntgui")]
unsafe fn w32_window (f: *mut frame, window_prompting: c_long, minibuffer_only: bool)
{
  block_input ();

  /* Use the resource name as the top-level window name
     for looking up resources.  Make a non-Lisp copy
     for the window manager, so GC relocation won't bother it.

     Elsewhere we specify the window name for the window manager.  */
  (*f).namebuf = xlispstrdup (Vx_resource_name);

  my_create_window (f);

  validate_x_resource_name ();

  /* w32_set_name normally ignores requests to set the name if the
     requested name is the same as the current name.  This is the one
     place where that assumption isn't correct; f->name is set, but
     the server hasn't been told.  */
  {
    let name: Lisp_Object;
    /* C 2039-2080's `f->explicit_name`, the `bool_bf` frame.h 417 packs: the mirror's flag table
       has the unit/shift, so the port reads and writes it through `frame_flag`/`frame_set_flag`
       (slice 12's correction of slices 6 and 7, which wrote the member directly).  */
    let explicit: c_int = frame_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT)
      as c_int;

    frame_set_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT, 0);
    name = (*f).name;
    fset_name (f, Qnil ());
    w32_set_name (f, name, explicit != 0);
  }

  unblock_input ();

  if ! minibuffer_only && FRAME_EXTERNAL_MENU_BAR (f)
      && FRAME_PARENT_FRAME (f).is_null ()
    { initialize_frame_menubar (f); }

  if FRAME_W32_WINDOW! (f).is_null ()
    { error (c"Unable to create window".as_ptr ()); }
}

/// C 6123-6152: `w32_icon` - C's `static void` (a private `unsafe fn` here); C 6135-6138's two
/// `gui_display_get_arg` calls pass C's two null `const char *` arguments, and C 6145's message is
/// C's own text.
/// C: static void w32_icon (struct frame *f, Lisp_Object parms);
#[cfg (feature = "have_ntgui")]
unsafe fn w32_icon (f: *mut frame, parms: Lisp_Object)
{
  let mut icon_x: Lisp_Object; let mut icon_y: Lisp_Object;
  let dpyinfo: *mut w32_display_info = &raw mut one_w32_display_info;

  /* Set the position of the icon.  Note that Windows 95 groups all
     icons in the tray.  */
  icon_x = gui_display_get_arg (dpyinfo, parms, Qicon_left (), core::ptr::null (),
				core::ptr::null (), RES_TYPE_NUMBER);
  icon_y = gui_display_get_arg (dpyinfo, parms, Qicon_top (), core::ptr::null (),
				core::ptr::null (), RES_TYPE_NUMBER);
  if ! BASE_EQ (icon_x, Qunbound ()) && ! BASE_EQ (icon_y, Qunbound ())
    {
      CHECK_FIXNUM (icon_x);
      CHECK_FIXNUM (icon_y);
    }
  else if ! BASE_EQ (icon_x, Qunbound ()) || ! BASE_EQ (icon_y, Qunbound ())
    { error (c"Both left and top icon corners of icon must be specified".as_ptr ()); }

  block_input ();

  /* TODO: Start up iconic or window? */

  unblock_input ();
}


/// C 6155-6178: `w32_make_gc` - C's `static void`; C 6168-6171's `XCreateGC` takes C's `NULL` as the
/// display and the record's address, and C 6166/6167's two colour reads are the mirror's
/// `FRAME_BACKGROUND_PIXEL` and the `cursor_pixel` field (whose `COLORREF` is a `DWORD`, so the
/// record's `c_ulong` member needs the widening cast C's own promotion gives it).
/// C: static void w32_make_gc (struct frame *f);
#[cfg (feature = "have_ntgui")]
unsafe fn w32_make_gc (f: *mut frame)
{
  let mut gc_values: Emacs_GC = core::mem::zeroed ();   /* C's uninitialised `Emacs_GC gc_values;` */

  block_input ();

  /* Create the GC's of this frame.
     Note that many default values are used.  */

  /* Cursor has cursor-color background, background-color foreground.  */
  gc_values.foreground = FRAME_BACKGROUND_PIXEL (f);
  gc_values.background = (*((*f).output_data.w32 as *mut w32_output)).cursor_pixel as c_ulong;
  (*((*f).output_data.w32 as *mut w32_output)).cursor_gc
    = XCreateGC (core::ptr::null_mut (), FRAME_W32_WINDOW! (f),
		 (GCForeground | GCBackground) as c_ulong,
		 &raw mut gc_values);

  /* Reliefs.  */
  (*((*f).output_data.w32 as *mut w32_output)).white_relief.gc = core::ptr::null_mut ();
  (*((*f).output_data.w32 as *mut w32_output)).black_relief.gc = core::ptr::null_mut ();

  unblock_input ();
}


/// C 6181-6208: `unwind_create_frame` - C's `static Lisp_Object`; C 6193-6195/6200-6203's two
/// `#ifdef GLYPH_DEBUG` arms carry the `glyph_debug` feature (registered, P-51's reading), and C
/// 6202's `dpyinfo_refcount` is C 293's file-scope `static int` this unit already declares.
/// C: static Lisp_Object unwind_create_frame (Lisp_Object frame);
#[cfg (feature = "have_ntgui")]
unsafe fn unwind_create_frame (frame: Lisp_Object) -> Lisp_Object
{
  let f: *mut frame = XFRAME (frame);

  /* If frame is ``official'', nothing to do.  */
  if NILP (Fmemq (frame, Vframe_list))
    {
#[cfg (feature = "glyph_debug")]
      let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO (f);

      w32_free_frame_resources (f);
      free_glyphs (f);

#[cfg (feature = "glyph_debug")]
      {
	/* Check that reference counts are indeed correct.  */
	eassert! ((*dpyinfo).reference_count == dpyinfo_refcount);
      }
      return Qt ();
    }

  return Qnil ();
}

/// C 6210-6214: `do_unwind_create_frame` - C's `static void`, one call through to the function
/// above (the `unwind-protect` body C 6213 hands to `record_unwind_protect`).  C 6389 of the
/// next slice takes this function's address (`record_unwind_protect (do_unwind_create_frame,
/// frame)`), so the C ABI is part of the item and the definition carries it - the spelling
/// the four sibling units use for the same function (`src/androidfns.rs` 991,
/// `src/haikufns.rs` 1289, `src/pgtkfns.rs` 2007, `src/xfns.rs` 6688); slice 7 corrects it.
/// C: static void do_unwind_create_frame (Lisp_Object frame);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn do_unwind_create_frame (frame: Lisp_Object)
{
  unwind_create_frame (frame);
}


/// C 6242/6250: the two readers this slice needs beyond the ones upward - `font_open_by_name`
/// (`font.h` 883, `src/frame.rs` 4788's declaration shape) and `gui_default_parameter`
/// (`dispextern.h` 4001-4003, `src/pgtkfns.rs` 561-566's shape).
/// C: extern Lisp_Object font_open_by_name (struct frame *f, Lisp_Object name);
/// C: extern Lisp_Object gui_default_parameter (struct frame *, Lisp_Object, Lisp_Object,
/// C:   Lisp_Object, const char *, const char *, enum resource_types);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;
  pub fn gui_default_parameter (f: *mut frame, alist: Lisp_Object, prop: Lisp_Object,
				deflt: Lisp_Object, name: *const c_char, x_name: *const c_char,
				r#type: resource_types) -> Lisp_Object;
}

/// C 6216-6251: `w32_default_font_parameter` - C's non-`static` `void` (so a `pub` definition); C
/// 6233-6238's function-local `static const char *names[]` is the file-scope array below (the
/// `src/font.rs` 2706 `style_names` shape: a `*const c_char` array with C's own literals and the
/// terminating null), and C 6226-6228's `?:` is the `if` expression the port writes it as.
/// C: static const char *names[] = { "Courier New-10", "-*-Courier-normal-r-*-*-13-*-*-*-c-*-iso8859-1",
/// C:   "-*-Fixedsys-normal-r-*-*-12-*-*-*-c-*-iso8859-1", "Fixedsys", NULL };
/// C: void w32_default_font_parameter (struct frame *f, Lisp_Object parms);
#[cfg (feature = "have_ntgui")]
static mut names: [*const c_char; 5]
  = [c"Courier New-10".as_ptr (),
     c"-*-Courier-normal-r-*-*-13-*-*-*-c-*-iso8859-1".as_ptr (),
     c"-*-Fixedsys-normal-r-*-*-12-*-*-*-c-*-iso8859-1".as_ptr (),
     c"Fixedsys".as_ptr (),
     core::ptr::null ()];
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_default_font_parameter (f: *mut frame, parms: Lisp_Object)
{
  let dpyinfo: *mut w32_display_info = FRAME_DISPLAY_INFO (f);
  let mut font_param: Lisp_Object = gui_display_get_arg (dpyinfo,
							parms, Qfont (), core::ptr::null (),
							core::ptr::null (),
							RES_TYPE_STRING);
  let mut font: Lisp_Object;
  if BASE_EQ (font_param, Qunbound ())
    { font_param = Qnil (); }
  font = if ! NILP (font_param)
    { font_param }
    else
    { gui_display_get_arg (dpyinfo, parms, Qfont (), c"font".as_ptr (), c"Font".as_ptr (),
			   RES_TYPE_STRING) };

  if ! STRINGP (font)
    {
      let mut i: c_int = 0;
      /* C's `for (i = 0; names[i]; i++)` - the null terminator is the test, and the `break`
	 C 6244 writes is the one below.  */
      while ! names[i as usize].is_null ()
	{
	  font = font_open_by_name (f, build_unibyte_string (names[i as usize]));
	  if ! NILP (font)
	    { break; }
	  i += 1;
	}
      if NILP (font)
	{ error (c"No suitable font was found".as_ptr ()); }
    }

  gui_default_parameter (f, parms, Qfont (), font, c"font".as_ptr (), c"Font".as_ptr (),
			 RES_TYPE_STRING);
}

/* C's `f->output_method = …` (frame.h 446's `ENUM_BF (output_method) output_method : 4`):
   `include/frame.rs` mirrors the four-bit reader (`output_method (f)`), C 446's
   `_UNIT`/`_SHIFT`/`_MASK` and the `bits1` word, but not C's assignment shape, so this unit adds
   it under C's own name (the `src/androidfns.rs` 283-302 device, `src/pgtkfns.rs` 240-249).
   C 6367's `f->output_method = output_w32;` is the only write this unit makes.  */
#[cfg (feature = "have_ntgui")]
macro_rules! set_output_method {
  ($f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
		  | ((($v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT)
  };
}
#[cfg (feature = "have_ntgui")]
macro_rules! output_method {
  ($f:expr) => { output_method ($f) };
  ($f:expr, $v:expr) => { set_output_method! ($f, $v) };
}

/// C 8019's writer: frame.h 1035's `#define FRAME_TOTAL_COLS(f) ((f)->total_cols)`.  The mirror
/// carries the reader; a unit that writes it adds the write arm under C's own name (the
/// `src/androidfns.rs` 2963-2967 / `src/haikufns.rs` 449-454 device).
#[cfg (feature = "have_ntgui")]
macro_rules! FRAME_TOTAL_COLS {
  ($f:expr, $v:expr) => { (*$f).total_cols = $v };
}
/// C 8035's expansion: dispextern.h 291-292's
/// `#define SET_TEXT_POS(pos, charpos, bytepos) ((pos).charpos = (charpos), (pos).bytepos = (bytepos))`
/// - the mirror's copy is a function, and this file needs C's assignment shape (the
/// `src/androidfns.rs` 2968-2974 device).
#[cfg (feature = "have_ntgui")]
macro_rules! SET_TEXT_POS {
  ($pos:expr, $cp:expr, $bp:expr) => {{
    $pos.charpos = $cp;
    $pos.bytepos = $bp;
  }};
}

/* keyboard.h 38's `KVAR` is a token-pasting macro (`((kboard)->field ## _)`) with no Rust
   counterpart (the `include/keyboard.rs` 10-13 note); the port keeps the name and one arm per
   member, the shape `src/haikufns.rs` 1297-1301 spells for these two C lines (C 6696-6697), and
   `include/keyboard.rs` 79 carries the field the arm names.  */
#[cfg (feature = "have_ntgui")]
macro_rules! KVAR {
  ($kb:expr, Vdefault_minibuffer_frame) => { (*$kb).Vdefault_minibuffer_frame_ };
}

/// C 6316/6320/6324/6329, C 6333 and C 6335-6348's readers: C's own `frame.h` prototypes for
/// `make_frame` (1479), `make_minibuffer_frame` (1489), `make_frame_without_minibuffer` (1491),
/// `frame_set_id_from_params` (1430, whose C return type is `EMACS_UINT` - C 6333 discards it)
/// and `fset_parent_frame` (774's C `INLINE` setter), plus `gui_figure_window_size` (1865),
/// `register_font_driver` (font.h 896) and `gui_init_fringe` (xterm.h 1162, C 6375).  None of the
/// eight is declared by this unit's includes: `include/frame.rs` carries `init_frame_faces` and
/// `include/w32term.rs` the three `w32_*` readers this slice calls, but the record and the
/// definitions of the rest live in other units, which is why the sibling `*fns` units declare
/// them the same way.  `struct kboard` has a mirror (`include/keyboard.rs` 56), so the two
/// prototypes that name it keep the record and P-72 rule ③ does not apply; `gui_init_fringe`'s
/// parameter is `struct redisplay_interface *` in C and the untyped `*mut c_void` in the two
/// sibling spellings (`src/haikuterm.rs` 6260, `src/pgtkterm.rs` 10260), which this slice copies.
/// C: extern struct frame *make_frame (bool);
/// C: extern struct frame *make_minibuffer_frame (void);
/// C: extern struct frame *make_frame_without_minibuffer (Lisp_Object, struct kboard *, Lisp_Object);
/// C: extern EMACS_UINT frame_set_id_from_params (struct frame *, Lisp_Object);
/// C: INLINE void fset_parent_frame (struct frame *f, Lisp_Object val);
/// C: extern long gui_figure_window_size (struct frame *, Lisp_Object, bool, bool);
/// C: extern void register_font_driver (struct font_driver const *, struct frame *);
/// C: extern void gui_init_fringe (struct redisplay_interface *);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn make_frame (mini_p: bool) -> *mut frame;
  pub fn make_minibuffer_frame () -> *mut frame;
  pub fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut kboard,
					display: Lisp_Object) -> *mut frame;
  pub fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object);
  pub fn fset_parent_frame (f: *mut frame, val: Lisp_Object);
  pub fn gui_figure_window_size (f: *mut frame, parms: Lisp_Object, x_frame: bool,
				 toolbar_p: bool) -> c_long;
  pub fn register_font_driver (driver: *const font_driver, f: *mut frame);
  pub fn gui_init_fringe (rif: *mut c_void);
}

/// C 6253-6712: `x-create-frame` - the file's first `DEFUN` (`Fx_create_frame`,
/// `Sx_create_frame`, C 6254's `1, 1, 0` and C 6256's single `Lisp_Object parameters`), the whole
/// item from C 6253 through C 6712's closing brace (C 6714 opens `xw-color-defined-p`, the next
/// one).  C 6255's doc body is `xfns.c`'s (`doc: /* SKIP: real doc in xfns.c.  */`), so the port
/// carries the `doc:` string the sibling `*fns` units carry.
///
/// C's `struct kboard *kb` (C 6267) is read at C 6289 from `dpyinfo->terminal->kboard`, whose
/// mirror keeps `terminal.kboard` as the untyped `*mut c_void` (P-72 rule ③): the read carries
/// C's own type as a cast, and the three consumers keep the record
/// (`make_frame_without_minibuffer` twice, C 6698's `kset_default_minibuffer_frame` and C 6696's
/// `KVAR`, the `src/haikufns.rs` 1683 spelling of the same three C lines).
///
/// C 6426-6432's `#ifdef HAVE_HARFBUZZ` carries the `have_harfbuzz` arm around the one
/// `register_font_driver` call, both C 6430's and C 6432's calls stay text, and the three driver
/// objects are the other units' (`src/w32dwrite.rs` 2313 and `src/w32font.rs` 3731,
/// `src/w32uniscribe.rs` 2547): `struct font_driver` has a mirror (`include/font.rs` 553), so the
/// three declarations keep the record here the way `src/w32font.rs` 1954 spells the prototype.
///
/// The item is `text-only` (P-86 ③⑵): this machine has no Windows SDK, so the `<windows.h>` names
/// it reads (C 6362-6363's `GetSystemMetrics`, C 6586-6600's eighteen `IDC_*` ids and
/// `w32_load_cursor`) are spelled from PORTING.md's table and never compiled here - the file
/// head's note; `w32_load_cursor`, `GetSystemMetrics` and `SM_CX*SCROLL` are this unit's own.
/// C: DEFUN ("x-create-frame", Fx_create_frame, Sx_create_frame,
/// C:        1, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object parameters)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-create-frame", Fx_create_frame, Sx_create_frame,
	1, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(parameters: Lisp_Object),
	{
  let mut f: *mut frame;
  let mut frame: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut name: Lisp_Object;
  let mut minibuffer_only: bool = false;
  let mut window_prompting: c_long = 0;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut display: Lisp_Object;
  let mut dpyinfo: *mut w32_display_info = core::ptr::null_mut ();
  let mut parent: Lisp_Object;
  let mut parent_frame: Lisp_Object;
  let mut kb: *mut kboard;
  let mut parameters = parameters;              /* C 6275 assigns the parameter.  */

  /* C 6269-6271.  */
  if ! FRAME_W32_P (SELECTED_FRAME ())
      && ! FRAME_INITIAL_P (SELECTED_FRAME ())
    { error (c"Cannot create a GUI frame in a -nw session".as_ptr ()); }

  /* Make copy of frame parameters because the original is in pure
     storage now. */
  parameters = Fcopy_alist (parameters);

  /* Use this general default value to start with
     until we know if this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name;

  /* C 6281-6285: C's two `0` name arguments are the null pointers the port spells
     `core::ptr::null ()` (the `src/w32fns.rs` 9335 precedent).  */
  display = gui_display_get_arg (dpyinfo, parameters, Qterminal (),
				 core::ptr::null (), core::ptr::null (),
				 RES_TYPE_NUMBER);
  if BASE_EQ (display, Qunbound ())
    { display = gui_display_get_arg (dpyinfo, parameters, Qdisplay (),
				     core::ptr::null (), core::ptr::null (),
				     RES_TYPE_STRING); }
  if BASE_EQ (display, Qunbound ())
    { display = Qnil (); }
  dpyinfo = check_x_display_info (display);
  /* C 6289's `dpyinfo->terminal->kboard` keeps C's type as this cast (the note above).  */
  kb = (*((*dpyinfo).terminal)).kboard as *mut kboard;

  if (*((*dpyinfo).terminal)).name.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  name = gui_display_get_arg (dpyinfo, parameters, Qname (),
			      c"name".as_ptr (), c"Name".as_ptr (),
			      RES_TYPE_STRING);
  if ! STRINGP (name)
      && ! BASE_EQ (name, Qunbound ())
      && ! NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }

  if STRINGP (name)
    { Vx_resource_name = name; }

  /* See if parent window is specified.  */
  parent = gui_display_get_arg (dpyinfo, parameters, Qparent_id (),
				core::ptr::null (), core::ptr::null (),
				RES_TYPE_NUMBER);
  if BASE_EQ (parent, Qunbound ())
    { parent = Qnil (); }
  else if ! NILP (parent)
    { CHECK_FIXNUM (parent); }

  /* make_frame_without_minibuffer can run Lisp code and garbage collect.  */
  /* No need to protect DISPLAY because that's not used after passing
     it to make_frame_without_minibuffer.  */
  frame = Qnil ();
  tem = gui_display_get_arg (dpyinfo, parameters, Qminibuffer (),
			     c"minibuffer".as_ptr (), c"Minibuffer".as_ptr (),
			     RES_TYPE_SYMBOL);
  if EQ (tem, Qnone ()) || NILP (tem)
    { f = make_frame_without_minibuffer (Qnil (), kb, display); }
  else if EQ (tem, Qonly ())
    {
      f = make_minibuffer_frame ();
      minibuffer_only = true;
    }
  else if WINDOWP (tem)
    { f = make_frame_without_minibuffer (tem, kb, display); }
  else
    { f = make_frame (true); }

  XSETFRAME! (frame, f);

  frame_set_id_from_params (f, parameters);

  parent_frame = gui_display_get_arg (dpyinfo, parameters, Qparent_frame (),
				      core::ptr::null (), core::ptr::null (),
				      RES_TYPE_SYMBOL);
  /* Apply `parent-frame' parameter only when no `parent-id' was
     specified.  */
  if ! NILP (parent_frame)
      && (! NILP (parent)
	  || ! FRAMEP (parent_frame)
	  || ! FRAME_LIVE_P (XFRAME (parent_frame))
	  || ! FRAME_W32_P (XFRAME (parent_frame)))
    { parent_frame = Qnil (); }

  fset_parent_frame (f, parent_frame);
  store_frame_param (f, Qparent_frame (), parent_frame);

  tem = gui_display_get_arg (dpyinfo, parameters, Qundecorated (),
			     core::ptr::null (), core::ptr::null (),
			     RES_TYPE_BOOLEAN);
  FRAME_UNDECORATED! (f, ! NILP (tem) && ! BASE_EQ (tem, Qunbound ()));
  store_frame_param (f, Qundecorated (),
		     if FRAME_UNDECORATED (f) != 0 { Qt () } else { Qnil () });

  tem = gui_display_get_arg (dpyinfo, parameters, Qskip_taskbar (),
			     core::ptr::null (), core::ptr::null (),
			     RES_TYPE_BOOLEAN);
  FRAME_SKIP_TASKBAR! (f, ! NILP (tem) && ! BASE_EQ (tem, Qunbound ()));
  store_frame_param (f, Qskip_taskbar (),
		     if NILP (tem) || BASE_EQ (tem, Qunbound ()) { Qnil () } else { Qt () });

  /* By default, make scrollbars the system standard width and height. */
  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, GetSystemMetrics (SM_CXVSCROLL));
  FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f, GetSystemMetrics (SM_CXHSCROLL));

  (*f).terminal = (*dpyinfo).terminal;

  output_method! (f, output_w32);
  (*f).output_data.w32 = xzalloc (core::mem::size_of::<w32_output> ());
  FRAME_FONTSET! (f) = -1;

  /* Need to finish setting up of user-defined fringe bitmaps that
     were defined before the first GUI frame was created (e.g., while
     in daemon mode).  */
  if (*((*f).terminal)).reference_count == 0
    { gui_init_fringe ((*((*f).terminal)).rif); }

  fset_icon_name (f, gui_display_get_arg (dpyinfo, parameters, Qicon_name (),
					  c"iconName".as_ptr (), c"Title".as_ptr (),
					  RES_TYPE_STRING));
  if ! STRINGP ((*f).icon_name)
    { fset_icon_name (f, Qnil ()); }

  /*  FRAME_DISPLAY_INFO (f) = dpyinfo; */

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (Some (do_unwind_create_frame), frame);

  /* C 6391-6393: `#ifdef GLYPH_DEBUG`.  */
  #[cfg (feature = "glyph_debug")]
  {
    dpyinfo_refcount = (*dpyinfo).reference_count;
  }

  /* Specify the parent under which to make this window - this seems to
     have no effect on Windows because parent_desc is explicitly reset
     below.  */
  if ! NILP (parent)
    {
      /* Cast to UINT_PTR shuts up compiler warnings about cast to
	 pointer from integer of different size.  */
      (*((*f).output_data.w32 as *mut w32_output)).parent_desc
	= XFIXNAT (parent) as UINT_PTR as Window;
      (*((*f).output_data.w32 as *mut w32_output)).explicit_parent = true;
    }
  else
    {
      (*((*f).output_data.w32 as *mut w32_output)).parent_desc
	= (*FRAME_DISPLAY_INFO! (f)).root_window;
      (*((*f).output_data.w32 as *mut w32_output)).explicit_parent = false;
    }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)
    {
      fset_name (f, build_string ((*dpyinfo).w32_id_name));
      FRAME_EXPLICIT_NAME! (f, false);
    }
  else
    {
      fset_name (f, name);
      FRAME_EXPLICIT_NAME! (f, true);
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);
    }

  /* C 6426-6429: `#ifdef HAVE_HARFBUZZ`.  */
  #[cfg (feature = "have_harfbuzz")]
  {
    if harfbuzz_available != 0
      { register_font_driver (&raw mut harfbuzz_font_driver, f); }
  }
  if uniscribe_available != 0
    { register_font_driver (&raw mut uniscribe_font_driver, f); }
  register_font_driver (&raw mut w32font_driver, f);

  gui_default_parameter (f, parameters, Qfont_backend (), Qnil (),
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values
     that are needed to determine window geometry.  */
  w32_default_font_parameter (f, parameters);

  /* Default BorderWidth to 0 to match other platforms.  */
  gui_default_parameter (f, parameters, Qborder_width (), make_fixnum (0),
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* We recognize either internalBorderWidth or internalBorder
     (which is what xterm calls it).  */
  if NILP (Fassq (Qinternal_border_width (), parameters))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parameters, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())
	{ parameters = Fcons (Fcons (Qinternal_border_width (), value), parameters); }
    }

  gui_default_parameter (f, parameters, Qinternal_border_width (), make_fixnum (0),
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);

  /* Same for child frames.  */
  if NILP (Fassq (Qchild_frame_border_width (), parameters))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parameters, Qchild_frame_border_width (),
				   c"childFrameBorder".as_ptr (), c"childFrameBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())
	{ parameters = Fcons (Fcons (Qchild_frame_border_width (), value), parameters); }
    }

  gui_default_parameter (f, parameters, Qchild_frame_border_width (), Qnil (),
			 c"childFrameBorderWidth".as_ptr (),
			 c"childFrameBorderWidth".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qright_divider_width (), make_fixnum (0),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qbottom_divider_width (), make_fixnum (0),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qvertical_scroll_bars (), Qright (),
			 c"verticalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parameters, Qhorizontal_scroll_bars (), Qnil (),
			 c"horizontalScrollBars".as_ptr (), c"ScrollBars".as_ptr (),
			 RES_TYPE_SYMBOL);

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parameters, Qforeground_color (), build_string (c"black".as_ptr ()),
			 c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parameters, Qbackground_color (), build_string (c"white".as_ptr ()),
			 c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parameters, Qmouse_color (), build_string (c"black".as_ptr ()),
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parameters, Qborder_color (), build_string (c"black".as_ptr ()),
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parameters, Qscreen_gamma (), Qnil (),
			 c"screenGamma".as_ptr (), c"ScreenGamma".as_ptr (), RES_TYPE_FLOAT);
  gui_default_parameter (f, parameters, Qline_spacing (), Qnil (),
			 c"lineSpacing".as_ptr (), c"LineSpacing".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qleft_fringe (), Qnil (),
			 c"leftFringe".as_ptr (), c"LeftFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qright_fringe (), Qnil (),
			 c"rightFringe".as_ptr (), c"RightFringe".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qno_focus_on_map (), Qnil (),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parameters, Qno_accept_focus (), Qnil (),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parameters, Qno_special_glyphs (), Qnil (),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parameters, Qalpha_background (), Qnil (),
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qborders_respect_alpha_background (),
			 Qnil (), c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  /* Process alpha here (Bug#16619).  On XP this fails with child
     frames.  For `no-focus-on-map' frames delay processing of alpha
     until the frame becomes visible.  */
  if FRAME_NO_FOCUS_ON_MAP (f) == 0
    { gui_default_parameter (f, parameters, Qalpha (), Qnil (),
			     c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER); }

  /* Init faces first since we need the frame's column width/line
     height in various occasions.  */
  init_frame_faces (f);

  /* We have to call adjust_frame_size here since otherwise
     w32_set_tool_bar_lines will already work with the character sizes
     installed by init_frame_faces while the frame's pixel size is still
     calculated from a character size of 1 and we subsequently hit the
     (height >= 0) assertion in window_box_height.

     The non-pixelwise code apparently worked around this because it
     had one frame line vs one toolbar line which left us with a zero
     root window height which was obviously wrong as well ...

     Also process `min-width' and `min-height' parameters right here
     because `frame-windows-min-size' needs them.  */
  tem = gui_display_get_arg (dpyinfo, parameters, Qmin_width (),
			     core::ptr::null (), core::ptr::null (),
			     RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_width (), tem); }
  tem = gui_display_get_arg (dpyinfo, parameters, Qmin_height (),
			     core::ptr::null (), core::ptr::null (),
			     RES_TYPE_NUMBER);
  if FIXNUMP (tem)
    { store_frame_param (f, Qmin_height (), tem); }
  adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
		     FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 5, true,
		     Qx_create_frame_1 ());

  /* The X resources controlling the menu-bar and tool-bar are
     processed specially at startup, and reflected in the mode
     variables; ignore them here.  */
  if NILP (parent_frame)
    {
      gui_default_parameter (f, parameters, Qmenu_bar_lines (),
			     if NILP (Vmenu_bar_mode) { make_fixnum (0) }
			     else { make_fixnum (1) },
			     core::ptr::null (), core::ptr::null (), RES_TYPE_NUMBER);
    }
  else
    /* No menu bar for child frames.  */
    { store_frame_param (f, Qmenu_bar_lines (), make_fixnum (0)); }

  gui_default_parameter (f, parameters, Qtab_bar_lines (),
			 if NILP (Vtab_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 core::ptr::null (), core::ptr::null (), RES_TYPE_NUMBER);

  gui_default_parameter (f, parameters, Qtool_bar_lines (),
			 if NILP (Vtool_bar_mode) { make_fixnum (0) } else { make_fixnum (1) },
			 core::ptr::null (), core::ptr::null (), RES_TYPE_NUMBER);

  gui_default_parameter (f, parameters, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (),
			 c"InhibitDoubleBuffering".as_ptr (), RES_TYPE_BOOLEAN);

  gui_default_parameter (f, parameters, Qbuffer_predicate (), Qnil (),
			 c"bufferPredicate".as_ptr (), c"BufferPredicate".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parameters, Qtitle (), Qnil (),
			 c"title".as_ptr (), c"Title".as_ptr (), RES_TYPE_STRING);

  (*((*f).output_data.w32 as *mut w32_output)).parent_desc
    = (*FRAME_DISPLAY_INFO! (f)).root_window;
  (*((*f).output_data.w32 as *mut w32_output)).text_cursor = w32_load_cursor (IDC_IBEAM);
  (*((*f).output_data.w32 as *mut w32_output)).nontext_cursor = w32_load_cursor (IDC_ARROW);
  (*((*f).output_data.w32 as *mut w32_output)).modeline_cursor = w32_load_cursor (IDC_ARROW);
  (*((*f).output_data.w32 as *mut w32_output)).hand_cursor = w32_load_cursor (IDC_HAND);
  (*((*f).output_data.w32 as *mut w32_output)).hourglass_cursor = w32_load_cursor (IDC_WAIT);
  (*((*f).output_data.w32 as *mut w32_output)).horizontal_drag_cursor
    = w32_load_cursor (IDC_SIZEWE);
  (*((*f).output_data.w32 as *mut w32_output)).vertical_drag_cursor
    = w32_load_cursor (IDC_SIZENS);
  (*((*f).output_data.w32 as *mut w32_output)).left_edge_cursor = w32_load_cursor (IDC_SIZEWE);
  (*((*f).output_data.w32 as *mut w32_output)).top_left_corner_cursor
    = w32_load_cursor (IDC_SIZENWSE);
  (*((*f).output_data.w32 as *mut w32_output)).top_edge_cursor = w32_load_cursor (IDC_SIZENS);
  (*((*f).output_data.w32 as *mut w32_output)).top_right_corner_cursor
    = w32_load_cursor (IDC_SIZENESW);
  (*((*f).output_data.w32 as *mut w32_output)).right_edge_cursor = w32_load_cursor (IDC_SIZEWE);
  (*((*f).output_data.w32 as *mut w32_output)).bottom_right_corner_cursor
    = w32_load_cursor (IDC_SIZENWSE);
  (*((*f).output_data.w32 as *mut w32_output)).bottom_edge_cursor = w32_load_cursor (IDC_SIZENS);
  (*((*f).output_data.w32 as *mut w32_output)).bottom_left_corner_cursor
    = w32_load_cursor (IDC_SIZENESW);

  (*((*f).output_data.w32 as *mut w32_output)).current_cursor
    = (*((*f).output_data.w32 as *mut w32_output)).nontext_cursor;

  window_prompting = gui_figure_window_size (f, parameters, true, true);

  tem = gui_display_get_arg (dpyinfo, parameters, Qunsplittable (),
			     core::ptr::null (), core::ptr::null (),
			     RES_TYPE_BOOLEAN);
  frame_set_flag (f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT,
		  (minibuffer_only || EQ (tem, Qt ())) as u32);        /* C 6608's `f->no_split`.  */

  w32_window (f, window_prompting, minibuffer_only);
  w32_icon (f, parameters);

  w32_make_gc (f);

  /* Now consider the frame official.  */
  (*((*f).terminal)).reference_count += 1;
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;
  Vframe_list = Fcons (frame, Vframe_list);

  /* We need to do this after creating the window, so that the
     icon-creation functions can say whose icon they're describing.  */
  gui_default_parameter (f, parameters, Qicon_type (), Qnil (),
			 c"bitmapIcon".as_ptr (), c"BitmapIcon".as_ptr (),
			 RES_TYPE_SYMBOL);

  gui_default_parameter (f, parameters, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parameters, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parameters, Qcursor_type (), Qbox (),
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parameters, Qscroll_bar_width (), Qnil (),
			 c"scrollBarWidth".as_ptr (), c"ScrollBarWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qscroll_bar_height (), Qnil (),
			 c"scrollBarHeight".as_ptr (), c"ScrollBarHeight".as_ptr (),
			 RES_TYPE_NUMBER);

  /* Allow set_window_size_hook, now.  */
  frame_set_flag (f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT,
		  true as u32);                                        /* C 6637's bit.  */

  /* Tell the server what size and position, etc, we want, and how
     badly we want them.  This should be done after we have the menu
     bar so that its size can be taken into account.  */
  block_input ();
  w32_wm_set_size_hint (f, window_prompting, false);
  unblock_input ();

  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qx_create_frame_2 ());

  /* Process fullscreen parameter here in the hope that normalizing a
     fullheight/fullwidth frame will produce the size set by the last
     adjust_frame_size call.  */
  gui_default_parameter (f, parameters, Qfullscreen (), Qnil (),
			 c"fullscreen".as_ptr (), c"Fullscreen".as_ptr (),
			 RES_TYPE_SYMBOL);
  gui_default_parameter (f, parameters, Qz_group (), Qnil (),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_SYMBOL);

  gui_default_parameter (f, parameters, Qalpha_background (), Qnil (),
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parameters, Qborders_respect_alpha_background (),
			 Qnil (), c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);

  /* Make the window appear on the frame and enable display, unless
     the caller says not to.  However, with explicit parent, Emacs
     cannot control visibility, so don't try.  */
  if ! (*((*f).output_data.w32 as *mut w32_output)).explicit_parent
    {
      let mut visibility: Lisp_Object
	= gui_display_get_arg (dpyinfo, parameters, Qvisibility (),
			       core::ptr::null (), core::ptr::null (),
			       RES_TYPE_SYMBOL);

      if EQ (visibility, Qicon ())
	{ w32_iconify_frame (f); }
      else
	{
	  if BASE_EQ (visibility, Qunbound ())
	    { visibility = Qt (); }

	  if ! NILP (visibility)
	    { w32_make_frame_visible (f); }
	  else
	    { frame_set_flag (f, FRAME_WAS_INVISIBLE_UNIT, FRAME_WAS_INVISIBLE_SHIFT, true as u32); }
	}

      store_frame_param (f, Qvisibility (), visibility);
    }

  /* For `no-focus-on-map' frames set alpha here.  */
  if FRAME_NO_FOCUS_ON_MAP (f) != 0
    { gui_default_parameter (f, parameters, Qalpha (), Qnil (),
			     c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER); }

  /* Initialize `default-minibuffer-frame' in case this is the first
     frame on this terminal.  */
  if FRAME_HAS_MINIBUF_P (f)
      && (! FRAMEP (KVAR! (kb, Vdefault_minibuffer_frame))
	  || ! FRAME_LIVE_P (XFRAME (KVAR! (kb, Vdefault_minibuffer_frame))))
    { kset_default_minibuffer_frame (kb, frame); }

  /* All remaining specified parameters, which have not been "used" by
     gui_display_get_arg and friends, now go in the misc. alist of the
     frame.  */
  tem = parameters;
  while CONSP (tem)
    {
      if CONSP (XCAR (tem)) && ! NILP (XCAR (XCAR (tem)))
	{ fset_param_alist (f, Fcons (XCAR (tem), (*f).param_alist)); }
      tem = XCDR (tem);
    }

  /* Make sure windows on this frame appear in calls to next-window
     and similar functions.  */
  Vwindow_list = Qnil ();

  return unbind_to (count, frame);
});

/// C 6719/6734's reader: C's `frame.h` prototype for `decode_window_system_frame` (1475), which
/// this unit's includes do not carry (`src/androidfns.rs` 108 declares the same one).  The
/// `wdpyinfo`-and-`frame` shape is frame.h's `WINDOW_SYSTEM_RETURN` arm, `struct frame *`.
/// C: extern WINDOW_SYSTEM_RETURN struct frame *decode_window_system_frame (Lisp_Object);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}

/// C 6714-6727: `xw-color-defined-p` - `w32_defined_color` (this unit's own, C 1126) on the
/// `SSDATA` of the string, C 6723's `&foo` being the port's `&raw mut foo`.  C 6718's
/// `Emacs_Color foo;` is one of PORTING.md 1.5's outputs-only locals - Rust needs a value before
/// the address can be taken, so it is the zero-initialised `core::mem::zeroed ()` this unit
/// already uses for C 1201's `cdef` (the ledger names both sites of this slice).
/// C: DEFUN ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p, 1, 2, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object color, Lisp_Object frame)
#[cfg (feature = "have_ntgui")]
DEFUN! ("xw-color-defined-p", Fxw_color_defined_p, Sxw_color_defined_p, 1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut foo: Emacs_Color = unsafe { core::mem::zeroed () };
  let f: *mut frame = decode_window_system_frame (frame);

  CHECK_STRING (color);

  if w32_defined_color (f, SSDATA (color), &raw mut foo, false, false)
    { return Qt (); }
  else
    { return Qnil (); }
});

/// C 6729-6744: `xw-color-values` (C 6733's `Emacs_Color foo;` is C 6718's twin, PORTING.md 1.5
/// again).  C's three `(GetRValue (x) << 8) | GetRValue (x)` expressions
/// lean on the integer promotion a `BYTE` gets before `<<`; the port casts each read to `c_int`
/// first, the spelling C 2094's sibling uses for the same channel macros (`(GetRValue! (…) as
/// c_int * 256) as c_ushort`).
/// C: DEFUN ("xw-color-values", Fxw_color_values, Sxw_color_values, 1, 2, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object color, Lisp_Object frame)
#[cfg (feature = "have_ntgui")]
DEFUN! ("xw-color-values", Fxw_color_values, Sxw_color_values, 1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(color: Lisp_Object, frame: Lisp_Object),
	{
  let mut foo: Emacs_Color = unsafe { core::mem::zeroed () };
  let f: *mut frame = decode_window_system_frame (frame);

  CHECK_STRING (color);

  if w32_defined_color (f, SSDATA (color), &raw mut foo, false, false)
    { return list3i ((((GetRValue! (foo.pixel) as c_int) << 8)
		      | GetRValue! (foo.pixel) as c_int) as EMACS_INT,
		     (((GetGValue! (foo.pixel) as c_int) << 8)
		      | GetGValue! (foo.pixel) as c_int) as EMACS_INT,
		     (((GetBValue! (foo.pixel) as c_int) << 8)
		      | GetBValue! (foo.pixel) as c_int) as EMACS_INT); }
  else
    { return Qnil (); }
});

/// C 6746-6756: `xw-display-color-p` (C's `int` multiplication against 2).
/// C: DEFUN ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  if ((*dpyinfo).n_planes * (*dpyinfo).n_cbits) <= 2
    { return Qnil (); }

  return Qt ();
});

/// C 6758-6769: `x-display-grayscale-p` (the same product against 1).
/// C: DEFUN ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  if ((*dpyinfo).n_planes * (*dpyinfo).n_cbits) <= 1
    { return Qnil (); }

  return Qt ();
});

/// C 6771-6779: `x-display-pixel-width` - C 6778's reader is `w32term.h`'s declaration, which
/// `include/w32term.rs` 1479-1480 carries.
/// C: DEFUN ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-pixel-width", Fx_display_pixel_width, Sx_display_pixel_width, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  return make_fixnum (w32_display_pixel_width (dpyinfo) as EMACS_INT);
});

/// C 6781-6789: `x-display-pixel-height`.
/// C: DEFUN ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-pixel-height", Fx_display_pixel_height, Sx_display_pixel_height, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  return make_fixnum (w32_display_pixel_height (dpyinfo) as EMACS_INT);
});

/// C 6791-6799: `x-display-planes`.
/// C: DEFUN ("x-display-planes", Fx_display_planes, Sx_display_planes, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-planes", Fx_display_planes, Sx_display_planes, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  return make_fixnum (((*dpyinfo).n_planes * (*dpyinfo).n_cbits) as EMACS_INT);
});

/// C 6801-6816: `x-display-color-cells` (C 6807's `int cap`, C 6814's `min` in the shift count -
/// `include/lisp.rs` 3160's `min`, which C reaches through `lisp.h`).
/// C: DEFUN ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-color-cells", Fx_display_color_cells, Sx_display_color_cells, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let cap: c_int;

  /* Don't use NCOLORS: it returns incorrect results under remote
   * desktop.  We force 24+ bit depths to 24-bit, both to prevent an
   * overflow and because probably is more meaningful on Windows
   * anyway.  */

  cap = 1 << min ((*dpyinfo).n_planes * (*dpyinfo).n_cbits, 24);
  return make_fixnum (cap as EMACS_INT);
});

/// C 6818-6825: `x-server-max-request-size` - a literal 1.
/// C: DEFUN ("x-server-max-request-size", Fx_server_max_request_size, Sx_server_max_request_size,
/// C:        0, 1, 0, doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-server-max-request-size", Fx_server_max_request_size, Sx_server_max_request_size,
	0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  return make_fixnum (1);
});

/// C 6827-6832: `x-server-vendor`.
/// C: DEFUN ("x-server-vendor", Fx_server_vendor, Sx_server_vendor, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object terminal)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-server-vendor", Fx_server_vendor, Sx_server_vendor, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  return build_string (c"Microsoft Corp.".as_ptr ());
});

/// C 6834-6839: `x-server-version` (C 882-888's three file-scope ints, this unit's own).
/// C: DEFUN ("x-server-version", Fx_server_version, Sx_server_version, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object terminal)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-server-version", Fx_server_version, Sx_server_version, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(terminal: Lisp_Object),
	{
  return list3i (w32_major_version as EMACS_INT, w32_minor_version as EMACS_INT,
		 w32_build_number as EMACS_INT);
});

/// C 6841-6846: `x-display-screens`.
/// C: DEFUN ("x-display-screens", Fx_display_screens, Sx_display_screens, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-screens", Fx_display_screens, Sx_display_screens, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  return make_fixnum (1);
});

/// C 6848-6863: `x-display-mm-height` (C 6857's `NULL` HWND is the null pointer the reader's own
/// parameter type names; C 6858-6859's two `(double)` casts are the port's `as c_double`, and
/// C 6862's double result becomes an `EMACS_INT` the way C's implicit conversion does).
/// C: DEFUN ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-mm-height", Fx_display_mm_height, Sx_display_mm_height, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let hdc: HDC;
  let mut mm_per_pixel: c_double;

  hdc = GetDC (core::ptr::null_mut ());
  mm_per_pixel = (GetDeviceCaps (hdc, VERTSIZE) as c_double)
		 / (GetDeviceCaps (hdc, VERTRES) as c_double);
  ReleaseDC (core::ptr::null_mut (), hdc);

  return make_fixnum ((w32_display_pixel_height (dpyinfo) as c_double * mm_per_pixel + 0.5)
		      as EMACS_INT);
});

/// C 6865-6879: `x-display-mm-width` (the same two readers, `HORZSIZE`/`HORZRES`).
/// C: DEFUN ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-mm-width", Fx_display_mm_width, Sx_display_mm_width, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let hdc: HDC;
  let mut mm_per_pixel: c_double;

  hdc = GetDC (core::ptr::null_mut ());
  mm_per_pixel = (GetDeviceCaps (hdc, HORZSIZE) as c_double)
		 / (GetDeviceCaps (hdc, HORZRES) as c_double);
  ReleaseDC (core::ptr::null_mut (), hdc);

  return make_fixnum ((w32_display_pixel_width (dpyinfo) as c_double * mm_per_pixel + 0.5)
		      as EMACS_INT);
});

/// C 6881-6887: `x-display-backing-store` - the static symbol, no display read at all.
/// C: DEFUN ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store,
/// C:        0, 1, 0, doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-backing-store", Fx_display_backing_store, Sx_display_backing_store, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  return Qnot_useful ();
});

/// C 6889-6907: `x-display-visual-class` (C 6897's `dpyinfo->has_palette` is the file's `!= 0`
/// truth test on the mirror's `c_int` member).
/// C: DEFUN ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-visual-class", Fx_display_visual_class, Sx_display_visual_class, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let mut result: Lisp_Object = Qnil ();

  if (*dpyinfo).has_palette != 0
    { result = Qpseudo_color (); }
  else if (*dpyinfo).n_planes * (*dpyinfo).n_cbits == 1
    { result = Qstatic_gray (); }
  else if (*dpyinfo).n_planes * (*dpyinfo).n_cbits == 4
    { result = Qstatic_color (); }
  else if (*dpyinfo).n_planes * (*dpyinfo).n_cbits > 8
    { result = Qtrue_color (); }

  return result;
});

/// C 6909-6915: `x-display-save-under` - `Qnil`, with no display read.
/// C: DEFUN ("x-display-save-under", Fx_display_save_under, Sx_display_save_under, 0, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-save-under", Fx_display_save_under, Sx_display_save_under, 0, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  return Qnil ();
});

/// C 6917-6925: `w32_monitor_enum` - C's `static BOOL CALLBACK ALIGN_STACK` (the `extern "system"`
/// plus the mirror's `ALIGN_STACK`, the `src/w32font.rs` 1958-1966 device), the callback
/// `enum_display_monitors_fn` is called with.  C's two unused parameters (`hdc`, `rcMonitor`) stay
/// in the signature because `MonitorEnum_Proc` fixes it.
/// C: static BOOL CALLBACK ALIGN_STACK
/// C: w32_monitor_enum (HMONITOR monitor, HDC hdc, RECT *rcMonitor, LPARAM dwData)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn w32_monitor_enum (monitor: HMONITOR, hdc: HDC, rcMonitor: *mut RECT,
					    dwData: LPARAM) -> BOOL
{
  let monitor_list: *mut Lisp_Object = dwData as *mut Lisp_Object;

  *monitor_list = Fcons (make_mint_ptr (monitor), *monitor_list);

  return TRUE;
}

/// C 6927-7025: `w32_display_monitor_attributes_list` - the enumeration arm over the three lazily
/// bound entry points (C 6935-6941's two tests are the `Option` tests C 498-503 already gave this
/// unit, `(f.unwrap ()) (…)` at the two call sites).  C 6948's `xmalloc (n * sizeof (*p))` is the
/// unit's `xmalloc` (a `usize` byte count) cast to the pointer's own type, C 6951's
/// `xmint_pointer` is the mirror's, C 6955's `Fmake_vector` and C 6971/6993/6998/7000/7006/7011's
/// `ASET`/`AREF`/`Fcons` are `include/lisp.rs`'s and `include/globals.rs`'s, C 6956's
/// `FOR_EACH_FRAME` is `include/frame.rs`'s three-argument macro (the `src/androidfns.rs` 1948
/// call shape), and C 6980's `struct MONITOR_INFO_EX mi;` is a PORTING.md 1.5 outputs-only local
/// (zero-initialised, as the ledger names).  C 6977's four `Lisp_Object` locals are C's one
/// declaration with one initialiser (`attributes = Qnil`), so the port writes one `let` per name.
/// C: static Lisp_Object
/// C: w32_display_monitor_attributes_list (void)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_display_monitor_attributes_list () -> Lisp_Object
{
  let mut attributes_list: Lisp_Object = Qnil ();
  let mut primary_monitor_attributes: Lisp_Object = Qnil ();
  let mut monitor_list: Lisp_Object = Qnil ();
  let mut monitor_frames: Lisp_Object;
  let mut rest: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut i: c_int;
  let mut n_monitors: c_int;
  let mut monitors: *mut HMONITOR;

  if ! (enum_display_monitors_fn.is_some () && get_monitor_info_fn.is_some ()
	&& monitor_from_window_fn.is_some ())
    { return Qnil (); }

  if ! ((enum_display_monitors_fn.unwrap ()) (core::ptr::null_mut (), core::ptr::null_mut (),
					      w32_monitor_enum,
					      (&raw mut monitor_list) as LPARAM) != 0)
      || NILP (monitor_list)
    { return Qnil (); }

  n_monitors = 0;
  rest = monitor_list;
  while CONSP (rest)
    {
      n_monitors += 1;
      rest = XCDR (rest);
    }

  monitors = xmalloc (n_monitors as usize * core::mem::size_of::<HMONITOR> ()) as *mut HMONITOR;
  i = 0;
  while i < n_monitors
    {
      *monitors.offset (i as isize) = xmint_pointer (XCAR (monitor_list)) as HMONITOR;
      monitor_list = XCDR (monitor_list);
      i += 1;
    }

  monitor_frames = Fmake_vector (make_fixnum (n_monitors as EMACS_INT), Qnil ());
  FOR_EACH_FRAME! (rest, frame, {
      let f: *mut frame = XFRAME (frame);

      if FRAME_W32_P (f) && ! FRAME_TOOLTIP_P (f)
	{
	  let monitor: HMONITOR =
	    (monitor_from_window_fn.unwrap ()) (FRAME_W32_WINDOW! (f),
						MONITOR_DEFAULT_TO_NEAREST);

	  i = 0;
	  while i < n_monitors
	    {
	      if *monitors.offset (i as isize) == monitor
		{ break; }
	      i += 1;
	    }

	  if i < n_monitors
	    { ASET (monitor_frames, i as isize,
		    Fcons (frame, AREF (monitor_frames, i as isize))); }
	}
  });

  i = 0;
  while i < n_monitors
    {
      let mut geometry: Lisp_Object;
      let mut workarea: Lisp_Object;
      let mut name: Lisp_Object;
      let mut attributes: Lisp_Object = Qnil ();
      let hdc: HDC;
      let mut width_mm: c_int;
      let mut height_mm: c_int;
      let mut mi: MONITOR_INFO_EX = core::mem::zeroed ();

      mi.cbSize = core::mem::size_of::<MONITOR_INFO_EX> () as DWORD;
      if ! ((get_monitor_info_fn.unwrap ()) (*monitors.offset (i as isize),
					     (&raw mut mi) as *mut MONITOR_INFO) != 0)
	{
	  i += 1;
	  continue;
	}

      hdc = CreateDCA (c"DISPLAY".as_ptr (), mi.szDevice.as_mut_ptr (),
		       core::ptr::null (), core::ptr::null_mut ());
      if hdc.is_null ()
	{
	  i += 1;
	  continue;
	}
      width_mm = GetDeviceCaps (hdc, HORZSIZE);
      height_mm = GetDeviceCaps (hdc, VERTSIZE);
      DeleteDC (hdc);

      attributes = Fcons (Fcons (Qframes (), AREF (monitor_frames, i as isize)),
			  attributes);

      name = DECODE_SYSTEM (build_unibyte_string (mi.szDevice.as_mut_ptr ()));

      attributes = Fcons (Fcons (Qname (), name), attributes);

      attributes = Fcons (Fcons (Qmm_size (),
				 list2i (width_mm as EMACS_INT, height_mm as EMACS_INT)),
			  attributes);

      workarea = list4i (mi.rcWork.left as EMACS_INT, mi.rcWork.top as EMACS_INT,
			 (mi.rcWork.right - mi.rcWork.left) as EMACS_INT,
			 (mi.rcWork.bottom - mi.rcWork.top) as EMACS_INT);
      attributes = Fcons (Fcons (Qworkarea (), workarea), attributes);

      geometry = list4i (mi.rcMonitor.left as EMACS_INT, mi.rcMonitor.top as EMACS_INT,
			 (mi.rcMonitor.right - mi.rcMonitor.left) as EMACS_INT,
			 (mi.rcMonitor.bottom - mi.rcMonitor.top) as EMACS_INT);
      attributes = Fcons (Fcons (Qgeometry (), geometry), attributes);

      if mi.dwFlags & MONITORINFOF_PRIMARY != 0
	{ primary_monitor_attributes = attributes; }
      else
	{ attributes_list = Fcons (attributes, attributes_list); }
      i += 1;
    }

  if ! NILP (primary_monitor_attributes)
    { attributes_list = Fcons (primary_monitor_attributes, attributes_list); }

  xfree (monitors as *mut c_void);

  return attributes_list;
}

/// C 7027-7082: `w32_display_monitor_attributes_list_fallback` - the single-pseudo-monitor arm.
/// C 7034's `RECT workarea_rect;` is the callee-filled local (`SystemParametersInfo`'s fourth
/// argument), i.e. PORTING.md 1.5's zero initialiser, and C 7058/7061's double results are the
/// `as c_int` truncations the two `x-display-mm-*` readers above spell the same way.
/// C: static Lisp_Object
/// C: w32_display_monitor_attributes_list_fallback (struct w32_display_info *dpyinfo)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_display_monitor_attributes_list_fallback (dpyinfo: *mut w32_display_info)
  -> Lisp_Object
{
  let mut geometry: Lisp_Object;
  let mut workarea: Lisp_Object;
  let mut frames: Lisp_Object;
  let mut rest: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut attributes: Lisp_Object = Qnil ();
  let hdc: HDC;
  let mut mm_per_pixel: c_double;
  let mut pixel_width: c_int;
  let mut pixel_height: c_int;
  let mut width_mm: c_int;
  let mut height_mm: c_int;
  let mut workarea_rect: RECT = core::mem::zeroed ();

  /* Fallback: treat (possibly) multiple physical monitors as if they
     formed a single monitor as a whole.  This should provide a
     consistent result at least on single monitor environments.  */
  attributes = Fcons (Fcons (Qname (), build_string (c"combined screen".as_ptr ())),
		      attributes);

  frames = Qnil ();
  FOR_EACH_FRAME! (rest, frame, {
      let f: *mut frame = XFRAME (frame);

      if FRAME_W32_P (f) && ! FRAME_TOOLTIP_P (f)
	{ frames = Fcons (frame, frames); }
  });
  attributes = Fcons (Fcons (Qframes (), frames), attributes);

  pixel_width = w32_display_pixel_width (dpyinfo);
  pixel_height = w32_display_pixel_height (dpyinfo);

  hdc = GetDC (core::ptr::null_mut ());
  mm_per_pixel = (GetDeviceCaps (hdc, HORZSIZE) as c_double)
		 / (GetDeviceCaps (hdc, HORZRES) as c_double);
  width_mm = (pixel_width as c_double * mm_per_pixel + 0.5) as c_int;
  mm_per_pixel = (GetDeviceCaps (hdc, VERTSIZE) as c_double)
		 / (GetDeviceCaps (hdc, VERTRES) as c_double);
  height_mm = (pixel_height as c_double * mm_per_pixel + 0.5) as c_int;
  ReleaseDC (core::ptr::null_mut (), hdc);
  attributes = Fcons (Fcons (Qmm_size (),
			     list2i (width_mm as EMACS_INT, height_mm as EMACS_INT)),
		      attributes);

  /* GetSystemMetrics below may return 0 for Windows 95 or NT 4.0, but
     we don't care.  */
  geometry = list4i (GetSystemMetrics (SM_XVIRTUALSCREEN) as EMACS_INT,
		     GetSystemMetrics (SM_YVIRTUALSCREEN) as EMACS_INT,
		     pixel_width as EMACS_INT, pixel_height as EMACS_INT);
  if SystemParametersInfo (SPI_GETWORKAREA, 0, (&raw mut workarea_rect) as *mut c_void, 0) != 0
    { workarea = list4i (workarea_rect.left as EMACS_INT, workarea_rect.top as EMACS_INT,
			 (workarea_rect.right - workarea_rect.left) as EMACS_INT,
			 (workarea_rect.bottom - workarea_rect.top) as EMACS_INT); }
  else
    { workarea = geometry; }
  attributes = Fcons (Fcons (Qworkarea (), workarea), attributes);

  attributes = Fcons (Fcons (Qgeometry (), geometry), attributes);

  return list1 (attributes);
}

/// C 7084-7106: `w32-display-monitor-attributes-list` - C 7087-7093's doc body (four paragraphs and
/// the two `\n\n` runs, the shape `src/androidfns.rs` 1971 gives its twin), and C 7099/7103's
/// `block_input`/`unblock_input` pair around the two arms.
/// C: DEFUN ("w32-display-monitor-attributes-list", Fw32_display_monitor_attributes_list,
/// C:        Sw32_display_monitor_attributes_list, 0, 1, 0,
/// C:        doc: /* Return a list of physical monitor attributes on the W32 display DISPLAY. … */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-display-monitor-attributes-list", Fw32_display_monitor_attributes_list,
	Sw32_display_monitor_attributes_list, 0, 1, 0,
	doc: "Return a list of physical monitor attributes on the W32 display DISPLAY.\n\nThe optional argument DISPLAY specifies which display to ask about.\nDISPLAY should be either a frame or a display name (a string).\nIf omitted or nil, that stands for the selected frame's display.\n\nInternal use only, use `display-monitor-attributes-list' instead.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let mut attributes_list: Lisp_Object;

  block_input ();
  attributes_list = w32_display_monitor_attributes_list ();
  if NILP (attributes_list)
    { attributes_list = w32_display_monitor_attributes_list_fallback (dpyinfo); }
  unblock_input ();

  return attributes_list;
});

/// C 7108-7136: `set-message-beep` - C's five-line doc body, C 7116's `CHECK_SYMBOL` and the
/// eight-arm ladder onto the file-scope `static unsigned int sound_type` this unit carries from
/// C 338 (writable, hence `static mut`).  C 7119/7133's `0xFFFFFFFF` is the same literal twice.
/// C: DEFUN ("set-message-beep", Fset_message_beep, Sset_message_beep, 1, 1, 0,
/// C:        doc: /* Set the sound generated when the bell is rung. … */)
/// C:   (Lisp_Object sound)
#[cfg (feature = "have_ntgui")]
DEFUN! ("set-message-beep", Fset_message_beep, Sset_message_beep, 1, 1, 0,
	doc: "Set the sound generated when the bell is rung.\nSOUND is `asterisk', `exclamation', `hand', `question', `ok', or `silent'\nto use the corresponding system sound for the bell.  The `silent' sound\nprevents Emacs from making any sound at all.\nSOUND is nil to use the normal beep.  ",
	(sound: Lisp_Object),
	{
  CHECK_SYMBOL (sound);

  if NILP (sound)
    { sound_type = 0xFFFFFFFF; }
  else if EQ (sound, Qasterisk ())
    { sound_type = MB_ICONASTERISK; }
  else if EQ (sound, Qexclamation ())
    { sound_type = MB_ICONEXCLAMATION; }
  else if EQ (sound, Qhand ())
    { sound_type = MB_ICONHAND; }
  else if EQ (sound, Qquestion ())
    { sound_type = MB_ICONQUESTION; }
  else if EQ (sound, Qok ())
    { sound_type = MB_OK; }
  else if EQ (sound, Qsilent ())
    { sound_type = MB_EMACS_SILENT; }
  else
    { sound_type = 0xFFFFFFFF; }

  return sound;
});

/// C 7142-7164: `w32_display_info_for_name` - the `one_w32_display_info` list walk (C 7149's
/// `for (dpyinfo = &one_w32_display_info; dpyinfo; dpyinfo = dpyinfo->next)` is this file's
/// `while ! dpyinfo.is_null ()` spelling), then C 7158's `w32_term_init` (whose C `char *`
/// parameters take the null pointer and `SSDATA`'s `*mut c_char`), C 7156's
/// `validate_x_resource_name` and C 7161's variadic `error`.
/// C: static struct w32_display_info *
/// C: w32_display_info_for_name (Lisp_Object name)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_display_info_for_name (name: Lisp_Object) -> *mut w32_display_info
{
  let mut dpyinfo: *mut w32_display_info;

  CHECK_STRING (name);

  dpyinfo = &raw mut one_w32_display_info;
  while ! dpyinfo.is_null ()
    {
      if ! NILP (Fstring_equal (XCAR ((*dpyinfo).name_list_element), name))
	{ return dpyinfo; }
      dpyinfo = (*dpyinfo).next;
    }

  /* Use this general default value to start with.  */
  Vx_resource_name = Vinvocation_name;

  validate_x_resource_name ();

  dpyinfo = w32_term_init (name, core::ptr::null_mut (), SSDATA (Vx_resource_name));

  if dpyinfo.is_null ()
    { error (c"Cannot connect to server %s".as_ptr (), SDATA (name)); }

  return dpyinfo;
}

/// C 7166-7244: `x-open-connection` - the `"w32"` name check (C 7180's `strcmp`), C 7185's
/// `window_system_available (NULL)`, the rgb.txt block (C 7193-7204, C's one `Lisp_Object
/// color_file;` local assigned before either read), `Vw32_color_map`'s two sources (C 7203/7206,
/// `w32_default_color_map` being this unit's own) and C 7209's `add_system_logical_colors_to_map`,
/// then C 7218-7225's basename block: C's `char basename[MAX_PATH], *str;` is a PORTING.md 1.5
/// local (the array) whose `str` is assigned by `strrchr` before the test, and C 7221's string is
/// `lib/include/string.rs`'s `lispstpcpy`.  C 7237's `fatal` is variadic and returns `!`.
/// C: DEFUN ("x-open-connection", Fx_open_connection, Sx_open_connection,
/// C:        1, 3, 0, doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display, Lisp_Object xrm_string, Lisp_Object must_succeed)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-open-connection", Fx_open_connection, Sx_open_connection,
	1, 3, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object, xrm_string: Lisp_Object, must_succeed: Lisp_Object),
	{
  let mut xrm_option: *mut c_char;
  let mut dpyinfo: *mut w32_display_info;

  CHECK_STRING (display);

  /* Signal an error in order to encourage correct use from callers.
   * If we ever support multiple window systems in the same Emacs,
   * we'll need callers to be precise about what window system they
   * want.  */

  if strcmp (SSDATA (display), c"w32".as_ptr ()) != 0
    { error (c"The name of the display in this Emacs must be \"w32\"".as_ptr ()); }

  /* If initialization has already been done, return now to avoid
     overwriting critical parts of one_w32_display_info.  */
  if window_system_available (core::ptr::null_mut ())
    { return Qnil (); }

  if ! NILP (xrm_string)
    { CHECK_STRING (xrm_string); }

  /* Allow color mapping to be defined externally; first look in user's
     HOME directory, then in Emacs etc dir for a file called rgb.txt. */
  {
    let mut color_file: Lisp_Object;

    color_file = build_string (c"~/rgb.txt".as_ptr ());

    if NILP (Ffile_readable_p (color_file))
      {
	color_file = Fexpand_file_name (build_string (c"rgb.txt".as_ptr ()),
					Fsymbol_value (Qdata_directory ()));
      }

    Vw32_color_map = Fx_load_color_file (color_file);
  }
  if NILP (Vw32_color_map)
    { Vw32_color_map = w32_default_color_map (); }

  /* Merge in system logical colors.  */
  add_system_logical_colors_to_map (&raw mut Vw32_color_map);

  if ! NILP (xrm_string)
    { xrm_option = SSDATA (xrm_string); }
  else
    { xrm_option = core::ptr::null_mut (); }

  /* Use this general default value to start with.  */
  /* First remove .exe suffix from invocation-name - it looks ugly. */
  {
    let mut basename: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut str_: *mut c_char;

    lispstpcpy (basename.as_mut_ptr (), Vinvocation_name);
    str_ = strrchr (basename.as_mut_ptr (), b'.' as c_int);
    if ! str_.is_null () { *str_ = 0; }
    Vinvocation_name = build_string (basename.as_mut_ptr ());
  }
  Vx_resource_name = Vinvocation_name;

  validate_x_resource_name ();

  /* This is what opens the connection and sets x_current_display.
     This also initializes many symbols, such as those used for input.  */
  dpyinfo = w32_term_init (display, xrm_option, SSDATA (Vx_resource_name));

  if dpyinfo.is_null ()
    {
      if ! NILP (must_succeed)
	{ fatal (c"Cannot connect to server %s.\n".as_ptr (), SDATA (display)); }
      else
	{ error (c"Cannot connect to server %s".as_ptr (), SDATA (display)); }
    }

  return Qnil ();
});

/// C 7246-7263: `x-close-connection` - C 7253-7254's reference-count test and its `error`, then
/// C 7256-7260's `block_input`/`image_destroy_all_bitmaps`/`w32_delete_display`/`unblock_input`.
/// C's `image_destroy_all_bitmaps` parameter is `dispextern.h` 3741's `Display_Info *`; each unit
/// declares the callee with its own display record (the `src/pgtkterm.rs` 6534-6544 device), so
/// the declaration at the end of this slice names `w32_display_info`.
/// C: DEFUN ("x-close-connection", Fx_close_connection, Sx_close_connection, 1, 1, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-close-connection", Fx_close_connection, Sx_close_connection, 1, 1, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);

  if (*dpyinfo).reference_count > 0
    { error (c"Display still has frames on it".as_ptr ()); }

  block_input ();
  image_destroy_all_bitmaps (dpyinfo);

  w32_delete_display (dpyinfo);
  unblock_input ();

  return Qnil ();
});

/// C 7265-7276: `x-display-list` - C's `(void)` argument list is the port's `()`, and C 7272's
/// `for (wdi = x_display_list; wdi; wdi = wdi->next)` is the file's `while ! ….is_null ()` walk
/// (the mirror carries `x_display_list` as a `static mut`, C 281/287's reading).
/// C: DEFUN ("x-display-list", Fx_display_list, Sx_display_list, 0, 0, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-display-list", Fx_display_list, Sx_display_list, 0, 0, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(),
	{
  let mut result: Lisp_Object = Qnil ();
  let mut wdi: *mut w32_display_info;

  wdi = x_display_list;
  while ! wdi.is_null ()
    {
      result = Fcons (XCAR ((*wdi).name_list_element), result);
      wdi = (*wdi).next;
    }

  return result;
});

/// C 7278-7283: `x-synchronize` - two unused parameters and a literal `Qnil`.
/// C: DEFUN ("x-synchronize", Fx_synchronize, Sx_synchronize, 1, 2, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object on, Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-synchronize", Fx_synchronize, Sx_synchronize, 1, 2, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(on: Lisp_Object, display: Lisp_Object),
	{
  return Qnil ();
});

/* C 7291-7388 is `#if 0 /* TODO : port window properties to W32 */` - C's own dead text, which
   the port keeps as C's own text (the C 1278-1417 reading of its own `#if 0` region): the three
   `DEFUN`s `x-change-window-property`, `x-delete-window-property` and `x-window-property` over
   Xlib's `XInternAtom`/`XChangeProperty`/`XGetWindowProperty`, none of which this port has (no
   Xlib mirror, and C's own preprocessor never compiles the block - which is why
   `extract-abi.py`'s checklist carries no subr row for the three: the `--check` reading of this
   file names no `missing subr` for them).

   #if 0 /* TODO : port window properties to W32 */
   
   DEFUN ("x-change-window-property", Fx_change_window_property,
          Sx_change_window_property, 2, 6, 0,
          doc: /* SKIP: real doc in xfns.c.  */)
     (Lisp_Object prop, Lisp_Object value, Lisp_Object frame,
      Lisp_Object type, Lisp_Object format, Lisp_Object outer_p)
   {
     struct frame *f = decode_window_system_frame (frame);
     Atom prop_atom;
   
     CHECK_STRING (prop);
     CHECK_STRING (value);
   
     block_input ();
     prop_atom = XInternAtom (FRAME_W32_DISPLAY (f), SDATA (prop), False);
     XChangeProperty (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f),
   		   prop_atom, XA_STRING, 8, PropModeReplace,
   		   SDATA (value), SCHARS (value));
   
     /* Make sure the property is set when we return.  */
     XFlush (FRAME_W32_DISPLAY (f));
     unblock_input ();
   
     return value;
   }
   
   
   DEFUN ("x-delete-window-property", Fx_delete_window_property,
          Sx_delete_window_property, 1, 2, 0,
          doc: /* SKIP: real doc in xfns.c.  */)
     (Lisp_Object prop, Lisp_Object frame)
   {
     struct frame *f = decode_window_system_frame (frame);
     Atom prop_atom;
   
     CHECK_STRING (prop);
     block_input ();
     prop_atom = XInternAtom (FRAME_W32_DISPLAY (f), SDATA (prop), False);
     XDeleteProperty (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f), prop_atom);
   
     /* Make sure the property is removed when we return.  */
     XFlush (FRAME_W32_DISPLAY (f));
     unblock_input ();
   
     return prop;
   }
   
   
   DEFUN ("x-window-property", Fx_window_property, Sx_window_property,
          1, 6, 0,
          doc: /* SKIP: real doc in xfns.c.  */)
     (Lisp_Object prop, Lisp_Object frame, Lisp_Object type,
      Lisp_Object source, Lisp_Object delete_p, Lisp_Object vector_ret_p)
   {
     struct frame *f = decode_window_system_frame (frame);
     Atom prop_atom;
     int rc;
     Lisp_Object prop_value = Qnil;
     char *tmp_data = NULL;
     Atom actual_type;
     int actual_format;
     unsigned long actual_size, bytes_remaining;
   
     CHECK_STRING (prop);
     block_input ();
     prop_atom = XInternAtom (FRAME_W32_DISPLAY (f), SDATA (prop), False);
     rc = XGetWindowProperty (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f),
   			   prop_atom, 0, 0, False, XA_STRING,
   			   &actual_type, &actual_format, &actual_size,
   			   &bytes_remaining, (unsigned char **) &tmp_data);
     if (rc == Success)
       {
         int size = bytes_remaining;
   
         XFree (tmp_data);
         tmp_data = NULL;
   
         rc = XGetWindowProperty (FRAME_W32_DISPLAY (f), FRAME_W32_WINDOW (f),
   			       prop_atom, 0, bytes_remaining,
   			       False, XA_STRING,
   			       &actual_type, &actual_format,
   			       &actual_size, &bytes_remaining,
   			       (unsigned char **) &tmp_data);
         if (rc == Success)
   	prop_value = make_string (tmp_data, size);
   
         XFree (tmp_data);
       }
   
     unblock_input ();
   
     return prop_value;
   
     return Qnil;
   }
   
   #endif /* TODO */
*/

/// C 7257's reader, declared the way `src/pgtkterm.rs` 6534-6544 declares its own copy (C's
/// `Display_Info` is `dispextern.h` 3741's name for the unit's display record).
/// C: extern void image_destroy_all_bitmaps (Display_Info *);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn image_destroy_all_bitmaps (dpyinfo: *mut w32_display_info);
}

/* C 7390-7392: C's Tool-tips banner, kept as C's own text (the C 3675-3701 reading of its
   banner comments).  The outer comment is the port's: C's banner closes a comment on its own
   last line, so the text needs one wrapper to stay a single comment in Rust (the C 1278-1417
   device, one nesting level shallower).

/***********************************************************************
				Tool tips
 ***********************************************************************/
 */

/* C 7394-7395 declares `compute_tip_xy` before its definition (C 7664-7761); Rust needs no
   forward declaration for a file-scope item, so C's prototype has no counterpart here (the
   PORTING.md 0.1-12 reading the `is_debugger_present` pair above already uses; the range table
   names this line as dropped).  */

/* C 7398-7415: the tool-tip objects.  The five `static Lisp_Object`s are zero-initialised in C,
   which is `Qnil` - the port's `lisp_h_Qnil ()` (`src/w32fns.rs` 5920's `w32_grabbed_keys`
   reading) - and C 7402's `Window tip_window;` is **not** static, so its definition carries
   `#[unsafe(no_mangle)]` (PORTING.md 0.2; the C 262-265 pair above is the same case, and
   `src/w32menu.c` reads the symbol).  */
/// C: static Lisp_Object tip_frame;
#[cfg (feature = "have_ntgui")]
static mut tip_frame: Lisp_Object = lisp_h_Qnil ();
/// C: Window tip_window;
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut tip_window: Window = core::ptr::null_mut ();
/// C: static Lisp_Object tip_timer;
#[cfg (feature = "have_ntgui")]
static mut tip_timer: Lisp_Object = lisp_h_Qnil ();
/// C: static Lisp_Object tip_last_string;
#[cfg (feature = "have_ntgui")]
static mut tip_last_string: Lisp_Object = lisp_h_Qnil ();
/// C: static Lisp_Object tip_last_frame;
#[cfg (feature = "have_ntgui")]
static mut tip_last_frame: Lisp_Object = lisp_h_Qnil ();
/// C: static Lisp_Object tip_last_parms;
#[cfg (feature = "have_ntgui")]
static mut tip_last_parms: Lisp_Object = lisp_h_Qnil ();

/// C 7418-7429: `unwind_create_tip_frame` - `unwind_create_frame`'s result drives the two resets
/// (C 7426-7427), and C 7476 of the next slice hands this function's address to
/// `record_unwind_protect`, so the definition carries the C ABI (the slice-7 correction of
/// `do_unwind_create_frame`; `src/androidfns.rs` 991 and `src/haikufns.rs` 1289 define the same
/// function the same way).
/// C: static void
/// C: unwind_create_tip_frame (Lisp_Object frame)
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn unwind_create_tip_frame (frame: Lisp_Object)
{
  let deleted: Lisp_Object;

  deleted = unwind_create_frame (frame);
  if EQ (deleted, Qt ())
    {
      tip_window = core::ptr::null_mut ();
      tip_frame = Qnil ();
    }
}

/// C 7432-7661: `w32_create_tip_frame` - the tooltip frame builder.  C 7459 assigns the
/// `parms` parameter, C 7483's `f->output_method` write is the file's `output_method!`, C 7493's
/// `FRAME_KBOARD (f) = kb;` writes through frame.h 919's `((f)->terminal->kboard)` (whose mirror
/// member is the untyped `*mut c_void`, so the cast is C's own type written out - the C 6289
/// reading), C 7490-7492's `#ifdef GLYPH_DEBUG` arm is the `glyph_debug` feature, C 7510-7513's
/// `#ifdef HAVE_HARFBUZZ` is `have_harfbuzz` (the C 6426-6432 block of slice 7), and the
/// `bool_bf` writes (C 7473's `wants_modeline`, C 7488's `tooltip`, C 7630's `no_split`,
/// C 7641's `can_set_window_size`) go through the mirror's flag table - slice 12's correction of
/// slices 6 and 7.  C 7603/7622/7624's `Fframe_parameter`/`Fmodify_frame_parameters` and C 7619's
/// `calln` are `include/globals.rs`'s and `include/lisp.rs` 3245's macro.
/// C: static Lisp_Object
/// C: w32_create_tip_frame (struct w32_display_info *dpyinfo, Lisp_Object parms)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_create_tip_frame (dpyinfo: *mut w32_display_info, parms: Lisp_Object)
  -> Lisp_Object
{
  let mut f: *mut frame;
  let mut frame: Lisp_Object;
  let mut name: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut kb: *mut kboard;
  let face_change_before: bool = face_change;
  let mut parms = parms;                       /* C 7459 assigns the parameter.  */

  /* Use this general default value to start with until we know if
     this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name;

  kb = (*((*dpyinfo).terminal)).kboard as *mut kboard;

  /* The calls to gui_display_get_arg remove elements from PARMS, so
     copy it to avoid destructive changes behind our caller's
     back.  */
  parms = Fcopy_alist (parms);

  /* Get the name of the frame to use for resource lookup.  */
  name = gui_display_get_arg (dpyinfo, parms, Qname (), c"name".as_ptr (), c"Name".as_ptr (),
			      RES_TYPE_STRING);
  if ! STRINGP (name)
      && ! BASE_EQ (name, Qunbound ())
      && ! NILP (name)
    { error (c"Invalid frame name--not a string or nil".as_ptr ()); }
  Vx_resource_name = name;

  frame = Qnil ();
  /* Make a frame without minibuffer nor mode-line.  */
  f = make_frame (false);
  frame_set_flag (f, FRAME_WANTS_MODELINE_UNIT, FRAME_WANTS_MODELINE_SHIFT, false as u32);
  XSETFRAME! (frame, f);

  record_unwind_protect (Some (unwind_create_tip_frame), frame);

  /* By setting the output method, we're essentially saying that
     the frame is live, as per FRAME_LIVE_P.  If we get a signal
     from this point on, w32_destroy_window might screw up reference
     counts etc.  */
  (*f).terminal = (*dpyinfo).terminal;
  output_method! (f, output_w32);
  (*f).output_data.w32 = xzalloc (core::mem::size_of::<w32_output> ());

  FRAME_FONTSET! (f) = -1;
  fset_icon_name (f, Qnil ());
  frame_set_flag (f, FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT, true as u32);

  /* C 7490-7492: `#ifdef GLYPH_DEBUG`.  */
  #[cfg (feature = "glyph_debug")]
  {
    dpyinfo_refcount = (*dpyinfo).reference_count;
  }
  (*((*f).terminal)).kboard = kb as *mut c_void;

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if BASE_EQ (name, Qunbound ()) || NILP (name)
    {
      fset_name (f, build_string ((*dpyinfo).w32_id_name));
      FRAME_EXPLICIT_NAME! (f, false);
    }
  else
    {
      fset_name (f, name);
      FRAME_EXPLICIT_NAME! (f, true);
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name (), name);
    }

  /* C 7510-7513: `#ifdef HAVE_HARFBUZZ`.  */
  #[cfg (feature = "have_harfbuzz")]
  {
    if harfbuzz_available != 0
      { register_font_driver (&raw mut harfbuzz_font_driver, f); }
  }
  if uniscribe_available != 0
    { register_font_driver (&raw mut uniscribe_font_driver, f); }
  register_font_driver (&raw mut w32font_driver, f);

  gui_default_parameter (f, parms, Qfont_backend (), Qnil (),
			 c"fontBackend".as_ptr (), c"FontBackend".as_ptr (),
			 RES_TYPE_STRING);

  /* Extract the window parameters from the supplied values
     that are needed to determine window geometry.  */
  w32_default_font_parameter (f, parms);

  gui_default_parameter (f, parms, Qborder_width (), make_fixnum (2),
			 c"borderWidth".as_ptr (), c"BorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  /* This defaults to 2 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if NILP (Fassq (Qinternal_border_width (), parms))
    {
      let value: Lisp_Object;

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width (),
				   c"internalBorder".as_ptr (), c"internalBorder".as_ptr (),
				   RES_TYPE_NUMBER);
      if ! BASE_EQ (value, Qunbound ())
	{ parms = Fcons (Fcons (Qinternal_border_width (), value), parms); }
    }

  gui_default_parameter (f, parms, Qinternal_border_width (), make_fixnum (1),
			 c"internalBorderWidth".as_ptr (), c"internalBorderWidth".as_ptr (),
			 RES_TYPE_NUMBER);
  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color (), build_string (c"black".as_ptr ()),
			 c"foreground".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color (), build_string (c"white".as_ptr ()),
			 c"background".as_ptr (), c"Background".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color (), build_string (c"black".as_ptr ()),
			 c"pointerColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qcursor_color (), build_string (c"black".as_ptr ()),
			 c"cursorColor".as_ptr (), c"Foreground".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color (), build_string (c"black".as_ptr ()),
			 c"borderColor".as_ptr (), c"BorderColor".as_ptr (), RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qno_special_glyphs (), Qt (),
			 core::ptr::null (), core::ptr::null (), RES_TYPE_BOOLEAN);

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);

  (*((*f).output_data.w32 as *mut w32_output)).dwStyle
    = WS_BORDER | WS_POPUP | WS_DISABLED;
  (*((*f).output_data.w32 as *mut w32_output)).parent_desc
    = (*FRAME_DISPLAY_INFO! (f)).root_window;
  (*((*f).output_data.w32 as *mut w32_output)).explicit_parent = false;

  gui_figure_window_size (f, parms, true, true);

  /* No fringes on tip frame.  */
  (*f).fringe_cols = 0;
  (*f).left_fringe_width = 0;
  (*f).right_fringe_width = 0;
  /* No dividers on tip frame.  */
  (*f).right_divider_width = 0;
  (*f).bottom_divider_width = 0;

  block_input ();
  my_create_tip_window (f);
  unblock_input ();

  w32_make_gc (f);

  gui_default_parameter (f, parms, Qauto_raise (), Qnil (),
			 c"autoRaise".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower (), Qnil (),
			 c"autoLower".as_ptr (), c"AutoRaiseLower".as_ptr (),
			 RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type (), Qbox (),
			 c"cursorType".as_ptr (), c"CursorType".as_ptr (),
			 RES_TYPE_SYMBOL);
  /* Process alpha here (Bug#17344).  */
  gui_default_parameter (f, parms, Qalpha (), Qnil (),
			 c"alpha".as_ptr (), c"Alpha".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background (), Qnil (),
			 c"alphaBackground".as_ptr (), c"AlphaBackground".as_ptr (),
			 RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qborders_respect_alpha_background (), Qnil (),
			 c"bordersRespectAlphaBackground".as_ptr (),
			 c"BordersRespectAlphaBackground".as_ptr (), RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qinhibit_double_buffering (), Qnil (),
			 c"inhibitDoubleBuffering".as_ptr (),
			 c"InhibitDoubleBuffering".as_ptr (), RES_TYPE_BOOLEAN);

  /* Add `tooltip' frame parameter's default value. */
  if NILP (Fframe_parameter (frame, Qtooltip ()))
    { Fmodify_frame_parameters (frame, Fcons (Fcons (Qtooltip (), Qt ()), Qnil ())); }

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which let's the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  {
    let bg: Lisp_Object = Fframe_parameter (frame, Qbackground_color ());
    let fg: Lisp_Object = Fframe_parameter (frame, Qforeground_color ());
    let mut colors: Lisp_Object = Qnil ();

    calln! (Qface_set_after_frame_default (), frame, Qnil ());

    if ! EQ (bg, Fframe_parameter (frame, Qbackground_color ()))
      { colors = Fcons (Fcons (Qbackground_color (), bg), colors); }
    if ! EQ (fg, Fframe_parameter (frame, Qforeground_color ()))
      { colors = Fcons (Fcons (Qforeground_color (), fg), colors); }

    if ! NILP (colors)
      { Fmodify_frame_parameters (frame, colors); }
  }

  frame_set_flag (f, FRAME_NO_SPLIT_UNIT, FRAME_NO_SPLIT_SHIFT, true as u32);

  /* Now that the frame is official, it counts as a reference to
     its display.  */
  (*FRAME_DISPLAY_INFO! (f)).reference_count += 1;
  (*((*f).terminal)).reference_count += 1;

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons (frame, Vframe_list);
  frame_set_flag (f, FRAME_CAN_SET_WINDOW_SIZE_UNIT, FRAME_CAN_SET_WINDOW_SIZE_SHIFT,
		  true as u32);
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qtip_frame ());

  /* Setting attributes of faces of the tooltip frame from resources
     and similar will set face_change, which leads to the
     clearing of all current matrices.  Since this isn't necessary
     here, avoid it by resetting face_change to the value it
     had before we created the tip frame.  */
  face_change = face_change_before;

  /* Discard the unwind_protect.  */
  return unbind_to (count, frame);
}

/// C 7663-7750: `compute_tip_xy` - the position computation C 7663's step-1 declaration of slice
/// 12 announced.  C 7672-7675's four `Fcdr (Fassq (…))` reads, C 7682's `POINT pt;`
/// (PORTING.md 1.5, zero-initialised), C 7687-7688's two pixel readings, C 7690-7694's
/// `block_input`/`GetCursorPos`/`unblock_input`, C 7702-7716's monitor clamp
/// (`monitor_from_point_fn`/`get_monitor_info_fn` as `Option`s, the C 498-503 device; C 7704's
/// `struct MONITOR_INFO info;` is the second PORTING.md 1.5 local) and C 7719-7749's two
/// `XR`/`YR` ladders - C's `*root_y = XFIXNUM (top)` narrows an `EMACS_INT` to the `int` the
/// caller passed, which the port writes with `as c_int`.
/// C: static void
/// C: compute_tip_xy (struct frame *f,
/// C: 		  Lisp_Object parms, Lisp_Object dx, Lisp_Object dy,
/// C: 		  int width, int height, int *root_x, int *root_y)
#[cfg (feature = "have_ntgui")]
unsafe fn compute_tip_xy (f: *mut frame, parms: Lisp_Object, dx: Lisp_Object, dy: Lisp_Object,
			  width: c_int, height: c_int, root_x: *mut c_int, root_y: *mut c_int)
{
  let mut left: Lisp_Object;
  let mut top: Lisp_Object;
  let mut right: Lisp_Object;
  let mut bottom: Lisp_Object;
  let mut min_x: c_int = 0;
  let mut min_y: c_int = 0;
  let mut max_x: c_int = 0;
  let mut max_y: c_int = 0;

  /* User-specified position?  */
  left = Fcdr (Fassq (Qleft (), parms));
  top = Fcdr (Fassq (Qtop (), parms));
  right = Fcdr (Fassq (Qright (), parms));
  bottom = Fcdr (Fassq (Qbottom (), parms));

  /* Move the tooltip window where the mouse pointer is.  Resize and
     show it.  */
  if (! FIXNUMP (left) && ! FIXNUMP (right))
      || (! FIXNUMP (top) && ! FIXNUMP (bottom))
    {
      let mut pt: POINT = core::mem::zeroed ();

      /* Default min and max values.  */
      min_x = 0;
      min_y = 0;
      max_x = w32_display_pixel_width (FRAME_DISPLAY_INFO! (f));
      max_y = w32_display_pixel_height (FRAME_DISPLAY_INFO! (f));

      block_input ();
      GetCursorPos (&raw mut pt);
      *root_x = pt.x;
      *root_y = pt.y;
      unblock_input ();

      /* If multiple monitor support is available, constrain the tip onto
	 the current monitor. This improves the above by allowing negative
	 coordinates if monitor positions are such that they are valid, and
	 snaps a tooltip onto a single monitor if we are close to the edge
	 where it would otherwise flow onto the other monitor (or into
	 nothingness if there is a gap in the overlap).  */
      if monitor_from_point_fn.is_some () && get_monitor_info_fn.is_some ()
	{
	  let mut info: MONITOR_INFO = core::mem::zeroed ();
	  let monitor: HMONITOR
	    = (monitor_from_point_fn.unwrap ()) (pt, MONITOR_DEFAULT_TO_NEAREST);
	  info.cbSize = core::mem::size_of::<MONITOR_INFO> () as DWORD;

	  if (get_monitor_info_fn.unwrap ()) (monitor, &raw mut info) != 0
	    {
	      min_x = info.rcWork.left;
	      min_y = info.rcWork.top;
	      max_x = info.rcWork.right;
	      max_y = info.rcWork.bottom;
	    }
	}
    }

  if FIXNUMP (top)
    { *root_y = XFIXNUM (top) as c_int; }
  else if FIXNUMP (bottom)
    { *root_y = XFIXNUM (bottom) as c_int - height; }
  else if *root_y + XFIXNUM (dy) as c_int <= min_y
    { *root_y = min_y; } /* Can happen for negative dy */
  else if *root_y + XFIXNUM (dy) as c_int + height <= max_y
    /* It fits below the pointer */
    { *root_y += XFIXNUM (dy) as c_int; }
  else if height + XFIXNUM (dy) as c_int + min_y <= *root_y
    /* It fits above the pointer.  */
    { *root_y -= height + XFIXNUM (dy) as c_int; }
  else
    /* Put it on the top.  */
    { *root_y = min_y; }

  if FIXNUMP (left)
    { *root_x = XFIXNUM (left) as c_int; }
  else if FIXNUMP (right)
    { *root_x = XFIXNUM (right) as c_int - width; }
  else if *root_x + XFIXNUM (dx) as c_int <= min_x
    { *root_x = 0; } /* Can happen for negative dx */
  else if *root_x + XFIXNUM (dx) as c_int + width <= max_x
    /* It fits to the right of the pointer.  */
    { *root_x += XFIXNUM (dx) as c_int; }
  else if width + XFIXNUM (dx) as c_int + min_x <= *root_x
    /* It fits to the left of the pointer.  */
    { *root_x -= width + XFIXNUM (dx) as c_int; }
  else
    /* Put it left justified on the screen -- it ought to fit that way.  */
    { *root_x = min_x; }
}

/**
 * w32_hide_tip:
 *
 * Hide currently visible tooltip and cancel its timer.
 *
 * This will try to make tooltip_frame invisible (if DELETE is false)
 * or delete tooltip_frame (if DELETE is true).
 *
 * Return Qt if the tooltip was either deleted or made invisible, Qnil
 * otherwise.
 */
/// C 7763-7806: `w32_hide_tip` - C 7768's `calln (Qcancel_timer, tip_timer)` is the port's
/// `calln!`, C 7780's `specpdl_ref count` C's own declaration order (after `was_open`), and
/// C 7790's `delete_frame` / C 7794's `w32_make_frame_invisible` are declared above / in
/// `include/w32term.rs`.
/// C: static Lisp_Object
/// C: w32_hide_tip (bool delete)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_hide_tip (delete: bool) -> Lisp_Object
{
  if ! NILP (tip_timer)
    {
      calln! (Qcancel_timer (), tip_timer);
      tip_timer = Qnil ();
    }

  if NILP (tip_frame)
      || (! delete && FRAMEP (tip_frame) && ! FRAME_VISIBLE_P (XFRAME (tip_frame)))
    { return Qnil (); }
  else
    {
      let mut was_open: Lisp_Object = Qnil ();

      let count: specpdl_ref = SPECPDL_INDEX ();
      specbind (Qinhibit_redisplay (), Qt ());
      specbind (Qinhibit_quit (), Qt ());

      if FRAMEP (tip_frame)
	{
	  if FRAME_LIVE_P (XFRAME (tip_frame))
	    {
	      if delete
		{
		  delete_frame (tip_frame, Qnil ());
		  tip_frame = Qnil ();
		}
	      else
		{ w32_make_frame_invisible (XFRAME (tip_frame)); }

	      was_open = Qt ();
	    }
	  else
	    { tip_frame = Qnil (); }
	}
      else
	{ tip_frame = Qnil (); }

      return unbind_to (count, was_open);
    }
}

/// C 7809-8106: `x-show-tip` - the file's largest `DEFUN` after `x-create-frame`, and the w32
/// sibling of `src/androidfns.rs`'s `Fx_show_tip` (C 2301-2583 there) and `src/haikufns.rs`'s:
/// the same six-argument shape, C's `goto start_timer` (C 7884) and its label (C 8101) as the
/// labelled block `'start_timer` whose `break 'start_timer` is the jump (PORTING.md 2.2), C's six
/// reassigned parameters re-bound with `let mut` (the `DEFUN!` macro's `$arg:ident` cannot say
/// `mut`; `docs/ports/macro-body-notes.tsv` records that), C 7816's `int root_x, root_y;` and
/// C 7819's `int width, height;` as PORTING.md 1.5 locals, C 7818's `struct text_pos pos;` as the
/// `text_pos` mirror's zeroed value, C 7849's `FRAME_VISIBLE_P`, C 7880/8089's
/// `SET_FRAME_VISIBLE!`, C 8019's `FRAME_TOTAL_COLS!` and C 8035's `SET_TEXT_POS!` (the two
/// unit-local macros above), C 8036's `try_window` with
/// dispextern.h 3700's `TRY_WINDOW_IGNORE_FONTS_CHANGE`, C 8039-8051's `#ifdef ENABLE_CHECKING`
/// arm (`enable_checking`, the `src/androidfns.rs` 3021-3025 shape) and C 8032's `Finsert (1,
/// &string)` (the port's `Finsert (1, &raw mut string)`, the sibling's spelling).
/// C: DEFUN ("x-show-tip", Fx_show_tip, Sx_show_tip, 1, 6, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object string, Lisp_Object frame, Lisp_Object parms,
/// C:    Lisp_Object timeout, Lisp_Object dx, Lisp_Object dy)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-show-tip", Fx_show_tip, Sx_show_tip, 1, 6, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(string: Lisp_Object, frame: Lisp_Object, parms: Lisp_Object, timeout: Lisp_Object,
	 dx: Lisp_Object, dy: Lisp_Object),
	{
  let mut tip_f: *mut frame;
  let mut w: *mut window;
  let mut root_x: c_int = 0;
  let mut root_y: c_int = 0;
  let old_buffer: *mut buffer;
  let mut pos: text_pos = core::mem::zeroed ();
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  let old_windows_or_buffers_changed: c_int = windows_or_buffers_changed;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut window: Lisp_Object;
  let mut size: Lisp_Object;
  let tip_buf: Lisp_Object;
  AUTO_STRING! (tip, c" *tip*".as_ptr ());

  /* C's `goto start_timer` (C 7884) and its label (C 8101) become this labelled block.  */
  let mut string = string;
  let mut frame = frame;
  let mut parms = parms;
  let mut timeout = timeout;
  let mut dx = dx;
  let mut dy = dy;

  'start_timer: {
  specbind (Qinhibit_redisplay (), Qt ());

  CHECK_STRING (string);

  if NILP (frame)
    { frame = selected_frame; }
  decode_window_system_frame (frame);

  if NILP (timeout)
    { timeout = Vx_show_tooltip_timeout; }
  CHECK_FIXNAT (timeout);

  if NILP (dx)
    { dx = make_fixnum (5); }
  else
    { CHECK_FIXNUM (dx); }

  if NILP (dy)
    { dy = make_fixnum (-10); }
  else
    { CHECK_FIXNUM (dy); }

  if FRAMEP (tip_frame) && FRAME_LIVE_P (XFRAME (tip_frame))
    {
      if FRAME_VISIBLE_P (XFRAME (tip_frame))
	  && EQ (frame, tip_last_frame)
	  && ! NILP (Fequal_including_properties (string, tip_last_string))
	  && ! NILP (Fequal (parms, tip_last_parms))
	{
	  /* Only DX and DY have changed.  */
	  tip_f = XFRAME (tip_frame);
	  if ! NILP (tip_timer)
	    {
	      let timer: Lisp_Object = tip_timer;

	      tip_timer = Qnil ();
	      calln! (Qcancel_timer (), timer);
	    }

	  block_input ();
	  compute_tip_xy (tip_f, parms, dx, dy, FRAME_PIXEL_WIDTH (tip_f),
			  FRAME_PIXEL_HEIGHT (tip_f), &raw mut root_x, &raw mut root_y);

	  /* Put tooltip in topmost group and in position.  */
	  SetWindowPos (FRAME_W32_WINDOW! (tip_f), HWND_TOPMOST,
			root_x, root_y, 0, 0,
			SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER);

	  /* Ensure tooltip is on top of other topmost windows (eg menus).  */
	  SetWindowPos (FRAME_W32_WINDOW! (tip_f), HWND_TOP,
			0, 0, 0, 0,
			SWP_NOMOVE | SWP_NOSIZE
			| SWP_NOACTIVATE | SWP_NOOWNERZORDER);

	  /* Let redisplay know that we have made the frame visible already.  */
	  SET_FRAME_VISIBLE! (tip_f, 1);
	  ShowWindow (FRAME_W32_WINDOW! (tip_f), SW_SHOWNOACTIVATE);
	  unblock_input ();

	  break 'start_timer;
	}
      else if tooltip_reuse_hidden_frame && EQ (frame, tip_last_frame)
	{
	  let mut delete: bool = false;
	  let mut tail: Lisp_Object;
	  let mut elt: Lisp_Object;
	  let mut parm: Lisp_Object;
	  let mut last: Lisp_Object;

	  /* Check if every parameter in PARMS has the same value in
	     tip_last_parms.  This may destruct tip_last_parms
	     which, however, will be recreated below.  */
	  tail = parms;
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);
	      parm = Fcar (elt);
	      /* The left, top, right and bottom parameters are handled
		 by compute_tip_xy so they can be ignored here.  */
	      if ! EQ (parm, Qleft ()) && ! EQ (parm, Qtop ())
		  && ! EQ (parm, Qright ()) && ! EQ (parm, Qbottom ())
		{
		  last = Fassq (parm, tip_last_parms);
		  if NILP (Fequal (Fcdr (elt), Fcdr (last)))
		    {
		      /* We lost, delete the old tooltip.  */
		      delete = true;
		      break;
		    }
		  else
		    { tip_last_parms = calln! (Qassq_delete_all (), parm, tip_last_parms); }
		}
	      else
		{ tip_last_parms = calln! (Qassq_delete_all (), parm, tip_last_parms); }
	      tail = XCDR (tail);
	    }

	  /* Now check if there's a parameter left in tip_last_parms with a
	     non-nil value.  */
	  tail = tip_last_parms;
	  while CONSP (tail)
	    {
	      elt = XCAR (tail);
	      parm = Fcar (elt);
	      if ! EQ (parm, Qleft ()) && ! EQ (parm, Qtop ()) && ! EQ (parm, Qright ())
		  && ! EQ (parm, Qbottom ()) && ! NILP (Fcdr (elt))
		{
		  /* We lost, delete the old tooltip.  */
		  delete = true;
		  break;
		}
	      tail = XCDR (tail);
	    }

	  w32_hide_tip (delete);
	}
      else
	{ w32_hide_tip (true); }
    }
  else
    { w32_hide_tip (true); }

  tip_last_frame = frame;
  tip_last_string = string;
  tip_last_parms = parms;

  /* Block input until the tip has been fully drawn, to avoid crashes
     when drawing tips in menus.  */
  block_input ();

  if ! FRAMEP (tip_frame) || ! FRAME_LIVE_P (XFRAME (tip_frame))
    {
      /* Add default values to frame parameters.  */
      if NILP (Fassq (Qname (), parms))
	{ parms = Fcons (Fcons (Qname (), build_string (c"tooltip".as_ptr ())), parms); }
      if NILP (Fassq (Qinternal_border_width (), parms))
	{ parms = Fcons (Fcons (Qinternal_border_width (), make_fixnum (3)), parms); }
      if NILP (Fassq (Qborder_width (), parms))
	{ parms = Fcons (Fcons (Qborder_width (), make_fixnum (1)), parms); }
      if NILP (Fassq (Qborder_color (), parms))
	{
	  parms = Fcons (Fcons (Qborder_color (), build_string (c"lightyellow".as_ptr ())),
			 parms);
	}
      if NILP (Fassq (Qbackground_color (), parms))
	{
	  parms = Fcons (Fcons (Qbackground_color (), build_string (c"lightyellow".as_ptr ())),
			 parms);
	}

      /* Create a frame for the tooltip and record it in the global
	 variable tip_frame.  */
      let f: *mut frame;		/* The value is unused.  */
      tip_frame = w32_create_tip_frame (FRAME_DISPLAY_INFO! (f), parms);
      if NILP (tip_frame)
	{
	  /* Creating the tip frame failed.  */
	  unblock_input ();
	  return unbind_to (count, Qnil ());
	}
    }
  else
    /* Required by X11 drag and drop, and left here in the interests of
       consistency and in the event drag and drop should be implemented
       on W32.  */
    { tip_window = FRAME_W32_WINDOW! (XFRAME (tip_frame)); }

  tip_f = XFRAME (tip_frame);
  window = FRAME_ROOT_WINDOW (tip_f);
  tip_buf = Fget_buffer_create (tip, Qnil ());
  /* We will mark the tip window a "pseudo-window" below, and such
     windows cannot have display margins.  */
  bset_left_margin_cols (XBUFFER (tip_buf), make_fixnum (0));
  bset_right_margin_cols (XBUFFER (tip_buf), make_fixnum (0));
  set_window_buffer (window, tip_buf, false, false);
  w = XWINDOW (window);
  (*w).pseudo_window_p = true;
  /* Try to avoid that `other-window' select us (Bug#47207).  */
  Fset_window_parameter (window, Qno_other_window (), Qt ());

  /* Set up the frame's root window.  Note: The following code does not
     try to size the window or its frame correctly.  Its only purpose is
     to make the subsequent text size calculations work.  The right
     sizes should get installed when the toolkit gets back to us.  */
  (*w).left_col = 0;
  (*w).top_line = 0;
  (*w).pixel_left = 0;
  (*w).pixel_top = 0;

  if CONSP (Vx_max_tooltip_size)
      && RANGED_FIXNUMP (1, XCAR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)
      && RANGED_FIXNUMP (1, XCDR (Vx_max_tooltip_size), INT_MAX as EMACS_INT)
    {
      (*w).total_cols = XFIXNAT (XCAR (Vx_max_tooltip_size)) as c_int;
      (*w).total_lines = XFIXNAT (XCDR (Vx_max_tooltip_size)) as c_int;
    }
  else
    {
      (*w).total_cols = 80;
      (*w).total_lines = 40;
    }

  (*w).pixel_width = (*w).total_cols * FRAME_COLUMN_WIDTH (tip_f);
  (*w).pixel_height = (*w).total_lines * FRAME_LINE_HEIGHT (tip_f);
  FRAME_TOTAL_COLS! (tip_f, WINDOW_TOTAL_COLS (w));
  adjust_frame_glyphs (tip_f);

  /* Insert STRING into the root window's buffer and fit the frame to
     the buffer.  */
  let count_1: specpdl_ref = SPECPDL_INDEX ();
  old_buffer = current_buffer! ();
  set_buffer_internal_1 (XBUFFER ((*w).contents));
  bset_truncate_lines (current_buffer! (), Qnil ());
  specbind (Qinhibit_read_only (), Qt ());
  specbind (Qinhibit_modification_hooks (), Qt ());
  specbind (Qinhibit_point_motion_hooks (), Qt ());
  Ferase_buffer ();
  Finsert (1, &raw mut string);
  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
  clear_glyph_matrix ((*w).current_matrix as *mut glyph_matrix);
  SET_TEXT_POS! (pos, BEGV! (), BEGV_BYTE! ());
  let displayed: bool = try_window (window, pos, TRY_WINDOW_IGNORE_FONTS_CHANGE) != 0;
  if ! displayed && NILP (Vx_max_tooltip_size)
    {
      /* C 8039-8051: `#ifdef ENABLE_CHECKING`.  */
      #[cfg (feature = "enable_checking")]
      {
	let mut row: *mut glyph_row =
	  (*((*w).desired_matrix as *mut glyph_matrix)).rows;
	let end: *mut glyph_row =
	  (*((*w).desired_matrix as *mut glyph_matrix)).rows.offset
	    ((*((*w).desired_matrix as *mut glyph_matrix)).nrows as isize);
	while row < end
	  {
	    if ! (*row).displays_text_p
		|| (*row).ends_at_zv_p
	      { break; }
	    row = row.offset (1);
	  }
	eassert! (row < end && (*row).ends_at_zv_p);
      }
    }
  /* Calculate size of tooltip window.  */
  size = Fwindow_text_pixel_size (window, Qnil (), Qnil (), Qnil (),
				  make_fixnum ((*w).pixel_height as EMACS_INT), Qnil (),
				  Qnil ());
  /* Add the frame's internal border to calculated size.  */
  width = XFIXNUM (Fcar (size)) as c_int + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);
  height = XFIXNUM (Fcdr (size)) as c_int + 2 * FRAME_INTERNAL_BORDER_WIDTH (tip_f);
  /* Calculate position of tooltip frame.  */
  compute_tip_xy (tip_f, parms, dx, dy, width, height, &raw mut root_x, &raw mut root_y);

  /* Show tooltip frame.  */
  {
    let mut rect: RECT = core::mem::zeroed ();
    let pad: c_int = if FIXNUMP (Vw32_tooltip_extra_pixels)
      { max (0, XFIXNUM (Vw32_tooltip_extra_pixels) as c_int) }
      else
      { FRAME_COLUMN_WIDTH (tip_f) };

    rect.left = 0;
    rect.top = 0;
    rect.right = width;
    rect.bottom = height;
    AdjustWindowRect (&raw mut rect, (*((*tip_f).output_data.w32 as *mut w32_output)).dwStyle,
		      FRAME_EXTERNAL_MENU_BAR (tip_f) as BOOL);

    /* Position and size tooltip and put it in the topmost group.  */
    SetWindowPos (FRAME_W32_WINDOW! (tip_f), HWND_TOPMOST,
		  root_x, root_y,
		  rect.right - rect.left + pad,
		  rect.bottom - rect.top, SWP_NOACTIVATE | SWP_NOOWNERZORDER);

    /* Ensure tooltip is on top of other topmost windows (eg menus).  */
    SetWindowPos (FRAME_W32_WINDOW! (tip_f), HWND_TOP,
		  0, 0, 0, 0,
		  SWP_NOMOVE | SWP_NOSIZE
		  | SWP_NOACTIVATE | SWP_NOOWNERZORDER);

    /* Let redisplay know that we have made the frame visible already.  */
    SET_FRAME_VISIBLE! (tip_f, 1);

    ShowWindow (FRAME_W32_WINDOW! (tip_f), SW_SHOWNOACTIVATE);
  }

  (*w).must_be_updated_p = true;
  update_single_window (w);
  set_buffer_internal_1 (old_buffer);
  unbind_to (count_1, Qnil ());
  unblock_input ();
  windows_or_buffers_changed = old_windows_or_buffers_changed;
  }   /* 'start_timer: (C 8101's label) */

  /* Let the tip disappear after timeout seconds.  */
  tip_timer = calln! (Qrun_at_time (), timeout, Qnil (), Qx_hide_tip ());

  return unbind_to (count, Qnil ());
});

/// C 8109-8114: `x-hide-tip` - one call, with C 8113's `!tooltip_reuse_hidden_frame` as the
/// argument (C's `int` variable negated gives 0/1, i.e. the port's `== 0`).
/// C: DEFUN ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(),
	{
  return w32_hide_tip (! tooltip_reuse_hidden_frame);
});

/* C 8116-8118: C's File-selection-dialog banner, kept as C's own text inside one wrapper
   comment (the C 7390-7392 device: C's banner closes a comment on its own last line).

/***********************************************************************
			File selection dialog
 ***********************************************************************/
 */

/// C 8120-8122: the three `<dlgs.h>` control ids C's `#define`s name -
/// `#define FILE_NAME_TEXT_FIELD edt1` / `#define FILE_NAME_COMBO_BOX cmb13` /
/// `#define FILE_NAME_LIST lst1`.  The port keeps C's three names *and* the header's own ids,
/// because C's macro bodies are exactly those id tokens: the `FILE_NAME_*` constants alias the
/// three `<dlgs.h>` values the way C's `#define`s alias the header's macros.
#[cfg (feature = "have_ntgui")]
pub const edt1: c_int = 0x0480;			/* `<dlgs.h>` */
#[cfg (feature = "have_ntgui")]
pub const cmb13: c_int = 0x047C;		/* `<dlgs.h>` */
#[cfg (feature = "have_ntgui")]
pub const lst1: c_int = 0x0460;			/* `<dlgs.h>` */
#[cfg (feature = "have_ntgui")]
pub const FILE_NAME_TEXT_FIELD: c_int = edt1;
#[cfg (feature = "have_ntgui")]
pub const FILE_NAME_COMBO_BOX: c_int = cmb13;
#[cfg (feature = "have_ntgui")]
pub const FILE_NAME_LIST: c_int = lst1;

/// C: #define WM_NOTIFY 0x004E / #define GW_CHILD 5 - the two `<winuser.h>` ids C 8132/8169 use.
#[cfg (feature = "have_ntgui")]
pub const WM_NOTIFY: UINT = 0x004E;
#[cfg (feature = "have_ntgui")]
pub const GW_CHILD: UINT = 5;

/// C: #define CDN_FIRST (0U-601U) / #define CDN_INITDONE (CDN_FIRST - 0x0000) / #define
/// C: CDN_TYPECHANGE (CDN_FIRST - 0x0006) / #define CDM_FIRST (WM_USER + 100) / #define
/// C: CDM_SETCONTROLTEXT (CDM_FIRST + 0x0004) - the five `<commdlg.h>` ids C 8147-8207 passes
/// around (the `-601`/`-607` forms are the unsigned 0xFFFFFDA7/0xFFFFFDA1 the header writes).
#[cfg (feature = "have_ntgui")]
pub const CDN_FIRST: UINT = 0xFFFFFDA7;
#[cfg (feature = "have_ntgui")]
pub const CDN_INITDONE: UINT = CDN_FIRST;
#[cfg (feature = "have_ntgui")]
pub const CDN_TYPECHANGE: UINT = CDN_FIRST - 0x0006;
#[cfg (feature = "have_ntgui")]
pub const CDM_FIRST: UINT = 0x0464;
#[cfg (feature = "have_ntgui")]
pub const CDM_SETCONTROLTEXT: UINT = CDM_FIRST + 0x0004;

/// C: typedef struct tagNMHDR { HWND hwndFrom; UINT_PTR idFrom; UINT code; } NMHDR - the
/// notification header the two `OFNOTIFY` records below start with (`<commdlg.h>`/`<winuser.h>`;
/// only its `code` member is read here).
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NMHDR
{
  pub hwndFrom: HWND,
  pub idFrom: UINT_PTR,
  pub code: UINT,
}

/// C: typedef struct _OFNOTIFYA { NMHDR hdr; LPOPENFILENAMEA lpOFN; LPSTR pszFile; } OFNOTIFYA;
/// C: typedef struct _OFNOTIFYW { NMHDR hdr; LPOPENFILENAMEW lpOFN; LPWSTR pszFile; } OFNOTIFYW;
/// - the two records C 8134-8135 casts C's `LPARAM` to.  `LPOFNHOOKPROC` is the function-pointer
/// typedef both `OPENFILENAME` records carry, spelled as the file's other `WINAPI` pointers are
/// (an `Option<unsafe extern "system" fn …>`).
#[cfg (feature = "have_ntgui")]
pub type LPOFNHOOKPROC
  = Option<unsafe extern "system" fn (hwnd: HWND, msg: UINT, wParam: WPARAM,
				      lParam: LPARAM) -> UINT_PTR>;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA
{
  pub lStructSize: DWORD,
  pub hwndOwner: HWND,
  pub hInstance: HINSTANCE,
  pub lpstrFilter: *const c_char,
  pub lpstrCustomFilter: *mut c_char,
  pub nMaxCustFilter: DWORD,
  pub nFilterIndex: DWORD,
  pub lpstrFile: *mut c_char,
  pub nMaxFile: DWORD,
  pub lpstrFileTitle: *mut c_char,
  pub nMaxFileTitle: DWORD,
  pub lpstrInitialDir: *const c_char,
  pub lpstrTitle: *const c_char,
  pub Flags: DWORD,
  pub nFileOffset: WORD,
  pub nFileExtension: WORD,
  pub lpstrDefExt: *const c_char,
  pub lCustData: LPARAM,
  pub lpfnHook: LPOFNHOOKPROC,
  pub lpTemplateName: *const c_char,
  pub pvReserved: *mut c_void,
  pub dwReserved: DWORD,
  pub FlagsEx: DWORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW
{
  pub lStructSize: DWORD,
  pub hwndOwner: HWND,
  pub hInstance: HINSTANCE,
  pub lpstrFilter: *const c_ushort,
  pub lpstrCustomFilter: *mut c_ushort,
  pub nMaxCustFilter: DWORD,
  pub nFilterIndex: DWORD,
  pub lpstrFile: *mut c_ushort,
  pub nMaxFile: DWORD,
  pub lpstrFileTitle: *mut c_ushort,
  pub nMaxFileTitle: DWORD,
  pub lpstrInitialDir: *const c_ushort,
  pub lpstrTitle: *const c_ushort,
  pub Flags: DWORD,
  pub nFileOffset: WORD,
  pub nFileExtension: WORD,
  pub lpstrDefExt: *const c_ushort,
  pub lCustData: LPARAM,
  pub lpfnHook: LPOFNHOOKPROC,
  pub lpTemplateName: *const c_ushort,
  pub pvReserved: *mut c_void,
  pub dwReserved: DWORD,
  pub FlagsEx: DWORD,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA
{
  pub hdr: NMHDR,
  pub lpOFN: *mut OPENFILENAMEA,
  pub pszFile: *mut c_char,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW
{
  pub hdr: NMHDR,
  pub lpOFN: *mut OPENFILENAMEW,
  pub pszFile: *mut c_ushort,
}

/// C 8132-8216's readers (`<winuser.h>`, the `AdjustWindowRect` block's sibling): `GetParent`,
/// `GetDlgItem`, `GetWindow`, the two `SendMessage` arms C's `W`/`A` pair names, `EnableWindow`
/// and `SetFocus` (the last is already declared above).
/// C: HWND WINAPI GetParent (HWND hWnd); HWND WINAPI GetDlgItem (HWND hDlg, int nIDDlgItem);
/// C: HWND WINAPI GetWindow (HWND hWnd, UINT uCmd); LRESULT WINAPI SendMessageW (HWND hWnd, UINT
/// C: Msg, WPARAM wParam, LPARAM lParam); LRESULT WINAPI SendMessageA (…); BOOL WINAPI
/// C: EnableWindow (HWND hWnd, BOOL bEnable);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetParent (hWnd: HWND) -> HWND;
  pub fn GetDlgItem (hDlg: HWND, nIDDlgItem: c_int) -> HWND;
  pub fn GetWindow (hWnd: HWND, uCmd: UINT) -> HWND;
  pub fn SendMessageW (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn SendMessageA (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn EnableWindow (hWnd: HWND, bEnable: BOOL) -> BOOL;
}

/// C 8190/8193 and C 8211/8214 spell two **wide** string literals inline (`L"Current Directory"`
/// and `L""`), which Rust has no syntax for: the port carries the same UTF-16 code units in the
/// two arrays below (host-endian, as C's `L"…"` is; the trailing `0x0000` is C's terminator) and
/// passes their address - the `DARK_MODE_APP_NAME` reading at the top of this file.
#[cfg (feature = "have_ntgui")]
pub const CURRENT_DIRECTORY_L: [u16; 18] =
  [0x0043, 0x0075, 0x0072, 0x0072, 0x0065, 0x006E, 0x0074, 0x0020, 0x0044, 0x0069, 0x0072,
   0x0065, 0x0063, 0x0074, 0x006F, 0x0072, 0x0079, 0x0000];
#[cfg (feature = "have_ntgui")]
pub const EMPTY_L: [u16; 1] = [0x0000];

/* C 8124-8128's comment.  */
/// C 8129-8221: `file_dialog_callback` - C's `static UINT_PTR CALLBACK` (the `extern "system"`
/// convention C's `CALLBACK` names, the `src/w32font.rs` 1958-1966 device).  C 8138-8142's
/// `#ifdef NTGUI_UNICODE` carries the `cygwin` feature (the `include/w32term.rs` NTGUI_UNICODE
/// note: `conf_post.h` 169-170 defines it only for `CYGWIN && HAVE_NTGUI`, so the `#else` arm is
/// the live one and both stand); C 8146-8152's `dropdown_changed` is C's `int` assigned from a
/// `||`, i.e. the port's `as c_int`.
/// C: static UINT_PTR CALLBACK
/// C: file_dialog_callback (HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn file_dialog_callback (hwnd: HWND, msg: UINT, wParam: WPARAM,
						lParam: LPARAM) -> UINT_PTR
{
  if msg == WM_NOTIFY
    {
      let notify_w: *mut OFNOTIFYW = lParam as *mut OFNOTIFYW;
      let notify_a: *mut OFNOTIFYA = lParam as *mut OFNOTIFYA;
      let mut dropdown_changed: c_int;
      let mut dir_index: c_int;
      #[cfg (feature = "cygwin")]
      const use_unicode: c_int = 1;
      #[cfg (not (feature = "cygwin"))]
      let use_unicode: c_int = w32_unicode_filenames as c_int;

      /* Detect when the Filter dropdown is changed.  */
      if use_unicode != 0
	{
	  dropdown_changed = ((*notify_w).hdr.code == CDN_TYPECHANGE
			      || (*notify_w).hdr.code == CDN_INITDONE) as c_int;
	}
      else
	{
	  dropdown_changed = ((*notify_a).hdr.code == CDN_TYPECHANGE
			      || (*notify_a).hdr.code == CDN_INITDONE) as c_int;
	}
      if dropdown_changed != 0
	{
	  let dialog: HWND = GetParent (hwnd);
	  let mut edit_control: HWND = GetDlgItem (dialog, FILE_NAME_TEXT_FIELD);
	  let list: HWND = GetDlgItem (dialog, FILE_NAME_LIST);
	  let mut hdr_code: c_int;

	  /* At least on Windows 7, the above attempt to get the window handle
	     to the File Name Text Field fails.	 The following code does the
	     job though.  Note that this code is based on my examination of the
	     window hierarchy using Microsoft Spy++.  bk */
	  if edit_control.is_null ()
	    {
	      let mut tmp: HWND = GetDlgItem (dialog, FILE_NAME_COMBO_BOX);
	      if ! tmp.is_null ()
		{
		  tmp = GetWindow (tmp, GW_CHILD);
		  if ! tmp.is_null ()
		    { edit_control = GetWindow (tmp, GW_CHILD); }
		}
	    }

	  /* Directories is in index 2.	 */
	  if use_unicode != 0
	    {
	      dir_index = (*(*notify_w).lpOFN).nFilterIndex as c_int;
	      hdr_code = (*notify_w).hdr.code as c_int;
	    }
	  else
	    {
	      dir_index = (*(*notify_a).lpOFN).nFilterIndex as c_int;
	      hdr_code = (*notify_a).hdr.code as c_int;
	    }
	  if dir_index == 2
	    {
	      if use_unicode != 0
		{
		  SendMessageW (dialog, CDM_SETCONTROLTEXT, FILE_NAME_TEXT_FIELD as WPARAM,
				(CURRENT_DIRECTORY_L.as_ptr ()) as LPARAM);
		}
	      else
		{
		  SendMessageA (dialog, CDM_SETCONTROLTEXT, FILE_NAME_TEXT_FIELD as WPARAM,
				(c"Current Directory".as_ptr ()) as LPARAM);
		}
	      EnableWindow (edit_control, FALSE);
	      /* Note that at least on Windows 7, the above call to EnableWindow
		 disables the window that would ordinarily have focus.	If we
		 do not set focus to some other window here, focus will land in
		 no man's land and the user will be unable to tab through the
		 dialog box (pressing tab will only result in a beep).
		 Avoid that problem by setting focus to the list here.	*/
	      if hdr_code == CDN_INITDONE as c_int
		{ SetFocus (list); }
	    }
	  else
	    {
	      /* Don't override default filename on init done.  */
	      if hdr_code == CDN_TYPECHANGE as c_int
		{
		  if use_unicode != 0
		    {
		      SendMessageW (dialog, CDM_SETCONTROLTEXT,
				    FILE_NAME_TEXT_FIELD as WPARAM,
				    (EMPTY_L.as_ptr ()) as LPARAM);
		    }
		  else
		    {
		      SendMessageA (dialog, CDM_SETCONTROLTEXT,
				    FILE_NAME_TEXT_FIELD as WPARAM,
				    (c"".as_ptr ()) as LPARAM);
		    }
		}
	      EnableWindow (edit_control, TRUE);
	    }
	}
    }
  return 0;
}

/**
 * w32_dialog_in_progress:
 *
 * This function is called by Fx_file_dialog and Fx_select_font and
 * serves to temporarily remove any Emacs frame currently in the
 * 'above' z-group from that group to assure that such a frame does
 * not hide the dialog window.  Frames that are temporarily removed
 * from the 'above' group have their z_group bit-field set to
 * z_group_above_suspended.  Any such frame is moved back to the
 * 'above' group as soon as the dialog finishes and has its z_group
 * bit-field reset to z_group_above.
 *
 * This function does not affect the z-order or the z-group state of
 * the dialog window itself.
 */
/// C 8238-8262: `w32_dialog_in_progress` - C's non-`static` `void`, so a `pub` definition with
/// `#[unsafe(no_mangle)]` (PORTING.md 0.2; `src/w32proc.rs`/`src/w32font.rs` call it);
/// C 8245's `if (!w32_disable_double_buffering)` is the mirror's `bool` global, and C 8253's
/// `FOR_EACH_FRAME` is the three-argument macro.
/// C: void
/// C: w32_dialog_in_progress (Lisp_Object in_progress)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_dialog_in_progress (in_progress: Lisp_Object)
{
  let mut frames: Lisp_Object;
  let mut frame: Lisp_Object;

  /* Indicate to w32_wnd_proc that the selection dialog is about to be
     open (or was closed, if IN_PROGRESS is nil).  */
  if ! w32_disable_double_buffering
    {
      enter_crit ();
      w32_selection_dialog_open = ! NILP (in_progress);
      leave_crit ();
    }

  /* Don't let frames in `above' z-group obscure dialog windows.  */
  FOR_EACH_FRAME! (frames, frame, {
      let f: *mut frame = XFRAME (frame);

      if ! NILP (in_progress) && FRAME_Z_GROUP_ABOVE (f)
	{ w32_set_z_group (f, Qabove_suspended (), Qabove ()); }
      else if NILP (in_progress) && FRAME_Z_GROUP_ABOVE_SUSPENDED (f)
	{ w32_set_z_group (f, Qabove (), Qabove_suspended ()); }
  });
}

/* C 8269/8271's two filter strings.  C's `static const wchar_t filter_w[]` is a **wide** literal
   with embedded NULs, which Rust cannot spell: the port carries the same UTF-16 code units in the
   array below (36 of them, the last `\0` inclusive) and `filter_a` the same bytes in a `[c_char]`
   - the `DARK_MODE_APP_NAME` / `CURRENT_DIRECTORY_L` device above.  */
#[cfg (feature = "have_ntgui")]
pub const FILTER_W: [c_ushort; 36] =
  [0x0041, 0x006C, 0x006C, 0x0020, 0x0046, 0x0069, 0x006C, 0x0065, 0x0073, 0x0020, 0x0028,
   0x002A, 0x002E, 0x002A, 0x0029, 0x0000, 0x002A, 0x002E, 0x002A, 0x0000, 0x0044, 0x0069,
   0x0072, 0x0065, 0x0063, 0x0074, 0x006F, 0x0072, 0x0069, 0x0065, 0x0073, 0x0000, 0x002A,
   0x007C, 0x002A, 0x0000];
#[cfg (feature = "have_ntgui")]
pub const FILTER_A: [c_char; 36] =
  [0x41, 0x6C, 0x6C, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x73, 0x20, 0x28, 0x2A, 0x2E, 0x2A, 0x29,
   0x00, 0x2A, 0x2E, 0x2A, 0x00, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x69, 0x65,
   0x73, 0x00, 0x2A, 0x7C, 0x2A, 0x00];

/// C: #define CP_ACP 0 / #define CP_UTF8 65001 - the two `<winnls.h>` code-page ids C 8380-8409
/// passes to the lazily bound `pMultiByteToWideChar`/`pWideCharToMultiByte` (`CP_UTF8` is
/// already declared above; `CP_ACP` joins it).
#[cfg (feature = "have_ntgui")]
pub const CP_ACP: UINT = 0;

/// C: #define OFN_HIDEREADONLY 0x00000004 / #define OFN_NOCHANGEDIR 0x00000008 / #define
/// C: OFN_ENABLEHOOK 0x00000020 / #define OFN_PATHMUSTEXIST 0x00000800 / #define
/// C: OFN_FILEMUSTEXIST 0x00001000 / #define OFN_EXPLORER 0x00080000 - the six `<commdlg.h>`
/// flags C 8441-8475 writes into the two `OPENFILENAME` records.
#[cfg (feature = "have_ntgui")]
pub const OFN_HIDEREADONLY: DWORD = 0x00000004;
#[cfg (feature = "have_ntgui")]
pub const OFN_NOCHANGEDIR: DWORD = 0x00000008;
#[cfg (feature = "have_ntgui")]
pub const OFN_ENABLEHOOK: DWORD = 0x00000020;
#[cfg (feature = "have_ntgui")]
pub const OFN_PATHMUSTEXIST: DWORD = 0x00000800;
#[cfg (feature = "have_ntgui")]
pub const OFN_FILEMUSTEXIST: DWORD = 0x00001000;
#[cfg (feature = "have_ntgui")]
pub const OFN_EXPLORER: DWORD = 0x00080000;

/// C 8493/8500's two readers (`<commdlg.h>`) and C 8535's `CommDlgExtendedError`.
/// C: BOOL WINAPI GetOpenFileNameW (LPOPENFILENAMEW); BOOL WINAPI GetOpenFileNameA
/// C: (LPOPENFILENAMEA); DWORD WINAPI CommDlgExtendedError (void);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetOpenFileNameW (lpofn: *mut OPENFILENAMEW) -> BOOL;
  pub fn GetOpenFileNameA (lpofn: *mut OPENFILENAMEA) -> BOOL;
  pub fn CommDlgExtendedError () -> DWORD;
}

/// C 8351-8353 and C 8511, the two helpers of C's `NTGUI_UNICODE` arm: `coding.h` 728's
/// `to_unicode` and 737's `from_unicode_buffer` (defined in `coding.c` 8573 and `w32.c`), which
/// the `include/coding.rs` mirror does not carry yet.  Both stay text here - the arm is dead
/// unless the `cygwin` feature is on - and `guardian-file-name`'s twin uses land with
/// `syms_of_w32fns` (C 12484).
/// C: extern wchar_t *to_unicode (Lisp_Object str, Lisp_Object *buf);
/// C: extern Lisp_Object from_unicode_buffer (const wchar_t *wstr);
#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
unsafe extern "C" {
  pub fn to_unicode (str: Lisp_Object, buf: *mut Lisp_Object) -> *mut c_ushort;
  pub fn from_unicode_buffer (wstr: *const c_ushort) -> Lisp_Object;
  /* C 8334/8336 and C 8522's two readers (`cygw32.h`) are `include/globals.rs`'s
     (`Fcygwin_convert_file_name_to_windows`/`_from_windows`), so this block declares only the two
     coding.h helpers above (`check-mirror-dupes.py` is the reading that says so).  */
}

/// C 8284-8291 and C 8298-8305: the two **anonymous** structs C wraps its `OPENFILENAME` records
/// in ("define a big enough struct for the new dialog to trick GetOpenFileName into giving us the
/// new dialogs").  The port names each after the variable it initialises (the file's `…_s` device,
/// `kbdhook_s`'s reading) and carries C 8286/8300's `#if _WIN32_WINNT < 0x500` arm under the
/// feature that names C's own condition - the file head's `win32_windows_ge_0410` convention, so
/// `win32_nt_lt_500`; `tools/check-cond-eval.py` cannot decide the value, which is why both arms
/// stay text.
/// C: struct { OPENFILENAMEW details;
/// C: #if _WIN32_WINNT < 0x500   PVOID pvReserved; DWORD dwReserved; DWORD FlagsEx;  #endif
/// C: } new_file_details_w;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct new_file_details_w_s
{
  pub details: OPENFILENAMEW,
  #[cfg (feature = "win32_nt_lt_500")]
  pub pvReserved: PVOID,
  #[cfg (feature = "win32_nt_lt_500")]
  pub dwReserved: DWORD,
  #[cfg (feature = "win32_nt_lt_500")]
  pub FlagsEx: DWORD,
}
/// C 8298-8305: the `A` twin of the record above.
#[cfg (feature = "have_ntgui")]
#[repr(C)]
pub struct new_file_details_a_s
{
  pub details: OPENFILENAMEA,
  #[cfg (feature = "win32_nt_lt_500")]
  pub pvReserved: PVOID,
  #[cfg (feature = "win32_nt_lt_500")]
  pub dwReserved: DWORD,
  #[cfg (feature = "win32_nt_lt_500")]
  pub FlagsEx: DWORD,
}

/// C 8264-8554: `x-file-dialog` - the file's third-largest `DEFUN`, and the one that puts
/// C 8284-8305's two wrapper records and C 8269-8271's filter strings to work.  C's order is kept
/// item for item: C 8274-8278's five locals, C 8294-8315's `NTGUI_UNICODE`/else declaration pairs
/// (both arms carried; the else arm is the live one, so its four path buffers and the two
/// `file_details` pointers are the ones this configuration compiles - C's `#ifdef`/`#else` split
/// is preserved as the two arms of a `#[cfg]` group), C 8317-8411's conversion block
/// (`CHECK_STRING`s, `Fexpand_file_name`, `Ffile_name_nondirectory`, `empty_unibyte_string`,
/// C 8347's literal, `ENCODE_FILE` + `Fcopy_sequence` + `unixtodos_filename`,
/// `filename_to_utf16`/`filename_to_ansi`, and C 8384/8402/8408's three **bare `alloca`s** - the
/// port's `sa_stack_alloc` (PORTING.md P-05, the `src/w32menu.rs` 1005 reading) - feeding
/// `pMultiByteToWideChar`/`pWideCharToMultiByte`), C 8413-8478's two record-filling arms
/// (`memset`, C 8422/8456's `w32_major_version` size choice, `countof`, the six `OFN_*` flags),
/// C 8480-8505's dialog call (`w32_dialog_in_progress`, the two `specbind`s,
/// `record_unwind_protect (w32_dialog_in_progress, Qnil)`, `GetOpenFileNameW`/`A`) and
/// C 8507-8553's result handling (`filename_from_utf16`/`filename_from_ansi`,
/// `dostounix_filename`, `DECODE_FILE`, the C 8527-8532 directory strip, C 8535's
/// `CommDlgExtendedError` and C 8539-8546's `Fcompleting_read` fallback with its eight
/// arguments).
/// C: DEFUN ("x-file-dialog", Fx_file_dialog, Sx_file_dialog, 2, 5, 0,
/// C:        doc: /* SKIP: real doc in xfns.c.  */)
/// C:   (Lisp_Object prompt, Lisp_Object dir, Lisp_Object default_filename,
/// C:    Lisp_Object mustmatch, Lisp_Object only_dir_p)
#[cfg (feature = "have_ntgui")]
DEFUN! ("x-file-dialog", Fx_file_dialog, Sx_file_dialog, 2, 5, 0,
	doc: "SKIP: real doc in xfns.c.  ",
	(prompt: Lisp_Object, dir: Lisp_Object, default_filename: Lisp_Object,
	 mustmatch: Lisp_Object, only_dir_p: Lisp_Object),
	{
  let mut filename: Lisp_Object = default_filename;
  let f: *mut frame = SELECTED_FRAME ();
  let mut file_opened: BOOL = FALSE;
  let orig_dir: Lisp_Object = dir;
  let orig_prompt: Lisp_Object = prompt;

  /* C 8284-8305's two wrapper records, then C 8293-8296's `#ifdef NTGUI_UNICODE` arm and
     C 8297-8315's `#else` arm - both carried, the else one live (PORTING.md P-51).  C's
     `wchar_t` is the mirror's `c_ushort` (the token family `tools/extract-abi.py` records).  */
  let mut new_file_details_w: new_file_details_w_s = core::mem::zeroed ();
  /* C 8294-8296: `#ifdef NTGUI_UNICODE`.  */
  #[cfg (feature = "cygwin")]
  let mut filename_buf_w: [c_ushort; 32 * 1024 + 1] = [0; 32 * 1024 + 1];   /* NT kernel maximum */
  #[cfg (feature = "cygwin")]
  let file_details_w: *mut OPENFILENAMEW = &raw mut new_file_details_w.details;
  #[cfg (feature = "cygwin")]
  const use_unicode: c_int = 1;
  /* C 8297-8315: `#else`.  */
  let mut new_file_details_a: new_file_details_a_s = core::mem::zeroed ();
  #[cfg (not (feature = "cygwin"))]
  let mut filename_buf_w: [c_ushort; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];
  #[cfg (not (feature = "cygwin"))]
  let mut dir_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];
  #[cfg (not (feature = "cygwin"))]
  let mut filename_buf_a: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];
  #[cfg (not (feature = "cygwin"))]
  let mut dir_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  #[cfg (not (feature = "cygwin"))]
  let file_details_w: *mut OPENFILENAMEW = &raw mut new_file_details_w.details;
  #[cfg (not (feature = "cygwin"))]
  let file_details_a: *mut OPENFILENAMEA = &raw mut new_file_details_a.details;
  #[cfg (not (feature = "cygwin"))]
  let use_unicode: c_int = w32_unicode_filenames as c_int;
  #[cfg (not (feature = "cygwin"))]
  let mut prompt_w: *mut c_ushort;
  #[cfg (not (feature = "cygwin"))]
  let mut prompt_a: *mut c_char;		/* C's `UNINIT`.  */
  #[cfg (not (feature = "cygwin"))]
  let mut len: c_int;
  #[cfg (not (feature = "cygwin"))]
  let mut fname_ret: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];

  /* C 8317-8318's block comment.  */
  {
    /* Note: under NTGUI_UNICODE, we do _NOT_ use ENCODE_FILE: the
       system file encoding expected by the platform APIs (e.g. Cygwin's
       POSIX implementation) may not be the same as the encoding expected
       by the Windows "ANSI" APIs!  */

    CHECK_STRING (prompt);
    CHECK_STRING (dir);

    dir = Fexpand_file_name (dir, Qnil ());

    if STRINGP (filename)
      { filename = Ffile_name_nondirectory (filename); }
    else
      { filename = empty_unibyte_string; }

    /* C 8333-8337: `#ifdef CYGWIN`.  */
    #[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
    {
      dir = Fcygwin_convert_file_name_to_windows (dir, Qt ());
      if SCHARS (filename) > 0
	{ filename = Fcygwin_convert_file_name_to_windows (filename, Qnil ()); }
    }

    CHECK_STRING (dir);
    CHECK_STRING (filename);

    /* The code in file_dialog_callback that attempts to set the text
       of the file name edit window when handling the CDN_INITDONE
       WM_NOTIFY message does not work.  Setting filename to "Current
       Directory" in the only_dir_p case here does work however.  */
    if SCHARS (filename) == 0 && ! NILP (only_dir_p)
      { filename = build_string (c"Current Directory".as_ptr ()); }

    /* Convert the values we've computed so far to system form.  */
    /* C 8350-8357: `#ifdef NTGUI_UNICODE`.  */
    #[cfg (feature = "cygwin")]
    {
      to_unicode (prompt, &raw mut prompt);
      to_unicode (dir, &raw mut dir);
      to_unicode (filename, &raw mut filename);
      if SBYTES (filename) + 1 > core::mem::size_of_val (&filename_buf_w) as isize
        { report_file_error (c"filename too long".as_ptr (), default_filename); }

      memcpy (filename_buf_w.as_mut_ptr () as *mut c_void, SDATA (filename) as *const c_void,
              SBYTES (filename) as usize + 1);
    }
    /* C 8358-8410: `#else` - the live arm.  */
    #[cfg (not (feature = "cygwin"))]
    {
    /* Convert the values we've computed so far to system form.  */
    prompt = ENCODE_FILE (prompt);
    dir = ENCODE_FILE (dir);
    filename = ENCODE_FILE (filename);

    /* We modify these in-place, so make copies for safety.  */
    dir = Fcopy_sequence (dir);
    unixtodos_filename (SSDATA (dir));
    filename = Fcopy_sequence (filename);
    unixtodos_filename (SSDATA (filename));
    if SBYTES (filename) >= MAX_UTF8_PATH as isize
      { report_file_error (c"filename too long".as_ptr (), default_filename); }
    if w32_unicode_filenames
      {
	filename_to_utf16 (SSDATA (dir), dir_w.as_mut_ptr ());
	if filename_to_utf16 (SSDATA (filename), filename_buf_w.as_mut_ptr ()) != 0
	  {
	    /* filename_to_utf16 sets errno to ENOENT when the file
	       name is too long or cannot be converted to UTF-16.  */
	    if *errno () == ENOENT && filename_buf_w[MAX_PATH as usize - 1] != 0
	      { report_file_error (c"filename too long".as_ptr (), default_filename); }
	  }
	len = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
				    SSDATA (prompt) as *const c_char, -1,
				    core::ptr::null_mut (), 0);
	if len > 32768
	  { len = 32768; }
	prompt_w = sa_stack_alloc (len as usize * core::mem::size_of::<c_ushort> ()) as *mut u16;
	(pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
				      SSDATA (prompt) as *const c_char, -1, prompt_w, len);
      }
    else
      {
	filename_to_ansi (SSDATA (dir), dir_a.as_mut_ptr ()) as c_int;
	if (filename_to_ansi (SSDATA (filename), filename_buf_a.as_mut_ptr ()) as c_int) != 0
	  {
	    /* filename_to_ansi sets errno to ENOENT when the file
	       name is too long or cannot be converted to UTF-16.  */
	    if *errno () == ENOENT && filename_buf_a[MAX_PATH as usize - 1] != 0
	      { report_file_error (c"filename too long".as_ptr (), default_filename); }
	  }
	len = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
				    SSDATA (prompt) as *const c_char, -1,
				    core::ptr::null_mut (), 0);
	if len > 32768
	  { len = 32768; }
	prompt_w = sa_stack_alloc (len as usize * core::mem::size_of::<c_ushort> ()) as *mut u16;
	(pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
				      SSDATA (prompt) as *const c_char, -1, prompt_w, len);
	len = (pWideCharToMultiByte.unwrap ()) (CP_ACP, 0, prompt_w, -1, core::ptr::null_mut (), 0,
				    core::ptr::null_mut (), core::ptr::null_mut ());
	if len > 32768
	  { len = 32768; }
	prompt_a = sa_stack_alloc (len as usize) as *mut c_char;
	(pWideCharToMultiByte.unwrap ()) (CP_ACP, 0, prompt_w, -1, prompt_a, len,
			      core::ptr::null_mut (), core::ptr::null_mut ());
      }

    }
    /* Fill in the structure for the call to GetOpenFileName below.
       For NTGUI_UNICODE builds (which run only on NT), we just use
       the actual size of the structure.  For non-NTGUI_UNICODE
       builds, we tell the OS we're using an old version of the
       structure if the OS isn't new enough to support the newer
       version.  */
    if use_unicode != 0
      {
	memset ((&raw mut new_file_details_w) as *mut c_void, 0,
		core::mem::size_of::<new_file_details_w_s> ());
	if w32_major_version > 4 && w32_major_version < 95
	  {
	    (*file_details_w).lStructSize =
	      core::mem::size_of::<new_file_details_w_s> () as DWORD;
	  }
	else
	  { (*file_details_w).lStructSize = core::mem::size_of::<OPENFILENAMEW> () as DWORD; }
	/* Set up the inout parameter for the selected file name.  */
	(*file_details_w).lpstrFile = filename_buf_w.as_mut_ptr ();
	(*file_details_w).nMaxFile = countof! (filename_buf_w) as DWORD;
	(*file_details_w).hwndOwner = FRAME_W32_WINDOW! (f);
	/* Undocumented Bug in Common File Dialog:
	   If a filter is not specified, shell links are not resolved.  */
	(*file_details_w).lpstrFilter = FILTER_W.as_ptr ();
	/* C 8433-8439: `#ifdef NTGUI_UNICODE`.  */
	#[cfg (feature = "cygwin")]
	{
	  (*file_details_w).lpstrInitialDir = SDATA (dir) as *mut c_ushort;
	  (*file_details_w).lpstrTitle = SDATA (prompt) as *mut guichar_t;
	}
	#[cfg (not (feature = "cygwin"))]
	{
	  (*file_details_w).lpstrInitialDir = dir_w.as_mut_ptr ();
	  (*file_details_w).lpstrTitle = prompt_w;
	}
	(*file_details_w).nFilterIndex = if NILP (only_dir_p) { 1 } else { 2 };
	(*file_details_w).Flags = OFN_HIDEREADONLY | OFN_NOCHANGEDIR
	  | OFN_EXPLORER | OFN_ENABLEHOOK;
	if ! NILP (mustmatch)
	  {
	    /* Require that the path to the parent directory exists.  */
	    (*file_details_w).Flags |= OFN_PATHMUSTEXIST;
	    /* If we are looking for a file, require that it exists.  */
	    if NILP (only_dir_p)
	      { (*file_details_w).Flags |= OFN_FILEMUSTEXIST; }
	  }
      }
    else
      {
	memset ((&raw mut new_file_details_a) as *mut c_void, 0,
		core::mem::size_of::<new_file_details_a_s> ());
	if w32_major_version > 4 && w32_major_version < 95
	  {
	    (*file_details_a).lStructSize =
	      core::mem::size_of::<new_file_details_a_s> () as DWORD;
	  }
	else
	  { (*file_details_a).lStructSize = core::mem::size_of::<OPENFILENAMEA> () as DWORD; }
	(*file_details_a).lpstrFile = filename_buf_a.as_mut_ptr ();
	(*file_details_a).nMaxFile = countof! (filename_buf_a) as DWORD;
	(*file_details_a).hwndOwner = FRAME_W32_WINDOW! (f);
	(*file_details_a).lpstrFilter = FILTER_A.as_ptr ();
	(*file_details_a).lpstrInitialDir = dir_a.as_mut_ptr ();
	(*file_details_a).lpstrTitle = prompt_a;
	(*file_details_a).nFilterIndex = if NILP (only_dir_p) { 1 } else { 2 };
	(*file_details_a).Flags = OFN_HIDEREADONLY | OFN_NOCHANGEDIR
	  | OFN_EXPLORER | OFN_ENABLEHOOK;
	if ! NILP (mustmatch)
	  {
	    /* Require that the path to the parent directory exists.  */
	    (*file_details_a).Flags |= OFN_PATHMUSTEXIST;
	    /* If we are looking for a file, require that it exists.  */
	    if NILP (only_dir_p)
	      { (*file_details_a).Flags |= OFN_FILEMUSTEXIST; }
	  }
      }

    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      w32_dialog_in_progress (Qt ());

      /* Prevent redisplay.  */
      specbind (Qinhibit_redisplay (), Qt ());
      record_unwind_protect (Some (w32_dialog_in_progress), Qnil ());
      block_input ();
      if use_unicode != 0
	{
	  (*file_details_w).lpfnHook = Some (file_dialog_callback);

	  file_opened = GetOpenFileNameW (file_details_w);
	}
      else
	{
	  (*file_details_a).lpfnHook = Some (file_dialog_callback);

	  file_opened = GetOpenFileNameA (file_details_a);
	}
      unblock_input ();
      unbind_to (count, Qnil ());
    }

    if file_opened != 0
      {
	/* Get an Emacs string from the value Windows gave us.  */
	/* C 8510-8511: `#ifdef NTGUI_UNICODE`.  */
	#[cfg (feature = "cygwin")]
	{ filename = from_unicode_buffer (filename_buf_w.as_mut_ptr ()); }
	/* C 8512-8519: `#else`.  */
	#[cfg (not (feature = "cygwin"))]
	{
	if use_unicode != 0
	  { filename_from_utf16 (filename_buf_w.as_mut_ptr (), fname_ret.as_mut_ptr ()); }
	else
	  { filename_from_ansi (filename_buf_a.as_mut_ptr (), fname_ret.as_mut_ptr ()); }
	dostounix_filename (fname_ret.as_mut_ptr ());
	filename = DECODE_FILE (build_unibyte_string (fname_ret.as_mut_ptr ()));
	}

	/* C 8521-8523: `#ifdef CYGWIN`.  */
	#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
	{ filename = Fcygwin_convert_file_name_from_windows (filename, Qt ()); }

	/* Strip the dummy filename off the end of the string if we
	   added it to select a directory.  */
	if (use_unicode != 0 && (*file_details_w).nFilterIndex == 2)
	    || (use_unicode == 0 && (*file_details_a).nFilterIndex == 2)
	  { filename = Ffile_name_directory (filename); }
      }
    /* User canceled the dialog without making a selection.  */
    else if CommDlgExtendedError () == 0
      { filename = Qnil (); }
    /* An error occurred, fallback on reading from the mini-buffer.  */
    else
      {
	filename = Fcompleting_read (orig_prompt,
				     Qread_file_name_internal (),
				     orig_dir,
				     mustmatch,
				     orig_dir,
				     Qfile_name_history (),
				     default_filename,
				     Qnil ());
      }
  }

  /* Make "Cancel" equivalent to C-g.  */
  if NILP (filename)
    { quit (); }

  return filename;
});

/// C 8557-8558: C's `#ifdef WINDOWSNT` and the lazily bound `SHFileOperationW` pointer - C's
/// `static int (WINAPI *pfnSHFileOperationW) (LPSHFILEOPSTRUCTW);`.  `globals_of_w32fns`
/// (C 12466) is what assigns it, and the storage is `Option<…>` because a C function pointer may
/// be NULL (the `include/w32term.rs` 1237-1245 reading).  The `FILEOP_FLAGS`, `SHFILEOPSTRUCTA/W`
/// and `LPSHFILEOPSTRUCTA/W` records below are `<shellapi.h>`'s, spelled in that header's order,
/// and the five `FOF_*` flags are what C 8618/8649 ORs together.
/// C: static int (WINAPI *pfnSHFileOperationW) (LPSHFILEOPSTRUCTW);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type PFNSHFILEOPERATIONW
  = Option<unsafe extern "system" fn (lpFileOp: *mut SHFILEOPSTRUCTW) -> c_int>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut pfnSHFileOperationW: PFNSHFILEOPERATIONW = None;

/// C: typedef WORD FILEOP_FLAGS; - `<shellapi.h>`'s flag type (the record member is a `WORD`).
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type FILEOP_FLAGS = WORD;
/// C: typedef struct _SHFILEOPSTRUCTW { HWND hwnd; UINT wFunc; PCWSTR pFrom; PCWSTR pTo;
/// C: FILEOP_FLAGS fFlags; BOOL fAnyOperationsAborted; LPVOID hNameMappings; PCWSTR
/// C: lpszProgressTitle; } SHFILEOPSTRUCTW, *LPSHFILEOPSTRUCTW;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct SHFILEOPSTRUCTW
{
  pub hwnd: HWND,
  pub wFunc: UINT,
  pub pFrom: *const c_ushort,
  pub pTo: *const c_ushort,
  pub fFlags: FILEOP_FLAGS,
  pub fAnyOperationsAborted: BOOL,
  pub hNameMappings: LPVOID,
  pub lpszProgressTitle: *const c_ushort,
}
/// C: typedef struct _SHFILEOPSTRUCTA { … PCSTR pFrom; PCSTR pTo; … } SHFILEOPSTRUCTA,
/// C: *LPSHFILEOPSTRUCTA; - the `A` twin (C 8629).
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct SHFILEOPSTRUCTA
{
  pub hwnd: HWND,
  pub wFunc: UINT,
  pub pFrom: *const c_char,
  pub pTo: *const c_char,
  pub fFlags: FILEOP_FLAGS,
  pub fAnyOperationsAborted: BOOL,
  pub hNameMappings: LPVOID,
  pub lpszProgressTitle: *const c_char,
}

/// C: #define FO_DELETE 3 / #define FOF_SILENT 0x0004 / #define FOF_NOCONFIRMATION 0x0010 /
/// C: #define FOF_ALLOWUNDO 0x0040 / #define FOF_NOERRORUI 0x0400 / #define
/// C: FOF_NO_CONNECTED_ELEMENTS 0x2000 - C 8616/8647's function code and four of the five flags
/// C 8618/8649 OR together, from `<shellapi.h>`; the fifth, `FOF_NO_CONNECTED_ELEMENTS`, is C 154's
/// `#ifndef` guard block and already stands at the top of this file as a `DWORD`, so the two OR
/// sites cast it down to `FILEOP_FLAGS` (`check-mirror-dupes` found the duplicate).
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FO_DELETE: UINT = 3;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FOF_SILENT: FILEOP_FLAGS = 0x0004;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FOF_NOCONFIRMATION: FILEOP_FLAGS = 0x0010;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FOF_ALLOWUNDO: FILEOP_FLAGS = 0x0040;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FOF_NOERRORUI: FILEOP_FLAGS = 0x0400;
/// C: #define ERROR_FILE_NOT_FOUND 2 - `<winerror.h>`, C 8640's result for the wildcard case.
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const ERROR_FILE_NOT_FOUND: c_int = 2;

/// C 8653's `SHFileOperationA` (`<shellapi.h>`) and the three CRT entries C 8612/8638/8643 uses:
/// `_wchmod`/`_chmod` (`<io.h>`) and `_mbspbrk` (`<mbstring.h>`).
/// C: int WINAPI SHFileOperationA (LPSHFILEOPSTRUCTA); int _wchmod (const wchar_t *, int);
/// C: int _chmod (const char *, int); unsigned char *_mbspbrk (const unsigned char *,
/// C: const unsigned char *);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn SHFileOperationA (lpFileOp: *mut SHFILEOPSTRUCTA) -> c_int;
  pub fn _wchmod (path: *const c_ushort, pmode: c_int) -> c_int;
  pub fn _chmod (path: *const c_char, pmode: c_int) -> c_int;
  pub fn _mbspbrk (str: *const c_uchar, charset: *const c_uchar) -> *mut c_uchar;
}

/* C 8560-8561's comment.  */
/// C 8562-8660: `system-move-file-to-trash` - C 8572/8596/8600's three readers
/// (`maybe_load_unicows_dll` is `w32.h` 176's, declared by `include/w32.rs`; `map_w32_filename`
/// is that mirror's; `os_subtype`/`OS_SUBTYPE_9X` are `include/w32common.rs`'s), C 8574-8579's
/// directory test, C 8584-8588's handler call (C's `calln` is the port's `calln!`), and
/// C 8591-8658's two arms - the `W` one over C 8602-8625's record and `_wchmod`, the `A` one over
/// C 8629-8653's with C 8638's `_mbspbrk` wildcard guard.  C 8624's `eassert` is the port's
/// `is_some ()` because the pointer is an `Option`.
/// C: DEFUN ("system-move-file-to-trash", Fsystem_move_file_to_trash,
/// C:        Ssystem_move_file_to_trash, 1, 1, 0,
/// C:        doc: /* Move file or directory named FILENAME to the recycle bin.  */)
/// C:   (Lisp_Object filename)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("system-move-file-to-trash", Fsystem_move_file_to_trash,
	Ssystem_move_file_to_trash, 1, 1, 0,
	doc: "Move file or directory named FILENAME to the recycle bin.  ",
	(filename: Lisp_Object),
	{
  let handler: Lisp_Object;
  let encoded_file: Lisp_Object;
  let mut operation: Lisp_Object;
  let mut filename = filename;                 /* C 8579 assigns the parameter.  */

  /* Required on Windows 9X.  */
  maybe_load_unicows_dll ();

  operation = Qdelete_file ();
  if ! NILP (Ffile_directory_p (filename))
      && NILP (Ffile_symlink_p (filename))
    {
      operation = Qdelete_directory ();
      filename = Fdirectory_file_name (filename);
    }

  /* Must have fully qualified file names for moving files to Recycle
     Bin. */
  filename = Fexpand_file_name (filename, Qnil ());

  handler = Ffind_file_name_handler (filename, operation);
  if ! NILP (handler)
    { return calln! (handler, operation, filename); }
  else
    {
      let path: *const c_char;
      let mut result: c_int;

      encoded_file = ENCODE_FILE (filename);

      path = map_w32_filename (SSDATA (encoded_file), core::ptr::null_mut ());

      /* The Unicode version of SHFileOperation is not supported on
	 Windows 9X. */
      if w32_unicode_filenames && os_subtype != OS_SUBTYPE_9X
	{
	  let mut file_op_w: SHFILEOPSTRUCTW = core::mem::zeroed ();
	  /* We need one more element beyond MAX_PATH because this is
	     a list of file names, with the last element double-null
	     terminated. */
	  let mut tmp_path_w: [c_ushort; MAX_PATH as usize + 1] =
	    [0; MAX_PATH as usize + 1];

	  memset ((&raw mut tmp_path_w) as *mut c_void, 0, core::mem::size_of_val (&tmp_path_w));
	  filename_to_utf16 (path, tmp_path_w.as_mut_ptr ());

	  /* On Windows, write permission is required to delete/move files.  */
	  _wchmod (tmp_path_w.as_mut_ptr (), 0o666);

	  memset ((&raw mut file_op_w) as *mut c_void, 0, core::mem::size_of_val (&file_op_w));
	  file_op_w.hwnd = HWND_DESKTOP;
	  file_op_w.wFunc = FO_DELETE;
	  file_op_w.pFrom = tmp_path_w.as_mut_ptr ();
	  file_op_w.fFlags = FOF_SILENT | FOF_NOCONFIRMATION | FOF_ALLOWUNDO
	    | FOF_NOERRORUI | FOF_NO_CONNECTED_ELEMENTS as FILEOP_FLAGS;
	  file_op_w.fAnyOperationsAborted = FALSE;

	  /* This is stated to exist on all versions of Windows NT Emacs
	     supports.  */
	  eassert! (pfnSHFileOperationW.is_some ());
	  result = (pfnSHFileOperationW.unwrap ()) (&raw mut file_op_w);
	}
      else
	{
	  let mut file_op_a: SHFILEOPSTRUCTA = core::mem::zeroed ();
	  let mut tmp_path_a: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];

	  memset ((&raw mut tmp_path_a) as *mut c_void, 0, core::mem::size_of_val (&tmp_path_a));
	  filename_to_ansi (path, tmp_path_a.as_mut_ptr ());

	  /* If a file cannot be represented in ANSI codepage, don't
	     let them inadvertently delete other files because some
	     characters are interpreted as a wildcards.  */
	  if ! (_mbspbrk (tmp_path_a.as_mut_ptr () as *const c_uchar,
			  c"?*".as_ptr () as *const c_uchar)).is_null ()
	    { result = ERROR_FILE_NOT_FOUND; }
	  else
	    {
	      _chmod (tmp_path_a.as_mut_ptr (), 0o666);

	      memset ((&raw mut file_op_a) as *mut c_void, 0,
		      core::mem::size_of_val (&file_op_a));
	      file_op_a.hwnd = HWND_DESKTOP;
	      file_op_a.wFunc = FO_DELETE;
	      file_op_a.pFrom = tmp_path_a.as_mut_ptr ();
	      file_op_a.fFlags = FOF_SILENT | FOF_NOCONFIRMATION | FOF_ALLOWUNDO
		| FOF_NOERRORUI | FOF_NO_CONNECTED_ELEMENTS as FILEOP_FLAGS;
	      file_op_a.fAnyOperationsAborted = FALSE;

	      result = SHFileOperationA (&raw mut file_op_a);
	    }
	}
      if result != 0
	{ report_file_error (c"Removing old name".as_ptr (), list1 (filename)); }
    }
  return Qnil ();
});

/* C 8662's `#endif /* WINDOWSNT */` has no Rust counterpart (the items above carry the
   `windowsnt` feature themselves), and C 8664-8667's banner follows it in C's order: the port
   keeps C's text inside one wrapper comment (the C 8116-8118 device).

/***********************************************************************
			 w32 specialized functions
 ***********************************************************************/
 */


/// C: #define WM_SYSCOMMAND 0x0112 - the `<winuser.h>` message C 8685 posts.
#[cfg (feature = "have_ntgui")]
pub const WM_SYSCOMMAND: UINT = 0x0112;

/// C 8691's `static BOOL (WINAPI *pfnShellExecuteExW) (LPSHELLEXECUTEINFOW);` and the
/// `<shellapi.h>` record it names - C 8765+'s `w32-shell-execute` fills every member, so the
/// record is declared in full now (`nShow` is C's `int`, `dwHotKey` a `DWORD`, and C's
/// `DUMMYUNIONNAME` is the anonymous union of `hIcon`/`hMonitor`, carried as a Rust `union` with
/// the same two names).  The pointer's storage is `Option<…>` (a C function pointer may be NULL);
/// `globals_of_w32fns` is what assigns it.
/// C: typedef struct _SHELLEXECUTEINFOW { DWORD cbSize; ULONG fMask; HWND hwnd; LPCWSTR lpVerb;
/// C: LPCWSTR lpFile; LPCWSTR lpParameters; LPCWSTR lpDirectory; int nShow; HINSTANCE hInstApp;
/// C: LPVOID lpIDList; LPCWSTR lpClass; HKEY hkeyClass; DWORD dwHotKey;
/// C: union { HANDLE hIcon; HANDLE hMonitor; } DUMMYUNIONNAME; HANDLE hProcess; }
/// C: SHELLEXECUTEINFOW, *LPSHELLEXECUTEINFOW;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOW_union
{
  pub hIcon: HANDLE,
  pub hMonitor: HANDLE,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOW
{
  pub cbSize: DWORD,
  pub fMask: c_ulong,
  pub hwnd: HWND,
  pub lpVerb: *const c_ushort,
  pub lpFile: *const c_ushort,
  pub lpParameters: *const c_ushort,
  pub lpDirectory: *const c_ushort,
  pub nShow: c_int,
  pub hInstApp: HINSTANCE,
  pub lpIDList: LPVOID,
  pub lpClass: *const c_ushort,
  pub hkeyClass: HKEY,
  pub dwHotKey: DWORD,
  pub r#DUMMYUNIONNAME: SHELLEXECUTEINFOW_union,
  pub hProcess: HANDLE,
}
#[cfg (feature = "have_ntgui")]
pub type LPSHELLEXECUTEINFOW = *mut SHELLEXECUTEINFOW;
#[cfg (all (feature = "have_ntgui", not (feature = "cygwin")))]
pub type PFNSHELLEXECUTEEXW
  = Option<unsafe extern "system" fn (lpExecInfo: *mut SHELLEXECUTEINFOW) -> BOOL>;
#[cfg (all (feature = "have_ntgui", not (feature = "cygwin")))]
static mut pfnShellExecuteExW: PFNSHELLEXECUTEEXW = None;

/// C 8669-8688: `w32-send-sys-command` - C 8680's `decode_window_system_frame`, C 8682's
/// `CHECK_FIXNUM`, C 8684-8685's `FRAME_W32_P` test and its `PostMessage` (declared above as
/// `<winuser.h>`'s) with C's `XFIXNUM (command)` narrowed into the `WPARAM` parameter.
/// C: DEFUN ("w32-send-sys-command", Fw32_send_sys_command,
/// C:        Sw32_send_sys_command, 1, 2, 0,
/// C:        doc: /* Send frame a Windows WM_SYSCOMMAND message of type COMMAND. … */)
/// C:   (Lisp_Object command, Lisp_Object frame)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-send-sys-command", Fw32_send_sys_command,
	Sw32_send_sys_command, 1, 2, 0,
	doc: "Send frame a Windows WM_SYSCOMMAND message of type COMMAND.\nSome useful values for COMMAND are #xf030 to maximize frame (#xf020\nto minimize), #xf120 to restore frame to original size, and #xf100\nto activate the menubar for keyboard access.  #xf140 activates the\nscreen saver if defined.\n\nIf optional parameter FRAME is not specified, use selected frame.  ",
	(command: Lisp_Object, frame: Lisp_Object),
	{
  let f: *mut frame = decode_window_system_frame (frame);

  CHECK_FIXNUM (command);

  if FRAME_W32_P (f)
    { PostMessage (FRAME_W32_WINDOW! (f), WM_SYSCOMMAND, XFIXNUM (command) as WPARAM, 0); }

  return Qnil ();
});

/* C 8690-8692's `#ifndef CYGWIN` / `#endif` are the two `not (feature = "cygwin")` attributes on
   `PFNSHELLEXECUTEEXW` and its storage above; C's `#endif` line itself has no counterpart.  */

/// C 8838's `#else` arm needs `<shellapi.h>`'s `A` twin of slice 18's record (C 8964's
/// `SHELLEXECUTEINFOA shexinfo_a;`), so it is declared here in full with the same anonymous union
/// C's `DUMMYUNIONNAME` names.
/// C: typedef struct _SHELLEXECUTEINFOA { DWORD cbSize; ULONG fMask; HWND hwnd; LPCSTR lpVerb;
/// C: LPCSTR lpFile; LPCSTR lpParameters; LPCSTR lpDirectory; int nShow; HINSTANCE hInstApp;
/// C: LPVOID lpIDList; LPCSTR lpClass; HKEY hkeyClass; DWORD dwHotKey;
/// C: union { HANDLE hIcon; HANDLE hMonitor; } DUMMYUNIONNAME; HANDLE hProcess; }
/// C: SHELLEXECUTEINFOA, *LPSHELLEXECUTEINFOA;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union SHELLEXECUTEINFOA_union
{
  pub hIcon: HANDLE,
  pub hMonitor: HANDLE,
}
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHELLEXECUTEINFOA
{
  pub cbSize: DWORD,
  pub fMask: c_ulong,
  pub hwnd: HWND,
  pub lpVerb: *const c_char,
  pub lpFile: *const c_char,
  pub lpParameters: *const c_char,
  pub lpDirectory: *const c_char,
  pub nShow: c_int,
  pub hInstApp: HINSTANCE,
  pub lpIDList: LPVOID,
  pub lpClass: *const c_char,
  pub hkeyClass: HKEY,
  pub dwHotKey: DWORD,
  pub r#DUMMYUNIONNAME: SHELLEXECUTEINFOA_union,
  pub hProcess: HANDLE,
}

/// C: #define SEE_MASK_INVOKEIDLIST 0x0000000C / #define SEE_MASK_FLAG_DDEWAIT 0x00000100 /
/// C: #define SEE_MASK_FLAG_NO_UI 0x00000400 - the three `<shellapi.h>` mask bits C 8951/8984 ORs.
#[cfg (feature = "have_ntgui")]
pub const SEE_MASK_INVOKEIDLIST: c_ulong = 0x0000000C;
#[cfg (feature = "have_ntgui")]
pub const SEE_MASK_FLAG_DDEWAIT: c_ulong = 0x00000100;
#[cfg (feature = "have_ntgui")]
pub const SEE_MASK_FLAG_NO_UI: c_ulong = 0x00000400;

/// C: #define SW_SHOWDEFAULT 10 - the `<winuser.h>` show flag C 8797/8992 substitutes for a nil
/// `show_flag` (`SW_SHOWNOACTIVATE` already stands above).
#[cfg (feature = "have_ntgui")]
pub const SW_SHOWDEFAULT: c_int = 10;

/// C 8804-8832's ten `<shellapi.h>` `SE_ERR_*` codes - the values `ShellExecute` returns for
/// everything at or below 32.
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_FNF: intptr_t = 2;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_PNF: intptr_t = 3;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_ACCESSDENIED: intptr_t = 5;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_OOM: intptr_t = 8;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_SHARE: intptr_t = 26;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_ASSOCINCOMPLETE: intptr_t = 27;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_DDETIMEOUT: intptr_t = 28;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_DDEFAIL: intptr_t = 29;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_DDEBUSY: intptr_t = 30;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_NOASSOC: intptr_t = 31;
#[cfg (feature = "have_ntgui")]
pub const SE_ERR_DLLNOTFOUND: intptr_t = 32;

/// C 8805-8831's eight `<winerror.h>` codes, the arguments `w32_strerror` is handed
/// (`ERROR_FILE_NOT_FOUND` already stands above as slice 17's `c_int`).
#[cfg (feature = "have_ntgui")]
pub const ERROR_ACCESS_DENIED: c_int = 5;
#[cfg (feature = "have_ntgui")]
pub const ERROR_NO_ASSOCIATION: c_int = 1155;
#[cfg (feature = "have_ntgui")]
pub const ERROR_DDE_FAIL: c_int = 1156;
#[cfg (feature = "have_ntgui")]
pub const ERROR_DLL_NOT_FOUND: c_int = 1157;
#[cfg (feature = "have_ntgui")]
pub const ERROR_TIMEOUT: c_int = 1460;
#[cfg (feature = "have_ntgui")]
pub const ERROR_NOT_ENOUGH_MEMORY: c_int = 8;
#[cfg (feature = "have_ntgui")]
pub const ERROR_PATH_NOT_FOUND: c_int = 3;
#[cfg (feature = "have_ntgui")]
pub const ERROR_SHARING_VIOLATION: c_int = 32;

/// C 8794/8993's two `<shellapi.h>` entry points (`ShellExecuteW` belongs to the `CYGWIN` arm,
/// `ShellExecuteExA` to the live one) and C 8887's `faccessat`, whose declaration `lib/faccessat.rs`
/// carries for the gnulib replacement.
/// C: HINSTANCE WINAPI ShellExecuteW (HWND, LPCWSTR, LPCWSTR, LPCWSTR, LPCWSTR, INT);
/// C: BOOL WINAPI ShellExecuteExA (LPSHELLEXECUTEINFOA);
/// C: int faccessat (int, const char *, int, int);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn ShellExecuteExA (lpExecInfo: *mut SHELLEXECUTEINFOA) -> BOOL;
}
#[cfg (all (feature = "have_ntgui", feature = "cygwin"))]
unsafe extern "system" {
  pub fn ShellExecuteW (hwnd: HWND, lpOperation: *const c_ushort, lpFile: *const c_ushort,
			lpParameters: *const c_ushort, lpDirectory: *const c_ushort,
			nShowCmd: c_int) -> HINSTANCE;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn faccessat (fd: c_int, path: *const c_char, mode: c_int, flag: c_int) -> c_int;
}

/// C 8863's `IS_DIRECTORY_SEP` - `buffer.h`'s macro
/// (`#define IS_DIRECTORY_SEP(c) ((c) == '/' || (c) == '\\')`), which the `include/buffer.rs`
/// mirror does not carry; the port keeps C's name as a `macro_rules!` (the file's device for a C
/// macro that expands inside an expression).
#[cfg (feature = "have_ntgui")]
macro_rules! IS_DIRECTORY_SEP {
  ($c:expr) => { ($c) == b'/' as c_char || ($c) == b'\\' as c_char };
}

/// C 8694-9014: `w32-shell-execute` - C 8695-8759's 65-line doc body is the `doc:` literal below,
/// generated from C's own text.  C 8762-8771's declarations split along C 8765's `#ifdef CYGWIN`
/// (the dead arm's `intptr_t result` versus the live arm's `use_unicode`, the three `*_a` pointers,
/// `absdoc`, `handler` and `success`); C 8763's `BVAR (current_buffer, directory)` is
/// `include/buffer.rs`'s macro; C 8776-8836 is the `CYGWIN` arm (`GUI_ENCODE_FILE`/
/// `GUI_ENCODE_SYSTEM`/`GUI_SDATA` are `include/w32term.rs`'s macros, `ShellExecuteW` and the ten
/// `SE_ERR_*` codes the case list, `w32_strerror` this file's own - C 10400-ish); C 8838-9002 is
/// the live arm (C 8840's `"file:///"` literal and its `sizeof - 1` length, C 8863's
/// `IS_DIRECTORY_SEP` with `c_isalpha`, C 8874-8896's absolute-name probe with `faccessat` and
/// `unixtodos_filename`, C 8899-8905's `ENCODE_FILE`/`ENCODE_SYSTEM` chain, C 8908-8915's
/// `pMultiByteToWideChar` + `xmalloc` pass, C 8916-8960's `SHELLEXECUTEINFOW` fill -
/// `filename_to_utf16`, the `alloca`-fed `params_w`, C 8932-8943's ASCII-only `ops_w` loop,
/// `pfnShellExecuteExW` - and C 8961-8996's `SHELLEXECUTEINFOA` twin with `codepage_for_filenames`,
/// `pWideCharToMultiByte`, `filename_to_ansi` and `ShellExecuteExA`); then C 9005-9013's shared
/// tail (`Vlocale_coding_system`, `code_convert_string_norecord`, `error`).
/// C: DEFUN ("w32-shell-execute", Fw32_shell_execute, Sw32_shell_execute, 2, 4, 0,
/// C:        doc: /* Get Windows to perform OPERATION on DOCUMENT. … */)
/// C:   (Lisp_Object operation, Lisp_Object document, Lisp_Object parameters, Lisp_Object show_flag)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-shell-execute", Fw32_shell_execute, Sw32_shell_execute, 2, 4, 0,
	doc: "Get Windows to perform OPERATION on DOCUMENT.\nThis is a wrapper around the ShellExecute system function, which\ninvokes the application registered to handle OPERATION for DOCUMENT.\n\nOPERATION is either nil or a string that names a supported operation.\nWhat operations can be used depends on the particular DOCUMENT and its\nhandler application, but typically it is one of the following common\noperations:\n\n \"open\"    - open DOCUMENT, which could be a file, a directory, or an\n	       executable program (application).  If it is an application,\n	       that application is launched in the current buffer's default\n	       directory.  Otherwise, the application associated with\n	       DOCUMENT is launched in the buffer's default directory.\n \"opennew\" - like \"open\", but instruct the application to open\n	       DOCUMENT in a new window.\n \"openas\"  - open the \"Open With\" dialog for DOCUMENT.\n \"print\"   - print DOCUMENT, which must be a file.\n \"printto\" - print DOCUMENT, which must be a file, to a specified printer.\n	       The printer should be provided in PARAMETERS, see below.\n \"explore\" - start the Windows Explorer on DOCUMENT.\n \"edit\"    - launch an editor and open DOCUMENT for editing; which\n	       editor is launched depends on the association for the\n	       specified DOCUMENT.\n \"find\"    - initiate search starting from DOCUMENT, which must specify\n	       a directory.\n \"delete\"  - move DOCUMENT, a file or a directory, to Recycle Bin.\n \"copy\"    - copy DOCUMENT, which must be a file or a directory, into\n	       the clipboard.\n \"cut\"     - move DOCUMENT, a file or a directory, into the clipboard.\n \"paste\"   - paste the file whose name is in the clipboard into DOCUMENT,\n	       which must be a directory.\n \"pastelink\"\n	   - create a shortcut in DOCUMENT (which must be a directory)\n	       the file or directory whose name is in the clipboard.\n \"runas\"   - run DOCUMENT, which must be an executable file, with\n	       elevated privileges (a.k.a. \"as Administrator\").\n \"properties\"\n	   - open the property sheet dialog for DOCUMENT.\n nil       - invoke the default OPERATION, or \"open\" if default is\n	       not defined or unavailable.\n\nDOCUMENT is typically the name of a document file or a URL, but can\nalso be an executable program to run, or a directory to open in the\nWindows Explorer.  If it is a file or a directory, it must be a local\none; this function does not support remote file names.\n\nIf DOCUMENT is an executable program, the optional third arg PARAMETERS\ncan be a string containing command line parameters, separated by blanks,\nthat will be passed to the program.  Some values of OPERATION also require\nparameters (e.g., \"printto\" requires the printer address).  Otherwise,\nPARAMETERS should be nil or unspecified.  Note that double quote characters\nin PARAMETERS must each be enclosed in 2 additional quotes, as in \"\"\".\n\nOptional fourth argument SHOW-FLAG can be used to control how the\napplication will be displayed when it is invoked.  If SHOW-FLAG is nil\nor unspecified, the application is displayed as if SHOW-FLAG of 10 was\nspecified, otherwise it is an integer between 0 and 11 representing\na ShowWindow flag:\n\n  0 - start hidden\n  1 - start as normal-size window\n  3 - start in a maximized window\n  6 - start in a minimized window\n 10 - start as the application itself specifies; this is the default.",
	(operation: Lisp_Object, document: Lisp_Object, parameters: Lisp_Object,
	 show_flag: Lisp_Object),
	{
  let mut errstr: *const c_char;
  let mut current_dir: Lisp_Object = BVAR! (current_buffer! (), directory_);
  let mut doc_w: *mut c_ushort = core::ptr::null_mut ();
  let mut params_w: *mut c_ushort = core::ptr::null_mut ();
  let mut ops_w: *mut c_ushort = core::ptr::null_mut ();
  /* C 8785/8790/8864/8874/8974/8980 assign these three parameters.  */
  let mut operation = operation;
  let mut document = document;
  let mut parameters = parameters;

  CHECK_STRING (document);

  /* C 8765-8836: `#ifdef CYGWIN`.  */
  #[cfg (feature = "cygwin")]
  {
    let mut result: intptr_t;

    current_dir = Fcygwin_convert_file_name_to_windows (current_dir, Qt ());
    document = Fcygwin_convert_file_name_to_windows (document, Qt ());

    /* Encode filename, current directory and parameters.  */
    current_dir = GUI_ENCODE_FILE! (current_dir);
    document = GUI_ENCODE_FILE! (document);
    if STRINGP (parameters)
      {
	parameters = GUI_ENCODE_SYSTEM! (parameters);
	params_w = GUI_SDATA! (parameters);
      }
    if STRINGP (operation)
      {
	operation = GUI_ENCODE_SYSTEM! (operation);
	ops_w = GUI_SDATA! (operation);
      }
    doc_w = GUI_SDATA! (document);
    result = (ShellExecuteW (core::ptr::null_mut (), ops_w, doc_w, params_w,
			     GUI_SDATA! (current_dir),
			     if FIXNUMP (show_flag) { XFIXNUM (show_flag) as c_int }
			     else { SW_SHOWDEFAULT })) as intptr_t;

    if result > 32
      { return Qt (); }

    match result
      {
	SE_ERR_ACCESSDENIED =>
	  { errstr = w32_strerror (ERROR_ACCESS_DENIED); }
	SE_ERR_ASSOCINCOMPLETE | SE_ERR_NOASSOC =>
	  { errstr = w32_strerror (ERROR_NO_ASSOCIATION); }
	SE_ERR_DDEBUSY | SE_ERR_DDEFAIL =>
	  { errstr = w32_strerror (ERROR_DDE_FAIL); }
	SE_ERR_DDETIMEOUT =>
	  { errstr = w32_strerror (ERROR_TIMEOUT); }
	SE_ERR_DLLNOTFOUND =>
	  { errstr = w32_strerror (ERROR_DLL_NOT_FOUND); }
	SE_ERR_FNF =>
	  { errstr = w32_strerror (ERROR_FILE_NOT_FOUND); }
	SE_ERR_OOM =>
	  { errstr = w32_strerror (ERROR_NOT_ENOUGH_MEMORY); }
	SE_ERR_PNF =>
	  { errstr = w32_strerror (ERROR_PATH_NOT_FOUND); }
	SE_ERR_SHARE =>
	  { errstr = w32_strerror (ERROR_SHARING_VIOLATION); }
	_ =>
	  { errstr = w32_strerror (0); }
      }
  }

  /* C 8838-9002: `#else`.  */
  #[cfg (not (feature = "cygwin"))]
  {
  let use_unicode: c_int = w32_unicode_filenames as c_int;
  let mut doc_a: *mut c_char = core::ptr::null_mut ();
  let mut params_a: *mut c_char = core::ptr::null_mut ();
  let mut ops_a: *mut c_char = core::ptr::null_mut ();
  let mut absdoc: Lisp_Object;
  let mut handler: Lisp_Object;
  let mut success: BOOL;
  /* C 8840-8842: the local `const char file_url_str[] = "file:///"`, its `sizeof - 1` length and
     `int doclen;`.  */
  let file_url_len: c_int = 8;
  let mut doclen: c_int;

  /* Required on Windows 9X.  */
  maybe_load_unicows_dll ();

  if strncmp (SSDATA (document), c"file:///".as_ptr (), file_url_len as usize) == 0
    {
      /* Passing "file:///" URLs to ShellExecute causes shlwapi.dll to
	 start a thread in some rare system configurations, … */
      let mut p: *const c_char = SSDATA (document).offset (file_url_len as isize);

      if c_isalpha (*p as c_int) && *p.offset (1) == b':' as c_char
	  && IS_DIRECTORY_SEP! (*p.offset (2))
	{
	  document = Fsubstring_no_properties (document,
					       make_fixnum (file_url_len as EMACS_INT), Qnil ());
	}
      p = p;
    }
  /* We have a situation here.  If DOCUMENT is a relative file name, … */
  absdoc = Fexpand_file_name (document, Qnil ());
  /* Don't call file name handlers for file-exists-p, since they might … */
  handler = Ffind_file_name_handler (absdoc, Qfile_exists_p ());
  if NILP (handler)
    {
      let absdoc_encoded: Lisp_Object = Fcopy_sequence (ENCODE_FILE (absdoc));

      if faccessat (AT_FDCWD, SSDATA (absdoc_encoded), F_OK, AT_EACCESS) == 0
	{
	  /* ShellExecute fails if DOCUMENT is a UNC with forward
	     slashes …  */
	  unixtodos_filename (SSDATA (absdoc_encoded));
	  document = absdoc_encoded;
	}
      else
	{ document = ENCODE_FILE (document); }
    }
  else
    { document = ENCODE_FILE (document); }

  current_dir = ENCODE_FILE (current_dir);
  /* Cannot use filename_to_utf16/ansi with DOCUMENT, since it could
     be a URL that is not limited to MAX_PATH characters.  */
  doclen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
					     SSDATA (document), -1,
					     core::ptr::null_mut (), 0);
  doc_w = xmalloc (doclen as usize * core::mem::size_of::<c_ushort> ()) as *mut c_ushort;
  (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
				    SSDATA (document), -1, doc_w, doclen);
  if use_unicode != 0 && pfnShellExecuteExW.is_some ()
    {
      let mut current_dir_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];
      let mut shexinfo_w: SHELLEXECUTEINFOW = core::mem::zeroed ();

      /* Encode the current directory and parameters, and convert
	 operation to UTF-16.  */
      filename_to_utf16 (SSDATA (current_dir), current_dir_w.as_mut_ptr ());
      if STRINGP (parameters)
	{
	  let mut len: c_int;

	  parameters = ENCODE_SYSTEM (parameters);
	  len = (pMultiByteToWideChar.unwrap ()) (CP_ACP, multiByteToWideCharFlags,
						  SSDATA (parameters), -1,
						  core::ptr::null_mut (), 0);
	  if len > 32768
	    { len = 32768; }
	  params_w = sa_stack_alloc (len as usize * core::mem::size_of::<c_ushort> ())
	    as *mut c_ushort;
	  (pMultiByteToWideChar.unwrap ()) (CP_ACP, multiByteToWideCharFlags,
					    SSDATA (parameters), -1, params_w, len);
	  *params_w.offset (len as isize - 1) = 0;
	}
      if STRINGP (operation)
	{
	  /* Assume OPERATION is pure ASCII.  */
	  let mut s: *const c_char = SSDATA (operation);
	  let mut d: *mut c_ushort;
	  let mut len: c_int = SBYTES (operation) as c_int + 1;

	  if len > 32768
	    { len = 32768; }
	  d = sa_stack_alloc (len as usize * core::mem::size_of::<c_ushort> ()) as *mut c_ushort;
	  ops_w = d;
	  while d < ops_w.offset (len as isize - 1)
	    {
	      *d = *s as c_ushort;
	      d = d.offset (1);
	      s = s.offset (1);
	    }
	  *d = 0;
	}

      /* Using ShellExecuteEx and setting the SEE_MASK_INVOKEIDLIST
	 flag succeeds with more OPERATIONs (a.k.a. "verbs"), …  */
      memset ((&raw mut shexinfo_w) as *mut c_void, 0, core::mem::size_of_val (&shexinfo_w));
      shexinfo_w.cbSize = core::mem::size_of_val (&shexinfo_w) as DWORD;
      shexinfo_w.fMask =
	SEE_MASK_INVOKEIDLIST | SEE_MASK_FLAG_DDEWAIT | SEE_MASK_FLAG_NO_UI;
      shexinfo_w.hwnd = core::ptr::null_mut ();
      shexinfo_w.lpVerb = ops_w;
      shexinfo_w.lpFile = doc_w;
      shexinfo_w.lpParameters = params_w;
      shexinfo_w.lpDirectory = current_dir_w.as_mut_ptr ();
      shexinfo_w.nShow =
	if FIXNUMP (show_flag) { XFIXNUM (show_flag) as c_int } else { SW_SHOWDEFAULT };
      success = (pfnShellExecuteExW.unwrap ()) (&raw mut shexinfo_w);
      xfree (doc_w as *mut c_void);
    }
  else
    {
      let mut current_dir_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
      let mut shexinfo_a: SHELLEXECUTEINFOA = core::mem::zeroed ();
      let codepage: c_int = codepage_for_filenames (core::ptr::null_mut ());
      let ldoc_a: c_int = (pWideCharToMultiByte.unwrap ()) (codepage as UINT, 0, doc_w, -1,
							    core::ptr::null_mut (), 0,
							    core::ptr::null_mut (),
							    core::ptr::null_mut ());

      doc_a = xmalloc (ldoc_a as usize) as *mut c_char;
      (pWideCharToMultiByte.unwrap ()) (codepage as UINT, 0, doc_w, -1, doc_a, ldoc_a,
					core::ptr::null_mut (), core::ptr::null_mut ());
      filename_to_ansi (SSDATA (current_dir), current_dir_a.as_mut_ptr ());
      if STRINGP (parameters)
	{
	  parameters = ENCODE_SYSTEM (parameters);
	  params_a = SSDATA (parameters);
	}
      if STRINGP (operation)
	{
	  /* Assume OPERATION is pure ASCII.  */
	  ops_a = SSDATA (operation);
	}
      memset ((&raw mut shexinfo_a) as *mut c_void, 0, core::mem::size_of_val (&shexinfo_a));
      shexinfo_a.cbSize = core::mem::size_of_val (&shexinfo_a) as DWORD;
      shexinfo_a.fMask =
	SEE_MASK_INVOKEIDLIST | SEE_MASK_FLAG_DDEWAIT | SEE_MASK_FLAG_NO_UI;
      shexinfo_a.hwnd = core::ptr::null_mut ();
      shexinfo_a.lpVerb = ops_a;
      shexinfo_a.lpFile = doc_a;
      shexinfo_a.lpParameters = params_a;
      shexinfo_a.lpDirectory = current_dir_a.as_mut_ptr ();
      shexinfo_a.nShow =
	if FIXNUMP (show_flag) { XFIXNUM (show_flag) as c_int } else { SW_SHOWDEFAULT };
      success = ShellExecuteExA (&raw mut shexinfo_a);
      xfree (doc_w as *mut c_void);
      xfree (doc_a as *mut c_void);
    }

  if success != 0
    { return Qt (); }

  errstr = w32_strerror (0);
  }

  /* The error string might be encoded in the locale's encoding.  */
  if ! NILP (Vlocale_coding_system)
    {
      let decoded: Lisp_Object =
	code_convert_string_norecord (build_unibyte_string (errstr),
				      Vlocale_coding_system, false);
      errstr = SSDATA (decoded);
    }
  error (c"ShellExecute failed: %s".as_ptr (), errstr);
});

/// C 9045-9046's readers: `keyboard.h` 461's `parse_modifiers` and 497's
/// `lucid_event_type_list_p`, neither of which the `include/keyboard.rs` mirror carries yet
/// (`keyboard.c` 7566/8010 define them).
/// C: extern Lisp_Object parse_modifiers (Lisp_Object); extern bool lucid_event_type_list_p
/// C: (Lisp_Object);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn parse_modifiers (symbol: Lisp_Object) -> Lisp_Object;
  pub fn lucid_event_type_list_p (object: Lisp_Object) -> bool;
}

/// C 9101-9106: C's `#ifndef MOD_ALT` guard block - "Supply defs missing from mingw32", i.e. the
/// four `<winuser.h>` modifier bits carried with C's values and no `cfg` (the file's `#ifndef`
/// reading, `include/w32term.rs` 37-39's `ALIGN_STACK` note).
/// C: #define MOD_ALT 0x0001 / #define MOD_CONTROL 0x0002 / #define MOD_SHIFT 0x0004 /
/// C: #define MOD_WIN 0x0008
#[cfg (feature = "have_ntgui")]
pub const MOD_ALT: c_int = 0x0001;
#[cfg (feature = "have_ntgui")]
pub const MOD_CONTROL: c_int = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const MOD_SHIFT: c_int = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const MOD_WIN: c_int = 0x0008;

/* C 9016-9018's comment.  */
/// C 9019-9043: `lookup_vk_code` - C 9024's `for (i = 0; i < 256; i++)` over
/// `lispy_function_keys` (the mirror's `[*const c_char; 256]`), then C 9029-9040's
/// `w32_kbdhook_active! ()` block with C's own character comparisons and `c_toupper`
/// (`lib/include/c-ctype.rs`).  C's `char *key` is a string the caller owns, so the port's
/// parameter is `*const c_char` - the `w32_parse_and_hook_hot_key` call below is its only caller.
/// C: static int
/// C: lookup_vk_code (char *key)
#[cfg (feature = "have_ntgui")]
unsafe fn lookup_vk_code (key: *const c_char) -> c_int
{
  let mut i: c_int;

  i = 0;
  while i < 256
    {
      if ! lispy_function_keys[i as usize].is_null ()
	  && strcmp (lispy_function_keys[i as usize], key) == 0
	{ return i; }
      i += 1;
    }

  if w32_kbdhook_active! ()
    {
      /* Alphanumerics map to themselves.  */
      if *key.offset (1) == 0
	{
	  if (*key >= b'A' as c_char && *key <= b'Z' as c_char)
	      || (*key >= b'0' as c_char && *key <= b'9' as c_char)
	    { return *key as c_int; }
	  if *key >= b'a' as c_char && *key <= b'z' as c_char
	    { return c_toupper (*key as c_int); }
	}
    }

  return -1;
}

/* C 9045-9046's comment.  */
/// C 9047-9145: `w32_parse_and_hook_hot_key` - C 9050-9056's six locals (C's `register` keyword
/// has no Rust counterpart), C 9058-9069's `CHECK_VECTOR`/`ASIZE`/`AREF` chain with
/// `lucid_event_type_list_p`/`Fevent_convert_list`, C 9072-9091's symbol-versus-fixnum arms
/// (`parse_modifiers`, `SYMBOL_NAME`, `VK_ANY`, `CHARACTERBITS`), C 9096-9098's `alt_modifier`
/// rule, and C 9108-9144's two `w32_kbdhook_active! ()` arms - the first over `hook_w32_key` with
/// `VK_MENU`/`VK_LWIN`/`VK_RWIN` and the four modifier symbols, the second building C's
/// `w32_modifiers` and returning `HOTKEY! (…)`.
/// C: static Lisp_Object
/// C: w32_parse_and_hook_hot_key (Lisp_Object key, int hook)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_parse_and_hook_hot_key (key: Lisp_Object, hook: c_int) -> Lisp_Object
{
  let mut c: Lisp_Object;
  let mut vk_code: c_int = 0;
  let mut lisp_modifiers: c_int = 0;
  let mut w32_modifiers: c_int;
  let mut res: Lisp_Object = Qnil ();
  let vkname: *mut c_char;

  CHECK_VECTOR (key);

  if ASIZE (key) != 1
    { return Qnil (); }

  c = AREF (key, 0);

  if CONSP (c) && lucid_event_type_list_p (c)
    { c = Fevent_convert_list (c); }

  if ! FIXNUMP (c) && ! SYMBOLP (c)
    { error (c"Key definition is invalid".as_ptr ()); }

  /* Work out the base key and the modifiers.  */
  if SYMBOLP (c)
    {
      c = parse_modifiers (c);
      lisp_modifiers = XFIXNUM (Fcar (Fcdr (c))) as c_int;
      c = Fcar (c);
      if ! SYMBOLP (c)
	{ emacs_abort (); }
      vkname = SSDATA (SYMBOL_NAME (c));
      /* [s-], [M-], [h-]: Register all keys for this modifier */
      if w32_kbdhook_active! () && *vkname == 0
	{ vk_code = VK_ANY; }
      else
	{ vk_code = lookup_vk_code (vkname); }
    }
  else if FIXNUMP (c)
    {
      lisp_modifiers = (XFIXNUM (c) & ! (CHARACTERBITS as EMACS_INT)) as c_int;
      /* Many ascii characters are their own virtual key code.  */
      vk_code = (XFIXNUM (c) & (CHARACTERBITS as EMACS_INT)) as c_int;
    }

  if vk_code < 0 || vk_code > 255
    { return Qnil (); }

  if (lisp_modifiers & meta_modifier) != 0
      && ! NILP (Vw32_alt_is_meta)
    { lisp_modifiers |= alt_modifier; }

  /* Supply defs missing from mingw32.  (C 9101-9106's `#ifndef MOD_ALT` block stands above.)  */

  if w32_kbdhook_active! ()
    {
      /* Register Alt-x combinations.  */
      if lisp_modifiers & alt_modifier != 0
	{
	  hook_w32_key (hook, VK_MENU, vk_code);
	  res = Qt ();
	}
      /* Register Win-x combinations based on modifier mappings.  */
      if ((lisp_modifiers & hyper_modifier != 0)
	  && EQ (Vw32_lwindow_modifier, Qhyper ()))
	  || ((lisp_modifiers & super_modifier != 0)
	      && EQ (Vw32_lwindow_modifier, Qsuper ()))
	{
	  hook_w32_key (hook, VK_LWIN, vk_code);
	  res = Qt ();
	}
      if ((lisp_modifiers & hyper_modifier != 0)
	  && EQ (Vw32_rwindow_modifier, Qhyper ()))
	  || ((lisp_modifiers & super_modifier != 0)
	      && EQ (Vw32_rwindow_modifier, Qsuper ()))
	{
	  hook_w32_key (hook, VK_RWIN, vk_code);
	  res = Qt ();
	}
      return res;
    }
  else
    {
      /* Convert lisp modifiers to Windows hot-key form.  */
      w32_modifiers = if lisp_modifiers & hyper_modifier != 0 { MOD_WIN } else { 0 };
      w32_modifiers |= if lisp_modifiers & alt_modifier != 0 { MOD_ALT } else { 0 };
      w32_modifiers |= if lisp_modifiers & ctrl_modifier != 0 { MOD_CONTROL } else { 0 };
      w32_modifiers |= if lisp_modifiers & shift_modifier != 0 { MOD_SHIFT } else { 0 };

      return HOTKEY! (vk_code as EMACS_INT, w32_modifiers as EMACS_INT);
    }
}

/// C 9147-9198: `w32-register-hot-key` - C 9177's `w32_parse_and_hook_hot_key (key, 1)`, C 9179-9180's
/// `!w32_kbdhook_active` / `NILP (key)` / `Fmemq (key, w32_grabbed_keys)` test, C 9183's
/// `Fmemq (Qnil, w32_grabbed_keys)` slot search, C 9186-9189's `Fcons`-or-`XSETCAR` branch and
/// C 9193-9194's `PostThreadMessage (dwWindowsThreadId, WM_EMACS_REGISTER_HOT_KEY, …)`.  `key` is
/// C's reassigned parameter, so the body re-binds it with `let mut` (the `x-show-tip` `DEFUN!`
/// reading: the macro's `$arg:ident` cannot say `mut`).
/// C: DEFUN ("w32-register-hot-key", Fw32_register_hot_key,
/// C:        Sw32_register_hot_key, 1, 1, 0,
/// C:        doc: /* Register KEY as a hot-key combination. … */)
/// C:   (Lisp_Object key)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-register-hot-key", Fw32_register_hot_key,
	Sw32_register_hot_key, 1, 1, 0,
	doc: "Register KEY as a hot-key combination.\nCertain key combinations like Alt-Tab and Win-R are reserved for\nsystem use on Windows, and therefore are normally intercepted by the\nsystem.  These key combinations can be used in Emacs by registering\nthem as hot-keys, except for Win-L which always locks the computer.\n\nOn Windows 98 and ME, KEY must be a one element key definition in\nvector form that would be acceptable to `define-key' (e.g. [A-tab] for\nAlt-Tab).  The meta modifier is interpreted as Alt if\n`w32-alt-is-meta' is t, and hyper is always interpreted as the Windows\nmodifier keys.  The return value is the hotkey-id if registered,\notherwise nil.\n\nOn Windows versions since NT, KEY can also be specified as just a\nmodifier key, [M-], [s-] or [H-], to indicate that all combinations\nof the respective modifier key should be processed by Emacs instead\nof the operating system.  The super and hyper modifiers are\ninterpreted according to the current values of `w32-lwindow-modifier'\nand `w32-rwindow-modifier'.  For instance, setting `w32-lwindow-modifier'\nto `super' and then calling `(w32-register-hot-key [s-])' grabs all\ncombinations of the left Windows key to Emacs as keys with the Super\nmodifier, but leaves the right Windows key free for the operating\nsystem keyboard shortcuts.\n\nThe return value is t if the call affected any key combinations,\notherwise nil.",
	(key: Lisp_Object),
	{
  let mut key = key;

  key = w32_parse_and_hook_hot_key (key, 1);

  if ! w32_kbdhook_active! ()
      && ! NILP (key) && NILP (Fmemq (key, w32_grabbed_keys))
    {
      /* Reuse an empty slot if possible.  */
      let item: Lisp_Object = Fmemq (Qnil (), w32_grabbed_keys);

      /* Safe to add new key to list, even if we have focus.  */
      if NILP (item)
	{ w32_grabbed_keys = Fcons (key, w32_grabbed_keys); }
      else
	{ XSETCAR (item, key); }

      /* Notify input thread about new hot-key definition, so that it
	 takes effect without needing to switch focus.  */
      PostThreadMessage (dwWindowsThreadId, WM_EMACS_REGISTER_HOT_KEY as UINT,
			 XFIXNUM (key) as WPARAM, 0);
    }

  return key;
});

/// C 9200-9235: `w32-unregister-hot-key` - C 9207-9208's `FIXNUMP` guard and the
/// `w32_parse_and_hook_hot_key (key, 0)` parse, C 9210-9211's `w32_kbdhook_active` return, C 9213's
/// `Fmemq`, C 9219's `eassert (CONSP (item))`, C 9222's `XUNTAG (item, Lisp_Cons, struct Lisp_Cons)`
/// into the `LPARAM` the input thread reads as the list tail (C's own comment says why) and
/// C 9226-9231's `PostThreadMessage` + `GetMessage` wait for `WM_EMACS_DONE`.
/// C: DEFUN ("w32-unregister-hot-key", Fw32_unregister_hot_key,
/// C:        Sw32_unregister_hot_key, 1, 1, 0,
/// C:        doc: /* Unregister KEY as a hot-key combination.  */)
/// C:   (Lisp_Object key)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-unregister-hot-key", Fw32_unregister_hot_key,
	Sw32_unregister_hot_key, 1, 1, 0,
	doc: "Unregister KEY as a hot-key combination.",
	(key: Lisp_Object),
	{
  let mut key = key;
  let mut item: Lisp_Object;

  if ! FIXNUMP (key)
    { key = w32_parse_and_hook_hot_key (key, 0); }

  if w32_kbdhook_active! ()
    { return key; }

  item = Fmemq (key, w32_grabbed_keys);

  if ! NILP (item)
    {
      let mut lparam: LPARAM;

      eassert! (CONSP (item));
      /* Pass the tail of the list as a pointer to a Lisp_Cons cell,
	 so that it works in a --with-wide-int build as well.  */
      lparam = XUNTAG! (item, Lisp_Type::Lisp_Cons, Lisp_Cons) as LPARAM;

      /* Notify input thread about hot-key definition being removed, so
	 that it takes effect without needing focus switch.  */
      if PostThreadMessage (dwWindowsThreadId, WM_EMACS_UNREGISTER_HOT_KEY as UINT,
			     XFIXNUM (XCAR (item)) as WPARAM, lparam) != 0
	{
	  let mut msg: MSG = core::mem::zeroed ();   /* C's uninitialised `MSG msg;` (PORTING.md 1.5) */
	  GetMessage (&raw mut msg, core::ptr::null_mut (), WM_EMACS_DONE as UINT,
		      WM_EMACS_DONE as UINT);
	}
      return Qt ();
    }
  return Qnil ();
});

/// C 9237-9243: `w32-registered-hot-keys` - C 9242's one expression, `Fdelq (Qnil,
/// Fcopy_sequence (w32_grabbed_keys))`: the copy is C's (the caller must not see the live slot
/// list), and `Fdelq` drops the nil slots `w32-unregister-hot-key` leaves behind.
/// C: DEFUN ("w32-registered-hot-keys", Fw32_registered_hot_keys,
/// C:        Sw32_registered_hot_keys, 0, 0, 0,
/// C:        doc: /* Return list of registered hot-key IDs.  */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-registered-hot-keys", Fw32_registered_hot_keys,
	Sw32_registered_hot_keys, 0, 0, 0,
	doc: "Return list of registered hot-key IDs.",
	(),
	{
  return Fdelq (Qnil (), Fcopy_sequence (w32_grabbed_keys));
});

/// C 9245-9275: `w32-reconstruct-hot-key` - C 9254's `CHECK_FIXNUM`, C 9256-9257's
/// `HOTKEY_VK_CODE`/`HOTKEY_MODIFIERS` unpacking (the file's two macros above), C 9259-9262's
/// `lispy_function_keys`-or-`make_fixnum` choice, and C 9264-9272's four `Fcons` layers in C's own
/// order (shift, control, alt-or-meta by `Vw32_alt_is_meta`, then hyper for `MOD_WIN`).
/// C: DEFUN ("w32-reconstruct-hot-key", Fw32_reconstruct_hot_key,
/// C:        Sw32_reconstruct_hot_key, 1, 1, 0,
/// C:        doc: /* Convert hot-key ID to a lisp key combination.  */)
/// C:   (Lisp_Object hotkeyid)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-reconstruct-hot-key", Fw32_reconstruct_hot_key,
	Sw32_reconstruct_hot_key, 1, 1, 0,
	doc: "Convert hot-key ID to a lisp key combination.\nusage: (w32-reconstruct-hot-key ID)",
	(hotkeyid: Lisp_Object),
	{
  let mut vk_code: c_int;
  let mut w32_modifiers: c_int;
  let mut key: Lisp_Object;

  CHECK_FIXNUM (hotkeyid);

  vk_code = HOTKEY_VK_CODE! (hotkeyid) as c_int;
  w32_modifiers = HOTKEY_MODIFIERS! (hotkeyid) as c_int;

  if vk_code < 256 && ! lispy_function_keys[vk_code as usize].is_null ()
    { key = intern (lispy_function_keys[vk_code as usize]); }
  else
    { key = make_fixnum (vk_code as EMACS_INT); }

  key = Fcons (key, Qnil ());
  if w32_modifiers & MOD_SHIFT != 0
    { key = Fcons (Qshift (), key); }
  if w32_modifiers & MOD_CONTROL != 0
    { key = Fcons (Qctrl (), key); }
  if w32_modifiers & MOD_ALT != 0
    { key = Fcons (if NILP (Vw32_alt_is_meta) { Qalt () } else { Qmeta () }, key); }
  if w32_modifiers & MOD_WIN != 0
    { key = Fcons (Qhyper (), key); }

  return key;
});

/// C 9277-9316: `w32-toggle-lock-key` - C 9292-9299's three `EQ` arms over `Qcapslock` /
/// `Qkp_numlock` / `Qscroll` with `VK_CAPITAL` / `VK_NUMLOCK` / `VK_SCROLL`, C 9301-9302's
/// console-thread handoff to `w32_console_toggle_lock_key` (this file's own definition, landed
/// with this slice because the call needs it - PORTING.md P-91 ①), C 9304-9307's
/// `NILP`/`XUFIXNUM` `lparam` and C 9308-9313's `PostThreadMessage` + `GetMessage` wait whose
/// `msg.wParam` is the answer.
/// C: DEFUN ("w32-toggle-lock-key", Fw32_toggle_lock_key,
/// C:        Sw32_toggle_lock_key, 1, 2, 0,
/// C:        doc: /* Toggle the state of the lock key KEY. … */)
/// C:   (Lisp_Object key, Lisp_Object new_state)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-toggle-lock-key", Fw32_toggle_lock_key,
	Sw32_toggle_lock_key, 1, 2, 0,
	doc: "Toggle the state of the lock key KEY.\nKEY can be `capslock', `kp-numlock', or `scroll'.\nIf the optional parameter NEW-STATE is a number, then the state of KEY\nis set to off if the low bit of NEW-STATE is zero, otherwise on.\nIf NEW-STATE is omitted or nil, the function toggles the state,\n\nValue is the new state of the key, or nil if the function failed\nto change the state.",
	(key: Lisp_Object, new_state: Lisp_Object),
	{
  let mut vk_code: c_int;
  let mut lparam: LPARAM;

  if EQ (key, Qcapslock ())
    { vk_code = VK_CAPITAL; }
  else if EQ (key, Qkp_numlock ())
    { vk_code = VK_NUMLOCK; }
  else if EQ (key, Qscroll ())
    { vk_code = VK_SCROLL; }
  else
    { return Qnil (); }

  if dwWindowsThreadId == 0
    { return make_fixnum (w32_console_toggle_lock_key (vk_code, new_state) as EMACS_INT); }

  if NILP (new_state)
    { lparam = -1; }
  else
    { lparam = (XUFIXNUM (new_state) & 1) as LPARAM; }
  if PostThreadMessage (dwWindowsThreadId, WM_EMACS_TOGGLE_LOCK_KEY as UINT,
			vk_code as WPARAM, lparam) != 0
    {
      let mut msg: MSG = core::mem::zeroed ();   /* C's uninitialised `MSG msg;` (PORTING.md 1.5) */
      GetMessage (&raw mut msg, core::ptr::null_mut (), WM_EMACS_DONE as UINT,
		  WM_EMACS_DONE as UINT);
      return make_fixnum (msg.wParam as EMACS_INT);
    }
  return Qnil ();
});

/// C 9318-9337: `w32-window-exists-p` - C 9327-9330's two `NILP`/`CHECK_STRING` guards, C 9332-9333's
/// `FindWindow` call with C's two `STRINGP`-guarded `(LPCTSTR) SDATA (…)`-or-`NULL` arguments and
/// C 9334-9336's handle test.  C writes C's parameter name `class`, which is a Rust keyword, so the
/// port spells it `r#class` (PORTING.md 0.2's raw-identifier rule).
/// C: DEFUN ("w32-window-exists-p", Fw32_window_exists_p, Sw32_window_exists_p,
/// C:        2, 2, 0,
/// C:        doc: /* Return non-nil if a window exists with the specified CLASS and NAME. … */)
/// C:   (Lisp_Object class, Lisp_Object name)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-window-exists-p", Fw32_window_exists_p, Sw32_window_exists_p,
	2, 2, 0,
	doc: "Return non-nil if a window exists with the specified CLASS and NAME.\n\nThis is a direct interface to the Windows API FindWindow function.",
	(r#class: Lisp_Object, name: Lisp_Object),
	{
  let mut hnd: HWND;

  if ! NILP (r#class)
    { CHECK_STRING (r#class); }
  if ! NILP (name)
    { CHECK_STRING (name); }

  hnd = if w32_unicode_gui != 0
	  { FindWindowW (if STRINGP (r#class) { SDATA (r#class) as LPCWSTR }
			 else { core::ptr::null () },
			 if STRINGP (name) { SDATA (name) as LPCWSTR }
			 else { core::ptr::null () }) }
	else
	  { FindWindowA (if STRINGP (r#class) { SDATA (r#class) as LPCTSTR }
			 else { core::ptr::null () },
			 if STRINGP (name) { SDATA (name) as LPCTSTR }
			 else { core::ptr::null () }) };
  if hnd.is_null ()
    { return Qnil (); }
  return Qt ();
});

/// C 9398-9750's new `<windows.h>` surface: the three records the geometry DEFUNs fill
/// (`WINDOWINFO` is C 9386's, `MENUBARINFO` C 9385's), the five entry points they call and the five
/// constants they pass.  None of these names is in the reference file (they are `<winuser.h>`'s), so
/// each one is spelled from the header - the slice-8 reading - and the two records' layouts are
/// **unmeasured** on this machine (no SDK, no mingw toolchain): `WINDOWINFO`'s field list and
/// `MENUBARINFO`'s one bitfield word are corroborated against the Win32 metadata projection
/// (`windows-sys` 0.52's `WINDOWINFO`/`MENUBARINFO`, which gives the same field order and a
/// `_bitfield: i32` tail), not against a compiled `sizeof`.
/// C: #define SM_CYCAPTION 4 / #define SM_CYMENU 15 / #define GW_HWNDNEXT 2
/// C: #define SPI_GETMOUSETRAILS 0x005E / #define SPI_SETMOUSETRAILS 0x005D
/// C: typedef struct tagWINDOWINFO { DWORD cbSize; RECT rcWindow; RECT rcClient; DWORD dwStyle;
/// C: DWORD dwExStyle; DWORD dwWindowStatus; UINT cxWindowBorders; UINT cyWindowBorders;
/// C: ATOM atomWindowType; WORD wCreatorVersion; } WINDOWINFO, *PWINDOWINFO, *LPWINDOWINFO;
/// C: typedef struct tagMENUBARINFO { DWORD cbSize; RECT rcBar; HMENU hMenu; HWND hwndMenu;
/// C: BOOL fBarFocused : 1; BOOL fFocused : 1; } MENUBARINFO, *PMENUBARINFO, *LPMENUBARINFO;
/// C: BOOL WINAPI GetWindowInfo (HWND hwnd, PWINDOWINFO pwi);
/// C: BOOL WINAPI GetMenuBarInfo (HWND hwnd, LONG idObject, LONG idItem, PMENUBARINFO pmbi);
/// C: HWND WINAPI GetTopWindow (HWND hWnd);
/// C: BOOL WINAPI ClientToScreen (HWND hWnd, LPPOINT lpPoint);
/// C: BOOL WINAPI SetCursorPos (int X, int Y);
#[cfg (feature = "have_ntgui")]
pub const SM_CYCAPTION: c_int = 4;
#[cfg (feature = "have_ntgui")]
pub const SM_CYMENU: c_int = 15;
#[cfg (feature = "have_ntgui")]
pub const GW_HWNDNEXT: UINT = 2;
#[cfg (feature = "have_ntgui")]
pub const SPI_GETMOUSETRAILS: UINT = 0x005E;
#[cfg (feature = "have_ntgui")]
pub const SPI_SETMOUSETRAILS: UINT = 0x005D;
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOWINFO
{
  pub cbSize: DWORD,
  pub rcWindow: RECT,
  pub rcClient: RECT,
  pub dwStyle: DWORD,
  pub dwExStyle: DWORD,
  pub dwWindowStatus: DWORD,
  pub cxWindowBorders: UINT,
  pub cyWindowBorders: UINT,
  pub atomWindowType: ATOM,
  pub wCreatorVersion: WORD,
}
/// The two `BOOL … : 1` bitfields are one `bits` word, LSB first: `fBarFocused:1 | fFocused:1`
/// (PORTING.md 1.3, the `src/w32.rs` `DCB` device).  Nothing in this file writes either bit.
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENUBARINFO
{
  pub cbSize: DWORD,
  pub rcBar: RECT,
  pub hMenu: HMENU,
  pub hwndMenu: HWND,
  pub bits: DWORD,
}
/// C 9383/9494/9696's `frame.h` 1476 prototype, which this unit's include list does not carry (the
/// `decode_window_system_frame` shape below).
/// C: extern struct frame *decode_live_frame (Lisp_Object);
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn decode_live_frame (arg0: Lisp_Object) -> *mut frame;
}
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetWindowInfo (hwnd: HWND, pwi: *mut WINDOWINFO) -> BOOL;
  pub fn GetMenuBarInfo (hwnd: HWND, idObject: LONG, idItem: LONG,
                         pmbi: *mut MENUBARINFO) -> BOOL;
  pub fn GetTopWindow (hWnd: HWND) -> HWND;
  pub fn ClientToScreen (hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
  pub fn SetCursorPos (X: c_int, Y: c_int) -> BOOL;
}

/// C 9339-9486: `w32-frame-geometry` - the alist of C 9450-9485.  C 9385-9393's locals (two
/// records and eleven scalars, the records zero-initialised per PORTING.md 1.5), C 9395-9396's
/// initial/W32 test, C 9400-9403's `GetWindowInfo` fill and the two border reads, C 9405-9422's
/// `get_title_bar_info_fn` block (`TITLEBAR_INFO` is C 167-190's pair above, its `rgstate[0]` test is
/// C 9415's `!(rgstate[0] & 0x00008001)`), C 9423-9424's `WS_CAPTION` fallback, C 9426-9432's
/// `GetMenuBarInfo` and the two menu metrics, C 9435-9438's four rectangle edges and C 9441-9448's
/// menu-bar height fix-up.  C 9410-9411 and C 9427-9428 are C's chained assignments: two statements
/// each, innermost store first (the C 6051-6052 `tip_window` reading).
/// C: DEFUN ("w32-frame-geometry", Fw32_frame_geometry, Sw32_frame_geometry, 0, 1, 0,
/// C:        doc: /* Return geometric attributes of FRAME. … */)
/// C:   (Lisp_Object frame)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-frame-geometry", Fw32_frame_geometry, Sw32_frame_geometry, 0, 1, 0,
	doc: "Return geometric attributes of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is an association list of the attributes listed below.  All height\nand width values are in pixels.\n\n`outer-position' is a cons of the outer left and top edges of FRAME\n  relative to the origin - the position (0, 0) - of FRAME's display.\n\n`outer-size' is a cons of the outer width and height of FRAME.  The\n  outer size includes the title bar and the external borders as well as\n  any menu and/or tool bar of frame.\n\n`external-border-size' is a cons of the horizontal and vertical width of\n  FRAME's external borders as supplied by the window manager.\n\n`title-bar-size' is a cons of the width and height of the title bar of\n  FRAME as supplied by the window manager.  If both of them are zero,\n  FRAME has no title bar.  If only the width is zero, Emacs was not\n  able to retrieve the width information.\n\n`menu-bar-external', if non-nil, means the menu bar is external (never\n  included in the inner edges of FRAME).\n\n`menu-bar-size' is a cons of the width and height of the menu bar of\n  FRAME.\n\n`tab-bar-size' is a cons of the width and height of the tab bar of\n  FRAME.\n\n`tool-bar-external', if non-nil, means the tool bar is external (never\n  included in the inner edges of FRAME).\n\n`tool-bar-position' tells on which side the tool bar on FRAME is and can\n  be one of `left', `top', `right' or `bottom'.  If this is nil, FRAME\n  has no tool bar.\n\n`tool-bar-size' is a cons of the width and height of the tool bar of\n  FRAME.\n\n`internal-border-width' is the width of the internal border of\n  FRAME.",
	(frame: Lisp_Object),
	{
  let f: *mut frame = decode_live_frame (frame);

  let mut menu_bar: MENUBARINFO = core::mem::zeroed ();
  let mut window: WINDOWINFO = core::mem::zeroed ();
  let mut left: c_int;
  let mut top: c_int;
  let mut right: c_int;
  let mut bottom: c_int;
  let mut external_border_width: c_uint;
  let mut external_border_height: c_uint;
  let mut title_bar_width: c_int = 0;
  let mut title_bar_height: c_int = 0;
  let mut single_menu_bar_height: c_int;
  let mut wrapped_menu_bar_height: c_int;
  let mut menu_bar_height: c_int;
  let tab_bar_height: c_int = FRAME_TAB_BAR_HEIGHT (f);
  let tool_bar_height: c_int = FRAME_TOOL_BAR_HEIGHT (f);
  let internal_border_width: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);

  if FRAME_INITIAL_P (f) || ! FRAME_W32_P (f)
    { return Qnil (); }

  block_input ();
  /* Outer rectangle and borders.  */
  window.cbSize = core::mem::size_of_val (&window) as DWORD;
  GetWindowInfo (FRAME_W32_WINDOW! (f), &raw mut window);
  external_border_width = window.cxWindowBorders;
  external_border_height = window.cyWindowBorders;
  /* Title bar.  */
  if get_title_bar_info_fn.is_some ()
    {
      let mut title_bar: TITLEBAR_INFO = core::mem::zeroed ();

      title_bar.cbSize = core::mem::size_of_val (&title_bar) as DWORD;
      title_bar.rcTitleBar.left = 0;
      title_bar.rcTitleBar.right = 0;
      title_bar.rcTitleBar.top = 0;
      title_bar.rcTitleBar.bottom = 0;
      let mut i: c_int = 0;
      while i < 6
	{
	  title_bar.rgstate[i as usize] = 0;
	  i += 1;
	}
      if (get_title_bar_info_fn.unwrap ()) (FRAME_W32_WINDOW! (f), &raw mut title_bar) != 0
	  && title_bar.rgstate[0] & 0x00008001 == 0
	{
	  title_bar_width = title_bar.rcTitleBar.right - title_bar.rcTitleBar.left;
	  title_bar_height = title_bar.rcTitleBar.bottom - title_bar.rcTitleBar.top;
	}
    }
  else if (window.dwStyle & WS_CAPTION) == WS_CAPTION
    { title_bar_height = GetSystemMetrics (SM_CYCAPTION); }
  /* Menu bar.  */
  menu_bar.cbSize = core::mem::size_of_val (&menu_bar) as DWORD;
  menu_bar.rcBar.left = 0;
  menu_bar.rcBar.right = 0;
  menu_bar.rcBar.top = 0;
  menu_bar.rcBar.bottom = 0;

  GetMenuBarInfo (FRAME_W32_WINDOW! (f), 0xFFFFFFFD, 0, &raw mut menu_bar);
  single_menu_bar_height = GetSystemMetrics (SM_CYMENU);
  wrapped_menu_bar_height = GetSystemMetrics (SM_CYMENUSIZE);
  unblock_input ();

  left = window.rcWindow.left;
  top = window.rcWindow.top;
  right = window.rcWindow.right;
  bottom = window.rcWindow.bottom;

  /* Menu bar.  */
  menu_bar_height = menu_bar.rcBar.bottom - menu_bar.rcBar.top;
  /* Fix menu bar height reported by GetMenuBarInfo.  */
  if menu_bar_height > single_menu_bar_height
    /* A wrapped menu bar.  */
    { menu_bar_height += single_menu_bar_height - wrapped_menu_bar_height; }
  else if menu_bar_height > 0
    /* A single line menu bar.  */
    { menu_bar_height = single_menu_bar_height; }

  return list! (Fcons (Qouter_position (),
		       Fcons (make_fixnum (left as EMACS_INT), make_fixnum (top as EMACS_INT))),
		Fcons (Qouter_size (),
		       Fcons (make_fixnum ((right - left) as EMACS_INT),
			      make_fixnum ((bottom - top) as EMACS_INT))),
		Fcons (Qexternal_border_size (),
		       Fcons (make_fixnum (external_border_width as EMACS_INT),
			      make_fixnum (external_border_height as EMACS_INT))),
		Fcons (Qtitle_bar_size (),
		       Fcons (make_fixnum (title_bar_width as EMACS_INT),
			      make_fixnum (title_bar_height as EMACS_INT))),
		Fcons (Qmenu_bar_external (), Qt ()),
		Fcons (Qmenu_bar_size (),
		       Fcons (make_fixnum ((menu_bar.rcBar.right - menu_bar.rcBar.left) as EMACS_INT),
			      make_fixnum (menu_bar_height as EMACS_INT))),
		Fcons (Qtab_bar_size (),
		       Fcons (make_fixnum
			      (if tab_bar_height != 0
				 { (right - left - 2 * (external_border_width as c_int)
				    - 2 * internal_border_width) as EMACS_INT }
			       else { 0 }),
			      make_fixnum (tab_bar_height as EMACS_INT))),
		Fcons (Qtool_bar_external (), Qnil ()),
		Fcons (Qtool_bar_position (), (if tool_bar_height != 0
					       { FRAME_TOOL_BAR_POSITION (f) }
					       else { Qnil () })),
		Fcons (Qtool_bar_size (),
		       Fcons (make_fixnum
			      (if tool_bar_height != 0
				 { (right - left - 2 * (external_border_width as c_int)
				    - 2 * internal_border_width) as EMACS_INT }
			       else { 0 }),
			      make_fixnum (tool_bar_height as EMACS_INT))),
		Fcons (Qinternal_border_width (),
		       make_fixnum (internal_border_width as EMACS_INT)));
});

/// C 9488-9577: `w32-frame-edges` - C 9506's frame decode and C 9508-9509's test, then C 9511-9528's
/// `outer-edges` arm (`GetWindowRect`'s rectangle straight into `list4`) and C 9529-9576's other arm
/// (`GetClientRect` plus `ClientToScreen` for the native rectangle, then `inner-edges`' four
/// `FRAME_*`-adjusted coordinates or the native rectangle itself).  `success` is C's `BOOL`, so the
/// port keeps the type and spells C's `false` as `0` (the `record_in_backtrace` notes row class).
/// C: DEFUN ("w32-frame-edges", Fw32_frame_edges, Sw32_frame_edges, 0, 2, 0,
/// C:        doc: /* Return edge coordinates of FRAME. … */)
/// C:   (Lisp_Object frame, Lisp_Object type)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-frame-edges", Fw32_frame_edges, Sw32_frame_edges, 0, 2, 0,
	doc: "Return edge coordinates of FRAME.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a list of the form (LEFT, TOP, RIGHT, BOTTOM).  All values are\nin pixels relative to the origin - the position (0, 0) - of FRAME's\ndisplay.\n\nIf optional argument TYPE is the symbol `outer-edges', return the outer\nedges of FRAME.  The outer edges comprise the decorations of the window\nmanager (like the title bar or external borders) as well as any external\nmenu or tool bar of FRAME.  If optional argument TYPE is the symbol\n`native-edges' or nil, return the native edges of FRAME.  The native\nedges exclude the decorations of the window manager and any external\nmenu or tool bar of FRAME.  If TYPE is the symbol `inner-edges', return\nthe inner edges of FRAME.  These edges exclude title bar, any borders,\nmenu bar or tool bar of FRAME.",
	(frame: Lisp_Object, r#type: Lisp_Object),
	{
  let f: *mut frame = decode_live_frame (frame);

  if FRAME_INITIAL_P (f) || ! FRAME_W32_P (f)
    { return Qnil (); }

  if EQ (r#type, Qouter_edges ())
    {
      let mut rectangle: RECT = core::mem::zeroed ();
      let mut success: BOOL = 0;

      block_input ();
      /* Outer frame rectangle, including outer borders and title bar. */
      success = GetWindowRect (FRAME_W32_WINDOW! (f), &raw mut rectangle);
      unblock_input ();

      if success != 0
	{ return list4 (make_fixnum (rectangle.left as EMACS_INT),
			make_fixnum (rectangle.top as EMACS_INT),
			make_fixnum (rectangle.right as EMACS_INT),
			make_fixnum (rectangle.bottom as EMACS_INT)); }
      else
	{ return Qnil (); }
    }
  else
    {
      let mut rectangle: RECT = core::mem::zeroed ();
      let mut pt: POINT = core::mem::zeroed ();
      let mut left: c_int;
      let mut top: c_int;
      let mut right: c_int;
      let mut bottom: c_int;
      let mut success: BOOL;

      block_input ();
      /* Inner frame rectangle, excluding borders and title bar.  */
      success = GetClientRect (FRAME_W32_WINDOW! (f), &raw mut rectangle);
      /* Get top-left corner of native rectangle in screen
	 coordinates.  */
      if success == 0
	{
	  unblock_input ();
	  return Qnil ();
	}

      pt.x = 0;
      pt.y = 0;
      success = ClientToScreen (FRAME_W32_WINDOW! (f), &raw mut pt);
      unblock_input ();

      if success == 0
	{ return Qnil (); }

      left = pt.x;
      top = pt.y;
      right = left + rectangle.right;
      bottom = top + rectangle.bottom;

      if EQ (r#type, Qinner_edges ())
	{
	  let internal_border_width: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);

	  return list4 (make_fixnum ((left + internal_border_width) as EMACS_INT),
			make_fixnum ((top
				      + FRAME_TAB_BAR_HEIGHT (f)
				      + FRAME_TOOL_BAR_TOP_HEIGHT (f)
				      + internal_border_width) as EMACS_INT),
			make_fixnum ((right - internal_border_width) as EMACS_INT),
			make_fixnum ((bottom - internal_border_width
				      - FRAME_TOOL_BAR_BOTTOM_HEIGHT (f)) as EMACS_INT));
	}
      else
	{ return list4 (make_fixnum (left as EMACS_INT), make_fixnum (top as EMACS_INT),
			make_fixnum (right as EMACS_INT), make_fixnum (bottom as EMACS_INT)); }
    }
});

/// C 9579-9607: `static Lisp_Object w32_frame_list_z_order (struct w32_display_info *, HWND)` -
/// C 9589's two locals, C 9591's `while (window)` walk with C 9593's `w32_window_to_frame`, C 9597's
/// `XSETFRAME`/`Fcons` pair, C 9602's `GetNextWindow (window, GW_HWNDNEXT)` (C's macro: the port
/// calls `<winuser.h>`'s real entry point, the notes row records the name) and C 9606's `Fnreverse`.
/// C: static Lisp_Object
/// C: w32_frame_list_z_order (struct w32_display_info *dpyinfo, HWND window)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_list_z_order (dpyinfo: *mut w32_display_info, mut window: HWND) -> Lisp_Object
{
  let mut frame: Lisp_Object;
  let mut frames: Lisp_Object = Qnil ();

  while ! window.is_null ()
    {
      let f: *mut frame = w32_window_to_frame (dpyinfo, window);

      if ! f.is_null ()
	{
	  XSETFRAME! (frame, f);
	  frames = Fcons (frame, frames);
	}

      block_input ();
      window = GetWindow (window, GW_HWNDNEXT);
      unblock_input ();
    }

  return Fnreverse (frames);
}

/// C 9609-9633: `w32-frame-list-z-order` - C 9622's `check_x_display_info`, C 9626-9629's
/// `FRAMEP`/`FRAME_LIVE_P` choice between `GetWindow (…, GW_CHILD)` (a live frame argument) and
/// `GetTopWindow (NULL)`, and C 9632's call into the walk above.
/// C: DEFUN ("w32-frame-list-z-order", Fw32_frame_list_z_order,
/// C:        Sw32_frame_list_z_order, 0, 1, 0,
/// C:        doc: /* Return list of Emacs's frames, in Z (stacking) order. … */)
/// C:   (Lisp_Object display)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-frame-list-z-order", Fw32_frame_list_z_order,
	Sw32_frame_list_z_order, 0, 1, 0,
	doc: "Return list of Emacs's frames, in Z (stacking) order.\nThe optional argument DISPLAY specifies which display to ask about.\nDISPLAY should be either a frame or a display name (a string).  If\nomitted or nil, that stands for the selected frame's display.\n\nAs a special case, if DISPLAY is non-nil and specifies a live frame,\nreturn the child frames of that frame in Z (stacking) order.\n\nFrames are listed from topmost (first) to bottommost (last).",
	(display: Lisp_Object),
	{
  let dpyinfo: *mut w32_display_info = check_x_display_info (display);
  let mut window: HWND;

  block_input ();
  if FRAMEP (display) && FRAME_LIVE_P (XFRAME (display))
    { window = GetWindow (FRAME_W32_WINDOW! (XFRAME (display)), GW_CHILD); }
  else
    { window = GetTopWindow (core::ptr::null_mut ()); }
  unblock_input ();

  return w32_frame_list_z_order (dpyinfo, window);
});

/// C 9635-9674: `static void w32_frame_restack (struct frame *, struct frame *, bool)` - C 9646-9647's
/// two handles, C 9650-9667's `above_flag` arm (the two `GetNextWindow` guards around the two
/// `SetWindowPos` calls, whose comments explain why F1 goes below F2 first) and C 9668-9672's
/// single-call `else if` arm.  `SetWindowPos`'s `SWP_*` set is C 3183-3185's.
/// C: static void
/// C: w32_frame_restack (struct frame *f1, struct frame *f2, bool above_flag)
#[cfg (feature = "have_ntgui")]
unsafe fn w32_frame_restack (f1: *mut frame, f2: *mut frame, above_flag: bool)
{
  let hwnd1: HWND = FRAME_W32_WINDOW! (f1);
  let hwnd2: HWND = FRAME_W32_WINDOW! (f2);

  block_input ();
  if above_flag
    /* Put F1 above F2 in the z-order.  */
    {
      if GetWindow (hwnd1, GW_HWNDNEXT) != hwnd2
	{
	  /* Make sure F1 is below F2 first because we must not
	     change the relative position of F2 wrt any other
	     window but F1.  */
	  if GetWindow (hwnd2, GW_HWNDNEXT) != hwnd1
	    { SetWindowPos (hwnd1, hwnd2, 0, 0, 0, 0,
			    SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
			    | SWP_FRAMECHANGED); }
	  /* Now put F1 above F2.  */
	  SetWindowPos (hwnd2, hwnd1, 0, 0, 0, 0,
			SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
			| SWP_FRAMECHANGED);
	}
    }
  else if GetWindow (hwnd2, GW_HWNDNEXT) != hwnd1
    /* Put F1 below F2 in the z-order.  */
    { SetWindowPos (hwnd1, hwnd2, 0, 0, 0, 0,
		    SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE
		    | SWP_FRAMECHANGED); }
  unblock_input ();
}

/// C 9676-9707: `w32-frame-restack` - C 9694-9695's two frame decodes, C 9697's `FRAME_W32_P` pair,
/// C 9699's call with C's `!NILP (above)` and C 9704's `error ("Cannot restack frames")`.
/// C: DEFUN ("w32-frame-restack", Fw32_frame_restack, Sw32_frame_restack, 2, 3, 0,
/// C:        doc: /* Restack FRAME1 below FRAME2. … */)
/// C:      (Lisp_Object frame1, Lisp_Object frame2, Lisp_Object above)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-frame-restack", Fw32_frame_restack, Sw32_frame_restack, 2, 3, 0,
	doc: "Restack FRAME1 below FRAME2.\nThis means that if both frames are visible and the display areas of\nthese frames overlap, FRAME2 (partially) obscures FRAME1.  If optional\nthird argument ABOVE is non-nil, restack FRAME1 above FRAME2.  This\nmeans that if both frames are visible and the display areas of these\nframes overlap, FRAME1 (partially) obscures FRAME2.\n\nThis may be thought of as an atomic action performed in two steps: The\nfirst step removes FRAME1's window-system window from the display.  The\nsecond step reinserts FRAME1's window below (above if ABOVE is true)\nthat of FRAME2.  Hence the position of FRAME2 in its display's Z\n(stacking) order relative to all other frames excluding FRAME1 remains\nunaltered.\n\nSome window managers may refuse to restack windows.",
	(frame1: Lisp_Object, frame2: Lisp_Object, above: Lisp_Object),
	{
  let f1: *mut frame = decode_live_frame (frame1);
  let f2: *mut frame = decode_live_frame (frame2);

  if FRAME_W32_P (f1) && FRAME_W32_P (f2)
    {
      w32_frame_restack (f1, f2, ! NILP (above));
      return Qt ();
    }
  else
    {
      error (c"Cannot restack frames".as_ptr ());
      return Qnil ();
    }
});

/// C 9709-9724: `w32-mouse-absolute-pixel-position` - C 9717's `POINT` (zero-initialised,
/// PORTING.md 1.5), C 9720's `GetCursorPos` and C 9723's `Fcons` of the two coordinates.
/// C: DEFUN ("w32-mouse-absolute-pixel-position", Fw32_mouse_absolute_pixel_position,
/// C:        Sw32_mouse_absolute_pixel_position, 0, 0, 0,
/// C:        doc: /* Return absolute position of mouse cursor in pixels. … */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-mouse-absolute-pixel-position", Fw32_mouse_absolute_pixel_position,
	Sw32_mouse_absolute_pixel_position, 0, 0, 0,
	doc: "Return absolute position of mouse cursor in pixels.\nThe position is returned as a cons cell (X . Y) of the coordinates of\nthe mouse cursor position in pixels relative to a position (0, 0) of the\nselected frame's display.",
	(),
	{
  let mut pt: POINT = core::mem::zeroed ();

  block_input ();
  GetCursorPos (&raw mut pt);
  unblock_input ();

  return Fcons (make_fixnum (pt.x as EMACS_INT), make_fixnum (pt.y as EMACS_INT));
});

/// C 9726-9752: `w32-set-mouse-absolute-pixel-position` - C 9733-9737's four locals (`ret` is C's
/// `BOOL`, so C's `false` is the port's `0`), C 9739-9749's body: the mouse-trails guard pair on NT
/// 6.0+ (`os_subtype`, `w32_major_version`/`w32_minor_version`), `SPI_GETMOUSETRAILS`'s saved
/// `trail_num`, `SetCursorPos` and `SPI_SETMOUSETRAILS`'s restore.
/// C: DEFUN ("w32-set-mouse-absolute-pixel-position", Fw32_set_mouse_absolute_pixel_position,
/// C:        Sw32_set_mouse_absolute_pixel_position, 2, 2, 0,
/// C:        doc: /* Move mouse pointer to absolute pixel position (X, Y). … */)
/// C:   (Lisp_Object x, Lisp_Object y)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-set-mouse-absolute-pixel-position", Fw32_set_mouse_absolute_pixel_position,
	Sw32_set_mouse_absolute_pixel_position, 2, 2, 0,
	doc: "Move mouse pointer to absolute pixel position (X, Y).\nThe coordinates X and Y are interpreted in pixels relative to a position\n(0, 0) of the selected frame's display.",
	(x: Lisp_Object, y: Lisp_Object),
	{
  let mut trail_num: UINT = 0;
  let mut ret: BOOL = 0;

  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

  block_input ();
  /* When "mouse trails" are in effect, moving the mouse cursor
     sometimes leaves behind an annoying "ghost" of the pointer.
     Avoid that by momentarily switching off mouse trails.  */
  if os_subtype == OS_SUBTYPE_NT
      && w32_major_version + w32_minor_version >= 6
    { ret = SystemParametersInfo (SPI_GETMOUSETRAILS, 0,
				  &raw mut trail_num as *mut c_void, 0); }
  SetCursorPos (xval, yval);
  if ret != 0
    { SystemParametersInfo (SPI_SETMOUSETRAILS, trail_num, core::ptr::null_mut (), 0); }
  unblock_input ();

  return Qnil ();
});

/* C 9755's `#ifdef WINDOWSNT` opens here and C 9890's `#endif` closes it; C 9893-9999 is the
   second such block.  Both DEFUNs' `have_ntgui`-and-`windowsnt` gates below are that nesting.  */

/// C 9756-9759: the two `GetDiskFreeSpaceEx` pointer types.  `get_proc_addr` (`w32common.h`
/// 65-70) returns `VOIDFNPTR`, so each cast is the `transmute` the `w32menu.rs` loader uses.
/// C: typedef BOOL (WINAPI *GetDiskFreeSpaceExW_Proc)
/// C:   (LPCWSTR, PULARGE_INTEGER, PULARGE_INTEGER, PULARGE_INTEGER);
/// C: typedef BOOL (WINAPI *GetDiskFreeSpaceExA_Proc)
/// C:   (LPCSTR, PULARGE_INTEGER, PULARGE_INTEGER, PULARGE_INTEGER);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type LPCSTR = *const c_char;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetDiskFreeSpaceExW_Proc
  = Option<unsafe extern "system" fn (arg0: LPCWSTR, arg1: PULARGE_INTEGER, arg2: PULARGE_INTEGER,
				      arg3: PULARGE_INTEGER) -> BOOL>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetDiskFreeSpaceExA_Proc
  = Option<unsafe extern "system" fn (arg0: LPCSTR, arg1: PULARGE_INTEGER, arg2: PULARGE_INTEGER,
				      arg3: PULARGE_INTEGER) -> BOOL>;

/// C 9840-9857's `LARGE_INTEGER` and C 9846-9848's `ULARGE_INTEGER` casts: the two `<winnt.h>`
/// unions and their pointer, spelled as `src/w32.rs` 392-404 / 1391-1404 spells them (the same
/// `QuadPart` member C reads, `LONGLONG`'s `i64`).
/// C: typedef union _LARGE_INTEGER { struct { DWORD LowPart; LONG HighPart; } u;
/// C:   LONGLONG QuadPart; } LARGE_INTEGER;
/// C: typedef union _ULARGE_INTEGER { struct { DWORD LowPart; DWORD HighPart; } u;
/// C:   ULONGLONG QuadPart; } ULARGE_INTEGER;
/// C: typedef ULARGE_INTEGER *PULARGE_INTEGER;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LARGE_INTEGER_u
{
  pub LowPart: DWORD,
  pub HighPart: LONG,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union LARGE_INTEGER
{
  pub u: LARGE_INTEGER_u,
  pub QuadPart: i64,                       /* C's `LONGLONG` (PORTING.md 1.1's `i64` row) */
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ULARGE_INTEGER_u
{
  pub LowPart: DWORD,
  pub HighPart: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ULARGE_INTEGER
{
  pub u: ULARGE_INTEGER_u,
  pub QuadPart: u64,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type PULARGE_INTEGER = *mut ULARGE_INTEGER;

/// C 9866-9877's two `<fileapi.h>` entry points (the A/W pair C's `w32_unicode_filenames` test
/// picks between).
/// C: BOOL WINAPI GetDiskFreeSpaceW (LPCWSTR lpRootPathName, LPDWORD lpSectorsPerCluster,
/// C: LPDWORD lpBytesPerSector, LPDWORD lpNumberOfFreeClusters, LPDWORD lpTotalNumberOfClusters);
/// C: BOOL WINAPI GetDiskFreeSpaceA (LPCSTR lpRootPathName, …);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" {
  pub fn GetDiskFreeSpaceW (lpRootPathName: LPCWSTR, lpSectorsPerCluster: LPDWORD,
			    lpBytesPerSector: LPDWORD, lpNumberOfFreeClusters: LPDWORD,
			    lpTotalNumberOfClusters: LPDWORD) -> BOOL;
  pub fn GetDiskFreeSpaceA (lpRootPathName: LPCSTR, lpSectorsPerCluster: LPDWORD,
			    lpBytesPerSector: LPDWORD, lpNumberOfFreeClusters: LPDWORD,
			    lpTotalNumberOfClusters: LPDWORD) -> BOOL;
}

/// C 9761-9890: `file-system-info` - C 9765's two locals (and C 9768-9769's `filename`
/// reassignment, so the port re-binds the parameter), C 9767-9769's `CHECK_STRING` /
/// `Fexpand_file_name` / `ENCODE_FILE`, C 9773-9780's handler probe (`Ffind_file_name_handler`,
/// `calln`, the `CONSP`-or-`NILP` early return and C 9779's `error`), C 9789-9797's dynamic probe
/// for `GetDiskFreeSpaceExW`/`A` with `w32_unicode_filenames` choosing which pointer matters,
/// C 9801-9804's three root-name buffers and C 9808-9829's two root-name forms (the drive-letter
/// case and the UNC prefix walk with C's `do … while (*name)`), C 9831-9834's
/// `filename_to_utf16`/`filename_to_ansi` choice, C 9836-9858's `LARGE_INTEGER` arm (`list3` of the
/// three `QuadPart` doubles, C's own comment about the unsigned-to-double cast) and C 9859-9885's
/// `GetDiskFreeSpaceW`/`A` arm with the three products.
/// C: DEFUN ("file-system-info", Ffile_system_info, Sfile_system_info, 1, 1, 0,
/// C:        doc: /* SKIP: Real doc in fileio.c.  */)
/// C:   (Lisp_Object filename)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("file-system-info", Ffile_system_info, Sfile_system_info, 1, 1, 0,
	doc: "SKIP: Real doc in fileio.c.",
	(filename: Lisp_Object),
	{
  let mut filename = filename;
  let mut encoded: Lisp_Object;
  let mut value: Lisp_Object;

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());
  encoded = ENCODE_FILE (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object = Ffind_file_name_handler (encoded, Qfile_system_info ());
  if ! NILP (handler)
    {
      value = calln! (handler, Qfile_system_info (), encoded);
      if CONSP (value) || NILP (value)
	{ return value; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  value = Qnil ();

  /* Determining the required information on Windows turns out, sadly,
     to be more involved than one would hope.  The original Windows API
     call for this will return bogus information on some systems, but we
     must dynamically probe for the replacement api, since that was
     added rather late on.  */
  {
    let hKernel: HMODULE = GetModuleHandle (c"kernel32".as_ptr ());
    let pfn_GetDiskFreeSpaceExW: GetDiskFreeSpaceExW_Proc
      = core::mem::transmute::<VOIDFNPTR, GetDiskFreeSpaceExW_Proc> (
	  get_proc_addr (hKernel, c"GetDiskFreeSpaceExW".as_ptr ()));
    let pfn_GetDiskFreeSpaceExA: GetDiskFreeSpaceExA_Proc
      = core::mem::transmute::<VOIDFNPTR, GetDiskFreeSpaceExA_Proc> (
	  get_proc_addr (hKernel, c"GetDiskFreeSpaceExA".as_ptr ()));
    let have_pfn_GetDiskFreeSpaceEx: bool
      = (w32_unicode_filenames && pfn_GetDiskFreeSpaceExW.is_some ())
	|| (! w32_unicode_filenames && pfn_GetDiskFreeSpaceExA.is_some ());

    /* On Windows, we may need to specify the root directory of the
       volume holding FILENAME.  */
    let mut rootname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];
    let mut rootname_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut rootname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut name: *mut c_char = SSDATA (encoded) as *mut c_char;
    let mut result: BOOL;

    /* find the root name of the volume if given */
    if c_isalpha (*name as c_int) && *name.offset (1) == b':' as c_char
      {
	rootname[0] = *name;
	rootname[1] = *name.offset (1);
	rootname[2] = b'\\' as c_char;
	rootname[3] = 0;
      }
    else if IS_DIRECTORY_SEP! (*name) && IS_DIRECTORY_SEP! (*name.offset (1))
      {
	let mut str_: *mut c_char = rootname.as_mut_ptr ();
	let mut slashes: c_int = 4;
	loop
	  {
	    if IS_DIRECTORY_SEP! (*name) && { slashes -= 1; slashes == 0 }
	      { break; }
	    *str_ = *name;
	    str_ = str_.add (1);
	    name = name.add (1);
	    if *name == 0
	      { break; }
	  }

	*str_ = b'\\' as c_char;
	str_ = str_.add (1);
	*str_ = 0;
      }

    if w32_unicode_filenames
      { filename_to_utf16 (rootname.as_ptr (), rootname_w.as_mut_ptr ()); }
    else
      { filename_to_ansi (rootname.as_ptr (), rootname_a.as_mut_ptr ()); }

    if have_pfn_GetDiskFreeSpaceEx
      {
	/* Unsigned large integers cannot be cast to double, so
	   use signed ones instead.  */
	let mut availbytes: LARGE_INTEGER = core::mem::zeroed ();
	let mut freebytes: LARGE_INTEGER = core::mem::zeroed ();
	let mut totalbytes: LARGE_INTEGER = core::mem::zeroed ();

	if w32_unicode_filenames
	  { result = (pfn_GetDiskFreeSpaceExW.unwrap ())
	      (rootname_w.as_ptr (),
	       &raw mut availbytes as *mut ULARGE_INTEGER,
	       &raw mut totalbytes as *mut ULARGE_INTEGER,
	       &raw mut freebytes as *mut ULARGE_INTEGER); }
	else
	  { result = (pfn_GetDiskFreeSpaceExA.unwrap ())
	      (rootname_a.as_ptr (),
	       &raw mut availbytes as *mut ULARGE_INTEGER,
	       &raw mut totalbytes as *mut ULARGE_INTEGER,
	       &raw mut freebytes as *mut ULARGE_INTEGER); }
	if result != 0
	  {
	    value = list3 (make_float (totalbytes.QuadPart as c_double),
			   make_float (freebytes.QuadPart as c_double),
			   make_float (availbytes.QuadPart as c_double));
	  }
      }
    else
      {
	let mut sectors_per_cluster: DWORD;
	let mut bytes_per_sector: DWORD;
	let mut free_clusters: DWORD;
	let mut total_clusters: DWORD;

	if w32_unicode_filenames
	  { result = GetDiskFreeSpaceW (rootname_w.as_ptr (),
					&raw mut sectors_per_cluster,
					&raw mut bytes_per_sector,
					&raw mut free_clusters,
					&raw mut total_clusters); }
	else
	  { result = GetDiskFreeSpaceA (rootname_a.as_ptr (),
					&raw mut sectors_per_cluster,
					&raw mut bytes_per_sector,
					&raw mut free_clusters,
					&raw mut total_clusters); }
	if result != 0
	  {
	    value = list3 (make_float ((total_clusters as c_double)
				       * (sectors_per_cluster as c_double)
				       * (bytes_per_sector as c_double)),
			   make_float ((free_clusters as c_double)
				       * (sectors_per_cluster as c_double)
				       * (bytes_per_sector as c_double)),
			   make_float ((free_clusters as c_double)
				       * (sectors_per_cluster as c_double)
				       * (bytes_per_sector as c_double)));
	  }
      }
  }

  return value;
});

/* C 9893's second `#ifdef WINDOWSNT` block: the printer records and entry points
   C 9901-9964 use.  */

/// C 9901-9902's two records (`<winspool.h>`), whose field lists are corroborated against the
/// Win32 metadata projection (`windows-sys` 0.52's `PRINTER_INFO_2A`/`2W`: the same twenty-one
/// fields in the same order); their layouts are **unmeasured** here (no SDK on this machine), and
/// `pDevMode`/`pSecurityDescriptor` are C's pointers to records this unit has no mirror of
/// (PORTING.md P-72 rule ③), so they stay `*mut c_void`.
/// C: typedef struct _PRINTER_INFO_2W { LPWSTR pServerName; LPWSTR pPrinterName;
/// C: LPWSTR pShareName; LPWSTR pPortName; LPWSTR pDriverName; LPWSTR pComment;
/// C: LPWSTR pLocation; LPDEVMODEW pDevMode; LPWSTR pSepFile; LPWSTR pPrintProcessor;
/// C: LPWSTR pDatatype; LPWSTR pParameters; PSECURITY_DESCRIPTOR pSecurityDescriptor;
/// C: DWORD Attributes; DWORD Priority; DWORD DefaultPriority; DWORD StartTime; DWORD UntilTime;
/// C: DWORD Status; DWORD cJobs; DWORD AveragePPM; } PRINTER_INFO_2W, *PPRINTER_INFO_2W;
/// C: typedef struct _PRINTER_INFO_2A { … same fields as LPSTR … } PRINTER_INFO_2A;
/// C: #define PRINTER_ATTRIBUTE_SHARED 0x00000008
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const PRINTER_ATTRIBUTE_SHARED: DWORD = 0x00000008;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_2W
{
  pub pServerName: LPWSTR,
  pub pPrinterName: LPWSTR,
  pub pShareName: LPWSTR,
  pub pPortName: LPWSTR,
  pub pDriverName: LPWSTR,
  pub pComment: LPWSTR,
  pub pLocation: LPWSTR,
  pub pDevMode: *mut c_void,
  pub pSepFile: LPWSTR,
  pub pPrintProcessor: LPWSTR,
  pub pDatatype: LPWSTR,
  pub pParameters: LPWSTR,
  pub pSecurityDescriptor: *mut c_void,
  pub Attributes: DWORD,
  pub Priority: DWORD,
  pub DefaultPriority: DWORD,
  pub StartTime: DWORD,
  pub UntilTime: DWORD,
  pub Status: DWORD,
  pub cJobs: DWORD,
  pub AveragePPM: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRINTER_INFO_2A
{
  pub pServerName: LPSTR,
  pub pPrinterName: LPSTR,
  pub pShareName: LPSTR,
  pub pPortName: LPSTR,
  pub pDriverName: LPSTR,
  pub pComment: LPSTR,
  pub pLocation: LPSTR,
  pub pDevMode: *mut c_void,
  pub pSepFile: LPSTR,
  pub pPrintProcessor: LPSTR,
  pub pDatatype: LPSTR,
  pub pParameters: LPSTR,
  pub pSecurityDescriptor: *mut c_void,
  pub Attributes: DWORD,
  pub Priority: DWORD,
  pub DefaultPriority: DWORD,
  pub StartTime: DWORD,
  pub UntilTime: DWORD,
  pub Status: DWORD,
  pub cJobs: DWORD,
  pub AveragePPM: DWORD,
}

/// C 9910's `GetProfileString` and C 9915's `OpenPrinter` are `<winbase.h>`/`<winspool.h>` macros
/// whose un-suffixed names this build resolves to the **A** entry points: `pname_buf` is a `char`
/// buffer at both sites, so a `W` call could not be written at all (the notes rows record the two
/// token gaps).  C 9919-9964's `GetPrinterW`/`GetPrinterA`, `ClosePrinter` and the two `strtok`/
/// `strncpy` (`<string.h>`) declarations follow.
/// C: DWORD WINAPI GetProfileStringA (LPCSTR lpAppName, LPCSTR lpKeyName, LPCSTR lpDefault,
/// C: LPSTR lpReturnedString, DWORD nSize);
/// C: BOOL WINAPI OpenPrinterA (LPSTR pPrinterName, LPHANDLE phPrinter, LPPRINTER_DEFAULTSA pDefault);
/// C: BOOL WINAPI GetPrinterW (HANDLE hPrinter, DWORD Level, LPBYTE pPrinter, DWORD cbBuf,
/// C: LPDWORD pcbNeeded);
/// C: BOOL WINAPI GetPrinterA (HANDLE hPrinter, DWORD Level, LPBYTE pPrinter, DWORD cbBuf,
/// C: LPDWORD pcbNeeded);
/// C: BOOL WINAPI ClosePrinter (HANDLE hPrinter);
/// C: char *strtok (char *s, const char *delim);
/// C: char *strncpy (char *dest, const char *src, size_t n);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" {
  pub fn GetProfileStringA (lpAppName: LPCSTR, lpKeyName: LPCSTR, lpDefault: LPCSTR,
			    lpReturnedString: LPSTR, nSize: DWORD) -> DWORD;
  pub fn OpenPrinterA (pPrinterName: LPSTR, phPrinter: LPHANDLE,
		       pDefault: *const c_void) -> BOOL;
  pub fn GetPrinterW (hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, cbBuf: DWORD,
		      pcbNeeded: LPDWORD) -> BOOL;
  pub fn GetPrinterA (hPrinter: HANDLE, Level: DWORD, pPrinter: LPBYTE, cbBuf: DWORD,
		      pcbNeeded: LPDWORD) -> BOOL;
  pub fn ClosePrinter (hPrinter: HANDLE) -> BOOL;
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn strtok (s: *mut c_char, delim: *const c_char) -> *mut c_char;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
}

/// C 9898: C's function-local `static char pname_buf[2 * MAX_UTF8_PATH + 3 - 1]`, which the file's
/// standing reading moves to file scope with its initialiser (the `force_quit_count`/`coords`
/// device).
/// C: static char pname_buf[2 * MAX_UTF8_PATH + 3 - 1];
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut pname_buf: [c_char; (2 * MAX_UTF8_PATH + 3 - 1) as usize]
  = [0; (2 * MAX_UTF8_PATH + 3 - 1) as usize];

/// C 9893-9999: `default-printer-name` - C 9898's `pname_buf` above, C 9899-9905's locals
/// (`ppi2w`/`ppi2a` are C's `NULL`-initialised pointers), C 9910-9911's `GetProfileString` probe
/// and C 9913's `strtok`, C 9915-9916's `OpenPrinter`, C 9918-9926's `GetPrinterW`/`A` size probe
/// (`w32_unicode_filenames && os_subtype != OS_SUBTYPE_9X`, C's own comment about unicows) and
/// `dwNeeded` test, C 9928-9974's two arm bodies (`xmalloc`, the second `GetPrinter`, C's
/// `Attributes & PRINTER_ATTRIBUTE_SHARED` + `pServerName` test with the shared/local split,
/// `xfree` on failure), C 9976-9995's two name shapes (`snprintf` for the remote printer,
/// `strncpy` + the `strtok` first-port cut for the local one, both ending in C's
/// `pname_buf[sizeof (pname_buf) - 1] = '\0'`) and C 9997's
/// `DECODE_FILE (build_unibyte_string (pname_buf))`.
/// C: DEFUN ("default-printer-name", Fdefault_printer_name, Sdefault_printer_name,
/// C:        0, 0, 0, doc: /* Return the name of Windows default printer device.  */)
/// C:   (void)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("default-printer-name", Fdefault_printer_name, Sdefault_printer_name,
	0, 0, 0,
	doc: "Return the name of Windows default printer device.",
	(),
	{
  let mut err: c_int;
  let mut hPrn: HANDLE;
  let mut ppi2w: *mut PRINTER_INFO_2W = core::ptr::null_mut ();
  let mut ppi2a: *mut PRINTER_INFO_2A = core::ptr::null_mut ();
  let mut dwNeeded: DWORD = 0;
  let mut dwReturned: DWORD = 0;
  let mut server_name: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];
  let mut share_name: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];
  let mut port_name: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];

  /* Retrieve the default string from Win.ini (the registry).
   * String will be in form "printername,drivername,portname".
   * This is the most portable way to get the default printer. */
  if GetProfileStringA (c"windows".as_ptr (), c"device".as_ptr (), c",,".as_ptr (),
			pname_buf.as_mut_ptr (),
			core::mem::size_of_val (&pname_buf) as DWORD) == 0
    { return Qnil (); }
  /* printername precedes first "," character */
  strtok (pname_buf.as_mut_ptr (), c",".as_ptr ());
  /* We want to know more than the printer name */
  if OpenPrinterA (pname_buf.as_mut_ptr (), &raw mut hPrn, core::ptr::null ()) == 0
    { return Qnil (); }
  /* GetPrinterW is not supported by unicows.dll.  */
  if w32_unicode_filenames && os_subtype != OS_SUBTYPE_9X
    { GetPrinterW (hPrn, 2, core::ptr::null_mut (), 0, &raw mut dwNeeded); }
  else
    { GetPrinterA (hPrn, 2, core::ptr::null_mut (), 0, &raw mut dwNeeded); }
  if dwNeeded == 0
    {
      ClosePrinter (hPrn);
      return Qnil ();
    }
  /* Call GetPrinter again with big enough memory block.  */
  if w32_unicode_filenames && os_subtype != OS_SUBTYPE_9X
    {
      /* Allocate memory for the PRINTER_INFO_2 struct.  */
      ppi2w = xmalloc (dwNeeded as usize) as *mut PRINTER_INFO_2W;
      err = GetPrinterW (hPrn, 2, ppi2w as LPBYTE, dwNeeded, &raw mut dwReturned) as c_int;
      ClosePrinter (hPrn);
      if err == 0
	{
	  xfree (ppi2w as *mut c_void);
	  return Qnil ();
	}

      if (*ppi2w).Attributes & PRINTER_ATTRIBUTE_SHARED != 0
	  && ! (*ppi2w).pServerName.is_null ()
	{
	  filename_from_utf16 ((*ppi2w).pServerName, server_name.as_mut_ptr ());
	  filename_from_utf16 ((*ppi2w).pShareName, share_name.as_mut_ptr ());
	}
      else
	{
	  server_name[0] = 0;
	  filename_from_utf16 ((*ppi2w).pPortName, port_name.as_mut_ptr ());
	}
    }
  else
    {
      ppi2a = xmalloc (dwNeeded as usize) as *mut PRINTER_INFO_2A;
      err = GetPrinterA (hPrn, 2, ppi2a as LPBYTE, dwNeeded, &raw mut dwReturned) as c_int;
      ClosePrinter (hPrn);
      if err == 0
	{
	  xfree (ppi2a as *mut c_void);
	  return Qnil ();
	}

      if (*ppi2a).Attributes & PRINTER_ATTRIBUTE_SHARED != 0
	  && ! (*ppi2a).pServerName.is_null ()
	{
	  filename_from_ansi ((*ppi2a).pServerName, server_name.as_mut_ptr ());
	  filename_from_ansi ((*ppi2a).pShareName, share_name.as_mut_ptr ());
	}
      else
	{
	  server_name[0] = 0;
	  filename_from_ansi ((*ppi2a).pPortName, port_name.as_mut_ptr ());
	}
    }

  if server_name[0] != 0
    {
      /* a remote printer */
      if server_name[0] == b'\\' as c_char
	{ snprintf (pname_buf.as_mut_ptr (), core::mem::size_of_val (&pname_buf),
		    c"%s\\%s".as_ptr (), server_name.as_ptr (), share_name.as_ptr ()); }
      else
	{ snprintf (pname_buf.as_mut_ptr (), core::mem::size_of_val (&pname_buf),
		    c"\\\\%s\\%s".as_ptr (), server_name.as_ptr (),
		    share_name.as_ptr ()); }
      pname_buf[core::mem::size_of_val (&pname_buf) - 1] = 0;
    }
  else
    {
      /* a local printer */
      strncpy (pname_buf.as_mut_ptr (), port_name.as_ptr (),
	       core::mem::size_of_val (&pname_buf));
      pname_buf[core::mem::size_of_val (&pname_buf) - 1] = 0;
      /* `pPortName' can include several ports, delimited by ','.
       * we only use the first one. */
      strtok (pname_buf.as_mut_ptr (), c",".as_ptr ());
    }

  return DECODE_FILE (build_unibyte_string (pname_buf.as_ptr ()));
});

/// C 10002-10110's new `<windows.h>`/`<stdio.h>` surface: `GetLastError`, `FormatMessageA` with the
/// two `FORMAT_MESSAGE_*` bits, `GetVersion`, `GetSystemInfo`, `GetVersionExA`, `OutputDebugStringA`
/// and the `_snprintf`/`vsprintf` pair (the latter two are MSVCRT's, spelled here as
/// `src/nt/cmdproxy.rs` 261-265 spells the first).  `SM_CMOUSEBUTTONS` is C 10080's metric.
/// C: DWORD WINAPI GetLastError (void);
/// C: DWORD WINAPI FormatMessageA (DWORD dwFlags, LPCVOID lpSource, DWORD dwMessageId,
/// C: DWORD dwLanguageId, LPSTR lpBuffer, DWORD nSize, va_list *Arguments);
/// C: DWORD WINAPI GetVersion (void);
/// C: void WINAPI GetSystemInfo (LPSYSTEM_INFO lpSystemInfo);
/// C: BOOL WINAPI GetVersionExA (LPOSVERSIONINFOA lpVersionInformation);
/// C: void WINAPI OutputDebugStringA (LPCSTR lpOutputString);
/// C: int _snprintf (char *buffer, size_t count, const char *format, …);
/// C: int vsprintf (char *s, const char *format, va_list arg);
/// C: #define SM_CMOUSEBUTTONS 43
/// C: #define FORMAT_MESSAGE_FROM_SYSTEM 0x00001000
/// C: #define FORMAT_MESSAGE_IGNORE_INSERTS 0x00000200
#[cfg (feature = "have_ntgui")]
pub const SM_CMOUSEBUTTONS: c_int = 43;
#[cfg (feature = "have_ntgui")]
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
#[cfg (feature = "have_ntgui")]
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn GetLastError () -> DWORD;
  pub fn FormatMessageA (dwFlags: DWORD, lpSource: *const c_void, dwMessageId: DWORD,
			 dwLanguageId: DWORD, lpBuffer: LPSTR, nSize: DWORD,
			 arguments: *const *const c_char) -> DWORD;
  pub fn GetVersion () -> DWORD;
  pub fn GetSystemInfo (lpSystemInfo: *mut SYSTEM_INFO);
  pub fn GetVersionExA (lpVersionInformation: *mut OSVERSIONINFO) -> BOOL;
  pub fn OutputDebugStringA (lpOutputString: LPCSTR);
}
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn _snprintf (buffer: *mut c_char, count: usize, format: *const c_char, ...) -> c_int;
  pub fn vsprintf (s: *mut c_char, format: *const c_char, arg: ...) -> c_int;
}

/// C 10002-10027: `w32_strerror` - C 10006's function-local `static char buf[500]` (file scope
/// below; C's own name `buf` is taken by C 3630-3632's `static char buf[64]` above, which the
/// `emacsdebug` gate compiles, so this one keeps the trailing-underscore spelling PORTING.md 0.2
/// folds), C 10009-10010's `GetLastError` default, C 10012-10017's `FormatMessage` call with C's
/// two flag bits (the macro's arm: the buffer is `char`, so the A entry point - the `GetProfileString`
/// reading of slice 23), C 10019-10021's trailing-CRLF trim and C 10023-10024's `sprintf` fallback.
/// C: char *
/// C: w32_strerror (int error_no)
#[cfg (feature = "have_ntgui")]
static mut buf_: [c_char; 500] = [0; 500];
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_strerror (error_no: c_int) -> *mut c_char
{
  let mut error_no = error_no;
  let mut ret: DWORD;

  if error_no == 0
    { error_no = GetLastError () as c_int; }

  ret = FormatMessageA (FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
			core::ptr::null (),
			error_no as DWORD,
			0, /* choose most suitable language */
			buf_.as_mut_ptr (), core::mem::size_of_val (&buf_) as DWORD,
			core::ptr::null ());

  while ret > 0 && (buf_[ret as usize - 1] == b'\n' as c_char
		    || buf_[ret as usize - 1] == b'\r' as c_char)
    { ret -= 1; }
  buf_[ret as usize] = 0;
  if ret == 0
    { sprintf (buf_.as_mut_ptr (), c"w32 error %d".as_ptr (), error_no); }

  return buf_.as_mut_ptr ();
}

/// C 10029-10038: C 10030-10031's comment, C 10032's forward declaration (dropped, PORTING.md
/// 0.1-12) and the definition C 10034-10038 - a non-`static` C function, so `#[unsafe(no_mangle)]`.
/// C: DWORD w32_last_error (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_last_error () -> DWORD
{
  return GetLastError ();
}

/// C 10040-10081: `cache_system_info` - C 10044-10053's function-local union (the port declares the
/// same two records in the body, the `src/thread.rs` 788-795 reading; C's anonymous union has no
/// tag, so the port names it after C's own variable `version`), C 10056's `hinst` cache, C 10059-10061's
/// `GetVersion` unpacking (C's `char major`/`char minor`/`short platform` are `c_char`/`c_short`), C 10063-10066's
/// platform-bit test, C 10069-10070's `GetSystemInfo` and `syspage_mask`, C 10073-10076's
/// `OSVERSIONINFO` size + `GetVersionEx` and `dwBuildNumber`, C 10077-10078's 9X truncation and
/// C 10080's mouse-button metric.
/// C: void
/// C: cache_system_info (void)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cache_system_info ()
{
  /* C 10044-10053's in-body `union { struct info { … } info; DWORD data; } version;`.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  struct info
  {
    major: c_char,
    minor: c_char,
    platform: c_short,
  }
  #[repr(C)]
  #[derive(Clone, Copy)]
  union version
  {
    info: info,
    data: DWORD,
  }
  let mut version: version = core::mem::zeroed ();

  /* Cache the module handle of Emacs itself.  */
  hinst = GetModuleHandle (core::ptr::null ());

  /* Cache the version of the operating system.  */
  version.data = GetVersion ();
  w32_major_version = version.info.major as c_int;
  w32_minor_version = version.info.minor as c_int;

  if version.info.platform as c_int & 0x8000 != 0
    { os_subtype = OS_SUBTYPE_9X; }
  else
    { os_subtype = OS_SUBTYPE_NT; }

  /* Cache page size, allocation unit, processor type, etc.  */
  GetSystemInfo (&raw mut sysinfo_cache);
  syspage_mask = sysinfo_cache.dwPageSize as DWORD_PTR - 1;

  /* Cache os info.  */
  osinfo_cache.dwOSVersionInfoSize = core::mem::size_of::<OSVERSIONINFO> () as DWORD;
  GetVersionExA (&raw mut osinfo_cache);

  w32_build_number = osinfo_cache.dwBuildNumber as c_int;
  if os_subtype == OS_SUBTYPE_9X
    { w32_build_number &= 0xffff; }

  w32_num_mouse_buttons = GetSystemMetrics (SM_CMOUSEBUTTONS) as EMACS_INT;
}

/// C 10083-10093: `w32_version_string` under C's `#ifdef WINDOWSNT` - C 10088's function-local
/// `static char version_string[…]` (file scope below with its initialiser, the `coords` device; the
/// notes row records the three numbers its bound takes out of the body) and C 10089-10090's
/// `_snprintf` of the three cached numbers.
/// C: char *
/// C: w32_version_string (void)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut version_string: [c_char; 3 + 1 + 3 + 1 + 10 + 1] = [0; 3 + 1 + 3 + 1 + 10 + 1];
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_version_string () -> *mut c_char
{
  /* NNN.NNN.NNNNNNNNNN */
  _snprintf (version_string.as_mut_ptr (), core::mem::size_of_val (&version_string),
	     c"%d.%d.%d".as_ptr (), w32_major_version, w32_minor_version, w32_build_number);
  return version_string.as_mut_ptr ();
}

/// C 10095-10110: `_DebPrint` under C's `#ifdef EMACSDEBUG` - C 10099's `char buf[1024]`, C 10100-10104's
/// `va_list`/`va_start`/`vsprintf`/`va_end` (the `...` parameter *is* the list, PORTING.md P-04; the
/// notes row carries the three tokens that have no counterpart), C 10105-10107's `CYGWIN` arm and
/// C 10108's `OutputDebugString` (the A entry point: the buffer is `char`).
/// C: void
/// C: _DebPrint (const char *fmt, ...)
#[cfg (all (feature = "have_ntgui", feature = "emacsdebug"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _DebPrint (fmt: *const c_char, mut args: ...)
{
  let mut buf: [c_char; 1024] = [0; 1024];

  vsprintf (buf.as_mut_ptr (), fmt, args);
  #[cfg (feature = "cygwin")]
  { fprintf (stderr, c"%s".as_ptr (), buf.as_mut_ptr ()); }
  OutputDebugStringA (buf.as_mut_ptr ());
}

/// C 10140-10303's new `<winuser.h>`/`<wincon.h>`/`<winnls.h>` surface: the five `VK_*` codes
/// C 10178-10184 compares (`VK_BACK`/`VK_TAB`/`VK_ESCAPE`/`VK_HELP`/`VK_F24` - the others are
/// already spelled above or come from `include/w32term.rs`), the two console/codepage entry points
/// and the three sound ones, plus `LPWORD`.
/// C: #define VK_BACK 0x08 / #define VK_TAB 0x09 / #define VK_ESCAPE 0x1B / #define VK_HELP 0x2F /
/// C: #define VK_F24 0x87
/// C: typedef WORD *LPWORD;
/// C: int WINAPI ToUnicode (UINT wVirtKey, UINT wScanCode, const BYTE *lpKeyState, LPWSTR pwszBuff,
/// C: int cchBuff, UINT wFlags);
/// C: int WINAPI ToAscii (UINT uVirtKey, UINT uScanCode, const BYTE *lpKeyState, LPWORD lpChar,
/// C: UINT uFlags);
/// C: int WINAPI WideCharToMultiByte (UINT CodePage, DWORD dwFlags, LPCWCH lpWideCharStr,
/// C: int cchWideChar, LPSTR lpMultiByteStr, int cbMultiByte, LPCCH lpDefaultChar,
/// C: LPBOOL lpUsedDefaultChar);
/// C: UINT WINAPI GetConsoleCP (void);
/// C: BOOL WINAPI Beep (DWORD dwFreq, DWORD dwDuration);
/// C: BOOL WINAPI MessageBeep (UINT uType);
#[cfg (feature = "have_ntgui")]
pub const VK_BACK: c_int = 0x08;
#[cfg (feature = "have_ntgui")]
pub const VK_TAB: c_int = 0x09;
#[cfg (feature = "have_ntgui")]
pub const VK_ESCAPE: c_int = 0x1B;
#[cfg (feature = "have_ntgui")]
pub const VK_HELP: c_int = 0x2F;
#[cfg (feature = "have_ntgui")]
pub const VK_F24: c_int = 0x87;
#[cfg (feature = "have_ntgui")]
pub type LPWORD = *mut WORD;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn ToUnicode (wVirtKey: UINT, wScanCode: UINT, lpKeyState: *const BYTE, pwszBuff: LPWSTR,
		    cchBuff: c_int, wFlags: UINT) -> c_int;
  pub fn ToAscii (uVirtKey: UINT, uScanCode: UINT, lpKeyState: *const BYTE, lpChar: LPWORD,
		  uFlags: UINT) -> c_int;
  pub fn WideCharToMultiByte (CodePage: UINT, dwFlags: DWORD, lpWideCharStr: *const WCHAR,
			      cchWideChar: c_int, lpMultiByteStr: LPSTR, cbMultiByte: c_int,
			      lpDefaultChar: *const c_char, lpUsedDefaultChar: *mut BOOL) -> c_int;
  pub fn GetConsoleCP () -> UINT;
  pub fn Beep (dwFreq: DWORD, dwDuration: DWORD) -> BOOL;
  pub fn MessageBeep (uType: UINT) -> BOOL;
}

/// C 10140-10202: `w32_kbd_mods_to_emacs` (C 10140-10141's own two comment lines first) - C 10145's
/// `retval`, C 10149-10152's AltGr strip (`!NILP` + the two-state test and C's `&= ~`), C 10154-10155's
/// alt-or-meta choice by `Vw32_alt_is_meta`, C 10157-10163's control/meta pair, C 10165-10172's four
/// `w32_key_to_modifier` calls, C 10176-10199's CapsLock ladder (C's own "keys that should _not_ be
/// affected" list with `VK_NUMPAD_CLEAR`/`VK_NUMPAD_DELETE` from `include/w32term.rs`) and C 10201's
/// return.  `mods` is C's reassigned parameter and the five `<wincon.h>` bits are `c_int` in this
/// file while `mods` is `DWORD`, so each use carries C's promotion as a cast.
/// C: int
/// C: w32_kbd_mods_to_emacs (DWORD mods, WORD key)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_kbd_mods_to_emacs (mods: DWORD, key: WORD) -> c_int
{
  let mut mods = mods;
  let mut retval: c_int = 0;

  /* If we recognize right-alt and left-ctrl as AltGr, and it has been
     pressed, first remove those modifiers.  */
  if ! NILP (Vw32_recognize_altgr)
      && (mods & (RIGHT_ALT_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD))
      == (RIGHT_ALT_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD)
    { mods &= ! (RIGHT_ALT_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD); }

  if mods & (RIGHT_ALT_PRESSED as DWORD | LEFT_ALT_PRESSED as DWORD) != 0
    { retval = if NILP (Vw32_alt_is_meta) { alt_modifier } else { meta_modifier }; }

  if mods & (RIGHT_CTRL_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD) != 0
    {
      retval |= ctrl_modifier;
      if (mods & (RIGHT_CTRL_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD))
	  == (RIGHT_CTRL_PRESSED as DWORD | LEFT_CTRL_PRESSED as DWORD)
	{ retval |= meta_modifier; }
    }

  if mods & LEFT_WIN_PRESSED as DWORD != 0
    { retval |= w32_key_to_modifier (VK_LWIN) as c_int; }
  if mods & RIGHT_WIN_PRESSED as DWORD != 0
    { retval |= w32_key_to_modifier (VK_RWIN) as c_int; }
  if mods & APPS_PRESSED as DWORD != 0
    { retval |= w32_key_to_modifier (VK_APPS) as c_int; }
  if mods & SCROLLLOCK_ON as DWORD != 0
    { retval |= w32_key_to_modifier (VK_SCROLL) as c_int; }

  /* Just in case someone wanted the original behavior, make it
     optional by setting w32-capslock-is-shiftlock to t.  */
  if NILP (Vw32_capslock_is_shiftlock)
      /* Keys that should _not_ be affected by CapsLock.  */
      && (    (key as c_int == VK_BACK)
	   || (key as c_int == VK_TAB)
	   || (key as c_int == VK_CLEAR)
	   || (key as c_int == VK_RETURN)
	   || (key as c_int == VK_ESCAPE)
	   || ((key as c_int >= VK_SPACE) && (key as c_int <= VK_HELP))
	   || ((key as c_int >= VK_NUMPAD0) && (key as c_int <= VK_F24))
	   || ((key as c_int >= VK_NUMPAD_CLEAR) && (key as c_int <= VK_NUMPAD_DELETE))
	 )
    {
      /* Only consider shift state.  */
      if (mods & SHIFT_PRESSED as DWORD) != 0
	{ retval |= shift_modifier; }
    }
  else
    {
      /* Ignore CapsLock state if not enabled.  */
      if NILP (Vw32_enable_caps_lock)
	{ mods &= ! (CAPSLOCK_ON as DWORD); }
      if (mods & (SHIFT_PRESSED as DWORD | CAPSLOCK_ON as DWORD)) != 0
	{ retval |= shift_modifier; }
    }

  return retval;
}

/// C 10204-10214's comment and the two function-local statics C 10213-10214 declares (`ansi_code`
/// and `isdead`, which Rust has no function-scope `static` for - the `force_quit_count` reading).
/// C: static BYTE ansi_code[4];
/// C: static int isdead = 0;
#[cfg (feature = "have_ntgui")]
static mut ansi_code: [BYTE; 4] = [0; 4];
#[cfg (feature = "have_ntgui")]
static mut isdead: c_int = 0;

/// C 10207-10287: `w32_kbd_patch_key` - C 10210-10212's three locals, C 10216-10223's dead-key
/// short circuits, C 10225-10240's `keystate` fill (the `memset`, the virtual key and the four
/// AltGr states), C 10242-10252's `#if 0` block carried as C's own text, C 10256-10276's NT arm
/// (`ToUnicode`, `GetConsoleCP` when `cpId == -1`, the `uChar.UnicodeChar` write and
/// `WideCharToMultiByte`), C 10277-10281's `ToAscii` arm, C 10283-10286's exit.
/// C: int
/// C: w32_kbd_patch_key (KEY_EVENT_RECORD *event, int cpId)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_kbd_patch_key (event: *mut KEY_EVENT_RECORD, cpId: c_int) -> c_int
{
  let mut cpId = cpId;
  let key_code: c_uint = (*event).wVirtualKeyCode as c_uint;
  let mods: c_uint = (*event).dwControlKeyState as c_uint;
  let mut keystate: [BYTE; 256] = [0; 256];

  if isdead == 2
    {
      (*event).uChar.AsciiChar = ansi_code[2] as CHAR;
      isdead = 0;
      return 1;
    }
  if (*event).uChar.AsciiChar != 0
    { return 1; }

  memset (keystate.as_mut_ptr () as *mut c_void, 0, core::mem::size_of_val (&keystate));
  keystate[key_code as usize] = 0x80;
  if mods & SHIFT_PRESSED as c_uint != 0
    { keystate[VK_SHIFT as usize] = 0x80; }
  if mods & CAPSLOCK_ON as c_uint != 0
    { keystate[VK_CAPITAL as usize] = 1; }
  /* If we recognize right-alt and left-ctrl as AltGr, set the key
     states accordingly before invoking ToAscii.  */
  if ! NILP (Vw32_recognize_altgr)
      && (mods & LEFT_CTRL_PRESSED as c_uint) != 0 && (mods & RIGHT_ALT_PRESSED as c_uint) != 0
    {
      keystate[VK_CONTROL as usize] = 0x80;
      keystate[VK_LCONTROL as usize] = 0x80;
      keystate[VK_MENU as usize] = 0x80;
      keystate[VK_RMENU as usize] = 0x80;
    }

  /* C 10242-10252's `#if 0` arm (`is_dead_key`'s note about the NT4 console bug) is carried as
     C's own text, the file's reading for a region no configuration compiles.  */

  /*
  if (is_dead_key (event->wVirtualKeyCode))
    return 0;
  */

  /* On NT, call ToUnicode instead and then convert to the current
     console input codepage.  */
  if os_subtype == OS_SUBTYPE_NT
    {
      let mut buf: [WCHAR; 128] = [0; 128];

      isdead = ToUnicode ((*event).wVirtualKeyCode as UINT, (*event).wVirtualScanCode as UINT,
			  keystate.as_ptr (), buf.as_mut_ptr (), 128, 0);
      if isdead > 0
	{
	  /* When we are called from the GUI message processing code,
	     we are passed the current keyboard codepage, a positive
	     number, to use below.  */
	  if cpId == -1
	    { cpId = GetConsoleCP () as c_int; }

	  (*event).uChar.UnicodeChar = buf[isdead as usize - 1];
	  isdead = WideCharToMultiByte (cpId as UINT, 0, buf.as_ptr (), isdead,
					ansi_code.as_mut_ptr () as LPSTR, 4,
					core::ptr::null (), core::ptr::null_mut ());
	}
      else
	{ isdead = 0; }
    }
  else
    {
      isdead = ToAscii ((*event).wVirtualKeyCode as UINT, (*event).wVirtualScanCode as UINT,
			keystate.as_ptr (), ansi_code.as_mut_ptr () as LPWORD, 0);
    }

  if isdead == 0
    { return 0; }
  (*event).uChar.AsciiChar = ansi_code[0] as CHAR;
  return isdead;
}


/// C 10290-10303: `w32_sys_ring_bell` - C 10293's `sound_type` test against C 339's
/// `MB_EMACS_SILENT`, C 10295's `Beep (666, 100)` and C 10302's `MessageBeep (sound_type)`.  C never
/// reads `f` and the port keeps the parameter without an `_` (the `window_prompting` reading).
/// C: void
/// C: w32_sys_ring_bell (struct frame *f)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_sys_ring_bell (f: *mut frame)
{
  if sound_type == 0xFFFFFFFF
    {
      Beep (666, 100);
    }
  else if sound_type == MB_EMACS_SILENT
    {
      /* Do nothing.  */
    }
  else
    { MessageBeep (sound_type); }
}

/// C 10305-10312: `w32--menu-bar-in-use` - C 10311's one ternary over C 303's `menubar_in_use`
/// (this file defines it).
/// C: DEFUN ("w32--menu-bar-in-use", Fw32__menu_bar_in_use, Sw32__menu_bar_in_use,
/// C:        0, 0, 0,
/// C:        doc: /* Return non-nil when a menu-bar menu is being used. … */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32--menu-bar-in-use", Fw32__menu_bar_in_use, Sw32__menu_bar_in_use,
	0, 0, 0,
	doc: "Return non-nil when a menu-bar menu is being used.\nInternal use only.",
	(),
	{
  return if menubar_in_use != 0 { Qt () } else { Qnil () };
});

/* C 10314's `#if defined WINDOWSNT && !defined HAVE_DBUS` opens the tray-notification block; the
   three features on every item below are the file gate plus C's own two conditions
   (`feature-coverage.tsv` 138 names `have_dbus`).  */

/// C 10316-10320: the `Shell_NotifyIconW` pointer the block loads by hand (C 10553-10555) and the
/// banner comment before it.  C's type is the SDK's `PNOTIFYICONDATAW`; this unit has only its own
/// `MY_NOTIFYICONDATAW` (below), so the second parameter is untyped (PORTING.md P-72 rule ③) and
/// the one call site casts.
/// C: static BOOL (WINAPI *pfnShell_NotifyIconW) (DWORD, PNOTIFYICONDATAW);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
static mut pfnShell_NotifyIconW
  : Option<unsafe extern "system" fn (arg0: DWORD, arg1: *mut c_void) -> BOOL> = None;

/// C 10321-10341: `MY_NOTIFYICONDATAW`, C's private copy of `<shellapi.h>`'s `NOTIFYICONDATAW`
/// ("A private struct declaration to avoid compile-time limits").  The field list and their order
/// are the SDK's, corroborated against the Win32 metadata projection (`windows-sys` 0.52's
/// `NOTIFYICONDATAW`: the same members); the layout is **unmeasured** here (no `<windows.h>` on this
/// machine).  C's anonymous union member keeps the SDK's own compatibility name `DUMMYUNIONNAME`
/// (the `SHELLEXECUTEINFOW` reading below).
/// C: typedef struct MY_NOTIFYICONDATAW { DWORD cbSize; HWND hWnd; UINT uID; UINT uFlags;
/// C: UINT uCallbackMessage; HICON hIcon; WCHAR szTip[128]; DWORD dwState; DWORD dwStateMask;
/// C: WCHAR szInfo[256]; _ANONYMOUS_UNION union { UINT uTimeout; UINT uVersion; } DUMMYUNIONNAME;
/// C: WCHAR szInfoTitle[64]; DWORD dwInfoFlags; GUID guidItem; HICON hBalloonIcon; }
/// C: MY_NOTIFYICONDATAW;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union MY_NOTIFYICONDATAW_union
{
  pub uTimeout: UINT,
  pub uVersion: UINT,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MY_NOTIFYICONDATAW
{
  pub cbSize: DWORD,
  pub hWnd: HWND,
  pub uID: UINT,
  pub uFlags: UINT,
  pub uCallbackMessage: UINT,
  pub hIcon: HICON,
  pub szTip: [WCHAR; 128],
  pub dwState: DWORD,
  pub dwStateMask: DWORD,
  pub szInfo: [WCHAR; 256],
  pub r#DUMMYUNIONNAME: MY_NOTIFYICONDATAW_union,
  pub szInfoTitle: [WCHAR; 64],
  pub dwInfoFlags: DWORD,
  pub guidItem: GUID,
  pub hBalloonIcon: HICON,
}

/// C 10343-10345: the three sizes.  C's `offsetof (MY_NOTIFYICONDATAW, szTip[64])` names an
/// *element* of the array, so the port adds 64 `WCHAR`s to that field's offset (`offsetof` is
/// `core::mem::offset_of!`, the `src/alloc.rs` 391 reading).
/// C: #define MYNOTIFYICONDATAW_V1_SIZE offsetof (MY_NOTIFYICONDATAW, szTip[64])
/// C: #define MYNOTIFYICONDATAW_V2_SIZE offsetof (MY_NOTIFYICONDATAW, guidItem)
/// C: #define MYNOTIFYICONDATAW_V3_SIZE offsetof (MY_NOTIFYICONDATAW, hBalloonIcon)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
macro_rules! MYNOTIFYICONDATAW_V1_SIZE {
  () => { core::mem::offset_of! (MY_NOTIFYICONDATAW, szTip)
	  + 64 * core::mem::size_of::<WCHAR> () };
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
macro_rules! MYNOTIFYICONDATAW_V2_SIZE {
  () => { core::mem::offset_of! (MY_NOTIFYICONDATAW, guidItem) };
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
macro_rules! MYNOTIFYICONDATAW_V3_SIZE {
  () => { core::mem::offset_of! (MY_NOTIFYICONDATAW, hBalloonIcon) };
}

/// C 10346-10360: the five `#ifndef` guards (the file's standing reading: C's value, no `cfg`) and
/// C 10363-10364's two `#define`s.
/// C: #ifndef NIF_INFO / # define NIF_INFO 0x00000010 / #endif
/// C: #ifndef NIIF_NONE / # define NIIF_NONE 0x00000000 / #endif
/// C: #ifndef NIIF_INFO / # define NIIF_INFO 0x00000001 / #endif
/// C: #ifndef NIIF_WARNING / # define NIIF_WARNING 0x00000002 / #endif
/// C: #ifndef NIIF_ERROR / # define NIIF_ERROR 0x00000003 / #endif
/// C: #define EMACS_TRAY_NOTIFICATION_ID_INIT  42	/* arbitrary */
/// C: #define EMACS_NOTIFICATION_MSG           (WM_APP + 1)
/// C: #define WM_APP 0x8000
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const WM_APP: c_int = 0x8000;
/// C 10471's three other `<shellapi.h>` flags (C's `#ifndef NIF_INFO` guard above declares only
/// `NIF_INFO`; these three come from the header).
/// C: #define NIF_MESSAGE 0x00000001 / #define NIF_ICON 0x00000002 / #define NIF_TIP 0x00000004
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIF_MESSAGE: DWORD = 0x00000001;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIF_ICON: DWORD = 0x00000002;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIF_TIP: DWORD = 0x00000004;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIF_INFO: DWORD = 0x00000010;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIIF_NONE: DWORD = 0x00000000;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIIF_INFO: DWORD = 0x00000001;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIIF_WARNING: DWORD = 0x00000002;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIIF_ERROR: DWORD = 0x00000003;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const EMACS_TRAY_NOTIFICATION_ID_INIT: c_int = 42;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const EMACS_NOTIFICATION_MSG: c_int = WM_APP + 1;

/// C 10366-10371: `enum NI_Severity`, the four members as C's names with C's implicit values (the
/// `OS_SUBTYPE_9X`/`OS_SUBTYPE_NT` reading of a small enum); the *type* `enum NI_Severity` is the
/// `c_int` that `add_tray_notification`'s parameter takes below.
/// C: enum NI_Severity { Ni_None, Ni_Info, Ni_Warn, Ni_Err };
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const Ni_None: c_int = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const Ni_Info: c_int = 1;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const Ni_Warn: c_int = 2;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const Ni_Err: c_int = 3;

/// C 10373-10402: `get_dll_version` - C 10378's `version`, C 10379's `LoadLibrary` (C's plain macro
/// whose argument is a `char` string: the A entry point, the notes row records the token), C 10383-10384's
/// `DllGetVersion` lookup (`get_proc_addr` + the `transmute` the loader sites use), C 10388-10396's
/// `DLLVERSIONINFO` (`memset`, `cbSize`, the call, `SUCCEEDED`, `MAKEDLLVERULL`) and C 10398's
/// `FreeLibrary`.  The `<shlwapi.h>` record and macro are new here.
/// C: typedef HRESULT (STDAPICALLTYPE *DLLGETVERSIONPROC) (DLLVERSIONINFO *);
/// C: typedef struct _DLLVERSIONINFO { DWORD cbSize; DWORD dwMajorVersion; DWORD dwMinorVersion;
/// C: DWORD dwBuildNumber; } DLLVERSIONINFO;
/// C: #define MAKEDLLVERULL(major, minor, build, qfe) \
/// C:   (((ULONGLONG)(major) << 48) | ((ULONGLONG)(minor) << 32) | ((ULONGLONG)(build) << 16) | (ULONGLONG)(qfe))
/// C: HMODULE WINAPI LoadLibraryA (LPCSTR lpLibFileName);
/// C: BOOL WINAPI FreeLibrary (HMODULE hLibModule);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub type ULONGLONG = u64;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub type DLLGETVERSIONPROC
  = Option<unsafe extern "system" fn (arg0: *mut DLLVERSIONINFO) -> HRESULT>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DLLVERSIONINFO
{
  pub cbSize: DWORD,
  pub dwMajorVersion: DWORD,
  pub dwMinorVersion: DWORD,
  pub dwBuildNumber: DWORD,
  pub dwPlatformID: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
macro_rules! MAKEDLLVERULL {
  ($major:expr, $minor:expr, $build:expr, $qfe:expr) => {
    ((($major) as ULONGLONG) << 48 | (($minor) as ULONGLONG) << 32
     | (($build) as ULONGLONG) << 16 | ($qfe) as ULONGLONG)
  };
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe extern "system" {
  pub fn LoadLibraryA (lpLibFileName: LPCSTR) -> HMODULE;
  pub fn FreeLibrary (hLibModule: HMODULE) -> BOOL;
}

#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe fn get_dll_version (dll_name: *const c_char) -> ULONGLONG
{
  let mut version: ULONGLONG = 0;
  let hdll: HINSTANCE = LoadLibraryA (dll_name);

  if ! hdll.is_null ()
    {
      let pDllGetVersion: DLLGETVERSIONPROC
	= core::mem::transmute::<VOIDFNPTR, DLLGETVERSIONPROC> (
	    get_proc_addr (hdll, c"DllGetVersion".as_ptr ()));

      if pDllGetVersion.is_some ()
	{
	  let mut dvi: DLLVERSIONINFO = core::mem::zeroed ();
	  let result: HRESULT;

	  memset (&raw mut dvi as *mut c_void, 0, core::mem::size_of_val (&dvi));
	  dvi.cbSize = core::mem::size_of_val (&dvi) as DWORD;
	  result = (pDllGetVersion.unwrap ()) (&raw mut dvi);
	  if SUCCEEDED! (result)
	    {
	      version = MAKEDLLVERULL! (dvi.dwMajorVersion, dvi.dwMinorVersion, 0, 0);
	    }
	}
      FreeLibrary (hdll);
    }

  return version;
}

/// C 10404-10427: `utf8_mbslen_lim` - C 10411-10412's three locals, C 10414-10421's `CHAR_BYTES`
/// walk (`include/character.rs`'s function) and C 10423-10424's terminating-byte count.
/// C: static int
/// C: utf8_mbslen_lim (const char *str, int lim)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe fn utf8_mbslen_lim (str_: *const c_char, lim: c_int) -> c_int
{
  let mut p: *const c_char = str_;
  let mut mblen: c_int = 0;
  let mut nchars: c_int = 0;

  while *p != 0 && nchars < lim
    {
      let nbytes: c_int = CHAR_BYTES (*p as c_int);

      mblen += nbytes;
      nchars += 1;
      p = p.add (nbytes as usize);
    }

  if *p == 0 && nchars < lim
    { mblen += 1; }

  return mblen;
}

/// C 10456-10599's new `<shellapi.h>`/`<winuser.h>` surface: the two `Shell_NotifyIcon` messages,
/// the two image-loading flags, the `LoadImageW`/`LoadImageA` pair C's explicit calls name (this unit
/// declares `LoadImage` - the macro - above), `DestroyIcon` and the CRT's `wcscpy`.
/// C: #define NIM_ADD 0x00000000 / #define NIM_DELETE 0x00000002
/// C: #define IMAGE_ICON 1 / #define LR_LOADFROMFILE 0x00000010
/// C: HANDLE WINAPI LoadImageW (HINSTANCE hinst, LPCWSTR lpszName, UINT uType, int cxDesired, int
/// C: cyDesired, UINT fuLoad);
/// C: HANDLE WINAPI LoadImageA (HINSTANCE hinst, LPCSTR lpszName, UINT uType, int cxDesired, int
/// C: cyDesired, UINT fuLoad);
/// C: BOOL WINAPI DestroyIcon (HICON hIcon);
/// C: wchar_t *wcscpy (wchar_t *dest, const wchar_t *src);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIM_ADD: DWORD = 0x00000000;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const NIM_DELETE: DWORD = 0x00000002;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const IMAGE_ICON: UINT = 1;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
pub const LR_LOADFROMFILE: UINT = 0x00000010;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe extern "system" {
  pub fn LoadImageW (hinst: HINSTANCE, lpszName: LPCWSTR, uType: UINT, cxDesired: c_int,
		     cyDesired: c_int, fuLoad: UINT) -> HANDLE;
  pub fn LoadImageA (hinst: HINSTANCE, lpszName: LPCSTR, uType: UINT, cxDesired: c_int,
		     cyDesired: c_int, fuLoad: UINT) -> HANDLE;
  pub fn DestroyIcon (hIcon: HICON) -> BOOL;
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe extern "C" {
  pub fn wcscpy (dest: *mut c_ushort, src: *const c_ushort) -> *mut c_ushort;
}

/// C 10429: `static unsigned short last_tray_notification_id;` - the tray counter (C's
/// tentative-definition zero).
/// C: static unsigned short last_tray_notification_id;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
static mut last_tray_notification_id: c_ushort = 0;

/// C 10431-10605: `add_tray_notification` - C 10438's `retval`, C 10442-10445's locals (`nidw`,
/// the Shell32 version, the three wide buffers and `tiplen`), C 10447's `memset`, C 10449-10463's
/// comment and four version-dependent `cbSize` choices, C 10464-10472's window/counter/flags/message
/// fills, C 10473-10501's icon load (the class icon, or the file's own through
/// `filename_to_utf16`/`filename_to_ansi` with C 10483/10496's `ENOENT` exits), C 10502-10514's
/// `GetLastError` split, C 10516-10540's tip conversion (`pMultiByteToWideChar` over
/// `utf8_mbslen_lim`'s cap and `wcscpy`), C 10542-10589's Windows-2000 tail (the message, the
/// timeout, the title and the `severity` switch), C 10591-10599's `NIM_ADD` call, C 10600-10602's
/// `done:` cleanup and C 10604's return.  C's three `goto done` sites are the labelled block below.
/// C: static int
/// C: add_tray_notification (struct frame *f, const char *icon, const char *tip,
/// C:                       enum NI_Severity severity, unsigned timeout,
/// C:                       const char *title, const char *msg)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe fn add_tray_notification (f: *mut frame, icon: *const c_char, tip: *const c_char,
				 severity: c_int, timeout: c_uint, title: *const c_char,
				 msg: *const c_char) -> c_int
{
  let mut retval: c_int = -1;

  if FRAME_W32_P (f)
    {
      let mut nidw: MY_NOTIFYICONDATAW = core::mem::zeroed ();
      let shell_dll_version: ULONGLONG = get_dll_version (c"Shell32.dll".as_ptr ());
      let mut tipw: [c_ushort; 128] = [0; 128];
      let mut msgw: [c_ushort; 256] = [0; 256];
      let mut titlew: [c_ushort; 64] = [0; 64];
      let mut tiplen: c_int;

      memset (&raw mut nidw as *mut c_void, 0, core::mem::size_of_val (&nidw));

      /* MSDN says the full struct is supported since Vista, whose
	 Shell32.dll version is said to be 6.0.6.  But DllGetVersion
	 cannot report the 3rd field value, it reports "build number"
	 instead, which is something else.  So we use the Windows 7's
	 version 6.1 as cutoff, and Vista loses.  (Actually, the loss
	 is not a real one, since we don't expose the hBalloonIcon
	 member of the struct to Lisp.)  */
      if shell_dll_version >= MAKEDLLVERULL! (6, 1, 0, 0) /* >= Windows 7 */
	{ nidw.cbSize = core::mem::size_of_val (&nidw) as DWORD; }
      else if shell_dll_version >= MAKEDLLVERULL! (6, 0, 0, 0) /* XP */
	{ nidw.cbSize = MYNOTIFYICONDATAW_V3_SIZE! () as DWORD; }
      else if shell_dll_version >= MAKEDLLVERULL! (5, 0, 0, 0) /* W2K */
	{ nidw.cbSize = MYNOTIFYICONDATAW_V2_SIZE! () as DWORD; }
      else
	{ nidw.cbSize = MYNOTIFYICONDATAW_V1_SIZE! () as DWORD; }	/* < W2K */
      nidw.hWnd = FRAME_W32_WINDOW! (f);
      if last_tray_notification_id == 0
	{ last_tray_notification_id = EMACS_TRAY_NOTIFICATION_ID_INIT as c_ushort; }
      else
	{ last_tray_notification_id += 1; }
      retval = last_tray_notification_id as c_int;
      nidw.uID = last_tray_notification_id as UINT;
      nidw.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_INFO;
      nidw.uCallbackMessage = EMACS_NOTIFICATION_MSG as UINT;
      if *icon == 0
	{ nidw.hIcon = LoadIcon (hinst, EMACS_CLASS.as_ptr () as LPSTR); }
      else
	{
	  if w32_unicode_filenames
	    {
	      let mut icon_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];

	      if filename_to_utf16 (icon, icon_w.as_mut_ptr ()) != 0
		{
		  *errno () = ENOENT;
		  return -1;
		}
	      nidw.hIcon = LoadImageW (core::ptr::null_mut (), icon_w.as_ptr (), IMAGE_ICON, 0, 0,
				       LR_DEFAULTSIZE | LR_LOADFROMFILE) as HICON;
	    }
	  else
	    {
	      let mut icon_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];

	      if filename_to_ansi (icon, icon_a.as_mut_ptr ()) != 0
		{
		  *errno () = ENOENT;
		  return -1;
		}
	      nidw.hIcon = LoadImageA (core::ptr::null_mut (), icon_a.as_ptr (), IMAGE_ICON, 0, 0,
				       LR_DEFAULTSIZE | LR_LOADFROMFILE) as HICON;
	    }
	}
      if nidw.hIcon.is_null ()
	{
	  match GetLastError () as c_int
	    {
	    ERROR_FILE_NOT_FOUND => { *errno () = ENOENT; }
	    _ => { *errno () = ENOMEM; }
	    }
	  return -1;
	}

      /* Windows 9X and NT4 support only 64 characters in the Tip,
	 later versions support up to 128.  */
      'done: {
	if nidw.cbSize == MYNOTIFYICONDATAW_V1_SIZE! () as DWORD
	  {
	    tiplen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
						      tip, utf8_mbslen_lim (tip, 63),
						      tipw.as_mut_ptr (), 64);
	    if tiplen >= 63
	      { tipw[63] = 0; }
	  }
	else
	  {
	    tiplen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
						      tip, utf8_mbslen_lim (tip, 127),
						      tipw.as_mut_ptr (), 128);
	    if tiplen >= 127
	      { tipw[127] = 0; }
	  }
	if tiplen == 0
	  {
	    *errno () = EINVAL;
	    retval = -1;
	    break 'done;
	  }
	wcscpy (nidw.szTip.as_mut_ptr (), tipw.as_ptr ());

	/* The rest of the structure is only supported since Windows 2000.  */
	if nidw.cbSize > MYNOTIFYICONDATAW_V1_SIZE! () as DWORD
	  {
	    let mut slen: c_int;

	    slen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
						     msg, utf8_mbslen_lim (msg, 255),
						     msgw.as_mut_ptr (), 256);
	    if slen >= 255
	      { msgw[255] = 0; }
	    else if slen == 0
	      {
		*errno () = EINVAL;
		retval = -1;
		break 'done;
	      }
	    wcscpy (nidw.szInfo.as_mut_ptr (), msgw.as_ptr ());
	    nidw.r#DUMMYUNIONNAME.uTimeout = timeout;   /* C's anonymous union member */
	    slen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, multiByteToWideCharFlags,
						     title, utf8_mbslen_lim (title, 63),
						     titlew.as_mut_ptr (), 64);
	    if slen >= 63
	      { titlew[63] = 0; }
	    else if slen == 0
	      {
		*errno () = EINVAL;
		retval = -1;
		break 'done;
	      }
	    wcscpy (nidw.szInfoTitle.as_mut_ptr (), titlew.as_ptr ());

	    match severity
	      {
	      Ni_None => { nidw.dwInfoFlags = NIIF_NONE; }
	      Ni_Info => { nidw.dwInfoFlags = NIIF_INFO; }
	      Ni_Warn => { nidw.dwInfoFlags = NIIF_WARNING; }
	      Ni_Err => { nidw.dwInfoFlags = NIIF_ERROR; }
	      _ => { nidw.dwInfoFlags = NIIF_INFO; }	/* C's `default:` (with `Ni_Info`) */
	      }
	  }

	if (pfnShell_NotifyIconW.unwrap ()) (NIM_ADD, &raw mut nidw as *mut c_void) == 0
	  {
	    /* GetLastError returns meaningless results when
	       Shell_NotifyIcon fails.  */
	    DebPrint! ((c"Shell_NotifyIcon ADD failed (err=%d)\n".as_ptr (),
			GetLastError ()));
	    *errno () = EINVAL;
	    retval = -1;
	  }
      }

      if *icon != 0 && DestroyIcon (nidw.hIcon) == 0
	{ DebPrint! ((c"DestroyIcon failed (err=%d)\n".as_ptr (), GetLastError ())); }
    }
  return retval;
}

/// C 10607-10633: `delete_tray_notification` - C 10607-10611's own comment, C 10617's `nidw`,
/// C 10619-10621's three fills, C 10623-10630's `NIM_DELETE` call (C 10627's `DebPrint`) and
/// C 10631-10632's two returns.
/// C: static void
/// C: delete_tray_notification (struct frame *f, int id)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe fn delete_tray_notification (f: *mut frame, id: c_int)
{
  if FRAME_W32_P (f)
    {
      let mut nidw: MY_NOTIFYICONDATAW = core::mem::zeroed ();

      memset (&raw mut nidw as *mut c_void, 0, core::mem::size_of_val (&nidw));
      nidw.hWnd = FRAME_W32_WINDOW! (f);
      nidw.uID = id as UINT;

      if (pfnShell_NotifyIconW.unwrap ()) (NIM_DELETE, &raw mut nidw as *mut c_void) == 0
	{
	  /* GetLastError returns meaningless results when
	     Shell_NotifyIcon fails.  */
	  DebPrint! ((c"Shell_NotifyIcon DELETE failed\n".as_ptr ()));
	  *errno () = EINVAL;
	  return;
	}
    }
  return;
}

/// C 10720-10754's `plist_get`: `lisp.h` 4280's prototype (`extern Lisp_Object plist_get
/// (Lisp_Object plist, Lisp_Object prop);`), which `include/lisp.rs` does not carry yet and whose
/// definition is `src/w32.c`'s 16056 (`src/w32.rs` 16364 declares the same name for its own unit) -
/// the `decode_live_frame` shape of slice 22.
/// C: extern Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
unsafe extern "C" {
  pub fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
}

/// C 10635-10763: `w32-notification-notify` - C 10700-10708's locals (`0, MANY` per C 10637),
/// C 10711's `maybe_load_unicows_dll`, C 10713-10714's argument-count/pointer guard,
/// C 10715's `CHECK_KEYWORD_ARGS`, C 10717's `Flist` and C 10719-10758's five `plist_get` reads
/// (the icon through `ENCODE_FILE`/`Fexpand_file_name`, the three strings through
/// `code_convert_string_norecord` with C's `1`, and C 10733-10744's severity ladder), then
/// C 10761-10762's `add_tray_notification` call and the nil-or-fixnum return.
/// C: DEFUN ("w32-notification-notify",
/// C:        Fw32_notification_notify, Sw32_notification_notify,
/// C:        0, MANY, 0,
/// C:        doc: /* Display an MS-Windows tray notification as specified by PARAMS. … */)
/// C:   (ptrdiff_t nargs, Lisp_Object *args)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
DEFUN! ("w32-notification-notify", Fw32_notification_notify, Sw32_notification_notify,
	0, MANY, 0,
	doc: "Display an MS-Windows tray notification as specified by PARAMS.\n\nValue is the integer unique ID of the notification that can be used\nto remove the notification using `w32-notification-close', which see.\nIf the function fails, the return value is nil.\n\nTray notifications, a.k.a. \"taskbar messages\", are messages that\ninform the user about events unrelated to the current user activity,\nsuch as a significant system event, by briefly displaying informative\ntext in a balloon from an icon in the notification area of the taskbar.\n\nParameters in PARAMS are specified as keyword/value pairs.  All the\nparameters are optional, but if no parameters are specified, the\nfunction will do nothing and return nil.\n\nThe following parameters are supported:\n\n:icon ICON       -- Display ICON in the system tray.  If ICON is a string,\n                    it should specify a file name from which to load the\n                    icon; the specified file should be a .ico Windows icon\n                    file.  If ICON is not a string, or if this parameter\n                    is not specified, the standard Emacs icon will be used.\n\n:tip TIP         -- Use TIP as the tooltip for the notification.  If TIP\n                    is a string, this is the text of a tooltip that will\n                    be shown when the mouse pointer hovers over the tray\n                    icon added by the notification.  If TIP is not a\n                    string, or if this parameter is not specified, the\n                    default tooltip text is \"Emacs notification\".  The\n                    tooltip text can be up to 127 characters long (63\n                    on Windows versions before W2K).  Longer strings\n                    will be truncated.\n\n:level LEVEL     -- Notification severity level, one of `info',\n                    `warning', or `error'.  If given, the value\n                    determines the icon displayed to the left of the\n                    notification title, but only if the `:title'\n                    parameter (see below) is also specified and is a\n                    string.\n\n:title TITLE     -- The title of the notification.  If TITLE is a string,\n                    it is displayed in a larger font immediately above\n                    the body text.  The title text can be up to 63\n                    characters long; longer text will be truncated.\n\n:body BODY       -- The body of the notification.  If BODY is a string,\n                    it specifies the text of the notification message.\n                    Use embedded newlines to control how the text is\n                    broken into lines.  The body text can be up to 255\n                    characters long, and will be truncated if it's longer.\n\nNote that versions of Windows before W2K support only `:icon' and `:tip'.\nYou can pass the other parameters, but they will be ignored on\nthose old systems.\n\nThere can be at most one active notification at any given time per each\nEmacs frame.  An active notification must be removed by calling the\nfunction `w32-notification-close', with the same frame selected as the\none which was selected when the notification was created, before a new\none can be shown for the same frame.  The caller must track which\nnotification was created from which frame, using the returned ID value.\n\nusage: (w32-notification-notify &rest PARAMS)",
	(nargs: isize, args: *mut Lisp_Object),
	{
  let f: *mut frame = SELECTED_FRAME ();
  let mut arg_plist: Lisp_Object;
  let mut lres: Lisp_Object;
  let mut retval: c_int;
  let mut icon: *mut c_char;
  let mut tip: *mut c_char;
  let mut title: *mut c_char;
  let mut msg: *mut c_char;
  let mut severity: c_int;
  let mut timeout: c_uint = 0;

  /* Required on Windows 9X.  */
  maybe_load_unicows_dll ();

  if nargs == 0 || pfnShell_NotifyIconW.is_none ()
    { return Qnil (); }
  CHECK_KEYWORD_ARGS (nargs);

  arg_plist = Flist (nargs, args);

  /* Icon.  */
  lres = plist_get (arg_plist, QCicon ());
  if STRINGP (lres)
    { icon = SSDATA (ENCODE_FILE (Fexpand_file_name (lres, Qnil ()))) as *mut c_char; }
  else
    { icon = c"".as_ptr () as *mut c_char; }

  /* Tip.  */
  lres = plist_get (arg_plist, QCtip ());
  if STRINGP (lres)
    { tip = SSDATA (code_convert_string_norecord (lres, Qutf_8 (), true)) as *mut c_char; }
  else
    { tip = c"Emacs notification".as_ptr () as *mut c_char; }

  /* Severity.  */
  lres = plist_get (arg_plist, QClevel ());
  if NILP (lres)
    { severity = Ni_None; }
  else if EQ (lres, Qinfo ())
    { severity = Ni_Info; }
  else if EQ (lres, Qwarning ())
    { severity = Ni_Warn; }
  else if EQ (lres, Qerror ())
    { severity = Ni_Err; }
  else
    { severity = Ni_Info; }

  /* Title.  */
  lres = plist_get (arg_plist, QCtitle ());
  if STRINGP (lres)
    { title = SSDATA (code_convert_string_norecord (lres, Qutf_8 (), true)) as *mut c_char; }
  else
    { title = c"".as_ptr () as *mut c_char; }

  /* Notification body text.  */
  lres = plist_get (arg_plist, QCbody ());
  if STRINGP (lres)
    { msg = SSDATA (code_convert_string_norecord (lres, Qutf_8 (), true)) as *mut c_char; }
  else
    { msg = c"".as_ptr () as *mut c_char; }

  /* Do it!  */
  retval = add_tray_notification (f, icon, tip, severity, timeout, title, msg);
  return if retval < 0 { Qnil () } else { make_fixnum (retval as EMACS_INT) };
});

/// C 10765-10779: `w32-notification-close` - C 10773's selected frame and C 10775-10776's
/// `FIXNUMP`/pointer test around C 10776's `delete_tray_notification` call.
/// C: DEFUN ("w32-notification-close",
/// C:        Fw32_notification_close, Sw32_notification_close,
/// C:        1, 1, 0,
/// C:        doc: /* Remove the MS-Windows tray notification specified by its ID. … */)
/// C:   (Lisp_Object id)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus")))]
DEFUN! ("w32-notification-close", Fw32_notification_close, Sw32_notification_close,
	1, 1, 0,
	doc: "Remove the MS-Windows tray notification specified by its ID.\nThe frame which was selected when the notification was created must\nbe selected when removing the notification.",
	(id: Lisp_Object),
	{
  let f: *mut frame = SELECTED_FRAME ();

  if FIXNUMP (id) && pfnShell_NotifyIconW.is_some ()
    { delete_tray_notification (f, XFIXNUM (id) as c_int); }

  return Qnil ();
});

/* C 10781's `#endif  /* WINDOWSNT && !HAVE_DBUS */` closes the tray block; C's line itself has no
   counterpart beyond the end of the three-condition items above (PORTING.md 0.1-4).  */

/// C 10783-10818: `w32-get-ime-open-status` - C 10799-10802's selected-frame test (C's
/// `FRAMEP (selected_frame) && FRAME_LIVE_P (XFRAME (selected_frame))` ternary with `NULL`),
/// C 10806-10807's window and IME context, C 10810-10811's two pointer calls, C 10813's
/// `retval ? Qt : Qnil` and C 10817's fall-through return.  This item is **outside** the tray
/// block's `#ifdef` (C 10781 closed it): its gate is the file gate alone.
/// C: DEFUN ("w32-get-ime-open-status",
/// C:        Fw32_get_ime_open_status, Sw32_get_ime_open_status,
/// C:        0, 0, 0,
/// C:        doc: /* Return non-nil if IME is active, otherwise return nil. … */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-get-ime-open-status", Fw32_get_ime_open_status, Sw32_get_ime_open_status,
	0, 0, 0,
	doc: "Return non-nil if IME is active, otherwise return nil.\n\nIME, the MS-Windows Input Method Editor, can be active or inactive.\nThis function returns non-nil if the IME is active, otherwise nil.\n\nCaveat: on Windows 11 and later, this function might return non-nil\neven when IME is not active, or nil when it's active.  This is due to\n\"new\" TSF-based IME which have known compatibility issues with\nIME-related APIs which Emacs uses.  A workaround is to switch to the\nlegacy IME mode, a.k.a. the \"previous version of Microsoft IME\", in\nthe IME Compatibility settings.",
	(),
	{
  let sf: *mut frame
    = if FRAMEP (selected_frame) && FRAME_LIVE_P (XFRAME (selected_frame))
      { XFRAME (selected_frame) }
      else { core::ptr::null_mut () };

  if ! sf.is_null ()
    {
      let current_window: HWND = FRAME_W32_WINDOW! (sf);
      let context: HIMC = (get_ime_context_fn.unwrap ()) (current_window);
      if ! context.is_null ()
	{
	  let retval: BOOL = (get_ime_open_status_fn.unwrap ()) (context);
	  (release_ime_context_fn.unwrap ()) (current_window, context);

	  return if retval != 0 { Qt () } else { Qnil () };
	}
    }

  return Qnil ();
});

/// C 10820-10833: `w32-set-ime-open-status` - C 10829's `unsigned` from C's nil test and
/// C 10831's `PostThreadMessage (dwWindowsThreadId, WM_EMACS_IME_STATUS, …)`.
/// C: DEFUN ("w32-set-ime-open-status",
/// C:        Fw32_set_ime_open_status, Sw32_set_ime_open_status,
/// C:        1, 1, 0,
/// C:        doc: /* Open or close the IME according to STATUS. … */)
/// C:   (Lisp_Object status)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-set-ime-open-status", Fw32_set_ime_open_status, Sw32_set_ime_open_status,
	1, 1, 0,
	doc: "Open or close the IME according to STATUS.\n\nThis function activates the IME, the MS-Windows Input Method Editor,\nif STATUS is non-nil, otherwise it deactivates the IME.",
	(status: Lisp_Object),
	{
  let ime_status: c_uint = if NILP (status) { 0 } else { 1 };

  PostThreadMessage (dwWindowsThreadId, WM_EMACS_IME_STATUS as UINT,
		     ime_status as WPARAM, 0);
  return Qnil ();
});

/* C 10836's `#ifdef WINDOWSNT` opens the registry section, whose banner C 10837-10839 carries
   below as C's own text (the C 8116-8118 wrapper device); C 10902's `#endif` closes the section, and
   C 10904-10956's `w32_get_resource` stands after it (the file gate alone).  */

/*
/***********************************************************************
			  Reading Registry
 ***********************************************************************/
*/

/// C 10840-10900: `w32-read-registry` - C 10877-10879's three checks, C 10881-10893's root-key
/// ladder (`HKEY_CLASSES_ROOT`/`HKEY_USERS`/`HKEY_CURRENT_CONFIG` are spelled in the C 890-932 block
/// above) with C 10893's `error` over `SDATA (SYMBOL_NAME (root))`, C 10895's first
/// `w32_read_registry` call and C 10896-10897's nil-root retry under `HKEY_LOCAL_MACHINE`.
/// C: DEFUN ("w32-read-registry",
/// C:        Fw32_read_registry, Sw32_read_registry,
/// C:        3, 3, 0,
/// C:        doc: /* Return the value stored in MS-Windows Registry under ROOT/KEY/NAME. … */)
/// C:   (Lisp_Object root, Lisp_Object key, Lisp_Object name)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-read-registry", Fw32_read_registry, Sw32_read_registry,
	3, 3, 0,
	doc: "Return the value stored in MS-Windows Registry under ROOT/KEY/NAME.\n\nROOT is a symbol, one of `HKCR', `HKCU', `HKLM', `HKU', or `HKCC'.\nIt can also be nil, which means try `HKCU', and if that fails, try `HKLM'.\n\nKEY and NAME must be strings, and NAME must not include slashes.\nKEY can use either forward- or back-slashes.\nTo access the default value of KEY (if it is defined), use NAME\nthat is an empty string.\n\nIf the named KEY or its subkey called NAME don't exist, or cannot be\naccessed by the current user, the function returns nil.  Otherwise,\nthe return value depends on the type of the data stored in Registry:\n\n  If the data type is REG_NONE, the function returns t.\n  If the data type is REG_DWORD or REG_QWORD, the function returns\n    its integer value.  If the value is too large for a fixnum,\n    the function returns a bignum.\n  If the data type is REG_BINARY, the function returns a vector whose\n    elements are individual bytes of the value.\n  If the data type is REG_SZ, the function returns a string.\n  If the data type is REG_EXPAND_SZ, the function returns a string\n    with all the %..% references to environment variables replaced\n    by the values of those variables.  If the expansion fails, or\n    some variables are not defined in the environment, some or all\n    of the environment variables will remain unexpanded.\n  If the data type is REG_MULTI_SZ, the function returns a list whose\n    elements are the individual strings.\n\nNote that this function doesn't know whether a string value is a file\nname, so file names will be returned with backslashes, which may need\nto be converted to forward slashes by the caller.",
	(root: Lisp_Object, key: Lisp_Object, name: Lisp_Object),
	{
  CHECK_SYMBOL (root);
  CHECK_STRING (key);
  CHECK_STRING (name);

  let mut rootkey: HKEY = HKEY_CURRENT_USER;
  if EQ (root, QHKCR ())
    { rootkey = HKEY_CLASSES_ROOT; }
  else if EQ (root, QHKCU ())
    { rootkey = HKEY_CURRENT_USER; }
  else if EQ (root, QHKLM ())
    { rootkey = HKEY_LOCAL_MACHINE; }
  else if EQ (root, QHKU ())
    { rootkey = HKEY_USERS; }
  else if EQ (root, QHKCC ())
    { rootkey = HKEY_CURRENT_CONFIG; }
  else if ! NILP (root)
    { error (c"Unknown root key: %s".as_ptr (), SDATA (SYMBOL_NAME (root))); }

  let mut val: Lisp_Object = w32_read_registry (rootkey, key, name);
  if NILP (val) && NILP (root)
    { val = w32_read_registry (HKEY_LOCAL_MACHINE, key, name); }

  return val;
});

/* C 10902's `#endif  /* WINDOWSNT */` closes the section; C's line itself has no counterpart
   beyond the end of the two-condition item above (PORTING.md 0.1-4).  */

/// C 10904-10956: `w32_get_resource` (C's own comment first) - C 10914-10916's three locals,
/// C 10921-10936's `HKEY_CURRENT_USER` block and C 10938-10953's `HKEY_LOCAL_MACHINE` block (each:
/// `RegOpenKeyEx`, the two `RegQueryValueEx` calls with C's assignment-in-condition written as the
/// port's `&& { … }` block, `xfree` and `RegCloseKey`) and C 10955's `NULL` return.
/// C: LPBYTE
/// C: w32_get_resource (const char *key, const char *name, LPDWORD lpdwtype)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_get_resource (key: *const c_char, name: *const c_char,
					   lpdwtype: LPDWORD) -> LPBYTE
{
  let mut lpvalue: LPBYTE;
  let mut hrootkey: HKEY = core::ptr::null_mut ();
  let mut cbData: DWORD;

  /* Check both the current user and the local machine to see if
     we have any resources.  */

  if RegOpenKeyEx (HKEY_CURRENT_USER, key, 0, KEY_READ, &raw mut hrootkey) == ERROR_SUCCESS
    {
      lpvalue = core::ptr::null_mut ();

      if RegQueryValueEx (hrootkey, name, core::ptr::null_mut (), core::ptr::null_mut (),
			  core::ptr::null_mut (), &raw mut cbData) == ERROR_SUCCESS
	  && { lpvalue = xmalloc (cbData as usize) as LPBYTE; ! lpvalue.is_null () }
	  && RegQueryValueEx (hrootkey, name, core::ptr::null_mut (), lpdwtype, lpvalue,
			      &raw mut cbData) == ERROR_SUCCESS
	{
	  RegCloseKey (hrootkey);
	  return lpvalue;
	}

      xfree (lpvalue as *mut c_void);

      RegCloseKey (hrootkey);
    }

  if RegOpenKeyEx (HKEY_LOCAL_MACHINE, key, 0, KEY_READ, &raw mut hrootkey) == ERROR_SUCCESS
    {
      lpvalue = core::ptr::null_mut ();

      if RegQueryValueEx (hrootkey, name, core::ptr::null_mut (), core::ptr::null_mut (),
			  core::ptr::null_mut (), &raw mut cbData) == ERROR_SUCCESS
	  && { lpvalue = xmalloc (cbData as usize) as LPBYTE; ! lpvalue.is_null () }
	  && RegQueryValueEx (hrootkey, name, core::ptr::null_mut (), lpdwtype, lpvalue,
			      &raw mut cbData) == ERROR_SUCCESS
	{
	  RegCloseKey (hrootkey);
	  return lpvalue;
	}

      xfree (lpvalue as *mut c_void);

      RegCloseKey (hrootkey);
    }

  return core::ptr::null_mut ();
}

/// C 10964-10965 and C 11018-11019: the two loads-by-hand pointers this section carries, and
/// C 10987-10988/10997-10998's explicit `SystemParametersInfoW`/`A` pair (the alias
/// `SystemParametersInfo` is declared in the C 1997 block above), `SPI_SETDESKWALLPAPER` and
/// `SPIF_SENDCHANGE`, `GetTickCount` and C 11028's `LASTINPUTINFO` with its pointer type.
/// C: typedef BOOL (WINAPI * SystemParametersInfoW_Proc) (UINT,UINT,PVOID,UINT);
/// C: typedef BOOL (WINAPI *GetLastInputInfo_Proc) (PLASTINPUTINFO);
/// C: typedef struct tagLASTINPUTINFO { UINT cbSize; DWORD dwTime; } LASTINPUTINFO, *PLASTINPUTINFO;
/// C: BOOL WINAPI SystemParametersInfoW (UINT uiAction, UINT uiParam, PVOID pvParam, UINT fWinIni);
/// C: BOOL WINAPI SystemParametersInfoA (UINT uiAction, UINT uiParam, PVOID pvParam, UINT fWinIni);
/// C: BOOL WINAPI GetLastInputInfo (PLASTINPUTINFO plii);
/// C: DWORD WINAPI GetTickCount (void);
/// C: #define SPI_SETDESKWALLPAPER 0x0014 / #define SPIF_SENDCHANGE 0x2
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type SystemParametersInfoW_Proc
  = Option<unsafe extern "system" fn (arg0: UINT, arg1: UINT, arg2: *mut c_void,
				      arg3: UINT) -> BOOL>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetLastInputInfo_Proc
  = Option<unsafe extern "system" fn (arg0: PLASTINPUTINFO) -> BOOL>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type PLASTINPUTINFO = *mut LASTINPUTINFO;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LASTINPUTINFO
{
  pub cbSize: UINT,
  pub dwTime: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const SPI_SETDESKWALLPAPER: UINT = 0x0014;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const SPIF_SENDCHANGE: UINT = 0x2;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" {
  pub fn SystemParametersInfoW (uiAction: UINT, uiParam: UINT, pvParam: *mut c_void,
				fWinIni: UINT) -> BOOL;
  pub fn SystemParametersInfoA (uiAction: UINT, uiParam: UINT, pvParam: *mut c_void,
				fWinIni: UINT) -> BOOL;
  pub fn GetLastInputInfo (plii: PLASTINPUTINFO) -> BOOL;
  pub fn GetTickCount () -> DWORD;
}

/// C 10965: `static SystemParametersInfoW_Proc system_parameters_info_w_fn = NULL;` - C's file-scope
/// pointer, which `globals_of_w32fns` loads (C 12460-12475).
/// C: static SystemParametersInfoW_Proc system_parameters_info_w_fn = NULL;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut system_parameters_info_w_fn: SystemParametersInfoW_Proc = None;

/* C 10960-10962: the section's banner, carried as C's own text (the C 8116-8118 wrapper device).  */

/*
/***********************************************************************
\t\t\t    Wallpaper
 ***********************************************************************/
*/

/// C 10967-11015: `w32-set-wallpaper` - C 10971-10974's four locals, C 10976-10979's comment,
/// C 10980-10989's `w32_unicode_filenames` + `system_parameters_info_w_fn` arm (the `wchar_t` buffer,
/// `filename_to_utf16`'s failure setting `ERROR_FILE_NOT_FOUND`, C 10987-10988's
/// `SystemParametersInfoW` call) and C 10990-10999's ANSI arm, then C 11000-11012's three-way error
/// report (`error` over `fname`, `w32_strerror (err)` for the other two).
/// C: DEFUN ("w32-set-wallpaper", Fw32_set_wallpaper, Sw32_set_wallpaper, 1, 1, 0,
/// C:        doc: /* Set the desktop wallpaper image to IMAGE-FILE.  */)
/// C:   (Lisp_Object image_file)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-set-wallpaper", Fw32_set_wallpaper, Sw32_set_wallpaper, 1, 1, 0,
	doc: "Set the desktop wallpaper image to IMAGE-FILE.",
	(image_file: Lisp_Object),
	{
  let encoded: Lisp_Object = ENCODE_FILE (Fexpand_file_name (image_file, Qnil ()));
  let fname: *mut c_char = SSDATA (encoded) as *mut c_char;
  let mut result: BOOL = 0;
  let mut err: DWORD = 0;

  /* UNICOWS.DLL seems to have SystemParametersInfoW, but it doesn't
     seem to be worth the hassle to support that on Windows 9X for the
     benefit of this minor feature.  Let them use on Windows 9X only
     image file names that can be encoded by the system codepage.  */
  if w32_unicode_filenames && system_parameters_info_w_fn.is_some ()
    {
      let mut fname_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];

      if filename_to_utf16 (fname, fname_w.as_mut_ptr ()) != 0
	{ err = ERROR_FILE_NOT_FOUND as DWORD; }
      else
	{ result = SystemParametersInfoW (SPI_SETDESKWALLPAPER, 0,
					  fname_w.as_mut_ptr () as *mut c_void,
					  SPIF_SENDCHANGE); }
    }
  else
    {
      let mut fname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];

      if filename_to_ansi (fname, fname_a.as_mut_ptr ()) != 0
	{ err = ERROR_FILE_NOT_FOUND as DWORD; }
      else
	{ result = SystemParametersInfoA (SPI_SETDESKWALLPAPER, 0,
					  fname_a.as_mut_ptr () as *mut c_void,
					  SPIF_SENDCHANGE); }
    }
  if result == 0
    {
      if err == ERROR_FILE_NOT_FOUND as DWORD
	{ error (c"Wallpaper file %s does not exist or cannot be accessed".as_ptr (), fname); }
      else
	{
	  err = GetLastError ();
	  if err != 0
	    { error (c"Could not set desktop wallpaper: %s".as_ptr (),
		     w32_strerror (err as c_int)); }
	  else
	    { error (c"Could not set desktop wallpaper (wrong image type?)".as_ptr ()); }
	}
    }

  return Qnil ();
});

/// C 11017-11019: C's one-line comment and the second load-by-hand pointer.
/// C: /* Return time in milliseconds since the last input event.  */
/// C: static GetLastInputInfo_Proc get_last_input_info_fn = NULL;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
static mut get_last_input_info_fn: GetLastInputInfo_Proc = None;

/// C 11021-11039: `w32-system-idle-time` - C 11028-11029's `LASTINPUTINFO` with C's `sizeof info`,
/// C 11031's pointer test and call, C 11033's `GetTickCount () - info.dwTime` (C's DWORD
/// subtraction, which wraps - the port keeps that with `wrapping_sub`), C 11034-11035's
/// `EMACS_INT_MAX` guard over the widened value and C 11036/11038's two returns.
/// C: DEFUN ("w32-system-idle-time", Fw32_system_idle_time, Sw32_system_idle_time,
/// C:        0, 0, 0,
/// C:        doc: /* Return the time in milliseconds since last system-wide input event. … */)
/// C:   (void)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-system-idle-time", Fw32_system_idle_time, Sw32_system_idle_time,
	0, 0, 0,
	doc: "Return the time in milliseconds since last system-wide input event.\n\nReturn -1 if the required system API is not available or fails.",
	(),
	{
  let mut info: LASTINPUTINFO = core::mem::zeroed ();
  info.cbSize = core::mem::size_of_val (&info) as UINT;

  if get_last_input_info_fn.is_some ()
      && (get_last_input_info_fn.unwrap ()) (&raw mut info) != 0
    {
      let time_since_last_input: DWORD
	= GetTickCount ().wrapping_sub (info.dwTime);
      if time_since_last_input as EMACS_INT > EMACS_INT_MAX
	{ return Vmost_positive_fixnum; }
      return make_fixnum (time_since_last_input as EMACS_INT);
    }
  return make_fixnum (-1);
});

/* C 11041's `#endif` closes the wallpaper/idle-time `#ifdef WINDOWSNT`; C's line itself has no
   counterpart beyond the end of the two two-condition items above (PORTING.md 0.1-4).  */

/* C 11044's `#ifdef WINDOWSNT` opens the badge/taskbar section; C 11050-11054's `#ifndef MINGW_W64`
   guard and the two GUIDs, C 11056-11245's `w32-badge`, and C 11348's `#endif` close it at the end
   of the next slice.  */

/*
/***********************************************************************
		     Taskbar Indicators
 ***********************************************************************/
*/

/// C 11050-11054: the `#ifndef MINGW_W64` guard with the SDK's two `DEFINE_GUID`s ("mingw.org's
/// MinGW doesn't have this stuff").  `DEFINE_GUID` defines a const GUID object; the port spells the
/// eleven numbers as the `GUID` record above.
/// C: DEFINE_GUID(CLSID_TaskbarList, 0x56fdf344, 0xfd6d, 0x11d0, 0x95,0x8a, 0x00,0x60,0x97,0xc9,0xa0,0x90);
/// C: DEFINE_GUID(IID_ITaskbarList3, 0xea1afb91, 0x9e28, 0x4b86, 0x90,0xe9, 0x9e,0x9f,0x8a,0x5e,0xef,0xaf);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub static CLSID_TaskbarList: GUID
  = GUID { Data1: 0x56fdf344, Data2: 0xfd6d, Data3: 0x11d0,
	   Data4: [0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90] };
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub static IID_ITaskbarList3: GUID
  = GUID { Data1: 0xea1afb91, Data2: 0x9e28, Data3: 0x4b86,
	   Data4: [0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf] };

/// C 11077/11079-11083's COM entry points and C 11084's `S_OK`, C 11113-11114's `SM_CXSMICON`,
/// C 11119-11130's `BITMAPV5HEADER` choices (its own record below), C 11133-11134's
/// `DIB_RGB_COLORS`, C 11155's `SPI_GETNONCLIENTMETRICS`, C 11160's `DEFAULT_GUI_FONT`,
/// C 11166-11169's three `LOGFONT` values, C 11174's `LOGPIXELSY`, C 11176's `MM_TEXT`,
/// C 11182's `TRANSPARENT` and C 11191's four `DT_*` flags.
/// C: HRESULT WINAPI CoInitialize (LPVOID pvReserved);
/// C: HRESULT WINAPI CoCreateInstance (REFCLSID rclsid, LPUNKNOWN pUnkOuter, DWORD dwClsContext,
/// C: REFIID riid, LPVOID *ppv);
/// C: #define CLSCTX_INPROC_SERVER 0x1
/// C: #define SM_CXSMICON 49 / #define SPI_GETNONCLIENTMETRICS 0x0029
/// C: #define BI_BITFIELDS 3 / #define DIB_RGB_COLORS 0
/// C: #define MM_TEXT 1 / #define TRANSPARENT 1 / #define DEFAULT_GUI_FONT 17 / #define LOGPIXELSY 90
/// C: #define OUT_OUTLINE_PRECIS 8 / #define ANTIALIASED_QUALITY 4
/// C: #define DT_CENTER 0x00000001 / #define DT_VCENTER 0x00000004 / #define DT_SINGLELINE
/// C: 0x00000020 / #define DT_NOCLIP 0x00000100
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const SM_CXSMICON: c_int = 49;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const SPI_GETNONCLIENTMETRICS: UINT = 0x0029;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const BI_BITFIELDS: DWORD = 3;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DIB_RGB_COLORS: UINT = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const MM_TEXT: c_int = 1;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const TRANSPARENT: c_int = 1;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DEFAULT_GUI_FONT: c_int = 17;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const LOGPIXELSY: c_int = 90;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const OUT_OUTLINE_PRECIS: BYTE = 8;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const ANTIALIASED_QUALITY: BYTE = 4;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DT_CENTER: UINT = 0x00000001;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DT_VCENTER: UINT = 0x00000004;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DT_SINGLELINE: UINT = 0x00000020;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const DT_NOCLIP: UINT = 0x00000100;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "system" {
  pub fn CoInitialize (pvReserved: *mut c_void) -> HRESULT;
  pub fn CoCreateInstance (rclsid: *const GUID, pUnkOuter: *mut c_void, dwClsContext: DWORD,
			   riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT;
  pub fn CreateCompatibleDC (hdc: HDC) -> HDC;
  pub fn CreateDIBSection (hdc: HDC, pbmi: *const BITMAPINFO, usage: UINT,
			   ppvBits: *mut *mut c_void, hSection: HANDLE, offset: DWORD) -> HBITMAP;
  pub fn CreateSolidBrush (color: COLORREF) -> HBRUSH;
  pub fn Ellipse (hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
  pub fn GetStockObject (i: c_int) -> HGDIOBJ;
  pub fn SetMapMode (hdc: HDC, iMode: c_int) -> c_int;
  pub fn SetBkMode (hdc: HDC, mode: c_int) -> c_int;
  pub fn SetTextColor (hdc: HDC, color: COLORREF) -> COLORREF;
  /// C 11188's `DrawText` is the SDK's alias (`DrawTextA`), which is the one C's `char` badge wants -
  /// the C 890-932 block's `RegOpenKeyEx` convention, so the port keeps C's name.
  /// C: int WINAPI DrawTextA (HDC hdc, LPCSTR lpchText, int cchText, LPRECT lprc, UINT format);
  pub fn DrawText (hdc: HDC, lpchText: LPCSTR, cchText: c_int, lprc: *mut RECT,
		   format: UINT) -> c_int;
  pub fn CreateBitmap (nWidth: c_int, nHeight: c_int, nPlanes: UINT, nBitCount: UINT,
		       lpBits: *const c_void) -> HBITMAP;
  pub fn CreateIconIndirect (piconinfo: *mut ICONINFO) -> HICON;
  pub fn MulDiv (nNumber: c_int, nNumerator: c_int, nDenominator: c_int) -> c_int;
}

/// C 11119-11130's `BITMAPV5HEADER` (twenty-four fields, corroborated against the Win32 metadata
/// projection - `windows-sys` 0.52's record; the layout is unmeasured here) with C 11133's
/// `BITMAPINFO` cast (its header record below) and C 11125-11130's `ICONINFO`.
/// C: typedef struct { DWORD bV5Size; LONG bV5Width; LONG bV5Height; WORD bV5Planes;
/// C: WORD bV5BitCount; DWORD bV5Compression; DWORD bV5SizeImage; LONG bV5XPelsPerMeter;
/// C: LONG bV5YPelsPerMeter; DWORD bV5ClrUsed; DWORD bV5ClrImportant; DWORD bV5RedMask;
/// C: DWORD bV5GreenMask; DWORD bV5BlueMask; DWORD bV5AlphaMask; DWORD bV5CSType;
/// C: CIEXYZTRIPLE bV5Endpoints; DWORD bV5GammaRed; DWORD bV5GammaGreen; DWORD bV5GammaBlue;
/// C: DWORD bV5Intent; DWORD bV5ProfileData; DWORD bV5ProfileSize; DWORD bV5Reserved; }
/// C: BITMAPV5HEADER;
/// C: typedef struct tagRGBQUAD { BYTE rgbBlue; BYTE rgbGreen; BYTE rgbRed; BYTE rgbReserved; } RGBQUAD;
/// C: typedef struct tagBITMAPINFOHEADER { DWORD biSize; LONG biWidth; LONG biHeight; WORD biPlanes;
/// C: WORD biBitCount; DWORD biCompression; DWORD biSizeImage; LONG biXPelsPerMeter;
/// C: LONG biYPelsPerMeter; DWORD biClrUsed; DWORD biClrImportant; } BITMAPINFOHEADER;
/// C: typedef struct tagBITMAPINFO { BITMAPINFOHEADER bmiHeader; RGBQUAD bmiColors[1]; } BITMAPINFO;
/// C: typedef struct tagICONINFO { BOOL fIcon; DWORD xHotspot; DWORD yHotspot; HBITMAP hbmMask;
/// C: HBITMAP hbmColor; } ICONINFO;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPV5HEADER
{
  pub bV5Size: DWORD,
  pub bV5Width: LONG,
  pub bV5Height: LONG,
  pub bV5Planes: WORD,
  pub bV5BitCount: WORD,
  pub bV5Compression: DWORD,
  pub bV5SizeImage: DWORD,
  pub bV5XPelsPerMeter: LONG,
  pub bV5YPelsPerMeter: LONG,
  pub bV5ClrUsed: DWORD,
  pub bV5ClrImportant: DWORD,
  pub bV5RedMask: DWORD,
  pub bV5GreenMask: DWORD,
  pub bV5BlueMask: DWORD,
  pub bV5AlphaMask: DWORD,
  pub bV5CSType: DWORD,
  pub bV5Endpoints: CIEXYZTRIPLE,
  pub bV5GammaRed: DWORD,
  pub bV5GammaGreen: DWORD,
  pub bV5GammaBlue: DWORD,
  pub bV5Intent: DWORD,
  pub bV5ProfileData: DWORD,
  pub bV5ProfileSize: DWORD,
  pub bV5Reserved: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RGBQUAD
{
  pub rgbBlue: BYTE,
  pub rgbGreen: BYTE,
  pub rgbRed: BYTE,
  pub rgbReserved: BYTE,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
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
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPINFO
{
  pub bmiHeader: BITMAPINFOHEADER,
  pub bmiColors: [RGBQUAD; 1],
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ICONINFO
{
  pub fIcon: BOOL,
  pub xHotspot: DWORD,
  pub yHotspot: DWORD,
  pub hbmMask: HBITMAP,
  pub hbmColor: HBITMAP,
}

/// C 11152-11156's `NONCLIENTMETRICS` (fifteen fields, the `A` form: this file's `LOGFONT` is the
/// `CHAR`-faced one and C's `SystemParametersInfo` call below is the alias whose ANSI form fills a
/// `char` buffer - the same reading), corroborated against `windows-sys` 0.52's `NONCLIENTMETRICSW`.
/// C: typedef struct tagNONCLIENTMETRICSA { UINT cbSize; int iBorderWidth; int iScrollWidth;
/// C: int iScrollHeight; int iCaptionWidth; int iCaptionHeight; LOGFONTA lfCaptionFont;
/// C: int iSmCaptionWidth; int iSmCaptionHeight; LOGFONTA lfSmCaptionFont; int iMenuWidth;
/// C: int iMenuHeight; LOGFONTA lfMenuFont, lfStatusFont, lfMessageFont;
/// C: #if (WINVER >= 0x0600) int iPaddedBorderWidth; #endif } NONCLIENTMETRICSA;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NONCLIENTMETRICS
{
  pub cbSize: UINT,
  pub iBorderWidth: c_int,
  pub iScrollWidth: c_int,
  pub iScrollHeight: c_int,
  pub iCaptionWidth: c_int,
  pub iCaptionHeight: c_int,
  pub lfCaptionFont: LOGFONT,
  pub iSmCaptionWidth: c_int,
  pub iSmCaptionHeight: c_int,
  pub lfSmCaptionFont: LOGFONT,
  pub iMenuWidth: c_int,
  pub iMenuHeight: c_int,
  pub lfMenuFont: LOGFONT,
  pub lfStatusFont: LOGFONT,
  pub lfMessageFont: LOGFONT,
  pub iPaddedBorderWidth: c_int,
}

/// C 11078/11083/11232/11241/11243's `ITaskbarList3`: the interface pointer C declares and the
/// vtable its three calls walk (`SetOverlayIcon`, `Release`; the first store is the inherited
/// `IUnknown` trio).  The method order is the SDK's - `IUnknown`'s three, `ITaskbarList`'s five,
/// `ITaskbarList2`'s one, then `ITaskbarList3`'s twelve - and this machine has **no** authority for
/// it (`windows-sys` 0.52 carries no `ITaskbarList3`), so the vtable is spelled from the documented
/// order and marked unmeasured in the ledger.
/// C: typedef struct ITaskbarList3Vtbl { HRESULT (STDMETHODCALLTYPE *QueryInterface)(ITaskbarList3 *This,
/// C: REFIID riid, void **ppvObject); ULONG (STDMETHODCALLTYPE *AddRef)(ITaskbarList3 *This);
/// C: ULONG (STDMETHODCALLTYPE *Release)(ITaskbarList3 *This);
/// C: HRESULT (STDMETHODCALLTYPE *HrInit)(ITaskbarList3 *This);
/// C: HRESULT (STDMETHODCALLTYPE *AddTab)(ITaskbarList3 *This, HWND hwnd);
/// C: HRESULT (STDMETHODCALLTYPE *DeleteTab)(ITaskbarList3 *This, HWND hwnd);
/// C: HRESULT (STDMETHODCALLTYPE *ActivateTab)(ITaskbarList3 *This, HWND hwnd);
/// C: HRESULT (STDMETHODCALLTYPE *SetActiveAlt)(ITaskbarList3 *This, HWND hwnd);
/// C: HRESULT (STDMETHODCALLTYPE *MarkFullscreenWindow)(ITaskbarList3 *This, HWND hwnd, BOOL fFullscreen);
/// C: HRESULT (STDMETHODCALLTYPE *SetProgressValue)(ITaskbarList3 *This, HWND hwnd,
/// C: ULONGLONG ullCompleted, ULONGLONG ullTotal);
/// C: HRESULT (STDMETHODCALLTYPE *SetProgressState)(ITaskbarList3 *This, HWND hwnd, TBPFLAG tbpFlags);
/// C: HRESULT (STDMETHODCALLTYPE *RegisterTab)(ITaskbarList3 *This, HWND hwndTab, HWND hwndMDI);
/// C: HRESULT (STDMETHODCALLTYPE *UnregisterTab)(ITaskbarList3 *This, HWND hwndTab);
/// C: HRESULT (STDMETHODCALLTYPE *SetTabOrder)(ITaskbarList3 *This, HWND hwndTab, HWND hwndInsertBefore);
/// C: HRESULT (STDMETHODCALLTYPE *SetTabActive)(ITaskbarList3 *This, HWND hwndTab, HWND hwndMDI,
/// C: DWORD dwReserved);
/// C: HRESULT (STDMETHODCALLTYPE *ThumbBarAddButtons)(ITaskbarList3 *This, HWND hwnd, UINT cButtons,
/// C: LPTHUMBBUTTON pButton);
/// C: HRESULT (STDMETHODCALLTYPE *ThumbBarUpdateButtons)(ITaskbarList3 *This, HWND hwnd, UINT cButtons,
/// C: LPTHUMBBUTTON pButton);
/// C: HRESULT (STDMETHODCALLTYPE *ThumbBarSetImageList)(ITaskbarList3 *This, HWND hwnd, HIMAGELIST himl);
/// C: HRESULT (STDMETHODCALLTYPE *SetOverlayIcon)(ITaskbarList3 *This, HWND hwnd, HICON hIcon,
/// C: LPCWSTR pszDescription);
/// C: HRESULT (STDMETHODCALLTYPE *SetThumbnailTooltip)(ITaskbarList3 *This, HWND hwnd, LPCWSTR pszTip);
/// C: HRESULT (STDMETHODCALLTYPE *SetThumbnailClip)(ITaskbarList3 *This, HWND hwnd, RECT *prcClip);
/// C: } ITaskbarList3Vtbl; typedef struct ITaskbarList3 { ITaskbarList3Vtbl *lpVtbl; } ITaskbarList3;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ITaskbarList3Vtbl
{
  pub QueryInterface: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, riid: *const GUID,
							 ppvObject: *mut *mut c_void) -> HRESULT>,
  pub AddRef: Option<unsafe extern "system" fn (This: *mut ITaskbarList3) -> ULONG>,
  pub Release: Option<unsafe extern "system" fn (This: *mut ITaskbarList3) -> ULONG>,
  pub HrInit: Option<unsafe extern "system" fn (This: *mut ITaskbarList3) -> HRESULT>,
  pub AddTab: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND) -> HRESULT>,
  pub DeleteTab: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND) -> HRESULT>,
  pub ActivateTab: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND) -> HRESULT>,
  pub SetActiveAlt: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND) -> HRESULT>,
  pub MarkFullscreenWindow: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							      fFullscreen: BOOL) -> HRESULT>,
  pub SetProgressValue: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							  ullCompleted: ULONGLONG,
							  ullTotal: ULONGLONG) -> HRESULT>,
  pub SetProgressState: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							  tbpFlags: c_int) -> HRESULT>,
  pub RegisterTab: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwndTab: HWND,
						     hwndMDI: HWND) -> HRESULT>,
  pub UnregisterTab: Option<unsafe extern "system" fn (This: *mut ITaskbarList3,
						       hwndTab: HWND) -> HRESULT>,
  pub SetTabOrder: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwndTab: HWND,
						     hwndInsertBefore: HWND) -> HRESULT>,
  pub SetTabActive: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwndTab: HWND,
						      hwndMDI: HWND, dwReserved: DWORD) -> HRESULT>,
  pub ThumbBarAddButtons: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							    cButtons: UINT,
							    pButton: *mut c_void) -> HRESULT>,
  pub ThumbBarUpdateButtons: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							       cButtons: UINT,
							       pButton: *mut c_void) -> HRESULT>,
  pub ThumbBarSetImageList: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							      himl: *mut c_void) -> HRESULT>,
  pub SetOverlayIcon: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							hIcon: HICON,
							pszDescription: LPCWSTR) -> HRESULT>,
  pub SetThumbnailTooltip: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							     pszTip: LPCWSTR) -> HRESULT>,
  pub SetThumbnailClip: Option<unsafe extern "system" fn (This: *mut ITaskbarList3, hwnd: HWND,
							  prcClip: *mut RECT) -> HRESULT>,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ITaskbarList3
{
  pub lpVtbl: *mut ITaskbarList3Vtbl,
}

/// C 11056-11245: `w32-badge` - C 11068-11075's window resolution, C 11077-11085's
/// `CoInitialize`/`CoCreateInstance` pair with C 11084's `S_OK` test, C 11087-11100's badge/colour
/// parsing (`parse_color_spec` twice with C's two `RGB` calls), C 11102-11110's UTF-8 badge and the
/// two `pMultiByteToWideChar` calls over C's `alloca` buffer (`sa_stack_alloc`, P-05),
/// C 11112-11114's icon metrics, C 11116-11137's DC/DIB setup (`BITMAPV5HEADER` with C's
/// twenty-four stores, `CreateDIBSection`, `SelectObject`), C 11139-11144's circle fill,
/// C 11146-11179's font scaling (`SPI_GETNONCLIENTMETRICS` into `NONCLIENTMETRICS`, the `LOGFONT`
/// rework with `MulDiv`/`LOGPIXELSY`, `SetMapMode`'s save/restore), C 11181-11193's text draw,
/// C 11195-11214's alpha-channel walk over `bitmap_pixels`'s two nested loops, C 11216-11237's
/// mask/`ICONINFO`/`CreateIconIndirect` and the `SetOverlayIcon` call, and C 11240-11244's else arm,
/// `Release` and `Qt`.
/// C: DEFUN ("w32-badge",
/// C:        Fw32_badge,
/// C:        Sw32_badge,
/// C:        3, 3, 0,
/// C:        doc: /* Display a taskbar icon overlay image on the selected frame. … */)
/// C:   (Lisp_Object badge, Lisp_Object background, Lisp_Object foreground)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-badge", Fw32_badge, Sw32_badge,
	3, 3, 0,
	doc: "Display a taskbar icon overlay image on the selected frame.\nBADGE is a string.  If BADGE is nil, remove the overlay.  Do nothing if\nWindows does not support the ITaskbarList3 interface and return nil,\notherwise return t.  Do nothing if the selected frame is not (yet)\nassociated with a window handle.  BACKGROUND and FOREGROUND are RGB\ntriplet strings of the form \"#RRGGBB\".",
	(badge: Lisp_Object, background: Lisp_Object, foreground: Lisp_Object),
	{
  let sf: *mut frame = SELECTED_FRAME ();
  let mut hwnd: HWND = core::ptr::null_mut ();

  if FRAME_W32_P (sf) && FRAME_LIVE_P (sf)
    { hwnd = FRAME_W32_WINDOW! (sf); }

  if hwnd.is_null ()
    { return Qnil (); }

  CoInitialize (core::ptr::null_mut ());
  let mut task_bar_list: *mut ITaskbarList3 = core::ptr::null_mut ();
  let r: HRESULT = CoCreateInstance (&raw const CLSID_TaskbarList,
				     core::ptr::null_mut (),
				     CLSCTX_INPROC_SERVER,
				     &raw const IID_ITaskbarList3,
				     &raw mut task_bar_list as *mut *mut c_void);
  if r != S_OK
    { return Qnil (); }

  if ! NILP (badge) && STRINGP (badge)
      && STRINGP (background) && STRINGP (foreground)
    {
      let mut bg_rgb: COLORREF;
      let mut fg_rgb: COLORREF;
      let mut r: c_ushort = 0;
      let mut g: c_ushort = 0;
      let mut b: c_ushort = 0;
      if parse_color_spec (SSDATA (background) as *const c_char, &raw mut r, &raw mut g, &raw mut b)
	{ bg_rgb = RGB! (r, g, b); }
      else
	{ return Qnil (); }
      if parse_color_spec (SSDATA (foreground) as *const c_char, &raw mut r, &raw mut g, &raw mut b)
	{ fg_rgb = RGB! (r, g, b); }
      else
	{ return Qnil (); }

      /* Prepare a string for drawing and as alt-text.  */
      let badge_utf8: Lisp_Object = ENCODE_UTF_8 (badge);
      let wide_len: c_int = (pMultiByteToWideChar.unwrap ()) (CP_UTF8, 0,
							      SSDATA (badge_utf8) as *const c_char,
							      -1, core::ptr::null_mut (), 0);
      let badge_w: *mut c_ushort
	= sa_stack_alloc ((wide_len + 1) as usize * core::mem::size_of::<c_ushort> ())
	    as *mut c_ushort;
      (pMultiByteToWideChar.unwrap ()) (CP_UTF8, 0, SSDATA (badge_utf8) as *const c_char, -1,
					badge_w as LPWSTR, wide_len);

      /* Use the small icon size Windows suggests to not hard code 16x16.  */
      let icon_width: c_int = GetSystemMetrics (SM_CXSMICON);
      let icon_height: c_int = GetSystemMetrics (SM_CXSMICON);

      let hwnd_dc: HDC = GetDC (hwnd);
      let dc: HDC = CreateCompatibleDC (hwnd_dc);

      let mut bi: BITMAPV5HEADER = core::mem::zeroed ();
      memset (&raw mut bi as *mut c_void, 0, core::mem::size_of_val (&bi));
      bi.bV5Size = core::mem::size_of_val (&bi) as DWORD;
      bi.bV5Width = icon_width;
      bi.bV5Height = -icon_height; /* Negative for a top-down DIB.  */
      bi.bV5Planes = 1;
      bi.bV5BitCount = 32;
      bi.bV5Compression = BI_BITFIELDS;  /* Enable the masks below.  */
      bi.bV5RedMask   =  0x00FF0000;
      bi.bV5GreenMask =  0x0000FF00;
      bi.bV5BlueMask  =  0x000000FF;
      bi.bV5AlphaMask =  0xFF000000;

      let mut bitmap_pixels: *mut DWORD = core::ptr::null_mut ();
      let bitmap: HBITMAP = CreateDIBSection (dc, &raw const bi as *const BITMAPINFO,
					      DIB_RGB_COLORS,
					      &raw mut bitmap_pixels as *mut *mut c_void,
					      core::ptr::null_mut (), 0);
      let old_bitmap: HGDIOBJ = SelectObject (dc, bitmap);

      /* Draw a circle filled with bg.  */
      let bg_brush: HBRUSH = CreateSolidBrush (bg_rgb);
      let old_brush: HGDIOBJ = SelectObject (dc, bg_brush);
      Ellipse (dc, 0, 0, icon_width, icon_height);
      SelectObject (dc, old_brush);
      DeleteObject (bg_brush);

      /* Derive a font scaled to fit the icon.  First find the system's
	 base font.  Then scale it to fit icon_height.  */
      let base_font: HFONT;
      let mut clean_up_base_font: BOOL = 0;
      if system_parameters_info_w_fn.is_some ()
	{
	  let mut ncm: NONCLIENTMETRICS = core::mem::zeroed ();
	  memset (&raw mut ncm as *mut c_void, 0, core::mem::size_of_val (&ncm));
	  ncm.cbSize = core::mem::size_of_val (&ncm) as UINT;
	  SystemParametersInfo (SPI_GETNONCLIENTMETRICS, core::mem::size_of_val (&ncm) as UINT,
				&raw mut ncm as *mut c_void, 0);
	  base_font = CreateFontIndirect (&raw const ncm.lfSmCaptionFont);
	  clean_up_base_font = 1;
	}
      else
	{ base_font = GetStockObject (DEFAULT_GUI_FONT) as HFONT; }
      if clean_up_base_font != 0
	{ DeleteObject (base_font); }

      let mut lf: LOGFONT = core::mem::zeroed ();
      GetObject (base_font, core::mem::size_of_val (&lf) as c_int, &raw mut lf as *mut c_void);
      lf.lfWeight = FW_BOLD as LONG;
      lf.lfOutPrecision = OUT_OUTLINE_PRECIS;
      /* ClearType quality needs opqaue, but we draw transparent.  */
      lf.lfQuality = ANTIALIASED_QUALITY;
      /* Negative lfHeight indicates pixel units vs. positive in points.
	 Use the LOGPIXELSY px/in of the primary monitor.  */
      lf.lfHeight = -MulDiv (icon_height / 2, /* Fit ~3 chars.  */
			     72,
			     GetDeviceCaps (GetDC (core::ptr::null_mut ()),
					    LOGPIXELSY)) as LONG;
      /* Ensure lfHeight pixel interpretation.  */
      let old_map_mode: c_int = SetMapMode (dc, MM_TEXT);
      let scaled_font: HFONT = CreateFontIndirect (&raw const lf);
      let old_font: HGDIOBJ = SelectObject (dc, scaled_font);
      SetMapMode (dc, old_map_mode);

      /* Draw badge text.  */
      SetBkMode (dc, TRANSPARENT);
      SetTextColor (dc, fg_rgb);
      let mut rect: RECT = core::mem::zeroed ();
      rect.left = 0;
      rect.top = 0;
      rect.right = icon_width;
      rect.bottom = icon_height;
      DrawText (dc, SSDATA (badge_utf8) as *const c_char,
		-1, /* Indicate null-terminated string.  */
		&raw mut rect,
		DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOCLIP);
      SelectObject (dc, old_font);
      DeleteObject (scaled_font);

      /* Make the circle and its text opaque by setting the alpha
	 channel on each pixel falling within the circle.  */
      let circle_center_x: c_int = icon_width / 2;
      let circle_center_y: c_int = icon_height / 2;
      let circle_radius: c_int = (if icon_width < icon_height
				  { icon_width }
				  else { icon_height }) / 2 - 2;
      let circle_radius_sq: c_int = circle_radius * circle_radius;
      let mut pixel: *mut DWORD;
      let mut y: c_int = 0;
      while y < icon_height
	{
	  let mut x: c_int = 0;
	  while x < icon_width
	    {
	      let dx: c_int = x - circle_center_x;
	      let dy: c_int = y - circle_center_y;
	      if dx * dx + dy * dy <= circle_radius_sq
		{
		  pixel = bitmap_pixels.offset ((y * icon_width + x) as isize);
		  *pixel |= 0xff000000; /* Flip the 0xAARRGGBB alpha channel.  */
		}
	      x += 1;
	    }
	  y += 1;
	}

      /* Dummy monochrome bitmap mask, ignored when the color bitmap has
	 an alpha channel, but needed to satisfy CreateIconIndirect.  */
      let mask_bitmap: HBITMAP = CreateBitmap (icon_width, icon_height, 1, 1, core::ptr::null ());

      /* https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createiconindirect
	 hbmMask and hbmColor members of the ICONINFO structure should
	 not already be selected into a device context.  */
      SelectObject (dc, old_bitmap);

      let mut icon_info: ICONINFO = core::mem::zeroed ();
      memset (&raw mut icon_info as *mut c_void, 0, core::mem::size_of_val (&icon_info));
      icon_info.fIcon = 1;
      icon_info.hbmMask = mask_bitmap;
      icon_info.hbmColor = bitmap;

      let icon: HICON = CreateIconIndirect (&raw mut icon_info);
      ((*((*task_bar_list).lpVtbl)).SetOverlayIcon.unwrap ()) (task_bar_list, hwnd, icon, badge_w);

      DestroyIcon (icon);
      DeleteObject (mask_bitmap);
      DeleteObject (bitmap);
      DeleteDC (dc);
      ReleaseDC (hwnd, hwnd_dc);
    }
  else
    { ((*((*task_bar_list).lpVtbl)).SetOverlayIcon.unwrap ()) (task_bar_list, hwnd,
							      core::ptr::null_mut (),
							      core::ptr::null ()); }

  ((*((*task_bar_list).lpVtbl)).Release.unwrap ()) (task_bar_list);
  return Qt ();
});

/* C 11247-11348 closes the badge/taskbar `#ifdef WINDOWSNT`: C 11247-11295's
   `w32-request-user-attention` and C 11297-11346's `w32-progress-indicator`, with C 11348's
   `#endif` having no item of its own.  The System Sleep Support banner (C 11350-11352) and its
   first `typedef` (C 11354) open the following slice, so the range table's remainder starts at
   11350.  */

/// C 11255-11295: `w32-request-user-attention` - C 11262-11264's `flash_window_ex_fn` NULL test
/// (the pointer `globals_of_w32fns`, C 12460-12475, loads), C 11266-11275's selected-frame
/// resolution (the same shape `w32_badge` carries), C 11277-11283's `FLASHWINFO` field run with
/// C's `sizeof(flash_info)`, and C 11284-11292's three `dwFlags` arms (`Qinformational` ->
/// `FLASHW_TRAY | FLASHW_TIMERNOFG`, `Qcritical` -> `FLASHW_ALL | FLASHW_TIMERNOFG`, otherwise
/// `FLASHW_STOP`), then C 11294's call and `Qt`.  Both `Q` symbols are the mirror's
/// (`include/globals.rs` 1052 and 601).
/// C: DEFUN ("w32-request-user-attention", Fw32_request_user_attention,
/// C:        Sw32_request_user_attention, 1, 1, 0,
/// C:        doc: /* Flash the selected frame's taskbar icon and/or its window. … */)
/// C:   (Lisp_Object urgency)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-request-user-attention", Fw32_request_user_attention, Sw32_request_user_attention,
	1, 1, 0,
	doc: "Flash the selected frame's taskbar icon and/or its window.\nIf URGENCY is nil, cancel the request, if any.  If URGENCY is the symbol\n`informational', flash the taskbar icon.  If URGENCY is the symbol\n`critical', flash the taskbar icon and the frame.  Windows stops\nflashing if the user focuses the frame.  Do nothing if Windows does not\nsupport FlashWindowEx and return nil, otherwise return t.  Do nothing if\nthe frame is not (yet) associated with a window handle.",
	(urgency: Lisp_Object),
	{
  if flash_window_ex_fn.is_none ()
    { return Qnil (); }

  let sf: *mut frame = SELECTED_FRAME ();
  let mut hwnd: HWND = core::ptr::null_mut ();

  if FRAME_W32_P (sf) && FRAME_LIVE_P (sf)
    { hwnd = FRAME_W32_WINDOW! (sf); }

  if hwnd.is_null ()
    { return Qnil (); }

  let mut flash_info: FLASHWINFO = core::mem::zeroed ();
  flash_info.cbSize = core::mem::size_of_val (&flash_info) as UINT;
  flash_info.uCount = 0;
  flash_info.dwTimeout = 0;
  flash_info.hwnd = hwnd;
  if ! NILP (urgency) && SYMBOLP (urgency)
    {
      /* The intended caller, 'system-taskbar-attention', has an
	 optional timer to clear the attention indicator so this will
	 flash until cleared via the timer, or the window comes to the
	 foreground.  For informational attention, flash the tray icon.
	 For critical attention, flash the tray icon and the window.  */
      if EQ (urgency, Qinformational ())
	{ flash_info.dwFlags = FLASHW_TRAY | FLASHW_TIMERNOFG; }
      else if EQ (urgency, Qcritical ())
	{ flash_info.dwFlags = FLASHW_ALL | FLASHW_TIMERNOFG; }
    }
  else
    { flash_info.dwFlags = FLASHW_STOP; }

  (flash_window_ex_fn.unwrap ()) (&raw mut flash_info as *mut c_void);
  return Qt ();
});

/// C 11303-11346: `w32-progress-indicator` - the same selected-frame resolution as the item above,
/// C 11323-11330's `CoInitialize`/`CoCreateInstance` pair, C 11332-11335's scaling (`ULONGLONG`
/// zero, the `FLOATP` test and C's `(ULONGLONG) (100.0 * XFLOAT_DATA (progress))` cast), and
/// C 11336-11344's two arms (`TBPF_NORMAL` + `SetProgressValue (…, adj_progress, 100)` against
/// `TBPF_NOPROGRESS`), then the `Release` and `Qt`.
/// C: DEFUN ("w32-progress-indicator", Fw32_progress_indicator, Sw32_progress_indicator,
/// C:        1, 1, 0, doc: /* Show a progress bar on the selected frame's taskbar icon. … */)
/// C:   (Lisp_Object progress)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
DEFUN! ("w32-progress-indicator", Fw32_progress_indicator, Sw32_progress_indicator,
	1, 1, 0,
	doc: "Show a progress bar on the selected frame's taskbar icon.\nPROGRESS is a float in the range 0.0 to 1.0.  If PROGRESS is nil, remove\nthe progress indicator.  Do nothing if Windows does not support the\nITaskbarList3 interface and return nil, otherwise return t.  Do nothing\nif the selected frame is not (yet) associated with a window handle",
	(progress: Lisp_Object),
	{
  let sf: *mut frame = SELECTED_FRAME ();
  let mut hwnd: HWND = core::ptr::null_mut ();

  if FRAME_W32_P (sf) && FRAME_LIVE_P (sf)
    { hwnd = FRAME_W32_WINDOW! (sf); }

  if hwnd.is_null ()
    { return Qnil (); }

  CoInitialize (core::ptr::null_mut ());
  let mut task_bar_list: *mut ITaskbarList3 = core::ptr::null_mut ();
  let r: HRESULT = CoCreateInstance (&raw const CLSID_TaskbarList,
				     core::ptr::null_mut (),
				     CLSCTX_INPROC_SERVER,
				     &raw const IID_ITaskbarList3,
				     &raw mut task_bar_list as *mut *mut c_void);
  if r != S_OK
    { return Qnil (); }

  /* Scale task bar progress from 0.0-1.0 to 0-100.  */
  let mut adj_progress: ULONGLONG = 0;
  if ! NILP (progress) && FLOATP (progress)
    { adj_progress = (100.0 * XFLOAT_DATA (progress)) as ULONGLONG; }
  if adj_progress > 0
    {
      ((*((*task_bar_list).lpVtbl)).SetProgressState.unwrap ()) (task_bar_list,
								hwnd, TBPF_NORMAL);
      ((*((*task_bar_list).lpVtbl)).SetProgressValue.unwrap ()) (task_bar_list, hwnd,
								 adj_progress, 100);
    }
  else
    {
      ((*((*task_bar_list).lpVtbl)).SetProgressState.unwrap ()) (task_bar_list,
								 hwnd, TBPF_NOPROGRESS);
    }

  ((*((*task_bar_list).lpVtbl)).Release.unwrap ()) (task_bar_list);
  return Qt ();
});

/// C 11277-11283's `FLASHWINFO` (the record C declares inside the item above) and C 11289-11292's
/// four `FLASHW_*` flags, corroborated against the Win32 metadata projection.
/// C: typedef struct { UINT cbSize; HWND hwnd; DWORD dwFlags; UINT uCount; DWORD dwTimeout; }
/// C: FLASHWINFO, *PFLASHWINFO;
/// C: #define FLASHW_STOP 0 / #define FLASHW_TRAY 0x00000002
/// C: #define FLASHW_ALL (FLASHW_CAPTION | FLASHW_TRAY) / #define FLASHW_TIMERNOFG 0x0000000C
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLASHWINFO
{
  pub cbSize: UINT,
  pub hwnd: HWND,
  pub dwFlags: DWORD,
  pub uCount: UINT,
  pub dwTimeout: DWORD,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FLASHW_STOP: UINT = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FLASHW_TRAY: UINT = 0x00000002;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FLASHW_ALL: UINT = 0x00000003;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const FLASHW_TIMERNOFG: UINT = 0x0000000C;

/// C 11338/11343's two `TBPFLAG` members (`TBPF_NORMAL` 0x2 and `TBPF_NOPROGRESS` 0x0), the SDK's
/// enum read as the `c_int` this unit's `ITaskbarList3Vtbl.SetProgressState` takes.
/// C: typedef enum TBPFLAG { TBPF_NOPROGRESS = 0, TBPF_INDETERMINATE = 0x1, TBPF_NORMAL = 0x2,
/// C: TBPF_ERROR = 0x4, TBPF_PAUSED = 0x8 } TBPFLAG;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const TBPF_NORMAL: c_int = 0x2;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const TBPF_NOPROGRESS: c_int = 0x0;

/* C 11350-11477 is the System Sleep Support group: C 11350-11352's banner, C 11354-11355's
   `SetThreadExecutionState_Proc` with its pointer, C 11357-11358's two counters, the three
   `DEFUN`s (C 11360, C 11391, C 11414), C 11424-11449's `PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE`
   with `sleep_notification_callback`, C 11451-11455's `RegisterSuspendResumeNotification_Proc`,
   its pointer and `sleep_notification_handle`, C 11457-11460's
   `MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`, and C 11462-11477's
   `w32_register_for_sleep_notifications`.  C 11479's Exporting frames banner opens the next
   slice, so the range table's remainder starts there.  */

/*
/***********************************************************************
		     System Sleep Support
 ***********************************************************************/
*/

/// C 11354-11358: the four file-scope objects the three items below read - the dynamically loaded
/// `SetThreadExecutionState` pointer (`globals_of_w32fns`, C 12325-12327, is what loads it) and
/// C's two `static unsigned int` counters.
/// C: typedef ULONG (WINAPI * SetThreadExecutionState_Proc) (IN ULONG);
/// C: static SetThreadExecutionState_Proc SetThreadExecutionState_fn = NULL;
/// C: static unsigned int sleep_block_id = 0;
/// C: static unsigned int sleep_block_count = 0;
#[cfg (feature = "have_ntgui")]
pub type SetThreadExecutionState_Proc = unsafe extern "system" fn (ULONG) -> ULONG;
#[cfg (feature = "have_ntgui")]
static mut SetThreadExecutionState_fn: Option<SetThreadExecutionState_Proc> = None;
#[cfg (feature = "have_ntgui")]
static mut sleep_block_id: c_uint = 0;
#[cfg (feature = "have_ntgui")]
static mut sleep_block_count: c_uint = 0;

/// C 11378/11408's `SetThreadExecutionState` (`winbase.h`; the guard above tests the *pointer*,
/// which is C's own text), with C 11368-11372's `EXECUTION_STATE` and its three `ES_*` values.
/// C: EXECUTION_STATE WINAPI SetThreadExecutionState (EXECUTION_STATE esFlags);
/// C: #define ES_SYSTEM_REQUIRED 0x00000001 / #define ES_DISPLAY_REQUIRED 0x00000002
/// C: #define ES_CONTINUOUS 0x80000000
#[cfg (feature = "have_ntgui")]
pub type EXECUTION_STATE = DWORD;
#[cfg (feature = "have_ntgui")]
pub const ES_SYSTEM_REQUIRED: EXECUTION_STATE = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const ES_DISPLAY_REQUIRED: EXECUTION_STATE = 0x00000002;
#[cfg (feature = "have_ntgui")]
pub const ES_CONTINUOUS: EXECUTION_STATE = 0x80000000;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn SetThreadExecutionState (esFlags: EXECUTION_STATE) -> EXECUTION_STATE;
}

/// C 11362-11389: `w32-block-system-sleep` - the pointer guard, C 11366's comment, C 11367's
/// `ES_SYSTEM_REQUIRED | ES_CONTINUOUS` initialiser, C 11368-11369's `NILP` test, and
/// C 11371-11387's two arms (`++sleep_block_count`/`++sleep_block_id` as two statements and the
/// synthesised token, against the `Qnil` the failed call returns).
/// C: DEFUN ("w32-block-system-sleep", Fw32_block_system_sleep, Sw32_block_system_sleep,
/// C:        1, 1, 0, doc: /* Block system idle sleep. … */)
/// C:   (Lisp_Object allow_display_sleep)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-block-system-sleep", Fw32_block_system_sleep, Sw32_block_system_sleep,
	1, 1, 0,
	doc: "Block system idle sleep.\nIf ALLOW-DISPLAY-SLEEP is non-nil, block the screen from sleeping.\nReturn a token to unblock this block using `w32-unblock-system-sleep',\nor nil if the block fails.",
	(allow_display_sleep: Lisp_Object),
	{
  if SetThreadExecutionState_fn.is_none ()
    { return Qnil (); }

  /* ES_CONTINUOUS keeps the state until cleared.  */
  let mut new_state: EXECUTION_STATE = ES_SYSTEM_REQUIRED | ES_CONTINUOUS;
  if NILP (allow_display_sleep)
    { new_state |= ES_DISPLAY_REQUIRED; }

  if SetThreadExecutionState (new_state) == 0
    { return Qnil (); }
  else
    {
      /* One more block and next id.  */
      sleep_block_count += 1;
      sleep_block_id += 1;

      /* Synthesize a token.  */
      return make_fixnum (sleep_block_id as EMACS_INT);
    }
});

/// C 11393-11412: `w32-unblock-system-sleep` - the pointer guard, C 11396-11397's comment with
/// the empty-count test, and C 11401-11410's `--sleep_block_count == 0` pre-decrement (a block
/// expression, the `pgtk_hide_hourglass` device) paired with the `ES_CONTINUOUS` clear.
/// C: DEFUN ("w32-unblock-system-sleep", Fw32_unblock_system_sleep, Sw32_unblock_system_sleep,
/// C:        0, 0, 0, doc: /* Unblock system idle sleep. … */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-unblock-system-sleep", Fw32_unblock_system_sleep, Sw32_unblock_system_sleep,
	0, 0, 0,
	doc: "Unblock system idle sleep.\nReturn non-nil if the TOKEN block was unblocked.",
	(),
	{
  if SetThreadExecutionState_fn.is_none ()
    { return Qnil (); }

  /* No blocks to unblock.  */
  if sleep_block_count == 0
    { return Qnil (); }

  /* One fewer block.  */
  if { sleep_block_count -= 1; sleep_block_count == 0 }
      && SetThreadExecutionState (ES_CONTINUOUS) == 0
    { return Qnil (); }
  else
    { return Qt (); }
});

/// C 11416-11422: `w32-system-sleep-block-count` - C's one-line body over the counter.
/// C: DEFUN ("w32-system-sleep-block-count", Fw32_system_sleep_block_count,
/// C:        Sw32_system_sleep_block_count, 0, 0, 0,
/// C:        doc: /* Return the w32 sleep block count.  */)
/// C:   (void)
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-system-sleep-block-count", Fw32_system_sleep_block_count,
	Sw32_system_sleep_block_count,
	0, 0, 0,
	doc: "Return the w32 sleep block count.",
	(),
	{
  return make_fixnum (sleep_block_count as EMACS_INT);
});

/// `struct input_event`'s two bit fields - `termhooks.h` 373-412's `enum event_kind kind :
/// EVENT_KIND_WIDTH` and the `part` that shares the same unit (`include/termhooks.rs` carries
/// `bits`, `INPUT_EVENT_KIND_MASK` and `INPUT_EVENT_PART_SHIFT`) - are C members the port reaches
/// through a per-unit macro, because `macro_rules!` is textual and each unit's copy belongs to
/// that unit (PORTING.md P-80's member-named shape; the `src/w32inevt.rs` 644 copy is this one's
/// twin).  This unit uses the `kind` pair (C 11432's `ie.kind = SLEEP_EVENT;`).
#[cfg (feature = "have_ntgui")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/// C 11424-11449: the power-notification callback - C 11424-11425's
/// `PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE`, then C 11427-11429's `static ULONG CALLBACK ALIGN_STACK`
/// definition whose body is C 11431-11432's `input_event` (the file's first use of the mirror's
/// `EVENT_INIT`, the `src/dbusbind.rs` 2530 device) and C 11434-11446's three `case` arms.
/// `ALIGN_STACK` is the mirror's empty macro (a Rust item cannot express C's
/// `__attribute__((force_align_arg_pointer))`; `include/w32term.rs` 63-67 records that).
/// C: typedef ULONG (CALLBACK *PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE)
/// C:   (PVOID Context, ULONG Type, PVOID Setting);
/// C: static ULONG CALLBACK ALIGN_STACK
/// C: sleep_notification_callback (PVOID _Context, ULONG Type, PVOID _Setting)
#[cfg (feature = "have_ntgui")]
pub type PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE
  = unsafe extern "system" fn (Context: *mut c_void, Type: ULONG, Setting: *mut c_void) -> ULONG;
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn sleep_notification_callback (_Context: *mut c_void, Type: ULONG,
						       _Setting: *mut c_void) -> ULONG
{
  ALIGN_STACK! ();
  let mut ie: input_event = core::mem::zeroed ();
  EVENT_INIT (&raw mut ie);
  input_event_bits! (kind, &raw mut ie, SLEEP_EVENT);

  match Type
    {
      PBT_APMRESUMEAUTOMATIC =>
	{
	  /* Ignore this event.  No user is present.  */
	}
      PBT_APMSUSPEND =>
	{
	  ie.arg = list1 (Qpre_sleep ());
	  kbd_buffer_store_event (&raw mut ie as *mut c_void);
	}
      PBT_APMRESUMESUSPEND =>
	{
	  ie.arg = list1 (Qpost_wake ());
	  kbd_buffer_store_event (&raw mut ie as *mut c_void);
	}
      _ => {}
    }
  return 0;
}

/// C 11427-11446's three `PBT_*` case labels (`powrprof.h`; the third is the resume) - the
/// `switch (Type)` above is the port's `match`, whose trailing wildcard is C's absent `default`
/// (PORTING.md 2.2, the `add_tray_notification` reading), recorded in this file's notes rows.
/// C: #define PBT_APMRESUMEAUTOMATIC 0x0002 / #define PBT_APMSUSPEND 0x0004
/// C: #define PBT_APMRESUMESUSPEND 0x0008
#[cfg (feature = "have_ntgui")]
pub const PBT_APMRESUMEAUTOMATIC: ULONG = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const PBT_APMSUSPEND: ULONG = 0x0004;
#[cfg (feature = "have_ntgui")]
pub const PBT_APMRESUMESUSPEND: ULONG = 0x0008;

/// C 11451-11477: the registration half - C 11451-11453's `RegisterSuspendResumeNotification_Proc`
/// with its pointer, C 11455's handle, C 11457-11460's `MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`
/// (the struct and its `P…` alias; only the value type is used) and C 11462-11477's
/// `w32_register_for_sleep_notifications` with C 11463-11464's comment, the `params` field pair
/// and C 11473-11474's call with C 11457's `DEVICE_NOTIFY_CALLBACK` value 2.
/// C: typedef HPOWERNOTIFY (WINAPI * RegisterSuspendResumeNotification_Proc) (IN HANDLE, IN DWORD);
/// C: static RegisterSuspendResumeNotification_Proc RegisterSuspendResumeNotification_fn = NULL;
/// C: static HPOWERNOTIFY sleep_notification_handle = 0;
/// C: typedef struct _MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
/// C:   PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE Callback; PVOID Context;
/// C: } MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS, *PMY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;
/// C: void w32_register_for_sleep_notifications (void)
#[cfg (feature = "have_ntgui")]
pub type HPOWERNOTIFY = HANDLE;
#[cfg (feature = "have_ntgui")]
pub type RegisterSuspendResumeNotification_Proc
  = unsafe extern "system" fn (HANDLE, DWORD) -> HPOWERNOTIFY;
#[cfg (feature = "have_ntgui")]
static mut RegisterSuspendResumeNotification_fn: Option<RegisterSuspendResumeNotification_Proc>
  = None;
#[cfg (feature = "have_ntgui")]
static mut sleep_notification_handle: HPOWERNOTIFY = core::ptr::null_mut ();
#[cfg (feature = "have_ntgui")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS
{
  pub Callback: PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE,
  pub Context: *mut c_void,
}
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_register_for_sleep_notifications ()
{
  /* PowerRegisterSuspendResumeNotification is not a user-space call so
     we use RegisterSuspendResumeNotification.  */
  if RegisterSuspendResumeNotification_fn.is_some ()
    {
      let mut params: MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS = core::mem::zeroed ();
      params.Callback = sleep_notification_callback;
      params.Context = core::ptr::null_mut ();

      /* DEVICE_NOTIFY_CALLBACK = 2  */
      sleep_notification_handle
	= (RegisterSuspendResumeNotification_fn.unwrap ()) (&raw mut params as *mut c_void, 2);
    }
}

/* C 11479-12488's text is not in the port yet (the file's remainder), but C 10112's
   `w32_console_toggle_lock_key` is: C 9302 above calls it, and Rust has no "declare then define"
   pair inside one file (PORTING.md 0.1-12 / P-91 ① - "land whichever the slice needs first").
   It therefore stands below the slices that insert their text above it, rather than at its C
   position, and the ledger's `## C line ranges` table carries the landed rows (`9147-9337`,
   `9339-9752`, `10112-10138`) around the remainder.  Later slices of this lane insert their own
   text *above* this block, so the file's item order still equals C's order (P-91 ②: Rust items
   are order-independent).  */

/// C 10112-10138: `int w32_console_toggle_lock_key (int vk_code, Lisp_Object new_state)` - the
/// Cygwin/console fallback `w32-toggle-lock-key` uses when `dwWindowsThreadId` is zero.  C 10115's
/// `GetKeyState (vk_code) & 1` (the C 3775 shape), C 10117-10119's `NILP`/`FIXNUMP`/`XUFIXNUM` test,
/// C 10121-10123's `#ifdef WINDOWSNT` assignment to `faked_key` (`w32inevt.c`'s object, declared in
/// this file at C 312), C 10125-10133's three `keybd_event` taps (`KEYEVENTF_EXTENDEDKEY`, the two
/// `KEYEVENTF_KEYUP`s around a press) and C 10134's `cur_state = !cur_state;` (C's `!` on an `int`,
/// the `cur_state` reading at C 3792).
/// C: int
/// C: w32_console_toggle_lock_key (int vk_code, Lisp_Object new_state)
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_console_toggle_lock_key (vk_code: c_int, new_state: Lisp_Object)
  -> c_int
{
  let mut cur_state: c_int = GetKeyState (vk_code) as c_int & 1;

  if NILP (new_state)
      || (FIXNUMP (new_state)
	  && (XUFIXNUM (new_state) & 1) as c_int != cur_state)
    {
      #[cfg (feature = "windowsnt")]
      { faked_key = vk_code; }

      keybd_event (vk_code as BYTE,
		   MapVirtualKey (vk_code as UINT, 0) as BYTE,
		   KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
      keybd_event (vk_code as BYTE,
		   MapVirtualKey (vk_code as UINT, 0) as BYTE,
		   KEYEVENTF_EXTENDEDKEY | 0, 0);
      keybd_event (vk_code as BYTE,
		   MapVirtualKey (vk_code as UINT, 0) as BYTE,
		   KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
      cur_state = if cur_state == 0 { 1 } else { 0 };
    }

  return cur_state;
}
