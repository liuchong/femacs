// femacs: port of src/w32.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**
// `w32.o` sits in `W32_OBJ` (`emacs/src/Makefile.in` 482, `W32_OBJ=@W32_OBJ@`, filled by
// `configure.ac` 3051 - the `else` arm of the `HAVE_W32` test's Cygwin/non-Cygwin split, i.e. the
// MinGW build, whose `<ms-w32.h>` is what `#define`s `WINDOWSNT`), and
// `docs/baseline/frozen-build-sources.txt` lists no `w32.o` for this target - so `extract-abi.py`'s
// checklist for the file is empty (P-86 ②) and the ledger `docs/ports/w32.md` carries that
// clause's substitute: the **C line-range table**, the `## Evidence` readings and the
// `## Deviations`.  This batch is P-86 ④'s first slice: **C 1-1197**, ending at the `}` of
// `get_system_times` (C 1177-1197); **the next live definition is `create_symbolic_link`
// (C 1199-1284)**.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`, not
//     `HAVE_NTGUI`: a Cygwin build with `--with-w32` compiles `w32fns.o`/`w32term.o` etc.
//     (`configure.ac` 3030-3031, both arms of the split) but takes the Cygwin branch at 3050, so
//     `w32.o` is *not* in its `W32_OBJ` - this file's build condition is the MinGW one;
//   * the file's own `#if`s are the derived conditions this slice carries with both arms:
//     C 140 `_WIN32_WINNT < 0x0500` (`win32_winnt_lt_0500`) with C 141's
//     `__MINGW32__`/`__W32API_MAJOR_VERSION` test (`mingw32_w32api_lt_3_15`), C 174 `MINGW_W64`
//     (`mingw_w64`), C 248/C 257 `_WIN32_WINNT < 0x0501` + `ORIG_WIN32_WINNT`
//     (`win32_winnt_lt_0501`), C 385 `_UNICODE` (`unicode`) and C 552's
//     `_WIN32_WINNT < 0x0501` + `__W32API_MAJOR_VERSION > 5 …` (`win32_winnt_lt_0501`,
//     `mingw32_w32api_ge_5_2`); the `#ifndef SDDL_REVISION_1`/`FILE_DEVICE_FILE_SYSTEM`/… blocks
//     (C 170, 206-222) are "define it if the SDK has not" guards and carry C's value with no `cfg`
//     (the `w32term.rs` reading of the same shape);
//   * the `<windows.h>`/SDK section below stands where C 101's `#include <windows.h>` and C 134-163
//     name the headers: they have no file in the reference tree (P-88 ①), so the names this slice
//     uses are declared here, and **no `windows.h` exists on this machine** (see the acquisition
//     note) - every record's size is `UNVERIFIED` and the layout assertions are owed.
//
// **Acquisition note - this batch is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h` anywhere), so there is no
// `<windows.h>` to compile or measure against, **no arm flip on a Windows target and no layout
// measurement**: the readings the ledger carries are the frozen `build-tu`, the machine-checked
// coverage table, the `#if` verdicts, and an **arm-reach pre-flight** over the unit's own text
// (with `/tmp` stand-ins for the mirrors' known gaps and two mutation tests) - which is a `/tmp`
// harness reading, **not** a Windows-target compile.
//
// `docs/ports/w32.md` carries the C line ranges, the acceptance readings, every deviation (the
// `<windows.h>` section, the `#undef` blocks, the dropped forward declarations, the four names
// this slice defines that `include/w32.rs` also declares, the two `ltime`/`sys_*` references whose
// definitions are past this slice) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 19-49: the include graph.  Every header sits inside the file gate, as C's own build has it (no
   translation unit the frozen configuration builds reaches this file; P-86 ⑧).  The headers the
   tree has no file for (`<stdarg.h>`, `<io.h>`, `<mbstring.h>`, `<sys/utime.h>`, `<windows.h>` and
   the SDK headers of C 134-163, `<epaths.h>`) are recorded at their C lines with a `// C …` line
   (P-88 ①): the names this slice takes from them stand in the `<windows.h>`/SDK section below.
   Where the reference tree carries *two* headers of one name, the one `nt/inc` overrides is the
   one included - that is what `<dirent.h>`/`<grp.h>`/`<pwd.h>`/`<signal.h>`/`<sys/acl.h>`/
   `<sys/file.h>`/`<sys/socket.h>`/`<sys/time.h>`/`<mingw_time.h>`/`<ms-w32.h>` resolve to in the
   MinGW build, and the `nt/inc/include/` mirrors' own notes name `src/w32.c` as one of the
   units that reaches them.  */
/* C 20-22: the file's attribution comment, carried as C's own text at its C position.

/*
   Geoff Voelker (voelker@cs.washington.edu)                         7-29-94
*/
*/

/// C 24: `#define DEFER_MS_W32_H` - the marker `nt/inc/ms-w32.h` reads to defer itself.  A Rust
/// `macro_rules!` is not seen before its definition, and the marker names a compile-time switch
/// rather than a call, so the port carries it as the boolean constant that switch stands for (the
/// `nt/inc/include/ms-w32.rs` `WINDOWSNT`/`MINGW_W64` reading).
/// C: #define DEFER_MS_W32_H
#[cfg (feature = "windowsnt")]
pub const DEFER_MS_W32_H: bool = true;
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/mingw_time.rs");     // C 27: `#include <mingw_time.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/stddef.in.rs");         // C 28: `#include <stddef.h>` /* for offsetof */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");            // C 29: `#include <stdlib.h>`
// C 30: `#include <stdarg.h>` - the runtime's own header; this slice names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");             // C 31: `#include <stdio.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/float.in.rs");          // C 32: `#include <float.h>` /* for DBL_EPSILON */
// C 33: `#include <io.h>` - the runtime's own header; this slice names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../lib/include/errno.rs");             // C 34: `#include <errno.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/fcntl.in.rs");          // C 35: `#include <fcntl.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/signal.rs");         // C 36: `#include <signal.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/file.rs");       // C 37: `#include <sys/file.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/sys_types.in.rs");      // C 38: `#include <sys/types.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/time.in.rs");           // C 39: `#include <time.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/time.rs");       // C 40: `#include <sys/time.h>`
// C 41: `#include <sys/utime.h>` - the runtime's own header; this slice names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../lib/include/math.rs");              // C 42: `#include <math.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/nproc.rs");             // C 43: `#include <nproc.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/ms-w32.rs");         // C 46: `#include <ms-w32.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");            // C 48: `#include <string.h>` /* for strerror */
// C 49: `#include <mbstring.h>` - the runtime's own header (`_mbspbrk`, `_mbslwr`, `_mbsrchr`);
// this slice names nothing from it.

/* C 51-75: the `#undef access` … `#undef clock` block.  It cancels the `#define access sys_access`
   … renames `nt/inc/ms-w32.h` installs above, so that this file's own definitions of those names
   are not rewritten.  Rust has no object-like rename to cancel and the ms-w32 mirror spells those
   renames as `macro_rules!` (bound at call sites, not at definitions), so the block is carried as
   C's own text (the `cm.rs` 86 reading).

#undef access
#undef chdir
#undef chmod
#undef creat
#undef ctime
#undef fopen
#undef link
#undef mkdir
#undef open
#undef rename
#undef rmdir
#undef unlink

#undef close
#undef dup
#undef dup2
#undef pipe
#undef read
#undef write

#undef strerror

#undef localtime

#undef clock
*/

/* C 77-89: the thirteen prototypes of the `sys_*` emulations.  Each is a *declaration* of a
   function this file defines later in the same translation unit (C 4348 `sys_access`, C 8824
   `sys_dup`, …), so it has no Rust item (PORTING.md 0.1-12); the definitions stand at C's
   positions and the names C spells here are the `#[unsafe(no_mangle)]` names there.
   C: char *sys_ctime (const time_t *);
   C: int sys_chdir (const char *);
   C: int sys_creat (const char *, int);
   C: FILE *sys_fopen (const char *, const char *);
   C: int sys_open (const char *, int, ...);
   C: int sys_rename (char const *, char const *);
   C: int sys_rmdir (const char *);
   C: int sys_close (int);
   C: int sys_dup2 (int, int);
   C: int sys_read (int, char *, unsigned int);
   C: int sys_write (int, const void *, unsigned int);
   C: struct tm *sys_localtime (const time_t *);
   C: clock_t sys_clock (void);  */

/* C 91-93: `#ifdef HAVE_MODULES` / `extern void dynlib_reset_last_error (void);`.  The macro is
   `undef` in the frozen table and this slice's live text needs nothing from it; the two arms are
   carried, the arm's declaration with the feature that names the other side (PORTING.md P-51).  */
/// C: extern void dynlib_reset_last_error (void);
#[cfg (all (feature = "windowsnt", feature = "have_modules"))]
unsafe extern "C" {
  pub fn dynlib_reset_last_error ();
}

#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                     // C 95: `#include "lisp.h"`
// C 96: `#include "epaths.h"` /* for PATH_EXEC */ - a generated header (from `src/epaths.in`) with
// no file in the reference tree and no mirror; this slice names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/pwd.rs");            // C 98: `#include <pwd.h>`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/grp.rs");            // C 99: `#include <grp.h>`
// C 101: `#include <windows.h>` - the header the section below stands for.

/* ----------------------------------------------------------------------------------------------
   `<windows.h>` and the rest of the SDK surface this slice names.

   C 101, 134-163 and 226-263 name `<windows.h>`, `<lmcons.h>`, `<shlobj.h>`, `<tlhelp32.h>`,
   `<psapi.h>`, `<w32api.h>`, `<winioctl.h>`, `<aclapi.h>`, `<sddl.h>`, `<sys/socket.h>`,
   `<iphlpapi.h>`, `<wincrypt.h>` and `<pthread.h>` - none of them has a file in the reference tree
   (P-88 ①: the declaration site is C's own `#include` line, and no file is added under
   `femacs/src/include/`).  What follows is the names *this slice* uses, each with the SDK's own
   text above it.

   **No measurement was possible**: this machine has no `windows.h` at all (see the file note), so
   every scalar is spelled from PORTING.md 1.1's fixed widths (`DWORD`/`LONG`/`ULONG`/`UINT` are
   the 32-bit Windows types whatever this target's `c_long` is, `HANDLE` and the SDK's handle
   records are `void *`), and every record below is **UNVERIFIED** - its layout assertions are owed
   (the ledger's owed list).  Two of the records are the pair `include/w32common.rs` and
   `w32fns.rs` *deferred* (`SYSTEM_INFO`, `OSVERSIONINFO`): this slice embeds both **by value** in
   its own body (C 573's local, C 1174's store), so it cannot be transcribed without them - the
   ledger records the divergence and the reconciliation it owes.
   ---------------------------------------------------------------------------------------------- */

/// C 101's `<windows.h>` names: the scalar types and the C-isms this slice's typedefs use.
/// C: typedef unsigned char BYTE; typedef unsigned short WCHAR; typedef unsigned short USHORT;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned int UINT;
/// C: typedef int BOOL; typedef unsigned char BOOLEAN; typedef unsigned short LANGID;
/// C: typedef unsigned long ULONG; typedef unsigned long long DWORDLONG, ULONGLONG;
/// C: typedef unsigned long long ULONGLONG; typedef LONG HRESULT;
/// C: typedef UINT_PTR SIZE_T; typedef void *HANDLE, *HMODULE, *PVOID, *HKEY, *PSID, *PACL;
#[cfg (feature = "windowsnt")]
pub type BYTE = u8;
#[cfg (feature = "windowsnt")]
pub type WCHAR = c_ushort;
#[cfg (feature = "windowsnt")]
pub type USHORT = c_ushort;
#[cfg (feature = "windowsnt")]
pub type WORD = c_ushort;
#[cfg (feature = "windowsnt")]
pub type UCHAR = c_uchar;
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
#[cfg (feature = "windowsnt")]
pub type LONG = i32;
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type BOOLEAN = c_uchar;
#[cfg (feature = "windowsnt")]
pub type LANGID = c_ushort;
#[cfg (feature = "windowsnt")]
pub type ULONG = u32;
#[cfg (feature = "windowsnt")]
pub type DWORDLONG = u64;
#[cfg (feature = "windowsnt")]
pub type ULONGLONG = u64;
#[cfg (feature = "windowsnt")]
pub type HRESULT = LONG;
#[cfg (feature = "windowsnt")]
pub type SIZE_T = usize;
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HKEY = *mut c_void;
/// `PSID`, `PACL` and — from C 165's `<sys/acl.h>` — `PSECURITY_DESCRIPTOR` are all `void *` at
/// the ABI.  `PSECURITY_DESCRIPTOR` is the one of them a mirror already carries
/// (`nt/inc/sys/include/acl.rs` 27, the P-72 ③ pointer reading), so this unit uses that copy and
/// declares only the two C 376-376/404-411's own text needs.
/// C: typedef void *PSID; typedef void *PACL;
#[cfg (feature = "windowsnt")]
pub type PSID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PACL = *mut c_void;

