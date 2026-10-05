// femacs: port of src/w32inevt.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32inevt.o`
// joins `W32_OBJ` only in `configure.ac` 3051 - the non-Cygwin arm of the split inside the
// `HAVE_W32` test (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` - and
// `docs/baseline/frozen-build-sources.txt` lists no `w32inevt.o` for this target, so
// `extract-abi.py`'s checklist for the file is empty (P-86 ②) and the ledger
// `docs/ports/w32inevt.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*
//     (P-86 ⑧: an ungated `include!` breaks the frozen compile at once).  It is `WINDOWSNT`, not
//     `HAVE_NTGUI`: `HAVE_NTGUI` is on for *both* the Cygwin and the native Windows GUI build
//     (3024), while `w32inevt.o` is added to `W32_OBJ` only in the non-Cygwin arm (3051) - the
//     build whose `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The units of that same arm
//     (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`, `src/w32console.rs`) already landed with
//     this gate, so this one does too;
//   * C 30/33's two `#ifndef` guards are "define it if the SDK has not", so the port carries the
//     two constants with C's own values and no `cfg` (the `include/w32term.rs` reading of its nine);
//   * C 120-130's `#if 0` is dead in every configuration; C's own text is kept as a comment (the
//     `src/cm.rs` 86 shape);
//   * C 644/733's `#if HAVE_W32NOTIFY` / `#else` pair is `feature = "have_w32notify"`; the macro
//     is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1675, so C's `#else` arm is the
//     frozen one and **both arms are carried** (P-51).
//
// **Acquisition note - this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wincon.h`), so there is **no
// `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
// measurement: every record, every value and every prototype of the `<windows.h>` stand-in section
// below is `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are
// owed.  What the round does carry is the frozen `build-tu` reading, the machine-checked coverage
// table, the `#if` verdicts, a per-item C citation, the per-file error census of the `windowsnt` +
// `have_ntgui` + `have_window_system` arm, and a **mutation test** over this unit's own lines.
//
// `docs/ports/w32inevt.md` carries the C line ranges, the acceptance readings, every deviation
// (the dropped `extern` declarations, the `<windows.h>` stand-in surface, the two collision rows
// the mirrors leave, the per-unit bit-field accessors, the lock-key `goto`'s structure) and the
// items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-18 (with C 18's separator) is the file's opening ownership/title block, dropped (PORTING.md
   0.1-19); the port's first line is the revision line above.

   C 20-23 is the file's own second comment - the two authors and the note that the file came from
   ntkbd.c.  C's words are carried:  */

/*
   Drew Bliss                   01-Oct-93
     Adapted from ntkbd.c by Tim Fleehart
*/

/* C 26-44: the include graph.  `w32inevt.o` is built only by the native Windows GUI configuration,
   so the whole graph sits under the file gate (P-86 ⑧: with the includes outside it the frozen
   compile immediately eats `E0428`s).  C 28's `<windows.h>` has no mirror in this tree (P-88 ①:
   the declaration site is C's own `#include` line, and no file is added under `femacs/src/include/`)
   so the names this file takes from it stand in the section below; the other edges are the
   mirrors, whose own nested `include!`s `xtask` expands and dedupes (PORTING.md 0.1-4).  */
#[cfg (feature = "windowsnt")]
include!("include/config.rs");                  // C 26: `#include <config.h>`
/* C 26's AH_BOTTOM: `#include <conf_post.h>` (`DebPrint`, the `eassert` guards).  */
#[cfg (feature = "windowsnt")]
include!("include/conf_post.rs");
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");            // C 27: `#include <stdio.h>`

/* ----------------------------------------------------------------------------------------------
   C 28: `#include <windows.h>`.

   The names this file takes from it stand here, each with the SDK's own text above it.  The
   spellings repeat `src/w32console.rs`'s and `src/w32.rs`'s sections so that a Windows translation
   unit sees one set of names: `DWORD` is `u32`, `WORD` is `c_ushort`, `BOOL` is C's `int`,
   `HANDLE` is `void *`, and `WINAPI` is the `extern "system"` ABI Rust spells `__stdcall` with.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar, record, constant and prototype below is **UNVERIFIED** - its size, offsets and values are
   the documented ones and the layout assertions are owed (the ledger's `## Owed`).  The values are
   `<windef.h>`/`<wincon.h>`/`<winuser.h>`'s; `wchar_t` is the Windows 16-bit one, which the tree
   spells `c_ushort` (`include/w32.rs` 31's reading of the same type).
   ---------------------------------------------------------------------------------------------- */

/// C 28's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned long ULONG;
/// C: typedef unsigned int UINT; typedef int BOOL; typedef short SHORT; typedef char CHAR;
/// C: typedef unsigned short wchar_t (the Windows 16-bit one);
#[cfg (feature = "windowsnt")]
pub type BYTE = u8;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type WCHAR = c_ushort;
#[cfg (feature = "windowsnt")]
pub type wchar_t = c_ushort;
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
pub type CHAR = c_char;
/// C's handle names are all `void *` at the ABI (the `include/w32gui.rs`/`src/w32.rs` reading), so
/// the implicit conversions at the call sites need no cast.
/// C: typedef void *HANDLE; typedef void *LPVOID; typedef DWORD *LPDWORD; typedef BYTE *LPBYTE;
/// C: typedef const char *LPCCH; typedef wchar_t *LPWSTR; typedef ULONG_PTR *PULONG_PTR;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;
#[cfg (feature = "windowsnt")]
pub type LPBYTE = *mut u8;
#[cfg (feature = "windowsnt")]
pub type LPCCH = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPWSTR = *mut wchar_t;
#[cfg (feature = "windowsnt")]
pub type ULONG_PTR = usize;

