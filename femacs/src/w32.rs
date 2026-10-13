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
/// C: typedef struct _PROCESS_MEMORY_COUNTERS { DWORD cb; DWORD PageFaultCount;
/// C:   SIZE_T PeakWorkingSetSize; SIZE_T WorkingSetSize; SIZE_T QuotaPeakPagedPoolUsage;
/// C:   SIZE_T QuotaPagedPoolUsage; SIZE_T QuotaPeakNonPagedPoolUsage;
/// C:   SIZE_T QuotaNonPagedPoolUsage; SIZE_T PagefileUsage; SIZE_T PeakPagefileUsage; }
/// C:   PROCESS_MEMORY_COUNTERS, *PPROCESS_MEMORY_COUNTERS;
/// C: typedef struct tagPROCESSENTRY32 { DWORD dwSize; DWORD cntUsage; DWORD th32ProcessID;
/// C:   ULONG_PTR th32DefaultHeapID; DWORD th32ModuleID; DWORD cntThreads;
/// C:   DWORD th32ParentProcessID; LONG pcPriClassBase; DWORD dwFlags;
/// C:   CHAR szExeFile[MAX_PATH]; } PROCESSENTRY32, *LPPROCESSENTRY32;
/// C: typedef struct _IP_ADAPTER_INFO { … } IP_ADAPTER_INFO, *PIP_ADAPTER_INFO;
/// C: typedef struct _IP_ADAPTER_ADDRESSES_LH { … } IP_ADAPTER_ADDRESSES, *PIP_ADAPTER_ADDRESSES;
/// C: typedef enum _SE_OBJECT_TYPE { SE_UNKNOWN_OBJECT_TYPE = 0, … } SE_OBJECT_TYPE;
/// C: typedef enum _SID_NAME_USE { SidTypeUser = 1, … } SID_NAME_USE, *PSID_NAME_USE;
/// C: typedef enum _TOKEN_INFORMATION_CLASS { TokenUser = 1, … } TOKEN_INFORMATION_CLASS;
/// C: typedef enum _SECURITY_IMPERSONATION_LEVEL { SecurityAnonymous, … } SECURITY_IMPERSONATION_LEVEL;
/* Slice 19's `system_process_attributes` holds a `PROCESS_MEMORY_COUNTERS` *by value* (C 7453)
   and reads its `PageFaultCount`/`WorkingSetSize`, so - as with `PROCESSENTRY32` and
   `MEMORYSTATUS` - the record is spelled here with C's member names and order (P-88,
   `UNVERIFIED`) and the alias names it as C's does.  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MEMORY_COUNTERS
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
}
#[cfg (feature = "windowsnt")]
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
/* Slice 18's `list_system_processes` (C 7296-7305) reads `dwSize` and `th32ProcessID` of a
   `PROCESSENTRY32` *by value*, so the record itself - until then carried as the pointer alias the
   P-72 ③ rule allows - is spelled here with C's member names and order (P-88, `UNVERIFIED`: the
   SDK's `<tlhelp32.h>` is not in the reference tree), and the alias now names it as C's does.  */
/// `ULONG_PTR` is pointer-sized; `w32term.rs` 175 spells it `usize` (PORTING.md 1.1's handle
/// rule), and this record's fourth member is the unit's first use of it.  C: `typedef ULONG_PTR
/// unsigned long;` is how `w32term.rs` quotes the SDK there.
#[cfg (feature = "windowsnt")]
pub type ULONG_PTR = usize;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSENTRY32
{
  pub dwSize: DWORD,
  pub cntUsage: DWORD,
  pub th32ProcessID: DWORD,
  pub th32DefaultHeapID: ULONG_PTR,
  pub th32ModuleID: DWORD,
  pub cntThreads: DWORD,
  pub th32ParentProcessID: DWORD,
  pub pcPriClassBase: LONG,
  pub dwFlags: DWORD,
  pub szExeFile: [c_char; MAX_PATH as usize],
}
#[cfg (feature = "windowsnt")]
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;
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
/// C: typedef struct _MEMORYSTATUS { DWORD dwLength; DWORD dwMemoryLoad; SIZE_T dwTotalPhys;
/// C:   SIZE_T dwAvailPhys; SIZE_T dwTotalPageFile; SIZE_T dwAvailPageFile; SIZE_T dwTotalVirtual;
/// C:   SIZE_T dwAvailVirtual; } MEMORYSTATUS, *LPMEMORYSTATUS;
/* Slice 19's `system_process_attributes` reads `dwTotalPhys` of a `MEMORYSTATUS` *by value*
   (C 7656/7661), so - as with `PROCESSENTRY32` in slice 18 - the record itself is spelled here
   (C's member names and order, P-88, `UNVERIFIED`) and the alias names it as C's does.  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORYSTATUS
{
  pub dwLength: DWORD,
  pub dwMemoryLoad: DWORD,
  pub dwTotalPhys: SIZE_T,
  pub dwAvailPhys: SIZE_T,
  pub dwTotalPageFile: SIZE_T,
  pub dwAvailPageFile: SIZE_T,
  pub dwTotalVirtual: SIZE_T,
  pub dwAvailVirtual: SIZE_T,
}
#[cfg (feature = "windowsnt")]
pub type LPMEMORYSTATUS = *mut MEMORYSTATUS;
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

/* C 140-159's typedef is the *older-SDK* arm; the `#else` is `<psapi.h>`'s own record, which
   C 7454's `PROCESS_MEMORY_COUNTERS_EX mem_ex` reads by value (C 7667/7673/7682 use
   `WorkingSetSize` and `PrivateUsage`).  The port spells that arm here, under C's own condition's
   complement so the two copies never coexist (PORTING.md's one-definition-per-arm rule).  */
#[cfg (all (feature = "windowsnt",
           not (all (feature = "win32_winnt_lt_0500", feature = "mingw32_w32api_lt_3_15"))))]
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
#[cfg (all (feature = "windowsnt",
           not (all (feature = "win32_winnt_lt_0500", feature = "mingw32_w32api_lt_3_15"))))]
pub type PROCESS_MEMORY_COUNTERS_EX = _PROCESS_MEMORY_COUNTERS_EX;
#[cfg (all (feature = "windowsnt",
           not (all (feature = "win32_winnt_lt_0500", feature = "mingw32_w32api_lt_3_15"))))]
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

/* C 2862: `#define REG_ROOT` - C's string macro is the port's same-named `&CStr` constant
   (PORTING.md 2.1; an external registry path, allowlisted), passed to `w32_get_resource` below.  */
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
/* C 4174-4176: C 4174's blank, C 4175's `/* Emulate faccessat(2).  */` and C 4176's `int`.  */
/* C 4177-4246: `faccessat`.  The `<winnt.h>` reparse point pair C 4218/4219 first names are
   declared here (the P-88 device this lane uses for the SDK's own macros; no Windows SDK or
   mingw toolchain is on this machine, so the values carry the ledger's `UNVERIFIED` marking),
   and C 4223/4228 call `chase_symlinks`, whose C 808 prototype the port has no item for yet -
   the definition is past this slice, the `ltime` class the ledger's Deviations 4 records.  */
#[cfg (feature = "windowsnt")]
/// C 4191-4204/4248-4253: `IS_DEVICE_SEP` - the same generated `<config.h>` family `IS_DIRECTORY_SEP`
/// comes from (`configure.ac` 6494's Windows branch, `((_c_) == ':')`; the frozen `build/emacs-ref/src/config.h`
/// 2094 carries the Darwin branch's `0`).
/// C: configure.ac 6494: `#define IS_DEVICE_SEP(_c_) ((_c_) == ':')`
/// C: build/emacs-ref/src/config.h 2094: `#define IS_DEVICE_SEP(_c_) 0` (the frozen target)
macro_rules! IS_DEVICE_SEP {
  ($c:expr) => { (($c) == b':' as c_int) };
}
#[cfg (feature = "windowsnt")]

#[cfg (feature = "windowsnt")]
/// C: #define FILE_ATTRIBUTE_REPARSE_POINT 0x00000400
pub const FILE_ATTRIBUTE_REPARSE_POINT: DWORD = 0x00000400;
#[cfg (feature = "windowsnt")]
/// C: #define FILE_SUPPORTS_REPARSE_POINTS 0x00040000 (`<winnt.h>`'s volume flag)
pub const FILE_SUPPORTS_REPARSE_POINTS: DWORD = 0x00040000;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faccessat (dirfd: c_int, path: *const c_char, mode: c_int, // C 4177
				    flags: c_int)
  -> c_int
{
  let mut attributes: DWORD;                                            // C 4179
  let mut fullname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 4180

  /* C leaves `attributes` uninitialised (PORTING.md 1.5); C 4180's array starts zeroed.  */
  attributes = 0;

  /* Rely on a hack: an open directory is modeled as file descriptor 0, // C 4182
     and its actual file name is stored in dir_pathname by opendir.     // C 4183
     This is good enough for the current usage in Emacs, but is fragile.  */ // C 4184
  if dirfd != AT_FDCWD                                                  // C 4185
      && ! (IS_DIRECTORY_SEP! (*path as c_int)                          // C 4186
	   || IS_DEVICE_SEP! (*path.offset (1) as c_int))                      // C 4187
    {
      let mut lastc: c_char = *dir_pathname.offset ((strlen (dir_pathname) - 1) as isize); // C 4189

      if (_snprintf ((&raw mut fullname) as *mut c_char,                 // C 4191
		   core::mem::size_of_val (&fullname),
		   c"%s%s%s".as_ptr (),                                               // C 4191
		   dir_pathname,                                                      // C 4192
		   if IS_DIRECTORY_SEP! (lastc as c_int) { c"".as_ptr () } else { c"/".as_ptr () }, // C 4192
		   path)                                                              // C 4192
	  < 0)                                                                 // C 4193
	{
	  *errno () = ENAMETOOLONG;                                            // C 4195
	  return -1;                                                           // C 4196
	}
      path = (&raw mut fullname) as *const c_char;                      // C 4198 (C's array decay)
    }

  /* When dired.c calls us with F_OK and a trailing slash, it actually  // C 4201
     wants to know whether PATH is a directory.  */                     // C 4202
  if IS_DIRECTORY_SEP! (*path.offset ((strlen (path) - 1) as isize) as c_int) && mode == F_OK // C 4203
    { mode |= D_OK; }                                                   // C 4204

  /* MSVCRT implementation of 'access' doesn't recognize D_OK, and its  // C 4206
     newer versions blow up when passed D_OK.  */                       // C 4207
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4208

  attributes = access_attrs (path);                                     // C 4210
  if attributes == -1 as DWORD                                          // C 4211 (C's `-1` against the DWORD)
    { return -1; } /* PATH doesn't exist or is inaccessible */

  /* If the last element of PATH is a symlink, we need to resolve it    // C 4214
     to get the attributes of its target file.  Note: any symlinks in   // C 4215
     PATH elements other than the last one are transparently resolved   // C 4216
     by GetFileAttributes below.  */                                    // C 4217
  let not_a_symlink: c_int = ((attributes & FILE_ATTRIBUTE_REPARSE_POINT) == 0) as c_int; // C 4218
  if (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) != 0            // C 4219
      && (flags & AT_SYMLINK_NOFOLLOW) == 0                             // C 4220
      && not_a_symlink == 0                                             // C 4221 (C's `!not_a_symlink`)
    {
      path = chase_symlinks (path);                                     // C 4223
      attributes = access_attrs (path);                                 // C 4224
      if attributes == -1 as DWORD                                      // C 4225
	{ return -1; }                                                         // C 4226
    }

  if ((mode & X_OK) != 0                                                 // C 4229
      && ! (is_exec (path) != 0 || (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0)) // C 4230
    {
      *errno () = EACCES;                                               // C 4232
      return -1;                                                        // C 4233
    }
  if ((mode & W_OK) != 0 && (attributes & FILE_ATTRIBUTE_READONLY) != 0)  // C 4235
    {
      *errno () = EACCES;                                               // C 4237
      return -1;                                                        // C 4238
    }
  if ((mode & D_OK) != 0 && (attributes & FILE_ATTRIBUTE_DIRECTORY) == 0) // C 4240
    {
      *errno () = EACCES;                                               // C 4242
      return -1;                                                        // C 4243
    }
  return 0;                                                             // C 4245
}
/* C 4248-4339: C 4248-4253's comment, C 4254's `int` and `w32_accessible_directory_p`.  The
   three `<winerror.h>` names and `<errno.h>`'s `ENODEV` this slice first names are declared
   here (the P-88 device; the SDK values carry the ledger's `UNVERIFIED` marking).  */
#[cfg (feature = "windowsnt")]
/// C: #define ERROR_NO_MORE_FILES 18L
pub const ERROR_NO_MORE_FILES: DWORD = 18;
#[cfg (feature = "windowsnt")]
/// C: #define ERROR_NO_MORE_ITEMS 259L
pub const ERROR_NO_MORE_ITEMS: DWORD = 259;
#[cfg (feature = "windowsnt")]
/// <errno.h>: `#define ENODEV 19` (`lib/include/errno.rs` does not carry it).
pub const ENODEV: c_int = 19;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_accessible_directory_p (dirname: *const c_char, dirlen: ptrdiff_t) // C 4254
  -> c_int
{
  let mut pattern: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 4257
  let last_slash: bool = dirlen > 0                                     // C 4258
	&& IS_DIRECTORY_SEP! (*dirname.offset ((dirlen - 1) as isize) as c_int);
  let mut dh: HANDLE;                                                   // C 4259

  /* C leaves `dh` uninitialised (PORTING.md 1.5) and zeroes nothing: C 4257's array is
     written by C 4280's `strcpy` before any read.  */
  dh = std::ptr::null_mut ();

  /* Network volumes need a different reading method.  */               // C 4261
  if is_unc_volume (dirname) != 0                                       // C 4262 (C's `is_unc_volume (…)` test)
    {
      let mut read_result: *mut c_void = std::ptr::null_mut ();         // C 4264
      let mut fnw: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4265
      let mut fna: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4266

      dh = open_unc_volume (dirname);                                   // C 4268
      if dh != INVALID_HANDLE_VALUE                                     // C 4269
	{
	  read_result = read_unc_volume (dh, fnw.as_mut_ptr (), fna.as_mut_ptr (), // C 4271
				  MAX_PATH);
	  close_unc_volume (dh);                                               // C 4272
	}
      /* Treat empty volumes as accessible.  */                         // C 4274
      return (read_result != std::ptr::null_mut ()                      // C 4275
	      || GetLastError () == ERROR_NO_MORE_ITEMS) as c_int;
    }

  /* Note: map_w32_filename makes sure DIRNAME is not longer than       // C 4278
     MAX_UTF8_PATH.  */                                                 // C 4279
  strcpy ((&raw mut pattern) as *mut c_char, map_w32_filename (dirname, std::ptr::null_mut ())); // C 4280

  /* Note: No need to resolve symlinks in FILENAME, because FindFirst   // C 4282
     opens the directory that is the target of a symlink.  */           // C 4283
  if w32_unicode_filenames                                              // C 4284
    {
      let mut pat_w: [c_ushort; MAX_PATH as usize + 2] = [0; MAX_PATH as usize + 2]; // C 4286
      let mut dfd_w: WIN32_FIND_DATAW = std::mem::zeroed ();            // C 4287

      filename_to_utf16 ((&raw const pattern) as *const c_char, (&raw mut pat_w) as *mut c_ushort); // C 4289
      if ! last_slash                                                   // C 4290
	{ wcscat (pat_w.as_mut_ptr (), BACKSLASH_W.as_ptr ()); }               // C 4291 (C's `L"\\"`)
      wcscat (pat_w.as_mut_ptr (), STAR_W.as_ptr ());                   // C 4292 (C's `L"*"`)
      dh = FindFirstFileW ((&raw const pat_w) as LPCWSTR, (&raw mut dfd_w)); // C 4293
    }
  else                                                                  // C 4295
    {
      let mut pat_a: [c_char; MAX_PATH as usize + 2] = [0; MAX_PATH as usize + 2]; // C 4297
      let mut dfd_a: WIN32_FIND_DATAA = std::mem::zeroed ();            // C 4298

      filename_to_ansi ((&raw const pattern) as *const c_char, (&raw mut pat_a) as *mut c_char); // C 4300
      if ! last_slash                                                   // C 4301
	{ strcpy ((&raw mut pat_a) as *mut c_char, c"\\".as_ptr ()); }      // C 4302
      strcat ((&raw mut pat_a) as *mut c_char, c"*".as_ptr ());         // C 4303
      /* In case DIRNAME cannot be expressed in characters from the     // C 4304
	 current ANSI codepage.  */                                            // C 4305
      if ! _mbspbrk ((&raw const pat_a) as *const c_uchar, c"?".as_ptr () as *const c_uchar).is_null () // C 4306
	{
	  *errno () = ENOENT;                                                  // C 4308
	  return 0;                                                            // C 4309
	}
      dh = FindFirstFileA ((&raw const pat_a) as LPCSTR, (&raw mut dfd_a)); // C 4311
    }

  if dh == INVALID_HANDLE_VALUE                                         // C 4314
    {
      let mut w32err: DWORD = GetLastError ();                          // C 4316

      match w32err                                                      // C 4318
	{
	  ERROR_INVALID_NAME                                                   // C 4320
	  | ERROR_BAD_PATHNAME                                                 // C 4321
	  | ERROR_FILE_NOT_FOUND                                               // C 4322
	  | ERROR_PATH_NOT_FOUND                                               // C 4323
	  | ERROR_NO_MORE_FILES                                                // C 4324
	  | ERROR_BAD_NETPATH =>                                               // C 4325
	    { *errno () = ENOENT; }                                            // C 4326
	  ERROR_NOT_READY =>                                                   // C 4328
	    { *errno () = ENODEV; }                                            // C 4329
	  _ =>                                                                 // C 4331
	    { *errno () = EACCES; }                                            // C 4332
	}
    return 0;                                                           // C 4335
    }
  FindClose (dh);                                                       // C 4337
  return 1;                                                             // C 4338
}
/* C 4341-4395: C 4341-4346's comment, C 4347's `static int` and `sys_access`.  C 4359 embeds an
   assignment in its condition, and Rust has no such expression, so the port writes the two
   statements C's one means (the same reading this slice gives C 4191's `_snprintf … < 0`).  */
#[cfg (feature = "windowsnt")]
unsafe fn sys_access (fname: *const c_char, mode: c_int) -> c_int       // C 4347
{
  let mut fname_copy: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4350
  let mut p: *mut c_char;
  let mut attributes: DWORD;                                            // C 4351

  /* C leaves `p` and `attributes` uninitialised (PORTING.md 1.5).  */
  p = std::ptr::null_mut ();
  attributes = 0;

  strcpy ((&raw mut fname_copy) as *mut c_char, fname);                 // C 4353
  /* Do the equivalent of unixtodos_filename.  */                       // C 4354
  p = (&raw mut fname_copy) as *mut c_char;                             // C 4355
  while *p != 0                                                         // C 4355 (C's `for (p = fname_copy; *p; …)`)
    {
      if *p == b'/' as c_char                                           // C 4356
	{ *p = b'\\' as c_char; }                                              // C 4357
      p = CharNext (p);                                                 // C 4355 (the for's increment)
    }

  attributes = GetFileAttributesA ((&raw const fname_copy) as LPCSTR);  // C 4359
  if attributes == -1 as DWORD                                          // C 4359 (C's `== -1`)
    {
      let mut w32err: DWORD = GetLastError ();                          // C 4361

      match w32err                                                      // C 4363
	{
	  ERROR_INVALID_NAME                                                   // C 4365
	  | ERROR_BAD_PATHNAME                                                 // C 4366
	  | ERROR_FILE_NOT_FOUND                                               // C 4367
	  | ERROR_BAD_NETPATH =>                                               // C 4368
	    { *errno () = ENOENT; }                                            // C 4369
	  _ =>                                                                 // C 4371
	    { *errno () = EACCES; }                                            // C 4372
	}
      return -1;                                                        // C 4375
    }
  if ((mode & X_OK) != 0                                                 // C 4377
      && ! (is_exec ((&raw const fname_copy) as *const c_char) != 0     // C 4378
	   || (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0))                   // C 4379
    {
      *errno () = EACCES;                                               // C 4381
      return -1;                                                        // C 4382
    }
  if ((mode & W_OK) != 0 && (attributes & FILE_ATTRIBUTE_READONLY) != 0)  // C 4384
    {
      *errno () = EACCES;                                               // C 4386
      return -1;                                                        // C 4387
    }
  if ((mode & D_OK) != 0 && (attributes & FILE_ATTRIBUTE_DIRECTORY) == 0) // C 4389
    {
      *errno () = EACCES;                                               // C 4391
      return -1;                                                        // C 4392
    }
  return 0;                                                             // C 4394
}

/* C 4396-4399: C 4396's blank and C 4397-4399's comment (the next item is C 4401's
   `sys_rename`, past this slice).  */
/* C 4396-4402: C 4396's blank, C 4397-4400's four-line comment, C 4401's blank and C 4402's
   `int`.  */
/* C 4403-4533: the six CRT shadowing wrappers.  The MSVC CRT names this slice first calls
   (`_chdir`/`_wchdir`, `_chmod`/`_wchmod`, `_creat`/`_wcreat`, `_open`/`_wopen`, `_fdopen`) and
   the three `<fcntl.h>` text/binary macros are declared here (the P-88 device this lane uses;
   no Windows SDK or mingw toolchain is on this machine, so those values carry the ledger's
   `UNVERIFIED` marking).  `O_RDONLY`/`O_WRONLY`/`O_RDWR`/`O_CREAT`/`O_TRUNC` come from the
   unit's own C 35 `#include <fcntl.h>` (`lib/include/fcntl.in.rs`), whose values are the
   frozen target's - the CRT's own numbers differ, which the ledger records.  */
/// C: int _chdir (const char *);
/// C: int _wchdir (const wchar_t *);
/// C: int _chmod (const char *, int);
/// C: int _wchmod (const wchar_t *, int);
/// C: int _creat (const char *, int);
/// C: int _wcreat (const wchar_t *, int);
/// C: int _open (const char *, int, ...);
/// C: int _wopen (const wchar_t *, int, ...);
/// C: FILE *_fdopen (int, const char *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _chdir (path: *const c_char) -> c_int;
  pub fn _wchdir (path: *const c_ushort) -> c_int;
  pub fn _chmod (path: *const c_char, mode: c_int) -> c_int;
  pub fn _wchmod (path: *const c_ushort, mode: c_int) -> c_int;
  pub fn _creat (path: *const c_char, mode: c_int) -> c_int;
  pub fn _wcreat (path: *const c_ushort, mode: c_int) -> c_int;
  pub fn _open (path: *const c_char, oflag: c_int, ...) -> c_int;
  pub fn _wopen (path: *const c_ushort, oflag: c_int, ...) -> c_int;
  pub fn _fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
}
#[cfg (feature = "windowsnt")]
#[cfg (feature = "windowsnt")]
#[cfg (feature = "windowsnt")]
/// C: #define _O_NOINHERIT 0x0080
pub const _O_NOINHERIT: c_int = 0x0080;
/* C 4403-4422: `sys_chdir`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_chdir (path: *const c_char) -> c_int       // C 4403
{
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4405
  if w32_unicode_filenames                                              // C 4406
    {
      let mut newdir_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4408

      if filename_to_utf16 (path, newdir_w.as_mut_ptr ()) == 0          // C 4410
	{ return _wchdir (newdir_w.as_ptr ()); }                               // C 4411
      return -1;                                                        // C 4412
    }
  else                                                                  // C 4414
    {
      let mut newdir_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4416

      if filename_to_ansi (path, newdir_a.as_mut_ptr ()) == 0           // C 4418
	{ return _chdir (newdir_a.as_ptr ()); }                                // C 4419
      return -1;                                                        // C 4420
    }
}
/* C 4424-4441: `static int` and `chmod_worker`.  */
#[cfg (feature = "windowsnt")]
unsafe fn chmod_worker (path: *const c_char, mode: c_int) -> c_int      // C 4424
{
  if w32_unicode_filenames                                              // C 4427
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4429

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 4431
      return _wchmod (path_w.as_ptr (), mode);                          // C 4432
    }
  else                                                                  // C 4434
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4436

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 4438
      return _chmod (path_a.as_ptr (), mode);                           // C 4439
    }
}
/* C 4443-4448: `sys_chmod`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_chmod (path: *const c_char, mode: c_int) -> c_int // C 4443
{
  path = chase_symlinks (map_w32_filename (path, std::ptr::null_mut ())); // C 4446
  return chmod_worker (path, mode);                                     // C 4447
}
/* C 4450-4455: `lchmod`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lchmod (path: *const c_char, mode: mode_t) -> c_int // C 4450
{
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4453
  return chmod_worker (path, mode as c_int);                            // C 4454 (C's `mode_t`-into-`int`)
}
/* C 4457-4475: `sys_creat`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_creat (path: *const c_char, mode: c_int) -> c_int // C 4457
{
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4460
  if w32_unicode_filenames                                              // C 4461
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4463

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 4465
      return _wcreat (path_w.as_ptr (), mode);                          // C 4466
    }
  else                                                                  // C 4468
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4470

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 4472
      return _creat (path_a.as_ptr (), mode);                           // C 4473
    }
}
/* C 4477-4533: `sys_fopen`.  C 4496's `while (*++mode)` steps and tests in one operator, so the
   port writes the step and the test as the loop's first two statements (the `image.rs` 11103
   device); the `mode` parameter is mutated, hence `mut mode`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_fopen (path: *const c_char, mut mode: *const c_char) -> *mut FILE // C 4477
{
  let mut fd: c_int;                                                    // C 4480
  let mut oflag: c_int;                                                 // C 4481
  let mode_save: *const c_char = mode;                                  // C 4482

  /* C leaves `fd` and `oflag` uninitialised (PORTING.md 1.5).  */
  fd = 0;
  oflag = 0;

  /* Force all file handles to be non-inheritable.  This is necessary to // C 4484
     ensure child processes don't unwittingly inherit handles that might // C 4485
     prevent future file access. */                                     // C 4486

  if *mode == b'r' as c_char                                            // C 4488
    { oflag = O_RDONLY; }                                               // C 4489
  else if *mode == b'w' as c_char || *mode == b'a' as c_char            // C 4490
    { oflag = O_WRONLY | O_CREAT | O_TRUNC; }                           // C 4491
  else
    { return std::ptr::null_mut (); }                                   // C 4493

  /* Only do simplistic option parsing. */                              // C 4495
  loop
    {
      mode = mode.offset (1);                                           // C 4496 (C's `*++mode` step)
      if *mode == 0                                                     // C 4496 (C's `*++mode` test)
	{ break; }
      if *mode == b'+' as c_char                                        // C 4497
	{
	  oflag &= ! (O_RDONLY | O_WRONLY);                                    // C 4499
	  oflag |= O_RDWR;                                                     // C 4500
	}
      else if *mode == b'b' as c_char                                   // C 4502
	{
	  oflag &= ! O_TEXT;                                                   // C 4504
	  oflag |= O_BINARY;                                                   // C 4505
	}
      else if *mode == b't' as c_char                                   // C 4507
	{
	  oflag &= ! O_BINARY;                                                 // C 4509
	  oflag |= O_TEXT;                                                     // C 4510
	}
      else { break; }                                                   // C 4512
    }

  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4514
  if w32_unicode_filenames                                              // C 4515
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4517

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 4519
      fd = _wopen (path_w.as_mut_ptr (), oflag | _O_NOINHERIT, 0o644);  // C 4520
    }
  else                                                                  // C 4522
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4524

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 4526
      fd = _open (path_a.as_mut_ptr (), oflag | _O_NOINHERIT, 0o644);   // C 4527
    }
  if fd < 0                                                             // C 4529
    { return std::ptr::null_mut (); }                                   // C 4530

  return _fdopen (fd, mode_save);                                       // C 4532
}

/* C 4534-4536: C 4534's blank, C 4535's `/* This only works on NTFS volumes, but is useful to
   have.  */` and C 4536's `int` (the next item is C 4537's `sys_link`, past this slice).  */
/* C 4534-4536: C 4534's blank, C 4535's `/* This only works on NTFS volumes, but is useful to
   have.  */` and C 4536's `int`.  */
/* C 4537-4652: `sys_link`.  The `<winbase.h>`/`<winnt.h>` records and scalars this slice first
   names are declared here (the P-88 device this lane uses; no Windows SDK or mingw toolchain is
   on this machine, so all of them carry the ledger's `UNVERIFIED` marking), together with C
   4577-4580's anonymous local struct and C 4594's `offsetof`.  */
/// C: typedef struct _WIN32_STREAM_ID { DWORD dwStreamId; DWORD dwStreamAttributes;
/// C:   LARGE_INTEGER Size; DWORD dwStreamNameSize; WCHAR cStreamName[ANYSIZE_ARRAY]; } WIN32_STREAM_ID;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_STREAM_ID
{
  pub dwStreamId: DWORD,
  pub dwStreamAttributes: DWORD,
  pub Size: LARGE_INTEGER,
  pub dwStreamNameSize: DWORD,
  pub cStreamName: [WCHAR; 1],                    /* C's `ANYSIZE_ARRAY` (1) array */
}
/* C 4577-4580: the anonymous local
   `struct { WIN32_STREAM_ID wid; WCHAR wbuffer[MAX_PATH]; } data;` - Rust has no anonymous
   struct, so the port names it (the `LARGE_INTEGER_u` device above).  */
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct w32_sys_link_data
{
  pub wid: WIN32_STREAM_ID,
  pub wbuffer: [WCHAR; MAX_PATH as usize],         /* extra space for link name */
}
/// C: #define BACKUP_LINK 0x00000005 (`<winnt.h>`'s stream-ID list)
#[cfg (feature = "windowsnt")]
pub const BACKUP_LINK: DWORD = 0x00000005;
/// C: #define OPEN_EXISTING 3 (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const OPEN_EXISTING: DWORD = 3;
/// C: #define FILE_FLAG_BACKUP_SEMANTICS 0x02000000 (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const FILE_FLAG_BACKUP_SEMANTICS: DWORD = 0x02000000;
/// C: #define ERROR_NOT_SAME_DEVICE 17L
#[cfg (feature = "windowsnt")]
pub const ERROR_NOT_SAME_DEVICE: DWORD = 17;
/// C: #define ERROR_TOO_MANY_LINKS 1142L
#[cfg (feature = "windowsnt")]
pub const ERROR_TOO_MANY_LINKS: DWORD = 1142;
/// <errno.h>: `#define EMLINK 31` (`lib/include/errno.rs` does not carry it).
#[cfg (feature = "windowsnt")]
pub const EMLINK: c_int = 31;
/// C: WINBASEAPI HANDLE WINAPI CreateFileA (LPCSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE);
/// C: WINBASEAPI HANDLE WINAPI CreateFileW (LPCWSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE);
/// C: WINBASEAPI BOOL WINAPI BackupWrite (HANDLE, LPBYTE, DWORD, LPDWORD, BOOL, BOOL, LPVOID *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn CreateFileA (lpFileName: LPCSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
		      lpSecurityAttributes: LPVOID, dwCreationDisposition: DWORD,
		      dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE) -> HANDLE;
  pub fn CreateFileW (lpFileName: LPCWSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
		      lpSecurityAttributes: LPVOID, dwCreationDisposition: DWORD,
		      dwFlagsAndAttributes: DWORD, hTemplateFile: HANDLE) -> HANDLE;
  pub fn BackupWrite (hFile: HANDLE, lpBuffer: LPBYTE, nNumberOfBytesToWrite: DWORD,
		      lpNumberOfBytesWritten: LPDWORD, bAbort: BOOL, bProcessSecurity: BOOL,
		      lpContext: *mut LPVOID) -> BOOL;
}
/// C 4659-4672: the two `<direct.h>` CRT names `sys_mkdir` calls.
/// C: int _mkdir (const char *);
/// C: int _wmkdir (const wchar_t *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _mkdir (path: *const c_char) -> c_int;
  pub fn _wmkdir (path: *const c_ushort) -> c_int;
}
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_link (old: *const c_char, new: *const c_char) -> c_int // C 4537
{
  let mut fileh: HANDLE;                                                // C 4539
  let mut result: c_int = -1;                                           // C 4540
  let mut oldname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 4541
  let mut newname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];
  let mut oldname_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4542
  let mut oldname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4543

  /* C leaves `fileh` uninitialised (PORTING.md 1.5); C 4540's `-1` and the arrays are C's own
     initialisers.  */
  fileh = std::ptr::null_mut ();

  if old.is_null () || new.is_null ()                                   // C 4545
    {
      *errno () = ENOENT;                                               // C 4547
      return -1;                                                        // C 4548
    }

  strcpy ((&raw mut oldname) as *mut c_char, map_w32_filename (old, std::ptr::null_mut ())); // C 4551
  strcpy ((&raw mut newname) as *mut c_char, map_w32_filename (new, std::ptr::null_mut ())); // C 4552

  if w32_unicode_filenames                                              // C 4554
    {
      filename_to_utf16 ((&raw const oldname) as *const c_char, oldname_w.as_mut_ptr ()); // C 4556
      fileh = CreateFileW (oldname_w.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 4557
			   FILE_FLAG_BACKUP_SEMANTICS, std::ptr::null_mut ());               // C 4558
    }
  else                                                                  // C 4560
    {
      filename_to_ansi ((&raw const oldname) as *const c_char, oldname_a.as_mut_ptr ()); // C 4562
      fileh = CreateFileA (oldname_a.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 4563
			   FILE_FLAG_BACKUP_SEMANTICS, std::ptr::null_mut ());               // C 4564
    }
  if fileh != INVALID_HANDLE_VALUE                                      // C 4566
    {
      let mut wlen: c_int;                                              // C 4568

      /* C leaves `wlen` uninitialised (PORTING.md 1.5).  */
      wlen = 0;

      /* Confusingly, the "alternate" stream name field does not apply  // C 4570
         when restoring a hard link, and instead contains the actual    // C 4571
         stream data for the link (ie. the name of the link to create). // C 4572
         The WIN32_STREAM_ID structure before the cStreamName field is  // C 4573
         the stream header, which is then immediately followed by the   // C 4574
         stream data.  */                                               // C 4575

      let mut data: w32_sys_link_data = std::mem::zeroed ();            // C 4577-4580 (C's anonymous struct)

      /* We used to pass MB_PRECOMPOSED as the 2nd arg here, but MSDN   // C 4582
	 indicates that flag is unsupported for CP_UTF8, and OTOH says         // C 4583
         it is the default anyway.  */                                  // C 4584
      wlen = (pMultiByteToWideChar.unwrap ()) (CP_UTF8 as UINT,         // C 4585
				        multiByteToWideCharFlags as c_ulong,
				        newname.as_ptr (), -1,
				        data.wid.cStreamName.as_mut_ptr (), MAX_PATH);              // C 4586
      if wlen > 0                                                       // C 4587
	{
	  let mut context: LPVOID = std::ptr::null_mut ();                     // C 4589
	  let mut wbytes: DWORD = 0;                                           // C 4590

	  data.wid.dwStreamId = BACKUP_LINK;                                   // C 4592
	  data.wid.dwStreamAttributes = 0;                                     // C 4593
	  data.wid.Size.u.LowPart = (wlen as c_uint).wrapping_mul (std::mem::size_of::<WCHAR> () as c_uint); // C 4594 (C's `wlen * sizeof (WCHAR)`; the SDK's nameless inner struct is `u`)
	  data.wid.Size.u.HighPart = 0;                                        // C 4595
	  data.wid.dwStreamNameSize = 0;                                       // C 4596

	  if (BackupWrite (fileh, (&raw mut data) as LPBYTE,                   // C 4598
			  (core::mem::offset_of! (WIN32_STREAM_ID, cStreamName) as DWORD)    // C 4599
			  .wrapping_add (data.wid.Size.u.LowPart),                           // C 4600
			  (&raw mut wbytes), 0, 0, (&raw mut context)) != 0                  // C 4601
	      && BackupWrite (fileh, std::ptr::null_mut (), 0, (&raw mut wbytes), 1, 0, // C 4602
			  (&raw mut context)) != 0)
	    {
	      /* succeeded */                                                  // C 4604
	      result = 0;                                                      // C 4605
	    }
	  else
	    {
	      let mut err: DWORD = GetLastError ();                            // C 4609
	      let mut attributes: DWORD;                                       // C 4610

	      /* C leaves `attributes` uninitialised (PORTING.md 1.5).  */
	      attributes = 0;

	      match err                                                        // C 4612
		{
		  ERROR_ACCESS_DENIED =>                                              // C 4614
		    {
		      /* This is what happens when OLDNAME is a directory,            // C 4615
			 since Windows doesn't support hard links to                         // C 4616
			 directories.  Posix says to set errno to EPERM in                   // C 4617
			 that case.  */                                                      // C 4618
		      if w32_unicode_filenames                                        // C 4619
			{ attributes = GetFileAttributesW (oldname_w.as_ptr ()); }           // C 4620
		      else
			{ attributes = GetFileAttributesA (oldname_a.as_ptr ()); }           // C 4622
		      if (attributes != -1 as DWORD                                    // C 4623
			  && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0)                    // C 4624
			{ *errno () = EPERM; }                                               // C 4625
		      else if (attributes == -1 as DWORD                               // C 4626
			   && is_unc_volume ((&raw const oldname) as *const c_char) != 0     // C 4627
			   && unc_volume_file_attributes ((&raw const oldname) as *const c_char) != -1 as DWORD) // C 4628
			{ *errno () = EPERM; }                                               // C 4629
		      else
			{ *errno () = EACCES; }                                              // C 4631
		    }
		  ERROR_TOO_MANY_LINKS =>                                             // C 4633
		    { *errno () = EMLINK; }                                           // C 4634
		  ERROR_NOT_SAME_DEVICE =>                                            // C 4636
		    { *errno () = EXDEV; }                                            // C 4637
		  _ =>                                                                // C 4639
		    { *errno () = EINVAL; }                                           // C 4640
		}
	    }
	}

      CloseHandle (fileh);                                              // C 4646
    }
  else                                                                  // C 4648
    { *errno () = ENOENT; }                                             // C 4649

  return result;                                                        // C 4651
}
/* C 4654-4673: C 4653's blank and `sys_mkdir` (C's `mode_t mode` is unused in the body).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_mkdir (path: *const c_char, mode: mode_t) -> c_int // C 4654
{
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 4657

  if w32_unicode_filenames                                              // C 4659
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4661

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 4663
      return _wmkdir (path_w.as_ptr ());                                // C 4664
    }
  else                                                                  // C 4666
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4668

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 4670
      return _mkdir (path_a.as_ptr ());                                 // C 4671
    }
}

/* C 4674: the blank before C 4675's `int` (the next item is C 4675's `sys_open`, past this
   slice).  */
/* C 4674-4675: C 4674's blank and C 4675's `int`.  */
/* C 4675-4729: `sys_open`.  C 4676's `...` parameter and C 4680/4683-4684/4726's `va_list ap`
   / `va_start` / `va_arg (ap, int)` / `va_end` are PORTING.md P-04's shape
   (`docs/spikes/2026-10-02-variadics.md` 37: the `...` parameter *is* the list, so `va_start`
   and `va_end` have no statement and the one read is `ap.next_arg::<c_int> ()`).  The two CRT
   names and the SDK scalar this slice first spells are declared here (P-88, `UNVERIFIED`).  */
/// C: #define _O_CREAT 0x0100 (the CRT's own value; the mirror's `O_CREAT` is the frozen target's)
#[cfg (feature = "windowsnt")]
pub const _O_CREAT: c_int = 0x0100;
/// C: #define _O_EXCL 0x0200 (as above)
#[cfg (feature = "windowsnt")]
pub const _O_EXCL: c_int = 0x0200;
/// C: #define INVALID_FILE_ATTRIBUTES ((DWORD) -1) (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const INVALID_FILE_ATTRIBUTES: DWORD = -1i32 as DWORD;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_open (path: *const c_char, oflag: c_int, mut ap: ...) -> c_int // C 4675
{
  let mpath: *const c_char = map_w32_filename (path, std::ptr::null_mut ()); // C 4678
  let mut res: c_int = -1;                                              // C 4679
  let mut mode: c_int;                                                  // C 4681

  /* C 4680's `va_list ap;` is the port's `...` parameter (P-04).  */
  mode = ap.next_arg::<c_int> ();                                        // C 4684

  if w32_unicode_filenames                                              // C 4686
    {
      let mut mpath_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4688

      filename_to_utf16 (mpath, mpath_w.as_mut_ptr ());                 // C 4690
      /* If possible, try to open file without _O_CREAT, to be able to  // C 4691
	 write to existing hidden and system files.  Force all file            // C 4692
	 handles to be non-inheritable. */                                     // C 4693
      if (oflag & (_O_CREAT | _O_EXCL)) != (_O_CREAT | _O_EXCL)         // C 4694
	{ res = _wopen (mpath_w.as_mut_ptr (), (oflag & ! _O_CREAT) | _O_NOINHERIT, mode); } // C 4695
      if res < 0                                                        // C 4696
	{ res = _wopen (mpath_w.as_mut_ptr (), oflag | _O_NOINHERIT, mode); }  // C 4697
      if res < 0 && *errno () == EACCES                                 // C 4698
	{
	  let mut attributes: DWORD = GetFileAttributesW (mpath_w.as_ptr ());  // C 4700

	  if (attributes != INVALID_FILE_ATTRIBUTES                            // C 4702
	      && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0)                 // C 4703
	    { *errno () = EISDIR; }                                            // C 4704
	}
    }
  else                                                                  // C 4707
    {
      let mut mpath_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4709

      filename_to_ansi (mpath, mpath_a.as_mut_ptr ());                  // C 4711
      if (oflag & (_O_CREAT | _O_EXCL)) != (_O_CREAT | _O_EXCL)         // C 4712
	{ res = _open (mpath_a.as_mut_ptr (), (oflag & ! _O_CREAT) | _O_NOINHERIT, mode); } // C 4713
      if res < 0                                                        // C 4714
	{ res = _open (mpath_a.as_mut_ptr (), oflag | _O_NOINHERIT, mode); }   // C 4715
      if res < 0 && *errno () == EACCES                                 // C 4716
	{
	  let mut attributes: DWORD = GetFileAttributesA (mpath_a.as_ptr ());  // C 4718

	  if (attributes != INVALID_FILE_ATTRIBUTES                            // C 4720
	      && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0)                 // C 4721
	    { *errno () = EISDIR; }                                            // C 4722
	}
    }

  /* C 4726's `va_end (ap);` has no statement (P-04).  */               // C 4726

  return res;                                                           // C 4728
}

/* C 4730-4754: C 4730's blank, C 4731-4732's
   `/* This is not currently used, but might be needed again at some point; DO NOT DELETE!  */`
   and C 4733's `#if 0` … C 4754's `#endif`: dead in every configuration, so the block is
   **recorded, not carried** (the `android.c` 1124-1360 device).  */
/* C 4756-4760: C 4756's `int` and `fchmod` (a body that is only C 4759's `return 0;`).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fchmod (fd: c_int, mode: mode_t) -> c_int      // C 4756
{
  return 0;                                                             // C 4759
}
/* C 4762-4782: C 4761's blank, C 4762's `int` and `fchmodat` (the same directory hack
   `faccessat` uses, then C 4780-4781's two-way dispatch).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fchmodat (fd: c_int, path: *const c_char, mode: mode_t, flags: c_int) // C 4762
  -> c_int
{
  let mut fullname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 4767

  /* Rely on a hack: an open directory is modeled as file descriptor 0, // C 4765
     as in fstatat.  FIXME: Add proper support for fchmodat.  */        // C 4766
  if fd != AT_FDCWD                                                     // C 4769
    {
      if (_snprintf ((&raw mut fullname) as *mut c_char,                // C 4771
		   core::mem::size_of_val (&fullname),
		   c"%s/%s".as_ptr (),                                                // C 4771
		   dir_pathname, path)                                                // C 4771
	  < 0)                                                                 // C 4772
	{
	  *errno () = ENAMETOOLONG;                                            // C 4774
	  return -1;                                                           // C 4775
	}
      path = (&raw mut fullname) as *const c_char;                      // C 4777
    }

  return                                                                // C 4780
    if flags == AT_SYMLINK_NOFOLLOW { lchmod (path, mode) } else { sys_chmod (path, mode as c_int) }; // C 4781 (C's `mode_t`-into-`int` at the `sys_chmod` call)
}

/* C 4783-4784: C 4783's blank and C 4784's `int` (the next item is C 4785's
   `sys_rename_replace`, past this slice).  */
/* C 4783-4785: C 4783's blank, C 4784's `int` and C 4785's signature.  */
/* C 4785-5007: `sys_rename_replace`.  The five CRT names it first calls and the one
   `<winerror.h>` value are declared here (P-88, `UNVERIFIED`); C 4989/4997-5004's
   `return_result:` label is the port's labelled block (the `androidmenu.rs` `'finish` shape:
   C 4901/4913/4918/4921/4960/4972/4977/4980's forward jumps are `break 'return_result`), and
   C 4813/4925/4984 call `is_symlink`, whose C 807 prototype the port has no item for yet (the
   `ltime` class).  */
/// C: int rename (const char *, const char *); (`w32.c` 60's `#undef rename` undoes
/// `nt/inc/ms-w32.h` 223's `#define rename sys_rename`, so C's own calls are the CRT's.)
/// C: int _wrename (const wchar_t *, const wchar_t *);
/// C: int _rmdir (const char *);
/// C: int _wrmdir (const wchar_t *);
/// C: int _unlink (const char *);
/// C: int _wunlink (const wchar_t *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  /// `rename` itself: the unit's `<stdio.h>` reach does not carry it, so it is declared here
  /// (P-88).
  pub fn rename (old: *const c_char, new: *const c_char) -> c_int;
  pub fn _wrename (old: *const c_ushort, new: *const c_ushort) -> c_int;
  pub fn _rmdir (path: *const c_char) -> c_int;
  pub fn _wrmdir (path: *const c_ushort) -> c_int;
  pub fn _unlink (path: *const c_char) -> c_int;
  pub fn _wunlink (path: *const c_ushort) -> c_int;
}
/// C: #define ERROR_PRIVILEGE_NOT_HELD 1314L
#[cfg (feature = "windowsnt")]
pub const ERROR_PRIVILEGE_NOT_HELD: DWORD = 1314;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_rename_replace (mut oldname: *const c_char, mut newname: *const c_char, // C 4785
						      force: BOOL)
  -> c_int
{
  let mut result: BOOL;                                                 // C 4787
  let mut temp: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 4788
  let mut temp_a: [c_char; MAX_PATH as usize + 15] = [0; MAX_PATH as usize + 15];  /* "+ 15": pacify GCC */
  let mut newname_dev: c_int;                                           // C 4789
  let mut oldname_dev: c_int;                                           // C 4790
  let mut have_temp_a: bool = false;                                    // C 4791
  let mut oldname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4792

  /* C leaves `result`, `newname_dev` and `oldname_dev` uninitialised (PORTING.md 1.5).  */
  result = 0;
  newname_dev = 0;
  oldname_dev = 0;

  strcpy ((&raw mut temp) as *mut c_char, map_w32_filename (oldname, std::ptr::null_mut ())); // C 4807

  /* 'rename' (which calls MoveFileW) renames the _target_ of the       // C 4809
     symlink, which is different from Posix behavior and not what we    // C 4810
     want here.  So in that case we pretend this is a cross-device move, // C 4811
     for which Frename_file already has a workaround.  */               // C 4812
  if is_symlink ((&raw const temp) as *const c_char) != 0               // C 4813
    {
      *errno () = EXDEV;                                                // C 4815
      return -1;                                                        // C 4816
    }
  /* volume_info is set indirectly by map_w32_filename.  */             // C 4818
  oldname_dev = volume_info.serialnum as c_int;                         // C 4819 (C's `DWORD`-into-`int`)

  if os_subtype == OS_SUBTYPE_9X                                        // C 4821
    {
      let mut o: *mut c_char;                                           // C 4823
      let mut p: *mut c_char;                                           // C 4824
      let mut i: c_int = 0;                                             // C 4825

      /* C leaves `o` and `p` uninitialised (PORTING.md 1.5).  */
	  o = std::ptr::null_mut ();
	  p = std::ptr::null_mut ();

      oldname = map_w32_filename (oldname, std::ptr::null_mut ());      // C 4827
      filename_to_ansi (oldname, oldname_a.as_mut_ptr ());              // C 4828
      filename_to_ansi ((&raw const temp) as *const c_char, temp_a.as_mut_ptr ()); // C 4829
      o = strrchr ((&raw mut oldname_a) as *mut c_char, b'\\' as c_int); // C 4830
      if ! o.is_null ()                                                 // C 4830
	{ o = o.offset (1); }                                                  // C 4831
	      else
	{ o = (&raw mut oldname_a) as *mut c_char; }                           // C 4833 (C's array decay)

      p = strrchr ((&raw mut temp_a) as *mut c_char, b'\\' as c_int);   // C 4835
      if ! p.is_null ()                                                 // C 4835
	{ p = p.offset (1); }                                                  // C 4836
	      else
	{ p = (&raw mut temp_a) as *mut c_char; }                              // C 4838 (C's array decay)

      loop                                                              // C 4840
	{
	  /* Force temp name to require a manufactured 8.3 alias - this        // C 4842
	     seems to make the second rename work properly.  */                // C 4843
	  sprintf (p, c"_.%s.%d".as_ptr (), o, i);                             // C 4844
	  i += 1;                                                              // C 4845 (C's `i++`)
	  result = rename ((&raw const oldname_a) as *const c_char, (&raw const temp_a) as *const c_char); // C 4846 (C's `rename`)

	  /* This loop must surely terminate!  */                              // C 4848
	  if ! (result < 0 && *errno () == EEXIST)                             // C 4849 (C's `while (result < 0 && errno == EEXIST)`)
	    { break; }
	}
      if result < 0                                                     // C 4850
	{ return -1; }                                                         // C 4851
      have_temp_a = true;                                               // C 4852
    }

  /* If FORCE, emulate Unix behavior - newname is deleted if it already exists // C 4855
     (at least if it is a file; don't do this for directories).         // C 4856
                                                                        // C 4857
     Since we mustn't do this if we are just changing the case of the   // C 4858
     file name (we would end up deleting the file we are trying to      // C 4859
     rename!), we let rename detect if the destination file already     // C 4860
     exists - that way we avoid the possible pitfalls of trying to      // C 4861
     determine ourselves whether two names really refer to the same     // C 4862
     file, which is not always possible in the general case.  (Consider // C 4863
     all the permutations of shared or subst'd drives, etc.)  */        // C 4864

  newname = map_w32_filename (newname, std::ptr::null_mut ());          // C 4866

  /* volume_info is set indirectly by map_w32_filename.  */             // C 4868
  newname_dev = volume_info.serialnum as c_int;                         // C 4869 (C's `DWORD`-into-`int`)

  'return_result: {                                                     // C 4989 (C's `return_result:` label; the jumps are `break 'return_result`)
  if w32_unicode_filenames                                              // C 4871
    {
      let mut temp_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4873
      let mut newname_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize];

      filename_to_utf16 ((&raw const temp) as *const c_char, temp_w.as_mut_ptr ()); // C 4875
      filename_to_utf16 (newname, newname_w.as_mut_ptr ());             // C 4876
      result = _wrename (temp_w.as_mut_ptr (), newname_w.as_mut_ptr ()); // C 4877
      if result < 0                                                     // C 4878
	{
	  let mut w32err: DWORD = GetLastError ();                             // C 4880

	  if *errno () == EACCES                                               // C 4882
	      && newname_dev != oldname_dev                                    // C 4883
	    {
	      let mut attributes: DWORD;                                       // C 4885
	      /* The implementation of `rename' on Windows does not return     // C 4886
		 errno = EXDEV when you are moving a directory to a                   // C 4887
		 different storage device (ex. logical disk).  It returns             // C 4888
		 EACCES instead.  So here we handle such situations and               // C 4889
		 return EXDEV.  */                                                    // C 4890
	      /* C leaves `attributes` uninitialised (PORTING.md 1.5).  */
	      attributes = 0;
	      attributes = GetFileAttributesW (temp_w.as_ptr ());              // C 4891
	      if attributes != -1 as DWORD                                     // C 4891
		  && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0                     // C 4892
		{ *errno () = EXDEV; }                                                // C 4893
	    }
	  else if *errno () == EEXIST && force != 0                            // C 4895
	    {
	      let mut attributes_old: DWORD;                                   // C 4897
	      let mut attributes_new: DWORD;                                   // C 4898

	      /* C leaves the two uninitialised (PORTING.md 1.5).  */
	      attributes_old = 0;
	      attributes_new = 0;

	      if _wchmod (newname_w.as_mut_ptr (), 0o666) != 0                 // C 4900 (C's `0666`)
		{ break 'return_result; }                                             // C 4901 (C's `goto return_result`)
	      attributes_old = GetFileAttributesW (temp_w.as_ptr ());          // C 4902
	      attributes_new = GetFileAttributesW (newname_w.as_ptr ());       // C 4903
	      if (attributes_old != -1 as DWORD && attributes_new != -1 as DWORD // C 4904
		  && ((attributes_old & FILE_ATTRIBUTE_DIRECTORY)                     // C 4905
		      != (attributes_new & FILE_ATTRIBUTE_DIRECTORY)))               // C 4906
		{
		  if (attributes_old & FILE_ATTRIBUTE_DIRECTORY) != 0                 // C 4908
		    { *errno () = ENOTDIR; }                                          // C 4909
		  else
		    { *errno () = EISDIR; }                                           // C 4911
		  result = -1;                                                        // C 4912
		  break 'return_result;                                               // C 4913 (C's `goto return_result`)
		}
	      if (attributes_new & FILE_ATTRIBUTE_DIRECTORY) != 0              // C 4915
		{
		  if _wrmdir (newname_w.as_mut_ptr ()) != 0                           // C 4917
		    { break 'return_result; }                                         // C 4918 (C's `goto return_result`)
		}
	      else if _wunlink (newname_w.as_mut_ptr ()) != 0                  // C 4920
		{ break 'return_result; }                                             // C 4921 (C's `goto return_result`)
	      result = _wrename (temp_w.as_mut_ptr (), newname_w.as_mut_ptr ()); // C 4922
	    }
	  else if w32err == ERROR_PRIVILEGE_NOT_HELD                           // C 4924
		   && is_symlink ((&raw const temp) as *const c_char) != 0            // C 4925
	    {
	      /* This is Windows prohibiting the user from creating a          // C 4927
		 symlink in another place, since that requires                        // C 4928
		 privileges.  */                                                      // C 4929
	      *errno () = EPERM;                                               // C 4930
	    }
	}
    }
  else                                                                  // C 4934
    {
      let mut newname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 4936

      if ! have_temp_a                                                  // C 4938
	{ filename_to_ansi ((&raw const temp) as *const c_char, temp_a.as_mut_ptr ()); } // C 4939
      filename_to_ansi (newname, newname_a.as_mut_ptr ());              // C 4940
      result = rename ((&raw const temp_a) as *const c_char, (&raw const newname_a) as *const c_char); // C 4941 (C's `rename`)
      if result < 0                                                     // C 4942
	{
	  let mut w32err: DWORD = GetLastError ();                             // C 4944

	  if *errno () == EACCES                                               // C 4946
	      && newname_dev != oldname_dev                                    // C 4947
	    {
	      let mut attributes: DWORD;                                       // C 4949
	      /* C leaves `attributes` uninitialised (PORTING.md 1.5).  */
	      attributes = 0;
	      attributes = GetFileAttributesA (temp_a.as_ptr ());              // C 4950
	      if attributes != -1 as DWORD                                     // C 4950
		  && (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0                     // C 4951
		{ *errno () = EXDEV; }                                                // C 4952
	    }
	  else if *errno () == EEXIST && force != 0                            // C 4954
	    {
	      let mut attributes_old: DWORD;                                   // C 4956
	      let mut attributes_new: DWORD;                                   // C 4957

	      /* C leaves the two uninitialised (PORTING.md 1.5).  */
	      attributes_old = 0;
	      attributes_new = 0;

	      if _chmod (newname_a.as_mut_ptr (), 0o666) != 0                  // C 4959 (C's `0666`)
		{ break 'return_result; }                                             // C 4960 (C's `goto return_result`)
	      attributes_old = GetFileAttributesA (temp_a.as_ptr ());          // C 4961
	      attributes_new = GetFileAttributesA (newname_a.as_ptr ());       // C 4962
	      if (attributes_old != -1 as DWORD && attributes_new != -1 as DWORD // C 4963
		  && ((attributes_old & FILE_ATTRIBUTE_DIRECTORY)                     // C 4964
		      != (attributes_new & FILE_ATTRIBUTE_DIRECTORY)))               // C 4965
		{
		  if (attributes_old & FILE_ATTRIBUTE_DIRECTORY) != 0                 // C 4967
		    { *errno () = ENOTDIR; }                                          // C 4968
		  else
		    { *errno () = EISDIR; }                                           // C 4970
		  result = -1;                                                        // C 4971
		  break 'return_result;                                               // C 4972 (C's `goto return_result`)
		}
	      if (attributes_new & FILE_ATTRIBUTE_DIRECTORY) != 0              // C 4974
		{
		  if _rmdir (newname_a.as_mut_ptr ()) != 0                            // C 4976
		    { break 'return_result; }                                         // C 4977 (C's `goto return_result`)
		}
	      else if _unlink (newname_a.as_mut_ptr ()) != 0                   // C 4979
		{ break 'return_result; }                                             // C 4980 (C's `goto return_result`)
	      result = rename ((&raw const temp_a) as *const c_char, (&raw const newname_a) as *const c_char); // C 4981
	    }
	  else if w32err == ERROR_PRIVILEGE_NOT_HELD                           // C 4983
		   && is_symlink ((&raw const temp) as *const c_char) != 0            // C 4984
	    { *errno () = EPERM; }                                             // C 4985
	}
    }
  }

  /* return_result: */                                                  // C 4989
  /* This label is also invoked on failure, and on Windows 9X, restores // C 4990
     the initial name of files that will have been renamed in           // C 4991
     preparation for being moved.  It ought to be valid to rename temp_a // C 4992
     to its previous name, just as it would, but for the failure, have  // C 4993
     been renamed to the target.  */                                    // C 4994

  if have_temp_a && result != 0                                         // C 4996
    {
      let mut save_errno: c_int = *errno ();                            // C 4998

      /* XXX: what if a new file has replaced oldname_a in the          // C 5000
	 meantime?  */                                                         // C 5001
      rename ((&raw const temp_a) as *const c_char, (&raw const oldname_a) as *const c_char); // C 5002 (C's `rename`)
      *errno () = save_errno;                                           // C 5003
    }

  return result;                                                        // C 5006
}
/* C 5009-5013: C 5009's `int` and `sys_rename` (C's `TRUE` is the `BOOL` `1`).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_rename (old: *const c_char, new: *const c_char) -> c_int // C 5009
{
  return sys_rename_replace (old, new, 1);                              // C 5012 (C's `TRUE`)
}

/* C 5014-5015: C 5014's blank and C 5015's `int` (the next item is C 5016's `sys_rmdir`, past
   this slice).  */
/* C 5014-5016: C 5014's blank, C 5015's `int` and C 5016's signature.  */
/* C 5016-5034: `sys_rmdir`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_rmdir (path: *const c_char) -> c_int       // C 5016
{
  path = map_w32_filename (path, std::ptr::null_mut ());                // C 5018

  if w32_unicode_filenames                                              // C 5020
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5022

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 5024
      return _wrmdir (path_w.as_mut_ptr ());                            // C 5025
    }
  else                                                                  // C 5027
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5029

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 5031
      return _rmdir (path_a.as_mut_ptr ());                             // C 5032
    }
}
/* C 5036-5078: C 5036's `int` and `sys_unlink` (C 5051/5068's `e = errno` save, C 5056/5071's
   `is_symlink (path) & FILE_ATTRIBUTE_DIRECTORY` test, where C's `int` converts to the
   `DWORD`).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_unlink (path: *const c_char) -> c_int      // C 5036
{
  let mut rmstatus: c_int;                                              // C 5039
  let mut e: c_int;

  /* C leaves the two locals uninitialised (PORTING.md 1.5).  */
  rmstatus = 0;
  e = 0;

  path = map_w32_filename (path, std::ptr::null_mut ());                // C 5041

  if w32_unicode_filenames                                              // C 5043
    {
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5045

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 5047
      /* On Unix, unlink works without write permission.  */            // C 5048
      _wchmod (path_w.as_mut_ptr (), 0o666);                            // C 5049 (C's `0666`)
      rmstatus = _wunlink (path_w.as_mut_ptr ());                       // C 5050
      e = *errno ();                                                    // C 5051
      /* Symlinks to directories can only be deleted by _rmdir;         // C 5052
	 _unlink returns EACCES.  */                                           // C 5053
      if (rmstatus != 0                                                 // C 5054
	  && *errno () == EACCES                                               // C 5055
	  && ((is_symlink (path) as DWORD) & FILE_ATTRIBUTE_DIRECTORY) != 0)   // C 5056
	{ rmstatus = _wrmdir (path_w.as_mut_ptr ()); }                         // C 5057
	      else
	{ *errno () = e; }                                                     // C 5059
    }
  else                                                                  // C 5061
    {
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5063

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 5065
      _chmod (path_a.as_mut_ptr (), 0o666);                             // C 5066 (C's `0666`)
      rmstatus = _unlink (path_a.as_mut_ptr ());                        // C 5067
      e = *errno ();                                                    // C 5068
      if (rmstatus != 0                                                 // C 5069
	  && *errno () == EACCES                                               // C 5070
	  && ((is_symlink (path) as DWORD) & FILE_ATTRIBUTE_DIRECTORY) != 0)   // C 5071
	{ rmstatus = _rmdir (path_a.as_mut_ptr ()); }                          // C 5072
	      else
	{ *errno () = e; }                                                     // C 5074
    }

  return rmstatus;                                                      // C 5077
}
/* C 5080-5090: the three file-scope statics and the `FILETIME_TO_U64` macro.  The SDK types this
   group first names (`SYSTEMTIME`, `SystemTimeToFileTime`, `CompareFileTime`) are declared
   below (P-88, `UNVERIFIED`), and C's `do { … } while (0)` macro is the port's block
   (PORTING.md 2.1).  */
#[cfg (feature = "windowsnt")]
static mut utc_base_ft: FILETIME = unsafe { std::mem::zeroed () };   // C 5080 (tentative)
#[cfg (feature = "windowsnt")]
static mut utc_base: ULONGLONG = 0;  /* In 100ns units */           // C 5081 // C 5081
#[cfg (feature = "windowsnt")]
static mut init: c_int = 0;                                          // C 5082 // C 5082
#[cfg (feature = "windowsnt")]
/// C: typedef struct _SYSTEMTIME { WORD wYear; WORD wMonth; WORD wDayOfWeek; WORD wDay;
/// C:   WORD wHour; WORD wMinute; WORD wSecond; WORD wMilliseconds; } SYSTEMTIME;
#[repr(C)]
#[derive(Clone, Copy)]
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
/// C: WINBASEAPI BOOL WINAPI SystemTimeToFileTime (const SYSTEMTIME *, LPFILETIME);
/// C: WINBASEAPI LONG WINAPI CompareFileTime (const FILETIME *, const FILETIME *);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn SystemTimeToFileTime (lpSystemTime: *const SYSTEMTIME, lpFileTime: *mut FILETIME) -> BOOL;
  pub fn CompareFileTime (lpFileTime1: *const FILETIME, lpFileTime2: *const FILETIME) -> LONG;
}
/// C 5084-5090's `#define FILETIME_TO_U64(result, ft) do { ULARGE_INTEGER uiTemp; … } while (0)`.
macro_rules! FILETIME_TO_U64 {
  ($result:expr, $ft:expr) =>
    {
      let mut uiTemp: ULARGE_INTEGER = unsafe { std::mem::zeroed () };
      uiTemp.u.LowPart = ($ft).dwLowDateTime;
      uiTemp.u.HighPart = ($ft).dwHighDateTime;
      $result = uiTemp.QuadPart;
    };
}
/* C 5092-5108: C 5092's `static void` and `initialize_utc_base`.  */
#[cfg (feature = "windowsnt")]
unsafe fn initialize_utc_base ()                                        // C 5092
{
  let mut st: SYSTEMTIME = std::mem::zeroed ();                         // C 5096

  /* Determine the delta between 1-Jan-1601 and 1-Jan-1970. */          // C 5095
  st.wYear = 1970;                                                      // C 5098
  st.wMonth = 1;                                                        // C 5099
  st.wDay = 1;                                                          // C 5100
  st.wHour = 0;                                                         // C 5101
  st.wMinute = 0;                                                       // C 5102
  st.wSecond = 0;                                                       // C 5103
  st.wMilliseconds = 0;                                                 // C 5104

  SystemTimeToFileTime ((&raw const st), (&raw mut utc_base_ft));       // C 5106
  FILETIME_TO_U64! (utc_base, utc_base_ft);                             // C 5107
}
/* C 5110-5130: C 5110's `static time_t` and `convert_time`.  */
#[cfg (feature = "windowsnt")]
unsafe fn convert_time (ft: FILETIME, time_nsec: *mut c_int) -> time_t  // C 5110
{
  let mut tmp: ULONGLONG;                                               // C 5113
  let mut time_sec: time_t;                                             // C 5114

  /* C leaves the two locals uninitialised (PORTING.md 1.5).  */
  tmp = 0;
  time_sec = 0;

  if init == 0                                                          // C 5116
    {
      initialize_utc_base ();                                           // C 5118
      init = 1;                                                         // C 5119
    }

  if CompareFileTime ((&raw const ft), (&raw const utc_base_ft)) < 0    // C 5122
    { return 0; }                                                       // C 5123

  FILETIME_TO_U64! (tmp, ft);                                           // C 5125
  tmp -= utc_base;                                                      // C 5126
  time_sec = (tmp / 10000000) as time_t;                                // C 5127 (C's `(time_t) (tmp / 10000000L)`)
  *time_nsec = ((tmp - time_sec as ULONGLONG * 10000000) * 100) as c_int; // C 5128 (C's `int`-store)
  return time_sec;                                                      // C 5129
}
/* C 5132-5148: C 5132's `static void` and `convert_from_timespec`.  */
#[cfg (feature = "windowsnt")]
unsafe fn convert_from_timespec (time: timespec, pft: *mut FILETIME)    // C 5132
{
  let mut tmp: ULARGE_INTEGER = std::mem::zeroed ();                    // C 5135

  if init == 0                                                          // C 5137
    {
      initialize_utc_base ();                                           // C 5139
      init = 1;                                                         // C 5140
    }

  /* time in 100ns units since 1-Jan-1601 */                            // C 5143
  tmp.QuadPart =                                                        // C 5144
    time.tv_sec as ULONGLONG * 10000000 + time.tv_nsec as c_long / 100 + utc_base; // C 5145
  (*pft).dwHighDateTime = tmp.u.HighPart;                               // C 5146
  (*pft).dwLowDateTime = tmp.u.LowPart;                                 // C 5147
}

/* C 5150: the blank before C 5150's `static PSECURITY_DESCRIPTOR` (the next item is C 5150's
   `get_file_security_desc_by_handle`, past this slice).  */
/* C 5149-5151: C 5149's blank, C 5150's `static PSECURITY_DESCRIPTOR` and C 5151's signature.  */
/* C 5150-5202: the three ACL entries.  The `<winnt.h>`/`<accctrl.h>` names they first use are
   declared here (P-88, `UNVERIFIED`); `get_security_info`, `get_file_security`,
   `get_sid_sub_authority_count` and `get_sid_sub_authority` are this unit's own C 700-830
   wrappers (carried in slice 1), so their signatures are the port's own.  */
/// C: #define OWNER_SECURITY_INFORMATION 0x00000001
#[cfg (feature = "windowsnt")]
pub const OWNER_SECURITY_INFORMATION: DWORD = 0x00000001;
/// C: #define GROUP_SECURITY_INFORMATION 0x00000002
#[cfg (feature = "windowsnt")]
pub const GROUP_SECURITY_INFORMATION: DWORD = 0x00000002;
/// C: typedef enum _SE_OBJECT_TYPE { SE_UNKNOWN_OBJECT_TYPE = 0, SE_FILE_OBJECT, … };
#[cfg (feature = "windowsnt")]
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = 1;
/// C: #define ERROR_SUCCESS 0L
#[cfg (feature = "windowsnt")]
pub const ERROR_SUCCESS: DWORD = 0;
#[cfg (feature = "windowsnt")]
unsafe fn get_file_security_desc_by_handle (h: HANDLE) -> PSECURITY_DESCRIPTOR // C 5150
{
  let mut psd: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();            // C 5153
  let mut err: DWORD;                                                   // C 5154
  let mut si: SECURITY_INFORMATION = OWNER_SECURITY_INFORMATION         // C 5155
    | GROUP_SECURITY_INFORMATION  /* | DACL_SECURITY_INFORMATION */ ;   // C 5156

  /* C leaves `err` uninitialised (PORTING.md 1.5).  */
  err = 0;

  err = get_security_info (h, SE_FILE_OBJECT, si,                       // C 5158
			   std::ptr::null_mut (), std::ptr::null_mut (),                     // C 5159
			   std::ptr::null_mut (), std::ptr::null_mut (), (&raw mut psd));
  if err != ERROR_SUCCESS                                               // C 5160
    { return std::ptr::null_mut (); }                                   // C 5161

  return psd;                                                           // C 5163
}
/* C 5166-5189: C 5166's `static PSECURITY_DESCRIPTOR` and `get_file_security_desc_by_name`.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_file_security_desc_by_name (fname: *const c_char) -> PSECURITY_DESCRIPTOR // C 5166
{
  let mut psd: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();            // C 5169
  let mut sd_len: DWORD;                                                // C 5170
  let mut err: DWORD;
  let mut si: SECURITY_INFORMATION = OWNER_SECURITY_INFORMATION         // C 5171
    | GROUP_SECURITY_INFORMATION  /* | DACL_SECURITY_INFORMATION */ ;   // C 5172

  /* C leaves `sd_len` and `err` uninitialised (PORTING.md 1.5).  */
  sd_len = 0;
  err = 0;

  if get_file_security (fname, si, psd, 0, (&raw mut sd_len)) == 0      // C 5174
    {
      err = GetLastError ();                                            // C 5176
      if err != ERROR_INSUFFICIENT_BUFFER                               // C 5177
	{ return std::ptr::null_mut (); }                                      // C 5178
    }

  psd = xmalloc (sd_len as usize) as PSECURITY_DESCRIPTOR;              // C 5181 (C's `xmalloc (sd_len)`)
  if get_file_security (fname, si, psd, sd_len, (&raw mut sd_len)) == 0 // C 5182
    {
      xfree (psd);                                                      // C 5184
      return std::ptr::null_mut ();                                     // C 5185
    }

  return psd;                                                           // C 5188
}
/* C 5191-5202: C 5191's `static DWORD` and `get_rid`.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_rid (sid: PSID) -> DWORD                                  // C 5191
{
  let mut n_subauthorities: c_uint;                                     // C 5194

  /* C leaves the local uninitialised (PORTING.md 1.5).  */
  n_subauthorities = 0;

  /* Use the last sub-authority value of the RID, the relative          // C 5196
     portion of the SID, as user/group ID. */                           // C 5197
  n_subauthorities = *get_sid_sub_authority_count (sid) as c_uint;      // C 5198 (C's `unsigned char`-into-`unsigned`)
  if n_subauthorities < 1                                               // C 5199
    { return 0; } /* the "World" RID */                                 // C 5200
  return *get_sid_sub_authority (sid, n_subauthorities - 1);            // C 5201
}

/* C 5203-5204: C 5203's blank and C 5204's `/* Caching SID and account values for faster
   lokup.  */` (the next item is C 5206's `struct w32_id`, past this slice).  */
/* C 5203-5213: C 5203's blank, C 5204's `/* Caching SID and account values for faster lokup.  */`,
   C 5206-5211's `struct w32_id` (whose `sid` member is C's `FLEXIBLE_ARRAY_MEMBER` array, the
   `comp.rs` 884 device) and C 5213's tentative pointer.  */
/// C: struct w32_id { unsigned rid; struct w32_id *next; char name[GNLEN+1];
/// C:   unsigned char sid[FLEXIBLE_ARRAY_MEMBER]; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct w32_id
{
  pub rid: c_uint,
  pub next: *mut w32_id,
  pub name: [c_char; GNLEN + 1],
  pub sid: [c_uchar; 0],                   /* FLEXIBLE_ARRAY_MEMBER */
}
#[cfg (feature = "windowsnt")]
static mut w32_idlist: *mut w32_id = std::ptr::null_mut ();  // C 5213 (tentative, i.e. NULL) // C 5213
/* C 5215-5236: C 5215's `static int` and `w32_cached_id` (C 5220's two-initialiser `for` with its
   `break` is the port's `while` plus the loop-carried assignments, C 5222's `(PSID)tail->sid`
   cast being the flexible array's address).  */
#[cfg (feature = "windowsnt")]
unsafe fn w32_cached_id (sid: PSID, id: *mut c_uint, name: *mut c_char) -> c_int // C 5215
{
  let mut tail: *mut w32_id;                                            // C 5218
  let mut found: *mut w32_id;

  /* C leaves `tail` and `found` uninitialised (PORTING.md 1.5).  */
  tail = std::ptr::null_mut ();
  found = std::ptr::null_mut ();

  found = std::ptr::null_mut ();                                        // C 5220
  tail = w32_idlist;                                                    // C 5220
  while ! tail.is_null ()                                               // C 5220
    {
      if equal_sid ((&raw mut (*tail).sid) as PSID, sid) != 0           // C 5222
	{
	  found = tail;                                                        // C 5224
	  break;                                                               // C 5225
	}
      tail = (*tail).next;                                              // C 5220 (the for's increment)
    }
  if ! found.is_null ()                                                 // C 5228
    {
      *id = (*found).rid;                                               // C 5230
      strcpy (name, (&raw const (*found).name) as *const c_char);       // C 5231
      return 1;                                                         // C 5232
    }
  else                                                                  // C 5234
    { return 0; }                                                       // C 5235
}
/* C 5238-5259: C 5238's `static void` and `w32_add_to_cache` (C 5249's `offsetof` is the port's
   `core::mem::offset_of!`).  */
#[cfg (feature = "windowsnt")]
unsafe fn w32_add_to_cache (sid: PSID, id: c_uint, name: *mut c_char)   // C 5238
{
  let mut sid_len: DWORD;                                               // C 5241
  let mut new_entry: *mut w32_id;                                       // C 5242

  /* C leaves the two locals uninitialised (PORTING.md 1.5).  */
  sid_len = 0;
  new_entry = std::ptr::null_mut ();

  /* We don't want to leave behind stale cache from when Emacs was      // C 5244
     dumped.  */                                                        // C 5245
  if initialized != 0                                                   // C 5246
    {
      sid_len = get_length_sid (sid);                                   // C 5248
      new_entry = xmalloc (core::mem::offset_of! (w32_id, sid) + sid_len as usize) as *mut w32_id; // C 5249 (C's `offsetof (struct w32_id, sid) + sid_len`)
      if ! new_entry.is_null ()                                         // C 5250
	{
	  (*new_entry).rid = id;                                               // C 5252
	  strcpy ((&raw mut (*new_entry).name) as *mut c_char, name);          // C 5253
	  copy_sid (sid_len, (&raw mut (*new_entry).sid) as PSID, sid);        // C 5254
	  (*new_entry).next = w32_idlist;                                      // C 5255
	  w32_idlist = new_entry;                                              // C 5256
	}
    }
}
/* C 5261-5262: the two object-like macros the next function switches on.  */
/// C: #define UID 1
#[cfg (feature = "windowsnt")]
pub const UID: c_int = 1;
/// C: #define GID 2
#[cfg (feature = "windowsnt")]
pub const GID: c_int = 2;
/* C 5264-5300: C 5264's `static int` and `get_name_and_id`.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_name_and_id (psd: PSECURITY_DESCRIPTOR, id: *mut c_uint, nm: *mut c_char, // C 5264
			what: c_int)
  -> c_int
{
  let mut sid: PSID = std::ptr::null_mut ();                            // C 5267
  let mut dflt: BOOL;                                                   // C 5268
  let mut ignore: SID_NAME_USE = std::mem::zeroed ();                   // C 5269
  let mut name: [c_char; UNLEN + 1] = [0; UNLEN + 1]; // C 5270
  let mut name_len: DWORD = core::mem::size_of_val (&name) as DWORD;    // C 5271
  let mut domain: [c_char; 1024] = [0; 1024];                           // C 5272
  let mut domain_len: DWORD = core::mem::size_of_val (&domain) as DWORD; // C 5273
  let mut use_dflt: c_int = 0;                                          // C 5274
  let mut result: c_int;                                                // C 5275

  /* C leaves `dflt` and `result` uninitialised (PORTING.md 1.5).  */
  dflt = 0;
  result = 0;

  if what == UID                                                        // C 5277
    { result = get_security_descriptor_owner (psd, (&raw mut sid), (&raw mut dflt)); } // C 5278
  else if what == GID                                                   // C 5279
    { result = get_security_descriptor_group (psd, (&raw mut sid), (&raw mut dflt)); } // C 5280
  else
    { result = 0; }                                                     // C 5282

  if (result == 0 || is_valid_sid (sid) == 0)                           // C 5284
    { use_dflt = 1; }                                                   // C 5285
  else if w32_cached_id (sid, id, nm) == 0                              // C 5286
    {
      if (lookup_account_sid (std::ptr::null (), sid, name.as_mut_ptr (), (&raw mut name_len), // C 5288
			   domain.as_mut_ptr (), (&raw mut domain_len), (&raw mut ignore)) == 0 // C 5289
	  || name_len > (UNLEN + 1) as DWORD)                                    // C 5290
	{ use_dflt = 1; }                                                      // C 5291
	      else
	{
	  *id = get_rid (sid);                                                 // C 5294
	  strcpy (nm, name.as_ptr ());                                         // C 5295
	  w32_add_to_cache (sid, *id, name.as_mut_ptr ());                     // C 5296
	}
    }
  return use_dflt;                                                      // C 5299
}
/* C 5302-5331: C 5302's `static void` and `get_file_owner_and_group`.  */
#[cfg (feature = "windowsnt")]
unsafe fn get_file_owner_and_group (psd: PSECURITY_DESCRIPTOR, st: *mut stat) // C 5302
{
  let mut dflt_usr: c_int = 0;                                          // C 5305
  let mut dflt_grp: c_int = 0;

  if psd.is_null ()                                                     // C 5307
    {
      dflt_usr = 1;                                                     // C 5309
      dflt_grp = 1;                                                     // C 5310
    }
  else
    {
      if get_name_and_id (psd, (&raw mut (*st).st_uid), ((&raw mut (*st).st_uname) as *mut c_char), UID) != 0 // C 5314
	{ dflt_usr = 1; }                                                      // C 5315
      if get_name_and_id (psd, (&raw mut (*st).st_gid), ((&raw mut (*st).st_gname) as *mut c_char), GID) != 0 // C 5316
	{ dflt_grp = 1; }                                                      // C 5317
    }
  /* Consider files to belong to current user/group, if we cannot get   // C 5319
     more accurate information.  */                                     // C 5320
  if dflt_usr != 0                                                      // C 5321
    {
      (*st).st_uid = dflt_passwd.pw_uid;                             // C 5323
      strcpy (((&raw mut (*st).st_uname) as *mut c_char), dflt_passwd.pw_name); // C 5324
    }
  if dflt_grp != 0                                                      // C 5326
    {
      (*st).st_gid = dflt_passwd.pw_gid;                             // C 5328
      strcpy (((&raw mut (*st).st_gname) as *mut c_char), dflt_group.gr_name); // C 5329
    }
}

/* C 5333-5334: C 5333's `/* Return non-zero if NAME is a potentially slow filesystem.  */` and
   C 5334's forward declaration (the next item is C 5336's `int`/`is_slow_fs`, past this
   slice).  */
/* C 5332-5336: C 5332's blank, C 5333's `/* Return non-zero if NAME is a potentially slow
   filesystem.  */`, C 5334's prototype (`int is_slow_fs (const char *);` - the C 800-812
   forward-declaration block device: the C line is carried here, the port needs no item) and
   C 5336's `int`.  */
/* C 5336-5355: `is_slow_fs`.  The two `<winbase.h>` drive types it first names are declared here
   (P-88, `UNVERIFIED`).  */
/// C: #define DRIVE_FIXED 3
#[cfg (feature = "windowsnt")]
pub const DRIVE_FIXED: UINT = 3;
/// C: #define DRIVE_RAMDISK 6
#[cfg (feature = "windowsnt")]
pub const DRIVE_RAMDISK: UINT = 6;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_slow_fs (name: *const c_char) -> c_int      // C 5336
{
  let mut drive_root: [c_char; 4] = [0; 4];                             // C 5339
  let mut devtype: UINT;                                                // C 5340

  /* C leaves the local uninitialised (PORTING.md 1.5).  */
  devtype = 0;

  if IS_DIRECTORY_SEP! (*name as c_int) && IS_DIRECTORY_SEP! (*name.offset (1) as c_int) // C 5342
    { devtype = DRIVE_REMOTE; }   /* assume UNC name is remote */       // C 5343
  else if ! (strlen (name) >= 2 && IS_DEVICE_SEP! (*name.offset (1) as c_int)) // C 5344
    { devtype = GetDriveType (std::ptr::null ()); } /* use root of current drive */ // C 5345
  else
    {
      /* GetDriveType needs the root directory of the drive.  */        // C 5348
      strncpy ((&raw mut drive_root) as *mut c_char, name, 2);          // C 5349
      drive_root[2] = b'\\' as c_char;                                  // C 5350
      drive_root[3] = 0;                                                // C 5351 (C's `'\\0'`)
      devtype = GetDriveType ((&raw mut drive_root) as *const c_char);  // C 5352
    }
  return (! (devtype == DRIVE_FIXED || devtype == DRIVE_RAMDISK)) as c_int; // C 5354
}
/* C 5357-5360: C 5357-5359's three-line comment and C 5360's tentative
   `int w32_stat_get_owner_group;` - a non-`static` definition, so `#[unsafe(no_mangle)]`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut w32_stat_get_owner_group: c_int = 0;   // C 5360 (tentative, i.e. zero) // C 5360

/* C 5362-5365: C 5362-5365's four-line comment.  The next item is C 5367's `static int` and
   `stat_worker` (C 5368-5780, 413 C lines) - the next slices carry it; its C 5405 uses **bare
   `alloca`**, whose port device is `sa_stack_alloc` (P-05, landed in `alloc.rs` on 2026-10-04;
   the `haikuterm.rs` 4947 / `editfns.rs` 2477 shape), and its C 5396/5402 use `strpbrk`/`strlen`
   (both reachable).  */

/* C 5362-5368: C 5362-5365's four-line comment (carried verbatim below), C 5366's blank,
   C 5367's `static int` and C 5368's signature.  The records and calls this slice first names
   (`BY_HANDLE_FILE_INFORMATION`, `GetFileInformationByHandle`, `GetFileType` and its four type
   values, `LocalFree`, `ERROR_INVALID_FUNCTION`, `FILE_FLAG_OPEN_REPARSE_POINT`, `READ_CONTROL`,
   `S_IREAD`, `S_IWRITE`, `S_IEXEC`) are declared here (P-88, `UNVERIFIED`); C 5405's **bare
   `alloca`** is P-05's landed device, `sa_stack_alloc` (the `haikuterm.rs` 4947 / `editfns.rs`
   2477 shape).  **Control flow**: C 5467's `goto no_true_file_attributes` jumps into the `else`
   arm's first statement and C 5575's fall-through leaves the if/else for the C 5735 tail, so
   both arms sit in the labelled block `'plan_b`: C 5466's test is written inverted, and
   `break 'plan_b` at C 5575 stands in for the fall-through (`## Deviations`).  */
/* MSVC stat function can't cope with UNC names and has other bugs, so  // C 5362
   replace it with our own.  This also allows us to calculate consistent // C 5363
   inode values and owner/group without hacks in the main Emacs code,   // C 5364
   and support file names encoded in UTF-8. */                          // C 5365

/// C: typedef struct _BY_HANDLE_FILE_INFORMATION { DWORD dwFileAttributes;
/// C:   FILETIME ftCreationTime; FILETIME ftLastAccessTime; FILETIME ftLastWriteTime;
/// C:   DWORD dwVolumeSerialNumber; DWORD nFileSizeHigh; DWORD nFileSizeLow;
/// C:   DWORD nNumberOfLinks; DWORD nFileIndexHigh; DWORD nFileIndexLow; } BY_HANDLE_FILE_INFORMATION;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
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
/// C: WINBASEAPI BOOL WINAPI GetFileInformationByHandle (HANDLE hFile,
/// C:   LPBY_HANDLE_FILE_INFORMATION lpFileInformation);
/// C: WINBASEAPI DWORD WINAPI GetFileType (HANDLE hFile);
/// C: HLOCAL WINAPI LocalFree (HLOCAL hMem);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn GetFileInformationByHandle (hFile: HANDLE,
				     lpFileInformation: *mut BY_HANDLE_FILE_INFORMATION) -> BOOL;
  pub fn GetFileType (hFile: HANDLE) -> DWORD;
  pub fn LocalFree (hMem: *mut c_void) -> *mut c_void;
}
/// C: #define FILE_TYPE_UNKNOWN 0x0000
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_UNKNOWN: DWORD = 0x0000;
/// C: #define FILE_TYPE_DISK 0x0001
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_DISK: DWORD = 0x0001;
/// C: #define FILE_TYPE_CHAR 0x0002
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_CHAR: DWORD = 0x0002;
/// C: #define FILE_TYPE_PIPE 0x0003
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_PIPE: DWORD = 0x0003;
/// C: #define ERROR_INVALID_FUNCTION 1L (`<winerror.h>`; C 5496's comment is where the C text
/// names it)
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_FUNCTION: DWORD = 1;
/// C: #define FILE_FLAG_OPEN_REPARSE_POINT 0x00200000 (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const FILE_FLAG_OPEN_REPARSE_POINT: DWORD = 0x00200000;
/// C: #define READ_CONTROL 0x00020000 (`<winnt.h>`, a standard access right)
#[cfg (feature = "windowsnt")]
pub const READ_CONTROL: DWORD = 0x00020000;
/// C: #define S_IREAD 0x0100 (`<sys/stat.h>`; `nt/inc/sys/include/stat.rs` 44 mirrors it, and
/// C:   C 5762/5766/5768 name the three)
#[cfg (feature = "windowsnt")]
pub const S_IREAD: c_int = 0x0100;
/// C: #define S_IWRITE 0x0080
#[cfg (feature = "windowsnt")]
pub const S_IWRITE: c_int = 0x0080;
/// C: #define S_IEXEC 0x0040
#[cfg (feature = "windowsnt")]
pub const S_IEXEC: c_int = 0x0040;

#[cfg (feature = "windowsnt")]
unsafe fn stat_worker (mut path: *const c_char, buf: *mut stat, follow_symlinks: c_int) -> c_int // C 5367
{
  let mut name: *mut c_char;                                            // C 5370
  let mut save_name: *mut c_char;
  let mut r: *mut c_char;
  let mut wfd_w: WIN32_FIND_DATAW;                                      // C 5371
  let mut wfd_a: WIN32_FIND_DATAA;                                      // C 5372
  let mut fh: HANDLE;                                                   // C 5373
  let mut fake_inode: c_ulonglong = 0;                                  // C 5374 (C's `unsigned __int64`)
  let mut permission: c_int;                                            // C 5375
  let mut len: c_int;                                                   // C 5376
  let mut rootdir: c_int = FALSE;                                       // C 5377
  let mut psd: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();            // C 5378
  let mut is_a_symlink: c_int = 0;                                      // C 5379
  let mut file_flags: DWORD = FILE_FLAG_BACKUP_SEMANTICS;               // C 5380
  let mut access_rights: DWORD = 0;                                     // C 5381
  let mut fattrs: DWORD = 0;                                            // C 5382
  let mut serialnum: DWORD = 0;
  let mut fs_high: DWORD = 0;
  let mut fs_low: DWORD = 0;
  let mut nlinks: DWORD = 1;
  let mut ctime: FILETIME;                                              // C 5383
  let mut atime: FILETIME;
  let mut wtime: FILETIME;
  let mut name_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5384
  let mut name_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 5385

  /* C 5370-5376 and C 5383 leave `name`, `save_name`, `r`, `wfd_w`, `wfd_a`, `fh`, `permission`,
     `len`, `ctime`, `atime` and `wtime` uninitialised (PORTING.md 1.5).  */
  name = std::ptr::null_mut ();
  save_name = std::ptr::null_mut ();
  r = std::ptr::null_mut ();
  wfd_w = std::mem::zeroed ();
  wfd_a = std::mem::zeroed ();
  fh = std::ptr::null_mut ();
  permission = 0;
  len = 0;
  ctime = std::mem::zeroed ();
  atime = std::mem::zeroed ();
  wtime = std::mem::zeroed ();

  if path.is_null () || buf.is_null ()                                  // C 5387
    {
      *errno () = EFAULT;                                               // C 5389
      return -1;                                                        // C 5390
    }

  save_name = map_w32_filename (path, (&raw mut path));                 // C 5393 (C's `(char *)` cast and `&path`)
  name = save_name;
  /* Must be valid filename, no wild cards or other invalid             // C 5394
     characters.  */                                                    // C 5395
  if ! strpbrk (name, c"*?|<>\"").is_null ()                            // C 5396
    {
      *errno () = ENOENT;                                               // C 5398
      return -1;                                                        // C 5399
    }

  len = strlen (name) as c_int;                                         // C 5402
  /* Allocate 1 extra byte so that we could append a slash to a root    // C 5403
     directory, down below.  */                                         // C 5404
  name = strcpy (sa_stack_alloc ((len + 2) as usize) as *mut c_char, name); // C 5405 (C's bare `alloca`; P-05's device, `sa_stack_alloc`)

  /* Avoid a somewhat costly call to is_symlink if the filesystem       // C 5407
     doesn't support symlinks.  */                                      // C 5408
  if (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) != 0            // C 5409
    { is_a_symlink = is_symlink (name); }                               // C 5410

  /* Plan A: Open the file and get all the necessary information via    // C 5412
     the resulting handle.  This solves several issues in one blow:     // C 5413
                                                                        // C 5414
      . retrieves attributes for the target of a symlink, if needed     // C 5415
      . gets attributes of root directories and symlinks pointing to    // C 5416
        root directories, thus avoiding the need for special-casing     // C 5417
        these and detecting them by examining the file-name format      // C 5418
      . retrieves more accurate attributes (e.g., non-zero size for     // C 5419
        some directories, esp. directories that are junction points)    // C 5420
      . correctly resolves "c:/..", "/.." and similar file names        // C 5421
      . avoids run-time penalties for 99% of use cases                  // C 5422
                                                                        // C 5423
     Plan A is always tried first, unless the user asked not to (but    // C 5424
     if the file is a symlink and we need to follow links, we try Plan  // C 5425
     A even if the user asked not to).                                  // C 5426
                                                                        // C 5427
     If Plan A fails, we go to Plan B (below), where various            // C 5428
     potentially expensive techniques must be used to handle "special"  // C 5429
     files such as UNC volumes etc.  */                                 // C 5430
  /* C 5467's `goto no_true_file_attributes` and C 5575's fall-through: the port writes both arms
     in the labelled block `'plan_b` (the `androidselect.rs` 414 `'fail` / PORTING.md 2.2 device).  */
  'plan_b: {                                                            // C 5431-5577 (the device)
  if (! (NILP (Vw32_get_true_file_attributes)                           // C 5431
	 || (EQ (Vw32_get_true_file_attributes, Qlocal ()) && is_slow_fs (name) != 0)) // C 5432
	 /* Following symlinks requires getting the info by handle.  */        // C 5433
	 || (is_a_symlink != 0 && follow_symlinks != 0))                       // C 5434
    {
      let mut info: BY_HANDLE_FILE_INFORMATION;                         // C 5436

      /* C leaves it uninitialised (PORTING.md 1.5).  */
      info = std::mem::zeroed ();

      if is_a_symlink != 0 && follow_symlinks == 0                      // C 5438
	{ file_flags |= FILE_FLAG_OPEN_REPARSE_POINT; }                        // C 5439
      /* READ_CONTROL access rights are required to get security info   // C 5440
	 by handle.  But if the OS doesn't support security in the             // C 5441
	 first place, we don't need to try.  */                                // C 5442
      if is_windows_9x () != TRUE                                       // C 5443 (C's `TRUE`)
	{ access_rights |= READ_CONTROL; }                                     // C 5444

      if w32_unicode_filenames                                          // C 5446
	{
	  filename_to_utf16 (name, name_w.as_mut_ptr ());                      // C 5448
	  fh = CreateFileW (name_w.as_ptr (), access_rights, 0, std::ptr::null_mut (), // C 5449
			   OPEN_EXISTING, file_flags, std::ptr::null_mut ());                // C 5450
	  /* If CreateFile fails with READ_CONTROL, try again with             // C 5451
	     zero as access rights.  */                                        // C 5452
	  if fh == INVALID_HANDLE_VALUE && access_rights != 0                  // C 5453
	    { fh = CreateFileW (name_w.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 5454
			       file_flags, std::ptr::null_mut ()); }                         // C 5455
	}
      else                                                              // C 5457
	{
	  filename_to_ansi (name, name_a.as_mut_ptr ());                       // C 5459
	  fh = CreateFileA (name_a.as_ptr (), access_rights, 0, std::ptr::null_mut (), // C 5460
			   OPEN_EXISTING, file_flags, std::ptr::null_mut ());                // C 5461
	  if fh == INVALID_HANDLE_VALUE && access_rights != 0                  // C 5462
	    { fh = CreateFileA (name_a.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 5463
			       file_flags, std::ptr::null_mut ()); }                         // C 5464
	}
      if fh != INVALID_HANDLE_VALUE                                     // C 5466-5467 (C's test inverted; the `goto` is the fall into Plan B)
	{
      /* This is more accurate in terms of getting the correct number   // C 5469
	 of links, but is quite slow (it is noticeable when Emacs is           // C 5470
	 making a list of file name completions). */                           // C 5471
	  if GetFileInformationByHandle (fh, (&raw mut info)) != 0             // C 5472
	    {
	      nlinks = info.nNumberOfLinks;                                    // C 5474
	  /* Might as well use file index to fake inode values, but this       // C 5475
	     is not guaranteed to be unique unless we keep a handle open       // C 5476
	     all the time (even then there are situations where it is          // C 5477
	     not unique).  Reputedly, there are at most 48 bits of info        // C 5478
	     (on NTFS, presumably less on FAT). */                             // C 5479
	      fake_inode = info.nFileIndexHigh as c_ulonglong;                 // C 5480
	      fake_inode <<= 32;                                               // C 5481
	      fake_inode += info.nFileIndexLow as c_ulonglong;                 // C 5482
	      serialnum = info.dwVolumeSerialNumber;                           // C 5483
	      fs_high = info.nFileSizeHigh;                                    // C 5484
	      fs_low  = info.nFileSizeLow;                                     // C 5485
	      ctime = info.ftCreationTime;                                     // C 5486
	      atime = info.ftLastAccessTime;                                   // C 5487
	      wtime = info.ftLastWriteTime;                                    // C 5488
	      fattrs = info.dwFileAttributes;                                  // C 5489
	    }
	  else                                                                 // C 5491
	    {
	  /* We don't go to Plan B here, because it's not clear that           // C 5493
	     it's a good idea.  The only known use case where                  // C 5494
	     CreateFile succeeds, but GetFileInformationByHandle fails         // C 5495
	     (with ERROR_INVALID_FUNCTION) is for character devices            // C 5496
	     such as NUL, PRN, etc.  For these, switching to Plan B is         // C 5497
	     a net loss, because we lose the character device                  // C 5498
	     attribute returned by GetFileType below (FindFirstFile            // C 5499
	     doesn't set that bit in the attributes), and the other            // C 5500
	     fields don't make sense for character devices anyway.             // C 5501
	     Emacs doesn't really care for non-file entities in the            // C 5502
	     context of l?stat, so neither do we.  */                          // C 5503
	      /* w32err is assigned so one could put a breakpoint here and     // C 5505
		 examine its value, when GetFileInformationByHandle                   // C 5506
		 fails. */                                                            // C 5507
	      let mut w32err: DWORD = GetLastError ();                         // C 5508

	      match w32err                                                     // C 5510 (C's `switch`; ARMDEVICE: the unmatched cases fall out)
		{
		  ERROR_FILE_NOT_FOUND => /* can this ever happen? */                 // C 5512
		    {
		      *errno () = ENOENT;                                             // C 5513
		      return -1;                                                      // C 5514
		    }
		  _ => {}
		}
	    }

      /* Test for a symlink before testing for a directory, since       // C 5518
	 symlinks to directories have the directory bit set, but we            // C 5519
	 don't want them to appear as directories.  */                         // C 5520
	  if is_a_symlink != 0 && follow_symlinks == 0                         // C 5521
	    { (*buf).st_mode = S_IFLNK as c_ushort; }                          // C 5522
	  else if (fattrs & FILE_ATTRIBUTE_DIRECTORY) != 0                     // C 5523
	    { (*buf).st_mode = S_IFDIR as c_ushort; }                          // C 5524
	  else                                                                 // C 5525
	    {
	      let mut ftype: DWORD = GetFileType (fh);                         // C 5527

	      match ftype                                                      // C 5529 (C's `switch`; C 5533/5536 `break`s are arm ends)
		{
		  FILE_TYPE_DISK =>                                                   // C 5531
		    { (*buf).st_mode = S_IFREG as c_ushort; }                         // C 5532
		  FILE_TYPE_PIPE =>                                                   // C 5534
		    { (*buf).st_mode = S_IFIFO as c_ushort; }                         // C 5535
		  FILE_TYPE_CHAR                                                      // C 5537
		  | FILE_TYPE_UNKNOWN                                                 // C 5538
		  | _ => /* default: */                                               // C 5539
		    { (*buf).st_mode = S_IFCHR as c_ushort; }                         // C 5540
		}
	    }
      /* We produce the fallback owner and group data, based on the     // C 5543
	 current user that runs Emacs, in the following cases:                 // C 5544
                                                                        // C 5545
	  . caller didn't request owner and group info                         // C 5546
	  . this is Windows 9X                                                 // C 5547
	  . getting security by handle failed, and we need to produce          // C 5548
	    information for the target of a symlink (this is better            // C 5549
	    than producing a potentially misleading info about the             // C 5550
	    symlink itself)                                                    // C 5551
                                                                        // C 5552
	 If getting security by handle fails, and we don't need to             // C 5553
	 resolve symlinks, we try getting security by name.  */                // C 5554
	  if w32_stat_get_owner_group == 0 || is_windows_9x () == TRUE         // C 5555 (C's `TRUE`)
	    { get_file_owner_and_group (std::ptr::null_mut (), buf); }         // C 5556
	  else                                                                 // C 5557
	    {
	      psd = get_file_security_desc_by_handle (fh);                     // C 5559
	      if ! psd.is_null ()                                              // C 5560
		{
		  get_file_owner_and_group (psd, buf);                                // C 5562
		  LocalFree (psd);                                                    // C 5563
		}
	      else if ! (is_a_symlink != 0 && follow_symlinks != 0)            // C 5565
		{
		  psd = get_file_security_desc_by_name (name);                        // C 5567
		  get_file_owner_and_group (psd, buf);                                // C 5568
		  xfree (psd);                                                        // C 5569
		}
	      else                                                             // C 5571
		{ get_file_owner_and_group (std::ptr::null_mut (), buf); }            // C 5572
	    }
	  CloseHandle (fh);                                                    // C 5574
	}   /* the device's brace: C's `if (fh == INVALID_HANDLE_VALUE) goto` has none.  */
	break 'plan_b;                                                         // C 5575 (C's fall-through out of the if/else, to the C 5735 tail)
    }                                                                   // C 5575

  /* no_true_file_attributes: Plan B.  */                               // C 5577-5578 (C's `else` and its label)
      /* Plan B: Either getting a handle on the file failed, or the     // C 5579
	 caller explicitly asked us to not bother making this                  // C 5580
	 information more accurate.                                            // C 5581
                                                                        // C 5582
	 Implementation note: In Plan B, we never bother to resolve            // C 5583
	 symlinks, even if we got here because we tried Plan A and             // C 5584
	 failed.  That's because, even if the caller asked for extra           // C 5585
	 precision by setting Vw32_get_true_file_attributes to t,              // C 5586
	 resolving symlinks requires acquiring a file handle to the            // C 5587
	 symlink, which we already know will fail.  And if the user            // C 5588
	 did not ask for extra precision, resolving symlinks will fly          // C 5589
	 in the face of that request, since the user then wants the            // C 5590
	 lightweight version of the code.  */                                  // C 5591
  rootdir = (path >= save_name.offset (len as isize - 1)                // C 5592
	     && (IS_DIRECTORY_SEP! (*path as c_int) || *path == 0)) as c_int;  // C 5593 (C's `*path == 0`)

      /* If name is "c:/.." or "/.." then stat "c:/" or "/".  */        // C 5595
  r = if IS_DEVICE_SEP! (*name.offset (1) as c_int) { name.offset (2) } else { name }; // C 5596 (C's ternary)
  if (IS_DIRECTORY_SEP! (*r as c_int)                                   // C 5597
	  && *r.offset (1) == b'.' as c_char && *r.offset (2) == b'.' as c_char // C 5598
	  && *r.offset (3) == 0)                                               // C 5598 (C's `r[3] == '\0'`)
	{ *r.offset (1) = 0; *r.offset (2) = 0; }                              // C 5599

      /* Note: If NAME is a symlink to the root of a UNC volume         // C 5601
	 (i.e. "\\SERVER"), we will not detect that here, and we will          // C 5602
	 return data about the symlink as result of FindFirst below.           // C 5603
	 This is unfortunate, but that marginal use case does not              // C 5604
	 justify a call to chase_symlinks which would impose a penalty         // C 5605
	 on all the other use cases.  (We get here for symlinks to             // C 5606
	 roots of UNC volumes because CreateFile above fails for them,         // C 5607
	 unlike with symlinks to root directories X:\ of drives.)  */          // C 5608
  if is_unc_volume (name) != 0                                          // C 5609
    {
      fattrs = unc_volume_file_attributes (name);                       // C 5611
      if fattrs == -1 as DWORD                                          // C 5612 (C's `-1` against the DWORD)
	{ return -1; }                                                         // C 5613

      ctime = utc_base_ft; atime = utc_base_ft; wtime = utc_base_ft;    // C 5615 (C's chain)
    }
  else if rootdir != 0                                                  // C 5617
    {
	  /* Make sure root directories end in a slash.  */                    // C 5619
      if ! IS_DIRECTORY_SEP! (*name.offset (len as isize - 1) as c_int) // C 5620
	{ strcpy (name.offset (len as isize), c"\\".as_ptr ()); }              // C 5621 (C's `strcpy (name + len, "\\")`)
      if GetDriveType (name) < 2                                        // C 5622
	{
	  *errno () = ENOENT;                                                  // C 5624
	  return -1;                                                           // C 5625
	}

      fattrs = FILE_ATTRIBUTE_DIRECTORY;                                // C 5628
      ctime = utc_base_ft; atime = utc_base_ft; wtime = utc_base_ft;    // C 5629 (C's chain)
    }
  else                                                                  // C 5631
    {
      let mut have_wfd: c_int = -1;                                     // C 5633

	  /* Make sure non-root directories do NOT end in a slash,             // C 5635
	     otherwise FindFirstFile might fail.  */                           // C 5636
      if IS_DIRECTORY_SEP! (*name.offset (len as isize - 1) as c_int)   // C 5637
	{ *name.offset (len as isize - 1) = 0; }                               // C 5638

	  /* (This is hacky, but helps when doing file completions on          // C 5640
	     network drives.)  Optimize by using information available from    // C 5641
	     active readdir if possible.  */                                   // C 5642
      len = strlen (dir_pathname) as c_int;                             // C 5643
      if IS_DIRECTORY_SEP! (*dir_pathname.offset (len as isize - 1) as c_int) // C 5644
	{ len -= 1; }
      if (dir_find_handle != INVALID_HANDLE_VALUE                       // C 5646
	  && last_dir_find_data != -1                                          // C 5647
	  && ! (is_a_symlink != 0 && follow_symlinks != 0)                     // C 5648
	      /* The 2 file-name comparisons below support only ASCII          // C 5649
		 characters, and will lose (compare not equal) when                   // C 5650
		 the file names include non-ASCII characters that are                 // C 5651
		 the same but for the case.  However, doing this                      // C 5652
		 properly involves: (a) converting both file names to                 // C 5653
		 UTF-16, (b) lower-casing both names using CharLowerW,                // C 5654
		 and (c) comparing the results; this would be quite a                 // C 5655
		 bit slower, whereas Plan B is for users who want                     // C 5656
		 lightweight albeit inaccurate version of 'stat'.  */                 // C 5657
	  && c_strncasecmp (save_name, dir_pathname.as_ptr (), len as usize) == 0 // C 5658
	  && IS_DIRECTORY_SEP! (*name.offset (len as isize) as c_int)          // C 5659
	  && xstrcasecmp (name.offset (len as isize + 1), (&raw mut dir_static.d_name) as *const c_char) == 0) // C 5660
	{
	  have_wfd = last_dir_find_data;                                       // C 5662
	  /* This was the last entry returned by readdir.  */                  // C 5663
	  if last_dir_find_data == DIR_FIND_DATA_W                             // C 5664
	    { wfd_w = dir_find_data_w; }                                       // C 5665
	  else
	    { wfd_a = dir_find_data_a; }                                       // C 5667
	}
	      else                                                             // C 5669
	{
	  logon_network_drive (name);                                          // C 5671

	  if w32_unicode_filenames                                             // C 5673
	    {
	      filename_to_utf16 (name, name_w.as_mut_ptr ());                  // C 5675
	      fh = FindFirstFileW (name_w.as_ptr (), (&raw mut wfd_w));        // C 5676
	      have_wfd = DIR_FIND_DATA_W;                                      // C 5677
	    }
	  else
	    {
	      filename_to_ansi (name, name_a.as_mut_ptr ());                   // C 5681
		  /* If NAME includes characters not representable by                 // C 5682
		     the current ANSI codepage, filename_to_ansi                      // C 5683
		     usually replaces them with a '?'.  We don't want                 // C 5684
		     to let FindFirstFileA interpret those as wildcards,              // C 5685
		     and "succeed", returning us data from some random                // C 5686
		     file in the same directory.  */                                  // C 5687
	      if ! _mbspbrk (name_a.as_ptr () as *const c_uchar, c"?".as_ptr () as *const c_uchar).is_null () // C 5688
		{ fh = INVALID_HANDLE_VALUE; }                                        // C 5689
	      else
		{ fh = FindFirstFileA (name_a.as_ptr (), (&raw mut wfd_a)); }         // C 5691
	      have_wfd = DIR_FIND_DATA_A;                                      // C 5692
	    }
	  if fh == INVALID_HANDLE_VALUE                                        // C 5694
	    {
	      *errno () = ENOENT;                                              // C 5696
	      return -1;                                                       // C 5697
	    }
	  FindClose (fh);                                                      // C 5699
	}
	  /* Note: if NAME is a symlink, the information we get from           // C 5701
	     FindFirstFile is for the symlink, not its target.  */             // C 5702
	  if have_wfd == DIR_FIND_DATA_W                                       // C 5703
	    {
	      fattrs = wfd_w.dwFileAttributes;                                 // C 5705
	      ctime = wfd_w.ftCreationTime;                                    // C 5706
	      atime = wfd_w.ftLastAccessTime;                                  // C 5707
	      wtime = wfd_w.ftLastWriteTime;                                   // C 5708
	      fs_high = wfd_w.nFileSizeHigh;                                   // C 5709
	      fs_low = wfd_w.nFileSizeLow;                                     // C 5710
	    }
	  else                                                                 // C 5712
	    {
	      fattrs = wfd_a.dwFileAttributes;                                 // C 5714
	      ctime = wfd_a.ftCreationTime;                                    // C 5715
	      atime = wfd_a.ftLastAccessTime;                                  // C 5716
	      wtime = wfd_a.ftLastWriteTime;                                   // C 5717
	      fs_high = wfd_a.nFileSizeHigh;                                   // C 5718
	      fs_low = wfd_a.nFileSizeLow;                                     // C 5719
	    }
	  fake_inode = 0;                                                      // C 5721
	  nlinks = 1;                                                          // C 5722
	  serialnum = volume_info.serialnum;                                   // C 5723
	}
      if is_a_symlink != 0 && follow_symlinks == 0                      // C 5725
	{ (*buf).st_mode = S_IFLNK as c_ushort; }                              // C 5726
      else if (fattrs & FILE_ATTRIBUTE_DIRECTORY) != 0                  // C 5727
	{ (*buf).st_mode = S_IFDIR as c_ushort; }                              // C 5728
      else                                                              // C 5729
	{ (*buf).st_mode = S_IFREG as c_ushort; }                              // C 5730

        get_file_owner_and_group (std::ptr::null_mut (), buf);          // C 5732
  }                                                                     // C 5733 (C's `}` for the Plan B `else`; here it closes the `'plan_b` device)

  (*buf).st_ino = fake_inode;                                           // C 5735

  (*buf).st_dev = serialnum as dev_t;                                   // C 5737
  (*buf).st_rdev = serialnum as dev_t;                                  // C 5738

  (*buf).st_size = fs_high as c_ulonglong;                              // C 5740 (C's `off_t`/`st_size`)
  (*buf).st_size <<= 32;                                                // C 5741
  (*buf).st_size += fs_low as c_ulonglong;                              // C 5742
  (*buf).st_nlink = nlinks as c_short;                                  // C 5743

  /* Convert timestamps to Unix format. */                              // C 5745
  (*buf).st_mtime = convert_time (wtime, (&raw mut (*buf).st_mtimensec)); // C 5746
  (*buf).st_atime = convert_time (atime, (&raw mut (*buf).st_atimensec)); // C 5747
  if (*buf).st_atime == 0                                               // C 5748
    {
      (*buf).st_atime = (*buf).st_mtime;                                // C 5750
      (*buf).st_atimensec = (*buf).st_mtimensec;                        // C 5751
    }
  (*buf).st_ctime = convert_time (ctime, (&raw mut (*buf).st_ctimensec)); // C 5753
  if (*buf).st_ctime == 0                                               // C 5754
    {
      (*buf).st_ctime = (*buf).st_mtime;                                // C 5756
      (*buf).st_ctimensec = (*buf).st_mtimensec;                        // C 5757
    }

  /* determine rwx permissions */                                       // C 5760
  if is_a_symlink != 0 && follow_symlinks == 0                          // C 5761
    { permission = S_IREAD | S_IWRITE | S_IEXEC; } /* Posix expectations */ // C 5762
  else                                                                  // C 5763
    {
      if (fattrs & FILE_ATTRIBUTE_READONLY) != 0                        // C 5765
	{ permission = S_IREAD; }                                              // C 5766
      else                                                              // C 5767
	{ permission = S_IREAD | S_IWRITE; }                                   // C 5768

      if (fattrs & FILE_ATTRIBUTE_DIRECTORY) != 0                       // C 5770
	{ permission |= S_IEXEC; }                                             // C 5771
      else if is_exec (name) != 0                                       // C 5772
	{ permission |= S_IEXEC; }                                             // C 5773
    }

  (*buf).st_mode |= (permission | (permission >> 3) | (permission >> 6)) as c_ushort; // C 5776

  return 0;                                                             // C 5778
}                                                                       // C 5779
/* C 5781-5824: C 5782's `stat`, C 5788's `lstat`, C 5794's `fstatat` and C 5826's `fstat`
   (with C 5823-5824's "Provide fstat and utimensat as well as stat for consistent handling of
   file timestamps." comment carried verbatim).  The names this slice first uses are declared
   here (P-88, `UNVERIFIED`): `<io.h>`'s `_get_osfhandle`/`_umask`, `<winbase.h>`'s
   `SetFileTime` with `FILE_TYPE_REMOTE`, `FILE_WRITE_ATTRIBUTES` and the three `FILE_SHARE_*`
   flags, `AT_EMPTY_PATH` (the `nt/inc/include/unistd.rs` 34 value; C 273's `"systime.h"` gives
   the unit `current_timespec`, and the `<fcntl.h>` mirror gives `AT_FDCWD`/
   `AT_SYMLINK_NOFOLLOW`), `ENFILE` (the CRT's errno), and `<winerror.h>`'s
   `ERROR_DEV_NOT_EXIST`/`ERROR_TOO_MANY_OPEN_FILES`/`ERROR_SHARING_VIOLATION`.  C 5833-5855's
   `switch`/`goto non_disk`/`non_disk:` label is the `lib-src/ntlib.rs` 1005/1029 device (the
   same C body), and C 6278's `#if 0` region is dead text.  */
/// C: intptr_t _get_osfhandle (int fd);  (`<io.h>`, the `sysdep.rs` 1063 spelling)
/// C: int _umask (int pmode);  (`<io.h>`)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _get_osfhandle (fd: c_int) -> isize;
  pub fn _umask (pmode: c_int) -> c_int;
}
/// C: WINBASEAPI BOOL WINAPI SetFileTime (HANDLE hFile, const FILETIME *lpCreationTime,
/// C:   const FILETIME *lpLastAccessTime, const FILETIME *lpLastWriteTime);
/// C: #define FILE_TYPE_REMOTE 0x8000 (`<winbase.h>`; the remote-drive bit)
/// C: #define FILE_WRITE_ATTRIBUTES 0x00000100 / #define FILE_SHARE_READ 0x00000001
/// C: #define FILE_SHARE_WRITE 0x00000002 / #define FILE_SHARE_DELETE 0x00000004 (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn SetFileTime (hFile: HANDLE, lpCreationTime: *const FILETIME,
		      lpLastAccessTime: *const FILETIME, lpLastWriteTime: *const FILETIME) -> BOOL;
}
#[cfg (feature = "windowsnt")]
pub const FILE_TYPE_REMOTE: DWORD = 0x8000;
#[cfg (feature = "windowsnt")]
pub const FILE_WRITE_ATTRIBUTES: DWORD = 0x00000100;
#[cfg (feature = "windowsnt")]
pub const FILE_SHARE_READ: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const FILE_SHARE_WRITE: DWORD = 0x00000002;
#[cfg (feature = "windowsnt")]
pub const FILE_SHARE_DELETE: DWORD = 0x00000004;
/// C: #define AT_EMPTY_PATH 8192 (`<fcntl.h>`; `nt/inc/include/unistd.rs` 34 mirrors the CRT's
/// C:   value, and C 5803 is where this unit names it)
#[cfg (feature = "windowsnt")]
pub const AT_EMPTY_PATH: c_int = 8192;
/// C: #define ENFILE 23 (`<errno.h>`; C 6070 names it against `sys_stat`'s `EINVAL`)
#[cfg (feature = "windowsnt")]
pub const ENFILE: c_int = 23;
/// C: #define ERROR_DEV_NOT_EXIST 55L
#[cfg (feature = "windowsnt")]
pub const ERROR_DEV_NOT_EXIST: DWORD = 55;
/// C: #define ERROR_TOO_MANY_OPEN_FILES 4L
#[cfg (feature = "windowsnt")]
pub const ERROR_TOO_MANY_OPEN_FILES: DWORD = 4;
/// C: #define ERROR_SHARING_VIOLATION 32L
#[cfg (feature = "windowsnt")]
pub const ERROR_SHARING_VIOLATION: DWORD = 32;

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stat (path: *const c_char, buf: *mut stat) -> c_int // C 5781
{                                                                       // C 5783
  return stat_worker (path, buf, 1);                                    // C 5784
}                                                                       // C 5785

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lstat (path: *const c_char, buf: *mut stat) -> c_int // C 5787
{                                                                       // C 5789
  return stat_worker (path, buf, 0);                                    // C 5790
}                                                                       // C 5791

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fstatat (fd: c_int, name: *const c_char, st: *mut stat, flags: c_int) -> c_int // C 5793
{                                                                       // C 5795
  /* Rely on a hack: an open directory is modeled as file descriptor 0. // C 5796
     This is good enough for the current usage in Emacs, but is fragile. // C 5797

     FIXME: Add proper support for fdopendir, fstatat, readlinkat.      // C 5799
     Gnulib does this and can serve as a model.  */                     // C 5800
  let mut fullname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 5801

  if (flags & AT_EMPTY_PATH) != 0 && name.is_null ()                    // C 5803 (C's parens)
    { name = c"".as_ptr (); }                                           // C 5804 (C's `name = "";`)

  if fd != AT_FDCWD                                                     // C 5806
    {                                                                   // C 5807
      let mut lastc: c_char = *dir_pathname.offset ((strlen (dir_pathname) - 1) as isize); // C 5808

      if (_snprintf ((&raw mut fullname) as *mut c_char,                // C 5810
		   core::mem::size_of_val (&fullname),
		   c"%s%s%s".as_ptr (),
		   dir_pathname.as_ptr (),                                            // C 5811
		   if IS_DIRECTORY_SEP! (lastc as c_int) { c"".as_ptr () } else { c"/".as_ptr () }, name) // C 5811
	  < 0)                                                                 // C 5812
	{                                                                      // C 5813
	  *errno () = ENAMETOOLONG;                                            // C 5814
	  return -1;                                                           // C 5815
	}                                                                      // C 5816
      name = (&raw mut fullname) as *const c_char;                      // C 5817 (C's array decay)
    }                                                                   // C 5818

  return stat_worker (name, st, (! ((flags & AT_SYMLINK_NOFOLLOW) != 0)) as c_int); // C 5820 (C's `!` + the `as`-binding parens)
}                                                                       // C 5821


/* Provide fstat and utimensat as well as stat for consistent handling  // C 5823
   of file timestamps. */                                               // C 5824
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fstat (desc: c_int, buf: *mut stat) -> c_int   // C 5825
{                                                                       // C 5827
  let mut fh: HANDLE = _get_osfhandle (desc) as HANDLE;                 // C 5828
  let mut info: BY_HANDLE_FILE_INFORMATION;                             // C 5829
  let mut fake_inode: c_ulonglong;                                      // C 5830 (C's `unsigned __int64`)
  let mut permission: c_int;                                            // C 5831

  /* C leaves `info`, `fake_inode` and `permission` uninitialised (PORTING.md 1.5).  */
  info = std::mem::zeroed ();
  fake_inode = 0;
  permission = 0;

  /* C 5833-5855's `switch` and its `goto non_disk` (C 5845): the switch is this `match`, its
     `break` (C 5842) is the end of the `FILE_TYPE_DISK` arm, and the `non_disk:` label's block
     is the `if non_disk` tail below (the `lib-src/ntlib.rs` 1005/1029 device, same C body).  */
  let mut non_disk: c_int = 0;
  match GetFileType (fh) & ! FILE_TYPE_REMOTE                           // C 5833
	{
	  FILE_TYPE_DISK =>                                                    // C 5835
	    {
	      (*buf).st_mode = S_IFREG as c_ushort;                            // C 5836
	      if GetFileInformationByHandle (fh, (&raw mut info)) == 0         // C 5837 (C's `!`)
		{
		  *errno () = EACCES;                                                 // C 5839
		  return -1;                                                          // C 5840
		}
	    }
	  FILE_TYPE_PIPE =>                                                    // C 5843
	    {
	      (*buf).st_mode = S_IFIFO as c_ushort;                            // C 5844
	      non_disk = 1;                              /* C 5845's `goto non_disk;` */ // C 5845
	    }
	  FILE_TYPE_CHAR                                                       // C 5846
	  | FILE_TYPE_UNKNOWN                                                  // C 5847
	  | _ =>                                                               // C 5848 (C's `default:`)
	    {
	      (*buf).st_mode = S_IFCHR as c_ushort;                            // C 5849
	      non_disk = 1;                              /* C 5850's `non_disk:` label */ // C 5850
	    }
	}
  if non_disk != 0
    {
      memset ((&raw mut info) as *mut c_void, 0, core::mem::size_of_val (&info)); // C 5851 (C's `sizeof (info)`)
      info.dwFileAttributes = 0;                                        // C 5852
      info.ftCreationTime = utc_base_ft;                                // C 5853
      info.ftLastAccessTime = utc_base_ft;                              // C 5854
      info.ftLastWriteTime = utc_base_ft;                               // C 5855
    }

  if (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0            // C 5858
      { (*buf).st_mode = S_IFDIR as c_ushort; }                         // C 5859

  (*buf).st_nlink = info.nNumberOfLinks as c_short;                     // C 5861 (the mirror's `st_nlink`)
  /* Might as well use file index to fake inode values, but this        // C 5862
     is not guaranteed to be unique unless we keep a handle open        // C 5863
     all the time (even then there are situations where it is           // C 5864
     not unique).  Reputedly, there are at most 48 bits of info         // C 5865
     (on NTFS, presumably less on FAT). */                              // C 5866
  fake_inode = info.nFileIndexHigh as c_ulonglong;                      // C 5867
  fake_inode <<= 32;                                                    // C 5868
  fake_inode += info.nFileIndexLow as c_ulonglong;                      // C 5869

  /* MSVC defines _ino_t to be short; other libc's might not.  */       // C 5871
  if core::mem::size_of_val (&(*buf).st_ino) == 2                       // C 5872 (C's `sizeof (buf->st_ino)`)
    { (*buf).st_ino = fake_inode ^ (fake_inode >> 16); }                // C 5873
  else                                                                  // C 5874
    { (*buf).st_ino = fake_inode; }                                     // C 5875


  /* If the caller so requested, get the true file owner and group.     // C 5877
     Otherwise, consider the file to belong to the current user.  */    // C 5878
  if w32_stat_get_owner_group == 0 || is_windows_9x () == TRUE          // C 5879 (C's `TRUE`)
    { get_file_owner_and_group (std::ptr::null_mut (), buf); }          // C 5880
  else                                                                  // C 5881
    {                                                                   // C 5882
      let mut psd: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();        // C 5883

      psd = get_file_security_desc_by_handle (fh);                      // C 5885
      if ! psd.is_null ()                                               // C 5886
	{
	  get_file_owner_and_group (psd, buf);                                 // C 5888
	  LocalFree (psd);                                                     // C 5889
	}
      else                                                              // C 5891
	{ get_file_owner_and_group (std::ptr::null_mut (), buf); }             // C 5892
    }                                                                   // C 5893

  (*buf).st_dev = info.dwVolumeSerialNumber as dev_t;                   // C 5895
  (*buf).st_rdev = info.dwVolumeSerialNumber as dev_t;                  // C 5896

  (*buf).st_size = info.nFileSizeHigh as c_ulonglong;                   // C 5898 (C's `off_t`/`st_size`)
  (*buf).st_size <<= 32;                                                // C 5899
  (*buf).st_size += info.nFileSizeLow as c_ulonglong;                   // C 5900

  /* Convert timestamps to Unix format. */                              // C 5902
  (*buf).st_mtime = convert_time (info.ftLastWriteTime, (&raw mut (*buf).st_mtimensec)); // C 5903
  (*buf).st_atime = convert_time (info.ftLastAccessTime, (&raw mut (*buf).st_atimensec)); // C 5904
  if (*buf).st_atime == 0                                               // C 5905
    {                                                                   // C 5906
      (*buf).st_atime = (*buf).st_mtime;                                // C 5907
      (*buf).st_atimensec = (*buf).st_mtimensec;                        // C 5908
    }                                                                   // C 5909
  (*buf).st_ctime = convert_time (info.ftCreationTime, (&raw mut (*buf).st_ctimensec)); // C 5910
  if (*buf).st_ctime == 0                                               // C 5911
    {                                                                   // C 5912
      (*buf).st_ctime = (*buf).st_mtime;                                // C 5913
      (*buf).st_ctimensec = (*buf).st_mtimensec;                        // C 5914
    }                                                                   // C 5915


  /* determine rwx permissions */                                       // C 5917
  if (info.dwFileAttributes & FILE_ATTRIBUTE_READONLY) != 0             // C 5918
    { permission = S_IREAD; }                                           // C 5919
  else                                                                  // C 5920
    { permission = S_IREAD | S_IWRITE; }                                // C 5921

  if (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0            // C 5923
    { permission |= S_IEXEC; }                                          // C 5924
  else                                                                  // C 5925
    {                                                                   // C 5926
      /* C 5927-5935's `#if 0` region is dead in every configuration and is carried as C's own
	 text (the C 3390-3406 reading); C's `else` body is therefore empty.  */
      /* C: #if 0 /* no way of knowing the filename */
	 C:   char * p = strrchr (name, '.');
	 C:   if (p != NULL &&
	 C: 	  (xstrcasecmp (p, ".exe") == 0 ||
	 C: 	   xstrcasecmp (p, ".com") == 0 ||
	 C: 	   xstrcasecmp (p, ".bat") == 0 ||
	 C: 	   xstrcasecmp (p, ".cmd") == 0))
	 C: 	permission |= S_IEXEC;
	 C: #endif
	  */
    }                                                                   // C 5936

  (*buf).st_mode |= (permission | (permission >> 3) | (permission >> 6)) as c_ushort; // C 5938

  return 0;                                                             // C 5940
}                                                                       // C 5941

/* Emulate utimensat.  */                                               // C 5943

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utimensat (fd: c_int, name: *const c_char, times: *const timespec, flag: c_int) -> c_int // C 5945 (C's `times[2]` parameter; the array type would put a `;` before the body brace)
{                                                                       // C 5947
  let mut ltimes: [timespec; 2];                                        // C 5948
  let mut fh: HANDLE;                                                   // C 5949
  let mut mtime: FILETIME;                                              // C 5950
  let mut atime: FILETIME;                                              // C 5951
  let mut flags_and_attrs: DWORD = FILE_FLAG_BACKUP_SEMANTICS;          // C 5952

  /* C leaves `ltimes`, `fh`, `mtime` and `atime` uninitialised (PORTING.md 1.5).  */
  ltimes = std::mem::zeroed ();
  fh = std::ptr::null_mut ();
  mtime = std::mem::zeroed ();
  atime = std::mem::zeroed ();

  /* Rely on a hack: an open directory is modeled as file descriptor 0. // C 5954
     This is good enough for the current usage in Emacs, but is fragile. // C 5955

     FIXME: Add proper support for utimensat.                           // C 5957
     Gnulib does this and can serve as a model.  */                     // C 5958
  let mut fullname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 5959

  if fd != AT_FDCWD                                                     // C 5961
    {                                                                   // C 5962
      let mut lastc: c_char = *dir_pathname.offset ((strlen (dir_pathname) - 1) as isize); // C 5963

      if (_snprintf ((&raw mut fullname) as *mut c_char,                // C 5965
		   core::mem::size_of_val (&fullname),
		   c"%s%s%s".as_ptr (),
		   dir_pathname.as_ptr (),                                            // C 5966
		   if IS_DIRECTORY_SEP! (lastc as c_int) { c"".as_ptr () } else { c"/".as_ptr () }, name) // C 5966
	  < 0)                                                                 // C 5967
	{                                                                      // C 5968
	  *errno () = ENAMETOOLONG;                                            // C 5969
	  return -1;                                                           // C 5970
	}                                                                      // C 5971
      name = (&raw mut fullname) as *const c_char;                      // C 5972 (C's array decay)
    }                                                                   // C 5973

  if times.is_null ()                                                   // C 5975 (C's `times == NULL`)
    {                                                                   // C 5976
      memset ((&raw mut ltimes) as *mut c_void, 0, core::mem::size_of_val (&ltimes)); // C 5977
      /* C 5978's chained `ltimes[0] = ltimes[1] = current_timespec ();` calls the function
	 **once**; the port keeps that with one temporary.  */
      let ts: timespec = current_timespec ();                           // C 5978 (the single call)
      ltimes[0] = ts; ltimes[1] = ts;                                   // C 5978
    }                                                                   // C 5979
  else                                                                  // C 5980
    {                                                                   // C 5981
      if times[0].tv_nsec == UTIME_OMIT && times[1].tv_nsec == UTIME_OMIT // C 5982
	{ return 0; }		/* nothing to do */                                     // C 5983 (C's single statement)
      if ((times[0].tv_nsec != UTIME_NOW && times[0].tv_nsec != UTIME_OMIT // C 5984
	   && ! (0 <= times[0].tv_nsec && times[0].tv_nsec < 1000000000))      // C 5985
	  || (times[1].tv_nsec != UTIME_NOW && times[1].tv_nsec != UTIME_OMIT  // C 5986
	      && ! (0 <= times[1].tv_nsec && times[1].tv_nsec < 1000000000)))  // C 5987
	{                                                                      // C 5988
	  *errno () = EINVAL;	/* reject invalid timespec values */             // C 5989
	  return -1;                                                           // C 5990
	}                                                                      // C 5991

      memcpy ((&raw mut ltimes) as *mut c_void, times as *const c_void, core::mem::size_of_val (&ltimes)); // C 5993 (C's `sizeof (ltimes)`)
      if ltimes[0].tv_nsec == UTIME_NOW                                 // C 5994
	{ ltimes[0] = current_timespec (); }                                   // C 5995
      if ltimes[1].tv_nsec == UTIME_NOW                                 // C 5996
	{ ltimes[1] = current_timespec (); }                                   // C 5997
    }                                                                   // C 5998

  if flag == AT_SYMLINK_NOFOLLOW                                        // C 6000
    { flags_and_attrs |= FILE_FLAG_OPEN_REPARSE_POINT; }                // C 6001
  if w32_unicode_filenames                                              // C 6002
    {                                                                   // C 6003
      let mut name_utf16: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6004 (C's `wchar_t`)

      if filename_to_utf16 (name, name_utf16.as_mut_ptr ()) != 0        // C 6006
	{ return -1; }	/* errno set by filename_to_utf16 */                    // C 6007

      /* Need write access to set times.  */                            // C 6009
      fh = CreateFileW (name_utf16.as_ptr (), FILE_WRITE_ATTRIBUTES,    // C 6010
			/* If NAME specifies a directory, FILE_SHARE_DELETE                  // C 6011
			   allows other processes to delete files inside it,                 // C 6012
			   while we have the directory open.  */                             // C 6013
			FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,              // C 6014
			0, OPEN_EXISTING, flags_and_attrs, std::ptr::null_mut ());           // C 6015
    }                                                                   // C 6016
  else                                                                  // C 6017
    {                                                                   // C 6018
      let mut name_ansi: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6019

      if filename_to_ansi (name, name_ansi.as_mut_ptr ()) != 0          // C 6021
	{ return -1; }	/* errno set by filename_to_ansi */                     // C 6022

      fh = CreateFileA (name_ansi.as_ptr (), FILE_WRITE_ATTRIBUTES,     // C 6024
			FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,              // C 6025
			0, OPEN_EXISTING, flags_and_attrs, std::ptr::null_mut ());           // C 6026
    }                                                                   // C 6027
  if fh != INVALID_HANDLE_VALUE                                         // C 6028
    {                                                                   // C 6029
      let mut patime: *mut FILETIME;                                    // C 6030 (C's `FILETIME *patime, *pmtime;`)
      let mut pmtime: *mut FILETIME;                                    // C 6030
      if ltimes[0].tv_nsec == UTIME_OMIT                                // C 6031
	{ patime = std::ptr::null_mut (); }                                    // C 6032
      else                                                              // C 6033
	{                                                                      // C 6034
	  convert_from_timespec (ltimes[0], (&raw mut atime));                 // C 6035
	  patime = (&raw mut atime);                                           // C 6036
	}                                                                      // C 6037
      if ltimes[1].tv_nsec == UTIME_OMIT                                // C 6038
	{ pmtime = std::ptr::null_mut (); }                                    // C 6039
      else                                                              // C 6040
	{                                                                      // C 6041
	  convert_from_timespec (ltimes[1], (&raw mut mtime));                 // C 6042
	  pmtime = (&raw mut mtime);                                           // C 6043
	}                                                                      // C 6044
      if SetFileTime (fh, std::ptr::null (), patime, pmtime) == 0       // C 6045 (C's `!SetFileTime`)
	{                                                                      // C 6046
	  CloseHandle (fh);                                                    // C 6047
	  *errno () = EACCES;                                                  // C 6048
	  return -1;                                                           // C 6049
	}                                                                      // C 6050
      CloseHandle (fh);                                                 // C 6051
    }                                                                   // C 6052
  else                                                                  // C 6053
    {                                                                   // C 6054
      let err: DWORD = GetLastError ();                                 // C 6055

      match err                                                         // C 6057 (C's `switch (err)`)
	{
	  ERROR_FILE_NOT_FOUND |                                               // C 6059
	  ERROR_PATH_NOT_FOUND |                                               // C 6060
	  ERROR_INVALID_DRIVE |                                                // C 6061
	  ERROR_BAD_NETPATH |                                                  // C 6062
	  ERROR_DEV_NOT_EXIST |                                                // C 6063
	  /* ERROR_INVALID_NAME is the error CreateFile sets when the          // C 6064
	     file name includes ?s, i.e. translation to ANSI failed.  */       // C 6065
	  ERROR_INVALID_NAME =>                                                // C 6066
	    { *errno () = ENOENT; }                                            // C 6067
	  ERROR_TOO_MANY_OPEN_FILES =>                                         // C 6069
	    { *errno () = ENFILE; }                                            // C 6070
	  ERROR_ACCESS_DENIED |                                                // C 6072
	  ERROR_SHARING_VIOLATION =>                                           // C 6073
	    { *errno () = EACCES; }                                            // C 6074
	  _ => /* default: */                                                  // C 6076
	    { *errno () = EINVAL; }                                            // C 6077
	}
      return -1;                                                        // C 6080
    }                                                                   // C 6081
  return 0;                                                             // C 6082
}                                                                       // C 6083

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_umask (mode: c_int) -> c_int               // C 6085
{                                                                       // C 6087
  static mut current_mask: c_int = 0;   /* C 6088's function-local `static int` (the C 569-586 device) */ // C 6088
  let mut retval: c_int;                                                // C 6089 (C's `int retval`)
  let mut arg: c_int = 0;                                               // C 6089

  /* C leaves `retval` uninitialised (PORTING.md 1.5).  */
  retval = 0;

  /* The only bit we really support is the write bit.  Files are        // C 6091
     always readable on MS-Windows, and the execute bit does not exist  // C 6092
     at all.  */                                                        // C 6093
  /* FIXME: if the GROUP and OTHER bits are reset, we should use ACLs   // C 6094
     to prevent access by other users on NTFS.  */                      // C 6095
  if (mode & S_IWRITE) != 0                                             // C 6096
    { arg |= S_IWRITE; }                                                // C 6097

  retval = _umask (arg);                                                // C 6099
  /* Merge into the return value the bits they've set the last time,    // C 6100
     which msvcrt.dll ignores and never returns.  Emacs insists on its  // C 6101
     notion of mask being identical to what we return.  */              // C 6102
  retval |= (current_mask & ! S_IWRITE);                                // C 6103 (C's `~`)
  current_mask = mode;                                                  // C 6104

  return retval;                                                        // C 6106
}                                                                       // C 6107

/* C 6108-6117: C 6108-6109's blank and form feed, C 6110's "Symlink-related functions."
   banner, C 6111-6116's two `#ifndef` guards (the `SDDL_REVISION_1` device) and C 6117's
   blank.  The names this slice first uses are declared here (P-88, `UNVERIFIED`): the
   `<config.h>` macro `IS_ANY_SEP` (`configure.ac` 6496/6506, both branches cited),
   `<winbase.h>`'s `SE_BACKUP_NAME` and `DeviceIoControl`, `<winioctl.h>`'s
   `MAXIMUM_REPARSE_DATA_BUFFER_SIZE`, `<winnt.h>`'s `IO_REPARSE_TAG_SYMLINK` and the five
   `<winerror.h>` codes of C 6206-6232/6453-6463.  C 6252's `static int is_symlink` and
   C 6331's `ssize_t readlink` are this slice's two other definitions, and `readlink`'s
   C 6432 uses P-05's `sa_stack_alloc` for C's bare `alloca`.  */
/* Symlink-related functions.  */                                       // C 6110
/* C 6111-6116: two `#ifndef` guards - the `SDDL_REVISION_1` device (C 168-172): "we define
   them by hand if not already defined", so the constant carries C's value and no `cfg`.
   C: #ifndef SYMBOLIC_LINK_FLAG_DIRECTORY
   C: #define SYMBOLIC_LINK_FLAG_DIRECTORY 0x1
   C: #endif
   C: #ifndef SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE
   C: #define SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE 0x2
   C: #endif  */
#[cfg (feature = "windowsnt")]
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: DWORD = 0x1;
#[cfg (feature = "windowsnt")]
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: DWORD = 0x2;
/// C: configure.ac 6496: `#define IS_ANY_SEP(_c_) (IS_DIRECTORY_SEP (_c_) || IS_DEVICE_SEP (_c_))`
/// C: configure.ac 6506: `#define IS_ANY_SEP(_c_) (IS_DIRECTORY_SEP (_c_))` (the frozen target)
#[cfg (feature = "windowsnt")]
macro_rules! IS_ANY_SEP {
  ($c:expr) => { (IS_DIRECTORY_SEP! ($c) || IS_DEVICE_SEP! ($c)) };
}
/// C: #define SE_BACKUP_NAME TEXT("SeBackupPrivilege")  (`<winbase.h>`; the `unicode` arm is
/// owed as for `SE_CREATE_SYMBOLIC_LINK_NAME`, whose section above records why the constant is
/// the ANSI text)
#[cfg (feature = "windowsnt")]
pub const SE_BACKUP_NAME: &core::ffi::CStr = c"SeBackupPrivilege";
/// C: #define MAXIMUM_REPARSE_DATA_BUFFER_SIZE (16 * 1024)  (`<winioctl.h>`)
#[cfg (feature = "windowsnt")]
pub const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: DWORD = 16 * 1024;
/// C: #define IO_REPARSE_TAG_SYMLINK 0xA000000C  (`<winnt.h>`/`<winioctl.h>`)
#[cfg (feature = "windowsnt")]
pub const IO_REPARSE_TAG_SYMLINK: DWORD = 0xA000000C;
/// C: #define ERROR_FILE_EXISTS 80L
#[cfg (feature = "windowsnt")]
pub const ERROR_FILE_EXISTS: DWORD = 80;
/// C: #define ERROR_INVALID_REPARSE_DATA 4392L
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_REPARSE_DATA: DWORD = 4392;
/// C: #define ERROR_DIRECTORY 267L
#[cfg (feature = "windowsnt")]
pub const ERROR_DIRECTORY: DWORD = 267;
/// C: #define ERROR_NOT_ALL_ASSIGNED 1300L
#[cfg (feature = "windowsnt")]
pub const ERROR_NOT_ALL_ASSIGNED: DWORD = 1300;
/// C: #define ERROR_DISK_FULL 112L
#[cfg (feature = "windowsnt")]
pub const ERROR_DISK_FULL: DWORD = 112;
/// C: WINBASEAPI BOOL WINAPI DeviceIoControl (HANDLE, DWORD, LPVOID, DWORD, LPVOID, DWORD,
/// C:   LPDWORD, LPOVERLAPPED);  C 6410 passes `NULL` for the `LPOVERLAPPED`, so the port
/// declares that parameter `*mut c_void` (P-72 rc 3: a pointer to a record the unit never
/// names).
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn DeviceIoControl (hDevice: HANDLE, dwIoControlCode: DWORD, lpInBuffer: *mut c_void,
			  nInBufferSize: DWORD, lpOutBuffer: *mut c_void, nOutBufferSize: DWORD,
			  lpBytesReturned: *mut DWORD, lpOverlapped: *mut c_void) -> BOOL;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink (filename: *const c_char, linkname: *const c_char) -> c_int // C 6118
{                                                                       // C 6120
  let mut linkfn: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6121
  let mut tgtfn: *mut c_char;                                           // C 6121
  /* The SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE flag is           // C 6122
     supported from Windows 10 build 14972.  It is only supported if    // C 6123
     Developer Mode is enabled, and is ignored if it isn't.  */         // C 6124
  let mut flags: DWORD =                                                // C 6125
    if (os_subtype == OS_SUBTYPE_NT                                     // C 6126 (C's outer paren pair)
	&& (w32_major_version > 10                                             // C 6127
	    || (w32_major_version == 10 && w32_build_number >= 14972)))        // C 6128
	{ SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE } else { 0 };           // C 6129 (C's ternary)
  let mut dir_access: c_int;                                            // C 6130 (C's `int dir_access`)
  let mut filename_ends_in_slash: c_int;                                // C 6130

  /* C leaves `tgtfn`, `dir_access` and `filename_ends_in_slash` uninitialised
     (PORTING.md 1.5).  */
  tgtfn = std::ptr::null_mut ();
  dir_access = 0;
  filename_ends_in_slash = 0;

  /* Diagnostics follows Posix as much as possible.  */                 // C 6132
  if filename.is_null () || linkname.is_null ()                         // C 6133 (C's `== NULL`)
    {                                                                   // C 6134
      *errno () = EFAULT;                                               // C 6135
      return -1;                                                        // C 6136
    }                                                                   // C 6137
  if *filename == 0                                                     // C 6138 (C's `!*filename`)
    {                                                                   // C 6139
      *errno () = ENOENT;                                               // C 6140
      return -1;                                                        // C 6141
    }                                                                   // C 6142
  if strlen (filename) > MAX_UTF8_PATH as usize || strlen (linkname) > MAX_UTF8_PATH as usize // C 6143 (C's `size_t` comparisons)
    {                                                                   // C 6144
      *errno () = ENAMETOOLONG;                                         // C 6145
      return -1;                                                        // C 6146
    }                                                                   // C 6147

  strcpy (linkfn.as_mut_ptr (), map_w32_filename (linkname, std::ptr::null_mut ())); // C 6149
  if (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) == 0            // C 6150
    {                                                                   // C 6151
      *errno () = EPERM;                                                // C 6152
      return -1;                                                        // C 6153
    }                                                                   // C 6154

  /* Note: since empty FILENAME was already rejected, we can safely     // C 6156
     refer to FILENAME[1].  */                                          // C 6157
  if ! (IS_DIRECTORY_SEP! (*filename as c_int) || IS_DEVICE_SEP! (*filename.offset (1) as c_int)) // C 6158 (the macros return `bool`)
    {                                                                   // C 6159
      /* Non-absolute FILENAME is understood as being relative to       // C 6160
	 LINKNAME's directory.  We need to prepend that directory to           // C 6161
	 FILENAME to get correct results from faccessat below, since           // C 6162
	 otherwise it will interpret FILENAME relative to the                  // C 6163
	 directory where the Emacs process runs.  Note that                    // C 6164
	 make-symbolic-link always makes sure LINKNAME is a fully              // C 6165
	 expanded file name.  */                                               // C 6166
      let mut tem: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6167
      let mut p: *mut c_char = linkfn.as_mut_ptr ().offset (strlen (linkfn.as_ptr ()) as isize); // C 6168

      while p > linkfn.as_mut_ptr () && ! IS_ANY_SEP! (*p.offset (-1) as c_int) // C 6170-6171 (C's `p[-1]`)
	{ p = p.offset (-1); }                                                 // C 6171
      if p > linkfn.as_mut_ptr ()                                       // C 6172
	{ strncpy (tem.as_mut_ptr (), linkfn.as_mut_ptr (), p.offset_from (linkfn.as_mut_ptr ()) as usize); } // C 6173
      strcpy (tem.as_mut_ptr ().offset (p.offset_from (linkfn.as_mut_ptr ())), filename); // C 6174
      dir_access = faccessat (AT_FDCWD, tem.as_ptr (), D_OK, AT_EACCESS); // C 6175
    }                                                                   // C 6176
  else                                                                  // C 6177
    { dir_access = faccessat (AT_FDCWD, filename, D_OK, AT_EACCESS); }  // C 6178

  /* Since Windows distinguishes between symlinks to directories and    // C 6180
     to files, we provide a kludgy feature: if FILENAME doesn't         // C 6181
     exist, but ends in a slash, we create a symlink to directory.  If  // C 6182
     FILENAME exists and is a directory, we always create a symlink to  // C 6183
     directory.  */                                                     // C 6184
  filename_ends_in_slash = IS_DIRECTORY_SEP! (*filename.offset ((strlen (filename) - 1) as isize) as c_int) as c_int; // C 6185 (the `bool` store)
  if dir_access == 0 || filename_ends_in_slash != 0                     // C 6186
    { flags |= SYMBOLIC_LINK_FLAG_DIRECTORY; }                          // C 6187

  tgtfn = map_w32_filename (filename, std::ptr::null_mut ()) as *mut c_char; // C 6189 (C's `(char *)` cast)
  if filename_ends_in_slash != 0                                        // C 6190
	{ *tgtfn.offset ((strlen (tgtfn) - 1) as isize) = 0; }                 // C 6191 (C's `'\0'`)

  *errno () = 0;                                                        // C 6193
  if create_symbolic_link (linkfn.as_mut_ptr (), tgtfn, flags) == 0     // C 6194 (C's `!`; the callee is a `BOOLEAN`)
    {                                                                   // C 6195
      /* ENOSYS is set by create_symbolic_link, when it detects that    // C 6196
	 the OS doesn't support the CreateSymbolicLink API.  */                // C 6197
      if *errno () != ENOSYS                                            // C 6198
	{                                                                      // C 6199
	  let w32err: DWORD = GetLastError ();                                 // C 6200

	  match w32err                                                         // C 6202 (C's `switch`)
		{
	      /* ERROR_SUCCESS is sometimes returned when LINKFN and           // C 6204
		 TGTFN point to the same file name, go figure.  */                    // C 6205
		  ERROR_SUCCESS |                                                     // C 6206
		  ERROR_FILE_EXISTS =>                                                // C 6207
		    { *errno () = EEXIST; }                                           // C 6208
		  ERROR_ACCESS_DENIED =>                                              // C 6210
		    { *errno () = EACCES; }                                           // C 6211
		  ERROR_FILE_NOT_FOUND |                                              // C 6213
		  ERROR_PATH_NOT_FOUND |                                              // C 6214
		  ERROR_BAD_NETPATH |                                                 // C 6215
		  ERROR_INVALID_REPARSE_DATA =>                                       // C 6216
		    { *errno () = ENOENT; }                                           // C 6217
		  ERROR_DIRECTORY =>                                                  // C 6219
		    { *errno () = EISDIR; }                                           // C 6220
		  ERROR_PRIVILEGE_NOT_HELD |                                          // C 6222
		  ERROR_NOT_ALL_ASSIGNED =>                                           // C 6223
		    { *errno () = EPERM; }                                            // C 6224
		  ERROR_DISK_FULL =>                                                  // C 6226
		    { *errno () = ENOSPC; }                                           // C 6227
		  _ => /* default: */                                                 // C 6229
		    { *errno () = EINVAL; }                                           // C 6230
		}
	}                                                                      // C 6233
      return -1;                                                        // C 6234
    }                                                                   // C 6235
  return 0;                                                             // C 6236
}                                                                       // C 6237

/* A quick inexpensive test of whether FILENAME identifies a file that  // C 6239
   is a symlink.  Returns non-zero if it is, zero otherwise.  FILENAME  // C 6240
   must already be in the normalized form returned by                   // C 6241
   map_w32_filename.  If the symlink is to a directory, the             // C 6242
   FILE_ATTRIBUTE_DIRECTORY bit will be set in the return value.        // C 6243

   Note: for repeated operations on many files, it is best to test      // C 6245
   whether the underlying volume actually supports symlinks, by         // C 6246
   testing the FILE_SUPPORTS_REPARSE_POINTS bit in volume's flags, and  // C 6247
   avoid the call to this function if it doesn't.  That's because the   // C 6248
   call to GetFileAttributes takes a non-negligible time, especially    // C 6249
   on non-local or removable filesystems.  See stat_worker for an       // C 6250
   example of how to do that.  */                                       // C 6251
#[cfg (feature = "windowsnt")]
unsafe fn is_symlink (filename: *const c_char) -> c_int                 // C 6252 (C's `static int`)
{                                                                       // C 6254
  let mut attrs: DWORD;                                                 // C 6255
  let mut filename_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6256 (C's `wchar_t`)
  let mut filename_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6257
  let mut wfdw: WIN32_FIND_DATAW;                                       // C 6258
  let mut wfda: WIN32_FIND_DATAA;                                       // C 6259
  let mut fh: HANDLE;                                                   // C 6260
  let mut attrs_mean_symlink: c_int;                                    // C 6261

  /* C leaves `attrs`, `wfdw`, `wfda`, `fh` and `attrs_mean_symlink` uninitialised
     (PORTING.md 1.5).  */
  attrs = 0;
  wfdw = std::mem::zeroed ();
  wfda = std::mem::zeroed ();
  fh = std::ptr::null_mut ();
  attrs_mean_symlink = 0;

  if w32_unicode_filenames                                              // C 6263
    {                                                                   // C 6264
      filename_to_utf16 (filename, filename_w.as_mut_ptr ());           // C 6265
      attrs = GetFileAttributesW (filename_w.as_ptr ());                // C 6266
    }                                                                   // C 6267
  else                                                                  // C 6268
    {                                                                   // C 6269
      filename_to_ansi (filename, filename_a.as_mut_ptr ());            // C 6270
      attrs = GetFileAttributesA (filename_a.as_ptr ());                // C 6271
    }                                                                   // C 6272
  if attrs == -1 as DWORD                                               // C 6273 (C's `-1`)
    {                                                                   // C 6274
      let w32err: DWORD = GetLastError ();                              // C 6275

      match w32err                                                      // C 6277
	{
	  ERROR_BAD_NETPATH =>	/* network share, can't be a symlink */         // C 6279
	    {}                                                                 // C 6280 (C's `break;`)
	  ERROR_ACCESS_DENIED =>                                               // C 6281
	    { *errno () = EACCES; }                                            // C 6282
	  ERROR_FILE_NOT_FOUND |                                               // C 6284
	  ERROR_PATH_NOT_FOUND |                                               // C 6285
	  _ => /* default: */                                                  // C 6286
	    { *errno () = ENOENT; }                                            // C 6287
	}
      return 0;                                                         // C 6290
    }                                                                   // C 6291
  if (attrs & FILE_ATTRIBUTE_REPARSE_POINT) == 0                        // C 6292
    { return 0; }                                                       // C 6293
  logon_network_drive (filename);                                       // C 6294
  if w32_unicode_filenames                                              // C 6295
    {                                                                   // C 6296
      fh = FindFirstFileW (filename_w.as_ptr (), (&raw mut wfdw));      // C 6297
      attrs_mean_symlink =                                              // C 6298
	((wfdw.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) != 0           // C 6299
	 && (wfdw.dwReserved0 & IO_REPARSE_TAG_SYMLINK) == IO_REPARSE_TAG_SYMLINK) as c_int; // C 6300
      if attrs_mean_symlink != 0                                        // C 6301
	{ attrs_mean_symlink |= (wfdw.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) as c_int; } // C 6302
    }                                                                   // C 6303
  else if ! _mbspbrk (filename_a.as_ptr () as *const c_uchar, c"?".as_ptr () as *const c_uchar).is_null () // C 6304 (C's `_mbspbrk (filename_a, "?")`)
    {                                                                   // C 6305
      /* filename_to_ansi failed to convert the file name.  */          // C 6306
      *errno () = ENOENT;                                               // C 6307
      return 0;                                                         // C 6308
    }                                                                   // C 6309
  else                                                                  // C 6310
    {                                                                   // C 6311
      fh = FindFirstFileA (filename_a.as_ptr (), (&raw mut wfda));      // C 6312
      attrs_mean_symlink =                                              // C 6313
	((wfda.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) != 0           // C 6314
	 && (wfda.dwReserved0 & IO_REPARSE_TAG_SYMLINK) == IO_REPARSE_TAG_SYMLINK) as c_int; // C 6315
      if attrs_mean_symlink != 0                                        // C 6316
	{ attrs_mean_symlink |= (wfda.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) as c_int; } // C 6317
    }                                                                   // C 6318
  if fh == INVALID_HANDLE_VALUE                                         // C 6319
    { return 0; }                                                       // C 6320
  FindClose (fh);                                                       // C 6321
  return attrs_mean_symlink;                                            // C 6322
}                                                                       // C 6323

/* If NAME identifies a symbolic link, copy into BUF the file name of   // C 6325
   the symlink's target.  Copy at most BUF_SIZE bytes, and do NOT       // C 6326
   null-terminate the target name, even if it fits.  Return the number  // C 6327
   of bytes copied, or -1 if NAME is not a symlink or any error was     // C 6328
   encountered while resolving it.  The file name copied into BUF is    // C 6329
   encoded in the current ANSI codepage.  */                            // C 6330
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn readlink (name: *const c_char, buf: *mut c_char, buf_size: usize) -> ssize_t // C 6331 (C's `const char *`/`size_t`)
{                                                                       // C 6333
  let mut path: *const c_char;                                          // C 6334
  let mut privs: TOKEN_PRIVILEGES;                                      // C 6335
  let mut restore_privs: c_int = 0;                                     // C 6336
  let mut sh: HANDLE;                                                   // C 6337
  let mut retval: ssize_t;                                              // C 6338
  let mut resolved: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6339

  /* C leaves `path`, `privs`, `sh` and `retval` uninitialised (PORTING.md 1.5).  */
  path = std::ptr::null ();
  privs = std::mem::zeroed ();
  sh = std::ptr::null_mut ();
  retval = 0;

  if name.is_null ()                                                    // C 6341 (C's `== NULL`)
    {                                                                   // C 6342
      *errno () = EFAULT;                                               // C 6343
      return -1;                                                        // C 6344
    }                                                                   // C 6345
  if *name == 0                                                         // C 6346 (C's `!*name`)
    {                                                                   // C 6347
      *errno () = ENOENT;                                               // C 6348
      return -1;                                                        // C 6349
    }                                                                   // C 6350

  path = map_w32_filename (name, std::ptr::null_mut ());                // C 6352

  if strlen (path) > MAX_UTF8_PATH as usize                             // C 6354 (C's `size_t` comparison)
    {                                                                   // C 6355
      *errno () = ENAMETOOLONG;                                         // C 6356
      return -1;                                                        // C 6357
    }                                                                   // C 6358

  *errno () = 0;                                                        // C 6360
  if is_windows_9x () == TRUE                                           // C 6361
      || (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) == 0        // C 6362
      || is_symlink (path) == 0                                         // C 6363 (C's `!is_symlink (path)`)
    {                                                                   // C 6364
      if *errno () == 0                                                 // C 6365 (C's `!errno`)
	{ *errno () = EINVAL; }	/* not a symlink */                            // C 6366
      return -1;                                                        // C 6367
    }                                                                   // C 6368

  /* Done with simple tests, now we're in for some _real_ work.  */     // C 6370
  if enable_privilege (SE_BACKUP_NAME.as_ptr (), TRUE, (&raw mut privs)) != 0 // C 6371 (C's truth test)
    { restore_privs = 1; }                                              // C 6372
  /* Implementation note: From here and onward, don't return early,     // C 6373
     since that will fail to restore the original set of privileges of  // C 6374
     the calling thread.  */                                            // C 6375

  retval = -1;	/* not too optimistic, are we? */                        // C 6377

  /* Note: In the next call to CreateFile, we use zero as the 2nd       // C 6379
     argument because, when the symlink is a hidden/system file,        // C 6380
     e.g. 'C:\Users\All Users', GENERIC_READ fails with                 // C 6381
     ERROR_ACCESS_DENIED.  Zero seems to work just fine, both for file  // C 6382
     and directory symlinks.  */                                        // C 6383
  if w32_unicode_filenames                                              // C 6384
    {                                                                   // C 6385
      let mut path_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6386 (C's `wchar_t`)

      filename_to_utf16 (path, path_w.as_mut_ptr ());                   // C 6388
      sh = CreateFileW (path_w.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 6389
			FILE_FLAG_OPEN_REPARSE_POINT                                         // C 6390
			| FILE_FLAG_BACKUP_SEMANTICS,                                        // C 6391
			std::ptr::null_mut ());                                              // C 6392
    }                                                                   // C 6393
  else                                                                  // C 6394
    {                                                                   // C 6395
      let mut path_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6396

      filename_to_ansi (path, path_a.as_mut_ptr ());                    // C 6398
      sh = CreateFileA (path_a.as_ptr (), 0, 0, std::ptr::null_mut (), OPEN_EXISTING, // C 6399
			FILE_FLAG_OPEN_REPARSE_POINT                                         // C 6400
			| FILE_FLAG_BACKUP_SEMANTICS,                                        // C 6401
			std::ptr::null_mut ());                                              // C 6402
    }                                                                   // C 6403
  if sh != INVALID_HANDLE_VALUE                                         // C 6404
    {                                                                   // C 6405
      let mut reparse_buf: [BYTE; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize] = [0; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize]; // C 6406
      let mut reparse_data: *mut REPARSE_DATA_BUFFER = reparse_buf.as_mut_ptr () as *mut REPARSE_DATA_BUFFER; // C 6407 (C's `&reparse_buf[0]` cast)
      let mut retbytes: DWORD;                                          // C 6408

      /* C leaves `retbytes` uninitialised (PORTING.md 1.5).  */
      retbytes = 0;

      if DeviceIoControl (sh, FSCTL_GET_REPARSE_POINT, std::ptr::null_mut (), 0, // C 6410
			    reparse_buf.as_mut_ptr () as *mut c_void, MAXIMUM_REPARSE_DATA_BUFFER_SIZE, // C 6411
			    (&raw mut retbytes), std::ptr::null_mut ()) == 0                 // C 6412 (C's `!`)
	{ *errno () = EIO; }                                                   // C 6413
      else if (*reparse_data).ReparseTag != IO_REPARSE_TAG_SYMLINK      // C 6414
	{ *errno () = EINVAL; }                                                // C 6415
      else                                                              // C 6416
	{                                                                      // C 6417
	  /* Copy the link target name, in wide characters, from               // C 6418
	     reparse_data, then convert it to multibyte encoding in            // C 6419
	     the current locale's codepage.  */                                // C 6420
	  let mut lwname: *mut WCHAR;                                          // C 6421
	  let mut lname_size: size_t;                                          // C 6422
	  let lwname_len: USHORT =                                             // C 6423
	    (*reparse_data).SymbolicLinkReparseBuffer.PrintNameLength;         // C 6424
	  let lwname_src: *mut WCHAR =                                         // C 6425
	    (*reparse_data).SymbolicLinkReparseBuffer.PathBuffer.as_mut_ptr () // C 6426
	    .offset (((*reparse_data).SymbolicLinkReparseBuffer.PrintNameOffset // C 6427
		      / core::mem::size_of::<WCHAR> ()) as isize);                    // C 6427
	  let mut size_to_copy: size_t = buf_size;                             // C 6428

  /* C leaves `lwname` and `lname_size` uninitialised (PORTING.md 1.5).  */
  lwname = std::ptr::null_mut ();
  lname_size = 0;

	  /* According to MSDN, PrintNameLength does not include the           // C 6430
	     terminating null character.  */                                   // C 6431
	  lwname = sa_stack_alloc (((lwname_len + 1) as usize) * core::mem::size_of::<WCHAR> ()) as *mut WCHAR; // C 6432 (C's bare `alloca`; P-05's device)
	  memcpy (lwname as *mut c_void, lwname_src as *const c_void, lwname_len as usize); // C 6433
	  *lwname.offset ((lwname_len as usize / core::mem::size_of::<WCHAR> ()) as isize) = 0; /* null-terminate */ // C 6434
	  filename_from_utf16 (lwname, resolved.as_mut_ptr ());                // C 6435
	  dostounix_filename (resolved.as_mut_ptr ());                         // C 6436
	  lname_size = strlen (resolved.as_ptr ()) + 1;                        // C 6437
	  if lname_size <= buf_size                                            // C 6438
	    { size_to_copy = lname_size; }                                     // C 6439
	  memcpy (buf as *mut c_void, resolved.as_ptr () as *const c_void, size_to_copy); // C 6440
	  /* Success!  */                                                      // C 6441
	  retval = size_to_copy as ssize_t;                                    // C 6442 (the `size_t`-into-`ssize_t` store)
	}                                                                      // C 6443
      CloseHandle (sh);                                                 // C 6444
    }                                                                   // C 6445
  else                                                                  // C 6446
    {                                                                   // C 6447
      /* CreateFile failed.  */                                         // C 6448
      let w32err2: DWORD = GetLastError ();                             // C 6449

      match w32err2                                                     // C 6451
	{
	  ERROR_FILE_NOT_FOUND |                                               // C 6453
	  ERROR_PATH_NOT_FOUND =>                                              // C 6454
	    { *errno () = ENOENT; }                                            // C 6455
	  ERROR_ACCESS_DENIED |                                                // C 6457
	  ERROR_TOO_MANY_OPEN_FILES =>                                         // C 6458
	    { *errno () = EACCES; }                                            // C 6459
	  _ => /* default: */                                                  // C 6461
	    { *errno () = EPERM; }                                             // C 6462
	}
    }                                                                   // C 6465
  if restore_privs != 0                                                 // C 6466 (C's truth test)
    {                                                                   // C 6467
      restore_privilege ((&raw mut privs));                             // C 6468
      revert_to_self ();                                                // C 6469
    }                                                                   // C 6470

  return retval;                                                        // C 6472
}                                                                       // C 6473

/* C 6474-6497: C 6474's blank, C 6475's `ssize_t readlinkat` + C 6476-6477's signature,
   C 6497's `chase_symlinks` banner comment (carried verbatim below) and C 6517's
   `static char *`.  The two names this slice first uses are declared here (P-88,
   `UNVERIFIED`): `<winbase.h>`'s `GetFullPathNameW`/`GetFullPathNameA` (C 6535/6544 pass
   `NULL` for the `lpFilePart`, which is why that parameter is `*mut LPWSTR` and the call site
   passes `std::ptr::null_mut ()`; P-72 rc 3 covers the other direction).  C 6520's
   function-local `static char target[MAX_UTF8_PATH]` is a function-local `static mut` (the
   `map_w32_filename` `shortname` device), and C 6551-6592's `do { … } while (…)` is PORTING.md
   P-95's `loop { … if ! (…) { break; } }`.  C 6613-6614's form feed and "Posix ACL
   emulation." banner and the four ACL entries that follow it are the rest of the slice.  */
/// C: DWORD GetFullPathNameW (LPCWSTR lpFileName, DWORD nBufferLength,
/// C:   LPWSTR lpBuffer, LPWSTR *lpFilePart);
/// C: DWORD GetFullPathNameA (LPCSTR lpFileName, DWORD nBufferLength,
/// C:   LPSTR lpBuffer, LPSTR *lpFilePart);
/// C 206-222's five guards are inside C 174's `#if defined(MINGW_W64)` (the `#endif` at C
/// 223 closes it), so `FSCTL_GET_REPARSE_POINT`, the record and `CTL_CODE` are this unit's own
/// only under `mingw_w64`; C's other arm takes all of them from `<winioctl.h>`/`<ntifs.h>` -
/// the SDK gap Owed 18 records, and the arm reading's three rows for this slice.
/// C: #define DACL_SECURITY_INFORMATION 0x00000004  (`<winnt.h>`, the third of the four
/// `SECURITY_INFORMATION` bits; `OWNER_`/`GROUP_` are slice 9's in-unit pair)
#[cfg (feature = "windowsnt")]
pub const DACL_SECURITY_INFORMATION: SECURITY_INFORMATION = 0x00000004;
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetFullPathNameW (lpFileName: LPCWSTR, nBufferLength: DWORD, lpBuffer: LPWSTR,
			  lpFilePart: *mut LPWSTR) -> DWORD;
  pub fn GetFullPathNameA (lpFileName: LPCSTR, nBufferLength: DWORD, lpBuffer: LPSTR,
			  lpFilePart: *mut LPSTR) -> DWORD;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn readlinkat (fd: c_int, name: *const c_char, buffer: *mut c_char, // C 6475
				    buffer_size: usize) -> ssize_t                                  // C 6477
{                                                                       // C 6478
  /* Rely on a hack: an open directory is modeled as file descriptor 0, // C 6479
     as in fstatat.  FIXME: Add proper support for readlinkat.  */      // C 6480
  let mut fullname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6481

  if fd != AT_FDCWD                                                     // C 6483
    {                                                                   // C 6484
      if (_snprintf ((&raw mut fullname) as *mut c_char,                // C 6485
		  core::mem::size_of_val (&fullname),
		  c"%s/%s".as_ptr (),
		  dir_pathname.as_ptr (), name)                                       // C 6485
	  < 0)                                                                 // C 6486
	{                                                                      // C 6487
	  *errno () = ENAMETOOLONG;                                            // C 6488
	  return -1;                                                           // C 6489
	}                                                                      // C 6490
      name = (&raw mut fullname) as *const c_char;                      // C 6491 (C's array decay)
    }                                                                   // C 6492

  return readlink (name, buffer, buffer_size);                          // C 6494
}                                                                       // C 6495

/* If FILE is a symlink, return its target (stored in a static          // C 6497
   buffer); otherwise return FILE.                                      // C 6498

   This function repeatedly resolves symlinks in the last component of  // C 6500
   a chain of symlink file names, as in foo -> bar -> baz -> ...,       // C 6501
   until it arrives at a file whose last component is not a symlink,    // C 6502
   or some error occurs.  It returns the target of the last             // C 6503
   successfully resolved symlink in the chain.  If it succeeds to       // C 6504
   resolve even a single symlink, the value returned is an absolute     // C 6505
   file name with backslashes (result of GetFullPathName).  By          // C 6506
   contrast, if the original FILE is returned, it is unaltered.         // C 6507

   Note: This function can set errno even if it succeeds.               // C 6509

   Implementation note: we only resolve the last portion ("basename")   // C 6511
   of the argument FILE and of each following file in the chain,        // C 6512
   disregarding any possible symlinks in its leading directories.       // C 6513
   This is because Windows system calls and library functions           // C 6514
   transparently resolve symlinks in leading directories and return     // C 6515
   correct information, as long as the basename is not a symlink.  */   // C 6516
#[cfg (feature = "windowsnt")]
unsafe fn chase_symlinks (file: *const c_char) -> *mut c_char           // C 6517 (C's `static char *`)
{                                                                       // C 6519
  static mut target: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];  /* C 6520 */
  let mut link: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6521
  let mut target_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6522 (C's `wchar_t`)
  let mut link_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6522
  let mut target_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6523
  let mut link_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6523
  let mut res: ssize_t;                                                 // C 6524
  let mut link_len: ssize_t;                                            // C 6524
  let mut loop_count: c_int = 0;                                        // C 6525

  /* C leaves `res` and `link_len` uninitialised (PORTING.md 1.5).  */
  res = 0;
  link_len = 0;

  if is_windows_9x () == TRUE || is_symlink (file) == 0                 // C 6527 (C's `!is_symlink (file)`)
    { return file as *mut c_char; }                                     // C 6528 (C's `(char *)` cast)

  if w32_unicode_filenames                                              // C 6530
    {                                                                   // C 6531
      let mut file_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6532 (C's `wchar_t`)

      filename_to_utf16 (file, file_w.as_mut_ptr ());                   // C 6534
      if GetFullPathNameW (file_w.as_ptr (), MAX_PATH as DWORD, link_w.as_mut_ptr (), std::ptr::null_mut ()) == 0 // C 6535
	{ return file as *mut c_char; }                                        // C 6536
      filename_from_utf16 (link_w.as_mut_ptr (), link.as_mut_ptr ());   // C 6537
    }                                                                   // C 6538
  else                                                                  // C 6539
    {                                                                   // C 6540
      let mut file_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6541

      filename_to_ansi (file, file_a.as_mut_ptr ());                    // C 6543
      if GetFullPathNameA (file_a.as_ptr (), MAX_PATH as DWORD, link_a.as_mut_ptr (), std::ptr::null_mut ()) == 0 // C 6544
	{ return file as *mut c_char; }                                        // C 6545
      filename_from_ansi (link_a.as_mut_ptr (), link.as_mut_ptr ());    // C 6546
    }                                                                   // C 6547
  link_len = strlen (link.as_ptr ()) as ssize_t;                        // C 6548 (C's `DWORD`-into-`ssize_t` store)

  *target.as_mut_ptr () = 0;                                            // C 6550 (C's `target[0] = '\0';`)
  /* C 6551's `do {` - P-95's device: the negation of C 6592's condition breaks the loop.  */
  loop                                                                  // C 6551
    {

    /* Remove trailing slashes, as we want to resolve the last          // C 6553
       non-trivial part of the link name.  */                           // C 6554
    while link_len > 3 && IS_DIRECTORY_SEP! (*link.as_ptr ().offset (link_len - 1) as c_int) // C 6555
      { link_len -= 1; *link.as_mut_ptr ().offset (link_len) = 0; }     // C 6556 (C's `link[link_len--] = '\0'`)

    res = readlink (link.as_ptr (), target.as_mut_ptr (), MAX_UTF8_PATH as usize); // C 6558
    if res > 0                                                          // C 6559
      {                                                                 // C 6560
	*target.as_mut_ptr ().offset (res as isize) = 0;                       // C 6561 (C's `'\0'`)
	if ! (IS_DEVICE_SEP! (*target.as_ptr ().offset (1) as c_int)           // C 6562 (the macros return `bool`)
	      || (IS_DIRECTORY_SEP! (*target.as_ptr () as c_int) && IS_DIRECTORY_SEP! (*target.as_ptr ().offset (1) as c_int))) // C 6563
	  {
	    /* Target is relative.  Append it to the directory part of         // C 6565
	       the symlink, then copy the result back to target.  */           // C 6566
	    let mut p: *mut c_char = link.as_mut_ptr ().offset (link_len as isize); // C 6567

	    while p > link.as_mut_ptr () && ! IS_ANY_SEP! (*p.offset (-1) as c_int) // C 6569 (C's `p[-1]`)
	      { p = p.offset (-1); }                                           // C 6570
	    strcpy (p, target.as_ptr ());                                      // C 6571
	    strcpy (target.as_mut_ptr (), link.as_ptr ());                     // C 6572
	  }
	/* Resolve any "." and ".." to get a fully-qualified file name         // C 6574
	   in link[] again. */                                                 // C 6575
	if w32_unicode_filenames                                               // C 6576
	  {
	    filename_to_utf16 (target.as_ptr (), target_w.as_mut_ptr ());      // C 6578
	    link_len = GetFullPathNameW (target_w.as_ptr (), MAX_PATH as DWORD, link_w.as_mut_ptr (), std::ptr::null_mut ()) as ssize_t; // C 6579
	    if link_len > 0                                                    // C 6580
	      { filename_from_utf16 (link_w.as_mut_ptr (), link.as_mut_ptr ()); } // C 6581
	  }
	else                                                                   // C 6583
	  {
	    filename_to_ansi (target.as_ptr (), target_a.as_mut_ptr ());       // C 6585
	    link_len = GetFullPathNameA (target_a.as_ptr (), MAX_PATH as DWORD, link_a.as_mut_ptr (), std::ptr::null_mut ()) as ssize_t; // C 6586
	    if link_len > 0                                                    // C 6587
	      { filename_from_ansi (link_a.as_mut_ptr (), link.as_mut_ptr ()); } // C 6588
	  }
	link_len = strlen (link.as_ptr ()) as ssize_t;                         // C 6590
      }                                                                 // C 6591
    if ! (res > 0 && link_len > 0                                       // C 6592 (C's do-while condition)
	  && { loop_count += 1; loop_count <= 100 })                           // C 6592 (C's `++loop_count`)
      { break; }
    }                                                                   // C 6592

  if loop_count > 100                                                   // C 6594
    { *errno () = ELOOP; }                                              // C 6595

  if *target.as_ptr () == 0 /* not a single call to readlink succeeded */ // C 6597 (C's `'\0'`)
    { return file as *mut c_char; }                                     // C 6598 (C's `(char *)` cast)
  return target.as_mut_ptr ();                                          // C 6599
}                                                                       // C 6600

/* Return non-zero if FILE's filesystem supports symlinks.  */          // C 6602
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlinks_supported (file: *const c_char) -> bool // C 6603 (C's `bool`)
{                                                                       // C 6605
  if is_windows_9x () != TRUE                                           // C 6606
      && get_volume_info (file, std::ptr::null_mut ()) != 0             // C 6607 (C's truth test)
      && (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) != 0        // C 6608
    { return true; }                                                    // C 6609
  return false;                                                         // C 6610
}                                                                       // C 6611

/* Posix ACL emulation.  */                                             // C 6614

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_valid (acl: acl_t) -> c_int                // C 6616
{                                                                       // C 6618
  return if is_valid_security_descriptor (acl) != 0 { 0 } else { -1 };  // C 6619 (C's ternary)
}                                                                       // C 6620

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
/// C: `char * ATTRIBUTE_MALLOC` - the attribute is an allocator annotation, not part of the
/// return type (the `image.rs` 5253 reading of PORTING.md 8).
pub unsafe extern "C" fn acl_to_text (acl: acl_t, size: *mut ssize_t) -> *mut c_char // C 6622
{                                                                       // C 6624
  let mut str_acl: LPTSTR;                                              // C 6625
  let mut flags: SECURITY_INFORMATION =                                 // C 6626
    OWNER_SECURITY_INFORMATION |                                        // C 6627
    GROUP_SECURITY_INFORMATION |                                        // C 6628
    DACL_SECURITY_INFORMATION;                                          // C 6629
  let mut retval: *mut c_char = std::ptr::null_mut ();                  // C 6630 (C's `= NULL`)
  let mut local_size: ULONG;                                            // C 6631
  let mut e: c_int = *errno ();                                         // C 6632 (C's `int e = errno;`)

  /* C leaves `str_acl` and `local_size` uninitialised (PORTING.md 1.5).  */
  str_acl = std::ptr::null_mut ();
  local_size = 0;

  *errno () = 0;                                                        // C 6634

  if convert_sd_to_sddl (acl, SDDL_REVISION_1, flags, (&raw mut str_acl), (&raw mut local_size)) != 0 // C 6636 (C's truth test)
    {                                                                   // C 6637
      *errno () = e;                                                    // C 6638
      /* We don't want to mix heaps, so we duplicate the string in our  // C 6639
	 heap and free the one allocated by the API.  */                       // C 6640
      retval = xstrdup (str_acl);                                       // C 6641
      if ! size.is_null ()                                              // C 6642 (C's `if (size)`)
	{ *size = local_size; }                                                // C 6643 (the `ULONG`-into-`ssize_t` store)
      LocalFree (str_acl as *mut c_void);                               // C 6644 (C's `char *`-into-`HLOCAL`)
    }                                                                   // C 6645
  else if *errno () != ENOTSUP                                          // C 6646
    { *errno () = EINVAL; }                                             // C 6647

  return retval;                                                        // C 6649
}                                                                       // C 6650

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_from_text (acl_str: *const c_char) -> acl_t // C 6652
{                                                                       // C 6654
  let mut psd: PSECURITY_DESCRIPTOR;                                    // C 6655
  let mut retval: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();         // C 6655 (C's `= NULL`)
  let mut sd_size: ULONG;                                               // C 6656
  let mut e: c_int = *errno ();                                         // C 6657

  /* C leaves `psd` and `sd_size` uninitialised (PORTING.md 1.5).  */
  psd = std::ptr::null_mut ();
  sd_size = 0;

  *errno () = 0;                                                        // C 6659

  if convert_sddl_to_sd (acl_str, SDDL_REVISION_1, (&raw mut psd), (&raw mut sd_size)) != 0 // C 6661 (C's truth test)
    {                                                                   // C 6662
      *errno () = e;                                                    // C 6663
      retval = xmalloc (sd_size as usize);                              // C 6664 (C's `void *` into `PSECURITY_DESCRIPTOR`)
      memcpy (retval, psd, sd_size as usize);                           // C 6665
      LocalFree (psd);                                                  // C 6666
    }                                                                   // C 6667
  else if *errno () != ENOTSUP                                          // C 6668
    { *errno () = EINVAL; }                                             // C 6669

  return retval;                                                        // C 6671
}                                                                       // C 6672

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_free (ptr: *mut c_void) -> c_int           // C 6674
{                                                                       // C 6676
  xfree (ptr);                                                          // C 6677
  return 0;                                                             // C 6678
}                                                                       // C 6679

/* C 6680-6694: C 6680's blank, C 6681's `acl_t` + C 6682's `acl_get_file` signature (whose
   C 6697-6700 assigns to the `fname` parameter, so the port takes it `mut` - the
   `stat_worker (mut path, …)` device) and C 6688-6693's locals.  The three names this slice
   first uses are declared here (P-88, `UNVERIFIED`): `<winbase.h>`'s two privilege strings
   (`SE_TAKE_OWNERSHIP_NAME`, `SE_RESTORE_NAME`; the `SE_BACKUP_NAME` device of slice 14, with
   its owed `unicode` arm) and `<winerror.h>`'s `ERROR_INVALID_OWNER`.  C 6753's `acl_set_file`
   and C 6891's `acl_errno_valid` (C's `bool`, the `symlinks_supported` reading) are the rest
   of the slice.  */
/// C: #define SE_TAKE_OWNERSHIP_NAME TEXT("SeTakeOwnershipPrivilege")  (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const SE_TAKE_OWNERSHIP_NAME: &core::ffi::CStr = c"SeTakeOwnershipPrivilege";
/// C: #define SE_RESTORE_NAME TEXT("SeRestorePrivilege")  (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const SE_RESTORE_NAME: &core::ffi::CStr = c"SeRestorePrivilege";
/// C: #define ERROR_INVALID_OWNER 1305L
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_OWNER: DWORD = 1305;

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_get_file (mut fname: *const c_char, r#type: acl_type_t) -> acl_t // C 6681 (C's `acl_t`/`const char *`)
{                                                                       // C 6683
  let mut psd: PSECURITY_DESCRIPTOR = std::ptr::null_mut ();            // C 6684 (C's `= NULL`)
  let mut filename: *const c_char;                                      // C 6685

  /* C leaves `filename` uninitialised (PORTING.md 1.5).  */
  filename = std::ptr::null ();

  if r#type == ACL_TYPE_ACCESS                                          // C 6687
    {                                                                   // C 6688
      let mut sd_len: DWORD;                                            // C 6689 (C's `DWORD sd_len`)
      let mut err: DWORD;                                               // C 6689
      let mut si: SECURITY_INFORMATION =                                // C 6690
	OWNER_SECURITY_INFORMATION |                                           // C 6691
	GROUP_SECURITY_INFORMATION |                                           // C 6692
	DACL_SECURITY_INFORMATION ;                                            // C 6693
      let e: c_int = *errno ();                                         // C 6694 (C's `int e = errno;`)

      /* C leaves `sd_len` and `err` uninitialised (PORTING.md 1.5).  */
      sd_len = 0;
      err = 0;

      filename = map_w32_filename (fname, std::ptr::null_mut ());       // C 6696
      if (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) != 0        // C 6697
	{ fname = chase_symlinks (filename); }                                 // C 6698
      else                                                              // C 6699
	{ fname = filename; }                                                  // C 6700

      *errno () = 0;                                                    // C 6702
      if get_file_security (fname, si, psd, 0, (&raw mut sd_len)) == 0  // C 6703 (C's `!`)
	  && *errno () != ENOTSUP                                              // C 6704
	{                                                                      // C 6705
	  err = GetLastError ();                                               // C 6706
	  if err == ERROR_INSUFFICIENT_BUFFER                                  // C 6707
	    {                                                                  // C 6708
	      psd = xmalloc (sd_len as usize);                                 // C 6709 (C's `void *` into `PSECURITY_DESCRIPTOR`)
	      if get_file_security (fname, si, psd, sd_len, (&raw mut sd_len)) == 0 // C 6710 (C's `!`)
		{                                                                     // C 6711
		  xfree (psd);                                                        // C 6712
		  err = GetLastError ();                                              // C 6713
		  if err == ERROR_NOT_SUPPORTED                                       // C 6714
		      || err == ERROR_ACCESS_DENIED                                   // C 6715
		      || err == ERROR_INVALID_FUNCTION                                // C 6716
		    { *errno () = ENOTSUP; }                                          // C 6717
		  else if err == ERROR_FILE_NOT_FOUND                                 // C 6718
			   || err == ERROR_PATH_NOT_FOUND                                    // C 6719
			   || err == ERROR_INVALID_NAME                                      // C 6720
		    { *errno () = ENOENT; }                                           // C 6721
		  else                                                                // C 6722
		    { *errno () = EIO; }                                              // C 6723
		  psd = std::ptr::null_mut ();                                        // C 6724 (C's `psd = NULL;`)
		}                                                                     // C 6725
	    }                                                                  // C 6726
	  else if (err == ERROR_FILE_NOT_FOUND                                 // C 6727 (C's outer paren pair)
		   || err == ERROR_PATH_NOT_FOUND                                     // C 6728
		   /* ERROR_INVALID_NAME is what we get if                            // C 6729
		      w32-unicode-filenames is nil and the file cannot                // C 6730
		      be encoded in the current ANSI codepage. */                     // C 6731
		   || err == ERROR_INVALID_NAME)                                      // C 6732 (C's comment above is the ERR block)
	    { *errno () = ENOENT; }                                            // C 6733
	  else if (err == ERROR_NOT_SUPPORTED                                  // C 6734 (C's outer paren pair)
		   /* ERROR_ACCESS_DENIED or ERROR_INVALID_FUNCTION is                // C 6735
		      what we get for a volume mounted by WebDAV,                     // C 6736
		      which evidently doesn't support ACLs.  */                       // C 6737
		   || err == ERROR_ACCESS_DENIED                                      // C 6738
		   || err == ERROR_INVALID_FUNCTION)                                  // C 6739
	    { *errno () = ENOTSUP; }                                           // C 6740
	  else                                                                 // C 6741
	    { *errno () = EIO; }                                               // C 6742
	}                                                                      // C 6743
      else if *errno () == 0                                            // C 6744 (C's `!errno`)
	{ *errno () = e; }                                                     // C 6745
    }                                                                   // C 6746
  else if r#type != ACL_TYPE_DEFAULT                                    // C 6747
    { *errno () = EINVAL; }                                             // C 6748

  return psd;                                                           // C 6750
}                                                                       // C 6751

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_set_file (mut fname: *const c_char, r#type: acl_type_t, acl: acl_t) -> c_int // C 6753 (C's `const char *`/`acl_type_t`/`acl_t`)
{                                                                       // C 6755
  let mut old1: TOKEN_PRIVILEGES;                                       // C 6756
  let mut old2: TOKEN_PRIVILEGES;                                       // C 6756
  let mut err: DWORD;                                                   // C 6757
  let mut st: c_int = 0;                                                // C 6758 (C's `int st = 0, retval = -1;`)
  let mut retval: c_int = -1;                                           // C 6758
  let mut flags: SECURITY_INFORMATION = 0;                              // C 6759
  let mut psidOwner: PSID;                                              // C 6760
  let mut psidGroup: PSID;                                              // C 6760
  let mut pacl: PACL;                                                   // C 6761
  let mut dflt: BOOL;                                                   // C 6762
  let mut dacl_present: BOOL;                                           // C 6763
  let mut e: c_int;                                                     // C 6764
  let mut filename: *const c_char;                                      // C 6765

  /* C leaves these uninitialised (PORTING.md 1.5): the two `TOKEN_PRIVILEGES`, `err`,
     `psidOwner`, `psidGroup`, `pacl`, `dflt`, `dacl_present`, `e` and `filename`.  */
  old1 = std::mem::zeroed ();
  old2 = std::mem::zeroed ();
  err = 0;
  psidOwner = std::ptr::null_mut ();
  psidGroup = std::ptr::null_mut ();
  pacl = std::ptr::null_mut ();
  dflt = 0;
  dacl_present = 0;
  e = 0;
  filename = std::ptr::null ();

  if acl_valid (acl) != 0                                               // C 6767
      || (r#type != ACL_TYPE_DEFAULT && r#type != ACL_TYPE_ACCESS)      // C 6768
    {                                                                   // C 6769
      *errno () = EINVAL;                                               // C 6770
      return -1;                                                        // C 6771
    }                                                                   // C 6772

  if r#type == ACL_TYPE_DEFAULT                                         // C 6774
    {                                                                   // C 6775
      *errno () = ENOSYS;                                               // C 6776
      return -1;                                                        // C 6777
    }                                                                   // C 6778

  filename = map_w32_filename (fname, std::ptr::null_mut ());           // C 6780
  if (volume_info.flags & FILE_SUPPORTS_REPARSE_POINTS) != 0            // C 6781
    { fname = chase_symlinks (filename); }                              // C 6782
  else                                                                  // C 6783
    { fname = filename; }                                               // C 6784

  if get_security_descriptor_owner (acl, (&raw mut psidOwner), (&raw mut dflt)) != 0 // C 6786 (C's truth test)
      && psidOwner != std::ptr::null_mut ()                             // C 6787 (C's `&& psidOwner`)
    { flags |= OWNER_SECURITY_INFORMATION; }                            // C 6788
  if get_security_descriptor_group (acl, (&raw mut psidGroup), (&raw mut dflt)) != 0 // C 6789
      && psidGroup != std::ptr::null_mut ()                             // C 6790
    { flags |= GROUP_SECURITY_INFORMATION; }                            // C 6791
  if get_security_descriptor_dacl (acl, (&raw mut dacl_present), (&raw mut pacl), (&raw mut dflt)) != 0 // C 6792
      && dacl_present != 0                                              // C 6793
    { flags |= DACL_SECURITY_INFORMATION; }                             // C 6794
  if flags == 0                                                         // C 6795 (C's `!flags`)
    { return 0; }                                                       // C 6796

  /* According to KB-245153, setting the owner will succeed if either:  // C 6798
     (1) the caller is the user who will be the new owner, and has the  // C 6799
         SE_TAKE_OWNERSHIP privilege, or                                // C 6800
     (2) the caller has the SE_RESTORE privilege, in which case she can // C 6801
         set any valid user or group as the owner                       // C 6802

     We request below both SE_TAKE_OWNERSHIP and SE_RESTORE             // C 6804
     privileges, and disregard any failures in obtaining them.  If      // C 6805
     these privileges cannot be obtained, and do not already exist in   // C 6806
     the calling thread's security token, this function could fail      // C 6807
     with EPERM.  */                                                    // C 6808
  if enable_privilege (SE_TAKE_OWNERSHIP_NAME.as_ptr (), TRUE, (&raw mut old1)) != 0 // C 6809 (C's truth test)
    { st += 1; }                                                        // C 6810 (C's `st++`)
  if enable_privilege (SE_RESTORE_NAME.as_ptr (), TRUE, (&raw mut old2)) != 0 // C 6811
    { st += 1; }                                                        // C 6812 (C's `st++`)

  e = *errno ();                                                        // C 6814 (C's `e = errno;` reads the location)
  *errno () = 0;                                                        // C 6815
  /* SetFileSecurity is deprecated by MS, and sometimes fails when      // C 6816
     DACL inheritance is involved, but it seems to preserve ownership   // C 6817
     better than SetNamedSecurityInfo, which is important e.g., in      // C 6818
     copy-file.  */                                                     // C 6819
  if set_file_security (fname, flags, acl) == 0                         // C 6820 (C's `!`)
    {                                                                   // C 6821
      err = GetLastError ();                                            // C 6822

      if *errno () != ENOTSUP                                           // C 6824
	{ err = set_named_security_info (fname, SE_FILE_OBJECT, flags,         // C 6825 (C's single statement)
					 psidOwner, psidGroup, pacl, std::ptr::null_mut ()); }             // C 6826
    }                                                                   // C 6827
  else                                                                  // C 6828
    { err = ERROR_SUCCESS; }                                            // C 6829
  if err != ERROR_SUCCESS                                               // C 6830
    {                                                                   // C 6831
      if *errno () == ENOTSUP                                           // C 6832 (C's empty `;` statement)
	{ }                                                                    // C 6833
      else if err == ERROR_INVALID_OWNER                                // C 6834
	       || err == ERROR_NOT_ALL_ASSIGNED                                // C 6835
	       || err == ERROR_ACCESS_DENIED                                   // C 6836
	{                                                                      // C 6837
	  /* Maybe the requested ACL and the one the file already has          // C 6838
	     are identical, in which case we can silently ignore the           // C 6839
	     failure.  (And no, Windows doesn't.)  */                          // C 6840
	  let current_acl: acl_t = acl_get_file (fname, ACL_TYPE_ACCESS);      // C 6841 (C's local inside the block)

	  *errno () = EPERM;                                                   // C 6843
	  if ! current_acl.is_null ()                                          // C 6844 (C's truth test)
	    {                                                                  // C 6845
	      let acl_from: *mut c_char = acl_to_text (current_acl, std::ptr::null_mut ()); // C 6846 (C's `char *acl_from`)
	      let acl_to: *mut c_char = acl_to_text (acl, std::ptr::null_mut ()); // C 6847

	      if ! acl_from.is_null () && ! acl_to.is_null () && xstrcasecmp (acl_from, acl_to) == 0 // C 6849 (C's two truth tests)
		{                                                                     // C 6850
		  retval = 0;                                                         // C 6851
		  *errno () = e;                                                      // C 6852
		}                                                                     // C 6853
	      if ! acl_from.is_null ()                                         // C 6854 (C's truth test)
		{ acl_free (acl_from as *mut c_void); }                               // C 6855 (C's `char *` into `void *`)
	      if ! acl_to.is_null ()                                           // C 6856
		{ acl_free (acl_to as *mut c_void); }                                 // C 6857
	      acl_free (current_acl);                                          // C 6858
	    }                                                                  // C 6859
	}                                                                      // C 6860
      else if (err == ERROR_FILE_NOT_FOUND                              // C 6861 (C's outer paren pair)
	       || err == ERROR_PATH_NOT_FOUND                                  // C 6862
	       /* ERROR_INVALID_NAME is what we get if                         // C 6863
		  w32-unicode-filenames is nil and the file cannot be                 // C 6864
		  encoded in the current ANSI codepage. */                            // C 6865
	       || err == ERROR_INVALID_NAME)                                   // C 6866 (C's comment above is the ERR block)
	{ *errno () = ENOENT; }                                                // C 6867
      else                                                              // C 6868
	{ *errno () = EACCES; }                                                // C 6869
    }                                                                   // C 6870
  else                                                                  // C 6871
    {                                                                   // C 6872
      retval = 0;                                                       // C 6873
      *errno () = e;                                                    // C 6874
    }                                                                   // C 6875

  if st != 0                                                            // C 6877 (C's truth test)
    {                                                                   // C 6878
      if st >= 2                                                        // C 6879
	{ restore_privilege ((&raw mut old2)); }                               // C 6880
      restore_privilege ((&raw mut old1));                              // C 6881
      revert_to_self ();                                                // C 6882
    }                                                                   // C 6883

  return retval;                                                        // C 6885
}                                                                       // C 6886

/* Return true if errno value ERRNUM indicates that ACLs are well       // C 6888
   supported on this system.  ERRNUM should be an errno value obtained  // C 6889
   after an ACL-related system call fails.  */                          // C 6890
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_errno_valid (errnum: c_int) -> bool        // C 6891 (C's `bool`)
{                                                                       // C 6893
  match errnum                                                          // C 6894 (C's `switch`)
    {
      EBUSY |                                                           // C 6896
      EINVAL |                                                          // C 6897
      ENOTSUP =>                                                        // C 6898
	{ return false; }                                                      // C 6899
      _ => /* default: */                                               // C 6900
	{ return true; }                                                       // C 6901
    }
}                                                                       // C 6903

/* C 6904-6915: C 6904-6905's blank and form feed, C 6906-6909's `careadlinkat` banner
   (carried verbatim) and C 6910-6914's signature - whose parameter list is the
   `lib/include/careadlinkat.rs` declaration's to the letter (that mirror is in the C 277
   include list, and a definition *shadows* the mirror's glob-imported declaration instead of
   colliding with it - PORTING.md 0.1-10, the `timespec_decls` reading - so this entry point
   needs no mirror gate, unlike slice 13-16's five).  The five names this slice first uses are
   declared here (P-88, `UNVERIFIED`):
   `<winbase.h>`'s `CopyFileW`/`CopyFileA`, `SetFileAttributesW`/`SetFileAttributesA` and
   `SetLastError`, and `<winerror.h>`'s `ERROR_ENCRYPTION_FAILED`.  The rest of the slice is
   `w32_copy_file` (C 6936-7043), C 7045-7049's two banners and the five WINAPI proxy
   wrappers C 7051-7186 holds (`create_toolhelp32_snapshot`, `process32_first`/`_next`,
   `open_thread_token`, `impersonate_self`, `revert_to_self`) - all of them the `LOAD_DLL_FN!`
   shape slice 1's `open_process_token` established.  */
/// C: WINBASEAPI BOOL WINAPI CopyFileW (LPCWSTR, LPCWSTR, BOOL);
/// C: WINBASEAPI BOOL WINAPI CopyFileA (LPCSTR, LPCSTR, BOOL);
/// C: WINBASEAPI BOOL WINAPI SetFileAttributesW (LPCWSTR, DWORD);
/// C: WINBASEAPI BOOL WINAPI SetFileAttributesA (LPCSTR, DWORD);
/// C: WINBASEAPI void WINAPI SetLastError (DWORD dwErrCode);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn CopyFileW (lpExistingFileName: LPCWSTR, lpNewFileName: LPCWSTR,
		    bFailIfExists: BOOL) -> BOOL;
  pub fn CopyFileA (lpExistingFileName: LPCSTR, lpNewFileName: LPCSTR,
		    bFailIfExists: BOOL) -> BOOL;
  pub fn SetFileAttributesW (lpFileName: LPCWSTR, dwFileAttributes: DWORD) -> BOOL;
  pub fn SetFileAttributesA (lpFileName: LPCSTR, dwFileAttributes: DWORD) -> BOOL;
  pub fn SetLastError (dwErrCode: DWORD);
}
/// C: #define ERROR_ENCRYPTION_FAILED 6000L
#[cfg (feature = "windowsnt")]
pub const ERROR_ENCRYPTION_FAILED: DWORD = 6000;

/* MS-Windows version of careadlinkat (cf. ../lib/careadlinkat.c).  We  // C 6906
   have a fixed max size for file names, so we don't need the kind of   // C 6907
   alloc/malloc/realloc dance the gnulib version does.  We also don't   // C 6908
   support FD-relative symlinks.  */                                    // C 6909
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn careadlinkat (fd: c_int, filename: *const c_char, // C 6910
				    buffer: *mut c_char, buffer_size: size_t,                       // C 6911
				    alloc: *const allocator,                                        // C 6913
				    preadlinkat: Option<unsafe extern "C" fn (c_int, *const c_char, // C 6914
							     *mut c_char, size_t)                                        // C 6914
							  -> ssize_t>)                                                   // C 6914
  -> *mut c_char                                                        // C 6910
{                                                                       // C 6915
  let mut linkname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 6916
  let mut link_size: ssize_t;                                           // C 6917

  /* C leaves `link_size` uninitialised (PORTING.md 1.5).  */
  link_size = 0;

  link_size = (preadlinkat.unwrap ()) (fd, filename, linkname.as_mut_ptr (), core::mem::size_of_val (&linkname)); // C 6919 (C's call-through)

  if link_size > 0                                                      // C 6921
    {                                                                   // C 6922
      let mut retval: *mut c_char = buffer;                             // C 6923

      *linkname.as_mut_ptr ().offset (link_size as isize) = 0;          // C 6925 (C's `linkname[link_size++] = '\0'`)
      link_size += 1;                                                   // C 6925 (C's post-increment)
      if link_size > buffer_size as ssize_t                             // C 6926 (C's `size_t` comparison)
	{
    /* C 6927 selects the allocator with a ternary over two **function pointers**
       (`alloc ? alloc->allocate : xmalloc`); the port names the selection in a local of
       the mirror's field type and calls it the same way C does - blindly.  */
	  let allocate: Option<unsafe extern "C" fn (usize) -> *mut c_void> =  // C 6927
	    if alloc.is_null () { Some (xmalloc) } else { (*alloc).allocate }; // C 6927
	  retval = (allocate.unwrap ()) (link_size as usize) as *mut c_char;   // C 6927 (C's call)
	}
      if ! retval.is_null ()                                            // C 6928 (C's truth test)
	{ memcpy (retval as *mut c_void, linkname.as_ptr () as *const c_void, link_size as usize); } // C 6929

      return retval;                                                    // C 6931
    }                                                                   // C 6932
  return std::ptr::null_mut ();                                         // C 6933 (C's `NULL`)
}                                                                       // C 6934

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_copy_file (from: *const c_char, to: *const c_char, // C 6936
				   mut keep_time: c_int, mut preserve_ownership: c_int, copy_acls: c_int) -> c_int // C 6937
{                                                                       // C 6939
  let mut acl: acl_t = std::ptr::null_mut ();                           // C 6940 (C's `= NULL`)
  let mut copy_result: BOOL;                                            // C 6941
  let mut from_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6942 (C's `wchar_t`)
  let mut to_w: [c_ushort; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6942
  let mut from_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize]; // C 6943
  let mut to_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   // C 6943

  /* C leaves `copy_result` uninitialised (PORTING.md 1.5).  */
  copy_result = 0;

  /* We ignore preserve_ownership for now.  */                          // C 6945
  preserve_ownership = preserve_ownership;                              // C 6946 (C's unused-parameter silencer; the binding is `mut` for it)

  if copy_acls != 0                                                     // C 6948 (C's truth test)
    {                                                                   // C 6949
      acl = acl_get_file (from, ACL_TYPE_ACCESS);                       // C 6950
      if acl.is_null () && acl_errno_valid (*errno ())                  // C 6951 (C's `== NULL && acl_errno_valid (errno)`)
	{ return -2; }
    }                                                                   // C 6953
  if w32_unicode_filenames                                              // C 6954
    {                                                                   // C 6955
      filename_to_utf16 (from, from_w.as_mut_ptr ());                   // C 6956
      filename_to_utf16 (to, to_w.as_mut_ptr ());                       // C 6957
      copy_result = CopyFileW (from_w.as_ptr (), to_w.as_ptr (), FALSE); // C 6958
    }                                                                   // C 6959
  else                                                                  // C 6960
    {                                                                   // C 6961
      filename_to_ansi (from, from_a.as_mut_ptr ());                    // C 6962
      filename_to_ansi (to, to_a.as_mut_ptr ());                        // C 6963
      copy_result = CopyFileA (from_a.as_ptr (), to_a.as_ptr (), FALSE); // C 6964
    }                                                                   // C 6965
  if copy_result == 0                                                   // C 6966 (C's `!copy_result`)
    {                                                                   // C 6967
      /* CopyFile doesn't set errno when it fails.  By far the most     // C 6968
	 "popular" reason is that the target is read-only.  */                 // C 6969
      let err: DWORD = GetLastError ();                                 // C 6970

      match err                                                         // C 6972
	{
	  ERROR_FILE_NOT_FOUND =>                                              // C 6974
	    { *errno () = ENOENT; }                                            // C 6975
	  ERROR_ACCESS_DENIED =>                                               // C 6977
	    { *errno () = EACCES; }                                            // C 6978
	  ERROR_ENCRYPTION_FAILED =>                                           // C 6980
	    { *errno () = EIO; }                                               // C 6981
	  _ => /* default: */                                                  // C 6983
	    { *errno () = EPERM; }                                             // C 6984
	}

      if ! acl.is_null ()                                               // C 6988 (C's truth test)
	{ acl_free (acl); }                                                    // C 6989
      return -1;                                                        // C 6990
    }                                                                   // C 6991
  /* CopyFile retains the timestamp by default.  However, see           // C 6992
     "Community Additions" for CopyFile: it sounds like that is not     // C 6993
     entirely true.  Testing on Windows XP confirms that modified time  // C 6994
     is copied, but creation and last-access times are not.             // C 6995
     FIXME?  */                                                         // C 6996
  else if keep_time == 0                                                // C 6997 (C's `!keep_time`)
    {                                                                   // C 6998
      let mut tnow: [timespec; 2] = std::mem::zeroed ();                // C 6999
      let mut attributes: DWORD;                                        // C 7000

      /* C leaves `attributes` uninitialised (PORTING.md 1.5).  */
      attributes = 0;

      /* C 7002's chained `tnow[0] = tnow[1] = current_timespec ();` calls the function once.  */
      let ts: timespec = current_timespec ();                           // C 7002 (the single call)
      tnow[0] = ts; tnow[1] = ts;                                       // C 7002
      if w32_unicode_filenames                                          // C 7003
	{
	  /* Ensure file is writable while its times are set.  */              // C 7005
	  attributes = GetFileAttributesW (to_w.as_ptr ());                    // C 7006
	  SetFileAttributesW (to_w.as_ptr (), attributes & ! FILE_ATTRIBUTE_READONLY); // C 7007 (C's `~`)
	  if utimensat (AT_FDCWD, to, tnow.as_mut_ptr (), 0) != 0              // C 7008 (C's truth test)
	    {
	      /* Restore original attributes.  */                              // C 7010
	      SetFileAttributesW (to_w.as_ptr (), attributes);                 // C 7011
	      if ! acl.is_null ()                                              // C 7012
		{ acl_free (acl); }                                                   // C 7013
	      return -3;                                                       // C 7014
	    }
	  /* Restore original attributes.  */                                  // C 7016
	  SetFileAttributesW (to_w.as_ptr (), attributes);                     // C 7017
	}
	else                                                                   // C 7019
	  {
	    attributes = GetFileAttributesA (to_a.as_ptr ());                  // C 7021
	    SetFileAttributesA (to_a.as_ptr (), attributes & ! FILE_ATTRIBUTE_READONLY); // C 7022 (C's `~`)
	    if utimensat (AT_FDCWD, to, tnow.as_mut_ptr (), 0) != 0            // C 7023
	      {
		SetFileAttributesA (to_a.as_ptr (), attributes);                      // C 7025
		if ! acl.is_null ()                                                   // C 7026
		  { acl_free (acl); }                                                 // C 7027
		return -3;                                                            // C 7028
	      }
	    SetFileAttributesA (to_a.as_ptr (), attributes);                   // C 7030
	  }
    }                                                                   // C 7032
  if ! acl.is_null ()                                                   // C 7033 (C's `!= NULL`)
    {                                                                   // C 7034
      let fail: bool =                                                  // C 7035
	acl_set_file (to, ACL_TYPE_ACCESS, acl) != 0;                          // C 7036
      acl_free (acl);                                                   // C 7037
      if fail && acl_errno_valid (*errno ())                            // C 7038 (C's `&& acl_errno_valid (errno)`)
	{ return -4; }                                                         // C 7039
    }                                                                   // C 7040

  return 0;                                                             // C 7042
}                                                                       // C 7043


/* Support for browsing other processes and their attributes.  See      // C 7046
   process.c for the Lisp bindings.  */                                 // C 7047

/* Helper wrapper functions.  */                                        // C 7049
/// C: static HANDLE WINAPI
/// C: create_toolhelp32_snapshot (DWORD Flags, DWORD Ignored)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn create_toolhelp32_snapshot (Flags: DWORD, Ignored: DWORD) -> HANDLE // C 7051
{                                                                       // C 7053
  static mut s_pfn_Create_Toolhelp32_Snapshot: CreateToolhelp32Snapshot_Proc = None; // C 7054 (C's function-local `static … = NULL`)

  if g_b_init_create_toolhelp32_snapshot == 0                           // C 7056
    {                                                                   // C 7057
      g_b_init_create_toolhelp32_snapshot = 1;                          // C 7058
      s_pfn_Create_Toolhelp32_Snapshot = std::mem::transmute::<VOIDFNPTR, CreateToolhelp32Snapshot_Proc> // C 7059
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7060
			c"CreateToolhelp32Snapshot".as_ptr ()));                             // C 7061
    }                                                                   // C 7062
  if s_pfn_Create_Toolhelp32_Snapshot.is_none ()                        // C 7063 (C's `== NULL`)
    {                                                                   // C 7064
      return INVALID_HANDLE_VALUE;                                      // C 7065
    }                                                                   // C 7066
  return (s_pfn_Create_Toolhelp32_Snapshot.unwrap ()) (Flags, Ignored); // C 7067 (C's call-through)
}                                                                       // C 7068

/// C: static BOOL WINAPI
/// C: process32_first (HANDLE hSnapshot, LPPROCESSENTRY32 lppe)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn process32_first (hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL // C 7070
{                                                                       // C 7072
  static mut s_pfn_Process32_First: Process32First_Proc = None;         // C 7073 (C's function-local `static … = NULL`)

  if g_b_init_process32_first == 0                                      // C 7075
    {                                                                   // C 7076
      g_b_init_process32_first = 1;                                     // C 7077
      s_pfn_Process32_First = std::mem::transmute::<VOIDFNPTR, Process32First_Proc> // C 7078
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7079
			c"Process32First".as_ptr ()));                                       // C 7080
    }                                                                   // C 7081
  if s_pfn_Process32_First.is_none ()                                   // C 7082 (C's `== NULL`)
    {                                                                   // C 7083
      return FALSE;                                                     // C 7084
    }                                                                   // C 7085
  return (s_pfn_Process32_First.unwrap ()) (hSnapshot, lppe);           // C 7086 (C's call-through)
}                                                                       // C 7087

/// C: static BOOL WINAPI
/// C: process32_next (HANDLE hSnapshot, LPPROCESSENTRY32 lppe)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn process32_next (hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL // C 7089
{                                                                       // C 7091
  static mut s_pfn_Process32_Next: Process32Next_Proc = None;           // C 7092 (C's function-local `static … = NULL`)

  if g_b_init_process32_next == 0                                       // C 7094
    {                                                                   // C 7095
      g_b_init_process32_next = 1;                                      // C 7096
      s_pfn_Process32_Next = std::mem::transmute::<VOIDFNPTR, Process32Next_Proc> // C 7097
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7098
			c"Process32Next".as_ptr ()));                                        // C 7099
    }                                                                   // C 7100
  if s_pfn_Process32_Next.is_none ()                                    // C 7101 (C's `== NULL`)
    {                                                                   // C 7102
      return FALSE;                                                     // C 7103
    }                                                                   // C 7104
  return (s_pfn_Process32_Next.unwrap ()) (hSnapshot, lppe);            // C 7105 (C's call-through)
}                                                                       // C 7106

/// C: static BOOL WINAPI
/// C: open_thread_token (HANDLE ThreadHandle,
/// C: 		   DWORD DesiredAccess,
/// C: 		   BOOL OpenAsSelf,
/// C: 		   PHANDLE TokenHandle)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn open_thread_token (ThreadHandle: HANDLE, DesiredAccess: DWORD, // C 7108
				      OpenAsSelf: BOOL, TokenHandle: PHANDLE) -> BOOL               // C 7110
{                                                                       // C 7113
  static mut s_pfn_Open_Thread_Token: OpenThreadToken_Proc = None;      // C 7114 (C's function-local `static … = NULL`)
  let mut hm_advapi32: HMODULE = std::ptr::null_mut ();                 // C 7115 (C's `= NULL`)
  if is_windows_9x () == TRUE                                           // C 7116
    {                                                                   // C 7117
      SetLastError (ERROR_NOT_SUPPORTED);                               // C 7118
      return FALSE;                                                     // C 7119
    }                                                                   // C 7120
  if g_b_init_open_thread_token == 0                                    // C 7121
    {                                                                   // C 7122
      g_b_init_open_thread_token = 1;                                   // C 7123
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());            // C 7124
      s_pfn_Open_Thread_Token = std::mem::transmute::<VOIDFNPTR, OpenThreadToken_Proc> // C 7125
        (get_proc_addr (hm_advapi32, c"OpenThreadToken".as_ptr ()));    // C 7126
    }                                                                   // C 7127
  if s_pfn_Open_Thread_Token.is_none ()                                 // C 7128 (C's `== NULL`)
    {                                                                   // C 7129
      SetLastError (ERROR_NOT_SUPPORTED);                               // C 7130
      return FALSE;                                                     // C 7131
    }                                                                   // C 7132
  return (s_pfn_Open_Thread_Token.unwrap ()) (ThreadHandle, DesiredAccess, OpenAsSelf, TokenHandle); // C 7133-7139 (C's call-through)
}                                                                       // C 7140

/// C: static BOOL WINAPI
/// C: impersonate_self (SECURITY_IMPERSONATION_LEVEL ImpersonationLevel)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn impersonate_self (ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL) -> BOOL // C 7142
{                                                                       // C 7144
  static mut s_pfn_Impersonate_Self: ImpersonateSelf_Proc = None;       // C 7145 (C's function-local `static … = NULL`)
  let mut hm_advapi32: HMODULE = std::ptr::null_mut ();                 // C 7146 (C's `= NULL`)
  if is_windows_9x () == TRUE                                           // C 7147
    {                                                                   // C 7148
      return FALSE;                                                     // C 7149
    }                                                                   // C 7150
  if g_b_init_impersonate_self == 0                                     // C 7151
    {                                                                   // C 7152
      g_b_init_impersonate_self = 1;                                    // C 7153
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());            // C 7154
      s_pfn_Impersonate_Self = std::mem::transmute::<VOIDFNPTR, ImpersonateSelf_Proc> // C 7155
        (get_proc_addr (hm_advapi32, c"ImpersonateSelf".as_ptr ()));    // C 7156
    }                                                                   // C 7157
  if s_pfn_Impersonate_Self.is_none ()                                  // C 7158 (C's `== NULL`)
    {                                                                   // C 7159
      return FALSE;                                                     // C 7160
    }                                                                   // C 7161
  return s_pfn_Impersonate_Self.unwrap () (ImpersonationLevel);         // C 7162 (C's call-through)
}                                                                       // C 7163

/// C: static BOOL WINAPI
/// C: revert_to_self (void)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn revert_to_self () -> BOOL                     // C 7165
{                                                                       // C 7167
  static mut s_pfn_Revert_To_Self: RevertToSelf_Proc = None;            // C 7168 (C's function-local `static … = NULL`)
  let mut hm_advapi32: HMODULE = std::ptr::null_mut ();                 // C 7169 (C's `= NULL`)
  if is_windows_9x () == TRUE                                           // C 7170
    {                                                                   // C 7171
      return FALSE;                                                     // C 7172
    }                                                                   // C 7173
  if g_b_init_revert_to_self == 0                                       // C 7174
    {                                                                   // C 7175
      g_b_init_revert_to_self = 1;                                      // C 7176
      hm_advapi32 = LoadLibrary (c"Advapi32.dll".as_ptr ());            // C 7177
      s_pfn_Revert_To_Self = std::mem::transmute::<VOIDFNPTR, RevertToSelf_Proc> // C 7178
        (get_proc_addr (hm_advapi32, c"RevertToSelf".as_ptr ()));       // C 7179
    }                                                                   // C 7180
  if s_pfn_Revert_To_Self.is_none ()                                    // C 7181 (C's `== NULL`)
    {                                                                   // C 7182
      return FALSE;                                                     // C 7183
    }                                                                   // C 7184
  return s_pfn_Revert_To_Self.unwrap () ();                             // C 7185 (C's call-through)
}                                                                       // C 7186

/* C 7187-7192: C 7187's blank, C 7188-7191's `static BOOL WINAPI get_process_memory_info`
   declaration (carried as the doc lines below, the slice-17 device) and C 7192's `{`.  This
   slice carries the process/memory probe group the C 7046 banner introduces: the four WINAPI
   proxies C 7188-7284 (`get_process_memory_info`, `get_process_working_set_size`,
   `global_memory_status`, `global_memory_status_ex`), C 7286's `list_system_processes` (the
   unit's first `PROCESSENTRY32` *by value* - the record is spelled at C 430's block above),
   C 7315's `enable_privilege` and C 7352's `restore_privilege` - the two helpers whose calls
   have been unresolved names since slice 16 - and C 7373's `ltime` with C 7382's
   `U64_TO_LISP_TIME` macro (one of the four `missing macro` rows).  The names this slice first
   uses are declared here (P-88, `UNVERIFIED`): `<tlhelp32.h>`'s `TH32CS_SNAPPROCESS`,
   `<winnt.h>`'s `TOKEN_ADJUST_PRIVILEGES`/`SE_PRIVILEGE_ENABLED`/`SecurityImpersonation`,
   `<winerror.h>`'s `ERROR_NO_TOKEN`, and `<securitybaseapi.h>`'s `AdjustTokenPrivileges`,
   `LookupPrivilegeValue` and `GetCurrentThread`.  */
/// C: #define TH32CS_SNAPPROCESS 0x00000002  (`<tlhelp32.h>`)
#[cfg (feature = "windowsnt")]
pub const TH32CS_SNAPPROCESS: DWORD = 0x00000002;
/// C: #define TOKEN_ADJUST_PRIVILEGES (0x0020)  (`<winnt.h>`; `TOKEN_QUERY` is C 1815's)
#[cfg (feature = "windowsnt")]
pub const TOKEN_ADJUST_PRIVILEGES: DWORD = 0x0020;
/// C: #define SE_PRIVILEGE_ENABLED (0x00000002L)  (`<winnt.h>`)
#[cfg (feature = "windowsnt")]
pub const SE_PRIVILEGE_ENABLED: DWORD = 0x00000002;
/// C: typedef enum _SECURITY_IMPERSONATION_LEVEL { SecurityAnonymous, SecurityIdentification,
/// C:   SecurityImpersonation, SecurityDelegation } SECURITY_IMPERSONATION_LEVEL;  C 7331
/// names the third member, so it is declared at its first use (the SDK enums above are
/// carried as `c_int` aliases, and this one's members are the three that follow).
#[cfg (feature = "windowsnt")]
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = 2;
/// C: #define ERROR_NO_TOKEN 1008L
#[cfg (feature = "windowsnt")]
pub const ERROR_NO_TOKEN: DWORD = 1008;
/// C: WINBASEAPI BOOL WINAPI AdjustTokenPrivileges (HANDLE TokenHandle, BOOL DisableAllPrivileges,
/// C:   PTOKEN_PRIVILEGES NewState, DWORD BufferLength, PTOKEN_PRIVILEGES PreviousState,
/// C:   PDWORD ReturnLength);
/// C: WINADVAPI BOOL WINAPI LookupPrivilegeValueA (LPCSTR lpSystemName, LPCSTR lpName, PLUID lpLuid);
/// C: WINBASEAPI HANDLE WINAPI GetCurrentThread (VOID);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn AdjustTokenPrivileges (TokenHandle: HANDLE, DisableAllPrivileges: BOOL,
				NewState: *mut TOKEN_PRIVILEGES, BufferLength: DWORD,
				PreviousState: *mut TOKEN_PRIVILEGES, ReturnLength: PDWORD) -> BOOL;
  pub fn LookupPrivilegeValueA (lpSystemName: LPCSTR, lpName: LPCSTR, lpLuid: *mut LUID) -> BOOL;
  pub fn GetCurrentThread () -> HANDLE;
}

/// C: static BOOL WINAPI
/// C: get_process_memory_info (HANDLE h_proc,
/// C: 			 PPROCESS_MEMORY_COUNTERS mem_counters,
/// C: 			 DWORD bufsize)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_process_memory_info (h_proc: HANDLE,      // C 7188
					       mem_counters: PPROCESS_MEMORY_COUNTERS,                     // C 7190
					       bufsize: DWORD) -> BOOL                                     // C 7191
{                                                                       // C 7192
  static mut s_pfn_Get_Process_Memory_Info: GetProcessMemoryInfo_Proc = None; // C 7193 (C's function-local `static … = NULL`)
  let mut hm_psapi: HMODULE = std::ptr::null_mut ();                    // C 7194 (C's `= NULL`)
  if is_windows_9x () == TRUE                                           // C 7195
    {                                                                   // C 7196
      return FALSE;                                                     // C 7197
    }                                                                   // C 7198
  if g_b_init_get_process_memory_info == 0                              // C 7199
    {                                                                   // C 7200
      g_b_init_get_process_memory_info = 1;                             // C 7201
      hm_psapi = LoadLibrary (c"Psapi.dll".as_ptr ());                  // C 7202
      if ! hm_psapi.is_null ()                                          // C 7203 (C's truth test)
	{
	  s_pfn_Get_Process_Memory_Info = std::mem::transmute::<VOIDFNPTR, GetProcessMemoryInfo_Proc> // C 7204
	    (get_proc_addr (hm_psapi, c"GetProcessMemoryInfo".as_ptr ()));     // C 7205
	}
    }                                                                   // C 7206
  if s_pfn_Get_Process_Memory_Info.is_none ()                           // C 7207 (C's `== NULL`)
    {                                                                   // C 7208
      return FALSE;                                                     // C 7209
    }                                                                   // C 7210
  return (s_pfn_Get_Process_Memory_Info.unwrap ()) (h_proc, mem_counters, bufsize); // C 7211 (C's call-through)
}                                                                       // C 7212

/// C: static BOOL WINAPI
/// C: get_process_working_set_size (HANDLE h_proc,
/// C: 			      PSIZE_T minrss,
/// C: 			      PSIZE_T maxrss)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn get_process_working_set_size (h_proc: HANDLE, // C 7214
						      minrss: *mut SIZE_T,                                        // C 7216 (C's `PSIZE_T`, the 1060 typedef's own spelling)
						      maxrss: *mut SIZE_T) -> BOOL                                // C 7217
{                                                                       // C 7218
  static mut s_pfn_Get_Process_Working_Set_Size: GetProcessWorkingSetSize_Proc = None; // C 7219-7220 (C's wrapped function-local `static … = NULL`)

  if is_windows_9x () == TRUE                                           // C 7222
    {                                                                   // C 7223
      return FALSE;                                                     // C 7224
    }                                                                   // C 7225
  if g_b_init_get_process_working_set_size == 0                         // C 7226
    {                                                                   // C 7227
      g_b_init_get_process_working_set_size = 1;                        // C 7228
      s_pfn_Get_Process_Working_Set_Size = std::mem::transmute::<VOIDFNPTR, GetProcessWorkingSetSize_Proc> // C 7229
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7230
			c"GetProcessWorkingSetSize".as_ptr ()));                             // C 7231
    }                                                                   // C 7232
  if s_pfn_Get_Process_Working_Set_Size.is_none ()                      // C 7233 (C's `== NULL`)
    {                                                                   // C 7234
      return FALSE;                                                     // C 7235
    }                                                                   // C 7236
  return (s_pfn_Get_Process_Working_Set_Size.unwrap ()) (h_proc, minrss, maxrss); // C 7237 (C's call-through)
}                                                                       // C 7238

/// C: static BOOL WINAPI
/// C: global_memory_status (MEMORYSTATUS *buf)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn global_memory_status (buf: LPMEMORYSTATUS) -> BOOL // C 7240
{                                                                       // C 7242
  static mut s_pfn_Global_Memory_Status: GlobalMemoryStatus_Proc = None; // C 7243 (C's function-local `static … = NULL`)

  if is_windows_9x () == TRUE                                           // C 7245
    {                                                                   // C 7246
      return FALSE;                                                     // C 7247
    }                                                                   // C 7248
  if g_b_init_global_memory_status == 0                                 // C 7249
    {                                                                   // C 7250
      g_b_init_global_memory_status = 1;                                // C 7251
      s_pfn_Global_Memory_Status = std::mem::transmute::<VOIDFNPTR, GlobalMemoryStatus_Proc> // C 7252
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7253
			c"GlobalMemoryStatus".as_ptr ()));                                   // C 7254
    }                                                                   // C 7255
  if s_pfn_Global_Memory_Status.is_none ()                              // C 7256 (C's `== NULL`)
    {                                                                   // C 7257
      return FALSE;                                                     // C 7258
    }                                                                   // C 7259
  return (s_pfn_Global_Memory_Status.unwrap ()) (buf);                  // C 7260 (C's call-through)
}                                                                       // C 7261

/// C: static BOOL WINAPI
/// C: global_memory_status_ex (MEMORY_STATUS_EX *buf)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn global_memory_status_ex (buf: *mut MEMORY_STATUS_EX) -> BOOL // C 7263
{                                                                       // C 7265
  static mut s_pfn_Global_Memory_Status_Ex: GlobalMemoryStatusEx_Proc = None; // C 7266 (C's function-local `static … = NULL`)

  if is_windows_9x () == TRUE                                           // C 7268
    {                                                                   // C 7269
      return FALSE;                                                     // C 7270
    }                                                                   // C 7271
  if g_b_init_global_memory_status_ex == 0                              // C 7272
    {                                                                   // C 7273
      g_b_init_global_memory_status_ex = 1;                             // C 7274
      s_pfn_Global_Memory_Status_Ex = std::mem::transmute::<VOIDFNPTR, GlobalMemoryStatusEx_Proc> // C 7275
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),           // C 7276
			c"GlobalMemoryStatusEx".as_ptr ()));                                 // C 7277
    }                                                                   // C 7278
  if s_pfn_Global_Memory_Status_Ex.is_none ()                           // C 7279 (C's `== NULL`)
    {                                                                   // C 7280
      return FALSE;                                                     // C 7281
    }                                                                   // C 7282
  return (s_pfn_Global_Memory_Status_Ex.unwrap ()) (buf);               // C 7283 (C's call-through)
}                                                                       // C 7284

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_system_processes () -> Lisp_Object        // C 7286
{                                                                       // C 7288
  let mut proclist: Lisp_Object = Qnil ();                              // C 7289 (C's `= Qnil`)
  let mut h_snapshot: HANDLE;                                           // C 7290

  /* C leaves `h_snapshot` uninitialised (PORTING.md 1.5).  */
  h_snapshot = std::ptr::null_mut ();

  h_snapshot = create_toolhelp32_snapshot (TH32CS_SNAPPROCESS, 0);      // C 7292

  if h_snapshot != INVALID_HANDLE_VALUE                                 // C 7294
    {                                                                   // C 7295
      let mut proc_entry: PROCESSENTRY32;                               // C 7296
      let mut proc_id: DWORD;                                           // C 7297
      let mut res: BOOL;                                                // C 7298

      /* C leaves the three uninitialised (PORTING.md 1.5).  */
      proc_entry = std::mem::zeroed ();
      proc_id = 0;
      res = 0;

      proc_entry.dwSize = core::mem::size_of::<PROCESSENTRY32> () as DWORD; // C 7300 (C's `sizeof (PROCESSENTRY32)`)
      /* C 7301-7302's `for (res = process32_first (…); res; res = process32_next (…))` is the
	 port's `while` with the loop-carried assignments (the `w32_cached_id` device).  */
      res = process32_first (h_snapshot, (&raw mut proc_entry));        // C 7301
      while res != 0                                                    // C 7301 (C's `res` condition)
	{
	  proc_id = proc_entry.th32ProcessID;                                  // C 7304
	  proclist = Fcons (INT_TO_INTEGER (proc_id as EMACS_INT), proclist);  // C 7305 (C's `INT_TO_INTEGER (proc_id)`)
	  res = process32_next  (h_snapshot, (&raw mut proc_entry));           // C 7302
	}

      CloseHandle (h_snapshot);                                         // C 7308
      proclist = Fnreverse (proclist);                                  // C 7309
    }                                                                   // C 7310

  return proclist;                                                      // C 7312
}                                                                       // C 7313

#[cfg (feature = "windowsnt")]
unsafe fn enable_privilege (priv_name: LPCTSTR, enable_p: BOOL, old_priv: *mut TOKEN_PRIVILEGES) -> c_int // C 7315 (C's `static int`)
{                                                                       // C 7317
  let mut r#priv: TOKEN_PRIVILEGES;                                     // C 7318
  let mut priv_size: DWORD = core::mem::size_of_val (&r#priv) as DWORD; // C 7319 (C's `sizeof (r#priv)`)
  let mut opriv_size: DWORD = core::mem::size_of_val (&*old_priv) as DWORD; // C 7320 (C's `sizeof (*old_priv)`)
  let mut h_token: HANDLE = std::ptr::null_mut ();                      // C 7321 (C's `= NULL`)
  let mut h_thread: HANDLE = GetCurrentThread ();                       // C 7322
  let mut ret_val: c_int = 0;                                           // C 7323
  let mut res: BOOL;                                                    // C 7324

  /* C leaves `r#priv` and `res` uninitialised (PORTING.md 1.5).  */
  r#priv = std::mem::zeroed ();
  res = 0;

  res = open_thread_token (h_thread,                                    // C 7326
			   TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,                            // C 7327
			   FALSE, (&raw mut h_token));                                       // C 7328
  if res == 0 && GetLastError () == ERROR_NO_TOKEN                      // C 7329 (C's `!res`)
    {                                                                   // C 7330
      if impersonate_self (SecurityImpersonation) != 0                  // C 7331 (C's truth test)
	  { res = open_thread_token (h_thread,                                 // C 7332
				   TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,                           // C 7333
				   FALSE, (&raw mut h_token)); }                                    // C 7334
    }                                                                   // C 7335
  if res != 0                                                           // C 7336 (C's truth test)
    {                                                                   // C 7337
      r#priv.PrivilegeCount = 1;                                        // C 7338
      r#priv.Privileges[0].Attributes = if enable_p != 0 { SE_PRIVILEGE_ENABLED } else { 0 }; // C 7339 (C's ternary)
      LookupPrivilegeValueA (std::ptr::null (), priv_name, (&raw mut r#priv.Privileges[0].Luid)); // C 7340 (C's `LookupPrivilegeValue (NULL, …)`)
      if AdjustTokenPrivileges (h_token, FALSE, (&raw mut r#priv), priv_size, // C 7341
				 old_priv, (&raw mut opriv_size)) != 0                              // C 7342
	  && GetLastError () != ERROR_NOT_ALL_ASSIGNED                         // C 7343
	{ ret_val = 1; }                                                       // C 7344
    }                                                                   // C 7345
  if ! h_token.is_null ()                                               // C 7346 (C's truth test)
    { CloseHandle (h_token); }                                          // C 7347

  return ret_val;                                                       // C 7349
}                                                                       // C 7350

#[cfg (feature = "windowsnt")]
unsafe fn restore_privilege (r#priv: *mut TOKEN_PRIVILEGES) -> c_int    // C 7352 (C's `static int`)
{                                                                       // C 7354
  let mut priv_size: DWORD = core::mem::size_of_val (&*r#priv) as DWORD; // C 7355 (C's `sizeof (*r#priv)`)
  let mut h_token: HANDLE = std::ptr::null_mut ();                      // C 7356 (C's `= NULL`)
  let mut ret_val: c_int = 0;                                           // C 7357

  if open_thread_token (GetCurrentThread (),                            // C 7359
			 TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,                              // C 7360
			 FALSE, (&raw mut h_token)) != 0                                     // C 7361 (C's truth test)
    {                                                                   // C 7362
      if AdjustTokenPrivileges (h_token, FALSE, r#priv, priv_size, std::ptr::null_mut (), std::ptr::null_mut ()) != 0 // C 7363 (C's `&&` first half; C passes NULL twice)
	  && GetLastError () != ERROR_NOT_ALL_ASSIGNED                         // C 7364
	{ ret_val = 1; }                                                       // C 7365
    }                                                                   // C 7366
  if ! h_token.is_null ()                                               // C 7367 (C's truth test)
    { CloseHandle (h_token); }                                          // C 7368

  return ret_val;                                                       // C 7370
}                                                                       // C 7371

#[cfg (feature = "windowsnt")]
unsafe fn ltime (time_100ns: ULONGLONG) -> Lisp_Object                  // C 7373 (C's `static Lisp_Object`)
{                                                                       // C 7375
  let time_sec: ULONGLONG = time_100ns / 10000000;                      // C 7376
  let subsec: c_int = (time_100ns % 10000000) as c_int;                 // C 7377 (C's `int subsec`)
  return list4i ((time_sec >> 16) as EMACS_INT, (time_sec & 0xffff) as EMACS_INT, // C 7378 (the `EMACS_INT` stores)
		  (subsec / 10) as EMACS_INT, (subsec % 10 * 100000) as EMACS_INT);   // C 7379
}                                                                       // C 7380

/// C: #define U64_TO_LISP_TIME(time) ltime (time)
#[cfg (feature = "windowsnt")]
macro_rules! U64_TO_LISP_TIME {
  ($time:expr) => { ltime ($time) };
}

/* C 7383-7388: C 7383's blank, C 7384-7387's `static int process_times` declaration (the
   doc lines below, the slice-17 device for carried *code* text) and C 7388's `{`.  This
   slice carries `process_times` (7384-7432, the unit's first user of C 7382's
   `U64_TO_LISP_TIME` macro) and C 7434's `system_process_attributes` (7434-7716, the
   `process-attributes` entry point: C 7453-7457's records, C 7474-7503's snapshot walk,
   C 7511-7526's `OpenProcess`/`SE_DEBUG_NAME` retry, C 7527-7604's token/ACL-group reads,
   C 7605-7644's fallbacks, C 7657-7699's three memory probes and C 7701-7709's six
   `process_times` attributes).  The two names this slice first uses are declared here (P-88,
   `UNVERIFIED`): `<winnt.h>`'s `PROCESS_QUERY_INFORMATION` (C 1528 carries `PROCESS_VM_READ`
   already) and `<winbase.h>`'s `SE_DEBUG_NAME` (the `&CStr` device of slice 14's
   `SE_BACKUP_NAME`).  C 7456's `MEMORYSTATUS memst` is the second SDK record this lane has
   had to spell (the C 513-517 block above), and `Vlocale_coding_system`/
   `code_convert_string_norecord` come from C 274-275's includes.  */
/// C: #define PROCESS_QUERY_INFORMATION (0x0400)  (`<winnt.h>`)
#[cfg (feature = "windowsnt")]
pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
/// C: #define SE_DEBUG_NAME TEXT("SeDebugPrivilege")  (`<winbase.h>`; the `unicode` arm is
/// owed as for the three `SE_*_NAME` constants above)
#[cfg (feature = "windowsnt")]
pub const SE_DEBUG_NAME: &core::ffi::CStr = c"SeDebugPrivilege";

/// C: WINBASEAPI void WINAPI GetSystemTimeAsFileTime (LPFILETIME lpSystemTimeAsFileTime);
/// (`LPFILETIME` is C 566's own alias; the entry point itself is `femacs/src/w32proc.rs`'s, so
/// it is declared here for this unit's use - C 7398 is its first reader.
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetSystemTimeAsFileTime (lpSystemTimeAsFileTime: LPFILETIME);
}

/// C: static int
/// C: process_times (HANDLE h_proc, Lisp_Object *ctime, Lisp_Object *etime,
/// C: 	       Lisp_Object *stime, Lisp_Object *utime, Lisp_Object *ttime,
/// C: 	       double *pcpu)
#[cfg (feature = "windowsnt")]
unsafe fn process_times (h_proc: HANDLE, ctime: *mut Lisp_Object, etime: *mut Lisp_Object, // C 7384
			  stime: *mut Lisp_Object, utime: *mut Lisp_Object, ttime: *mut Lisp_Object, // C 7386
			  pcpu: *mut c_double) -> c_int                                      // C 7387
{                                                                       // C 7388
  let mut ft_creation: FILETIME;                                        // C 7389
  let mut ft_exit: FILETIME;                                            // C 7389
  let mut ft_kernel: FILETIME;                                          // C 7389
  let mut ft_user: FILETIME;                                            // C 7389
  let mut ft_current: FILETIME;                                         // C 7389
  let mut tem1: ULONGLONG;                                              // C 7390
  let mut tem2: ULONGLONG;                                              // C 7390
  let mut tem3: ULONGLONG;                                              // C 7390
  let mut tem: ULONGLONG;                                               // C 7390

  /* C leaves all nine uninitialised (PORTING.md 1.5).  */
  ft_creation = std::mem::zeroed ();
  ft_exit = std::mem::zeroed ();
  ft_kernel = std::mem::zeroed ();
  ft_user = std::mem::zeroed ();
  ft_current = std::mem::zeroed ();
  tem1 = 0;
  tem2 = 0;
  tem3 = 0;
  tem = 0;

  if h_proc.is_null ()                                                  // C 7392 (C's `!h_proc`)
      || get_process_times_fn.is_none ()                                // C 7393 (C's `!get_process_times_fn`)
      || (get_process_times_fn.unwrap ()) (h_proc, (&raw mut ft_creation), (&raw mut ft_exit), // C 7394
				   (&raw mut ft_kernel), (&raw mut ft_user)) == 0                   // C 7395 (C's `!` of the call)
    { return 0; }                                                       // C 7396

  GetSystemTimeAsFileTime ((&raw mut ft_current));                      // C 7398

  FILETIME_TO_U64! (tem1, ft_kernel);                                   // C 7400
  *stime = U64_TO_LISP_TIME! (tem1);                                    // C 7401

  FILETIME_TO_U64! (tem2, ft_user);                                     // C 7403
  *utime = U64_TO_LISP_TIME! (tem2);                                    // C 7404

  tem3 = tem1 + tem2;                                                   // C 7406
  *ttime = U64_TO_LISP_TIME! (tem3);                                    // C 7407

  FILETIME_TO_U64! (tem, ft_creation);                                  // C 7409
  /* Process no 4 (System) returns zero creation time.  */              // C 7410
  if tem != 0                                                           // C 7411 (C's `if (tem)`)
    { tem -= utc_base; }                                                // C 7412
  *ctime = U64_TO_LISP_TIME! (tem);                                     // C 7413

  if tem != 0                                                           // C 7415 (C's `if (tem)`)
    {                                                                   // C 7416
      FILETIME_TO_U64! (tem3, ft_current);                              // C 7417
      tem = (tem3 - utc_base) - tem;                                    // C 7418
    }                                                                   // C 7419
  *etime = U64_TO_LISP_TIME! (tem);                                     // C 7420

  if tem != 0                                                           // C 7422 (C's `if (tem)`)
    {                                                                   // C 7423
      *pcpu = 100.0 * ((tem1 + tem2) as c_double) / (tem as c_double);  // C 7424 (C's `double` arithmetic)
      if *pcpu > 100.0                                                  // C 7425 (C's `> 100`)
	{ *pcpu = 100.0; }                                                     // C 7426
    }                                                                   // C 7427
  else                                                                  // C 7428
    { *pcpu = 0.0; }                                                    // C 7429

  return 1;                                                             // C 7431
}                                                                       // C 7432

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object // C 7434
{                                                                       // C 7436
  let mut attrs: Lisp_Object = Qnil ();                                 // C 7437 (C's `= Qnil`)
  let mut cmd_str: Lisp_Object;                                         // C 7438
  let mut decoded_cmd: Lisp_Object;                                     // C 7438
  let mut tem: Lisp_Object;                                             // C 7438
  let mut h_snapshot: HANDLE;                                           // C 7439
  let mut h_proc: HANDLE;                                               // C 7439
  let mut proc_id: DWORD;                                               // C 7440
  let mut found_proc: c_int = 0;                                        // C 7441
  let mut uname: [c_char; UNLEN + 1] = [0; UNLEN + 1];                  // C 7442 (C's `UNLEN+1`/`GNLEN+1`/1025)
  let mut gname: [c_char; GNLEN + 1] = [0; GNLEN + 1];                  // C 7442
  let mut domain: [c_char; 1025] = [0; 1025];                           // C 7442
  let mut ulength: DWORD = core::mem::size_of_val (&uname) as DWORD;    // C 7443 (C's three `sizeof`s)
  let mut dlength: DWORD = core::mem::size_of_val (&domain) as DWORD;   // C 7443
  let mut needed: DWORD;                                                // C 7443
  let mut glength: DWORD = core::mem::size_of_val (&gname) as DWORD;    // C 7444
  let mut token: HANDLE = std::ptr::null_mut ();                        // C 7445 (C's `= NULL`)
  let mut user_type: SID_NAME_USE;                                      // C 7446
  let mut buf: *mut c_uchar = std::ptr::null_mut ();                    // C 7447 (C's `= NULL`)
  let mut blen: DWORD = 0;                                              // C 7448
  let mut user_token: TOKEN_USER;                                       // C 7449
  let mut group_token: TOKEN_PRIMARY_GROUP;                             // C 7450
  let mut euid: c_uint;                                                 // C 7451 (C's `unsigned`)
  let mut egid: c_uint;                                                 // C 7452
  let mut mem: PROCESS_MEMORY_COUNTERS;                                 // C 7453
  let mut mem_ex: PROCESS_MEMORY_COUNTERS_EX;                           // C 7454
  let mut minrss: SIZE_T;                                               // C 7455
  let mut maxrss: SIZE_T;                                               // C 7455
  let mut memst: MEMORYSTATUS;                                          // C 7456
  let mut memstex: MEMORY_STATUS_EX;                                    // C 7457
  let mut totphys: c_double = 0.0;                                      // C 7458
  let mut ctime: Lisp_Object;                                           // C 7459
  let mut stime: Lisp_Object;                                           // C 7459
  let mut utime: Lisp_Object;                                           // C 7459
  let mut etime: Lisp_Object;                                           // C 7459
  let mut ttime: Lisp_Object;                                           // C 7459
  let mut pcpu: c_double;                                               // C 7460
  let mut result: BOOL = FALSE;                                         // C 7461 (C's `= FALSE`)

  /* C leaves these uninitialised (PORTING.md 1.5): `cmd_str`, `decoded_cmd`, `tem`,
     `h_snapshot`, `h_proc`, `proc_id`, `needed`, `user_type`, `user_token`, `group_token`,
     `euid`, `egid`, the five records, `minrss`, `maxrss`, `ctime`, `stime`, `utime`, `etime`,
     `ttime` and `pcpu`.  */
  cmd_str = std::ptr::null_mut ();
  decoded_cmd = std::ptr::null_mut ();
  tem = std::ptr::null_mut ();
  h_snapshot = std::ptr::null_mut ();
  h_proc = std::ptr::null_mut ();
  proc_id = 0;
  needed = 0;
  user_type = 0;
  user_token = std::mem::zeroed ();
  group_token = std::mem::zeroed ();
  euid = 0;
  egid = 0;
  mem = std::mem::zeroed ();
  mem_ex = std::mem::zeroed ();
  minrss = 0;
  maxrss = 0;
  memst = std::mem::zeroed ();
  memstex = std::mem::zeroed ();
  ctime = std::ptr::null_mut ();
  stime = std::ptr::null_mut ();
  utime = std::ptr::null_mut ();
  etime = std::ptr::null_mut ();
  ttime = std::ptr::null_mut ();
  pcpu = 0.0;

  CHECK_NUMBER (pid);                                                   // C 7463
  proc_id = if FLOATP (pid) { XFLOAT_DATA (pid) as DWORD } else { XFIXNUM (pid) as DWORD }; // C 7464 (C's ternary; `FLOATP` is a `bool`)

  h_snapshot = create_toolhelp32_snapshot (TH32CS_SNAPPROCESS, 0);      // C 7466

  if h_snapshot != INVALID_HANDLE_VALUE                                 // C 7468
    {                                                                   // C 7469
      let mut pe: PROCESSENTRY32;                                       // C 7470
      let mut res: BOOL;                                                // C 7471

      /* C leaves both uninitialised (PORTING.md 1.5).  */
      pe = std::mem::zeroed ();
      res = 0;

      pe.dwSize = core::mem::size_of::<PROCESSENTRY32> () as DWORD;     // C 7473 (C's `sizeof (PROCESSENTRY32)`)
      /* C 7474-7475's two-assignment `for` (the `list_system_processes` device), with C's
	 `break` at C 7501.  */
      res = process32_first (h_snapshot, (&raw mut pe));                // C 7474
      while res != 0                                                    // C 7474 (C's `res` condition)
	{
	  if proc_id == pe.th32ProcessID                                       // C 7477
	    {
	      if proc_id == 0                                                  // C 7479
		{ decoded_cmd = build_string (c"Idle".as_ptr ()); }                   // C 7480 (C's `build_string ("Idle")`)
	      else                                                             // C 7481
		{                                                                     // C 7482
		  /* Decode the command name from locale-specific                     // C 7483
		     encoding.  */                                                    // C 7484
		  cmd_str = build_unibyte_string (pe.szExeFile.as_ptr ());            // C 7485

		  decoded_cmd =                                                       // C 7487
		    code_convert_string_norecord (cmd_str,                            // C 7488
					  Vlocale_coding_system, false);                                   // C 7489 (C's `0`)
		}
	      attrs = Fcons (Fcons (Qcomm (), decoded_cmd), attrs);            // C 7491 (the Q-symbol calls)
	      attrs = Fcons (Fcons (Qppid (),                                  // C 7492
				    INT_TO_INTEGER (pe.th32ParentProcessID as EMACS_INT)),          // C 7493
			     attrs);                                                         // C 7494
	      attrs = Fcons (Fcons (Qpri (), make_fixnum (pe.pcPriClassBase as EMACS_INT)), // C 7495
			     attrs);                                                         // C 7496
	      attrs = Fcons (Fcons (Qthcount (),                               // C 7497
				    INT_TO_INTEGER (pe.cntThreads as EMACS_INT)),                   // C 7498
			     attrs);                                                         // C 7499
	      found_proc = 1;                                                  // C 7500
	      break;                                                           // C 7501
	    }
	  res = process32_next  (h_snapshot, (&raw mut pe));                   // C 7475
	}

      CloseHandle (h_snapshot);                                         // C 7505
    }                                                                   // C 7506

  if found_proc == 0                                                    // C 7508 (C's `!found_proc`)
    { return Qnil (); }                                                 // C 7509 (C's `return Qnil`)

  h_proc = OpenProcess (PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,    // C 7511
			FALSE, proc_id);                                                     // C 7512
  /* If we were denied a handle to the process, try again after         // C 7513
     enabling the SeDebugPrivilege in our process.  */                  // C 7514
  if h_proc.is_null ()                                                  // C 7515 (C's `!h_proc`)
    {                                                                   // C 7516
      let mut priv_current: TOKEN_PRIVILEGES;                           // C 7517

      /* C leaves it uninitialised (PORTING.md 1.5).  */
      priv_current = std::mem::zeroed ();

      if enable_privilege (SE_DEBUG_NAME.as_ptr (), TRUE, (&raw mut priv_current)) != 0 // C 7519 (C's truth test)
	{
	  h_proc = OpenProcess (PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,   // C 7521
				FALSE, proc_id);                                                    // C 7522
	  restore_privilege ((&raw mut priv_current));                         // C 7523
	  revert_to_self ();                                                   // C 7524
	}
    }                                                                   // C 7526
  if ! h_proc.is_null ()                                                // C 7527 (C's truth test)
    {                                                                   // C 7528
      result = open_process_token (h_proc, TOKEN_QUERY, (&raw mut token)); // C 7529
      if result != 0                                                    // C 7530 (C's truth test)
	{
	  result = get_token_information (token, TokenUser, std::ptr::null_mut (), 0, (&raw mut blen)); // C 7532 (C's `NULL`)
	  if result == 0 && GetLastError () == ERROR_INSUFFICIENT_BUFFER       // C 7533 (C's `!result`)
	    {
	      buf = xmalloc (blen as usize) as *mut c_uchar;                   // C 7535 (C's `void *` into `unsigned char *`)
	      result = get_token_information (token, TokenUser,                // C 7536
					      buf as LPVOID, blen, (&raw mut needed));                     // C 7537
	      if result != 0                                                   // C 7538 (C's truth test)
		{
		  memcpy ((&raw mut user_token) as *mut c_void, buf as *const c_void, core::mem::size_of_val (&user_token)); // C 7540 (C's `sizeof (user_token)`)
		  if w32_cached_id (user_token.User.Sid, (&raw mut euid), uname.as_mut_ptr ()) == 0 // C 7541 (C's `!`)
		    {
		      euid = get_rid (user_token.User.Sid);                           // C 7543
		      result = lookup_account_sid (std::ptr::null_mut (), user_token.User.Sid, // C 7544
						   uname.as_mut_ptr (), (&raw mut ulength),                       // C 7545
						   domain.as_mut_ptr (), (&raw mut dlength),                      // C 7546
						   (&raw mut user_type));                                         // C 7547
		      if result != 0                                                  // C 7548 (C's truth test)
			{ w32_add_to_cache (user_token.User.Sid, euid, uname.as_mut_ptr ()); } // C 7549
		      else                                                            // C 7550
			{                                                                    // C 7551
			  strcpy (uname.as_mut_ptr (), c"unknown".as_ptr ());                // C 7552
			  result = TRUE;                                                     // C 7553
			}
		    }
		  ulength = strlen (uname.as_ptr ()) as DWORD;                        // C 7556
		}
	    }
	}
      if result != 0                                                    // C 7560 (C's truth test)
	{
	  /* Determine a reasonable euid and gid values.  */                   // C 7562
	  if xstrcasecmp (c"administrator".as_ptr (), uname.as_mut_ptr ()) == 0 // C 7563 (C's quoted text)
	    {
	      euid = 500;	/* well-known Administrator uid */                   // C 7565
	      egid = 513;	/* well-known None gid */                            // C 7566
	    }
	  else                                                                 // C 7568
	    {
	      /* Get group id and name.  */                                    // C 7570
	      result = get_token_information (token, TokenPrimaryGroup,        // C 7571
					      buf as LPVOID, blen, (&raw mut needed));                     // C 7572
	      if result == 0 && GetLastError () == ERROR_INSUFFICIENT_BUFFER   // C 7573 (C's `!result`)
		{
		  buf = xrealloc (buf as *mut c_void,                                 // C 7575
			  { blen = needed; blen } as usize) as *mut c_uchar;                 // C 7575 (C's `blen = needed` in the argument)
		  result = get_token_information (token, TokenPrimaryGroup,           // C 7576
					  buf as LPVOID, blen, (&raw mut needed));                         // C 7577
		}
	      if result != 0                                                   // C 7579 (C's truth test)
		{
		  memcpy ((&raw mut group_token) as *mut c_void, buf as *const c_void, core::mem::size_of_val (&group_token)); // C 7581 (C's `sizeof (group_token)`)
		  if w32_cached_id (group_token.PrimaryGroup, (&raw mut egid), gname.as_mut_ptr ()) == 0 // C 7582 (C's `!`)
		    {
		      egid = get_rid (group_token.PrimaryGroup);                      // C 7584
		      dlength = core::mem::size_of_val (&domain) as DWORD;            // C 7585 (C's `sizeof (domain)`)
		      result =                                                        // C 7586
			lookup_account_sid (std::ptr::null_mut (), group_token.PrimaryGroup, // C 7587
					    gname.as_mut_ptr (), (&raw mut glength), std::ptr::null_mut (), // C 7588
					    (&raw mut dlength), (&raw mut user_type));                     // C 7589
		      if result != 0                                                  // C 7590 (C's truth test)
			{ w32_add_to_cache (group_token.PrimaryGroup,                        // C 7591 (C's single statement)
					    egid, gname.as_mut_ptr ()); }                                  // C 7592
		      else                                                            // C 7593
			{
			  strcpy (gname.as_mut_ptr (), c"None".as_ptr ());                   // C 7595
			  result = TRUE;                                                     // C 7596
			}
		    }
		  glength = strlen (gname.as_ptr ()) as DWORD;                        // C 7599 (the `size_t`-into-`DWORD` store)
		}
	    }
	}
      xfree (buf as *mut c_void);                                       // C 7603 (C's `unsigned char *` into `void *`)
    }                                                                   // C 7604
  if result == 0                                                        // C 7605 (C's `!result`)
    {                                                                   // C 7606
      if is_windows_9x () == 0                                          // C 7607 (C's `!is_windows_9x ()`)
	{
	  /* We couldn't open the process token, presumably because of         // C 7609
	     insufficient access rights.  Assume this process is run           // C 7610
	     by the system.  */                                                // C 7611
	  strcpy (uname.as_mut_ptr (), c"SYSTEM".as_ptr ());                   // C 7612
	  strcpy (gname.as_mut_ptr (), c"None".as_ptr ());                     // C 7613
	  euid = 18;	/* SYSTEM */                                              // C 7614
	  egid = 513;	/* None */                                               // C 7615
	  glength = strlen (gname.as_ptr ()) as DWORD;                         // C 7616
	  ulength = strlen (uname.as_ptr ()) as DWORD;                         // C 7617
	}
      /* If we are running under Windows 9X, where security calls are   // C 7619
	 not supported, we assume all processes are run by the current         // C 7620
	 user.  */                                                             // C 7621
      else if GetUserName (uname.as_mut_ptr (), (&raw mut ulength)) != 0 // C 7622 (C's truth test)
	{
	  if xstrcasecmp (c"administrator".as_ptr (), uname.as_mut_ptr ()) == 0 // C 7624
	    { euid = 0; }                                                      // C 7625
	  else                                                                 // C 7626
	    { euid = 123; }                                                    // C 7627
	  egid = euid;                                                         // C 7628
	  strcpy (gname.as_mut_ptr (), c"None".as_ptr ());                     // C 7629
	  glength = strlen (gname.as_ptr ()) as DWORD;                         // C 7630
	  ulength = strlen (uname.as_ptr ()) as DWORD;                         // C 7631
	}
      else                                                              // C 7633
	{
	  euid = 123;                                                          // C 7635
	  egid = 123;                                                          // C 7636
	  strcpy (uname.as_mut_ptr (), c"administrator".as_ptr ());            // C 7637
	  ulength = strlen (uname.as_ptr ()) as DWORD;                         // C 7638
	  strcpy (gname.as_mut_ptr (), c"None".as_ptr ());                     // C 7639
	  glength = strlen (gname.as_ptr ()) as DWORD;                         // C 7640
	}
      if ! token.is_null ()                                             // C 7642 (C's truth test)
	{ CloseHandle (token); }                                               // C 7643
    }                                                                   // C 7644

  attrs = Fcons (Fcons (Qeuid (), INT_TO_INTEGER (euid as EMACS_INT)), attrs); // C 7646
  tem = make_unibyte_string (uname.as_mut_ptr (), ulength as ptrdiff_t); // C 7647
  attrs = Fcons (Fcons (Quser (),                                       // C 7648
			 code_convert_string_norecord (tem, Vlocale_coding_system, false)),  // C 7649
		 attrs);                                                              // C 7650
  attrs = Fcons (Fcons (Qegid (), INT_TO_INTEGER (egid as EMACS_INT)), attrs); // C 7651
  tem = make_unibyte_string (gname.as_mut_ptr (), glength as ptrdiff_t); // C 7652
  attrs = Fcons (Fcons (Qgroup (),                                      // C 7653
			 code_convert_string_norecord (tem, Vlocale_coding_system, false)),  // C 7654
		 attrs);                                                              // C 7655

  memstex.dwLength = core::mem::size_of_val (&memstex) as DWORD;        // C 7657 (C's `sizeof (memstex)`)
  if global_memory_status_ex ((&raw mut memstex)) != 0                  // C 7658 (C's truth test)
    { totphys = memstex.ullTotalPhys as c_double / 1024.0; }            // C 7659 (C's `double` division)
  else if global_memory_status ((&raw mut memst)) != 0                  // C 7660 (C's truth test)
    { totphys = memst.dwTotalPhys as c_double / 1024.0; }               // C 7661

  if ! h_proc.is_null ()                                                // C 7663 (C's truth test)
      && get_process_memory_info (h_proc, (&raw mut mem_ex) as PPROCESS_MEMORY_COUNTERS, // C 7664 (C's `(PROCESS_MEMORY_COUNTERS *)` cast)
				  core::mem::size_of_val (&mem_ex) as DWORD) != 0                   // C 7665 (C's `sizeof (mem_ex)`)
    {                                                                   // C 7666
      let rss: SIZE_T = mem_ex.WorkingSetSize / 1024;                   // C 7667

      attrs = Fcons (Fcons (Qmajflt (),                                 // C 7669
			    INT_TO_INTEGER (mem_ex.PageFaultCount as EMACS_INT)),            // C 7670
		     attrs);                                                          // C 7671
      attrs = Fcons (Fcons (Qvsize (),                                  // C 7672
			    INT_TO_INTEGER (mem_ex.PrivateUsage as usize / 1024) as Lisp_Object), // C 7673 (C's `INT_TO_INTEGER` of a `SIZE_T` division)
		     attrs);                                                          // C 7674
      attrs = Fcons (Fcons (Qrss (), INT_TO_INTEGER (rss as EMACS_INT)), attrs); // C 7675
      if totphys != 0.0                                                 // C 7676 (C's `if (totphys)`)
	{ attrs = Fcons (Fcons (Qpmem (), make_float (100.0 * rss as c_double / totphys)), attrs); } // C 7677
    }                                                                   // C 7678
  else if ! h_proc.is_null ()                                           // C 7679 (C's truth test)
	   && get_process_memory_info (h_proc, (&raw mut mem) as PPROCESS_MEMORY_COUNTERS, // C 7680
				      core::mem::size_of_val (&mem) as DWORD) != 0                  // C 7680 (C's `sizeof (mem)`)
    {                                                                   // C 7681
      let rss: SIZE_T = mem_ex.WorkingSetSize / 1024;                   // C 7682 (C reads `mem_ex` here - C's own text)

      attrs = Fcons (Fcons (Qmajflt (),                                 // C 7684
			    INT_TO_INTEGER (mem.PageFaultCount as EMACS_INT)),               // C 7685
		     attrs);                                                          // C 7686
      attrs = Fcons (Fcons (Qrss (), INT_TO_INTEGER (rss as EMACS_INT)), attrs); // C 7687
      if totphys != 0.0                                                 // C 7688 (C's `if (totphys)`)
	{ attrs = Fcons (Fcons (Qpmem (), make_float (100.0 * rss as c_double / totphys)), attrs); } // C 7689
    }                                                                   // C 7690
  else if ! h_proc.is_null ()                                           // C 7691 (C's truth test)
	   && get_process_working_set_size (h_proc, (&raw mut minrss), (&raw mut maxrss)) != 0 // C 7692 (C's truth test)
    {                                                                   // C 7693
      let rss: DWORD = (maxrss / 1024) as DWORD;                        // C 7694 (C's `DWORD rss`)

      attrs = Fcons (Fcons (Qrss (), INT_TO_INTEGER ((maxrss / 1024) as EMACS_INT)), attrs); // C 7696
      if totphys != 0.0                                                 // C 7697 (C's `if (totphys)`)
	{ attrs = Fcons (Fcons (Qpmem (), make_float (100.0 * rss as c_double / totphys)), attrs); } // C 7698
    }                                                                   // C 7699

  if process_times (h_proc, (&raw mut ctime), (&raw mut etime), (&raw mut stime), (&raw mut utime), (&raw mut ttime), (&raw mut pcpu)) != 0 // C 7701 (C's truth test)
    {                                                                   // C 7702
      attrs = Fcons (Fcons (Qutime (), utime), attrs);                  // C 7703
      attrs = Fcons (Fcons (Qstime (), stime), attrs);                  // C 7704
      attrs = Fcons (Fcons (Qtime (),  ttime), attrs);                  // C 7705
      attrs = Fcons (Fcons (Qstart (), ctime), attrs);                  // C 7706
      attrs = Fcons (Fcons (Qetime (), etime), attrs);                  // C 7707
      attrs = Fcons (Fcons (Qpcpu (), make_float (pcpu)), attrs);       // C 7708
    }                                                                   // C 7709

  /* FIXME: Retrieve command line by walking the PEB of the process.  */ // C 7711

  if ! h_proc.is_null ()                                                // C 7713 (C's truth test)
    { CloseHandle (h_proc); }                                           // C 7714
  return attrs;                                                         // C 7715
}                                                                       // C 7716

/* C 7717-7722: C 7717's blank, C 7718's `int` + C 7719-7720's `w32_memory_info` signature
   and C 7722's first local.  This slice carries `w32_memory_info` (7718-7747), C 7749-7756's
   winsock banner, C 7759-7803's thirty-three `pfn_*` function-pointer statics (whose `PASCAL`
   calling convention is `extern "system"`, the unit's `*_Proc` device) and C 7804-7806's
   `#ifndef HANDLE_FLAG_INHERIT` guard.  The socket types those declarations name are declared
   here (P-88, `UNVERIFIED`) - `femacs/nt/inc/sys/include/socket.rs` (C 226) says so itself: it
   spells `u_short`/`u_long`/`struct sockaddr`/`hostent`/`servent`/`addrinfo` the way C does and
   assigns them to the including unit.  `sockaddr`/`hostent`/`servent`/`addrinfo`/`u_short`/
   `u_long` are also six of the names the `windowsnt` probe has carried as unresolved since
   slice 1 (the ledger's Owed 9), and `c_longlong` (three more rows, from the `nt/inc` mirrors'
   `__int64` fields) is added to the `include/lisp.rs` import list, which every unit shares.  */
/* `WORD` needs no declaration here: this unit's own SDK type block above already spells it
   (`pub type WORD = c_ushort;`, C 227's `typedef unsigned short WORD;`).  */
/// C: typedef UINT_PTR SOCKET;
#[cfg (feature = "windowsnt")]
pub type SOCKET = usize;
/// C: typedef unsigned short u_short; typedef unsigned long u_long;  (`<winsock.h>`; `u_long`
/// is the Windows 32-bit `unsigned long`, PORTING.md 1.1's width rule)
#[cfg (feature = "windowsnt")]
pub type u_short = c_ushort;
#[cfg (feature = "windowsnt")]
pub type u_long = u32;
/// C: struct sockaddr { u_short sa_family; char sa_data[14]; };  (`<winsock.h>`)
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr
{
  pub sa_family: u_short,
  pub sa_data: [c_char; 14],
}
/// C: struct hostent { char *h_name; char **h_aliases; short h_addrtype; short h_length;
/// C:   char **h_addr_list; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct hostent
{
  pub h_name: *mut c_char,
  pub h_aliases: *mut *mut c_char,
  pub h_addrtype: c_short,
  pub h_length: c_short,
  pub h_addr_list: *mut *mut c_char,
}
/// C: struct servent { char *s_name; char **s_aliases; short s_port; char *s_proto; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct servent
{
  pub s_name: *mut c_char,
  pub s_aliases: *mut *mut c_char,
  pub s_port: c_short,
  pub s_proto: *mut c_char,
}
/// C: struct addrinfo { int ai_flags; int ai_family; int ai_socktype; int ai_protocol;
/// C:   size_t ai_addrlen; char *ai_canonname; struct sockaddr *ai_addr;
/// C:   struct addrinfo *ai_next; };
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct addrinfo
{
  pub ai_flags: c_int,
  pub ai_family: c_int,
  pub ai_socktype: c_int,
  pub ai_protocol: c_int,
  pub ai_addrlen: size_t,
  pub ai_canonname: *mut c_char,
  pub ai_addr: *mut sockaddr,
  pub ai_next: *mut addrinfo,
}
/// C: typedef struct WSAData { WORD wVersion; WORD wHighVersion; char szDescription[257];
/// C:   char szSystemStatus[129]; unsigned short iMaxSockets; unsigned short iMaxUdpDg;
/// C:   char *lpVendorInfo; } WSADATA, *LPWSADATA;  (`<winsock2.h>`; C 7837 holds one by value)
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSADATA
{
  pub wVersion: WORD,
  pub wHighVersion: WORD,
  pub szDescription: [c_char; 257],
  pub szSystemStatus: [c_char; 129],
  pub iMaxSockets: c_ushort,
  pub iMaxUdpDg: c_ushort,
  pub lpVendorInfo: *mut c_char,
}
#[cfg (feature = "windowsnt")]
pub type LPWSADATA = *mut WSADATA;
/// C: #define FD_MAX_EVENTS 10  and `typedef struct _WSANETWORKEVENTS { long lNetworkEvents;
/// C:   int iErrorCode[FD_MAX_EVENTS]; } WSANETWORKEVENTS, *LPWSANETWORKEVENTS;`
#[cfg (feature = "windowsnt")]
pub const FD_MAX_EVENTS: c_int = 10;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSANETWORKEVENTS
{
  pub lNetworkEvents: c_long,
  pub iErrorCode: [c_int; FD_MAX_EVENTS as usize],
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_memory_info (totalram: *mut c_ulonglong, freeram: *mut c_ulonglong, // C 7718
				     totalswap: *mut c_ulonglong, freeswap: *mut c_ulonglong) -> c_int // C 7720
{                                                                       // C 7721
  let mut memst: MEMORYSTATUS;                                          // C 7722
  let mut memstex: MEMORY_STATUS_EX;                                    // C 7723
  memstex.dwLength = core::mem::size_of_val (&memstex) as DWORD;        // C 7724 (C's `sizeof (memstex)`)
  /* Use GlobalMemoryStatusEx if available, as it can report more than  // C 7725
     2GB of memory.  */                                                 // C 7726
  if global_memory_status_ex ((&raw mut memstex)) != 0                  // C 7727 (C's truth test)
    {                                                                   // C 7728
      *totalram  = memstex.ullTotalPhys;                                // C 7729
      *freeram   = memstex.ullAvailPhys;                                // C 7730
      *totalswap = memstex.ullTotalPageFile;                            // C 7731
      *freeswap  = memstex.ullAvailPageFile;                            // C 7732
      return 0;                                                         // C 7733
    }                                                                   // C 7734

  memst.dwLength = core::mem::size_of_val (&memst) as DWORD;            // C 7736 (C's `sizeof (memst)`)
  if global_memory_status ((&raw mut memst)) != 0                       // C 7737 (C's truth test)
    {                                                                   // C 7738
      *totalram = memst.dwTotalPhys;                                    // C 7739
      *freeram   = memst.dwAvailPhys;                                   // C 7740
      *totalswap = memst.dwTotalPageFile;                               // C 7741
      *freeswap  = memst.dwAvailPageFile;                               // C 7742
      return 0;                                                         // C 7743
    }                                                                   // C 7744
  else                                                                  // C 7745
    { return -1; }                                                      // C 7746
}                                                                       // C 7747


/* Wrappers for  winsock functions to map between our file descriptors  // C 7750
   and winsock's handles; also set h_errno for convenience.             // C 7751

   To allow Emacs to run on systems which don't have winsock support    // C 7753
   installed, we dynamically link to winsock on startup if present, and // C 7754
   otherwise provide the minimum necessary functionality                // C 7755
   (eg. gethostname). */                                                // C 7756

/* function pointers for relevant socket functions */                   // C 7758
/// C: static int (PASCAL *pfn_WSAStartup) (WORD wVersionRequired, LPWSADATA lpWSAData);
#[cfg (feature = "windowsnt")]
static mut pfn_WSAStartup: Option<unsafe extern "system" fn (WORD, LPWSADATA) -> c_int> = None;   // C 7759
/// C: static void (PASCAL *pfn_WSASetLastError) (int iError);
#[cfg (feature = "windowsnt")]
static mut pfn_WSASetLastError: Option<unsafe extern "system" fn (c_int)> = None;   // C 7760
/// C: static int (PASCAL *pfn_WSAGetLastError) (void);
#[cfg (feature = "windowsnt")]
static mut pfn_WSAGetLastError: Option<unsafe extern "system" fn () -> c_int> = None;   // C 7761
/// C: static int (PASCAL *pfn_WSAEventSelect) (SOCKET s, HANDLE hEventObject, long lNetworkEvents);
#[cfg (feature = "windowsnt")]
static mut pfn_WSAEventSelect: Option<unsafe extern "system" fn (SOCKET, HANDLE, c_long) -> c_int> = None;   // C 7762
/// C: static int (PASCAL *pfn_WSAEnumNetworkEvents) (SOCKET s, HANDLE hEventObject,
/// C: 					WSANETWORKEVENTS *NetworkEvents);
#[cfg (feature = "windowsnt")]
static mut pfn_WSAEnumNetworkEvents: Option<unsafe extern "system" fn (SOCKET, HANDLE,   // C 7763
						      *mut WSANETWORKEVENTS) -> c_int> = None;
/// C: static HANDLE (PASCAL *pfn_WSACreateEvent) (void);
#[cfg (feature = "windowsnt")]
static mut pfn_WSACreateEvent: Option<unsafe extern "system" fn () -> HANDLE> = None;   // C 7766
/// C: static int (PASCAL *pfn_WSACloseEvent) (HANDLE hEvent);
#[cfg (feature = "windowsnt")]
static mut pfn_WSACloseEvent: Option<unsafe extern "system" fn (HANDLE) -> c_int> = None;   // C 7767
/// C: static int (PASCAL *pfn_socket) (int af, int type, int protocol);
#[cfg (feature = "windowsnt")]
static mut pfn_socket: Option<unsafe extern "system" fn (c_int, c_int, c_int) -> c_int> = None;   // C 7768
/// C: static int (PASCAL *pfn_bind) (SOCKET s, const struct sockaddr *addr, int namelen);
#[cfg (feature = "windowsnt")]
static mut pfn_bind: Option<unsafe extern "system" fn (SOCKET, *const sockaddr, c_int) -> c_int> = None;   // C 7769
/// C: static int (PASCAL *pfn_connect) (SOCKET s, const struct sockaddr *addr, int namelen);
#[cfg (feature = "windowsnt")]
static mut pfn_connect: Option<unsafe extern "system" fn (SOCKET, *const sockaddr, c_int) -> c_int> = None;   // C 7770
/// C: static int (PASCAL *pfn_ioctlsocket) (SOCKET s, long cmd, u_long *argp);
#[cfg (feature = "windowsnt")]
static mut pfn_ioctlsocket: Option<unsafe extern "system" fn (SOCKET, c_long, *mut u_long) -> c_int> = None;   // C 7771
/// C: static int (PASCAL *pfn_recv) (SOCKET s, char * buf, int len, int flags);
#[cfg (feature = "windowsnt")]
static mut pfn_recv: Option<unsafe extern "system" fn (SOCKET, *mut c_char, c_int, c_int) -> c_int> = None;   // C 7772
/// C: static int (PASCAL *pfn_send) (SOCKET s, const char * buf, int len, int flags);
#[cfg (feature = "windowsnt")]
static mut pfn_send: Option<unsafe extern "system" fn (SOCKET, *const c_char, c_int, c_int) -> c_int> = None;   // C 7773
/// C: static int (PASCAL *pfn_closesocket) (SOCKET s);
#[cfg (feature = "windowsnt")]
static mut pfn_closesocket: Option<unsafe extern "system" fn (SOCKET) -> c_int> = None;   // C 7774
/// C: static int (PASCAL *pfn_shutdown) (SOCKET s, int how);
#[cfg (feature = "windowsnt")]
static mut pfn_shutdown: Option<unsafe extern "system" fn (SOCKET, c_int) -> c_int> = None;   // C 7775
/// C: static int (PASCAL *pfn_WSACleanup) (void);
#[cfg (feature = "windowsnt")]
static mut pfn_WSACleanup: Option<unsafe extern "system" fn () -> c_int> = None;   // C 7776
/// C: static u_short (PASCAL *pfn_htons) (u_short hostshort);
#[cfg (feature = "windowsnt")]
static mut pfn_htons: Option<unsafe extern "system" fn (u_short) -> u_short> = None;   // C 7778
/// C: static u_short (PASCAL *pfn_ntohs) (u_short netshort);
#[cfg (feature = "windowsnt")]
static mut pfn_ntohs: Option<unsafe extern "system" fn (u_short) -> u_short> = None;   // C 7779
/// C: static u_long (PASCAL *pfn_htonl) (u_long hostlong);
#[cfg (feature = "windowsnt")]
static mut pfn_htonl: Option<unsafe extern "system" fn (u_long) -> u_long> = None;   // C 7780
/// C: static u_long (PASCAL *pfn_ntohl) (u_long netlong);
#[cfg (feature = "windowsnt")]
static mut pfn_ntohl: Option<unsafe extern "system" fn (u_long) -> u_long> = None;   // C 7781
/// C: static unsigned long (PASCAL *pfn_inet_addr) (const char * cp);
#[cfg (feature = "windowsnt")]
static mut pfn_inet_addr: Option<unsafe extern "system" fn (*const c_char) -> c_ulong> = None;   // C 7782
/// C: static int (PASCAL *pfn_gethostname) (char * name, int namelen);
#[cfg (feature = "windowsnt")]
static mut pfn_gethostname: Option<unsafe extern "system" fn (*mut c_char, c_int) -> c_int> = None;   // C 7783
/// C: static struct hostent * (PASCAL *pfn_gethostbyname) (const char * name);
#[cfg (feature = "windowsnt")]
static mut pfn_gethostbyname: Option<unsafe extern "system" fn (*const c_char) -> *mut hostent> = None;   // C 7784
/// C: static struct servent * (PASCAL *pfn_getservbyname) (const char * name, const char * proto);
#[cfg (feature = "windowsnt")]
static mut pfn_getservbyname: Option<unsafe extern "system" fn (*const c_char, *const c_char) -> *mut servent> = None;   // C 7785
/// C: static int (PASCAL *pfn_getpeername) (SOCKET s, struct sockaddr *addr, int * namelen);
#[cfg (feature = "windowsnt")]
static mut pfn_getpeername: Option<unsafe extern "system" fn (SOCKET, *mut sockaddr, *mut c_int) -> c_int> = None;   // C 7786
/// C: static int (PASCAL *pfn_setsockopt) (SOCKET s, int level, int optname,
/// C: 			      const char * optval, int optlen);
#[cfg (feature = "windowsnt")]
static mut pfn_setsockopt: Option<unsafe extern "system" fn (SOCKET, c_int, c_int,   // C 7787
					      *const c_char, c_int) -> c_int> = None;
/// C: static int (PASCAL *pfn_listen) (SOCKET s, int backlog);
#[cfg (feature = "windowsnt")]
static mut pfn_listen: Option<unsafe extern "system" fn (SOCKET, c_int) -> c_int> = None;   // C 7789
/// C: static int (PASCAL *pfn_getsockname) (SOCKET s, struct sockaddr * name,
/// C: 			       int * namelen);
#[cfg (feature = "windowsnt")]
static mut pfn_getsockname: Option<unsafe extern "system" fn (SOCKET, *mut sockaddr,   // C 7790
					      *mut c_int) -> c_int> = None;
/// C: static SOCKET (PASCAL *pfn_accept) (SOCKET s, struct sockaddr * addr, int * addrlen);
#[cfg (feature = "windowsnt")]
static mut pfn_accept: Option<unsafe extern "system" fn (SOCKET, *mut sockaddr, *mut c_int) -> SOCKET> = None;   // C 7792
/// C: static int (PASCAL *pfn_recvfrom) (SOCKET s, char * buf, int len, int flags,
/// C: 		       struct sockaddr * from, int * fromlen);
#[cfg (feature = "windowsnt")]
static mut pfn_recvfrom: Option<unsafe extern "system" fn (SOCKET, *mut c_char, c_int, c_int,   // C 7793
					      *mut sockaddr, *mut c_int) -> c_int> = None;
/// C: static int (PASCAL *pfn_sendto) (SOCKET s, const char * buf, int len, int flags,
/// C: 			  const struct sockaddr * to, int tolen);
#[cfg (feature = "windowsnt")]
static mut pfn_sendto: Option<unsafe extern "system" fn (SOCKET, *const c_char, c_int, c_int,   // C 7795
				      *const sockaddr, c_int) -> c_int> = None;
/// C: static int (PASCAL *pfn_getaddrinfo) (const char *, const char *,
/// C: 			       const struct addrinfo *, struct addrinfo **);
#[cfg (feature = "windowsnt")]
static mut pfn_getaddrinfo: Option<unsafe extern "system" fn (*const c_char, *const c_char,   // C 7798
					      *const addrinfo, *mut *mut addrinfo) -> c_int> = None;
/// C: static void (PASCAL *pfn_freeaddrinfo) (struct addrinfo *);
#[cfg (feature = "windowsnt")]
static mut pfn_freeaddrinfo: Option<unsafe extern "system" fn (*mut addrinfo)> = None;   // C 7800
/// C: /* SetHandleInformation is only needed to make sockets non-inheritable. */
/* /* SetHandleInformation is only needed to make sockets non-inheritable. */  */
/// C: static BOOL (WINAPI *pfn_SetHandleInformation) (HANDLE object, DWORD mask, DWORD flags);
#[cfg (feature = "windowsnt")]
static mut pfn_SetHandleInformation: Option<unsafe extern "system" fn (HANDLE, DWORD, DWORD) -> BOOL> = None;   // C 7803

/* C 7804-7806: the `#ifndef HANDLE_FLAG_INHERIT` guard (the `SDDL_REVISION_1` device): the name
   is also one of `w32.c`'s three remaining `missing macro` rows.  */
/// C: #ifndef HANDLE_FLAG_INHERIT
/// C: #define HANDLE_FLAG_INHERIT	1
/// C: #endif
#[cfg (feature = "windowsnt")]
pub const HANDLE_FLAG_INHERIT: DWORD = 1;

/* C 7807-7814: C 7807's blank, C 7808-7810's `extern HANDLE winsock_lib;` / `HANDLE
   winsock_lib;` pair and C 7810's `static int winsock_inuse;`, C 7812's prototype (dropped
   per PORTING.md 0.1-12, its line carried here) and C 7814-7815's `BOOL` + `term_winsock`.
   This slice carries `term_winsock` (7814-7832), `init_winsock` (7834-7927, with C 7852-7887's
   `LOAD_PROC` macro and C 7921's `fail:` label) and the two file-scope objects C 7809/7810
   define - two of this file's six `missing file-scope object` rows.  The names this slice
   first uses are declared here (P-88, `UNVERIFIED`): `<winsock2.h>`'s `WSAENETDOWN` (the
   `nt/inc/sys/include/socket.rs` 135 `ENETDOWN` row already *uses* it and says that header "has
   no mirror") and `<kernel32.h>`'s `FreeLibrary`.  `release_listen_threads` (C 7819) is
   `src/w32.h` 173's declaration, i.e. the in-scope `include/w32.rs` mirror.  */
/// C: extern void release_listen_threads (void);  [src/w32.h 173]
/// C: #define WSAENETDOWN 10050  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENETDOWN: c_int = 10050;
/// C: WINBASEAPI BOOL WINAPI FreeLibrary (HMODULE hLibModule);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn FreeLibrary (hLibModule: HMODULE) -> BOOL;
}

/// C 7852-7854: `#define LOAD_PROC(fn) if ((pfn_##fn = (void *) get_proc_addr (winsock_lib,
/// #fn)) == NULL) goto fail;` - `do { … } while` is a block expression (PORTING.md 2.1), and C's
/// `##fn`/`#fn` token pasting has no Rust spelling, so the call sites pass the pasted field (the
/// `LOAD_DLL_FN!` device of `decompress.rs` 237, whose doc comment records the same rule).  C's
/// `goto fail` is `break 'fail` - the label's block is `init_winsock`'s own below.
#[cfg (feature = "windowsnt")]
macro_rules! LOAD_PROC {
  ($label:lifetime, $pfn:ident, $name:expr) => {
    {
      $pfn = std::mem::transmute::<VOIDFNPTR, _> (get_proc_addr (winsock_lib, $name.as_ptr ()));
      if $pfn.is_none () { break $label; }
    }
  };
}

/* C 7808-7809's `extern`-then-define pair is one item (PORTING.md 0.1-12), and C spells the
   definition without `static`, so it is exported.  */
/// C: HANDLE winsock_lib;  (C 7808's `extern` + C 7809's definition)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut winsock_lib: HANDLE = std::ptr::null_mut ();
/// C: static int winsock_inuse;
#[cfg (feature = "windowsnt")]
static mut winsock_inuse: c_int = 0;                                    // C 7810 (C's tentative `static int`)

/* C 7812: `BOOL term_winsock (void);` - C's own forward declaration of the function right
   below; the port needs no item for it (PORTING.md 0.1-12), so the line is carried here.  */

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn term_winsock () -> BOOL                        // C 7814
{                                                                       // C 7816
  if ! winsock_lib.is_null () && winsock_inuse == 0                     // C 7817 (C's two tests)
    {                                                                   // C 7818
      release_listen_threads ();                                        // C 7819
      /* Not sure what would cause WSAENETDOWN, or even if it can happen // C 7820
	 after WSAStartup returns successfully, but it seems reasonable        // C 7821
	 to allow unloading winsock anyway in that case. */                    // C 7822
      if pfn_WSACleanup.unwrap () () == 0                               // C 7823 (C's call-through)
	  || pfn_WSAGetLastError.unwrap () () == WSAENETDOWN                   // C 7824 (C's `||` continuation)
	{                                                                      // C 7825
	  if FreeLibrary (winsock_lib) != 0                                    // C 7826 (C's truth test)
	  { winsock_lib = std::ptr::null_mut (); }                             // C 7827 (C's `= NULL`)
	  return TRUE;                                                         // C 7828
	}                                                                      // C 7829
    }                                                                   // C 7830
  return FALSE;                                                         // C 7831
}                                                                       // C 7832

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_winsock (load_now: c_int) -> BOOL         // C 7834
{                                                                       // C 7836
  let mut winsockData: WSADATA;                                         // C 7837

  /* C leaves it uninitialised (PORTING.md 1.5).  */
  winsockData = std::mem::zeroed ();

  if ! winsock_lib.is_null ()                                           // C 7839 (C's `!= NULL`)
    { return TRUE; }                                                    // C 7840

  pfn_SetHandleInformation                                              // C 7842
    = std::mem::transmute::<VOIDFNPTR, _> (get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()), // C 7843
					     c"SetHandleInformation".as_ptr ()));                          // C 7844

  winsock_lib = LoadLibrary (c"Ws2_32.dll".as_ptr ());                  // C 7846 (C's quoted name)

  if ! winsock_lib.is_null ()                                           // C 7848 (C's `!= NULL`)
    {                                                                   // C 7849
      /* dynamically link to socket functions  */                       // C 7850

      /* C 7856-7886's thirty `LOAD_PROC` invocations - each passes the pasted `pfn_##fn` field
	 and C's `#fn` string, and `break 'fail` stands in for the macro's `goto fail`.  The block
	 below is C 7921's `fail:` label's body reached from them.  */
      'fail: {                                                          // C 7852-7924 (the device)
	LOAD_PROC! ('fail, pfn_WSAStartup, c"WSAStartup");                     // C 7856
	LOAD_PROC! ('fail, pfn_WSASetLastError, c"WSASetLastError");           // C 7857
	LOAD_PROC! ('fail, pfn_WSAGetLastError, c"WSAGetLastError");           // C 7858
	LOAD_PROC! ('fail, pfn_WSAEventSelect, c"WSAEventSelect");             // C 7859
	LOAD_PROC! ('fail, pfn_WSAEnumNetworkEvents, c"WSAEnumNetworkEvents"); // C 7860
	LOAD_PROC! ('fail, pfn_WSACreateEvent, c"WSACreateEvent");             // C 7861
	LOAD_PROC! ('fail, pfn_WSACloseEvent, c"WSACloseEvent");               // C 7862
	LOAD_PROC! ('fail, pfn_socket, c"socket");                             // C 7863
	LOAD_PROC! ('fail, pfn_bind, c"bind");                                 // C 7864
	LOAD_PROC! ('fail, pfn_connect, c"connect");                           // C 7865
	LOAD_PROC! ('fail, pfn_ioctlsocket, c"ioctlsocket");                   // C 7866
	LOAD_PROC! ('fail, pfn_recv, c"recv");                                 // C 7867
	LOAD_PROC! ('fail, pfn_send, c"send");                                 // C 7868
	LOAD_PROC! ('fail, pfn_closesocket, c"closesocket");                   // C 7869
	LOAD_PROC! ('fail, pfn_shutdown, c"shutdown");                         // C 7870
	LOAD_PROC! ('fail, pfn_htons, c"htons");                               // C 7871
	LOAD_PROC! ('fail, pfn_ntohs, c"ntohs");                               // C 7872
	LOAD_PROC! ('fail, pfn_htonl, c"htonl");                               // C 7873
	LOAD_PROC! ('fail, pfn_ntohl, c"ntohl");                               // C 7874
	LOAD_PROC! ('fail, pfn_inet_addr, c"inet_addr");                       // C 7875
	LOAD_PROC! ('fail, pfn_gethostname, c"gethostname");                   // C 7876
	LOAD_PROC! ('fail, pfn_gethostbyname, c"gethostbyname");               // C 7877
	LOAD_PROC! ('fail, pfn_getservbyname, c"getservbyname");               // C 7878
	LOAD_PROC! ('fail, pfn_getpeername, c"getpeername");                   // C 7879
	LOAD_PROC! ('fail, pfn_WSACleanup, c"WSACleanup");                     // C 7880
	LOAD_PROC! ('fail, pfn_setsockopt, c"setsockopt");                     // C 7881
	LOAD_PROC! ('fail, pfn_listen, c"listen");                             // C 7882
	LOAD_PROC! ('fail, pfn_getsockname, c"getsockname");                   // C 7883
	LOAD_PROC! ('fail, pfn_accept, c"accept");                             // C 7884
	LOAD_PROC! ('fail, pfn_recvfrom, c"recvfrom");                         // C 7885
	LOAD_PROC! ('fail, pfn_sendto, c"sendto");                             // C 7886

	/* Try loading functions not available before XP.  */                  // C 7889
	pfn_getaddrinfo = std::mem::transmute::<VOIDFNPTR, _> (get_proc_addr (winsock_lib, c"getaddrinfo".as_ptr ())); // C 7890
	pfn_freeaddrinfo = std::mem::transmute::<VOIDFNPTR, _> (get_proc_addr (winsock_lib, c"freeaddrinfo".as_ptr ())); // C 7891
      /* Paranoia: these two functions should go together, so if one    // C 7892
	 is absent, we cannot use the other.  */                               // C 7893
	if pfn_getaddrinfo.is_none ()                                          // C 7894 (C's `== NULL`)
	  { pfn_freeaddrinfo = None; }
	else if pfn_freeaddrinfo.is_none ()                                    // C 7896
	  { pfn_getaddrinfo = None; }

	/* specify version 1.1 of winsock */                                   // C 7899
	if pfn_WSAStartup.unwrap () (0x101, (&raw mut winsockData)) == 0       // C 7900 (C's call-through)
	  {
	    if winsockData.wVersion != 0x101                                   // C 7902
	      { break 'fail; }                                                 // C 7903 (C's `goto fail`)

	    if load_now == 0                                                   // C 7905 (C's `!load_now`)
	      {
	      /* Report that winsock exists and is usable, but leave           // C 7907
		 socket functions disabled.  I am assuming that calling               // C 7908
		 WSAStartup does not require any network interaction,                 // C 7909
		 and in particular does not cause or require a dial-up                // C 7910
		 connection to be established. */                                     // C 7911
		pfn_WSACleanup.unwrap () ();                                          // C 7913
		FreeLibrary (winsock_lib);                                            // C 7914
		winsock_lib = std::ptr::null_mut ();                                  // C 7915 (C's `= NULL`)
	      }
	    winsock_inuse = 0;                                                 // C 7917
	    return TRUE;                                                       // C 7918
	  }

	/* C 7921-7923's `fail:` label body: reached by the thirty `LOAD_PROC` breaks and by the
	   fall-through of C 7900's `if` (when `pfn_WSAStartup` returns non-zero).  */
	FreeLibrary (winsock_lib);                                             // C 7922
	winsock_lib = std::ptr::null_mut ();                                   // C 7923 (C's `= NULL`)
      }
    }                                                                   // C 7924

  return FALSE;                                                         // C 7926
}                                                                       // C 7927


/* C 7928-7929's two blanks and C 7930's `int h_errno = 0;`: the definition of the file-scope
   object the `nt/inc/sys/include/socket.rs` 232 `h_errno` row declares (a definition shadows a
   glob-imported declaration, PORTING.md 0.1-10), and C does not spell it `static`, so it is
   exported.  The `WSAE*` names C 7948-7957's `switch` tests are the `<winsock2.h>` family that
   the socket.rs mirror says has "no mirror" (its 105/149 rows `EWOULDBLOCK`/`ENOTCONN` already
   *use* two of them); `ENOTEMPTY` is `<errno.h>`'s, which the `lib/include/errno.rs` mirror does
   not carry and `nt/inc/sys/socket.h` 263 leaves commented out.  Every value here is
   `UNVERIFIED` (P-88): this host has no Windows SDK to measure them with - Owed 24.  */
/// C: #define WSAEACCES 10013  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEACCES: c_int = 10013;
/// C: #define WSAEBADF 10009  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEBADF: c_int = 10009;
/// C: #define WSAEFAULT 10014  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEFAULT: c_int = 10014;
/// C: #define WSAEINTR 10004  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEINTR: c_int = 10004;
/// C: #define WSAEINVAL 10022  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEINVAL: c_int = 10022;
/// C: #define WSAEMFILE 10024  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEMFILE: c_int = 10024;
/// C: #define WSAENAMETOOLONG 10063  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENAMETOOLONG: c_int = 10063;
/// C: #define WSAENOTEMPTY 10066  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENOTEMPTY: c_int = 10066;
/// C: #define WSAEWOULDBLOCK 10035  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEWOULDBLOCK: c_int = 10035;
/// C: #define WSAENOTCONN 10057  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENOTCONN: c_int = 10057;
/// C: #define ENOTEMPTY 41  (`<errno.h>`, the CRT's own)
#[cfg (feature = "windowsnt")]
pub const ENOTEMPTY: c_int = 41;
/// C: int h_errno = 0;
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub static mut h_errno: c_int = 0;                                      // C 7930

/* Function to map winsock error codes to errno codes for those errno   // C 7932
   code defined in errno.h (errno values not defined by errno.h are     // C 7933
   already in nt/inc/sys/socket.h).  */                                 // C 7934

#[cfg (feature = "windowsnt")]
unsafe fn set_errno ()                                                  // C 7935 (C's `static void` + `set_errno (void)`)
{                                                                       // C 7937
  let mut wsa_err: c_int;                                               // C 7938
                                                                        // C 7939
  h_errno = 0;                                                          // C 7940
  if winsock_lib.is_null ()                                             // C 7941 (C's `== NULL`)
    { wsa_err = EINVAL; }                                               // C 7942
  else                                                                  // C 7943
    { wsa_err = pfn_WSAGetLastError.unwrap () (); }                     // C 7944 (C's call-through)
                                                                        // C 7945
  match wsa_err                                                         // C 7946
    {                                                                   // C 7947
    WSAEACCES => { *errno () = EACCES; }                                // C 7948
    WSAEBADF => { *errno () = EBADF; }                                  // C 7949
    WSAEFAULT => { *errno () = EFAULT; }                                // C 7950
    WSAEINTR => { *errno () = EINTR; }                                  // C 7951
    WSAEINVAL => { *errno () = EINVAL; }                                // C 7952
    WSAEMFILE => { *errno () = EMFILE; }                                // C 7953
    WSAENAMETOOLONG => { *errno () = ENAMETOOLONG; }                    // C 7954
    WSAENOTEMPTY => { *errno () = ENOTEMPTY; }                          // C 7955
    WSAEWOULDBLOCK => { *errno () = EWOULDBLOCK; }                      // C 7956
    WSAENOTCONN => { *errno () = ENOTCONN; }                            // C 7957
    _ => { *errno () = wsa_err; }                                       // C 7958 (C's `default`)
    }                                                                   // C 7959
}                                                                       // C 7960

#[cfg (feature = "windowsnt")]
unsafe fn check_errno ()                                                // C 7962 (C's `static void` + `check_errno (void)`)
{                                                                       // C 7964
  h_errno = 0;                                                          // C 7965
  if ! winsock_lib.is_null ()                                           // C 7966 (C's `!= NULL`)
    { pfn_WSASetLastError.unwrap () (0); }                              // C 7967 (C's call-through)
}                                                                       // C 7968


/* C 7969's blank and C 7970's comment - C's own text.  The table below is C 7971-8044's
   `static struct { int errnum; const char * msg; } _wsa_errlist[]`, whose 63 rows are C's
   (C 7981/8019/8037/8042 are the initializer's blank lines).  Its `#ifdef WSAENOMORE` arm
   (C 8024-8036) is eleven more rows: `<winsock2.h>` defines that family only from
   `_WIN32_WINNT >= 0x0600` on, and C 141-143 leaves `_WIN32_WINNT` alone under `MINGW_W64`
   ("MinGW64 defines this to a higher value in its system headers") while the non-MINGW_W64
   arm pins it to 0x0400 - so both arms are live configurations and the rows carry the
   `wsaenomore` feature (the C 140/141 `win32_winnt_lt_0500`/`mingw_w64` device).  Rust's
   array length must be written, so it is the count under the same feature.  */
/* Extend strerror to handle the winsock-specific error codes.  */

/* The names this slice first uses are declared here (P-88, `UNVERIFIED`): the `<winsock2.h>`
   `WSA*` family the socket mirror's own note assigns to its including unit (its rows 106-172
   already *use* them, which is why they are unresolved names in the arm reading), and the
   CRT's `_sys_nerr`/`sys_errlist` below `sys_strerror`'s comment.  The ten names slice 22
   declared are not repeated.  */
/// C: #define WSAEINPROGRESS 10036  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEINPROGRESS: c_int = 10036;
/// C: #define WSAEALREADY 10037  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEALREADY: c_int = 10037;
/// C: #define WSAENOTSOCK 10038  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENOTSOCK: c_int = 10038;
/// C: #define WSAEDESTADDRREQ 10039  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEDESTADDRREQ: c_int = 10039;
/// C: #define WSAEMSGSIZE 10040  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEMSGSIZE: c_int = 10040;
/// C: #define WSAEPROTOTYPE 10041  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEPROTOTYPE: c_int = 10041;
/// C: #define WSAENOPROTOOPT 10042  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENOPROTOOPT: c_int = 10042;
/// C: #define WSAEPROTONOSUPPORT 10043  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEPROTONOSUPPORT: c_int = 10043;
/// C: #define WSAESOCKTNOSUPPORT 10044  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAESOCKTNOSUPPORT: c_int = 10044;
/// C: #define WSAEOPNOTSUPP 10045  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEOPNOTSUPP: c_int = 10045;
/// C: #define WSAEPFNOSUPPORT 10046  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEPFNOSUPPORT: c_int = 10046;
/// C: #define WSAEAFNOSUPPORT 10047  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEAFNOSUPPORT: c_int = 10047;
/// C: #define WSAEADDRINUSE 10048  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEADDRINUSE: c_int = 10048;
/// C: #define WSAEADDRNOTAVAIL 10049  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEADDRNOTAVAIL: c_int = 10049;
/// C: #define WSAENETUNREACH 10051  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENETUNREACH: c_int = 10051;
/// C: #define WSAENETRESET 10052  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENETRESET: c_int = 10052;
/// C: #define WSAECONNABORTED 10053  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAECONNABORTED: c_int = 10053;
/// C: #define WSAECONNRESET 10054  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAECONNRESET: c_int = 10054;
/// C: #define WSAENOBUFS 10055  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAENOBUFS: c_int = 10055;
/// C: #define WSAEISCONN 10056  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEISCONN: c_int = 10056;
/// C: #define WSAESHUTDOWN 10058  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAESHUTDOWN: c_int = 10058;
/// C: #define WSAETOOMANYREFS 10059  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAETOOMANYREFS: c_int = 10059;
/// C: #define WSAETIMEDOUT 10060  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAETIMEDOUT: c_int = 10060;
/// C: #define WSAECONNREFUSED 10061  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAECONNREFUSED: c_int = 10061;
/// C: #define WSAELOOP 10062  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAELOOP: c_int = 10062;
/// C: #define WSAEHOSTDOWN 10064  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEHOSTDOWN: c_int = 10064;
/// C: #define WSAEHOSTUNREACH 10065  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEHOSTUNREACH: c_int = 10065;
/// C: #define WSAEPROCLIM 10067  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEPROCLIM: c_int = 10067;
/// C: #define WSAEUSERS 10068  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEUSERS: c_int = 10068;
/// C: #define WSAEDQUOT 10069  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEDQUOT: c_int = 10069;
/// C: #define WSAESTALE 10070  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAESTALE: c_int = 10070;
/// C: #define WSAEREMOTE 10071  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEREMOTE: c_int = 10071;
/// C: #define WSASYSNOTREADY 10091  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSASYSNOTREADY: c_int = 10091;
/// C: #define WSAVERNOTSUPPORTED 10092  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAVERNOTSUPPORTED: c_int = 10092;
/// C: #define WSANOTINITIALISED 10093  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSANOTINITIALISED: c_int = 10093;
/// C: #define WSAEDISCON 10101  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAEDISCON: c_int = 10101;
/// C: #define WSAHOST_NOT_FOUND 11001  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSAHOST_NOT_FOUND: c_int = 11001;
/// C: #define WSATRY_AGAIN 11002  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSATRY_AGAIN: c_int = 11002;
/// C: #define WSANO_RECOVERY 11003  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSANO_RECOVERY: c_int = 11003;
/// C: #define WSANO_DATA 11004  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const WSANO_DATA: c_int = 11004;

/// C: #define WSAENOMORE 10202  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAENOMORE: c_int = 10202;
/// C: #define WSAECANCELLED 10103  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAECANCELLED: c_int = 10103;
/// C: #define WSAEINVALIDPROCTABLE 10104  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAEINVALIDPROCTABLE: c_int = 10104;
/// C: #define WSAEINVALIDPROVIDER 10105  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAEINVALIDPROVIDER: c_int = 10105;
/// C: #define WSAEPROVIDERFAILEDINIT 10106  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAEPROVIDERFAILEDINIT: c_int = 10106;
/// C: #define WSASYSCALLFAILURE 10107  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSASYSCALLFAILURE: c_int = 10107;
/// C: #define WSASERVICE_NOT_FOUND 10108  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSASERVICE_NOT_FOUND: c_int = 10108;
/// C: #define WSATYPE_NOT_FOUND 10109  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSATYPE_NOT_FOUND: c_int = 10109;
/// C: #define WSA_E_NO_MORE 10110  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSA_E_NO_MORE: c_int = 10110;
/// C: #define WSA_E_CANCELLED 10111  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSA_E_CANCELLED: c_int = 10111;
/// C: #define WSAEREFUSED 10112  (`<winsock2.h>`, the `WSAENOMORE` arm)
#[cfg (all (feature = "windowsnt", feature = "wsaenomore"))]
pub const WSAEREFUSED: c_int = 10112;

/* C 7971-7974: the element type and the array.  C's record has no tag and no typedef, so the
   port names the type after the variable (P-67, the `w16select.rs` 268-280 device); Rust's
   type and value namespaces keep `struct _wsa_errlist` and `static mut _wsa_errlist` apart.  */
/// C: static struct { int errnum; const char * msg; } _wsa_errlist[];
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct _wsa_errlist
{
  pub errnum: c_int,
  pub msg: *const c_char,
}

#[cfg (feature = "windowsnt")]
static mut _wsa_errlist: [_wsa_errlist;
                          if cfg! (feature = "wsaenomore") { 63 } else { 52 }] =
  [
    _wsa_errlist { errnum: WSAEINTR, msg: c"Interrupted function call".as_ptr () }, // C 7975
    _wsa_errlist { errnum: WSAEBADF, msg: c"Bad file descriptor".as_ptr () }, // C 7976
    _wsa_errlist { errnum: WSAEACCES, msg: c"Permission denied".as_ptr () }, // C 7977
    _wsa_errlist { errnum: WSAEFAULT, msg: c"Bad address".as_ptr () },  // C 7978
    _wsa_errlist { errnum: WSAEINVAL, msg: c"Invalid argument".as_ptr () }, // C 7979
    _wsa_errlist { errnum: WSAEMFILE, msg: c"Too many open files".as_ptr () }, // C 7980
    _wsa_errlist { errnum: WSAEWOULDBLOCK, msg: c"Resource temporarily unavailable".as_ptr () }, // C 7982
    _wsa_errlist { errnum: WSAEINPROGRESS, msg: c"Operation now in progress".as_ptr () }, // C 7983
    _wsa_errlist { errnum: WSAEALREADY, msg: c"Operation already in progress".as_ptr () }, // C 7984
    _wsa_errlist { errnum: WSAENOTSOCK, msg: c"Socket operation on non-socket".as_ptr () }, // C 7985
    _wsa_errlist { errnum: WSAEDESTADDRREQ, msg: c"Destination address required".as_ptr () }, // C 7986
    _wsa_errlist { errnum: WSAEMSGSIZE, msg: c"Message too long".as_ptr () }, // C 7987
    _wsa_errlist { errnum: WSAEPROTOTYPE, msg: c"Protocol wrong type for socket".as_ptr () }, // C 7988
    _wsa_errlist { errnum: WSAENOPROTOOPT, msg: c"Bad protocol option".as_ptr () }, // C 7989
    _wsa_errlist { errnum: WSAEPROTONOSUPPORT, msg: c"Protocol not supported".as_ptr () }, // C 7990
    _wsa_errlist { errnum: WSAESOCKTNOSUPPORT, msg: c"Socket type not supported".as_ptr () }, // C 7991
    _wsa_errlist { errnum: WSAEOPNOTSUPP, msg: c"Operation not supported".as_ptr () }, // C 7992
    _wsa_errlist { errnum: WSAEPFNOSUPPORT, msg: c"Protocol family not supported".as_ptr () }, // C 7993
    _wsa_errlist { errnum: WSAEAFNOSUPPORT, msg: c"Address family not supported by protocol family".as_ptr () }, // C 7994
    _wsa_errlist { errnum: WSAEADDRINUSE, msg: c"Address already in use".as_ptr () }, // C 7995
    _wsa_errlist { errnum: WSAEADDRNOTAVAIL, msg: c"Cannot assign requested address".as_ptr () }, // C 7996
    _wsa_errlist { errnum: WSAENETDOWN, msg: c"Network is down".as_ptr () }, // C 7997
    _wsa_errlist { errnum: WSAENETUNREACH, msg: c"Network is unreachable".as_ptr () }, // C 7998
    _wsa_errlist { errnum: WSAENETRESET, msg: c"Network dropped connection on reset".as_ptr () }, // C 7999
    _wsa_errlist { errnum: WSAECONNABORTED, msg: c"Software caused connection abort".as_ptr () }, // C 8000
    _wsa_errlist { errnum: WSAECONNRESET, msg: c"Connection reset by peer".as_ptr () }, // C 8001
    _wsa_errlist { errnum: WSAENOBUFS, msg: c"No buffer space available".as_ptr () }, // C 8002
    _wsa_errlist { errnum: WSAEISCONN, msg: c"Socket is already connected".as_ptr () }, // C 8003
    _wsa_errlist { errnum: WSAENOTCONN, msg: c"Socket is not connected".as_ptr () }, // C 8004
    _wsa_errlist { errnum: WSAESHUTDOWN, msg: c"Cannot send after socket shutdown".as_ptr () }, // C 8005
    _wsa_errlist { errnum: WSAETOOMANYREFS, msg: c"Too many references".as_ptr () },  /* not sure */ // C 8006
    _wsa_errlist { errnum: WSAETIMEDOUT, msg: c"Connection timed out".as_ptr () }, // C 8007
    _wsa_errlist { errnum: WSAECONNREFUSED, msg: c"Connection refused".as_ptr () }, // C 8008
    _wsa_errlist { errnum: WSAELOOP, msg: c"Network loop".as_ptr () },  /* not sure */ // C 8009
    _wsa_errlist { errnum: WSAENAMETOOLONG, msg: c"Name is too long".as_ptr () }, // C 8010
    _wsa_errlist { errnum: WSAEHOSTDOWN, msg: c"Host is down".as_ptr () }, // C 8011
    _wsa_errlist { errnum: WSAEHOSTUNREACH, msg: c"No route to host".as_ptr () }, // C 8012
    _wsa_errlist { errnum: WSAENOTEMPTY, msg: c"Buffer not empty".as_ptr () },  /* not sure */ // C 8013
    _wsa_errlist { errnum: WSAEPROCLIM, msg: c"Too many processes".as_ptr () }, // C 8014
    _wsa_errlist { errnum: WSAEUSERS, msg: c"Too many users".as_ptr () },  /* not sure */ // C 8015
    _wsa_errlist { errnum: WSAEDQUOT, msg: c"Double quote in host name".as_ptr () },  /* really not sure */ // C 8016
    _wsa_errlist { errnum: WSAESTALE, msg: c"Data is stale".as_ptr () },  /* not sure */ // C 8017
    _wsa_errlist { errnum: WSAEREMOTE, msg: c"Remote error".as_ptr () },  /* not sure */ // C 8018
    _wsa_errlist { errnum: WSASYSNOTREADY, msg: c"Network subsystem is unavailable".as_ptr () }, // C 8020
    _wsa_errlist { errnum: WSAVERNOTSUPPORTED, msg: c"WINSOCK.DLL version out of range".as_ptr () }, // C 8021
    _wsa_errlist { errnum: WSANOTINITIALISED, msg: c"Winsock not initialized successfully".as_ptr () }, // C 8022
    _wsa_errlist { errnum: WSAEDISCON, msg: c"Graceful shutdown in progress".as_ptr () }, // C 8023
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAENOMORE, msg: c"No more operations allowed".as_ptr () },  /* not sure */ // C 8025
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAECANCELLED, msg: c"Operation cancelled".as_ptr () },  /* not sure */ // C 8026
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAEINVALIDPROCTABLE, msg: c"Invalid procedure table from service provider".as_ptr () }, // C 8027
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAEINVALIDPROVIDER, msg: c"Invalid service provider version number".as_ptr () }, // C 8028
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAEPROVIDERFAILEDINIT, msg: c"Unable to initialize a service provider".as_ptr () }, // C 8029
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSASYSCALLFAILURE, msg: c"System call failure".as_ptr () }, // C 8030
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSASERVICE_NOT_FOUND, msg: c"Service not found".as_ptr () },  /* not sure */ // C 8031
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSATYPE_NOT_FOUND, msg: c"Class type not found".as_ptr () }, // C 8032
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSA_E_NO_MORE, msg: c"No more resources available".as_ptr () },  /* really not sure */ // C 8033
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSA_E_CANCELLED, msg: c"Operation already cancelled".as_ptr () },  /* really not sure */ // C 8034
#[cfg (feature = "wsaenomore")]
    _wsa_errlist { errnum: WSAEREFUSED, msg: c"Operation refused".as_ptr () },  /* not sure */ // C 8035
    _wsa_errlist { errnum: WSAHOST_NOT_FOUND, msg: c"Host not found".as_ptr () }, // C 8038
    _wsa_errlist { errnum: WSATRY_AGAIN, msg: c"Authoritative host not found during name lookup".as_ptr () }, // C 8039
    _wsa_errlist { errnum: WSANO_RECOVERY, msg: c"Non-recoverable error during name lookup".as_ptr () }, // C 8040
    _wsa_errlist { errnum: WSANO_DATA, msg: c"Valid name, no data record of requested type".as_ptr () }, // C 8041
    _wsa_errlist { errnum: -1, msg: std::ptr::null () },                // C 8043
  ];

/* C 8046-8061: `sys_strerror`.  C 8052/8053 read the CRT's own error table: `sys_nerr` is
   ms-w32.h 460-462's rename and has a macro mirror (`sys_nerr! ()`), whose body names
   `_sys_nerr` - so the unit declares the two names C's lines read (P-88, `UNVERIFIED`;
   Owed 25), the first with the mirror's own spelling and the second with C's.  */
/// C: extern int _sys_nerr;  (`<stdio.h>`; ms-w32.h 461 renames C's `sys_nerr` to it)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut _sys_nerr: c_int;
}
/// C: extern char *sys_errlist[];  (`<stdio.h>`)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut sys_errlist: [*mut c_char; 0];
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_strerror (error_no: c_int) -> *mut c_char  // C 8046
{                                                                       // C 8048
  let mut i: c_int;                                                     // C 8049
  static mut unknown_msg: [c_char; 40] = [0; 40];                       // C 8050

  if error_no >= 0 && error_no < sys_nerr! ()                           // C 8052
    { return sys_errlist[error_no as usize]; }                          // C 8053

  i = 0;                                                                // C 8055
  while _wsa_errlist[i as usize].errnum >= 0                            // C 8055
    {                                                                   // C 8055 (C's loop body)
      if _wsa_errlist[i as usize].errnum == error_no                    // C 8056
        { return _wsa_errlist[i as usize].msg as *mut c_char; }         // C 8057 (C's cast)
      i += 1;                                                           // C 8055 (the for's increment)
    }                                                                   // C 8058

  sprintf (unknown_msg.as_mut_ptr (), c"Unidentified error: %d".as_ptr (), error_no); // C 8059
  return unknown_msg.as_mut_ptr ();                                     // C 8060
}                                                                       // C 8061


/* C 8062's blank and C 8063-8065's `[andrewi 3-May-96]` note, carried as C's own text.  C 8067's
   `SOCK_HANDLE` is this file's last `missing macro`: C's cast is Rust's `as SOCKET` and the
   index is `as usize` into the `fd_info` array C 3300 defines and this unit already carries
   (`femacs/src/w32.rs` 5355).  C 8069's prototype of `socket_to_fd` has no Rust item
   (PORTING.md 0.1-12), so the name stays unresolved until the next slice defines it - the
   slice-12 `is_symlink` reading.  `INVALID_SOCKET` (`<winsock2.h>`'s `(SOCKET) (~0)`) is
   declared in-unit (P-88, `UNVERIFIED`, Owed 26), like `INVALID_HANDLE_VALUE` (C 1540).  */
/* [andrewi 3-May-96] I've had conflicting results using both methods,
   but I believe the method of keeping the socket handle separate (and
   insuring it is not inheritable) is the correct one. */

/// C: #define SOCK_HANDLE(fd) ((SOCKET) fd_info[fd].hnd)
#[cfg (feature = "windowsnt")]
macro_rules! SOCK_HANDLE { ($fd:expr) => { (fd_info[$fd as usize].hnd) as SOCKET }; }
/// C: #define INVALID_SOCKET (SOCKET) (~0)  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const INVALID_SOCKET: SOCKET = -1isize as SOCKET;

/* C 8069: `static int socket_to_fd (SOCKET s);` - C's own forward declaration of the function
   right below; the port needs no item for it (PORTING.md 0.1-12), so the line is carried here.  */

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_socket (af: c_int, r#type: c_int, protocol: c_int) -> c_int // C 8071
{                                                                       // C 8073
  let mut s: SOCKET;                                                    // C 8074

  /* C leaves `s` uninitialised (PORTING.md 1.5).  */
  s = 0;

  if winsock_lib.is_null ()                                             // C 8076 (C's `== NULL`)
    {                                                                   // C 8077
      *errno () = ENETDOWN;                                             // C 8078
      return -1;                                                        // C 8079
    }                                                                   // C 8080

  check_errno ();                                                       // C 8082

  /* call the real socket function */                                   // C 8084
  s = pfn_socket.unwrap () (af, r#type, protocol) as SOCKET;            // C 8085 (C's call-through and widening)

  if s != INVALID_SOCKET                                                // C 8087
    { return socket_to_fd (s); }                                        // C 8088

  set_errno ();                                                         // C 8090
  return -1;                                                            // C 8091
}                                                                       // C 8092


/* C 8093's blank and C 8094's `/* Convert a SOCKET to a file descriptor.  */`.  This slice is
   the first reader of `src/w32.h` 74-113's `child_process`, which the `include/w32.rs` mirror
   *defers* (its note 34-42: C 100's `PROCESS_INFORMATION` and C 110-111's two `OVERLAPPED`
   records are members **by value** and come from `<windows.h>`, which has no mirror), so the
   unit spells the record and those three - exactly the way it spells the four SDK records of
   Deviations 18 (C 420/430's blocks) and the slice-20 socket types: documented layouts, every
   one of them `UNVERIFIED` (P-88, Owed 27).  C's per-member prose (w32.h 76-112) is not
   carried, as in the mirror's own deferred note; C's member names, order and types are.  */
/* Convert a SOCKET to a file descriptor.  */
/// C: w32.h 74: `typedef struct _child_process`
/// C: typedef struct _PROCESS_INFORMATION { HANDLE hProcess; HANDLE hThread; DWORD dwProcessId;
/// C: DWORD dwThreadId; } PROCESS_INFORMATION;  (`<winbase.h>`; 24 bytes)
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct PROCESS_INFORMATION
{
  pub hProcess: HANDLE,
  pub hThread: HANDLE,
  pub dwProcessId: DWORD,
  pub dwThreadId: DWORD,
}
/// C: typedef struct _OVERLAPPED { ULONG_PTR Internal; ULONG_PTR InternalHigh;
/// C: union { struct { DWORD Offset; DWORD OffsetHigh; }; PVOID Pointer; }; HANDLE hEvent; }
/// C: OVERLAPPED;  (`<winbase.h>`; 32 bytes.  C 99-100's union and its inner struct are
/// anonymous, so the port names them after the container (P-67): `OVERLAPPED_inner` and
/// `OVERLAPPED_offset`.)
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct OVERLAPPED
{
  pub Internal: ULONG_PTR,
  pub InternalHigh: ULONG_PTR,
  pub inner: OVERLAPPED_inner,
  pub hEvent: HANDLE,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub union OVERLAPPED_inner
{
  pub offset: OVERLAPPED_offset,
  pub Pointer: *mut c_void,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct OVERLAPPED_offset
{
  pub Offset: DWORD,
  pub OffsetHigh: DWORD,
}
/// C: w32.h 78: `int                 fd;`  (C's eleven members, in C's order: `fd`, `pid`,
/// C: `char_avail`, `char_consumed`, `thrd`, `hwnd`, `procinfo`, `status`, `errcode`, `chr`,
/// C: `ovl_read`, `ovl_write`; C's `volatile` has no ABI effect on this target.)
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct _child_process
{
  pub fd: c_int,
  pub pid: c_int,
  pub char_avail: HANDLE,
  pub char_consumed: HANDLE,
  pub thrd: HANDLE,
  pub hwnd: *mut c_void,
  pub procinfo: PROCESS_INFORMATION,
  pub status: c_int,
  pub errcode: c_int,
  pub chr: c_char,
  pub ovl_read: OVERLAPPED,
  pub ovl_write: OVERLAPPED,
}
#[cfg (feature = "windowsnt")]
pub type child_process = _child_process;

/// C: #define DUPLICATE_SAME_ACCESS 0x00000002  (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const DUPLICATE_SAME_ACCESS: DWORD = 0x00000002;
/// C: #define FIONBIO 0x8004667e  (`<winsock2.h>`; `_IOW ('f', 126, u_long)` on this SDK)
#[cfg (feature = "windowsnt")]
pub const FIONBIO: c_long = 0x8004667e;
/// C: #define _O_RDWR 0x0002  (the CRT's `<fcntl.h>`; the underscore spelling, not gnulib's)
#[cfg (feature = "windowsnt")]
pub const _O_RDWR: c_int = 0x0002;
/// C: WINBASEAPI BOOL WINAPI DuplicateHandle (HANDLE, HANDLE, HANDLE, LPHANDLE, DWORD, BOOL, DWORD);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn DuplicateHandle (hSourceProcessHandle: HANDLE, hSourceHandle: HANDLE,
                          hTargetProcessHandle: HANDLE, lpTargetHandle: *mut HANDLE,
                          dwDesiredAccess: DWORD, bInheritHandle: BOOL,
                          dwOptions: DWORD) -> BOOL;
}

/// C: int _close (int);  (the CRT's `<io.h>`; the `lib/include/unistd.in.rs` copy of the name is
/// its `windows`-arm gnulib rename, so for this arm the name is the including unit's)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _close (fd: c_int) -> c_int;
}

/* Although under NT 3.5 _open_osfhandle will accept a socket           // C 8101
   handle, if opened with SO_OPENTYPE == SO_SYNCHRONOUS_NONALERT,       // C 8102
   that does not work under NT 3.1.  However, we can get the same       // C 8103
   effect by using a backdoor function to replace an existing           // C 8104
   descriptor handle with the one we want. */                           // C 8105

/* allocate a file descriptor (with appropriate flags) */               // C 8107

#[cfg (feature = "windowsnt")]
unsafe fn socket_to_fd (s: SOCKET) -> c_int                             // C 8095 (C's `static int` + `socket_to_fd (SOCKET s)`)
{                                                                       // C 8097
  let mut fd: c_int;                                                    // C 8098
  let mut cp: *mut child_process;                                       // C 8099

  /* C leaves `fd` and `cp` uninitialised (PORTING.md 1.5).  */
  fd = 0;
  cp = std::ptr::null_mut ();

  fd = _open (c"NUL:".as_ptr (), _O_RDWR);                              // C 8108
  if fd >= 0                                                            // C 8109
    {                                                                   // C 8110
      /* Make a non-inheritable copy of the socket handle.  Note        // C 8111
	 that it is possible that sockets aren't actually kernel               // C 8112
	 handles, which appears to be the case on Windows 9x when              // C 8113
	 the MS Proxy winsock client is installed.  */                         // C 8114
      {                                                                 // C 8115 (C's nested block)
	/* Apparently there is a bug in NT 3.51 with some service              // C 8116
	   packs, which prevents using DuplicateHandle to make a               // C 8117
	   socket handle non-inheritable (causes WSACleanup to                 // C 8118
	   hang).  The work-around is to use SetHandleInformation              // C 8119
	   instead if it is available and implemented. */                      // C 8120
	if pfn_SetHandleInformation.is_some ()                                 // C 8121 (C's truth test)
	  {                                                                    // C 8122
	    pfn_SetHandleInformation.unwrap () (s as HANDLE, HANDLE_FLAG_INHERIT, 0); // C 8123 (C's cast + call-through)
	  }                                                                    // C 8124
	else                                                                   // C 8125
	  {                                                                    // C 8126
	    let parent: HANDLE = GetCurrentProcess ();                         // C 8127
	    let mut new_s: HANDLE = INVALID_HANDLE_VALUE;                      // C 8128 (C's `= INVALID_HANDLE_VALUE`)

	    if DuplicateHandle (parent,                                        // C 8130
				s as HANDLE,                                                        // C 8131 (C's cast)
				parent,                                                             // C 8132
				&mut new_s,                                                         // C 8133
				0,                                                                  // C 8134
				FALSE,                                                              // C 8135
				DUPLICATE_SAME_ACCESS) != 0                                         // C 8136 (C's truth test)
	      {                                                                // C 8137
		/* It is possible that DuplicateHandle succeeds even                  // C 8138
		   though the socket wasn't really a kernel handle,                   // C 8139
		   because a real handle has the same value.  So                      // C 8140
		   test whether the new handle really is a socket.  */                // C 8141
		let mut nonblocking: c_ulong = 0;                                     // C 8142 (C's `unsigned long`)
		if pfn_ioctlsocket.unwrap () (new_s as SOCKET, FIONBIO, &mut nonblocking) == 0 // C 8143 (C's call-through)
		  {                                                                   // C 8144
		    pfn_closesocket.unwrap () (s);                                    // C 8145 (C's call-through)
		    s = new_s as SOCKET;                                              // C 8146 (C's cast)
		  }                                                                   // C 8147
		else                                                                  // C 8148
		  {                                                                   // C 8149
		    CloseHandle (new_s);                                              // C 8150
		  }                                                                   // C 8151
	      }                                                                // C 8152
	  }                                                                    // C 8153
      }                                                                 // C 8154
      eassert! (fd < MAXDESC);                                          // C 8155
      fd_info[fd as usize].hnd = s as HANDLE;                           // C 8156 (C's cast)

      /* set our own internal flags */                                  // C 8158
      fd_info[fd as usize].flags = (FILE_SOCKET | FILE_BINARY | FILE_READ | FILE_WRITE) as c_uint; // C 8159

      cp = new_child () as *mut child_process;                          // C 8161 (the mirror's `*mut c_void` return)
      if ! cp.is_null ()                                                // C 8162 (C's truth test)
	{                                                                      // C 8163
	  (*cp).fd = fd;                                                       // C 8164
	  (*cp).status = STATUS_READ_ACKNOWLEDGED;                             // C 8165

	  /* attach child_process to fd_info */                                // C 8167
	  if ! fd_info[fd as usize].cp.is_null ()                              // C 8168 (C's `!= NULL`)
	    {                                                                  // C 8169
	      DebPrint! (("sys_socket: fd_info[%d] apparently in use!\n", fd)); // C 8170
	      emacs_abort ();                                                  // C 8171
	    }                                                                  // C 8172

	  fd_info[fd as usize].cp = cp as *mut c_void;                         // C 8174 (the mirror's `*mut c_void` depth)

	  /* success! */                                                       // C 8176
	  winsock_inuse += 1;                                                  // C 8177 (C's `winsock_inuse++`)
	  return fd;                                                           // C 8178
	}                                                                      // C 8179

      /* clean up */                                                    // C 8181
      _close (fd);                                                      // C 8182
    }                                                                   // C 8183
  else                                                                  // C 8184
    {
      pfn_closesocket.unwrap () (s);                                    // C 8185 (C's call-through)
    }
  *errno () = EMFILE;                                                   // C 8186
  return -1;                                                            // C 8187
}                                                                       // C 8188


/* C 8189's blank and C 8190-8191's `int` + `sys_bind`, then C 8211-8212's `sys_connect`.  Both
   are the socket entry points' shape: the `winsock_lib` guard, `check_errno`, the `flags &
   FILE_SOCKET` test, the `pfn_*` call through `SOCK_HANDLE` (this file's macro, slice 24) and
   `set_errno` on `SOCKET_ERROR`.  `SOCKET_ERROR` (`<winsock2.h>`'s `-1`) is declared in-unit
   (P-88, `UNVERIFIED`, Owed 28); `ENOTSOCK`/`EWOULDBLOCK`/`EINPROGRESS` are the socket
   mirror's rows (C 149-172's renames) and `FILE_SOCKET`/`FILE_NDELAY`/`FILE_CONNECT` are
   `w32.h` 130-142's flags, which the `include/w32.rs` mirror carries.  */
/// C: #define SOCKET_ERROR (-1)  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const SOCKET_ERROR: c_int = -1;

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_bind (s: c_int, addr: *const sockaddr, namelen: c_int) -> c_int // C 8190
{                                                                       // C 8192
  if winsock_lib.is_null ()                                             // C 8193 (C's `== NULL`)
    {                                                                   // C 8194
      *errno () = ENOTSOCK;                                             // C 8195
      return SOCKET_ERROR;                                              // C 8196
    }                                                                   // C 8197

  check_errno ();                                                       // C 8199
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8200 (C's truth test)
    {                                                                   // C 8201
      let rc: c_int = pfn_bind.unwrap () (SOCK_HANDLE! (s), addr, namelen); // C 8202 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8203
	{ set_errno (); }                                                      // C 8204
      return rc;                                                        // C 8205
    }                                                                   // C 8206
  *errno () = ENOTSOCK;                                                 // C 8207
  return SOCKET_ERROR;                                                  // C 8208
}                                                                       // C 8209

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_connect (s: c_int, name: *const sockaddr, namelen: c_int) -> c_int // C 8211
{                                                                       // C 8213
  if winsock_lib.is_null ()                                             // C 8214 (C's `== NULL`)
    {                                                                   // C 8215
      *errno () = ENOTSOCK;                                             // C 8216
      return SOCKET_ERROR;                                              // C 8217
    }                                                                   // C 8218

  check_errno ();                                                       // C 8220
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8221 (C's truth test)
    {                                                                   // C 8222
      let rc: c_int = pfn_connect.unwrap () (SOCK_HANDLE! (s), name, namelen); // C 8223 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8224
	{                                                                      // C 8225
	  set_errno ();                                                        // C 8226
	  /* If this is a non-blocking 'connect', set the bit in flags         // C 8227
	     that will tell reader_thread to wait for connection               // C 8228
	     before trying to read.  */                                        // C 8229
	  if *errno () == EWOULDBLOCK && (fd_info[s as usize].flags & FILE_NDELAY as c_uint) != 0 // C 8230
	    {                                                                  // C 8231
	      *errno () = EINPROGRESS; /* that's what process.c expects */     // C 8232
	      fd_info[s as usize].flags |= FILE_CONNECT as c_uint;             // C 8233
	    }                                                                  // C 8234
	}                                                                      // C 8235
      return rc;                                                        // C 8236
    }                                                                   // C 8237
  *errno () = ENOTSOCK;                                                 // C 8238
  return SOCKET_ERROR;                                                  // C 8239
}                                                                       // C 8240


/* C 8241's blank and C 8242-8274's five one-line wrappers: `sys_htons`/`sys_ntohs` (C 8242-8254),
   `sys_htonl`/`sys_ntohl` (C 8255-8267) and `sys_inet_addr` (C 8269-8274).  Each is one C
   ternary over `winsock_lib` (`P ? pfn_X (arg) : arg`), written `return if … { … } else { … };`
   (the C 6619 site's device, whose marker names C's ternary).  `INADDR_NONE`
   (`<winsock2.h>`'s `0xffffffff`) is declared in-unit (P-88, `UNVERIFIED`, Owed 29); C's
   `unsigned long` return is slice 20's `pfn_inet_addr` spelling (`c_ulong`), and `u_short`/
   `u_long` are this unit's own type aliases (C 7760-7781's block).  */
/// C: #define INADDR_NONE 0xffffffff  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const INADDR_NONE: c_ulong = 0xffffffff;

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_htons (hostshort: u_short) -> u_short      // C 8242
{                                                                       // C 8244
  return if ! winsock_lib.is_null ()                                    // C 8245 (C's ternary)
    { pfn_htons.unwrap () (hostshort) }                                 // C 8246 (C's call-through)
    else { hostshort };                                                 // C 8246
}                                                                       // C 8247

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_ntohs (netshort: u_short) -> u_short       // C 8249
{                                                                       // C 8251
  return if ! winsock_lib.is_null ()                                    // C 8252 (C's ternary)
    { pfn_ntohs.unwrap () (netshort) }                                  // C 8253 (C's call-through)
    else { netshort };                                                  // C 8253
}                                                                       // C 8254
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_htonl (hostlong: u_long) -> u_long         // C 8255
{                                                                       // C 8257
  return if ! winsock_lib.is_null ()                                    // C 8258 (C's ternary)
    { pfn_htonl.unwrap () (hostlong) }                                  // C 8259 (C's call-through)
    else { hostlong };                                                  // C 8259
}                                                                       // C 8260

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_ntohl (netlong: u_long) -> u_long          // C 8262
{                                                                       // C 8264
  return if ! winsock_lib.is_null ()                                    // C 8265 (C's ternary)
    { pfn_ntohl.unwrap () (netlong) }                                   // C 8266 (C's call-through)
    else { netlong };                                                   // C 8266
}                                                                       // C 8267

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_inet_addr (cp: *const c_char) -> c_ulong   // C 8269
{                                                                       // C 8271
  return if ! winsock_lib.is_null ()                                    // C 8272 (C's ternary)
    { pfn_inet_addr.unwrap () (cp) }                                    // C 8273 (C's call-through)
    else { INADDR_NONE };                                               // C 8273
}                                                                       // C 8274


/* C 8275's blank, C 8276-8277's `Wrapper for gethostname` comment (C's own text), and C 8278-8308's
   `sys_gethostname`: the `winsock_lib` arm through slice 20's `pfn_gethostname`, the
   `GetComputerNameA` arm C 8292 selects by `sizeof (sname)`, the `EFAULT` exit, and C 8303-8306's
   UTF-8 conversion tail (`filename_from_ansi`, `strncpy`, the NUL store).  `MAX_COMPUTERNAME_LENGTH`
   (`<winbase.h>`'s 15) and `GetComputerNameA` are declared in-unit (P-88, `UNVERIFIED`, Owed 30);
   `MAX_UTF8_PATH` is the `ms-w32.h` mirror's 93 row and `filename_from_ansi` this unit's own
   definition (C 833's block).  */
/* Wrapper for gethostname.  Note: NAMELEN is the space available in
   NAME excluding the terminating null.  */
/// C: #define MAX_COMPUTERNAME_LENGTH 15  (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const MAX_COMPUTERNAME_LENGTH: c_int = 15;
/// C: WINBASEAPI BOOL WINAPI GetComputerNameA (LPSTR lpBuffer, LPDWORD nSize);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetComputerNameA (lpBuffer: *mut c_char, nSize: *mut DWORD) -> BOOL;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_gethostname (name: *mut c_char, namelen: c_int) -> c_int // C 8278
{                                                                       // C 8280
  let mut retval: c_int;                                                // C 8281
  let mut nlen: c_int = namelen;                                        // C 8282 (C's `int nlen = namelen;`)
  let mut sname: [c_char; 256 + 1] = [0; 256 + 1];                      // C 8283 (C's `char sname[256+1]`)

  /* C leaves `retval` uninitialised (PORTING.md 1.5).  */
  retval = 0;

  if ! winsock_lib.is_null ()                                           // C 8285 (C's `!= NULL`)
    {                                                                   // C 8286
      check_errno ();                                                   // C 8287
      retval = pfn_gethostname.unwrap () (sname.as_mut_ptr (),          // C 8288 (C's call-through)
					 core::mem::size_of_val (&sname) as c_int);                        // C 8288 (C's `sizeof (sname)`)
      if retval == SOCKET_ERROR                                         // C 8289
	{ set_errno (); }                                                      // C 8290
    }                                                                   // C 8291
  else if core::mem::size_of_val (&sname) > MAX_COMPUTERNAME_LENGTH as usize // C 8292 (C's `sizeof` and its int comparison)
    { retval = (GetComputerNameA (sname.as_mut_ptr (),                  // C 8293 (C's `!` and `(DWORD *)` cast)
				       &raw mut nlen as *mut DWORD) == 0) as c_int; }               // C 8293
  else                                                                  // C 8294
    {                                                                   // C 8295
      retval = SOCKET_ERROR;                                            // C 8296
      *errno () = EFAULT;                                               // C 8297
    }                                                                   // C 8298
  /* The rest of the code wants the name in UTF-8.  The host name is not // C 8299
     a file name, but it's encoded in the ANSI codepage and its size    // C 8300
     must be at most 256 characters.  So treating it as a file name     // C 8301
     should be okay.  */                                                // C 8302
  let mut hostname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize]; // C 8303
  filename_from_ansi (sname.as_mut_ptr (), hostname.as_mut_ptr ());     // C 8304
  strncpy (name, hostname.as_mut_ptr (), namelen as usize);             // C 8305 (C's int-to-`size_t`)
  *name.offset (namelen as isize) = 0;                                  // C 8306 (C's `name[namelen] = '\0'`)
  return retval;                                                        // C 8307
}                                                                       // C 8308


/* C 8309's blank; C 8310-8333's `sys_gethostbyname` and C 8335-8351's `sys_getservbyname` - the
   two name-lookup wrappers: the `winsock_lib` guards, the `pfn_*` call-throughs of slice 20's
   statics, and `sys_gethostbyname`'s C 8314/8328/8331 `h_errno` saves and stores (this unit's own
   C 7930 object, slice 22).  `NO_RECOVERY` (`<netdb.h>`'s 3 - no header in the reference tree
   defines it) is declared in-unit (P-88, `UNVERIFIED`, Owed 31).  */
/// C: #define NO_RECOVERY 3  (`<netdb.h>`)
#[cfg (feature = "windowsnt")]
pub const NO_RECOVERY: c_int = 3;

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_gethostbyname (name: *const c_char) -> *mut hostent // C 8310
{                                                                       // C 8312
  let mut host: *mut hostent;                                           // C 8313
  let mut h_err: c_int = h_errno;                                       // C 8314 (this unit's own C 7930 object)

  /* C leaves `host` uninitialised (PORTING.md 1.5).  */
  host = std::ptr::null_mut ();

  if winsock_lib.is_null ()                                             // C 8316 (C's `== NULL`)
    {                                                                   // C 8317
      h_errno = NO_RECOVERY;                                            // C 8318
      *errno () = ENETDOWN;                                             // C 8319
      return std::ptr::null_mut ();                                     // C 8320 (C's `NULL`)
    }                                                                   // C 8321

  check_errno ();                                                       // C 8323
  host = pfn_gethostbyname.unwrap () (name);                            // C 8324 (C's call-through)
  if host.is_null ()                                                    // C 8325 (C's `!host`)
    {                                                                   // C 8326
      set_errno ();                                                     // C 8327
      h_errno = *errno ();                                              // C 8328 (C's `errno` read)
    }                                                                   // C 8329
  else                                                                  // C 8330
    { h_errno = h_err; }                                                // C 8331
  return host;                                                          // C 8332
}                                                                       // C 8333

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_getservbyname (name: *const c_char, proto: *const c_char) -> *mut servent // C 8335
{                                                                       // C 8337
  let mut serv: *mut servent;                                           // C 8338

  /* C leaves `serv` uninitialised (PORTING.md 1.5).  */
  serv = std::ptr::null_mut ();

  if winsock_lib.is_null ()                                             // C 8340 (C's `== NULL`)
    {                                                                   // C 8341
      *errno () = ENETDOWN;                                             // C 8342
      return std::ptr::null_mut ();                                     // C 8343 (C's `NULL`)
    }                                                                   // C 8344

  check_errno ();                                                       // C 8346
  serv = pfn_getservbyname.unwrap () (name, proto);                     // C 8347 (C's call-through)
  if serv.is_null ()                                                    // C 8348 (C's `!serv`)
    { set_errno (); }                                                   // C 8349
  return serv;                                                          // C 8350
}                                                                       // C 8351

/* C 8352's blank and C 8353-8372's `sys_getpeername`: the same shape as `sys_bind`/`sys_connect`
   (slice 26) - the `winsock_lib` guard storing `ENETDOWN`, `check_errno`, the `flags & FILE_SOCKET`
   test, the `pfn_getpeername` call through `SOCK_HANDLE` and the two `SOCKET_ERROR` exits.  Every
   name it uses is already in scope; the three locals are C's own parameters.  */

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_getpeername (s: c_int, addr: *mut sockaddr, namelen: *mut c_int) -> c_int // C 8353
{                                                                       // C 8355
  if winsock_lib.is_null ()                                             // C 8356 (C's `== NULL`)
    {                                                                   // C 8357
      *errno () = ENETDOWN;                                             // C 8358
      return SOCKET_ERROR;                                              // C 8359
    }                                                                   // C 8360

  check_errno ();                                                       // C 8362
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8363 (C's truth test)
    {                                                                   // C 8364
      let rc: c_int = pfn_getpeername.unwrap () (SOCK_HANDLE! (s), addr, namelen); // C 8365 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8366
	{ set_errno (); }                                                      // C 8367
      return rc;                                                        // C 8368
    }                                                                   // C 8369
  *errno () = ENOTSOCK;                                                 // C 8370
  return SOCKET_ERROR;                                                  // C 8371
}                                                                       // C 8372


/* C 8373's blank and C 8374-8464's `sys_getaddrinfo` - one piece (its body holds C's single `else`
   block, so no cut inside it keeps the braces balanced).  The five names it needs are spelled
   here, all `UNVERIFIED` (P-88, Owed 32): `<winsock2.h>`'s `SOCK_DGRAM` (2), `in_addr` and
   `sockaddr_in` (C 8395/8412-8420 read them by value), `xzalloc` (C 8423; `xalloc.h`'s name - the
   port's `lib/include/xalloc.rs` mirror is not in this unit's include graph); `strtoul` (C 8416) is
   the `stdlib.in.rs` mirror's own declaration - the arm reading's first run of this slice measured one
   `E0428` for an in-unit copy of it, so the unit does not re-spell it.  C 8431/8433's `h_addr` is `<netdb.h>`'s
   `#define h_addr h_addr_list[0]`, so the port writes `*(*host_info).h_addr_list`.  `WSAEINVAL`
   (C 8400) and `WSAHOST_NOT_FOUND` (C 8403/8418/8441) are slices 22/23's, and
   `sys_getservbyname`/`sys_gethostbyname`/`sys_inet_addr`/`sys_htons` slices 27/29's.  */
/// C: #define SOCK_DGRAM 2  (`<winsock2.h>`)
#[cfg (feature = "windowsnt")]
pub const SOCK_DGRAM: c_int = 2;
/// C: typedef struct in_addr { u_long s_addr; } in_addr;  (`<winsock2.h>`; 4 bytes)
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct in_addr
{
  pub s_addr: u_long,
}
/// C: typedef struct sockaddr_in { short sin_family; u_short sin_port; struct in_addr sin_addr;
/// C: char sin_zero[8]; } sockaddr_in;  (`<winsock2.h>`; 16 bytes)
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct sockaddr_in
{
  pub sin_family: c_short,
  pub sin_port: u_short,
  pub sin_addr: in_addr,
  pub sin_zero: [c_char; 8],
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_getaddrinfo (node: *const c_char, service: *const c_char, // C 8374
                                          hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int // C 8375
{                                                                       // C 8377
  let mut rc: c_int;                                                    // C 8379

  /* C leaves `rc` uninitialised (PORTING.md 1.5).  */
  rc = 0;

  if winsock_lib.is_null ()                                             // C 8380 (C's `== NULL`)
    {                                                                   // C 8381
      *errno () = ENETDOWN;                                             // C 8382
      return SOCKET_ERROR;                                              // C 8383
    }                                                                   // C 8384

  check_errno ();                                                       // C 8386
  if pfn_getaddrinfo.is_some ()                                         // C 8387 (C's truth test)
    { rc = pfn_getaddrinfo.unwrap () (node, service, hints, res); }     // C 8388 (C's call-through)
  else                                                                  // C 8389
    {                                                                   // C 8390
      let mut port: c_int = 0;                                          // C 8391 (C's `int port = 0;`)
      let mut host_info: *mut hostent;                                  // C 8392
      /// C: struct gai_storage { struct addrinfo addrinfo; struct sockaddr_in sockaddr_in; };
      /// (C 8393-8396; the record has no tag, so the type takes the variable's name - P-67 - and
      /// the unit spells it where C declares it, in the block's own scope.)
      #[repr(C)]
      struct gai_storage
      {
	addrinfo: addrinfo,                                                    // C 8394
	sockaddr_in: sockaddr_in,                                              // C 8395
      }
      let mut gai_storage: *mut gai_storage;                            // C 8396

      /* C leaves `host_info` and `gai_storage` uninitialised (PORTING.md 1.5).  */
      host_info = std::ptr::null_mut ();
      gai_storage = std::ptr::null_mut ();

      /* We don't (yet) support any flags, as Emacs doesn't need that.  */ // C 8398
      if ! hints.is_null () && (*hints).ai_flags != 0                   // C 8399
	{ return WSAEINVAL; }                                                  // C 8400
      /* NODE cannot be NULL, since process.c has fallbacks for that.  */ // C 8401
      if node.is_null ()                                                // C 8402 (C's `!node`)
	{ return WSAHOST_NOT_FOUND; }                                          // C 8403

      if ! service.is_null ()                                           // C 8405
	{                                                                      // C 8406
	  let protocol: *const c_char = if ! hints.is_null () && (*hints).ai_socktype == SOCK_DGRAM // C 8407-8408 (C's ternary)
	    { c"udp".as_ptr () } else { c"tcp".as_ptr () };                    // C 8408
	  let mut srv: *mut servent = sys_getservbyname (service, protocol);   // C 8409

	  if ! srv.is_null ()                                                  // C 8410
	    { port = (*srv).s_port as c_int; }                                 // C 8411 (C's `short`-to-`int`)
	  else if *service >= b'0' as c_char && *service <= b'9' as c_char     // C 8412
	    {                                                                  // C 8413
	      let mut endp: *mut c_char;                                       // C 8414

	      /* C leaves `endp` uninitialised (PORTING.md 1.5).  */
	      endp = std::ptr::null_mut ();

	      port = strtoul (service, &raw mut endp, 10) as c_int;            // C 8416 (C's `unsigned long`-to-`int`)
	      if *endp != 0 || port > 65536                                    // C 8417 (C's truth test)
		{ return WSAHOST_NOT_FOUND; }                                         // C 8418
	      port = sys_htons (port as u_short) as c_int;                     // C 8419 (C's `(unsigned short)` and the widening store)
	    }                                                                  // C 8420
	  else                                                                 // C 8421
	    { return WSAHOST_NOT_FOUND; }                                      // C 8422
	}                                                                      // C 8423

      gai_storage = xzalloc (core::mem::size_of::<gai_storage> ()) as *mut gai_storage; // C 8424 (C's `sizeof *gai_storage`)
      (*gai_storage).sockaddr_in.sin_port = port as u_short;            // C 8425 (C's `int`-to-`u_short`)
      host_info = sys_gethostbyname (node);                             // C 8426
      if ! host_info.is_null ()                                         // C 8427
	{                                                                      // C 8428
	  memcpy (&raw mut (*gai_storage).sockaddr_in.sin_addr as *mut c_void, // C 8429
		  *(*host_info).h_addr_list as *const c_void,                         // C 8431 (C's `h_addr` macro; C 8430 is the continuation)
		  (*host_info).h_length as usize);                                    // C 8432 (C's `short`-to-`size_t`)
	  (*gai_storage).sockaddr_in.sin_family = (*host_info).h_addrtype;     // C 8433
	}                                                                      // C 8434
      else                                                              // C 8435
	{                                                                      // C 8436
	  /* Attempt to interpret host as numeric inet address.  */            // C 8437
	  let mut numeric_addr: c_ulong = sys_inet_addr (node);                // C 8438

	  if numeric_addr == -1i32 as c_ulong                                  // C 8439 (C's `-1` against a 32-bit `unsigned long`)
	    {                                                                  // C 8440
	      free (gai_storage as *mut c_void);                               // C 8441
	      return WSAHOST_NOT_FOUND;                                        // C 8442
	    }                                                                  // C 8443

	  memcpy (&raw mut (*gai_storage).sockaddr_in.sin_addr as *mut c_void, // C 8444
		  &raw const numeric_addr as *const c_void,                           // C 8445
		  core::mem::size_of_val (&(*gai_storage).sockaddr_in.sin_addr));     // C 8446 (C's `sizeof (…)`)
	  (*gai_storage).sockaddr_in.sin_family                                // C 8447
	    = if ! hints.is_null () { (*hints).ai_family as c_short } else { 0 }; // C 8447 (C's ternary)
	}                                                                      // C 8448

      (*gai_storage).addrinfo.ai_addr = &raw mut (*gai_storage).sockaddr_in as *mut sockaddr; // C 8450-8451 (C's cast)
      (*gai_storage).addrinfo.ai_addrlen = core::mem::size_of_val (&(*gai_storage).sockaddr_in); // C 8452
      (*gai_storage).addrinfo.ai_protocol                               // C 8453
	= if ! hints.is_null () { (*hints).ai_protocol } else { 0 };           // C 8453 (C's ternary)
      (*gai_storage).addrinfo.ai_socktype                               // C 8454
	= if ! hints.is_null () { (*hints).ai_socktype } else { 0 };           // C 8454 (C's ternary)
      (*gai_storage).addrinfo.ai_family = (*gai_storage).sockaddr_in.sin_family as c_int; // C 8455 (C's `short`-to-`int`)
      (*gai_storage).addrinfo.ai_next = std::ptr::null_mut ();          // C 8456 (C's `NULL`)

      *res = &raw mut (*gai_storage).addrinfo;                          // C 8458
      rc = 0;                                                           // C 8459
    }                                                                   // C 8460

  return rc;                                                            // C 8463
}                                                                       // C 8464


/* C 8465's blank and C 8466-8549's four wrappers: `sys_freeaddrinfo` (8466-8483, the
   `pfn_freeaddrinfo`/`eassert`+`xfree` pair), `sys_shutdown` (8485-8504), `sys_setsockopt`
   (8506-8526, with C 8519's `(const char *)` cast) and `sys_listen` (8528-8549, the only one that
   also raises a flag - C 8544's `fd_info[s].flags |= FILE_LISTEN`).  Every name is in scope: the
   four `pfn_*` are slice 20's statics, `FILE_LISTEN` is `w32.h`'s flag (the `include/w32.rs`
   mirror, its 251 row), `xfree` the `<lisp.h>`-family call the file already makes (`as *mut
   c_void`, C 2390's spelling), `eassert` the `include/lisp.rs` 392 macro and `SOCK_HANDLE`
   slice 24's.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_freeaddrinfo (ai: *mut addrinfo)           // C 8466
{                                                                       // C 8468
  if winsock_lib.is_null ()                                             // C 8469 (C's `== NULL`)
    {                                                                   // C 8470
      *errno () = ENETDOWN;                                             // C 8471
      return;                                                           // C 8472
    }                                                                   // C 8473

  check_errno ();                                                       // C 8475
  if pfn_freeaddrinfo.is_some ()                                        // C 8476 (C's truth test)
    { pfn_freeaddrinfo.unwrap () (ai); }                                // C 8477 (C's call-through)
  else                                                                  // C 8478
    {                                                                   // C 8479
      eassert! ((*ai).ai_next.is_null ());                              // C 8480 (C's `== NULL`)
      xfree (ai as *mut c_void);                                        // C 8481
    }                                                                   // C 8482
}                                                                       // C 8483

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_shutdown (s: c_int, how: c_int) -> c_int   // C 8485
{                                                                       // C 8487
  if winsock_lib.is_null ()                                             // C 8488 (C's `== NULL`)
    {                                                                   // C 8489
      *errno () = ENETDOWN;                                             // C 8490
      return SOCKET_ERROR;                                              // C 8491
    }                                                                   // C 8492

  check_errno ();                                                       // C 8494
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8495 (C's truth test)
    {                                                                   // C 8496
      let rc: c_int = pfn_shutdown.unwrap () (SOCK_HANDLE! (s), how);   // C 8497 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8498
	{ set_errno (); }                                                      // C 8499
      return rc;                                                        // C 8500
    }                                                                   // C 8501
  *errno () = ENOTSOCK;                                                 // C 8502
  return SOCKET_ERROR;                                                  // C 8503
}                                                                       // C 8504

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_setsockopt (s: c_int, level: c_int, optname: c_int, // C 8506
                                        optval: *const c_void, optlen: c_int) -> c_int // C 8507
{                                                                       // C 8508
  if winsock_lib.is_null ()                                             // C 8509 (C's `== NULL`)
    {                                                                   // C 8510
      *errno () = ENETDOWN;                                             // C 8511
      return SOCKET_ERROR;                                              // C 8512
    }                                                                   // C 8513

  check_errno ();                                                       // C 8515
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8516 (C's truth test)
    {                                                                   // C 8517
      let rc: c_int = pfn_setsockopt.unwrap () (SOCK_HANDLE! (s), level, optname, // C 8518 (C's call-through)
					       optval as *const c_char, optlen);                           // C 8519 (C's `(const char *)` cast)
      if rc == SOCKET_ERROR                                             // C 8520
	{ set_errno (); }                                                      // C 8521
      return rc;                                                        // C 8522
    }                                                                   // C 8523
  *errno () = ENOTSOCK;                                                 // C 8524
  return SOCKET_ERROR;                                                  // C 8525
}                                                                       // C 8526

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_listen (s: c_int, backlog: c_int) -> c_int // C 8528
{                                                                       // C 8530
  if winsock_lib.is_null ()                                             // C 8531 (C's `== NULL`)
    {                                                                   // C 8532
      *errno () = ENETDOWN;                                             // C 8533
      return SOCKET_ERROR;                                              // C 8534
    }                                                                   // C 8535

  check_errno ();                                                       // C 8537
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8538 (C's truth test)
    {                                                                   // C 8539
      let rc: c_int = pfn_listen.unwrap () (SOCK_HANDLE! (s), backlog); // C 8540 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8541
	{ set_errno (); }                                                      // C 8542
      else                                                              // C 8543
	{ fd_info[s as usize].flags |= FILE_LISTEN as c_uint; }                // C 8544
      return rc;                                                        // C 8545
    }                                                                   // C 8546
  *errno () = ENOTSOCK;                                                 // C 8547
  return SOCKET_ERROR;                                                  // C 8548
}                                                                       // C 8549


/* C 8550's blank and C 8551-8644's four wrappers: `sys_getsockname` (8551-8570, slice 30's shape),
   `sys_accept` (8572-8600 - the only one that touches the child record: C 8582 tests `FILE_LISTEN`,
   C 8593-8594 store `cp->status` and call `ResetEvent (cp->char_avail)`), `sys_recvfrom`
   (8602-8622) and `sys_sendto` (8624-8644).  `ResetEvent` (`<winbase.h>`'s `BOOL WINAPI
   ResetEvent (HANDLE)`) is declared in-unit (P-88, `UNVERIFIED`, Owed 33); `fd_info[s].cp` is the
   mirror's `*mut c_void`, so C 8593-8594's two member reads go through slice 25's
   `child_process` cast.  */
/// C: WINBASEAPI BOOL WINAPI ResetEvent (HANDLE hEvent);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn ResetEvent (hEvent: HANDLE) -> BOOL;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_getsockname (s: c_int, name: *mut sockaddr, namelen: *mut c_int) -> c_int // C 8551
{                                                                       // C 8553
  if winsock_lib.is_null ()                                             // C 8554 (C's `== NULL`)
    {                                                                   // C 8555
      *errno () = ENETDOWN;                                             // C 8556
      return SOCKET_ERROR;                                              // C 8557
    }                                                                   // C 8558

  check_errno ();                                                       // C 8560
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8561 (C's truth test)
    {                                                                   // C 8562
      let rc: c_int = pfn_getsockname.unwrap () (SOCK_HANDLE! (s), name, namelen); // C 8563 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8564
	{ set_errno (); }                                                      // C 8565
      return rc;                                                        // C 8566
    }                                                                   // C 8567
  *errno () = ENOTSOCK;                                                 // C 8568
  return SOCKET_ERROR;                                                  // C 8569
}                                                                       // C 8570

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_accept (s: c_int, addr: *mut sockaddr, addrlen: *mut c_int) -> c_int // C 8572
{                                                                       // C 8574
  if winsock_lib.is_null ()                                             // C 8575 (C's `== NULL`)
    {                                                                   // C 8576
      *errno () = ENETDOWN;                                             // C 8577
      return -1;                                                        // C 8578
    }                                                                   // C 8579

  check_errno ();                                                       // C 8581
  if (fd_info[s as usize].flags & FILE_LISTEN as c_uint) != 0           // C 8582 (C's truth test)
    {                                                                   // C 8583
      let mut t: SOCKET = pfn_accept.unwrap () (SOCK_HANDLE! (s), addr, addrlen); // C 8584 (C's call-through)
      let mut fd: c_int = -1;                                           // C 8585 (C's `int fd = -1;`)
      if t == INVALID_SOCKET                                            // C 8586
	{ set_errno (); }                                                      // C 8587
      else                                                              // C 8588
	{ fd = socket_to_fd (t); }                                             // C 8589

      if fd >= 0                                                        // C 8591
	{                                                                      // C 8592
	  (*(fd_info[s as usize].cp as *mut child_process)).status             // C 8593
	    = STATUS_READ_ACKNOWLEDGED;                                        // C 8593 (C's record store)
	  ResetEvent ((*(fd_info[s as usize].cp as *mut child_process)).char_avail); // C 8594
	}                                                                      // C 8595
      return fd;                                                        // C 8596
    }                                                                   // C 8597
  *errno () = ENOTSOCK;                                                 // C 8598
  return -1;                                                            // C 8599
}                                                                       // C 8600

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_recvfrom (s: c_int, buf: *mut c_char, len: c_int, flags: c_int, // C 8602
				       from: *mut sockaddr, fromlen: *mut c_int) -> c_int           // C 8603
{                                                                       // C 8605
  if winsock_lib.is_null ()                                             // C 8606 (C's `== NULL`)
    {                                                                   // C 8607
      *errno () = ENETDOWN;                                             // C 8608
      return SOCKET_ERROR;                                              // C 8609
    }                                                                   // C 8610

  check_errno ();                                                       // C 8612
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8613 (C's truth test)
    {                                                                   // C 8614
      let rc: c_int = pfn_recvfrom.unwrap () (SOCK_HANDLE! (s), buf, len, flags, from, fromlen); // C 8615 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8616
	{ set_errno (); }                                                      // C 8617
      return rc;                                                        // C 8618
    }                                                                   // C 8619
  *errno () = ENOTSOCK;                                                 // C 8620
  return SOCKET_ERROR;                                                  // C 8621
}                                                                       // C 8622

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_sendto (s: c_int, buf: *const c_char, len: c_int, flags: c_int, // C 8624
				       to: *const sockaddr, tolen: c_int) -> c_int                  // C 8625
{                                                                       // C 8627
  if winsock_lib.is_null ()                                             // C 8628 (C's `== NULL`)
    {                                                                   // C 8629
      *errno () = ENETDOWN;                                             // C 8630
      return SOCKET_ERROR;                                              // C 8631
    }                                                                   // C 8632

  check_errno ();                                                       // C 8634
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8635 (C's truth test)
    {                                                                   // C 8636
      let rc: c_int = pfn_sendto.unwrap () (SOCK_HANDLE! (s), buf, len, flags, to, tolen); // C 8637 (C's call-through)
      if rc == SOCKET_ERROR                                             // C 8638
	{ set_errno (); }                                                      // C 8639
      return rc;                                                        // C 8640
    }                                                                   // C 8641
  *errno () = ENOTSOCK;                                                 // C 8642
  return SOCKET_ERROR;                                                  // C 8643
}                                                                       // C 8644


/* C 8645's blank, C 8646-8647's `Windows does not have an fcntl function` comment (C's own text)
   and C 8648-8707's `fcntl` - this file's own `<fcntl.h>` emulation: C 8654-8655's
   `F_DUPFD_CLOEXEC` shortcut to `sys_dup` (whose C definition is C 8824, past this slice, so the
   name stays unresolved - the slice-24 `socket_to_fd` reading), the socket arm's `F_SETFL`/
   `O_NONBLOCK` pair through `pfn_ioctlsocket`/`FIONBIO` with `FILE_NDELAY`, and the pipe arm's
   `SetNamedPipeHandleState`/`PIPE_NOWAIT`.  `F_DUPFD_CLOEXEC`/`F_SETFL`/`O_NONBLOCK`/`F_GETFL`
   are `lib/include/fcntl.in.rs`'s constants (in this unit's include graph, C 35), `FIONBIO` slice
   25's and `FILE_SOCKET`/`FILE_PIPE`/`FILE_WRITE`/`FILE_NDELAY` `w32.h`'s; `PIPE_NOWAIT` and
   `SetNamedPipeHandleState` (`<winbase.h>`) are declared in-unit (P-88, `UNVERIFIED`, Owed 34).  */
/* Windows does not have an fcntl function.  Provide an implementation
   good enough for Emacs.  */
/// C: #define PIPE_NOWAIT 0x00000001  (`<winbase.h>`)
#[cfg (feature = "windowsnt")]
pub const PIPE_NOWAIT: DWORD = 0x00000001;
/// C: WINBASEAPI BOOL WINAPI SetNamedPipeHandleState (HANDLE, LPDWORD, LPDWORD, LPDWORD);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn SetNamedPipeHandleState (hNamedPipe: HANDLE, lpMode: *mut DWORD,
                                  lpMaxCollectionCount: *mut DWORD,
                                  lpCollectDataTimeout: *mut DWORD) -> BOOL;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fcntl (s: c_int, cmd: c_int, options: c_int) -> c_int // C 8648
{                                                                       // C 8650
  /* In the w32 Emacs port, fcntl (fd, F_DUPFD_CLOEXEC, fd1) is always  // C 8651
     invoked in a context where fd1 is closed and all descriptors less  // C 8652
     than fd1 are open, so sys_dup is an adequate implementation.  */   // C 8653
  if cmd == F_DUPFD_CLOEXEC                                             // C 8654
    { return sys_dup (s); }                                             // C 8655

  check_errno ();                                                       // C 8657
  if (fd_info[s as usize].flags & FILE_SOCKET as c_uint) != 0           // C 8658 (C's truth test)
    {                                                                   // C 8659
      if winsock_lib.is_null ()                                         // C 8660 (C's `== NULL`)
	{                                                                      // C 8661
	  *errno () = ENETDOWN;                                                // C 8662
	  return -1;                                                           // C 8663
	}                                                                      // C 8664

      if cmd == F_SETFL && options == O_NONBLOCK                        // C 8666
	{                                                                      // C 8667
	  let mut nblock: c_ulong = 1;                                         // C 8668 (C's `unsigned long`)
	  let rc: c_int = pfn_ioctlsocket.unwrap () (SOCK_HANDLE! (s), FIONBIO, &mut nblock); // C 8669 (C's call-through)
	  if rc == SOCKET_ERROR                                                // C 8670
	    { set_errno (); }                                                  // C 8671
	  /* Keep track of the fact that we set this to non-blocking.  */      // C 8672
	  fd_info[s as usize].flags |= FILE_NDELAY as c_uint;                  // C 8673
	  return rc;                                                           // C 8674
	}                                                                      // C 8675
      else                                                              // C 8676
	{                                                                      // C 8677
	  *errno () = EINVAL;                                                  // C 8678
	  return SOCKET_ERROR;                                                 // C 8679
	}                                                                      // C 8680
    }                                                                   // C 8681
  else if (fd_info[s as usize].flags & (FILE_PIPE | FILE_WRITE) as c_uint) // C 8682 (C's two-line condition)
	   == (FILE_PIPE | FILE_WRITE) as c_uint                               // C 8683
    {                                                                   // C 8684
      /* Force our writes to pipes be non-blocking.  */                 // C 8685
      if cmd == F_SETFL && options == O_NONBLOCK                        // C 8686
	{                                                                      // C 8687
	  let h: HANDLE = _get_osfhandle (s) as HANDLE;                        // C 8688 (C's `(HANDLE)` cast)
	  let mut pipe_mode: DWORD = PIPE_NOWAIT;                              // C 8689

	  if SetNamedPipeHandleState (h, &raw mut pipe_mode, std::ptr::null_mut (), // C 8691 (C's truth test)
				      std::ptr::null_mut ()) == 0                                   // C 8691 (C's two `NULL`s)
	    {                                                                  // C 8692
	      DebPrint! (("SetNamedPipeHandleState: %lu\n", GetLastError ())); // C 8693
	      return SOCKET_ERROR;                                             // C 8694
	    }                                                                  // C 8695
	  fd_info[s as usize].flags |= FILE_NDELAY as c_uint;                  // C 8696
	  return 0;                                                            // C 8697
	}                                                                      // C 8698
      else                                                              // C 8699
	{                                                                      // C 8700
	  *errno () = EINVAL;                                                  // C 8701
	  return SOCKET_ERROR;                                                 // C 8702
	}                                                                      // C 8703
    }                                                                   // C 8704
  *errno () = ENOTSOCK;                                                 // C 8705
  return SOCKET_ERROR;                                                  // C 8706
}                                                                       // C 8707


/* C 8708's blank, C 8710-8711's `Shadow main io functions` comment (C's own text) and C 8713-8821's
  `sys_close` - the descriptor teardown, and the first function to walk the child record: C 8726-8740
  read `fd_info[fd].cp` and the thread's two flags, C 8742-8780's `CHILD_ACTIVE` arm scans `MAXDESC`
  for a second user of the record and then shuts the socket down (`pfn_shutdown`, `pfn_closesocket`,
  `winsock_inuse--`), and C 8787-8810's second arm decides between `_close` and leaving
  `FILE_DONT_CLOSE` for the reader thread.  `STILL_ACTIVE` and `GetExitCodeThread` (`<winbase.h>`)
  are declared in-unit (P-88, `UNVERIFIED`, Owed 35); `CHILD_ACTIVE`/`delete_child`/
  `STATUS_READ_IN_PROGRESS` are the `w32.h` mirror's rows, `FILE_DONT_CLOSE` its 261 and the
  `child_process` record slice 25's own spelling.  */
/* Shadow main io functions: we need to handle pipes and sockets more
   intelligently.  */
/// C: #define STILL_ACTIVE 0x00000103  (`<winbase.h>`; 259)
#[cfg (feature = "windowsnt")]
pub const STILL_ACTIVE: DWORD = 0x00000103;
/// C: WINBASEAPI BOOL WINAPI GetExitCodeThread (HANDLE hThread, LPDWORD lpExitCode);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetExitCodeThread (hThread: HANDLE, lpExitCode: *mut DWORD) -> BOOL;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_close (fd: c_int) -> c_int                 // C 8713
{                                                                       // C 8715
  let mut rc: c_int = -1;                                               // C 8716 (C's `int rc = -1;`)
  let mut reader_thread_exited: bool = false;                           // C 8717 (C's `bool … = false;`)
  let mut reader_thread_reading: bool = false;                          // C 8718

  if fd < 0                                                             // C 8720
    {                                                                   // C 8721
      *errno () = EBADF;                                                // C 8722
      return -1;                                                        // C 8723
    }                                                                   // C 8724

  if fd < MAXDESC && ! fd_info[fd as usize].cp.is_null ()               // C 8726 (C's truth test)
    {                                                                   // C 8727
      let cp: *mut child_process = fd_info[fd as usize].cp as *mut child_process; // C 8728 (the mirror's `*mut c_void` depth)
      let mut thrd_status: DWORD = STILL_ACTIVE;                        // C 8729

      /* Thread handle will be NULL if we already called delete_child.  */ // C 8731
      if !(*cp).thrd.is_null ()                                         // C 8732 (C's `!= NULL`)
	  && GetExitCodeThread ((*cp).thrd, &raw mut thrd_status) != 0         // C 8733 (C's truth test)
	  && thrd_status != STILL_ACTIVE                                       // C 8734
	{ reader_thread_exited = true; }                                       // C 8735

      if !(*cp).thrd.is_null () && (*cp).status == STATUS_READ_IN_PROGRESS // C 8737
	{ reader_thread_reading = true; }                                      // C 8738

      fd_info[fd as usize].cp = std::ptr::null_mut ();                  // C 8740 (C's `= NULL`)

      if CHILD_ACTIVE! (cp)                                             // C 8742 (the mirror's macro)
	{                                                                      // C 8743
	  /* if last descriptor to active child_process then cleanup */        // C 8744
	  let mut i: c_int;                                                    // C 8745

	  /* C leaves `i` uninitialised (PORTING.md 1.5).  */
	  i = 0;                                                               // C 8746
	  while i < MAXDESC                                                    // C 8746
	    {                                                                  // C 8747
	      if i == fd                                                       // C 8748
		{                                                                     // C 8749
		  i += 1;                                                             // C 8748 (C's `continue` plus the `for`'s increment)
		  continue;
		}
	      if fd_info[i as usize].cp == cp as *mut c_void                   // C 8750
		{ break; }                                                            // C 8751
	      i += 1;                                                          // C 8746 (the for's increment)
	    }                                                                  // C 8752
	  if i == MAXDESC                                                      // C 8753
	    {                                                                  // C 8754
	      if (fd_info[fd as usize].flags & FILE_SOCKET as c_uint) != 0     // C 8755
		{                                                                     // C 8756
		  if winsock_lib.is_null () { emacs_abort (); }                       // C 8757 (C's one-line `if`)

		  pfn_shutdown.unwrap () (SOCK_HANDLE! (fd), 2);                      // C 8759 (C's call-through)
		  rc = pfn_closesocket.unwrap () (SOCK_HANDLE! (fd));                 // C 8760

		  winsock_inuse -= 1;                                                 // C 8762 (C's `winsock_inuse--` and its comment)
		}                                                                     // C 8763
	      /* If the process handle is NULL, it's either a socket           // C 8764
		 or serial connection, or a subprocess that was                       // C 8765
		 already reaped by reap_subprocess, but whose                         // C 8766
		 resources were not yet freed, because its output was                 // C 8767
		 not fully read yet by the time it was reaped.  (This                 // C 8768
		 usually happens with async subprocesses whose output                 // C 8769
		 is being read by Emacs.)  Otherwise, this process was                // C 8770
		 not reaped yet, so we set its FD to a negative value                 // C 8771
		 to make sure sys_select will eventually get to                       // C 8772
		 calling the SIGCHLD handler for it, which will then                  // C 8773
		 invoke waitpid and reap_subprocess.  */                              // C 8774
	      if (*cp).procinfo.hProcess.is_null ()                            // C 8775
		{ delete_child (cp as *mut c_void); }                                 // C 8776
	      else                                                             // C 8777
		{ (*cp).fd = -1; }                                                    // C 8778
	    }                                                                  // C 8779
	}                                                                      // C 8780
    }                                                                   // C 8781

  /* Note that sockets do not need special treatment here (at least on  // C 8783
     NT and Windows 95 using the standard tcp/ip stacks) - it appears that // C 8784
     closesocket is equivalent to CloseHandle, which is to be expected  // C 8785
     because socket handles are fully fledged kernel handles. */        // C 8786
  if fd < MAXDESC                                                       // C 8787
    {                                                                   // C 8788
      if (((fd_info[fd as usize].flags & FILE_DONT_CLOSE as c_uint) == 0 // C 8789
	   /* Don't attempt to close a descriptor while reader thread is       // C 8790
	      stuck in _sys_read_ahead attempting to read from it, as          // C 8791
	      that will hang Emacs.  */                                        // C 8792
	   && !(((fd_info[fd as usize].flags & (FILE_PIPE | FILE_READ) as c_uint) // C 8793
		 == (FILE_PIPE | FILE_READ) as c_uint)                                // C 8794
		&& reader_thread_reading))                                            // C 8795
	  /* If the reader thread already exited, close the descriptor,        // C 8796
	     since otherwise no one will close it, and we will be              // C 8797
	     leaking descriptors.  */                                          // C 8798
	  || reader_thread_exited)                                             // C 8799
	{                                                                      // C 8800
	  fd_info[fd as usize].flags = 0;                                      // C 8801
	  rc = _close (fd);                                                    // C 8802
	}                                                                      // C 8803
      else                                                              // C 8804
	{                                                                      // C 8805
	  /* We don't close here descriptors open by pipe processes            // C 8806
	     for reading from the pipe when the reader thread might            // C 8807
	     still be running, since that thread might be stuck in             // C 8808
	     _sys_read_ahead, and then we will hang here.  If the              // C 8809
	     reader thread exits normally, it will close the                   // C 8810
	     descriptor; otherwise we will leave a zombie thread               // C 8811
	     hanging around.  */                                               // C 8812
	  rc = 0;                                                              // C 8813
	  /* Leave the flag set for the reader thread to close the             // C 8814
	     descriptor.  */                                                   // C 8815
	  fd_info[fd as usize].flags = FILE_DONT_CLOSE as c_uint;              // C 8816
	}                                                                      // C 8817
    }                                                                   // C 8818

  return rc;                                                            // C 8820
}                                                                       // C 8821


/* C 8822's blank and C 8823-8875's `sys_dup`/`sys_dup2` - the two descriptor duplicators, which is
  what retires the one name this lane owed itself (`sys_dup`, called from C 8655 in slice 34): C 8828's
  `_dup` and C 8868's `_dup2` are the CRT's, and both copy this unit's own `filedesc` record
  (`fd_info[new_fd] = fd_info[fd]`, C 8832/8872).  The two CRT names are declared in-unit (P-88,
  `UNVERIFIED`, Owed 36); `INVALID_HANDLE_VALUE` is C 1540's constant, `_get_osfhandle` the unit's own
  C 8525 declaration and `sys_close` slice 35's definition.  */
/// C: int _dup (int);  (`<io.h>`)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _dup (fd: c_int) -> c_int;
}
/// C: int _dup2 (int src, int dst);  (`<io.h>`; returns 0 on success)
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _dup2 (src: c_int, dst: c_int) -> c_int;
}

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_dup (fd: c_int) -> c_int                   // C 8823
{                                                                       // C 8825
  let mut new_fd: c_int;                                                // C 8826

  /* C leaves `new_fd` uninitialised (PORTING.md 1.5).  */
  new_fd = 0;

  new_fd = _dup (fd);                                                   // C 8828
  if new_fd >= 0 && new_fd < MAXDESC                                    // C 8829
    {                                                                   // C 8830
      /* duplicate our internal info as well */                         // C 8831
      fd_info[new_fd as usize] = fd_info[fd as usize];                  // C 8832
    }                                                                   // C 8833
  return new_fd;                                                        // C 8834
}                                                                       // C 8835

#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_dup2 (src: c_int, dst: c_int) -> c_int     // C 8837
{                                                                       // C 8839
  let mut rc: c_int;                                                    // C 8840

  /* C leaves `rc` uninitialised (PORTING.md 1.5).  */
  rc = 0;

  if dst < 0 || dst >= MAXDESC                                          // C 8842
    {                                                                   // C 8843
      *errno () = EBADF;                                                // C 8844
      return -1;                                                        // C 8845
    }                                                                   // C 8846

  /* MS _dup2 seems to have weird side effect when invoked with 2       // C 8848
     identical arguments: an attempt to fclose the corresponding stdio  // C 8849
     stream after that hangs (we do close standard streams in           // C 8850
     init_ntproc).  Attempt to avoid that by not calling _dup2 that     // C 8851
     way: if SRC is valid, we know that dup2 should be a no-op, so do   // C 8852
     nothing and return DST.  */                                        // C 8853
  if src == dst                                                         // C 8854
    {                                                                   // C 8855
      if _get_osfhandle (src) as HANDLE == INVALID_HANDLE_VALUE         // C 8856 (C's `(HANDLE)` cast)
	{                                                                      // C 8857
	  *errno () = EBADF;                                                   // C 8858
	  return -1;                                                           // C 8859
	}                                                                      // C 8860
      return dst;                                                       // C 8861
    }                                                                   // C 8862

  /* Make sure we close the destination first if it's a pipe or socket.  */ // C 8864
  if fd_info[dst as usize].flags != 0                                   // C 8865
    { sys_close (dst); }                                                // C 8866

  rc = _dup2 (src, dst);                                                // C 8868
  if rc == 0                                                            // C 8869
    {                                                                   // C 8870
      /* Duplicate our internal info as well.  */                       // C 8871
      fd_info[dst as usize] = fd_info[src as usize];                    // C 8872
    }                                                                   // C 8873
  return if rc == 0 { dst } else { rc };                                // C 8874 (C's ternary)
}                                                                       // C 8875