/// C 101's `<windows.h>`: the pointer typedefs C's declarations below spell.  C's `LP`/`P`
/// prefixes are all pointers to the names above; `LPCTSTR`/`LPTSTR`/`TCHAR` are the ANSI forms
/// (`_UNICODE` is not defined in this configuration, and both arms are carried - C 385).
/// C: typedef DWORD *PDWORD, *LPDWORD; typedef UCHAR *PUCHAR; typedef ULONG *PULONG;
/// C: typedef BYTE *LPBYTE; typedef BOOL *LPBOOL; typedef HANDLE *PHANDLE; typedef HKEY *PHKEY;
/// C: typedef const char *LPCSTR, *LPCTSTR; typedef char *LPSTR, *LPTSTR;
/// C: typedef const WCHAR *LPCWSTR; typedef WCHAR *LPWSTR; typedef void *LPVOID;
#[cfg (feature = "windowsnt")]
pub type PDWORD = *mut DWORD;
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;
#[cfg (feature = "windowsnt")]
pub type PUCHAR = *mut UCHAR;
#[cfg (feature = "windowsnt")]
pub type PULONG = *mut ULONG;
#[cfg (feature = "windowsnt")]
pub type LPBYTE = *mut BYTE;
#[cfg (feature = "windowsnt")]
pub type LPBOOL = *mut BOOL;
#[cfg (feature = "windowsnt")]
pub type PHANDLE = *mut HANDLE;
#[cfg (feature = "windowsnt")]
pub type PHKEY = *mut HKEY;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPCTSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPTSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPCWSTR = *const c_ushort;
#[cfg (feature = "windowsnt")]
pub type LPWSTR = *mut c_ushort;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;

/// C 101's `<windows.h>` constants this slice tests or passes.
/// C: #define TRUE 1
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
/// C: #define FALSE 0
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
/// C: #define MAX_PATH 260
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;
/// C: #define VER_PLATFORM_WIN32_WINDOWS 0x01
#[cfg (feature = "windowsnt")]
pub const VER_PLATFORM_WIN32_WINDOWS: DWORD = 0x01;

/// C 101's `<windows.h>` §the `WINAPI`/`CALLBACK` convention: `__stdcall`, which Rust spells
/// `extern "system"`; the name appears in the `/// C:` lines of the typedefs below that use it.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
/// C: #define ZeroMemory RtlZeroMemory
/// C: #define RtlZeroMemory(Destination,Length) memset((Destination),0,(Length))
#[cfg (feature = "windowsnt")]
macro_rules! ZeroMemory {
  ($dest:expr, $len:expr) => { core::ptr::write_bytes ($dest as *mut u8, 0, $len) };
}

/// C 573/578: `OSVERSIONINFO` and `GetVersionEx` - `<winver.h>`'s record and its one entry point.
/// The record is the ANSI form: `_UNICODE` selects `OSVERSIONINFOW`/`GetVersionExW` in the SDK,
/// and this configuration leaves it undefined (C 385's `#else` arm is the live one).  The
/// record's size (5 DWORDs and a 128-byte ANSI buffer, 148) is `UNVERIFIED` here.
/// C: typedef struct _OSVERSIONINFOA { DWORD dwOSVersionInfoSize; DWORD dwMajorVersion;
/// C:   DWORD dwMinorVersion; DWORD dwBuildNumber; DWORD dwPlatformId;
/// C:   CHAR szCSDVersion[128]; } OSVERSIONINFOA, *POSVERSIONINFOA, *LPOSVERSIONINFOA;
/// C: WINBASEAPI BOOL WINAPI GetVersionExA (LPOSVERSIONINFOA lpVersionInformation);
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
/// C 575: `GetVersionEx (&os_ver)` - the SDK's `GetVersionExA`/`GetVersionExW` alias under
/// `_UNICODE`; C's text writes the alias and the port keeps C's name, so a Windows target
/// supplies whichever entry point its own `#define` picks (the `lstrcmpi` reading, `w32fns.rs`).
/// C: WINBASEAPI BOOL WINAPI GetVersionExA (LPOSVERSIONINFOA lpVersionInformation);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetVersionEx (lpVersionInformation: *mut OSVERSIONINFO) -> BOOL;
}

/// C 599/605/607: `FILETIME`, `GetCurrentProcess` and the `LARGE_INTEGER` arithmetic C 602-606
/// does.  `LARGE_INTEGER` is a union whose two `LowPart`/`HighPart` views live in a *nameless*
/// inner struct in the SDK; Rust has no nameless struct, so the port names it `u` and C's
/// `user_int.LowPart` reads as `user_int.u.LowPart` (a recorded deviation, not a different
/// member).  The record is 8 bytes wide and `UNVERIFIED` here.
/// C: typedef struct _FILETIME { DWORD dwLowDateTime; DWORD dwHighDateTime; } FILETIME, *PFILETIME,
/// C:   *LPFILETIME;
/// C: typedef union _LARGE_INTEGER { struct { DWORD LowPart; LONG HighPart; } u;
/// C:   LONGLONG QuadPart; } LARGE_INTEGER;
/// C: WINBASEAPI HANDLE WINAPI GetCurrentProcess (void);
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
pub struct LARGE_INTEGER_u
{
  pub LowPart: DWORD,
  pub HighPart: LONG,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union LARGE_INTEGER
{
  pub u: LARGE_INTEGER_u,
  pub QuadPart: i64,                       /* C's `LONGLONG` (PORTING.md 1.1's `i64` row) */
}
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetCurrentProcess () -> HANDLE;
}

/// C 785/820/877/936/988/1016/1045/1071/1094/1117/1140/1167/1191: the loader entry points the
/// wrapper functions below call.  `LoadLibrary`/`GetModuleHandle` are the SDK's `…A`/`…W` alias
/// pair under `_UNICODE` (C's text keeps the alias) and `get_proc_addr` is `w32common.h`'s own
/// inline loader, which the mirror carries.
/// C: WINBASEAPI HMODULE WINAPI LoadLibraryA (LPCSTR lpLibFileName);
/// C: WINBASEAPI HMODULE WINAPI GetModuleHandleA (LPCSTR lpModuleName);
/// C: INLINE VOIDFNPTR get_proc_addr (HINSTANCE handle, LPCSTR fname)   [w32common.h 65-70]
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn LoadLibrary (lpLibFileName: LPCSTR) -> HMODULE;
  pub fn GetModuleHandle (lpModuleName: LPCSTR) -> HMODULE;
}

/// C 482: `PPROCESS_MEMORY_COUNTERS ppsmemCounters` - `<psapi.h>`'s record, named only through
/// this pointer.  Every `<psapi.h>`/`<tlhelp32.h>`/`<iphlpapi.h>`/`<aclapi.h>`/`<sddl.h>` record
/// the slice uses through a pointer is therefore spelled as the pointer C's text spells (PORTING.md
/// 0.1-2's incomplete-type rule), with the SDK's own line above it:
/// C: typedef struct _PROCESS_MEMORY_COUNTERS { DWORD cb; DWORD PageFaultCount; …; } PROCESS_MEMORY_COUNTERS, *PPROCESS_MEMORY_COUNTERS;
/// C: typedef struct tagPROCESSENTRY32 { DWORD dwSize; DWORD cntUsage; …; } PROCESSENTRY32, *LPPROCESSENTRY32;
/// C: typedef struct _IP_ADAPTER_INFO { … } IP_ADAPTER_INFO, *PIP_ADAPTER_INFO;
/// C: typedef struct _IP_ADAPTER_ADDRESSES_LH { … } IP_ADAPTER_ADDRESSES, *PIP_ADAPTER_ADDRESSES;
/// C: typedef enum _SE_OBJECT_TYPE { SE_UNKNOWN_OBJECT_TYPE = 0, … } SE_OBJECT_TYPE;
/// C: typedef enum _SID_NAME_USE { SidTypeUser = 1, … } SID_NAME_USE, *PSID_NAME_USE;
/// C: typedef enum _TOKEN_INFORMATION_CLASS { TokenUser = 1, … } TOKEN_INFORMATION_CLASS;
/// C: typedef enum _SECURITY_IMPERSONATION_LEVEL { SecurityAnonymous, … } SECURITY_IMPERSONATION_LEVEL;
#[cfg (feature = "windowsnt")]
pub type PPROCESS_MEMORY_COUNTERS = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPPROCESSENTRY32 = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PIP_ADAPTER_INFO = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PIP_ADAPTER_ADDRESSES = *mut c_void;
/// The five SDK enums C's parameter lists name.  C passes each by value and the SDK makes each an
/// `int`-sized enum, so the port spells them `c_int` (PORTING.md 1.3's shape for an unnamed enum;
/// the `include/w32.rs` reading of `OS_SUBTYPE_*` is the same choice).
#[cfg (feature = "windowsnt")]
pub type SE_OBJECT_TYPE = c_int;
#[cfg (feature = "windowsnt")]
pub type SID_NAME_USE = c_int;
#[cfg (feature = "windowsnt")]
pub type PSID_NAME_USE = *mut SID_NAME_USE;
#[cfg (feature = "windowsnt")]
pub type TOKEN_INFORMATION_CLASS = c_int;
#[cfg (feature = "windowsnt")]
pub type SECURITY_IMPERSONATION_LEVEL = c_int;

/// C 769/915/1120: `SECURITY_INFORMATION`, the `PSID *`/`PACL *` out-parameters and `REGSAM` of
/// C 540-541's two registry typedefs.  The SDK makes `SECURITY_INFORMATION` a `DWORD` bitmask and
/// `REGSAM` a `DWORD`-sized access mask.
/// C: typedef DWORD SECURITY_INFORMATION, *PSECURITY_INFORMATION;
/// C: typedef DWORD REGSAM, *PREGSAM;
#[cfg (feature = "windowsnt")]
pub type SECURITY_INFORMATION = DWORD;
#[cfg (feature = "windowsnt")]
pub type REGSAM = u32;

/// C 290: `TOKEN_PRIVILEGES *` - `<winnt.h>`'s record, named only through that pointer in the
/// prototype of `enable_privilege`/`restore_privilege` (C 291-292, dropped with the forward
/// declarations below).
/// C: typedef struct _TOKEN_PRIVILEGES { DWORD PrivilegeCount; LUID_AND_ATTRIBUTES Privileges[ANYSIZE_ARRAY]; } TOKEN_PRIVILEGES, *PTOKEN_PRIVILEGES;

