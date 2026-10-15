// femacs: port of src/w32select.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` and the C `static` objects plain `static mut`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32select.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 483, `W32_OBJ=@W32_OBJ@`, filled by the non-Cygwin
// `HAVE_W32` test of `configure.ac` 3023-3024, whose 3030-3031 add `w32select.o` to `W32_OBJ` before the
// Cygwin split at 3044 (corrected 2026-10-05, status 3.587) - the build whose `nt/inc/ms-w32.h` 25-27
// `#define`s `WINDOWSNT`), and `docs/baseline/frozen-build-sources.txt` lists no `w32select.o` for
// this target, so `extract-abi.py`'s checklist for the file is empty (P-86 (2)) and the ledger
// `docs/ports/w32select.md` carries that clause's substitute: the **C line-range table**, the
// `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_ntgui"` (corrected 2026-10-05, status 3.587: 3051 is the `else`
//     of the Cygwin split, not the line that adds this object; 3030-3031 add `w32select.o` for **both**
//     GUI arms), on every item *and on the include graph*, which
//     is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`, not `HAVE_NTGUI`:
//     `w32select.o` is added only by the MinGW arm of the split (the same arm that `#define`s the
//     name), and the file's text never tests `HAVE_NTGUI` - its one `#ifdef WINDOWSNT` (C 828) is
//     the file's own name, so the file gate selects that arm too;
//   * the file's own `#if`s are four, and `tools/check-cond-eval.py src/w32select.c` reads
//     `4 conditions, 0 unknown, 0 disagreements [VERIFIED]`: C 80 `#ifndef CF_DIBV5` (the frozen
//     table has no such macro, so C's own `#define`s are the live arm) -> `not (feature =
//     "cf_dibv5")`; C 93 `#ifdef CYGWIN` (undef here) -> `feature = "cygwin"`, dead arm; C 144
//     `#if TRACE` (`TRACE` is defined nowhere in the tree or the table, so C's `#else` is the live
//     arm) -> `feature = "trace"`, dead arm; C 828 `#ifdef WINDOWSNT` -> the file gate itself.  The
//     ledger's `## Gate mapping` table lists all four.  No other text in the file is conditional;
//   * the `<windows.h>`, `<wingdi.h>`, `<wtypes.h>` and `<gdiplus.h>` sections below stand where
//     the headers enter this translation unit (C 76-79).  The reference tree has no file for any of
//     them (P-88 (1): the declaration site is C's own `#include` line, and no file is added under
//     `femacs/src/include/`), so the names this file takes from them are declared there, each with
//     the SDK's own text above it.  The spellings repeat `src/w32fns.rs`'s and `src/w32proc.rs`'s
//     sections (`DWORD` is the 32-bit Windows unsigned whatever this target's `c_long` is,
//     `HANDLE`/`HWND`/`HGLOBAL` are `void *`, `BOOL` is C's `int`, `SIZE_T` is `usize`, `WINAPI`/
//     `CALLBACK` are `extern "system"`), so that a `windowsnt` translation unit sees one set of
//     names.  `DWORD_PTR` is `include/w32common.rs`'s and is not repeated here (a second
//     declaration is `E0428`);
//   * the three entry points C defines (`syms_of_w32select`, `globals_of_w32select`,
//     `term_w32select`) are declared at crate root by `include/w32select.rs` (C 88's header), which
//     is what this unit's own definitions collide with - the `docs/ports/w32heap.md` owed-2 shape,
//     reported in the ledger's `## Owed`;
//   * C's `#include <string.h>` and `#include <stdlib.h>` are reached through C 85's `lisp.h` 30 and
//     C 75's `conf_post.h` -> `ms-w32.h` 410; the port's `include/lisp.rs` carries the `<stdio.h>`
//     edge but not the `<string.h>` one, so this unit adds the two edges at their C 75/C 85
//     position (the ledger's `## Deviations`).
//
// **Acquisition note - this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and
// no mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h` anywhere), so there is no
// `<windows.h>` to compile or measure against: **every record and every value below is
// `UNVERIFIED`** (the widths and values are the documented Windows ones, not measurements) and the
// layout assertions are owed.  What the round does show is that the `windowsnt` arm type-checks
// under this machine's `rustc` - the ledger's `## Evidence` carries that reading and names the
// errors that belong to the include graph rather than to this unit.
//
// `docs/ports/w32select.md` carries the C line ranges, the acceptance readings, every deviation (the
// dropped forward declarations, the `<windows.h>` stand-in section, the `_memccpy`/`strstr`/
// `filename_to_utf16` declarations, the two `match`es that stand for C's two `switch`es, the labelled
// blocks that stand for C's `goto`s, the zero-initialised locals) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use,
         let_underscore_drop, clippy::all)]

/* C 1-17 is the file's opening block, dropped (PORTING.md 0.1-19); the port's first line is the
   revision line above.  C 19-21 is the rest of the title comment, and C 23-63 is the "Notes on
   usage of selection-coding-system and next-selection-coding-system" design comment - carried as
   C's own text at its C position.  */

/*
 * Notes on usage of selection-coding-system and
 * next-selection-coding-system on MS Windows:
 *
 * The selection coding system variables apply only to the version of
 * the clipboard data that is closest in type, i.e. when a 16-bit
 * Unicode coding system is given, they apply to he Unicode clipboard
 * (CF_UNICODETEXT), when a well-known console codepage is given, they
 * apply to the console version of the clipboard data (CF_OEMTEXT),
 * else they apply to the normal 8-bit text clipboard (CF_TEXT).
 *
 * When pasting (getting data from the OS), the clipboard format that
 * matches the {next-}selection-coding-system is retrieved.  If
 * Unicode is requested, but not available, 8-bit text (CF_TEXT) is
 * used.  In all other cases the OS will transparently convert
 * formats, so no other fallback is needed.
 *
 * When copying or cutting (sending data to the OS), the data is
 * announced and stored internally, but only actually rendered on
 * request.  The requestor determines the format provided.  The
 * {next-}selection-coding-system is only used, when its corresponding
 * clipboard type matches the type requested.
 *
 * Scenarios to use the facilities for customizing the selection
 * coding system are:
 *
 *   ;; Generally use KOI8-R instead of the russian MS codepage for
 *   ;; the 8-bit clipboard.
 *   (set-selection-coding-system 'koi8-r-dos)
 *
 * Or
 *
 *   ;; Create a special clipboard copy function that uses codepage
 *   ;; 1253 (Greek) to copy Greek text to a specific non-Unicode
 *   ;; application.
 *   (defun greek-copy (beg end)
 *     (interactive "r")
 *     (set-next-selection-coding-system 'cp1253-dos)
 *     (copy-region-as-kill beg end))
 *   (global-set-key "\C-c\C-c" 'greek-copy)
 */

/*
 * Ideas for further directions:                                  // C 65-72
 *
 * The encoding and decoding routines could be moved to Lisp code
 * similar to how xselect.c does it (using well-known routine names
 * for the delayed rendering).  If the definition of which clipboard
 * types should be supported is also moved to Lisp, functionality
 * could be expanded to CF_HTML, CF_RTF and maybe other types.
 */

/* C 75-91: the include graph.  Every header sits inside the file gate, as C's own build has it (no
   translation unit the frozen configuration builds reaches this file; P-86 (8)).  Where the
   reference tree carries two headers of one name, the one `nt/inc` overrides is the one included -
   that is what `<string.h>` and `<stdlib.h>` resolve to in the MinGW build.  C 76-79's four headers
   have no file in the reference tree; the names this file takes from them stand in the section
   below, at that C position.  */
include!("include/config.rs");                  // C 75: `#include <config.h>`
include!("include/conf_post.rs");               // C 75's AH_BOTTOM `#include <conf_post.h>`
#[cfg (feature = "have_ntgui")]
include!("../nt/inc/include/ms-w32.rs");        // C 75 -> `conf_post.h` 38: `#include <ms-w32.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/string.rs");           // C 85 -> `lisp.h` 30 / `ms-w32.h` 271: `#include <string.h>`
#[cfg (feature = "have_ntgui")]
include!("../lib/include/stdlib.rs");           // C 75 -> `ms-w32.h` 410: `#include <stdlib.h>` (`strtol`, `strtoul`)

/* ----------------------------------------------------------------------------------------------
   C 76-79: `<windows.h>`, `<wingdi.h>`, `<wtypes.h>`, `<gdiplus.h>`.

   No file in the reference tree carries any of the four (P-88 (1): the declaration site is C's own
   `#include` line, and no file is added under `femacs/src/include/`).  What follows is the names
   *this file* uses, with the SDK's own text above each one.

   **No measurement was possible**: this machine has no `<windows.h>` and no mingw toolchain (see
   the acquisition note), so every scalar is spelled from PORTING.md 1.1's fixed Windows widths and
   every record below is **UNVERIFIED** - its size, offsets and values are the documented ones and
   the layout assertions are owed (the ledger's `## Owed`).
   ---------------------------------------------------------------------------------------------- */

/// C 76's `<windows.h>`: the scalar types (`windef.h`/`winnt.h`).
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned long DWORD;
/// C: typedef long LONG; typedef unsigned long ULONG; typedef int BOOL; typedef unsigned int UINT;
/// C: typedef UINT_PTR WPARAM; typedef LONG_PTR LPARAM; typedef LONG_PTR LRESULT;
/// C: typedef ULONG_PTR SIZE_T; typedef ULONG_PTR UINT_PTR; typedef long long LONG_PTR;
/// C: typedef void *HANDLE, *HWND, *HINSTANCE, *HMENU, *HGLOBAL, *LPVOID; typedef void *LPSTR;
/// C: typedef const char *LPCSTR; typedef const void *LPCVOID;
/// C: typedef DWORD LCID; typedef WORD LANGID; typedef WORD ATOM; typedef DWORD LCTYPE;
/// C: typedef wchar_t WCHAR;
#[cfg (feature = "have_ntgui")]
pub type BYTE = u8;
#[cfg (feature = "have_ntgui")]
pub type WORD = c_ushort;
#[cfg (feature = "have_ntgui")]
pub type DWORD = u32;
#[cfg (feature = "have_ntgui")]
pub type LONG = i32;
#[cfg (feature = "have_ntgui")]
pub type ULONG = u32;
#[cfg (feature = "have_ntgui")]
pub type BOOL = c_int;
#[cfg (feature = "have_ntgui")]
pub type UINT = c_uint;
#[cfg (feature = "have_ntgui")]
pub type UINT_PTR = usize;
#[cfg (feature = "have_ntgui")]
pub type LONG_PTR = isize;
#[cfg (feature = "have_ntgui")]
pub type SIZE_T = usize;
#[cfg (feature = "have_ntgui")]
pub type WPARAM = UINT_PTR;
#[cfg (feature = "have_ntgui")]
pub type LPARAM = LONG_PTR;
#[cfg (feature = "have_ntgui")]
pub type LRESULT = LONG_PTR;
#[cfg (feature = "have_ntgui")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HWND = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HMENU = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type HGLOBAL = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "have_ntgui")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "have_ntgui")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "have_ntgui")]
pub type LPCVOID = *const c_void;
#[cfg (feature = "have_ntgui")]
pub type LCID = DWORD;
#[cfg (feature = "have_ntgui")]
pub type LANGID = WORD;
#[cfg (feature = "have_ntgui")]
pub type ATOM = WORD;
#[cfg (feature = "have_ntgui")]
pub type LCTYPE = DWORD;
#[cfg (feature = "have_ntgui")]
pub type WCHAR = c_ushort;

/// C 76's `<windows.h>`: `TRUE`/`FALSE`, and the `WINAPI`/`CALLBACK` convention.  The two spelling
/// macros are `__stdcall`, which Rust spells `extern "system"` - the names below are that
/// convention, and no macro item is written for them (`src/w32fns.rs`'s reading).
/// C: #define FALSE 0
/// C: #define TRUE 1
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define MAX_PATH 260
#[cfg (feature = "have_ntgui")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "have_ntgui")]
pub const TRUE: BOOL = 1;
#[cfg (feature = "have_ntgui")]
pub const MAX_PATH: usize = 260;

