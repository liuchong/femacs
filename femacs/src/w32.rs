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
// C 19: `#include <config.h>`, whose AH_BOTTOM `#include <conf_post.h>` is the mirror below.
// The edge is the one `femacs/src/w32select.rs` 134 records for the same header; this unit's
// `DebPrint` call (C 2400) reaches the macro that mirror carries.  The ledger's `## Deviations`
// names it.
#[cfg (feature = "windowsnt")]
include!("include/conf_post.rs");

// C 95's `lisp.h` 28: `#include <stdcountof.h>` - C's own `countof` macro, which this file uses
// (`countof (tempdirs)`, `countof (samples)`, `countof (dflt_envvars)`).  `include/lisp.rs` does
// not carry the edge, so the unit includes the mirror itself (the ledger's `## Deviations`).
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdcountof.rs");

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
/* C 270's `#include <dirent.h>` (`nt/inc/dirent.h`) names `_fsize_t` in `struct dirent`'s
   `d_size` member, and that header is not the mirror's to declare (its own note records that the
   runtime type "stays spelled the way C spells it, named from the including unit") - so the unit
   declares it at the include line (P-88 (1), the `nt/inc/sys/include/acl.rs` `ssize_t` reading).
   The value is the SDK's `unsigned long`, **UNVERIFIED** here (no `<io.h>` on this machine).
   C: typedef unsigned long _fsize_t;  */
#[cfg (feature = "windowsnt")]
pub type _fsize_t = c_ulong;
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

/* ----------------------------------------------------------------------------------------------
   `<windows.h>` and the SDK surface the **second** slice (C 1199-4173) names, at the same
   declaration sites as the section above: C 101's `#include <windows.h>` (with `<winbase.h>`,
   `<winuser.h>`, `<winnls.h>`, `<winreg.h>`, `<winerror.h>`, `<wincrypt.h>` and `<winnetwk.h>`
   behind it), C 134-135's `<lmcons.h>`/`<shlobj.h>` and C 161-163's
   `<winioctl.h>`/`<aclapi.h>`/`<sddl.h>`.  None of them has a file in the reference tree (P-88 ①),
   so every name stands here with the SDK's own text above it and no file is added under
   `femacs/src/include/`.

   **No measurement was possible**: this machine has no `windows.h` at all (the file note), so
   every scalar is PORTING.md 1.1's fixed width (`DWORD`/`LONG`/`ULONG`/`UINT` are the 32-bit
   Windows types whatever this target's `c_long` is; `HANDLE` and the SDK's handle records are
   `void *`), every record below is `#[repr(C)]` and **UNVERIFIED**, and its layout assertion is
   owed (the ledger's owed list).  `ERROR_*`'s values are `<winerror.h>`'s, `CP_*`/`LOCALE_*`'s
   are `<winnls.h>`'s and the `RESOURCE*`/`CONNECT_*`/`WNet*` family's are `<winnetwk.h>`'s.
   ---------------------------------------------------------------------------------------------- */

/// C 1422/2900/4204: the `<winerror.h>` values the file's four error switches name.
/// C: #define NO_ERROR 0L
/// C: #define ERROR_FILE_NOT_FOUND 2L
/// C: #define ERROR_PATH_NOT_FOUND 3L
/// C: #define ERROR_ACCESS_DENIED 5L
/// C: #define ERROR_INVALID_DRIVE 15L
/// C: #define ERROR_NOT_READY 21L
/// C: #define ERROR_NOT_SUPPORTED 50L
/// C: #define ERROR_BAD_NETPATH 53L
/// C: #define ERROR_NETWORK_ACCESS_DENIED 65L
/// C: #define ERROR_BAD_NET_NAME 67L
/// C: #define ERROR_ALREADY_ASSIGNED 85L
/// C: #define ERROR_INVALID_PARAMETER 87L
/// C: #define ERROR_INSUFFICIENT_BUFFER 122L
/// C: #define ERROR_INVALID_NAME 123L
/// C: #define ERROR_BAD_PATHNAME 161L
/// C: #define ERROR_BUSY 170L
/// C: #define ERROR_INVALID_FLAGS 1004L
/// C: #define ERROR_NO_UNICODE_TRANSLATION 1113L
/// C: #define ERROR_NO_NET_OR_BAD_PATH 1203L
/// C: #define ERROR_NO_NETWORK 1222L
/// C: #define ERROR_CANCELLED 1223L
/// C: #define ERROR_LOGON_FAILURE 1326L
#[cfg (feature = "windowsnt")]
pub const NO_ERROR: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub const ERROR_FILE_NOT_FOUND: DWORD = 2;
#[cfg (feature = "windowsnt")]
pub const ERROR_PATH_NOT_FOUND: DWORD = 3;
#[cfg (feature = "windowsnt")]
pub const ERROR_ACCESS_DENIED: DWORD = 5;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_DRIVE: DWORD = 15;
#[cfg (feature = "windowsnt")]
pub const ERROR_NOT_READY: DWORD = 21;
#[cfg (feature = "windowsnt")]
pub const ERROR_NOT_SUPPORTED: DWORD = 50;
#[cfg (feature = "windowsnt")]
pub const ERROR_BAD_NETPATH: DWORD = 53;
#[cfg (feature = "windowsnt")]
pub const ERROR_NETWORK_ACCESS_DENIED: DWORD = 65;
#[cfg (feature = "windowsnt")]
pub const ERROR_BAD_NET_NAME: DWORD = 67;
#[cfg (feature = "windowsnt")]
pub const ERROR_ALREADY_ASSIGNED: DWORD = 85;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_PARAMETER: DWORD = 87;
#[cfg (feature = "windowsnt")]
pub const ERROR_INSUFFICIENT_BUFFER: DWORD = 122;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_NAME: DWORD = 123;
#[cfg (feature = "windowsnt")]
pub const ERROR_BAD_PATHNAME: DWORD = 161;
#[cfg (feature = "windowsnt")]
pub const ERROR_BUSY: DWORD = 170;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_FLAGS: DWORD = 1004;
#[cfg (feature = "windowsnt")]
pub const ERROR_NO_UNICODE_TRANSLATION: DWORD = 1113;
#[cfg (feature = "windowsnt")]
pub const ERROR_NO_NET_OR_BAD_PATH: DWORD = 1203;
#[cfg (feature = "windowsnt")]
pub const ERROR_NO_NETWORK: DWORD = 1222;
#[cfg (feature = "windowsnt")]
pub const ERROR_CANCELLED: DWORD = 1223;
#[cfg (feature = "windowsnt")]
pub const ERROR_LOGON_FAILURE: DWORD = 1326;

/// C 1707-1763/2486-2492: the file-name codepage - `<winnls.h>`'s two codepage numbers and the
/// two array bounds of its `CPINFO`.
/// C: #define CP_ACP 0
/// C: #define CP_UTF8 65001
/// C: #define MAX_DEFAULTCHAR 2
/// C: #define MAX_LEADBYTES 12
#[cfg (feature = "windowsnt")]
pub const CP_ACP: c_int = 0;
#[cfg (feature = "windowsnt")]
pub const CP_UTF8: c_int = 65001;
#[cfg (feature = "windowsnt")]
pub const MAX_DEFAULTCHAR: usize = 2;
#[cfg (feature = "windowsnt")]
pub const MAX_LEADBYTES: usize = 12;

/// C 1713/1749/1752/2488: `CPINFO` - `<winnls.h>`'s record, embedded by value (a function-local
/// `static`, one local and the two `GetCPInfo` calls), whose `MaxCharSize` this file reads.  Its
/// size (4 + 2 + 12, padded) is `UNVERIFIED` here.
/// C: typedef struct _cpinfo { UINT MaxCharSize; BYTE DefaultChar[MAX_DEFAULTCHAR];
/// C:   BYTE LeadByte[MAX_LEADBYTES]; } CPINFO, *LPCPINFO;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPINFO
{
  pub MaxCharSize: UINT,
  pub DefaultChar: [BYTE; MAX_DEFAULTCHAR],
  pub LeadByte: [BYTE; MAX_LEADBYTES],
}

/// C 2024-2035: `ULARGE_INTEGER` - `<winnt.h>`'s union, read through `.QuadPart` after a
/// `memcpy` of a `FILETIME`.  The SDK's nameless inner struct is named `u`, exactly as the
/// section above names `LARGE_INTEGER`'s (a recorded deviation, not a different member).
/// C: typedef union _ULARGE_INTEGER { struct { DWORD LowPart; DWORD HighPart; } u;
/// C:   ULONGLONG QuadPart; } ULARGE_INTEGER;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ULARGE_INTEGER_u
{
  pub LowPart: DWORD,
  pub HighPart: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ULARGE_INTEGER
{
  pub u: ULARGE_INTEGER_u,
  pub QuadPart: u64,
}

/// C 2393: `static HCRYPTPROV w32_crypto_hprov;` - `<wincrypt.h>`'s provider handle, which the
/// SDK makes a `ULONG_PTR` (`usize`: PORTING.md 1.1's handle rule).
/// C: typedef ULONG_PTR HCRYPTPROV;
#[cfg (feature = "windowsnt")]
pub type HCRYPTPROV = usize;

/// C 1237/1272: `TOKEN_PRIVILEGES priv_current;` - the record `enable_privilege` fills and
/// `restore_privilege` reads back, embedded **by value** here (the note at C 290 above declares
/// the name only as a pointer, which is all the previous slice's prototypes needed).
/// `LUID`/`LUID_AND_ATTRIBUTES` are `<winnt.h>`'s, and `Privileges` is the SDK's `ANYSIZE_ARRAY`
/// (1) array, so the port spells that one element; both records are `UNVERIFIED`.
/// C: typedef struct _LUID { DWORD LowPart; LONG HighPart; } LUID, *PLUID;
/// C: typedef struct _LUID_AND_ATTRIBUTES { LUID Luid; DWORD Attributes; } LUID_AND_ATTRIBUTES, *PLUID_AND_ATTRIBUTES;
/// C: typedef struct _TOKEN_PRIVILEGES { DWORD PrivilegeCount; LUID_AND_ATTRIBUTES Privileges[ANYSIZE_ARRAY]; } TOKEN_PRIVILEGES, *PTOKEN_PRIVILEGES;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LUID
{
  pub LowPart: DWORD,
  pub HighPart: LONG,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LUID_AND_ATTRIBUTES
{
  pub Luid: LUID,
  pub Attributes: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TOKEN_PRIVILEGES
{
  pub PrivilegeCount: DWORD,
  pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}

/// C 2277-2279: `TOKEN_USER user_token;` and `TOKEN_PRIMARY_GROUP group_token;` - the two records
/// `get_token_information` writes into the caller's buffer and this file `memcpy`s out by value.
/// `<winnt.h>`'s `SID_AND_ATTRIBUTES` is what `TOKEN_USER` wraps; both are `UNVERIFIED` here.
/// C: typedef struct _SID_AND_ATTRIBUTES { PSID Sid; DWORD Attributes; } SID_AND_ATTRIBUTES, *PSID_AND_ATTRIBUTES;
/// C: typedef struct _TOKEN_USER { SID_AND_ATTRIBUTES User; } TOKEN_USER, *PTOKEN_USER;
/// C: typedef struct _TOKEN_PRIMARY_GROUP { PSID PrimaryGroup; } TOKEN_PRIMARY_GROUP, *PTOKEN_PRIMARY_GROUP;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SID_AND_ATTRIBUTES
{
  pub Sid: PSID,
  pub Attributes: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TOKEN_USER
{
  pub User: SID_AND_ATTRIBUTES,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TOKEN_PRIMARY_GROUP
{
  pub PrimaryGroup: PSID,
}

/// C 3673-3674: `WIN32_FIND_DATAA`/`WIN32_FIND_DATAW` - `<minwinbase.h>`'s two records, embedded
/// by value as file-scope statics and filled by `FindFirstFile`/`FindNextFile`.  The members this
/// slice reads are `cFileName` and `cAlternateFileName`; `MAX_PATH`/14 are the SDK's sizes and
/// the `W` form's characters are the Windows 16-bit `WCHAR` (the section above).  Both records
/// are `UNVERIFIED`.
/// C: typedef struct _WIN32_FIND_DATAA { DWORD dwFileAttributes; FILETIME ftCreationTime;
/// C:   FILETIME ftLastAccessTime; FILETIME ftLastWriteTime; DWORD nFileSizeHigh; DWORD nFileSizeLow;
/// C:   DWORD dwReserved0; DWORD dwReserved1; CHAR cFileName[MAX_PATH]; CHAR cAlternateFileName[14];
/// C:   } WIN32_FIND_DATAA, *PWIN32_FIND_DATAA, *LPWIN32_FIND_DATAA;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_FIND_DATAA
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
/// C: typedef struct _WIN32_FIND_DATAW { DWORD dwFileAttributes; FILETIME ftCreationTime;
/// C:   FILETIME ftLastAccessTime; FILETIME ftLastWriteTime; DWORD nFileSizeHigh; DWORD nFileSizeLow;
/// C:   DWORD dwReserved0; DWORD dwReserved1; WCHAR cFileName[MAX_PATH]; WCHAR cAlternateFileName[14];
/// C:   } WIN32_FIND_DATAW, *PWIN32_FIND_DATAW, *LPWIN32_FIND_DATAW;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_FIND_DATAW
{
  pub dwFileAttributes: DWORD,
  pub ftCreationTime: FILETIME,
  pub ftLastAccessTime: FILETIME,
  pub ftLastWriteTime: FILETIME,
  pub nFileSizeHigh: DWORD,
  pub nFileSizeLow: DWORD,
  pub dwReserved0: DWORD,
  pub dwReserved1: DWORD,
  pub cFileName: [WCHAR; MAX_PATH as usize],
  pub cAlternateFileName: [WCHAR; 14],
}

/// C 3893-3900/3924-3931: `NETRESOURCEW`/`NETRESOURCEA` - `<winnetwk.h>`'s two records, embedded
/// by value in `open_unc_volume` (C 3896) and `logon_network_drive` (C 4058), plus the two
/// pointer aliases `read_unc_volume` casts its enumeration buffer to.  Both are `UNVERIFIED`.
/// C: typedef struct _NETRESOURCEW { DWORD dwScope; DWORD dwType; DWORD dwDisplayType; DWORD dwUsage;
/// C:   LPWSTR lpLocalName; LPWSTR lpRemoteName; LPWSTR lpComment; LPWSTR lpProvider; } NETRESOURCEW, *LPNETRESOURCEW;
/// C: typedef struct _NETRESOURCEA { DWORD dwScope; DWORD dwType; DWORD dwDisplayType; DWORD dwUsage;
/// C:   LPSTR lpLocalName; LPSTR lpRemoteName; LPSTR lpComment; LPSTR lpProvider; } NETRESOURCEA, *LPNETRESOURCEA;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETRESOURCEW
{
  pub dwScope: DWORD,
  pub dwType: DWORD,
  pub dwDisplayType: DWORD,
  pub dwUsage: DWORD,
  pub lpLocalName: LPWSTR,
  pub lpRemoteName: LPWSTR,
  pub lpComment: LPWSTR,
  pub lpProvider: LPWSTR,
}
#[cfg (feature = "windowsnt")]
pub type LPNETRESOURCEW = *mut NETRESOURCEW;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETRESOURCEA
{
  pub dwScope: DWORD,
  pub dwType: DWORD,
  pub dwDisplayType: DWORD,
  pub dwUsage: DWORD,
  pub lpLocalName: LPSTR,
  pub lpRemoteName: LPSTR,
  pub lpComment: LPSTR,
  pub lpProvider: LPSTR,
}
#[cfg (feature = "windowsnt")]
pub type LPNETRESOURCEA = *mut NETRESOURCEA;

/// C 3892-3931/4004/4044/4127: `<winnetwk.h>`'s eight record-field values and the
/// `<winbase.h>`/`<winnt.h>` file-attribute pair `access_attrs` builds.
/// C: #define RESOURCE_GLOBALNET 0x00000002
/// C: #define RESOURCETYPE_DISK 0x00000001
/// C: #define RESOURCEDISPLAYTYPE_SERVER 0x00000002
/// C: #define RESOURCEDISPLAYTYPE_SHARE 0x00000003
/// C: #define RESOURCEUSAGE_CONTAINER 0x00000002
/// C: #define RESOURCEUSAGE_CONNECTABLE 0x00000001
/// C: #define CONNECT_INTERACTIVE 0x00000008
/// C: #define DRIVE_REMOTE 4
/// C: #define FILE_ATTRIBUTE_READONLY 0x00000001
/// C: #define FILE_ATTRIBUTE_DIRECTORY 0x00000010
#[cfg (feature = "windowsnt")]
pub const RESOURCE_GLOBALNET: DWORD = 0x00000002;
#[cfg (feature = "windowsnt")]
pub const RESOURCETYPE_DISK: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const RESOURCEDISPLAYTYPE_SERVER: DWORD = 0x00000002;
#[cfg (feature = "windowsnt")]
pub const RESOURCEDISPLAYTYPE_SHARE: DWORD = 0x00000003;
#[cfg (feature = "windowsnt")]
pub const RESOURCEUSAGE_CONTAINER: DWORD = 0x00000002;
#[cfg (feature = "windowsnt")]
pub const RESOURCEUSAGE_CONNECTABLE: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const CONNECT_INTERACTIVE: DWORD = 0x00000008;
#[cfg (feature = "windowsnt")]
pub const DRIVE_REMOTE: UINT = 4;
#[cfg (feature = "windowsnt")]
pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x00000010;

/// C 2925: `SM_CMOUSEBUTTONS` (`<winuser.h>`) and C 2956's `PROCESS_VM_READ` (`<winnt.h>`).
/// C: #define SM_CMOUSEBUTTONS 43
/// C: #define PROCESS_VM_READ (0x0010)
#[cfg (feature = "windowsnt")]
pub const SM_CMOUSEBUTTONS: c_int = 43;
#[cfg (feature = "windowsnt")]
pub const PROCESS_VM_READ: DWORD = 0x0010;

/// C 1438-1441: the three `GetAdaptersAddresses` flags `<iphlpapi.h>` defines.
/// C: #define GAA_FLAG_SKIP_ANYCAST 0x0002
/// C: #define GAA_FLAG_SKIP_MULTICAST 0x0004
/// C: #define GAA_FLAG_SKIP_DNS_SERVER 0x0008
#[cfg (feature = "windowsnt")]
pub const GAA_FLAG_SKIP_ANYCAST: ULONG = 0x0002;
#[cfg (feature = "windowsnt")]
pub const GAA_FLAG_SKIP_MULTICAST: ULONG = 0x0004;
#[cfg (feature = "windowsnt")]
pub const GAA_FLAG_SKIP_DNS_SERVER: ULONG = 0x0008;

/// C 2363-2365: `<wincrypt.h>`'s provider type/flags, `<winerror.h>`'s `S_OK`,
/// `<shlobj.h>`'s `CSIDL_APPDATA`, `<winnls.h>`'s three locale constants and `<winnt.h>`'s
/// `SORT_DEFAULT`/`MAKELCID`.  `LCID` is a `DWORD` and `LCTYPE` a `DWORD`.
/// C: #define PROV_RSA_FULL 1
/// C: #define CRYPT_VERIFYCONTEXT 0xF0000000
/// C: #define CRYPT_SILENT 0x00000040
/// C: #define S_OK ((HRESULT) 0L)
/// C: #define CSIDL_APPDATA 0x001a
/// C: #define LOCALE_USER_DEFAULT 0x0400
/// C: #define LOCALE_SABBREVLANGNAME 0x00000003
/// C: #define LOCALE_USE_CP_ACP 0x40000000
/// C: #define SORT_DEFAULT 0
/// C: typedef DWORD LCID, *PLCID;
#[cfg (feature = "windowsnt")]
pub const PROV_RSA_FULL: DWORD = 1;
#[cfg (feature = "windowsnt")]
pub const CRYPT_VERIFYCONTEXT: DWORD = 0xF0000000;
#[cfg (feature = "windowsnt")]
pub const CRYPT_SILENT: DWORD = 0x00000040;
#[cfg (feature = "windowsnt")]
pub const S_OK: HRESULT = 0;
#[cfg (feature = "windowsnt")]
pub const CSIDL_APPDATA: c_int = 0x001a;
#[cfg (feature = "windowsnt")]
pub const LOCALE_USER_DEFAULT: LCID = 0x0400;
#[cfg (feature = "windowsnt")]
pub const LOCALE_SABBREVLANGNAME: DWORD = 0x00000003;
#[cfg (feature = "windowsnt")]
pub const LOCALE_USE_CP_ACP: DWORD = 0x40000000;
#[cfg (feature = "windowsnt")]
pub const SORT_DEFAULT: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub type LCID = DWORD;
/// C 3054: `LCID def_lcid = MAKELCID (lang_id, SORT_DEFAULT);` - the SDK's macro as a
/// `macro_rules!` of C's own expression (PORTING.md 2.1).
/// C: #define MAKELCID(lgid, srtid) ((DWORD) ((((DWORD) ((WORD) (srtid))) << 16) | ((DWORD) ((WORD) (lgid)))))
#[cfg (feature = "windowsnt")]
macro_rules! MAKELCID {
  ($lgid:expr, $srtid:expr) => {
    ((((($srtid) as WORD) as DWORD) << 16) | ((($lgid) as WORD) as DWORD)) };
}

/// C 1239/1274/1245: `SE_CREATE_SYMBOLIC_LINK_NAME`, the one SDK *macro* this slice passes; the
/// SDK builds it with `TEXT ()`, i.e. the wide form under `_UNICODE`.  The port's `LPCTSTR` is
/// the ANSI form (see the section above), so the constant here is the ANSI text and the `unicode`
/// arm is owed (the ledger records it).
/// C: #define SE_CREATE_SYMBOLIC_LINK_NAME TEXT("SeCreateSymbolicLinkPrivilege")
#[cfg (feature = "windowsnt")]
pub const SE_CREATE_SYMBOLIC_LINK_NAME: &core::ffi::CStr
  = c"SeCreateSymbolicLinkPrivilege";

/// C 1540/1560/2393: the SDK entry points this slice calls, none of which is loaded through
/// `get_proc_addr` (those are the entry-point typedefs above).  Every one is `WINAPI` and so
/// `unsafe extern "system"`; the `…A`/`…W` aliases C's text spells without the suffix
/// (`GetFileAttributes` at C 4129 is the `A`-free spelling, `GetShortPathName`/`GetVolumeInformation`/
/// `FindFirstFile`/`FindNextFile`/`GetCurrentDirectory`/`GetModuleFileName`/`GetUserName`/
/// `ExpandEnvironmentStrings`/`CharNext`/`WNetOpenEnum`/`WNetEnumResource`/`WNetAddConnection2`
/// are the `…A`/`…W` pairs) are kept as C spells them: a Windows target supplies whichever entry
/// point its own `#define` picks (the `GetVersionEx` reading, section above).
/// C: WINBASEAPI HANDLE WINAPI OpenProcess (DWORD dwDesiredAccess, BOOL bInheritHandle, DWORD dwProcessId);
/// C: WINBASEAPI BOOL WINAPI ReadProcessMemory (HANDLE hProcess, LPCVOID lpBaseAddress, LPVOID lpBuffer, SIZE_T nSize, SIZE_T *lpNumberOfBytesRead);
/// C: WINBASEAPI BOOL WINAPI CloseHandle (HANDLE hObject);
/// C: WINBASEAPI DWORD WINAPI GetLastError (void);
/// C: WINBASEAPI DWORD WINAPI GetCurrentProcessId (void);
/// C: WINBASEAPI void WINAPI GetSystemInfo (LPSYSTEM_INFO lpSystemInfo);
/// C: WINBASEAPI int WINAPI GetSystemMetrics (int nIndex);
/// C: WINBASEAPI void WINAPI Sleep (DWORD dwMilliseconds);
/// C: WINBASEAPI DWORD WINAPI GetTickCount (void);
/// C: WINBASEAPI UINT WINAPI GetDriveTypeA (LPCSTR lpRootPathName);
/// C: WINBASEAPI DWORD WINAPI GetFileAttributesA (LPCSTR lpFileName);
/// C: WINBASEAPI DWORD WINAPI GetFileAttributesW (LPCWSTR lpFileName);
/// C: WINBASEAPI BOOL WINAPI GetVolumeInformationA (LPCSTR, LPSTR, DWORD, LPDWORD, LPDWORD, LPDWORD, LPSTR, DWORD);
/// C: WINBASEAPI BOOL WINAPI GetVolumeInformationW (LPCWSTR, LPWSTR, DWORD, LPDWORD, LPDWORD, LPDWORD, LPWSTR, DWORD);
/// C: WINBASEAPI DWORD WINAPI GetShortPathNameA (LPCSTR lpszLongPath, LPSTR lpszShortPath, DWORD cchBuffer);
/// C: WINBASEAPI DWORD WINAPI GetShortPathNameW (LPCWSTR lpszLongPath, LPWSTR lpszShortPath, DWORD cchBuffer);
/// C: WINBASEAPI HANDLE WINAPI FindFirstFileA (LPCSTR lpFileName, LPWIN32_FIND_DATAA lpFindFileData);
/// C: WINBASEAPI HANDLE WINAPI FindFirstFileW (LPCWSTR lpFileName, LPWIN32_FIND_DATAW lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindNextFileA (HANDLE hFindFile, LPWIN32_FIND_DATAA lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindNextFileW (HANDLE hFindFile, LPWIN32_FIND_DATAW lpFindFileData);
/// C: WINBASEAPI BOOL WINAPI FindClose (HANDLE hFindFile);
/// C: WINBASEAPI DWORD WINAPI GetCurrentDirectoryA (DWORD nBufferLength, LPSTR lpBuffer);
/// C: WINBASEAPI DWORD WINAPI GetCurrentDirectoryW (DWORD nBufferLength, LPWSTR lpBuffer);
/// C: WINBASEAPI DWORD WINAPI GetModuleFileNameA (HMODULE hModule, LPSTR lpFilename, DWORD nSize);
/// C: WINBASEAPI BOOL WINAPI GetUserNameA (LPSTR lpBuffer, LPDWORD pcbBuffer);
/// C: WINBASEAPI BOOL WINAPI GetCPInfo (UINT CodePage, LPCPINFO lpCPInfo);
/// C: WINBASEAPI DWORD WINAPI ExpandEnvironmentStringsA (LPCSTR lpSrc, LPSTR lpDst, DWORD nSize);
/// C: WINADVAPI BOOL WINAPI CryptAcquireContextA (HCRYPTPROV *phProv, LPCSTR szContainer, LPCSTR szProvider, DWORD dwProvType, DWORD dwFlags);
/// C: WINADVAPI BOOL WINAPI CryptGenRandom (HCRYPTPROV hProv, DWORD dwLen, BYTE *pbBuffer);
/// C: DWORD WINAPI WNetOpenEnumW (DWORD dwScope, DWORD dwType, DWORD dwUsage, LPNETRESOURCEW lpNetResource, LPHANDLE lphEnum);
/// C: DWORD WINAPI WNetOpenEnumA (DWORD dwScope, DWORD dwType, DWORD dwUsage, LPNETRESOURCEA lpNetResource, LPHANDLE lphEnum);
/// C: DWORD WINAPI WNetEnumResourceW (HANDLE hEnum, LPDWORD lpcCount, LPVOID lpBuffer, LPDWORD lpBufferSize);
/// C: DWORD WINAPI WNetEnumResourceA (HANDLE hEnum, LPDWORD lpcCount, LPVOID lpBuffer, LPDWORD lpBufferSize);
/// C: DWORD WINAPI WNetCloseEnum (HANDLE hEnum);
/// C: DWORD WINAPI WNetAddConnection2W (LPNETRESOURCEW lpNetResource, LPCWSTR lpPassword, LPCWSTR lpUserName, DWORD dwFlags);
/// C: DWORD WINAPI WNetAddConnection2A (LPNETRESOURCEA lpNetResource, LPCSTR lpPassword, LPCSTR lpUserName, DWORD dwFlags);
/// C: LPSTR WINAPI CharNextA (LPCSTR lpsz);
/// C: LPSTR WINAPI CharNextExA (WORD CodePage, LPCSTR lpCurrentChar, DWORD dwFlags);
/// C: LPWSTR WINAPI CharLowerW (LPWSTR lpsz);
/// C: DWORD WINAPI GetLocaleInfoA (LCID Locale, LCTYPE LCType, LPSTR lpLCData, int cchData);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn OpenProcess (dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
  pub fn ReadProcessMemory (hProcess: HANDLE, lpBaseAddress: *const c_void, lpBuffer: *mut c_void,
                            nSize: SIZE_T, lpNumberOfBytesRead: *mut SIZE_T) -> BOOL;
  pub fn CloseHandle (hObject: HANDLE) -> BOOL;
  pub fn GetLastError () -> DWORD;
  pub fn GetCurrentProcessId () -> DWORD;
  pub fn GetSystemInfo (lpSystemInfo: LPSYSTEM_INFO);
  pub fn GetSystemMetrics (nIndex: c_int) -> c_int;
  pub fn Sleep (dwMilliseconds: DWORD);
  pub fn GetTickCount () -> DWORD;
  pub fn GetDriveType (lpRootPathName: LPCSTR) -> UINT;
  pub fn GetFileAttributesA (lpFileName: LPCSTR) -> DWORD;
  pub fn GetFileAttributesW (lpFileName: LPCWSTR) -> DWORD;
  pub fn GetVolumeInformationA (lpRootPathName: LPCSTR, lpVolumeNameBuffer: LPSTR,
                                nVolumeNameSize: DWORD, lpVolumeSerialNumber: LPDWORD,
                                lpMaximumComponentLength: LPDWORD, lpFileSystemFlags: LPDWORD,
                                lpFileSystemNameBuffer: LPSTR, nFileSystemNameSize: DWORD) -> BOOL;
  pub fn GetVolumeInformationW (lpRootPathName: LPCWSTR, lpVolumeNameBuffer: LPWSTR,
                                nVolumeNameSize: DWORD, lpVolumeSerialNumber: LPDWORD,
                                lpMaximumComponentLength: LPDWORD, lpFileSystemFlags: LPDWORD,
                                lpFileSystemNameBuffer: LPWSTR, nFileSystemNameSize: DWORD) -> BOOL;
  pub fn GetShortPathNameA (lpszLongPath: LPCSTR, lpszShortPath: LPSTR, cchBuffer: DWORD) -> DWORD;
  pub fn GetShortPathNameW (lpszLongPath: LPCWSTR, lpszShortPath: LPWSTR, cchBuffer: DWORD) -> DWORD;
  pub fn FindFirstFileA (lpFileName: LPCSTR, lpFindFileData: *mut WIN32_FIND_DATAA) -> HANDLE;
  pub fn FindFirstFileW (lpFileName: LPCWSTR, lpFindFileData: *mut WIN32_FIND_DATAW) -> HANDLE;
  pub fn FindNextFileA (hFindFile: HANDLE, lpFindFileData: *mut WIN32_FIND_DATAA) -> BOOL;
  pub fn FindNextFileW (hFindFile: HANDLE, lpFindFileData: *mut WIN32_FIND_DATAW) -> BOOL;
  pub fn FindClose (hFindFile: HANDLE) -> BOOL;
  pub fn GetCurrentDirectoryA (nBufferLength: DWORD, lpBuffer: LPSTR) -> DWORD;
  pub fn GetCurrentDirectoryW (nBufferLength: DWORD, lpBuffer: LPWSTR) -> DWORD;
  pub fn GetModuleFileNameA (hModule: HMODULE, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
  pub fn GetUserName (lpBuffer: LPSTR, pcbBuffer: LPDWORD) -> BOOL;
  pub fn GetCPInfo (CodePage: UINT, lpCPInfo: *mut CPINFO) -> BOOL;
  pub fn ExpandEnvironmentStrings (lpSrc: LPCSTR, lpDst: LPSTR, nSize: DWORD) -> DWORD;
  pub fn CryptAcquireContext (phProv: *mut HCRYPTPROV, szContainer: LPCSTR, szProvider: LPCSTR,
                              dwProvType: DWORD, dwFlags: DWORD) -> BOOL;
  pub fn CryptGenRandom (hProv: HCRYPTPROV, dwLen: DWORD, pbBuffer: *mut BYTE) -> BOOL;
  pub fn WNetOpenEnumW (dwScope: DWORD, dwType: DWORD, dwUsage: DWORD,
                        lpNetResource: LPNETRESOURCEW, lphEnum: PHANDLE) -> DWORD;
  pub fn WNetOpenEnumA (dwScope: DWORD, dwType: DWORD, dwUsage: DWORD,
                        lpNetResource: LPNETRESOURCEA, lphEnum: PHANDLE) -> DWORD;
  pub fn WNetEnumResourceW (hEnum: HANDLE, lpcCount: LPDWORD, lpBuffer: LPVOID,
                            lpBufferSize: LPDWORD) -> DWORD;
  pub fn WNetEnumResourceA (hEnum: HANDLE, lpcCount: LPDWORD, lpBuffer: LPVOID,
                            lpBufferSize: LPDWORD) -> DWORD;
  pub fn WNetCloseEnum (hEnum: HANDLE) -> DWORD;
  pub fn WNetAddConnection2W (lpNetResource: LPNETRESOURCEW, lpPassword: LPCWSTR,
                              lpUserName: LPCWSTR, dwFlags: DWORD) -> DWORD;
  pub fn WNetAddConnection2A (lpNetResource: LPNETRESOURCEA, lpPassword: LPCSTR,
                              lpUserName: LPCSTR, dwFlags: DWORD) -> DWORD;
  pub fn CharNext (lpsz: LPCSTR) -> LPSTR;
  pub fn CharNextExA (CodePage: WORD, lpCurrentChar: LPCSTR, dwFlags: DWORD) -> LPSTR;
  pub fn CharLowerW (lpsz: LPWSTR) -> LPWSTR;
  pub fn GetLocaleInfo (Locale: LCID, LCType: DWORD, lpLCData: LPSTR, cchData: c_int) -> c_int;
}

/// C 1560/2810-2811/2903/3936: `alloca`'s four sites are PORTING.md P-05's stack pool
/// `sa_stack_alloc` (`femacs/src/alloc.rs` 1039), which `include/lisp.rs` 3978 already declares
/// (the macro form `USE_SAFE_ALLOCA`/`SAFE_ALLOCA!` sits at its 2991-3018), so this unit calls it
/// through that declaration and declares nothing of its own (the `check-mirror-dupes.py` rule the
/// `nt/addpm.rs` 55 / `nt/cmdproxy.rs` 288 / `nt/runemacs.rs` 320 sites satisfy the same way).
/// C: #define alloca __builtin_alloca

/// C 2963/1463: `strdup` and `difftime` - the runtime's and `<time.h>`'s own names, which no
/// mirror in this unit's graph carries (the ledger's owed rows).
/// C: char *strdup (const char *s);
/// C: double difftime (time_t time1, time_t time0);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn strdup (s: *const c_char) -> *mut c_char;
  pub fn difftime (time1: time_t, time0: time_t) -> f64;
}

/// C 3105/3122/3133: the four `_snprintf`/`_mbs*` CRT names and the `_putenv` this file's
/// environment layer calls; `unsigned char *` is `*mut c_uchar` and the two `*_mbs*` search
/// functions take a `char *` in the CRT's own text.
/// C: int _snprintf (char *buffer, size_t count, const char *format, ...);
/// C: int _strnicmp (const char *s1, const char *s2, size_t n);
/// C: unsigned char *_mbspbrk (const unsigned char *string, const unsigned char *charset);
/// C: char *_mbsrchr (const char *string, unsigned int c);
/// C: char *_mbschr (const char *string, unsigned int c);
/// C: unsigned char *_mbslwr (unsigned char *string);
/// C: int _putenv (const char *envstring);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _snprintf (buffer: *mut c_char, count: usize, format: *const c_char, ...) -> c_int;
  pub fn _strnicmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
  pub fn _mbspbrk (string: *const c_uchar, charset: *const c_uchar) -> *mut c_uchar;
  pub fn _mbsrchr (string: *const c_char, c: c_uint) -> *mut c_char;
  pub fn _mbschr (string: *const c_char, c: c_uint) -> *mut c_char;
  pub fn _mbslwr (string: *mut c_uchar) -> *mut c_uchar;
  pub fn _putenv (envstring: *const c_char) -> c_int;
}

/// C 2396-2397/2407: `_wgetenv` and the three wide-string helpers, whose `wchar_t` is the
/// Windows 16-bit one (`*mut c_ushort`, the section above's reading); `struct _utimbuf` is the
/// record C's own comment says is "buffer-equivalent" to `struct utimbuf`.
/// C: wchar_t *_wgetenv (const wchar_t *varname);
/// C: wchar_t *wcscat (wchar_t *dest, const wchar_t *src);
/// C: wchar_t *wcscpy (wchar_t *dest, const wchar_t *src);
/// C: wchar_t *wcsncpy (wchar_t *dest, const wchar_t *src, size_t n);
/// C: int _futime (int fd, struct _utimbuf *filetime);
/// C: struct _utimbuf { __time64_t actime; __time64_t modtime; };
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _wgetenv (varname: *const c_ushort) -> *mut c_ushort;
  pub fn wcscat (dest: *mut c_ushort, src: *const c_ushort) -> *mut c_ushort;
  pub fn wcscpy (dest: *mut c_ushort, src: *const c_ushort) -> *mut c_ushort;
  pub fn wcsncpy (dest: *mut c_ushort, src: *const c_ushort, n: usize) -> *mut c_ushort;
  pub fn _futime (fd: c_int, filetime: *mut _utimbuf) -> c_int;
}
/// C 3255-3260: `struct _utimbuf` - the record `_futime` takes; C's own comment records that it
/// keeps two spellings of it (`struct _utimbuf` and `struct utimbuf`), and this file uses this
/// one.  Its two members are `__time64_t` (the CRT's 64-bit `time_t` on this build).
/// C: struct _utimbuf { __time64_t actime; __time64_t modtime; };
/// C: typedef __int64 __time64_t;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _utimbuf
{
  pub actime: _time64_t,
  pub modtime: _time64_t,
}
#[cfg (feature = "windowsnt")]
pub type _time64_t = i64;

/// C 2258/2922/2924: `srand`/`rand` (`<stdlib.h>`'s own names, which `lib/include/stdlib.rs`
/// carries only `malloc`/`strtol` from) and `<lmcons.h>`'s two name-buffer lengths.
/// C: int srand (unsigned seed); int rand (void);
/// C: #define UNLEN 256
/// C: #define GNLEN 256
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn srand (seed: c_uint);
  pub fn rand () -> c_int;
}
#[cfg (feature = "windowsnt")]
pub const UNLEN: usize = 256;
#[cfg (feature = "windowsnt")]
pub const GNLEN: usize = 256;

/// C 2446: `DBL_EPSILON` - `<float.h>`'s own value, which `lib/include/float.in.rs` does not
/// carry (the ledger's owed row names the mirror round).
/// C: #define DBL_EPSILON 2.2204460492503131e-16
#[cfg (feature = "windowsnt")]
pub const DBL_EPSILON: f64 = 2.2204460492503131e-16;

/// C 2962/3100: `PATH_EXEC` - the generated `<epaths.h>` (C 96, recorded in the file note); the
/// value here is the frozen build's measured one (`femacs/src/callproc.rs` 275 keeps the same
/// constant for the same header), so a Windows build's own `epaths.h` is what a Windows target
/// supplies - the ledger records that this unit's copy is the frozen value.
/// C: #define PATH_EXEC "/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0"
#[cfg (feature = "windowsnt")]
pub const PATH_EXEC: &core::ffi::CStr
  = c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0";

/// C 2579/2582/2589/3537/3623/3778/4004/4127: `IS_DIRECTORY_SEP` - `configure.ac` 6493's
/// generated `<config.h>` macro for the non-Cygwin Windows build, carried as C's own expression
/// (the `femacs/lib-src/ntlib.rs` 522 reading of the same macro).
/// C: configure.ac 6493: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == '/' || (_c_) == '\\')`
#[cfg (feature = "windowsnt")]
macro_rules! IS_DIRECTORY_SEP {
  ($c:expr) => { (($c) == b'/' as c_int || ($c) == b'\\' as c_int) };
}

/// C 1540/1560/3145/3300/4007/4118: `INVALID_HANDLE_VALUE` - `<winnt.h>`'s `-1` handle, and the
/// two `<winreg.h>` value types `init_environment` switches on.
/// C: #define INVALID_HANDLE_VALUE ((HANDLE) (LONG_PTR) -1)
/// C: #define REG_SZ 1
/// C: #define REG_EXPAND_SZ 2
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
#[cfg (feature = "windowsnt")]
pub const REG_SZ: DWORD = 1;
#[cfg (feature = "windowsnt")]
pub const REG_EXPAND_SZ: DWORD = 2;

/// C 2067-2068: `getavg`'s two `long double` locals.  PORTING.md 1.1's `c_longdouble` is this
/// target's `c_double` (`src/editfns.rs` 197's `pub type c_longdouble = c_double;`, the same
/// measurement `lib-src/ntlib.rs` 81 records: `long double` and `double` are one width on every
/// platform the port supports), so the unit declares the alias with C's own type text.
/// C: long double sys = …; long double idl = …;
#[cfg (feature = "windowsnt")]
pub type c_longdouble = c_double;

/// C 274's `#include "dispextern.h"` supplies C's `INLINE int xstrcasecmp (…)`; that mirror keeps
/// its copy in a module the including unit does not glob-import (`include/dispextern.rs` 2594-2604,
/// so that a unit defining the name does not collide with it), which leaves each user to carry the
/// inline - exactly what `src/font.rs` does for the same name.  This unit's copy carries C's body.
/// C: INLINE int xstrcasecmp (char const *a, char const *b) { return c_strcasecmp (a, b); }
#[cfg (feature = "windowsnt")]
#[inline(always)]
pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int
{
  unsafe { c_strcasecmp (a, b) }
}

/// C 2277/2288/2295/2314: `<winnt.h>`'s access mask and the two `TOKEN_INFORMATION_CLASS`
/// members `init_user_info` asks for (`TokenUser` is 1, `TokenPrimaryGroup` 5 in the SDK's
/// enumeration, of which the section above already carries the `c_int`-sized alias).
/// C: #define TOKEN_QUERY (0x0008)
/// C: typedef enum _TOKEN_INFORMATION_CLASS { TokenUser = 1, TokenGroups, TokenPrivileges, …, TokenPrimaryGroup = 5, … } TOKEN_INFORMATION_CLASS;
#[cfg (feature = "windowsnt")]
pub const TOKEN_QUERY: DWORD = 0x0008;
#[cfg (feature = "windowsnt")]
pub const TokenUser: TOKEN_INFORMATION_CLASS = 1;
#[cfg (feature = "windowsnt")]
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = 5;

/// C 2230/3652/3798-3799/2606/2779: the four `<string.h>` names this slice needs that
/// `lib/include/string.rs` does not carry.
/// C: char *strcat (char *dest, const char *src);
/// C: char *strncpy (char *dest, const char *src, size_t n);
/// C: char *strncat (char *dest, const char *src, size_t n);
/// C: char *strpbrk (const char *s, const char *accept);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn strncpy (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn strncat (dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
  pub fn strpbrk (s: *const c_char, accept: *const c_char) -> *mut c_char;
}

