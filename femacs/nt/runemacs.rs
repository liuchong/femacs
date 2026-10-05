// femacs: port of nt/runemacs.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, local), the same control flow and the same error model.  `WinMain` is
// C's program entry point (`int WINAPI`), so it keeps `#[unsafe(no_mangle)]` and the `extern "system"`
// convention WINAPI stands for (PORTING.md §5.1); C's two file-`static` helpers stay plain `unsafe fn`,
// and C 50-51's prototype pair is dropped (PORTING.md 0.1-12, forward declarations).
//
// **This is a P-86 unit: the frozen configuration builds no `nt/` object at all.**  `runemacs` is a
// Windows-only program (`emacs/nt/Makefile.in`'s `OTHERS`; `docs/baseline/frozen-build-sources.txt`
// records no `nt/runemacs` object for this target), so `extract-abi.py`'s checklist for the file is
// empty (P-86 ②) and the whole text is carried under the **build-level file gate**
// `feature = "windowsnt"`, on every item *and on the include graph* (P-86 ⑧) - which is why the
// frozen configuration compiles an empty object from it.  C 94's `#ifdef CHOOSE_NEWEST_EXE` is
// `feature = "choose_newest_exe"`: C 41's `#define` is *commented out*, so the frozen side is C 124's
// `#else` arm, and **both arms are carried** (P-86 ①).
//
// C 46's `<windows.h>` and C 48's `<malloc.h>` have no mirror - there is no Windows SDK and no MSVC
// CRT on this machine (the `femacs/src/w32proc.rs` acquisition note: no `windows.h`/`winnt.h`
// anywhere, no mingw-w64) - so the names this unit uses are declared at C 46's position below
// (P-88 ①, the `src/w32*.rs` shape): every SDK `typedef`/struct/constant/entry point carries its
// own `/// C:` line, **every constant value is a placeholder** and the whole section is
// `UNVERIFIED` (P-88 ②: no numbers from memory).  `alloca` (C 78/99) is P-05's `sa_stack_alloc`
// (`femacs/src/alloc.rs` 1039).
//
// Ledger: `docs/ports/runemacs.md` carries the C line ranges, the acceptance readings and every
// deviation (the dropped `DEFER_MS_W32_H`/forward declarations, the zero-initialised locals
// PORTING.md 1.5 requires, the placeholder values, the AppUserModelID array, the labelled block
// that replaces C's `goto error`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens, unused_labels, clippy::all)]

use std::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

include!("../src/include/config.rs");        // C 44: `#include <config.h>`

/* C 43: `#define DEFER_MS_W32_H` - the reference's own guard so that `w32.h` (which several `nt/`
   units include before `<windows.h>`) does not pull it in twice.  This unit includes `<windows.h>`
   itself at C 46 and no `w32.h` is in its graph, so the macro has no site in the port (Deviation 1).  */

#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");        // C 47: `#include <string.h>`

/* ----------------------------------------------------------------------------------------------
   C 46: `#include <windows.h>` - the SDK surface this unit uses (P-88 ①: no mirror carries
   `<windows.h>`, so the unit declares what it names).  C 48's `<malloc.h>` contributes only
   `alloca`, handled below as P-05's `sa_stack_alloc`.
   ---------------------------------------------------------------------------------------------- */

/// C: typedef void *HANDLE, *HMODULE, *HINSTANCE, *LPVOID; (the `w32proc.rs` 276-284 reading)
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
/// C: typedef unsigned long DWORD;   // measured (`src/w32.rs` 230, `w32proc.rs` 257)
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
/// C: typedef int BOOL;   // measured (`src/w32.rs` 236)
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
/// C: typedef unsigned short WORD;   // measured (`w32proc.rs` 253)
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
/// C: typedef unsigned int UINT;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
/// C: typedef LONG_PTR HRESULT; (`LONG_PTR` is a pointer-sized signed integer on a 64-bit target)
#[cfg (feature = "windowsnt")]
pub type HRESULT = isize;
/// C: typedef INT_PTR (WINAPI *FARPROC) (void); - `GetProcAddress`'s result.
#[cfg (feature = "windowsnt")]
pub type FARPROC = Option<unsafe extern "system" fn () -> isize>;

