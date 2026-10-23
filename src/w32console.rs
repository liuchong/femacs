// femacs: port of src/w32console.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32console.o`
// is added to `W32_OBJ` by `configure.ac` 3051 - the `else` arm of the Cygwin split inside the
// `HAVE_W32` test (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` - and
// `docs/baseline/frozen-build-sources.txt` lists no `w32console.o` for this target, so
// `extract-abi.py`'s checklist for the file is empty (P-86 (2)) and the ledger
// `docs/ports/w32console.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*
//     (P-86 (8)), which is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`,
//     not `HAVE_NTGUI`: the two macros separate different things - `HAVE_NTGUI` is on for *both*
//     the Cygwin and the native Windows GUI build (3024), while `w32console.o` joins `W32_OBJ`
//     only in the non-Cygwin arm (3051), which is exactly the build whose `nt/inc/ms-w32.h` 23-25
//     `#define`s `WINDOWSNT`.  The three units of that same arm already landed with this gate
//     (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`), so C 42-44's `#ifdef WINDOWSNT` include
//     edge needs no feature of its own: the file gate *is* that arm;
//   * C 73-75 is `#ifndef USE_SEPARATE_SCREEN` around one static, and C 682-688 / C 1030-1046 are
//     two `#ifdef USE_SEPARATE_SCREEN`/`#else`/`#endif` pairs; `USE_SEPARATE_SCREEN` is not in
//     `docs/baseline/config/darwin-arm64.macros.tsv` (undefined), so the `#else` arms are the
//     frozen ones and **both arms of each pair are carried** under `feature =
//     "use_separate_screen"` (P-51: the feature names the non-frozen side);
//   * `tools/check-cond-eval.py src/w32console.c` reads **`4 conditions, 0 unknown, 0
//     disagreements [VERIFIED]`**, and the four are exactly C 42, C 73, C 682 and C 1030 - so
//     that is the whole conditional text of the file.
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty,
// no Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wincon.h`), so there is **no
// `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
// measurement: every record and every value of the `<windows.h>` stand-in section below is
// `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are owed.
// What the round does carry is the frozen `build-tu` reading, the machine-checked coverage table,
// the `#if` verdicts, a per-item C citation and the `probe` reading of the `windowsnt` +
// `have_ntgui` + `have_window_system` arm.
//
// `docs/ports/w32console.md` carries the C line ranges, the acceptance readings, every deviation
// (the dropped forward declarations, the dropped `w32.h` include edge, the in-place `<windows.h>`
// surface, the in-unit declarations no mirror carries, the `face` record the unit declares itself,
// the `LEFT`/`RIGHT` re-definition, the `atoi`/`exit`/`min` use-site declarations) and the items
// the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-17 (with C 18's blank separator) is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
   first line is the revision line above.

   C 19-25 is the file's own second comment - the three authors and the two-line note about the
   Virtual Terminal Sequences.  C's words are carried:  */

/*
   Tim Fleehart (apollo@online.com)		1-17-92
   Geoff Voelker (voelker@cs.washington.edu)	9-12-93
   Ewan Townshend (ewan@etown.dev)              2025-08
   * c. ~ 2025: 24bit RGB support in Windows (10+) Terminal
   * https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
*/

/* C 28-44: the include graph.  `w32console.o` is built only by the native Windows GUI
   configuration, so the whole graph sits under the file gate (P-86 (8): with the includes
   outside it the frozen compile immediately eats `E0428`s).  C 31's `<windows.h>` has no mirror
   in this tree (P-88 (1): the declaration site is C's own `#include` line, and no file is added
   under `src/include/`), so the names *this file* takes from it stand in the section
   below.  */
#[cfg (feature = "windowsnt")]
include!("include/config.rs");                  // C 28: `#include <config.h>`
/* C 28's AH_BOTTOM: `#include <conf_post.h>` (`DebPrint`, the `eassert` guards).  */
#[cfg (feature = "windowsnt")]
include!("include/conf_post.rs");
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");            // C 30: `#include <stdio.h>` (`printf`, `fflush`, `stdout`, `snprintf`)

/* ----------------------------------------------------------------------------------------------
   C 31: `#include <windows.h>`.

   The names this file takes from it stand here, each with the SDK's own text above it.  The
   spellings repeat `src/w32.rs`'s and `src/w32uniscribe.rs`'s sections, so that a Windows
   translation unit sees one set of names: `DWORD` is `u32`, `WORD` is `c_ushort`, `BOOL` is C's
   `int`, `HANDLE`/`HWND` are `void *`, `LPCSTR` is `const char *`, and `WINAPI`/`CALLBACK` are
   the `extern "system"` ABI Rust spells `__stdcall` with.

   **No measurement was possible**: this machine has no `<windows.h>` (see the file note), so every
   scalar and every record below is **UNVERIFIED** - its size, offsets and values are the
   documented ones and the layout assertions are owed (the ledger's `## Owed`).  The values are
   `<windef.h>`/`<wincon.h>`'s.
   ---------------------------------------------------------------------------------------------- */

/// C 31's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned long ULONG;
/// C: typedef unsigned int UINT; typedef int BOOL; typedef short SHORT; typedef char CHAR;
#[cfg (feature = "windowsnt")]
pub type BYTE = u8;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type WCHAR = c_ushort;
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
/// the implicit conversions at the assignment sites need no cast.
/// C: typedef void *HANDLE; typedef void *HWND; typedef void *LPVOID; typedef void *HGDIOBJ;
/// C: typedef const char *LPCSTR; typedef char *LPSTR; typedef const void *LPCVOID;
/// C: typedef DWORD *LPDWORD; typedef unsigned char *LPBYTE; typedef LONG_PTR LPARAM;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HWND = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HGDIOBJ = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPCVOID = *const c_void;
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;
#[cfg (feature = "windowsnt")]
pub type LPBYTE = *mut u8;
#[cfg (feature = "windowsnt")]
pub type LPARAM = isize;
/// C's `WINAPI`/`CALLBACK` are `__stdcall`; the note stays on the declarations below.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
/// C: typedef int (CALLBACK *WNDENUMPROC) (HWND, LPARAM);
#[cfg (feature = "windowsnt")]
pub type WNDENUMPROC = Option<unsafe extern "system" fn (HWND, LPARAM) -> BOOL>;

/// C 31's `<windows.h>` (continued): the console records (`<wincon.h>`).
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
/// C: typedef struct _CHAR_INFO { union { WCHAR UnicodeChar; CHAR AsciiChar; } Char; WORD Attributes; } CHAR_INFO;
/// The anonymous union member keeps C's name `Char`; the union itself is named
/// `<struct>_<member>` (the P-67 convention `include/dispextern.rs` uses for `glyph_slice`).
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHAR_INFO_Char
{
  pub UnicodeChar: WCHAR,
  pub AsciiChar: CHAR,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHAR_INFO
{
  pub Char: CHAR_INFO_Char,
  pub Attributes: WORD,
}
/// C: typedef struct _CONSOLE_SCREEN_BUFFER_INFO { COORD dwSize; COORD dwCursorPosition; WORD wAttributes; SMALL_RECT srWindow; COORD dwMaximumWindowSize; } CONSOLE_SCREEN_BUFFER_INFO;
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
/// C: typedef struct _CONSOLE_CURSOR_INFO { DWORD dwSize; BOOL bVisible; } CONSOLE_CURSOR_INFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_CURSOR_INFO
{
  pub dwSize: DWORD,
  pub bVisible: BOOL,
}

/// C 31's `<windows.h>` (continued): the console entry points this file calls, all `<wincon.h>`'s.
/// `WriteConsoleOutputCharacter`/`FillConsoleOutputCharacter` are the names C's text writes - the
/// `<wincon.h>` macros select the `A` entry point when `UNICODE` is not defined, which is the
/// MinGW build's case - so the port declares the name C writes, with the `LPCSTR`/`CHAR`
/// signature that arm has.
/// C: BOOL WINAPI WriteConsole (HANDLE, const void *, DWORD, LPDWORD, LPVOID);
/// C: BOOL WINAPI WriteConsoleOutputCharacter (HANDLE, LPCSTR, DWORD, COORD, LPDWORD);
/// C: BOOL WINAPI FillConsoleOutputCharacter (HANDLE, CHAR, DWORD, COORD, LPDWORD);
/// C: BOOL WINAPI FillConsoleOutputAttribute (HANDLE, WORD, DWORD, COORD, LPDWORD);
/// C: BOOL WINAPI ScrollConsoleScreenBuffer (HANDLE, const SMALL_RECT *, const SMALL_RECT *, COORD, const CHAR_INFO *);
/// C: BOOL WINAPI GetConsoleScreenBufferInfo (HANDLE, PCONSOLE_SCREEN_BUFFER_INFO);
/// C: BOOL WINAPI GetConsoleCursorInfo (HANDLE, PCONSOLE_CURSOR_INFO);
/// C: BOOL WINAPI SetConsoleCursorInfo (HANDLE, const CONSOLE_CURSOR_INFO *);
/// C: BOOL WINAPI SetConsoleCursorPosition (HANDLE, COORD);
/// C: BOOL WINAPI SetConsoleWindowInfo (HANDLE, BOOL, const SMALL_RECT *);
/// C: BOOL WINAPI SetConsoleScreenBufferSize (HANDLE, COORD);
/// C: HANDLE WINAPI CreateConsoleScreenBuffer (DWORD, DWORD, const SECURITY_ATTRIBUTES *, DWORD, LPVOID);
/// C: BOOL WINAPI SetConsoleActiveScreenBuffer (HANDLE);
/// C: BOOL WINAPI GetConsoleMode (HANDLE, LPDWORD);
/// C: BOOL WINAPI SetConsoleMode (HANDLE, DWORD);
/// C: HANDLE WINAPI GetStdHandle (DWORD);
/// C: DWORD WINAPI GetLastError (void);
/// C: int WINAPI GetClassName (HWND, LPSTR, int);
/// C: BOOL WINAPI EnumThreadWindows (DWORD, WNDENUMPROC, LPARAM);
/// C: DWORD WINAPI GetCurrentThreadId (void);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn WriteConsole (h: HANDLE, buf: LPCVOID, n: DWORD, written: LPDWORD,
                       reserved: LPVOID) -> BOOL;
  pub fn WriteConsoleOutputCharacter (h: HANDLE, s: LPCSTR, n: DWORD, pos: COORD,
                                     written: LPDWORD) -> BOOL;
  pub fn FillConsoleOutputCharacter (h: HANDLE, c: CHAR, n: DWORD, pos: COORD,
                                     written: LPDWORD) -> BOOL;
  pub fn FillConsoleOutputAttribute (h: HANDLE, attr: WORD, n: DWORD, pos: COORD,
                                     written: LPDWORD) -> BOOL;
  pub fn ScrollConsoleScreenBuffer (h: HANDLE, scroll: *const SMALL_RECT,
                                    clip: *const SMALL_RECT, dest: COORD,
                                    fill: *const CHAR_INFO) -> BOOL;
  pub fn GetConsoleScreenBufferInfo (h: HANDLE, info: *mut CONSOLE_SCREEN_BUFFER_INFO) -> BOOL;
  pub fn GetConsoleCursorInfo (h: HANDLE, info: *mut CONSOLE_CURSOR_INFO) -> BOOL;
  pub fn SetConsoleCursorInfo (h: HANDLE, info: *const CONSOLE_CURSOR_INFO) -> BOOL;
  pub fn SetConsoleCursorPosition (h: HANDLE, pos: COORD) -> BOOL;
  pub fn SetConsoleWindowInfo (h: HANDLE, absolute: BOOL, window: *const SMALL_RECT) -> BOOL;
  pub fn SetConsoleScreenBufferSize (h: HANDLE, size: COORD) -> BOOL;
  pub fn CreateConsoleScreenBuffer (access: DWORD, share: DWORD,
                                    security: *const c_void, type_: DWORD,
                                    reserved: LPVOID) -> HANDLE;
  pub fn SetConsoleActiveScreenBuffer (h: HANDLE) -> BOOL;
  pub fn GetConsoleMode (h: HANDLE, mode: LPDWORD) -> BOOL;
  pub fn SetConsoleMode (h: HANDLE, mode: DWORD) -> BOOL;
  pub fn GetStdHandle (std_handle: DWORD) -> HANDLE;
  pub fn GetLastError () -> DWORD;
  pub fn GetClassName (h: HWND, name: LPSTR, max_count: c_int) -> c_int;
  pub fn EnumThreadWindows (thread_id: DWORD, cb: WNDENUMPROC, lparam: LPARAM) -> BOOL;
  pub fn GetCurrentThreadId () -> DWORD;
}

