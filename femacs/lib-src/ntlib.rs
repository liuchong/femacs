// femacs: port of lib-src/ntlib.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); C's two
// `static` helpers (`convert_time`, `is_exec`) stay plain `unsafe fn`.
//
// **This is one of P-86's class: the frozen configuration compiles nothing from this file.**  The
// gate is the build's own, not a `#if` in the text: `lib-src/Makefile.in` 460 has the `ntlib.o`
// rule and every `lib-src` program links `$(NTLIB)`, while `configure.ac` 3068 sets
// `NTLIB="ntlib.$ac_objext"` **only** in the non-Cygwin arm of the `HAVE_W32` split (3037-3068) -
// i.e. the MinGW build, whose `nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`.  This target is darwin:
// `docs/baseline/frozen-build-sources.txt` records no `lib-src/ntlib.c` (its batch-F section
// derives the directory's units from `INSTALLABLES`/`UTILITIES`/`DONT_INSTALL` and names this file
// among those that appear in no list at all), `tools/extract-abi.py`'s checklist for it is empty
// (P-86 ②), and the port's frozen object is the 336-byte empty one (Evidence ①).  The **file gate
// is `feature = "windowsnt"`** - the registered name (`docs/baseline/feature-coverage.tsv` 8) - on
// every item *and* on the include graph (P-86 ⑧).
//
// **`text-only`** (P-86 ③⑵): this machine has no Windows SDK and no mingw toolchain
// (`which x86_64-w64-mingw32-gcc` is empty, `find` finds no `windows.h`/`winnt.h`), so nothing here
// is compiled against a Windows target.  What the round *does* show is that the arm the `windowsnt`
// gate selects type-checks under the pinned `rustc` (Evidence ②) - the reading `src/systhread.rs`
// (status 3.498) and `src/haiku.rs` (3.499) also take - and every width, offset and constant this
// file does not read from a landed `femacs` unit is marked UNVERIFIED.
//
// The file is the Windows support programs' shadow layer: the ten Unix routines MSVC's CRT does not
// provide, `stat`/`lstat`/`fstat` over `nt/inc/sys/stat.h`'s own `struct stat`, the `sys_*` entries
// `nt/inc/ms-w32.h` renames, and a `CODESET`-only `nl_langinfo`.
//
// Four decisions the C text does not make for us (each is a `## Deviations` entry in
// `docs/ports/ntlib.md`):
//
//   * C 23's `#define DEFER_MS_W32_H` before `<config.h>` makes this one of the few units whose
//     `conf_post.h` does **not** include `nt/inc/ms-w32.h` (`src/conf_post.h` 37).  No part of
//     `ms-w32.h` is therefore in this translation unit's graph, which is why the file defines
//     `MAXPATHLEN` itself at C 64 and why the CRT's own names (`_chdir`, `_open`, `_unlink`,
//     `stricmp`, …) are declared in the unit (P-88 ①) rather than taken from that mirror.
//   * C 44's `#include "ntlib.h"` is consumed through a module plus a glob import.  The mirror
//     keeps its `unsafe extern "C"` items at its own top level while this unit *defines* all ten of
//     them, which is the P-39 `E0428` class; the wrapper is the P-21/P-28 shadowing mechanism C's
//     "prototype plus definition" needs (`femacs/src/hbfont.rs` 622-628 applied from the consumer),
//     and `## Owed` 1 carries the move-back into the mirror.
//   * `<sys/types.h>` (C 30) reaches gnulib's generated `lib/sys/types.h` in this build, and its
//     mirror declares `uid_t`/`gid_t`, which C 44's `<pwd.h>` (`nt/inc/pwd.h` 18-19) declares too;
//     one Rust translation unit cannot hold both (`E0428`), so that mirror is consumed through a
//     module and the two names this unit reads are imported from it.
//   * `<sys/stat.h>` (C 33) is the `nt/inc` override - this file's own C 287-288 comment says so -
//     so the unit includes `nt/inc/sys/include/stat.rs` and *not* the frozen target's
//     `lib/include/sys_stat.in.rs`: both declare `struct stat`, `stat`/`lstat`/`fstat` and the
//     `S_IF*` values, so one unit can hold only one spelling of each.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens, unreachable_patterns, clippy::all)]

use std::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
               c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");                         // C 24: `#include <config.h>`

/* C 5-6: the file's attribution, carried as C's own text at its C position (the
   `femacs/src/w32.rs` 70-76 shape).

Author: Geoff Voelker (voelker@cs.washington.edu)
Created: 10-8-94
*/

/// C 23: `#define DEFER_MS_W32_H` - the marker `nt/inc/ms-w32.h` reads to defer itself, set before
/// `<config.h>` so that `src/conf_post.h` 37 skips that include for this unit.  A `#define` of a
/// name has no Rust counterpart (PORTING.md 0.1), so the switch it stands for is the constant
/// below (the `femacs/src/w32.rs` 77-83 shape).
/// C: #define DEFER_MS_W32_H
#[cfg (feature = "windowsnt")]
pub const DEFER_MS_W32_H: bool = true;

/// C's `long double` is the port's `c_longdouble`, aliased to `c_double` the way
/// `femacs/src/editfns.rs` 197 and `femacs/src/haiku.rs` 474-478 do (PORTING.md's `long double`
/// row: never downgrade silently).  C 257's `utc_base` and C 263's `ret` are its only users.
#[cfg (feature = "windowsnt")]
pub type c_longdouble = c_double;

/* ----------------------------------------------------------------------------------------------
   C 26: `#include <windows.h>`.  The header has no file in the reference tree and no mirror in
   this one (P-88 ①: the declaration site is C's own `#include` line), so the names *this file*
   uses are declared here, each with the SDK's own text above it.  The spellings repeat
   `femacs/nt/runemacs.rs` 66-268 and `femacs/src/w32proc.rs` 226-447 so that a `windowsnt`
   translation unit sees one set of names.

   **No measurement was possible**: no Windows SDK and no mingw toolchain exist on this machine
   (the acquisition note in the header), so every record's size and offset in this section is
   **UNVERIFIED** and no `size_of`/`offset_of` assertion is written for any of them (`## Owed` 2).
   ---------------------------------------------------------------------------------------------- */

/// C 26's `<windows.h>`: the scalar types and the handle typedefs this file's declarations use.
/// C: typedef void *HANDLE, *LPVOID; typedef char *LPSTR; typedef const char *LPCSTR;
/// C: typedef unsigned short WORD; typedef unsigned long DWORD, *LPDWORD; typedef long LONG;
/// C: typedef unsigned int UINT; typedef int BOOL;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub type LONG = i32;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;