/// C: #define MAX_PATH 260   // measured (`src/w32.rs` 316, clang against the frozen SDK table)
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;
/// C: #define TRUE 1 / #define FALSE 0   // measured (`src/w32.rs` 310/313)
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
/// C: #define INVALID_HANDLE_VALUE ((HANDLE) (LONG_PTR) -1)   // measured (`src/sysdep.rs` 1058)
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
/// C: #define VER_PLATFORM_WIN32_WINDOWS 0x1   // measured (`src/w32.rs`, its own `GetVersionEx` use)
#[cfg (feature = "windowsnt")]
pub const VER_PLATFORM_WIN32_WINDOWS: DWORD = 0x1;

/* The `STARTUPINFO.dwFlags`/`wShowWindow` and `CreateProcess` constants, and the `MessageBox` family
   C 214-234 uses: no SDK is installed, so each value below is a **placeholder** and the section is
   UNVERIFIED (P-88 ②).  The ledger's Owed list names the Windows host that would measure them.  */
/// C: #define STARTF_USESHOWWINDOW 0x00000001
#[cfg (feature = "windowsnt")]
pub const STARTF_USESHOWWINDOW: DWORD = 0x00000001;   // placeholder, not <winbase.h>'s value
/// C: #define STARTF_USECOUNTCHARS 0x00000008
#[cfg (feature = "windowsnt")]
pub const STARTF_USECOUNTCHARS: DWORD = 0x00000008;   // placeholder, not <winbase.h>'s value
/// C: #define SW_HIDE 0
#[cfg (feature = "windowsnt")]
pub const SW_HIDE: WORD = 0;                          // placeholder, not <winuser.h>'s value
/// C: #define SW_SHOWMINNOACTIVE 7
#[cfg (feature = "windowsnt")]
pub const SW_SHOWMINNOACTIVE: c_int = 7;              // placeholder, not <winuser.h>'s value
/// C: #define SW_SHOWMAXIMIZED 3
#[cfg (feature = "windowsnt")]
pub const SW_SHOWMAXIMIZED: c_int = 3;                // placeholder, not <winuser.h>'s value
/// C: #define NORMAL_PRIORITY_CLASS 0x00000020
#[cfg (feature = "windowsnt")]
pub const NORMAL_PRIORITY_CLASS: DWORD = 0x00000020;  // placeholder, not <winbase.h>'s value
/// C: #define HIGH_PRIORITY_CLASS 0x00000080
#[cfg (feature = "windowsnt")]
pub const HIGH_PRIORITY_CLASS: DWORD = 0x00000080;    // placeholder, not <winbase.h>'s value
/// C: #define IDLE_PRIORITY_CLASS 0x00000040
#[cfg (feature = "windowsnt")]
pub const IDLE_PRIORITY_CLASS: DWORD = 0x00000040;    // placeholder, not <winbase.h>'s value
/// C: #define INFINITE 0xFFFFFFFF
#[cfg (feature = "windowsnt")]
pub const INFINITE: DWORD = 0xFFFFFFFF;               // placeholder, not <winbase.h>'s value
/// C: #define MB_OK 0x00000000
#[cfg (feature = "windowsnt")]
pub const MB_OK: UINT = 0x00000000;                   // placeholder, not <winuser.h>'s value
/// C: #define MB_ICONERROR 0x00000010 / #define MB_ICONSTOP MB_ICONERROR
#[cfg (feature = "windowsnt")]
pub const MB_ICONERROR: UINT = 0x00000010;            // placeholder, not <winuser.h>'s value
#[cfg (feature = "windowsnt")]
pub const MB_ICONSTOP: UINT = MB_ICONERROR;
/// C: #define MB_TASKMODAL 0x00002000
#[cfg (feature = "windowsnt")]
pub const MB_TASKMODAL: UINT = 0x00002000;            // placeholder, not <winuser.h>'s value
/// C: #define MB_SETFOREGROUND 0x00010000
#[cfg (feature = "windowsnt")]
pub const MB_SETFOREGROUND: UINT = 0x00010000;        // placeholder, not <winuser.h>'s value
/// C: #define IDOK 1
#[cfg (feature = "windowsnt")]
pub const IDOK: c_int = 1;                            // placeholder, not <winuser.h>'s value

/// C 206: `set_user_model` is called with C's own wide-string AppUserModelID - the id the Windows
/// shell groups this program's windows by.  **This is our program saying what it is called**, so the port carries
/// its own name (PORTING.md 0.1-19's name half, status 3.433/3.503): Rust has no wide-string
/// literal, so the UTF-16 code units are spelled in an array (host-endian, as C's `L"..."` is;
/// the trailing `0x0000` is C's terminator) - the `src/w32fns.rs` 391 `DARK_MODE_APP_NAME` shape,
/// and the same treatment `PACKAGE_NAME`/`emacs.rs 359` got.  C's own text stays in the ledger.
#[cfg (feature = "windowsnt")]
pub const USER_MODEL_ID: [u16; 7] =
  [0x0066, 0x0065, 0x006D, 0x0061, 0x0063, 0x0073, 0x0000];