/// C 31's `<windows.h>` (continued): the `<wincon.h>` constants this file writes.  C's
/// `STD_INPUT_HANDLE`/`STD_OUTPUT_HANDLE` are `((DWORD) -10)`/`((DWORD) -11)`; the port spells the
/// same 32-bit value.
/// C: #define STD_INPUT_HANDLE ((DWORD)-10)
/// C: #define STD_OUTPUT_HANDLE ((DWORD)-11)
/// C: #define ENABLE_PROCESSED_OUTPUT 0x0001
/// C: #define ENABLE_WRAP_AT_EOL_OUTPUT 0x0002
/// C: #define ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004
/// C: #define DISABLE_NEWLINE_AUTO_RETURN 0x0008
/// C: #define ENABLE_MOUSE_INPUT 0x0010
/// C: #define ENABLE_INSERT_MODE 0x0020
/// C: #define ENABLE_QUICK_EDIT_MODE 0x0040
/// C: #define ENABLE_EXTENDED_FLAGS 0x0080
/// C: #define ENABLE_WINDOW_INPUT 0x0008
/// C: #define ENABLE_VIRTUAL_TERMINAL_INPUT 0x0200
/// C: #define GENERIC_READ 0x80000000L
/// C: #define GENERIC_WRITE 0x40000000L
/// C: #define CONSOLE_TEXTMODE_BUFFER 1
/// C: #define INVALID_HANDLE_VALUE ((HANDLE)(LONG_PTR)-1)
/// C: #define CTRL_C_EVENT 0
/// C: #define CTRL_BREAK_EVENT 1
#[cfg (feature = "windowsnt")]
pub const STD_INPUT_HANDLE: DWORD = -10i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
#[cfg (feature = "windowsnt")]
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
#[cfg (feature = "windowsnt")]
pub const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;
#[cfg (feature = "windowsnt")]
pub const ENABLE_MOUSE_INPUT: DWORD = 0x0010;
#[cfg (feature = "windowsnt")]
pub const ENABLE_INSERT_MODE: DWORD = 0x0020;
#[cfg (feature = "windowsnt")]
pub const ENABLE_QUICK_EDIT_MODE: DWORD = 0x0040;
#[cfg (feature = "windowsnt")]
pub const ENABLE_EXTENDED_FLAGS: DWORD = 0x0080;
#[cfg (feature = "windowsnt")]
pub const ENABLE_WINDOW_INPUT: DWORD = 0x0008;
#[cfg (feature = "windowsnt")]
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
#[cfg (feature = "windowsnt")]
pub const GENERIC_READ: DWORD = 0x80000000;
#[cfg (feature = "windowsnt")]
pub const GENERIC_WRITE: DWORD = 0x40000000;
#[cfg (feature = "windowsnt")]
pub const CONSOLE_TEXTMODE_BUFFER: DWORD = 1;
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
#[cfg (feature = "windowsnt")]
pub const CTRL_C_EVENT: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub const CTRL_BREAK_EVENT: DWORD = 1;

/* C 33-40: the reference headers.  `lisp.rs` brings its own `<stdcountof.h>`/`<string.h>`/
   `<stdlib.h>` chain, and `termchar.h` 23 `#include`s `dispextern.h` and `w32term.h` 22
   `#include`s `frame.h`/`window.h`/`termhooks.h`/`w32gui.h`, so the mirrors below name exactly the
   headers C names - the nestings are the graph `xtask` expands and dedupes (PORTING.md 0.1-4).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdcountof.rs");       // lisp.h 28: `#include <stdcountof.h>` (`countof`, C 218)
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");           // lisp.h: `#include <stdlib.h>` (`getenv`, C 1057)
#[cfg (feature = "windowsnt")]
include!("../lib/include/min-max.rs");          // lisp.h 71-73: `min` (C 1074-1075)
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                    // C 33: `#include "lisp.h"`
#[cfg (feature = "windowsnt")]
include!("include/coding.rs");                  // C 34: `#include "coding.h"` (`struct coding_system`)
#[cfg (feature = "windowsnt")]
include!("include/termchar.rs");                // C 35: `#include "termchar.h"`	/* for FRAME_TTY */
#[cfg (feature = "windowsnt")]
include!("include/dispextern.rs");              // C 36: `#include "dispextern.h"`	/* for tty_defined_color */
#[cfg (feature = "windowsnt")]
include!("include/menu.rs");                    // C 37: `#include "menu.h"`	/* for tty_menu_show */
#[cfg (feature = "windowsnt")]
include!("include/w32term.rs");                 // C 38: `#include "w32term.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");               // C 39: `#include "w32common.h"`	/* for os_subtype */
#[cfg (feature = "windowsnt")]
include!("include/w32inevt.rs");                // C 40: `#include "w32inevt.h"`

/* C 42-44: `#ifdef WINDOWSNT / #include "w32.h" / #endif`.

   The edge is **dropped, and this is the reason**: the include is C's forward declaration of
   `syms_of_ntterm` ("for syms_of_ntterm", C 43), which this very unit *defines* at C 1240 - and
   `include/w32.rs` declares that name in its **top-level** `unsafe extern "C"` block (C 275-490),
   not in one of the module + glob-import blocks it uses for the names units define, so a unit that
   both includes the mirror and defines the function is `E0428 the name syms_of_ntterm is defined
   multiple times`.  C's own declaration is redundant here; the alternate arm is C's empty one.
   The ledger's `## Owed` names the mirror row that would let the edge return.  */

/* C 46-62: the fifteen `static` prototypes.  Dropped (PORTING.md 0.1-12: Rust has no static
   function prototypes, and every one of them is defined in this file).  Their definitions follow
   in C's order.  */

/* C 64-75: the file's own static state.  C's tentative definitions (no initializer) are the
   port's explicit zero initializers (PORTING.md 1.5).  */
#[cfg (feature = "windowsnt")]
static mut cursor_coords: COORD = COORD { X: 0, Y: 0 };
#[cfg (feature = "windowsnt")]
static mut prev_screen: HANDLE = std::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut cur_screen: HANDLE = std::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut char_attr_normal: WORD = 0;
#[cfg (feature = "windowsnt")]
static mut prev_console_mode: DWORD = 0;
#[cfg (feature = "windowsnt")]
static mut prev_output_mode: DWORD = 0;
#[cfg (feature = "windowsnt")]
static mut bg_normal: c_int = 0;
#[cfg (feature = "windowsnt")]
static mut fg_normal: c_int = 0;

#[cfg (feature = "windowsnt")]
static mut console_cursor_info: CONSOLE_CURSOR_INFO = CONSOLE_CURSOR_INFO { dwSize: 0, bVisible: 0 };
/* C 73-75: `#ifndef USE_SEPARATE_SCREEN / static CONSOLE_CURSOR_INFO prev_console_cursor; /
   #endif`.  The `#else` arm is C's absent one.  */
#[cfg (all (feature = "windowsnt", not (feature = "use_separate_screen")))]
static mut prev_console_cursor: CONSOLE_CURSOR_INFO
  = CONSOLE_CURSOR_INFO { dwSize: 0, bVisible: 0 };

/* C 77-83: two `extern` declarations each followed by the definition C writes for the same name
   (`keyboard_handle` C 77-78, `current_tty` C 82-83) - the declarations are dropped (0.1-12) and
   the definitions keep C's non-`static` linkage.  */
/// C 78: `extern HANDLE keyboard_handle;` is C 77's own declaration of the object defined here;
/// `src/w32fns.rs` 965-968 and `src/w32inevt.c`/`src/w32proc.c`/`src/w32xfns.c` declare the same
/// name, and this is its single definition (PORTING.md 0.1-3).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut keyboard_handle: HANDLE = std::ptr::null_mut ();
/// C 79: defined by `src/w32console.c`, declared by `w32inevt.h` 22 and read by `src/w32inevt.c`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut w32_console_unicode_input: c_int = 0;
/// C 80-81: `extern int w32_use_virtual_terminal; int w32_use_virtual_terminal = 1;`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut w32_use_virtual_terminal: c_int = 1;
/// C 83: `struct tty_display_info *current_tty = NULL;` - the object `cm.h` 165 declares.  The w32
/// build is `CM_OBJ=` (configure.ac 3071), so `src/cm.c`'s own definition is not linked and this
/// one is the live one; `src/cm.rs` 75 is the other configuration's.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut current_tty: *mut tty_display_info = std::ptr::null_mut ();

/* ----------------------------------------------------------------------------------------------
   Use-site declarations for the names this file calls that no mirror on its include graph
   carries (P-66/P-72 rule 4, the `src/term.rs` 150-215 / `src/w32term.rs` 1750-1770 shape).  Each
   keeps C's own prototype in a `/// C:` line, and the ledger's `## Owed` names the mirror rows
   that would let them retire.
   ---------------------------------------------------------------------------------------------- */

/// C 999's `&tty_defined_color`: declared by `dispextern.h` 3796, defined by `src/xfaces.c`, and
/// not carried by `include/dispextern.rs` yet (the `term.rs` 197 copy is the sibling).
/// C: extern bool tty_defined_color (struct frame *, const char *, Emacs_Color *, bool, bool);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn tty_defined_color (f: *mut frame, color_name: *const c_char, color_def: *mut Emacs_Color,
                        alloc_p: bool, _default: bool) -> bool;
}

/// C 558-559's `root_xy` and C 545's `root_frame`: declared by `dispextern.h` 4020 and
/// `frame.h` 1508, defined by `src/dispnew.c` 3662 and `src/frame.rs` 2517's C file.
/// C: void root_xy (struct frame *f, int x, int y, int *rx, int *ry);
/// C: extern struct frame *root_frame (struct frame *f);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn root_xy (f: *mut frame, x: c_int, y: c_int, root_x: *mut c_int, root_y: *mut c_int);
  fn root_frame (f: *mut frame) -> *mut frame;
}

/// C 603's `write_glyphs` and C 421/492's `encode_terminal_code`: declared by `dispextern.h`
/// 3942 and `termhooks.h` 973, both defined by `src/term.c` (`src/term.rs` 279 declares the first
/// at its own use site, 863 defines the second).
/// C: extern void write_glyphs (struct frame *, struct glyph *, int);
/// C: extern unsigned char *encode_terminal_code (struct glyph *, int, struct coding_system *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn write_glyphs (f: *mut frame, string: *mut glyph, len: c_int);
  fn encode_terminal_code (src: *mut glyph, src_len: c_int, coding: *mut coding_system)
    -> *mut u8;
}

/// C 1041's `exit` and C 1065-1066's `atoi`: `<stdlib.h>`'s, which `lib/include/stdlib.rs` does
/// not carry (it has `getenv`, `strtol`, the allocators).
/// C: void exit (int);
/// C: int atoi (const char *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  fn exit (status: c_int) -> !;
  fn atoi (nptr: *const c_char) -> c_int;
}

/* C 85-97: the console control handler.  C's prototype (C 89) is the definition's own; the
   definition keeps C's external linkage, and `src/w32proc.rs` 566-572 declares it (C 115's own
   `extern BOOL ctrl_c_handler (unsigned long type);`).  */
/// C 91-97: `BOOL ctrl_c_handler (unsigned long type)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctrl_c_handler (type_: c_ulong) -> BOOL
{
  /* Only ignore "interrupt" events when running interactively.  */
  return if ! noninteractive
	   && (type_ == CTRL_C_EVENT as c_ulong || type_ == CTRL_BREAK_EVENT as c_ulong)
    { TRUE } else { FALSE };
}

/* C 99-102: `#define DEFAULTP(p) (p == FACE_TTY_DEFAULT_COLOR || p == FACE_TTY_DEFAULT_FG_COLOR
   || p == FACE_TTY_DEFAULT_BG_COLOR)`.  The three sentinels are `dispextern.h` 1932-1940's, and
   `include/dispextern.rs` keeps them in `dispextern_unit_names` (a module it does not glob-import,
   because each unit declares them itself - `src/term.rs` 293-295 is the sibling copy), so the pair
   of declarations below is that unit-side copy.  */