/// C 2911-2918/3170: `cmd_error_internal` - `lisp.h`'s declaration, which `include/lisp.rs` does
/// not carry (the `femacs/src/w32select.rs` 647 reading of the same name), and `environ` - the
/// CRT's environment vector, which C 3177's loop walks.
/// C: lisp.h 5099: `extern void cmd_error_internal (Lisp_Object, const char *);`
/// C: extern char **environ;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn cmd_error_internal (error: Lisp_Object, msg: *const c_char);
  pub static mut environ: *mut *mut c_char;
}

/// C 2474: `SecureZeroMemory` - `<winnt.h>`'s `#define SecureZeroMemory RtlSecureZeroMemory`
/// (whose `memzero` the CRT implements without being optimizable away, which is the whole reason
/// `explicit_bzero` calls it).  The port's macro writes the same `$n` zero bytes through a
/// volatile pointer, which is the property the SDK's builtin provides.
/// C: #define SecureZeroMemory RtlSecureZeroMemory
#[cfg (feature = "windowsnt")]
macro_rules! SecureZeroMemory {
  ($p:expr, $n:expr) => {
    { let mut p_: *mut u8 = $p as *mut u8;
      let mut n_: usize = $n;
      while n_ > 0 { unsafe { core::ptr::write_volatile (p_, 0u8); p_ = p_.add (1); } n_ -= 1; } }
  };
}

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

/* C 1199-1284: `create_symbolic_link` - the first definition of this slice.  C's two
   function-local `static`s are function-local `static mut`s (the previous slice's reading), the
   two `wchar_t [MAX_PATH]` pairs are `[c_ushort; MAX_PATH]` (the Windows 16-bit wide character,
   the section above), and C's `filename_to_utf16`/`filename_to_ansi` calls name this file's own
   definitions (C 1766/1820).  `enable_privilege`/`restore_privilege`/`revert_to_self` are three
   of the forward-declared helpers whose definitions are past this slice (C 7316/7353/7166), so
   they stay unresolved names until those slices land (the ledger's P-91 (4) row).  C's
   `if (w32_unicode_filenames)` is the `bool` DEFVAR global of `w32term.c` 8357
   (`include/globals.rs` 7167), and C's `!retval` is the port's `retval == 0` (a `BOOLEAN`).  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn create_symbolic_link (lpSymlinkFilename: LPCSTR,
                                                lpTargetFileName: LPCSTR,
                                                dwFlags: DWORD) -> BOOLEAN
{
  static mut s_pfn_Create_Symbolic_LinkW: CreateSymbolicLinkW_Proc = None;    /* C 1203 */
  static mut s_pfn_Create_Symbolic_LinkA: CreateSymbolicLinkA_Proc = None;    /* C 1204 */
  let mut retval: BOOLEAN = 0;                                                /* C 1205 */

  if is_windows_9x () == TRUE                                                 /* C 1207 */
    {
      *errno () = ENOSYS;                                                     /* C 1209 */
      return 0;                                                               /* C 1210 */
    }
  if w32_unicode_filenames                                                    /* C 1212 */
    {
      let mut symfn_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 1214 */
      let mut tgtfn_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];

      if g_b_init_create_symbolic_link_w == 0                                 /* C 1216 */
        {
          g_b_init_create_symbolic_link_w = 1;                                /* C 1218 */
          s_pfn_Create_Symbolic_LinkW
            = std::mem::transmute::<VOIDFNPTR, CreateSymbolicLinkW_Proc>
                (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
                                c"CreateSymbolicLinkW".as_ptr ()));           /* C 1219-1221 */
        }
      if s_pfn_Create_Symbolic_LinkW.is_none ()                               /* C 1223 */
        {
          *errno () = ENOSYS;                                                 /* C 1225 */
          return 0;                                                           /* C 1226 */
        }

      filename_to_utf16 (lpSymlinkFilename, symfn_w.as_mut_ptr ());            /* C 1229 */
      filename_to_utf16 (lpTargetFileName, tgtfn_w.as_mut_ptr ());             /* C 1230 */
      retval = (s_pfn_Create_Symbolic_LinkW.unwrap ()) (symfn_w.as_mut_ptr (),
                                                        tgtfn_w.as_mut_ptr (),
                                                        dwFlags);              /* C 1231 */
      /* If we were denied creation of the symlink, try again after
	 enabling the SeCreateSymbolicLinkPrivilege for our process.  */
      if retval == 0                                                          /* C 1234 */
        {
          let mut priv_current: TOKEN_PRIVILEGES = core::mem::zeroed ();       /* C 1236 */

          if enable_privilege (SE_CREATE_SYMBOLIC_LINK_NAME.as_ptr (), TRUE,
                               &raw mut priv_current) != 0                     /* C 1238-1239 */
            {
              retval = (s_pfn_Create_Symbolic_LinkW.unwrap ()) (symfn_w.as_mut_ptr (),
                                                                tgtfn_w.as_mut_ptr (),
                                                                dwFlags);      /* C 1241 */
              restore_privilege (&raw mut priv_current);                       /* C 1242 */
              revert_to_self ();                                               /* C 1243 */
            }
        }
    }
  else
    {
      let mut symfn_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   /* C 1249 */
      let mut tgtfn_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];

      if g_b_init_create_symbolic_link_a == 0                                 /* C 1251 */
        {
          g_b_init_create_symbolic_link_a = 1;                                /* C 1253 */
          s_pfn_Create_Symbolic_LinkA
            = std::mem::transmute::<VOIDFNPTR, CreateSymbolicLinkA_Proc>
                (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
                                c"CreateSymbolicLinkA".as_ptr ()));           /* C 1254-1256 */
        }
      if s_pfn_Create_Symbolic_LinkA.is_none ()                               /* C 1258 */
        {
          *errno () = ENOSYS;                                                 /* C 1260 */
          return 0;                                                           /* C 1261 */
        }

      filename_to_ansi (lpSymlinkFilename, symfn_a.as_mut_ptr ());             /* C 1264 */
      filename_to_ansi (lpTargetFileName, tgtfn_a.as_mut_ptr ());              /* C 1265 */
      retval = (s_pfn_Create_Symbolic_LinkA.unwrap ()) (symfn_a.as_mut_ptr (),
                                                        tgtfn_a.as_mut_ptr (),
                                                        dwFlags);              /* C 1266 */
      /* If we were denied creation of the symlink, try again after
	 enabling the SeCreateSymbolicLinkPrivilege for our process.  */
      if retval == 0                                                          /* C 1269 */
        {
          let mut priv_current: TOKEN_PRIVILEGES = core::mem::zeroed ();       /* C 1271 */

          if enable_privilege (SE_CREATE_SYMBOLIC_LINK_NAME.as_ptr (), TRUE,
                               &raw mut priv_current) != 0                     /* C 1273-1274 */
            {
              retval = (s_pfn_Create_Symbolic_LinkA.unwrap ()) (symfn_a.as_mut_ptr (),
                                                                tgtfn_a.as_mut_ptr (),
                                                                dwFlags);      /* C 1276 */
              restore_privilege (&raw mut priv_current);                       /* C 1277 */
              revert_to_self ();                                               /* C 1278 */
            }
        }
    }
  return retval;                                                              /* C 1283 */
}

/* C 1285-1311: `is_valid_security_descriptor`.  C's `return s_pfn_… (pSecurityDescriptor);` is
   the port's `.unwrap ()` call-through (the previous slice's convention for a nullable C
   function pointer).  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn is_valid_security_descriptor (pSecurityDescriptor: PSECURITY_DESCRIPTOR)
  -> BOOL
{
  static mut s_pfn_Is_Valid_Security_Descriptor_Proc:
    IsValidSecurityDescriptor_Proc = None;                                    /* C 1289 */

  if is_windows_9x () == TRUE                                                 /* C 1291 */
    {
      *errno () = ENOTSUP;                                                    /* C 1293 */
      return FALSE;                                                           /* C 1294 */
    }

  if g_b_init_is_valid_security_descriptor == 0                               /* C 1297 */
    {
      g_b_init_is_valid_security_descriptor = 1;                              /* C 1299 */
      s_pfn_Is_Valid_Security_Descriptor_Proc
        = std::mem::transmute::<VOIDFNPTR, IsValidSecurityDescriptor_Proc>
            (get_proc_addr (GetModuleHandle (c"Advapi32.dll".as_ptr ()),
                            c"IsValidSecurityDescriptor".as_ptr ()));         /* C 1300-1302 */
    }
  if s_pfn_Is_Valid_Security_Descriptor_Proc.is_none ()                       /* C 1304 */
    {
      *errno () = ENOTSUP;                                                    /* C 1306 */
      return FALSE;                                                           /* C 1307 */
    }

  return (s_pfn_Is_Valid_Security_Descriptor_Proc.unwrap ()) (pSecurityDescriptor);  /* C 1310 */
}

/* C 1313-1357: `convert_sd_to_sddl`.  C 1332-1342's `#ifdef _UNICODE` pair is carried with both
   arms under `unicode` (P-51; the `LookupAccountSid_Name` precedent above), and C 1351-1355's
   call-through is the port's `.unwrap ()`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn convert_sd_to_sddl (SecurityDescriptor: PSECURITY_DESCRIPTOR,
                                              RequestedStringSDRevision: DWORD,
                                              SecurityInformation: SECURITY_INFORMATION,
                                              StringSecurityDescriptor: *mut LPTSTR,
                                              StringSecurityDescriptorLen: PULONG) -> BOOL
{
  static mut s_pfn_Convert_SD_To_SDDL:
    ConvertSecurityDescriptorToStringSecurityDescriptor_Proc = None;          /* C 1317 */
  let mut retval: BOOL = 0;                                                   /* C 1318 */

  if is_windows_9x () == TRUE                                                 /* C 1320 */
    {
      *errno () = ENOTSUP;                                                    /* C 1322 */
      return FALSE;                                                           /* C 1323 */
    }

  if g_b_init_convert_sd_to_sddl == 0                                         /* C 1326 */
    {
      g_b_init_convert_sd_to_sddl = 1;                                        /* C 1328 */
      #[cfg (feature = "unicode")]
      {
        s_pfn_Convert_SD_To_SDDL
          = std::mem::transmute::<VOIDFNPTR,
                                  ConvertSecurityDescriptorToStringSecurityDescriptor_Proc>
              (get_proc_addr (GetModuleHandle (c"Advapi32.dll".as_ptr ()),
                              c"ConvertSecurityDescriptorToStringSecurityDescriptorW"
                                .as_ptr ()));                                 /* C 1334-1336 */
      }
      #[cfg (not (feature = "unicode"))]
      {
        s_pfn_Convert_SD_To_SDDL
          = std::mem::transmute::<VOIDFNPTR,
                                  ConvertSecurityDescriptorToStringSecurityDescriptor_Proc>
              (get_proc_addr (GetModuleHandle (c"Advapi32.dll".as_ptr ()),
                              c"ConvertSecurityDescriptorToStringSecurityDescriptorA"
                                .as_ptr ()));                                 /* C 1339-1341 */
      }
    }
  if s_pfn_Convert_SD_To_SDDL.is_none ()                                      /* C 1344 */
    {
      *errno () = ENOTSUP;                                                    /* C 1346 */
      return FALSE;                                                           /* C 1347 */
    }

  retval = (s_pfn_Convert_SD_To_SDDL.unwrap ()) (SecurityDescriptor,
                                                 RequestedStringSDRevision,
                                                 SecurityInformation,
                                                 StringSecurityDescriptor,
                                                 StringSecurityDescriptorLen);  /* C 1351-1355 */

  return retval;                                                              /* C 1357 */
}

