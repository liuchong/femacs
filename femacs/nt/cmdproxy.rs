// femacs: port of nt/cmdproxy.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, local), the same control flow and the same error model.  C 616's
// `main` is the program entry point (`int main (int, char **)`), so it keeps `#[unsafe(no_mangle)]`
// and C's calling convention; the file's `static` helpers stay plain `unsafe fn`, and C's forward
// declarations (C 64-72's prototypes and C 519's `console_event_handler`) are dropped
// (PORTING.md 0.1-12).  C 844's `goto pass_to_shell` is the labelled `loop` at that position and
// C 959's backward `goto` is its `continue` (PORTING.md 2.2).
//
// **This is a P-86 unit: the frozen configuration builds no `nt/` object at all.**  `cmdproxy` is
// built by `emacs/nt/Makefile.in` for the Windows port only (`docs/baseline/frozen-build-sources.txt`
// records no `nt/cmdproxy` object for this target), so `extract-abi.py`'s checklist for the file is
// empty (P-86 clause 2) and the whole text is carried under the **build-level file gate**
// `feature = "windowsnt"`, on every item *and on the include graph* (P-86 clause 8) - which is why
// the frozen `build-tu` compiles an empty object from it.  The file's own conditions are C 49's
// `#ifdef _UCRT` (both arms carried), C 64's `__GNUC__` comparison (both declaration arms carried),
// and the three `#if 0` regions C 497-510, C 543-550 and C 800-809, which C's own preprocessor
// never compiles (the port keeps C's text inside a comment, the `src/cm.rs` 86 reading).
//
// C 37's `<windows.h>` and the names C 39-43's headers contribute have no mirror here: there is no
// Windows SDK and no MSVC CRT on this machine (the `femacs/src/w32proc.rs` acquisition note: no
// `windows.h`/`winnt.h` anywhere, no mingw-w64), so the names this unit takes are declared at C 37's
// position below (P-88 clause 1, the `femacs/nt/runemacs.rs` shape): every SDK `typedef`, record,
// constant and entry point carries its own `/// C:` line, **every value is a placeholder unless its
// line names the landed unit that measured it**, and the whole section is `UNVERIFIED` (P-88
// clause 2: no numbers from memory).  `alloca` (C 389/429/755/883/928) is P-05's `sa_stack_alloc`
// (`femacs/src/alloc.rs` 1039).
//
// Ledger: `docs/ports/cmdproxy.md` carries the C line ranges, the readings of this round and every
// deviation (the dropped prototypes, the zero-initialised locals PORTING.md 1.5 requires, the
// placeholder SDK values, the `_UCRT` alias, the `if`/`match` shapes C's control flow takes).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens, unused_labels, unreachable_code,
         clippy::all)]

/* The four names below are the mirrors' own: `lib/include/stdlib.rs` 14/16/37 spells its three
   string-to-number entries with `c_long`/`c_double`/`c_ulong`, which an `include!`d mirror takes
   from the unit that includes it (the `nt/addpm.rs` 40 reading).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");        // C 35: `#include <config.h>`

/* C 30: `#define DEFER_MS_W32_H` - the marker `nt/inc/ms-w32.h` reads to defer itself.  No `w32.h`
   is in this unit's graph, and the name is the row `docs/ports/excluded-macros.txt` carries for it
   (`femacs/src/w32.rs` 83 is the const that replaces it elsewhere).  It has no site here.  */

/* C 31-34: C's own note that it does not want the stdio-consolesafe redefinitions of the *printf
   functions (option (a): that would pull in stdio, see below; option (b): this file has its own
   implementations of *printf), then `#define OMIT_CONSOLESAFE 1`.  `src/conf_post.h` 400 reads the
   value (`#if defined WINDOWSNT && !(defined OMIT_CONSOLESAFE && OMIT_CONSOLESAFE == 1)`); the
   `config.rs` mirror is generated from the frozen configuration's own table, which has no
   `WINDOWSNT`, so the macro's *name* is kept here and the branch it steers is the Windows
   target's.  */
#[cfg (feature = "windowsnt")]
pub const OMIT_CONSOLESAFE: c_int = 1;

/* ----------------------------------------------------------------------------------------------
   C 37: `#include <windows.h>` - the SDK surface this unit uses (P-88 clause 1: no mirror carries
   `<windows.h>`, so the unit declares what it names).  Every constant value below is a
   **placeholder** unless the line names the landed unit that already measured it, and every record
   layout is unmeasured: there is no Windows SDK, CRT or MinGW-w64 on this machine, so the section
   is UNVERIFIED (P-88 clause 2).
   ---------------------------------------------------------------------------------------------- */

/// C: typedef void *HANDLE, *HMODULE, *LPVOID; (the `w32proc.rs` 276-284 reading)
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
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
/// C: typedef unsigned int UINT;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
/// C: typedef BOOL (WINAPI *PHANDLER_ROUTINE) (DWORD); - the callback C 633 installs.
#[cfg (feature = "windowsnt")]
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn (DWORD) -> BOOL>;

/// C: #define MAX_PATH 260   // measured (`src/w32.rs` 316)
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;
/// C: #define TRUE 1 / #define FALSE 0   // measured (`src/w32.rs` 310/313)
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
/// C: #define STD_OUTPUT_HANDLE ((DWORD) -11)   // C 61 reads it through GetStdHandle
#[cfg (feature = "windowsnt")]
pub const STD_OUTPUT_HANDLE: DWORD = 0xFFFFFFF5;      // placeholder, not <winbase.h>'s value
/// C: #define STD_ERROR_HANDLE ((DWORD) -12)   // C 62 reads it through GetStdHandle
#[cfg (feature = "windowsnt")]
pub const STD_ERROR_HANDLE: DWORD = 0xFFFFFFF4;       // placeholder, not <winbase.h>'s value
/// C: #define CTRL_C_EVENT 0   // C 526's first case label
#[cfg (feature = "windowsnt")]
pub const CTRL_C_EVENT: DWORD = 0;                    // placeholder, not <wincon.h>'s value
/// C: #define CTRL_BREAK_EVENT 1   // C 527's second case label
#[cfg (feature = "windowsnt")]
pub const CTRL_BREAK_EVENT: DWORD = 1;                // placeholder, not <wincon.h>'s value
/// C: #define WAIT_OBJECT_0 ((STATUS_WAIT_0) + 0)   // C 537 compares against it
#[cfg (feature = "windowsnt")]
pub const WAIT_OBJECT_0: DWORD = 0x00000000;          // placeholder, not <winbase.h>'s value
/// C: #define INFINITE 0xFFFFFFFF   // C 588's wait
#[cfg (feature = "windowsnt")]
pub const INFINITE: DWORD = 0xFFFFFFFF;               // placeholder, not <winbase.h>'s value
/// C: #define STATUS_CONTROL_C_EXIT ((NTSTATUS) 0xC000013AL)   // C 539's exit code
#[cfg (feature = "windowsnt")]
pub const STATUS_CONTROL_C_EXIT: c_int = 0xC000013Au32 as c_int;  // placeholder, not <ntstatus.h>'s value

/* C 402's `min` is `<windows.h>`'s `<windef.h>` macro, and the port defines it here as its `max`
   twin is defined in `femacs/nt/runemacs.rs` 336-341 (P-88 clause 1).  */
/// C: #define min(a,b) (((a) < (b)) ? (a) : (b))   [<windef.h>, through <windows.h>]
#[cfg (feature = "windowsnt")]
macro_rules! min {
  ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } };
}