/// C 26's `<windows.h>`: the values C's text uses directly (C 296's `FALSE`, C 196/356's
/// `INVALID_HANDLE_VALUE`, C 138's `GetUserName` result).
/// C: #define TRUE 1 / #define FALSE 0
/// C: #define INVALID_HANDLE_VALUE ((HANDLE) (LONG_PTR) -1)
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

/// C 26's `<windows.h>`: `MAX_PATH`, the length the SDK declares `WIN32_FIND_DATA.cFileName` with
/// (the value is the one `femacs/nt/runemacs.rs` 116 carries for the same constant).
/// C: #define MAX_PATH 260
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;

/// C 26's `<windows.h>`: the two `dwFileAttributes` bits this file tests (C 344, 363, 394, 399) and
/// the two `S_IF*` values it stores through `struct stat` (C 363-364, 445, 453, 458, 468).
/// C: #define FILE_ATTRIBUTE_READONLY 0x00000001
/// C: #define FILE_ATTRIBUTE_DIRECTORY 0x00000010
#[cfg (feature = "windowsnt")]
pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x00000010;

/// C 26's `<windows.h>`: `GetFileType`'s four values and the remote-drive bit C 442 masks off.
/// C: #define FILE_TYPE_UNKNOWN 0x0000 / #define FILE_TYPE_DISK 0x0001
/// C: #define FILE_TYPE_CHAR 0x0002 / #define FILE_TYPE_PIPE 0x0003
/// C: #define FILE_TYPE_REMOTE 0x8000
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_UNKNOWN: DWORD = 0x0000;
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_DISK: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_CHAR: DWORD = 0x0002;
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_PIPE: DWORD = 0x0003;
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_REMOTE: DWORD = 0x8000;

/// C 26's `<windows.h>`: the two standard handles C 193-194 asks for, the two console-mode bits
/// C 208 sets, `OpenProcess`'s one access right (C 107) and `WaitForSingleObject`'s three results
/// (C 119-125).
/// C: #define STD_INPUT_HANDLE ((DWORD) -10) / #define STD_ERROR_HANDLE ((DWORD) -12)
/// C: #define ENABLE_PROCESSED_INPUT 0x0001 / #define ENABLE_LINE_INPUT 0x0002
/// C: #define SYNCHRONIZE 0x00100000L
/// C: #define WAIT_OBJECT_0 0x00000000L / #define WAIT_TIMEOUT 0x00000102L
/// C: #define WAIT_FAILED ((DWORD) 0xFFFFFFFF)
#[cfg (feature = "windowsnt")]
pub const STD_INPUT_HANDLE: DWORD = -10i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const STD_ERROR_HANDLE: DWORD = -12i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
#[cfg (feature = "windowsnt")]
pub const SYNCHRONIZE: DWORD = 0x00100000;
#[cfg (feature = "windowsnt")]
pub const WAIT_OBJECT_0: DWORD = 0x00000000;
#[cfg (feature = "windowsnt")]
pub const WAIT_TIMEOUT: DWORD = 0x00000102;
#[cfg (feature = "windowsnt")]
pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;

/// C 26's `<windows.h>`: the timestamp record C 256/265/268/313 reads (the `w32proc.rs` 332-342
/// spelling, member order included).
/// C: typedef struct _FILETIME { DWORD dwLowDateTime; DWORD dwHighDateTime; } FILETIME;
#[cfg (feature = "windowsnt")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct FILETIME
{
  pub dwLowDateTime: DWORD,
  pub dwHighDateTime: DWORD,
}

/// C 26's `<windows.h>`: the calendar record C 302-310 fills before `SystemTimeToFileTime`.
/// C: typedef struct _SYSTEMTIME { WORD wYear, wMonth, wDayOfWeek, wDay, wHour, wMinute, wSecond,
/// C:   wMilliseconds; } SYSTEMTIME;
#[cfg (feature = "windowsnt")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SYSTEMTIME
{
  pub wYear: WORD,
  pub wMonth: WORD,
  pub wDayOfWeek: WORD,
  pub wDay: WORD,
  pub wHour: WORD,
  pub wMinute: WORD,
  pub wSecond: WORD,
  pub wMilliseconds: WORD,
}

/// C 26's `<windows.h>`: the find record C 292/343-348/355/374-386 reads and fills.  C 355's
/// `FindFirstFile` is the SDK's `...A`/`...W` alias pair under `_UNICODE` and C's `char`-typed
/// `cFileName` shows this build is the ANSI one, so the port keeps C's alias name and the ANSI
/// record (the `femacs/nt/runemacs.rs` 214-238 reading).
/// C: typedef struct _WIN32_FIND_DATAA { DWORD dwFileAttributes; FILETIME ftCreationTime;
/// C:   FILETIME ftLastAccessTime; FILETIME ftLastWriteTime; DWORD nFileSizeHigh, nFileSizeLow,
/// C:   dwReserved0, dwReserved1; CHAR cFileName[MAX_PATH]; CHAR cAlternateFileName[14];
/// C:   } WIN32_FIND_DATAA;
#[cfg (feature = "windowsnt")]
#[repr (C)]
#[derive (Clone, Copy)]
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

/// C 26's `<windows.h>`: the handle-information record C 419/446/460-464 reads.
/// C: typedef struct _BY_HANDLE_FILE_INFORMATION { DWORD dwFileAttributes; FILETIME ftCreationTime,
/// C:   ftLastAccessTime, ftLastWriteTime; DWORD dwVolumeSerialNumber, nFileSizeHigh, nFileSizeLow,
/// C:   nNumberOfLinks, nFileIndexHigh, nFileIndexLow; } BY_HANDLE_FILE_INFORMATION;
#[cfg (feature = "windowsnt")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct BY_HANDLE_FILE_INFORMATION
{
  pub dwFileAttributes: DWORD,
  pub ftCreationTime: FILETIME,
  pub ftLastAccessTime: FILETIME,
  pub ftLastWriteTime: FILETIME,
  pub dwVolumeSerialNumber: DWORD,
  pub nFileSizeHigh: DWORD,
  pub nFileSizeLow: DWORD,
  pub nNumberOfLinks: DWORD,
  pub nFileIndexHigh: DWORD,
  pub nFileIndexLow: DWORD,
}