/* C 1359-1401: `convert_sddl_to_sd` - the mirror image of `convert_sd_to_sddl`, with the same
   `#ifdef _UNICODE` pair (C 1377-1387, both arms under `unicode`) and the same `.unwrap ()`
   call-through.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn convert_sddl_to_sd (StringSecurityDescriptor: LPCTSTR,
                                              StringSDRevision: DWORD,
                                              SecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
                                              SecurityDescriptorSize: PULONG) -> BOOL
{
  static mut s_pfn_Convert_SDDL_To_SD:
    ConvertStringSecurityDescriptorToSecurityDescriptor_Proc = None;          /* C 1363 */
  let mut retval: BOOL = 0;                                                   /* C 1364 */

  if is_windows_9x () == TRUE                                                 /* C 1366 */
    {
      *errno () = ENOTSUP;                                                    /* C 1368 */
      return FALSE;                                                           /* C 1369 */
    }

  if g_b_init_convert_sddl_to_sd == 0                                         /* C 1372 */
    {
      g_b_init_convert_sddl_to_sd = 1;                                        /* C 1374 */
      #[cfg (feature = "unicode")]
      {
        s_pfn_Convert_SDDL_To_SD
          = std::mem::transmute::<VOIDFNPTR,
                                  ConvertStringSecurityDescriptorToSecurityDescriptor_Proc>
              (get_proc_addr (GetModuleHandle (c"Advapi32.dll".as_ptr ()),
                              c"ConvertStringSecurityDescriptorToSecurityDescriptorW"
                                .as_ptr ()));                                 /* C 1379-1381 */
      }
      #[cfg (not (feature = "unicode"))]
      {
        s_pfn_Convert_SDDL_To_SD
          = std::mem::transmute::<VOIDFNPTR,
                                  ConvertStringSecurityDescriptorToSecurityDescriptor_Proc>
              (get_proc_addr (GetModuleHandle (c"Advapi32.dll".as_ptr ()),
                              c"ConvertStringSecurityDescriptorToSecurityDescriptorA"
                                .as_ptr ()));                                 /* C 1384-1386 */
      }
    }
  if s_pfn_Convert_SDDL_To_SD.is_none ()                                      /* C 1389 */
    {
      *errno () = ENOTSUP;                                                    /* C 1391 */
      return FALSE;                                                           /* C 1392 */
    }

  retval = (s_pfn_Convert_SDDL_To_SD.unwrap ()) (StringSecurityDescriptor,
                                                 StringSDRevision,
                                                 SecurityDescriptor,
                                                 SecurityDescriptorSize);     /* C 1395-1398 */

  return retval;                                                              /* C 1400 */
}

/* C 1403-1423: `get_adapters_info`.  This one loads `Iphlpapi.dll` through `LoadLibrary` (not
   `GetModuleHandle`), and its `hm_iphlpapi` is C's `HMODULE hm_iphlpapi = NULL;`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_adapters_info (pAdapterInfo: PIP_ADAPTER_INFO,
                                             pOutBufLen: PULONG) -> DWORD
{
  static mut s_pfn_Get_Adapters_Info: GetAdaptersInfo_Proc = None;            /* C 1407 */
  let mut hm_iphlpapi: HMODULE = core::ptr::null_mut ();                      /* C 1408 */

  if is_windows_9x () == TRUE                                                 /* C 1410 */
    {
      return ERROR_NOT_SUPPORTED;                                             /* C 1411 */
    }

  if g_b_init_get_adapters_info == 0                                          /* C 1413 */
    {
      g_b_init_get_adapters_info = 1;                                         /* C 1415 */
      hm_iphlpapi = LoadLibrary (c"Iphlpapi.dll".as_ptr ());                  /* C 1416 */
      if !hm_iphlpapi.is_null ()                                              /* C 1417 */
        {
          s_pfn_Get_Adapters_Info
            = std::mem::transmute::<VOIDFNPTR, GetAdaptersInfo_Proc>
                (get_proc_addr (hm_iphlpapi, c"GetAdaptersInfo".as_ptr ()));  /* C 1418-1419 */
        }
    }
  if s_pfn_Get_Adapters_Info.is_none ()                                       /* C 1421 */
    {
      return ERROR_NOT_SUPPORTED;                                             /* C 1422 */
    }
  return (s_pfn_Get_Adapters_Info.unwrap ()) (pAdapterInfo, pOutBufLen);      /* C 1423 */
}

/* C 1425-1448: `get_adapters_addresses`.  C's mid-block `ULONG flags = …` declaration follows the
   two guards, and its `NULL` argument is the port's null pointer.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_adapters_addresses (family: ULONG,
                                                  pAdapterAddresses: PIP_ADAPTER_ADDRESSES,
                                                  pOutBufLen: PULONG) -> DWORD
{
  static mut s_pfn_Get_Adapters_Addresses: GetAdaptersAddresses_Proc = None;  /* C 1429 */
  let mut hm_iphlpapi: HMODULE = core::ptr::null_mut ();                      /* C 1430 */

  if is_windows_9x () == TRUE                                                 /* C 1432 */
    {
      return ERROR_NOT_SUPPORTED;                                             /* C 1433 */
    }

  if g_b_init_get_adapters_addresses == 0                                     /* C 1435 */
    {
      g_b_init_get_adapters_addresses = 1;                                    /* C 1437 */
      hm_iphlpapi = LoadLibrary (c"Iphlpapi.dll".as_ptr ());                  /* C 1438 */
      if !hm_iphlpapi.is_null ()                                              /* C 1439 */
        {
          s_pfn_Get_Adapters_Addresses
            = std::mem::transmute::<VOIDFNPTR, GetAdaptersAddresses_Proc>
                (get_proc_addr (hm_iphlpapi, c"GetAdaptersAddresses".as_ptr ()));  /* C 1440-1441 */
        }
    }
  if s_pfn_Get_Adapters_Addresses.is_none ()                                  /* C 1443 */
    {
      return ERROR_NOT_SUPPORTED;                                             /* C 1444 */
    }
  let mut flags: ULONG = GAA_FLAG_SKIP_ANYCAST                               /* C 1445-1447 */
    | GAA_FLAG_SKIP_MULTICAST
    | GAA_FLAG_SKIP_DNS_SERVER;
  return (s_pfn_Get_Adapters_Addresses.unwrap ()) (family, flags,
                                                   core::ptr::null_mut (),
                                                   pAdapterAddresses, pOutBufLen);  /* C 1448 */
}

/* C 1450-1472: `reg_open_key_ex_w`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn reg_open_key_ex_w (hkey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD,
                                             samDesired: REGSAM, phkResult: PHKEY) -> LONG
{
  static mut s_pfn_Reg_Open_Key_Ex_w: RegOpenKeyExW_Proc = None;              /* C 1454 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                      /* C 1455 */

  if is_windows_9x () == TRUE                                                 /* C 1457 */
    {
      return ERROR_NOT_SUPPORTED as LONG;                                     /* C 1458 */
    }

  if g_b_init_reg_open_key_ex_w == 0                                          /* C 1460 */
    {
      g_b_init_reg_open_key_ex_w = 1;                                         /* C 1462 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                  /* C 1463 */
      if !hm_advapi32.is_null ()                                              /* C 1464 */
        {
          s_pfn_Reg_Open_Key_Ex_w
            = std::mem::transmute::<VOIDFNPTR, RegOpenKeyExW_Proc>
                (get_proc_addr (hm_advapi32, c"RegOpenKeyExW".as_ptr ()));    /* C 1465-1466 */
        }
    }
  if s_pfn_Reg_Open_Key_Ex_w.is_none ()                                       /* C 1468 */
    {
      return ERROR_NOT_SUPPORTED as LONG;                                     /* C 1469 */
    }
  return (s_pfn_Reg_Open_Key_Ex_w.unwrap ()) (hkey, lpSubKey, ulOptions,
                                              samDesired, phkResult);         /* C 1470-1471 */
}

/* C 1474-1496: `reg_query_value_ex_w`.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn reg_query_value_ex_w (hkey: HKEY, lpValueName: LPCWSTR,
                                               lpReserved: LPDWORD, lpType: LPDWORD,
                                               lpData: LPBYTE, lpcbData: LPDWORD) -> LONG
{
  static mut s_pfn_Reg_Query_Value_Ex_w: RegQueryValueExW_Proc = None;        /* C 1479 */
  let mut hm_advapi32: HMODULE = core::ptr::null_mut ();                      /* C 1480 */

  if is_windows_9x () == TRUE                                                 /* C 1482 */
    {
      return ERROR_NOT_SUPPORTED as LONG;                                     /* C 1483 */
    }

  if g_b_init_reg_query_value_ex_w == 0                                       /* C 1485 */
    {
      g_b_init_reg_query_value_ex_w = 1;                                      /* C 1487 */
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());                  /* C 1488 */
      if !hm_advapi32.is_null ()                                              /* C 1489 */
        {
          s_pfn_Reg_Query_Value_Ex_w
            = std::mem::transmute::<VOIDFNPTR, RegQueryValueExW_Proc>
                (get_proc_addr (hm_advapi32, c"RegQueryValueExW".as_ptr ()));  /* C 1490-1491 */
        }
    }
  if s_pfn_Reg_Query_Value_Ex_w.is_none ()                                    /* C 1493 */
    {
      return ERROR_NOT_SUPPORTED as LONG;                                     /* C 1494 */
    }
  return (s_pfn_Reg_Query_Value_Ex_w.unwrap ()) (hkey, lpValueName, lpReserved,
                                                 lpType, lpData, lpcbData);   /* C 1495-1496 */
}

/* C 1498-1521: `expand_environment_strings_w`, the one wrapper of this group whose failure arm
   stores `ENOSYS` into `errno` and returns `FALSE` (a `DWORD` return, so C's `FALSE` converts).  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn expand_environment_strings_w (lpSrc: LPCWSTR, lpDst: LPWSTR,
                                                        nSize: DWORD) -> DWORD
{
  static mut s_pfn_Expand_Environment_Strings_w: ExpandEnvironmentStringsW_Proc = None;  /* C 1502 */
  let mut hm_kernel32: HMODULE = core::ptr::null_mut ();                      /* C 1503 */

  if is_windows_9x () == TRUE                                                 /* C 1505 */
    {
      return ERROR_NOT_SUPPORTED;                                             /* C 1506 */
    }

  if g_b_init_expand_environment_strings_w == 0                              /* C 1508 */
    {
      g_b_init_expand_environment_strings_w = 1;                              /* C 1510 */
      hm_kernel32 = LoadLibrary (c"Kernel32.dll".as_ptr ());                  /* C 1511 */
      if !hm_kernel32.is_null ()                                              /* C 1512 */
        {
          s_pfn_Expand_Environment_Strings_w
            = std::mem::transmute::<VOIDFNPTR, ExpandEnvironmentStringsW_Proc>
                (get_proc_addr (hm_kernel32,
                                c"ExpandEnvironmentStringsW".as_ptr ()));     /* C 1513-1514 */
        }
    }
  if s_pfn_Expand_Environment_Strings_w.is_none ()                            /* C 1516 */
    {
      *errno () = ENOSYS;                                                     /* C 1518 */
      return FALSE as DWORD;                                                  /* C 1519 */
    }
  return (s_pfn_Expand_Environment_Strings_w.unwrap ()) (lpSrc, lpDst, nSize);  /* C 1521 */
}

/* C 1523-1543: `get_user_default_ui_language` - the last of the wrapper group, with C's two
   `return 0;` arms.  */
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_user_default_ui_language () -> LANGID
{
  static mut s_pfn_GetUserDefaultUILanguage: GetUserDefaultUILanguage_Proc = None;  /* C 1527 */
  let mut hm_kernel32: HMODULE = core::ptr::null_mut ();                      /* C 1528 */

  if is_windows_9x () == TRUE                                                 /* C 1530 */
    {
      return 0;                                                               /* C 1531 */
    }

  if g_b_init_get_user_default_ui_language == 0                               /* C 1533 */
    {
      g_b_init_get_user_default_ui_language = 1;                              /* C 1535 */
      hm_kernel32 = LoadLibrary (c"Kernel32.dll".as_ptr ());                  /* C 1536 */
      if !hm_kernel32.is_null ()                                              /* C 1537 */
        {
          s_pfn_GetUserDefaultUILanguage
            = std::mem::transmute::<VOIDFNPTR, GetUserDefaultUILanguage_Proc>
                (get_proc_addr (hm_kernel32,
                                c"GetUserDefaultUILanguage".as_ptr ()));      /* C 1538-1539 */
        }
    }
  if s_pfn_GetUserDefaultUILanguage.is_none ()                                /* C 1541 */
    {
      return 0;                                                               /* C 1542 */
    }
  return (s_pfn_GetUserDefaultUILanguage.unwrap ()) ();                       /* C 1543 */
}

/* C 1545-1567: `w32_valid_pointer_p` - "Return 1 if P is a valid pointer to an object of size
   SIZE.  Return 0 if P is NOT a valid pointer.  Return -1 if we cannot validate P.  This is
   called from alloc.c:valid_pointer_p."  C's `alloca (size)` is PORTING.md P-05's stack pool and
   C's uninitialized `SIZE_T done` is zeroed (PORTING.md 1.5).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_valid_pointer_p (p: *mut c_void, size: c_int) -> c_int
{
  let mut done: SIZE_T = 0;                                                   /* C 1555 */
  let h: HANDLE = OpenProcess (PROCESS_VM_READ, FALSE, GetCurrentProcessId ());  /* C 1556 */

  if !h.is_null ()                                                            /* C 1558 */
    {
      let buf: *mut c_uchar = sa_stack_alloc (size as usize) as *mut c_uchar;  /* C 1560 */
      let retval: c_int = ReadProcessMemory (h, p, buf as *mut c_void, size as SIZE_T,
                                            &raw mut done);                   /* C 1561 */

      CloseHandle (h);                                                        /* C 1563 */
      return retval;                                                          /* C 1564 */
    }
  else
    {
      return -1;                                                              /* C 1567 */
    }
}



/* Here's an overview of how the Windows build supports file names
   that cannot be encoded by the current system codepage.

   From the POV of Lisp and layers of C code above the functions here,
   Emacs on Windows pretends that its file names are encoded in UTF-8;
   see encode_file and decode_file on coding.c.  Any file name that is
   passed as a unibyte string to C functions defined here is assumed
   to be in UTF-8 encoding.  Any file name returned by functions
   defined here must be in UTF-8 encoding, with only a few exceptions
   reserved for a couple of special cases.  (Be sure to use
   MAX_UTF8_PATH for char arrays that store UTF-8 encoded file names,
   as they can be much longer than MAX_PATH!)

   The UTF-8 encoded file names cannot be passed to system APIs, as
   Windows does not support that.  Therefore, they are converted
   either to UTF-16 or to the ANSI codepage, depending on the value of
   w32-unicode-filenames, before calling any system APIs or CRT library
   functions.  The default value of that variable is determined by the
   OS on which Emacs runs: nil on Windows 9X and t otherwise, but the
   user can change that default (although I don't see why would she
   want to).

   The 4 functions defined below, filename_to_utf16, filename_to_ansi,
   filename_from_utf16, and filename_from_ansi, are the workhorses of
   these conversions.  They rely on Windows native APIs
   MultiByteToWideChar and WideCharToMultiByte; we cannot use
   functions from coding.c here, because they allocate memory, which
   is a bad idea on the level of libc, which is what the functions
   here emulate.  (If you worry about performance due to constant
   conversion back and forth from UTF-8 to UTF-16, then don't: first,
   it was measured to take only a few microseconds on a not-so-fast
   machine, and second, that's exactly what the ANSI APIs we used
   before did anyway, because they are just thin wrappers around the
   Unicode APIs.)

   The variables file-name-coding-system and default-file-name-coding-system
   still exist, but are actually used only when a file name needs to
   be converted to the ANSI codepage.  This happens all the time when
   w32-unicode-filenames is nil, but can also happen from time to time
   when it is t.  Otherwise, these variables have no effect on file-name
   encoding when w32-unicode-filenames is t; this is similar to
   selection-coding-system.

   This arrangement works very well, but it has a few gotchas and
   limitations:

   . Lisp code that encodes or decodes file names manually should
     normally use 'utf-8' as the coding-system on Windows,
     disregarding file-name-coding-system.  This is a somewhat
     unpleasant consequence, but it cannot be avoided.  Fortunately,
     very few Lisp packages need to do that.

     More generally, passing to library functions (e.g., fopen or
     opendir) file names already encoded in the ANSI codepage is
     explicitly *verboten*, as all those functions, as shadowed and
     emulated here, assume they will receive UTF-8 encoded file names.

     For the same reasons, no CRT function or Win32 API can be called
     directly in Emacs sources, without either converting the file
     names from UTF-8 to UTF-16 or ANSI codepage, or going through
     some shadowing function defined here.

   . Environment variables stored in Vprocess_environment are encoded
     in the ANSI codepage, so if getenv/egetenv is used for a variable
     whose value is a file name or a list of directories, it needs to
     be converted to UTF-8, before it is used as argument to functions
     or decoded into a Lisp string.

   . File names passed to external libraries, like the image libraries
     and GnuTLS, need special handling.  These libraries generally
     don't support UTF-16 or UTF-8 file names, so they must get file
     names encoded in the ANSI codepage.  To facilitate using these
     libraries with file names that are not encodable in the ANSI
     codepage, use the function ansi_encode_filename, which will try
     to use the short 8+3 alias of a file name if that file name is
     not encodable in the ANSI codepage.  See image.c and gnutls.c for
     examples of how this should be done.

   . Running subprocesses in non-ASCII directories and with non-ASCII
     file arguments is limited to the current codepage (even though
     Emacs is perfectly capable of finding an executable program file
     in a directory whose name cannot be encoded in the current
     codepage).  This is because the command-line arguments are
     encoded _before_ they get to the w32-specific level, and the
     encoding is not known in advance (it doesn't have to be the
     current ANSI codepage), so w32proc.c functions cannot re-encode
     them in UTF-16.  This should be fixed, but will also require
     changes in cmdproxy.  The current limitation is not terribly bad
     anyway, since very few, if any, Windows console programs that are
     likely to be invoked by Emacs support UTF-16 encoded command
     lines.

   . For similar reasons, server.el and emacsclient are also limited
     to the current ANSI codepage for now.

   . Emacs itself can only handle command-line arguments encoded in
     the current codepage.

   . Turning on w32-unicode-filename on Windows 9X (if it at all
     works) requires UNICOWS.DLL, which is thus a requirement even in
     non-GUI sessions, something that we previously avoided.  */

/* C 1673-1676: `/* Converting file names from UTF-8 to either UTF-16 or the ANSI codepage
   defined by file-name-coding-system.  */` */
/* C 1678-1679: `/* Current codepage for encoding file names.  */` and `static int
   file_name_codepage;` - C's file-scope `static`, so a non-`pub` `static mut` (PORTING.md 0.2),
   zero-initialized as C's tentative definition is.  */
#[cfg (feature = "windowsnt")]
static mut file_name_codepage: c_int = 0;

/* C 1681-1702: `w32_init_file_name_codepage`, whose C 1689-1701 `#ifdef HAVE_PDUMPER` arm is
   live (`include/config.rs` 12 carries the constant as `true`) and has no `#else` - nothing is
   carried for the other side (the `fns.rs` C 5383 reading).  C's `w32_ansi_code_page = CP_ACP`
   is the `EMACS_INT` DEFVAR global of `w32fns.c` 12414 (`include/globals.rs` 7149), so the
   `int`→`EMACS_INT` conversion is written out.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_init_file_name_codepage ()
{
  file_name_codepage = CP_ACP;                                                /* C 1687 */
  w32_ansi_code_page = CP_ACP as EMACS_INT;                                   /* C 1688 */
  /* C 1689-1701: `#ifdef HAVE_PDUMPER` - live.  */
  Vdefault_file_name_coding_system = Qnil ();                                 /* C 1700 */
  Vfile_name_coding_system = Qnil ();                                         /* C 1701 */
}

/* C 1704-1763: `codepage_for_filenames`.  C's two function-local `static`s (a `Lisp_Object` and
   a `CPINFO cp`) are function-local `static mut`s, the `CPINFO` static's zero value is spelled
   out (no `mem::zeroed` in a static initializer), C's uninitialized `char *end` is zeroed
   (PORTING.md 1.5), C's `strtol` result narrows to `int`, and `emacs_abort` is the port's
   `-> !`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn codepage_for_filenames (cp_info: *mut CPINFO) -> c_int
{
  static mut last_file_name_encoding: Lisp_Object = core::ptr::null_mut ();   /* C 1712 */
  static mut cp: CPINFO = CPINFO { MaxCharSize: 0,
                                   DefaultChar: [0; MAX_DEFAULTCHAR],
                                   LeadByte: [0; MAX_LEADBYTES] };            /* C 1713 */
  let mut current_encoding: Lisp_Object;                                      /* C 1714 */

  current_encoding = Vfile_name_coding_system;                                /* C 1716 */
  if NILP (current_encoding)                                                   /* C 1717 */
    {
      current_encoding = Vdefault_file_name_coding_system;                    /* C 1718 */
    }

  if !EQ (last_file_name_encoding, current_encoding)                           /* C 1720-1721 */
      || NILP (last_file_name_encoding)
    {
      /* Default to the current ANSI codepage.  */
      file_name_codepage = w32_ansi_code_page as c_int;                       /* C 1725 */

      if !NILP (current_encoding)                                              /* C 1727 */
        {
          let cpname: *mut c_char = SSDATA (SYMBOL_NAME (current_encoding));  /* C 1729 */
          let mut cp_: *mut c_char = core::ptr::null_mut ();  /* C 1730 (C's local `cp`, renamed: it shadows the static `cp` above, which Rust forbids - E0530) */
          let mut end: *mut c_char = core::ptr::null_mut ();
          let mut cpnum: c_int;                                               /* C 1731 */

          if strncmp (cpname, c"cp".as_ptr (), 2) == 0                        /* C 1733 */
            {
              cp_ = cpname.offset (2);                                        /* C 1734 */
            }
          else if strncmp (cpname, c"windows-".as_ptr (), 8) == 0             /* C 1735 */
            {
              cp_ = cpname.offset (8);                                        /* C 1736 */
            }

          if !cp_.is_null ()                                                  /* C 1738 */
            {
              end = cp_;                                                      /* C 1740 */
              cpnum = strtol (cp_, &raw mut end, 10) as c_int;                /* C 1741 */
              if cpnum != 0 && *end == 0 && end.offset_from (cp_) >= 2        /* C 1742 */
                {
                  file_name_codepage = cpnum;                                 /* C 1743 */
                }
            }
        }

      if file_name_codepage == 0                                              /* C 1747 */
        {
          file_name_codepage = CP_ACP; /* CP_ACP = 0, but let's not assume that */  /* C 1748 */
        }

      if GetCPInfo (file_name_codepage as UINT, &raw mut cp) == 0             /* C 1750 */
        {
          file_name_codepage = CP_ACP;                                        /* C 1752 */
          if GetCPInfo (file_name_codepage as UINT, &raw mut cp) == 0         /* C 1753 */
            {
              emacs_abort ();                                                 /* C 1754 */
            }
        }

      /* Cache the new value.  */
      last_file_name_encoding = current_encoding;                             /* C 1758 */
    }
  if !cp_info.is_null ()                                                      /* C 1760 */
    {
      *cp_info = cp;                                                          /* C 1761 */
    }

  return file_name_codepage;                                                  /* C 1763 */
}

/* C 1765-1790: `filename_to_utf16` (`src/w32.c` 1766).  C calls the loaded entry point directly;
   the port's `pMultiByteToWideChar` is an `Option<…>` (the previous slice's declaration), so the
   call-through is `.unwrap () (…)` - the file's convention for every loaded pointer.  C's
   `switch (err)` is the port's `match` with C's case groups and its `default`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename_to_utf16 (fn_in: *const c_char, fn_out: *mut c_ushort) -> c_int
{
  let mut result: c_int = (pMultiByteToWideChar.unwrap ()) (CP_UTF8 as UINT,
                                                            multiByteToWideCharFlags as c_ulong,
                                                            fn_in, -1, fn_out,
                                                            MAX_PATH);          /* C 1768-1769 */

  if result == 0                                                              /* C 1771 */
    {
      let err: DWORD = GetLastError ();                                       /* C 1773 */

      match err                                                               /* C 1775-1783 */
        {
          ERROR_INVALID_FLAGS | ERROR_INVALID_PARAMETER =>
            { *errno () = EINVAL; }                                           /* C 1777-1778 */
          ERROR_INSUFFICIENT_BUFFER | ERROR_NO_UNICODE_TRANSLATION =>
            { *errno () = ENOENT; }                                           /* C 1780-1781 */
          _ =>
            { *errno () = ENOENT; }                                           /* C 1782 */
        }
      return -1;                                                              /* C 1784 */
    }
  return 0;                                                                   /* C 1786 */
}

/* C 1792-1817: `filename_from_utf16` - C's `NULL, NULL` tail (`lpDefaultChar`, `lpUsedDefaultChar`)
   and its `MAX_UTF8_PATH` output bound.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename_from_utf16 (fn_in: *const c_ushort, fn_out: *mut c_char) -> c_int
{
  let mut result: c_int = (pWideCharToMultiByte.unwrap ()) (CP_UTF8 as UINT, 0, fn_in, -1,
                                                            fn_out, MAX_UTF8_PATH,
                                                            core::ptr::null (),
                                                            core::ptr::null_mut ());  /* C 1795-1796 */

  if result == 0                                                              /* C 1798 */
    {
      let err: DWORD = GetLastError ();                                       /* C 1800 */

      match err                                                               /* C 1802-1810 */
        {
          ERROR_INVALID_FLAGS | ERROR_INVALID_PARAMETER =>
            { *errno () = EINVAL; }
          ERROR_INSUFFICIENT_BUFFER | ERROR_NO_UNICODE_TRANSLATION =>
            { *errno () = ENOENT; }
          _ =>
            { *errno () = ENOENT; }
        }
      return -1;                                                              /* C 1811 */
    }
  return 0;                                                                   /* C 1813 */
}

/* C 1819-1852: `filename_to_ansi` - the two-step conversion (UTF-8 → UTF-16 → the codepage
   `codepage_for_filenames` reports), whose `MAX_PATH` bound is C's.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename_to_ansi (fn_in: *const c_char, fn_out: *mut c_char) -> c_int
{
  let mut fn_utf16: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];   /* C 1822 */

  if filename_to_utf16 (fn_in, fn_utf16.as_mut_ptr ()) == 0                   /* C 1824 */
    {
      let mut result: c_int;                                                  /* C 1826 */
      let codepage: c_int = codepage_for_filenames (core::ptr::null_mut ());  /* C 1827 */

      result = (pWideCharToMultiByte.unwrap ()) (codepage as UINT, 0, fn_utf16.as_mut_ptr (),
                                                 -1, fn_out, MAX_PATH,
                                                 core::ptr::null (),
                                                 core::ptr::null_mut ());     /* C 1829-1830 */
      if result == 0                                                          /* C 1831 */
        {
          let err: DWORD = GetLastError ();                                   /* C 1833 */

          match err                                                           /* C 1835-1843 */
            {
              ERROR_INVALID_FLAGS | ERROR_INVALID_PARAMETER =>
                { *errno () = EINVAL; }
              ERROR_INSUFFICIENT_BUFFER | ERROR_NO_UNICODE_TRANSLATION =>
                { *errno () = ENOENT; }
              _ =>
                { *errno () = ENOENT; }
            }
          return -1;                                                          /* C 1844 */
        }
      return 0;                                                               /* C 1846 */
    }
  return -1;                                                                  /* C 1848 */
}

/* C 1854-1881: `filename_from_ansi` - the reverse pair, ending in
   `filename_from_utf16 (fn_utf16, fn_out)`'s return value.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename_from_ansi (fn_in: *const c_char, fn_out: *mut c_char) -> c_int
{
  let mut fn_utf16: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];   /* C 1857 */
  let codepage: c_int = codepage_for_filenames (core::ptr::null_mut ());      /* C 1858 */
  let mut result: c_int = (pMultiByteToWideChar.unwrap ()) (codepage as UINT,
                                                            multiByteToWideCharFlags as c_ulong,
                                                            fn_in, -1,
                                                            fn_utf16.as_mut_ptr (),
                                                            MAX_PATH);        /* C 1859-1860 */

  if result == 0                                                              /* C 1862 */
    {
      let err: DWORD = GetLastError ();                                       /* C 1864 */

      match err                                                               /* C 1866-1874 */
        {
          ERROR_INVALID_FLAGS | ERROR_INVALID_PARAMETER =>
            { *errno () = EINVAL; }
          ERROR_INSUFFICIENT_BUFFER | ERROR_NO_UNICODE_TRANSLATION =>
            { *errno () = ENOENT; }
          _ =>
            { *errno () = ENOENT; }
        }
      return -1;                                                              /* C 1875 */
    }
  return filename_from_utf16 (fn_utf16.as_mut_ptr (), fn_out);                /* C 1877 */
}