/* C 61-62: `#define stdout GetStdHandle (STD_OUTPUT_HANDLE)` / `#define stderr
   GetStdHandle (STD_ERROR_HANDLE)` - two object-like macros of a call, carried as the same-named
   `macro_rules!` (PORTING.md 2.1).  */
/// C: #define stdout GetStdHandle (STD_OUTPUT_HANDLE)
#[cfg (feature = "windowsnt")]
macro_rules! stdout {
  () => { GetStdHandle (STD_OUTPUT_HANDLE) };
}
/// C: #define stderr GetStdHandle (STD_ERROR_HANDLE)
#[cfg (feature = "windowsnt")]
macro_rules! stderr {
  () => { GetStdHandle (STD_ERROR_HANDLE) };
}

/// C: typedef struct _STARTUPINFOA { DWORD cb; LPSTR lpReserved, lpDesktop, lpTitle; DWORD dwX,
/// C:   dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags; WORD
/// C:   wShowWindow, cbReserved2; LPBYTE lpReserved2; HANDLE hStdInput, hStdOutput, hStdError; }
/// C:   STARTUPINFOA;
/// This copy is `femacs/nt/runemacs.rs` 157-184's, the same SDK header and the same ANSI arm; every
/// offset stays `UNVERIFIED` (no Windows SDK/toolchain on this machine).
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
  pub wShowWindow: u16,          /* C: WORD wShowWindow; */
  pub cbReserved2: u16,          /* C: WORD cbReserved2; */
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

/// C 80/81/635/650/657/663/664/673/691/692/918: the `<windows.h>` entry points this unit calls.
/// Each is the SDK's `...A`/`...W` alias pair under `_UNICODE`; C's text writes the alias and the
/// port keeps C's name (the `src/w32.rs` 404-409 reading of the same pair).
/// C: WINBASEAPI BOOL WINAPI WriteFile (HANDLE hFile, LPCVOID lpBuffer, DWORD nNumberOfBytesToWrite, LPDWORD lpNumberOfBytesWritten, LPOVERLAPPED lpOverlapped);
/// C: WINBASEAPI HANDLE WINAPI GetStdHandle (DWORD nStdHandle);
/// C: WINBASEAPI DWORD WINAPI GetCurrentDirectoryA (DWORD nBufferLength, LPSTR lpBuffer);
/// C: WINBASEAPI BOOL WINAPI SetCurrentDirectoryA (LPCSTR lpPathName);
/// C: WINBASEAPI DWORD WINAPI GetModuleFileNameA (HMODULE hModule, LPSTR lpFilename, DWORD nSize);
/// C: WINBASEAPI DWORD WINAPI SearchPathA (LPCSTR lpPath, LPCSTR lpFileName, LPCSTR lpExtension, DWORD nBufferLength, LPSTR lpBuffer, LPSTR *lpFilePart);
/// C: WINBASEAPI DWORD WINAPI GetShortPathNameA (LPCSTR lpszLongPath, LPSTR lpszShortPath, DWORD cchBuffer);
/// C: WINBASEAPI LPTCH WINAPI GetEnvironmentStrings (void);
/// C: WINBASEAPI BOOL WINAPI FreeEnvironmentStringsA (LPSTR penv);
/// C: WINBASEAPI LPSTR WINAPI GetCommandLineA (void);
/// C: WINBASEAPI BOOL WINAPI SetConsoleCtrlHandler (PHANDLER_ROUTINE HandlerRoutine, BOOL Add);
/// C: WINBASEAPI BOOL WINAPI SetConsoleCP (UINT wCodePageID);
/// C: WINBASEAPI BOOL WINAPI SetConsoleOutputCP (UINT wCodePageID);
/// C: WINBASEAPI UINT WINAPI GetACP (void);
/// C: WINBASEAPI BOOL WINAPI TerminateProcess (HANDLE hProcess, UINT uExitCode);
/// C: WINBASEAPI BOOL WINAPI CreateProcessA (LPCSTR lpApplicationName, LPSTR lpCommandLine, LPSECURITY_ATTRIBUTES lpProcessAttributes, LPSECURITY_ATTRIBUTES lpThreadAttributes, BOOL bInheritHandles, DWORD dwCreationFlags, LPVOID lpEnvironment, LPCSTR lpCurrentDirectory, LPSTARTUPINFOA lpStartupInfo, LPPROCESS_INFORMATION lpProcessInformation);
/// C: WINBASEAPI DWORD WINAPI WaitForSingleObject (HANDLE hHandle, DWORD dwMilliseconds);
/// C: WINBASEAPI BOOL WINAPI GetExitCodeProcess (HANDLE hProcess, LPDWORD lpExitCode);
/// C: WINBASEAPI BOOL WINAPI CloseHandle (HANDLE hObject);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn WriteFile (hFile: HANDLE, lpBuffer: *const c_void, nNumberOfBytesToWrite: DWORD,
                    lpNumberOfBytesWritten: *mut DWORD, lpOverlapped: *mut c_void) -> BOOL;
  pub fn GetStdHandle (nStdHandle: DWORD) -> HANDLE;
  pub fn GetCurrentDirectory (nBufferLength: DWORD, lpBuffer: LPSTR) -> DWORD;
  pub fn SetCurrentDirectory (lpPathName: LPCSTR) -> BOOL;
  pub fn GetModuleFileName (hModule: HMODULE, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
  pub fn SearchPath (lpPath: LPCSTR, lpFileName: LPCSTR, lpExtension: LPCSTR,
                     nBufferLength: DWORD, lpBuffer: LPSTR, lpFilePart: *mut LPSTR) -> DWORD;
  pub fn GetShortPathName (lpszLongPath: LPCSTR, lpszShortPath: LPSTR, cchBuffer: DWORD) -> DWORD;
  pub fn GetEnvironmentStrings () -> LPSTR;
  pub fn FreeEnvironmentStrings (penv: LPSTR) -> BOOL;
  pub fn GetCommandLine () -> LPSTR;
  pub fn SetConsoleCtrlHandler (HandlerRoutine: PHANDLER_ROUTINE, Add: BOOL) -> BOOL;
  pub fn SetConsoleCP (wCodePageID: UINT) -> BOOL;
  pub fn SetConsoleOutputCP (wCodePageID: UINT) -> BOOL;
  pub fn GetACP () -> UINT;
  pub fn TerminateProcess (hProcess: HANDLE, uExitCode: UINT) -> BOOL;
  pub fn CreateProcess (lpApplicationName: LPCSTR, lpCommandLine: LPSTR,
                        lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
                        lpThreadAttributes: *mut SECURITY_ATTRIBUTES, bInheritHandles: BOOL,
                        dwCreationFlags: DWORD, lpEnvironment: LPVOID,
                        lpCurrentDirectory: LPCSTR, lpStartupInfo: *mut STARTUPINFO,
                        lpProcessInformation: *mut PROCESS_INFORMATION) -> BOOL;
  pub fn WaitForSingleObject (hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
  pub fn GetExitCodeProcess (hProcess: HANDLE, lpExitCode: *mut DWORD) -> BOOL;
  pub fn CloseHandle (hObject: HANDLE) -> BOOL;
}

/// C 80: `wvsprintf (buf, msg, args)` - the SDK declares this one `WINAPIV` (cdecl), so it sits in
/// its own block, and its third parameter is the CRT's `va_list`, which the port spells
/// `core::ffi::VaList<'_>` (PORTING.md 2.4).
/// C: WINUSERAPI int WINAPIV wvsprintfA (LPSTR, LPCSTR, va_list arglist);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn wvsprintf (param0: LPSTR, param1: LPCSTR, param2: core::ffi::VaList<'_>) -> c_int;
}