/// C 76's `<windows.h>`: the window-procedure type and the window classes.  `WNDCLASSA` is the
/// record C's `RegisterClass` call builds - C stores a `char[]` in `lpszClassName` (C 466) and
/// `char[]` in `CreateWindow`'s first argument (C 473), i.e. the ANSI half, so the port spells that
/// half's record and keeps C's own name for the alias C uses.
/// C: typedef LRESULT (CALLBACK *WNDPROC) (HWND, UINT, WPARAM, LPARAM);
/// C: typedef struct tagWNDCLASSA { UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int cbWndExtra;
/// C:   HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground;
/// C:   LPCSTR lpszMenuName; LPCSTR lpszClassName; } WNDCLASSA;
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
  pub hIcon: *mut c_void,
  pub hCursor: *mut c_void,
  pub hbrBackground: *mut c_void,
  pub lpszMenuName: LPCSTR,
  pub lpszClassName: LPCSTR,
}

/// C 76's `<windows.h>`: the locale enumeration callback C hands to `EnumSystemLocales` (C 566).
/// C: typedef BOOL (CALLBACK *LOCALE_ENUMPROCA) (LPSTR);
#[cfg (feature = "have_ntgui")]
pub type LOCALE_ENUMPROCA = Option<unsafe extern "system" fn (LPSTR) -> BOOL>;

/// C 76's `<windows.h>`: the clipboard and locale constants.  C 80-84's `#ifndef CF_DIBV5` block is
/// carried below at that position; the values here are `winuser.h`'s and `winnls.h`'s own.
/// C: #define GMEM_MOVEABLE 0x0002
/// C: #define GMEM_DDESHARE 0x2000
/// C: #define CF_TEXT 1 / #define CF_OEMTEXT 7 / #define CF_UNICODETEXT 13 / #define CF_LOCALE 16
/// C: #define CF_MAX 18
/// C: #define WM_DESTROY 0x0002 / #define WM_RENDERFORMAT 0x0305
/// C: #define WM_RENDERALLFORMATS 0x0306 / #define WM_DESTROYCLIPBOARD 0x0307
/// C: #define LCID_INSTALLED 0x00000001
/// C: #define LOCALE_IDEFAULTANSICODEPAGE 0x00001004
/// C: #define LOCALE_IDEFAULTCODEPAGE 0x0000000B
/// C: #define CP_ACP 0 / #define CP_OEMCP 1
/// C: #define SORT_DEFAULT 0x0
/// C: #define LOCALE_NEUTRAL MAKELCID (MAKELANGID (LANG_NEUTRAL, SUBLANG_NEUTRAL), SORT_DEFAULT)
#[cfg (feature = "have_ntgui")]
pub const GMEM_MOVEABLE: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const GMEM_DDESHARE: UINT = 0x2000;
#[cfg (feature = "have_ntgui")]
pub const CF_TEXT: UINT = 1;
#[cfg (feature = "have_ntgui")]
pub const CF_OEMTEXT: UINT = 7;
#[cfg (feature = "have_ntgui")]
pub const CF_UNICODETEXT: UINT = 13;
#[cfg (feature = "have_ntgui")]
pub const CF_LOCALE: UINT = 16;
#[cfg (feature = "have_ntgui")]
pub const WM_DESTROY: UINT = 0x0002;
#[cfg (feature = "have_ntgui")]
pub const WM_RENDERFORMAT: UINT = 0x0305;
#[cfg (feature = "have_ntgui")]
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
#[cfg (feature = "have_ntgui")]
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
#[cfg (feature = "have_ntgui")]
pub const LCID_INSTALLED: DWORD = 0x00000001;
#[cfg (feature = "have_ntgui")]
pub const LOCALE_IDEFAULTANSICODEPAGE: LCTYPE = 0x00001004;
#[cfg (feature = "have_ntgui")]
pub const LOCALE_IDEFAULTCODEPAGE: LCTYPE = 0x0000000B;
#[cfg (feature = "have_ntgui")]
pub const CP_ACP: UINT = 0;
#[cfg (feature = "have_ntgui")]
pub const CP_OEMCP: UINT = 1;
#[cfg (feature = "have_ntgui")]
pub const SORT_DEFAULT: DWORD = 0x0;
/// C's `LOCALE_NEUTRAL` is `MAKELCID (MAKELANGID (LANG_NEUTRAL, SUBLANG_NEUTRAL), SORT_DEFAULT)`,
/// and all three of `LANG_NEUTRAL`/`SUBLANG_NEUTRAL`/`SORT_DEFAULT` are 0, so the value is 0.
#[cfg (feature = "have_ntgui")]
pub const LOCALE_NEUTRAL: LCID = 0;

/// C 76's `<windows.h>`: the two macro pairs C's text uses (C 1086, C 1359 and C 1361).
/// C: #define MAKELCID(lgid, srtid) ((LCID)(((DWORD)(srtid) << 16) | ((DWORD)(lgid))))
/// C: #define LANGIDFROMLCID(lcid) ((LANGID)(lcid))
#[cfg (feature = "have_ntgui")]
macro_rules! MAKELCID {
  ($lgid:expr, $srtid:expr) => { (((($srtid) as DWORD) << 16) | (($lgid) as DWORD)) as LCID };
}
#[cfg (feature = "have_ntgui")]
macro_rules! LANGIDFROMLCID {
  ($lcid:expr) => { ($lcid) as LANGID };
}