/// C 28's `<windows.h>` (continued): the console records (`<wincon.h>`).
/// C: typedef struct _COORD { SHORT X; SHORT Y; } COORD, *PCOORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COORD
{
  pub X: SHORT,
  pub Y: SHORT,
}
/// C: typedef struct _SMALL_RECT { SHORT Left; SHORT Top; SHORT Right; SHORT Bottom; } SMALL_RECT;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SMALL_RECT
{
  pub Left: SHORT,
  pub Top: SHORT,
  pub Right: SHORT,
  pub Bottom: SHORT,
}
/// C: typedef struct _KEY_EVENT_RECORD { BOOL bKeyDown; WORD wRepeatCount; WORD wVirtualKeyCode;
/// C: WORD wVirtualScanCode; union { WCHAR UnicodeChar; CHAR AsciiChar; } uChar;
/// C: DWORD dwControlKeyState; } KEY_EVENT_RECORD;
/// The anonymous union member keeps C's name `uChar`; the union itself is named
/// `<struct>_<member>` (the P-67 convention `include/dispextern.rs` uses for `glyph_slice`).
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_uChar
{
  pub UnicodeChar: WCHAR,
  pub AsciiChar: CHAR,
}
#[cfg (feature = "windowsnt")]
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
/// C: typedef struct _MOUSE_EVENT_RECORD { COORD dwMousePosition; DWORD dwButtonState; DWORD
/// C: dwControlKeyState; DWORD dwEventFlags; } MOUSE_EVENT_RECORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MOUSE_EVENT_RECORD
{
  pub dwMousePosition: COORD,
  pub dwButtonState: DWORD,
  pub dwControlKeyState: DWORD,
  pub dwEventFlags: DWORD,
}
/// C: typedef struct _WINDOW_BUFFER_SIZE_RECORD { COORD dwSize; } WINDOW_BUFFER_SIZE_RECORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOW_BUFFER_SIZE_RECORD
{
  pub dwSize: COORD,
}
/// C: typedef struct _MENU_EVENT_RECORD { UINT dwCommandId; } MENU_EVENT_RECORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MENU_EVENT_RECORD
{
  pub dwCommandId: UINT,
}
/// C: typedef struct _FOCUS_EVENT_RECORD { BOOL bSetFocus; } FOCUS_EVENT_RECORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FOCUS_EVENT_RECORD
{
  pub bSetFocus: BOOL,
}
/// C: typedef struct _INPUT_RECORD { WORD EventType; union { KEY_EVENT_RECORD KeyEvent;
/// C: MOUSE_EVENT_RECORD MouseEvent; WINDOW_BUFFER_SIZE_RECORD WindowBufferSizeEvent;
/// C: MENU_EVENT_RECORD MenuEvent; FOCUS_EVENT_RECORD FocusEvent; } Event; } INPUT_RECORD;
#[cfg (feature = "windowsnt")]
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
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD
{
  pub EventType: WORD,
  pub Event: INPUT_RECORD_Event,
}
#[cfg (feature = "windowsnt")]
pub type PINPUT_RECORD = *mut INPUT_RECORD;
/// C: typedef struct _CONSOLE_SCREEN_BUFFER_INFO { COORD dwSize; COORD dwCursorPosition; WORD
/// C: wAttributes; SMALL_RECT srWindow; COORD dwMaximumWindowSize; } CONSOLE_SCREEN_BUFFER_INFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_SCREEN_BUFFER_INFO
{
  pub dwSize: COORD,
  pub dwCursorPosition: COORD,
  pub wAttributes: WORD,
  pub srWindow: SMALL_RECT,
  pub dwMaximumWindowSize: COORD,
}
#[cfg (feature = "windowsnt")]
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;
/// C: typedef struct _FILE_NOTIFY_INFORMATION { DWORD NextEntryOffset; DWORD Action; DWORD
/// C: FileNameLength; WCHAR FileName[1]; } FILE_NOTIFY_INFORMATION;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_INFORMATION
{
  pub NextEntryOffset: DWORD,
  pub Action: DWORD,
  pub FileNameLength: DWORD,
  pub FileName: [WCHAR; 1],
}
/// C 644-739's `HAVE_W32NOTIFY` arm reaches `keyboard_handle`'s critical section through
/// `enter_crit`/`leave_crit`, which expand to `EnterCriticalSection (&critsect)` (the
/// `include/w32term.rs` 1028-1044 pair), so this file's arm needs the record and the two entry
/// points as well.
/// C: typedef struct _RTL_CRITICAL_SECTION { PRTL_CRITICAL_SECTION_DEBUG DebugInfo; LONG LockCount;
/// C: LONG RecursionCount; HANDLE OwningThread; HANDLE LockSemaphore; ULONG_PTR SpinCount; }
/// C: CRITICAL_SECTION, *PCRITICAL_SECTION, *LPCRITICAL_SECTION;
/// C: void WINAPI EnterCriticalSection (LPCRITICAL_SECTION); void WINAPI LeaveCriticalSection
/// C: (LPCRITICAL_SECTION);
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
  pub SpinCount: ULONG_PTR,
}
#[cfg (feature = "windowsnt")]
pub type LPCRITICAL_SECTION = *mut CRITICAL_SECTION;
/// C 666/675's `enter_crit ()`/`leave_crit ()` expand to these two `<synchapi.h>` entry points
/// (the `include/w32term.rs` 1028-1044 pair), so C's `HAVE_W32NOTIFY` arm names them.
/// C: WINBASEAPI void WINAPI EnterCriticalSection (LPCRITICAL_SECTION);
/// C: WINBASEAPI void WINAPI LeaveCriticalSection (LPCRITICAL_SECTION);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn EnterCriticalSection (section: LPCRITICAL_SECTION);
  pub fn LeaveCriticalSection (section: LPCRITICAL_SECTION);
}

/// C 28's `<windows.h>` (continued): the entry points this file calls, all `<wincon.h>`'s.  The
/// `A`/`W` pair of C 69-71 is kept as C's own spelling (this configuration leaves `_UNICODE`
/// undefined, so `ReadConsoleInput` would resolve to the ANSI entry, but C 70-71 names both
/// explicitly); `WINAPI` is `extern "system"`, the ABI Rust spells `__stdcall` with.
/// C: BOOL WINAPI ReadConsoleInputA (HANDLE, PINPUT_RECORD, DWORD, LPDWORD);
/// C: BOOL WINAPI ReadConsoleInputW (HANDLE, PINPUT_RECORD, DWORD, LPDWORD);
/// C: BOOL WINAPI GetNumberOfConsoleInputEvents (HANDLE, LPDWORD);
/// C: BOOL WINAPI GetConsoleScreenBufferInfo (HANDLE, PCONSOLE_SCREEN_BUFFER_INFO);
/// C: HANDLE WINAPI GetStdHandle (DWORD);
/// C: UINT WINAPI GetConsoleCP (void);
/// C: int WINAPI MultiByteToWideChar (UINT, DWORD, LPCCH, int, LPWSTR, int);
/// C: BOOL WINAPI IsDBCSLeadByteEx (UINT, BYTE);
/// C: DWORD WINAPI GetTickCount (void);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn ReadConsoleInputA (h: HANDLE, rec: PINPUT_RECORD, recsize: DWORD,
                            waiting: LPDWORD) -> BOOL;
  pub fn ReadConsoleInputW (h: HANDLE, rec: PINPUT_RECORD, recsize: DWORD,
                            waiting: LPDWORD) -> BOOL;
  pub fn GetNumberOfConsoleInputEvents (h: HANDLE, waiting: LPDWORD) -> BOOL;
  pub fn GetConsoleScreenBufferInfo (h: HANDLE, info: PCONSOLE_SCREEN_BUFFER_INFO) -> BOOL;
  pub fn GetStdHandle (which: DWORD) -> HANDLE;
  pub fn GetConsoleCP () -> UINT;
  pub fn MultiByteToWideChar (codepage: UINT, flags: DWORD, mb: LPCCH, cb: c_int,
                              wc: LPWSTR, cch: c_int) -> c_int;
  pub fn IsDBCSLeadByteEx (codepage: UINT, testchar: BYTE) -> BOOL;
  pub fn GetTickCount () -> DWORD;
}
/// C 28's `<windows.h>` (continued): the keyboard entry points (`<winuser.h>`).
/// C: void WINAPI keybd_event (BYTE, BYTE, DWORD, ULONG_PTR);
/// C: UINT WINAPI MapVirtualKeyA (UINT, UINT); SHORT WINAPI GetAsyncKeyState (int);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn keybd_event (vk: BYTE, scan: BYTE, flags: DWORD, extra: ULONG_PTR);
  pub fn MapVirtualKey (code: UINT, maptype: UINT) -> UINT;
  pub fn GetAsyncKeyState (vk: c_int) -> SHORT;
}