/* C 1883-1885: `/* The directory where we started, in UTF-8. */` and `static char
   startup_dir[MAX_UTF8_PATH];` - C's file-scope `static`, so a non-`pub` `static mut`
   (PORTING.md 0.2).  */
#[cfg (feature = "windowsnt")]
static mut startup_dir: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];

/* C 1887-1912: `w32_get_current_directory`.  C's `!GetCurrentDirectoryW (…, …)` truth test and
   its two `emacs_abort ()` arms are the port's `== 0` and the port's `-> !`.  */
#[cfg (feature = "windowsnt")]
unsafe fn w32_get_current_directory (cwd: *mut c_char)
{
  /* FIXME: Do we need to resolve possible symlinks in startup_dir?
     Does it matter anywhere in Emacs?  */
  if w32_unicode_filenames                                                    /* C 1896 */
    {
      let mut wstartup_dir: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 1898 */

      if GetCurrentDirectoryW (MAX_PATH as DWORD, wstartup_dir.as_mut_ptr ()) == 0  /* C 1900 */
        {
          emacs_abort ();                                                     /* C 1901 */
        }
      filename_from_utf16 (wstartup_dir.as_mut_ptr (), cwd);                   /* C 1902 */
    }
  else
    {
      let mut astartup_dir: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 1906 */

      if GetCurrentDirectoryA (MAX_PATH as DWORD, astartup_dir.as_mut_ptr ()) == 0  /* C 1908 */
        {
          emacs_abort ();                                                     /* C 1909 */
        }
      filename_from_ansi (astartup_dir.as_mut_ptr (), cwd);                    /* C 1910 */
    }
}

/* C 1914-1919: `w32_init_current_directory` - "For external callers.  Used by 'main' in
   emacs.c."  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_init_current_directory ()
{
  w32_get_current_directory (startup_dir.as_mut_ptr ());                      /* C 1918 */
}

/* C 1921-1944: `getcwd`.  C's `#if 0` arm (C 1935-1940, `GetCurrentDirectory`) is dead in every
   configuration, so it is carried as C's own text and `GetCurrentDirectoryA` is declared with
   the section above (`tools/extract-abi.py`'s `dead_branch_map` reads the literal `#if 0` as the
   reference's own dead region - the `ccl.rs` 2599 / `character.rs` 118 reading).
   C: #if 0
   C:   if (GetCurrentDirectory (MAXPATHLEN, dir) > 0)
   C:     return dir;
   C:   return NULL;
   C: #else  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getcwd (dir: *mut c_char, dirsize: c_int) -> *mut c_char
{
  if dirsize == 0                                                             /* C 1925 */
    {
      *errno () = EINVAL;                                                     /* C 1927 */
      return core::ptr::null_mut ();                                          /* C 1928 */
    }
  if (dirsize as usize) <= strlen (startup_dir.as_ptr ())                     /* C 1930 */
    {
      *errno () = ERANGE;                                                     /* C 1932 */
      return core::ptr::null_mut ();                                          /* C 1933 */
    }
  /* C 1941-1942: "Emacs doesn't actually change directory itself, it stays in the same directory
     where it was started."  */
  strcpy (dir, startup_dir.as_ptr ());                                        /* C 1942 */
  return dir;                                                                 /* C 1943 */
}

/* C 1946-1953: `/* Emulate getloadavg.  */` and `struct load_sample` - C's four members in
   C's order (`time_t` is `lib/include/time.in.rs` 36's `i64` and `ULONGLONG` is `u64`).  */
/* C: struct load_sample {
   C:   time_t sample_time;
   C:   ULONGLONG idle;
   C:   ULONGLONG kernel;
   C:   ULONGLONG user;
   C: };  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct load_sample
{
  pub sample_time: time_t,
  pub idle: u64,
  pub kernel: u64,
  pub user: u64,
}

/* C 1955-1962: `/* Number of processors on this machine.  */`, `static unsigned
   num_of_processors;`, the 16-minute circular buffer `samples[16*60]`, the two indices and
   `max_idx = countof (samples)`.  The `countof` macro is `lib/include/stdcountof.rs`
   (`lisp.h` 28's own `#include <stdcountof.h>`), which this unit's include graph reaches by the
   edge recorded in the ledger's Deviations.  */
#[cfg (feature = "windowsnt")]
static mut num_of_processors: c_uint = 0;                                     /* C 1956 */
/// C 1959's `samples[16*60]` and C 1962's `countof (samples)`: the length is a const because a
/// `static mut` may not be read in a static initializer (Rust's `E0133`), and C's own `16*60` is
/// the expression the port keeps.
#[cfg (feature = "windowsnt")]
const SAMPLES_LEN: usize = 16 * 60;
#[cfg (feature = "windowsnt")]
static mut samples: [load_sample; SAMPLES_LEN]
  = [load_sample { sample_time: 0, idle: 0, kernel: 0, user: 0 }; SAMPLES_LEN];  /* C 1959 */
#[cfg (feature = "windowsnt")]
static mut first_idx: c_int = -1;                                             /* C 1960 */
#[cfg (feature = "windowsnt")]
static mut last_idx: c_int = -1;
#[cfg (feature = "windowsnt")]
static mut max_idx: c_int = SAMPLES_LEN as c_int;                             /* C 1962: `countof (samples)` */

/* C 1964-1973: `buf_next` - C's `from + 1` wrapped at `max_idx`.  */
#[cfg (feature = "windowsnt")]
unsafe fn buf_next (from: c_int) -> c_int
{
  let mut next_idx: c_int = from + 1;                                         /* C 1968 */

  if next_idx >= max_idx                                                      /* C 1970 */
    {
      next_idx = 0;                                                           /* C 1971 */
    }

  return next_idx;                                                            /* C 1973 */
}

/* C 1975-1984: `buf_prev`.  */
#[cfg (feature = "windowsnt")]
unsafe fn buf_prev (from: c_int) -> c_int
{
  let mut prev_idx: c_int = from - 1;                                         /* C 1979 */

  if prev_idx < 0                                                             /* C 1981 */
    {
      prev_idx = max_idx - 1;                                                 /* C 1982 */
    }

  return prev_idx;                                                            /* C 1984 */
}

/* C 1986-2005: `w32_get_nproc`.  C's uninitialized `SYSTEM_INFO sysinfo;` is zeroed
   (PORTING.md 1.5) and its `num_of_processors` is the file-scope `unsigned` above.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_get_nproc () -> c_uint
{
  let mut sysinfo: SYSTEM_INFO = core::mem::zeroed ();                        /* C 1990 */

  /* Initialize the number of processors on this machine.  */
  if (num_of_processors as c_int) <= 0                                        /* C 1993 */
    {
      get_native_system_info (&raw mut sysinfo);                              /* C 1995 */
      num_of_processors = sysinfo.dwNumberOfProcessors;                       /* C 1996 */
      if (num_of_processors as c_int) <= 0                                    /* C 1997 */
        {
          GetSystemInfo (&raw mut sysinfo);                                   /* C 1999 */
          num_of_processors = sysinfo.dwNumberOfProcessors;                   /* C 2000 */
        }
      if (num_of_processors as c_int) <= 0                                    /* C 2002 */
        {
          num_of_processors = 1;                                              /* C 2003 */
        }
    }
  return num_of_processors;                                                   /* C 2005 */
}

/* C 2007-2015: `num_processors` - "Emulate Gnulib's 'num_processors'.  We cannot use the Gnulib
   version because it unconditionally calls APIs that aren't available on old MS-Windows
   versions."  The `enum nproc_query` parameter comes from `lib/include/nproc.rs` 20.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn num_processors (query: nproc_query) -> c_ulong
{
  /* We ignore QUERY.  */
  return w32_get_nproc () as c_ulong;                                         /* C 2014 */
}

/* C 2017-2046: `sample_system_load`.  C's `(void) w32_get_nproc ();` discards the count (the
   port keeps the call as its own statement), the three `FILETIME` locals are zeroed, and the
   three `ULARGE_INTEGER` locals are the union whose `.QuadPart` the caller stores.  */
#[cfg (feature = "windowsnt")]
unsafe fn sample_system_load (idle: *mut ULONGLONG, kernel: *mut ULONGLONG,
                              user: *mut ULONGLONG)
{
  let mut ft_idle: FILETIME = core::mem::zeroed ();                           /* C 2021 */
  let mut ft_user: FILETIME = core::mem::zeroed ();
  let mut ft_kernel: FILETIME = core::mem::zeroed ();

  w32_get_nproc ();                                                           /* C 2023  (C: (void) w32_get_nproc ();) */

  /* TODO: Take into account threads that are ready to run, by
     sampling the "\System\Processor Queue Length" performance
     counter.  The code below accounts only for threads that are
     actually running.  */

  if get_system_times (&raw mut ft_idle, &raw mut ft_kernel, &raw mut ft_user) != 0  /* C 2031 */
    {
      let mut uidle: ULARGE_INTEGER = core::mem::zeroed ();                   /* C 2033 */
      let mut ukernel: ULARGE_INTEGER = core::mem::zeroed ();
      let mut uuser: ULARGE_INTEGER = core::mem::zeroed ();

      memcpy (&raw mut uidle as *mut c_void, &raw mut ft_idle as *mut c_void,
              core::mem::size_of_val (&ft_idle));                             /* C 2035 */
      memcpy (&raw mut ukernel as *mut c_void, &raw mut ft_kernel as *mut c_void,
              core::mem::size_of_val (&ft_kernel));                           /* C 2036 */
      memcpy (&raw mut uuser as *mut c_void, &raw mut ft_user as *mut c_void,
              core::mem::size_of_val (&ft_user));                             /* C 2037 */
      *idle = uidle.QuadPart;                                                 /* C 2038 */
      *kernel = ukernel.QuadPart;                                             /* C 2039 */
      *user = uuser.QuadPart;                                                 /* C 2040 */
    }
  else
    {
      *idle = 0;                                                              /* C 2044 */
      *kernel = 0;                                                            /* C 2045 */
      *user = 0;                                                              /* C 2046 */
    }
}

/* C 2048-2081: `getavg` - "Produce the load average for a given time interval, using the samples
   in the samples[] array.  WHICH can be 0, 1, or 2, meaning 1-minute, 5-minute, or 15-minute
   average, respectively."  C's two `long double` locals stay `c_longdouble` (PORTING.md 1.1), the
   `double` assignment they feed is written out as `as f64`, and the bottom-tested
   `for (idx = …; ; idx = buf_prev (idx))` keeps its increment at the loop's end (skipped by both
   `break`s, as in C).  */
#[cfg (feature = "windowsnt")]
unsafe fn getavg (which: c_int) -> f64
{
  let mut retval: f64 = -1.0;                                                 /* C 2054 */
  let mut tdiff: f64 = 0.0;                                                   /* C 2055 */
  let mut idx: c_int = 0;                                                     /* C 2056 */
  let mut span: f64 = (if which == 0 { 1.0 } else if which == 1 { 5.0 } else { 15.0 }) * 60.0;
  let now: time_t = samples[last_idx as usize].sample_time;                   /* C 2058 */

  if first_idx != last_idx                                                    /* C 2060 */
    {
      idx = buf_prev (last_idx);                                              /* C 2062 */
      loop
	{
	  tdiff = difftime (now, samples[idx as usize].sample_time);          /* C 2064 */
	  if tdiff >= span - 2.0 * DBL_EPSILON * (now as f64)                 /* C 2065 */
	    {
	      let sys: c_longdouble
		= (samples[last_idx as usize].kernel + samples[last_idx as usize].user
		   - (samples[idx as usize].kernel + samples[idx as usize].user)) as c_longdouble;
	      let idl: c_longdouble
		= (samples[last_idx as usize].idle - samples[idx as usize].idle) as c_longdouble;

	      retval = (((1.0 as c_longdouble) - idl / sys)
			* (num_of_processors as c_longdouble)) as f64;        /* C 2071 */
	      break;                                                          /* C 2072 */
	    }
	  if idx == first_idx                                                 /* C 2074 */
	    {
	      break;                                                          /* C 2075 */
	    }
	  idx = buf_prev (idx);                                               /* C 2062 (the for's increment) */
	}
    }

  return retval;                                                              /* C 2080 */
}

/* C 2083-2146: `getloadavg`.  C's `time (NULL)` is the port's null `*mut time_t`, the
   `samples[last_idx]` reads and writes are the array above, and C's `loadavg[elem++] = 0.09;`
   is split into the store and the increment in C's own order.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getloadavg (loadavg: *mut f64, nelem: c_int) -> c_int
{
  let mut elem: c_int;                                                        /* C 2087 */
  let mut idle: ULONGLONG = 0;                                                /* C 2088 */
  let mut kernel: ULONGLONG = 0;
  let mut user: ULONGLONG = 0;
  let mut now: time_t = time (core::ptr::null_mut ());                        /* C 2089 */

  /* If system time jumped back for some reason, delete all samples
     whose time is later than the current wall-clock time.  This
     prevents load average figures from becoming frozen for prolonged
     periods of time, when system time is reset backwards.  */
  if last_idx >= 0                                                            /* C 2096 */
    {
      while difftime (now, samples[last_idx as usize].sample_time) < -1.0     /* C 2098 */
	{
	  if last_idx == first_idx                                            /* C 2100 */
	    {
	      last_idx = -1;                                                  /* C 2102 (C: first_idx = last_idx = -1;) */
	      first_idx = -1;
	      break;                                                          /* C 2103 */
	    }
	  last_idx = buf_prev (last_idx);                                     /* C 2105 */
	}
    }

  /* Store another sample.  We ignore samples that are less than 1 sec
     apart.  */
  if last_idx < 0                                                             /* C 2111 */
      || (difftime (now, samples[last_idx as usize].sample_time)              /* C 2112 */
	  >= 1.0 - 2.0 * DBL_EPSILON * (now as f64))
    {
      sample_system_load (&raw mut idle, &raw mut kernel, &raw mut user);     /* C 2115 */
      last_idx = buf_next (last_idx);                                         /* C 2116 */
      samples[last_idx as usize].sample_time = now;                           /* C 2117 */
      samples[last_idx as usize].idle = idle;                                 /* C 2118 */
      samples[last_idx as usize].kernel = kernel;                             /* C 2119 */
      samples[last_idx as usize].user = user;                                 /* C 2120 */
      /* If the buffer has more that 15 min worth of samples, discard
	 the old ones.  */
      if first_idx == -1                                                      /* C 2124 */
	{
	  first_idx = last_idx;                                               /* C 2125 */
	}
      while first_idx != last_idx                                             /* C 2126 */
	     && (difftime (now, samples[first_idx as usize].sample_time)      /* C 2127-2128 */
	         >= 15.0 * 60.0 + 2.0 * DBL_EPSILON * (now as f64))
	{
	  first_idx = buf_next (first_idx);                                   /* C 2129 */
	}
    }

  elem = 0;                                                                   /* C 2132 */
  while elem < nelem                                                          /* C 2132 */
    {
      let avg: f64 = getavg (elem);                                           /* C 2134 */

      if avg < 0.0                                                            /* C 2136 */
	{
	  break;                                                              /* C 2137 */
	}
      *loadavg.offset (elem as isize) = avg;                                  /* C 2138 */
      elem += 1;                                                              /* C 2132 (the for's increment) */
    }

  /* Always return at least one element, otherwise load-average
     returns nil, and Lisp programs might decide we cannot measure
     system load.  For example, jit-lock-stealth-load's defcustom
     might decide that feature is "unsupported".  */
  if elem == 0                                                                /* C 2142 */
    {
      *loadavg.offset (elem as isize) = 0.09;	/* < display-time-load-average-threshold */
      elem += 1;                                                              /* C 2143 (C: loadavg[elem++] = 0.09;) */
    }

  return elem;                                                                /* C 2145 */
}

/* C 2148-2178: `/* Emulate getpwuid, getpwnam and others.  */`, `#define PASSWD_FIELD_SIZE 256`,
   the five `dflt_passwd_*` buffers, the `dflt_passwd` record whose C initializer takes each
   buffer's *address* (Rust's `&raw mut` of the same static - C's own array decay), `dflt_group_name`
   (`GNLEN+1`) and the `dflt_group` record.  `struct passwd`/`struct group` are
   `nt/inc/include/pwd.rs` 18 and `grp.rs` 20.  */
#[cfg (feature = "windowsnt")]
pub const PASSWD_FIELD_SIZE: usize = 256;                                     /* C 2150 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd_name: [c_char; PASSWD_FIELD_SIZE] = [0; PASSWD_FIELD_SIZE];    /* C 2152 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd_passwd: [c_char; PASSWD_FIELD_SIZE] = [0; PASSWD_FIELD_SIZE];  /* C 2153 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd_gecos: [c_char; PASSWD_FIELD_SIZE] = [0; PASSWD_FIELD_SIZE];   /* C 2154 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd_dir: [c_char; MAX_UTF8_PATH as usize]
  = [0; MAX_UTF8_PATH as usize];                                              /* C 2155 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd_shell: [c_char; MAX_UTF8_PATH as usize]
  = [0; MAX_UTF8_PATH as usize];                                              /* C 2156 */
#[cfg (feature = "windowsnt")]
static mut dflt_passwd: passwd = passwd
{
  pw_name: &raw mut dflt_passwd_name as *mut c_char,                          /* C 2160 */
  pw_passwd: &raw mut dflt_passwd_passwd as *mut c_char,                      /* C 2161 */
  pw_uid: 0,                                                                  /* C 2162 */
  pw_gid: 0,                                                                  /* C 2163 */
  pw_quota: 0,                                                                /* C 2164 */
  pw_gecos: &raw mut dflt_passwd_gecos as *mut c_char,                        /* C 2165 */
  pw_dir: &raw mut dflt_passwd_dir as *mut c_char,                            /* C 2166 */
  pw_shell: &raw mut dflt_passwd_shell as *mut c_char,                        /* C 2167 */
};                                                                            /* C 2158-2168 */
#[cfg (feature = "windowsnt")]
static mut dflt_group_name: [c_char; GNLEN + 1] = [0; GNLEN + 1];             /* C 2170 */
#[cfg (feature = "windowsnt")]
static mut dflt_group: group = group
{
  /* When group information is not available, we return this as the
     group for all files.  */
  gr_name: &raw mut dflt_group_name as *mut c_char,                           /* C 2176 */
  gr_gid: 0,                                                                  /* C 2177 */
};                                                                            /* C 2172-2178 */

/* C 2180-2184: `getuid`.  Its pwd.h prototype lives in `nt/inc/include/pwd.rs`'s `pwd_decls`
   module (`use pwd_decls::*;`), so this definition shadows the glob import.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getuid () -> c_uint
{
  return dflt_passwd.pw_uid;                                                  /* C 2183 */
}

/* C 2186-2193: `geteuid` - "I could imagine arguing for checking to see whether the user is in
   the Administrators group and returning a UID of 0 for that case, but I don't know how wise
   that would be in the long run."  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn geteuid () -> c_uint
{
  return getuid ();                                                           /* C 2192 */
}

/* C 2195-2199: `getgid`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getgid () -> c_uint
{
  return dflt_passwd.pw_gid;                                                  /* C 2198 */
}

/* C 2201-2205: `getegid`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getegid () -> c_uint
{
  return getgid ();                                                           /* C 2204 */
}

/* C 2207-2213: `getpwuid`.  C's `return &dflt_passwd;` is the address of the static above.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpwuid (uid: c_uint) -> *mut passwd
{
  if uid == dflt_passwd.pw_uid                                                 /* C 2210 */
    {
      return &raw mut dflt_passwd;                                            /* C 2211 */
    }
  return core::ptr::null_mut ();                                              /* C 2212 */
}

/* C 2215-2221: `getgrgid`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getgrgid (gid: gid_t) -> *mut group
{
  if gid == dflt_passwd.pw_gid                                                 /* C 2218 */
    {
      return &raw mut dflt_group;                                             /* C 2219 */
    }
  return core::ptr::null_mut ();                                              /* C 2220 */
}

/* C 2223-2258: `getpwnam`.  C's two `getenv` calls, its `logname ? logname : username` conditional
   expression (Rust's `if`/`else` expression), the function-local `static struct passwd alias_user`
   and `static char alias_name[…]` (function-local `static mut`s), the `memcpy` of the whole record
   and the `strncat` that fills the alias name are all C's own statements.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpwnam (name: *mut c_char) -> *mut passwd
{
  let mut pw: *mut passwd;                                                    /* C 2226 */

  pw = getpwuid (getuid ());                                                  /* C 2228 */
  if pw.is_null ()                                                            /* C 2229 */
    {
      return pw;                                                              /* C 2230 */
    }

  if xstrcasecmp (name, (*pw).pw_name) != 0                                   /* C 2232 */
    {
      /* Mimic what init_editfns does with these environment
	 variables, so that the likes of ~USER is recognized by
	 expand-file-name even if $LOGNAME gives a name different from
	 the real username produced by the process token.  */
      let logname: *mut c_char = getenv (c"LOGNAME".as_ptr ());               /* C 2238 */
      let username: *mut c_char = getenv (c"USERNAME".as_ptr ());             /* C 2239 */
      if (!logname.is_null () || !username.is_null ())                        /* C 2240-2241 */
	  && xstrcasecmp (name, if !logname.is_null () { logname } else { username }) == 0
	{
	  static mut alias_user: passwd = passwd                              /* C 2244 */
	  {
	    pw_name: core::ptr::null_mut (), pw_passwd: core::ptr::null_mut (),
	    pw_uid: 0, pw_gid: 0, pw_quota: 0, pw_gecos: core::ptr::null_mut (),
	    pw_dir: core::ptr::null_mut (), pw_shell: core::ptr::null_mut (),
	  };
	  static mut alias_name: [c_char; PASSWD_FIELD_SIZE] = [0; PASSWD_FIELD_SIZE];  /* C 2245 */

	  memcpy (&raw mut alias_user as *mut c_void, &raw mut dflt_passwd as *mut c_void,
		  core::mem::size_of_val (&dflt_passwd));                     /* C 2247 */
	  alias_name[0] = 0;                                                      /* C 2248 */
	  strncat (alias_name.as_mut_ptr (),
		   if !logname.is_null () { logname } else { username },
		   PASSWD_FIELD_SIZE - 1);                                    /* C 2249-2250 */
	  alias_user.pw_name = alias_name.as_mut_ptr ();                          /* C 2251 */
	  pw = &raw mut alias_user;                                               /* C 2252 */
	}
      else
	{
	  return core::ptr::null_mut ();                                      /* C 2255 */
	}
    }

  return pw;                                                                  /* C 2258 */
}

/* C 2361-2364: the two wide literals `init_user_info` passes to `_wgetenv`.  A C `L"…"` has no
   Rust syntax, so the port writes the same UTF-16 code units in arrays (the `w32fns.rs` 385
   `DARK_MODE_APP_NAME` reading; the trailing `0x0000` is C's terminator).
   C: wchar_t *home = _wgetenv (L"HOME");
   C: wchar_t *shell = _wgetenv (L"SHELL");  */
#[cfg (feature = "windowsnt")]
static HOME_WIDE: [WCHAR; 5] = [0x0048, 0x004F, 0x004D, 0x0045, 0x0000];
#[cfg (feature = "windowsnt")]
static SHELL_WIDE: [WCHAR; 6] = [0x0053, 0x0048, 0x0045, 0x004C, 0x004C, 0x0000];

/* C 2260-2391: `init_user_info` - "Find the user's real name by opening the process token and
   looking up the name associated with the user-sid in that token.  Use the relative portion of
   the identifier authority value from the user-sid as the user id value (same for group id using
   the primary group sid from the process token)."  C's uninitialized `DWORD needed;` is zeroed
   (PORTING.md 1.5), its `sizeof` expressions are `core::mem::size_of_val`, its `buf` reallocation
   is `xrealloc` returning `*mut c_void`, and its `blen = needed` assignment is the block
   expression that keeps C's value.  The three `(LPVOID) buf` casts are `as LPVOID`.  */