/// C: typedef struct _STARTUPINFOA { DWORD cb; LPSTR lpReserved, lpDesktop, lpTitle; DWORD dwX, dwY,
/// C:   dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags; WORD wShowWindow,
/// C:   cbReserved2; LPBYTE lpReserved2; HANDLE hStdInput, hStdOutput, hStdError; } STARTUPINFOA;
/// Every offset is `UNVERIFIED` here (no Windows SDK/toolchain on this machine).
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STARTUPINFO
{
  pub cb: DWORD,
  pub lpReserved: LPSTR,
  pub lpDesktop: LPSTR,
  pub lpTitle: LPSTR,
  pub dwX: DWORD,
  pub dwY: DWORD,
  pub dwXSize: DWORD,
  pub dwYSize: DWORD,
  pub dwXCountChars: DWORD,
  pub dwYCountChars: DWORD,
  pub dwFillAttribute: DWORD,
  pub dwFlags: DWORD,
  pub wShowWindow: WORD,
  pub cbReserved2: WORD,
  pub lpReserved2: *mut u8,      /* C: LPBYTE lpReserved2; */
  pub hStdInput: HANDLE,
  pub hStdOutput: HANDLE,
  pub hStdError: HANDLE,
}

/// C: typedef struct _SECURITY_ATTRIBUTES { DWORD nLength; LPVOID lpSecurityDescriptor;
/// C:   BOOL bInheritHandle; } SECURITY_ATTRIBUTES, *PSECURITY_ATTRIBUTES, *LPSECURITY_ATTRIBUTES;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES
{
  pub nLength: DWORD,
  pub lpSecurityDescriptor: LPVOID,
  pub bInheritHandle: BOOL,
}

/// C: typedef struct _PROCESS_INFORMATION { HANDLE hProcess, hThread; DWORD dwProcessId,
/// C:   dwThreadId; } PROCESS_INFORMATION, *PPROCESS_INFORMATION, *LPPROCESS_INFORMATION;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_INFORMATION
{
  pub hProcess: HANDLE,
  pub hThread: HANDLE,
  pub dwProcessId: DWORD,
  pub dwThreadId: DWORD,
}

/// C 249-253's record, declared exactly as `src/w32.rs` 343 declares it (the same SDK header, the
/// same ANSI arm: the machine has no toolchain to measure either copy, and that file's ledger says
/// the same).
/// C: typedef struct _OSVERSIONINFOA { DWORD dwOSVersionInfoSize; DWORD dwMajorVersion;
/// C:   DWORD dwMinorVersion; DWORD dwBuildNumber; DWORD dwPlatformId; CHAR szCSDVersion[128];
/// C:   } OSVERSIONINFOA, *POSVERSIONINFOA, *LPOSVERSIONINFOA;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OSVERSIONINFO
{
  pub dwOSVersionInfoSize: DWORD,
  pub dwMajorVersion: DWORD,
  pub dwMinorVersion: DWORD,
  pub dwBuildNumber: DWORD,
  pub dwPlatformId: DWORD,
  pub szCSDVersion: [c_char; 128],
}