/// C 28's `<windows.h>` (continued): the constants this file spells, `<wincon.h>`'s unless the
/// note says otherwise.  Each is `UNVERIFIED` with the records above (no SDK here).
/// C: #define KEY_EVENT 0x0001 / #define MOUSE_EVENT 0x0002 / #define WINDOW_BUFFER_SIZE_EVENT
/// C: 0x0004 / #define MENU_EVENT 0x0008 / #define FOCUS_EVENT 0x0010  (the `INPUT_RECORD`
/// C: `EventType` values; C 804's local is `unsigned` and the switch compares through it, so
/// C: the port types the five the way C's own local is typed)
/// C: #define RIGHT_ALT_PRESSED 0x0001 / #define LEFT_ALT_PRESSED 0x0002 / #define
/// C: RIGHT_CTRL_PRESSED 0x0004 / #define LEFT_CTRL_PRESSED 0x0008 / #define SHIFT_PRESSED 0x0010
/// C: #define NUMLOCK_ON 0x0020 / #define SCROLLLOCK_ON 0x0040 / #define CAPSLOCK_ON 0x0080
/// C: #define ENHANCED_KEY 0x0100  (the `dwControlKeyState` bits)
/// C: #define STD_OUTPUT_HANDLE ((DWORD)-11)
/// C: #define KEYEVENTF_EXTENDEDKEY 0x0001 / #define KEYEVENTF_KEYUP 0x0002
#[cfg (feature = "windowsnt")]
pub const KEY_EVENT: c_uint = 0x0001;
#[cfg (feature = "windowsnt")]
pub const MOUSE_EVENT: c_uint = 0x0002;
#[cfg (feature = "windowsnt")]
pub const WINDOW_BUFFER_SIZE_EVENT: c_uint = 0x0004;
#[cfg (feature = "windowsnt")]
pub const MENU_EVENT: c_uint = 0x0008;
#[cfg (feature = "windowsnt")]
pub const FOCUS_EVENT: c_uint = 0x0010;
#[cfg (feature = "windowsnt")]
pub const RIGHT_ALT_PRESSED: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const LEFT_ALT_PRESSED: DWORD = 0x0002;
#[cfg (feature = "windowsnt")]
pub const RIGHT_CTRL_PRESSED: DWORD = 0x0004;
#[cfg (feature = "windowsnt")]
pub const LEFT_CTRL_PRESSED: DWORD = 0x0008;
#[cfg (feature = "windowsnt")]
pub const SHIFT_PRESSED: DWORD = 0x0010;
#[cfg (feature = "windowsnt")]
pub const NUMLOCK_ON: DWORD = 0x0020;
#[cfg (feature = "windowsnt")]
pub const SCROLLLOCK_ON: DWORD = 0x0040;
#[cfg (feature = "windowsnt")]
pub const CAPSLOCK_ON: DWORD = 0x0080;
#[cfg (feature = "windowsnt")]
pub const ENHANCED_KEY: DWORD = 0x0100;
#[cfg (feature = "windowsnt")]
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const KEYEVENTF_EXTENDEDKEY: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const KEYEVENTF_KEYUP: DWORD = 0x0002;
/// C 28's `<windows.h>` (continued): the virtual-key constants `<winuser.h>`'s, the ones this file
/// names that `include/w32term.rs` 831-836 does *not* carry (`VK_LWIN`/`VK_RWIN`/`VK_APPS` come
/// from that mirror, which is why they are absent here - a second declaration in this unit would be
/// `E0428`).
/// C: #define VK_CANCEL 0x03 / #define VK_SHIFT 0x10 / #define VK_CONTROL 0x11 / #define VK_MENU
/// C: 0x12 / #define VK_PAUSE 0x13 / #define VK_CAPITAL 0x14 / #define VK_SPACE 0x20 / #define
/// C: VK_NUMLOCK 0x90 / #define VK_SCROLL 0x91
#[cfg (feature = "windowsnt")]
pub const VK_CANCEL: c_int = 0x03;
#[cfg (feature = "windowsnt")]
pub const VK_SHIFT: c_int = 0x10;
#[cfg (feature = "windowsnt")]
pub const VK_CONTROL: c_int = 0x11;
#[cfg (feature = "windowsnt")]
pub const VK_MENU: c_int = 0x12;
#[cfg (feature = "windowsnt")]
pub const VK_PAUSE: c_int = 0x13;
#[cfg (feature = "windowsnt")]
pub const VK_CAPITAL: c_int = 0x14;
#[cfg (feature = "windowsnt")]
pub const VK_SPACE: c_int = 0x20;
#[cfg (feature = "windowsnt")]
pub const VK_NUMLOCK: c_int = 0x90;
#[cfg (feature = "windowsnt")]
pub const VK_SCROLL: c_int = 0x91;

/* C 30-35: `#ifndef MOUSE_MOVED / #define MOUSE_MOVED 1 / #endif` and its `MOUSE_HWHEELED 8`
   twin.  Each is "define it if the SDK has not", so the port carries the constant with C's own
   value and no `cfg` (the `include/w32term.rs` reading of its nine guards); both are `<wincon.h>`'s
   `dwEventFlags` values, compared with that `DWORD` field at C 489/547/562, so the port's type is
   the field's.  */
/// C: #define MOUSE_MOVED 1
#[cfg (feature = "windowsnt")]
pub const MOUSE_MOVED: DWORD = 1;
/// C: #define MOUSE_HWHEELED 8
#[cfg (feature = "windowsnt")]
pub const MOUSE_HWHEELED: DWORD = 8;
/// C 546/571's two other `<wincon.h>` `dwEventFlags` values, which the two guards above do not
/// cover: "mouse wheeled" and "double click" (C 572 shares the second's case with the switch's
/// `default`, which is this port's `_` arm).
/// C: #define MOUSE_WHEELED 0x0004 / #define DOUBLE_CLICK 0x0002
#[cfg (feature = "windowsnt")]
pub const MOUSE_WHEELED: DWORD = 0x0004;
#[cfg (feature = "windowsnt")]
pub const DOUBLE_CLICK: DWORD = 0x0002;

/* C 37-44: the emacs headers.  C's order is the mirror order; `xtask` expands and dedupes the
   nestings (PORTING.md 0.1-4).  C 41's `termchar.h` is the `Mouse_HLInfo`/`tty_display_info` one
   (C's own comment says so), C 42's `w32term.h` brings the keyboard-hook names and the keypad
   constants, and C 44's `w32common.h` brings `os_subtype`, which `w32_kbdhook_active` reads.  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");           // lisp.h's `<stdlib.h>` (C 727-728's `free`)
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                    // C 37: `#include "lisp.h"`
#[cfg (feature = "windowsnt")]
include!("include/keyboard.rs");                // C 38: `#include "keyboard.h"`
#[cfg (feature = "windowsnt")]
include!("include/frame.rs");                   // C 39: `#include "frame.h"`
#[cfg (feature = "windowsnt")]
include!("include/blockinput.rs");              // C 40: `#include "blockinput.h"`
#[cfg (feature = "windowsnt")]
include!("include/termchar.rs");                // C 41: `#include "termchar.h"`	/* for Mouse_HLInfo, tty_display_info */
#[cfg (feature = "windowsnt")]
include!("include/w32term.rs");                 // C 42: `#include "w32term.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32inevt.rs");                // C 43: `#include "w32inevt.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");               // C 44: `#include "w32common.h"`

/* ----------------------------------------------------------------------------------------------
   Use-site declarations for the names this file calls that no mirror on its include graph
   carries (P-66/P-72 rule 4, the `src/w32console.rs` 415-470 shape).  Each keeps C's own
   prototype in a `/// C:` line, and the ledger's `## Owed` names the mirror rows that would let
   them retire.
   ---------------------------------------------------------------------------------------------- */

/// C 47's `keyboard_handle`: `w32console.c` 78 defines it (`src/w32console.rs` 411) and C 91/97
/// read it here; no mirror carries the declaration, so this unit spells it.
/// C: extern HANDLE keyboard_handle;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut keyboard_handle: HANDLE;
}

/// C 54's `map_keypad_keys`: `w32fns.c` defines it (C's own comment says so) and C 287-289 calls
/// it; no mirror carries it.  C 55's `w32_key_to_modifier` is the *other* name of that comment and
/// is **dropped**: C declares it and never calls it (the PORTING.md 0.1-12 rule for a forward
/// declaration with no use site), which `## Deviations` 2 records.
/// C: extern unsigned int map_keypad_keys (unsigned int, unsigned int);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn map_keypad_keys (key: c_uint, extended: c_uint) -> c_uint;
}

/// C 621/639-641's `change_frame_size`: declared by `dispextern.h` 3924, defined by `src/dispnew.c`
/// (`src/dispnew.rs` 7509), and not carried by `include/dispextern.rs` yet.
/// C: void change_frame_size (struct frame *, int, int, bool, bool, bool);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                        pretend: bool, delay: bool, safe: bool);
}

/// C 501/825's `clear_mouse_face`: declared by `dispextern.h` 3691, defined by `src/xdisp.c`, and
/// not carried by `include/dispextern.rs` (its 4039 records the row).
/// C: extern bool clear_mouse_face (Mouse_HLInfo *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
}

/// C 534's `note_mouse_highlight`: declared by `dispextern.h` 3688, defined by `src/xdisp.c`, and
/// not carried by `include/dispextern.rs` either (the same 4039 row).
/// C: extern void note_mouse_highlight (struct frame *, int, int);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
}

/// C 538-540's `gen_help_event`: declared by `keyboard.h` 513-514, defined by `src/keyboard.c`, and
/// not carried by `include/keyboard.rs`.
/// C: extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, ptrdiff_t);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn gen_help_event (help: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
                     obj: Lisp_Object, pos: ptrdiff_t);
}

/// C 291's `lispy_function_keys`: declared by `keyboard.h` 525 as an incomplete array and *defined*
/// by `src/keyboard.c` 5513's `HAVE_NTGUI` arm with 256 slots (the same row `src/xwidget.rs`
/// 1233-1240 spells for the X11 arm), so the port declares those 256 slots rather than an
/// incomplete array C 291 could not index.
/// C: extern const char *const lispy_function_keys[];
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut lispy_function_keys: [*const c_char; 256];
}