#[cfg (feature = "windowsnt")]
unsafe fn init_user_info ()
{
  let mut uname: [c_char; UNLEN + 1] = [0; UNLEN + 1];                        /* C 2270 */
  let mut gname: [c_char; GNLEN + 1] = [0; GNLEN + 1];
  let mut domain: [c_char; 1025] = [0; 1025];
  let mut ulength: DWORD = core::mem::size_of_val (&uname) as DWORD;          /* C 2271 */
  let mut dlength: DWORD = core::mem::size_of_val (&domain) as DWORD;
  let mut needed: DWORD = 0;
  let mut glength: DWORD = core::mem::size_of_val (&gname) as DWORD;          /* C 2272 */
  let mut token: HANDLE = core::ptr::null_mut ();                             /* C 2273 */
  let mut user_type: SID_NAME_USE = 0;                                        /* C 2274 */
  let mut buf: *mut c_uchar = core::ptr::null_mut ();                         /* C 2275 */
  let mut blen: DWORD = 0;                                                    /* C 2276 */
  let mut user_token: TOKEN_USER = core::mem::zeroed ();                      /* C 2277 */
  let mut group_token: TOKEN_PRIMARY_GROUP = core::mem::zeroed ();            /* C 2278 */
  let mut result: BOOL;                                                       /* C 2279 */

  result = open_process_token (GetCurrentProcess (), TOKEN_QUERY, &raw mut token);  /* C 2281 */
  if result != 0                                                              /* C 2282 */
    {
      result = get_token_information (token, TokenUser, core::ptr::null_mut (), 0,
                                      &raw mut blen);                         /* C 2284 */
      if result == 0 && GetLastError () == ERROR_INSUFFICIENT_BUFFER          /* C 2285 */
	{
	  buf = xmalloc (blen as usize) as *mut c_uchar;                      /* C 2287 */
	  result = get_token_information (token, TokenUser, buf as LPVOID, blen,
					  &raw mut needed);                   /* C 2288-2289 */
	  if result != 0                                                      /* C 2290 */
	    {
	      memcpy (&raw mut user_token as *mut c_void, buf as *const c_void,
		      core::mem::size_of_val (&user_token));                  /* C 2292 */
	      result = lookup_account_sid (core::ptr::null (), user_token.User.Sid,
					   uname.as_mut_ptr (), &raw mut ulength,
					   domain.as_mut_ptr (), &raw mut dlength,
					   &raw mut user_type);               /* C 2293-2295 */
	    }
	}
      else
	{
	  result = FALSE;                                                     /* C 2299 */
	}
    }
  if result != 0                                                              /* C 2302 */
    {
      strcpy (dflt_passwd.pw_name, uname.as_ptr ());                          /* C 2304 */
      /* Determine a reasonable uid value.  */
      if xstrcasecmp (c"administrator".as_ptr (), uname.as_ptr ()) == 0       /* C 2306 */
	{
	  dflt_passwd.pw_uid = 500; /* well-known Administrator uid */         /* C 2308 */
	  dflt_passwd.pw_gid = 513; /* well-known None gid */                  /* C 2309 */
	}
      else
	{
	  /* Use the last sub-authority value of the RID, the relative
	     portion of the SID, as user/group ID. */
	  dflt_passwd.pw_uid = get_rid (user_token.User.Sid);                 /* C 2314 */

	  /* Get group id and name.  */
	  result = get_token_information (token, TokenPrimaryGroup,
					  buf as LPVOID, blen, &raw mut needed);  /* C 2317-2318 */
	  if result == 0 && GetLastError () == ERROR_INSUFFICIENT_BUFFER      /* C 2319 */
	    {
	      buf = xrealloc (buf as *mut c_void,
			      { blen = needed; blen } as usize) as *mut c_uchar;  /* C 2321 */
	      result = get_token_information (token, TokenPrimaryGroup,
					      buf as LPVOID, blen, &raw mut needed);  /* C 2322-2323 */
	    }
	  if result != 0                                                      /* C 2325 */
	    {
	      memcpy (&raw mut group_token as *mut c_void, buf as *const c_void,
		      core::mem::size_of_val (&group_token));                 /* C 2327 */
	      dflt_passwd.pw_gid = get_rid (group_token.PrimaryGroup);        /* C 2328 */
	      dlength = core::mem::size_of_val (&domain) as DWORD;            /* C 2329 */
	      /* If we can get at the real Primary Group name, use that.
		 Otherwise, the default group name was already set to
		 "None" in globals_of_w32.  */
	      if lookup_account_sid (core::ptr::null (), group_token.PrimaryGroup,
				     gname.as_mut_ptr (), &raw mut glength,
				     core::ptr::null_mut (), &raw mut dlength,
				     &raw mut user_type) != 0                 /* C 2334-2336 */
		{
		  strcpy (dflt_group_name.as_mut_ptr (), gname.as_ptr ());    /* C 2337 */
		}
	    }
	  else
	    {
	      dflt_passwd.pw_gid = dflt_passwd.pw_uid;                        /* C 2341 */
	    }
	}
    }
  /* If security calls are not supported (presumably because we
     are running under Windows 9X), fallback to this: */
  else if GetUserName (uname.as_mut_ptr (), &raw mut ulength) != 0            /* C 2346 */
    {
      strcpy (dflt_passwd.pw_name, uname.as_ptr ());                          /* C 2348 */
      if xstrcasecmp (c"administrator".as_ptr (), uname.as_ptr ()) == 0       /* C 2349 */
	{
	  dflt_passwd.pw_uid = 0;                                             /* C 2350 */
	}
      else
	{
	  dflt_passwd.pw_uid = 123;                                           /* C 2352 */
	}
      dflt_passwd.pw_gid = dflt_passwd.pw_uid;                                /* C 2353 */
    }
  else
    {
      strcpy (dflt_passwd.pw_name, c"unknown".as_ptr ());                     /* C 2357 */
      dflt_passwd.pw_uid = 123;                                               /* C 2358 */
      dflt_passwd.pw_gid = 123;                                               /* C 2359 */
    }
  dflt_group.gr_gid = dflt_passwd.pw_gid;                                     /* C 2361 */

  /* Set dir and shell from environment variables. */
  if w32_unicode_filenames                                                    /* C 2364 */
    {
      let home: *mut c_ushort = _wgetenv (HOME_WIDE.as_ptr ());               /* C 2366 */
      let shell: *mut c_ushort = _wgetenv (SHELL_WIDE.as_ptr ());             /* C 2367 */

      /* Ensure HOME and SHELL are defined. */
      if home.is_null ()                                                      /* C 2370 */
	{
	  emacs_abort ();                                                     /* C 2371 */
	}
      if shell.is_null ()                                                     /* C 2372 */
	{
	  emacs_abort ();                                                     /* C 2373 */
	}
      filename_from_utf16 (home, dflt_passwd.pw_dir);                         /* C 2374 */
      filename_from_utf16 (shell, dflt_passwd.pw_shell);                      /* C 2375 */
    }
  else
    {
      let home: *mut c_char = getenv (c"HOME".as_ptr ());                     /* C 2379 */
      let shell: *mut c_char = getenv (c"SHELL".as_ptr ());                   /* C 2380 */

      if home.is_null ()                                                      /* C 2382 */
	{
	  emacs_abort ();                                                     /* C 2383 */
	}
      if shell.is_null ()                                                     /* C 2384 */
	{
	  emacs_abort ();                                                     /* C 2385 */
	}
      filename_from_ansi (home, dflt_passwd.pw_dir);                          /* C 2386 */
      filename_from_ansi (shell, dflt_passwd.pw_shell);                       /* C 2387 */
    }

  xfree (buf as *mut c_void);                                                 /* C 2390 */
  if !token.is_null ()                                                        /* C 2391 */
    {
      CloseHandle (token);                                                    /* C 2392 */
    }
}

/* C 2393-2406: `static HCRYPTPROV w32_crypto_hprov;` and `w32_init_crypt_random`.  C's
   `DebPrint ((…))` is `include/conf_post.rs` 182's `macro_rules! DebPrint` (the arm this
   configuration takes is the no-op), and the unit's include graph reaches conf_post through
   C 19's `<config.h>` (its AH_BOTTOM `#include <conf_post.h>`; the ledger's Deviations record
   that edge).  */
#[cfg (feature = "windowsnt")]
static mut w32_crypto_hprov: HCRYPTPROV = 0;                                  /* C 2393 */
#[cfg (feature = "windowsnt")]
unsafe fn w32_init_crypt_random () -> c_int
{
  if CryptAcquireContext (&raw mut w32_crypto_hprov, core::ptr::null (),
			  core::ptr::null (), PROV_RSA_FULL,
			  CRYPT_VERIFYCONTEXT | CRYPT_SILENT) == 0            /* C 2397-2398 */
    {
      DebPrint! (("CryptAcquireContext failed with error %x\n",
		  GetLastError ()));                                          /* C 2400-2401 */
      w32_crypto_hprov = 0;                                                   /* C 2402 */
      return -1;                                                              /* C 2403 */
    }
  return 0;                                                                   /* C 2405 */
}

/* C 2408-2419: `w32_init_random` - C's `(BYTE *) buf` cast and its two nested `if`s.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_init_random (buf: *mut c_void, buflen: isize) -> c_int
{
  if w32_crypto_hprov == 0                                                    /* C 2411 */
    {
      w32_init_crypt_random ();                                               /* C 2412 */
    }
  if w32_crypto_hprov != 0                                                    /* C 2413 */
    {
      if CryptGenRandom (w32_crypto_hprov, buflen as DWORD, buf as *mut BYTE) != 0  /* C 2415 */
	{
	  return 0;                                                           /* C 2416 */
	}
    }
  return -1;                                                                  /* C 2419 */
}

/* C 2421-2429: `/* MS-Windows 'rand' produces separate identical series for each thread, so we
   replace it with our version.  */`, the AS183 attribution, `static int ix = 3172, iy = 9814,
   iz = 20125;` and the three `RAND_MAX_*` macros (C's constants, so the port's are consts).  */
/* C 2431-2439: `rand_as183` - "Algorithm AS183: An Efficient and Portable Pseudo-random Number
   Generator, by B.A. Wichmann, I.D. Hill.  AS, v31, No. 2 (1982)."  */
#[cfg (feature = "windowsnt")]
static mut ix: c_int = 3172;                                                  /* C 2426 */
#[cfg (feature = "windowsnt")]
static mut iy: c_int = 9814;
#[cfg (feature = "windowsnt")]
static mut iz: c_int = 20125;
#[cfg (feature = "windowsnt")]
pub const RAND_MAX_X: c_int = 30269;                                          /* C 2427 */
#[cfg (feature = "windowsnt")]
pub const RAND_MAX_Y: c_int = 30307;                                          /* C 2428 */
#[cfg (feature = "windowsnt")]
pub const RAND_MAX_Z: c_int = 30323;                                          /* C 2429 */
#[cfg (feature = "windowsnt")]
unsafe fn rand_as183 () -> c_int
{
  ix = (171 * ix) % RAND_MAX_X;                                               /* C 2434 */
  iy = (172 * iy) % RAND_MAX_Y;                                               /* C 2435 */
  iz = (170 * iz) % RAND_MAX_Z;                                               /* C 2436 */

  return (ix + iy + iz) & 0x7fff;                                             /* C 2438 */
}

/* C 2441-2451: `random`.  C 2446's `#if EMACS_INT_MAX > INT_MAX` is the one condition of this
   slice the evaluator *disagrees* with (clang True, the evaluator False - `EMACS_INT_MAX` is a
   `config.h` macro), and `P-02` fixes the wide-integer target, so the arm clang compiles is the
   one carried (the `fns.rs` C 5383 reading) and C 2448-2449's `#else` text is carried here.
   C: #if EMACS_INT_MAX > INT_MAX
   C:   return ((rand_as183 () << 30) | (rand_as183 () << 15) | rand_as183 ());
   C: #else
   C:   return ((rand_as183 () << 15) | rand_as183 ());
   C: #endif  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn random () -> c_int
{
  /* rand_as183 () gives us 15 random bits...hack together 30 bits for
     Emacs with 32-bit EMACS_INT, and at least 31 bit for wider EMACS_INT.  */
  return ((rand_as183 () << 30) | (rand_as183 () << 15) | rand_as183 ());     /* C 2447 */
}

/* C 2453-2460: `srandom` - the CRT's `srand`/`rand` (declared in the section above) and the
   three `RAND_MAX_*` divisors.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn srandom (seed: c_int)
{
  srand (seed as c_uint);                                                     /* C 2457 */
  ix = rand () % RAND_MAX_X;                                                  /* C 2458 */
  iy = rand () % RAND_MAX_Y;                                                  /* C 2459 */
  iz = rand () % RAND_MAX_Z;                                                  /* C 2460 */
}

/* C 2462-2480: `explicit_bzero` - "Emulate explicit_bzero.  This is to avoid using the Gnulib
   version, because it calls SecureZeroMemory at will, disregarding systems older than Windows
   XP, which didn't have that function."  Both arms of C 2471's `#if _WIN32_WINNT >= 0x0501` are
   carried (`win32_winnt_ge_0501`, named after C's own predicate - the `w32fns.rs` C 167
   reading), and C 2476's `asm volatile ("" ::: "memory")` is the port's compiler fence (C's text
   is above it).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn explicit_bzero (buf: *mut c_void, len: usize)
{
  #[cfg (feature = "win32_winnt_ge_0501")]
  {
    /* We are compiling for XP or newer, most probably with MinGW64.
       We can use SecureZeroMemory.  */
    SecureZeroMemory! (buf, len);                                             /* C 2474 */
  }
  #[cfg (not (feature = "win32_winnt_ge_0501"))]
  {
    memset (buf, 0, len);                                                     /* C 2477 */
    /* Compiler barrier.  */
    /* C 2479: `asm volatile ("" ::: "memory");` */
    core::sync::atomic::compiler_fence (core::sync::atomic::Ordering::SeqCst);  /* C 2479 */
  }
}

/* C 2482-2492: `max_filename_mbslen` - "Return the maximum length in bytes of a multibyte
   character sequence encoded in the current ANSI codepage."  C's uninitialized `CPINFO cp_info;`
   is zeroed and `MaxCharSize` (a `UINT`) narrows to the `int` return.  */
#[cfg (feature = "windowsnt")]
unsafe fn max_filename_mbslen () -> c_int
{
  let mut cp_info: CPINFO = core::mem::zeroed ();                             /* C 2488 */

  codepage_for_filenames (&raw mut cp_info);                                  /* C 2490 */
  return cp_info.MaxCharSize as c_int;                                        /* C 2491 */
}

/* C 2494-2521: `normalize_filename` - "Normalize filename by converting in-place all of its path
   separators to the separator specified by PATH_SEP."  C's `register` storage class has no Rust
   counterpart (PORTING.md's token table skips it), `*fp += 'a' - 'A'` is written as C's own
   `char` arithmetic (`c_int` then back), and the two separator tests are C's.  */
#[cfg (feature = "windowsnt")]
unsafe fn normalize_filename (mut fp: *mut c_char, path_sep: c_char)
{
  let mut p2: *mut c_char;                                                    /* C 2501 */

  /* Always lower-case drive letters a-z, even if the filesystem
     preserves case in filenames.
     This is so filenames can be compared by string comparison
     functions that are case-sensitive.  Even case-preserving filesystems
     do not distinguish case in drive letters.  */
  p2 = fp.offset (1);                                                         /* C 2508 */

  if *p2 == b':' as c_char && *fp >= b'A' as c_char && *fp <= b'Z' as c_char  /* C 2510 */
    {
      *fp = ((*fp as c_int) + (b'a' as c_int - b'A' as c_int)) as c_char;     /* C 2512 */
      fp = fp.offset (2);                                                     /* C 2513 */
    }

  while *fp != 0                                                              /* C 2516 */
    {
      if (*fp == b'/' as c_char || *fp == b'\\' as c_char) && *fp != path_sep  /* C 2518 */
	{
	  *fp = path_sep;                                                     /* C 2519 */
	}
      fp = fp.offset (1);                                                     /* C 2520 */
    }
}

/* C 2523-2528: `/* Destructively turn backslashes into slashes.  */` and `dostounix_filename`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dostounix_filename (p: *mut c_char)
{
  normalize_filename (p, b'/' as c_char);                                     /* C 2527 */
}

/* C 2530-2535: `/* Destructively turn slashes into backslashes.  */` and `unixtodos_filename`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtodos_filename (p: *mut c_char)
{
  normalize_filename (p, b'\\' as c_char);                                    /* C 2534 */
}

/* C 2537-2562: `crlf_to_lf` - "Remove all CR's that are followed by a LF.  (From msdos.c…)".  The
   two `*np++`/`*buf++` post-increments are split into the store and the step, in C's order.  */
#[cfg (feature = "windowsnt")]
unsafe fn crlf_to_lf (n: c_int, mut buf: *mut c_char) -> c_int
{
  let mut np: *mut c_uchar = buf as *mut c_uchar;                             /* C 2542 */
  let startp: *mut c_uchar = np;                                              /* C 2543 */
  let endp: *mut c_char = buf.offset (n as isize);                            /* C 2544 */

  if n == 0                                                                   /* C 2546 */
    {
      return n;                                                               /* C 2547 */
    }
  while (buf as usize) < (endp as usize) - 1                                  /* C 2548 */
    {
      if *buf as c_uchar == 0x0d                                              /* C 2550 */
	{
	  buf = buf.offset (1);
	  if *buf as c_uchar != 0x0a                                              /* C 2551 */
	    {
	      *np = 0x0d;                                                         /* C 2552 */
	      np = np.offset (1);
	    }
	}
      else
	{
	  *np = *buf as c_uchar;                                                  /* C 2555 */
	  np = np.offset (1);
	  buf = buf.offset (1);
	}
    }
  if (buf as usize) < (endp as usize)                                         /* C 2557 */
    {
      *np = *buf as c_uchar;                                                  /* C 2558 */
      np = np.offset (1);
      buf = buf.offset (1);
    }
  return np.offset_from (startp) as c_int;                                    /* C 2559 */
}

/* C 2564-2602: `parse_root` - "Parse the root part of file name, if present.  Return length and
   optionally store pointer to char after root."  C's three shape tests (`c_isalpha`,
   `IS_DIRECTORY_SEP`, the drive `:`), its `do … while (*name)` loop with the `--slashes` test and
   its `name - start` return are kept; `pPath` is C's `const char **`.  */
#[cfg (feature = "windowsnt")]
unsafe fn parse_root (name: *const c_char, pPath: *mut *const c_char) -> c_int
{
  let mut name: *const c_char = name;
  let start: *const c_char = name;                                            /* C 2569 */

  if name.is_null ()                                                          /* C 2571 */
    {
      return 0;                                                               /* C 2572 */
    }

  /* find the root name of the volume if given */
  if c_isalpha (*name as c_int) && *name.offset (1) == b':' as c_char         /* C 2575 */
    {
      /* skip past drive specifier */
      name = name.offset (2);                                                 /* C 2578 */
      if IS_DIRECTORY_SEP! (*name as c_int)                                   /* C 2579 */
	{
	  name = name.offset (1);
	}
    }
  else if IS_DIRECTORY_SEP! (*name as c_int)                                  /* C 2582 */
	   && IS_DIRECTORY_SEP! (*name.offset (1) as c_int)
    {
      let mut slashes: c_int = 2;                                             /* C 2584 */

      name = name.offset (2);                                                 /* C 2586 */
      loop
	{
	  if *name == 0 || (IS_DIRECTORY_SEP! (*name as c_int)
			    && { slashes -= 1; slashes == 0 })                /* C 2589 */
	    {
	      break;
	    }
	  name = name.offset (1);
	  if *name == 0                                                       /* C 2593 */
	    {
	      break;
	    }
	}
      if IS_DIRECTORY_SEP! (*name as c_int)                                   /* C 2594 */
	{
	  name = name.offset (1);
	}
    }

  if !pPath.is_null ()                                                        /* C 2597 */
    {
      *pPath = name;                                                          /* C 2598 */
    }

  return name.offset_from (start) as c_int;                                   /* C 2600 */
}

/* C 2604-2666: `get_long_basename` - "Get long base name for name; name is assumed to be
   absolute."  The two `WIN32_FIND_DATA*` records live in the section above, C's
   `INVALID_HANDLE_VALUE` is the `<winnt.h>` constant, the `_mbspbrk`/`strrchr` arm is the ANSI
   one, and C's `(len = strlen (fname_utf8)) < size` assignment-in-condition is the same
   assignment inside the `if`.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_long_basename (name: *mut c_char, buf: *mut c_char, size: c_int) -> c_int
{
  let mut dir_handle: HANDLE = INVALID_HANDLE_VALUE;                          /* C 2608 */
  let mut fname_utf8: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 2609 */
  let mut len: c_int = 0;                                                     /* C 2610 */
  let mut cstatus: c_int = -1;                                                /* C 2611 */

  /* Must be valid filename, no wild cards or other invalid characters.  */
  if !strpbrk (name, c"*?|<>\""  .as_ptr ()).is_null ()                       /* C 2614 */
    {
      return 0;                                                               /* C 2615 */
    }

  if w32_unicode_filenames                                                    /* C 2617 */
    {
      let mut fname_utf16: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2619 */
      let mut find_data_wide: WIN32_FIND_DATAW = core::mem::zeroed ();        /* C 2620 */

      filename_to_utf16 (name, fname_utf16.as_mut_ptr ());                    /* C 2622 */
      dir_handle = FindFirstFileW (fname_utf16.as_mut_ptr (), &raw mut find_data_wide);  /* C 2623 */
      if dir_handle != INVALID_HANDLE_VALUE                                   /* C 2624 */
	{
	  cstatus = filename_from_utf16 (find_data_wide.cFileName.as_ptr (),
					 fname_utf8.as_mut_ptr ());           /* C 2625 */
	}
    }
  else
    {
      let mut fname_ansi: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2629 */
      let mut find_data_ansi: WIN32_FIND_DATAA = core::mem::zeroed ();        /* C 2630 */

      filename_to_ansi (name, fname_ansi.as_mut_ptr ());                      /* C 2632 */
      /* If the ANSI name includes ? characters, it is not encodable
	 in the ANSI codepage.  In that case, we deliver the question
	 marks to the caller; calling FindFirstFileA in this case
	 could return some unrelated file name in the same
	 directory.  */
      if !_mbspbrk (fname_ansi.as_ptr () as *const c_uchar,
		    c"?".as_ptr () as *const c_uchar).is_null ()              /* C 2638 */
	{
	  /* Find the basename of fname_ansi.  */
	  let mut p: *mut c_char = strrchr (fname_ansi.as_ptr (),
					    b'\\' as c_int);                  /* C 2641 */

	  if p.is_null ()                                                     /* C 2643 */
	    {
	      p = fname_ansi.as_mut_ptr ();                                   /* C 2644 */
	    }
	  else
	    {
	      p = p.offset (1);                                               /* C 2646 */
	    }
	  cstatus = filename_from_ansi (p, fname_utf8.as_mut_ptr ());         /* C 2647 */
	}
      else
	{
	  dir_handle = FindFirstFileA (fname_ansi.as_mut_ptr (),
				       &raw mut find_data_ansi);              /* C 2651 */
	  if dir_handle != INVALID_HANDLE_VALUE                               /* C 2652 */
	    {
	      cstatus = filename_from_ansi (find_data_ansi.cFileName.as_ptr (),
					    fname_utf8.as_mut_ptr ());        /* C 2653 */
	    }
	}
    }

  if cstatus == 0                                                             /* C 2657 */
    {
      len = strlen (fname_utf8.as_ptr ()) as c_int;   /* C 2657 (C: the assignment inside the condition) */
      if len < size
	{
	  memcpy (buf as *mut c_void, fname_utf8.as_ptr () as *const c_void,
		  len as usize + 1);                                          /* C 2658 */
	}
      else
	{
	  len = 0;                                                            /* C 2660 */
	}
    }
  else
    {
      len = 0;                                                                /* C 2660 */
    }

  if dir_handle != INVALID_HANDLE_VALUE                                       /* C 2662 */
    {
      FindClose (dir_handle);                                                 /* C 2663 */
    }

  return len;                                                                 /* C 2665 */
}

/* C 2668-2718: `w32_get_long_filename` - "Get long name for file, if possible (assumed to be
   absolute)."  C's `(const char **) &p` cast is the port's `&raw mut p as *mut *const c_char`,
   the `*p++`/`*o++` post-increments are split in C's order, and `parse_root`/`get_long_basename`
   are this file's own definitions above.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_get_long_filename (name: *const c_char, buf: *mut c_char,
                                                size: c_int) -> BOOL
{
  let mut o: *mut c_char = buf;                                               /* C 2672 */
  let mut p: *mut c_char = core::ptr::null_mut ();                            /* C 2673 */
  let mut q: *const c_char;                                                   /* C 2674 */
  let mut full: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 2675 */
  let mut len: c_int;                                                         /* C 2676 */

  len = strlen (name) as c_int;                                               /* C 2678 */
  if len >= MAX_UTF8_PATH                                                     /* C 2679 */
    {
      return FALSE;                                                           /* C 2680 */
    }

  /* Use local copy for destructive modification.  */
  memcpy (full.as_mut_ptr () as *mut c_void, name as *const c_void,
	  len as usize + 1);                                                  /* C 2683 */
  unixtodos_filename (full.as_mut_ptr ());                                    /* C 2684 */

  /* Copy root part verbatim.  */
  len = parse_root (full.as_ptr (),
		    &raw mut p as *mut *const c_char);                        /* C 2687 */
  memcpy (o as *mut c_void, full.as_ptr () as *const c_void, len as usize);   /* C 2688 */
  o = o.offset (len as isize);                                                /* C 2689 */
  *o = 0;                                                                     /* C 2690 */
  size -= len;                                                                /* C 2691 */

  while !p.is_null () && *p != 0                                              /* C 2693 */
    {
      q = p;                                                                  /* C 2695 */
      p = strchr (q, b'\\' as c_int);                                         /* C 2696 */
      if !p.is_null ()                                                        /* C 2697 */
	{
	  *p = 0;
	}
      len = get_long_basename (full.as_mut_ptr (), o, size);                  /* C 2698 */
      if len > 0                                                              /* C 2699 */
	{
	  o = o.offset (len as isize);                                        /* C 2701 */
	  size -= len;                                                        /* C 2702 */
	  if !p.is_null ()                                                    /* C 2703 */
	    {
	      *p = b'\\' as c_char;
	      p = p.offset (1);                                               /* C 2705 */
	      if size < 2                                                     /* C 2706 */
		{
		  return FALSE;                                               /* C 2707 */
		}
	      *o = b'\\' as c_char;                                           /* C 2708 */
	      o = o.offset (1);
	      size -= 1;                                                      /* C 2709 */
	      *o = 0;                                                         /* C 2710 */
	    }
	}
      else
	{
	  return FALSE;                                                       /* C 2714 */
	}
    }

  return TRUE;                                                                /* C 2717 */
}

/* C 2720-2741: `w32_get_short_filename` - the `…W`/`…A` pair with C's `retval && retval < size`
   test on the wide side.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_get_short_filename (name: *const c_char, buf: *mut c_char,
                                                 size: c_int) -> c_uint
{
  if w32_unicode_filenames                                                    /* C 2723 */
    {
      let mut name_utf16: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2725 */
      let mut short_name: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];
      let mut retval: c_uint;                                                 /* C 2726 */

      filename_to_utf16 (name, name_utf16.as_mut_ptr ());                     /* C 2728 */
      retval = GetShortPathNameW (name_utf16.as_mut_ptr (), short_name.as_mut_ptr (),
				  size as DWORD);                             /* C 2729 */
      if retval != 0 && retval < size as c_uint                               /* C 2730 */
	{
	  filename_from_utf16 (short_name.as_mut_ptr (), buf);                /* C 2731 */
	}
      return retval;                                                          /* C 2732 */
    }
  else
    {
      let mut name_ansi: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2736 */

      filename_to_ansi (name, name_ansi.as_mut_ptr ());                       /* C 2738 */
      return GetShortPathNameA (name_ansi.as_mut_ptr (), buf, size as DWORD);  /* C 2739 */
    }
}

/* C 2743-2773: `ansi_encode_filename` - "Re-encode FILENAME, a UTF-8 encoded unibyte string,
   using the MS-Windows ANSI codepage.  If FILENAME includes characters not supported by the ANSI
   codepage, return the 8+3 alias of FILENAME, if it exists."  C's two `SSDATA` calls and its
   `build_string`/`build_unibyte_string` pair are `lisp.rs`'s.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ansi_encode_filename (filename: Lisp_Object) -> Lisp_Object
{
  let mut encoded_filename: Lisp_Object;                                      /* C 2755 */
  let mut fname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];        /* C 2756 */

  filename_to_ansi (SSDATA (filename), fname.as_mut_ptr ());                  /* C 2758 */
  if !_mbspbrk (fname.as_ptr () as *const c_uchar,
		c"?".as_ptr () as *const c_uchar).is_null ()                  /* C 2759 */
    {
      let mut shortname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2761 */

      if w32_get_short_filename (SSDATA (filename), shortname.as_mut_ptr (),
				 MAX_PATH) != 0                               /* C 2763 */
	{
	  dostounix_filename (shortname.as_mut_ptr ());                         /* C 2765 */
	  encoded_filename = build_string (shortname.as_ptr ());                /* C 2766 */
	}
      else
	{
	  encoded_filename = build_unibyte_string (fname.as_ptr ());            /* C 2769 */
	}
    }
  else
    {
      encoded_filename = build_unibyte_string (fname.as_ptr ());              /* C 2772 */
    }
  return encoded_filename;                                                    /* C 2773 */
}

/* C 2775-2787: `is_unc_volume` - C's `IS_DIRECTORY_SEP` pair, the `ptr[2]` test and the
   `strpbrk` character set.  */
#[cfg (feature = "windowsnt")]
unsafe fn is_unc_volume (filename: *const c_char) -> c_int
{
  let mut ptr: *const c_char = filename;                                      /* C 2779 */

  if !IS_DIRECTORY_SEP! (*ptr as c_int)                                       /* C 2781 */
      || !IS_DIRECTORY_SEP! (*ptr.offset (1) as c_int) || *ptr.offset (2) == 0
    {
      return 0;                                                               /* C 2782 */
    }

  if !strpbrk (ptr.offset (2), c"*?|<>\"\\/".as_ptr ()).is_null ()            /* C 2784 */
    {
      return 0;                                                               /* C 2785 */
    }

  return 1;                                                                   /* C 2787 */
}

/* C 2789-2815: `unsetenv` - "Emulate the Posix unsetenv."  C's three rejection tests, the 32K
   bound (the MS documentation's, with C's own comment), the `alloca` stack-pool allocation
   (PORTING.md P-05) and the `var[name_len++] = '='` store are C's statements.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unsetenv (name: *const c_char) -> c_int
{
  let mut var: *mut c_char;                                                   /* C 2793 */
  let mut name_len: usize;                                                    /* C 2794 */

  if name.is_null () || *name == 0 || !strchr (name, b'=' as c_int).is_null ()  /* C 2796 */
    {
      *errno () = EINVAL;                                                     /* C 2798 */
      return -1;                                                              /* C 2799 */
    }
  name_len = strlen (name);                                                   /* C 2801 */
  /* MS docs says an environment variable cannot be longer than 32K.  */
  if name_len > 32767                                                         /* C 2803 */
    {
      *errno () = ENOMEM;                                                     /* C 2805 */
      return 0;                                                               /* C 2806 */
    }
  /* It is safe to use 'alloca' with 32K size, since the stack is at
     least 2MB, and we set it to 8MB in the link command line.  */
  var = sa_stack_alloc (name_len + 2) as *mut c_char;                          /* C 2810 */
  memcpy (var as *mut c_void, name as *const c_void, name_len);               /* C 2811 */
  *var.offset (name_len as isize) = b'=' as c_char;                           /* C 2812 */
  name_len += 1;
  *var.offset (name_len as isize) = 0;                                        /* C 2813 */
  return _putenv (var);                                                       /* C 2814 */
}

/* C 2817-2860: `sys_putenv` - "MS _putenv doesn't support removing a variable when the argument
   does not include the '=' character, so we fix that here."  The `TZ=<` conversion loop keeps
   C's two `memmove`/`memset` arms and its `abbrlen` arithmetic; C's `for (char *p = str + 4; *p;
   p++)` with the loop's `break` is the port's `while` with the step at the end.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_putenv (str: *mut c_char) -> c_int
{
  let name_end: *const c_char = strchr (str, b'=' as c_int);                  /* C 2821 */

  if name_end.is_null ()                                                      /* C 2823 */
    {
      /* Remove the variable from the environment.  */
      return unsetenv (str);                                                  /* C 2825 */
    }

  if strncmp (str, c"TZ=<".as_ptr (), 4) == 0                                 /* C 2828 */
    {
      /* MS-Windows does not support POSIX.1-2001 angle-bracket TZ
	 abbreviation syntax.  Convert to POSIX.1-1988 syntax if possible,
	 and to the undocumented placeholder "ZZZ" otherwise.  */
      let mut supported_abbr: bool = true;                                    /* C 2834 */
      let mut p: *mut c_char = str.offset (4);                                /* C 2835 */
      while *p != 0
	{
	  if (b'0' as c_char <= *p && *p <= b'9' as c_char) || *p == b'-' as c_char
	      || *p == b'+' as c_char                                          /* C 2837 */
	    {
	      supported_abbr = false;                                          /* C 2838 */
	    }
	  else if *p == b'>' as c_char                                         /* C 2839 */
	    {
	      let mut abbrlen: isize;                                          /* C 2841 */
	      if supported_abbr                                                /* C 2842 */
		{
		  abbrlen = p.offset_from (str.offset (4));                     /* C 2844 */
		  memmove (str.offset (3) as *mut c_void,
			   str.offset (4) as *const c_void, abbrlen as usize);  /* C 2845 */
		}
	      else
		{
		  abbrlen = 3;                                                  /* C 2849 */
		  memset (str.offset (3) as *mut c_void, b'Z' as c_int,
			  abbrlen as usize);                                    /* C 2850 */
		}
	      memmove (str.offset (3 + abbrlen) as *mut c_void,
		       p.offset (1) as *const c_void, strlen (p));              /* C 2852 */
	      break;                                                            /* C 2853 */
	    }
	  p = p.offset (1);                                                     /* C 2835 (the for's increment) */
	}
    }

  return _putenv (str);                                                       /* C 2860 */
}

/* C 2862: `#define REG_ROOT "SOFTWARE\\GNU\\Emacs"` - C's string macro is the port's same-named
   `&CStr` constant (PORTING.md 2.1), which `init_environment` below passes to `w32_get_resource`.  */