/// C 100/110-112's record - the ANSI `WIN32_FIND_DATAA` (C 105's `FindFirstFile` is the SDK's
/// `...A`/`...W` alias pair under `_UNICODE`, and C 64's `char modname[]` shows this build is the
/// ANSI one, so the port keeps C's alias name and the ANSI record).  `src/w32.rs` 374's `FILETIME`
/// is reused by name: same record, same SDK header.
/// C: typedef struct _WIN32_FIND_DATAA { DWORD dwFileAttributes; FILETIME ftCreationTime;
/// C:   FILETIME ftLastAccessTime; FILETIME ftLastWriteTime; DWORD nFileSizeHigh, nFileSizeLow,
/// C:   dwReserved0, dwReserved1; CHAR cFileName[MAX_PATH]; CHAR cAlternateFileName[14];
/// C:   } WIN32_FIND_DATAA, *PWIN32_FIND_DATAA, *LPWIN32_FIND_DATAA;
/// C: typedef struct _FILETIME { DWORD dwLowDateTime; DWORD dwHighDateTime; } FILETIME;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILETIME
{
  pub dwLowDateTime: DWORD,
  pub dwHighDateTime: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_FIND_DATA
{
  pub dwFileAttributes: DWORD,
  pub ftCreationTime: FILETIME,
  pub ftLastAccessTime: FILETIME,
  pub ftLastWriteTime: FILETIME,
  pub nFileSizeHigh: DWORD,
  pub nFileSizeLow: DWORD,
  pub dwReserved0: DWORD,
  pub dwReserved1: DWORD,
  pub cFileName: [c_char; MAX_PATH as usize],
  pub cAlternateFileName: [c_char; 14],
}

/// C 72/105/110/118/126/145/215/224/249/252: the `<windows.h>` entry points this unit calls.  Each
/// is the SDK's `...A`/`...W` alias pair under `_UNICODE`; C's text writes the alias and the port
/// keeps C's name (`src/w32.rs` 404-409's `LoadLibrary`/`GetModuleHandle` reading).
/// C: WINBASEAPI DWORD WINAPI GetModuleFileNameA (HMODULE hModule, LPSTR lpFilename, DWORD nSize);
/// C: WINBASEAPI HANDLE WINAPI FindFirstFileA (LPCSTR lpFileName, LPWIN32_FIND_DATAA lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindNextFileA (HANDLE hFindFile, LPWIN32_FIND_DATAA lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindClose (HANDLE hFindFile);
/// C: WINBASEAPI BOOL WINAPI SetEnvironmentVariableA (LPCSTR lpName, LPCSTR lpValue);
/// C: WINBASEAPI BOOL WINAPI CreateProcessA (LPCSTR lpApplicationName, LPSTR lpCommandLine,
/// C:   LPSECURITY_ATTRIBUTES lpProcessAttributes, LPSECURITY_ATTRIBUTES lpThreadAttributes,
/// C:   BOOL bInheritHandles, DWORD dwCreationFlags, LPVOID lpEnvironment, LPCSTR lpCurrentDirectory,
/// C:   LPSTARTUPINFOA lpStartupInfo, LPPROCESS_INFORMATION lpProcessInformation);
/// C: WINBASEAPI DWORD WINAPI WaitForSingleObject (HANDLE hHandle, DWORD dwMilliseconds);
/// C: WINBASEAPI BOOL WINAPI GetExitCodeProcess (HANDLE hProcess, LPDWORD lpExitCode);
/// C: WINBASEAPI BOOL WINAPI CloseHandle (HANDLE hObject);
/// C: WINBASEAPI HMODULE WINAPI LoadLibraryA (LPCSTR lpLibFileName);
/// C: WINBASEAPI FARPROC WINAPI GetProcAddress (HMODULE hModule, LPCSTR lpProcName);
/// C: WINBASEAPI BOOL WINAPI FreeLibrary (HMODULE hLibModule);
/// C: WINBASEAPI int WINAPI MessageBoxA (HWND hWnd, LPCSTR lpText, LPCSTR lpCaption, UINT uType);
/// C: WINBASEAPI BOOL WINAPI GetVersionExA (LPOSVERSIONINFOA lpVersionInformation);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetModuleFileName (hModule: HMODULE, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
  pub fn FindFirstFile (lpFileName: LPCSTR, lpFindFileData: *mut WIN32_FIND_DATA) -> HANDLE;
  pub fn FindNextFile (hFindFile: HANDLE, lpFindFileData: *mut WIN32_FIND_DATA) -> BOOL;
  pub fn FindClose (hFindFile: HANDLE) -> BOOL;
  pub fn SetEnvironmentVariable (lpName: LPCSTR, lpValue: LPCSTR) -> BOOL;
  pub fn CreateProcess (lpApplicationName: LPCSTR, lpCommandLine: LPSTR,
                        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
                        lpThreadAttributes: *mut SECURITY_ATTRIBUTES, bInheritHandles: BOOL,
                        dwCreationFlags: DWORD, lpEnvironment: LPVOID,
                        lpCurrentDirectory: LPCSTR, lpStartupInfo: *mut STARTUPINFO,
                        lpProcessInformation: *mut PROCESS_INFORMATION) -> BOOL;
  pub fn WaitForSingleObject (hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
  pub fn GetExitCodeProcess (hProcess: HANDLE, lpExitCode: *mut DWORD) -> BOOL;
  pub fn CloseHandle (hObject: HANDLE) -> BOOL;
  pub fn LoadLibrary (lpLibFileName: LPCSTR) -> HMODULE;
  pub fn GetProcAddress (hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
  pub fn FreeLibrary (hLibModule: HMODULE) -> BOOL;
  pub fn MessageBox (hWnd: *mut c_void, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
  pub fn GetVersionEx (lpVersionInformation: *mut OSVERSIONINFO) -> BOOL;
}

/* C 87/91/92/103/104/118/119/131/132/162/163/184/185: `strcat` and `stricmp`, which
   `lib/include/string.rs` does not carry (it has strlen/strcmp/strncmp/strcpy/strchr/strspn/
   strerror/strrchr) - declared at the use site per P-88 ①, `stricmp` being the MSVC CRT's name for
   a case-insensitive `strcmp`.  */
/// C: <string.h>: `char *strcat (char *dest, const char *src);`
/// C: <string.h> (MSVC): `int stricmp (const char *s1, const char *s2);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn stricmp (s1: *const c_char, s2: *const c_char) -> c_int;
}

/// C 78/99: `alloca` (C 48's `<malloc.h>`) is P-05's stack-pool allocator, `femacs/src/alloc.rs`
/// 1039 (`pub unsafe extern "C" fn sa_stack_alloc (size: usize) -> *mut c_void`), reached here by
/// its own name - the `src/sfnt.rs` `sfnt_build_outline_edges` reading, one `## Deviations` entry.
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn sa_stack_alloc (size: usize) -> *mut c_void;
}

/* C: #define ZeroMemory RtlZeroMemory / #define RtlZeroMemory(Destination,Length) memset((Destination),0,(Length))
   C 250: the SDK's zeroing macro, whose target here is a `ZeroMemory` the unit defines itself (the
   `src/w32.rs` 325-328 treatment of the same macro: `<windows.h>`'s spelling of `memset`).  */
#[cfg (feature = "windowsnt")]
macro_rules! ZeroMemory {
  ($dst:expr, $len:expr) => { memset ($dst as *mut c_void, 0, $len) };
}

/* C 82: `max (sizeof (iconic_opt), sizeof (maximized_opt))` - `max` is `<windows.h>`'s `<windef.h>`
   macro (`#define max(a,b) (((a) > (b)) ? (a) : (b))`, MSVC's, unless `NOMINMAX`); no mirror carries
   it, so the unit defines the same-shaped macro at C 82's position (P-88 ①, its own `## Deviations`
   entry - the port's `lib/include/min-max.rs` `max!` is gnulib's *different* header).  */
/// C: #define max(a,b) (((a) > (b)) ? (a) : (b))   [<windef.h>, through <windows.h>]
#[cfg (feature = "windowsnt")]
macro_rules! max {
  ($a:expr, $b:expr) => { if ($a) > ($b) { $a } else { $b } };
}

/// C: <string.h>: `void *memset (void *s, int c, size_t n);` - the target of `ZeroMemory!` above
/// (`lib/include/string.rs` does not carry it; `src/w32.rs` 1163 reaches the same name through
/// `lisp.h`'s chain, which this unit has no part of).
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn memset (s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

/* ----------------------------------------------------------------------------------------------
   C 53-190: `WinMain` and C 192-270: the two `static` helpers, in C's order.
   ---------------------------------------------------------------------------------------------- */

// C 53-54: `int WINAPI WinMain (HINSTANCE hSelf, HINSTANCE hPrev, LPSTR cmdline, int nShow)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn WinMain (hSelf: HINSTANCE, hPrev: HINSTANCE, cmdline: LPSTR,
                                       nShow: c_int) -> c_int
{
  // C 56-65: the locals.  PORTING.md 1.5: C leaves `start`/`sec_attrs`/`child` uninitialised (C 137
  // zeroes `start`, C 152-154 fills `sec_attrs` field by field, and `CreateProcess` writes `child`
  // before its first read), so the port zero-initialises the three records and says so here;
  // `modname` is C's `char modname[MAX_PATH]`, written by `GetModuleFileName` at C 72; `new_cmdline`
  // and `p` are assigned before their first read on every path.
  let mut start: STARTUPINFO = core::mem::zeroed ();              /* C 56 */
  let mut sec_attrs: SECURITY_ATTRIBUTES = core::mem::zeroed ();  /* C 57 */
  let mut child: PROCESS_INFORMATION = core::mem::zeroed ();      /* C 58 */
  let mut wait_for_child: c_int = FALSE;                          /* C 59 */
  let mut priority_class: DWORD = NORMAL_PRIORITY_CLASS;          /* C 60 */
  let mut ret_code: DWORD = 0;                                    /* C 61 */
  let new_cmdline: *mut c_char;                                   /* C 62 */
  let mut p: *mut c_char;                                         /* C 63 */
  let mut modname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 64 */
  // C 65: `static const char iconic_opt[] = "--iconic ", maximized_opt[] = "--maximized ";` - the
  // port keeps C's bytes (without the NUL) and adds one where C's `sizeof` counts the terminator
  // (the `xg_set_widget_bg` `format` reading).
  const iconic_opt: &[u8] = b"--iconic ";
  const maximized_opt: &[u8] = b"--maximized ";

  /* C 67-188: `goto error` jumps out of this sequence to C 190's label, which the port spells as a
     labelled block whose `break` is the `goto` (PORTING.md 2.2); the block's tail below is C 190-192.  */
  'error: {
    // C 67-68: `if (!ensure_unicows_dll ()) goto error;`
    if !ensure_unicows_dll ()
      {
        break 'error;
      }

    // C 70
    set_user_model_id ();

    // C 72-73: `if (!GetModuleFileName (NULL, modname, MAX_PATH)) goto error;` - the return value
    // is the count written, so C's `!value` is `== 0` (PORTING.md 2.3's truth test).
    if GetModuleFileName (std::ptr::null_mut (), modname.as_mut_ptr (), MAX_PATH as DWORD) == 0
      {
        break 'error;
      }
    // C 74-76: `if ((p = strrchr (modname, '\\')) == NULL) goto error; *p = 0;`
    p = strrchr (modname.as_ptr (), b'\\' as c_int);
    if p.is_null ()
      {
        break 'error;
      }
    *p = 0;

    // C 78-84: `new_cmdline = alloca (MAX_PATH + strlen (cmdline) + ((nShow == SW_SHOWMINNOACTIVE
    //           || nShow == SW_SHOWMAXIMIZED) ? max (sizeof (iconic_opt), sizeof (maximized_opt)) : 0) + 3);`
    new_cmdline = sa_stack_alloc ((MAX_PATH
                                   + strlen (cmdline) as c_int
                                   + (if nShow == SW_SHOWMINNOACTIVE
                                         || nShow == SW_SHOWMAXIMIZED
                                       { (max! (iconic_opt.len () + 1, maximized_opt.len () + 1))
                                           as c_int }
                                       else { 0 })
                                   + 3) as usize) as *mut c_char;
    // C 85-86: C's `/* Quote executable name in case of spaces in the path. */` comment, then
    // `*new_cmdline = '"';`
    *new_cmdline = b'"' as c_char;
    // C 87: `strcpy (new_cmdline + 1, modname);`
    strcpy (new_cmdline.offset (1), modname.as_ptr ());
    /* C 88-92: detect and handle an un-installed runemacs.exe in an nt/ subdirectory, while
       emacs.exe is in src/.  */
    p = strrchr (new_cmdline, b'\\' as c_int);
    if ! p.is_null () && stricmp (p, c"\\nt".as_ptr ()) == 0
      {
        strcpy (p, c"\\src".as_ptr ());
      }

    /* C 94-123: `#ifdef CHOOSE_NEWEST_EXE` - "silly hack to allow new versions to be installed on
       server even when current version is in use".  C 41's `#define` is commented out, so this arm
       is the dead one on every configuration the tree records; it is carried as its own feature
       (P-86 ①).  */
    #[cfg (feature = "choose_newest_exe")]
    {
      let best_name: *mut c_char = sa_stack_alloc ((MAX_PATH + 1) as usize) as *mut c_char;
      let mut best_time: FILETIME = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };  /* C 100 */
      let mut wfd: WIN32_FIND_DATA = core::mem::zeroed ();                             /* C 101 */
      let mut fh: HANDLE;                                                              /* C 102 */
      p = new_cmdline.offset (strlen (new_cmdline) as isize);                          /* C 103 */
      strcpy (p, c"\\emacs*.exe\" ".as_ptr ());                                        /* C 104 */
      fh = FindFirstFile (new_cmdline, &raw mut wfd);                                  /* C 105 */
      if fh == INVALID_HANDLE_VALUE                                                    /* C 106-107 */
        {
          break 'error;
        }
      /* C 108-122: `do { … } while (FindNextFile (fh, &wfd));` - C's do-while is the port's `loop`
         with the test moved to the end (PORTING.md 2.2).  */
      loop
        {
          if wfd.ftLastWriteTime.dwHighDateTime > best_time.dwHighDateTime
            || (wfd.ftLastWriteTime.dwHighDateTime == best_time.dwHighDateTime
                && wfd.ftLastWriteTime.dwLowDateTime > best_time.dwLowDateTime)
            {
              best_time = wfd.ftLastWriteTime;
              strcpy (best_name, wfd.cFileName.as_ptr ());
            }
          if !FindNextFile (fh, &raw mut wfd)
            {
              break;
            }
        }
      FindClose (fh);                                                                  /* C 120 */
      /* C 121-123: `*p++ = '\\'; strcpy (p, best_name); strcat (p, " ");` - Rust has no postfix
         increment, so the write and the step are two statements (the `src/sfnt.rs` reading of the
         same C idiom).  */
      *p = b'\\' as c_char;
      p = p.offset (1);
      strcpy (p, best_name);
      strcat (p, c" ".as_ptr ());
    }
    /* C 124-125: `#else` - `strcat (new_cmdline, "\\emacs.exe\" ");`  */
    #[cfg (not (feature = "choose_newest_exe"))]
    {
      strcat (new_cmdline, c"\\emacs.exe\" ".as_ptr ());
    }

    /* C 127-146: append the original arguments, applying the ones this program recognises
       (`-wait`, `-high`, `-low`) itself.  */
    while *cmdline == b'-' as c_char || *cmdline == b'/' as c_char
      {
        if strncmp (cmdline.offset (1), c"wait".as_ptr (), 4) == 0
          {
            wait_for_child = TRUE;
            cmdline = cmdline.offset (5);
          }
        else if strncmp (cmdline.offset (1), c"high".as_ptr (), 4) == 0
          {
            priority_class = HIGH_PRIORITY_CLASS;
            cmdline = cmdline.offset (5);
          }
        else if strncmp (cmdline.offset (1), c"low".as_ptr (), 3) == 0
          {
            priority_class = IDLE_PRIORITY_CLASS;
            cmdline = cmdline.offset (4);
          }
        else
          {
            break;
          }
        /* C 144-145: `while (*++cmdline == ' ');` - the pre-increment is the port's two steps.  */
        loop
          {
            cmdline = cmdline.offset (1);
            if ! (*cmdline == b' ' as c_char)
              {
                break;
              }
          }
      }

    /* C 148-153: pass `--iconic`/`--maximized` on when the shortcut asked for that window state.  */
    if nShow == SW_SHOWMINNOACTIVE
      {
        strcat (new_cmdline, iconic_opt.as_ptr () as *const c_char);
      }
    else if nShow == SW_SHOWMAXIMIZED
      {
        strcat (new_cmdline, maximized_opt.as_ptr () as *const c_char);
      }
    strcat (new_cmdline, cmdline);

    /* C 155-162: set `emacs_dir` when runemacs was in `%emacs_dir%\bin`.  */
    p = strrchr (modname.as_ptr (), b'\\' as c_int);
    if ! p.is_null () && stricmp (p, c"\\bin".as_ptr ()) == 0
      {
        *p = 0;
        /* C 159-160: `for (p = modname; *p; p++) if (*p == '\\') *p = '/';`  */
        p = modname.as_mut_ptr ();
        while *p != 0
          {
            if *p == b'\\' as c_char
              {
                *p = b'/' as c_char;
              }
            p = p.offset (1);
          }
        SetEnvironmentVariable (c"emacs_dir".as_ptr (), modname.as_ptr ());
      }

    /* C 172: `memset (&start, 0, sizeof (start));` - C writes `memset` here and the SDK's
       `ZeroMemory` in `ensure_unicows_dll` (C 239), so the port spells both as C does.  */
    memset (&raw mut start, 0, core::mem::size_of::<STARTUPINFO> ());
    start.cb = core::mem::size_of::<STARTUPINFO> () as DWORD;
    start.dwFlags = STARTF_USESHOWWINDOW | STARTF_USECOUNTCHARS;
    start.wShowWindow = SW_HIDE;
    /* C 169-171: don't waste memory if the user specified a huge default screen buffer.  */
    start.dwXCountChars = 80;
    start.dwYCountChars = 25;

    /* C 173-175 */
    sec_attrs.nLength = core::mem::size_of::<SECURITY_ATTRIBUTES> () as DWORD;
    sec_attrs.lpSecurityDescriptor = std::ptr::null_mut ();
    sec_attrs.bInheritHandle = FALSE;

    /* C 177-188 */
    if CreateProcess (std::ptr::null (), new_cmdline, &raw mut sec_attrs, std::ptr::null_mut (),
                      TRUE, priority_class, std::ptr::null_mut (), std::ptr::null (),
                      &raw mut start, &raw mut child) != 0
      {
        if wait_for_child != 0
          {
            WaitForSingleObject (child.hProcess, INFINITE);
            GetExitCodeProcess (child.hProcess, &raw mut ret_code);
          }
        CloseHandle (child.hThread);
        CloseHandle (child.hProcess);
      }
    else
      {
        break 'error;
      }
    return ret_code as c_int;
  }

  /* C 190-192: `error: MessageBox (NULL, "Could not start Emacs.", "Error", MB_ICONSTOP); return 1;`  */
  MessageBox (std::ptr::null_mut (), c"Could not start Emacs.".as_ptr (), c"Error".as_ptr (),
              MB_ICONSTOP);
  1
}