/* C 46-55's four `extern` declarations are the section above (the C 47 handle, C 54's
   `map_keypad_keys`, and C 75's `faked_key`, whose definition follows in C's order).  */

/* C 49-51: the last mouse motion's position and time.  C's tentative definitions (no initializer)
   are the port's explicit zero initializers (PORTING.md 1.5); `Time` is `systime.h`'s, reached
   through `termhooks.h`'s `include!` (`include/termhooks.rs` 22).  */
/// C: static COORD movement_pos; static Time movement_time;
#[cfg (feature = "windowsnt")]
static mut movement_pos: COORD = COORD { X: 0, Y: 0 };
#[cfg (feature = "windowsnt")]
static mut movement_time: Time = 0;

/* C 57-63: the event queue and the DBCS lead byte.  `EVENT_QUEUE_SIZE` is C's `#define` (50) and
   the two pointers start where C's initializers put them - at the queue's first slot.  */
#[cfg (feature = "windowsnt")]
const EVENT_QUEUE_SIZE: usize = 50;             // C 58: `#define EVENT_QUEUE_SIZE 50`
/// C: static INPUT_RECORD event_queue[EVENT_QUEUE_SIZE];
/// C: static INPUT_RECORD *queue_ptr = event_queue, *queue_end = event_queue;
#[cfg (feature = "windowsnt")]
static mut event_queue: [INPUT_RECORD; EVENT_QUEUE_SIZE] = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut queue_ptr: *mut INPUT_RECORD = &raw mut event_queue as *mut INPUT_RECORD;
#[cfg (feature = "windowsnt")]
static mut queue_end: *mut INPUT_RECORD = &raw mut event_queue as *mut INPUT_RECORD;

/// C 63: `static char dbcs_lead = 0;` - "Temporarily store lead byte of DBCS input sequences."
#[cfg (feature = "windowsnt")]
static mut dbcs_lead: c_char = 0;

/* C 65-72: `static inline BOOL w32_read_console_input (HANDLE h, INPUT_RECORD *rec, DWORD recsize,
   DWORD *waiting)`.  `w32_console_unicode_input` is `w32inevt.h` 22's (`include/w32inevt.rs` 19,
   defined by `src/w32console.rs` 414).  */
#[cfg (feature = "windowsnt")]
unsafe fn w32_read_console_input (h: HANDLE, rec: *mut INPUT_RECORD, recsize: DWORD,
                                  waiting: *mut DWORD) -> BOOL
{
  return if w32_console_unicode_input != 0
         { ReadConsoleInputW (h, rec, recsize, waiting) }
         else { ReadConsoleInputA (h, rec, recsize, waiting) };
}

/* C 74-76: "Set by w32_console_toggle_lock_key."  C's first line is this unit's own declaration of
   the object the second line defines; the declaration is dropped here and the definition keeps C's
   non-`static` linkage (PORTING.md 0.1-3).  */
/// C: int faked_key;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut faked_key: c_int = 0;

/* C 78-104: `static int fill_queue (BOOL block)`.  C's `queue_end - queue_ptr` is the pointer
   difference the port spells `offset_from` (the `src/msdos.rs` reading of the same idiom), and C's
   `(int) events_waiting` keeps its `as c_int`.  */
#[cfg (feature = "windowsnt")]
unsafe fn fill_queue (block: BOOL) -> c_int
{
  let mut rc: BOOL = 0;
  let mut events_waiting: DWORD = 0;

  if queue_ptr < queue_end
    { return queue_end.offset_from (queue_ptr) as c_int; }

  if block == 0
    {
      /* Check to see if there are some events to read before we try
	 because we can't block.  */
      if GetNumberOfConsoleInputEvents (keyboard_handle, &raw mut events_waiting) == 0
        { return -1; }
      if events_waiting == 0
        { return 0; }
    }

  rc = w32_read_console_input (keyboard_handle, &raw mut event_queue as *mut INPUT_RECORD,
			       EVENT_QUEUE_SIZE as DWORD, &raw mut events_waiting);
  if rc == 0
    { return -1; }
  queue_ptr = &raw mut event_queue as *mut INPUT_RECORD;
  queue_end = (&raw mut event_queue as *mut INPUT_RECORD).offset (events_waiting as isize);
  return events_waiting as c_int;
}

/* C 106-114: `static struct frame * get_frame (void)` - "In a generic, multi-frame world this
   should take a console handle and return the frame for it.  Right now, there's only one frame so
   return it."  */
#[cfg (feature = "windowsnt")]
unsafe fn get_frame () -> *mut frame
{
  return SELECTED_FRAME ();
}

/* C 116-118 is the two-line banner above C 139's `key_event` ("Translate console modifiers to emacs
   modifiers.  German keyboard support (Kai Morgan Zeise 2/18/95)."), kept where C has it.  */

/* C 120-130 is `#if 0`: C's preprocessor never compiles it, so the port keeps C's text as a comment
   (the `src/cm.rs` 86 reading).

#if 0
/* Return nonzero if the virtual key is a dead key.  */
static int
is_dead_key (int wparam)
{
  unsigned int code = MapVirtualKey (wparam, 2);

  /* Windows 95 returns 0x8000, NT returns 0x80000000.  */
  return (code & 0x80008000) ? 1 : 0;
}
#endif
*/

/* C 132-138: `key_event`'s two banners - the codepage note and "return code -1 means that
   event_queue_ptr won't be incremented.  In other word, this event makes two key codes.  (by himi)".  */

/* `struct input_event`'s two bit fields - `termhooks.h` 373-412's `enum event_kind kind :
   EVENT_KIND_WIDTH` and the `part` that shares the same unit (`include/termhooks.rs` 108-128
   carries `bits`, `INPUT_EVENT_KIND_MASK` and `INPUT_EVENT_PART_SHIFT`) - are C members the port
   reaches through a per-unit macro, because `macro_rules!` is textual and the `src/keyboard.rs`
   copy belongs to that unit (PORTING.md P-80's member-named shape, the `src/msdos.rs` 3424 copy).
   This unit uses the `kind` pair (C writes `emacs_ev->kind = x;`).  One `## Deviations` entry.  */
#[cfg (feature = "windowsnt")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* C 139-409: `static int key_event (KEY_EVENT_RECORD *event, struct input_event *emacs_ev,
   int *isdead)`.  `mod_key_state` is C's function-local `static int`, and the two switches keep
   C's case order.

   C's second switch (C 177-274) reaches one shared body through a `goto` into the middle of the
   switch - the `disable_lock_key` label at C 236, reached from C 224/229/234 - and no path falls
   *into* the label: the three cases that reach it jump, and `VK_SCROLL`'s own arm returns.  The
   port keeps the shared body once, reached through the `disable_lock_key` flag the three cases set
   (PORTING.md 2.2: "显式结构化写法", the `src/msdos.rs` reading of the same construct), which is
   the same body reached for the same three states.  One `## Deviations` entry.  */