#[cfg (feature = "windowsnt")]
pub const REG_ROOT: &core::ffi::CStr = c"SOFTWARE\\GNU\\Emacs";

/* C 2861-3213: `init_environment` - the slice's longest function, called from `main`
   (`emacs.c`).  The argv comment is C's.  C's `static const char * const tempdirs[]` is a
   `static` array of `&CStr` (the port's string-constant shape), `countof` is
   `lib/include/stdcountof.rs`, C's `for (i = 0; …; i++)` loops are the port's `while` with the
   step at the end, the six `alloca`-style buffers are `sa_stack_alloc` (P-05), the inner
   `env_entry` records keep C's two members, and C's two `#define`s are consts.  `sys_access`
   (C 2900/2962/3100/3110) and `w32_get_resource` are declared elsewhere - the first by this
   file's own definition past this slice (C 4348, the ledger's P-91 (4) row), the second by
   `include/w32.rs` 306.  */
/* The argv[] array holds ANSI-encoded strings, and so this function
   works with ANS_encoded strings.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_environment (argv: *mut *mut c_char)
{
  static TEMPDIRS: [&core::ffi::CStr; 4]
    = [c"$TMPDIR", c"$TEMP", c"$TMP", c"c:/"];                                /* C 2867-2869 */

  let mut i: c_int;                                                           /* C 2871 */

  let imax: c_int = countof! (TEMPDIRS) as c_int;                             /* C 2873 */

  /* Implementation note: This function explicitly works with ANSI
     file names, not with UTF-8 encoded file names.  This is because
     this function pushes variables into the Emacs's environment, and
     the environment variables are always assumed to be in the
     locale-specific encoding.  Do NOT call any functions that accept
     UTF-8 file names from this function!  */

  /* Make sure they have a usable $TMPDIR.  Many Emacs functions use
     temporary files and assume "/tmp" if $TMPDIR is unset, which
     will break on DOS/Windows.  Refuse to work if we cannot find
     a directory, not even "c:/", usable for that purpose.  */
  i = 0;                                                                      /* C 2885 */
  while i < imax                                                              /* C 2885 */
    {
      let mut tmp: *const c_char = TEMPDIRS[i as usize].as_ptr ();            /* C 2887 */

      if *tmp == b'$' as c_char                                               /* C 2889 */
	{
	  tmp = getenv (tmp.offset (1));                                      /* C 2890 */
	}
      /* Note that `access' can lie to us if the directory resides on a
	 read-only filesystem, like CD-ROM or a write-protected floppy.
	 The only way to be really sure is to actually create a file and
	 see if it succeeds.  But I think that's too much to ask.  */

      /* MSVCRT's _access crashes with D_OK, so we use our replacement.  */
      if !tmp.is_null () && sys_access (tmp, D_OK) == 0                       /* C 2900 */
	{
	  let var: *mut c_char = sa_stack_alloc (strlen (tmp) + 8) as *mut c_char;  /* C 2902 */
	  sprintf (var, c"TMPDIR=%s".as_ptr (), tmp);                         /* C 2903 */
	  _putenv (strdup (var));                                             /* C 2904 */
	  break;                                                              /* C 2905 */
	}
      i += 1;                                                                 /* C 2885 (the for's increment) */
    }
  if i >= imax                                                                /* C 2907 */
    {
      cmd_error_internal                                                      /* C 2908-2912 */
	(Fcons (Qerror (),
		Fcons (build_string (c"no usable temporary directories found!!".as_ptr ()),
		       Qnil ())),
	 c"While setting TMPDIR: ".as_ptr ());
    }

  /* Check for environment variables and use registry settings if they
     don't exist.  Fallback on default values where applicable.  */
  {                                                                           /* C 2915 */
    let mut i: c_int;                                                         /* C 2917 */
    let mut lpval: LPBYTE;                                                    /* C 2918 */
    let mut dwType: DWORD;                                                    /* C 2919 */
    let mut locale_name: [c_char; 32] = [0; 32];                              /* C 2920 */
    let mut default_home: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 2921 */
    let mut appdata: c_int = 0;                                               /* C 2922 */

    /* C 2924-2929: C's own inner declaration `static const struct env_entry { … }
       dflt_envvars[] = { … }`, whose two members are kept; the port spells the table as a `const`
       item (a shared `static` of raw pointers is not `Sync`, and C never takes its address - the
       port copies the records into `env_vars`).
       C: static const struct env_entry
       C: {
       C:   const char * name;
       C:   const char * def_value;
       C: } dflt_envvars[] =  */
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct env_entry
    {
      name: *const c_char,
      def_value: *const c_char,
    }
    const DFLT_ENVVARS: [env_entry; 11] =                                    /* C 2924-2944 */
    [
      /* If the default value is NULL, we will use the value from the
	 outside environment or the Registry, but will not push the
	 variable into the Emacs environment if it is defined neither
	 in the Registry nor in the outside environment.  */
      env_entry { name: c"HOME".as_ptr (), def_value: c"C:/".as_ptr () },                /* C 2936 */
      env_entry { name: c"PRELOAD_WINSOCK".as_ptr (), def_value: core::ptr::null () },  /* C 2937 */
      env_entry { name: c"emacs_dir".as_ptr (), def_value: c"C:/emacs".as_ptr () },     /* C 2938 */
      env_entry { name: c"EMACSLOADPATH".as_ptr (), def_value: core::ptr::null () },    /* C 2939 */
      env_entry { name: c"SHELL".as_ptr (), def_value: c"cmdproxy.exe".as_ptr () }, /* perhaps it is somewhere on PATH */  /* C 2940 */
      env_entry { name: c"EMACSDATA".as_ptr (), def_value: core::ptr::null () },        /* C 2941 */
      env_entry { name: c"EMACSPATH".as_ptr (), def_value: core::ptr::null () },        /* C 2942 */
      env_entry { name: c"INFOPATH".as_ptr (), def_value: core::ptr::null () },         /* C 2943 */
      env_entry { name: c"EMACSDOC".as_ptr (), def_value: core::ptr::null () },         /* C 2944 */
      env_entry { name: c"TERM".as_ptr (), def_value: c"cmd".as_ptr () },               /* C 2945 */
      env_entry { name: c"LANG".as_ptr (), def_value: core::ptr::null () },             /* C 2946 */
    ];

    const N_ENV_VARS: usize = countof! (DFLT_ENVVARS);                        /* C 2946 */
    /* We need to copy dflt_envvars[] and work on the copy because we
       don't want the dumped Emacs to inherit the values of
       environment variables we saw during dumping (which could be on
       a different system).  The defaults above must be left intact.  */
    let mut env_vars: [env_entry; countof! (DFLT_ENVVARS)]                      /* C 2952 */
      = [env_entry { name: core::ptr::null (), def_value: core::ptr::null () };
         countof! (DFLT_ENVVARS)];

    i = 0;                                                                   /* C 2954 */
    while i < N_ENV_VARS as c_int
      {
	env_vars[i as usize] = DFLT_ENVVARS[i as usize];                      /* C 2955 */
	i += 1;
      }

    /* For backwards compatibility, check if a .emacs file exists in C:/
       If not, then we can try to default to the appdata directory under the
       user's profile, which is more likely to be writable.   */
    if sys_access (c"C:/.emacs".as_ptr (), F_OK) != 0                        /* C 2962 */
      {
	let mut profile_result: HRESULT = 0;                                  /* C 2964 */
	/* Dynamically load ShGetFolderPath, as it won't exist on versions
	   of Windows 95 and NT4 that have not been updated to include
	   MSIE 5.  */
	let mut get_folder_path: ShGetFolderPath_fn;                          /* C 2968 */
	get_folder_path = std::mem::transmute::<VOIDFNPTR, ShGetFolderPath_fn>
	  (get_proc_addr (GetModuleHandle (c"shell32.dll".as_ptr ()),
			  c"SHGetFolderPathA".as_ptr ()));                    /* C 2969-2971 */

	if get_folder_path.is_some ()                                         /* C 2973 */
	  {
	    profile_result = (get_folder_path.unwrap ()) (core::ptr::null_mut (),
							  CSIDL_APPDATA,
							  core::ptr::null_mut (), 0,
							  default_home.as_mut_ptr ());  /* C 2975-2976 */

	    /* If we can't get the appdata dir, revert to old behavior.	 */
	    if profile_result == S_OK                                         /* C 2979 */
	      {
		env_vars[0].def_value = default_home.as_ptr ();               /* C 2981 */
		appdata = 1;                                                  /* C 2982 */
	      }
	  }
      }

  /* Get default locale info and use it for LANG.  */
  if GetLocaleInfo (LOCALE_USER_DEFAULT,                                      /* C 2988-2990 */
		    LOCALE_SABBREVLANGNAME | LOCALE_USE_CP_ACP,
		    locale_name.as_mut_ptr (), core::mem::size_of_val (&locale_name) as c_int) != 0
    {
      /* Microsoft are migrating away of locale IDs, replacing them
	 with locale names, such as "en-US", and are therefore
	 deprecating the APIs which use LCID etc.  As part of that
	 deprecation, they don't bother inventing LCID and LANGID
	 codes for new locales and language/culture combinations;
	 instead, those get LCID of 0xC000 and LANGID of 0x2000, for
	 which the LCID/LANGID oriented APIs return "ZZZ" as the
	 "language name".  Such "language name" is useless for our
	 purposes.  So we instead use the default UI language, in the
	 hope of getting something usable.  */
      if strcmp (locale_name.as_ptr (), c"ZZZ".as_ptr ()) == 0                /* C 3004 */
	{
	  let lang_id: LANGID = get_user_default_ui_language ();              /* C 3006 */

	  if lang_id != 0                                                     /* C 3008 */
	    {
	      /* Disregard the sorting order differences between cultures.  */
	      let def_lcid: LCID = MAKELCID! (lang_id, SORT_DEFAULT);         /* C 3011 */
	      let mut locale_name_def: [c_char; 32] = [0; 32];                /* C 3012 */

	      if GetLocaleInfo (def_lcid,                                     /* C 3014-3016 */
				LOCALE_SABBREVLANGNAME | LOCALE_USE_CP_ACP,
				locale_name_def.as_mut_ptr (),
				core::mem::size_of_val (&locale_name_def) as c_int) != 0
		{
		  strcpy (locale_name.as_mut_ptr (), locale_name_def.as_ptr ());  /* C 3017 */
		}
	    }
	}
      i = 0;                                                                  /* C 3020 */
      while i < N_ENV_VARS as c_int
	{
	  if strcmp (env_vars[i as usize].name, c"LANG".as_ptr ()) == 0       /* C 3022 */
	    {
	      env_vars[i as usize].def_value = locale_name.as_ptr ();         /* C 3024 */
	      break;                                                          /* C 3025 */
	    }
	  i += 1;
	}
    }

/// C 3027: `#define SET_ENV_BUF_SIZE (4 * MAX_PATH)` `/* to cover EMACSLOADPATH */` - C's
/// buffer-size macro, used for the three `char buf[...]` locals below; the port's constant is the
/// same expression.
#[cfg (feature = "windowsnt")]
pub const SET_ENV_BUF_SIZE: usize = (4 * MAX_PATH) as usize;

    /* Treat emacs_dir specially: set it unconditionally based on our
       location.  */
    {                                                                         /* C 3031 */
      let mut p: *mut c_char;                                                 /* C 3033 */
      let mut modname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3034 */

      if GetModuleFileNameA (core::ptr::null_mut (), modname.as_mut_ptr (),
			     MAX_PATH as DWORD) == 0                          /* C 3036 */
	{
	  emacs_abort ();                                                     /* C 3037 */
	}
      p = _mbsrchr (modname.as_ptr (), b'\\' as c_uint);                      /* C 3038 */
      if p.is_null ()                                                         /* C 3038 */
	{
	  emacs_abort ();                                                     /* C 3039 */
	}
      *p = 0;                                                                 /* C 3040 */

      p = _mbsrchr (modname.as_ptr (), b'\\' as c_uint);                      /* C 3042 */
      if !p.is_null ()                                                        /* C 3042-3043 */
	  /* From bin means installed Emacs, from src means uninstalled.  */
	  && (xstrcasecmp (p, c"\\bin".as_ptr ()) == 0
	      || xstrcasecmp (p, c"\\src".as_ptr ()) == 0)
	{
	  let mut buf: [c_char; SET_ENV_BUF_SIZE] = [0; SET_ENV_BUF_SIZE];    /* C 3045 */
	  let within_build_tree: c_int
	    = (xstrcasecmp (p, c"\\src".as_ptr ()) == 0) as c_int;            /* C 3046 */

	  *p = 0;                                                             /* C 3048 */
	  p = modname.as_mut_ptr ();                                          /* C 3049 */
	  while *p != 0                                                       /* C 3049 */
	    {
	      if *p == b'\\' as c_char                                        /* C 3050 */
		{
		  *p = b'/' as c_char;
		}
	      p = CharNext (p);                                               /* C 3049 (the for's increment) */
	    }

	  _snprintf (buf.as_mut_ptr (),
		     core::mem::size_of_val (&buf) - 1,
		     c"emacs_dir=%s".as_ptr (), modname.as_ptr ());           /* C 3052 */
	  _putenv (strdup (buf.as_ptr ()));                                   /* C 3053 */
	  /* If we are running from the Posix-like build tree, define
	     SHELL to point to our own cmdproxy.  The loop below will
	     then disregard PATH_EXEC and the default value.  */
	  if within_build_tree != 0                                           /* C 3057 */
	    {
	      _snprintf (buf.as_mut_ptr (),
			 core::mem::size_of_val (&buf) - 1,
			 c"SHELL=%s/nt/cmdproxy.exe".as_ptr (),
			 modname.as_ptr ());                                  /* C 3059-3061 */
	      _putenv (strdup (buf.as_ptr ()));                              /* C 3062 */
	    }
	}
    }

    i = 0;                                                                    /* C 3066 */
    while i < N_ENV_VARS as c_int                                             /* C 3066 */
      {
	if getenv (env_vars[i as usize].name).is_null ()                      /* C 3068 */
	  {
	    let mut dont_free: c_int = 0;                                     /* C 3070 */
	    let mut bufc: [c_char; SET_ENV_BUF_SIZE] = [0; SET_ENV_BUF_SIZE];  /* C 3071 */

	    lpval = w32_get_resource (REG_ROOT.as_ptr (),                    /* C 3073 */
				      env_vars[i as usize].name,
				      &raw mut dwType as *mut c_ulong);      /* C: LPDWORD, which `include/w32.rs` 311 spells `*mut c_ulong` */
	    if lpval.is_null ()                                               /* C 3073 */
		/* Also ignore empty environment variables.  */
		|| *lpval == 0
	      {
		xfree (lpval as *mut c_void);                                 /* C 3077 */
		dont_free = 1;                                                /* C 3078 */
		if strcmp (env_vars[i as usize].name, c"SHELL".as_ptr ()) == 0  /* C 3079 */
		  {
		    /* Look for cmdproxy.exe in every directory in
		       PATH_EXEC.  FIXME: This does not find cmdproxy
		       in nt/ when we run uninstalled.  */
		    let mut fname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3084 */
		    let mut pstart: *const c_char = PATH_EXEC.as_ptr ();       /* C 3085 */
		    let mut pend: *const c_char;
		    loop
		      {
			pend = _mbschr (pstart, b';' as c_uint);              /* C 3087 */
			if pend.is_null ()                                    /* C 3088 */
			  {
			    pend = pstart.offset (strlen (pstart) as isize);  /* C 3089 */
			  }
			/* Be defensive against series of ;;; characters.  */
			if pend > pstart                                       /* C 3092 */
			  {
			    strncpy (fname.as_mut_ptr (), pstart,
				     pend.offset_from (pstart) as usize);     /* C 3094 */
			    *fname.as_mut_ptr ().offset (pend.offset_from (pstart)) = b'/' as c_char;  /* C 3095 */
			    strcpy (fname.as_mut_ptr ().offset (pend.offset_from (pstart) + 1),
				    c"cmdproxy.exe".as_ptr ());               /* C 3096 */
			    ExpandEnvironmentStrings (fname.as_mut_ptr () as LPSTR,
						      bufc.as_mut_ptr (),
						      core::mem::size_of_val (&bufc) as DWORD);  /* C 3097-3098 */
			    if sys_access (bufc.as_ptr (), F_OK) == 0         /* C 3100 */
			      {
				lpval = bufc.as_mut_ptr () as LPBYTE;         /* C 3102 */
				dwType = REG_SZ;                              /* C 3103 */
				break;                                        /* C 3104 */
			      }
			  }
			if *pend != 0                                         /* C 3107 */
			  {
			    pstart = pend.offset (1);                         /* C 3108 */
			  }
			else
			  {
			    pstart = pend;                                    /* C 3110 */
			  }
			if *pstart == 0                                       /* C 3111 */
			  {
			    /* If not found in any directory, use the
			       default as the last resort.  */
			    lpval = env_vars[i as usize].def_value as LPBYTE;  /* C 3115 */
			    dwType = REG_EXPAND_SZ;                           /* C 3116 */
			  }
			if *pstart == 0                                       /* C 3118 */
			  {
			    break;                                            /* C 3118 */
			  }
		      }
		  }
		else
		  {
		    lpval = env_vars[i as usize].def_value as LPBYTE;         /* C 3123 */
		    dwType = REG_EXPAND_SZ;                                   /* C 3124 */
		  }
		if strcmp (env_vars[i as usize].name, c"HOME".as_ptr ()) == 0 && appdata == 0  /* C 3126 */
		  {
		    Vdelayed_warnings_list                                  /* C 3127-3132 */
		      = Fcons
		      (list2 (intern (c"initialization".as_ptr ()),
			      build_string (c"Use of `C:\\.emacs' without defining `HOME'\nin the environment is deprecated, see `Windows HOME' in the Emacs manual.".as_ptr ())),
		       Vdelayed_warnings_list);
		  }
	      }

	    if !lpval.is_null ()                                              /* C 3135 */
	      {
		let mut buf1: [c_char; SET_ENV_BUF_SIZE] = [0; SET_ENV_BUF_SIZE];  /* C 3137 */
		let mut buf2: [c_char; SET_ENV_BUF_SIZE] = [0; SET_ENV_BUF_SIZE];

		if dwType == REG_EXPAND_SZ                                     /* C 3139 */
		  {
		    ExpandEnvironmentStrings (lpval as LPSTR, buf1.as_mut_ptr (),
					      core::mem::size_of_val (&buf1) as DWORD);  /* C 3140 */
		  }
		else if dwType == REG_SZ                                      /* C 3141 */
		  {
		    strcpy (buf1.as_mut_ptr (), lpval as *const c_char);      /* C 3142 */
		  }
		if dwType == REG_EXPAND_SZ || dwType == REG_SZ                /* C 3143 */
		  {
		    _snprintf (buf2.as_mut_ptr (),
			       core::mem::size_of_val (&buf2) - 1,
			       c"%s=%s".as_ptr (), env_vars[i as usize].name,
			       buf1.as_ptr ());                                  /* C 3145-3146 */
		    _putenv (strdup (buf2.as_ptr ()));                        /* C 3147 */
		  }

		if dont_free == 0                                             /* C 3150 */
		  {
		    xfree (lpval as *mut c_void);                             /* C 3151 */
		  }
	      }
	  }
	i += 1;                                                              /* C 3066 (the for's increment) */
      }
  }

  /* Rebuild system configuration to reflect invoking system.  */
  Vsystem_configuration = build_string (EMACS_CONFIGURATION! ());              /* C 3158 */

  /* Another special case: on NT, the PATH variable is actually named
     "Path" although cmd.exe (perhaps NT itself) arranges for
     environment variable lookup and setting to be case insensitive.
     However, Emacs assumes a fully case sensitive environment, so we
     need to change "Path" to "PATH" to match the expectations of
     various elisp packages.  We do this by the sneaky method of
     modifying the string in the C runtime environ entry.

     The same applies to COMSPEC.  */
  {                                                                           /* C 3169 */
    let mut envp: *mut *mut c_char;                                          /* C 3170 */
    let path: *const c_char = c"PATH=".as_ptr ();                            /* C 3171 */
    let path_len: c_int = strlen (path) as c_int;
    let comspec: *const c_char = c"COMSPEC=".as_ptr ();                      /* C 3173 */
    let comspec_len: c_int = strlen (comspec) as c_int;                      /* C 3174 */

    envp = environ;                                                          /* C 3176 */
    while *envp != core::ptr::null_mut ()                                    /* C 3176 */
      {
	if _strnicmp (*envp, path, path_len as usize) == 0                    /* C 3177 */
	  {
	    memcpy (*envp as *mut c_void, path as *const c_void,
		    path_len as usize);                                       /* C 3178 */
	  }
	else if _strnicmp (*envp, comspec, comspec_len as usize) == 0         /* C 3179 */
	  {
	    memcpy (*envp as *mut c_void, comspec as *const c_void,
		    comspec_len as usize);                                    /* C 3180 */
	  }
	envp = envp.offset (1);                                               /* C 3176 (the for's increment) */
      }

    /* Make the same modification to `process-environment' which has
       already been initialized in set_initial_environment.  */
    let mut env: Lisp_Object = Vprocess_environment;                         /* C 3184 */
    while CONSP (env)                                                        /* C 3184 */
    {
      let entry: Lisp_Object = XCAR (env);                                   /* C 3186 */
      if _strnicmp (SDATA (entry) as *const c_char, path,                        /* C 3187 */
		    path_len as usize) == 0
        {
	  let mut i: c_int = 0;                                               /* C 3188 */
	  while i < path_len
	    {
	      SSET (entry, i as usize, *path.offset (i as isize) as u8);      /* C 3189 */
	      i += 1;
	    }
	}
      else if _strnicmp (SDATA (entry) as *const c_char, comspec,                /* C 3191 */
			 comspec_len as usize) == 0
        {
	  let mut i: c_int = 0;                                               /* C 3191 */
	  while i < comspec_len
	    {
	      SSET (entry, i as usize, *comspec.offset (i as isize) as u8);   /* C 3192 */
	      i += 1;
	    }
	}
      env = XCDR (env);                                                      /* C 3184 (the for's increment) */
    }
  }

  /* Remember the initial working directory for getcwd.  */
  w32_get_current_directory (startup_dir.as_mut_ptr ());                      /* C 3197 */

  {                                                                           /* C 3199 */
    static mut modname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3200 */

    if GetModuleFileNameA (core::ptr::null_mut (), modname.as_mut_ptr (),
			   MAX_PATH as DWORD) == 0                            /* C 3202 */
      {
	emacs_abort ();                                                       /* C 3203 */
      }
    *argv.offset (0) = modname.as_mut_ptr ();                                 /* C 3204 */
  }

  /* Determine if there is a middle mouse button, to allow parse_button
     to decide whether right mouse events should be mouse-2 or
     mouse-3. */
  w32_num_mouse_buttons = GetSystemMetrics (SM_CMOUSEBUTTONS) as EMACS_INT;   /* C 3210 */

  init_user_info ();                                                          /* C 3212 */
}

/* C 3215-3230: `emacs_root_dir` - "Called from expand-file-name when default-directory is not a
   string."  C's `root_dir[parse_root (root_dir, NULL)] = '\0'` indexes with `parse_root`'s return
   value.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_root_dir () -> *mut c_char
{
  static mut root_dir: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3220 */
  let mut p: *const c_char;                                                   /* C 3221 */

  p = getenv (c"emacs_dir".as_ptr ());                                        /* C 3223 */
  if p.is_null ()                                                             /* C 3224 */
    {
      emacs_abort ();                                                         /* C 3225 */
    }
  filename_from_ansi (p, root_dir.as_mut_ptr ());                             /* C 3226 */
  *root_dir.as_mut_ptr ().offset (parse_root (root_dir.as_ptr (),
					      core::ptr::null_mut ()) as isize) = 0;  /* C 3227 */
  dostounix_filename (root_dir.as_mut_ptr ());                                /* C 3228 */
  return root_dir.as_mut_ptr ();                                              /* C 3229 */
}