/// C 26's `<windows.h>`: the fourteen entry points this file calls.  Each `WINBASEAPI` spelling is
/// the SDK's, `WINAPI` is Rust's `extern "system"` (PORTING.md §5.1), and the `...A` aliases keep
/// C's own name where C writes the alias (C 355's `FindFirstFile`, C 138's `GetUserName`, C 80's
/// `GetCurrentDirectory`, C 338's `GetDriveType`).
/// C: WINBASEAPI void WINAPI Sleep (DWORD dwMilliseconds);
/// C: WINBASEAPI DWORD WINAPI GetCurrentDirectoryA (DWORD nBufferLength, LPSTR lpBuffer);
/// C: WINBASEAPI UINT WINAPI GetDriveTypeA (LPCSTR lpRootPathName);
/// C: WINBASEAPI HANDLE WINAPI FindFirstFileA (LPCSTR lpFileName, LPWIN32_FIND_DATAA lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindClose (HANDLE hFindFile);
/// C: WINBASEAPI HANDLE WINAPI OpenProcess (DWORD dwDesiredAccess, BOOL bInheritHandle, DWORD dwProcessId);
/// C: WINBASEAPI DWORD WINAPI GetLastError (void);
/// C: WINBASEAPI DWORD WINAPI WaitForSingleObject (HANDLE hHandle, DWORD dwMilliseconds);
/// C: WINBASEAPI BOOL WINAPI GetUserNameA (LPSTR lpBuffer, LPDWORD pcbBuffer);
/// C: WINBASEAPI HANDLE WINAPI GetStdHandle (DWORD nStdHandle);
/// C: WINBASEAPI DWORD WINAPI GetFileType (HANDLE hFile);
/// C: WINBASEAPI BOOL WINAPI GetConsoleMode (HANDLE hConsoleHandle, LPDWORD lpMode);
/// C: WINBASEAPI BOOL WINAPI SetConsoleMode (HANDLE hConsoleHandle, DWORD dwMode);
/// C: WINBASEAPI BOOL WINAPI WriteFile (HANDLE hFile, LPCVOID lpBuffer, DWORD nNumberOfBytesToWrite,
/// C:   LPDWORD lpNumberOfBytesWritten, LPOVERLAPPED lpOverlapped);
/// C: WINBASEAPI BOOL WINAPI ReadFile (HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToRead,
/// C:   LPDWORD lpNumberOfBytesRead, LPOVERLAPPED lpOverlapped);
/// C: WINBASEAPI BOOL WINAPI GetFileInformationByHandle (HANDLE hFile,
/// C:   LPBY_HANDLE_FILE_INFORMATION lpFileInformation);
/// C: WINBASEAPI LONG WINAPI CompareFileTime (const FILETIME *, const FILETIME *);
/// C: WINBASEAPI BOOL WINAPI SystemTimeToFileTime (const SYSTEMTIME *, LPFILETIME);
/// C: WINBASEAPI UINT WINAPI GetACP (void);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn Sleep (dwMilliseconds: DWORD);
  pub fn GetCurrentDirectory (nBufferLength: DWORD, lpBuffer: LPSTR) -> DWORD;
  pub fn GetDriveType (lpRootPathName: LPCSTR) -> UINT;
  pub fn FindFirstFile (lpFileName: LPCSTR, lpFindFileData: *mut WIN32_FIND_DATA) -> HANDLE;
  pub fn FindClose (hFindFile: HANDLE) -> BOOL;
  pub fn OpenProcess (dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
  pub fn GetLastError () -> DWORD;
  pub fn WaitForSingleObject (hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
  pub fn GetUserName (lpBuffer: LPSTR, pcbBuffer: LPDWORD) -> BOOL;
  pub fn GetStdHandle (nStdHandle: DWORD) -> HANDLE;
  pub fn GetFileType (hFile: HANDLE) -> DWORD;
  pub fn GetConsoleMode (hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
  pub fn SetConsoleMode (hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
  pub fn WriteFile (hFile: HANDLE, lpBuffer: *const c_void, nNumberOfBytesToWrite: DWORD,
                    lpNumberOfBytesWritten: LPDWORD, lpOverlapped: *mut c_void) -> BOOL;
  pub fn ReadFile (hFile: HANDLE, lpBuffer: *mut c_void, nNumberOfBytesToRead: DWORD,
                   lpNumberOfBytesRead: LPDWORD, lpOverlapped: *mut c_void) -> BOOL;
  pub fn GetFileInformationByHandle (hFile: HANDLE,
                                     lpFileInformation: *mut BY_HANDLE_FILE_INFORMATION) -> BOOL;
  pub fn CompareFileTime (lpFileTime1: *const FILETIME, lpFileTime2: *const FILETIME) -> LONG;
  pub fn SystemTimeToFileTime (lpSystemTime: *const SYSTEMTIME,
                               lpFileTime: *mut FILETIME) -> BOOL;
  pub fn GetACP () -> UINT;
}

/* C 26's `<windows.h>` reaches `<string.h>` through `<winnt.h>`, which is where this file's
   `strlen`/`strcpy`/`strchr`/`strrchr`/`memcpy`/`memmove`/`memset` come from (P-72 rule 4: the
   edge a header chain carries is named at the header that carries it).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");            // C 26's `<windows.h>` chain: `<string.h>`

/* C 27-42: the header graph.  Every edge carries the file gate (P-86 ⑧).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");            // C 27: `#include <stdlib.h>` (`getenv`; MSVC's
                                                 //   `_MAX_PATH` and `atoi`/`exit` are below)
/* C 28: `#include <stdarg.h>` - C's `va_list`/`va_start`/`va_arg`/`va_end` have no port
   counterpart: the `...` parameter *is* the list (PORTING.md P-04, the `femacs/lib/open.rs`
   44-46 reading).  C 529-538's `sys_open` is this file's one variadic site.  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.in.rs");          // C 29: `#include <stdio.h>` (ntlib.h 23's own
                                                 //   copy of the line): `FILE`, `fopen`, `printf`,
                                                 //   `sprintf`, `rename`, `FILENAME_MAX`
/* C 30: `#include <sys/types.h>` - in this build gnulib's generated `lib/sys/types.h` (from
   `lib/sys_types.in.h`), which `#include_next`s MinGW's own.  The mirror declares `uid_t`/`gid_t`,
   and C 44's `<pwd.h>` (`nt/inc/pwd.h` 18-19) declares them as well; one Rust translation unit
   cannot hold two declarations of one name (`E0428`), so the mirror is consumed through a module
   and the two names this file reads are imported from it.  */
#[cfg (feature = "windowsnt")]
mod sys_types
{
  use super::*;
  include!("../lib/include/sys_types.in.rs");
}
#[cfg (feature = "windowsnt")]
use sys_types::{mode_t, dev_t};
#[cfg (feature = "windowsnt")]
include!("../lib/include/time.in.rs");           // C 31: `#include <time.h>` (`time_t`, and the
                                                 //   `struct timespec` the stat mirror declares
                                                 //   `utimensat` with)
/* C 32: `#include <direct.h>` - the CRT's own header (no mirror): `_chdir`/`_mkdir`/`_getdrive`
   are declared at their use sites below (P-88 ①).  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/stat.rs");       // C 33: `#include <sys/stat.h>` - the `nt/inc`
                                                 //   override (C 287-288's comment says so), whose
                                                 //   `struct stat` C 289-407 and C 415-513 write
#[cfg (feature = "windowsnt")]
include!("../lib/include/math.rs");              // C 34: `#include <math.h>` (`fmodl` is declared
                                                 //   below - the mirror carries `fmod` only)
#[cfg (feature = "windowsnt")]
include!("../lib/include/errno.rs");             // C 35: `#include <errno.h>` (`*errno ()`,
                                                 //   `EFAULT`/`ENOENT`/`EEXIST`/`EACCES`)
#[cfg (feature = "windowsnt")]
include!("../lib/include/ctype.rs");             // C 36: `#include <ctype.h>` (`tolower`)
/* C 37: `#include <sys/timeb.h>` - the CRT's own header, and this file names nothing from it.  */
/* C 38: `#include <mbstring.h>` - the CRT's own header (`_mbspbrk` is declared below).  */
/* C 39: `#include <locale.h>` - the CRT's own header (`setlocale`/`LC_CTYPE` are below).  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/nl_types.rs");       // C 41: `#include <nl_types.h>` (`nl_item`)
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/langinfo.rs");       // C 42: `#include <langinfo.h>` (`CODESET`, and
                                                 //   the `nl_langinfo` prototype C 542 defines; the
                                                 //   mirror includes `nl_types.rs` itself)

/* C 44: `#include "ntlib.h"` - consumed as C consumes it, but through a module plus a glob import:
   that mirror still keeps its `unsafe extern "C"` items at its own top level while this unit
   *defines* all ten of them (C 69-236), the P-39 `E0428` class.  The module is the P-21/P-28
   shadowing mechanism ("a same-named definition shadows a glob import"), so the mirror's prototypes
   and this unit's definitions coexist exactly as C's do; `## Owed` 1 moves the module into the
   mirror and deletes this wrapper.  See `## Deviations` 2.  */
#[cfg (feature = "windowsnt")]
mod ntlib_decls
{
  use super::*;
  include!("include/ntlib.rs");                  // C 44: `#include "ntlib.h"`
}
#[cfg (feature = "windowsnt")]
use ntlib_decls::*;

/* ntlib.h 18-19's own includes, taken at that header's line 44: `<pwd.h>` (whose `struct passwd`
   C 179's `getpwuid` returns, and which also declares `uid_t`/`gid_t` and the three lookups this
   file defines) and `<malloc.h>` (whose one name, `alloca`, is P-05's stack pool).  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/pwd.rs");            // ntlib.h 18: `#include <pwd.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/alloca.in.rs");         // ntlib.h 19: `#include <malloc.h>` (P-05)

/* ----------------------------------------------------------------------------------------------
   The CRT's own names.  C 23's `DEFER_MS_W32_H` keeps `nt/inc/ms-w32.h` out of this unit's graph
   (`src/conf_post.h` 37), so no mirror carries them: each is declared at the `#include` line it
   comes from (P-88 ①), and the widths the CRT's headers would give them are **UNVERIFIED** here
   (no MSVC headers on this machine - `## Owed` 2).
   ---------------------------------------------------------------------------------------------- */

/// C 32's `<direct.h>`: the three entries C 247, 253 and 371 call.
/// C: <direct.h>: `int _chdir (const char *);`
/// C: <direct.h>: `int _mkdir (const char *);`
/// C: <direct.h>: `int _getdrive (void);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _chdir (path: *const c_char) -> c_int;
  pub fn _mkdir (path: *const c_char) -> c_int;
  pub fn _getdrive () -> c_int;
}

/// ntlib.h 22's `<io.h>` and its own rename table: C 523's `unlink (to)` is the `unlink -> _unlink`
/// `#define` of ntlib.h 66-67, and C 535's `_open` is spelled with its underscore already.  C 418's
/// `_get_osfhandle` is the same header's (the `femacs/src/sysdep.rs` 1063 spelling).
/// C: <io.h>: `int _open (const char *filename, int oflag, int pmode);`
/// C: <io.h>: `intptr_t _get_osfhandle (int fd);`
/// C: <io.h>: `int _unlink (const char *filename);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _open (path: *const c_char, oflag: c_int, mode: c_int) -> c_int;
  pub fn _get_osfhandle (fd: c_int) -> isize;
  pub fn _unlink (path: *const c_char) -> c_int;
}

/// C 38's `<mbstring.h>`: the wildcard scan C 323 runs over the path.  C passes a `char *` where the
/// header declares `const unsigned char *`, which is the implicit conversion the port spells.
/// C: <mbstring.h>: `unsigned char *_mbspbrk (const unsigned char *, const unsigned char *);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _mbspbrk (string: *const c_uchar, charset: *const c_uchar) -> *mut c_uchar;
}