/* C 45-51: C's own note that UCRT has a C99-compatible `snprintf` while `_snprintf` is defined
   inline in `<stdio.h>` (a header this file deliberately does not include; the differences between
   the two do not matter here), then `#ifdef _UCRT` / `# define _snprintf snprintf` / `#endif`.
   Both arms are carried (P-86 clause 1); `_UCRT` is undefined in the frozen configuration, so the
   arm live there is C 55's own `extern int _snprintf (char *buffer, size_t count, const char
   *format, ...);` below.  C's `#define` is the port's `use` alias, so the call sites in `main` keep
   C's own `_snprintf` spelling on both arms.  */
/// C: <stdio.h> (UCRT): `int snprintf (char *buffer, size_t count, const char *format, ...);`
#[cfg (all (feature = "windowsnt", feature = "ucrt"))]
unsafe extern "C" {
  pub fn snprintf (buffer: *mut c_char, count: usize, format: *const c_char, ...) -> c_int;
}
#[cfg (all (feature = "windowsnt", feature = "ucrt"))]
use self::snprintf as _snprintf;

/* C 53-55: C's note that it does not want to include `<stdio.h>` "because we are already
   duplicating lots of it here", then the declaration of the one stdio name it calls.  */
/// C: <stdio.h> (MSVC): `int _snprintf (char *buffer, size_t count, const char *format, ...);`
#[cfg (all (feature = "windowsnt", not (feature = "ucrt")))]
unsafe extern "C" {
  pub fn _snprintf (buffer: *mut c_char, count: usize, format: *const c_char, ...) -> c_int;
}

/* C 39: `#include <stdarg.h>` (`va_args`) - the port's equivalent is `core::ffi::VaList`
   (PORTING.md 2.4: `va_start`/`va_end` have no counterpart, the `...` parameter *is* the list).

   C 40: `#include <malloc.h>` (`alloca`) - no mirror; the only name this unit takes from it is
   `alloca`, which is PORTING.md P-05's stack pool: `sa_stack_alloc` (`femacs/src/alloc.rs` 1039,
   the one deliberate class of port-only exports).  */
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn sa_stack_alloc (size: usize) -> *mut c_void;
}

#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");        // C 41: `#include <stdlib.h>` (getenv below)
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");        // C 42: `#include <string.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/ctype.rs");         // C 43: `#include <ctype.h>` (isspace, isalpha)

/* The `<stdlib.h>`/`<string.h>` names those three mirrors do not carry, declared at their use sites
   (P-88 clause 1, the `nt/runemacs.rs` 289-297 shape).  `stricmp` and `strnicmp` are the MSVC CRT's
   case-insensitive `strcmp`/`strncmp`; `strdup` is the CRT's `char *strdup (const char *s);`; `exit`
   is `<stdlib.h>`'s entry, whose `_Noreturn` is the port's `-> !`.  */
/// C: <stdlib.h>: `int atoi (const char *nptr);`
/// C: <stdlib.h>: `void exit (int status) _Noreturn;`
/// C: <stdlib.h> (POSIX): `char *strdup (const char *s);`
/// C: <string.h>: `char *strcat (char *dest, const char *src);`
/// C: <string.h>: `char *strpbrk (const char *s, const char *accept);`
/// C: <string.h>: `char *strncpy (char *dest, const char *src, size_t n);`
/// C: <string.h> (MSVC): `int stricmp (const char *s1, const char *s2);`
/// C: <string.h> (MSVC): `int strnicmp (const char *s1, const char *s2, size_t n);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn atoi (nptr: *const c_char) -> c_int;
  pub fn exit (status: c_int) -> !;
  pub fn strdup (s: *const c_char) -> *mut c_char;
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn strpbrk (s: *const c_char, accept: *const c_char) -> *mut c_char;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn stricmp (s1: *const c_char, s2: *const c_char) -> c_int;
  pub fn strnicmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

/* C 64-68: `#if __GNUC__ + (__GNUC_MINOR__ >= 4) >= 5` / `void fail (const char *, ...)
   __attribute__((noreturn));` / `#else` / `void fail (const char *, ...);` / `#endif`.  Both arms
   declare the same function and differ only in the noreturn attribute, and C's prototypes are
   dropped here (PORTING.md 0.1-12: Rust has no function prototypes), so the condition has no site
   of its own; its *consequence* is the definition at C 110, which the port spells `-> !` because
   the body's `exit (-1)` is a non-returning exit and PORTING.md's attribute table maps the noreturn
   family to the never type.  The frozen macro table has `__GNUC__` 4 and `__GNUC_MINOR__` 2
   (`docs/baseline/config/darwin-arm64.macros.tsv` 6264/6259), which is the `#else` arm.  C 69-72's
   four declarations (`vfprintf`, `fprintf`, `printf`, `warn`) are dropped the same way; all five
   definitions follow, in C's order.  */

/*******  Mock C library routines  *********************************/

/* These routines are used primarily to minimize the executable size.  */

/// C: C 74-75: `int vfprintf (HANDLE hnd, const char *msg, va_list args)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf (hnd: HANDLE, msg: *const c_char, args: core::ffi::VaList<'_>)
                                   -> c_int
{
  let mut bytes_written: DWORD = 0;    /* C 77: zero-initialised (PORTING.md 1.5) */
  let mut buf: [c_char; 1024] = [0; 1024];

  wvsprintf (buf.as_mut_ptr (), msg, args);
  /* C 81: `WriteFile (hnd, buf, strlen (buf), &bytes_written, NULL)` - C converts the `size_t`
     to the parameter's `DWORD` implicitly, so the port writes the conversion out (P-72 3).  */
  return WriteFile (hnd, buf.as_ptr ().cast (), strlen (buf.as_ptr ()) as DWORD,
                    &mut bytes_written, core::ptr::null_mut ());
}

/// C: C 84-85: `int fprintf (HANDLE hnd, const char *msg, ...)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf (hnd: HANDLE, msg: *const c_char, mut args: ...) -> c_int
{
  /* C's `va_list args; va_start (args, msg);` / `va_end (args);` have no port counterpart
     (PORTING.md 2.4): the `...` parameter *is* the `VaList`.  */
  let rc: c_int;

  rc = vfprintf (hnd, msg, args);

  return rc;
}

/// C: C 97-98: `int printf (const char *msg, ...)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf (msg: *const c_char, mut args: ...) -> c_int
{
  let rc: c_int;

  rc = vfprintf (stdout! (), msg, args);

  return rc;
}

/// C: C 110-111: `void fail (const char *msg, ...)` - `-> !` is C 65's
/// `__attribute__((noreturn))` (see the note above the definitions).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fail (msg: *const c_char, mut args: ...) -> !
{
  vfprintf (stderr! (), msg, args);

  exit (-1);
}

/// C: C 122-123: `void warn (const char *msg, ...)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn warn (msg: *const c_char, mut args: ...)
{
  vfprintf (stderr! (), msg, args);
}

/******************************************************************/