#[cfg (feature = "windowsnt")]
unsafe fn key_event (event: *mut KEY_EVENT_RECORD, emacs_ev: *mut input_event,
                    isdead: *mut c_int) -> c_int
{
  static mut mod_key_state: c_int = 0;
  let mut wParam: c_int = 0;
  let mut disable_lock_key: bool = false;

  *isdead = 0;

  /* Skip key-up events.  */
  if (*event).bKeyDown == 0
    {
      match (*event).wVirtualKeyCode as c_int
	{
	  VK_LWIN =>
	    {
	      if ! w32_kbdhook_active! ()
		{ mod_key_state &= ! LEFT_WIN_PRESSED; }
	    }
	  VK_RWIN =>
	    {
	      if ! w32_kbdhook_active! ()
		{ mod_key_state &= ! RIGHT_WIN_PRESSED; }
	    }
	  VK_APPS =>
	    {
	      mod_key_state &= ! APPS_PRESSED;
	    }
	  _ => {}
	}
      return 0;
    }

  /* Ignore keystrokes we fake ourself; see below.
     When Windows IME messages are sent, wVirtualKeyCode is 0.
     So check and make sure they aren't ignored.  */
  if (*event).wVirtualKeyCode != 0 && faked_key == (*event).wVirtualKeyCode as c_int
    {
      faked_key = 0;
      return 0;
    }

  /* To make it easier to debug this code, ignore modifier keys!  */
  match (*event).wVirtualKeyCode as c_int
    {
      VK_LWIN =>
	{
	  if NILP (Vw32_pass_lwindow_to_system)
	    {
	      /* Prevent system from acting on keyup (which opens the Start
		 menu if no other key was pressed) by simulating a press of
		 Space which we will ignore.  */
	      if mod_key_state & LEFT_WIN_PRESSED == 0
		{
		  if FIXNUMP (Vw32_phantom_key_code)
		    { faked_key = (XUFIXNUM (Vw32_phantom_key_code) & 255) as c_int; }
		  else
		    { faked_key = VK_SPACE; }
		  keybd_event (faked_key as BYTE,
			       MapVirtualKey (faked_key as UINT, 0) as BYTE, 0, 0);
		}
	    }
	  if ! w32_kbdhook_active! ()
	    { mod_key_state |= LEFT_WIN_PRESSED; }
	  if ! NILP (Vw32_lwindow_modifier)
	    { return 0; }
	}
      VK_RWIN =>
	{
	  if NILP (Vw32_pass_rwindow_to_system)
	    {
	      if mod_key_state & RIGHT_WIN_PRESSED == 0
		{
		  if FIXNUMP (Vw32_phantom_key_code)
		    { faked_key = (XUFIXNUM (Vw32_phantom_key_code) & 255) as c_int; }
		  else
		    { faked_key = VK_SPACE; }
		  keybd_event (faked_key as BYTE,
			       MapVirtualKey (faked_key as UINT, 0) as BYTE, 0, 0);
		}
	    }
	  if ! w32_kbdhook_active! ()
	    { mod_key_state |= RIGHT_WIN_PRESSED; }
	  if ! NILP (Vw32_rwindow_modifier)
	    { return 0; }
	}
      VK_APPS =>
	{
	  mod_key_state |= APPS_PRESSED;
	  if ! NILP (Vw32_apps_modifier)
	    { return 0; }
	}
      VK_CAPITAL =>
	{
	  /* Decide whether to treat as modifier or function key.  */
	  if NILP (Vw32_enable_caps_lock)
	    { disable_lock_key = true; }
	  else
	    { return 0; }
	}
      VK_NUMLOCK =>
	{
	  /* Decide whether to treat as modifier or function key.  */
	  if NILP (Vw32_enable_num_lock)
	    { disable_lock_key = true; }
	  else
	    { return 0; }
	}
      VK_SCROLL =>
	{
	  /* Decide whether to treat as modifier or function key.  */
	  if NILP (Vw32_scroll_lock_modifier)
	    { disable_lock_key = true; }
	  else
	    { return 0; }
	}
      VK_MENU | VK_CONTROL | VK_SHIFT =>
	{ return 0; }
      VK_CANCEL =>
	{
	  /* Windows maps Ctrl-Pause (aka Ctrl-Break) into VK_CANCEL,
	     which is confusing for purposes of key binding; convert
	     VK_CANCEL events into VK_PAUSE events.  */
	  (*event).wVirtualKeyCode = VK_PAUSE as WORD;
	}
      VK_PAUSE =>
	{
	  /* Windows maps Ctrl-NumLock into VK_PAUSE, which is confusing
	     for purposes of key binding; convert these back into
	     VK_NUMLOCK events, at least when we want to see NumLock key
	     presses.  (Note that there is never any possibility that
	     VK_PAUSE with Ctrl really is C-Pause as per above.)  */
	  if NILP (Vw32_enable_num_lock)
	      && (*event).dwControlKeyState
		 & (LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED) != 0
	    { (*event).wVirtualKeyCode = VK_NUMLOCK as WORD; }
	}
      _ => {}
    }

  /* C 236-252: the `disable_lock_key` label's body, reached from the three cases above.
     "Ensure the appropriate lock key state is off (and the indicator light as well)."  */
  if disable_lock_key
    {
      wParam = (*event).wVirtualKeyCode as c_int;
      if GetAsyncKeyState (wParam) & 0x8000 != 0
	{
	  /* Fake another press of the relevant key.  Apparently, this
	     really is the only way to turn off the indicator.  */
	  faked_key = wParam;
	  keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
		       KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
	  keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
		       KEYEVENTF_EXTENDEDKEY | 0, 0);
	  keybd_event (wParam as BYTE, MapVirtualKey (wParam as UINT, 0) as BYTE,
		       KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
	}
    }

  /* Recognize state of Windows and Apps keys.  */
  (*event).dwControlKeyState |= mod_key_state as DWORD;
  if w32_kbdhook_active! ()
    {
      if check_w32_winkey_state (VK_LWIN) != 0
	{ (*event).dwControlKeyState |= LEFT_WIN_PRESSED as DWORD; }
      if check_w32_winkey_state (VK_RWIN) != 0
	{ (*event).dwControlKeyState |= RIGHT_WIN_PRESSED as DWORD; }
    }

  /* Distinguish numeric keypad keys from extended keys.  */
  (*event).wVirtualKeyCode =
    map_keypad_keys ((*event).wVirtualKeyCode as c_uint,
		     ((*event).dwControlKeyState & ENHANCED_KEY)) as WORD;

  if lispy_function_keys[(*event).wVirtualKeyCode as usize].is_null ()
    {
      if ! NILP (Vw32_recognize_altgr)
	  && (*event).dwControlKeyState & LEFT_CTRL_PRESSED != 0
	  && (*event).dwControlKeyState & RIGHT_ALT_PRESSED != 0
	{
	  /* Don't try to interpret AltGr key chords; ToAscii seems not
	     to process them correctly.  */
	}
      /* Handle key chords including any modifiers other than shift
         directly, in order to preserve as much modifier information as
         possible.  */
      else if (*event).dwControlKeyState
	       & (  RIGHT_CTRL_PRESSED | LEFT_CTRL_PRESSED
		  | RIGHT_ALT_PRESSED | LEFT_ALT_PRESSED
		  | (if ! NILP (Vw32_lwindow_modifier)
		     { LEFT_WIN_PRESSED as DWORD } else { 0 })
		  | (if ! NILP (Vw32_rwindow_modifier)
		     { RIGHT_WIN_PRESSED as DWORD } else { 0 })
		  | (if ! NILP (Vw32_apps_modifier)
		     { APPS_PRESSED as DWORD } else { 0 })
		  | (if ! NILP (Vw32_scroll_lock_modifier)
		     { SCROLLLOCK_ON } else { 0 })) != 0
	{
	  /* Don't translate modified alphabetic keystrokes, so the user
	     doesn't need to constantly switch layout to type control or
	     meta keystrokes when the normal layout translates
	     alphabetic characters to non-ascii characters.  */
	  if 'A' as WORD <= (*event).wVirtualKeyCode
	      && (*event).wVirtualKeyCode <= 'Z' as WORD
	    {
	      (*event).uChar.AsciiChar = (*event).wVirtualKeyCode as CHAR;
	      if (*event).dwControlKeyState & SHIFT_PRESSED == 0
		{ (*event).uChar.AsciiChar = ((*event).uChar.AsciiChar as c_int)
					      .wrapping_add ('a' as c_int - 'A' as c_int)
					      as CHAR; }
	    }
	  /* Try to handle unrecognized keystrokes by determining the
             base character (ie. translating the base key plus shift
             modifier).  */
	  else if (*event).uChar.AsciiChar == 0
	    { w32_kbd_patch_key (event, -1); }
	}

      if (w32_console_unicode_input != 0 && (*event).uChar.UnicodeChar == 0)
	  || (w32_console_unicode_input == 0 && (*event).uChar.UnicodeChar == 0)
	{
	  input_event_bits! (kind, emacs_ev, NO_EVENT);
	  return 0;
	}
      else if (w32_console_unicode_input != 0 && (*event).uChar.UnicodeChar < 128)
	       || (w32_console_unicode_input == 0 && (*event).uChar.AsciiChar > 0)
	{
	  /* Pure ASCII characters < 128.  */
	  input_event_bits! (kind, emacs_ev, ASCII_KEYSTROKE_EVENT);
	  (*emacs_ev).code = (*event).uChar.AsciiChar as c_uint;
	}
      else if (*event).uChar.UnicodeChar > 0
	       && w32_console_unicode_input != 0
	{
	  /* Unicode codepoint; only valid if we are using Unicode
	     console input mode.  */
	  input_event_bits! (kind, emacs_ev, MULTIBYTE_CHAR_KEYSTROKE_EVENT);
	  (*emacs_ev).code = (*event).uChar.UnicodeChar as c_uint;
	}
      else
	{
	  /* Fallback handling of non-ASCII characters for non-Unicode
	     versions of Windows, and for non-Unicode input on NT
	     family of Windows.  Only characters in the current
	     console codepage are supported by this fallback.  */
	  let mut code: wchar_t = 0;
	  let mut dbcs: [c_char; 2] = [0; 2];
          let mut cpId: c_int = 0;

	  /* Get the current console input codepage to interpret this
	     key with.  Note that the system defaults for the OEM
	     codepage could have been changed by calling SetConsoleCP
	     or w32-set-console-codepage, so using GetLocaleInfo to
	     get LOCALE_IDEFAULTCODEPAGE is not TRT here.  */
          cpId = GetConsoleCP () as c_int;

	  dbcs[0] = dbcs_lead;
	  dbcs[1] = (*event).uChar.AsciiChar;
	  if dbcs_lead != 0
	    {
	      dbcs_lead = 0;
	      if MultiByteToWideChar (cpId as UINT, 0, &raw const dbcs[0], 2,
				      &raw mut code, 1) == 0
		{
		  /* Garbage  */
		  DebPrint! (("Invalid DBCS sequence: %d %d\n",
			     dbcs[0], dbcs[1]));
		  input_event_bits! (kind, emacs_ev, NO_EVENT);
		}
	    }
	  else if IsDBCSLeadByteEx (cpId as UINT, dbcs[1] as BYTE) != 0
	    {
	      dbcs_lead = dbcs[1];
	      input_event_bits! (kind, emacs_ev, NO_EVENT);
	    }
	  else
	    {
	      if MultiByteToWideChar (cpId as UINT, 0, &raw const dbcs[1], 1,
				      &raw mut code, 1) == 0
		{
		  /* Garbage  */
		  DebPrint! (("Invalid character: %d\n", dbcs[1]));
		  input_event_bits! (kind, emacs_ev, NO_EVENT);
		}
	    }
	  input_event_bits! (kind, emacs_ev, MULTIBYTE_CHAR_KEYSTROKE_EVENT);
	  (*emacs_ev).code = code as c_uint;
	}
    }
  else
    {
      /* Function keys and other non-character keys.  */
      input_event_bits! (kind, emacs_ev, NON_ASCII_KEYSTROKE_EVENT);
      (*emacs_ev).code = (*event).wVirtualKeyCode as c_uint;
    }

  XSETFRAME! ((*emacs_ev).frame_or_window, get_frame ());
  (*emacs_ev).modifiers = w32_kbd_mods_to_emacs ((*event).dwControlKeyState,
					      (*event).wVirtualKeyCode) as c_uint;
  (*emacs_ev).timestamp = GetTickCount () as Time;
  return 1;
}