/// C 39's `<locale.h>`: the category C 552 asks for and the entry point it calls (the
/// `femacs/src/emacs.rs` 341 spelling).  The MSVCRT header's own value for `LC_CTYPE` is
/// **UNVERIFIED** here (`## Owed` 2).
/// C: <locale.h>: `char *setlocale (int, const char *);`
/// C: <locale.h>: `#define LC_CTYPE 2`
#[cfg (feature = "windowsnt")]
pub const LC_CTYPE: c_int = 2;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn setlocale (category: c_int, locale: *const c_char) -> *mut c_char;
}

/// C 27's `<stdlib.h>`: `atoi` (C 102, 107) and `exit` (C 112, 128).  The mirror carries neither
/// (`femacs/lib/include/stdlib.rs` has `getenv`, `malloc`/`free`/`strtol`/… only), and
/// `femacs/src/w32proc.rs` 176-190 declares `exit` the same way.
/// C: <stdlib.h>: `int atoi (const char *);`
/// C: <stdlib.h>: `_Noreturn void exit (int);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn atoi (s: *const c_char) -> c_int;
  pub fn exit (status: c_int) -> !;
}

/// C 34's `<math.h>`: `fmodl` (C 271).  The mirror carries `fmod` only
/// (`femacs/lib/include/math.rs` 55), and C's `long double` is the port's `c_longdouble`.
/// C: <math.h>: `long double fmodl (long double, long double);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn fmodl (x: c_longdouble, y: c_longdouble) -> c_longdouble;
}

/// ntlib.h 19's `<malloc.h>`: C 297's `alloca (FILENAME_MAX)` is P-05's stack pool, whose entry
/// `femacs/src/alloc.rs` 1039 defines and `femacs/lib-src/pop.rs` 326-336 declares this way.
/// C: `#define alloca __builtin_alloca` (lib/alloca.in.h 47)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn sa_stack_alloc (size: usize) -> *mut c_void;
}

/// C 26's `<windows.h>` (MSVC's `<stdlib.h>`): `_MAX_PATH`, which C 64's `MAXPATHLEN` expands to.
/// C: <stdlib.h> (MSVC): `#define _MAX_PATH 260`
#[cfg (feature = "windowsnt")]
pub const _MAX_PATH: c_int = 260;