/// C 1174: `lpSystemInfo->dwNumberOfProcessors` - `<sysinfoapi.h>`'s record, embedded **by
/// value** in the caller's storage (the same record `include/w32common.rs` 35-36 defers; see the
/// section note).  The SDK's first member is a *nameless* union; Rust has no nameless member, so
/// the port names it `u` - the one member this slice reads (`dwNumberOfProcessors`) is outside it,
/// so no C expression changes shape.  The record's size follows `DWORD_PTR = usize`, i.e. 48 bytes
/// on a 64-bit Windows target, and is `UNVERIFIED` here.
/// C: typedef struct _SYSTEM_INFO { union { DWORD dwOemId; struct { WORD wProcessorArchitecture;
/// C:   WORD wReserved; }; } DUMMYUNIONNAME; DWORD dwPageSize; LPVOID lpMinimumApplicationAddress;
/// C:   LPVOID lpMaximumApplicationAddress; DWORD_PTR dwActiveProcessorMask; DWORD dwNumberOfProcessors;
/// C:   DWORD dwProcessorType; DWORD dwAllocationGranularity; WORD wProcessorLevel;
/// C:   WORD wProcessorRevision; } SYSTEM_INFO, *LPSYSTEM_INFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_INFO_u
{
  pub dwOemId: DWORD,
  pub wReserved_arch: [WORD; 2],
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO
{
  pub u: SYSTEM_INFO_u,
  pub dwPageSize: DWORD,
  pub lpMinimumApplicationAddress: LPVOID,
  pub lpMaximumApplicationAddress: LPVOID,
  pub dwActiveProcessorMask: usize,   /* DWORD_PTR */
  pub dwNumberOfProcessors: DWORD,
  pub dwProcessorType: DWORD,
  pub dwAllocationGranularity: DWORD,
  pub wProcessorLevel: WORD,
  pub wProcessorRevision: WORD,
}
#[cfg (feature = "windowsnt")]
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;

/// C 376-381/488-491/503-506: the two further pointer typedefs the wrapper signatures name.
/// `LPMEMORYSTATUS` is `<winbase.h>`'s `MEMORYSTATUS *`, used only as a pointer here (the P-72 ③
/// reading: the record itself gets no Rust spelling), and `LPFILETIME` is `FILETIME *`.
/// C: typedef struct _MEMORYSTATUS { DWORD dwLength; … } MEMORYSTATUS, *LPMEMORYSTATUS;
#[cfg (feature = "windowsnt")]
pub type LPMEMORYSTATUS = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPFILETIME = *mut FILETIME;

/* C 102-114: `MEMORY_STATUS_EX` - the record C spells out because "some versions of compiler
   define MEMORYSTATUSEX, some don't, so we use a different name".  C's nine members in C's order;
   `DWORDLONG` is `unsigned long long` (PORTING.md 1.1's `u64` row) and the two typedef names are
   C's own.  C 113's `} MEMORY_STATUS_EX,*LPMEMORY_STATUS_EX;` is the two aliases.
   C: typedef struct _MEMORY_STATUS_EX { DWORD dwLength; … DWORDLONG ullAvailExtendedVirtual; }
   C:   MEMORY_STATUS_EX, *LPMEMORY_STATUS_EX;  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _MEMORY_STATUS_EX
{
  pub dwLength: DWORD,
  pub dwMemoryLoad: DWORD,
  pub ullTotalPhys: DWORDLONG,
  pub ullAvailPhys: DWORDLONG,
  pub ullTotalPageFile: DWORDLONG,
  pub ullAvailPageFile: DWORDLONG,
  pub ullTotalVirtual: DWORDLONG,
  pub ullAvailVirtual: DWORDLONG,
  pub ullAvailExtendedVirtual: DWORDLONG,
}
/// C 114: `} MEMORY_STATUS_EX,*LPMEMORY_STATUS_EX;` - the record's own name and the pointer alias.
#[cfg (feature = "windowsnt")]
pub type MEMORY_STATUS_EX = _MEMORY_STATUS_EX;
#[cfg (feature = "windowsnt")]
pub type LPMEMORY_STATUS_EX = *mut MEMORY_STATUS_EX;

/* C 116-132: the three exception records "so that GDB would know about these data types".  C's
   `extern PEXCEPTION_POINTERS excptr; PEXCEPTION_POINTERS excptr;` pairs are one definition each
   in Rust (two items of one name are `E0428`, PORTING.md 0.1-12): the definition stands at C's
   position and the `extern` line is recorded.  `PEXCEPTION_POINTERS`/`PEXCEPTION_RECORD`/`PCONTEXT`
   are `<winnt.h>`'s pointers to records no file in the reference tree carries, so they are the
   SDK's own pointer spellings.
   C: extern PEXCEPTION_POINTERS excptr; PEXCEPTION_POINTERS excptr;
   C: extern PEXCEPTION_RECORD excprec; PEXCEPTION_RECORD excprec;
   C: extern PCONTEXT ctxrec; PCONTEXT ctxrec;  */
/// C: typedef struct _EXCEPTION_POINTERS *PEXCEPTION_POINTERS;
/// C: typedef struct _EXCEPTION_RECORD *PEXCEPTION_RECORD;
/// C: typedef struct _CONTEXT *PCONTEXT;
#[cfg (feature = "windowsnt")]
pub type PEXCEPTION_POINTERS = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PEXCEPTION_RECORD = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type PCONTEXT = *mut c_void;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut excptr: PEXCEPTION_POINTERS = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut excprec: PEXCEPTION_RECORD = core::ptr::null_mut ();
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut ctxrec: PCONTEXT = core::ptr::null_mut ();

// C 134-135: `#include <lmcons.h>` / `#include <shlobj.h>` - SDK headers with no file in the
// reference tree; `ShGetFolderPath_fn` below is the one name this slice takes from them.
// C 137-139: `#include <tlhelp32.h>` / `<psapi.h>` / `<w32api.h>` - likewise; `PROCESSENTRY32` and
// `PROCESS_MEMORY_COUNTERS` stand above, and `__W32API_MAJOR_VERSION`/`__W32API_MINOR_VERSION` are
// value-tested by C 141/C 553 (the derived features of the file note).

/* C 140-159: `#if _WIN32_WINNT < 0x0500` around the older-SDK spelling of
   `PROCESS_MEMORY_COUNTERS_EX`, and C 141's second condition (the `psapi.h` that lacks it).  Both
   arms are carried: the record is C's own text (eleven members, `SIZE_T` = `usize`), and the
   `#else` arm is the SDK's own declaration, of which this slice takes nothing.
   C: #if _WIN32_WINNT < 0x0500
   C: #if !defined (__MINGW32__) || __W32API_MAJOR_VERSION < 3 || (__W32API_MAJOR_VERSION == 3 && __W32API_MINOR_VERSION < 15)
   C: typedef struct _PROCESS_MEMORY_COUNTERS_EX { DWORD cb; … SIZE_T PrivateUsage; }
   C:   PROCESS_MEMORY_COUNTERS_EX,*PPROCESS_MEMORY_COUNTERS_EX;
   C: #endif
   C: #endif  */
#[cfg (all (feature = "windowsnt", feature = "win32_winnt_lt_0500",
           feature = "mingw32_w32api_lt_3_15"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _PROCESS_MEMORY_COUNTERS_EX
{
  pub cb: DWORD,
  pub PageFaultCount: DWORD,
  pub PeakWorkingSetSize: SIZE_T,
  pub WorkingSetSize: SIZE_T,
  pub QuotaPeakPagedPoolUsage: SIZE_T,
  pub QuotaPagedPoolUsage: SIZE_T,
  pub QuotaPeakNonPagedPoolUsage: SIZE_T,
  pub QuotaNonPagedPoolUsage: SIZE_T,
  pub PagefileUsage: SIZE_T,
  pub PeakPagefileUsage: SIZE_T,
  pub PrivateUsage: SIZE_T,
}
/// C 157: `} PROCESS_MEMORY_COUNTERS_EX,*PPROCESS_MEMORY_COUNTERS_EX;` - the two names C's own
/// typedef introduces (the arm's `PPROCESS_MEMORY_COUNTERS_EX` is `PPROCESS_MEMORY_COUNTERS_EX *`).
#[cfg (all (feature = "windowsnt", feature = "win32_winnt_lt_0500",
           feature = "mingw32_w32api_lt_3_15"))]
pub type PROCESS_MEMORY_COUNTERS_EX = _PROCESS_MEMORY_COUNTERS_EX;
#[cfg (all (feature = "windowsnt", feature = "win32_winnt_lt_0500",
           feature = "mingw32_w32api_lt_3_15"))]
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;

// C 161-163: `#include <winioctl.h>` / `<aclapi.h>` / `<sddl.h>` - SDK headers with no file in the
// reference tree; the names this slice takes from them (`CTL_CODE`, `FSCTL_GET_REPARSE_POINT`,
// `SDDL_REVISION_1`, `SE_OBJECT_TYPE`, the `Convert…` entry points) stand at their C lines below.
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/acl.rs");        // C 165: `#include <sys/acl.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/acl.rs");               // C 166: `#include <acl.h>`

/* C 168-172: "This is not in MinGW's sddl.h (but they are in MSVC headers), so we define them by
   hand if not already defined": an `#ifndef` guard, i.e. a constant with C's value and no `cfg`
   (the `w32term.rs` reading of the same shape).
   C: #ifndef SDDL_REVISION_1
   C: #define SDDL_REVISION_1	1
   C: #endif	/* SDDL_REVISION_1 */  */
#[cfg (feature = "windowsnt")]
pub const SDDL_REVISION_1: c_int = 1;

/* C 174-223: `#if defined(MINGW_W64)` - "MinGW64 doesn't provide the definition of
   REPARSE_DATA_BUFFER and the associated macros, except on ntifs.h, which cannot be included…".
   Both arms are carried: this arm is the record (C's own member text, the nameless inner structs
   named here as `SymbolicLinkReparseBuffer`/`MountPointReparseBuffer`/`GenericReparseBuffer` are
   C's member name `DUMMYUNIONNAME` is kept as the Rust member name - mingw-w64 defines that macro
   to nothing, so C's consumers reach the inner names through the nameless member and the port
   reaches them through `DUMMYUNIONNAME` (a recorded deviation) - while the rest of C's member text
   is kept), the `#else` arm is the SDK's
   own declaration, of which this slice takes nothing.  The five `#ifndef` guards inside are the
   same "define it if the SDK has not" reading, so the constants carry C's values with no `cfg`.
   C: typedef struct _REPARSE_DATA_BUFFER { ULONG ReparseTag; USHORT ReparseDataLength;
   C:   USHORT Reserved; union { … } DUMMYUNIONNAME; } REPARSE_DATA_BUFFER, *PREPARSE_DATA_BUFFER;  */
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_symbolic_link
{
  pub SubstituteNameOffset: USHORT,
  pub SubstituteNameLength: USHORT,
  pub PrintNameOffset: USHORT,
  pub PrintNameLength: USHORT,
  pub Flags: ULONG,
  pub PathBuffer: [WCHAR; 1],
}
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_mount_point
{
  pub SubstituteNameOffset: USHORT,
  pub SubstituteNameLength: USHORT,
  pub PrintNameOffset: USHORT,
  pub PrintNameLength: USHORT,
  pub PathBuffer: [WCHAR; 1],
}
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_generic
{
  pub DataBuffer: [UCHAR; 1],
}
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_u
{
  pub SymbolicLinkReparseBuffer: REPARSE_DATA_BUFFER_symbolic_link,
  pub MountPointReparseBuffer: REPARSE_DATA_BUFFER_mount_point,
  pub GenericReparseBuffer: REPARSE_DATA_BUFFER_generic,
}
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _REPARSE_DATA_BUFFER
{
  pub ReparseTag: ULONG,
  pub ReparseDataLength: USHORT,
  pub Reserved: USHORT,
  pub DUMMYUNIONNAME: REPARSE_DATA_BUFFER_u,
}
/// C 204: `} REPARSE_DATA_BUFFER, *PREPARSE_DATA_BUFFER;` - the record's name and the pointer.
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub type REPARSE_DATA_BUFFER = _REPARSE_DATA_BUFFER;
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub type PREPARSE_DATA_BUFFER = *mut REPARSE_DATA_BUFFER;

/// C 206-222: the five guarded macros.  `CTL_CODE` is C's function-like macro and keeps its name
/// (PORTING.md 2.2); `FSCTL_GET_REPARSE_POINT` is its one user.
/// C: #define FILE_DEVICE_FILE_SYSTEM	9
/// C: #define METHOD_BUFFERED	        0
/// C: #define FILE_ANY_ACCESS	        0x00000000
/// C: #define CTL_CODE(t,f,m,a)       (((t)<<16)|((a)<<14)|((f)<<2)|(m))
/// C: #define FSCTL_GET_REPARSE_POINT \
/// C:   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 42, METHOD_BUFFERED, FILE_ANY_ACCESS)
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
macro_rules! CTL_CODE {
  ($t:expr, $f:expr, $m:expr, $a:expr) => { (($t) << 16) | (($a) << 14) | (($f) << 2) | ($m) };
}
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub const FILE_DEVICE_FILE_SYSTEM: c_int = 9;
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub const METHOD_BUFFERED: c_int = 0;
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub const FILE_ANY_ACCESS: c_int = 0x00000000;
#[cfg (all (feature = "windowsnt", feature = "mingw_w64"))]
pub const FSCTL_GET_REPARSE_POINT: c_int
  = CTL_CODE! (FILE_DEVICE_FILE_SYSTEM, 42, METHOD_BUFFERED, FILE_ANY_ACCESS);