/* `f->mouse_moved` (`frame.h` 434, a `bool_bf`): the reader and writer the `struct frame` flag run
   needs.  `include/frame.rs` 520-530 carries the shift and unit constants but no accessor, and the
   tree defines this per consuming unit (`src/window.rs` 152's `set_frame_flag1!`, the
   `src/androidterm.rs` 235 `mouse_moved!` shape); this unit's copy carries the file gate.
   One `## Deviations` entry.  */
#[cfg (feature = "windowsnt")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "windowsnt")]
macro_rules! mouse_moved {
  ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
}

/* C 411-435: "Mouse position hook."  `w32_console_mouse_position` is one of the two entry points
   `w32inevt.h` declares (C 25-28's prototype) and this unit defines.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_console_mouse_position (f: *mut *mut frame,
						     mut insist: c_int,
						     bar_window: *mut Lisp_Object,
						     part: *mut scroll_bar_part,
						     x: *mut Lisp_Object,
						     y: *mut Lisp_Object,
						     time: *mut Time)
{
  block_input ();

  insist = insist;

  *f = get_frame ();
  *bar_window = Qnil ();
  *part = scroll_bar_above_handle;
  mouse_moved! (*f, 0);

  XSETINT! (*x, movement_pos.X as EMACS_INT);
  XSETINT! (*y, movement_pos.Y as EMACS_INT);
  *time = movement_time;

  unblock_input ();
}

/* C 437-450: "Remember mouse motion and notify emacs."  `f->mouse_moved` is `frame.h` 434's
   `bool_bf`, read and written through this unit's `mouse_moved!` above (the storage unit
   `include/frame.rs` 520-530 carries the shift for).  */
#[cfg (feature = "windowsnt")]
unsafe fn mouse_moved_to (x: c_int, y: c_int)
{
  /* If we're in the same place, ignore it.  */
  if x != movement_pos.X as c_int || y != movement_pos.Y as c_int
    {
      let f: *mut frame = get_frame ();
      mouse_moved! (f, 1);
      movement_pos.X = x as SHORT;
      movement_pos.Y = y as SHORT;
      movement_time = GetTickCount () as Time;
    }
}

/* C 452-462: the console's button-bit order - "least significant - Leftmost button / next -
   Rightmost button / next - Leftmost+1 / next - Leftmost+2...", with the 0/1/2 values for the
   three standard buttons and "Others increase from there."  */
/* C 464-468: `#define NUM_TRANSLATED_MOUSE_BUTTONS 5` and the translation table.  */
#[cfg (feature = "windowsnt")]
const NUM_TRANSLATED_MOUSE_BUTTONS: c_int = 5;  // C 464: `#define NUM_TRANSLATED_MOUSE_BUTTONS 5`
/// C: static int emacs_button_translation[NUM_TRANSLATED_MOUSE_BUTTONS] = { 0, 2, 1, 3, 4 };
#[cfg (feature = "windowsnt")]
static mut emacs_button_translation: [c_int; NUM_TRANSLATED_MOUSE_BUTTONS as usize] =
  [0, 2, 1, 3, 4];

/* `Mouse_HLInfo`'s third flag bit - `dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, the
   third bit of the record's `bits` byte (`include/dispextern.rs` 2127 and its `MOUSE_FACE_HIDDEN`
   mask).  The mirror carries the `mouse_face_defer` pair but not this one, so the unit keeps C's
   member token as a macro (the `src/msdos.rs` 3434-3444 shape).  */
#[cfg (feature = "windowsnt")]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN }; };
}

/* C 470-614: `static int do_mouse_event (MOUSE_EVENT_RECORD *event, struct input_event
   *emacs_ev)`.  `button_state` is C's function-local `static DWORD`; C's `case DOUBLE_CLICK:
   default:` pair is the `_` arm (Rust's `match` needs a catch-all where C's `switch` does not), and
   C's `for (i = 0; mask; i++, mask <<= 1)` is a `while` whose `break` is C's.  */