/// C 30's `<sys/types.h>` (MSVCRT's own half, which gnulib's generated `lib/sys/types.h` reaches
/// with `#include_next`): the four `_`-prefixed aliases C 33's `<sys/stat.h>` declares its two
/// MSVCRT records with.  No mirror carries them - `femacs/nt/inc/sys/include/stat.rs`'s own head
/// note says the including unit names them (P-87 ③) - and their widths are **UNVERIFIED** here
/// (`## Owed` 2); `_ino_t` is the one with a reading in the reference tree, `src/w32.c` 5871's
/// "MSVC defines _ino_t to be short; other libc's might not".
/// C: <sys/types.h> (MSVC): `typedef unsigned int _dev_t;`
/// C: <sys/types.h> (MSVC): `typedef unsigned short _ino_t; typedef unsigned short _mode_t;`
/// C: <sys/types.h> (MSVC): `typedef long _off_t;`
#[cfg (feature = "windowsnt")]
pub type _dev_t = c_uint;
#[cfg (feature = "windowsnt")]
pub type _ino_t = c_ushort;
#[cfg (feature = "windowsnt")]
pub type _mode_t = c_ushort;
#[cfg (feature = "windowsnt")]
pub type _off_t = c_long;

/* C 46-52: the seven forward declarations (`sys_ctime`, `sys_fopen`, `sys_mkdir`, `sys_chdir`,
   `mkostemp`, `sys_rename`, `sys_open`) - dropped (PORTING.md 0.1-12: C's forward declaration has
   no Rust counterpart).  Five of them are the definitions below; `sys_ctime` is `src/w32.c` 3280's
   and `mkostemp` gnulib's, and this file calls neither.  */

/* C 54-62: `#ifndef _TIMEZONE_DEFINED` / `struct timezone` / `#endif` - the file's only `#if`.
   The comment above it says which side MinGW-w64 takes (it defines the marker and `struct
   timespec` in its own system headers), so both arms are carried: the record below is the arm a
   toolchain without that marker selects.  */
/// C: struct timezone { int tz_minuteswest; int tz_dsttime; };
#[cfg (all (feature = "windowsnt", not (feature = "_timezone_defined")))]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct timezone
{
  pub tz_minuteswest: c_int,                    /* minutes west of Greenwich */
  pub tz_dsttime: c_int,                        /* type of dst correction */
}

/// C 64: `#define MAXPATHLEN _MAX_PATH` - the `nt/inc/ms-w32.h` 151-152 pair is *not* in this
/// unit's graph (C 23 deferred that header), so this file's own definition is the only one.
/// C: #define MAXPATHLEN _MAX_PATH
#[cfg (feature = "windowsnt")]
pub const MAXPATHLEN: c_int = _MAX_PATH;

/// C 334/335/352's `IS_DIRECTORY_SEP`: this build gets it from the generated `<config.h>`, whose
/// `mingw32` arm tests **both** separators (`configure.ac` 6493) - unlike the `DIRECTORY_SEP`
/// comparison every other platform takes (`configure.ac` 6486 defines `DIRECTORY_SEP` as `'/'`
/// everywhere).  A `#define` of a name has no Rust counterpart (PORTING.md 0.1), so the port
/// carries the macro; the comparison is between `char`s as C's `(_c_) == '/'` is.
/// C: configure.ac 6493: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == '/' || (_c_) == '\\')`
#[cfg (feature = "windowsnt")]
macro_rules! IS_DIRECTORY_SEP {
  ($c:expr) => { ($c) == b'/' as c_char || ($c) == b'\\' as c_char };
}

/* C 66-68: "Emulate sleep...we could have done this with a define, but that would necessitate
   including windows.h in the files that used it.  This is much easier."  */
/// C 69-70: `unsigned sleep (unsigned seconds)`.
/// C: unsigned sleep (unsigned seconds);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sleep (seconds: c_uint) -> c_uint
{
  Sleep (seconds * 1000);
  return 0;
}

/* C 76: "Get the current working directory."  */
/// C 77-78: `char *getwd (char *dir)`.
/// C: char *getwd (char *dir);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getwd (dir: *mut c_char) -> *mut c_char
{
  if GetCurrentDirectory (MAXPATHLEN as DWORD, dir) > 0
    { return dir; }
  return core::ptr::null_mut ();
}

/// C 85-86: the two file-scope objects `getppid` keeps its handle and its answer in.
/// C: static HANDLE getppid_parent; static int getppid_ppid;
#[cfg (feature = "windowsnt")]
static mut getppid_parent: HANDLE = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
static mut getppid_ppid: c_int = 0;

/// C 88-89: `int getppid (void)`.
/// C: int getppid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getppid () -> c_int
{
  let mut ppid: *mut c_char;
  let mut result: DWORD;

  ppid = getenv (c"EM_PARENT_PROCESS_ID".as_ptr ());
  if ppid.is_null ()
    {
      printf (c"no pid.\n".as_ptr ());
      return 0;
    }
  else
    {
      getppid_ppid = atoi (ppid);
    }

  if getppid_parent.is_null ()
    {
      getppid_parent = OpenProcess (SYNCHRONIZE, FALSE, atoi (ppid) as DWORD);
      if getppid_parent.is_null ()
	{
	  printf (c"Failed to open handle to parent process: %lu\n".as_ptr (),
		  GetLastError ());
	  exit (1);
	}
    }

  result = WaitForSingleObject (getppid_parent, 0);
  match result
    {
      WAIT_TIMEOUT =>
	{
	  /* The parent is still alive.  */
	  return getppid_ppid;
	}
      WAIT_OBJECT_0 =>
	{
	  /* The parent is gone.  Return the pid of Unix init (1).  */
	  return 1;
	}
      WAIT_FAILED | _ =>
	{
	  printf (c"Checking parent status failed: %lu\n".as_ptr (), GetLastError ());
	  exit (1);
	}
    }
}

/// C 132-133: `char * getlogin (void)`.
/// C: char * getlogin (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getlogin () -> *mut c_char
{
  static mut user_name: [c_char; 256] = [0; 256];
  let mut length: DWORD = core::mem::size_of::<[c_char; 256]> () as DWORD;

  if GetUserName (user_name.as_mut_ptr (), &mut length) != 0
    { return user_name.as_mut_ptr (); }
  return core::ptr::null_mut ();
}

/// C 143-144: `unsigned getuid (void)`.
/// C: unsigned getuid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getuid () -> c_uint
{
  return 0;
}

/// C 149-150: `unsigned geteuid (void)`.
/// C: unsigned geteuid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn geteuid () -> c_uint
{
  return getuid ();
}

/// C 155-156: `unsigned getgid (void)`.
/// C: unsigned getgid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getgid () -> c_uint
{
  return 0;
}