// C 194-214: `void set_user_model_id (void)`
#[cfg (feature = "windowsnt")]
unsafe fn set_user_model_id ()
{
  let mut shell: HMODULE;                                            /* C 208 */
  /* C 209: `HRESULT (WINAPI * set_user_model) (const wchar_t * id);` - C's local declaration.
     `wchar_t` is the SDK's own typedef (a 16-bit code unit on this ABI, declared here because no
     mirror carries `<wchar.h>`), `HRESULT` the file's alias for it, and `WINAPI` is the calling
     convention the port spells `extern "system"` (PORTING.md §5.1; the `winapi` token row is in
     `docs/ports/macro-body-notes.tsv`).  */
  type wchar_t = u16;
  type set_user_model = Option<unsafe extern "system" fn (id: *const wchar_t) -> HRESULT>;
  let mut set_user_model: set_user_model;                            /* C 209 */

  /* C 199-201: on Windows 7 and later, set the user model ID so emacsclient-launched files group
     with Emacs frames in the UI.  */
  shell = LoadLibrary (c"shell32.dll".as_ptr ());
  if ! shell.is_null ()
    {
      /* C 203-205: `set_user_model = (void *) GetProcAddress (shell,
         "SetCurrentProcessExplicitAppUserModelID");` - `GetProcAddress`'s `FARPROC` is cast to the
         function-pointer type C declares at C 197 (PORTING.md's function-pointer rule).  */
      /* C 211-212: `set_user_model = (void *) GetProcAddress (shell,
         "SetCurrentProcessExplicitAppUserModelID");`  */
      set_user_model = core::mem::transmute::<FARPROC, set_user_model>
        (GetProcAddress (shell, c"SetCurrentProcessExplicitAppUserModelID".as_ptr ()));

      /* C 207-212: if the function is defined we are on Windows 7 or newer, and the UI groups
         emacs, runemacs and emacsclient by this ID.  */
      if set_user_model.is_some ()
        {
          (set_user_model.unwrap ()) (USER_MODEL_ID.as_ptr ());
        }

      FreeLibrary (shell);
    }
}