#[cfg (feature = "windowsnt")]
unsafe fn do_mouse_event (event: *mut MOUSE_EVENT_RECORD,
			  emacs_ev: *mut input_event) -> c_int
{
  static mut button_state: DWORD = 0;
  let mut but_change: DWORD = 0;
  let mut mask: DWORD = 0;
  let mut flags: DWORD = (*event).dwEventFlags;
  let mut i: c_int = 0;

  let f: *mut frame = get_frame ();

  /* For now, mouse events on child frames are ignored, because the
     coordinate conversion is not in place; FIXME.  */
  if FRAMEP ((*f).parent_frame)
    { return 0; }
  /* Mouse didn't move unless MOUSE_MOVED says it did.  */
  mouse_moved! (f, 0);

  match flags
    {
      MOUSE_MOVED =>
	{
	  let hlinfo: *mut Mouse_HLInfo = MOUSE_HL_INFO! (f);
	  let mx: c_int = (*event).dwMousePosition.X as c_int;
	  let my: c_int = (*event).dwMousePosition.Y as c_int;

	  mouse_moved_to (mx, my);

	  if mouse_moved! (f)
	    {
	      if mouse_face_hidden! (hlinfo)
		{
		  mouse_face_hidden! (hlinfo, 0);
		  clear_mouse_face (hlinfo);
		}

	      /* Generate SELECT_WINDOW_EVENTs when needed.  */
	      if ! NILP (Vmouse_autoselect_window)
		{
		  let mouse_window: Lisp_Object
		    = window_from_coordinates (f as *mut c_void, mx, my,
					       std::ptr::null_mut (), false, false, false);
		  /* A window will be selected only when it is not
		     selected now, and the last mouse movement event was
		     not in it.  A minibuffer window will be selected iff
		     it is active.  */
		  if WINDOWP (mouse_window)
		      && ! EQ (mouse_window, last_mouse_window)
		      && ! EQ (mouse_window, selected_window)
		    {
		      let mut event: input_event = core::mem::zeroed ();

		      EVENT_INIT (&raw mut event);
		      input_event_bits! (kind, &raw mut event, SELECT_WINDOW_EVENT);
		      event.frame_or_window = mouse_window;
		      event.arg = Qnil ();
		      event.timestamp = movement_time;
		      kbd_buffer_store_event (&raw mut event as *mut c_void);
		    }
		  last_mouse_window = mouse_window;
		}
	      else
		{ last_mouse_window = Qnil (); }

	      previous_help_echo_string = help_echo_string;
	      help_echo_string = Qnil ();
	      help_echo_object = Qnil ();
	      help_echo_window = Qnil ();
	      help_echo_pos = -1;
	      note_mouse_highlight (f, mx, my);
	      /* If the contents of the global variable help_echo has
		 changed (inside note_mouse_highlight), generate a HELP_EVENT.  */
	      if ! NILP (help_echo_string) || ! NILP (previous_help_echo_string)
		{ gen_help_event (help_echo_string, selected_frame,
				  help_echo_window, help_echo_object,
				  help_echo_pos); }
	    }
	  /* We already called kbd_buffer_store_event, so indicate to
	     the caller it shouldn't.  */
	  return 0;
	}
      MOUSE_WHEELED | MOUSE_HWHEELED =>
	{
	  /* Mouse positions in console wheel events are reported to
	     ReadConsoleInput relative to the display's top-left
	     corner(!), not relative to the origin of the console screen
	     buffer.  This makes these coordinates unusable; e.g.,
	     scrolling the tab-line in general doesn't work.
	     FIXME (but how?).  */
	  let mx: c_int = (*event).dwMousePosition.X as c_int;
	  let my: c_int = (*event).dwMousePosition.Y as c_int;
	  let down_p: bool = (*event).dwButtonState & 0x10000000 != 0;

	  input_event_bits! (kind, emacs_ev,
			     if flags == MOUSE_HWHEELED { HORIZ_WHEEL_EVENT }
			     else { WHEEL_EVENT });
	  (*emacs_ev).code = 0;
	  (*emacs_ev).modifiers = (if down_p { down_modifier } else { up_modifier }) as c_uint;
	  (*emacs_ev).modifiers |=
	    w32_kbd_mods_to_emacs ((*event).dwControlKeyState, 0) as c_uint;
	  XSETINT! ((*emacs_ev).x, mx as EMACS_INT);
	  XSETINT! ((*emacs_ev).y, my as EMACS_INT);
	  XSETFRAME! ((*emacs_ev).frame_or_window, f);
	  (*emacs_ev).arg = Qnil ();
	  (*emacs_ev).timestamp = GetTickCount () as Time;
	  return 1;
	}
      _ =>	/* C 571-572's `case DOUBLE_CLICK: default:` - mouse pressed or released */
	{
	  /* It looks like the console code sends us a button-release
	     mouse event with dwButtonState == 0 when a window is
	     activated and when the mouse is first clicked.  Ignore this
	     case.  */
	  if (*event).dwButtonState == button_state
	    { return 0; }

	  /* Find out what button has changed state since the last button
	     event.  */
	  but_change = button_state ^ (*event).dwButtonState;
	  mask = 1;
	  i = 0;
	  while mask != 0
	    {
	      if but_change & mask != 0
		{
		  if i < NUM_TRANSLATED_MOUSE_BUTTONS
		    { (*emacs_ev).code = emacs_button_translation[i as usize] as c_uint; }
		  else
		    { (*emacs_ev).code = i as c_uint; }
		  break;
		}
	      i += 1;
	      mask = mask.wrapping_shl (1);
	    }

	  button_state = (*event).dwButtonState;
	  (*emacs_ev).modifiers = w32_kbd_mods_to_emacs ((*event).dwControlKeyState, 0) as c_uint;
	  (*emacs_ev).timestamp = GetTickCount () as Time;

	  let x: c_int = (*event).dwMousePosition.X as c_int;
	  let y: c_int = (*event).dwMousePosition.Y as c_int;
	  (*emacs_ev).arg = tty_handle_tab_bar_click (f, x, y,
						    (button_state & mask) != 0,
						    emacs_ev);

	  (*emacs_ev).modifiers |= (if button_state & mask != 0 { down_modifier }
				 else { up_modifier }) as c_uint;

	  input_event_bits! (kind, emacs_ev, MOUSE_CLICK_EVENT);
	  XSETFASTINT! ((*emacs_ev).x, x as EMACS_INT);
	  XSETFASTINT! ((*emacs_ev).y, y as EMACS_INT);
	  XSETFRAME! ((*emacs_ev).frame_or_window, f);

	  return 1;
	}
    }
}

/* C 616-623: `static void resize_event (WINDOW_BUFFER_SIZE_RECORD *event)`.  */
#[cfg (feature = "windowsnt")]
unsafe fn resize_event (event: *mut WINDOW_BUFFER_SIZE_RECORD)
{
  let f: *mut frame = get_frame ();

  change_frame_size (f, (*event).dwSize.X as c_int, (*event).dwSize.Y as c_int,
		     false, true, false);
  SET_FRAME_GARBAGED! (f);
}

/* C 625-642: `static void maybe_generate_resize_event (void)`.  */
#[cfg (feature = "windowsnt")]
unsafe fn maybe_generate_resize_event ()
{
  let mut info: CONSOLE_SCREEN_BUFFER_INFO = core::mem::zeroed ();
  let f: *mut frame = get_frame ();

  /* Only resize the root frame.  */
  if FRAMEP ((*f).parent_frame)
    { return; }

  GetConsoleScreenBufferInfo (GetStdHandle (STD_OUTPUT_HANDLE), &raw mut info);

  /* It is okay to call this unconditionally, since it will do nothing
     if the size hasn't actually changed.  */
  change_frame_size (f, 1 + info.srWindow.Right as c_int - info.srWindow.Left as c_int,
		     1 + info.srWindow.Bottom as c_int - info.srWindow.Top as c_int,
		     false, true, false);
}

/* C 644-739: `handle_file_notifications (struct input_event *hold_quit)` - C's `#if HAVE_W32NOTIFY`
   arm and its `#else` twin.  The macro is `undef` in the frozen configuration
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1675), so C's second arm is the frozen one and
   both are carried under `feature = "have_w32notify"` (P-51: the feature names the non-frozen
   side).  The record and the three helpers it drives are `w32term.h` 785-800's
   (`include/w32term.rs` 1085-1116), and `Qutf_16le` and `initialized` are `globals.rs`'s.  */