/* C 225-245: "TCP connection support" - `<sys/socket.h>` (which `nt/inc` provides) and the
   `#undef socket` … `#undef sendto` block that cancels its renames, the C 51-75 shape.
   C: #include <sys/socket.h>
   C: #undef socket … #undef sendto  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/socket.rs");     // C 226: `#include <sys/socket.h>`

/* C 247-261: "We need at least XP level for GetAdaptersAddresses stuff" - the macro dance that
   raises `_WIN32_WINNT` for `<iphlpapi.h>` and restores it, and the include itself.  The dance has
   no Rust counterpart (`#undef`/`#define` of an SDK header's version test) and `<iphlpapi.h>` has
   no file in the reference tree, so both are carried as C's text; the names this slice takes from
   the header are the two `PIP_ADAPTER_*` pointers above.
   C: #if _WIN32_WINNT < 0x0501
   C: # undef ORIG_WIN32_WINNT
   C: # define ORIG_WIN32_WINNT _WIN32_WINNT
   C: # undef _WIN32_WINNT
   C: # define _WIN32_WINNT 0x0501
   C: #endif
   C: #include <iphlpapi.h>	/* should be after winsock2.h */
   C: #ifdef ORIG_WIN32_WINNT
   C: # undef _WIN32_WINNT
   C: # define _WIN32_WINNT ORIG_WIN32_WINNT
   C: # undef ORIG_WIN32_WINNT
   C: #endif  */

// C 263: `#include <wincrypt.h>` - an SDK header with no file in the reference tree; this slice
// names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../lib/include/c-ctype.rs");           // C 265: `#include <c-ctype.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/c-strcase.rs");         // C 266: `#include <c-strcase.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/utimens.rs");           // C 267: `#include <utimens.h>` /* for fdutimens */

#[cfg (feature = "windowsnt")]
include!("include/w32.rs");                      // C 269: `#include "w32.h"`
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/dirent.rs");         // C 270: `#include <dirent.h>`
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");                // C 271: `#include "w32common.h"`
#[cfg (feature = "windowsnt")]
include!("include/w32select.rs");                // C 272: `#include "w32select.h"`
#[cfg (feature = "windowsnt")]
include!("include/systime.rs");                  // C 273: `#include "systime.h"`
#[cfg (feature = "windowsnt")]
include!("include/dispextern.rs");               // C 274: `#include "dispextern.h"` /* for xstrcasecmp */
#[cfg (feature = "windowsnt")]
include!("include/coding.rs");                   // C 275: `#include "coding.h"` /* for Vlocale_coding_system */
#[cfg (feature = "windowsnt")]
include!("../lib/include/careadlinkat.rs");      // C 277: `#include "careadlinkat.h"`
#[cfg (feature = "windowsnt")]
include!("../lib/include/allocator.rs");         // C 278: `#include "allocator.h"`
#[cfg (feature = "windowsnt")]
include!("include/process.rs");                  // C 281: `#include "process.h"`
#[cfg (feature = "windowsnt")]
include!("include/systty.rs");                   // C 282: `#include "systty.h"`

/* C 284-285: `ShGetFolderPath_fn`, the one wrapper record `<shlobj.h>` gives this slice.  C's
   `HRESULT (WINAPI *) (IN HWND, IN int, IN HANDLE, IN DWORD, OUT char *)` is an `extern "system"`
   function pointer with C's five parameters; a C function pointer is nullable, so the port makes
   it an `Option` (PORTING.md P-27).  `SAL`'s `IN`/`OUT` annotations are C's own text.
   C: typedef HRESULT (WINAPI * ShGetFolderPath_fn)
   C:   (IN HWND, IN int, IN HANDLE, IN DWORD, OUT char *);  */
#[cfg (feature = "windowsnt")]
pub type ShGetFolderPath_fn
  = Option<unsafe extern "system" fn (*mut c_void, c_int, *mut c_void, DWORD,
                                      *mut c_char) -> HRESULT>;

/* C 287-298: the forward-declaration block.  Every line is a *declaration* of a function this file
   defines later (`get_rid`, `is_symlink`, `chase_symlinks`, `enable_privilege`,
   `restore_privilege`, `revert_to_self`, `sys_access`) or another unit defines (`e_malloc` in
   `lib/allocator.h`'s unit, `sys_select` in `w32proc.c`, `sys_dup` here at C 8824), so the block
   has no Rust item (PORTING.md 0.1-12) and this slice calls none of them.
   C: static DWORD get_rid (PSID);
   C: static int is_symlink (const char *);
   C: static char * chase_symlinks (const char *);
   C: static int enable_privilege (LPCTSTR, BOOL, TOKEN_PRIVILEGES *);
   C: static int restore_privilege (TOKEN_PRIVILEGES *);
   C: static BOOL WINAPI revert_to_self (void);
   C: static int sys_access (const char *, int);
   C: extern void *e_malloc (size_t);
   C: extern int sys_select (int, SELECT_TYPE *, SELECT_TYPE *, SELECT_TYPE *,
   C: 		       const struct timespec *, const sigset_t *);
   C: extern int sys_dup (int);  */

/* C 301-309: the "Initialization states" banner, whose WARNING names `globals_of_w32` (C 8620+,
   past this slice) as the place each `g_b_init_*` object is reset before dumping.  */
/* C 310-353: the forty-four `static BOOL g_b_init_*;` flags.  C's `static` is internal linkage, so
   the port writes a non-`pub` `static mut` with no `#[unsafe(no_mangle)]` (PORTING.md 0.2), and
   C's tentative definitions are zero.  */
#[cfg (feature = "windowsnt")]
static mut g_b_init_is_windows_9x: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_open_process_token: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_token_information: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_lookup_account_sid: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_sid_sub_authority: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_sid_sub_authority_count: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_security_info: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_file_security_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_file_security_a: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_security_descriptor_owner: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_security_descriptor_group: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_is_valid_sid: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_create_toolhelp32_snapshot: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_process32_first: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_process32_next: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_open_thread_token: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_impersonate_self: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_revert_to_self: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_process_memory_info: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_process_working_set_size: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_global_memory_status: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_global_memory_status_ex: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_length_sid: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_equal_sid: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_copy_sid: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_native_system_info: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_system_times: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_create_symbolic_link_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_create_symbolic_link_a: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_security_descriptor_dacl: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_convert_sd_to_sddl: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_convert_sddl_to_sd: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_is_valid_security_descriptor: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_set_file_security_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_set_file_security_a: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_set_named_security_info_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_set_named_security_info_a: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_adapters_info: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_adapters_addresses: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_reg_open_key_ex_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_reg_query_value_ex_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_expand_environment_strings_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_user_default_ui_language: BOOL = 0;
#[cfg (feature = "windowsnt")]
static mut g_b_init_get_console_font_size: BOOL = 0;

/* C 355-358: the two `extern`-then-define pairs (no header declares either name), so the
   definition is the single item (the `w32fns.rs` `msh_mousewheel` reading) and it keeps C's
   external linkage.
   C: extern BOOL g_b_init_compare_string_w; BOOL g_b_init_compare_string_w;
   C: extern BOOL g_b_init_debug_break_process; BOOL g_b_init_debug_break_process;  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut g_b_init_compare_string_w: BOOL = 0;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut g_b_init_debug_break_process: BOOL = 0;

/* C 360-364: the "BEGIN: Wrapper functions around OpenProcessToken and other functions in
   advapi32.dll that are only supported in Windows NT / 2k / XP" banner.  */
/* C 365-535: the SDK entry-point typedefs.  Each C `typedef R (WINAPI *name) (params);` is the
   port's `Option<unsafe extern "system" fn (params) -> R>` (PORTING.md P-27: a C function pointer
   is nullable), with C's parameter list and `WINAPI` = `extern "system"`.  The SDK record types the
   parameter lists name are the declarations of the section above.  */
/// C: typedef BOOL (WINAPI * OpenProcessToken_Proc) (HANDLE, DWORD, PHANDLE);
#[cfg (feature = "windowsnt")]
pub type OpenProcessToken_Proc
  = Option<unsafe extern "system" fn (HANDLE, DWORD, PHANDLE) -> BOOL>;
/// C: typedef BOOL (WINAPI * GetTokenInformation_Proc) (HANDLE, TOKEN_INFORMATION_CLASS, LPVOID, DWORD, PDWORD);
#[cfg (feature = "windowsnt")]
pub type GetTokenInformation_Proc
  = Option<unsafe extern "system" fn (HANDLE, TOKEN_INFORMATION_CLASS, LPVOID, DWORD,
                                      PDWORD) -> BOOL>;
/// C: typedef BOOL (WINAPI * GetProcessTimes_Proc) (HANDLE, LPFILETIME, LPFILETIME, LPFILETIME, LPFILETIME);
#[cfg (feature = "windowsnt")]
pub type GetProcessTimes_Proc
  = Option<unsafe extern "system" fn (HANDLE, LPFILETIME, LPFILETIME, LPFILETIME,
                                      LPFILETIME) -> BOOL>;
/// C: typedef BOOL (WINAPI * LookupAccountSid_Proc) (LPCTSTR, PSID, LPTSTR, LPDWORD, LPTSTR, LPDWORD, PSID_NAME_USE);
#[cfg (feature = "windowsnt")]
pub type LookupAccountSid_Proc
  = Option<unsafe extern "system" fn (LPCTSTR, PSID, LPTSTR, LPDWORD, LPTSTR, LPDWORD,
                                      PSID_NAME_USE) -> BOOL>;
/// C: typedef PDWORD (WINAPI * GetSidSubAuthority_Proc) (PSID, DWORD);
#[cfg (feature = "windowsnt")]
pub type GetSidSubAuthority_Proc
  = Option<unsafe extern "system" fn (PSID, DWORD) -> PDWORD>;
/// C: typedef PUCHAR (WINAPI * GetSidSubAuthorityCount_Proc) (PSID);
#[cfg (feature = "windowsnt")]
pub type GetSidSubAuthorityCount_Proc = Option<unsafe extern "system" fn (PSID) -> PUCHAR>;
/// C: typedef DWORD (WINAPI * GetSecurityInfo_Proc) (HANDLE, SE_OBJECT_TYPE, SECURITY_INFORMATION, PSID *, PSID *, PACL *, PACL *, PSECURITY_DESCRIPTOR *);
#[cfg (feature = "windowsnt")]
pub type GetSecurityInfo_Proc
  = Option<unsafe extern "system" fn (HANDLE, SE_OBJECT_TYPE, SECURITY_INFORMATION, *mut PSID,
                                      *mut PSID, *mut PACL, *mut PACL,
                                      *mut PSECURITY_DESCRIPTOR) -> DWORD>;
/// C: typedef BOOL (WINAPI * GetFileSecurityW_Proc) (LPCWSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, DWORD, LPDWORD);
#[cfg (feature = "windowsnt")]
pub type GetFileSecurityW_Proc
  = Option<unsafe extern "system" fn (LPCWSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, DWORD,
                                      LPDWORD) -> BOOL>;
/// C: typedef BOOL (WINAPI * GetFileSecurityA_Proc) (LPCSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, DWORD, LPDWORD);
#[cfg (feature = "windowsnt")]
pub type GetFileSecurityA_Proc
  = Option<unsafe extern "system" fn (LPCSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, DWORD,
                                      LPDWORD) -> BOOL>;
/// C: typedef BOOL (WINAPI *SetFileSecurityW_Proc) (LPCWSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR);
#[cfg (feature = "windowsnt")]
pub type SetFileSecurityW_Proc
  = Option<unsafe extern "system" fn (LPCWSTR, SECURITY_INFORMATION,
                                      PSECURITY_DESCRIPTOR) -> BOOL>;
/// C: typedef BOOL (WINAPI *SetFileSecurityA_Proc) (LPCSTR, SECURITY_INFORMATION, PSECURITY_DESCRIPTOR);
#[cfg (feature = "windowsnt")]
pub type SetFileSecurityA_Proc
  = Option<unsafe extern "system" fn (LPCSTR, SECURITY_INFORMATION,
                                      PSECURITY_DESCRIPTOR) -> BOOL>;
/// C: typedef DWORD (WINAPI *SetNamedSecurityInfoW_Proc) (LPCWSTR, SE_OBJECT_TYPE, SECURITY_INFORMATION, PSID, PSID, PACL, PACL);
#[cfg (feature = "windowsnt")]
pub type SetNamedSecurityInfoW_Proc
  = Option<unsafe extern "system" fn (LPCWSTR, SE_OBJECT_TYPE, SECURITY_INFORMATION, PSID, PSID,
                                      PACL, PACL) -> DWORD>;