/* C 3232-3270: `fdutimens` - "Set the access and modification time stamps of FD (a.k.a. FILE) to
   be TIMESPEC[0] and TIMESPEC[1]", ending in `utimensat`, whose definition is past this slice
   (C 5946, the ledger's P-91 (4) row).  C's `struct _utimbuf _ut;` is the CRT record of the
   section above and its two `time_t` stores are the record's members.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdutimens (fd: c_int, file: *const c_char,
                                    timespec: *const timespec) -> c_int
{
  if timespec.is_null ()                                                      /* C 3246 */
    {
      *errno () = ENOSYS;                                                     /* C 3248 */
      return -1;                                                              /* C 3249 */
    }
  if fd < 0 && file.is_null ()                                                /* C 3251 */
    {
      *errno () = EBADF;                                                      /* C 3253 */
      return -1;                                                              /* C 3254 */
    }
  /* _futime's prototype defines 2nd arg as having the type 'struct
     _utimbuf', while utime needs to accept 'struct utimbuf' for
     compatibility with Posix.  So we need to use 2 different (but
     equivalent) types to avoid compiler warnings, sigh.  */
  if fd >= 0                                                                  /* C 3260 */
    {
      let mut _ut: _utimbuf = core::mem::zeroed ();                           /* C 3263 */

      _ut.actime = (*timespec.offset (0)).tv_sec;                             /* C 3264 */
      _ut.modtime = (*timespec.offset (1)).tv_sec;                            /* C 3265 */
      return _futime (fd, &raw mut _ut);                                      /* C 3266 */
    }
  else
    {
      return utimensat (fd, file, timespec, 0);                               /* C 3269 */
    }
}

/* C 3272-3284: the two ASCII banners and `sys_ctime` - "Place a wrapper around the MSVC version of
   ctime.  It returns NULL on network directories, so we handle that case here.
   (Ulrich Leodolter, 1/11/95)."  C's `(char *)"…"` cast on the fallback literal is the port's
   `as *mut c_char`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_ctime (t: *const time_t) -> *mut c_char
{
  let str_: *mut c_char = ctime (t);                                          /* C 3281 */

  return if !str_.is_null () { str_ }                                         /* C 3282 */
         else { c"Sun Jan 01 00:00:00 1970".as_ptr () as *mut c_char };
}

/* C 3286-3293: `sys_sleep` - "Emulate sleep…we could have done this with a define, but that would
   necessitate including windows.h in the files that used it.  This is much easier."  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_sleep (seconds: c_int)
{
  Sleep ((seconds * 1000) as DWORD);                                          /* C 3292 */
}

/* C 3295-3297: the two MSVC-internal declarations C keeps outside any header -
   "Internal MSVC functions for low-level descriptor munging"; C's `__cdecl` is the port's `"C"`.
   C: extern int __cdecl _set_osfhnd (int fd, long h);
   C: extern int __cdecl _free_osfhnd (int fd);  */
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _set_osfhnd (fd: c_int, h: c_long) -> c_int;
  pub fn _free_osfhnd (fd: c_int) -> c_int;
}

/* C 3299-3300: `/* parallel array of private info on file handles */` and
   `filedesc fd_info [ MAXDESC ];` - C's tentative definition, so the port's `[filedesc; MAXDESC]`
   zeroed (each member zero, as C's static storage is) and `#[unsafe(no_mangle)]` because C's
   definition is external.  `filedesc` and `MAXDESC` are `include/w32.rs`'s (its `w32_objs`
   module, whose glob import this definition shadows).  */
#[cfg (feature = "windowsnt")]
const FILEDESC_INIT: filedesc = filedesc { flags: 0, hnd: core::ptr::null_mut (),
                                           cp: core::ptr::null_mut () };
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut fd_info: [filedesc; MAXDESC as usize] = [FILEDESC_INIT; MAXDESC as usize];

/* C 3302-3334: `typedef struct volume_info_data { … } volume_info_data;` - C's seven members in
   C's order (`next` is the self-referential pointer, `type` is C's member name and so the port's
   raw identifier) - `static volume_info_data volume_info;` (the object "Global referenced by
   various functions"), `static BOOL fixed_drives[26];` (the fixed-drive table `VOLINFO_STILL_VALID`
   reads) and C's two macros, which are the port's `macro_rules!` of C's own expressions.  C's
   `#define` macros re-evaluate their arguments at every use, and so do these.  */
/* C: typedef struct volume_info_data {
   C:   struct volume_info_data * next;
   C:   DWORD     timestamp;
   C:   char *    root_dir;
   C:   DWORD     serialnum;
   C:   DWORD     maxcomp;
   C:   DWORD     flags;
   C:   char *    name;
   C:   char *    type;
   C: } volume_info_data;  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct volume_info_data
{
  pub next: *mut volume_info_data,
  pub timestamp: DWORD,
  pub root_dir: *mut c_char,
  pub serialnum: DWORD,
  pub maxcomp: DWORD,
  pub flags: DWORD,
  pub name: *mut c_char,
  pub r#type: *mut c_char,
}
#[cfg (feature = "windowsnt")]
static mut volume_info: volume_info_data = volume_info_data
{
  next: core::ptr::null_mut (), timestamp: 0, root_dir: core::ptr::null_mut (),
  serialnum: 0, maxcomp: 0, flags: 0, name: core::ptr::null_mut (),
  r#type: core::ptr::null_mut (),
};                                                                            /* C 3318 */
#[cfg (feature = "windowsnt")]
static mut fixed_drives: [BOOL; 26] = [0; 26];                                /* C 3322 */
/// C 3335: `#define DRIVE_INDEX( c ) ( (c) <= 'Z' ? (c) - 'A' : (c) - 'a' )`
#[cfg (feature = "windowsnt")]
macro_rules! DRIVE_INDEX {
  ($c:expr) => { (if ($c) <= b'Z' as c_int { ($c) - (b'A' as c_int) } else { ($c) - (b'a' as c_int) }) };
}
/// C 3327-3330: `#define VOLINFO_STILL_VALID( root_dir, info ) …` - the `c_isalpha` test plus the
/// ten-second expiry on `GetTickCount ()`.
/// C: #define VOLINFO_STILL_VALID( root_dir, info )		\
/// C:   ( ( c_isalpha (root_dir[0]) &&				\
/// C:       fixed_drives[ DRIVE_INDEX (root_dir[0]) ] )	\
/// C:     || GetTickCount () - info->timestamp < 10000 )
#[cfg (feature = "windowsnt")]
macro_rules! VOLINFO_STILL_VALID {
  ($root_dir:expr, $info:expr) => {
    ((c_isalpha (*($root_dir).offset (0) as c_int)
      && fixed_drives[DRIVE_INDEX! (*($root_dir).offset (0) as c_int) as usize] != 0)
     || (GetTickCount ()).wrapping_sub ((*($info)).timestamp) < 10000) };
}

/* C 3341-3348: `/* Cache support functions.  */`, `/* Simple linked list with linear search is
   sufficient.  */` and `static volume_info_data *volume_cache = NULL;`.  */
/* C 3337-3346: `lookup_volume_info` - C's `for (info = volume_cache; info; info = info->next)`
   with the `xstrcasecmp` break.  */
#[cfg (feature = "windowsnt")]
static mut volume_cache: *mut volume_info_data = core::ptr::null_mut ();      /* C 3335 */
#[cfg (feature = "windowsnt")]
unsafe fn lookup_volume_info (root_dir: *mut c_char) -> *mut volume_info_data
{
  let mut info: *mut volume_info_data;                                        /* C 3340 */

  info = volume_cache;                                                        /* C 3342 */
  while !info.is_null ()                                                      /* C 3342 */
    {
      if xstrcasecmp ((*info).root_dir, root_dir) == 0                        /* C 3343 */
	{
	  break;
	}
      info = (*info).next;                                                    /* C 3354 (the for's increment) */
    }
  return info;                                                                /* C 3344 */
}

/* C 3348-3355: `add_volume_info`.  */
#[cfg (feature = "windowsnt")]
unsafe fn add_volume_info (root_dir: *mut c_char, info: *mut volume_info_data)
{
  (*info).root_dir = xstrdup (root_dir);                                      /* C 3350 */
  unixtodos_filename ((*info).root_dir);                                      /* C 3351 */
  (*info).next = volume_cache;                                                /* C 3352 */
  volume_cache = info;                                                        /* C 3353 */
}

/* C 3362-3477: `GetCachedVolumeInformation` - "Wrapper for GetVolumeInformation, which uses
   caching to avoid performance penalty…".  C's uninitialized `DWORD serialnum/maxcomp/flags` are
   zeroed (PORTING.md 1.5), the two `char [MAX_PATH+1]` locals are C's, C's `(const char **)&root_dir`
   cast is the port's `&raw mut … as *mut *const c_char`, the local named `type` is the port's raw
   identifier, and C 3390-3406's `#if 0` block (`WNetGetConnection`) is dead text carried as C's
   own text (the `ccl.rs` 2599 reading).
   C: #if 0
   C:   /* No point doing this, because WNetGetConnection is even slower than
   C:      GetVolumeInformation, consistently taking ~50ms on a 486 (FWIW,
   C:      GetDriveType is about the only call of this type which does not
   C:      involve network access, and so is extremely quick).  */
   C:   /* Map drive letter to UNC if remote. */
   C:   if (c_isalpha (root_dir[0]) && !fixed[DRIVE_INDEX (root_dir[0])])
   C:     {
   C:       char remote_name[ 256 ];
   C:       char drive[3] = { root_dir[0], ':' };
   C:       if (WNetGetConnection (drive, remote_name, sizeof (remote_name))
   C: 	  == NO_ERROR)
   C: 	/* do something */ ;
   C:     }
   C: #endif  */
#[cfg (feature = "windowsnt")]
unsafe fn GetCachedVolumeInformation (root_dir: *mut c_char) -> *mut volume_info_data
{
  let mut info: *mut volume_info_data;                                        /* C 3364 */
  let mut default_root: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3365 */
  let mut name: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3366 */
  let mut r#type: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3367 */

  /* NULL for root_dir means use root from current directory.  */
  if root_dir.is_null ()                                                      /* C 3370 */
    {
      w32_get_current_directory (default_root.as_mut_ptr ());                 /* C 3372 */
      parse_root (default_root.as_ptr (),
		  &raw mut root_dir as *mut *const c_char);                   /* C 3373 */
      *root_dir = 0;                                                          /* C 3374 */
      root_dir = default_root.as_mut_ptr ();                                  /* C 3375 */
    }

  /* Local fixed drives can be cached permanently.  Removable drives
     cannot be cached permanently, since the volume name and serial
     number (if nothing else) can change.  Remote drives should be
     treated as if they are removable, since there is no sure way to
     tell whether they are or not.  Also, the UNC association of drive
     letters mapped to remote volumes can be changed at any time (even
     by other processes) without notice.

     As a compromise, so we can benefit from caching info for remote
     volumes, we use a simple expiry mechanism to invalidate cache
     entries that are more than ten seconds old.  */

  info = lookup_volume_info (root_dir);                                       /* C 3408 */

  if info.is_null () || !VOLINFO_STILL_VALID! (root_dir, info)                /* C 3410 */
    {
      let mut serialnum: DWORD = 0;                                           /* C 3412 */
      let mut maxcomp: DWORD = 0;
      let mut flags: DWORD = 0;

      /* Info is not cached, or is stale. */
      if w32_unicode_filenames                                                /* C 3417 */
	{
	  let mut root_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3419 */
	  let mut name_w: [c_ushort; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3420 */
	  let mut type_w: [c_ushort; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3421 */

	  filename_to_utf16 (root_dir, root_w.as_mut_ptr ());                   /* C 3423 */
	  if GetVolumeInformationW (root_w.as_mut_ptr (),                       /* C 3424-3429 */
				    name_w.as_mut_ptr (),
				    core::mem::size_of_val (&name_w) as DWORD,
				    &raw mut serialnum,
				    &raw mut maxcomp,
				    &raw mut flags,
				    type_w.as_mut_ptr (),
				    core::mem::size_of_val (&type_w) as DWORD) == 0
	    {
	      return core::ptr::null_mut ();                                    /* C 3430 */
	    }
	  /* Hmm... not really 100% correct, as these 2 are not file
	     names...  */
	  filename_from_utf16 (name_w.as_mut_ptr (), name.as_mut_ptr ());       /* C 3433 */
	  filename_from_utf16 (type_w.as_mut_ptr (), r#type.as_mut_ptr ());     /* C 3434 */
	}
      else
	{
	  let mut root_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3438 */
	  let mut name_a: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3439 */
	  let mut type_a: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1];  /* C 3440 */

	  filename_to_ansi (root_dir, root_a.as_mut_ptr ());                    /* C 3442 */
	  if GetVolumeInformationA (root_a.as_mut_ptr (),                       /* C 3443-3448 */
				    name_a.as_mut_ptr (),
				    core::mem::size_of_val (&name_a) as DWORD,
				    &raw mut serialnum,
				    &raw mut maxcomp,
				    &raw mut flags,
				    type_a.as_mut_ptr (),
				    core::mem::size_of_val (&type_a) as DWORD) == 0
	    {
	      return core::ptr::null_mut ();                                    /* C 3449 */
	    }
	  filename_from_ansi (name_a.as_mut_ptr (), name.as_mut_ptr ());       /* C 3450 */
	  filename_from_ansi (type_a.as_mut_ptr (), r#type.as_mut_ptr ());     /* C 3451 */
	}

      /* Cache the volume information for future use, overwriting existing
	 entry if present.  */
      if info.is_null ()                                                      /* C 3456 */
	{
	  info = xmalloc (core::mem::size_of::<volume_info_data> ()) as *mut volume_info_data;  /* C 3458 */
	  add_volume_info (root_dir, info);                                     /* C 3459 */
	}
      else
	{
	  xfree ((*info).name as *mut c_void);                                  /* C 3463 */
	  xfree ((*info).r#type as *mut c_void);                                /* C 3464 */
	}

      (*info).name = xstrdup (name.as_mut_ptr ());                            /* C 3467 */
      unixtodos_filename ((*info).name);                                      /* C 3468 */
      (*info).serialnum = serialnum;                                          /* C 3469 */
      (*info).maxcomp = maxcomp;                                              /* C 3470 */
      (*info).flags = flags;                                                  /* C 3471 */
      (*info).r#type = xstrdup (r#type.as_mut_ptr ());                        /* C 3472 */
      (*info).timestamp = GetTickCount ();                                    /* C 3473 */
    }

  return info;                                                                /* C 3476 */
}

/* C 3479-3516: `get_volume_info` - "Get information on the volume where NAME is held; set path
   pointer to start of pathname in NAME (past UNC header\volume header if present), if pPath is
   non-NULL."  C's `volume_info = *info;` is the whole-record copy, and its `char *rootname = NULL;`
   is C's own initializer.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_volume_info (name: *const c_char, pPath: *mut *const c_char) -> c_int
{
  let mut temp: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3491 */
  let mut rootname: *mut c_char = core::ptr::null_mut (); /* default to current volume */  /* C 3492 */
  let mut info: *mut volume_info_data;                                        /* C 3493 */
  let root_len: c_int = parse_root (name, pPath);                             /* C 3494 */

  if name.is_null ()                                                          /* C 3496 */
    {
      return FALSE;                                                           /* C 3497 */
    }

  /* Copy the root name of the volume, if given.  */
  if root_len != 0                                                            /* C 3500 */
    {
      strncpy (temp.as_mut_ptr (), name, root_len as usize);                  /* C 3502 */
      *temp.as_mut_ptr ().offset (root_len as isize) = 0;                     /* C 3503 */
      unixtodos_filename (temp.as_mut_ptr ());                                /* C 3504 */
      rootname = temp.as_mut_ptr ();                                          /* C 3505 */
    }

  info = GetCachedVolumeInformation (rootname);                               /* C 3508 */
  if !info.is_null ()                                                         /* C 3509 */
    {
      /* Set global referenced by other functions.  */
      volume_info = *info;                                                    /* C 3512 */
      return TRUE;                                                            /* C 3513 */
    }
  return FALSE;                                                               /* C 3515 */
}

/* C 3518-3527: `is_fat_volume` - "Determine if volume is FAT format (ie. only supports short 8.3
   names); also set path pointer to start of pathname in name, if pPath is non-NULL."  */
#[cfg (feature = "windowsnt")]
unsafe fn is_fat_volume (name: *const c_char, pPath: *mut *const c_char) -> c_int
{
  if get_volume_info (name, pPath) != 0                                       /* C 3522 */
    {
      return (volume_info.maxcomp == 12) as c_int;                            /* C 3523 */
    }
  return FALSE;                                                               /* C 3524 */
}

/* C 3528-3631: `map_w32_filename` - "Convert all slashes in a filename to backslashes, and map
   filename to a valid 8.3 name if necessary.  The result is a pointer to a static buffer, so
   CAVEAT EMPTOR!"  C's uninitialized `char * path` is zeroed (PORTING.md 1.5: C reads it only
   after `is_fat_volume` has written it, and the port initializes it because Rust requires a
   value), the `for`-less `while ((c = *name++))` keeps C's assign-then-test order, the three
   `case` groups of the `switch` are C's (the `~`/`#` arm falls through to `default`, which the
   port spells by repeating that body - PORTING.md 2.2), and `c_tolower`'s `int` result narrows
   back to `char` as C's assignment does.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_w32_filename (name: *const c_char,
                                           pPath: *mut *const c_char) -> *const c_char
{
  static mut shortname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3533 */
  let mut str_: *mut c_char = shortname.as_mut_ptr ();                        /* C 3534 */
  let mut c: c_char = 0;                                                      /* C 3535 */
  let mut path: *mut c_char = core::ptr::null_mut ();                         /* C 3536 */
  let mut name: *const c_char = name;
  let save_name: *const c_char = name;                                        /* C 3537 */

  if strlen (name) >= core::mem::size_of_val (&shortname) as usize            /* C 3539 */
    {
      /* Return a filename which will cause callers to fail.  */
      strcpy (shortname.as_mut_ptr (), c"?".as_ptr ());                       /* C 3544 */
      return shortname.as_ptr ();                                             /* C 3545 */
    }

  if !fatal_error_in_progress                                                /* C 3548 */
      && is_fat_volume (name, &raw mut path as *mut *const c_char) != 0      /* truncate to 8.3 */
    {
      let mut left: c_int = 8;	/* maximum number of chars in part */        /* C 3551 */
      let mut extn: c_int = 0;	/* extension added? */                     /* C 3552 */
      let mut dots: c_int = 2;	/* maximum number of dots allowed */       /* C 3553 */

      while (name as usize) < (path as usize)                                /* C 3555 */
	{
	  *str_ = *name;	/* skip past UNC header */                     /* C 3556 */
	  str_ = str_.offset (1);
	  name = name.offset (1);
	}

      loop                                                                   /* C 3558 */
        {
	  c = *name; name = name.offset (1);
	  if !(c != 0) { break; }
	  match c as c_uchar
	    {
	      b':' | b'\\' | b'/' =>                                         /* C 3562-3569 */
		{
		  *str_ = if c == b':' as c_char { b':' as c_char } else { b'\\' as c_char };
		  str_ = str_.offset (1);
		  extn = 0;		/* reset extension flags */            /* C 3566 */
		  dots = 2;		/* max 2 dots */                       /* C 3567 */
		  left = 8;		/* max length 8 for main part */       /* C 3568 */
		}
	      b'.' =>                                                        /* C 3570 */
		{
		  if dots != 0
		    {
		      /* Convert path components of the form .xxx to _xxx,
			 but leave . and .. as they are.  This allows .emacs
			 to be read as _emacs, for example.  */

		      if *name == 0 || *name == b'.' as c_char
			  || IS_DIRECTORY_SEP! (*name as c_int)                  /* C 3577-3579 */
			{
			  *str_ = b'.' as c_char; str_ = str_.offset (1);
			  dots -= 1;
			}
		      else
			{
			  *str_ = b'_' as c_char; str_ = str_.offset (1);
			  left -= 1;
			  dots = 0;
			}
		    }
		  else if extn == 0
		    {
		      *str_ = b'.' as c_char; str_ = str_.offset (1);
		      extn = 1;		/* we've got an extension */
		      left = 3;		/* 3 chars in extension */
		    }
		  else
		    {
		      /* any embedded dots after the first are converted to _ */
		      *str_ = b'_' as c_char; str_ = str_.offset (1);
		    }
		}
	      b'~' | b'#' =>                                                 /* C 3603 */
		{
		  /* don't lose these, they're important */
		  if left == 0
		    {
		      *str_.offset (-1) = c;	/* replace last character of part */  /* C 3606 */
		    }
		  /* FALLTHRU */
		  if left != 0 && b'A' as c_char <= c && c <= b'Z' as c_char     /* C 3610 */
		    {
		      *str_ = c_tolower (c as c_int) as c_char; /* map to lower case (looks nicer) */
		      str_ = str_.offset (1);
		      left -= 1;
		      dots = 0;		  /* started a path component */
		    }
		}
	      _ =>
		{
		  if left != 0 && b'A' as c_char <= c && c <= b'Z' as c_char     /* C 3610 */
		    {
		      *str_ = c_tolower (c as c_int) as c_char;
		      str_ = str_.offset (1);
		      left -= 1;
		      dots = 0;
		    }
		}
	    }
	}
      *str_ = 0;                                                              /* C 3619 */
    }
  else
    {
      strcpy (shortname.as_mut_ptr (), name);                                 /* C 3623 */
      unixtodos_filename (shortname.as_mut_ptr ());                           /* C 3624 */
    }

  if !pPath.is_null ()                                                        /* C 3613 */
    {
      *pPath = shortname.as_ptr ().offset (path.offset_from (save_name));      /* C 3628 */
    }

  return shortname.as_ptr ();                                                 /* C 3630 */
}

/* C 3633-3643: `is_exec` - C's one `strrchr` and its four `xstrcasecmp` extensions.  */
#[cfg (feature = "windowsnt")]
unsafe fn is_exec (name: *const c_char) -> c_int
{
  let p: *mut c_char = strrchr (name, b'.' as c_int);                         /* C 3636 */

  return (!p.is_null ()                                                       /* C 3638-3642 */
	  && (xstrcasecmp (p, c".exe".as_ptr ()) == 0
	      || xstrcasecmp (p, c".com".as_ptr ()) == 0
	      || xstrcasecmp (p, c".bat".as_ptr ()) == 0
	      || xstrcasecmp (p, c".cmd".as_ptr ()) == 0)) as c_int;
}

/* C 3645-3666: the `/* Emulate the Unix directory procedures opendir, closedir, and readdir.  … */
   banner, the file's directory statics, the fourteen `DIR_FIND_DATA_*`/`last_dir_find_data`
   objects, the UNC enumeration handle and the three forward declarations of `open_unc_volume`/
   `read_unc_volume`/`close_unc_volume` (definitions at C 3888/3941/3994, so they have no Rust
   item - PORTING.md 0.1-12 - and are recorded here).
   C 3664-3666 (dropped):
   C: static HANDLE open_unc_volume (const char *);
   C: static void  *read_unc_volume (HANDLE, wchar_t *, char *, int);
   C: static void   close_unc_volume (HANDLE);  */
#[cfg (feature = "windowsnt")]
static mut dir_static: dirent = dirent                           /* C 3651 */
{
  d_ino: 0, d_reclen: 0, d_namlen: 0, d_type: 0,
  d_time_create: 0, d_time_access: 0, d_time_write: 0, d_size: 0,
  d_name: [0; (MAXNAMLEN * 4 + 1) as usize],
};
#[cfg (feature = "windowsnt")]
static mut dir_find_handle: HANDLE = INVALID_HANDLE_VALUE;                    /* C 3652 */
#[cfg (feature = "windowsnt")]
static mut dir_is_fat: c_int = 0;                                             /* C 3653 */
#[cfg (feature = "windowsnt")]
static mut dir_pathname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3654 */
#[cfg (feature = "windowsnt")]
static mut dir_find_data_w: WIN32_FIND_DATAW = WIN32_FIND_DATAW   /* C 3655 */
{
  dwFileAttributes: 0, ftCreationTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  ftLastAccessTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  ftLastWriteTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  nFileSizeHigh: 0, nFileSizeLow: 0, dwReserved0: 0, dwReserved1: 0,
  cFileName: [0; MAX_PATH as usize], cAlternateFileName: [0; 14],
};
#[cfg (feature = "windowsnt")]
static mut dir_find_data_a: WIN32_FIND_DATAA = WIN32_FIND_DATAA   /* C 3656 */
{
  dwFileAttributes: 0, ftCreationTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  ftLastAccessTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  ftLastWriteTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
  nFileSizeHigh: 0, nFileSizeLow: 0, dwReserved0: 0, dwReserved1: 0,
  cFileName: [0; MAX_PATH as usize], cAlternateFileName: [0; 14],
};
#[cfg (feature = "windowsnt")]
pub const DIR_FIND_DATA_W: c_int = 1;                                         /* C 3657 */
#[cfg (feature = "windowsnt")]
pub const DIR_FIND_DATA_A: c_int = 2;                                         /* C 3658 */
#[cfg (feature = "windowsnt")]
static mut last_dir_find_data: c_int = -1;                                    /* C 3659 */

/* Support shares on a network resource as subdirectories of a read-only
   root directory. */
#[cfg (feature = "windowsnt")]
static mut wnet_enum_handle: HANDLE = INVALID_HANDLE_VALUE;                   /* C 3665 */

/* C 3669-3706: `sys_opendir` - "Opening is done by FindFirstFile.  However, a read is inherent to
   this operation, so we defer the open until read time."  C's `dirp = (DIR *) malloc (sizeof
   (DIR))` truth test is the port's `is_null ()`, and `map_w32_filename`/`is_fat_volume`/
   `is_unc_volume`/`open_unc_volume` are this file's own definitions (the last one past this
   slice).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_opendir (filename: *const c_char) -> *mut DIR
{
  let mut dirp: *mut DIR;                                                     /* C 3671 */

  /* Opening is done by FindFirstFile.  However, a read is inherent to
     this operation, so we defer the open until read time.  */

  if dir_find_handle != INVALID_HANDLE_VALUE                                  /* C 3677 */
    {
      return core::ptr::null_mut ();                                          /* C 3678 */
    }
  if wnet_enum_handle != INVALID_HANDLE_VALUE                                 /* C 3679 */
    {
      return core::ptr::null_mut ();                                          /* C 3680 */
    }

  /* Note: We don't support traversal of UNC volumes via symlinks.
     Doing so would mean punishing 99.99% of use cases by resolving
     all the possible symlinks in FILENAME, recursively. */
  if is_unc_volume (filename) != 0                                            /* C 3686 */
    {
      wnet_enum_handle = open_unc_volume (filename);                          /* C 3688 */
      if wnet_enum_handle == INVALID_HANDLE_VALUE                             /* C 3689 */
	{
	  return core::ptr::null_mut ();                                      /* C 3690 */
	}
    }

  dirp = malloc (core::mem::size_of::<DIR> ()) as *mut DIR;                   /* C 3693 */
  if dirp.is_null ()                                                          /* C 3693 */
    {
      return core::ptr::null_mut ();                                          /* C 3694 */
    }

  (*dirp).dd_fd = 0;                                                          /* C 3696 */
  (*dirp).dd_loc = 0;                                                         /* C 3697 */
  (*dirp).dd_size = 0;                                                        /* C 3698 */

  strncpy (dir_pathname.as_mut_ptr (), map_w32_filename (filename,
							 core::ptr::null_mut ()),
	   MAX_UTF8_PATH as usize - 1);                                       /* C 3700-3701 */
  *dir_pathname.as_mut_ptr ().offset (MAX_UTF8_PATH as isize - 1) = 0;        /* C 3702 */
  /* Note: We don't support symlinks to file names on FAT volumes.
     Doing so would mean punishing 99.99% of use cases by resolving
     all the possible symlinks in FILENAME, recursively.  */
  dir_is_fat = is_fat_volume (filename, core::ptr::null_mut ());              /* C 3706 */

  return dirp;                                                                /* C 3707 */
}

/* C 3709-3723: `sys_closedir` - the two handle branches C tests ("If we have a find-handle open,
   close it.") and the `xfree` of the caller's `DIR`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_closedir (dirp: *mut DIR)
{
  /* If we have a find-handle open, close it.  */
  if dir_find_handle != INVALID_HANDLE_VALUE                                  /* C 3713 */
    {
      FindClose (dir_find_handle);                                            /* C 3715 */
      dir_find_handle = INVALID_HANDLE_VALUE;                                 /* C 3716 */
    }
  else if wnet_enum_handle != INVALID_HANDLE_VALUE                            /* C 3718 */
    {
      close_unc_volume (wnet_enum_handle);                                    /* C 3720 */
      wnet_enum_handle = INVALID_HANDLE_VALUE;                                /* C 3721 */
    }
  xfree (dirp as *mut c_char as *mut c_void);                                 /* C 3723 */
}