/// C: #define FACE_TTY_DEFAULT_COLOR ((unsigned long) -1)
/// C: #define FACE_TTY_DEFAULT_FG_COLOR ((unsigned long) -2)
/// C: #define FACE_TTY_DEFAULT_BG_COLOR ((unsigned long) -3)
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_DEFAULT_COLOR: c_ulong = -1i64 as c_ulong;
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_DEFAULT_FG_COLOR: c_ulong = -2i64 as c_ulong;
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_DEFAULT_BG_COLOR: c_ulong = -3i64 as c_ulong;
/// C 99-102: C's three-way `==` chain, one `logic-or` of three comparisons.
/// C: #define DEFAULTP(p) (p == FACE_TTY_DEFAULT_COLOR || p == FACE_TTY_DEFAULT_FG_COLOR || p == FACE_TTY_DEFAULT_BG_COLOR)
#[cfg (feature = "windowsnt")]
macro_rules! DEFAULTP {
  ($p:expr) => { $p == FACE_TTY_DEFAULT_COLOR
                 || $p == FACE_TTY_DEFAULT_FG_COLOR
                 || $p == FACE_TTY_DEFAULT_BG_COLOR };
}

/* C 104: `#define SEQMAX 256 /* Arbitrary upper limit on VT sequence size */`  */
/// C: #define SEQMAX 256
#[cfg (feature = "windowsnt")]
const SEQMAX: usize = 256;

/* C 106-111: C's variadic `do { … } while (0)` statement macro.  The port writes the one-shot
   block C's loop encloses (PORTING.md 2.2's `do…while (0)` class, the `countof`/`RULE` shape), so
   `do`/`while` and the trailing `0` have no counterpart token; C's `__VA_ARGS__` is the
   `$($args:expr),*` tail.  C's `*i < sz` and `*i += snprintf (…)` mix `DWORD` (`u32`) with
   `size_t` (`usize`), so the width the C compiler applies implicitly is written out (`as usize`,
   and the `wrapping_add` that is C's unsigned `+=` of a possibly-negative `int`).  */
/// C: #define SSPRINTF(buf, i, sz, fmt, ...)					\
/// C:   do {									\
/// C:     eassert (*i < sz && sz <= SEQMAX);					\
/// C:     if (fmt && *i < sz && sz <= SEQMAX)				\
/// C:       *i += snprintf (buf + *i, sz - *i, fmt, __VA_ARGS__);		\
/// C:   } while (0)
#[cfg (feature = "windowsnt")]
macro_rules! SSPRINTF {
  ($buf:expr, $i:expr, $sz:expr, $fmt:expr $(, $args:expr)* $(,)?) => { {
    eassert! (*$i < ($sz as DWORD) && (($sz as usize) <= SEQMAX));
    if ! ($fmt).is_null () && *$i < ($sz as DWORD) && (($sz as usize) <= SEQMAX)
      {
        *$i = (*$i).wrapping_add (snprintf (($buf).as_mut_ptr ().add (*$i as usize),
                                           ($sz as usize) - (*$i as usize), $fmt
                                           $(, $args)*) as DWORD);
      }
  } };
}

/* Writes virtual terminal sequence to screen */
/// C 114-122: `static int w32con_write_vt_seq (const char *seq)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32con_write_vt_seq (seq: *const c_char) -> c_int
{
  let mut buf: [c_char; SEQMAX] = [0; SEQMAX];                          /* C 117 */
  let mut n: DWORD = 0;                                                 /* C 118 */
  let mut k: DWORD = 0;
  SSPRINTF! (buf, &mut n, SEQMAX, seq, std::ptr::null_mut::<c_void> ());            /* C 119 */
  if n != 0
    { WriteConsole (cur_screen, buf.as_ptr () as LPCSTR as LPCVOID, n, &mut k,
                    std::ptr::null_mut ()); }                           /* C 120 */
  return k as c_int;                                                    /* C 121 */
}

/// C 124-130: `static COORD w32con_get_cursor_coords (void)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32con_get_cursor_coords () -> COORD
{
  let mut info: CONSOLE_SCREEN_BUFFER_INFO = core::mem::zeroed ();       /* C 127 */
  GetConsoleScreenBufferInfo (GetStdHandle (STD_OUTPUT_HANDLE), &mut info);  /* C 128 */
  return info.dwCursorPosition;                                         /* C 129 */
}

/* Move the cursor to (ROW, COL) on FRAME.  */
/// C 133-142: `static void w32con_move_cursor (struct frame *f, int row, int col)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_move_cursor (f: *mut frame, row: c_int, col: c_int)
{
  cursor_coords.X = col as SHORT;                                       /* C 136 */
  cursor_coords.Y = row as SHORT;                                       /* C 137 */

  /* TODO: for multi-tty support, cur_screen should be replaced with a
     reference to the terminal for this frame.  */
  SetConsoleCursorPosition (cur_screen, cursor_coords);                 /* C 141 */
}

/// C 144-150: `void w32con_hide_cursor (void)` (`w32term.c` defines the non-Windows names
/// `tty_hide_cursor`/`tty_show_cursor`; `src/term.rs` calls these two).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32con_hide_cursor ()
{
  GetConsoleCursorInfo (cur_screen, &mut console_cursor_info);          /* C 147 */
  console_cursor_info.bVisible = FALSE;                                 /* C 148 */
  SetConsoleCursorInfo (cur_screen, &console_cursor_info);              /* C 149 */
}

/// C 152-158: `void w32con_show_cursor (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32con_show_cursor ()
{
  GetConsoleCursorInfo (cur_screen, &mut console_cursor_info);          /* C 155 */
  console_cursor_info.bVisible = TRUE;                                  /* C 156 */
  SetConsoleCursorInfo (cur_screen, &console_cursor_info);              /* C 157 */
}

/// C 160-183: `static void w32con_set_cursor_size (EMACS_INT esize)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32con_set_cursor_size (esize: EMACS_INT)
{
  if w32_use_virtual_terminal != 0 && ! w32__terminal_is_conhost         /* C 164 */
    {
      /* The Virtual Terminal Sequences don't support arbitrary sizes of
         the block and underline cursor, so we can only provide 2
         extreme shapes.  */
      if esize < 50                                                      /* C 169 */
	{ w32con_write_vt_seq (c"\x1b[3 q".as_ptr ()); }
      else
	{ w32con_write_vt_seq (c"\x1b[1 q".as_ptr ()); }
    }
  else
    {
      /* Use legacy APIs, which on Windows Terminal don't change the
         cursor shape.  */
      let mut cci: CONSOLE_CURSOR_INFO = core::mem::zeroed ();           /* C 178 */
      cci.dwSize = esize as DWORD;                                       /* C 179 */
      cci.bVisible = TRUE;                                               /* C 180 */
      SetConsoleCursorInfo (cur_screen, &raw const cci);                 /* C 181 */
    }
}

/* Clear from cursor to end of screen.  */
/// C 187-192: `static void w32con_clear_to_end (struct frame *f)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_clear_to_end (f: *mut frame)
{
  w32con_clear_end_of_line (f, FRAME_COLS (f) - 1);                      /* C 190 */
  w32con_ins_del_lines (f, cursor_coords.Y as c_int,
			FRAME_TOTAL_LINES (f) - cursor_coords.Y as c_int - 1);  /* C 191 */
}

/* Clear the frame.  */
/// C 195-213: `static void w32con_clear_frame (struct frame *f)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_clear_frame (f: *mut frame)
{
  let mut dest: COORD = core::mem::zeroed ();                            /* C 198 */
  let mut n: c_int = 0;                                                  /* C 199 */
  let mut r: DWORD = 0;                                                  /* C 200 */
  let mut info: CONSOLE_SCREEN_BUFFER_INFO = core::mem::zeroed ();       /* C 201 */

  GetConsoleScreenBufferInfo (GetStdHandle (STD_OUTPUT_HANDLE), &mut info);  /* C 203 */

  /* Remember that the screen buffer might be wider than the window.  */
  n = FRAME_TOTAL_LINES (f) * info.dwSize.X as c_int;                    /* C 206 */
  dest.X = 0;                                                           /* C 207 */
  dest.Y = 0;                                                           /* C 207 (one chained assignment in C) */

  FillConsoleOutputAttribute (cur_screen, char_attr_normal, n as DWORD, dest, &mut r);  /* C 209 */
  FillConsoleOutputCharacter (cur_screen, b' ' as CHAR, n as DWORD, dest, &mut r);  /* C 210 */

  w32con_move_cursor (f, 0, 0);                                          /* C 212 */
}

/* C 216-219: the "empty row" buffer the clear-to-end-of-line path reuses.  C 218's
   `countof (glyph_base)` is the constant it evaluates to (80); the ledger records why the
   `sizeof` fold cannot be written here (Rust forbids a shared reference to a `static mut` in a
   `static` initializer, E0133 measured).  */
/// C: static struct glyph glyph_base[80];
/// C: static struct glyph *glyphs = glyph_base;
/// C: static size_t glyphs_len = countof (glyph_base);
/// C: static BOOL  ceol_initialized = FALSE;
#[cfg (feature = "windowsnt")]
static mut glyph_base: [glyph; 80] = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut glyphs: *mut glyph
  = unsafe { core::ptr::addr_of_mut! (glyph_base) as *mut glyph };
#[cfg (feature = "windowsnt")]
static mut glyphs_len: size_t = 80;
#[cfg (feature = "windowsnt")]
static mut ceol_initialized: BOOL = FALSE;

/* Clear from Cursor to end (what's "standout marker"?).  */
/// C 222-247: `static void w32con_clear_end_of_line (struct frame *f, int end)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_clear_end_of_line (f: *mut frame, end: c_int)
{
  /* Time to reallocate our "empty row"?  With today's large screens,
     it is not unthinkable to see TTY frames well in excess of
     80-character width.  */
  if ((end - cursor_coords.X as c_int) as size_t) > glyphs_len            /* C 228 */
    {
      if glyphs == core::ptr::addr_of_mut! (glyph_base) as *mut glyph    /* C 230 */
	{ glyphs = std::ptr::null_mut (); }                             /* C 231 */
      glyphs = xrealloc (glyphs as *mut c_void,
			 (FRAME_COLS (f) as size_t) * core::mem::size_of::<glyph> ())
	as *mut glyph;                                                  /* C 232 */
      glyphs_len = FRAME_COLS (f) as size_t;                            /* C 233 */
      ceol_initialized = FALSE;                                         /* C 234 */
    }
  if ceol_initialized == FALSE                                          /* C 236 */
    {
      let mut i: c_int = 0;                                             /* C 238 */
      while (i as size_t) < glyphs_len                                  /* C 239 */
	{
	  memcpy (core::ptr::addr_of_mut! (*glyphs.add (i as usize)) as *mut c_void,
		  core::ptr::addr_of! (space_glyph) as *const c_void,
		  core::mem::size_of::<glyph> ());                      /* C 241 */
	  (*glyphs.add (i as usize)).frame = std::ptr::null_mut ();       /* C 242 */
	  i += 1;
	}
      ceol_initialized = TRUE;                                          /* C 244 */
    }
  w32con_write_glyphs (f, glyphs, end - cursor_coords.X as c_int);      /* C 246 */
}

/* Insert n lines at vpos. if n is negative delete -n lines.  */
/// C 250-316: `static void w32con_ins_del_lines (struct frame *f, int vpos, int n)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_ins_del_lines (f: *mut frame, vpos: c_int, n: c_int)
{
  let mut i: c_int = 0;                                                 /* C 253 */
  let mut nb: c_int = 0;
  let mut scroll: SMALL_RECT = core::mem::zeroed ();                    /* C 254 */
  let mut clip: SMALL_RECT = core::mem::zeroed ();                      /* C 255 */
  let mut dest: COORD = core::mem::zeroed ();                           /* C 256 */
  let mut fill: CHAR_INFO = core::mem::zeroed ();                       /* C 257 */

  if n < 0                                                             /* C 259 */
    {
      scroll.Top = (vpos - n) as SHORT;                                 /* C 261 */
      scroll.Bottom = FRAME_TOTAL_LINES (f) as SHORT;                   /* C 262 */
      dest.Y = vpos as SHORT;                                           /* C 263 */
    }
  else
    {
      scroll.Top = vpos as SHORT;                                       /* C 267 */
      scroll.Bottom = (FRAME_TOTAL_LINES (f) - n) as SHORT;             /* C 268 */
      dest.Y = (vpos + n) as SHORT;                                     /* C 269 */
    }
  clip.Top = 0;                                                        /* C 271 (one chained assignment in C) */
  clip.Left = 0;                                                       /* C 271 (one chained assignment in C) */
  scroll.Left = 0;                                                     /* C 271 (one chained assignment in C) */
  clip.Right = FRAME_COLS (f) as SHORT;                                /* C 272 (one chained assignment in C) */
  scroll.Right = FRAME_COLS (f) as SHORT;                              /* C 272 (one chained assignment in C) */
  clip.Bottom = FRAME_TOTAL_LINES (f) as SHORT;                        /* C 273 */

  dest.X = 0;                                                          /* C 275 */

  fill.Char.AsciiChar = 0x20;                                           /* C 277 */
  fill.Attributes = char_attr_normal;                                   /* C 278 */

  ScrollConsoleScreenBuffer (cur_screen, &scroll, &clip, dest, &fill);  /* C 280 */

  /* Here we have to deal with a w32 console flake: If the scroll
     region looks like abc and we scroll c to a and fill with d we get
     cbd... if we scroll block c one line at a time to a, we get cdd...
     Emacs expects cdd consistently... So we have to deal with that
     here... (this also occurs scrolling the same way in the other
     direction.  */

  if n > 0                                                             /* C 289 */
    {
      if scroll.Bottom < dest.Y                                        /* C 291 */
	{
	  i = scroll.Bottom as c_int;                                  /* C 293 */
	  while i < dest.Y as c_int
	    {
	      w32con_move_cursor (f, i, 0);                            /* C 295 */
	      w32con_clear_end_of_line (f, FRAME_COLS (f));            /* C 296 */
	      i += 1;
	    }
	}
    }
  else
    {
      nb = dest.Y as c_int + (scroll.Bottom as c_int - scroll.Top as c_int) + 1;  /* C 302 */

      if nb < scroll.Top as c_int                                      /* C 304 */
	{
	  i = nb;                                                      /* C 306 */
	  while i < scroll.Top as c_int
	    {
	      w32con_move_cursor (f, i, 0);                            /* C 308 */
	      w32con_clear_end_of_line (f, FRAME_COLS (f));            /* C 309 */
	      i += 1;
	    }
	}
    }

  cursor_coords.X = 0;                                                 /* C 314 */
  cursor_coords.Y = vpos as SHORT;                                     /* C 315 */
}