/// C: typedef DWORD (WINAPI *SetNamedSecurityInfoA_Proc) (LPCSTR, SE_OBJECT_TYPE, SECURITY_INFORMATION, PSID, PSID, PACL, PACL);
#[cfg (feature = "windowsnt")]
pub type SetNamedSecurityInfoA_Proc
  = Option<unsafe extern "system" fn (LPCSTR, SE_OBJECT_TYPE, SECURITY_INFORMATION, PSID, PSID,
                                      PACL, PACL) -> DWORD>;
/// C: typedef BOOL (WINAPI * GetSecurityDescriptorOwner_Proc) (PSECURITY_DESCRIPTOR, PSID *, LPBOOL);
#[cfg (feature = "windowsnt")]
pub type GetSecurityDescriptorOwner_Proc
  = Option<unsafe extern "system" fn (PSECURITY_DESCRIPTOR, *mut PSID, LPBOOL) -> BOOL>;
/// C: typedef BOOL (WINAPI * GetSecurityDescriptorGroup_Proc) (PSECURITY_DESCRIPTOR, PSID *, LPBOOL);
#[cfg (feature = "windowsnt")]
pub type GetSecurityDescriptorGroup_Proc
  = Option<unsafe extern "system" fn (PSECURITY_DESCRIPTOR, *mut PSID, LPBOOL) -> BOOL>;
/// C: typedef BOOL (WINAPI *GetSecurityDescriptorDacl_Proc) (PSECURITY_DESCRIPTOR, LPBOOL, PACL *, LPBOOL);
#[cfg (feature = "windowsnt")]
pub type GetSecurityDescriptorDacl_Proc
  = Option<unsafe extern "system" fn (PSECURITY_DESCRIPTOR, LPBOOL, *mut PACL, LPBOOL) -> BOOL>;
/// C: typedef BOOL (WINAPI * IsValidSid_Proc) (PSID);
#[cfg (feature = "windowsnt")]
pub type IsValidSid_Proc = Option<unsafe extern "system" fn (PSID) -> BOOL>;
/// C: typedef HANDLE (WINAPI * CreateToolhelp32Snapshot_Proc) (DWORD, DWORD);
#[cfg (feature = "windowsnt")]
pub type CreateToolhelp32Snapshot_Proc
  = Option<unsafe extern "system" fn (DWORD, DWORD) -> HANDLE>;
/// C: typedef BOOL (WINAPI * Process32First_Proc) (HANDLE, LPPROCESSENTRY32);
#[cfg (feature = "windowsnt")]
pub type Process32First_Proc = Option<unsafe extern "system" fn (HANDLE, LPPROCESSENTRY32) -> BOOL>;
/// C: typedef BOOL (WINAPI * Process32Next_Proc) (HANDLE, LPPROCESSENTRY32);
#[cfg (feature = "windowsnt")]
pub type Process32Next_Proc = Option<unsafe extern "system" fn (HANDLE, LPPROCESSENTRY32) -> BOOL>;
/// C: typedef BOOL (WINAPI * OpenThreadToken_Proc) (HANDLE, DWORD, BOOL, PHANDLE);
#[cfg (feature = "windowsnt")]
pub type OpenThreadToken_Proc
  = Option<unsafe extern "system" fn (HANDLE, DWORD, BOOL, PHANDLE) -> BOOL>;
/// C: typedef BOOL (WINAPI * ImpersonateSelf_Proc) (SECURITY_IMPERSONATION_LEVEL);
#[cfg (feature = "windowsnt")]
pub type ImpersonateSelf_Proc
  = Option<unsafe extern "system" fn (SECURITY_IMPERSONATION_LEVEL) -> BOOL>;
/// C: typedef BOOL (WINAPI * RevertToSelf_Proc) (void);
#[cfg (feature = "windowsnt")]
pub type RevertToSelf_Proc = Option<unsafe extern "system" fn () -> BOOL>;
/// C: typedef BOOL (WINAPI * GetProcessMemoryInfo_Proc) (HANDLE, PPROCESS_MEMORY_COUNTERS, DWORD);
#[cfg (feature = "windowsnt")]
pub type GetProcessMemoryInfo_Proc
  = Option<unsafe extern "system" fn (HANDLE, PPROCESS_MEMORY_COUNTERS, DWORD) -> BOOL>;
/// C: typedef BOOL (WINAPI * GetProcessWorkingSetSize_Proc) (HANDLE, PSIZE_T, PSIZE_T);
#[cfg (feature = "windowsnt")]
pub type GetProcessWorkingSetSize_Proc
  = Option<unsafe extern "system" fn (HANDLE, *mut SIZE_T, *mut SIZE_T) -> BOOL>;
/// C: typedef BOOL (WINAPI * GlobalMemoryStatus_Proc) (LPMEMORYSTATUS);
#[cfg (feature = "windowsnt")]
pub type GlobalMemoryStatus_Proc
  = Option<unsafe extern "system" fn (LPMEMORYSTATUS) -> BOOL>;
/// C: typedef BOOL (WINAPI * GlobalMemoryStatusEx_Proc) (LPMEMORY_STATUS_EX);
#[cfg (feature = "windowsnt")]
pub type GlobalMemoryStatusEx_Proc
  = Option<unsafe extern "system" fn (LPMEMORY_STATUS_EX) -> BOOL>;
/// C: typedef BOOL (WINAPI * CopySid_Proc) (DWORD, PSID, PSID);
#[cfg (feature = "windowsnt")]
pub type CopySid_Proc = Option<unsafe extern "system" fn (DWORD, PSID, PSID) -> BOOL>;
/// C: typedef BOOL (WINAPI * EqualSid_Proc) (PSID, PSID);
#[cfg (feature = "windowsnt")]
pub type EqualSid_Proc = Option<unsafe extern "system" fn (PSID, PSID) -> BOOL>;
/// C: typedef DWORD (WINAPI * GetLengthSid_Proc) (PSID);
#[cfg (feature = "windowsnt")]
pub type GetLengthSid_Proc = Option<unsafe extern "system" fn (PSID) -> DWORD>;
/// C: typedef void (WINAPI * GetNativeSystemInfo_Proc) (LPSYSTEM_INFO);
#[cfg (feature = "windowsnt")]
pub type GetNativeSystemInfo_Proc
  = Option<unsafe extern "system" fn (LPSYSTEM_INFO)>;
/// C: typedef BOOL (WINAPI * GetSystemTimes_Proc) (LPFILETIME, LPFILETIME, LPFILETIME);
#[cfg (feature = "windowsnt")]
pub type GetSystemTimes_Proc
  = Option<unsafe extern "system" fn (LPFILETIME, LPFILETIME, LPFILETIME) -> BOOL>;
/// C: typedef BOOLEAN (WINAPI *CreateSymbolicLinkW_Proc) (LPCWSTR, LPCWSTR, DWORD);
#[cfg (feature = "windowsnt")]
pub type CreateSymbolicLinkW_Proc
  = Option<unsafe extern "system" fn (LPCWSTR, LPCWSTR, DWORD) -> BOOLEAN>;
/// C: typedef BOOLEAN (WINAPI *CreateSymbolicLinkA_Proc) (LPCSTR, LPCSTR, DWORD);
#[cfg (feature = "windowsnt")]
pub type CreateSymbolicLinkA_Proc
  = Option<unsafe extern "system" fn (LPCSTR, LPCSTR, DWORD) -> BOOLEAN>;
/// C: typedef BOOL (WINAPI *ConvertStringSecurityDescriptorToSecurityDescriptor_Proc) (LPCTSTR, DWORD, PSECURITY_DESCRIPTOR *, PULONG);
#[cfg (feature = "windowsnt")]
pub type ConvertStringSecurityDescriptorToSecurityDescriptor_Proc
  = Option<unsafe extern "system" fn (LPCTSTR, DWORD, *mut PSECURITY_DESCRIPTOR,
                                      PULONG) -> BOOL>;
/// C: typedef BOOL (WINAPI *ConvertSecurityDescriptorToStringSecurityDescriptor_Proc) (PSECURITY_DESCRIPTOR, DWORD, SECURITY_INFORMATION, LPTSTR *, PULONG);
#[cfg (feature = "windowsnt")]
pub type ConvertSecurityDescriptorToStringSecurityDescriptor_Proc
  = Option<unsafe extern "system" fn (PSECURITY_DESCRIPTOR, DWORD, SECURITY_INFORMATION,
                                      *mut LPTSTR, PULONG) -> BOOL>;
/// C: typedef BOOL (WINAPI *IsValidSecurityDescriptor_Proc) (PSECURITY_DESCRIPTOR);
#[cfg (feature = "windowsnt")]
pub type IsValidSecurityDescriptor_Proc
  = Option<unsafe extern "system" fn (PSECURITY_DESCRIPTOR) -> BOOL>;
/// C: typedef DWORD (WINAPI *GetAdaptersInfo_Proc) (PIP_ADAPTER_INFO, PULONG);
#[cfg (feature = "windowsnt")]
pub type GetAdaptersInfo_Proc
  = Option<unsafe extern "system" fn (PIP_ADAPTER_INFO, PULONG) -> DWORD>;
/// C: typedef DWORD (WINAPI *GetAdaptersAddresses_Proc) (ULONG, ULONG, PVOID, PIP_ADAPTER_ADDRESSES, PULONG);
#[cfg (feature = "windowsnt")]
pub type GetAdaptersAddresses_Proc
  = Option<unsafe extern "system" fn (ULONG, ULONG, PVOID, PIP_ADAPTER_ADDRESSES,
                                      PULONG) -> DWORD>;

/* C 537-539: the two loaded entry points and the flag C's own text spells.  `w32.h` 191-193
   declares all three (the mirror's C 189-193 section), so these definitions are the single
   definitions of those names; the mirror currently declares them *outside* its `*_decls` module,
   which is `E0428` in a `windowsnt` translation unit - the ledger records it and the owed mirror
   treatment.  The two `*_Proc` types are the mirror's (C 189-190).
   C: int (WINAPI *pMultiByteToWideChar)(UINT,DWORD,LPCSTR,int,LPWSTR,int);
   C: int (WINAPI *pWideCharToMultiByte)(UINT,DWORD,LPCWSTR,int,LPSTR,int,LPCSTR,LPBOOL);
   C: DWORD multiByteToWideCharFlags;  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut pMultiByteToWideChar: MultiByteToWideChar_Proc = None;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut pWideCharToMultiByte: WideCharToMultiByte_Proc = None;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut multiByteToWideCharFlags: DWORD = 0;

/// C 540-543: the four registry/environment/language typedefs the slice after this one uses.
/// C: typedef LONG (WINAPI *RegOpenKeyExW_Proc) (HKEY,LPCWSTR,DWORD,REGSAM,PHKEY);
/// C: typedef LONG (WINAPI *RegQueryValueExW_Proc) (HKEY,LPCWSTR,LPDWORD,LPDWORD,LPBYTE,LPDWORD);
/// C: typedef DWORD (WINAPI *ExpandEnvironmentStringsW_Proc) (LPCWSTR,LPWSTR,DWORD);
/// C: typedef LANGID (WINAPI *GetUserDefaultUILanguage_Proc) (void);
#[cfg (feature = "windowsnt")]
pub type RegOpenKeyExW_Proc
  = Option<unsafe extern "system" fn (HKEY, LPCWSTR, DWORD, REGSAM, PHKEY) -> LONG>;
#[cfg (feature = "windowsnt")]
pub type RegQueryValueExW_Proc
  = Option<unsafe extern "system" fn (HKEY, LPCWSTR, LPDWORD, LPDWORD, LPBYTE, LPDWORD) -> LONG>;
#[cfg (feature = "windowsnt")]
pub type ExpandEnvironmentStringsW_Proc
  = Option<unsafe extern "system" fn (LPCWSTR, LPWSTR, DWORD) -> DWORD>;
#[cfg (feature = "windowsnt")]
pub type GetUserDefaultUILanguage_Proc = Option<unsafe extern "system" fn () -> LANGID>;

/// C 545: `typedef COORD (WINAPI *GetConsoleFontSize_Proc) (HANDLE, DWORD);` - `COORD` is
/// `<windef.h>`'s pair of 16-bit coordinates.
/// C: typedef struct _COORD { SHORT X; SHORT Y; } COORD, *PCOORD;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COORD
{
  pub X: c_short,
  pub Y: c_short,
}
#[cfg (feature = "windowsnt")]
pub type GetConsoleFontSize_Proc
  = Option<unsafe extern "system" fn (HANDLE, DWORD) -> COORD>;

/* C 547-560: "Old versions of mingw.org's MinGW, before v5.2.0, don't have a _WIN32_WINNT guard
   for CONSOLE_FONT_INFO in wincon.h, and so don't need the conditional definition below".  Both
   arms are carried: C 555-559's record is `wincon.h`'s when the guard is false, so the port
   declares it **once** in the SDK section (the section's `COORD` and `CONSOLE_FONT_INFO`) and
   keeps C's arm text here - C 562's `GetCurrentConsoleFont_Proc` names the record in *both* arms,
   which is why a second, `cfg`-gated declaration would be `E0428` in the guard's arm.
   C: #if _WIN32_WINNT < 0x0501 \\
   C:     && (__W32API_MAJOR_VERSION > 5 \\
   C: 	|| (__W32API_MAJOR_VERSION == 5 && __W32API_MINOR_VERSION >= 2))
   C: typedef struct { DWORD nFont; COORD dwFontSize; } CONSOLE_FONT_INFO;
   C: #endif  */