#[cfg (all (feature = "windowsnt", feature = "have_w32notify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_file_notifications (hold_quit: *mut input_event) -> c_int
{
  let mut ns: *mut notifications_set = std::ptr::null_mut ();
  let mut nevents: c_int = 0;
  let mut done: c_int = 0;

  /* We cannot process notification before Emacs is fully initialized,
     since we need the UTF-16LE coding-system to be set up.  */
  if ! initialized
    {
      return nevents;
    }

  while done == 0
    {
      ns = std::ptr::null_mut ();

      /* Find out if there is a record available in the linked list of
	 notifications sets.  If so, unlink the set from the linked list.
	 Use the critical section.  */
      enter_crit! ();
      if (*notifications_set_head).next != notifications_set_head
	{
	  ns = (*notifications_set_head).next;
	  (*(*ns).prev).next = (*ns).next;
	  (*(*ns).next).prev = (*ns).prev;
	}
      else
	{ done = 1; }
      leave_crit! ();

      if ! ns.is_null ()
	{
	  let mut p: *mut BYTE = (*ns).notifications;
	  let mut fni: *mut FILE_NOTIFY_INFORMATION = p as *mut FILE_NOTIFY_INFORMATION;
	  let min_size: DWORD
	    = core::mem::offset_of! (FILE_NOTIFY_INFORMATION, FileName) as DWORD
	      + core::mem::size_of::<wchar_t> () as DWORD;
	  let mut inev: input_event = core::mem::zeroed ();
	  let mut info_size: DWORD = (*ns).size;
	  let cs: Lisp_Object = Qutf_16le ();
	  let obj: Lisp_Object = w32_get_watch_object ((*ns).desc);

	  /* notifications size could be zero when the buffer of
	     notifications overflowed on the OS level, or when the
	     directory being watched was itself deleted.  Do nothing in
	     that case.  */
	  if info_size != 0
	      && ! NILP (obj) && CONSP (obj)
	    {
	      let callback: Lisp_Object = XCDR (obj);

	      EVENT_INIT (&raw mut inev);

	      while info_size >= min_size
		{
		  let utf_16_fn: Lisp_Object
		    = make_unibyte_string ((*fni).FileName.as_ptr () as *const c_char,
					   (*fni).FileNameLength as ptrdiff_t);
		  /* Note: mule-conf is preloaded, so utf-16le must
		     already be defined at this point.  */
		  let fname: Lisp_Object
		    = code_convert_string_norecord (utf_16_fn, cs, false);
		  let action: Lisp_Object = w32_lispy_file_action ((*fni).Action);

		  input_event_bits! (kind, &raw mut inev, FILE_NOTIFY_EVENT);
		  inev.timestamp = GetTickCount () as Time;
		  inev.modifiers = 0;
		  inev.frame_or_window = callback;
		  inev.arg = Fcons (action, fname);
		  inev.arg = list3 (make_pointer_integer ((*ns).desc),
				    action, fname);
		  kbd_buffer_store_event_hold (&raw mut inev, hold_quit);
		  nevents += 1;
		  if (*fni).NextEntryOffset == 0
		    { break; }
		  p = p.offset ((*fni).NextEntryOffset as isize);
		  fni = p as *mut FILE_NOTIFY_INFORMATION;
		  info_size = info_size.wrapping_sub ((*fni).NextEntryOffset);
		}
	    }
	  /* Free this notification set.  */
	  free ((*ns).notifications as *mut c_void);
	  free (ns as *mut c_void);
	}
    }
  return nevents;
}

/* C 733-739: `#else  /* !HAVE_W32NOTIFY */`'s twin - the arm the frozen configuration compiles.  */
#[cfg (all (feature = "windowsnt", not (feature = "have_w32notify")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_file_notifications (hold_quit: *mut input_event) -> c_int
{
  return 0;
}

/* C 741-771: the file's own overview of console input on MS-Windows, between
   `handle_file_notifications` and the reader.  C's words are carried:  */

/* Here's an overview of how Emacs input works in non-GUI sessions on
   MS-Windows.  (For description of the GUI input, see the commentary
   before w32_msg_pump in w32fns.c.)

   When Emacs is idle, it loops inside wait_reading_process_output,
   calling pselect periodically to check whether any input is
   available.  On Windows, pselect is redirected to sys_select, which
   uses MsgWaitForMultipleObjects to wait for input, either from the
   keyboard or from any of the Emacs subprocesses.  In addition,
   MsgWaitForMultipleObjects wakes up when some Windows message is
   posted to the input queue of the Emacs's main thread (which is the
   thread in which sys_select runs).

   When the Emacs's console window has focus, Windows sends input
   events that originate from the keyboard or the mouse; these events
   wake up MsgWaitForMultipleObjects, which reports that input is
   available.  Emacs then calls w32_console_read_socket, below, to
   read the input.  w32_console_read_socket uses
   GetNumberOfConsoleInputEvents and ReadConsoleInput to peek at and
   read the console input events.

   One type of non-keyboard input event that gets reported as input
   available is due to the Emacs's console window receiving focus.
   When that happens, Emacs gets the FOCUS_EVENT event and sys_select
   reports some input; however, w32_console_read_socket ignores such
   events when called to read them.

   Note that any other Windows message sent to the main thread will
   also wake up MsgWaitForMultipleObjects.  These messages get
   immediately dispatched to their destinations by calling
   drain_message_queue.  */

/* C 773-862: `int w32_console_read_socket (struct terminal *terminal, struct input_event
   *hold_quit)` - the second entry point `w32inevt.h` declares (C 25-26).  C's `for (;;)` is the
   `loop` and C's `break` out of it is the same `break`; `evtype` keeps C's `unsigned` and the
   `EventType` field's `WORD` values are widened to it (C's own conversion), because the switch
   labels are that field's values.  `terminal->display_info.tty->mouse_highlight` is C's two-hop
   chain; the union member access and the record's field are one place expression.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_console_read_socket (terminal: *mut terminal,
						  hold_quit: *mut input_event) -> c_int
{
  let mut nev: c_int = 0;
  let mut add: c_int = 0;
  let mut isdead: c_int = 0;

  block_input ();

  loop
    {
      let nfnotify: c_int = handle_file_notifications (hold_quit);

      nev = fill_queue (0);
      if nev <= 0
	{
	  /* If nev == -1, there was some kind of error
	     If nev == 0 then no events were available
	     so return.  */
	  if nfnotify != 0
	    { nev = 0; }
	  break;
	}

      while nev > 0
	{
	  let mut inev: input_event = core::mem::zeroed ();
	  /* Having a separate variable with this value makes
	     debugging easier, as otherwise the compiler might
	     rearrange the switch below in a way that makes it hard to
	     track the event type.  */
	  let evtype: c_uint = (*queue_ptr).EventType as c_uint;

	  EVENT_INIT (&raw mut inev);
	  input_event_bits! (kind, &raw mut inev, NO_EVENT);
	  inev.arg = Qnil ();

	  match evtype
	    {
	      KEY_EVENT =>
		{
		  add = key_event (&raw mut (*queue_ptr).Event.KeyEvent, &raw mut inev,
				   &raw mut isdead);
		  if add == -1	/* 95.7.25 by himi */
		    {
		      queue_ptr = queue_ptr.offset (-1);
		      add = 1;
		    }
		  if add != 0
		    {
		      /* `terminal->display_info.tty->mouse_highlight`: the union member is the
			 mirror's untyped `*mut c_void` (P-72 rule 3), so C's pointer chain needs the
			 cast C's own prototype implies (the `MOUSE_HL_INFO` body's shape).  */
		      let hlinfo: *mut Mouse_HLInfo
			= &raw mut (*((*terminal).display_info.tty
				      as *mut tty_display_info)).mouse_highlight;
		      if ! mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)
			{
			  clear_mouse_face (hlinfo);
			  mouse_face_hidden! (hlinfo, 1);
			}
		      kbd_buffer_store_event_hold (&raw mut inev, hold_quit);
		    }
		}

	      MOUSE_EVENT =>
		{
		  add = do_mouse_event (&raw mut (*queue_ptr).Event.MouseEvent, &raw mut inev);
		  if add != 0
		    { kbd_buffer_store_event_hold (&raw mut inev, hold_quit); }
		}

	      WINDOW_BUFFER_SIZE_EVENT =>
		{
		  if w32_use_full_screen_buffer
		    { resize_event (&raw mut (*queue_ptr).Event.WindowBufferSizeEvent); }
		}

	      _ =>	/* C 843-846's `case MENU_EVENT: case FOCUS_EVENT:` - internal
			   event types, ignored.  */
		{
		  /* Internal event types, ignored. */
		}
	    }

	  queue_ptr = queue_ptr.offset (1);
	  nev -= 1;
	}
    }

  /* We don't get told about changes in the window size (only the buffer
     size, which we no longer care about), so we have to check it
     periodically.  */
  if ! w32_use_full_screen_buffer
    { maybe_generate_resize_event (); }

  unblock_input ();
  return nev;
}