/* C 318-321: `#undef LEFT / #undef RIGHT / #define LEFT 1 / #define RIGHT 0`.  Rust has no
   `#undef`, and no mirror on this graph declares either name, so C's redefinition is carried as
   the two constants it ends with (the ledger records the two `#undef`s).  */
/// C: #undef	LEFT
/// C: #undef	RIGHT
/// C: #define	LEFT	1
/// C: #define	RIGHT	0
#[cfg (feature = "windowsnt")]
const LEFT: c_int = 1;
#[cfg (feature = "windowsnt")]
const RIGHT: c_int = 0;

/// C 323-354: `static void scroll_line (struct frame *f, int dist, int direction)`.
#[cfg (feature = "windowsnt")]
unsafe fn scroll_line (f: *mut frame, dist: c_int, direction: c_int)
{
  /* The idea here is to implement a horizontal scroll in one line to
     implement delete and half of insert.  */
  let mut scroll: SMALL_RECT = core::mem::zeroed ();                    /* C 328 */
  let mut clip: SMALL_RECT = core::mem::zeroed ();
  let mut dest: COORD = core::mem::zeroed ();                           /* C 329 */
  let mut fill: CHAR_INFO = core::mem::zeroed ();                       /* C 330 */

  scroll.Top = cursor_coords.Y;                                         /* C 332 (one chained assignment in C) */
  clip.Top = cursor_coords.Y;                                           /* C 332 (one chained assignment in C) */
  clip.Bottom = cursor_coords.Y;                                        /* C 332 (one chained assignment in C) */
  scroll.Bottom = cursor_coords.Y;                                      /* C 332 (one chained assignment in C) */
  clip.Left = 0;                                                        /* C 333 */
  clip.Right = FRAME_COLS (f) as SHORT;                                 /* C 334 */

  if direction == LEFT                                                 /* C 336 */
    {
      scroll.Left = (cursor_coords.X as c_int + dist) as SHORT;         /* C 338 */
      scroll.Right = (FRAME_COLS (f) - 1) as SHORT;                     /* C 339 */
    }
  else
    {
      scroll.Left = cursor_coords.X;                                    /* C 343 */
      scroll.Right = (FRAME_COLS (f) - dist - 1) as SHORT;              /* C 344 */
    }

  dest.X = cursor_coords.X;                                             /* C 347 */
  dest.Y = cursor_coords.Y;                                            /* C 348 */

  fill.Char.AsciiChar = 0x20;                                           /* C 350 */
  fill.Attributes = char_attr_normal;                                   /* C 351 */

  ScrollConsoleScreenBuffer (cur_screen, &scroll, &clip, dest, &fill);  /* C 353 */
}

/* coding.h 469-479: `common_flags:14` (bits 0-13) and `mode:5` (bits 14-18) are two bitfields of
   `coding_system.bits`; `include/coding.rs` carries the shift/mask constants (its C 507-516
   section) and the readers, so the writers this unit needs are here with the member name as the
   macro's first token (PORTING.md 1.3/P-80, the `src/term.rs` 823-839 copy).  */
/// C: unsigned int common_flags : 14;
/// C: ENUM_BF (coding_mode) mode : 5;
#[cfg (feature = "windowsnt")]
macro_rules! coding_common_flags {
  (common_flags, $c:expr) =>
    { (((*$c).bits >> CODING_SYSTEM_COMMON_FLAGS_SHIFT) & CODING_SYSTEM_COMMON_FLAGS_MASK) as c_int };
}
#[cfg (feature = "windowsnt")]
macro_rules! coding_mode {
  (mode, $c:expr) =>
    { (((*$c).bits >> CODING_SYSTEM_MODE_SHIFT) & CODING_SYSTEM_MODE_MASK) as c_int };
  (mode, $c:expr, $v:expr) => {{
    let mask = CODING_SYSTEM_MODE_MASK << CODING_SYSTEM_MODE_SHIFT;
    (*$c).bits = ((*$c).bits & ! mask)
      | (((($v) as u32) & CODING_SYSTEM_MODE_MASK) << CODING_SYSTEM_MODE_SHIFT);
  }};
}

/* If start is zero insert blanks instead of a string at start ?. */
/// C 358-376: `static void w32con_insert_glyphs (struct frame *f, struct glyph *start, int len)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_insert_glyphs (f: *mut frame, start: *mut glyph, len: c_int)
{
  scroll_line (f, len, RIGHT);                                          /* C 362 */

  /* Move len chars to the right starting at cursor_coords, fill with blanks */
  if ! start.is_null ()                                                 /* C 365 */
    {
      /* Print the first len characters of start, cursor_coords.X adjusted
	 by write_glyphs.  */

      w32con_write_glyphs (f, start, len);                             /* C 370 */
    }
  else
    {
      w32con_clear_end_of_line (f, cursor_coords.X as c_int + len);     /* C 374 */
    }
}

/// C 378-467: `static void w32con_write_glyphs (struct frame *f, struct glyph *string, int len)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_write_glyphs (f: *mut frame, mut string: *mut glyph, mut len: c_int)
{
  let mut r: DWORD = 0;                                                 /* C 382 */
  let mut conversion_buffer: LPCSTR = std::ptr::null ();                /* C 383 */
  let mut coding: *mut coding_system = std::ptr::null_mut ();           /* C 384 */

  w32con_hide_cursor ();                                                /* C 386 */

  if len <= 0                                                           /* C 388 */
    { return; }

  /* If terminal_coding does any conversion, use it, otherwise use
     safe_terminal_coding.  We can't use CODING_REQUIRE_ENCODING here
     because it always return 1 if the member src_multibyte is 1.  */
  coding = if coding_common_flags! (common_flags,
				    (FRAME_TERMINAL_CODING! (f) as *mut coding_system))
    & CODING_REQUIRE_ENCODING_MASK as c_int != 0
    { FRAME_TERMINAL_CODING! (f) as *mut coding_system }
    else { &raw mut safe_terminal_coding };                             /* C 394-395 */
  /* The mode bit CODING_MODE_LAST_BLOCK should be set to 1 only at
     the tail.  */
  coding_mode! (mode, coding, coding_mode! (mode, coding) & ! CODING_MODE_LAST_BLOCK);  /* C 398 */

  while len > 0                                                         /* C 400 */
    {
      /* Identify a run of glyphs with the same face.  */
      let face_id: c_int = glyph_bits! (face_id, *string);              /* C 403 */
      /* Since this is called to deliver the frame glyph matrix to the
	 glass, some of the glyphs might be from a child frame, which
	 affects the interpretation of face ID.  */
      let face_id_frame: *mut frame = (*string).frame.cast::<frame> (); /* C 407 */
      let mut n: c_int = 1;                                             /* C 408 */

      while n < len                                                     /* C 410 */
	{
	  if ! (glyph_bits! (face_id, *string.add (n as usize)) == face_id
		&& (*string.add (n as usize)).frame == face_id_frame as *mut c_void)
	    { break; }                                                  /* C 411-413 */
	  n += 1;
	}

      /* w32con_clear_end_of_line sets frame of glyphs to NULL.  */
      let attr_frame: *mut frame
	= if ! face_id_frame.is_null () { face_id_frame } else { f };   /* C 416 */

      if n == len                                                       /* C 418 */
	/* This is the last run.  */
	{ coding_mode! (mode, coding, coding_mode! (mode, coding) | CODING_MODE_LAST_BLOCK); }  /* C 420 */
      conversion_buffer = encode_terminal_code (string, n, coding) as LPCSTR;  /* C 421 */
      if (*coding).produced > 0                                         /* C 422 */
	{
	  if w32_use_virtual_terminal != 0                                 /* C 424 */
	    {
	      turn_on_face (f, face_id);                                    /* C 426 */
	      WriteConsole (cur_screen, conversion_buffer as LPCVOID,
			    (*coding).produced as DWORD, &mut r,
			    std::ptr::null_mut ());                         /* C 427-428 */
	      turn_off_face (f, face_id);                                   /* C 429 */
	      cursor_coords = w32con_get_cursor_coords ();                  /* C 430 */
	    }
	  else
	    {
	      /* Account for character width.
		 FIXME: this doesn't handle character compositions.  */
	      let ncols: ptrdiff_t = strwidth ((*coding).source as *const c_char,
					       (*coding).src_bytes);        /* C 436 */

	      /* Turn appearance modes of the face of the run on.  */
	      let char_attr: WORD = w32_face_attributes (attr_frame, face_id);  /* C 439 */

	      /* Set the attribute for these characters.  */
	      if FillConsoleOutputAttribute (cur_screen, char_attr, ncols as DWORD,
					     cursor_coords, &mut r) == 0    /* C 442 */
		{
		  printf (c"Failed writing console attributes: %lu\n".as_ptr (),
			  GetLastError ());                                 /* C 445-446 */
		  fflush (stdout);                                         /* C 447 */
		}

	      /* Write the characters.  */
	      if WriteConsoleOutputCharacter (cur_screen, conversion_buffer,
					      (*coding).produced as DWORD, cursor_coords,
					      &mut r) == 0                  /* C 451 */
		{
		  printf (c"Failed writing console characters: %lu\n".as_ptr (),
			  GetLastError ());                                 /* C 455-456 */
		  fflush (stdout);                                         /* C 457 */
		}

	      cursor_coords.X = (cursor_coords.X as ptrdiff_t + ncols) as SHORT;  /* C 460 */
	      w32con_move_cursor (f, cursor_coords.Y as c_int, cursor_coords.X as c_int);  /* C 461 */
	    }
	}
      len -= n;                                                         /* C 464 */
      string = string.add (n as usize);                                 /* C 465 */
    }
}