/// C 555-559's record and its `<wincon.h>` counterpart - **UNVERIFIED** here (no SDK to measure).
/// C: typedef struct _CONSOLE_FONT_INFO { DWORD nFont; COORD dwFontSize; } CONSOLE_FONT_INFO, *PCONSOLE_FONT_INFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_FONT_INFO
{
  pub nFont: DWORD,
  pub dwFontSize: COORD,
}
/// C 562-565: the last of the file's entry-point typedefs.
/// C: typedef BOOL (WINAPI *GetCurrentConsoleFont_Proc) (HANDLE, BOOL, CONSOLE_FONT_INFO *);
#[cfg (feature = "windowsnt")]
pub type GetCurrentConsoleFont_Proc
  = Option<unsafe extern "system" fn (HANDLE, BOOL, *mut CONSOLE_FONT_INFO) -> BOOL>;

/* C 568: the `/* ** A utility function ** */` banner.  */
/* C 569-586: `is_windows_9x`.  C's local `static BOOL s_b_ret = 0;` is a function-local
   `static mut` (Rust keeps it in C's own scope), C's uninitialized `OSVERSIONINFO os_ver;` is
   zeroed (PORTING.md 1.5), C's `GetVersionEx (&os_ver)` truth test is the port's `!= 0`, and C's
   `sizeof (OSVERSIONINFO)` is `core::mem::size_of::<OSVERSIONINFO> ()`.  */
#[cfg (feature = "windowsnt")]
unsafe fn is_windows_9x () -> BOOL
{
  static mut s_b_ret: BOOL = 0;                                         /* C 571 */
  let mut os_ver: OSVERSIONINFO = core::mem::zeroed ();                 /* C 572 */
  if g_b_init_is_windows_9x == 0                                        /* C 573 */
    {
      g_b_init_is_windows_9x = 1;                                       /* C 575 */
      ZeroMemory! (&raw mut os_ver, core::mem::size_of::<OSVERSIONINFO> ());  /* C 576 */
      os_ver.dwOSVersionInfoSize
        = core::mem::size_of::<OSVERSIONINFO> () as DWORD;              /* C 577 */
      if GetVersionEx (&raw mut os_ver) != 0                            /* C 578 */
        {
          s_b_ret = (os_ver.dwPlatformId == VER_PLATFORM_WIN32_WINDOWS) as BOOL;  /* C 580 */
        }
    }
  return s_b_ret;                                                       /* C 584 */
}

/* C 588: `static Lisp_Object ltime (ULONGLONG);` - a forward declaration of a function this file
   *does* define, at C 7374 - past this slice.  It has no Rust item (PORTING.md 0.1-12: a function
   item is visible for its whole module), so the call at C 607 resolves to C 7374's definition
   once that slice lands; until then it is the one unresolved name inside this slice's own text
   (the ledger's Deviations and owed list name it).
   C: static Lisp_Object ltime (ULONGLONG);  */

/* C 590-612: `w32_get_internal_run_time`.  C's two function-pointer tests keep C's `.unwrap ()`
   call shape (the `w32fns.rs` reading), the four `FILETIME` locals are zeroed (C declares them
   without an initializer), and the `LARGE_INTEGER` member path is the `u.LowPart` spelling of the
   record's named inner struct.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_get_internal_run_time () -> Lisp_Object
{
  if get_process_times_fn.is_some ()                                    /* C 592 */
    {
      let mut create: FILETIME = core::mem::zeroed ();                  /* C 594 */
      let mut exit: FILETIME = core::mem::zeroed ();
      let mut kernel: FILETIME = core::mem::zeroed ();
      let mut user: FILETIME = core::mem::zeroed ();
      let proc: HANDLE = GetCurrentProcess ();                          /* C 595 */
      if (get_process_times_fn.unwrap ()) (proc, &raw mut create, &raw mut exit,
                                           &raw mut kernel, &raw mut user) != 0  /* C 596 */
        {
          let mut user_int: LARGE_INTEGER = core::mem::zeroed ();       /* C 598 */
          let mut kernel_int: LARGE_INTEGER = core::mem::zeroed ();     /* C 599 */
          let mut total: LARGE_INTEGER = core::mem::zeroed ();          /* C 600 */
          user_int.u.LowPart = user.dwLowDateTime;                      /* C 601 */
          user_int.u.HighPart = user.dwHighDateTime as LONG;            /* C 602 */
          kernel_int.u.LowPart = kernel.dwLowDateTime;                  /* C 603 */
          kernel_int.u.HighPart = kernel.dwHighDateTime as LONG;        /* C 604 */
          total.QuadPart = user_int.QuadPart + kernel_int.QuadPart;     /* C 605 */
          return ltime (total.QuadPart as ULONGLONG);                   /* C 607 */
        }
    }
  return Fcurrent_time ();                                              /* C 609 */
}

/* C 614-618: the `/* ** The wrapper functions ** */` banner.  */
/* C 619-645: `open_process_token`.  Each of the wrapper functions below has C's five-statement
   shape: the file-local `s_pfn_*` object, the `hm_advapi32` handle, the `is_windows_9x ()` guard,
   the `g_b_init_*`-guarded load, and the null test before the call.  C's cast
   `(OpenProcessToken_Proc) get_proc_addr (…)` is the `transmute` the two pointer types' equal
   widths make the same operation (`decompress.rs` 237's `LOAD_DLL_FN!` reading), and C's `#func`
   string is the literal at the call.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn open_process_token (ProcessHandle: HANDLE, DesiredAccess: DWORD,
                                              TokenHandle: PHANDLE) -> BOOL
{
  static mut s_pfn_Open_Process_Token: OpenProcessToken_Proc = None;    /* C 625 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                /* C 626 */
  if is_windows_9x () == TRUE                                           /* C 627 */
    {
      return FALSE;                                                     /* C 629 */
    }
  if g_b_init_open_process_token == 0                                   /* C 631 */
    {
      g_b_init_open_process_token = 1;                                  /* C 633 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());            /* C 634 */
      s_pfn_Open_Process_Token
        = std::mem::transmute::<VOIDFNPTR, OpenProcessToken_Proc>
            (get_proc_addr (hm_advapi32, c"OpenProcessToken".as_ptr ()));   /* C 635-636 */
    }
  if s_pfn_Open_Process_Token.is_none ()                                /* C 638 */
    {
      return FALSE;                                                     /* C 640 */
    }
  return (s_pfn_Open_Process_Token.unwrap ()) (ProcessHandle, DesiredAccess, TokenHandle);  /* C 642-645 */
}

/* C 646-679: `get_token_information`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_token_information (TokenHandle: HANDLE,
                                                 TokenInformationClass: TOKEN_INFORMATION_CLASS,
                                                 TokenInformation: LPVOID,
                                                 TokenInformationLength: DWORD,
                                                 ReturnLength: PDWORD) -> BOOL
{
  static mut s_pfn_Get_Token_Information: GetTokenInformation_Proc = None;      /* C 657 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                        /* C 658 */
  if is_windows_9x () == TRUE                                                   /* C 659 */
    {
      return FALSE;                                                             /* C 661 */
    }
  if g_b_init_get_token_information == 0                                        /* C 663 */
    {
      g_b_init_get_token_information = 1;                                       /* C 665 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                    /* C 666 */
      s_pfn_Get_Token_Information
        = std::mem::transmute::<VOIDFNPTR, GetTokenInformation_Proc>
            (get_proc_addr (hm_advapi32, c"GetTokenInformation".as_ptr ()));    /* C 667-668 */
    }
  if s_pfn_Get_Token_Information.is_none ()                                     /* C 670 */
    {
      return FALSE;                                                             /* C 672 */
    }
  return (s_pfn_Get_Token_Information.unwrap ()) (TokenHandle, TokenInformationClass,
                                                  TokenInformation, TokenInformationLength,
                                                  ReturnLength);                /* C 674-679 */
}

/* C 680-717: `lookup_account_sid`.  Its `get_proc_addr` argument is C's `LookupAccountSid_Name`
   (C 385-389), the `_UNICODE`-selected literal below.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn lookup_account_sid (lpSystemName: LPCTSTR, Sid: PSID, Name: LPTSTR,
                                              cbName: LPDWORD, DomainName: LPTSTR,
                                              cbDomainName: LPDWORD,
                                              peUse: PSID_NAME_USE) -> BOOL
{
  static mut s_pfn_Lookup_Account_Sid: LookupAccountSid_Proc = None;            /* C 691 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                        /* C 692 */
  if is_windows_9x () == TRUE                                                   /* C 693 */
    {
      return FALSE;                                                             /* C 695 */
    }
  if g_b_init_lookup_account_sid == 0                                           /* C 697 */
    {
      g_b_init_lookup_account_sid = 1;                                          /* C 699 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                    /* C 700 */
      s_pfn_Lookup_Account_Sid
        = std::mem::transmute::<VOIDFNPTR, LookupAccountSid_Proc>
            (get_proc_addr (hm_advapi32, LookupAccountSid_Name.as_ptr ()));     /* C 701-702 */
    }
  if s_pfn_Lookup_Account_Sid.is_none ()                                        /* C 704 */
    {
      return FALSE;                                                             /* C 706 */
    }
  return (s_pfn_Lookup_Account_Sid.unwrap ()) (lpSystemName, Sid, Name, cbName, DomainName,
                                               cbDomainName, peUse);            /* C 708-716 */
}

/// C 383: `static GetProcessTimes_Proc get_process_times_fn = NULL;` - C's `static` (internal
/// linkage) object of the typedef above, zeroed (PORTING.md 0.1-13).  Its two call sites are in
/// `w32_get_internal_run_time`.
/// C: static GetProcessTimes_Proc get_process_times_fn = NULL;
#[cfg (feature = "windowsnt")]
static mut get_process_times_fn: GetProcessTimes_Proc = None;

/* C 385-389: the `_UNICODE`-selected entry-point name.  C's `static const char * const` is a
   `&CStr` constant, and both arms are carried - `_UNICODE` is `undef` in this configuration, so
   the `#else` arm is the live one (the file note's derived feature list).
   C: #ifdef _UNICODE
   C: static const char * const LookupAccountSid_Name = "LookupAccountSidW";
   C: #else
   C: static const char * const LookupAccountSid_Name = "LookupAccountSidA";
   C: #endif  */
#[cfg (all (feature = "windowsnt", feature = "unicode"))]
static LookupAccountSid_Name: &core::ffi::CStr = c"LookupAccountSidW";
#[cfg (all (feature = "windowsnt", not (feature = "unicode")))]
static LookupAccountSid_Name: &core::ffi::CStr = c"LookupAccountSidA";

/* C 718-741: `get_sid_sub_authority`.  Its function-local `static DWORD zero = 0U;` is returned
   by address in both of C's early exits, so the port takes `&raw mut zero`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_sid_sub_authority (pSid: PSID, n: DWORD) -> PDWORD
{
  static mut s_pfn_Get_Sid_Sub_Authority: GetSidSubAuthority_Proc = None;      /* C 721 */
  static mut zero: DWORD = 0;                                                  /* C 722 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                       /* C 723 */
  if is_windows_9x () == TRUE                                                  /* C 724 */
    {
      return &raw mut zero;                                                    /* C 726 */
    }
  if g_b_init_get_sid_sub_authority == 0                                       /* C 728 */
    {
      g_b_init_get_sid_sub_authority = 1;                                      /* C 730 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                   /* C 731 */
      s_pfn_Get_Sid_Sub_Authority
        = std::mem::transmute::<VOIDFNPTR, GetSidSubAuthority_Proc>
            (get_proc_addr (hm_advapi32, c"GetSidSubAuthority".as_ptr ()));    /* C 732-733 */
    }
  if s_pfn_Get_Sid_Sub_Authority.is_none ()                                    /* C 735 */
    {
      return &raw mut zero;                                                    /* C 737 */
    }
  return (s_pfn_Get_Sid_Sub_Authority.unwrap ()) (pSid, n);                    /* C 739 */
}