// C 76's `<windows.h>`: the two window entry points C 468-473 calls, `DefWindowProc` (C 459) and
// the clipboard family (C 235 + 1298 and the rest of the file).  Each is the SDK's A/W alias; C's
// text writes the alias and the port keeps C's name, so a Windows target supplies whichever entry
// point its own `#define` picks (PORTING.md 0.1's rename convention, the `src/w32fns.rs` 305-312
// spelling of `lstrcmpi`).
// C: WINUSERAPI BOOL WINAPI RegisterClassA (const WNDCLASSA *);
// C: WINUSERAPI HWND WINAPI CreateWindowA (LPCSTR, LPCSTR, DWORD, int, int, int, int, HWND, HMENU,
// C:                                      HINSTANCE, LPVOID);
// C: WINUSERAPI BOOL WINAPI DestroyWindow (HWND);
// C: WINUSERAPI LRESULT WINAPI DefWindowProcA (HWND, UINT, WPARAM, LPARAM);
// C: WINUSERAPI BOOL WINAPI EnumSystemLocalesA (LOCALE_ENUMPROCA, DWORD);
// C: WINUSERAPI int WINAPI GetLocaleInfoA (LCID, LCTYPE, LPSTR, int);
// C: WINUSERAPI LCID WINAPI GetUserDefaultLCID (void);
// C: WINUSERAPI UINT WINAPI GetACP (void);
// C: WINUSERAPI UINT WINAPI GetOEMCP (void);
// C: WINBASEAPI int WINAPI lstrlenW (LPCWSTR);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn RegisterClass (lpWndClass: *const WNDCLASS) -> ATOM;
  pub fn CreateWindow (lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD, x: c_int,
                       y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU,
                       hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
  pub fn DestroyWindow (hWnd: HWND) -> BOOL;
  pub fn DefWindowProc (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn EnumSystemLocales (lpLocaleEnumProc: LOCALE_ENUMPROCA, dwFlags: DWORD) -> BOOL;
  pub fn GetLocaleInfo (Locale: LCID, LCType: LCTYPE, lpLCData: LPSTR, cchData: c_int) -> c_int;
  pub fn GetUserDefaultLCID () -> LCID;
  pub fn GetACP () -> UINT;
  pub fn GetOEMCP () -> UINT;
  pub fn lstrlenW (lpString: *const c_ushort) -> c_int;
}

// C 76's `<windows.h>`: the clipboard entry points and the global-memory family (C 172-206,
// C 235-311, C 680-767, C 1211-1305).
// C: WINUSERAPI BOOL WINAPI OpenClipboard (HWND);
// C: WINUSERAPI BOOL WINAPI CloseClipboard (void);
// C: WINUSERAPI BOOL WINAPI EmptyClipboard (void);
// C: WINUSERAPI HANDLE WINAPI SetClipboardData (UINT, HANDLE);
// C: WINUSERAPI HANDLE WINAPI GetClipboardData (UINT);
// C: WINUSERAPI UINT WINAPI EnumClipboardFormats (UINT);
// C: WINUSERAPI int WINAPI GetClipboardFormatNameA (UINT, LPSTR, int);
// C: WINBASEAPI HGLOBAL WINAPI GlobalAlloc (UINT, SIZE_T);
// C: WINBASEAPI LPVOID WINAPI GlobalLock (HGLOBAL);
// C: WINBASEAPI BOOL WINAPI GlobalUnlock (HGLOBAL);
// C: WINBASEAPI HGLOBAL WINAPI GlobalFree (HGLOBAL);
// C: WINBASEAPI SIZE_T WINAPI GlobalSize (HGLOBAL);
#[cfg (feature = "have_ntgui")]
unsafe extern "system" {
  pub fn OpenClipboard (hWndNewOwner: HWND) -> BOOL;
  pub fn CloseClipboard () -> BOOL;
  pub fn EmptyClipboard () -> BOOL;
  pub fn SetClipboardData (uFormat: UINT, hMem: HANDLE) -> HANDLE;
  pub fn GetClipboardData (uFormat: UINT) -> HANDLE;
  pub fn EnumClipboardFormats (format: UINT) -> UINT;
  pub fn GetClipboardFormatName (format: UINT, lpszFormatName: LPSTR, cchMaxCount: c_int)
    -> c_int;
  pub fn GlobalAlloc (uFlags: UINT, dwBytes: SIZE_T) -> HGLOBAL;
  pub fn GlobalLock (hMem: HGLOBAL) -> LPVOID;
  pub fn GlobalUnlock (hMem: HGLOBAL) -> BOOL;
  pub fn GlobalFree (hMem: HGLOBAL) -> HGLOBAL;
  pub fn GlobalSize (hMem: HGLOBAL) -> SIZE_T;
}

/* C 80-84: `#ifndef CF_DIBV5` - "define it if the SDK has not".  The frozen table has no CF_DIBV5
   (the `windows.h` chain is not in the tree), so C's own `#define`s are the arm carried here, and
   the other arm (a SDK that defines it) carries the same two values, which `winuser.h` defines
   itself.  `CF_MAX` is C's `#undef`+`#define` pair: one name per scope in Rust, so it is written
   once with the value both arms give it.  */
#[cfg (all (feature = "have_ntgui", not (feature = "cf_dibv5")))]
pub const CF_DIBV5: UINT = 17;
#[cfg (all (feature = "have_ntgui", not (feature = "cf_dibv5")))]
pub const CF_MAX: UINT = 18;
#[cfg (all (feature = "have_ntgui", feature = "cf_dibv5"))]
pub const CF_DIBV5: UINT = 17;
#[cfg (all (feature = "have_ntgui", feature = "cf_dibv5"))]
pub const CF_MAX: UINT = 18;

/* C 79's `<gdiplus.h>` (and C 91's `w32gdiplus.h`, whose mirror leaves the GDI+ types to its
   includer - its own head note says so).  C 826-864's `convert_dibv5_to_png` names these.  */
/// C: typedef INT GpStatus; typedef enum Status { Ok = 0, GenericError, ... } Status;
/// C: #define PixelFormat32bppARGB (10 | (32 << 8) | PixelFormatAlpha | PixelFormatGDI |
/// C:   PixelFormatCanonical)
/// C: typedef enum RotateFlipType { RotateNoneFlipNone = 0, Rotate90FlipNone, Rotate180FlipNone,
/// C:   Rotate270FlipNone, RotateNoneFlipX, Rotate90FlipX, Rotate180FlipX, Rotate270FlipX,
/// C:   RotateNoneFlipY = Rotate180FlipX, ... } RotateFlipType;
#[cfg (feature = "have_ntgui")]
pub type GpStatus = c_int;
#[cfg (feature = "have_ntgui")]
pub type GpBitmap = *mut c_void;
/// C's GDI+ handle and scalar names that `include/w32gdiplus.rs`'s typedef list *uses* without
/// declaring (its own head note leaves them to the includer: "they come from `<windows.h>` and
/// GDI+, the tree has no mirror of either, and the Windows target supplies them").  The three
/// handles are `void *` (`GDI+`'s `GpImage`/`GpGraphics`/`GpBitmap` are incomplete types C only
/// ever points at), which is what makes the mirror's `*mut GpBitmap` and `*mut GpImage` the same
/// Rust type - C 845's `GpBitmap **` argument and C 857's `GpImage *` parameter then agree, as
/// C's own (warning-grade) conversion does.  **UNVERIFIED**: no `gdiplus.h` on this machine.
/// C: typedef INT GpStatus; typedef struct GpImage GpImage; typedef struct GpGraphics GpGraphics;
/// C: typedef struct GpBitmap GpBitmap; typedef ULONG_PTR ULONG_PTR; typedef INT INT;
/// C: typedef UINT32 ARGB; typedef UINT32 PROPID; typedef int InterpolationMode; typedef int GpUnit;
/// C: typedef void *HDC, *HBITMAP, *HPALETTE, *IStream; typedef VOID (*DrawImageAbort) (VOID *);
/// C: typedef BOOL (*GetThumbnailImageAbort) (VOID *);
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GpImage = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GpGraphics = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type HDC = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type HBITMAP = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type HPALETTE = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type IStream = *mut c_void;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type ULONG_PTR = usize;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type INT = c_int;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type ARGB = u32;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type PROPID = u32;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type InterpolationMode = c_int;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GpUnit = c_int;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type DrawImageAbort = Option<unsafe extern "system" fn (*mut c_void) -> BOOL>;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn (*mut c_void) -> BOOL>;
/// C's five GDI+ records that only ever cross this graph behind a pointer (the mirror's typedef
/// list names them; its own note leaves their text to the includer).  PORTING.md 1.2's
/// incomplete-type shape: the address is what the declarations use, and the layouts are owed.
/// C: typedef struct GdiplusStartupInput { ... } GdiplusStartupInput;
/// C: typedef struct GdiplusStartupOutput { ... } GdiplusStartupOutput;
/// C: typedef struct PropertyItem { ... } PropertyItem;
/// C: typedef struct ImageCodecInfo { ... } ImageCodecInfo;
/// C: typedef struct EncoderParameters { ... } EncoderParameters;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct GdiplusStartupInput { _private: [u8; 0] }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct GdiplusStartupOutput { _private: [u8; 0] }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct PropertyItem { _private: [u8; 0] }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct ImageCodecInfo { _private: [u8; 0] }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
pub struct EncoderParameters { _private: [u8; 0] }
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type PixelFormat = c_int;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type RotateFlipType = c_int;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const Ok: GpStatus = 0;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const RotateNoneFlipY: RotateFlipType = 6;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const PixelFormatAlpha: PixelFormat = 0x00040000;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const PixelFormatGDI: PixelFormat = 0x00020000;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const PixelFormatCanonical: PixelFormat = 0x00200000;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub const PixelFormat32bppARGB: PixelFormat =
  10 | (32 << 8) | PixelFormatAlpha | PixelFormatGDI | PixelFormatCanonical;

/// C 77's `<wingdi.h>`: the DIB record C 834-846 reads and the `BI_BITFIELDS` compression code.
/// `FXPT2DOT30` is `LONG`, `CIEXYZ` is three of them and `CIEXYZTRIPLE` three `CIEXYZ`s, so
/// `bV5Endpoints` is nine `LONG`s.  **UNVERIFIED** (owed: a Windows target measures
/// `sizeof (BITMAPV5HEADER)` and the offsets of `bV5SizeImage`, `bV5Height`, `bV5Size`,
/// `bV5ClrUsed`, `bV5Compression` and `bV5Width`).
/// C: typedef LONG FXPT2DOT30;
/// C: typedef struct { FXPT2DOT30 ciexyzX, ciexyzY, ciexyzZ; } CIEXYZ;
/// C: typedef struct { CIEXYZ ciexyzRed, ciexyzGreen, ciexyzBlue; } CIEXYZTRIPLE;
/// C: typedef struct tagBITMAPV5HEADER { DWORD bV5Size; LONG bV5Width; LONG bV5Height;
/// C:   WORD bV5Planes; WORD bV5BitCount; DWORD bV5Compression; DWORD bV5SizeImage;
/// C:   LONG bV5XPelsPerMeter; LONG bV5YPelsPerMeter; DWORD bV5ClrUsed; DWORD bV5ClrImportant;
/// C:   DWORD bV5RedMask; DWORD bV5GreenMask; DWORD bV5BlueMask; DWORD bV5AlphaMask;
/// C:   DWORD bV5CSType; CIEXYZTRIPLE bV5Endpoints; DWORD bV5GammaRed; DWORD bV5GammaGreen;
/// C:   DWORD bV5GammaBlue; DWORD bV5Intent; DWORD bV5ProfileData; DWORD bV5ProfileSize;
/// C:   DWORD bV5Reserved; } BITMAPV5HEADER;
/// C: #define BI_BITFIELDS 3L
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type FXPT2DOT30 = LONG;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CIEXYZ
{
  pub ciexyzX: FXPT2DOT30,
  pub ciexyzY: FXPT2DOT30,
  pub ciexyzZ: FXPT2DOT30,
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CIEXYZTRIPLE
{
  pub ciexyzRed: CIEXYZ,
  pub ciexyzGreen: CIEXYZ,
  pub ciexyzBlue: CIEXYZ,
}
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
/// C: typedef struct tagRGBQUAD { BYTE rgbBlue; BYTE rgbGreen; BYTE rgbRed; BYTE rgbReserved; }
/// C:   RGBQUAD;
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
pub const BI_BITFIELDS: DWORD = 3;

/// C 77's `<wingdi.h>` / C 79's `<gdiplus.h>`: the GDI+ entry points C 843-858 calls.  The port's
/// `include/w32gdiplus.rs` carries C 95-141's `#undef`/`#define` pairs as the macros
/// `GdipCreateBitmapFromScan0!` etc. (each yielding the `fn_*` object C's `#define` names), so a
/// call site is `(GdipCreateBitmapFromScan0! ().unwrap ()) (...)` - C's own `#define` plus the
/// port's nullable function pointer (`include/w32.rs` 323-329's reading).
/// C: extern int w32_gdip_get_encoder_clsid (const char *, CLSID *);
/// C: WINGDIPAPI GpStatus GdipCreateBitmapFromScan0 (INT, INT, INT, PixelFormat, BYTE *, GpBitmap **);
/// C: WINGDIPAPI GpStatus GdipImageRotateFlip (GpImage *, RotateFlipType);
/// C: WINGDIPAPI GpStatus GdipSaveImageToFile (GpImage *, GDIPCONST WCHAR *, GDIPCONST CLSID *,
/// C:   GDIPCONST EncoderParameters *);
/// C: WINGDIPAPI GpStatus GdipDisposeImage (GpImage *);

/// C 77's `<wingdi.h>`: the CLSID C 829 declares, spelled from `_GUID`'s member list.  No mirror
/// carries it (`include/w32gdiplus.rs` leaves it to its includer by its own note).
/// C: typedef struct _GUID { unsigned long Data1; unsigned short Data2; unsigned short Data3;
/// C:   unsigned char Data4[8]; } GUID;
/// C: typedef GUID CLSID;
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID
{
  pub Data1: u32,
  pub Data2: u16,
  pub Data3: u16,
  pub Data4: [u8; 8],
}
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
pub type CLSID = GUID;

#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/lisp.rs");                    // C 85: `#include "lisp.h"` (its C 34 `#include <stdio.h>`: `fprintf`, `sprintf`, `stderr`)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32common.rs");               // C 86: `#include "w32common.h"` (`os_subtype`, `OS_SUBTYPE_NT`)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32term.rs");                 // C 87: `#include "w32term.h"` (`ALIGN_STACK`, `w32_gdiplus_startup`)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32select.rs");               // C 88: `#include "w32select.h"`
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/blockinput.rs");              // C 89: `#include "blockinput.h"` (`block_input`, `unblock_input`)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/coding.rs");                  // C 90: `#include "coding.h"` (`coding_system`, `CODING_*`)
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32gdiplus.rs");              // C 91: `#include "w32gdiplus.h"` (the `Gdip*` entry-point macros)

/* C 93-97: `#ifdef CYGWIN` - the Cygwin GUI build's two extra headers and the `_memccpy` rename.
   `CYGWIN` is `undef` in the frozen table, so this is the dead arm carried whole.  */
/* C 94-95: the `CYGWIN` arm's two headers.  They are recorded as C's own lines rather than as
   `include!` edges, because their targets are already in this graph at C 85 (`lisp.h` 30's
   `<string.h>`, `lisp.h` 34's `<stdio.h>`): `xtask`'s `materialize` degrades a *gated* edge whose
   target has an ungated reach into a comment (P-89 (3), status 3.429) and leaves the edge's
   `#[cfg]` attribute attached to the next item, so the faithful `include!` spelling made this
   unit's own `unsafe extern` block `cygwin`-gated and its four declarations invisible - measured,
   four `E0425`s in this unit's text (`## Evidence` (2)).  The arm is dead in every configuration
   this tree builds (`CYGWIN` is `undef`), its text is C's, and the `_memccpy` rename it installs
   is recorded below.  */
// C 94: `#include <string.h>`
// C 95: `#include <stdio.h>`
// C 96: `#define _memccpy memccpy` - the rename the Cygwin arm installs.  The port declares the
// runtime's own name below (the arm carried here is the non-Cygwin one) and this note is C's
// `#define`; the ledger's `## Deviations` records it.

/* C 85's `lisp.h`, C 96's runtime and C 90's `coding.h`: the names this file reaches that no mirror
   in its graph carries.  Each stands with the header's own text (P-88 (1)); the ledger's `## Owed`
   names the mirror rows.  */
// C: lisp.h 4806: `extern Lisp_Object internal_condition_case_1 (Lisp_Object (*) (Lisp_Object),
// C:   Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object));`
// C: lisp.h 5099: `extern void cmd_error_internal (Lisp_Object, const char *);`
// C: `<string.h>`: `char *strstr (const char *, const char *);` - the port's
//   `lib/include/string.rs` carries `memcpy`/`memset`/`memcmp`/`strlen`/`strcmp`/`strcpy`/`memchr`
//   but not `strstr` (gnulib's `string.in.rs` 248-253 owns the `rpl_strstr` rename and is not in
//   this unit's graph).
// C: `<string.h>`, the runtime's own name: `void *_memccpy (void *, const void *, int, size_t);`
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
                                    arg: Lisp_Object, handlers: Lisp_Object,
                                    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  pub fn cmd_error_internal (error: Lisp_Object, msg: *const c_char);
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
  pub fn _memccpy (dest: *mut c_void, src: *const c_void, c: c_int, count: SIZE_T) -> *mut c_void;
}
// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);` -
// `include/coding.rs` carries `encode_coding_object` and `decode_coding_object` but not this one.
#[cfg (feature = "have_ntgui")]
unsafe extern "C" {
  pub fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
}
// `filename_to_utf16` is declared in `w32.h` 209 and **nowhere in this file's include graph**
// (C 87's `w32term.h` reaches `w32gui.h`/`frame.h`/`atimer.h`, none of which includes `w32.h`), so
// C 856's call is an implicit declaration - the ledger's Findings.  The declaration stands here
// with the header's own text.
// C: w32.h 209: `extern int  filename_to_utf16 (const char *, wchar_t *);`
#[cfg (all (feature = "have_ntgui", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn filename_to_utf16 (ansi: *const c_char, wide: *mut WCHAR) -> c_int;
}

/* C 99-116: C's forward declarations of this file's own functions.  Rust has no forward
   declarations (PORTING.md 0.1-12: a declaration plus a definition is `E0428`, and items are
   order-independent), so the block has no counterpart; every definition stands at its C position.
   C: static HGLOBAL convert_to_handle_as_ascii (void);
   C: static HGLOBAL convert_to_handle_as_coded (Lisp_Object coding_system);
   C: static Lisp_Object render (Lisp_Object oformat);
   C: static Lisp_Object render_locale (void);
   C: static Lisp_Object render_all (Lisp_Object ignore);
   C: static void run_protected (Lisp_Object (*code) (Lisp_Object), Lisp_Object arg);
   C: static Lisp_Object lisp_error_handler (Lisp_Object error);
   C: static LRESULT CALLBACK ALIGN_STACK owner_callback (HWND win, UINT msg, WPARAM wp, LPARAM lp);
   C: static HWND create_owner (void);
   C: static void setup_config (void);
   C: static BOOL WINAPI enum_locale_callback (/*const*/ char* loc_string);
   C: static UINT cp_from_locale (LCID lcid, UINT format);
   C: static Lisp_Object coding_from_cp (UINT codepage);
   C: static Lisp_Object validate_coding_system (Lisp_Object coding_system);
   C: static void setup_windows_coding_system (Lisp_Object coding_system,
                                               struct coding_system * coding); */