/* Used for mouse highlight.  */
/// C 470-534: `static void w32con_write_glyphs_with_face (struct frame *f, int x, int y,
/// struct glyph *string, int len, int face_id)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32con_write_glyphs_with_face (f: *mut frame, x: c_int, y: c_int,
					 string: *mut glyph, len: c_int, face_id: c_int)
{
  let mut conversion_buffer: LPCSTR = std::ptr::null ();                /* C 475 */
  let mut coding: *mut coding_system = std::ptr::null_mut ();           /* C 476 */

  w32con_hide_cursor ();                                                /* C 478 */

  if len <= 0                                                           /* C 480 */
    { return; }

  /* If terminal_coding does any conversion, use it, otherwise use
     safe_terminal_coding.  We can't use CODING_REQUIRE_ENCODING here
     because it always return 1 if the member src_multibyte is 1.  */
  coding = if coding_common_flags! (common_flags,
				    (FRAME_TERMINAL_CODING! (f) as *mut coding_system))
    & CODING_REQUIRE_ENCODING_MASK as c_int != 0
    { FRAME_TERMINAL_CODING! (f) as *mut coding_system }
    else { &raw mut safe_terminal_coding };                             /* C 486-487 */
  /* We are going to write the entire block of glyphs in one go, as
     they all have the same face.  So this _is_ the last block.  */
  coding_mode! (mode, coding, coding_mode! (mode, coding) | CODING_MODE_LAST_BLOCK);  /* C 490 */

  conversion_buffer = encode_terminal_code (string, len, coding) as LPCSTR;  /* C 492 */
  if (*coding).produced > 0                                             /* C 493 */
    {
      let mut filled: DWORD = 0;                                        /* C 495 */
      let mut written: DWORD = 0;
      if w32_use_virtual_terminal != 0                                  /* C 496 */
	{
	  let saved_coords: COORD = cursor_coords;                      /* C 498 */
	  w32con_move_cursor (f, y, x);                                 /* C 499 */
	  turn_on_face (f, face_id);                                    /* C 500 */
	  WriteConsole (cur_screen, conversion_buffer as LPCVOID,
			(*coding).produced as DWORD, &mut written,
			std::ptr::null_mut ());                             /* C 501-502 */
	  turn_off_face (f, face_id);                                   /* C 503 */
	  w32con_move_cursor (f, saved_coords.Y as c_int, saved_coords.X as c_int);  /* C 504 */
	}
      else
	{
	  /* Compute the character attributes corresponding to the face.  */
	  let char_attr: DWORD = w32_face_attributes (f, face_id) as DWORD;  /* C 509 */
	  let mut start_coords: COORD = core::mem::zeroed ();           /* C 510 */

	  start_coords.X = x as SHORT;                                  /* C 512 */
	  start_coords.Y = y as SHORT;                                  /* C 513 */

	  /* Account for character width.
	     FIXME: this doesn't handle character compositions.  */
	  let ncols: ptrdiff_t = strwidth ((*coding).source as *const c_char,
					   (*coding).src_bytes);        /* C 517 */

	  /* Set the attribute for these characters.  */
	  if FillConsoleOutputAttribute (cur_screen, char_attr as WORD, ncols as DWORD,
					 start_coords, &mut filled) == 0  /* C 520 */
	    { DebPrint! ((c"Failed writing console attributes: %d\n".as_ptr (), GetLastError ())); }  /* C 522 */
	  else
	    {
	      /* Write the characters.  */
	      if WriteConsoleOutputCharacter (cur_screen, conversion_buffer,
					      (*coding).produced as DWORD, start_coords,
					      &mut written) == 0        /* C 526 */
		{ DebPrint! ((c"Failed writing console characters: %d\n".as_ptr (),
			      GetLastError ())); }                          /* C 529-530 */
	    }
	}
    }
}

/* Implementation of draw_row_with_mouse_face for W32 console.  */
/// C 537-634: `void tty_draw_row_with_mouse_face (struct window *w, struct glyph_row *window_row,
/// int window_start_x, int window_end_x, enum draw_glyphs_face draw)`.  `src/term.rs` 3171 is the
/// other definition of this name (C's `src/term.c` defines it too, and the MSDOS backend has its
/// own); C's build links this one for the w32 console.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_draw_row_with_mouse_face (w: *mut window, window_row: *mut glyph_row,
						       mut window_start_x: c_int,
						       mut window_end_x: c_int,
						       draw: draw_glyphs_face)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       /* C 543 */
  let root: *mut frame = root_frame (f);                                /* C 544 */

  /* Window coordinates are relative to the text area.  Make
     them relative to the window's left edge,  */
  window_end_x = min! (window_end_x, (*window_row).used [TEXT_AREA as usize] as c_int);  /* C 549 */
  window_start_x += (*window_row).used [LEFT_MARGIN_AREA as usize] as c_int;  /* C 550 */
  window_end_x += (*window_row).used [LEFT_MARGIN_AREA as usize] as c_int;    /* C 551 */

  /* Translate from window to window's frame.  */
  let frame_start_x: c_int = WINDOW_LEFT_EDGE_X! (w) + window_start_x;  /* C 554 */
  let frame_end_x: c_int = WINDOW_LEFT_EDGE_X! (w) + window_end_x;      /* C 555 */
  let frame_y: c_int = (*window_row).y + WINDOW_TOP_EDGE_Y! (w);        /* C 556 */

  /* Translate from (possible) child frame to root frame.  */
  let mut root_start_x: c_int = 0;                                      /* C 559 */
  let mut root_end_x: c_int = 0;
  let mut root_y: c_int = 0;
  root_xy (f, frame_start_x, frame_y, &mut root_start_x, &mut root_y);  /* C 560 */
  root_xy (f, frame_end_x, frame_y, &mut root_end_x, &mut root_y);      /* C 561 */
  let root_row: *mut glyph_row
    = MATRIX_ROW! ((*root).current_matrix as *mut glyph_matrix, root_y);  /* C 562 */

  /* Remember current cursor coordinates so that we can restore
     them at the end.  */
  let save_coords: COORD = cursor_coords;                               /* C 566 */

  /* If the root frame displays child frames, we cannot naively
     write to the terminal what the window thinks should be drawn.
     Instead, write only those parts that are not obscured by
     other frames.  */
  let mut root_x: c_int = root_start_x;                                 /* C 573 */
  while root_x < root_end_x
    {
      /* Find the start of a run of glyphs from frame F.  */
      let mut root_start: *mut glyph
	= (*root_row).glyphs [TEXT_AREA as usize].add (root_x as usize);  /* C 575 */
      while root_x < root_end_x && (*root_start).frame != f as *mut c_void  /* C 576 */
	{ root_x += 1; root_start = root_start.add (1); }

      /* If start of a run of glyphs from F found.  */
      let root_run_start_x: c_int = root_x;                             /* C 579 */
      if root_run_start_x < root_end_x                                  /* C 580 */
	{
	  /* Find the end of the run of glyphs from frame F.  */
	  let mut root_end: *mut glyph = root_start;                    /* C 582 */
	  while root_x < root_end_x && (*root_end).frame == f as *mut c_void  /* C 583 */
	    { root_x += 1; root_end = root_end.add (1); }

	  /* If we have a run glyphs to output, do it.  */
	  if root_end > root_start                                       /* C 586 */
	    {
	      w32con_move_cursor (root, root_y, root_run_start_x);       /* C 588 */

	      let nglyphs: ptrdiff_t = root_end.offset_from (root_start);  /* C 590 */
	      match draw                                                 /* C 591 */
		{
		  DRAW_NORMAL_TEXT =>                                     /* C 594 */
		    { write_glyphs (f, root_start, nglyphs as c_int); }
		  DRAW_MOUSE_FACE =>                                      /* C 597 */
		    {
		      let tty: *mut tty_display_info = FRAME_TTY! (f);    /* C 599 */
		      let face_id: c_int
			= (*tty).mouse_highlight.mouse_face_face_id;      /* C 600 */
		      w32con_write_glyphs_with_face (f, root_run_start_x, root_y,
						     root_start, nglyphs as c_int,
						     face_id);            /* C 601-603 */
		    }
		  DRAW_INVERSE_VIDEO |                                    /* C 606 */
		  DRAW_CURSOR |
		  DRAW_IMAGE_RAISED |
		  DRAW_IMAGE_SUNKEN =>
		    { emacs_abort (); }                                   /* C 610 */
		  /* C 606-610 cover every enumerator; C's switch needs no default, Rust's match does (the `src/term.rs` 3240-3242 shape).  */
		  _ => {}
		}
	    }
	}
    }

  /* Restore cursor where it was before.  */
  w32con_move_cursor (f, save_coords.Y as c_int, save_coords.X as c_int);  /* C 632 */
}

/// C 636-643: `static void w32con_delete_glyphs (struct frame *f, int n)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_delete_glyphs (f: *mut frame, n: c_int)
{
  /* delete chars means scroll chars from cursor_coords.X + n to
     cursor_coords.X, anything beyond the edge of the screen should
     come out empty...  */

  scroll_line (f, n, LEFT);                                             /* C 642 */
}

/* window.h's `bool_bf cursor_off_p` bit of `struct window::bits`, which `include/window.rs` 192
   names by shift (`WINDOW_CURSOR_OFF_P_SHIFT`) and reads through the private `window_bit`
   (`include/window.rs` 515).  The member name stays the macro's first token (PORTING.md 1.3/P-80,
   the `src/term.rs` 406-410 copy).  */
/// C: bool_bf cursor_off_p : 1;
#[cfg (feature = "windowsnt")]
macro_rules! window_bits {
  (cursor_off_p, $w:expr) => { window_bit ($w, WINDOW_CURSOR_OFF_P_SHIFT) };
}

/// C 645-658: `static void w32con_setup_virtual_terminal (void)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32con_setup_virtual_terminal ()
{
  /* Disable unless 24bit color is supported (v. > 10.0.15063).  */
  w32_use_virtual_terminal
    = if w32_use_virtual_terminal != 0
      && (w32_major_version > 10
	  || (w32_major_version == 10
	      && (w32_minor_version > 0 || w32_build_number > 15063)))
    { 1 } else { 0 };                                                   /* C 636-639 */

  let mut out_mode: DWORD = 0;                                          /* C 641 */
  GetConsoleMode (cur_screen, &mut out_mode);                           /* C 642 */
  out_mode |= ENABLE_PROCESSED_OUTPUT;                                  /* C 643 */
  out_mode |= DISABLE_NEWLINE_AUTO_RETURN;                              /* C 644 */

  if w32_use_virtual_terminal != 0                                      /* C 646 */
    { out_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING; }
  else
    { out_mode &= ! ENABLE_VIRTUAL_TERMINAL_PROCESSING; }               /* C 649 */

  let out_mode_set: c_int = SetConsoleMode (cur_screen, out_mode);      /* C 651 */
  w32_use_virtual_terminal
    = if w32_use_virtual_terminal != 0 && out_mode_set != 0 { 1 } else { 0 };  /* C 652 */

  let max_colors: c_int = if w32_use_virtual_terminal != 0 { 16777216 } else { 16 };  /* C 654 */
  tty_setup_colors (current_tty, max_colors);                           /* C 655 */

  safe_calln! (Qtty_set_up_initial_frame_faces ());                     /* C 657 */
}

/// C 660-691: `static void w32con_reset_terminal_modes (struct terminal *t)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_reset_terminal_modes (t: *mut terminal)
{
  let mut dest: COORD = core::mem::zeroed ();                           /* C 663 */
  let mut info: CONSOLE_SCREEN_BUFFER_INFO = core::mem::zeroed ();      /* C 664 */
  let mut n: c_int = 0;                                                 /* C 665 */
  let mut r: DWORD = 0;                                                 /* C 666 */

  /* Clear the complete screen buffer.  This is required because Emacs
     sets the cursor position to the top of the buffer, but there might
     be other output below the bottom of the Emacs frame if the screen buffer
     is larger than the window size.  */
  GetConsoleScreenBufferInfo (cur_screen, &mut info);                   /* C 672 */
  dest.X = 0;                                                           /* C 673 */
  dest.Y = 0;                                                           /* C 674 */
  n = info.dwSize.X as c_int * info.dwSize.Y as c_int;                  /* C 675 */

  FillConsoleOutputAttribute (cur_screen, char_attr_normal, n as DWORD, dest, &mut r);  /* C 677 */
  FillConsoleOutputCharacter (cur_screen, b' ' as CHAR, n as DWORD, dest, &mut r);  /* C 678 */
  /* Now that the screen is clear, put the cursor at the top.  */
  SetConsoleCursorPosition (cur_screen, dest);                          /* C 680 */

  #[cfg (feature = "use_separate_screen")]
  { SetConsoleActiveScreenBuffer (prev_screen); }                       /* C 683 */
  #[cfg (not (feature = "use_separate_screen"))]
  {
    SetConsoleCursorInfo (prev_screen, &prev_console_cursor);           /* C 685 */
    if w32_use_virtual_terminal != 0                                    /* C 686 */
      { w32con_set_cursor_size (prev_console_cursor.dwSize as EMACS_INT); }  /* C 687 */
  }

  SetConsoleMode (keyboard_handle, prev_console_mode);                  /* C 690 */
}

/// C 693-714: `static void w32con_set_terminal_modes (struct terminal *t)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_set_terminal_modes (t: *mut terminal)
{
  /* Make cursor big and visible (100 on Windows 95 makes it disappear).  */
  w32con_set_cursor_size (99);                                          /* C 697 */

  SetConsoleActiveScreenBuffer (cur_screen);                            /* C 699 */

  /* If Quick Edit is enabled for the console, it will get in the way
     of receiving mouse events, so we disable it.  But leave the
     Insert Mode as it was set by the user.  */
  let mut new_console_mode: DWORD
    = ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS;  /* C 704-705 */
  if (prev_console_mode & ENABLE_INSERT_MODE) != 0                      /* C 706 */
    { new_console_mode |= ENABLE_INSERT_MODE; }                         /* C 707 */
  SetConsoleMode (keyboard_handle, new_console_mode);                   /* C 708 */

  /* Initialize input mode: interrupt_input off, no flow control, allow
     8 bit character input, standard quit char.  */
  Fset_input_mode (Qnil (), Qnil (), make_fixnum (2), Qnil ());         /* C 712 */
  w32con_setup_virtual_terminal ();                                     /* C 713 */
}