/* C 742-764: `get_sid_sub_authority_count`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_sid_sub_authority_count (pSid: PSID) -> PUCHAR
{
  static mut s_pfn_Get_Sid_Sub_Authority_Count: GetSidSubAuthorityCount_Proc = None;  /* C 745 */
  static mut zero: UCHAR = 0;                                                         /* C 746 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                              /* C 747 */
  if is_windows_9x () == TRUE                                                         /* C 748 */
    {
      return &raw mut zero;                                                           /* C 750 */
    }
  if g_b_init_get_sid_sub_authority_count == 0                                        /* C 752 */
    {
      g_b_init_get_sid_sub_authority_count = 1;                                       /* C 754 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                          /* C 755 */
      s_pfn_Get_Sid_Sub_Authority_Count
        = std::mem::transmute::<VOIDFNPTR, GetSidSubAuthorityCount_Proc>
            (get_proc_addr (hm_advapi32, c"GetSidSubAuthorityCount".as_ptr ()));      /* C 756-757 */
    }
  if s_pfn_Get_Sid_Sub_Authority_Count.is_none ()                                     /* C 759 */
    {
      return &raw mut zero;                                                           /* C 761 */
    }
  return (s_pfn_Get_Sid_Sub_Authority_Count.unwrap ()) (pSid);                        /* C 763 */
}

/* C 765-796: `get_security_info`.  This one has no `errno` store: C 778's guard returns FALSE
   with no error set, and C's call spans C 793-795.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_security_info (handle: HANDLE, ObjectType: SE_OBJECT_TYPE,
                                             SecurityInfo: SECURITY_INFORMATION,
                                             ppsidOwner: *mut PSID, ppsidGroup: *mut PSID,
                                             ppDacl: *mut PACL, ppSacl: *mut PACL,
                                             ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR)
  -> DWORD
{
  static mut s_pfn_Get_Security_Info: GetSecurityInfo_Proc = None;                    /* C 776 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                              /* C 777 */
  if is_windows_9x () == TRUE                                                         /* C 778 */
    {
      return FALSE as DWORD;                                                          /* C 780 */
    }
  if g_b_init_get_security_info == 0                                                  /* C 782 */
    {
      g_b_init_get_security_info = 1;                                                 /* C 784 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                          /* C 785 */
      s_pfn_Get_Security_Info
        = std::mem::transmute::<VOIDFNPTR, GetSecurityInfo_Proc>
            (get_proc_addr (hm_advapi32, c"GetSecurityInfo".as_ptr ()));              /* C 786-787 */
    }
  if s_pfn_Get_Security_Info.is_none ()                                               /* C 789 */
    {
      return FALSE as DWORD;                                                          /* C 791 */
    }
  return (s_pfn_Get_Security_Info.unwrap ()) (handle, ObjectType, SecurityInfo, ppsidOwner,
                                              ppsidGroup, ppDacl, ppSacl,
                                              ppSecurityDescriptor);                  /* C 793-795 */
}

/* C 797-855: `get_file_security`.  C's two-armed body (the `w32_unicode_filenames` test) declares
   a `MAX_PATH`-long buffer in each arm - `wchar_t filename_w[MAX_PATH]` (the Windows 16-bit
   `wchar_t`, so `c_ushort`: the `include/w32.rs` 405-418 reading of the same parameter) and
   `char filename_a[MAX_PATH]` - and both are passed as pointers to `filename_to_utf16`/
   `filename_to_ansi` (the mirror's declarations).  C's uninitialized buffers are zeroed
   (PORTING.md 1.5).  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_file_security (lpFileName: *const c_char,
                                             RequestedInformation: SECURITY_INFORMATION,
                                             pSecurityDescriptor: PSECURITY_DESCRIPTOR,
                                             nLength: DWORD, lpnLengthNeeded: LPDWORD) -> BOOL
{
  static mut s_pfn_Get_File_SecurityA: GetFileSecurityA_Proc = None;      /* C 805 */
  static mut s_pfn_Get_File_SecurityW: GetFileSecurityW_Proc = None;      /* C 806 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                  /* C 807 */
  if is_windows_9x () == TRUE                                             /* C 808 */
    {
      *errno () = ENOTSUP;                                                /* C 810 */
      return FALSE;                                                       /* C 811 */
    }
  if w32_unicode_filenames                                                 /* C 813 */
    {
      let mut filename_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 815 */

      if g_b_init_get_file_security_w == 0                                 /* C 817 */
        {
          g_b_init_get_file_security_w = 1;                                /* C 819 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());           /* C 820 */
          s_pfn_Get_File_SecurityW
            = std::mem::transmute::<VOIDFNPTR, GetFileSecurityW_Proc>
                (get_proc_addr (hm_advapi32, c"GetFileSecurityW".as_ptr ()));      /* C 821-822 */
        }
      if s_pfn_Get_File_SecurityW.is_none ()                               /* C 824 */
        {
          *errno () = ENOTSUP;                                             /* C 826 */
          return FALSE;                                                    /* C 827 */
        }
      filename_to_utf16 (lpFileName, filename_w.as_mut_ptr ());            /* C 829 */
      return (s_pfn_Get_File_SecurityW.unwrap ()) (filename_w.as_mut_ptr (), RequestedInformation,
                                                   pSecurityDescriptor, nLength,
                                                   lpnLengthNeeded);       /* C 830-832 */
    }
  else
    {
      let mut filename_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 836 */

      if g_b_init_get_file_security_a == 0                                 /* C 838 */
        {
          g_b_init_get_file_security_a = 1;                                /* C 840 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());           /* C 841 */
          s_pfn_Get_File_SecurityA
            = std::mem::transmute::<VOIDFNPTR, GetFileSecurityA_Proc>
                (get_proc_addr (hm_advapi32, c"GetFileSecurityA".as_ptr ()));      /* C 842-843 */
        }
      if s_pfn_Get_File_SecurityA.is_none ()                               /* C 845 */
        {
          *errno () = ENOTSUP;                                             /* C 847 */
          return FALSE;                                                    /* C 848 */
        }
      filename_to_ansi (lpFileName, filename_a.as_mut_ptr ());             /* C 850 */
      return (s_pfn_Get_File_SecurityA.unwrap ()) (filename_a.as_mut_ptr (), RequestedInformation,
                                                   pSecurityDescriptor, nLength,
                                                   lpnLengthNeeded);       /* C 851-853 */
    }
}

/* C 856-910: `set_file_security`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn set_file_security (lpFileName: *const c_char,
                                             SecurityInformation: SECURITY_INFORMATION,
                                             pSecurityDescriptor: PSECURITY_DESCRIPTOR) -> BOOL
{
  static mut s_pfn_Set_File_SecurityW: SetFileSecurityW_Proc = None;      /* C 862 */
  static mut s_pfn_Set_File_SecurityA: SetFileSecurityA_Proc = None;      /* C 863 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                  /* C 864 */
  if is_windows_9x () == TRUE                                             /* C 865 */
    {
      *errno () = ENOTSUP;                                                /* C 867 */
      return FALSE;                                                       /* C 868 */
    }
  if w32_unicode_filenames                                                 /* C 870 */
    {
      let mut filename_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 872 */

      if g_b_init_set_file_security_w == 0                                 /* C 874 */
        {
          g_b_init_set_file_security_w = 1;                                /* C 876 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());           /* C 877 */
          s_pfn_Set_File_SecurityW
            = std::mem::transmute::<VOIDFNPTR, SetFileSecurityW_Proc>
                (get_proc_addr (hm_advapi32, c"SetFileSecurityW".as_ptr ()));      /* C 878-879 */
        }
      if s_pfn_Set_File_SecurityW.is_none ()                               /* C 881 */
        {
          *errno () = ENOTSUP;                                             /* C 883 */
          return FALSE;                                                    /* C 884 */
        }
      filename_to_utf16 (lpFileName, filename_w.as_mut_ptr ());            /* C 886 */
      return (s_pfn_Set_File_SecurityW.unwrap ()) (filename_w.as_mut_ptr (),
                                                   SecurityInformation,
                                                   pSecurityDescriptor);   /* C 887-888 */
    }
  else
    {
      let mut filename_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 892 */

      if g_b_init_set_file_security_a == 0                                 /* C 894 */
        {
          g_b_init_set_file_security_a = 1;                                /* C 896 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());           /* C 897 */
          s_pfn_Set_File_SecurityA
            = std::mem::transmute::<VOIDFNPTR, SetFileSecurityA_Proc>
                (get_proc_addr (hm_advapi32, c"SetFileSecurityA".as_ptr ()));      /* C 898-899 */
        }
      if s_pfn_Set_File_SecurityA.is_none ()                               /* C 901 */
        {
          *errno () = ENOTSUP;                                             /* C 903 */
          return FALSE;                                                    /* C 904 */
        }
      filename_to_ansi (lpFileName, filename_a.as_mut_ptr ());             /* C 906 */
      return (s_pfn_Set_File_SecurityA.unwrap ()) (filename_a.as_mut_ptr (),
                                                   SecurityInformation,
                                                   pSecurityDescriptor);   /* C 907-908 */
    }
}

/* C 911-971: `set_named_security_info`.  Its guard and its two null tests return *the errno
   value* (`ENOTSUP`), which C spells with `return ENOTSUP;`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn set_named_security_info (lpObjectName: LPCTSTR,
                                                   ObjectType: SE_OBJECT_TYPE,
                                                   SecurityInformation: SECURITY_INFORMATION,
                                                   psidOwner: PSID, psidGroup: PSID,
                                                   pDacl: PACL, pSacl: PACL) -> DWORD
{
  static mut s_pfn_Set_Named_Security_InfoW: SetNamedSecurityInfoW_Proc = None;   /* C 921 */
  static mut s_pfn_Set_Named_Security_InfoA: SetNamedSecurityInfoA_Proc = None;   /* C 922 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                         /* C 923 */
  if is_windows_9x () == TRUE                                                    /* C 924 */
    {
      *errno () = ENOTSUP;                                                       /* C 926 */
      return ENOTSUP as DWORD;                                                   /* C 927 */
    }
  if w32_unicode_filenames                                                        /* C 929 */
    {
      let mut filename_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; /* C 931 */

      if g_b_init_set_named_security_info_w == 0                                  /* C 933 */
        {
          g_b_init_set_named_security_info_w = 1;                                 /* C 935 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                  /* C 936 */
          s_pfn_Set_Named_Security_InfoW
            = std::mem::transmute::<VOIDFNPTR, SetNamedSecurityInfoW_Proc>
                (get_proc_addr (hm_advapi32, c"SetNamedSecurityInfoW".as_ptr ()));  /* C 937-938 */
        }
      if s_pfn_Set_Named_Security_InfoW.is_none ()                                /* C 940 */
        {
          *errno () = ENOTSUP;                                                    /* C 942 */
          return ENOTSUP as DWORD;                                                /* C 943 */
        }
      filename_to_utf16 (lpObjectName, filename_w.as_mut_ptr ());                 /* C 945 */
      return (s_pfn_Set_Named_Security_InfoW.unwrap ()) (filename_w.as_mut_ptr (), ObjectType,
                                                         SecurityInformation, psidOwner,
                                                         psidGroup, pDacl, pSacl);  /* C 946-948 */
    }
  else
    {
      let mut filename_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   /* C 952 */

      if g_b_init_set_named_security_info_a == 0                                  /* C 954 */
        {
          g_b_init_set_named_security_info_a = 1;                                 /* C 956 */
          hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                  /* C 957 */
          s_pfn_Set_Named_Security_InfoA
            = std::mem::transmute::<VOIDFNPTR, SetNamedSecurityInfoA_Proc>
                (get_proc_addr (hm_advapi32, c"SetNamedSecurityInfoA".as_ptr ()));  /* C 958-959 */
        }
      if s_pfn_Set_Named_Security_InfoA.is_none ()                                /* C 961 */
        {
          *errno () = ENOTSUP;                                                    /* C 963 */
          return ENOTSUP as DWORD;                                                /* C 964 */
        }
      filename_to_ansi (lpObjectName, filename_a.as_mut_ptr ());                  /* C 966 */
      return (s_pfn_Set_Named_Security_InfoA.unwrap ()) (filename_a.as_mut_ptr (), ObjectType,
                                                         SecurityInformation, psidOwner,
                                                         psidGroup, pDacl, pSacl);  /* C 967-969 */
    }
}