/// C: C 134-135: `static char *canon_filename (char *fname)`
#[cfg (feature = "windowsnt")]
unsafe fn canon_filename (fname: *mut c_char) -> *mut c_char
{
  let mut p: *mut c_char = fname;

  while *p != 0
    {
      if *p == b'/' as c_char
	{ *p = b'\\' as c_char; }
      p = p.add (1);
    }

  return fname;
}

/// C: C 149-150: `static const char *skip_space (const char *str)`
#[cfg (feature = "windowsnt")]
unsafe fn skip_space (mut str: *const c_char) -> *const c_char
{
  while isspace (*str as c_int) != 0 { str = str.add (1); }
  return str;
}

/// C: C 156-157: `static const char *skip_nonspace (const char *str)`
#[cfg (feature = "windowsnt")]
unsafe fn skip_nonspace (mut str: *const c_char) -> *const c_char
{
  while *str != 0 && isspace (*str as c_int) == 0 { str = str.add (1); }
  return str;
}

/* C 163-165: "This value is never changed by the code.  We keep the code that supports also the
   value of '\"', but let's allow the compiler to optimize it out, until someone actually uses
   that."  */
/// C: C 166: `static const int escape_char = '\\';`
#[cfg (feature = "windowsnt")]
const escape_char: c_int = b'\\' as c_int;

/* Get next token from input, advancing pointer.  */
/// C: C 169-170: `static int get_next_token (char *buf, const char **pSrc)`
#[cfg (feature = "windowsnt")]
unsafe fn get_next_token (buf: *mut c_char, pSrc: *mut *const c_char) -> c_int
{
  let mut p: *const c_char = *pSrc;
  let mut o: *mut c_char = buf;

  p = skip_space (p);
  if *p == b'"' as c_char
    {
      let mut escape_char_run: c_int = 0;

      /* Go through src until an ending quote is found, unescaping
	 quotes along the way.  If the escape char is not quote, then do
	 special handling of multiple escape chars preceding a quote
	 char (ie. the reverse of what Emacs does to escape quotes).  */
      p = p.add (1);
      loop
	{
	  if (*p as c_int) == escape_char && escape_char != b'"' as c_int
	    {
	      escape_char_run += 1;
	      p = p.add (1);
	      continue;
	    }
	  else if *p == b'"' as c_char
	    {
	      while escape_char_run > 1
		{
		  *o = escape_char as c_char; o = o.add (1);
		  escape_char_run -= 2;
		}

	      if escape_char_run > 0
		{
		  /* escaped quote */
		  *o = *p; o = o.add (1); p = p.add (1);
		  escape_char_run = 0;
		}
	      else if (*p.offset (1) as c_int) == escape_char && escape_char == b'"' as c_int
		{
		  /* quote escaped by doubling */
		  *o = *p; o = o.add (1);
		  p = p.add (2);
		}
	      else
		{
		  /* The ending quote.  */
		  *o = 0;
		  /* Leave input pointer after token.  */
		  p = p.add (1);
		  break;
		}
	    }
	  else if *p == 0
	    {
	      /* End of string, but no ending quote found.  We might want to
		 flag this as an error, but for now will consider the end as
		 the end of the token.  */
	      if escape_char == b'\\' as c_int
		{
		  /* Output literal backslashes.  Note that if the
		     token ends with an unpaired backslash, we eat it
		     up here.  But since this case invokes undefined
		     behavior anyway, it's okay.  */
		  while escape_char_run > 1
		    {
		      *o = escape_char as c_char; o = o.add (1);
		      escape_char_run -= 2;
		    }
		}
	      *o = 0;
	      break;
	    }
	  else
	    {
	      if escape_char == b'\\' as c_int
		{
		  /* Output literal backslashes.  Note that we don't
		     treat a backslash as an escape character here,
		     since it doesn't precede a quote.  */
		  while escape_char_run > 0
		    {
		      *o = escape_char as c_char; o = o.add (1);
		      escape_char_run -= 1;
		    }
		}
	      *o = *p; o = o.add (1); p = p.add (1);
	    }
	}
    }
  else
    {
      /* Next token is delimited by whitespace.  */
      let p1: *const c_char = skip_nonspace (p);
      memcpy (o.cast (), p.cast (), p1.offset_from (p) as usize);
      o = o.add (p1.offset_from (p) as usize);
      *o = 0;
      p = p1;
    }

  *pSrc = p;

  return o.offset_from (buf) as c_int;
}

/* Return TRUE if PROGNAME is a batch file. */
/// C: C 272-273: `static BOOL batch_file_p (const char *progname)`
#[cfg (feature = "windowsnt")]
unsafe fn batch_file_p (progname: *const c_char) -> BOOL
{
  let exts: [*const c_char; 2] = [c".bat".as_ptr (), c".cmd".as_ptr ()];
  let n_exts: c_int = (core::mem::size_of::<[*const c_char; 2]> ()
		       / core::mem::size_of::<*const c_char> ()) as c_int;
  let mut i: c_int = 0;              /* C 277: assigned by the loop below (PORTING.md 1.5) */

  let ext: *const c_char = strrchr (progname, b'.' as c_int);

  if !ext.is_null ()
    {
      i = 0;
      while i < n_exts
	{
	  if stricmp (ext, exts[i as usize]) == 0
	    { return TRUE; }
	  i += 1;
	}
    }

  return FALSE;
}

/* Search for EXEC file in DIR.  If EXEC does not have an extension,
   DIR is searched for EXEC with the standard extensions appended.  */
/// C: C 295-296: `static int search_dir (const char *dir, const char *exec, int bufsize,
/// C:   char *buffer)`
#[cfg (feature = "windowsnt")]
unsafe fn search_dir (dir: *const c_char, exec: *const c_char, bufsize: c_int,
		      buffer: *mut c_char) -> c_int
{
  let exts: [*const c_char; 4] =
    [c".bat".as_ptr (), c".cmd".as_ptr (), c".exe".as_ptr (), c".com".as_ptr ()];
  let n_exts: c_int = (core::mem::size_of::<[*const c_char; 4]> ()
		       / core::mem::size_of::<*const c_char> ()) as c_int;
  let mut dummy: *mut c_char = core::ptr::null_mut ();   /* C 300: written by SearchPath (1.5) */
  let mut i: c_int = 0;              /* C 301: assigned by the loops below (PORTING.md 1.5) */
  let mut rc: c_int = 0;
  let mut pext: *const c_char = strrchr (exec, b'\\' as c_int);

  /* Does EXEC already include an extension?  */
  if pext.is_null ()
    { pext = exec; }
  pext = strchr (pext, b'.' as c_int);

  /* Search the directory for the program.  */
  if !pext.is_null ()
    {
      /* SearchPath will not append an extension if the file already
	 has an extension, so we must append it ourselves.  */
      let mut exec_ext: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
      let mut p: *mut c_char;

      p = strcpy (exec_ext.as_mut_ptr (), exec).add (strlen (exec));

      /* Search first without any extension; if found, we are done.  */
      rc = SearchPath (dir, exec_ext.as_ptr (), core::ptr::null (), bufsize as DWORD, buffer,
		       &mut dummy) as c_int;
      if rc > 0
	{ return rc; }

      /* Try the known extensions.  */
      i = 0;
      while i < n_exts
	{
	  strcpy (p, exts[i as usize]);
	  rc = SearchPath (dir, exec_ext.as_ptr (), core::ptr::null (), bufsize as DWORD, buffer,
			   &mut dummy) as c_int;
	  if rc > 0
	    { return rc; }
	  i += 1;
	}
    }
  else
    {
      i = 0;
      while i < n_exts
	{
	  rc = SearchPath (dir, exec, exts[i as usize], bufsize as DWORD, buffer, &mut dummy)
	       as c_int;
	  if rc > 0
	    { return rc; }
	  i += 1;
	}
    }

  return 0;
}