/// C 161-162: `unsigned getegid (void)`.
/// C: unsigned getegid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getegid () -> c_uint
{
  return 0;
}

/// C 167-168: `int setuid (unsigned uid)`.
/// C: int setuid (unsigned uid);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setuid (uid: c_uint) -> c_int
{
  return 0;
}

/// C 173-174: `int setregid (unsigned rgid, unsigned gid)`.
/// C: int setregid (unsigned rgid, unsigned gid);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setregid (rgid: c_uint, gid: c_uint) -> c_int
{
  return 0;
}

/// C 179-180: `struct passwd * getpwuid (unsigned uid)`.
/// C: struct passwd *getpwuid (unsigned uid);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpwuid (uid: c_uint) -> *mut passwd
{
  return core::ptr::null_mut ();
}

/// C 185-186: `char * getpass (const char * prompt)`.
/// C: char * getpass (const char * prompt);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpass (prompt: *const c_char) -> *mut c_char
{
  static mut input: [c_char; 256] = [0; 256];
  let mut r#in: HANDLE = core::ptr::null_mut ();
  let mut r#err: HANDLE = core::ptr::null_mut ();
  let mut count: DWORD = 0;

  r#in = GetStdHandle (STD_INPUT_HANDLE);
  r#err = GetStdHandle (STD_ERROR_HANDLE);

  if r#in == INVALID_HANDLE_VALUE || r#err == INVALID_HANDLE_VALUE
    { return core::ptr::null_mut (); }

  if WriteFile (r#err, prompt as *const c_void, strlen (prompt) as DWORD, &mut count,
		core::ptr::null_mut ()) != 0
    {
      let mut istty: c_int = (GetFileType (r#in) == FILE_TYPE_CHAR) as c_int;
      let mut old_flags: DWORD = 0;
      let mut rc: c_int;

      if istty != 0
	{
	  if GetConsoleMode (r#in, &mut old_flags) != 0
	    { SetConsoleMode (r#in, ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT); }
	  else
	    { istty = 0; }
	}
      rc = ReadFile (r#in, input.as_mut_ptr () as *mut c_void,
		     core::mem::size_of::<[c_char; 256]> () as DWORD, &mut count,
		     core::ptr::null_mut ());
      if count >= 2 && input[(count - 2) as usize] == b'\r' as c_char
	{ input[(count - 2) as usize] = b'\0' as c_char; }
      else
	{
	  let mut buf: [c_char; 256] = [0; 256];
	  while ReadFile (r#in, buf.as_mut_ptr () as *mut c_void,
			  core::mem::size_of::<[c_char; 256]> () as DWORD, &mut count,
			  core::ptr::null_mut ()) > 0
	    {
	      if count >= 2 && buf[(count - 2) as usize] == b'\r' as c_char
		{ break; }
	    }
	}
      WriteFile (r#err, c"\r\n".as_ptr () as *const c_void, 2, &mut count,
		 core::ptr::null_mut ());
      if istty != 0
	{ SetConsoleMode (r#in, old_flags); }
      if rc != 0
	{ return input.as_mut_ptr (); }
    }

  return core::ptr::null_mut ();
}

/// C 232-233: `int fchown (int fd, unsigned uid, unsigned gid)`.
/// C: int fchown (int fd, unsigned uid, unsigned gid);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fchown (fd: c_int, uid: c_uint, gid: c_uint) -> c_int
{
  return 0;
}

/// C 238-239: `FILE * sys_fopen (const char * path, const char * mode)`.
/// C: FILE *sys_fopen (const char *, const char *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_fopen (path: *const c_char, mode: *const c_char) -> *mut FILE
{
  return fopen (path, mode);
}

/// C 244-245: `int sys_chdir (const char * path)`.
/// C: int sys_chdir (const char *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_chdir (path: *const c_char) -> c_int
{
  return _chdir (path);
}

/// C 250-251: `int sys_mkdir (const char * path, mode_t mode)`.
/// C: int sys_mkdir (const char *, mode_t);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mkdir (path: *const c_char, mode: mode_t) -> c_int
{
  return _mkdir (path);
}

/// C 256-258: the three file-scope objects the two timestamp readers share.
/// C: static FILETIME utc_base_ft; static long double utc_base; static int init = 0;
#[cfg (feature = "windowsnt")]
static mut utc_base_ft: FILETIME = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
#[cfg (feature = "windowsnt")]
static mut utc_base: c_longdouble = 0.0;
#[cfg (feature = "windowsnt")]
static mut init: c_int = 0;

/// C 260-261: `static time_t convert_time (FILETIME ft, int *time_nsec)`.
/// C: static time_t convert_time (FILETIME ft, int *time_nsec);
#[cfg (feature = "windowsnt")]
unsafe fn convert_time (ft: FILETIME, time_nsec: *mut c_int) -> time_t
{
  let mut ret: c_longdouble;

  if CompareFileTime (&ft, &utc_base_ft) < 0
    { return 0; }

  ret = ft.dwHighDateTime as c_longdouble * 4096.0 * 1024.0 * 1024.0
    + ft.dwLowDateTime as c_longdouble;
  ret -= utc_base;
  *time_nsec = fmodl (ret, 1.0e7) as c_int * 100;
  return (ret * 1.0e-7) as time_t;
}

/// C 281-284's `stricmp`: C reaches `<string.h>` through C 26's `<windows.h>` -> `<winnt.h>`, and
/// the mirror that carries `strlen`/`strcpy`/`memcpy`/… does not carry this one (it is the MSVC
/// CRT's name for a case-insensitive `strcmp`, the `femacs/nt/runemacs.rs` 259-265 reading).
/// C: <string.h> (MSVC): `int stricmp (const char *s1, const char *s2);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn stricmp (s1: *const c_char, s2: *const c_char) -> c_int;
}

/// C 275-276: `static int is_exec (const char * name)`.
/// C: static int is_exec (const char * name);
#[cfg (feature = "windowsnt")]
unsafe fn is_exec (name: *const c_char) -> c_int
{
  let p: *mut c_char = strrchr (name, b'.' as c_int);
  return (p != core::ptr::null_mut ()
	  && (stricmp (p, c".exe".as_ptr ()) == 0
	      || stricmp (p, c".com".as_ptr ()) == 0
	      || stricmp (p, c".bat".as_ptr ()) == 0
	      || stricmp (p, c".cmd".as_ptr ()) == 0)) as c_int;
}

/* C 287-288: "We need stat/fsfat below because nt/inc/sys/stat.h defines struct stat that is
   incompatible with the MS run-time libraries."  */
/// C 289-290: `int stat (const char * path, struct stat * buf)`.
/// C: int stat (const char * path, struct stat * buf);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stat (path: *const c_char, buf: *mut stat) -> c_int
{
  let mut wfd: WIN32_FIND_DATA = core::mem::zeroed ();
  let mut fh: HANDLE;
  let mut permission: c_int;
  let mut len: c_int;
  let mut rootdir: c_int = FALSE;
  let name: *mut c_char = sa_stack_alloc (FILENAME_MAX as usize) as *mut c_char;

  if init == 0
    {
      /* Determine the delta between 1-Jan-1601 and 1-Jan-1970. */
      let mut st: SYSTEMTIME = core::mem::zeroed ();

      st.wYear = 1970;
      st.wMonth = 1;
      st.wDay = 1;
      st.wHour = 0;
      st.wMinute = 0;
      st.wSecond = 0;
      st.wMilliseconds = 0;

      SystemTimeToFileTime (&st, &mut utc_base_ft);
      utc_base = utc_base_ft.dwHighDateTime as c_longdouble * 4096.0 * 1024.0 * 1024.0
	+ utc_base_ft.dwLowDateTime as c_longdouble;
      init = 1;
    }

  if path.is_null () || buf.is_null () || *path == b'\0' as c_char
    {
      *errno () = EFAULT;
      return -1;
    }
  if ! _mbspbrk (path as *const c_uchar, c"*?|<>\"" .as_ptr () as *const c_uchar).is_null ()
    {
      *errno () = ENOENT;
      return -1;
    }

  strcpy (name, path);
  /* Remove trailing directory separator, unless name is the root
     directory of a drive in which case ensure there is a trailing
     separator. */
  len = strlen (name) as c_int;
  rootdir = (IS_DIRECTORY_SEP! (*name)
	     || (len == 3 && *name.offset (1) == b':' as c_char
		 && IS_DIRECTORY_SEP! (*name.offset (2)))) as c_int;
  if rootdir != 0
    {
      if GetDriveType (name) < 2
	{
	  *errno () = ENOENT;
	  return -1;
	}
      memset (&mut wfd as *mut WIN32_FIND_DATA as *mut c_void, 0,
	      core::mem::size_of::<WIN32_FIND_DATA> ());
      wfd.dwFileAttributes = FILE_ATTRIBUTE_DIRECTORY;
      wfd.ftCreationTime = utc_base_ft;
      wfd.ftLastAccessTime = utc_base_ft;
      wfd.ftLastWriteTime = utc_base_ft;
      strcpy (wfd.cFileName.as_mut_ptr (), name);
    }
  else
    {
      if IS_DIRECTORY_SEP! (*name.offset (len as isize - 1))
	{ *name.offset (len as isize - 1) = 0; }

      fh = FindFirstFile (name, &mut wfd);
      if fh == INVALID_HANDLE_VALUE
	{
	  *errno () = ENOENT;
	  return -1;
	}
      FindClose (fh);
    }
  (*buf).st_mode = if (wfd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0
    { S_IFDIR as c_ushort } else { S_IFREG as c_ushort };
  (*buf).st_nlink = 1;
  (*buf).st_ino = 0;

  if *name != 0 && *name.offset (1) == b':' as c_char
    { (*buf).st_dev = tolower (*name as c_int) - b'a' as c_int + 1; }
  else
    { (*buf).st_dev = _getdrive (); }
  (*buf).st_rdev = (*buf).st_dev;

  (*buf).st_size = wfd.nFileSizeHigh as c_ulonglong;
  (*buf).st_size <<= 32;
  (*buf).st_size += wfd.nFileSizeLow as c_ulonglong;

  /* Convert timestamps to Unix format. */
  (*buf).st_mtime = convert_time (wfd.ftLastWriteTime, &mut (*buf).st_mtimensec);
  (*buf).st_atime = convert_time (wfd.ftLastAccessTime, &mut (*buf).st_atimensec);
  if (*buf).st_atime == 0
    {
      (*buf).st_atime = (*buf).st_mtime;
      (*buf).st_atimensec = (*buf).st_mtimensec;
    }
  (*buf).st_ctime = convert_time (wfd.ftCreationTime, &mut (*buf).st_ctimensec);
  if (*buf).st_ctime == 0
    {
      (*buf).st_ctime = (*buf).st_mtime;
      (*buf).st_ctimensec = (*buf).st_mtimensec;
    }

  /* determine rwx permissions */
  if (wfd.dwFileAttributes & FILE_ATTRIBUTE_READONLY) != 0
    { permission = S_IREAD; }
  else
    { permission = S_IREAD | S_IWRITE; }

  if (wfd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0
    { permission |= S_IEXEC; }
  else if is_exec (name) != 0
    { permission |= S_IEXEC; }

  (*buf).st_mode |= (permission | (permission >> 3) | (permission >> 6)) as c_ushort;

  return 0;
}

/// C 409-410: `int lstat (const char * path, struct stat * buf)`.
/// C: int lstat (const char * path, struct stat * buf);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lstat (path: *const c_char, buf: *mut stat) -> c_int
{
  return stat (path, buf);
}

/// C 415-416: `int fstat (int desc, struct stat * buf)`.
/// C: int fstat (int desc, struct stat * buf);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fstat (desc: c_int, buf: *mut stat) -> c_int
{
  let fh: HANDLE = _get_osfhandle (desc) as HANDLE;
  let mut info: BY_HANDLE_FILE_INFORMATION = core::mem::zeroed ();
  let mut fake_inode: c_ulonglong;
  let mut permission: c_int;
  let mut non_disk: c_int = 0;

  if init == 0
    {
      /* Determine the delta between 1-Jan-1601 and 1-Jan-1970. */
      let mut st: SYSTEMTIME = core::mem::zeroed ();

      st.wYear = 1970;
      st.wMonth = 1;
      st.wDay = 1;
      st.wHour = 0;
      st.wMinute = 0;
      st.wSecond = 0;
      st.wMilliseconds = 0;

      SystemTimeToFileTime (&st, &mut utc_base_ft);
      utc_base = utc_base_ft.dwHighDateTime as c_longdouble * 4096.0 * 1024.0 * 1024.0
	+ utc_base_ft.dwLowDateTime as c_longdouble;
      init = 1;
    }

  /* C 442-465's `switch` and its `goto non_disk` (C 454): the switch is this `match`, its `break`
     (C 451) is the end of the `FILE_TYPE_DISK` arm and falls through to C 467, and the `non_disk:`
     label's block is the `if non_disk` tail below (`## Deviations` 4).  */
  match GetFileType (fh) & ! FILE_TYPE_REMOTE
    {
      FILE_TYPE_DISK =>
	{
	  (*buf).st_mode = S_IFREG as c_ushort;
	  if GetFileInformationByHandle (fh, &mut info) == 0
	    {
	      *errno () = EACCES;
	      return -1;
	    }
	}
      FILE_TYPE_PIPE =>
	{
	  (*buf).st_mode = S_IFIFO as c_ushort;
	  non_disk = 1;                              /* C 454: `goto non_disk;` */
	}
      FILE_TYPE_CHAR | FILE_TYPE_UNKNOWN | _ =>
	{
	  (*buf).st_mode = S_IFCHR as c_ushort;      /* C 458's shared label block */
	  non_disk = 1;
	}
    }
  if non_disk != 0
    {
      memset (&mut info as *mut BY_HANDLE_FILE_INFORMATION as *mut c_void, 0,
	      core::mem::size_of::<BY_HANDLE_FILE_INFORMATION> ());
      info.dwFileAttributes = 0;
      info.ftCreationTime = utc_base_ft;
      info.ftLastAccessTime = utc_base_ft;
      info.ftLastWriteTime = utc_base_ft;
    }

  if (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0
    { (*buf).st_mode = S_IFDIR as c_ushort; }

  (*buf).st_nlink = info.nNumberOfLinks as c_short;
  /* Might as well use file index to fake inode values, but this
     is not guaranteed to be unique unless we keep a handle open
     all the time. */
  fake_inode = info.nFileIndexHigh as c_ulonglong;
  fake_inode <<= 32;
  fake_inode += info.nFileIndexLow as c_ulonglong;
  (*buf).st_ino = fake_inode;

  (*buf).st_dev = info.dwVolumeSerialNumber as dev_t;
  (*buf).st_rdev = info.dwVolumeSerialNumber as dev_t;

  (*buf).st_size = info.nFileSizeHigh as c_ulonglong;
  (*buf).st_size <<= 32;
  (*buf).st_size += info.nFileSizeLow as c_ulonglong;

  /* Convert timestamps to Unix format. */
  (*buf).st_mtime = convert_time (info.ftLastWriteTime, &mut (*buf).st_mtimensec);
  (*buf).st_atime = convert_time (info.ftLastAccessTime, &mut (*buf).st_atimensec);
  if (*buf).st_atime == 0
    {
      (*buf).st_atime = (*buf).st_mtime;
      (*buf).st_atimensec = (*buf).st_mtimensec;
    }
  (*buf).st_ctime = convert_time (info.ftCreationTime, &mut (*buf).st_ctimensec);
  if (*buf).st_ctime == 0
    {
      (*buf).st_ctime = (*buf).st_mtime;
      (*buf).st_ctimensec = (*buf).st_mtimensec;
    }

  /* determine rwx permissions */
  if (info.dwFileAttributes & FILE_ATTRIBUTE_READONLY) != 0
    { permission = S_IREAD; }
  else
    { permission = S_IREAD | S_IWRITE; }

  if (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0
    { permission |= S_IEXEC; }

  (*buf).st_mode |= (permission | (permission >> 3) | (permission >> 6)) as c_ushort;

  return 0;
}

/* C 515: "On Windows, you cannot rename into an existing file."  */
/// C 516-517: `int sys_rename (const char *from, const char *to)`.
/// C: int sys_rename (const char *, const char *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_rename (from: *const c_char, to: *const c_char) -> c_int
{
  let mut retval: c_int = rename (from, to);

  if retval < 0 && *errno () == EEXIST
    {
      /* C's `unlink` is ntlib.h 66-67's `#define unlink _unlink` (`## Deviations` 5).  */
      if _unlink (to) == 0
	{ retval = rename (from, to); }
    }
  return retval;
}

/// C 529-530: `int sys_open (const char * path, int oflag, ...)`.  C's `va_list ap; va_start (ap,
/// oflag); … va_end (ap);` has no port counterpart: the `...` parameter *is* the list, and
/// `va_arg (ap, int)` is `next_arg::<c_int> ()` (PORTING.md P-04, the `femacs/lib/open.rs` 44-46
/// reading).
/// C: int sys_open (const char *, int, ...);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_open (path: *const c_char, oflag: c_int, mut ap: ...) -> c_int
{
  let mode: c_int = ap.next_arg::<c_int> ();
  let retval: c_int = _open (path, oflag, mode);
  return retval;
}

/* C 540-541: "Emulation of nl_langinfo that supports only CODESET.  Used in Gnulib regex.c."  */
/// C 542-543: `char * nl_langinfo (nl_item item)`.
/// C: char *nl_langinfo (nl_item item);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nl_langinfo (item: nl_item) -> *mut c_char
{
  match item
    {
      CODESET =>
	{
	  /* Shamelessly stolen from Gnulib's nl_langinfo.c, modulo CPP directives.  */
	  static mut buf: [c_char; 2 + 10 + 1] = [0; 2 + 10 + 1];
	  let locale: *const c_char = setlocale (LC_CTYPE, core::ptr::null ());
	  let mut codeset: *mut c_char = buf.as_mut_ptr ();
	  let mut codesetlen: usize;
	  *codeset = b'\0' as c_char;

	  if ! locale.is_null () && *locale != 0
	    {
	      /* If the locale name contains an encoding after the dot, return it.  */
	      let dot: *mut c_char = strchr (locale, b'.' as c_int);

	      if ! dot.is_null ()
		{
		  /* Look for the possible @... trailer and remove it, if any.  */
		  let codeset_start: *mut c_char = dot.offset (1);
		  let modifier: *const c_char = strchr (codeset_start, b'@' as c_int);

		  if modifier.is_null ()
		    { codeset = codeset_start; }
		  else
		    {
		      codesetlen = modifier.offset_from (codeset_start) as usize;
		      if codesetlen < core::mem::size_of::<[c_char; 2 + 10 + 1]> ()
			{
			  codeset = memcpy (buf.as_mut_ptr () as *mut c_void,
					    codeset_start as *const c_void,
					    codesetlen) as *mut c_char;
			  *codeset.add (codesetlen) = b'\0' as c_char;
			}
		    }
		}
	    }
	  /* If setlocale is successful, it returns the number of the
	     codepage, as a string.  Otherwise, fall back on Windows
	     API GetACP, which returns the locale's codepage as a
	     number (although this doesn't change according to what
	     the 'setlocale' call specified).  Either way, prepend
	     "CP" to make it a valid codeset name.  */
	  codesetlen = strlen (codeset);
	  if 0 < codesetlen && codesetlen < core::mem::size_of::<[c_char; 2 + 10 + 1]> () - 2
	    { memmove (buf.as_mut_ptr ().add (2) as *mut c_void, codeset as *const c_void,
		       codesetlen + 1); }
	  else
	    { sprintf (buf.as_mut_ptr ().add (2), c"%u".as_ptr (), GetACP ()); }
	  codeset = memcpy (buf.as_mut_ptr () as *mut c_void, c"CP".as_ptr () as *const c_void,
			    2) as *mut c_char;

	  return codeset;
	}
      _ =>
	{ return c"".as_ptr () as *mut c_char; }
    }
}