/* C 118-142: "Internal pseudo-constants, initialized in globals_of_w32select() based on current
   system parameters", the clipboard window, and the configured/current transfer state.  C leaves
   every one of them zero (static storage); PORTING.md 1.5's zero-initialisation is that value, and
   a `Lisp_Object` is `lisp_h_Qnil ()` (`NIL_IS_ZERO`, lisp.h 1999).  */
/// C 120: `static LCID DEFAULT_LCID;`
#[cfg (feature = "have_ntgui")]
static mut DEFAULT_LCID: LCID = 0;
/// C 121: `static UINT ANSICP, OEMCP;`
#[cfg (feature = "have_ntgui")]
static mut ANSICP: UINT = 0;
#[cfg (feature = "have_ntgui")]
static mut OEMCP: UINT = 0;
/// C 122: `static Lisp_Object QANSICP, QOEMCP;` (this file's own two symbols, not the generated
/// `globals.rs` set: C 1338-1339 and C 1362-1363 are their only writers).
#[cfg (feature = "have_ntgui")]
static mut QANSICP: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_ntgui")]
static mut QOEMCP: Lisp_Object = lisp_h_Qnil ();
/// C 125: `static HWND clipboard_owner;`
#[cfg (feature = "have_ntgui")]
static mut clipboard_owner: HWND = core::ptr::null_mut ();
/// C 126-128: "A flag to tell WM_DESTROYCLIPBOARD who is to blame this time (just checking
/// GetClipboardOwner() doesn't work, sadly)."
#[cfg (feature = "have_ntgui")]
static mut modifying_clipboard: c_int = 0;
/// C 130-135: "Configured transfer parameters, based on the last inspection of
/// selection-coding-system."
#[cfg (feature = "have_ntgui")]
static mut cfg_coding_system: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_ntgui")]
static mut cfg_codepage: UINT = 0;
#[cfg (feature = "have_ntgui")]
static mut cfg_lcid: LCID = 0;
#[cfg (feature = "have_ntgui")]
static mut cfg_clipboard_type: UINT = 0;
/// C 137-142: "The current state for delayed rendering."
#[cfg (feature = "have_ntgui")]
static mut current_text: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_ntgui")]
static mut current_coding_system: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_ntgui")]
static mut current_requires_encoding: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut current_num_nls: c_int = 0;
#[cfg (feature = "have_ntgui")]
static mut current_clipboard_type: UINT = 0;
#[cfg (feature = "have_ntgui")]
static mut current_lcid: LCID = 0;

/* C 144-148: `#if TRACE` - the trace statements the three `ONTRACE` arms carry.  `TRACE` is not
   defined in this tree or in the frozen macro table, so C's `#else` (the empty arm) is the one this
   build compiles; both arms are carried, and the one-item-per-name rule keeps the two
   `macro_rules!`s in their `cfg` arms.  */
#[cfg (feature = "trace")]
macro_rules! ONTRACE { ($($stmt:tt)*) => { $($stmt)* }; }
#[cfg (not (feature = "trace"))]
macro_rules! ONTRACE { ($($stmt:tt)*) => {}; }

/* C 151-153: "This function assumes that there is no multibyte character in current_text, so we can
   short-cut encoding."  */
/// C 154-208: `convert_to_handle_as_ascii`.
#[cfg (feature = "have_ntgui")]
unsafe fn convert_to_handle_as_ascii () -> HGLOBAL
{
  let mut htext: HGLOBAL = core::ptr::null_mut ();   // C 156: `HGLOBAL htext = NULL;`
  let mut nbytes: c_int;                             // C 157
  let mut truelen: c_int;                            // C 158
  let mut src: *mut u8;                              // C 159: `unsigned char *src;`
  let mut dst: *mut u8;                              // C 160

  ONTRACE! (fprintf (stderr, c"convert_to_handle_as_ascii\n".as_ptr ()));        // C 163

  nbytes = SBYTES (current_text) as c_int + 1;       // C 165
  src = SDATA (current_text);                        // C 166

  /* C 168-171: "We need to add to the size the number of LF chars where we have to insert CR chars
     (the standard CF_TEXT clipboard format uses CRLF line endings, while Emacs uses just LF
     internally)."  */
  truelen = nbytes + current_num_nls;                // C 173

  htext = GlobalAlloc (GMEM_MOVEABLE | GMEM_DDESHARE, truelen as SIZE_T);        // C 175
  if htext.is_null ()                                // C 175-176
    { return core::ptr::null_mut (); }

  dst = GlobalLock (htext) as *mut u8;               // C 178
  if dst.is_null ()                                  // C 178-182
    {
      GlobalFree (htext);
      return core::ptr::null_mut ();
    }

  /* C 184: "convert to CRLF line endings expected by clipboard" */
  loop                                               // C 185
    {
      let next: *mut u8;                             // C 187
      /* C 188: "copy next line or remaining bytes including '\0'" */
      next = _memccpy (dst as *mut c_void, src as *const c_void, b'\n' as c_int,
                       nbytes as SIZE_T) as *mut u8;
      if !next.is_null ()                            // C 189
	{
	  /* C 191: "copied one line ending with '\n'" */
	  let copied: c_int = (next as isize - dst as isize) as c_int;   // C 192
	  nbytes -= copied;                                              // C 193
	  src = src.offset (copied as isize);                            // C 194
	  /* C 195: "insert '\r' before '\n'" */
	  *next.offset (-1) = b'\r';                                     // C 196
	  *next = b'\n';                                                 // C 197
	  dst = next.offset (1);                                         // C 198
	}
      else
	/* C 201: "copied remaining partial line -> now finished" */
	{ break; }                                                   // C 202
    }

  GlobalUnlock (htext);                              // C 205

  htext                                              // C 207
}

/* C 210-213: "This function assumes that there are multibyte or NUL characters in current_text, or
   that we need to construct Unicode.  It runs the text through the encoding machinery."  */
/// C 214-248: `convert_to_handle_as_coded`.
#[cfg (feature = "have_ntgui")]
unsafe fn convert_to_handle_as_coded (coding_system: Lisp_Object) -> HGLOBAL
{
  let mut htext: HGLOBAL;                            // C 216
  let mut dst: *mut u8 = core::ptr::null_mut ();     // C 217: `unsigned char *dst = NULL;`
  let mut coding: coding_system = core::mem::zeroed ();  // C 218 (zero-initialised, PORTING.md 1.5)

  ONTRACE! (fprintf (stderr, c"convert_to_handle_as_coded: %s\n".as_ptr (),
		     SDATA (SYMBOL_NAME (coding_system))));               // C 221-222

  setup_windows_coding_system (coding_system, &raw mut coding);           // C 224
  coding.dst_bytes = SBYTES (current_text) * 2;                           // C 225
  coding.destination = xmalloc (coding.dst_bytes as usize) as *mut u8;    // C 226
  encode_coding_object (&raw mut coding, current_text, 0, 0,
			SCHARS (current_text), SBYTES (current_text), Qnil ());  // C 227-228

  htext = GlobalAlloc (GMEM_MOVEABLE | GMEM_DDESHARE, (coding.produced + 2) as SIZE_T);  // C 230

  if !htext.is_null ()                               // C 232
    { dst = GlobalLock (htext) as *mut u8; }

  if !dst.is_null ()                                 // C 234
    {
      memcpy (dst as *mut c_void, coding.destination as *const c_void,
	      coding.produced as usize);                                         // C 236
      /* C 237-238: "Add the string terminator.  Add two NULs in case we are producing Unicode
	 here." - C's chained store is two stores in Rust.  */
      *dst.offset (coding.produced) = 0;
      *dst.offset (coding.produced + 1) = 0;

      GlobalUnlock (htext);                                                  // C 240
    }

  xfree (coding.destination as *mut c_void);                                 // C 243

  htext                                                                      // C 245
}

/* C 250-296: `render`.  C's `Lisp_Object (*code) (Lisp_Object)` parameter of `run_protected`
   (C 393) and of `internal_condition_case_1` (C 408) takes this function's address, so it is
   `extern "C"` like the helper PORTING.md 1.4 requires of every address-taken `static`.  */
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn render (oformat: Lisp_Object) -> Lisp_Object
{
  let mut htext: HGLOBAL = core::ptr::null_mut ();   // C 252
  let mut format: UINT = XFIXNAT (oformat) as UINT;  // C 253

  ONTRACE! (fprintf (stderr, c"render\n".as_ptr ()));                          // C 256

  if NILP (current_text)                             // C 258
    { return Qnil (); }

  if current_requires_encoding != 0 || format == CF_UNICODETEXT   // C 260
    {
      if format == current_clipboard_type                           // C 262
	{ htext = convert_to_handle_as_coded (current_coding_system); }   // C 263
      else
	{
	match format                                               // C 266
	  {
	    CF_UNICODETEXT =>                                      // C 268
	      {
		htext = convert_to_handle_as_coded (Qutf_16le_dos ());    // C 269
	      }
	    CF_TEXT | CF_OEMTEXT =>                                // C 271-272
	      {
		let mut cs: Lisp_Object;                           // C 274
		cs = coding_from_cp (cp_from_locale (current_lcid, format));   // C 275
		htext = convert_to_handle_as_coded (cs);           // C 276
	      }
	    _ => {}
	  }
	}
    }
  else
    { htext = convert_to_handle_as_ascii (); }                     // C 288

  ONTRACE! (fprintf (stderr, c"render: htext = 0x%08X\n".as_ptr (), htext as c_uint));  // C 284

  if htext.is_null ()                                            // C 286
    { return Qnil (); }

  if SetClipboardData (format, htext).is_null ()                  // C 288
    {
      GlobalFree (htext);                                         // C 290
      return Qnil ();
    }

  Qt ()                                                          // C 294
}

/// C 298-329: `render_locale`.
#[cfg (feature = "have_ntgui")]
unsafe fn render_locale () -> Lisp_Object
{
  let mut hlocale: HANDLE = core::ptr::null_mut ();   // C 300
  let mut lcid_ptr: *mut LCID;                        // C 301

  ONTRACE! (fprintf (stderr, c"render_locale\n".as_ptr ()));               // C 304

  if current_lcid == LOCALE_NEUTRAL || current_lcid == DEFAULT_LCID        // C 306
    { return Qt (); }

  hlocale = GlobalAlloc (GMEM_MOVEABLE | GMEM_DDESHARE, core::mem::size_of::<LCID> ());  // C 308
  if hlocale.is_null ()                                                   // C 309
    { return Qnil (); }

  lcid_ptr = GlobalLock (hlocale) as *mut LCID;                           // C 311
  if lcid_ptr.is_null ()                                                  // C 311
    {
      GlobalFree (hlocale);                                               // C 313
      return Qnil ();
    }

  *lcid_ptr = current_lcid;                                               // C 316
  GlobalUnlock (hlocale);                                                 // C 317

  if SetClipboardData (CF_LOCALE, hlocale).is_null ()                     // C 319
    {
      GlobalFree (hlocale);                                               // C 321
      return Qnil ();
    }

  Qt ()                                                                   // C 325
}

/* C 331-333: "At the end of the program, we want to ensure that our clipboard data survives us.
   This code will do that."  */