/* hmmm... perhaps these let us bracket screen changes so that we can flush
   clumps rather than one-character-at-a-time...

   we'll start with not moving the cursor while an update is in progress.  */
/// C 720-723: `static void w32con_update_begin (struct frame *f)` - an empty body, carried as
/// C's own empty one.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_update_begin (f: *mut frame)
{
}

/// C 725-734: `static void w32con_update_end (struct frame *f)`.
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn w32con_update_end (f: *mut frame)
{
  SetConsoleCursorPosition (cur_screen, cursor_coords);                  /* C 728 */
  if ! window_bits! (cursor_off_p, XWINDOW (selected_window))            /* C 729 */
    && (cursor_coords.X as c_int) < FRAME_COLS (f)                        /* C 730 */
    { w32con_show_cursor (); }                                          /* C 731 */
  else
    { w32con_hide_cursor (); }                                          /* C 733 */
}

/***********************************************************************
			stubs from termcap.c
 ***********************************************************************/

/// C 742-745: `void sys_tputs (char *str, int nlines, int (*outfun) (int))` - C's empty body.
/// C 740's prototype is dropped (PORTING.md 0.1-12).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_tputs (str: *mut c_char, nlines: c_int,
				    outfun: Option<unsafe extern "C" fn (c_int) -> c_int>)
{
}

/// C 749-753: `char *sys_tgetstr (char *cap, char **area)`.
/// C 747's prototype is dropped (PORTING.md 0.1-12).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_tgetstr (cap: *mut c_char, area: *mut *mut c_char) -> *mut c_char
{
  return std::ptr::null_mut ();                                         /* C 752 */
}


/***********************************************************************
			stubs from cm.c
 ***********************************************************************/

/// C 761: `int cost = 0;` (C 760's `extern int cost;` is its own declaration).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut cost: c_int = 0;

/// C 765-769: `int evalcost (int c)`.  C's `static cm.c` version increments `cost`; this file's
/// stub is the identity, and it is the one the w32 build links (`CM_OBJ=`, configure.ac 3071).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn evalcost (c: c_int) -> c_int
{
  return c;                                                             /* C 768 */
}

/// C 773-777: `int cmputc (int c)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmputc (c: c_int) -> c_int
{
  return c;                                                             /* C 776 */
}

/// C 781-784: `void cmcheckmagic (struct tty_display_info *tty)` - C's empty body.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmcheckmagic (tty: *mut tty_display_info)
{
}

/// C 788-791: `void cmcostinit (struct tty_display_info *tty)` - C's empty body.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmcostinit (tty: *mut tty_display_info)
{
}

/// C 795-798: `void cmgoto (struct tty_display_info *tty, int row, int col)` - C's empty body.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmgoto (tty: *mut tty_display_info, row: c_int, col: c_int)
{
}

/// C 802-805: `void Wcm_clear (struct tty_display_info *tty)` - C's empty body.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Wcm_clear (tty: *mut tty_display_info)
{
}


/* Report the current cursor position.  The following two functions
   are used in term.c's tty menu code, so they are not really
   "stubs".  */
/// C 811-815: `int cursorX (struct tty_display_info *tty)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cursorX (tty: *mut tty_display_info) -> c_int
{
  return cursor_coords.X as c_int;                                      /* C 814 */
}

/// C 817-821: `int cursorY (struct tty_display_info *tty)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cursorY (tty: *mut tty_display_info) -> c_int
{
  return cursor_coords.Y as c_int;                                      /* C 820 */
}

/***********************************************************************
				Faces
 ***********************************************************************/

/* ----------------------------------------------------------------------------------------------
   `struct face`, `struct face_cache` and the `FACE_FROM_ID`/`FRAME_FACE_CACHE` pair.

   `dispextern.h` 1787-2009 declares both records, and `include/dispextern.rs` carries a copy behind
   `dispextern_face_records` (2185-2312 - the *window-system* layout, 296 bytes, `gc`/`stipple` at
   168/176) while `frame.rs`'s accessors sit in `pub mod frame_face_accessors`, which that mirror
   does not glob-import.  This unit needs the frozen target's own 280-byte record: `src/term.rs`
   302-400 declares exactly this copy for the same reason (a unit may only name what it declares or
   includes, P-72), so the four items and the `bits` accessors are repeated here with the measured
   offsets that unit records.  The ledger's `## Owed` names the mirror row that would let them
   retire.
   ---------------------------------------------------------------------------------------------- */

/// C: struct face_cache { struct face **buckets; struct frame *f; struct face **faces_by_id;
/// C:                    ptrdiff_t size; int used; bool_bf menu_face_changed_p : 1; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  pub f: *mut frame,
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub _pad_tail: [u8; 4],               // dispextern.h 2005-2008 (`menu_face_changed_p : 1` + padding)
}
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::size_of::<face_cache> () == 40);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face_cache, faces_by_id) == 16);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face_cache, used) == 32);

/// C: struct face { Lisp_Object lface[LFACE_VECTOR_SIZE]; int id; ...; unsigned long foreground;
/// C:               unsigned long background; unsigned long underline_color; ...; bool_bf …; };
/// The measured frozen layout: `lface`@0, `id`@160, `foreground`@168, `background`@176,
/// `underline_color`@184, the `bool_bf`/`ENUM_BF` run in the three bytes at 240-242, size 280.
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct face
{
  pub _pad_lface: [u8; 160],            // dispextern.h 1789-1794: `Lisp_Object lface[LFACE_VECTOR_SIZE]`
  pub id: c_int,                        // 1795, offset 160
  pub _pad_id: [u8; 4],
  pub foreground: c_ulong,              // 1816, offset 168
  pub background: c_ulong,              // 1819, offset 176
  pub underline_color: c_ulong,         // 1823, offset 184
  pub _pad_mid: [u8; 48],               // 1824-1871: overline/strike/box colours … box widths
  pub bits: [u8; 3],                    // 240-242: `underline : 3` + the six `bool_bf` members
  pub _pad_tail: [u8; 37],
}
/// dispextern.h 2400-2402: `ENUM_BF (face_underline_type) underline : 3` (bits 2-4 of the byte at
/// 240) and the six `bool_bf` members of the bytes at 241-242.  This unit reads `underline`,
/// `tty_bold_p`, `tty_italic_p`, `tty_reverse_p`, `tty_strike_through_p` and `tty_overline_p`.
#[cfg (feature = "windowsnt")]
pub const FACE_UNDERLINE_MASK: u8 = 0x1c;
#[cfg (feature = "windowsnt")]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_BOLD_P: u8 = 1 << 7;        // 241:7
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_ITALIC_P: u8 = 1 << 0;      // 242:0
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_REVERSE_P: u8 = 1 << 1;     // 242:1
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_STRIKE_THROUGH_P: u8 = 1 << 2;  // 242:2
#[cfg (feature = "windowsnt")]
pub const FACE_TTY_OVERLINE_P: u8 = 1 << 3;    // 242:3
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::size_of::<face> () == 280);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face, foreground) == 168);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face, background) == 176);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face, underline_color) == 184);
#[cfg (feature = "windowsnt")]
const _: () = core::assert! (core::mem::offset_of! (face, bits) == 240);

/// C: #define FRAME_FACE_CACHE(F) (F)->face_cache  (frame.h 1379)
#[cfg (feature = "windowsnt")]
macro_rules! FRAME_FACE_CACHE {
  ($f:expr) => { (*$f).face_cache as *mut face_cache };
}
/// C: INLINE struct face *FACE_FROM_ID (struct frame *f, int id)  (frame.h 1646-1650)
#[cfg (feature = "windowsnt")]
#[inline(always)]
unsafe fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < (*FRAME_FACE_CACHE! (f)).used);
  *(*FRAME_FACE_CACHE! (f)).faces_by_id.add (id as usize)
}