/* C 3725-3740: the two wide literals `sys_readdir` passes to `wcscat` (a C `L"…"` has no Rust
   syntax - the `w32fns.rs` 385 reading).
   C: wcscat (fnw, L"\\");
   C: wcscat (fnw, L"*");  */
#[cfg (feature = "windowsnt")]
static BACKSLASH_W: [WCHAR; 2] = [0x005C, 0x0000];
#[cfg (feature = "windowsnt")]
static STAR_W: [WCHAR; 2] = [0x002A, 0x0000];

/* C 3726-3885: `sys_readdir` - the last definition of this slice but for `open_unc_volume`'s group.
   C's `switch (GetLastError ())` has a `FALLTHROUGH` into an *empty* `default`, so the port's
   match keeps C's arms (with that case first: its fallthrough body is empty - PORTING.md 2.2),
   C's `_mbspbrk`/`_mbslwr`/`CharLowerW`/`wic` paths are the ANSI and wide arms, and C's
   `sizeof (struct dirent) - MAX_UTF8_PATH + 3 + …` keeps its *unsigned* arithmetic (PORTING.md 1.1:
   the subtractions wrap, hence `wrapping_sub`).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_readdir (dirp: *mut DIR) -> *mut dirent
{
  let mut downcase: c_int = (!NILP (Vw32_downcase_file_names)) as c_int;      /* C 3728 */

  if wnet_enum_handle != INVALID_HANDLE_VALUE                                 /* C 3730 */
    {
      if read_unc_volume (wnet_enum_handle,
			  dir_find_data_w.cFileName.as_mut_ptr (),
			  dir_find_data_a.cFileName.as_mut_ptr (),
			  MAX_PATH).is_null ()                                /* C 3731-3734 */
	{
	  return core::ptr::null_mut ();                                      /* C 3735 */
	}
    }
  /* If we aren't dir_finding, do a find-first, otherwise do a find-next. */
  else if dir_find_handle == INVALID_HANDLE_VALUE                             /* C 3738 */
    {
      let mut filename: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 3740 */
      let mut ln: c_int;                                                      /* C 3741 */
      let mut last_slash: bool = true;                                        /* C 3742 */

      /* Note: We don't need to worry about dir_pathname being longer
	 than MAX_UTF8_PATH, as sys_opendir already took care of that
	 when it called map_w32_filename: that function will put a "?"
	 in its return value in that case, thus failing all the calls
	 below.  */
      strcpy (filename.as_mut_ptr (), dir_pathname.as_ptr ());                /* C 3749 */
      ln = strlen (filename.as_ptr ()) as c_int;                              /* C 3750 */
      if !IS_DIRECTORY_SEP! (*filename.as_ptr ().offset (ln as isize - 1) as c_int)  /* C 3751 */
	{
	  last_slash = false;                                                 /* C 3752 */
	}

      /* Note: No need to resolve symlinks in FILENAME, because
	 FindFirst opens the directory that is the target of a
	 symlink.  */
      if w32_unicode_filenames                                                /* C 3758 */
	{
	  let mut fnw: [c_ushort; MAX_PATH as usize + 2] = [0; MAX_PATH as usize + 2];  /* C 3760 */

	  filename_to_utf16 (filename.as_mut_ptr (), fnw.as_mut_ptr ());        /* C 3762 */
	  if !last_slash                                                       /* C 3763 */
	    {
	      wcscat (fnw.as_mut_ptr (), BACKSLASH_W.as_ptr ());               /* C 3764 */
	    }
	  wcscat (fnw.as_mut_ptr (), STAR_W.as_ptr ());                        /* C 3765 */
	  dir_find_handle = FindFirstFileW (fnw.as_mut_ptr (),
					    &raw mut dir_find_data_w);        /* C 3766 */
	}
      else
	{
	  let mut fna: [c_char; MAX_PATH as usize + 2] = [0; MAX_PATH as usize + 2];  /* C 3770 */

	  filename_to_ansi (filename.as_mut_ptr (), fna.as_mut_ptr ());        /* C 3772 */
	  if !last_slash                                                       /* C 3773 */
	    {
	      strcat (fna.as_mut_ptr (), c"\\".as_ptr ());                     /* C 3774 */
	    }
	  strcat (fna.as_mut_ptr (), c"*".as_ptr ());                          /* C 3775 */
	  /* If FILENAME is not representable by the current ANSI
	     codepage, we don't want FindFirstFileA to interpret the
	     '?' characters as a wildcard.  */
	  if !_mbspbrk (fna.as_ptr () as *const c_uchar,
			c"?".as_ptr () as *const c_uchar).is_null ()          /* C 3780 */
	    {
	      dir_find_handle = INVALID_HANDLE_VALUE;                          /* C 3781 */
	    }
	  else
	    {
	      dir_find_handle = FindFirstFileA (fna.as_mut_ptr (),
						&raw mut dir_find_data_a);    /* C 3783 */
	    }
	}

      if dir_find_handle == INVALID_HANDLE_VALUE                              /* C 3786 */
	{
	  /* Any changes in the value of errno here should be in sync
	     with what directory_files_internal does when it calls
	     readdir.  */
	  match GetLastError ()                                               /* C 3791 */
	    {
	      /* Windows uses this value when FindFirstFile finds no
		 files that match the wildcard.  This is not supposed
		 to happen, since our wildcard is "*", but just in
		 case, if there's some weird empty directory with not
		 even "." and ".." entries...  */
	      ERROR_FILE_NOT_FOUND =>                                         /* C 3799 */
		{ *errno () = 0; }              /* FALLTHROUGH into the empty `default`  C 3800 */
	      ERROR_ACCESS_DENIED | ERROR_NETWORK_ACCESS_DENIED =>            /* C 3804-3805 */
		{ *errno () = EACCES; }
	      ERROR_PATH_NOT_FOUND | ERROR_INVALID_DRIVE | ERROR_NOT_READY
	      | ERROR_BAD_NETPATH | ERROR_BAD_NET_NAME =>                     /* C 3808-3812 */
		{ *errno () = ENOENT; }
	      _ =>
		{ }                                                           /* C 3802 */
	    }
	  return core::ptr::null_mut ();                                      /* C 3814 */
	}
    }
  else if w32_unicode_filenames                                               /* C 3817 */
    {
      if FindNextFileW (dir_find_handle, &raw mut dir_find_data_w) == 0       /* C 3819 */
	{
	  *errno () = 0;                                                      /* C 3821 */
	  return core::ptr::null_mut ();                                      /* C 3822 */
	}
    }
  else
    {
      if FindNextFileA (dir_find_handle, &raw mut dir_find_data_a) == 0       /* C 3828 */
	{
	  *errno () = 0;                                                      /* C 3830 */
	  return core::ptr::null_mut ();                                      /* C 3831 */
	}
    }

  /* Emacs never uses this value, so don't bother making it match
     value returned by stat().  */
  dir_static.d_ino = 1;                                                       /* C 3837 */

  if w32_unicode_filenames                                                    /* C 3839 */
    {
      if downcase != 0 || dir_is_fat != 0                                     /* C 3841 */
	{
	  let mut tem: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 3843 */

	  wcscpy (tem.as_mut_ptr (), dir_find_data_w.cFileName.as_ptr ());     /* C 3845 */
	  CharLowerW (tem.as_mut_ptr ());                                      /* C 3846 */
	  filename_from_utf16 (tem.as_mut_ptr (), dir_static.d_name.as_mut_ptr ());  /* C 3847 */
	}
      else
	{
	  filename_from_utf16 (dir_find_data_w.cFileName.as_mut_ptr (),
			       dir_static.d_name.as_mut_ptr ());              /* C 3850 */
	}
      last_dir_find_data = DIR_FIND_DATA_W;                                   /* C 3851 */
    }
  else
    {
      let mut tem: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];      /* C 3855 */

      /* If the file name in cFileName[] includes `?' characters, it
	 means the original file name used characters that cannot be
	 represented by the current ANSI codepage.  To avoid total
	 lossage, retrieve the short 8+3 alias of the long file
	 name.  */
      if !_mbspbrk (dir_find_data_a.cFileName.as_ptr () as *const c_uchar,
		    c"?".as_ptr () as *const c_uchar).is_null ()               /* C 3863 */
	{
	  strcpy (tem.as_mut_ptr (), dir_find_data_a.cAlternateFileName.as_ptr ());  /* C 3865 */
	  /* 8+3 aliases are returned in all caps, which could break
	     various alists that look at filenames' extensions.  */
	  downcase = 1;                                                       /* C 3868 */
	}
      else if downcase != 0 || dir_is_fat != 0                                /* C 3870 */
	{
	  strcpy (tem.as_mut_ptr (), dir_find_data_a.cFileName.as_ptr ());    /* C 3871 */
	}
      else
	{
	  filename_from_ansi (dir_find_data_a.cFileName.as_mut_ptr (),
			      dir_static.d_name.as_mut_ptr ());               /* C 3873 */
	}
      if downcase != 0 || dir_is_fat != 0                                     /* C 3874 */
	{
	  _mbslwr (tem.as_mut_ptr () as *mut c_uchar);                        /* C 3876 */
	  filename_from_ansi (tem.as_mut_ptr (), dir_static.d_name.as_mut_ptr ());  /* C 3877 */
	}
      last_dir_find_data = DIR_FIND_DATA_A;                                   /* C 3879 */
    }

  dir_static.d_namlen = strlen (dir_static.d_name.as_ptr ()) as c_ushort;     /* C 3881 */
  dir_static.d_reclen = (core::mem::size_of::<dirent> ()                      /* C 3882-3883 */
			 .wrapping_sub (MAX_UTF8_PATH as usize)
			 .wrapping_add (3)
			 .wrapping_add (dir_static.d_namlen as usize)
			 .wrapping_sub ((dir_static.d_namlen as usize) % 4)) as c_ushort;

  return &raw mut dir_static;                                                 /* C 3885 */
}

/* C 3887-3938: `open_unc_volume` - "Support shares on a network resource as subdirectories of a
   read-only root directory."  C's uninitialized `DWORD result` and `HANDLE henum` are zeroed
   (PORTING.md 1.5), the `NETRESOURCEW`/`NETRESOURCEA` records are the section above's, and C's
   `result == NO_ERROR` test is the SDK's own error code.  */
#[cfg (feature = "windowsnt")]
unsafe fn open_unc_volume (path: *const c_char) -> HANDLE
{
  let fn_: *const c_char = map_w32_filename (path, core::ptr::null_mut ());   /* C 3890 */
  let mut result: DWORD = 0;                                                  /* C 3891 */
  let mut henum: HANDLE = core::ptr::null_mut ();                             /* C 3892 */

  if w32_unicode_filenames                                                    /* C 3894 */
    {
      let mut nrw: NETRESOURCEW = core::mem::zeroed ();                       /* C 3896 */
      let mut fnw: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];    /* C 3897 */

      nrw.dwScope = RESOURCE_GLOBALNET;                                       /* C 3899 */
      nrw.dwType = RESOURCETYPE_DISK;                                         /* C 3900 */
      nrw.dwDisplayType = RESOURCEDISPLAYTYPE_SERVER;                         /* C 3901 */
      nrw.dwUsage = RESOURCEUSAGE_CONTAINER;                                  /* C 3902 */
      nrw.lpLocalName = core::ptr::null_mut ();                               /* C 3903 */
      filename_to_utf16 (fn_, fnw.as_mut_ptr ());                             /* C 3904 */
      nrw.lpRemoteName = fnw.as_mut_ptr ();                                   /* C 3905 */
      nrw.lpComment = core::ptr::null_mut ();                                 /* C 3906 */
      nrw.lpProvider = core::ptr::null_mut ();                                /* C 3907 */

      result = WNetOpenEnumW (RESOURCE_GLOBALNET, RESOURCETYPE_DISK,
			      RESOURCEUSAGE_CONNECTABLE, &raw mut nrw,
			      &raw mut henum);                                    /* C 3909-3910 */
    }
  else
    {
      let mut nra: NETRESOURCEA = core::mem::zeroed ();                       /* C 3914 */
      let mut fna: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];      /* C 3915 */

      nra.dwScope = RESOURCE_GLOBALNET;                                       /* C 3917 */
      nra.dwType = RESOURCETYPE_DISK;                                         /* C 3918 */
      nra.dwDisplayType = RESOURCEDISPLAYTYPE_SERVER;                         /* C 3919 */
      nra.dwUsage = RESOURCEUSAGE_CONTAINER;                                  /* C 3920 */
      nra.lpLocalName = core::ptr::null_mut ();                               /* C 3921 */
      filename_to_ansi (fn_, fna.as_mut_ptr ());                              /* C 3922 */
      nra.lpRemoteName = fna.as_mut_ptr ();                                   /* C 3923 */
      nra.lpComment = core::ptr::null_mut ();                                 /* C 3924 */
      nra.lpProvider = core::ptr::null_mut ();                                /* C 3925 */

      result = WNetOpenEnumA (RESOURCE_GLOBALNET, RESOURCETYPE_DISK,
			      RESOURCEUSAGE_CONNECTABLE, &raw mut nra,
			      &raw mut henum);                                    /* C 3927-3928 */
    }
  if result == NO_ERROR                                                       /* C 3930 */
    {
      return henum;                                                           /* C 3931 */
    }
  else
    {
      /* Make sure directory_files_internal reports a sensible error.  */
      *errno () = ENOENT;                                                     /* C 3935 */
      return INVALID_HANDLE_VALUE;                                            /* C 3936 */
    }
}

/* C 3940-3991: `read_unc_volume` - C's `bufsize = 512` (doubled on the wide side), the
   `alloca`-style buffer (P-05's `sa_stack_alloc`), the `((LPNETRESOURCEW) buffer)->lpRemoteName`
   cast and the `CharNextExA` walk on the ANSI side.  C's `int result` holds a `DWORD`.  */
#[cfg (feature = "windowsnt")]
unsafe fn read_unc_volume (henum: HANDLE, fname_w: *mut c_ushort, fname_a: *mut c_char,
			   size: c_int) -> *mut c_void
{
  let mut count: DWORD;                                                       /* C 3943 */
  let mut result: c_int;                                                      /* C 3944 */
  let mut buffer: *mut c_char;                                                /* C 3945 */
  let mut bufsize: DWORD = 512;                                               /* C 3946 */
  let mut retval: *mut c_void;                                                /* C 3947 */

  count = 1;                                                                  /* C 3949 */
  if w32_unicode_filenames                                                    /* C 3950 */
    {
      let mut ptrw: *mut c_ushort;                                            /* C 3952 */

      bufsize *= 2;                                                         /* C 3954 */
      buffer = sa_stack_alloc (bufsize as usize) as *mut c_char;              /* C 3955 */
      result = WNetEnumResourceW (henum, &raw mut count, buffer as LPVOID,
				  &raw mut bufsize) as c_int;                 /* C 3956 */
      if result != NO_ERROR as c_int                                          /* C 3957 */
	{
	  return core::ptr::null_mut ();                                      /* C 3958 */
	}
      /* WNetEnumResource returns \\resource\share...skip forward to "share". */
      ptrw = (*(buffer as LPNETRESOURCEW)).lpRemoteName;                      /* C 3961 */
      ptrw = ptrw.offset (2);                                                 /* C 3962 */
      while *ptrw != 0 && *ptrw != b'/' as c_ushort && *ptrw != b'\\' as c_ushort  /* C 3963 */
	{
	  ptrw = ptrw.offset (1);
	}
      ptrw = ptrw.offset (1);                                                 /* C 3964 */
      wcsncpy (fname_w, ptrw, size as usize);                                 /* C 3965 */
      retval = fname_w as *mut c_void;                                        /* C 3966 */
    }
  else
    {
      let dbcs_p: c_int = (max_filename_mbslen () > 1) as c_int;              /* C 3970 */
      let mut ptra: *mut c_char;                                              /* C 3971 */

      buffer = sa_stack_alloc (bufsize as usize) as *mut c_char;              /* C 3973 */
      result = WNetEnumResourceA (henum, &raw mut count, buffer as LPVOID,
				  &raw mut bufsize) as c_int;                 /* C 3974 */
      if result != NO_ERROR as c_int                                          /* C 3975 */
	{
	  return core::ptr::null_mut ();                                      /* C 3976 */
	}
      ptra = (*(buffer as LPNETRESOURCEA)).lpRemoteName;                      /* C 3978 */
      ptra = ptra.offset (2);                                                 /* C 3979 */
      if dbcs_p == 0                                                          /* C 3980 */
	{
	  while *ptra != 0 && !IS_DIRECTORY_SEP! (*ptra as c_int)             /* C 3981 */
	    {
	      ptra = ptra.offset (1);
	    }
	}
      else
	{
	  while *ptra != 0 && !IS_DIRECTORY_SEP! (*ptra as c_int)             /* C 3984 */
	    {
	      ptra = CharNextExA (file_name_codepage as WORD, ptra, 0);       /* C 3985 */
	    }
	}
      ptra = ptra.offset (1);                                                 /* C 3987 */
      strncpy (fname_a, ptra, size as usize);                                 /* C 3988 */
      retval = fname_a as *mut c_void;                                        /* C 3989 */
    }

  return retval;                                                              /* C 3991 */
}

/* C 3993-3998: `close_unc_volume`.  */
#[cfg (feature = "windowsnt")]
unsafe fn close_unc_volume (henum: HANDLE)
{
  if henum != INVALID_HANDLE_VALUE                                            /* C 3996 */
    {
      WNetCloseEnum (henum);                                                  /* C 3997 */
    }
}

/* C 4000-4015: `unc_volume_file_attributes` - C's uninitialized `HANDLE henum`/`DWORD attrs`
   zeroed (PORTING.md 1.5), and `-1` written as the `DWORD` C's return type makes it.  */
#[cfg (feature = "windowsnt")]
unsafe fn unc_volume_file_attributes (path: *const c_char) -> DWORD
{
  let mut henum: HANDLE = core::ptr::null_mut ();                             /* C 4004 */
  let mut attrs: DWORD = 0;                                                   /* C 4005 */

  henum = open_unc_volume (path);                                             /* C 4007 */
  if henum == INVALID_HANDLE_VALUE                                            /* C 4008 */
    {
      return (-1 as c_int) as DWORD;                                          /* C 4009 */
    }

  attrs = FILE_ATTRIBUTE_READONLY | FILE_ATTRIBUTE_DIRECTORY;                 /* C 4011 */

  close_unc_volume (henum);                                                   /* C 4013 */

  return attrs;                                                               /* C 4015 */
}

/* C 4017-4053: `logon_network_drive` - "Ensure a network connection is authenticated."  C's three
   `drvtype` arms (`DRIVE_REMOTE` for a UNC path, `GetDriveType (NULL)` for a bare name and the
   three-character `drive` buffer otherwise), its `n_slashes` truncation loop and the sixteen-case
   `switch (val)` are kept; C's `drive[3] = '\0'` is the port's null terminator in the array.  */
#[cfg (feature = "windowsnt")]
unsafe fn logon_network_drive (path: *const c_char)
{
  let mut share: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 4021 */
  let mut n_slashes: c_int;                                                   /* C 4022 */
  let mut drive: [c_char; 4] = [0; 4];                                        /* C 4023 */
  let mut drvtype: UINT;                                                      /* C 4024 */
  let mut p: *mut c_char;                                                     /* C 4025 */
  let mut val: DWORD;                                                         /* C 4026 */

  if IS_DIRECTORY_SEP! (*path as c_int)                                       /* C 4028 */
      && IS_DIRECTORY_SEP! (*path.offset (1) as c_int)
    {
      drvtype = DRIVE_REMOTE;                                                 /* C 4029 */
    }
  else if *path == 0 || *path.offset (1) != b':' as c_char                    /* C 4030 */
    {
      drvtype = GetDriveType (core::ptr::null ());                            /* C 4031 */
    }
  else
    {
      drive[0] = *path;                                                       /* C 4034 */
      drive[1] = b':' as c_char;                                              /* C 4035 */
      drive[2] = b'\\' as c_char;                                             /* C 4036 */
      drive[3] = 0;                                                           /* C 4037 */
      drvtype = GetDriveType (drive.as_ptr ());                               /* C 4038 */
    }

  /* Only logon to networked drives.  */
  if drvtype != DRIVE_REMOTE                                                 /* C 4042 */
    {
      return;                                                                 /* C 4043 */
    }

  n_slashes = 2;                                                              /* C 4045 */
  strncpy (share.as_mut_ptr (), path, MAX_UTF8_PATH as usize - 1);            /* C 4046 */
  /* Truncate to just server and share name.  */
  p = share.as_mut_ptr ().offset (2);                                         /* C 4048 */
  while *p != 0 && (p as usize) < (share.as_mut_ptr () as usize
				   + MAX_UTF8_PATH as usize)                  /* C 4048 */
    {
      if IS_DIRECTORY_SEP! (*p as c_int)                                      /* C 4050 */
	{
	  n_slashes += 1;
	  if n_slashes > 3
	    {
	      *p = 0;                                                         /* C 4052 */
	      break;                                                          /* C 4053 */
	    }
	}
      p = p.offset (1);                                                       /* C 4048 (the for's increment) */
    }

  if w32_unicode_filenames                                                    /* C 4057 */
    {
      let mut resourcew: NETRESOURCEW = core::mem::zeroed ();                 /* C 4059 */
      let mut share_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 4060 */

      resourcew.dwScope = RESOURCE_GLOBALNET;                                 /* C 4062 */
      resourcew.dwType = RESOURCETYPE_DISK;                                   /* C 4063 */
      resourcew.dwDisplayType = RESOURCEDISPLAYTYPE_SHARE;                    /* C 4064 */
      resourcew.dwUsage = RESOURCEUSAGE_CONTAINER;                            /* C 4065 */
      resourcew.lpLocalName = core::ptr::null_mut ();                         /* C 4066 */
      filename_to_utf16 (share.as_mut_ptr (), share_w.as_mut_ptr ());         /* C 4067 */
      resourcew.lpRemoteName = share_w.as_mut_ptr ();                         /* C 4068 */
      resourcew.lpProvider = core::ptr::null_mut ();                          /* C 4069 */

      val = WNetAddConnection2W (&raw mut resourcew, core::ptr::null (),
				 core::ptr::null (),
				 CONNECT_INTERACTIVE);                        /* C 4071 */
    }
  else
    {
      let mut resourcea: NETRESOURCEA = core::mem::zeroed ();                 /* C 4075 */
      let mut share_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 4076 */

      resourcea.dwScope = RESOURCE_GLOBALNET;                                 /* C 4078 */
      resourcea.dwType = RESOURCETYPE_DISK;                                   /* C 4079 */
      resourcea.dwDisplayType = RESOURCEDISPLAYTYPE_SHARE;                    /* C 4080 */
      resourcea.dwUsage = RESOURCEUSAGE_CONTAINER;                            /* C 4081 */
      resourcea.lpLocalName = core::ptr::null_mut ();                         /* C 4082 */
      filename_to_ansi (share.as_mut_ptr (), share_a.as_mut_ptr ());          /* C 4083 */
      resourcea.lpRemoteName = share_a.as_mut_ptr ();                         /* C 4084 */
      resourcea.lpProvider = core::ptr::null_mut ();                          /* C 4085 */

      val = WNetAddConnection2A (&raw mut resourcea, core::ptr::null (),
				 core::ptr::null (),
				 CONNECT_INTERACTIVE);                        /* C 4087 */
    }

  match val                                                                    /* C 4090 */
    {
      NO_ERROR | ERROR_ALREADY_ASSIGNED =>                                     /* C 4092-4094 */
	{ }
      ERROR_ACCESS_DENIED | ERROR_LOGON_FAILURE =>                             /* C 4095-4097 */
	{ *errno () = EACCES; }
      ERROR_BUSY =>                                                            /* C 4098-4099 */
	{ *errno () = EAGAIN; }
      ERROR_BAD_NET_NAME | ERROR_NO_NET_OR_BAD_PATH | ERROR_NO_NETWORK
      | ERROR_CANCELLED =>                                                     /* C 4100-4103 */
	{ *errno () = ENOENT; }
      _ =>
	{ *errno () = ENOENT; }                                               /* C 4104-4106 (default) */
    }
}

/* C 4108-4173: `access_attrs` - "Subroutine of faccessat.  Determines attributes of FILE (which
   is assumed to be in UTF-8 and after map_w32_filename) as reported by GetFileAttributes.
   Returns -1 if it fails…".  C's `if (attrs == -1)` and the nested `switch (w32err)` keep C's
   `FALLTHROUGH` into the `ENOENT` group (`ERROR_INVALID_NAME`/`ERROR_BAD_PATHNAME` fall through
   when the UNC branch does not return), which the port spells by repeating that group - PORTING.md
   2.2 - and `is_unc_volume`/`unc_volume_file_attributes` are this file's own definitions.  */
#[cfg (feature = "windowsnt")]
unsafe fn access_attrs (file: *const c_char) -> DWORD
{
  let mut attrs: DWORD;                                                       /* C 4120 */

  if w32_unicode_filenames                                                    /* C 4122 */
    {
      let mut file_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];  /* C 4124 */

      filename_to_utf16 (file, file_w.as_mut_ptr ());                         /* C 4126 */
      attrs = GetFileAttributesW (file_w.as_mut_ptr ());                      /* C 4127 */
    }
  else
    {
      let mut file_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];    /* C 4131 */

      filename_to_ansi (file, file_a.as_mut_ptr ());                          /* C 4133 */
      attrs = GetFileAttributesA (file_a.as_mut_ptr ());                      /* C 4134 */
    }

  if attrs == (-1 as c_int) as DWORD                                          /* C 4137 */
    {
      let w32err: DWORD = GetLastError ();                                    /* C 4139 */

      match w32err                                                           /* C 4141 */
	{
	  ERROR_INVALID_NAME | ERROR_BAD_PATHNAME =>                          /* C 4143-4144 */
	    {
	      if is_unc_volume (file) != 0                                    /* C 4145 */
		{
		  attrs = unc_volume_file_attributes (file);                  /* C 4147 */
		  if attrs == (-1 as c_int) as DWORD                          /* C 4148 */
		    {
		      *errno () = EACCES;                                     /* C 4150 */
		      return (-1 as c_int) as DWORD;                          /* C 4151 */
		    }
		  return attrs;                                               /* C 4153 */
		}
	      /* FALLTHROUGH */
	      *errno () = ENOENT;                                             /* C 4159 */
	    }
	  ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND | ERROR_INVALID_DRIVE
	  | ERROR_NOT_READY | ERROR_BAD_NETPATH | ERROR_BAD_NET_NAME =>       /* C 4158-4164 */
	    { *errno () = ENOENT; }
	  _ =>
	    { *errno () = EACCES; }                                           /* C 4165-4167 */
	}
      return (-1 as c_int) as DWORD;                                          /* C 4169 */
    }

  return attrs;                                                               /* C 4172 */
}