/* Return the absolute name of executable file PROG, including
   any file extensions.  If an absolute name for PROG cannot be found,
   return NULL.  */
/// C: C 348-349: `static char *make_absolute (const char *prog)`
#[cfg (feature = "windowsnt")]
unsafe fn make_absolute (prog: *const c_char) -> *mut c_char
{
  let mut absname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut dir: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut curdir: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut p: *mut c_char;            /* C 354: assigned before its first read (PORTING.md 1.5) */
  let mut path: *mut c_char;         /* C 354: assigned before its first read (PORTING.md 1.5) */
  let mut fname: *const c_char;      /* C 355: assigned before its first read (PORTING.md 1.5) */

  /* At least partial absolute path specified; search there.  */
  if (isalpha (*prog as c_int) != 0 && *prog.offset (1) == b':' as c_char)
      || (*prog == b'\\' as c_char)
    {
      /* Split the directory from the filename.  */
      fname = strrchr (prog, b'\\' as c_int);
      if fname.is_null ()
	/* Only a drive specifier is given.  */
	{ fname = prog.add (2); }
      strncpy (dir.as_mut_ptr (), prog, fname.offset_from (prog) as usize);
      *dir.as_mut_ptr ().add (fname.offset_from (prog) as usize) = 0;

      /* Search the directory for the program.  */
      if search_dir (dir.as_ptr (), prog, MAX_PATH, absname.as_mut_ptr ()) > 0
	{ return strdup (absname.as_ptr ()); }
      else
	{ return core::ptr::null_mut (); }
    }

  if GetCurrentDirectory (MAX_PATH as DWORD, curdir.as_mut_ptr ()) == 0
    { return core::ptr::null_mut (); }

  /* Relative path; search in current dir. */
  if !strpbrk (prog, c"\\".as_ptr ()).is_null ()
    {
      if search_dir (curdir.as_ptr (), prog, MAX_PATH, absname.as_mut_ptr ()) > 0
	{ return strdup (absname.as_ptr ()); }
      else
	{ return core::ptr::null_mut (); }
    }

  /* Just filename; search current directory then PATH.  */
  path = sa_stack_alloc (strlen (getenv (c"PATH".as_ptr ())) + strlen (curdir.as_ptr ()) + 2)
	 as *mut c_char;
  strcpy (path, curdir.as_ptr ());
  strcat (path, c";".as_ptr ());
  strcat (path, getenv (c"PATH".as_ptr ()));

  while *path != 0
    {
      let len: usize;

      /* Get next directory from path.  */
      p = path;
      while *p != 0 && *p != b';' as c_char { p = p.add (1); }
      /* A broken PATH could have too long directory names in it.  */
      len = min! (p.offset_from (path) as usize,
		  core::mem::size_of::<[c_char; MAX_PATH as usize]> () - 1);
      strncpy (dir.as_mut_ptr (), path, len);
      *dir.as_mut_ptr ().add (len) = 0;

      /* Search the directory for the program.  */
      if search_dir (dir.as_ptr (), prog, MAX_PATH, absname.as_mut_ptr ()) > 0
	{ return strdup (absname.as_ptr ()); }

      /* Move to the next directory.  */
      path = p.add (1);
    }

  return core::ptr::null_mut ();
}

/* Try to decode the given command line the way cmd would do it.  On
   success, return 1 with cmdline dequoted.  Otherwise, when we've
   found constructs only cmd can properly interpret, return 0 and
   leave cmdline unchanged.  */
/// C: C 421-422: `static int try_dequote_cmdline (char* cmdline)`
#[cfg (feature = "windowsnt")]
unsafe fn try_dequote_cmdline (cmdline: *mut c_char) -> c_int
{
  /* Dequoting can only subtract characters, so the length of the
     original command line is a bound on the amount of scratch space
     we need.  This length, in turn, is bounded by the 32k
     CreateProcess limit.  */
  let mut old_pos: *mut c_char = cmdline;
  let mut new_cmdline: *mut c_char = sa_stack_alloc (strlen (cmdline)) as *mut c_char;
  let mut new_pos: *mut c_char = new_cmdline;
  let mut c: c_char = 0;             /* C 431: assigned by the loop below (PORTING.md 1.5) */

  /* C 433-437: `enum { NORMAL, AFTER_CARET, INSIDE_QUOTE } state = NORMAL;` - C's enum is
     anonymous, so the port names the type (`femacs/src/sort.rs` 659's `enum Label` shape) and
     keeps C's three member names.  */
  enum State { NORMAL, AFTER_CARET, INSIDE_QUOTE }
  let mut state: State = State::NORMAL;

  loop
    {
      c = *old_pos; old_pos = old_pos.add (1);
      if c == 0
	{ break; }
      /* C 441's `switch (state)` and its two inner `switch (c)`es: the character is compared
	 through `as u8`, because C's `case '"'` labels cannot be written as a Rust pattern.  */
      match state
	{
	  State::NORMAL =>
	    {
	      match c as u8
		{
		  b'"' =>
		    {
		      *new_pos = c; new_pos = new_pos.add (1);
		      state = State::INSIDE_QUOTE;
		    }
		  b'^' =>
		    {
		      state = State::AFTER_CARET;
		    }
		  b'<' | b'>' | b'&' | b'|' | b'(' | b')' | b'%' | b'!' =>
		    {
		      /* We saw an unquoted shell metacharacter and we don't
			 understand it. Bail out.  */
		      return 0;
		    }
		  _ =>
		    {
		      *new_pos = c; new_pos = new_pos.add (1);
		    }
		}
	    }
	  State::AFTER_CARET =>
	    {
	      *new_pos = c; new_pos = new_pos.add (1);
	      state = State::NORMAL;
	    }
	  State::INSIDE_QUOTE =>
	    {
	      match c as u8
		{
		  b'"' =>
		    {
		      *new_pos = c; new_pos = new_pos.add (1);
		      state = State::NORMAL;
		    }
		  b'%' | b'!' =>
		    {
		      /* Variable substitution inside quote.  Bail out.  */
		      return 0;
		    }
		  _ =>
		    {
		      *new_pos = c; new_pos = new_pos.add (1);
		    }
		}
	    }
	}
    }

  /* We were able to dequote the entire string.  Copy our scratch
     buffer on top of the original buffer and return success.  */
  memcpy (cmdline.cast (), new_cmdline.cast (), new_pos.offset_from (new_cmdline) as usize);
  *cmdline.add (new_pos.offset_from (new_cmdline) as usize) = 0;
  return 1;
}

/*****************************************************************/

/* C 497-510: `#if 0` - the two file-scope objects `char ** _argv;` / `int _argc;` and the
   `setup_argv` definition between them:

     char ** _argv;
     int     _argc;

     /* Parse commandline into argv array, allowing proper quoting of args.  */
     void
     setup_argv (void)
     {
       char * cmdline = GetCommandLine ();
       int arg_bytes = 0;

     }

   - dead text in every configuration (P-08); `docs/ports/cmdproxy.md` records it.  */

/* Information about child proc is global, to allow for automatic
   termination when interrupted.  At the moment, only one child process
   can be running at any one time.  */