/// C 334-390: `render_all`.
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn render_all (ignore: Lisp_Object) -> Lisp_Object
{
  ONTRACE! (fprintf (stderr, c"render_all\n".as_ptr ()));                  // C 337

  /* C 339-341: "According to the docs we should not call OpenClipboard() here, but testing on W2K
     and working code in other projects shows that it is actually necessary."  */
  OpenClipboard (core::ptr::null_mut ());                                  // C 343

  /* C 345-349: "There is no useful means to report errors here, there are none expected anyway, and
     even if there were errors, they wouldn't do any harm.  So we just go ahead and do what has to
     be done without bothering with error handling."  */
  modifying_clipboard += 1;                                               // C 351
  EmptyClipboard ();                                                      // C 352
  modifying_clipboard -= 1;                                               // C 353

  /* C 355-381: "For text formats that we don't render here, the OS can use its own translation
     rules instead, so we don't really need to offer everything.  To minimize memory consumption we
     cover three possible situations based on our primary format as detected from
     selection-coding-system (see setup_config()):

       - Post CF_TEXT only.  Let the OS convert to CF_OEMTEXT and the OS (on NT) or the application
	 (on 9x/Me) convert to CF_UNICODETEXT.

       - Post CF_OEMTEXT only.  Similar automatic conversions happen as for CF_TEXT.

       - Post CF_UNICODETEXT + CF_TEXT.  9x itself ignores CF_UNICODETEXT, even though some
	 applications can still handle it.

	 Note 1: We render the less capable CF_TEXT *before* the more capable CF_UNICODETEXT, to
	 prevent clobbering through automatic conversions, just in case.

	 Note 2: We could check os_subtype here and only render the additional CF_TEXT on 9x/Me.  But
	 OTOH with current_clipboard_type == CF_UNICODETEXT we don't involve the automatic conversions
	 anywhere else, so to get consistent results, we probably don't want to rely on it here
	 either."  */

  render_locale ();                                                       // C 383

  if current_clipboard_type == CF_UNICODETEXT                             // C 385
    { render (make_fixnum (CF_TEXT as EMACS_INT)); }                      // C 386
  render (make_fixnum (current_clipboard_type as EMACS_INT));             // C 387

  CloseClipboard ();                                                      // C 389

  Qnil ()                                                                 // C 391
}

/// C 392-413: `run_protected`.  The function-pointer parameter is the mirror's
/// `Option<unsafe extern "C" fn …>` shape (PORTING.md 1.4), which is also what
/// `internal_condition_case_1` takes.
#[cfg (feature = "have_ntgui")]
unsafe fn run_protected (code: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
			 arg: Lisp_Object)
{
  /* C 395-397: "FIXME: This works but it doesn't feel right.  Too much fiddling with global
     variables and calling strange looking functions.  Is this really the right way to run Lisp
     callbacks?"  */
  let mut owfi: bool;                                        // C 399: `int owfi;`

  block_input ();                                            // C 401

  /* C 403-404: "Fsignal calls emacs_abort () if it sees that waiting_for_input is set."  */
  owfi = waiting_for_input! ();                              // C 405
  waiting_for_input! () = false;                             // C 406

  internal_condition_case_1 (code, arg, Qt (), Some (lisp_error_handler));   // C 408

  waiting_for_input! () = owfi;                              // C 410

  unblock_input ();                                          // C 412
}

/// C 415-422: `lisp_error_handler`.
#[cfg (feature = "have_ntgui")]
unsafe extern "C" fn lisp_error_handler (error: Lisp_Object) -> Lisp_Object
{
  Vsignaling_function = Qnil ();                             // C 418
  cmd_error_internal (error, c"Error in delayed clipboard rendering: ".as_ptr ());   // C 419
  Vinhibit_quit = Qt ();                                     // C 420
  Qt ()                                                      // C 421
}

/// C 425-460: `owner_callback`.  C's `LRESULT CALLBACK ALIGN_STACK` is the `extern "system"`
/// convention (the stand-in section) and the `ALIGN_STACK` attribute, which `include/w32term.rs`
/// records as having no Rust item - so no attribute stands here.  C's `switch` is the `match`
/// below; `WM_DESTROY`'s `break` is the empty arm, which falls through to the `DefWindowProc` call
/// C's `break` reaches (PORTING.md 2.2).
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn owner_callback (win: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT
{
  match msg                                                        // C 428
    {
      WM_RENDERFORMAT =>                                           // C 430
	{
	  ONTRACE! (fprintf (stderr, c"WM_RENDERFORMAT\n".as_ptr ()));      // C 431
	  run_protected (Some (render), make_fixnum (wp as EMACS_INT));     // C 432
	  return 0;
	}

      WM_RENDERALLFORMATS =>                                       // C 435
	{
	  ONTRACE! (fprintf (stderr, c"WM_RENDERALLFORMATS\n".as_ptr ()));  // C 436
	  run_protected (Some (render_all), Qnil ());                       // C 437
	  return 0;
	}

      WM_DESTROYCLIPBOARD =>                                       // C 440
	{
	  if modifying_clipboard == 0                              // C 441
	    {
	      ONTRACE! (fprintf (stderr, c"WM_DESTROYCLIPBOARD (other)\n".as_ptr ()));   // C 443
	      current_text = Qnil ();                              // C 444
	      current_coding_system = Qnil ();                     // C 445
	    }
	  else
	    {
	      ONTRACE! (fprintf (stderr, c"WM_DESTROYCLIPBOARD (self)\n".as_ptr ()));    // C 449
	    }
	  return 0;                                                // C 451
	}

      WM_DESTROY =>                                                // C 454
	{
	  if win == clipboard_owner                                // C 455
	    { clipboard_owner = core::ptr::null_mut (); }          // C 456
	  /* C 457: `break;` - falls through to the call below.  */
	}

      _ => {}                                                      // C 428's `default`
    }

  DefWindowProc (win, msg, wp, lp)                                 // C 459
}

/// C 462-475: `create_owner`.
#[cfg (feature = "have_ntgui")]
unsafe fn create_owner () -> HWND
{
  let CLASSNAME: &core::ffi::CStr = c"Emacs Clipboard";   // C 465: `static const char CLASSNAME[]`
  let mut wc: WNDCLASS = core::mem::zeroed ();            // C 466 (zero-initialised, PORTING.md 1.5)

  memset (&raw mut wc as *mut c_void, 0, core::mem::size_of::<WNDCLASS> ());   // C 468
  wc.lpszClassName = CLASSNAME.as_ptr ();                                      // C 469
  wc.lpfnWndProc = Some (owner_callback);                                      // C 470
  RegisterClass (&raw const wc);                                               // C 471

  CreateWindow (CLASSNAME.as_ptr (), CLASSNAME.as_ptr (), 0, 0, 0, 0, 0,
		core::ptr::null_mut (), core::ptr::null_mut (),
		core::ptr::null_mut (), core::ptr::null_mut ())            // C 473-474
}

/* C 477: "Called on exit by term_ntproc() in w32.c" */
/// C 479-488: `term_w32select`.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn term_w32select ()
{
  /* C 481: "This is needed to trigger WM_RENDERALLFORMATS."  */
  if !clipboard_owner.is_null ()                        // C 482
    {
      DestroyWindow (clipboard_owner);                  // C 484
      clipboard_owner = core::ptr::null_mut ();         // C 485
    }
}

/// C 490-568: `setup_config`.
#[cfg (feature = "have_ntgui")]
unsafe fn setup_config ()
{
  let mut coding_name: *const c_char;      // C 492
  let mut cp: *const c_char;               // C 493
  let mut end: *mut c_char;                // C 494
  let mut slen: c_int;                     // C 495
  let mut coding_system: Lisp_Object;      // C 496
  let mut dos_coding_system: Lisp_Object;  // C 497

  CHECK_SYMBOL (Vselection_coding_system);                    // C 499

  coding_system = if NILP (Vnext_selection_coding_system)     // C 501-502
    { Vselection_coding_system } else { Vnext_selection_coding_system };

  dos_coding_system = validate_coding_system (coding_system); // C 504
  if NILP (dos_coding_system)                                // C 505
    { Fsignal (Qerror (), list2 (build_string (c"Coding system is invalid or doesn't have an eol variant for dos line ends".as_ptr ()),
			    coding_system)); }           // C 506-510

  /* C 512: "Check if we have it cached" */
  if !NILP (cfg_coding_system)                               // C 513
      && EQ (cfg_coding_system, dos_coding_system)
    { return; }                                              // C 515
  cfg_coding_system = dos_coding_system;                     // C 516

  /* C 518: "Set some sensible fallbacks" */
  cfg_codepage = ANSICP;                                     // C 519
  cfg_lcid = LOCALE_NEUTRAL;                                 // C 520
  cfg_clipboard_type = CF_TEXT;                              // C 521

  /* C 523: "Interpret the coding system symbol name" */
  coding_name = SSDATA (SYMBOL_NAME (cfg_coding_system));    // C 524

  /* C 526: `"(.*-)?utf-16.*" -> CF_UNICODETEXT` */
  cp = strstr (coding_name, c"utf-16".as_ptr ());            // C 527
  if !cp.is_null () && (cp == coding_name || *cp.offset (-1) == b'-' as c_char)   // C 528
    {
      cfg_clipboard_type = CF_UNICODETEXT;                   // C 530
      return;
    }

  /* C 533: `"cp[0-9]+.*" or "windows-[0-9]+.*" -> CF_TEXT or CF_OEMTEXT` */
  slen = strlen (coding_name) as c_int;                      // C 534
  if slen >= 4 && *coding_name == b'c' as c_char
      && *coding_name.offset (1) == b'p' as c_char           // C 535
    { cp = coding_name.offset (2); }                         // C 536
  else if slen >= 10 && memcmp (coding_name as *const c_void, c"windows-".as_ptr () as *const c_void,
				8) == 0                     // C 537
    { cp = coding_name.offset (8); }                         // C 538
  else
    { return; }                                              // C 539

  end = cp as *mut c_char;                                   // C 541
  cfg_codepage = strtol (cp, &raw mut end, 10) as UINT;      // C 542

  /* C 544-545: "Error return from strtol() or number of digits < 2 -> Restore the default and drop
     it."  */
  if cfg_codepage == 0 || (end as isize - cp as isize) < 2   // C 546
    {
      cfg_codepage = ANSICP;                                 // C 548
      return;
    }

  /* C 551: "Is it the currently active system default?"  */
  if cfg_codepage == ANSICP                                  // C 552
    {
      /* C 554: `cfg_clipboard_type = CF_TEXT;` */
      return;
    }
  if cfg_codepage == OEMCP                                   // C 556
    {
      cfg_clipboard_type = CF_OEMTEXT;                       // C 558
      return;
    }

  /* C 561: "Else determine a suitable locale the hard way."  */
  EnumSystemLocales (Some (enum_locale_callback), LCID_INSTALLED);      // C 562
}

/// C 570-597: `enum_locale_callback`.
#[cfg (feature = "have_ntgui")]
unsafe extern "system" fn enum_locale_callback (loc_string: *mut c_char) -> BOOL
{
  let mut lcid: LCID;                                // C 572
  let mut codepage: UINT;                            // C 573

  lcid = strtoul (loc_string, core::ptr::null_mut (), 16) as LCID;      // C 575

  /* C 577: "Is the wanted codepage the "ANSI" codepage for this locale?"  */
  codepage = cp_from_locale (lcid, CF_TEXT);                            // C 578
  if codepage == cfg_codepage                                           // C 579
    {
      cfg_lcid = lcid;                                                  // C 581
      cfg_clipboard_type = CF_TEXT;                                     // C 582
      return FALSE; /* C 583: "Stop enumeration" */
    }

  /* C 586: "Is the wanted codepage the OEM codepage for this locale?"  */
  codepage = cp_from_locale (lcid, CF_OEMTEXT);                         // C 587
  if codepage == cfg_codepage                                           // C 588
    {
      cfg_lcid = lcid;                                                  // C 590
      cfg_clipboard_type = CF_OEMTEXT;                                  // C 591
      return FALSE; /* C 592: "Stop enumeration" */
    }

  return TRUE; /* C 595: "Continue enumeration" */
}

/// C 599-617: `cp_from_locale`.
#[cfg (feature = "have_ntgui")]
unsafe fn cp_from_locale (lcid: LCID, format: UINT) -> UINT
{
  let mut buffer: [c_char; 20] = [0; 20];            // C 601: `char buffer[20] = "";`
  let mut variant: UINT;                             // C 602
  let mut cp: UINT;                                  // C 602

  variant = if format == CF_TEXT                     // C 604-605
    { LOCALE_IDEFAULTANSICODEPAGE } else { LOCALE_IDEFAULTCODEPAGE };

  GetLocaleInfo (lcid, variant, buffer.as_mut_ptr (), buffer.len () as c_int);   // C 607
  cp = strtoul (buffer.as_ptr (), core::ptr::null_mut (), 10) as UINT;           // C 608

  if cp == CP_ACP                                    // C 610
    { return ANSICP; }
  else if cp == CP_OEMCP                             // C 612
    { return OEMCP; }
  else
    { return cp; }                                   // C 615
}

/// C 619-628: `coding_from_cp`.
#[cfg (feature = "have_ntgui")]
unsafe fn coding_from_cp (codepage: UINT) -> Lisp_Object
{
  let mut buffer: [c_char; 30] = [0; 30];            // C 621 (zero-initialised, PORTING.md 1.5)
  sprintf (buffer.as_mut_ptr (), c"cp%d-dos".as_ptr (), codepage as c_int);      // C 622
  intern (buffer.as_ptr ())                          // C 623
  /* C 624-627: "We don't need to check that this coding system actually exists right here, because
     that is done later for all coding systems used, regardless of where they originate."  */
}

/// C 630-656: `validate_coding_system`.
#[cfg (feature = "have_ntgui")]
unsafe fn validate_coding_system (coding_system: Lisp_Object) -> Lisp_Object
{
  let mut eol_type: Lisp_Object;                     // C 632

  /* C 634: "Make sure the input is valid."  */
  if NILP (Fcoding_system_p (coding_system))         // C 635
    { return Qnil (); }

  /* C 637-638: "Make sure we use a DOS coding system as mandated by the system specs."  */
  eol_type = Fcoding_system_eol_type (coding_system);      // C 639

  /* C 641: "Already a DOS coding system?"  */
  if BASE_EQ (eol_type, make_fixnum (1))             // C 642
    { return coding_system; }

  /* C 644: "Get EOL_TYPE vector of the base of CODING_SYSTEM."  */
  if !VECTORP (eol_type)                             // C 645
    {
      eol_type = Fcoding_system_eol_type (Fcoding_system_base (coding_system));   // C 647
      if !VECTORP (eol_type)                         // C 648
	{ return Qnil (); }
    }

  AREF (eol_type, 1)                                 // C 651
}

/// C 658-677: `setup_windows_coding_system`.
#[cfg (feature = "have_ntgui")]
unsafe fn setup_windows_coding_system (coding_system: Lisp_Object, coding: *mut coding_system)
{
  memset (coding as *mut c_void, 0, core::mem::size_of::<coding_system> ());   // C 662
  setup_coding_system (coding_system, coding);                                 // C 663

  /* C 665-675: "Unset CODING_ANNOTATE_COMPOSITION_MASK.  Previous code had comments about crashes
     in encode_coding_iso2022 trying to dereference a null pointer when composition was on.
     Selection data should not contain any composition sequence on Windows.

     CODING_ANNOTATION_MASK also includes CODING_ANNOTATE_DIRECTION_MASK and
     CODING_ANNOTATE_CHARSET_MASK, which both apply to ISO6429 only.  We don't know if these really
     need to be unset on Windows, but it probably doesn't hurt either."  */
  /* `coding->common_flags &= ~CODING_ANNOTATION_MASK;` - the two bitfields share the mirror's
     `bits: u32` storage unit (`include/coding.rs` 244-268 spells the shifts), so C's two writers are
     the two masked writes below.  */
  (*coding).bits = ((*coding).bits & CODING_SYSTEM_COMMON_FLAGS_MASK)
                   & !(CODING_ANNOTATION_MASK << CODING_SYSTEM_COMMON_FLAGS_SHIFT);   // C 676
  (*coding).bits |= ((CODING_MODE_LAST_BLOCK | CODING_MODE_SAFE_ENCODING) as u32)
                    << CODING_SYSTEM_MODE_SHIFT;                                      // C 677
}

/* C 679-681: "This sets the clipboard data to the given text."  */
/* C 683-798: `Fw32_set_clipboard_data`.  */
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-set-clipboard-data", Fw32_set_clipboard_data,
       Sw32_set_clipboard_data, 1, 2, 0,
       doc: "This sets the clipboard data to the given text.",
       (string: Lisp_Object, ignored: Lisp_Object),
       {
  let mut ok: BOOL = TRUE;                        // C 685
  let mut nbytes: c_int;                          // C 686
  let mut src: *mut u8;                           // C 687
  let mut dst: *mut u8;                           // C 688
  let mut end: *mut u8;                           // C 689

  /* C 691-692: "This parameter used to be the current frame, but we don't use that any more."  */
  let _ = ignored;                                // C 693: `(void) ignored;`

  CHECK_STRING (string);                          // C 695

  setup_config ();                                // C 697

  current_text = string;                          // C 699
  current_coding_system = cfg_coding_system;      // C 700
  current_clipboard_type = cfg_clipboard_type;    // C 701
  current_lcid = cfg_lcid;                        // C 702
  current_num_nls = 0;                            // C 703
  current_requires_encoding = 0;                  // C 704

  block_input ();                                 // C 706

  /* C 708-711: "Check for non-ASCII characters.  While we are at it, count the number of LFs, so we
     know how many CRs we will have to add later (just in the case where we can use our internal
     ASCII rendering, see code and comment in convert_to_handle_as_ascii() above)."  */
  nbytes = SBYTES (string) as c_int;              // C 712
  src = SDATA (string);                           // C 713

  dst = src;                                      // C 715 (the `for` initialiser, with `end`)
  end = src.offset (nbytes as isize);             // C 715
  while dst < end                                 // C 715
    {
      if *dst == b'\n'                            // C 717
	{ current_num_nls += 1; }                 // C 718
      else if *dst >= 0x80 || *dst == 0           // C 719
	{
	  current_requires_encoding = 1;          // C 721
	  break;                                  // C 723
	}
      dst = dst.offset (1);                       // C 715's increment
    }

  if current_requires_encoding == 0               // C 726
    {
      /* C 728-729: "If all we have is ASCII we don't need to pretend we offer anything fancy."  */
      current_coding_system = Qraw_text ();       // C 730
      current_clipboard_type = CF_TEXT;           // C 731
      current_lcid = LOCALE_NEUTRAL;              // C 732
    }

  /* C's `goto error` (C 738) and `goto done` (C 775) are this labelled block (PORTING.md 2.2): the
     error block sits at its end, so the `!ok` fall-through reaches it as C's does, and `break` is
     `goto done`.  */
  'flow: {
    if OpenClipboard (clipboard_owner) == 0       // C 736
      { /* C 738: `goto error` */ }
    else
      {
	modifying_clipboard += 1;                 // C 740
	ok = EmptyClipboard ();                   // C 741
	modifying_clipboard -= 1;                 // C 742

	/* C 744-745: "If we have something non-ASCII we may want to set a locale.  We do that directly
	   (non-delayed), as it's just a small bit."  */
	if ok != 0                                // C 746
	  { ok = (!NILP (render_locale ())) as BOOL; }                  // C 747

	if ok != 0                                // C 749
	  {
	    if clipboard_owner.is_null ()         // C 751
	      {
		/* C 752-754: "If for some reason we don't have a clipboard_owner, we just set the text
		   format as chosen by the configuration and than forget about the whole thing."  */
		ok = (!NILP (render (make_fixnum (current_clipboard_type as EMACS_INT)))) as BOOL;
		current_text = Qnil ();                       // C 756
		current_coding_system = Qnil ();              // C 757
	      }
	    else
	      {
		/* C 760-764: "Advertise all supported formats so that whatever the requestor chooses,
		   only one encoding step needs to be made.  This is intentionally different from what
		   we do in the handler for WM_RENDERALLFORMATS."  */
		SetClipboardData (CF_UNICODETEXT, core::ptr::null_mut ());   // C 765
		SetClipboardData (CF_TEXT, core::ptr::null_mut ());          // C 766
		SetClipboardData (CF_OEMTEXT, core::ptr::null_mut ());       // C 767
	      }
	  }

	CloseClipboard ();                              // C 771

	/* C 773-778: "With delayed rendering we haven't really "used" this coding system yet, and it's
	   even unclear if we ever will.  But this is a way to tell the upper level what we *would* use
	   under ideal circumstances.

	   We don't signal the actually used coding-system later when we finally render, because that
	   can happen at any time and we don't want to disturb the "foreground" action."  */
	if ok != 0                                      // C 774
	  { Vlast_coding_system_used = current_coding_system; }         // C 775

	Vnext_selection_coding_system = Qnil ();        // C 777

	if ok != 0                                      // C 779
	  { break 'flow; }                              // C 779: `goto done`
      }

    /* C 781: `error:` */
    ok = FALSE;                                     // C 783
    current_text = Qnil ();                         // C 784
    current_coding_system = Qnil ();                // C 785
  }

  /* C 787: `done:` */
  unblock_input ();                                 // C 788

  return if ok != 0 { string } else { Qnil () };    // C 790
});