/* C 972-999: `get_security_descriptor_owner`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_security_descriptor_owner
  (pSecurityDescriptor: PSECURITY_DESCRIPTOR, pOwner: *mut PSID,
   lpbOwnerDefaulted: LPBOOL) -> BOOL
{
  static mut s_pfn_Get_Security_Descriptor_Owner: GetSecurityDescriptorOwner_Proc = None;  /* C 978 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                                   /* C 979 */
  if is_windows_9x () == TRUE                                                              /* C 980 */
    {
      *errno () = ENOTSUP;                                                                 /* C 982 */
      return FALSE;                                                                        /* C 983 */
    }
  if g_b_init_get_security_descriptor_owner == 0                                           /* C 985 */
    {
      g_b_init_get_security_descriptor_owner = 1;                                          /* C 987 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                               /* C 988 */
      s_pfn_Get_Security_Descriptor_Owner
        = std::mem::transmute::<VOIDFNPTR, GetSecurityDescriptorOwner_Proc>
            (get_proc_addr (hm_advapi32, c"GetSecurityDescriptorOwner".as_ptr ()));        /* C 989-990 */
    }
  if s_pfn_Get_Security_Descriptor_Owner.is_none ()                                        /* C 992 */
    {
      *errno () = ENOTSUP;                                                                 /* C 994 */
      return FALSE;                                                                        /* C 995 */
    }
  return (s_pfn_Get_Security_Descriptor_Owner.unwrap ()) (pSecurityDescriptor, pOwner,
                                                          lpbOwnerDefaulted);              /* C 997-998 */
}

/* C 1000-1027: `get_security_descriptor_group`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_security_descriptor_group
  (pSecurityDescriptor: PSECURITY_DESCRIPTOR, pGroup: *mut PSID,
   lpbGroupDefaulted: LPBOOL) -> BOOL
{
  static mut s_pfn_Get_Security_Descriptor_Group: GetSecurityDescriptorGroup_Proc = None;  /* C 1006 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                                   /* C 1007 */
  if is_windows_9x () == TRUE                                                              /* C 1008 */
    {
      *errno () = ENOTSUP;                                                                 /* C 1010 */
      return FALSE;                                                                        /* C 1011 */
    }
  if g_b_init_get_security_descriptor_group == 0                                           /* C 1013 */
    {
      g_b_init_get_security_descriptor_group = 1;                                          /* C 1015 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                               /* C 1016 */
      s_pfn_Get_Security_Descriptor_Group
        = std::mem::transmute::<VOIDFNPTR, GetSecurityDescriptorGroup_Proc>
            (get_proc_addr (hm_advapi32, c"GetSecurityDescriptorGroup".as_ptr ()));        /* C 1017-1018 */
    }
  if s_pfn_Get_Security_Descriptor_Group.is_none ()                                        /* C 1020 */
    {
      *errno () = ENOTSUP;                                                                 /* C 1022 */
      return FALSE;                                                                        /* C 1023 */
    }
  return (s_pfn_Get_Security_Descriptor_Group.unwrap ()) (pSecurityDescriptor, pGroup,
                                                          lpbGroupDefaulted);              /* C 1025-1026 */
}

/* C 1028-1057: `get_security_descriptor_dacl`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_security_descriptor_dacl
  (pSecurityDescriptor: PSECURITY_DESCRIPTOR, lpbDaclPresent: LPBOOL, pDacl: *mut PACL,
   lpbDaclDefaulted: LPBOOL) -> BOOL
{
  static mut s_pfn_Get_Security_Descriptor_Dacl: GetSecurityDescriptorDacl_Proc = None;  /* C 1035 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                                 /* C 1036 */
  if is_windows_9x () == TRUE                                                            /* C 1037 */
    {
      *errno () = ENOTSUP;                                                               /* C 1039 */
      return FALSE;                                                                      /* C 1040 */
    }
  if g_b_init_get_security_descriptor_dacl == 0                                          /* C 1042 */
    {
      g_b_init_get_security_descriptor_dacl = 1;                                         /* C 1044 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                             /* C 1045 */
      s_pfn_Get_Security_Descriptor_Dacl
        = std::mem::transmute::<VOIDFNPTR, GetSecurityDescriptorDacl_Proc>
            (get_proc_addr (hm_advapi32, c"GetSecurityDescriptorDacl".as_ptr ()));       /* C 1046-1047 */
    }
  if s_pfn_Get_Security_Descriptor_Dacl.is_none ()                                       /* C 1049 */
    {
      *errno () = ENOTSUP;                                                               /* C 1051 */
      return FALSE;                                                                      /* C 1052 */
    }
  return (s_pfn_Get_Security_Descriptor_Dacl.unwrap ()) (pSecurityDescriptor, lpbDaclPresent,
                                                         pDacl, lpbDaclDefaulted);       /* C 1054-1056 */
}

/* C 1058-1080: `is_valid_sid`.  Its guard returns FALSE with no `errno` store (C 1064-1067).  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn is_valid_sid (sid: PSID) -> BOOL
{
  static mut s_pfn_Is_Valid_Sid: IsValidSid_Proc = None;                 /* C 1062 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                 /* C 1063 */
  if is_windows_9x () == TRUE                                            /* C 1064 */
    {
      return FALSE;                                                      /* C 1066 */
    }
  if g_b_init_is_valid_sid == 0                                          /* C 1068 */
    {
      g_b_init_is_valid_sid = 1;                                         /* C 1070 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());             /* C 1071 */
      s_pfn_Is_Valid_Sid
        = std::mem::transmute::<VOIDFNPTR, IsValidSid_Proc>
            (get_proc_addr (hm_advapi32, c"IsValidSid".as_ptr ()));      /* C 1072-1073 */
    }
  if s_pfn_Is_Valid_Sid.is_none ()                                       /* C 1075 */
    {
      return FALSE;                                                      /* C 1077 */
    }
  return (s_pfn_Is_Valid_Sid.unwrap ()) (sid);                           /* C 1079 */
}

/* C 1081-1103: `equal_sid`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn equal_sid (sid1: PSID, sid2: PSID) -> BOOL
{
  static mut s_pfn_Equal_Sid: EqualSid_Proc = None;                      /* C 1085 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                 /* C 1086 */
  if is_windows_9x () == TRUE                                            /* C 1087 */
    {
      return FALSE;                                                      /* C 1089 */
    }
  if g_b_init_equal_sid == 0                                             /* C 1091 */
    {
      g_b_init_equal_sid = 1;                                            /* C 1093 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());             /* C 1094 */
      s_pfn_Equal_Sid
        = std::mem::transmute::<VOIDFNPTR, EqualSid_Proc>
            (get_proc_addr (hm_advapi32, c"EqualSid".as_ptr ()));        /* C 1095-1096 */
    }
  if s_pfn_Equal_Sid.is_none ()                                          /* C 1098 */
    {
      return FALSE;                                                      /* C 1100 */
    }
  return (s_pfn_Equal_Sid.unwrap ()) (sid1, sid2);                       /* C 1102 */
}

/* C 1104-1126: `get_length_sid`.  Its two early exits return 0.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_length_sid (sid: PSID) -> DWORD
{
  static mut s_pfn_Get_Length_Sid: GetLengthSid_Proc = None;             /* C 1108 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                 /* C 1109 */
  if is_windows_9x () == TRUE                                            /* C 1110 */
    {
      return 0;                                                          /* C 1112 */
    }
  if g_b_init_get_length_sid == 0                                        /* C 1114 */
    {
      g_b_init_get_length_sid = 1;                                       /* C 1116 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());             /* C 1117 */
      s_pfn_Get_Length_Sid
        = std::mem::transmute::<VOIDFNPTR, GetLengthSid_Proc>
            (get_proc_addr (hm_advapi32, c"GetLengthSid".as_ptr ()));    /* C 1118-1119 */
    }
  if s_pfn_Get_Length_Sid.is_none ()                                     /* C 1121 */
    {
      return 0;                                                          /* C 1123 */
    }
  return (s_pfn_Get_Length_Sid.unwrap ()) (sid);                         /* C 1125 */
}

/* C 1127-1149: `copy_sid`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn copy_sid (destlen: DWORD, dest: PSID, src: PSID) -> BOOL
{
  static mut s_pfn_Copy_Sid: CopySid_Proc = None;                        /* C 1131 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                 /* C 1132 */
  if is_windows_9x () == TRUE                                            /* C 1133 */
    {
      return FALSE;                                                      /* C 1135 */
    }
  if g_b_init_copy_sid == 0                                              /* C 1137 */
    {
      g_b_init_copy_sid = 1;                                             /* C 1139 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());             /* C 1140 */
      s_pfn_Copy_Sid
        = std::mem::transmute::<VOIDFNPTR, CopySid_Proc>
            (get_proc_addr (hm_advapi32, c"CopySid".as_ptr ()));         /* C 1141-1142 */
    }
  if s_pfn_Copy_Sid.is_none ()                                           /* C 1144 */
    {
      return FALSE;                                                      /* C 1146 */
    }
  return (s_pfn_Copy_Sid.unwrap ()) (destlen, dest, src);                /* C 1148 */
}

/* C 1151-1155: the "END: Wrapper functions around OpenProcessToken and other functions in
   advapi32.dll" banner.  */
/* C 1156-1175: `get_native_system_info`.  It is the one wrapper with no `is_windows_9x ()`
   early exit: C 1161's test wraps the whole load and C 1173-1174's `else` stores the `-1` into the
   caller's record (the `LPSYSTEM_INFO` of the SDK section), which is C's implicit
   `int`→`DWORD` conversion written out.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_native_system_info (lpSystemInfo: LPSYSTEM_INFO)
{
  static mut s_pfn_Get_Native_System_Info: GetNativeSystemInfo_Proc = None;   /* C 1160 */
  if is_windows_9x () != TRUE                                                 /* C 1161 */
    {
      if g_b_init_get_native_system_info == 0                                 /* C 1163 */
        {
          g_b_init_get_native_system_info = 1;                                /* C 1165 */
          s_pfn_Get_Native_System_Info
            = std::mem::transmute::<VOIDFNPTR, GetNativeSystemInfo_Proc>
                (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
                                c"GetNativeSystemInfo".as_ptr ()));           /* C 1166-1168 */
        }
      if s_pfn_Get_Native_System_Info.is_some ()                              /* C 1170 */
        {
          (s_pfn_Get_Native_System_Info.unwrap ()) (lpSystemInfo);            /* C 1171 */
        }
    }
  else
    {
      (*lpSystemInfo).dwNumberOfProcessors = (-1 as c_int) as DWORD;           /* C 1174 */
    }
}

/* C 1176-1197: `get_system_times` - the last definition of this slice.  C 1194-1195's braceless
   `if (s_pfn_Get_System_times == NULL) return FALSE;` keeps its statement.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_system_times (lpIdleTime: LPFILETIME, lpKernelTime: LPFILETIME,
                                            lpUserTime: LPFILETIME) -> BOOL
{
  static mut s_pfn_Get_System_times: GetSystemTimes_Proc = None;          /* C 1182 */
  if is_windows_9x () == TRUE                                             /* C 1183 */
    {
      return FALSE;                                                       /* C 1185 */
    }
  if g_b_init_get_system_times == 0                                       /* C 1187 */
    {
      g_b_init_get_system_times = 1;                                      /* C 1189 */
      s_pfn_Get_System_times
        = std::mem::transmute::<VOIDFNPTR, GetSystemTimes_Proc>
            (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
                            c"GetSystemTimes".as_ptr ()));                /* C 1190-1192 */
    }
  if s_pfn_Get_System_times.is_none ()                                    /* C 1194 */
    {
      return FALSE;                                                       /* C 1195 */
    }
  return (s_pfn_Get_System_times.unwrap ()) (lpIdleTime, lpKernelTime, lpUserTime);  /* C 1196 */
}