/// C: C 516: `static PROCESS_INFORMATION child;` - C's zero-initialised record
#[cfg (feature = "windowsnt")]
static mut child: PROCESS_INFORMATION =
  PROCESS_INFORMATION { hProcess: core::ptr::null_mut (), hThread: core::ptr::null_mut (),
			dwProcessId: 0, dwThreadId: 0 };
/// C: C 517: `static int interactive = TRUE;`
#[cfg (feature = "windowsnt")]
static mut interactive: c_int = TRUE;

/* C 519's `BOOL console_event_handler (DWORD);` is a forward declaration and is dropped
   (PORTING.md 0.1-12); its definition follows in C's order.  */

/// C: C 521-522: `BOOL console_event_handler (DWORD event)` - `extern "system"`, not `extern "C"`:
/// C's own cast at C 633 (`(PHANDLER_ROUTINE) console_event_handler`) asserts that this function is
/// a `WINAPI` callback, and Rust has no cast into a function-pointer type, so the two must agree
/// (the `nt/addpm.rs` 66-72 reading of the same cast, `## Deviations`).
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn console_event_handler (event: DWORD) -> BOOL
{
  match event
    {
      CTRL_C_EVENT | CTRL_BREAK_EVENT =>
	{
	  if interactive == 0
	    {
	      /* Both command.com and cmd.exe have the annoying behavior of
		 prompting "Terminate batch job (y/n)?" when interrupted
		 while running a batch file, even if running in
		 non-interactive (-c) mode.  Try to make up for this
		 deficiency by forcibly terminating the subprocess if
		 running non-interactively.  */
	      if !child.hProcess.is_null ()
		  && WaitForSingleObject (child.hProcess, 500) != WAIT_OBJECT_0
		{ TerminateProcess (child.hProcess, 0); }
	      exit (STATUS_CONTROL_C_EXIT);
	    }
	}
      /* C 543-550: `#if 0` - the `default:` arm `fail ("cmdproxy: received %d event\n", event);`
	 and the `if (child.hProcess) TerminateProcess (child.hProcess, 0);` below it - is dead
	 text in every configuration (P-08); `docs/ports/cmdproxy.md` records it.  C's `switch`
	 has no `default` either, so Rust's `match` needs the empty arm (the `src/w32console.rs`
	 591-616 reading of the same shape).  */
      _ => {}
    }
  return TRUE;
}

/* Change from normal usage; return value indicates whether spawn
   succeeded or failed - program return code is returned separately.  */
/// C: C 557-558: `static int spawn (const char *progname, char *cmdline, const char *dir,
/// C:   int *retcode)`
#[cfg (feature = "windowsnt")]
unsafe fn spawn (mut progname: *const c_char, cmdline: *mut c_char, dir: *const c_char,
		 retcode: *mut c_int) -> c_int
{
  let mut success: BOOL = FALSE;
  let mut sec_attrs: SECURITY_ATTRIBUTES = core::mem::zeroed ();  /* C 561 (PORTING.md 1.5) */
  let mut start: STARTUPINFO = core::mem::zeroed ();              /* C 562 (PORTING.md 1.5) */
  /* In theory, passing NULL for the environment block to CreateProcess
     is the same as passing the value of GetEnvironmentStrings, but
     doing this explicitly seems to cure problems running DOS programs
     in some cases.  */
  let envblock: LPSTR = GetEnvironmentStrings ();

  sec_attrs.nLength = core::mem::size_of::<SECURITY_ATTRIBUTES> () as DWORD;
  sec_attrs.lpSecurityDescriptor = core::ptr::null_mut ();
  sec_attrs.bInheritHandle = FALSE;

  memset ((&raw mut start).cast (), 0, core::mem::size_of::<STARTUPINFO> ());
  start.cb = core::mem::size_of::<STARTUPINFO> () as DWORD;

  /* CreateProcess handles batch files as progname specially. This
     special handling fails when both the batch file and arguments are
     quoted.  We pass NULL as progname to avoid the special
     handling. */
  if !progname.is_null () && *cmdline == b'"' as c_char && batch_file_p (progname) != 0
      { progname = core::ptr::null (); }

  /* C 584's `envblock` is C's implicit `char *` -> `LPVOID` conversion on the seventh argument
     (P-72 3, the `nt/runemacs.rs` 546 cast).  */
  if CreateProcess (progname, cmdline, &mut sec_attrs, core::ptr::null_mut (), TRUE,
		    0, envblock.cast (), dir, &mut start, &raw mut child) != 0
  {
    success = TRUE;
    /* wait for completion and pass on return code */
    WaitForSingleObject (child.hProcess, INFINITE);
    if !retcode.is_null ()
      { GetExitCodeProcess (child.hProcess, retcode as *mut DWORD); }
    CloseHandle (child.hThread);
    CloseHandle (child.hProcess);
    child.hProcess = core::ptr::null_mut ();
  }

  FreeEnvironmentStrings (envblock);

  return success;
}

/* Return size of current environment block.  */
/// C: C 602-603: `static int get_env_size (void)`
#[cfg (feature = "windowsnt")]
unsafe fn get_env_size () -> c_int
{
  let start: LPSTR = GetEnvironmentStrings ();
  let mut tmp: LPSTR = start;

  while *tmp != 0 || *tmp.offset (1) != 0
    { tmp = tmp.add (1); }
  FreeEnvironmentStrings (start);
  return  tmp.offset (2).offset_from (start) as c_int;
}

/*******  Main program  ********************************************/