/* C 800-802: "Xlib-like names for standard Windows clipboard data formats.  They are in upper-case
   to mimic xselect.c.  A couple of the names were changed to be more like their X counterparts."  */
/// C 803-822: `stdfmt_name`.  C's `const char *[]` keeps C's own strings; the port's spelling of a
/// C string literal is `c"..."`, so the array's element type is `&CStr` and a use site takes
/// `.as_ptr ()` (the same bytes C's initialiser has).
#[cfg (feature = "have_ntgui")]
static stdfmt_name: [&core::ffi::CStr; 18] =
[
  c"UNDEFINED", c"STRING", c"BITMAP", c"METAFILE", c"SYMLINK", c"DIF", c"TIFF", c"OEM_STRING",
  c"DIB", c"PALETTE", c"PENDATA", c"RIFF", c"WAVE", c"UTF8_STRING", c"ENHMETAFILE",
  c"FILE_NAMES", /* DND */
  c"LOCALE", /* not used */
  c"DIBV5",
];

/// C 824: "Must be called with block_input() active."  ;;
/// C 825-865: `convert_dibv5_to_png`.  C's `#ifdef WINDOWSNT` arm is the file gate itself (the
/// file only ever exists on that build), and C's `#else` arm - the one-item `return false` - is
/// carried under the negated gate.  C's `size` parameter is unused in the body, as C's own text
/// shows.
#[cfg (feature = "have_ntgui")]
unsafe fn convert_dibv5_to_png (data: *mut c_char, size: c_int, temp_file: *mut c_char) -> bool
{
  #[cfg (feature = "windowsnt")]
  {
    let mut clsid_png: CLSID = core::mem::zeroed ();      // C 829 (zero-initialised, PORTING.md 1.5)

    if !w32_gdiplus_startup ()
	|| w32_gdip_get_encoder_clsid (c"png".as_ptr (), &raw mut clsid_png as *mut c_void) == 0
      { return false; }                                   // C 831-833

    let bmi: *mut BITMAPV5HEADER = data as *mut BITMAPV5HEADER;      // C 835
    /* C 836: `bmi->bV5SizeImage / bmi->bV5Height` - C's usual arithmetic conversions make the
       `LONG` operand unsigned, so the port spells the conversion.  */
    let stride: c_int =
      ((*bmi).bV5SizeImage / ((*bmi).bV5Height as DWORD)) as c_int;   // C 836
    let mut offset: c_long =
      (*bmi).bV5Size as c_long + (*bmi).bV5ClrUsed as c_long
      * core::mem::size_of::<RGBQUAD> () as c_long;                  // C 837
    if (*bmi).bV5Compression == BI_BITFIELDS                         // C 838
      { offset += 12; }                                              // C 839
    let scan0: *mut BYTE = (data as *mut u8).offset (offset as isize) as *mut BYTE;   // C 840

    let mut bitmap: *mut GpBitmap = core::ptr::null_mut ();          // C 842

    let status: GpStatus                                              // C 844-846
      = (GdipCreateBitmapFromScan0! ().unwrap ()) ((*bmi).bV5Width, (*bmi).bV5Height, stride,
						   PixelFormat32bppARGB, scan0,
						   &raw mut bitmap);

    if status != Ok                                                   // C 848
      { return false; }

    /* C 850: "The bitmap comes upside down."  */
    (GdipImageRotateFlip! ().unwrap ()) (bitmap, RotateNoneFlipY);    // C 852

    let mut wide_filename: [WCHAR; MAX_PATH] = [0; MAX_PATH];         // C 854
    filename_to_utf16 (temp_file, wide_filename.as_mut_ptr ());       // C 856

    let status: GpStatus                                              // C 857-858
      = (GdipSaveImageToFile! ().unwrap ()) (bitmap, wide_filename.as_ptr (),
					     &raw const clsid_png, core::ptr::null ());
    (GdipDisposeImage! ().unwrap ()) (bitmap);                        // C 858
    if status != Ok                                                   // C 859
      { return false; }
    return true;                                                      // C 861
  }
  #[cfg (not (feature = "windowsnt"))]
  {
    /* C 862-864: `#else /* !WINDOWSNT */` ... `return false;` */
    let _ = (data, size, temp_file);
    return false;
  }
}

/// C 867-878: `get_clipboard_format_name`.
#[cfg (feature = "have_ntgui")]
unsafe fn get_clipboard_format_name (mut format_index: c_int, name: *mut c_char) -> c_int
{
  *name = 0;                                                             // C 870
  format_index = EnumClipboardFormats (format_index as UINT) as c_int;   // C 871
  if format_index == 0                                                   // C 872
    { return 0; }
  if format_index < CF_MAX as c_int                                      // C 874
    { strcpy (name, stdfmt_name[format_index as usize].as_ptr ()); }
  GetClipboardFormatName (format_index as UINT, name, 256);              // C 875
  return format_index;                                                   // C 876
}

/* C 880-963: `Fw32__get_clipboard_data_media`.  */
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32--get-clipboard-data-media", Fw32__get_clipboard_data_media,
       Sw32__get_clipboard_data_media, 3, 3, 0,
       doc: "Gets media (not plain text) clipboard data in one of the given formats.\n\nFORMATS is a list of formats.\nTEMP-FILE-IN is the name of the file to store the data.\n\nElements in FORMATS are symbols naming a format, such a image/png, or\nimage/jpeg.  For compatibility with X systems, some conventional\nformat names are translated to equivalent MIME types, as configured with\nthe variable 'w32--selection-target-translations'.\n\nThe file named in TEMP-FILE-IN must be created by the caller, and also\ndeleted if required.\n\nReturns nil it there is no such format, or something failed.\nIf it returns t, then the caller should read the file to get the data.\nIf it returns a string, then that is the data and the file is not used.\n\nWhen returning a string, it will be unibyte if IS-TEXTUAL is nil (the\ncontent is binary data).",
       (formats: Lisp_Object, temp_file_in: Lisp_Object, is_textual: Lisp_Object),
       {
  CHECK_LIST (formats);                          // C 904
  CHECK_STRING (temp_file_in);                   // C 905

  let mut temp_file_in = temp_file_in;           // C 907 reassigns the parameter
  temp_file_in = Fexpand_file_name (temp_file_in, Qnil ());           // C 907
  let temp_file: *mut c_char = SSDATA (ENCODE_FILE (temp_file_in));   // C 908

  let mut result: Lisp_Object = Qnil ();         // C 910

  block_input ();                                // C 912
  if OpenClipboard (core::ptr::null_mut ()) == 0 // C 913
    {
      unblock_input ();                          // C 915
      return Qnil ();
    }

  let mut format_index: c_int = 0;               // C 918
  loop
    {
      /* C 920 is `static char name[256];`: the function-static C declares (the buffer the
	 `get_clipboard_format_name` call below fills).  */
      static mut name: [c_char; 256] = [0; 256];  // C 921
      format_index = get_clipboard_format_name (format_index, name.as_mut_ptr ());   // C 922
      if format_index == 0                       // C 923
	{ break; }                               // C 924

      /* C 926: "If name doesn't match any of the formats, try the next format."  */
      let mut match_: bool = false;              // C 927
      let mut tail: Lisp_Object = formats;       // C 928
      while CONSP (tail)                         // C 928
	{
	  if strcmp (name.as_ptr (), SSDATA (SYMBOL_NAME (XCAR (tail)))) == 0    // C 929
	    { match_ = true; }
	  tail = XCDR (tail);                      // C 928's increment
	}
      if !match_                                 // C 930
	{ continue; }                            // C 931

      /* C 933: "Of the standard formats, only DIBV5 is supported."  */
      if format_index < CF_MAX as c_int && format_index != CF_DIBV5 as c_int    // C 934
	{ continue; }                            // C 935

      /* C 937: "Found the format."  */
      let d: HANDLE = GetClipboardData (format_index as UINT);                  // C 938
      if d.is_null ()                           // C 939
	{ break; }
      let size: c_int = GlobalSize (d) as c_int;                                // C 940
      let data: *mut c_char = GlobalLock (d) as *mut c_char;                    // C 941
      if data.is_null ()                        // C 942
	{ break; }
      if strcmp (name.as_ptr (), c"DIBV5".as_ptr ()) == 0                       // C 943
	{
	  if convert_dibv5_to_png (data, size, temp_file)                       // C 944-946
	    { result = Qt (); }                                                 // C 947-948
	}
      else
	{
	  if NILP (is_textual)                                                  // C 952-953
	    { result = make_unibyte_string (data, size as ptrdiff_t); }          // C 954
	  else
	    { result = make_string (data, size as ptrdiff_t); }                  // C 956
	}
      GlobalUnlock (d);                         // C 958
      break;                                    // C 959
    }
  CloseClipboard ();                            // C 961
  unblock_input ();                             // C 962
  return result;                                // C 963
});

/* C 965-1180: `Fw32_get_clipboard_data`.  */
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-get-clipboard-data", Fw32_get_clipboard_data,
       Sw32_get_clipboard_data, 0, 1, 0,
       doc: "This gets the clipboard data in text format.",
       (ignored: Lisp_Object),
       {
  let mut htext: HGLOBAL;                                    // C 971
  let mut ret: Lisp_Object = Qnil ();                        // C 972
  let mut actual_clipboard_type: UINT;                       // C 973
  let mut use_configured_coding_system: c_int = 1;           // C 974

  /* C 976-977: "This parameter used to be the current frame, but we don't use that any more."  */
  let _ = ignored;                                           // C 978: `(void) ignored;`

  /* C 980-981: "Don't pass our own text from the clipboard (which might be troublesome if the
     killed text includes null characters)."  */
  if !NILP (current_text)                                    // C 982
    { return ret; }

  setup_config ();                                           // C 984
  actual_clipboard_type = cfg_clipboard_type;                // C 985

  block_input ();                                            // C 987

  'done: {
    if OpenClipboard (clipboard_owner) == 0                  // C 989
      { break 'done; }                                       // C 990: `goto done`

    'closeclip: {
      htext = GetClipboardData (actual_clipboard_type);      // C 992
      if htext.is_null ()                                    // C 992
	{
	  /* C 994-997: "If we want CF_UNICODETEXT but can't get it, the current coding system is
	     useless.  OTOH we can still try and decode CF_TEXT based on the locale that the system
	     gives us and that we get down below."  */
	  if actual_clipboard_type == CF_UNICODETEXT             // C 998
	    {
	      htext = GetClipboardData (CF_TEXT);                // C 1000
	      if !htext.is_null ()                               // C 1001
		{
		  actual_clipboard_type = CF_TEXT;               // C 1003
		  use_configured_coding_system = 0;              // C 1004
		}
	    }
	}
      if htext.is_null ()                                    // C 1008
	{ break 'closeclip; }                                // C 1009: `goto closeclip`

      {
	let mut src: *mut u8;                                // C 1012
	let mut dst: *mut u8;                                // C 1013
	let mut nbytes: c_int;                               // C 1014
	let mut truelen: c_int;                              // C 1015
	let mut require_decoding: c_int = 0;                 // C 1016

	src = GlobalLock (htext) as *mut u8;                 // C 1018
	if src.is_null ()                                    // C 1018
	  { break 'closeclip; }                              // C 1019: `goto closeclip`

	/* C 1021-1022: "If the clipboard data contains any non-ascii code, we need to decode it with
	   a coding system."  */
	if actual_clipboard_type == CF_UNICODETEXT           // C 1023
	  {
	    nbytes = lstrlenW (src as *const WCHAR) * 2;     // C 1025
	    require_decoding = 1;                            // C 1026
	  }
	else
	  {
	    let mut i: c_int;                                // C 1030

	    nbytes = strlen (src as *const c_char) as c_int; // C 1032

	    i = 0;                                           // C 1034
	    while i < nbytes                                // C 1034
	      {
		if *src.offset (i as isize) >= 0x80          // C 1036
		  {
		    require_decoding = 1;                    // C 1038
		    break;                                   // C 1039
		  }
		i += 1;                                      // C 1034's increment
	      }
	  }

	if require_decoding != 0                             // C 1043
	  {
	    let mut coding: coding_system = core::mem::zeroed ();   // C 1046 (zero-init, PORTING.md 1.5)
	    let mut coding_system: Lisp_Object = Qnil ();           // C 1047
	    let mut dos_coding_system: Lisp_Object;                 // C 1048

	    /* C 1050-1053: "`next-selection-coding-system' should override everything, even when the
	       locale passed by the system disagrees.  The only exception is when
	       `next-selection-coding-system' requested CF_UNICODETEXT and we couldn't get that."  */
	    if use_configured_coding_system != 0
		&& !NILP (Vnext_selection_coding_system)         // C 1054-1055
	    { coding_system = Vnext_selection_coding_system; }   // C 1056

	    /* C 1058-1059: "If we have CF_TEXT or CF_OEMTEXT, we want to check out CF_LOCALE, too."  */
	    else if actual_clipboard_type != CF_UNICODETEXT      // C 1060
	      {
		let hlocale: HGLOBAL;                            // C 1062
		let mut lcid: LCID = DEFAULT_LCID;               // C 1063
		let mut cp: UINT;                                // C 1064

		/* C 1066-1069: "Documentation says that the OS always generates CF_LOCALE info
		   automatically, so the locale handle should always be present.  Fact is that this is
		   not always true on 9x ;-(."  */
		hlocale = GetClipboardData (CF_LOCALE);          // C 1070
		if !hlocale.is_null ()                           // C 1071
		  {
		    let lcid_ptr: *const LCID;                   // C 1073
		    lcid_ptr = GlobalLock (hlocale) as *const LCID;   // C 1074
		    if !lcid_ptr.is_null ()                      // C 1075
		      {
			lcid = *lcid_ptr;                        // C 1077
			GlobalUnlock (hlocale);                  // C 1078
		      }

		    /* C 1081-1085: "9x has garbage as the sort order (to be exact there is another
		       instance of the language id in the upper word).  We don't care about sort order
		       anyway, so we just filter out the unneeded mis-information to avoid
		       irritations."  */
		    lcid = MAKELCID! (LANGIDFROMLCID! (lcid), SORT_DEFAULT);       // C 1086
		  }

		/* C 1089-1092: "If we are using fallback from CF_UNICODETEXT, we can't use the
		   configured coding system.  Also we don't want to use it, if the system has supplied
		   us with a locale and it is not just the system default."  */
		if use_configured_coding_system == 0 || lcid != DEFAULT_LCID   // C 1093
		  {
		    cp = cp_from_locale (lcid, actual_clipboard_type);         // C 1095
		    /* C 1096-1100: "If it's just our current standard setting anyway, use the coding
		       system that the user has selected.  Otherwise create a new spec to match the
		       locale that was specified by the other side or the system."  */
		    if use_configured_coding_system == 0 || cp != cfg_codepage  // C 1101
		      { coding_system = coding_from_cp (cp); }                 // C 1102
		  }
	      }

	    if NILP (coding_system)                                   // C 1106
	      { coding_system = Vselection_coding_system; }            // C 1107
	    Vnext_selection_coding_system = Qnil ();                   // C 1108

	    dos_coding_system = validate_coding_system (coding_system);      // C 1110
	    if !NILP (dos_coding_system)                              // C 1111
	      {
		setup_windows_coding_system (dos_coding_system, &raw mut coding);   // C 1113
		coding.source = src as *const u8;                     // C 1114
		decode_coding_object (&raw mut coding, Qnil (), 0, 0, nbytes as ptrdiff_t,
				      nbytes as ptrdiff_t, Qt ());        // C 1115
		ret = coding.dst_object;                              // C 1116

		Vlast_coding_system_used = CODING_ID_NAME (coding.id as hash_idx_t);   // C 1118
	      }
	  }
	else
	  {
	    /* C 1123-1124: "FIXME: We may want to repeat the code in this branch for the Unicode
	       case."  */

	    /* C 1126-1129: "Need to know final size after CR chars are removed because we can't
	       change the string size manually, and doing an extra copy is silly.  We only remove CR
	       when it appears as part of CRLF."  */

	    truelen = nbytes;                                        // C 1131
	    dst = src;                                               // C 1132
	    /* C 1133: "avoid using strchr because it recomputes the length every time" - C's
	       assignment-in-condition is the `loop` below.  */
	    loop
	      {
		dst = memchr (dst as *const c_void, b'\r' as c_int,
			      (nbytes as isize - (dst as isize - src as isize)) as SIZE_T)
		      as *mut u8;                                    // C 1134
		if dst.is_null () { break; }                         // C 1134's condition
		if *dst.offset (1) == b'\n'                          // C 1136 "safe because of trailing '\0'"
		  { truelen -= 1; }
		dst = dst.offset (1);                                // C 1138
	      }

	    ret = make_uninit_string (truelen as EMACS_INT);         // C 1141

	    /* C 1143-1144: "Convert CRLF line endings (the standard CF_TEXT clipboard format) to LF
	       endings as used internally by Emacs."  */

	    dst = SDATA (ret);                                      // C 1146
	    loop                                                    // C 1147
	      {
		let next: *mut u8;                                  // C 1149
		/* C 1150: "copy next line or remaining bytes excluding '\0'"  */
		next = _memccpy (dst as *mut c_void, src as *const c_void, b'\r' as c_int,
				 nbytes as SIZE_T) as *mut u8;      // C 1151
		if !next.is_null ()                                 // C 1152
		  {
		    /* C 1154: "copied one line ending with '\r'"  */
		    let copied: c_int = (next as isize - dst as isize) as c_int;   // C 1155
		    nbytes -= copied;                               // C 1156
		    dst = dst.offset (copied as isize);             // C 1157
		    src = src.offset (copied as isize);             // C 1158
		    if *src == b'\n'                                // C 1159
		      { dst = dst.offset (-1); }                    // C 1160 "overwrite '\r' with '\n'"
		  }
		else
		  /* C 1163: "copied remaining partial line -> now finished"  */
		  { break; }                                        // C 1164
	      }

	    Vlast_coding_system_used = Qraw_text ();                // C 1167
	  }

	GlobalUnlock (htext);                                       // C 1178
      }

      /* C 1181: `closeclip:` */
    }

    CloseClipboard ();                                          // C 1182
  }

  /* C 1184: `done:` */
  unblock_input ();                                             // C 1186

  ret                                                           // C 1188
});

/* C 1182: "Support checking for a clipboard selection."  */
/* C 1184-1226: `Fw32_selection_exists_p`.  */
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-selection-exists-p", Fw32_selection_exists_p,
       Sw32_selection_exists_p, 0, 2, 0,
       doc: "Whether there is an owner for the given X selection.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.  (X expects\nthese literal upper-case names.)  The symbol nil is the same as\n`PRIMARY', and t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.",
       (selection: Lisp_Object, terminal: Lisp_Object),
       {
  CHECK_SYMBOL (selection);                                   // C 1199

  /* C 1201-1202: "Return nil for PRIMARY and SECONDARY selections; for CLIPBOARD, check if the
     clipboard currently has valid text format contents."  */

  if EQ (selection, QCLIPBOARD ())                            // C 1204
    {
      let mut val: Lisp_Object = Qnil ();                     // C 1206

      setup_config ();                                        // C 1208

      if OpenClipboard (core::ptr::null_mut ()) != 0          // C 1210
	{
	  let mut format: UINT = 0;                             // C 1211
	  loop
	    {
	      format = EnumClipboardFormats (format);           // C 1211
	      if format == 0 { break; }                         // C 1211's condition
	      /* C 1212-1215: "Check CF_TEXT in addition to cfg_clipboard_type, because we can fall
		 back on that if CF_UNICODETEXT is not available.  Actually a check for CF_TEXT only
		 should be enough."  */
	      if format == cfg_clipboard_type || format == CF_TEXT      // C 1216
		{
		  val = Qt ();                                    // C 1218
		  break;                                          // C 1219
		}
	    }
	  CloseClipboard ();                                      // C 1221
	}
      return val;                                             // C 1223
    }
  return Qnil ();                                             // C 1225
});

/* C 1228: "Support enumerating available clipboard selection formats."  */
/* C 1230-1310: `Fw32_selection_targets`.  */
#[cfg (feature = "have_ntgui")]
DEFUN! ("w32-selection-targets", Fw32_selection_targets,
       Sw32_selection_targets, 0, 2, 0,
       doc: "Return a vector of data formats available in the specified SELECTION.\nSELECTION should be the name of the selection in question, typically\none of the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.\nThe symbol nil is the same as `PRIMARY', and t is the same as `SECONDARY'.\n\nTERMINAL should be a terminal object or a frame specifying the X\nserver to query.  If omitted or nil, that stands for the selected\nframe's display, or the first available X display.\n\nThis function currently ignores TERMINAL, and only returns non-nil\nfor `CLIPBOARD'.  The return value is a vector of symbols, each symbol\nrepresenting a data format that is currently available in the clipboard.",
       (selection: Lisp_Object, terminal: Lisp_Object),
       {
  CHECK_SYMBOL (selection);                                   // C 1247

  /* C 1249-1250: "Return nil for PRIMARY and SECONDARY selections; for CLIPBOARD, check if the
     clipboard currently has valid text format contents."  */

  if EQ (selection, QCLIPBOARD ())                            // C 1252
    {
      let mut val: Lisp_Object = Qnil ();                     // C 1254

      setup_config ();                                        // C 1256

      if OpenClipboard (core::ptr::null_mut ()) != 0          // C 1258
	{
	  let mut format: UINT = 0;                             // C 1259

	  /* C 1261-1265: "Count how many formats are available.  We ignore the CF_LOCALE format, and
	     don't put it into the vector we return, because CF_LOCALE is automatically created by
	     Windows for any text in the clipboard, so its presence in the value will simply
	     confuse."  */
	  let mut fmtcount: c_int = 0;                          // C 1266
	  loop
	    {
	      format = EnumClipboardFormats (format);           // C 1267
	      if format == 0 { break; }                         // C 1267's condition
	      if format != CF_LOCALE                            // C 1268
		{ fmtcount += 1; }                              // C 1269
	    }

	  if fmtcount > 0                                       // C 1271
	    {
	      let mut i: c_int;                                 // C 1273

	      /* C 1275-1276: "We generate a vector because that's what xselect.c does in this
		 case."  */
	      val = Fmake_vector (make_fixnum (fmtcount as EMACS_INT), Qnil ());   // C 1277
	      /* C 1278-1281: "Note: when stepping with GDB through this code, the loop below
		 terminates immediately because EnumClipboardFormats for some reason returns with
		 "Thread does not have a clipboard open" error."  */
	      i = 0;                                              // C 1282
	      format = 0;                                         // C 1282
	      loop
		{
		  format = EnumClipboardFormats (format);         // C 1283
		  if format == 0 { break; }                       // C 1283's condition
		  let name: *const c_char;                        // C 1285
		  /* C 1293's `char fmt_name[256]` is hoisted out of the `else` arm: C 1298 stores
		     its address in `name`, which C 1300 uses after the arm's block ends, and a Rust
		     local's storage ends with its block (the ledger's `## Deviations`).  */
		  let mut fmt_name: [c_char; 256] = [0; 256];
		  if format == CF_LOCALE                         // C 1287
		    { continue; }
		  else if format < CF_MAX                        // C 1289
		    { name = stdfmt_name[format as usize].as_ptr (); }        // C 1290
		  else
		    {
		      if GetClipboardFormatName (format, fmt_name.as_mut_ptr (),
						 fmt_name.len () as c_int) == 0    // C 1295-1296
			{ continue; }                            // C 1297
		      name = fmt_name.as_ptr ();                 // C 1298
		    }
		  ASET (val, i as isize, intern (name));          // C 1300
		  i += 1;                                         // C 1301
		}
	    }
	  CloseClipboard ();                                      // C 1304
	}
      return val;                                             // C 1306
    }
  /* C 1308: "For PRIMARY and SECONDARY we cons the values in w32--get-selection."  */
  return Qnil ();                                             // C 1309
});

/* C's two `globals.f_<name>` fields this unit's `syms_of_w32select` registers (its
   `DEFVAR_LISP` calls at C 1324 and 1330), defined here as the port's storage - C's
   `struct emacs_globals` is zero-initialised and the DEFVAR macros only take each field's
   address, so the initialisers are `lisp_h_Qnil ()` (docs/status.md 3.29, the `src/cmds.rs`
   row and the `src/alloc.rs` 8019-8025 note).  */
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut Vselection_coding_system: Lisp_Object = lisp_h_Qnil ();      // C 1324
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub static mut Vnext_selection_coding_system: Lisp_Object = lisp_h_Qnil (); // C 1330

/* C 1312-1314: "One-time init.  Called in the un-dumped Emacs, but not in the dumped version."  */
/// C 1315-1343: `syms_of_w32select`.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_w32select ()
{
  defsubr (&raw mut Sw32_set_clipboard_data);             // C 1317
  defsubr (&raw mut Sw32_get_clipboard_data);             // C 1318
  defsubr (&raw mut Sw32__get_clipboard_data_media);      // C 1319
  defsubr (&raw mut Sw32_selection_exists_p);             // C 1320
  defsubr (&raw mut Sw32_selection_targets);              // C 1321

  DEFVAR_LISP! (c"selection-coding-system", Vselection_coding_system,
		"SKIP: real doc in select.el.");         // C 1323-1325
  /* C 1326-1327: "The actual value is set dynamically in the dumped Emacs, see below."  */
  Vselection_coding_system = Qnil ();                    // C 1328

  DEFVAR_LISP! (c"next-selection-coding-system", Vnext_selection_coding_system,
		"SKIP: real doc in select.el.");         // C 1330-1331
  Vnext_selection_coding_system = Qnil ();               // C 1332

  DEFSYM! (QCLIPBOARD, "CLIPBOARD");                     // C 1334

  cfg_coding_system = Qnil ();       staticpro (&raw const cfg_coding_system);     // C 1336
  current_text = Qnil ();            staticpro (&raw const current_text);          // C 1337
  current_coding_system = Qnil ();   staticpro (&raw const current_coding_system); // C 1338

  DEFSYM! (Qutf_16le_dos, "utf-16le-dos");               // C 1340
  QANSICP = Qnil (); staticpro (&raw const QANSICP);     // C 1341
  QOEMCP = Qnil ();  staticpro (&raw const QOEMCP);      // C 1342
}

/* C 1345-1347: "One-time init.  Called in the dumped Emacs, but not in the un-dumped version."  */
/// C 1348-1370: `globals_of_w32select`.
#[cfg (feature = "have_ntgui")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn globals_of_w32select ()
{
  DEFAULT_LCID = GetUserDefaultLCID ();                  // C 1351
  /* C 1352-1353: "Drop the sort order from the LCID, so we can compare this with CF_LOCALE objects
     that have the same fix on 9x."  */
  DEFAULT_LCID = MAKELCID! (LANGIDFROMLCID! (DEFAULT_LCID), SORT_DEFAULT);   // C 1354

  ANSICP = GetACP ();                                    // C 1356
  OEMCP = GetOEMCP ();                                   // C 1357

  QANSICP = coding_from_cp (ANSICP);                     // C 1359
  QOEMCP = coding_from_cp (OEMCP);                       // C 1360

  if os_subtype == OS_SUBTYPE_NT                         // C 1362
    { Vselection_coding_system = Qutf_16le_dos (); }     // C 1363
  else if inhibit_window_system                          // C 1364
    { Vselection_coding_system = QOEMCP; }               // C 1365
  else
    { Vselection_coding_system = QANSICP; }              // C 1367

  clipboard_owner = create_owner ();                     // C 1369
}