/// C: bool_bf tty_bold_p : 1; (dispextern.h 1861-1895 - the member name is the macro's first
/// token, PORTING.md 1.3/P-80, the `src/term.rs` 396-403 copy).
#[cfg (feature = "windowsnt")]
macro_rules! face_bits {
  (underline, $f:expr) =>
    { ((((*$f).bits [0] & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT) as face_underline_type) };
  (tty_bold_p, $f:expr) => { ((*$f).bits [1] & FACE_TTY_BOLD_P) != 0 };
  (tty_italic_p, $f:expr) => { ((*$f).bits [1] & FACE_TTY_ITALIC_P) != 0 };
  (tty_reverse_p, $f:expr) => { ((*$f).bits [1] & FACE_TTY_REVERSE_P) != 0 };
  (tty_strike_through_p, $f:expr) => { ((*$f).bits [2] & FACE_TTY_STRIKE_THROUGH_P) != 0 };
  (tty_overline_p, $f:expr) => { ((*$f).bits [2] & FACE_TTY_OVERLINE_P) != 0 };
}

/* C hook assignments at C 983-1002: termhooks.h degrades `struct frame *`/`struct window *`
   parameters to `*mut c_void` (P-72 rule 3) while the definitions keep C's pointer, so each
   assignment reinterprets the function pointer - C's own implicit conversion, spelled out (the
   `src/term.rs` 137-144 `hook!` copy).  */
#[cfg (feature = "windowsnt")]
macro_rules! hook {
  ($target:ty, $f:expr) => { unsafe {
    let p: *const c_void = $f as *const c_void;
    std::mem::transmute::<*const c_void, Option<$target>> (p) } };
}

/* Turn appearances of face FACE_ID on tty frame F on.  */
/// C 830-861: `static WORD w32_face_attributes (struct frame *f, int face_id)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32_face_attributes (f: *mut frame, face_id: c_int) -> WORD
{
  let mut char_attr: WORD;                                              /* C 833 */
  let face: *mut face = FACE_FROM_ID (f, face_id);                      /* C 834 */

  char_attr = char_attr_normal;                                         /* C 836 */

  /* Reverse the default color if requested. If background and
     foreground are specified, then they have been reversed already.  */
  if face_bits! (tty_reverse_p, face)                                   /* C 840 */
    { char_attr = (char_attr & 0xff00) + ((char_attr & 0x000f) << 4)
	+ ((char_attr & 0x00f0) >> 4); }                                /* C 841-842 */

  /* Before the terminal is properly initialized, all colors map to 0.
     Don't try to resolve them.  */
  if NILP (Vtty_defined_color_alist)                                    /* C 846 */
    { return char_attr; }                                               /* C 847 */

  /* Colors should be in the range 0...15 unless they are one of
     FACE_TTY_DEFAULT_COLOR, FACE_TTY_DEFAULT_FG_COLOR or
     FACE_TTY_DEFAULT_BG_COLOR.  Other out of range colors are
     invalid, so it is better to use the default color if they ever
     get through to here.  */
  if (*face).foreground >= 0 && (*face).foreground < 16                  /* C 854 */
    { char_attr = ((char_attr & 0xfff0) as c_ulong + (*face).foreground) as WORD; }  /* C 855 */

  if (*face).background >= 0 && (*face).background < 16                  /* C 857 */
    { char_attr = ((char_attr & 0xff0f) as c_ulong + ((*face).background << 4)) as WORD; }  /* C 858 */

  return char_attr;                                                     /* C 860 */
}

/// C 863-950: `static void turn_on_face (struct frame *f, int face_id)`.
#[cfg (feature = "windowsnt")]
unsafe fn turn_on_face (f: *mut frame, face_id: c_int)
{
  let face: *mut face = FACE_FROM_ID (f, face_id);                      /* C 866 */
  let tty: *mut tty_display_info = FRAME_TTY! (f);                      /* C 867 */
  let mut fg: c_ulong = (*face).foreground;                             /* C 868 */
  let mut bg: c_ulong = (*face).background;                             /* C 869 */

  /* construct combined VT sequence for face attributes */
  let mut n: DWORD = 0;                                                 /* C 872 */
  let mut sz: size_t = SEQMAX;                                          /* C 873 */
  let mut seq: [c_char; SEQMAX] = [0; SEQMAX];                          /* C 874 (C's VLA, SEQMAX bytes) */
  sz -= 1;                                                              /* C 875 */

  if face_bits! (tty_bold_p, face)                                      /* C 877 */
    { SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_bold_mode, std::ptr::null_mut::<c_void> ()); }  /* C 878 */
  if face_bits! (tty_italic_p, face)                                    /* C 879 */
    { SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_italic_mode, std::ptr::null_mut::<c_void> ()); }  /* C 880 */
  if face_bits! (tty_strike_through_p, face)                            /* C 881 */
    { SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_strike_through_mode, std::ptr::null_mut::<c_void> ()); }  /* C 882 */
  if face_bits! (underline, face) != 0                                  /* C 883 */
    {
      if face_bits! (underline, face) == FACE_UNDERLINE_SINGLE          /* C 885 */
	{ SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_underline_mode, std::ptr::null_mut::<c_void> ()); }  /* C 886 */
      else
	{ SSPRINTF! (seq, &mut n, sz, (*tty).TF_set_underline_style, face_bits! (underline, face)); }  /* C 888 */
    }
  if face_bits! (tty_overline_p, face) as c_int != 0                    /* C 890 */
    { SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_overline_mode, std::ptr::null_mut::<c_void> ()); }  /* C 891 */
  /* Note: when face->tty_reverse_p != 0 and fg and bg are specified,
     their values are already swapped and reversing them here would swap
     them back, but we need to handle the reversal when unspecified.  */
  if face_bits! (tty_reverse_p, face) && DEFAULTP! (fg) && DEFAULTP! (bg)  /* C 895 */
    { SSPRINTF! (seq, &mut n, sz, (*tty).TS_enter_reverse_mode, std::ptr::null_mut::<c_void> ()); }  /* C 896 */

  if DEFAULTP! (fg) { fg = fg_normal as c_ulong; }                      /* C 898 */
  if DEFAULTP! (bg) { bg = bg_normal as c_ulong; }                      /* C 899 */

  let set_fg: *const c_char = (*tty).TS_set_foreground;                 /* C 901 */
  let set_bg: *const c_char = (*tty).TS_set_background;                 /* C 902 */
  if (*tty).TN_max_colors == 8 || (*tty).TN_max_colors == 16            /* C 903 */
    {
      /* fg and bg are indices into 16 base colors (see link at top).  */
      let mut fgi: c_ulong = 0;                                         /* C 906 */
      let mut bgi: c_ulong = 0;

      fgi = if fg >= 0 && fg < 8                                        /* C 908-912 */
	{ fg + 30 }
	else if fg >= 8 && fg < 16
	{ fg - 8 + 90 }
	else { 0 };
      if fgi != 0                                                       /* C 913 */
	{ SSPRINTF! (seq, &mut n, sz, set_fg, fgi); }                    /* C 914 */

      bgi = if bg >= 0 && bg < 8                                        /* C 916-920 */
	{ bg + 40 }
	else if bg >= 8 && bg < 16
	{ bg - 8 + 100 }
	else { 0 };
      if bgi != 0                                                       /* C 921 */
	{ SSPRINTF! (seq, &mut n, sz, set_bg, bgi); }                    /* C 922 */
    }
  else if (*tty).TN_max_colors == 256                                   /* C 924 */
    {
      /* fg and bg are xterm indices.  */
      if fg >= 0 && fg < 256                                            /* C 927 */
	{ SSPRINTF! (seq, &mut n, sz, set_fg, fg); }                     /* C 928 */

      if bg >= 0 && bg < 256                                            /* C 930 */
	{ SSPRINTF! (seq, &mut n, sz, set_bg, bg); }                     /* C 931 */
    }
  else if (*tty).TN_max_colors == 16777216                              /* C 933 */
    {
      /* fg and bg are pixel values -- decompose to rgb triples.  */
      let rf: c_ulong = fg / 65536;                                     /* C 936 */
      let gf: c_ulong = (fg / 256) & 255;
      let bf: c_ulong = fg & 255;
      let rb: c_ulong = bg / 65536;                                     /* C 937 */
      let gb: c_ulong = (bg / 256) & 255;
      let bb: c_ulong = bg & 255;
      SSPRINTF! (seq, &mut n, sz, set_fg, rf, gf, bf);                   /* C 938 */
      SSPRINTF! (seq, &mut n, sz, set_bg, rb, gb, bb);                   /* C 939 */
    }
  /* Non-default underline color supported only in TrueColor mode.  */
  if (*face).underline_color != 0 && (*tty).TN_max_colors == 16777216   /* C 942 */
    {
      let ur: c_ulong = (*face).underline_color / 65536;                /* C 944 */
      let ug: c_ulong = ((*face).underline_color / 256) & 255;          /* C 945 */
      let ub: c_ulong = (*face).underline_color & 255;                  /* C 946 */
      SSPRINTF! (seq, &mut n, sz, (*tty).TF_set_underline_color, ur, ug, ub);  /* C 947 */
    }
  w32con_write_vt_seq (seq.as_ptr () as *const c_char);                 /* C 949 */
}

/// C 952-957: `static void turn_off_face (struct frame *f, int face_id)`.
#[cfg (feature = "windowsnt")]
unsafe fn turn_off_face (f: *mut frame, face_id: c_int)
{
  let tty: *mut tty_display_info = FRAME_TTY! (f);                      /* C 955 */
  w32con_write_vt_seq ((*tty).TS_exit_attribute_mode);                  /* C 956 */
}

/* The IME window is needed to receive the session notifications
   required to reset the low level keyboard hook state.  */

/// C 962-975: `static BOOL CALLBACK find_ime_window (HWND hwnd, LPARAM arg)`.
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn find_ime_window (hwnd: HWND, arg: LPARAM) -> BOOL
{
  let mut window_class: [c_char; 32] = [0; 32];                          /* C 965 */

  GetClassName (hwnd, window_class.as_mut_ptr (), core::mem::size_of_val (&window_class) as c_int);  /* C 967 */
  if strcmp (window_class.as_ptr (), c"IME".as_ptr ()) == 0              /* C 968 */
    {
      *(arg as *mut HWND) = hwnd;                                       /* C 970 */
      return FALSE;                                                     /* C 971 */
    }
  /* keep looking */
  return TRUE;                                                          /* C 974 */
}

/// C 977-1156: `void initialize_w32_display (struct terminal *term, int *width, int *height)`.
/// The hook assignments at C 983-1002 are the `hook!` reinterpretations the note above records;
/// the four names they name that come from other units (`w32_sys_ring_bell`,
/// `w32_initialize_display_info`, `init_crit`, `setup_w32_kbdhook`) are the `w32term.h` mirror's.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_w32_display (term: *mut terminal, width: *mut c_int,
						 height: *mut c_int)
{
  let mut info: CONSOLE_SCREEN_BUFFER_INFO = core::mem::zeroed ();        /* C 980 */

  (*term).rif = std::ptr::null_mut (); /* No window based redisplay on the console.  */  /* C 982 */
  (*term).cursor_to_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int),
				  w32con_move_cursor);                    /* C 983 */
  (*term).raw_cursor_to_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int),
				      w32con_move_cursor);                /* C 984 */
  (*term).clear_to_end_hook = hook! (unsafe extern "C" fn (*mut c_void),
				     w32con_clear_to_end);                /* C 985 */
  (*term).clear_frame_hook = hook! (unsafe extern "C" fn (*mut c_void),
				    w32con_clear_frame);                  /* C 986 */
  (*term).clear_end_of_line_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int),
					  w32con_clear_end_of_line);      /* C 987 */
  (*term).ins_del_lines_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int),
				      w32con_ins_del_lines);              /* C 988 */
  (*term).insert_glyphs_hook = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void, c_int),
				      w32con_insert_glyphs);              /* C 989 */
  (*term).write_glyphs_hook = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void, c_int),
				     w32con_write_glyphs);                /* C 990 */
  (*term).delete_glyphs_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int),
				      w32con_delete_glyphs);              /* C 991 */
  (*term).ring_bell_hook = hook! (unsafe extern "C" fn (*mut c_void),
				  w32_sys_ring_bell);                     /* C 992 */
  (*term).reset_terminal_modes_hook = hook! (unsafe extern "C" fn (*mut terminal),
					     w32con_reset_terminal_modes);  /* C 993 */
  (*term).set_terminal_modes_hook = hook! (unsafe extern "C" fn (*mut terminal),
					   w32con_set_terminal_modes);    /* C 994 */
  (*term).set_terminal_window_hook = None;                              /* C 995 */
  (*term).update_begin_hook = hook! (unsafe extern "C" fn (*mut c_void),
				     w32con_update_begin);                /* C 996 */
  (*term).update_end_hook = hook! (unsafe extern "C" fn (*mut c_void),
				   w32con_update_end);                    /* C 997 */

  (*term).defined_color_hook = hook! (unsafe extern "C" fn (*mut c_void, *const c_char,
							    *mut Emacs_Color, bool, bool) -> bool,
				      tty_defined_color); /* xfaces.c */  /* C 999 */
  (*term).read_socket_hook
    = hook! (unsafe extern "C" fn (*mut terminal, *mut input_event) -> c_int,
	     w32_console_read_socket);                                    /* C 1000 */
  (*term).mouse_position_hook
    = hook! (unsafe extern "C" fn (*mut *mut c_void, c_int, *mut Lisp_Object,
				   *mut scroll_bar_part, *mut Lisp_Object, *mut Lisp_Object,
				   *mut Time),
	     w32_console_mouse_position);                                 /* C 1001 */
  (*term).menu_show_hook
    = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int, Lisp_Object,
				   *mut *const c_char) -> Lisp_Object,
	     tty_menu_show);                                              /* C 1002 */

  /* The following are not used on the console.  */
  (*term).frame_rehighlight_hook = None;                                /* C 1005 */
  (*term).frame_raise_lower_hook = None;                                /* C 1006 */
  (*term).set_vertical_scroll_bar_hook = None;                          /* C 1007 */
  (*term).set_horizontal_scroll_bar_hook = None;                        /* C 1008 */
  (*term).condemn_scroll_bars_hook = None;                              /* C 1009 */
  (*term).redeem_scroll_bar_hook = None;                                /* C 1010 */
  (*term).judge_scroll_bars_hook = None;                                /* C 1011 */
  (*term).frame_up_to_date_hook = None;                                 /* C 1012 */

  /* Initialize the mouse-highlight data.  */
  reset_mouse_highlight (core::ptr::addr_of_mut! ((*((*term).display_info.tty
						     as *mut tty_display_info)).mouse_highlight));  /* C 1015 */

  /* Initialize interrupt_handle.  */
  init_crit ();                                                         /* C 1018 */

  /* Remember original console settings.  */
  keyboard_handle = GetStdHandle (STD_INPUT_HANDLE);                    /* C 1021 */
  GetConsoleMode (keyboard_handle, &mut prev_console_mode);             /* C 1022 */
  /* Make sure ENABLE_EXTENDED_FLAGS is set in console settings,
     otherwise restoring the original setting of ENABLE_MOUSE_INPUT
     will not work.  */
  prev_console_mode |= ENABLE_EXTENDED_FLAGS;                           /* C 1026 */

  prev_screen = GetStdHandle (STD_OUTPUT_HANDLE);                       /* C 1028 */

  #[cfg (feature = "use_separate_screen")]
  {
    cur_screen = CreateConsoleScreenBuffer (GENERIC_READ | GENERIC_WRITE,
					    0, std::ptr::null (),
					    CONSOLE_TEXTMODE_BUFFER,
					    std::ptr::null_mut ());         /* C 1031-1034 */

    if cur_screen == INVALID_HANDLE_VALUE                               /* C 1036 */
      {
	printf (c"CreateConsoleScreenBuffer failed in initialize_w32_display\n".as_ptr ());  /* C 1038 */
	printf (c"LastError = 0x%lx\n".as_ptr (), GetLastError ());       /* C 1039 */
	fflush (stdout);                                                /* C 1040 */
	exit (1);                                                       /* C 1041 */
      }
  }
  #[cfg (not (feature = "use_separate_screen"))]
  {
    cur_screen = prev_screen;                                           /* C 1044 */
    GetConsoleCursorInfo (prev_screen, &mut prev_console_cursor);       /* C 1045 */
  }

  /* Record whether we are on ConHost or Windows Terminal.  */
  const virt_mode_flags: DWORD
    = ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING;     /* C 1049-1050 */
  GetConsoleMode (cur_screen, &mut prev_output_mode);                   /* C 1051 */
  w32__terminal_is_conhost
    = (prev_output_mode & virt_mode_flags) != virt_mode_flags;          /* C 1052-1053 */

  /* Respect setting of LINES and COLUMNS environment variables.  */
  {
    let lines: *mut c_char = getenv (c"LINES".as_ptr ());               /* C 1057 */
    let columns: *mut c_char = getenv (c"COLUMNS".as_ptr ());           /* C 1058 */

    if ! lines.is_null () && ! columns.is_null ()                       /* C 1060 */
      {
	let mut new_win_dims: SMALL_RECT = core::mem::zeroed ();        /* C 1062 */
	let mut new_size: COORD = core::mem::zeroed ();                 /* C 1063 */

	new_size.X = atoi (columns) as SHORT;                           /* C 1065 */
	new_size.Y = atoi (lines) as SHORT;                             /* C 1066 */

	GetConsoleScreenBufferInfo (cur_screen, &mut info);             /* C 1068 */

	/* Shrink the window first, so the buffer dimensions can be
           reduced if necessary.  */
	new_win_dims.Top = 0;                                           /* C 1072 */
	new_win_dims.Left = 0;                                          /* C 1073 */
	new_win_dims.Bottom
	  = ((min! (new_size.Y as c_int, info.dwSize.Y as c_int)) - 1) as SHORT;  /* C 1074 */
	new_win_dims.Right
	  = ((min! (new_size.X as c_int, info.dwSize.X as c_int)) - 1) as SHORT;  /* C 1075 */
	SetConsoleWindowInfo (cur_screen, TRUE, &new_win_dims);         /* C 1076 */

	SetConsoleScreenBufferSize (cur_screen, new_size);              /* C 1078 */

	/* Set the window size to match the buffer dimension.  */
	new_win_dims.Top = 0;                                           /* C 1081 */
	new_win_dims.Left = 0;                                          /* C 1082 */
	new_win_dims.Bottom = (new_size.Y as c_int - 1) as SHORT;        /* C 1083 */
	new_win_dims.Right = (new_size.X as c_int - 1) as SHORT;         /* C 1084 */
	SetConsoleWindowInfo (cur_screen, TRUE, &new_win_dims);         /* C 1085 */
      }
  }

  if GetConsoleScreenBufferInfo (cur_screen, &mut info) == 0            /* C 1089 */
    {
      printf (c"GetConsoleScreenBufferInfo failed in initialize_w32_display\n".as_ptr ());  /* C 1091 */
      printf (c"LastError = 0x%lx\n".as_ptr (), GetLastError ());       /* C 1092 */
      fflush (stdout);                                                  /* C 1093 */
      exit (1);                                                         /* C 1094 */
    }

  char_attr_normal = info.wAttributes;                                  /* C 1097 */
  fg_normal = (char_attr_normal & 0x000f) as c_int;                      /* C 1098 */
  bg_normal = ((char_attr_normal >> 4) & 0x000f) as c_int;               /* C 1099 */

  /* Determine if the info returned by GetConsoleScreenBufferInfo
     is realistic.  Old MS Telnet servers used to only fill out
     the dwSize portion, even modern one fill the whole struct with
     garbage when using non-MS telnet clients.  */
  if (w32_use_full_screen_buffer
       && ((info.dwSize.Y as c_int) < 20 || (info.dwSize.Y as c_int) > 100
	   || (info.dwSize.X as c_int) < 40 || (info.dwSize.X as c_int) > 200))  /* C 1105-1107 */
      || (! w32_use_full_screen_buffer
	  && ((info.srWindow.Bottom as c_int - info.srWindow.Top as c_int) < 20
	      || (info.srWindow.Bottom as c_int - info.srWindow.Top as c_int) > 100
	      || (info.srWindow.Right as c_int - info.srWindow.Left as c_int) < 40
	      || (info.srWindow.Right as c_int - info.srWindow.Left as c_int) > 100))  /* C 1108-1112 */
    {
      *height = 25;                                                     /* C 1114 */
      *width = 80;                                                      /* C 1115 */
    }

  else if w32_use_full_screen_buffer                                     /* C 1118 */
    {
      *height = info.dwSize.Y as c_int;	/* lines per page */              /* C 1120 */
      *width = info.dwSize.X as c_int;	/* characters per line */         /* C 1121 */
    }
  else
    {
      /* Lines per page.  Use buffer coords instead of buffer size.  */
      *height = 1 + info.srWindow.Bottom as c_int - info.srWindow.Top as c_int;     /* C 1126 */
      /* Characters per line.  Use buffer coords instead of buffer size.  */
      *width = 1 + info.srWindow.Right as c_int - info.srWindow.Left as c_int;       /* C 1128 */
    }

  /* Force reinitialization of the "empty row" buffer, in case they
     dumped from a running session.  */
  if glyphs != core::ptr::addr_of_mut! (glyph_base) as *mut glyph        /* C 1133 */
    {
      glyphs = std::ptr::null_mut ();                                    /* C 1135 */
      glyphs_len = 0;                                                    /* C 1136 */
      ceol_initialized = FALSE;                                          /* C 1137 */
    }

  if os_subtype == OS_SUBTYPE_NT                                        /* C 1140 */
    { w32_console_unicode_input = 1; }                                   /* C 1141 */
  else
    { w32_console_unicode_input = 0; }                                   /* C 1143 */

  /* Setup w32_display_info structure for this frame.  */
  w32_initialize_display_info (build_string (c"Console".as_ptr ()));     /* C 1146 */

  let mut hwnd: HWND = std::ptr::null_mut ();                           /* C 1148 */
  EnumThreadWindows (GetCurrentThreadId (), Some (find_ime_window),
		     core::ptr::addr_of_mut! (hwnd) as LPARAM);          /* C 1149 */

  /* Set up the keyboard hook.  */
  setup_w32_kbdhook (hwnd);                                             /* C 1152 */

  /* Set current_tty to the tty of this terminal */
  current_tty = (*term).display_info.tty as *mut tty_display_info;       /* C 1155 */
}