/// C: C 616-617: `int main (int argc, char **argv)`
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (mut argc: c_int, mut argv: *mut *mut c_char) -> c_int
{
  let mut rc: c_int = 0;             /* C 619: `spawn` writes it through `&rc` (1.5) */
  let mut need_shell: c_int;         /* C 620: assigned at C 750 and C 958 (PORTING.md 1.5) */
  let mut cmdline: *mut c_char;      /* C 621: assigned at C 748 (PORTING.md 1.5) */
  let mut progname: *mut c_char;     /* C 622: assigned at C 747 (PORTING.md 1.5) */
  let mut envsize: c_int;            /* C 623: assigned at C 754 (PORTING.md 1.5) */
  let mut pass_through_args: *mut *mut c_char;   /* C 624: assigned at C 755 (1.5) */
  let mut num_pass_through_args: c_int;          /* C 625: assigned at C 756 (1.5) */
  let mut modname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut path: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut dir: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];
  let mut status: c_int;             /* C 629: assigned at C 673 (PORTING.md 1.5) */

  interactive = TRUE;

  /* C 633: `SetConsoleCtrlHandler ((PHANDLER_ROUTINE) console_event_handler, TRUE);` - Rust has no
     cast into a function-pointer type, so the cast target is the binding's type (the
     `nt/addpm.rs` 66-72 reading of the same cast).  */
  let handler: PHANDLER_ROUTINE = Some (console_event_handler);
  SetConsoleCtrlHandler (handler, TRUE);

  if GetCurrentDirectory (core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD,
			  dir.as_mut_ptr ()) == 0
    { fail (c"error: GetCurrentDirectory failed\n".as_ptr ()); }

  /* We serve double duty: we can be called either as a proxy for the
     real shell (that is, because we are defined to be the user shell),
     or in our role as a helper application for running DOS programs.
     In the former case, we interpret the command line options as if we
     were a Unix shell, but in the latter case we simply pass our
     command line to CreateProcess.  We know which case we are dealing
     with by whether argv[0] refers to ourself or to some other program.
     (This relies on an arcane feature of CreateProcess, where we can
     specify cmdproxy as the module to run, but specify a different
     program in the command line - the MSVC startup code sets argv[0]
     from the command line.)  */

  if GetModuleFileName (core::ptr::null_mut (), modname.as_mut_ptr (),
			core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD) == 0
    { fail (c"error: GetModuleFileName failed\n".as_ptr ()); }

  /* Change directory to location of .exe so startup directory can be
     deleted.  */
  progname = strrchr (modname.as_ptr (), b'\\' as c_int);
  *progname = 0;
  SetCurrentDirectory (modname.as_ptr ());
  *progname = b'\\' as c_char;

  /* Due to problems with interaction between API functions that use "OEM"
     codepage vs API functions that use the "ANSI" codepage, we need to
     make things consistent by choosing one and sticking with it.  */
  SetConsoleCP (GetACP ());
  SetConsoleOutputCP (GetACP ());

  /* Although Emacs always sets argv[0] to an absolute pathname, we
     might get run in other ways as well, so convert argv[0] to an
     absolute name before comparing to the module name.  */
  path[0] = 0;
  /* The call to SearchPath will find argv[0] in the current
     directory, append ".exe" to it if needed, and also canonicalize
     it, to resolve references to ".", "..", etc.  */
  status = SearchPath (core::ptr::null (), *argv, c".exe".as_ptr (),
		       core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD,
		       path.as_mut_ptr (), &mut progname) as c_int;
  if !(status > 0 && stricmp (modname.as_ptr (), path.as_ptr ()) == 0)
    {
      if status <= 0
	{
	  let mut s: *mut c_char;      /* C 679: assigned before its first read (1.5) */

	  /* Make sure we have argv[0] in path[], as the failed
	     SearchPath might not have copied it there.  */
	  strcpy (path.as_mut_ptr (), *argv);
	  /* argv[0] could include forward slashes; convert them all
	     to backslashes, for strrchr calls below to DTRT.  */
	  s = path.as_mut_ptr ();
	  while *s != 0
	    {
	      if *s == b'/' as c_char
		{ *s = b'\\' as c_char; }
	      s = s.add (1);
	    }
	}
      /* Perhaps MODNAME and PATH use mixed short and long file names.  */
      if !(GetShortPathName (modname.as_ptr (), modname.as_mut_ptr (),
			     core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD) != 0
	   && GetShortPathName (path.as_ptr (), path.as_mut_ptr (),
				core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD) != 0
	   && stricmp (modname.as_ptr (), path.as_ptr ()) == 0)
	{
	  let mut p: *mut c_char = strrchr (path.as_ptr (), b'\\' as c_int);
	  let mut q: *mut c_char = strrchr (modname.as_ptr (), b'\\' as c_int);
	  let mut pdot: *mut c_char;   /* C 704: assigned before its first read (1.5) */
	  let mut qdot: *mut c_char;   /* C 704: assigned before its first read (1.5) */

	  if p.is_null ()
	    { p = strchr (path.as_ptr (), b':' as c_int); }
	  if p.is_null ()
	    { p = path.as_mut_ptr (); }
	  else
	    { p = p.add (1); }
	  if q.is_null ()
	    { q = strchr (modname.as_ptr (), b':' as c_int); }
	  if q.is_null ()
	    { q = modname.as_mut_ptr (); }
	  else
	    { q = q.add (1); }

	  pdot = strrchr (p, b'.' as c_int);
	  if pdot.is_null () || stricmp (pdot, c".exe".as_ptr ()) != 0
	    { pdot = p.add (strlen (p)); }
	  qdot = strrchr (q, b'.' as c_int);
	  if qdot.is_null () || stricmp (qdot, c".exe".as_ptr ()) != 0
	    { qdot = q.add (strlen (q)); }
	  if pdot.offset_from (p) != qdot.offset_from (q)
	      || strnicmp (p, q, pdot.offset_from (p) as usize) != 0
	    {
	      /* We are being used as a helper to run a DOS app; just
		 pass command line to DOS app without change.  */
	      /* TODO: fill in progname.  */
	      if spawn (core::ptr::null (), GetCommandLine (), dir.as_ptr (), &mut rc) != 0
		{ return rc; }
	      fail (c"Could not run %s\n".as_ptr (), GetCommandLine ());
	    }
	}
    }

  /* Process command line.  If running interactively (-c or /c not
     specified) then spawn a real command shell, passing it the command
     line arguments.

     If not running interactively, then attempt to execute the specified
     command directly.  If necessary, spawn a real shell to execute the
     command.

  */

  progname = core::ptr::null_mut ();
  cmdline = core::ptr::null_mut ();
  /* If no args, spawn real shell for interactive use.  */
  need_shell = TRUE;
  interactive = TRUE;
  /* Ask command.com to create an environment block with a reasonable
     amount of free space.  */
  envsize = get_env_size () + 300;
  pass_through_args = sa_stack_alloc ((argc as usize) * core::mem::size_of::<*mut c_char> ())
		      as *mut *mut c_char;
  num_pass_through_args = 0;

  while { argc -= 1; argc > 0 }
    {
      argv = argv.add (1);
      /* Act on switches we recognize (mostly single letter switches,
	 except for -e); all unrecognized switches and extra args are
	 passed on to real shell if used (only really of benefit for
	 interactive use, but allow for batch use as well).  Accept / as
	 switch char for compatibility with cmd.exe.  */
      if (**argv == b'-' as c_char || **argv == b'/' as c_char)
	  && *(*argv).offset (1) != 0
	{
	  if (*(*argv).offset (1) == b'c' as c_char || *(*argv).offset (1) == b'C' as c_char)
	      && *(*argv).offset (2) == 0
	    {
	      argc -= 1;
	      if argc == 0
		{ fail (c"error: expecting arg for %s\n".as_ptr (), *argv); }
	      argv = argv.add (1);
	      cmdline = *argv;
	      interactive = FALSE;
	    }
	  else if (*(*argv).offset (1) == b'i' as c_char || *(*argv).offset (1) == b'I' as c_char)
	      && *(*argv).offset (2) == 0
	    {
	      if !cmdline.is_null ()
		{ warn (c"warning: %s ignored because of -c\n".as_ptr (), *argv); }
	    }
	  else if (*(*argv).offset (1) == b'e' as c_char || *(*argv).offset (1) == b'E' as c_char)
	      && *(*argv).offset (2) == b':' as c_char
	    {
	      let requested_envsize: c_int = atoi ((*argv).offset (3));
	      /* Enforce a reasonable minimum size, as above.  */
	      if requested_envsize > envsize
		{ envsize = requested_envsize; }
	      /* For sanity, enforce a reasonable maximum.  */
	      if envsize > 32768
		{ envsize = 32768; }
	    }
	  else
	    {
	      /* warn ("warning: unknown option %s ignored", *argv); */
	      *pass_through_args.add (num_pass_through_args as usize) = *argv;
	      num_pass_through_args += 1;
	    }
	}
      else
	{ break; }
    }

  /* C 800-809: `#if 0` - the note that cmd.exe ignores extra (non-switch) args in interactive mode
     and that they cannot be passed on when -c was given, then the loop that collects any remaining
     args after (initial) switches:

       while (argc-- > 0)
         {
           pass_through_args[num_pass_through_args++] = *argv++;
         }

     - dead text in every configuration (P-08); `docs/ports/cmdproxy.md` records it.  The `#else`
     arm C 811-813 is the live one and follows.  */
  /* Probably a mistake for there to be extra args; not fatal.  */
  if argc > 0
    { warn (c"warning: extra args ignored after '%s'\n".as_ptr (), *argv.offset (-1)); }

  *pass_through_args.add (num_pass_through_args as usize) = core::ptr::null_mut ();

  /* If -c option, determine if we must spawn a real shell, or if we can
     execute the command directly ourself.  */
  if !cmdline.is_null ()
    {
      let mut args: *const c_char;   /* C 822: assigned on the next line (PORTING.md 1.5) */

      /* The program name is the first token of cmdline.  Since
         filenames cannot legally contain embedded quotes, the value
         of escape_char doesn't matter.  */
      args = cmdline;
      if get_next_token (path.as_mut_ptr (), &mut args) == 0
	{ fail (c"error: no program name specified.\n".as_ptr ()); }

      canon_filename (path.as_mut_ptr ());
      progname = make_absolute (path.as_ptr ());

      /* If we found the program and the rest of the command line does
         not contain unquoted shell metacharacters, run the program
         directly (if not found it might be an internal shell command,
         so don't fail).  */
      if !progname.is_null () && try_dequote_cmdline (cmdline) != 0
	{ need_shell = FALSE; }
      else
	{ progname = core::ptr::null_mut (); }
    }

  /* C 844: `pass_to_shell:` - C's backward `goto` at C 959 re-enters here, so the label is the
     port's `loop` and that `goto` is its `continue` (PORTING.md 2.2).  */
  'pass_to_shell: loop
    {
      if need_shell != 0
	{
	  let mut p: *mut c_char;    /* C 847: assigned before its first read (PORTING.md 1.5) */
	  let mut extra_arg_space: c_int = 0;
	  let mut maxlen: c_int = 0; /* C 849: assigned at C 881/926 (PORTING.md 1.5) */
	  let mut remlen: c_int = 0; /* C 849: assigned at C 881/926 (PORTING.md 1.5) */
	  let mut run_command_dot_com: c_int = 0;   /* C 850: assigned at C 863 (1.5) */

	  progname = getenv (c"COMSPEC".as_ptr ());
	  if progname.is_null ()
	    { fail (c"error: COMSPEC is not set\n".as_ptr ()); }

	  canon_filename (progname);
	  progname = make_absolute (progname);

	  if progname.is_null () || strchr (progname, b'\\' as c_int).is_null ()
	    { fail (c"error: the program %s could not be found.\n".as_ptr (),
		    getenv (c"COMSPEC".as_ptr ())); }

	  /* Need to set environment size when running command.com.  */
	  run_command_dot_com =
	    (stricmp (strrchr (progname, b'\\' as c_int), c"command.com".as_ptr ()) == 0) as c_int;

	  /* Work out how much extra space is required for
	     pass_through_args.  */
	  argv = pass_through_args;
	  while *argv != core::ptr::null_mut ()
	    {
	      /* We don't expect to have to quote switches.  */
	      extra_arg_space += strlen (*argv) as c_int + 2;
	      argv = argv.add (1);
	    }

	  if !cmdline.is_null ()
	    {
	      let mut buf: *mut c_char;   /* C 874: assigned below (PORTING.md 1.5) */

	      /* Convert to syntax expected by cmd.exe/command.com for
		 running non-interactively.  Always quote program name in
		 case path contains spaces (fortunately it can't contain
		 quotes, since they are illegal in path names).  */

	      /* C 881-882's `remlen = maxlen = ...` is split into its two assignments, in C's
		 evaluation order (the port's own deviation note).  */
	      maxlen = strlen (progname) as c_int + extra_arg_space
		       + strlen (cmdline) as c_int + 16 + 2;
	      remlen = maxlen;
	      p = sa_stack_alloc ((maxlen + 1) as usize) as *mut c_char;
	      buf = p;

	      /* Quote progname in case it contains spaces.  */
	      p = p.add (_snprintf (p, remlen as usize, c"\"%s\"".as_ptr (), progname) as usize);
	      remlen = maxlen - (p.offset_from (buf) as c_int);

	      /* Include pass_through_args verbatim; these are just switches
                 so should not need quoting.  */
	      argv = pass_through_args;
	      while *argv != core::ptr::null_mut ()
		{
		  p = p.add (_snprintf (p, remlen as usize, c" %s".as_ptr (), *argv) as usize);
		  remlen = maxlen - (p.offset_from (buf) as c_int);
		  argv = argv.add (1);
		}

	      /* Now that we know we will be invoking the shell, quote the
		 command line after the "/c" switch as the shell expects:
		 a single pair of quotes enclosing the entire command
		 tail, no matter whether quotes are used in the command
		 line, and how many of them are there.  See the output of
		 "cmd /?" for how cmd.exe treats quotes.  */
	      if run_command_dot_com != 0
		{ _snprintf (p, remlen as usize, c" /e:%d /c \"%s\"".as_ptr (), envsize, cmdline); }
	      else
		{ _snprintf (p, remlen as usize, c" /c \"%s\"".as_ptr (), cmdline); }
	      cmdline = buf;
	    }
	  else
	    {
	      if run_command_dot_com != 0
		{
		  /* Provide dir arg expected by command.com when first
		     started interactively (the "command search path").  To
		     avoid potential problems with spaces in command dir
		     (which cannot be quoted - command.com doesn't like it),
		     we always use the 8.3 form.  */
		  GetShortPathName (progname, path.as_mut_ptr (),
				    core::mem::size_of::<[c_char; MAX_PATH as usize]> () as DWORD);
		  p = strrchr (path.as_ptr (), b'\\' as c_int);
		  /* Trailing slash is acceptable, so always leave it.  */
		  p = p.add (1);
		  *p = 0;
		}
	      else
		{ path[0] = 0; }

	      maxlen = strlen (progname) as c_int + extra_arg_space
		       + strlen (path.as_ptr ()) as c_int + 13;
	      remlen = maxlen;
	      p = sa_stack_alloc ((maxlen + 1) as usize) as *mut c_char;
	      cmdline = p;

	      /* Quote progname in case it contains spaces.  */
	      p = p.add (_snprintf (p, remlen as usize, c"\"%s\" %s".as_ptr (), progname,
				    path.as_ptr ()) as usize);
	      remlen = maxlen - (p.offset_from (cmdline) as c_int);

	      /* Include pass_through_args verbatim; these are just switches
                 so should not need quoting.  */
	      argv = pass_through_args;
	      while *argv != core::ptr::null_mut ()
		{
		  p = p.add (_snprintf (p, remlen as usize, c" %s".as_ptr (), *argv) as usize);
		  remlen = maxlen - (p.offset_from (cmdline) as c_int);
		  argv = argv.add (1);
		}

	      if run_command_dot_com != 0
		{ _snprintf (p, remlen as usize, c" /e:%d".as_ptr (), envsize); }
	    }
	}

      if progname.is_null ()
	{ fail (c"Internal error: program name not defined\n".as_ptr ()); }

      if cmdline.is_null ()
	{ cmdline = progname; }

      if spawn (progname, cmdline, dir.as_ptr (), &mut rc) != 0
	{ return rc; }

      if need_shell == 0
	{
	  need_shell = TRUE;
	  continue 'pass_to_shell;
	}

      fail (c"Could not run %s\n".as_ptr (), progname);

      return 0;
    }
}