// C 216-270: `static int ensure_unicows_dll (void)`
#[cfg (feature = "windowsnt")]
unsafe fn ensure_unicows_dll () -> c_int
{
  let mut os_ver: OSVERSIONINFO = core::mem::zeroed ();              /* C 218 */
  let mut h: HMODULE;                                                /* C 219 */

  ZeroMemory! (&raw mut os_ver, core::mem::size_of::<OSVERSIONINFO> ());       /* C 221 */
  os_ver.dwOSVersionInfoSize = core::mem::size_of::<OSVERSIONINFO> () as DWORD; /* C 222 */
  if GetVersionEx (&raw mut os_ver) == 0                                        /* C 223-224 */
    { return 0; }

  if os_ver.dwPlatformId == VER_PLATFORM_WIN32_WINDOWS                          /* C 226 */
    {
      h = LoadLibrary (c"Unicows.dll".as_ptr ());                               /* C 228 */
      if h.is_null ()                                                           /* C 229 */
        {
          let button: c_int;                                                    /* C 231 */

          /* C 233-240: C's four adjacent string literals are one literal.  */
          button = MessageBox (std::ptr::null_mut (),
                               c"Emacs cannot load the UNICOWS.DLL library.\n\
This library is essential for using Emacs\n\
on this system.  You need to install it.\n\n\
Emacs will exit when you click OK.".as_ptr (),
                               c"Emacs cannot load UNICOWS.DLL".as_ptr (),
                               MB_ICONERROR | MB_TASKMODAL | MB_SETFOREGROUND | MB_OK);
          match button                                                          /* C 241-248 */
            {
              IDOK => { return 0; }
              _ => { return 0; }
            }
        }
      FreeLibrary (h);                                                          /* C 251 */
      return 1;                                                                 /* C 252 */
    }
  return 1;                                                                     /* C 270 */
}