/* C 1159-1185: `DEFUN ("set-screen-color", Fset_screen_color, Sset_screen_color, 2, 3, 0, …)`.  */
#[cfg (feature = "windowsnt")]
DEFUN! ("set-screen-color", Fset_screen_color, Sset_screen_color, 2, 3, 0,
       doc: "Set screen foreground and background colors.\n\nArguments should be indices for colors in the list returned by `tty-color-alist'.\nIf VTP is non-nil, settings affect virtual terminal processing only.\nOtherwise, arguments should be between 0 and 15, and settings will\nbe effective only when virtual terminal processing is disabled.\n\nSee w32console.el and the documentation for `w32-use-virtual-terminal'.  ",
       (foreground: Lisp_Object, background: Lisp_Object, vtp: Lisp_Object),
       {
	 let fg: c_int = XFIXNAT (foreground) as c_int;                 /* C 1170 */
	 let bg: c_int = XFIXNAT (background) as c_int;                 /* C 1171 */

	 if NILP (vtp)                                                  /* C 1173 */
	   {
	     char_attr_normal = (fg + (bg << 4)) as WORD;               /* C 1175 */
	   }
	 else
	   {
	     fg_normal = fg;                                            /* C 1179 */
	     bg_normal = bg;                                            /* C 1180 */
	   }

	 Frecenter (Qnil (), Qt ());                                    /* C 1183 */
	 return Qt ();                                                  /* C 1184 */
       });

/* C 1187-1204: `DEFUN ("get-screen-color", Fget_screen_color, Sget_screen_color, 0, 1, 0, …)`.  */
#[cfg (feature = "windowsnt")]
DEFUN! ("get-screen-color", Fget_screen_color, Sget_screen_color, 0, 1, 0,
       doc: "Get color indices of the current screen foreground and background.\n\nThe colors are returned as a list of 2 indices (FOREGROUND BACKGROUND) for\ncolors in the list returned by `tty-color-alist`.\n\nIf VTP is non-nil, returns settings effective when virtual terminal\nprocessing is enabled.  Otherwise, returns settings effective when\nvirtual terminal processing is disabled.\n\nSee w32console.el and the documentation for `w32-use-virtual-terminal'.  ",
       (vtp: Lisp_Object),
       {
	 let fg: c_int = if NILP (vtp) { (char_attr_normal & 0x000f) as c_int } else { fg_normal };  /* C 1200 */
	 let bg: c_int = if NILP (vtp) { ((char_attr_normal >> 4) & 0x000f) as c_int }
	   else { bg_normal };                                          /* C 1201 */

	 return Fcons (make_fixnum (fg as EMACS_INT),
		       Fcons (make_fixnum (bg as EMACS_INT), Qnil ()));  /* C 1203 */
       });

/* C 1206-1221: `DEFUN ("w32-use-virtual-terminal", Fw32_use_virtual_terminal,
   Sw32_use_virtual_terminal, 1, 1, 0, …)`.  */
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-use-virtual-terminal", Fw32_use_virtual_terminal, Sw32_use_virtual_terminal, 1, 1, 0,
       doc: "Enables (disables) virtual terminal sequence processing if argument is t (nil).  ",
       (arg: Lisp_Object),
       {
	 if EQ (arg, Qt ())                                             /* C 1210 */
	   { w32_use_virtual_terminal = 1; }                            /* C 1211 */
	 else if EQ (arg, Qnil ())                                      /* C 1212 */
	   { w32_use_virtual_terminal = 0; }                            /* C 1213 */
	 else {
	   error (c"Invalid argument: expects t or nil.".as_ptr ());     /* C 1215 */
	   return Qnil ();                                              /* C 1216 */
	 }

	 w32con_setup_virtual_terminal ();                              /* C 1219 */
	 return Qt ();                                                  /* C 1220 */
       });

/* C 1223-1228: `DEFUN ("w32-use-virtual-terminal-p", Fw32_use_virtual_terminal_p,
   Sw32_use_virtual_terminal_p, 0, 0, 0, …)`.  */
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-use-virtual-terminal-p", Fw32_use_virtual_terminal_p,
       Sw32_use_virtual_terminal_p, 0, 0, 0,
       doc: "Returns t (nil) if virtual terminal sequence processing is enabled (disabled).  ",
       (),
       {
	 return if w32_use_virtual_terminal != 0 { Qt () } else { Qnil () };  /* C 1227 */
       });

/* C 1230-1237: `DEFUN ("set-cursor-size", Fset_cursor_size, Sset_cursor_size, 1, 1, 0, …)`.  */
#[cfg (feature = "windowsnt")]
DEFUN! ("set-cursor-size", Fset_cursor_size, Sset_cursor_size, 1, 1, 0,
       doc: "Set cursor size.  ",
       (size: Lisp_Object),
       {
	 w32con_set_cursor_size (XFIXNAT (size));                        /* C 1234 */

	 return Qt ();                                                  /* C 1236 */
       });

/* The two `DEFVAR_BOOL` carriers this unit owns.  C's storage for a `DEFVAR_BOOL` is the field
   `globals.f_<name>` that `DEFVAR_BOOL` (lisp.h 3507-3512) points its `Lisp_Fwd` at, and the
   mirror's `pub mod variables` declares the name `extern`; the port keeps the storage with the
   `DEFVAR_*` that registers it (the `src/coding.rs` 237-238 precedent), so the definition here is
   the explicit item that shadows the glob import and provides the symbol the other w32 units -
   `src/w32inevt.c`, `src/w32fns.c` - read.  */
/// C 1243: `w32_use_full_screen_buffer` (the `DEFVAR_BOOL` at C 1242-1248 registers it).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut w32_use_full_screen_buffer: bool = false;
/// C 1252: `w32__terminal_is_conhost` (the `DEFVAR_BOOL` at C 1251-1255 registers it).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut w32__terminal_is_conhost: bool = false;

/// C 1239-1262: `void syms_of_ntterm (void)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_ntterm ()
{
  DEFVAR_BOOL! (c"w32-use-full-screen-buffer", w32_use_full_screen_buffer,
	       "Non-nil means make terminal frames use the full screen buffer dimensions.\nThis is desirable when running Emacs over telnet.\nA value of nil means use the current console window dimensions; this\nmay be preferable when working directly at the console with a large\nscroll-back buffer.  ");                                          /* C 1242-1248 */
  w32_use_full_screen_buffer = false;                                   /* C 1249 */

  DEFVAR_BOOL! (c"w32--terminal-is-conhost", w32__terminal_is_conhost,
	       "Non-nil means Emacs text-mode terminal is MS-Windows ConHost.\nIf nil, Emacs is displaying text-mode frames on the Windows Terminal.  ");  /* C 1251-1254 */
  w32__terminal_is_conhost = false;                                     /* C 1255 */

  defsubr (&raw mut Sset_screen_color);                                 /* C 1257 */
  defsubr (&raw mut Sget_screen_color);                                 /* C 1258 */
  defsubr (&raw mut Sw32_use_virtual_terminal);                         /* C 1259 */
  defsubr (&raw mut Sw32_use_virtual_terminal_p);                       /* C 1260 */
  defsubr (&raw mut Sset_cursor_size);                                  /* C 1261 */
}
